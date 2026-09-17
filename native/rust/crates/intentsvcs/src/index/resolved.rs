//! Level 3: references resolved to the definitions they name, by each
//! language's own toolchain (ST0076 WP-05, vc decision 25).
//!
//! **ONE CORE FOR EVERY LANGUAGE.** Rust reads rust-analyzer's SCIP export and
//! Elixir reads the compiler tracer (WP-06), and both hand this module the same
//! thing: the files the tool read, with the hash of the bytes it read, and the
//! references it saw. What the store keeps is decided here, once, so the two
//! languages mean one thing by level 3.
//!
//! **PURE (IN-AG-PFIC-001).** [`join`], [`locations`] and [`must_run_full`]
//! read values and return values. Running a toolchain is a [`Resolver`]'s job,
//! and writing what `join` decides is the store's, both called from
//! `Facade::index_resolve`.
//!
//! # What a run keeps
//!
//! **A RESOLVED ROW ALWAYS JOINS A WRITTEN REFERENCE**, on path, line and name.
//! The syntax tiers write a row for every reference the grammar can see, and a
//! toolchain sees more: fields, variants, constants, compiler expansion. A
//! reference only the toolchain sees is counted and not stored. Whether to store
//! those is a later ruling for hv, and this is the shape it starts from.
//!
//! **THE JOIN COMPARES THE SAME BYTES.** A written row was extracted from the
//! bytes the index read, and a tool's reference from the bytes the tool read,
//! so a file is joined only where the two hashes agree.
//!
//! # The counts
//!
//! `matched`, `unmatched` and `dropped` partition everything the tool emitted,
//! the references a reader excluded included, so a join that stops matching
//! shows as a count that moved.
//!
//! - `matched`: joined a written reference, and stored.
//! - `unmatched`: in a file the run joined, on no written reference.
//! - `dropped`: compared with nothing, counted by reason in `dropped_by`: the
//!   core's own ([`CORE_REASONS`]) and the ones a reader declares in
//!   [`Resolver::excludes`]. The reasons are published words, so a closed
//!   roster: a run whose reader excludes for a reason it did not declare is
//!   refused as failed rather than printed (vc, 2026-09-17).
//! - `ambiguous`: stored keys naming two or more targets. There is no tie-break
//!   at write (`expect` on an `Option` and on a `Result` on one line), so both
//!   are stored and the count says how often a key named more than one.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use super::symbols::Symbol;

/// What the last run of a language did: it stored.
pub const CURRENT: &str = "current";

/// A reference the tool gave no line.
pub const NO_LINE: &str = "no-line";
/// A reference in a file outside the project.
pub const OUTSIDE_THE_PROJECT: &str = "outside-the-project";
/// A reference in a file the tool did not report reading, so there are no
/// bytes to compare.
pub const UNREAD: &str = "unread";
/// A reference in a file the index does not hold.
pub const NOT_INDEXED: &str = "not-indexed";
/// A reference in a file whose bytes the index read differ from the bytes the
/// tool read.
pub const MOVED: &str = "moved";

/// The reasons the core drops a reference for, whatever the reader.
pub const CORE_REASONS: &[&str] = &[NO_LINE, OUTSIDE_THE_PROJECT, UNREAD, NOT_INDEXED, MOVED];

/// A written reference as a resolved row joins it: path, line and name, as a
/// `symbols` row holds them.
pub type Key = (String, u32, String);

/// A file a toolchain read, and the hash of the bytes it read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Read {
  /// Relative to the project root, as the index keys a path.
  pub path: String,
  /// Hex SHA-256, as `index_file.indexed_sha256` spells it.
  pub sha256: String,
}

/// One reference a toolchain saw.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference {
  /// Relative to the project root; `None` for a file outside it.
  pub path: Option<String>,
  /// 1-based, as a `symbols` row counts lines; `None` where the tool gave none.
  pub line: Option<u32>,
  /// The name as written, which is what a written row's `name` holds.
  pub name: String,
  /// The definition it names, as one printable name per language with no tool
  /// or crate version in it, so one definition prints the same way every run.
  pub target: String,
  /// Where that definition is. `None` where the tool does not say, and then
  /// the core looks for it among the definition rows (see [`locations`]).
  pub target_path: Option<String>,
  pub target_line: Option<u32>,
}

/// What one toolchain run read and saw.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Trace {
  pub read: Vec<Read>,
  pub references: Vec<Reference>,
  /// What the reader emitted no reference for, counted by reason (Elixir's
  /// operators, which no written row can match). They count as dropped, so the
  /// partition covers everything the tool emitted and not only what the reader
  /// passed on.
  pub excluded: BTreeMap<String, u64>,
}

