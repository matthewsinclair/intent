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
/// It sits under [`intent_dir`] beside [`home_pointer`], so the operator's
/// CONFIGURATION is one directory they can inspect or delete.
///
/// **THAT USED TO SAY "the whole of Intent's per-user state" AND THAT WAS
/// FALSE IN BOTH READINGS** (vc, 2026-08-29). D19 puts the daemon's logs,
/// plist and PID file under `~/.local/share/intent/`, so Intent's per-user
/// state has always been TWO directories -- and the sentence misled whichever
/// one you read it as naming, most expensively for whoever writes the plist.
/// **The split is real and deliberate rather than a defect to converge:**
/// configuration is state an operator authors and may delete, and daemon
/// runtime state is not. See [`daemon_state_dir`]. What was wrong was a
/// sentence claiming a unity the layout never had.
pub fn global_config() -> Result<PathBuf, UserStateError> {
  Ok(intent_dir()?.join("config.json"))
}

/// `~/.local/share/intent/` -- where the daemon keeps its runtime state.
///
/// **D19's DIRECTORY RATHER THAN [`intent_dir`], AND THE SPLIT IS A CONFLICT
/// THIS FUNCTION INHERITS RATHER THAN CREATES.** D19 puts the LaunchAgent
/// plist under `~/Library/LaunchAgents/` and the daemon's logs under
/// `~/.local/share/intent/`; the module note above says the whole of Intent's
/// per-user state is one directory, meaning `~/.intent`. Both cannot be true.
/// A numbered decision outranks a module's habit, and a socket is daemon
/// runtime state of exactly the class D19 addressed -- so the daemon's
/// footprint stays together, beside the logs and the PID file that D19 already
/// placed, rather than being split across two roots by where its first
/// consumer happened to look. **The two-homes problem is open with vc and
/// lands on whoever writes the plist, whichever root wins.**
///
/// **RUNTIME STATE, NOT CONFIGURATION, AND THAT IS WHY IT IS NOT UNDER
/// [`intent_dir`] EVEN ON THE MERITS.** `~/.intent` is described as something
/// an operator can inspect or delete; a live socket and a PID file are things
/// deleting which orphans a running daemon. Keeping them apart means the
/// invitation to delete stays honest.
pub fn daemon_state_dir() -> Result<PathBuf, UserStateError> {
  Ok(daemon_state_dir_under(&home()?))
}

/// [`daemon_state_dir`]'s layout, against any root.
///
/// **THE SPLIT `install.rs` USES, AND FOR THE REASON THIS MODULE ALREADY GIVES
/// FOR IT**: the one ambient read stays in [`home`] and the rest is a pure
/// mapping a test can drive against a temp directory. The alternative is a
/// test that spells the layout out for itself, which makes the test a SECOND
/// HOME for the path -- and a second home that agrees today is exactly the one
/// that stops agreeing without saying so.
pub fn daemon_state_dir_under(root: &std::path::Path) -> PathBuf {
  root.join(".local").join("share").join("intent")
}

/// `~/.local/share/intent/intentd.sock` -- the address `intentd` binds and the
/// CLI probes.
///
/// **ONE HOME FOR AN ADDRESS TWO BINARIES MUST AGREE ON.** The routing rule
/// lives in [`crate::daemon`] and takes a path; the path is named here because
/// `$HOME` is confined to this module and nowhere else can read it. A second
/// spelling anywhere would be a daemon listening where the CLI never looks,
/// and the failure is silent in the worst direction -- a CLI that finds no
/// daemon simply runs in-process, correctly, forever.
///
/// **THE PATH IS SHORT ON PURPOSE.** `sun_path` is 104 bytes on macOS and 108
/// on Linux, and a unix socket address that overruns it fails at bind and
/// connect with an error naming neither the limit nor the path. Anything
/// deeper than this needs that limit checked rather than assumed.
pub fn daemon_socket() -> Result<PathBuf, UserStateError> {
  Ok(daemon_socket_under(&home()?))
}

/// [`daemon_socket`]'s layout, against any root. See [`daemon_state_dir_under`].
pub fn daemon_socket_under(root: &std::path::Path) -> PathBuf {
  daemon_state_dir_under(root).join("intentd.sock")
}

/// `~/.local/share/intent/intentd.addr` -- the loopback address the running
/// daemon published for itself.
///
/// **THERE IS NO PORT CONSTANT ANYWHERE AND THAT IS THE RULING, NOT AN
/// OMISSION** (hv, 2026-08-29). The daemon binds `127.0.0.1:0`, lets the kernel
/// assign, and WRITES what it got here; every client reads it. A named default
/// and a compile-time environment variable were both priced and lost: a literal
/// collides eventually, and a build that needs an env var to compile is a cost
/// this estate has no targets to justify.
///
/// **IT COSTS CLIENTS NOTHING BECAUSE THEY ALREADY READ THIS DIRECTORY.** The
/// socket path is resolved from here too, so an address file is one more read
/// in a place already being read -- which is why `--browser` can build its URL
/// from what it found rather than from a constant it would have to keep in step.
pub fn daemon_address_file() -> Result<PathBuf, UserStateError> {
  Ok(daemon_address_file_under(&home()?))
}

/// [`daemon_address_file`]'s layout, against any root.
pub fn daemon_address_file_under(root: &std::path::Path) -> PathBuf {
  daemon_state_dir_under(root).join("intentd.addr")
}

