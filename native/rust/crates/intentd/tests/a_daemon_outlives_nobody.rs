//! `AT-01.1` / `AT-01.3` / `AT-02.1` (ST0073): **a daemon whose owner dies stops
//! by itself, and a daemon with no owner keeps serving.**
//!
//! # THE TWO ARMS ARE ONE DECISION READ IN BOTH DIRECTIONS
//!
//! They live in one file deliberately. The lifeline arm proves the leak is
//! fixed; the no-lifeline arm proves production is untouched. **An
//! implementation that exited when NO lifeline was passed would sail through
//! the first arm and silently kill the `launchd` daemon** -- whose plist is
//! `RunAtLoad true`, `KeepAlive false`, with no `Sockets` key, so there is no
//! socket activation and it would not come back until the next login. The
//! symptom would be "the daemon is sometimes not running" and nothing in it
//! would name this change. Separating the arms invites one to be run without
//! the other.
//!
//! # THE OWNER IS A REAL PROCESS AND IT IS KILLED WITH `SIGKILL`
//!
//! **A test that terminated its parent politely would measure the path that
//! already works.** Nine test files in this workspace carry an `impl Drop` and
//! every one of them is correct; the leak happened anyway, because `Drop` runs
//! on a normal return or an unwinding panic and NOT on an interrupted
//! `cargo test`, a killed build, a Ctrl-C or a crashed TUI. `SIGKILL` is the
//! one signal no destructor, handler or `atexit` survives, so it is the only
//! kill that exercises the case the lifeline exists for.
//!
//! # WHAT IS ASSERTED IS THE PROCESS, NOT THE SOCKET
//!
//! The arm waits for the daemon to be REAPED, by pid. A check that the address
//! stopped answering would pass against a wedged process still holding the
//! store -- which is the exact defect this thread exists to remove, so the
//! weaker assertion would be satisfied by the failure.

use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;

use intentsvcs::daemon::{self, Route};

const ATTEMPTS: u32 = 300;
const PAUSE: Duration = Duration::from_millis(50);

/// A `HOME` of this test's own, so nothing here can reach the developer's
/// daemon or be reached by a concurrent session's.
fn isolated_home(tag: &str) -> PathBuf {
  static NEXT: AtomicU32 = AtomicU32::new(0);
  // Short, because a unix socket address is a fixed-size field and `$TMPDIR` on
  // macOS is a ~50-character generated path.
  // **AT START, NEVER AT EXIT.** The `Drop` below removes this directory on the
  // happy path; the path that produced 902 abandoned homes in `/tmp` is the one
  // where the binary is killed and no `Drop` runs. Sweeping here cleans up the
  // PREVIOUS run's corpses, which is the only ordering that survives this
  // process being killed too. Idempotent per process.
  testkit::sweep_once();
  let dir = PathBuf::from("/tmp").join(format!(
    "intent-fixture-{tag}-{}-{}",
    std::process::id(),
    NEXT.fetch_add(1, Ordering::Relaxed)
  ));
  std::fs::create_dir_all(&dir).expect("create an isolated HOME");
  dir
}

/// Block until the SHIPPED routing predicate says a daemon is answering here.
///
/// **IT WAITS ON THE THING UNDER TEST, NOT ON A SLEEP.** A fixed sleep is
/// either too short on a loaded machine -- a flake that reads as a lifeline
/// defect -- or too long on every other run.
fn wait_until_answering(home: &Path) {
  for _ in 0..ATTEMPTS {
    if let Ok(c) = daemon::candidates_under(home)
      && matches!(daemon::route(&c), Route::Daemon(_))
    {
      return;
    }
    std::thread::sleep(PAUSE);
  }
  panic!(
    "intentd never answered under HOME={}, so every assertion below would be about a daemon that was never there",
    home.display()
  );
}

/// A child this test will reap even if an assertion fires first.
///
/// **THIS FILE LEAKED A DAEMON ON ITS FIRST RUN, WHICH IS THE DEFECT IT IS
/// ABOUT.** An arm asserted, panicked, and the `kill` written after the
/// assertion never ran -- leaving a real `intentd` on `/tmp` with `ppid=1`.
/// **A test for a leak that leaks on its failure path is not a test, and the
/// failure path is the one that matters**: a green run cleans up either way.
struct Reaped(Child);

impl Reaped {
  fn id(&self) -> u32 {
    self.0.id()
  }

  fn running(&mut self) -> bool {
    self.0.try_wait().expect("wait on a child").is_none()
  }

