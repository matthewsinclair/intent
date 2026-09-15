//! **`intentfiles::unpin`, THE PRIMITIVE THE CLOSING VERBS NEED** (AC-05.2).
//!
//! `pin` existed and its inverse did not, which is why AC-05.2 could not be
//! built: hv's criterion says `st done` and `st cancel` REMOVE the entry, and
//! nothing in the module removed one. vc ruled the primitive in scope on the
//! ground that it is a consequence of the criterion rather than an addition to
//! it -- if the closing verbs remove an entry then something removes one.
//!
//! # Asked the consumer's question, never counted
//!
//! [`intentfiles::realised`] answers from every entry in the file, so the
//! question a closing verb depends on is "is this thread still realised".
//! Every case below asks that through `realised_from` rather than counting
//! lines, because a line count can pass while that answer is wrong.
//!
//! # What this file does NOT cover
//!
//! The lifecycle wiring. `st new` / `st done` / `st cancel` / `st reopen` /
//! `st reinstate` calling this, and `--dehydrate` / `--keep` suppressing it,
//! are AC-05.2's actual criterion and live in `lifecycle_verbs_edit_the_list.rs`.
//! **A green here is a green about a function, not about the row.**

use intentsvcs::intentfiles::{Sigil, pin, realised_from, unpin};

/// Three entries, plus content that must survive.
const LISTED: &str = "\
# a hand-maintained note
STEELTHREAD:ST0011  # listed, with a reason
STEELTHREAD:ST0056
STEELTHREAD:ST0057
";

/// **Removing an entry leaves its neighbours listed.**
#[test]
fn removing_an_entry_leaves_its_neighbours() {
  let after = unpin(LISTED, Sigil::SteelThread, "ST0056").expect("unpins");
  assert!(
    !realised_from(&after).declares("ST0056"),
    "ST0056 is still realised after being unpinned, so `st done` would report success over an \
     artefact `organize` goes on writing:\n{after}"
  );
  assert!(
    realised_from(&after).declares("ST0057"),
    "the entry after it must be untouched"
  );
  assert!(
    realised_from(&after).declares("ST0011"),
    "as must the entry before it"
  );
}

/// Removing the first entry keeps the note above it.
#[test]
fn unpinning_the_first_entry_keeps_the_note_above_it() {
  let after = unpin(LISTED, Sigil::SteelThread, "ST0011").expect("unpins");
  assert!(!realised_from(&after).declares("ST0011"));
  assert!(
    after.contains("# a hand-maintained note"),
    "a comment that is not an entry survives:\n{after}"
  );
}

/// **A CLOSING VERB MUST BE RE-RUNNABLE.** `st done` on an already-closed
/// thread, a re-run after a partial failure, and a thread created with
/// `--dehydrate` and then closed all arrive with nothing to remove.
#[test]
fn unpinning_something_absent_is_a_no_op_and_not_an_error() {
  let after = unpin(LISTED, Sigil::SteelThread, "ST0099").expect("absent is not an error");
  assert_eq!(
    after, LISTED,
    "the file is returned byte for byte, not reflowed"
  );

  let once = unpin(LISTED, Sigil::SteelThread, "ST0011").expect("unpins");
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
      unpin(LISTED, Sigil::SteelThread, bad).is_err(),
      "`{bad}` must be refused, not answered with an unchanged file"
    );
  }
  // And an unreadable manifest is refused before anything is removed, so a
  // typo in one line cannot cost the caller a different line.
  assert!(unpin("THREAD:ST0056\n", Sigil::SteelThread, "ST0011").is_err());
}

/// **THE ROUND TRIP, AND IT IS NOT A TAUTOLOGY.** `pin` appends; `unpin`
/// removes wherever it finds. Pinning then unpinning must return the file it
/// started from, or one of the two is moving something it should not.
#[test]
fn pin_then_unpin_restores_the_original() {
  let pinned = pin(LISTED, Sigil::SteelThread, "ST0042", Some("a reason")).expect("pins");
  assert_ne!(pinned, LISTED, "the pin must actually have written");

  let restored = unpin(&pinned, Sigil::SteelThread, "ST0042").expect("unpins");
  assert_eq!(
    restored, LISTED,
    "pin followed by unpin is not the identity, so one of them disturbs the file"
  );
}
