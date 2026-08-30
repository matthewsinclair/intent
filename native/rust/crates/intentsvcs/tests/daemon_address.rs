//! `AC-08.4` / hv's **D6**: the daemon publishes where it bound, and the
//! publication does not outlive it.
//!
//! **THE WRITER AND THE READER ARE PROVEN TO AGREE, NEVER ASSUMED TO.**
//! `daemon::candidates_under` parses the address file and `Published` writes
//! it, and they are two homes for one format. So the first test here does not
//! inspect the file at all -- it publishes, then asks the READER what it found,
//! which is the only check that fails when either side drifts.
//!
//! **THE WRITER AND THE PROBE ARE DESIGNED AGAINST EACH OTHER ON PURPOSE.**
//! `Published` tries never to leave a stale address; `Endpoint::answers`
//! assumes it did anyway. Neither is redundant: `SIGKILL` runs no destructor,
//! so a design where the reader trusted the writer's discipline would be one
//! hard kill from an outage.

use std::net::SocketAddr;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

use intentsvcs::daemon::{self, Endpoint, Published, Route};
use intentsvcs::userstate;

fn address_file(root: &Path) -> std::path::PathBuf {
  userstate::daemon_address_file_under(root)
}

/// The published address, as the SHIPPED READER sees it.
///
/// Deliberately not `read_to_string` plus a parse: that would be a third home
/// for the format, and a test carrying its own parser cannot notice the two
/// real ones disagreeing.
fn tcp_candidate(root: &Path) -> Option<SocketAddr> {
  daemon::candidates_under(root)
    .expect("a published address must be readable")
    .into_iter()
    .find_map(|e| match e {
      Endpoint::Tcp(addr) => Some(addr),
      Endpoint::Unix(_) => None,
    })
}

#[test]
fn what_the_daemon_publishes_is_what_the_router_reads() {
  let dir = tempfile::tempdir().expect("tempdir");
  assert_eq!(
    tcp_candidate(dir.path()),
    None,
    "the fixture must start with nothing published, or it proves nothing about publishing"
  );

  let (listener, published) = Published::bind_loopback_under(dir.path()).expect("bind and publish");
  let bound = listener.local_addr().expect("local_addr");

  assert_eq!(
    tcp_candidate(dir.path()),
    Some(bound),
    "the router read a different address from the one the listener is bound to. The writer and the parser are two homes for one format and they have drifted"
  );
  assert_eq!(published.endpoint(), Endpoint::Tcp(bound));
}

/// D6's whole point: no port constant exists, so the port cannot be predicted.
#[test]
fn the_port_comes_from_the_kernel_and_not_from_a_constant() {
  let a = tempfile::tempdir().expect("tempdir");
  let b = tempfile::tempdir().expect("tempdir");
  let (_la, pa) = Published::bind_loopback_under(a.path()).expect("bind a");
  let (_lb, pb) = Published::bind_loopback_under(b.path()).expect("bind b");

  let (Endpoint::Tcp(aa), Endpoint::Tcp(bb)) = (pa.endpoint(), pb.endpoint()) else {
    panic!("a loopback publication must be a TCP endpoint");
  };
  assert_ne!(
    aa.port(),
    bb.port(),
    "two daemons got the same port, so something is choosing one rather than asking the kernel"
  );
  assert!(aa.ip().is_loopback() && bb.ip().is_loopback());
}

#[test]
fn the_publication_does_not_outlive_the_daemon() {
  let dir = tempfile::tempdir().expect("tempdir");
  let (listener, published) = Published::bind_loopback_under(dir.path()).expect("bind and publish");
  assert!(address_file(dir.path()).exists());

  drop(published);
  drop(listener);

  assert!(
    !address_file(dir.path()).exists(),
    "the address file survived the daemon. Every client would now be routed at a port nobody is listening on"
  );
  assert_eq!(tcp_candidate(dir.path()), None);
}

/// **The failure path is the one that must still clean up.**
#[test]
fn a_panicking_daemon_still_withdraws_its_address() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path().to_path_buf();

  let caught = std::panic::catch_unwind({
    let root = root.clone();
    move || {
      let (_listener, _published) =
        Published::bind_loopback_under(&root).expect("bind and publish");
      assert!(userstate::daemon_address_file_under(&root).exists());
      panic!("the daemon dies here");
    }
  });
  assert!(caught.is_err(), "the fixture must actually panic");

  assert!(
    !address_file(&root).exists(),
    "an address published by a process that panicked was left behind. Cleanup written after the work is dead code on exactly the path that needs it"
  );
}

