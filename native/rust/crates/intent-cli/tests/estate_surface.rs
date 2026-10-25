//! ST0086 WP-01's SURFACE: `intent estate status`, `wip` and `prune`, and the
//! two MCP tools, driven through the real binary; and WP-03's `/estate` in the
//! explorer, driven through its key machine.
//!
//! **THE LOGIC IS `intentsvcs::estate`'s AND ITS OWN TESTS OWN IT** (vc's split,
//! 2026-10-24). These arms prove what only the surface can get wrong: that the
//! terminal form and `--format json` render the reader's answer, that an
//! unreadable session file is a warning and not a failure, that `wip` refuses
//! the two ambiguous forms with a remedy, that `prune` never touches Claude
//! Code's own files, and that the MCP tool answers the same value.
//!
//! The fixture is a whole host in a tempdir: a HOME, a Claude directory whose
//! `sessions/<pid>.json` files name REAL pids -- this test process, alive, and a
//! child that has already exited, gone, each with its start read through the
//! estate module -- and one Intent project with a board
//! whose session id matches a session file.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::common::level_three::INITIALIZE;
use crate::common::{intent, mcp_session};

/// `pid`'s start in the form Claude Code writes as `procStart` on THIS OS.
///
/// **TAKEN FROM THE ESTATE MODULE, NEVER RE-DERIVED HERE.** The form is per OS
/// (ps's `lstart` on macOS, the kernel's start time in clock ticks on Linux,
/// kovacs-sysadm's measurement of 2026-10-24), and it has one home. A fixture
/// that computed its own would agree with the reader only on the OS it was
/// written on. A pid that is not running has no start, and any value marks it.
fn proc_start(pid: u32) -> String {
  intentsvcs::estate::proc_start_of(pid)
    .expect("read a process's start")
    .unwrap_or_else(|| "not running".to_string())
}

/// A pid that has exited: a child spawned and waited for.
fn dead_pid() -> u32 {
  let mut child = Command::new("/usr/bin/true").spawn().expect("spawn true");
  let pid = child.id();
  child.wait().expect("wait for true");
  pid
}

struct Host {
  _dir: tempfile::TempDir,
  home: PathBuf,
  claude: PathBuf,
  project: PathBuf,
}

impl Host {
  fn new() -> Host {
    let dir = tempfile::tempdir().expect("the host's tempdir");
    let home = dir.path().join("home");
    let claude = home.join(".claude");
    let project = dir.path().join("proj");
    std::fs::create_dir_all(claude.join("sessions")).expect("mkdir sessions");
    std::fs::create_dir_all(&project).expect("mkdir project");
    let init = intent()
      .env("HOME", &home)
      .arg("init")
      .current_dir(&project)
      .output()
      .expect("run intent init");
    assert!(
      init.status.success(),
      "intent init: {}",
      String::from_utf8_lossy(&init.stderr)
    );
    Host {
      _dir: dir,
      home,
      claude,
      project,
    }
  }

  /// A `Command` for `intent` on this host: its HOME and its Claude directory.
  fn cmd(&self) -> Command {
    let mut c = intent();
    c.env("HOME", &self.home)
      .env("CLAUDE_CONFIG_DIR", &self.claude)
      .env_remove("CLAUDE_CODE_SESSION_ID");
    c
  }

  /// One Claude Code session file, in the shape Claude Code writes it.
  fn session(&self, pid: u32, session_id: &str, name: &str, cwd: &Path, status: &str) -> PathBuf {
    let path = self.claude.join("sessions").join(format!("{pid}.json"));
    let body = serde_json::json!({
      "pid": pid,
      "sessionId": session_id,
      "cwd": cwd,
      "startedAt": 1_792_820_660_805_u64,
      "procStart": proc_start(pid),
      "version": "2.1.310",
      "kind": "interactive",
      "entrypoint": "cli",
      "name": name,
      "status": status,
      "updatedAt": 1_792_848_120_372_u64,
      "statusUpdatedAt": 1_792_848_120_372_u64,
    });
    std::fs::write(&path, serde_json::to_vec_pretty(&body).unwrap()).expect("write a session file");
    path
  }

  /// A board for `node` in the project, carrying `session_id` in its header.
  fn board(&self, node: &str, session_id: &str, focus: &str) {
    let dir = self.project.join("intent/whiteboard").join(node);
    std::fs::create_dir_all(&dir).expect("mkdir the board");
    let body = serde_json::json!({
      "schema": "intent/board@4.0",
      "node": {
        "moniker": node,
        "name": "Test Node",
        "role": "worker",
        "session_id": session_id,
        "heartbeat_at": "2026-10-24T13:00:00.000Z",
        "status": "active",
        "focus": focus,
        "claims": ["ST0001"],
        "recorded_at": "2026-10-24T13:00:00.000Z",
      },
      "items": [],
      "sent": [],
      "handled": [],
    });
    std::fs::write(
      dir.join("board.json"),
      serde_json::to_vec_pretty(&body).unwrap(),
    )
    .expect("write board.json");
  }

