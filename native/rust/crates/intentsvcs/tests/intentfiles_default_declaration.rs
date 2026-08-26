//! WP-11: the DEFAULT declaration is every **WIP** thread and nothing else, and
//! it is ONE function with four callers.
//!
//! **THE LOAD-BEARING TEST HERE IS THE ROUND TRIP.** A generated manifest the
//! tool's own parser refuses is the worst available outcome: the grammar ABORTS
//! on a line it cannot read, so a bad header or a mis-spelled sigil would not
//! degrade gracefully -- it would take out every subsequent `organize`, `doctor`
//! and `migrate` run on the estate that generated it. Asserting the ids are
//! present says nothing about whether the file can be read back.
//!
//! **THIS FILE ARGUED THE OPPOSITE UNTIL 2026-08-26, AND THE REVERSAL IS WORTH
//! KEEPING RATHER THAN TIDYING AWAY.** It used to read: *open is asserted as a
//! predicate, not as a list ... a filter that happened to key on `Wip` alone
//! would pass a single-status test and quietly realise nothing for a
//! Triage-heavy estate.* That reasoning was sound and its premise was wrong.
//! **`!is_closed()` is a definition by EXCLUSION**, so every status nobody
//! thought about is swept IN by default -- which is how a fleet project came to
//! realise 57 threads. hv, first-hand, on seeing it: *"Now it has NOT STARTED
//! STs!??!"* and *"It should ONLY HAVE WIP STs!!!!!"*
//!
//! So the predicate is now stated POSITIVELY -- `status == Wip` -- and the
//! property that changed is which direction a new status defaults to. Under the
//! old rule a seventh status would be realised without anyone deciding; under
//! this one it is not realised until someone says so. **A set that cannot
//! acquire members by accident is the one worth having here**, because the
//! accident puts files on disk that no work refers to.
//!
//! `only_wip_is_declared` drives all six statuses rather than a representative
//! pair, so the four that must NOT be declared are each named.

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

/// All six, so each status that must NOT be realised is named rather than
/// covered by a representative.
#[test]
fn only_wip_is_declared() {
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
    ["ST0001"],
    "WIP alone is realised. Triage, Not Started and Hold are NOT -- they were \
     under the old `!is_closed()` rule, which is the defect hv found on a \
     57-thread estate, and Completed and Cancelled never were"
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
  // BOTH Wip, because this test's subject is the GRAMMAR and not the
  // predicate: a fixture whose second thread is filtered out would assert the
  // round trip over one entry while reading as though it covered two.
  let threads = vec![
    ("ST0056".to_string(), ThreadStatus::Wip),
    ("ST0057".to_string(), ThreadStatus::Wip),
  ];
  let text = default_declaration(&threads);

  let parsed = parse(&text).unwrap_or_else(|e| {
    panic!("the tool cannot read what the tool just wrote: {e}\ngenerated:\n{text}")
  });
  assert_eq!(parsed.entries.len(), 2);
}
