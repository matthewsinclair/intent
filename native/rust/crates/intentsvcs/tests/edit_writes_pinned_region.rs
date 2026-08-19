//! AT-05.2 / AC-05.2: **a hand realisation writes to the PINNED region and
//! survives the next `organize`.**
//!
//! Writing it to the generated region means the next run reverts it, because
//! that region is a function of status and the artefact opened by hand is
//! typically closed.
//!
//! # What this row tests that AT-02.3 does not
//!
//! AT-02.3 proves an EXISTING pin survives a status change. It says nothing
//! about where a NEW one goes, and the two are separate failures: a `pin` that
//! wrote into the generated region would pass every AT-02.3 case, because that
//! row starts from a manifest whose pin is already in the right place. **The
//! defect this row exists for is a correct mechanism fed from the wrong end.**
//!
//! So every case here does the writing itself and then runs the rewrite, and
//! the assertion is never "the pin is present" alone -- presence one line
//! after writing it proves only that a string was appended somewhere. It is
//! presence AFTER an `organize` that does not offer the artefact.
//!
//! # The discriminating case
//!
//! A thread that is ALREADY in the generated region when it is pinned by hand.
//! That is the realistic shape -- you open something the estate is currently
//! realising, it later closes, and the question is whether your decision
//! outlives status. A `pin` that saw the id already present and returned early
//! would pass a naive presence check and lose the artefact one `organize`
//! later, which is AC-02.3's harm arriving through the edit path.

use intentsvcs::intentfiles::{Generated, Region, Sigil, parse, pin, render};

const STARTING: &str = "\
# BEGIN INTENT
STEELTHREAD:ST0056
STEELTHREAD:ST0011
# END INTENT
";

/// `organize` running with `ST0011` no longer in status -- it has closed.
fn organize_without_st0011(text: &str) -> String {
  render(text, &[Generated::new(Sigil::SteelThread, "ST0056")]).expect("renders")
}

#[test]
fn a_hand_pin_survives_an_organize_that_drops_it_from_status() {
  let pinned = pin(
    STARTING,
    Sigil::SteelThread,
    "ST0011",
    Some("opened by hand: still cited by the completed-NULL work"),
  )
  .expect("pinning a valid id succeeds");

  // Before the rewrite it must be in the PINNED region, not merely present.
  let m = parse(&pinned).expect("parses");
  let p = m.pinned().find(|e| e.id == "ST0011").expect(
    "the hand pin must land OUTSIDE the markers -- inside, the next\n       \
             organize reverts it, which is the whole of AC-05.2",
  );
  assert_eq!(p.region, Region::Pinned);
  assert_eq!(
    p.comment.as_deref(),
    Some("opened by hand: still cited by the completed-NULL work"),
    "the reason travels with the pin or the next reader deletes it as unexplained"
  );

  // And now the actual property: it outlives status.
  let after = organize_without_st0011(&pinned);
  let m = parse(&after).expect("parses");
  let survivors: Vec<Region> = m
    .entries
    .iter()
    .filter(|e| e.id == "ST0011")
    .map(|e| e.region)
    .collect();
  assert_eq!(
    survivors,
    vec![Region::Pinned],
    "exactly one ST0011 remains and it is the pin -- the generated copy is gone\n       \
     because status no longer offers it, and that is the contrast being tested"
  );
}

/// **The control.** Without it, the case above passes against a `render` that
/// never removes anything at all.
#[test]
fn an_unpinned_thread_in_the_same_run_does_not_survive() {
  let pinned = pin(STARTING, Sigil::SteelThread, "ST0011", None).expect("pins");
  let after = render(&pinned, &[]).expect("organize with nothing in status");
  let m = parse(&after).expect("parses");

  assert!(
    m.entries.iter().any(|e| e.id == "ST0011"),
    "the pinned one stays"
  );
  assert!(
    !m.entries.iter().any(|e| e.id == "ST0056"),
    "the unpinned one goes -- if both survive, the pin distinguishes nothing"
  );
}

/// Pinning something the generated region already carries. The early-return
/// trap: `ST0011` is present, so a `pin` that checked PRESENCE rather than
/// PINNED-NESS would do nothing and lose it at the next rewrite.
#[test]
fn pinning_an_artefact_already_in_the_generated_region_still_pins_it() {
  let before = parse(STARTING).expect("parses");
  assert_eq!(
    before.pinned().count(),
    0,
    "the fixture starts with ST0011 in the GENERATED region only"
  );
  assert!(before.generated().any(|e| e.id == "ST0011"));

  let pinned = pin(STARTING, Sigil::SteelThread, "ST0011", None).expect("pins");
  let after = organize_without_st0011(&pinned);
  assert!(
    parse(&after)
      .expect("parses")
      .pinned()
      .any(|e| e.id == "ST0011"),
    "a pin must be written even when the id is already visible in the region --\n       \
     checking presence instead of region is how the decision is silently dropped"
  );
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
  let b = pin(&a, Sigil::Issue, "0042", Some("needed offline")).expect("pins");

  let m = parse(&b).expect("parses");
  let order: Vec<&str> = m.pinned().map(|e| e.id.as_str()).collect();
  assert_eq!(order, vec!["ST0011", "0042"], "in the order they were made");

  assert!(
    b.starts_with("# hand-maintained: see the 2026-08 ruling\n"),
    "the file's existing content is not reflowed to make room for a pin"
  );
  assert!(
    b.contains("ISSUE:0042  # needed offline"),
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
