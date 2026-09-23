//! Issue 0523: `intent wb edit <kind> <id> <text> --node <moniker>` is a
//! declared door. It answers where the old text went, prints the same text
//! back as unchanged, and refuses an address the board does not carry with the
//! board to read. The two cases a commit decides are held by
//! `intentsvcs/tests/wb_edit_keeps_the_old_text_out_of_a_commit.rs`; this
//! drives the binary over a project no commit holds, so the draft is amended,
//! and over a git work tree where a file the edit does not rewrite still holds
//! the old text, so the answer names it and withholds the all-clear.

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

/// Make `dir` a git work tree, so the edit's scan of what the next commit
/// would carry has a repository to ask.
fn git_init(dir: &Path) {
  let git = std::process::Command::new("git")
    .args(["init", "-q"])
    .current_dir(dir)
    .status()
    .expect("run git init");
  assert!(git.success(), "git init");
}

/// Commit everything in `dir`, with no user config, signing or hooks of the
/// machine running the suite.
fn git_commit_all(dir: &Path) {
  for args in [
    vec!["add", "-A"],
    vec![
      "-c",
      "user.name=t",
      "-c",
      "user.email=t@example.com",
      "-c",
      "commit.gpgSign=false",
      "-c",
      "core.hooksPath=/dev/null",
      "commit",
      "-q",
      "-m",
      "seed",
    ],
  ] {
    let git = std::process::Command::new("git")
      .args(&args)
      .current_dir(dir)
      .status()
      .expect("run git");
    assert!(git.success(), "git {args:?}");
  }
}

/// Every file under `intent/` holding `needle`, project-relative and sorted.
/// The store under `.cache/` is left out, as the edit's own scan leaves it.
fn holding(root: &Path, needle: &str) -> Vec<String> {
  fn walk(dir: &Path, root: &Path, needle: &str, out: &mut Vec<String>) {
    for entry in std::fs::read_dir(dir).expect("read dir") {
      let path = entry.expect("entry").path();
      if path.is_dir() {
        if path.file_name().is_some_and(|n| n != ".cache") {
          walk(&path, root, needle, out);
        }
      } else if std::fs::read(&path)
        .expect("read file")
        .windows(needle.len())
        .any(|w| w == needle.as_bytes())
      {
        out.push(
          path
            .strip_prefix(root)
            .expect("under the root")
            .display()
            .to_string(),
        );
      }
    }
  }
  let mut out = Vec::new();
  walk(&root.join("intent"), root, needle, &mut out);
  out.sort();
  out
}

/// A project whose `cc` board holds one todo.
fn seeded() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  ok(dir.path(), &["init", "wbedit"]);
  ok(
    dir.path(),
    &[
      "wb",
      "register",
      "cc",
      "--name",
      "Control Claude",
      "--role",
      "control",
    ],
  );
  ok(
    dir.path(),
    &["wb", "add", "todo", "client ACME-4471 owes", "--node", "cc"],
  );
  dir
}

#[test]
fn an_edit_says_where_the_old_text_went_and_the_board_reads_the_new() {
  let dir = seeded();

  let said = ok(
    dir.path(),
    &["wb", "edit", "todo", "1", "a client owes", "--node", "cc"],
  );

  assert!(said.contains("ok: cc todo 1 edited"), "{said}");
  assert!(
    said.contains("no file under intent/ holds the old text, at HEAD or in the next commit"),
    "{said}"
  );
  let shown = ok(dir.path(), &["wb", "show", "cc"]);
  assert!(shown.contains("a client owes"), "{shown}");
  assert!(!shown.contains("ACME-4471"), "{shown}");
}

#[test]
fn the_same_text_again_is_reported_unchanged() {
  let dir = seeded();

  let said = ok(
    dir.path(),
    &[
      "wb",
      "edit",
      "todo",
      "1",
      "client ACME-4471 owes",
      "--node",
      "cc",
    ],
  );

  assert!(said.contains("ok: cc todo 1 unchanged"), "{said}");
}

#[test]
fn a_number_the_board_does_not_carry_is_refused_with_the_board_to_read() {
  let dir = seeded();

  let (said, code) = run(
    dir.path(),
    &["wb", "edit", "todo", "9", "a client owes", "--node", "cc"],
  );

  assert_eq!(code, 1, "{said}");
  assert!(said.contains("`cc` has no `todo` 9"), "{said}");
  assert!(said.contains("intent wb show cc"), "{said}");
}

