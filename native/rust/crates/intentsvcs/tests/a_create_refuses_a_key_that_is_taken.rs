//! **A VERB NAMED `add`/`new` FAILS ON AN EXISTING KEY RATHER THAN REPLACING
//! IT** -- issue 0131, ruled by hv 2026-08-28.
//!
//! # The incident this is the deterministic form of
//!
//! Two nodes ran `intent issues add` on 2026-08-28 and **both were told
//! `created: intent/.canon/issues/0126.json`**. One filing survived; the other
//! reached neither the store nor the extract, and nothing said so. `git show
//! 9e9ee8ab:intent/.canon/issues/0126.json` -- the losing node's own commit --
//! carries the other node's issue.
//!
//! Two code facts made it possible. `next_issue_number()` is `max() + 1` over
//! the canon this facade loaded, a read with no reservation; and the landing
//! was `INSERT ... ON CONFLICT (number) DO UPDATE SET ... body =
//! excluded.body`. **A create implemented as an upsert.** There was no
//! unique-violation path: the collision was converted into a full overwrite and
//! the call returned `Ok`.
//!
//! # The issue reported that the race would not reproduce, and that stands
//!
//! Three drives failed to reproduce it, and the informative one explains why:
//! `next_issue_number()` reads canon from the STORE, not from the extract, so
//! two SEQUENTIAL adds can never collide. The issue says in terms that the
//! window is not characterised, and this file does not claim to characterise it
//! either.
//!
//! **So this drives the PROPERTY rather than the race.** Two facades opened
//! before either writes both compute the same next number -- which is the state
//! the incident was in, however it got there -- and the second write must be
//! refused. A test that waited for a scheduler to produce that state would be
//! measuring the scheduler.
//!
//! # What the arms are for
//!
//! The refusal is worth nothing on its own: a store that refused every write
//! would pass a refusal test and fail at everything else. So the ordinary
//! create, the ordinary CHANGE, and a whole-estate resync are all driven here.
//! **The change arms are not padding -- they are the ones that would have gone
//! red if the door had been implemented by dropping the `ON CONFLICT` clause
//! outright**, which is the obvious reading of the ruling and is wrong:
//! `issues close` clones canon, edits one field and writes the whole row back.

//! # Every facade here is `facade_on_disk`, and that is not incidental
//!
//! `Fixture::facade()` opens an IN-MEMORY store, so two of them share no
//! database at all -- each writes its own and then renders the extract, and the
//! second simply overwrites the first one's FILE. Written that way first, this
//! file reproduced a loss with the same symptom and a different cause, and the
//! refusal under test could not have prevented it. **A fixture that cannot
//! exhibit the defect cannot clear it, and one that exhibits a DIFFERENT defect
//! with the same symptom is worse, because it looks like coverage.** Two nodes
//! running `intent` share one on-disk store; that is the population.

mod common;

use common::{Fixture, sample_issue, sample_thread};
use intentsvcs::facade::FacadeError;
use intentsvcs::model::IssueStatus;

/// The ordinary path, and the non-vacuity control for everything below.
///
/// Two sequential adds through ONE facade allocate two numbers and both
/// survive. If this ever goes red, the refusal arms are passing because the
/// verb is broken rather than because the door is shut.
#[test]
fn two_sequential_adds_allocate_two_numbers_and_both_survive() {
  let fx = Fixture::new();
  let mut f = fx.facade_on_disk();

  let first = f
    .issue_add("the first filing", None, None, "body one")
    .unwrap();
  let second = f
    .issue_add("the second filing", None, None, "body two")
    .unwrap();

  assert_ne!(first, second, "two adds must not land on one number");
  assert_eq!(
    f.issue_show(first).unwrap().title,
    "the first filing",
    "the first filing must survive the second"
  );
  assert_eq!(f.issue_show(second).unwrap().title, "the second filing");
}

/// **THE DISCRIMINATING ARM.** Two facades, both loaded before either wrote, so
/// both compute the same next number. The second must be REFUSED.
///
/// Before the fix this returned `Ok` and the first filing was gone.
#[test]
fn two_facades_that_both_think_the_number_is_free_do_not_both_get_it() {
  let fx = Fixture::new();

  // Both open BEFORE either writes -- the state the incident was in. Nothing
  // here is timing-dependent: the canon each one holds is fixed at open.
  let mut a = fx.facade_on_disk();
  let mut b = fx.facade_on_disk();

  let landed = a
    .issue_add("the filing that survives", None, None, "")
    .unwrap();

  let refused = b.issue_add("the filing that must not overwrite it", None, None, "");
  let err = refused.expect_err(
    "the second facade allocated a number the first had already taken, and writing it \
     would replace the first filing -- a create must refuse",
  );

  // **THE VARIANT IS ASSERTED, NOT JUST THE FACT OF AN ERROR.** A refusal that
  // travelled as `FacadeError::Store` would render "could not update the
  // runtime store" and pass a test that only checked `is_err` -- and it would
  // tell the operator to look at their disk over a key collision.
  assert!(
    matches!(err, FacadeError::IssueExists { number } if number == landed),
    "the refusal must name the issue it hit, as its own variant: {err:?}"
  );
  assert!(
    err.to_string().contains(&format!("{landed:04}")),
    "and the rendered message must carry the number: {err}"
  );
}

