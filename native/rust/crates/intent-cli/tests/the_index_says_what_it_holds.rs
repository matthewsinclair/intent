//! **AT-19.6 / AC-19.6: `intent index status` and `intent index rebuild` exist,
//! report what the index holds, and are registered and exposed like every
//! verb.**
//!
//! **THE SKIPPED PATHS ARE THE PROPERTY UNDER TEST, NOT THE COUNTS.** AC-18.2
//! says nothing is skipped silently, and `3 skipped` is silence with a number on
//! it: the operator looking for one file still cannot tell whether it is among
//! them. So the arm below plants a file that WILL be skipped and asserts its
//! path is on the surface.

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

fn estate() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let (_, err, code) = run(&["init", "index-status-fixture"], dir.path());
  assert_eq!(code, 0, "fixture init failed: {err}");
  dir
}

/// AC-19.6's first half: **both verbs answer, and an index that holds nothing
/// says so rather than printing an empty screen.**
#[test]
fn a_status_before_a_rebuild_says_the_index_holds_nothing() {
  let dir = estate();
  let root = dir.path();

  let (out, err, code) = run(&["index", "status"], root);
  assert_eq!(code, 0, "status failed: {err}");
  assert!(
    err.contains("holds nothing") && err.contains("index rebuild"),
    "an unbuilt index names the verb that builds it, on stderr: {err:?}"
  );
  assert!(
    !out.contains("prose") && !out.contains("code"),
    "nothing is held yet, so no corpus has a row: {out:?}"
  );

  let (rebuilt, err, code) = run(&["index", "rebuild"], root);
  assert_eq!(code, 0, "rebuild failed: {err}");
  assert!(
    rebuilt.contains("prose"),
    "a fresh Intent project carries prose, so the rebuild holds some: {rebuilt:?}"
  );

  // **THE TWO VERBS AGREE, WHICH IS THE WHOLE REASON `status` DOES NOT WALK.**
  // A status that surveyed the tree could differ from the rebuild that wrote
  // the rows, and an operator comparing them is how a stale index is noticed.
  let (after, err, code) = run(&["index", "status"], root);
  assert_eq!(code, 0, "status after rebuild failed: {err}");
  assert_eq!(
    after, rebuilt,
    "status reads the rows the rebuild wrote, so the two renderings agree"
  );
}

/// AC-19.6 with AC-18.2: **a skipped path is named, never counted away.**
#[test]
fn a_skipped_file_is_named_on_the_surface() {
  let dir = estate();
  let root = dir.path();
  std::fs::write(root.join("logo.png"), [0u8, 1, 2, 0, 255]).expect("plant a binary file");

  let (out, err, code) = run(&["index", "rebuild"], root);
  assert_eq!(code, 0, "rebuild failed: {err}");
  assert!(
    out.contains("logo.png"),
    "the skipped PATH is the answer, not its count: {out:?}"
  );

  // Every reason is named even with nothing under it, so an operator can tell
  // "no symlinks were skipped" from "this build does not look for symlinks".
  for reason in intentsvcs::index::status::REASONS {
    assert!(
      out.contains(reason.as_str()),
      "`{}` is not named in the report: {out:?}",
      reason.as_str()
    );
  }
}

