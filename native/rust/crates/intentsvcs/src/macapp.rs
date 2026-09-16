//! The macOS menubar app's lifecycle: where it is, whether it runs, start and
//! stop.
//!
//! **THE SPLIT FROM `daemon.rs` IS THE SAME ONE `launchagent.rs` RECORDS.**
//! `daemon.rs` answers whether a daemon is reachable and where; this answers
//! whether a GUI application is running and where its bundle is. A machine can
//! have either without the other -- the app is a client of `daemon status` and
//! runs perfectly well with no daemon at all -- so folding them would put two
//! questions with different answers behind one predicate.
//!
//! **PORTED FROM `geodica app` (../Gtools) RATHER THAN INVENTED**, on hv's
//! standing directive. Three things came across because they are the parts that
//! were paid for: `lsappinfo` over `pgrep`, a graceful quit that is then
//! VERIFIED, and a three-state status whose exit code carries the state.

use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

/// Fixed at `project.yml:50` (`PRODUCT_BUNDLE_IDENTIFIER`). LaunchServices keys
/// on this, so it is the address for every question below.
pub const BUNDLE_ID: &str = "com.matthewsinclair.intent.macos";

/// How often [`start`] and [`stop`] look for the app while it settles.
///
/// **BOTH DIRECTIONS ARE REQUESTS, NOT ACTS.** `osascript` returns as soon as
/// the quit is delivered and `open` returns as soon as LaunchServices accepts the
/// launch; in neither case has the app finished doing what was asked. Reporting
/// on delivery is IN-AG-NO-SILENT-001 at its most literal, and it was a live
/// defect in the launch half until the dual-path harness caught `app restart`
/// answering 1 and 0 on timing alone.
const SETTLE_POLL_INTERVAL: Duration = Duration::from_millis(200);

/// How long a quit may take before [`stop`] reports the app still running.
const QUIT_DEADLINE: Duration = Duration::from_secs(2);

/// How long a launch may take to register with LaunchServices before [`start`]
/// says it did not (issue 0423).
///
/// **A DEADLINE, NOT A COUNT OF POLLS, AND GENEROUS ON PURPOSE.** It was ten
/// polls at 200 ms, and a relaunch straight after a quit took longer than those
/// 2 s to register: `app start` and `app restart` then reported a launch that
/// had succeeded as a failure (hv, 2026-09-16). The wait returns the moment the
/// app appears, so the length costs only a launch that is genuinely slow.
pub const LAUNCH_DEADLINE: Duration = Duration::from_secs(15);

/// The first reading that shows what is awaited, among the readings taken
/// inside `deadline` at `interval`, or `None` when the deadline passes first
/// (issue 0423).
///
/// **PURE OVER THE READINGS**, so the verdict can be driven without an app: the
/// callers hand it a lazy sequence that polls and sleeps, and it takes no more
/// of that sequence than the deadline allows.
pub fn awaited<T>(
  readings: impl IntoIterator<Item = Option<T>>,
  deadline: Duration,
  interval: Duration,
) -> Option<T> {
  let polls = deadline.as_millis() / interval.as_millis().max(1);
  readings
    .into_iter()
    .take(usize::try_from(polls).unwrap_or(usize::MAX))
    .flatten()
    .next()
}

/// What an operator is shown, and what the exit code says.
///
/// **THREE STATES, NOT TWO, AND THE THIRD IS THE ONE A BOOLEAN LOSES.** `not
/// installed` and `installed but not running` have different remedies -- build
/// it versus start it -- and a bare `running: no` sends the operator to the
/// wrong one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
  /// Answering, with the bundle LaunchServices says it launched from.
  Running { pid: u32, bundle: PathBuf },
  /// A bundle exists on disk and no process is holding it.
  Installed { bundle: PathBuf },
  /// No bundle in any location this machine builds or installs to.
  NotInstalled,
}

impl State {
  /// `0` running, `1` installed and not running, `2` not installed -- geodica's
  /// convention, kept so a script written against one tool reads the other.
  pub fn code(&self) -> i32 {
    match self {
      Self::Running { .. } => 0,
      Self::Installed { .. } => 1,
      Self::NotInstalled => 2,
    }
  }
}

