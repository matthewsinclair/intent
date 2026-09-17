//! AT-05.1 / AC-05.1 (ST0076 WP-05): the level-3 core (vc decision 25, and vc's
//! rulings of 2026-09-17 on dc's review of the reader contract): what a
//! resolution run stores, replaces, keeps and reports, through the one door
//! both languages use.
//!
//! **PROVEN WITH AN IN-MEMORY READER.** A toolchain is a reader's concern and
//! has its own arms; these hand the door a trace, so what they prove is the
//! part every language shares: the join against written references and its
//! conservation law, the per-file replace, the failure record, staleness, the
//! purge, and when a run is forced full.

use crate::common::{Fixture, git_init_at};
use intentsvcs::facade::{Facade, FacadeError};
use intentsvcs::index::resolved::{
  Joined, Outcome, Read, Reference, Resolver, Scope, Tally, Trace, Unresolved,
};
use intentsvcs::index::symbols::{EXTRACTOR_VERSION, Symbol};
use intentsvcs::model::sha256_hex;
use intentsvcs::store::Store;
use std::cell::Cell;
use std::collections::BTreeMap;
use std::rc::Rc;

const LIB: &str = "fn main() {\n  helper();\n  other();\n}\nfn helper() {}\n";
const TWO: &str = "fn two() {\n  helper();\n}\n";

/// A reader that hands back the outcome it was built with, and remembers
/// whether the door asked it for a full run.
struct Handed {
  outcome: Result<Trace, Unresolved>,
  saw_full: Rc<Cell<Option<bool>>>,
}

impl Resolver for Handed {
  fn lang(&self) -> &'static str {
    "rust"
  }
  fn tool(&self) -> &'static str {
    "fixture-analyzer"
  }
  fn manifest(&self) -> intentsvcs::index::resolved::Manifest {
    intentsvcs::index::rust_analyzer::MANIFEST
  }
  fn trace(&self, scope: &Scope<'_>) -> Result<Trace, Unresolved> {
    assert!(
      scope.cache.ends_with("intent/.cache/resolve/rust") && scope.cache.is_dir(),
      "the door creates Intent's own build directory before the tool runs: {scope:?}"
    );
    assert!(
      scope.indexed.iter().any(|p| p == "src/lib.rs"),
      "the reader is handed the paths the index holds, to find its project in: {scope:?}"
    );
    self.saw_full.set(Some(scope.full));
    self.outcome.clone()
  }
  fn excludes(&self) -> &'static [&'static str] {
    &["operator"]
  }
  fn target_of(&self, def: &Symbol) -> Option<String> {
    Some(format!("crate::{}", def.name))
  }
}

/// Run the door over a reader handing back `outcome`, returning what it
/// answered and whether it asked for a full run.
fn resolve(
  facade: &mut Facade,
  lang: Option<&str>,
  outcome: Result<Trace, Unresolved>,
) -> (Outcome, Option<bool>) {
  let saw_full = Rc::new(Cell::new(None));
  let readers: Vec<Box<dyn Resolver>> = vec![Box::new(Handed {
    outcome,
    saw_full: Rc::clone(&saw_full),
  })];
  let answer = facade
    .index_resolve(lang, false, &readers)
    .expect("the door answers");
  (answer, saw_full.get())
}

/// A project declaring rust, with both files indexed by the real extractor.
fn indexed() -> (Fixture, Facade) {
  let fx = Fixture::new();
  git_init_at(fx.root());
  fx.write_file(".gitignore", "intent/.cache/\n");
  fx.write_file("src/lib.rs", LIB);
  fx.write_file("src/two.rs", TWO);
  let mut facade = fx.facade_on_disk();
  facade.index_rebuild().expect("rebuild");
  (fx, facade)
}

fn read(path: &str, text: &str) -> Read {
  Read {
    path: path.to_string(),
    sha256: sha256_hex(text.as_bytes()),
  }
}

