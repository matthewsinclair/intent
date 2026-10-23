//! Issue 0630: a peer's board write that is still landing refuses no one's
//! commit. A write set lands one file at a time, a board's `board.json` before
//! its `wip.md`, and the file index takes in what landed only once the whole
//! set has. So between the renames a reader sees the new `board.json` beside
//! the old `wip.md`, and the file index still records that old `wip.md` as the
//! store's last render: doctor reports it store-stale, uncounted.
//!
//! That holds only while a generated view's row is its renderer's alone. A
//! scan records what it meets, and a second scan of the same bytes marks them
//! `clean`, so the scan carries a view's row forward and never writes one, and
//! a write records every view its set carried, written or already right. A
//! hand edit then matches no row and stays a counted view-skew, however many
//! times the tree is scanned.

use crate::common::Fixture;
use intentsvcs::finding::FindingClass;
use intentsvcs::model::WbItemKind;
use intentsvcs::store::Store;
use intentsvcs::sync::Scope;

const BOARD: &str = "intent/whiteboard/aa/board.json";
const VIEW: &str = "intent/whiteboard/aa/wip.md";
const TODO: &str = "intent/todo.md";

/// Doctor's findings on `path`, judged against the fixture's store.
fn findings_on(fx: &Fixture, path: &str) -> Vec<(FindingClass, String)> {
  let store = Store::open(&fx.project().db_path()).expect("open the store");
  intentsvcs::facade::Facade::doctor(
    &fx.project(),
    &crate::common::facade_ctx(),
    Some(&store),
    intentsvcs::doctor::Scope::All,
  )
  .findings
  .into_iter()
  .filter(|f| f.file == path)
  .map(|f| (f.class, f.detail))
  .collect()
}

/// The file index's rows for `paths`, as the store holds them now.
fn rows(store: &Store, paths: &[&str]) -> Vec<intentsvcs::sync::FileEntry> {
  store
    .file_index()
    .expect("read the file index")
    .into_iter()
    .filter(|row| paths.contains(&row.path.as_str()))
    .collect()
}

/// The file index's rows for `paths`, read through a store opened for it.
fn rows_now(fx: &Fixture, paths: &[&str]) -> Vec<intentsvcs::sync::FileEntry> {
  rows(
    &Store::open(&fx.project().db_path()).expect("open the store"),
    paths,
  )
}

/// Two registered nodes and `aa`'s board written once.
fn one_board_write() -> Fixture {
  let fx = Fixture::new();
  let mut f = fx.facade_on_disk();
  f.wb_register("aa", "Node A", "worker")
    .expect("register aa");
  f.wb_register("bb", "Node B", "worker")
    .expect("register bb");
  f.wb_touch("aa").expect("the first board write");
  fx
}

/// The disk and the file index held as they stand while a second board write
/// is still landing: the second write's model and `board.json`, the first
/// write's `wip.md`, and the index still recording the first write's files.
fn a_second_write_still_landing() -> Fixture {
  let fx = one_board_write();
  let first_view = fx.read(VIEW);
  let mut f = fx.facade_on_disk();
  let first_rows = rows(f.store(), &[BOARD, VIEW]);
  assert_eq!(
    first_rows.len(),
    2,
    "the first write recorded both files: {first_rows:?}"
  );
  f.wb_add("aa", WbItemKind::Todo, "the second write")
    .expect("the second board write");
  assert_ne!(
    fx.read(VIEW),
    first_view,
    "the control: the second write moved the view"
  );
  drop(f);

  std::fs::write(fx.path(VIEW), &first_view).expect("put the first wip.md back");
  let mut store = Store::open(&fx.project().db_path()).expect("open the store");
  store
    .record_file_entries(&first_rows)
    .expect("put the first write's index rows back");
  fx
}

#[test]
fn a_view_a_write_is_still_landing_is_store_stale_and_refuses_no_commit() {
  let fx = a_second_write_still_landing();
  let found = findings_on(&fx, VIEW);
  assert_eq!(found.len(), 1, "{found:?}");
  assert_eq!(found[0].0, FindingClass::StoreStale, "{found:?}");
  assert!(
    !found[0].0.is_actionable(),
    "a peer's write still landing must not refuse a commit: {found:?}"
  );
}