/// Every bundle location this machine can produce, in the order a running app
/// would most likely have come from.
///
/// **INSTALLED BEATS BUILT, AND Release BEATS Debug.** A developer with both
/// means the installed one when they say `start`; the dev builds are here so the
/// verb works at all on a machine that has never run `app-install`, which is the
/// ordinary state of this repository.
///
/// **NOT ONE ENVIRONMENT VARIABLE IS READ HERE** (`AC-11.3`, enforced by
/// `no_intent_home.rs`). This function read `$INTENT_MACOS_STATE_DIR`,
/// `$XDG_STATE_HOME` and `$HOME` to find the build output; the guard refused all
/// three and its remedy was to route through [`crate::userstate`], which is the
/// one module allowed the ambient read. **The guard was right on its own stated
/// ground:** every machine in this estate sets those variables, so nothing here
/// would have failed, and the first report would have come from a brew install
/// on a machine with no clone.
fn candidates() -> Vec<PathBuf> {
  let mut out = vec![PathBuf::from("/Applications/Intent.app")];
  if let Ok(products) = crate::userstate::macos_app_build_dir() {
    out.push(products.join("Release/Intent.app"));
    out.push(products.join("Debug/Intent.app"));
  }
  out
}

/// The first candidate bundle that exists, if any.
pub fn installed_bundle() -> Option<PathBuf> {
  candidates().into_iter().find(|p| p.is_dir())
}

/// Ask LaunchServices for one field about the bundle.
///
/// **`lsappinfo`, NEVER `pgrep`, AND THE REASON IS NOT STYLE** (geodica's, paid
/// for there): a hardened-runtime app is not visible in the process table to its
/// own children, so a `pgrep` shelled from inside `Intent.app` reports the app
/// dead while it is plainly running. LaunchServices answers the same for every
/// caller, inside the bundle or out.
fn ls_field(field: &str) -> Option<String> {
  let out = Command::new("/usr/bin/lsappinfo")
    .args(["info", "-only", field, BUNDLE_ID])
    .output()
    .ok()?;
  if !out.status.success() {
    return None;
  }
  parse_ls_field(&String::from_utf8_lossy(&out.stdout))
}

/// The parse, split from the call so it can be driven without a running app.
///
/// `lsappinfo` answers `"pid"=8329` or `"LSBundlePath"="/path/to/Intent.app"`,
/// and **prints an empty line rather than failing when nothing holds the bundle
/// id** -- which is the case that matters, because it is the ordinary one and a
/// parse that returned `Some("")` there would read as a running app with a blank
/// path.
fn parse_ls_field(text: &str) -> Option<String> {
  let (_, after) = text.split_once('=')?;
  let value = after.trim().trim_matches('"').trim();
  (!value.is_empty()).then(|| value.to_string())
}

/// The running app's pid, or `None` when nothing holds the bundle id.
pub fn pid() -> Option<u32> {
  ls_field("pid")?.parse().ok()
}

/// Where the app is, what it is doing, and nothing derived beyond that.
pub fn status() -> State {
  match pid() {
    Some(pid) => {
      // **THE RUNNING BUNDLE IS AUTHORITATIVE OVER THE CANDIDATE LIST.** An app
      // launched from a path this function would not have guessed is still the
      // app that is running, and reporting the guess would name a bundle the
      // operator is not looking at.
      let bundle = ls_field("bundlepath")
        .map(PathBuf::from)
        .or_else(installed_bundle)
        .unwrap_or_else(|| PathBuf::from("(unknown)"));
      State::Running { pid, bundle }
    }
    None => match installed_bundle() {
      Some(bundle) => State::Installed { bundle },
      None => State::NotInstalled,
    },
  }
}

