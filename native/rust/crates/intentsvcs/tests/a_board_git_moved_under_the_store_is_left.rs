//! **Issue 0635: a board whose file moved after the store recorded it is left
//! as it is and named by a scoped restore, and another node's board write is
//! refused naming it, rather than either taking the store's older render.**
//!
//! 0614's guard left a board only when the file index held no record of it,
//! and a 3.2.5 warm records every board, so once a store existed the guard
//! never fired. dc drove it on 3.2.5 and on 0630's stack, byte-identically:
//! a store warmed at an older commit, the checkout moved forward under it, and
//! a scoped `intent sync --to-store ST0056` wrote every node's older board
//! over the newer files with nothing on stderr
//! (refs/bank/dc/dc3/stale-store-repro.sh). Here one write leaves the boards
//! and their records, and aa's file then moves forward as a checkout moves it:
//! the store is never told.
//!
//! **THE BOARD WRITE REFUSES; IT DOES NOT LEAVE** (issue 0554 (b)). Every verb
//! that renders every board asks first whether a migrated board on disk is
//! ahead of the store, and a file whose bytes no longer match the index's
//! record is. So bb's own write is refused, naming aa, with `sync --to-store`
//! as the remedy, before anything is written. The restore has no such gate,
//! which is why it wrote the older boards over the newer files.

use crate::common::{Fixture, sample_thread};
use intentsvcs::facade::FacadeError;
use intentsvcs::model::{Board, WbItemKind};
use intentsvcs::sync::Scope;

const AA: &str = "intent/whiteboard/aa/board.json";
const BB: &str = "intent/whiteboard/bb/board.json";
const ID: &str = "ST0001";

/// aa and bb registered, with one item on aa's board, and a thread for a
/// scoped restore to name, on disk and in the store as one write left them.
fn two_boards_as_the_store_left_them(fx: &Fixture) {
  fx.write_thread(&sample_thread(ID));
  let mut f = fx.facade_on_disk();
  f.sync_from_disk(&Scope::All)
    .expect("seed the store from the thread");
  f.wb_register("aa", "Node A", "worker")
    .expect("register aa");
  f.wb_register("bb", "Node B", "worker")
    .expect("register bb");
  f.wb_add("aa", WbItemKind::Todo, "aa's first item")
    .expect("aa's item");
}

/// aa's board as a later commit carries it, with one more item, written over
/// the file as a checkout writes it. Returns the newer text.
fn moved_forward(fx: &Fixture) -> String {
  let mut board: Board = serde_json::from_str(&fx.read(AA)).expect("aa's board");
  let mut item = board.items[0].clone();
  item.seq += 1;
  item.text = "an item the later commit carries".to_string();
  board.items.push(item);
  let newer = intentsvcs::model::to_canonical_json(&board).expect("json");
  assert_ne!(newer, fx.read(AA), "control: the moved board differs");
  fx.write_file(AA, &newer);
  newer
}

/// The file index's whole row for `rel`, its sha and its state, read with no
/// facade open.
fn row(fx: &Fixture, rel: &str) -> Option<String> {
  rusqlite::Connection::open(fx.project().db_path())
    .expect("the store")
    .query_row(
      "SELECT sha256 || ' ' || state FROM file_index WHERE path = ?1",
      [rel],
      |r| r.get(0),
    )
    .ok()
}

/// The nodes a board write was refused for, as not in the store, or `None`
/// when it was not refused that way.
fn refused_as_not_in_the_store(result: Result<(), FacadeError>) -> Option<Vec<String>> {
  match result {
    Err(FacadeError::WbBoardsNotInTheStore { nodes }) => Some(nodes),
    _ => None,
  }
}

/// The paths `notes` names as left, read through the notes' JSON face.
fn named_as_left(notes: &[intentsvcs::facade::Note]) -> Vec<String> {
  intentsvcs::facade::notes_json(notes)
    .as_array()
    .expect("the notes are an array")
    .iter()
    .filter(|note| note["kind"] == "boards-left-unrecorded")
    .flat_map(|note| note["paths"].as_array().cloned().unwrap_or_default())
    .filter_map(|path| path.as_str().map(str::to_string))
    .collect()
}

#[test]
fn a_scoped_restore_leaves_a_board_whose_file_moved_under_the_store() {
  let fx = Fixture::new();
  two_boards_as_the_store_left_them(&fx);
  let newer = moved_forward(&fx);
  let bb_before = fx.read(BB);
  let aa_row = row(&fx, AA);
  assert!(aa_row.is_some(), "control: the store records aa's board");

  let mut f = fx.facade_on_disk();
  f.sync_from_disk(&Scope::Threads(vec![ID.to_string()]))
    .expect("the scoped restore");
  let notes = f.take_notes();
  drop(f);

  assert_eq!(
    (
      fx.read(AA) == newer,
      named_as_left(&notes),
      fx.read(BB) == bb_before,
      row(&fx, AA) == aa_row,
    ),
    (true, vec![AA.to_string()], true, true),
    "aa's newer file is left and named, bb's board is untouched, and the store still records the bytes it wrote for aa"
  );
}

/// **THE SCAN ARM.** A full scan between the checkout and the write meets the
/// moved file, twice, and a second sighting of the same bytes is the one that
/// reads as settled. The record that decides must still be the store's own, so
/// afterwards the restore leaves aa and bb's own write is refused naming it.
#[test]
fn a_full_scan_between_the_checkout_and_the_write_does_not_read_a_moved_board_as_unmoved() {
  let fx = Fixture::new();
  two_boards_as_the_store_left_them(&fx);
  let newer = moved_forward(&fx);
  let aa_row = row(&fx, AA);

  let mut f = fx.facade_on_disk();
  for _ in 0..2 {
    f.ingest_from_disk(&Scope::All)
      .expect("a full scan of the tree");
  }
  let scanned = fx.read(AA) == newer;
  let _ = f.take_notes();
  f.sync_from_disk(&Scope::Threads(vec![ID.to_string()]))
    .expect("the scoped restore");
  let restored = named_as_left(&f.take_notes());
  let after_restore = fx.read(AA) == newer;
  let touched = refused_as_not_in_the_store(f.wb_touch("bb"));
  drop(f);

  assert_eq!(
    (
      scanned,
      after_restore,
      restored,
      fx.read(AA) == newer,
      touched,
      row(&fx, AA) == aa_row,
    ),
    (
      true,
      true,
      vec![AA.to_string()],
      true,
      Some(vec!["aa".to_string()]),
      true
    ),
    "after two full scans, the scoped restore leaves aa's newer file and names it, bb's own write is refused naming aa, and the store's record of aa is the one it wrote"
  );
}

/// **WITH NO SCAN AT ALL** bb's own write is refused naming aa as well: the
/// refusal is the board write's answer to a moved board, not something a scan
/// leaves behind.
#[test]
fn another_nodes_board_write_is_refused_naming_a_board_whose_file_moved() {
  let fx = Fixture::new();
  two_boards_as_the_store_left_them(&fx);
  let newer = moved_forward(&fx);
  let bb_before = fx.read(BB);
  let aa_row = row(&fx, AA);

  let mut f = fx.facade_on_disk();
  let touched = refused_as_not_in_the_store(f.wb_touch("bb"));
  drop(f);

  assert_eq!(
    (
      touched,
      fx.read(AA) == newer,
      fx.read(BB) == bb_before,
      row(&fx, AA) == aa_row,
    ),
    (Some(vec!["aa".to_string()]), true, true, true),
    "bb's own write is refused naming aa before anything is written: aa's newer file, bb's board and the store's record of aa are as they were"
  );
}
