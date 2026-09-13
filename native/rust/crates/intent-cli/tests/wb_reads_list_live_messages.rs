//! Issue 0322: `wb show` and `wb pickup` list a node's LIVE messages and count
//! the handled ones in one line, and `--all` lists every message. The JSON face
//! follows the same default and carries `handled_count`.
//!
//! The fixture holds one handled and one live message on the same board, so a
//! read that printed every message and a read that printed none would each go
//! red on a different assertion.

use std::path::Path;
use std::process::Command;

fn run(cwd: &Path, args: &[&str]) -> (String, i32) {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(cwd)
    .env("HOME", testkit::fixture_home())
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run intent");
  (
    format!(
      "{}{}",
      String::from_utf8_lossy(&out.stdout),
      String::from_utf8_lossy(&out.stderr)
    ),
    out.status.code().unwrap_or(-1),
  )
}

const HANDLED: &str = "the message cc has already answered";
const LIVE: &str = "the message cc still owes an answer";
const HANDLED_LINE: &str = "handled: 1 message(s) -- --all lists them";

/// cc's board with one message from vc cleared and one still live.
fn seeded() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let steps: [&[&str]; 6] = [
    &["init", "wbreads"],
    &[
      "wb", "register", "cc", "--name", "Control", "--role", "control",
    ],
    &[
      "wb",
      "register",
      "vc",
      "--name",
      "Validation",
      "--role",
      "validation",
    ],
    &["wb", "ask", "cc", HANDLED, "--node", "vc"],
    &["wb", "clear", "vc", "--node", "cc"],
    &["wb", "ask", "cc", LIVE, "--node", "vc"],
  ];
  for args in steps {
    let (text, code) = run(dir.path(), args);
    assert_eq!(code, 0, "{args:?}: {text}");
  }
  dir
}

fn assert_default_and_all(dir: &Path, verb: &[&str]) {
  let (text, code) = run(dir, verb);
  assert_eq!(code, 0, "{text}");
  assert!(text.contains(LIVE), "the live message is listed:\n{text}");
  assert!(
    !text.contains(HANDLED),
    "the handled message is left out by default:\n{text}"
  );
  assert!(
    text.contains(HANDLED_LINE),
    "the handled messages are counted in one line:\n{text}"
  );

  let all: Vec<&str> = verb.iter().copied().chain(["--all"]).collect();
  let (text, code) = run(dir, &all);
  assert_eq!(code, 0, "{text}");
  assert!(
    text.contains(LIVE),
    "`--all` lists the live message:\n{text}"
  );
  assert!(
    text.contains(HANDLED),
    "`--all` lists the handled message:\n{text}"
  );
}

#[test]
fn wb_show_lists_live_messages_and_all_lists_the_handled_ones() {
  let dir = seeded();
  assert_default_and_all(dir.path(), &["wb", "show", "cc"]);
}

#[test]
fn wb_pickup_lists_live_messages_and_all_lists_the_handled_ones() {
  let dir = seeded();
  assert_default_and_all(dir.path(), &["wb", "pickup", "--node", "cc"]);
}

#[test]
fn the_json_face_carries_live_messages_and_the_handled_count() {
  let dir = seeded();
  let read = |args: &[&str]| -> serde_json::Value {
    let (text, code) = run(dir.path(), args);
    assert_eq!(code, 0, "{text}");
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("{e}: {text}"))
  };

  let board = read(&["wb", "show", "cc", "--json"]);
  let messages = board["messages"].as_array().expect("messages");
  assert_eq!(messages.len(), 1, "{board}");
  assert_eq!(messages[0]["body"], LIVE, "{board}");
  assert_eq!(board["handled_count"], 1, "{board}");

  let board = read(&["wb", "show", "cc", "--json", "--all"]);
  assert_eq!(
    board["messages"].as_array().expect("messages").len(),
    2,
    "{board}"
  );
  assert_eq!(board["handled_count"], 1, "{board}");
}
