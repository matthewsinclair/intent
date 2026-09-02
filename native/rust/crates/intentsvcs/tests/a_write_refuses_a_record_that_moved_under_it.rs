//! **A CANON WRITE DERIVED FROM A STALE RECORD IS REFUSED, INSIDE THE
//! MUTATION'S OWN TRANSACTION** -- issue 0206, ruled by vc 2026-09-01.
//!
//! # The defect
//!
//! Every canon verb is a read-modify-write over the snapshot the facade loaded
//! when it opened: clone the canon, edit one field, write the whole record
//! back. Two sessions editing DIFFERENT fields of one thread each write the
//! whole thing, and the second carries the first's field at its **pre-edit**
//! value. No error, no conflict, valid canon afterwards.
//!
//! Measured on the shipping binary before this fix: **9 of 15 concurrent pairs
//! lost a write**, every one `rc=0` with no error text and a 0.01s wall time --
//! the two writes never even contended at the SQLite level, because two
//! well-formed serialised transactions is exactly what the store saw and it was
//! correct to accept both. **The transaction was never what got corrupted. The
//! record was.**
//!
//! # What is under test, and what deliberately is not
//!
//! This drives the PROPERTY, not the race: two facades opened before either
//! writes are the state the incident was in, however it got there. A test that
//! waited for a scheduler to produce that state would be measuring the
//! scheduler. The issue says in terms that the window is not characterised, and
//! this file does not characterise it either.
//!
//! # Every facade here is `facade_on_disk`, and that is not incidental
//!
//! `Fixture::facade()` opens an IN-MEMORY store, so two of them share no
//! database and cannot exhibit this at all -- each would write its own and the
//! loss would come from the second overwriting the first's FILE, which is a
//! different defect with the same symptom. **A fixture that cannot exhibit the
//! defect cannot clear it, and one that exhibits a DIFFERENT defect with the
//! same symptom is worse, because it looks like coverage.** Two nodes running
//! `intent` share one on-disk store; that is the population.

use crate::common::{Fixture, sample_thread};
use intentsvcs::facade::FacadeError;
use intentsvcs::sync::Scope;

/// A criterion's text, through the PUBLIC surface.
///
/// `Facade::criterion` is private, and a test reaching for it would be asserting
/// against a seam an operator cannot see.
fn ac_text(f: &intentsvcs::facade::Facade, st: &str, ac: &str) -> String {
  f.st_show(st)
    .unwrap()
    .criteria
    .iter()
    .find(|c| c.id == ac)
    .unwrap_or_else(|| panic!("{st} has no {ac}"))
    .text
    .clone()
}

/// Read a thread's write counter straight out of the database.
///
/// Through `rusqlite` rather than the facade, because `revision` is per-machine
/// write metadata and deliberately not modelled content -- there is no public
/// API for it, and inventing one so a test could read it would put a column in
/// the model to serve the test.
fn revision_of(fx: &Fixture, id: &str) -> i64 {
  rusqlite::Connection::open(fx.project().db_path())
    .unwrap()
    .query_row(
      "SELECT revision FROM threads WHERE id = ?1",
      rusqlite::params![id],
      |row| row.get(0),
    )
    .unwrap()
}

/// **THE NON-VACUITY CONTROL, AND EVERY REFUSAL ARM BELOW DEPENDS ON IT.**
///
/// Ordinary serial work -- open, write, close, open, write -- must not refuse.
/// A store that refused every change would pass all three refusal arms and be
/// completely broken, and this is the arm that says so.
#[test]
fn sequential_edits_through_fresh_facades_both_land() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));

  let mut first = fx.facade_on_disk();
  first
    .ac_edit("ST0001", "AC-03.1", "the first edit")
    .unwrap();

  // A SECOND facade, opened after the first write landed, so its snapshot is
  // current. This is the shape of one operator running two commands.
  let mut second = fx.facade_on_disk();
  second
    .ac_edit("ST0001", "AC-03.2", "the second edit")
    .expect("a write derived from a CURRENT record must not refuse");

  let after = fx.facade_on_disk();
  assert_eq!(ac_text(&after, "ST0001", "AC-03.1"), "the first edit");
  assert_eq!(ac_text(&after, "ST0001", "AC-03.2"), "the second edit");
}