#[test]
fn a_hand_edit_of_the_view_stays_a_counted_skew() {
  let fx = one_board_write();
  let mut edited = fx.read(VIEW);
  edited.push_str("\nA LINE SOMEBODY TYPED\n");
  std::fs::write(fx.path(VIEW), &edited).expect("edit the view by hand");
  let found = findings_on(&fx, VIEW);
  assert_eq!(found.len(), 1, "{found:?}");
  assert_eq!(found[0].0, FindingClass::ViewSkew, "{found:?}");
  assert!(found[0].0.is_actionable(), "{found:?}");
}

/// **A ROW A SCAN WROTE BEFORE THE SCAN STOPPED WRITING VIEWS' ROWS, AND LEFT
/// `changed`, IS NOT THE STORE'S RENDER.** Only a `clean` row excuses a
/// difference, so a hand edit such a scan recorded stays counted.
#[test]
fn a_hand_edit_a_scan_has_recorded_stays_a_counted_skew() {
  let fx = one_board_write();
  let mut edited = fx.read(VIEW);
  edited.push_str("\nA LINE SOMEBODY TYPED\n");
  std::fs::write(fx.path(VIEW), &edited).expect("edit the view by hand");
  let mut store = Store::open(&fx.project().db_path()).expect("open the store");
  let mut scanned = rows(&store, &[VIEW]);
  assert_eq!(scanned.len(), 1, "the view has an index row: {scanned:?}");
  scanned[0].sha256 = intentsvcs::model::sha256_hex(edited.as_bytes());
  scanned[0].state = intentsvcs::sync::FileState::Changed;
  store
    .record_file_entries(&scanned)
    .expect("record what a scan would");
  drop(store);
  let found = findings_on(&fx, VIEW);
  assert_eq!(found.len(), 1, "{found:?}");
  assert_eq!(found[0].0, FindingClass::ViewSkew, "{found:?}");
}

/// **A SCAN NEVER WRITES A VIEW'S ROW.** A store holding no thread, issue or
/// board is cold at every open, so every open of a fresh project scans the
/// tree. Driven before the fix: two `doctor` runs took a hand edit of
/// `todo.md` to `clean`, and a check that excuses the store's own render then
/// excused the edit.
#[test]
fn two_opens_of_a_fresh_project_leave_a_hand_edited_view_counted() {
  let fx = Fixture::new();
  let mut f = fx.facade_on_disk();
  f.sync_to_disk(&Scope::All).expect("render every view");
  drop(f);
  let render = rows_now(&fx, &[TODO]);
  assert_eq!(render.len(), 1, "the render recorded todo.md: {render:?}");

  let edited = "edited by hand\n";
  std::fs::write(fx.path(TODO), edited).expect("edit the view by hand");
  drop(fx.facade_on_disk());
  drop(fx.facade_on_disk());

  let after = rows_now(&fx, &[TODO]);
  assert_eq!(after, render, "the scans left the renderer's row as it was");
  let found = findings_on(&fx, TODO);
  assert_eq!(found.len(), 1, "{found:?}");
  assert_eq!(found[0].0, FindingClass::ViewSkew, "{found:?}");
}

/// **A WRITE RECORDS A VIEW IT FOUND ALREADY RIGHT.** A write set skips a path
/// whose bytes already match, so a view `init` wrote, or one no write has had
/// to change since, had no row of its renderer's once the scan stopped writing
/// them. The row is taken away here to stand in for one.
#[test]
fn a_projection_that_finds_a_view_already_right_records_it() {
  let fx = one_board_write();
  let mut f = fx.facade_on_disk();
  f.sync_to_disk(&Scope::All).expect("render every view");
  drop(f);
  let conn = rusqlite::Connection::open(fx.project().db_path()).expect("the store");
  conn
    .execute("DELETE FROM file_index WHERE path = ?1", [TODO])
    .expect("forget todo.md's row");
  drop(conn);
  assert!(rows_now(&fx, &[TODO]).is_empty(), "control: no row");

  let mut f = fx.facade_on_disk();
  f.sync_to_disk(&Scope::All)
    .expect("render every view again");
  drop(f);

  let after = rows_now(&fx, &[TODO]);
  assert_eq!(after.len(), 1, "the render recorded todo.md: {after:?}");
  assert_eq!(
    (after[0].sha256.clone(), after[0].state),
    (
      intentsvcs::model::sha256_hex(fx.read(TODO).as_bytes()),
      intentsvcs::sync::FileState::Clean,
    ),
  );
}
