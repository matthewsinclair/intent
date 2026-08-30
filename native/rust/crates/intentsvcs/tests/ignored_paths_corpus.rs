//! AT-03.7 / AC-03.7 (D29): a path git would never commit is never canon, so
//! it never produces residue and never blocks a read.
//!
//! **The bug this closes made every macOS checkout dead on arrival.** Ingest
//! walks the filesystem; git does not. Every directory on a Mac acquires a
//! `.DS_Store`, `.gitignore` excludes them, and strict ingest then correctly
//! refused a corpus containing what it correctly could not parse -- so
//! `intent search` exited 1 having read nothing, on a clean checkout, with 24
//! residue lines naming 12 gitignored files. Because AC-10.2 makes residue a
//! migration BLOCK, that failure propagated straight to the fleet rollout.
//!
//! **D05 is not weakened; the corpus is defined.** The derivation changed with
//! D01's reversal and the conclusion did not, which is the interesting part: it
//! used to run "durable truth is committed JSON, so an uncommittable path
//! cannot be canon", and it now runs through D34 -- the committed extract is
//! the interchange and ingest is the only door into the DB, so a path git can
//! never commit can never travel, and therefore can never become canon.
//!
//! The discriminating test is `the_same_unreadable_file_is_in_or_out_of_scope
//! _by_ignore_status_alone` -- identical bytes, opposite outcomes, differing
//! only in whether a repository exists to ignore them. Without that pairing,
//! a scan that had simply stopped reporting unparseable files would pass every
//! other test here.

use std::path::Path;
use std::process::Command;

/// A byte sequence that is not valid UTF-8, as a real `.DS_Store` is.
///
/// Spelled out because the first version of this fixture used `\x00\x01` and
/// friends, which ARE valid UTF-8 control characters -- so the file parsed
/// fine, nothing was flagged, and the test passed while proving nothing.
const NOT_UTF8: &[u8] = b"\xff\xfe\x00Bud1\xff\xfe";

struct Project {
  dir: tempfile::TempDir,
}

impl Project {
  fn new() -> Self {
    let dir = tempfile::tempdir().expect("tempdir");
    let config = dir.path().join("intent").join(".config");
    std::fs::create_dir_all(&config).expect("mkdir");
    std::fs::write(
      config.join("config.json"),
      "{\n  \"intent_version\": \"3.0.0\",\n  \"project_name\": \"Corpus\",\n  \"author\": \"cc\",\n  \"intent_dir\": \"intent\",\n  \"languages\": [\"rust\"]\n}\n",
    )
    .expect("config");
    Self { dir }
  }

  fn root(&self) -> &Path {
    self.dir.path()
  }

  /// Make it a git repository with a `.gitignore`. Real git, because the rule
  /// IS git's ignore semantics -- nested files, negation, directory-vs-file --
  /// and a reimplementation would disagree exactly where it matters.
  fn with_git(self, gitignore: &str) -> Self {
    let ok = Command::new("git")
      .args(["init", "-q"])
      .current_dir(self.root())
      .status()
      .expect("run git")
      .success();
    assert!(ok, "git init failed");
    std::fs::write(self.root().join(".gitignore"), gitignore).expect("gitignore");
    self
  }

  /// Add a `.git/info/exclude` rule: per-clone, uncommitted, and therefore
  /// NOT part of what the repository says about itself.
  fn with_local_exclude(self, rule: &str) -> Self {
    let info = self.root().join(".git").join("info");
    std::fs::create_dir_all(&info).expect("mkdir .git/info");
    std::fs::write(info.join("exclude"), rule).expect("write exclude");
    self
  }

  fn write(&self, rel: &str, bytes: &[u8]) {
    let path = self.root().join(rel);
    std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
    std::fs::write(path, bytes).expect("write");
  }

