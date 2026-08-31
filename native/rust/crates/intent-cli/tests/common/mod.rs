//! Helpers shared by the integration tests in this crate.
//!
//! **A DIRECTORY, NOT `tests/common.rs`.** Cargo compiles every `.rs` FILE
//! directly under `tests/` as its own test binary; a directory module is not
//! one, so this is the spelling that shares code without inventing an empty
//! test target.
//!
//! **AND NOT `testkit`, WHICH WOULD HAVE BEEN THE OBVIOUS HOME.** That crate
//! declares zero dependencies on purpose -- its own manifest says a testkit
//! with a dependency graph becomes a thing to reason about rather than a thing
//! to reach for -- and `dep_graph_guard.rs` walks every manifest under
//! `crates/`. `openpty` needs `libc`, which is already a dev-dependency HERE
//! and would be a new one THERE. Both callers of the harness below live in
//! this crate, so the narrower home is also the correct one.

#![allow(dead_code)]

/// A connected pseudo-terminal pair, as owned files.
///
/// **THE MASTER MUST OUTLIVE THE CHILD.** Dropping it closes the terminal's
/// other end, and the child then reads EOF -- or takes a hangup -- in the
/// middle of whatever it was doing. Every caller keeps it in scope for the
/// whole run, which is why this returns it rather than using it itself.
///
/// **IT IS THE ONLY WAY TO REACH A TERMINAL-GATED ARM FROM A TEST, AND THE
/// ALTERNATIVE WAS TRIED AND FAILED.** `script -q /dev/null` allocates a pty
/// from a shell, but it calls `tcgetattr` on ITS OWN stdin -- so under a
/// harness whose stdin is a socket or a pipe it exits 1 with `Operation not
/// supported on socket` and produces nothing. Two nodes measured that
/// independently on 2026-08-29, one of them after reporting the opposite from a
/// run that did not reproduce. `openpty` asks the kernel directly and does not
/// care what the ambient stdin is, so it behaves the same under a terminal,
/// under a test harness and under CI.
pub fn pty_pair() -> (std::fs::File, std::fs::File) {
  use std::os::fd::FromRawFd;
  let mut master: libc::c_int = 0;
  let mut slave: libc::c_int = 0;
  // SAFETY: both out-parameters are valid for writes, and the three null
  // pointers are documented as "use the defaults" for termios, winsize and the
  // returned slave name.
  let rc = unsafe {
    libc::openpty(
      &mut master,
      &mut slave,
      std::ptr::null_mut(),
      std::ptr::null_mut(),
      std::ptr::null_mut(),
    )
  };
  assert_eq!(
    rc,
    0,
    "openpty failed, so this test can say nothing about the arm it exists for: {}",
    std::io::Error::last_os_error()
  );
  // SAFETY: openpty returned 0, so both descriptors are open and owned by us.
  unsafe {
    (
      std::fs::File::from_raw_fd(master),
      std::fs::File::from_raw_fd(slave),
    )
  }
}

/// Everything the child wrote to the terminal, read after it has exited.
///
/// **`EIO` IS THE END OF THE STREAM HERE, NOT A FAILURE.** A pty master whose
/// every slave descriptor has closed returns `EIO` rather than a clean zero on
/// Darwin, so a reader that treats an error as a fault reports one on every
/// successful run. Anything else is propagated by panicking, because a test
/// that cannot read the terminal must not quietly return "nothing was written"
/// -- which is exactly the assertion its caller is about to make.
pub fn drain(mut master: std::fs::File) -> String {
  use std::io::Read;
  let mut out = Vec::new();
  let mut buf = [0u8; 4096];
  loop {
    match master.read(&mut buf) {
      Ok(0) => break,
      Ok(n) => out.extend_from_slice(&buf[..n]),
      Err(e) if e.raw_os_error() == Some(libc::EIO) => break,
      Err(e) => panic!(
        "reading the terminal failed, so this test can assert nothing about what was written to it: {e}"
      ),
    }
  }
  // The line discipline turns a bare newline into CRLF on the way out. Callers
  // compare against paths they built themselves, which carry neither.
  String::from_utf8_lossy(&out).replace('\r', "")
}

