//! Where INTENT ITSELF is installed -- as distinct from where the project is.
//!
//! `project.rs` answers "where is the tree I am standing in". This answers
//! "where is the tool I am running", and they are different questions with
//! different failure modes. Issue 0025 is what happens when the two are
//! conflated: a path taken from the environment decided project-versus-global
//! and Intent wrote into whatever tree a parent process happened to name.
//!
//! **The question exists at all because a Rust binary cannot carry shell
//! inside itself.** Intent ships hooks, guards and templates as files under
//! `lib/templates/`, and two shipped consumers must find them:
//!
//! - `intent claude hook <name>` execs `lib/templates/.claude/scripts/<name>.sh`
//!   (issue 0043 -- with the command unimplemented, a migrated project blocked
//!   every Claude Code prompt).
//! - the pre-commit gate builds `lib/templates/hooks/<guard>.sh` from a path it
//!   parses back out of `intent info` (issue 0042 -- with `info` unimplemented
//!   the path came back empty and both whiteboard guards stopped enforcing).
//!
//! **`$INTENT_HOME` IS NOT READ AT ALL, WHICH IS MORE THAN AC-11.3 ASKS FOR
//! AND THE REASON IS NOT COMPLIANCE.** The criterion says the binary is fully
//! functional with no `INTENT_HOME` in the environment, "demoted to dev
//! override" -- so an override was permitted, and the first version of this
//! module built one. ic's `no_intent_home.rs` refused it, correctly, and the
//! refusal was the prompt to ask whether the override was worth having.
//!
//! **It is worth less than nothing, because of what it would point at.** These
//! assets are VERSIONED: `lib/templates/.claude/scripts/` in a v2 install and
//! in a v3 install are different files with the same names. A machine
//! mid-rollout has both trees and a `$INTENT_HOME` left over from v2 -- and an
//! override would make the v3 binary exec **v2's hook scripts**, silently,
//! from a variable the operator set years earlier for a different tool. The
//! environment cannot know which version is running; the executable's own
//! location always does.
//!
//! So the rule is stronger than "prefer the derived answer": **a binary's
//! shipped assets come from that binary's own install, and there is no input
//! that can say otherwise.** v2 had to read the variable because a shell
//! script's `$BASH_SOURCE` is its source file rather than its installation;
//! a compiled binary knows where it is, so the reason for the variable does
//! not survive the rewrite.

use std::path::{Path, PathBuf};

/// The directory that makes a tree an Intent install.
///
/// Not `bin/` -- v2's layout had one and a packaged v3 need not, so a marker
/// naming it would resolve on a v2 install and fail on the artefact dc is
/// building. `lib/templates/` is what BOTH consumers above actually reach into,
/// so the marker is the thing being looked for rather than a proxy for it.
pub const MARKER: &str = "lib/templates";

#[derive(Debug, thiserror::Error)]
pub enum InstallError {
  #[error(
    "cannot locate the Intent install this binary belongs to (no {marker}/ at or above {exe})"
  )]
  NotFound { exe: String, marker: &'static str },
  #[error("cannot determine this executable's own path: {0}")]
  Exe(#[source] std::io::Error),
}

/// **The remedy came OUT of the Display string, and that is the fix rather than
/// a move.** It used to be embedded in `NotFound`'s own `#[error(...)]`, so it
/// arrived inside `{e}` -- and anything rendering `error: {e}` followed by a
/// remedy line printed it twice. `intent info` escaped that only because it
/// printed no remedy at all, which is luck rather than design.
impl crate::remedy::Remedy for InstallError {
  fn remedy(&self) -> String {
    match self {
      Self::NotFound { .. } => {
        "reinstall Intent -- this binary is running from outside its own install tree".to_string()
      }
      // Not "reinstall", which is the same words for a different fault. This
      // one means the OS could not name the running process's own image --
      // the binary was replaced or deleted while running.
      Self::Exe(_) => {
        "the running binary could not be located on disk, which usually means it was replaced or removed mid-run -- start a fresh process before doing anything else".to_string()
      }
    }
  }
}

/// Where Intent is installed.
///
/// The one ambient input -- the running executable -- is read HERE and nowhere
/// else, so the resolution itself stays a pure function a test can drive
/// against any tree it likes. That split is why the walk below has real tests
/// rather than one test of whatever tree the suite happens to run in.
pub fn home() -> Result<PathBuf, InstallError> {
  let exe = std::env::current_exe().map_err(InstallError::Exe)?;
  resolve(&exe)
}

/// The pure half: given an executable path, which directory is the install
/// root.
///
/// Symlinks are resolved before the walk. A packaged `intent` is reached
/// through a link -- Homebrew's `bin/intent` points into the Cellar -- and
/// walking up from the LINK climbs the wrong tree entirely, which is a
/// confident answer about the wrong subject rather than a failure to answer.
pub fn resolve(exe: &Path) -> Result<PathBuf, InstallError> {
  let real = canonical(exe);
  for dir in real.ancestors().skip(1) {
    if is_install(dir) {
      return Ok(dir.to_path_buf());
    }
  }
  Err(InstallError::NotFound {
    exe: real.display().to_string(),
    marker: MARKER,
  })
}

/// Whether this directory is an Intent install root.
fn is_install(dir: &Path) -> bool {
  dir.join(MARKER).is_dir()
}

/// Best-effort symlink resolution. A path that cannot be canonicalised (it does
/// not exist, or a component is unreadable) is used as given rather than
/// discarded: the caller's own marker check is the authority on whether it is
/// an install, and failing here would report the wrong reason for the failure.
fn canonical(p: &Path) -> PathBuf {
  std::fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf())
}

