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
pub const EXTRACTOR_VERSION: i64 = 3;

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
  extract(&grammar, grammar.query, path, bytes)
}

/// The rows `source` names in `bytes`, parsed with `grammar`. One home for the
/// parse and the read, so a test's own query takes exactly the path a shipped
/// query takes.
fn extract(
  grammar: &Grammar,
  source: Option<&str>,
  path: &str,
  bytes: &[u8],
) -> Result<Vec<Symbol>, NoSymbols> {
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
  let Some(source) = source else {
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

/// What a reference row in `lang` does not yet cover, in the words every
/// published surface uses, or `None` where nothing is known to be missing.
///
/// **THE ONE HOME FOR THE CLAIM, AND THE REGISTER'S TEXT IS HELD TO IT**
/// (ST0076 WP-04, AC-04.2). The search row's `when_to_use` and the MCP
/// `instructions` line carry these sentences verbatim and a test in the CLI
/// crate fails when they differ, so the change that widens a language's
/// references (WP-02 for Rust, WP-03 for Elixir) edits this and the register
/// together, or does not build green.
pub fn what_a_reference_misses(lang: &str) -> Option<&'static str> {
  match lang {
    "rust" => Some(
      "in Rust, inside a macro invocation a name the tokens do not show as a call or a path is an occurrence of the name rather than a use, and its qualifier is only the one segment before it; a name in a nested use list such as use a::{b::C} has no use row; and a name passed as a value outside a macro, such as map(f), is not a reference",
    ),
    "elixir" => Some(
      "in Elixir, nothing import or use brings in is applied, so a call to an imported function, Kernel's included, is a level-1 reference with no module; and a call through a module held in a variable, such as mod.fun(..), has no qualifier",
    ),
    "swift" => Some("Swift files give definitions only"),
    _ => None,
  }
}

/// Every subkind `lang`'s query can write, in the query's order, each once.
///
/// **DERIVED FROM THE COMPILED QUERY AND NEVER LISTED** (vc, 2026-09-17, for
/// `--subkind`'s refusal). A roster written beside the queries would agree with
/// them until a capture was added, so a pattern's `@definition.*` and
/// `@reference.*` captures are read off the compiled query, which is the same
/// object [`symbols_of`] extracts with. **ONE THING IS READ FROM THE SOURCE
/// TEXT**: a pattern that carries `@subkind` writes that node's text instead, and
/// the compiled query does not expose the `#any-of?` list that bounds it, so the
/// list is read from that pattern's own bytes.
///
/// Empty for a language this build carries no query for, which is the same
/// answer [`readiness`] gives in other words.
pub fn subkinds(lang: &str) -> Vec<String> {
  use tree_sitter::CaptureQuantifier;
  let Some(grammar) = grammar(lang) else {
    return Vec::new();
  };
  let Some(source) = grammar.query else {
    return Vec::new();
  };
  let Ok(query) = tree_sitter::Query::new(&grammar.language, source) else {
    return Vec::new();
  };
  let names = query.capture_names();
  let mut out: Vec<String> = Vec::new();
  for pattern in 0..query.pattern_count() {
    let quantifiers = query.capture_quantifiers(pattern);
    let present = |i: usize| quantifiers[i] != CaptureQuantifier::Zero;
    let spelled = if names
      .iter()
      .enumerate()
      .any(|(i, name)| *name == "subkind" && present(i))
    {
      any_of_subkind(
        &source[query.start_byte_for_pattern(pattern)..query.end_byte_for_pattern(pattern)],
      )
    } else {
      Vec::new()
    };
    for (i, name) in names.iter().enumerate() {
      if !present(i) {
        continue;
      }
      let Some(sub) = name
        .strip_prefix("definition.")
        .or_else(|| name.strip_prefix("reference."))
      else {
        continue;
      };
      let words = if spelled.is_empty() {
        vec![sub.to_string()]
      } else {
        spelled.clone()
      };
      for word in words {
        if !out.contains(&word) {
          out.push(word);
        }
      }
    }
  }
  out
}

/// The quoted words of a pattern's `(#any-of? @subkind ...)`, in order.
fn any_of_subkind(pattern: &str) -> Vec<String> {
  let Some(start) = pattern.find("#any-of? @subkind") else {
    return Vec::new();
  };
  let rest = &pattern[start + "#any-of? @subkind".len()..];
  let list = &rest[..rest.find(')').unwrap_or(rest.len())];
  list
    .split('"')
    .skip(1)
    .step_by(2)
    .map(str::to_string)
    .collect()
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
  /// A written arity (`&Map.get/2`), read in place of a counted one.
  arity: Option<tree_sitter::Node<'t>>,
  /// `arity.piped`: one argument arrives by a pipe and is not written.
  piped: bool,
  /// `name.expand "alias"`: the name, not the qualifier, is what an alias
  /// expands.
  expand_name: bool,
  /// `@name.base`: the node the name is written after (`MyApp` in
  /// `alias MyApp.{Repo, Mailer}`), expanded and joined before the name.
  name_base: Option<tree_sitter::Node<'t>>,
}

/// One alias form (`@alias`) and what it names.
struct AliasForm<'t> {
  node: tree_sitter::Node<'t>,
  /// The ancestor kind that bounds it (`alias.scope`); the file when absent.
  scope: Option<String>,
  base: Option<tree_sitter::Node<'t>>,
  paths: Vec<tree_sitter::Node<'t>>,
  short: Option<tree_sitter::Node<'t>>,
}

