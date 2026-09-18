//! ST0078 WP-05 through the real binary and real git: **after a pull, one
//! command brings a clone and its store back into step** (AT-05.1 to AT-05.4).
//!
//! hv, 2026-09-18: nobody should type ten commands to bring a clone and its
//! store back into step. Bare `intent sync` prints the plan and writes
//! nothing; `intent sync --apply` applies it. Alice and Bob clone one origin
//! under one isolated `HOME`, with no daemon and no hooks, so every change
//! here is the verb's.
//!
//! **THE MERGE ARMS DRIVE THE CASE P2 COULD NOT**: P2's renumber repairs a
//! collision before the merge. Here the merge has already stopped on it, the
//! old id's paths hold the pulled side, and the plain verb would move them.

use std::path::{Path, PathBuf};
use std::process::Command;

struct Said {
  text: String,
  code: i32,
}

fn intent(args: &[&str], cwd: &Path, home: &Path) -> Said {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(cwd)
    .env("HOME", home)
    .env_remove("XDG_CONFIG_HOME")
    .env_remove("XDG_DATA_HOME")
    .env_remove("XDG_STATE_HOME")
    .env_remove("XDG_RUNTIME_DIR")
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run the v3 binary");
  Said {
    text: format!(
      "{}{}",
      String::from_utf8_lossy(&out.stdout),
      String::from_utf8_lossy(&out.stderr)
    ),
    code: out.status.code().unwrap_or(-1),
  }
}

fn intent_ok(args: &[&str], cwd: &Path, home: &Path) -> String {
  let said = intent(args, cwd, home);
  assert_eq!(said.code, 0, "`intent {}`:\n{}", args.join(" "), said.text);
  said.text
}

/// Git with an identity and no hooks, so the fixture's commits are the
/// commits and nothing else.
fn git(args: &[&str], cwd: &Path) -> (String, bool) {
  let out = Command::new("git")
    .args(["-c", "user.name=t", "-c", "user.email=t@example.com"])
    .args(["-c", "core.hooksPath=/dev/null"])
    .args(args)
    .current_dir(cwd)
    .output()
    .expect("run git");
  (
    format!(
      "{}{}",
      String::from_utf8_lossy(&out.stdout),
      String::from_utf8_lossy(&out.stderr)
    ),
    out.status.success(),
  )
}

fn git_ok(args: &[&str], cwd: &Path) -> String {
  let (out, success) = git(args, cwd);
  assert!(success, "`git {}` failed:\n{out}", args.join(" "));
  out
}

/// A bare origin, Alice's clone with an initialised project pushed to it, and
/// Bob's clone of that.
struct Team {
  _dir: tempfile::TempDir,
  home: tempfile::TempDir,
  origin: PathBuf,
  alice: PathBuf,
  bob: PathBuf,
}

impl Team {
  fn new() -> Self {
    let dir = tempfile::tempdir().expect("tempdir");
    let home = tempfile::tempdir().expect("tempdir");
    let (origin, alice, bob) = (
      dir.path().join("origin.git"),
      dir.path().join("alice"),
      dir.path().join("bob"),
    );
    git_ok(
      &["init", "-q", "--bare", "-b", "main", "origin.git"],
      dir.path(),
    );
    git_ok(&["clone", "-q", "origin.git", "alice"], dir.path());
    intent_ok(&["init", "Team"], &alice, home.path());
    git_ok(&["add", "-A"], &alice);
    git_ok(&["commit", "-q", "-m", "intent init"], &alice);
    git_ok(&["push", "-q", "origin", "HEAD:main"], &alice);
    git_ok(&["clone", "-q", "origin.git", "bob"], dir.path());
    intent_ok(&["sync", "--apply"], &bob, home.path());
    Self {
      _dir: dir,
      home,
      origin,
      alice,
      bob,
    }
  }

  fn home(&self) -> &Path {
    self.home.path()
  }

  fn commit(&self, clone: &Path, message: &str) {
    git_ok(&["add", "-A"], clone);
    git_ok(&["commit", "-q", "-m", message], clone);
  }

