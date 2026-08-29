//! The fiat close obeys D42: `FiatRecord.at` goes in EMPTY and comes back
//! filled with what the database actually wrote.
//!
//! # Why this file exists rather than an arm in `fiat_state_serde`
//!
//! That file proves the record's SHAPE survives serde. This one proves the
//! record's TIME was not invented, which is a different failure and a silent
//! one: a `FiatRecord` carrying a plausible RFC 3339 string serialises,
//! round-trips and renders perfectly whether the value came from the store or
//! from a clock read in Rust. **Nothing about the artefact distinguishes them**,
//! so the only place the distinction is observable is against the event the
//! same mutation wrote.
//!
//! # The lever, and why equality is the assertion rather than a format check
//!
//! `at.len() == 24` and `ends_with('Z')` are true of any correctly-formatted
//! stamp from any source, so on their own they are a format test wearing a
//! provenance test's name -- they would pass unchanged if `ac_fc` called a
//! process clock. **The load-bearing assertion is that `at` EQUALS the `ac.fc`
//! event's own `ts`**, because the event's stamp is written by a column DEFAULT
//! inside the INSERT and nothing in the application can reproduce it.
//!
//! Two-sided by construction: remove the patch loop in `apply_with_state` and
//! `at` is the empty string it was handed in as; replace it with a Rust clock
//! and the two values differ. There is no third way to pass.

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::model::AcState;
use intentsvcs::remedy::Remedy;

const ST: &str = "ST0056";
/// Test-backed and `Computed` in the fixture -- one of the two states `ac.fc`
/// is declared from.
const OPEN: &str = "AC-03.1";
/// Non-test and `Satisfied` in the fixture. **Deliberately never fiat-closed
/// here**: it is the control that shows the stamping walk touches only what the
/// close touched.
const SATISFIED: &str = "AC-03.2";

fn loaded() -> (Fixture, intentsvcs::facade::Facade) {
  let fx = Fixture::new();
  let mut facade = fx.facade_on_disk();
  fx.write_thread(&sample_thread(ST));
  facade
    .sync_from_disk(&intentsvcs::sync::Scope::All)
    .expect("ingest");
  (fx, facade)
}

fn state(facade: &intentsvcs::facade::Facade, ac: &str) -> AcState {
  facade
    .st_show(ST)
    .expect("thread")
    .criteria
    .iter()
    .find(|c| c.id == ac)
    .expect("criterion")
    .state
    .clone()
}

#[test]
fn the_close_carries_the_time_the_database_wrote_on_its_own_event() {
  let (_fx, mut facade) = loaded();
  facade
    .ac_fc(ST, OPEN, "hv closed it on authority", "hv")
    .expect("fc");

  let AcState::Fiat(record) = state(&facade, OPEN) else {
    panic!("the close did not land in `fiat`");
  };

  assert!(
    !record.at.is_empty(),
    "`at` went in empty and came back empty, so nothing patched it from the write"
  );

  let events = facade.store().events().expect("events");
  let event = events
    .iter()
    .find(|e| e.op == "ac.fc")
    .expect("the close wrote its event");

  // THE ASSERTION. Everything above narrows; this one identifies the source.
  assert_eq!(
    record.at, event.ts,
    "the record's time must BE the database's stamp on this mutation's event, not a \
     separately-produced value that happens to look like one"
  );
}

#[test]
fn a_criterion_the_close_did_not_touch_gains_no_record() {
  // THE CONTROL for the walk. It runs over every criterion of every changed
  // thread on every mutation, so an arm asserting only that the closed row got
  // a stamp would pass just as well if the walk stamped all of them.
  let (_fx, mut facade) = loaded();
  facade
    .ac_fc(ST, OPEN, "hv closed it on authority", "hv")
    .expect("fc");

  assert!(
    matches!(state(&facade, SATISFIED), AcState::Satisfied { .. }),
    "a criterion nobody closed must be untouched by the close's stamping walk"
  );
}

#[test]
fn a_later_mutation_does_not_re_stamp_a_close_that_already_happened() {
  // **`is_empty()` IS THE SELECTOR AND THIS IS WHAT MAKES IT LOAD-BEARING.**
  // Without this arm the walk could stamp unconditionally and every other test
  // here would still pass -- and the defect would be invisible until a fiat
  // close silently claimed to have happened at the time of some unrelated
  // write days later. Same distinction `Stamp::CarriedFromTheExtract` draws one
  // layer down: recording that something happens NOW and transporting a record
  // of something that happened THEN are different acts.
  let (_fx, mut facade) = loaded();
  facade
    .ac_fc(ST, OPEN, "hv closed it on authority", "hv")
    .expect("fc");
  let AcState::Fiat(first) = state(&facade, OPEN) else {
    panic!("the close did not land in `fiat`");
  };

  facade
    .st_hold(ST, "an unrelated mutation, later")
    .expect("hold");

  let AcState::Fiat(after) = state(&facade, OPEN) else {
    panic!("the close did not survive an unrelated mutation");
  };
  assert_eq!(
    first.at, after.at,
    "an unrelated later write re-stamped a close that had already happened, rewriting when it \
     happened to the moment of a write that had nothing to do with it"
  );
}

#[test]
fn a_close_with_no_reason_refuses_and_writes_nothing() {
  // AC-00.1's second half. The refusal itself is the machine's
  // `Guard::ReasonRecorded` and is walked by `mutation_completeness`; what is
  // asserted here is the part that walk does not reach -- that the refusal left
  // the criterion where it found it.
  let (_fx, mut facade) = loaded();
  let before = state(&facade, OPEN);

  let refused = facade.ac_fc(ST, OPEN, "   ", "hv");
  assert!(
    refused.is_err(),
    "a fiat close with a blank reason must refuse: the reason is the whole record"
  );
  assert_eq!(
    before,
    state(&facade, OPEN),
    "the refusal must write NOTHING -- a criterion moved by a call that reported failure is worse \
     than one moved by a call that reported success"
  );
}

#[test]
fn a_second_close_refuses_and_names_the_undo() {
  // **ic DECLARED THIS IN THE DISPATCH ROW BEFORE IT WAS TRUE, and said so.**
  // The row read: `Fiat` is not a from-state, so a second `fc` refuses naming
  // `ac reinstate`. Driven, the machine refused correctly and the message named
  // no undo at all -- it recited the legal from-states. **A declaration nobody
  // drove is a requirement, not a record**, and this arm is what turns it into
  // one.
  let (_fx, mut facade) = loaded();
  facade
    .ac_fc(ST, OPEN, "the premise did not reproduce", "hv")
    .expect("fc");

  let again = facade
    .ac_fc(ST, OPEN, "closing it a second time", "hv")
    .expect_err("a criterion already closed on authority cannot be closed again");

  let rendered = again.to_string();
  assert!(
    rendered.contains("already fiat-closed"),
    "the refusal must say the requirement is already closed, not recite a from-state list: \
     {rendered}"
  );
  assert!(
    rendered.contains("the premise did not reproduce"),
    "**the STANDING reason belongs in the refusal**: the operator is about to replace one human \
     judgement with another and needs to see the one already on the record. Got: {rendered}"
  );
  assert!(
    again.remedy().contains("ac reinstate"),
    "the remedy must name the undo -- that is what makes the refusal actionable, and it is what \
     the dispatch row declares. Got: {}",
    again.remedy()
  );
}
