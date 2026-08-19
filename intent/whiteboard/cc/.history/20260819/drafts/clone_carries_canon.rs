//! AT-01.2 / ST0057 AC-01.2: a fresh clone carries canon for every artefact.
//!
//! **The criterion forbids the cheap check by name**: "checked by cloning,
//! never by reading `.gitignore`, because the question is what git DOES and
//! not what a rule appears to say." So both arms here ask git, and neither
//! parses an ignore file.
//!
//! **Why this does NOT use `common::Fixture::clone_extract`.** That helper is a
//! directory copy with ONE HARDCODED EXCEPTION (`.cache/`), and its doc
//! comment claims it is "what `git clone` leaves behind". For its own callers
//! -- cold-start tests wanting canon with no database -- that is fine. Here it
//! is a decoy: it encodes the AUTHOR'S BELIEF about the ignore rules instead
//! of asking git. If a commit added `intent/.*/` to `.gitignore` -- the exact
//! tidy-looking edit AC-01.5 names -- `clone_extract` would copy `.canon/`
//! anyway and report a complete clone. **It is blind to precisely the failure
//! this criterion exists to catch**, so reusing it here would produce a green
//! that means nothing.
//!
//! **Arm (b) prints both figures and REQUIRES THE DENOMINATOR TO BE NON-ZERO.**
//! If `.canon/` were ignored it would never be committed, so a denominator
//! taken from `git ls-tree HEAD` would itself be empty and the equality would
//! hold at 0 == 0 -- passing loudest exactly when the estate is broken. The
//! denominator is therefore the WORKTREE's canon population, which exists
//! whatever git thinks of it.

use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;
use testkit::repo_root;

/// Every canon file in a worktree, as paths relative to the project root.
///
/// **Walked recursively rather than over a hardcoded `["st", "issues"]`.** A
/// fixed subdirectory list is the EXTENT limb: if canon grows a third artefact
/// kind, the list under-counts BOTH sides, the equality still holds, and the
/// new kind is silently exempt from the criterion that exists to carry it.
fn canon_population(root: &Path) -> BTreeSet<String> {
  let canon_root = root.join("intent/.canon");
  let mut found = BTreeSet::new();
  walk(&canon_root, &canon_root, &mut found);
  found
}

fn walk(base: &Path, dir: &Path, found: &mut BTreeSet<String>) {
  let Ok(entries) = std::fs::read_dir(dir) else { return };
  for entry in entries.flatten() {
    let path = entry.path();
    if path.is_dir() {
      walk(base, &path, found);
    } else if path.extension().is_some_and(|e| e == "json") {
      let rel = path.strip_prefix(base).expect("under canon root");
      found.insert(format!("intent/.canon/{}", rel.to_string_lossy()));
    }
  }
}

#[test]
fn no_canon_path_is_matched_by_any_ignore_rule() {
  let root = repo_root();
  let canon: Vec<String> = canon_population(&root).into_iter().collect();
  assert!(!canon.is_empty(), "no canon files found under intent/.canon -- the probe cannot fail");

  // `check-ignore` is git's OWN matcher, so this asks what git does rather
  // than what a rule appears to say. Exit 0 means AT LEAST ONE path matched,
  // which is the failure; exit 1 means none did, which is the pass. Reading
  // this backwards is the whole hazard, so it is asserted on the code.
  //
  // **THIS ARM IS NOT A CHEAPER VERSION OF THE CLONE ARM; IT COVERS A CASE THE
  // CLONE ARM STRUCTURALLY CANNOT SEE.** Git's ignore rules apply only to
  // UNTRACKED paths, so adding `intent/.*/` to `.gitignore` today leaves all
  // existing canon tracked and cloning perfectly -- while every artefact
  // minted afterwards is silently skipped by `git add`. The clone arm stays
  // GREEN as the estate is progressively hollowed out. `--no-index` is what
  // makes this arm see the rule at all, because it asks whether the path WOULD
  // be ignored rather than whether git currently tracks it.
  let out = Command::new("git")
    .arg("-C").arg(&root)
    .arg("check-ignore").arg("--no-index").arg("-v")
    .args(&canon)
    .output()
    .expect("git check-ignore runs");

  let matched = String::from_utf8_lossy(&out.stdout);
  assert!(
    !out.status.success(),
    "{} of {} canon paths are matched by an ignore rule, so they never reach a clone (D29):\n{}",
    matched.lines().count(),
    canon.len(),
    matched
  );
}

