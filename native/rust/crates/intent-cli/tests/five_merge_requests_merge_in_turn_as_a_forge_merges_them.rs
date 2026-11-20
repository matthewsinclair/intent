//! **ST0096 AC-04.1: five merge requests opened from one main merge one after
//! another as a forge merges them, with no conflict and no hand step.**
//!
//! The reproduction this answers (vc, 2026-11-19, brew 3.4.0): five branches
//! each minted and started a thread from one main, and a second clone merged
//! them as a forge's merge button does. After the first merge none of the other
//! four merged: each conflicted with main in `intent/.intentfiles`,
//! `intent/st/steel_threads.md` and `intent/todo.md`, the files every record
//! shared. ST0096 takes realisation out of the shared manifest (WP-01) and the
//! two generated lists out of git (WP-02), so a record's change writes only its
//! own files, and ST0095 makes every mint's id its own.
//!
//! **THE REAL BINARY AND REAL GIT, AND A FORGE THAT RUNS NEITHER `intent` NOR A
//! MERGE DRIVER.** The five merge requests are opened in two clones and a
//! worktree of each, all from one `main`, and each runs `st new`, `st start`,
//! `wp new` and `issues add` and commits exactly what those verbs printed. The
//! forge merges each in turn in its bare repository with `merge-tree`, as a
//! forge does, so a `.gitattributes` driver could not help even if one were
//! declared.
//!
//! **IT PROVES AC-04.1 FOR WORKING COPIES WHOSE HOOKS ARE WIRED.** Every
//! branch switch and every pull in a copy runs `intent sync --apply` after it,
//! the one command the shipped post-checkout and post-merge hook runs
//! (`lib/templates/hooks/post-pull.sh`), through `Forge`'s one switch helper;
//! a fresh clone or worktree is not synced, as the hook skips a fresh
//! checkout. A copy that switches branches without it is
//! issue 0739, which this rig found: one copy here opens two merge requests,
//! and its store would carry the first into the second. After each merge one
//! working copy pulls `main` and runs `intent doctor`.
//!
//! Out of scope: two merge requests that change one thread (AC-04.2, after
//! WP-05).

use crate::common::{Forge, created, shown};

/// One merge request: its branch and the records it minted.
struct Request {
  branch: &'static str,
  thread: String,
  issue: String,
}

