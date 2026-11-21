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

pub mod level_three;

/// The path of one of `root`'s canon files, from its path relative to the
/// intent directory, as `intentsvcs::project::canon_ac_rel` and its kin name a
/// thread's records (ST0096 WP-05).
pub fn canon_file(root: &std::path::Path, rel: &str) -> std::path::PathBuf {
  intentsvcs::project::Project::open(root)
    .expect("the project opens")
    .intent_dir()
    .join(rel)
}

/// One of `root`'s canon files, by its path relative to the intent directory,
/// parsed as plain JSON.
pub fn canon_json(root: &std::path::Path, rel: &str) -> serde_json::Value {
  let path = canon_file(root, rel);
  let text =
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
  serde_json::from_str(&text).unwrap_or_else(|e| panic!("{} is JSON: {e}", path.display()))
}

/// Give a project made by hand the project state `intent init` writes, through
/// the library's one encoder, so it is in the release's canon format and its
/// mints are not refused as those of a project `intent upgrade` has not moved
/// (ST0095 WP-06). The intentsvcs tests' `Fixture::new` does the same for
/// theirs; a test crate's helpers cannot cross into another's.
pub fn born_upgraded(root: &std::path::Path) {
  let project = intentsvcs::project::Project::open(root).expect("a project with its config");
  let path = project.project_json();
  std::fs::create_dir_all(path.parent().expect("the canon directory")).expect("mkdir canon");
  let state = intentsvcs::canon_format::encode_project_state(
    None,
    intentsvcs::canon_format::CanonFormat::CURRENT,
  )
  .expect("the project state renders");
  std::fs::write(&path, state).expect("write the project state");
}

/// Put a project `intent init` made back in 3.4.0's format, for a test whose
/// subject is that format's `.intentfiles` (ST0096 WP-01): no project state,
/// and no store, so the next open ingests a project that names no format. A
/// mint refuses in this format, so its threads and issues are seeded with
/// [`seed_ordinal_thread`] and [`seed_ordinal_issue`].
pub fn born_ordinal(root: &std::path::Path) {
  let project = intentsvcs::project::Project::open(root).expect("a project with its config");
  std::fs::remove_file(project.project_json()).expect("remove the project state");
  let db = project.db_path();
  for path in [
    db.clone(),
    db.with_extension("db-wal"),
    db.with_extension("db-shm"),
  ] {
    match std::fs::remove_file(&path) {
      Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
      done => done.expect("remove the store"),
    }
  }
}

/// The manifest 3.4.0's `intent init` wrote: the header, declaring nothing,
/// through the one function that derives it.
pub fn seed_init_manifest(root: &std::path::Path) {
  let project = intentsvcs::project::Project::open(root).expect("a project with its config");
  std::fs::write(
    project.intentfiles_path(),
    intentsvcs::intentfiles::default_declaration(&[], &[]),
  )
  .expect("write the manifest");
}

/// The `intent` binary cargo built for these tests, as a path.
///
/// **THE ONLY PLACE IN THIS CRATE'S TESTS THAT NAMES `CARGO_BIN_EXE_intent`
/// (issue 0493).** `every_spawn_has_a_fixture_home.rs` refuses the literal
/// anywhere else. So a new file reaches the binary through here, and a spawn
/// through here gets the fixture HOME without having to know it needs one.
/// This is for a caller that needs the PATH: to copy the binary, link it, or
/// hand it to a shell. It is not for spawning. A spawn goes through [`intent`],
/// and a copy is spawned through `testkit::fixtured_command`.
pub fn intent_path() -> &'static std::path::Path {
  std::path::Path::new(env!("CARGO_BIN_EXE_intent"))
}

/// A `Command` for the `intent` binary with the fixture HOME and no XDG
/// variables (`testkit::fixtured_command`). This is how a test spawns it.
pub fn intent() -> std::process::Command {
  testkit::fixtured_command(intent_path())
}

/// A HOME where the skill that releases the prompt gate is installed, for an
/// arm that drives `require-in-session` and needs its block branch.
///
/// **WITHOUT IT THE GATE FAILS OPEN AND WRITES A SENTINEL (issue 0601 (b)).**
/// The fixture HOME has no `~/.claude/skills/in-session`, so the gate passes
/// the prompt with exit 0 and touches `/tmp/intent/in-session-<id>.sentinel`,
/// shared by every run on the machine. A fixed id is then released for every
/// later run as well, and an id inherited from the operator's Claude session
/// releases THAT session's gate. The releaser is only tested for presence, so
/// an empty file is enough, as the bats arm in `no_absolute_home_paths.bats`
/// plants it.
pub fn gate_home() -> tempfile::TempDir {
  let home = tempfile::tempdir().expect("the gate's HOME");
  let scripts = home.path().join(".claude/skills/in-session/scripts");
  std::fs::create_dir_all(&scripts).expect("mkdir the releaser's directory");
  std::fs::write(scripts.join("release-gate.sh"), "").expect("plant the releaser");
  home
}

/// A gate session id no sentinel can already exist for: `tag` plus this
/// process's id. A fixed id is only as good as the machine's `/tmp/intent`, and
/// a build carrying the fail-open branch without [`gate_home`] wrote sentinels
/// for the fixed ids these arms used, which then pass every prompt.
pub fn gate_session(tag: &str) -> String {
  format!("{tag}-{}", std::process::id())
}

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

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
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
/// `None` leaves the fixture HOME [`intent`] gives every spawn (issue 0493; it
/// left the operator's own until then).
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
  mcp_session_with_env(root, home, &[], frames)
}

/// [`mcp_session`] with each named variable set to `Some` value or removed for
/// `None`, for an arm about what the server reads from the session that
/// started it (ST0092 WP-05: `CLAUDE_CODE_SESSION_ID`).
pub fn mcp_session_with_env(
  root: &Path,
  home: Option<&Path>,
  env: &[(&str, Option<&str>)],
  frames: &[&str],
) -> (std::process::Output, Vec<serde_json::Value>) {
  use std::io::Write;
  let mut cmd = intent();
  for (name, value) in env {
    match value {
      Some(v) => cmd.env(name, v),
      None => cmd.env_remove(name),
    };
  }
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

/// The `intent_search` tool's envelope for one call with `arguments`, a JSON
/// object, in an [`mcp_session`] of its own against `root`: the initialize
/// frame, the one call, and the answer's text unwrapped and parsed.
pub fn search_tool_answer(root: &Path, home: Option<&Path>, arguments: &str) -> serde_json::Value {
  let call = format!(
    r#"{{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{{"name":"intent_search","arguments":{arguments}}}}}"#
  );
  let (out, frames) = mcp_session(
    root,
    home,
    &[
      r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"drive","version":"0"}}}"#,
      &call,
    ],
  );
  assert!(
    out.status.success(),
    "the MCP session failed: {}",
    String::from_utf8_lossy(&out.stderr)
  );
  let answer = frames
    .iter()
    .find(|frame| frame["id"] == 2)
    .unwrap_or_else(|| panic!("no response to the call: {frames:?}"));
  let text = answer["result"]["content"][0]["text"]
    .as_str()
    .unwrap_or_else(|| panic!("the tool answered no text-wrapped JSON: {answer}"));
  serde_json::from_str(text)
    .unwrap_or_else(|e| panic!("the tool's envelope is not JSON: {text}: {e}"))
}

