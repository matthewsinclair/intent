//! **A CLI WRITE NEVER RENDERS A THREAD'S COVER OVER A HAND EDIT THE STORE CAN
//! CARRY** (issue `0559`).
//!
//! The README's route: `st new`, `st edit` to type an Objective into the cover,
//! then `ac new`. With no intentd running the `ac new` rendered the cover over
//! the Objective and said so in a warning whose remedy was git, and in a fresh
//! project nothing had been committed. It now refuses before it writes, and the
//! command it names is read out of its own output and RUN, then the verb is run
//! again: the remedy is the thing under test, not its wording.

use std::path::Path;

fn intent(root: &Path, args: &[&str]) -> (bool, String) {
  let out = crate::common::intent()
    .args(args)
    .current_dir(root)
    .env("EDITOR", "true")
    .env("VISUAL", "true")
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

/// Replace the cover's Objective with `text`, as a person in an editor would.
fn type_an_objective(root: &Path, text: &str) {
  let cover = root.join("intent/st/ST0001/info.md");
  let was = std::fs::read_to_string(&cover).expect("`st edit` realised the cover");
  let from = was.find("## Objective\n\n").expect("an Objective") + "## Objective\n\n".len();
  let to = was.find("## Context\n\n").expect("a Context");
  std::fs::write(&cover, format!("{}{text}\n\n{}", &was[..from], &was[to..])).expect("the edit");
}

#[test]
fn a_write_refuses_over_a_carriable_cover_edit_and_its_remedy_keeps_the_edit() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  step(root, &["init", "probe"]);
  step(root, &["st", "new", "A thread with a reason to exist"]);
  step(root, &["st", "edit", "ST0001"]);
  type_an_objective(root, "Typed by hand into the cover.");

  let args = ["ac", "new", "ST0001", "AC-01.1", "--text", "a criterion"];
  let (ok, refused) = intent(root, &args);
  assert!(!ok, "the write went ahead over the edit:\n{refused}");
  assert!(
    refused.contains("intent/st/ST0001/info.md"),
    "the refusal names the cover:\n{refused}"
  );
  let command = refused
    .split('`')
    .find(|span| span.starts_with("intent sync --to-store"))
    .unwrap_or_else(|| panic!("the refusal names no carrying verb:\n{refused}"));
  assert_eq!(command, "intent sync --to-store ST0001");

  let remedy: Vec<&str> = command.split_whitespace().skip(1).collect();
  step(root, &remedy);
  step(root, &args);

  let (_, shown) = intent(root, &["st", "show", "ST0001"]);
  assert!(
    shown.contains("Typed by hand into the cover."),
    "the edit did not survive the remedy and the re-run:\n{shown}"
  );
  let cover = std::fs::read_to_string(root.join("intent/st/ST0001/info.md")).expect("the cover");
  assert!(cover.contains("Typed by hand into the cover."));
}

/// The limit, held so a later reading cannot claim more: an edit the cover
/// cannot carry -- a heading added by hand -- is not refused. The write goes
/// ahead and names what it overwrote, as before.
#[test]
fn an_edit_the_cover_cannot_carry_is_overwritten_with_the_warning() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  step(root, &["init", "probe"]);
  step(root, &["st", "new", "A thread with a hand-added section"]);
  step(root, &["st", "edit", "ST0001"]);
  let cover = root.join("intent/st/ST0001/info.md");
  let was = std::fs::read_to_string(&cover).expect("the cover");
  let at = was.find("## Context\n\n").expect("a Context");
  std::fs::write(
    &cover,
    format!("{}## Scope\n\nAdded by hand.\n\n{}", &was[..at], &was[at..]),
  )
  .expect("the edit");

  let (ok, out) = intent(
    root,
    &["ac", "new", "ST0001", "AC-01.1", "--text", "a criterion"],
  );
  assert!(ok, "an edit the cover cannot carry is not refused:\n{out}");
  assert!(
    out.contains("overwrote bytes that were not the store's render"),
    "the overwrite is named:\n{out}"
  );
}
