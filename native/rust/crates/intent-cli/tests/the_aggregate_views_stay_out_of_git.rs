//! ST0096 WP-02 (AC-02.1, AC-02.2, AC-02.3) and issue 0707's doctor half: **the
//! aggregate views stay on disk and out of git.** `intent/st/steel_threads.md`,
//! `intent/todo.md` and every thread's `acceptance.md` are rendered from many
//! records, so a change to one record rewrites them, and two merge requests
//! that change different records both write them (hv decisions 66 and 74).
//! Every clone renders its own: `init` ignores them, `intent upgrade` takes an
//! older project's copies out of git's index, and doctor names any still
//! tracked, shown and not counted. One a clone has not rendered, or that is
//! behind its store, is the clone's own business: shown with the verb that
//! renders it, and never counted (cc's meaning, 2026-11-20).

use crate::common::{created, git_commit_all, git_init, ok, run};
use std::path::Path;

/// `git` in `root` for what `common`'s helpers do not do, listing the index and
/// adding past the ignore rules, asserted to succeed, with no user config,
/// signing or hooks of the machine running the suite: its stdout.
fn git(root: &Path, args: &[&str]) -> String {
  let out = std::process::Command::new("git")
    .args(["-c", "user.name=t", "-c", "user.email=t@example.com"])
    .args([
      "-c",
      "commit.gpgSign=false",
      "-c",
      "core.hooksPath=/dev/null",
    ])
    .args(args)
    .current_dir(root)
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run git");
  assert!(
    out.status.success(),
    "git {args:?} failed: {}",
    String::from_utf8_lossy(&out.stderr)
  );
  String::from_utf8_lossy(&out.stdout).into_owned()
}

/// A project in its own git repository with one realised thread, so all three
/// kinds of aggregate view are on disk: the thread, and the views' paths.
fn project_with_a_realised_thread(root: &Path) -> (String, [String; 3]) {
  git_init(root);
  ok(root, &["init", "views"]);
  let st = created(&ok(root, &["st", "new", "A thread the views index"]));
  ok(root, &["st", "start", st.as_str()]);
  let views = [
    "intent/st/steel_threads.md".to_string(),
    "intent/todo.md".to_string(),
    format!("intent/st/{st}/acceptance.md"),
  ];
  (st, views)
}

/// Which of `paths` git's index holds.
fn tracked(root: &Path, paths: &[String]) -> String {
  let mut args = vec!["ls-files", "--"];
  args.extend(paths.iter().map(String::as_str));
  git(root, &args)
}

/// The views committed as an older Intent committed them, by force past the
/// ignore rules: a project not yet upgraded.
fn track_by_force(root: &Path, views: &[String]) {
  let mut add = vec!["add", "-f", "--"];
  add.extend(views.iter().map(String::as_str));
  git(root, &add);
  git(
    root,
    &[
      "commit",
      "-q",
      "-m",
      "the views, as an older Intent committed them",
    ],
  );
}

/// `intent doctor`'s whole output and its exit code.
fn doctor(root: &Path) -> (String, i32) {
  let (out, err, code) = run(root, &["doctor"]);
  (format!("{out}{err}"), code)
}

/// `intent <args>` in `root` with no `git` on PATH, so the repository is there
/// and git cannot be asked: its whole output and its exit code.
fn run_without_git(root: &Path, args: &[&str]) -> (String, Option<i32>) {
  let no_git = tempfile::tempdir().expect("a PATH with no git on it");
  let out = crate::common::intent()
    .args(args)
    .current_dir(root)
    .env("HOME", testkit::fixture_home())
    .env("PATH", no_git.path())
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run the v3 binary");
  (
    format!(
      "{}{}",
      String::from_utf8_lossy(&out.stdout),
      String::from_utf8_lossy(&out.stderr)
    ),
    out.status.code(),
  )
}

#[test]
fn a_new_project_renders_its_aggregate_views_and_commits_none() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  let (st, views) = project_with_a_realised_thread(root);
  git_commit_all(root);

  for view in &views {
    assert!(root.join(view).is_file(), "{view} is on disk");
  }
  assert_eq!(tracked(root, &views), "", "and git tracks none of them");

  // A verb keeps them current, and names none of them as the caller's.
  let before = std::fs::read_to_string(root.join(&views[1])).expect("todo.md");
  let out = ok(
    root,
    &["wp", "new", st.as_str(), "A package the views list"],
  );
  let after = std::fs::read_to_string(root.join(&views[1])).expect("todo.md");
  assert!(
    after != before && after.contains("A package the views list"),
    "wp new re-renders todo.md:\n{after}"
  );
  // The positive control: the verb does name the view it re-rendered, so the
  // absence of a `wrote:` line below is a choice and not a silent run.
  assert!(
    out
      .lines()
      .any(|line| line == format!("re-rendered: {}", views[1])),
    "wp new names todo.md re-rendered, nobody's to commit:\n{out}"
  );
  for view in &views {
    assert!(
      !out.lines().any(|line| line == format!("wrote: {view}")),
      "{view} is nobody's to commit, so no wrote: line names it:\n{out}"
    );
  }
  assert_eq!(
    tracked(root, &views),
    "",
    "and git still tracks none of them"
  );
}

