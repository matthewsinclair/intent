//! Helpers shared by the integration tests in this crate.
//!
//! **A DIRECTORY, NOT `tests/common.rs`.** Cargo compiles every `.rs` FILE
//! directly under `tests/` as its own test binary; a directory module is not
//! one, so this is the spelling that shares code without inventing an empty
//! test target.
//!
//! **AND NOT `testkit`, WHICH WOULD HAVE BEEN THE OBVIOUS HOME.** That crate
//! declares zero dependencies on purpose -- its own manifest says a testkit
//! with a dependency graph becomes a thing to reason about rather than a thing
//! to reach for -- and `dep_graph_guard.rs` walks every manifest under
//! `crates/`. `openpty` needs `libc`, which is already a dev-dependency HERE
//! and would be a new one THERE. Both callers of the harness below live in
//! this crate, so the narrower home is also the correct one.

#![allow(dead_code)]

/// A connected pseudo-terminal pair, as owned files.
///
/// **THE MASTER MUST OUTLIVE THE CHILD.** Dropping it closes the terminal's
/// other end, and the child then reads EOF -- or takes a hangup -- in the
/// middle of whatever it was doing. Every caller keeps it in scope for the
/// whole run, which is why this returns it rather than using it itself.
///
/// **IT IS THE ONLY WAY TO REACH A TERMINAL-GATED ARM FROM A TEST, AND THE
/// ALTERNATIVE WAS TRIED AND FAILED.** `script -q /dev/null` allocates a pty
/// from a shell, but it calls `tcgetattr` on ITS OWN stdin -- so under a
/// harness whose stdin is a socket or a pipe it exits 1 with `Operation not
/// supported on socket` and produces nothing. Two nodes measured that
/// independently on 2026-08-29, one of them after reporting the opposite from a
/// run that did not reproduce. `openpty` asks the kernel directly and does not
/// care what the ambient stdin is, so it behaves the same under a terminal,
/// under a test harness and under CI.
pub fn pty_pair() -> (std::fs::File, std::fs::File) {
  use std::os::fd::FromRawFd;
  let mut master: libc::c_int = 0;
  let mut slave: libc::c_int = 0;
  // SAFETY: both out-parameters are valid for writes, and the three null
  // pointers are documented as "use the defaults" for termios, winsize and the
  // returned slave name.
  let rc = unsafe {
    libc::openpty(
      &mut master,
      &mut slave,
      std::ptr::null_mut(),
      std::ptr::null_mut(),
      std::ptr::null_mut(),
    )
  };
  assert_eq!(
    rc,
    0,
    "openpty failed, so this test can say nothing about the arm it exists for: {}",
    std::io::Error::last_os_error()
  );
  // SAFETY: openpty returned 0, so both descriptors are open and owned by us.
  unsafe {
    (
      std::fs::File::from_raw_fd(master),
      std::fs::File::from_raw_fd(slave),
    )
  }
}

/// Everything the child wrote to the terminal, read after it has exited.
///
/// **`EIO` IS THE END OF THE STREAM HERE, NOT A FAILURE.** A pty master whose
/// every slave descriptor has closed returns `EIO` rather than a clean zero on
/// Darwin, so a reader that treats an error as a fault reports one on every
/// successful run. Anything else is propagated by panicking, because a test
/// that cannot read the terminal must not quietly return "nothing was written"
/// -- which is exactly the assertion its caller is about to make.
pub fn drain(mut master: std::fs::File) -> String {
  use std::io::Read;
  let mut out = Vec::new();
  let mut buf = [0u8; 4096];
  loop {
    match master.read(&mut buf) {
      Ok(0) => break,
      Ok(n) => out.extend_from_slice(&buf[..n]),
      Err(e) if e.raw_os_error() == Some(libc::EIO) => break,
      Err(e) => panic!(
        "reading the terminal failed, so this test can assert nothing about what was written to it: {e}"
      ),
    }
  }
  // The line discipline turns a bare newline into CRLF on the way out. Callers
  // compare against paths they built themselves, which carry neither.
  String::from_utf8_lossy(&out).replace('\r', "")
}

// ---------------------------------------------------------------------------
// A REAL `intentd`, AND THE HELPERS IT NEEDS.
//
// **MOVED HERE ON ITS THIRD CALLER, WHICH IS THE TRIGGER ITS OWN NOTE SET.**
// It lived in `daemon_and_local_agree.rs` deliberately: adding to `common`
// rebuilds every test binary in this crate, and two peers were mid-build the
// day it was written. The note said *a third caller moves it to `common`*, and
// `AC-08.5`'s carve-out witness is that caller -- a bare listener cannot answer
// `Op::Registry`, so nothing that needs a REAL daemon can be satisfied without
// this type.
//
// **A SECOND COPY WOULD DRIFT, AND THE DRIFT WOULD BE SILENT.** The parts that
// are easy to get subtly wrong -- reaping children before the parent, waiting
// on a real op rather than on the liveness probe, an isolated `HOME` -- are
// exactly the parts a copied fixture keeps while the original is fixed.
// ---------------------------------------------------------------------------

