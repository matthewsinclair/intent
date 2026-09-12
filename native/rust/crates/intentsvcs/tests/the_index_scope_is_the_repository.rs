//! AT-18.1, and AC-18.4's first half.
//!
//! AC-18.1 / AC-18.4: the index's scope is the gitignore-aware repository, it
//! is decided by ONE ignore statement shared with the sync scanner, and the
//! three exclusions that are not gitignored are out by RULE.
//!
//! **THE INSTRUMENT IS REAL GIT, AND IT IS ASKED ALONGSIDE US.** The rule is
//! git's ignore semantics -- nested files, negation, directory-vs-file, and
//! which directories are never descended into at all -- so every probe here
//! is put to `git check-ignore` as well, and disagreement fails. A
//! reimplementation of those semantics agrees with itself perfectly and
//! disagrees with git exactly where it matters.
//!
//! git is asked with `core.excludesFile=/dev/null` so the ANSWER does not
//! depend on whose machine is running the suite -- the machine-local half of
//! the same question `Ignored::for_root` declines for the same reason.
//!
//! **WHAT `the_ignore_walk_does_not_descend_into_what_it_ignores` PROVES, AND
//! WHAT IT DOES NOT.** It proves non-descent semantically rather than by the
//! clock: a `.gitignore` inside an ignored directory is unreachable to a walk
//! that prunes, and decisive to one that does not, so the two implementations
//! give opposite answers about one path. It says nothing about wall-clock, and
//! a timing assertion is deliberately not attempted -- the cost that forced
//! this design was 613,811 visited paths on one machine's build directory,
//! which is unbounded and machine-local and therefore not a threshold anyone
//! can write down.

//! **THE IGNORED BUILD DIRECTORY IN THESE FIXTURES IS `build/` AND NOT
//! `target/`, WHICH IS NOT A PREFERENCE.** `the_binary_under_test_is_the_one_
//! cargo_built` reds any test line that joins a path under `target/debug/` or
//! `target/release/`, because that spelling is how four files came to spawn
//! whatever the last default-target-dir build had left. The guard cannot tell
//! a fixture path inside a tempdir from a binary resolution, and it is right
//! not to try -- so the fixture uses a name that is not the trap's.

use std::path::{Path, PathBuf};
use std::process::Command;

use intentsvcs::sync::{ROOT_FILES, Scanned, scan};

struct Repo {
  dir: tempfile::TempDir,
}

impl Repo {
  /// A real git repository with a root `.gitignore`.
  fn new(gitignore: &str) -> Self {
    let dir = tempfile::tempdir().expect("tempdir");
    let ok = Command::new("git")
      .args(["init", "-q"])
      .current_dir(dir.path())
      .status()
      .expect("run git")
      .success();
    assert!(ok, "git init failed");
    std::fs::write(dir.path().join(".gitignore"), gitignore).expect("gitignore");
    Self { dir }
  }

  /// The same tree with NO repository over it.
  fn without_git(gitignore: &str) -> Self {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(dir.path().join(".gitignore"), gitignore).expect("gitignore");
    Self { dir }
  }

  fn root(&self) -> &Path {
    self.dir.path()
  }

  fn write(&self, rel: &str, body: &str) -> PathBuf {
    let path = self.root().join(rel);
    std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
    std::fs::write(&path, body).expect("write");
    path
  }

  fn scope(&self) -> Scanned {
    Scanned::for_root(self.root())
  }

  /// git's own answer, with the machine's global excludes taken out of it.
  fn git_ignores(&self, rel: &str) -> bool {
    Command::new("git")
      .args([
        "-c",
        "core.excludesFile=/dev/null",
        "check-ignore",
        "-q",
        rel,
      ])
      .current_dir(self.root())
      .status()
      .expect("run git check-ignore")
      .success()
  }

  /// Our answer and git's, together, so a divergence is named rather than
  /// discovered later by a user whose corpus is missing a file.
  fn agrees_with_git(&self, scope: &Scanned, rel: &str) {
    let ours = !scope.in_repository(&self.root().join(rel));
    let theirs = self.git_ignores(rel);
    assert_eq!(
      ours, theirs,
      "`{rel}`: the index says ignored={ours}, git says ignored={theirs} -- the \
       corpus rule IS git's ignore semantics, so a disagreement is a defect here"
    );
  }
}

