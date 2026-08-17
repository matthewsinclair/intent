//! **A DOCUMENT IS A SECTION LIST PLUS AN ORDER**, and the legacy parser was
//! conserving only the first half.
//!
//! D28's shape is two fields -- `objective` and `body` -- so that a section the
//! v2 template never named survives the migration instead of being dropped by a
//! model with a fixed set of headings. That much worked. What did not is that
//! `legacy::sections` returned a `BTreeMap`, and the consumer rebuilds `body` by
//! joining it, so **the reassembled document came back in ALPHABETICAL order**.
//! Measured across this repository's own estate: 140 of 140 work packages
//! differ from what their author wrote.
//!
//! **The reason it survived is the reason this file exists rather than another
//! assertion in an existing one.** Every section was present, so every
//! per-section check passed -- and the census that found it hashes each section
//! body independently, which is exactly the shape that cannot see a reordering.
//! Only a comparison at the level of the WHOLE DOCUMENT can, because the defect
//! is not in any section, it is in the sequence. The same fixture that proves a
//! section survives will pass under both implementations; nothing short of
//! asserting the order can separate them.
//!
//! **And the comment on the code was true.** It said a section the template
//! never named survives, which it does. It was silent about the order, and a
//! reader auditing the function against its own documentation got agreement --
//! including one who was about to copy the line into the thread parser, where it
//! would have closed a genuine 178-section hole while propagating this defect,
//! and made the section counts reconcile on the way.

mod common;

use common::Fixture;
use intentsvcs::legacy;

/// The authored heading order, chosen so that **sorted and authored cannot
/// coincide**: it is very nearly reverse-alphabetical.
///
/// A fixture whose authored order happens to be alphabetical passes under the
/// defect and under the fix, which makes it not a weak test but an unbuilt one.
/// `sorted_differs_from_authored` below asserts that this list actually
/// discriminates, so the day someone renames a heading here and quietly makes
/// the two orders agree, that is a failure rather than a silent downgrade.
const AUTHORED: &[&str] = &["Objective", "Zebra", "Middle", "Deliverables", "Acceptance"];

/// Everything after `Objective` -- what `body` is required to carry, in order.
fn authored_body_order() -> Vec<&'static str> {
  AUTHORED
    .iter()
    .copied()
    .filter(|h| *h != "Objective")
    .collect()
}

fn v2_estate(fixture: &Fixture) {
  fixture.write_file(
    "intent/.config/config.json",
    "{\"intent_version\":\"2.19.0\",\"project_name\":\"P\",\"author\":\"cc\",\"intent_dir\":\"intent\",\"languages\":[\"rust\"]}\n",
  );
  fixture.write_file(
    "intent/st/ST0001/info.md",
    "---\nverblock: \"17 Aug 2026:v0.1: cc - x\"\nintent_version: 2.19.0\nstatus: Completed\nslug: a-slug\ncreated: 20260817\ncompleted: 20260817\n---\n\n# ST0001: A thread\n\n## Objective\n\nShip it.\n",
  );
  let mut wp = String::from(
    "---\ntitle: A work package\nscope: Small\nstatus: Done\n---\n\n# WP01: A work package\n",
  );
  for heading in AUTHORED {
    wp.push_str(&format!("\n## {heading}\n\nThe body of {heading}.\n"));
  }
  fixture.write_file("intent/st/ST0001/WP/01/info.md", &wp);
}

/// The anti-vacuity half, and it comes first because the test after it is
/// conditional on it: if the authored order and the sorted order agree, the
/// assertion below cannot fail under the defect it was written for.
#[test]
fn sorted_differs_from_authored() {
  let authored = authored_body_order();
  let mut sorted = authored.clone();
  sorted.sort_unstable();
  assert_ne!(
    authored, sorted,
    "the fixture's headings are in alphabetical order, so it cannot tell a parser that \
     preserves authored order from one that sorts -- pick headings that differ"
  );
}

/// **The sections come back in the order they were written.**
#[test]
fn a_work_packages_sections_come_back_in_the_order_they_were_written() {
  let fixture = Fixture::new();
  v2_estate(&fixture);
  let scan = legacy::scan(&fixture.project()).expect("scan the v2 estate");

  let wp = &scan.threads[0].wps[0];
  assert_eq!(
    wp.objective, "The body of Objective.",
    "the objective is still lifted out of the document"
  );

  // The order the headings actually appear in the reassembled body, read by
  // position rather than by presence -- presence is what already passed.
  let found: Vec<&str> = wp
    .body
    .lines()
    .filter_map(|l| l.strip_prefix("## "))
    .collect();

  assert_eq!(
    found,
    authored_body_order(),
    "the reassembled document is not the authored one. Every section is present and the \
     SEQUENCE is wrong, which is why a per-section comparison reports this estate clean"
  );
}

/// **A repeated heading is two sections, not one.**
///
/// The map this replaced used `insert`, so a second `## Notes` overwrote the
/// first and the loss left no trace -- a separate conservation hole in the same
/// helper, found only because switching to a `Vec` for the ordering made it
/// impossible to reproduce.
#[test]
fn a_repeated_heading_is_not_swallowed_by_the_first_one() {
  let fixture = Fixture::new();
  fixture.write_file(
    "intent/.config/config.json",
    "{\"intent_version\":\"2.19.0\",\"project_name\":\"P\",\"author\":\"cc\",\"intent_dir\":\"intent\",\"languages\":[\"rust\"]}\n",
  );
  fixture.write_file(
    "intent/st/ST0001/info.md",
    "---\nstatus: Completed\ncreated: 20260817\ncompleted: 20260817\n---\n\n# ST0001: A thread\n\n## Objective\n\nShip it.\n",
  );
  fixture.write_file(
    "intent/st/ST0001/WP/01/info.md",
    "---\ntitle: A work package\nscope: Small\nstatus: Done\n---\n\n# WP01: A work package\n\n## Objective\n\nDo it.\n\n## Notes\n\nThe first note.\n\n## Notes\n\nThe second note.\n",
  );
  let scan = legacy::scan(&fixture.project()).expect("scan the v2 estate");

  let body = &scan.threads[0].wps[0].body;
  assert!(
    body.contains("The first note.") && body.contains("The second note."),
    "both bodies under a repeated heading survive; the map this replaced kept only the \
     last and said nothing: {body}"
  );
}