/// Why a run stored nothing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unresolved {
  /// The tool is not where this process can run it.
  Missing { detail: String },
  /// The tool ran and failed, at the file and line it named where it named
  /// one.
  Failed {
    path: Option<String>,
    line: Option<u32>,
    detail: String,
  },
  /// The project holds no build for the tool to read: no `mix.exs`, no Cargo
  /// workspace. The tool was not run.
  ///
  /// **FAILED ONLY WHEN THE LANGUAGE WAS ASKED FOR BY NAME** (vc, 2026-09-17).
  /// A run over every declared language names it as not applicable and leaves
  /// its record alone, because otherwise every Rust-only estate would carry a
  /// failed Elixir tier after each resolve, and a permanent false alarm is how
  /// a real failure comes to be ignored.
  NotApplicable { detail: String },
}

impl Unresolved {
  /// The state the language's record takes, in the store's spelling. A
  /// language not applicable is recorded only when it was asked for by name,
  /// and then as failed.
  pub fn state(&self) -> &'static str {
    match self {
      Unresolved::Missing { .. } => "missing",
      Unresolved::Failed { .. } | Unresolved::NotApplicable { .. } => "failed",
    }
  }
}

/// What a run is asked to trace.
#[derive(Debug, Clone, Copy)]
pub struct Scope<'a> {
  /// The project root.
  pub root: &'a Path,
  /// A directory of Intent's own and never the project's build directory (vc
  /// decision 25 (6)). The facade creates it and never empties it: it is the
  /// tool's incremental state, and it persists between runs.
  pub cache: &'a Path,
  /// Rebuild everything, where the tool keeps incremental state. The facade
  /// sets it whatever the caller asked when [`must_run_full`] says so.
  pub full: bool,
  /// Every path the index holds, relative to the root and in path order.
  ///
  /// **WHERE A READER FINDS ITS PROJECT, RATHER THAN BY WALKING THE TREE OR
  /// ASKING GIT** (vc, 2026-09-17): the index is already the one answer to
  /// which files a project holds, it honours the ignores every other door
  /// honours, and it works where there is no repository. Rust's root-most
  /// `Cargo.toml` manifests and Elixir's `mix.exs` are read from here.
  pub indexed: &'a [String],
}

/// A language's toolchain, read into a [`Trace`].
///
/// **THE IMPURE HALF, AND THE ONLY ONE.** An implementation runs a tool and
/// reads what it wrote; everything a run decides after that is [`join`]'s. A
/// test proves the core with an implementation that returns a trace it was
/// handed, which is why this is a trait.
pub trait Resolver {
  /// The language, as `index_file.lang` spells it.
  fn lang(&self) -> &'static str;
  /// The tool, as a person would install it.
  fn tool(&self) -> &'static str;
  /// Run the tool over the project the scope describes. A scope holding
  /// nothing for the tool is [`Unresolved::NotApplicable`], and the tool is
  /// not run.
  fn trace(&self, scope: &Scope<'_>) -> Result<Trace, Unresolved>;
  /// The reasons this reader excludes references for, in [`Trace::excluded`].
  /// A closed roster: a trace naming any other reason is refused.
  fn excludes(&self) -> &'static [&'static str] {
    &[]
  }
  /// A definition row printed the way this reader prints a target, for a
  /// reader whose tool names a target without saying where it is. `None`
  /// where the row is not something a target can name.
  fn target_of(&self, _def: &Symbol) -> Option<String> {
    None
  }
  /// Which of the definition rows printing one target is where it is
  /// defined, or `None` where the language cannot say.
  ///
  /// **EXACTLY ONE, UNLESS THE LANGUAGE KNOWS BETTER.** Two rows printing one
  /// target are two definitions to the core, and it will not choose between
  /// them. A language where several rows are one definition overrides this:
  /// Elixir writes a row per clause, and a function's clauses in one file are
  /// one function, defined at its first.
  fn locate<'a>(&self, defs: &[&'a Symbol]) -> Option<&'a Symbol> {
    match defs {
      [one] => Some(*one),
      _ => None,
    }
  }
}

/// The resolvers this build carries.
///
/// **EMPTY UNTIL A LANGUAGE'S READER LANDS.** The Rust reader (WP-05) and the
/// Elixir reader (WP-06) each add themselves here, and `intent index resolve`
/// names the languages this list covers when it is asked for one it does not.
pub fn readers() -> Vec<Box<dyn Resolver>> {
  Vec::new()
}

