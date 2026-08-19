//! AT-02.3 / AC-02.3: **a pin to a thread that later CLOSES still realises
//! that thread after `organize`.**
//!
//! This is the two-region design's whole reason. Without the split: pin
//! `ST0011` because you still need to read it, it closes, `organize`
//! regenerates the block from status, and the pin is gone along with the files
//! -- with nothing in the output naming the decision.
//!
//! **The test is the SEQUENCE, not the state.** A manifest that happens to
//! contain a pinned closed thread proves nothing; what has to hold is that the
//! thread survives the transition, so every case here renders TWICE -- once
//! with the thread in status, once without -- and compares the two.
//!
//! **And the decision must survive with it.** A pin that persists as a bare id
//! is a line the next reader deletes as unexplained, so the comment carrying
//! its reason is asserted alongside the artefact. AC-02.3's stated harm is
//! "nothing in the output names the decision", which a surviving-but-anonymous
//! pin reproduces one cycle later.

use intentsvcs::intentfiles::{Generated, Region, Sigil, parse, render};

/// `ST0011` is pinned WITH its reason. `ST0056` and `ST0057` are open and
/// arrive from status.
const WITH_PIN: &str = "\
STEELTHREAD:ST0011  # pinned: the completed-NULL work still cites it
# BEGIN INTENT
STEELTHREAD:ST0056
STEELTHREAD:ST0057
# END INTENT
";

fn ids(text: &str) -> Vec<String> {
  parse(text)
    .expect("parses")
    .entries
    .iter()
    .map(|e| e.id.clone())
    .collect()
}

/// The transition itself: `ST0011` is in status, then it closes and is not.
#[test]
fn a_pinned_thread_survives_leaving_status() {
  let open = render(
    WITH_PIN,
    &[
      Generated::new(Sigil::SteelThread, "ST0056"),
      Generated::new(Sigil::SteelThread, "ST0057"),
    ],
  )
  .expect("renders while open");

  // ST0011 closes. `organize` runs again and status no longer offers it.
  let closed = render(
    &open,
    &[
      Generated::new(Sigil::SteelThread, "ST0056"),
      Generated::new(Sigil::SteelThread, "ST0057"),
    ],
  )
  .expect("renders after the close");

  assert!(
    ids(&closed).contains(&"ST0011".to_string()),
    "the pin is what makes a closed thread still realise -- it is gone"
  );

  let m = parse(&closed).expect("parses");
  let pin = m
    .pinned()
    .find(|e| e.id == "ST0011")
    .expect("ST0011 must still be a PIN, not have drifted into the region");
  assert_eq!(pin.region, Region::Pinned);
  assert_eq!(
    pin.comment.as_deref(),
    Some("pinned: the completed-NULL work still cites it"),
    "the pin survives AND names its decision -- an anonymous survivor is a line\n       \
     the next reader deletes, which is AC-02.3's harm one cycle later"
  );
}

/// **The discriminating case.** Everything above would also pass if the writer
/// simply never removed anything. What makes the pin meaningful is that an
/// UNPINNED thread in the same position DOES disappear when it leaves status.
#[test]
fn an_unpinned_thread_does_not_survive_leaving_status() {
  let before = render(
    WITH_PIN,
    &[
      Generated::new(Sigil::SteelThread, "ST0056"),
      Generated::new(Sigil::SteelThread, "ST0057"),
    ],
  )
  .expect("renders");
  assert!(ids(&before).contains(&"ST0057".to_string()));

  // ST0057 closes. It was never pinned.
  let after = render(&before, &[Generated::new(Sigil::SteelThread, "ST0056")])
    .expect("renders after the close");

  assert!(
    !ids(&after).contains(&"ST0057".to_string()),
    "an unpinned thread leaving status must leave the manifest, or the pin\n       \
     distinguishes nothing and the two regions are decoration"
  );
  assert!(
    ids(&after).contains(&"ST0011".to_string()),
    "and the pinned one is still here in the same run -- the contrast is the test"
  );
}

/// A pin and a generated line naming the SAME thread. The thread closes; the
/// generated copy goes and the pin stays. Nothing here should dedupe: the two
/// regions answer different questions and a writer that collapsed them would
/// silently drop the pin the moment status happened to agree with it.
#[test]
fn a_pin_shadowed_by_status_outlives_it() {
  let both = "\
STEELTHREAD:ST0011  # pinned regardless of status
# BEGIN INTENT
STEELTHREAD:ST0011
STEELTHREAD:ST0056
# END INTENT
";
  let open = render(
    both,
    &[
      Generated::new(Sigil::SteelThread, "ST0011"),
      Generated::new(Sigil::SteelThread, "ST0056"),
    ],
  )
  .expect("renders");
  let m = parse(&open).expect("parses");
  assert_eq!(
    m.entries.iter().filter(|e| e.id == "ST0011").count(),
    2,
    "the pin and the generated line coexist -- they are different declarations"
  );

  let closed = render(&open, &[Generated::new(Sigil::SteelThread, "ST0056")])
    .expect("renders after the close");
  let m = parse(&closed).expect("parses");
  let survivors: Vec<Region> = m
    .entries
    .iter()
    .filter(|e| e.id == "ST0011")
    .map(|e| e.region)
    .collect();
  assert_eq!(
    survivors,
    vec![Region::Pinned],
    "exactly one ST0011 remains and it is the PIN"
  );
}
