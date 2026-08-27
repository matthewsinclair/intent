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
use std::process::{Command, Output};

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

/// A `HOME` for any test that spawns the `intent` binary.
///
/// **A TEST BINARY INHERITS THE OPERATOR'S REAL `HOME`, AND SOME VERBS WRITE
/// THERE.** `intent bootstrap` publishes `~/.intent/home`, the machine-global
/// install pointer the pre-commit shim resolves on every commit. On 2026-08-27
/// two arms of `dispatch_ssot` published it to a scratch worktree that was later
/// deleted, and the estate spent an evening with a pointer naming a directory
/// that did not exist.
///
/// **NEITHER ARM WAS WRONG WHEN IT WAS WRITTEN.** Both drive every shipped
/// family bare, looking for the ones that answer *is a known command that is not
/// implemented yet*, and `bootstrap` answered exactly that until it was
/// implemented. **The subject changed underneath the test** -- so this is not a
/// helper for tests that touch per-user state, it is a helper for tests that
/// cannot know whether they do.
///
/// **UNDER `target/` RATHER THAN THE SYSTEM TEMP DIRECTORY**, on purpose: it is
/// already build output, `cargo clean` removes it, and it never accumulates in
/// `/tmp` where nothing prunes it. Per-process, so parallel test binaries do not
/// share one.
///
/// **std ONLY.** This crate declares no dependencies and `dep_graph_guard.rs`
/// enforces that, so no `tempfile` here.
pub fn fixture_home() -> &'static Path {
  static DIR: std::sync::OnceLock<PathBuf> = std::sync::OnceLock::new();
  DIR
    .get_or_init(|| {
      let dir = workspace_root()
        .join("target/test-home")
        .join(std::process::id().to_string());
      std::fs::create_dir_all(&dir).expect("create the fixture HOME");
      dir
    })
    .as_path()
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
/// Run a JUST-COPIED binary, retrying while Linux reports it busy.
///
/// **ETXTBSY IS A PROPERTY OF THIS HARNESS, NOT OF THE THING UNDER TEST.**
/// `fs::copy` closes its own destination handle, but a test binary is
/// multi-threaded and several of its tests fork (`Command::output`). A child
/// forked between another thread's open and close inherits that write fd, and
/// between `fork` and `execve` it still holds it -- Linux refuses to `execve`
/// a file any process has open for writing. macOS does not enforce that, which
/// is exactly why the macOS leg stays green while ubuntu reddens on
/// `Os { code: 26, kind: ExecutableFileBusy }`.
///
/// Bounded retry rather than a mutex: the window is microseconds, it belongs to
/// the harness, and serialising these tests would slow them to buy determinism
/// these assertions do not need. A retry that never succeeds still fails, and
/// says why.
///
/// Matched on `raw_os_error() == 26` rather than `ErrorKind::ExecutableFileBusy`
/// so this does not depend on that variant's stabilisation.
///
/// # Why it lives HERE rather than beside one of its callers
///
/// **It was written correctly, reasoned correctly, and applied to a population
/// of one when the population was three.** `eb4fe67c` fixed the site that had
/// just reddened CI, and its commit message says "the ONE
/// exec-of-a-just-copied-binary". Three integration tests copy
/// `CARGO_BIN_EXE_intent` out and exec the copy -- `embedded_init`,
/// `info_exit_code` and `migrated_guards_still_refuse` -- and over the twelve
/// `rust` runs to 2026-08-24 **four failed, all on this error, across two of
/// those three files**. The third has the identical exposure and had simply not
/// lost the race yet.
///
/// **The set came from what was in hand rather than from what the property
/// reaches**, and one grep for the copy call settles it. That is the same class
/// this crate's own header describes: a duplication found by grepping one name,
/// which could not see the copies wearing the other.
pub fn output_retrying_busy(mut build: impl FnMut() -> Command, what: &str) -> Output {
  const ETXTBSY: i32 = 26;
  let mut last = String::new();
  for _ in 0..100 {
    match build().output() {
      Ok(out) => return out,
      Err(e) if e.raw_os_error() == Some(ETXTBSY) => {
        last = e.to_string();
        std::thread::sleep(std::time::Duration::from_millis(20));
      }
      Err(e) => panic!("{what}: {e}"),
    }
  }
  panic!(
    "{what}: still busy after 100 attempts over ~2s ({last}) -- that is no longer a fork race"
  );
}

#[cfg(test)]
mod tests {
  use super::*;

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
}