/// **THE DISCRIMINATING ARM.** Two facades, both loaded before either wrote, so
/// both hold the same snapshot. They edit DIFFERENT criteria, which is what
/// makes this a lost write rather than a conflict anyone would expect: neither
/// operator touched the other's field.
///
/// Before this fix the second returned `Ok` and the first edit was gone.
#[test]
fn two_facades_holding_one_snapshot_do_not_both_get_to_write() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));

  let mut a = fx.facade_on_disk();
  let mut b = fx.facade_on_disk();

  a.ac_edit("ST0001", "AC-03.1", "the edit that survives")
    .unwrap();

  let err = b
    .ac_edit("ST0001", "AC-03.2", "the edit derived from a stale record")
    .expect_err(
      "the second facade's write carries AC-03.1 at its pre-edit value and would erase the \
       first edit -- it must be refused",
    );

  // **THE VARIANT IS ASSERTED, NOT MERELY THAT AN ERROR HAPPENED.** Travelling
  // as `FacadeError::Store` would render "could not update the runtime store"
  // and pass an `is_err` check, while telling the operator to go and look at
  // their disk over what is in fact two sessions working one thread.
  assert!(
    matches!(&err, FacadeError::RecordMovedUnderTheWrite { subject } if subject.contains("ST0001")),
    "the refusal must name the record it hit, as its own variant: {err:?}"
  );
}

/// **AND THE FIRST EDIT IS STILL THERE**, which is the whole of the ruling.
///
/// The arm above proves a refusal; this proves the refusal is protecting
/// something. A store that refused AND had already written would pass that
/// assertion and fail this one.
#[test]
fn the_refused_write_leaves_the_edit_it_would_have_erased() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));

  let mut a = fx.facade_on_disk();
  let mut b = fx.facade_on_disk();

  a.ac_edit("ST0001", "AC-03.1", "the edit that survives")
    .unwrap();
  let _ = b.ac_edit("ST0001", "AC-03.2", "the write that must not land");

  // Read through a THIRD facade, so this is about what the store holds rather
  // than about either writer's in-memory canon.
  let after = fx.facade_on_disk();
  assert_eq!(
    ac_text(&after, "ST0001", "AC-03.1"),
    "the edit that survives",
    "the surviving edit must still be there"
  );
  assert_ne!(
    ac_text(&after, "ST0001", "AC-03.2"),
    "the write that must not land",
    "and the refused write must not have landed either -- a refusal that wrote anyway is worse \
     than no refusal, because it reports failure and succeeds"
  );
}

/// **THE CONTROL THAT MAKES THE FINDING SPECIFIC** -- and it is 0206's own.
///
/// The same two concurrent facades, editing two DIFFERENT threads, must both
/// land. Concurrency does not break the verb; it breaks it **on one record**.
/// The measured harness saw zero cross-thread losses in 40 iterations, and a
/// fix that refused here would be refusing correct work.
#[test]
fn two_facades_editing_different_threads_both_land() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  fx.write_thread(&sample_thread("ST0002"));

  let mut a = fx.facade_on_disk();
  let mut b = fx.facade_on_disk();

  a.ac_edit("ST0001", "AC-03.1", "a change to one thread")
    .unwrap();
  b.ac_edit("ST0002", "AC-03.1", "a change to the other")
    .expect("a stale snapshot of a thread this write does not touch must not refuse it");

  let after = fx.facade_on_disk();
  assert_eq!(
    ac_text(&after, "ST0001", "AC-03.1"),
    "a change to one thread"
  );
  assert_eq!(
    ac_text(&after, "ST0002", "AC-03.1"),
    "a change to the other"
  );
}

