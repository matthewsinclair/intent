//! **`intent init` in a directory that is not a project but is not empty
//! either.**
//!
//! `init_from_empty_dir.rs` names the case it covers, and by naming it, names
//! the one nobody covered. The absence of `intent/.config/config.json` is what
//! made a directory NOT-A-PROJECT; it never made the directory EMPTY, and
//! those two claims were collapsed into one `exists()` test. Everything after
//! it was a bare `fs::write`, so a `CLAUDE.md`, an `AGENTS.md` or an
//! `intent/wip.md` that was already there was destroyed without a word --
//! which is hv's batch-4 class exactly: bytes removed or overwritten without
//! being named first.
//!
//! **THE FIXTURES CARRY RECOGNISABLE BYTES AND THE TEST READS THEM BACK.**
//! Asserting a refusal proves the tool said no; only re-reading the file
//! proves it did not write before saying it.

use std::path::Path;

fn run(args: &[&str], cwd: &Path) -> (String, String, i32) {
  let out = crate::common::intent()
    .args(args)
    .current_dir(cwd)
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run the v3 binary");
  (
    String::from_utf8_lossy(&out.stdout).into_owned(),
    String::from_utf8_lossy(&out.stderr).into_owned(),
    out.status.code().unwrap_or(-1),
  )
}

const MINE: &str = "# this file is the operator's, and init did not write it\n";

fn seed(dir: &Path, rel: &str) {
  let path = dir.join(rel);
  if let Some(parent) = path.parent() {
    std::fs::create_dir_all(parent).expect("make the fixture's parent");
  }
  std::fs::write(&path, MINE).expect("seed the fixture");
}

fn unchanged(dir: &Path, rel: &str) {
  let body = std::fs::read_to_string(dir.join(rel))
    .unwrap_or_else(|e| panic!("{rel} is gone after a refusal that claimed to write nothing: {e}"));
  assert_eq!(body, MINE, "{rel} was overwritten by a run that refused");
}

/// The five destinations vc's item names, each on its own, because a check
/// that fires on one of them tells you nothing about the other four.
#[test]
fn init_refuses_over_each_file_it_would_have_overwritten() {
  for rel in [
    "CLAUDE.md",
    "AGENTS.md",
    "intent/wip.md",
    "intent/llm/RULES.md",
    "intent/llm/ARCHITECTURE.md",
  ] {
    let dir = tempfile::tempdir().expect("tempdir");
    seed(dir.path(), rel);

    let (out, err, code) = run(&["init", "fixture-project"], dir.path());
    assert_ne!(code, 0, "init overwrote {rel} and reported success: {out}");
    assert!(
      err.contains(rel),
      "the refusal does not name {rel}, so the operator cannot act on it: {err}"
    );
    unchanged(dir.path(), rel);

    // AND IT CREATED NO PROJECT. A refusal that left a config behind would
    // make the directory a project whose starter content was never written.
    assert!(
      !dir.path().join("intent/.config/config.json").exists(),
      "the refusal over {rel} still created a project"
    );
  }
}

/// **EVERY COLLISION IS NAMED AT ONCE, NOT THE FIRST ONE.** An operator told
/// about one file moves it, re-runs, and loses the second -- learning about it
/// only afterwards, which is the same defect one directory further on.
#[test]
fn the_refusal_names_every_collision_not_just_the_first() {
  let dir = tempfile::tempdir().expect("tempdir");
  seed(dir.path(), "CLAUDE.md");
  seed(dir.path(), "AGENTS.md");
  seed(dir.path(), "intent/wip.md");

  let (_, err, code) = run(&["init", "fixture-project"], dir.path());
  assert_ne!(code, 0, "init proceeded over three of its own destinations");
  for rel in ["CLAUDE.md", "AGENTS.md", "intent/wip.md"] {
    assert!(
      err.contains(rel),
      "the refusal names some collisions and not {rel}: {err}"
    );
    unchanged(dir.path(), rel);
  }
}

/// **A REFUSAL IS A 1, NOT A 2.** `init`'s own precedent: `AlreadyAProject`
/// was moved off `2` because a verb declining a job is not a build unable to
/// answer, and this is the same kind of no.
#[test]
fn the_refusal_takes_the_code_a_refusal_takes() {
  let dir = tempfile::tempdir().expect("tempdir");
  seed(dir.path(), "CLAUDE.md");
  let (_, _, code) = run(&["init", "fixture-project"], dir.path());
  assert_eq!(code, 1, "a refusal did not take the refusal's exit code");
}

/// **THE CONTROL, AND IT IS THE HALF THAT KEEPS THE FIX HONEST.** A check that
/// refused whenever the directory held anything would be easy and wrong:
/// `init` is run in populated directories constantly -- a README, a `src/`, a
/// `.git/`. Only files `init` itself writes may block it.
#[test]
fn a_file_init_does_not_write_does_not_block_it() {
  let dir = tempfile::tempdir().expect("tempdir");
  seed(dir.path(), "README.md");
  seed(dir.path(), "src/main.rs");

  let (out, err, code) = run(&["init", "fixture-project"], dir.path());
  assert_eq!(
    code, 0,
    "init refused over files it does not write: {err}{out}"
  );
  unchanged(dir.path(), "README.md");
  unchanged(dir.path(), "src/main.rs");
  assert!(
    dir.path().join("intent/.config/config.json").is_file(),
    "init reported success and created no project"
  );
}

/// **`.prettierignore` IS THE DELIBERATE EXCEPTION, AND IT IS A CONTROL TOO.**
/// Its writer appends what is missing and leaves everything already there
/// alone, so it is not at risk -- and refusing over a file most repositories
/// carry would make `init` unusable in exactly the populated directory this
/// check exists to protect.
#[test]
fn an_existing_prettierignore_is_kept_and_does_not_block_init() {
  let dir = tempfile::tempdir().expect("tempdir");
  std::fs::write(dir.path().join(".prettierignore"), "node_modules/\n")
    .expect("seed .prettierignore");

  let (out, err, code) = run(&["init", "fixture-project"], dir.path());
  assert_eq!(
    code, 0,
    "an existing .prettierignore blocked init: {err}{out}"
  );

  let body = std::fs::read_to_string(dir.path().join(".prettierignore")).expect("read it back");
  assert!(
    body.contains("node_modules/"),
    "the operator's own pattern was discarded: {body}"
  );
}