// ---------------------------------------------------------------------------
// A REAL `intentd`, AND THE HELPERS IT NEEDS.
//
// **MOVED HERE ON ITS THIRD CALLER, WHICH IS THE TRIGGER ITS OWN NOTE SET.**
// It lived in `daemon_and_local_agree.rs` deliberately: adding to `common`
// rebuilds every test binary in this crate, and two peers were mid-build the
// day it was written. The note said *a third caller moves it to `common`*, and
// `AC-08.5`'s carve-out witness is that caller -- a bare listener cannot answer
// `Op::Registry`, so nothing that needs a REAL daemon can be satisfied without
// this type.
//
// **A SECOND COPY WOULD DRIFT, AND THE DRIFT WOULD BE SILENT.** The parts that
// are easy to get subtly wrong -- reaping children before the parent, waiting
// on a real op rather than on the liveness probe, an isolated `HOME` -- are
// exactly the parts a copied fixture keeps while the original is fixed.
// ---------------------------------------------------------------------------

use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicU32, Ordering};

use intent_cli::dispatch;
use intentsvcs::daemon::{self, Endpoint, Route};
use intentsvcs::wire::{self, Op, Request, Response};

/// How many times a condition is asked before the test gives up on it.
///
/// A retry count rather than a deadline: what these loops require is that they
/// TERMINATE, which says nothing about the time and keeps this workspace's
/// clock guard (D42) true here rather than exempted.
pub const ATTEMPTS: u32 = 400;

pub const PAUSE: std::time::Duration = std::time::Duration::from_millis(20);

/// A short, unique directory under `/tmp`.
///
/// **NOT `tempfile`, AND NOT FOR TIDINESS.** A unix socket address is a
/// fixed-size field, so the whole path has to fit; `$TMPDIR` on macOS is a
/// ~50-character generated path and the daemon's own suffix is another 32.
/// Drive one `intent mcp` session against `root`: every frame written in
/// order, stdin closed (an MCP host's goodbye), every stdout line parsed as a
/// frame. `home` isolates daemon discovery exactly as the daemon fixtures do;
/// `None` leaves the ambient one.
///
/// **ONE DRIVER FOR EVERY TEST THAT SPEAKS TO THE SERVER**, so no test file
/// owns its own opinion about how frames are written or when stdin closes --
/// the failure when two drivers drift is a test that hangs on a server that
/// was waiting for a newline.
pub fn mcp_session(
  root: &Path,
  home: Option<&Path>,
  frames: &[&str],
) -> (std::process::Output, Vec<serde_json::Value>) {
  use std::io::Write;
  let mut cmd = Command::new(env!("CARGO_BIN_EXE_intent"));
  cmd
    .arg("mcp")
    .current_dir(root)
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped());
  if let Some(home) = home {
    cmd.env("HOME", home);
  }
  let mut child = cmd.spawn().expect("spawn intent mcp");
  {
    let stdin = child.stdin.as_mut().expect("stdin");
    for frame in frames {
      stdin.write_all(frame.as_bytes()).expect("write a frame");
      stdin.write_all(b"\n").expect("terminate it");
    }
    stdin.flush().expect("flush");
  }
  drop(child.stdin.take());
  let out = child.wait_with_output().expect("wait for the server");
  let parsed = String::from_utf8_lossy(&out.stdout)
    .lines()
    .map(|line| {
      serde_json::from_str(line).unwrap_or_else(|e| panic!("not one JSON frame: {line}: {e}"))
    })
    .collect();
  (out, parsed)
}

pub fn short_dir(tag: &str) -> PathBuf {
  static NEXT: AtomicU32 = AtomicU32::new(0);
  let dir = PathBuf::from("/tmp").join(format!(
    "intent-fixture-{tag}-{}-{}",
    std::process::id(),
    NEXT.fetch_add(1, Ordering::Relaxed)
  ));
  std::fs::create_dir_all(&dir).expect("create an isolated directory");
  dir
}

/// The pids whose parent is `pid`.
pub fn children_of(pid: u32) -> Vec<String> {
  let out = Command::new("pgrep")
    .args(["-P", &pid.to_string()])
    .output()
    .expect("pgrep runs");
  String::from_utf8_lossy(&out.stdout)
    .split_whitespace()
    .map(str::to_string)
    .collect()
}

/// A real `intentd`, started through the shipped `intent daemon run`.
pub struct RealDaemon {
  child: Child,
  home: PathBuf,
}