/// One short name an alias puts in scope, over a byte range of the file.
struct AliasEntry {
  short: String,
  full: String,
  start: usize,
  end: usize,
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
  aliases: Vec<AliasForm<'t>>,
  /// `@qualifier.self` and `@self` nodes: each stands for its enclosing
  /// container's name. `@self` registers the node and qualifies nothing.
  selves: Vec<tree_sitter::Node<'t>>,
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
    let (mut arity, mut alias, mut alias_base, mut alias_as) = (None, None, None, None);
    let mut name_base = None;
    let mut alias_paths = Vec::new();
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
          "name.base" => name_base = Some(capture.node),
          "self" => read.selves.push(capture.node),
          "subkind" => subkind = Some(capture.node),
          "trait" => trait_node = Some(capture.node),
          "params" => params = Some(capture.node),
          "qualifier" => qualifier = Some(capture.node),
          "qualifier.self" => {
            read.selves.push(capture.node);
            qualifier = qualifier.or(Some(capture.node));
          }
          "arity" => arity = Some(capture.node),
          "alias" => alias = Some(capture.node),
          "alias.path" => alias_paths.push(capture.node),
          "alias.base" => alias_base = Some(capture.node),
          // **THE SHORT NAME AN `as:` GIVES IS INTRODUCED, NOT REFERENCED**, as a
          // definition's own name is not a reference to it. The query cannot
          // say so: a second `@ignore` in the alias form fails its `#eq?`.
          "alias.as" => {
            alias_as = Some(capture.node);
            read.ignored.insert(capture.node.byte_range());
          }
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
    let property = |key: &str| {
      query
        .property_settings(m.pattern_index)
        .iter()
        .find(|p| &*p.key == key)
        .and_then(|p| p.value.as_deref().map(str::to_string))
    };
    let name_from_container = property("name.from").as_deref() == Some("container");
    if let Some(node) = alias {
      read.aliases.push(AliasForm {
        node,
        scope: property("alias.scope"),
        base: alias_base,
        paths: alias_paths,
        short: alias_as,
      });
    }
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
        arity,
        piped: property("arity.piped").as_deref() == Some("true"),
        expand_name: property("name.expand").as_deref() == Some("alias"),
        name_base,
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
    aliases,
    selves,
  } = read;
  let text = |n: tree_sitter::Node<'_>| n.utf8_text(bytes).unwrap_or_default().to_string();
  let names = Names {
    containers: &containers,
    selves: &selves,
    aliases: alias_entries(&aliases, &containers, &selves, separator, bytes),
    separator,
    bytes,
  };
  // **ONE ROW PER NAME NODE AND KIND, AND THE EARLIER PATTERN WINS** (ST0076's
  // measured duplicate definitions, and issue 0358's `DISTINCT`). rust's own
  // tags query named a method both `@definition.method` and
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
        // **A NAME WRITTEN AFTER A BASE IS THE BASE, EXPANDED, THEN THE NAME AS
        // WRITTEN** (`alias MyApp.{Repo, Mailer}` names `MyApp.Repo`). An alias
        // is not in scope inside its own form, so expanding the name alone would
        // answer `Repo`, or an earlier alias of that short name.
        match (c.name_base, c.expand_name) {
          (Some(base), _) => format!("{}{separator}{}", names.expand(base), text(node)),
          (None, true) => names.expand(node),
          (None, false) => text(node),
        }
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
    //
    // **A WRITTEN ARITY IS READ AS WRITTEN** (`&Map.get/2`), and text that is
    // not a number is no arity rather than 0. **A PIPED CALL TAKES ONE ARGUMENT
    // IT DOES NOT WRITE**, so `m |> Map.get(:k)` is `get/2`.
    let counted = c.params.map(|list| {
      params
        .values()
        .filter(|(p, _)| p.parent().is_some_and(|parent| parent.id() == list.id()))
        .fold((0u32, 0u32), |(all, required), (_, optional)| {
          (all + 1, required + u32::from(!optional))
        })
    });
    let written = c
      .arity
      .and_then(|n| text(n).trim().parse::<u32>().ok())
      .map(|n| (n, n));
    let piped = u32::from(c.piped);
    let arity = match c.arity {
      Some(_) => written,
      None => counted,
    }
    .map(|(all, required)| (all + piped, required + piped));
    // **A ROW HAS ITS NAME OR ITS QUALIFIER EXPANDED, NEVER BOTH**: a module
    // reference is the module, and a call is qualified by one.
    let qualifier = c.qualifier.map(|q| match c.expand_name {
      true => text(q),
      false => names.expand(q),
    });
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
      // wrote, not which `new` it reaches. The one rewrite is the file's own:
      // an alias the file declared, or the container a `@qualifier.self` names.
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

