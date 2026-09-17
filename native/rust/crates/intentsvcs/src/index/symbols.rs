//! **SYMBOLS COME FROM A QUERY PER GRAMMAR, AND NOTHING PER-LANGUAGE IS
//! WRITTEN IN RUST HERE.**
//!
//! Every maintained tree-sitter grammar ships a tagging query naming its
//! definitions and references -- the convention `ctags`-style tools already
//! consume -- and Swift and Lua are read through theirs. **Rust and Elixir are
//! read through Intent's own** (`queries/rust.scm`, `queries/elixir.scm`), on
//! hv's ruling of 2026-09-16 that the index is typed (ST0076, reversing AC-20.2
//! for those two): a definition's kind, its container and its arity are in no
//! grammar's tags query. What stays true is the split: **everything a language
//! knows is in its query file, in one capture vocabulary this module reads the
//! same way for every grammar**, so there is still no per-language branch here
//! for the next person to forget.
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
  /// What the row IS, in the language's own vocabulary (ST0076's design):
  /// `method`, `struct`, `defp`, `call`. Empty only on a row written before
  /// the schema carried it.
  pub subkind: String,
  /// The enclosing impl, trait, type or module, named from the file's own
  /// syntax; `None` at the top of a file.
  pub container: Option<String>,
  /// What the container is: `impl`, `trait`, `module`, `struct` and so on.
  pub container_kind: Option<String>,
  /// The trait an enclosing `impl Trait for Type` implements.
  pub trait_name: Option<String>,
  /// The parameter count, where the language has one.
  pub arity: Option<u32>,
  /// The count without the parameters that carry a default; equal to `arity`
  /// where the language has no defaults.
  pub arity_min: Option<u32>,
  /// The path a qualified reference was written with (level 2).
  pub qualifier: Option<String>,
  /// Where the answer came from: 1 a definition or an unqualified syntax
  /// reference, 2 a qualified syntax reference, 3 resolved.
  pub level: u8,
}

/// **TWO VALUES, AND IT IS AN ENUM RATHER THAN A STRING BECAUSE THE SECOND ONE
/// IS A CLAIM THAT IS EASY TO OVERSTATE.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
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
      // Issue 0359: no grammar is declared for it, which is `no-grammar`;
      // `unknown` read as a lookup that failed.
      Readiness::Unknown => "no-grammar",
    }
  }
}

/// What this build can do for a language.
pub fn readiness(lang: &str) -> Readiness {
  match grammar(lang).map(|g| g.query.is_some()) {
    Some(true) => Readiness::Ready,
    Some(false) => Readiness::NoTagsQuery,
    None => match feature_of(lang) {
      Some(feature) => Readiness::NoGrammar { feature },
      None => Readiness::Unknown,
    },
  }
}

/// The version of the extraction below, recorded per file on `index_file`
/// (ST0076). **A FILE WHOSE SYMBOLS AN OLDER EXTRACTOR WROTE IS RE-EXTRACTED**
/// by the next reconcile rather than left to mix silently with rows of another
/// shape. Raise it in the same change as anything that alters what a query or
/// this module emits.
pub const EXTRACTOR_VERSION: i64 = 2;

/// The symbols in `bytes`, as the query for `lang` names them.
///
/// `path` is carried onto every row untouched: this module never interprets a
/// path, and the caller's spelling is the one every surface shows.
pub fn symbols_of(lang: &str, path: &str, bytes: &[u8]) -> Result<Vec<Symbol>, NoSymbols> {
  let Some(grammar) = grammar(lang) else {
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
  let lang = grammar.lang;

  let mut parser = tree_sitter::Parser::new();
  if parser.set_language(&grammar.language).is_err() {
    return Err(NoSymbols::Unparsable {
      lang: lang.to_string(),
    });
  }
  let Some(tree) = parser.parse(bytes, None) else {
    return Err(NoSymbols::Unparsable {
      lang: lang.to_string(),
    });
  };

  // **THE QUERY'S ABSENCE IS REPORTED RATHER THAN ANSWERED EMPTY.** A grammar
  // with no query is a fact about the grammar, not about the file, and a caller
  // told "no symbols" would record that the file has none.
  let Some(source) = grammar.query else {
    return Err(NoSymbols::NoTagsQuery {
      lang: lang.to_string(),
    });
  };
  let Ok(query) = tree_sitter::Query::new(&grammar.language, source) else {
    return Err(NoSymbols::NoTagsQuery {
      lang: lang.to_string(),
    });
  };
  let read = read_matches(&query, &tree, bytes);
  Ok(rows_of(path, lang, grammar.module_separator, bytes, read))
}

/// A row a match proposed, before the file's containers and parameters are
/// known.
struct Candidate<'t> {
  pattern: usize,
  kind: SymbolKind,
  subkind: String,
  span: tree_sitter::Node<'t>,
  name: Option<tree_sitter::Node<'t>>,
  name_from_container: bool,
  params: Option<tree_sitter::Node<'t>>,
  qualifier: Option<tree_sitter::Node<'t>>,
}

