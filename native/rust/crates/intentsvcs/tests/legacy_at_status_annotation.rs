//! **THE STATUS IS A TOKEN; THE WORDS AFTER IT ARE NOT PART OF IT.**
//!
//! A row ending `-- status: green. Proven to discriminate: ...` has no further
//! ` -- `, so [`field_end`] runs the status field to END OF LINE and the value
//! becomes `green. Proven to discriminate: ...`. That matches no known status,
//! `acceptance_test` returns `None`, and the row is DROPPED.
//!
//! **MEASURED, NOT SUPPOSED.** arca_cli `ST0011` at `33e3c2d`: 55 AT ids in
//! the v2 source, 29 in canon, 26 lost. Cross-tabbed by row shape the
//! discrimination is perfect in both directions -- 26 lost / 0 kept for this
//! shape, 0 lost / 29 kept for every other -- and `AT-09.4` is the control,
//! sitting in the SAME work package as three lost rows and surviving only
//! because its trailing prose is attached with ` -- ` instead of `. `.
//!
//! **THE LOSS CORRELATES WITH AUTHORING QUALITY, WHICH IS WHY IT HID SO LONG.**
//! The work packages that adopted the "Proven to discriminate:" discipline
//! wrote a sentence after every status, so every one of those rows died. The
//! better the author, the surer the drop -- and any sample drawn from early
//! work packages under-reports it (ic).
//!
//! **FOUR SHAPES, THREE OF WHICH ARE LOSSY.** vc read the last three verbatim
//! off a second corpus (Lamplight) after this was diagnosed on the first:
//!
//! - `status: green` / `status: green -- <more>` -- always worked.
//! - `status: green. <sentence>` -- shape (a), the arca_cli corpus.
//! - `status: green.` -- shape (b), a BARE trailing period, value `green.`.
//! - `status: green (<parenthetical>)` -- shape (c), and the parenthetical may
//!   contain ` -- ` itself, which the bracket-aware [`field_end`] already
//!   survives; only the token terminator was missing.
//!
//! Rows 1-5 are arca_cli `ST0011`, byte for byte. Rows 6-9 reproduce the
//! Lamplight shapes vc quoted; they are constructed, and say so.

mod common;

use common::Fixture;
use intentsvcs::legacy;
use intentsvcs::model::AtStatus;

const ROWS: &str = r#"## Acceptance Criteria

- AC-06.1 (non-test) A thing -- evidence: e -- satisfied: yes
- AC-09.1 (non-test) A thing -- evidence: e -- satisfied: yes
- AC-09.4 (non-test) A thing -- evidence: e -- satisfied: yes
- AC-12.1 (non-test) A thing -- evidence: e -- satisfied: yes
- AC-15.1 (non-test) A thing -- evidence: e -- satisfied: yes
- AC-20.1 (non-test) A thing -- evidence: e -- satisfied: yes
- AC-20.2 (non-test) A thing -- evidence: e -- satisfied: yes
- AC-20.3 (non-test) A thing -- evidence: e -- satisfied: yes
- AC-20.4 (non-test) A thing -- evidence: e -- satisfied: yes

## Acceptance Tests

- AT-06.1 test/arca_cli/commands/sys_cmd_test.exs (14 tests) -- covers AC-06.1 -- status: green
- AT-09.4 test/arca_cli/testing/cli_fixtures_pattern_test.exs (9 tests) -- covers AC-09.4 -- status: green -- fixed early, in WP-02, because it blocked the version fixture
- AT-09.1 test/arca_cli/no_test_env_gate_test.exs (4 tests) -- covers AC-09.1 -- status: green. Proven to discriminate: a temporary `Mix.env()` added to `output.ex` turned it red naming `lib/arca_cli/output.ex:217`, and removing it turned it green. Carries its own control test, so a scanner that silently matched nothing could not report the invariant as satisfied.
- AT-12.1 test/arca_cli/output/renderer_parity_test.exs, the `@outcome_table` rows (8 tests) -- covers AC-12.1 -- status: green. This table exists because AT-12.2 compares the renderers against `Ctx.outcome/1`, which proves the four sites AGREE but cannot prove the authority is right: both sides would move together. The expected outcomes here are literals, so a change to `Ctx.outcome/1` has to be argued for in this table rather than silently ratified by the tests that depend on it.
- AT-15.1 test/arca_cli/error_format_test.exs::"failure: a setting that does not exist" -- covers AC-15.1 -- status: green. Not a new test: it is the existing dialect assertion, which went red at the bump naming the raw tuple it had started printing. A test that already asserted the right thing and simply began failing is the best possible evidence that the contract changed.
- AT-20.1 test/lamplight/count_arm_test.exs (3 tests) -- covers AC-20.1 -- status: green.
- AT-20.2 test/lamplight/floor_test.exs (4 tests) -- covers AC-20.2 -- status: green (2026-07-25 cc: the anti-vacuity floor -- a zero-row pass is not a pass)
- AT-20.3 test/lamplight/fold_test.exs (2 tests) -- covers AC-20.3 -- status: green (mutation-proved: removing the terminal fold reds it). The atomic case is asserted separately.
- AT-20.4 (non-test) The evidence is inline above -- covers AC-20.4 -- status: n/a (AC-03.2 is non-test; its evidence is inline above). It previously read as a test row.
"#;