#[test]
fn an_upgrade_untracks_the_aggregate_views_that_doctor_names() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  let (_, views) = project_with_a_realised_thread(root);
  git_commit_all(root);
  let (said, clean) = doctor(root);
  assert_eq!(clean, 0, "the project starts clean under doctor:\n{said}");

  track_by_force(root, &views);
  let (said, code) = doctor(root);
  assert_eq!(
    code, clean,
    "a tracked aggregate view is not counted:\n{said}"
  );
  assert!(
    said.contains("aggregate-view-tracked")
      && views.iter().all(|view| said.contains(view.as_str())),
    "doctor names every tracked aggregate view:\n{said}"
  );

  let (_, err, code) = run(root, &["upgrade"]);
  assert_eq!(code, 0, "upgrade: {err}");
  for view in &views {
    assert!(
      err.contains(&format!("untracked: {view} -- ")),
      "the upgrade names {view} as untracked:\n{err}"
    );
    assert!(root.join(view).is_file(), "and leaves {view} on disk");
  }
  assert_eq!(
    tracked(root, &views),
    "",
    "the upgrade took them out of the index"
  );

  // A re-run before anyone commits names each again: its deletion is still
  // staged, and the index has not moved on.
  let (_, err, code) = run(root, &["upgrade"]);
  assert_eq!(code, 0, "a second upgrade: {err}");
  for view in &views {
    assert!(
      err.contains(&format!("untracked: {view} -- ")),
      "a re-run before the commit names {view} again:\n{err}"
    );
  }

  // Committed as the line says, with a plain `git commit`.
  git(
    root,
    &["commit", "-q", "-m", "the aggregate views leave git"],
  );
  let (_, err, code) = run(root, &["upgrade"]);
  assert_eq!(code, 0, "an upgrade after the commit: {err}");
  assert!(
    !err.contains("untracked: "),
    "once the deletion is committed an upgrade names none:\n{err}"
  );
  let (said, _) = doctor(root);
  assert!(
    !said.contains("aggregate-view-tracked"),
    "doctor names none once they are untracked:\n{said}"
  );
}

/// The critic's first finding on wip1: **A GIT THAT CANNOT BE ASKED HALTS THE
/// UPGRADE**, and is never read as a tree with nothing tracked, which would
/// mark the project upgraded with its aggregate views still in git's index.
#[test]
fn an_upgrade_that_cannot_ask_git_halts_and_untracks_nothing() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  let (_, views) = project_with_a_realised_thread(root);
  git_commit_all(root);
  track_by_force(root, &views);

  let (said, code) = run_without_git(root, &["upgrade"]);
  assert_eq!(code, Some(1), "the upgrade halts:\n{said}");
  assert!(
    said.contains("taking the aggregate views out of git's index")
      && said.contains("`intent upgrade` is the command to run again"),
    "it says where it stopped and what to run once git answers:\n{said}"
  );
  assert_eq!(
    tracked(root, &views).lines().count(),
    views.len(),
    "and git's index still holds every view:\n{said}"
  );
}

/// Issue 0730: **ONE ANSWER FOR A GIT THAT CANNOT BE ASKED.** doctor asks git
/// whether it tracks the store and whether it tracks the aggregate views, and
/// with the repository there and no `git` to ask, both say so: the store is
/// never read as untracked where the views are read as unasked.
#[test]
fn doctor_says_git_was_not_asked_of_the_store_and_the_views_alike() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  project_with_a_realised_thread(root);
  git_commit_all(root);

  let (said, _) = run_without_git(root, &["doctor", "--verbose"]);
  for about in ["the store", "the aggregate views"] {
    assert!(
      said.contains(&format!("git was not asked whether it tracks {about}")),
      "doctor says git was not asked whether it tracks {about}:\n{said}"
    );
  }
}