fn to(path: &str, line: Option<u32>, name: &str, target: &str) -> Reference {
  Reference {
    path: Some(path.to_string()),
    line,
    name: name.to_string(),
    target: target.to_string(),
    target_path: None,
    target_line: None,
  }
}

/// Both files read, and one reference joined in each.
fn both() -> Trace {
  Trace {
    read: vec![read("src/lib.rs", LIB), read("src/two.rs", TWO)],
    references: vec![
      to("src/lib.rs", Some(2), "helper", "crate::helper"),
      to("src/two.rs", Some(2), "helper", "crate::helper"),
    ],
    excluded: BTreeMap::new(),
  }
}

#[test]
fn a_run_stores_the_references_that_join_a_written_one_and_counts_every_other_once() {
  let (fx, mut facade) = indexed();
  let trace = Trace {
    read: vec![read("src/lib.rs", LIB)],
    references: vec![
      to("src/lib.rs", Some(2), "helper", "crate::helper"),
      to("src/lib.rs", Some(3), "other", "crate::a::other"),
      to("src/lib.rs", Some(3), "other", "crate::b::other"),
      to("src/lib.rs", Some(1), "String", "alloc::string::String"),
      to("src/lib.rs", None, "helper", "crate::helper"),
    ],
    excluded: BTreeMap::from([("operator".to_string(), 2)]),
  };

  let (outcome, full) = resolve(&mut facade, None, Ok(trace.clone()));

  let run = &outcome.resolution["rust"];
  let t = &run.tally;
  assert_eq!(
    t.matched + t.unmatched + t.dropped,
    trace.references.len() as u64 + 2,
    "THE CONSERVATION LAW: the five references and two exclusions are each counted once"
  );
  assert_eq!(
    (run.state.as_str(), run.run, t),
    (
      "current",
      1,
      &Tally {
        matched: 3,
        unmatched: 1,
        dropped: 3,
        ambiguous: 1,
        dropped_by: BTreeMap::from([("no-line".to_string(), 1), ("operator".to_string(), 2)]),
      }
    ),
    "{run:?}"
  );
  assert_eq!(
    fx.resolved_in("src/lib.rs"),
    vec![
      (
        2,
        "helper".to_string(),
        "crate::helper".to_string(),
        Some("src/lib.rs".to_string()),
        Some(5)
      ),
      (
        3,
        "other".to_string(),
        "crate::a::other".to_string(),
        None,
        None
      ),
      (
        3,
        "other".to_string(),
        "crate::b::other".to_string(),
        None,
        None
      ),
    ],
    "a line calling two definitions of one name stores both, and a target the \
     tool did not locate is placed at the one definition that prints it"
  );
  assert_eq!(
    facade
      .index_status()
      .expect("status")
      .resolution
      .get("rust"),
    Some(run),
    "index status reads back what the run reported"
  );
  assert_eq!(
    full,
    Some(true),
    "a language with no stored run is run full, whatever the caller asked"
  );
}

#[test]
fn a_run_replaces_the_files_it_read_and_keeps_every_other_file() {
  let (fx, mut facade) = indexed();
  resolve(&mut facade, None, Ok(both()));

  let only_two = Trace {
    read: vec![read("src/two.rs", TWO)],
    ..Trace::default()
  };
  let (outcome, full) = resolve(&mut facade, None, Ok(only_two));

  assert_eq!(outcome.resolution["rust"].run, 2);
  assert_eq!(
    full,
    Some(false),
    "a stored, current run under this extractor lets the next one be incremental"
  );
  assert!(
    fx.resolved_in("src/two.rs").is_empty(),
    "the file the run read resolves nothing now, so its earlier row is replaced by none"
  );
  assert_eq!(
    fx.resolved_in("src/lib.rs").len(),
    1,
    "the file the run did not read keeps what the first run stored"
  );
}

