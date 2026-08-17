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
use intentsvcs::contract::{Resolved, resolve};
use intentsvcs::facade::{FacadeError, Outcome};
use intentsvcs::model::{AcState, AtStatus};
use intentsvcs::remedy::Remedy;

fn state(facade: &intentsvcs::facade::Facade, st: &str, ac: &str) -> Resolved {
  let thread = facade.st_show(st).expect("thread");
  let criterion = thread
    .criteria
    .iter()
    .find(|c| c.id == ac)
    .unwrap_or_else(|| panic!("no {ac}"));
  resolve(thread, criterion)
}

#[test]
fn a_test_backed_criterion_is_satisfied_only_by_a_green_test() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();

  assert_eq!(state(&facade, "ST0056", "AC-03.1"), Resolved::Satisfied);

  // Both covering ATs must go red -- the fixture has two, and satisfaction is
  // "ANY covering AT is green".
  facade.at_set("ST0056", "AT-03.1", AtStatus::Red).unwrap();
  assert_eq!(
    state(&facade, "ST0056", "AC-03.1"),
    Resolved::Satisfied,
    "one covering AT is still green, so the criterion holds"
  );
  facade.at_set("ST0056", "AT-03.7", AtStatus::Red).unwrap();
  assert_eq!(state(&facade, "ST0056", "AC-03.1"), Resolved::Unsatisfied);
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

  // The fixture's AC-03.2 arrives satisfied, and since the AC verbs began
  // enforcing the declared graph, satisfying a satisfied criterion is a
  // refusal rather than a rewrite. Withdrawing the claim first is the honest
  // route and exercises the round trip the evidence has to survive.
  facade
    .ac_unsatisfy("ST0056", "AC-03.2")
    .expect("unsatisfy first");
  facade
    .ac_satisfy("ST0056", "AC-03.2", "the render was reviewed at 476f1e1")
    .expect("satisfy");

  let thread = facade.st_show("ST0056").unwrap();
  let criterion = thread.criteria.iter().find(|c| c.id == "AC-03.2").unwrap();
  assert!(
    matches!(criterion.state, AcState::Satisfied { .. }),
    "the recorded state IS the satisfaction now -- there is no separate flag to check"
  );
  assert_eq!(
    criterion.state.evidence(),
    Some("the render was reviewed at 476f1e1"),
    "and the evidence lives INSIDE `Satisfied`, so it cannot outlive it"
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
  // The target has to EXIST -- the ratified machine guards `ac descope` with
  // "target thread exists" and the guard is now enforced.
  fx.write_thread(&sample_thread("ST0057"));
  let mut facade = fx.facade();

  facade
    .ac_descope("ST0056", "AC-03.1", "ST0057", Some("hv"), Some("moved"))
    .expect("descope");

  assert_eq!(state(&facade, "ST0056", "AC-03.1"), Resolved::Descoped);
  let thread = facade.st_show("ST0056").unwrap();
  match &thread
    .criteria
    .iter()
    .find(|c| c.id == "AC-03.1")
    .unwrap()
    .state
  {
    AcState::Descoped { to, by, reason } => {
      assert_eq!(to, "ST0057");
      assert_eq!(by.as_deref(), Some("hv"));
      assert_eq!(reason.as_deref(), Some("moved"));
    }
    other => panic!("expected Descoped, got: {other:?}"),
  }
}

/// **Renamed from `withdraw_requires_a_reason_and_records_it`, which claimed a
/// requirement it never drove.** It passes a reason and checks it reaches the
/// view -- the RECORDS half. Nothing here ever withheld one, so the requirement
/// half was asserted by the name alone, and reading the test list told you the
/// guard was covered when the guard did not exist.
/// `withdrawing_a_criterion_with_a_blank_reason_is_refused` now carries it.
#[test]
fn withdraw_records_its_reason_in_the_view() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();

  facade
    .ac_withdraw("ST0056", "AC-03.1", "the premise did not reproduce", None)
    .expect("withdraw");

  assert_eq!(state(&facade, "ST0056", "AC-03.1"), Resolved::Withdrawn);
  assert!(
    fx.read("intent/st/ST0056/acceptance.md")
      .contains("WITHDRAWN: the premise did not reproduce"),
    "the reason is on the record, which is the whole point of withdraw over deletion"
  );
}

