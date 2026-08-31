//! **`RealDaemon::restart` MOVES THE PROCESS AND NOT THE ADDRESS, AND BOTH
//! HALVES ARE ASSERTED HERE.**
//!
//! ic needs this for `AC-09.3` -- *bridge mode survives a daemon restart
//! mid-session via per-request target resolution*. The harness capability is
//! cc's seam, so the harness is driven here rather than inside the row's own
//! test: **an API landed with no driver is an API whose first exercise is
//! someone else's red**, and the failure would arrive attributed to their row.
//!
//! **THE ADDRESS ARM IS THE ONE THAT MATTERS, BECAUSE THE OBVIOUS WITNESS IS
//! FALSE BY CONSTRUCTION.** `userstate::daemon_socket_under` is
//! `<home>/.local/share/intent/intentd.sock` -- no pid, no port, no nonce -- so
//! a restart under the same home yields the IDENTICAL endpoint. A test written
//! as *the daemon moved, so its endpoint changed* asserts something that cannot
//! be true, and would pass or fail for reasons unrelated to its subject. This
//! file pins the unchanged address deliberately so the next reader meets the
//! fact before they design against it.

mod common;

use common::RealDaemon;

#[test]
fn a_restart_replaces_the_process_and_keeps_the_address() {
  let daemon = RealDaemon::start();
  let before_pid = daemon.pid();
  let before_home = daemon.home().to_path_buf();
  let before_endpoint = daemon
    .endpoint()
    .expect("a daemon that answered `Op::Registry` has an endpoint");

  let daemon = daemon.restart();

  // **THE PROCESS MOVED.** `restart` asserts this internally too; asserting it
  // again here is not redundant -- an internal assertion proves the harness
  // noticed, and this proves the harness EXPOSES what it noticed, which is the
  // half ic's row consumes.
  assert_ne!(
    before_pid,
    daemon.pid(),
    "the pid is unchanged, so nothing restarted and the witness is inert"
  );

  // **AND THE ADDRESS DID NOT.** This is the arm that stops a future test
  // reaching for the wrong observable.
  assert_eq!(
    before_home,
    daemon.home(),
    "restart must reuse the home -- a new home makes it a fresh daemon rather than a restart, \
     and the routing question the row asks disappears"
  );
  assert_eq!(
    Some(before_endpoint),
    daemon.endpoint(),
    "the socket path is derived from HOME alone, so a restart cannot move it -- if this ever \
     fails, `pid()` is no longer the only witness and AC-09.3's test may use the endpoint"
  );
}

/// **THE NEW DAEMON SERVES, RATHER THAN MERELY EXISTING.**
///
/// `restart` already waits on `Op::Registry` before returning, so this arm
/// looks redundant -- and it is the same shape as a guard on the wrong side of
/// the wire. The wait proves the harness waited; this proves a caller reaching
/// the returned value gets a daemon that answers, which is what ic's bridge
/// call will actually do. **A readiness wait that returned early would leave
/// the first arm green and this one red**, which is the only way to tell them
/// apart.
#[test]
fn the_restarted_daemon_answers_a_real_op() {
  let daemon = RealDaemon::start().restart();
  daemon.wait_until_it_answers_a_real_op();
  assert!(
    daemon.endpoint().is_some(),
    "the restarted daemon is not routable, so `restart` returned before it was serving"
  );
}
