//! WP-11: the DEFAULT declaration is every OPEN thread and nothing else, and it
//! is ONE function with four callers.
//!
//! **THE LOAD-BEARING TEST HERE IS THE ROUND TRIP.** A generated manifest the
//! tool's own parser refuses is the worst available outcome: the grammar ABORTS
//! on a line it cannot read, so a bad header or a mis-spelled sigil would not
//! degrade gracefully -- it would take out every subsequent `organize`, `doctor`
//! and `migrate` run on the estate that generated it. Asserting the ids are
//! present says nothing about whether the file can be read back.
//!
//! **OPEN IS ASSERTED AS A PREDICATE, NOT AS A LIST.** WP-11 names the four open
//! statuses, but the implementation asks `!is_closed()` so that a sixth status
//! cannot silently drop out of every project's default the day it is added.
//! `every_open_status_is_declared` drives all four rather than a representative
//! one, because a filter that happened to key on `Wip` alone would pass a
//! single-status test and quietly realise nothing for a Triage-heavy estate.

use intentsvcs::intentfiles::{default_declaration, parse};
use intentsvcs::model::ThreadStatus;

fn declared(text: &str) -> Vec<String> {
  parse(text)
    .expect("a generated default must parse with the tool's own grammar")
    .entries
    .iter()
    .map(|e| e.id.clone())
    .collect()
}

/// THE DISCRIMINATING PAIR: the same id, once open and once closed. Without both
/// halves, an empty result proves nothing -- it could mean the filter works, or
/// that the function emits no declarations at all.
#[test]
fn an_open_thread_is_declared_and_the_same_id_closed_is_not() {
  let open = default_declaration(&[("ST0056".to_string(), ThreadStatus::Wip)]);
  assert_eq!(
    declared(&open),
    ["ST0056"],
    "an open thread must be declared"
  );

  let closed = default_declaration(&[("ST0056".to_string(), ThreadStatus::Completed)]);
  assert!(
    declared(&closed).is_empty(),
    "the SAME id, closed, must not be declared -- if this list is non-empty the \
     filter is not reading status at all"
  );
}

/// All four, because a filter keyed on one open status passes a one-status test.
#[test]
fn every_open_status_is_declared() {
  let threads = vec![
    ("ST0001".to_string(), ThreadStatus::Wip),
    ("ST0002".to_string(), ThreadStatus::Triage),
    ("ST0003".to_string(), ThreadStatus::NotStarted),
    ("ST0004".to_string(), ThreadStatus::Hold),
    ("ST0005".to_string(), ThreadStatus::Completed),
    ("ST0006".to_string(), ThreadStatus::Cancelled),
  ];
  assert_eq!(
    declared(&default_declaration(&threads)),
    ["ST0001", "ST0002", "ST0003", "ST0004"],
    "every OPEN status is declared and both closed statuses are not"
  );
}

/// Committed and diffed across nineteen estates, so iteration order must not
/// reach the file.
#[test]
fn the_declaration_is_sorted_regardless_of_input_order() {
  let scrambled = vec![
    ("ST0064".to_string(), ThreadStatus::Wip),
    ("ST0002".to_string(), ThreadStatus::Wip),
    ("ST0046".to_string(), ThreadStatus::Wip),
  ];
  assert_eq!(
    declared(&default_declaration(&scrambled)),
    ["ST0002", "ST0046", "ST0064"]
  );
}

/// `intent init`'s case, and the distinction the manifest's own header turns on:
/// PRESENT-and-declaring-nothing means keep nothing; ABSENT means nobody has
/// said. A header-only file is the first, and it must still parse.
#[test]
fn an_empty_project_yields_a_header_that_declares_nothing_and_still_parses() {
  let text = default_declaration(&[]);
  assert!(
    declared(&text).is_empty(),
    "a fresh project declares nothing"
  );
  assert!(
    text.starts_with("# .intentfiles"),
    "the generated file must carry its header, or the next reader has no way to \
     learn what the grammar is.\ngot:\n{text}"
  );
}

/// The round trip, stated as its own test because everything above depends on it
/// and a failure here means the others are testing a file nobody can read.
#[test]
fn a_generated_default_is_readable_by_the_grammar_that_will_read_it() {
  let threads = vec![
    ("ST0056".to_string(), ThreadStatus::Wip),
    ("ST0057".to_string(), ThreadStatus::Triage),
  ];
  let text = default_declaration(&threads);

  let parsed = parse(&text).unwrap_or_else(|e| {
    panic!("the tool cannot read what the tool just wrote: {e}\ngenerated:\n{text}")
  });
  assert_eq!(parsed.entries.len(), 2);
}