impl RealDaemon {
  pub fn start() -> RealDaemon {
    refuse_a_stale_sibling_daemon();
    // **AN ISOLATED `HOME`, AND THIS IS THE ONE LINE THAT MUST NOT BE WRONG.**
    // A daemon started under the real `$HOME` answers every peer session's
    // liveness probe at once and holds the `sync`/`ingest` family off the
    // store -- so a careless fixture here takes four developers' verbs down
    // together. That is not hypothetical: it happened on this machine on
    // 2026-08-30, from an `intentd --help`.
    let home = short_dir("dualpath-home");
    let running = RealDaemon {
      child: spawn_under(&home),
      home,
    };
    running.wait_until_it_answers_a_real_op();
    running
  }

  /// The pid of the process actually SERVING this daemon.
  ///
  /// **IT IS THE CHILD'S OWN PID BECAUSE `daemon run` EXECS** -- `render.rs`
  /// calls `Command::exec()`, which REPLACES the `intent` process with
  /// `intentd` rather than spawning it, so no grandchild exists and this pid is
  /// the server. **If that ever becomes a spawn this silently starts returning
  /// the wrapper**, and a test asserting "the daemon moved" would then be
  /// asserting that a wrapper moved. [`RealDaemon::restart`] checks the
  /// assumption rather than restating it.
  pub fn pid(&self) -> u32 {
    self.child.id()
  }

  /// Kill this daemon and bring a fresh one up **under the same home**,
  /// returning once it answers a real op.
  ///
  /// **THE HOME IS DELIBERATELY REUSED AND THE SOCKET PATH DOES NOT MOVE.**
  /// `userstate::daemon_socket_under` is `<home>/.local/share/intent/intentd.sock`
  /// -- no pid, no port, no nonce -- so a restart produces the IDENTICAL
  /// `Endpoint::Unix`. **An endpoint comparison is therefore not a witness that
  /// anything restarted**, and a test written against one would assert
  /// something false by construction. The pid is the witness, which is why
  /// [`RealDaemon::pid`] exists beside this.
  ///
  /// **WHAT THIS IS FOR (ic, AC-09.3): per-request target resolution.** A
  /// client that resolved once at startup, or that cached a connection, fails
  /// against a dead-then-live process behind an unchanged path; one that
  /// resolves per request passes. That is the row's actual subject, and the
  /// unchanged socket makes it a sharper test rather than a weaker one.
  ///
  /// **`Drop` IS NOT USED TO STOP THE OLD ONE**, because `Drop` also removes
  /// the home -- which is the one thing a restart must not do. The reaping is
  /// shared with `Drop` through [`RealDaemon::reap`] so there is one kill path
  /// and not two to drift.
  pub fn restart(mut self) -> RealDaemon {
    let was = self.pid();
    self.reap();
    self.child = spawn_under(&self.home);
    self.wait_until_it_answers_a_real_op();

    assert_ne!(
      was,
      self.pid(),
      "the restarted daemon reports the same pid as the one just killed, so nothing moved \
       and any test using this as a witness is measuring nothing"
    );
    // **THE EXEC ASSUMPTION, CHECKED WHERE IT IS RELIED ON.** `pid()` is the
    // server only while `daemon run` execs. A grandchild here means it now
    // spawns, `pid()` has quietly become the wrapper's, and the assertion
    // above still passes -- so this is the arm that would catch it.
    assert!(
      children_of(self.pid()).is_empty(),
      "`intent daemon run` has a child process, so it no longer execs into intentd -- \
       `pid()` is now the wrapper's and is no longer a witness for which daemon answered"
    );
    self
  }

  /// Stop the child and everything under it, WITHOUT touching the home.
  ///
  /// **BY PID, NEVER BY NAME** -- reaping everything that looks like an
  /// `intentd` would kill a concurrent session's daemon; four of us share this
  /// machine.
  fn reap(&mut self) {
    for child in children_of(self.child.id()) {
      let _ = Command::new("kill").arg("-TERM").arg(&child).status();
    }
    let _ = self.child.kill();
    let _ = self.child.wait();
  }

  pub fn home(&self) -> &Path {
    &self.home
  }

  pub fn endpoint(&self) -> Option<Endpoint> {
    let candidates = daemon::candidates_under(&self.home).ok()?;
    match daemon::route(&candidates) {
      Route::Daemon(endpoint) => Some(endpoint),
      Route::InProcess => None,
    }
  }

