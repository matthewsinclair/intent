//! **AT-19.2 / AC-19.2: `--json` and the MCP search tool are two renderings of
//! ONE envelope, from one facade call.**
//!
//! The two faces are where a result shape goes to drift: they are written on
//! different days, read by different callers, and only one of them is ever read
//! by eye. `--sql` established the discipline in WP-17 (AT-17.2) and this is
//! the same assertion for the text door, which is the one an agent reaches for
//! first.
//!
//! **IT COMPARES THE WHOLE VALUE, NOT A FIELD.** Picking fields would pass for
//! any pair of shapes that happen to agree on the fields the test remembered to
//! name, and the thing being defended is that no second assembly exists at all.

use std::path::Path;
use std::process::Command;

fn run(args: &[&str], cwd: &Path) -> (String, String, i32) {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(cwd)
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run the v3 binary");
  (
    String::from_utf8_lossy(&out.stdout).into_owned(),
    String::from_utf8_lossy(&out.stderr).into_owned(),
    out.status.code().unwrap_or(-1),
  )
}

/// A thread carrying two attachments that both name one word, so the answer has
/// more than one hit and a cap has something to cap.
fn estate() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  let (_, err, code) = run(&["init", "search-envelope-fixture"], root);
  assert_eq!(code, 0, "fixture init failed: {err}");
  let (_, err, code) = run(&["st", "new", "The indexed thread"], root);
  assert_eq!(code, 0, "fixture st new failed: {err}");
  for (name, body) in [
    ("one.md", "# One\n\nA quokka lives in this section.\n"),
    ("two.md", "# Two\n\nA quokka, and a second quokka.\n"),
  ] {
    let source = root.join(name);
    std::fs::write(&source, body).expect("write the attachment source");
    let (_, err, code) = run(
      &[
        "st",
        "attach",
        "ST0001",
        name,
        "--from",
        source.to_str().expect("utf8 path"),
      ],
      root,
    );
    assert_eq!(code, 0, "fixture attach failed: {err}");
  }
  dir
}

#[test]
fn the_mcp_tool_and_json_answer_the_same_envelope() {
  let dir = estate();
  let root = dir.path();

  let (out, err, code) = run(&["search", "quokka", "--json"], root);
  assert_eq!(code, 0, "the CLI search failed: {err}");
  let from_cli: serde_json::Value = serde_json::from_str(&out).expect("the envelope is JSON");
  assert!(
    !from_cli["groups"][0]["hits"]
      .as_array()
      .expect("the lexical group is an array")
      .is_empty(),
    "the fixture must produce hits, or this test compares two empty answers: {from_cli}"
  );

  let (out, frames) = crate::common::mcp_session(
    root,
    None,
    &[
      r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"drive","version":"0"}}}"#,
      r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"intent_search","arguments":{"query":"quokka"}}}"#,
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
    .expect("a response to the call");
  let text = answer["result"]["content"][0]["text"]
    .as_str()
    .expect("the tool answers text-wrapped JSON");
  let from_mcp: serde_json::Value =
    serde_json::from_str(text).expect("the tool's envelope is JSON");

  assert_eq!(
    from_cli, from_mcp,
    "the two faces answer differently for one query"
  );
}

/// The other half of AC-19.2: **`--json` is terminal-channel and is NOT a tool
/// parameter.** It selects a rendering, and the tool has only one rendering to
/// select, so exposing it would publish a parameter that cannot change the
/// answer.
#[test]
fn the_rendering_flag_is_not_published_as_a_tool_parameter() {
  let dir = estate();
  let (out, frames) = crate::common::mcp_session(
    dir.path(),
    None,
    &[
      r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"drive","version":"0"}}}"#,
      r#"{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}"#,
    ],
  );
  assert!(
    out.status.success(),
    "the MCP session failed: {}",
    String::from_utf8_lossy(&out.stderr)
  );
  let tools = frames
    .iter()
    .find(|frame| frame["id"] == 2)
    .expect("a response to tools/list")["result"]["tools"]
    .as_array()
    .expect("a tool list")
    .clone();
  let search = tools
    .iter()
    .find(|tool| tool["name"] == "intent_search")
    .expect("the search tool is published");
  let properties = search["inputSchema"]["properties"]
    .as_object()
    .expect("the tool declares parameters");
  assert!(
    properties.contains_key("query"),
    "the text door is the tool's first parameter: {properties:?}"
  );
  for filter in ["kind", "path", "limit"] {
    assert!(
      properties.contains_key(filter),
      "`{filter}` is part of the question and belongs on the tool: {properties:?}"
    );
  }
  assert!(
    !properties.contains_key("json"),
    "`--json` selects a terminal rendering and must not reach the tool: {properties:?}"
  );
}
