//! Issue `0420`: a store write met `database is locked` with no connection
//! holding the lock by the time anyone looked.
//!
//! **TWO MECHANISMS, AND EACH ARM HOLDS ONE.** A write transaction opened
//! DEFERRED that reads before it writes (the 0206 compare-and-swap does) is a
//! read transaction asking to upgrade, and SQLite refuses the upgrade at once
//! rather than running the busy handler -- so the five-second wait never
//! applied, and the refusal needed only another writer active at that moment.
//! And the canon half of the prose index was replaced under `secure-delete`,
//! whose per-row index rewrite made every mutation and every ingest hold the
//! writer lock for seconds before the `rebuild` threw that work away.
//!
//! No timing arm: this host's load holds above ten, and a wait measured here
//! says more about the load than about the store.

use crate::common::{Fixture, sample_thread};
use intentsvcs::sync::Scope;

/// Hold the writer lock from a second connection for `for_ms`, then commit.
///
/// **THE LOCK IS TAKEN BEFORE THIS RETURNS**, so the write under test starts
/// with the writer already holding it, never racing it for the lock.
fn hold_the_writer_lock(fx: &Fixture, for_ms: u64) -> std::thread::JoinHandle<()> {
  let db = fx.project().db_path();
  let (held, is_held) = std::sync::mpsc::channel();
  let holder = std::thread::spawn(move || {
    let conn = rusqlite::Connection::open(db).expect("the holder opens the store");
    conn
      .execute_batch("BEGIN IMMEDIATE")
      .expect("the holder takes the writer lock");
    held.send(()).expect("the test is waiting");
    std::thread::sleep(std::time::Duration::from_millis(for_ms));
    conn
      .execute_batch("COMMIT")
      .expect("the holder releases it");
  });
  is_held.recv().expect("the holder took the lock");
  holder
}

#[test]
fn an_edit_that_reads_first_waits_for_a_held_writer_and_lands() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut f = fx.facade_on_disk();

  // Well inside the store's five-second wait, so a write that waits lands.
  let holder = hold_the_writer_lock(&fx, 1500);
  let edited = f.ac_edit(
    "ST0001",
    "AC-03.1",
    Some("the edit that waited".to_string()),
    None,
  );
  holder.join().expect("the holder finished");

  edited.expect(
    "an edit derived from a current record must wait for the writer lock and land; a \
     refusal here is the deferred upgrade issue 0420 names, refused without waiting",
  );
  let after = fx.facade_on_disk();
  let text = after
    .st_show("ST0001")
    .unwrap()
    .criteria
    .iter()
    .find(|c| c.id == "AC-03.1")
    .expect("ST0001 has AC-03.1")
    .text
    .clone();
  assert_eq!(text, "the edit that waited");
}

fn secure_delete_of(fx: &Fixture, table: &str) -> String {
  rusqlite::Connection::open(fx.project().db_path())
    .unwrap()
    .query_row(
      &format!("SELECT v FROM {table}_config WHERE k = 'secure-delete'"),
      [],
      |row| row.get::<_, rusqlite::types::Value>(0),
    )
    .map(|v| format!("{v:?}"))
    .unwrap_or_else(|e| format!("no row: {e}"))
}

/// **THE WHOLESALE REPLACE TURNS `secure-delete` OFF AND MUST TURN IT BACK ON.**
/// Issue 0355's scoped refresh door deletes without a rebuild after it, and it
/// relies on the setting to leave no tombstones. A replace that left it off
/// would change nothing any search answers today, and would grow the index
/// with every refresh from then on.
#[test]
fn secure_delete_is_on_after_a_mutation_an_ingest_and_an_index_rebuild() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut f = fx.facade_on_disk();
  let on = format!("{:?}", rusqlite::types::Value::Integer(1));

  f.ac_edit(
    "ST0001",
    "AC-03.1",
    Some("a mutation replaces the canon half".to_string()),
    None,
  )
  .expect("the edit lands");
  assert_eq!(
    secure_delete_of(&fx, "doc_sections"),
    on,
    "after a mutation"
  );

  f.ingest_from_disk(&Scope::All).expect("the ingest runs");
  assert_eq!(secure_delete_of(&fx, "doc_sections"), on, "after an ingest");

  std::fs::create_dir_all(fx.root().join("src")).expect("src");
  std::fs::write(fx.root().join("src/lib.rs"), "pub fn indexed() {}\n").expect("lib");
  f.index_rebuild().expect("the index rebuilds");
  assert_eq!(
    secure_delete_of(&fx, "src_sections"),
    on,
    "after an index rebuild, which replaces the source half"
  );
}

/// **ONE DOOR OPENS A TRANSACTION IN THE STORE, AND IT TAKES THE WRITER LOCK.**
/// Every transaction the store opens writes, so a second spelling is a write
/// that can come back deferred -- and be refused without waiting -- with
/// nothing else to notice it.
#[test]
fn the_store_opens_every_transaction_through_the_write_door() {
  let path = testkit::workspace_root().join("crates/intentsvcs/src/store.rs");
  let body = std::fs::read_to_string(&path).expect("store.rs is readable");
  assert_eq!(
    body.matches(".transaction()").count(),
    0,
    "{}: a transaction opened with rusqlite's default is DEFERRED; open it through `Store::write_tx` (issue 0420)",
    path.display()
  );
  assert_eq!(
    body.matches("transaction_with_behavior(").count(),
    1,
    "{}: the writer lock is taken in exactly one place, `Store::write_tx`; a second spelling is a second home for it (issue 0420)",
    path.display()
  );
}
