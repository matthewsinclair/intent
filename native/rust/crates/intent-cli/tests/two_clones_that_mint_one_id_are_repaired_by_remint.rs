//! **ST0078 WP-02's repair through the real binary, as ST0095 WP-10 leaves it:
//! two clones hold one ordinal, and one verb moves it** (AC-10.5, AC-06.3).
//!
//! ST0078's worked example E4, driven: Alice and Bob clone one origin and each
//! holds the same ordinal, as two branches cut before the upgrade do. Bob runs
//! `st remint`, which moves his to the git-style id the one mint derives, after
//! which the merge is clean and `doctor`, the verdict the commit gate runs, is
//! clean. The verb takes the id it moves and never the id it moves to, so no
//! verb mints an ordinal; the two-argument `renumber` that named one is refused
//! with `remint` as its remedy.
//!
//! **UNDER THE FORGE'S OWN `HOME`, WITH NO DAEMON**, as the design drove it: a
//! collaborator who never registered the project has no daemon watching it.
//! The origin, both clones and every commit are common's [`Forge`]'s.

use crate::common::{Forge, Said};

/// The id a remint moved to, read off its `ok:` line, `ok: <old> reminted to
/// <new>`: the one mint chose it, so a test reads it rather than names it.
fn reminted_to(out: &str) -> String {
  out
    .lines()
    .find_map(|l| l.strip_prefix("ok: "))
    .and_then(|l| l.rsplit_once(" reminted to "))
    .map(|(_, to)| to.to_string())
    .unwrap_or_else(|| panic!("no `reminted to` line:\n{out}"))
}

// ST0095 AT-10.9: the arm below.
#[test]
fn a_thread_both_clones_hold_is_reminted_and_the_merge_is_clean_under_doctor() {
  let forge = Forge::new();
  let alice = forge.clone_as("alice");
  forge.intent_ok(&alice, &["init", "Team"]);
  forge.commit_all(&alice, "intent init");
  forge.git(&alice, &["push", "-q", "origin", "HEAD:main"]);
  let bob = forge.clone_as("bob");
  assert!(forge.bare().exists());

  // Both hold one ordinal, the collision under repair. **SEEDED, NOT MINTED**
  // (ST0095 WP-06): a mint makes a git-style id now, which never collides, so
  // this is the ordinal a branch cut before the upgrade carries. Bob starts
  // his, so it is realised and declared.
  let alices = intentsvcs::id::ThreadId::from_ordinal(1)
    .expect("an ordinal thread")
    .stored();
  crate::common::seed_ordinal_thread(&alice, &alices, "Alice's next", &[]);
  forge.commit_all(&alice, &format!("alice: {alices}"));
  forge.git(&alice, &["push", "-q", "origin", "HEAD:main"]);
  let bobs = alices.clone();
  crate::common::seed_ordinal_thread(&bob, &bobs, "Bob's next", &[]);
  forge.intent_ok(&bob, &["st", "start", bobs.as_str()]);
  forge.commit_all(&bob, &format!("bob: {bobs}"));

  // The two-argument form named the id to move to, so it could mint an
  // ordinal; it is refused, and its remedy is the one-argument verb.
  let (out, err, code) = forge.intent(&bob, &["st", "renumber", bobs.as_str(), "ST0002"]);
  assert_eq!(code, 2, "`st renumber` is refused as retired:\n{out}{err}");
  assert!(
    err.contains("`intent st renumber` was retired") && err.contains("`intent st remint <id>`"),
    "the refusal names `st remint` as its remedy:\n{err}"
  );

  // Bob's record of the clashing id, where the project keeps it. It is on disk
  // before the repair, so its absence after the repair is the move.
  let bobs_record = crate::common::thread_canon_file(&bob, &bobs);
  assert!(
    bobs_record.exists(),
    "bob's record of {bobs} is on disk before the remint"
  );

  // The repair.
  let out = forge.intent_ok(&bob, &["st", "remint", bobs.as_str()]);
  let moved = reminted_to(&out);
  let moved_id = intentsvcs::id::ThreadId::parse(&moved).expect("a thread id");
  assert!(
    moved_id.is_hashed(),
    "a remint moves to a git-style id:\n{out}"
  );
  assert!(
    out.contains(&format!("moved: intent/st/{bobs} -> intent/st/{moved}")),
    "the realised directory moves and says so:\n{out}"
  );
  assert!(
    !bobs_record.exists(),
    "the remint leaves no record of {bobs} on bob's side"
  );
  assert!(crate::common::info_view(&bob, &moved).exists());
  forge.commit_all(&bob, "bob: remint");

  // The merge now touches canon cleanly, and nothing conflicts: the two lists
  // both clones rewrote are each clone's own render and travel in no commit
  // (ST0096), so the pull merges and commits by itself.
  let merge = Said::from(forge.git_try(&bob, &["pull", "-q", "--no-rebase", "origin", "main"]));
  let conflicted = forge.git(&bob, &["diff", "--name-only", "--diff-filter=U"]);
  assert!(
    merge.code == 0 && conflicted.trim().is_empty(),
    "after a remint the merge is clean, and {conflicted} conflicted:\n{}",
    merge.text
  );
  forge.intent_ok(&bob, &["sync", "--apply"]);
  forge.intent_ok(&bob, &["sync", "--to-disk"]);
  forge.intent_ok(&bob, &["todo", "update"]);
  // Whatever the three left to commit; the lists they rewrote are ignored, so
  // this may be nothing at all.
  forge.git(&bob, &["add", "-A"]);
  forge.commit(&bob, &["--allow-empty", "-m", "merge main"]);

  let list = forge.intent_ok(&bob, &["st", "list", "--status", "all"]);
  assert!(
    list.contains(&alices) && list.contains("Alice's next"),
    "{list}"
  );
  assert!(
    list.contains(&moved_id.shown()) && list.contains("Bob's next"),
    "{list}"
  );
  let (out, err, code) = forge.intent(&bob, &["doctor"]);
  assert!(
    out.contains("doctor: 0 finding(s)") && code == 0,
    "the merged clone is not clean under doctor:\n{out}{err}"
  );

  // A git-style id is never moved again: nothing is left to remint.
  let (out, err, code) = forge.intent(&bob, &["st", "remint", moved.as_str()]);
  assert_eq!(code, 1, "{out}{err}");
  assert!(
    err.contains("a git-style id stays as it is"),
    "a git-style id is refused as having nothing to remint:\n{err}"
  );
}

