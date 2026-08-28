//! testkit's own arms, moved out of `src/` so that where they live matches the
//! rule that governs them.
//!
//! **`no_intent_home.rs` scans `src/` and only `src/`, deliberately: "tests do
//! not ship", and scanning `tests/` would forbid the technique that proves the
//! property.** That rationale is about whether code SHIPS; the mechanism asks
//! where the file SITS. For a crate whose tests live in `tests/` the two agree.
//! For an inline `#[cfg(test)] mod tests` they do not -- and Rust unit tests
//! have nowhere else to be, so the divergence is guaranteed rather than
//! unlucky.
//!
//! `the_pointer_a_verb_would_write_is_not_the_operators_pointer` reads `$HOME`,
//! which the guard confines to `userstate.rs`. **Routing it through that module
//! would destroy the arm**: it exists to prove the fixture pointer differs from
//! the OPERATOR'S real one, so it needs the raw value. Reading it through the
//! thing under isolation would compare an answer to itself -- the same defect
//! as a parser that agrees with its own output.
//!
//! So neither of the guard's two escape hatches fitted, and a third was not
//! needed: **these arms only ever call `testkit`'s public API**, so moving the
//! file is the whole fix. No exception row, no ruling, no `#[cfg(test)]`-aware
//! parser in a guard -- and a brace counter over source containing string
//! literals could stop scanning silently, which is worse than the false
//! positive it would remove.

use std::path::PathBuf;
use testkit::{fixture_home, repo_root, workspace_root};

#[test]
fn the_two_roots_are_not_the_same_directory() {
  // The hazard this crate was written about. If a later simplification makes
  // these agree, every caller of one of them is reading the wrong tree.
  assert_ne!(repo_root(), workspace_root());
}

/// **THE PROPERTY IS THE DERIVED PATH, NOT THE PREFIX.**
///
/// The first cut of this arm asserted the fixture does not sit UNDER the real
/// `HOME`, and it failed -- correctly. This repo lives at `~/Devel/prj/Intent`,
/// so anything under `target/` is inside the operator's home tree while being
/// a perfectly good fixture. What has to differ is the thing a verb actually
/// writes: `$HOME/.intent/home`.
#[test]
fn the_pointer_a_verb_would_write_is_not_the_operators_pointer() {
  let real = PathBuf::from(std::env::var("HOME").expect("a HOME to be isolated from"));
  assert_ne!(
    fixture_home().join(".intent/home"),
    real.join(".intent/home"),
    "a fixture HOME that resolves to the operator's own pointer isolates nothing"
  );
  assert!(
    fixture_home().is_dir(),
    "it must exist to be usable as a HOME"
  );
}

#[test]
fn the_workspace_root_sits_under_the_repo_root() {
  assert!(
    workspace_root().starts_with(repo_root()),
    "workspace {} should sit under repo {}",
    workspace_root().display(),
    repo_root().display()
  );
}

#[test]
fn each_root_carries_what_its_callers_reach_for() {
  // Named separately from the locating predicates: a caller of repo_root()
  // wants schema/, and a caller of workspace_root() wants crates/. Asserting
  // the predicate back at itself would be the test restating the function.
  assert!(repo_root().join("intent").is_dir());
  assert!(workspace_root().join("crates").is_dir());
}