  /// Wait, bounded, for the child to be reaped. `None` means it outlived it.
  fn wait_for_exit(&mut self) -> Option<std::process::ExitStatus> {
    for _ in 0..ATTEMPTS {
      match self.0.try_wait() {
        Ok(Some(status)) => return Some(status),
        Ok(None) => std::thread::sleep(PAUSE),
        Err(e) => panic!("could not wait on the child: {e}"),
      }
    }
    None
  }

  fn kill_now(&mut self) {
    let _ = self.0.kill();
    let _ = self.0.wait();
  }
}

impl Drop for Reaped {
  fn drop(&mut self) {
    self.kill_now();
  }
}

#[test]
fn invariant_a_daemon_whose_owner_is_killed_stops_by_itself() {
  let home = isolated_home("lifeline-owned");

  // **THE OWNER IS A SEPARATE PROCESS HOLDING THE WRITE END, WHICH IS WHAT
  // MAKES THIS TEST ABOUT A PARENT DEATH AT ALL.** If this test process held
  // the write end it could only close it by running code -- which is the thing
  // that does not happen when a build is killed. `sleep` never writes to its
  // stdout, so the pipe carries no data and closes only when the process ends.
  let (reader, writer) = std::io::pipe().expect("a pipe");
  let mut owner = Reaped(
    Command::new("sleep")
      .arg("300")
      .stdout(Stdio::from(writer))
      .spawn()
      .expect("sleep runs"),
  );

  let mut daemon_proc = Reaped(
    Command::new(env!("CARGO_BIN_EXE_intentd"))
      .env("HOME", &home)
      .stdin(Stdio::from(reader))
      .stdout(Stdio::null())
      .stderr(Stdio::null())
      .spawn()
      .expect("intentd is built beside this test by cargo"),
  );

  // Both ends are now owned by the two children; this process holds neither, so
  // nothing this test does can close the lifeline except killing the owner.
  wait_until_answering(&home);

  // **ANTI-VACUITY: IT MUST STILL BE RUNNING BEFORE THE OWNER DIES.** Without
  // this the arm below passes against a daemon that crashed at startup for an
  // unrelated reason, and would report a lifeline that does not work as one
  // that does.
  assert!(
    daemon_proc.running(),
    "intentd had already exited before its owner was killed, so this arm is measuring a crash rather than a lifeline"
  );

  // SIGKILL: no destructor, no handler, no atexit. The kernel closes the write
  // end because the process is gone, and for no other reason.
  owner.kill_now();

  let status = daemon_proc.wait_for_exit().unwrap_or_else(|| {
    panic!(
      "THE DAEMON OUTLIVED ITS OWNER. Its owner was SIGKILLed and it is still running under HOME={} -- which is the leak this thread exists to remove: 64 processes on one machine by 2026-09-10, 64.9 CPU-hours between them. The lifeline is not reaching the serve loop.",
      home.display()
    )
  });
  assert!(
    status.success(),
    "the daemon noticed its owner and did not exit cleanly: {status}. A lifeline that ends the process without unwinding leaves the stale socket the guards exist to prevent"
  );

  let _ = std::fs::remove_dir_all(&home);
}

#[test]
fn invariant_a_daemon_with_no_lifeline_serves_until_signalled() {
  let home = isolated_home("lifeline-none");

  // **THIS IS THE PRODUCTION PATH: `launchd` sets no environment of ours and
  // hands the daemon `/dev/null` on stdin.** `/dev/null` reads EOF instantly,
  // so an implementation that treated stdin as a lifeline unconditionally would
  // die here on its first poll -- which is precisely the silent breakage this
  // arm exists to catch.
  // LIFELINE-EXEMPT: this arm exists to spawn a daemon with NO lifeline -- it
  // is the production control, and `Stdio::null()` is what launchd hands one.
  // Arming it would delete the only test of the supervised path. It is reaped
  // by `Reaped` and by its own SIGTERM below.
  let mut daemon_proc = Reaped(
    Command::new(env!("CARGO_BIN_EXE_intentd"))
      .env("HOME", &home)
      .stdin(Stdio::null())
      .stdout(Stdio::null())
      .stderr(Stdio::null())
      .spawn()
      .expect("intentd is built beside this test by cargo"),
  );

  wait_until_answering(&home);

  // Long enough that an EOF-on-stdin implementation has had every chance to
  // act. It is not a race: the failing implementation exits in microseconds.
  std::thread::sleep(Duration::from_millis(750));
  assert!(
    daemon_proc.running(),
    "AN UNSUPERVISED DAEMON STOPPED ON ITS OWN. With no INTENT_DAEMON_LIFELINE set, intentd must serve until signalled -- this is launchd's daemon, whose plist is KeepAlive false with no socket activation, so an exit here means it does not come back until the next login and nothing in the failure names this code."
  );

  // **THE POSITIVE CONTROL ON THE ASSERTION ABOVE, AND WITHOUT IT THAT
  // ASSERTION IS DECORATION.** `try_wait().is_none()` is also what a broken
  // observation returns. Sending a signal this daemon IS specified to honour
  // proves this test can see an exit when there is one to see.
  let signalled = Command::new("kill")
    .arg("-TERM")
    .arg(daemon_proc.id().to_string())
    .status()
    .expect("kill runs");
  assert!(signalled.success(), "could not signal the daemon");
  assert!(
    daemon_proc.wait_for_exit().is_some(),
    "the daemon did not exit on SIGTERM, so this test cannot observe an exit at all and its 'still running' assertion above establishes nothing"
  );

  let _ = std::fs::remove_dir_all(&home);
}