  fn thread(&self, id: &str) {
    self.write(
      &format!("intent/.canon/st/{id}.json"),
      format!(
        "{{\n  \"schema\": \"intent/thread@3.0\",\n  \"id\": \"{id}\",\n  \"title\": \"A thread\",\n  \"status\": \"wip\",\n  \"created\": \"2026-08-14\",\n  \"objective\": \"\",\n  \"context\": \"\"\n}}\n"
      )
      .as_bytes(),
    );
  }

  fn project(&self) -> intentsvcs::project::Project {
    intentsvcs::project::Project::open(self.root()).expect("open")
  }

  /// The corpus rule lives in the SCAN, which is what decides whether a file
  /// is in scope at all. `ingest::read` only ever opens `st/<ID>/thread.json`
  /// and `issues/*.json` by name, so it never sees a stray file and cannot be
  /// the seam this rule is tested at -- the first version of these tests
  /// called it and reported three false failures.
  fn unparsed(&self) -> Vec<String> {
    intentsvcs::sync::scan(self.root(), &[])
      .expect("scan")
      .into_iter()
      .filter(|e| e.state == intentsvcs::sync::FileState::Unparsed)
      .map(|e| e.path)
      .collect()
  }

  /// The full daily-driver load, which is what actually refused on a Mac.
  fn resync(&self) -> Result<intentsvcs::ingest::Canon, intentsvcs::ingest::IngestError> {
    let mut store = intentsvcs::store::Store::open_in_memory().expect("store");
    intentsvcs::ingest::resync(&self.project(), &mut store, &intentsvcs::sync::Scope::All)
  }
}

/// **The discriminating pair.** Identical bytes in identical places; the only
/// difference is whether a repository exists to ignore them.
#[test]
fn the_same_unreadable_file_is_in_or_out_of_scope_by_ignore_status_alone() {
  let ignored = Project::new().with_git(".DS_Store\n");
  ignored.thread("ST0001");
  ignored.write("intent/st/.DS_Store", NOT_UTF8);
  assert!(
    ignored.unparsed().is_empty(),
    "a gitignored path is not canon, so it cannot refuse the read: {:?}",
    ignored.unparsed()
  );
  assert!(ignored.resync().is_ok(), "and the whole load succeeds");

  let in_scope = Project::new();
  in_scope.thread("ST0001");
  in_scope.write("intent/st/.DS_Store", NOT_UTF8);
  assert_eq!(
    in_scope.unparsed(),
    vec!["intent/st/.DS_Store".to_string()],
    "with no repository there are no ignore rules, so the same file IS in scope and strict ingest still refuses it -- the corpus narrowed, the strictness did not"
  );
  assert!(in_scope.resync().is_err(), "and the load refuses");
}

/// Edge one, and getting it backwards is worse than the original bug: the rule
/// keys on IGNORED, never on untracked. A `thread.json` you created ten
/// seconds ago and have not committed is most of what a working session looks
/// like, and it must ingest.
#[test]
fn an_untracked_but_unignored_thread_still_ingests() {
  let fx = Project::new().with_git(".DS_Store\nintent/.cache/\n");
  fx.thread("ST0001");
  fx.write("intent/st/.DS_Store", NOT_UTF8);

  let status = Command::new("git")
    .args(["status", "--porcelain", "intent/.canon/st/ST0001.json"])
    .current_dir(fx.root())
    .output()
    .expect("git status");
  assert!(
    String::from_utf8_lossy(&status.stdout).starts_with("??"),
    "precondition: the canon file is untracked, never committed"
  );

  let canon = fx.resync().expect("an untracked thread is still canon");
  assert_eq!(canon.threads.len(), 1, "and it was actually read");
}

/// Edge two: no repository means no ignore rules, so the corpus degrades to
/// everything-in-scope rather than to nothing. A rule that silently emptied
/// the corpus without git would make a non-git project look like an empty one.
#[test]
fn a_project_without_git_still_has_a_corpus() {
  let fx = Project::new();
  fx.thread("ST0001");
  let canon = fx.resync().expect("ingest");
  assert_eq!(
    canon.threads.len(),
    1,
    "no git means nothing ignored, not everything ignored"
  );
}

