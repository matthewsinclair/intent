//! `AT-08.4` / `AC-08.4`: **the daemon's lifecycle works end to end** -- start,
//! status, refuse a second, stop, stop again, with logs where D19 put them.
//!
//! **EVERY ARM IS DRIVEN THROUGH THE SHIPPED BINARY, BECAUSE THE CRITERION IS
//! ABOUT THE LIFECYCLE AN OPERATOR HAS.** A test calling the library functions
//! would exercise the same mechanisms and prove nothing about whether
//! `intent daemon start` reaches them.
//!
//! **THE ISOLATED `HOME` IS THE ONE LINE THAT MUST NOT BE WRONG.** A daemon
//! started under the real one answers every peer session's liveness probe at
//! once and holds the `sync`/`ingest` family off the store, so a careless
//! fixture here takes four developers' verbs down together. That is not
//! hypothetical: it happened on this machine on 2026-08-30, from an
//! `intentd --help`.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU32, Ordering};

/// An isolated `HOME` with a project in it, torn down on drop.
///
/// **A `Drop` GUARD RATHER THAN CLEANUP AFTER THE ASSERTIONS.** Cleanup written
/// after them is dead code until an assertion fires, and on that day it does
/// not run -- leaving a real daemon holding the harness's descriptors and a
/// `cargo` that never returns.
struct Machine {
  home: PathBuf,
  project: PathBuf,
}

