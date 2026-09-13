//! Where the OPERATOR's own per-user state lives -- as distinct from where the
//! tool is installed, and from where the project is.
//!
//! Three different questions with three different failure modes, and this
//! estate has conflated two of them before (issue 0025). `project.rs` answers
//! *where is the tree I am standing in*. `install.rs` answers *where is the
//! tool I am running*. This answers *where is the state that belongs to the
//! person running it* -- configuration, the install pointer, the payload
//! manifests, the daemon's logs and the daemon's socket.
//!
//! **THE LAYOUT IS THE XDG BASE DIRECTORY SPECIFICATION** (hv, 2026-09-13,
//! ST0074 WP-05). Configuration under `$XDG_CONFIG_HOME/intent`, data under
//! `$XDG_DATA_HOME/intent`, logs and build output under `$XDG_STATE_HOME/intent`,
//! and what a running daemon publishes under `$XDG_RUNTIME_DIR/intent`. Each
//! variable takes the specification's default when it is unset, empty or not
//! an absolute path. The runtime directory falls back to `<state>/run` without
//! the warning the specification suggests, because macOS never sets
//! `XDG_RUNTIME_DIR` and the warning would print on every command there.
//! [`Dirs`] is that mapping, and nothing else knows it.
//!
//! **THIS IS THE ONE PLACE THOSE VARIABLES ARE READ, AND THE CONFINEMENT IS THE
//! POINT OF THE RULINGS RATHER THAN A TIDINESS PREFERENCE.** `$HOME` (hv,
//! 2026-08-22), `$USER` (hv, 2026-08-27) and the four `XDG_*` variables (hv,
//! 2026-09-13) are rows in `no_intent_home.rs`'s `ALLOWED`, each confined to
//! this file by path. A second reader anywhere else fails the same way an
//! unapproved variable does, so the audit surface stays one file wide.
//!
//! **`~/.intent/` IS READ BY [`migrate_legacy`] AND BY NOTHING ELSE.** It was
//! the layout up to 3.0.1; the first command of a build that knows this one
//! moves what it owns out of it.

use std::path::{Path, PathBuf};

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
  #[error("could not move `{}` to `{}`", from.display(), to.display())]
  Unmovable {
    from: PathBuf,
    to: PathBuf,
    #[source]
    cause: std::io::Error,
  },
  #[error("could not remove `{}`", path.display())]
  Unremovable {
    path: PathBuf,
    #[source]
    cause: std::io::Error,
  },
}

impl crate::remedy::Remedy for UserStateError {
  fn remedy(&self) -> String {
    match self {
      Self::NoHome => "per-user state (skills, subagents, extensions) lives under your home directory, so this command cannot run in an environment without one. If you are inside a wrapper that strips the environment, run it outside; the project commands do not need $HOME and are unaffected.".to_string(),
      Self::Unmovable { .. } => "Intent keeps its per-user files in the XDG layout, and this one is still where an earlier build put it. Move it to the path named above by hand, then re-run. A move across filesystems is the usual cause, when $XDG_CONFIG_HOME or $XDG_DATA_HOME is on another volume from your home directory.".to_string(),
      Self::Unremovable { .. } => "the file is left over from an earlier layout and nothing reads it. Remove it by hand, then re-run.".to_string(),
    }
  }
}

/// The operator's home directory.
///
/// The one `$HOME` read in this module, kept in a single function so the rest
/// stays a pure mapping a test can drive against any root it likes -- the same
/// split `install.rs` uses, and the reason its walk has real tests rather than
/// one test of whatever tree the suite happens to run in.
pub fn home() -> Result<PathBuf, UserStateError> {
  match std::env::var("HOME") {
    Ok(h) if !h.is_empty() => Ok(PathBuf::from(h)),
    _ => Err(UserStateError::NoHome),
  }
}

/// The four XDG variables as the environment gave them, before any default.
#[derive(Debug, Default, Clone)]
pub struct Xdg {
  pub config_home: Option<String>,
  pub data_home: Option<String>,
  pub state_home: Option<String>,
  pub runtime_dir: Option<String>,
}