/// The rule is general, not a `.DS_Store` special case.
///
/// `intent/.cache/intent.db` escapes today through path shape (`SKIPPED_DIRS`)
/// rather than through any rule -- clean by luck. WP-13 widens the corpus to
/// the whole project for search, at which point a binary SQLite file walks
/// into scope. This asserts an ignored binary somewhere `SKIPPED_DIRS` does
/// NOT cover, so it can only pass by the ignore rule.
#[test]
fn the_rule_is_general_and_not_a_ds_store_special_case() {
  let fx = Project::new().with_git("*.bin\n");
  fx.thread("ST0001");
  fx.write("intent/st/ST0001/artefact.bin", NOT_UTF8);

  assert!(
    !intentsvcs::sync::SKIPPED_DIRS.contains(&"artefact.bin"),
    "precondition: nothing in the path-shape skip list covers this file"
  );
  assert!(
    fx.unparsed().is_empty(),
    "an ignored binary is out of corpus wherever it sits, by rule rather than by name: {:?}",
    fx.unparsed()
  );
}

/// A negated ignore puts a file BACK in scope. This is why the rule delegates
/// to git rather than parsing `.gitignore` by hand: negation, nesting and
/// directory semantics are where a reimplementation quietly disagrees.
#[test]
fn a_negated_ignore_rule_puts_a_file_back_in_scope() {
  let fx = Project::new().with_git("*.bin\n!keep.bin\n");
  fx.thread("ST0001");
  fx.write("intent/st/ST0001/keep.bin", NOT_UTF8);

  assert_eq!(
    fx.unparsed(),
    vec!["intent/st/ST0001/keep.bin".to_string()],
    "git would commit this file, so it is in corpus, so strict ingest refuses it"
  );
}

/// **The corpus is a property of the REPOSITORY, not of the clone.**
///
/// `.git/info/exclude` is real git ignore machinery and the walker honours it
/// by default -- so a file excluded there was silently out of corpus here and
/// in corpus in a fresh clone of the same commit. Two operators, same
/// repository, different answers to "what does this project contain", and
/// under AC-10.2 different answers to "does this project migrate".
///
/// It also fails D29's derivation on its own terms: the rule is that a path
/// git can NEVER commit can never be canon, and this one is one `git add`
/// away from being committed by anybody who has not written that exclude.
#[test]
fn a_clone_local_exclude_does_not_shrink_the_corpus() {
  let fx = Project::new()
    .with_git("*.bin\n")
    .with_local_exclude("local.dat\n");
  fx.thread("ST0001");
  fx.write("intent/st/ST0001/local.dat", NOT_UTF8);
  fx.write("intent/st/ST0001/shared.bin", NOT_UTF8);

  assert_eq!(
    fx.unparsed(),
    vec!["intent/st/ST0001/local.dat".to_string()],
    "the committed `.gitignore` rule takes `shared.bin` out of corpus; the \
     clone-local exclude must NOT take `local.dat` out, because a fresh clone \
     would carry it"
  );
}

/// Residue is reported ONCE per finding.
///
/// The refusal rendered twice -- once as the error variant's own message and
/// again as its `source` -- so 12 findings printed as 24 lines while the
/// summary correctly said 12. AC-10.2 shows a migrator its residue per line,
/// so a migration would have reported twelve problems as twenty-four.
#[test]
fn each_finding_is_reported_exactly_once() {
  let fx = Project::new();
  fx.thread("ST0001");
  for name in ["a", "b", "c"] {
    fx.write(&format!("intent/st/{name}.DS_Store"), NOT_UTF8);
  }

  let err = fx.resync().expect_err("refused");
  let rendered = format!("{err}");
  let lines = rendered.lines().filter(|l| l.contains("residue:")).count();
  assert_eq!(
    lines, 3,
    "three unreadable files, three residue lines -- got:\n{rendered}"
  );
}

