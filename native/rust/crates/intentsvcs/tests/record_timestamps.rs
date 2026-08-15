//! AT-02.8 / AC-02.8: **every table carries a record timestamp, and the
//! DATABASE writes it.**
//!
//! **The discriminating case is not "is the column populated".** It is
//! populated whether the database filled it or a caller passed one in, so a
//! test that inserts a row and reads the column back passes on the defect. What
//! is proved here instead is that the value arrives when the CALLER HAS NO TIME
//! TO GIVE -- the facade is driven through real verbs, no time crosses its
//! surface, and the column is non-empty anyway. That is a property a
//! caller-supplied stamp cannot have.
//!
//! Second arm: two writes in sequence are non-decreasing. A read-then-write gap
//! cannot guarantee that -- a value read early and written late can land after
//! a value read later and written sooner -- so it is the property that
//! distinguishes "the database stamped it" from "someone asked what time it was
//! and then wrote the answer down".
//!
//! Third arm, and it is the one that decays silently: **`created_at` must NOT
//! move on the second write.** That is the whole reason `threads` and `issues`
//! are upserted rather than deleted and re-inserted. Restore the delete and
//! every column here still populates, still parses, still orders -- and
//! `created_at` quietly becomes `updated_at` under the wrong name, which is
//! precisely the defect AC-02.8 was raised to remove.

mod common;

use common::{Fixture, sample_issue, sample_thread};
use intentsvcs::store::{DDL, RECORD_TIMESTAMPS};
use rusqlite::Connection;

/// Every `CREATE TABLE` in the DDL, as `(name, body)`.
///
/// **The roster is DISCOVERED, never listed.** A hand-kept list is a list
/// someone must remember to extend on the day they add a table, which is the
/// day they are thinking about something else -- and this criterion exists
/// because eight tables shipped with no record timestamp and nobody noticed.
fn table_bodies() -> Vec<(String, String)> {
  let mut out = Vec::new();
  for chunk in DDL.split("CREATE TABLE IF NOT EXISTS ").skip(1) {
    let (name, rest) = chunk.split_once(" (").expect("a table header");
    let body = rest.split(");").next().expect("a table body ends");
    out.push((name.trim().to_string(), body.to_string()));
  }
  out
}

/// Every record-timestamp column a table declares, as `(column, has_default)`.
///
/// **Returned per column rather than as a yes/no for the table, and the first
/// cut of this file got that wrong in a way worth keeping the note for.** It
/// asked "does this table have A stamp with a DEFAULT", which is satisfied by a
/// single good column -- so stripping the DEFAULT off `file_index.created_at`
/// left `updated_at` to answer for it and the check passed. The mutation did
/// break the build, loudly, through three unrelated snapshot tests hitting a
/// NOT NULL violation. **A defect that only shows up somewhere else is a defect
/// this guard does not cover**, and the guard is the thing that is supposed to
/// say what is wrong.
///
/// The DEFAULT is the requirement rather than a detail of style: a column the
/// caller is expected to fill is a column the caller can fill WRONG, and D42's
/// whole claim is that the stamp and the write are one operation.
fn stamp_columns(name: &str, body: &str) -> Vec<(String, bool)> {
  // `event_log.ts` IS this table's record timestamp -- append-only rows, so
  // there is nothing an `updated_at` could record. Declared in the DDL rather
  // than left as an absence, and checked here rather than assumed.
  let wanted: Vec<&str> = if name == "event_log" {
    vec!["ts"]
  } else {
    RECORD_TIMESTAMPS.to_vec()
  };
  body
    .lines()
    .map(str::trim)
    .filter_map(|line| {
      wanted
        .iter()
        .find(|c| line.starts_with(&format!("{c} ")))
        .map(|c| ((*c).to_string(), line.contains("DEFAULT (")))
    })
    .collect()
}

#[test]
fn every_table_records_when_the_database_wrote_the_row() {
  let bodies = table_bodies();

  // The parse has to find every table, or the coverage claim is over a subset
  // and reads as complete. A table declared without `IF NOT EXISTS` would slip
  // past the split silently.
  assert_eq!(
    bodies.len(),
    DDL.matches("CREATE TABLE").count(),
    "the walk missed a table -- found {:?}",
    bodies.iter().map(|(n, _)| n).collect::<Vec<_>>()
  );
  assert!(bodies.len() >= 8, "precondition: the DDL was read at all");

  let mut missing = Vec::new();
  let mut caller_filled = Vec::new();
  for (name, body) in &bodies {
    let columns = stamp_columns(name, body);
    if columns.is_empty() {
      missing.push(name.clone());
    }
    for (column, has_default) in columns {
      if !has_default {
        caller_filled.push(format!("{name}.{column}"));
      }
    }
  }

  assert!(
    missing.is_empty(),
    "these tables record nothing about when this database wrote a row, so nothing can order or \
     merge them (AC-02.8): {missing:?}"
  );
  assert!(
    caller_filled.is_empty(),
    "these record-timestamp columns have no DEFAULT, so the caller is expected to supply the \
     value -- which is the confection D42 forbids, wearing the right column name: {caller_filled:?}"
  );
}

