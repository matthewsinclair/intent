//! Issue 0414: `sync --to-store` printed "nothing the store already held was
//! overwritten" while it renumbered every whiteboard row, restamped
//! `updated_at` on all of them, and changed a node's name and role.
//!
//! vc decision 22: the restore applies the DIFFERENCE by natural key -- a node
//! by moniker, an item by (node, kind, seq), a message by (sender, recipient,
//! recorded_at, body) counted as a multiset, and then, for the messages that
//! leaves unpaired, by (sender, recipient, recorded_at) in order (issue 0525)
//! -- so an unchanged row keeps its id
//! and `updated_at`, and the preview names every board difference beside the
//! threads and issues, which is what makes the no-overwrite line true when it
//! prints.

use crate::common::Fixture;
use intentsvcs::model::{Board, WbItemKind};
use intentsvcs::store::Store;
use intentsvcs::sync::Scope;
use rusqlite::Connection;

type Rows = Vec<(i64, String, String)>;

/// Every item and message row as (id, what it says, updated_at), in id order.
fn rows(db: &Connection) -> (Rows, Rows) {
  let read = |sql: &str| -> Rows {
    let mut stmt = db.prepare(sql).expect(sql);
    stmt
      .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))
      .expect(sql)
      .collect::<Result<_, _>>()
      .expect(sql)
  };
  (
    read("SELECT id, text, updated_at FROM wb_item ORDER BY id"),
    read("SELECT id, body, updated_at FROM wb_message ORDER BY id"),
  )
}

/// Two nodes, two items, and three messages -- two of them identical.
fn boards(fx: &Fixture) -> Vec<Board> {
  let mut f = fx.facade_on_disk();
  f.wb_register("cc", "Control Claude", "control")
    .expect("register cc");
  f.wb_register("vc", "Validation Claude", "validation")
    .expect("register vc");
  f.wb_add("cc", WbItemKind::Todo, "write the door")
    .expect("add");
  f.wb_add("cc", WbItemKind::Todo, "diff by key")
    .expect("add");
  f.wb_ask("vc", "cc", "go", None, false).expect("ask");
  let mut boards = f.boards().expect("boards");
  let twin = boards[0].messages[0].clone();
  boards[0].messages.push(twin.clone());
  let mut other = twin;
  other.body = "and again".to_string();
  boards[0].messages.push(other);
  boards
}

fn later() {
  // Past the last write's millisecond, so a restamped row is seen.
  std::thread::sleep(std::time::Duration::from_millis(5));
}

#[test]
fn a_restore_of_what_the_store_holds_moves_no_row() {
  let fx = Fixture::new();
  let boards = boards(&fx);
  let path = fx.project().db_path();
  let mut store = Store::open(&path).expect("store");
  store.replace_boards(&boards).expect("seed");
  let db = Connection::open(&path).expect("open");
  let before = rows(&db);
  assert_eq!(
    before.1.len(),
    3,
    "precondition: two identical messages stay two"
  );

  later();
  store.replace_boards(&boards).expect("the same boards");
  assert_eq!(
    rows(&db),
    before,
    "an unchanged row keeps its id and its updated_at"
  );
}

#[test]
fn a_restore_updates_the_row_that_differs_and_leaves_the_rest() {
  let fx = Fixture::new();
  let mut boards = boards(&fx);
  let path = fx.project().db_path();
  let mut store = Store::open(&path).expect("store");
  store.replace_boards(&boards).expect("seed");
  let db = Connection::open(&path).expect("open");
  let (items, messages) = rows(&db);

  later();
  boards[0].items[0].text = "write the one door".to_string();
  boards[0].messages.remove(1);
  store
    .replace_boards(&boards)
    .expect("one change, one removal");

  let (items_after, messages_after) = rows(&db);
  assert_eq!(
    items_after[0].0, items[0].0,
    "the changed item keeps its id"
  );
  assert_eq!(items_after[0].1, "write the one door");
  assert!(
    items_after[0].2 > items[0].2,
    "the changed item's updated_at moved: {items_after:?}"
  );
  assert_eq!(items_after[1], items[1], "the untouched item did not move");
  assert_eq!(
    messages_after,
    [messages[0].clone(), messages[2].clone()],
    "one of two identical messages went, and nothing else moved"
  );
}

