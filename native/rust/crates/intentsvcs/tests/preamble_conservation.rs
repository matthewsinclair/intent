//! **The region above the first heading, which was not being carried anywhere.**
//!
//! `legacy.rs`'s section walk buffers a line only once a `## ` has been seen, so
//! every byte before the first heading fell on the floor -- and
//! `conservation_check.sh` had been reporting exactly that as `LOST-PROSE`
//! since its arm was written. **I proposed this field believing the region was
//! carried and merely unclassified. It was not carried at all**, which is why
//! vc specced it before it was written rather than after: the field is worth
//! building under either premise, but its PURPOSE differs between them.
//!
//! **396 regions / 88,648 bytes across nine projects.** On the canary at
//! `42fb5269`: 20 regions, 15 thread-level and 5 work-package, 6135 bytes
//! stripped. ST0010's 485 bytes are a cancelled thread's deprecation
//! blockquote and its supersession pointer -- precisely what the cancellation
//! discipline exists to preserve, dropped with no drop record. **That is what
//! makes it a conservation defect rather than a convenience.**
//!
//! # Why it is not `body`, and why that is the first thing tested
//!
//! `body` renders after `## Objective`. A preamble carried there comes back
//! BELOW headings its author wrote it above -- **bytes preserved, position
//! moved, and a silent MOVE is harder to see than a silent drop** because every
//! byte count still reconciles. So the position is asserted, not just the
//! content.
//!
//! # The strip is a ruling, not a convenience
//!
//! `conservation_check.sh` deliberately refuses to rule that trimming is
//! acceptable -- *"a check that silently adopted the migrator's own
//! normalisation would be certifying it"* -- and reports only WHICH KIND of
//! difference occurred. The contract rules it instead: the field stores the
//! stripped region, the blank lines are markdown layout the renderer re-emits,
//! and the 20 regions land as `NORMALISED-PROSE` rather than `CONSERVED`. A
//! reported, counted, non-finding.

mod common;

use common::{Fixture, ctx};
use intentsvcs::legacy;
use intentsvcs::views;

fn estate(fixture: &Fixture, thread_pre: &str, wp_pre: &str) {
  fixture.write_file(
    "intent/.config/config.json",
    "{\"intent_version\":\"2.19.0\",\"project_name\":\"P\",\"author\":\"cc\",\"intent_dir\":\"intent\",\"languages\":[\"rust\"]}\n",
  );
  fixture.write_file(
    "intent/st/ST0001/info.md",
    &format!(
      "---\nstatus: Completed\ncreated: 20260816\n---\n\n# ST0001: A thread\n{thread_pre}\n## Objective\n\nShip it.\n\n## Context\n\nBecause.\n"
    ),
  );
  fixture.write_file(
    "intent/st/ST0001/WP/01/info.md",
    &format!(
      "---\ntitle: A work package\nscope: S\nstatus: Done\n---\n\n# WP-01: A work package\n{wp_pre}\n## Objective\n\nDo it.\n"
    ),
  );
}

fn scan(fixture: &Fixture) -> legacy::Scan {
  legacy::scan(&fixture.project()).expect("scan the v2 estate")
}

/// The bytes survive, at both levels. 5 of the canary's 20 are work-package
/// ones, so shipping only the thread half would close 75% of the hole.
#[test]
fn prose_above_the_first_heading_is_carried_at_both_levels() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "\n> Superseded by ST0042.\n\n- **Author**: someone\n",
    "\nA numbat note above every heading.\n",
  );
  let scan = scan(&fixture);
  let thread = &scan.threads[0];

  assert_eq!(
    thread.preamble, "> Superseded by ST0042.\n\n- **Author**: someone",
    "the thread's region, verbatim and unclassified"
  );
  assert_eq!(
    thread.wps[0].preamble, "A numbat note above every heading.",
    "and the work package's"
  );
}