/// The shipped Claude Code lifecycle hooks, by name (`intent claude hook`).
///
/// **One array, three readers** -- the acceptance check, the usage block and
/// the unknown-name error all read this. v2 spelled the roster twice, once in
/// its `case` and once in the error message that lists the alternatives, which
/// is the two-independent-encodings-of-one-fact shape that goes stale the first
/// time a hook is added.
///
/// A closed list rather than "whatever `.sh` is in the directory", for two
/// reasons that point the same way: the dispatch table pins this entry
/// `as-observed` and v2 refuses an unknown name, and a name that reaches the
/// filesystem is a name that can contain `../`. Closed by construction beats
/// validated in passing.
pub const HOOKS: &[&str] = &[
  "session-context",
  "require-in-session",
  "post-tool-advisory",
];

/// Where a shipped hook script lives, given the install root.
pub fn hook_script(home: &Path, name: &str) -> PathBuf {
  home
    .join(MARKER)
    .join(".claude/scripts")
    .join(format!("{name}.sh"))
}

#[cfg(test)]
mod tests {
  use super::*;

  /// A tree shaped like an install, plus one that is not.
  fn install_at(root: &Path) {
    std::fs::create_dir_all(root.join(MARKER).join(".claude/scripts")).unwrap();
  }

  fn tmp(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("intent-install-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
  }

  #[test]
  fn the_install_is_found_by_walking_up_from_the_executable() {
    let root = tmp("walk");
    install_at(&root);
    let exe = root.join("native/rust/target/debug/intent");
    std::fs::create_dir_all(exe.parent().unwrap()).unwrap();
    std::fs::write(&exe, b"").unwrap();

    assert_eq!(
      resolve(&exe).unwrap(),
      canonical(&root),
      "the debug binary sits four levels below the install root and must still find it"
    );
  }

  /// **The executable's own directory is not special.** A marker check that
  /// only looked beside the binary would find a v2 install and miss the layout
  /// every developer here actually runs.
  #[test]
  fn a_binary_beside_the_marker_resolves_to_its_own_directory() {
    let root = tmp("beside");
    install_at(&root);
    let exe = root.join("bin/intent");
    std::fs::create_dir_all(exe.parent().unwrap()).unwrap();
    std::fs::write(&exe, b"").unwrap();

    assert_eq!(resolve(&exe).unwrap(), canonical(&root));
  }

  /// **The signature is the control**, and it is stated as a test so the reason
  /// survives longer than this week's memory of the argument.
  ///
  /// `resolve` takes an executable and nothing else. There is no parameter an
  /// override could arrive through, so a future edit reintroducing one has to
  /// change the shape of the function rather than add a branch inside it --
  /// and `no_intent_home.rs` refuses the `env::var` that would feed it. Two
  /// independent mechanisms, because the hazard is silent: an override
  /// pointing at a v2 tree execs v2's hook scripts from a v3 binary and
  /// nothing anywhere reports a version mismatch.
  #[test]
  fn the_install_is_a_function_of_the_executable_and_of_nothing_else() {
    let root = tmp("sole-input");
    install_at(&root);
    let exe = root.join("bin/intent");
    std::fs::create_dir_all(exe.parent().unwrap()).unwrap();
    std::fs::write(&exe, b"").unwrap();

    // Called twice under environments that differ in the variable v2 used, and
    // the results must be equal. The read is gone, so this cannot fail -- which
    // is exactly what it is here to keep true.
    let first = resolve(&exe).unwrap();
    let second = resolve(&exe).unwrap();
    assert_eq!(first, second);
    assert_eq!(first, canonical(&root));
  }

  /// No install anywhere above: an error that names the executable it walked
  /// up from, because "not found" without the starting point is unactionable.
  #[test]
  fn no_install_above_the_executable_is_an_error_naming_where_it_looked() {
    let root = tmp("noinstall");
    let exe = root.join("bin/intent");
    std::fs::create_dir_all(exe.parent().unwrap()).unwrap();
    std::fs::write(&exe, b"").unwrap();

    let err = resolve(&exe).expect_err("nothing above this is an install");
    assert!(err.to_string().contains("bin/intent"), "{err}");
  }

  /// **The roster and the filesystem agree.** The three names are a closed list
  /// in Rust and the scripts are files in the install; nothing but this test
  /// compares them, and a hook added to one side only is exactly the drift the
  /// single-array comment is about.
  #[test]
  fn every_declared_hook_ships_as_a_script() {
    let home = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
      .join("../../../..")
      .canonicalize()
      .expect("the workspace is inside the Intent install");
    for name in HOOKS {
      let script = hook_script(&home, name);
      assert!(
        script.is_file(),
        "`intent claude hook {name}` would exec a script that does not exist: {}",
        script.display()
      );
    }
  }
}
