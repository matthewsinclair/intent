//! **ONE SENTENCE, TWO ALPHABETS: A KEYED FIELD'S VALUE IS ITS LEADING TOKEN,
//! AND WHAT FOLLOWS THE TOKEN IS ANNOTATION.**
//!
//! `status:` and `satisfied:` learned that rule at `1583d1ad`. The covers
//! clause had never learned it: it cut an id at the first ` (`, so a qualifier
//! written without parentheses became part of the id, matched no criterion, and
//! was reported as a dangling reference against a contract sitting in the same
//! file. Same defect, same fix, different alphabet.
//!
//! **AND THE HALF THAT STOPS THE RULE FROM WIDENING WHAT IS BELIEVED.** A bare
//! leading run would read ``satisfied: yes|no` on the AC line; test-backed ACs
//! are...`` -- documentation ABOUT the field, 191 rows of it fleet-wide -- as a
//! `satisfied: yes`. So the token counts only when the character after it is
//! one this vocabulary can be followed by. Reading prose as data is not a
//! recovered row; it is an invented one.
//!
//! Measured before it was written, and RE-measured after a ratified boundary
//! sent it back: 34 rows across 13 Lamplight files get correct ids where they
//! got garbage, 4 `status:` rows recover from behind an emphasised
//! parenthetical, ZERO rows regress, and all 243 documentation rows keep
//! refusing.
//!
//! **The first draft claimed 7 recovered and was wrong**, because it stripped
//! `*` and `_` off both ends of the value. That defeated
//! `a_note_wrapped_in_markdown_emphasis_is_still_refused` through the back
//! door -- trimming the trailing `_` of `satisfied: yes _(...)_` left a `)` for
//! `strip_suffix` to find. The one `satisfied:` row among the seven is
//! deliberately still refused; re-ruling it is a separate decision, which is
//! what that test asked for in as many words.

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

/// **A QUALIFIER CARRYING ITS OWN ` -- ` NO LONGER TRUNCATES THE COVERS SPAN.**
///
/// Real shape, Lamplight `ST0222` in `COMPLETED/`: `AC-04.2 (canon read-only +
/// Canon/Training tab differentiation -- the un-gated half) -- status: green`.
/// `covers` had its own non-bracket-aware spelling of "where does this value
/// end", cut at the ` -- ` INSIDE the parenthetical, and took the qualifier
/// with it in silence. Seven rows fleet-wide, every one of them in a closed
/// thread, so every one of them carried rather than blocked.
#[test]
fn a_separator_inside_a_qualifier_does_not_truncate_the_covers_span() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "## Acceptance Criteria\n\n\
     - AC-04.2 (non-test) The gate -- evidence: e -- satisfied: yes\n\
     \n## Acceptance Tests\n\n\
     - AT-04.1 test/a_test.exs (1 test) -- covers AC-04.2 (canon read-only -- the un-gated half) -- status: green\n",
  );
  let scan = legacy::scan(&fixture.project()).expect("the row reads");

  assert_eq!(
    scan.threads[0].tests[0].covers,
    vec!["AC-04.2".to_string()],
    "the id is the leading token, and the parenthetical is not part of it"
  );
  // **THE QUALIFIER IS THE LOAD-BEARING ASSERTION, AND AN EARLIER DRAFT OF THIS
  // TEST DID NOT MAKE IT.** The truncation does not damage the ID -- `AC-04.2`
  // survives the old cut perfectly well, because the cut lands AFTER it. What
  // the old cut eats is the second half of the QUALIFIER, so a test that
  // checked only `covers` passed with the defect fully present. vc caught it by
  // mutation: restoring `rest.find(" -- ")` at the covers span left this test
  // GREEN. **A test that passes for a reason other than the one it names is the
  // well-formed substitute this file exists to refuse.**
  let note = scan.threads[0].tests[0].note.as_deref().unwrap_or_default();
  assert!(
    note.ends_with("AC-04.2: canon read-only -- the un-gated half"),
    "the qualifier must survive WHOLE, keyed to its id: {note:?}"
  );
  assert!(
    !note.ends_with("canon read-only"),
    "and it must not stop at the ` -- ` inside the parenthetical, which is \
     exactly where the old cut ended it: {note:?}"
  );
  assert!(
    scan.residue.is_empty() && scan.carried.is_empty(),
    "a row whose covers resolve manufactures no finding: {:?}{:?}",
    scan.residue,
    scan.carried
  );
}

