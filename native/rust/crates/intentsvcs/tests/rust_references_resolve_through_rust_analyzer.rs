//! AT-05.2 / AC-05.1 (ST0076 WP-05): Rust's reader, rust-analyzer's SCIP
//! export read into level 3 through the one door both languages use.
//!
//! **HERMETIC WHERE IT CAN BE, AND ONE ARM THAT IS NOT.** The export of a tiny
//! crate is checked in beside its sources (`tests/fixtures/scip/tiny/`), so the
//! decode and the join run in every build; the absence of the tool is proven
//! by naming a program that does not exist. Only the arm under
//! `needs_the_toolchain` runs the real rust-analyzer, and it is ignored in CI
//! and run by `bin/devbin test all` (hv, 2026-09-17).

use crate::common::{Fixture, git_init_at};
use intentsvcs::index::resolved::{Outcome, Resolver, Scope, Trace, Unresolved};
use intentsvcs::index::rust_analyzer::{self, RustAnalyzer};
use intentsvcs::index::scip;
use std::collections::BTreeMap;

const CARGO_TOML: &str = include_str!("fixtures/scip/tiny/Cargo.toml");
const CARGO_LOCK: &str = include_str!("fixtures/scip/tiny/Cargo.lock");
const LIB: &str = include_str!("fixtures/scip/tiny/src/lib.rs");
const STORE: &str = include_str!("fixtures/scip/tiny/src/store.rs");
/// `rust-analyzer scip` over the crate above, with the index's metadata
/// removed so the file names no machine's paths.
const EXPORT: &[u8] = include_bytes!("fixtures/scip/tiny/tiny.scip");

/// A project holding the tiny crate at its root, indexed by the real extractor.
fn tiny() -> (Fixture, intentsvcs::facade::Facade) {
  let fx = Fixture::new();
  git_init_at(fx.root());
  fx.write_file(".gitignore", "intent/.cache/\ntarget/\n");
  fx.write_file("Cargo.toml", CARGO_TOML);
  fx.write_file("Cargo.lock", CARGO_LOCK);
  fx.write_file("src/lib.rs", LIB);
  fx.write_file("src/store.rs", STORE);
  let mut facade = fx.facade_on_disk();
  facade.index_rebuild().expect("rebuild");
  (fx, facade)
}

/// A reader that reads the checked-in export instead of running the tool, and
/// the source bytes the export was made from.
struct CheckedIn;

impl Resolver for CheckedIn {
  fn lang(&self) -> &'static str {
    "rust"
  }
  fn tool(&self) -> &'static str {
    "rust-analyzer"
  }
  fn excludes(&self) -> &'static [&'static str] {
    RustAnalyzer::default().excludes()
  }
  fn trace(&self, scope: &Scope<'_>) -> Result<Trace, Unresolved> {
    let index = scip::decode(EXPORT).expect("the checked-in export decodes");
    let bytes = BTreeMap::from([
      ("src/lib.rs".to_string(), LIB.as_bytes().to_vec()),
      ("src/store.rs".to_string(), STORE.as_bytes().to_vec()),
    ]);
    assert_eq!(rust_analyzer::roots(scope.indexed), vec![String::new()]);
    rust_analyzer::trace_of("", &index, &bytes)
  }
}

fn resolve(
  facade: &mut intentsvcs::facade::Facade,
  lang: Option<&str>,
  reader: Box<dyn Resolver>,
) -> Outcome {
  facade
    .index_resolve(lang, false, &[reader])
    .expect("the door answers")
}