/// Issue 0440: the counts stay the stored run's, and status says how many
/// of the files holding the language's rows that run joined.
#[test]
fn status_says_how_many_of_the_files_holding_rows_the_stored_run_joined() {
  let (_fx, mut facade) = indexed();
  resolve(&mut facade, None, Ok(both()));
  let only_two = Trace {
    read: vec![read("src/two.rs", TWO)],
    references: vec![to("src/two.rs", Some(2), "helper", "crate::helper")],
    excluded: BTreeMap::new(),
  };
  resolve(&mut facade, None, Ok(only_two));

  let status = facade.index_status().expect("status");
  let run = &status.resolution["rust"];
  assert_eq!(
    (run.run, run.files, run.joined, &run.tally),
    (
      2,
      2,
      1,
      &Tally {
        matched: 1,
        ..Tally::default()
      }
    ),
    "the second run joined one of the two files, and the counts are that one file's: {run:?}"
  );
}

#[test]
fn a_run_that_fails_writes_its_record_and_nothing_else_and_the_next_run_is_full() {
  let (fx, mut facade) = indexed();
  let stored = resolve(&mut facade, None, Ok(both())).0.resolution["rust"].clone();

  let failed = Unresolved::Failed {
    path: Some("src/lib.rs".to_string()),
    line: Some(3),
    detail: "the build script panicked".to_string(),
  };
  let run = resolve(&mut facade, None, Err(failed)).0.resolution["rust"].clone();
  assert_eq!(
    (
      run.state.as_str(),
      run.path.as_deref(),
      run.line,
      run.detail.as_deref()
    ),
    (
      "failed",
      Some("src/lib.rs"),
      Some(3),
      Some("the build script panicked")
    )
  );
  assert_eq!(
    (&run.resolved_at, run.run, &run.tally),
    (&stored.resolved_at, stored.run, &stored.tally),
    "the counts and the stamp stay the stored run's"
  );
  assert_eq!(fx.resolved_in("src/lib.rs").len(), 1, "and so do its rows");

  let missing = Unresolved::Missing {
    detail: "fixture-analyzer is not on PATH".to_string(),
  };
  let (outcome, full) = resolve(&mut facade, None, Err(missing));
  let run = &outcome.resolution["rust"];
  assert_eq!(
    (run.state.as_str(), run.detail.as_deref()),
    ("missing", Some("fixture-analyzer is not on PATH"))
  );
  assert_eq!(full, Some(true), "the run after a failure is run full");
}

#[test]
fn a_reason_the_reader_did_not_declare_fails_the_run() {
  let (_fx, mut facade) = indexed();
  let trace = Trace {
    excluded: BTreeMap::from([("macro-noise".to_string(), 4)]),
    ..both()
  };
  let run = resolve(&mut facade, None, Ok(trace)).0.resolution["rust"].clone();
  assert_eq!(run.state, "failed");
  assert!(
    run
      .detail
      .as_deref()
      .is_some_and(|d| d.contains("`macro-noise`")),
    "the published words are a closed roster, so the undeclared one is named: {run:?}"
  );
}

#[test]
fn an_edited_file_is_named_stale_until_a_run_reads_its_new_bytes() {
  let (fx, mut facade) = indexed();
  resolve(&mut facade, None, Ok(both()));

  let edited = format!("{LIB}fn later() {{}}\n");
  fx.write_file("src/lib.rs", &edited);
  facade.index_refresh(None).expect("reconcile");

  let status = facade.index_status().expect("status");
  assert_eq!(
    status.resolution["rust"].stale,
    vec!["src/lib.rs".to_string()],
    "the rows were resolved against bytes the index no longer holds"
  );
  assert_eq!(
    fx.resolved_in("src/lib.rs").len(),
    1,
    "and a reconcile does not delete them"
  );

  let reread = Trace {
    read: vec![read("src/lib.rs", &edited)],
    references: vec![to("src/lib.rs", Some(2), "helper", "crate::helper")],
    excluded: BTreeMap::new(),
  };
  let (outcome, _) = resolve(&mut facade, None, Ok(reread));
  assert!(
    outcome.resolution["rust"].stale.is_empty(),
    "{:?}",
    outcome.resolution["rust"]
  );
}

