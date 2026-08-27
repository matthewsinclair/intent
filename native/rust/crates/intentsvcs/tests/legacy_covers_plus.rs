//! **TWELVE COVERAGE LINKS ARE SILENTLY SWALLOWED INTO QUALIFIER PROSE,
//! BECAUSE `+` IS NOT A SEPARATOR AND THE LEADING-TOKEN RULE TAKES THE FIRST
//! ID AND CALLS THE REST AN ANNOTATION.**
//!
//! `split_outside_brackets` splits a covers span on `,` at depth 0 and on
//! nothing else. So `covers AC-13.1 + AC-13.4` is ONE span: the id is the
//! leading token `AC-13.1`, and `+ AC-13.4` becomes that criterion's
//! qualifier. The row arrives, the accounting closes, and `AC-13.4` simply has
//! one fewer covering test than its author wrote.
//!
//! # Measured, estate-wide, 2026-08-27
//!
//! 32 covers spans contain a `+`, and **20 of them are inside a parenthetical**
//! -- `AC-05.1 (path-transition render + first-visit dedup)` -- where the depth
//! guard already does the right thing and a naive split would shred a qualifier
//! into prose that then gets read as an id. That is the control below, and it
//! is why this splits at DEPTH 0 rather than on the character.
//!
//! **Twelve are a real top-level join, and they divide 9 / 3:**
//!
//! - **9 whose second operand LEADS with an id** -- `AC-13.1 + AC-13.4`,
//!   `AC-03.2 (aggregate + non-zero exit) + AC-03.1 (the --only error legs)`.
//!   These recover exactly, because once `+` separates them the existing
//!   leading-token rule reads each side unchanged.
//! - **3 whose second operand is PROSE mentioning an id** -- `the pure part of
//!   AC-06.1`, `the pure part of AC-05.1`, `the WP-12 slice of AC-10.3`.
//!
//! # The three are NAMED RESIDUE and their id is deliberately NOT scraped out
//!
//! A regex would find `AC-06.1` in "the pure part of AC-06.1" and the coverage
//! link would look recovered. **It would be a claim the author declined to
//! make**: they wrote that the test covers PART of that criterion, and
//! recording a full covering relation asserts something stronger than the row
//! says. `covers` has no way to express "partly", so the honest outcome is a
//! finding that quotes the span and lets a human decide.
//!
//! **This is the same class as reading prose ABOUT a marker as a marker**, which
//! cost `ST0056 AC-03.17` a silent kind-flip one commit ago. An id inside a
//! sentence is a mention, not a reference.

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

/// Every criterion the AT with `id` covers, in order.
fn covered(scan: &legacy::Scan, id: &str) -> Vec<String> {
  scan.threads[0]
    .tests
    .iter()
    .find(|t| t.id == id)
    .unwrap_or_else(|| panic!("{id} reaches canon"))
    .covers
    .clone()
}

fn details(scan: &legacy::Scan) -> Vec<String> {
  scan
    .residue
    .iter()
    .chain(scan.carried.iter())
    .map(|f| f.detail.clone())
    .collect()
}

const CRITERIA: &str = "## Acceptance Criteria\n\n\
   - AC-13.1 One. -- satisfied: yes\n\
   - AC-13.2 Two. -- satisfied: yes\n\
   - AC-13.3 Three. -- satisfied: yes\n\
   - AC-13.4 Four. -- satisfied: yes\n\
   - AC-06.1 Five. -- satisfied: yes\n\
   - AC-06.2 Six. -- satisfied: yes\n\
   - AC-03.1 Seven. -- satisfied: yes\n\
   - AC-03.2 Eight. -- satisfied: yes\n\
   - AC-05.1 Nine. -- satisfied: yes\n\
   \n## Acceptance Tests\n\n";

