//! AT-08.3 / AC-08.3: the CLI routing rule.
//!
//! **THE RULE IS `EXISTS AND ANSWERS`, AND THE STALE ENDPOINT IS THE CASE THAT
//! SEPARATES IT FROM `PRESENT`.** A unix socket file outlives the process that
//! bound it, so a presence test routes every store verb at an address nobody is
//! listening on -- while the in-process engine that would have served the
//! request was available the whole time. The criterion requires that case to be
//! DRIVEN rather than inferred from the live and absent cases, which is
//! `a_stale_socket_file_runs_in_process` below: it binds, drops the listener,
//! asserts the file is still there, and only then asks for the route.
//!
//! **PRODUCTION HANDS THE RULE ONE CANDIDATE; THIS FILE HANDS IT MIXED LISTS.**
//! **THE POSITIVE CASE IS NOT IN THIS FILE, AND THAT IS WHERE TO LOOK NEXT.**
//! Every "live daemon" here is a constructed fixture, which is right for the
//! phantoms -- the inherited-descriptor race is 1-in-300 and cannot be summoned
//! -- and was WRONG for the good case: this suite was green at 11-of-11 while
//! every live fixture in it was a bare listener, which IS the phantom. A real
//! `intentd` drives the happy path once in
//! `crates/intentd/tests/routing_against_a_real_daemon.rs`, and that file is
//! what closed `AT-08.3`.
//!
//! `daemon::route` folds over candidates and its body names no transport, so a
//! third one is a variant rather than an edit -- but a list that is only ever
//! driven with a single element cannot exhibit a body that stopped being
//! transport-agnostic. So the ordering test runs both transports in both
//! orders. That is the AC-08.2 lesson one layer up: a corpus that cannot
//! exhibit the defect passes for free, and its green is indistinguishable from
//! a real one.
//!
//! **`the_shipped_cli_routes_on_a_live_socket_and_not_otherwise` IS THE ONE
//! THAT PROVES THE RULE IS WIRED.** Everything above it tests a function; that
//! test drives the shipped binary against a fixture `$HOME`, with the socket
//! absent, then live, then removed. **The first and third runs are the
//! controls**: they must agree with each other exactly, which is what
//! establishes that the difference in the middle run is the socket and not the
//! fixture, the ordering, or anything the second run left behind.
//!
//! **WHAT THIS FILE DOES NOT REACH.** It knows nothing about what happens after
//! routing, because nothing does yet -- there is no daemon and no client, and
//! the routed arm is a refusal. When the client lands, the assertions on that
//! arm change from "refuses, naming the endpoint" to "answers"; the three-point
//! structure and the stale case do not.

use std::fs;
use std::io::Write;
use std::net::{Ipv4Addr, SocketAddr, TcpListener};
use std::os::unix::net::UnixListener;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::time::Duration;

use intentsvcs::daemon::{self, Endpoint, Route};
use intentsvcs::userstate;

/// The longest `sun_path` a unix socket address can hold: 104 bytes on macOS,
/// 108 on Linux. Overrunning it fails at bind and at connect with an error
/// naming neither the limit nor the path, so the fixtures check themselves
/// rather than reporting a length problem as a routing result.
const SUN_PATH_MAX: usize = 104;

/// A minimal project the CLI will open, in a temp directory.
fn project() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let config = dir.path().join("intent").join(".config");
  fs::create_dir_all(&config).expect("mkdir");
  fs::write(
    config.join("config.json"),
    "{\n  \"intent_version\": \"3.0.0\",\n  \"project_name\": \"Routing\",\n  \"author\": \"cc\",\n  \"intent_dir\": \"intent\",\n  \"languages\": [\"rust\"]\n}\n",
  )
  .expect("write config");
  dir
}

/// A socket path under `root`, with its parent made and its length checked.
///
/// **THE LAYOUT COMES FROM `userstate`, NOT FROM HERE.** Spelling
/// `.local/share/intent/intentd.sock` out in a test would make the test a
/// second home for the address the CLI resolves -- and a second home that
/// agrees today is the one that stops agreeing silently.
fn socket_path(root: &Path) -> PathBuf {
  let path = userstate::daemon_socket_under(root);
  fs::create_dir_all(path.parent().expect("socket has a parent")).expect("mkdir socket dir");
  assert!(
    path.as_os_str().len() < SUN_PATH_MAX,
    "the fixture socket path is {} bytes, over the {SUN_PATH_MAX}-byte sun_path limit: {}",
    path.as_os_str().len(),
    path.display()
  );
  path
}

