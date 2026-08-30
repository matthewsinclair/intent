//! `AC-08.2`, first half: a verb the daemon cannot serve yet FALLS THROUGH to
//! in-process, and the sync/ingest family still refuses.
//!
//! **vc's RULING, 2026-08-30, AND THE REASON MATTERS MORE THAN THE RULE.**
//! `design.md:22` says *if the intentd socket exists and answers, the CLI MUST
//! route to it (never two sync engines live at once)*. The parenthetical is the
//! JUSTIFICATION for the rule rather than a second independent rule -- and this
//! estate's own measurement refuted it: the store serialises writes, a second
//! writer is refused cleanly at rc=1, readers never block, and a whole sync is
//! one transaction, so two engines cannot half-apply anything.
//!
//! **SO REFUSING EVERY VERB WAS PROTECTING AGAINST SOMETHING THAT CANNOT
//! HAPPEN, AT A COST THAT CAN.** rc=2 on every store verb while a daemon runs
//! is strictly worse for an operator than the real residual, which is
//! duplicated ingest work and last-writer-wins about which one lands.
//!
//! **THE CARVE-OUT IS NARROW BECAUSE THE PROHIBITION IS NARROW.** `sync` and
//! `ingest` are the two families where the parenthetical is literally true --
//! two of those really would both watch and both ingest -- so they keep
//! refusing until they can route.
//!
//! **THE FIXTURE IS A LISTENER THAT ANSWERS THE PROBE, NOT AN `intentd`.** The
//! CLI's behaviour here is a property of *something is answering*, and the
//! shipped recogniser and reply are what it answers with, so this is a faithful
//! stand-in rather than a convenient one. It also keeps the test inside one
//! crate: cargo builds a package's own binaries for its tests and not another's.

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixListener;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use intentsvcs::daemon;

/// A short, unique directory: a unix socket address is a fixed-size field, so
/// `$TMPDIR` on macOS leaves too little room for the daemon's own suffix.
fn short_dir(prefix: &str) -> PathBuf {
  static NEXT: AtomicU32 = AtomicU32::new(0);
  let dir = PathBuf::from("/tmp").join(format!(
    "{prefix}-{}-{}",
    std::process::id(),
    NEXT.fetch_add(1, Ordering::Relaxed)
  ));
  std::fs::create_dir_all(&dir).expect("create an isolated directory");
  dir
}

/// A listener that answers the liveness probe, stopped when dropped.
///
/// **IT ANSWERS WITH THE SHIPPED RECOGNISER AND THE SHIPPED REPLY.** A fixture
/// that wrote its own bytes would be testing that the CLI accepts whatever this
/// file happens to send, which is a different claim from the one being made.
struct AnsweringDaemon {
  home: PathBuf,
  stop: Arc<AtomicBool>,
}

impl AnsweringDaemon {
  fn start() -> AnsweringDaemon {
    let home = short_dir("fallback-home");
    let socket = intentsvcs::userstate::daemon_socket_under(&home);
    std::fs::create_dir_all(socket.parent().expect("a parent")).expect("state dir");
    let listener = UnixListener::bind(&socket).expect("bind the fixture listener");
    listener
      .set_nonblocking(true)
      .expect("so the accept loop can notice the stop flag");

    let stop = Arc::new(AtomicBool::new(false));
    let flag = Arc::clone(&stop);
    std::thread::spawn(move || {
      while !flag.load(Ordering::Relaxed) {
        match listener.accept() {
          Ok((stream, _)) => {
            let _ = stream.set_nonblocking(false);
            let mut reader = BufReader::new(&stream);
            let mut line = Vec::new();
            if reader.read_until(b'\n', &mut line).is_ok() && daemon::is_probe_frame(&line) {
              let mut out = &stream;
              let _ = out.write_all(daemon::PROBE_REPLY);
              let _ = out.flush();
            }
          }
          Err(_) => std::thread::sleep(std::time::Duration::from_millis(10)),
        }
      }
    });

    let answering = AnsweringDaemon { home, stop };
    answering.wait_until_seen();
    answering
  }

