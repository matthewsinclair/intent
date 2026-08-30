//! The LaunchAgent: rendering the plist, enrolling, and unenrolling (`AC-08.4`).
//!
//! **D19 FIXES EVERY NAME AND LOCATION HERE, SO THIS MODULE CHOOSES NOTHING.**
//! The label is `com.matthewsinclair.intentd`, the plist lives under
//! `~/Library/LaunchAgents/`, and the logs under `~/.local/share/intent/`. All
//! three are read from [`crate::userstate`] rather than spelled again, because
//! the plist's `Label`, the plist's FILENAME and every `launchctl` argument are
//! the same string -- and a second spelling makes `launchctl` operate on a job
//! that does not exist, reported in a way that reads like the daemon being
//! absent.
//!
//! **THE FILESYSTEM HALF AND THE `launchctl` HALF ARE SEPARATE ON PURPOSE.**
//! [`write_plist`] is a pure mapping from a root and a binary path to bytes on
//! disk, so the rendering can be driven against a temporary directory on any
//! machine. [`load`] and [`unload`] shell out. A single `enrol` that did both
//! would make the plist's CONTENT -- the part with an escaping bug in it --
//! testable only on a macOS box willing to have a real job registered.
//!
//! **THIS IS NOT `daemon.rs`, AND THE SPLIT IS A REAL DISTINCTION RATHER THAN
//! TIDINESS.** That module answers *is a daemon reachable, and where*; this one
//! answers *is the daemon enrolled to start at login*. **A daemon can be either
//! without the other**: enrolled and not running (just logged in, launchd has
//! not started it yet), or running and not enrolled (`intent daemon start`
//! without the flag). Merging them would produce one predicate that is wrong in
//! both of those states.

use std::path::{Path, PathBuf};

use crate::remedy::Remedy;
use crate::userstate::{self, LAUNCH_AGENT_LABEL};