#[test]
fn a_run_under_another_extractor_leaves_every_file_stale_and_the_next_run_full() {
  let (fx, mut facade) = indexed();
  let joined = Joined {
    files: both().read,
    ..Joined::default()
  };
  Store::open(&fx.project().db_path())
    .expect("store")
    .replace_resolved("rust", "fixture-analyzer", EXTRACTOR_VERSION - 1, &joined)
    .expect("a run an older build stored");

  assert_eq!(
    facade.index_status().expect("status").resolution["rust"].stale,
    vec!["src/lib.rs".to_string(), "src/two.rs".to_string()],
    "the written rows that run joined have been re-extracted since, in every file"
  );
  let (outcome, full) = resolve(&mut facade, None, Ok(both()));
  assert_eq!(full, Some(true));
  assert!(outcome.resolution["rust"].stale.is_empty());
}

#[test]
fn a_file_that_left_the_index_keeps_its_rows_until_the_next_run_purges_them() {
  let (fx, mut facade) = indexed();
  resolve(&mut facade, None, Ok(both()));

  std::fs::remove_file(fx.path("src/two.rs")).expect("remove");
  facade.index_refresh(None).expect("reconcile");
  assert_eq!(
    fx.resolved_in("src/two.rs").len(),
    1,
    "a reconcile never deletes a resolved row"
  );
  assert_eq!(
    facade.index_status().expect("status").resolution["rust"].stale,
    vec!["src/two.rs".to_string()]
  );

  let lib_only = Trace {
    read: vec![read("src/lib.rs", LIB)],
    references: vec![to("src/lib.rs", Some(2), "helper", "crate::helper")],
    excluded: BTreeMap::new(),
  };
  let (outcome, _) = resolve(&mut facade, None, Ok(lib_only));
  assert!(
    fx.resolved_in("src/two.rs").is_empty(),
    "the run purges the path that left the index"
  );
  assert!(
    outcome.resolution["rust"].stale.is_empty(),
    "{:?}",
    outcome.resolution["rust"]
  );
}

#[test]
fn a_language_the_project_holds_nothing_for_is_skipped_unless_asked_for_by_name() {
  let (_fx, mut facade) = indexed();
  let absent = || Unresolved::NotApplicable {
    detail: "no Cargo workspace under the project root".to_string(),
  };

  let (outcome, _) = resolve(&mut facade, None, Err(absent()));
  assert_eq!(
    (
      outcome.resolution.len(),
      outcome.not_applicable.get("rust").map(String::as_str)
    ),
    (0, Some("no Cargo workspace under the project root")),
    "a run over every declared language names it and writes nothing"
  );
  assert!(
    facade.index_status().expect("status").resolution.is_empty(),
    "so no record says a tier failed that nobody asked for"
  );

  let (outcome, _) = resolve(&mut facade, Some("rust"), Err(absent()));
  assert_eq!(
    outcome.resolution["rust"].state, "failed",
    "asked for by name, it is a failure"
  );
}

#[test]
fn a_language_this_build_cannot_resolve_is_refused_by_name() {
  let (_fx, mut facade) = indexed();
  let readers: Vec<Box<dyn Resolver>> = vec![Box::new(Handed {
    outcome: Ok(both()),
    saw_full: Rc::new(Cell::new(None)),
  })];
  let err = facade
    .index_resolve(Some("elixir"), false, &readers)
    .expect_err("no elixir reader");
  assert!(
    matches!(&err, FacadeError::NoResolver { asked, built } if asked == "`elixir`" && built == "`rust`"),
    "the refusal names what was asked and what this build carries: {err:?}"
  );
}