#[test]
fn a_fresh_clone_carries_canon_for_every_artefact() {
  let root = repo_root();
  let source = canon_population(&root);
  assert!(!source.is_empty(), "the source tree has no canon -- 0 == 0 would pass vacuously");

  let dest = tempfile::tempdir().expect("tempdir");
  let clone_at = dest.path().join("clone");
  let out = Command::new("git")
    .arg("clone").arg("--quiet").arg("--local")
    .arg(&root).arg(&clone_at)
    .output()
    .expect("git clone runs");
  assert!(out.status.success(), "git clone failed: {}", String::from_utf8_lossy(&out.stderr));

  let cloned = canon_population(&clone_at);
  let missing: Vec<&String> = source.difference(&cloned).collect();

  // A canon file absent from the clone is either ignored (the defect) or
  // merely uncommitted (developer state). Both are reported, because a clone
  // genuinely does not carry either -- but they are DIFFERENT findings and a
  // bare count cannot tell them apart.
  assert!(
    missing.is_empty(),
    "clone carries {} of {} canon artefacts; {} did not travel:\n{}",
    cloned.len(),
    source.len(),
    missing.len(),
    missing.iter().map(|m| format!("  {m}")).collect::<Vec<_>>().join("\n")
  );
}

/// **THE TRACKED-VERSUS-PRESENT ARM (ic).** Every check that asks git for its
/// own state is blind to this defect, because **git is the thing that has been
/// lied to**: `git status --porcelain` returns ZERO BYTES over an unstaged new
/// canon file, and a clone stays complete and correct throughout. **The disk is
/// the second channel and it is the one nobody consults.** This arm compares
/// what git HOLDS against what is actually THERE. It needs no flag and no
/// knowledge of any rule, and it diverges from the first new artefact onward.
///
/// **It does not subsume the check-ignore arm and does not replace it.** In the
/// window between the bad edit and the first mint there is nothing yet to
/// diverge, so `--no-index` is the only detector with zero latency. Two arms,
/// two latencies, both kept.
///
/// **When it fires it asks git WHY**, because "untracked" has two causes with
/// opposite meanings: matched by an ignore rule (the defect) or simply minted
/// and not yet added (ordinary work in progress). A bare list cannot tell them
/// apart and would train a reader to dismiss the real one.
#[test]
fn every_canon_file_on_disk_is_tracked_by_git() {
  let root = repo_root();
  let present = canon_population(&root);
  assert!(!present.is_empty(), "no canon on disk -- the comparison has nothing to compare");

  let out = Command::new("git")
    .arg("-C").arg(&root)
    .arg("ls-files").arg("--").arg("intent/.canon")
    .output()
    .expect("git ls-files runs");
  let tracked: BTreeSet<String> =
    String::from_utf8_lossy(&out.stdout).lines().map(str::to_string).collect();

  let untracked: Vec<&String> = present.difference(&tracked).collect();
  if untracked.is_empty() {
    return;
  }

  let ignored = Command::new("git")
    .arg("-C").arg(&root)
    .arg("check-ignore").arg("--no-index").arg("-v")
    .args(untracked.iter().map(|s| s.as_str()))
    .output()
    .expect("git check-ignore runs");
  let why = if ignored.status.success() {
    format!("MATCHED BY AN IGNORE RULE -- this is the defect:\n{}", String::from_utf8_lossy(&ignored.stdout))
  } else {
    "not matched by any ignore rule, so this is uncommitted work in progress rather than the defect".to_string()
  };

  panic!(
    "git holds {} of {} canon files on disk; {} are present but untracked.\n{}",
    tracked.len(),
    present.len(),
    untracked.len(),
    why
  );
}
