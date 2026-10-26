//! Issue 0645: **A HOOK'S GIT ENVIRONMENT NAMES THE HOOK'S REPOSITORY, AND NO
//! OTHER TREE IS READ THROUGH IT.**
//!
//! A pre-commit hook in a linked worktree runs with an absolute `GIT_DIR` (the
//! worktree's gitdir), and with a temporary `GIT_INDEX_FILE` under
//! `commit --only`. The tree function spawned `git -C <install root>` and
//! inherited both, so it read the hook's repository for the install tree's
//! commit, got `bad object`, and doctor refused every gated commit made from a
//! worktree, in every estate.
//!
//! The arms drive the binary, because the environment is the CHILD's: a library
//! arm would have to set the test process's own environment, which every
//! parallel test shares. Each fixture is a linked worktree of a scratch
//! repository, so `GIT_DIR` is absolute exactly as a hook receives it.
//!
//! The control is the other half of the ruling: a read of the PROJECT's tree
//! must keep the hook's environment, since that is how it finds the index being
//! committed.

use std::path::{Path, PathBuf};
use std::process::Output;

use crate::critic_surface::{git_in, planted_rules};

/// A scratch repository with one commit and a linked worktree of it, and the
/// worktree's absolute gitdir: the `GIT_DIR` a hook run in it receives.
fn linked_worktree(dir: &Path) -> (PathBuf, PathBuf) {
  let repo = dir.join("repo");
  std::fs::create_dir_all(&repo).expect("the scratch repository");
  git_in(&repo, &["init", "-q", "."]);
  git_in(
    &repo,
    &[
      "-c",
      "user.name=t",
      "-c",
      "user.email=t@example.com",
      "commit",
      "-q",
      "--allow-empty",
      "-m",
      "base",
    ],
  );
  git_in(&repo, &["worktree", "add", "-q", "--detach", "../wt"]);
  let wt = dir.join("wt");
  let gitdir =
    std::fs::canonicalize(repo.join(".git/worktrees/wt")).expect("the worktree's gitdir");
  assert!(gitdir.is_absolute());
  (wt, gitdir)
}

fn run(args: &[&str], cwd: &Path, env: &[(&str, &Path)]) -> Output {
  let mut cmd = crate::common::intent();
  cmd
    .args(args)
    .current_dir(cwd)
    .stdin(testkit::lifeline_for(args));
  for (k, v) in env {
    cmd.env(k, v);
  }
  cmd.output().expect("run the v3 binary")
}

fn text(out: &Output) -> String {
  format!(
    "{}{}",
    String::from_utf8_lossy(&out.stdout),
    String::from_utf8_lossy(&out.stderr)
  )
}

/// The version line, and so the tree function under it, answers the install
/// tree's commit and dirty state under a foreign absolute `GIT_DIR` exactly as
/// it does without one.
#[test]
fn the_version_line_reads_the_install_tree_under_a_foreign_git_dir() {
  let dir = tempfile::tempdir().expect("tempdir");
  let (wt, gitdir) = linked_worktree(dir.path());

  let plain = run(&["--version"], &wt, &[]);
  let hooked = run(&["--version"], &wt, &[("GIT_DIR", gitdir.as_path())]);

  // VACUITY GUARD: the baseline names a commit, so equality below compares a
  // read that answered, not two lines that both gave up.
  let line = String::from_utf8_lossy(&plain.stdout).into_owned();
  assert_eq!(plain.status.code(), Some(0), "{}", text(&plain));
  assert!(
    !line.contains("unknown"),
    "the baseline must name the build's commit, or this arm compares nothing: {line}"
  );
  assert_eq!(
    String::from_utf8_lossy(&hooked.stdout),
    line,
    "a hook's GIT_DIR changed the version line, so the tree function read the hook's \
     repository for the install tree's commit\nstderr: {}",
    String::from_utf8_lossy(&hooked.stderr)
  );
}

/// Doctor under a linked worktree's hook environment reports no install-tree
/// finding, and the same install findings as without it.
#[test]
fn doctor_under_a_worktree_hooks_git_dir_reports_no_install_tree_finding() {
  let dir = tempfile::tempdir().expect("tempdir");
  let (wt, gitdir) = linked_worktree(dir.path());
  intentsvcs::init::init(&wt, "Hooked", "test", env!("CARGO_PKG_VERSION"))
    .expect("the shipped initialiser creates a project in the worktree");

  let install_lines = |out: &Output| -> Vec<String> {
    text(out)
      .lines()
      .filter(|l| l.contains("install-"))
      .map(str::to_string)
      .collect()
  };
  let plain = run(&["doctor"], &wt, &[]);
  let hooked = run(&["doctor"], &wt, &[("GIT_DIR", gitdir.as_path())]);

  assert!(
    !text(&hooked).contains("install-tree-unknown"),
    "doctor under the hook's GIT_DIR reported the install tree unreadable:\n{}",
    text(&hooked)
  );
  assert_eq!(
    install_lines(&hooked),
    install_lines(&plain),
    "the install findings differ under the hook's GIT_DIR\nwith: {}\nwithout: {}",
    text(&hooked),
    text(&plain)
  );
}

/// The control: a read of the project's own tree keeps the hook's environment,
/// so the staged set is the one `GIT_INDEX_FILE` names, as under
/// `commit --only`.
#[test]
fn a_project_tree_read_under_the_hooks_environment_sees_the_hooks_index() {
  let dir = tempfile::tempdir().expect("tempdir");
  let (wt, gitdir) = linked_worktree(dir.path());
  let rules = planted_rules(&wt, &[("IN-SH-TEST-953", &["**/*.sh"])]);
  std::fs::write(wt.join("s.sh"), "#!/bin/bash\necho PLANTED\n").expect("the planted script");
  // Staged in a second index only, as `commit --only` stages for its hook; the
  // worktree's own index holds nothing.
  let index = dir.path().join("hook-index");
  let st = std::process::Command::new("git")
    .args(["add", "s.sh"])
    .current_dir(&wt)
    .env("GIT_DIR", gitdir.as_path())
    .env("GIT_INDEX_FILE", &index)
    .status()
    .expect("git");
  assert!(
    st.success(),
    "fixture setup failed: git add into the hook's index"
  );

  let args = [
    "critic",
    "shell",
    "--rules",
    rules.to_str().expect("utf-8 path"),
    "--staged",
  ];
  let hooked = run(
    &args,
    &wt,
    &[
      ("GIT_DIR", gitdir.as_path()),
      ("GIT_INDEX_FILE", index.as_path()),
    ],
  );
  let own = run(&args, &wt, &[("GIT_DIR", gitdir.as_path())]);

  assert_eq!(
    hooked.status.code(),
    Some(1),
    "the staged read lost the hook's GIT_INDEX_FILE and judged another index:\n{}",
    text(&hooked)
  );
  // The worktree's own index stages nothing, so no rule was asked of any file:
  // 4, nothing judged, which the gate passes (issue 0648).
  assert_eq!(
    own.status.code(),
    Some(4),
    "the worktree's own index holds nothing, so the finding above came from the \
     hook's index and not from the work tree:\n{}",
    text(&own)
  );
}