/// **THE POSITION, WHICH IS THE WHOLE REASON THIS IS NOT `body`.**
///
/// A test asserting only that the bytes survive passes just as happily against
/// an implementation that files them in `body` -- where they render below two
/// headings their author wrote them above. Every byte count still reconciles,
/// which is what makes the move harder to catch than the drop.
#[test]
fn the_region_renders_above_the_first_generated_heading() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "\n> Superseded by ST0042.\n",
    "\nA numbat note.\n",
  );
  let scan = scan(&fixture);
  let thread = &scan.threads[0];

  let view = views::info(thread, &ctx());
  let pre = view.find("> Superseded by ST0042.").expect("carried");
  let heading = view.find("## Objective").expect("generated");
  assert!(
    pre < heading,
    "the preamble rendered BELOW the first generated heading, so its bytes \
     were kept and its position was not:\n{view}"
  );

  let wp_view = views::wp_info(thread, &thread.wps[0], &ctx());
  let wp_pre = wp_view.find("A numbat note.").expect("carried");
  let wp_heading = wp_view.find("## Objective").expect("generated");
  assert!(wp_pre < wp_heading, "same one level down:\n{wp_view}");
}

/// Stored STRIPPED -- the contract's ruling, and the 78 bytes that separate
/// this project's two published measurements of one corpus.
#[test]
fn the_stored_region_is_stripped() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "\n\n\n   Padded prose.   \n\n\n",
    "\n\n  Also padded.  \n\n",
  );
  let scan = scan(&fixture);

  assert_eq!(scan.threads[0].preamble, "Padded prose.");
  assert_eq!(scan.threads[0].wps[0].preamble, "Also padded.");
}

/// **THE CONTROL.** Without it every test above passes against an
/// implementation that captures the whole document, or the title, or the
/// sections it was never supposed to touch.
#[test]
fn nothing_below_the_first_heading_and_no_title_is_captured() {
  let fixture = Fixture::new();
  estate(&fixture, "\nJust this.\n", "\n");
  let scan = scan(&fixture);
  let thread = &scan.threads[0];

  assert_eq!(thread.preamble, "Just this.");
  assert!(
    !thread.preamble.contains("ST0001") && !thread.preamble.contains("A thread"),
    "the `# ` title line is modelled and must not appear twice: {:?}",
    thread.preamble
  );
  for below in ["Ship it.", "Because.", "## Objective", "## Context"] {
    assert!(
      !thread.preamble.contains(below),
      "{below:?} is below the first heading and belongs to a section: {:?}",
      thread.preamble
    );
  }
  assert!(
    thread.wps[0].preamble.is_empty(),
    "a document with nothing above its first heading has no preamble, not a \
     blank one: {:?}",
    thread.wps[0].preamble
  );
}

/// A round trip through the view: what a reader sees between the title and the
/// first heading is exactly what canon holds.
///
/// **Asserted by slicing the view rather than by calling the parser**, and that
/// is deliberate. Exposing `legacy::preamble` to make this convenient would add
/// a public seam for a reason that expires the moment this test is written --
/// and a seam added for a temporary reason is one nobody removes. Slicing also
/// asks the stronger question: the region has to come back CONTIGUOUS and in
/// the right place, not merely be present somewhere in the file.
#[test]
fn the_rendered_region_sits_between_the_title_and_the_first_heading() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "\n> Superseded by ST0042.\n\n- **Author**: someone\n",
    "\n",
  );
  let thread = scan(&fixture).threads.remove(0);

  let view = views::info(&thread, &ctx());
  let after_title = view
    .split_once("# ST0001: A thread\n")
    .expect("the title line")
    .1;
  let between = after_title
    .split_once("## ")
    .expect("the first generated heading")
    .0;

  // **THE PREMISE, and it is here because the mutation found this test passing
  // over the original defect.** With the region dropped on the floor, canon
  // holds `""` and the view renders nothing between the title and the first
  // heading -- so the equality below is satisfied by two empty strings and the
  // test certifies a round trip of nothing. An equality assertion needs a
  // non-empty subject or it is a tautology about absence.
  assert!(
    !thread.preamble.is_empty(),
    "premise: this fixture must actually carry a preamble"
  );
  assert_eq!(
    between.trim(),
    thread.preamble,
    "the bytes between the title and the first heading are not what canon \
     holds -- so the view and the model disagree about the region:\n{view}"
  );
}
