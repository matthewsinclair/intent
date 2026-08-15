//! Test-only helpers shared across the workspace's integration tests.
//!
//! # Why this crate exists
//!
//! Rust gives integration tests no way to share a helper across crates: each
//! file under `tests/` is its own binary, and `mod common;` cannot cross a crate
//! boundary. So the only options are a dev-dependency crate or copy-paste, and
//! the estate had chosen copy-paste **nine times**.
//!
//! # What the nine copies actually were
//!
//! Not one helper duplicated nine times. **Two different functions returning two
//! different directories, wearing names similar enough that nobody noticed**:
//!
//! | name               | copies | returns                                    |
//! | ------------------ | ------ | ------------------------------------------ |
//! | `repo_root()`      | 5      | the repository root, carrying `schema/`    |
//! | `workspace_root()` | 4      | `native/rust`, the cargo workspace root    |
//!
//! They differ by two directory levels. The duplication was found by grepping
//! `repo_root`, which is why it was reported as four copies -- **the other five
//! were invisible to the search that found the first four.** Two names for
//! adjacent concepts is worse than nine copies of one name, because it defeats
//! the only tool anyone was going to use to look.
//!
//! Naming them apart is therefore load-bearing rather than cosmetic. Reaching
//! for "the root" and getting the wrong one of these fails as a missing file two
//! levels away from where you are looking.
//!
//! # Searched, never counted
//!
//! Both functions SEARCH for a structural marker; neither counts levels.
//!
//! `ancestors().nth(2)` and `.parent().parent()` were correct until `a1a949c`
//! moved every native source to `native/rust/`, at which point everything that
//! counted its way to the repository root broke at once -- and **a counted path
//! that is wrong does not fail where it was written; it fails as a file-not-found
//! somewhere else entirely.** One copy still counted when this crate replaced
//! them (`dep_graph_guard.rs`), which is the whole argument for having one home:
//! the fix was applied to the copies someone remembered.
//!
//! Counting is not *always* wrong -- `crates/<name>` sitting two levels under a
//! workspace root is a cargo convention, not a project choice. It is refused here
//! anyway, because the reason to prefer searching is that **the failure mode of a
//! search is a loud panic at the point of use, and the failure mode of a count is
//! a plausible wrong directory.**

use std::path::{Path, PathBuf};

/// The REPOSITORY root: the directory carrying `schema/` and `surface/`.
///
/// Two levels above [`workspace_root`]. Use this to reach committed canon --
/// the schema faces, the surface dispatch table, `intent/`.
///
/// Deliberately not located by `.git`: a git worktree has a `.git` FILE rather
/// than a directory, and this estate runs sacrificial worktrees routinely, so a
/// `.git`-is-a-dir test would fail in exactly the environment used to verify
/// destructive changes.
pub fn repo_root() -> PathBuf {
  ancestor_where(|d| d.join("schema").is_dir() && d.join("surface").is_dir())
    .expect("a repository root carrying schema/ and surface/ above this crate")
}

/// The CARGO WORKSPACE root: `native/rust`, the directory holding `crates/`.
///
/// Two levels below [`repo_root`]. Use this to walk sources -- `crates/*/src`,
/// `crates/*/tests` -- and to read the workspace manifest.
///
/// Located by the `[workspace]` table, which is the definition of a workspace
/// root rather than a proxy for it. A `crates/`-is-a-dir test would also work
/// today and would silently pick the wrong directory the moment any crate grew
/// a `crates/` subdirectory of its own.
pub fn workspace_root() -> PathBuf {
  ancestor_where(|d| {
    let manifest = d.join("Cargo.toml");
    manifest.is_file()
      && std::fs::read_to_string(&manifest)
        .map(|s| s.lines().any(|l| l.trim_start().starts_with("[workspace]")))
        .unwrap_or(false)
  })
  .expect("a Cargo.toml declaring [workspace] above this crate")
}

/// The nearest ancestor of this crate's manifest directory satisfying `pred`.
///
/// `CARGO_MANIFEST_DIR` is the compiling crate's directory, which cargo sets for
/// every test binary, so this resolves relative to the caller rather than to the
/// process's working directory -- tests that `cd` into a tempdir still find the
/// tree they were built from.
fn ancestor_where(pred: impl Fn(&Path) -> bool) -> Option<PathBuf> {
  Path::new(env!("CARGO_MANIFEST_DIR"))
    .ancestors()
    .find(|d| pred(d))
    .map(Path::to_path_buf)
}

/// Mutation-proven. Replacing `workspace_root`'s body with `repo_root()` --
/// collapsing the distinction this crate exists to keep -- fails
/// `the_two_roots_are_not_the_same_directory` and
/// `each_root_carries_what_its_callers_reach_for`.
///
/// **`the_workspace_root_sits_under_the_repo_root` PASSES under that mutation**,
/// and is kept knowing so: it catches the different failure of a root escaping
/// the repository entirely (a search that ran off the top and returned `/`),
/// which neither of the others would notice. Recorded rather than quietly
/// retained -- a test that survives the obvious mutation needs its own reason.
///
/// The first attempt at this mutation did not apply: it computed the wrong root
/// as a discarded statement and returned the right one, so all three tests
/// passed. **An unapplied mutation reports "nothing failed", which is
/// indistinguishable from a test that does not check.** The map above was taken
/// only after printing the mutated function body and seeing it change.
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn the_two_roots_are_not_the_same_directory() {
    // The hazard this crate was written about. If a later simplification makes
    // these agree, every caller of one of them is reading the wrong tree.
    assert_ne!(repo_root(), workspace_root());
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
}
