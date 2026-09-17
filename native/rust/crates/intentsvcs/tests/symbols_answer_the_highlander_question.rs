//! AC-20.3 and AC-20.1's first half: `--kind def <name>` is answered from the
//! store, and a declared language's file names its symbols.
//!
//! **THE TWO ARMS ARE DELIBERATELY DRIVEN AT DIFFERENT DEPTHS**, because the
//! two claims are different. Whether the store answers the Highlander question
//! is true of every build; whether a file NAMES symbols depends on a grammar
//! being compiled in, and every `lang-*` feature is off until hv rules the
//! binary-size line. So the first arm plants rows and asks the question, and
//! the second is gated on the feature and runs in a build that carries the
//! grammar.

use crate::common;

use common::Fixture;
use intentsvcs::index::symbols::{Span, Symbol, SymbolKind};
use intentsvcs::search::{HitKind, SearchQuery, Tier};
use intentsvcs::store::Store;

/// AT-20.3.
#[test]
fn a_definition_is_answered_by_name_from_the_store() {
  let fx = Fixture::new();
  std::fs::write(fx.root().join("lib.rs"), "fn assemble_widget() {}\n").expect("write");

  {
    let mut store = Store::open(&fx.project().db_path()).expect("store");
    store
      .replace_symbols_for(
        &["lib.rs".to_string()],
        &[
          Symbol {
            path: "lib.rs".to_string(),
            lang: "rust",
            name: "assemble_widget".to_string(),
            kind: SymbolKind::Def,
            span: Span {
              start_line: 1,
              end_line: 1,
            },
            subkind: "function".to_string(),
            container: None,
            container_kind: None,
            trait_name: None,
            arity: Some(0),
            arity_min: Some(0),
            qualifier: None,
            level: 1,
          },
          Symbol {
            path: "lib.rs".to_string(),
            lang: "rust",
            name: "assemble_widget".to_string(),
            kind: SymbolKind::Ref,
            span: Span {
              start_line: 1,
              end_line: 1,
            },
            subkind: "call".to_string(),
            container: None,
            container_kind: None,
            trait_name: None,
            arity: Some(0),
            arity_min: Some(0),
            qualifier: None,
            level: 1,
          },
        ],
      )
      .expect("plant the rows a parse would have written");
  }

  let facade = fx.facade_on_disk();
  let answer = facade
    .search_all(
      "assemble_widget",
      &SearchQuery {
        kinds: vec![HitKind::Def],
        ..SearchQuery::default()
      },
    )
    .expect("the search answered");

  let structural = answer
    .groups
    .iter()
    .find(|g| g.tier == Tier::Structural)
    .expect("the structural tier is a group of its own");
  assert_eq!(
    structural.hits.len(),
    1,
    "`--kind def` answers with the definition and not the name-matched \
     reference beside it: {structural:?}"
  );
  let hit = &structural.hits[0];
  assert_eq!(hit.kind, HitKind::Def);
  assert_eq!(hit.name, "assemble_widget");
  assert_eq!(hit.path, "lib.rs");
  assert_eq!(hit.lang.as_deref(), Some("rust"));
  assert_eq!(
    hit.span.map(|s| s.start_line),
    Some(1),
    "the span survives because the file still holds the bytes that were parsed"
  );
  assert!(
    hit.snippet.contains("assemble_widget"),
    "and the line it names is shown: {hit:?}"
  );
}

/// AT-20.1, the first half: the hits exist for a language the project declares.
/// The second half -- a language absent from the array parses nothing -- is
/// ic's arm, and the row's note names it, because one criterion answered in two
/// files is one record or it is neither.
///
/// **GATED, BECAUSE EVERY GRAMMAR IS OFF UNTIL hv RULES THE SIZE LINE.** Run
/// with `--features lang-rust`; in a build without it there is no grammar and
/// the claim is not one this binary can make.
#[cfg(feature = "lang-rust")]
#[test]
fn a_declared_languages_file_names_its_symbols_through_the_index() {
  let fx = Fixture::new();
  let ok = std::process::Command::new("git")
    .args(["init", "-q"])
    .current_dir(fx.root())
    .status()
    .expect("run git")
    .success();
  assert!(ok, "git init failed");
  std::fs::write(fx.root().join(".gitignore"), "").expect("gitignore");
  std::fs::create_dir_all(fx.root().join("src")).expect("mkdir");
  std::fs::write(
    fx.root().join("src/lib.rs"),
    "pub fn assemble_widget() -> usize {\n  1\n}\n",
  )
  .expect("write");

  let mut facade = fx.facade_on_disk();
  facade.index_rebuild().expect("rebuild");

  let answer = facade
    .search_all(
      "assemble_widget",
      &SearchQuery {
        kinds: vec![HitKind::Def],
        ..SearchQuery::default()
      },
    )
    .expect("the search answered");
  let structural = answer
    .groups
    .iter()
    .find(|g| g.tier == Tier::Structural)
    .expect("the structural tier is a group of its own");
  assert_eq!(
    structural
      .hits
      .iter()
      .map(|h| h.path.as_str())
      .collect::<Vec<_>>(),
    vec!["src/lib.rs"],
    "the project declares rust and this build carries the grammar, so the file \
     names its definition: {answer:?}"
  );
}
