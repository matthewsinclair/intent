//! **`intentfiles::pin`, THE PRIMITIVE -- NOT AC-05.2's CRITERION.**
//!
//! This file was `edit_writes_pinned_region.rs` and carried AT-05.2 with it.
//! **Both halves of that name were wrong by 2026-08-20**: hv's ruling deleted
//! the two-region design, so there is no "pinned region" to write to as
//! against a generated one, and the criterion moved to
//! `lifecycle_verbs_edit_the_list.rs`, which drives the VERBS. What is left
//! here is what was always true -- the properties of `pin` as a function --
//! and it is a better file for holding only that.
//!
//! Its inverse lives in `unpin_removes_from_the_list.rs`, which also holds the
//! round trip between the two.
//!
//! # The thing that was left to break, broke -- and that was the point
//!
//! `pins_accumulate_in_order_without_disturbing_the_file` pinned an `ISSUE:`.
//! hv ruled on 2026-08-20 that issues are canon-and-store only and `ISSUE:`
//! left the grammar, so `Sigil::Issue` went. **It was left as-is rather than
//! pre-emptively rewritten, on the ground that the compiler naming the line is
//! a better record of the dependency than a comment predicting it.** It named
//! the line: one error, `E0599` at what was line 117, and nothing else in the
//! workspace failed to build.
//!
//! The test now accumulates two STEELTHREAD pins. **Losing the second sigil
//! costs this test nothing, because ORDER and NON-DISTURBANCE were never facts
//! about the sigil** -- and the version that used two different sigils could
//! not distinguish "pins accumulate in order" from "the two sigils happen to
//! sort that way", which the same-sigil version cannot confuse.

use intentsvcs::intentfiles::{Sigil, parse, pin};

/// Only `ST0056` is listed.
const STARTING: &str = "\
# a hand-maintained note
STEELTHREAD:ST0056
";

/// `intent edit` on the same thread twice is ordinary. The manifest must not
/// grow a line for it.
#[test]
fn pinning_is_idempotent() {
  let once = pin(STARTING, Sigil::SteelThread, "ST0011", Some("a reason")).expect("pins");
  assert_ne!(once, STARTING, "precondition: the first pin writes");
  let twice = pin(&once, Sigil::SteelThread, "ST0011", Some("a reason")).expect("pins again");
  assert_eq!(once, twice, "a second edit of the same thread is a no-op");

  // And a different reason does not silently replace the first: the pin is
  // already there, and rewriting somebody's note is not this function's job.
  let relabelled = pin(
    &once,
    Sigil::SteelThread,
    "ST0011",
    Some("a different reason"),
  )
  .expect("pins");
  assert_eq!(
    relabelled, once,
    "an existing pin is left alone -- overwriting its reason would erase a\n       \
     decision to record a decision"
  );
}

/// Pins accumulate at the end of the list in the order they were made, and
/// nothing else in the file moves.
#[test]
fn pins_accumulate_in_order_without_disturbing_the_file() {
  let with_note = "# hand-maintained: see the 2026-08 ruling\nSTEELTHREAD:ST0056\n";
  let a = pin(with_note, Sigil::SteelThread, "ST0011", None).expect("pins");
  let b = pin(&a, Sigil::SteelThread, "ST0042", Some("needed offline")).expect("pins");

  let added = |text: &str| -> Vec<String> {
    parse(text)
      .expect("parses")
      .entries
      .iter()
      .filter(|e| e.id != "ST0056")
      .map(|e| e.id.clone())
      .collect()
  };
  // **ST0042 SORTS AFTER ST0011, SO THIS ASSERTION HAS TO EARN ITS KEEP.**
  // Insertion order and ascending id agree on this pair, which means the
  // assertion alone cannot tell them apart -- so the control is the reverse
  // pair below, where they disagree.
  assert_eq!(
    added(&b),
    vec!["ST0011", "ST0042"],
    "in the order they were made"
  );

  let later_first = pin(with_note, Sigil::SteelThread, "ST0042", None).expect("pins");
  let reversed = pin(&later_first, Sigil::SteelThread, "ST0011", None).expect("pins");
  assert_eq!(
    added(&reversed),
    vec!["ST0042", "ST0011"],
    "pinned in descending order, the file must hold them that way -- if this comes back sorted, \
     `pin` is ordering the list and the assertion above was reading a coincidence"
  );

  assert!(
    b.starts_with(with_note),
    "the file's existing content is not reflowed to make room for a pin"
  );
  assert!(
    b.contains("STEELTHREAD:ST0042  # needed offline"),
    "the reason is written beside the artefact, not on its own line"
  );
}

/// A pin the grammar would refuse is refused AT THE WRITE, not left to be
/// discovered on the next read. Writing it would turn a typo into a manifest
/// nothing can parse -- including the `organize` that would otherwise fix it.
#[test]
fn an_unwritable_id_is_refused_before_it_reaches_the_file() {
  for bad in ["ST56", "intent/st/ST0011", "", "ST0011 "] {
    assert!(
      pin(STARTING, Sigil::SteelThread, bad, None).is_err(),
      "`{bad}` must be refused at the write"
    );
  }
  // And an unreadable manifest is not written to at all.
  assert!(pin("THREAD:ST0056\n", Sigil::SteelThread, "ST0011", None).is_err());
}
