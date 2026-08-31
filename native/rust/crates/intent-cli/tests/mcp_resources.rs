//! **`AT-09.5`: MCP resources serve the read surfaces, and each reads exactly
//! as its CLI `show`.**
//!
//! The row scopes to the surfaces that HAVE a CLI read to agree with (vc's
//! reworded `AC-09.5`, `f27829df`): ST / WP / issue entities, through the
//! `st_show` / `wp_show` / `issue_show` facade doors. wip.md and the whiteboard
//! boards are deliberately absent — no facade door, no CLI read to match — and
//! that absence is asserted, not merely omitted.
//!
//! # Contents match BY CONSTRUCTION, not by comparison
//!
//! The resource read and the CLI `show` render through the ONE
//! `crate::show::*` renderer (`451b4d7f`), so this test does not check that two
//! renderers coincide on today's data — the trap `AC-09.4` names. It drives
//! both DOORS end to end — the real `intent mcp` server for the resource, the
//! real `intent … show` for the CLI — and requires the bytes to be equal.
//! cc's caution kept: the one-renderer seam and this end-to-end drive catch
//! different halves, so both exist.
//!
//! # `resources/list` == `resources/read`
//!
//! Every URI the server lists must read without refusal — a listed resource the
//! read cannot serve is the tool-roster defect on the resource surface, one
//! surface over.

mod common;

use std::path::{Path, PathBuf};
use std::process::Command;

use common::{mcp_session, short_dir};
use serde_json::{Value, json};

struct Fixture(PathBuf);

impl Drop for Fixture {
  fn drop(&mut self) {
    let _ = std::fs::remove_dir_all(&self.0);
  }
}

/// A project carrying one of each resource kind: a thread, a work package under
/// it, and an issue.
fn project() -> (Fixture, String, u32, u32) {
  let root = short_dir("mcp-resources");
  intentsvcs::init::init(&root, "Resources", "test", env!("CARGO_PKG_VERSION"))
    .expect("the shipped initialiser creates a project");
  let opened = intentsvcs::project::Project::open(&root).expect("the project just created");
  let ctx = intentsvcs::facade::FacadeContext {
    principal: "test".to_string(),
    project_id: opened.config().project_id.clone().unwrap_or_default(),
    version: env!("CARGO_PKG_VERSION").to_string(),
  };
  let mut facade = intentsvcs::facade::Facade::open(opened, ctx).expect("open the new project");
  let st = facade
    .st_new("A thread for resources")
    .expect("mint a thread");
  let seq = facade
    .wp_new(&st, "A work package", intentsvcs::model::TShirt::M)
    .expect("mint a work package");
  let number = facade
    .issue_add(
      "An issue for resources",
      Some("medium"),
      Some("ic"),
      "issue body prose",
    )
    .expect("mint an issue");
  (Fixture(root), st, seq, number)
}

/// The exact stdout of an `intent …` invocation in the project.
fn cli(root: &Path, argv: &[&str]) -> String {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(argv)
    .current_dir(root)
    .output()
    .expect("the intent binary runs");
  assert!(
    out.status.success(),
    "`intent {}` failed: {}",
    argv.join(" "),
    String::from_utf8_lossy(&out.stderr)
  );
  String::from_utf8(out.stdout).expect("utf8 stdout")
}

fn init_frame() -> &'static str {
  r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"resources","version":"0"}}}"#
}

/// Drive one session and return the parsed frames, asserting a clean exit.
fn session(root: &Path, frames: &[&str]) -> Vec<Value> {
  let (out, parsed) = mcp_session(root, None, frames);
  assert!(
    out.status.success(),
    "the server did not exit cleanly: {}",
    String::from_utf8_lossy(&out.stderr)
  );
  parsed
}

fn response<'a>(frames: &'a [Value], id: i64) -> &'a Value {
  frames
    .iter()
    .find(|f| f["id"] == json!(id))
    .unwrap_or_else(|| panic!("no response with id {id}: {frames:?}"))
}

#[test]
fn initialize_declares_the_resources_capability() {
  let (fx, _st, _seq, _n) = project();
  let frames = session(&fx.0, &[init_frame()]);
  let caps = &response(&frames, 1)["result"]["capabilities"];
  assert!(
    caps.get("resources").is_some(),
    "the server does not declare the resources capability: {caps}"
  );
}

