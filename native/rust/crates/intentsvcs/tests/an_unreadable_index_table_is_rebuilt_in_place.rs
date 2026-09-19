//! Issue 0453: **`intent index rebuild` repairs an unreadable search-index
//! table in place, and repairs nothing else.**
//!
//! 0447 made a store whose `doc_sections` cannot be read say so. It did not
//! make any verb work: every door opened the store through that read,
//! `index rebuild` included, so the verb that should repair the table sat
//! behind the wall it was meant to take down.
//!
//! **THE DOOR HERE IS THE ONE THE CLI TAKES**: `intent index rebuild` opens
//! with `Opening::RepairingIndex` and then runs `index_rebuild`, and nothing
//! else. The CLI crate may not reach the store's SQL (D06, held by
//! `dep_graph_guard.rs`), so the fault is injected here, where it can be.
//!
//! **THE DAMAGE IS MADE THE WAY 0447's DRIVE MADE IT**, on the store file
//! directly and after the canon is in it.

use crate::common::{Fixture, facade_ctx, sample_thread};
use intentsvcs::facade::{Facade, FacadeError, Opening};
use intentsvcs::ingest::IngestError;
use intentsvcs::remedy::Remedy;
use intentsvcs::store::{Store, StoreError};

fn warm(fx: &Fixture) {
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = Facade::open(fx.project(), facade_ctx()).expect("the store warms from canon");
  facade
    .issue_add(
      "an issue with prose",
      None,
      None,
      "the zebracorn lives here",
    )
    .expect("file an issue whose body is indexed");
}

fn damage(fx: &Fixture, sql: &str) {
  let db = rusqlite::Connection::open(fx.root().join("intent/.cache/intent.db"))
    .expect("open the store directly");
  db.execute_batch(sql).expect("damage the store");
}

const UNREADABLE_INDEX: &str =
  "DROP TABLE doc_sections; CREATE TABLE doc_sections (unrelated TEXT);";

fn schema_of(fx: &Fixture, table: &str) -> String {
  let db = rusqlite::Connection::open(fx.root().join("intent/.cache/intent.db"))
    .expect("open the store directly");
  db.query_row(
    "SELECT sql FROM sqlite_master WHERE name = ?1",
    [table],
    |row| row.get(0),
  )
  .expect("the table is still declared")
}

#[test]
fn the_rebuild_repairs_the_table_the_open_refuses_on() {
  let fx = Fixture::new();
  warm(&fx);
  damage(&fx, UNREADABLE_INDEX);

  // The wall 0447 named: the ordinary open refuses.
  assert!(
    matches!(
      Facade::open(fx.project(), facade_ctx()),
      Err(FacadeError::Ingest(IngestError::IndexUnreadable {
        table: "doc_sections",
        ..
      }))
    ),
    "the fault must stop the ordinary open, or nothing below is tested"
  );

  let mut facade = Facade::open_as(fx.project(), facade_ctx(), Opening::RepairingIndex)
    .expect("the repairing open gets past the table the ordinary one refuses on");
  facade.index_rebuild().expect("and the rebuild runs");
  drop(facade);

  // Every door works again, the ordinary open first.
  let facade = Facade::open(fx.project(), facade_ctx()).expect("the ordinary open works again");
  assert_eq!(
    facade.st_list().len(),
    1,
    "the thread the store held is still there"
  );
  assert_eq!(facade.issue_list().len(), 1, "and the issue");
  drop(facade);

  // **CANON'S HALF OF THE INDEX IS PUT BACK FROM THE RECORDS**, not left for
  // the next full ingest: an issue body and an attachment of the thread are
  // both findable, and neither is a file the rebuild's walk reads.
  let store = Store::open(&fx.project().db_path()).expect("the store opens");
  let found = |q: &str| store.search(q).expect("search runs");
  assert!(
    found("zebracorn").iter().any(|s| s.owner_type == "issue"),
    "the issue body is indexed again"
  );
  assert!(
    !found("quokka").is_empty(),
    "the thread's attachment prose is indexed again"
  );
}

#[test]
fn a_rebuild_of_a_healthy_store_changes_what_it_finds_not_at_all() {
  let fx = Fixture::new();
  warm(&fx);
  let before = {
    let store = Store::open(&fx.project().db_path()).expect("the store opens");
    let mut rows = store.doc_sections().expect("readable");
    rows.retain(|s| s.owner_type != "file");
    rows
  };

  let mut facade = Facade::open_as(fx.project(), facade_ctx(), Opening::RepairingIndex)
    .expect("the repairing open works on a healthy store");
  facade.index_rebuild().expect("the rebuild runs");
  drop(facade);

  let store = Store::open(&fx.project().db_path()).expect("the store opens");
  let mut after = store.doc_sections().expect("readable");
  after.retain(|s| s.owner_type != "file");
  assert_eq!(
    before, after,
    "canon's half of the index is recomputed to the same rows, so a rebuild on a healthy store loses nothing"
  );
}

#[test]
fn a_table_the_store_owns_that_cannot_be_read_stops_the_repair_before_anything_is_dropped() {
  let fx = Fixture::new();
  warm(&fx);
  damage(&fx, UNREADABLE_INDEX);
  damage(
    &fx,
    "DROP TABLE embeddings; CREATE VIEW embeddings AS SELECT * FROM nowhere;",
  );

  let err = match Facade::open_as(fx.project(), facade_ctx(), Opening::RepairingIndex) {
    Ok(_) => panic!("the repair ran past a table it does not own and cannot read"),
    Err(e) => e,
  };
  let FacadeError::Store(StoreError::IndexRepairBlocked { ref table, .. }) = err else {
    panic!("an unreadable owned table is its own refusal: {err:?}");
  };
  assert_eq!(table, "embeddings", "and it names the table");

  let remedy = err.remedy();
  assert!(
    remedy.contains("nothing was dropped or changed") && remedy.contains("`embeddings`"),
    "{remedy}"
  );
  assert!(
    remedy.contains("Do NOT delete the store"),
    "the store is the source of truth: {remedy}"
  );

  // **NOTHING WAS DROPPED**: the damaged index table is exactly as it was,
  // because the probe runs before the first write.
  assert_eq!(
    schema_of(&fx, "doc_sections"),
    "CREATE TABLE doc_sections (unrelated TEXT)"
  );
  assert!(
    schema_of(&fx, "embeddings").starts_with("CREATE VIEW embeddings"),
    "and the owned table is never touched"
  );
}
