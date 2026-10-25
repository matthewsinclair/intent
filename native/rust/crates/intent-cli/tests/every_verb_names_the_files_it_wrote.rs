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

// **Issue 0632: the writers 0624 left reporting in shapes of their own.** Each
// names the files it wrote on `wrote:` lines and the files it took off disk on
// `removed:` lines, through the one printer, so a node's commit is read off
// what the verb printed: the `wrote:` set is exactly the changed paths still
// on disk, and the `removed:` set exactly the ones gone. Over the verbs of dc's
// drive (refs/bank/dc/0632/drive.sh). Each arm is red on a binary without the
// fix, which prints neither.

/// The paths a run printed under `removed:`.
fn removed(out: &str, err: &str) -> BTreeSet<String> {
  out
    .lines()
    .chain(err.lines())
    .filter_map(|line| line.strip_prefix("removed: "))
    .map(str::to_string)
    .collect()
}

/// Run one verb and hold it to its whole footprint, writes and removals: what
/// it printed under `wrote:` is exactly what `git status` shows changed and
/// still on disk, and what it printed under `removed:` exactly what shows
/// changed and gone. Then commit, so the next verb's footprint stands alone.
fn names_what_it_wrote_and_removed(dir: &Path, args: &[&str]) -> String {
  let (out, err, code) = run(dir, args);
  assert_eq!(code, 0, "{args:?}: {out}{err}");
  let changed = crate::common::git_changed(dir);
  assert!(
    !changed.is_empty(),
    "{args:?} changed nothing, so it proves nothing: {out}{err}"
  );
  let (present, gone): (BTreeSet<String>, BTreeSet<String>) = changed
    .into_iter()
    .partition(|path| dir.join(path).exists());
  assert_eq!(wrote(&out, &err), present, "{args:?} printed:\n{out}{err}");
  assert_eq!(removed(&out, &err), gone, "{args:?} printed:\n{out}{err}");
  crate::common::git_commit_all(dir);
  out
}

#[test]
fn init_names_every_file_it_wrote() {
  let dir = tempfile::tempdir().expect("tempdir");
  crate::common::git_init(dir.path());
  names_what_it_wrote_and_removed(dir.path(), &["init", "probe"]);
}

#[test]
fn the_language_verbs_name_the_config_they_wrote() {
  let dir = project();
  let d = dir.path();
  names_what_it_wrote_and_removed(d, &["lang", "init", "rust"]);
  names_what_it_wrote_and_removed(d, &["lang", "remove", "rust"]);
  // THE CONTROL: a removal that changes nothing names nothing.
  let (out, err, code) = run(d, &["lang", "remove", "rust"]);
  assert_eq!(code, 0, "{out}{err}");
  assert!(
    wrote(&out, &err).is_empty() && removed(&out, &err).is_empty(),
    "a no-op named files:\n{out}{err}"
  );
}

#[test]
fn hydrate_and_dehydrate_name_what_they_wrote_and_removed() {
  let dir = project();
  let d = dir.path();
  setup(d, &["st", "new", "a probe thread"]);
  names_what_it_wrote_and_removed(d, &["st", "hydrate", "ST0001"]);
  let out = names_what_it_wrote_and_removed(d, &["st", "dehydrate", "ST0001"]);
  assert!(
    !removed(&out, "").is_empty(),
    "dehydrating a realised thread removed nothing: {out}"
  );
}

#[test]
fn the_renumbers_name_what_they_moved_as_written_and_removed() {
  let dir = project();
  let d = dir.path();
  setup(d, &["st", "new", "a probe thread"]);
  setup(d, &["st", "hydrate", "ST0001"]);
  names_what_it_wrote_and_removed(d, &["st", "renumber", "ST0001", "ST0002"]);
  setup(d, &["issues", "add", "a probe issue"]);
  names_what_it_wrote_and_removed(d, &["issues", "renumber", "0001", "0002"]);
}

#[test]
fn organize_names_what_it_hydrated_and_removed() {
  let dir = project();
  let d = dir.path();
  setup(d, &["st", "new", "a probe thread"]);
  // Declared in the manifest and not on disk: organize realises it.
  let manifest = d.join("intent/.intentfiles");
  let text = std::fs::read_to_string(&manifest).expect("the manifest");
  std::fs::write(&manifest, format!("{text}STEELTHREAD:ST0001\n")).expect("declare it");
  crate::common::git_commit_all(d);
  names_what_it_wrote_and_removed(d, &["organize", "--apply"]);
  // Then undeclared while on disk: organize removes its views.
  std::fs::write(&manifest, &text).expect("undeclare it");
  crate::common::git_commit_all(d);
  let out = names_what_it_wrote_and_removed(d, &["organize", "--apply"]);
  assert!(
    !removed(&out, "").is_empty(),
    "organizing an undeclared realised thread removed nothing: {out}"
  );
}

#[test]
fn sync_names_what_it_wrote_in_each_direction() {
  let dir = project();
  let d = dir.path();
  setup(d, &["st", "new", "a probe thread"]);
  setup(d, &["st", "hydrate", "ST0001"]);
  std::fs::remove_file(d.join("intent/st/ST0001/info.md")).expect("remove a view");
  crate::common::git_commit_all(d);
  names_what_it_wrote_and_removed(d, &["sync", "--to-disk"]);
  let cover = d.join("intent/st/ST0001/info.md");
  let text = std::fs::read_to_string(&cover).expect("the cover");
  let edited = text.replacen(
    "## Objective\n\n",
    "## Objective\n\nan objective written by hand\n\n",
    1,
  );
  assert_ne!(edited, text, "the cover has no Objective heading to edit");
  std::fs::write(&cover, edited).expect("hand-edit the objective");
  crate::common::git_commit_all(d);
  names_what_it_wrote_and_removed(d, &["sync", "--to-store", "ST0001"]);
}

#[test]
fn agents_sync_and_claude_upgrade_name_every_file_they_wrote() {
  let dir = project();
  let d = dir.path();
  std::fs::remove_file(d.join("AGENTS.md")).expect("remove AGENTS.md");
  crate::common::git_commit_all(d);
  names_what_it_wrote_and_removed(d, &["agents", "sync"]);
  names_what_it_wrote_and_removed(d, &["claude", "upgrade", "--apply"]);
}

// **Issue 0642: `st attach` printed only its `ok:` line**, so its event and the
// canon row it writes were found in `git status`, the rule 0632 closed for
// every other writer; `st detach`, its inverse, printed the same way. Red on a
// binary without the fix, which prints no `wrote:` line for either.
#[test]
fn attach_and_detach_name_every_file_they_wrote() {
  let dir = project();
  let d = dir.path();
  setup(d, &["st", "new", "a probe thread"]);
  // The source sits outside the project, so the only paths that move are the
  // verb's own.
  let source = tempfile::tempdir().expect("tempdir");
  let from = source.path().join("design.md");
  std::fs::write(&from, "# Design\n\nThe probe's design.\n").expect("write the source");
  let from = from.to_str().expect("a UTF-8 path");
  names_what_it_wrote(d, &["st", "attach", "ST0001", "design.md", "--from", from]);
  names_what_it_wrote_and_removed(d, &["st", "detach", "ST0001", "design.md"]);
}
