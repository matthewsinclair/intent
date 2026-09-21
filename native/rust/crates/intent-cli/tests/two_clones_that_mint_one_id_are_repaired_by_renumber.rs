//! **ST0078 WP-02 through the real binary: two clones mint one id, and one
//! verb repairs it** (AT-02.1, AT-02.2).
//!
//! ST0078's worked example E4, driven: Alice and Bob clone one origin, both run
//! `st new`, and both get `ST0001`. Before this verb Bob's repair was a
//! hand-renamed canon file, a hand-edited id and a store restore. Now it is
//! `st renumber`, after which the merge conflicts only in the generated views,
//! which are taken from either side and regenerated -- and `doctor`, the
//! verdict the commit gate runs, is clean.
//!
//! **UNDER ITS OWN `HOME`, WITH NO DAEMON**, as the design drove it: a
//! collaborator who never registered the project has no daemon watching it.

use std::path::Path;
use std::process::Command;

fn run(args: &[&str], cwd: &Path, home: &Path) -> (String, String, i32) {
  let out = crate::common::intent()
    .args(args)
    .current_dir(cwd)
    .env("HOME", home)
    .env_remove("XDG_CONFIG_HOME")
    .env_remove("XDG_DATA_HOME")
    .env_remove("XDG_STATE_HOME")
    .env_remove("XDG_RUNTIME_DIR")
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run the v3 binary");
  (
    String::from_utf8_lossy(&out.stdout).into_owned(),
    String::from_utf8_lossy(&out.stderr).into_owned(),
    out.status.code().unwrap_or(-1),
  )
}

fn ok(args: &[&str], cwd: &Path, home: &Path) -> String {
  let (out, err, code) = run(args, cwd, home);
  assert_eq!(code, 0, "`intent {}` failed:\n{out}{err}", args.join(" "));
  out
}

/// Git with an identity and no hooks, so the fixture's commits are the
/// commits and nothing else.
fn git(args: &[&str], cwd: &Path) -> (String, bool) {
  let out = Command::new("git")
    .args(["-c", "user.name=t", "-c", "user.email=t@example.com"])
    .args(["-c", "core.hooksPath=/dev/null"])
    .args(args)
    .current_dir(cwd)
    .output()
    .expect("run git");
  (
    format!(
      "{}{}",
      String::from_utf8_lossy(&out.stdout),
      String::from_utf8_lossy(&out.stderr)
    ),
    out.status.success(),
  )
}

fn git_ok(args: &[&str], cwd: &Path) -> String {
  let (out, success) = git(args, cwd);
  assert!(success, "`git {}` failed:\n{out}", args.join(" "));
  out
}

