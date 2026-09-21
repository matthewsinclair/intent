//! **`uninstall` counted the files it DESTROYED and named the files it KEPT.**
//!
//! The two halves of one line, as it stood:
//!
//! ```text
//!   in-debug  removed (<n> file(s)); left 1 this build did not install: NOTES.md
//! ```
//!
//! `NOTES.md` survived and is named. The files that are gone are a number.
//! So the only part of the report an operator can act on describes the part
//! that needs no action, and the part that is unrecoverable -- which is the
//! part they would want to check against, or copy back from a backup -- is
//! withheld. hv's batch-4 class is bytes removed without being named; this is
//! the reporting half of it, and the information was in hand the whole time.
//!
//! **THE PRECEDENT IS IN THE SAME `match`.** `Outcome::Updated` already prints
//! `retired: <names>` for the files an update drops. Two arms of one expression
//! answered the same question two ways.

use std::path::Path;

fn run(home: &Path, args: &[&str]) -> (i32, String) {
  let out = crate::common::intent()
    .args(args)
    .env("HOME", home)
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("spawn intent");
  let mut text = String::from_utf8_lossy(&out.stdout).to_string();
  text.push_str(&String::from_utf8_lossy(&out.stderr));
  (out.status.code().unwrap_or(-1), text)
}

/// A skill removal names the file it deleted.
#[test]
fn removing_a_skill_names_the_files_it_deleted() {
  let home = tempfile::tempdir().expect("tempdir");
  let (code, text) = run(home.path(), &["claude", "skills", "install", "in-debug"]);
  assert_eq!(code, 0, "could not install the fixture skill: {text}");
  assert!(
    home
      .path()
      .join(".claude/skills/in-debug/SKILL.md")
      .is_file(),
    "the fixture skill is not on disk"
  );

  let (code, text) = run(home.path(), &["claude", "skills", "uninstall", "in-debug"]);
  assert_eq!(code, 0, "uninstall refused: {text}");
  assert!(
    text.contains("SKILL.md"),
    "the removal does not name the file it deleted: {text}"
  );
  assert!(
    !home
      .path()
      .join(".claude/skills/in-debug/SKILL.md")
      .exists(),
    "the test proves nothing: the file is still there"
  );
}

/// **THE SAME RULE OVER THE OTHER PAYLOAD KIND**, because a rule that reached
/// one family and not the other is what item 3 of this batch was about.
#[test]
fn removing_a_subagent_names_the_file_it_deleted() {
  let home = tempfile::tempdir().expect("tempdir");
  let (code, text) = run(
    home.path(),
    &["claude", "subagents", "install", "critic-shell"],
  );
  assert_eq!(code, 0, "could not install the fixture subagent: {text}");

  let (code, text) = run(
    home.path(),
    &["claude", "subagents", "uninstall", "critic-shell"],
  );
  assert_eq!(code, 0, "uninstall refused: {text}");
  assert!(
    text.contains("critic-shell.md"),
    "the removal does not name the file it deleted: {text}"
  );
}

/// **THE KEPT FILE IS STILL NAMED.** The fix is to name BOTH halves, not to
/// trade one for the other -- a file this build did not write is left behind
/// and stays loadable, and an operator who wants it gone needs its path.
#[test]
fn a_file_this_build_did_not_write_is_still_named_and_still_kept() {
  let home = tempfile::tempdir().expect("tempdir");
  let (code, text) = run(home.path(), &["claude", "skills", "install", "in-debug"]);
  assert_eq!(code, 0, "install failed: {text}");

  let stray = home.path().join(".claude/skills/in-debug/NOTES.md");
  std::fs::write(&stray, "the operator's own notes\n").expect("plant a stray file");

  let (_, text) = run(home.path(), &["claude", "skills", "uninstall", "in-debug"]);
  assert!(
    text.contains("NOTES.md"),
    "the file it kept is no longer named: {text}"
  );
  assert!(
    text.contains("SKILL.md"),
    "the file it deleted is still not named: {text}"
  );
  assert!(
    stray.exists(),
    "a file this build did not write was deleted"
  );
}
