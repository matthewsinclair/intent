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
use std::net::{Ipv4Addr, SocketAddr, TcpListener};
use std::os::unix::net::UnixListener;
use std::path::{Path, PathBuf};
use std::process::Command;

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
  let path = socket_path(dir.path());
  let _listener = UnixListener::bind(&path).expect("bind the fixture socket");

  let endpoint = Endpoint::Unix(path);
  assert_eq!(route_of(&[endpoint.clone()]), Route::Daemon(endpoint));
}

/// The case the criterion names: the file is there and nobody is listening.
#[test]
fn a_stale_socket_file_runs_in_process() {
  let dir = tempfile::tempdir().expect("tempdir");
  assert_eq!(route_of(&[stale_endpoint(dir.path())]), Route::InProcess);
}

#[test]
fn a_live_tcp_endpoint_is_routed_to() {
  let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).expect("bind ephemeral");
  let endpoint = Endpoint::Tcp(listener.local_addr().expect("local_addr"));

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

  let unix_path = socket_path(dir.path());
  let _unix = UnixListener::bind(&unix_path).expect("bind the fixture socket");
  let live_unix = Endpoint::Unix(unix_path);

  let tcp = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).expect("bind ephemeral");
  let live_tcp = Endpoint::Tcp(tcp.local_addr().expect("local_addr"));

  let stale_dir = tempfile::tempdir().expect("tempdir");
  let stale_unix = stale_endpoint(stale_dir.path());
  let dead_tcp = Endpoint::Tcp(closed_port());

  // **THE FIXTURES ARE CHECKED BEFORE THEY ARE USED, AND THAT IS NOT
  // CEREMONY.** Every assertion below reads a fold's RESULT, so a fixture that
  // is not in the state its name claims produces a failure blaming the fold --
  // the one part that is certainly innocent. Asking each endpoint directly
  // costs four connects and makes a broken fixture say so in its own words.
  assert!(
    live_unix.answers(),
    "the live socket fixture is not answering: {live_unix}"
  );
  assert!(
    live_tcp.answers(),
    "the live TCP fixture is not answering: {live_tcp}"
  );

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

  let listener = UnixListener::bind(&socket).expect("bind the fixture socket");
  let routed = run();
  drop(listener);
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