/// **A COMMA INSIDE A QUALIFIER IS NOT A LIST SEPARATOR.**
///
/// Lamplight `ST0238`: `AC-05.1 (asserts ...), AC-05.2 (asserts `room_read` +
/// `cecilia_shielded` both false, so the naming's requires cannot hold`. A bare
/// `split(',')` shredded it into a THIRD span beginning `so the na`, which
/// resolved against nothing and was reported as a broken reference to a
/// criterion no author ever wrote.
#[test]
fn a_comma_inside_a_qualifier_does_not_become_a_second_id() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "## Acceptance Criteria\n\n\
     - AC-05.1 (non-test) One -- evidence: e -- satisfied: yes\n\
     - AC-05.2 (non-test) Two -- evidence: e -- satisfied: yes\n\
     \n## Acceptance Tests\n\n\
     - AT-05.1 test/a_test.exs (1 test) -- covers AC-05.1 (asserts a, then b), AC-05.2 (asserts c, so d) -- status: green\n",
  );
  let scan = legacy::scan(&fixture.project()).expect("the row reads");

  assert_eq!(
    scan.threads[0].tests[0].covers,
    vec!["AC-05.1".to_string(), "AC-05.2".to_string()],
    "two ids, not three, and no prose among them"
  );
  assert!(
    scan.residue.is_empty(),
    "the shredded tail used to arrive as a BrokenReference: {:?}",
    scan.residue
  );
}

/// **A QUALIFIER WRITTEN WITHOUT PARENTHESES IS STILL A QUALIFIER.**
///
/// vc's shape, simulated against `covers` before the file was touched:
/// `covers AC-00.3 clause 3` took everything before the first ` (` as the id --
/// and with no ` (` present, that was the whole span. The estate carries zero
/// instances because lamplight-vc rewrote the one row to the documented form,
/// so this fixture is constructed and says so.
#[test]
fn an_id_qualified_without_parentheses_is_still_an_id() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "## Acceptance Criteria\n\n\
     - AC-00.3 (non-test) A clause -- evidence: e -- satisfied: yes\n\
     \n## Acceptance Tests\n\n\
     - AT-00.1 test/a_test.exs (1 test) -- covers AC-00.3 clause 3 -- status: green\n",
  );
  let scan = legacy::scan(&fixture.project()).expect("the row reads");

  assert_eq!(
    scan.threads[0].tests[0].covers,
    vec!["AC-00.3".to_string()],
    "the id ends at the first character that is not id-shaped"
  );
  assert!(
    scan.residue.is_empty(),
    "and no broken reference is manufactured: {:?}",
    scan.residue
  );
}

/// **A SPAN CARRYING NO ID IS NAMED, NOT PUSHED INTO THE ID LIST.**
///
/// Laksa `ST0086`: `covers AC-10.1, and retrospectively guards AC-00.1,
/// AC-03.2, AC-06.4`. The good ids must survive -- refusing the whole row would
/// lose an AT that arrives correctly today -- and the prose span must be
/// reported rather than dropped, because a silent drop and a clean row are the
/// same `covers.len()`.
#[test]
fn a_span_that_carries_no_criterion_id_is_named_rather_than_pushed_as_one() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "## Acceptance Criteria\n\n\
     - AC-10.1 (non-test) One -- evidence: e -- satisfied: yes\n\
     \n## Acceptance Tests\n\n\
     - AT-10.1 test/a_test.exs (1 test) -- covers AC-10.1, and retrospectively guards it -- status: green\n",
  );
  let scan = legacy::scan(&fixture.project()).expect("the row still arrives");

  assert_eq!(
    scan.threads[0].tests[0].covers,
    vec!["AC-10.1".to_string()],
    "the readable id survives"
  );
  assert!(
    scan
      .residue
      .iter()
      .any(|f| f.detail.contains("and retrospectively guards it")),
    "and the unreadable span is quoted back rather than dropped: {:?}",
    scan.residue
  );
}

/// **A TOKEN FOLLOWED BY AN ALTERNATION IS NOT THAT TOKEN.**
///
/// This is the half that keeps the generalised rule honest, and it is the
/// reason the rule is not simply "the leading run". The fleet carries 191 rows
/// of prose ABOUT the field whose leading run is a perfectly good `yes`. They
/// refuse today; reading them would not recover a row, it would invent one.
#[test]
fn a_token_followed_by_an_alternation_is_refused_rather_than_read() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "## Acceptance Criteria\n\n\
     - AC-01.1 (non-test) One -- evidence: e -- satisfied: yes|no is written on the AC line\n\
     \n## Acceptance Tests\n\n\
     - AT-01.1 test/a_test.exs (1 test) -- covers AC-01.1 -- status: green\n",
  );
  let scan =
    legacy::scan(&fixture.project()).expect("a NAMED refusal is not an accounting failure");

  assert!(
    scan.threads[0].criteria.is_empty(),
    "documentation about the field is not a verdict"
  );
  assert!(
    scan
      .residue
      .iter()
      .any(|f| f.detail.contains("yes|no is written on the AC line")),
    "and the refusal quotes the whole value, not the fragment it was willing to read: {:?}",
    scan.residue
  );
}

