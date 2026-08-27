//! **A test that drives verbs it does not name must fixture the ambients it
//! cannot predict.**
//!
//! # The incident this exists because of
//!
//! On 2026-08-27 `~/.intent/home` -- the machine-global install pointer the
//! pre-commit shim resolves on every commit, in every estate on the machine --
//! named a scratch worktree that had been deleted. Written by two arms of
//! `dispatch_ssot`, which build their argument vectors from
//! `surface/dispatch-table.json` and drive every shipped family bare, hunting
//! for the ones that answer *is a known command that is not implemented yet*.
//!
//! **`bootstrap` answered exactly that until `431590a3` gave it an
//! implementation.** Nothing in the test changed. **The subject changed
//! underneath it**, and a probe for a refusal became a live setup command
//! against the operator's machine. The fix was `9c2ba9ed`; this file is why the
//! fix is not the whole answer.
//!
//! # Why this predicate and not "every test that spawns the binary"
//!
//! Measured before it was chosen: **43 test files spawn `intent` and 30 of them
//! never fixture `HOME`.** A blanket rule would red thirty files across four
//! nodes' lanes to describe a hazard that does not apply to most of them -- a
//! test that spells `st list` can only ever run `st list`, and `st list` will
//! not grow a per-user write.
//!
//! **The hazard is not spawning the binary. It is spawning verbs chosen by
//! DATA.** A table-driven test's reach is the table's contents at run time, so
//! its blast radius grows on somebody else's commit, in a file its author never
//! touched. That is the population this guard binds, and it is nine files.
//!
//! # What it cannot do
//!
//! It reads source text, so it establishes that the fixture is SPELLED, not
//! that every spawn in the file uses it. A file with two runners and one
//! fixtured passes here. That limit is real and is why `testkit::fixture_home`
//! exists as one shared helper rather than as advice: the cheap thing to do and
//! the correct thing to do are the same call.
//!
//! Its own honesty check is below -- a scan whose population is empty passes
//! for free, so the population is asserted before the property is.

use std::path::{Path, PathBuf};

use testkit::workspace_root;

/// Every `.rs` under any crate's `tests/`.
fn test_sources() -> Vec<PathBuf> {
  fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
      return;
    };
    for entry in entries.flatten() {
      let path = entry.path();
      if path.is_dir() {
        walk(&path, out);
      } else if path.extension().is_some_and(|e| e == "rs") {
        out.push(path);
      }
    }
  }
  let mut out = Vec::new();
  for entry in std::fs::read_dir(workspace_root().join("crates"))
    .expect("read the crates dir")
    .flatten()
  {
    let tests = entry.path().join("tests");
    if tests.is_dir() {
      walk(&tests, &mut out);
    }
  }
  out.sort();
  out
}

/// Spawns the binary, by either of the two idioms in use.
///
/// **`Command::new` IS PART OF THE PREDICATE BECAUSE THIS FILE FAILED ITSELF
/// WITHOUT IT.** Naming the binary is not spawning it: THIS guard mentions both
/// idioms, as string literals, in the very functions that look for them -- so
/// on its first run it reported itself as an unfixtured table-driven test. A
/// scan whose corpus includes the scanner is a scan that will one day be
/// satisfied by editing its own prose, which is the opposite of what it is for.
fn spawns_the_binary(src: &str) -> bool {
  src.contains("Command::new")
    && (src.contains("CARGO_BIN_EXE_intent") || src.contains("target/debug/intent"))
}

/// Chooses what to run from the dispatch table rather than by naming it.
fn driven_by_the_table(src: &str) -> bool {
  src.contains("dispatch::table()") || src.contains("dispatch-table.json")
}

fn fixtures_home(src: &str) -> bool {
  src.contains(".env(\"HOME\"")
}

/// This file's own name, so the scanner is not in its own corpus.
///
/// **DERIVED FROM `file!()` RATHER THAN SPELLED**, so renaming this file cannot
/// silently re-admit it -- a hardcoded name would stop matching and the
/// exclusion would quietly stop applying, which is the same class of silent
/// drift the guard exists to catch.
fn this_file() -> &'static str {
  Path::new(file!())
    .file_name()
    .and_then(|n| n.to_str())
    .expect("this file has a name")
}

/// The files this guard binds: they spawn the binary AND choose verbs by data.
///
/// **THE SCANNER EXCLUDES ITSELF, AND IT LEARNED TO THE HARD WAY -- TWICE.**
/// Every marker this file looks for appears IN this file, as a string literal
/// in the function that looks for it, so the first two runs reported the guard
/// as its own only violation. Tightening the predicate did not help and could
/// not: any predicate precise enough to describe the hazard is a predicate this
/// file must spell out in full.
fn population() -> Vec<(PathBuf, String)> {
  test_sources()
    .into_iter()
    .filter(|p| p.file_name().and_then(|n| n.to_str()) != Some(this_file()))
    .filter_map(|p| {
      let src = std::fs::read_to_string(&p).ok()?;
      (spawns_the_binary(&src) && driven_by_the_table(&src)).then_some((p, src))
    })
    .collect()
}

/// **THE POPULATION IS ASSERTED BEFORE THE PROPERTY IS.**
///
/// A scan whose filter matches nothing passes for free and reports a clean
/// estate -- the failure this repository has met from four directions and has
/// no wish to meet from a fifth. If a refactor renames `dispatch::table()` or
/// changes how tests reach the binary, this arm goes red rather than the guard
/// going quietly blind.
#[test]
fn the_scan_can_see_the_files_it_is_meant_to_bind() {
  let found = population();
  assert!(
    found.len() >= 8,
    "the guard found only {} table-driven test files. It was written against 9, and \
     a scan that has stopped matching reports a clean estate rather than an error: {:?}",
    found.len(),
    found.iter().map(|(p, _)| p.file_name()).collect::<Vec<_>>()
  );
  assert!(
    found.iter().any(|(p, _)| p.ends_with("dispatch_ssot.rs")),
    "the file the incident came from must be in the population, or the guard does \
     not bind the case it was written for"
  );
}

#[test]
fn a_test_that_drives_verbs_it_does_not_name_fixtures_home() {
  let unfixtured: Vec<String> = population()
    .into_iter()
    .filter(|(_, src)| !fixtures_home(src))
    .map(|(p, _)| {
      p.file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned()
    })
    .collect();

  assert!(
    unfixtured.is_empty(),
    "these tests choose which verbs to run from the dispatch table and inherit the \
     operator's real HOME: {unfixtured:?}\n\
     \n\
     Their reach is the table's contents AT RUN TIME, so a verb implemented later \
     is a verb they will drive -- which is exactly how `dispatch_ssot` came to \
     publish this machine's install pointer to a scratch worktree that was then \
     deleted (2026-08-27; fixed at `9c2ba9ed`).\n\
     \n\
     Remedy: `.env(\"HOME\", testkit::fixture_home())` on the command."
  );
}
