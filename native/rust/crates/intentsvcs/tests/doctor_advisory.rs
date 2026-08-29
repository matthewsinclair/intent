//! An advisory is printed, never counted -- hv, 2026-08-26.
//!
//! Baize printed 66 `AT-x.y carries a legacy reference and its thread is still
//! WIP -- ADVISORY, not a refusal` lines at exit 1 under the
//! `model-inconsistent` remedy ("the canon says two things that cannot both be
//! true"), which made a clean `doctor` unreachable on any live estate whose
//! AT rows still cite tests in the v2 `file::name` grammar. The text already
//! said "not a refusal"; the class, the count and the exit code said otherwise.
//! These arms pin the three of them to the text.
//!
//! The baseline is `doctor_checks.rs`'s clean thread, seeded the same way, and
//! the first arm is that baseline reporting nothing -- the positive control
//! without which "healthy after adding an advisory" would also pass on a
//! doctor that reports nothing at all.
mod common;

use common::{Fixture, ctx};
use intentsvcs::finding::FindingClass;
use intentsvcs::model::{
  AcKind, AcState, AcceptanceTest, AtKind, AtStatus, Criterion, Legacy, THREAD_SCHEMA, TShirt,
  Thread, ThreadStatus, WorkPackage, WpStatus,
};

fn clean_thread(id: &str) -> Thread {
  Thread {
    attachments: Vec::new(),
    body: String::new(),
    preamble: String::new(),
    schema: THREAD_SCHEMA.to_string(),
    id: id.to_string(),
    title: "A clean thread".to_string(),
    slug: None,
    status: ThreadStatus::Wip,
    status_reason: None,
    created: "2026-08-14".to_string(),
    completed: None,
    acceptance: None,
    objective: "Be internally consistent.".to_string(),
    context: String::new(),
    related: vec![],
    wps: vec![WorkPackage {
      seq: 1,
      title: "The only package".to_string(),
      scope: Some(TShirt::S),
      scope_legacy: None,
      status: WpStatus::Done,
      status_reason: None,
      objective: String::new(),
      body: String::new(),
      preamble: String::new(),
    }],
    criteria: vec![Criterion {
      id: "AC-01.1".to_string(),
      text: "the thing works".to_string(),
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

/// The v2 `file::name` citation the migration carries whole on a live thread.
fn with_a_legacy_reference(mut thread: Thread) -> Thread {
  let at = &mut thread.tests[0];
  at.file = None;
  at.legacy = Some(Legacy {
    raw: "apps/x/test/y_test.exs::a name with spaces".to_string(),
  });
  thread
}

fn seed(fx: &Fixture, thread: &Thread) {
  fx.write_thread(thread);
  let project = fx.project();
  let canon = intentsvcs::ingest::read(&project).expect("fixture canon reads");
  intentsvcs::views::write_all(&project, &canon, &ctx()).expect("write views");
}

fn diagnose(fx: &Fixture) -> intentsvcs::doctor::Report {
  intentsvcs::doctor::diagnose(&fx.project(), &ctx(), None)
}

#[test]
fn the_clean_baseline_reports_nothing() {
  let fx = Fixture::new();
  seed(&fx, &clean_thread("ST0001"));
  let report = diagnose(&fx);
  assert!(
    report.findings.is_empty(),
    "the baseline must be silent or every arm below is measuring noise: {:?}",
    report.findings
  );
  assert_eq!(report.advisories(), 0);
  assert!(report.is_healthy());
  assert_eq!(report.exit_code(), 0);
}

#[test]
fn a_legacy_reference_on_a_live_thread_is_an_advisory_and_the_report_is_healthy() {
  let fx = Fixture::new();
  seed(&fx, &with_a_legacy_reference(clean_thread("ST0001")));
  let report = diagnose(&fx);

  let advisories: Vec<_> = report
    .findings
    .iter()
    .filter(|f| f.class == FindingClass::Advisory)
    .collect();
  assert_eq!(
    advisories.len(),
    1,
    "exactly one advisory: {:?}",
    report.findings
  );
  assert!(
    advisories[0]
      .detail
      .contains("AT-01.1 carries a legacy reference")
      && advisories[0].detail.contains("ADVISORY, not a refusal"),
    "the advisory names the row and keeps the sentence that explains the carry policy: {:?}",
    advisories[0].detail
  );
  assert_eq!(
    report.findings.len(),
    1,
    "nothing else is reported for it -- in particular no model-inconsistent twin: {:?}",
    report.findings
  );
  assert_eq!(report.advisories(), 1);
  assert_eq!(report.actionable(), 0);
  assert!(
    report.is_healthy(),
    "advisories alone are healthy: {:?}",
    report.findings
  );
  assert_eq!(report.exit_code(), 0, "advisories alone exit 0");
}

#[test]
fn the_advisory_is_printed_under_its_own_word() {
  let fx = Fixture::new();
  seed(&fx, &with_a_legacy_reference(clean_thread("ST0001")));
  let report = diagnose(&fx);
  let rendered = report.findings[0].to_string();
  assert!(
    rendered.starts_with("advisory: "),
    "printed under its own word, not `residue:` -- a gate grepping `^residue:` must not see it: {rendered}"
  );
  assert!(
    rendered.contains("-- advisory --"),
    "the class is spelled on the line: {rendered}"
  );
  assert!(
    rendered.contains("remedy: nothing is owed now"),
    "the remedy says no action is owed: {rendered}"
  );
}

#[test]
fn a_real_finding_beside_an_advisory_still_fails_the_report() {
  let fx = Fixture::new();
  let mut thread = with_a_legacy_reference(clean_thread("ST0001"));
  // The contradiction doctor already names: a file citation AND a legacy one
  // on the same row. That is a fault, and it must keep failing the report
  // with an advisory standing right beside it.
  thread.tests[0].file = Some("tests/clean.rs".to_string());
  seed(&fx, &thread);
  let report = diagnose(&fx);

  assert!(
    report
      .findings
      .iter()
      .any(|f| f.class == FindingClass::ModelInconsistent
        && f.detail.contains("alternatives, not a pair")),
    "the pair conflict is still a finding: {:?}",
    report.findings
  );
  assert_eq!(report.advisories(), 1, "{:?}", report.findings);
  assert!(report.actionable() >= 1, "{:?}", report.findings);
  assert!(
    !report.is_healthy(),
    "a real finding is not hidden by an advisory beside it"
  );
  assert_eq!(report.exit_code(), 1);
}