use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicU32, Ordering};

use intentsvcs::daemon::{self, Endpoint, Route};
use intentsvcs::wire::{self, Op, Request, Response};

/// How many times a condition is asked before the test gives up on it.
///
/// A retry count rather than a deadline: what these loops require is that they
/// TERMINATE, which says nothing about the time and keeps this workspace's
/// clock guard (D42) true here rather than exempted.
pub const ATTEMPTS: u32 = 400;

pub const PAUSE: std::time::Duration = std::time::Duration::from_millis(20);

/// A short, unique directory under `/tmp`.
///
/// **NOT `tempfile`, AND NOT FOR TIDINESS.** A unix socket address is a
/// fixed-size field, so the whole path has to fit; `$TMPDIR` on macOS is a
/// ~50-character generated path and the daemon's own suffix is another 32.
pub fn short_dir(tag: &str) -> PathBuf {
  static NEXT: AtomicU32 = AtomicU32::new(0);
  let dir = PathBuf::from("/tmp").join(format!(
    "intent-fixture-{tag}-{}-{}",
    std::process::id(),
    NEXT.fetch_add(1, Ordering::Relaxed)
  ));
  std::fs::create_dir_all(&dir).expect("create an isolated directory");
  dir
}

/// The pids whose parent is `pid`.
pub fn children_of(pid: u32) -> Vec<String> {
  let out = Command::new("pgrep")
    .args(["-P", &pid.to_string()])
    .output()
    .expect("pgrep runs");
  String::from_utf8_lossy(&out.stdout)
    .split_whitespace()
    .map(str::to_string)
    .collect()
}

/// A real `intentd`, started through the shipped `intent daemon run`.
pub struct RealDaemon {
  child: Child,
  home: PathBuf,
}

impl RealDaemon {
  pub fn start() -> RealDaemon {
    // **AN ISOLATED `HOME`, AND THIS IS THE ONE LINE THAT MUST NOT BE WRONG.**
    // A daemon started under the real `$HOME` answers every peer session's
    // liveness probe at once and holds the `sync`/`ingest` family off the
    // store -- so a careless fixture here takes four developers' verbs down
    // together. That is not hypothetical: it happened on this machine on
    // 2026-08-30, from an `intentd --help`.
    let home = short_dir("dualpath-home");
    let child = Command::new(env!("CARGO_BIN_EXE_intent"))
      .args(["daemon", "run"])
      .env("HOME", &home)
      .stdout(Stdio::null())
      .stderr(Stdio::null())
      .spawn()
      .expect("the shipped intent binary runs");

    let running = RealDaemon { child, home };
    running.wait_until_it_answers_a_real_op();
    running
  }

  pub fn home(&self) -> &Path {
    &self.home
  }

  pub fn endpoint(&self) -> Option<Endpoint> {
    let candidates = daemon::candidates_under(&self.home).ok()?;
    match daemon::route(&candidates) {
      Route::Daemon(endpoint) => Some(endpoint),
      Route::InProcess => None,
    }
  }

  /// Block until this daemon answers a REAL op, not merely the probe.
  ///
  /// **THE READINESS CONDITION IS DELIBERATELY STRONGER THAN `daemon::route`,
  /// AND THE DIFFERENCE IS THE WHOLE POINT OF USING A REAL DAEMON.** `route`
  /// asks whether something answers the liveness probe, and a bare listener
  /// does -- that is the phantom this crate's routing tests spend three
  /// fixtures on. `Op::Registry` requires a serving `intentd`, so waiting on it
  /// makes the fixture unfalsifiably real rather than merely present.
  pub fn wait_until_it_answers_a_real_op(&self) {
    for _ in 0..ATTEMPTS {
      if let Some(endpoint) = self.endpoint()
        && matches!(
          wire::ask(
            &endpoint,
            &Request {
              root: self.home.clone(),
              op: Op::Registry,
            },
          ),
          Ok(Response::Registry { .. })
        )
      {
        return;
      }
      std::thread::sleep(PAUSE);
    }
    panic!(
      "no intentd answered `Op::Registry` under HOME={} in {ATTEMPTS} attempts.\n\nIf `intent daemon run` refused, the usual cause is that `target/debug/intentd` is absent or stale: `cargo test -p intent-cli` builds THIS package's binaries and not another package's, so the sibling `intentd` this verb execs into is whatever an earlier build left. Run `cargo build -p intentd` (or drive the workspace) and try again.",
      self.home.display()
    );
  }

