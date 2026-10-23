//! **Issue 0614: a board write leaves a board the file index holds no record
//! of, rather than writing the store's board over a file that may be newer.**
//!
//! A store an Intent before 0619's fix warmed recorded no board's bytes. In a
//! linked worktree whose store was warmed so, a rebase brought main's newer
//! board for a peer, the carrier took only the event, and the next board write
//! by any node rendered the store's older board over the newer file -- driven
//! on 3.2.4, and on 3.2.5 over a store 3.2.4 warmed (refs/bank/dc/0614/). The
//! rows are deleted here to stand in for that warm, and aa's file is replaced
//! by the newer one the rebase brought. bb's own write must still land (vc's
//! ruling, 2026-10-23), aa's file must be left byte for byte and named, and
//! the index must not record bytes the store did not write.

use crate::common::Fixture;
use intentsvcs::sync::Scope;

const AA: &str = "intent/whiteboard/aa/board.json";
const BB: &str = "intent/whiteboard/bb/board.json";

/// The paths `notes` names as left, read through the notes' JSON face so the
/// arm compiles against a build that has no such note.
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
fn a_board_write_leaves_a_peers_board_the_index_holds_no_record_of() {
  let fx = Fixture::new();
  let mut f = fx.facade_on_disk();
  f.wb_register("aa", "Node A", "worker")
    .expect("register aa");
  f.wb_register("bb", "Node B", "worker")
    .expect("register bb");
  f.sync_to_disk(&Scope::All).expect("render every view");
  drop(f);

  let store = fx.project().db_path();
  let conn = rusqlite::Connection::open(&store).expect("the store");
  conn
    .execute(
      "DELETE FROM file_index WHERE path LIKE 'intent/whiteboard/%/board.json'",
      [],
    )
    .expect("forget every board's bytes, as an older warm did");
  drop(conn);
  let newer = fx.read(AA).replace(
    "\"name\": \"Node A\"",
    "\"name\": \"Node A renamed on main\"",
  );
  assert_ne!(newer, fx.read(AA), "control: the newer board differs");
  fx.write_file(AA, &newer);
  let bb_before = fx.read(BB);

  let mut f = fx.facade_on_disk();
  f.wb_touch("bb").expect("bb's own write");
  let notes = f.take_notes();
  drop(f);

  let aa_rows: i64 = rusqlite::Connection::open(&store)
    .expect("the store")
    .query_row(
      "SELECT count(*) FROM file_index WHERE path = ?1",
      [AA],
      |r| r.get(0),
    )
    .expect("count aa's record");
  assert_eq!(
    (
      fx.read(AA) == newer,
      named_as_left(&notes),
      fx.read(BB) != bb_before,
      aa_rows,
    ),
    (true, vec![AA.to_string()], true, 0),
    "aa's newer file is left and named, bb's own write lands, and nothing records aa's bytes"
  );
}
