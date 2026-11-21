//! **ST0096 AC-05.2: two branches that change one thread's different records
//! change disjoint files, and git merges them with no conflict.**
//!
//! Before WP-05 a thread's work packages, criteria and tests were all inline
//! in its one canon file, so two merge requests that added a package and a
//! criterion to one thread both rewrote that file and met there. In the
//! release's format each record is a file of its own under the thread's
//! records directory, so the two change two sets of files.
//!
//! **THE REAL BINARY AND REAL GIT, MERGED AS A FORGE MERGES.** One branch adds
//! a work package, the other a criterion and a test, each from one base and
//! each committing exactly what its verbs printed. The forge merges both in its
//! bare repository, and a working copy that pulls the result finds both
//! changes and a clean `intent doctor`. dc's AC-04.2 proves the same through
//! the forge stand-in at scale; this is WP-05's own two-branch test.

use crate::common::{Forge, created};
use std::collections::BTreeSet;

/// The paths a branch changed against `base`, as git names them.
fn changed(forge: &Forge, cwd: &std::path::Path, base: &str, branch: &str) -> BTreeSet<String> {
  forge
    .git(cwd, &["diff", "--name-only", base, branch])
    .lines()
    .map(str::to_string)
    .collect()
}

// ST0096 AT-05.2: the arm below.
#[test]
fn two_branches_that_change_one_thread_merge_clean() {
  let forge = Forge::new();
  let alice = forge.clone_as("alice");
  forge.intent_ok(&alice, &["init", "Forge"]);
  let new = forge.intent_ok(&alice, &["st", "new", "One thread, two changes"]);
  let thread = created(&new);
  forge.intent_ok(&alice, &["st", "start", thread.as_str()]);
  forge.git(&alice, &["add", "-A"]);
  forge.git(&alice, &["commit", "-q", "-m", "one thread"]);
  forge.git(&alice, &["push", "-q", "origin", "HEAD:main"]);
  let base = forge.git(&forge.bare(), &["rev-parse", "refs/heads/main"]);
  let base = base.trim();

  let (package_branch, criterion_branch) = ("add-a-package", "add-a-criterion");
  forge.switch(&alice, &["-c", package_branch, base]);
  let package = forge.intent_ok(
    &alice,
    &["wp", "new", thread.as_str(), "A package on one branch"],
  );
  created(&package);
  forge.commit_printed(&alice, &[&package], package_branch);
  forge.git(&alice, &["push", "-q", "origin", package_branch]);

  let bob = forge.clone_as("bob");
  forge.switch(&bob, &["-c", criterion_branch, base]);
  let (criterion, test) = ("AC-00.1", "AT-00.1");
  let ac = forge.intent_ok(
    &bob,
    &[
      "ac",
      "new",
      thread.as_str(),
      criterion,
      "--text",
      "a criterion on the other branch",
    ],
  );
  let at = forge.intent_ok(
    &bob,
    &[
      "at",
      "new",
      thread.as_str(),
      test,
      "--covers",
      criterion,
      "--kind",
      "non-test",
      "--prose",
      "a test on the other branch",
    ],
  );
  forge.commit_printed(&bob, &[&ac, &at], criterion_branch);
  forge.git(&bob, &["push", "-q", "origin", criterion_branch]);

  let left = changed(&forge, &alice, base, package_branch);
  forge.git(&alice, &["fetch", "-q", "origin"]);
  let right = changed(&forge, &alice, base, &format!("origin/{criterion_branch}"));
  assert!(
    !left.is_empty() && !right.is_empty(),
    "each branch changed something: {left:?} {right:?}"
  );
  let shared: Vec<&String> = left.intersection(&right).collect();
  assert!(
    shared.is_empty(),
    "the two branches changed one file: {shared:?}"
  );

  forge.merge(package_branch);
  forge.merge(criterion_branch);
  forge.switch(&alice, &["main"]);
  forge.pull(&alice);

  let (out, err, code) = forge.intent(&alice, &["doctor"]);
  assert!(
    code == 0 && out.contains("doctor: 0 finding(s)"),
    "doctor is not clean on the merge:\n{out}{err}"
  );
  let packages = forge.intent_ok(&alice, &["wp", "list", thread.as_str()]);
  assert!(
    packages.contains("A package on one branch"),
    "the merge carries the package:\n{packages}"
  );
  let criteria = forge.intent_ok(&alice, &["ac", "list", thread.as_str()]);
  assert!(
    criteria.contains(criterion),
    "the merge carries the criterion:\n{criteria}"
  );
  let tests = forge.intent_ok(&alice, &["at", "list", thread.as_str()]);
  assert!(tests.contains(test), "the merge carries the test:\n{tests}");
}