/// Serialises socket-fixture teardown against process spawns.
///
/// **A DROPPED LISTENER CAN KEEP ANSWERING, AND THE CAUSE IS `fork`.** macOS
/// has no `SOCK_CLOEXEC`, so a unix socket is created by `socket()` and then
/// marked close-on-exec by a SECOND syscall. A `fork` landing between the two
/// leaks the listening fd into the child, and the socket then stays alive --
/// still accepting connections -- until that child exits, however long ago its
/// owner dropped it. Measured here rather than reasoned about: with a sibling
/// thread spawning children, 1 of 300 dropped listeners still answered, and
/// the suite failed 9 times in 30 runs before this existed. **Serialised
/// (`--test-threads 1`) it failed 0 in 25, which is what named the mechanism.**
///
/// **THIS IS THE TEST PROCESS'S OWN HAZARD AND NOT A DEFECT IN THE RULE** --
/// see [`Endpoint::answers`], which records what it means in the field. Nothing
/// in a real deployment binds and releases sockets while forking in another
/// thread; this binary does, because one test drives the shipped CLI while the
/// others build fixtures.
static FORK_GUARD: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// Held across anything that must not overlap a `fork`.
///
/// Poisoning is deliberately ignored: a panic in one test must not convert
/// every sibling into a second, louder failure about a lock.
fn no_forks_here() -> std::sync::MutexGuard<'static, ()> {
  FORK_GUARD.lock().unwrap_or_else(|e| e.into_inner())
}

/// A socket file whose listener has gone: the case that separates `exists`
/// from `answers`.
///
/// **THE FIXTURE CHECKS ITSELF, IN ONE PLACE, FOR BOTH ITS USERS.** Every
/// assertion driven off this endpoint reads a routing RESULT, so a fixture that
/// is not stale produces a failure blaming the rule -- the one part that is
/// certainly innocent.
fn stale_endpoint(root: &Path) -> Endpoint {
  let path = socket_path(root);
  let guard = no_forks_here();
  drop(UnixListener::bind(&path).expect("bind the stale fixture"));
  assert!(
    path.exists(),
    "the stale-socket fixture is not stale: the file did not outlive its listener, so it cannot distinguish `exists` from `answers`"
  );
  let endpoint = Endpoint::Unix(path);
  assert!(
    !endpoint.answers(),
    "the stale-socket fixture IS answering: {endpoint}. A child process is holding the listening fd -- see FORK_GUARD"
  );
  drop(guard);
  endpoint
}

/// A TCP address nothing is listening on.
///
/// **BOUND, RELEASED, AND THEN CHECKED, BECAUSE THE FIRST TWO ARE NOT
/// SUFFICIENT.** Binding port 0 and reading `local_addr` proves the port was
/// free at that instant; it proves nothing about the instant the rule probes
/// it, because the kernel hands ephemeral ports out from a small range and
/// anything else on the machine may take it in between. Measured: with eight
/// copies of this binary running at once, 1 in 32 runs saw the released port
/// reclaimed. **So the fixture verifies its own claim and tries again, rather
/// than asserting a property it merely arranged.**
fn closed_port() -> SocketAddr {
  const TRIES: usize = 16;
  for _ in 0..TRIES {
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).expect("bind ephemeral");
    let addr = listener.local_addr().expect("local_addr");
    drop(listener);
    if !Endpoint::Tcp(addr).answers() {
      return addr;
    }
  }
  panic!(
    "could not obtain a closed loopback port in {TRIES} attempts: every port this fixture released was reclaimed before it could be probed"
  );
}

/// What a conforming endpoint sends back. Its CONTENT is deliberately not a
/// contract -- see `daemon::completes_a_round_trip`, which requires a byte and
/// says nothing about which byte.
const REPLY: &[u8] = b"{\"ok\":true}\n";

/// How long a responder waits between polls of its own stop flag.
const POLL: Duration = Duration::from_millis(1);

/// How long a responder waits for the probe's request before giving up on it.
const SERVE_BUDGET: Duration = Duration::from_millis(500);

/// What a fixture endpoint does with a connection it has accepted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Behaviour {
  /// Read the request, then answer it -- what a daemon does.
  Answer,
  /// Accept and close without writing: a daemon shutting down, or a server
  /// dropping input it cannot parse. **A `read` of `Ok(0)`, which is a clean
  /// EOF and not a timeout** -- a third phantom, reached by neither the stale
  /// file nor the inherited descriptor.
  AcceptAndClose,
}