/// `AT-02.1` (`ST0073` `AC-02.1`): **a daemon whose own state directory is
/// removed stops serving.**
///
/// # THE CONFOUND THAT MAKES THE OBVIOUS VERSION OF THIS TEST WORTHLESS
///
/// The first drive of this subject was a shell probe holding a lifeline FIFO,
/// and **the FIFO cannot live inside the home.** Put it there -- which is the
/// obvious placement, and where the next person will reach for it -- and
/// removing the home closes the pipe, the daemon exits ON THE LIFELINE, and the
/// run reads as a clean pass while proving nothing about state directories at
/// all.
///
/// **THIS ARM REMOVES THE CONFOUND RATHER THAN MANAGING IT: the daemon is
/// started SUPERVISED, with no lifeline of any kind.** `Stdio::null()` is what
/// `launchd` hands one, so there is no owner, no pipe and nothing for an EOF to
/// arrive on. An exit here cannot be the lifeline's doing because there is no
/// lifeline. That is stronger than holding a pipe and asserting it is still
/// held, and it needs no assertion to stay true.
///
/// # THE POSITIVE CONTROL IS THE ROW'S OWN DEMAND
///
/// `AC-02.1` says it in as many words: *with a positive control that the same
/// daemon was answering immediately before the removal -- without that control
/// the arm passes against a daemon that never started.* `wait_until_answering`
/// is that control, and it waits on the shipped routing predicate rather than
/// on a sleep.
///
/// # MEASURED BEFORE IT WAS BUILT
///
/// Driven 2026-09-10 against the build that had no state-directory arm: the
/// daemon ANSWERED, its home was removed, and it was **still alive 8 seconds
/// later**. So this row had a demonstrated subject before any code was written
/// for it, which is the difference between a test and decoration.
#[test]
fn invariant_a_daemon_whose_state_directory_is_removed_stops_by_itself() {
  let home = isolated_home("statedir-removed");

  // LIFELINE-EXEMPT, AND THE EXEMPTION IS THE POINT OF THE ARM: a lifeline here
  // would be the alternative explanation for the exit this test asserts.
  let mut daemon_proc = Reaped(
    Command::new(env!("CARGO_BIN_EXE_intentd"))
      .env("HOME", &home)
      .stdin(Stdio::null())
      .stdout(Stdio::null())
      .stderr(Stdio::null())
      .spawn()
      .expect("intentd is built beside this test by cargo"),
  );

  wait_until_answering(&home);

  std::fs::remove_dir_all(&home).expect("remove the daemon's state directory");

  assert!(
    daemon_proc.wait_for_exit().is_some(),
    "A DAEMON WHOSE STATE DIRECTORY WAS REMOVED KEPT RUNNING. It cannot serve -- its socket lived inside that tree, so the path a client would connect to is gone -- and it is still holding the store, which makes it a concurrent writer nothing can reach and nothing will stop until the run ends."
  );
}