  /// Block until this daemon answers a REAL op, not merely the probe.
  ///
  /// **THE READINESS CONDITION IS DELIBERATELY STRONGER THAN `daemon::route`,
  /// AND THE DIFFERENCE IS THE WHOLE POINT OF USING A REAL DAEMON.** `route`
  /// asks whether something answers the liveness probe, and a bare listener
  /// does -- that is the phantom this crate's routing tests spend three
  /// fixtures on. `Op::Registry` requires a serving `intentd`, so waiting on it
  /// makes the fixture unfalsifiably real rather than merely present.
  pub fn wait_until_it_answers_a_real_op(&self) {
    for _ in 0..ATTEMPTS {
      if let Some(endpoint) = self.endpoint()
        && matches!(
          wire::ask(
            &endpoint,
            &Request {
              root: self.home.clone(),
              op: Op::Registry,
            },
          ),
          Ok(Response::Registry { .. })
        )
      {
        return;
      }
      std::thread::sleep(PAUSE);
    }
    panic!(
      "no intentd answered `Op::Registry` under HOME={} in {ATTEMPTS} attempts.\n\nIf `intent daemon run` refused, the usual cause is that `target/debug/intentd` is absent or stale: `cargo test -p intent-cli` builds THIS package's binaries and not another package's, so the sibling `intentd` this verb execs into is whatever an earlier build left. Run `cargo build -p intentd` (or drive the workspace) and try again.",
      self.home.display()
    );
  }

  /// How many ops this daemon has dispatched to `root`'s store.
  ///
  /// **`Op::Registry` IS IN `wire::UNCOUNTED`, WHICH IS WHY READING THE COUNTER
  /// DOES NOT MOVE IT.** vc declared that set rather than leaving it implied by
  /// a branch in the dispatcher, precisely so this measurement exists: an
  /// instrument that perturbed its own subject would report `+2` for every
  /// bracketed verb and there would be nothing to compare against.
  ///
  /// A project the daemon has never opened is not in the listing at all, and
  /// that is **0 dispatches**, not an error -- it is the state every one of
  /// these brackets starts in.
  pub fn dispatched(&self, root: &Path) -> u64 {
    let endpoint = self
      .endpoint()
      .expect("the daemon was answering when this test started");
    let response = wire::ask(
      &endpoint,
      &Request {
        root: root.to_path_buf(),
        op: Op::Registry,
      },
    )
    .expect("the shipped client completes a round trip to a live daemon");

    let Response::Registry { projects } = response else {
      panic!("intentd answered Op::Registry with something else: {response:?}");
    };
    // **CANONICALISED ON BOTH SIDES, BECAUSE `/tmp` IS A SYMLINK ON macOS.**
    // The daemon holds the resolved root; this fixture creates paths under the
    // symlinked one. Comparing the two as written finds nothing, reports 0
    // dispatches for every verb, and the `--daemon` arm fails with a message
    // about routing that would be entirely about a path.
    let wanted = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    projects
      .iter()
      .find(|p| p.root.canonicalize().unwrap_or_else(|_| p.root.clone()) == wanted)
      .map(|p| p.dispatched)
      .unwrap_or(0)
  }

  /// Is this daemon WATCHING the project at `root`?
  ///
  /// **THE OBSERVABLE `AC-08.5`'s NARROWED CARVE-OUT IS ASSERTED AGAINST, AND
  /// IT IS READ OFF THE WIRE RATHER THAN INFERRED FROM HAVING CONTACTED THE
  /// DAEMON.** A fixture that assumed *I asked about this project, therefore it
  /// is watched* would agree with itself: registration and watching are
  /// separate acts, and `watch::start` is allowed to fail leaving the project
  /// SERVED AND NOT WATCHED. That state is exactly the one where the carve-out
  /// must NOT refuse, so a test that could not see it would be unable to tell
  /// a working narrowing from a broken one.
  ///
  /// `false` for a project this daemon has never opened, which is the other
  /// half of the same question.
  pub fn watching(&self, root: &Path) -> bool {
    let endpoint = self
      .endpoint()
      .expect("the daemon was answering when this test started");
    let response = wire::ask(
      &endpoint,
      &Request {
        root: root.to_path_buf(),
        op: Op::Registry,
      },
    )
    .expect("the shipped client completes a round trip to a live daemon");
    let Response::Registry { projects } = response else {
      panic!("intentd answered Op::Registry with something else: {response:?}");
    };
    let wanted = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    projects
      .iter()
      .find(|p| p.root.canonicalize().unwrap_or_else(|_| p.root.clone()) == wanted)
      .map(|p| p.watched)
      .unwrap_or(false)
  }
}

