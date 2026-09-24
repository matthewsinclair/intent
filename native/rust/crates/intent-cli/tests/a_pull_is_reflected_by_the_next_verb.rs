//! ST0078 AT-03.1 (AC-03.1): **after a `git pull`, a branch checkout or a
//! rewrite, with no daemon running, the next verb answers from the merged
//! canon.**
//!
//! Measured on 2026-09-18 before this was built (ST0078 `design.md`, E3): two
//! clones, a bare origin, no daemon. After a fast-forward pull brought ST0002,
//! `intent st show ST0002` answered `no steel thread ST0002 in this project` at
//! rc=1, and every verb went on answering from the pre-pull store until someone
//! ran the whole-store restore.
//!
//! **EVERYTHING HERE IS THE SHIPPED PATH, DRIVEN.** Real git, a real bare
//! origin, the hooks wired by `intent claude upgrade --apply` from the
//! template the binary's own install root serves, and the `intent` those hooks
//! find on PATH. A test that called the facade would pass while a hook that
//! never ran, or ran the wrong flag, left the store behind.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::common::short_dir;

/// Two clones of one bare origin: Alice writes, Bob pulls with the hooks wired.
struct Team {
  dir: PathBuf,
  home: PathBuf,
}

/// What a command printed, on both streams, and its exit code.
struct Ran {
  said: String,
  code: i32,
}

impl Team {
  /// Alice's project carries ST0001 before Bob clones it, so Bob's store holds
  /// something when the pulls below arrive. **An empty store is cold**, and
  /// the next open warms it from canon whatever the hooks do, which would let
  /// every arm below pass with the hooks deleted.
  fn new() -> Team {
    let dir = short_dir("pull");
    let home = dir.join("home");
    std::fs::create_dir_all(&home).expect("the isolated HOME");
    std::fs::write(
      home.join(".gitconfig"),
      "[user]\n  email = t@example.com\n  name = t\n[init]\n  defaultBranch = main\n",
    )
    .expect("git's identity for the fixture commits");
    let team = Team { dir, home };

    team.git(&team.dir, &["init", "-q", "--bare", "origin.git"]);
    let alice = team.alice();
    std::fs::create_dir_all(&alice).expect("Alice's clone");
    team.git(&alice, &["init", "-q"]);
    team.intent_ok(&alice, &["init", "Team"]);
    team.intent_ok(&alice, &["st", "new", "Onboarding guide"]);
    team.commit(&alice, "init");
    team.git(&alice, &["remote", "add", "origin", "../origin.git"]);
    team.git(&alice, &["push", "-q", "-u", "origin", "main"]);

    team.git(&team.dir, &["clone", "-q", "origin.git", "bob"]);
    let bob = team.bob();
    team.intent_ok(&bob, &["claude", "upgrade", "--apply", "--skip-settings"]);
    // The shim that verb installs refuses every commit until the install
    // pointer names an install, and the post-merge hook's doctor line says so
    // (issue 0570), which is a finding the arms below would read as the pull's
    // own output. So this HOME records one, as a bootstrapped machine does.
    team.intent_ok(&bob, &["bootstrap"]);
    let listed = team.intent(&bob, &["st", "list", "--status", "all"]);
    assert!(
      listed.said.contains("ST0001"),
      "Bob's store warmed from the clone's canon: {}",
      listed.said
    );
    team
  }

  fn alice(&self) -> PathBuf {
    self.dir.join("alice")
  }

  fn bob(&self) -> PathBuf {
    self.dir.join("bob")
  }

  /// `PATH` with the binary under test first, so the hooks find THIS `intent`.
  fn path(&self) -> String {
    let bin = crate::common::intent_path()
      .parent()
      .expect("the binary's directory");
    format!("{}:/usr/bin:/bin", bin.display())
  }

  fn command(&self, program: impl AsRef<std::ffi::OsStr>, cwd: &Path, path: &str) -> Command {
    let mut command = testkit::fixtured_command(program);
    command
      .current_dir(cwd)
      .env("HOME", &self.home)
      .env("PATH", path)
      .env_remove("XDG_CONFIG_HOME")
      .env_remove("XDG_DATA_HOME")
      .env_remove("XDG_STATE_HOME")
      .env_remove("XDG_RUNTIME_DIR")
      .env_remove("GIT_DIR")
      .env_remove("GIT_INDEX_FILE")
      .env_remove("GIT_WORK_TREE");
    command
  }

