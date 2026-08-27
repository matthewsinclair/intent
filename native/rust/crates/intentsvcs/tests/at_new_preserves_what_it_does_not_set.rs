//! **`at new` on an existing id is a FULL REPLACEMENT, and every field the verb
//! does not set is a field it destroys.**
//!
//! `note` and `legacy` are the two. Both were built as `None` and then applied
//! over the stored row, so a re-create did not merely fail to set a note -- it
//! ATE one that was already there.
//!
//! # Why this is a trap rather than a wall
//!
//! The safe path was the undocumented one. A re-cite done as a canon edit plus
//! `intent sync --to-store` carries `file` and KEEPS `note`; the verb built for
//! the job was the one that lost it. So the operator who reached for the tool
//! lost data and the operator who hand-edited JSON did not, which is the wrong
//! way round and gives no signal at the moment of loss.
//!
//! Measured on this repository before the fix: six ST0061 AT notes destroyed by
//! a single re-cite, found by a peer and recovered from a git blob. **An AT
//! row's `note` is where a control and a known limitation live** -- lose it and
//! the row still reads as sound, which is why losing six strings was worse than
//! losing six strings.
//!
//! # What these tests assert, and what a weaker version would have missed
//!
//! **A ROW THAT SURVIVES IS NOT PROOF THAT THE NOTE DID.** Asserting the row is
//! still present, or that the thread still has N tests, stays green through
//! exactly the defect being fixed -- the row IS still there, carrying `None`.
//! So the note is asserted BY VALUE, against the string read before the write.
//!
//! And each arm reads its subject FIRST and fails if the fixture is not
//! carrying one. A preservation test whose fixture has nothing to preserve
//! passes vacuously and reads identically to one that works.

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::facade::Facade;
use intentsvcs::model::{AtKind, AtStatus};

fn row<'a>(facade: &'a Facade, at: &str) -> &'a intentsvcs::model::AcceptanceTest {
  facade.canon().threads[0]
    .tests
    .iter()
    .find(|t| t.id == at)
    .unwrap_or_else(|| panic!("{at} is missing from the fixture"))
}

/// Re-create `AT-03.1` -- which the fixture carries WITH a note -- changing the
/// fields the verb does set, and read the note back by value.
#[test]
fn a_re_create_keeps_the_note_it_was_never_given() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();

  let before = row(&facade, "AT-03.1")
    .note
    .clone()
    .expect("precondition: the fixture's AT-03.1 must CARRY a note, or this test proves nothing");
  assert!(
    !before.trim().is_empty(),
    "precondition: the carried note must have content"
  );

  facade
    .at_new(
      "ST0001",
      "AT-03.1",
      AtKind::Test,
      None,
      None,
      vec!["AC-03.1".to_string()],
      AtStatus::ToWrite,
      None,
    )
    .expect("re-creating an existing row is a legal PUT");

  assert_eq!(
    row(&facade, "AT-03.1").note.as_deref(),
    Some(before.as_str()),
    "`at new` replaced the row and dropped the note that was on it. The row is still present and \
     still passes lint -- which is why presence is not the thing to assert."
  );
}

/// The same question for `legacy`, whose loss is quieter: it is the field that
/// says a row was carried from a v2 estate, and a row that loses it looks like
/// one authored here.
#[test]
fn a_re_create_keeps_the_legacy_marker_it_was_never_given() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();

  let before = row(&facade, "AT-03.7")
    .legacy
    .clone()
    .expect("precondition: the fixture's AT-03.7 must CARRY a legacy marker");

  facade
    .at_new(
      "ST0001",
      "AT-03.7",
      AtKind::Test,
      None,
      None,
      vec!["AC-03.1".to_string()],
      AtStatus::ToWrite,
      None,
    )
    .expect("re-creating an existing row is a legal PUT");

  assert_eq!(
    row(&facade, "AT-03.7").legacy.as_ref(),
    Some(&before),
    "`at new` dropped the legacy marker, so a row carried from a v2 estate now reads as authored \
     here"
  );
}

