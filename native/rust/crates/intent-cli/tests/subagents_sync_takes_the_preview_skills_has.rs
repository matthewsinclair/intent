//! **`claude subagents` declared no `--dry-run`, so the preview its own engine
//! already implements was unreachable.**
//!
//! `payload.rs` has one preview and both families reach it through one renderer
//! function -- `w.verb` is the only thing that differs. `claude skills` carries
//! the `--dry-run` row; `claude subagents` did not, and `spine.rs` declares
//! table flags onto clap, so the flag that exists in the engine was refused at
//! the parser with `unexpected argument` at rc=1. An operator who wanted to see
//! what a sync would do to a subagent they had edited had exactly one way to
//! find out, which was to let it happen.
//!
//! **THAT MATTERS MORE SINCE BATCH 2.** `sync` and `uninstall` now HOLD a unit
//! edited since it was installed, and `--force` discards it and names the
//! checksum. The preview is how an operator decides whether to type `--force`,
//! and it was available for skills and not for subagents -- the same decision,
//! the same risk, one of them blind.
//!
//! And `--force`'s own help said it overwrites *an agent manifest*. It
//! overwrites the subagent's `.md`, which is the thing an operator edits.

use std::path::Path;
use std::process::Command;

fn run(home: &Path, args: &[&str]) -> (i32, String) {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .env("HOME", home)
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("spawn intent");
  let mut text = String::from_utf8_lossy(&out.stdout).to_string();
  text.push_str(&String::from_utf8_lossy(&out.stderr));
  (out.status.code().unwrap_or(-1), text)
}

/// A subagent this install carries, installed into an isolated HOME and then
/// edited, so a sync has something real to report.
fn an_edited_subagent(home: &Path) -> std::path::PathBuf {
  let (code, text) = run(home, &["claude", "subagents", "install", "critic-shell"]);
  assert_eq!(code, 0, "could not install the fixture subagent: {text}");
  let path = home.join(".claude/agents/critic-shell.md");
  let before = std::fs::read_to_string(&path).expect("the installed subagent is not on disk");
  std::fs::write(
    &path,
    format!("{before}\n<!-- the operator's own line -->\n"),
  )
  .expect("edit the subagent");
  path
}

/// **THE DEFECT.** The flag the engine implements reaches the operator, and the
/// preview is exactly the thing an operator needs before deciding about
/// `--force`: it names the hold, and it writes nothing.
///
/// **THE EXIT IS 1 AND THAT IS THE RIGHT ANSWER, NOT A LEFTOVER REFUSAL.** The
/// fixture edits the subagent, so batch 2's hold fires and a unit needs a
/// decision -- which is the undecided exit every other undecided state takes.
/// Softening it under `--dry-run` would make the preview disagree with the run
/// it is previewing, which is the one thing a preview may never do. The arm
/// that proves the flag is READ is the absence of clap's parser refusal, and it
/// is asserted rather than inferred from the code.
#[test]
fn subagents_sync_takes_dry_run_and_writes_nothing() {
  let home = tempfile::tempdir().expect("tempdir");
  let path = an_edited_subagent(home.path());
  let edited = std::fs::read_to_string(&path).expect("read the edit back");

  let (code, text) = run(home.path(), &["claude", "subagents", "sync", "--dry-run"]);
  assert!(
    !text.contains("unexpected argument"),
    "the flag is still refused at the parser: {text}"
  );
  assert!(
    text.contains("dry run: nothing below is written"),
    "the preview does not say it is a preview: {text}"
  );
  assert!(
    text.contains("would change"),
    "a preview must say what WOULD change, not what did: {text}"
  );
  assert!(
    text.contains("HELD"),
    "the preview does not name the hold the operator is deciding about: {text}"
  );
  assert_eq!(
    code, 1,
    "a held unit must still report that a decision is needed: {text}"
  );
  assert_eq!(
    std::fs::read_to_string(&path).expect("read it back"),
    edited,
    "--dry-run wrote to the subagent it was previewing"
  );
}

/// **AND WITH NOTHING TO DECIDE IT IS A 0**, so the arm above is pinning the
/// hold rather than a preview that always fails.
#[test]
fn a_preview_with_nothing_held_reports_success() {
  let home = tempfile::tempdir().expect("tempdir");
  let (code, text) = run(home.path(), &["claude", "subagents", "sync", "--dry-run"]);
  assert_eq!(
    code, 0,
    "a preview with nothing installed was not a 0: {text}"
  );
}

/// **THE CONTROL THAT MAKES THE ARM ABOVE MEAN SOMETHING.** `claude skills`
/// already had the flag, so if the harness were wrong this would fail too --
/// and the asymmetry between the two families is the whole finding.
#[test]
fn skills_sync_already_took_the_flag_and_still_does() {
  let home = tempfile::tempdir().expect("tempdir");
  let (code, text) = run(home.path(), &["claude", "skills", "sync", "--dry-run"]);
  assert_eq!(code, 0, "`skills sync --dry-run` was refused: {text}");
}

/// **REFUSED WHERE IT CANNOT BE HONOURED, exactly as it is for skills.** A
/// `--dry-run` that silently performs the write is the worst failure available
/// to this flag: the operator typed it to avoid writing, and a successful-looking
/// run then confirms the opposite of what happened.
#[test]
fn subagents_install_refuses_dry_run_rather_than_writing_under_it() {
  let home = tempfile::tempdir().expect("tempdir");
  let (code, text) = run(
    home.path(),
    &[
      "claude",
      "subagents",
      "install",
      "critic-shell",
      "--dry-run",
    ],
  );
  assert_ne!(code, 0, "`install --dry-run` performed a write: {text}");
  assert!(
    text.contains("--dry-run") && text.contains("sync"),
    "the refusal must name the flag and where it works: {text}"
  );
  assert!(
    !home.path().join(".claude/agents/critic-shell.md").exists(),
    "the refusal still installed the subagent"
  );
}

/// **THE HELP NAMES WHAT THE FLAG OVERWRITES.** It said *an agent manifest*,
/// which is not the file at risk -- the operator edits the subagent's `.md`,
/// and since batch 2 `--force` is what discards that edit. A flag whose help
/// does not name what it destroys is how batch 2's defect read as intended
/// behaviour for so long.
#[test]
fn the_force_help_names_the_file_it_discards() {
  let home = tempfile::tempdir().expect("tempdir");
  let (code, text) = run(home.path(), &["claude", "subagents", "--help"]);
  assert_eq!(code, 0, "--help failed: {text}");
  assert!(
    !text.contains("agent manifest"),
    "the help still calls the subagent's own file a manifest: {text}"
  );
  assert!(
    text.contains("changed here"),
    "the help does not say the flag overwrites a subagent changed here: {text}"
  );
}
