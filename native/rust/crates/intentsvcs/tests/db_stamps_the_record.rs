//! D42: DB records have a timestamp field, and that is the source of truth for
//! time. Nothing else. Ever.
//!
//! **The application does not supply a time, and this is stronger than "read
//! the clock from the DB".** Between asking what time it is and writing the row
//! there is a gap, and a write that is retried, deferred or batched inside it
//! is stamped when it was PREPARED rather than when it happened. A column
//! DEFAULT has no gap: the stamp and the write are one operation.
//!
//! Three earlier positions were all wrong in the same direction, each one
//! better-sourced than the last and none of them the ruling: the process clock,
//! then a caller-supplied `FacadeContext.today`, then `Store::now()` read into
//! a variable and written into the record. **Asking is the act being ruled on.**

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::event::{Envelope, LOCAL_PRINCIPAL, Subject};
use intentsvcs::store::{SCHEMA_VERSION, Store};
use rusqlite::Connection;

fn subject() -> Subject {
  Subject {
    kind: "wp".to_string(),
    id: "ST0056/02".to_string(),
  }
}

fn minted() -> Envelope {
  Envelope::minted(
    LOCAL_PRINCIPAL,
    "00000000-0000-0000-0000-000000000000",
    "wp.start",
    subject(),
    serde_json::json!({"from": "not-started", "to": "wip"}),
  )
}

/// A minted envelope has NO time, and the database gives it one.
#[test]
fn an_envelope_is_minted_without_a_time_and_the_database_supplies_it() {
  let envelope = minted();
  assert!(
    envelope.ts.is_empty(),
    "nothing has happened yet, so there is no time to carry: {:?}",
    envelope.ts
  );

  let store = Store::open_in_memory().expect("open");
  let stamp = store.append_event(&envelope).expect("append");

  assert!(
    !stamp.is_empty(),
    "the database stamped the row and returned what it wrote"
  );
  // The shape the log declares: RFC 3339 UTC, seconds precision.
  assert_eq!(stamp.len(), 20, "YYYY-MM-DDTHH:MM:SSZ -- got {stamp}");
  assert!(stamp.ends_with('Z'), "UTC, explicitly: {stamp}");

  let stored = store.events().expect("read back");
  assert_eq!(stored.len(), 1);
  assert_eq!(
    stored[0].ts, stamp,
    "and the row carries exactly what the insert returned"
  );
}

/// **A restore carries the original time; it does not re-record.**
///
/// This is the discriminating case for the two-act split. If a restore
/// re-stamped, then reconstituting a store from a clone of yesterday's extract
/// would produce a log claiming everything happened at the moment of the
/// restore -- and every stamp in it would look perfectly valid.
#[test]
fn a_restore_carries_the_original_time_rather_than_recording_a_new_one() {
  let store = Store::open_in_memory().expect("open");

  let mut historical = minted();
  historical.ts = "2019-03-14T09:26:53Z".to_string();
  let carried = store.restore_event(&historical).expect("restore");

  assert_eq!(
    carried, "2019-03-14T09:26:53Z",
    "the extract's own stamp survived the write"
  );
  let stored = store.events().expect("read back");
  assert_eq!(
    stored[0].ts, "2019-03-14T09:26:53Z",
    "a record of something that happened THEN is not a record of something happening NOW"
  );
}

/// The two acts really are different at the same call site, on the same
/// envelope. A single mode could not do both, which is why there are two.
#[test]
fn the_two_write_acts_disagree_on_purpose() {
  let store = Store::open_in_memory().expect("open");

  let mut old = minted();
  old.ts = "2019-03-14T09:26:53Z".to_string();
  let restored = store.restore_event(&old).expect("restore");

  let mut fresh = minted();
  fresh.ts = "2019-03-14T09:26:53Z".to_string(); // deliberately set, and ignored
  fresh.id = format!("{}X", fresh.id);
  let recorded = store.append_event(&fresh).expect("append");

  assert_eq!(restored, "2019-03-14T09:26:53Z");
  assert_ne!(
    recorded, "2019-03-14T09:26:53Z",
    "append IGNORES whatever the envelope carries -- the database decides, and a caller cannot smuggle a time in through the field"
  );
}

/// The whole mutation path is stamped by the database: no facade call reads a
/// clock, and every event it writes still comes out with a time.
#[test]
fn a_facade_mutation_produces_a_database_stamped_event() {
  let fx = Fixture::new();
  let mut facade = fx.facade_on_disk();
  fx.write_thread(&sample_thread("ST0056"));
  facade.sync_from_disk().expect("ingest");
  facade
    .st_hold("ST0056", "waiting on the fleet")
    .expect("hold");

  let events = facade.store().events().expect("events");
  let held = events
    .iter()
    .find(|e| e.op == "st.hold")
    .expect("the mutation wrote its event");
  assert_eq!(held.ts.len(), 20, "stamped by the DB: {}", held.ts);
  assert!(held.ts.ends_with('Z'));

  drop(facade);
}

/// **A store at the previous schema version is MIGRATED, not refused** -- and
/// the migration does not rewrite the history it inherits.
///
/// This direction was unreachable until now: at version 1, `SCHEMA_VERSION - 1`
/// is 0, and 0 is the ABSENCE of a version rather than schema zero. Version 2
/// is the first release where an older stamped store can exist at all.
#[test]
fn a_store_at_the_previous_version_is_migrated_and_keeps_its_history() {
  let dir = tempfile::tempdir().expect("tempdir");
  let path = dir.path().join("v1.db");

  // A v1 store: `event_log.ts` with no DEFAULT, which is the shape the stamp
  // was introduced against.
  {
    let conn = Connection::open(&path).expect("create");
    conn
      .execute_batch(
        "CREATE TABLE event_log (
           id TEXT PRIMARY KEY,
           ts TEXT NOT NULL,
           principal TEXT NOT NULL,
           project_id TEXT NOT NULL,
           op TEXT NOT NULL,
           subject_type TEXT NOT NULL,
           subject_id TEXT NOT NULL,
           payload TEXT NOT NULL
         );
         INSERT INTO event_log VALUES ('01ARZ3', '2019-03-14T09:26:53Z', 'local', 'p', 'st.new', 'thread', 'ST0001', '{}');",
      )
      .expect("lay down a v1 event log");
    conn
      .pragma_update(None, "user_version", 1)
      .expect("stamp 1");
  }

  let store = Store::open(&path).expect("a store one version behind is migrated, never refused");

  let events = store.events().expect("read the migrated log");
  assert_eq!(events.len(), 1, "the row survived the table rebuild");
  assert_eq!(
    events[0].ts, "2019-03-14T09:26:53Z",
    "and it kept its ORIGINAL stamp -- re-stamping would move the whole log to the moment of the upgrade"
  );

  // The point of the migration: the DEFAULT now fires on this store too.
  let stamp = store
    .append_event(&minted())
    .expect("append after migrating");
  assert_ne!(
    stamp, "2019-03-14T09:26:53Z",
    "a new record gets a new, database-assigned time"
  );
  drop(store);

  assert_eq!(
    Connection::open(&path)
      .expect("reopen")
      .pragma_query_value(None, "user_version", |r| r.get::<_, i32>(0))
      .expect("version"),
    SCHEMA_VERSION,
    "and the store is stamped with the version it was migrated to"
  );
}