/// THE CONTROL FOR THE ARM ABOVE, AND ITS BUDGET IS WHAT MAKES IT DISTINCT.
///
/// **"The daemon exited" is also what a daemon that exits for any other reason
/// looks like**, so an arm that only removes the directory and sees an exit
/// cannot attribute it. This one changes exactly one variable -- the directory
/// stays -- and requires the daemon to still be there.
///
/// **IT IS NOT A SECOND HOME FOR `invariant_a_daemon_with_no_lifeline_serves_
/// until_signalled`, AND THE DIFFERENCE IS THE DEADLINE.** That arm waits 750ms,
/// which is chosen against an EOF-on-stdin implementation that would die in
/// microseconds. The state-directory check needs two consecutive misses on a
/// two-second period, so its worst case is four seconds: 750ms establishes
/// nothing about it, and this arm has to outlast the mechanism it controls for.
#[test]
fn invariant_a_daemon_whose_state_directory_remains_keeps_serving() {
  let home = isolated_home("statedir-kept");

  // LIFELINE-EXEMPT for the same reason as the arm above: the two must differ
  // in the directory and in nothing else.
  let mut daemon_proc = Reaped(
    Command::new(env!("CARGO_BIN_EXE_intentd"))
      .env("HOME", &home)
      .stdin(Stdio::null())
      .stdout(Stdio::null())
      .stderr(Stdio::null())
      .spawn()
      .expect("intentd is built beside this test by cargo"),
  );

  wait_until_answering(&home);

  // Comfortably past the two-miss, two-second worst case of the state-directory
  // check, so a daemon that exits here is exiting on a timer rather than on the
  // removal that never happened.
  std::thread::sleep(Duration::from_millis(5_000));
  assert!(
    daemon_proc.running(),
    "A DAEMON WITH ITS STATE DIRECTORY INTACT STOPPED ON ITS OWN. The state-directory check is firing on something other than the directory, which means the arm above proves nothing and launchd's daemon -- KeepAlive false, no socket activation -- would not come back until the next login."
  );

  // **THE POSITIVE CONTROL ON THE ASSERTION ABOVE.** `running()` is also what a
  // broken observation returns; proving this test can see an exit when there is
  // one to see is what stops "still running" being vacuous.
  let signalled = Command::new("kill")
    .arg("-TERM")
    .arg(daemon_proc.id().to_string())
    .status()
    .expect("kill runs");
  assert!(signalled.success(), "could not signal the daemon");
  assert!(
    daemon_proc.wait_for_exit().is_some(),
    "the daemon did not exit on SIGTERM, so this test cannot observe an exit at all and its 'still running' assertion above establishes nothing"
  );

  let _ = std::fs::remove_dir_all(&home);
}

