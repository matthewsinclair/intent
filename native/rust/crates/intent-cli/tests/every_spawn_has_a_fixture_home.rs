//! **Every spawn of the binary under test goes through the door that fixtures
//! its HOME (issue 0493).**
//!
//! A test binary inherits the operator's real `HOME`, and a verb's reach can
//! grow long after the test that drives it was written. `bootstrap` answered
//! *not implemented* until the day it published the machine's install pointer
//! to a scratch worktree. So a test cannot know whether it touches per-user
//! state, and the only safe default is that none of them reach the operator's.
//! The day it was counted, 47 of this crate's test files spawned the binary with
//! no HOME of their own, and none of them was yet a live case.
//!
//! **THE DOOR IS `crate::common::intent()`, AND THIS CENSUS IS WHAT MAKES IT THE
//! ONLY ONE.** The binary's path is `env!("CARGO_BIN_EXE_intent")`, a
//! compile-time constant with no other spelling, so a spawn that goes round the
//! door has to write that literal. The census greps the TOKEN in every file
//! under `tests/`. It never infers a builder's shape, which is the file-level
//! rule ruled out on 0493 because it fails silently in both directions: it
//! passes a file that fixtures its first spawn and not its second, and it fails
//! a file that fixtures correctly through a helper.
//!
//! **IT REPLACED `table_driven_tests_fixture_their_home.rs`** (vc, 2026-09-21,
//! named to hv before it landed). That guard bound only files choosing verbs
//! from the dispatch table and proved the fixture was SPELLED somewhere in the
//! file. Its whole population lies under this census, which binds every spawn,
//! so two guards over one population was the Highlander case.
//!
//! **WHAT IT CANNOT SEE, stated rather than implied.** A spawn built from a path
//! that is not the literal, eg a copy of the binary, `common::intent_path()`
//! handed to `Command::new`, or a shell running `intent` from `PATH`, is outside
//! a token census. The sites that existed when this was written were swept by
//! hand to `testkit::fixtured_command`. What keeps the next one honest is that
//! the door is the easiest thing to reach for, not that this file can see it.

use std::path::{Path, PathBuf};

/// The token a spawn round the door must write. Built from two halves so this
/// file does not contain it and never has to exclude itself.
const LITERAL: &str = concat!("env!(\"CARGO_BIN", "_EXE_intent\")");

/// The one file that names the literal, because it is the door.
const DOOR: &str = "common/mod.rs";

/// Files that name the literal for a reason that is not a spawn, each with that
/// reason. Expected empty or near it. **A test that only passes under the real
/// HOME is a finding, not an entry here.**
const ALLOWED: &[(&str, &str)] = &[(
  "the_binary_under_test_is_the_one_cargo_built.rs",
  "a raw-string fixture: the build-directory guard's own control asserts that \
   the literal idiom is not an offence, and it spawns nothing",
)];

/// Every `.rs` under this crate's `tests/`, relative to it, discovered by
/// walking so a new file is covered the day it is written.
fn test_sources() -> Vec<(String, PathBuf)> {
  fn walk(root: &Path, dir: &Path, out: &mut Vec<(String, PathBuf)>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
      return;
    };
    for entry in entries.flatten() {
      let path = entry.path();
      if path.is_dir() {
        walk(root, &path, out);
      } else if path.extension().is_some_and(|e| e == "rs") {
        let rel = path
          .strip_prefix(root)
          .expect("under tests/")
          .to_string_lossy()
          .replace('\\', "/");
        out.push((rel, path));
      }
    }
  }
  let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
  let mut out = Vec::new();
  walk(&root, &root, &mut out);
  out.sort();
  out
}

/// Whether a source's CODE names the literal. Comment lines are skipped, so
/// prose explaining the rule, this file's included, is never an offence.
fn names_the_literal(src: &str) -> bool {
  src
    .lines()
    .filter(|l| !l.trim_start().starts_with("//"))
    .any(|l| l.contains(LITERAL))
}

#[test]
fn every_spawn_of_the_binary_goes_through_the_fixtured_door() {
  let files = test_sources();
  assert!(
    files.len() > 100,
    "precondition: the walk found this crate's tests ({} files)",
    files.len()
  );

  // THE POSITIVE CONTROL. The door itself names the literal, so a scanner that
  // cannot see it there is reading nothing and would pass any estate.
  let door = files
    .iter()
    .find(|(rel, _)| rel == DOOR)
    .expect("the door file is in the walk");
  assert!(
    names_the_literal(&std::fs::read_to_string(&door.1).expect("read the door")),
    "`{DOOR}` no longer names {LITERAL}, so either the door moved or this census \
     cannot see the token it exists to find"
  );

  let offenders: Vec<&str> = files
    .iter()
    .filter(|(rel, _)| rel != DOOR && !ALLOWED.iter().any(|(name, _)| name == rel))
    .filter(|(_, path)| names_the_literal(&std::fs::read_to_string(path).unwrap_or_default()))
    .map(|(rel, _)| rel.as_str())
    .collect();

  assert!(
    offenders.is_empty(),
    "these files name {LITERAL} outside `{DOOR}`, so they reach the binary round the \
     door and a spawn there inherits the operator's real HOME (issue 0493): {offenders:?}\n\
     \n\
     Remedy: spawn with `crate::common::intent()`, or take the path from \
     `crate::common::intent_path()` and spawn a copy with `testkit::fixtured_command`."
  );
}

/// **THE MATCHER, DRIVEN ON BOTH SIDES OF ITS LINE.** A census whose matcher
/// can match nothing reports a clean estate for free, and one that matches
/// prose reds on its own explanation.
#[test]
fn the_census_matcher_flags_a_spawn_and_passes_prose() {
  let spawn = format!("  let out = std::process::Command::new({LITERAL})");
  assert!(
    names_the_literal(&spawn),
    "a spawn written round the door must be flagged"
  );
  let prose = format!("  /// used to spawn `Command::new({LITERAL})` directly");
  assert!(
    !names_the_literal(&prose),
    "a doc comment naming the literal is not a spawn"
  );
  assert!(
    !names_the_literal("  let out = crate::common::intent()"),
    "the door itself must not be an offence"
  );
}

/// **EVERY ALLOWED ENTRY STILL NEEDS TO BE THERE.** An exemption that stops
/// describing reality passes forever, so each entry must still exist and still
/// name the literal. It goes on the day its reason does.
#[test]
fn every_allowed_entry_still_names_the_literal() {
  let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
  for (name, reason) in ALLOWED {
    assert!(
      !reason.is_empty(),
      "`{name}` is allowed with no reason beside it"
    );
    let src = std::fs::read_to_string(root.join(name))
      .unwrap_or_else(|e| panic!("`{name}` is allowed and cannot be read ({e}): remove the entry"));
    assert!(
      names_the_literal(&src),
      "`{name}` is allowed and no longer names {LITERAL}, so the exemption has \
       outlived its reason: remove the entry"
    );
  }
}
