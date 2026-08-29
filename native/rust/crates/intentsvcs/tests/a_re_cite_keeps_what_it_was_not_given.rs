//! **A RE-CITE KEEPS EVERY FIELD IT WAS NOT GIVEN**, and the door that promises
//! that is now `at edit` rather than `at new`.
//!
//! `note` and `legacy` are the two fields at stake. Under the old create-as-
//! replace they were built as `None` and applied over the stored row, so a
//! re-create did not merely fail to set a note -- it ATE one that was there.
//!
//! # Why this was a trap rather than a wall
//!
//! The safe path was the undocumented one. A re-cite done as a canon edit plus
//! `intent sync --to-store` carries `file` and KEEPS `note`; the verb built for
//! the job was the one that lost it. So the operator who reached for the tool
//! lost data and the operator who hand-edited JSON did not, which is the wrong
//! way round and gives no signal at the moment of loss.
//!
//! Measured on this repository: six ST0061 AT notes destroyed by a single
//! re-cite, found by a peer and recovered from a git blob. **An AT row's `note`
//! is where a control and a known limitation live** -- lose it and the row
//! still reads as sound, which is why losing six strings was worse than losing
//! six strings.
//!
//! # What changed, and why these tests moved doors rather than being deleted
//!
//! hv ruled 2026-08-28 that a create must FAIL on an existing key. `at new`
//! therefore cannot reach a stored row at all, and the carry it grew -- reading
//! the stored `note` and `legacy` forward -- became unreachable and was
//! removed. **The property did not stop mattering; it changed owner.** These
//! arms drive `at_edit`, where preservation is the verb's whole purpose rather
//! than a patch over the wrong door, and the two `at_new` arms that are still
//! meaningful (a genuinely new row, and the refusal itself) stay below as the
//! controls that keep the preservation claim honest.
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
use intentsvcs::facade::{Facade, FacadeError};
use intentsvcs::model::{AtKind, AtStatus};

fn row<'a>(facade: &'a Facade, at: &str) -> &'a intentsvcs::model::AcceptanceTest {
  facade.canon().threads[0]
    .tests
    .iter()
    .find(|t| t.id == at)
    .unwrap_or_else(|| panic!("{at} is missing from the fixture"))
}

/// Re-cite `AT-03.1` -- which the fixture carries WITH a note -- changing the
/// field the caller names, and read the note back by value.
#[test]
fn a_re_cite_keeps_the_note_it_was_never_given() {
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
    .at_edit(
      "ST0001",
      "AT-03.1",
      None,
      None,
      Some(vec!["AC-03.2".to_string()]),
    )
    .expect("re-citing coverage on an existing row is what this verb is for");

  let after = row(&facade, "AT-03.1");
  assert_eq!(
    after.note.as_deref(),
    Some(before.as_str()),
    "`at edit` replaced the row and dropped the note that was on it. The row is still present and \
     still passes lint -- which is why presence is not the thing to assert."
  );
  assert_eq!(
    after.covers,
    vec!["AC-03.2".to_string()],
    "the field that WAS named did not move, so this arm could pass without the verb doing anything"
  );
}

/// The same question for `legacy`, whose loss is quieter: it is the field that
/// says a row was carried from a v2 estate, and a row that loses it looks like
/// one authored here.
#[test]
fn a_re_cite_keeps_the_legacy_marker_it_was_never_given() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();

  let before = row(&facade, "AT-03.7")
    .legacy
    .clone()
    .expect("precondition: the fixture's AT-03.7 must CARRY a legacy marker");

  facade
    .at_edit(
      "ST0001",
      "AT-03.7",
      None,
      None,
      Some(vec!["AC-03.2".to_string()]),
    )
    .expect("re-citing coverage is legal");

  assert_eq!(
    row(&facade, "AT-03.7").legacy.as_ref(),
    Some(&before),
    "`at edit` dropped the legacy marker, so a row carried from a v2 estate now reads as authored \
     here"
  );
}

/// **THE FIELD-BY-FIELD CONTROL, AND WITHOUT IT THE TWO ABOVE ARE SATISFIED BY
/// A VERB THAT WRITES NOTHING AT ALL.** Preservation and inertia are
/// indistinguishable when the only assertions are about fields that did not
/// move, so this drives each editable field on its own and requires the named
/// one to change while its two neighbours hold.
#[test]
fn each_named_field_moves_and_its_neighbours_do_not() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));

  // `prose` on the non-test row, which the fixture gives a value and no file.
  let mut facade = fx.facade();
  let before = row(&facade, "AT-03.2").clone();
  assert!(
    before.prose.is_some() && before.file.is_none(),
    "precondition: AT-03.2 is the prose-bearing row"
  );
  facade
    .at_edit(
      "ST0001",
      "AT-03.2",
      None,
      Some("re-read against the rendered view".to_string()),
      None,
    )
    .expect("prose is editable");
  let after = row(&facade, "AT-03.2");
  assert_eq!(
    after.prose.as_deref(),
    Some("re-read against the rendered view"),
    "--prose did not reach the row"
  );
  assert_eq!(after.covers, before.covers, "--prose moved the coverage");
  assert_eq!(after.status, before.status, "--prose moved the status");
  assert_eq!(after.kind, before.kind, "--prose moved the kind");
}

/// **THE INVENTION CONTROL.** Carrying a value forward and fabricating one are
/// indistinguishable when every fixture row already has one, so a genuinely new
/// id must come out of `at new` with nothing on it.
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

/// **THE ARM THAT KEEPS THE MOVE HONEST.** These preservation claims are only
/// interesting because the create door can no longer reach a stored row; if
/// `at new` still replaced, this file would be testing the less dangerous of
/// two paths and reporting the estate safe.
#[test]
fn the_create_door_can_no_longer_reach_a_stored_row_at_all() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();

  let before = row(&facade, "AT-03.1").clone();
  let err = facade
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
    .expect_err("a create on a taken id must refuse");

  assert!(
    matches!(&err, FacadeError::TestExists { st, at } if st == "ST0001" && at == "AT-03.1"),
    "the refusal must name the key that is taken, not a generic write failure: {err}"
  );
  assert_eq!(
    row(&facade, "AT-03.1"),
    &before,
    "the refused create still changed the stored row -- a refusal that writes is worse than a \
     replace that admits it"
  );
}

/// **THE TRAP `--note` WOULD HAVE FALLEN INTO.** `at_set` short-circuits when
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