/// One resolved row, as `resolved` holds it.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize)]
pub struct Row {
  pub path: String,
  pub line: u32,
  pub name: String,
  pub target: String,
  pub target_path: Option<String>,
  pub target_line: Option<u32>,
}

/// The counts a run records (vc decision 25 (4)). The fifth, the stale paths,
/// is read at status time, because it moves when the files move and not when a
/// run does.
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize)]
pub struct Tally {
  pub matched: u64,
  pub unmatched: u64,
  pub dropped: u64,
  pub ambiguous: u64,
  /// `dropped`, by reason. The values sum to `dropped`.
  pub dropped_by: BTreeMap<String, u64>,
}

impl Tally {
  fn drop_for(&mut self, reason: &str, n: u64) {
    self.dropped += n;
    *self.dropped_by.entry(reason.to_string()).or_default() += n;
  }
}

/// What a run stores: the files it joined, their rows, and its counts.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Joined {
  /// Every file the run joined, in path order, including the ones that
  /// resolve nothing now: their earlier rows are replaced by none.
  pub files: Vec<Read>,
  /// In key order, one per path, line, name and target.
  pub rows: Vec<Row>,
  pub tally: Tally,
}

/// One language's level 3, as `intent index status` reports it: the record of
/// its last run, and the paths gone stale since.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Run {
  /// `current`, `missing` or `failed`: what the last run did.
  pub state: String,
  pub tool: String,
  /// The last run's failure: the file and line the tool named, and what it
  /// said. `None` after a run that stored.
  pub path: Option<String>,
  pub line: Option<u32>,
  pub detail: Option<String>,
  /// When the rows the store holds were resolved. `None` until a run stores,
  /// and unchanged by a run that fails.
  pub resolved_at: Option<String>,
  /// How many runs have stored.
  pub run: u64,
  /// The extractor version whose written rows the last stored run joined
  /// against (vc, 2026-09-17). A build writing another version has
  /// re-extracted those rows, so every resolved path is stale.
  pub symbols_version: Option<i64>,
  /// The counts of the last run that stored.
  #[serde(flatten)]
  pub tally: Tally,
  /// Paths whose rows were resolved against written rows the index no longer
  /// holds, in path order: the file's bytes moved, or the extractor did.
  ///
  /// **PATHS, NOT A COUNT**, for the reason `index status` lists its skipped
  /// files: a reader deciding whether a caller list can be trusted needs to
  /// know whether the file in front of them is one of them.
  pub stale: Vec<String>,
}

/// Whether a run must rebuild everything, whatever the caller asked (vc,
/// 2026-09-17, on dc's review of the reader contract).
///
/// **AN INCREMENTAL RUN TRACES ONLY WHAT THE TOOL REBUILDS**, so over a build
/// cache that already exists it can trace nothing, store nothing, and report a
/// success with an empty tier. That is safe only when the store already holds
/// what the cache built: a run stored, the last run stored too, and it joined
/// against the written rows this build's extractor writes.
pub fn must_run_full(previous: Option<&Run>, extractor: i64) -> bool {
  match previous {
    None => true,
    Some(run) => {
      run.resolved_at.is_none() || run.state != CURRENT || run.symbols_version != Some(extractor)
    }
  }
}

/// Where each target is defined, for a reader whose tool does not say: the
/// reader prints each definition row as a target ([`Resolver::target_of`]),
/// and chooses among the rows printing one target ([`Resolver::locate`]).
///
/// **A TARGET THE READER CANNOT PLACE IS PLACED NOWHERE.** Choosing a row the
/// language does not choose would be a tie-break the store refuses everywhere
/// else, made silently.
pub fn locations(
  defs: &[Symbol],
  reader: &dyn Resolver,
) -> BTreeMap<String, Option<(String, u32)>> {
  let mut by_target: BTreeMap<String, Vec<&Symbol>> = BTreeMap::new();
  for def in defs {
    if let Some(target) = reader.target_of(def) {
      by_target.entry(target).or_default().push(def);
    }
  }
  by_target
    .into_iter()
    .map(|(target, rows)| {
      let place = reader
        .locate(&rows)
        .map(|def| (def.path.clone(), def.span.start_line));
      (target, place)
    })
    .collect()
}

/// The reasons a trace excludes references for that its reader did not
/// declare, in order. Empty when the trace keeps to the roster.
pub fn undeclared<'a>(trace: &'a Trace, declared: &[&str]) -> Vec<&'a str> {
  trace
    .excluded
    .keys()
    .map(String::as_str)
    .filter(|reason| !declared.contains(reason))
    .collect()
}

