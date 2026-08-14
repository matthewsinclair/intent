//! AT-04.2 / AC-04.2: ac/at operations implement the four AC states, with
//! satisfaction COMPUTED for test-backed criteria (never stored) and inline
//! evidence for non-test ones.
//!
//! The computed-never-stored half is the one with teeth. It is easy to build a
//! facade that writes a `satisfied` flag on any criterion and reports success;
//! what makes it correct is that the flag has no effect on a test-backed
//! criterion and the facade refuses to pretend otherwise.

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::contract::{AcState, ac_state};
use intentsvcs::facade::FacadeError;
use intentsvcs::model::{AcScope, AtStatus};

fn state(facade: &intentsvcs::facade::Facade, st: &str, ac: &str) -> AcState {
  let thread = facade.st_show(st).expect("thread");
  let criterion = thread
    .criteria
    .iter()
    .find(|c| c.id == ac)
    .unwrap_or_else(|| panic!("no {ac}"));
  ac_state(thread, criterion)
}

#[test]
fn a_test_backed_criterion_is_satisfied_only_by_a_green_test() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();

  assert_eq!(state(&facade, "ST0056", "AC-03.1"), AcState::Satisfied);

  // Both covering ATs must go red -- the fixture has two, and satisfaction is
  // "ANY covering AT is green".
  facade.at_set("ST0056", "AT-03.1", AtStatus::Red).unwrap();
  assert_eq!(
    state(&facade, "ST0056", "AC-03.1"),
    AcState::Satisfied,
    "one covering AT is still green, so the criterion holds"
  );
  facade.at_set("ST0056", "AT-03.7", AtStatus::Red).unwrap();
  assert_eq!(state(&facade, "ST0056", "AC-03.1"), AcState::Unsatisfied);
}

/// The refusal that keeps satisfaction single-homed.
#[test]
fn satisfying_a_test_backed_criterion_directly_is_refused() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();

  let err = facade
    .ac_satisfy("ST0056", "AC-03.1", "I checked it myself")
    .expect_err("must refuse");
  match &err {
    FacadeError::ComputedSatisfaction { ac } => assert_eq!(ac, "AC-03.1"),
    other => panic!("expected ComputedSatisfaction, got: {other}"),
  }
  assert!(
    err.remedy().contains("at set"),
    "the remedy names the verb that WOULD work: {}",
    err.remedy()
  );
}

#[test]
fn a_non_test_criterion_carries_its_evidence_inline() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();

  facade
    .ac_satisfy("ST0056", "AC-03.2", "the render was reviewed at 476f1e1")
    .expect("satisfy");

  let thread = facade.st_show("ST0056").unwrap();
  let criterion = thread.criteria.iter().find(|c| c.id == "AC-03.2").unwrap();
  assert_eq!(criterion.satisfied, Some(true));
  assert_eq!(
    criterion.evidence.as_deref(),
    Some("the render was reviewed at 476f1e1")
  );
  assert!(
    fx.read("intent/st/ST0056/acceptance.md")
      .contains("the render was reviewed at 476f1e1"),
    "the evidence reaches the generated contract view"
  );
}

#[test]
fn descope_carries_its_target_and_reporter() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();

  facade
    .ac_descope("ST0056", "AC-03.1", "ST0057", Some("hv"), Some("moved"))
    .expect("descope");

  assert_eq!(state(&facade, "ST0056", "AC-03.1"), AcState::Descoped);
  let thread = facade.st_show("ST0056").unwrap();
  match &thread
    .criteria
    .iter()
    .find(|c| c.id == "AC-03.1")
    .unwrap()
    .scope
  {
    AcScope::Descoped { to, by, reason } => {
      assert_eq!(to, "ST0057");
      assert_eq!(by.as_deref(), Some("hv"));
      assert_eq!(reason.as_deref(), Some("moved"));
    }
    other => panic!("expected Descoped, got: {other:?}"),
  }
}

#[test]
fn withdraw_requires_a_reason_and_records_it() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();

  facade
    .ac_withdraw("ST0056", "AC-03.1", "the premise did not reproduce", None)
    .expect("withdraw");

  assert_eq!(state(&facade, "ST0056", "AC-03.1"), AcState::Withdrawn);
  assert!(
    fx.read("intent/st/ST0056/acceptance.md")
      .contains("WITHDRAWN: the premise did not reproduce"),
    "the reason is on the record, which is the whole point of withdraw over deletion"
  );
}

#[test]
fn reinstate_returns_a_criterion_to_scope() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();

  // The fixture ships AC-03.9 descoped and AC-03.8 withdrawn.
  facade.ac_reinstate("ST0056", "AC-03.9").expect("reinstate");
  assert_eq!(state(&facade, "ST0056", "AC-03.9"), AcState::Unsatisfied);
  facade.ac_reinstate("ST0056", "AC-03.8").expect("reinstate");
  assert_eq!(state(&facade, "ST0056", "AC-03.8"), AcState::Unsatisfied);
}

#[test]
fn reinstating_an_in_scope_criterion_is_refused() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();

  match facade.ac_reinstate("ST0056", "AC-03.1") {
    Err(FacadeError::NotOffScope { ac }) => assert_eq!(ac, "AC-03.1"),
    other => panic!("expected NotOffScope, got: {other:?}"),
  }
}

#[test]
fn a_no_op_scope_change_is_refused_rather_than_silently_accepted() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();

  facade
    .ac_withdraw("ST0056", "AC-03.1", "r", None)
    .expect("withdraw");
  match facade.ac_withdraw("ST0056", "AC-03.1", "r", None) {
    Err(FacadeError::ScopeUnchanged { ac, state }) => {
      assert_eq!(ac, "AC-03.1");
      assert_eq!(state, "withdrawn");
    }
    other => panic!("expected ScopeUnchanged, got: {other:?}"),
  }
}

#[test]
fn unknown_criteria_and_tests_are_refused_by_name() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();

  match facade.ac_satisfy("ST0056", "AC-99.9", "x") {
    Err(FacadeError::NoSuchCriterion { ac, st }) => {
      assert_eq!(ac, "AC-99.9");
      assert_eq!(st, "ST0056");
    }
    other => panic!("expected NoSuchCriterion, got: {other:?}"),
  }
  match facade.at_set("ST0056", "AT-99.9", AtStatus::Green) {
    Err(FacadeError::NoSuchTest { at, .. }) => assert_eq!(at, "AT-99.9"),
    other => panic!("expected NoSuchTest, got: {other:?}"),
  }
}

/// Satisfaction has no storage, so it cannot go stale. Changing an AT changes
/// the answer with nothing to update.
#[test]
fn satisfaction_has_no_second_home_to_go_stale() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();

  facade.at_set("ST0056", "AT-03.1", AtStatus::Red).unwrap();
  facade.at_set("ST0056", "AT-03.7", AtStatus::Red).unwrap();

  let canon = fx.read("intent/st/ST0056/thread.json");
  let value: serde_json::Value = serde_json::from_str(&canon).expect("parse canon");
  let criterion = value["criteria"]
    .as_array()
    .expect("criteria")
    .iter()
    .find(|c| c["id"] == "AC-03.1")
    .expect("AC-03.1");
  assert!(
    criterion.get("satisfied").is_none(),
    "a test-backed criterion carries no satisfied field in canon; there is nowhere for a stale answer to live: {criterion}"
  );
  assert!(
    fx.read("intent/st/ST0056/acceptance.md")
      .contains("AC-03.1 strict ingest refuses schema-invalid canon -- satisfied: no (computed)"),
    "the view reports the computed answer and says it is computed"
  );
}
