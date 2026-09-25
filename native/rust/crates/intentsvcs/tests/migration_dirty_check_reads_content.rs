//! **THE MIGRATION'S DIRTY CHECK READS CONTENT, NOT STAT DATA.**
//!
//! `intent upgrade` refuses a tree with uncommitted changes. It asked
//! `git diff-index`, which compares the index's cached stat data, so a file
//! whose mtime moved and whose bytes did not was listed as uncommitted: driven,
//! 25 listed where `git status` showed 1. The control is a real content change,
//! which must still be caught.

use std::path::Path;
use std::process::Command;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use intentsvcs::sync::{TreeState, tree_state};

fn git(root: &Path, args: &[&str]) {
  let out = Command::new("git")
    .args(["-c", "init.defaultBranch=main"])
    .args(["-c", "user.name=dirty-check"])
    .args(["-c", "user.email=dirty-check@invalid"])
    .args(["-c", "commit.gpgsign=false"])
    .args(args)
    .current_dir(root)
    .env_remove("GIT_DIR")
    .env_remove("GIT_INDEX_FILE")
    .env_remove("GIT_WORK_TREE")
    .output()
    .expect("run git");
  assert!(
    out.status.success(),
    "git {args:?}: {}",
    String::from_utf8_lossy(&out.stderr)
  );
}

fn committed_tree() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  std::fs::write(dir.path().join("a.md"), "unchanged\n").expect("write");
  git(dir.path(), &["init", "--quiet"]);
  git(dir.path(), &["add", "a.md"]);
  git(dir.path(), &["commit", "--quiet", "-m", "fixture"]);
  dir
}

fn fixed_instant() -> SystemTime {
  UNIX_EPOCH + Duration::from_secs(2_000_000_000)
}

#[test]
fn a_file_whose_only_change_is_its_mtime_is_not_uncommitted() {
  let dir = committed_tree();
  let file = std::fs::File::options()
    .write(true)
    .open(dir.path().join("a.md"))
    .expect("open");
  file
    // A fixed instant, not the clock: the workspace reads no clock
    // but the daemon log's (one_clock), and any time other than the checkout's
    // own moves the stat data.
    .set_modified(fixed_instant())
    .expect("move the mtime");
  drop(file);

  match tree_state(dir.path()) {
    TreeState::Clean => {}
    TreeState::Dirty(paths) => panic!("a stat-only change was reported as uncommitted: {paths:?}"),
    TreeState::NoWorkTree => panic!("the fixture read as having no work tree"),
  }
}

#[test]
fn a_content_change_is_still_uncommitted() {
  let dir = committed_tree();
  std::fs::write(dir.path().join("a.md"), "changed\n").expect("write");
  assert!(
    matches!(tree_state(dir.path()), TreeState::Dirty(ref paths) if paths.len() == 1),
    "a content change was not caught"
  );
}
