//! Issue 0535, driven through the binary. A fresh clone's first open takes its
//! boards. A store an Intent before 0535 warmed, which holds the estate and not
//! its boards, is sent to `sync --to-store`, and no door writes a board over a
//! file that store has not taken in.
//!
//! The facade arms are in intentsvcs' `a_fresh_clone_takes_its_boards.rs`. This
//! file holds the doors a person types, because a facade arm is not the CLI:
//! `wb status` printed its "no nodes are registered" remedy from the renderer,
//! where no facade arm could see it.

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

/// A project whose store holds cc's migrated board and one issue.
fn seeded() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let steps: [&[&str]; 4] = [
    &["init", "wbclone"],
    &[
      "wb", "register", "cc", "--name", "Control", "--role", "control",
    ],
    &["wb", "add", "doing", "carry the boards", "--node", "cc"],
    &["issues", "add", "a filed issue"],
  ];
  for args in steps {
    let (text, code) = run(dir.path(), args);
    assert_eq!(code, 0, "{args:?}: {text}");
  }
  dir
}

/// The project as a clone has it: every file but the store, which is
/// gitignored and so never travels.
fn clone_of(estate: &Path) -> tempfile::TempDir {
  fn copy(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).expect("mkdir");
    for entry in std::fs::read_dir(from).expect("read_dir") {
      let entry = entry.expect("entry");
      let (source, target) = (entry.path(), to.join(entry.file_name()));
      if source.ends_with("intent/.cache") {
        continue;
      }
      if entry.file_type().expect("file_type").is_dir() {
        copy(&source, &target);
      } else {
        std::fs::copy(&source, &target).expect("copy");
      }
    }
  }
  let dir = tempfile::tempdir().expect("tempdir");
  copy(estate, dir.path());
  dir
}

/// The warm an Intent before 0535 ran on a clone's first open: the threads and
/// issues the extract carries, and no board.
fn warm_as_before_0535(clone: &Path) {
  let project = intentsvcs::project::Project::open(clone).expect("the clone's project");
  let canon = intentsvcs::ingest::read(&project).expect("the extract reads");
  intentsvcs::store::Store::open(&project.db_path())
    .expect("the clone's store")
    .rebuild(&canon.threads, &canon.issues)
    .expect("the older warm");
}

#[test]
fn a_fresh_clone_lists_its_nodes_on_its_first_open() {
  let estate = seeded();
  let clone = clone_of(estate.path());
  assert!(
    !clone.path().join("intent/.cache/intent.db").exists(),
    "precondition: a clone carries no store"
  );
  let (text, code) = run(clone.path(), &["wb", "status"]);
  assert_eq!(code, 0, "{text}");
  assert!(
    text.starts_with("cc (control) Control"),
    "the first open took cc's board: {text}"
  );
}

#[test]
fn a_clone_warmed_without_its_boards_is_sent_to_sync_to_store() {
  let estate = seeded();
  let clone = clone_of(estate.path());
  warm_as_before_0535(clone.path());
  let board = clone.path().join("intent/whiteboard/cc/board.json");
  let before = std::fs::read(&board).expect("cc's board.json");

  let (text, code) = run(clone.path(), &["wb", "status"]);
  assert_ne!(
    code, 0,
    "wb status refuses an empty roster beside a board: {text}"
  );
  assert!(
    text.contains("intent sync --to-store") && !text.contains("`intent wb register`"),
    "wb status names the restore, not the register: {text}"
  );
  for args in [&["wb", "register"][..], &["sync", "--to-disk"][..]] {
    let (text, code) = run(clone.path(), args);
    assert_ne!(code, 0, "{args:?} refuses: {text}");
    assert!(
      text.contains("intent sync --to-store"),
      "{args:?} names the restore: {text}"
    );
  }
  assert_eq!(
    std::fs::read(&board).expect("cc's board.json"),
    before,
    "no refused door wrote cc's board"
  );

  let (text, code) = run(clone.path(), &["sync", "--to-store"]);
  assert_eq!(code, 0, "sync --to-store: {text}");
  let (text, code) = run(clone.path(), &["wb", "status"]);
  assert_eq!(code, 0, "{text}");
  assert!(
    text.starts_with("cc (control) Control"),
    "the restore took cc's board: {text}"
  );
}