/// Intent's own directory of each XDG kind, resolved.
///
/// **A VALUE, NOT A SET OF FUNCTIONS OF `$HOME`, BECAUSE THE ANSWER NOW HAS
/// FIVE INPUTS.** Every `*_under` below takes one, so a test builds it with
/// [`Dirs::at_home`] against a temporary directory and never has to mutate the
/// process environment, which would race every sibling test.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dirs {
  /// `$HOME`: the root of the defaults, and of the two directories other
  /// programs define, `~/Library/LaunchAgents/` and `~/.claude/`.
  pub home: PathBuf,
  /// `$XDG_CONFIG_HOME/intent`: what the operator authors.
  pub config: PathBuf,
  /// `$XDG_DATA_HOME/intent`: what Intent writes and keeps.
  pub data: PathBuf,
  /// `$XDG_STATE_HOME/intent`: logs and build output, which survive a restart
  /// and nobody authors.
  pub state: PathBuf,
  /// `$XDG_RUNTIME_DIR/intent`, else `<state>/run`: what a running daemon
  /// publishes, meaningless once it stops.
  pub runtime: PathBuf,
}

impl Dirs {
  /// The layout under `home` with every variable unset.
  pub fn at_home(home: &Path) -> Dirs {
    Dirs::resolve(home, &Xdg::default())
  }

  /// The pure mapping from `home` and the variables to the layout.
  ///
  /// **A VALUE THAT IS NOT AN ABSOLUTE PATH IS IGNORED**, as the specification
  /// says, and an empty one is such a value.
  pub fn resolve(home: &Path, xdg: &Xdg) -> Dirs {
    let base = |value: &Option<String>, default: &str| match value.as_deref().map(Path::new) {
      Some(path) if path.is_absolute() => path.to_path_buf(),
      _ => home.join(default),
    };
    let state = base(&xdg.state_home, ".local/state").join("intent");
    let runtime = match xdg.runtime_dir.as_deref().map(Path::new) {
      Some(path) if path.is_absolute() => path.join("intent"),
      _ => state.join("run"),
    };
    Dirs {
      home: home.to_path_buf(),
      config: base(&xdg.config_home, ".config").join("intent"),
      data: base(&xdg.data_home, ".local/share").join("intent"),
      state,
      runtime,
    }
  }
}

/// This operator's layout: `$HOME` and the four `XDG_*` variables, read here.
pub fn dirs() -> Result<Dirs, UserStateError> {
  let xdg = Xdg {
    config_home: std::env::var("XDG_CONFIG_HOME").ok(),
    data_home: std::env::var("XDG_DATA_HOME").ok(),
    state_home: std::env::var("XDG_STATE_HOME").ok(),
    runtime_dir: std::env::var("XDG_RUNTIME_DIR").ok(),
  };
  Ok(Dirs::resolve(&home()?, &xdg))
}

/// `$XDG_DATA_HOME/intent/home` -- the one line naming this machine's Intent
/// install root.
///
/// **THE POINTER THE PRE-COMMIT SHIM READS, AND THE ONLY THING IT READS**
/// (hv ruling 1, 2026-08-27). The gate stopped being copied into each project;
/// a shim resolves the install root from this file and execs the one gate body
/// out of it. See `lib/templates/hooks/pre-commit-shim.sh`, which spells this
/// path for itself because it is shell, and so is the second home this layout
/// cannot close.
///
/// **A CACHE THE SOURCE PUBLISHES ABOUT ITSELF.** The value is
/// [`crate::install::home`]'s answer and nothing else's -- the moment a second
/// thing can write here there are two answers to a question that must have one.
///
/// It is data rather than configuration: nobody authors it, and it describes
/// THIS MACHINE, so a binary that has been moved, relinked or replaced must not
/// be able to take its own pointer with it.
pub fn home_pointer() -> Result<PathBuf, UserStateError> {
  Ok(dirs()?.data.join("home"))
}

/// `$XDG_CONFIG_HOME/intent/config.json` -- the operator's own configuration:
/// `author`, written by `intent bootstrap`, and the explorer's settings,
/// written by `/settings`.
pub fn global_config() -> Result<PathBuf, UserStateError> {
  Ok(global_config_under(&dirs()?))
}

/// [`global_config`]'s layout, against any [`Dirs`].
pub fn global_config_under(dirs: &Dirs) -> PathBuf {
  dirs.config.join("config.json")
}

