//! **A FIELD THE v2 LINE CARRIES AND THIS GRAMMAR NEVER READS IS NAMED, NOT
//! SWALLOWED.**
//!
//! Lamplight's disposed criteria write `-- withdrawn: <reason>` and
//! `-- descoped-to: ST0347 -- by: hv -- on: 2026-08-21`. The AC reader knows
//! exactly two keys, `evidence:` and `satisfied:`, so all of it landed in the
//! row's text and 19 rows arrived with their disposition as prose. **Nothing
//! failed. Nothing was refused. The migration exited 0**, and the only symptom
//! was a gate reading 25/37 where v2 read 24/24.
//!
//! **THE FIX IS THE CLASS AND NOT THE TWO KEYS, AND THAT IS THE WHOLE POINT OF
//! THE FILE.** Teaching the reader `withdrawn:` and `descoped-to:` closes those
//! 19 and leaves the next convention exactly as silent -- which is the argument
//! `thread_dirs` already lost, where an allowlist of three bucket names closed
//! the instance and left the class open. **An allowlist can only ever know the
//! conventions that existed when it was written.**

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

fn details(scan: &legacy::Scan) -> Vec<String> {
  scan
    .residue
    .iter()
    .chain(scan.carried.iter())
    .map(|f| f.detail.clone())
    .collect()
}

/// **THE CRITERION: a disposed AC row names every key the reader walked past.**
///
/// Modelled on Lamplight `ST0346 AC-05.5` rather than invented: `descoped-to:`
/// with its own `by:` and `on:`, which is why the attribution never has to be
/// minted later.
#[test]
fn an_ac_row_naming_a_key_the_grammar_does_not_read_says_so() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "## Acceptance Criteria\n\n\
     - AC-01.1 The field round-trips without loss. -- descoped-to: ST0347 -- by: hv -- on: 2026-08-21 -- reason: duplicate -- rounds: 3 -- control: none\n\
     \n## Acceptance Tests\n\n\
     - AT-01.1 test/a_test.exs -- covers AC-01.1 -- status: green\n",
  );
  let scan = legacy::scan(&fixture.project()).expect("the row arrives; it is not refused");

  let said = details(&scan);
  let named = said
    .iter()
    .find(|d| d.contains("AC-01.1") && d.contains("does not read"))
    .unwrap_or_else(|| panic!("the unread keys must be named: {said:?}"));
  // **THE SPECIMEN MOVED AND THE PROPERTY DID NOT.** `descoped-to`, `by` and
  // `reason` are now READ -- the parser builds `AcState::Descoped` out of them --
  // so they are no longer evidence of anything being walked past. `rounds:` and
  // `control:` are real Lamplight keys nobody has taught the reader, which is
  // this file's own thesis: the report is a CLASS, and it must still name a
  // convention invented after it was written.
  for key in ["on", "rounds", "control"] {
    assert!(
      named.contains(&format!("`{key}`")),
      "every unread key on the row, not just the first: `{key}` missing from {named}"
    );
  }
  // The other half of the same boundary: a key the grammar now READS must stop
  // being reported, or the finding tells an operator to go and fix a field that
  // round-tripped correctly.
  for key in ["descoped-to", "by", "reason"] {
    assert!(
      !named.contains(&format!("`{key}`")),
      "`{key}` is read into the criterion's state now, so reporting it as unread sends the \
       operator after a field that arrived intact: {named}"
    );
  }
}

/// **THE ARM THAT STOPS THIS RE-LITIGATING A RULING THAT WAS ALREADY PAID FOR.**
///
/// An AT row's tail after its status value is `note`'s region. v2 declines to
/// parse it -- `AT_G_NOTE='( -- .*)?'`, greedy to end of line -- and vc ruled
/// it has no interior structure, against a 14-row corpus where a keyed read of
/// it lost twelve notes outright. **On Lamplight that region carries 95
/// ` -- red-first: ` and 5 ` -- mutation-proved: `**, and reporting them would
/// be this check inventing the grammar that ruling exists to refuse.
#[test]
fn a_key_inside_the_at_note_region_is_prose_and_is_not_reported() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "## Acceptance Criteria\n\n\
     - AC-01.1 (non-test) A thing -- evidence: e -- satisfied: yes\n\
     \n## Acceptance Tests\n\n\
     - AT-01.1 test/a_test.exs -- covers AC-01.1 -- status: green -- red-first: proven on the real tree -- mutation-proved: yes\n",
  );
  let scan = legacy::scan(&fixture.project()).expect("scan");

  let said = details(&scan);
  for key in ["red-first", "mutation-proved"] {
    assert!(
      !said
        .iter()
        .any(|d| d.contains("does not read") && d.contains(key)),
      "`{key}` sits in the ratified note region and must not be reported: {said:?}"
    );
  }
}

