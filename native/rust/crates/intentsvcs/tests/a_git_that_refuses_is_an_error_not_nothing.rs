//! Issue Ib7a70c3: **A GIT THAT REFUSES IS AN ERROR, NOT "NOTHING".** Inside a
//! repository git cannot read, `gitstate::staged` answered that nothing was
//! staged and `gitstate::export_ignore_decided` that nothing decided
//! `export-ignore`, because both asked `gitstate::is_work_tree`, which read a git
//! that could not be asked as no work tree. Issue 0730 gave git one answer: a git
//! that refuses is a `GitStateError` at every door. `is_work_tree` is retired,
//! and every caller asks `work_tree`, the three-answer reader: these two,
//! `uncommitted` and `grep_head`; the facade's guards of them
//! (`event_committed`, `text_at_head`, `text_in_next_commit`); its other four
//! (`name_boards_reshaped_since_head` and `scan_next_commit`, which note it,
//! `pulled_boards` and `sync_plan`, which refuse); and `sync::tree_state`, whose
//! private copy is gone, so the upgrade's dirty-tree guard is never skipped for
//! a tree git would not show. The arms here drive the two gitstate readers and
//! `tree_state`, one representative of the callers.
//!
//! The repository is a `.git` file naming a git directory that does not exist,
//! so the `.git` probe finds a repository and git refuses to read it.

use intentsvcs::gitstate::{self, GitStateError};

fn a_repository_git_refuses() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let gitdir = dir.path().join("no-such-git-directory");
  std::fs::write(
    dir.path().join(".git"),
    format!("gitdir: {}\n", gitdir.display()),
  )
  .expect("write the .git file");
  assert!(
    matches!(
      gitstate::work_tree(dir.path()),
      Err(GitStateError::Refused { .. })
    ),
    "the fixture needs git to refuse this repository"
  );
  dir
}

#[test]
fn staged_refuses_where_git_refuses() {
  let dir = a_repository_git_refuses();
  let answer = gitstate::staged(dir.path(), &["a.md".to_string()]);
  assert!(
    matches!(answer, Err(GitStateError::Refused { .. })),
    "a git that refuses is an error, not nothing staged: {answer:?}"
  );
}

#[test]
fn export_ignore_decided_refuses_where_git_refuses() {
  let dir = a_repository_git_refuses();
  let answer = gitstate::export_ignore_decided(dir.path(), ".githooks");
  assert!(
    matches!(answer, Err(GitStateError::Refused { .. })),
    "a git that refuses is an error, not nothing decided: {answer:?}"
  );
}

/// The upgrade's dirty-tree guard read the tree through `sync::tree_state`,
/// whose own copy of the work-tree reader answered "no work tree" for a git
/// that refused, and a format move with no work tree keeps no guard. Named by
/// the variants that existed before, so the arm compiles and goes red there.
#[test]
fn tree_state_is_not_no_work_tree_where_git_refuses() {
  use intentsvcs::sync::{TreeState, tree_state};
  let dir = a_repository_git_refuses();
  let state = tree_state(dir.path());
  assert!(
    !matches!(
      state,
      TreeState::NoWorkTree | TreeState::Clean | TreeState::Dirty(_)
    ),
    "a git that refuses is its own answer, not no work tree, clean or dirty"
  );
}
