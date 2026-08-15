//! AT-06.2 / AC-06.2: doctor as model/DB integrity queries plus file checks,
//! including the skew check and unparsed-state reporting.
//!
//! **Every model check gets its own fixture.** One estate carrying all seven
//! faults would pass a "doctor found something" assertion while six of the
//! seven checks were dead, and a dead check is indistinguishable from a clean
//! bill of health. The all-at-once case is asserted too, but for a different
//! property: that the report does not stop at the first finding.
//!
//! The load-bearing test in this file is
//! `doctor_runs_on_a_project_that_cannot_be_opened`. Doctor going through the
//! normal open path is not a small bug: a duplicate criterion id trips a
//! UNIQUE constraint during the DB load, so the command died with a SQLite
//! message before reporting anything -- while the tool's own remedy advised
//! running `intent doctor`. A doctor that only works on healthy projects is
//! not a doctor.

mod common;

use common::{Fixture, VERSION, ctx};
use intentsvcs::finding::{Finding, FindingClass};
use intentsvcs::model::{
  AcKind, AcState, AcceptanceTest, AtKind, AtStatus, Criterion, THREAD_SCHEMA, TShirt, Thread,
  ThreadStatus, WorkPackage, WpStatus,
};

/// A thread with nothing wrong with it: every reference resolves, and it names
/// no related thread, because the shared `sample_thread` cites ST0043/ST0044
/// which do not exist in a one-thread fixture.
fn clean_thread(id: &str) -> Thread {
  Thread {
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
      scope: TShirt::S,
      status: WpStatus::Wip,
      status_reason: None,
      objective: String::new(),
      body: String::new(),
    }],
    criteria: vec![Criterion {
      id: "AC-01.1".to_string(),
      text: "the thing works".to_string(),
      kind: AcKind::Test,
      state: AcState::Computed,
    }],
    tests: vec![AcceptanceTest {
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

/// Write a thread, then bring the generated views into step so a test about
/// MODEL faults is not also reporting skew it created itself.
fn seed(fx: &Fixture, thread: &Thread) {
  fx.write_thread(thread);
  let project = fx.project();
  let canon = intentsvcs::ingest::read(&project).expect("fixture canon reads");
  intentsvcs::views::write_all(&project, &canon, &ctx()).expect("write views");
}

fn run(fx: &Fixture) -> Vec<Finding> {
  intentsvcs::doctor::diagnose(&fx.project(), &ctx()).findings
}

/// The detail texts, for asserting that a specific check fired.
fn details(findings: &[Finding]) -> String {
  findings
    .iter()
    .map(std::string::ToString::to_string)
    .collect::<Vec<_>>()
    .join("\n")
}

// ---------------------------------------------------------------------------
// The clean bill of health -- the control for every test below
// ---------------------------------------------------------------------------

#[test]
fn a_consistent_project_reports_nothing() {
  let fx = Fixture::new();
  seed(&fx, &clean_thread("ST0001"));

  let report = intentsvcs::doctor::diagnose(&fx.project(), &ctx());
  assert!(
    report.is_healthy(),
    "a clean estate must report nothing, or every finding below is noise: {}",
    details(&report.findings)
  );
  assert_eq!(report.exit_code(), 0);
  assert_eq!(report.threads_checked, 1);
  assert!(
    report.views_checked > 0 && report.files_checked > 0,
    "a clean report must say what it COVERED -- 'nothing found' over an estate nobody read is the same sentence as a real pass: {report:?}"
  );
}

// ---------------------------------------------------------------------------
// Model integrity, one fixture per check
// ---------------------------------------------------------------------------

#[test]
fn an_acceptance_test_covering_a_criterion_that_does_not_exist_is_found() {
  let fx = Fixture::new();
  let mut thread = clean_thread("ST0001");
  thread.tests[0].covers = vec!["AC-99.9".to_string()];
  seed(&fx, &thread);

  let found = details(&run(&fx));
  assert!(
    found.contains("AT-01.1 covers AC-99.9"),
    "the gate reads `covers` to decide satisfaction, so a typo'd id is a test that proves nothing while looking like one that does: {found}"
  );
}

#[test]
fn an_acceptance_test_covering_nothing_is_found() {
  let fx = Fixture::new();
  let mut thread = clean_thread("ST0001");
  thread.tests[0].covers = vec![];
  seed(&fx, &thread);

  assert!(details(&run(&fx)).contains("covers nothing"));
}

#[test]
fn a_duplicate_criterion_id_within_a_thread_is_found() {
  let fx = Fixture::new();
  let mut thread = clean_thread("ST0001");
  let mut dup = thread.criteria[0].clone();
  dup.text = "a second criterion wearing the same id".to_string();
  dup.kind = AcKind::NonTest;
  thread.criteria.push(dup);
  seed(&fx, &thread);

  let findings = run(&fx);
  assert!(
    findings
      .iter()
      .any(|f| f.class == FindingClass::DuplicateId),
    "the schema constrains each element, never the collection, so a repeated id validates fine: {}",
    details(&findings)
  );
}

#[test]
fn a_criterion_in_a_group_with_no_work_package_is_found() {
  let fx = Fixture::new();
  let mut thread = clean_thread("ST0001");
  thread.criteria[0].id = "AC-07.1".to_string();
  thread.tests[0].covers = vec!["AC-07.1".to_string()];
  seed(&fx, &thread);

  assert!(details(&run(&fx)).contains("AC-07.1 belongs to WP-07"));
}

#[test]
fn a_thread_level_group_is_not_mistaken_for_a_missing_work_package() {
  let fx = Fixture::new();
  let mut thread = clean_thread("ST0001");
  thread.criteria[0].id = "AC-00.1".to_string();
  thread.tests[0].covers = vec!["AC-00.1".to_string()];
  seed(&fx, &thread);

  let found = details(&run(&fx));
  assert!(
    !found.contains("WP-00"),
    "group 00 is thread-level and always legitimate: {found}"
  );
}

#[test]
fn a_test_backed_criterion_carrying_stored_satisfaction_is_found() {
  let fx = Fixture::new();
  let mut thread = clean_thread("ST0001");
  thread.criteria[0].state = intentsvcs::model::AcState::Satisfied {
    evidence: "hand-authored on a test-backed AC, which canon can be".to_string(),
  };
  seed(&fx, &thread);

  assert!(details(&run(&fx)).contains("double truth"));
}

#[test]
fn a_criterion_descoped_to_a_thread_that_does_not_exist_is_found() {
  let fx = Fixture::new();
  let mut thread = clean_thread("ST0001");
  thread.criteria[0].state = AcState::Descoped {
    to: "ST4242".to_string(),
    by: None,
    reason: None,
  };
  seed(&fx, &thread);

  let found = details(&run(&fx));
  assert!(
    found.contains("descoped to ST4242"),
    "a requirement moved somewhere that does not exist is held by nobody: {found}"
  );
}

#[test]
fn a_related_thread_that_does_not_exist_is_found() {
  let fx = Fixture::new();
  let mut thread = clean_thread("ST0001");
  thread.related = vec![intentsvcs::model::Related {
    id: "ST9999".to_string(),
    note: None,
  }];
  seed(&fx, &thread);

  assert!(details(&run(&fx)).contains("names ST9999 as related"));
}

#[test]
fn completion_and_status_must_agree_in_both_directions() {
  let fx = Fixture::new();
  let mut thread = clean_thread("ST0001");
  thread.status = ThreadStatus::Completed;
  thread.completed = None;
  seed(&fx, &thread);
  assert!(
    details(&run(&fx)).contains("Completed with no completion date"),
    "direction one"
  );

  let fx2 = Fixture::new();
  let mut other = clean_thread("ST0001");
  other.status = ThreadStatus::Wip;
  other.completed = Some("2026-08-14".to_string());
  seed(&fx2, &other);
  assert!(
    details(&run(&fx2)).contains("carries a completion date"),
    "direction two -- checking only the first would let a WIP thread claim a completion date"
  );
}

// ---------------------------------------------------------------------------
// File checks
// ---------------------------------------------------------------------------

#[test]
fn a_hand_edited_generated_view_is_reported_as_skew() {
  let fx = Fixture::new();
  seed(&fx, &clean_thread("ST0001"));
  let view = fx.path("intent/st/ST0001/info.md");
  let edited = format!(
    "{}\n<!-- a hand edit -->\n",
    fx.read("intent/st/ST0001/info.md")
  );
  std::fs::write(&view, edited).expect("hand-edit the view");

  let findings = run(&fx);
  assert!(
    findings.iter().any(|f| f.class == FindingClass::ViewSkew),
    "a hand edit is CAUGHT, never silently outvoted: {}",
    details(&findings)
  );
}

#[test]
fn a_file_with_conflict_markers_is_reported_unparsed() {
  let fx = Fixture::new();
  seed(&fx, &clean_thread("ST0001"));
  fx.write_prose(
    "ST0001",
    "design.md",
    "# Design\n\n<<<<<<< HEAD\nours\n=======\ntheirs\n>>>>>>> branch\n",
  );

  let findings = run(&fx);
  assert!(
    findings
      .iter()
      .any(|f| f.class == FindingClass::ConflictMarkers),
    "v2 grepped straight through conflict markers; v3 reports them: {}",
    details(&findings)
  );
}

// ---------------------------------------------------------------------------
// Robustness -- the reason this command exists
// ---------------------------------------------------------------------------

/// **The load-bearing test.** Doctor must work on a project nothing else can
/// open, because that is exactly when someone reaches for it.
#[test]
fn doctor_runs_on_a_project_that_cannot_be_opened() {
  let fx = Fixture::new();
  let mut thread = clean_thread("ST0001");
  let mut dup = thread.criteria[0].clone();
  dup.kind = AcKind::NonTest;
  thread.criteria.push(dup);
  seed(&fx, &thread);

  // The control: the ordinary path genuinely cannot open this estate, so the
  // test below is not passing because the fault was too mild to matter.
  let opened = intentsvcs::facade::Facade::open_in_memory(fx.project(), common::facade_ctx());
  assert!(
    opened.is_err(),
    "precondition: a duplicate criterion id must defeat the normal open path"
  );

  let report = intentsvcs::doctor::diagnose(&fx.project(), &ctx());
  assert!(
    !report.is_healthy(),
    "doctor ran and reported on an estate the facade could not open"
  );
  let found = details(&report.findings);
  assert!(
    found.contains("cannot be loaded into a store"),
    "and it names the DB failure as a diagnosis rather than dying of it: {found}"
  );
  assert!(
    found.contains("declared more than once"),
    "and still reports the model fault that CAUSED it: {found}"
  );
}

/// A COLD cache is the normal state and must never be a finding; a STALE one
/// must be.
///
/// `intent/.cache/` is gitignored (D21), so an empty store is what every fresh
/// clone has. The first version of the DB check reported it, which would have
/// fired on the commonest healthy state there is -- and a health check that
/// cries wolf is one nobody reads. The second half of this test is what stops
/// the narrowing from quietly disabling the check altogether.
#[test]
fn a_cold_cache_is_healthy_and_a_stale_one_is_not() {
  let fx = Fixture::new();
  seed(&fx, &clean_thread("ST0001"));

  assert!(
    intentsvcs::doctor::diagnose(&fx.project(), &ctx()).is_healthy(),
    "a project whose on-disk cache was never written is healthy"
  );

  // Populate the on-disk cache, then move the canon out from under it.
  intentsvcs::facade::Facade::open(fx.project(), common::facade_ctx()).expect("populate the cache");
  let mut moved = clean_thread("ST0001");
  moved.title = "A different title entirely".to_string();
  fx.write_thread(&moved);

  let findings = run(&fx);
  assert!(
    findings
      .iter()
      .any(|f| f.file.contains("intent.db") && f.detail.contains("does not match a rebuild")),
    "a populated cache disagreeing with canon is a real diagnosis -- doctor does not open the facade, so it can still SEE the staleness rather than silently repairing it: {}",
    details(&findings)
  );
}

/// Canon that will not parse becomes findings, not a crash.
#[test]
fn unreadable_canon_becomes_findings() {
  let fx = Fixture::new();
  fx.write_raw_thread("ST0001", "{ this is not json");

  let report = intentsvcs::doctor::diagnose(&fx.project(), &ctx());
  assert!(!report.is_healthy());
  assert!(
    report
      .findings
      .iter()
      .any(|f| f.class == FindingClass::MalformedJson),
    "{}",
    details(&report.findings)
  );
}

/// The report never stops at the first finding.
#[test]
fn every_fault_is_reported_in_one_pass() {
  let fx = Fixture::new();
  let mut thread = clean_thread("ST0001");
  thread.status = ThreadStatus::Completed;
  thread.tests[0].covers = vec!["AC-99.9".to_string()];
  thread.related = vec![intentsvcs::model::Related {
    id: "ST9999".to_string(),
    note: None,
  }];
  thread.criteria[0].state = intentsvcs::model::AcState::Satisfied {
    evidence: "hand-authored on a test-backed AC, which canon can be".to_string(),
  };
  seed(&fx, &thread);

  let findings = run(&fx);
  let found = details(&findings);
  for expected in [
    "Completed with no completion date",
    "covers AC-99.9",
    "names ST9999 as related",
    "double truth",
  ] {
    assert!(
      found.contains(expected),
      "one fix-and-rerun cycle, not one per defect -- missing {expected:?} in:\n{found}"
    );
  }
}

/// Doctor repairs nothing. A report about a state the reporter has already
/// changed describes something that no longer exists.
#[test]
fn doctor_changes_nothing_it_looks_at() {
  let fx = Fixture::new();
  seed(&fx, &clean_thread("ST0001"));
  let view = fx.path("intent/st/ST0001/info.md");
  std::fs::write(&view, "deliberately skewed\n").expect("skew the view");

  let first = run(&fx);
  assert!(!first.is_empty(), "precondition: there is something to fix");
  assert_eq!(
    fx.read("intent/st/ST0001/info.md"),
    "deliberately skewed\n",
    "doctor did not rewrite the file it reported on"
  );

  let second = run(&fx);
  assert_eq!(
    details(&first),
    details(&second),
    "running twice reports the same thing, because the first run repaired nothing"
  );
}

/// The version reaches the renderer, so skew is judged against the same
/// banner the tool writes -- not against a default that would make every view
/// look skewed.
#[test]
fn skew_is_judged_against_the_rendering_version() {
  let fx = Fixture::new();
  seed(&fx, &clean_thread("ST0001"));

  let other = intentsvcs::views::RenderContext {
    version: "9.9.9-not-the-fixture-version",
    todo_watermark: None,
  };
  let findings = intentsvcs::doctor::diagnose(&fx.project(), &other).findings;
  assert!(
    findings.iter().any(|f| f.class == FindingClass::ViewSkew),
    "a different version renders a different banner, which IS skew: {}",
    details(&findings)
  );
  assert_ne!(
    VERSION, "9.9.9-not-the-fixture-version",
    "the two versions must actually differ or this proves nothing"
  );
}