/// vc's catch, constructed: an unconditional remove deletes a LIVE peer's claim.
///
/// **THE INVERSE FAILURE, AND THE WORSE DIRECTION.** A stale address is a false
/// positive the probe already refuses; a MISSING address is a false negative no
/// probe can correct, because there is nothing left to probe.
#[test]
fn a_departing_daemon_does_not_withdraw_a_live_peers_address() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();

  let (_first_listener, first) = Published::bind_loopback_under(root).expect("bind a");
  let (_second_listener, second) = Published::bind_loopback_under(root).expect("bind b");

  // B published second, so the file is B's.
  assert_eq!(
    tcp_candidate(root),
    match second.endpoint() {
      Endpoint::Tcp(addr) => Some(addr),
      Endpoint::Unix(_) => None,
    },
    "the fixture is not in the state it claims: the file does not hold the second daemon's address"
  );

  drop(first);

  assert_eq!(
    tcp_candidate(root),
    match second.endpoint() {
      Endpoint::Tcp(addr) => Some(addr),
      Endpoint::Unix(_) => None,
    },
    "a departing daemon deleted a LIVE daemon's published address. The file now says nothing is running while something is, and no probe can discover it"
  );
}

/// A hard kill leaves a stale address, and the probe is what makes it harmless.
#[test]
fn an_address_left_by_a_hard_kill_still_routes_as_absent() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();

  let (listener, published) = Published::bind_loopback_under(root).expect("bind and publish");
  let addr = listener.local_addr().expect("local_addr");
  // `SIGKILL` runs no destructor: the file stays, the listener does not.
  std::mem::forget(published);
  drop(listener);

  assert!(
    address_file(root).exists(),
    "the fixture must leave the address behind, or it is not testing a hard kill"
  );
  assert_eq!(
    tcp_candidate(root),
    Some(addr),
    "the stale address must still be OFFERED as a candidate -- the routing rule's job is to reject it on liveness, not to fail to see it"
  );

  let candidates = daemon::candidates_under(root).expect("candidates");
  assert_eq!(
    daemon::route(&candidates),
    Route::InProcess,
    "a published address whose owner is gone routed as a daemon. This is AC-08.3's whole subject, reached through the writer rather than through a fixture"
  );
}

/// The publication REPLACES the file rather than rewriting it in place.
///
/// **THIS WITNESSES THE MECHANISM, NOT THE OUTCOME, AND THAT IS THE ONLY THING
/// AVAILABLE TO WITNESS** (vc, 2026-08-30). Atomicity here belongs to
/// `rename(2)`, not to our code: a reader CANNOT observe a torn write because
/// the syscall forbids it, so no concurrency fixture would ever fail -- not
/// today, and not after somebody replaced the temp-and-rename with a direct
/// truncating open, which is the regression that would silently remove the
/// property. A race window too small to hit reliably is not a test.
///
/// **THE INODE IS THE DISCRIMINATOR AND IT IS EXACT.** `rename` swings the
/// directory entry at a different file, so the path's inode CHANGES; a
/// truncating write opens the existing file and keeps it. One number separates
/// the two implementations, deterministically, with no timing anywhere.
#[test]
fn republishing_swings_the_file_rather_than_rewriting_it() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();

  let (_first_listener, first) = Published::bind_loopback_under(root).expect("bind a");
  let before = std::fs::metadata(address_file(root))
    .expect("published")
    .ino();

  let (_second_listener, second) = Published::bind_loopback_under(root).expect("bind b");
  let after = std::fs::metadata(address_file(root))
    .expect("republished")
    .ino();

  assert_ne!(
    first.endpoint(),
    second.endpoint(),
    "the fixture needs two DIFFERENT addresses, or the republish it is measuring is a no-op"
  );
  assert_ne!(
    before, after,
    "the address file kept its inode across a republish, so it was rewritten IN PLACE rather than renamed over. Atomicity here is rename(2)'s and nothing else's -- a reader can now see a half-written address, and AC-08.3's parser REFUSES one, so every concurrent command becomes a hard error"
  );
}

// ---------------------------------------------------------------------------
// The socket's twin of the above. `Bound` guards the unix socket file the way
// `Published` guards the address, and differs exactly where the mechanism does.
// ---------------------------------------------------------------------------

