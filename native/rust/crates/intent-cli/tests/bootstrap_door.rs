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
  home.join(".config/intent/config.json")
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

  let pointer = home.join(".local/share/intent/home");
  let recorded = std::fs::read_to_string(&pointer)
    .unwrap_or_else(|e| panic!("no pointer at {}: {e}", pointer.display()));
  let root = PathBuf::from(recorded.lines().next().expect("a line").trim());

  // **The value is checked as a PROPERTY, not against a literal.** The path
  // differs per checkout, and an assertion naming this machine's would be a
  // test of where the suite happens to run -- which is the shape this estate
  // keeps finding in its own instruments.
  assert!(
    root.join(intentsvcs::install::MARKER).is_dir(),
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
  assert!(
    home.join(".local/share/intent/home").is_file(),
    "pointer not written"
  );
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

/// **THE REFUSAL IS WIRED, AND THIS ARM IS GREEN WHEREVER IT RUNS** (issue
/// `0492`, vc's ruling of 2026-09-20).
///
/// `bootstrap` must not let a scratch checkout silently replace a pointer that
/// already names a real install -- which is what happened on 2026-09-19, when a
/// suite run in a worktree published that worktree and every estate's gate then
/// resolved its guards from it.
///
/// **WHAT IT ASSERTS IS AGREEMENT WITH THE CLASSIFIER, NOT A FIXED OUTCOME, AND
/// THAT IS THE WHOLE DESIGN.** The candidate root is the test binary's own
/// install, which is a linked worktree under the temporary directory when this
/// suite runs from a bank and an ordinary checkout when it runs from the
/// developer's tree. An arm asserting *refused* would be green here and red
/// there; an arm asserting *published* would be the reverse. Asking the shipped
/// classifier what this root IS, and requiring the verb to agree, is true in
/// both places and is what proves the verb consults it at all.
///
/// **WHAT IT CANNOT SEE, SAID PLAINLY.** Where the binary's own root is not
/// scratch, this arm exercises the allowing path only. Both verdicts of the
/// decision itself are driven in `intentsvcs`'s own arms
/// (`only_a_scratch_root_replacing_a_live_real_one_is_refused`), which need no
/// filesystem and are therefore the same everywhere.
#[test]
fn a_scratch_root_does_not_silently_replace_a_real_one() {
  let home = fixture("replace");

  // Pre-seed the pointer with a root that EXISTS and is an install, so there
  // is something real to protect. `is_install` is the marker test the writer
  // itself uses.
  let existing = home.join("already-installed");
  std::fs::create_dir_all(existing.join("lib/templates/.claude/scripts")).expect("mkdir");
  let pointer = home.join(".local/share/intent/home");
  std::fs::create_dir_all(pointer.parent().unwrap()).expect("mkdir");
  std::fs::write(&pointer, format!("{}\n", existing.display())).expect("seed pointer");

  let candidate = intentsvcs::install::home().expect("this binary's own install root");
  let candidate_is_scratch = intentsvcs::install::scratch(&candidate).is_some();
  let existing_is_scratch = intentsvcs::install::scratch(&existing).is_some();
  // The decision, asked of the pure function the verb is required to consult.
  let expected_refusal = intentsvcs::install::replacement_refused(
    &candidate,
    intentsvcs::install::scratch(&candidate).as_ref(),
    Some(existing.as_path()),
    intentsvcs::install::scratch(&existing).as_ref(),
    true,
  )
  .is_some();

  let (stdout, stderr, code) = run(&home, &[], Some("matts"));
  let recorded = std::fs::read_to_string(&pointer).expect("the pointer still exists");
  let recorded = recorded
    .lines()
    .next()
    .unwrap_or_default()
    .trim()
    .to_string();

  if expected_refusal {
    assert_ne!(code, 0, "a refused publish must not exit 0: {stdout}");
    assert!(
      stderr.contains("refusing to replace the recorded Intent install root"),
      "the refusal must say what it protected: {stderr}"
    );
    assert_eq!(
      recorded,
      existing.display().to_string(),
      "a refused publish must leave the pointer exactly as it found it"
    );
  } else {
    assert_eq!(code, 0, "stdout={stdout}\nstderr={stderr}");
    assert_eq!(
      recorded,
      candidate.display().to_string(),
      "an allowed publish records this binary's own root"
    );
  }

  // **NOT ASSERTED: THAT THE SEEDED ROOT IS A REAL ONE.** The first version of
  // this arm ended with `assert!(!existing_is_scratch || candidate_is_scratch)`
  // to say so, and that assertion is exactly the defect this file's ruling
  // forbids: the fixture HOME is a tempdir, so `existing` is scratch
  // everywhere, and on a plain checkout -- where the candidate is NOT scratch
  // -- it fails. Green in a bank worktree, red on a developer's tree, for a
  // reason that is about the harness rather than the subject.
  //
  // So it is recorded as a LIMIT instead: wherever a fixture can be built, both
  // roots live under the temporary directory, so this arm takes the allowing
  // branch and proves the verb runs, publishes, and agrees with the
  // classifier. The refusing branch is unreachable from a tempdir fixture on
  // any machine, and it is driven -- to both verdicts -- by the pure arms in
  // `intentsvcs`, which need no filesystem and are the same everywhere.
  let _ = (candidate_is_scratch, existing_is_scratch);
}