/// The servable verbs that refuse without an argument.
///
/// **THE LOOPS OVER [`crate::render::daemon_servable_paths`] SPLIT THE PATH AND
/// RAN IT**, which was right while every servable verb took no arguments and
/// stopped being right the moment one required a query. A bare `intent search`
/// is a usage error, and two usage errors are identical, exit the same way and
/// prove nothing about routing -- so the comparison would have gone green on a
/// verb that never ran.
///
/// **DECLARED ONCE HERE RATHER THAN IN EACH CALLER**, because two copies of
/// this list is a second statement of which verbs are special, and the copies
/// agree on the day they are written. `daemon_and_local_agree.rs` deferred the
/// move to `common` on the cost of rebuilding ~70 test targets; the suite has
/// since become ONE target, so that cost is gone and the reason with it.
const NEEDS_A_QUERY: &[&str] = &["search"];

/// The command line a servable verb is invoked with in a routing test.
///
/// `findable` is the text this fixture wants a search to hit -- passed in
/// rather than declared here, because each caller's fixture mints its own and
/// the assertions downstream are about THAT text appearing in the answer.
pub fn servable_argv<'a>(path: &'a str, findable: &'a str) -> Vec<&'a str> {
  let mut argv: Vec<&str> = path.split(' ').collect();
  if NEEDS_A_QUERY.contains(&path) {
    argv.push(findable);
  }
  argv
}

/// Put a fixture's findable text somewhere the file index can reach it.
///
/// **`st_new` WRITES THE STORE AND THE TREE IS REALISED LAZILY**, so a thread's
/// own prose is not a file any reconcile walks. Without this a search for the
/// minted title returns an honest empty set from both the local and the daemon
/// path -- identical, exit 0, and vacuous, which is the agreement these
/// fixtures mint a thread specifically to refuse.
pub fn seed_findable_text(root: &Path, findable: &str) {
  let docs = root.join("intent").join("docs");
  std::fs::create_dir_all(&docs).expect("the project's docs directory");
  std::fs::write(docs.join("findable.md"), format!("# {findable}\n"))
    .expect("write the file a search has to find");
}

/// A facade on the project at `root`, as a test opens one: the shipped
/// project reader and the test context (ST0095 WP-10, T5). intentd's
/// `tests/common::facade_on` mirrors it, because `testkit` takes no
/// dependencies and so cannot hold one copy for both crates.
pub fn facade_on(root: &Path) -> intentsvcs::facade::Facade {
  let opened = intentsvcs::project::Project::open(root).expect("the project opens");
  let ctx = intentsvcs::facade::FacadeContext::for_tests(
    "test".to_string(),
    opened.config().project_id.clone().unwrap_or_default(),
    env!("CARGO_PKG_VERSION").to_string(),
  );
  intentsvcs::facade::Facade::open(opened, ctx).expect("a facade on the project")
}

/// A project the shipped initialiser makes at a fresh short path named for
/// `tag`, and a facade on it: the start of every test that asks a daemon about
/// a record it minted (ST0095 WP-10, T5).
pub fn initialised(tag: &str, name: &str) -> (PathBuf, intentsvcs::facade::Facade) {
  let root = short_dir(tag);
  intentsvcs::init::init(&root, name, "test", env!("CARGO_PKG_VERSION"))
    .expect("the shipped initialiser creates a project");
  let facade = facade_on(&root);
  (root, facade)
}

/// Write a v2 steel thread the way v2.19 writes one: the one copy the
/// upgrade and ingest tests share (ST0095 WP-10, T3). intentsvcs writes v2
/// views inline in its own tests; folding them into `testkit` is a later step.
pub fn v2_thread(dir: &std::path::Path, id: &str, status: &str) {
  let st = dir.join("intent/st").join(id);
  std::fs::create_dir_all(&st).expect("mkdir st");
  std::fs::write(
    st.join("info.md"),
    format!(
      "---\nverblock: \"14 Aug 2026:v0.1: cc - x\"\nintent_version: 2.19.0\nstatus: {status}\nslug: a-slug\ncreated: 20260814\ncompleted:\n---\n\n# {id}: A thread\n\n## Objective\n\nShip it.\n\n## Context\n\nBecause.\n"
    ),
  )
  .expect("write info.md");
}

pub fn short_dir(tag: &str) -> PathBuf {
  static NEXT: AtomicU32 = AtomicU32::new(0);
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
  /// `userstate::daemon_socket_under` is `<home>/.local/state/intent/run/intentd.sock`
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
    let candidates =
      daemon::candidates_under(&intentsvcs::userstate::Dirs::at_home(&self.home)).ok()?;
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
    self.registered(root).map(|p| p.dispatched).unwrap_or(0)
  }

  /// How many times this daemon has re-read the project at `root` from disk
  /// because its tree changed (`AC-08.5`), read over `Op::Registry`, which is
  /// uncounted, so polling it does not move it.
  ///
  /// `0` for a project this daemon has never opened, as [`Self::dispatched`].
  pub fn ingested(&self, root: &Path) -> u64 {
    self.registered(root).map(|p| p.ingested).unwrap_or(0)
  }

  /// This daemon's registry row for the project at `root`, if it has one.
  ///
  /// **ONE READ OF `Op::Registry` FOR THE THREE COUNTERS ABOVE AND BELOW**, so
  /// they cannot drift about how a root is matched.
  fn registered(&self, root: &Path) -> Option<wire::RegisteredProject> {
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
      .into_iter()
      .find(|p| p.root.canonicalize().unwrap_or_else(|_| p.root.clone()) == wanted)
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
    self.registered(root).map(|p| p.watched).unwrap_or(false)
  }
}

