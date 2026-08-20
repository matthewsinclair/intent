//! AT-06.4, covering ST0057 AC-06.4 -- **`intent init` creates a working
//! project from an empty directory, and the text realisation is exercised
//! end-to-end from one.**
//!
//! **BOTH HALVES, IN ONE TEST, BECAUSE THE CRITERION IS ONE CLAIM.** "Creates a
//! project" and "the realisation runs from it" are separately checkable and
//! separately worthless: a directory with the right files in it is not
//! evidence that the tool can work in it, and a realisation driven from a
//! hand-built fixture says nothing about what `init` produces. The criterion's
//! own wording is why -- *you cannot demonstrate a fallback from a clean
//! directory if you cannot create a clean directory*.

use std::path::Path;
use std::process::Command;

fn run(args: &[&str], cwd: &Path) -> (String, String, i32) {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(cwd)
    .output()
    .expect("run the v3 binary");
  (
    String::from_utf8_lossy(&out.stdout).into_owned(),
    String::from_utf8_lossy(&out.stderr).into_owned(),
    out.status.code().unwrap_or(-1),
  )
}

/// **EMPTY MEANS EMPTY, AND IT IS ASSERTED RATHER THAN ASSUMED.**
///
/// A fixture built to have nothing in it is the one most likely to have
/// something in it -- `parity.md:338` records a "no git" fixture that sat
/// inside a repository and reported a measurement of something else entirely.
/// The directory is checked before the command runs.
fn empty_dir() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let count = std::fs::read_dir(dir.path())
    .expect("read the fixture")
    .count();
  assert_eq!(count, 0, "the empty fixture is not empty: {count} entries");
  dir
}

#[test]
fn init_creates_a_project_from_nothing() {
  let dir = empty_dir();
  let (out, err, code) = run(&["init", "fixture-project"], dir.path());
  assert_eq!(code, 0, "init failed: {err}");

  // THE ONE FILE THAT MAKES IT A PROJECT. Named on its own because everything
  // else `init` writes is starter content whose absence would not stop the
  // tool working -- and a test that checked only the total would not tell the
  // two apart.
  assert!(
    dir.path().join("intent/.config/config.json").is_file(),
    "no config was written, so the directory is not a project: {out}"
  );
  let config = std::fs::read_to_string(dir.path().join("intent/.config/config.json"))
    .expect("read the config");
  assert!(
    config.contains("\"project_name\": \"fixture-project\""),
    "the name on the command line did not reach the config: {config}"
  );

  // **THE CREATION TIME CAME FROM THE DATABASE (D42).** Nothing in `init` may
  // take or ask for a time, so the only place this value can have come from is
  // the store's own INSERT. A non-empty `created` is the observable end of
  // that rule; an empty one would mean somebody passed a time in.
  assert!(
    config.contains("\"created\":") && !config.contains("\"created\": \"\""),
    "the config carries no creation time, so the store's stamp did not reach it: {config}"
  );
}

#[test]
fn the_project_init_creates_actually_works() {
  let dir = empty_dir();
  let (_, err, code) = run(&["init", "fixture-project"], dir.path());
  assert_eq!(code, 0, "init failed: {err}");

  // The first command anyone types. `st new` is the whole test of "working":
  // it opens the store, writes canon and renders views, so a project it
  // succeeds in is one every other verb can reach.
  let (out, err, code) = run(&["st", "new", "the first thread"], dir.path());
  assert_eq!(
    code, 0,
    "st new failed in a freshly initialised project: {err}"
  );
  assert_eq!(out.trim(), "created: ST0001");
}

/// **THE REALISATION, END TO END, FROM A DIRECTORY `init` MADE.** This is the
/// half of AC-06.4 that the criterion adds to "a working project", and it is
/// the reason the row was a PRECONDITION of the assurance rather than a
/// neighbouring gap.
#[test]
fn the_text_realisation_runs_from_an_initialised_project() {
  let dir = empty_dir();
  let (_, err, code) = run(&["init", "fixture-project"], dir.path());
  assert_eq!(code, 0, "init failed: {err}");
  let (_, err, code) = run(&["st", "new", "the first thread"], dir.path());
  assert_eq!(code, 0, "st new failed: {err}");

  let (out, err, code) = run(&["export", "--format", "md"], dir.path());
  assert_eq!(code, 0, "the realisation was refused: {err}");
  assert!(
    out.contains(".backup/text/"),
    "it does not say where the realisation landed: {out}"
  );
  // COMPLETE, not merely non-empty. A realisation that wrote some of the
  // estate and reported success is the failure AC-06.1's denominator exists
  // for, and driving it from an `init`-made project is what closes the loop.
  assert!(
    out.contains("complete:"),
    "the realisation did not report itself complete from an init-made project: {out}"
  );
  assert!(
    out.contains("threads 1/1"),
    "the thread created above did not reach the realisation: {out}"
  );
}

/// **INIT REFUSES A DIRECTORY THAT IS ALREADY A PROJECT**, rather than merging
/// into it. Someone running `init` twice has almost certainly mistaken which
/// directory they are in, and overwriting a tuned config is not recoverable
/// from the message that would follow.
#[test]
fn init_refuses_an_existing_project() {
  let dir = empty_dir();
  let (_, err, code) = run(&["init", "fixture-project"], dir.path());
  assert_eq!(code, 0, "init failed: {err}");

  let (out, err, code) = run(&["init", "again"], dir.path());
  assert_ne!(code, 0, "a second init was accepted over a live project");
  assert!(
    out.is_empty(),
    "a refusal wrote {} bytes to stdout",
    out.len()
  );
  assert!(
    err.contains("already an Intent project"),
    "the refusal does not say what is wrong: {err}"
  );
}

/// **A FLAG WHOSE SUBSYSTEM IS ABSENT REFUSES BY NAME.** `--with-st0000` and
/// `--lang` are `disposition: keep` in the table, so they are not withheld --
/// they are declared and their machinery is not built. Accepting either would
/// report a project set up in a way it is not, and nothing downstream would
/// ever say so.
#[test]
fn a_flag_whose_subsystem_is_unimplemented_refuses_rather_than_ignoring() {
  for (flag, args) in [
    ("--with-st0000", vec!["init", "p", "--with-st0000"]),
    ("--lang", vec!["init", "p", "--lang", "rust"]),
  ] {
    let dir = empty_dir();
    let (_, err, code) = run(&args, dir.path());
    assert_ne!(code, 0, "`{flag}` was accepted with nothing behind it");
    assert!(
      err.contains(flag),
      "the refusal does not name the flag the operator typed: {err}"
    );
    // AND IT LEFT NOTHING BEHIND. A refusal that half-initialised would be
    // worse than either accepting or refusing cleanly.
    assert!(
      !dir.path().join("intent/.config/config.json").exists(),
      "`{flag}` refused and still created a project"
    );
  }
}