/// Serve one accepted connection.
///
/// **THE REPLY IS CONDITIONAL ON HAVING READ THE REQUEST, WHICH IS WHAT MAKES
/// THE PROBE'S WRITE LOAD-BEARING.** A fixture that answers whatever arrives --
/// including nothing -- cannot tell a round trip from a bare connect that
/// happened to be greeted, so a probe that stopped sending anything would still
/// find every fixture live.
fn serve_probe<S: std::io::Read + Write>(mut stream: S, mode: Behaviour) -> std::io::Result<bool> {
  if mode == Behaviour::AcceptAndClose {
    return Ok(true);
  }
  let mut request = [0u8; 64];
  match stream.read(&mut request) {
    Ok(read) if read > 0 => stream.write_all(REPLY).map(|()| true),
    // A daemon does not answer a request it never received.
    _ => Ok(true),
  }
}

/// A listener a [`Responder`] can drive without knowing its transport.
///
/// The same reason `daemon::Probeable` exists one crate over: `std` gives
/// `UnixListener` and `TcpListener` the same shape and no common trait, and two
/// copies of an accept loop is two things that can stop agreeing.
trait Listens: Send + Sync + 'static {
  fn set_nonblocking(&self, on: bool) -> std::io::Result<()>;
  /// `Ok(true)` served a connection, `Ok(false)` there was none waiting.
  ///
  /// **THE ACCEPTED STREAM IS PUT BACK INTO BLOCKING MODE EXPLICITLY.** POSIX
  /// does not have it inherit `O_NONBLOCK` from the listener and some platforms
  /// do it anyway, so a fixture that relies on either would read zero bytes and
  /// answer a request it never got -- silently, and only on one OS.
  fn serve_once(&self, mode: Behaviour) -> std::io::Result<bool>;
}

impl Listens for UnixListener {
  fn set_nonblocking(&self, on: bool) -> std::io::Result<()> {
    UnixListener::set_nonblocking(self, on)
  }
  fn serve_once(&self, mode: Behaviour) -> std::io::Result<bool> {
    match self.accept() {
      Ok((stream, _)) => {
        stream.set_nonblocking(false)?;
        stream.set_read_timeout(Some(SERVE_BUDGET))?;
        serve_probe(stream, mode)
      }
      Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(false),
      Err(e) => Err(e),
    }
  }
}

impl Listens for TcpListener {
  fn set_nonblocking(&self, on: bool) -> std::io::Result<()> {
    TcpListener::set_nonblocking(self, on)
  }
  fn serve_once(&self, mode: Behaviour) -> std::io::Result<bool> {
    match self.accept() {
      Ok((stream, _)) => {
        stream.set_nonblocking(false)?;
        stream.set_read_timeout(Some(SERVE_BUDGET))?;
        serve_probe(stream, mode)
      }
      Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(false),
      Err(e) => Err(e),
    }
  }
}

/// An endpoint that ACCEPTS and answers -- the only fixture that resembles a
/// running daemon.
///
/// **BEFORE THIS EXISTED, EVERY "LIVE" FIXTURE IN THIS FILE WAS A BARE
/// LISTENER, WHICH IS THE PHANTOM OF `AC-08.3` CASE 2 EXACTLY.** The suite was
/// green, and it could not have distinguished a live daemon from an inherited
/// listening fd, because its live fixture WAS one. Four tests flipped the
/// moment the probe became a round trip, which is what says the change did
/// something: a fixture that cannot exhibit the defect passes for free, and its
/// green is indistinguishable from a real one.
///
/// Shuts down on drop rather than detaching: a leaked thread holding a listener
/// is the fd-inheritance hazard FORK_GUARD exists for, so this must not create
/// one on the way out.
struct Responder {
  stop: Arc<AtomicBool>,
  thread: Option<std::thread::JoinHandle<()>>,
}

impl Responder {
  fn spawn<L: Listens>(listener: L) -> Self {
    Responder::spawn_as(listener, Behaviour::Answer)
  }

  fn spawn_as<L: Listens>(listener: L, mode: Behaviour) -> Self {
    listener
      .set_nonblocking(true)
      .expect("a responder polls, so its listener must be non-blocking");
    let stop = Arc::new(AtomicBool::new(false));
    let flag = Arc::clone(&stop);
    let thread = std::thread::spawn(move || {
      while !flag.load(Ordering::Relaxed) {
        // A probe that has already gone is not this fixture's problem; a
        // responder that dies on one reset connection would fail the NEXT
        // test's assertion, blaming the rule for a fixture that went away.
        if !listener.serve_once(mode).unwrap_or(false) {
          std::thread::sleep(POLL);
        }
      }
    });
    Responder {
      stop,
      thread: Some(thread),
    }
  }
}

impl Drop for Responder {
  fn drop(&mut self) {
    self.stop.store(true, Ordering::Relaxed);
    if let Some(thread) = self.thread.take() {
      let _ = thread.join();
    }
  }
}

