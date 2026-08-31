//! What an operator's spelling NAMES in this project, as distinct from what it
//! SPELLS.
//!
//! # Two doors for two questions, which is not two homes for one
//!
//! [`crate::address::promote`] answers *what does this token spell* and is
//! PURE: it never reads the store, which is why `mcp.rs` can call it three
//! times inside argument validation before any work begins. This module answers
//! *what does it name HERE*, which no amount of grammar can settle, because
//! `0164` is a well-formed id in both families -- `THREAD_DIGITS` and
//! `ISSUE_DIGITS` are both 4 -- and only the project knows which of them is
//! actually there.
//!
//! **THE LADDER LIVES HERE RATHER THAN INSIDE `promote` FOR A REASON THAT IS
//! STRUCTURAL AND NOT STYLISTIC** (hv, 2026-08-31). Putting it inside would
//! make the pure door take a store, and a ladder implemented per verb instead
//! would be the second resolver the whole address ruling exists to prevent.
//!
//! # Presence is INJECTED, exactly as [`crate::nav::land`] injects its own
//!
//! This module never sees a facade. The face that has one passes a probe, so
//! the CLI, the TUI and the MCP tier share one presence rule instead of writing
//! three -- and this crate keeps `resolve` testable against a set literal.

use crate::address::{Address, AddressError, Entity};
use crate::model;

/// What a spelling resolved to.
///
/// **A VALUE, NEVER PROSE, AND THAT IS A CONTRACT RATHER THAN A PREFERENCE**
/// (vc, 2026-08-31). `nav` renders candidates as a list, the MCP tier needs
/// structured data, and a CLI verb prints a refusal -- three different
/// renderings of one fact. A resolver that returned a sentence would force the
/// MCP tier to parse English, which is precisely the defect refused on
/// `daemon status` the same day.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Resolution {
  /// Exactly one thing is named, and this is it.
  Resolved(Address),
  /// The spelling names more than one thing THAT EXISTS, in ladder order.
  ///
  /// **This is the variant that makes the whole exercise honest.** See
  /// [`resolve`] for why a precedence rule was refused here.
  Ambiguous(Vec<Address>),
  /// The spelling is well formed and names nothing in this project.
  ///
  /// **`searched` carries what was LOOKED FOR, not what was found**, so a face
  /// can say where it looked. A bare *nothing found* sends an operator hunting
  /// without telling them which two places were already checked.
  Unresolvable { searched: Vec<Address> },
}

/// Resolve an operator's spelling against this project.
///
/// # The ladder is walked in full, and the first hit does not win
///
/// hv ruled a ladder -- thread, then issue -- and a ladder is ordinarily read
/// as precedence: take the first rung that hits. **That reading is refused
/// here, and the measurement is why.** On this estate 48 of 69 thread numbers
/// are also issue numbers (vc), so precedence would silently pick the thread
/// for the majority of ambiguous inputs -- answering confidently about the
/// entity the caller did not name, which is `0189` exactly, one layer up.
///
/// So every rung is probed and the HITS are counted. The order still does real
/// work: it fixes the order of [`Resolution::Ambiguous`]'s candidates and of
/// `searched`, so two runs of the same question report the same way.
///
/// # The third rung has no bare spelling, and that is a fact about work
/// packages rather than an omission here
///
/// hv's ladder names *thread, then issue, then work package*. A work package is
/// addressed `ST0056/03` and its sequence number is meaningless without the
/// thread that owns it -- there is no bare number a caller could type that
/// names one. The rung is therefore unreachable BY CONSTRUCTION rather than
/// unimplemented, and it is recorded here so the next reader meets the reason
/// instead of a missing arm.
///
/// # A spelling that is not ambiguous never reaches the store
///
/// A tagged id (`ST0059`, `s59`, `i59`) or a full address is returned as
/// [`Resolution::Resolved`] WITHOUT a probe. Existence is a different question,
/// and [`crate::nav::Unlanded::Absent`] already answers it; probing here would
/// be a second home for that answer and would make a resolver refuse things
/// that resolve perfectly well.
///
/// # A refusal of the GRAMMAR is an error, not a resolution
///
/// `ST5x` names nothing under any spelling, and that is not an outcome of
/// searching -- nothing was searched. It comes back as the [`AddressError`]
/// `promote` produced, in the parser's own words, so a face can carry the
/// message and remedy that were written where the fact lives.
pub fn resolve(input: &str, exists: impl Fn(&Address) -> bool) -> Result<Resolution, AddressError> {
  let seq = match crate::address::promote(input) {
    Ok(address) => return Ok(Resolution::Resolved(address)),
    Err(AddressError::AmbiguousId { seq, .. }) => seq,
    Err(refusal) => return Err(refusal),
  };

  let searched = rungs(seq);
  let mut found: Vec<Address> = searched.iter().filter(|a| exists(a)).cloned().collect();

  Ok(match found.len() {
    0 => Resolution::Unresolvable { searched },
    // `swap_remove(0)` on a one-element vector is `remove(0)` without the
    // shift, and it cannot panic here because the arm establishes the length.
    1 => Resolution::Resolved(found.swap_remove(0)),
    _ => Resolution::Ambiguous(found),
  })
}

