//! **AN AT THAT NAMES NO SUBJECT, WHICH IS WHAT `to-write` MEANS.**
//!
//! `- AT-04.3 -- covers AC-04.3 -- status: to-write -- <note>` is a test that
//! does not exist yet, so it has no path to name. The grammar demanded one
//! anyway, and the mechanism is a single character: `acceptance_test` takes the
//! id with a `split_once(' ')`, which consumes the ONLY space before the `--`,
//! and `covers()` looks for ` -- covers ` WITH its leading space. The marker was
//! not found, `covers()` returned `None`, and the row was refused before its
//! status was ever read.
//!
//! **THE ROW'S OWN STATUS SAID THE SUBJECT COULD NOT EXIST.** Sixteen rows in
//! one project, every one of them `to-write`, all sixteen blocking a live
//! migration. Bisected on four probes: a file reference accepts, a `(non-test)`
//! marker accepts, and the note, the value and the parenthetical are all
//! irrelevant -- only the subject moved the verdict.
//!
//! **AND THE RENDERER HAD THE SAME DEFECT ONE LAYER DOWN.** `(no reference)` was
//! written for a state a v2 row could not reach while the grammar refused it.
//! The moment these rows parsed, every projection wrote those words into rows
//! whose authors wrote nothing there -- a fix that stops refusing and starts
//! rewriting is worse than the refusal, because a block is loud and a rewrite
//! is not. `authored_row_round_trip` caught it; this file pins it.

mod common;

use common::Fixture;
use intentsvcs::legacy;
use intentsvcs::model::AtStatus;

/// The two rows are Lamplight `ST0201:121-122`, byte for byte, extracted by
/// script. **Not composed** -- everybody writing a test for an AT row writes a
/// file path into it, which is exactly the spelling that already worked.
const SUBJECTLESS: &str = r#"## Acceptance Criteria

- AC-04.3 (non-test) A thing -- evidence: e -- satisfied: yes
- AC-04.4 (non-test) Another -- evidence: e -- satisfied: yes

## Acceptance Tests

- AT-04.3 -- covers AC-04.3 -- status: to-write -- LLM-failure fallback to authored text + telemetry, red-first; RESHAPE candidate under ST0241: selection's likely home is the external aigent; hv rules at the ST0241 settle
- AT-04.4 -- covers AC-04.4 -- status: to-write -- off-catalog selection rejected typed, red-first; RESHAPE candidate under ST0241, as AT-04.3
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

fn scan(fixture: &Fixture) -> legacy::Scan {
  legacy::scan(&fixture.project()).expect("scan the v2 estate")
}

/// **BOTH REAL ROWS INGEST, AND THEY BLOCK NOTHING.**
#[test]
fn an_at_naming_no_subject_is_read_rather_than_refused() {
  let fixture = Fixture::new();
  v2_estate(&fixture, SUBJECTLESS);
  let scan = scan(&fixture);

  assert!(
    scan.residue.is_empty(),
    "a subjectless AT on a LIVE thread must not block the migration: {:?}",
    scan.residue
  );
  assert_eq!(
    scan.threads[0].tests.len(),
    2,
    "both rows must arrive; a shortfall means `covers()` is still missing its leading space"
  );
}

/// **AND THE ROW IS READ CORRECTLY, not merely accepted.** Coverage and status
/// are the two things the refusal was destroying.
#[test]
fn its_status_and_coverage_survive() {
  let fixture = Fixture::new();
  v2_estate(&fixture, SUBJECTLESS);
  let scan = scan(&fixture);
  let tests = &scan.threads[0].tests;

  // **WITHOUT THIS THE ARM PASSES VACUOUSLY**, and the mutation proof is what
  // said so: revert the parser fix, `tests` is empty, `for t in tests` never
  // runs a body, and a test asserting nothing reports green. A loop over a
  // collection that the defect EMPTIES cannot witness the defect.
  assert_eq!(
    tests.len(),
    2,
    "both rows must be present before anything is asserted about them"
  );

  for t in tests {
    assert_eq!(t.status, AtStatus::ToWrite, "{} lost its status", t.id);
    assert!(!t.covers.is_empty(), "{} lost its coverage", t.id);
    assert!(t.file.is_none(), "{} invented a file reference", t.id);
    assert!(
      t.note.as_deref().is_some_and(|n| !n.trim().is_empty()),
      "{} dropped its note -- the half the author actually wrote",
      t.id
    );
  }
}

/// **THE CONTROL: a row WITH a subject still parses the way it always did.**
/// Without this arm the fix could have widened the grammar by breaking the
/// ordinary path and nothing here would say so.
#[test]
fn a_row_that_names_its_test_is_unaffected() {
  let fixture = Fixture::new();
  v2_estate(
    &fixture,
    "## Acceptance Criteria\n\n- AC-01.1 (non-test) A thing -- evidence: e -- satisfied: yes\n\n## Acceptance Tests\n\n- AT-01.1 `apps/x/test/a_test.exs` -- covers AC-01.1 -- status: green -- landed\n",
  );
  let scan = scan(&fixture);

  assert!(scan.residue.is_empty(), "{:?}", scan.residue);
  let t = &scan.threads[0].tests[0];
  assert_eq!(t.file.as_deref(), Some("apps/x/test/a_test.exs"));
  assert_eq!(t.status, AtStatus::Green);
}
