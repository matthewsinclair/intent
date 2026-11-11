//! ST0092 WP-05 at the process surface: what a caller reads when a declared
//! team closes a pair or a door that is not the mediator's writes as hv, on
//! the CLI and on MCP, and that the test-only context never ships.
//!
//! The library arms in `intentsvcs/tests/wb_hv.rs` hold what each refusal
//! decides; these hold what reaches a terminal or an MCP host.

use std::path::Path;
use std::process::Output;

use intentsvcs::teams::Teams;

const SESSION: &str = "CLAUDE_CODE_SESSION_ID";

/// One run of this build's `intent` in `cwd` from hv's own terminal: no
/// Claude Code session, whatever session runs the suite.
fn run(cwd: &Path, args: &[&str]) -> Output {
  crate::common::intent()
    .args(args)
    .current_dir(cwd)
    .env_remove(SESSION)
    .env("HOME", testkit::fixture_home())
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run intent")
}

fn text(out: &Output) -> String {
  format!(
    "{}{}",
    String::from_utf8_lossy(&out.stdout),
    String::from_utf8_lossy(&out.stderr)
  )
}

/// A project whose board declares `team`, and its human seat, mediator and a
/// seat that is neither, read from the declaration the apply wrote.
struct Declared {
  dir: tempfile::TempDir,
  human: String,
  mediator: String,
  worker: String,
}

fn declared(name: &str) -> Declared {
  let dir = tempfile::tempdir().expect("tempdir");
  for args in [&["init", name][..], &["wb", "init", "team", "--apply"]] {
    let out = run(dir.path(), args);
    assert_eq!(out.status.code(), Some(0), "{args:?}: {}", text(&out));
  }
  let file = std::fs::read_to_string(dir.path().join("intent/whiteboard/declaration.json"))
    .expect("the declaration");
  let d: serde_json::Value = serde_json::from_str(&file).expect("plain JSON");
  let (human, mediator) = (
    d["human"].as_str().expect("human").to_string(),
    d["mediator"].as_str().expect("mediator").to_string(),
  );
  let worker = Teams::new(&testkit::repo_root())
    .resolve(&["team".to_string()], None, false)
    .expect("team resolves")
    .seats
    .into_iter()
    .map(|s| s.moniker)
    .find(|m| m != &human && m != &mediator)
    .expect("a seat that is neither");
  Declared {
    dir,
    human,
    mediator,
    worker,
  }
}

/// ST0092 AT-05.2 at the surface: an ask to hv from a seat that is not the
/// mediator exits 1, and the command its remedy gives runs as written.
#[test]
fn at_05_2_the_refusal_names_the_mediator_and_its_command_runs_as_written() {
  let t = declared("hvaskdoor");
  let body = "is the 'cut' today?";
  let out = run(
    t.dir.path(),
    &["wb", "ask", &t.human, body, "--node", &t.worker],
  );
  assert_eq!(out.status.code(), Some(1), "{}", text(&out));
  let said = String::from_utf8_lossy(&out.stderr).into_owned();
  assert!(said.contains(&format!("`{}`", t.mediator)), "{said}");
  let start = said
    .find("`intent wb ask ")
    .unwrap_or_else(|| panic!("no command in the remedy: {said}"));
  let command = &said[start + 1..];
  let command = &command[..command.find('`').expect("the command's closing backtick")];
  let binary = crate::common::intent().get_program().to_owned();
  let runnable = command.replacen("intent", &format!("'{}'", binary.to_string_lossy()), 1);
  let ran = std::process::Command::new("sh")
    .arg("-c")
    .arg(&runnable)
    .current_dir(t.dir.path())
    .env_remove(SESSION)
    .env("HOME", testkit::fixture_home())
    .output()
    .expect("run the remedy");
  assert_eq!(ran.status.code(), Some(0), "`{command}`: {}", text(&ran));
  let mediator_board = std::fs::read_to_string(t.dir.path().join(format!(
    "intent/whiteboard/{}/inbox.{}.md",
    t.mediator, t.worker
  )))
  .expect("the mediator's inbox from the worker");
  assert!(mediator_board.contains(body), "{mediator_board}");
}

/// ST0092 AT-05.4 on MCP (vc's ruling on the second row read): a node-taking
/// tool as hv from a worker's session is refused, and so is one from a server
/// that carries no session at all, because MCP never reads as hv's terminal.
#[test]
fn at_05_4_mcp_writes_as_hv_only_from_the_mediators_session() {
  let t = declared("hvmcpdoor");
  for (node, session) in [
    (&t.worker, "session-of-a-worker"),
    (&t.mediator, "session-of-the-mediator"),
  ] {
    let out = run(
      t.dir.path(),
      &["wb", "pickup", "--node", node, "--session", session],
    );
    assert_eq!(out.status.code(), Some(0), "bind {node}: {}", text(&out));
  }
  let init = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"drive","version":"0"}}}"#;
  let touch = format!(
    r#"{{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{{"name":"intent_wb_touch","arguments":{{"node":"{}"}}}}}}"#,
    t.human
  );
  let answer = |session: Option<&str>| {
    let (out, frames) = crate::common::mcp_session_with_env(
      t.dir.path(),
      None,
      &[(SESSION, session)],
      &[init, &touch],
    );
    assert!(out.status.success(), "{}", text(&out));
    frames
      .into_iter()
      .find(|f| f["id"] == 2)
      .map(|f| f.to_string())
      .expect("an answer to the call")
  };
  let refusal = "is written on this board only from";
  let worker = answer(Some("session-of-a-worker"));
  assert!(worker.contains(refusal), "{worker}");
  let nobody = answer(None);
  assert!(
    nobody.contains(refusal) && nobody.contains("this call says neither"),
    "an MCP call with no session read as hv's terminal: {nobody}"
  );
  let mediator = answer(Some("session-of-the-mediator"));
  assert!(!mediator.contains(refusal), "{mediator}");
}

/// vc's condition (2) on INV-12: the test-only context, which says a call came
/// from hv's terminal, is never built by shipped source, so no shipped door
/// can hand it out. Each crate's trailing `#[cfg(test)]` module is exempt.
#[test]
fn the_test_context_is_never_built_by_shipped_source() {
  let mut defined = false;
  let mut built = Vec::new();
  for file in crate::common::shipped_sources() {
    let code = std::fs::read_to_string(&file).expect("read shipped source");
    let shipped = code.split("#[cfg(test)]").next().unwrap_or_default();
    for line in shipped.lines() {
      if line.contains("pub fn for_tests(") {
        defined = true;
      } else if line.contains("for_tests(") {
        built.push(format!("{}: {}", file.display(), line.trim()));
      }
    }
  }
  assert!(
    defined,
    "precondition: the scan reads the file that defines it"
  );
  assert!(
    built.is_empty(),
    "shipped source builds the test context:\n{}",
    built.join("\n")
  );
}
