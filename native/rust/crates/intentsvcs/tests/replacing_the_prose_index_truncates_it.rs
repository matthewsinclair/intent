//! **THE SECOND MECHANISM OF ISSUE 0234, AND THE ONE THAT ACCOUNTS FOR THE
//! GIGABYTES.**
//!
//! `doc_sections` is written one way only: emptied and refilled. `DELETE FROM`
//! an FTS5 table does not remove the deleted rows' terms from the inverted
//! index -- it writes a DELETE MARKER for each of them into `doc_sections_data`
//! -- so under this write pattern the index accumulates tombstones on every
//! mutation, for the life of the store.
//!
//! **NOTHING REPORTS IT.** The row count is correct, the content table is the
//! right size, searches keep answering correctly, and `VACUUM` reclaims none of
//! it because tombstones are live data rather than free pages. The only visible
//! symptom is the one that surfaced this: `intent explore` taking seconds to
//! start on a project whose canon is a few megabytes.
//!
//! Measured on the worst project in the estate, after the duplicate rows of the
//! first mechanism were already repaired: **859 sections holding 5 MB of
//! content, against 589 MB of `doc_sections_data`.** An FTS5 `'rebuild'`
//! against the emptied content table took the whole store from 2.3 GB to
//! 14.8 MB with search intact.
//!
//! # Why this measures bytes rather than rows
//!
//! Every row-level assertion in the sibling file passes with this defect
//! present, because the defect is not in the rows. The tombstones live in a
//! shadow table the public API has no reader for, which is why this one reaches
//! past the facade to `doc_sections_data` directly -- there is no honest
//! narrower instrument, and a test written against the surface that IS exposed
//! would have gone green while the store grew to gigabytes.

use crate::common::{Fixture, sample_thread};
use intentsvcs::ingest;
use intentsvcs::search::{SearchQuery, Tier};
use intentsvcs::store::Store;

/// The size of the inverted index, in bytes of stored segment data.
///
/// Through a fresh `rusqlite` connection rather than the facade: this is a
/// shadow table FTS5 owns, and the store deliberately exposes no reader for it.
fn index_bytes(fx: &Fixture, shadow: &str) -> i64 {
  rusqlite::Connection::open(fx.project().db_path())
    .expect("open the store")
    .query_row(
      &format!("SELECT coalesce(sum(length(block)), 0) FROM {shadow}"),
      [],
      |row| row.get(0),
    )
    .expect("measure the inverted index")
}

#[test]
fn replacing_the_index_truncates_it_rather_than_tombstoning_it() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let canon = ingest::read(&fx.project()).expect("read the canon");
  assert!(
    !canon.sections.is_empty(),
    "the fixture indexed no prose at all, so nothing below can observe growth"
  );

  {
    let mut store = Store::open(&fx.project().db_path()).expect("open the store");
    store
      .replace_doc_sections(&canon.sections)
      .expect("the first write");
  }
  let once = index_bytes(&fx, "doc_sections_data");
  assert!(
    once > 0,
    "the inverted index is empty after a write, so this instrument cannot see it grow"
  );

  // **THE SAME SECTIONS, TWENTY MORE TIMES.** Identical input is the point: a
  // store holding exactly what it held before must not have grown. Twenty is
  // an ordinary week of mutations on a live project, not a stress test.
  {
    let mut store = Store::open(&fx.project().db_path()).expect("reopen the store");
    for _ in 0..20 {
      store
        .replace_doc_sections(&canon.sections)
        .expect("a replacement");
    }
  }
  let twenty_one = index_bytes(&fx, "doc_sections_data");

  assert!(
    twenty_one <= once * 2,
    "twenty-one identical writes left an index {twenty_one} bytes wide where one write leaves {once} -- \
     the replacement is tombstoning the old terms instead of truncating them"
  );
}

#[test]
fn a_scoped_refresh_keeps_search_answering_and_leaves_no_tombstones() {
  // Issue 0355: the scoped refresh door rebuilt both search tables from the
  // whole corpus after each delete, so a one-file refresh paid for every
  // section in the project. It no longer rebuilds, and the tables'
  // `secure-delete` is what keeps its deletes from leaving tombstones.
  let fx = Fixture::new();
  for dir in ["docs", "src"] {
    std::fs::create_dir_all(fx.root().join(dir)).expect("mkdir");
  }
  std::fs::write(
    fx.root().join("docs/guide.md"),
    "# Guide\n\nabout wombats\n",
  )
  .expect("guide");
  // Two words of different lengths, so every round changes each file's size
  // and every refresh deletes and re-inserts its sections.
  let rewrite = |round: usize| {
    let word = if round.is_multiple_of(2) {
      "quokka"
    } else {
      "wallaby"
    };
    let prose: String = (0..60).map(|i| format!("{word} {word}{i}\n")).collect();
    let code: String = (0..60).map(|i| format!("fn {word}_{i}() {{}}\n")).collect();
    std::fs::write(
      fx.root().join("docs/notes.md"),
      format!("# Notes\n\n{prose}"),
    )
    .expect("notes");
    std::fs::write(fx.root().join("src/lib.rs"), code).expect("lib");
  };
  let named = [
    fx.root().join("docs/notes.md"),
    fx.root().join("src/lib.rs"),
  ];

  let mut f = fx.facade_on_disk();
  rewrite(0);
  f.index_rebuild().expect("rebuild");
  rewrite(1);
  f.index_refresh(Some(&named)).expect("the first refresh");
  let once = (
    index_bytes(&fx, "doc_sections_data"),
    index_bytes(&fx, "src_sections_data"),
  );
  for round in 2..22 {
    rewrite(round);
    f.index_refresh(Some(&named)).expect("a refresh");
  }
  let twenty_one = (
    index_bytes(&fx, "doc_sections_data"),
    index_bytes(&fx, "src_sections_data"),
  );

  let paths = |word: &str| {
    let mut found: Vec<String> = f
      .search_all(word, &SearchQuery::default())
      .expect("the search answered")
      .groups
      .into_iter()
      .find(|g| g.tier == Tier::Lexical)
      .expect("the lexical tier")
      .hits
      .into_iter()
      .map(|h| h.path)
      .collect();
    found.sort();
    found.dedup();
    found
  };
  assert_eq!(
    paths("wallaby"),
    vec!["docs/notes.md", "src/lib.rs"],
    "the last round's words are found in both files"
  );
  assert!(
    paths("quokka").is_empty(),
    "the replaced words are gone: {:?}",
    paths("quokka")
  );
  assert_eq!(
    paths("wombats"),
    vec!["docs/guide.md"],
    "a file no refresh named still answers"
  );
  assert!(
    twenty_one.0 <= once.0 * 2 && twenty_one.1 <= once.1 * 2,
    "twenty-one refreshes of two files left the prose and source indexes {twenty_one:?} bytes wide where one \
     leaves {once:?} -- the scoped delete is tombstoning the old terms"
  );
}
