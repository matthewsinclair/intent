//! `0189`: `intent edit issue 0056 --path` printed thread ST0056's `info.md`
//! at rc=0.
//!
//! **THE CALLER NAMED AN ISSUE, THE TOOL ANSWERED ABOUT A THREAD, AND REPORTED
//! SUCCESS.** `0149`'s class one step worse: `0149` discarded the kind and
//! REFUSED about a subject nobody named, where this discarded it and SUCCEEDED.
//! Under `--editor` rather than `--path` it puts an operator in the wrong file
//! with nothing saying so.
//!
//! **THE MECHANISM WAS A NAME, NOT A MISSING ARGUMENT** (vc). `render.rs`
//! probed `arg(m, "address")` -- which is `explore`'s argument name, where this
//! verb's is `id` -- so the probe always erred and always fell through to
//! `thread_arg`, the THREAD parser. The declared `address-or-id` type was
//! already on the right argument; the code asked for a different verb's.
//!
//! **IT SURVIVED BECAUSE THE FALLBACK IS RIGHT BY ACCIDENT ON EVERY COMMON
//! PATH**, which is why no test caught it and why the arms below are shaped the
//! way they are. `edit st ST0056` and `edit st 56` both work THROUGH the
//! defect. And because `THREAD_DIGITS == ISSUE_DIGITS == 4`, an issue spelling
//! produces a WELL-FORMED thread id rather than an error -- so the wrong answer
//! is visible only when an issue number is also a thread number, **which is 48
//! of 69 on this estate** (vc). Common and silent, not rare and loud.

use std::path::Path;
use std::process::Command;

fn bin() -> std::path::PathBuf {
  std::path::PathBuf::from(env!("CARGO_BIN_EXE_intent"))
}

/// A project with one thread and one issue **whose numbers collide** -- which
/// is the only configuration in which the defect is observable at all.
fn seeded() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  run(dir.path(), &["init", "kinds"]);
  run(dir.path(), &["st", "new", "a thread"]);
  run(dir.path(), &["issues", "add", "an issue"]);
  dir
}

fn run(cwd: &Path, args: &[&str]) -> (String, i32) {
  let out = Command::new(bin())
    .args(args)
    .current_dir(cwd)
    .env("HOME", testkit::fixture_home())
    .output()
    .expect("run intent");
  (
    format!(
      "{}{}",
      String::from_utf8_lossy(&out.stdout),
      String::from_utf8_lossy(&out.stderr)
    ),
    out.status.code().unwrap_or(-1),
  )
}

#[test]
fn naming_an_issue_does_not_answer_about_the_thread_of_the_same_number() {
  let dir = seeded();
  let (thread, rc) = run(dir.path(), &["edit", "st", "0001", "--path"]);
  assert_eq!(rc, 0, "the thread case must keep working: {thread}");
  assert!(
    thread.contains("ST0001"),
    "the fixture's thread did not resolve, so the comparison below proves nothing: {thread}"
  );

  let (issue, rc) = run(dir.path(), &["edit", "issue", "0001", "--path"]);
  // **THE ASSERTION IS THAT IT IS NOT THE THREAD'S ANSWER**, not that it is any
  // particular refusal. An issue has no realised form, so refusing is correct
  // here -- but the DEFECT was answering about something else, and that is what
  // must not come back whatever the refusal later says.
  assert!(
    !issue.contains("ST0001/info.md"),
    "naming `issue` returned the THREAD's file -- 0189, the wrong subject at rc={rc}: {issue}"
  );
}

#[test]
fn the_kind_vocabulary_the_table_declares_is_enforced() {
  // `intent edit banana ST0056 --path` printed a path at rc=0: the enum was
  // declared and nothing honoured it, the same gap `--format` carried until
  // `07ad9876`.
  let dir = seeded();
  let (out, rc) = run(dir.path(), &["edit", "banana", "0001", "--path"]);
  assert_ne!(rc, 0, "an undeclared kind was accepted: {out}");
  assert!(
    out.contains("st") && out.contains("issue"),
    "the refusal must name the vocabulary it is enforcing: {out}"
  );
}

#[test]
fn the_declared_address_or_id_type_accepts_an_address() {
  // The row's own note: "`address::promote` stays the one door". `thread_arg`
  // was not that door -- it splits on `/` and reads `intent:` as a thread id.
  //
  // **THE AUTHORITY IS EMPTY, SO THE CANONICAL FORM CARRIES THREE SLASHES.**
  // Measured 2026-08-31: the two-slash spelling appears ten times across this
  // estate's boards and the canonical one ONCE, because nothing emits it and
  // four parties each reconstructed the same wrong form from intuition.
  let dir = seeded();
  let (out, rc) = run(
    dir.path(),
    &["edit", "st", "intent:///threads/ST0001", "--path"],
  );
  assert_eq!(rc, 0, "the canonical address form was refused: {out}");
  assert!(
    out.contains("ST0001"),
    "the address resolved to the wrong thread: {out}"
  );
}

#[test]
fn a_kind_that_contradicts_the_address_is_refused_rather_than_resolved() {
  // Naming two entities in one invocation and picking either would be the
  // wrong-subject-silently shape this whole file exists to close.
  let dir = seeded();
  let (out, rc) = run(
    dir.path(),
    &["edit", "st", "intent:///issues/0001", "--path"],
  );
  assert_ne!(
    rc, 0,
    "a contradiction between kind and address was resolved rather than refused: {out}"
  );
  assert!(
    out.contains("issue"),
    "the refusal must name what the address actually carries: {out}"
  );
}