/// **THE ARM THAT KILLED THE OBVIOUS DESIGN -- AND IT KILLED IT FOR A REASON
/// SHARPER THAN THE ONE THIS FILE FIRST CLAIMED.**
///
/// `threads.revision` shipped at `544a83d3` described as this fix's
/// compare-and-swap token. The first version of this arm asserted that a resync
/// BUMPS every revision in the estate, which would have made a counter-based
/// CAS merely noisy. **Driving it said `0 -> 0`, and the real mechanism is
/// worse:** `rebuild` DELETEs every row before re-inserting it, so the upsert
/// never hits a conflict, the conflict clause carrying `revision + 1` never
/// fires, and the column comes back at its DEFAULT. **The counter does not go
/// up across a sync. It goes back to zero.**
///
/// That is not noise, it is unsound: a facade that loaded at 0, a peer write
/// that took the record to 1, and a sync that resets it to 0 leaves the stale
/// facade seeing `0 == 0` and writing straight over the peer. **A counter-based
/// CAS would have FAILED OPEN on exactly the case it was built for.**
///
/// Every assertion here is load-bearing. The first proves the counter moves at
/// all, so the second is a reset rather than a column nothing ever touches --
/// without it this arm would pass on a build where `revision` was dead code.
#[test]
fn a_resync_resets_the_counter_and_the_write_is_still_allowed() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));

  // Move the counter, so its value below is a reset and not a zero that was
  // never anything else.
  let mut bumper = fx.facade_on_disk();
  bumper
    .ac_edit("ST0001", "AC-03.1", "a write to move the counter")
    .unwrap();
  let bumped = revision_of(&fx, "ST0001");
  assert_eq!(
    bumped, 1,
    "the change door's conflict clause must bump the counter, or nothing below distinguishes a \
     reset from a column that never moves"
  );

  // The snapshot this write will be derived from, taken BEFORE the resync.
  let mut writer = fx.facade_on_disk();

  // A whole-estate resync. Nothing was edited on disk, so this rewrites the
  // same content it already holds.
  let mut syncer = fx.facade_on_disk();
  syncer.sync_from_disk(&Scope::All).unwrap();

  let after_sync = revision_of(&fx, "ST0001");
  assert_eq!(
    after_sync, 0,
    "a resync deletes and re-inserts, so the counter returns to its default -- it went \
     {bumped} -> {after_sync}. A CAS on this token would compare a stale session's value \
     against a number that has been reset underneath it"
  );

  writer
    .ac_edit("ST0001", "AC-03.2", "an edit across a resync")
    .expect(
      "the record's CONTENT did not move, only its write counter -- and refusing here would \
       refuse every facade opened before any sync",
    );

  assert_eq!(
    ac_text(&fx.facade_on_disk(), "ST0001", "AC-03.2"),
    "an edit across a resync"
  );
}

/// The same property for issues, because the incident that opened the sibling
/// ruling was an issue body and one ruling covers the mutation surface rather
/// than one verb.
#[test]
fn two_facades_holding_one_issue_snapshot_do_not_both_get_to_write() {
  let fx = Fixture::new();
  let mut seed = fx.facade_on_disk();
  let n = seed
    .issue_add(
      "an issue two people will edit",
      None,
      None,
      "the original body",
    )
    .unwrap();

  let mut a = fx.facade_on_disk();
  let mut b = fx.facade_on_disk();

  a.issue_edit(n, None, Some("the title that survives"), None)
    .unwrap();
  let err = b
    .issue_edit(n, Some("a body derived from a stale record"), None, None)
    .expect_err("the second write carries the title at its pre-edit value and must be refused");

  assert!(
    matches!(&err, FacadeError::RecordMovedUnderTheWrite { subject } if subject.contains(&format!("{n:04}"))),
    "the refusal must name the issue it hit: {err:?}"
  );

  let after = fx.facade_on_disk();
  assert_eq!(
    after.issue_show(n).unwrap().title,
    "the title that survives"
  );
}