/// `rescope` undoes a descope; `reinstate` undoes a withdrawal. Each returns
/// the criterion to scope, unsatisfied.
#[test]
fn rescope_and_reinstate_each_return_their_own_state_to_scope() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();

  // The fixture ships AC-03.9 descoped and AC-03.8 withdrawn.
  facade.ac_rescope("ST0056", "AC-03.9").expect("rescope");
  assert_eq!(state(&facade, "ST0056", "AC-03.9"), Resolved::Unsatisfied);
  facade.ac_reinstate("ST0056", "AC-03.8").expect("reinstate");
  assert_eq!(state(&facade, "ST0056", "AC-03.8"), Resolved::Unsatisfied);
}

/// **The two are not aliases**, and each refusal names the other verb.
///
/// v2 enforces this (`bin/intent_acceptance:1241` and `:1246`) and it is not
/// pedantry: a descoped requirement still exists on another thread, and a
/// withdrawn one does not exist at all. A single verb that undid whichever it
/// found would answer "done" to a question nobody asked -- and the first
/// version of this facade had exactly that, because it was designed from what
/// the two commands looked like rather than from what v2 does.
#[test]
fn rescope_and_reinstate_refuse_each_others_states_and_name_the_right_verb() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();

  // AC-03.9 is DESCOPED, so reinstate must refuse and point at rescope.
  match facade.ac_reinstate("ST0056", "AC-03.9") {
    Err(e @ FacadeError::WrongOffScopeState { .. }) => {
      let rendered = e.render();
      assert!(rendered.contains("descoped, not withdrawn"), "{rendered}");
      assert!(
        rendered.contains("ac rescope"),
        "the remedy names the verb that WOULD work: {rendered}"
      );
    }
    other => panic!("expected WrongOffScopeState, got: {other:?}"),
  }

  // AC-03.8 is WITHDRAWN, so rescope must refuse and point at reinstate.
  match facade.ac_rescope("ST0056", "AC-03.8") {
    Err(e @ FacadeError::WrongOffScopeState { .. }) => {
      let rendered = e.render();
      assert!(rendered.contains("withdrawn, not descoped"), "{rendered}");
      assert!(rendered.contains("ac reinstate"), "{rendered}");
    }
    other => panic!("expected WrongOffScopeState, got: {other:?}"),
  }

  // And neither refusal changed anything.
  assert_eq!(state(&facade, "ST0056", "AC-03.9"), Resolved::Descoped);
  assert_eq!(state(&facade, "ST0056", "AC-03.8"), Resolved::Withdrawn);
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

  assert_eq!(
    facade
      .ac_withdraw("ST0056", "AC-03.1", "r", None)
      .expect("withdraw"),
    Outcome::Moved
  );

  // **This asserted `ScopeUnchanged` -- an ERROR -- until hv ruled self-loops
  // legal (2026-08-17), and the variant is pruned rather than deprecated.**
  // Asking for the state the criterion is already in is not a movement, so it is
  // accepted and reported. The assertion is on the OUTCOME rather than on `Ok`,
  // because `Ok(Moved)` here would mean a second withdrawal was recorded -- a
  // duplicate envelope for one decision, stamped at a second time under D42.
  // **Asserted through `already()` rather than against the bare variant, so the
  // STATE is checked too** (issue 0050). `AlreadyThere` now carries the state the
  // entity is in, and a no-op that reported the wrong state would have satisfied
  // the previous form of this assertion.
  assert_eq!(
    facade
      .ac_withdraw("ST0056", "AC-03.1", "r", None)
      .expect("a self-loop is accepted, not refused")
      .already(),
    Some("withdrawn"),
    "a repeated withdrawal must be a NO-OP: accepted, reported as the state it is already in, and nothing written"
  );
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

/// **The guard the ratified machine declared and nothing enforced.**
///
/// `doctor` already reported the resulting state -- "descoped to X, which is
/// not a steel thread in this project" -- so the estate was DETECTING a
/// condition it could refuse. Detection after the fact is the reminder-shaped
/// thing D33 rules against, and the row was in the ratified table all along.
///
/// The discriminating half is the pair: the same call succeeds once the target
/// exists, so this is a guard rather than a blanket refusal.
#[test]
fn descoping_to_a_thread_that_does_not_exist_is_refused() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();

  let refused = facade
    .ac_descope("ST0056", "AC-03.2", "ST9999", None, None)
    .expect_err("the target does not exist");
  assert!(
    matches!(refused, FacadeError::DescopeTargetMissing { .. }),
    "got {refused:?}"
  );
  assert!(
    refused.remedy().contains("ST9999"),
    "the remedy names the thread that has to exist first: {}",
    refused.remedy()
  );
  assert_eq!(
    state(&facade, "ST0056", "AC-03.2"),
    Resolved::Satisfied,
    "and nothing moved -- the fixture criterion is satisfied and a refused mutation changes nothing"
  );

  fx.write_thread(&sample_thread("ST9999"));
  let mut facade = fx.facade();
  facade
    .ac_descope("ST0056", "AC-03.2", "ST9999", None, None)
    .expect("the same call, once the target exists");
}

