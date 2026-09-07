//! **AN UNBALANCED `[n/a` IS STILL AN `n/a` JUSTIFICATION, NOT A PATH.**
//!
//! `bracket_citation` reads a citation to its CLOSING bracket and returns
//! `None` when there is not one -- deliberately, so a row whose `[` never
//! closes cannot swallow the keyed fields after it. That decision is right and
//! is untouched here.
//!
//! What was wrong is what happened NEXT. The `n/a` exclusion was asked of the
//! BALANCED citation, so an unbalanced row fell through to the naive ` -- `
//! split, arrived at the path rule as `n/a`, and satisfied it on the slash
//! `n/a` has always carried. The row then stores `[n/a` as its test file.
//!
//! **MEASURED, NOT IMAGINED: fifteen rows on Lamplight** store `[n/a`, a bare
//! `[`, or a whole sentence in `file`. Each one makes `ac gate` report `cites a
//! file that does not exist` against work that is done -- the same visible
//! damage the bracket reader was built to stop, reached by the one route it
//! declined to read.
//!
//! **THE ROW BELOW IS REAL**, from Lamplight ST0270: the `[` opens and nothing
//! closes it before the keyed fields begin.

use crate::common::Fixture;
use intentsvcs::legacy;
use intentsvcs::model::AtStatus;

const UNBALANCED: &str = r#"## Acceptance Criteria

- AC-01.1 (non-test) A thing -- evidence: e -- satisfied: yes
- AC-01.2 (non-test) A thing -- evidence: e -- satisfied: yes

## Acceptance Tests

- AT-01.1 [n/a -- covers AC-01.1 -- status: n/a
- AT-01.2 [n/a: no harness exists yet -- covers AC-01.2 -- status: n/a
"#;

fn v2_estate(fixture: &Fixture, acceptance: &str) {
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

fn tests_of(acceptance: &str) -> Vec<intentsvcs::model::AcceptanceTest> {
  let fixture = Fixture::new();
  v2_estate(&fixture, acceptance);
  let scan = legacy::scan(&fixture.project()).expect("scan the v2 estate");
  scan.threads[0].tests.clone()
}

/// **NO ROW STORES A BRACKET TOKEN AS ITS TEST FILE.**
///
/// The assertion is on `file` rather than on a finding count, because the
/// defect is silent: the migration succeeds, the row arrives, and the damage
/// only surfaces later as a gate refusing a file nobody ever cited.
#[test]
fn an_unbalanced_bracket_never_lands_in_the_file_field() {
  let tests = tests_of(UNBALANCED);
  assert_eq!(
    tests.len(),
    2,
    "both rows must arrive before anything is asserted"
  );

  for t in &tests {
    assert!(
      t.file.is_none(),
      "{} stored `{:?}` as a test file -- an `n/a` justification is not a citation",
      t.id,
      t.file
    );
    assert!(
      !t.file.as_deref().unwrap_or("").starts_with('['),
      "{} stored a bracket token as a path",
      t.id
    );
  }
}

/// **AND THE ROWS ARE STILL READ CORRECTLY OTHERWISE**, which is what stops the
/// arm above being satisfied by a fix that simply drops the rows, or by one
/// that lets the unbalanced bracket swallow the keyed fields after it. A row
/// that never arrives also has no bad `file`.
#[test]
fn the_rows_still_arrive_with_their_keyed_fields_intact() {
  let tests = tests_of(UNBALANCED);
  for t in &tests {
    assert_eq!(t.status, AtStatus::Na, "{} lost its n/a status", t.id);
  }
  assert_eq!(
    tests
      .iter()
      .find(|t| t.id == "AT-01.1")
      .expect("AT-01.1")
      .covers,
    vec!["AC-01.1".to_string()],
    "the unbalanced bracket must not have swallowed the covers clause"
  );
  assert_eq!(
    tests
      .iter()
      .find(|t| t.id == "AT-01.2")
      .expect("AT-01.2")
      .covers,
    vec!["AC-01.2".to_string()]
  );
}
