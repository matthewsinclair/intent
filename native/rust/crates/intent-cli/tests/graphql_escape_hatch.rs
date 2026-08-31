//! The escape hatch end to end (`AT-09.2`, and the bridging clause of
//! `AT-00.4`): both faces ship one document to a real intentd and answer the
//! same thing; with no daemon, both refuse by remedy and execute nothing.
//!
//! **THE DISCRIMINATOR IS THE DAEMON'S DISPATCH COUNTER, NOT THE ANSWER.** An
//! empty listing comes from anywhere, so the fixture project carries a minted
//! thread and the terminal arm reads `dispatched` before and after -- the same
//! proof the `AC-08.2` harness rests on. **The bound is READS ONLY** (vc under
//! hv's pen, 2026-08-31): the mutation arm below is that bound, driven.

mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::{RealDaemon, mcp_session, short_dir};
use serde_json::Value;

const MINTED: &str = "Minted for the hatch";
const DOCUMENT: &str = "{ threads { id title } }";

struct Fixture(PathBuf);

impl Drop for Fixture {
  fn drop(&mut self) {
    let _ = std::fs::remove_dir_all(&self.0);
  }
}

/// An Intent project at a fresh short path, carrying one findable thread.
fn project() -> (Fixture, String) {
  let root = short_dir("hatch-proj");
  intentsvcs::init::init(&root, "Hatch", "test", env!("CARGO_PKG_VERSION"))
    .expect("the shipped initialiser creates a project");
  let opened = intentsvcs::project::Project::open(&root).expect("the project just created");
  let ctx = intentsvcs::facade::FacadeContext {
    principal: "test".to_string(),
    project_id: opened.config().project_id.clone().unwrap_or_default(),
    version: env!("CARGO_PKG_VERSION").to_string(),
  };
  let mut facade = intentsvcs::facade::Facade::open(opened, ctx).expect("open the new project");
  let id = facade.st_new(MINTED).expect("mint one thread");
  (Fixture(root), id)
}

fn run(home: &Path, root: &Path, argv: &[&str]) -> Output {
  Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(argv)
    .current_dir(root)
    .env("HOME", home)
    .output()
    .expect("the intent binary runs")
}

fn text(bytes: &[u8]) -> String {
  String::from_utf8_lossy(bytes).to_string()
}

fn answer(out: &Output) -> Value {
  serde_json::from_str(&text(&out.stdout)).unwrap_or_else(|e| {
    panic!(
      "stdout is not one JSON document: {e}\nstdout: {}\nstderr: {}",
      text(&out.stdout),
      text(&out.stderr)
    )
  })
}

fn initialize() -> &'static str {
  r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"hatch-test","version":"0"}}}"#
}

fn tool_call(id: u32, arguments: &Value) -> String {
  format!(
    r#"{{"jsonrpc":"2.0","id":{id},"method":"tools/call","params":{{"name":"intent_graphql","arguments":{arguments}}}}}"#
  )
}

#[test]
fn the_terminal_face_answers_through_a_real_daemon_and_the_counter_moves() {
  let daemon = RealDaemon::start();
  let (project, id) = project();

  // First contact registers the project; the baseline is read after it so
  // the delta is the document's alone.
  let first = run(daemon.home(), &project.0, &["graphql", DOCUMENT]);
  assert_eq!(
    first.status.code(),
    Some(0),
    "stdout: {}\nstderr: {}",
    text(&first.stdout),
    text(&first.stderr)
  );
  let before = daemon.dispatched(&project.0);

  let out = run(daemon.home(), &project.0, &["graphql", DOCUMENT]);
  assert_eq!(out.status.code(), Some(0), "stderr: {}", text(&out.stderr));
  assert!(
    out.stderr.is_empty(),
    "a clean answer says nothing on stderr: {}",
    text(&out.stderr)
  );
  let got = answer(&out);
  assert!(got["errors"].is_null(), "{got}");
  let threads = got["data"]["threads"]
    .as_array()
    .expect("threads is a list");
  assert!(
    threads
      .iter()
      .any(|t| t["id"] == id.as_str() && t["title"] == MINTED),
    "the minted thread reached stdout through the daemon: {got}"
  );
  assert_eq!(
    daemon.dispatched(&project.0),
    before + 1,
    "one document is one dispatch, which is the proof it left this process"
  );
}