/// **`AC-08.5`'s SCOPE HALF: THE PREDICATE AND THE WALK CANNOT DISAGREE.**
///
/// `intentd`'s watcher has to decide whether a path somebody else just changed
/// is one a sync would read, and it cannot walk the tree to find out. So
/// `sync::Scanned` answers that question, and `sync::scan` was rewritten to ask
/// the same object -- because **a watcher carrying its own idea of scope drifts
/// in the direction that loops.**
///
/// **THE LOOP IS THE REASON `AC-08.5` SAYS `gitignore-aware` AT ALL.** The store
/// lives at `intent/.cache/intent.db`, INSIDE the watched tree, and every ingest
/// writes it. A watcher that triggered on any change under `intent/` would
/// trigger on the write its own ingest just made, forever, on an idle machine.
///
/// **ONE ARM COMPARES THE TWO AND THE OTHER IS THE ONE THAT MATTERS.** Asking
/// only "is everything the scan returns also `includes`?" is a comparison whose
/// two sides now share a source, which is an identity rather than a
/// measurement. The load-bearing arm is the opposite direction -- paths the scan
/// NEVER SEES must be excluded -- because the scan is silent about them by
/// construction and so cannot be the reference for them.
#[test]
fn the_scope_predicate_and_the_walk_cannot_disagree() {
  let p = Project::new().with_git("*.log\n");
  p.thread("ST0001");
  p.write("AGENTS.md", b"# agents\n");
  // Out of scope, one per mechanism, so a single rule going missing is visible.
  p.write("intent/.cache/intent.db", b"SQLite format 3\x00"); // SKIPPED_DIRS
  p.write("intent/.backup/db-2026-08-30.sqlite", b"snapshot"); // SKIPPED_DIRS
  p.write("intent/noise.log", b"ignored by .gitignore\n"); // gitignore
  p.write("native/rust/src/main.rs", b"fn main() {}\n"); // outside the scope

  let scope = intentsvcs::sync::Scanned::for_root(p.root());

  let scanned: Vec<std::path::PathBuf> = intentsvcs::sync::scan(p.root(), &[])
    .expect("scan")
    .into_iter()
    .map(|e| p.root().join(e.path))
    .collect();
  assert!(
    scanned.len() >= 2,
    "the fixture scanned {} path(s), which is too few for either direction to mean anything: {scanned:?}",
    scanned.len()
  );

  let disagreed: Vec<&std::path::PathBuf> = scanned
    .iter()
    .filter(|path| !scope.includes(path))
    .collect();
  assert!(
    disagreed.is_empty(),
    "the walk read these and the predicate excludes them, so the watcher would ignore edits the sync engine acts on: {disagreed:?}"
  );

  // **THE ARM THE SCAN CANNOT SUPPLY.** Each of these exists on disk and is
  // out of scope for a DIFFERENT reason; asserting them by name is what makes
  // this a test of the predicate rather than of its agreement with a walk that
  // never looked.
  let out_of_scope = [
    "intent/.cache/intent.db",
    "intent/.backup/db-2026-08-30.sqlite",
    "intent/noise.log",
    "native/rust/src/main.rs",
  ];
  for rel in out_of_scope {
    let path = p.root().join(rel);
    assert!(
      path.exists(),
      "the fixture does not have `{rel}` on disk, so excluding it proves nothing"
    );
    assert!(
      !scope.includes(&path),
      "`{rel}` is out of the sync's scope and the predicate includes it. For `intent/.cache/intent.db` this is not a tidiness matter: the ingest WRITES that file, so a watcher acting on it re-triggers itself forever"
    );
  }

  // And the positive control for the predicate itself: a real in-scope path
  // must be included, or every exclusion above holds for the wrong reason.
  assert!(
    scope.includes(&p.root().join("intent/.canon/st/ST0001.json")),
    "the predicate excludes a canon thread file, so it is not answering the question it claims to"
  );
  assert!(
    scope.includes(&p.root().join("AGENTS.md")),
    "the predicate excludes a ROOT_FILES member"
  );
}