#[derive(Debug, thiserror::Error)]
pub enum LaunchAgentError {
  #[error("`{path}` could not be written: {cause}")]
  Unwritable {
    path: PathBuf,
    cause: std::io::Error,
  },
  #[error("`{path}` could not be removed: {cause}")]
  Unremovable {
    path: PathBuf,
    cause: std::io::Error,
  },
  #[error("`launchctl` could not be run: {0}")]
  NoLaunchctl(std::io::Error),
  #[error("`launchctl {verb}` refused: {stderr}")]
  Refused { verb: String, stderr: String },
  #[error(transparent)]
  UserState(#[from] userstate::UserStateError),
}

impl Remedy for LaunchAgentError {
  fn remedy(&self) -> String {
    match self {
      LaunchAgentError::Unwritable { path, .. } => format!(
        "enrolling the daemon writes one file, `{}`. Check that its directory exists and is writable -- on macOS `~/Library/LaunchAgents/` is created on demand and may be absent on a fresh account.",
        path.display()
      ),
      LaunchAgentError::Unremovable { path, .. } => format!(
        "the job was unloaded but `{}` is still there, so the daemon will be enrolled again at the next login. Remove that file to finish unenrolling.",
        path.display()
      ),
      // **NOT "install launchctl".** It ships with macOS and cannot be
      // installed; the honest reading of its absence is that this is not a
      // macOS machine, and the daemon runs perfectly well unenrolled.
      LaunchAgentError::NoLaunchctl(_) => {
        "`launchctl` is part of macOS, so this is either not a macOS machine or the environment has no usable PATH. The daemon does not need to be enrolled to run: `intent daemon start` without the enrolment flag starts one for this session.".to_string()
      }
      LaunchAgentError::Refused { .. } => format!(
        "`launchctl` rejected the job. Check whether one is already loaded with `launchctl list | grep {LAUNCH_AGENT_LABEL}`, and unload it before enrolling again."
      ),
      LaunchAgentError::UserState(e) => e.remedy(),
    }
  }
}

/// The plist `launchd` reads, as text.
///
/// **`RunAtLoad` IS TRUE AND `KeepAlive` IS FALSE, AND THE SECOND ONE IS THE
/// DECISION.** `KeepAlive` would have launchd restart the daemon whenever it
/// exits -- including every time an operator runs `intent daemon stop`, which
/// would make the stop verb appear to do nothing. **A supervisor that fights
/// the operator's own command is worse than no supervisor**, and the daemon has
/// nothing to supervise it through: it holds no state a restart would recover,
/// because the store is on disk and every client reconnects.
pub fn plist(binary: &Path, log: &Path, error_log: &Path) -> String {
  format!(
    r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>Label</key>
  <string>{label}</string>
  <key>ProgramArguments</key>
  <array>
    <string>{binary}</string>
  </array>
  <key>RunAtLoad</key>
  <true/>
  <key>KeepAlive</key>
  <false/>
  <key>StandardOutPath</key>
  <string>{log}</string>
  <key>StandardErrorPath</key>
  <string>{error_log}</string>
</dict>
</plist>
"#,
    label = escape(LAUNCH_AGENT_LABEL),
    binary = escape(&binary.display().to_string()),
    log = escape(&log.display().to_string()),
    error_log = escape(&error_log.display().to_string()),
  )
}

/// XML-escape a value going into the plist.
///
/// **A PATH IS ATTACKER-FREE AND STILL NOT XML-SAFE, WHICH IS WHY THIS IS NOT
/// PARANOIA.** `&` is legal in a macOS directory name and appears in real ones
/// -- a home under `/Users/a&b`, a checkout in a folder someone named `R&D`.
/// Interpolated raw it produces a plist that is not well-formed XML, and
/// `launchd` rejects the whole job with a parse error that says nothing about
/// the ampersand. **The failure is total, silent about its cause, and only ever
/// reproduces on the machine whose path contains one.**
fn escape(raw: &str) -> String {
  raw
    .replace('&', "&amp;")
    .replace('<', "&lt;")
    .replace('>', "&gt;")
}

/// Write the plist under `root`, creating `~/Library/LaunchAgents/` if absent.
///
/// Returns the path written. Does NOT tell `launchd` about it -- see [`load`].
pub fn write_plist(root: &Path, binary: &Path) -> Result<PathBuf, LaunchAgentError> {
  let path = userstate::launch_agent_plist_under(root);
  if let Some(parent) = path.parent() {
    std::fs::create_dir_all(parent).map_err(|cause| LaunchAgentError::Unwritable {
      path: parent.to_path_buf(),
      cause,
    })?;
  }
  // The log directory is created here rather than by the daemon, because
  // `launchd` opens `StandardOutPath` ITSELF before the process starts: a
  // missing directory fails the job at spawn, where the daemon's own code has
  // not run and cannot report anything.
  let log = userstate::daemon_log_under(root);
  if let Some(parent) = log.parent() {
    std::fs::create_dir_all(parent).map_err(|cause| LaunchAgentError::Unwritable {
      path: parent.to_path_buf(),
      cause,
    })?;
  }
  let body = plist(binary, &log, &userstate::daemon_error_log_under(root));
  std::fs::write(&path, body).map_err(|cause| LaunchAgentError::Unwritable {
    path: path.clone(),
    cause,
  })?;
  Ok(path)
}

/// Is a plist present under `root`?
///
/// **PRESENCE OF THE FILE, NEVER `launchctl list`.** Enrolment is a property of
/// the machine that survives logout, and `launchctl list` answers about the
/// CURRENT session -- so it reports "not enrolled" for a correctly enrolled
/// daemon whenever it is asked from outside a GUI login session, which is
/// exactly where a test or a remote shell asks from.
pub fn is_enrolled(root: &Path) -> bool {
  userstate::launch_agent_plist_under(root).is_file()
}

/// Remove the plist under `root`. `true` when there was one to remove.
pub fn remove_plist(root: &Path) -> Result<bool, LaunchAgentError> {
  let path = userstate::launch_agent_plist_under(root);
  if !path.exists() {
    return Ok(false);
  }
  std::fs::remove_file(&path)
    .map(|()| true)
    .map_err(|cause| LaunchAgentError::Unremovable { path, cause })
}

/// Tell `launchd` about the plist at `path`.
pub fn load(path: &Path) -> Result<(), LaunchAgentError> {
  launchctl("load", path)
}

/// Tell `launchd` to forget the job described by the plist at `path`.
pub fn unload(path: &Path) -> Result<(), LaunchAgentError> {
  launchctl("unload", path)
}

/// **`load -w` / `unload -w` RATHER THAN `bootstrap` / `bootout`, AND IT IS A
/// COMPATIBILITY CALL RATHER THAN A PREFERENCE.** The modern spelling needs a
/// domain target -- `gui/$UID` -- which means reading the uid and being wrong
/// in exactly the environments where enrolment is unusual: a remote shell, a CI
/// runner, `sudo`. `load` infers the domain from the calling session, which is
/// the behaviour an operator running this from their own terminal wants. `-w`
/// clears the `Disabled` key so a previously-unloaded job enrols again rather
/// than silently staying off.
fn launchctl(verb: &str, path: &Path) -> Result<(), LaunchAgentError> {
  let out = std::process::Command::new("launchctl")
    .arg(verb)
    .arg("-w")
    .arg(path)
    .output()
    .map_err(LaunchAgentError::NoLaunchctl)?;
  if out.status.success() {
    return Ok(());
  }
  Err(LaunchAgentError::Refused {
    verb: verb.to_string(),
    stderr: String::from_utf8_lossy(&out.stderr).trim().to_string(),
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  fn root() -> tempfile::TempDir {
    tempfile::tempdir().expect("tempdir")
  }

  #[test]
  fn the_plist_names_the_label_the_binary_and_both_logs() {
    let home = root();
    let binary = std::path::Path::new("/opt/intent/bin/intentd");
    let written = write_plist(home.path(), binary).expect("write the plist");
    let body = std::fs::read_to_string(&written).expect("read it back");

    assert!(body.contains(LAUNCH_AGENT_LABEL), "no label: {body}");
    assert!(
      body.contains("/opt/intent/bin/intentd"),
      "no binary: {body}"
    );
    assert!(
      body.contains(
        &userstate::daemon_log_under(home.path())
          .display()
          .to_string()
      ),
      "no stdout log: {body}"
    );
    assert!(
      body.contains(
        &userstate::daemon_error_log_under(home.path())
          .display()
          .to_string()
      ),
      "no stderr log: {body}"
    );
  }

  /// **THE ONE ARM THAT IS ABOUT A DEFECT RATHER THAN A SHAPE.**
  ///
  /// A home directory containing `&` is legal on macOS and real -- `R&D`, a
  /// user called `a&b`. Interpolated raw it makes the plist malformed XML, and
  /// `launchd` rejects the entire job with a parse error naming nothing. **The
  /// failure is total, silent about its cause, and reproduces only on the
  /// machine whose path contains one**, which is the worst possible
  /// distribution for a bug in an install path.
  #[test]
  fn a_path_containing_an_ampersand_stays_well_formed() {
    let home = root();
    let awkward = home.path().join("R&D <lab>");
    std::fs::create_dir_all(&awkward).expect("create the awkward root");

    let written = write_plist(&awkward, std::path::Path::new("/bin/intentd"))
      .expect("write under an awkward root");
    let body = std::fs::read_to_string(&written).expect("read it back");

    assert!(
      body.contains("R&amp;D"),
      "the ampersand was not escaped, so this plist is not well-formed XML: {body}"
    );
    assert!(
      body.contains("&lt;lab&gt;"),
      "the angle brackets were not escaped: {body}"
    );

    // **THE CHECK THAT WOULD CATCH A HALF-DONE ESCAPE.** Asserting the escaped
    // form is present says nothing about whether a RAW one is also there --
    // one unescaped occurrence is enough to make launchd reject the job, and
    // the positive assertion above would still pass. Every `&` must open an
    // entity.
    for (i, _) in body.match_indices('&') {
      let tail = &body[i..];
      assert!(
        tail.starts_with("&amp;") || tail.starts_with("&lt;") || tail.starts_with("&gt;"),
        "a raw `&` survived at byte {i}, so the plist is malformed: {}",
        &tail[..tail.len().min(40)]
      );
    }
  }

  #[test]
  fn writing_creates_the_directories_launchd_opens_before_the_process_starts() {
    let home = root();
    write_plist(home.path(), std::path::Path::new("/bin/intentd")).expect("write");

    assert!(
      userstate::launch_agent_plist_under(home.path()).is_file(),
      "no plist"
    );
    // launchd opens StandardOutPath itself, BEFORE the daemon runs, so a
    // missing directory fails the job where no Intent code can report it.
    assert!(
      userstate::daemon_log_under(home.path())
        .parent()
        .expect("a parent")
        .is_dir(),
      "the log directory was not created, so launchd would fail the job at spawn"
    );
  }

  #[test]
  fn enrolment_is_the_plists_presence_and_removing_it_is_reversible() {
    let home = root();
    assert!(
      !is_enrolled(home.path()),
      "enrolled before anything was written"
    );

    write_plist(home.path(), std::path::Path::new("/bin/intentd")).expect("write");
    assert!(
      is_enrolled(home.path()),
      "not enrolled after writing the plist"
    );

    assert!(
      remove_plist(home.path()).expect("remove"),
      "removal reported nothing to remove"
    );
    assert!(!is_enrolled(home.path()), "still enrolled after removal");

    // **REMOVING AGAIN IS `false`, NOT AN ERROR.** Unenrolling something that
    // is not enrolled is the operator getting what they asked for, and a
    // failure here would make `stop` noisy on every machine that never
    // enrolled.
    assert!(
      !remove_plist(home.path()).expect("second remove"),
      "removing an absent plist reported that it removed one"
    );
  }
}
