//! **A CITATION AND THE WORDS AFTER IT ARE TWO THINGS.**
//!
//! `test/cdsync.bats (whole suite, 328 tests)` went into `file` entire, so
//! canon asserted a test file whose name ends in `328 tests)`. Same defect as
//! `field()`'s unbounded cut, one field over: a mark that is also ordinary
//! prose, read as structure.
//!
//! **THE ROWS HERE ARE REAL AND COMMITTED, extracted from disk by script.**
//! Nobody writing a test for an AT row invents a path with a parenthetical
//! after it -- they write the clean spelling that already worked, which is why
//! this survived every fixture the crate owns.
//!
//! **NO REFUSE-ON-ABSENCE.** A `to-write` row cites a file that does not exist
//! yet BY DESIGN, so a check that the path resolves would refuse the rows the
//! grammar exists to carry. Widen what parses, never what refuses.

mod common;

use common::Fixture;
use intentsvcs::legacy;
use intentsvcs::model::AtStatus;

/// Row 1 is devbin-vc's minimal case. Rows 2-4 are committed canon, byte for
/// byte: Cdsync `ST0004`, Riffle `ST0004` twice. Row 5 is Prolix `ST0028`,
/// where the author's backtick closes the citation early and the outer pair is
/// left unbalanced -- the mark that ends the path is a backtick, not ` (`.
const ANNOTATED: &str = r#"## Acceptance Criteria

- AC-01.4 (non-test) A thing -- evidence: e -- satisfied: yes
- AC-00.3 (non-test) A thing -- evidence: e -- satisfied: yes
- AC-02.5 (non-test) A thing -- evidence: e -- satisfied: yes
- AC-02.2 (non-test) A thing -- evidence: e -- satisfied: yes
- AC-1.7 (non-test) A thing -- evidence: e -- satisfied: yes

## Acceptance Tests

