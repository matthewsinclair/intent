//! `AC-08.4`: **the running daemon's pid comes from the lock, and only from a
//! lock somebody else is holding.**
//!
//! `intent daemon stop` asks over the socket first and falls back to `SIGTERM`,
//! because a wedged daemon will not answer its own socket -- which is exactly
//! when stopping it matters most. Signalling needs a pid, and `AC-08.12`'s lock
//! is where it lives, because a lock cannot go stale the way a pid file can.
//!
//! **THE INTERESTING ARM IS THE WINDOW, NOT THE HAPPY PATH** (vc, 2026-08-30).
//! Holding the lock proves A WRITER is alive; it does not prove the pid in the
//! file is. Between one daemon dying and the next writing its pid, a reader
//! sees a held lock over stale or empty content -- and a stale pid may name a
//! stranger the kernel has recycled. **The daemon truncates under the lock
//! before it writes**, so that window shows an empty file, and this test is
//! what says an empty file must be REFUSED rather than rounded down to "no
//! daemon is running".

use std::io::Write;

use intentsvcs::daemon::{self, DaemonError};
use intentsvcs::userstate;

fn root() -> tempfile::TempDir {
  tempfile::tempdir().expect("tempdir")
}

/// Take the lock the way a daemon does, and keep holding it.
///
/// **A SECOND `File` ON THE SAME PATH IN THIS PROCESS IS A REAL CONFLICT**, not
/// a simulation: `flock` is per open file description, so this contends with
/// the reader exactly as another process would. That is what makes a
/// same-process fixture honest here.
fn hold(path: &std::path::Path) -> std::fs::File {
  std::fs::create_dir_all(path.parent().expect("a parent")).expect("state dir");
  let lock = std::fs::File::options()
    .create(true)
    .read(true)
    .write(true)
    .open(path)
    .expect("open the lock");
  lock
    .try_lock()
    .expect("nothing else holds this fixture's lock");
  lock
}

#[test]
fn no_lock_file_at_all_is_no_daemon() {
  let home = root();
  assert_eq!(
    daemon::running_pid_under(home.path()).expect("absence is a state, not an error"),
    None,
    "a root that never ran a daemon reported one"
  );
}

#[test]
fn a_lock_nobody_holds_is_no_daemon_whatever_it_says() {
  let home = root();
  let path = userstate::daemon_lock_under(home.path());
  std::fs::create_dir_all(path.parent().expect("a parent")).expect("state dir");

  // A pid left behind by a daemon that died. **The content is not the
  // question**: what makes it a corpse is that nobody holds the lock.
  std::fs::write(&path, "424242").expect("write a stale pid");

  assert_eq!(
    daemon::running_pid_under(home.path()).expect("read"),
    None,
    "a stale pid in an unheld lock was reported as a running daemon, which is the pid-file defect the lock exists to remove"
  );
}

#[test]
fn a_held_lock_publishes_the_holders_pid() {
  let home = root();
  let path = userstate::daemon_lock_under(home.path());
  let mut lock = hold(&path);
  lock.set_len(0).expect("truncate");
  write!(lock, "{}", std::process::id()).expect("write the pid");
  lock.flush().expect("flush");

  assert_eq!(
    daemon::running_pid_under(home.path()).expect("read"),
    Some(std::process::id()),
    "a held lock carrying a pid did not report it"
  );
}

/// **THE WINDOW, AND THE ONLY ARM HERE ABOUT A DEFECT.**
///
/// A held lock over an empty file is a daemon that has acquired and not yet
/// published. **`None` would be a lie** -- it says no daemon is running about a
/// machine where one demonstrably is, and the caller starts a second. A guess
/// would be worse: there is nothing to guess from, and anything invented gets
/// delivered to a process by number.
#[test]
fn a_held_lock_with_no_pid_yet_is_refused_rather_than_reported_as_absent() {
  let home = root();
  let path = userstate::daemon_lock_under(home.path());
  let _lock = hold(&path);

  match daemon::running_pid_under(home.path()) {
    Err(DaemonError::UnpublishedPid { found, .. }) => {
      assert!(
        found.trim().is_empty(),
        "the refusal reported content that is not empty: {found:?}"
      );
    }
    other => panic!(
      "a daemon holding the lock without a published pid must be refused, never reported as absent or guessed at: {other:?}"
    ),
  }
}

/// **A SHORT READ IS A VALID PID BELONGING TO A STRANGER** (vc's sharpest point
/// on this). `12` out of `12345` parses, so a reader that merely required
/// parseability would signal an unrelated process. This arm exists because the
/// refusal above must not be implemented as "refuse if empty".
#[test]
fn a_partial_pid_is_refused_even_though_it_parses() {
  let home = root();
  let path = userstate::daemon_lock_under(home.path());
  let mut lock = hold(&path);
  lock.set_len(0).expect("truncate");
  write!(lock, "12 34").expect("write something parseable-looking");
  lock.flush().expect("flush");

  assert!(
    matches!(
      daemon::running_pid_under(home.path()),
      Err(DaemonError::UnpublishedPid { .. })
    ),
    "content that is not a single whole pid was accepted"
  );
}
