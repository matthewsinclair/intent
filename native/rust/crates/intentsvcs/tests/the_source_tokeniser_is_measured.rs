//! **AT-19.4 / AC-19.4: the source table is tokenised without stemming,
//! `snake_case` splits into its words, and the tokeniser decision carries the
//! recall that decided it.**
//!
//! # The measurement this file pins
//!
//! Driven 2026-09-12 against the real `src_sections` at `f8ed61135`, over seven
//! Intent source files (`critic.rs`, `fts.rs`, `prose.rs`, `sql_gate.rs`,
//! `nav.rs`, `form.rs`, `remedy.rs`), 157,538 bytes, indexed by `intent index
//! rebuild`. The query set was taken FROM THE CORPUS rather than invented: 68
//! `snake_case` function names and 21 CamelCase types, each with the file that
//! defines it, and recall means the defining file comes back.
//!
//! | query form                 | unicode61 | trigram |
//! | -------------------------- | --------- | ------- |
//! | whole name                 | 68/68     | 68/68   |
//! | last snake word            | 68/68     | 64/68   |
//! | first snake word           | 68/68     | 47/68   |
//! | inner fragment, 5 chars    | 10/67     | 67/67   |
//!
//! CamelCase: whole name 21/21 both; inner fragment 0/10 unicode61, 10/10
//! trigram.
//!
//! **THE CONTROL, because a mirror is worthless without one:** the in-memory
//! unicode61 table the trigram arm was measured against agrees with the REAL
//! `src_sections` on all 68 whole-name queries, path set for path set.
//!
//! **EVERY TRIGRAM MISS IS A QUERY SHORTER THAN THREE CHARACTERS, CHECKED
//! RATHER THAN ASSUMED:** all 21 first-word misses are 1 or 2 characters (`as`,
//! `a`, `no`) and all 4 last-word misses are 2 (`of`, `it`, `on`). FTS5's
//! trigram tokeniser cannot index a term shorter than three, so under trigram
//! **a one- or two-character query returns nothing at all** -- which reaches
//! real identifiers: `fs`, `os`, `db`, `id`.
//!
//! **SO THE RULING (vc, 2026-09-12) IS unicode61 WITHOUT STEMMING, AND THE
//! RECORDED ALTERNATIVE IS NOT THE ONE THE DESIGN GUESSED.** The design said
//! trigram replaces unicode61 if fragment recall is poor; the measurement says
//! trigram would cost every short query entirely and make one query mean two
//! things across the prose and source tables. Fragment search, if it is ever
//! wanted, is a second column or a tier -- a package, not a migration.
//!
//! # What the arms below hold
//!
//! Not the sweep -- a recall sweep is a MEASUREMENT and belongs in prose where
//! a reader meets its conditions. What is asserted here is the set of
//! properties the decision RESTS on, so the day the tokeniser changes under it,
//! this file says so instead of the numbers above quietly becoming fiction.

use crate::common::Fixture;

const SOURCE: &str = "pub fn parse_disabled(text: &str) -> Vec<String> {\n  \
                      let mut out = Vec::new();\n  \
                      out.push(text.to_string());\n  \
                      out\n\
                      }\n\n\
                      pub struct SeverityFilter;\n\n\
                      pub fn as_str(s: &SeverityFilter) -> &'static str {\n  \
                      let _ = s;\n  \
                      \"x\"\n\
                      }\n";

/// The paths `src_sections` returns for one FTS5 expression, read through the
/// store's own read-only door.
fn matches(facade: &intentsvcs::facade::Facade, expression: &str) -> Vec<String> {
  let conn = facade
    .store()
    .read_only_connection()
    .expect("the store has a read-only door");
  let mut stmt = conn
    .prepare("SELECT path FROM src_sections WHERE src_sections MATCH ?1 ORDER BY path")
    .expect("prepare the match");
  let rows = stmt
    .query_map([expression], |row| row.get::<_, String>(0))
    .expect("run the match");
  rows.map(|r| r.expect("a path came back")).collect()
}

fn indexed() -> (Fixture, intentsvcs::facade::Facade) {
  let fx = Fixture::new();
  std::fs::create_dir_all(fx.path("src")).expect("mkdir src");
  std::fs::write(fx.path("src/lib.rs"), SOURCE).expect("write the fixture source");
  let mut facade = fx.facade_on_disk();
  facade.index_rebuild().expect("the index was built");
  (fx, facade)
}

/// AC-19.4's first half: **`snake_case` splits into its words**, so either end
/// of a name finds it -- which is the property that makes the whole tier
/// usable without anyone learning a query language.
#[test]
fn a_snake_case_name_is_found_by_its_whole_form_and_by_either_word() {
  let (_fx, facade) = indexed();
  for query in ["parse_disabled", "disabled", "parse"] {
    assert!(
      matches(&facade, query).iter().any(|p| p == "src/lib.rs"),
      "`{query}` did not find the file defining `parse_disabled`"
    );
  }
}

/// **THE PROPERTY THE RULING TURNS ON, AND THE ONE TRIGRAM WOULD BREAK
/// SILENTLY**: a query of one or two characters is answered at all.
#[test]
fn a_two_character_query_still_answers() {
  let (_fx, facade) = indexed();
  assert!(
    matches(&facade, "as").iter().any(|p| p == "src/lib.rs"),
    "`as` is a word of `as_str` and must be reachable -- under trigram this \
     query returns nothing at all, which is why the tokeniser stayed"
  );
}

/// **THE MEASURED COST, PINNED SO IT CANNOT CHANGE IN SILENCE.** An inner
/// fragment does NOT match, and that is the trade the ruling accepted. If a
/// later tokeniser makes this pass, the numbers in this file's header stop
/// describing the build and the header is what must change with it.
#[test]
fn an_inner_fragment_does_not_match_and_the_prefix_form_does() {
  let (_fx, facade) = indexed();
  assert!(
    matches(&facade, "isabl").is_empty(),
    "unicode61 indexes whole words, so an inner fragment matches nothing -- \
     measured at 10/67 on snake names and 0/10 on CamelCase"
  );
  // The remedy for a fragment, and it is exact rather than fuzzy: the prefix
  // form the escaper already preserves.
  assert!(
    matches(&facade, "Severity*")
      .iter()
      .any(|p| p == "src/lib.rs"),
    "the prefix form is what reaches a CamelCase name, and it is the remedy \
     this trade rests on"
  );
}