/// Errors this module can produce. Each names what to do next, because every
/// one of them is reached by an operator typing a lifecycle verb.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
  #[error(
    "Intent.app is not built or installed on this machine -- looked in /Applications and in the build output"
  )]
  NotInstalled,
  #[error("could not launch {}: {source}", bundle.display())]
  Launch {
    bundle: PathBuf,
    source: std::io::Error,
  },
  #[error("could not ask Intent.app to quit: {source}")]
  Quit { source: std::io::Error },
  #[error(
    "a quit was delivered to Intent.app (pid {pid}) and it is still running after {waited:?}"
  )]
  StillRunning { pid: u32, waited: Duration },
  /// The launch was accepted and the app had not registered when the wait ran
  /// out (issue 0423). **WHAT WAS MEASURED, NOT A VERDICT**: an app that
  /// registers a moment later is running, so this never says it did not start.
  #[error(
    "asked LaunchServices to open {} and Intent.app did not register with it within {}s",
    bundle.display(),
    waited.as_secs()
  )]
  NotRegistered { bundle: PathBuf, waited: Duration },
}

/// What to run when no bundle is found, given the install root this binary
/// resolved.
///
/// **NOT "install it" AS A BARE INSTRUCTION.** Two different commands produce a
/// bundle and they are not interchangeable: one builds a Debug app for this
/// tree, the other installs a Release one for the machine. Issue 0327:
/// `bin/devbin` exists only in a source tree, so an installed Intent is sent to
/// the release's own bundle instead. The root is a parameter so both branches
/// can be driven; [`crate::install::home`] is the one ambient read, at the call.
fn not_installed_remedy(root: Option<&std::path::Path>) -> String {
  if root.is_some_and(|root| root.join("bin/devbin").is_file()) {
    "`bin/devbin macos app-build` builds the app for this tree; `bin/devbin macos app-install` builds Release and installs it to /Applications. `intent app status` will then say where it found it.".to_string()
  } else {
    format!(
      "download `Intent.app.zip` from the v{v} release (https://github.com/matthewsinclair/intent/releases/tag/v{v}), unzip it and move `Intent.app` to /Applications. `intent app status` will then say where it found it.",
      v = env!("CARGO_PKG_VERSION")
    )
  }
}

impl crate::remedy::Remedy for AppError {
  fn remedy(&self) -> String {
    match self {
      Self::NotInstalled => not_installed_remedy(crate::install::home().ok().as_deref()),
      Self::Launch { bundle, .. } => format!(
        "LaunchServices refused to open `{}`. Check the bundle is complete -- `bin/devbin macos app-verify` reports a missing executable or Info.plist -- and rebuild it with `bin/devbin macos app-build` if it is not.",
        bundle.display()
      ),
      Self::Quit { .. } => "`osascript` could not deliver the quit. It ships with macOS, so its absence means this is not a macOS machine; otherwise quit the app from its menubar.".to_string(),
      // **THE PID IS THE REMEDY, AND `kill` IS OFFERED LAST DELIBERATELY.** The
      // app was ASKED to quit and declined, which is usually an app doing
      // something rather than an app stuck; killing it discards whatever that
      // was.
      Self::StillRunning { pid, .. } => format!(
        "Intent.app was asked to quit and has not gone. Look at it -- it may be showing a dialog. If it is genuinely wedged, `kill {pid}`, and `kill -9 {pid}` only after that fails."
      ),
      Self::NotRegistered { .. } => "it may still be starting: `intent app status` says whether it is running now, so look for a failure only if that says it is not".to_string(),
    }
  }
}

