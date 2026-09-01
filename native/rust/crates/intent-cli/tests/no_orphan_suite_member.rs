//! **A TEST FILE NOBODY DECLARES IS NEVER COMPILED AND SAYS NOTHING.**
//!
//! `autotests = false` bought one linked binary instead of one per file, and it
//! inverted the failure rather than removing it. Before: a stray `tests/quick.rs`
//! silently became another target. After: a stray `tests/quick.rs` silently becomes
//! NOTHING -- not compiled, not run, not reported. **Both are silent; only the second
//! can lose coverage that someone believed they had**, which is why the consolidation
//! does not ship without this file.
//!
//! Adopted from Laksa, which hit the same inversion taking hv's 2026-08-27 ruling and
//! built the guard the same morning (`01c00c91f`). Carried here rather than reinvented.
//!
//! **KEYED ON `#[path]`, NEVER ON THE MOD NAME.** Only the path decides what gets
//! compiled: `#[path = "a.rs"] mod b;` compiles `a.rs`, and a guard reading `mod b`
//! would report `b.rs` -- a file that need not exist -- as covered while `a.rs` went
//! unchecked. The two agree by convention everywhere in this tree today, which is
//! exactly the condition under which reading the wrong one passes.

use std::collections::BTreeSet;
use std::path::Path;

/// Files that legitimately are not suite members: the suite itself, plus anything
/// carrying its OWN `[[test]]` target in the manifest -- read from the manifest rather
/// than listed here, so adding an isolated target cannot desynchronise from this guard.
fn independently_declared(manifest: &str) -> BTreeSet<String> {
  let mut out = BTreeSet::new();
  for line in manifest.lines() {
    let line = line.trim();
    if let Some(rest) = line.strip_prefix("path = \"tests/") {
      if let Some(name) = rest.strip_suffix("\"") {
        out.insert(name.to_string());
      }
    }
  }
  out
}

fn declared_paths(suite: &str) -> BTreeSet<String> {
  let mut out = BTreeSet::new();
  for line in suite.lines() {
    let line = line.trim();
    if let Some(rest) = line.strip_prefix("#[path = \"") {
      if let Some(p) = rest.strip_suffix("\"]") {
        out.insert(p.to_string());
      }
    }
  }
  out
}

#[test]
fn every_test_file_is_a_declared_suite_member() {
  let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
  let suite = std::fs::read_to_string(dir.join("suite.rs")).expect("the suite must exist");
  let manifest = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
    .expect("the manifest must exist");

  let declared = declared_paths(&suite);
  let independent = independently_declared(&manifest);

  let mut orphans: Vec<String> = Vec::new();
  for entry in std::fs::read_dir(&dir).expect("tests/ must be readable") {
    let entry = entry.expect("a readable dir entry");
    if !entry.file_type().expect("a file type").is_file() {
      continue;
    }
    let name = entry.file_name().to_string_lossy().to_string();
    if !name.ends_with(".rs") || name == "suite.rs" {
      continue;
    }
    if declared.contains(&name) || independent.contains(&name) {
      continue;
    }
    orphans.push(name);
  }
  orphans.sort();

  assert!(
    orphans.is_empty(),
    "these files under tests/ are compiled by NOTHING and run NOWHERE -- declare each in \
     tests/suite.rs as `#[path = \"<name>\"] mod <stem>;`, or give it its own [[test]] in \
     Cargo.toml if it must stay a separate process: {orphans:?}"
  );

  // The guard's own positive control. A guard over a set it cannot read is a guard that
  // passes over an empty directory, and an empty `declared` would make every branch
  // above vacuous while reporting exactly the same green.
  assert!(
    !declared.is_empty(),
    "no `#[path]` declarations parsed out of suite.rs -- this guard was reading nothing \
     and would have passed over any orphan at all"
  );
}