  /// **THE FIXTURE IS CONFIRMED LIVE BY THE SHIPPED PREDICATE BEFORE ANY CLAIM
  /// RESTS ON IT.** Without this, a fixture that failed to answer would make
  /// every assertion below pass for the wrong reason: the CLI would run
  /// in-process because it saw NO daemon, and the fallback would look proven
  /// while never having been exercised.
  fn wait_until_seen(&self) {
    for _ in 0..500 {
      let candidates = daemon::candidates_under(&self.home).expect("readable");
      if matches!(daemon::route(&candidates), daemon::Route::Daemon(_)) {
        return;
      }
      std::thread::sleep(std::time::Duration::from_millis(20));
    }
    panic!(
      "the fixture listener never answered the shipped probe, so nothing below would be testing the daemon-present path"
    );
  }

  fn home(&self) -> &Path {
    &self.home
  }
}

impl Drop for AnsweringDaemon {
  fn drop(&mut self) {
    self.stop.store(true, Ordering::Relaxed);
    let _ = std::fs::remove_dir_all(&self.home);
  }
}

/// An Intent project at a fresh short path, built by the shipped initialiser.
fn project() -> PathBuf {
  let root = short_dir("fallback-proj");
  intentsvcs::init::init(&root, "Fallback", "test", env!("CARGO_PKG_VERSION"))
    .expect("the shipped initialiser creates a project");
  root
}

/// Run the shipped `intent` binary in a project, with a chosen `HOME`.
fn run(home: &Path, root: &Path, argv: &[&str]) -> Output {
  Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(argv)
    .current_dir(root)
    .env("HOME", home)
    .output()
    .expect("the intent binary runs")
}

#[test]
fn a_verb_the_daemon_cannot_serve_falls_through_instead_of_refusing() {
  let daemon = AnsweringDaemon::start();
  let root = project();

  let answered = run(daemon.home(), &root, &["st", "list", "--status", "all"]);
  let stderr = String::from_utf8_lossy(&answered.stderr);
  assert_eq!(
    answered.status.code(),
    Some(0),
    "a store verb refused while a daemon was answering. Refusing is not a safety measure here: the store serialises writes, so the thing the refusal protected against cannot happen, and rc=2 on every verb is strictly worse for the operator than the real residual. stderr: {stderr}"
  );
  assert!(
    !stderr.contains("owns this project's store"),
    "the routing refusal reached a verb it no longer governs: {stderr}"
  );

  let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn the_sync_family_still_refuses_because_its_prohibition_is_literally_true() {
  let daemon = AnsweringDaemon::start();
  let root = project();

  let refused = run(daemon.home(), &root, &["sync", "--to-disk"]);
  let stderr = String::from_utf8_lossy(&refused.stderr);
  assert_eq!(
    refused.status.code(),
    Some(2),
    "sync ran alongside a daemon. This is the one family where `never two sync engines live at once` bites as written -- two of these really would both watch and both ingest. stderr: {stderr}"
  );
  assert!(
    stderr.contains("sync") && stderr.contains("ingest"),
    "the refusal must name WHY these two are different from every other verb, or it reads as the blanket refusal it replaced: {stderr}"
  );

  let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn with_no_daemon_both_verbs_behave_exactly_as_before() {
  // **THE CONTROL THAT ATTRIBUTES BOTH RESULTS ABOVE TO THE DAEMON.** Without
  // it, `st list` succeeding and `sync` refusing could be facts about those two
  // verbs rather than about routing -- and the second would be especially
  // convincing, because a `sync` that refused for its own unrelated reasons
  // would look exactly like the carve-out working.
  let home = short_dir("fallback-nodaemon-home");
  let root = project();

  let listed = run(&home, &root, &["st", "list", "--status", "all"]);
  assert_eq!(
    listed.status.code(),
    Some(0),
    "st list works with no daemon"
  );

  let synced = run(&home, &root, &["sync", "--to-disk"]);
  let stderr = String::from_utf8_lossy(&synced.stderr);
  assert_eq!(
    synced.status.code(),
    Some(0),
    "sync REFUSED with no daemon running, so the refusal above was not about routing at all: {stderr}"
  );

  let _ = std::fs::remove_dir_all(&home);
  let _ = std::fs::remove_dir_all(&root);
}
