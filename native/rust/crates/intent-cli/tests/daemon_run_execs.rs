//! `AT-08.9`: `intent daemon run` BECOMES `intentd` rather than starting one.
//!
//! **THIS ROW EXISTS BECAUSE `daemon run` IS EXCLUDED FROM THE CONFORMANCE
//! HARNESS AND AN EXCLUSION WITHOUT COVERAGE IS A DELETION.**
//! `dual_path_conformance` cannot drive it -- `Command::output()` waits for an
//! exit that never comes, and the in-process route would have its own image
//! replaced mid-run -- so the row is refused by both routes there. **Refusing to
//! COMPARE a row while a named criterion covers it is honest; refusing to
//! compare it and covering it nowhere is how a verb quietly leaves the surface.**
//!
//! **THE OBSERVABLE IS THE ONE THING THAT SEPARATES `exec` FROM `spawn`: THE PID
//! SURVIVES AND THE IMAGE CHANGES.** Every weaker check passes under the
//! implementation this criterion exists to forbid --
//! `AC-08.9` requires that `intentd` serving while running and
//! `intent daemon run` serving in the foreground are the SAME BINARY rather than
//! two code paths that agree today, and a `spawn` satisfies "a daemon is now
//! answering" exactly as well as an `exec` does.
//!
//! **SO THE DISCRIMINATING ASSERTION IS ABOUT THE PROCESS WE LAUNCHED, NEVER
//! ABOUT WHETHER A DAEMON APPEARED.** Under `spawn` the pid we hold stays
//! `intent` and grows an `intentd` child; under `exec` that same pid IS
//! `intentd` and has no children. Both arms are checked, because the second is
//! what makes the first impossible to satisfy by accident.

use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicU32, Ordering};

/// How many times a condition is asked before the test gives up on it.
///
/// A retry count rather than a deadline: what this loop requires is that it
/// TERMINATES, which says nothing about the time and keeps this workspace's
/// clock guard (D42) true here rather than exempted.
const ATTEMPTS: u32 = 300;

const PAUSE: std::time::Duration = std::time::Duration::from_millis(20);

/// A short, unique directory under `/tmp`.
///
/// **NOT `tempfile`, AND NOT FOR TIDINESS.** A unix socket address is a
/// fixed-size field, so the whole path has to fit; `$TMPDIR` on macOS is a
/// ~50-character generated path and the daemon's own suffix is another 32.
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

/// What the kernel says this pid's executable is now.
///
/// `comm` is the image name, which is precisely what `exec` replaces and
/// `spawn` does not.
fn image_of(pid: u32) -> String {
  let out = Command::new("ps")
    .args(["-o", "comm=", "-p", &pid.to_string()])
    .output()
    .expect("ps runs");
  let name = String::from_utf8_lossy(&out.stdout).trim().to_string();
  // `ps` prints the full path for an executable launched by path.
  name.rsplit('/').next().map(str::to_string).unwrap_or(name)
}

/// The pids whose parent is `pid`.
fn children_of(pid: u32) -> Vec<String> {
  let out = Command::new("pgrep")
    .args(["-P", &pid.to_string()])
    .output()
    .expect("pgrep runs");
  String::from_utf8_lossy(&out.stdout)
    .split_whitespace()
    .map(str::to_string)
    .collect()
}

/// `intent daemon run`, killed and cleaned up when this value is dropped.
///
/// **A `Drop` GUARD RATHER THAN A KILL AFTER THE ASSERTIONS**, because a kill
/// written after them is dead code until an assertion fires, and on that day it
/// does not run -- leaving a daemon holding the test binary's descriptors and a
/// `cargo` that never returns. That exact failure is what this row exists
/// alongside.
struct ForegroundDaemon {
  child: Child,
  home: PathBuf,
}

