//! AT-05.2 / AC-05.2 -- **AND THIS FILE DOES NOT YET TEST AC-05.2's CRITERION.
//! THE ROW STAYS RED. SAYING SO IS THE POINT OF THIS HEADER.**
//!
//! What it holds today is the subset of the old file that survives hv's
//! 2026-08-19 ruling: the properties of `intentfiles::pin` itself, which are
//! still true and still worth pinning down. What it no longer holds is anything
//! about a GENERATED REGION being rewritten from status.
//!
//! # What was deleted and why
//!
//! The old file's headline case was *a hand realisation writes to the PINNED
//! region and survives the next `organize`*, driven through
//! `intentfiles::render` with `Generated` entries standing for "what status
//! offers". **hv replaced that design**: `.intentfiles` is durable state,
//! commands CHANGE it, and NOTHING recomputes it from status. With no
//! regeneration there is no reversion to survive, so the case had no subject --
//! and `render` and `Generated` are deleted from the module entirely.
//!
//! Two tests went with them: `a_hand_pin_survives_an_organize_that_drops_it_
//! from_status` and its control `an_unpinned_thread_in_the_same_run_does_not_
//! survive`. A third, `pinning_an_artefact_already_in_the_generated_region_
//! still_pins_it`, is kept in its surviving half -- the early-return trap it
//! guards is a fact about `pin`, not about the rewrite it used to check through.
//!
//! # What AC-05.2 now requires, and what will replace this file
//!
//! That the LIFECYCLE VERBS edit the list: `st new` adds the entry and
//! `--dehydrate` does not, `st done` and `st cancel` remove it and `--keep` does
//! not, `st reopen` and `st reinstate` add it back -- **and that the closing
//! verbs WARN, naming the paths, when the artefact holds on-disk bytes the store
//! has never seen.** A warning and never a refusal: `organize.rs:695` is the
//! only line in the tool that removes an estate file, and a second authority
//! over a destructive act it does not perform would refuse work the real
//! authority allows.
//!
//! **None of that is built** -- no lifecycle verb touches `.intentfiles` today,
//! and `--dehydrate` and `--keep` are declared, documented, and read by nothing.
//! So this file cannot yet assert it, AT-05.2 is correctly red, and **a green
//! here before those verbs exist would be the failure the whole row is about.**
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
