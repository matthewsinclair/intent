//! The LaunchAgent: rendering the plist, enrolling, and unenrolling (`AC-08.4`).
//!
//! **D19 FIXES EVERY NAME AND LOCATION HERE, SO THIS MODULE CHOOSES NOTHING.**
//! The label is `com.matthewsinclair.intentd`, the plist lives under
//! `~/Library/LaunchAgents/`, and the logs under the XDG state directory. All
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
  #[error("login enrolment is macOS's, through launchd, and this machine runs {os}")]
  NotMacOs { os: String },
  #[error("intentd is already running (pid {pid}) and is not enrolled at login")]
  RunningUnenrolled { pid: u32 },
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
      // **NOTHING WAS WRITTEN, AND THAT IS WHAT THE REFUSAL IS FOR** (ST0081,
      // WP-05): off macOS there is no launchd to hand a job to, and the daemon
      // runs here exactly as it does unenrolled on a Mac.
      LaunchAgentError::NotMacOs { .. } => {
        "nothing was written. The daemon runs here without enrolment: `intent daemon start` without the enrolment flag starts it, and `intent daemon stop` stops it.".to_string()
      }
      // **NOTHING WAS WRITTEN AND NOTHING WAS STOPPED** (issue 0666). Loading
      // a plist beside the running daemon has `launchd` start a second one,
      // which races it for the socket, and stopping the running one is the
      // operator's call, because every session on the machine is using it.
      LaunchAgentError::RunningUnenrolled { .. } => {
        "nothing was written, and the running daemon was left answering. To enrol it, stop it with `intent daemon stop`, then run `intent daemon start --at-login`, which writes the plist and has `launchd` start the daemon.".to_string()
      }
      LaunchAgentError::UserState(e) => e.remedy(),
    }
  }
}

/// The plist `launchd` reads, as text.
///
/// **THE STAMP IS A COMMENT RATHER THAN A `<key>`, AND THAT IS A DELIBERATE
/// LIMIT ON WHAT WE PUT IN SOMEBODY ELSE'S GRAMMAR.** `launchd`'s plist
/// vocabulary is closed; an unknown key is tolerated but it is not ours to
/// invent, and a comment cannot change how the job is interpreted.
///
/// **AND IT SAYS WHAT THE FILE IS, NOT WHICH CRITERION REQUIRED IT** (D37). The
/// first version carried `AC-08.7, design.md:82` into the generated artefact,
/// which `no_pm_state_in_output` refused -- correctly: a user reading their own
/// `~/Library/LaunchAgents` learns nothing from an Intent work-package id, and
/// what they need is that edits do not survive an upgrade and which verb to use
/// instead. **The reasoning belongs here, where a maintainer reads it, and the
/// consequence belongs there, where an operator does.**
///
/// **`RunAtLoad` IS TRUE AND `KeepAlive` IS FALSE, AND THE SECOND ONE IS THE
/// DECISION.** `KeepAlive` would have launchd restart the daemon whenever it
/// exits -- including every time an operator runs `intent daemon stop`, which
/// would make the stop verb appear to do nothing. **A supervisor that fights
/// the operator's own command is worse than no supervisor**, and the daemon has
/// nothing to supervise it through: it holds no state a restart would recover,
/// because the store is on disk and every client reconnects.
///
/// **`EnvironmentVariables` CARRIES `CLAUDE_CONFIG_DIR` AND NOTHING ELSE**
/// (issue 0671). `launchd` starts the job with an environment of its own, so a
/// daemon started at login has no variable its plist does not name, and one
/// without this variable looks for Claude Code's sessions under `$HOME/.claude`.
/// On a machine that moves that directory it sees none, and the estate page
/// marks every running slot ORPHANED. With no value to carry the dict is left
/// out, and the plist is the one every earlier build wrote.
pub fn plist(
  binary: &Path,
  log: &Path,
  error_log: &Path,
  claude_config_dir: Option<&str>,
) -> String {
  let environment = claude_config_dir.map_or_else(String::new, |dir| {
    format!(
      "  <key>EnvironmentVariables</key>\n  <dict>\n    <key>{key}</key>\n    <string>{dir}</string>\n  </dict>\n",
      key = userstate::CLAUDE_CONFIG_DIR_VAR,
      dir = escape(dir),
    )
  });
  format!(
    r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>Label</key>
  <string>{label}</string>
  <!-- {stamp_key}: {version}
       Written by Intent. This file is regenerated when the version above is
       not the version of the running tool, so edits to it do not survive an
       upgrade. To change how the daemon starts, use `intent daemon start
       --at-login`; to stop it starting at login, `intent daemon stop
       --at-login`. -->
  <key>ProgramArguments</key>
  <array>
    <string>{binary}</string>
  </array>
{environment}  <key>RunAtLoad</key>
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
    stamp_key = STAMP_KEY,
    version = escape(crate::faces::INTENT_VER),
    binary = escape(&binary.display().to_string()),
    log = escape(&log.display().to_string()),
    error_log = escape(&error_log.display().to_string()),
  )
}

