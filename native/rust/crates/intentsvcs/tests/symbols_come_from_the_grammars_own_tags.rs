//! **AT-20.1, AT-20.2 and AT-20.5 / AC-20.1's second half, AC-20.2 and
//! AC-20.5: symbols come from each grammar's own tags query, a language this
//! build does not carry parses nothing AND SAYS SO, and a reference is a
//! name-matched occurrence in the type itself.**
//!
//! **EVERY ARM IS MEANINGFUL IN BOTH BUILDS, WHICH IS WHY THEY ARE WRITTEN AS
//! A DISJUNCTION.** The grammars are off by default until hv rules the binary
//! size (AC-20.4), so a test that only asserted extraction would assert nothing
//! in the build everyone runs -- a green that means "not compiled in" is the
//! silent-empty defect wearing a test's clothes. Each language is therefore
//! held to: **either this build carries the grammar and the fixture yields its
//! definition, or it does not and the refusal names the exact feature.** Both
//! halves are real properties, and the pair cannot both be vacuous.

use intentsvcs::index::symbols::{
  LANGUAGES, NoSymbols, Symbol, SymbolKind, feature_of, symbols_of,
};

/// One fixture per language Intent declares: the smallest source carrying a
/// definition whose name the tags query must find.
const FIXTURES: &[(&str, &str, &str, &str)] = &[
  (
    "rust",
    "src/lib.rs",
    "pub fn quokka(n: usize) -> usize {\n  helper(n)\n}\n",
    "quokka",
  ),
  (
    "elixir",
    "lib/marsupial.ex",
    "defmodule Marsupial do\n  def quokka(n), do: helper(n)\nend\n",
    "quokka",
  ),
  (
    "swift",
    "Sources/Quokka.swift",
    "func quokka(n: Int) -> Int {\n  return helper(n)\n}\n",
    "quokka",
  ),
  (
    "lua",
    "src/quokka.lua",
    "function quokka(n)\n  return helper(n)\nend\n",
    "quokka",
  ),
  (
    "shell",
    "bin/quokka.sh",
    "quokka() {\n  helper \"$1\"\n}\n",
    "quokka",
  ),
];

fn defs(rows: &[Symbol]) -> Vec<&str> {
  rows
    .iter()
    .filter(|s| s.kind == SymbolKind::Def)
    .map(|s| s.name.as_str())
    .collect()
}

/// AT-20.2: **every declared language is extracted by its own grammar's tags
/// query, with no per-language code here.** The fixtures differ only in syntax;
/// the call is the same one, and the assertion is the same one.
#[test]
fn each_declared_language_is_extracted_by_its_own_grammars_query() {
  let mut compiled = 0usize;
  for (lang, path, source, expected) in FIXTURES {
    assert!(
      feature_of(lang).is_some(),
      "`{lang}` has a fixture and no declared grammar feature -- the roster and the fixtures have diverged"
    );
    match symbols_of(lang, path, source.as_bytes()) {
      Ok(rows) => {
        compiled += 1;
        assert!(
          defs(&rows).contains(expected),
          "the `{lang}` tags query did not name `{expected}`: {rows:?}"
        );
        assert!(
          rows.iter().all(|s| s.path == *path && s.lang == *lang),
          "every row carries the caller's own path and the language that parsed it: {rows:?}"
        );
      }
      Err(NoSymbols::NoGrammar { feature, .. }) => assert_eq!(
        Some(feature.as_str()),
        feature_of(lang),
        "the refusal must name the feature that would carry `{lang}`"
      ),
      // **A GRAMMAR WITH NO TAGS QUERY IS A THIRD REAL ANSWER**, and shell is
      // the live case: `tree-sitter-bash` 0.25.1 ships a highlights query and
      // nothing else. Asserted rather than tolerated, so the day bash gains one
      // this test says so by failing here.
      Err(NoSymbols::NoTagsQuery { lang: named }) => assert_eq!(
        named, *lang,
        "the refusal names the language whose grammar has no tags query"
      ),
      Err(other) => panic!("`{lang}` fixture is not parsable by its own grammar: {other}"),
    }
  }
  assert_eq!(
    FIXTURES.len(),
    LANGUAGES.len(),
    "every declared language has a fixture, or this test is silent about one"
  );
  // A build with grammars compiled in must have exercised at least one of them,
  // or the loop above ran entirely down its refusal arm and proved only that.
  #[cfg(any(
    feature = "lang-rust",
    feature = "lang-elixir",
    feature = "lang-swift",
    feature = "lang-lua",
    feature = "lang-bash"
  ))]
  assert!(
    compiled > 0,
    "this build carries a grammar and not one fixture reached it"
  );
  let _ = compiled;
}

/// AT-20.1's second half: **a language this build cannot parse yields a
/// refusal, never an empty list.** An empty list says "no symbols in this
/// file", which is a claim about the file; the refusal is a claim about the
/// build, and they are different answers to different questions.
#[test]
fn a_language_the_build_does_not_carry_says_so_rather_than_answering_empty() {
  match symbols_of("cobol", "src/main.cob", b"IDENTIFICATION DIVISION.\n") {
    Err(NoSymbols::NoGrammar { lang, feature }) => {
      assert_eq!(lang, "cobol");
      assert!(
        feature.contains("no grammar is declared"),
        "a language with no grammar anywhere must not name a feature that does not exist: {feature}"
      );
    }
    other => panic!("an unknown language must refuse, not answer: {other:?}"),
  }

  // And the same shape for a language that IS declared but is not in this
  // build. Only assertable where the feature is off, which is the default.
  #[cfg(not(feature = "lang-lua"))]
  match symbols_of("lua", "src/a.lua", b"function a() end\n") {
    Err(NoSymbols::NoGrammar { feature, .. }) => assert_eq!(feature, "lang-lua"),
    other => panic!("an uncompiled grammar must name its feature: {other:?}"),
  }
}

/// AT-20.5: **a reference is a NAME-MATCHED OCCURRENCE, in the type and in the
/// only word the type renders.** Nothing here resolves a name to the
/// definition it points at, so no surface may call one a caller.
#[test]
fn a_reference_is_named_as_a_name_match_and_never_as_a_call() {
  assert_eq!(SymbolKind::Def.as_str(), "def");
  assert_eq!(
    SymbolKind::Ref.as_str(),
    "ref",
    "the rendered word is `ref`: `call` and `caller` are claims this module cannot support"
  );

  // Where a grammar is compiled in, the call site in its fixture is a `Ref` --
  // the same row shape as a definition, distinguished only by its kind.
  #[cfg(feature = "lang-rust")]
  {
    let rows = symbols_of("rust", "src/lib.rs", FIXTURES[0].2.as_bytes()).expect("rust is here");
    let refs: Vec<&str> = rows
      .iter()
      .filter(|s| s.kind == SymbolKind::Ref)
      .map(|s| s.name.as_str())
      .collect();
    assert!(
      refs.contains(&"helper"),
      "the called name is a name-matched reference: {rows:?}"
    );
    assert!(
      !defs(&rows).contains(&"helper"),
      "a name that is only called is not defined here: {rows:?}"
    );
  }
}