/// `$XDG_CONFIG_HOME/intent/projects.json` -- the project registry (ST0074
/// WP-03): the Intent projects this machine knows about. See [`crate::projects`].
///
/// **CONFIGURATION, BECAUSE THE OPERATOR AUTHORS IT** as much as `intent
/// explore` and `intent discover` do, and `intentd` only reads it.
pub fn project_registry() -> Result<PathBuf, UserStateError> {
  Ok(project_registry_under(&dirs()?))
}

/// [`project_registry`]'s layout, against any [`Dirs`].
pub fn project_registry_under(dirs: &Dirs) -> PathBuf {
  dirs.config.join("projects.json")
}

/// Where `bin/devbin macos app-build` leaves the built `Intent.app` bundles.
///
/// Mirrors `APP_STATE_DIR` in `bin/.devbin/cmd/macos`. **That is a second home
/// for this layout and it is not one this module can close**: the builder is a
/// shell verb and the reader is Rust, so they cannot share a constant. What
/// keeps them honest is that a disagreement makes `intent app status` report
/// `not installed` on a machine that has just built the app -- loud, and on the
/// verb whose whole subject is where the bundle is. The devbin verb's
/// `$INTENT_MACOS_STATE_DIR` override is deliberately not carried across: a
/// shipped binary must not have its answer depend on a variable only a
/// developer sets.
pub fn macos_app_build_dir() -> Result<PathBuf, UserStateError> {
  Ok(macos_app_build_dir_under(&dirs()?))
}

/// [`macos_app_build_dir`]'s layout, against any [`Dirs`].
pub fn macos_app_build_dir_under(dirs: &Dirs) -> PathBuf {
  dirs
    .state
    .join("build")
    .join("macos")
    .join("Build")
    .join("Products")
}

/// The directory a running daemon publishes into, and watches.
///
/// **RUNTIME STATE, NOT CONFIGURATION AND NOT DATA.** A live socket and a lock
/// are things deleting which orphans a running daemon, and nothing in them
/// outlives it -- which is also why `intentd` exits when this directory is
/// removed out from under it.
pub fn daemon_runtime_dir_under(dirs: &Dirs) -> PathBuf {
  dirs.runtime.clone()
}

/// `<runtime>/intentd.sock` -- the address `intentd` binds and the CLI probes.
///
/// **ONE HOME FOR AN ADDRESS TWO BINARIES MUST AGREE ON.** The routing rule
/// lives in [`crate::daemon`] and takes a [`Dirs`]; the path is named here. A
/// second spelling anywhere would be a daemon listening where the CLI never
/// looks, and the failure is silent in the worst direction -- a CLI that finds
/// no daemon simply runs in-process, correctly, forever.
///
/// **THE PATH IS SHORT ON PURPOSE.** `sun_path` is 104 bytes on macOS and 108
/// on Linux, and a unix socket address that overruns it fails at bind and
/// connect with an error naming neither the limit nor the path. The macOS
/// default, `~/.local/state/intent/run/intentd.sock`, leaves a home path over
/// sixty bytes of room; anything deeper needs that limit checked.
pub fn daemon_socket_under(dirs: &Dirs) -> PathBuf {
  dirs.runtime.join("intentd.sock")
}

/// `<runtime>/intentd.addr` -- the loopback address the running daemon
/// published for itself.
///
/// **THERE IS NO PORT CONSTANT ANYWHERE AND THAT IS THE RULING, NOT AN
/// OMISSION** (hv, 2026-08-29). The daemon binds `127.0.0.1:0`, lets the kernel
/// assign, and WRITES what it got here; every client reads it.
pub fn daemon_address_file_under(dirs: &Dirs) -> PathBuf {
  dirs.runtime.join("intentd.addr")
}

/// `<runtime>/intentd.token` -- the secret the HTTP face requires and the
/// socket face does not (D56).
///
/// **LOOPBACK IS NOT A PERMISSION BOUNDARY** -- every local process reaches
/// `127.0.0.1`, and so does any page the operator's browser happens to be
/// showing -- so the port needs a check the socket does not. **It lives beside
/// the address file because it has the address file's lifetime**: both are
/// written by a starting daemon and meaningless when it stops, and nobody
/// authors either.
pub fn daemon_token_file_under(dirs: &Dirs) -> PathBuf {
  dirs.runtime.join("intentd.token")
}