/// `~/.local/share/intent/intentd.token` -- the secret the HTTP face requires
/// and the socket face does not (D56).
///
/// **THE TWO TRANSPORTS HAVE DIFFERENT AUTHZ STORIES AND THIS FILE IS THE
/// SECOND ONE.** The workspace manifest records the split beside `axum`:
/// filesystem permissions are the socket's authz, and the HTTP half carries
/// one auto-generated token. **Loopback is not a permission boundary** -- every
/// local process reaches `127.0.0.1`, and so does any page the operator's
/// browser happens to be showing -- so the port needs a check the socket does
/// not, and treating them uniformly gives the socket a check it does not need
/// or the port none at all.
///
/// **IT LIVES BESIDE THE ADDRESS FILE BECAUSE IT HAS THE ADDRESS FILE'S
/// LIFETIME.** Both are written by a starting daemon, both are meaningless
/// when it stops, and both are read by a client asking *where do I connect and
/// what do I say*. A token under [`global_config`] would be operator
/// configuration, which it is not: nobody authors it and deleting it costs
/// nothing but a restart.
pub fn daemon_token_file() -> Result<PathBuf, UserStateError> {
  Ok(daemon_token_file_under(&home()?))
}

/// [`daemon_token_file`]'s layout, against any root.
pub fn daemon_token_file_under(root: &std::path::Path) -> PathBuf {
  daemon_state_dir_under(root).join("intentd.token")
}

/// The file whose LOCK means "a daemon is running here" (`AC-08.12`).
///
/// **A SEPARATE FILE FROM THE SOCKET, AND THE SEPARATION IS THE MECHANISM.**
/// The lock has to survive being asked about while the socket is being
/// unlinked and rebound, and a lock on the socket itself would vanish with it.
/// It is also the one file here whose CONTENT is irrelevant -- what carries the
/// meaning is the kernel's lock on the open descriptor, which is released on
/// process death by any means including `SIGKILL`. That is the whole reason it
/// exists rather than a pid file: **a pid file goes stale and a lock cannot.**
pub fn daemon_lock_under(root: &std::path::Path) -> PathBuf {
  daemon_state_dir_under(root).join("intentd.lock")
}

/// [`daemon_lock_under`] against the operator's own home.
pub fn daemon_lock() -> Result<PathBuf, UserStateError> {
  Ok(daemon_lock_under(&home()?))
}

/// `~/Library/LaunchAgents/com.matthewsinclair.intentd.plist` -- the enrolment.
///
/// **D19's LOCATION, AND IT IS THE ONE PATH HERE THAT IS NOT OURS TO CHOOSE.**
/// Every other file in this module sits where Intent decided to put it;
/// `launchd` only reads per-user agents from `~/Library/LaunchAgents/`, so this
/// is a location the platform fixes and D19 records rather than selects.
///
/// **IT IS DELIBERATELY NOT UNDER [`daemon_state_dir`], THOUGH EVERYTHING ELSE
/// THE DAEMON OWNS IS.** That split is D19's and it is the reason the comment
/// on [`global_config`] once misled: Intent's per-user footprint is not one
/// directory and never was. The plist is the ONE piece of daemon state another
/// program owns the reading of, which is exactly why it lives where that
/// program looks.
pub fn launch_agent_plist() -> Result<PathBuf, UserStateError> {
  Ok(launch_agent_plist_under(&home()?))
}

/// [`launch_agent_plist`]'s layout, against any root.
pub fn launch_agent_plist_under(root: &std::path::Path) -> PathBuf {
  root
    .join("Library")
    .join("LaunchAgents")
    .join(format!("{LAUNCH_AGENT_LABEL}.plist"))
}

/// The reverse-domain label `launchd` knows the daemon by (D19).
///
/// **ONE HOME, BECAUSE THREE THINGS MUST AGREE ABOUT IT AND TWO OF THEM ARE
/// NOT FILES.** The plist's own `Label` key, the plist's FILENAME, and every
/// `launchctl` argument naming the job are the same string; a second spelling
/// anywhere means `launchctl` operates on a job that does not exist and says
/// so in a way that reads like the daemon being absent.
pub const LAUNCH_AGENT_LABEL: &str = "com.matthewsinclair.intentd";

/// Where the daemon's stdout goes (D19: logs at `~/.local/share/intent/`).
///
/// **NAMED HERE RATHER THAN IN THE PLIST WRITER, BECAUSE TWO PROGRAMS NEED IT
/// AND ONLY ONE OF THEM WRITES THE PLIST.** `launchd` is told this path once,
/// at enrolment; whoever answers *where are the logs* has to produce the same
/// path months later without reading the plist back. A literal in the plist
/// writer would be correct at enrolment and unavailable to every reader after.
pub fn daemon_log_under(root: &std::path::Path) -> PathBuf {
  daemon_state_dir_under(root).join("intentd.log")
}

/// [`daemon_log_under`] against the operator's own home.
pub fn daemon_log() -> Result<PathBuf, UserStateError> {
  Ok(daemon_log_under(&home()?))
}

/// Where the daemon's stderr goes.
///
/// **SEPARATE FROM [`daemon_log_under`] BECAUSE THE DAEMON ALREADY TREATS THEM
/// AS SEPARATE.** `intentd` reports refusals and its served-and-not-watched
/// notices on stderr and says nothing on stdout in normal running, so merging
/// them would bury the only lines anybody reads under the ones nobody does.
pub fn daemon_error_log_under(root: &std::path::Path) -> PathBuf {
  daemon_state_dir_under(root).join("intentd.err.log")
}

/// [`daemon_error_log_under`] against the operator's own home.
pub fn daemon_error_log() -> Result<PathBuf, UserStateError> {
  Ok(daemon_error_log_under(&home()?))
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
      daemon_state_dir().unwrap(),
      daemon_socket().unwrap(),
      daemon_address_file().unwrap(),
      daemon_lock().unwrap(),
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