/// Start `intent daemon run` under a given home. **ONE spawn site**, shared by
/// [`RealDaemon::start`] and [`RealDaemon::restart`], so the two cannot drift
/// about the flags, the environment or the redirected output -- a restart that
/// differed from the original start in any of those would be testing a
/// different daemon than the one the test set up.
fn spawn_under(home: &Path) -> Child {
  Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(["daemon", "run"])
    .env("HOME", home)
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .spawn()
    .expect("the shipped intent binary runs")
}

impl Drop for RealDaemon {
  /// **REAP THE CHILDREN BEFORE THE PARENT, AND CLEAN UP IN `Drop` RATHER THAN
  /// AFTER THE ASSERTIONS.** A kill written after them is dead code until an
  /// assertion fires, and on that day it does not run -- leaving a real daemon
  /// holding the harness's descriptors and a `cargo` that never returns. The
  /// child reaping is for the case where `daemon run` ever stops being an
  /// `exec`: a spawn makes `intentd` a GRANDCHILD, so killing `intent` orphans
  /// it to init. **That is not hypothetical -- a sibling fixture leaked exactly
  /// that while its subject was broken**, which is the worst possible moment
  /// and the case a happy-path cleanup never covers.
  ///
  /// **BY PID, NEVER BY NAME.** Reaping everything that looks like an `intentd`
  /// would kill a concurrent session's daemon; four of us share this machine.
  fn drop(&mut self) {
    // **THE KILL IS `reap`'s AND THE HOME REMOVAL IS THIS FUNCTION'S**, which
    // is the whole reason they are separate: `restart` needs the first without
    // the second, and a second copy of the reaping here would be the two-homes
    // shape in the one place a leak costs a hung `cargo`.
    self.reap();
    let _ = std::fs::remove_dir_all(&self.home);
  }
}

// ---------------------------------------------------------------------------
// SHIPPED-SOURCE SCANNING
//
// **MOVED HERE FROM `no_pm_state_in_output.rs` ON 2026-08-30, AND THE MOVE IS
// THE POINT.** `AC-09.4`'s clause 2 needs the same walk -- every hand-kept
// roster of command paths is derived from the table or declared as an
// exception -- and a test binary cannot import another test binary. **The
// alternative was not to wait, it was to FORK**, which would have committed a
// second copy of a source walk inside the commit closing the row about
// hand-kept duplication: an artefact refuting its own message.
//
// cc's `RealDaemon` move above set the direction, and cc's own correction is
// worth keeping with it: their note's trigger was TIMING, not a count.
// `common` compiles into every test binary in this crate, so adding to it
// rebuilds every target for every session on this machine -- and that bill is
// identical at two callers and at three, so a rule letting the third pay and
// making the second fork would be arbitrary.
// ---------------------------------------------------------------------------

