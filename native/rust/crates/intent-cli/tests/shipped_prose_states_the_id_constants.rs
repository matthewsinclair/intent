//! **SHIPPED PROSE THAT RESTATES AN ID CONSTANT IS HELD TO IT** (ST1c2db93
//! P28a). The root-file template every project's `AGENTS.md` renders from and
//! the skills an agent loads each tell a reader how an id is spelled: how many
//! hex a git-style id shows, how few a typed prefix may carry, what an
//! ordinal looks like. Those numbers live in `intentsvcs::id`, and the prose
//! is written by hand, so a change to a constant would leave the prose
//! teaching the old shape with nothing failing. This file builds each phrase
//! from the constants and asserts the prose carries it, so either side moving
//! alone fails here.

use intentsvcs::id::{
  CLAIM_ADDRESS_FORMS, EXAMPLE_HEX, EXAMPLE_WP_HEX, ISSUE_DIGITS, ISSUE_MARKER, MIN_PREFIX_HEX,
  SHOWN_HEX, THREAD_DIGITS, THREAD_PREFIX, WP_DIGITS, WP_SHOWN_HEX, prefix_floor,
};
use testkit::repo_root;

/// The root-file template `AGENTS.md` renders from.
const AGENTS_TEMPLATE: &str = "lib/templates/llm/_AGENTS.md";
/// The essentials skill every session loads.
const IN_ESSENTIALS: &str = "intent/plugins/claude/skills/in-essentials/SKILL.md";
/// The whiteboard skill, which teaches a claim's address.
const IN_WHITEBOARD: &str = "intent/plugins/claude/skills/in-whiteboard/SKILL.md";

/// A count as the prose spells it. Panics on a count the prose has no word
/// for, which is a prompt to extend this table, not to change the prose.
fn word(n: usize) -> &'static str {
  const WORDS: [&str; 11] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
  ];
  WORDS
    .get(n)
    .copied()
    .unwrap_or_else(|| panic!("no word for {n}: extend this table"))
}

fn read(rel: &str) -> String {
  let path = repo_root().join(rel);
  std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn assert_states(rel: &str, text: &str, phrase: &str) {
  assert!(
    text.contains(phrase),
    "{rel} no longer states `{phrase}`, which `intentsvcs::id`'s constants spell: \
     the prose and the constants have parted"
  );
}

/// The prefix rule: the fewest hex a typed prefix carries, and the more it
/// needs when it is all digits, read from `prefix_floor` itself.
fn prefix_phrase() -> String {
  let all_digits = prefix_floor(&"0".repeat(MIN_PREFIX_HEX), THREAD_DIGITS);
  format!(
    "any unique prefix of at least {} hex of a git-style id, or {} if they are all digits",
    word(MIN_PREFIX_HEX),
    word(all_digits)
  )
}

/// A thread's, a package's and an issue's id as a face shows them, and as an
/// ordinal one is spelled, each built from the constants.
fn shown_thread() -> String {
  format!("{THREAD_PREFIX}{}", &EXAMPLE_HEX[..SHOWN_HEX])
}
fn shown_package() -> String {
  format!("{}/{}", shown_thread(), &EXAMPLE_WP_HEX[..WP_SHOWN_HEX])
}
fn shown_issue() -> String {
  format!("{ISSUE_MARKER}{}", &EXAMPLE_HEX[..SHOWN_HEX])
}
fn ordinal_thread() -> String {
  format!("{THREAD_PREFIX}{}", "0".repeat(THREAD_DIGITS))
}
fn ordinal_package() -> String {
  format!("{}/{}", ordinal_thread(), "0".repeat(WP_DIGITS - 1) + "1")
}
fn ordinal_issue() -> String {
  "0".repeat(ISSUE_DIGITS)
}

#[test]
fn the_agents_template_states_the_id_constants() {
  let text = read(AGENTS_TEMPLATE);
  for phrase in [
    prefix_phrase(),
    format!("`{}` for a thread", shown_thread()),
    format!("`{}` for one of its work packages", shown_package()),
    format!("`{}` for an issue", shown_issue()),
    format!(
      "`{}`, `{}`, issue `{}`",
      ordinal_thread(),
      ordinal_package(),
      ordinal_issue()
    ),
  ] {
    assert_states(AGENTS_TEMPLATE, &text, &phrase);
  }
}

#[test]
fn the_essentials_skill_states_the_id_constants() {
  let text = read(IN_ESSENTIALS);
  for phrase in [
    prefix_phrase(),
    format!(
      "a thread shows as `{THREAD_PREFIX}` and {} hex, eg `{}`",
      word(SHOWN_HEX),
      shown_thread()
    ),
    format!(
      "an issue as `{ISSUE_MARKER}` and {} hex, eg `{}`",
      word(SHOWN_HEX),
      shown_issue()
    ),
    format!(
      "a slash and {} hex, eg `{}`",
      word(WP_SHOWN_HEX),
      shown_package()
    ),
    format!(
      "`{}`, issue `{}`, work package `{}`",
      ordinal_thread(),
      ordinal_issue(),
      ordinal_package()
    ),
  ] {
    assert_states(IN_ESSENTIALS, &text, &phrase);
  }
}

/// The whiteboard skill teaches a claim's address in both families, each id
/// whole as a board stores it, by quoting `id::CLAIM_ADDRESS_FORMS`, the one
/// sentence every claim refusal and MCP's label descriptions quote too.
#[test]
fn the_whiteboard_skill_states_the_id_constants() {
  assert_states(IN_WHITEBOARD, &read(IN_WHITEBOARD), CLAIM_ADDRESS_FORMS);
}
