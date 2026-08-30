//! `AC-08.11` obligation TWO: the daemon's listeners are close-on-exec, and the
//! window it cannot close is narrowed as far as the platform allows.
//!
//! **THE OBLIGATION IS REAL AND THE CODE THAT MEETS IT MAY NOT BE OURS, WHICH
//! IS PRECISELY WHY IT NEEDS A WITNESS.** macOS has no atomic `SOCK_CLOEXEC`,
//! so a listening descriptor is created by `socket()` and marked close-on-exec
//! by a second syscall; a `fork`+`exec` landing between them leaks the listener
//! into a child that may outlive its owner. `cli_routing.rs` is the client's
//! half -- it survives the leak by requiring a completed round trip rather than
//! a successful connect. This file is the daemon's half: the window is narrowed
//! at the source.
//!
//! **A PROPERTY ENFORCED BY SOMETHING OUTSIDE YOUR CODE CANNOT BE WITNESSED BY
//! TESTING ITS OUTCOME.** Rust's standard library sets the flag itself, on the
//! syscall after the socket is created, which is as tight as the platform
//! permits and tighter than anything this crate could add afterwards. That
//! makes the OUTCOME hold under any implementation that still goes through
//! `std` -- so a test that forked a child and watched what it inherited would
//! pass with every line of our own hygiene deleted. What is testable, and what
//! these tests assert, is THE FLAG ITSELF on the descriptor the constructor
//! hands back.
//!
//! **WHAT THAT ACTUALLY GUARDS AGAINST IS NOT MALICE.** It is somebody
//! replacing a `std` bind with a hand-rolled `socket()` + `bind()` -- to set
//! `SO_REUSEADDR`, to pass a pre-opened descriptor, to support an abstract
//! socket -- and reasonably not thinking about a flag `std` had been setting
//! for them. That change compiles, passes every routing test, and reopens the
//! leak. These two assertions are what go red on it.

use std::os::fd::AsRawFd;

use intentsvcs::daemon::{Bound, Published};

/// Is this descriptor marked close-on-exec?
///
/// Reads the flag rather than inferring it from behaviour. `F_GETFD` returning
/// a negative value is a failure to MEASURE and is reported as one -- an
/// unreadable flag must never arrive here as an absent one, which is the same
/// absent-versus-unreadable split the address reader carries.
fn is_close_on_exec(fd: std::os::fd::RawFd) -> bool {
  let flags = unsafe { libc::fcntl(fd, libc::F_GETFD) };
  assert!(
    flags >= 0,
    "F_GETFD failed on fd {fd}, so this test measured nothing and must not be read as a pass: {}",
    std::io::Error::last_os_error()
  );
  flags & libc::FD_CLOEXEC != 0
}

#[test]
fn the_unix_listener_the_daemon_binds_is_close_on_exec() {
  let dir = tempfile::tempdir().expect("tempdir");
  let (listener, _bound) =
    Bound::bind_socket_under(dir.path()).expect("bind the daemon's unix socket");
  assert!(
    is_close_on_exec(listener.as_raw_fd()),
    "the daemon's unix listener is not FD_CLOEXEC. A fork+exec anywhere in the process now leaks the LISTENING descriptor into a child, and if that child outlives the daemon the socket keeps accepting with nobody behind it -- AC-08.3 case 2, which the client survives and which the daemon owes narrowing"
  );
}

#[test]
fn the_loopback_listener_the_daemon_binds_is_close_on_exec() {
  let dir = tempfile::tempdir().expect("tempdir");
  let (listener, _published) =
    Published::bind_loopback_under(dir.path()).expect("bind the daemon's loopback port");
  assert!(
    is_close_on_exec(listener.as_raw_fd()),
    "the daemon's TCP listener is not FD_CLOEXEC. The leak is worse on this transport than on the socket: a leaked unix listener is reachable only through a path the daemon unlinks, while a leaked port stays reachable by every process on the machine"
  );
}

#[test]
fn the_probe_would_be_answered_by_a_leaked_descriptor_which_is_why_this_matters() {
  // **THE CONTROL THAT SHOWS THE FLAG IS LOAD-BEARING RATHER THAN TIDY.** Both
  // assertions above are about a bit. This one is about what the bit prevents:
  // a descriptor with `FD_CLOEXEC` CLEARED survives `exec`, and this drives
  // that difference on a real descriptor rather than describing it.
  //
  // It does NOT fork -- the point is the flag's semantics, not a race -- and it
  // restores the flag it clears, so nothing downstream inherits the fixture.
  let dir = tempfile::tempdir().expect("tempdir");
  let (listener, _bound) = Bound::bind_socket_under(dir.path()).expect("bind");
  let fd = listener.as_raw_fd();

  assert!(is_close_on_exec(fd), "baseline: the flag starts set");

  let cleared = unsafe { libc::fcntl(fd, libc::F_SETFD, 0) };
  assert_eq!(
    cleared, 0,
    "clearing FD_CLOEXEC must succeed for this control to mean anything"
  );
  assert!(
    !is_close_on_exec(fd),
    "the mutation must actually change the descriptor before either verdict counts"
  );

  let restored = unsafe { libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) };
  assert_eq!(restored, 0, "restore must succeed");
  assert!(
    is_close_on_exec(fd),
    "the fixture is left exactly as it was found"
  );
}