/// A live unix endpoint: bound, accepting, and CHECKED to be answering.
///
/// The fixture verifies its own claim for the same reason [`stale_endpoint`]
/// does -- every assertion driven off it reads a routing RESULT, so a fixture
/// that is not live produces a failure blaming the rule.
fn live_unix(root: &Path) -> (Endpoint, Responder) {
  let path = socket_path(root);
  let guard = no_forks_here();
  let responder = Responder::spawn(UnixListener::bind(&path).expect("bind the live fixture"));
  drop(guard);
  let endpoint = Endpoint::Unix(path);
  assert!(
    endpoint.answers(),
    "the live-socket fixture is not answering: {endpoint}. Nothing downstream of this can be read as a routing result"
  );
  (endpoint, responder)
}

/// A live TCP endpoint on an ephemeral loopback port.
fn live_tcp() -> (Endpoint, Responder) {
  let guard = no_forks_here();
  let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).expect("bind ephemeral");
  let addr = listener.local_addr().expect("local_addr");
  let responder = Responder::spawn(listener);
  drop(guard);
  let endpoint = Endpoint::Tcp(addr);
  assert!(
    endpoint.answers(),
    "the live-TCP fixture is not answering: {endpoint}"
  );
  (endpoint, responder)
}

#[test]
fn an_absent_endpoint_runs_in_process() {
  let dir = tempfile::tempdir().expect("tempdir");
  let path = socket_path(dir.path());
  assert!(!path.exists(), "the fixture must start with no socket");

  assert_eq!(route_of(&[Endpoint::Unix(path)]), Route::InProcess);
}

#[test]
fn a_live_unix_endpoint_is_routed_to() {
  let dir = tempfile::tempdir().expect("tempdir");
  let (endpoint, _responder) = live_unix(dir.path());

  assert_eq!(route_of(&[endpoint.clone()]), Route::Daemon(endpoint));
}

/// The case the criterion names: the file is there and nobody is listening.
#[test]
fn a_stale_socket_file_runs_in_process() {
  let dir = tempfile::tempdir().expect("tempdir");
  assert_eq!(route_of(&[stale_endpoint(dir.path())]), Route::InProcess);
}

/// How long a probe gets to RETURN before this suite calls it a hang.
///
/// Twenty times `daemon::PROBE_DEADLINE`, because the number being checked is
/// not the deadline -- it is that a deadline exists at all. A bound tight
/// enough to measure the constant would flake on a loaded machine and would be
/// testing a value the rule is free to change.
const MUST_RETURN_WITHIN: Duration = Duration::from_secs(5);

/// Route a candidate list on another thread, so a probe that never returns
/// FAILS instead of hanging the suite.
///
/// **A TEST THAT ASSERTS "IT DOES NOT HANG" BY TIMING A DIRECT CALL CANNOT
/// FAIL -- IT HANGS TOO.** The one thing `AC-08.3` demands of the deadline is
/// that expiry ends the probe, and the direct-call form is exactly the
/// instrument that cannot observe its absence.
fn route_within(candidates: Vec<Endpoint>) -> Route {
  let (tx, rx) = mpsc::channel();
  std::thread::spawn(move || {
    let _ = tx.send(daemon::route(&candidates));
  });
  rx.recv_timeout(MUST_RETURN_WITHIN).unwrap_or_else(|_| {
    panic!(
      "the routing probe did not return within {MUST_RETURN_WITHIN:?}. A deadline that never expires is the outage the rule exists to prevent: every store verb blocks behind a daemon that is not there"
    )
  })
}

/// Ask ONE endpoint whether it answers, on another thread, for the same reason
/// [`route_within`] exists: a direct call cannot fail when the answer is a hang.
fn answers_within(endpoint: Endpoint) -> bool {
  let (tx, rx) = mpsc::channel();
  std::thread::spawn(move || {
    let _ = tx.send(endpoint.answers());
  });
  rx.recv_timeout(MUST_RETURN_WITHIN)
    .unwrap_or_else(|_| panic!("the liveness probe did not return within {MUST_RETURN_WITHIN:?}"))
}

/// A forked child holding every descriptor this process had open, accepting
/// nothing.
///
/// **REAPED ON DROP, INCLUDING ON PANIC, AND THAT IS NOT TIDINESS.** A forked
/// child inherits the test binary's STDOUT PIPE. If an assertion panics before
/// the child is killed, the orphan holds the write end open forever and `cargo
/// test` blocks reading it -- so **a failing test becomes a hung build**, with
/// no output naming which test failed. Found by mutating the read deadline
/// away: the probe hung, the panic fired ahead of the kill, and the mutation
/// battery stalled instead of reporting a kill. A `Drop` guard is the only
/// form that survives the panic path, which is exactly the path that matters.
struct ForkedChild(libc::pid_t);

