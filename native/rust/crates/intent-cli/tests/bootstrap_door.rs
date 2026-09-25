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

fn fixture(name: &str) -> PathBuf {
  let dir = std::env::temp_dir().join(format!("intent-bootstrap-door-{name}"));
  let _ = std::fs::remove_dir_all(&dir);
  std::fs::create_dir_all(&dir).expect("fixture home");
  dir
}

/// One run of `intent bootstrap` against a fixture home.
fn run(home: &Path, args: &[&str], user: Option<&str>) -> (String, String, i32) {
  let mut cmd = crate::common::intent();
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

// ---- `--check` (issue `0533`) ----
//
// Every arm below takes a fresh `tempfile` HOME rather than `fixture`'s fixed
// path, because the first of them asserts that the HOME it was handed is still
// empty afterwards, and a fixed path is shared with any other run of this file.

/// Plant the install pointer the shim reads, naming `root`.
fn plant_pointer(home: &Path, root: &Path) -> PathBuf {
  let pointer = home.join(".local/share/intent/home");
  std::fs::create_dir_all(pointer.parent().expect("a parent")).expect("mkdir");
  std::fs::write(&pointer, format!("{}\n", root.display())).expect("plant the pointer");
  pointer
}

/// A directory that is an install by the marker the shim tests.
fn an_install(at: &Path) -> PathBuf {
  let hooks = at.join(intentsvcs::install::MARKER).join("hooks");
  std::fs::create_dir_all(&hooks).expect("mkdir");
  std::fs::write(hooks.join("pre-commit.sh"), "#!/usr/bin/env bash\n")
    .expect("write the gate body");
  at.to_path_buf()
}

/// **NO POINTER: THE GATE CANNOT RUN, SO THE CHECK EXITS 1, AND IT WRITES
/// NOTHING.** `--check` shares a verb with the one command that writes the
/// pointer, so the HOME it was handed empty must still be empty: a check that
/// wrote the pointer would pass every later arm and be the defect itself.
#[test]
fn check_with_no_pointer_exits_1_and_writes_nothing() {
  let home = tempfile::tempdir().expect("fixture home");
  let (stdout, stderr, code) = run(home.path(), &["--check"], Some("matts"));
  assert_eq!(code, 1, "stdout={stdout}\nstderr={stderr}");
  assert_eq!(stderr, "", "the verdict is on stdout, as --where's is");
  let pointer = home.path().join(".local/share/intent/home");
  assert!(
    stdout.contains(&format!("pointer:  {}", pointer.display())),
    "{stdout}"
  );
  assert!(stdout.contains("state:    ABSENT"), "{stdout}");
  assert_eq!(
    std::fs::read_dir(home.path()).expect("read HOME").count(),
    0,
    "--check wrote under HOME"
  );
}

/// **AN EMPTY POINTER IS ABSENT TO THE GATE, AND `--check` SAYS WHICH ABSENT
/// IT IS** (vc's ruling, 2026-09-23): the file exists and names nothing.
///
/// **AND THE SHIM'S `--where` NAMES IT THE SAME WAY** (issue `0547`). It used
/// to print `UNUSABLE` with an `<empty>` root, which sends a reader looking for
/// a broken root that was never written. Both forms of empty are driven: no
/// bytes at all, and the lone newline a truncated write leaves.
#[test]
fn check_on_an_empty_pointer_says_it_is_empty() {
  const EMPTY: &str = "state:    ABSENT (the pointer file exists and is empty)";
  let shim = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("../../../../lib/templates/hooks/pre-commit-shim.sh");
  for bytes in ["", "\n"] {
    let home = tempfile::tempdir().expect("fixture home");
    let pointer = home.path().join(".local/share/intent/home");
    std::fs::create_dir_all(pointer.parent().expect("a parent")).expect("mkdir");
    std::fs::write(&pointer, bytes).expect("an empty pointer");

    let (stdout, stderr, code) = run(home.path(), &["--check"], Some("matts"));
    assert_eq!(code, 1, "{bytes:?}: stdout={stdout}\nstderr={stderr}");
    assert!(stdout.contains(EMPTY), "{bytes:?}: {stdout}");

    let out = std::process::Command::new("bash")
      .arg(&shim)
      .arg("--where")
      .env("HOME", home.path())
      .env_remove("XDG_DATA_HOME")
      .output()
      .expect("run the shim's --where");
    let where_ = String::from_utf8_lossy(&out.stdout);
    assert_eq!(out.status.code(), Some(1), "{bytes:?}: {where_}");
    assert!(where_.contains(EMPTY), "{bytes:?}: --where said {where_}");
    assert!(!where_.contains("UNUSABLE"), "{bytes:?}: {where_}");
  }
}

/// **A POINTER NAMING NO INSTALL: EXIT 1, AND THE PATH QUOTED BACK**, as the
/// shim quotes it. The pointer is left exactly as found: repairing it is the
/// installer's job, never a check's.
#[test]
fn check_on_a_pointer_naming_no_install_exits_1_and_names_it() {
  let home = tempfile::tempdir().expect("fixture home");
  let elsewhere = home.path().join("not-an-install");
  std::fs::create_dir_all(&elsewhere).expect("mkdir");
  let pointer = plant_pointer(home.path(), &elsewhere);

  let (stdout, stderr, code) = run(home.path(), &["--check"], Some("matts"));
  assert_eq!(code, 1, "stdout={stdout}\nstderr={stderr}");
  assert!(
    stdout.contains(&format!("root:     {}", elsewhere.display())),
    "{stdout}"
  );
  assert!(
    stdout.contains("state:    UNUSABLE (no lib/templates under that root)"),
    "{stdout}"
  );
  assert!(!stdout.contains("gate:"), "no gate to name: {stdout}");
  assert_eq!(
    std::fs::read_to_string(&pointer).expect("the pointer"),
    format!("{}\n", elsewhere.display()),
    "--check must not repair the pointer"
  );
}

/// **THE POINTER NAMES THIS BINARY'S OWN INSTALL: EXIT 0 AND THE GATE NAMED.**
/// No note: the control for the next arm.
#[test]
fn check_on_this_install_exits_0_and_names_the_gate() {
  let home = tempfile::tempdir().expect("fixture home");
  let own = intentsvcs::install::home().expect("this binary's own install root");
  plant_pointer(home.path(), &own);

  let (stdout, stderr, code) = run(home.path(), &["--check"], Some("matts"));
  assert_eq!(code, 0, "stdout={stdout}\nstderr={stderr}");
  assert!(stdout.contains("state:    OK"), "{stdout}");
  assert!(
    stdout.contains(&format!(
      "gate:     {}",
      intentsvcs::install::gate_script(&own).display()
    )),
    "{stdout}"
  );
  assert!(!stdout.contains("this binary:"), "{stdout}");
  assert!(!stdout.contains("note:"), "{stdout}");
}

/// **ANOTHER INSTALL: THE GATE RUNS, AND BOTH ROOTS ARE PRINTED** (vc's rider,
/// 2026-09-23): the pointer's under `root:`, this binary's under `this binary:`.
#[test]
fn check_on_another_install_exits_0_and_prints_both_roots() {
  let home = tempfile::tempdir().expect("fixture home");
  let other = an_install(&home.path().join("another-install"));
  plant_pointer(home.path(), &other);
  let own = intentsvcs::install::home().expect("this binary's own install root");

  let (stdout, stderr, code) = run(home.path(), &["--check"], Some("matts"));
  assert_eq!(code, 0, "stdout={stdout}\nstderr={stderr}");
  assert!(
    stdout.contains(&format!("root:     {}", other.display())),
    "{stdout}"
  );
  assert!(
    stdout.contains("note: the gate will run from a DIFFERENT install than this binary."),
    "{stdout}"
  );
  assert!(
    stdout.contains(&format!("  this binary:      {}", own.display())),
    "{stdout}"
  );
  assert!(!stdout.contains("versioned Homebrew keg"), "{stdout}");
}

/// **AN INSTALL WITHOUT ITS GATE BODY: NO GATE, RC 1** (issue `0561`). The
/// shim refuses every commit in this state with its FAILURE 3, so `--check`
/// answering OK and 0 said the gate runs when it never does.
#[test]
fn check_on_an_install_without_its_gate_body_says_no_gate_and_exits_1() {
  let home = tempfile::tempdir().expect("fixture home");
  let broken = an_install(&home.path().join("broken-install"));
  std::fs::remove_file(
    broken
      .join(intentsvcs::install::MARKER)
      .join("hooks/pre-commit.sh"),
  )
  .expect("remove the gate body");
  plant_pointer(home.path(), &broken);

  let (stdout, stderr, code) = run(home.path(), &["--check"], Some("matts"));
  assert_eq!(code, 1, "stdout={stdout}\nstderr={stderr}");
  assert!(stdout.contains("state:    NO GATE"), "{stdout}");
  assert!(!stdout.contains("state:    OK"), "{stdout}");
}

/// **A VERSIONED HOMEBREW KEG IS A NOTE AT RC 0** (issues `0527` and `0533`).
/// The gate runs today, and the directory it runs from is the one the next
/// `brew upgrade` deletes.
#[test]
fn check_on_a_versioned_keg_notes_it_and_exits_0() {
  let home = tempfile::tempdir().expect("fixture home");
  let keg = an_install(&home.path().join("brew/Cellar/intent/9.9.9/libexec"));
  plant_pointer(home.path(), &keg);

  let (stdout, stderr, code) = run(home.path(), &["--check"], Some("matts"));
  assert_eq!(code, 0, "stdout={stdout}\nstderr={stderr}");
  assert!(stdout.contains("state:    OK"), "{stdout}");
  assert!(
    stdout.contains("note: the install pointer names a versioned Homebrew keg"),
    "{stdout}"
  );
}

/// **`--quiet` PRINTS NOTHING AND LEAVES THE VERDICT IN THE EXIT CODE**, both
/// ways.
#[test]
fn check_quiet_prints_nothing_and_keeps_the_exit_code() {
  let home = tempfile::tempdir().expect("fixture home");
  let (stdout, _, code) = run(home.path(), &["--check", "--quiet"], Some("matts"));
  assert_eq!((stdout.as_str(), code), ("", 1));

  let own = intentsvcs::install::home().expect("this binary's own install root");
  plant_pointer(home.path(), &own);
  let (stdout, _, code) = run(home.path(), &["--check", "--quiet"], Some("matts"));
  assert_eq!((stdout.as_str(), code), ("", 0));
}

/// **`--force` WITH `--check` IS REFUSED, AND NOTHING IS WRITTEN.** A check
/// has nothing to force, and a flag accepted and dropped reads as honoured.
#[test]
fn check_with_force_is_refused_and_writes_nothing() {
  let home = tempfile::tempdir().expect("fixture home");
  let (stdout, stderr, code) = run(home.path(), &["--check", "--force"], Some("matts"));
  assert_eq!(code, 1, "stdout={stdout}\nstderr={stderr}");
  assert!(stderr.contains("`--check` writes nothing"), "{stderr}");
  assert_eq!(stdout, "");
  assert_eq!(
    std::fs::read_dir(home.path()).expect("read HOME").count(),
    0,
    "a refused --check wrote under HOME"
  );
}

/// **`intent info` PRINTS THE SAME ANSWER ON ITS `Gate root:` LINE, AND ITS
/// `INTENT_HOME:` LINE AND EXIT CODE ARE AS THEY WERE** (issue `0533`). The
/// second run is the gate that cannot run, which must not reach the exit code.
#[test]
fn info_prints_the_gate_root_and_keeps_its_line_and_code() {
  let home = tempfile::tempdir().expect("fixture home");
  let other = an_install(&home.path().join("another-install"));
  let pointer = plant_pointer(home.path(), &other);
  let own = intentsvcs::install::home().expect("this binary's own install root");
  let info = || {
    let out = crate::common::intent()
      .arg("info")
      .env("HOME", home.path())
      .current_dir(home.path())
      .output()
      .expect("run intent info");
    (
      String::from_utf8_lossy(&out.stdout).into_owned(),
      out.status.code(),
    )
  };

  let (stdout, code) = info();
  assert_eq!(code, Some(0), "{stdout}");
  assert!(
    stdout.contains(&format!(
      "  Gate root:       {} (a different install from INTENT_HOME)\n",
      other.display()
    )),
    "{stdout}"
  );
  assert!(
    stdout.contains(&format!("  INTENT_HOME:     {}\n", own.display())),
    "the line the pre-commit gate parses: {stdout}"
  );

  std::fs::remove_file(&pointer).expect("remove the pointer");
  let (stdout, code) = info();
  assert_eq!(
    code,
    Some(0),
    "a gate that cannot run is not info's failure: {stdout}"
  );
  assert!(stdout.contains("  Gate root:       <none> -- "), "{stdout}");
}
