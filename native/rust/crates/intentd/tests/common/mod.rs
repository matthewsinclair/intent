#![allow(dead_code)]
//! A running `intentd`, an isolated `HOME`, and a client that speaks to it.
//!
//! `dead_code` is allowed because cargo compiles this module separately into
//! EVERY test binary in the crate, so anything one suite does not use is
//! reported unused there -- a warning about the other suite's needs.
//!
//! **SHARED BECAUSE TWO SUITES NEED A LIVE DAEMON AND A SECOND COPY WOULD
//! DRIFT.** The kill-on-drop discipline and the bounded wait below are the
//! parts that are easy to get subtly wrong -- a child that outlives a failed
//! assertion holds the harness's descriptors and `cargo` never returns -- so
//! they exist once.

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;

use intentsvcs::daemon::{self, Route};
use intentsvcs::wire::{self, Request, Response};

/// How many times a condition is asked before the test gives up on it.
///
/// **A COUNT RATHER THAN A DEADLINE, AND IT IS THE BETTER EXPRESSION OF THE
/// PROPERTY.** What these loops require is that they TERMINATE, and a retry
/// count says that without asking anything about the time -- which is also what
/// keeps this workspace's clock guard (D42) true here rather than exempted. It
/// is more robust as well: a machine that suspends mid-test blows a wall-clock
/// budget and cannot disturb a count.
pub const ATTEMPTS: u32 = 500;

/// How long to pause between attempts. `sleep` yields the thread; it reads no
/// clock and answers no question about the time.
pub const PAUSE: Duration = Duration::from_millis(20);

/// A short, unique directory under `/tmp`.
///
/// **NOT `tempfile`, AND NOT FOR TIDINESS.** A unix socket address is a
/// fixed-size field, so the whole path has to fit; `$TMPDIR` on macOS is a
/// ~50-character generated path and the daemon's own suffix is another 32,
/// which leaves too little room to rely on. The daemon reported this exact
/// refusal the first time it was started under a long directory.
pub fn short_dir(prefix: &str) -> PathBuf {
  static NEXT: AtomicU32 = AtomicU32::new(0);
  let dir = PathBuf::from("/tmp").join(format!(
    "{prefix}-{}-{}",
    std::process::id(),
    NEXT.fetch_add(1, Ordering::Relaxed)
  ));
  std::fs::create_dir_all(&dir).expect("create an isolated directory");
  dir
}

/// An Intent project at a fresh short path, built through the shipped
/// initialiser.
///
/// Deliberately `intentsvcs::init` rather than a hand-assembled directory: a
/// fixture that constructed the layout itself would be a second opinion about
/// what a project is, and would keep passing after the real one changed.
pub fn project(name: &str) -> PathBuf {
  let root = short_dir("intentd-proj");
  intentsvcs::init::init(&root, name, "test", env!("CARGO_PKG_VERSION"))
    .expect("the shipped initialiser creates a project");

  // **A THREAD NAMED AFTER THE PROJECT, BECAUSE A FRESH `init` CREATES NONE.**
  // Measured rather than assumed: the first version of this asserted a fresh
  // project carries `ST0000` and it does not -- that is `init --with-st0000`.
  // The consequence was worse than a wrong assertion: two empty projects have
  // IDENTICAL listings, so a registry that served one store to everybody would
  // have passed a test written to catch exactly that. The title is what makes
  // the two answers distinguishable at all.
  let project = intentsvcs::project::Project::open(&root).expect("the project just created");
  let ctx = intentsvcs::facade::FacadeContext {
    principal: "test".to_string(),
    project_id: project.config().project_id.clone().unwrap_or_default(),
    version: env!("CARGO_PKG_VERSION").to_string(),
  };
  let mut facade = intentsvcs::facade::Facade::open(project, ctx).expect("open the new project");
  facade
    .st_new(&format!("{name} thread"))
    .expect("mint one thread so this project is distinguishable from another");
  root
}

/// A running `intentd`, killed and cleaned up when this value is dropped.
///
/// **A `Drop` GUARD RATHER THAN A KILL AFTER THE ASSERTIONS**, because a kill
/// written after them is dead code until an assertion fires, and on that day it
/// does not run -- leaving a daemon holding the test binary's descriptors and a
/// `cargo` that never returns. Its stdio is `null` for the same reason.
pub struct RunningDaemon {
  child: Child,
  home: PathBuf,
}

