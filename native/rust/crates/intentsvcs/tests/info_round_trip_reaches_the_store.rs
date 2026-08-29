//! **THE WIRING, NOT THE READER.** `info_round_trip.rs` proves
//! `views::info_read_back` classifies edits correctly; this proves an edit made
//! in `info.md` actually reaches the store through `sync --to-store`, and that
//! a refused sync leaves BOTH homes untouched.
//!
//! The two are deliberately separate files. A reader that works and a wiring
//! nobody drove is the shape where every unit arm is green and the feature does
//! not exist -- and the unit arms cannot tell you which of the two you have.

mod common;

use common::{Fixture, ctx, sample_thread};
use intentsvcs::store::Store;
use intentsvcs::sync::Scope;
use intentsvcs::views;

/// An estate with a PERSISTENT store, because the round-trip needs a baseline.
///
/// **A FRESH STORE PER SYNC WAS THE WRONG HARNESS AND IT HID A REAL DEFECT.**
/// `sync::scan` marks a file `Changed` when it is new as well as when it moved,
/// so with an empty index every cover reads as edited -- which is how the first
/// cut of this feature came to refuse a plain `sync --to-store` on a fresh
/// project. Keeping one store across the syncs models what actually happens:
/// the estate has been synced before, and then somebody edits.
struct Estate {
  fx: Fixture,
  store: Store,
}

impl Estate {
  fn new() -> Self {
    Self {
      fx: Fixture::new(),
      store: Store::open_in_memory().expect("store"),
    }
  }

  /// Render the thread's cover to disk exactly as `--to-disk` would, and hand
  /// back the bytes so an arm can edit them.
  fn realise(&self, thread: &intentsvcs::model::Thread) -> String {
    let path = self.fx.project().info_view(&thread.id);
    std::fs::create_dir_all(path.parent().expect("thread dir")).expect("mkdir");
    let rendered = views::info(thread, &ctx());
    std::fs::write(&path, &rendered).expect("realise the cover");
    rendered
  }

  fn cover(&self) -> std::path::PathBuf {
    self.fx.project().info_view("ST0001")
  }

  fn write_cover(&self, text: &str) {
    std::fs::write(self.cover(), text).expect("write the cover");
  }

  fn read_cover(&self) -> String {
    std::fs::read_to_string(self.cover()).expect("read the cover")
  }

  fn sync(&mut self) -> Result<Vec<intentsvcs::model::Thread>, intentsvcs::ingest::IngestError> {
    intentsvcs::ingest::resync(&self.fx.project(), &mut self.store, &Scope::All)?;
    Ok(self.store.load_canon().expect("load back").0)
  }
}

#[test]
fn success_an_edit_to_objective_in_the_file_reaches_the_store() {
  let mut e = Estate::new();
  let thread = sample_thread("ST0001");
  e.fx.write_thread(&thread);
  let rendered = e.realise(&thread);

  // **THE BASELINE SYNC, AND IT IS NOT CEREMONY.** It indexes the cover, which
  // is what makes the NEXT sync able to say a human moved it.
  let before = e.sync().expect("an unedited estate must sync");
  assert_eq!(before[0].objective, thread.objective);

  let edited = rendered.replace(&thread.objective, "Edited by hand, in the file.");
  assert_ne!(edited, rendered, "the edit must land, or the arm is inert");
  e.write_cover(&edited);

  let after = e.sync().expect("an Objective edit is in the allow-list");
  assert_eq!(
    after[0].objective, "Edited by hand, in the file.",
    "hv ruled the cover round-trips; the edit did not reach the store"
  );
  assert_eq!(
    after[0].context, thread.context,
    "an untouched section must not move"
  );
}