/// **THE CONTROL, AND WITHOUT IT THE TWO ABOVE ARE SATISFIED BY A VERB THAT
/// INVENTS NOTES.** Carrying a value forward and fabricating one are
/// indistinguishable when every fixture row already has one, so a genuinely new
/// id must come out with nothing on it.
#[test]
fn a_genuinely_new_row_carries_no_note_from_anywhere() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();

  assert!(
    !facade.canon().threads[0]
      .tests
      .iter()
      .any(|t| t.id == "AT-03.4"),
    "precondition: AT-03.4 must be ABSENT"
  );

  facade
    .at_new(
      "ST0001",
      "AT-03.4",
      AtKind::Test,
      None,
      None,
      vec!["AC-03.1".to_string()],
      AtStatus::ToWrite,
      None,
    )
    .expect("the verb creates it");

  let made = row(&facade, "AT-03.4");
  assert_eq!(
    made.note, None,
    "a brand-new row came out carrying a note from somewhere -- preservation has become invention"
  );
  assert_eq!(
    made.legacy, None,
    "a brand-new row came out marked legacy, so it claims a v2 provenance it does not have"
  );
}

/// An explicit note OVERRIDES what was carried. The carry exists so that
/// SILENCE means "not saying"; it must not mean "cannot say".
#[test]
fn an_explicit_note_overrides_the_carried_one() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();

  let before = row(&facade, "AT-03.1")
    .note
    .clone()
    .expect("precondition: AT-03.1 carries a note to be overridden");

  facade
    .at_new(
      "ST0001",
      "AT-03.1",
      AtKind::Test,
      None,
      None,
      vec!["AC-03.1".to_string()],
      AtStatus::ToWrite,
      Some("the instrument cannot see the middle state".to_string()),
    )
    .expect("re-creating with a note is legal");

  let now = row(&facade, "AT-03.1").note.clone();
  assert_eq!(
    now.as_deref(),
    Some("the instrument cannot see the middle state"),
    "an explicit --note did not reach the row"
  );
  assert_ne!(
    now.as_deref(),
    Some(before.as_str()),
    "the carry won over an explicit note, so the flag cannot change an existing reason"
  );
}

/// **THE TRAP THIS FLAG WOULD HAVE FALLEN INTO.** `at_set` short-circuits when
/// the status is already the target -- correctly, because writing an envelope
/// for a movement that did not happen is what that guard was added to stop. But
/// annotating a row you are NOT moving is the common case, so the guard has to
/// separate nothing-to-do from nothing-to-do-about-the-status.
#[test]
fn a_note_lands_on_a_row_whose_status_does_not_move() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();

  let start = row(&facade, "AT-03.1").status;
  assert_eq!(
    start,
    AtStatus::Green,
    "precondition: AT-03.1 starts green, so setting it green is the self-loop this test needs"
  );

  facade
    .at_set(
      "ST0001",
      "AT-03.1",
      AtStatus::Green,
      Some("green on the first run is not evidence".to_string()),
    )
    .expect("a note-only write is legal");

  let after = row(&facade, "AT-03.1");
  assert_eq!(
    after.note.as_deref(),
    Some("green on the first run is not evidence"),
    "the note was discarded because the status had not moved -- the flag is inert in exactly the \
     case it is most wanted"
  );
  assert_eq!(
    after.status,
    AtStatus::Green,
    "a note-only write moved the status"
  );
}

/// The other side of that guard, which must NOT change: no note and no
/// movement is still nothing to do. Without this, widening the guard could
/// quietly reintroduce the self-loop envelope it exists to prevent.
#[test]
fn a_self_loop_with_no_note_is_still_nothing_to_do() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();

  let outcome = facade
    .at_set("ST0001", "AT-03.1", AtStatus::Green, None)
    .expect("a no-op self-loop is not an error");

  assert!(
    matches!(outcome, intentsvcs::facade::Outcome::AlreadyThere { .. }),
    "a self-loop carrying no note must still report AlreadyThere and write nothing, or the \
     envelope records a movement that did not happen: {outcome:?}"
  );
}
