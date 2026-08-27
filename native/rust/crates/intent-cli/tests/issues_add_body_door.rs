//! **THE CREATE DOOR FOR AN ISSUE'S PROSE** (hv, ruled GO 2026-08-27).
//!
//! `Issue::body` was declared in the model and carried through canon while
//! `issues add` took `<TITLE>` and `--severity` only, so the field had no
//! writer at all: prose was authored by editing the file, a route that stops
//! existing under the disk-optional model. 57 of 78 issues on this estate carry
//! a non-empty body.
//!
//! **THE PROPERTY THAT NEEDED A TEST IS THE REFUSAL, NOT THE CARRY.** `--body`
//! and `--from` both name the same field, and the tempting implementation gives
//! one precedence -- which resolves the ambiguity SILENTLY, in favour of
//! whichever the implementer tested first. An author who passes both has made a
//! mistake they can see; an author whose file was quietly discarded has one they
//! cannot.

use std::path::Path;
use std::process::{Command, Output};

fn project() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let config = dir.path().join("intent/.config");
  std::fs::create_dir_all(&config).expect("mkdir");
  std::fs::write(
    config.join("config.json"),
    "{\"intent_version\":\"3.0.0\",\"project_name\":\"I\",\"author\":\"cc\",\"intent_dir\":\"intent\",\"languages\":[\"rust\"]}\n",
  )
  .expect("write config");
  std::fs::create_dir_all(dir.path().join("intent/.canon/issues")).expect("mkdir issues");
  dir
}

fn run(root: &Path, args: &[&str]) -> Output {
  Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(root)
    .output()
    .expect("run the v3 binary")
}

fn body_of(root: &Path, number: &str) -> String {
  let shown =
    String::from_utf8_lossy(&run(root, &["issues", "show", number, "--json"]).stdout).to_string();
  let v: serde_json::Value = serde_json::from_str(&shown).expect("json");
  v["body"].as_str().unwrap_or_default().to_string()
}

/// `--body` reaches the record.
#[test]
fn an_inline_body_is_carried() {
  let dir = project();
  let made = run(
    dir.path(),
    &["issues", "add", "A thing", "--body", "the prose"],
  );
  assert!(made.status.success(), "add succeeds");
  assert_eq!(body_of(dir.path(), "0001"), "the prose");
}

/// `--from` reads the file, BYTE FOR BYTE -- no trim, because an issue body
/// round-trips to a document and a trim costs a byte on every trip.
#[test]
fn a_body_from_a_file_is_carried_whole() {
  let dir = project();
  let src = dir.path().join("body.md");
  std::fs::write(&src, "# Heading\n\n  ragged  \n\n").expect("write");
  let made = run(
    dir.path(),
    &[
      "issues",
      "add",
      "A thing",
      "--from",
      src.to_str().expect("utf8"),
    ],
  );
  assert!(made.status.success(), "add succeeds");
  assert_eq!(body_of(dir.path(), "0001"), "# Heading\n\n  ragged  \n\n");
}

/// **THE ARM THE DOOR EXISTS FOR: BOTH TOGETHER REFUSE.**
#[test]
fn body_and_from_together_refuse_rather_than_one_winning() {
  let dir = project();
  let src = dir.path().join("body.md");
  std::fs::write(&src, "from the file").expect("write");
  let out = run(
    dir.path(),
    &[
      "issues",
      "add",
      "A thing",
      "--body",
      "inline",
      "--from",
      src.to_str().expect("utf8"),
    ],
  );
  assert!(!out.status.success(), "passing both must not succeed");
  let said = String::from_utf8_lossy(&out.stderr).to_string();
  assert!(
    said.contains("--body") && said.contains("--from"),
    "the refusal names both flags: {said}"
  );
  // **AND IT CREATED NOTHING.** A refusal that still wrote the issue would be
  // worse than a precedence rule, because the author would have BOTH a refusal
  // and a record.
  assert!(
    !dir.path().join("intent/.canon/issues/0001.json").exists(),
    "a refused invocation writes no issue"
  );
}

/// An unreadable `--from` refuses and is NEVER a silently empty body -- which
/// would be the exact defect this door was opened to close, landing in the
/// record as an issue somebody wrote nothing in.
#[test]
fn an_unreadable_from_refuses_rather_than_creating_an_empty_body() {
  let dir = project();
  let out = run(
    dir.path(),
    &["issues", "add", "A thing", "--from", "no/such/file.md"],
  );
  assert!(
    !out.status.success(),
    "an unreadable --from must not succeed"
  );
  assert!(
    !dir.path().join("intent/.canon/issues/0001.json").exists(),
    "and writes no issue"
  );
}

/// The control: neither flag is the ordinary case, and it still works. Nobody
/// wrote a body, which is a state rather than a gap.
#[test]
fn neither_flag_leaves_the_body_empty_and_still_creates_the_issue() {
  let dir = project();
  let made = run(dir.path(), &["issues", "add", "A thing"]);
  assert!(made.status.success(), "add still works with no body flag");
  assert_eq!(body_of(dir.path(), "0001"), "");
}