/// Every SHIPPED command path the dispatch table declares -- from **both** of
/// its row homes.
///
/// **This was wrong in two directions at once, which is why it looked right**
/// (vc, issue 0037). It walked `families[].entries[]` and stopped, so the
/// top-level `new_surface` array -- `search`, `sync`, `schema`, `export`,
/// `ingest`, `backup`, `daemon`, `mcp`, eight rows with zero overlap -- was
/// never scanned by ANY surface in this file. Their help lives in the
/// compiled-in JSON rather than in Rust literals, so the string-literal scan
/// does not reach them either. And it took every row regardless of
/// disposition, so it also drove five RETIRED paths. One enumerator, too
/// narrow and too wide.
///
/// **The count assertion was the reason nobody noticed.** It read
/// `paths.len() > 20` under the message "precondition: the dispatch table
/// declares the command surface" -- a sentence that reads as a coverage claim
/// and is a did-the-file-parse check. It passes at 104 and it passes at 112,
/// so it could not see a twelfth of the surface be absent. **A precondition
/// whose message describes a stronger property than it tests is worse than no
/// message**, because it answers the question a reader came to ask.
///
/// So the shape is now: read both homes, and filter on the SAME
/// [`dispatch::Entry::is_shipped`] the spine applies when it builds the
/// surface -- reusing that decision rather than making a second one that can
/// drift from it. The table is read through the typed `dispatch::table()` for
/// the same reason.
pub fn declared_paths() -> Vec<String> {
  let table = dispatch::table();
  let from_families: Vec<String> = table
    .families
    .iter()
    .flat_map(|f| f.entries.iter())
    .filter(|e| e.is_shipped())
    .map(|e| e.path.clone())
    .collect();
  let from_new_surface: Vec<String> = table
    .new_surface
    .iter()
    .filter(|e| e.is_shipped())
    .map(|e| e.path.clone())
    .collect();

  // **Each home is asserted separately, because the defect was one home
  // returning nothing while the total still looked healthy.** A single total
  // cannot distinguish "both homes read" from "one home read and the other is
  // large"; these two can, and they are what actually regressed.
  assert!(
    !from_families.is_empty(),
    "precondition: no shipped row was read from `families`, so the ported surface is unscanned"
  );
  assert!(
    !from_new_surface.is_empty(),
    "precondition: no shipped row was read from `new_surface`, so v3's own commands are \
     unscanned -- this is issue 0037 exactly, and it passed for a day"
  );

  let paths: Vec<String> = from_families.into_iter().chain(from_new_surface).collect();

  // And the total EQUALS what the table declares as shipped, computed by
  // counting rather than by collecting, so going short is an error rather than
  // a smaller number that still satisfies a `>`.
  let shipped = table
    .families
    .iter()
    .flat_map(|f| f.entries.iter())
    .chain(table.new_surface.iter())
    .filter(|e| e.is_shipped())
    .count();
  assert_eq!(
    paths.len(),
    shipped,
    "the scan covers every shipped row the table declares, or it covers an unstated subset"
  );
  paths
}

/// Shipped source: the three crates that become binaries or are linked into
/// one.
pub fn shipped_sources() -> Vec<PathBuf> {
  let crates = Path::new(env!("CARGO_MANIFEST_DIR"))
    .parent()
    .expect("crates/ above this crate")
    .to_path_buf();
  let mut files = Vec::new();
  for name in ["intent-cli", "intentsvcs", "intentd"] {
    collect_rs(&crates.join(name).join("src"), &mut files);
  }
  assert!(
    files.len() > 10,
    "precondition: the shipped crates have source, found {}",
    files.len()
  );
  files.sort();
  files
}

pub fn collect_rs(dir: &Path, out: &mut Vec<PathBuf>) {
  let Ok(entries) = std::fs::read_dir(dir) else {
    return;
  };
  for e in entries.filter_map(Result::ok) {
    let p = e.path();
    if p.is_dir() {
      collect_rs(&p, out);
    } else if p.extension().is_some_and(|x| x == "rs") {
      out.push(p);
    }
  }
}