/// What `intent index resolve` answers: the record of every language the run
/// covered, and each declared language it did not run because the project
/// holds nothing for its tool.
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize)]
pub struct Outcome {
  pub resolution: BTreeMap<String, Run>,
  /// Language to why, for the languages a run over every declared language
  /// found nothing to resolve in. Their records are left as they were.
  pub not_applicable: BTreeMap<String, String>,
}

/// Decide what a run stores.
///
/// `indexed` is the hash the index read each file at, for the files it holds
/// in this language; `written` is every written reference in them; `located`
/// is [`locations`] over its definitions.
pub fn join(
  trace: &Trace,
  indexed: &BTreeMap<String, String>,
  written: &BTreeSet<Key>,
  located: &BTreeMap<String, Option<(String, u32)>>,
) -> Joined {
  let mut tally = Tally::default();
  for (reason, n) in &trace.excluded {
    tally.drop_for(reason, *n);
  }

  let mut read: BTreeMap<&str, &Read> = BTreeMap::new();
  for file in &trace.read {
    read.entry(file.path.as_str()).or_insert(file);
  }

  let mut rows: BTreeMap<(&str, u32, &str, &str), Row> = BTreeMap::new();
  for reference in &trace.references {
    let Some(line) = reference.line else {
      tally.drop_for(NO_LINE, 1);
      continue;
    };
    let Some(path) = reference.path.as_deref() else {
      tally.drop_for(OUTSIDE_THE_PROJECT, 1);
      continue;
    };
    let Some(file) = read.get(path) else {
      tally.drop_for(UNREAD, 1);
      continue;
    };
    match indexed.get(path) {
      None => {
        tally.drop_for(NOT_INDEXED, 1);
        continue;
      }
      Some(sha) if *sha != file.sha256 => {
        tally.drop_for(MOVED, 1);
        continue;
      }
      Some(_) => {}
    }
    if !written.contains(&(path.to_string(), line, reference.name.clone())) {
      tally.unmatched += 1;
      continue;
    }
    tally.matched += 1;
    rows
      .entry((
        path,
        line,
        reference.name.as_str(),
        reference.target.as_str(),
      ))
      .or_insert_with(|| {
        let (target_path, target_line) = match (&reference.target_path, reference.target_line) {
          (None, None) => match located.get(&reference.target) {
            Some(Some((p, l))) => (Some(p.clone()), Some(*l)),
            _ => (None, None),
          },
          (p, l) => (p.clone(), l),
        };
        Row {
          path: path.to_string(),
          line,
          name: reference.name.clone(),
          target: reference.target.clone(),
          target_path,
          target_line,
        }
      });
  }

  let mut targets: BTreeMap<(&str, u32, &str), u64> = BTreeMap::new();
  for (path, line, name, _) in rows.keys() {
    *targets.entry((*path, *line, *name)).or_default() += 1;
  }
  tally.ambiguous = targets.values().filter(|n| **n > 1).count() as u64;

  Joined {
    files: read
      .into_values()
      .filter(|file| indexed.get(&file.path) == Some(&file.sha256))
      .cloned()
      .collect(),
    rows: rows.into_values().collect(),
    tally,
  }
}

#[cfg(test)]
mod tests {
  use super::super::symbols::{Span, SymbolKind};
  use super::*;

  fn reference(path: Option<&str>, line: Option<u32>, name: &str, target: &str) -> Reference {
    Reference {
      path: path.map(str::to_string),
      line,
      name: name.to_string(),
      target: target.to_string(),
      target_path: None,
      target_line: None,
    }
  }

  fn read(path: &str, sha256: &str) -> Read {
    Read {
      path: path.to_string(),
      sha256: sha256.to_string(),
    }
  }

  fn def(path: &str, name: &str, line: u32) -> Symbol {
    Symbol {
      path: path.to_string(),
      lang: "rust",
      name: name.to_string(),
      kind: SymbolKind::Def,
      span: Span {
        start_line: line,
        end_line: line,
      },
      subkind: "function".to_string(),
      container: None,
      container_kind: None,
      trait_name: None,
      arity: Some(0),
      arity_min: Some(0),
      qualifier: None,
      level: 1,
    }
  }

