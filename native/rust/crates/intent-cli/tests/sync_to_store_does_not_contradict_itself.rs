//! **A VERB MUST NOT PRINT BOTH ANSWERS TO THE QUESTION IT WAS ASKED.**
//!
//! `sync --to-store` printed `note: ... this restore overwrites nothing`
//! immediately followed by `ok: store replaced from the extract, <n> thread(s)`.
//! An operator checking whether a repair had run was told both, and **the
//! second is the one that reads as a result** -- so a run that changed nothing
//! reported itself as a replacement.
//!
//! Reproduced on a clean v3 fixture on 2026-08-28 while confirming what
//! survived of issue `0111` after its filed mechanism was withdrawn. It is the
//! part of that issue that holds under either account of the Lamplight
//! observation, which is why it is the part that got fixed.
//!
//! # This asserts the INVARIANT, not the wording
//!
//! Pinning the exact sentences would break on every rephrasing and would teach
//! the next author nothing. The property is that the two lines cannot
//! disagree: **if the run says it overwrote nothing, it must not also say the
//! store was replaced.** Any rewording that keeps that promise passes.
//!
//! # The control is the other arm, and without it this proves nothing
//!
//! An estate where the two sides genuinely differ must still produce the
//! warning AND the word `replaced` -- otherwise this file is satisfied by a
//! binary that stopped saying `replaced` at all, or by one whose `sync` stopped
//! working, and neither is the fix.

use std::path::Path;

fn run(cwd: &Path, args: &[&str]) -> (String, i32) {
  let out = crate::common::intent()
    .args(args)
    .current_dir(cwd)
    .env("HOME", testkit::fixture_home())
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run the v3 binary");
  (
    format!(
      "{}{}",
      String::from_utf8_lossy(&out.stdout),
      String::from_utf8_lossy(&out.stderr)
    ),
    out.status.code().unwrap_or(-1),
  )
}

/// An estate carrying one thread with one criterion and one acceptance test,
/// so the extract has a field worth disagreeing about. It answers the project
/// and the thread's id as its mint printed it.
fn seeded() -> (tempfile::TempDir, String) {
  let dir = tempfile::tempdir().expect("tempdir");
  run(dir.path(), &["init", "syncproj"]);
  let st = crate::common::created(&run(dir.path(), &["st", "new", "A thread"]).0);
  run(
    dir.path(),
    &["ac", "new", st.as_str(), "AC-01.1", "--text", "a criterion"],
  );
  run(
    dir.path(),
    &[
      "at",
      "new",
      st.as_str(),
      "AT-01.1",
      "--covers",
      "AC-01.1",
      "--file",
      "test/probe_test.exs",
    ],
  );
  (dir, st)
}

#[test]
fn a_run_that_overwrote_nothing_does_not_report_a_replacement() {
  let (dir, st) = seeded();
  let (out, rc) = run(dir.path(), &["sync", "--to-store", st.as_str()]);

  assert_eq!(rc, 0, "sync --to-store refused on a healthy estate: {out}");
  assert!(
    out.contains("overwrites nothing"),
    "the fixture was built so the store and the extract match, and the verb did not say so -- \
     this test is not exercising the branch it was written for: {out}"
  );
  assert!(
    !out.contains("store replaced from the canon extract"),
    "the run reported overwriting nothing AND replacing the store. Those are answers to the same \
     question and only one of them can be true -- and `replaced` is the one an operator reads as \
     the result: {out}"
  );
}