/// Every string-literal CONTENT in one Rust file, comments and the trailing
/// test module excluded.
///
/// **A line-based "is there a quote on this line" test does not work here and
/// the reason is load-bearing.** `store.rs`'s DDL is one string literal
/// spanning two hundred lines, and its interior `--` comment lines carry no
/// quote at all -- they are literal content that `intent schema ddl.sql` prints
/// verbatim. A line-based scan sees no quote and skips exactly the lines that
/// are published. So this tracks literal spans.
///
/// **Char literals are HANDLED rather than assumed absent, and the reason is
/// that the assumption failed within the hour.** This first asserted that no
/// `'"'` appeared in shipped source, on the grounds that none did. Then
/// `faces.rs` grew a `marker()` reader whose last step is `.trim_matches('"')`
/// -- ordinary, correct code -- and the assertion fired. **It fired rather than
/// mis-scanning, which is the whole argument for stating an assumption instead
/// of relying on it**, but a guard that refuses legitimate code is a guard
/// someone deletes, so the scanner learned the construct.
///
/// A leading `'` is ambiguous in Rust -- `'a` is a lifetime, `'x'` is a char --
/// so it is disambiguated by looking for the closing quote two or three bytes
/// on. A lifetime has none and simply advances.
///
/// Block comments are still asserted absent, because there are none and
/// handling nesting is real work for a construct this codebase does not use.
pub fn string_literals(code: &str) -> Vec<String> {
  // **THE BLOCK-COMMENT ASSERTION MOVED INTO THE WALK ON 2026-08-20, AND THE
  // REASON IS THAT ITS DETECTION HAD STOPPED MATCHING ITS SUBJECT.**
  //
  // It stood here as `!code.contains("/*")` over the whole file text, which is
  // the one place in this function that does not know whether it is inside a
  // literal. `critic.rs` landed glob patterns -- `"test/**/*_test.exs"`,
  // `"lib/**/*.ex"`, `"lib/*.ex"` -- and every one of them contains `/*` inside
  // a STRING. Two of eight tests failed reporting a block comment in a file
  // that has none.
  //
  // **The assumption the doc states is still true and was never the problem.**
  // There are no block comments in shipped source, and the scanner still
  // refuses rather than mis-scanning if one appears. What was wrong was a
  // substring test standing in for a syntactic fact -- the same shape as
  // ST0039's greppable proxies, where a regex that cannot see structure is
  // asked a question only structure can answer.
  //
  // So the check now fires from inside the walk, at a point where `i` is known
  // to be outside every literal, comment and raw string. It is strictly more
  // precise: a real block comment is still caught, and correct code that merely
  // spells `/*` is not.

  // The trailing `#[cfg(test)] mod tests` is Intent's own test fixtures, which
  // AC-00.9 exempts: they are never compiled into a shipped binary, so nothing
  // in them can be emitted. Every shipped file has at most one, at the end --
  // asserted, because truncating at the first would silently drop real code if
  // that ever stopped being true.
  assert!(
    code.matches("#[cfg(test)]").count() <= 1,
    "a shipped file grew a second `#[cfg(test)]`, so truncating at the first would drop shipped \
     code from this scan"
  );
  let code = match code.find("#[cfg(test)]") {
    Some(at) => &code[..at],
    None => code,
  };

  let b = code.as_bytes();
  let mut out = Vec::new();
  let mut i = 0;
  while i < b.len() {
    // A line comment, outside any literal.
    if b[i] == b'/' && i + 1 < b.len() && b[i + 1] == b'/' {
      while i < b.len() && b[i] != b'\n' {
        i += 1;
      }
      continue;
    }
    // A block comment, outside any literal -- the assertion the pre-check
    // above used to make on raw text. Reaching here means `i` is not inside a
    // string, a raw string, a char literal or a line comment, so this is a real
    // `/*` and not a glob.
    assert!(
      !(b[i] == b'/' && i + 1 < b.len() && b[i + 1] == b'*'),
      "a block comment appeared in shipped source at byte {i}; this scanner only skips `//` line \
       comments, so it would read the comment's body as code. **Check it is a real block comment \
       and not a `/*` inside a string** -- glob patterns like `lib/**/*.ex` are correct code and \
       reach this line only if the walk above has a hole"
    );
    // A char literal, which may CONTAIN a quote (`'"'`) and would otherwise be
    // read as opening a string. A lifetime (`'static`) has no closing quote in
    // that position and falls through to the plain advance below.
    if b[i] == b'\'' {
      let closes_at = if b.get(i + 1) == Some(&b'\\') { 3 } else { 2 };
      if b.get(i + closes_at) == Some(&b'\'') {
        i += closes_at + 1;
        continue;
      }
    }
    // A raw string: `r"`, `r#"`, `r##"` ... which has no escapes, so it ends
    // only at a quote followed by the same number of hashes.
    if b[i] == b'r' && (i == 0 || !(b[i - 1].is_ascii_alphanumeric() || b[i - 1] == b'_')) {
      let mut h = i + 1;
      while h < b.len() && b[h] == b'#' {
        h += 1;
      }
      if h < b.len() && b[h] == b'"' {
        let hashes = h - (i + 1);
        let start = h + 1;
        let mut j = start;
        while j < b.len() {
          if b[j] == b'"'
            && b[j + 1..]
              .iter()
              .take(hashes)
              .filter(|c| **c == b'#')
              .count()
              == hashes
          {
            break;
          }
          j += 1;
        }
        out.push(code[start..j.min(code.len())].to_string());
        i = (j + 1 + hashes).min(b.len());
        continue;
      }
    }
    if b[i] == b'"' {
      let start = i + 1;
      let mut j = start;
      while j < b.len() {
        match b[j] {
          b'\\' => j += 2,
          b'"' => break,
          _ => j += 1,
        }
      }
      out.push(code[start..j.min(code.len())].to_string());
      i = (j + 1).min(b.len());
      continue;
    }
    i += 1;
  }
  out
}