/// AT-04.1 (ST0096 AC-04.1): five merge requests opened from one main, in two
/// clones and two worktrees, each creating and starting a thread and adding a
/// work package and an issue, merge in turn by a bare repository's merge as a
/// forge merges, with no conflict, and `intent doctor` is clean after each is
/// pulled, with both lists naming every record merged so far and none that is
/// not, for working copies whose hooks are wired.
#[test]
fn five_merge_requests_merge_in_turn_as_a_forge_merges_them() {
  let forge = Forge::new();

  // One main, born in the first clone and pushed.
  let alice = forge.clone_as("alice");
  forge.intent_ok(&alice, &["init", "Forge"]);
  // A board on main, so every copy carries tracked board files through each
  // checkout and pull: the store carries them, and a sync that planned the
  // index without them would leave a step behind (issue 0741).
  forge.intent_ok(
    &alice,
    &[
      "wb",
      "register",
      "al",
      "--name",
      "Alice's agent",
      "--role",
      "worker",
    ],
  );
  forge.git(&alice, &["add", "-A"]);
  forge.git(&alice, &["commit", "-q", "-m", "intent init"]);
  forge.git(&alice, &["push", "-q", "origin", "HEAD:main"]);
  let base = forge.git(&forge.bare(), &["rev-parse", "refs/heads/main"]);
  let base = base.trim();

  let bob = forge.clone_as("bob");
  let alice_worktree = forge.worktree_of(&alice, "alice-worktree");
  let bob_worktree = forge.worktree_of(&bob, "bob-worktree");
  let copies = [alice, alice_worktree, bob, bob_worktree];

  // The first copy opens the first merge request and then pulls each merge;
  // the other four are spread over the other three copies, so one of them
  // opens two from one working copy, the case that found issue 0739.
  let branches = ["mr-one", "mr-two", "mr-three", "mr-four", "mr-five"];
  let openers = std::iter::once(&copies[0]).chain(copies[1..].iter().cycle());
  let requests: Vec<Request> = branches
    .iter()
    .zip(openers)
    .map(|(&branch, copy)| {
      forge.switch(copy, &["-c", branch, base]);
      let new = forge.intent_ok(copy, &["st", "new", format!("Thread of {branch}").as_str()]);
      let thread = created(&new);
      let start = forge.intent_ok(copy, &["st", "start", thread.as_str()]);
      let package = forge.intent_ok(
        copy,
        &[
          "wp",
          "new",
          thread.as_str(),
          format!("Package of {branch}").as_str(),
        ],
      );
      // Read for its assertion alone: the mint named exactly one package.
      created(&package);
      let filed = forge.intent_ok(
        copy,
        &["issues", "add", format!("Issue of {branch}").as_str()],
      );
      let issue = created(&filed);
      forge.commit_printed(copy, &[&new, &start, &package, &filed], branch);
      forge.git(copy, &["push", "-q", "origin", branch]);
      Request {
        branch,
        thread,
        issue,
      }
    })
    .collect();

  let puller = &copies[0];
  forge.switch(puller, &["main"]);
  for (merged, request) in requests.iter().enumerate() {
    forge.merge(request.branch);
    forge.pull(puller);

    let (out, err, code) = forge.intent(puller, &["doctor"]);
    assert!(
      code == 0 && out.contains("doctor: 0 finding(s)"),
      "after merging `{}`, doctor is not clean on the pulled main:\n{out}{err}",
      request.branch
    );

    let threads = forge.intent_ok(puller, &["st", "list", "--status", "all"]);
    let issues = forge.intent_ok(puller, &["issues", "list", "--kind", "all"]);
    let (done, open) = requests.split_at(merged + 1);
    for r in done {
      assert!(
        threads.contains(&shown(&r.thread)) && issues.contains(&shown(&r.issue)),
        "after merging `{}`, the lists do not name `{}`'s thread {} and issue {}:\n{threads}\n{issues}",
        request.branch,
        r.branch,
        r.thread,
        r.issue
      );
    }
    for r in open {
      assert!(
        !threads.contains(&shown(&r.thread)) && !issues.contains(&shown(&r.issue)),
        "after merging `{}`, the lists name `{}`'s records, which main has not merged:\n{threads}\n{issues}",
        request.branch,
        r.branch
      );
    }
  }
}

/// **THE FORGE'S MERGE CAN FIND A CONFLICT**, so its finding none above is a
/// finding. Two branches from one main each write different bytes to one path,
/// as `.intentfiles` and the two lists were written before ST0096. The first
/// merges; the second is refused with that path named, and `main` stays at the
/// first merge.
#[test]
fn the_forges_merge_refuses_a_conflict_and_names_its_path() {
  let forge = Forge::new();
  let alice = forge.clone_as("alice");
  let shared = "shared.md";
  std::fs::write(alice.join(shared), "one main\n").expect("write the shared file");
  forge.git(&alice, &["add", "--", shared]);
  forge.git(&alice, &["commit", "-q", "-m", "one main"]);
  forge.git(&alice, &["push", "-q", "origin", "HEAD:main"]);
  let base = forge.git(&forge.bare(), &["rev-parse", "refs/heads/main"]);
  let base = base.trim();

  let branches = ["left", "right"];
  for branch in branches {
    forge.git(&alice, &["switch", "-q", "-c", branch, base]);
    std::fs::write(alice.join(shared), format!("{branch}\n")).expect("change the shared file");
    forge.git(&alice, &["commit", "-q", "-am", branch]);
    forge.git(&alice, &["push", "-q", "origin", branch]);
  }

  forge.merge(branches[0]);
  let first = forge.git(&forge.bare(), &["rev-parse", "refs/heads/main"]);
  assert_eq!(
    forge.merge_try(branches[1]),
    Err(vec![shared.to_string()]),
    "the second branch's change to {shared} is a conflict with the first's"
  );
  assert_eq!(
    forge.git(&forge.bare(), &["rev-parse", "refs/heads/main"]),
    first,
    "a refused merge leaves main where it was"
  );
}
