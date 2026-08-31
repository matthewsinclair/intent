//! `ST0064` `AC-01.6`: the three endpoint states an operator's SURFACE shows,
//! and the ruling that the split is earned by the REMEDY rather than by the
//! vocabulary.
//!
//! **THE PROHIBITION DOES NOT FORCE THE ROW, WHICH IS WHY THIS FILE EXISTS.**
//! `AC-01.6` forbids rendering `RUNNING` for an endpoint that connects without
//! answering, and BOTH candidate discriminators satisfy that -- the kernel lock
//! and a bare `connect()` alike. So a build cannot be read as compliant merely
//! because its vocabulary matches the criterion's sentence. What must be shown
//! is that STALE and ABSENT carry DIFFERENT REMEDIES, and the case that
//! separates them is the orphaned listening descriptor.
//!
//! **THE SIX-CASE MEASUREMENT THIS WAS RULED ON (cc, 2026-08-31).** The two
//! readings agree on five cases and disagree on exactly one:
//!
//! | case                                | lock | conn | ans | LOCK   | CONNECT |
//! |-------------------------------------|------|------|-----|--------|---------|
//! | live daemon                         | y    | y    | y   | LIVE   | LIVE    |
//! | daemon hung / SIGSTOP               | y    | y    | n   | STALE  | STALE   |
//! | bound, no accept loop               | y    | y    | n   | STALE  | STALE   |
//! | hard-killed, socket file left       | n    | n    | n   | ABSENT | ABSENT  |
//! | **orphaned listening fd, parent dead** | n | y    | n   | **ABSENT** | **STALE** |
//! | nothing ever ran                    | n    | n    | n   | ABSENT | ABSENT  |
//!
//! **vc RULED IT ONTO `ABSENT`, AND THE GROUND IS THE REMEDY.** An orphan has
//! no holder to investigate -- the parent is dead and the kernel released the
//! lock -- so calling it stale would declare a remedy nobody can carry out.
//! `STALE` means *a holder is alive and not serving*, whose remedy is to
//! investigate that pid and NOT to unlink (`AC-08.12`, where being wrong is
//! destructive rather than wasteful).
//!
//! **CONSTRUCTED, NEVER WAITED FOR** (`AC-08.3`). The orphan case is the
//! 1-in-300 race, and a suite that passes because a race did not fire has
//! measured nothing. It is built here as its DETERMINISTIC EQUIVALENT: a
//! listener that accepts and never answers while holding no lock is the same
//! observable as an inherited descriptor whose parent has died, which is the
//! criterion's own reasoning for keying on the observable rather than on the
//! mechanism.

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixListener;

use intentsvcs::daemon::{self, Bound, Health};
use intentsvcs::userstate;

/// A listener on `listener` that answers the probe exactly as the daemon must.
///
/// Takes an already-bound listener rather than a path, so the same helper
/// serves the orphan case (bound directly, no lock) and the live case (bound
/// through `Bound`, lock held) -- **which is what lets those two arms differ in
/// the ONE variable under test.**
fn answer_on(listener: UnixListener) {
  std::thread::spawn(move || {
    let Ok((stream, _)) = listener.accept() else {
      return;
    };
    let mut reader = BufReader::new(&stream);
    let mut line = Vec::new();
    if reader.read_until(b'\n', &mut line).is_err() {
      return;
    }
    if daemon::is_probe_frame(&line) {
      let mut out = &stream;
      let _ = out.write_all(daemon::PROBE_REPLY);
      let _ = out.flush();
    }
  });
}

#[test]
fn nothing_running_is_absent() {
  let dir = tempfile::tempdir().expect("tempdir");
  assert_eq!(
    daemon::health_under(dir.path()).expect("health"),
    Health::Absent,
    "an estate where no daemon has ever run must read as ABSENT"
  );
}