/// The marker naming the build that generated a plist (`AC-08.7`).
///
/// **ONE HOME, BECAUSE THE WRITER AND THE READER MUST AGREE ABOUT IT** and they
/// are in different functions -- a second spelling makes every plist read as
/// unstamped, which self-heals by rewriting a correct file forever.
pub const STAMP_KEY: &str = "INTENT_VER";

/// The version that generated the plist under `dirs`, if it says.
///
/// `None` for a plist with no marker, which is what every plist written before
/// `AC-08.7` looks like -- **and that is the case the criterion is mostly
/// about**: an old install heals without a migration precisely because an
/// absent marker is treated as stale rather than as a reason to refuse.
pub fn stamped_version(dirs: &userstate::Dirs) -> Option<String> {
  let body = std::fs::read_to_string(userstate::launch_agent_plist_under(dirs)).ok()?;
  let marker = format!("{STAMP_KEY}: ");
  let at = body.find(&marker)? + marker.len();
  let rest = &body[at..];
  let end = rest.find('\n').unwrap_or(rest.len());
  Some(rest[..end].trim().to_string())
}

/// Does the plist under `dirs` need regenerating, for a caller whose
/// `CLAUDE_CONFIG_DIR` is `claude_config_dir`?
///
/// **NOT ENROLLED IS NOT STALE.** An absent plist is an operator who never
/// enrolled, and healing it would enrol them -- turning a repair into a
/// decision they did not make. `AC-08.7` is about artefacts that EXIST being
/// brought up to date, never about creating ones that do not.
///
/// **A STAMP FROM ANOTHER BUILD IS STALE, AND SO IS A PLIST THAT DOES NOT CARRY
/// THE CALLER'S VALUE** (issue 0671). A daemon booted without the variable heals
/// an old plist into one stamped by this build and carrying nothing, which a
/// stamp alone then reads as current, so `intent daemon start --at-login` from a
/// shell that has the variable could never add it. For a caller with no value
/// a plist is never stale on content, so a daemon without one leaves a current
/// plist alone. [`write_plist`] keeps the value a plist carries when the caller
/// has none, so a rewrite never drops one.
pub fn is_stale(dirs: &userstate::Dirs, claude_config_dir: Option<&str>) -> bool {
  if !is_enrolled(dirs) {
    return false;
  }
  stamped_version(dirs).as_deref() != Some(crate::faces::INTENT_VER)
    || adds(dirs, claude_config_dir).is_some()
}

/// The `CLAUDE_CONFIG_DIR` the plist under `dirs` carries, if it carries one.
///
/// Read back as [`plist`] writes it, the first `<string>` after the key, with
/// the XML escape undone, so a value with `&` in it reads back as it was given.
pub fn carried_claude_config_dir(dirs: &userstate::Dirs) -> Option<String> {
  let body = std::fs::read_to_string(userstate::launch_agent_plist_under(dirs)).ok()?;
  carried_in(&body)
}