/// **REFUSE A SIBLING `intentd` OLDER THAN THE SOURCE IT IS SUPPOSED TO BE.**
///
/// **THE MEASURED EPISODE (ic, 2026-08-30, found by hitting it).**
/// `a_project_with_backups_turned_off_is_opened_and_not_backed_up` went red on
/// a clean HEAD -- the daemon opened a backups-off project and swept it anyway.
/// It read as a defect in `backup.enabled`, a key that had landed hours
/// earlier, in a lane that was not ic's. **It was a binary.** `strings |
/// grep -c 'backup.enabled = false'` gives 1 for a daemon built after
/// `42402762` and 0 for the one that target dir was holding.
///
/// **WHY NOTHING CAUGHT IT, AND BOTH EXISTING PROTECTIONS MISS BY DESIGN.**
/// `render.rs`'s `intentd_candidates` resolves the daemon as
/// `current_exe().parent().join("intentd")` -- a SIBLING -- and `cargo test -p
/// intent-cli` builds THIS package's binaries, never another package's, so the
/// sibling is whatever an earlier build left. [`wait_until_it_answers_a_real_op`]
/// panics when no daemon answers, which covers ABSENT and refusing. The
/// version check covers a CROSS-version mismatch, and both binaries say
/// `3.0.0`. **Same version, different build, nothing to compare** -- so a stale
/// daemon starts cleanly and returns a confident wrong answer.
///
/// **THAT IS STRICTLY WORSE THAN A CRASH, WHICH IS WHY THIS EXISTS.** An absent
/// daemon fails loudly and blames nothing. A stale one attributes its own
/// staleness to whoever owns the key under test, and sends them to read correct
/// code looking for a defect that is not there.
///
/// **IT REFUSES AND NEVER REBUILDS.** A harness that quietly ran `cargo build`
/// would hide the class from the node that needs to learn its binary was
/// stale -- the same reasoning the clock guard uses for printing the right
/// stamp rather than writing it.
///
/// **THE COMPARISON IS AGAINST SOURCE MTIME, WHICH IS WHAT `stale` MEANS
/// HERE.** Comparing against the sibling `intent` would be quieter and wrong:
/// cargo does not relink an unchanged binary, so both would be old together and
/// the check would pass on exactly the tree it is meant to refuse. An absent
/// sibling is left alone -- that case already has a better message one call
/// down, and duplicating it here would give one failure two homes.
fn refuse_a_stale_sibling_daemon() {
  let intent = Path::new(env!("CARGO_BIN_EXE_intent"));
  let Some(sibling) = intent.parent().map(|dir| dir.join("intentd")) else {
    return;
  };
  let Ok(built) = std::fs::metadata(&sibling).and_then(|m| m.modified()) else {
    // Absent, or unreadable: `wait_until_it_answers_a_real_op` says it better.
    return;
  };

  let root = testkit::workspace_root();
  let mut newest = std::time::UNIX_EPOCH;
  let mut newest_path = std::path::PathBuf::new();
  for crate_name in ["intentd", "intentsvcs"] {
    newest_source(
      &root.join("crates").join(crate_name).join("src"),
      &mut newest,
      &mut newest_path,
    );
  }

  assert!(
    built >= newest,
    "the sibling `intentd` this test would spawn is OLDER than the code it is supposed to be \
     running, so it will answer confidently with behaviour that is no longer in the tree -- and \
     the failure will look like a defect in whatever key the test is about.\n\n  \
     daemon:  {}\n  newer:   {}\n\n  \
     `cargo test -p intent-cli` builds this package's binaries and NOT another package's, and \
     the daemon is resolved as a sibling of the `intent` cargo just built.\n  \
     fix: cargo build -p intentd",
    sibling.display(),
    newest_path.display()
  );
}

/// The newest modification time under `dir`, and the file carrying it.
fn newest_source(
  dir: &Path,
  newest: &mut std::time::SystemTime,
  newest_path: &mut std::path::PathBuf,
) {
  let Ok(entries) = std::fs::read_dir(dir) else {
    return;
  };
  for entry in entries.flatten() {
    let path = entry.path();
    if path.is_dir() {
      newest_source(&path, newest, newest_path);
    } else if path.extension().is_some_and(|e| e == "rs")
      && let Ok(at) = entry.metadata().and_then(|m| m.modified())
      && at > *newest
    {
      *newest = at;
      *newest_path = path;
    }
  }
}