/// The file whose LOCK means "a daemon is running here" (`AC-08.12`).
///
/// **A SEPARATE FILE FROM THE SOCKET, AND THE SEPARATION IS THE MECHANISM.**
/// The lock has to survive being asked about while the socket is being
/// unlinked and rebound. What carries the meaning is the kernel's lock on the
/// open descriptor, released on process death by any means including
/// `SIGKILL`: **a pid file goes stale and a lock cannot.**
pub fn daemon_lock_under(dirs: &Dirs) -> PathBuf {
  dirs.runtime.join("intentd.lock")
}

/// `~/Library/LaunchAgents/com.matthewsinclair.intentd.plist` -- the enrolment.
///
/// **THE ONE PATH HERE THAT IS NOT OURS TO CHOOSE, AND NOT XDG's EITHER.**
/// `launchd` only reads per-user agents from `~/Library/LaunchAgents/`, so this
/// is a location the platform fixes and D19 records rather than selects.
pub fn launch_agent_plist_under(dirs: &Dirs) -> PathBuf {
  dirs
    .home
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

/// `<state>/intentd.log` -- where the daemon's stdout goes.
///
/// **NAMED HERE RATHER THAN IN THE PLIST WRITER, BECAUSE TWO PROGRAMS NEED IT
/// AND ONLY ONE OF THEM WRITES THE PLIST.** `launchd` is told this path once,
/// at enrolment; whoever answers *where are the logs* has to produce the same
/// path months later without reading the plist back.
pub fn daemon_log_under(dirs: &Dirs) -> PathBuf {
  dirs.state.join("intentd.log")
}

/// `<state>/intentd.err.log` -- where the daemon's stderr goes.
///
/// **SEPARATE FROM [`daemon_log_under`] BECAUSE THE DAEMON ALREADY TREATS THEM
/// AS SEPARATE.** `intentd` reports refusals and its served-and-not-watched
/// notices on stderr and says nothing on stdout in normal running, so merging
/// them would bury the only lines anybody reads under the ones nobody does.
pub fn daemon_error_log_under(dirs: &Dirs) -> PathBuf {
  dirs.state.join("intentd.err.log")
}

/// The operator's login name, when the environment names one.
///
/// **`$USER` IS GRANTED FOR `bootstrap` AND CONFINED HERE** -- hv, 2026-08-27,
/// with the row and the reason in `no_intent_home.rs`.
///
/// **`None` IS A NORMAL ANSWER, NOT AN ERROR, AND THE DIFFERENCE FROM
/// [`home`] IS THE POINT.** A missing `HOME` means per-user state cannot exist,
/// which is a refusal. A missing `USER` means only that nobody can be named --
/// `bootstrap` writes the rest of the config and reports the identity as
/// unset, which is a true statement the operator can act on in one edit.
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
/// under [`Dirs`] is ours to structure; everything under this path has a layout
/// Claude Code defines, so a v3-specific filename is available in the first and
/// not the second. That asymmetry is exactly why the skills manifest could be
/// given its own path and the installed skills could not.
pub fn claude_dir() -> Result<PathBuf, UserStateError> {
  Ok(home()?.join(".claude"))
}

/// Where v3 records what IT installed: `$XDG_DATA_HOME/intent/<kind's manifest>`.
///
/// See [`crate::payload::Kind::manifest_relative`] -- one answer PER KIND, so
/// the separation has to hold for each of them rather than once.
pub fn payload_manifest(kind: crate::payload::Kind) -> Result<PathBuf, UserStateError> {
  Ok(dirs()?.data.join(kind.manifest_relative()))
}

/// Where installed skills land, which is Claude Code's layout and not ours.
pub fn payload_target(kind: crate::payload::Kind) -> Result<PathBuf, UserStateError> {
  Ok(claude_dir()?.join(kind.target_subdir()))
}

/// The extension base, when extensions are wired.
///
/// **ALWAYS `None` TODAY, AND IT IS A HELD RULING RATHER THAN AN OVERSIGHT.**
/// hv ruled `ext` out of the 3.0.0 cut on 2026-08-30. When it is wired, its
/// directory is `$XDG_DATA_HOME/intent/ext`, which [`migrate_legacy`] already
/// moves an old one into. `Provenance::Ext` is reachable in the library and
/// unreachable from the CLI until then, and that consequence is named rather
/// than swallowed.
pub fn ext_base() -> Option<PathBuf> {
  None
}

/// What [`migrate_legacy`] moved, for the one line the CLI prints.
#[derive(Debug, PartialEq, Eq)]
pub struct Migrated {
  pub from: PathBuf,
  /// Each entry moved, with where it went.
  pub moved: Vec<(String, PathBuf)>,
  /// What is still in `from` afterwards. Empty means `from` was removed.
  pub left: Vec<String>,
}

impl std::fmt::Display for Migrated {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let moved: Vec<String> = self
      .moved
      .iter()
      .map(|(name, to)| format!("{name} to {}", to.display()))
      .collect();
    write!(
      f,
      "note: moved {} out of {} into the XDG layout",
      moved.join(", "),
      self.from.display()
    )?;
    if self.left.is_empty() {
      write!(f, ", and removed it")
    } else {
      write!(
        f,
        ", and kept it for what Intent does not own: {}",
        self.left.join(", ")
      )
    }
  }
}

