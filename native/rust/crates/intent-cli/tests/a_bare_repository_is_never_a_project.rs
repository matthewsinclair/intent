//! Issue 0609: a bare git repository is never a project.
//!
//! A node launch once ran in the bare repository a working copy pushes to, the
//! two sharing a basename, and nothing objected: `intent init` wrote a project
//! beside `HEAD`, `config`, `objects` and `refs`, `intent wb register` put a
//! node in it, and `intent claude start` launched a session there. These arms
//! are the issue's acceptance for the first two verbs, driven through the real
//! binary; `claude start`'s is in `claude_cwi_door.rs`, beside its door.

use std::path::Path;

fn run(args: &[&str], cwd: &Path) -> (String, i32) {
  let out = crate::common::intent()
    .args(args)
    .current_dir(cwd)
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run the v3 binary");
  let text = format!(
    "{}{}",
    String::from_utf8_lossy(&out.stdout),
    String::from_utf8_lossy(&out.stderr)
  );
  (text, out.status.code().unwrap_or(-1))
}

fn git_init_bare(dir: &Path) {
  let status = std::process::Command::new("git")
    .args(["init", "-q", "--bare"])
    .arg(dir)
    .status()
    .expect("run git");
  assert!(status.success(), "git init --bare failed");
}

/// Every path under `dir`, so "writes nothing" is a comparison, not a hope.
fn tree(dir: &Path) -> Vec<String> {
  fn walk(root: &Path, dir: &Path, out: &mut Vec<String>) {
    for entry in std::fs::read_dir(dir).expect("read the fixture").flatten() {
      let path = entry.path();
      out.push(
        path
          .strip_prefix(root)
          .expect("under root")
          .display()
          .to_string(),
      );
      if path.is_dir() {
        walk(root, &path, out);
      }
    }
  }
  let mut out = Vec::new();
  walk(dir, dir, &mut out);
  out.sort();
  out
}

/// In a fresh `git init --bare`, `intent init` exits 1, names the repository and
/// where a project belongs instead, and writes nothing.
#[test]
fn init_in_a_bare_repository_refuses_and_writes_nothing() {
  let dir = tempfile::tempdir().expect("tempdir");
  let bare = dir.path().join("x.git");
  git_init_bare(&bare);
  let before = tree(&bare);

  let (text, code) = run(&["init", "stray"], &bare);
  assert_eq!(code, 1, "init in a bare repository must be refused: {text}");
  for said in ["is a bare git repository", "working copy", "git clone"] {
    assert!(
      text.contains(said),
      "the refusal does not say `{said}`: {text}"
    );
  }
  assert_eq!(tree(&bare), before, "init wrote into the bare repository");
}

/// In a bare repository holding an `intent/` tree, as the incident left one,
/// `intent wb register` exits 1: the project is inert.
#[test]
fn a_project_inside_a_bare_repository_is_refused() {
  let dir = tempfile::tempdir().expect("tempdir");
  let (text, code) = run(&["init", "stray"], dir.path());
  assert_eq!(code, 0, "the fixture's init: {text}");
  git_init_bare(dir.path());

  let (text, code) = run(
    &[
      "wb",
      "register",
      "dc",
      "--name",
      "DevX Claude",
      "--role",
      "worker",
    ],
    dir.path(),
  );
  assert_eq!(
    code, 1,
    "a project in a bare repository must be refused: {text}"
  );
  assert!(
    text.contains("is a bare git repository"),
    "the refusal does not say why: {text}"
  );
}
