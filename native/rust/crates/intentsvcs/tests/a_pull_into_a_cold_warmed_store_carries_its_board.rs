//! **Issue 0619: a store warmed cold from a clone's files records every board
//! it read, so the next pull's change to one of them is taken.**
//!
//! The cold warm of a fresh clone put every board into the store and recorded
//! none of their bytes in the file index, while a write and a sync read both
//! record what they touch. A board counts as moved only against such a record,
//! so a later pull that changed a board the store held read as the store being
//! newer: the hook's `sync --apply` took the pull's events and left the board,
//! and a teammate's message reached the clone only through a manual `intent
//! sync --to-store` (dc's measurement, 2026-10-22, on the installed 3.2.4
//! pair). It recorded none because the pass decided which boards were ahead
//! of the store before the warm took them in, against a store that held none,
//! and the files of a board ahead keep the rows they had before the pass.

use crate::common::{Fixture, git_out};
use intentsvcs::sync::Scope;

const BODY: &str = "hello across the clones";

/// How many messages from aa to bb carrying [`BODY`] `fx`'s store holds.
fn held(fx: &Fixture) -> i64 {
  rusqlite::Connection::open(fx.project().db_path())
    .expect("the store")
    .query_row(
      "SELECT count(*) FROM wb_message WHERE sender = 'aa' AND recipient = 'bb' AND body = ?1",
      [BODY],
      |r| r.get(0),
    )
    .expect("count the message")
}

#[test]
fn a_pull_that_changes_a_board_a_cold_warmed_store_holds_is_carried() {
  let origin = Fixture::new();
  origin.write_file(".gitignore", "intent/.cache/\n");
  origin.git_init();
  let mut f = origin.facade_on_disk();
  f.wb_register("aa", "Node A", "worker")
    .expect("register aa");
  f.wb_register("bb", "Node B", "worker")
    .expect("register bb");
  f.sync_to_disk(&Scope::All).expect("render every view");
  drop(f);
  origin.git_commit_all();

  // A teammate's clone, its store warmed cold from the files at its first open
  // and written by nothing since.
  let clone = Fixture {
    dir: tempfile::tempdir().expect("tempdir"),
  };
  git_out(
    clone.root(),
    &["clone", "-q", &origin.root().to_string_lossy(), "."],
  );
  assert_eq!(
    clone
      .facade_on_disk()
      .carry_pulled_boards()
      .expect("a carry with nothing pulled"),
    Vec::<String>::new(),
    "control: the warm leaves nothing to carry"
  );

  // aa asks bb in the origin, which commits it, and the clone pulls.
  let mut f = origin.facade_on_disk();
  f.wb_ask("aa", "bb", BODY, None, false).expect("aa asks bb");
  drop(f);
  git_out(origin.root(), &["add", "-A"]);
  git_out(origin.root(), &["commit", "-q", "-m", "aa asks bb"]);
  git_out(clone.root(), &["pull", "-q"]);

  let carried = clone
    .facade_on_disk()
    .carry_pulled_boards()
    .expect("the hook's carry");
  assert_eq!(
    (carried, held(&clone)),
    (vec!["aa".to_string()], 1),
    "the pull's board is taken into the cold-warmed store, and with it aa's message"
  );
}
