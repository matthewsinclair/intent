//! AT-02.2 / AC-02.2: the committed schema faces under `schema/` match what
//! the types generate -- CI fails on drift (design.md, "one master, three
//! faces").
//!
//! Regenerate deliberately with:
//!   INTENT_BLESS=1 cargo test -p intentsvcs --test schema_faces_drift
//! and commit the face change WITH the type change that caused it.

use std::fs;
use std::path::{Path, PathBuf};

/// The REPOSITORY root -- where `schema/`, `surface/` and `bin/` live.
///
/// Searched, never counted. This was `ancestors().nth(2)`, which was correct
/// only while the workspace root and the repository root were the same
/// directory; `native/rust/` made them different and every counting caller
/// broke at once. A depth is a claim about a layout, and it goes stale
/// silently -- a search for the thing actually wanted cannot.
///
/// The marker is `schema/` AND `surface/` together, because either alone
/// could match some unrelated ancestor.
fn repo_root() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR"))
    .ancestors()
    .find(|d| d.join("schema").is_dir() && d.join("surface").is_dir())
    .expect("a repository root carrying schema/ and surface/ above this crate")
    .to_path_buf()
}

fn schema_dir() -> PathBuf {
  repo_root().join("schema")
}

#[test]
fn committed_faces_match_the_types() {
  let dir = schema_dir();
  let bless = std::env::var_os("INTENT_BLESS").is_some();
  if bless {
    fs::create_dir_all(&dir).expect("create schema/");
  }
  let mut drifted = Vec::new();
  for (name, generated) in intentsvcs::faces::faces() {
    let path = dir.join(name);
    if bless {
      fs::write(&path, &generated).unwrap_or_else(|e| panic!("bless {}: {e}", path.display()));
      continue;
    }
    match fs::read_to_string(&path) {
      Ok(committed) if committed == generated => {}
      Ok(_) => drifted.push(format!("{name}: committed face differs from the types")),
      Err(_) => drifted.push(format!("{name}: committed face missing")),
    }
  }
  assert!(
    drifted.is_empty(),
    "schema faces drifted from the model types:\n  {}\nregenerate deliberately: INTENT_BLESS=1 cargo test -p intentsvcs --test schema_faces_drift",
    drifted.join("\n  ")
  );
}