/// Launch the app, or say so if it is already up.
///
/// **`open` RATHER THAN EXECUTING THE BINARY.** `open` goes through
/// LaunchServices, which is what registers the app as a GUI session owner and
/// gives it a menubar. Executing `Contents/MacOS/Intent` directly starts a
/// process with no such registration -- which is why `app-run` exists as a
/// separate developer verb and is deliberately not what this does.
pub fn start() -> Result<(u32, PathBuf), AppError> {
  if let Some(pid) = pid() {
    let bundle = ls_field("bundlepath")
      .map(PathBuf::from)
      .unwrap_or_else(|| PathBuf::from("(unknown)"));
    return Ok((pid, bundle));
  }
  let bundle = installed_bundle().ok_or(AppError::NotInstalled)?;
  Command::new("/usr/bin/open")
    .arg(&bundle)
    .status()
    .map_err(|source| AppError::Launch {
      bundle: bundle.clone(),
      source,
    })?;
  // **A LAUNCH IS A REQUEST, EXACTLY AS A QUIT IS, AND THIS POLLS FOR THE SAME
  // REASON [`stop`] DOES.** `open` returns once LaunchServices has accepted the
  // request, not once the app has registered -- so reading `status()` on the next
  // line is a race, and it loses often enough to be caught: the dual-path
  // conformance harness found `app restart` exiting 1 down one route and 0 down
  // the other, on nothing but timing. **The asymmetry was the defect** -- this
  // module already knew a quit had to be verified, and applied it to only one of
  // the two directions.
  let readings = std::iter::from_fn(|| {
    let reading = pid();
    if reading.is_none() {
      std::thread::sleep(SETTLE_POLL_INTERVAL);
    }
    Some(reading)
  });
  match awaited(readings, LAUNCH_DEADLINE, SETTLE_POLL_INTERVAL) {
    Some(pid) => {
      let bundle = ls_field("bundlepath").map(PathBuf::from).unwrap_or(bundle);
      Ok((pid, bundle))
    }
    None => Err(AppError::NotRegistered {
      bundle,
      waited: LAUNCH_DEADLINE,
    }),
  }
}

/// Ask the app to quit, then CONFIRM it went.
///
/// Returns the pid it stopped, or `None` when nothing was running -- the caller
/// needs the difference to avoid reporting an act it did not perform.
pub fn stop() -> Result<Option<u32>, AppError> {
  let Some(pid) = pid() else {
    return Ok(None);
  };
  Command::new("/usr/bin/osascript")
    .args([
      "-e",
      &format!("tell application id \"{BUNDLE_ID}\" to quit"),
    ])
    .status()
    .map_err(|source| AppError::Quit { source })?;
  let readings = std::iter::from_fn(|| {
    std::thread::sleep(SETTLE_POLL_INTERVAL);
    Some(crate::macapp::pid().is_none().then_some(()))
  });
  match awaited(readings, QUIT_DEADLINE, SETTLE_POLL_INTERVAL) {
    Some(()) => Ok(Some(pid)),
    None => Err(AppError::StillRunning {
      pid,
      waited: QUIT_DEADLINE,
    }),
  }
}