#[test]
fn the_checked_in_export_resolves_the_crates_calls_to_the_definitions_they_name() {
  let (fx, mut facade) = tiny();
  let outcome = resolve(&mut facade, None, Box::new(CheckedIn));

  let run = &outcome.resolution["rust"];
  let emitted = scip::decode(EXPORT)
    .expect("decodes")
    .documents
    .iter()
    .flat_map(|d| &d.occurrences)
    .filter(|o| !o.is_definition())
    .count() as u64;
  let t = &run.tally;
  assert_eq!(
    t.matched + t.unmatched + t.dropped,
    emitted,
    "THE CONSERVATION LAW, on a real export: every reference occurrence counted once: {run:?}"
  );
  assert_eq!(run.state, "current", "{run:?}");

  let lib = fx.resolved_in("src/lib.rs");
  for expected in [
    (
      18,
      "new",
      "tiny::Widget::new()",
      Some("src/lib.rs"),
      Some(8),
    ),
    (
      19,
      "open",
      "tiny::store::open()",
      Some("src/store.rs"),
      Some(1),
    ),
    (
      20,
      "size",
      "tiny::Widget::size()",
      Some("src/lib.rs"),
      Some(12),
    ),
  ] {
    let (line, name, target, path, at) = expected;
    assert!(
      lib.contains(&(
        line,
        name.to_string(),
        target.to_string(),
        path.map(str::to_string),
        at
      )),
      "{expected:?} missing from {lib:#?}"
    );
  }
  assert!(
    !lib.iter().any(|(line, name, target, _, _)| *line == 20
      && name == "size"
      && target == "tiny::Widget::size"),
    "the method call resolves to the method and not to the field of the same name: {lib:#?}"
  );
  assert_eq!(
    t.dropped_by.get(rust_analyzer::LOCAL),
    Some(&4),
    "the four uses of locals are counted, by reason, and not stored: {run:?}"
  );
  assert_eq!(
    t.dropped_by.get(rust_analyzer::OPERATOR),
    Some(&3),
    "`widget.size() + opened.len()` calls `add` through the `+` and the space either side of it, \
     three references no written row can name, counted by reason: {run:?}"
  );
}

#[test]
fn a_missing_rust_analyzer_is_named_and_never_answers_as_an_empty_tier() {
  let (_fx, mut facade) = tiny();
  let absent = RustAnalyzer {
    program: "rust-analyzer-that-is-not-installed".into(),
  };
  let outcome = resolve(&mut facade, Some("rust"), Box::new(absent));
  let run = &outcome.resolution["rust"];
  assert_eq!(run.state, "missing", "{run:?}");
  assert!(
    run
      .detail
      .as_deref()
      .is_some_and(|d| d.contains("rust-analyzer-that-is-not-installed")),
    "the record names the program it could not run: {run:?}"
  );
}

#[test]
fn a_project_holding_no_cargo_manifest_is_not_applicable() {
  let fx = Fixture::new();
  git_init_at(fx.root());
  fx.write_file(".gitignore", "intent/.cache/\n");
  fx.write_file("notes.md", "no crate here\n");
  let mut facade = fx.facade_on_disk();
  facade.index_rebuild().expect("rebuild");
  let outcome = resolve(&mut facade, None, Box::new(RustAnalyzer::default()));
  assert_eq!(
    outcome.not_applicable.get("rust").map(String::as_str),
    Some("the index holds no Cargo.toml"),
    "a Rust-less project is named, and rust-analyzer is never run over it: {outcome:?}"
  );
}

/// **RUN BY `bin/devbin test all`, NEVER IN CI** (hv, 2026-09-17): the real
/// rust-analyzer over the tiny crate, end to end. It fails by name when the
/// tool is absent rather than skipping.
mod needs_the_toolchain {
  use super::*;

  #[test]
  #[ignore = "needs rust-analyzer on PATH; bin/devbin test all runs it"]
  fn rust_analyzer_resolves_the_tiny_crate_end_to_end() {
    let (fx, mut facade) = tiny();
    let outcome = resolve(&mut facade, None, Box::new(RustAnalyzer::default()));
    let run = &outcome.resolution["rust"];
    assert_eq!(
      run.state, "current",
      "this arm needs rust-analyzer on PATH, and the run did not store: {run:?}"
    );
    assert!(
      fx.resolved_in("src/lib.rs")
        .iter()
        .any(|(line, name, target, path, at)| {
          *line == 19
            && name == "open"
            && target == "tiny::store::open()"
            && path.as_deref() == Some("src/store.rs")
            && *at == Some(1)
        }),
      "the real export resolves the module-qualified call: {:#?}",
      fx.resolved_in("src/lib.rs")
    );
    assert!(
      fx.root().join("intent/.cache/resolve/rust/target").is_dir(),
      "the export built into Intent's own directory, never the crate's `target/`"
    );
    assert!(
      !fx.root().join("target").exists(),
      "and nothing was built into the crate's own target directory"
    );
  }
}