// ST0095 AT-10.10: the arm below.
#[test]
fn an_issue_remint_reports_what_it_moved_and_the_prose_it_left() {
  // The forge's HOME alone: this arm makes no clone, and its project is a
  // directory beside the bare repository.
  let forge = Forge::new();
  let project = forge.path("project");
  std::fs::create_dir_all(&project).expect("the project's directory");
  forge.intent_ok(&project, &["init", "Team"]);
  crate::common::seed_ordinal_issue(&project, 1, "First");
  let first = intentsvcs::id::IssueId::from_ordinal(1)
    .expect("an ordinal issue")
    .stored();
  let body = project.join("body.md");
  std::fs::write(&body, format!("Follows on from {first}.\n")).expect("write body");
  let second = crate::common::created(&forge.intent_ok(
    &project,
    &[
      "issues",
      "add",
      "Second",
      "--from",
      body.to_str().expect("utf-8"),
    ],
  ));

  let (out, err, code) = forge.intent(&project, &["issues", "renumber", first.as_str(), "7"]);
  assert_eq!(
    code, 2,
    "`issues renumber` is refused as retired:\n{out}{err}"
  );
  assert!(
    err.contains("`intent issues remint <id>`"),
    "the refusal names `issues remint` as its remedy:\n{err}"
  );

  let out = forge.intent_ok(&project, &["issues", "remint", first.as_str()]);
  let moved = reminted_to(&out);
  assert!(
    intentsvcs::id::IssueId::parse(&moved)
      .expect("an issue id")
      .is_hashed(),
    "a remint moves to a git-style id:\n{out}"
  );
  assert!(
    out.contains(&format!("moved: intent/.canon/issues/{first}.json removed")),
    "{out}"
  );
  assert!(
    out
      .lines()
      .any(|l| l.starts_with("prose: ") && l.contains(second.as_str())),
    "issue {second}'s body names {first} and is reported, not rewritten:\n{out}"
  );
  let shown = forge.intent_ok(&project, &["issues", "show", second.as_str()]);
  assert!(
    shown.contains(&format!("Follows on from {first}.")),
    "{shown}"
  );
  forge.intent_ok(&project, &["issues", "show", moved.as_str()]);
  let (out, err, _) = forge.intent(&project, &["doctor"]);
  assert!(out.contains("doctor: 0 finding(s)"), "{out}{err}");
}
