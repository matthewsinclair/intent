//! **Issue 0624: a verb that writes a tracked file names it on a `wrote:`
//! line.** The board verbs have since 0612. Every other verb wrote its event,
//! its canon record and its views and printed none of them, so a node found
//! what to commit in `git status` and read each path first. One arm per verb
//! family, over the verbs of dc's drive (cd40e9b7a): each runs in a scratch
//! git project committed before it, and the paths it prints under `wrote:` are
//! the paths `git status` shows changed, no more and no fewer. Each arm is red
//! on a binary without the fix, which prints none.

use std::collections::BTreeSet;
use std::path::Path;

/// stdout, stderr and the exit code of one `intent` run in `cwd`.
fn run(cwd: &Path, args: &[&str]) -> (String, String, i32) {
  let out = crate::common::intent()
    .args(args)
    .current_dir(cwd)
    .env("HOME", testkit::fixture_home())
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run intent");
  (
    String::from_utf8_lossy(&out.stdout).into_owned(),
    String::from_utf8_lossy(&out.stderr).into_owned(),
    out.status.code().unwrap_or(-1),
  )
}

/// A project in its own git repository, committed, so the next verb's
/// footprint is what `git status` shows.
fn project() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  crate::common::git_init(dir.path());
  setup(dir.path(), &["init", "probe"]);
  dir
}

/// A step an arm stands on rather than one it judges: run it, then commit
/// what it wrote.
fn setup(dir: &Path, args: &[&str]) {
  let (out, err, code) = run(dir, args);
  assert_eq!(code, 0, "{args:?}: {out}{err}");
  crate::common::git_commit_all(dir);
}

/// The paths a run printed under `wrote:`, from either stream: `edit` prints
/// its own on stderr, because its stdout is the one path a script reads.
fn wrote(out: &str, err: &str) -> BTreeSet<String> {
  out
    .lines()
    .chain(err.lines())
    .filter_map(|line| line.strip_prefix("wrote: "))
    .map(str::to_string)
    .collect()
}

/// Run one verb and hold it to its footprint: what it printed under `wrote:`
/// is exactly what `git status` shows it changed. Then commit, so the next
/// verb's footprint stands alone. Hands back stdout for an arm with more to
/// say about it.
fn names_what_it_wrote(dir: &Path, args: &[&str]) -> String {
  let (out, err, code) = run(dir, args);
  assert_eq!(code, 0, "{args:?}: {out}{err}");
  let changed = crate::common::git_changed(dir);
  assert!(
    !changed.is_empty(),
    "{args:?} changed nothing, so it proves nothing: {out}{err}"
  );
  assert_eq!(wrote(&out, &err), changed, "{args:?} printed:\n{out}{err}");
  crate::common::git_commit_all(dir);
  out
}

#[test]
fn the_issue_verbs_name_every_file_they_wrote() {
  let dir = project();
  let d = dir.path();
  names_what_it_wrote(d, &["issues", "add", "a probe issue"]);
  names_what_it_wrote(d, &["issues", "edit", "0001", "--body", "an edited body"]);
  names_what_it_wrote(d, &["issues", "close", "0001"]);
}

#[test]
fn the_thread_verbs_name_every_file_they_wrote() {
  let dir = project();
  let d = dir.path();
  names_what_it_wrote(d, &["st", "new", "a probe thread"]);
  // Realising the thread writes its files; with stdout not a terminal, `st
  // edit` prints the path there alone and its `wrote:` lines go to stderr.
  let path = names_what_it_wrote(d, &["st", "edit", "ST0001"]);
  assert_eq!(path.lines().count(), 1, "stdout is the one path: {path}");
  assert!(
    path.trim_end().ends_with("intent/st/ST0001/info.md"),
    "{path}"
  );
  names_what_it_wrote(d, &["st", "start", "ST0001"]);
  setup(
    d,
    &["ac", "new", "--text", "a criterion", "ST0001", "AC-01.1"],
  );
  setup(
    d,
    &[
      "ac",
      "satisfy",
      "--evidence",
      "the 0624 arm",
      "ST0001",
      "AC-01.1",
    ],
  );
  names_what_it_wrote(d, &["st", "done", "ST0001"]);
}

