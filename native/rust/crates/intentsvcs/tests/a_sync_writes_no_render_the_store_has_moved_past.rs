//! Issue `0441`: intentd's background sync wrote a render it took before a
//! command-line write over that write's canon file and view, and recorded the
//! bytes as the store's own. Nothing said so until the next write to the
//! subject warned over them.
//!
//! **ONE ARM FOR EVERY SUBJECT THE SYNC WRITES.** The guard is on the pass's
//! commit rather than on a kind of file, so an issue stands for all of them.
//! The seam is the pass's own two steps: the edit lands after `ingest_render`
//! has taken its snapshot and before `ingest_commit` writes, which is the
//! window the live estate hit, reached without a clock.

use crate::common::Fixture;
use intentsvcs::facade::Note;
use intentsvcs::sync::Scope;

#[test]
fn an_edit_inside_a_sync_keeps_its_files_and_the_close_after_it_overwrites_nothing() {
  let fx = Fixture::new();
  let mut writer = fx.facade_on_disk();
  let number = writer
    .issue_add(
      "a sync in flight",
      None,
      Some("tester"),
      "the body before the edit",
    )
    .expect("the issue is filed");

  let mut daemon = fx.facade_on_disk();
  let render = daemon
    .ingest_render(&Scope::All)
    .expect("the sync takes its snapshot and renders it");
  writer
    .issue_edit(number, Some("the body the edit wrote"), None, None)
    .expect("the edit lands while the sync holds its render");
  daemon
    .ingest_commit(&Scope::All, render)
    .expect("the sync's commit runs");

  let project = fx.project();
  let stored = fx
    .facade_on_disk()
    .issue_show(number)
    .expect("the store holds the issue")
    .clone();
  assert_eq!(
    std::fs::read_to_string(project.issue_json(number)).expect("the canon file is on disk"),
    intentsvcs::model::to_canonical_json(&stored).expect("the stored issue renders"),
    "the canon file on disk is the store's render, not the one the sync took before the edit"
  );
  let view = std::fs::read_to_string(project.issue_view(number)).expect("the view is on disk");
  assert!(
    view.contains("the body the edit wrote"),
    "the view carries the edit, not the sync's earlier render:\n{view}"
  );

  let closed = fx
    .facade_on_disk()
    .issue_close(number)
    .expect("the issue closes");
  let overwrote: Vec<&Note> = closed
    .notes()
    .iter()
    .filter(|note| matches!(note, Note::OverwroteForeignBytes(_)))
    .collect();
  assert!(
    overwrote.is_empty(),
    "the close found every file as the store last rendered it: {overwrote:?}"
  );
}