/// Issue 0707's doctor half: a closed issue's view, and a cancelled thread's,
/// stay on disk, so every clone's sync removes them until one commit on main
/// does, and doctor says so. The thread's `acceptance.md` is an aggregate view
/// git ignores, and the plan still sees it, so it goes too.
#[test]
fn a_closed_records_views_are_named_until_they_are_removed() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  git_init(root);
  ok(root, &["init", "views"]);
  let issue = created(&ok(root, &["issues", "add", "An issue that closes"]));
  ok(root, &["issues", "close", issue.as_str()]);
  let st = created(&ok(root, &["st", "new", "A thread that is cancelled"]));
  ok(root, &["st", "start", st.as_str()]);
  ok(
    root,
    &["st", "cancel", st.as_str(), "--reason", "nobody needs it"],
  );
  let views = [
    format!("intent/issues/{issue}.md"),
    format!("intent/st/{st}/acceptance.md"),
  ];
  for view in &views {
    assert!(root.join(view).is_file(), "the close leaves {view} on disk");
  }

  let (said, _) = doctor(root);
  assert!(
    said.contains("unrealised-view")
      && said.contains(&format!("{issue}.md"))
      && said.contains(&format!("intent/st/{st}")),
    "doctor names the closed records' views:\n{said}"
  );

  ok(root, &["organize", "--apply"]);
  for view in &views {
    assert!(!root.join(view).exists(), "organize --apply removes {view}");
  }
  let (said, _) = doctor(root);
  assert!(
    !said.contains("unrealised-view"),
    "and doctor names nothing once they are gone:\n{said}"
  );
}

/// AC-02.3: **AN IGNORED AGGREGATE VIEW IS THE CLONE'S OWN RENDER.** A fresh
/// clone has none, and a pull that changes records leaves them behind the
/// store; doctor shows both with the verb that renders them, and counts
/// neither, so the first commit on a fresh clone passes the commit gate.
#[test]
fn a_clones_own_render_is_shown_and_never_counted() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  let (_, views) = project_with_a_realised_thread(root);
  git_commit_all(root);
  let (said, clean) = doctor(root);
  assert_eq!(clean, 0, "the project starts clean under doctor:\n{said}");

  // A fresh clone's state: git carried none of them.
  for view in &views {
    std::fs::remove_file(root.join(view)).expect("remove the view");
  }
  let (said, code) = doctor(root);
  assert_eq!(
    code, clean,
    "a missing aggregate view is not counted:\n{said}"
  );
  for view in &views {
    assert!(
      said.contains("aggregate-view-stale") && said.contains(view.as_str()),
      "doctor shows {view} as missing:\n{said}"
    );
  }

  // The plan sees them whatever git ignores, so a sync renders them.
  ok(root, &["sync", "--apply"]);
  for view in &views {
    assert!(root.join(view).is_file(), "sync --apply renders {view}");
  }
  let (said, _) = doctor(root);
  assert!(
    !said.contains("aggregate-view-stale"),
    "and doctor shows none once they are rendered:\n{said}"
  );

  // Behind the store, as a pull that changed records leaves it.
  let todo = root.join(&views[1]);
  let mut text = std::fs::read_to_string(&todo).expect("todo.md");
  text.push_str("\na line the store does not render\n");
  std::fs::write(&todo, text).expect("put todo.md behind the store");
  let (said, code) = doctor(root);
  assert_eq!(
    code, clean,
    "a stale aggregate view is not counted:\n{said}"
  );
  assert!(
    said.contains("aggregate-view-stale") && said.contains(views[1].as_str()),
    "doctor shows the stale view:\n{said}"
  );
  ok(root, &["sync", "--to-disk"]);
  let (said, _) = doctor(root);
  assert!(
    !said.contains("aggregate-view-stale"),
    "sync --to-disk re-renders it, as the finding says:\n{said}"
  );
}

/// AC-02.3's other half: **A VIEW STILL TRACKED IS COUNTED AS BEFORE**, in a
/// project not yet upgraded, because a commit does carry it.
#[test]
fn a_tracked_aggregate_view_that_differs_is_counted_as_before() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  let (_, views) = project_with_a_realised_thread(root);
  git_commit_all(root);
  track_by_force(root, &views);

  let todo = root.join(&views[1]);
  let mut text = std::fs::read_to_string(&todo).expect("todo.md");
  text.push_str("\na line the store does not render\n");
  std::fs::write(&todo, text).expect("edit the tracked todo.md");
  let (said, code) = doctor(root);
  assert_ne!(code, 0, "a tracked view that differs is counted:\n{said}");
  assert!(
    said.contains("view-skew") && said.contains(views[1].as_str()),
    "as view skew, naming it:\n{said}"
  );
  assert!(
    !said.contains("aggregate-view-stale"),
    "and never as the clone's own render:\n{said}"
  );
}