/// The entries the 3.0.1 layout kept under `~/.intent/`, and where each goes.
fn legacy_moves(dirs: &Dirs) -> [(&'static str, PathBuf); 6] {
  [
    ("config.json", global_config_under(dirs)),
    ("home", dirs.data.join("home")),
    ("skills", dirs.data.join("skills")),
    ("subagents", dirs.data.join("subagents")),
    ("agents", dirs.data.join("agents")),
    ("ext", dirs.data.join("ext")),
  ]
}

/// Move what Intent owns out of `~/.intent/` into the layout, once.
///
/// **`Ok(None)` WHEN THERE WAS NOTHING TO MOVE**, which is every run after the
/// first, so the CLI can call this on every command and print only when it did
/// something.
///
/// **AN ENTRY ALREADY AT ITS NEW PATH IS LEFT WHERE IT IS**, except a
/// configuration that is not v3's: v2 is ignored (hv, 2026-09-13) and its
/// `~/.config/intent/config.json` is replaced. So a `~/.intent/` recreated by
/// an older binary after the move is never read again rather than overwriting
/// newer state.
///
/// **`~/.intent/` IS REMOVED ONLY WHEN EMPTY.** Anything Intent did not put
/// there is somebody else's, and a migration that deleted it would be the one
/// destructive path in a move.
pub fn migrate_legacy(dirs: &Dirs) -> Result<Option<Migrated>, UserStateError> {
  let from = dirs.home.join(".intent");
  if !from.is_dir() {
    return Ok(None);
  }
  let mut moved = Vec::new();
  for (name, to) in legacy_moves(dirs) {
    let source = from.join(name);
    if !source.exists() || (to.exists() && !replaceable(name, &to)) {
      continue;
    }
    let unmovable = |cause: std::io::Error| UserStateError::Unmovable {
      from: source.clone(),
      to: to.clone(),
      cause,
    };
    if let Some(parent) = to.parent() {
      std::fs::create_dir_all(parent).map_err(&unmovable)?;
    }
    std::fs::rename(&source, &to).map_err(&unmovable)?;
    moved.push((name.to_string(), to));
  }
  if moved.is_empty() {
    return Ok(None);
  }
  let unremovable = |cause: std::io::Error| UserStateError::Unremovable {
    path: from.clone(),
    cause,
  };
  let mut left: Vec<String> = std::fs::read_dir(&from)
    .map_err(unremovable)?
    .flatten()
    .map(|entry| entry.file_name().to_string_lossy().into_owned())
    .collect();
  left.sort();
  if left.is_empty() {
    std::fs::remove_dir(&from).map_err(|cause| UserStateError::Unremovable {
      path: from.clone(),
      cause,
    })?;
  }
  Ok(Some(Migrated { from, moved, left }))
}

/// Whether the file already at an entry's new path gives way to the old one:
/// only a configuration that is not v3's.
fn replaceable(name: &str, to: &Path) -> bool {
  name == "config.json" && !is_v3_config(to)
}

/// **`intent_version` IS THE DISCRIMINATOR, BY ITS MAJOR NUMBER.** v2 and v3
/// both write the key, so its presence says nothing; v2's reads `2.x`.
fn is_v3_config(path: &Path) -> bool {
  std::fs::read_to_string(path)
    .ok()
    .and_then(|text| serde_json::from_str::<serde_json::Value>(&text).ok())
    .and_then(|doc| {
      doc
        .get("intent_version")?
        .as_str()?
        .split('.')
        .next()?
        .parse::<u32>()
        .ok()
    })
    .is_some_and(|major| major >= 3)
}

/// The layout a 3.0.1 daemon published its runtime files in:
/// `~/.local/share/intent`. `None` when that is this layout's runtime
/// directory too, so there is nothing separate to look at.
///
/// **READ BY `daemon stop`, SO AN OLD DAEMON STILL RUNNING ACROSS THE UPGRADE IS
/// STOPPED RATHER THAN LEFT SERVING BESIDE A NEW ONE.** A 3.0.1 daemon watches
/// `~/.local/share/intent` for removal, and that directory is this layout's
/// data directory, so it never goes away and the old daemon never exits on its
/// own.
pub fn legacy_daemon(dirs: &Dirs) -> Option<Dirs> {
  let runtime = dirs.home.join(".local").join("share").join("intent");
  (runtime != dirs.runtime).then(|| Dirs {
    runtime,
    ..dirs.clone()
  })
}

/// Remove what a stopped 3.0.1 daemon left in [`legacy_daemon`]'s directory,
/// returning the files removed. Only the daemon's own files: the directory is
/// this layout's data directory now.
pub fn remove_legacy_runtime(legacy: &Dirs) -> Result<Vec<PathBuf>, UserStateError> {
  let mut removed = Vec::new();
  for path in [
    daemon_socket_under(legacy),
    daemon_address_file_under(legacy),
    daemon_token_file_under(legacy),
    daemon_lock_under(legacy),
    legacy.runtime.join("intentd.log"),
    legacy.runtime.join("intentd.err.log"),
  ] {
    match std::fs::remove_file(&path) {
      Ok(()) => removed.push(path),
      Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
      Err(cause) => return Err(UserStateError::Unremovable { path, cause }),
    }
  }
  Ok(removed)
}

#[cfg(test)]
mod tests {
  use super::*;

  /// Each variable wins only as an absolute path; empty and relative values
  /// take the default, and the runtime directory falls back under state.
  #[test]
  fn xdg_values_win_only_as_absolute_paths() {
    let home = Path::new("/h");
    let dirs = Dirs::resolve(
      home,
      &Xdg {
        config_home: Some("/cfg".into()),
        data_home: Some(String::new()),
        state_home: Some("relative".into()),
        runtime_dir: None,
      },
    );
    assert_eq!(dirs.config, PathBuf::from("/cfg/intent"));
    assert_eq!(dirs.data, PathBuf::from("/h/.local/share/intent"));
    assert_eq!(dirs.state, PathBuf::from("/h/.local/state/intent"));
    assert_eq!(dirs.runtime, PathBuf::from("/h/.local/state/intent/run"));

    let linux = Dirs::resolve(
      home,
      &Xdg {
        runtime_dir: Some("/run/user/1000".into()),
        ..Xdg::default()
      },
    );
    assert_eq!(linux.runtime, PathBuf::from("/run/user/1000/intent"));
    assert_eq!(linux.config, PathBuf::from("/h/.config/intent"));
  }

  /// The daemon's files sit in the kind of directory their lifetime names.
  #[test]
  fn each_file_lives_under_its_kind() {
    let dirs = Dirs::at_home(Path::new("/h"));
    for path in [
      daemon_socket_under(&dirs),
      daemon_address_file_under(&dirs),
      daemon_token_file_under(&dirs),
      daemon_lock_under(&dirs),
    ] {
      assert!(path.starts_with(&dirs.runtime), "{}", path.display());
    }
    for path in [
      daemon_log_under(&dirs),
      daemon_error_log_under(&dirs),
      macos_app_build_dir_under(&dirs),
    ] {
      assert!(path.starts_with(&dirs.state), "{}", path.display());
    }
    assert_eq!(
      global_config_under(&dirs),
      PathBuf::from("/h/.config/intent/config.json")
    );
    assert!(launch_agent_plist_under(&dirs).starts_with("/h/Library/LaunchAgents"));
  }

  fn legacy_home() -> (tempfile::TempDir, Dirs) {
    let dir = tempfile::tempdir().expect("tempdir");
    let dirs = Dirs::at_home(dir.path());
    std::fs::create_dir_all(dir.path().join(".intent/skills")).expect("legacy skills");
    std::fs::write(dir.path().join(".intent/home"), "/install\n").expect("legacy pointer");
    std::fs::write(
      dir.path().join(".intent/skills/installed-skills.v3.json"),
      "{}",
    )
    .expect("legacy manifest");
    std::fs::write(
      dir.path().join(".intent/config.json"),
      r#"{"intent_version": "3.0.1", "author": "old"}"#,
    )
    .expect("legacy config");
    (dir, dirs)
  }

  /// The known entries move, a v2 config at the new path is replaced, and
  /// `~/.intent/` goes once it is empty.
  #[test]
  fn migration_moves_what_intent_owns_and_removes_the_emptied_directory() {
    let (dir, dirs) = legacy_home();
    std::fs::create_dir_all(&dirs.config).expect("config dir");
    std::fs::write(global_config_under(&dirs), r#"{"intent_version": "2.0.0"}"#)
      .expect("v2 config");

    let migrated = migrate_legacy(&dirs)
      .expect("migrate")
      .expect("something moved");

    let names: Vec<&str> = migrated.moved.iter().map(|(n, _)| n.as_str()).collect();
    assert_eq!(names, ["config.json", "home", "skills"]);
    assert!(migrated.left.is_empty());
    assert!(!dir.path().join(".intent").exists());
    assert_eq!(
      std::fs::read_to_string(dirs.data.join("home")).unwrap(),
      "/install\n"
    );
    assert!(dirs.data.join("skills/installed-skills.v3.json").is_file());
    assert!(
      std::fs::read_to_string(global_config_under(&dirs))
        .unwrap()
        .contains("\"old\"")
    );
    assert_eq!(migrate_legacy(&dirs).expect("second run"), None);
  }

  /// A v3 config already in place wins, and what Intent did not put in
  /// `~/.intent/` keeps the directory alive.
  #[test]
  fn migration_keeps_a_v3_config_and_what_it_does_not_own() {
    let (dir, dirs) = legacy_home();
    std::fs::create_dir_all(&dirs.config).expect("config dir");
    std::fs::write(
      global_config_under(&dirs),
      r#"{"intent_version": "3.0.2", "author": "new"}"#,
    )
    .expect("v3 config");
    std::fs::create_dir_all(dir.path().join(".intent/evidence")).expect("foreign entry");

    let migrated = migrate_legacy(&dirs)
      .expect("migrate")
      .expect("something moved");

    let names: Vec<&str> = migrated.moved.iter().map(|(n, _)| n.as_str()).collect();
    assert_eq!(names, ["home", "skills"]);
    assert_eq!(migrated.left, ["config.json", "evidence"]);
    assert!(
      std::fs::read_to_string(global_config_under(&dirs))
        .unwrap()
        .contains("\"new\"")
    );
  }

  /// **THE MANIFEST PATH IS THE CLASS RULING'S ONE MECHANICAL CHECK.** If this
  /// ever equals v2's file, the two tools resume overwriting each other
  /// forever while both report success -- for every kind, because the second
  /// one inherits the hazard without inheriting the check.
  #[test]
  fn no_kinds_manifest_is_v2s() {
    for (kind, v2_path) in [
      (crate::payload::Kind::Skills, "skills/installed-skills.json"),
      (
        crate::payload::Kind::Agents,
        "subagents/installed-agents.json",
      ),
    ] {
      let Ok(path) = payload_manifest(kind) else {
        return;
      };
      assert!(
        !path.ends_with(v2_path),
        "{kind:?} writes v2's manifest path, which resumes the mutual clobber"
      );
      assert!(path.starts_with(dirs().unwrap().data));
    }
  }

  /// **TWO KINDS MUST NOT SHARE A MANIFEST OR A TARGET.** Nothing structural
  /// stops `manifest_relative` returning one string for both, and if they did,
  /// installing a skill would silently evict every recorded subagent.
  #[test]
  fn the_kinds_do_not_collide_with_each_other() {
    use crate::payload::Kind;
    let (Ok(sm), Ok(am)) = (
      payload_manifest(Kind::Skills),
      payload_manifest(Kind::Agents),
    ) else {
      return;
    };
    assert_ne!(sm, am, "both kinds write the same manifest file");
    let (Ok(st), Ok(at)) = (payload_target(Kind::Skills), payload_target(Kind::Agents)) else {
      return;
    };
    assert_ne!(st, at, "both kinds install into the same directory");
  }

  /// Extensions stay unwired until they are ruled on.
  #[test]
  fn extensions_are_not_quietly_enabled() {
    assert!(ext_base().is_none());
  }
}
