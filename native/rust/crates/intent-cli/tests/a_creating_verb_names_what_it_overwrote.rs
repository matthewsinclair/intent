//! **THE VERBS WITH NO NOTES CHANNEL ARE WHERE A SILENT OVERWRITE HIDES**
//! (batch 4, hv's silent-deletion gate; vc, 2026-09-12).
//!
//! `st start` and its siblings return an `Outcome`, so the record of what their
//! projection overwrote reaches the operator. `st new` returns an id and
//! `st attach` printed its own line without reading what came back -- and both
//! project the estate exactly as a transition does. Measured before this
//! landed: each of them overwrote a hand-edited generated view and said
//! nothing at all.
//!
//! **THE TWO GAPS ARE DIFFERENT SIZES AND THE TEST DOES NOT CARE.** `attach`
//! already carried the note and the CLI dropped it on the floor; `st new` had
//! nowhere to put one. From the operator's side those are the same defect, so
//! they are asserted the same way.
//!
//! Each case asserts the hand edit is GONE before asking whether the verb said
//! so, because a test of an announcement passes for free when the thing being
//! announced never happened.

use std::path::Path;

fn run(args: &[&str], cwd: &Path) -> (String, String, i32) {
  let out = crate::common::intent()
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

fn estate() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  let (_, err, code) = run(&["init", "creating-verb-fixture"], root);
  assert_eq!(code, 0, "fixture init failed: {err}");
  let (_, err, code) = run(&["st", "new", "The thread that exists"], root);
  assert_eq!(code, 0, "fixture st new failed: {err}");
  let (_, err, code) = run(&["edit", "st", "ST0001", "--path"], root);
  assert_eq!(code, 0, "fixture realise failed: {err}");
  dir
}

fn hand_edit(root: &Path, rel: &str) {
  let path = root.join(rel);
  let text = std::fs::read_to_string(&path).expect("read the generated view");
  std::fs::write(&path, format!("{text}\nA LINE SOMEBODY TYPED\n")).expect("hand-edit the view");
}

fn gone(root: &Path, rel: &str) {
  let after = std::fs::read_to_string(root.join(rel)).expect("read the view back");
  assert!(
    !after.contains("A LINE SOMEBODY TYPED"),
    "the hand edit survived in {rel}, so this run never exercised an overwrite"
  );
}

#[test]
fn st_attach_names_the_view_whose_bytes_were_not_the_stores_render() {
  let dir = estate();
  let root = dir.path();
  let edited = "intent/st/ST0001/info.md";
  hand_edit(root, edited);

  let source = root.join("attachment-source.md");
  std::fs::write(&source, "notes an operator wrote\n").expect("write the source file");
  let (out, err, code) = run(
    &[
      "st",
      "attach",
      "ST0001",
      "notes.md",
      "--from",
      source.to_str().expect("utf-8 path"),
    ],
    root,
  );
  assert_eq!(code, 0, "the attach itself failed: {err}");
  gone(root, edited);

  assert!(
    out.contains("ok: notes.md written to ST0001"),
    "the result line moved or changed shape: {out:?}"
  );
  assert!(
    err.contains(edited) && err.contains("were not the store's render"),
    "attach overwrote {edited} and did not name it: {err:?}"
  );
}

#[test]
fn st_new_names_the_estate_view_whose_bytes_were_not_the_stores_render() {
  let dir = estate();
  let root = dir.path();
  let edited = "intent/st/steel_threads.md";
  hand_edit(root, edited);

  let (out, err, code) = run(&["st", "new", "The thread that is created"], root);
  assert_eq!(code, 0, "the create itself failed: {err}");
  gone(root, edited);

  assert!(
    out.contains("created: ST0002"),
    "the result line moved or changed shape: {out:?}"
  );
  assert!(
    err.contains(edited) && err.contains("were not the store's render"),
    "st new overwrote {edited} and did not name it: {err:?}"
  );
}

/// **THE ORDINARY CREATE STAYS QUIET.** Both verbs rewrite the estate's views
/// every time they run, and a line about that on every create is the receipt
/// this batch already removed once.
#[test]
fn a_create_over_an_estate_that_agrees_says_nothing() {
  let dir = estate();
  let root = dir.path();
  let (_, err, code) = run(&["st", "new", "Another thread"], root);
  assert_eq!(code, 0, "the create itself failed: {err}");
  assert!(
    !err.contains("were not the store's render"),
    "a create that overwrote nobody's work still warned about it: {err:?}"
  );
}