/// The rungs a bare number could name, in hv's ladder order.
///
/// **BOTH IDS ARE SPELLED BY [`crate::model`] AND NEITHER IS FORMATTED HERE.**
/// `model::issue_id` was added for this: before it, the issue spelling existed
/// inline inside `promote` and nowhere else, so a resolver formatting its own
/// would have been the second declaration of a width this module does not own.
/// The [`Entity`] values are CONSTRUCTED rather than built as URL strings and
/// parsed back, which `address_resolution_single_home` refuses by name.
fn rungs(seq: u32) -> Vec<Address> {
  [
    Entity::Thread {
      id: model::thread_id(seq),
    },
    Entity::Issue {
      id: model::issue_id(seq),
    },
  ]
  .into_iter()
  .map(|entity| Address {
    authority: None,
    entity,
    format: None,
  })
  .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  /// Nothing exists. Every ambiguous spelling is unresolvable, and the two
  /// rungs are reported as searched.
  fn barren(_: &Address) -> bool {
    false
  }

  #[test]
  fn a_tagged_spelling_resolves_without_asking_the_store() {
    // **THE PROBE PANICS, WHICH IS THE ASSERTION.** A test that merely
    // observed the right answer would pass for an implementation that probed
    // and ignored the result; this one fails if the store is touched at all.
    let never = |_: &Address| panic!("a tagged spelling must not reach the store");
    for spelling in ["ST0059", "s59", "i59"] {
      assert!(
        matches!(resolve(spelling, never), Ok(Resolution::Resolved(_))),
        "{spelling} is unambiguous and must resolve without a probe"
      );
    }
  }

  #[test]
  fn a_bare_number_naming_one_thing_resolves_to_that_thing() {
    let only_the_issue = |a: &Address| matches!(a.entity, Entity::Issue { .. });
    let Ok(Resolution::Resolved(address)) = resolve("0059", only_the_issue) else {
      panic!("one hit is a resolution");
    };
    assert_eq!(address.entity, Entity::Issue { id: "0059".into() });

    let only_the_thread = |a: &Address| matches!(a.entity, Entity::Thread { .. });
    let Ok(Resolution::Resolved(address)) = resolve("0059", only_the_thread) else {
      panic!("one hit is a resolution");
    };
    assert_eq!(
      address.entity,
      Entity::Thread {
        id: "ST0059".into()
      }
    );
  }

  #[test]
  fn a_bare_number_naming_two_things_is_ambiguous_and_never_picks_one() {
    // **THIS IS THE ARM THE WIDTH REMOVAL EXISTS FOR.** Before it, `0059`
    // silently became the ISSUE by its digit count, and 48 of 69 thread
    // numbers on this estate are also issue numbers -- so the wrong answer was
    // the common case rather than a corner.
    let both = |_: &Address| true;
    let Ok(Resolution::Ambiguous(candidates)) = resolve("0059", both) else {
      panic!("two hits is an ambiguity, not a precedence question");
    };
    assert_eq!(
      candidates
        .iter()
        .map(|a| a.entity.form())
        .collect::<Vec<_>>(),
      vec!["thread", "issue"],
      "the ladder's order fixes the report's order"
    );
  }

  #[test]
  fn nothing_found_reports_both_places_it_looked() {
    let Ok(Resolution::Unresolvable { searched }) = resolve("0059", barren) else {
      panic!("no hit is unresolvable");
    };
    assert_eq!(
      searched.iter().map(|a| a.to_url()).collect::<Vec<_>>(),
      vec![
        "intent:///threads/ST0059".to_string(),
        "intent:///issues/0059".to_string()
      ],
      "searched names what was looked for, so a face can say where it looked"
    );
  }

  #[test]
  fn a_grammar_refusal_is_an_error_and_not_an_outcome_of_searching() {
    // `ST5x` names nothing under any spelling. Nothing was searched, so
    // `Unresolvable` would be a false report of a search that never ran.
    assert!(matches!(
      resolve("ST5x", barren),
      Err(AddressError::NotAddressable { .. })
    ));
    assert!(matches!(
      resolve("", barren),
      Err(AddressError::NotAddressable { .. })
    ));
  }

  #[test]
  fn a_full_address_resolves_as_itself() {
    let never = |_: &Address| panic!("an address needs no store to be read");
    let Ok(Resolution::Resolved(address)) = resolve("intent:///issues/0059", never) else {
      panic!("a full address is already resolved");
    };
    assert_eq!(address.entity, Entity::Issue { id: "0059".into() });
  }
}