/// **THE HEADLINE.** Lamplight `ST0257 AT-13.1`, verbatim.
#[test]
fn a_plus_joins_two_criteria_and_the_second_is_not_lost() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    &format!("{CRITERIA}- AT-13.1 test/a_test.exs -- covers AC-13.1 + AC-13.4 -- status: green\n"),
  );
  let scan = legacy::scan(&fixture.project()).expect("scan");
  assert_eq!(
    covered(&scan, "AT-13.1"),
    vec!["AC-13.1".to_string(), "AC-13.4".to_string()],
    "`+` joins two criteria. Reading it as a qualifier keeps the first and turns the second into \
     annotation prose, so AC-13.4 loses a covering test its author wrote and nothing reports it"
  );
}

/// **CONTROL, AND THE REASON THIS SPLITS AT DEPTH 0 RATHER THAN ON THE
/// CHARACTER.** 20 of the estate's 32 `+` spans look like this one.
#[test]
fn a_plus_inside_a_qualifier_is_not_a_separator() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    &format!(
      "{CRITERIA}- AT-05.1 test/a_test.exs -- covers AC-05.1 (path-transition render + first-visit dedup) -- status: green\n"
    ),
  );
  let scan = legacy::scan(&fixture.project()).expect("scan");
  assert_eq!(
    covered(&scan, "AT-05.1"),
    vec!["AC-05.1".to_string()],
    "the `+` is inside the author's parenthetical. Splitting on it shreds the qualifier and the \
     tail becomes prose that is then read as an id -- the exact failure `split_outside_brackets` \
     was written to stop, reintroduced one character over"
  );
  assert!(
    details(&scan)
      .iter()
      .all(|d| !d.contains("carries no criterion id")),
    "and nothing is reported unreadable: {:?}",
    details(&scan)
  );
}

/// Both operands carrying their own parenthetical. Lamplight `ST0305 AT-03.2`.
#[test]
fn both_operands_keep_their_own_qualifier() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    &format!(
      "{CRITERIA}- AT-03.2 test/a_test.exs -- covers AC-03.2 (aggregate + non-zero exit) + AC-03.1 (the --only error legs) -- status: green\n"
    ),
  );
  let scan = legacy::scan(&fixture.project()).expect("scan");
  assert_eq!(
    covered(&scan, "AT-03.2"),
    vec!["AC-03.2".to_string(), "AC-03.1".to_string()],
    "one `+` is inside a qualifier and one is a separator, in the same span"
  );
}

/// **A COMMA AND A PLUS COMPOSE, because the estate writes both.**
#[test]
fn a_comma_and_a_plus_compose() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    &format!(
      "{CRITERIA}- AT-13.2 test/a_test.exs -- covers AC-13.2 (one), AC-13.3 + AC-13.4 -- status: green\n"
    ),
  );
  let scan = legacy::scan(&fixture.project()).expect("scan");
  assert_eq!(
    covered(&scan, "AT-13.2"),
    vec![
      "AC-13.2".to_string(),
      "AC-13.3".to_string(),
      "AC-13.4".to_string()
    ]
  );
}

/// **THE THREE. An id inside a sentence is a MENTION, not a reference.**
///
/// Lamplight `ST0327 AT-06.1`, verbatim. Scraping `AC-06.1` out of "the pure
/// part of AC-06.1" would record a full covering relation the author explicitly
/// declined to write -- `covers` cannot say "partly" -- so the span is quoted
/// back and a human decides.
#[test]
fn a_plus_operand_that_is_prose_becomes_named_residue() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    &format!(
      "{CRITERIA}- AT-06.1 test/a_test.exs -- covers AC-06.2 + the pure part of AC-06.1 -- status: green\n"
    ),
  );
  let scan = legacy::scan(&fixture.project()).expect("scan");
  assert_eq!(
    covered(&scan, "AT-06.1"),
    vec!["AC-06.2".to_string()],
    "the readable operand still resolves"
  );
  let said = details(&scan);
  assert!(
    said
      .iter()
      .any(|d| d.contains("the pure part of AC-06.1") && d.contains("carries no criterion id")),
    "the unreadable operand is QUOTED BACK, not dropped and not scraped: {said:?}"
  );
  assert!(
    !covered(&scan, "AT-06.1").contains(&"AC-06.1".to_string()),
    "recording AC-06.1 as covered asserts something stronger than the row says -- the author \
     wrote that the test covers PART of it, and `covers` has no way to say partly"
  );
}
