//! AT-03.5 / ST0057 AC-03.5: **`sync --to-store` REPORTS an attachment whose
//! on-disk bytes differ from what the INDEX holds for that path.**
//!
//! # The defect, measured rather than anticipated
//!
//! The ingest path reads the WORKTREE. So an uncommitted edit sitting in the
//! tree is carried into canon by whoever syncs next, and canon then records an
//! artefact whose bytes exist in NO COMMIT -- **indistinguishable on inspection
//! from a correct one**, because there is nothing about a canon entry that says
//! where its bytes came from. dc measured it twice in one afternoon, once to the
//! node who wrote the commit-yours-first rule. **A rule that lives in a peer
//! message is followed until somebody is mid-task.**
//!
//! # The comparison is against the INDEX, and the staged arm is what proves it
//!
//! `git commit` records the index, so the index is what the NEXT commit will
//! contain. Compare against HEAD instead and every staged file reports as
//! uncommitted -- which is the normal state of a commit being assembled, and a
//! check that fires on ordinary work is one people learn to skip past.
//!
//! **So `a_staged_edit_is_not_reported` is the load-bearing arm.** An
//! implementation comparing to HEAD passes every other test in this file and
//! fails only that one.
//!
//! # Reported, never refused
//!
//! Canon holding uncommitted bytes in a working tree is a dirty tree: normal,
//! reversible, nobody's problem. It becomes permanent and inspectable-but-wrong
//! at the COMMIT, which is AC-03.6's gate. Refusing here would block the
//! ordinary act of saving your own in-flight work.

mod common;

use intentsvcs::sync::{NotInIndex, Uncommitted, uncommitted};
use std::path::Path;
use std::process::Command;

const REL: &str = "intent/st/ST0001/reference.md";

struct Repo {
  dir: tempfile::TempDir,
}

impl Repo {
  /// A real git repository, because the question IS git's -- what the index
  /// holds -- and a reimplementation would disagree exactly where it matters.
  fn new() -> Self {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    for args in [
      vec!["init", "-q"],
      vec!["config", "user.email", "t@example.com"],
      vec!["config", "user.name", "t"],
    ] {
      let ok = Command::new("git")
        .args(&args)
        .current_dir(root)
        .status()
        .expect("run git")
        .success();
      assert!(ok, "git {args:?} failed");
    }
    Self { dir }
  }

  fn root(&self) -> &Path {
    self.dir.path()
  }

  fn write(&self, rel: &str, body: &str) {
    let path = self.root().join(rel);
    std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
    std::fs::write(path, body).expect("write");
  }

  fn git(&self, args: &[&str]) {
    let ok = Command::new("git")
      .args(args)
      .current_dir(self.root())
      .status()
      .expect("run git")
      .success();
    assert!(ok, "git {args:?} failed");
  }

  fn found(&self, paths: &[&str]) -> Vec<Uncommitted> {
    let owned: Vec<String> = paths.iter().map(|p| (*p).to_string()).collect();
    uncommitted(self.root(), &owned).expect("a repository exists, so the question is answerable")
  }
}

/// A repository whose attachment is committed and whose worktree agrees with
/// the index -- the row's own "demonstrated red first against a tree where the
/// bytes agree" starting point.
fn committed() -> Repo {
  let repo = Repo::new();
  repo.write(REL, "# Reference\n");
  repo.git(&["add", "."]);
  repo.git(&["commit", "-qm", "the attachment as committed"]);
  repo
}

/// **THE CONTROL, and the row asks for it by name.**
#[test]
fn an_attachment_matching_the_index_is_not_reported() {
  let repo = committed();
  assert_eq!(
    repo.found(&[REL]),
    Vec::new(),
    "an attachment whose bytes agree with the index is reported as uncommitted, so every detection \
     below would fire for a tree in which nothing is wrong"
  );
}

/// **THE CRITERION: an edit in the worktree that was never staged is reported,
/// and the report names the path.**
#[test]
fn an_attachment_edited_in_the_worktree_and_not_staged_is_reported_by_path() {
  let repo = committed();
  repo.write(REL, "# Reference\n\nEdited, and never staged.\n");

  let found = repo.found(&[REL]);
  assert_eq!(
    found.len(),
    1,
    "one attachment was edited and {} were reported: {found:?}",
    found.len()
  );
  assert_eq!(found[0].path, REL, "the report does not name the path");
  assert_eq!(found[0].state, NotInIndex::Modified);
  assert!(
    found[0].to_string().contains(REL),
    "the rendered line does not carry the path an operator has to go and look at: {}",
    found[0]
  );
}