/// Issue 0430: **a skip by policy is listed and leaves the answer whole; a
/// skip that hid text makes it partial.** A binary file holds nothing a text
/// query could match, so an answer that reported it as incomplete sent every
/// whole-tree query to grep. An unreadable file is the control: its text is
/// unread, and the answer must still say so.
#[test]
fn a_binary_skip_is_listed_and_an_unreadable_one_makes_the_answer_partial() {
  use std::os::unix::fs::PermissionsExt;
  let dir = estate();
  let root = dir.path();
  std::fs::write(root.join("logo.png"), [0u8, 1, 2, 0, 255]).expect("plant a binary file");
  let (_, err, code) = run(&["index", "rebuild"], root);
  assert_eq!(code, 0, "rebuild failed: {err}");

  let freshness = |root: &Path| -> (serde_json::Value, String) {
    let (out, err, code) = run(&["search", "quokka", "--json"], root);
    assert_eq!(code, 0, "search failed: {err}");
    let answer: serde_json::Value = serde_json::from_str(&out).expect("the envelope is JSON");
    let (_, terminal_err, code) = run(&["search", "quokka"], root);
    assert_eq!(code, 0, "terminal search failed: {terminal_err}");
    (answer["index"].clone(), terminal_err)
  };
  let reason_of = |index: &serde_json::Value, path: &str| -> Option<String> {
    index["skipped"]
      .as_array()
      .expect("skipped is a list")
      .iter()
      .find(|skip| skip["path"] == path)
      .and_then(|skip| skip["reason"].as_str().map(str::to_string))
  };

  let (index, terminal_err) = freshness(root);
  assert_eq!(
    reason_of(&index, "logo.png").as_deref(),
    Some("binary"),
    "the binary is still listed: {index}"
  );
  assert_eq!(
    index["complete"], true,
    "a binary skip leaves the answer whole: {index}"
  );
  assert!(
    !terminal_err.contains("logo.png"),
    "the terminal does not call a whole answer partial: {terminal_err:?}"
  );

  let secret = root.join("secret.txt");
  std::fs::write(&secret, "a quokka nobody can read\n").expect("plant a text file");
  std::fs::set_permissions(&secret, std::fs::Permissions::from_mode(0o000)).expect("chmod 000");
  let (_, err, code) = run(&["index", "rebuild"], root);
  assert_eq!(code, 0, "rebuild failed: {err}");

  let (index, terminal_err) = freshness(root);
  assert_eq!(
    reason_of(&index, "secret.txt").as_deref(),
    Some("unreadable"),
    "the control needs the file skipped as unreadable: {index}"
  );
  assert_eq!(
    index["complete"], false,
    "unread text makes the answer partial: {index}"
  );
  assert!(
    terminal_err.contains("secret.txt") && !terminal_err.contains("logo.png"),
    "the terminal names the gap and not the policy skip: {terminal_err:?}"
  );
}

/// AC-19.6's second half: **registered and exposed like every verb** -- and the
/// register's own decision about the WRITE half is observable, not just written
/// down: `index status` is a tool, `index rebuild` is withheld.
#[test]
fn the_read_half_is_a_tool_and_the_write_half_is_withheld() {
  let dir = estate();
  let (out, frames) = crate::common::mcp_session(
    dir.path(),
    None,
    &[
      r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"drive","version":"0"}}}"#,
      r#"{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}"#,
      r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"intent_index_status","arguments":{}}}"#,
    ],
  );
  assert!(
    out.status.success(),
    "the MCP session failed: {}",
    String::from_utf8_lossy(&out.stderr)
  );
  let names: Vec<String> = frames
    .iter()
    .find(|f| f["id"] == 2)
    .expect("a response to tools/list")["result"]["tools"]
    .as_array()
    .expect("a tool list")
    .iter()
    .filter_map(|t| t["name"].as_str().map(str::to_string))
    .collect();
  assert!(
    names.contains(&"intent_index_status".to_string()),
    "the read is exposed: {names:?}"
  );
  assert!(
    !names.contains(&"intent_index_rebuild".to_string()),
    "the write is withheld, and the register carries the reason: {names:?}"
  );

  let answered = frames
    .iter()
    .find(|f| f["id"] == 3)
    .expect("a response to the call");
  let text = answered["result"]["content"][0]["text"]
    .as_str()
    .expect("the tool answers text-wrapped JSON");
  let summary: serde_json::Value = serde_json::from_str(text).expect("the summary is JSON");
  assert!(
    summary["skipped"].is_object() && summary["empty"].is_boolean(),
    "the tool answers the same summary the terminal renders, skipped paths included: {summary}"
  );
}