/// [`carried_claude_config_dir`] over a plist's text. `None` unless the key's
/// own value follows it, with nothing but whitespace between them.
fn carried_in(body: &str) -> Option<String> {
  let key = format!("<key>{}</key>", userstate::CLAUDE_CONFIG_DIR_VAR);
  let after = &body[body.find(&key)? + key.len()..];
  let rest = after.trim_start().strip_prefix("<string>")?;
  let value = unescape(&rest[..rest.find("</string>")?]);
  (!value.is_empty()).then_some(value)
}

/// The value a rewrite for this caller would add to the plist under `dirs`:
/// the caller's own, when it is set and the plist does not already carry it.
/// `None` when the caller has none, or the plist carries the same.
fn adds(dirs: &userstate::Dirs, claude_config_dir: Option<&str>) -> Option<String> {
  let dir = claude_config_dir.filter(|dir| !dir.is_empty())?;
  (carried_claude_config_dir(dirs).as_deref() != Some(dir)).then(|| dir.to_string())
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

/// [`escape`] undone, `&amp;` last, so a value that held the text `&lt;` reads
/// back as that text rather than as `<`.
fn unescape(escaped: &str) -> String {
  escaped
    .replace("&lt;", "<")
    .replace("&gt;", ">")
    .replace("&amp;", "&")
}

/// Whether this machine can enrol the daemon at login. Only macOS can: enrolment
/// hands the job to `launchd`, and `os` is the operating system the caller
/// names, `std::env::consts::OS` in the CLI.
///
/// **ASKED BEFORE ANYTHING IS WRITTEN, WHICH IS WHY IT EXISTS** (ST0081,
/// WP-05). `intent daemon start --at-login` wrote the plist first and failed at
/// `launchctl` second, so a Linux machine was left a `~/Library/LaunchAgents/`
/// file nothing would ever read. PURE in `os`, so the refusal is driven on any
/// host rather than only on the platform that refuses.
pub fn enrolment_supported(os: &str) -> Result<(), LaunchAgentError> {
  if os == "macos" {
    return Ok(());
  }
  Err(LaunchAgentError::NotMacOs { os: os.to_string() })
}

/// Write the plist under `dirs`, creating `~/Library/LaunchAgents/` if absent.
///
/// Returns the path written. Does NOT tell `launchd` about it -- see [`load`].
///
/// **THE PLIST CARRIES THE CALLER'S `CLAUDE_CONFIG_DIR`, OR ELSE THE ONE IT
/// ALREADY CARRIES** (issue 0671). The caller's wins when both are set. The
/// daemon's boot heal is a caller too, and one that may have been started
/// without the variable, so a rewrite with no value keeps the old file's value
/// rather than dropping it.
pub fn write_plist(
  dirs: &userstate::Dirs,
  binary: &Path,
  claude_config_dir: Option<&str>,
) -> Result<PathBuf, LaunchAgentError> {
  let carries = claude_config_dir
    .filter(|dir| !dir.is_empty())
    .map(str::to_string)
    .or_else(|| carried_claude_config_dir(dirs));
  let path = userstate::launch_agent_plist_under(dirs);
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
  let log = userstate::daemon_log_under(dirs);
  if let Some(parent) = log.parent() {
    std::fs::create_dir_all(parent).map_err(|cause| LaunchAgentError::Unwritable {
      path: parent.to_path_buf(),
      cause,
    })?;
  }
  let body = plist(
    binary,
    &log,
    &userstate::daemon_error_log_under(dirs),
    carries.as_deref(),
  );
  std::fs::write(&path, body).map_err(|cause| LaunchAgentError::Unwritable {
    path: path.clone(),
    cause,
  })?;
  Ok(path)
}

/// Is a plist present under `dirs`?
///
/// **PRESENCE OF THE FILE, NEVER `launchctl list`.** Enrolment is a property of
/// the machine that survives logout, and `launchctl list` answers about the
/// CURRENT session -- so it reports "not enrolled" for a correctly enrolled
/// daemon whenever it is asked from outside a GUI login session, which is
/// exactly where a test or a remote shell asks from.
pub fn is_enrolled(dirs: &userstate::Dirs) -> bool {
  userstate::launch_agent_plist_under(dirs).is_file()
}

/// What the plist under `dirs` says about enrolment at login.
///
/// **THE THREE STATES `intent daemon start --at-login` ANSWERS DIFFERENTLY
/// BESIDE A RUNNING DAEMON** (issue 0666). Read through [`is_enrolled`],
/// [`is_stale`] and [`stamped_version`] rather than by a second reading of the
/// file, so this and those predicates cannot disagree about one plist.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Enrolment {
  /// No plist: never enrolled, or unenrolled since.
  Absent,
  /// A plist a rewrite would change: `stamped` is the version its stamp names,
  /// `None` for one written before the stamp existed, and `adds` the
  /// `CLAUDE_CONFIG_DIR` a rewrite would bring that it does not carry, `None`
  /// when it already carries the caller's or the caller has none.
  Stale {
    stamped: Option<String>,
    adds: Option<String>,
  },
  /// A plist this build wrote, carrying the caller's value when it has one.
  Current,
}