use intentsvcs::daemon::{Bound, DaemonError};
use std::io::{Read, Write};
use std::os::unix::net::UnixListener;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// A socket that ACCEPTS and answers -- the only fixture a live daemon
/// resembles, and the lesson `cli_routing.rs` paid for: a bare listener that
/// never accepts is the PHANTOM, not the live case.
struct Answering {
  stop: Arc<AtomicBool>,
  thread: Option<std::thread::JoinHandle<()>>,
}

impl Answering {
  fn on(listener: UnixListener) -> Self {
    listener.set_nonblocking(true).expect("non-blocking");
    let stop = Arc::new(AtomicBool::new(false));
    let flag = Arc::clone(&stop);
    let thread = std::thread::spawn(move || {
      while !flag.load(Ordering::Relaxed) {
        match listener.accept() {
          Ok((mut s, _)) => {
            let _ = s.set_nonblocking(false);
            let _ = s.set_read_timeout(Some(std::time::Duration::from_millis(500)));
            let mut buf = [0u8; 64];
            if matches!(s.read(&mut buf), Ok(n) if n > 0) {
              let _ = s.write_all(b"{\"ok\":true}\n");
            }
          }
          _ => std::thread::sleep(std::time::Duration::from_millis(1)),
        }
      }
    });
    Answering {
      stop,
      thread: Some(thread),
    }
  }
}

impl Drop for Answering {
  fn drop(&mut self) {
    self.stop.store(true, Ordering::Relaxed);
    if let Some(t) = self.thread.take() {
      let _ = t.join();
    }
  }
}

#[test]
fn binding_creates_the_socket_and_dropping_removes_it() {
  let dir = tempfile::tempdir().expect("tempdir");
  let socket = userstate::daemon_socket_under(dir.path());
  assert!(!socket.exists(), "the fixture must start with no socket");

  let (listener, bound) = Bound::bind_socket_under(dir.path()).expect("bind");
  assert!(socket.exists());
  assert_eq!(bound.endpoint(), Endpoint::Unix(socket.clone()));

  drop(bound);
  drop(listener);
  assert!(
    !socket.exists(),
    "the socket outlived the daemon, making AC-08.3 case 1 this daemon's own doing rather than a crash"
  );
}

/// The restart-after-crash case: a corpse must not block every future start.
#[test]
fn a_stale_socket_is_cleared_rather_than_blocking_the_restart() {
  let dir = tempfile::tempdir().expect("tempdir");
  let socket = userstate::daemon_socket_under(dir.path());
  std::fs::create_dir_all(socket.parent().expect("parent")).expect("mkdir");
  // A crash: the file outlives the listener, and `bind` refuses a path that
  // exists, so without this one crash makes every future start impossible.
  drop(UnixListener::bind(&socket).expect("bind the corpse"));
  assert!(socket.exists(), "the fixture is not stale");

  let (_l, _b) = Bound::bind_socket_under(dir.path()).expect("a corpse must not block a restart");
  assert!(socket.exists());
}

/// A daemon that IS answering is not evicted.
#[test]
fn a_live_daemon_is_refused_rather_than_evicted() {
  let dir = tempfile::tempdir().expect("tempdir");
  let (listener, first) = Bound::bind_socket_under(dir.path()).expect("bind the first daemon");
  let _answering = Answering::on(listener);
  assert!(
    first.endpoint().answers(),
    "the fixture's first daemon is not answering, so this would exercise the stale path instead"
  );

  match Bound::bind_socket_under(dir.path()) {
    Err(DaemonError::AlreadyRunning { .. }) => {}
    Err(other) => panic!("a live daemon must be refused as AlreadyRunning, got: {other}"),
    Ok(_) => panic!(
      "a second daemon EVICTED a live one. It now serves every client while the first holds a listener no path reaches"
    ),
  }
}

/// Compare-and-unlink, by identity rather than by content.
#[test]
fn an_evicted_daemon_does_not_unlink_its_successors_socket() {
  let dir = tempfile::tempdir().expect("tempdir");
  let socket = userstate::daemon_socket_under(dir.path());

  // The first never accepts, so it reads as a corpse and is evicted.
  let (first_listener, first) = Bound::bind_socket_under(dir.path()).expect("bind first");
  let (_second_listener, second) = Bound::bind_socket_under(dir.path()).expect("evict the corpse");
  assert!(socket.exists());

  drop(first);
  drop(first_listener);

  assert!(
    socket.exists(),
    "the evicted daemon unlinked its SUCCESSOR's socket on the way out, leaving a live daemon unreachable -- the false negative no probe can correct"
  );
  assert_eq!(second.endpoint(), Endpoint::Unix(socket));
}
