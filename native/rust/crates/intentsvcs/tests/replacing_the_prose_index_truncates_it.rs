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
use intentsvcs::store::Store;

/// The size of the inverted index, in bytes of stored segment data.
///
/// Through a fresh `rusqlite` connection rather than the facade: this is a
/// shadow table FTS5 owns, and the store deliberately exposes no reader for it.
fn index_bytes(fx: &Fixture) -> i64 {
  rusqlite::Connection::open(fx.project().db_path())
    .expect("open the store")
    .query_row(
      "SELECT coalesce(sum(length(block)), 0) FROM doc_sections_data",
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
  let once = index_bytes(&fx);
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
  let twenty_one = index_bytes(&fx);

  assert!(
    twenty_one <= once * 2,
    "twenty-one identical writes left an index {twenty_one} bytes wide where one write leaves {once} -- \
     the replacement is tombstoning the old terms instead of truncating them"
  );
}
