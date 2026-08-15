//! AT-06.7 / AC-06.7: authored work-package prose survives canon -> view ->
//! canon byte-identical, and the view carries the sections the template never
//! named.
//!
//! **D28's two-field shape is what is under test here, not just the renderer.**
//! `work_package` carries `objective` and `body` -- two fields, not a set of
//! named sections -- because real work packages exceed the template freely:
//! ST0056's own WP-13 is a spec running to hundreds of lines under headings no
//! template foresaw. A model with a fixed set of sections silently drops what
//! it did not anticipate, and WP-10's migrator would have done exactly that to
//! every one of them.
//!
//! The view half was the last of AC-06.7's three (vc): the canon half and the
//! search half were verified independently, and there was no `WP/<NN>/info.md`
//! for the round trip to pass through. v2's is the same mixed
//! authored/generated file that D22 split at thread level, and D22 was never
//! applied one level down.
//!
//! This sits at the library level rather than driving the binary because the
//! property is about the RENDERER: `cli_end_to_end.rs` covers the same ground
//! through the real CLI, which is a different question (does the wiring reach
//! it) from this one (does the rendering preserve it).

mod common;

use common::{Fixture, ctx, sample_thread};
use intentsvcs::model::to_canonical_json;
use intentsvcs::views;

/// The section headings `sample_thread`'s WP-03 body carries, none of which
/// the template names.
const AUTHORED: &[&str] = &[
  "## Why the incumbents go",
  "The quokka clause",
  "## The seams",
  "`pipe | inside`",
];

#[test]
fn the_view_carries_authored_sections_verbatim() {
  let thread = sample_thread("ST0001");
  let wp = thread
    .wps
    .iter()
    .find(|w| w.seq == 3)
    .expect("the fixture's markup-bearing work package");
  assert!(
    !wp.body.trim().is_empty(),
    "the fixture must carry a body, or this test proves nothing"
  );

  let view = views::wp_info(&thread, wp, &ctx());
  for fragment in AUTHORED {
    assert!(
      view.contains(fragment),
      "the view drops {fragment:?}, which is the failure D28's two-field shape \
       exists to prevent:\n{view}"
    );
  }
  assert!(
    view.contains("# WP-03: "),
    "and it is a work-package cover: {view}"
  );
}

/// The cover points at the contract rather than restating it.
///
/// Load-bearing, not decorative: acceptance criteria live in the thread's
/// `acceptance.md`, and a second copy on a work-package cover is a second
/// thing to keep current. v2's template says the same, so this is parity and
/// principle agreeing.
#[test]
fn the_cover_points_at_the_contract_and_does_not_restate_it() {
  let thread = sample_thread("ST0001");
  let wp = thread.wps.iter().find(|w| w.seq == 3).expect("wp");
  let view = views::wp_info(&thread, wp, &ctx());

  assert!(view.contains("## Acceptance"), "{view}");
  assert!(
    view.contains("acceptance.md"),
    "it names where the criteria live: {view}"
  );
  assert!(
    !view.contains("AC-03.1"),
    "and it does not copy them: {view}"
  );
  // `## Deliverables` is unmodelled by D28, so it arrives inside `body` or not
  // at all. Rendering an empty one would be the renderer asserting a section
  // the model does not have.
  assert!(
    !view.contains("## Deliverables"),
    "an unmodelled section is not invented from nothing: {view}"
  );
}

/// Canon -> view -> canon: rendering does not perturb the canon it read.
#[test]
fn rendering_leaves_the_canon_byte_identical() {
  let fx = Fixture::new();
  let project = fx.project();
  let thread = sample_thread("ST0001");

  let path = project.thread_json(&thread.id);
  std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
  let before = to_canonical_json(&thread).expect("serialise");
  std::fs::write(&path, &before).expect("write canon");

  let mut store = intentsvcs::store::Store::open_in_memory().expect("store");
  let canon = intentsvcs::ingest::resync(&project, &mut store).expect("resync");
  views::write_all(&project, &canon, &ctx()).expect("write views");

  let after = std::fs::read_to_string(&path).expect("read canon");
  assert_eq!(
    before, after,
    "the round trip must not rewrite a file it merely read"
  );
  assert!(
    project.wp_info_view("ST0001", 3).is_file(),
    "and the view it renders is where the model says it goes"
  );
}

/// Idempotent: the same model renders the same bytes (AC-03.2 applied to the
/// newest view). A view that changed on every regeneration would make the skew
/// check report files nobody had touched.
#[test]
fn the_view_renders_the_same_bytes_twice() {
  let thread = sample_thread("ST0001");
  let wp = thread.wps.iter().find(|w| w.seq == 3).expect("wp");
  assert_eq!(
    views::wp_info(&thread, wp, &ctx()),
    views::wp_info(&thread, wp, &ctx())
  );
}
