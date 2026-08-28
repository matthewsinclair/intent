//! **Issue 0119: `ac new` on an existing id is a full replace, and it said
//! `created` while doing it.**
//!
//! `ac new` is the only writer of a criterion's prose -- the other nine `ac`
//! arms are transitions on a row that already exists -- so repairing one
//! SENTENCE means re-running it. Re-running it with `--text` alone wrote the
//! `--kind` default over the stored value, and a criterion's state is derived
//! from its kind, so a test-backed criterion silently stopped being computed
//! from its covering tests and became plainly unsatisfied. **The gate then
//! passed over fewer criteria than anyone had counted**, and both runs printed
//! `created`.
//!
//! # Why this DISCLOSES rather than refusing or merging
//!
//! ic ratified the shape as an idempotent PUT to the entity address (2026-08-26,
//! on hv's ruling over issue 0088), and a PUT writes the whole representation
//! by definition. Both fixes the issue proposes -- refuse without `--replace`,
//! or preserve unsupplied fields -- would overturn that ratification, which the
//! issue does not cite. **The defect that survives the ruling is that the door
//! FABRICATES the half the caller omitted and then reports the write as a
//! creation.** So the semantics question goes to the ratification pile and this
//! closes the silence.
//!
//! # The controls are the whole test
//!
//! "a warning appears" is satisfied by a build that warns on every replace,
//! which would train the reader to ignore it. Three arms below establish that
//! it fires ONLY when the caller did not name the field AND the stored value
//! was actually lost.

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
fn a_create_says_created_and_discloses_nothing() {
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
  assert!(
    !err.contains("warning:"),
    "nothing was overwritten, so there is nothing to disclose: {err:?}"
  );
}

#[test]
fn a_replace_that_drops_a_kind_the_caller_did_not_name_says_so() {
  let dir = seeded();

  let (out, err, code) = run(
    dir.path(),
    &["ac", "new", "ST0001", "AC-01.1", "--text", "corrected"],
  );

  assert_eq!(
    code, 0,
    "the PUT still succeeds -- this is disclosure, not a refusal"
  );
  assert!(
    !out.contains("created"),
    "the write replaced an existing criterion and must not call that a creation: {out:?}"
  );
  assert!(
    err.contains("warning:") && err.contains("--kind"),
    "the caller was not told that the stored kind was overwritten: {err:?}"
  );
  // The consequence, not just the field: a reader who does not already know
  // that state is derived from kind cannot act on the field name alone.
  assert!(
    err.contains("unsatisfied"),
    "the disclosure must name what it COST, not only what it changed: {err:?}"
  );
}

#[test]
fn a_replace_the_caller_spelled_out_is_not_a_warning() {
  let dir = seeded();

  // Same value as stored: nothing lost, and the caller said it.
  let (_, kept, _) = run(
    dir.path(),
    &[
      "ac", "new", "ST0001", "AC-01.1", "--text", "second", "--kind", "test",
    ],
  );
  assert!(
    !kept.contains("warning:"),
    "naming the field it already had is not a loss: {kept:?}"
  );

  // DIFFERENT value from stored -- a real change, but a deliberate one. The
  // warning is about a silent default, never about the caller's own intent.
  let (_, chosen, _) = run(
    dir.path(),
    &[
      "ac", "new", "ST0001", "AC-01.1", "--text", "third", "--kind", "non-test",
    ],
  );
  assert!(
    !chosen.contains("warning:"),
    "the caller asked for non-test; warning about a change they typed would train them to \
     ignore the one that matters: {chosen:?}"
  );
}

#[test]
fn a_replace_that_loses_nothing_is_not_a_warning() {
  let dir = tempfile::tempdir().expect("tempdir");
  run(dir.path(), &["init", "acproj"]);
  run(dir.path(), &["st", "new", "A thread"]);
  run(
    dir.path(),
    &["ac", "new", "ST0001", "AC-01.1", "--text", "first"],
  );

  // Stored kind is already the default, so the default overwrites nothing.
  let (out, err, _) = run(
    dir.path(),
    &["ac", "new", "ST0001", "AC-01.1", "--text", "second"],
  );
  assert!(
    out.contains("replaced"),
    "it is still a replace and still says so: {out:?}"
  );
  assert!(
    !err.contains("warning:"),
    "the default equalled the stored value, so nothing was lost: {err:?}"
  );
}
