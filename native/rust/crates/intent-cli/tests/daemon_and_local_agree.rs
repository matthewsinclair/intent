//! `AT-08.2` / `AC-08.2`: **the same verb, answered locally and answered by a
//! real `intentd`, produces identical results.**
//!
//! **THE HARD PART IS NOT THE COMPARISON, IT IS PROVING THE TWO RUNS WERE
//! DIFFERENT RUNS.** A harness that runs a verb twice and diffs the output
//! agrees with itself by construction the moment the client stops routing:
//! two in-process runs are identical, so "both answered the same" is exactly
//! what a completely broken `--daemon` produces. **The green is
//! indistinguishable from the defect**, which is this estate's oldest class and
//! the reason `AC-08.2` was given a discriminator before it was given a test.
//!
//! **SO EVERY COMPARISON HERE IS BRACKETED BY THE DAEMON'S OWN DISPATCH
//! COUNTER** (vc's ruling, 2026-08-30). `RegisteredProject::dispatched` counts
//! ops the daemon sent to a project's store -- never connections, never
//! liveness probes -- so reading it before and after a single verb gives
//! per-verb attribution with no per-verb wire surface:
//!
//! - the LOCAL run must move it by **0**. That arm is what proves a local run
//!   is local, and it is the one that fails if the reversal ever un-reverses.
//! - the `--daemon` run must move it by exactly **1**. That arm proves the
//!   answer came from another process rather than from a client that quietly
//!   fell back.
//!
//! **DECLARED AT THE CLIENT, OBSERVED AT THE SERVER.** The path list is
//! `render::daemon_servable_paths()` and the count comes off the wire, so the
//! two facts have independent sources -- which is what makes this a test rather
//! than a tautology.
//!
//! **THE DAEMON IS A REAL `intentd`, STARTED THROUGH `intent daemon run`.** A
//! fixture listener would be the fixture-that-is-an-instance-of-the-bad-case
//! again: this crate's suite once ran 11 of 11 green while every "live daemon"
//! in it was a bare listener, which IS the phantom those tests existed to
//! reject. Nothing here can be satisfied by a listener, because a listener
//! cannot answer `Op::Registry`.
//!
//! **AND `AT-08.2`'s DECLARED PATH IS `crates/intentd/tests/`, WHICH THIS FILE
//! IS NOT. THAT IS DELIBERATE AND IS WITH vc.** `AC-08.2` is about *the verb
//! surface*, and only the CLI has one; `intentd`'s integration tests cannot
//! reach the `intent` binary at all, because cargo builds a package's own
//! binaries for its tests and not another package's. An op-level comparison
//! could live there -- daemon `Response` against `Facade` -- but it would be a
//! different and weaker claim than the row's own words. **Reported rather than
//! resolved here: picking one of two disagreeing texts is how a design gets
//! amended by whoever was typing.**

use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::sync::atomic::{AtomicU32, Ordering};

use intentsvcs::daemon::{self, Endpoint, Route};
use intentsvcs::wire::{self, Op, Request, Response};

/// How many times a condition is asked before the test gives up on it.
///
/// A retry count rather than a deadline: what these loops require is that they
/// TERMINATE, which says nothing about the time and keeps this workspace's
/// clock guard (D42) true here rather than exempted.
const ATTEMPTS: u32 = 400;

const PAUSE: std::time::Duration = std::time::Duration::from_millis(20);

/// The title minted into the fixture project.
///
/// **THE GUARD AGAINST A VACUOUS AGREEMENT.** Two empty listings are identical,
/// and so are two error messages. Requiring the project's own thread to appear
/// in both answers means the comparison is between two real renderings of real
/// data rather than between two blanks.
const MINTED: &str = "the thread both paths have to find";

