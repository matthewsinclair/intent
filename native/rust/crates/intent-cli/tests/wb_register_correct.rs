//! Issue 0417, under vc decision 21 (1): `wb register <moniker> --name --role
//! --correct` changes a REGISTERED node's name and role and nothing else. It
//! never creates a node, it answers `unchanged` at rc 0 when nothing differs,
//! the board it names keeps its items, and the refusal of a plain re-register
//! with other values names it with both values. Until it, a node registered
//! wrong could be repaired only by a hand `DELETE` on `wb_node`.

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

fn ok(cwd: &Path, args: &[&str]) -> String {
  let (text, code) = run(cwd, args);
  assert_eq!(code, 0, "{args:?}: {text}");
  text
}

/// A project with devbin's wrong `dc` row and one item on its board.
fn seeded() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  ok(dir.path(), &["init", "wbcorrect"]);
  ok(
    dir.path(),
    &[
      "wb",
      "register",
      "dc",
      "--name",
      "devbin-dc",
      "--role",
      "dc",
    ],
  );
  ok(
    dir.path(),
    &[
      "wb",
      "add",
      "todo",
      "the work the board carries",
      "--node",
      "dc",
    ],
  );
  dir
}

#[test]
fn correct_changes_name_and_role_and_keeps_the_board() {
  let dir = seeded();
  let (refused, code) = run(
    dir.path(),
    &[
      "wb",
      "register",
      "dc",
      "--name",
      "DevX Claude",
      "--role",
      "devx",
    ],
  );
  assert_ne!(
    code, 0,
    "a plain re-register with other values is refused: {refused}"
  );
  for value in ["--correct", "devbin-dc", "DevX Claude"] {
    assert!(
      refused.contains(value),
      "the refusal does not name `{value}`: {refused}"
    );
  }

  ok(
    dir.path(),
    &[
      "wb",
      "register",
      "dc",
      "--name",
      "DevX Claude",
      "--role",
      "devx",
      "--correct",
    ],
  );
  let board = ok(dir.path(), &["wb", "show", "dc"]);
  assert!(board.contains("DevX Claude"), "the name moved:\n{board}");
  assert!(board.contains("(devx)"), "the role moved:\n{board}");
  assert!(
    board.contains("the work the board carries"),
    "the board kept its items:\n{board}"
  );

  let again = ok(
    dir.path(),
    &[
      "wb",
      "register",
      "dc",
      "--name",
      "DevX Claude",
      "--role",
      "devx",
      "--correct",
    ],
  );
  assert!(
    again.contains("unchanged"),
    "a correction to the values held reports the node unchanged:\n{again}"
  );
}

#[test]
fn correct_never_creates_a_node() {
  let dir = seeded();
  let (text, code) = run(
    dir.path(),
    &[
      "wb",
      "register",
      "zz",
      "--name",
      "Zed",
      "--role",
      "worker",
      "--correct",
    ],
  );
  assert_ne!(
    code, 0,
    "a correction of an unregistered moniker is refused: {text}"
  );
  assert!(
    text.contains("intent wb register zz"),
    "the refusal names plain register: {text}"
  );
  let status = ok(dir.path(), &["wb", "status"]);
  assert!(
    !status.contains("zz"),
    "the correction created a node:\n{status}"
  );
}
