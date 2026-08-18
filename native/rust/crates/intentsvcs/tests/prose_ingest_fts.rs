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
//! **THE VEHICLE IS ISSUE BODIES, AND IT USED TO BE `design.md`** (2026-08-18).
//! D57-6 deleted `THREAD_PROSE`, so a thread's `design.md` is an attachment
//! carried verbatim and no longer split into sections -- which took these
//! tests out with it.
//!
//! **They were re-pointed rather than rewritten, and the distinction is the
//! one that decides whether a green is honest.** AC-03.6 says "prose bodies"
//! and names no file, no entity and no route: `design.md` was this TEST's
//! chosen fixture, never the CRITERION's subject. Issue bodies are prose
//! bodies, still route through `prose::split`, and exercise every property
//! this file asserts. **The criterion is unchanged and still covered.**
//!
//! The contrast is AC-06.4, which NAMES ST prose as one of three sources. Its
//! vehicle dying makes it FALSE, so its tests stay red and are not re-pointed.
//! **The test each time: does the criterion's own text name the thing that
//! died?** If it does, a re-point would be hiding a real defect.
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
  let mut issue = common::sample_issue(56);
  issue.body = DESIGN.to_string();
  fx.write_issue(&issue);

  let mut store = Store::open_in_memory().expect("open");
  ingest::load(&fx.project(), &mut store).expect("load");

  let sections = store
    .doc_sections_for("intent/issues/0056.json")
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
  let mut issue = common::sample_issue(56);
  issue.body = DESIGN.to_string();
  fx.write_issue(&issue);

  let mut store = Store::open_in_memory().expect("open");
  ingest::load(&fx.project(), &mut store).expect("load");

  let headings: Vec<String> = store
    .doc_sections_for("intent/issues/0056.json")
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
  let mut issue = common::sample_issue(56);
  issue.body = DESIGN.to_string();
  fx.write_issue(&issue);
  let mut other = common::sample_issue(57);
  other.body = "# Implementation\n\nNothing built yet.\n".to_string();
  fx.write_issue(&other);

  let mut store = Store::open_in_memory().expect("open");
  ingest::load(&fx.project(), &mut store).expect("load");

  let hits = store.search("idempotent").expect("search");
  assert_eq!(hits.len(), 1, "one section matches: {hits:?}");
  assert_eq!(hits[0].file, "intent/issues/0056.json");
  assert_eq!(hits[0].heading.as_deref(), Some("Views"));
  assert_eq!(hits[0].owner_type, "issue");
  assert_eq!(hits[0].owner_id, "56");

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
  let mut issue = common::sample_issue(56);
  issue.body = DESIGN.to_string();
  fx.write_issue(&issue);

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

/// **The body is indexed from the FIELD, and the sibling file is not a second
/// route to the same place.**
///
/// This wrote `intent/issues/0021.md` and searched for a word in it. No such
/// file was ever produced by anything -- the model had no body field, so the
/// migration had nowhere to put one -- which made the test a demonstration
/// that a file nothing writes is indexed when hand-written.
#[test]
fn an_issue_body_is_indexed_against_its_issue() {
  let fx = Fixture::new();
  let mut issue = common::sample_issue(21);
  issue.body = "# 0021\n\nThe credo_checks mechanism was a second enforcement path.\n".to_string();
  fx.write_issue(&issue);

  let mut store = Store::open_in_memory().expect("open");
  ingest::load(&fx.project(), &mut store).expect("load");

  let hits = store.search("enforcement").expect("search");
  assert_eq!(hits.len(), 1);
  assert_eq!(hits[0].owner_type, "issue");
  assert_eq!(hits[0].owner_id, "21");
  assert_eq!(
    hits[0].file, "intent/issues/0021.json",
    "the hit addresses the canon that holds the prose, not a markdown file \
     beside it -- under disk-optional there may be no such file to name"
  );
}

/// The counter-arm: a body-shaped file beside the canon is NOT a second home.
///
/// Without this, the test above passes whether the field is indexed or the old
/// file branch is -- the two are indistinguishable when the fixture writes
/// both, which is how the removed branch survived having no producer.
#[test]
fn a_markdown_file_beside_the_issue_canon_is_not_indexed() {
  let fx = Fixture::new();
  let mut issue = common::sample_issue(21);
  issue.body = "# 0021\n\nThe modelled prose.\n".to_string();
  fx.write_issue(&issue);
  fx.write_file("intent/issues/0021.md", "# 0021\n\nA quokka wrote this.\n");

  let mut store = Store::open_in_memory().expect("open");
  ingest::load(&fx.project(), &mut store).expect("load");

  assert!(
    store.search("quokka").expect("search").is_empty(),
    "a file beside the canon is not canon: an issue's prose has one home"
  );
  assert_eq!(
    store.search("modelled").expect("search").len(),
    1,
    "and the one home is still indexed"
  );
}

/// **A heading that appears TWICE in one file yields TWO sections, and neither
/// is collapsed into the other.**
///
/// Asked by vc, who found the class in their own census: `section_text` cuts by
/// NAME, so two sections sharing a heading cannot both be addressed there, and
/// whether the same limit reached ingest was NOT established. It does not: the
/// splitter keys on POSITION (`seq`) and the heading is a label, so a duplicate
/// is two addressable sections that happen to be called the same thing.
///
/// **The estate has exactly two such files** -- `ST0026/impl.md` carries
/// `## Test Status` at lines 262 and 275, and one issue carries `## Related`
/// twice -- so this is a bounded class rather than a worry, and it is pinned
/// here rather than argued from a reading of the splitter.
#[test]
fn a_heading_repeated_in_one_file_yields_two_sections_rather_than_one() {
  let fx = Fixture::new();
  let mut issue = common::sample_issue(26);
  issue.body = "# Impl\n\n## Test Status\n\nThe first pass, red.\n\n## Notes\n\nBetween.\n\n## Test Status\n\nThe second pass, green.\n".to_string();
  fx.write_issue(&issue);

  let mut store = Store::open_in_memory().expect("open");
  ingest::load(&fx.project(), &mut store).expect("load");

  let sections = store
    .doc_sections_for("intent/issues/0026.json")
    .expect("query");
  let repeated: Vec<&str> = sections
    .iter()
    .filter(|s| s.heading.as_deref() == Some("Test Status"))
    .map(|s| s.body.trim())
    .collect();
  assert_eq!(
    repeated,
    vec!["The first pass, red.", "The second pass, green."],
    "both survive, in document order, with their own bodies -- a name-keyed \
     reader would have kept one and lost the other with nothing reporting it"
  );
}
