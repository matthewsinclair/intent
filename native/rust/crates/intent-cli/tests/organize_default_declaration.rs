//! ST0057 **AC-11.1, AC-11.2, AC-11.4, AC-11.5**: `intent organize --default`
//! writes `.intentfiles` from thread status, and **never removes a file**.
//!
//! **DRIVEN THROUGH THE BINARY, NOT THE FACADE, BECAUSE THE CRITERIA ARE ABOUT
//! A COMMAND.** Two of the four -- the tty refusal and the alias -- exist only
//! at the surface: a facade-level test cannot tell `organize` from `organise`
//! and has no terminal to be absent from.
//!
//! # The one arm that is NOT driven here, named rather than left to be noticed
//!
//! **AC-11.2's `--force` answered `y` ON A TTY is not covered.** It needs a
//! pseudo-terminal, which this estate has no harness for. What IS covered is
//! the half that can fail silently: `--force` WITHOUT a tty writing nothing and
//! exiting non-zero. **The uncovered half fails loudly** -- an operator typing
//! it sees the prompt or does not -- while the covered half is the one that
//! would let a script regenerate a declaration with nobody watching.

use std::process::{Command, Output};

fn intent(dir: &std::path::Path, args: &[&str]) -> Output {
  Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(dir)
    // **stdin CLOSED, DELIBERATELY: that is what makes these runs non-tty.**
    // Inheriting the harness's stdin would make the result depend on whether
    // cargo was run from a terminal -- so the same assertion would pass locally
    // and mean something different in CI.
    .stdin(std::process::Stdio::null())
    .output()
    .expect("run the v3 binary")
}

/// A project with one open thread and one closed one.
fn project() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  assert!(
    intent(root, &["init", "Fixture"]).status.success(),
    "the fixture must initialise"
  );
  for title in ["An open thread", "A thread to close"] {
    assert!(
      intent(root, &["st", "new", title]).status.success(),
      "st new must succeed"
    );
  }
  dir
}

fn manifest(root: &std::path::Path) -> String {
  std::fs::read_to_string(root.join("intent/.intentfiles")).expect("manifest")
}

fn declared(text: &str) -> Vec<String> {
  text
    .lines()
    .filter(|l| l.starts_with("STEELTHREAD:"))
    .map(|l| l.trim().to_string())
    .collect()
}

/// Every file under the project, with its bytes -- the only way to assert
/// **AC-11.4** without trusting a report about what was written.
fn tree(root: &std::path::Path) -> std::collections::BTreeMap<String, Vec<u8>> {
  fn walk(
    dir: &std::path::Path,
    root: &std::path::Path,
    out: &mut std::collections::BTreeMap<String, Vec<u8>>,
  ) {
    let Ok(entries) = std::fs::read_dir(dir) else {
      return;
    };
    for e in entries.flatten() {
      let p = e.path();
      if p.is_dir() {
        walk(&p, root, out);
      } else if let Ok(bytes) = std::fs::read(&p) {
        out.insert(
          p.strip_prefix(root).unwrap_or(&p).display().to_string(),
          bytes,
        );
      }
    }
  }
  let mut out = std::collections::BTreeMap::new();
  walk(root, root, &mut out);
  out
}

// ---------------------------------------------------------------------------
// AC-11.1 -- ABSENT: WRITE IT FROM STATUS
// ---------------------------------------------------------------------------

#[test]
fn an_absent_manifest_is_written_from_status_and_declares_only_the_open() {
  let dir = project();
  let root = dir.path();
  std::fs::remove_file(root.join("intent/.intentfiles")).expect("start from absent");

  // **THREE POPULATIONS, NOT TWO, BECAUSE THE PREDICATE CHANGED UNDER THIS
  // TEST.** `--default` declares WIP and nothing else (hv, 2026-08-26), so the
  // fixture needs a thread that is neither WIP nor closed: `ST0002` stays in
  // `triage`. A fixture of one open and one closed thread CANNOT TELL THE TWO
  // RULES APART -- the old `!is_closed()` predicate declared Triage too, and
  // would pass an assertion written against it.
  assert!(
    intent(root, &["st", "start", "ST0001"]).status.success(),
    "moving the first thread to WIP must succeed"
  );

  let out = intent(root, &["organize", "--default"]);
  assert!(out.status.success(), "exit 0: {out:?}");

  assert_eq!(
    declared(&manifest(root)),
    vec!["STEELTHREAD:ST0001"],
    "the WIP thread and nothing else -- ST0002 is `triage`, which the old \
     `!is_closed()` rule declared and this one does not"
  );
}

// ---------------------------------------------------------------------------
// AC-11.2 -- PRESENT: CHANGE NOT ONE BYTE
// ---------------------------------------------------------------------------

#[test]
fn a_present_manifest_is_left_byte_identical_and_force_is_named() {
  let dir = project();
  let root = dir.path();
  let before = manifest(root);

  let out = intent(root, &["organize", "--default"]);
  assert!(out.status.success(), "exit 0 on a present file: {out:?}");
  assert_eq!(manifest(root), before, "not one byte changed");

  let said = String::from_utf8_lossy(&out.stdout).into_owned();
  assert!(
    said.contains("--force"),
    "the report must name the spelling that WOULD regenerate it, or the operator \
     is told no and not told how: {said}"
  );
  assert!(
    said.contains("declares"),
    "and must say how many entries the present file declares: {said}"
  );
}

