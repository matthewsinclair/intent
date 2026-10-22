//! Issue 0605: **every scoped ingest doubled the file and whiteboard halves of
//! the prose index.**
//!
//! A scoped restore (`intent sync --to-store <ID>`, and the daemon's sync of
//! one thread from disk) composed the sections it installs by reading every
//! stored section back. The canon writer's delete spares the file and
//! whiteboard halves, and its insert took every section it was handed, so each
//! of those rows went in again beside itself: Utilz's store reached 2^11
//! copies of every such section in one day, and 5.3 GB.
//!
//! The rows are read through a fresh `rusqlite` connection, as the sibling
//! `replacing_the_prose_index_truncates_it.rs` reads its shadow tables: the
//! defect is a count per owner type, which no public reader returns.

use std::collections::BTreeMap;

use crate::common::{Fixture, sample_thread};
use intentsvcs::ingest;
use intentsvcs::prose::{DocSection, FILE_OWNER, WB_OWNER};
use intentsvcs::store::Store;
use intentsvcs::sync::Scope;

fn section(owner_type: &str, owner_id: &str, file: &str, body: &str) -> DocSection {
  DocSection {
    owner_type: owner_type.to_string(),
    owner_id: owner_id.to_string(),
    file: file.to_string(),
    seq: 0,
    heading: None,
    level: 0,
    body: body.to_string(),
  }
}

/// `doc_sections` rows by owner type.
fn rows_by_owner(fx: &Fixture) -> BTreeMap<String, i64> {
  let conn = rusqlite::Connection::open(fx.project().db_path()).expect("open the store");
  let mut stmt = conn
    .prepare("SELECT owner_type, count(*) FROM doc_sections GROUP BY owner_type")
    .expect("prepare the count");
  stmt
    .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
    .expect("count the rows")
    .collect::<Result<_, _>>()
    .expect("read the counts")
}

/// The scoped sync, run twice, leaves every half's row count where it was.
#[test]
fn a_scoped_sync_leaves_every_halfs_row_count_where_it_was() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let project = fx.project();
  {
    let mut store = Store::open(&project.db_path()).expect("open the store");
    ingest::load(&project, &mut store).expect("load the canon");
    store
      .replace_file_sections(&[section(
        FILE_OWNER,
        "README.md",
        "README.md",
        "readme prose\n",
      )])
      .expect("write the file half");
    store
      .replace_wb_sections_for(
        "dc",
        &[section(
          WB_OWNER,
          "dc",
          "intent/whiteboard/dc/.history/x.md",
          "a fold\n",
        )],
      )
      .expect("write the whiteboard half");
  }
  let before = rows_by_owner(&fx);
  assert_eq!(
    before.get(FILE_OWNER),
    Some(&1),
    "the file half: {before:?}"
  );
  assert_eq!(
    before.get(WB_OWNER),
    Some(&1),
    "the whiteboard half: {before:?}"
  );
  assert!(
    before.len() > 2,
    "the canon half is empty, so the arm cannot see it kept: {before:?}"
  );

  for _ in 0..2 {
    let mut store = Store::open(&project.db_path()).expect("reopen the store");
    ingest::resync(
      &project,
      &mut store,
      &Scope::Threads(vec!["ST0001".to_string()]),
    )
    .expect("the scoped sync");
  }
  assert_eq!(
    rows_by_owner(&fx),
    before,
    "a scoped sync changed how many rows a half holds"
  );
}

/// Rung 31 leaves one row per (owner_type, owner_id, file, seq) on a store
/// the doubling reached, and the full-text index still finds the content.
#[test]
fn the_rung_leaves_one_row_per_key_and_the_content_findable() {
  let fx = Fixture::new();
  let path = fx.project().db_path();
  drop(Store::open(&path).expect("create the store"));
  {
    let conn = rusqlite::Connection::open(&path).expect("open the store");
    for _ in 0..4 {
      conn
        .execute(
          "INSERT INTO doc_sections (owner_type, owner_id, file, seq, heading, level, body)
           VALUES (?1, 'README.md', 'README.md', 0, NULL, 0, 'quixotic doubling')",
          [FILE_OWNER],
        )
        .expect("plant a duplicate");
    }
    conn
      .execute(
        "INSERT INTO doc_sections (owner_type, owner_id, file, seq, heading, level, body)
         VALUES ('thread', 'ST0001', 'intent/st/ST0001/info.md', 0, NULL, 0, 'a single row')",
        [],
      )
      .expect("plant a row with no duplicate");
    // The store is the current DDL's, and rung 32 after this one reads the
    // message tables as schema 31 held them.
    crate::common::wb_messages_as_at_31(&conn);
    conn
      .pragma_update(None, "user_version", 30)
      .expect("stamp the version before the rung");
  }

  drop(Store::open(&path).expect("the store migrates"));

  let conn = rusqlite::Connection::open(&path).expect("reopen the store");
  let count = |sql: &str| -> i64 { conn.query_row(sql, [], |row| row.get(0)).expect("count") };
  assert_eq!(
    count("SELECT count(*) FROM doc_sections WHERE owner_type = 'file'"),
    1,
    "four copies of one key leave one"
  );
  assert_eq!(
    count("SELECT count(*) FROM doc_sections WHERE owner_type = 'thread'"),
    1,
    "a key with no duplicate keeps its row"
  );
  assert_eq!(
    count("SELECT count(*) FROM doc_sections WHERE doc_sections MATCH 'quixotic'"),
    1,
    "the full-text index finds the one row left"
  );
}

/// Doctor names duplicated sections, shown and not counted, and says nothing
/// on a store with none.
#[test]
fn doctor_shows_duplicated_sections_without_counting_them() {
  let fx = Fixture::new();
  let store = Store::open(&fx.project().db_path()).expect("open the store");
  let duplicated = |store: &Store| -> Vec<String> {
    intentsvcs::doctor::diagnose(
      &fx.project(),
      &crate::common::ctx(),
      Some(store),
      intentsvcs::doctor::Scope::All,
    )
    .findings
    .into_iter()
    .filter(|f| f.class == intentsvcs::finding::FindingClass::ProseIndexDuplicated)
    .map(|f| f.detail)
    .collect()
  };
  assert!(duplicated(&store).is_empty(), "a clean store names nothing");

  let conn = rusqlite::Connection::open(fx.project().db_path()).expect("open the store");
  for _ in 0..2 {
    conn
      .execute(
        "INSERT INTO doc_sections (owner_type, owner_id, file, seq, heading, level, body)
         VALUES ('wb_node', 'dc', 'intent/whiteboard/dc/.history/x.md', 0, NULL, 0, 'a fold')",
        [],
      )
      .expect("plant a duplicate");
  }
  let named = duplicated(&store);
  assert_eq!(named.len(), 1, "one finding: {named:?}");
  assert!(named[0].contains("each of 1 section"), "{named:?}");
  assert!(
    !intentsvcs::finding::FindingClass::ProseIndexDuplicated.is_actionable()
      && intentsvcs::finding::FindingClass::ProseIndexDuplicated.is_shown_by_default(),
    "shown on a default run and not counted"
  );
}
