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
//! Its inverse lives in `unpin_removes_from_the_list.rs`. **The two are
//! deliberately not merged**: `unpin` clears both regions while `pin` writes
//! to one, and a single file asserting both invites a reader to take the
//! asymmetry for an inconsistency instead of the correctness condition it is.
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

use intentsvcs::intentfiles::{Region, Sigil, parse, pin};

/// `ST0011` sits inside the markers; nothing is pinned.
const STARTING: &str = "\
# BEGIN INTENT
STEELTHREAD:ST0056
STEELTHREAD:ST0011
# END INTENT
";

/// The early-return trap, and it is the reason `pin` cannot test PRESENCE.
///
/// `ST0011` is already visible in the file, so a `pin` that returned early on
/// "the id is here" would do nothing -- and the caller's decision would never be
/// recorded. **Presence and pinned-ness disagree on the ordinary path**, not in
/// a corner: `Facade::hydrate` runs its pin step unconditionally for exactly
/// this reason, and its own doc cites this file as having reddened it first.
#[test]
fn pinning_an_artefact_already_present_still_pins_it() {
  let before = parse(STARTING).expect("parses");
  assert_eq!(
    before.pinned().count(),
    0,
    "the fixture starts with ST0011 present but NOT pinned -- if it starts pinned, \
     this test proves nothing"
  );
  assert!(before.generated().any(|e| e.id == "ST0011"));

  let after = pin(STARTING, Sigil::SteelThread, "ST0011", None).expect("pins");
  let m = parse(&after).expect("parses");
  let pinned = m
    .pinned()
    .find(|e| e.id == "ST0011")
    .expect("a pin must be written even when the id is already visible in the file");
  assert_eq!(pinned.region, Region::Pinned);
}

/// `intent edit` on the same thread twice is ordinary. The manifest must not
/// grow a line for it.
#[test]
fn pinning_is_idempotent() {
  let once = pin(STARTING, Sigil::SteelThread, "ST0011", Some("a reason")).expect("pins");
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

/// Pins accumulate above the markers in the order they were made, and nothing
/// else in the file moves.
#[test]
fn pins_accumulate_in_order_without_disturbing_the_file() {
  let with_note =
    "# hand-maintained: see the 2026-08 ruling\n# BEGIN INTENT\nSTEELTHREAD:ST0056\n# END INTENT\n";
  let a = pin(with_note, Sigil::SteelThread, "ST0011", None).expect("pins");
  let b = pin(&a, Sigil::SteelThread, "ST0042", Some("needed offline")).expect("pins");

  let m = parse(&b).expect("parses");
  let order: Vec<&str> = m.pinned().map(|e| e.id.as_str()).collect();
  // **ST0042 SORTS AFTER ST0011, SO THIS ASSERTION HAS TO EARN ITS KEEP.**
  // Insertion order and ascending id agree on this pair, which means the
  // assertion alone cannot tell them apart -- so the control is the reverse
  // pair below, where they disagree.
  assert_eq!(
    order,
    vec!["ST0011", "ST0042"],
    "in the order they were made"
  );

  let later_first = pin(with_note, Sigil::SteelThread, "ST0042", None).expect("pins");
  let reversed = pin(&later_first, Sigil::SteelThread, "ST0011", None).expect("pins");
  let m = parse(&reversed).expect("parses");
  let order: Vec<&str> = m.pinned().map(|e| e.id.as_str()).collect();
  assert_eq!(
    order,
    vec!["ST0042", "ST0011"],
    "pinned in descending order, the file must hold them that way -- if this comes back sorted, \
     `pin` is ordering the region and the assertion above was reading a coincidence"
  );

  assert!(
    b.starts_with("# hand-maintained: see the 2026-08 ruling\n"),
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