/// **THE ARM THAT PROVES THE COMPARISON IS AGAINST THE INDEX AND NOT HEAD.**
///
/// The same edit, staged. Its bytes ARE what the next commit will record, so
/// there is nothing to report. **An implementation comparing to HEAD reports it
/// and is wrong** -- it would fire on every commit anyone is in the middle of
/// assembling, and a check that fires on ordinary work is one people learn to
/// skip.
#[test]
fn a_staged_edit_is_not_reported() {
  let repo = committed();
  repo.write(REL, "# Reference\n\nEdited, and staged.\n");
  repo.git(&["add", REL]);

  assert_eq!(
    repo.found(&[REL]),
    Vec::new(),
    "a STAGED edit is reported as uncommitted. The comparison is against HEAD rather than the \
     index, so this fires for every commit in the middle of being assembled"
  );

  // And the pairing that makes it discrimination rather than a check that
  // stopped reporting: unstage it and the same bytes report again.
  repo.git(&["restore", "--staged", REL]);
  assert_eq!(
    repo.found(&[REL]).len(),
    1,
    "the same bytes, no longer staged, are NOT reported -- so the arm above passed because the \
     check went quiet, not because staging answered it"
  );
}

/// **AN UNTRACKED ATTACHMENT IS REPORTED, and it is the one most likely to be
/// missed.**
///
/// `diff-files` answers only for TRACKED files, so a check built on it alone is
/// silent about a file the index has never heard of -- which is precisely the
/// file whose bytes are least likely to be in any commit.
#[test]
fn an_attachment_the_index_has_never_heard_of_is_reported() {
  let repo = committed();
  let fresh = "intent/st/ST0001/brand-new.md";
  repo.write(fresh, "# Written ten seconds ago\n");

  let found = repo.found(&[REL, fresh]);
  assert_eq!(
    found.len(),
    1,
    "expected only the untracked file: {found:?}"
  );
  assert_eq!(found[0].path, fresh);
  assert_eq!(
    found[0].state,
    NotInIndex::Untracked,
    "an untracked file is reported as a worktree modification, which sends the operator to `git \
     diff` for a file that has no diff"
  );
}

/// **A PATH WITH SPACES SURVIVES, AND THIS ESTATE HAS THE COUNTEREXAMPLE ON
/// DISK.**
///
/// It carries a `.webloc` whose name contains spaces -- the file AC-03.3's
/// naming gate exists to reject -- and a whitespace split of a path list turned
/// that ONE file into EIGHT fragments while the run printed a plausible
/// four-digit total. Porcelain output additionally QUOTES such paths. `-z` and
/// NUL splitting is why this passes, and nothing else about the implementation
/// would tell a reader that mattered.
#[test]
fn a_path_with_spaces_is_one_finding_and_not_several() {
  let repo = committed();
  let awkward = "intent/st/ST0001/Prompts, workflows and more.webloc";
  repo.write(awkward, "<?xml version=\"1.0\"?>\n");

  let found = repo.found(&[awkward]);
  assert_eq!(
    found.len(),
    1,
    "a path containing spaces produced {} finding(s) instead of one -- the path list was split on \
     whitespace: {found:?}",
    found.len()
  );
  assert_eq!(
    found[0].path, awkward,
    "the path came back fragmented or requoted: {:?}",
    found[0].path
  );
}

/// **NO REPOSITORY IS `None`, NOT AN EMPTY LIST.**
///
/// The two mean opposite things: "nothing is uncommitted" is a clean bill of
/// health, and "I could not ask" is the absence of one. Collapsing them would
/// let a project with no git report as fully committed, which is the reassuring
/// answer and the wrong one.
#[test]
fn a_tree_with_no_repository_answers_unknown_rather_than_clean() {
  let dir = tempfile::tempdir().expect("tempdir");
  std::fs::create_dir_all(dir.path().join("intent/st/ST0001")).expect("mkdir");
  std::fs::write(dir.path().join(REL), "# Reference\n").expect("write");

  assert_eq!(
    uncommitted(dir.path(), &[REL.to_string()]),
    None,
    "a tree with no repository answered the question instead of declining it. An empty list here \
     reads as a clean bill of health nobody earned"
  );
}

/// Only the paths asked about are reported -- the whole-repo query is
/// intersected, not returned.
#[test]
fn a_file_that_is_not_an_attachment_is_not_reported() {
  let repo = committed();
  repo.write("README.md", "unstaged, and none of this check's business\n");

  assert_eq!(
    repo.found(&[REL]),
    Vec::new(),
    "a dirty file nobody asked about was reported. The query is over the whole repository, so the \
     intersection with the attachment list is the only thing keeping this to its subject"
  );
}