#[test]
fn a_tracked_source_file_is_in_scope_and_a_gitignored_one_is_not() {
  let repo = Repo::new("build/\n*.tmp\n");
  repo.write("native/rust/src/lib.rs", "fn main() {}\n");
  repo.write("build/out/huge.o", "binary-ish\n");
  repo.write("notes/scratch.tmp", "transient\n");
  repo.write("intent/wip.md", "# wip\n");
  let scope = repo.scope();

  assert!(
    scope.in_repository(&repo.root().join("native/rust/src/lib.rs")),
    "a tracked source file is what widening the scope was FOR"
  );
  assert!(
    !scope.in_repository(&repo.root().join("build/out/huge.o")),
    "a file under an ignored directory is out, and the directory prunes its whole subtree"
  );
  assert!(
    !scope.in_repository(&repo.root().join("notes/scratch.tmp")),
    "and a file the root rules ignore by pattern is out wherever it sits"
  );
  assert!(
    scope.in_repository(&repo.root().join("intent/wip.md")),
    "Intent's own prose stays in the corpus it was already in"
  );

  for rel in [
    "native/rust/src/lib.rs",
    "build/out/huge.o",
    "notes/scratch.tmp",
    "intent/wip.md",
  ] {
    repo.agrees_with_git(&scope, rel);
  }
}

#[test]
fn git_the_store_and_the_backup_are_out_by_rule_and_not_by_being_ignored() {
  // **THE `.gitignore` DELIBERATELY NAMES NONE OF THEM.** In a real Intent
  // project `intent/.cache/` IS gitignored, which is exactly why this fixture
  // must not be: with the pattern present, an implementation that had dropped
  // the rule entirely would still pass.
  let repo = Repo::new("build/\n");
  repo.write("intent/.cache/intent.db", "SQLite format 3\0");
  repo.write("intent/.backup/2026-09-12/thread.json", "{}\n");
  repo.write("intent/.treeindex/cache.json", "{}\n");
  repo.write(".git/objects/ab/cdef", "object\n");
  let scope = repo.scope();

  for rel in [
    "intent/.cache/intent.db",
    "intent/.backup/2026-09-12/thread.json",
    "intent/.treeindex/cache.json",
  ] {
    assert!(
      !scope.in_repository(&repo.root().join(rel)),
      "`{rel}` is out by rule: the store is truth and the backup is a copy of \
       truth, and a copy of truth must never re-enter through the ingest gate"
    );
    assert!(
      !repo.git_ignores(rel),
      "the fixture must keep `{rel}` UNignored, or this arm passes with the rule gone"
    );
  }
  assert!(
    !scope.in_repository(&repo.root().join(".git/objects/ab/cdef")),
    "`.git/` is out by rule too: git does not ignore its own directory, it \
     never walks it, so a predicate that only asked the ignore matcher would \
     take the object database into the corpus"
  );
  assert!(
    !repo.git_ignores(".git/objects/ab/cdef"),
    "and git itself reports `.git` as NOT ignored, which is the whole point"
  );
}

#[test]
fn the_ignore_walk_does_not_descend_into_what_it_ignores() {
  let repo = Repo::new("vendor/\n");
  // Unreachable by construction: git never descends into an excluded
  // directory, so this file is never read and cannot un-ignore anything.
  repo.write("vendor/.gitignore", "!*\n");
  repo.write("vendor/x.rs", "fn vendored() {}\n");
  // The control, in the same fixture: a nested `.gitignore` in a directory the
  // walk DOES reach is read and decides.
  repo.write("live/.gitignore", "notes.log\n");
  repo.write("live/notes.log", "chatter\n");
  repo.write("live/keep.rs", "fn kept() {}\n");
  let scope = repo.scope();

  assert!(
    !scope.in_repository(&repo.root().join("vendor/x.rs")),
    "a `.gitignore` inside an ignored directory is not consulted -- an \
     implementation that enumerated the tree unfiltered would find it, honour \
     its `!*`, and take the whole of an ignored subtree into the corpus"
  );
  assert!(
    !scope.in_repository(&repo.root().join("live/notes.log")),
    "and a nested `.gitignore` the walk reaches DOES decide, or the arm above \
     would pass under a rule that simply never reads a nested file"
  );
  assert!(scope.in_repository(&repo.root().join("live/keep.rs")));

  for rel in ["vendor/x.rs", "live/notes.log", "live/keep.rs"] {
    repo.agrees_with_git(&scope, rel);
  }
}

