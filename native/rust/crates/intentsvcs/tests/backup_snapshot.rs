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