  fn status_json(&self) -> serde_json::Value {
    let out = self
      .cmd()
      .args(["estate", "status", "--format", "json"])
      .output()
      .expect("run estate status");
    assert!(
      out.status.success(),
      "estate status --format json: {}",
      String::from_utf8_lossy(&out.stderr)
    );
    serde_json::from_slice(&out.stdout).expect("estate status --format json prints one JSON value")
  }
}

fn row<'a>(v: &'a serde_json::Value, name: &str) -> &'a serde_json::Value {
  v["sessions"]
    .as_array()
    .expect("sessions is a list")
    .iter()
    .find(|r| r["name"] == name)
    .unwrap_or_else(|| panic!("no session row named {name} in {v:#}"))
}

/// AT-01.1: **the reader returns one value, schema `intent/estate-status@1`,
/// with every key present and typed, and a later package's keys at the empty
/// value of their shape.**
#[test]
fn status_json_carries_every_key_of_the_boundary_from_the_first_work_package() {
  let host = Host::new();
  let v = host.status_json();
  for key in [
    "schema",
    "host",
    "version",
    "taken_at",
    "census",
    "controller",
    "lanes",
    "queue",
    "recent",
    "usage",
    "sessions",
    "unreadable",
  ] {
    assert!(v.get(key).is_some(), "the boundary has no `{key}`: {v:#}");
  }
  assert_eq!(v["schema"], "intent/estate-status@1");
}

/// AT-01.3 and AT-01.5: **project, node, focus and claims are joined from
/// the board whose session id matches, never stored, and a session whose pid
/// is dead renders as gone.**
#[test]
fn status_renders_a_live_node_with_its_board_and_a_gone_session_as_gone() {
  let host = Host::new();
  let me = std::process::id();
  host.session(me, "sess-live", "proj-ic", &host.project, "busy");
  host.board("ic", "sess-live", "building the estate surface");
  let gone = dead_pid();
  host.session(gone, "sess-gone", "proj-dc", &host.project, "idle");

  let v = host.status_json();
  let live = row(&v, "proj-ic");
  assert_eq!(live["state"], "busy");
  assert_eq!(live["node"], "ic");
  assert_eq!(live["line"], "building the estate surface");
  assert_eq!(row(&v, "proj-dc")["state"], "gone");

  let out = host
    .cmd()
    .args(["estate", "status"])
    .output()
    .expect("run estate status");
  assert!(out.status.success());
  let text = String::from_utf8_lossy(&out.stdout);
  let line = text
    .lines()
    .find(|l| l.starts_with("proj-ic"))
    .unwrap_or_else(|| panic!("no proj-ic row in:\n{text}"));
  for cell in ["ic", "busy", "ST0001", "building the estate surface"] {
    assert!(
      line.contains(cell),
      "the proj-ic row lacks `{cell}`: {line}"
    );
  }
  assert!(
    text
      .lines()
      .next()
      .is_some_and(|h| h.starts_with("estate ")),
    "the terminal form opens with its header line:\n{text}"
  );
}

/// AT-01.5: **a session file that cannot be read is listed as unreadable,
/// and nobody is called gone on its account.**
#[test]
fn an_unreadable_session_file_is_a_warning_on_stderr_and_the_verb_still_answers() {
  let host = Host::new();
  host.session(
    std::process::id(),
    "sess-ok",
    "readable",
    &host.project,
    "idle",
  );
  let bad = host.claude.join("sessions").join("99999999.json");
  // A session file missing a pinned field: `cwd` is gone.
  std::fs::write(
    &bad,
    r#"{"pid": 99999999, "sessionId": "x", "name": "broken"}"#,
  )
  .expect("write a malformed session file");

  let out = host
    .cmd()
    .args(["estate", "status"])
    .output()
    .expect("run estate status");
  assert!(
    out.status.success(),
    "one unreadable file must not fail the verb: {}",
    String::from_utf8_lossy(&out.stderr)
  );
  let err = String::from_utf8_lossy(&out.stderr);
  assert!(
    err
      .lines()
      .any(|l| l.starts_with("warning: ") && l.contains("99999999.json")),
    "the unreadable file is named on stderr:\n{err}"
  );
  assert!(String::from_utf8_lossy(&out.stdout).contains("readable"));

  let v = host.status_json();
  let unreadable = v["unreadable"].as_array().expect("unreadable is a list");
  assert!(
    unreadable.iter().any(|u| u["path"]
      .as_str()
      .is_some_and(|p| p.ends_with("99999999.json"))),
    "the json form carries the same file under `unreadable`: {v:#}"
  );
}