  fn ran(&self, mut command: Command) -> Ran {
    let out = command.output().expect("the command runs");
    Ran {
      said: format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
      ),
      code: out.status.code().unwrap_or(-1),
    }
  }

  fn intent(&self, cwd: &Path, args: &[&str]) -> Ran {
    let mut command = self.command(crate::common::intent_path(), cwd, &self.path());
    command.args(args).stdin(testkit::lifeline_for(args));
    self.ran(command)
  }

  fn intent_ok(&self, cwd: &Path, args: &[&str]) {
    let ran = self.intent(cwd, args);
    assert_eq!(ran.code, 0, "intent {args:?} failed: {}", ran.said);
  }

  /// git with the binary under test on PATH, which is how the hooks reach it.
  fn git_with_path(&self, cwd: &Path, args: &[&str], path: &str) -> Ran {
    let mut command = self.command("git", cwd, path);
    command.args(args);
    self.ran(command)
  }

  fn git(&self, cwd: &Path, args: &[&str]) -> Ran {
    let ran = self.git_with_path(cwd, args, &self.path());
    assert_eq!(ran.code, 0, "git {args:?} failed: {}", ran.said);
    ran
  }

  /// `--no-verify`: the commit gate is not this test's subject, and it needs a
  /// bootstrapped install pointer this isolated HOME does not carry.
  fn commit(&self, cwd: &Path, message: &str) {
    self.git(cwd, &["add", "-A"]);
    self.git(cwd, &["commit", "-q", "--no-verify", "-m", message]);
  }

  /// Alice mints a thread and pushes it.
  fn alice_pushes(&self, title: &str) {
    let alice = self.alice();
    self.intent_ok(&alice, &["st", "new", title]);
    self.commit(&alice, title);
    self.git(&alice, &["push", "-q"]);
  }
}

impl Drop for Team {
  fn drop(&mut self) {
    let _ = std::fs::remove_dir_all(&self.dir);
  }
}

#[test]
fn a_pull_brings_the_pulled_thread_into_the_store_and_says_so_once() {
  let team = Team::new();
  let bob = team.bob();
  for hook in ["post-merge", "post-checkout", "post-rewrite"] {
    let carrier = bob.join(".git/hooks").join(format!("{hook}.intent"));
    assert!(
      carrier.is_file(),
      "`claude upgrade --apply` wired no {hook} carrier"
    );
  }

  team.alice_pushes("Release checklist");
  let pulled = team.git(&bob, &["pull", "-q"]);
  assert_eq!(
    pulled
      .said
      .lines()
      .filter(|l| l.starts_with("intent ("))
      .collect::<Vec<_>>(),
    // The pull carries Alice's `st.new` as a committed event file too (ST0078
    // P1), and the same line names it.
    vec![
      "intent (post-merge): took 1 change(s) from the files into the store: ST0002; and 1 event file(s)"
    ],
    "one line, naming what the pull changed in the store: {}",
    pulled.said
  );

  let shown = team.intent(&bob, &["st", "show", "ST0002"]);
  assert_eq!(
    shown.code, 0,
    "the next verb answers from the merged canon: {}",
    shown.said
  );
  assert!(shown.said.contains("Release checklist"), "{}", shown.said);
}

#[test]
fn a_pull_that_changes_no_thread_prints_nothing() {
  let team = Team::new();
  let alice = team.alice();
  std::fs::write(alice.join("README.md"), "a file Intent does not model\n").expect("write");
  team.commit(&alice, "readme");
  team.git(&alice, &["push", "-q"]);

  let pulled = team.git(&team.bob(), &["pull", "-q"]);
  assert!(
    !pulled.said.contains("intent ("),
    "a pull that changed nothing in the store printed from the hook: {}",
    pulled.said
  );
}

/// Issue 0456, ic's repro from the P4 drive: **a branch switch leaves no
/// untracked canon behind, and the branch merges.** main is committed before
/// any thread exists, so it never carried `project.json`. A write on a branch
/// used to create that file, git removed it on the switch back, and the hook's
/// ingest wrote it again untracked -- and the merge then refused over it.
#[test]
fn a_branch_switch_leaves_no_untracked_canon_and_the_branch_merges() {
  let dir = short_dir("switch");
  let home = dir.join("home");
  std::fs::create_dir_all(&home).expect("the isolated HOME");
  std::fs::write(
    home.join(".gitconfig"),
    "[user]\n  email = t@example.com\n  name = t\n[init]\n  defaultBranch = main\n",
  )
  .expect("git's identity for the fixture commits");
  let team = Team { dir, home };
  let repo = team.dir.join("solo");
  std::fs::create_dir_all(&repo).expect("the repository");
  team.git(&repo, &["init", "-q"]);
  team.intent_ok(&repo, &["init", "Solo"]);
  team.intent_ok(&repo, &["claude", "upgrade", "--apply", "--skip-settings"]);
  team.commit(&repo, "init");

  team.git(&repo, &["switch", "-q", "-c", "b"]);
  team.intent_ok(&repo, &["st", "new", "On a branch"]);
  team.commit(&repo, "b: ST0001");
  team.git(&repo, &["switch", "-q", "main"]);

  let status = team.git(&repo, &["status", "--porcelain"]);
  assert!(
    !status.said.contains("?? intent/"),
    "the switch left an untracked file under intent/: {}",
    status.said
  );
  let merged = team.git(&repo, &["merge", "-q", "--no-edit", "b"]);
  assert_eq!(merged.code, 0, "{}", merged.said);
  let listed = team.intent(&repo, &["st", "list", "--status", "all"]);
  assert!(listed.said.contains("On a branch"), "{}", listed.said);
}