#[test]
fn invariant_the_lifeline_is_event_driven_and_carries_no_interval() {
  // **STRUCTURAL, BECAUSE A TIMING ASSERTION CANNOT TELL A FAST POLL FROM AN
  // EVENT.** A polled implementation passes the first arm given a generous
  // enough deadline and leaves a window proportional to its period, during
  // which an orphan is still holding the store -- invisible to any test that
  // waits long enough to be reliable.
  let src = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/main.rs"))
    .expect("intentd's main.rs is readable from its own test");

  let start = src
    .find("impl Lifeline {")
    .expect("the Lifeline impl is gone from main.rs -- this check has lost its subject and would pass over an empty string, which is the vacuous satisfaction it exists to refuse");
  let rest = &src[start..];
  let end = rest
    .find("\n}\n")
    .expect("could not find the end of the Lifeline impl");
  let body = &rest[..end];

  // Positive control on the EXTRACTION, not on the source: a slice that missed
  // its target would contain none of these and pass every assertion below.
  assert!(
    body.contains("Lifeline::Owned") && body.contains("std::thread::spawn"),
    "the extracted block is not the Lifeline impl, so the assertions below are about the wrong text"
  );

  for banned in ["sleep", "interval", "getppid", "Duration", "timeout"] {
    assert!(
      !body.contains(banned),
      "the lifeline path now mentions `{banned}`. AC-01.3 requires it to be event-driven with no interval to configure: a poll leaves a window during which an orphaned daemon still holds the store, and the window does not appear in any test generous enough to be reliable."
    );
  }

  // **THE DISCRIMINATOR HAS ONE HOME AND THIS SIDE MUST ASK IT, NOT COPY IT.**
  // Two processes ask "is my stdin a lifeline": `intentd`, to decide whether it
  // has an owner to outlive, and `intent daemon start`, to decide whether to
  // RELAY the one it was handed. A copy in either would be two answers to the
  // one question that decides whether a daemon can be left running for ever.
  assert!(
    body.contains("intentsvcs::daemon::stdin_is_a_lifeline()"),
    "intentd is no longer asking the shared predicate. If this side has grown its own copy of the fifo test, `intent daemon start` and the daemon it spawns can disagree about whether the daemon has an owner -- and the disagreement is silent."
  );
  assert!(
    !body.contains("is_fifo"),
    "the fifo test has been copied back into intentd. It belongs in `intentsvcs::daemon` because two crates ask it."
  );

  // **AND THE ONE HOME STILL SAYS WHAT IT MUST**, checked here rather than
  // trusted: a widened arming condition means launchd's daemon -- handed
  // `/dev/null`, a character device -- would enter the owned branch and exit,
  // and its plist is `KeepAlive false` with no socket activation, so it would
  // not come back until the next login.
  let shared = std::fs::read_to_string(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../intentsvcs/src/daemon.rs"
  ))
  .expect("intentsvcs::daemon is readable from here");
  assert!(
    shared.contains("pub fn stdin_is_a_lifeline() -> bool"),
    "the shared predicate is gone from intentsvcs::daemon, so this check has lost its subject"
  );
  assert!(
    shared.contains("m.file_type().is_fifo()"),
    "the shared predicate no longer keys on a fifo. Anything wider takes in `/dev/null`, which is what launchd hands a daemon."
  );

  // **NO SECOND ENVIRONMENT VARIABLE, WHICH IS AC-11.3's INVARIANT AND NOT
  // MINE.** The first build of this feature read `INTENT_DAEMON_LIFELINE` and
  // `the_shipped_surface_reads_exactly_one_environment_variable` refused it.
  // Asserted here too so the cheap regression is caught in the file that would
  // cause it, rather than only in the crate next door.
  // **KEYED ON A USE, NOT ON THE NAME, AND THE FIRST VERSION WAS KEYED ON THE
  // NAME AND FAILED ON ITS OWN DOCUMENTATION.** `main.rs` mentions
  // `INTENT_DAEMON_LIFELINE` in prose, explaining why the variable was removed
  // -- a mention, not a read. A guard that cannot tell the two apart makes
  // recording the reason for a decision into a violation of it, which is how
  // the reason gets deleted.
  assert!(
    !body.contains("env::var"),
    "the lifeline is reading an environment variable again. AC-11.3 allows the shipped surface exactly one, and adding a second needs an hv ruling and a row in ALLOWED -- not a quiet addition that passes here because every machine in this estate sets it."
  );
}

#[test]
fn invariant_the_owner_may_write_on_the_lifeline_without_ending_it() {
  // **A LIFELINE IS NOT A MESSAGE CHANNEL, AND CONFUSING THE TWO IS A REAL
  // RISK.** An implementation that exited on any readable event -- rather than
  // on EOF specifically -- would kill the daemon the moment its owner wrote a
  // byte for any reason. Nothing writes today; this arm is what keeps that true
  // by accident from becoming relied upon.
  let home = isolated_home("lifeline-chatty");

  let (reader, writer) = std::io::pipe().expect("a pipe");
  // **`exec` IS LOAD-BEARING AND ITS ABSENCE LEAKED A DAEMON.** Without it `sh`
  // may fork for `sleep`, so killing the pid this test holds leaves a CHILD
  // still holding the write end -- no EOF, and the arm reports a working
  // lifeline as broken while leaking the daemon it was measuring.
  let mut owner = Reaped(
    Command::new("sh")
      .arg("-c")
      .arg("echo hello; exec sleep 300")
      .stdout(Stdio::from(writer))
      .spawn()
      .expect("sh runs"),
  );

  let mut daemon_proc = Reaped(
    Command::new(env!("CARGO_BIN_EXE_intentd"))
      .env("HOME", &home)
      .stdin(Stdio::from(reader))
      .stdout(Stdio::null())
      .stderr(Stdio::null())
      .spawn()
      .expect("intentd runs"),
  );

  wait_until_answering(&home);
  std::thread::sleep(Duration::from_millis(500));
  assert!(
    daemon_proc.running(),
    "the daemon exited when its owner wrote a byte. The lifeline must end on EOF -- the owner being GONE -- not on readability."
  );

  // The owner is still there and the daemon is still serving; now end the owner
  // and require the same exit as the first arm, so this arm cannot pass by the
  // lifeline having been disabled altogether.
  owner.kill_now();
  assert!(
    daemon_proc.wait_for_exit().is_some(),
    "after a write, the lifeline no longer notices the owner's death -- a consumed or closed reader would look exactly like this"
  );

  let _ = std::fs::remove_dir_all(&home);
}