/// AT-01.4: **`estate wip` writes only Intent's own session record, whole,
/// and prints `wrote:`.**
#[test]
fn wip_writes_a_line_the_status_shows_and_clear_removes_it() {
  let host = Host::new();
  let outside = host.home.join("elsewhere");
  std::fs::create_dir_all(&outside).unwrap();
  host.session(
    std::process::id(),
    "sess-coord",
    "coordinator",
    &outside,
    "busy",
  );

  let set = host
    .cmd()
    .args([
      "estate",
      "wip",
      "judging asks on the slot desk",
      "--session",
      "sess-coord",
    ])
    .output()
    .expect("run estate wip");
  assert!(
    set.status.success(),
    "{}",
    String::from_utf8_lossy(&set.stderr)
  );
  let said = String::from_utf8_lossy(&set.stdout);
  let path = said
    .trim()
    .strip_prefix("wrote: ")
    .unwrap_or_else(|| panic!("estate wip names what it wrote: {said}"));
  assert!(
    Path::new(path).is_file(),
    "the record it named exists: {path}"
  );

  let v = host.status_json();
  let coord = row(&v, "coordinator");
  assert_eq!(coord["line"], "judging asks on the slot desk");
  assert_eq!(coord["line_is_wip"], true);
  assert!(coord["node"].is_null());

  let clear = host
    .cmd()
    .args(["estate", "wip", "--clear", "--session", "sess-coord"])
    .output()
    .expect("run estate wip --clear");
  assert!(
    clear.status.success(),
    "{}",
    String::from_utf8_lossy(&clear.stderr)
  );
  assert_eq!(
    String::from_utf8_lossy(&clear.stdout).trim(),
    format!("removed: {path}")
  );
  assert!(!Path::new(path).exists(), "--clear removed the record");
}

/// AT-01.7: **an estate verb's error is typed and carries a remedy.**
#[test]
fn wip_refuses_with_a_remedy_when_it_cannot_tell_what_was_meant() {
  let host = Host::new();
  for args in [
    vec!["estate", "wip", "--session", "s"],
    vec!["estate", "wip", "a line", "--clear", "--session", "s"],
    vec!["estate", "wip", "a line"],
  ] {
    let out = host
      .cmd()
      .args(&args)
      .stdin(testkit::lifeline_for(&args))
      .output()
      .expect("run estate wip");
    assert_eq!(out.status.code(), Some(1), "{args:?} refuses at 1");
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(err.contains("remedy:"), "{args:?} names a remedy:\n{err}");
  }
}

/// AT-01.7: **an estate verb's error is typed and carries a remedy.**
///
/// ST0086 WP-02: `--browser` refuses rather than opening a page nothing serves,
/// and refuses alongside `--format`, which asks for the opposite thing; each
/// refusal names its remedy and no browser is opened by either.
#[test]
fn status_browser_refuses_with_no_daemon_and_beside_a_format() {
  let host = Host::new();
  let out = host
    .cmd()
    .args(["estate", "status", "--browser"])
    .output()
    .expect("run estate status --browser");
  assert_eq!(
    out.status.code(),
    Some(1),
    "no daemon answers, so it refuses"
  );
  let err = String::from_utf8_lossy(&out.stderr);
  assert!(
    err.contains("no `intentd` is answering") && err.contains("intent daemon start"),
    "the refusal does not say no daemon answers and how to start one:\n{err}"
  );

  let out = host
    .cmd()
    .args(["estate", "status", "--browser", "--format", "json"])
    .output()
    .expect("run estate status --browser --format json");
  assert_eq!(out.status.code(), Some(1), "the two flags together refuse");
  let err = String::from_utf8_lossy(&out.stderr);
  assert!(
    err.contains("--browser") && err.contains("--format") && err.contains("remedy:"),
    "the refusal does not name both flags and a remedy:\n{err}"
  );
  assert!(
    out.stdout.is_empty(),
    "it printed an answer as well as refusing"
  );
}