/// Read [`Enrolment`] from the plist under `dirs`, for a caller whose
/// `CLAUDE_CONFIG_DIR` is `claude_config_dir` (see [`is_stale`]).
pub fn enrolment(dirs: &userstate::Dirs, claude_config_dir: Option<&str>) -> Enrolment {
  if !is_enrolled(dirs) {
    Enrolment::Absent
  } else if is_stale(dirs, claude_config_dir) {
    Enrolment::Stale {
      stamped: stamped_version(dirs),
      adds: adds(dirs, claude_config_dir),
    }
  } else {
    Enrolment::Current
  }
}

/// What `intent daemon start --at-login` owes beside a daemon already running.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Beside {
  /// This build's plist is in place, so the enrolment holds and nothing is
  /// written.
  Holds,
  /// An enrolment a rewrite would change: rewrite the plist with
  /// [`write_plist`] and load nothing. `was` is the version it replaces, `None`
  /// for a plist with no stamp, and `adds` the `CLAUDE_CONFIG_DIR` the rewrite
  /// brings, as [`Enrolment::Stale`] names them.
  Rewrite {
    was: Option<String>,
    adds: Option<String>,
  },
}

/// The answer `intent daemon start --at-login` gives beside the daemon already
/// running as `pid` (issue 0666).
///
/// **NO ANSWER STOPS, RESTARTS OR LOADS ANYTHING.** The plist carries
/// `RunAtLoad`, so loading it beside the running daemon would have `launchd`
/// start a second one that races it for the socket, and the running daemon is
/// the one every session on the machine is using. So with no plist the answer
/// is a refusal, never an `ok:` for an enrolment nothing made.
///
/// **A STALE PLIST IS REWRITTEN RATHER THAN REFUSED**, because repairing an
/// enrolment that exists is not making a new one (see [`is_stale`]), and
/// rewriting without loading is what the daemon's own boot does: `launchd`
/// reads the file at the next login, and writing it starts nothing.
///
/// PURE in what it is handed, so every answer is driven on any host.
pub fn beside_a_running_daemon(pid: u32, enrolment: Enrolment) -> Result<Beside, LaunchAgentError> {
  match enrolment {
    Enrolment::Absent => Err(LaunchAgentError::RunningUnenrolled { pid }),
    Enrolment::Stale { stamped, adds } => Ok(Beside::Rewrite { was: stamped, adds }),
    Enrolment::Current => Ok(Beside::Holds),
  }
}

