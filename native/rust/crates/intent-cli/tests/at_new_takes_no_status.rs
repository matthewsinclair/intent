//! **`at new` TAKES NO STATUS, AND A NEW ROW STARTS AT ITS KIND'S ENTRY**
//! (issue 0339, vc's ruling of 2026-09-15).
//!
//! `at new --status green` created a passing row that nobody had seen fail,
//! around the red-before-green edge issue 0337 ruled for `at green`. Leaving the
//! flag off already gave the kind's entry, so the flag is retired on the register
//! and the facade no longer takes a status: a verdict is recorded afterwards with
//! `at red`, `at green` or `at na`.
//!
//! # Why this drives the BINARY
//!
//! The retirement is a surface fact. The flag leaves `--help` and the parser
//! refuses it, and neither is visible to a facade test, which can no longer even
//! express a status to pass.

use std::path::Path;

fn run(cwd: &Path, args: &[&str]) -> (String, String, i32) {
  let out = crate::common::intent()
    .args(args)
    .current_dir(cwd)
    .env("HOME", testkit::fixture_home())
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run the v3 binary");
  (
    String::from_utf8_lossy(&out.stdout).into_owned(),
    String::from_utf8_lossy(&out.stderr).into_owned(),
    out.status.code().unwrap_or(-1),
  )
}

/// The stored test rows of the thread whose id is `st`, read out of canon
/// rather than out of a rendering.
fn stored_tests(cwd: &Path, st: &str) -> Vec<serde_json::Value> {
  let text = std::fs::read_to_string(cwd.join(format!("intent/.canon/st/{st}.json")))
    .expect("the thread's canon file");
  let thread: serde_json::Value = serde_json::from_str(&text).expect("canon is JSON");
  thread["tests"].as_array().cloned().unwrap_or_default()
}

/// A project with one thread and one test-backed criterion for a row to cover.
/// It answers the project and the thread's id as its mint printed it.
fn seeded() -> (tempfile::TempDir, String) {
  let dir = tempfile::tempdir().expect("tempdir");
  run(dir.path(), &["init", "atproj"]);
  let st = crate::common::created(&run(dir.path(), &["st", "new", "A thread"]).0);
  let (_, err, code) = run(
    dir.path(),
    &[
      "ac",
      "new",
      st.as_str(),
      "AC-01.1",
      "--text",
      "a criterion",
      "--kind",
      "test",
    ],
  );
  assert_eq!(code, 0, "the fixture criterion must exist: {err}");
  (dir, st)
}

#[test]
fn a_named_status_is_refused_and_writes_no_row() {
  let (dir, st) = seeded();

  let (out, err, code) = run(
    dir.path(),
    &[
      "at",
      "new",
      st.as_str(),
      "AT-01.1",
      "--covers",
      "AC-01.1",
      "--status",
      "green",
    ],
  );

  assert_ne!(code, 0, "`at new --status green` must be refused: {out:?}");
  assert!(
    err.contains("--status"),
    "the refusal must name the flag it refused: {err:?}"
  );
  assert!(
    stored_tests(dir.path(), &st)
      .iter()
      .all(|t| t["id"] != "AT-01.1"),
    "a refused create must write no row"
  );
}

/// **The control that makes the refusal above mean something**: the same create
/// without the flag succeeds, so that arm refused the FLAG and not the verb.
#[test]
fn without_a_status_a_test_row_starts_at_to_write() {
  let (dir, st) = seeded();

  let (out, err, code) = run(
    dir.path(),
    &["at", "new", st.as_str(), "AT-01.1", "--covers", "AC-01.1"],
  );

  assert_eq!(code, 0, "the create must succeed: {out:?} {err:?}");
  let rows = stored_tests(dir.path(), &st);
  let row = rows
    .iter()
    .find(|t| t["id"] == "AT-01.1")
    .expect("the created row is in canon");
  assert_eq!(
    row["status"], "to-write",
    "a test row starts at its kind's entry: {row}"
  );
}