#[test]
fn a_run_that_did_overwrite_still_says_so() {
  let (dir, st) = seeded();

  // Make the extract genuinely differ from the store, which is the only state
  // in which `replaced` is a true word. The citation is the test's, so it is
  // in the test's own canon file (ST0096 WP-05).
  let extract = crate::common::canon_file(
    dir.path(),
    &intentsvcs::project::canon_at_rel(&st, "AT-01.1"),
  );
  let text = std::fs::read_to_string(&extract).expect("read the extract");
  assert!(
    text.contains("test/probe_test.exs"),
    "the fixture's citation is not in the extract, so the edit below would change nothing and \
     the control would pass vacuously"
  );
  std::fs::write(
    &extract,
    text.replace("test/probe_test.exs", "test/changed_test.exs"),
  )
  .expect("write the extract");

  let (out, rc) = run(dir.path(), &["sync", "--to-store", st.as_str()]);

  assert_eq!(rc, 0, "sync --to-store refused: {out}");
  assert!(
    out.contains("OVERWRITES")
      && out.contains(&format!("{}: differs on disk", crate::common::shown(&st))),
    "a genuine store/extract divergence was not reported -- the warning path is what makes the \
     other test's silence meaningful: {out}"
  );
  assert!(
    out.contains("replaced from the canon extract"),
    "the verb stopped saying `replaced` even when it did replace, which satisfies the sibling \
     test for the wrong reason: {out}"
  );
}

/// The `ok:` line of a run's output.
fn ok_line(out: &str) -> String {
  out
    .lines()
    .find(|l| l.starts_with("ok: "))
    .unwrap_or_else(|| panic!("the run printed no `ok:` line: {out}"))
    .to_string()
}

/// **0069: THE CONFIRMATION NAMES THE SCOPE IT OPERATED ON AND COUNTS WHAT
/// CHANGED, NOT WHAT EXISTS.** A run scoped to one thread said the STORE was
/// replaced; a run whose warning listed three ISSUES confirmed with a count of
/// THREADS, none of which had changed.
#[test]
fn the_confirmation_names_its_scope_and_counts_what_changed() {
  let (dir, st) = seeded();
  let issue = crate::common::created(&run(dir.path(), &["issues", "add", "An issue"]).0);

  // The citation is the test's, so it is in the test's own canon file
  // (ST0096 WP-05), not the thread's.
  let cited = crate::common::canon_file(
    dir.path(),
    &intentsvcs::project::canon_at_rel(&st, "AT-01.1"),
  );
  let text = std::fs::read_to_string(&cited).expect("read the test's extract");
  assert!(
    text.contains("test/probe_test.exs"),
    "the fixture's citation is not in the test's extract, so the edit below would change \
     nothing and this arm would pass vacuously"
  );
  std::fs::write(
    &cited,
    text.replace("test/probe_test.exs", "test/changed_test.exs"),
  )
  .expect("write the test's extract");
  let (out, rc) = run(dir.path(), &["sync", "--to-store", st.as_str()]);
  assert_eq!(rc, 0, "sync --to-store {st} refused: {out}");
  let ok = ok_line(&out);
  assert!(
    ok.contains(st.as_str()) && !ok.contains("store replaced"),
    "a run scoped to one thread confirms the whole store: {ok}"
  );

  let extract = dir
    .path()
    .join(format!("intent/.canon/issues/{issue}.json"));
  let text = std::fs::read_to_string(&extract).expect("read the issue's extract");
  assert!(
    text.contains("An issue"),
    "the issue's title is in its extract"
  );
  std::fs::write(&extract, text.replace("An issue", "An edited issue")).expect("write the issue");
  let (out, rc) = run(dir.path(), &["sync", "--to-store"]);
  assert_eq!(rc, 0, "sync --to-store refused: {out}");
  // The warning names the issue by its shown id, as every message does
  // (ST0095 WP-04; until then it dropped the leading zeros), so the expected
  // line is built from the minted id as the id waist reads it back.
  let number = intentsvcs::id::IssueId::parse(&issue).expect("a minted issue id reads back");
  assert!(
    out.contains(&format!("issue {}: differs on disk", number.shown())),
    "the fixture's one difference is the issue, or this arm tests nothing: {out}"
  );
  let ok = ok_line(&out);
  assert!(
    ok.contains("1 difference") && !ok.contains("thread(s)"),
    "the confirmation counts a population rather than the one difference it took: {ok}"
  );
}