impl RunningDaemon {
  pub fn start() -> RunningDaemon {
    let home = short_dir("intentd-home");
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

  pub fn home(&self) -> &Path {
    &self.home
  }

  /// What the SHIPPED routing function decides, given this daemon's addresses.
  ///
  /// Deliberately `candidates_under` + `route` rather than a hand-rolled
  /// connect: a test carrying its own probe would pass while the two real ones
  /// disagreed.
  pub fn route(&self) -> Route {
    let candidates =
      daemon::candidates_under(&self.home).expect("a published address must be readable");
    daemon::route(&candidates)
  }

  /// Block until the shipped predicate says a daemon is there, or fail saying
  /// so.
  ///
  /// **IT WAITS ON THE THING UNDER TEST, NOT ON A SLEEP.** A fixed sleep is
  /// either too short on a loaded machine -- a flake that reads as a routing
  /// defect -- or too long on every other run.
  pub fn wait_until_answering(&self) {
    for _ in 0..ATTEMPTS {
      if matches!(self.route(), Route::Daemon(_)) {
        return;
      }
      std::thread::sleep(PAUSE);
    }
    panic!(
      "intentd did not answer in {ATTEMPTS} attempts under HOME={}. It either failed to bind or is not answering the probe on its accept path",
      self.home.display()
    );
  }

  /// Kill the daemon and wait for its address to go quiet, bounded.
  pub fn stop_and_settle(&mut self) {
    let _ = self.child.kill();
    let _ = self.child.wait();
    for _ in 0..ATTEMPTS {
      if matches!(self.route(), Route::InProcess) {
        return;
      }
      std::thread::sleep(PAUSE);
    }
    panic!("the address still routed to a daemon after the process was killed and reaped");
  }

  /// The socket this daemon is listening on.
  pub fn socket(&self) -> PathBuf {
    intentsvcs::userstate::daemon_socket_under(&self.home)
  }

  /// Open a connection and ask a sequence of requests on it.
  ///
  /// **ONE CONNECTION FOR THE WHOLE SEQUENCE, WHICH IS WHAT MAKES THE BINDING
  /// TESTABLE AT ALL.** `AC-08.1`'s per-connection binding is a property of a
  /// connection's history, so a helper that opened a fresh one per request
  /// could not express the case it exists for.
  ///
  /// **THIS IS THE ONE PLACE THAT DOES NOT USE THE SHIPPED CLIENT, AND THE
  /// REASON IS THE SAME SENTENCE.** `wire::ask` is one request per connection,
  /// which is what every real caller does; a multi-request conversation is a
  /// shape the CLI has no use for and only this test needs. Everything else
  /// goes through the shipped path -- see [`RunningDaemon::ask`].
  pub fn conversation(&self, requests: &[Request]) -> Vec<Response> {
    let stream = UnixStream::connect(self.socket()).expect("connect to the daemon");
    stream
      .set_read_timeout(Some(Duration::from_secs(30)))
      .expect("bounded read");
    let mut writer = &stream;
    let mut reader = BufReader::new(&stream);
    let mut answers = Vec::new();
    for request in requests {
      let framed = wire::frame(request).expect("serialisable");
      writer.write_all(&framed).expect("write the request");
      writer.flush().expect("flush");
      let mut line = String::new();
      reader.read_line(&mut line).expect("read the response");
      assert!(
        !line.is_empty(),
        "the daemon closed the connection without answering {request:?}"
      );
      answers.push(
        wire::parse_response(line.as_bytes()).expect("the daemon answers a readable response"),
      );
    }
    answers
  }

  /// Every address this daemon published, whether or not it is answering.
  ///
  /// Deliberately the shipped reader rather than a second opinion about where
  /// addresses live.
  pub fn candidates(&self) -> Vec<daemon::Endpoint> {
    daemon::candidates_under(&self.home).expect("a published address must be readable")
  }

  /// The loopback address this daemon published, if it published one.
  pub fn tcp(&self) -> Option<daemon::Endpoint> {
    self
      .candidates()
      .into_iter()
      .find(|e| matches!(e, daemon::Endpoint::Tcp(_)))
  }

  /// Ask one request over a NAMED endpoint, through the shipped client.
  pub fn ask_over(&self, endpoint: &daemon::Endpoint, request: Request) -> Response {
    match wire::ask(endpoint, &request) {
      Ok(response) => response,
      Err(e) => panic!("the shipped client could not complete a round trip over {endpoint}: {e}"),
    }
  }

  /// Ask one request, THROUGH THE SHIPPED CLIENT.
  ///
  /// **DELIBERATELY `wire::ask` RATHER THAN THIS FILE'S OWN ROUND TRIP.** The
  /// hand-rolled version above was written first and is a second opinion about
  /// the wire -- the deadline, the newline, what a closed connection means --
  /// and a fixture that carried its own client would pass while the real one
  /// was broken. Routing the common case through the shipped code is what makes
  /// most of these tests evidence about the client as well as the daemon.
  pub fn ask(&self, request: Request) -> Response {
    let endpoint = intentsvcs::daemon::Endpoint::Unix(self.socket());
    match wire::ask(&endpoint, &request) {
      Ok(response) => response,
      Err(e) => panic!("the shipped client could not complete a round trip: {e}"),
    }
  }
}

impl Drop for RunningDaemon {
  fn drop(&mut self) {
    let _ = self.child.kill();
    let _ = self.child.wait();
    let _ = std::fs::remove_dir_all(&self.home);
  }
}