impl ForkedChild {
  fn holding_everything() -> Self {
    // SAFETY: the child calls only `pause` and `_exit`, both async-signal-safe.
    // Nothing else may go there -- this process is multi-threaded, so any
    // allocation or lock in the child can deadlock against a mutex another
    // thread held at the instant of the fork.
    let pid = unsafe { libc::fork() };
    assert!(
      pid >= 0,
      "fork failed, so AC-08.3 case 2 cannot be constructed"
    );
    if pid == 0 {
      unsafe {
        libc::pause();
        libc::_exit(0);
      }
    }
    ForkedChild(pid)
  }
}

impl Drop for ForkedChild {
  fn drop(&mut self) {
    unsafe {
      libc::kill(self.0, libc::SIGKILL);
      libc::waitpid(self.0, std::ptr::null_mut(), 0);
    }
  }
}

/// `AC-08.3` CASE 2, deterministic form: bound, listening, never accepting.
///
/// **THE SAME OBSERVABLE AS THE INHERITED FD BELOW, WITH NONE OF THE RACE.** A
/// client cannot tell a phantom holding a leaked listening descriptor from a
/// listener whose owner simply never calls `accept` -- both are a socket file
/// on disk, a `connect` that succeeds into the backlog, and a reply that never
/// comes. So this is the case that can be asserted every run, and the fork
/// below is the case that proves the construction faithful.
#[test]
fn a_listener_that_never_accepts_runs_in_process() {
  let dir = tempfile::tempdir().expect("tempdir");
  let path = socket_path(dir.path());
  let guard = no_forks_here();
  let listener = UnixListener::bind(&path).expect("bind the phantom fixture");
  drop(guard);

  let endpoint = Endpoint::Unix(path.clone());
  assert!(
    path.exists(),
    "the phantom fixture has no socket file, so it is testing absence rather than a dead listener"
  );

  assert_eq!(
    route_within(vec![endpoint]),
    Route::InProcess,
    "a listener that never accepts is not a daemon that can serve a request. A connect succeeded and nothing answered, which is what `exists and answers` rules out"
  );
  drop(listener);
}

/// `AC-08.3` CASE 2 as the criterion words it: a child holds the listening
/// descriptor and the owner is gone.
///
/// **WHAT IS REPRODUCED IS THE OBSERVABLE, NOT THE RACE, AND THE DIFFERENCE IS
/// WORTH STATING.** In the field the leak happens because macOS has no
/// `SOCK_CLOEXEC`, so a `fork`+`exec` between `socket()` and the `FD_CLOEXEC`
/// that follows it inherits the listener -- a 1-in-300 event this suite must
/// never sit waiting for. Here the `fork` is deliberate and there is no
/// `exec`, which inherits the descriptor unconditionally. **The client sees the
/// identical thing either way**: a socket file, an open backlog, no acceptor.
/// That is the whole population the routing rule has to survive.
///
/// The parent cannot die -- it is the test process -- so what dies is the
/// listener it owns. The child then holds the socket open with its owner gone,
/// which is the state the criterion names.
#[test]
fn an_inherited_listening_fd_runs_in_process() {
  let dir = tempfile::tempdir().expect("tempdir");
  let path = socket_path(dir.path());

  let guard = no_forks_here();
  let listener = UnixListener::bind(&path).expect("bind before the fork");
  let _child = ForkedChild::holding_everything();
  drop(listener);
  drop(guard);

  let alive = path.exists();
  let endpoint = Endpoint::Unix(path);
  let answered = answers_within(endpoint.clone());
  let routed = route_within(vec![endpoint]);

  assert!(
    alive,
    "the socket file did not outlive the parent's listener, so this fixture is testing absence and not an inherited descriptor"
  );
  assert!(
    !answered,
    "an endpoint whose only owner is a child that never accepts reported itself as ANSWERING. This is the false positive AC-08.3 case 2 names: a successful connect is not an answer"
  );
  assert_eq!(
    routed,
    Route::InProcess,
    "a daemon whose listening fd leaked into a surviving child is not a daemon. Routing to it strands every store verb at an endpoint with no acceptor"
  );
}

