//! The 0442 detector's probes, read on a real store file.
//!
//! **THE DAMAGE IS PLANTED IN 0442's OWN SHAPE**: a content row and its docsize
//! row removed together while the index keeps its postings, which is what
//! `sqlite3Fts5StorageDelete`'s gated order leaves when the index half is the
//! half that survives. That shape is the reason the index-side probe exists: the
//! shadow-table probe was measured reading zero over it, so this arm holds both
//! halves of that property at once -- the orphan named, the shadows silent.
use intentsvcs::doctor::{Orphans, Pair, SearchIndexReading};
use intentsvcs::index::source::whole_file;
use intentsvcs::store::Store;

fn store_with_code(dir: &std::path::Path) -> std::path::PathBuf {
  let db = dir.join("intent.db");
  let mut store = Store::open(&db).expect("open a fresh store");
  store
    .replace_src_sections(&[
      whole_file("a.rs", "fn alpha() -> u32 { 1 }"),
      whole_file("b.rs", "fn beta() -> u32 { alpha() + 1 }"),
    ])
    .expect("write source rows");
  db
}

fn src(readings: &[SearchIndexReading]) -> &SearchIndexReading {
  readings
    .iter()
    .find(|r| r.table == "src_sections")
    .expect("the source table is read")
}

#[test]
fn a_clean_index_reads_clean_on_every_probe() {
  let dir = tempfile::tempdir().expect("tempdir");
  let db = store_with_code(dir.path());
  let readings = Store::open(&db)
    .expect("reopen")
    .read_search_index()
    .expect("the probes run");
  assert_eq!(
    readings
      .iter()
      .map(|r| r.table.as_str())
      .collect::<Vec<_>>(),
    ["src_sections", "doc_sections"]
  );
  for r in &readings {
    assert_eq!(r.orphaned, Orphans::Docids(vec![]), "{}", r.table);
    assert_eq!(r.structure, None, "{}", r.table);
    assert_eq!(r.pair(), Pair::BothClean, "{}", r.table);
    assert!(!r.damaged(), "{}", r.table);
  }
}

#[test]
fn a_document_the_index_holds_with_no_content_row_is_named_while_the_shadows_agree() {
  let dir = tempfile::tempdir().expect("tempdir");
  let db = store_with_code(dir.path());
  let conn = rusqlite::Connection::open(&db).expect("a second connection");
  let victim: i64 = conn
    .query_row("SELECT min(id) FROM src_sections_content", [], |row| {
      row.get(0)
    })
    .expect("a content row");
  conn
    .execute("DELETE FROM src_sections_content WHERE id = ?1", [victim])
    .expect("remove the content row");
  conn
    .execute("DELETE FROM src_sections_docsize WHERE id = ?1", [victim])
    .expect("remove its docsize row");
  drop(conn);

  let readings = Store::open(&db)
    .expect("reopen")
    .read_search_index()
    .expect("the probes run");
  let r = src(&readings);
  assert_eq!(
    r.orphaned,
    Orphans::Docids(vec![victim]),
    "the index-side probe names the docid"
  );
  assert!(
    !r.shadows_disagree(),
    "the shadow tables lost the row together, so that probe reads zero here: {r:?}"
  );
  assert!(r.damaged(), "{r:?}");
  assert_ne!(r.pair(), Pair::BothClean, "{r:?}");
}

#[test]
fn the_pair_verdict_is_a_statement_about_both_probes() {
  let reading = |orphaned: Vec<i64>, structure: Option<String>| SearchIndexReading {
    table: "src_sections".to_string(),
    orphaned: Orphans::Docids(orphaned),
    structure,
    docsize_without_content: 0,
    content_without_docsize: 0,
  };
  let dirty = || Some("fts5: checksum mismatch".to_string());
  assert_eq!(reading(vec![7], dirty()).pair(), Pair::BothDirty);
  assert_eq!(reading(vec![7], None).pair(), Pair::OrphansOnly);
  assert_eq!(reading(vec![], dirty()).pair(), Pair::StructureOnly);
  assert_eq!(reading(vec![], None).pair(), Pair::BothClean);

  // A probe that could not read the index is not a clean reading.
  let unreadable = SearchIndexReading {
    orphaned: Orphans::Unreadable("database disk image is malformed".to_string()),
    ..reading(vec![], None)
  };
  assert_eq!(unreadable.pair(), Pair::OrphansOnly);

  // The shadow probe alone damages a table without moving the pair.
  let shadows = SearchIndexReading {
    docsize_without_content: 1,
    ..reading(vec![], None)
  };
  assert_eq!(shadows.pair(), Pair::BothClean);
  assert!(shadows.damaged());
}