#[test]
fn the_mcp_face_ships_the_same_document_to_the_same_daemon() {
  let daemon = RealDaemon::start();
  let (project, _id) = project();

  let terminal = run(daemon.home(), &project.0, &["graphql", DOCUMENT]);
  assert_eq!(
    terminal.status.code(),
    Some(0),
    "stderr: {}",
    text(&terminal.stderr)
  );
  let from_terminal = answer(&terminal);

  let call = tool_call(2, &serde_json::json!({ "query": DOCUMENT }));
  let (out, frames) = mcp_session(&project.0, Some(daemon.home()), &[initialize(), &call]);
  assert!(out.status.success(), "stderr: {}", text(&out.stderr));
  let result = &frames[1]["result"];
  assert_eq!(result["isError"], false, "{result}");
  let from_mcp: Value =
    serde_json::from_str(result["content"][0]["text"].as_str().expect("text content"))
      .expect("the tool's text content is the JSON answer");

  // The two faces carry ONE answer: parsed rather than byte-compared, because
  // the terminal pretty-prints and the tool result is whatever the server's
  // renderer chose, and the claim is about the value.
  assert_eq!(from_mcp, from_terminal);
}

#[test]
fn with_no_daemon_both_faces_refuse_by_remedy_and_execute_nothing() {
  let home = short_dir("hatch-nodaemon");
  let (project, _id) = project();

  let out = run(&home, &project.0, &["graphql", DOCUMENT]);
  assert_eq!(
    out.status.code(),
    Some(2),
    "nothing could answer, which is rc=2 and never a local answer\nstdout: {}\nstderr: {}",
    text(&out.stdout),
    text(&out.stderr)
  );
  assert!(
    out.stdout.is_empty(),
    "no answer was executed here: {}",
    text(&out.stdout)
  );
  let stderr = text(&out.stderr);
  assert!(stderr.contains("intent daemon start"), "{stderr}");
  // The wrapped `AskError` remedy promises *this command executes in-process*;
  // the hatch's own text says the opposite in the same words, so the
  // assertion is on the PROMISE, not on the phrase.
  assert!(
    !stderr.contains("executes in-process"),
    "the hatch has no in-process fallback to offer: {stderr}"
  );

  let call = tool_call(2, &serde_json::json!({ "query": DOCUMENT }));
  let (out, frames) = mcp_session(&project.0, Some(&home), &[initialize(), &call]);
  assert!(
    out.status.success(),
    "a refused tool call never kills the session: {}",
    text(&out.stderr)
  );
  let result = &frames[1]["result"];
  assert_eq!(result["isError"], true, "{result}");
  let content = result["content"][0]["text"].as_str().expect("text content");
  assert!(content.contains("intent daemon start"), "{content}");

  let _ = std::fs::remove_dir_all(&home);
}

#[test]
fn a_document_the_schema_refuses_prints_the_answer_and_exits_1() {
  let daemon = RealDaemon::start();
  let (project, _id) = project();

  let out = run(
    daemon.home(),
    &project.0,
    &["graphql", "mutation { anything }"],
  );
  assert_eq!(
    out.status.code(),
    Some(1),
    "the schema answered no, which is a verdict\nstdout: {}\nstderr: {}",
    text(&out.stdout),
    text(&out.stderr)
  );
  assert!(
    out.stderr.is_empty(),
    "the verdict is on stdout and stderr stays clean: {}",
    text(&out.stderr)
  );
  let got = answer(&out);
  assert!(got["data"].is_null(), "{got}");
  assert!(
    got["errors"].as_array().is_some_and(|e| !e.is_empty()),
    "the refusal travels inside the answer: {got}"
  );
}

#[test]
fn variables_reach_the_document_and_a_bad_value_is_refused_before_the_wire() {
  let daemon = RealDaemon::start();
  let (project, id) = project();

  let out = run(
    daemon.home(),
    &project.0,
    &[
      "graphql",
      "query($id: String!) { thread(id: $id) { title } }",
      "--variables",
      &format!(r#"{{"id":"{id}"}}"#),
    ],
  );
  assert_eq!(out.status.code(), Some(0), "stderr: {}", text(&out.stderr));
  assert_eq!(answer(&out)["data"]["thread"]["title"], MINTED);

  // Under a HOME with no daemon: a bad value must be refused as a bad value,
  // at rc=1, and never reach the point where the missing daemon is noticed.
  // That ordering is the claim; the daemon-less HOME is what makes it testable.
  let home = short_dir("hatch-nodaemon-vars");
  let out = run(
    &home,
    &project.0,
    &["graphql", DOCUMENT, "--variables", "[1]"],
  );
  assert_eq!(out.status.code(), Some(1), "stderr: {}", text(&out.stderr));
  let stderr = text(&out.stderr);
  assert!(stderr.contains("--variables"), "{stderr}");
  assert!(
    !stderr.contains("daemon start"),
    "refused before discovery: {stderr}"
  );
  let _ = std::fs::remove_dir_all(&home);
}