/// **AND THE FIRST FILING IS STILL THERE**, which is the whole of the ruling.
///
/// The arm above proves a refusal; this proves the refusal is protecting
/// something. A store that refused AND had already clobbered the row would pass
/// the assertion above and fail this one.
#[test]
fn the_refused_write_leaves_the_first_filing_byte_intact() {
  let fx = Fixture::new();
  let mut a = fx.facade_on_disk();
  let mut b = fx.facade_on_disk();

  let landed = a
    .issue_add(
      "the filing that survives",
      None,
      None,
      "the body that survives",
    )
    .unwrap();
  let _ = b.issue_add("the overwrite that must not happen", None, None, "clobber");

  // Read through a THIRD facade, so the assertion is about what the store
  // holds rather than about either writer's in-memory canon.
  let after = fx.facade_on_disk();
  let issue = after.issue_show(landed).unwrap();
  assert_eq!(issue.title, "the filing that survives");
  assert_eq!(issue.body, "the body that survives");
  assert_eq!(
    after.issue_list().len(),
    1,
    "the refused write must not have landed under any number"
  );
}

/// The same property for threads, because `st new` allocates its id the same
/// way and one ruling covers the mutation surface rather than one verb.
#[test]
fn two_facades_do_not_both_get_the_same_thread_id() {
  let fx = Fixture::new();
  let mut a = fx.facade_on_disk();
  let mut b = fx.facade_on_disk();

  let landed = a.st_new("the thread that survives").unwrap();
  let err = b
    .st_new("the thread that must not overwrite it")
    .expect_err("a create must refuse an id that is already taken");
  // **`ThreadExists` IS REACHABLE FOR THE FIRST TIME HERE.** `st_new` carried a
  // pre-check against its own just-read canon -- false by construction -- and
  // `error_remedies.rs` exempted the variant from its drive as needing "a
  // colliding id, which `st new` allocates around". The collision is real; the
  // place it was being looked for could not see it.
  assert!(
    matches!(&err, FacadeError::ThreadExists { id } if id == &landed),
    "the refusal must name the thread it hit, as its own variant: {err:?}"
  );

  let after = fx.facade_on_disk();
  assert_eq!(
    after.st_show(&landed).unwrap().title,
    "the thread that survives"
  );
}

/// **THE ARM THAT WOULD CATCH THE OBVIOUS WRONG FIX.** Dropping `ON CONFLICT`
/// outright shuts the create door and every other door with it: `issues close`
/// clones canon, edits `status`, and writes the whole row back through the same
/// statement.
#[test]
fn an_ordinary_change_still_writes_the_whole_row_back() {
  let fx = Fixture::new();
  let mut f = fx.facade_on_disk();

  let n = f
    .issue_add("an issue to close", None, None, "with a body")
    .unwrap();
  f.issue_close(n)
    .expect("closing an issue is a CHANGE, not a create, and must not refuse");

  let after = fx.facade_on_disk();
  let issue = after.issue_show(n).unwrap();
  assert_eq!(issue.status, IssueStatus::Closed);
  assert_eq!(
    issue.body, "with a body",
    "a change must carry the fields it does not set"
  );
  assert!(
    issue.closed.as_ref().is_some_and(|d| !d.is_empty()),
    "the close date is filled by the database inside the write"
  );

  f = fx.facade_on_disk();
  f.issue_open(n).expect("reopening is a change too");
}

/// **AND A WHOLE-ESTATE RESYNC STILL RELOADS ROWS IT ALREADY HOLDS.** `rebuild`
/// is the disk -> db direction and REPLACES the estate, so every row it writes
/// exists already by construction. Sending those through the create door would
/// refuse every resync there is -- the loudest possible regression, and the
/// reason `rebuild` names its door explicitly rather than inheriting one.
#[test]
fn a_resync_over_an_estate_the_store_already_holds_does_not_refuse() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  fx.write_issue(&sample_issue(1));

  // Twice. The first warms the store from the extract; the second is the one
  // that would refuse if a reload were read as a create.
  for pass in 1..=2 {
    let f = fx.facade_on_disk();
    assert_eq!(
      f.issue_list().len(),
      1,
      "pass {pass}: the resync must reload the estate it already holds"
    );
    assert!(f.st_show("ST0001").is_ok(), "pass {pass}");
  }
}