/// A short, unique directory under `/tmp`.
///
/// **NOT `tempfile`, AND NOT FOR TIDINESS.** A unix socket address is a
/// fixed-size field, so the whole path has to fit; `$TMPDIR` on macOS is a
/// ~50-character generated path and the daemon's own suffix is another 32.
fn short_dir(tag: &str) -> PathBuf {
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
fn children_of(pid: u32) -> Vec<String> {
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
struct RealDaemon {
  child: Child,
  home: PathBuf,
}

impl RealDaemon {
  fn start() -> RealDaemon {
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

  fn home(&self) -> &Path {
    &self.home
  }

  fn endpoint(&self) -> Option<Endpoint> {
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
  fn wait_until_it_answers_a_real_op(&self) {
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
  fn dispatched(&self, root: &Path) -> u64 {
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

/// A fixture project directory, removed when this value is dropped.
///
/// **A `Drop` GUARD RATHER THAN A `remove_dir_all` AFTER THE ASSERTIONS**, and
/// the first version of this file got it wrong in the way that is easy to miss:
/// the DAEMON was cleaned up in `Drop` and the PROJECT was cleaned up on the
/// last line. **Cleanup written after the assertions is dead code until an
/// assertion fires, and on that day it does not run** -- so the directories
/// accumulate precisely on the runs where something went wrong, which is when
/// somebody is least likely to be looking at `/tmp`. Measured: 14 of them from
/// today's red runs before this existed.
///
/// **IT IS HERE RATHER THAN IN `tests/common/mod.rs` DELIBERATELY, AND THE
/// REASON IS TIMING, NOT DESIGN.** `common` is compiled into every test binary
/// in this crate, so adding to it rebuilds ~70 targets for every session on
/// this machine -- and two peers are mid-build while this is being written, one
/// of them measuring build cost. **A third caller moves it to `common`**;
/// `routing_is_opt_in.rs` is the second and is owed the same treatment the next
/// time the tree is quiet.
struct Fixture(PathBuf);

impl Fixture {
  fn path(&self) -> &Path {
    &self.0
  }
}

impl Drop for Fixture {
  fn drop(&mut self) {
    let _ = std::fs::remove_dir_all(&self.0);
  }
}

/// An Intent project at a fresh short path, carrying one findable thread.
fn project() -> Fixture {
  let root = short_dir("dualpath-proj");
  intentsvcs::init::init(&root, "DualPath", "test", env!("CARGO_PKG_VERSION"))
    .expect("the shipped initialiser creates a project");

  let opened = intentsvcs::project::Project::open(&root).expect("the project just created");
  let ctx = intentsvcs::facade::FacadeContext {
    principal: "test".to_string(),
    project_id: opened.config().project_id.clone().unwrap_or_default(),
    version: env!("CARGO_PKG_VERSION").to_string(),
  };
  let mut facade = intentsvcs::facade::Facade::open(opened, ctx).expect("open the new project");
  let id = facade.st_new(MINTED).expect("mint one thread");
  // Started, because `st list` filters to WIP by default and a fresh thread is
  // not WIP -- so an unstarted fixture makes both paths agree on an EMPTY
  // listing, which is the vacuous agreement this file exists to refuse.
  facade.st_triage(&id).expect("triage it");
  facade.st_start(&id).expect("start it");
  Fixture(root)
}

fn run(home: &Path, root: &Path, argv: &[&str]) -> Output {
  Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(argv)
    .current_dir(root)
    .env("HOME", home)
    .output()
    .expect("the intent binary runs")
}

fn stdout(out: &Output) -> String {
  String::from_utf8_lossy(&out.stdout).to_string()
}

#[test]
fn every_servable_verb_answers_identically_locally_and_through_a_real_daemon() {
  let daemon = RealDaemon::start();
  let project = project();
  let root = project.path();

  let servable = intent_cli::render::daemon_servable_paths();
  assert!(
    !servable.is_empty(),
    "this build declares no daemon-servable verb, so AC-08.2 has nothing to compare and this test \
     would pass over an empty set"
  );

  for path in &servable {
    let argv: Vec<&str> = path.split(' ').collect();
    let mut daemon_argv = vec!["--daemon"];
    daemon_argv.extend(path.split(' '));

    let before = daemon.dispatched(root);

    let local = run(daemon.home(), root, &argv);
    let after_local = daemon.dispatched(root);

    let remote = run(daemon.home(), root, &daemon_argv);
    let after_remote = daemon.dispatched(root);

    // **THE LOCAL ARM FIRST, BECAUSE IT IS THE ONE THAT CATCHES THE REVERSAL
    // UN-REVERSING.** A build that went back to routing by default would still
    // pass the identity comparison below -- both runs would go to the daemon
    // and agree perfectly. This is the only assertion that can tell them apart.
    assert_eq!(
      after_local, before,
      "`intent {path}` moved the daemon's dispatch counter from {before} to {after_local}, so it \
       went over the wire. Routing is OPT-IN (hv, 2026-08-30): a daemon being up must not change \
       what a plain local command does"
    );
    assert_eq!(
      local.status.code(),
      Some(0),
      "the local run failed: {}",
      String::from_utf8_lossy(&local.stderr)
    );

    // **THE DAEMON ARM, AND `+1` RATHER THAN `> before` ON PURPOSE.** A client
    // that asked twice, or that probed the store on the way past, is a
    // different defect from one that never asked, and `greater than` cannot
    // tell either of them from the correct behaviour.
    assert_eq!(
      after_remote,
      after_local + 1,
      "`intent --daemon {path}` moved the dispatch counter from {after_local} to {after_remote}, \
       and exactly one dispatch was expected. Zero means the client answered locally and the \
       comparison below would be a run against itself; more than one means it asked more than \
       once. stderr: {}",
      String::from_utf8_lossy(&remote.stderr)
    );
    assert_eq!(
      remote.status.code(),
      Some(0),
      "the --daemon run failed: {}",
      String::from_utf8_lossy(&remote.stderr)
    );

    // **NON-VACUOUS BEFORE IDENTICAL.** Checked in this order deliberately: two
    // empty answers are identical, so asserting identity first would report a
    // pass on the one case that proves nothing.
    assert!(
      stdout(&local).contains(MINTED),
      "the local answer to `{path}` does not name the project's own thread, so there is nothing \
       here for the daemon to agree WITH: {}",
      stdout(&local)
    );

    assert_eq!(
      stdout(&remote),
      stdout(&local),
      "`intent --daemon {path}` and `intent {path}` produced different output. D32: the two entry \
       skins must return identical results, and the counter above says these really were two \
       different paths"
    );
  }
}

#[test]
fn the_dispatch_counter_can_tell_the_two_paths_apart() {
  // **THE POSITIVE CONTROL FOR THE INSTRUMENT, WITHOUT WHICH EVERY BRACKET
  // ABOVE IS FREE.** If `dispatched` returned a constant -- a canonicalisation
  // that never matches, a registry that forgets, a counter wired to nothing --
  // the local arm passes (it wants no change) and only the daemon arm fails,
  // which reads as a routing defect rather than as a broken measurement.
  //
  // **A GATE THAT CANNOT SAY *I COULD NOT MEASURE* EVENTUALLY SAYS SOMETHING
  // FALSE INSTEAD**, so this makes the counter demonstrate BOTH of its
  // outcomes -- a move and a non-move -- against known inputs.
  let daemon = RealDaemon::start();
  let project = project();
  let root = project.path();

  let servable = intent_cli::render::daemon_servable_paths();
  let path = servable
    .first()
    .expect("this build declares at least one daemon-servable verb");
  let mut daemon_argv = vec!["--daemon"];
  daemon_argv.extend(path.split(' '));

  let start = daemon.dispatched(root);

  // Reading it twice with nothing in between must not move it: `Op::Registry`
  // is declared UNCOUNTED, and an instrument that counted its own reads would
  // report a phantom dispatch for every bracket.
  assert_eq!(
    daemon.dispatched(root),
    start,
    "reading the counter moved the counter, so every before-and-after bracket in this file is \
     measuring the measurement. `Op::Registry` is in `wire::UNCOUNTED` for exactly this reason"
  );

  // And it must actually move when something is dispatched, or the equality
  // above is the equality of two constants.
  let _ = run(daemon.home(), root, &daemon_argv);
  assert_eq!(
    daemon.dispatched(root),
    start + 1,
    "the counter did not move for a verb that was dispatched, so it cannot witness anything and \
     the local arm above passes for free"
  );
}