/// **AND THE OTHER SIDE OF THAT BOUNDARY, WITHOUT WHICH THE ARM ABOVE PASSES
/// FOR A CHECK THAT NEVER FIRES ON AN AT ROW AT ALL.**
///
/// The same key BEFORE ` -- status: ` is in the parsed region and is genuinely
/// unread. Six such occurrences on Lamplight against 118 in the note.
#[test]
fn the_same_key_before_the_status_field_is_reported() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "## Acceptance Criteria\n\n\
     - AC-01.1 (non-test) A thing -- evidence: e -- satisfied: yes\n\
     \n## Acceptance Tests\n\n\
     - AT-01.1 test/a_test.exs -- covers AC-01.1 -- requires: a fixture -- status: green\n",
  );
  let scan = legacy::scan(&fixture.project()).expect("scan");

  let said = details(&scan);
  assert!(
    said
      .iter()
      .any(|d| d.contains("AT-01.1") && d.contains("does not read") && d.contains("requires")),
    "a key before the status field is in the parsed region: {said:?}"
  );
}

/// **PROSE IS NOT A FIELD, AND NEITHER IS A SEPARATOR INSIDE A BRACKET.**
///
/// Two controls in one row. ` -- and then: it broke` has a space in its
/// candidate key, so it is text; the ` -- note: ` inside the parenthetical is
/// at depth 1, which is the same reason `field_end` is bracket-aware. Without
/// this, the check reports authored prose and gets disabled rather than fixed.
#[test]
fn prose_and_a_bracketed_separator_are_not_read_as_fields() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "## Acceptance Criteria\n\n\
     - AC-01.1 A thing (the first cut -- note: it was wrong) -- and then: it broke -- evidence: e -- satisfied: yes\n\
     \n## Acceptance Tests\n\n\
     - AT-01.1 test/a_test.exs -- covers AC-01.1 -- status: green\n",
  );
  let scan = legacy::scan(&fixture.project()).expect("scan");

  let said = details(&scan);
  assert!(
    !said.iter().any(|d| d.contains("does not read")),
    "neither a spaced candidate nor a bracketed separator is a field: {said:?}"
  );
}

/// **THE ROW STILL ARRIVES, AND THE MIGRATOR'S OWN ARITHMETIC MUST NOT COUNT
/// THIS AS A REFUSAL.**
///
/// Half A closes `declared == stored + recorded` per file, and a finding
/// recorded inside that window is counted as a refusal. An unread field is a
/// finding about a row that ARRIVED, so recording it in the wrong place makes
/// the check report the migration broken because it worked. `scan` returning
/// `Ok` is the assertion -- the reconciliation raises an error, not residue.
#[test]
fn a_row_with_an_unread_field_still_arrives_and_the_accounting_still_closes() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "## Acceptance Criteria\n\n\
     - AC-01.1 A thing -- withdrawn: not on its own merits -- rounds: 3\n\
     - AC-01.2 (non-test) Another -- evidence: e -- satisfied: yes\n\
     \n## Acceptance Tests\n\n\
     - AT-01.1 test/a_test.exs -- covers AC-01.2 -- status: green\n",
  );
  let scan = legacy::scan(&fixture.project())
    .expect("an unread field must not make the reconciliation refuse the file");

  assert_eq!(
    scan.threads[0].criteria.len(),
    2,
    "both rows are stored -- the field is unread, the ROW is not lost"
  );
  assert!(
    details(&scan).iter().any(|d| d.contains("`rounds`")),
    "and an unread key is still named"
  );
  assert!(
    !details(&scan).iter().any(|d| d.contains("`withdrawn`")),
    "`withdrawn:` is read into `AcState::Withdrawn` now, so it is no longer walked past"
  );
}

