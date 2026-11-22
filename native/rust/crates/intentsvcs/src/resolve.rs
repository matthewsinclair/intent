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
use crate::id::{Found, IdScope, Named};

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
///
/// # A prefix is completed against the store's ids, never probed for
///
/// An address or a spelling that carries the front of a hashed id (ST0095
/// WP-07) is completed against `scope`, the store's ids injected as presence
/// is: one record that the front begins is [`Resolution::Resolved`] with its
/// id whole, several are [`Resolution::Ambiguous`], and none is
/// [`Resolution::Unresolvable`] naming what was looked for. Bare hex no
/// ordinal could be is looked for as both a thread and an issue. A whole id is
/// never completed, so its existence stays [`crate::nav::Unlanded::Absent`]'s.
pub fn resolve(
  input: &str,
  exists: impl Fn(&Address) -> bool,
  scope: &impl IdScope,
) -> Result<Resolution, AddressError> {
  let (thread, issue) = match crate::address::promote(input) {
    Ok(address) => return complete(address, scope).map_err(AddressError::IdRefused),
    Err(AddressError::AmbiguousId { thread, issue, .. }) => (thread, issue),
    Err(AddressError::PrefixOfEither { hex, .. }) => return Ok(either(input, &hex, scope)),
    Err(refusal) => return Err(refusal),
  };

  let searched = rungs(&thread, issue);
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
/// **BOTH IDS ARE SPELLED BY [`crate::id`] AND NEITHER IS FORMATTED HERE.** The
/// operator door hands back the two typed candidates a bare number names, so
/// a resolver formatting its own would be the second declaration of a width
/// this module does not own.
/// The [`Entity`] values are CONSTRUCTED rather than built as URL strings and
/// parsed back, which `address_resolution_single_home` refuses by name.
fn rungs(thread: &crate::id::ThreadId, issue: crate::id::IssueId) -> Vec<Address> {
  [
    Entity::Thread {
      id: thread.stored(),
    },
    Entity::Issue { id: issue.stored() },
  ]
  .into_iter()
  .map(|entity| Address {
    authority: None,
    entity,
    format: None,
  })
  .collect()
}

/// `address` with every id segment whole: each prefix completed against
/// `scope`, the thread before the package under it, so a package's front is
/// only ever completed among its own thread's packages.
///
/// **A SEGMENT THAT IS ALREADY WHOLE IS NOT LOOKED UP.** A thread or issue
/// segment the waist cannot read is refused in its words; a package segment it
/// cannot read is left for the lookup, which answers it as no package.
pub fn complete(address: Address, scope: &impl IdScope) -> Result<Resolution, crate::id::Refusal> {
  use crate::id::IdKind;
  let (authority, format) = (address.authority.clone(), address.format);
  let with = move |entity: Entity| Address {
    authority: authority.clone(),
    entity,
    format,
  };
  let Some(typed) = address.entity.thread() else {
    return Ok(match &address.entity {
      Entity::Issue { id } => {
        let found = crate::id::resolve_issue(id, scope)
          .map_err(|e| crate::id::refusal(id, &e, IdKind::Issue))?;
        answer(
          &address,
          found.map(|issue| with(Entity::Issue { id: issue.stored() })),
        )
      }
      _ => Resolution::Resolved(address),
    });
  };
  let thread = match crate::id::resolve_thread(typed, scope)
    .map_err(|e| crate::id::refusal(typed, &e, IdKind::Thread))?
  {
    Found::One(thread) => thread,
    found => {
      return Ok(answer(
        &address,
        found.map(|thread| with(address.entity.with_thread(&thread))),
      ));
    }
  };
  let entity = address.entity.with_thread(&thread);
  let Entity::Wp { wp, .. } = &entity else {
    return Ok(Resolution::Resolved(with(entity)));
  };
  // A package segment the waist cannot read is left for the lookup, whose
  // ruling is that such a segment names no package (`Facade::wp_of`).
  Ok(match crate::id::resolve_wp(&thread, wp, scope) {
    Ok(found) => answer(
      &address,
      found.map(|seq| {
        with(Entity::Wp {
          thread: thread.stored(),
          wp: seq.stored(),
        })
      }),
    ),
    Err(_) => Resolution::Resolved(with(entity)),
  })
}

/// A completion as a [`Resolution`]: the one address, every candidate, or the
/// address as typed, which is what was looked for.
fn answer(typed: &Address, found: Found<Address>) -> Resolution {
  match found {
    Found::One(address) => Resolution::Resolved(address),
    Found::Many(addresses) => Resolution::Ambiguous(addresses),
    Found::Nothing => Resolution::Unresolvable {
      searched: vec![typed.clone()],
    },
  }
}

/// Bare hex no ordinal could be, completed against threads and issues alike
/// (cc's rule 5): one record is resolved, several of either kind are one
/// ambiguity, and none names both places it was looked for.
fn either(input: &str, hex: &str, scope: &impl IdScope) -> Resolution {
  let address = |entity: Entity| Address {
    authority: None,
    entity,
    format: None,
  };
  let of = |named: Named| match named {
    Named::Thread(id) => address(Entity::Thread { id: id.stored() }),
    Named::Issue(id) => address(Entity::Issue { id: id.stored() }),
  };
  match crate::id::resolve_named(input, scope) {
    Ok(Found::One(named)) => Resolution::Resolved(of(named)),
    Ok(Found::Many(named)) => Resolution::Ambiguous(named.into_iter().map(of).collect()),
    Ok(Found::Nothing) | Err(_) => Resolution::Unresolvable {
      searched: vec![
        address(Entity::Thread {
          id: crate::id::Spelled::<crate::id::ThreadId>::Prefix(hex.to_string()).segment(),
        }),
        address(Entity::Issue {
          id: crate::id::Spelled::<crate::id::IssueId>::Prefix(hex.to_string()).segment(),
        }),
      ],
    },
  }
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
        matches!(
          resolve(spelling, never, &crate::id::NoRecords),
          Ok(Resolution::Resolved(_))
        ),
        "{spelling} is unambiguous and must resolve without a probe"
      );
    }
  }

  #[test]
  fn a_bare_number_naming_one_thing_resolves_to_that_thing() {
    let only_the_issue = |a: &Address| matches!(a.entity, Entity::Issue { .. });
    let Ok(Resolution::Resolved(address)) = resolve("0059", only_the_issue, &crate::id::NoRecords)
    else {
      panic!("one hit is a resolution");
    };
    assert_eq!(address.entity, Entity::Issue { id: "0059".into() });

    let only_the_thread = |a: &Address| matches!(a.entity, Entity::Thread { .. });
    let Ok(Resolution::Resolved(address)) = resolve("0059", only_the_thread, &crate::id::NoRecords)
    else {
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
    let Ok(Resolution::Ambiguous(candidates)) = resolve("0059", both, &crate::id::NoRecords) else {
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
    let Ok(Resolution::Unresolvable { searched }) = resolve("0059", barren, &crate::id::NoRecords)
    else {
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
      resolve("ST5x", barren, &crate::id::NoRecords),
      Err(AddressError::NotAddressable { .. })
    ));
    assert!(matches!(
      resolve("", barren, &crate::id::NoRecords),
      Err(AddressError::NotAddressable { .. })
    ));
  }

  #[test]
  fn a_full_address_resolves_as_itself() {
    let never = |_: &Address| panic!("an address needs no store to be read");
    let Ok(Resolution::Resolved(address)) =
      resolve("intent:///issues/0059", never, &crate::id::NoRecords)
    else {
      panic!("a full address is already resolved");
    };
    assert_eq!(address.entity, Entity::Issue { id: "0059".into() });
  }

  fn bare(entity: Entity) -> Address {
    Address {
      authority: None,
      entity,
      format: None,
    }
  }

  #[test]
  fn a_thread_or_issue_segment_the_waist_cannot_read_is_refused_in_its_words() {
    for entity in [
      Entity::Thread { id: "ST5x".into() },
      Entity::Issue { id: "5x".into() },
    ] {
      let typed = bare(entity.clone());
      assert!(
        complete(typed, &crate::id::NoRecords).is_err(),
        "{entity:?} is not an id, and completing it is refused rather than passed on"
      );
    }
  }

  #[test]
  fn a_package_segment_the_waist_cannot_read_is_left_for_the_lookup() {
    let typed = Entity::Wp {
      thread: "ST0059".into(),
      wp: "5x".into(),
    };
    let Ok(Resolution::Resolved(address)) = complete(bare(typed.clone()), &crate::id::NoRecords)
    else {
      panic!("an unreadable package segment is the lookup's to answer as no package");
    };
    assert_eq!(address.entity, typed);
  }

  #[test]
  fn a_word_that_begins_with_a_marker_is_not_a_short_prefix() {
    // A marker counts only before hex a door takes, so a node's name, and a
    // marked spelling too short for a prefix that reads exactly as one, is no
    // address at the door that does not know the collection.
    for word in ["ic", "Idea", "stab", "cc", "ST3f9"] {
      assert!(
        matches!(
          resolve(word, barren, &crate::id::NoRecords),
          Err(AddressError::NotAddressable { .. })
        ),
        "`{word}` is read as a word, not as a too-short prefix"
      );
    }
  }
}