/// Stop then start, and **a stopped app restarts rather than refusing**.
///
/// Someone typing `restart` at an app that is not running means start; refusing
/// would be correct about the word and useless about the intent. It goes through
/// both halves rather than signalling the app to relaunch itself, so a wedged
/// app is still recovered.
pub fn restart() -> Result<(u32, PathBuf), AppError> {
  stop()?;
  start()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn the_pid_and_the_bundle_path_parse_out_of_what_lsappinfo_actually_prints() {
    // Both forms taken verbatim from a live run against pid 8329 rather than
    // composed from the man page.
    assert_eq!(parse_ls_field("\"pid\"=8329\n").as_deref(), Some("8329"));
    assert_eq!(
      parse_ls_field("\"LSBundlePath\"=\"/Applications/Intent.app\"\n").as_deref(),
      Some("/Applications/Intent.app")
    );
  }

  #[test]
  fn nothing_running_is_none_and_never_an_empty_string() {
    // **THE CASE THAT MATTERS, AND IT IS THE ORDINARY ONE.** `lsappinfo` exits 0
    // and prints an empty value when no process holds the bundle id, so a parse
    // that kept `Some("")` would report a running app with a blank path -- and
    // `pid()` would then fail to parse and mask it, which is the accident that
    // looks like it works.
    for empty in ["", "\n", "\"pid\"=\n", "\"LSBundlePath\"=\"\"\n"] {
      assert_eq!(parse_ls_field(empty), None, "input: {empty:?}");
    }
  }

  #[test]
  fn the_exit_code_carries_the_state_and_the_three_are_distinct() {
    // A boolean loses the middle one, and the middle one has its own remedy.
    let running = State::Running {
      pid: 1,
      bundle: PathBuf::from("/Applications/Intent.app"),
    };
    let installed = State::Installed {
      bundle: PathBuf::from("/Applications/Intent.app"),
    };
    assert_eq!(running.code(), 0);
    assert_eq!(installed.code(), 1);
    assert_eq!(State::NotInstalled.code(), 2);
  }

  #[test]
  fn a_source_tree_is_sent_to_devbin_and_an_install_to_the_release_bundle() {
    let tree = tempfile::tempdir().expect("tempdir");
    std::fs::create_dir_all(tree.path().join("bin")).expect("bin");
    std::fs::write(tree.path().join("bin/devbin"), "").expect("devbin");
    let dev = not_installed_remedy(Some(tree.path()));
    assert!(dev.contains("`bin/devbin macos app-build`"), "{dev}");

    let keg = tempfile::tempdir().expect("tempdir");
    let installed = not_installed_remedy(Some(keg.path()));
    let release = format!("releases/tag/v{}", env!("CARGO_PKG_VERSION"));
    assert!(installed.contains(&release), "{installed}");
    assert!(!installed.contains("devbin"), "{installed}");
    assert_eq!(not_installed_remedy(None), installed);
  }

  /// Issue 0423: a launch that registers after the old 2 s is reported as
  /// started, as soon as it appears, and one that never appears inside the
  /// deadline is not.
  #[test]
  fn a_launch_that_registers_late_inside_the_deadline_is_started() {
    let late = |n: usize| (0..n).map(|_| None).chain(std::iter::once(Some(42u32)));
    let polls_in_the_old_wait = 10;
    assert_eq!(
      awaited(
        late(polls_in_the_old_wait + 5),
        LAUNCH_DEADLINE,
        SETTLE_POLL_INTERVAL
      ),
      Some(42),
      "a launch that registered after 2 s was not reported as started"
    );
    assert_eq!(
      awaited(
        late(polls_in_the_old_wait + 5),
        QUIT_DEADLINE,
        SETTLE_POLL_INTERVAL
      ),
      None,
      "the old 2 s budget should not have seen it, so this arm cannot tell the two apart"
    );
    let within = usize::try_from(LAUNCH_DEADLINE.as_millis() / SETTLE_POLL_INTERVAL.as_millis())
      .expect("a poll count fits");
    assert_eq!(
      awaited(late(within), LAUNCH_DEADLINE, SETTLE_POLL_INTERVAL),
      None,
      "a reading after the deadline was counted"
    );
    assert_eq!(
      awaited([Some(7u32)], LAUNCH_DEADLINE, SETTLE_POLL_INTERVAL),
      Some(7),
      "an app already registered was not taken at once"
    );
  }

  /// Issue 0423: when the wait runs out, the message says what was measured
  /// and the remedy sends the operator to `intent app status` first.
  #[test]
  fn a_launch_that_does_not_register_says_what_was_measured_not_that_it_failed() {
    use crate::remedy::Remedy;
    let e = AppError::NotRegistered {
      bundle: PathBuf::from("/Applications/Intent.app"),
      waited: LAUNCH_DEADLINE,
    };
    let said = e.to_string();
    assert!(said.contains("did not register with it within"), "{said}");
    assert!(!said.contains("did not start"), "{said}");
    assert!(e.remedy().contains("`intent app status`"), "{}", e.remedy());
  }

  #[test]
  fn installed_beats_built_and_release_beats_debug() {
    // The ORDER is the contract: a developer with both means the installed one.
    let c = candidates();
    let at = |needle: &str| c.iter().position(|p| p.to_string_lossy().contains(needle));
    assert_eq!(at("/Applications/"), Some(0), "candidates: {c:?}");
    let (rel, dbg) = (at("Products/Release"), at("Products/Debug"));
    if let (Some(r), Some(d)) = (rel, dbg) {
      assert!(r < d, "Release must precede Debug: {c:?}");
    }
  }
}
