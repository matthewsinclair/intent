//! ST0095 WP-06, AC-06.1: **two clones, and two worktrees, that each mint a
//! thread, a work package in one shared thread, and an issue from one base,
//! merge with different ids and no repair.**
//!
//! Every id is drawn from the event that makes its record, so no two copies
//! can compute the same one, and since ST0096 every record is a file of its
//! own and `.intentfiles` has left git. Four branches cut from one `main`, two
//! from two clones and two from two worktrees of a third clone, each mint all
//! three kinds of record. The forge merges all four as its merge button does,
//! and a working copy that pulls the result plans no repair, takes no side and
//! finds every record, under a clean `intent doctor`.

use crate::common::{Forge, created, shown};
use std::path::Path;

/// A thread, a package in `shared` and an issue, minted in `copy` on a branch
/// `branch` cut from the base, committed and pushed: their stored ids.
fn mint_on_a_branch(forge: &Forge, copy: &Path, branch: &str, shared: &str) -> [String; 3] {
  forge.switch(copy, &["-c", branch]);
  let st = forge.intent_ok(copy, &["st", "new", &format!("{branch}'s thread")]);
  let wp = forge.intent_ok(copy, &["wp", "new", shared, &format!("{branch}'s package")]);
  let issue = forge.intent_ok(copy, &["issues", "add", &format!("{branch}'s issue")]);
  forge.commit_printed(copy, &[&st, &wp, &issue], branch);
  let (st, wp, issue) = (created(&st), created(&wp), created(&issue));
  forge.git(copy, &["push", "-q", "origin", branch]);
  [st, wp, issue]
}

// ST0095 AT-06.1: the arm below.
#[test]
fn two_clones_and_two_worktrees_that_each_mint_merge_with_no_repair() {
  let forge = Forge::new();
  let alice = forge.clone_as("alice");
  forge.intent_ok(&alice, &["init", "Forge"]);
  let shared = created(&forge.intent_ok(&alice, &["st", "new", "One shared thread"]));
  forge.commit_all(&alice, "the base");
  forge.git(&alice, &["push", "-q", "origin", "HEAD:main"]);

  let bob = forge.clone_as("bob");
  let carol = forge.clone_as("carol");
  let copies = [
    (alice.clone(), "alices-clone"),
    (bob, "bobs-clone"),
    (
      forge.worktree_of(&carol, "carol-left"),
      "carols-left-worktree",
    ),
    (
      forge.worktree_of(&carol, "carol-right"),
      "carols-right-worktree",
    ),
  ];
  let minted: Vec<(&str, [String; 3])> = copies
    .iter()
    .map(|(copy, branch)| (*branch, mint_on_a_branch(&forge, copy, branch, &shared)))
    .collect();

  for (kind, at) in [("thread", 0), ("package", 1), ("issue", 2)] {
    let ids: std::collections::BTreeSet<&String> = minted.iter().map(|(_, ids)| &ids[at]).collect();
    assert_eq!(
      ids.len(),
      minted.len(),
      "two copies minted one {kind} id: {minted:?}"
    );
  }

  for (branch, _) in &minted {
    forge.merge(branch);
  }
  // A plain pull, so the plan read next is the merge's own, before anything
  // applies it ([`Forge::pull`] would apply it first).
  forge.git(&alice, &["switch", "-q", "main"]);
  forge.git(&alice, &["pull", "-q", "--ff-only", "origin", "main"]);
  let plan = forge.intent_said(&alice, &["sync"]);
  assert!(
    !plan.text.contains("renumber") && !plan.text.contains("take a side"),
    "a merge of minted ids plans no repair and takes no side: {}",
    plan.text
  );
  forge.intent_ok(&alice, &["sync", "--apply", "--yes"]);
  let (out, err, code) = forge.intent(&alice, &["doctor"]);
  assert!(
    code == 0 && out.contains("doctor: 0 finding(s)"),
    "doctor is not clean on the merge:\n{out}{err}"
  );
  let threads = forge.intent_ok(&alice, &["st", "list", "--status", "all"]);
  let packages = forge.intent_ok(&alice, &["wp", "list", shared.as_str()]);
  let issues = forge.intent_ok(&alice, &["issues", "list"]);
  for (branch, [st, wp, issue]) in &minted {
    assert!(
      threads.contains(&shown(st)),
      "{branch}'s thread {st} is gone:\n{threads}"
    );
    let package = crate::common::package_of(wp).shown();
    assert!(
      packages.contains(&package),
      "{branch}'s package {wp} is gone:\n{packages}"
    );
    assert!(
      issues.contains(&shown(issue)),
      "{branch}'s issue {issue} is gone:\n{issues}"
    );
  }
}
