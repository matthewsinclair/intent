//! **A completion that already happened can be recorded as such** (hv,
//! 2026-08-28, on issue 0118's fifth limb).
//!
//! `st done` and `st cancel` stamped TODAY and took no override, so the only
//! open CLI path recorded threads finished in February as finished on the day
//! the estate migrated. On Conflab that is nearly every remaining doctor
//! finding.
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
//! conflab-vc recovered completion dates from git: every one read as the local
//! day, while UTC got the commit made at 23:57 +0100 wrong. So the flag records
//! the date the caller states and converts nothing. **The whiteboard's
//! `date -u` discipline is the right rule in the wrong domain here** -- it
//! governs ordering stamps, and a completion date is a calendar fact.

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

/// A thread that CLOSED and records no date -- Conflab's fifty, and the case
/// the self-loop hid. Written as canon, because no verb produces it: a close
/// through the CLI always leaves a date behind.
fn closed_without_a_date(dir: &Path, id: &str) {
  let canon = dir.join("intent/.canon/st");
  std::fs::create_dir_all(&canon).expect("mkdir");
  std::fs::write(
    canon.join(format!("{id}.json")),
    format!(
      r#"{{
  "schema": "intent/thread@3.0",
  "id": "{id}",
  "slug": "a-closed-thread",
  "title": "A thread closed with no date",
  "status": "completed",
  "created": "2026-02-01",
  "objective": "",
  "context": "",
  "wps": [],
  "criteria": []
}}
"#
    ),
  )
  .expect("write canon");
}

/// Issue 0503: **A DATE THAT DIFFERS FROM THE ONE ON RECORD IS A RESTATEMENT,
/// AND THE CLOSING VERB IS NOT THAT DOOR.** It answered `ok:` at rc 0 and wrote
/// nothing, so the operator was told the date had been taken.
#[test]
fn a_date_differing_from_the_record_is_refused_and_names_the_door_that_restates_it() {
  let dir = project();
  closable(dir.path(), "ST0001", "A thread finished in February");
  let (_, err, code) = run(
    dir.path(),
    &["st", "done", "ST0001", "--date", "2026-02-14"],
  );
  assert_eq!(code, 0, "{err}");

  let (out, err, code) = run(
    dir.path(),
    &["st", "done", "ST0001", "--date", "2026-01-15"],
  );
  assert_ne!(code, 0, "a date that differs must not pass as ok: {out:?}");
  assert!(
    err.contains("already Completed") && err.contains("2026-02-14"),
    "the refusal names what is on record: {err:?}"
  );
  assert!(
    err.contains("intent set ST0001 completed 2026-01-15"),
    "the remedy names the door that restates it, with the date given: {err:?}"
  );
  assert_eq!(
    completed(dir.path(), "ST0001"),
    "2026-02-14",
    "the refusal wrote nothing"
  );
}

/// Issue 0503, the case Conflab has fifty of: **a thread that closed and
/// records NO date says so**, rather than reporting `ok:` and staying empty.
#[test]
fn a_closed_thread_with_no_recorded_date_says_so_in_the_refusal() {
  let dir = project();
  closed_without_a_date(dir.path(), "ST0001");

  let (out, err, code) = run(
    dir.path(),
    &["st", "done", "ST0001", "--date", "2026-01-15"],
  );
  assert_ne!(code, 0, "{out:?}");
  assert!(
    err.contains("records no completion date"),
    "the refusal names the empty record: {err:?}"
  );
  assert!(
    err.contains("intent set ST0001 completed 2026-01-15"),
    "the remedy is the door that writes it: {err:?}"
  );
  assert_eq!(completed(dir.path(), "ST0001"), "");
}