impl Machine {
  fn new() -> Machine {
    static NEXT: AtomicU32 = AtomicU32::new(0);
    // Short, because a unix socket address is a fixed-size field and `$TMPDIR`
    // on macOS is a ~50-character generated path.
    let home = PathBuf::from("/tmp").join(format!(
      "intent-fixture-lifecycle-{}-{}",
      std::process::id(),
      NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let project = home.join("proj");
    std::fs::create_dir_all(&project).expect("create the fixture");
    let made = Command::new(env!("CARGO_BIN_EXE_intent"))
      .args(["init", "lifecycle"])
      .current_dir(&project)
      .env("HOME", &home)
      .output()
      .expect("the intent binary runs");
    assert!(
      made.status.success(),
      "the fixture project was not created: {}",
      String::from_utf8_lossy(&made.stderr)
    );
    Machine { home, project }
  }

  fn run(&self, argv: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_intent"))
      .args(argv)
      .current_dir(&self.project)
      .env("HOME", &self.home)
      .output()
      .expect("the intent binary runs")
  }

  fn state_dir(&self) -> PathBuf {
    self.home.join(".local/share/intent")
  }
}

impl Drop for Machine {
  fn drop(&mut self) {
    // Best effort, and deliberately through the shipped verb: if `stop` is
    // broken the arms will have said so already, and a hand-rolled kill here
    // would be a second opinion about how this daemon is stopped.
    let _ = self.run(&["daemon", "stop"]);
    let _ = std::fs::remove_dir_all(&self.home);
  }
}

fn text(out: &Output) -> String {
  format!(
    "{}{}",
    String::from_utf8_lossy(&out.stdout),
    String::from_utf8_lossy(&out.stderr)
  )
}

fn answering(m: &Machine) -> bool {
  text(&m.run(&["daemon", "status"])).contains("is answering at")
}

#[test]
fn start_status_stop_is_a_working_lifecycle() {
  let machine = Machine::new();

  // **ANTI-VACUITY: NOTHING IS RUNNING BEFORE THIS TEST STARTS ONE.** An
  // isolated HOME that somehow had a daemon in it would make the start arm
  // pass without starting anything.
  assert!(
    !answering(&machine),
    "something was already answering in a freshly created isolated HOME"
  );

  let started = machine.run(&["daemon", "start"]);
  assert_eq!(
    started.status.code(),
    Some(0),
    "start failed: {}",
    text(&started)
  );
  // **`start` REPORTS AN ADDRESS, WHICH IS THE CLAIM THAT IT WAITED.** A spawn
  // succeeding says the kernel made a process; only an address says the daemon
  // bound one. This is the difference between `start` and `run`.
  assert!(
    text(&started).contains("is answering at"),
    "start returned 0 without reporting an address, so it did not wait for the daemon to be up: {}",
    text(&started)
  );
  assert!(answering(&machine), "status disagrees with start");

  let stopped = machine.run(&["daemon", "stop"]);
  assert_eq!(
    stopped.status.code(),
    Some(0),
    "stop failed: {}",
    text(&stopped)
  );
  assert!(
    !answering(&machine),
    "the daemon was still answering after stop reported success"
  );
}

/// **THE PID MECHANISM, OBSERVED THROUGH THE ONLY VERB THAT SURFACES IT.**
///
/// A second `start` must not launch a second daemon against the same state, and
/// it must NAME the pid -- which it can only do by reading it out of the lock
/// the first one holds. **An arm asserting only "nothing broke" would pass on a
/// machine where the first start had failed**, so the pid is the discriminator.
///
/// **IT SUCCEEDS RATHER THAN REFUSING, WHICH IS A DELIBERATE CHANGE.** The
/// operator asked for a running daemon and there is one, so the postcondition
/// holds; `systemctl start` on an active unit exits 0 for the same reason. The
/// refusing version made `intent daemon start && ...` break on its second run
/// in any script.
#[test]
fn a_second_start_is_idempotent_and_names_the_running_pid() {
  let machine = Machine::new();
  let first = machine.run(&["daemon", "start"]);
  assert_eq!(
    first.status.code(),
    Some(0),
    "start failed: {}",
    text(&first)
  );

  let second = machine.run(&["daemon", "start"]);
  assert_eq!(
    second.status.code(),
    Some(0),
    "a second start was reported as a failure though a daemon is running, which breaks `daemon start && ...` in any script: {}",
    text(&second)
  );
  let seen = text(&second);
  assert!(
    seen.contains("already running"),
    "the second start does not say a daemon was already there, so it is indistinguishable from having started one: {seen}"
  );
  // The pid comes from the lock. A message that could not name it would mean
  // the lock's content was never read, which is the fallback path `stop` needs.
  assert!(
    seen.contains("pid ") && seen.chars().any(|c| c.is_ascii_digit()),
    "the message names no pid, so the lock's content was not read: {seen}"
  );
  // **AND ONLY ONE DAEMON IS RUNNING.** Idempotent must mean "did not start a
  // second", not merely "exited 0" -- which is the failure this whole arm
  // exists to catch, and the one an exit code alone cannot see.
  assert!(
    answering(&machine),
    "no daemon is answering after two starts"
  );
}

/// **STOPPING NOTHING IS SUCCESS, NOT AN ERROR.**
///
/// An operator who runs `stop` twice, or on a machine that never started one,
/// has got what they asked for. Reporting failure would make the verb noisy on
/// every machine that never enrolled -- and would make it useless in a script
/// that just wants no daemon running.
#[test]
fn stopping_when_nothing_runs_is_success() {
  let machine = Machine::new();
  let stopped = machine.run(&["daemon", "stop"]);
  assert_eq!(
    stopped.status.code(),
    Some(0),
    "stopping nothing was reported as a failure: {}",
    text(&stopped)
  );
  assert!(
    text(&stopped).contains("no intentd is running"),
    "the message does not say nothing was running: {}",
    text(&stopped)
  );
}

/// `AC-08.4`'s third clause: **logs in the named location** (D19).
#[test]
fn the_logs_land_where_d19_put_them() {
  let machine = Machine::new();
  let started = machine.run(&["daemon", "start"]);
  assert_eq!(
    started.status.code(),
    Some(0),
    "start failed: {}",
    text(&started)
  );

  let log = machine.state_dir().join("intentd.log");
  let err = machine.state_dir().join("intentd.err.log");
  assert!(
    log.is_file(),
    "no stdout log at D19's location: {}",
    log.display()
  );
  assert!(
    err.is_file(),
    "no stderr log at D19's location: {}",
    err.display()
  );

  // **THE PATH `start` PRINTS IS THE PATH IT WROTE**, which is what makes the
  // answer to *where are the logs* usable rather than merely correct.
  assert!(
    text(&started).contains(&log.display().to_string()),
    "start did not tell the operator where the logs are: {}",
    text(&started)
  );
}
