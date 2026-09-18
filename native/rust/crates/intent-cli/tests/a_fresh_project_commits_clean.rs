//! Issue 0448 -- **a project `init` creates is clean under `doctor` straight
//! after `claude upgrade --apply`, with no sync in between.**
//!
//! `init` wrote neither `intent/st/steel_threads.md` nor `intent/todo.md`, and
//! `doctor` counts an absent aggregate view as skew even on an estate with no
//! thread. The pre-commit gate `claude upgrade --apply` installs runs `doctor`,
//! so the FIRST commit of every new project was refused by the gate the second
//! command had just wired, with `intent sync --to-disk` as a remedy the new user
//! had to work out. Driven by vc on 2026-09-18 for ST0078's worked examples.
//!
//! **THE ARM IS THE SEQUENCE A NEW USER RUNS, AND `doctor` IS ITS VERDICT**,
//! because `doctor` is what the gate runs: `init`, `bootstrap`, `claude upgrade
//! --apply`, then `doctor`. No `sync` anywhere, since needing one is the defect.
//!
//! **UNDER ITS OWN `HOME`.** `bootstrap` writes the install pointer under the
//! user's data directory and `claude upgrade` reads per-user state, so both run
//! against a tempdir and never against the developer's machine.

use std::path::Path;
use std::process::Command;

fn run(args: &[&str], cwd: &Path, home: &Path) -> (String, String, i32) {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
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

#[test]
fn a_project_init_creates_is_clean_under_doctor_after_claude_upgrade_with_no_sync() {
  let project = tempfile::tempdir().expect("tempdir");
  let home = tempfile::tempdir().expect("tempdir");
  let git = Command::new("git")
    .args(["init", "-q"])
    .current_dir(project.path())
    .status()
    .expect("run git init");
  assert!(git.success(), "git init failed in the fixture");

  let (_, err, code) = run(&["init", "Team"], project.path(), home.path());
  assert_eq!(code, 0, "init failed: {err}");
  let (_, err, code) = run(&["bootstrap"], project.path(), home.path());
  assert_eq!(code, 0, "bootstrap failed: {err}");
  let (_, err, code) = run(
    &["claude", "upgrade", "--apply", "--skip-settings"],
    project.path(),
    home.path(),
  );
  assert_eq!(code, 0, "claude upgrade --apply failed: {err}");

  let (out, err, code) = run(&["doctor"], project.path(), home.path());
  assert!(
    out.contains("doctor: 0 finding(s)"),
    "a project fresh from init is not clean under doctor, so the gate claude upgrade \
     installed refuses its first commit (issue 0448).\nstdout:\n{out}\nstderr:\n{err}"
  );
  assert_eq!(
    code, 0,
    "doctor exited {code} on a fresh project: {out}{err}"
  );
}
