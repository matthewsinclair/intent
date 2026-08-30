//! AT-00.5 / AT-02.5 / AT-17.3 / AC-02.5: the rusqlite Highlander (design.md D06).
//!
//! `rusqlite` is consumed by intentsvcs and NOWHERE else -- the dependency
//! graph is the enforcement of "nothing touches the DB except intentsvcs".
//! Comment lines are stripped before matching so a manifest may NAME the rule
//! without violating it.

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

#[test]
fn rusqlite_appears_in_exactly_one_crate_manifest() {
  let root = workspace_root();
  let crates_dir = root.join("crates");
  let mut consumers = Vec::new();
  for entry in fs::read_dir(&crates_dir).expect("crates/ exists") {
    let manifest = entry.expect("dir entry").path().join("Cargo.toml");
    if !manifest.is_file() {
      continue;
    }
    if without_comments(&manifest).contains("rusqlite") {
      consumers.push(manifest.display().to_string());
    }
  }
  assert_eq!(
    consumers,
    vec![
      crates_dir
        .join("intentsvcs")
        .join("Cargo.toml")
        .display()
        .to_string()
    ],
    "rusqlite may be consumed by intentsvcs and nowhere else (D06)"
  );
}

#[test]
fn workspace_declaration_is_not_a_consumer() {
  // The root manifest DECLARES the pinned version under
  // [workspace.dependencies]; declaring is not consuming. This test documents
  // that reading, so nobody "fixes" the guard by deleting the declaration.
  let root_manifest = without_comments(&workspace_root().join("Cargo.toml"));
  assert!(
    root_manifest.contains("[workspace.dependencies]"),
    "the root manifest pins workspace dependency versions"
  );
}
