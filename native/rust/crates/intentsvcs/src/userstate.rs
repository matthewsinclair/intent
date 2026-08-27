//! Where the OPERATOR's own per-user state lives -- as distinct from where the
//! tool is installed, and from where the project is.
//!
//! Three different questions with three different failure modes, and this
//! estate has conflated two of them before (issue 0025). `project.rs` answers
//! *where is the tree I am standing in*. `install.rs` answers *where is the
//! tool I am running*. This answers *where is the state that belongs to the
//! person running it* -- skills, subagents, extensions, caches.
//!
//! **THIS IS THE ONE PLACE `$HOME` IS READ, AND THAT CONFINEMENT IS THE POINT
//! OF THE RULING RATHER THAN A TIDINESS PREFERENCE.** AC-11.3's invariant held
//! the shipped surface to exactly one environment variable, `COLUMNS`, and
//! `no_intent_home::the_shipped_surface_reads_exactly_one_environment_variable`
//! enforced it structurally over every `src/**/*.rs`. hv granted `$HOME` on
//! 2026-08-22, with a row in `ALLOWED` rather than a quiet addition, so the
//! commands that manage per-user state can exist at all.
//!
//! **A GRANT THAT LANDS EVERYWHERE IS A DIFFERENT GRANT FROM THE ONE THAT WAS
//! ASKED FOR.** The question put to hv was whether per-user state may be
//! reached; it was not whether any file may consult the environment. So the
//! read is confined to this module and the test pins it here by path -- a
//! second `$HOME` read anywhere else fails the same way an unapproved variable
//! does. That keeps the invariant meaning what it said, and it keeps the audit
//! surface one file wide.
//!
//! **AND IT IS WHAT MAKES vc's CLASS RULING ENFORCEABLE IN ONE EDIT.** hv
//! adopted, 2026-08-22: *every v3 per-user store gets its own path and never
//! reads or writes v2's* -- ruled as a property after `installed-agents.json`
//! turned up as the exact sibling of `installed-skills.json` an hour after the
//! instance was ruled. Seven such stores exist. With every path named here,
//! the rule is checkable by reading one file; spread across seven call sites
//! it would be re-litigated seven times and lost on the eighth.
//!
//! **A `version` FIELD DOES NOT DISCHARGE THAT RULE.** It is a SCHEMA version
//! -- it says what shape a file is, never who wrote it -- and a field only the
//! newer party reads is a courtesy rather than a discriminator (ic). v2 is
//! shipped and can never be taught the branch. Four of the seven stores are
//! content files carrying no field at all, so there is nothing to discriminate
//! with even in principle. Separate paths are the only mechanism that works.

use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserStateError {
  /// **NOT "set `$HOME`", BECAUSE THAT IS ADVICE FOR A DIFFERENT FAULT.** A
  /// missing `$HOME` on a normal login shell means the environment was
  /// deliberately stripped -- a `env -i` wrapper, a daemon with a minimal
  /// environment, a container built without one. Telling that operator to
  /// export a variable hides that Intent is being run somewhere it cannot
  /// have per-user state at all.
  #[error("cannot locate your home directory: $HOME is not set in this environment")]
  NoHome,
}

impl crate::remedy::Remedy for UserStateError {
  fn remedy(&self) -> String {
    "per-user state (skills, subagents, extensions) lives under your home directory, so this command cannot run in an environment without one. If you are inside a wrapper that strips the environment, run it outside; the project commands do not need $HOME and are unaffected.".to_string()
  }
}

/// The operator's home directory.
///
/// The one ambient read in this module, kept in a single function so the rest
/// stays a pure mapping a test can drive against any root it likes -- the same
/// split `install.rs` uses, and the reason its walk has real tests rather than
/// one test of whatever tree the suite happens to run in.
pub fn home() -> Result<PathBuf, UserStateError> {
  match std::env::var("HOME") {
    Ok(h) if !h.is_empty() => Ok(PathBuf::from(h)),
    _ => Err(UserStateError::NoHome),
  }
}

/// `~/.intent` -- Intent's own per-user directory.
pub fn intent_dir() -> Result<PathBuf, UserStateError> {
  Ok(home()?.join(".intent"))
}

/// `~/.intent/home` -- the one line naming this machine's Intent install root.
///
/// **THE POINTER THE PRE-COMMIT SHIM READS, AND THE ONLY THING IT READS**
/// (hv ruling 1, 2026-08-27). The gate stopped being copied into each project;
/// a shim resolves the install root from this file and execs the one gate body
/// out of it. See `lib/templates/hooks/pre-commit-shim.sh`.
///
/// **A CACHE THE SOURCE PUBLISHES ABOUT ITSELF.** The value is
/// [`crate::install::home`]'s answer and nothing else's -- the moment a second
/// thing can write here there are two answers to a question that must have one,
/// which is the class the shim exists to remove rather than relocate.
///
/// It lives under [`intent_dir`] rather than beside the binary on purpose: it
/// describes THIS MACHINE, and a binary that has been moved, relinked or
/// replaced must not be able to take its own pointer with it.
pub fn home_pointer() -> Result<PathBuf, UserStateError> {
  Ok(intent_dir()?.join("home"))
}

