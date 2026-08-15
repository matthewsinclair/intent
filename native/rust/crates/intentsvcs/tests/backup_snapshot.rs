//! AT-03.11 / AC-03.10: the DB snapshot is taken through SQLite, and a file
//! copy of the same store is a defect rather than a slower alternative.
//!
//! **The discriminating case is a WAL-resident write with the writer
//! connection still open**, and getting that wrong is how the first attempt at
//! this measurement failed to reproduce the bug. The store opens in WAL mode,
//! so a committed transaction lives in `intent.db-wal` until something
//! checkpoints it -- and a lone reader CLOSING CLEANLY checkpoints and
//! truncates the WAL. So a test that drops the facade before snapshotting sees
//! a fully checkpointed `intent.db`, finds the copy perfectly good, and PASSES
//! ON THE DEFECT.
//!
//! Every test here therefore holds the facade alive across the snapshot.

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::store::Store;

/// Copy `intent.db` the way a plausible implementation would -- and only that
/// file, which is exactly the mistake.
///
/// A directory tar or a `cp` of the three WAL files has its own race (the
/// three are not captured atomically), so this is not a strawman standing in
/// for a better copy: it is the copy the AC names, and the point is that no
/// file-level copy is correct while a writer is open.
fn naive_copy(fx: &Fixture, dest: &std::path::Path) {
  std::fs::copy(fx.project().db_path(), dest).expect("copy the db file");
}

/// AC-03.10(a): the snapshot carries a committed write that is still in the
/// WAL, and a file copy of the same store does not.
#[test]
fn a_snapshot_carries_a_wal_resident_write_and_a_file_copy_does_not() {
  let fx = Fixture::new();
  let mut facade = fx.facade_on_disk();

  // A real mutation through the facade. It commits, and with the connection
  // still open it stays in the WAL -- nothing has checkpointed.
  facade
    .st_new("A thread that lives in the WAL")
    .expect("st new");
  let expected = facade.st_list().len();
  assert!(expected > 0, "precondition: the store holds something");

  let good = fx.path("snapshot.db");
  let bad = fx.path("copy.db");
  facade
    .store()
    .snapshot_into(&good)
    .expect("VACUUM INTO the snapshot");
  naive_copy(&fx, &bad);

  // The facade is STILL ALIVE here, deliberately. Dropping it first would
  // checkpoint the WAL and make the bad copy look good.
  let from_snapshot = Store::open(&good).expect("the snapshot opens");
  let (threads, _) = from_snapshot.load_canon().expect("read the snapshot");
  assert_eq!(
    threads.len(),
    expected,
    "the snapshot carries every committed thread"
  );

  let from_copy = Store::open(&bad).expect("the naive copy also opens -- that is the trap");
  let (copied, _) = from_copy.load_canon().expect("read the copy");
  eprintln!(
    "MEASURED: snapshot={} copy={} expected={}",
    threads.len(),
    copied.len(),
    expected
  );
  assert_ne!(
    copied.len(),
    expected,
    "a file copy taken while a writer is open MISSES the WAL-resident write. If this ever passes, either the store stopped using WAL or the fixture checkpointed -- both make this whole file vacuous"
  );

  drop(facade);
}

/// AC-03.10(b): a snapshot is restorable, and restoring it is PROVEN rather
/// than assumed -- it opens, it passes schema validation, and it returns the
/// same entity counts as the source.
#[test]
fn a_snapshot_restores_to_the_same_estate_it_was_taken_from() {
  let fx = Fixture::new();
  let mut facade = fx.facade_on_disk();
  fx.write_thread(&sample_thread("ST0056"));
  facade.sync_from_disk().expect("ingest the fixture");

  let (threads, issues) = (facade.st_list().len(), facade.canon().issues.len());
  let dump = facade.store().derived_dump().expect("dump the source");

  let snap = fx.path("restore.db");
  facade.store().snapshot_into(&snap).expect("snapshot");

  // Opening is the schema check: `Store::open` refuses an unstamped or
  // mismatched `user_version` rather than handing back a connection that fails
  // at the first query (AC-02.7). So a snapshot that opens has been validated,
  // and one that cannot is refused by name.
  let restored = Store::open(&snap).expect("a snapshot opens on the schema that wrote it");
  let (rt, ri) = restored.load_canon().expect("read the restored store");
  assert_eq!(rt.len(), threads, "same thread count");
  assert_eq!(ri.len(), issues, "same issue count");
  assert_eq!(
    restored.derived_dump().expect("dump the restored store"),
    dump,
    "and the same rows, not merely the same number of them -- a count can agree while the contents do not"
  );

  drop(facade);
}

