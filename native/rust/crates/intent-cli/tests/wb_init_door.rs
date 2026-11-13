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
    let out = exe.output(
      |p| {
        let mut c = testkit::fixtured_command(p);
        c.current_dir(dir.path())
          .args(args)
          .stdin(testkit::lifeline_for(args));
        c
      },
      "spawn the fixture install's intent",
    );
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

/// ST0092 AT-07.3 at the surface: a first render over a hand-written README
/// says it carried the whole file inside the block, and names the frontmatter
/// block it dropped on a `note:` line of its own; the dry run before it says
/// the same in the future tense and writes nothing.
#[test]
fn at_07_3_a_first_render_says_it_carried_the_readme_and_names_the_dropped_verblock() {
  let dir = project("wbreadmedoor");
  let readme = dir.path().join("intent/whiteboard/README.md");
  std::fs::create_dir_all(readme.parent().expect("a parent")).expect("mkdir");
  let hand = "---\nverblock: \"11 Nov 2026:v0.1: dc - by hand\"\n---\n\n# Ours\n\nBy hand.\n";
  std::fs::write(&readme, hand).expect("write");

  let (out, err, code) = run(dir.path(), &["wb", "init", "team"]);
  assert_eq!(code, 0, "{out}{err}");
  assert!(
    out.contains("README: first render: carries the whole existing README"),
    "{out}"
  );
  assert!(
    out.contains("would write: intent/whiteboard/README.md"),
    "{out}"
  );
  assert_eq!(
    std::fs::read_to_string(&readme).expect("read"),
    hand,
    "a dry run wrote"
  );

  let (out, err, code) = run(dir.path(), &["wb", "init", "team", "--apply"]);
  assert_eq!(code, 0, "{out}{err}");
  assert!(
    out.contains("README: first render: the whole existing README is carried inside its block"),
    "{out}"
  );
  assert!(
    out
      .lines()
      .any(|l| l.starts_with("note: the frontmatter block") && l.contains("verblock:")),
    "{out}"
  );
  assert!(out.contains("wrote: intent/whiteboard/README.md"), "{out}");
}

/// A fixture install carrying this repository's Team Profiles and nothing
/// else under its templates: no roster README template.
fn install_without_the_readme_template(root: &Path) -> crate::common::CopiedIntent {
  let exe = crate::common::fake_install(root);
  let profiles = testkit::repo_root().join(intentsvcs::teams::CANON_TEAMS);
  let into = root.join(intentsvcs::teams::CANON_TEAMS);
  std::fs::create_dir_all(&into).expect("the profiles directory");
  for entry in std::fs::read_dir(&profiles).expect("the shipped profiles") {
    let path = entry.expect("a profile").path();
    std::fs::copy(&path, into.join(path.file_name().expect("a name"))).expect("copy a profile");
  }
  assert!(
    !root
      .join(intentsvcs::install::MARKER)
      .join("whiteboard")
      .exists(),
    "the fixture carries the template"
  );
  exe
}

/// Run `args` with the fixture install's `intent` in `dir`: its exit code and
/// stderr.
fn run_with(exe: &crate::common::CopiedIntent, dir: &Path, args: &[&str]) -> (Option<i32>, String) {
  let out = exe.output(
    |p| {
      let mut c = testkit::fixtured_command(p);
      c.current_dir(dir)
        .args(args)
        .stdin(testkit::lifeline_for(args));
      c
    },
    "spawn the fixture install's intent",
  );
  (
    out.status.code(),
    String::from_utf8_lossy(&out.stderr).into_owned(),
  )
}

/// ST0092 WP-07 under INV-04: an install whose roster README template cannot
/// be read is this build unable to answer, exit 2, in the dry run and the
/// apply alike, and nothing is written.
#[test]
fn an_install_without_the_readme_template_exits_2_and_writes_nothing() {
  let dir = project("wbreadmenotemplate");
  let install = tempfile::tempdir().expect("a fixture install");
  let exe = install_without_the_readme_template(install.path());
  for args in [
    &["wb", "init", "team"][..],
    &["wb", "init", "team", "--apply"],
  ] {
    let (code, err) = run_with(&exe, dir.path(), args);
    assert_eq!(code, Some(2), "`intent {}`: {err}", args.join(" "));
    assert!(
      err.contains("error:") && err.contains("remedy:") && err.contains("template"),
      "{err}"
    );
  }
  for rel in [
    "intent/whiteboard/declaration.json",
    "intent/whiteboard/README.md",
  ] {
    assert!(
      !dir.path().join(rel).exists(),
      "an install that could not answer wrote {rel}"
    );
  }
}

/// ST0092 WP-03's owed arm under INV-04 (vc's ruling, cc's todo 48): `wb
/// register` on a declared board reads the board's profiles, and an install
/// with none is this build unable to answer, exit 2.
#[test]
fn wb_register_on_a_declared_board_with_no_profiles_exits_2() {
  let dir = project("wbregisternoprofiles");
  let (out, err, code) = run(dir.path(), &["wb", "init", "team", "--apply"]);
  assert_eq!(code, 0, "{out}{err}");
  let bare = tempfile::tempdir().expect("an install with no profiles directory");
  let exe = crate::common::fake_install(bare.path());
  let (code, err) = run_with(
    &exe,
    dir.path(),
    &["wb", "register", "zz", "--name", "Zed", "--role", "zed"],
  );
  assert_eq!(code, Some(2), "{err}");
  assert!(err.contains("error:") && err.contains("remedy:"), "{err}");
}