/// Start `intent daemon run` under a given home. **ONE spawn site**, shared by
/// [`RealDaemon::start`] and [`RealDaemon::restart`], so the two cannot drift
/// about the flags, the environment or the redirected output -- a restart that
/// differed from the original start in any of those would be testing a
/// different daemon than the one the test set up.
fn spawn_under(home: &Path) -> Child {
  // **THE LIFELINE (`ST0073`) SURVIVES THE `exec`, WHICH IS WHY IT CAN BE SET
  // HERE AT ALL.** `daemon run` execs `intentd`, so the environment and the
  // inherited stdin both carry across into the daemon -- and on the day that
  // stops being an exec, the daemon becomes a GRANDCHILD and this is the only
  // one of the two mechanisms that still reaches it, because the pipe is
  // inherited down the whole chain while a pid is not.
  intent()
    .args(["daemon", "run"])
    .env("HOME", home)
    .stdin(Stdio::piped())
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
pub fn refuse_a_stale_sibling_daemon() {
  let intent = intent_path();
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

// ---------------------------------------------------------------------------
// The HTTP face, as a client sees it.
//
// **THESE LIVED IN `the_web_face_answers_on_the_published_port.rs` UNTIL
// 2026-09-04, AND MOVED THE MOMENT A SECOND FILE NEEDED THEM.** `AT-17.6` asks
// the same daemon the same way, and a private copy of `http` in each file is
// two answers to *what does this port say* that agree until one of them is
// edited. Moved rather than copied, on the same reasoning `form::triples` was
// moved down a crate.
// ---------------------------------------------------------------------------

pub fn published(daemon: &RealDaemon) -> String {
  let path = intentsvcs::userstate::daemon_address_file_under(
    &intentsvcs::userstate::Dirs::at_home(daemon.home()),
  );
  std::fs::read_to_string(&path)
    .unwrap_or_else(|e| panic!("no address published at {}: {e}", path.display()))
    .trim()
    .to_string()
}

/// This run's secret, read the way a client reads it.
pub fn token(daemon: &RealDaemon) -> String {
  intentsvcs::daemon::Token::read_under(&intentsvcs::userstate::Dirs::at_home(daemon.home()))
    .expect("the daemon published a token")
}

/// One HTTP request over a bare socket, as `(status_line, body)`.
///
/// **HAND-ROLLED RATHER THAN THROUGH A CLIENT CRATE, AND THE REASON IS THE
/// SUBJECT.** This file's claim is about what the daemon does with the FIRST
/// BYTE of a connection; a client library would be free to reuse connections,
/// upgrade, or pipeline, and any of those would put something between the test
/// and the thing it measures. It also adds no dependency to assert a property
/// of a protocol this simple.
pub fn http(addr: &str, request: &str) -> (String, String) {
  let mut socket = TcpStream::connect(addr).expect("connect to the published port");
  socket
    .write_all(request.as_bytes())
    .expect("write the request");
  socket.flush().expect("flush");
  let mut reader = BufReader::new(socket);
  let mut status = String::new();
  reader.read_line(&mut status).expect("a status line");

  let mut length = 0usize;
  loop {
    let mut header = String::new();
    reader.read_line(&mut header).expect("a header line");
    if header.trim().is_empty() {
      break;
    }
    if let Some(value) = header.to_ascii_lowercase().strip_prefix("content-length:") {
      length = value.trim().parse().unwrap_or(0);
    }
  }
  let mut body = vec![0u8; length];
  std::io::Read::read_exact(&mut reader, &mut body).expect("the whole body");
  (
    status.trim().to_string(),
    String::from_utf8_lossy(&body).to_string(),
  )
}

pub fn get(addr: &str, path: &str) -> (String, String) {
  http(
    addr,
    &format!("GET {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n"),
  )
}

/// One `/op` call over HTTP, authorised the way a browser is.
///
/// **THE TOKEN IS PRESENTED AS A CLIENT PRESENTS IT**, because a test that
/// reached past the header would assert the daemon answers requests nobody can
/// actually make. `body` is the whole envelope including `root`, since every
/// op names its project.
pub fn ask_op(daemon: &RealDaemon, body: &str) -> (String, String) {
  let addr = published(daemon);
  let secret = token(daemon);
  http(
    &addr,
    &format!(
      "POST /op HTTP/1.1\r\nHost: localhost\r\nAuthorization: Bearer {secret}\r\n\
       Content-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
      body.len()
    ),
  )
}

/// Build a fake Intent install root in `dir`: the marker, `bin/`, and a COPY of
/// the `intent` binary. Returns the copy as a [`CopiedIntent`], whose one run
/// path retries while Linux reports it busy (issue 0279).
///
/// **THE MARKER COMES FROM [`intentsvcs::install::MARKER`], NOT FROM A
/// LITERAL.** `install::is_install` is exactly `dir.join(MARKER).is_dir()`, so
/// the constant is what MAKES a directory an install root, and a hardcoded copy
/// builds something the resolver does not recognise if it ever moves.
///
/// **COPIED, NOT SYMLINKED**, carried from the caller this was extracted from:
/// the binary must RESOLVE this tree as its own install, and resolution walks
/// up from the executable's real path.
///
/// The caller owns `dir`, which is the shape both callers already had -- one
/// hands in a `TempDir`'s path, the other a path it manages itself. Plugin
/// seeding is [`seed_plugin`], deliberately a separate call: the two callers
/// differed by exactly that step and folding it in would give one of them a
/// directory it never wanted.
pub fn fake_install(dir: &Path) -> CopiedIntent {
  std::fs::create_dir_all(dir.join(intentsvcs::install::MARKER)).expect("marker");
  std::fs::create_dir_all(dir.join("bin")).expect("bin");
  let exe = dir.join("bin/intent");
  std::fs::copy(intent_path(), &exe).expect("copy the binary");
  CopiedIntent(exe)
}

/// The `intent` a [`fake_install`] copied, run only through
/// [`testkit::output_retrying_busy`].
///
/// **THE RETRY IS THE TYPE'S, NOT EACH CALLER'S.** Exec of a just-copied binary
/// meets ETXTBSY on Linux when a sibling test's fork still holds the copy's
/// write fd (the helper's own doc says why). The helper was applied caller by
/// caller twice, and each time a caller it did not reach lost the race on CI
/// (candidate/v3.3.1, run 43296840034, `wb_init_door`). The path is private, so
/// a new caller cannot reach a bare `.output()` on the copy.
pub struct CopiedIntent(PathBuf);

impl CopiedIntent {
  /// Run the copy: `build` is handed its path and returns the command, built
  /// again for each attempt; `what` names the run in a panic.
  pub fn output(
    &self,
    mut build: impl FnMut(&Path) -> Command,
    what: &str,
  ) -> std::process::Output {
    testkit::output_retrying_busy(|| build(&self.0), what)
  }
}

/// Seed a file inside a fake install at `rel`, creating its parent. Returns the
/// path written.
///
/// The second half of `C is A plus a plugin step`: one caller needs a launcher
/// script reachable at its canonical path inside the install, and the other
/// needs no plugin tree at all.
pub fn seed_plugin(root: &Path, rel: &str, src: &Path) -> PathBuf {
  let dest = root.join(rel);
  std::fs::create_dir_all(dest.parent().expect("a plugin path has a parent")).expect("plugin dir");
  std::fs::copy(src, &dest).expect("copy the plugin file");
  dest
}

/// Make `dir` a git work tree, for a test that asks what the next commit would
/// carry.
pub fn git_init(dir: &Path) {
  let git = std::process::Command::new("git")
    .args(["init", "-q"])
    .current_dir(dir)
    .status()
    .expect("run git init");
  assert!(git.success(), "git init");
}

/// Commit everything in `dir`, with no user config, signing or hooks of the
/// machine running the suite. Each `git` runs a literal argv, because this file
/// reads the dispatch table and a spawn here with an argv held in a variable
/// reads to `every_daemon_spawn_carries_a_lifeline` as one that could serve.
pub fn git_commit_all(dir: &Path) {
  let add = std::process::Command::new("git")
    .args(["add", "-A"])
    .current_dir(dir)
    .status()
    .expect("run git add");
  assert!(add.success(), "git add -A");
  let commit = std::process::Command::new("git")
    .args([
      "-c",
      "user.name=t",
      "-c",
      "user.email=t@example.com",
      "-c",
      "commit.gpgSign=false",
      "-c",
      "core.hooksPath=/dev/null",
      "commit",
      "-q",
      "-m",
      "seed",
    ])
    .current_dir(dir)
    .status()
    .expect("run git commit");
  assert!(commit.success(), "git commit");
}

/// Every path `git status` shows changed in `dir`, untracked files included.
pub fn git_changed(dir: &Path) -> std::collections::BTreeSet<String> {
  let out = std::process::Command::new("git")
    .args(["status", "--porcelain", "--untracked-files=all"])
    .current_dir(dir)
    .output()
    .expect("run git status");
  assert!(out.status.success(), "git status");
  String::from_utf8_lossy(&out.stdout)
    .lines()
    .filter_map(|line| line.get(3..))
    .map(str::to_string)
    .collect()
}

/// What a verb printed before the `wrote:` lines that follow its result (issue
/// 0624): the result itself, for a test that holds a verb to its exact words.
pub fn result_lines(out: &str) -> String {
  out
    .lines()
    .take_while(|line| !line.starts_with("wrote: ") && !line.starts_with("re-rendered: "))
    .collect::<Vec<_>>()
    .join("\n")
}

/// The id a mint printed on its one `created:` line (ST0095 WP-02): the whole
/// id as every door takes it back, eg `ST0001`, `ST0001/01` or `0001` today.
/// This is the only place a CLI test learns an id it did not seed, so no test
/// names an id it expects a mint to produce. None, or more than one, fails, and
/// so does an empty id or one with a space in it.
pub fn created(out: &str) -> String {
  let ids: Vec<&str> = out
    .lines()
    .filter_map(|line| line.strip_prefix("created: "))
    .collect();
  assert_eq!(
    ids.len(),
    1,
    "a mint prints exactly one `created:` line, and this printed {}:\n{out}",
    ids.len()
  );
  let id = ids[0];
  assert!(
    !id.is_empty() && !id.contains(char::is_whitespace),
    "a minted id is one word, and this `created:` line carries `{id}`:\n{out}"
  );
  id.to_string()
}

/// **THE ONE HAND-WRITTEN THREAD CANON** (ST0095 WP-10, T1): a 3.4.0-format
/// record, `fields` laid over the minimum every reader takes. The schema is the
/// canon's own constant and the id is `id`, so no test spells either, and each
/// caller sets only what its subject needs.
pub fn thread_canon(id: &str, fields: serde_json::Value) -> String {
  let serde_json::Value::Object(mut canon) = serde_json::json!({
    "schema": intentsvcs::canon_format::THREAD_SCHEMA_V3,
    "id": id,
    "title": "A thread",
    "status": "not-started",
    "created": "2026-08-15",
    "objective": "",
    "context": "",
  }) else {
    unreachable!("a json! object literal is an object");
  };
  let serde_json::Value::Object(over) = fields else {
    panic!("thread_canon overlays an object of fields, got {fields}");
  };
  canon.extend(over);
  format!("{:#}\n", serde_json::Value::Object(canon))
}

/// Write [`thread_canon`] where the project keeps thread `id`'s canon.
pub fn write_thread_canon(root: &std::path::Path, id: &str, fields: serde_json::Value) {
  let project = intentsvcs::project::Project::open(root).expect("a project with its config");
  let path = project.thread_json(id);
  std::fs::create_dir_all(path.parent().expect("the thread canon directory")).expect("mkdir");
  std::fs::write(&path, thread_canon(id, fields)).expect("write the thread's canon");
}

/// Seed an ORDINAL thread, with `packages` (number, title) under it not
/// started, as committed canon, for a test whose subject is the ordinal family.
/// A mint makes git-style ids now (ST0095 WP-06), so an ordinal is what a
/// project carries from before its upgrade, and a test seeds one. Written in
/// 3.4.0's format, which every reader takes by its own schema, so a store that
/// holds nothing yet ingests it on its next open.
pub fn seed_ordinal_thread(
  root: &std::path::Path,
  id: &str,
  title: &str,
  packages: &[(u32, &str)],
) {
  let wps: Vec<serde_json::Value> = packages
    .iter()
    .map(|(seq, title)| {
      serde_json::json!({ "seq": seq, "title": title, "scope": "S", "status": "not-started" })
    })
    .collect();
  write_thread_canon(root, id, serde_json::json!({ "title": title, "wps": wps }));
}

/// Seed the creation event a record's mint wrote, as the committed event file
/// a branch cut before the upgrade carries beside its ordinal (ST0095 WP-06):
/// `id` is the event's own id, `op` the creation op, and `kind` and `subject`
/// the record it made. A repair of an ordinal both sides minted derives the
/// record's git-style id from this event, so a test that seeds one can name
/// the id the repair will choose.
pub fn seed_creation_event(root: &std::path::Path, id: &str, op: &str, kind: &str, subject: &str) {
  let project = intentsvcs::project::Project::open(root).expect("a project with its config");
  let envelope = intentsvcs::event::Envelope {
    id: id.to_string(),
    ts: "2026-08-15T10:00:00.000Z".to_string(),
    principal: "test".to_string(),
    project_id: String::new(),
    op: op.to_string(),
    subject: intentsvcs::event::Subject {
      kind: kind.to_string(),
      id: subject.to_string(),
    },
    payload: serde_json::json!({}),
  };
  let path = project.event_file(&envelope).expect("an event file's path");
  std::fs::create_dir_all(path.parent().expect("the event's directory")).expect("mkdir");
  let bytes = intentsvcs::model::to_canonical_json(&envelope).expect("the event serialises");
  std::fs::write(&path, bytes).expect("write the event file");
}

/// Seed an ORDINAL issue as committed canon, for the reason
/// [`seed_ordinal_thread`] gives.
pub fn seed_ordinal_issue(root: &std::path::Path, number: u32, title: &str) {
  let project = intentsvcs::project::Project::open(root).expect("a project with its config");
  let id = intentsvcs::id::IssueId::from_ordinal(number).expect("an issue ordinal that fits");
  let path = project.issue_json(id);
  std::fs::create_dir_all(path.parent().expect("the issue canon directory")).expect("mkdir");
  let canon = serde_json::json!({
    "schema": intentsvcs::canon_format::ISSUE_SCHEMA_V3,
    "number": number,
    "slug": "an-issue",
    "title": title,
    "status": "open",
    "created": "2026-08-15",
  });
  std::fs::write(&path, format!("{canon:#}\n")).expect("write the issue's canon");
}

/// The canon at `root`, read by the library's own reader, which writes
/// nothing: what a record says, for a test that drove the binary.
pub fn canon(root: &Path) -> intentsvcs::ingest::Canon {
  let project = intentsvcs::project::Project::open(root).expect("the project opens");
  intentsvcs::ingest::read(&project).expect("the canon reads")
}

/// Whether thread `id`'s record says it is realised, in an upgraded project,
/// which keeps realisation on its records (ST0096 WP-01). A 3.4.0 project
/// reads its manifest instead, and its records always answer false here.
pub fn realised_thread(root: &Path, id: &str) -> bool {
  canon(root)
    .threads
    .iter()
    .find(|t| t.id.stored() == id)
    .unwrap_or_else(|| panic!("thread {id} has a record"))
    .realised
}

/// The ids of the threads whose records say they are realised, sorted as
/// stored strings, in an upgraded project (see [`realised_thread`]).
pub fn realised_threads(root: &Path) -> Vec<String> {
  let mut ids: Vec<String> = canon(root)
    .threads
    .iter()
    .filter(|t| t.realised)
    .map(|t| t.id.stored())
    .collect();
  ids.sort();
  ids
}

/// Whether issue `id`'s record says it is realised, in an upgraded project
/// (see [`realised_thread`]).
pub fn realised_issue(root: &Path, id: &str) -> bool {
  canon(root)
    .issues
    .iter()
    .find(|i| i.number.stored() == id)
    .unwrap_or_else(|| panic!("issue {id} has a record"))
    .realised
}

/// The form a table, a view or a message prints of a minted id, from the stored
/// form [`created`] read. A git-style id shows shorter than it is stored
/// (ST0095 WP-06), so a test that looks for a minted id in a line asks for this,
/// read through the waist, and spells neither form by hand. A thread, an issue,
/// or a work package's `<thread>/<package>` address.
pub fn shown(stored: &str) -> String {
  intentsvcs::id::shown_of(stored)
    .unwrap_or_else(|| panic!("`{stored}` is no stored thread, issue or work package id"))
}

/// A minted work package's own id, read by the id waist from the part of its
/// address after its thread's: a test takes the package's directory, its `WP-`
/// heading or its criteria's group from it, and a library call takes it whole.
/// The address is the waist's `wp_address`, so it is split by the waist's
/// `split_wp_address` and its stored form is the only one this reads.
pub fn package_of(wp: &str) -> intentsvcs::id::WpId {
  let (_, package) = intentsvcs::id::split_wp_address(wp)
    .unwrap_or_else(|| panic!("`{wp}` is not a work package's address"));
  intentsvcs::id::WpId::parse(package)
    .unwrap_or_else(|e| panic!("`{wp}` does not end in a package's stored id: {e}"))
}

/// Run the binary under test in `cwd` under the fixture home, with a lifeline
/// on stdin for a daemon verb: stdout, stderr and the exit code.
pub fn run(cwd: &Path, args: &[&str]) -> (String, String, i32) {
  let out = intent()
    .args(args)
    .current_dir(cwd)
    .env("HOME", testkit::fixture_home())
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run the v3 binary");
  (
    String::from_utf8_lossy(&out.stdout).into_owned(),
    String::from_utf8_lossy(&out.stderr).into_owned(),
    out.status.code().unwrap_or(-1),
  )
}

/// [`run`], failing the test unless it exits 0: its stdout.
pub fn ok(cwd: &Path, args: &[&str]) -> String {
  let (out, err, code) = run(cwd, args);
  assert_eq!(code, 0, "`intent {}` failed: {err}", args.join(" "));
  out
}

/// Every file under a project's committed canon with its bytes, through the
/// library's one walk: for an arm that shows a refusal wrote nothing. Bytes,
/// not paths alone, because in 3.4.0's format a package is inside its thread's
/// file, so a write changes a file rather than adding one.
pub fn canon_files(root: &Path) -> std::collections::BTreeMap<PathBuf, Vec<u8>> {
  let canon = root.join("intent/.canon");
  intentsvcs::project::Project::files_in(&canon)
    .into_iter()
    .map(|rel| {
      let bytes = std::fs::read(canon.join(&rel)).expect("read a canon file");
      (rel, bytes)
    })
    .collect()
}

/// A run's two streams as one text, with whether it exited 0: for an arm that
/// reads a refusal's text as readily as a success's. The one copy of what six
/// test files each defined for themselves.
pub fn run_joined(cwd: &Path, args: &[&str]) -> (bool, String) {
  let said = Said::from(run(cwd, args));
  (said.code == 0, said.text)
}

/// What a run printed, both streams as one text, and its exit code: for an
/// arm that reads a line whichever stream carries it, as a git hook's line is
/// on stderr while the command's own is on stdout.
pub struct Said {
  pub text: String,
  pub code: i32,
}

impl From<(String, String, i32)> for Said {
  fn from((out, err, code): (String, String, i32)) -> Said {
    Said {
      text: format!("{out}{err}"),
      code,
    }
  }
}

/// The system's own directories, with no `intent` in them: what a forge's
/// `PATH` ends with, and all of it for [`Forge::git_without_intent`].
const SYSTEM_PATH: &str = "/usr/bin:/bin";

/// **A FORGE STAND-IN** (ST0096 WP-04): a bare repository holding `main`, the
/// clones and worktrees that open merge requests against it, and the merge a
/// forge's merge button runs, done in the bare repository by git alone.
///
/// Every git here, and every git the binary runs for it, reads one config, the
/// forge's own HOME's, which carries an identity, no hooks, no signing and no
/// background maintenance; the system config and every inherited `GIT_*`
/// variable are kept out ([`Forge::isolate`]). So the fixture's commits are the
/// commits, and a suite run under a hook or a `rebase -x` cannot point git at
/// the outer repository. Every `intent` runs under that HOME through
/// [`intent`]'s door. Everything lives under one tempdir, removed when the
/// forge drops.
///
/// **NO HOOKS BY DEFAULT, SO A CLEAN `doctor` HERE SAYS NOTHING OF HOOK
/// INSTALLATION.** [`Forge::new`]'s `core.hooksPath` is `/dev/null`, which
/// the binary reads too. A test whose subject is the hooks takes
/// [`Forge::with_hooks`] and wires them itself. **BUT A WORKING COPY HERE
/// BEHAVES AS ONE WITH ITS HOOKS WIRED**: every branch switch and every pull
/// through [`Forge::switch`] and [`Forge::pull`] goes through
/// [`Forge::after_checkout`], which runs what the shipped hook runs. A fresh
/// checkout, a clone or a new worktree, does not, because the hook skips one
/// too and leaves the store to the first verb. Without it, a copy that
/// switches branches keeps the old branch's records in its store and
/// re-renders them into the new one (issue 0739).
///
/// **THE ONE HOME FOR A MULTI-CLONE FIXTURE (issue 0737).** Every clone,
/// worktree, push and fixture commit in this crate's two-clone tests is made
/// here, so a clone is made one way.
pub struct Forge {
  dir: tempfile::TempDir,
  home: PathBuf,
  hooks: bool,
}

impl Forge {
  /// A bare `forge.git`, with `main` not yet born, and the forge's HOME, whose
  /// git runs no hook.
  pub fn new() -> Forge {
    Forge::made(false)
  }

  /// [`Forge::new`], but **THE FORGE'S GIT RUNS THE HOOKS A COPY WIRES**: its
  /// HOME's config leaves `core.hooksPath` unset, and its `PATH` puts the
  /// binary under test first, so a hook that runs `intent` runs this one.
  pub fn with_hooks() -> Forge {
    Forge::made(true)
  }

  fn made(hooks: bool) -> Forge {
    let dir = tempfile::tempdir().expect("the forge's tempdir");
    let home = dir.path().join("home");
    std::fs::create_dir_all(&home).expect("the forge's HOME");
    let core = if hooks {
      ""
    } else {
      "[core]\n  hooksPath = /dev/null\n"
    };
    std::fs::write(
      home.join(".gitconfig"),
      format!(
        "[user]\n  name = t\n  email = t@example.com\n{core}\
         [commit]\n  gpgSign = false\n[init]\n  defaultBranch = main\n[maintenance]\n  auto = false\n"
      ),
    )
    .expect("git's config for the forge's commits");
    let forge = Forge { dir, home, hooks };
    forge.git(forge.dir.path(), &["init", "-q", "--bare", "forge.git"]);
    forge
  }

  /// Where a working copy named `name` lives: beside the bare repository, in
  /// the forge's one tempdir.
  pub fn path(&self, name: &str) -> PathBuf {
    self.dir.path().join(name)
  }

  /// The bare repository, where only a merge is ever made.
  pub fn bare(&self) -> PathBuf {
    self.path("forge.git")
  }

  /// A clone of the forge, beside it, named `name`. A fresh checkout, so no
  /// sync, as the hook runs none: its first verb builds its store.
  pub fn clone_as(&self, name: &str) -> PathBuf {
    self.git(self.dir.path(), &["clone", "-q", "forge.git", name]);
    self.path(name)
  }

  /// A worktree of `clone`, beside it, named `name`, detached at the forge's
  /// `main` as `clone` last fetched it. A fresh checkout, so no sync: the
  /// hook sees the null previous HEAD `git worktree add` passes and runs
  /// nothing (post-pull.sh), and the worktree's first verb builds its store.
  pub fn worktree_of(&self, clone: &Path, name: &str) -> PathBuf {
    self.worktree_said(clone, name, "origin/main").0
  }

  /// A worktree of `clone`, beside it, named `name`, detached at `at`, with
  /// what `git worktree add` said on both streams, where a hook a copy wired
  /// prints: the one way a worktree is made here.
  pub fn worktree_said(&self, clone: &Path, name: &str, at: &str) -> (PathBuf, Said) {
    let path = self.path(name);
    let to = path.to_str().expect("a UTF-8 tempdir");
    let said = self.git_said(clone, &["worktree", "add", "-q", "--detach", to, at]);
    (path, said)
  }

  /// `git switch <args>` in `copy`, then [`Forge::after_checkout`]: the one
  /// way a working copy here changes branch.
  pub fn switch(&self, copy: &Path, args: &[&str]) {
    let mut switch = vec!["switch", "-q"];
    switch.extend_from_slice(args);
    self.git(copy, &switch);
    self.after_checkout(copy);
  }

  /// `git pull --ff-only` of the forge's `main` into `copy`, then
  /// [`Forge::after_checkout`].
  pub fn pull(&self, copy: &Path) {
    self.git(copy, &["pull", "-q", "--ff-only", "origin", "main"]);
    self.after_checkout(copy);
  }

  /// **WHAT THE SHIPPED HOOK RUNS AFTER A BRANCH SWITCH OR A MERGE**:
  /// `intent sync --apply`, the one command `lib/templates/hooks/post-pull.sh`
  /// runs on post-checkout and post-merge. On a [`Forge::new`] forge the
  /// copies have no hooks, so this stands for them, and a copy here is one
  /// whose hooks are wired. **A [`Forge::with_hooks`] forge is refused**: its
  /// wired hook would run the sync as well, and an arm could credit this with
  /// what the hook did. The hookless path is issue 0739: a copy that switches
  /// branches without it keeps the old branch's records in its store, and
  /// since 0739 landed its next write is refused before anything is written,
  /// naming `intent sync --apply` as the remedy.
  ///
  /// **STRICTER THAN THE HOOK, ON PURPOSE: ONE SYNC LEAVES NOTHING.** The hook
  /// exits 0 whatever sync says. Here the sync must leave no step (`left:`),
  /// a fresh `intent sync` straight after must plan nothing, quiet steps
  /// included, doctor must count nothing, and the working copy must be clean.
  /// Otherwise the test fails at the checkout that caused it. It holds because
  /// sync plans each step from the state the steps before it will leave (issue
  /// 0741): planned from the store as it stood, a pull of records another copy
  /// minted left a views step that did nothing, and a switch away from records
  /// missed an index step its own steps made due.
  ///
  /// **AND THE PLAN IS WHAT THE RUN DOES** (issue 0742): the views step a dry
  /// `intent sync` shows names the writes and removes the sync then reports,
  /// and is absent when it writes and removes nothing. Planned before the
  /// ingest step had written them, it promised writes the run did not make.
  fn after_checkout(&self, copy: &Path) {
    assert!(
      !self.hooks,
      "after_checkout stands for the hook only on a Forge::new forge; a Forge::with_hooks copy runs its own hook on a checkout"
    );
    let planned = self.intent_ok(copy, &["sync"]);
    let synced = self.intent_ok(copy, &["sync", "--apply"]);
    let fresh = self.intent_ok(copy, &["sync"]);
    let steps: Vec<&str> = fresh
      .lines()
      .map(str::trim_start)
      .filter(|line| {
        line
          .split_once(". ")
          .is_some_and(|(n, _)| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
      })
      .collect();
    assert!(
      !synced
        .lines()
        .any(|line| line.trim_start().starts_with("left: ")),
      "the sync in {} left a step for a person:\n{synced}",
      copy.display()
    );
    assert_eq!(
      views_planned(&planned),
      views_done(&synced),
      "in {}, the dry run's views step is not what the sync did:\n{planned}\n{synced}",
      copy.display()
    );
    assert!(
      steps.is_empty(),
      "after the sync in {}, a fresh plan still holds a step:\n{fresh}\n{synced}",
      copy.display()
    );
    let (doctor, err, code) = self.intent(copy, &["doctor"]);
    assert!(
      code == 0 && doctor.contains("doctor: 0 finding(s)"),
      "after the sync in {}, doctor counts a finding:\n{doctor}{err}\n{synced}",
      copy.display()
    );
    let changed = self.git(copy, &["status", "--porcelain", "--untracked-files=all"]);
    assert!(
      changed.trim().is_empty(),
      "the sync after a checkout in {} changed the working copy:\n{changed}\n{synced}",
      copy.display()
    );
  }

  /// The forge's HOME on `command`, which came through testkit's door and so
  /// carries no per-user variable of the machine running the suite, with the
  /// system git config off and every inherited `GIT_*` variable removed: the
  /// one isolation both the forge's git, with any hook it runs, and the binary
  /// under test get. On a [`Forge::with_hooks`] forge, `PATH` is the binary's
  /// directory and then the system's, so a hook finds this `intent` and no
  /// other.
  fn isolate(&self, command: &mut Command) {
    command.env("HOME", &self.home);
    for (name, _) in std::env::vars_os() {
      if name.to_string_lossy().starts_with("GIT_") {
        command.env_remove(name);
      }
    }
    command.env("GIT_CONFIG_NOSYSTEM", "1");
    if self.hooks {
      let bin = intent_path().parent().expect("the binary's directory");
      command.env("PATH", format!("{}:{SYSTEM_PATH}", bin.display()));
    }
  }

  /// git in `cwd`, isolated, on `path` when one is given: its stdout, its
  /// stderr and its exit code.
  fn git_on(&self, cwd: &Path, args: &[&str], path: Option<&str>) -> (String, String, i32) {
    let mut command = testkit::fixtured_command("git");
    self.isolate(&mut command);
    if let Some(path) = path {
      command.env("PATH", path);
    }
    command.current_dir(cwd);
    // LIFELINE-EXEMPT: this runs git with a git argv, never the CLI's, so it
    // starts no daemon whatever the argv holds.
    command.args(args);
    let out = command.output().expect("run git");
    (
      String::from_utf8_lossy(&out.stdout).into_owned(),
      String::from_utf8_lossy(&out.stderr).into_owned(),
      out.status.code().unwrap_or(-1),
    )
  }

  /// git in `cwd`: its stdout, its stderr and its exit code.
  pub fn git_try(&self, cwd: &Path, args: &[&str]) -> (String, String, i32) {
    self.git_on(cwd, args, None)
  }

  /// [`Forge::git_try`] with no `intent` on `PATH`, so a hook that runs one
  /// cannot find it.
  pub fn git_without_intent(&self, cwd: &Path, args: &[&str]) -> (String, String, i32) {
    self.git_on(cwd, args, Some(SYSTEM_PATH))
  }

  /// [`Forge::git_try`], failing the test unless git exits 0: its stdout and
  /// its stderr. The one place a forge's git must succeed.
  fn git_ok(&self, cwd: &Path, args: &[&str]) -> (String, String) {
    let (out, err, code) = self.git_try(cwd, args);
    assert_eq!(
      code,
      0,
      "`git {}` in {} failed:\n{out}{err}",
      args.join(" "),
      cwd.display()
    );
    (out, err)
  }

  /// git in `cwd`, failing the test unless it exits 0: its stdout.
  pub fn git(&self, cwd: &Path, args: &[&str]) -> String {
    self.git_ok(cwd, args).0
  }

  /// git in `cwd`, failing the test unless it exits 0: both streams as one
  /// text, for an arm that reads what a hook printed.
  pub fn git_said(&self, cwd: &Path, args: &[&str]) -> Said {
    let (out, err) = self.git_ok(cwd, args);
    Said::from((out, err, 0))
  }

  /// The binary under test in `cwd`, isolated as the forge's git is: stdout,
  /// stderr and the exit code.
  pub fn intent(&self, cwd: &Path, args: &[&str]) -> (String, String, i32) {
    let mut command = intent();
    command.current_dir(cwd);
    self.isolate(&mut command);
    let out = command
      .args(args)
      .stdin(testkit::lifeline_for(args))
      .output()
      .expect("run the v3 binary");
    (
      String::from_utf8_lossy(&out.stdout).into_owned(),
      String::from_utf8_lossy(&out.stderr).into_owned(),
      out.status.code().unwrap_or(-1),
    )
  }

  /// [`Forge::intent`], both streams as one text, whatever it exits.
  pub fn intent_said(&self, cwd: &Path, args: &[&str]) -> Said {
    Said::from(self.intent(cwd, args))
  }

  /// [`Forge::intent`], failing the test unless it exits 0: its stdout.
  pub fn intent_ok(&self, cwd: &Path, args: &[&str]) -> String {
    let (out, err, code) = self.intent(cwd, args);
    assert_eq!(
      code,
      0,
      "`intent {}` in {} failed:\n{out}{err}",
      args.join(" "),
      cwd.display()
    );
    out
  }

  /// `git commit -q --no-verify` in `cwd`, then `args`: the one way a fixture
  /// commits. **`--no-verify` BECAUSE THE COMMIT GATE IS NEVER A FORGE TEST'S
  /// SUBJECT**, and on a [`Forge::with_hooks`] forge the gate a copy wires
  /// needs an install pointer the forge's HOME carries only once a test runs
  /// `bootstrap`.
  pub fn commit(&self, cwd: &Path, args: &[&str]) {
    let mut commit = vec!["commit", "-q", "--no-verify"];
    commit.extend_from_slice(args);
    self.git(cwd, &commit);
  }

  /// Everything in `cwd`, added and committed as `message`.
  pub fn commit_all(&self, cwd: &Path, message: &str) {
    self.git(cwd, &["add", "-A"]);
    self.commit(cwd, &["-m", message]);
  }

  /// Commit, in `cwd`, exactly the paths `outs` printed under `wrote:` and
  /// `removed:`, and nothing else, then hold that nothing was left behind: a
  /// file a verb wrote and did not name would be missing from the merge
  /// request, so the working copy must be clean once the named paths are in.
  pub fn commit_printed(&self, cwd: &Path, outs: &[&str], message: &str) {
    let paths: std::collections::BTreeSet<&str> = outs
      .iter()
      .flat_map(|out| out.lines())
      .filter_map(|line| {
        line
          .strip_prefix("wrote: ")
          .or_else(|| line.strip_prefix("removed: "))
      })
      .collect();
    assert!(
      !paths.is_empty(),
      "the verbs named no file to commit:\n{}",
      outs.join("\n")
    );
    let mut add = vec!["add", "-A", "--"];
    add.extend(paths.iter().copied());
    self.git(cwd, &add);
    self.commit(cwd, &["-m", message]);
    let left = self.git(cwd, &["status", "--porcelain", "--untracked-files=all"]);
    assert!(
      left.trim().is_empty(),
      "the verbs wrote files they did not name, left out of `{message}`:\n{left}"
    );
  }

  /// Merge `branch` into `main` as a forge's merge button does: in the bare
  /// repository, by git alone, with no merge driver and no `intent`.
  /// `merge-tree --write-tree` makes the merged tree, `commit-tree` a commit
  /// with both tips as its parents, and `update-ref` moves `main` from the tip
  /// it merged against. A conflict leaves `main` where it was and hands back
  /// every conflicted path; any other failure of `merge-tree` fails the test
  /// as itself, never as a conflict.
  pub fn merge_try(&self, branch: &str) -> Result<(), Vec<String>> {
    let bare = self.bare();
    let main = self.git(&bare, &["rev-parse", "refs/heads/main"]);
    let main = main.trim();
    let head = format!("refs/heads/{branch}");
    let tip = self.git(&bare, &["rev-parse", head.as_str()]);
    let tip = tip.trim();
    let (out, err, code) = self.git_try(
      &bare,
      &[
        "merge-tree",
        "--write-tree",
        "--name-only",
        "--no-messages",
        main,
        tip,
      ],
    );
    // merge-tree's own contract: 0 a clean merge, 1 a conflicted one, and
    // anything else a merge it could not attempt.
    assert!(
      code == 0 || code == 1,
      "`git merge-tree` could not merge `{branch}` (exit {code}):\n{out}{err}"
    );
    let mut lines = out.lines();
    let tree = lines.next().unwrap_or_default();
    let conflicted: Vec<String> = lines
      .filter(|line| !line.is_empty())
      .map(str::to_string)
      .collect();
    assert_eq!(
      code == 1,
      !conflicted.is_empty(),
      "`git merge-tree` exited {code} and named {conflicted:?} for `{branch}`:\n{out}{err}"
    );
    conflicted.is_empty().then_some(()).ok_or(conflicted)?;
    let message = format!("Merge {branch}");
    let merged = self.git(
      &bare,
      &[
        "commit-tree",
        tree,
        "-p",
        main,
        "-p",
        tip,
        "-m",
        message.as_str(),
      ],
    );
    self.git(
      &bare,
      &["update-ref", "refs/heads/main", merged.trim(), main],
    );
    Ok(())
  }

  /// [`Forge::merge_try`], failing the test on a conflict, naming the branch
  /// and every conflicted path.
  pub fn merge(&self, branch: &str) {
    let merged = self.merge_try(branch);
    assert!(
      merged.is_ok(),
      "merging `{branch}` into main conflicts in: {}",
      merged.err().unwrap_or_default().join(", ")
    );
  }
}

/// The writes and removes a dry `intent sync`'s views step names, or none when
/// it plans no views step (issue 0742).
fn views_planned(plan: &str) -> (usize, usize) {
  let count = |text: &str| -> usize {
    text
      .split_whitespace()
      .next()
      .and_then(|n| n.trim_end_matches(':').parse().ok())
      .unwrap_or_else(|| panic!("a count in {text:?}"))
  };
  // The step by its name, `  N. views (<recoverability>): ...`, and a views
  // step whose writes do not parse fails here rather than reading as none.
  let step = plan.lines().map(str::trim_start).find(|line| {
    line
      .split_once(". ")
      .is_some_and(|(_, rest)| rest.starts_with("views ("))
  });
  let Some(step) = step else {
    return (0, 0);
  };
  let (_, said) = step
    .split_once(": write ")
    .unwrap_or_else(|| panic!("a views step names its writes: {step}"));
  match said.split_once(" and remove ") {
    Some((writes, removes)) => (count(writes), count(removes)),
    None => (count(said), 0),
  }
}

/// The writes and removes `intent sync --apply` reports its views step made,
/// or none when it ran no views step. The CLI prints it as
/// `ok: views: wrote N and removed M`.
fn views_done(applied: &str) -> (usize, usize) {
  applied
    .lines()
    .find_map(|line| line.trim_start().strip_prefix("ok: views: wrote "))
    .map(|said| {
      let (wrote, removed) = said
        .split_once(" and removed ")
        .expect("views: wrote N and removed M");
      (
        wrote
          .trim()
          .parse()
          .unwrap_or_else(|_| panic!("a count in {wrote:?}")),
        removed
          .trim()
          .parse()
          .unwrap_or_else(|_| panic!("a count in {removed:?}")),
      )
    })
    .unwrap_or((0, 0))
}

/// The remedy doctor prints for the group `member` is listed under: the
/// nearest `  remedy: ` line above it, without its lead (issue 0740). Doctor
/// prints a group's one remedy beneath its header and above its members
/// (`render::print_grouped`), so a finding's own line carries no remedy.
pub fn remedy_above<'a>(out: &'a str, member: &str) -> &'a str {
  let lines: Vec<&str> = out.lines().collect();
  let at = lines
    .iter()
    .position(|line| *line == member)
    .unwrap_or_else(|| panic!("{member:?} is not a line of:\n{out}"));
  lines[..at]
    .iter()
    .rev()
    .find_map(|line| line.strip_prefix("  remedy: "))
    .unwrap_or_else(|| panic!("no remedy line above {member:?} in:\n{out}"))
}
