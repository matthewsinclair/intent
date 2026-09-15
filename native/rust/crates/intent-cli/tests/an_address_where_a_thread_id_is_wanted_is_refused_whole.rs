//! A URL handed to a verb that takes a steel thread id is refused WHOLE: the
//! refusal names the argument as typed, says the verb takes a steel thread id,
//! and gives an example. `thread_spec` split its argument at the first `/`, so
//! `intent:///threads/ST0001` reached the id check as `intent:` and the refusal
//! named a fragment nobody had typed as an id (found driving issue 0338, ruled
//! by vc on 2026-09-15). Driven through the doors the ruling names: the CLI's
//! `st hydrate` and `st dehydrate`, and MCP's `st hydrate`, `st dehydrate` and
//! `st edit`, each with this project's own address and another project's.

use std::path::PathBuf;
use std::process::Command;

use crate::common::{mcp_session, short_dir};

/// Both are addresses and neither is an id: this project's own ST0001, which the
/// project carries, and another project's.
const ADDRESSES: [&str; 2] = [
  "intent:///threads/ST0001",
  "intent://elsewhere/threads/ST0001",
];

/// A project carrying ST0001, so a door that resolved either address here would
/// find a thread, which is what makes a refusal mean something.
fn estate() -> PathBuf {
  let root = short_dir("thread-id-door");
  std::fs::create_dir_all(&root).expect("the estate directory");
  intentsvcs::init::init(&root, "Doors", "test", env!("CARGO_PKG_VERSION"))
    .expect("a fresh project initialises");
  let project = intentsvcs::project::Project::open(&root).expect("the project opens");
  let ctx = intentsvcs::facade::FacadeContext {
    principal: "test".to_string(),
    project_id: String::new(),
    version: env!("CARGO_PKG_VERSION").to_string(),
  };
  let mut facade = intentsvcs::facade::Facade::open(project, ctx).expect("the facade opens");
  facade
    .st_new("A thread this project carries")
    .expect("a thread is created");
  root
}

/// The whole argument, what the verb takes, and an example -- and never the
/// scheme's head on its own, which is what the split handed the id check.
fn refuses_it_whole(said: &str, url: &str) -> bool {
  said.contains(&format!("`{url}`"))
    && said.contains("this verb takes a steel thread id")
    && said.contains("`ST0000`")
    && !said.contains("`intent:`")
}

#[test]
fn st_hydrate_and_dehydrate_refuse_an_address_whole() {
  let root = estate();
  let home = short_dir("thread-id-door-home");
  std::fs::create_dir_all(&home).expect("an isolated home");
  for verb in ["hydrate", "dehydrate"] {
    for url in ADDRESSES {
      let args = ["st", verb, url];
      let out = Command::new(env!("CARGO_BIN_EXE_intent"))
        .args(args)
        .current_dir(&root)
        .env("HOME", &home)
        .stdin(testkit::lifeline_for(&args))
        .output()
        .expect("the intent binary runs");
      let said = String::from_utf8_lossy(&out.stderr);
      assert!(
        refuses_it_whole(&said, url),
        "`intent st {verb} {url}` did not refuse the address whole: {said}"
      );
      assert_eq!(
        out.status.code(),
        Some(1),
        "`intent st {verb} {url}` did not exit 1: {said}"
      );
    }
  }
  let _ = std::fs::remove_dir_all(&root);
  let _ = std::fs::remove_dir_all(&home);
}

#[test]
fn the_mcp_thread_tools_refuse_an_address_whole() {
  let root = estate();
  let mut frames = vec![
    r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"thread-id-door","version":"0"}}}"#.to_string(),
  ];
  let mut asked = Vec::new();
  for tool in ["intent_st_hydrate", "intent_st_dehydrate", "intent_st_edit"] {
    for url in ADDRESSES {
      let id = frames.len() + 1;
      frames.push(
        serde_json::json!({
          "jsonrpc": "2.0",
          "id": id,
          "method": "tools/call",
          "params": {"name": tool, "arguments": {"id": url}},
        })
        .to_string(),
      );
      asked.push((id, tool, url));
    }
  }
  let sent: Vec<&str> = frames.iter().map(String::as_str).collect();
  let (out, parsed) = mcp_session(&root, None, &sent);
  assert!(
    out.status.success(),
    "the server did not exit cleanly: {}",
    String::from_utf8_lossy(&out.stderr)
  );
  for (id, tool, url) in asked {
    let answer = parsed
      .iter()
      .find(|f| f["id"] == serde_json::json!(id))
      .unwrap_or_else(|| panic!("no response with id {id}: {parsed:?}"));
    let said = answer["result"]["content"][0]["text"]
      .as_str()
      .unwrap_or_default();
    assert!(
      refuses_it_whole(said, url),
      "{tool} did not refuse {url} whole: {answer}"
    );
    assert_eq!(
      answer["result"]["isError"], true,
      "{tool} answered {url} rather than refusing it: {answer}"
    );
  }
  let _ = std::fs::remove_dir_all(&root);
}
