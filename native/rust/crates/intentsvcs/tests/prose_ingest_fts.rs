//! AT-03.6 / AC-03.6: prose bodies ingest verbatim into FTS-indexed doc
//! sections; a body round-trips byte-identical out of the store and is
//! retrievable by full-text query.
//!
//! AC-03.6 was added at WP-03 by vc after a grep of all 62 ACs for `fts|search`
//! returned nothing: `intent search` is a design.md deliverable with no v2
//! antecedent, so WP-06's parity suite could never have covered it either.
//! WP-03's own deliverable said "prose ingested verbatim (FTS-indexed)" with
//! nothing testing either half.
//!
//! "Verbatim" is the word under test. Prose is stored, never modelled, so the
//! only honest proof is that what comes out is byte-identical to what went in
//! -- including the blank lines, the trailing newline (or its absence), the
//! fenced code blocks, and the `#` characters that are content rather than
//! headings.

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::ingest;
use intentsvcs::prose;
use intentsvcs::store::Store;

const DESIGN: &str = "\
Preamble above any heading.

# Design

The truth model is schema-as-truth.

## Ingest

Strict: validate or refuse by name.

```sh
# this hash is content, not a heading
intent ingest --from-md
```

## Views

Deterministic and idempotent.
";

#[test]
fn a_prose_body_round_trips_byte_identical() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  fx.write_prose("ST0056", "design.md", DESIGN);

  let mut store = Store::open_in_memory().expect("open");
  ingest::load(&fx.project(), &mut store).expect("load");

  let sections = store
    .doc_sections_for("intent/st/ST0056/design.md")
    .expect("query sections");
  assert!(!sections.is_empty(), "the body was indexed");
  assert_eq!(
    prose::join(&sections),
    DESIGN,
    "prose is stored verbatim -- reassembling the sections reproduces the file byte for byte"
  );
}

#[test]
fn a_fenced_hash_does_not_split_a_section() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  fx.write_prose("ST0056", "design.md", DESIGN);

  let mut store = Store::open_in_memory().expect("open");
  ingest::load(&fx.project(), &mut store).expect("load");

  let headings: Vec<String> = store
    .doc_sections_for("intent/st/ST0056/design.md")
    .expect("query")
    .into_iter()
    .filter_map(|s| s.heading)
    .collect();
  assert_eq!(
    headings,
    vec!["Design", "Ingest", "Views"],
    "the `# this hash is content` line inside the fence is not a heading"
  );
}

#[test]
fn a_body_is_retrievable_by_full_text_query() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  fx.write_prose("ST0056", "design.md", DESIGN);
  fx.write_prose(
    "ST0056",
    "impl.md",
    "# Implementation\n\nNothing built yet.\n",
  );

  let mut store = Store::open_in_memory().expect("open");
  ingest::load(&fx.project(), &mut store).expect("load");

  let hits = store.search("idempotent").expect("search");
  assert_eq!(hits.len(), 1, "one section matches: {hits:?}");
  assert_eq!(hits[0].file, "intent/st/ST0056/design.md");
  assert_eq!(hits[0].heading.as_deref(), Some("Views"));
  assert_eq!(hits[0].owner_type, "thread");
  assert_eq!(hits[0].owner_id, "ST0056");

  // Stemming is on (porter), so the query need not match the surface form.
  assert_eq!(
    store.search("validate").expect("search").len(),
    1,
    "the porter tokenizer is configured, so `validate` finds `validate`/`validates`"
  );
  assert!(
    store.search("nonexistentword").expect("search").is_empty(),
    "a miss is empty, not everything"
  );
}

#[test]
fn a_heading_is_searchable_as_well_as_a_body() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  fx.write_prose("ST0056", "design.md", DESIGN);

  let mut store = Store::open_in_memory().expect("open");
  ingest::load(&fx.project(), &mut store).expect("load");

  let hits = store.search("Ingest").expect("search");
  assert!(
    hits.iter().any(|h| h.heading.as_deref() == Some("Ingest")),
    "headings are indexed, not only bodies: {hits:?}"
  );
}

/// Generated views are deliberately NOT indexed. Indexing them would index the
/// model twice and let a stale view answer a search -- the exact confusion the
/// authored-once principle exists to remove.
#[test]
fn generated_views_are_not_indexed_as_prose() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  fx.write_prose("ST0056", "design.md", DESIGN);
  fx.write_prose(
    "ST0056",
    "info.md",
    "# ST0056\n\nA stale generated cover.\n",
  );

  let mut store = Store::open_in_memory().expect("open");
  ingest::load(&fx.project(), &mut store).expect("load");

  assert!(
    store
      .doc_sections_for("intent/st/ST0056/info.md")
      .expect("query")
      .is_empty(),
    "info.md is a generated view; a search must never answer from it"
  );
  assert!(
    store.search("stale").expect("search").is_empty(),
    "content that exists only in a generated view is not searchable"
  );
}

#[test]
fn an_issue_body_is_indexed_against_its_issue() {
  let fx = Fixture::new();
  fx.write_issue(&common::sample_issue(21));
  fx.write_file(
    "intent/issues/0021.md",
    "# 0021\n\nThe credo_checks mechanism was a second enforcement path.\n",
  );

  let mut store = Store::open_in_memory().expect("open");
  ingest::load(&fx.project(), &mut store).expect("load");

  let hits = store.search("enforcement").expect("search");
  assert_eq!(hits.len(), 1);
  assert_eq!(hits[0].owner_type, "issue");
  assert_eq!(hits[0].owner_id, "21");
}
