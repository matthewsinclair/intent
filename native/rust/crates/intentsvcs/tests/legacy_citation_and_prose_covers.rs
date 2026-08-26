//! **A CITATION'S ANNOTATION IS NOT PART OF ITS PATH, AND A `covers` CLAUSE
//! WITH NO ID IN IT COVERS NOTHING.**
//!
//! Arca/arca_cli `ST0011` migrated at exit 0 and its gate then reported 19
//! findings against five work packages recorded `Done`. **Nothing failed. The
//! migration was clean.** The reader had quietly reshaped rows on the way in.
//!
//! Two separate defects arrived wearing one symptom, and they are in this one
//! file because telling them apart is the point:
//!
//! - **The citation split keeps a comma-annotation inside the path.**
//!   `split_citation` cuts at ` (` or a backtick and knew nothing about
//!   `path, describe "..."`, so the stored `file` was
//!   `test/.../dead_code_gate_test.exs, describe "purged symbols` and 16 rows
//!   reported *cites a file that does not exist* against paths that all exist.
//!   **241 rows on this machine carry the shape** -- Lamplight 111, Arca 48,
//!   Intent's own tree 17 -- and **not one of them is a second file**: the
//!   comma is followed by `describe` (110), `the` (50), `and` (25), never by
//!   another path. That measurement is what licenses cutting there.
//!
//! - **A prose `covers` clause became a criterion id.** `-- covers the gate
//!   itself --` is stored as `covers: ["the gate itself"]`, which then reads as
//!   a dangling reference to an id nobody ever wrote. **This half is already
//!   fixed** and the test below is what proves it rather than a claim that it
//!   is: today's reader takes the LEADING TOKEN as the id, so the span yields
//!   `the`, which is not a criterion id, and the row is named as unreadable
//!   instead. The store still holds the bad value because it was written by a
//!   binary predating that fix -- which is the whole argument for re-converting
//!   `ST0011` from source rather than repairing the reader again.

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

fn only_thread(scan: &legacy::Scan) -> &intentsvcs::model::Thread {
  scan.threads.first().expect("one thread")
}

/// **THE CRITERION: the stored path is the path, and the author's words go to
/// the note.**
///
/// Captured verbatim from `Arca/arca_cli` `ST0011` AT-07.1, only renumbered:
///
/// ```text
/// - AT-07.1 test/arca_cli/dead_code_gate_test.exs, describe "purged symbols (AC-07.1)" (7 tests) -- covers AC-07.1 -- status: green
/// ```
///
/// The old cut landed on the ` (` inside `(AC-07.1)`, so the comma clause was
/// swallowed into `file` and the `(7 tests)` half became the annotation. **The
/// path it produced does not exist, and 16 rows said so about files sitting on
/// disk.**
#[test]
fn a_comma_annotation_is_not_kept_inside_the_cited_path() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "## Acceptance Criteria\n\n\
     - AC-01.1 Purged symbols stay purged. -- evidence: the gate -- satisfied: yes\n\
     \n## Acceptance Tests\n\n\
     - AT-01.1 test/arca_cli/dead_code_gate_test.exs, describe \"purged symbols (AC-01.1)\" (7 tests) -- covers AC-01.1 -- status: green\n",
  );

  let scan = legacy::scan(&fixture.project()).expect("the scan completes");
  let test = only_thread(&scan).tests.first().expect("one AT row");

  assert_eq!(
    test.file.as_deref(),
    Some("test/arca_cli/dead_code_gate_test.exs"),
    "the cited file is the path and nothing after it"
  );

  // **NOTHING IS DROPPED**, which is the standing rule for this split: the
  // words the author wrote survive in the note even though they leave `file`.
  let note = test.note.clone().unwrap_or_default();
  assert!(
    note.contains("purged symbols"),
    "the annotation must survive in the note, not be discarded: {note:?}"
  );
}

