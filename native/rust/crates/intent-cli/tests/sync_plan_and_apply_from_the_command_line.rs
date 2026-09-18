//! ST0078 AT-03.4 (AC-03.3): **bare `intent sync` prints this clone's plan and
//! writes nothing; `intent sync --apply` applies it, beside a running daemon
//! too; `--apply` with a direction is refused.**
//!
//! hv's ruling of 2026-09-18 is `intent sync [--apply] [--to-disk|--to-store]`.
//! The bare verb used to refuse, asking which direction was meant; it now
//! answers from the state instead. Under P3 the plan has one step, the
//! daemon's own ingest pass, and these arms drive the shipped binary through
//! each door.
//!
//! **THE DAEMON ARM HAS BOTH HALVES, BECAUSE THE CLAIM IS THAT THE FLAG
//! DISCRIMINATES.** The two directions refuse where a daemon watches, since a
//! second sync engine would watch and ingest beside it. `--apply` watches
//! nothing and lands under the same hold-unless-moved lock, so it runs. The
//! `--to-disk` refusal beside it is what proves the daemon was watching.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use crate::common::{RealDaemon, short_dir};

fn run(home: &Path, root: &Path, argv: &[&str]) -> Output {
  Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(argv)
    .current_dir(root)
    .env("HOME", home)
    .stdin(testkit::lifeline_for(argv))
    .output()
    .expect("the intent binary runs")
}

fn text(out: &Output) -> String {
  format!(
    "{}{}",
    String::from_utf8_lossy(&out.stdout),
    String::from_utf8_lossy(&out.stderr)
  )
}

/// A project with ST0001 in a warm store, and a HOME of its own.
fn project(tag: &str) -> (PathBuf, PathBuf) {
  let dir = short_dir(tag);
  let root = dir.join("p");
  let home = dir.join("home");
  std::fs::create_dir_all(&home).expect("the isolated HOME");
  intentsvcs::init::init(&root, "Plan", "test", env!("CARGO_PKG_VERSION"))
    .expect("the shipped initialiser creates a project");
  let out = run(&home, &root, &["st", "new", "a thread in the store"]);
  assert_eq!(out.status.code(), Some(0), "{}", text(&out));
  (root, home)
}

/// What a pull does: a thread the store never wrote arrives on disk, canon and
/// views together, copied from a second project that minted it.
fn a_pull_brings_st0002(root: &Path, home: &Path) {
  let (other, _) = project("plan-origin");
  let out = run(home, &other, &["st", "new", "the thread a pull brought"]);
  assert_eq!(out.status.code(), Some(0), "{}", text(&out));
  let canon = "intent/.canon/st/ST0002.json";
  std::fs::copy(other.join(canon), root.join(canon)).expect("the pull brings the canon file");
}

/// Every file under the project, with its bytes, `.cache/` excluded.
fn tree(root: &Path) -> Vec<(PathBuf, Vec<u8>)> {
  fn walk(dir: &Path, out: &mut Vec<(PathBuf, Vec<u8>)>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
      return;
    };
    for entry in entries.flatten() {
      let path = entry.path();
      if path.ends_with(".cache") {
        continue;
      }
      if path.is_dir() {
        walk(&path, out);
      } else {
        out.push((path.clone(), std::fs::read(&path).unwrap_or_default()));
      }
    }
  }
  let mut out = Vec::new();
  walk(root, &mut out);
  out.sort();
  out
}

#[test]
fn the_bare_verb_prints_the_plan_and_writes_nothing() {
  let (root, home) = project("plan");
  a_pull_brings_st0002(&root, &home);
  let before = tree(&root);

  let plan = run(&home, &root, &["sync"]);
  let said = text(&plan);
  assert_eq!(plan.status.code(), Some(0), "a plan is an answer: {said}");
  assert!(
    said.starts_with("plan: ") && said.contains("and nothing has been written"),
    "{said}"
  );
  assert!(
    said.contains(". ingest (quiet): take 1 change(s) from the files into the store: ST0002"),
    "the step names what it would take and says it is quiet: {said}"
  );
  assert!(
    said
      .lines()
      .last()
      .is_some_and(|l| l.contains("  then: doctor (quiet): ")),
    "doctor is the last step: {said}"
  );
  assert_eq!(tree(&root), before, "the bare verb wrote nothing on disk");
  let shown = run(&home, &root, &["st", "show", "ST0002"]);
  assert_ne!(
    shown.status.code(),
    Some(0),
    "and nothing in the store: the plan is not the apply"
  );

  let applied = run(&home, &root, &["sync", "--apply"]);
  assert!(
    text(&applied)
      .lines()
      .any(|l| l == "ok: took 1 change(s) from the files into the store: ST0002"),
    "the apply takes what the plan named: {}",
    text(&applied)
  );
  assert!(
    text(&applied)
      .lines()
      .last()
      .is_some_and(|l| l.starts_with("doctor: ")),
    "and doctor's verdict is the last word: {}",
    text(&applied)
  );
  let shown = run(&home, &root, &["st", "show", "ST0002"]);
  assert_eq!(shown.status.code(), Some(0), "{}", text(&shown));

  let again = run(&home, &root, &["sync"]);
  assert!(
    text(&again).contains("plan: nothing to do"),
    "once applied, the plan is empty: {}",
    text(&again)
  );
}

#[test]
fn apply_with_a_direction_is_refused() {
  let (root, home) = project("plan-refuse");
  for direction in ["--to-disk", "--to-store"] {
    let out = run(&home, &root, &["sync", "--apply", direction]);
    assert_eq!(
      out.status.code(),
      Some(1),
      "`--apply {direction}` ran: {}",
      text(&out)
    );
    assert!(
      text(&out).contains("chooses two things at once"),
      "{}",
      text(&out)
    );
  }
}

#[test]
fn apply_runs_where_a_daemon_watches_and_the_directions_refuse() {
  let daemon = RealDaemon::start();
  let root = short_dir("apply-beside");
  intentsvcs::init::init(&root, "Beside", "test", env!("CARGO_PKG_VERSION"))
    .expect("the shipped initialiser creates a project");

  let contacted = run(daemon.home(), &root, &["--daemon", "st", "list"]);
  assert_eq!(
    contacted.status.code(),
    Some(0),
    "the daemon could not answer for the project: {}",
    text(&contacted)
  );
  assert!(
    daemon.watching(&root),
    "the daemon registered the project without watching it, so nothing below is beside a watcher"
  );

  let apply = run(daemon.home(), &root, &["sync", "--apply"]);
  assert_eq!(
    apply.status.code(),
    Some(0),
    "`sync --apply` refused beside the daemon that runs the same pass: {}",
    text(&apply)
  );
  assert!(
    text(&apply).lines().any(|l| l.starts_with("doctor: ")),
    "{}",
    text(&apply)
  );
  let plan = run(daemon.home(), &root, &["sync"]);
  assert_eq!(plan.status.code(), Some(0), "{}", text(&plan));

  let direction = run(daemon.home(), &root, &["sync", "--to-disk"]);
  assert_eq!(
    direction.status.code(),
    Some(2),
    "the control: a sync DIRECTION still refuses here, so the daemon was watching: {}",
    text(&direction)
  );

  let _ = std::fs::remove_dir_all(&root);
}
