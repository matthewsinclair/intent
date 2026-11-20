//! **ST0078 WP-02's repair through the real binary, as ST0095 WP-10 leaves it:
//! two clones hold one ordinal, and one verb moves it** (AC-10.5, AC-06.3).
//!
//! ST0078's worked example E4, driven: Alice and Bob clone one origin and each
//! holds the same ordinal, as two branches cut before the upgrade do. Bob runs
//! `st remint`, which moves his to the git-style id the one mint derives, after
//! which the merge is clean and `doctor`, the verdict the commit gate runs, is
//! clean. The verb takes the id it moves and never the id it moves to, so no
//! verb mints an ordinal; the two-argument `renumber` that named one is refused
//! with `remint` as its remedy.
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

/// The id a remint moved to, read off its `ok:` line, `ok: <old> reminted to
/// <new>`: the one mint chose it, so a test reads it rather than names it.
fn reminted_to(out: &str) -> String {
  out
    .lines()
    .find_map(|l| l.strip_prefix("ok: "))
    .and_then(|l| l.rsplit_once(" reminted to "))
    .map(|(_, to)| to.to_string())
    .unwrap_or_else(|| panic!("no `reminted to` line:\n{out}"))
}

// ST0095 AT-10.9: the arm below.
#[test]
fn a_thread_both_clones_hold_is_reminted_and_the_merge_is_clean_under_doctor() {
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

  // Both hold one ordinal, the collision under repair. **SEEDED, NOT MINTED**
  // (ST0095 WP-06): a mint makes a git-style id now, which never collides, so
  // this is the ordinal a branch cut before the upgrade carries. Bob starts
  // his, so it is realised and declared.
  let alices = intentsvcs::id::ThreadId::from_ordinal(1)
    .expect("an ordinal thread")
    .stored();
  crate::common::seed_ordinal_thread(&alice, &alices, "Alice's next", &[]);
  git_ok(&["add", "-A"], &alice);
  git_ok(
    &["commit", "-q", "-m", format!("alice: {alices}").as_str()],
    &alice,
  );
  git_ok(&["push", "-q", "origin", "HEAD:main"], &alice);
  let bobs = alices.clone();
  crate::common::seed_ordinal_thread(&bob, &bobs, "Bob's next", &[]);
  ok(&["st", "start", bobs.as_str()], &bob, home.path());
  git_ok(&["add", "-A"], &bob);
  git_ok(
    &["commit", "-q", "-m", format!("bob: {bobs}").as_str()],
    &bob,
  );

  // The two-argument form named the id to move to, so it could mint an
  // ordinal; it is refused, and its remedy is the one-argument verb.
  let (out, err, code) = run(
    &["st", "renumber", bobs.as_str(), "ST0002"],
    &bob,
    home.path(),
  );
  assert_eq!(code, 2, "`st renumber` is refused as retired:\n{out}{err}");
  assert!(
    err.contains("`intent st renumber` was retired") && err.contains("`intent st remint <id>`"),
    "the refusal names `st remint` as its remedy:\n{err}"
  );

  // The repair.
  let out = ok(&["st", "remint", bobs.as_str()], &bob, home.path());
  let moved = reminted_to(&out);
  let moved_id = intentsvcs::id::ThreadId::parse(&moved).expect("a thread id");
  assert!(
    moved_id.is_hashed(),
    "a remint moves to a git-style id:\n{out}"
  );
  assert!(
    out.contains(&format!("moved: intent/st/{bobs} -> intent/st/{moved}")),
    "the realised directory moves and says so:\n{out}"
  );
  assert!(!bob.join(format!("intent/.canon/st/{bobs}.json")).exists());
  assert!(bob.join(format!("intent/st/{moved}/info.md")).exists());
  git_ok(&["add", "-A"], &bob);
  git_ok(&["commit", "-q", "-m", "bob: remint"], &bob);

  // The merge now touches canon cleanly, and nothing conflicts: the two lists
  // both clones rewrote are each clone's own render and travel in no commit
  // (ST0096), so the pull merges and commits by itself.
  let (merge, merged) = git(&["pull", "-q", "--no-rebase", "origin", "main"], &bob);
  let conflicted = git_ok(&["diff", "--name-only", "--diff-filter=U"], &bob);
  assert!(
    merged && conflicted.trim().is_empty(),
    "after a remint the merge is clean, and {conflicted} conflicted:\n{merge}"
  );
  ok(&["sync", "--apply"], &bob, home.path());
  ok(&["sync", "--to-disk"], &bob, home.path());
  ok(&["todo", "update"], &bob, home.path());
  // Whatever the three left to commit; the lists they rewrote are ignored, so
  // this may be nothing at all.
  git_ok(&["add", "-A"], &bob);
  git_ok(&["commit", "-q", "--allow-empty", "-m", "merge main"], &bob);

  let list = ok(&["st", "list", "--status", "all"], &bob, home.path());
  assert!(
    list.contains(&alices) && list.contains("Alice's next"),
    "{list}"
  );
  assert!(
    list.contains(&moved_id.shown()) && list.contains("Bob's next"),
    "{list}"
  );
  let (out, err, code) = run(&["doctor"], &bob, home.path());
  assert!(
    out.contains("doctor: 0 finding(s)") && code == 0,
    "the merged clone is not clean under doctor:\n{out}{err}"
  );

  // A git-style id is never moved again: nothing is left to remint.
  let (out, err, code) = run(&["st", "remint", moved.as_str()], &bob, home.path());
  assert_eq!(code, 1, "{out}{err}");
  assert!(
    err.contains("a git-style id stays as it is"),
    "a git-style id is refused as having nothing to remint:\n{err}"
  );
}

