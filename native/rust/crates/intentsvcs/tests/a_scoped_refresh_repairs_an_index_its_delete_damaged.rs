//! **A SCOPED REFRESH REPAIRS A SEARCH TABLE ITS DELETE LEFT DAMAGED, AND SAYS
//! SO** (hv, 3.2.2).
//!
//! On 2026-09-25 Intent's own `src_sections` held docid 882, an earlier
//! `views.rs`, with no content row and no docsize row, and fts5's check read a
//! checksum mismatch. The row had gone through the scoped refresh's delete
//! under `secure-delete`; see `store::repair_if_damaged` for the reading of
//! SQLite's source that explains it, and for why it could not be reproduced.
//!
//! **THE DAMAGE IS PLANTED IN ITS MEASURED SHAPE**: a content row and its
//! docsize row removed behind FTS5's back, so the index still holds the doc's
//! terms. That is what 882 was, and it is what fts5's check and doctor's
//! orphan probe both read as dirty.

use crate::common::Fixture;
use std::path::PathBuf;

struct Tree {
  fx: Fixture,
  lib: PathBuf,
}

fn tree() -> Tree {
  let fx = Fixture::new();
  std::fs::create_dir_all(fx.root().join("src")).expect("mkdir");
  let lib = fx.root().join("src/lib.rs");
  std::fs::write(&lib, "fn wombat() {}\n").expect("lib");
  std::fs::write(
    fx.root().join("src/other.rs"),
    "fn quokka() {}\nfn wallaby() {}\n",
  )
  .expect("other");
  Tree { fx, lib }
}

fn store(fx: &Fixture) -> rusqlite::Connection {
  rusqlite::Connection::open(fx.project().db_path()).expect("open the store")
}

/// fts5's own check of `src_sections`, as `doctor` reads it.
fn objection(fx: &Fixture) -> Option<String> {
  objection_in(fx, "src_sections")
}

/// fts5's own check of one table: `None` for its single `ok`.
fn objection_in(fx: &Fixture, table: &str) -> Option<String> {
  let lines: Vec<String> = store(fx)
    .prepare(&format!("PRAGMA main.integrity_check({table})"))
    .expect("prepare the check")
    .query_map([], |row| row.get(0))
    .expect("run the check")
    .collect::<Result<_, _>>()
    .expect("read the check");
  (lines != ["ok"]).then(|| lines.join("; "))
}

/// Remove one file's content and docsize rows and leave its index entries:
/// the orphan 882 was. Returns the orphaned docid.
fn orphan(fx: &Fixture, path: &str) -> i64 {
  orphan_in(fx, "src_sections", "path", path)
}

fn orphan_in(fx: &Fixture, table: &str, column: &str, path: &str) -> i64 {
  let db = store(fx);
  let id: i64 = db
    .query_row(
      &format!("SELECT rowid FROM {table} WHERE {column} = ?1 LIMIT 1"),
      [path],
      |row| row.get(0),
    )
    .expect("the file is indexed");
  for shadow in ["content", "docsize"] {
    db.execute(&format!("DELETE FROM {table}_{shadow} WHERE id = ?1"), [id])
      .expect("drop a shadow row");
  }
  id
}

#[test]
fn a_refresh_whose_delete_leaves_the_table_damaged_rebuilds_it_and_names_the_repair() {
  let t = tree();
  let mut f = t.fx.facade_on_disk();
  f.index_rebuild().expect("rebuild");
  let docid = orphan(&t.fx, "src/other.rs");
  assert!(
    objection(&t.fx).is_some(),
    "the planted orphan did not trip fts5's check, so nothing below can see a repair"
  );

  std::fs::write(&t.lib, "fn wombat() {}\nfn numbat() {}\n").expect("edit lib");
  let refreshed = f
    .index_refresh(Some(std::slice::from_ref(&t.lib)))
    .expect("the refresh");

  let repair = refreshed
    .repaired
    .expect("the refresh deleted a row from a damaged table and did not repair it");
  assert_eq!(repair.table, "src_sections");
  assert_eq!(repair.orphaned, vec![docid]);
  assert_eq!(
    repair.remaining, None,
    "the rebuild did not clear fts5's objection"
  );
  let said = repair.sentence();
  assert!(
    said.contains("`src_sections`")
      && said.contains(&docid.to_string())
      && said.contains("rebuilt"),
    "the repair's sentence must name the table, the orphan and the rebuild: {said}"
  );
  assert_eq!(
    objection(&t.fx),
    None,
    "the table is still damaged after the repair"
  );
}

