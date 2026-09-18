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
    let bin = Path::new(env!("CARGO_BIN_EXE_intent"))
      .parent()
      .expect("the binary's directory");
    format!("{}:/usr/bin:/bin", bin.display())
  }

  fn command(&self, program: &str, cwd: &Path, path: &str) -> Command {
    let mut command = Command::new(program);
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
    let mut command = self.command(env!("CARGO_BIN_EXE_intent"), cwd, &self.path());
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
    vec!["intent (post-merge): took 1 change(s) from the files into the store: ST0002"],
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

  let took = intentsvcs::sync::ingested(&["ST0002".to_string()]);
  assert!(
    took.starts_with("took ") && template.contains("\"ok: took \"*)"),
    "the template prints on the word the verb begins with when it took something: {took}"
  );
  assert!(
    template.contains("\"left: \"*)") && template.contains("^doctor: "),
    "and on the line naming what was left, and tells doctor's verdict from a refusal"
  );
  assert!(
    !intentsvcs::sync::ingested(&[]).starts_with("took"),
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
