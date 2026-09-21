//! **`wb migrate` labels its `.history` count by what it counts, and names the
//! pre-migration copies it keeps (issue 0499).**
//!
//! The closing line read `N snapshot(s)` over every `.history` document the
//! carry took in, fold archives included, while the refusal's remedy calls the
//! pre-migration copies snapshots. On Conflab's carry on 2026-09-21 a node with
//! one copy read `29 snapshot(s)`, and the carrier could not tell from the
//! report where the dropped units went.
//!
//! Driven through the binary, because the lines are the CLI's rendering of the
//! facade's value and a facade arm would not see the rendering.

use std::path::Path;

/// A board with a lead paragraph above its first section: a unit the model
/// cannot carry, so `--drop-uncarried` keeps a copy of the file.
const BOARD: &str = "---\nnode: dc\nname: DevX Claude\nrole: worker\nsession_id: none\nstatus: paused\nfocus: \"a lead paragraph to drop\"\nclaims: []\n---\n\n# DevX Claude (dc)\n\n> A lead paragraph the model has no field for.\n\n## DOING\n\n- The one thing on the board.\n";

/// A fold archive already under the node's `.history/`: carried as a document,
/// counted, and not a copy of anything this carry dropped.
const ARCHIVE: &str = "# An archived board\n\n- Something that was done in August.\n";

/// An inbox with a hand-written line above its first entry, a unit the model
/// cannot carry, so it is kept too.
const INBOX_WITH_A_NOTE: &str = "# inbox: vc -> dc\n\n_A note somebody wrote by hand above the entries._\n\n## (2026-08-01 10:00Z)\n\nA message.\n";

/// An inbox the model carries entry by entry, the negative control: nothing is
/// left over, so no copy is kept and none is named.
const CLEAN_INBOX: &str = "# inbox: dc -> vc\n\n## (2026-08-01 11:00Z)\n\nAnother message.\n";

/// A peer whose board carries whole: the board is still kept (issue 0438), and
/// its clean inbox is not.
const PEER: &str = "---\nnode: vc\nname: Validation Claude\nrole: validation\nstatus: active\n---\n\n# Validation Claude (vc)\n\n## TODO\n\n- A queued thing.\n";

fn run(cwd: &Path, args: &[&str]) -> (String, i32) {
  let out = crate::common::intent()
    .args(args)
    .current_dir(cwd)
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

fn registered() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let (text, code) = run(dir.path(), &["init", "kept"]);
  assert_eq!(code, 0, "init: {text}");
  for (node, board) in [("dc", BOARD), ("vc", PEER)] {
    let home = dir.path().join("intent/whiteboard").join(node);
    std::fs::create_dir_all(&home).expect("the node's directory");
    std::fs::write(home.join("wip.md"), board).expect("the board");
  }
  let wb = dir.path().join("intent/whiteboard");
  std::fs::write(wb.join("dc/inbox.vc.md"), INBOX_WITH_A_NOTE).expect("dc's inbox");
  std::fs::write(wb.join("vc/inbox.dc.md"), CLEAN_INBOX).expect("vc's inbox");
  let archive = dir.path().join("intent/whiteboard/dc/.history/20260801");
  std::fs::create_dir_all(&archive).expect("the archive's directory");
  std::fs::write(archive.join("wip.md"), ARCHIVE).expect("the archive");
  let (text, code) = run(dir.path(), &["wb", "register"]);
  assert_eq!(code, 0, "register: {text}");
  dir
}

fn kept_lines(text: &str) -> Vec<&str> {
  text
    .lines()
    .filter_map(|l| l.strip_prefix("kept: "))
    .collect()
}

#[test]
fn the_count_is_labelled_by_the_directory_and_each_kept_copy_is_named() {
  let dir = registered();

  let (text, code) = run(dir.path(), &["wb", "migrate", "dc", "--drop-uncarried"]);
  assert_eq!(code, 0, "migrate dc: {text}");
  assert!(
    text.contains("3 .history document(s) carried"),
    "the archive and the two copies are all `.history` documents, and the label says so: {text}"
  );
  assert!(
    !text.contains("snapshot(s)"),
    "the count no longer borrows the word the remedy uses for the copies: {text}"
  );
  let board = "intent/whiteboard/dc/.history/pre-migration/wip.md";
  let inbox = "intent/whiteboard/dc/.history/pre-migration/inbox.vc.md";
  let said = |copy: &str| format!("{copy} -- the file as the carry read it, byte for byte");
  assert_eq!(
    kept_lines(&text),
    [said(inbox).as_str(), said(board).as_str()],
    "both copies holding a dropped unit are named, in path order, and the archive is not: {text}"
  );
  for (copy, bytes) in [(board, BOARD), (inbox, INBOX_WITH_A_NOTE)] {
    let on_disk = std::fs::read_to_string(dir.path().join(copy))
      .unwrap_or_else(|e| panic!("the named copy {copy} exists: {e}"));
    assert_eq!(
      on_disk, bytes,
      "{copy} is the file as it was, byte for byte"
    );
  }

  // THE NEGATIVE CONTROL: vc's inbox carries entry by entry, so no copy of it
  // is kept and none is named. Its board is kept whole, as every board is.
  let (text, code) = run(dir.path(), &["wb", "migrate", "vc"]);
  assert_eq!(code, 0, "migrate vc: {text}");
  assert_eq!(
    kept_lines(&text),
    [said("intent/whiteboard/vc/.history/pre-migration/wip.md").as_str()],
    "the clean inbox is not named: {text}"
  );
  assert!(
    !dir
      .path()
      .join("intent/whiteboard/vc/.history/pre-migration/inbox.dc.md")
      .exists(),
    "and no copy of it was written"
  );
  assert!(
    text.contains("1 .history document(s) carried"),
    "the board's copy is vc's one `.history` document: {text}"
  );
}
