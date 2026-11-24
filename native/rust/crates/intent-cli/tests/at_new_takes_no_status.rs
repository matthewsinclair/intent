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
/// rather than out of a rendering: each is a file of its own in the thread's
/// test directory (ST0096 WP-05), which is absent while it holds none.
fn stored_tests(cwd: &Path, st: &str) -> Vec<serde_json::Value> {
  use intentsvcs::project::{CANON_AT_DIR, canon_children_rel};
  let dir = crate::common::canon_file(cwd, &canon_children_rel(st, CANON_AT_DIR));
  let entries = match std::fs::read_dir(&dir) {
    Ok(entries) => entries,
    Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Vec::new(),
    Err(e) => panic!("list {}: {e}", dir.display()),
  };
  entries
    .map(|entry| {
      let path = entry.expect("a directory entry").path();
      let text = std::fs::read_to_string(&path).expect("a test's canon file");
      serde_json::from_str(&text).expect("canon is JSON")
    })
    .collect()
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

/// AE finding 4: `at new` says which state the row starts in and which verb
/// moves it on, and `at green` on that row names `at red` as the step to run
/// first.
#[test]
fn a_new_row_names_its_state_and_green_names_red_first() {
  let (dir, st) = seeded();
  let file = "tests/a_probe.rs";
  std::fs::create_dir_all(dir.path().join("tests")).expect("the test dir");
  std::fs::write(dir.path().join(file), "").expect("the cited test file");
  let next = format!("intent at red {st} AT-01.1");

  let (out, err, code) = run(
    dir.path(),
    &[
      "at",
      "new",
      st.as_str(),
      "AT-01.1",
      "--covers",
      "AC-01.1",
      "--file",
      file,
    ],
  );
  assert_eq!(code, 0, "the create must succeed: {out:?} {err:?}");
  assert!(
    out.contains("state: to-write") && out.contains(&format!("next: `{next}`")),
    "`at new` must name the row's state and the next verb: {out}"
  );

  let (out, err, code) = run(dir.path(), &["at", "green", st.as_str(), "AT-01.1"]);
  assert_ne!(code, 0, "green before red must be refused: {out:?}");
  assert!(
    err.contains(&format!("run `{next}` first")),
    "the refusal must name `at red` as the step to run first: {err}"
  );
}