  #[test]
  fn the_counts_partition_everything_the_tool_emitted_and_a_key_naming_two_targets_keeps_both() {
    let trace = Trace {
      read: vec![
        read("src/lib.rs", "aa"),
        read("src/moved.rs", "old"),
        read("src/unheld.rs", "cc"),
      ],
      references: vec![
        reference(Some("src/lib.rs"), Some(3), "expect", "Option::expect"),
        reference(Some("src/lib.rs"), Some(3), "expect", "Result::expect"),
        reference(Some("src/lib.rs"), Some(3), "expect", "Result::expect"),
        reference(Some("src/lib.rs"), Some(4), "field", "Widget::field"),
        reference(Some("src/moved.rs"), Some(1), "run", "run"),
        reference(Some("src/unheld.rs"), Some(1), "run", "run"),
        reference(Some("src/never.rs"), Some(1), "run", "run"),
        reference(None, Some(1), "run", "run"),
        reference(Some("src/lib.rs"), None, "run", "run"),
      ],
      excluded: BTreeMap::from([("operator".to_string(), 5)]),
    };
    let indexed = BTreeMap::from([
      ("src/lib.rs".to_string(), "aa".to_string()),
      ("src/moved.rs".to_string(), "new".to_string()),
    ]);
    let written = BTreeSet::from([
      ("src/lib.rs".to_string(), 3, "expect".to_string()),
      ("src/moved.rs".to_string(), 1, "run".to_string()),
    ]);

    let joined = join(&trace, &indexed, &written, &BTreeMap::new());

    let t = &joined.tally;
    assert_eq!(
      t.matched + t.unmatched + t.dropped,
      trace.references.len() as u64 + trace.excluded.values().sum::<u64>(),
      "THE CONSERVATION LAW: every reference the tool emitted is counted exactly once"
    );
    let by = |pairs: &[(&str, u64)]| {
      pairs
        .iter()
        .map(|(r, n)| (r.to_string(), *n))
        .collect::<BTreeMap<_, _>>()
    };
    assert_eq!(
      joined.tally,
      Tally {
        matched: 3,
        unmatched: 1,
        dropped: 10,
        ambiguous: 1,
        dropped_by: by(&[
          ("moved", 1),
          ("no-line", 1),
          ("not-indexed", 1),
          ("operator", 5),
          ("outside-the-project", 1),
          ("unread", 1),
        ]),
      },
      "matched, unmatched and dropped sum to the nine references and five exclusions"
    );
    assert_eq!(
      joined
        .rows
        .iter()
        .map(|r| (r.line, r.target.as_str()))
        .collect::<Vec<_>>(),
      vec![(3, "Option::expect"), (3, "Result::expect")],
      "both targets are stored and the repeated one collapses onto its row"
    );
    assert_eq!(
      joined
        .files
        .iter()
        .map(|f| f.path.as_str())
        .collect::<Vec<_>>(),
      vec!["src/lib.rs"],
      "a file whose bytes moved, or that the index does not hold, is not joined, \
       so no earlier rows of it are replaced"
    );
  }

  /// A reader that prints a definition as `A.<name>/<arity>` and keeps the
  /// core's exactly-one choice.
  struct Printing;

  impl Resolver for Printing {
    fn lang(&self) -> &'static str {
      "elixir"
    }
    fn tool(&self) -> &'static str {
      "fixture"
    }
    fn trace(&self, _: &Scope<'_>) -> Result<Trace, Unresolved> {
      Ok(Trace::default())
    }
    fn target_of(&self, def: &Symbol) -> Option<String> {
      Some(format!(
        "A.{}/{}",
        def.name,
        if def.name == "get" { 1 } else { 2 }
      ))
    }
  }

  #[test]
  fn a_target_the_tool_does_not_locate_is_located_by_the_one_definition_that_prints_it() {
    let trace = Trace {
      read: vec![read("lib/a.ex", "aa")],
      references: vec![
        reference(Some("lib/a.ex"), Some(2), "get", "A.get/1"),
        reference(Some("lib/a.ex"), Some(3), "put", "A.put/2"),
      ],
      excluded: BTreeMap::new(),
    };
    let indexed = BTreeMap::from([("lib/a.ex".to_string(), "aa".to_string())]);
    let written = BTreeSet::from([
      ("lib/a.ex".to_string(), 2, "get".to_string()),
      ("lib/a.ex".to_string(), 3, "put".to_string()),
    ]);
    let defs = [
      def("lib/a.ex", "get", 9),
      def("lib/a.ex", "put", 12),
      def("test/support/a.ex", "put", 4),
    ];
    let located = locations(&defs, &Printing);

    let joined = join(&trace, &indexed, &written, &located);

    assert_eq!(
      joined
        .rows
        .iter()
        .map(|r| (r.name.as_str(), r.target_path.as_deref(), r.target_line))
        .collect::<Vec<_>>(),
      vec![("get", Some("lib/a.ex"), Some(9)), ("put", None, None)],
      "two definitions printing one target, which the reader does not choose between, \
       locate it nowhere rather than at a guess"
    );
  }
}
