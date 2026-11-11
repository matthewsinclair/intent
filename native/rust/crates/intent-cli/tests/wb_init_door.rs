//! ST0092 WP-03: `intent wb init` and `intent wb remit` at the process surface.
//!
//! The library arms in `intentsvcs/tests/wb_init.rs` hold what each verb does;
//! these hold what a caller reads: a dry run's exit code on a refusal, the
//! apply's `wrote:` and `note:` lines, and `wb remit`'s two refusals.

use std::path::Path;

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

fn project(name: &str) -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let (out, err, code) = run(dir.path(), &["init", name]);
  assert_eq!(code, 0, "init: {out}{err}");
  dir
}

/// ST0092 AT-03.1 at the surface: the apply prints every file on a `wrote:`
/// line and the roster-commit `note:`, and the dry run before it writes none.
#[test]
fn at_03_1_the_apply_names_its_files_and_the_roster_commit() {
  let dir = project("wbinitdoor");
  let (out, err, code) = run(dir.path(), &["wb", "init", "team"]);
  assert_eq!(code, 0, "{out}{err}");
  assert!(
    out.contains("would write: intent/whiteboard/declaration.json"),
    "{out}"
  );
  assert!(!out.contains("\nwrote: "), "a dry run wrote: {out}");
  assert!(
    !dir
      .path()
      .join("intent/whiteboard/declaration.json")
      .exists()
  );

  let (out, err, code) = run(dir.path(), &["wb", "init", "team", "--apply"]);
  assert_eq!(code, 0, "{out}{err}");
  assert!(
    out.contains("wrote: intent/whiteboard/declaration.json"),
    "{out}"
  );
  assert!(
    out
      .lines()
      .any(|l| l.starts_with("note: ") && l.contains("mechanical commit")),
    "{out}"
  );
}

/// ST0092 AT-03.5 at the surface: a dry run that meets a refusal prints it and
/// exits 1, as the apply would (INV-04).
#[test]
fn at_03_5_a_dry_run_that_meets_a_refusal_exits_1() {
  let dir = project("wbinitrefuse");
  let (out, err, code) = run(dir.path(), &["wb", "init", "supervised"]);
  assert_eq!(code, 1, "{out}{err}");
  assert!(err.contains("error:") && err.contains("remedy:"), "{err}");
  let (out, err, code) = run(dir.path(), &["wb", "init", "no-such-profile"]);
  assert_eq!(code, 1, "{out}{err}");
}

/// The exit the row declares that no shipped install reaches: an install with
/// no profiles to read is this build unable to answer, exit 2, and not a
/// refusal of the caller's (INV-04, ic's finding 1 on ST0092 WP-03), driven on
/// a fixture install as `wb profile`'s own exit 2 is.
#[test]
fn an_install_whose_profiles_cannot_be_read_exits_2() {
  let dir = project("wbinitunreadable");
  let bare = tempfile::tempdir().expect("an install with no profiles directory");
  let exe = crate::common::fake_install(bare.path());
  for args in [
    &["wb", "init", "team"][..],
    &["wb", "init", "team", "--apply"],
  ] {
    let out = testkit::fixtured_command(&exe)
      .current_dir(dir.path())
      .args(args)
      .stdin(testkit::lifeline_for(args))
      .output()
      .expect("spawn the fixture install's intent");
    let err = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
      out.status.code(),
      Some(2),
      "`intent {}`: {err}",
      args.join(" ")
    );
    assert!(err.contains("error:") && err.contains("remedy:"), "{err}");
  }
  assert!(
    !dir
      .path()
      .join("intent/whiteboard/declaration.json")
      .exists(),
    "an install that could not answer wrote the declaration"
  );
}

/// ST0092 AT-03.7 at the surface: `wb remit` refuses a remit with `--default`,
/// and refuses neither, each naming both forms.
#[test]
fn at_03_7_remit_refuses_both_and_neither() {
  let dir = project("wbremitdoor");
  let (out, err, code) = run(dir.path(), &["wb", "init", "team", "--apply"]);
  assert_eq!(code, 0, "{out}{err}");
  for args in [
    &["wb", "remit", "cc", "a remit", "--default"][..],
    &["wb", "remit", "cc"][..],
  ] {
    let (out, err, code) = run(dir.path(), args);
    assert_eq!(code, 1, "{args:?}: {out}{err}");
    assert!(err.contains("--default") && err.contains("<text>"), "{err}");
  }
  let (out, err, code) = run(dir.path(), &["wb", "remit", "cc", "this project's own"]);
  assert_eq!(code, 0, "{out}{err}");
  assert!(out.starts_with("ok: cc's remit set"), "{out:?}");
}