/// **REFUSE, AND LEAVE BOTH HOMES ALONE.** hv chose this over carry-and-warn on
/// 2026-08-29. The store assertion is the half that matters: a refusal that
/// happened AFTER `store.rebuild` would still return an error and would already
/// have replaced the store.
///
/// **THE SUBJECT IS THE AUTHORED PREAMBLE, NOT A GENERATED SECTION.** The first
/// cut of this arm planted a byte in the generated `## Acceptance` cover, and
/// that was wrong: a view the model has moved past differs in exactly those
/// regions through nobody's fault, so refusing there refused `sync --to-store`
/// on any estate with a stale view. **hv's ruling is about text the operator
/// WROTE**, and the preamble is where that loss actually happens.
#[test]
fn failure_an_edit_outside_the_allow_list_refuses_and_the_store_is_untouched() {
  let mut e = Estate::new();
  let mut thread = sample_thread("ST0001");
  thread.preamble = "A line the author put above the first heading.".to_string();
  e.fx.write_thread(&thread);
  let rendered = e.realise(&thread);

  e.sync()
    .expect("control + baseline: the unedited estate syncs clean");

  let edited = rendered.replace(&thread.preamble, "A line the author then CHANGED by hand.");
  assert_ne!(edited, rendered, "the edit must land, or the arm is inert");
  e.write_cover(&edited);

  let err = e
    .sync()
    .expect_err("an edit to authored prose that cannot be carried must refuse");
  let said = err.to_string();
  assert!(
    said.contains("preamble"),
    "the refusal must NAME the region so the author can find their text: {said}"
  );
  assert!(
    said.contains("Objective") && said.contains("Context"),
    "the refusal must say which sections DO round trip, or it is a dead end: {said}"
  );

  // **THE FILE IS UNTOUCHED TOO.** A refusal that repaired the view would
  // destroy the operator's text while telling them it had refused.
  assert_eq!(
    e.read_cover(),
    edited,
    "the refusal rewrote the file it refused"
  );
}

/// A byte planted in a GENERATED region is regenerated, not refused -- and the
/// model must come through it untouched. This is vc's binding condition at the
/// wiring level: the store is the thing that must not be corrupted.
#[test]
fn success_a_plant_in_a_generated_region_syncs_clean_and_does_not_reach_the_store() {
  let mut e = Estate::new();
  let thread = sample_thread("ST0001");
  e.fx.write_thread(&thread);
  let rendered = e.realise(&thread);

  e.sync().expect("baseline");

  let planted = rendered.replace(
    "This cover never restates them.",
    "This cover never restates them. PLANTED BY HAND.",
  );
  assert_ne!(
    planted, rendered,
    "the plant must land, or the arm is inert"
  );
  e.write_cover(&planted);

  let threads = e
    .sync()
    .expect("a generated region is regenerated, not refused");
  assert_eq!(
    threads[0].objective, thread.objective,
    "the plant reached the model"
  );
  assert_eq!(
    threads[0].context, thread.context,
    "the plant reached the model"
  );
}

/// A thread whose cover has never been realised carries nothing and must not
/// refuse -- `--to-store` is exactly the verb someone runs on a fresh clone.
#[test]
fn success_a_thread_with_no_realised_cover_syncs_clean() {
  let mut e = Estate::new();
  let thread = sample_thread("ST0001");
  e.fx.write_thread(&thread);

  assert!(
    !e.cover().exists(),
    "the fixture must genuinely have no cover, or this arm proves nothing"
  );

  let threads = e
    .sync()
    .expect("an unrealised cover is absence, not an error");
  assert_eq!(threads[0].objective, thread.objective);
}

/// **A STALE VIEW IS NOT A HAND-EDIT.** The model moves ahead of its views
/// constantly -- every canon change does it -- and the view is only brought
/// back by `--to-disk`. A read-back that refused on that would refuse
/// `sync --to-store` on essentially every estate in the fleet.
///
/// **This is a REGRESSION arm, not a hypothetical.** The first cut of this
/// feature compared render-against-view with no other input and `sync_scope.rs`
/// failed instantly: two threads refused, naming `the frontmatter and preamble`,
/// on an estate where the only change was a retitle in canon.
#[test]
fn invariant_a_view_left_stale_by_a_canon_change_syncs_clean() {
  let mut e = Estate::new();
  let thread = sample_thread("ST0001");
  e.fx.write_thread(&thread);
  let rendered = e.realise(&thread);

  e.sync()
    .expect("control + baseline: the consistent estate syncs clean");

  // The MODEL moves; the view on disk is left exactly as it was.
  let mut moved = thread.clone();
  moved.title = "a title the view has never seen".to_string();
  e.fx.write_thread(&moved);

  assert_eq!(
    e.read_cover(),
    rendered,
    "the view must be UNTOUCHED for this arm to be about staleness at all"
  );
  assert_ne!(
    views::info(&moved, &ctx()),
    rendered,
    "the render must actually have moved, or there is no staleness to test"
  );

  let threads = e
    .sync()
    .expect("a stale view is not a hand-edit and must not refuse");
  assert_eq!(threads[0].title, "a title the view has never seen");
}