#[test]
fn a_kind_the_table_does_not_declare_is_refused() {
  let dir = seeded();

  let (said, code) = run(
    dir.path(),
    &[
      "wb",
      "edit",
      "nonsense",
      "1",
      "a client owes",
      "--node",
      "cc",
    ],
  );

  assert_eq!(code, 1, "{said}");
  assert!(said.contains("nonsense"), "{said}");
}

#[test]
fn a_file_the_edit_cannot_rewrite_is_named_and_no_commit_is_not_claimed() {
  // vc's arm for v4: `wb migrate` keeps the hand-authored board under
  // `.history/pre-migration/`, which no carrier rule looks at. Before the
  // carry is committed that copy is untracked and still holds the old text,
  // so the answer names it and gives no all-clear.
  let dir = tempfile::tempdir().expect("tempdir");
  ok(dir.path(), &["init", "wbedit"]);
  git_init(dir.path());
  let board = dir.path().join("intent/whiteboard/cc");
  std::fs::create_dir_all(&board).expect("cc's directory");
  std::fs::write(
    board.join("wip.md"),
    "---\nnode: cc\nname: Control Claude\nrole: control\nstatus: active\n---\n\n# Control Claude (cc)\n\n## TODO\n\n- client ACME-4471 owes\n",
  )
  .expect("a hand-authored board");
  ok(dir.path(), &["wb", "register"]);
  ok(dir.path(), &["wb", "migrate", "cc"]);

  let said = ok(
    dir.path(),
    &["wb", "edit", "todo", "1", "a client owes", "--node", "cc"],
  );

  assert!(
    said.contains("the next commit could carry the old text in")
      && said.contains("intent/whiteboard/cc/.history/pre-migration/"),
    "{said}"
  );
  assert!(!said.contains("no file under intent/ holds"), "{said}");
  assert!(
    !said.contains("the new text contains the old text"),
    "the new text does not contain the old, so there is no hint: {said}"
  );
}

#[test]
fn a_new_text_holding_the_old_lists_every_file_and_says_why() {
  // vc's ruling on v4: an edit that only adds to what an item said leaves the
  // old text wherever the new text is. The list stays whole, since a
  // redaction that kept the text must see exactly this, and one line says why
  // it is long, so an addition does not read as a failed edit.
  let dir = tempfile::tempdir().expect("tempdir");
  ok(dir.path(), &["init", "wbedit"]);
  git_init(dir.path());
  ok(
    dir.path(),
    &[
      "wb",
      "register",
      "cc",
      "--name",
      "Control Claude",
      "--role",
      "control",
    ],
  );
  ok(
    dir.path(),
    &["wb", "add", "todo", "a client owes", "--node", "cc"],
  );

  let said = ok(
    dir.path(),
    &[
      "wb",
      "edit",
      "todo",
      "1",
      "a client owes the renewal",
      "--node",
      "cc",
    ],
  );

  let listed = said
    .lines()
    .find_map(|l| l.strip_prefix("  the next commit could carry the old text in "))
    .unwrap_or_else(|| panic!("the list is printed: {said}"));
  let mut listed: Vec<String> = listed.split(", ").map(str::to_string).collect();
  listed.sort();
  assert_eq!(
    listed,
    holding(dir.path(), "a client owes"),
    "the list is every file holding the old text: {said}"
  );
  assert!(
    said.contains(
      "the new text contains the old text, so every file holding the new text also holds the old text"
    ),
    "{said}"
  );
  assert!(!said.contains("no file under intent/ holds"), "{said}");
}

/// A git work tree whose `cc` and `vc` boards are registered, committed.
fn two_committed_boards() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  ok(dir.path(), &["init", "wbedit"]);
  git_init(dir.path());
  for (node, name, role) in [
    ("cc", "Control Claude", "control"),
    ("vc", "Validation Claude", "validation"),
  ] {
    ok(
      dir.path(),
      &["wb", "register", node, "--name", name, "--role", role],
    );
  }
  git_commit_all(dir.path());
  dir
}

