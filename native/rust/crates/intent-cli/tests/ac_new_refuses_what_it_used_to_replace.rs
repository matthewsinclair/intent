//! **Issue 0119, then issue 0131: `ac new` on an existing id was a full
//! replace that said `created`. It now refuses, and `ac edit` is the door that
//! does what the caller wanted.**
//!
//! `ac new` was the only writer of a criterion's prose -- the other nine `ac`
//! arms are transitions on a row that already exists -- so repairing one
//! SENTENCE meant re-running it. Re-running with `--text` alone wrote the
//! `--kind` default over the stored value, and a criterion's state is derived
//! from its kind, so a test-backed criterion silently stopped being computed
//! from its covering tests and became plainly unsatisfied. **The gate then
//! passed over fewer criteria than anyone had counted**, and both runs printed
//! `created`.
//!
//! # This file used to test a DISCLOSURE, and the ruling that replaced it
//!
//! The first repair was a warning, and its reasoning was explicit: ic ratified
//! the shape as an idempotent PUT (2026-08-26), a PUT writes the whole
//! representation by definition, and both fixes issue 0119 proposed -- refuse
//! without `--replace`, or preserve unsupplied fields -- would overturn that
//! ratification. **So the semantics question went to the ratification pile
//! rather than being settled by whoever picked the issue up, and the pile has
//! now answered it.** hv ruled on 2026-08-28 (issue 0131) that a verb named
//! `add`/`new` must FAIL on an existing key. A later first-hand ruling from hv
//! on the same subject supersedes the ratification, the replace path is gone,
//! and the disclosure with it.
//!
//! **The arms were inverted rather than deleted.** What was "the warning fires
//! only when something was lost" is now "nothing can be lost through this
//! door, and the row is byte-intact after the refusal".
//!
//! # Why this drives the BINARY when the facade is already covered
//!
//! `a_create_refuses_a_child_id_that_is_taken.rs` proves the refusal at the
//! service layer. It cannot see the exit code, the message the operator
//! actually reads, or whether the CLI arm was wired at all -- and the surface
//! is built from `surface/dispatch-table.json`, so a row can be declared and
//! reach no dispatch. These arms run the real `intent`.

use std::path::Path;
use std::process::Command;

fn run(cwd: &Path, args: &[&str]) -> (String, String, i32) {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(cwd)
    .env("HOME", testkit::fixture_home())
    .output()
    .expect("run the v3 binary");
  (
    String::from_utf8_lossy(&out.stdout).into_owned(),
    String::from_utf8_lossy(&out.stderr).into_owned(),
    out.status.code().unwrap_or(-1),
  )
}

/// The stored criterion, read out of canon rather than out of a rendering.
/// **The rendering is the thing under test in half these arms**, so asserting
/// against it would let a broken write and a broken renderer cancel out.
fn stored(cwd: &Path, ac: &str) -> serde_json::Value {
  let text = std::fs::read_to_string(cwd.join("intent/.canon/st/ST0001.json"))
    .expect("the thread's canon file");
  let thread: serde_json::Value = serde_json::from_str(&text).expect("canon is JSON");
  thread["criteria"]
    .as_array()
    .expect("criteria")
    .iter()
    .find(|c| c["id"] == ac)
    .unwrap_or_else(|| panic!("{ac} is not in canon"))
    .clone()
}

/// A project with one thread and one TEST-BACKED criterion -- the shape that
/// has something to lose.
fn seeded() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  run(dir.path(), &["init", "acproj"]);
  run(dir.path(), &["st", "new", "A thread"]);
  let (_, err, code) = run(
    dir.path(),
    &[
      "ac",
      "new",
      "ST0001",
      "AC-01.1",
      "--text",
      "the original",
      "--kind",
      "test",
    ],
  );
  assert_eq!(code, 0, "the fixture criterion must exist: {err}");
  dir
}

#[test]
fn a_first_create_still_says_created() {
  let dir = tempfile::tempdir().expect("tempdir");
  run(dir.path(), &["init", "acproj"]);
  run(dir.path(), &["st", "new", "A thread"]);

  let (out, err, code) = run(
    dir.path(),
    &["ac", "new", "ST0001", "AC-01.1", "--text", "first"],
  );
  assert_eq!(code, 0, "{err}");
  assert!(
    out.contains("created"),
    "a first write is a creation and must still say so: {out:?}"
  );
}