#[test]
fn the_sync_corpus_is_exactly_what_it_was_before_the_scope_widened() {
  // **THE POINT OF THIS ARM IS THAT NOTHING MOVED.** `scan` feeds ingest, and
  // ingest is the only door into the store: a source file arriving in the sync
  // corpus would be ingested as canon. The index's scope is wider than the
  // scanner's deliberately, and the two are separate questions asked of one
  // ignore statement.
  let repo = Repo::new("build/\n");
  for name in ROOT_FILES {
    repo.write(name, "# root file\n");
  }
  repo.write("intent/wip.md", "# wip\n");
  repo.write("intent/.cache/intent.db", "SQLite format 3\0");
  repo.write("native/rust/src/lib.rs", "fn main() {}\n");
  repo.write("README.md", "# readme\n");
  repo.write("build/out/huge.o", "binary-ish\n");

  let mut got: Vec<String> = scan(repo.root(), &[])
    .expect("scan")
    .into_iter()
    .map(|e| e.path)
    .collect();
  got.sort();

  let mut want: Vec<String> = ROOT_FILES.iter().map(|n| (*n).to_string()).collect();
  want.push("intent/wip.md".to_string());
  want.sort();

  assert_eq!(
    got, want,
    "the sync corpus is Intent's own artefacts and nothing else: a source file \
     or a root `README.md` in this list would be a file ingest treats as canon"
  );
}

#[test]
fn one_ignore_statement_answers_both_questions() {
  // AC-18.4. Two predicates, two different questions, ONE statement of what
  // git ignores -- so a rule added to `.gitignore` moves both answers at once
  // and neither can drift from the other.
  let repo = Repo::new("intent/notes/*.draft\n");
  let drafted = repo.write("intent/notes/idea.draft", "half a thought\n");
  let kept = repo.write("intent/notes/idea.md", "the thought\n");
  let scope = repo.scope();

  assert!(!scope.includes(&drafted), "out of the sync corpus");
  assert!(
    !scope.in_repository(&drafted),
    "and out of the index corpus"
  );
  assert!(scope.includes(&kept));
  assert!(scope.in_repository(&kept));
  repo.agrees_with_git(&scope, "intent/notes/idea.draft");
  repo.agrees_with_git(&scope, "intent/notes/idea.md");
}

#[test]
fn a_project_inside_a_wider_repository_is_subject_to_its_rules() {
  // **THE ANCESTOR RULES ARE A CONDITION ON THE ANCESTRY, WHICH IS WHY THEY
  // NEED AN ARM OF THEIR OWN.** They came free from the walker while the
  // instrument was a difference between two walks; a matcher built from the
  // `.gitignore` files found INSIDE the root cannot see them at all, so the
  // corpus of a nested project silently widens the day the instrument changes.
  let outer = Repo::new("*.log\n");
  outer.write("proj/intent/wip.md", "# wip\n");
  outer.write("proj/notes.log", "chatter\n");
  outer.write("proj/src/keep.rs", "fn kept() {}\n");
  let project = outer.root().join("proj");
  let scope = Scanned::for_root(&project);

  assert!(
    !scope.in_repository(&project.join("notes.log")),
    "a project nested inside a repository is subject to that repository's \
     committed rules -- git reads them and so must the corpus"
  );
  assert!(scope.in_repository(&project.join("src/keep.rs")));
  assert!(scope.in_repository(&project.join("intent/wip.md")));

  // git's own answer about the same paths, from the repository that owns them.
  assert!(outer.git_ignores("proj/notes.log"));
  assert!(!outer.git_ignores("proj/src/keep.rs"));
}

#[test]
fn a_tree_git_does_not_govern_ignores_nothing_even_where_a_gitignore_file_sits() {
  // D29's own edge, stated in `Ignored`'s note: no repository means nothing is
  // ignored, not that everything is. A `.gitignore` no git would ever read is
  // a file like any other, and reading it here would take files out of the
  // corpus of a project that has no repository to put them back.
  let loose = Repo::without_git("*.log\n");
  loose.write("notes.log", "chatter\n");
  loose.write("src/keep.rs", "fn kept() {}\n");
  let scope = loose.scope();

  assert!(
    scope.in_repository(&loose.root().join("notes.log")),
    "no git means nothing ignored, not everything ignored -- the corpus \
     degrades to everything-in-scope"
  );
  assert!(scope.in_repository(&loose.root().join("src/keep.rs")));

  // **THE PAIR THAT MAKES IT A MEASUREMENT.** The identical tree, one `git
  // init` apart, gives the opposite answer about the identical file.
  let governed = Repo::new("*.log\n");
  governed.write("notes.log", "chatter\n");
  assert!(
    !governed
      .scope()
      .in_repository(&governed.root().join("notes.log"))
  );
}