/// What a rewrite of a stale plist changed, in the words `intent daemon start
/// --at-login` and the daemon's boot heal both print after "rewritten" and
/// "regenerated" (issue 0671).
///
/// A stamp from another build reads `(was <stamped>, now <this build>)`, and
/// `unstamped` stands in for no stamp. A value the rewrite adds reads `and now
/// carries CLAUDE_CONFIG_DIR <value>` after that, or `to carry
/// CLAUDE_CONFIG_DIR <value>` alone when the stamp was already this build's,
/// where `(was 3.2.9, now 3.2.9)` would say nothing. PURE, so every wording is
/// driven on any host.
pub fn rewritten(stamped: Option<&str>, adds: Option<&str>) -> String {
  let now = crate::faces::INTENT_VER;
  let restamped =
    (stamped != Some(now)).then(|| format!("(was {}, now {now})", stamped.unwrap_or("unstamped")));
  let carried = adds.map(|dir| format!("{} {dir}", userstate::CLAUDE_CONFIG_DIR_VAR));
  match (restamped, carried) {
    (Some(restamped), Some(carried)) => format!("{restamped} and now carries {carried}"),
    (Some(restamped), None) => restamped,
    (None, Some(carried)) => format!("to carry {carried}"),
    (None, None) => format!("(was {now}, now {now})"),
  }
}