#[test]
fn set_names_every_file_it_wrote() {
  let dir = project();
  let d = dir.path();
  setup(d, &["st", "new", "a probe thread"]);
  // The objective is read from outside the project, so the file it comes from
  // is no change of the verb's.
  let elsewhere = tempfile::tempdir().expect("tempdir");
  let objective = elsewhere.path().join("objective.txt");
  std::fs::write(&objective, "the objective, from a file\n").expect("write the objective");
  let from = objective.to_str().expect("a UTF-8 path");
  names_what_it_wrote(
    d,
    &[
      "set",
      "intent:///threads/ST0001",
      "objective",
      "--from",
      from,
    ],
  );
}

#[test]
fn the_work_package_verbs_name_every_file_they_wrote() {
  let dir = project();
  let d = dir.path();
  setup(d, &["st", "new", "a probe thread"]);
  setup(
    d,
    &["ac", "new", "--text", "a criterion", "ST0001", "AC-01.1"],
  );
  setup(
    d,
    &[
      "ac",
      "satisfy",
      "--evidence",
      "the 0624 arm",
      "ST0001",
      "AC-01.1",
    ],
  );
  names_what_it_wrote(d, &["wp", "new", "ST0001", "a probe package"]);
  names_what_it_wrote(d, &["wp", "start", "ST0001/01"]);
  names_what_it_wrote(d, &["wp", "done", "ST0001/01"]);
}

#[test]
fn the_criterion_verbs_name_every_file_they_wrote() {
  let dir = project();
  let d = dir.path();
  setup(d, &["st", "new", "a probe thread"]);
  names_what_it_wrote(
    d,
    &["ac", "new", "--text", "a criterion", "ST0001", "AC-01.1"],
  );
  names_what_it_wrote(
    d,
    &[
      "ac",
      "edit",
      "--text",
      "a reworded criterion",
      "ST0001",
      "AC-01.1",
    ],
  );
  names_what_it_wrote(
    d,
    &[
      "ac",
      "satisfy",
      "--evidence",
      "the 0624 arm",
      "ST0001",
      "AC-01.1",
    ],
  );
  names_what_it_wrote(
    d,
    &[
      "ac",
      "new",
      "--kind",
      "test",
      "--text",
      "a tested criterion",
      "ST0001",
      "AC-01.2",
    ],
  );
}

#[test]
fn the_acceptance_test_verbs_name_every_file_they_wrote() {
  let dir = project();
  let d = dir.path();
  setup(d, &["st", "new", "a probe thread"]);
  setup(
    d,
    &[
      "ac",
      "new",
      "--kind",
      "test",
      "--text",
      "a tested criterion",
      "ST0001",
      "AC-01.2",
    ],
  );
  std::fs::create_dir_all(d.join("tests")).expect("the tests directory");
  std::fs::write(
    d.join("tests/probe_test.rs"),
    "// AT-01.2: the file the probe cites\n",
  )
  .expect("write the cited file");
  crate::common::git_commit_all(d);
  names_what_it_wrote(
    d,
    &[
      "at",
      "new",
      "--covers",
      "AC-01.2",
      "--file",
      "tests/probe_test.rs",
      "ST0001",
      "AT-01.2",
    ],
  );
  names_what_it_wrote(d, &["at", "red", "ST0001", "AT-01.2"]);
  names_what_it_wrote(d, &["at", "green", "ST0001", "AT-01.2"]);
}

/// **A VERB THAT MOVED NOTHING LANDED NOTHING, SO IT NAMES NOTHING.** Starting
/// a started thread answers `already`, writes no file, and prints no `wrote:`
/// line.
#[test]
fn a_verb_that_moves_nothing_names_nothing() {
  let dir = project();
  let d = dir.path();
  setup(d, &["st", "new", "a probe thread"]);
  setup(d, &["st", "start", "ST0001"]);
  let (out, err, code) = run(d, &["st", "start", "ST0001"]);
  assert_eq!(code, 0, "{out}{err}");
  assert!(out.contains("ok: ST0001 already"), "{out}");
  assert!(
    wrote(&out, &err).is_empty(),
    "a no-op named files:\n{out}{err}"
  );
  assert!(
    crate::common::git_changed(d).is_empty(),
    "a no-op changed files"
  );
}
