//! **SYMBOLS COME FROM EACH GRAMMAR'S OWN `tags.scm`, AND NOTHING
//! PER-LANGUAGE IS WRITTEN HERE** (AC-20.2).
//!
//! Every maintained tree-sitter grammar ships a tagging query naming its
//! definitions and references -- the convention `ctags`-style tools already
//! consume. Reading it means **adding a language is a grammar entry and
//! nothing else**: no extractor of ours, no capture list, no per-language
//! branch for the next person to forget. The alternative, an extractor per
//! syntax, is five parsers to keep correct against five evolving languages,
//! and the first to drift reports symbols that are not there -- which is worse
//! than no structural tier at all, because an agent cannot tell.
//!
//! **PURE OVER BYTES.** No store, no disk, no clock: the caller hands over the
//! bytes it already has and gets rows back. So the whole module is driven from
//! fixture strings, and the walk that decides WHICH files to hand over lives
//! where the corpus does.
//!
//! **A `Ref` IS A NAME-MATCHED OCCURRENCE AND THE TYPE SAYS SO** (AC-20.5).
//! Without type resolution nothing here can say *caller*, and a surface that
//! said it would be the confident wrong answer this estate refuses. The word
//! travels with the type rather than being remembered at each rendering.

/// One symbol: a definition or a name-matched reference, and where it sits.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Symbol {
  /// Project-relative, as the caller named it.
  pub path: String,
  /// The language whose grammar produced this row.
  pub lang: &'static str,
  pub name: String,
  pub kind: SymbolKind,
  pub span: Span,
}

/// **TWO VALUES, AND IT IS AN ENUM RATHER THAN A STRING BECAUSE THE SECOND ONE
/// IS A CLAIM THAT IS EASY TO OVERSTATE.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SymbolKind {
  /// A definition: the grammar's `@definition.*` captures.
  Def,
  /// **A NAME-MATCHED OCCURRENCE.** The grammar's `@reference.*` captures say
  /// this identifier appears here; nothing in this module resolves it to the
  /// definition it names, so it is never a *call* and never a *caller*.
  Ref,
}

impl SymbolKind {
  pub fn as_str(self) -> &'static str {
    match self {
      SymbolKind::Def => "def",
      SymbolKind::Ref => "ref",
    }
  }
}

/// A 1-indexed, inclusive line range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct Span {
  pub start_line: u32,
  pub end_line: u32,
}

/// Why a parse produced nothing.
///
/// **AN ABSENT ANSWER IS NEVER AN EMPTY ONE** (`IN-AG-NO-SILENT-001`, and the
/// estate's dominant defect class). A file with no symbols and a language this
/// binary cannot parse both yield no rows, and a caller that cannot tell them
/// apart reports "nothing here" for a grammar that was never compiled in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NoSymbols {
  /// This build carries no grammar for that language. The feature naming it is
  /// in the message, because the fix is a build flag and nothing else.
  NoGrammar { lang: String, feature: String },
  /// The grammar is here and the bytes would not parse as that language.
  Unparsable { lang: String },
  /// **THE GRAMMAR IS COMPILED IN AND SHIPS NO TAGS QUERY**, so there is
  /// nothing for this module to run and no symbols to return.
  ///
  /// It is its own variant because the three answers send a reader to three
  /// different places: a build flag, a broken file, and **an upstream grammar
  /// that has not written a tags query** -- which nobody here can fix and which
  /// no amount of re-running changes. `tree-sitter-bash` 0.25.1 is the live
  /// case: it ships `HIGHLIGHT_QUERY` alone. Writing the query ourselves is the
  /// per-language extractor this module exists to avoid, and it would drift
  /// against bash's syntax the first time either moved.
  NoTagsQuery { lang: String },
}

impl std::fmt::Display for NoSymbols {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      NoSymbols::NoGrammar { lang, feature } => write!(
        f,
        "this build carries no `{lang}` grammar -- it is behind the `{feature}` feature"
      ),
      NoSymbols::Unparsable { lang } => {
        write!(f, "these bytes do not parse as `{lang}`")
      }
      NoSymbols::NoTagsQuery { lang } => write!(
        f,
        "the `{lang}` grammar ships no tags query, so it names no symbols"
      ),
    }
  }
}