#[test]
fn a_refresh_over_a_clean_table_repairs_nothing() {
  let t = tree();
  let mut f = t.fx.facade_on_disk();
  f.index_rebuild().expect("rebuild");
  std::fs::write(&t.lib, "fn wombat() {}\nfn numbat() {}\n").expect("edit lib");
  let refreshed = f
    .index_refresh(Some(std::slice::from_ref(&t.lib)))
    .expect("the refresh");
  assert_eq!(refreshed.updated, vec!["src/lib.rs".to_string()]);
  assert_eq!(
    refreshed.repaired, None,
    "a clean table was reported repaired"
  );
}

/// **THE CHECK RUNS ONLY AFTER A DELETE, AND THIS IS THE ARM THAT HOLDS IT.**
/// An insert cannot orphan anything, and the check costs a scan of the table,
/// so a refresh that only adds a file leaves even a damaged table to the next
/// refresh that deletes -- and to `doctor`, which reports it meanwhile.
#[test]
fn a_refresh_that_only_adds_a_file_does_not_pay_for_the_check() {
  let t = tree();
  let mut f = t.fx.facade_on_disk();
  f.index_rebuild().expect("rebuild");
  orphan(&t.fx, "src/other.rs");
  let added = t.fx.root().join("src/new.rs");
  std::fs::write(&added, "fn numbat() {}\n").expect("add a file");
  let refreshed = f
    .index_refresh(Some(std::slice::from_ref(&added)))
    .expect("the refresh");
  assert_eq!(refreshed.updated, vec!["src/new.rs".to_string()]);
  assert_eq!(refreshed.repaired, None);
  assert!(
    objection(&t.fx).is_some(),
    "an add-only refresh rebuilt the table"
  );
}

/// **`doc_sections` IS REPAIRED ON INTENTD'S SCHEDULED SWEEP, NOT IN THE
/// REFRESH**: its check holds the writer lock for about 330 ms on Intent's own
/// store. This drives the door the sweep calls; a clean table is left alone.
#[test]
fn the_prose_index_is_repaired_by_the_door_the_scheduled_sweep_calls() {
  let t = tree();
  std::fs::create_dir_all(t.fx.root().join("docs")).expect("mkdir");
  std::fs::write(
    t.fx.root().join("docs/guide.md"),
    "# Guide\n\nabout wombats\n",
  )
  .expect("guide");
  let mut f = t.fx.facade_on_disk();
  f.index_rebuild().expect("rebuild");
  assert_eq!(
    f.index_repair_prose().expect("the check"),
    None,
    "a clean table was repaired"
  );

  let docid = orphan_in(&t.fx, "doc_sections", "file", "docs/guide.md");
  assert!(
    objection_in(&t.fx, "doc_sections").is_some(),
    "the planted orphan did not trip fts5's check, so nothing below can see a repair"
  );
  let repair = f
    .index_repair_prose()
    .expect("the check")
    .expect("a damaged prose index was not repaired");
  assert_eq!(repair.table, "doc_sections");
  // Measured: on this planted shape the orphan probe reads no docid in the
  // prose table and fts5's own check is what objects, so that is what the
  // repair must name.
  assert!(
    repair.found.contains("fts5's own check objecting"),
    "the repair of docid {docid}'s damage must say what it found: {}",
    repair.sentence()
  );
  assert_eq!(repair.remaining, None);
  assert_eq!(objection_in(&t.fx, "doc_sections"), None);
}