#[test]
fn force_without_a_tty_writes_nothing_and_exits_non_zero() {
  let dir = project();
  let root = dir.path();
  let before = manifest(root);

  let out = intent(root, &["organize", "--default", "--force"]);
  assert_eq!(
    out.status.code(),
    Some(1),
    "no tty means no confirmation, and no confirmation means no write: {out:?}"
  );

  // **THE REASON, NOT JUST THE OUTCOME -- and this assertion is the whole test.**
  // vc mutated the tty guard to `if false` and this test STAYED GREEN: with the
  // guard gone, the confirmation read hits EOF on a null stdin, the answer is
  // not `y`, and the run refuses anyway. **Two different refusals sharing one
  // exit code, and the version of this test that read only the code could not
  // tell them apart** -- IN-AG-RED-CONTROL-001 in the file written to prove a
  // criterion. Only the tty guard can produce this wording.
  let said = String::from_utf8_lossy(&out.stderr).into_owned();
  assert!(
    said.contains("terminal"),
    "the refusal must name the TERMINAL as what is missing, or it is \
     indistinguishable from the empty-answer refusal one line further on: {said}"
  );
  assert!(
    said.contains("no flag or environment variable that answers for you"),
    "and must say that the absence of a human IS the refusal, which is the \
     property AC-11.2 measures: {said}"
  );
  assert_eq!(
    manifest(root),
    before,
    "the refusal must have written nothing"
  );

  // **THE CONTROL, AND IT IS WHAT STOPS THIS PASSING FOR THE WRONG REASON.**
  // A build where `--default` was broken outright would also refuse here. The
  // same fixture under bare `--default` must still succeed.
  assert!(
    intent(root, &["organize", "--default"]).status.success(),
    "bare --default on the same fixture must still work, or the refusal above \
     proves only that the verb is broken"
  );
}

#[test]
fn force_on_its_own_is_a_usage_error_rather_than_a_silent_no_op() {
  let dir = project();
  let out = intent(dir.path(), &["organize", "--force"]);
  assert_eq!(
    out.status.code(),
    Some(1),
    "`--force` without `--default` modifies nothing and must not read as a \
     forced reconciliation: {out:?}"
  );
}

// ---------------------------------------------------------------------------
// AC-11.4 -- NEVER REMOVES A FILE
// ---------------------------------------------------------------------------

#[test]
fn default_removes_no_file_belonging_to_an_undeclared_thread() {
  let dir = project();
  let root = dir.path();
  assert!(
    intent(root, &["st", "start", "ST0001"]).status.success(),
    "one thread WIP, so the declaration is non-empty and ST0002 is left out of it"
  );

  // **THE MANIFEST MUST BE ABSENT OR THIS VERB DOES NOTHING AT ALL.** `init`
  // now writes `.intentfiles` and `st new` adds each id to it, so by this point
  // the file is PRESENT -- and a present manifest is the no-op arm. The first
  // version of this test asserted that no file was removed by a run that never
  // executed anything downstream of the write, which is why vc's mutation
  // (`declare_default` calling `organize(Mode::Apply)`) could not redden it.
  // **A survival assertion needs a run that could have killed something.**
  std::fs::remove_file(root.join("intent/.intentfiles")).expect("make --default act");

  let before = tree(root);
  assert!(
    before.keys().any(|k| k.contains("ST0002")),
    "the UNDECLARED thread must have files on disk, or this test cannot fail"
  );

  assert!(
    intent(root, &["organize", "--default",]).status.success(),
    "exit 0"
  );

  let after = tree(root);
  for (path, bytes) in &before {
    // The manifest itself is the one file this verb writes.
    // The manifest is the one file this verb writes, and the store is not part
    // of the estate's authored surface.
    if path.ends_with(".intentfiles") || path.contains(".cache") {
      continue;
    }
    assert_eq!(
      after.get(path),
      Some(bytes),
      "{path} changed or vanished -- `--default` never removes a file"
    );
  }
}

// ---------------------------------------------------------------------------
// AC-11.5 -- ONE CODE PATH
// ---------------------------------------------------------------------------

#[test]
fn organise_and_organize_are_one_code_path() {
  let dir_z = project();
  let dir_s = project();
  std::fs::remove_file(dir_z.path().join("intent/.intentfiles")).expect("absent");
  std::fs::remove_file(dir_s.path().join("intent/.intentfiles")).expect("absent");

  let z = intent(dir_z.path(), &["organize", "--default"]);
  let s = intent(dir_s.path(), &["organise", "--default"]);

  assert_eq!(z.status.code(), s.status.code(), "same exit code");
  assert_eq!(
    String::from_utf8_lossy(&z.stdout),
    String::from_utf8_lossy(&s.stdout),
    "byte-identical output -- if the alias reached a second handler this is \
     where it would show"
  );
  assert_eq!(
    manifest(dir_z.path()),
    manifest(dir_s.path()),
    "byte-identical files"
  );
}
