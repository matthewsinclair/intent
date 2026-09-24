//! Issue 0417, under vc decision 21 (1): `wb register <moniker> --name --role
//! --correct` changes a REGISTERED node's name and role and nothing else. It
//! never creates a node, it answers `unchanged` at rc 0 when nothing differs,
//! the board it names keeps its items, and the refusal of a plain re-register
//! with other values names it with both values. Until it, a node registered
//! wrong could be repaired only by a hand `DELETE` on `wb_node`.
//!
//! Issue 0522 lives here too, because it is the same verb's refusals: every one
//! of them prints the whole form that works, and `--help` says what the
//! moniker, `--name` and `--role` are.

use std::path::Path;

fn run(cwd: &Path, args: &[&str]) -> (String, i32) {
  let out = crate::common::intent()
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
    text.contains(&intentsvcs::model::register_form("zz")),
    "the refusal names plain register: {text}"
  );
  let status = ok(dir.path(), &["wb", "status"]);
  assert!(
    !status.contains("zz"),
    "the correction created a node:\n{status}"
  );
}

/// Issue 0522: every refusal of the verb's arguments names the whole command
/// that works, with the moniker filled in where one was typed, and names
/// `intent wb status` for the nodes that exist. hv met two refusals in Gtools
/// that named only the flags, and could not get from either one to the form.
#[test]
fn every_register_refusal_names_the_whole_form() {
  let dir = tempfile::tempdir().expect("tempdir");
  ok(dir.path(), &["init", "wbrefuse"]);
  let form = intentsvcs::model::register_form;
  for (args, shown) in [
    (vec!["wb", "register", "dc", "--role", "dc"], form("dc")),
    (
      vec!["wb", "register", "--name", "dc", "--role", "dc"],
      form("<moniker>"),
    ),
    (
      vec!["wb", "register", "dc", "--correct"],
      format!("{} --correct", form("dc")),
    ),
  ] {
    let (refused, code) = run(dir.path(), &args);
    assert_ne!(code, 0, "{args:?} is refused: {refused}");
    for expected in [
      format!("remedy: `{shown}`"),
      "`intent wb status`".to_string(),
    ] {
      assert!(
        refused.contains(&expected),
        "{args:?} does not print {expected}: {refused}"
      );
    }
  }

  let (no_moniker, _) = run(
    dir.path(),
    &["wb", "register", "--name", "dc", "--role", "dc"],
  );
  assert!(
    no_moniker.contains("no moniker was given: the moniker comes first, as a positional"),
    "the refusal does not say the moniker is missing and where it goes: {no_moniker}"
  );
}

/// Issue 0522: `wb register --help` says what the moniker, `--name` and `--role`
/// are, each with an example, where it printed `[MONIKER]` with nothing beside it.
#[test]
fn register_help_says_what_each_part_is() {
  let dir = tempfile::tempdir().expect("tempdir");
  let help = ok(dir.path(), &["wb", "register", "--help"]);
  // clap wraps help to a width, so the examples are read with the wrap undone.
  let flat = help.split_whitespace().collect::<Vec<_>>().join(" ");
  for shown in ["handle, eg `qa`", "eg \"QA Claude\"", "eg `worker`"] {
    assert!(
      flat.contains(shown),
      "`wb register --help` does not show {shown}:\n{help}"
    );
  }
}

/// Issue 0541: 0522 made every remedy that sends someone to register a node
/// print the whole form, and two still named the bare `intent wb register`.
/// That reads a roster from hand-authored headers, and a project with no board
/// has none, so the bare form registered nothing. Both are driven in a project
/// with no `board.json` at all: the empty roster, and a node that is not on it.
#[test]
fn an_empty_roster_and_an_unregistered_node_name_the_whole_form() {
  let dir = tempfile::tempdir().expect("tempdir");
  ok(dir.path(), &["init", "wbempty"]);
  assert!(
    !dir.path().join("intent/whiteboard").exists()
      || std::fs::read_dir(dir.path().join("intent/whiteboard"))
        .expect("read the whiteboard")
        .filter_map(Result::ok)
        .all(|e| !e.path().join("board.json").exists()),
    "precondition: the project holds no board.json"
  );

  let text = ok(dir.path(), &["wb", "status"]);
  assert!(
    text.contains(&intentsvcs::model::register_form("<moniker>")),
    "wb status names the whole form for a roster with no node:\n{text}"
  );

  let (text, code) = run(dir.path(), &["wb", "show", "zz"]);
  assert_ne!(
    code, 0,
    "wb show refuses a node that is not registered: {text}"
  );
  assert!(
    text.contains(&intentsvcs::model::register_form("zz")),
    "the refusal names the whole form, with the moniker that was typed:\n{text}"
  );
}