/// **A MULTIBYTE CHARACTER ANYWHERE IN THE ROW USED TO ABORT THE WHOLE
/// MIGRATION, AND THE ROW THAT PROVED IT CARRIES NO UNREAD FIELD AT ALL.**
///
/// Captured from Conflab `ST0121/acceptance.md:83`, the last un-migrated
/// estate, which died on `intent upgrade` at rc 101 with nothing written:
///
/// ```text
/// start byte index 270 is not a char boundary; it is inside '✓' (bytes 269..272)
/// ```
///
/// The scanner walked `span.as_bytes()` and then sliced `span[i..]` at every
/// byte index, so an index landing inside a three-byte character panicked. **It
/// is the SCAN that panics, not the residue**, which is why this row is the
/// right fixture: its only keys are `evidence:` and `satisfied:`, both known,
/// so it has nothing to report and was destroyed anyway. **Every row is walked;
/// only some have anything to say.**
///
/// Measured across the estates on this machine: **179 rows in 7 projects**, not
/// the 2 the bug report named -- Lamplight 58, Baize 42, Laksa 5, and 68 in
/// Intent's own tree.
#[test]
fn a_row_carrying_a_multibyte_character_arrives_instead_of_aborting_the_scan() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "## Acceptance Criteria\n\n\
     - AC-01.1 (non-test) A stored-but-unverified credential is not rendered with the same affirmative marker as a verified one. -- evidence: `model list` column renamed `Key stored` with `yes`/`none` in place of `\u{2713}`/`\u{2717}`, plus a footer pointing at `verify-key` -- satisfied: yes\n\
     \n## Acceptance Tests\n\n\
     - AT-01.1 test/a_test.exs -- covers AC-01.1 -- status: green\n",
  );

  let scan = legacy::scan(&fixture.project()).expect("the scan completes rather than panicking");

  assert!(
    details(&scan).iter().all(|d| !d.contains("does not read")),
    "and the row has NO unread field -- both its keys are known -- so a fix that \
     reports one has invented it: {:?}",
    details(&scan)
  );
}

/// **THE ARM THAT FORBIDS THE LAZY FIX: the walk must CONTINUE past the
/// multibyte character, not stop at it.**
///
/// Skipping any row containing non-ASCII would pass the arm above and silently
/// lose every unread field that sits to the RIGHT of a checkmark -- trading a
/// loud panic for the exact silent loss this whole class was built to end.
/// Here `rounds:` follows the `\u{2713}`, so a fix that bails early goes red. The keys
/// are ones the reader does NOT know, so the arm cannot be satisfied by a parser
/// that merely learned this month's vocabulary.
#[test]
fn an_unread_field_after_a_multibyte_character_is_still_named() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "## Acceptance Criteria\n\n\
     - AC-01.1 The run showed `\u{2713}` twice. -- rounds: 3 -- control: none -- on: 2026-08-21\n\
     \n## Acceptance Tests\n\n\
     - AT-01.1 test/a_test.exs -- covers AC-01.1 -- status: green\n",
  );

  let scan = legacy::scan(&fixture.project()).expect("the scan completes");
  let said = details(&scan);
  let named = said
    .iter()
    .find(|d| d.contains("AC-01.1") && d.contains("does not read"))
    .unwrap_or_else(|| panic!("a key to the right of the checkmark is still a key: {said:?}"));
  for key in ["rounds", "control", "on"] {
    assert!(
      named.contains(&format!("`{key}`")),
      "`{key}` missing from {named}"
    );
  }
}

/// **AND THE AT SIDE, whose span is bounded at ` -- status: ` rather than being
/// the whole row.**
///
/// The bound is real but nearly worthless against this defect: of the 180
/// non-ASCII rows on this machine, it spares exactly ONE. The multibyte
/// character here sits BEFORE the status field, inside the scanned span, which
/// is the case the bound cannot help with.
///
/// **AND IT SITS AT BRACKET DEPTH 0, WHICH THIS TEST GOT WRONG ONCE.** The
/// first version of this fixture wrote the checkmark inside `(...)`, and it
/// passed against the unfixed scanner -- because the slice is guarded by
/// `depth == 0 &&`, and `&&` short-circuits, so a bracketed multibyte character
/// is never sliced and never panics. That is a real property (it is why the
/// fleet count is 161 rather than 179), but as a fixture it was a test passing
/// without touching its subject. The checkmark below is unbracketed.
#[test]
fn a_multibyte_character_before_the_at_status_field_does_not_abort_the_scan() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "## Acceptance Criteria\n\n\
     - AC-01.1 It holds. -- evidence: x -- satisfied: yes\n\
     \n## Acceptance Tests\n\n\
     - AT-01.1 test/a_test.exs -- covers AC-01.1 -- audit: hv signed \u{2713} -- status: green\n",
  );

  let scan = legacy::scan(&fixture.project()).expect("the scan completes");
  let said = details(&scan);
  assert!(
    said
      .iter()
      .any(|d| d.contains("AT-01.1") && d.contains("audit")),
    "and the unread key before the status bound is still reported: {said:?}"
  );
}