  /// How many ops this daemon has dispatched to `root`'s store.
  ///
  /// **`Op::Registry` IS IN `wire::UNCOUNTED`, WHICH IS WHY READING THE COUNTER
  /// DOES NOT MOVE IT.** vc declared that set rather than leaving it implied by
  /// a branch in the dispatcher, precisely so this measurement exists: an
  /// instrument that perturbed its own subject would report `+2` for every
  /// bracketed verb and there would be nothing to compare against.
  ///
  /// A project the daemon has never opened is not in the listing at all, and
  /// that is **0 dispatches**, not an error -- it is the state every one of
  /// these brackets starts in.
  pub fn dispatched(&self, root: &Path) -> u64 {
    let endpoint = self
      .endpoint()
      .expect("the daemon was answering when this test started");
    let response = wire::ask(
      &endpoint,
      &Request {
        root: root.to_path_buf(),
        op: Op::Registry,
      },
    )
    .expect("the shipped client completes a round trip to a live daemon");

    let Response::Registry { projects } = response else {
      panic!("intentd answered Op::Registry with something else: {response:?}");
    };
    // **CANONICALISED ON BOTH SIDES, BECAUSE `/tmp` IS A SYMLINK ON macOS.**
    // The daemon holds the resolved root; this fixture creates paths under the
    // symlinked one. Comparing the two as written finds nothing, reports 0
    // dispatches for every verb, and the `--daemon` arm fails with a message
    // about routing that would be entirely about a path.
    let wanted = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    projects
      .iter()
      .find(|p| p.root.canonicalize().unwrap_or_else(|_| p.root.clone()) == wanted)
      .map(|p| p.dispatched)
      .unwrap_or(0)
  }

  /// Is this daemon WATCHING the project at `root`?
  ///
  /// **THE OBSERVABLE `AC-08.5`'s NARROWED CARVE-OUT IS ASSERTED AGAINST, AND
  /// IT IS READ OFF THE WIRE RATHER THAN INFERRED FROM HAVING CONTACTED THE
  /// DAEMON.** A fixture that assumed *I asked about this project, therefore it
  /// is watched* would agree with itself: registration and watching are
  /// separate acts, and `watch::start` is allowed to fail leaving the project
  /// SERVED AND NOT WATCHED. That state is exactly the one where the carve-out
  /// must NOT refuse, so a test that could not see it would be unable to tell
  /// a working narrowing from a broken one.
  ///
  /// `false` for a project this daemon has never opened, which is the other
  /// half of the same question.
  pub fn watching(&self, root: &Path) -> bool {
    let endpoint = self
      .endpoint()
      .expect("the daemon was answering when this test started");
    let response = wire::ask(
      &endpoint,
      &Request {
        root: root.to_path_buf(),
        op: Op::Registry,
      },
    )
    .expect("the shipped client completes a round trip to a live daemon");
    let Response::Registry { projects } = response else {
      panic!("intentd answered Op::Registry with something else: {response:?}");
    };
    let wanted = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    projects
      .iter()
      .find(|p| p.root.canonicalize().unwrap_or_else(|_| p.root.clone()) == wanted)
      .map(|p| p.watched)
      .unwrap_or(false)
  }
}

impl Drop for RealDaemon {
  /// **REAP THE CHILDREN BEFORE THE PARENT, AND CLEAN UP IN `Drop` RATHER THAN
  /// AFTER THE ASSERTIONS.** A kill written after them is dead code until an
  /// assertion fires, and on that day it does not run -- leaving a real daemon
  /// holding the harness's descriptors and a `cargo` that never returns. The
  /// child reaping is for the case where `daemon run` ever stops being an
  /// `exec`: a spawn makes `intentd` a GRANDCHILD, so killing `intent` orphans
  /// it to init. **That is not hypothetical -- a sibling fixture leaked exactly
  /// that while its subject was broken**, which is the worst possible moment
  /// and the case a happy-path cleanup never covers.
  ///
  /// **BY PID, NEVER BY NAME.** Reaping everything that looks like an `intentd`
  /// would kill a concurrent session's daemon; four of us share this machine.
  fn drop(&mut self) {
    for child in children_of(self.child.id()) {
      let _ = Command::new("kill").arg("-TERM").arg(&child).status();
    }
    let _ = self.child.kill();
    let _ = self.child.wait();
    let _ = std::fs::remove_dir_all(&self.home);
  }
}