/// Every language this module knows a NAME for, and the feature that carries
/// its grammar.
///
/// **THE ROSTER IS DECLARED ONCE AND IS NOT CONDITIONAL.** A `#[cfg]`-gated
/// list would make an uncompiled language indistinguishable from an unknown
/// one, and those are different answers: one is a build flag, the other is a
/// language nothing supports. The `cfg` lives at the parse, where the grammar
/// actually is.
pub const LANGUAGES: &[(&str, &str)] = &[
  ("rust", "lang-rust"),
  ("elixir", "lang-elixir"),
  ("swift", "lang-swift"),
  ("lua", "lang-lua"),
  // `shell` is the name a project's `languages` array carries; bash is the
  // grammar, and the shell rules are written against bash's syntax.
  ("shell", "lang-bash"),
];

/// The feature carrying a language's grammar, or `None` for a language no
/// grammar is declared for at all.
pub fn feature_of(lang: &str) -> Option<&'static str> {
  LANGUAGES
    .iter()
    .find(|(name, _)| *name == lang)
    .map(|(_, feature)| *feature)
}

/// What this build can do for a language, asked WITHOUT parsing anything.
///
/// **THE THREE ANSWERS SEND A READER TO THREE DIFFERENT PLACES**, which is
/// [`NoSymbols`]'s reason restated as a property of the build rather than of a
/// file: a build flag, an upstream grammar that has not written a tags query,
/// and a language nothing here supports. `intent index status` reports it per
/// declared language, so a project whose `shell` files name no symbols reads
/// why instead of reading nothing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Readiness {
  /// A grammar is compiled in and it ships a tags query.
  Ready,
  /// The roster names a grammar for this language and this build does not
  /// carry it. The feature is the whole of the fix.
  NoGrammar { feature: &'static str },
  /// The grammar is compiled in and ships no tags query, so it names nothing.
  /// `tree-sitter-bash` is the live case and nobody here writes the query.
  NoTagsQuery,
  /// No grammar is declared for this language at all.
  Unknown,
}

impl Readiness {
  /// The stored and reported spelling.
  pub fn as_str(&self) -> &'static str {
    match self {
      Readiness::Ready => "ready",
      Readiness::NoGrammar { .. } => "no-grammar",
      Readiness::NoTagsQuery => "no-tags-query",
      Readiness::Unknown => "unknown",
    }
  }
}

/// What this build can do for a language.
pub fn readiness(lang: &str) -> Readiness {
  match grammar(lang) {
    Some((_, _, Some(_))) => Readiness::Ready,
    Some((_, _, None)) => Readiness::NoTagsQuery,
    None => match feature_of(lang) {
      Some(feature) => Readiness::NoGrammar { feature },
      None => Readiness::Unknown,
    },
  }
}