/// The two phantoms are INDISTINGUISHABLE, and the rule must treat them so.
///
/// **THIS IS WHY THE DETERMINISTIC TWIN IS ALLOWED TO STAND IN FOR THE RACE.**
/// If a leaked descriptor and a never-accepting listener could route
/// differently, the cheap fixture would be proving something about itself
/// rather than about the case the criterion names. Asserting they agree is what
/// makes the substitution honest rather than convenient.
#[test]
fn the_two_phantoms_route_identically() {
  let dir = tempfile::tempdir().expect("tempdir");
  let path = socket_path(dir.path());
  let guard = no_forks_here();
  let held = UnixListener::bind(&path).expect("bind the never-accepting fixture");
  drop(guard);
  let never_accepts = route_within(vec![Endpoint::Unix(path)]);
  drop(held);

  let forked_dir = tempfile::tempdir().expect("tempdir");
  let forked_path = socket_path(forked_dir.path());
  let guard = no_forks_here();
  let listener = UnixListener::bind(&forked_path).expect("bind before the fork");
  let _child = ForkedChild::holding_everything();
  drop(listener);
  drop(guard);
  let inherited = route_within(vec![Endpoint::Unix(forked_path)]);

  assert_eq!(
    never_accepts, inherited,
    "the two constructions of AC-08.3 case 2 routed differently, so the deterministic one is not a stand-in for the race and the criterion is only half covered"
  );
  assert_eq!(never_accepts, Route::InProcess);
}

/// The third phantom: something ACCEPTS and closes without answering.
///
/// **NEITHER OF THE CRITERION'S TWO CASES REACHES THIS, AND IT IS THE ONE THE
/// `read` RETURN VALUE DECIDES.** A stale file fails at connect and an
/// inherited descriptor times out; this one comes back promptly with a clean
/// EOF -- `Ok(0)`, a success. A probe that asks whether the read ERRORED
/// treats it as a live daemon and routes every store verb at a socket that is
/// on its way down.
#[test]
fn an_endpoint_that_accepts_and_closes_runs_in_process() {
  let dir = tempfile::tempdir().expect("tempdir");
  let path = socket_path(dir.path());
  let guard = no_forks_here();
  let listener = UnixListener::bind(&path).expect("bind the closing fixture");
  let _responder = Responder::spawn_as(listener, Behaviour::AcceptAndClose);
  drop(guard);

  assert_eq!(
    route_within(vec![Endpoint::Unix(path)]),
    Route::InProcess,
    "an endpoint that accepts and closes without answering is not serving anything. `Ok(0)` from the probe read is a clean EOF, not a reply"
  );
}

#[test]
fn a_live_tcp_endpoint_is_routed_to() {
  let (endpoint, _responder) = live_tcp();

  assert_eq!(route_of(&[endpoint.clone()]), Route::Daemon(endpoint));
}

#[test]
fn a_closed_tcp_port_runs_in_process() {
  assert_eq!(route_of(&[Endpoint::Tcp(closed_port())]), Route::InProcess);
}

#[test]
fn no_candidates_runs_in_process() {
  assert_eq!(route_of(&[]), Route::InProcess);
}

/// The routing body is driven across BOTH transports, in both orders.
///
/// A rule that folds over a list is only transport-agnostic if something makes
/// it fold over more than one kind of thing. Production hands it a single unix
/// candidate today, so without this test the multi-transport body would never
/// have run.
#[test]
fn the_first_answering_candidate_wins_and_the_body_names_no_transport() {
  let dir = tempfile::tempdir().expect("tempdir");

  // **THE FIXTURES CHECK THEMSELVES BEFORE THEY ARE USED, AND THAT IS NOT
  // CEREMONY.** Every assertion below reads a fold's RESULT, so a fixture that
  // is not in the state its name claims produces a failure blaming the fold --
  // the one part that is certainly innocent. `live_unix` / `live_tcp` /
  // `stale_endpoint` each assert their own liveness, in one place, for all
  // their callers.
  let (live_unix, _unix_responder) = live_unix(dir.path());
  let (live_tcp, _tcp_responder) = live_tcp();

  let stale_dir = tempfile::tempdir().expect("tempdir");
  let stale_unix = stale_endpoint(stale_dir.path());
  let dead_tcp = Endpoint::Tcp(closed_port());

  // A dead candidate of one transport must not stop a live one of the other
  // from being reached, in either direction.
  assert_eq!(
    route_of(&[dead_tcp.clone(), live_unix.clone()]),
    Route::Daemon(live_unix.clone()),
    "a closed TCP port ahead of a live socket must not end the fold"
  );
  assert_eq!(
    route_of(&[stale_unix.clone(), live_tcp.clone()]),
    Route::Daemon(live_tcp.clone()),
    "a stale socket ahead of a live TCP port must not end the fold"
  );

  // FIRST answering wins, so two live candidates resolve by position and not
  // by any preference the body holds about transports.
  assert_eq!(
    route_of(&[live_unix.clone(), live_tcp.clone()]),
    Route::Daemon(live_unix.clone())
  );
  assert_eq!(
    route_of(&[live_tcp.clone(), live_unix.clone()]),
    Route::Daemon(live_tcp)
  );

  // And a list of nothing but dead candidates, of both kinds, is in-process.
  assert_eq!(route_of(&[stale_unix, dead_tcp]), Route::InProcess);
}

