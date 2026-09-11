//! **THE REMEDY `doctor` PRINTS FOR A SKEWED VIEW CLEARS THE FINDING** (issue
//! `0283`).
//!
//! The repro is the one Utilz hit: a thread realised while open, then closed,
//! so `.intentfiles` no longer lists it, then a criterion added to it. Before
//! 0283's half B the model moved on and the view on disk did not -- nobody
//! edited anything -- and estates still carry views left that way. `doctor`
//! used to call that a hand edit and name `intent sync --to-disk`, which writes
//! the extract and leaves the view stale, so following the remedy changed
//! nothing and the finding stood.
//!
//! **THE ASSERTION IS THE LOOP, NOT THE WORDING.** The command is read out of
//! doctor's own output and RUN, and doctor is asked again. A remedy is a claim
//! that a verb clears a finding; this makes the claim the thing under test.

use std::path::Path;
use std::process::Command;

fn intent(root: &Path, args: &[&str]) -> (bool, String) {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(root)
    .stdin(testkit::lifeline_for(args))
    .output()
    .unwrap_or_else(|e| panic!("could not run `intent {args:?}`: {e}"));
  (
    out.status.success(),
    String::from_utf8_lossy(&out.stdout).into_owned() + &String::from_utf8_lossy(&out.stderr),
  )
}

fn step(root: &Path, args: &[&str]) {
  let (ok, out) = intent(root, args);
  assert!(ok, "`intent {}` failed: {out}", args.join(" "));
}

#[test]
fn the_remedy_printed_for_a_closed_threads_stale_view_clears_the_finding() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  step(root, &["init", "probe"]);
  step(root, &["st", "new", "A thread closed early"]);
  step(root, &["st", "start", "ST0001"]);
  step(root, &["organize", "--apply"]);
  // The close gate wants a met contract, so one criterion is satisfied first.
  step(
    root,
    &[
      "ac",
      "new",
      "ST0001",
      "AC-00.1",
      "--text",
      "the first requirement",
    ],
  );
  step(
    root,
    &["ac", "satisfy", "ST0001", "AC-00.1", "--evidence", "done"],
  );
  step(root, &["st", "done", "ST0001"]);

  // **THE STALE VIEW AN EARLIER BUILD LEFT, RE-CREATED.** Since 0283's half B
  // a change to an unlisted thread refreshes a view that still holds the prior
  // render, so the change alone no longer leaves one behind. What an estate
  // can still carry is a view from BEFORE that fix -- an older render the
  // store has moved past. Put the pre-change bytes back after the change, and
  // that is exactly the state.
  let view = root.join("intent/st/ST0001/acceptance.md");
  let older_render = std::fs::read_to_string(&view).expect("the closed thread's view is on disk");
  step(
    root,
    &[
      "ac",
      "new",
      "ST0001",
      "AC-00.2",
      "--text",
      "added after the thread closed",
    ],
  );
  std::fs::write(&view, &older_render).expect("restore the older render");

  let (_, before) = intent(root, &["doctor", "--scope", "all"]);
  let line = before
    .lines()
    .find(|l| l.contains("st/ST0001/acceptance.md") && l.contains("differs from the model"))
    .unwrap_or_else(|| panic!("the stale view was not reported at all:\n{before}"));
  assert!(
    line.contains("the store changed after it was last rendered"),
    "the finding still offers only a hand edit as the cause: {line}"
  );
  assert!(
    !line.contains("sync --to-disk"),
    "the finding names `sync --to-disk`, which does not rewrite an unlisted thread's view: {line}"
  );

  // Read the command out of the finding, then run exactly that.
  let command = line
    .split('`')
    .find(|span| span.starts_with("intent "))
    .unwrap_or_else(|| panic!("the finding names no command: {line}"));
  assert_eq!(command, "intent st hydrate ST0001", "{line}");
  let args: Vec<&str> = command.split_whitespace().skip(1).collect();
  step(root, &args);

  let (_, after) = intent(root, &["doctor", "--scope", "all"]);
  assert!(
    !after.contains("st/ST0001/acceptance.md"),
    "running the printed remedy did not clear the finding:\n{after}"
  );
}