/// **EMPHASIS AFTER THE TOKEN IS ANNOTATION; EMPHASIS WRAPPING IT IS NOT.**
///
/// `to-write **(gate, not a test)**.` reads, because the leading run is a clean
/// `to-write` followed by a space. `**green**` does NOT, because the run is
/// empty and there is no token to be followed by anything.
///
/// **The wrapping case is a RATIFIED BOUNDARY, not a gap here.**
/// `legacy_satisfied_verdict.rs` holds it explicitly: widening the verdict
/// vocabulary is a separate ruling from widening where a field ENDS, and the
/// two must not ride in together. An earlier draft of this change stripped `*`
/// and `_` from both ends, which recovered a few rows and -- through the
/// trailing `_` of `satisfied: yes _(...)_` -- silently defeated that boundary.
/// The test caught it. Re-ruling it is hv's and vc's, not this commit's.
#[test]
fn emphasis_after_the_token_reads_and_emphasis_wrapping_it_does_not() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "## Acceptance Criteria\n\n\
     - AC-01.1 (non-test) One -- evidence: e -- satisfied: yes\n\
     \n## Acceptance Tests\n\n\
     - AT-01.1 test/a_test.exs (1 test) -- covers AC-01.1 -- status: to-write **(gate, not a test)**. Same shape as ST0124\n\
     - AT-01.2 test/b_test.exs (1 test) -- covers AC-01.1 -- status: **green**\n",
  );
  let scan =
    legacy::scan(&fixture.project()).expect("a NAMED refusal is not an accounting failure");

  assert_eq!(
    scan.threads[0].tests.len(),
    1,
    "the first row reads its token and carries the emphasised parenthetical as annotation"
  );
  assert!(
    scan.residue.iter().any(|f| f.detail.contains("**green**")),
    "and the wrapped one is refused by its WHOLE value, not by a fragment: {:?}",
    scan.residue
  );
}

/// **A ROW WHOSE COVERS ARE ALL PROSE STILL ARRIVES, COVERING NOTHING.**
///
/// The arca_cli shape, and the one that caught this: `covers the gate itself`.
/// The old id cut pushed that prose in AS AN ID, so the row arrived carrying a
/// reference that resolved against nothing. Tightening the id rule without this
/// turned three real rows into three LOST rows -- the corpus count fell from 55
/// to 52 and nothing else said so.
///
/// **A wrong value is visible and correctable. An absent one is not.**
#[test]
fn a_row_whose_covers_are_all_prose_still_arrives_covering_nothing() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "## Acceptance Criteria\n\n\
     - AC-07.1 (non-test) One -- evidence: e -- satisfied: yes\n\
     \n## Acceptance Tests\n\n\
     - AT-07.3 test/a_test.exs (1 test) -- covers the gate itself -- status: green\n",
  );
  let scan = legacy::scan(&fixture.project()).expect("the row must not be lost");

  assert_eq!(scan.threads[0].tests.len(), 1, "the row ARRIVES");
  assert!(
    scan.threads[0].tests[0].covers.is_empty(),
    "covering nothing -- and an empty covers list satisfies nothing, so this \
     cannot come to settle a criterion by accident: {:?}",
    scan.threads[0].tests[0].covers
  );
  assert!(
    scan
      .residue
      .iter()
      .any(|f| f.detail.contains("the gate itself")),
    "and the span is quoted back by name: {:?}",
    scan.residue
  );
}

/// **A ROW WITH NO ` -- covers ` CLAUSE AT ALL ALSO ARRIVES**, and says so in a
/// DIFFERENT message.
///
/// Two ways of covering nothing, kept apart deliberately: a span of prose where
/// an id was expected is an author writing an id badly; no clause at all is an
/// author not writing one. Collapsing them would send an operator to go and fix
/// a span that is not there.
#[test]
fn a_row_with_no_covers_clause_arrives_and_says_which_kind_of_nothing() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "## Acceptance Criteria\n\n\
     - AC-16.1 (non-test) One -- evidence: e -- satisfied: yes\n\
     \n## Acceptance Tests\n\n\
     - AT-16.1 test/a_test.exs (1 test) -- status: green\n",
  );
  let scan = legacy::scan(&fixture.project()).expect("the row must not be lost");

  assert_eq!(scan.threads[0].tests.len(), 1, "the row ARRIVES");
  assert!(scan.threads[0].tests[0].covers.is_empty());
  assert!(
    scan
      .residue
      .iter()
      .any(|f| f.detail.contains("has no ` -- covers ` clause")),
    "named as an ABSENT clause, not as an unreadable span: {:?}",
    scan.residue
  );
}
