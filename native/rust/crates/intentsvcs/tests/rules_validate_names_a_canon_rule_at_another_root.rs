//! Issue 0640: a canon rule named by its path in another tree -- a linked
//! worktree beside the install root -- is that rule, not a second declaration
//! of its id. A rule's identity is its path under `intent/plugins/claude/rules/`.
//!
//! **TWO SCRATCH ROOTS, NOT THE REAL TREE**, so each arm controls both copies:
//! the library is one, the named file sits in the other.

use intentsvcs::rules::Library;
use std::path::{Path, PathBuf};

const RULES: &str = "intent/plugins/claude/rules";

const RULE: &str = "---\nid: IN-AG-TWIN-001\ntitle: A rule\nlanguage: agnostic\ncategory: fixt\nseverity: recommendation\nsummary: Fine.\nprinciples: []\napplies_when: []\n---\n\n# A rule\n";

/// An install root whose rules directory carries the real schema, so the
/// validator reads the vocabulary it ships with.
fn install(root: &Path) {
  let schema = Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("../../../..")
    .join(RULES)
    .join("_schema/rule-schema.md");
  let dir = root.join(RULES).join("_schema");
  std::fs::create_dir_all(&dir).expect("schema dir");
  std::fs::copy(&schema, dir.join("rule-schema.md")).expect("copy the schema");
}

/// One rule at `<root>/intent/plugins/claude/rules/agnostic/fixt/<slug>/RULE.md`.
fn plant(root: &Path, slug: &str) -> PathBuf {
  let dir = root.join(RULES).join("agnostic/fixt").join(slug);
  std::fs::create_dir_all(&dir).expect("rule dir");
  let path = dir.join("RULE.md");
  std::fs::write(&path, RULE).expect("write the rule");
  path
}

#[test]
fn the_same_canon_rule_at_another_root_validates_as_one_rule() {
  let library = tempfile::tempdir().expect("library root");
  let worktree = tempfile::tempdir().expect("second root");
  install(library.path());
  plant(library.path(), "twin");
  let named = plant(worktree.path(), "twin");

  let lib = Library::new(library.path(), None);
  let (findings, selected) = lib
    .validate(Some(named.to_str().expect("utf-8 path")))
    .expect("validate");

  assert_eq!(selected, 1, "the named rule is the one selected");
  assert!(
    findings.is_empty(),
    "the library's copy was counted as a second declaration: {findings:?}"
  );
}

#[test]
fn one_id_at_two_canon_relative_paths_is_still_refused() {
  let library = tempfile::tempdir().expect("library root");
  let worktree = tempfile::tempdir().expect("second root");
  install(library.path());
  plant(library.path(), "twin");
  let named = plant(worktree.path(), "other");

  let lib = Library::new(library.path(), None);
  let (findings, _) = lib
    .validate(Some(named.to_str().expect("utf-8 path")))
    .expect("validate");

  assert!(
    findings
      .iter()
      .any(|f| f.path == named && f.message.contains("is declared by 1 other rule file(s)")),
    "two rule files declaring one id were not refused: {findings:?}"
  );
}

#[test]
fn bare_validate_is_unchanged() {
  let library = tempfile::tempdir().expect("library root");
  install(library.path());
  plant(library.path(), "twin");

  let (findings, count) = Library::new(library.path(), None)
    .validate(None)
    .expect("validate");

  assert_eq!(count, 1, "the library holds one rule");
  assert!(
    findings.is_empty(),
    "a clean library reported: {findings:?}"
  );
}