/// What a file's own syntax says a written name stands for: its aliases and
/// its self-references.
struct Names<'a, 't> {
  containers: &'a [Container<'t>],
  selves: &'a [tree_sitter::Node<'t>],
  aliases: Vec<AliasEntry>,
  separator: &'a str,
  bytes: &'a [u8],
}

impl Names<'_, '_> {
  fn expand(&self, node: tree_sitter::Node<'_>) -> String {
    let written = node.utf8_text(self.bytes).unwrap_or_default();
    expand_at(
      written,
      node,
      self.containers,
      self.selves,
      &self.aliases,
      self.separator,
    )
  }
}

/// `written`, as the file declares it at `node`.
///
/// **A SELF-REFERENCE NAMES ITS CONTAINER** when it starts the name
/// (`__MODULE__.Sub` in `MyApp.Web` is `MyApp.Web.Sub`). **OTHERWISE THE FIRST
/// SEGMENT IS LOOKED UP AMONG THE ALIASES IN SCOPE AT `node`**, and the one
/// declared LATEST wins, so a second alias of the same short name shadows the
/// first for everything after it. Only the first segment is ever rewritten:
/// `Repo.Query` after `alias MyApp.Repo` is `MyApp.Repo.Query`.
fn expand_at(
  written: &str,
  node: tree_sitter::Node<'_>,
  containers: &[Container<'_>],
  selves: &[tree_sitter::Node<'_>],
  aliases: &[AliasEntry],
  separator: &str,
) -> String {
  let at = node.byte_range();
  if let Some(own) = selves
    .iter()
    .find(|s| s.start_byte() == at.start && s.end_byte() <= at.end)
  {
    let rest = written.get(own.end_byte() - at.start..).unwrap_or_default();
    return match innermost(containers, *own) {
      Some(c) => format!("{}{rest}", qualified(containers, c, separator)),
      None => written.to_string(),
    };
  }
  let first = written.split(separator).next().unwrap_or(written);
  aliases
    .iter()
    .filter(|a| a.short == first && a.start <= at.start && at.start < a.end)
    .max_by_key(|a| a.start)
    .map_or_else(
      || written.to_string(),
      |a| format!("{}{}", a.full, &written[first.len()..]),
    )
}

/// The file's aliases, in the order it declares them, each expanded by the
/// aliases before it (`alias MyApp.Accounts` then `alias Accounts.User` makes
/// `User` mean `MyApp.Accounts.User`).
///
/// **AN ALIAS HOLDS FROM THE END OF ITS FORM TO THE END OF THE NEAREST ANCESTOR
/// OF ITS `alias.scope` KIND**, or to the end of the file where no ancestor is
/// of that kind. For Elixir that kind is `do_block`, and the known gap is the
/// keyword form: `def f, do: (alias A.B; B.g())` has no `do_block` of its own,
/// so the alias reaches to the end of the enclosing module's block.
fn alias_entries(
  forms: &[AliasForm<'_>],
  containers: &[Container<'_>],
  selves: &[tree_sitter::Node<'_>],
  separator: &str,
  bytes: &[u8],
) -> Vec<AliasEntry> {
  let text = |n: tree_sitter::Node<'_>| n.utf8_text(bytes).unwrap_or_default();
  let mut order: Vec<&AliasForm<'_>> = forms.iter().collect();
  order.sort_by_key(|f| f.node.start_byte());
  let mut entries: Vec<AliasEntry> = Vec::new();
  for form in order {
    let end = form
      .scope
      .as_deref()
      .and_then(|kind| {
        std::iter::successors(form.node.parent(), |n| n.parent()).find(|n| n.kind() == kind)
      })
      .map_or(bytes.len(), |n| n.end_byte());
    let start = form.node.end_byte();
    let expand = |n: tree_sitter::Node<'_>, entries: &[AliasEntry]| {
      expand_at(text(n), n, containers, selves, entries, separator)
    };
    let base = form.base.map(|b| expand(b, &entries));
    let added: Vec<AliasEntry> = form
      .paths
      .iter()
      .map(|path| {
        let written = text(*path);
        let full = match &base {
          Some(base) => format!("{base}{separator}{written}"),
          None => expand(*path, &entries),
        };
        let short = match (form.short, form.paths.len()) {
          (Some(short), 1) => text(short).to_string(),
          _ => written
            .rsplit(separator)
            .next()
            .unwrap_or(written)
            .to_string(),
        };
        AliasEntry {
          short,
          full,
          start,
          end,
        }
      })
      .collect();
    entries.extend(added);
  }
  entries
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

#[cfg(all(test, feature = "lang-elixir"))]
mod tests {
  use super::*;

  /// **THE ARMS DRIVE THE EXTRACTOR'S SHAPES, NOT A SHIPPED QUERY**, so the
  /// query is the arms' own: it states each capture and property once, and
  /// `extract` reads it exactly as it reads `queries/elixir.scm`.
  const QUERY: &str = r#"
(call
  target: (identifier) @ignore
  (arguments . (alias) @name)
  (#eq? @ignore "defmodule")) @definition.module @container.module

(call
  target: (identifier) @ignore
  (arguments
    .
    [
      (alias) @alias.path
      (dot left: (identifier) @qualifier.self right: (alias)) @alias.path
      (dot left: (alias) @alias.base right: (tuple (alias) @alias.path))
    ]
    (keywords (pair value: (alias) @alias.as))?)
  (#eq? @ignore "alias")
  (#set! alias.scope "do_block")) @alias

(binary_operator
  operator: "|>"
  right: (call
    target: (dot left: (alias) @qualifier right: (identifier) @name @reference.call)
    (arguments) @params)
  (#set! arity.piped "true"))

(unary_operator
  operator: "&"
  operand: (binary_operator
    left: (call target: (dot left: (alias) @qualifier right: (identifier) @name @reference.call))
    operator: "/"
    right: (_) @arity))

(call
  target: (dot
    left: (identifier) @qualifier.self
    right: (identifier) @name @reference.call)
  (#eq? @qualifier.self "__MODULE__"))

(call
  target: (dot left: (alias) @qualifier right: (identifier) @name @reference.call)
  (arguments) @params)

(arguments (_) @param)

((alias) @name @reference.module
  (#set! name.expand "alias"))
"#;

  fn elixir(src: &str) -> Vec<Symbol> {
    let grammar = grammar("elixir").expect("the elixir grammar is in the default build");
    extract(&grammar, Some(QUERY), "lib/web.ex", src.as_bytes()).expect("the arms' query compiles")
  }

  /// (name, subkind, qualifier, arity, line) for one reference.
  type Written = (String, String, Option<String>, Option<u32>, u32);

  fn refs(rows: &[Symbol]) -> Vec<Written> {
    rows
      .iter()
      .filter(|s| s.kind == SymbolKind::Ref)
      .map(|s| {
        (
          s.name.clone(),
          s.subkind.clone(),
          s.qualifier.clone(),
          s.arity,
          s.span.start_line,
        )
      })
      .collect()
  }

  fn has(got: &[Written], want: (&str, &str, Option<&str>, Option<u32>, u32)) {
    let want = (
      want.0.to_string(),
      want.1.to_string(),
      want.2.map(str::to_string),
      want.3,
      want.4,
    );
    assert!(got.contains(&want), "{want:?} missing from {got:#?}");
  }

  const WEB: &str = "defmodule MyApp.Web do
  alias MyApp.Accounts
  alias Accounts.User
  alias MyApp.{Repo, Mailer}
  alias Other.Repo
  alias MyApp.Repo, as: R
  alias __MODULE__.Sub
  def f(m, x) do
    User.get(1)
    Repo.all()
    R.one()
    Sub.go()
    __MODULE__.g()
    m |> Map.get(:k)
    &Map.put/3
    Mailer.send(x)
    Repo.Query.run()
  end
end
Repo.all()
";

  #[test]
  fn an_alias_expands_the_qualifier_it_names_for_the_rest_of_its_block() {
    let got = refs(&elixir(WEB));
    has(&got, ("send", "call", Some("MyApp.Mailer"), Some(1), 16));
    has(&got, ("one", "call", Some("MyApp.Repo"), Some(0), 11));
    has(&got, ("run", "call", Some("Other.Repo.Query"), Some(0), 17));
    has(&got, ("all", "call", Some("Repo"), Some(0), 20));
  }

  #[test]
  fn a_later_alias_of_the_same_short_name_shadows_the_earlier() {
    let got = refs(&elixir(WEB));
    has(&got, ("all", "call", Some("Other.Repo"), Some(0), 10));
  }

  #[test]
  fn an_alias_path_is_expanded_by_the_aliases_and_the_self_reference_before_it() {
    let got = refs(&elixir(WEB));
    has(
      &got,
      ("get", "call", Some("MyApp.Accounts.User"), Some(1), 9),
    );
    has(&got, ("go", "call", Some("MyApp.Web.Sub"), Some(0), 12));
    has(&got, ("g", "call", Some("MyApp.Web"), None, 13));
  }

  #[test]
  fn a_row_has_its_name_or_its_qualifier_expanded_never_both() {
    let rows = elixir(WEB);
    let got = refs(&rows);
    has(&got, ("MyApp.Accounts.User", "module", None, None, 9));
    let user = rows
      .iter()
      .find(|s| s.name == "get")
      .expect("the call is a row");
    assert_eq!(
      (user.qualifier.as_deref(), user.level),
      (Some("MyApp.Accounts.User"), 2)
    );
  }

  #[test]
  fn a_piped_call_and_a_capture_carry_the_arity_they_are_called_with() {
    let got = refs(&elixir(WEB));
    has(&got, ("get", "call", Some("Map"), Some(2), 14));
    has(&got, ("put", "call", Some("Map"), Some(3), 15));
    let rows = elixir("defmodule A do\n  def f, do: &Map.put/x\nend\n");
    let put = rows
      .iter()
      .find(|s| s.name == "put")
      .expect("the capture is a row");
    assert_eq!(
      put.arity, None,
      "a written arity that is not a number is no arity"
    );
  }
}
