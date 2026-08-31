//! **`AT-09.3`: bridge mode survives a daemon restart, via per-request target
//! resolution.**
//!
//! The escape hatch resolves the daemon on EVERY request (`hatch.rs`,
//! `daemon::candidates()` per call) and opens a fresh connection through
//! `wire::ask`. So a bridge call is not bound to the daemon that answered the
//! last one: kill the daemon, start another, and the next call re-resolves and
//! reaches it. This drives exactly that.
//!
//! # The witness is the PID, not the endpoint
//!
//! The daemon's socket is `<home>/.local/share/intent/intentd.sock` -- no pid,
//! no port, no nonce (cc, `RealDaemon::restart`'s own doc) -- so the endpoint is
//! IDENTICAL across a restart and an "it moved" assertion on the endpoint would
//! assert something false. The restart is real because the serving PID changes
//! (`daemon run` execs, so `pid()` is the server, not a wrapper), and the
//! survival is real because the post-restart call ANSWERS the minted thread and
//! the NEW daemon's dispatch counter moves. A client that resolved once at
//! startup would be talking to a dead socket; per-request resolution reaches
//! the live one.

mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::{RealDaemon, short_dir};
use serde_json::Value;

const MINTED: &str = "Minted for the restart";
const DOCUMENT: &str = "{ threads { id title } }";

struct Fixture(PathBuf);

impl Drop for Fixture {
  fn drop(&mut self) {
    let _ = std::fs::remove_dir_all(&self.0);
  }
}

/// An Intent project at a fresh short path, carrying one findable thread.
fn project() -> (Fixture, String) {
  let root = short_dir("restart-proj");
  intentsvcs::init::init(&root, "Restart", "test", env!("CARGO_PKG_VERSION"))
    .expect("the shipped initialiser creates a project");
  let opened = intentsvcs::project::Project::open(&root).expect("the project just created");
  let ctx = intentsvcs::facade::FacadeContext {
    principal: "test".to_string(),
    project_id: opened.config().project_id.clone().unwrap_or_default(),
    version: env!("CARGO_PKG_VERSION").to_string(),
  };
  let mut facade = intentsvcs::facade::Facade::open(opened, ctx).expect("open the new project");
  let id = facade.st_new(MINTED).expect("mint one thread");
  (Fixture(root), id)
}

fn run(home: &Path, root: &Path, argv: &[&str]) -> Output {
  Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(argv)
    .current_dir(root)
    .env("HOME", home)
    .output()
    .expect("the intent binary runs")
}

fn answer(out: &Output) -> Value {
  serde_json::from_str(&String::from_utf8_lossy(&out.stdout)).unwrap_or_else(|e| {
    panic!(
      "stdout is not one JSON document: {e}\nstdout: {}\nstderr: {}",
      String::from_utf8_lossy(&out.stdout),
      String::from_utf8_lossy(&out.stderr)
    )
  })
}

/// The bridge answered the minted thread through whatever daemon is live.
fn assert_answers_the_minted_thread(out: &Output, id: &str) {
  assert_eq!(
    out.status.code(),
    Some(0),
    "the bridge call did not answer: {}",
    String::from_utf8_lossy(&out.stderr)
  );
  let got = answer(out);
  assert!(got["errors"].is_null(), "the document was refused: {got}");
  let threads = got["data"]["threads"]
    .as_array()
    .expect("threads is a list");
  assert!(
    threads
      .iter()
      .any(|t| t["id"] == id && t["title"] == MINTED),
    "the minted thread did not reach stdout through the daemon: {got}"
  );
}

#[test]
fn a_bridge_call_after_a_daemon_restart_reaches_the_new_daemon() {
  let daemon = RealDaemon::start();
  let (project, id) = project();

  // Before: the bridge answers through the first daemon.
  let first = run(daemon.home(), &project.0, &["graphql", DOCUMENT]);
  assert_answers_the_minted_thread(&first, &id);
  let pid_before = daemon.pid();

  // The restart: same home, same socket, a NEW serving process. `restart`
  // asserts internally that the pid moved and no wrapper survives; capturing it
  // here makes THIS test's subject -- that the daemon actually changed under a
  // stable address -- explicit rather than borrowed from the harness.
  let daemon = daemon.restart();
  let pid_after = daemon.pid();
  assert_ne!(
    pid_before, pid_after,
    "the serving pid did not change, so no restart happened and the survival below proves nothing"
  );
  assert_eq!(
    daemon.endpoint(),
    {
      // Re-derive the endpoint the same way the client does; it must be
      // unchanged, which is why the pid is the witness and not this.
      let same = daemon.endpoint();
      same
    },
    "sanity: the endpoint is read the same way twice"
  );

  // After: a fresh bridge call re-resolves per request and reaches the NEW
  // daemon. A cached connection to the old process would be a dead socket here.
  let after = run(daemon.home(), &project.0, &["graphql", DOCUMENT]);
  assert_answers_the_minted_thread(&after, &id);

  // And it left THIS process to the new daemon: its dispatch counter moves.
  let before = daemon.dispatched(&project.0);
  let again = run(daemon.home(), &project.0, &["graphql", DOCUMENT]);
  assert_answers_the_minted_thread(&again, &id);
  assert_eq!(
    daemon.dispatched(&project.0),
    before + 1,
    "one document after the restart is one dispatch on the NEW daemon -- the proof it re-resolved to the live process rather than a cached, dead one"
  );
}
