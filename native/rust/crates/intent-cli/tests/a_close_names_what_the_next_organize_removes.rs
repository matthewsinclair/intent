//! **A CLOSE THAT ARMS A REMOVAL NAMES WHAT IS ARMED** (batch 4, hv's
//! silent-deletion gate, 2026-09-12).
//!
//! `st cancel`, `st done` and `st hold` unlist the thread, and an unlisted
//! thread's files are what the next `intent organize --apply` removes. The verb
//! said `ok: <id> <moved>` and named nothing; the only warning it could give
//! covered attachments carrying uncommitted bytes, which is the sharpest case
//! and a strict subset -- **a committed file is still a file that disappears
//! from the operator's working tree without having been named.**
//!
//! **SAID BEFORE, NOT AFTER, and that is the same argument `closing_notes`
//! already makes** (AC-03.9): the close itself removes nothing, so the moment
//! to say what it arms is while the operator is still holding the decision.
//!
//! **RECOVERABILITY IS WHY THIS IS A NOTE AND NOT A REFUSAL.** These files are
//! projections: `intent st hydrate <id>` writes them back from the store, and
//! git has the committed ones. What is NOT recoverable is an attachment whose
//! bytes reached no commit, which is why that warning stays in its own words
//! rather than being folded into this list.

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

/// Two threads, one realised and closed, the other realised and left alone --
/// so the case can assert that the note names THIS thread's files and not the
/// estate's. It answers the project, the id of the thread that closes and the
/// id of the thread that stays, each as its mint printed it.
fn estate() -> (tempfile::TempDir, String, String) {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  let (_, err, code) = run(&["init", "dehydration-fixture"], root);
  assert_eq!(code, 0, "fixture init failed: {err}");
  let mint = |title: &str| {
    let (out, err, code) = run(&["st", "new", title], root);
    assert_eq!(code, 0, "fixture st new failed: {err}");
    crate::common::created(&out)
  };
  let closes = mint("The thread that closes");
  let stays = mint("The thread that stays");
  for id in [closes.as_str(), stays.as_str()] {
    let (_, err, code) = run(&["edit", "st", id, "--path"], root);
    assert_eq!(code, 0, "fixture realise failed for {id}: {err}");
  }
  (dir, closes, stays)
}

#[test]
fn a_close_names_every_file_the_next_organize_would_remove() {
  let (dir, closes, stays) = estate();
  let root = dir.path();
  let armed = [
    format!("intent/st/{closes}/info.md"),
    format!("intent/st/{closes}/acceptance.md"),
  ];

  // THE CONTROL: the files exist and are the ones the claim is about. A case
  // that ran against an unrealised thread would assert over an empty set and
  // pass by naming nothing.
  for rel in &armed {
    assert!(
      root.join(rel).exists(),
      "the fixture never realised {rel}, so this case has nothing to name"
    );
  }

  let (out, err, code) = run(
    &["st", "cancel", closes.as_str(), "--reason", "a probe"],
    root,
  );
  assert_eq!(code, 0, "the close itself failed: {err}");
  assert!(
    out.contains(&format!("ok: {closes} cancelled")),
    "the result line moved or changed shape: {out:?}"
  );

  for rel in &armed {
    assert!(
      err.contains(rel.as_str()),
      "closing the thread arms {rel} for removal and the verb did not name it: {err:?}"
    );
  }
  assert!(
    err.contains("organize"),
    "the note does not say what will do the removing: {err:?}"
  );
  // The OTHER thread stays listed, so nothing of its is armed. A note that
  // named the whole estate would be true of nothing and read as a catastrophe.
  assert!(
    !err.contains(stays.as_str()),
    "the note named a thread this close did not unlist: {err:?}"
  );
}

/// **`--keep` CLOSES WITHOUT UNLISTING, so no removal is coming and there is
/// nothing to arm.** Keying the note on the verb rather than on the removal
/// would warn here anyway -- correct-looking, and a warning about a consequence
/// that is not coming is how an operator learns to skim the ones that are.
#[test]
fn a_close_that_keeps_the_thread_listed_arms_nothing_and_says_nothing() {
  let (dir, closes, _) = estate();
  let root = dir.path();
  let (_, err, code) = run(
    &[
      "st",
      "cancel",
      closes.as_str(),
      "--reason",
      "a probe",
      "--keep",
    ],
    root,
  );
  assert_eq!(code, 0, "the close itself failed: {err}");
  assert!(
    !err.contains(&format!("intent/st/{closes}/info.md")),
    "a close that unlisted nothing still warned about a removal: {err:?}"
  );
}