/// Issue 0503, **A CONTROL RATHER THAN A DEFECT ARM, and green on the base as
/// well as on the fix**: the same date again is still nothing to do. The
/// self-loop keeps its meaning, and its `ok:` now means the record already says
/// this. A fix that refused every `--date` on a closed thread would satisfy the
/// three arms above and break this one.
#[test]
fn the_date_already_on_record_is_still_an_ok_self_loop() {
  let dir = project();
  closable(dir.path(), "ST0001", "A thread finished in February");
  run(
    dir.path(),
    &["st", "done", "ST0001", "--date", "2026-02-14"],
  );

  let (out, err, code) = run(
    dir.path(),
    &["st", "done", "ST0001", "--date", "2026-02-14"],
  );
  assert_eq!(code, 0, "{err}");
  assert!(out.contains("already Completed"), "{out:?}");
  assert_eq!(completed(dir.path(), "ST0001"), "2026-02-14");
}

/// Issue 0503: **a malformed date is refused on a closed thread exactly as it
/// is on the close.** It reached the self-loop's `ok:` and was dropped, which
/// is the same value landing in canon one call earlier would have been refused.
#[test]
fn a_malformed_date_on_a_closed_thread_is_refused_as_a_close_refuses_it() {
  let dir = project();
  closable(dir.path(), "ST0001", "A thread finished in February");
  run(
    dir.path(),
    &["st", "done", "ST0001", "--date", "2026-02-14"],
  );

  let (out, err, code) = run(
    dir.path(),
    &["st", "done", "ST0001", "--date", "2026-02-30"],
  );
  assert_ne!(code, 0, "{out:?}");
  assert!(
    err.contains("completed") && err.contains("2026-02-30"),
    "the refusal names the field and the value, as the close's does: {err:?}"
  );
  assert_eq!(completed(dir.path(), "ST0001"), "2026-02-14");
}

/// Issue 0504: **`intent set` refuses through its door what `st done --date`
/// refuses through its own.** The setter re-parsed `completed` as the string
/// the model declares, so any string passed.
#[test]
fn set_refuses_a_completed_value_that_is_not_a_date() {
  let dir = project();
  closable(dir.path(), "ST0001", "A thread finished in February");
  run(
    dir.path(),
    &["st", "done", "ST0001", "--date", "2026-02-14"],
  );

  let (out, err, code) = run(dir.path(), &["set", "ST0001", "completed", "not-a-date"]);
  assert_ne!(code, 0, "{out:?}");
  assert!(
    err.contains("completed") && err.contains("not-a-date"),
    "the refusal names the field and the value: {err:?}"
  );
  assert_eq!(
    completed(dir.path(), "ST0001"),
    "2026-02-14",
    "the refusal wrote nothing"
  );
}

/// Issue 0504: **a completion date on a thread that closed nothing is a claim
/// the model cannot support**, and this door recorded it at rc 0.
#[test]
fn set_refuses_a_completion_date_on_a_thread_that_is_not_closed() {
  let dir = project();
  run(dir.path(), &["st", "new", "A thread in triage"]);

  let (out, err, code) = run(dir.path(), &["set", "ST0001", "completed", "2026-03-03"]);
  assert_ne!(code, 0, "{out:?}");
  assert!(
    err.contains("records no completion date"),
    "the refusal names the status rule: {err:?}"
  );
  assert_eq!(completed(dir.path(), "ST0001"), "");
}

/// **THE CONTROL FOR BOTH REFUSALS ABOVE, and the workaround the triage's C6
/// row depends on**: a closed thread's date is still restated through this
/// door. A fix that refused every `set completed` would satisfy both tests
/// above and take the remedy with it.
#[test]
fn set_still_restates_a_completion_date_on_a_closed_thread() {
  let dir = project();
  closable(dir.path(), "ST0001", "A thread finished in February");
  run(
    dir.path(),
    &["st", "done", "ST0001", "--date", "2026-02-14"],
  );

  let (_, err, code) = run(dir.path(), &["set", "ST0001", "completed", "2026-01-15"]);
  assert_eq!(code, 0, "{err}");
  assert_eq!(completed(dir.path(), "ST0001"), "2026-01-15");
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
