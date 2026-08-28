//! **A completion that already happened can be recorded as such** (hv,
//! 2026-08-28, on issue 0118's fifth limb).
//!
//! `st done` and `st cancel` stamped TODAY and took no override, so the only
//! open CLI path recorded threads finished in February as finished on the day
//! the estate migrated. On Conflab that is 50 of 51 remaining doctor findings.
//!
//! # The date is DATA, not a second clock
//!
//! `one_clock` says there is no clock in this workspace: a record is stamped by
//! the write that creates it. This does not add one. The store already writes
//! `COALESCE(NULLIF(?7, ''), strftime('%Y-%m-%d', 'now'))`, so a non-empty
//! value is recorded as given and only the empty one reaches the clock -- the
//! flag rides a path that was already there, and the facade still holds no time.
//!
//! # A calendar fact in the AUTHOR'S local day
//!
//! conflab-vc recovered 64 completion dates from git: 64/64 read as the local
//! day, 63/64 forced to UTC, the miss committed at 23:57 +0100. So the flag
//! records the date the caller states and converts nothing. **The whiteboard's
//! `date -u` discipline is the right rule in the wrong domain here** -- it
//! governs ordering stamps, and a completion date is a calendar fact.

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

/// A thread in flight whose gate will pass, so a refusal in these tests is
/// about the DATE and never about the contract.
fn closable(dir: &Path, id: &str, title: &str) {
  run(dir, &["st", "new", title]);
  run(dir, &["st", "start", id]);
  run(dir, &["ac", "new", id, "AC-01.1", "--text", "it works"]);
  run(
    dir,
    &["ac", "satisfy", id, "AC-01.1", "--evidence", "driven"],
  );
}

fn project() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  run(dir.path(), &["init", "dateproj"]);
  dir
}

fn completed(dir: &Path, id: &str) -> String {
  let raw = std::fs::read_to_string(dir.join(format!("intent/.canon/st/{id}.json")))
    .expect("the thread's canon");
  let v: serde_json::Value = serde_json::from_str(&raw).expect("canon parses");
  v["completed"].as_str().unwrap_or_default().to_string()
}

#[test]
fn a_stated_date_is_what_the_thread_records() {
  let dir = project();
  closable(dir.path(), "ST0001", "A thread finished in February");

  let (out, err, code) = run(
    dir.path(),
    &["st", "done", "ST0001", "--date", "2026-02-14"],
  );
  assert_eq!(code, 0, "{err}");
  assert!(out.contains("done"), "{out:?}");
  assert_eq!(completed(dir.path(), "ST0001"), "2026-02-14");
}

#[test]
fn cancel_takes_the_same_flag() {
  let dir = project();
  run(dir.path(), &["st", "new", "An overtaken thread"]);
  run(dir.path(), &["st", "start", "ST0001"]);

  let (_, err, code) = run(
    dir.path(),
    &[
      "st",
      "cancel",
      "ST0001",
      "--reason",
      "overtaken",
      "--date",
      "2026-03-01",
    ],
  );
  assert_eq!(code, 0, "{err}");
  assert_eq!(completed(dir.path(), "ST0001"), "2026-03-01");
  // `--keep` is on both closing verbs for the same reason; a flag on one of two
  // identical acts is a surface to be memorised rather than understood.
}

/// **THE CONTROL, and the tests above prove little without it.** Every
/// assertion there is satisfied by a build whose completion date is whatever
/// was last passed in -- including one that stopped consulting the store
/// entirely. Absent the flag, the store must still stamp the day.
#[test]
fn without_the_flag_the_store_still_stamps_the_day() {
  let dir = project();
  closable(dir.path(), "ST0001", "A thread finished today");

  let (_, err, code) = run(dir.path(), &["st", "done", "ST0001"]);
  assert_eq!(code, 0, "{err}");

  let recorded = completed(dir.path(), "ST0001");
  assert_eq!(
    recorded.len(),
    10,
    "the store's own stamp is still an ISO date: {recorded:?}"
  );
  assert_ne!(
    recorded, "2026-02-14",
    "the flag's value must not leak into a run that did not pass one"
  );
}

#[test]
fn a_date_that_is_not_a_day_is_refused_and_nothing_is_written() {
  let dir = project();
  closable(dir.path(), "ST0001", "A thread");

  // Shape-valid and not a date. A checker that only matched `YYYY-MM-DD` would
  // admit it, and canon would hold a value no reader can turn back into a day.
  let (out, err, code) = run(
    dir.path(),
    &["st", "done", "ST0001", "--date", "2026-02-30"],
  );

  assert_ne!(code, 0, "the thirtieth of February is not a day");
  assert!(
    out.is_empty(),
    "a refusal writes nothing to stdout: {out:?}"
  );
  assert!(
    err.contains("completed") && err.contains("2026-02-30"),
    "the refusal names the field and the value: {err:?}"
  );
  // **THE HALF THAT MATTERS: the refusal is not partial.** A validation that
  // ran after the transition would leave the thread closed with a null date.
  assert_eq!(completed(dir.path(), "ST0001"), "");
  let (shown, _, _) = run(dir.path(), &["st", "show", "ST0001"]);
  assert!(
    shown.contains("WIP") || shown.contains("wip"),
    "the thread must still be open: {shown:?}"
  );
}

#[test]
fn a_malformed_date_is_refused_with_a_remedy_about_the_value() {
  let dir = project();
  closable(dir.path(), "ST0001", "A thread");

  let (_, err, code) = run(
    dir.path(),
    &["st", "done", "ST0001", "--date", "14/02/2026"],
  );
  assert_ne!(code, 0);
  // **The remedy has to be about the VALUE.** Before `ValueNotRecordable`
  // existed this refusal borrowed the addressing error's remedy and told the
  // operator to `PUT json to a caller-assigned id`, which is nothing they can
  // do about a slash.
  assert!(
    err.contains("restate the value"),
    "the remedy must name what the operator can change: {err:?}"
  );
  assert!(
    !err.contains("PUT"),
    "the addressing remedy is the wrong one here: {err:?}"
  );
}