/// `~/.intent/config.json` -- the operator's own Intent configuration.
///
/// **A v3-PRIVATE PATH, AND THAT IS THE CLASS RULING RATHER THAN A CHOICE MADE
/// HERE.** v2 keeps this at `~/.config/intent/config.json`; vc's rule, hv
/// adopted 2026-08-22, is that every v3 per-user store gets its own path and
/// never reads or writes v2's. Nothing in this crate reads v2's file and
/// nothing should: a shared config is how two tools that can never be taught
/// about each other come to disagree about who the operator is.
///
/// It sits under [`intent_dir`] beside [`home_pointer`], so the whole of
/// Intent's per-user state is one directory an operator can inspect or delete.
pub fn global_config() -> Result<PathBuf, UserStateError> {
  Ok(intent_dir()?.join("config.json"))
}

/// The operator's login name, when the environment names one.
///
/// **`$USER` IS GRANTED FOR `bootstrap` AND CONFINED HERE** -- hv, 2026-08-27,
/// with the row and the reason in `no_intent_home.rs`. It is read in this
/// module for the same purpose `HOME` is: so the grant stays one file wide and
/// a second reader anywhere else fails exactly the way an unapproved variable
/// does.
///
/// **`None` IS A NORMAL ANSWER, NOT AN ERROR, AND THE DIFFERENCE FROM
/// [`home`] IS THE POINT.** A missing `HOME` means per-user state cannot exist,
/// which is a refusal. A missing `USER` means only that nobody can be named --
/// `bootstrap` writes the rest of the config and reports the identity as
/// unset, which is a true statement the operator can act on in one edit.
/// Returning a `Result` here would push a decision the caller has already made
/// into an error path it would have to unwrap anyway.
///
/// **AND IT IS NOT A FALLBACK CHAIN.** No `LOGNAME`, no `whoami`, no `git
/// config user.name`. hv ruled the source; a second source consulted when the
/// first is empty is how an identity comes to depend on which machine the
/// command ran on.
pub fn author() -> Option<String> {
  match std::env::var("USER") {
    Ok(u) if !u.trim().is_empty() => Some(u.trim().to_string()),
    _ => None,
  }
}

/// `~/.claude` -- Claude Code's per-user directory, which Intent installs into
/// but does not own.
///
/// **INTENT IS A GUEST HERE AND THE DISTINCTION IS LOAD-BEARING.** Everything
/// under `intent_dir()` is ours to structure; everything under this path has a
/// layout Claude Code defines, so a v3-specific filename is available in the
/// first and not the second. That asymmetry is exactly why the skills manifest
/// could be moved to its own path and the installed skills could not.
pub fn claude_dir() -> Result<PathBuf, UserStateError> {
  Ok(home()?.join(".claude"))
}

/// Where v3 records what IT installed.
///
/// See [`crate::skills::MANIFEST_RELATIVE`] for why this is not v2's file.
pub fn skills_manifest() -> Result<PathBuf, UserStateError> {
  Ok(intent_dir()?.join(crate::skills::MANIFEST_RELATIVE))
}

/// Where installed skills land, which is Claude Code's layout and not ours.
///
/// **SHARED WITH v2 AND UNAVOIDABLY SO.** This is the directory Claude Code
/// reads; a v3-private one would install skills nothing loads. The manifests
/// separating is what stops the mutual clobber -- each tool now compares
/// against its own record of what it wrote, rather than against a number the
/// other one computed by a different function.
pub fn skills_target() -> Result<PathBuf, UserStateError> {
  Ok(claude_dir()?.join("skills"))
}

/// The extension base, when extensions are wired.
///
/// **ALWAYS `None` TODAY, AND IT IS A HELD RULING RATHER THAN AN OVERSIGHT.**
/// v2 resolves this through `$INTENT_EXT_DIR` and `$INTENT_EXT_DISABLE`, and
/// **hv granted `$HOME` and nothing else** -- reading two more variables off
/// the back of that grant is precisely the quiet addition the invariant's own
/// failure message forbids. `rules.rs` is parked on the identical seam.
///
/// **THE CONSEQUENCE IS NAMED RATHER THAN SWALLOWED:** an operator with skills
/// or rules under `~/.intent/ext` sees them from v2 and not from v3, and
/// `Provenance::Ext` is reachable in the library and unreachable from the CLI.
/// Defaulting to `~/.intent/ext` here without the two variables would be worse
/// than not wiring it: an operator who set `INTENT_EXT_DISABLE=1` would have
/// their extensions silently switched back on.
pub fn ext_base() -> Option<PathBuf> {
  None
}

#[cfg(test)]
mod tests {
  use super::*;

  /// The paths hang off one another, so a home relocation moves all of them
  /// and none can drift onto a different root.
  #[test]
  fn every_path_descends_from_the_one_home() {
    let Ok(h) = home() else {
      return;
    };
    for path in [
      intent_dir().unwrap(),
      claude_dir().unwrap(),
      skills_manifest().unwrap(),
      skills_target().unwrap(),
    ] {
      assert!(
        path.starts_with(&h),
        "{} escaped the home directory",
        path.display()
      );
    }
  }

  /// **THE MANIFEST PATH IS THE CLASS RULING'S ONE MECHANICAL CHECK.** If this
  /// ever equals v2's file, the two tools resume overwriting each other
  /// forever while both report success.
  #[test]
  fn the_skills_manifest_is_not_v2s() {
    let Ok(path) = skills_manifest() else {
      return;
    };
    assert!(!path.ends_with("skills/installed-skills.json"));
    assert!(path.starts_with(intent_dir().unwrap()));
  }

  /// Extensions stay unwired until the two variables they need are ruled on.
  #[test]
  fn extensions_are_not_quietly_enabled() {
    assert!(ext_base().is_none());
  }
}
