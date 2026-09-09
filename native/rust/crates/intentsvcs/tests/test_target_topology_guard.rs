//! **THE TRANSITION NOTHING ELSE WATCHES: a crate that GROWS a `tests/`
//! directory.** TN001's four parts govern crates that already have integration
//! tests, and its orphan guard lives inside `tests/suite.rs` -- so **the
//! enforcement mechanism cannot exist before the thing it enforces.** A crate
//! with no `tests/` has nothing holding the property, and the first file added
//! there decides silently which régime the crate is in.
//!
//! `testkit` is the worked case: it carried no `autotests` key at all, its one
//! test file was auto-discovered into its own target, and a second file would
//! have become a second target with nothing saying a word. TN001 names testkit
//! only to show that `grep -c '[[test]]'` is a bad instrument -- **it looked
//! straight at the crate and did not notice it was also an instance.**
//!
//! **THIS IS DELIBERATELY WEAKER THAN THE ORPHAN GUARD, AND THE WEAKNESS IS THE
//! POINT.** It reads a directory listing and a manifest; it never reads
//! `suite.rs`. That is what lets it cover the window before a suite exists,
//! where the orphan guard structurally cannot reach.
//!
//! **COMMENTS ARE STRIPPED BEFORE MATCHING**, so a manifest may NAME `autotests`
//! in prose without satisfying -- or violating -- the rule. `dep_graph_guard.rs`
//! established that here, and this file's own subject manifest carries exactly
//! such a comment.

use std::fs;
use std::path::Path;
use testkit::workspace_root;

/// Manifest content with `#` comment lines removed.
fn without_comments(path: &Path) -> String {
  fs::read_to_string(path)
    .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
    .lines()
    .filter(|line| !line.trim_start().starts_with('#'))
    .collect::<Vec<_>>()
    .join("\n")
}

/// Does this crate hold at least one `.rs` directly under `tests/`?
fn has_integration_tests(crate_dir: &Path) -> bool {
  let dir = crate_dir.join("tests");
  let Ok(entries) = fs::read_dir(&dir) else {
    return false;
  };
  entries.filter_map(Result::ok).any(|e| {
    e.file_type().map(|t| t.is_file()).unwrap_or(false)
      && e.file_name().to_string_lossy().ends_with(".rs")
  })
}

#[test]
fn a_crate_with_integration_tests_declares_exactly_how_they_are_built() {
  let crates_dir = workspace_root().join("crates");
  let mut examined = 0usize;
  let mut with_tests = 0usize;
  let mut breaches: Vec<String> = Vec::new();

  for entry in fs::read_dir(&crates_dir).expect("crates/ exists") {
    let dir = entry.expect("dir entry").path();
    let manifest_path = dir.join("Cargo.toml");
    if !manifest_path.is_file() {
      continue;
    }
    examined += 1;
    let manifest = without_comments(&manifest_path);
    let name = dir
      .file_name()
      .unwrap_or_default()
      .to_string_lossy()
      .to_string();

    let declares_off = manifest.contains("autotests = false");
    let declares_target = manifest.contains("[[test]]");

    if has_integration_tests(&dir) {
      with_tests += 1;
      if !declares_off {
        breaches.push(format!(
          "{name}: has tests/*.rs but no `autotests = false` -- every file there is its \
           own target, and a new one joins them silently"
        ));
      }
      if !declares_target {
        breaches.push(format!(
          "{name}: has tests/*.rs but declares no [[test]] target"
        ));
      }
    }

    // The inverse hazard, which is worse and is why this arm exists: turning
    // discovery off while declaring nothing means a file under `tests/` is
    // compiled by NOTHING. Passing today, armed for the first file anyone adds.
    if declares_off && !declares_target {
      breaches.push(format!(
        "{name}: `autotests = false` with no [[test]] target -- any file under tests/ \
         would be compiled by nothing and run nowhere"
      ));
    }
  }

  // **THE POPULATION IS ASSERTED BEFORE THE PROPERTY.** A walker aimed at the
  // wrong directory returns no crates, therefore no breaches, and reports a
  // clean estate -- the failure this estate has met from four directions.
  assert!(
    examined >= 2,
    "walked {} crate manifest(s) under {} -- this guard was reading nothing and \
     would have passed over any breach at all",
    examined,
    crates_dir.display()
  );
  assert!(
    with_tests >= 1,
    "no crate with integration tests found -- the property below is vacuous, so a \
     green here says nothing"
  );

  assert!(
    breaches.is_empty(),
    "TN001: one declared test target per crate. {breaches:#?}"
  );
}
