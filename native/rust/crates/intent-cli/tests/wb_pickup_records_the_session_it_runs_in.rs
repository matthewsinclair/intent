//! Issue 0433: `wb pickup` with no `--session` records the Claude Code session
//! it runs in, read from `CLAUDE_CODE_SESSION_ID`. A node booted by
//! `/in-whiteboard` passes no flag, and its header kept whatever id an earlier
//! session wrote, so the read that finds a node's transcript from its header
//! reached a dead one with nothing to say so.

use std::path::Path;
use std::process::Command;

const SESSION: &str = "0433-the-session-this-pickup-runs-in";

/// `intent` with the session variable set to `session`, or removed, so a test
/// run from inside a Claude Code session cannot lend the fixture its own id.
fn run(cwd: &Path, args: &[&str], session: Option<&str>) -> (String, i32) {
  let mut command = Command::new(env!("CARGO_BIN_EXE_intent"));
  command
    .args(args)
    .current_dir(cwd)
    .env("HOME", testkit::fixture_home())
    .env_remove("CLAUDE_CODE_SESSION_ID")
    .stdin(testkit::lifeline_for(args));
  if let Some(id) = session {
    command.env("CLAUDE_CODE_SESSION_ID", id);
  }
  let out = command.output().expect("run intent");
  (
    format!(
      "{}{}",
      String::from_utf8_lossy(&out.stdout),
      String::from_utf8_lossy(&out.stderr)
    ),
    out.status.code().unwrap_or(-1),
  )
}

#[test]
fn a_pickup_with_no_session_flag_records_the_session_it_runs_in() {
  let dir = tempfile::tempdir().expect("tempdir");
  let steps: [&[&str]; 2] = [
    &["init", "wbsession"],
    &[
      "wb", "register", "cc", "--name", "Control", "--role", "control",
    ],
  ];
  for args in steps {
    let (text, code) = run(dir.path(), args, None);
    assert_eq!(code, 0, "{args:?}: {text}");
  }

  let (text, code) = run(dir.path(), &["wb", "pickup", "--node", "cc"], Some(SESSION));
  assert_eq!(code, 0, "pickup: {text}");

  let (status, code) = run(dir.path(), &["wb", "status", "--json"], None);
  assert_eq!(code, 0, "status: {status}");
  let nodes: serde_json::Value = serde_json::from_str(&status).expect("status is JSON");
  let cc = nodes
    .as_array()
    .and_then(|nodes| nodes.iter().find(|n| n["moniker"] == "cc"))
    .unwrap_or_else(|| panic!("cc is on the roster: {status}"));
  assert_eq!(cc["session_id"], SESSION, "cc's header: {cc}");
}
