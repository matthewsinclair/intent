//! Issue Ib7a70c3: **A GIT THAT REFUSES IS AN ERROR, NOT "NOTHING".** Inside a
//! repository git cannot read, `gitstate::staged` answered that nothing was
//! staged and `gitstate::export_ignore_decided` that nothing decided
//! `export-ignore`, because both asked `is_work_tree`, which reads a git that
//! could not be asked as no work tree. Issue 0730 gave git one answer: a git
//! that refuses is a `GitStateError` at every door. Both now ask `work_tree`,
//! as do `uncommitted` and `grep_head`.
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