/// **The FTS5 table is the one exemption, and it is asserted rather than
/// assumed.** An exemption nobody checks passes forever, including on the day
/// it stops describing reality.
#[test]
fn the_only_table_without_one_is_the_virtual_one_and_it_cannot_have_one() {
  assert_eq!(
    DDL.matches("CREATE VIRTUAL TABLE").count(),
    1,
    "a second virtual table appeared; decide what its record timestamp is rather than inheriting \
     this exemption by being virtual"
  );
  assert!(
    DDL.contains("CREATE VIRTUAL TABLE IF NOT EXISTS doc_sections USING fts5"),
    "the exemption names doc_sections specifically"
  );
  // Not a policy choice: an FTS5 table's columns are the indexed surface and it
  // has no column defaults to give. Its rows are also wholly derived -- wiped
  // and recomputed from files that carry their own mtime -- so the question it
  // would answer is already answered elsewhere.
  assert!(
    !DDL.contains("doc_sections USING fts5 (\n  created_at"),
    "an FTS5 declaration cannot carry a DEFAULT column"
  );
}

/// **The gap check reports a real gap.** Run over a synthetic DDL that is one
/// column short, the same function must name the offender -- otherwise the
/// green above could mean "found nothing" rather than "found everything".
#[test]
fn a_table_missing_its_stamp_is_reported_rather_than_passed_over() {
  let stamped = "id TEXT PRIMARY KEY,\n  created_at TEXT NOT NULL DEFAULT (strftime('%s','now'))";
  assert_eq!(
    stamp_columns("widgets", stamped),
    vec![("created_at".to_string(), true)],
    "the canary must recognise a well-formed stamp, or it proves nothing when it fails one"
  );

  assert!(
    stamp_columns("widgets", "id TEXT PRIMARY KEY,\n  title TEXT NOT NULL").is_empty(),
    "a table with no record timestamp must be reported"
  );
  // The DEFAULT is the requirement, not the name. A column called `created_at`
  // that a caller is expected to fill is the confection with better spelling.
  assert_eq!(
    stamp_columns(
      "widgets",
      "id TEXT PRIMARY KEY,\n  created_at TEXT NOT NULL"
    ),
    vec![("created_at".to_string(), false)],
    "a stamp column with no DEFAULT leaves the caller to fill it, which is the thing being banned"
  );
  // **The case the first cut missed**: one good column must not answer for a
  // bad one. Reported per column, so a sibling cannot cover for it.
  assert_eq!(
    stamp_columns(
      "widgets",
      "created_at TEXT NOT NULL,\n  updated_at TEXT NOT NULL DEFAULT (strftime('%s','now'))"
    ),
    vec![
      ("created_at".to_string(), false),
      ("updated_at".to_string(), true)
    ],
    "a table with one stamped and one unstamped column must report the unstamped one"
  );
}

fn stamp(conn: &Connection, sql: &str) -> String {
  conn
    .query_row(sql, [], |r| r.get::<_, String>(0))
    .unwrap_or_else(|e| panic!("reading a record stamp: {sql} -- {e}"))
}

/// **The whole point: the caller never had a time, and the row has one.**
///
/// Every verb here takes a title or an id. Nothing in the chain accepts a
/// timestamp, so there is no value the test could have smuggled in even if it
/// wanted to -- which is what makes the assertion meaningful rather than
/// circular.
#[test]
fn a_row_written_through_the_facade_is_stamped_although_the_caller_had_no_time() {
  let fx = Fixture::new();
  let db = fx.project().db_path();
  {
    let mut facade = fx.facade_on_disk();
    fx.write_thread(&sample_thread("ST0056"));
    fx.write_issue(&sample_issue(21));
    facade.sync_from_disk().expect("ingest");
  }

  let conn = Connection::open(&db).expect("open the store directly");
  for sql in [
    "SELECT created_at FROM threads",
    "SELECT updated_at FROM threads",
    "SELECT written_at FROM wps",
    "SELECT written_at FROM criteria",
    "SELECT written_at FROM tests",
    "SELECT created_at FROM issues",
  ] {
    let value = stamp(&conn, sql);
    assert_eq!(
      value.len(),
      24,
      "{sql} -- RFC 3339 UTC at millisecond precision, got {value:?}"
    );
    assert!(value.ends_with('Z'), "{sql} -- UTC explicitly: {value}");
    assert!(value.starts_with("20"), "{sql} -- a real date: {value}");
  }
}