/// AT-01.4: **`estate prune` removes only Intent's own records for gone
/// sessions, prints `removed:` per file, and touches no Claude Code file.**
#[test]
fn prune_removes_a_gone_sessions_record_and_never_touches_the_claude_dir() {
  let host = Host::new();
  let gone = dead_pid();
  let file = host.session(gone, "sess-ended", "ended", &host.home, "idle");
  let set = host
    .cmd()
    .args(["estate", "wip", "was here", "--session", "sess-ended"])
    .output()
    .expect("run estate wip");
  assert!(
    set.status.success(),
    "{}",
    String::from_utf8_lossy(&set.stderr)
  );
  let record = String::from_utf8_lossy(&set.stdout)
    .trim()
    .strip_prefix("wrote: ")
    .map(PathBuf::from)
    .expect("wip names its record");
  let before = std::fs::read(&file).expect("read the session file");

  let out = host
    .cmd()
    .args(["estate", "prune"])
    .output()
    .expect("run estate prune");
  assert!(
    out.status.success(),
    "{}",
    String::from_utf8_lossy(&out.stderr)
  );
  let said = String::from_utf8_lossy(&out.stdout);
  assert!(
    said
      .lines()
      .any(|l| l == format!("removed: {}", record.display())),
    "prune names the record it removed:\n{said}"
  );
  assert!(!record.exists(), "the gone session's record is removed");
  assert_eq!(
    std::fs::read(&file).expect("Claude Code's session file is still there"),
    before,
    "prune never touches Claude Code's own file"
  );
}

/// AT-01.6: **`estate status` on MCP answers the reader's value unchanged,
/// as `estate status --json` prints it.**
#[test]
fn the_mcp_tool_answers_the_same_boundary_as_the_cli() {
  let host = Host::new();
  host.session(
    std::process::id(),
    "sess-mcp",
    "proj-mcp",
    &host.project,
    "idle",
  );
  let call = r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"intent_estate_status","arguments":{}}}"#;
  let (out, frames) = mcp_session(&host.project, Some(&host.home), &[INITIALIZE, call]);
  assert!(
    out.status.success(),
    "{}",
    String::from_utf8_lossy(&out.stderr)
  );
  let answer = frames
    .iter()
    .find(|f| f["id"] == 2)
    .unwrap_or_else(|| panic!("no answer to the estate call in {frames:?}"));
  let text = answer["result"]["content"][0]["text"]
    .as_str()
    .unwrap_or_else(|| panic!("the tool answers text content: {answer:#}"));
  let v: serde_json::Value =
    serde_json::from_str(text).expect("the tool's text is the boundary JSON");
  assert_eq!(v["schema"], "intent/estate-status@1");
}

/// ST0086 WP-03: in the explorer `/estate` opens the estate view, as a push,
/// already being there pushes nothing, and an argument is refused rather than
/// guessed at.
#[test]
fn slash_estate_opens_the_estate_view_in_the_explorer() {
  use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
  use intent_cli::tui::app::{App, Step};
  use intent_cli::tui::nav::View;
  let explorer = || {
    let mut app = App::explore();
    app.commands = intent_cli::tui::commands::vocabulary(&intent_cli::spine::build(
      &intent_cli::dispatch::table(),
    ));
    app
  };
  let enter = |app: &mut App, line: &str| {
    for c in line.chars() {
      app.on_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE), &[]);
    }
    app.on_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE), &[])
  };

  let mut app = explorer();
  let step = enter(&mut app, "/estate");
  assert!(
    matches!(step, Step::Continue),
    "`/estate` is a push, not a lend: {step:?}"
  );
  assert_eq!(
    app.stack.current(),
    &View::Estate,
    "`/estate` did not open the estate"
  );
  let depth = app.stack.depth();
  enter(&mut app, "/estate");
  assert_eq!(
    app.stack.depth(),
    depth,
    "`/estate` on the estate pushed it again"
  );

  let mut app = explorer();
  enter(&mut app, "/estate rhadamanth");
  assert_ne!(
    app.stack.current(),
    &View::Estate,
    "an argument opened the view"
  );
  assert!(
    app.notice.contains("takes no argument"),
    "the refusal does not say why: {}",
    app.notice
  );
}

impl Host {
  /// `intent estate statusline` with `input` piped to it, as a statusline
  /// command pipes the JSON Claude Code hands it.
  fn statusline(&self, input: &str) -> std::process::Output {
    use std::io::Write as _;
    let mut child = self
      .cmd()
      .args(["estate", "statusline"])
      .stdin(std::process::Stdio::piped())
      .stdout(std::process::Stdio::piped())
      .stderr(std::process::Stdio::piped())
      .spawn()
      .expect("run estate statusline");
    child
      .stdin
      .take()
      .expect("the piped stdin")
      .write_all(input.as_bytes())
      .expect("write the statusline input");
    child
      .wait_with_output()
      .expect("wait for estate statusline")
  }
}