  /// Both clones mint `ST0001`. Bob starts his, so it is realised and
  /// declared; the merge then stops on the canon add/add.
  fn both_mint_st0001_and_bob_pulls(&self) -> String {
    intent_ok(&["st", "new", "Alice's next"], &self.alice, self.home());
    self.commit(&self.alice, "alice: ST0001");
    git_ok(&["push", "-q", "origin", "HEAD:main"], &self.alice);
    intent_ok(&["st", "new", "Bob's next"], &self.bob, self.home());
    intent_ok(&["st", "start", "ST0001"], &self.bob, self.home());
    self.commit(&self.bob, "bob: ST0001");
    let (merge, merged) = git(&["pull", "-q", "--no-rebase", "origin", "main"], &self.bob);
    assert!(!merged, "the fixture needs the merge to stop:\n{merge}");
    merge
  }
}

fn unmerged(clone: &Path) -> String {
  git_ok(&["diff", "--name-only", "--diff-filter=U"], clone)
}

/// The digest a printed plan names, `(plan <digest>)`.
fn digest_of(plan: &str) -> String {
  plan
    .split("(plan ")
    .nth(1)
    .and_then(|rest| rest.split(')').next())
    .unwrap_or_default()
    .to_string()
}

/// AT-05.1: **the plan is ordered steps with their recoverability, under a
/// digest, and writes nothing; `--apply --plan <digest>` refuses a tree that
/// has moved.** Bob fetches Alice's commit without pulling, so the first step
/// is the branch behind its upstream -- reported, and nothing is run.
#[test]
fn the_plan_writes_nothing_and_an_apply_after_the_tree_moved_is_refused() {
  let team = Team::new();
  intent_ok(
    &["st", "new", "Release checklist"],
    &team.alice,
    team.home(),
  );
  team.commit(&team.alice, "alice: ST0001");
  git_ok(&["push", "-q", "origin", "HEAD:main"], &team.alice);
  git_ok(&["fetch", "-q"], &team.bob);

  let status_before = git_ok(&["status", "--porcelain"], &team.bob);
  let head_before = git_ok(&["rev-parse", "HEAD"], &team.bob);
  let plan = intent_ok(&["sync"], &team.bob, team.home());
  assert!(plan.starts_with("plan: "), "{plan}");
  assert!(
    plan.contains(". behind (quiet): origin/main holds 1 commit(s) this branch does not"),
    "the branch behind its upstream is the first thing said: {plan}"
  );
  assert!(
    plan
      .lines()
      .last()
      .is_some_and(|l| l.contains(". doctor (quiet): ")),
    "and doctor is last: {plan}"
  );
  let digest = digest_of(&plan);
  assert_eq!(digest.len(), 64, "the plan names its digest: {plan}");
  assert_eq!(
    git_ok(&["status", "--porcelain"], &team.bob),
    status_before,
    "the plan wrote nothing git can see"
  );

  // The tree moves between the plan and the apply.
  std::fs::write(
    team.bob.join("intent/moved.md"),
    "a file the plan never read\n",
  )
  .expect("move the tree");
  let refused = intent(
    &["sync", "--apply", "--plan", &digest],
    &team.bob,
    team.home(),
  );
  assert_eq!(refused.code, 1, "{}", refused.text);
  assert!(
    refused
      .text
      .contains("the plan you were shown read the tree as"),
    "{}",
    refused.text
  );
  assert!(refused.text.contains("remedy:"), "{}", refused.text);

  // Against the tree as it now stands, the apply runs, says the branch is
  // behind, and pulls nothing.
  let replanned = intent_ok(&["sync"], &team.bob, team.home());
  let applied = intent(
    &["sync", "--apply", "--plan", &digest_of(&replanned)],
    &team.bob,
    team.home(),
  );
  assert!(
    applied
      .text
      .contains("note: behind: origin/main holds 1 commit(s)"),
    "{}",
    applied.text
  );
  assert_eq!(
    git_ok(&["rev-parse", "HEAD"], &team.bob),
    head_before,
    "intent never pulls"
  );
}