#[test]
fn a_branch_checkout_is_ingested_and_a_file_checkout_is_not() {
  let team = Team::new();
  let bob = team.bob();
  team.alice_pushes("Release checklist");
  team.git(&bob, &["pull", "-q"]);

  // Back to the commit before ST0002: a branch checkout, git's flag 1.
  let back = team.git(&bob, &["switch", "-q", "-c", "before", "HEAD~1"]);
  assert!(
    back.said.contains(
      "intent (post-checkout): took 1 change(s) from the files into the store: ST0002 (removed)"
    ),
    "a branch checkout that removed ST0002's canon file takes the removal: {}",
    back.said
  );
  let gone = team.intent(&bob, &["st", "show", "ST0002"]);
  assert_ne!(gone.code, 0, "the store answers from the checked-out canon");

  // A file checkout, git's flag 0, moves no branch and must run nothing.
  let file = team.git(&bob, &["checkout", "--", "intent/todo.md"]);
  assert!(
    !file.said.contains("intent ("),
    "a file checkout ran the ingest: {}",
    file.said
  );
}

/// Issue 0483: **a fresh checkout builds no store, and a branch switch in it
/// still syncs.** `git worktree add` runs post-checkout with the null object id
/// as the previous HEAD, and the pass would build a whole store in what is
/// usually a throwaway tree.
#[test]
fn a_fresh_worktree_builds_no_store_and_a_switch_inside_it_still_syncs() {
  let team = Team::new();
  let bob = team.bob();
  team.alice_pushes("Release checklist");
  team.git(&bob, &["pull", "-q"]);

  let tree = team.dir.join("bobtree");
  let added = team.git(
    &bob,
    &[
      "worktree",
      "add",
      "-q",
      "--detach",
      tree.to_str().expect("utf-8 path"),
      "HEAD",
    ],
  );
  let store = tree.join("intent").join(".cache").join("intent.db");
  assert!(
    added
      .said
      .contains("intent (post-checkout): a fresh checkout"),
    "a fresh worktree does not say why it skipped: {}",
    added.said
  );
  assert!(
    !store.exists(),
    "a fresh worktree built a store: {}",
    added.said
  );

  // A branch switch inside that worktree passes a real previous HEAD.
  let switched = team.git(&tree, &["switch", "-q", "-c", "before", "HEAD~1"]);
  assert!(
    !switched.said.contains("a fresh checkout"),
    "a branch switch was taken for a fresh checkout: {}",
    switched.said
  );
  assert!(
    store.exists(),
    "a branch switch inside the worktree ran no pass: {}",
    switched.said
  );
}

#[test]
fn a_hook_that_cannot_do_its_job_says_so_in_one_line_and_never_fails_the_pull() {
  let team = Team::new();
  let bob = team.bob();
  team.alice_pushes("Release checklist");

  // No `intent` on PATH: git is still there, the binary is not.
  let pulled = team.git_with_path(&bob, &["pull", "-q"], "/usr/bin:/bin");
  assert_eq!(pulled.code, 0, "a hook failed a pull: {}", pulled.said);
  let said: Vec<&str> = pulled
    .said
    .lines()
    .filter(|l| l.starts_with("intent ("))
    .collect();
  assert_eq!(said.len(), 1, "one line, not silence: {}", pulled.said);
  assert!(
    said[0].contains("NOT brought up to date") && said[0].contains("intent sync --apply"),
    "it names the failure and the command to run: {}",
    said[0]
  );

  // A pass that refuses: the store file is not a database.
  team.alice_pushes("Third");
  std::fs::write(bob.join("intent/.cache/intent.db"), "not a database").expect("damage");
  let _ = std::fs::remove_file(bob.join("intent/.cache/intent.db-wal"));
  let _ = std::fs::remove_file(bob.join("intent/.cache/intent.db-shm"));
  let refused = team.git(&bob, &["pull", "-q"]);
  assert!(
    refused.said.lines().any(|l| l
      .starts_with("intent (post-merge): the store was NOT brought up to date")
      && l.contains("exited 1")),
    "a refusing pass is reported, in one line, with its exit code: {}",
    refused.said
  );
}