/// **STALENESS IN A GENERATED TABLE, WHICH IS THE COMMONEST KIND THERE IS.**
///
/// The arm above moves a thread's TITLE, and the title is stripped by
/// `preamble_of` rather than by the generated-section filter -- so it passes
/// even with that filter disabled. **Two different mechanisms protect against
/// staleness and only one of them was under test**, found by mutating the
/// filter and watching the wrong arm stay green.
///
/// A work package added, renamed, resized or moved through a status all rewrite
/// the `## Work Packages` table. That happens on every estate, constantly, and
/// the view is only brought level by `--to-disk`.
#[test]
fn invariant_a_stale_generated_table_syncs_clean() {
  let mut e = Estate::new();
  let thread = sample_thread("ST0001");
  e.fx.write_thread(&thread);
  let rendered = e.realise(&thread);

  e.sync()
    .expect("control + baseline: the consistent estate syncs clean");

  // The model gains a work package; the view on disk still shows the old table.
  let mut moved = thread.clone();
  let mut extra = moved.wps[0].clone();
  extra.seq = 7;
  extra.title = "A package the rendered table has never seen".to_string();
  moved.wps.push(extra);
  e.fx.write_thread(&moved);

  assert_eq!(
    e.read_cover(),
    rendered,
    "the view must be UNTOUCHED for this arm to be about staleness"
  );
  let fresh = views::info(&moved, &ctx());
  assert!(
    fresh.contains("A package the rendered table has never seen") && !rendered.contains("WP-07"),
    "the generated TABLE must be what moved, or this arm tests the wrong region"
  );

  let threads = e
    .sync()
    .expect("a stale generated table is not a hand-edit");
  assert_eq!(
    threads[0].wps.len(),
    thread.wps.len() + 1,
    "the canon change landed -- counted RELATIVE to the fixture, because a hardcoded \
     number here goes green by drifting the day the fixture gains a package"
  );
  assert_eq!(threads[0].objective, thread.objective);
}

/// **A PROJECT WITH NO BASELINE SYNCS CLEAN, EVEN WHEN ITS COVER PREDATES ITS
/// CANON.** This is the regression `cli_end_to_end` caught, and it is the one
/// that would have shipped: `sync::scan` reports a file as `Changed` when it is
/// NEW as well as when it moved, so on a project whose index has never been
/// written every cover reads as edited. The comparison then ran against a view
/// older than the canon beside it and refused a plain `intent sync --to-store`,
/// naming authored sections the operator had never opened.
///
/// **The subject is an AUTHORED region deliberately.** Filtering the generated
/// frame does not save this case -- authored `body` sections go stale exactly
/// the same way, and editing canon directly is how a criterion is minted.
#[test]
fn invariant_a_first_sync_with_no_index_carries_nothing_and_refuses_nothing() {
  let mut e = Estate::new();
  let thread = sample_thread("ST0001");
  e.fx.write_thread(&thread);
  e.realise(&thread);

  // Canon gains an authored section; the cover on disk still predates it, and
  // NOTHING has ever been indexed.
  let mut moved = thread.clone();
  moved.body = "## Why the incumbents go\n\nAuthored, and newer than the view.\n".to_string();
  e.fx.write_thread(&moved);

  assert!(
    views::info(&moved, &ctx()).contains("Why the incumbents go")
      && !e.read_cover().contains("Why the incumbents go"),
    "the AUTHORED section must be what the view is missing, or this arm tests the wrong region"
  );

  let threads = e
    .sync()
    .expect("a first sync has no baseline, so nothing was edited and nothing may refuse");
  assert_eq!(
    threads[0].body, moved.body,
    "the canon change must land -- the read-back skipped, it did not overwrite"
  );
}