/// Named so a failure reads as the rule rather than as a call.
fn route_of(candidates: &[Endpoint]) -> Route {
  daemon::route(candidates)
}

/// The rule is WIRED, driven through the shipped binary.
///
/// Three runs of one command against one fixture project, differing only in
/// whether a daemon is listening at the address the binary itself resolves.
#[test]
fn the_shipped_cli_routes_on_a_live_socket_and_not_otherwise() {
  let project = project();
  let home = tempfile::tempdir().expect("tempdir");
  let socket = socket_path(home.path());

  let run = || {
    // Held across the spawn: a `fork` here can leak a sibling test's listening
    // fd into the child and keep a released socket answering. See FORK_GUARD.
    let _no_forks = no_forks_here();
    let out = Command::new(env!("CARGO_BIN_EXE_intent"))
      .args(["st", "list"])
      .current_dir(project.path())
      .env("HOME", home.path())
      .output()
      .expect("run intent");
    (
      out.status.code().unwrap_or(-1),
      String::from_utf8_lossy(&out.stderr).trim_end().to_string(),
    )
  };

  let absent = run();
  assert!(
    !absent.1.contains("intentd is answering"),
    "with no socket the CLI must not report a daemon; got: {}",
    absent.1
  );

  let (_endpoint, responder) = live_unix(home.path());
  let routed = run();
  drop(responder);
  fs::remove_file(&socket).expect("remove the fixture socket");

  let after = run();

  assert_eq!(
    routed.0, 2,
    "a live daemon owns the store and this build has no client, which is exit 2 -- the build cannot answer -- rather than a verdict about the user's project. Got {} with: {}",
    routed.0, routed.1
  );
  assert!(
    routed.1.contains(&socket.display().to_string()),
    "the refusal must name the endpoint the CLI found, so an operator can tell WHICH daemon; got: {}",
    routed.1
  );
  // **NOT A SEVENTH COPY OF THE UNWIRED MARKER.** That sentence already lives
  // in six constants under five names across this suite, and the fix is one
  // `pub const` rather than another alias. This is a short fragment asserting a
  // property -- the routed refusal must not read as an unbuilt verb, because a
  // gate arm keyed on that marker would otherwise report every store verb on a
  // daemon machine as unimplemented.
  assert!(
    !routed.1.contains("not implemented yet"),
    "the routing refusal must not read as an unwired verb: the command IS built and the store IS reachable, by something else. Got: {}",
    routed.1
  );

  assert_eq!(
    after, absent,
    "removing the socket must restore the earlier behaviour exactly. If these differ, the middle run changed something and the comparison above was not measuring the socket"
  );
}

/// The candidate list is derived from what the daemon PUBLISHED (hv's D6).
///
/// No port constant exists to test, which is the point: the daemon binds
/// `127.0.0.1:0` and writes what it got, so these drive a FILE.
#[test]
fn the_candidate_list_is_read_from_the_daemons_published_address() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  let addr_file = userstate::daemon_address_file_under(root);
  std::fs::create_dir_all(addr_file.parent().expect("parent")).expect("mkdir");

  // Nothing published: the socket is the whole list. Absence is a STATE.
  let bare = daemon::candidates_under(root).expect("no address file is not an error");
  assert_eq!(
    bare,
    vec![Endpoint::Unix(userstate::daemon_socket_under(root))],
    "with no address file the list is the socket alone"
  );

  // Published: the socket FIRST -- it carries its own authz in filesystem
  // permissions, where a loopback port is reachable by anything on the box.
  std::fs::write(&addr_file, "127.0.0.1:54321\n").expect("write address");
  let published = daemon::candidates_under(root).expect("a readable address is not an error");
  assert_eq!(
    published,
    vec![
      Endpoint::Unix(userstate::daemon_socket_under(root)),
      Endpoint::Tcp("127.0.0.1:54321".parse().expect("parse")),
    ],
    "a published address adds a TCP candidate AFTER the socket"
  );

  // **UNREADABLE IS AN ERROR, NOT AN ABSENCE**, and this is the assertion that
  // separates them. Dropping the candidate would shorten the list, and a
  // shorter list runs in-process while a daemon holds the store -- the one
  // outcome the routing rule exists to prevent.
  std::fs::write(&addr_file, "not-an-address").expect("write address");
  let refused = daemon::candidates_under(root);
  assert!(
    refused.is_err(),
    "an address file that cannot be parsed must refuse, not silently drop the candidate; got {refused:?}"
  );
}

