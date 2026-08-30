//! `AT-08.3`: the positive case of the routing rule, driven against a daemon
//! that actually exists.
//!
//! **THIS IS THE ONE CASE THAT COULD NOT BE CONSTRUCTED, AND IT IS THE ONLY
//! ONE THIS FILE ADDS.** `cli_routing.rs` drives `AC-08.3` exhaustively and
//! well -- absent, stale, never-accepting, inherited-descriptor,
//! accept-and-close, mixed transports, concurrency. Every one of those is a
//! phantom, and **a phantom is better evidenced by construction than by a real
//! daemon**: the inherited-descriptor race is 1-in-300 and cannot be summoned
//! on demand, while a deterministic fixture reproduces the observable exactly.
//!
//! **BUT THE POSITIVE CASE OWES A REAL SUBJECT, AND UNTIL NOW IT HAD NONE.**
//! The suite was green at 11-of-11 while every "live daemon" in it was a bare
//! listener -- which IS the phantom, so the fixture standing for the good case
//! was an instance of the bad one and the suite's discriminating power was
//! zero at maximal count. No amount of positive-controlling the checker finds
//! that. What finds it is a real daemon, driven once.
//!
//! So: start `intentd`, ask the SHIPPED routing function what it sees, stop it,
//! and ask again. The second half is not decoration -- without it a `Daemon`
//! verdict could come from anything at that address, and the point is that it
//! came from this process.

use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

use intentsvcs::daemon::{self, Route};

/// How long the daemon gets to bind before the test gives up on it.
///
/// Bounded because an unbounded wait on a child that never binds is a hung
/// build naming no test, which is strictly worse than a failure: the instrument
/// gives NO answer, and no-answer is indistinguishable from still-working.
const STARTUP_BUDGET: Duration = Duration::from_secs(10);

/// A running `intentd`, killed when this value is dropped.
///
/// **A `Drop` GUARD RATHER THAN A KILL AFTER THE ASSERTIONS**, because a kill
/// written after them is dead code until an assertion fires, and on that day it
/// does not run -- leaving a daemon holding the test binary's descriptors and a
/// `cargo` that never returns.
///
/// Its stdio is `null` for the same reason: a child inheriting the harness's
/// stdout pipe keeps `cargo` waiting on a pipe nobody will close.
struct RunningDaemon {
  child: Child,
  home: PathBuf,
}

impl RunningDaemon {
  fn start() -> RunningDaemon {
    // **NOT `tempfile`, AND NOT FOR TIDINESS.** A unix socket address is a
    // fixed-size field, so the whole path has to fit; `$TMPDIR` on macOS is a
    // ~50-character generated path and the daemon's own suffix is another 32,
    // which leaves too little room to rely on. `/tmp` is short, present on
    // every platform this runs on, and the daemon reported this exact refusal
    // the first time it was started under a long directory.
    let home = PathBuf::from("/tmp").join(format!(
      "intentd-at083-{}-{}",
      std::process::id(),
      Instant::now().elapsed().as_nanos()
    ));
    std::fs::create_dir_all(&home).expect("create an isolated HOME");

    let child = Command::new(env!("CARGO_BIN_EXE_intentd"))
      .env("HOME", &home)
      .stdout(Stdio::null())
      .stderr(Stdio::null())
      .spawn()
      .expect("intentd is built beside this test by cargo");

    let running = RunningDaemon { child, home };
    running.wait_until_answering();
    running
  }

  /// Block until the shipped predicate says a daemon is there, or fail saying
  /// so.
  ///
  /// **IT WAITS ON THE THING UNDER TEST, NOT ON A SLEEP.** A fixed sleep is
  /// either too short on a loaded machine -- a flake that reads as a routing
  /// defect -- or too long on every other run.
  fn wait_until_answering(&self) {
    let deadline = Instant::now() + STARTUP_BUDGET;
    while Instant::now() < deadline {
      if matches!(self.route(), Route::Daemon(_)) {
        return;
      }
      std::thread::sleep(Duration::from_millis(20));
    }
    panic!(
      "intentd did not answer within {STARTUP_BUDGET:?} under HOME={}. The daemon either failed to bind or is not answering the probe on its accept path",
      self.home.display()
    );
  }

  /// What the SHIPPED routing function decides, given this daemon's addresses.
  ///
  /// Deliberately `candidates_under` + `route` rather than a hand-rolled
  /// connect: a test carrying its own probe would pass while the two real ones
  /// disagreed, which is the whole failure this row is about.
  fn route(&self) -> Route {
    let candidates =
      daemon::candidates_under(&self.home).expect("a published address must be readable");
    daemon::route(&candidates)
  }
}

impl Drop for RunningDaemon {
  fn drop(&mut self) {
    let _ = self.child.kill();
    let _ = self.child.wait();
    let _ = std::fs::remove_dir_all(&self.home);
  }
}

/// Stop a daemon and wait for the address to go quiet, bounded.
fn stop_and_settle(daemon: &mut RunningDaemon) {
  let _ = daemon.child.kill();
  let _ = daemon.child.wait();
  let deadline = Instant::now() + STARTUP_BUDGET;
  while Instant::now() < deadline {
    if matches!(daemon.route(), Route::InProcess) {
      return;
    }
    std::thread::sleep(Duration::from_millis(20));
  }
  panic!("the address still routed to a daemon after the process was killed and reaped");
}

#[test]
fn a_running_intentd_is_routed_to_and_a_stopped_one_is_not() {
  let mut daemon = RunningDaemon::start();

  let endpoint = match daemon.route() {
    Route::Daemon(endpoint) => endpoint,
    Route::InProcess => panic!(
      "a real intentd is running and answering, and the routing rule sent this invocation in-process. That is the failure AC-08.3 exists to prevent: the in-process engine would open a store the daemon owns, which is two sync engines on one database"
    ),
  };

  // **THE ENDPOINT IS REPORTED, NOT ASSERTED AGAINST A CONSTANT.** There is no
  // port literal anywhere in this estate -- the daemon binds `:0` and publishes
  // what the kernel gave it -- so pinning one here would mint the constant the
  // design went out of its way not to have.
  assert!(
    daemon.home.join(".local/share/intent").exists(),
    "the daemon published under the isolated HOME it was given, not the operator's"
  );

  // **THE CONTROL THAT ATTRIBUTES THE VERDICT ABOVE TO THIS PROCESS.** Without
  // it, `Daemon` could have come from anything reachable at that address, and
  // the test would be evidence that something answered rather than that the
  // daemon did.
  stop_and_settle(&mut daemon);
  assert!(
    matches!(daemon.route(), Route::InProcess),
    "the address kept routing to a daemon after the only process that could have been serving it was killed and reaped. The positive result above cannot then be attributed to it -- endpoint was {endpoint}"
  );
}