/// **THE HOOK READS THE VERB'S WORDS, SO THE WORDS ARE HELD HERE.** The
/// template prints on `ok: took` and `left: `, tells a `doctor:` verdict from a
/// refusal, and calls `sync --apply`; the flag's
/// spelling lives in the dispatch table (it was `--ingest` until hv ruled the
/// one-command shape on 2026-09-18), so a rename that missed the template fails here, not on a pull.
#[test]
fn the_hook_template_speaks_the_verbs_words() {
  let template = std::fs::read_to_string(
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../lib/templates/hooks/post-pull.sh"),
  )
  .expect("the hook template");
  assert!(template.contains("intent sync --apply"), "{template}");

  let table: serde_json::Value =
    serde_json::from_str(intent_cli::dispatch::TABLE).expect("the dispatch table parses");
  let sync = find_row(&table, "sync").expect("the table has a sync row");
  assert!(
    sync["flags"]
      .as_array()
      .expect("sync declares flags")
      .iter()
      .any(|f| f["spellings"]
        .as_array()
        .is_some_and(|s| s.iter().any(|s| s == "--apply"))),
    "the template calls a flag the table does not declare"
  );

  let took = intentsvcs::sync::ingested(&["ST0002".to_string()], &[]);
  assert!(
    took.starts_with("took ") && template.contains("\"ok: took \"*)"),
    "the template prints on the word the verb begins with when it took something: {took}"
  );
  assert!(
    template.contains("\"left: \"*)") && template.contains("^doctor: "),
    "and on the line naming what was left, and tells doctor's verdict from a refusal"
  );
  assert!(
    !intentsvcs::sync::ingested(&[], &[]).starts_with("took"),
    "and a pass that took nothing does not begin with it"
  );
}

/// The `sync` row wherever the table keeps it.
fn find_row<'a>(value: &'a serde_json::Value, path: &str) -> Option<&'a serde_json::Value> {
  match value {
    serde_json::Value::Object(map) => {
      if map.get("path").and_then(|p| p.as_str()) == Some(path) && map.contains_key("flags") {
        return Some(value);
      }
      map.values().find_map(|v| find_row(v, path))
    }
    serde_json::Value::Array(items) => items.iter().find_map(|v| find_row(v, path)),
    _ => None,
  }
}