/// AT-05.2 and AT-05.4: **a merge stopped on an id both clones minted is
/// repaired by `--apply --yes`**: this clone's thread moves to the next id free
/// in the store and the tree, the pulled one keeps its id, the store takes the
/// merged canon, the unmerged views are regenerated from it and staged, and
/// doctor's verdict is the exit code. **Only what it regenerated is staged,
/// and it never commits, pulls or pushes.**
#[test]
fn a_merge_stopped_on_a_twice_minted_id_is_repaired_and_only_what_was_regenerated_is_staged() {
  let team = Team::new();
  let merge = team.both_mint_st0001_and_bob_pulls();
  let bob = &team.bob;
  assert!(
    unmerged(bob).contains("intent/.canon/st/ST0001.json"),
    "the fixture stops on the canon add/add:\n{merge}"
  );
  std::fs::write(bob.join("notes.txt"), "Bob's own file\n").expect("an untracked file");
  let head = git_ok(&["rev-parse", "HEAD"], bob);
  let origin_main = git_ok(&["rev-parse", "main"], &team.origin);

  let plan = intent_ok(&["sync"], bob, team.home());
  assert!(
    plan.contains(
      ". renumber (reversible): both sides minted steel thread ST0001: this clone's moves to ST0002"
    ),
    "{plan}"
  );
  assert!(
    plan.contains(". ingest (quiet): take the merged canon into the store; what it takes can be read once the conflicts are resolved"),
    "the ingest waits on the canon conflict: {plan}"
  );

  // No terminal and no `--yes`: the renumber is left, and what reads the
  // merged canon waits on it.
  let left = intent(&["sync", "--apply"], bob, team.home());
  let line = left
    .text
    .lines()
    .find(|l| l.starts_with("left: "))
    .unwrap_or_default()
    .to_string();
  assert!(
    line.contains("renumber (reversible)") && line.contains("ingest (waits on the conflicts)"),
    "{}",
    left.text
  );
  assert!(
    unmerged(bob).contains("intent/.canon/st/ST0001.json"),
    "nothing was resolved without an answer"
  );

  // `--yes` answers the reversible steps.
  let repaired = intent(&["sync", "--apply", "--yes"], bob, team.home());
  assert!(
    repaired
      .text
      .contains("ok: renumbered this clone's ST0001 to ST0002, kept the pulled ST0001"),
    "{}",
    repaired.text
  );
  assert!(
    repaired.text.contains("ok: regenerated and staged"),
    "the unmerged views are regenerated and staged: {}",
    repaired.text
  );
  assert_eq!(
    unmerged(bob).trim(),
    "",
    "nothing is left unmerged: {}",
    repaired.text
  );
  let last = repaired.text.lines().last().unwrap_or_default();
  assert!(
    last.starts_with("doctor: 0 finding(s)") && repaired.code == 0,
    "doctor runs last and its verdict is the exit code (rc {}): {}",
    repaired.code,
    repaired.text
  );

  // AT-05.4: staged is intent's own, the untracked file is untouched, and no
  // commit, pull or push happened.
  let staged = git_ok(&["diff", "--cached", "--name-only"], bob);
  for path in staged.lines() {
    assert!(
      path.starts_with("intent/"),
      "sync staged a path it did not regenerate: {path}"
    );
  }
  assert!(
    git_ok(&["status", "--porcelain"], bob).contains("?? notes.txt"),
    "an untracked file stays untracked"
  );
  assert_eq!(
    git_ok(&["rev-parse", "HEAD"], bob),
    head,
    "sync never commits"
  );
  assert!(
    git(&["rev-parse", "-q", "--verify", "MERGE_HEAD"], bob).1,
    "the merge is still the person's to commit"
  );
  assert_eq!(
    git_ok(&["rev-parse", "main"], &team.origin),
    origin_main,
    "sync never pushes"
  );

  // The person commits the merge, and both threads are there.
  git_ok(&["commit", "-q", "--no-edit"], bob);
  let list = intent_ok(&["st", "list", "--status", "all"], bob, team.home());
  assert!(
    list.contains("ST0001") && list.contains("Alice's next"),
    "{list}"
  );
  assert!(
    list.contains("ST0002") && list.contains("Bob's next"),
    "{list}"
  );
}