#[test]
fn a_thread_both_clones_minted_is_renumbered_and_the_merge_is_clean_under_doctor() {
  let dir = tempfile::tempdir().expect("tempdir");
  let home = tempfile::tempdir().expect("tempdir");
  let (origin, alice, bob) = (
    dir.path().join("origin.git"),
    dir.path().join("alice"),
    dir.path().join("bob"),
  );
  git_ok(
    &["init", "-q", "--bare", "-b", "main", "origin.git"],
    dir.path(),
  );
  git_ok(&["clone", "-q", "origin.git", "alice"], dir.path());
  ok(&["init", "Team"], &alice, home.path());
  git_ok(&["add", "-A"], &alice);
  git_ok(&["commit", "-q", "-m", "intent init"], &alice);
  git_ok(&["push", "-q", "origin", "HEAD:main"], &alice);
  git_ok(&["clone", "-q", "origin.git", "bob"], dir.path());
  assert!(origin.exists());

  // Both mint ST0001. Bob starts his, so it is realised and declared.
  ok(&["st", "new", "Alice's next"], &alice, home.path());
  git_ok(&["add", "-A"], &alice);
  git_ok(&["commit", "-q", "-m", "alice: ST0001"], &alice);
  git_ok(&["push", "-q", "origin", "HEAD:main"], &alice);
  ok(&["st", "new", "Bob's next"], &bob, home.path());
  ok(&["st", "start", "ST0001"], &bob, home.path());
  git_ok(&["add", "-A"], &bob);
  git_ok(&["commit", "-q", "-m", "bob: ST0001"], &bob);

  // The repair.
  let out = ok(&["st", "renumber", "ST0001", "ST0002"], &bob, home.path());
  assert!(out.contains("ok: ST0001 renumbered to ST0002"), "{out}");
  assert!(
    out.contains("moved: intent/st/ST0001 -> intent/st/ST0002"),
    "the realised directory moves and says so:\n{out}"
  );
  assert!(!bob.join("intent/.canon/st/ST0001.json").exists());
  assert!(bob.join("intent/st/ST0002/info.md").exists());
  git_ok(&["add", "-A"], &bob);
  git_ok(&["commit", "-q", "-m", "bob: renumber to ST0002"], &bob);

  // The merge now touches canon cleanly; only generated views can conflict.
  let (merge, _) = git(&["pull", "-q", "--no-rebase", "origin", "main"], &bob);
  let conflicted = git_ok(&["diff", "--name-only", "--diff-filter=U"], &bob);
  for path in conflicted.lines() {
    assert!(
      path == "intent/st/steel_threads.md" || path == "intent/todo.md",
      "after a renumber only a generated view may conflict, and {path} did:\n{merge}"
    );
  }
  if !conflicted.trim().is_empty() {
    let views: Vec<&str> = conflicted.lines().collect();
    let mut args = vec!["checkout", "--theirs", "--"];
    args.extend(views);
    git_ok(&args, &bob);
  }
  ok(&["sync", "--apply"], &bob, home.path());
  ok(&["sync", "--to-disk"], &bob, home.path());
  ok(&["todo", "update"], &bob, home.path());
  git_ok(&["add", "-A"], &bob);
  git_ok(&["commit", "-q", "-m", "merge main"], &bob);

  let list = ok(&["st", "list", "--status", "all"], &bob, home.path());
  assert!(
    list.contains("ST0001") && list.contains("Alice's next"),
    "{list}"
  );
  assert!(
    list.contains("ST0002") && list.contains("Bob's next"),
    "{list}"
  );
  let (out, err, code) = run(&["doctor"], &bob, home.path());
  assert!(
    out.contains("doctor: 0 finding(s)") && code == 0,
    "the merged clone is not clean under doctor:\n{out}{err}"
  );

  // And the id it left is taken now: renumbering onto it is refused.
  let (out, err, code) = run(&["st", "renumber", "ST0002", "ST0001"], &bob, home.path());
  assert_eq!(code, 1, "{out}{err}");
  assert!(
    err.contains("steel thread ST0001 is already taken"),
    "{err}"
  );
  assert!(err.contains("remedy:"), "{err}");
}

#[test]
fn an_issue_renumber_reports_what_it_moved_and_the_prose_it_left() {
  let project = tempfile::tempdir().expect("tempdir");
  let home = tempfile::tempdir().expect("tempdir");
  ok(&["init", "Team"], project.path(), home.path());
  ok(&["issues", "add", "First"], project.path(), home.path());
  let body = project.path().join("body.md");
  std::fs::write(&body, "Follows on from 0001.\n").expect("write body");
  ok(
    &[
      "issues",
      "add",
      "Second",
      "--from",
      body.to_str().expect("utf-8"),
    ],
    project.path(),
    home.path(),
  );

  let (out, err, code) = run(
    &["issues", "renumber", "1", "2"],
    project.path(),
    home.path(),
  );
  assert_eq!(code, 1, "0002 is taken:\n{out}{err}");
  assert!(err.contains("issue 0002 is already taken"), "{err}");

  let out = ok(
    &["issues", "renumber", "1", "7"],
    project.path(),
    home.path(),
  );
  assert!(out.contains("ok: issue 0001 renumbered to 0007"), "{out}");
  assert!(
    out.contains("moved: intent/.canon/issues/0001.json removed"),
    "{out}"
  );
  assert!(
    out
      .lines()
      .any(|l| l.starts_with("prose: ") && l.contains("0002")),
    "issue 0002's body names 0001 and is reported, not rewritten:\n{out}"
  );
  let shown = ok(&["issues", "show", "2"], project.path(), home.path());
  assert!(shown.contains("Follows on from 0001."), "{shown}");
  ok(&["issues", "show", "7"], project.path(), home.path());
  let (out, err, _) = run(&["doctor"], project.path(), home.path());
  assert!(out.contains("doctor: 0 finding(s)"), "{out}{err}");
}