/// A snapshot never silently replaces an earlier one.
///
/// Retention counts snapshots; an operation that overwrote one would make the
/// count a lie while looking like it worked, so SQLite's refusal to write an
/// existing path is load-bearing rather than incidental.
#[test]
fn a_snapshot_refuses_to_overwrite_an_existing_file() {
  let fx = Fixture::new();
  let facade = fx.facade_on_disk();
  let dest = fx.path("taken.db");

  facade.store().snapshot_into(&dest).expect("the first one");
  let err = facade
    .store()
    .snapshot_into(&dest)
    .expect_err("the second must refuse rather than overwrite");
  assert!(
    err.to_string().contains("exists"),
    "and the refusal says why: {err}"
  );

  drop(facade);
}

// ---------------------------------------------------------------------------
// AC-03.10 (d): a backup that never ran and a backup that failed are DIFFERENT
// states, and neither is allowed to be silent.
// ---------------------------------------------------------------------------

use intentsvcs::store::SnapshotOutcome;

/// **An attempt is recorded BEFORE the copy, so a failure leaves evidence.**
///
/// The natural implementation records a snapshot after it succeeds, which makes
/// a crash mid-copy indistinguishable from a schedule that was never due --
/// both leave no row. Opening the row first means the only way to leave nothing
/// behind is to never have started.
#[test]
fn a_backup_attempt_is_recorded_before_it_can_succeed_or_fail() {
  let store = Store::open_in_memory().expect("open");
  let (id, stamp) = store.begin_snapshot().expect("begin");

  let open = store.snapshots().expect("read");
  assert_eq!(open.len(), 1, "the attempt exists before any file does");
  assert_eq!(open[0].id, id);
  assert_eq!(
    open[0].outcome, "attempted",
    "an attempt that has not finished says so, rather than looking like a success"
  );
  assert!(
    open[0].path.is_none(),
    "and it points at nothing, because nothing has been written yet"
  );
  assert_eq!(
    open[0].taken_at, stamp,
    "the stamp handed back is the one on the row -- it is what the snapshot file is named from, \
     so a caller never has to ask what time it is to name a file"
  );
  assert_eq!(
    stamp.len(),
    24,
    "DB-written, millisecond precision: {stamp}"
  );
}

#[test]
fn a_finished_attempt_carries_what_it_wrote() {
  let store = Store::open_in_memory().expect("open");
  let (id, _) = store.begin_snapshot().expect("begin");
  store
    .finish_snapshot(id, SnapshotOutcome::Ok, Some("db/x.db"), Some(4096), None)
    .expect("finish");

  let all = store.snapshots().expect("read");
  assert_eq!(all[0].outcome, "ok");
  assert_eq!(all[0].path.as_deref(), Some("db/x.db"));
  assert_eq!(all[0].bytes, Some(4096));
}

/// **A FAILED attempt must not reset staleness, and this is the discriminating
/// case for the whole arm.**
///
/// A schedule that fires hourly and fails every time is the worst state this
/// criterion covers: something IS happening, so a naive "when did we last try"
/// reports a healthy recent number while no restorable snapshot exists. Only
/// `ok` rows count, so a failing schedule reads exactly as stale as one that
/// never ran -- which is the truth, because in both cases there is nothing to
/// restore from.
#[test]
fn a_failed_attempt_does_not_make_the_backup_look_fresh() {
  let store = Store::open_in_memory().expect("open");

  assert!(
    store
      .hours_since_last_good_snapshot()
      .expect("query")
      .is_none(),
    "nothing has ever succeeded, and that is reported as an absence rather than as zero -- zero \
     would read as a backup taken this instant"
  );

  let (id, _) = store.begin_snapshot().expect("begin");
  store
    .finish_snapshot(id, SnapshotOutcome::Failed, None, None, Some("disk full"))
    .expect("finish");

  assert!(
    store
      .hours_since_last_good_snapshot()
      .expect("query")
      .is_none(),
    "a failure is still nothing to restore from; a schedule that runs and fails every hour must \
     not read as healthier than one that has never run"
  );
  let all = store.snapshots().expect("read");
  assert_eq!(all[0].outcome, "failed");
  assert_eq!(
    all[0].detail.as_deref(),
    Some("disk full"),
    "and the reason survives, or the report can only say that something went wrong"
  );
}

/// A successful backup makes the age readable, and the age is an INTERVAL
/// rather than a time.
///
/// Nothing in this test knows what time it is, and nothing can: the only value
/// that crosses out of SQLite is a number of hours. That is what keeps D42's
/// "reading a clock to decide" permission from being needed here at all.
#[test]
fn staleness_is_an_interval_the_database_computes() {
  let store = Store::open_in_memory().expect("open");
  let (id, _) = store.begin_snapshot().expect("begin");
  store
    .finish_snapshot(id, SnapshotOutcome::Ok, Some("db/x.db"), Some(1), None)
    .expect("finish");

  let hours = store
    .hours_since_last_good_snapshot()
    .expect("query")
    .expect("one has succeeded");
  assert!(
    (0.0..1.0).contains(&hours),
    "a backup taken moments ago is hours-old by a very small amount, got {hours}"
  );
}