/// Every row must ARRIVE. Before the fix, four of these nine did.
const EXPECTED_IDS: &[&str] = &[
  "AT-06.1", "AT-09.1", "AT-09.4", "AT-12.1", "AT-15.1", "AT-20.1", "AT-20.2", "AT-20.3", "AT-20.4",
];

/// The status each row must resolve to -- the leading TOKEN, never the prose.
const EXPECTED_STATUS: &[(&str, AtStatus)] = &[
  ("AT-06.1", AtStatus::Green),
  ("AT-09.1", AtStatus::Green),
  ("AT-09.4", AtStatus::Green),
  ("AT-12.1", AtStatus::Green),
  ("AT-15.1", AtStatus::Green),
  ("AT-20.1", AtStatus::Green),
  ("AT-20.2", AtStatus::Green),
  ("AT-20.3", AtStatus::Green),
  ("AT-20.4", AtStatus::Na),
];

/// A fragment of the annotation that must survive into the note. Each lies
/// PAST the cut, so a fix that widened parsing by DROPPING the author's words
/// stays red here -- which is the failure mode this arm exists for.
const WITNESSES: &[(&str, &str)] = &[
  ("AT-09.1", "Proven to discriminate"),
  ("AT-12.1", "cannot prove the authority is right"),
  ("AT-15.1", "Not a new test"),
  (
    "AT-20.2",
    "anti-vacuity floor -- a zero-row pass is not a pass",
  ),
  (
    "AT-20.3",
    "mutation-proved: removing the terminal fold reds it",
  ),
  ("AT-20.4", "its evidence is inline above"),
];

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

fn scan_of(acceptance: &str) -> legacy::Scan {
  let fixture = Fixture::new();
  v2_estate(&fixture, acceptance);
  legacy::scan(&fixture.project()).expect("scan the v2 estate")
}

/// **THE ROW ARRIVES.** This is the defect: 26 of 55 real rows did not.
#[test]
fn a_status_followed_by_prose_does_not_drop_the_row() {
  let scan = scan_of(ROWS);
  let got: Vec<&str> = scan.threads[0]
    .tests
    .iter()
    .map(|t| t.id.as_str())
    .collect();
  for want in EXPECTED_IDS {
    assert!(
      got.contains(want),
      "{want} was DROPPED -- migration would report ok having lost it.\n  arrived: {got:?}"
    );
  }
  assert_eq!(got.len(), EXPECTED_IDS.len(), "arrived: {got:?}");
}

/// **AND IT ARRIVES WITH THE RIGHT STATUS**, not merely present.
#[test]
fn the_status_is_the_leading_token_not_the_sentence() {
  let scan = scan_of(ROWS);
  for (id, want) in EXPECTED_STATUS {
    let t = scan.threads[0]
      .tests
      .iter()
      .find(|t| t.id == *id)
      .unwrap_or_else(|| panic!("{id} did not arrive at all"));
    assert_eq!(&t.status, want, "{id} resolved the wrong status");
  }
}

/// **THE AUTHOR'S WORDS ARE NOT THE PRICE OF THE FIX.**
#[test]
fn the_annotation_after_the_status_is_carried_into_the_note() {
  let scan = scan_of(ROWS);
  for (id, fragment) in WITNESSES {
    let t = scan.threads[0]
      .tests
      .iter()
      .find(|t| t.id == *id)
      .unwrap_or_else(|| panic!("{id} did not arrive at all"));
    let note = t.note.as_deref().unwrap_or("");
    assert!(
      note.contains(fragment),
      "{id} dropped its annotation: {fragment:?} not in note {note:?}"
    );
  }
}

/// **A ` -- ` INSIDE A PARENTHETICAL IS NOT A FIELD SEPARATOR** -- the trap in
/// shape (c), and the reason `field_end` counts brackets.
#[test]
fn a_separator_inside_the_parenthetical_does_not_split_the_annotation() {
  let scan = scan_of(ROWS);
  let t = scan.threads[0]
    .tests
    .iter()
    .find(|t| t.id == "AT-20.2")
    .expect("AT-20.2");
  let note = t.note.as_deref().unwrap_or("");
  assert!(
    note.contains("anti-vacuity floor -- a zero-row pass is not a pass"),
    "the parenthetical was split at its inner separator: {note:?}"
  );
}

/// **AND A ROW THAT REALLY IS UNREADABLE MUST BE LOUD.** Widening what parses
/// is only half: the reason 26 rows vanished in SILENCE is the half that
/// outlives this fix.
#[test]
fn an_unknown_status_token_is_named_rather_than_dropped_in_silence() {
  let bad = "## Acceptance Criteria\n\n- AC-30.1 (non-test) A thing -- evidence: e -- satisfied: yes\n\n## Acceptance Tests\n\n- AT-30.1 test/x_test.exs (1 test) -- covers AC-30.1 -- status: chartreuse\n";
  let scan = scan_of(bad);
  let named = scan
    .residue
    .iter()
    .chain(scan.carried.iter())
    .any(|f| format!("{f:?}").contains("30"));
  assert!(
    named,
    "an unreadable row produced NO finding -- residue {:?}, carried {:?}",
    scan.residue, scan.carried
  );
}