/// A node other rows inside it belong to.
struct Container<'t> {
  node: tree_sitter::Node<'t>,
  name: String,
  kind: String,
  trait_name: Option<String>,
}

#[derive(Default)]
struct Read<'t> {
  candidates: Vec<Candidate<'t>>,
  containers: Vec<Container<'t>>,
  /// Parameter node id to whether it is optional.
  params: std::collections::HashMap<usize, (tree_sitter::Node<'t>, bool)>,
  ignored: std::collections::HashSet<std::ops::Range<usize>>,
}

/// Every match of the query, sorted into the capture vocabulary the query
/// files state (`queries/rust.scm`).
///
/// **ONE MATCH IS ONE PROPOSAL, AND THE NAME COMES FROM THE MATCH'S `@name`.**
/// `@definition.*` captures the WHOLE definition node, so reading the name off
/// it yields `pub fn quokka(n: usize) -> usize {` -- driven, not reasoned about.
fn read_matches<'t>(
  query: &tree_sitter::Query,
  tree: &'t tree_sitter::Tree,
  bytes: &[u8],
) -> Read<'t> {
  use tree_sitter::StreamingIterator;
  let names = query.capture_names();
  let mut cursor = tree_sitter::QueryCursor::new();
  let mut matches = cursor.matches(query, tree.root_node(), bytes);
  let mut read = Read::default();
  let text = |n: tree_sitter::Node<'_>| n.utf8_text(bytes).unwrap_or_default().to_string();
  while let Some(m) = matches.next() {
    let mut row: Option<(SymbolKind, &str, tree_sitter::Node<'t>)> = None;
    let mut container: Option<(&str, tree_sitter::Node<'t>)> = None;
    let (mut name, mut subkind, mut trait_node, mut params, mut qualifier) =
      (None, None, None, None, None);
    for capture in m.captures() {
      let capture_name = names[capture.index as usize];
      // **THE VOCABULARY IS CLOSED AND ANYTHING ELSE IS SKIPPED RATHER THAN
      // GUESSED.** Grammars' own queries also carry `@doc`, `@local.scope` and
      // friends; a mapping that fell through to a row would report every doc
      // comment as an occurrence.
      if let Some(sub) = capture_name.strip_prefix("definition.") {
        row = Some((SymbolKind::Def, sub, capture.node));
      } else if let Some(sub) = capture_name.strip_prefix("reference.") {
        row = Some((SymbolKind::Ref, sub, capture.node));
      } else if let Some(kind) = capture_name.strip_prefix("container.") {
        container = Some((kind, capture.node));
      } else {
        match capture_name {
          "name" => name = Some(capture.node),
          "subkind" => subkind = Some(capture.node),
          "trait" => trait_node = Some(capture.node),
          "params" => params = Some(capture.node),
          "qualifier" => qualifier = Some(capture.node),
          "ignore" => {
            read.ignored.insert(capture.node.byte_range());
          }
          "param" => {
            read
              .params
              .entry(capture.node.id())
              .or_insert((capture.node, false));
          }
          "param.optional" => {
            read.params.insert(capture.node.id(), (capture.node, true));
          }
          _ => {}
        }
      }
    }
    let name_from_container = query
      .property_settings(m.pattern_index)
      .iter()
      .any(|p| &*p.key == "name.from" && p.value.as_deref() == Some("container"));
    if let (Some((kind, node)), Some(name)) = (container, name) {
      read.containers.push(Container {
        node,
        name: text(name),
        kind: kind.to_string(),
        trait_name: trait_node.map(text),
      });
    }
    if let Some((kind, sub, span)) = row {
      read.candidates.push(Candidate {
        pattern: m.pattern_index,
        kind,
        subkind: subkind.map_or_else(|| sub.to_string(), text),
        span,
        name,
        name_from_container,
        params,
        qualifier,
      });
    }
  }
  read
}

/// The rows a file's matches make, one per syntax node.
fn rows_of(
  path: &str,
  lang: &'static str,
  separator: &str,
  bytes: &[u8],
  read: Read<'_>,
) -> Vec<Symbol> {
  let Read {
    mut candidates,
    containers,
    params,
    ignored,
  } = read;
  // **ONE ROW PER NAME NODE AND KIND, AND THE EARLIER PATTERN WINS** (ST0076's
  // measured duplicate definitions, and issue 0358's `DISTINCT`). rust's own tags query named a method both `@definition.method` and
  // `@definition.function`, Swift's does the same for a class's members, and a
  // query cursor reports every match -- so the duplicate is resolved here, at
  // extraction, by the order the query file states, and never by a `DISTINCT`
  // at read.
  candidates.sort_by_key(|c| c.pattern);
  // **A NAME THAT DEFINES IS NEVER ALSO A REFERENCE TO ITSELF.** An Elixir
  // `def` head with parentheses has the shape of a call and a module's name is
  // an alias, so the reference patterns match both; the definition is the
  // truth about that node. A query cannot say it with `@ignore`, which drops
  // every row on the node, the definition included.
  let defining: std::collections::HashSet<_> = candidates
    .iter()
    .filter(|c| c.kind == SymbolKind::Def)
    .filter_map(|c| c.name.map(|n| n.byte_range()))
    .collect();
  let mut seen = std::collections::HashSet::new();
  let mut out = Vec::new();
  for c in candidates {
    let enclosing = innermost(&containers, c.span);
    let name = match (c.name, c.name_from_container) {
      (_, true) => match enclosing {
        Some(at) => qualified(&containers, at, separator),
        None => continue,
      },
      // **NO `@name` MEANS NO ROW.** A symbol whose name is the whole
      // declaration is not a symbol anybody can search for.
      (None, false) => continue,
      (Some(node), false) => {
        if ignored.contains(&node.byte_range())
          || (c.kind == SymbolKind::Ref && defining.contains(&node.byte_range()))
        {
          continue;
        }
        node.utf8_text(bytes).unwrap_or_default().to_string()
      }
    };
    let key = (
      c.name
        .map_or_else(|| c.span.byte_range(), |n| n.byte_range()),
      c.kind,
    );
    if name.is_empty() || !seen.insert(key) {
      continue;
    }
    // **THE ARITY COUNTS THE PARAMETERS THAT ARE CHILDREN OF THE `@params`
    // NODE**, so a pattern without one leaves the language's missing concept
    // absent rather than zero, and a `@params` node with no parameter children
    // is a real arity of 0.
    let arity = c.params.map(|list| {
      params
        .values()
        .filter(|(p, _)| p.parent().is_some_and(|parent| parent.id() == list.id()))
        .fold((0u32, 0u32), |(all, required), (_, optional)| {
          (all + 1, required + u32::from(!optional))
        })
    });
    let qualifier = c
      .qualifier
      .map(|q| q.utf8_text(bytes).unwrap_or_default().to_string());
    let (container, container_kind, trait_name) = match enclosing {
      Some(at) => (
        Some(qualified(&containers, at, separator)),
        Some(containers[at].kind.clone()),
        containers[at].trait_name.clone(),
      ),
      None => (None, None, None),
    };
    out.push(Symbol {
      path: path.to_string(),
      lang,
      name,
      kind: c.kind,
      span: Span {
        start_line: c.span.start_position().row as u32 + 1,
        end_line: c.span.end_position().row as u32 + 1,
      },
      subkind: c.subkind,
      container,
      container_kind,
      trait_name,
      arity: arity.map(|(all, _)| all),
      arity_min: arity.map(|(_, required)| required),
      // **A QUALIFIER IS THE PATH AS WRITTEN, AND IT IS WHAT MAKES A ROW LEVEL 2.**
      // Nothing here resolves it: `AddressError::new` says what the source
      // wrote, not which `new` it reaches.
      qualifier: qualifier.clone(),
      level: if qualifier.is_some() { 2 } else { 1 },
    });
  }
  // Path order within a file: the order a reader would meet them.
  out.sort_by(|a, b| {
    (a.span.start_line, a.span.end_line, &a.name, a.kind.as_str()).cmp(&(
      b.span.start_line,
      b.span.end_line,
      &b.name,
      b.kind.as_str(),
    ))
  });
  out
}

/// The innermost container strictly enclosing `node`, as an index.
fn innermost(containers: &[Container<'_>], node: tree_sitter::Node<'_>) -> Option<usize> {
  let within = |outer: &std::ops::Range<usize>, inner: &std::ops::Range<usize>| {
    outer.start <= inner.start && inner.end <= outer.end && outer != inner
  };
  containers
    .iter()
    .enumerate()
    .filter(|(_, c)| within(&c.node.byte_range(), &node.byte_range()))
    .min_by_key(|(_, c)| c.node.byte_range().len())
    .map(|(at, _)| at)
}

/// A container's name as a reader writes it: a module inside a module carries
/// the outer module's name before its own, joined by the language's separator
/// (`a::b`, `Intent.Store`). **FROM THE FILE'S OWN SYNTAX ONLY**: a file's
/// crate path is not written in the file, so it is never invented.
fn qualified(containers: &[Container<'_>], at: usize, separator: &str) -> String {
  let here = &containers[at];
  match innermost(containers, here.node) {
    Some(outer) if here.kind == "module" && containers[outer].kind == "module" => {
      format!(
        "{}{separator}{}",
        qualified(containers, outer, separator),
        here.name
      )
    }
    _ => here.name.clone(),
  }
}

/// A compiled grammar and the query that names its symbols.
struct Grammar {
  lang: &'static str,
  language: tree_sitter::Language,
  query: Option<&'static str>,
  /// What joins a module to the module inside it.
  module_separator: &'static str,
}

/// The compiled grammar for a language, when this build carries one.
///
/// **THIS IS THE ONLY `cfg` IN THE MODULE**, and it is here rather than around
/// the roster so that "no grammar compiled in" stays distinguishable from "no
/// such language".
///
/// **RUST AND ELIXIR ARE NAMED BY INTENT'S OWN QUERIES** (hv's ruling of
/// 2026-09-16, reversing AC-20.2 for those two in ST0076): typed definitions,
/// their containers and arity are not in any grammar's tags query. Each query
/// replaces the grammar's tags query entirely, because running both would give
/// one call two rows. Swift and Lua stay on their grammars' own.
fn grammar(lang: &str) -> Option<Grammar> {
  match lang {
    #[cfg(feature = "lang-rust")]
    "rust" => Some(Grammar {
      lang: "rust",
      language: tree_sitter_rust::LANGUAGE.into(),
      query: Some(include_str!("queries/rust.scm")),
      module_separator: "::",
    }),
    #[cfg(feature = "lang-elixir")]
    "elixir" => Some(Grammar {
      lang: "elixir",
      language: tree_sitter_elixir::LANGUAGE.into(),
      query: Some(include_str!("queries/elixir.scm")),
      module_separator: ".",
    }),
    #[cfg(feature = "lang-swift")]
    "swift" => Some(Grammar {
      lang: "swift",
      language: tree_sitter_swift::LANGUAGE.into(),
      query: Some(tree_sitter_swift::TAGS_QUERY),
      module_separator: ".",
    }),
    #[cfg(feature = "lang-lua")]
    "lua" => Some(Grammar {
      lang: "lua",
      language: tree_sitter_lua::LANGUAGE.into(),
      query: Some(tree_sitter_lua::TAGS_QUERY),
      module_separator: ".",
    }),
    #[cfg(feature = "lang-bash")]
    // **`tree-sitter-bash` SHIPS NO TAGS QUERY** (0.25.1: `HIGHLIGHT_QUERY`
    // alone), so the grammar parses and names nothing. Declared here anyway,
    // with the absence stated, rather than left out: a language missing from
    // this match is one this build has no grammar for, and that is a different
    // fact with a different fix.
    "shell" => Some(Grammar {
      lang: "shell",
      language: tree_sitter_bash::LANGUAGE.into(),
      query: None,
      module_separator: ".",
    }),
    _ => None,
  }
}