#[test]
fn a_holder_that_does_not_answer_is_stale() {
  let dir = tempfile::tempdir().expect("tempdir");
  let (_listener, bound) = Bound::bind_socket_under(dir.path()).expect("bind");
  assert!(
    !bound.endpoint().answers(),
    "this fixture needs a holder that is NOT answering, or it tests the live case under another name"
  );

  match daemon::health_under(dir.path()).expect("health") {
    Health::Stale { pid } => assert_eq!(
      pid,
      std::process::id(),
      "STALE must name the pid an operator is being told to investigate, and this fixture's holder is this process"
    ),
    other => panic!(
      "a lock-holding, non-answering daemon read as {other:?} rather than STALE. The operator is told nothing is running while a process holds their socket"
    ),
  }
}

#[test]
fn a_daemon_that_answers_is_live_even_though_it_holds_the_lock() {
  // **THE ORDER GUARD, AND IT IS THE ONE THAT FAILS TOTALLY RATHER THAN
  // RARELY.** A live daemon holds the lock AND answers, so a lock-first
  // projection reports EVERY healthy daemon as stale. That is the only reason
  // such a regression would be noticed at all -- which is precisely why it is
  // pinned here rather than left to the obviousness of the code.
  let dir = tempfile::tempdir().expect("tempdir");
  let (listener, bound) = Bound::bind_socket_under(dir.path()).expect("bind");
  let endpoint = bound.endpoint();
  answer_on(listener);

  assert_eq!(
    daemon::health_under(dir.path()).expect("health"),
    Health::Live(endpoint),
    "a daemon that holds the lock and answers the probe must be LIVE. Consulting the lock before the round trip renders every healthy daemon STALE"
  );
}

#[test]
fn an_orphaned_listener_with_no_holder_is_absent_and_not_stale() {
  // **THE RULED CASE.** Bound DIRECTLY rather than through `Bound`, so the
  // socket accepts connections while NO lock is held -- an inherited listening
  // descriptor whose parent has died, in its deterministic form.
  let dir = tempfile::tempdir().expect("tempdir");
  let path = userstate::daemon_socket_under(dir.path());
  std::fs::create_dir_all(path.parent().expect("socket has a parent")).expect("mkdir");
  let listener = UnixListener::bind(&path).expect("bind the orphan");

  // The half that makes this the case it claims to be: it CONNECTS.
  assert!(
    std::os::unix::net::UnixStream::connect(&path).is_ok(),
    "the fixture is not an orphaned LISTENER if nothing can connect to it -- it would be testing the hard-killed case instead"
  );

  assert_eq!(
    daemon::health_under(dir.path()).expect("health"),
    Health::Absent,
    "an orphaned listening descriptor read as STALE. There is no holder to investigate and nothing to leave alone, so STALE would declare a remedy that cannot be carried out -- and this is the one case the two candidate discriminators disagree on"
  );
  drop(listener);
}

#[test]
fn the_two_non_live_states_differ_in_what_they_give_an_operator() {
  // **THE REMEDY DIFFERENCE, ASSERTED RATHER THAN DESCRIBED** -- `AC-01.6`'s
  // actual requirement. A split whose two sides carry the same information is
  // a vocabulary change, and this arm is what stops the projection degenerating
  // into one while every other arm above stays green.
  let absent_dir = tempfile::tempdir().expect("tempdir");
  let stale_dir = tempfile::tempdir().expect("tempdir");
  let (_listener, _bound) = Bound::bind_socket_under(stale_dir.path()).expect("bind");

  let absent = daemon::health_under(absent_dir.path()).expect("health");
  let stale = daemon::health_under(stale_dir.path()).expect("health");

  assert_ne!(
    absent, stale,
    "the two non-live states are indistinguishable, so the projection has three names and two answers"
  );
  assert!(
    matches!(stale, Health::Stale { .. }),
    "the stale fixture did not produce STALE, so this comparison proves nothing"
  );
  assert!(
    matches!(absent, Health::Absent),
    "the absent fixture did not produce ABSENT, so this comparison proves nothing"
  );
}
