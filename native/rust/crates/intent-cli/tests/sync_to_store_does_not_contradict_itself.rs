//! **A VERB MUST NOT PRINT BOTH ANSWERS TO THE QUESTION IT WAS ASKED.**
//!
//! `sync --to-store` printed `note: ... this restore overwrites nothing`
//! immediately followed by `ok: store replaced from the extract, 1 thread(s)`.
//! An operator checking whether a repair had run was told both, and **the
//! second is the one that reads as a result** -- so a run that changed nothing
//! reported itself as a replacement.
//!
//! Reproduced on a clean v3 fixture on 2026-08-28 while confirming what
//! survived of issue `0111` after its filed mechanism was withdrawn. It is the
//! part of that issue that holds under either account of the Lamplight
//! observation, which is why it is the part that got fixed.
//!
//! # This asserts the INVARIANT, not the wording
//!
//! Pinning the exact sentences would break on every rephrasing and would teach
//! the next author nothing. The property is that the two lines cannot
//! disagree: **if the run says it overwrote nothing, it must not also say the
//! store was replaced.** Any rewording that keeps that promise passes.
//!
//! # The control is the other arm, and without it this proves nothing
//!
//! An estate where the two sides genuinely differ must still produce the
//! warning AND the word `replaced` -- otherwise this file is satisfied by a
//! binary that stopped saying `replaced` at all, or by one whose `sync` stopped
//! working, and neither is the fix.

use std::path::Path;
use std::process::Command;

fn run(cwd: &Path, args: &[&str]) -> (String, i32) {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(cwd)
    .env("HOME", testkit::fixture_home())
    .output()
    .expect("run the v3 binary");
  (
    format!(
      "{}{}",
      String::from_utf8_lossy(&out.stdout),
      String::from_utf8_lossy(&out.stderr)
    ),
    out.status.code().unwrap_or(-1),
  )
}

/// An estate carrying one thread with one criterion and one acceptance test,
/// so the extract has a field worth disagreeing about.
fn seeded() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  run(dir.path(), &["init", "syncproj"]);
  run(dir.path(), &["st", "new", "A thread"]);
  run(
    dir.path(),
    &["ac", "new", "ST0001", "AC-01.1", "--text", "a criterion"],
  );
  run(
    dir.path(),
    &[
      "at",
      "new",
      "ST0001",
      "AT-01.1",
      "--covers",
      "AC-01.1",
      "--file",
      "test/probe_test.exs",
    ],
  );
  dir
}

#[test]
fn a_run_that_overwrote_nothing_does_not_report_a_replacement() {
  let dir = seeded();
  let (out, rc) = run(dir.path(), &["sync", "--to-store", "ST0001"]);

  assert_eq!(rc, 0, "sync --to-store refused on a healthy estate: {out}");
  assert!(
    out.contains("overwrites nothing"),
    "the fixture was built so the store and the extract match, and the verb did not say so -- \
     this test is not exercising the branch it was written for: {out}"
  );
  assert!(
    !out.contains("store replaced from the canon extract"),
    "the run reported overwriting nothing AND replacing the store. Those are answers to the same \
     question and only one of them can be true -- and `replaced` is the one an operator reads as \
     the result: {out}"
  );
}

#[test]
fn a_run_that_did_overwrite_still_says_so() {
  let dir = seeded();

  // Make the extract genuinely differ from the store, which is the only state
  // in which `replaced` is a true word.
  let extract = dir.path().join("intent/.canon/st/ST0001.json");
  let text = std::fs::read_to_string(&extract).expect("read the extract");
  assert!(
    text.contains("test/probe_test.exs"),
    "the fixture's citation is not in the extract, so the edit below would change nothing and \
     the control would pass vacuously"
  );
  std::fs::write(
    &extract,
    text.replace("test/probe_test.exs", "test/changed_test.exs"),
  )
  .expect("write the extract");

  let (out, rc) = run(dir.path(), &["sync", "--to-store", "ST0001"]);

  assert_eq!(rc, 0, "sync --to-store refused: {out}");
  assert!(
    out.contains("OVERWRITES") && out.contains("ST0001: differs on disk"),
    "a genuine store/extract divergence was not reported -- the warning path is what makes the \
     other test's silence meaningful: {out}"
  );
  assert!(
    out.contains("store replaced from the canon extract"),
    "the verb stopped saying `replaced` even when it did replace, which satisfies the sibling \
     test for the wrong reason: {out}"
  );
}
