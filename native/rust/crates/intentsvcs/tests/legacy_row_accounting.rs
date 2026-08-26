//! **A ROW THAT GOES WITH NO RECORD IS THE DEFECT; THE PARSE BUG WAS ONLY THE
//! TRIGGER.**
//!
//! arca_cli `ST0011` lost 26 of 55 AT rows and 8 of 57 AC rows, and hop 2
//! printed `residue: 0 blocking, 3 carried` and `ok: this project is now Intent
//! v3.0.0`. Fixing the parser removes today's cause. **It does not remove the
//! property that made it invisible**, which is that nothing compared the rows
//! declared in a file against the rows that came out of it.
//!
//! `Scan::dispositions` already states the rule, as vc's condition 1: *a drop
//! with no record is indistinguishable from a section that was never there.*
//! This is that rule enforced per file, over AC and AT rows, by arithmetic that
//! must close.
//!
//! **AND IT BLOCKS ON A CLOSED THREAD, WHICH RESIDUE DOES NOT** (vc, (ii),
//! 2026-08-26). ST0011 sits in `COMPLETED/`, so every dropped row was routed to
//! `carried`, and carried does not block. The carry policy is for rows an
//! author wrote badly; it never covered rows the reader could not account for.

mod common;

use common::Fixture;
use intentsvcs::legacy;

fn estate(fixture: &Fixture, acceptance: &str) {
  fixture.write_file(
    "intent/.config/config.json",
    "{\"intent_version\":\"2.19.0\",\"project_name\":\"P\",\"author\":\"cc\",\"intent_dir\":\"intent\",\"languages\":[\"rust\"]}\n",
  );
  fixture.write_file(
    "intent/st/ST0002/info.md",
    "---\nverblock: \"24 Jun 2026:v0.4: matts - x\"\nintent_version: 2.19.0\nstatus: WIP\nslug: a-slug\ncreated: 20260624\n---\n\n# ST0002: A thread\n\n## Objective\n\nShip it.\n",
  );
  fixture.write_file("intent/st/ST0002/acceptance.md", acceptance);
}

/// A file every row of which reads. The accounting must not fire on health --
/// a check that refuses a correct estate is worse than the one it replaced.
#[test]
fn a_file_whose_rows_all_read_is_not_refused() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "## Acceptance Criteria\n\n\
     - AC-01.1 (non-test) A thing -- evidence: e -- satisfied: yes\n\
     - AC-01.2 (non-test) Another -- evidence: e -- satisfied: yes (hv signed off 2026-06-22)\n\
     \n## Acceptance Tests\n\n\
     - AT-01.1 test/a_test.exs (3 tests) -- covers AC-01.1 -- status: green\n\
     - AT-01.2 test/b_test.exs (4 tests) -- covers AC-01.2 -- status: green. Proven to discriminate.\n",
  );
  let scan = legacy::scan(&fixture.project()).expect("a healthy file must not be refused");
  assert_eq!(scan.threads[0].criteria.len(), 2);
  assert_eq!(scan.threads[0].tests.len(), 2);
}

/// **A ROW THAT IS REFUSED IS STILL ACCOUNTED FOR.** The arithmetic closes
/// because the refusal was RECORDED -- which is the whole distinction between a
/// row an author wrote badly and a row this reader lost.
#[test]
fn a_refused_row_is_accounted_for_by_the_finding_it_recorded() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "## Acceptance Criteria\n\n\
     - AC-01.1 (non-test) A thing -- evidence: e -- satisfied: yes\n\
     \n## Acceptance Tests\n\n\
     - AT-01.1 test/a_test.exs (1 test) -- covers AC-01.1 -- status: chartreuse\n",
  );
  let scan =
    legacy::scan(&fixture.project()).expect("a NAMED refusal is not an accounting failure");
  assert_eq!(scan.threads[0].tests.len(), 0, "the row is refused");
  assert_eq!(
    scan.residue.len() + scan.carried.len(),
    1,
    "and exactly one refusal was recorded for it"
  );
}

/// **THE REFUSAL NAMES THE ROW AND THE REASON.** It read `AC row` at a file and
/// a line, which tells an operator to go and look rather than what to fix.
#[test]
fn an_unreadable_verdict_names_the_criterion_and_why() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "## Acceptance Criteria\n\n\
     - AC-01.1 (non-test) A thing -- evidence: e -- satisfied: probably\n\
     \n## Acceptance Tests\n\n",
  );
  let scan = legacy::scan(&fixture.project()).expect("scan");
  let detail = scan
    .residue
    .iter()
    .chain(scan.carried.iter())
    .map(|f| f.detail.clone())
    .collect::<Vec<_>>()
    .join(" | ");
  assert!(
    detail.contains("AC-01.1"),
    "the row is not named: {detail:?}"
  );
  assert!(
    detail.contains("probably"),
    "the reason is not named: {detail:?}"
  );
}

/// **A TRUNCATED LINE IS NAMED AS TRUNCATED**, not as an unknown verdict. The
/// two have different remedies: one is a typo, the other is a lost half-line.
#[test]
fn a_truncated_parenthetical_says_that_it_is_truncated() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "## Acceptance Criteria\n\n\
     - AC-01.1 (non-test) A thing -- evidence: e -- satisfied: yes (hv signed off\n\
     \n## Acceptance Tests\n\n",
  );
  let scan = legacy::scan(&fixture.project()).expect("scan");
  assert!(
    scan.threads[0].criteria.is_empty(),
    "a truncated verdict still refuses"
  );
  let detail = scan
    .residue
    .iter()
    .chain(scan.carried.iter())
    .map(|f| f.detail.clone())
    .collect::<Vec<_>>()
    .join(" | ");
  assert!(
    detail.contains("AC-01.1") && detail.contains("unclosed"),
    "a truncation must be named as one: {detail:?}"
  );
}