#[test]
fn resources_list_names_each_entity_and_nothing_without_a_cli_read() {
  let (fx, st, seq, number) = project();
  let frames = session(
    &fx.0,
    &[
      init_frame(),
      r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
      r#"{"jsonrpc":"2.0","id":2,"method":"resources/list"}"#,
    ],
  );
  let listed = response(&frames, 2)["result"]["resources"]
    .as_array()
    .expect("resources/list carries a resources array")
    .clone();
  let uris: Vec<String> = listed
    .iter()
    .map(|r| r["uri"].as_str().expect("a resource uri").to_string())
    .collect();

  // The `intent://` address grammar is plural and owned by `address.rs`
  // (`address_of` / `address::parse`), not spelled here or in `mcp.rs`.
  for expected in [
    format!("intent:///threads/{st}"),
    format!("intent:///threads/{st}/wp/{seq}"),
    format!("intent:///issues/{number:04}"),
  ] {
    assert!(
      uris.contains(&expected),
      "resources/list is missing {expected}: {uris:?}"
    );
  }

  // wip.md and the boards are NOT resources: no `intent:///nodes/...` (the
  // whiteboard board address) and nothing spelling `wip`. Their absence is the
  // reworded criterion, asserted rather than assumed.
  for absent in ["intent:///nodes", "intent:///events", "intent:///wip"] {
    assert!(
      !uris.iter().any(|u| u.starts_with(absent)),
      "a `{absent}` resource is served, but it has no CLI read to match: {uris:?}"
    );
  }
}

#[test]
fn every_listed_resource_reads_as_exactly_its_cli_show() {
  let (fx, st, seq, number) = project();
  let root = &fx.0;

  // The three kinds against their CLI reads, byte for byte. The URI is the
  // plural `intent://` address; the CLI argv is the verb's own spelling.
  let cases: [(String, Vec<String>); 3] = [
    (
      format!("intent:///threads/{st}"),
      vec!["st".into(), "show".into(), st.clone()],
    ),
    (
      format!("intent:///threads/{st}/wp/{seq}"),
      vec!["wp".into(), "show".into(), format!("{st}/{seq}")],
    ),
    (
      format!("intent:///issues/{number:04}"),
      vec!["issues".into(), "show".into(), format!("{number:04}")],
    ),
  ];

  for (uri, argv) in &cases {
    let read =
      format!(r#"{{"jsonrpc":"2.0","id":2,"method":"resources/read","params":{{"uri":"{uri}"}}}}"#);
    let frames = session(root, &[init_frame(), &read]);
    let contents = response(&frames, 2)["result"]["contents"]
      .as_array()
      .unwrap_or_else(|| panic!("resources/read {uri} carries no contents: {frames:?}"));
    let text = contents[0]["text"].as_str().expect("the content is text");
    let argv_ref: Vec<&str> = argv.iter().map(String::as_str).collect();
    let shown = cli(root, &argv_ref);
    assert_eq!(
      text,
      shown,
      "resource {uri} does not read byte-identical to `intent {}` -- the two faces have diverged",
      argv.join(" ")
    );
  }
}

#[test]
fn resources_list_and_read_agree_every_listed_uri_reads() {
  let (fx, _st, _seq, _n) = project();
  let root = &fx.0;
  let list = session(
    root,
    &[
      init_frame(),
      r#"{"jsonrpc":"2.0","id":2,"method":"resources/list"}"#,
    ],
  );
  let uris: Vec<String> = response(&list, 2)["result"]["resources"]
    .as_array()
    .expect("resources array")
    .iter()
    .map(|r| r["uri"].as_str().expect("uri").to_string())
    .collect();
  assert!(
    !uris.is_empty(),
    "the project has resources but the list is empty"
  );

  for uri in &uris {
    let read =
      format!(r#"{{"jsonrpc":"2.0","id":3,"method":"resources/read","params":{{"uri":"{uri}"}}}}"#);
    let frames = session(root, &[init_frame(), &read]);
    let answer = response(&frames, 3);
    assert!(
      answer.get("error").is_none() && answer["result"]["contents"].is_array(),
      "a listed resource `{uri}` could not be read: {answer}"
    );
  }
}

#[test]
fn a_uri_that_names_nothing_is_refused_not_answered_empty() {
  let (fx, _st, _seq, _n) = project();
  let read = r#"{"jsonrpc":"2.0","id":2,"method":"resources/read","params":{"uri":"intent:///threads/ST9999"}}"#;
  let frames = session(&fx.0, &[init_frame(), read]);
  let answer = response(&frames, 2);
  assert!(
    answer.get("error").is_some(),
    "reading a nonexistent thread returned a result rather than an error: {answer}"
  );
}