/// The shipped binary reads the published address, not a constant.
#[test]
fn the_shipped_cli_refuses_an_address_it_cannot_read() {
  let project = project();
  let home = tempfile::tempdir().expect("tempdir");
  let addr_file = userstate::daemon_address_file_under(home.path());
  std::fs::create_dir_all(addr_file.parent().expect("parent")).expect("mkdir");

  let run = || {
    let _no_forks = no_forks_here();
    let out = Command::new(env!("CARGO_BIN_EXE_intent"))
      .args(["st", "list"])
      .current_dir(project.path())
      .env("HOME", home.path())
      .output()
      .expect("run intent");
    (
      out.status.code().unwrap_or(-1),
      String::from_utf8_lossy(&out.stderr).trim_end().to_string(),
    )
  };

  let baseline = run();

  std::fs::write(&addr_file, "obviously not an address").expect("write address");
  let refused = run();
  assert_ne!(
    refused, baseline,
    "a garbage address file must change the answer -- if it does not, the shipped binary is not reading it and this test proves nothing about production"
  );
  assert!(
    refused.1.contains(&addr_file.display().to_string()),
    "the refusal must name the file the operator has to fix; got: {}",
    refused.1
  );

  std::fs::remove_file(&addr_file).expect("remove address");
  assert_eq!(
    run(),
    baseline,
    "removing the address file must restore the earlier behaviour exactly"
  );
}

/// Every `.rs` file under a directory.
fn sources(dir: &Path) -> Vec<PathBuf> {
  let mut out = Vec::new();
  let mut stack = vec![dir.to_path_buf()];
  while let Some(next) = stack.pop() {
    for entry in fs::read_dir(&next).unwrap_or_else(|e| panic!("read {}: {e}", next.display())) {
      let path = entry.expect("dir entry").path();
      if path.is_dir() {
        stack.push(path);
      } else if path.extension().is_some_and(|e| e == "rs") {
        out.push(path);
      }
    }
  }
  out.sort();
  out
}

/// Two sync engines never run concurrently, enforced where it can be.
///
/// **THE RULE IS ABOUT THE INVOCATION, SO ONE UNGUARDED CONSTRUCTION SITE DOES
/// NOT WEAKEN IT -- IT DELETES IT FOR WHATEVER GOES THROUGH THAT SITE.** And it
/// would do so silently, and only on a machine running a daemon, which is the
/// one place nobody develops. The guarded door here is spelled exactly like its
/// unguarded twin, so nothing about reaching for the wrong one feels like a
/// decision; that is why this is mechanical rather than a convention.
#[test]
fn the_in_process_engine_has_exactly_one_door() {
  const DOOR: &str = "fn engine(";
  const CALL: &str = "Facade::open(";

  let src = testkit::workspace_root()
    .join("crates")
    .join("intent-cli")
    .join("src");
  let files = sources(&src);

  // A walk that found nothing agrees with every rule ever written, silently.
  assert!(
    files.len() > 3,
    "the source walk found only {} files under {} -- a broken walk passes this test vacuously",
    files.len(),
    src.display()
  );

  let mut sites = Vec::new();
  for file in &files {
    let code = fs::read_to_string(file).unwrap_or_else(|e| panic!("read {}: {e}", file.display()));
    let mut enclosing = "<none>".to_string();
    for (n, line) in code.lines().enumerate() {
      if line.starts_with("fn ") || line.starts_with("pub fn ") {
        enclosing = line.trim_end_matches(" {").to_string();
      }
      // Comment lines are skipped so the module's own prose may NAME the call
      // without counting as one -- the same allowance `dep_graph_guard` makes
      // for a manifest that documents the rule it obeys.
      if line.trim_start().starts_with("//") {
        continue;
      }
      if line.contains(CALL) {
        let shown = file.file_name().expect("file name").to_string_lossy();
        sites.push(format!("{shown}:{} in `{enclosing}`", n + 1));
      }
    }
  }

  assert_eq!(
    sites.len(),
    1,
    "the in-process engine must be constructed in exactly one place, and every other verb must reach it through `{DOOR}` -- that is where the routing rule is applied, and a second site skips it. Found: {sites:?}"
  );
  // Proving the scanner as well as the code: a scanner that finds nothing
  // would satisfy the count above by being broken.
  assert!(
    sites[0].contains(DOOR),
    "the one construction site is not inside `{DOOR}`, so it is not behind the routing rule: {}",
    sites[0]
  );
}