- AT-01.4 `path/to/x_test.exs (6 tests)` -- covers AC-01.4 -- status: green
- AT-00.1 test/cdsync.bats (whole suite, 328 tests) -- covers AC-00.3 -- status: green
- AT-02.5 test/riffle/cli/sia_pipelines_command_test.exs (6) -- covers AC-02.5 -- status: green
- AT-02.2 test/riffle/cli/sia_run_command_test.exs (registration + the Ctx-returning contract) -- covers AC-02.2 -- status: green
- AT-1.3 `native/ios/ProlixTests` (whole target, via `bin/prolix test swift`)` -- covers AC-1.7 -- status: green
"#;

/// What each row's `file` must be. **Every one of these is a path that exists
/// as a path** -- which is the whole point: before the split they were stored
/// with the annotation welded on.
const EXPECTED_PATHS: &[(&str, &str)] = &[
  ("AT-01.4", "path/to/x_test.exs"),
  ("AT-00.1", "test/cdsync.bats"),
  ("AT-02.5", "test/riffle/cli/sia_pipelines_command_test.exs"),
  ("AT-02.2", "test/riffle/cli/sia_run_command_test.exs"),
  ("AT-1.3", "native/ios/ProlixTests"),
];

/// A fragment of each row's annotation that must survive into the note. Each
/// lies PAST the cut, so a fix that split the path and dropped the words would
/// stay red here.
const WITNESSES: &[(&str, &str)] = &[
  ("AT-01.4", "6 tests"),
  ("AT-00.1", "whole suite, 328 tests"),
  ("AT-02.5", "(6)"),
  ("AT-02.2", "the Ctx-returning contract"),
  ("AT-1.3", "bin/prolix test swift"),
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

fn tests_of(acceptance: &str) -> Vec<intentsvcs::model::AcceptanceTest> {
  let fixture = Fixture::new();
  v2_estate(&fixture, acceptance);
  let scan = legacy::scan(&fixture.project()).expect("scan the v2 estate");
  assert!(
    scan.residue.is_empty(),
    "these rows must not block a LIVE thread: {:?}",
    scan.residue
  );
  scan.threads[0].tests.clone()
}

/// **THE PATH IS THE PATH, and the annotation is not part of it.**
#[test]
fn a_trailing_annotation_is_not_welded_onto_the_path() {
  let tests = tests_of(ANNOTATED);
  assert_eq!(
    tests.len(),
    EXPECTED_PATHS.len(),
    "every row must arrive before anything is asserted about them"
  );
  for (id, want) in EXPECTED_PATHS {
    let t = tests
      .iter()
      .find(|t| t.id == *id)
      .unwrap_or_else(|| panic!("{id} did not arrive at all"));
    assert_eq!(
      t.file.as_deref(),
      Some(*want),
      "{id} stored a path the filesystem does not have"
    );
  }
}

/// **AND THE AUTHOR'S WORDS ARE NOT THE PRICE OF THE FIX.** A split that
/// widened parsing by dropping the annotation would pass the arm above and be
/// worse than the defect.
#[test]
fn the_annotation_is_carried_into_the_note_rather_than_dropped() {
  let tests = tests_of(ANNOTATED);
  assert_eq!(tests.len(), WITNESSES.len());
  for (id, fragment) in WITNESSES {
    let t = tests.iter().find(|t| t.id == *id).unwrap();
    let note = t.note.as_deref().unwrap_or("");
    assert!(
      note.contains(fragment),
      "{id} dropped its annotation: {fragment:?} is not in note {note:?}"
    );
  }
}

/// **CONTROL: a plain path -- the overwhelming majority of the estate -- is
/// untouched.** Without this the split could have widened the odd row by
/// mangling the ordinary one and nothing here would say so.
#[test]
fn a_plain_path_citation_is_unaffected() {
  let tests = tests_of(
    "## Acceptance Criteria\n\n- AC-01.1 (non-test) A thing -- evidence: e -- satisfied: yes\n\n## Acceptance Tests\n\n- AT-01.1 `apps/x/test/a_test.exs` -- covers AC-01.1 -- status: green -- landed\n",
  );
  let t = &tests[0];
  assert_eq!(t.file.as_deref(), Some("apps/x/test/a_test.exs"));
  assert_eq!(t.note.as_deref(), Some("landed"), "the note gained text");
  assert_eq!(t.status, AtStatus::Green);
}

/// **CONTROL: a test NAME carrying `()` is not a path with an annotation.**
/// Utilz writes seven of these. Cutting at a bare `(` rather than at ` (` would
/// behead `each_utility()` into `each_utility`.
#[test]
fn a_test_name_with_empty_parens_is_not_split() {
  let tests = tests_of(
    "## Acceptance Criteria\n\n- AC-01.1 (non-test) A thing -- evidence: e -- satisfied: yes\n\n## Acceptance Tests\n\n- AT-01.1 `lib/x/each_utility() lists them one per line` -- covers AC-01.1 -- status: green\n",
  );
  assert_eq!(
    tests[0].file.as_deref(),
    Some("lib/x/each_utility() lists them one per line"),
    "a name with `()` was cut as though it were an annotation"
  );
}

/// **CONTROL: a `to-write` row cites a file that does not exist yet, and that
/// is CORRECT.** Pinned so nobody later adds a resolves-on-disk check and
/// refuses the rows the grammar exists to carry.
#[test]
fn a_to_write_row_citing_a_file_that_does_not_exist_is_still_read() {
  let tests = tests_of(
    "## Acceptance Criteria\n\n- AC-01.1 (non-test) A thing -- evidence: e -- satisfied: yes\n\n## Acceptance Tests\n\n- AT-01.1 `test/not/written/yet_test.exs (4 tests)` -- covers AC-01.1 -- status: to-write\n",
  );
  assert_eq!(
    tests[0].file.as_deref(),
    Some("test/not/written/yet_test.exs")
  );
  assert_eq!(tests[0].status, AtStatus::ToWrite);
}

/// **CONTROL: a legacy `::` reference is still carried WHOLE**, never split.
/// `is_path` is decided on the full citation before any cut, so this row's
/// classification is exactly what it was.
#[test]
fn a_legacy_reference_is_carried_whole() {
  let tests = tests_of(
    "## Acceptance Criteria\n\n- AC-01.1 (non-test) A thing -- evidence: e -- satisfied: yes\n\n## Acceptance Tests\n\n- AT-01.1 (legacy) test/x.bats::\"a name (with parens)\" -- covers AC-01.1 -- status: green\n",
  );
  assert!(tests[0].file.is_none(), "a legacy citation became a path");
  let raw = tests[0]
    .legacy
    .as_ref()
    .map(|l| l.raw.as_str())
    .unwrap_or("");
  assert!(
    raw.contains("(with parens)"),
    "the legacy reference was split: {raw:?}"
  );
}