#[test]
fn a_peers_committed_copy_is_named_at_head_and_no_all_clear_is_given() {
  // ic's review of v4, driven: a peer's committed item quoting the text is
  // in HEAD's tree, so the next commit carries it as well. The answer names
  // the peer's files and does not say no file holds the old text.
  let dir = two_committed_boards();
  let secret = "client Acme renewal, secret marker Q7";
  ok(dir.path(), &["wb", "add", "todo", secret, "--node", "vc"]);
  git_commit_all(dir.path());
  ok(dir.path(), &["wb", "add", "todo", secret, "--node", "cc"]);

  let said = ok(
    dir.path(),
    &["wb", "edit", "todo", "1", "client redacted", "--node", "cc"],
  );

  let at_head = said
    .lines()
    .find_map(|l| l.strip_prefix("  HEAD already carries the old text in "))
    .unwrap_or_else(|| panic!("the HEAD line is printed: {said}"));
  assert!(
    at_head.contains("intent/whiteboard/vc/board.json")
      && at_head.contains("intent/whiteboard/vc/wip.md"),
    "{said}"
  );
  assert!(!said.contains("no file under intent/ holds"), "{said}");
}

#[test]
fn a_clean_head_and_an_amended_draft_give_the_all_clear() {
  // The control for the arm above: HEAD holds nothing of the text and the
  // draft carrying it is amended, so both searches come back empty and the
  // answer says so, with what it searched.
  let dir = two_committed_boards();
  ok(
    dir.path(),
    &["wb", "add", "todo", "client Acme renewal", "--node", "cc"],
  );

  let said = ok(
    dir.path(),
    &["wb", "edit", "todo", "1", "client redacted", "--node", "cc"],
  );

  assert!(
    said.contains("edited in its uncommitted event")
      && said.contains("no file under intent/ holds the old text, at HEAD or in the next commit"),
    "{said}"
  );
  assert!(!said.contains("HEAD already carries"), "{said}");
}

#[test]
fn to_is_refused_on_an_item_and_required_for_a_message() {
  // `--to` is what tells an item from a message, so each form refuses the
  // other's shape, and each refusal shows both forms.
  let dir = seeded();

  let (said, code) = run(
    dir.path(),
    &[
      "wb",
      "edit",
      "todo",
      "1",
      "a client owes",
      "--to",
      "vc",
      "--node",
      "cc",
    ],
  );
  assert_eq!(code, 1, "{said}");
  assert!(
    said.contains("`--to` names the recipient of a message"),
    "{said}"
  );

  let (said, code) = run(
    dir.path(),
    &[
      "wb",
      "edit",
      "message",
      "2026-09-23 09:00Z",
      "a client owes",
      "--node",
      "cc",
    ],
  );
  assert_eq!(code, 1, "{said}");
  assert!(said.contains("needs `--to <recipient>`"), "{said}");
}

#[test]
fn a_message_you_sent_is_edited_by_its_heading_and_names_its_recipient() {
  let dir = seeded();
  ok(
    dir.path(),
    &[
      "wb",
      "register",
      "vc",
      "--name",
      "Validation Claude",
      "--role",
      "validation",
    ],
  );
  ok(
    dir.path(),
    &["wb", "ask", "vc", "client ACME-4471 owes", "--node", "cc"],
  );
  let inbox = std::fs::read_to_string(dir.path().join("intent/whiteboard/vc/inbox.cc.md"))
    .expect("vc's inbox from cc");
  let anchor = inbox
    .lines()
    .find_map(|l| l.strip_prefix("## ("))
    .and_then(|l| l.split(')').next())
    .expect("a heading")
    .to_string();

  let said = ok(
    dir.path(),
    &[
      "wb",
      "edit",
      "message",
      &anchor,
      "a client owes",
      "--to",
      "vc",
      "--node",
      "cc",
    ],
  );

  assert!(
    said.contains(&format!(
      "ok: cc message {anchor} for vc edited in its uncommitted event"
    )),
    "{said}"
  );
  let shown = ok(dir.path(), &["wb", "show", "vc"]);
  assert!(
    shown.contains("a client owes") && !shown.contains("ACME-4471"),
    "{shown}"
  );
}

#[test]
fn wb_show_marks_an_edited_item() {
  // Issue 0525: the CLI's own listing says so as well as the views do.
  let dir = seeded();
  ok(
    dir.path(),
    &["wb", "edit", "todo", "1", "a client owes", "--node", "cc"],
  );

  let shown = ok(dir.path(), &["wb", "show", "cc"]);

  assert!(shown.contains("[todo] 1 a client owes (edited)"), "{shown}");
}