const STATUSLINE_SESSION: &str = "00000000-0000-4000-8000-000000000000";

/// AT-02.3: **`estate watch` writes the class's file and `estate unwatch`
/// removes it, each printing its `wrote:` or `removed:` line.**
#[test]
fn watch_declares_a_class_the_status_lists_and_unwatch_removes_it() {
  let host = Host::new();
  let out = host
    .cmd()
    .args(["estate", "watch", "cargo", "cargo|rustc"])
    .output()
    .expect("run estate watch");
  assert!(
    out.status.success(),
    "{}",
    String::from_utf8_lossy(&out.stderr)
  );
  let file = String::from_utf8_lossy(&out.stdout)
    .trim()
    .strip_prefix("wrote: ")
    .map(PathBuf::from)
    .expect("watch names the file it wrote");
  assert!(
    file.ends_with("census/watches/cargo.json"),
    "{}",
    file.display()
  );
  assert!(file.exists(), "the watch file is written");

  let removed = host
    .cmd()
    .args(["estate", "unwatch", "cargo"])
    .output()
    .expect("run estate unwatch");
  assert!(removed.status.success());
  assert_eq!(
    String::from_utf8_lossy(&removed.stdout).trim(),
    "removed: census class `cargo`"
  );
  assert!(!file.exists(), "unwatch removes the watch file");

  let again = host
    .cmd()
    .args(["estate", "unwatch", "cargo"])
    .output()
    .expect("run estate unwatch again");
  assert!(again.status.success(), "a repeat is a no-op, not a failure");
  assert!(
    String::from_utf8_lossy(&again.stdout).contains("was not declared; nothing changed"),
    "{}",
    String::from_utf8_lossy(&again.stdout)
  );
}

/// AT-01.7 and AT-02.3: **a bad class or a bad pattern is refused with a
/// remedy.**
#[test]
fn watch_refuses_a_bad_class_and_a_bad_pattern_with_a_remedy() {
  let host = Host::new();
  for argv in [
    ["estate", "watch", "../cargo", "cargo"],
    ["estate", "watch", "cargo", "("],
  ] {
    let out = host
      .cmd()
      .args(argv)
      .stdin(testkit::lifeline_for(&argv))
      .output()
      .expect("run estate watch");
    let said = String::from_utf8_lossy(&out.stderr);
    assert_eq!(out.status.code(), Some(1), "{argv:?}: {said}");
    assert!(said.contains("remedy:"), "{argv:?} names a remedy: {said}");
  }
}

/// AT-02.4: **`estate statusline` writes the session record's rate limits,
/// prints `wrote:` on stderr and nothing on stdout.**
#[test]
fn statusline_keeps_the_usage_and_leaves_stdout_to_the_status_line() {
  let host = Host::new();
  let input = format!(
    r#"{{"session_id":"{STATUSLINE_SESSION}","model":{{"id":"m"}},"rate_limits":{{"five_hour":{{"used_percentage":42.5,"resets_at":1792857600}}}}}}"#
  );
  let out = host.statusline(&input);
  assert!(
    out.status.success(),
    "{}",
    String::from_utf8_lossy(&out.stderr)
  );
  assert!(
    out.stdout.is_empty(),
    "stdout is the status line, so nothing goes there"
  );
  let record = String::from_utf8_lossy(&out.stderr)
    .trim()
    .strip_prefix("wrote: ")
    .map(PathBuf::from)
    .expect("the wrote: line goes to stderr");
  assert!(record.exists(), "the usage record is written");
}

/// AT-02.4: **an input with no rate_limits exits 0 and writes nothing.**
#[test]
fn statusline_with_no_rate_limits_writes_and_prints_nothing() {
  let host = Host::new();
  let out = host.statusline(&format!(
    r#"{{"session_id":"{STATUSLINE_SESSION}","model":{{"id":"m"}}}}"#
  ));
  assert!(out.status.success());
  assert!(
    out.stdout.is_empty() && out.stderr.is_empty(),
    "nothing to keep, nothing said"
  );
}

/// AT-01.7 and AT-02.4: **an input that is not JSON is refused with a
/// remedy.**
#[test]
fn statusline_refuses_an_input_that_is_not_json_with_a_remedy() {
  let host = Host::new();
  let out = host.statusline("not json");
  let said = String::from_utf8_lossy(&out.stderr);
  assert_eq!(out.status.code(), Some(1), "{said}");
  assert!(said.contains("remedy:"), "{said}");
  assert!(
    out.stdout.is_empty(),
    "a refusal never reaches the status line"
  );
}
