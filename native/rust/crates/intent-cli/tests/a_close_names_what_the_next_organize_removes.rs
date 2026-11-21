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

/// Close `st` with `st done` over a met contract, as a person does: started,
/// realised by `organize --apply`, one criterion satisfied. It answers the
/// subject the verb names, read off its `ok:` line, and the stderr lines.
fn done_over_a_met_contract(root: &Path, st: &str) -> (String, Vec<String>) {
  for args in [
    vec!["st", "start", st],
    vec!["organize", "--apply"],
    vec![
      "ac",
      "new",
      st,
      "AC-00.1",
      "--text",
      "the first requirement",
    ],
    vec!["ac", "satisfy", st, "AC-00.1", "--evidence", "done"],
  ] {
    let (_, err, code) = run(&args, root);
    assert_eq!(code, 0, "fixture step {args:?} failed: {err}");
  }
  let (out, err, code) = run(&["st", "done", st], root);
  assert_eq!(code, 0, "the close itself failed: {err}");
  let subject = out
    .lines()
    .find_map(|line| line.strip_prefix("ok: ")?.strip_suffix(" done"))
    .unwrap_or_else(|| panic!("no `ok: <id> done` line: {out:?}"))
    .to_string();
  (subject, err.lines().map(str::to_string).collect())
}

/// The close note's whole run of lines: its header, each path it names, and
/// its remedy, which must follow the paths with nothing between.
fn note_lines(err: &[String], header: &str, remedy: &str) {
  let at = err
    .iter()
    .position(|line| line == header)
    .unwrap_or_else(|| panic!("no note header {header:?} in: {err:#?}"));
  let paths: Vec<&String> = err[at + 1..]
    .iter()
    .take_while(|line| !line.starts_with("  remedy:"))
    .collect();
  assert!(
    !paths.is_empty(),
    "the note names no file, so it arms nothing: {err:#?}"
  );
  for path in &paths {
    assert!(
      path.starts_with("  intent/st/"),
      "a line between the note's header and its remedy is not a thread file: {path:?}"
    );
  }
  assert_eq!(
    err.get(at + 1 + paths.len()).map(String::as_str),
    Some(remedy),
    "the note's remedy line: {err:#?}"
  );
}

/// **3.4.0'S NOTE, BYTE FOR BYTE, IN A PROJECT NOT YET UPGRADED** (todo 78,
/// critic-rust on 630f7d0f8): the close unlists the thread from the manifest,
/// and `--keep` closes without unlisting. The words come from
/// `RealisationWords`, and this reads them in the sentence the verb prints.
#[test]
fn a_close_in_the_manifest_format_prints_3_4_0s_note() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  let (_, err, code) = run(&["init", "ordinal-close"], root);
  assert_eq!(code, 0, "fixture init failed: {err}");
  crate::common::born_ordinal(root);
  crate::common::seed_init_manifest(root);
  let st = intentsvcs::id::ThreadId::from_ordinal(1)
    .expect("an ordinal thread")
    .stored();
  crate::common::seed_ordinal_thread(root, &st, "An ordinal thread", &[]);

  let (subject, err) = done_over_a_met_contract(root, &st);
  note_lines(
    &err,
    &format!("note: this unlists {subject}, so the next `intent organize --apply` will remove:"),
    &format!(
      "  remedy: `intent st hydrate {subject}` writes them back from the store, and `st done --keep` or `st cancel --keep` closes without unlisting"
    ),
  );
}

/// An upgraded project's note says what the close does to the record: it
/// unrealises it, and `--keep` leaves it realised. No manifest word survives.
#[test]
fn a_close_in_an_upgraded_project_says_it_unrealises_the_record() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  let (_, err, code) = run(&["init", "upgraded-close"], root);
  assert_eq!(code, 0, "fixture init failed: {err}");
  let (out, err, code) = run(&["st", "new", "A thread that closes"], root);
  assert_eq!(code, 0, "fixture st new failed: {err}");
  let st = crate::common::created(&out);

  let (subject, err) = done_over_a_met_contract(root, &st);
  note_lines(
    &err,
    &format!("note: this unrealises {subject}, so the next `intent organize --apply` will remove:"),
    &format!(
      "  remedy: `intent st hydrate {subject}` writes them back from the store, and `st done --keep` or `st cancel --keep` closes and leaves it realised"
    ),
  );
  assert!(
    !err.iter().any(|line| line.contains("unlist")),
    "a manifest's word in an upgraded project's note: {err:#?}"
  );
}
