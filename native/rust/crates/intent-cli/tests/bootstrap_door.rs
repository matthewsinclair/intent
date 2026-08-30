//! **`intent bootstrap` through the door, under a fixture `HOME`.**
//!
//! `flag_reachability` establishes that SOMETHING in the renderer reads
//! `--force` and `--quiet`; it is a whole-file scan and says so in its own doc.
//! For this family that limit has teeth rather than being theoretical:
//! `organize` also reads an id spelled `force`, so removing `bootstrap`'s read
//! entirely leaves that gate green. **Measured, by doing it** -- the mutation
//! passed, which is what this file exists to stop.
//!
//! So the flags are driven here instead, by their EFFECT, which is the only
//! thing a whole-file scan cannot fake.
//!
//! # Every arm sets `HOME`, and that is the isolation
//!
//! `bootstrap` writes into per-user state. A test that let it reach the real
//! `HOME` would mutate the developer's machine to assert something about a
//! fixture -- and worse, would pass on a machine already carrying the state it
//! meant to create. `corpus_machine_independence.rs` establishes the technique
//! in this crate; `no_intent_home.rs` deliberately scans only `src/`, so tests
//! setting `HOME` are the sanctioned way to build a controlled environment.

use std::path::{Path, PathBuf};
use std::process::Command;

fn bin() -> PathBuf {
  std::path::PathBuf::from(env!("CARGO_BIN_EXE_intent"))
}

fn fixture(name: &str) -> PathBuf {
  let dir = std::env::temp_dir().join(format!("intent-bootstrap-door-{name}"));
  let _ = std::fs::remove_dir_all(&dir);
  std::fs::create_dir_all(&dir).expect("fixture home");
  dir
}

/// One run of `intent bootstrap` against a fixture home.
fn run(home: &Path, args: &[&str], user: Option<&str>) -> (String, String, i32) {
  let mut cmd = Command::new(bin());
  cmd.arg("bootstrap").args(args).env("HOME", home);
  match user {
    Some(u) => {
      cmd.env("USER", u);
    }
    None => {
      cmd.env_remove("USER");
    }
  }
  let out = cmd.output().expect("run intent bootstrap");
  (
    String::from_utf8_lossy(&out.stdout).into_owned(),
    String::from_utf8_lossy(&out.stderr).into_owned(),
    out.status.code().unwrap_or(-1),
  )
}

fn config(home: &Path) -> PathBuf {
  home.join(".intent/config.json")
}

/// **The whole point of the command: the pointer the shim reads gets written.**
///
/// R1's shim resolves the install root from exactly this file and nothing else.
/// Until this arm existed, `install::publish_home()` had no caller at all.
#[test]
fn it_publishes_the_pointer_the_shim_reads() {
  let home = fixture("pointer");
  let (stdout, stderr, code) = run(&home, &[], Some("matts"));
  assert_eq!(code, 0, "stdout={stdout}\nstderr={stderr}");

  let pointer = home.join(".intent/home");
  let recorded = std::fs::read_to_string(&pointer)
    .unwrap_or_else(|e| panic!("no pointer at {}: {e}", pointer.display()));
  let root = PathBuf::from(recorded.lines().next().expect("a line").trim());

  // **The value is checked as a PROPERTY, not against a literal.** The path
  // differs per checkout, and an assertion naming this machine's would be a
  // test of where the suite happens to run -- which is the shape this estate
  // keeps finding in its own instruments.
  assert!(
    root.join("lib/templates").is_dir(),
    "the pointer must name a real install (the marker the shim validates); got {}",
    root.display()
  );
}

#[test]
fn a_fresh_machine_gets_a_config_naming_the_author_from_user() {
  let home = fixture("fresh");
  let (stdout, stderr, code) = run(&home, &[], Some("matts"));
  assert_eq!(code, 0, "stdout={stdout}\nstderr={stderr}");

  let text = std::fs::read_to_string(config(&home)).expect("config written");
  assert!(text.contains("\"author\": \"matts\""), "{text}");
  assert!(stdout.contains("author: matts"), "{stdout}");
}

/// **`--quiet` is DRIVEN, not merely spelled.** This is the arm the whole-file
/// scan cannot provide: it proves the renderer acts on the flag.
#[test]
fn quiet_suppresses_the_report_and_still_does_the_work() {
  let home = fixture("quiet");
  let (stdout, stderr, code) = run(&home, &["--quiet"], Some("matts"));
  assert_eq!(code, 0, "stderr={stderr}");
  assert_eq!(stdout, "", "--quiet must print nothing on success");

  // The work still happened -- a quiet flag that also skipped the setup would
  // pass an assertion about silence and be useless.
  assert!(home.join(".intent/home").is_file(), "pointer not written");
  assert!(config(&home).is_file(), "config not written");
}

/// **`--force` is DRIVEN by its effect on the file.**
#[test]
fn force_recreates_the_config_and_a_bare_run_does_not() {
  let home = fixture("force");
  run(&home, &[], Some("first"));
  let before = std::fs::read_to_string(config(&home)).expect("first write");
  assert!(before.contains("\"author\": \"first\""), "{before}");

  // Bare: keeps, byte for byte, even though USER now says something else.
  let (stdout, _, code) = run(&home, &[], Some("second"));
  assert_eq!(code, 0);
  assert_eq!(
    before,
    std::fs::read_to_string(config(&home)).expect("read"),
    "a bare re-run must not rewrite the config"
  );
  assert!(stdout.contains("already exists"), "{stdout}");

  // --force: replaces.
  let (stdout, _, code) = run(&home, &["--force"], Some("second"));
  assert_eq!(code, 0);
  let after = std::fs::read_to_string(config(&home)).expect("read");
  assert!(after.contains("\"author\": \"second\""), "{after}");
  assert!(stdout.contains("replaced"), "{stdout}");
}

/// An unset `USER` is a normal answer: the config is still written, and the
/// operator is told rather than left to find `unknown` in a project later.
#[test]
fn an_absent_user_still_sets_the_machine_up_and_says_the_author_is_unset() {
  let home = fixture("nouser");
  let (stdout, stderr, code) = run(&home, &[], None);
  assert_eq!(code, 0, "stderr={stderr}");
  assert!(config(&home).is_file(), "config must still be written");
  assert!(stdout.contains("author is unset"), "{stdout}");

  let text = std::fs::read_to_string(config(&home)).expect("read");
  assert!(
    !text.contains("author"),
    "no author key when USER names nobody: {text}"
  );
}

/// **The second run reports the pointer as unchanged rather than re-announcing
/// a write.** An operator re-running setup must be able to tell "already done"
/// from "done again".
#[test]
fn a_second_run_reports_the_pointer_unchanged() {
  let home = fixture("idempotent");
  let (first, _, _) = run(&home, &[], Some("matts"));
  assert!(first.contains("created:"), "{first}");

  let (second, _, code) = run(&home, &[], Some("matts"));
  assert_eq!(code, 0);
  assert!(
    second.contains("already recorded"),
    "the second run must not claim a fresh write: {second}"
  );
}

/// **v2 prints `export INTENT_HOME=...` here and v3 must never do so** -- the
/// binary reads no such variable, and setup advice for a variable nothing
/// reads teaches a wrong model of the tool.
#[test]
fn it_never_prints_the_v2_environment_advice() {
  let home = fixture("noenvadvice");
  let (stdout, _, _) = run(&home, &[], Some("matts"));
  assert!(!stdout.contains("INTENT_HOME"), "{stdout}");
  assert!(!stdout.contains("export "), "{stdout}");
}