/// Issue 0554 (b): **A BOARD A PULL BRINGS AHEAD OF THE STORE IS KEPT, NAMED,
/// AND CARRIED BY THE VERB NAMED.** The hooks never take board rows into the
/// store (0216), and their pass used to write the store's board over the
/// pulled `board.json`: the teammate's message was deleted from the file with
/// no line saying so, and the remedy then named would have finished the loss.
/// Now the file is left, the hook's `left:` line names it and the verb, every
/// board write refuses until it runs, and running it puts the message on the
/// board.
#[test]
fn a_pulled_board_is_kept_and_named_until_it_is_carried() {
  let team = Team::new();
  let (alice, bob) = (team.alice(), team.bob());
  team.intent_ok(
    &bob,
    &[
      "wb",
      "register",
      "bo",
      "--name",
      "Bob's agent",
      "--role",
      "worker",
    ],
  );
  team.commit(&bob, "wb: bo");
  team.git(&bob, &["push", "-q"]);

  team.git(&alice, &["pull", "-q"]);
  team.intent_ok(&alice, &["sync", "--to-store"]);
  team.intent_ok(
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
  team.intent_ok(
    &alice,
    &[
      "wb",
      "ask",
      "bo",
      "The onboarding guide is yours",
      "--node",
      "al",
    ],
  );
  // A thread change rides with the message, so the pull's pass runs the ingest
  // whose projection wrote the store's board over the file (driven).
  team.intent_ok(&alice, &["st", "new", "Release checklist"]);
  team.commit(&alice, "wb: al asks bo");
  team.git(&alice, &["push", "-q"]);

  let pulled = team.git(&bob, &["pull", "-q"]);
  let board = bob.join("intent/whiteboard/bo/board.json");
  assert!(
    std::fs::read_to_string(&board)
      .expect("Bob's board.json is on disk")
      .contains("The onboarding guide is yours"),
    "the pull's pass wrote the store's board over the pulled message: {}",
    pulled.said
  );
  let left = pulled
    .said
    .lines()
    .find(|l| l.starts_with("intent (post-merge): left:"))
    .unwrap_or_else(|| panic!("the hook named nothing it left: {}", pulled.said));
  assert!(
    left.contains("intent/whiteboard/bo/board.json") && left.contains("intent sync --to-store"),
    "the left line names the pulled board and the verb that carries it: {left}"
  );

  let pickup = team.intent(&bob, &["wb", "pickup", "--node", "bo"]);
  assert_ne!(
    pickup.code, 0,
    "a board write rendered over the pulled board: {}",
    pickup.said
  );
  assert!(
    pickup.said.contains("intent sync --to-store"),
    "{}",
    pickup.said
  );
  let doctor = team.intent(&bob, &["doctor"]);
  assert!(
    !doctor.said.contains("--to-disk` regenerates it"),
    "doctor names the discarding verb for the pulled board: {}",
    doctor.said
  );

  team.intent_ok(&bob, &["sync", "--to-store"]);
  let shown = team.intent(&bob, &["wb", "show", "bo"]);
  assert!(
    shown.said.contains("The onboarding guide is yours"),
    "the named verb took the pulled message onto the board: {}",
    shown.said
  );
  team.intent_ok(&bob, &["wb", "pickup", "--node", "bo"]);
}

/// The control for the arm above (vc, 2026-09-24): **A BOARD THIS CLONE WROTE
/// ITSELF IS NEVER KEPT OR NAMED BY A PULL.** Its file is where the store left
/// it, so a pull that brings nothing for it leaves the store's board as the
/// newer one, and carrying the file would roll it back.
#[test]
fn a_board_this_clone_wrote_is_not_kept_by_a_pull() {
  let team = Team::new();
  let bob = team.bob();
  team.intent_ok(
    &bob,
    &[
      "wb",
      "register",
      "bo",
      "--name",
      "Bob's agent",
      "--role",
      "worker",
    ],
  );
  team.intent_ok(
    &bob,
    &[
      "wb",
      "add",
      "todo",
      "Read the onboarding guide",
      "--node",
      "bo",
    ],
  );

  team.alice_pushes("Release checklist");
  let pulled = team.git(&bob, &["pull", "-q"]);
  assert!(
    !pulled.said.contains("left:") && !pulled.said.contains("whiteboard"),
    "a pull named a board this clone wrote: {}",
    pulled.said
  );
  team.intent_ok(&bob, &["wb", "pickup", "--node", "bo"]);
}

/// The hooks' path in issue 0559's family: **A PULL CARRIES A HAND EDIT TO A
/// COVER INTO THE STORE, AND NEVER DISCARDS IT.** The hook's ingest read the
/// edited Objective back and then kept the store's copy of the thread, because
/// a cover edit moves no canon file, so the projection rewrote the cover from
/// the store and the edit was gone with no line naming it. Now the pull takes
/// it, as it takes a pulled thread.
#[test]
fn a_pull_carries_a_hand_edited_cover_into_the_store() {
  let team = Team::new();
  let (alice, bob) = (team.alice(), team.bob());
  team.intent_ok(&alice, &["st", "start", "ST0001"]);
  team.intent_ok(&alice, &["organize", "--apply"]);
  team.commit(&alice, "ST0001 started");
  team.git(&alice, &["push", "-q"]);
  team.git(&bob, &["pull", "-q"]);

  let cover = bob.join("intent/st/ST0001/info.md");
  let text = std::fs::read_to_string(&cover).expect("the pulled cover is on disk");
  let from = text
    .find("## Objective\n\n")
    .expect("the cover has an Objective")
    + "## Objective\n\n".len();
  let to = text
    .find("## Context\n\n")
    .expect("the cover has a Context");
  std::fs::write(
    &cover,
    format!(
      "{}Typed by hand before the pull.\n\n{}",
      &text[..from],
      &text[to..]
    ),
  )
  .expect("the hand edit");

  team.alice_pushes("Release checklist");
  let pulled = team.git(&bob, &["pull", "-q"]);
  assert!(
    std::fs::read_to_string(&cover)
      .expect("the cover is still on disk")
      .contains("Typed by hand before the pull."),
    "the pull's views step discarded the hand edit: {}",
    pulled.said
  );
  assert!(
    pulled.said.contains(
      "intent (post-merge): took 2 change(s) from the files into the store: ST0001, ST0002"
    ),
    "the pull names the cover's thread among what it took: {}",
    pulled.said
  );
  let shown = team.intent(&bob, &["st", "show", "ST0001"]);
  assert!(
    shown.said.contains("Typed by hand before the pull."),
    "the pull carried the edit into the store: {}",
    shown.said
  );
}