/// **THE INVERSION.** This arm used to assert that the replace succeeded at
/// exit 0 and warned; it now asserts that it does not happen at all.
#[test]
fn a_second_create_refuses_and_leaves_the_row_exactly_as_it_was() {
  let dir = seeded();
  let before = stored(dir.path(), "AC-01.1");

  let (out, err, code) = run(
    dir.path(),
    &["ac", "new", "ST0001", "AC-01.1", "--text", "corrected"],
  );

  assert_ne!(code, 0, "a create on a taken id must not succeed: {out:?}");
  assert!(
    err.contains("already has criterion AC-01.1"),
    "the refusal must name the taken key: {err:?}"
  );
  assert!(
    !out.contains("created") && !out.contains("replaced"),
    "a refused write must not report one: {out:?}"
  );

  assert_eq!(
    stored(dir.path(), "AC-01.1"),
    before,
    "the refused create changed canon anyway. The `--kind` default overwriting a stored `test` \
     is issue 0119's exact mechanism, and a refusal that still writes reintroduces it under a \
     non-zero exit code, where nobody would look for it."
  );
}

/// **A REFUSAL THAT NAMES NO WAY FORWARD IS THE WALL THIS PACKAGE EXISTS TO
/// AVOID BUILDING**, and the operator reads the message rather than the
/// facade's variant name.
#[test]
fn the_refusal_tells_the_operator_which_verb_does_what_they_meant() {
  let dir = seeded();
  let (_, err, _) = run(
    dir.path(),
    &["ac", "new", "ST0001", "AC-01.1", "--text", "corrected"],
  );
  assert!(
    err.contains("ac edit ST0001 AC-01.1"),
    "the remedy must name the edit verb WITH the ids the caller just typed: {err:?}"
  );
}

/// The repair the refusal points at, driven end to end. **`kind` and `state`
/// are asserted by value out of canon**, because leaving them alone is the
/// entire difference between this verb and the one that caused issue 0119.
#[test]
fn ac_edit_rewords_through_the_binary_and_leaves_kind_and_state_alone() {
  let dir = seeded();
  let before = stored(dir.path(), "AC-01.1");
  assert_eq!(
    before["kind"], "test",
    "precondition: the fixture criterion is test-backed, which is what has something to lose"
  );

  let (out, err, code) = run(
    dir.path(),
    &["ac", "edit", "ST0001", "AC-01.1", "--text", "corrected"],
  );
  assert_eq!(code, 0, "the edit must succeed: {err:?}");
  assert!(out.contains("reworded"), "{out:?}");

  let after = stored(dir.path(), "AC-01.1");
  assert_eq!(after["text"], "corrected", "the reword did not land");
  assert_eq!(
    after["kind"], before["kind"],
    "the edit rewrote the kind -- issue 0119's mechanism, through the verb built to avoid it"
  );
  assert_eq!(
    after["state"], before["state"],
    "the edit moved the state, so satisfaction stopped being computed from covering tests"
  );
}

/// The AT side end to end: a re-cite moves the file and keeps the note. The
/// note is the field whose loss was measured at six rows on this repository.
#[test]
fn at_edit_re_cites_through_the_binary_and_keeps_the_note() {
  let dir = seeded();
  std::fs::create_dir_all(dir.path().join("tests")).expect("mkdir");
  std::fs::write(dir.path().join("tests/first.rs"), "// AT-01.1\n").expect("write");
  std::fs::write(dir.path().join("tests/moved.rs"), "// AT-01.1\n").expect("write");

  let (_, err, code) = run(
    dir.path(),
    &[
      "at",
      "new",
      "ST0001",
      "AT-01.1",
      "--covers",
      "AC-01.1",
      "--file",
      "tests/first.rs",
      "--status",
      "green",
      "--note",
      "green on the first run is not evidence",
    ],
  );
  assert_eq!(code, 0, "the fixture test row must exist: {err:?}");

  let (out, err, code) = run(
    dir.path(),
    &[
      "at",
      "edit",
      "ST0001",
      "AT-01.1",
      "--file",
      "tests/moved.rs",
    ],
  );
  assert_eq!(code, 0, "the re-cite must succeed: {err:?}");
  assert!(out.contains("re-cited"), "{out:?}");

  let text =
    std::fs::read_to_string(dir.path().join("intent/.canon/st/ST0001.json")).expect("canon file");
  let thread: serde_json::Value = serde_json::from_str(&text).expect("canon is JSON");
  let row = thread["tests"]
    .as_array()
    .expect("tests")
    .iter()
    .find(|t| t["id"] == "AT-01.1")
    .expect("the row");

  assert_eq!(row["file"], "tests/moved.rs", "the re-cite did not land");
  assert_eq!(
    row["note"], "green on the first run is not evidence",
    "the re-cite ate the note. This is the measured loss -- six ST0061 notes through `at new` -- \
     reproduced through the verb built to prevent it."
  );
  assert_eq!(
    row["status"], "green",
    "the re-cite reset the status, which `at green`/`at red`/`at na` own"
  );
}