// ST0095 AT-10.10: the arm below.
#[test]
fn an_issue_remint_reports_what_it_moved_and_the_prose_it_left() {
  let project = tempfile::tempdir().expect("tempdir");
  let home = tempfile::tempdir().expect("tempdir");
  ok(&["init", "Team"], project.path(), home.path());
  crate::common::seed_ordinal_issue(project.path(), 1, "First");
  let first = intentsvcs::id::IssueId::from_ordinal(1)
    .expect("an ordinal issue")
    .stored();
  let body = project.path().join("body.md");
  std::fs::write(&body, format!("Follows on from {first}.\n")).expect("write body");
  let second = crate::common::created(&ok(
    &[
      "issues",
      "add",
      "Second",
      "--from",
      body.to_str().expect("utf-8"),
    ],
    project.path(),
    home.path(),
  ));

  let (out, err, code) = run(
    &["issues", "renumber", first.as_str(), "7"],
    project.path(),
    home.path(),
  );
  assert_eq!(
    code, 2,
    "`issues renumber` is refused as retired:\n{out}{err}"
  );
  assert!(
    err.contains("`intent issues remint <id>`"),
    "the refusal names `issues remint` as its remedy:\n{err}"
  );

  let out = ok(
    &["issues", "remint", first.as_str()],
    project.path(),
    home.path(),
  );
  let moved = reminted_to(&out);
  assert!(
    intentsvcs::id::IssueId::parse(&moved)
      .expect("an issue id")
      .is_hashed(),
    "a remint moves to a git-style id:\n{out}"
  );
  assert!(
    out.contains(&format!("moved: intent/.canon/issues/{first}.json removed")),
    "{out}"
  );
  assert!(
    out
      .lines()
      .any(|l| l.starts_with("prose: ") && l.contains(second.as_str())),
    "issue {second}'s body names {first} and is reported, not rewritten:\n{out}"
  );
  let shown = ok(
    &["issues", "show", second.as_str()],
    project.path(),
    home.path(),
  );
  assert!(
    shown.contains(&format!("Follows on from {first}.")),
    "{shown}"
  );
  ok(
    &["issues", "show", moved.as_str()],
    project.path(),
    home.path(),
  );
  let (out, err, _) = run(&["doctor"], project.path(), home.path());
  assert!(out.contains("doctor: 0 finding(s)"), "{out}{err}");
}
