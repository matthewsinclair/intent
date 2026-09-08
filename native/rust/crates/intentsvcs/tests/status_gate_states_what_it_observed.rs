//! `status-gate-disagreement` states the disagreement and does not infer a cause.
//!
//! **THE DETAIL USED TO END `so anything sequencing off this field is planning
//! work that is already done`.** That is an inference from SATISFIED CRITERIA
//! to FINISHED WORK, and the two come apart on parked scope: a park is recorded
//! as its own satisfied criterion, so the gate passes over work nobody has
//! started. Measured across the fleet 2026-09-08: **nine work packages carry a
//! park or defer marker and every live one is `not-started`** -- so every one of
//! them was being told its work was already done.
//!
//! **THE STRING WAS UNPINNED AND THAT IS WHY THIS FILE EXISTS.** Changing a
//! user-facing claim in `doctor.rs` left all 1222 intentsvcs tests green. A
//! sentence nothing asserts is a sentence anyone can quietly restore.
//!
//! **WHAT WOULD HAVE TO BE SEEN FOR THESE TO FAIL**, stated because an arm
//! reading "the detail does not contain X" passes trivially on a build that
//! emits no detail at all: each arm first pins that the fixture PRODUCES a
//! passing-direction finding and that the detail names the direction, and only
//! then asserts what the sentence stops short of claiming.

use crate::common::{Fixture, ctx};
use intentsvcs::finding::FindingClass;
use intentsvcs::model::{
  AcKind, AcState, AcceptanceTest, AtKind, AtStatus, Criterion, THREAD_SCHEMA, TShirt, Thread,
  ThreadStatus, WorkPackage, WpStatus,
};

/// A thread whose single criterion is satisfied by a green test, with its work
/// package recorded `NotStarted` -- the PASSING direction, which is the shape
/// parked work takes.
fn park_shaped() -> Thread {
  Thread {
    attachments: Vec::new(),
    body: String::new(),
    preamble: String::new(),
    schema: THREAD_SCHEMA.to_string(),
    id: "ST0001".to_string(),
    title: "A thread with a satisfied scope nobody has started".to_string(),
    slug: None,
    status: ThreadStatus::Wip,
    status_reason: None,
    fiat: None,
    created: "2026-08-14".to_string(),
    completed: None,
    acceptance: None,
    objective: "Be internally consistent.".to_string(),
    context: String::new(),
    related: vec![],
    wps: vec![WorkPackage {
      seq: 1,
      title: "PARKED -- criteria authored at unpark".to_string(),
      scope: Some(TShirt::S),
      scope_legacy: None,
      status: WpStatus::NotStarted,
      status_reason: None,
      fiat: None,
      objective: String::new(),
      body: String::new(),
      preamble: String::new(),
    }],
    criteria: vec![Criterion {
      id: "AC-01.1".to_string(),
      text: "the park is recorded".to_string(),
      kind: AcKind::Test,
      state: AcState::Computed,
    }],
    tests: vec![AcceptanceTest {
      fiat: None,
      id: "AT-01.1".to_string(),
      kind: AtKind::Test,
      file: Some("tests/clean.rs".to_string()),
      prose: None,
      covers: vec!["AC-01.1".to_string()],
      status: AtStatus::Green,
      note: None,
      legacy: None,
    }],
  }
}

#[test]
fn the_passing_direction_reports_the_disagreement_and_does_not_claim_the_work_is_done() {
  let fx = Fixture::new();
  fx.write_thread(&park_shaped());
  let project = fx.project();
  let canon = intentsvcs::ingest::read(&project).expect("fixture canon reads");
  intentsvcs::views::write_all(&project, &canon, &ctx()).expect("write views");

  let report = intentsvcs::doctor::diagnose(&project, &ctx(), None, intentsvcs::doctor::Scope::All);
  let hit: Vec<_> = report
    .findings
    .iter()
    .filter(|f| f.class == FindingClass::StatusGateDisagreement)
    .collect();

  // **PRECONDITION FIRST.** Without it every assertion below passes on a build
  // that reports nothing at all, which is the failure they exist to catch.
  assert_eq!(
    hit.len(),
    1,
    "the fixture must produce exactly one passing-direction finding or nothing below is being checked: {:?}",
    report.findings
  );
  assert!(
    hit[0].detail.contains("gate PASSES"),
    "and it must be the PASSING direction, not the blocked one: {}",
    hit[0].detail
  );

  // The claim it is allowed to make: the two records disagree.
  assert!(
    hit[0]
      .detail
      .contains("the status and the contract disagree"),
    "the detail states the disagreement it observed: {}",
    hit[0].detail
  );
  // The claim it is NOT allowed to make: that the work is finished.
  for forbidden in ["already done", "sequencing off this field"] {
    assert!(
      !hit[0].detail.contains(forbidden),
      "the detail must not infer FINISHED WORK from SATISFIED CRITERIA -- a park \
       satisfies its own criterion over work nobody has started; found `{forbidden}` in: {}",
      hit[0].detail
    );
  }
}

#[test]
fn the_remedy_covers_both_directions_because_one_class_carries_both() {
  let remedy = FindingClass::StatusGateDisagreement.remedy();

  // **THE CLASS CARRIES TWO OPPOSITE FINDINGS AND `remedy()` TAKES NO
  // ARGUMENT**, so one sentence serves both. It used to address only the
  // blocked direction -- read the blocking ids, do NOT reach for `wp done` --
  // which is precisely wrong advice on the passing half, where `wp done` is
  // the correct action when the field really is stale.
  for required in ["BLOCKED", "PASSES"] {
    assert!(
      remedy.contains(required),
      "the remedy must name the `{required}` direction it serves: {remedy}"
    );
  }
  assert!(
    remedy.to_lowercase().contains("park"),
    "and it must name the case where a passing gate is CORRECT rather than stale: {remedy}"
  );
  // The machine constraint, driven 2026-09-08: `wp.done` is declared only from
  // `wip`, so the passing-direction advice is wrong without it.
  assert!(
    remedy.contains("wp start"),
    "a NotStarted work package cannot be closed directly; the remedy must say so: {remedy}"
  );
}
