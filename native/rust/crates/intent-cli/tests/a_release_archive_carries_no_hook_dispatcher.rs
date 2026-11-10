//! **A RELEASE ARCHIVE CARRIES NO COMMIT-GATE DISPATCHER** (issue 0650).
//!
//! `intent claude upgrade --apply` writes the commit gate's dispatchers into
//! the hooks directory `core.hooksPath` names, and a project tracks them so a
//! clone receives them. Nothing marked them `export-ignore`, so `git archive`
//! (a release tarball, a source archive, a Homebrew formula's url) carried
//! them. These arms run the upgrade in a fresh repository whose hooks live at
//! `.githooks`, then read the archive and doctor.
//!
//! The archive is of the tree `git write-tree` makes from the index, not of a
//! commit, so no commit runs the gate the upgrade has just installed.

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

fn git(cwd: &Path, args: &[&str]) -> String {
  let out = Command::new("git")
    .args(args)
    .current_dir(cwd)
    .output()
    .expect("run git");
  assert!(
    out.status.success(),
    "git {args:?} failed: {}",
    String::from_utf8_lossy(&out.stderr)
  );
  String::from_utf8_lossy(&out.stdout).into_owned()
}

/// A project fresh from `init`, its hooks at `.githooks`, upgraded.
fn upgraded_project(project: &Path, home: &Path) {
  git(project, &["init", "-q"]);
  git(project, &["config", "core.hooksPath", ".githooks"]);
  let (_, err, code) = run(&["init", "Team"], project, home);
  assert_eq!(code, 0, "init failed: {err}");
  let (_, err, code) = run(&["bootstrap"], project, home);
  assert_eq!(code, 0, "bootstrap failed: {err}");
  let (_, err, code) = run(
    &["claude", "upgrade", "--apply", "--skip-settings"],
    project,
    home,
  );
  assert_eq!(code, 0, "claude upgrade --apply failed: {err}");
  assert!(
    project.join(".githooks/pre-commit").is_file(),
    "the upgrade wrote no dispatcher into .githooks, so this fixture cannot exhibit the defect"
  );
}

/// The paths `git archive` writes for the staged tree, one per entry.
fn archive_listing(project: &Path) -> Vec<String> {
  git(project, &["add", "-A"]);
  let tree = git(project, &["write-tree"]);
  let scratch = tempfile::tempdir().expect("tempdir");
  let tar = scratch.path().join("release.tar");
  git(
    project,
    &["archive", "-o", &tar.display().to_string(), tree.trim()],
  );
  let out = Command::new("tar")
    .arg("-tf")
    .arg(&tar)
    .output()
    .expect("run tar");
  assert!(out.status.success(), "tar -t failed on the archive");
  String::from_utf8_lossy(&out.stdout)
    .lines()
    .map(str::to_string)
    .collect()
}

#[test]
fn an_upgrade_keeps_the_tracked_dispatchers_out_of_the_release_archive() {
  let project = tempfile::tempdir().expect("tempdir");
  let home = tempfile::tempdir().expect("tempdir");
  upgraded_project(project.path(), home.path());

  let listing = archive_listing(project.path());
  assert!(
    listing.iter().any(|p| p.starts_with("intent/")),
    "the archive carries no project file, so it can show nothing about the dispatchers: {listing:?}"
  );
  let shipped: Vec<&String> = listing
    .iter()
    .filter(|p| p.starts_with(".githooks"))
    .collect();
  assert!(
    shipped.is_empty(),
    "the release archive carries the commit gate's dispatchers: {shipped:?}"
  );
}

/// The line a default `intent doctor` prints for a `.gitattributes` the
/// upgrade would change.
const GITATTRIBUTES_BEHIND: &str = "  .gitattributes -- differs from what the templates";

#[test]
fn doctor_names_an_undecided_gitattributes_and_an_upgrade_keeps_an_opt_out() {
  let project = tempfile::tempdir().expect("tempdir");
  let home = tempfile::tempdir().expect("tempdir");
  upgraded_project(project.path(), home.path());
  let gitattributes = project.path().join(".gitattributes");

  std::fs::write(&gitattributes, "").expect("empty .gitattributes");
  let (out, err, _) = run(&["doctor"], project.path(), home.path());
  assert!(
    out.contains(GITATTRIBUTES_BEHIND),
    "doctor did not name a .gitattributes that lets the dispatchers ride every release archive:\n{out}{err}"
  );

  let opt_out = "/.githooks -export-ignore\n";
  std::fs::write(&gitattributes, opt_out).expect("write the opt-out");
  let (out, err, _) = run(&["doctor"], project.path(), home.path());
  assert!(
    !out.contains(GITATTRIBUTES_BEHIND),
    "doctor named a project that decided to ship its dispatchers:\n{out}{err}"
  );

  let (_, err, code) = run(
    &["claude", "upgrade", "--apply", "--skip-settings"],
    project.path(),
    home.path(),
  );
  assert_eq!(code, 0, "claude upgrade --apply failed: {err}");
  assert_eq!(
    std::fs::read_to_string(&gitattributes).expect("read .gitattributes"),
    opt_out,
    "the upgrade rewrote a .gitattributes whose own line had decided"
  );
  assert!(
    archive_listing(project.path())
      .iter()
      .any(|p| p == ".githooks/pre-commit"),
    "the project chose to ship its dispatchers and the archive does not carry them"
  );
}
