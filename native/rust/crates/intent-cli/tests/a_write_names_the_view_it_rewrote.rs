//! **A PROJECTION THAT OVERWRITES A FILE SAYS WHICH FILE** (batch 4, hv's
//! silent-deletion gate, 2026-09-12).
//!
//! Every mutating verb projects the whole declared estate: `facade::projection`
//! adds each rendered view to the write set unconditionally, and `WriteSet`
//! skips a path whose bytes already match. **So the only file a mutation
//! actually overwrites is one whose bytes differ from the render, which on a
//! generated view means a file somebody edited by hand** -- and the verb's
//! whole output was `ok: <id> <moved>`, naming nothing.
//!
//! **THIS IS NOT A REFUSAL AND MUST NOT BECOME ONE.** A generated view has one
//! writer and the store is the SSOT (D01, reversed 2026-08-15), so rewriting it
//! is the correct act; what was missing is the record of it. The note is what
//! makes the act visible, and it goes to stderr with the other notes because
//! INV-01 governs the `ok:` result line on stdout.
//!
//! **THE CONTROL IS THE FIRST ASSERTION, NOT A COMMENT.** This file tests an
//! ANNOUNCEMENT, so it is worthless unless the thing being announced really
//! happened: each case asserts the hand-edited bytes are gone before it asks
//! whether the verb said so. A run where the overwrite did not occur would
//! otherwise pass by printing nothing, which is the vacuous green this estate
//! keeps re-finding.

use std::path::Path;
use std::process::Command;

fn run(args: &[&str], cwd: &Path) -> (String, String, i32) {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(cwd)
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run the v3 binary");
  (
    String::from_utf8_lossy(&out.stdout).into_owned(),
    String::from_utf8_lossy(&out.stderr).into_owned(),
    out.status.code().unwrap_or(-1),
  )
}

/// **Built by the binary, never by hand**, for the reason
/// `cli_write_moves_only_what_changed.rs` states: a hand-written estate says
/// nothing about what the tool produces.
///
/// TWO threads, and the second one is the subject. The defect is *a mutation
/// rewrote a view belonging to a thread it was never asked about*, and on a
/// one-thread estate the subject's own view and the collateral one are the same
/// file -- so the case would pass against the very defect it exists to catch.
fn estate() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  let (_, err, code) = run(&["init", "rewrote-fixture"], root);
  assert_eq!(code, 0, "fixture init failed: {err}");
  for title in ["The thread that moves", "The thread that is edited"] {
    let (_, err, code) = run(&["st", "new", title], root);
    assert_eq!(code, 0, "fixture st new failed: {err}");
  }
  // `st new` writes the STORE; the tree is realised lazily, so the views this
  // case is about do not exist until something asks for them.
  for id in ["ST0001", "ST0002"] {
    let (_, err, code) = run(&["edit", "st", id, "--path"], root);
    assert_eq!(code, 0, "fixture realise failed for {id}: {err}");
  }
  dir
}

fn hand_edit(root: &Path, rel: &str) -> String {
  let path = root.join(rel);
  let text = std::fs::read_to_string(&path).expect("read the realised view");
  std::fs::write(&path, format!("{text}\nA LINE SOMEBODY TYPED\n")).expect("hand-edit the view");
  text
}

#[test]
fn a_mutation_names_the_unrelated_view_it_overwrote() {
  let dir = estate();
  let root = dir.path();
  let edited = "intent/st/ST0002/info.md";
  hand_edit(root, edited);

  let (out, err, code) = run(&["st", "start", "ST0001"], root);
  assert_eq!(code, 0, "the mutation itself failed: {err}");

  // THE CONTROL. If the hand-edited line survived, no overwrite happened and
  // everything below would be asking whether the verb announced a non-act.
  let after = std::fs::read_to_string(root.join(edited)).expect("read the view back");
  assert!(
    !after.contains("A LINE SOMEBODY TYPED"),
    "the hand edit survived, so this run never exercised an overwrite and the \
     assertions below would pass for free"
  );

  assert!(
    err.contains(edited),
    "the verb overwrote {edited} and did not name it. stdout: {out:?} stderr: {err:?}"
  );
  assert!(
    err.contains("rewrote:"),
    "the overwrite is not reported through the notes channel. stderr: {err:?}"
  );
  assert!(
    out.contains("ok: ST0001 started"),
    "the result line moved or changed shape: {out:?}"
  );
}

/// **SILENT ON A NO-OP, which is the same contract the event log holds.** A
/// second `st start` moves nothing, so the projection writes nothing, so there is
/// nothing to name -- and a note printed anyway would be a record of an act
/// that did not happen, which is how an operator learns to skim the notes that
/// are real.
#[test]
fn a_mutation_that_rewrites_nothing_names_nothing() {
  let dir = estate();
  let root = dir.path();
  let (_, err, code) = run(&["st", "start", "ST0001"], root);
  assert_eq!(code, 0, "the first start failed: {err}");

  let (_, err, code) = run(&["st", "start", "ST0001"], root);
  assert_eq!(code, 0, "the repeat start failed: {err}");
  assert!(
    !err.contains("rewrote:"),
    "a verb that wrote nothing still announced a rewrite: {err:?}"
  );
}