/// **Evidence that is not there is not evidence, and `ac.satisfy` took it.**
///
/// ic chased this to the facade and could not execute the last two links, so
/// this is those links run rather than read (their 19:26Z note). The reason it
/// is worse than an ordinary missing-validation defect is `contract.rs`'s own
/// header: evidence is a human judgement with NO GREEN TO READ. A non-test
/// criterion has evidence precisely because there is no test to run, so
/// satisfied-with-nothing is the one state the design exists to make
/// impossible, on the one verb whose whole job is recording that a criterion
/// was met.
///
/// **Blank as well as empty, because the CLI cannot tell them apart.** A shell
/// makes `--evidence ""` and `--evidence "  "` the same gesture, and a guard
/// that refuses one and stores the other is a guard that teaches its own
/// bypass.
#[test]
fn satisfying_a_criterion_with_no_evidence_is_refused() {
  for blank in ["", "   ", "\t\n "] {
    let fx = Fixture::new();
    fx.write_thread(&sample_thread("ST0056"));
    let mut facade = fx.facade();
    facade
      .ac_unsatisfy("ST0056", "AC-03.2")
      .expect("unsatisfy first");

    let Err(refused) = facade.ac_satisfy("ST0056", "AC-03.2", blank) else {
      panic!("evidence {blank:?} was accepted as evidence");
    };
    assert!(
      refused.remedy().contains("evidence"),
      "the remedy says what is missing: {}",
      refused.remedy()
    );
    assert_eq!(
      state(&facade, "ST0056", "AC-03.2"),
      Resolved::Unsatisfied,
      "and the refusal changed nothing -- {blank:?} left the criterion where it was"
    );
  }

  // The discriminating half: the same call with evidence succeeds, so this is
  // a guard and not a blanket refusal of the verb.
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();
  facade
    .ac_unsatisfy("ST0056", "AC-03.2")
    .expect("unsatisfy first");
  facade
    .ac_satisfy("ST0056", "AC-03.2", "reviewed at 53cb9f00")
    .expect("the same call, with evidence");
}

/// **The second guard the ratified machine declares and nothing enforces.**
///
/// `ac.withdraw` is `Guard::ReasonRecorded` in the ratified AC machine and in
/// the transcription -- and `set_ac_state` consults the declaration for the
/// FROM-STATE only, so the guard column has never been read for a criterion.
/// Exactly the shape of `descoping_to_a_thread_that_does_not_exist_is_refused`
/// above, in the same field's table, found while confirming ic's evidence
/// defect.
///
/// **Nothing could have caught it**: the blank-reason guard test in
/// `mutation_completeness.rs` loops `Thread` and `WorkPackage` and stops --
/// `Criterion` is not in the list, so the one entity whose guards go unread is
/// also the one the guard test does not visit.
#[test]
fn withdrawing_a_criterion_with_a_blank_reason_is_refused() {
  for blank in ["", "   "] {
    let fx = Fixture::new();
    fx.write_thread(&sample_thread("ST0056"));
    let mut facade = fx.facade();

    let refused = facade
      .ac_withdraw("ST0056", "AC-03.2", blank, Some("hv"))
      .expect_err("a withdrawal with no reason is a withdrawal nobody can review");
    assert!(
      matches!(refused, FacadeError::ReasonRequired { .. }),
      "the declared guard is ReasonRecorded, so its refusal is the declared one: got {refused:?}"
    );
    assert_eq!(
      state(&facade, "ST0056", "AC-03.2"),
      Resolved::Satisfied,
      "and nothing moved"
    );
  }
}

// **The other door is shut in `ingest_refusal.rs`**, where the refusal helper
// and the finding-class assertions live: a `thread.json` carrying a
// satisfaction with empty evidence is refused as schema-invalid before it can
// become a model. Both points, because neither implies the other -- the guard
// above shuts the door a VERB comes through, and canon is hand-authorable.
// This is the estate's settled shape for a rule of this kind rather than a new
// one: `Criterion`'s `kind`/`state` invariant is enforced at exactly those two
// places plus `doctor` (vc, 2026-08-15).