/// Two sequential writes are non-decreasing, and `created_at` does not move.
///
/// The first is the property a read-then-write gap cannot give. The second is
/// the upsert: destroy the row on each write and `created_at` follows
/// `updated_at`, which is a `created_at` that means nothing.
#[test]
fn a_second_write_moves_updated_at_and_leaves_created_at_where_it_was() {
  let fx = Fixture::new();
  let db = fx.project().db_path();

  let (first_created, first_updated) = {
    let mut facade = fx.facade_on_disk();
    fx.write_thread(&sample_thread("ST0056"));
    facade.sync_from_disk().expect("ingest");
    let conn = Connection::open(&db).expect("open");
    (
      stamp(&conn, "SELECT created_at FROM threads"),
      stamp(&conn, "SELECT updated_at FROM threads"),
    )
  };

  {
    let mut facade = fx.facade_on_disk();
    facade
      .st_hold("ST0056", "waiting on the fleet")
      .expect("hold");
  }

  let conn = Connection::open(&db).expect("reopen");
  let second_created = stamp(&conn, "SELECT created_at FROM threads");
  let second_updated = stamp(&conn, "SELECT updated_at FROM threads");

  assert_eq!(
    first_created, second_created,
    "`created_at` records the FIRST write and must not move -- if this fails, the row is being \
     deleted and re-inserted and the column now means `updated_at`"
  );
  assert!(
    second_updated >= first_updated,
    "two writes in sequence are non-decreasing; got {first_updated} then {second_updated}"
  );
  assert!(
    second_updated >= second_created,
    "a row cannot be updated before it was created: created {second_created}, updated \
     {second_updated}"
  );
}

/// **A rebuild re-stamps the RECORD time and must not touch the DOMAIN date.**
///
/// The two kinds of time in this schema, told apart by the one operation that
/// treats them differently. `created` is a fact about the world, carried in the
/// extract; `created_at` is a fact about this database, and a rebuild genuinely
/// did write the row, so re-stamping it is correct rather than tolerated.
#[test]
fn a_rebuild_restamps_the_record_time_and_carries_the_authored_date_through() {
  let fx = Fixture::new();
  let db = fx.project().db_path();
  {
    let mut facade = fx.facade_on_disk();
    fx.write_thread(&sample_thread("ST0056"));
    facade.sync_from_disk().expect("ingest");
  }
  let authored = {
    let conn = Connection::open(&db).expect("open");
    stamp(&conn, "SELECT created FROM threads")
  };

  // A machine that has only ever had the extract: same canon, a store that has
  // never seen it. Nothing is deleted -- D36 -- the second store is simply new.
  let clone = fx.clone_extract();
  let clone_db = clone.project().db_path();
  {
    let mut facade = clone.facade_on_disk();
    facade.sync_from_disk().expect("ingest on the clone");
  }

  let conn = Connection::open(&clone_db).expect("open the clone");
  assert_eq!(
    stamp(&conn, "SELECT created FROM threads"),
    authored,
    "the AUTHORED date travelled in the extract and must be identical on the clone -- re-stamping \
     it would report every thread as created the day someone cloned the repo"
  );
  assert_eq!(
    stamp(&conn, "SELECT created_at FROM threads").len(),
    24,
    "and the clone's own record timestamp is its own: this store wrote this row just now"
  );
}

/// **The domain date and the record stamp are the SAME INSTANT, by
/// construction.**
///
/// `st new` hands in an empty `created`; the INSERT fills it and `created_at`
/// takes its DEFAULT in the same statement. SQLite guarantees `'now'` returns
/// one value for every reference within a single statement, so these cannot
/// drift -- not even across a UTC midnight, which is the boundary a
/// read-then-write pair fails at silently.
///
/// This also settles vc's adopted derivation without implementing it twice: if
/// `created` is ever derived from the `st.new` event's `ts` instead, it must
/// still land on this day, and this assertion already says so.
#[test]
fn the_authored_date_and_the_record_stamp_agree_because_one_statement_wrote_both() {
  let fx = Fixture::new();
  let db = fx.project().db_path();
  let created = {
    let mut facade = fx.facade_on_disk();
    let id = facade.st_new("a thread that stamps itself").expect("new");
    facade.st_show(&id).expect("show").created.clone()
  };

  let conn = Connection::open(&db).expect("open");
  assert_eq!(
    stamp(&conn, "SELECT date(created_at) FROM threads"),
    created,
    "the domain date and the record stamp came out of one INSERT and must name one day"
  );
  assert_eq!(created.len(), 10, "a date, not a timestamp: {created}");

  // And the event the mutation wrote agrees too -- same transaction, so the
  // derivation vc adopted lands on the same day whichever way it is computed.
  let event_day = stamp(
    &conn,
    "SELECT date(ts) FROM event_log WHERE op = 'st.new' ORDER BY ts LIMIT 1",
  );
  assert_eq!(
    event_day, created,
    "`thread.created` and its own `st.new` event must name the same day"
  );
}