/// The symbols in `bytes`, as the grammar for `lang` tags them.
///
/// `path` is carried onto every row untouched: this module never interprets a
/// path, and the caller's spelling is the one every surface shows.
pub fn symbols_of(lang: &str, path: &str, bytes: &[u8]) -> Result<Vec<Symbol>, NoSymbols> {
  let Some((lang, language, tags)) = grammar(lang) else {
    return Err(match feature_of(lang) {
      Some(feature) => NoSymbols::NoGrammar {
        lang: lang.to_string(),
        feature: feature.to_string(),
      },
      // A language with no grammar declared anywhere is reported the same way
      // rather than specially: the caller's remedy is still "this build cannot
      // parse that", and inventing a feature name for it would name a flag
      // that does not exist.
      None => NoSymbols::NoGrammar {
        lang: lang.to_string(),
        feature: "none -- no grammar is declared for that language".to_string(),
      },
    });
  };

  let mut parser = tree_sitter::Parser::new();
  if parser.set_language(&language).is_err() {
    return Err(NoSymbols::Unparsable {
      lang: lang.to_string(),
    });
  }
  let Some(tree) = parser.parse(bytes, None) else {
    return Err(NoSymbols::Unparsable {
      lang: lang.to_string(),
    });
  };

  // **THE QUERY IS THE GRAMMAR'S OWN, AND ITS ABSENCE IS REPORTED RATHER THAN
  // ANSWERED EMPTY.** A grammar with no tags query is a fact about the grammar,
  // not about the file, and a caller told "no symbols" would record that the
  // file has none.
  let Some(tags) = tags else {
    return Err(NoSymbols::NoTagsQuery {
      lang: lang.to_string(),
    });
  };
  let Ok(query) = tree_sitter::Query::new(&language, tags) else {
    return Err(NoSymbols::NoTagsQuery {
      lang: lang.to_string(),
    });
  };
  let mut cursor = tree_sitter::QueryCursor::new();
  let names = query.capture_names();
  let mut out = Vec::new();
  {
    use tree_sitter::StreamingIterator;
    let mut matches = cursor.matches(&query, tree.root_node(), bytes);
    // **ONE SYMBOL PER MATCH, AND THE NAME COMES FROM THE MATCH'S `@name`
    // CAPTURE.** This is the tags convention and getting it wrong is silent:
    // `@definition.function` captures the WHOLE definition node, so reading the
    // name off it yields `pub fn quokka(n: usize) -> usize {` -- driven, not
    // reasoned about. The identifier is a separate capture in the SAME match,
    // which is why the iteration is over matches rather than over the flattened
    // captures the first draft walked.
    while let Some(m) = matches.next() {
      let mut kind = None;
      let mut span_node = None;
      let mut name_node = None;
      for capture in m.captures() {
        let capture_name = names[capture.index as usize];
        // **THE PREFIX IS THE CONVENTION AND ANYTHING ELSE IS SKIPPED RATHER
        // THAN GUESSED.** Grammars also carry `@doc`, `@local.scope` and
        // friends; a mapping that fell through to `Ref` would report every doc
        // comment as an occurrence.
        if capture_name.starts_with("definition.") {
          kind = Some(SymbolKind::Def);
          span_node = Some(capture.node);
        } else if capture_name.starts_with("reference.") {
          kind = Some(SymbolKind::Ref);
          span_node = Some(capture.node);
        } else if capture_name == "name" {
          name_node = Some(capture.node);
        }
      }
      let (Some(kind), Some(span_node)) = (kind, span_node) else {
        continue;
      };
      // **NO `@name` MEANS NO ROW.** A symbol whose name is the whole
      // declaration is not a symbol anybody can search for, and naming it after
      // its own source text would put syntax into the index.
      let Some(name_node) = name_node else {
        continue;
      };
      let Ok(name) = name_node.utf8_text(bytes) else {
        continue;
      };
      if name.is_empty() {
        continue;
      }
      out.push(Symbol {
        path: path.to_string(),
        lang,
        name: name.to_string(),
        kind,
        span: Span {
          start_line: span_node.start_position().row as u32 + 1,
          end_line: span_node.end_position().row as u32 + 1,
        },
      });
    }
  }
  // Path order within a file: the order a reader would meet them.
  out.sort_by_key(|s| (s.span.start_line, s.span.end_line, s.name.clone()));
  Ok(out)
}

/// The compiled grammar for a language, when this build carries one.
///
/// **THIS IS THE ONLY `cfg` IN THE MODULE**, and it is here rather than around
/// the roster so that "no grammar compiled in" stays distinguishable from "no
/// such language".
fn grammar(lang: &str) -> Option<(&'static str, tree_sitter::Language, Option<&'static str>)> {
  match lang {
    #[cfg(feature = "lang-rust")]
    "rust" => Some((
      "rust",
      tree_sitter_rust::LANGUAGE.into(),
      Some(tree_sitter_rust::TAGS_QUERY),
    )),
    #[cfg(feature = "lang-elixir")]
    "elixir" => Some((
      "elixir",
      tree_sitter_elixir::LANGUAGE.into(),
      Some(tree_sitter_elixir::TAGS_QUERY),
    )),
    #[cfg(feature = "lang-swift")]
    "swift" => Some((
      "swift",
      tree_sitter_swift::LANGUAGE.into(),
      Some(tree_sitter_swift::TAGS_QUERY),
    )),
    #[cfg(feature = "lang-lua")]
    "lua" => Some((
      "lua",
      tree_sitter_lua::LANGUAGE.into(),
      Some(tree_sitter_lua::TAGS_QUERY),
    )),
    #[cfg(feature = "lang-bash")]
    // **`tree-sitter-bash` SHIPS NO TAGS QUERY** (0.25.1: `HIGHLIGHT_QUERY`
    // alone), so the grammar parses and names nothing. Declared here anyway,
    // with the absence stated, rather than left out: a language missing from
    // this match is one this build has no grammar for, and that is a different
    // fact with a different fix.
    "shell" => Some(("shell", tree_sitter_bash::LANGUAGE.into(), None)),
    _ => None,
  }
}
