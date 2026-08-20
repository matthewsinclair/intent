//! **`intentfiles::unpin`, THE PRIMITIVE THE CLOSING VERBS NEED** (AC-05.2).
//!
//! `pin` existed and its inverse did not, which is why AC-05.2 could not be
//! built: hv's criterion says `st done` and `st cancel` REMOVE the entry, and
//! nothing in the module removed one. vc ruled the primitive in scope on the
//! ground that it is a consequence of the criterion rather than an addition to
//! it -- if the closing verbs remove an entry then something removes one.
//!
//! # The one property that is not obvious, and it is asserted first
//!
//! **`unpin` clears BOTH regions and `pin` writes only to one.** That
//! asymmetry looks like an inconsistency and is the correctness condition:
//! [`intentfiles::realised`] answers from every entry in the file regardless of
//! region, so a pinned-region-only removal would leave `st done` printing
//! success over an artefact that is still realised and whose files `organize`
//! goes on writing. **Where a line goes is a decision; whether a line is there
//! at all is a fact.**
//!
//! # What this file does NOT cover
//!
//! The lifecycle wiring. `st new` / `st done` / `st cancel` / `st reopen` /
//! `st reinstate` calling this, and `--dehydrate` / `--keep` suppressing it,
//! are AC-05.2's actual criterion and live in `edit_writes_pinned_region.rs`.
//! **A green here is a green about a function, not about the row.**

use intentsvcs::intentfiles::{Region, Sigil, parse, pin, realised_from, unpin};

/// A manifest with an entry in EACH region, plus content that must survive.
const BOTH_REGIONS: &str = "\
# a hand-maintained note
STEELTHREAD:ST0011  # pinned, with a reason
# BEGIN INTENT
STEELTHREAD:ST0056
STEELTHREAD:ST0057
# END INTENT
";

/// **THE CORRECTNESS CONDITION: a generated-region entry is removed too.**
///
/// Driven through `realised_from` rather than by counting lines, because the
/// consumer's question is "is this thread still realised", and a test that
/// asserts on line count can pass while the answer to that question is wrong.
#[test]
fn unpinning_clears_the_generated_region_not_only_the_pinned_one() {
  let before = parse(BOTH_REGIONS).expect("parses");
  assert_eq!(
    before
      .generated()
      .find(|e| e.id == "ST0056")
      .map(|e| e.region),
    Some(Region::Generated),
    "the fixture must start with ST0056 in the GENERATED region -- if it starts pinned, this \
     test proves nothing about the region that was the bug"
  );

  let after = unpin(BOTH_REGIONS, Sigil::SteelThread, "ST0056").expect("unpins");
  assert!(
    !realised_from(&after).declares("ST0056"),
    "ST0056 is still realised after being unpinned, so `st done` would report success over an \
     artefact `organize` goes on writing:\n{after}"
  );
  assert!(
    realised_from(&after).declares("ST0057"),
    "and its neighbour in the same region must be untouched"
  );
  assert!(
    realised_from(&after).declares("ST0011"),
    "as must the pinned entry"
  );
}

/// The pinned region is cleared too -- the case `pin` writes into.
#[test]
fn unpinning_clears_the_pinned_region() {
  let after = unpin(BOTH_REGIONS, Sigil::SteelThread, "ST0011").expect("unpins");
  assert!(!realised_from(&after).declares("ST0011"));
  assert!(
    after.contains("# a hand-maintained note"),
    "a comment that is not an entry survives:\n{after}"
  );
  assert!(
    after.contains("# BEGIN INTENT") && after.contains("# END INTENT"),
    "and so do the markers -- removing the last pinned entry must not collapse the region:\n{after}"
  );
}

/// **A CLOSING VERB MUST BE RE-RUNNABLE.** `st done` on an already-closed
/// thread, a re-run after a partial failure, and a thread created with
/// `--dehydrate` and then closed all arrive with nothing to remove.
#[test]
fn unpinning_something_absent_is_a_no_op_and_not_an_error() {
  let after = unpin(BOTH_REGIONS, Sigil::SteelThread, "ST0099").expect("absent is not an error");
  assert_eq!(
    after, BOTH_REGIONS,
    "the file is returned byte for byte, not reflowed"
  );

  let once = unpin(BOTH_REGIONS, Sigil::SteelThread, "ST0011").expect("unpins");
  let twice = unpin(&once, Sigil::SteelThread, "ST0011").expect("unpins again");
  assert_eq!(
    once, twice,
    "and a second removal of the same id changes nothing"
  );
}

/// **A MALFORMED ID IS REFUSED THOUGH IT COULD NOT HAVE MATCHED ANYTHING.**
///
/// The no-op answer would be indistinguishable from "that thread was not
/// listed" -- the ordinary outcome above, returned on every second call. One
/// of those two states is a caller bug and the other is expected, so they must
/// not share an answer.
#[test]
fn an_unwritable_id_is_refused_rather_than_silently_matching_nothing() {
  for bad in ["ST56", "intent/st/ST0011", "", "ST0011 "] {
    assert!(
      unpin(BOTH_REGIONS, Sigil::SteelThread, bad).is_err(),
      "`{bad}` must be refused, not answered with an unchanged file"
    );
  }
  // And an unreadable manifest is refused before anything is removed, so a
  // typo in one line cannot cost the caller a different line.
  assert!(unpin("THREAD:ST0056\n", Sigil::SteelThread, "ST0011").is_err());
}

/// **THE ROUND TRIP, AND IT IS NOT A TAUTOLOGY.** `pin` writes above the
/// BEGIN marker; `unpin` removes wherever it finds. Pinning then unpinning
/// must return the file it started from, or one of the two is moving something
/// it should not.
#[test]
fn pin_then_unpin_restores_the_original() {
  let pinned = pin(BOTH_REGIONS, Sigil::SteelThread, "ST0042", Some("a reason")).expect("pins");
  assert_ne!(pinned, BOTH_REGIONS, "the pin must actually have written");

  let restored = unpin(&pinned, Sigil::SteelThread, "ST0042").expect("unpins");
  assert_eq!(
    restored, BOTH_REGIONS,
    "pin followed by unpin is not the identity, so one of them disturbs the file"
  );
}
