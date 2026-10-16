//! ST0083 WP-06, AC-06.1: `intent deinit` lists what Intent wrote and writes
//! nothing; `--apply` removes and edits exactly that and keeps what a person
//! wrote; `--all` also takes `intent/` and the registry row, and refuses while
//! a daemon watches the tree.
//!
//! **THE FIXTURE IS THE SEQUENCE A REAL PROJECT RUNS**: `git init`, `init`,
//! `bootstrap`, `claude upgrade --apply`, then the owner's own settings key, so
//! every file `deinit` meets is one Intent's own verbs wrote.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::common::{ATTEMPTS, PAUSE, RealDaemon, short_dir};

fn run(args: &[&str], cwd: &Path, home: &Path) -> (String, String, i32) {
  let out = crate::common::intent()
    .args(args)
    .current_dir(cwd)
    .env("HOME", home)
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run the v3 binary");
  (
    String::from_utf8_lossy(&out.stdout).into_owned(),
    String::from_utf8_lossy(&out.stderr).into_owned(),
    out.status.code().unwrap_or(-1),
  )
}

fn ok(args: &[&str], cwd: &Path, home: &Path) -> String {
  let (out, err, code) = run(args, cwd, home);
  assert_eq!(code, 0, "`intent {}` failed:\n{out}{err}", args.join(" "));
  out
}

/// A canonical project at `root` under `home`, the owner's `model` key added to
/// its settings.
fn canonical(root: &Path, home: &Path) {
  std::fs::create_dir_all(root).expect("mkdir");
  let git = Command::new("git")
    .args(["init", "-q"])
    .current_dir(root)
    .status()
    .expect("git init");
  assert!(git.success());
  ok(&["init", "Book"], root, home);
  ok(&["bootstrap"], root, home);
  ok(&["claude", "upgrade", "--apply"], root, home);
  let path = root.join(".claude/settings.json");
  let mut settings: serde_json::Value =
    serde_json::from_str(&std::fs::read_to_string(&path).expect("settings")).expect("parse");
  settings["model"] = serde_json::json!("opus");
  std::fs::write(
    &path,
    serde_json::to_string_pretty(&settings).expect("render"),
  )
  .expect("write");
}

/// Every file under `dir` with its bytes.
fn tree(dir: &Path) -> Vec<(PathBuf, Vec<u8>)> {
  let mut out = Vec::new();
  let mut stack = vec![dir.to_path_buf()];
  while let Some(d) = stack.pop() {
    for entry in std::fs::read_dir(&d).expect("read_dir") {
      let path = entry.expect("entry").path();
      if path.is_dir() {
        stack.push(path);
      } else {
        out.push((path.clone(), std::fs::read(&path).expect("read")));
      }
    }
  }
  out.sort();
  out
}

#[test]
fn the_dry_run_lists_every_kind_of_act_and_writes_nothing() {
  let tmp = tempfile::tempdir().expect("tempdir");
  let (root, home) = (tmp.path().join("book"), tmp.path().join("home"));
  canonical(&root, &home);
  let before = tree(&root);
  let out = ok(&["deinit"], &root, &home);
  assert_eq!(tree(&root), before, "the dry run wrote:\n{out}");
  for line in [
    "would remove: AGENTS.md",
    "would remove: CLAUDE.md",
    "would edit: .claude/settings.json",
    "would remove: .mcp.json",
    "would remove: .git/hooks/pre-commit.intent",
    "done: dry run",
  ] {
    assert!(
      out.contains(line),
      "the dry run did not say `{line}`:\n{out}"
    );
  }
}

#[test]
fn apply_strips_what_intent_wrote_and_keeps_the_owners_settings_and_the_store() {
  let tmp = tempfile::tempdir().expect("tempdir");
  let (root, home) = (tmp.path().join("book"), tmp.path().join("home"));
  canonical(&root, &home);
  let out = ok(&["deinit", "--apply"], &root, &home);
  for gone in [
    "AGENTS.md",
    "CLAUDE.md",
    ".mcp.json",
    ".git/hooks/pre-commit.intent",
  ] {
    assert!(!root.join(gone).exists(), "{gone} survived --apply:\n{out}");
  }
  let settings = std::fs::read_to_string(root.join(".claude/settings.json")).expect("kept");
  assert_eq!(
    serde_json::from_str::<serde_json::Value>(&settings).expect("JSON"),
    serde_json::json!({ "model": "opus" }),
    "the owner's settings were not kept alone:\n{settings}"
  );
  let hook = root.join(".git/hooks/pre-commit");
  assert!(
    !std::fs::read_to_string(&hook)
      .unwrap_or_default()
      .contains("intent-chain-block"),
    "the chain block survived:\n{out}"
  );
  assert!(root.join("intent").is_dir(), "intent/ went without --all");
  assert!(
    out.contains("edited: .claude/settings.json") && out.contains("done:"),
    "{out}"
  );
}

#[test]
fn all_takes_intent_and_the_registry_row_too() {
  let tmp = tempfile::tempdir().expect("tempdir");
  let (root, home) = (tmp.path().join("book"), tmp.path().join("home"));
  canonical(&root, &home);
  ok(
    &["discover", tmp.path().to_str().expect("utf-8")],
    &root,
    &home,
  );
  let registry = home.join(".config/intent/projects.json");
  let canonical_root = root
    .canonicalize()
    .expect("canonical")
    .display()
    .to_string();
  assert!(
    std::fs::read_to_string(&registry)
      .expect("registry")
      .contains(&canonical_root),
    "the fixture is not registered"
  );
  let out = ok(&["deinit", "--apply", "--all"], &root, &home);
  assert!(
    !root.join("intent").exists(),
    "intent/ survived --all:\n{out}"
  );
  assert!(
    !std::fs::read_to_string(&registry)
      .expect("registry")
      .contains(&canonical_root),
    "the registry row survived --all:\n{out}"
  );
}

#[test]
fn all_refuses_while_a_daemon_watches_the_tree() {
  let daemon = RealDaemon::start();
  let root = short_dir("deinit-all");
  intentsvcs::init::init(&root, "Deinit", "test", env!("CARGO_PKG_VERSION")).expect("init");
  let (_, err, code) = run(&["--daemon", "st", "list"], &root, daemon.home());
  assert_eq!(
    code, 0,
    "the daemon could not answer for the project: {err}"
  );
  let mut watched = false;
  for _ in 0..ATTEMPTS * 12 {
    if daemon.watching(&root) {
      watched = true;
      break;
    }
    std::thread::sleep(PAUSE);
  }
  assert!(
    watched,
    "the daemon never watched the fixture, so the refusal is untested"
  );
  let (out, err, code) = run(&["deinit", "--apply", "--all"], &root, daemon.home());
  assert_eq!(
    code, 1,
    "deinit --all ran beside a watching daemon:\n{out}{err}"
  );
  assert!(err.contains("intentd is watching"), "{err}");
  assert!(
    root.join("intent").is_dir(),
    "intent/ went beside a watching daemon"
  );
}
