//! AT-03.10 / AC-03.9: `sync` has two directions, they differ in
//! destructiveness, and the difference is visible before it is paid.
//!
//! **The defect this exists about.** hv reversed D01 on 2026-08-15 -- the DB is
//! the SSOT and the files are re-creatable -- and `sync` had only its
//! DESTRUCTIVE half. It read the files and replaced the store from them, which
//! under the old model was a refresh and under the new one is a RESTORE. So
//! the routine operation (rewrite the files from truth) did not exist at all,
//! and the one verb anyone would reach for to repair stale files was the one
//! that would destroy the change they were stale against.
//!
//! It was found by checking the premise of a remedy while writing it: the
//! file-write-failure error had said "run `intent sync`". That instruction
//! would have told an operator to destroy the exact change the error had just
//! promised them was safe.
//!
//! **The discriminating case is the stale-file restore** (vc's call, and it is
//! the right one): a store that holds a change the files do not, restored from
//! those files. Everything else here is a property; that one is the loss.

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::model::ThreadStatus;

/// Put the store ahead of the files: mutate with the tree read-only, so the
/// change lands in truth and the projection is refused.
///
/// This is not a contrived state. It is precisely what
/// `FacadeError::ViewsNotWritten` reports, so the fixture is the situation an
/// operator is actually in when they reach for `sync`.
#[cfg(unix)]
fn store_ahead_of_disk(fx: &Fixture) -> intentsvcs::facade::Facade {
  let mut facade = fx.facade();
  facade.st_start("ST0056").expect("materialise the views");

  let mode = fx.make_readonly("intent");
  let result = facade.st_cancel("ST0056");
  fx.restore_mode("intent", mode);

  assert!(
    result.is_err(),
    "precondition: the projection must have failed, or the store is not ahead"
  );
  assert_eq!(
    facade.st_show("ST0056").expect("thread").status,
    ThreadStatus::Cancelled,
    "precondition: the change is in the store"
  );
  facade
}

/// **THE DISCRIMINATING CASE.** A restore from stale files destroys a change
/// that exists only in the source of truth.
///
/// This test asserts the LOSS rather than a guard against it, deliberately.
/// The direction is legitimate -- restoring from disk is a real operation
/// someone will want -- so the answer is not to forbid it but to make its cost
/// visible and never to default to it. Proving the cost is real is what makes
/// the refusal on the bare verb something other than ceremony.
#[cfg(unix)]
#[test]
fn a_restore_from_stale_files_destroys_a_change_the_store_alone_holds() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = store_ahead_of_disk(&fx);

  facade.sync_from_disk().expect("restore from disk");

  assert_eq!(
    facade.st_show("ST0056").expect("thread").status,
    ThreadStatus::Wip,
    "the cancel is GONE: the restore read files that never received it and replaced truth with them. This is the loss AC-03.9 exists to make visible, not a bug in the restore"
  );
}

/// And the routine direction repairs exactly that state instead.
#[cfg(unix)]
#[test]
fn the_routine_direction_rewrites_the_stale_files_from_truth() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = store_ahead_of_disk(&fx);

  let on_disk_before = fx.read("intent/st/ST0056/thread.json");
  assert!(
    !on_disk_before.contains("cancelled"),
    "precondition: the files are stale"
  );

  facade.sync_to_disk().expect("project from the store");

  assert!(
    fx.read("intent/st/ST0056/thread.json")
      .contains("cancelled"),
    "the files now carry the change the store held"
  );
  assert_eq!(
    facade.st_show("ST0056").expect("thread").status,
    ThreadStatus::Cancelled,
    "and truth was not disturbed by being projected"
  );
}

/// The destructive direction can be asked what it would cost, BEFORE it costs
/// it.
///
/// A summary afterwards is a receipt for a loss; the operator needed it one
/// moment earlier. AC-03.9 says the destructive direction states what it will
/// overwrite, and this is the statement.
#[cfg(unix)]
#[test]
fn the_overwrite_is_named_before_the_restore_runs() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let facade = store_ahead_of_disk(&fx);

  let would_lose = facade.sync_overwrite().expect("preview");
  assert!(
    would_lose.iter().any(|line| line.contains("ST0056")),
    "the preview names the thread that would be overwritten: {would_lose:?}"
  );

  // Still true afterwards: asking is not doing.
  assert_eq!(
    facade.st_show("ST0056").expect("thread").status,
    ThreadStatus::Cancelled,
    "the preview is a read -- it must not have performed the restore it described"
  );
}

/// An in-step estate previews EMPTY, which is what makes a non-empty preview
/// worth reading.
///
/// Without this the preview could list every entity every time and still pass
/// the test above -- an alarm that always fires is one an operator learns to
/// skip, which is how the destructive direction would become silent again by a
/// different route.
#[test]
fn an_estate_in_step_has_nothing_to_overwrite() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();
  facade.st_start("ST0056").expect("mutate and project");

  assert!(
    facade.sync_overwrite().expect("preview").is_empty(),
    "a tree that already matches the store would lose nothing, and must say so"
  );
}

/// The two directions compose to identity on an estate that is in step, so
/// running either one on a healthy project is not a change.
#[test]
fn the_directions_round_trip_on_a_healthy_estate() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();
  facade.st_start("ST0056").expect("mutate and project");

  let canon_before = fx.read("intent/st/ST0056/thread.json");
  let db_before = facade.store().snapshot().expect("snapshot");

  facade.sync_to_disk().expect("db -> disk");
  facade.sync_from_disk().expect("disk -> db");
  facade.sync_to_disk().expect("db -> disk again");

  assert_eq!(
    fx.read("intent/st/ST0056/thread.json"),
    canon_before,
    "the canon file is byte-identical after a full round trip"
  );
  assert_eq!(
    facade.store().snapshot().expect("snapshot"),
    db_before,
    "and so is the store -- neither direction drifts on an estate that is in step"
  );
}

/// A restore reports what it restored, and the routine direction reports what
/// it projected. Neither returns a bare success.
#[test]
fn both_directions_report_a_count_rather_than_a_bare_ok() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();

  assert_eq!(facade.sync_to_disk().expect("to disk"), 1);
  assert_eq!(facade.sync_from_disk().expect("from disk"), 1);
}
