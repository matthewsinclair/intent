//! Issue 0554 (a): **A PULLED BOARD IS TAKEN INTO THE STORE WHEN THE FILE IS
//! THE ONLY SIDE THAT MOVED, AND NEVER WHEN THE STORE MOVED TOO.**
//!
//! A pull brings a teammate's rows in `board.json`, and `sync --apply` never
//! took them, so every board verb refused until `intent sync --to-store`. It
//! now takes a board whose file moved since the store recorded it while the
//! store's own render still hashes to that record. The arm that holds the line
//! is the 0216 shape: a store write the file has not caught up with. Taking the
//! file then would revert that write, so the board is left, as before.

use crate::common::Fixture;
use intentsvcs::model::{Board, WbItemKind};
use intentsvcs::store::Store;

fn git(fx: &Fixture, args: &[&str]) {
  let out = std::process::Command::new("git")
    .args([
      "-c",
      "user.name=t",
      "-c",
      "user.email=t@t",
      "-c",
      "core.hooksPath=/dev/null",
    ])
    .args(args)
    .current_dir(fx.root())
    .output()
    .expect("git");
  assert!(
    out.status.success(),
    "git {args:?}: {}",
    String::from_utf8_lossy(&out.stderr)
  );
}

/// A registered, migrated board with one item, on disk and in the store as one
/// write left them and committed, and the path of its `board.json`.
fn a_board_as_the_store_left_it(fx: &Fixture) -> std::path::PathBuf {
  let mut f = fx.facade_on_disk();
  f.wb_register("cc", "Control Claude", "control")
    .expect("register");
  f.wb_add("cc", WbItemKind::Todo, "the first item")
    .expect("add");
  git(fx, &["init", "-q"]);
  git(fx, &["add", "intent/whiteboard"]);
  git(fx, &["commit", "-q", "-m", "the board"]);
  fx.root().join("intent/whiteboard/cc/board.json")
}

/// A pull: `bytes` land at `path` and are committed on top, as a merge would.
fn pull(fx: &Fixture, path: &std::path::Path, bytes: String) {
  std::fs::write(path, bytes).expect("the pull");
  git(fx, &["commit", "-q", "-am", "the pull"]);
}

/// The file a teammate's clone would push: this board with one more item.
fn pulled(path: &std::path::Path, text: &str) -> String {
  let mut board: Board =
    serde_json::from_str(&std::fs::read_to_string(path).expect("the board file")).expect("a board");
  let mut item = board.items[0].clone();
  item.seq += 1;
  item.text = text.to_string();
  board.items.push(item);
  intentsvcs::model::to_canonical_json(&board).expect("json")
}

fn item_texts(fx: &Fixture) -> Vec<String> {
  fx.facade_on_disk()
    .boards()
    .expect("boards")
    .into_iter()
    .flat_map(|b| b.items.into_iter().map(|i| i.text))
    .collect()
}

#[test]
fn a_pulled_board_is_taken_when_only_its_file_moved() {
  let fx = Fixture::new();
  let path = a_board_as_the_store_left_it(&fx);
  pull(&fx, &path, pulled(&path, "a teammate's item"));

  let carried = fx
    .facade_on_disk()
    .carry_pulled_boards()
    .expect("the carry");

  assert_eq!(carried, vec!["cc".to_string()]);
  assert!(
    item_texts(&fx).contains(&"a teammate's item".to_string()),
    "the pulled item is on the board: {:?}",
    item_texts(&fx)
  );
  assert!(
    fx.facade_on_disk()
      .carry_pulled_boards()
      .expect("again")
      .is_empty(),
    "a board the store has taken is not ahead any more"
  );
}

/// **THE 0216 ARM.** The store took a write whose render has not reached the
/// file, and then the file moved too. Both sides moved, so neither may outvote
/// the other: nothing is taken, and the store's write stands.
#[test]
fn a_board_both_sides_moved_is_left_and_the_store_s_write_stands() {
  let fx = Fixture::new();
  let path = a_board_as_the_store_left_it(&fx);

  let mut store = Store::open(&fx.project().db_path()).expect("the store");
  let mut boards = store.hydrate_boards().expect("boards");
  let mut item = boards[0].items[0].clone();
  item.seq += 1;
  item.text = "a write not yet rendered".to_string();
  boards[0].items.push(item);
  store.replace_boards(&boards).expect("the unrendered write");
  drop(store);

  pull(&fx, &path, pulled(&path, "a teammate's item"));

  let carried = fx
    .facade_on_disk()
    .carry_pulled_boards()
    .expect("the carry");

  assert!(
    carried.is_empty(),
    "a board both sides moved was taken: {carried:?}"
  );
  let texts = item_texts(&fx);
  assert!(
    texts.contains(&"a write not yet rendered".to_string()),
    "the store's write was reverted: {texts:?}"
  );
  assert!(
    !texts.contains(&"a teammate's item".to_string()),
    "{texts:?}"
  );
}

/// **AN OLDER CHECKOUT IS NOT A PULL** (vc, 2026-09-25): the file moved, and the
/// store did not, but the store's bytes are NEWER than the file's, so taking it
/// would roll the board back. It is left for the refusal 3.2.1 ships.
#[test]
fn a_checkout_of_an_older_board_is_not_taken() {
  let fx = Fixture::new();
  let path = a_board_as_the_store_left_it(&fx);
  pull(&fx, &path, pulled(&path, "a later item"));
  fx.facade_on_disk()
    .carry_pulled_boards()
    .expect("take the later board");
  git(&fx, &["checkout", "-q", "HEAD~1"]);

  let carried = fx
    .facade_on_disk()
    .carry_pulled_boards()
    .expect("the carry");

  assert!(
    carried.is_empty(),
    "an older checkout was taken: {carried:?}"
  );
  assert!(item_texts(&fx).contains(&"a later item".to_string()));
}

/// The control: a board whose file did not move is never taken, however the
/// store differs from it.
#[test]
fn a_board_whose_file_did_not_move_is_not_taken() {
  let fx = Fixture::new();
  a_board_as_the_store_left_it(&fx);
  assert!(
    fx.facade_on_disk()
      .carry_pulled_boards()
      .expect("the carry")
      .is_empty()
  );
}
