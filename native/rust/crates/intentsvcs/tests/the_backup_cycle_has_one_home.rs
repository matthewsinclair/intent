//! AT-08.8 / AC-08.8: **THE CHECK CLAUSE, AS A TEST: the scheduled path and the
//! manual path are the SAME FUNCTION, not two that agree today.**
//!
//! The criterion does not ask whether a daemon backup and a typed backup
//! produce the same result -- **two implementations agree on the day they are
//! written, which is exactly why agreeing is not the property worth
//! measuring.** It asks whether there is one implementation. That is a
//! question about call sites, so this reads call sites.
//!
//! **WHAT IT WOULD HAVE CAUGHT ON 2026-08-30, BEFORE THE CYCLE EXISTED.** The
//! backup cycle was not a function: `intent backup` composed
//! `backup::take`, `backup::Retention::from_project` and `backup::prune`
//! inline in `render.rs`, so the policy -- *a backup is followed by a prune
//! against this project's declared retention* -- lived in a renderer. Nothing
//! was wrong with the code and nothing was duplicated yet; the defect was that
//! **the only way for a second caller to do the same thing was to write it
//! again**, and the second caller was already specified.
//!
//! **ONLY THE TWO CALLS THAT WRITE ARE FORBIDDEN.** `Retention::from_project`
//! reads a setting and changes nothing, so anything that wants to REPORT the
//! retention may read it; forbidding that would buy no safety and block a
//! legitimate reader. The composition that has to be single-homed is
//! *snapshot, then prune*, and both of its halves change the estate.
//!
//! **`src/` ONLY, DELIBERATELY, AND FOR A DIFFERENT REASON THAN
//! `no_function_takes_a_time.rs` GIVES.** There the limit is that hv's rule is
//! about the shipped API. Here it is that `backup_retention.rs` drives `take`
//! and `prune` directly ON PURPOSE -- a prune has to be tested against
//! retentions no config would produce, and that is unit coverage of the two
//! halves rather than a second cycle. **A test calling both is not a second
//! implementation; a shipped path calling both is.**

use std::path::{Path, PathBuf};
use testkit::workspace_root;

/// The two service calls that WRITE, and so may have exactly one composer.
const WRITERS: &[&str] = &["take", "prune"];

/// The one file entitled to call them.
const THE_ONE_HOME: &str = "intentsvcs/src/backup.rs";

#[test]
fn nothing_outside_backup_rs_composes_a_backup_cycle() {
  let root = workspace_root();
  let mut offenders: Vec<String> = Vec::new();

  for path in sources(&root) {
    if shown(&root, &path).ends_with(THE_ONE_HOME) {
      continue;
    }
    let code = code_of(&path);
    for writer in WRITERS {
      // The qualified call, which is how every caller outside the module has
      // ever spelled it.
      if code.contains(&format!("backup::{writer}(")) {
        offenders.push(format!(
          "{}: calls `backup::{writer}(`",
          shown(&root, &path)
        ));
      }
      // **AND THE IMPORTED SPELLING, BECAUSE IT IS THE ONE WAY PAST THE FIRST
      // CHECK THAT COSTS NOTHING TO WRITE.** `use intentsvcs::backup::take;`
      // then a bare `take(..)` is the same call with the prefix moved into a
      // line at the top of the file.
      if code.contains(&format!("backup::{writer};"))
        || code.contains(&format!("backup::{{{writer}"))
        || code.contains(&format!("backup::{{self, {writer}"))
      {
        offenders.push(format!(
          "{}: imports `backup::{writer}`",
          shown(&root, &path)
        ));
      }
    }
  }

  assert!(
    offenders.is_empty(),
    "the backup cycle has more than one home. `intentsvcs::backup::cycle` is the \
     one composer of a snapshot and its prune, so that a scheduled backup and a \
     typed one cannot drift into two retention policies (`AC-08.8`). These \
     shipped paths reach the halves directly:\n  {}",
    offenders.join("\n  ")
  );
}

/// **THE POSITIVE HALF, AND WITHOUT IT THE GUARD ABOVE PASSES ON A TREE THAT
/// TAKES NO BACKUPS AT ALL.** A prohibition is satisfied most easily by
/// deleting the thing prohibited, so the forbidding test is green on a tree
/// where `intent backup` has been removed -- and that tree is broken in the
/// direction this criterion cares about most.
#[test]
fn the_one_home_has_callers() {
  let root = workspace_root();
  let callers: Vec<String> = sources(&root)
    .into_iter()
    .filter(|path| !shown(&root, path).ends_with(THE_ONE_HOME))
    .filter(|path| code_of(path).contains("backup::cycle("))
    .map(|path| shown(&root, &path))
    .collect();

  assert!(
    !callers.is_empty(),
    "nothing in any crate's `src/` calls `intentsvcs::backup::cycle`, so the \
     single-home guard beside this test is passing vacuously -- a tree that \
     takes no backups satisfies it perfectly"
  );
}

fn shown(root: &Path, path: &Path) -> String {
  path
    .strip_prefix(root)
    .unwrap_or(path)
    .display()
    .to_string()
}

/// Every `src/**/*.rs` in every crate, discovered by walking rather than
/// listed: the act that invalidates a hand-kept roster (adding a file) is not
/// the act that updates it.
fn sources(root: &Path) -> Vec<PathBuf> {
  fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
      return;
    };
    for entry in entries.flatten() {
      let path = entry.path();
      if path.is_dir() {
        walk(&path, out);
      } else if path.extension().is_some_and(|e| e == "rs") {
        out.push(path);
      }
    }
  }
  let mut out = Vec::new();
  for entry in std::fs::read_dir(root.join("crates"))
    .expect("read the crates dir")
    .flatten()
  {
    let src = entry.path().join("src");
    if src.is_dir() {
      walk(&src, &mut out);
    }
  }
  out.sort();
  out
}

/// The file with `//` lines dropped.
///
/// **DOC COMMENTS NAME THESE FUNCTIONS CONSTANTLY AND MUST NOT COUNT.** The
/// note on `cycle` explaining what it replaced says `take` and `prune` in
/// prose; a guard that read prose would make explaining the rule an offence
/// against it.
fn code_of(path: &Path) -> String {
  std::fs::read_to_string(path)
    .unwrap_or_default()
    .lines()
    .filter(|l| !l.trim_start().starts_with("//"))
    .collect::<Vec<_>>()
    .join("\n")
}