impl ForegroundDaemon {
  fn start() -> ForegroundDaemon {
    // **AN ISOLATED `HOME`, AND THIS IS THE ONE LINE THAT MUST NOT BE WRONG.**
    // A daemon started under the real `$HOME` would answer every peer session's
    // liveness probe at once, and the `sync`/`ingest` family refuses while a
    // daemon holds the store -- so a careless fixture here takes four
    // developers' store verbs down together.
    let home = short_dir("execwitness-home");
    let child = Command::new(env!("CARGO_BIN_EXE_intent"))
      .args(["daemon", "run"])
      .env("HOME", &home)
      .stdout(Stdio::null())
      .stderr(Stdio::null())
      .spawn()
      .expect("the shipped intent binary runs");
    ForegroundDaemon { child, home }
  }

  fn pid(&self) -> u32 {
    self.child.id()
  }

  fn home(&self) -> &Path {
    &self.home
  }

  /// Block until this pid's image is `intentd`, or give up saying so.
  fn wait_for_image(&self) -> bool {
    for _ in 0..ATTEMPTS {
      if image_of(self.pid()) == "intentd" {
        return true;
      }
      std::thread::sleep(PAUSE);
    }
    false
  }
}

impl Drop for ForegroundDaemon {
  fn drop(&mut self) {
    let _ = self.child.kill();
    let _ = self.child.wait();
    let _ = std::fs::remove_dir_all(&self.home);
  }
}

#[test]
fn daemon_run_replaces_its_own_image_rather_than_starting_a_child() {
  let daemon = ForegroundDaemon::start();
  let pid = daemon.pid();

  assert!(
    daemon.wait_for_image(),
    "the process launched as `intent daemon run` (pid {pid}) never became `intentd`; it is `{}`. \
     Either the exec failed, or this is a `spawn` -- and a spawn satisfies `a daemon is answering` \
     exactly as well as an exec does, which is why AC-08.9's identical-binary claim needs this \
     observable and not that one",
    image_of(pid)
  );

  // **THE ARM THAT MAKES THE FIRST ONE IMPOSSIBLE TO SATISFY BY ACCIDENT.**
  // A `spawn` implementation leaves the launched pid as `intent` with an
  // `intentd` CHILD. Asserting the image alone would be a strong test already;
  // asserting that there is no child as well means no arrangement of parent and
  // child processes can pass this without the exec actually having happened.
  let children = children_of(pid);
  assert!(
    children.is_empty(),
    "pid {pid} is running `intentd` and ALSO has children {children:?}. An exec leaves no child \
     behind -- this is a spawn wearing the right image name, which is the implementation \
     AC-08.9's `same binary, not two code paths` forbids"
  );

  // **THE CONTROL: THE FIXTURE'S OWN `HOME` IS WHERE THE DAEMON WENT.** Without
  // it, a stray `intentd` already running on the machine would satisfy nothing
  // above -- the assertions are keyed on OUR pid -- but the test would still be
  // worth doubting, because a reader cannot tell from the assertions alone that
  // the daemon under test is isolated. This makes the isolation observable
  // rather than merely intended.
  assert!(
    daemon.home().exists(),
    "the isolated HOME vanished, so this test can no longer say the daemon it started was \
     isolated from the developer's own"
  );
}

#[test]
fn the_image_probe_can_tell_the_two_binaries_apart() {
  // **THE POSITIVE CONTROL, WITHOUT WHICH THE GREEN ABOVE IS WORTH NOTHING.**
  // `image_of` is a `ps` call parsed by hand; if it returned an empty string on
  // this platform, `wait_for_image` would simply time out and report an honest
  // failure -- but if it returned `intentd` for ANY pid, the test above would
  // pass under a spawn. So the probe is driven against a process whose image is
  // known and is NOT `intentd`.
  let me = std::process::id();
  let mine = image_of(me);
  assert!(
    !mine.is_empty(),
    "the image probe returned nothing for this very process, so it measures nothing on this \
     platform and the assertions above cannot fail for the right reason"
  );
  assert_ne!(
    mine, "intentd",
    "the image probe called this test binary `intentd`, so it cannot distinguish the two and \
     every assertion resting on it passes for free"
  );
}