/// Remove the plist under `dirs`. `true` when there was one to remove.
pub fn remove_plist(dirs: &userstate::Dirs) -> Result<bool, LaunchAgentError> {
  let path = userstate::launch_agent_plist_under(dirs);
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
  fn enrolment_is_supported_on_macos() {
    assert!(enrolment_supported("macos").is_ok());
  }

  #[test]
  fn enrolment_is_refused_off_macos_naming_the_unenrolled_start() {
    let refused = enrolment_supported("linux").expect_err("linux has no launchd");
    let said = refused.render();

    assert!(said.contains("macOS"), "names no macOS: {said}");
    assert!(said.contains("linux"), "names no platform: {said}");
    assert!(
      said.contains("`intent daemon start` without the enrolment flag"),
      "names no unenrolled start: {said}"
    );
  }

  #[test]
  fn beside_a_running_daemon_this_builds_plist_holds() {
    assert_eq!(
      beside_a_running_daemon(4242, Enrolment::Current).expect("the enrolment holds"),
      Beside::Holds
    );
  }

  #[test]
  fn beside_a_running_daemon_another_builds_plist_is_rewritten_naming_its_version() {
    let stale = Enrolment::Stale {
      stamped: Some("3.2.7".to_string()),
      adds: None,
    };
    assert_eq!(
      beside_a_running_daemon(4242, stale).expect("a stale plist is rewritten"),
      Beside::Rewrite {
        was: Some("3.2.7".to_string()),
        adds: None,
      }
    );
  }

  #[test]
  fn beside_a_running_daemon_no_plist_is_refused_naming_the_pid_and_stop_then_enrol() {
    let refused =
      beside_a_running_daemon(4242, Enrolment::Absent).expect_err("nothing enrolled this machine");
    let said = refused.render();

    assert!(said.contains("pid 4242"), "names no running pid: {said}");
    assert!(
      said.contains("not enrolled at login"),
      "names no missing enrolment: {said}"
    );
    assert!(
      said.contains("nothing was written"),
      "does not say nothing was written: {said}"
    );
    assert!(
      said.contains("`intent daemon stop`") && said.contains("`intent daemon start --at-login`"),
      "names no stop-then-enrol remedy: {said}"
    );
  }

  #[test]
  fn the_enrolment_is_read_as_absent_stale_or_current_from_the_plist() {
    let home = root();
    let dirs = userstate::Dirs::at_home(home.path());
    assert_eq!(enrolment(&dirs, None), Enrolment::Absent);

    let plist = userstate::launch_agent_plist_under(&dirs);
    std::fs::create_dir_all(plist.parent().expect("a parent")).expect("create LaunchAgents");
    std::fs::write(&plist, "<plist><dict><key>Label</key></dict></plist>\n")
      .expect("plant a pre-stamp plist");
    assert_eq!(
      enrolment(&dirs, None),
      Enrolment::Stale {
        stamped: None,
        adds: None
      }
    );

    std::fs::write(
      &plist,
      format!("<plist>\n<!-- {STAMP_KEY}: 0.0.1\n-->\n</plist>\n"),
    )
    .expect("plant an older build's plist");
    assert_eq!(
      enrolment(&dirs, None),
      Enrolment::Stale {
        stamped: Some("0.0.1".to_string()),
        adds: None
      }
    );

    write_plist(&dirs, Path::new("/opt/intent/bin/intentd"), None)
      .expect("write this build's plist");
    assert_eq!(enrolment(&dirs, None), Enrolment::Current);
  }

  #[test]
  fn the_plist_names_the_label_the_binary_and_both_logs() {
    let home = root();
    let binary = std::path::Path::new("/opt/intent/bin/intentd");
    let written =
      write_plist(&userstate::Dirs::at_home(home.path()), binary, None).expect("write the plist");
    let body = std::fs::read_to_string(&written).expect("read it back");

    assert!(body.contains(LAUNCH_AGENT_LABEL), "no label: {body}");
    assert!(
      body.contains("/opt/intent/bin/intentd"),
      "no binary: {body}"
    );
    assert!(
      body.contains(
        &userstate::daemon_log_under(&userstate::Dirs::at_home(home.path()))
          .display()
          .to_string()
      ),
      "no stdout log: {body}"
    );
    assert!(
      body.contains(
        &userstate::daemon_error_log_under(&userstate::Dirs::at_home(home.path()))
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

    let written = write_plist(
      &userstate::Dirs::at_home(&awkward),
      std::path::Path::new("/bin/intentd"),
      None,
    )
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
    write_plist(
      &userstate::Dirs::at_home(home.path()),
      std::path::Path::new("/bin/intentd"),
      None,
    )
    .expect("write");

    assert!(
      userstate::launch_agent_plist_under(&userstate::Dirs::at_home(home.path())).is_file(),
      "no plist"
    );
    // launchd opens StandardOutPath itself, BEFORE the daemon runs, so a
    // missing directory fails the job where no Intent code can report it.
    assert!(
      userstate::daemon_log_under(&userstate::Dirs::at_home(home.path()))
        .parent()
        .expect("a parent")
        .is_dir(),
      "the log directory was not created, so launchd would fail the job at spawn"
    );
  }

  #[test]
  fn a_freshly_written_plist_carries_this_builds_stamp() {
    let home = root();
    write_plist(
      &userstate::Dirs::at_home(home.path()),
      std::path::Path::new("/bin/intentd"),
      None,
    )
    .expect("write");
    assert_eq!(
      stamped_version(&userstate::Dirs::at_home(home.path())).as_deref(),
      Some(crate::faces::INTENT_VER),
      "a plist this build just wrote does not name this build"
    );
    assert!(
      !is_stale(&userstate::Dirs::at_home(home.path()), None),
      "a plist this build just wrote reports as stale, so every boot would rewrite it forever"
    );
  }

  /// **THE CASE `AC-08.7` IS MOSTLY ABOUT: AN OLD INSTALL.**
  ///
  /// Every plist written before the stamp existed has no marker at all.
  /// Treating an absent marker as stale is what lets those heal **without a
  /// migration** -- the alternative is a one-off upgrade step that an operator
  /// who never runs it never gets.
  #[test]
  fn a_plist_with_no_marker_at_all_is_stale() {
    let home = root();
    let path = userstate::launch_agent_plist_under(&userstate::Dirs::at_home(home.path()));
    std::fs::create_dir_all(path.parent().expect("a parent")).expect("dirs");
    std::fs::write(&path, "<plist><dict><key>Label</key></dict></plist>\n")
      .expect("write a pre-stamp plist");

    assert_eq!(
      stamped_version(&userstate::Dirs::at_home(home.path())),
      None,
      "an unstamped plist reported a version"
    );
    assert!(
      is_stale(&userstate::Dirs::at_home(home.path()), None),
      "a plist from before the stamp existed is not being healed, which is the whole population the criterion names"
    );
  }

  #[test]
  fn a_plist_from_another_build_is_stale() {
    let home = root();
    write_plist(
      &userstate::Dirs::at_home(home.path()),
      std::path::Path::new("/bin/intentd"),
      None,
    )
    .expect("write");
    let path = userstate::launch_agent_plist_under(&userstate::Dirs::at_home(home.path()));
    let body = std::fs::read_to_string(&path).expect("read");
    let aged = body.replace(
      &format!("{STAMP_KEY}: {}", crate::faces::INTENT_VER),
      &format!("{STAMP_KEY}: 0.0.1-from-another-build"),
    );
    assert_ne!(aged, body, "the fixture did not actually age the stamp");
    std::fs::write(&path, aged).expect("write the aged plist");

    assert_eq!(
      stamped_version(&userstate::Dirs::at_home(home.path())).as_deref(),
      Some("0.0.1-from-another-build")
    );
    assert!(
      is_stale(&userstate::Dirs::at_home(home.path()), None),
      "a plist naming another build is not stale"
    );
  }

  /// **NOT ENROLLED IS NOT STALE, AND THIS IS THE ARM THAT BOUNDS THE FEATURE.**
  ///
  /// Self-healing that treated absence as staleness would ENROL an operator who
  /// never asked to be -- turning a repair into a decision they did not make,
  /// silently, on every boot. `AC-08.7` is about artefacts that exist being
  /// brought up to date; **an artefact that does not exist is not out of date,
  /// it is absent, and those are different states.**
  #[test]
  fn a_machine_that_never_enrolled_is_not_stale() {
    let home = root();
    assert!(
      !is_enrolled(&userstate::Dirs::at_home(home.path())),
      "the fixture is already enrolled"
    );
    assert!(
      !is_stale(&userstate::Dirs::at_home(home.path()), None),
      "a machine that never enrolled reports as stale, so booting would enrol it without being asked"
    );
  }

  #[test]
  fn regenerating_a_stale_plist_brings_the_stamp_current() {
    let home = root();
    let path = userstate::launch_agent_plist_under(&userstate::Dirs::at_home(home.path()));
    std::fs::create_dir_all(path.parent().expect("a parent")).expect("dirs");
    std::fs::write(&path, "<plist><dict></dict></plist>\n").expect("write an old plist");
    assert!(
      is_stale(&userstate::Dirs::at_home(home.path()), None),
      "the fixture is not stale to begin with"
    );

    write_plist(
      &userstate::Dirs::at_home(home.path()),
      std::path::Path::new("/bin/intentd"),
      None,
    )
    .expect("regenerate");
    assert!(
      !is_stale(&userstate::Dirs::at_home(home.path()), None),
      "regenerating left the plist stale, so the healing does not converge"
    );
  }

  #[test]
  fn enrolment_is_the_plists_presence_and_removing_it_is_reversible() {
    let home = root();
    assert!(
      !is_enrolled(&userstate::Dirs::at_home(home.path())),
      "enrolled before anything was written"
    );

    write_plist(
      &userstate::Dirs::at_home(home.path()),
      std::path::Path::new("/bin/intentd"),
      None,
    )
    .expect("write");
    assert!(
      is_enrolled(&userstate::Dirs::at_home(home.path())),
      "not enrolled after writing the plist"
    );

    assert!(
      remove_plist(&userstate::Dirs::at_home(home.path())).expect("remove"),
      "removal reported nothing to remove"
    );
    assert!(
      !is_enrolled(&userstate::Dirs::at_home(home.path())),
      "still enrolled after removal"
    );

    // **REMOVING AGAIN IS `false`, NOT AN ERROR.** Unenrolling something that
    // is not enrolled is the operator getting what they asked for, and a
    // failure here would make `stop` noisy on every machine that never
    // enrolled.
    assert!(
      !remove_plist(&userstate::Dirs::at_home(home.path())).expect("second remove"),
      "removing an absent plist reported that it removed one"
    );
  }

  /// Issue 0671: **the value goes in escaped and reads back as it was given**,
  /// and a plist with no value to carry names no environment at all.
  #[test]
  fn a_claude_config_dir_is_carried_escaped_and_reads_back_as_given() {
    let home = root();
    let dirs = userstate::Dirs::at_home(home.path());
    let binary = Path::new("/bin/intentd");
    let plist = userstate::launch_agent_plist_under(&dirs);

    write_plist(&dirs, binary, None).expect("write with no value");
    let bare = std::fs::read_to_string(&plist).expect("read the plist");
    assert!(
      !bare.contains("EnvironmentVariables"),
      "a plist with no value to carry names an environment: {bare}"
    );
    assert_eq!(carried_claude_config_dir(&dirs), None);

    let awkward = "/Users/a&b/R&D <lab>/.config/claude";
    write_plist(&dirs, binary, Some(awkward)).expect("write with a value");
    let body = std::fs::read_to_string(&plist).expect("read the plist again");
    assert!(
      body.contains("<key>CLAUDE_CONFIG_DIR</key>")
        && body.contains("<string>/Users/a&amp;b/R&amp;D &lt;lab&gt;/.config/claude</string>"),
      "the value is not carried, escaped, under its key: {body}"
    );
    assert_eq!(carried_claude_config_dir(&dirs).as_deref(), Some(awkward));
  }

  /// Issue 0671: **a plist that lacks the caller's value is stale, a caller with
  /// none never finds one stale on content, and a rewrite with none keeps the
  /// value the plist carries**, which is the daemon's boot heal without the
  /// variable.
  #[test]
  fn a_plist_lacking_the_callers_value_is_stale_and_a_rewrite_without_one_keeps_it() {
    let home = root();
    let dirs = userstate::Dirs::at_home(home.path());
    let binary = Path::new("/bin/intentd");
    let dir = "/Users/test/.config/claude";

    write_plist(&dirs, binary, None).expect("write this build's plist with no value");
    assert!(
      !is_stale(&dirs, None),
      "a caller with no value found this build's plist stale"
    );
    assert_eq!(
      enrolment(&dirs, Some(dir)),
      Enrolment::Stale {
        stamped: Some(crate::faces::INTENT_VER.to_string()),
        adds: Some(dir.to_string()),
      },
      "a plist lacking the caller's value is not stale for that caller"
    );

    write_plist(&dirs, binary, Some(dir)).expect("write the caller's value");
    assert_eq!(enrolment(&dirs, Some(dir)), Enrolment::Current);

    write_plist(&dirs, binary, None).expect("rewrite with no value");
    assert_eq!(
      carried_claude_config_dir(&dirs).as_deref(),
      Some(dir),
      "a rewrite with no value dropped the one the plist carried"
    );
    assert!(
      is_stale(&dirs, Some("/Users/test/elsewhere")),
      "a plist carrying another value is not stale for a caller with a different one"
    );
  }

  /// Issue 0671: **a rewrite names what it changed**: another build's stamp as
  /// before, a value it adds after that, and the value alone when the stamp was
  /// already this build's. The other build's stamp is derived from this build's
  /// version, so the two can never be equal whatever version is cut.
  #[test]
  fn a_rewrite_names_the_stamp_it_replaced_and_the_value_it_added() {
    let now = crate::faces::INTENT_VER;
    let other = format!("{now}-another-build");
    let dir = "/Users/test/.config/claude";
    assert_eq!(rewritten(None, None), format!("(was unstamped, now {now})"));
    assert_eq!(
      rewritten(Some(other.as_str()), Some(dir)),
      format!("(was {other}, now {now}) and now carries CLAUDE_CONFIG_DIR {dir}")
    );
    assert_eq!(
      rewritten(Some(now), Some(dir)),
      format!("to carry CLAUDE_CONFIG_DIR {dir}")
    );
  }
}