/// **THE CONTROL THAT KEEPS THE CUT HONEST: a plain path is untouched.**
///
/// This is the overwhelming majority of the estate, and a split that widened
/// what it cuts must not move it. Without this arm, a rule that cut at the
/// first comma-or-anything would pass the arm above and quietly truncate every
/// ordinary citation.
#[test]
fn a_plain_path_citation_is_unchanged_by_the_split() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "## Acceptance Criteria\n\n\
     - AC-01.1 It holds. -- evidence: x -- satisfied: yes\n\
     \n## Acceptance Tests\n\n\
     - AT-01.1 test/arca_cli/output/ansi_renderer_test.exs -- covers AC-01.1 -- status: green\n",
  );

  let scan = legacy::scan(&fixture.project()).expect("the scan completes");
  let test = only_thread(&scan).tests.first().expect("one AT row");
  assert_eq!(
    test.file.as_deref(),
    Some("test/arca_cli/output/ansi_renderer_test.exs")
  );
  assert!(
    test.note.is_none(),
    "and no annotation is manufactured from a citation that has none: {:?}",
    test.note
  );
}

/// **THE OTHER CONTROL: the `::name` form still splits where it always did.**
///
/// `path::"invariant: ..."` carries a `:`, so it fails the 0017 path rules and
/// is carried whole as a legacy reference. Adding a comma to the cut candidates
/// must not disturb that verdict.
#[test]
fn the_double_colon_form_is_still_carried_whole_as_a_legacy_reference() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "## Acceptance Criteria\n\n\
     - AC-01.1 It holds. -- evidence: x -- satisfied: yes\n\
     \n## Acceptance Tests\n\n\
     - AT-01.1 test/arca_cli/dead_code_gate_test.exs::\"invariant: the scanner finds strings\" -- covers AC-01.1 -- status: green\n",
  );

  let scan = legacy::scan(&fixture.project()).expect("the scan completes");
  let test = only_thread(&scan).tests.first().expect("one AT row");
  assert!(
    test.file.is_none(),
    "a citation carrying `:` is not a path: {:?}",
    test.file
  );
  assert!(
    test.legacy.is_some(),
    "it is carried whole as a legacy reference instead"
  );
}

/// **THE PROSE-COVERS HALF, WHICH IS ALREADY FIXED -- AND THIS IS THE PROOF
/// RATHER THAN THE CLAIM.**
///
/// Captured from `ST0011` AT-07.3: `-- covers the gate itself --`. The store
/// holds `covers: ["the gate itself"]`, an id nobody wrote, which the gate then
/// reports as a dangling reference.
///
/// **Today's reader cannot produce that value**, and the reason is worth
/// stating because it is what tells a stale store from a live defect: the id is
/// the LEADING TOKEN, so this span yields `the` -- not the whole phrase -- and
/// `the` is not a criterion id, so the span is named unreadable and no id is
/// invented. A reader that still had the old whole-span cut would store the
/// phrase and this arm would go red.
#[test]
fn a_prose_covers_clause_yields_no_criterion_id_and_is_named() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "## Acceptance Criteria\n\n\
     - AC-01.1 The gate holds. -- evidence: x -- satisfied: yes\n\
     \n## Acceptance Tests\n\n\
     - AT-01.1 test/arca_cli/dead_code_gate_test.exs -- covers the gate itself -- status: green\n",
  );

  let scan = legacy::scan(&fixture.project()).expect("the scan completes");
  let test = only_thread(&scan).tests.first().expect("one AT row");

  assert!(
    test.covers.is_empty(),
    "prose is not an id, and inventing one manufactures a dangling reference: {:?}",
    test.covers
  );
  let said: Vec<String> = scan
    .residue
    .iter()
    .chain(scan.carried.iter())
    .map(|f| f.detail.clone())
    .collect();
  assert!(
    said
      .iter()
      .any(|d| d.contains("the gate itself") && d.contains("no criterion id")),
    "and the clause is NAMED rather than silently dropped: {said:?}"
  );
}