/// AT-05.3: **a canon content conflict always asks a person, and `--yes`
/// does not answer it.** With no terminal it is left and named, and the steps
/// that read the merged canon wait. Answered -- here by the library's `ask`,
/// the one door the terminal prompt also goes through -- it takes the side
/// chosen and stages it.
#[test]
fn a_canon_content_conflict_needs_a_person_and_yes_does_not_answer_it() {
  let team = Team::new();
  intent_ok(&["st", "new", "Shared"], &team.alice, team.home());
  team.commit(&team.alice, "alice: ST0001");
  git_ok(&["push", "-q", "origin", "HEAD:main"], &team.alice);
  git_ok(&["pull", "-q", "--no-rebase", "origin", "main"], &team.bob);
  intent_ok(&["sync", "--apply"], &team.bob, team.home());

  intent_ok(
    &["set", "ST0001", "title", "Alice's title"],
    &team.alice,
    team.home(),
  );
  team.commit(&team.alice, "alice: retitle");
  git_ok(&["push", "-q", "origin", "HEAD:main"], &team.alice);
  intent_ok(
    &["set", "ST0001", "title", "Bob's title"],
    &team.bob,
    team.home(),
  );
  team.commit(&team.bob, "bob: retitle");
  let (merge, merged) = git(&["pull", "-q", "--no-rebase", "origin", "main"], &team.bob);
  assert!(!merged, "the fixture needs the merge to stop:\n{merge}");
  assert!(unmerged(&team.bob).contains("intent/.canon/st/ST0001.json"));

  let left = intent(&["sync", "--apply", "--yes"], &team.bob, team.home());
  let line = left
    .text
    .lines()
    .find(|l| l.starts_with("left: "))
    .unwrap_or_default()
    .to_string();
  assert!(
    line.contains("take a side (non-reversible)")
      && line.contains("ingest (waits on the conflicts)"),
    "`--yes` does not answer a non-reversible step: {}",
    left.text
  );
  assert!(
    unmerged(&team.bob).contains("intent/.canon/st/ST0001.json"),
    "and the conflict stands"
  );

  let project = intentsvcs::project::Project::open(&team.bob).expect("Bob's project");
  let ctx = intentsvcs::facade::FacadeContext {
    principal: "test".to_string(),
    project_id: project.config().project_id.clone().unwrap_or_default(),
    version: env!("CARGO_PKG_VERSION").to_string(),
  };
  let mut facade = intentsvcs::facade::Facade::open(project, ctx).expect("the facade opens");
  let mut asked = Vec::new();
  let applied = facade
    .sync_apply(
      &intentsvcs::sync::Scope::All,
      intentsvcs::plan::Asking {
        yes: false,
        terminal: true,
        shown: None,
      },
      &mut |step| {
        asked.push(step.name());
        match step.action {
          intentsvcs::plan::Action::TakeSide { .. } => {
            intentsvcs::plan::Decision::Take(intentsvcs::plan::Side::Theirs)
          }
          _ => intentsvcs::plan::Decision::Run,
        }
      },
    )
    .expect("the apply runs");
  assert!(
    asked.contains(&"take a side"),
    "the person was asked: {asked:?}"
  );
  assert!(
    applied
      .done
      .iter()
      .any(|l| l.starts_with("took theirs for intent/.canon/st/ST0001.json")),
    "{:?}",
    applied.done
  );
  assert!(
    !unmerged(&team.bob).contains("intent/.canon/st/ST0001.json"),
    "the side taken is staged"
  );
  let shown = intent_ok(&["st", "show", "ST0001"], &team.bob, team.home());
  assert!(shown.contains("Alice's title"), "{shown}");
}