#[test]
fn an_edited_body_is_updated_in_place_and_keeps_its_place() {
  // Issue 0525, from cc's restore note: a body `wb edit message` changed on
  // another clone arrives as a board whose message says something new. Keyed
  // by its body alone it was removed and re-added, last, which could renumber
  // `#<n>` in its minute. Now the row keeps its id and only its body moves,
  // and the twins beside it stay as they were.
  let fx = Fixture::new();
  let mut boards = boards(&fx);
  let path = fx.project().db_path();
  let mut store = Store::open(&path).expect("store");
  store.replace_boards(&boards).expect("seed");
  let db = Connection::open(&path).expect("open");
  let (_, messages) = rows(&db);

  later();
  boards[0].messages[2].body = "and again, edited".to_string();
  boards[0].messages[2].edited_at = Some("2026-09-23T10:30:00.000Z".to_string());
  store.replace_boards(&boards).expect("one edited body");

  let (_, messages_after) = rows(&db);
  assert_eq!(
    messages_after.iter().map(|m| m.0).collect::<Vec<_>>(),
    messages.iter().map(|m| m.0).collect::<Vec<_>>(),
    "no message changed its id: {messages_after:?}"
  );
  assert_eq!(
    (&messages_after[0], &messages_after[1]),
    (&messages[0], &messages[1]),
    "the twins did not move"
  );
  assert_eq!(messages_after[2].1, "and again, edited");
}

#[test]
fn the_preview_names_a_board_difference_and_is_quiet_without_one() {
  let fx = Fixture::new();
  let mut f = fx.facade_on_disk();
  f.wb_register("cc", "Control Claude", "control")
    .expect("register cc");
  f.wb_add("cc", WbItemKind::Todo, "write the door")
    .expect("add");
  assert_eq!(
    f.sync_overwrite(&Scope::All).expect("preview"),
    Vec::<String>::new(),
    "CONTROL: the board on disk is the board the store holds"
  );

  let file = fx.project().board_json("cc");
  let mut board: Board =
    serde_json::from_str(&std::fs::read_to_string(&file).expect("read")).expect("parse");
  board.items[0].text = "write two doors".to_string();
  board.node.role = "devx".to_string();
  std::fs::write(&file, serde_json::to_string_pretty(&board).expect("json")).expect("write");

  let preview = f.sync_overwrite(&Scope::All).expect("preview");
  assert!(
    preview
      .iter()
      .any(|l| l.contains("board cc") && l.contains("[todo] 1")),
    "the changed item is not named: {preview:?}"
  );
  assert!(
    preview
      .iter()
      .any(|l| l.contains("board cc") && l.contains("node")),
    "the changed node is not named: {preview:?}"
  );
}

/// **A ROW WHOSE NODE IS NOT ON THE ROSTER IS NAMED BEFORE IT IS DELETED.** A
/// hand `DELETE` on `wb_node` -- devbin's repair of a foreign registration --
/// leaves that node's items and messages behind. No board hydrates them, so a
/// diff of boards cannot see them, and a restore that removed them unnamed
/// would print the no-overwrite line over a deletion: 0414 again.
#[test]
fn rows_whose_node_is_gone_are_named_and_then_removed() {
  let fx = Fixture::new();
  let mut f = fx.facade_on_disk();
  f.wb_register("cc", "Control Claude", "control")
    .expect("register cc");
  f.wb_register("vc", "Validation Claude", "validation")
    .expect("register vc");
  f.wb_add("cc", WbItemKind::Todo, "write the door")
    .expect("add");
  f.wb_ask("vc", "cc", "go", None, false).expect("ask");
  let db = Connection::open(fx.project().db_path()).expect("open");
  db.execute("DELETE FROM wb_node WHERE moniker = 'cc'", [])
    .expect("the hand repair");
  std::fs::remove_dir_all(fx.project().whiteboard_dir().join("cc")).expect("and its views");

  let preview = f.sync_overwrite(&Scope::All).expect("preview");
  assert!(
    preview.iter().any(|l| l.contains("board cc")
      && l.contains("1 item(s) and 1 message(s)")
      && l.contains("not on the roster")),
    "the orphaned rows are about to be deleted and the preview did not name them: {preview:?}"
  );

  f.sync_from_disk(&Scope::All).expect("disk -> db");
  let left: i64 = db
    .query_row(
      "SELECT (SELECT count(*) FROM wb_item WHERE node = 'cc') + \
       (SELECT count(*) FROM wb_message WHERE recipient = 'cc')",
      [],
      |r| r.get(0),
    )
    .expect("count");
  assert_eq!(left, 0, "the sync removed the rows it named");
}
