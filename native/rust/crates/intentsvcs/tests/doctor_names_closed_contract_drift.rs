//! Issue 0665: a green AT row on a COMPLETED thread whose cited file has lost
//! the row's literal id is named by `doctor`, and nothing reopens.
//!
//! **THE CLOSE GATE'S L3 EXEMPTION STAYS.** These arms hold both halves of
//! hv's shape, relayed by devbin-vc on 2026-11-10. `doctor --scope closed`
//! names the row, and the gate and `at lint` (one `contract_report`) still pass
//! it, so the thread is not reopened by anything the fix adds.
//!
//! **EVERY ARM CAN FAIL ON THE FIXTURE.** The dropped row's file exists, so it
//! is not L2's missing file, and it lacks the id, so it is exactly the row the
//! exemption skips. The kept row carries its id, so an arm that named every
//! green row would fail the count. The live twin carries the same dropped row,
//! so an arm that ignored the thread's status would name it too.

use crate::common::{Fixture, ctx};
use intentsvcs::contract::{RepoFiles, completed_thread_drift, contract_report};
use intentsvcs::doctor::{Report, Scope};
use intentsvcs::finding::FindingClass;
use intentsvcs::model::{
  AcKind, AcState, AcceptanceTest, AtKind, AtStatus, Criterion, THREAD_SCHEMA, TShirt, Thread,
  ThreadStatus, WorkPackage, WpStatus,
};

const KEPT: &str = "tests/kept.rs";
const DROPPED: &str = "tests/dropped.rs";
const DROPPED_ROW: &str = "AT-01.2";

fn row(id: &str, file: &str, status: AtStatus) -> AcceptanceTest {
  AcceptanceTest {
    fiat: None,
    id: id.to_string(),
    kind: AtKind::Test,
    file: Some(file.to_string()),
    prose: None,
    covers: vec!["AC-01.1".to_string()],
    status,
    note: None,
    legacy: None,
  }
}

/// A thread whose kept row carries its id, whose dropped row cites a file that
/// survives without its id, and whose unwritten row cites that same file.
fn thread(id: &str, status: ThreadStatus) -> Thread {
  Thread {
    attachments: Vec::new(),
    body: String::new(),
    preamble: String::new(),
    schema: THREAD_SCHEMA.to_string(),
    id: id.to_string(),
    title: "A thread whose test lost its label".to_string(),
    slug: None,
    status,
    status_reason: None,
    fiat: None,
    created: "2026-08-14".to_string(),
    completed: status.is_closed().then(|| "2026-08-20".to_string()),
    acceptance: None,
    objective: "Keep its contract honest after the close.".to_string(),
    context: String::new(),
    related: vec![],
    wps: vec![WorkPackage {
      seq: 1,
      title: "The only package".to_string(),
      scope: Some(TShirt::S),
      scope_legacy: None,
      status_legacy: None,
      status: WpStatus::Done,
      status_reason: None,
      fiat: None,
      objective: String::new(),
      body: String::new(),
      preamble: String::new(),
    }],
    criteria: vec![Criterion {
      id: "AC-01.1".to_string(),
      text: "the thing works".to_string(),
      kind: AcKind::Test,
      state: AcState::Computed {},
    }],
    tests: vec![
      row("AT-01.1", KEPT, AtStatus::Green),
      row(DROPPED_ROW, DROPPED, AtStatus::Green),
      row("AT-01.3", DROPPED, AtStatus::ToWrite),
    ],
  }
}

fn seed(fx: &Fixture, threads: &[Thread]) {
  for t in threads {
    fx.write_thread(t);
  }
  // **AFTER `write_thread`, WHICH WRITES A STUB CARRYING EVERY CITING ROW'S ID.**
  // These two replace the stubs: the kept file still carries its row's id, and
  // the dropped file is what is left when the labelled test was deleted.
  fx.write_file(KEPT, "// AT-01.1 covers AC-01.1\n#[test]\nfn kept() {}\n");
  fx.write_file(DROPPED, "// the test that carried the id was deleted\n");
  let project = fx.project();
  let canon = intentsvcs::ingest::read(&project).expect("fixture canon reads");
  intentsvcs::views::write_all(&project, &canon, &ctx()).expect("write views");
}

fn diagnose(fx: &Fixture, scope: Scope) -> Report {
  intentsvcs::doctor::diagnose(&fx.project(), &ctx(), None, scope)
}

fn drift(report: &Report) -> Vec<&intentsvcs::finding::Finding> {
  report
    .findings
    .iter()
    .filter(|f| f.class == FindingClass::ClosedContractDrift)
    .collect()
}

#[test]
fn scope_closed_names_the_completed_threads_row_that_lost_its_id_and_no_other() {
  let fx = Fixture::new();
  seed(
    &fx,
    &[
      thread("ST0001", ThreadStatus::Completed),
      thread("ST0002", ThreadStatus::Wip),
    ],
  );
  let closed = diagnose(&fx, Scope::Closed);
  let named = drift(&closed);
  assert_eq!(
    named.len(),
    1,
    "only the completed thread's dropped row is drift: {named:?}"
  );
  assert!(
    named[0].detail.contains("ST0001") && named[0].detail.contains(DROPPED_ROW),
    "the line names the thread and the row: {}",
    named[0].detail
  );
  assert!(
    named[0].detail.contains(DROPPED),
    "and the file that lost the id: {}",
    named[0].detail
  );
}

#[test]
fn the_default_withholds_the_drift_and_counts_it() {
  let fx = Fixture::new();
  seed(&fx, &[thread("ST0001", ThreadStatus::Completed)]);
  let closed = diagnose(&fx, Scope::Closed);
  assert!(
    !drift(&closed).is_empty(),
    "the fixture must produce the drift, or this arm is vacuous"
  );
  let live = diagnose(&fx, Scope::Live);
  assert!(
    drift(&live).is_empty(),
    "a completed thread is not live work"
  );
  assert_eq!(
    live.out_of_scope,
    closed.findings.len(),
    "the default counts what it withheld"
  );
}

#[test]
fn the_drift_is_shown_and_never_counted_so_no_commit_is_refused() {
  let class = FindingClass::ClosedContractDrift;
  assert!(
    !class.is_actionable(),
    "a closed thread's history never refuses a commit"
  );
  assert!(
    class.is_shown_by_default(),
    "doctor names it without --verbose"
  );
}

#[test]
fn nothing_reopens_the_gate_and_at_lint_still_pass_the_completed_row() {
  let fx = Fixture::new();
  seed(&fx, &[thread("ST0001", ThreadStatus::Completed)]);
  let refs = RepoFiles(fx.root());
  let completed = thread("ST0001", ThreadStatus::Completed);
  assert!(
    !completed_thread_drift(&completed, &refs).is_empty(),
    "the drift is there to be exempted"
  );
  let report = contract_report(&completed, None, &refs);
  assert!(
    report.findings.is_empty(),
    "the close gate's L3 exemption is unchanged: {:?}",
    report.findings
  );
}

#[test]
fn a_live_thread_is_the_gates_and_not_this_checks() {
  let fx = Fixture::new();
  seed(&fx, &[thread("ST0002", ThreadStatus::Wip)]);
  let refs = RepoFiles(fx.root());
  let live = thread("ST0002", ThreadStatus::Wip);
  assert!(completed_thread_drift(&live, &refs).is_empty());
  assert!(
    contract_report(&live, None, &refs)
      .findings
      .iter()
      .any(|f| f.contains(DROPPED_ROW)),
    "the gate itself names a live thread's dropped row"
  );
}
