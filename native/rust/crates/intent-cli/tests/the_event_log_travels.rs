//! ST0078 P1 -- **the event log travels** (reverses D53).
//!
//! Every event is its own committed file at
//! `intent/.canon/events/<YYYY>/<MM>/<DD>/<ulid>.json`, written in the act's own
//! write set and naming its author. Ingest of those files is additive, doctor
//! checks them, and `upgrade` retires the old `intent/events.jsonl` ignore rule.
//!
//! Driven through the real binary on two git clones of one project, because
//! the property is about what crosses a clone: a facade arm on one tree could
//! pass while nothing reached the second.
//!
//! The acceptance tests this file carries, and their arms:
//!
//! - ST0078 AT-01.1 (AC-01.1): `an_act_on_one_clone_is_read_on_another_with_its_author`,
//!   `the_author_is_git_then_the_config_then_local` and
//!   `a_heartbeat_stays_on_the_machine_that_beat`.
//! - ST0078 AT-01.2 (AC-01.2): `ingest_takes_what_is_missing_and_changes_nothing_else`
//!   and `the_sync_plan_names_waiting_event_files_and_apply_takes_them`.
//! - ST0078 AT-01.3 (AC-01.3): `doctor_reports_bad_event_files_and_events_the_store_lacks`.
//! - ST0078 AT-01.4 (AC-01.4): `upgrade_retires_the_ignore_rule_and_export_keeps_one_file`,
//!   `upgrade_backfills_the_history_the_store_held_once` and
//!   `upgrade_on_a_migrated_estate_writes_no_event_log_file`.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// One person's clone, with its own git identity and its own isolated HOME, so
/// no bootstrap on the machine running the suite names the author.
struct Clone {
  root: PathBuf,
  home: PathBuf,
}

impl Clone {
  fn run(&self, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_intent"))
      .args(args)
      .current_dir(&self.root)
      .env("HOME", &self.home)
      .env("XDG_CONFIG_HOME", self.home.join(".config"))
      .env("XDG_DATA_HOME", self.home.join(".local/share"))
      .env("XDG_STATE_HOME", self.home.join(".local/state"))
      .stdin(testkit::lifeline_for(args))
      .output()
      .expect("run the v3 binary")
  }

  fn intent(&self, args: &[&str]) -> String {
    let out = self.run(args);
    assert_eq!(
      out.status.code(),
      Some(0),
      "`intent {}` failed\nstdout: {}\nstderr: {}",
      args.join(" "),
      String::from_utf8_lossy(&out.stdout),
      String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).to_string()
  }

  /// Doctor's whole output, whatever its verdict: a finding exits non-zero.
  fn doctor(&self, args: &[&str]) -> String {
    let mut argv = vec!["doctor"];
    argv.extend_from_slice(args);
    let out = self.run(&argv);
    format!(
      "{}{}",
      String::from_utf8_lossy(&out.stdout),
      String::from_utf8_lossy(&out.stderr)
    )
  }

  fn git(&self, args: &[&str]) -> String {
    git(&self.root, args)
  }

  fn commit(&self, message: &str) {
    self.git(&["add", "-A"]);
    self.git(&["commit", "-q", "-m", message]);
  }

  fn event_files(&self) -> Vec<PathBuf> {
    let mut out = Vec::new();
    walk(&self.root.join("intent/.canon/events"), &mut out);
    out.sort();
    out
  }

  fn events_json(&self) -> serde_json::Value {
    serde_json::from_str(&self.intent(&["events", "--format", "json"])).expect("events json")
  }
}

fn git(root: &Path, args: &[&str]) -> String {
  let out = Command::new("git")
    .args(args)
    .current_dir(root)
    .output()
    .expect("run git");
  assert!(
    out.status.success(),
    "git {} failed: {}",
    args.join(" "),
    String::from_utf8_lossy(&out.stderr)
  );
  String::from_utf8_lossy(&out.stdout).to_string()
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
  let Ok(entries) = std::fs::read_dir(dir) else {
    return;
  };
  for entry in entries.flatten() {
    let path = entry.path();
    if path.is_dir() {
      walk(&path, out);
    } else if path.extension().is_some_and(|x| x == "json") {
      out.push(path);
    }
  }
}

fn identify(root: &Path, name: &str) {
  git(root, &["config", "user.name", name]);
  git(
    root,
    &[
      "config",
      "user.email",
      &format!("{}@example.com", name.to_lowercase()),
    ],
  );
}

/// Alice's project, initialised and committed, and Bob's clone of it. Neither
/// machine has bootstrapped, so the config's author is `init`'s placeholder and
/// each event names the git identity of the clone it was made on.
fn two_clones(dir: &Path) -> (Clone, Clone) {
  let alice = Clone {
    root: dir.join("alice"),
    home: dir.join("home-alice"),
  };
  std::fs::create_dir_all(&alice.root).expect("mkdir alice");
  std::fs::create_dir_all(&alice.home).expect("mkdir alice home");
  alice.git(&["init", "-q"]);
  identify(&alice.root, "Alice");
  alice.intent(&["init", "Team"]);
  alice.intent(&["st", "new", "Onboarding guide"]);
  alice.commit("alice: init and ST0001");

  let bob = Clone {
    root: dir.join("bob"),
    home: dir.join("home-bob"),
  };
  std::fs::create_dir_all(&bob.home).expect("mkdir bob home");
  git(
    dir,
    &[
      "clone",
      "-q",
      alice.root.to_str().expect("utf8"),
      bob.root.to_str().expect("utf8"),
    ],
  );
  identify(&bob.root, "Bob");
  (alice, bob)
}

/// **AC-01.1: an act on one clone is readable on another after a pull, with
/// its author**, and its record is a committed file named by its id in its
/// day's directory, landing in the act's own write set.
#[test]
fn an_act_on_one_clone_is_read_on_another_with_its_author() {
  let dir = tempfile::tempdir().expect("tempdir");
  let (alice, bob) = two_clones(dir.path());

  // Each of Alice's events is one file under its day, named by its id.
  let files = alice.event_files();
  assert_eq!(files.len(), 2, "init and st.new, one file each: {files:?}");
  for path in &files {
    let e: serde_json::Value =
      serde_json::from_str(&std::fs::read_to_string(path).expect("read")).expect("json");
    let id = e["id"].as_str().expect("id");
    let ts = e["ts"].as_str().expect("ts");
    let want = alice
      .root
      .join("intent/.canon/events")
      .join(&ts[..4])
      .join(&ts[5..7])
      .join(&ts[8..10])
      .join(format!("{id}.json"));
    assert_eq!(
      path, &want,
      "an event lives under its own day, named by its id"
    );
    assert_eq!(e["principal"], "Alice <alice@example.com>", "{e}");
  }

  // Bob's cold clone reads Alice's acts, with her name on them.
  let read = bob.intent(&["events", "--subject", "ST0001"]);
  assert!(
    read.contains("st.new  ST0001  by Alice <alice@example.com>"),
    "Alice's act is read on Bob's clone with its author: {read}"
  );

  // Bob acts; the event file is in the same working-tree change as the canon
  // it records, so one commit carries both.
  bob.intent(&["st", "start", "ST0001"]);
  let status = bob.git(&["status", "--porcelain", "--untracked-files=all"]);
  assert!(
    status.contains("intent/.canon/st/ST0001.json"),
    "the act's canon changed: {status}"
  );
  let new_event = status
    .lines()
    .find(|l| l.starts_with("??") && l.contains("intent/.canon/events/"))
    .unwrap_or_else(|| panic!("the act's event file lands beside its canon: {status}"));
  bob.commit("bob: start ST0001");

  // Alice pulls; after the store takes the tree, Bob's act is read with Bob's name.
  alice.git(&[
    "pull",
    "-q",
    "--no-rebase",
    bob.root.to_str().expect("utf8"),
    "HEAD",
  ]);
  alice.intent(&["sync", "--apply"]);
  let read = alice.intent(&["events", "--subject", "ST0001"]);
  assert!(
    read.contains("st.start  ST0001  by Bob <bob@example.com>"),
    "Bob's act ({new_event}) is read on Alice's clone with its author: {read}"
  );
  assert!(
    read.contains("st.new  ST0001  by Alice <alice@example.com>"),
    "and her own act is still there: {read}"
  );
}

/// **The author rule's order**: git's name and email, else the configured
/// author, else `local`. `init`'s placeholder is not a name.
#[test]
fn the_author_is_git_then_the_config_then_local() {
  use intentsvcs::event::{UNKNOWN_AUTHOR, author};
  assert_eq!(author(Some("Alice"), Some("a@x"), "matts"), "Alice <a@x>");
  assert_eq!(author(Some("Alice"), None, "matts"), "Alice");
  assert_eq!(author(None, Some(" a@x "), ""), "<a@x>");
  assert_eq!(author(None, None, "matts"), "matts");
  assert_eq!(author(Some(" "), None, UNKNOWN_AUTHOR), "local");
  assert_eq!(author(None, None, "  "), "local");
}

/// **AC-01.1's other half: a machine-scoped act stays in the store.** Registering
/// a node is a project act and writes its file; a heartbeat describes this
/// machine alone, so its row is written and no file is.
#[test]
fn a_heartbeat_stays_on_the_machine_that_beat() {
  let dir = tempfile::tempdir().expect("tempdir");
  let (alice, _bob) = two_clones(dir.path());
  let before = alice.event_files().len();

  alice.intent(&[
    "wb", "register", "al", "--name", "Alice", "--role", "worker",
  ]);
  let registered = alice.event_files().len();
  assert_eq!(registered, before + 1, "wb.register travels");

  alice.intent(&["wb", "touch", "--node", "al"]);
  assert_eq!(
    alice.event_files().len(),
    registered,
    "wb.touch writes no event file"
  );
  let events = alice.events_json();
  assert!(
    events["events"]
      .as_array()
      .expect("rows")
      .iter()
      .any(|e| e["op"] == "wb.touch"),
    "the heartbeat is still recorded in the store: {events}"
  );
}

/// **AC-01.2: ingest is additive.** A file whose id the store holds is skipped
/// even when its bytes differ, one it lacks is inserted, no file is rewritten,
/// and a row stays when its file is absent.
#[test]
fn ingest_takes_what_is_missing_and_changes_nothing_else() {
  let dir = tempfile::tempdir().expect("tempdir");
  let (alice, bob) = two_clones(dir.path());
  bob.intent(&["wp", "new", "ST0001", "Write it"]);
  bob.commit("bob: WP-01");
  alice.git(&[
    "pull",
    "-q",
    "--no-rebase",
    bob.root.to_str().expect("utf8"),
    "HEAD",
  ]);

  // A file the store already holds, edited: its row must not follow it.
  let held = alice
    .event_files()
    .into_iter()
    .find(|p| {
      std::fs::read_to_string(p)
        .expect("read")
        .contains("\"st.new\"")
    })
    .expect("Alice's st.new file");
  let edited = std::fs::read_to_string(&held)
    .expect("read")
    .replace("Onboarding guide", "Rewritten after the fact");
  std::fs::write(&held, &edited).expect("edit");
  let before: Vec<(PathBuf, Vec<u8>)> = alice
    .event_files()
    .into_iter()
    .map(|p| {
      let bytes = std::fs::read(&p).expect("read");
      (p, bytes)
    })
    .collect();

  alice.intent(&["sync", "--apply"]);

  let events = alice.events_json();
  let rows = events["events"].as_array().expect("rows");
  assert!(
    rows
      .iter()
      .any(|e| e["op"] == "wp.new" && e["principal"] == "Bob <bob@example.com>"),
    "a committed event the store lacked is inserted: {events}"
  );
  let st_new = rows
    .iter()
    .find(|e| e["op"] == "st.new")
    .expect("st.new row");
  assert_eq!(
    st_new["payload"]["title"], "Onboarding guide",
    "a held id is skipped, so the row keeps what it recorded: {st_new}"
  );
  let after: Vec<(PathBuf, Vec<u8>)> = alice
    .event_files()
    .into_iter()
    .map(|p| {
      let bytes = std::fs::read(&p).expect("read");
      (p, bytes)
    })
    .collect();
  assert_eq!(before, after, "ingest writes no event file");

  // A row whose file is gone stays: a local act never committed is still a fact.
  let total = rows.len();
  std::fs::remove_file(&held).expect("remove");
  alice.intent(&["sync", "--apply"]);
  let events = alice.events_json();
  assert_eq!(
    events["events"].as_array().expect("rows").len(),
    total,
    "no row is deleted because its file is absent: {events}"
  );
}

/// **The sync plan names event files waiting to be taken, and `--apply` takes
/// them.** A pull that brings only event files changes no canon, so without a
/// step of its own the plan would say there is nothing to do while the apply
/// would act; and the apply's line begins `took`, which is the word the git
/// hooks print on.
#[test]
fn the_sync_plan_names_waiting_event_files_and_apply_takes_them() {
  let dir = tempfile::tempdir().expect("tempdir");
  let (alice, _bob) = two_clones(dir.path());
  let first = alice.event_files().remove(0);
  let mut foreign: serde_json::Value =
    serde_json::from_str(&std::fs::read_to_string(&first).expect("read")).expect("json");
  let id = "01M2SXFFFFFFFFFFFFFFFFFFFF";
  foreign["id"] = id.into();
  std::fs::write(
    first.parent().expect("day dir").join(format!("{id}.json")),
    serde_json::to_string_pretty(&foreign).expect("json"),
  )
  .expect("write");

  let plan = alice.intent(&["sync"]);
  assert!(
    plan.contains("1 event file(s) to take"),
    "the plan names the waiting event file: {plan}"
  );

  let applied = alice.intent(&["sync", "--apply"]);
  assert!(
    applied.contains("ok: took 1 event file(s) from the files into the store"),
    "an event-only take begins `took`: {applied}"
  );
  let read = alice.intent(&["events", "--format", "json"]);
  assert!(read.contains(id), "the event is in the store: {read}");

  let again = alice.intent(&["sync"]);
  assert!(
    !again.contains("event file(s) to take"),
    "once taken, nothing waits: {again}"
  );
}

/// **AC-01.4's backfill: `upgrade` writes, once, a file for every project event
/// the store holds and the tree lacks.** A project from before its events
/// travelled is a store full of history and no files, so the clone's files are
/// removed to make one. A machine-scoped event stays in the store, and a
/// second run writes nothing.
#[test]
fn upgrade_backfills_the_history_the_store_held_once() {
  let dir = tempfile::tempdir().expect("tempdir");
  let (alice, _bob) = two_clones(dir.path());
  alice.intent(&[
    "wb", "register", "al", "--name", "Alice", "--role", "worker",
  ]);
  alice.intent(&["wb", "touch", "--node", "al"]);
  std::fs::remove_dir_all(alice.root.join("intent/.canon/events")).expect("rm events");

  let events = alice.events_json();
  let rows = events["events"].as_array().expect("rows");
  let travelling: Vec<&str> = rows
    .iter()
    .filter(|e| intentsvcs::event::travels(e["op"].as_str().expect("op")))
    .map(|e| e["id"].as_str().expect("id"))
    .collect();
  assert!(
    rows.iter().any(|e| e["op"] == "wb.touch"),
    "the store holds a machine-scoped event, or this arm cannot see it skipped: {events}"
  );

  let out = alice.run(&["upgrade"]);
  let said = String::from_utf8_lossy(&out.stderr);
  assert_eq!(out.status.code(), Some(0), "upgrade: {said}");
  assert!(
    said.contains(&format!("backfilled: {} event file(s)", travelling.len())),
    "the backfill says how many it wrote: {said}"
  );
  let files = alice.event_files();
  let mut named: Vec<String> = files
    .iter()
    .map(|p| p.file_stem().expect("stem").to_string_lossy().into_owned())
    .collect();
  named.sort();
  // Read again: an act the upgrade itself made writes its own file.
  let events = alice.events_json();
  let mut want: Vec<String> = events["events"]
    .as_array()
    .expect("rows")
    .iter()
    .filter(|e| intentsvcs::event::travels(e["op"].as_str().expect("op")))
    .map(|e| e["id"].as_str().expect("id").to_string())
    .collect();
  want.sort();
  assert_eq!(
    named, want,
    "one file per project event, and none for a heartbeat"
  );

  let before: Vec<Vec<u8>> = files
    .iter()
    .map(|p| std::fs::read(p).expect("read"))
    .collect();
  let again = alice.run(&["upgrade"]);
  let said = String::from_utf8_lossy(&again.stderr);
  assert!(
    !said.contains("backfilled:"),
    "a second run writes nothing: {said}"
  );
  let after: Vec<Vec<u8>> = alice
    .event_files()
    .iter()
    .map(|p| std::fs::read(p).expect("read"))
    .collect();
  assert_eq!(before, after, "and rewrites nothing");
}

/// **Issue 0457: `upgrade` on a migrated estate leaves no `intent/events.jsonl`.**
/// Its re-emission wrote every canon part, and the last was an empty event log,
/// untracked once ST0078 retired the ignore rule. The status read is the whole
/// tree's: one scoped to `.canon/events/` was the read that could not see it.
#[test]
fn upgrade_on_a_migrated_estate_writes_no_event_log_file() {
  let dir = tempfile::tempdir().expect("tempdir");
  let (alice, _bob) = two_clones(dir.path());

  let out = alice.run(&["upgrade"]);
  assert_eq!(
    out.status.code(),
    Some(0),
    "upgrade: {}",
    String::from_utf8_lossy(&out.stderr)
  );
  let status = alice.git(&["status", "--short", "--untracked-files=all"]);
  assert!(
    !status.contains("events.jsonl"),
    "upgrade left the retired single-file log in the tree:\n{status}"
  );
  assert!(
    !alice.root.join("intent/events.jsonl").exists(),
    "and no ignore rule hides one"
  );
}

/// **AC-01.3: doctor checks the files and nothing else.** A file that is not an
/// envelope, and one whose name is not its id, are reported by path; a committed
/// event the store lacks is store-stale; and an event naming a thread canon
/// does not have builds no thread and draws no reconciliation finding.
#[test]
fn doctor_reports_bad_event_files_and_events_the_store_lacks() {
  let dir = tempfile::tempdir().expect("tempdir");
  let (alice, _bob) = two_clones(dir.path());
  let first = alice.event_files().remove(0);
  let day = first.parent().expect("day dir").to_path_buf();
  let good: serde_json::Value =
    serde_json::from_str(&std::fs::read_to_string(&first).expect("read")).expect("json");

  // Not an envelope.
  std::fs::write(
    day.join("01M2SXBBBBBBBBBBBBBBBBBBBB.json"),
    "{\"id\": \"x\"}\n",
  )
  .expect("write");
  // Named for one id, carrying another.
  let mut misnamed = good.clone();
  misnamed["id"] = "01M2SXCCCCCCCCCCCCCCCCCCCC".into();
  std::fs::write(
    day.join("01M2SXDDDDDDDDDDDDDDDDDDDD.json"),
    serde_json::to_string_pretty(&misnamed).expect("json"),
  )
  .expect("write");
  let out = alice.doctor(&[]);
  assert!(
    out.contains("schema-invalid") && out.contains("01M2SXBBBBBBBBBBBBBBBBBBBB.json"),
    "a file that is not an envelope is reported by path: {out}"
  );
  assert!(
    out.contains("unknown-file-shape")
      && out.contains("01M2SXDDDDDDDDDDDDDDDDDDDD.json")
      && out.contains("carries the id 01M2SXCCCCCCCCCCCCCCCCCCCC"),
    "a file whose name is not its id is reported by path: {out}"
  );
  std::fs::remove_file(day.join("01M2SXBBBBBBBBBBBBBBBBBBBB.json")).expect("rm");
  std::fs::remove_file(day.join("01M2SXDDDDDDDDDDDDDDDDDDDD.json")).expect("rm");

  // A well-formed committed event the store has never taken, which names a
  // thread canon does not have.
  let mut foreign = good;
  let id = "01M2SXEEEEEEEEEEEEEEEEEEEE";
  foreign["id"] = id.into();
  foreign["op"] = "st.new".into();
  foreign["subject"] = serde_json::json!({ "type": "thread", "id": "ST0099" });
  foreign["payload"] = serde_json::json!({ "title": "Only in the log" });
  std::fs::write(
    day.join(format!("{id}.json")),
    serde_json::to_string_pretty(&foreign).expect("json"),
  )
  .expect("write");
  let out = alice.doctor(&["--verbose"]);
  assert!(
    out.contains("store-stale") && out.contains("does not hold 1 committed event file(s)"),
    "a committed event the store lacks is store-stale: {out}"
  );

  alice.intent(&["sync", "--apply"]);
  let read = alice.intent(&["events", "--subject", "ST0099"]);
  assert!(read.contains(id), "the event is ingested: {read}");
  let show = alice.run(&["st", "show", "ST0099"]);
  assert_ne!(
    show.status.code(),
    Some(0),
    "nothing rebuilds state from events, so ST0099 exists only in the log"
  );
  let out = alice.doctor(&["--verbose"]);
  assert!(
    !out.contains("ST0099"),
    "doctor never reconciles canon against events: {out}"
  );
}

/// **AC-01.4: `upgrade` removes the retired ignore rule and adds nothing, and
/// `export` still produces the single-file form.**
#[test]
fn upgrade_retires_the_ignore_rule_and_export_keeps_one_file() {
  let dir = tempfile::tempdir().expect("tempdir");
  let (alice, _bob) = two_clones(dir.path());
  let gitignore = alice.root.join(".gitignore");
  let converged = std::fs::read_to_string(&gitignore).expect("read .gitignore");
  assert!(
    !converged.contains("events.jsonl"),
    "a project born now has no events.jsonl rule: {converged}"
  );
  // The block the converger wrote before ST0078.
  std::fs::write(
    &gitignore,
    format!(
      "{converged}\n# The event log lives in the store (D53); its file form is produced by `intent export`.\nintent/events.jsonl\n"
    ),
  )
  .expect("write");
  alice.commit("an estate from before ST0078");

  alice.intent(&["upgrade"]);
  let upgraded = std::fs::read_to_string(&gitignore).expect("read .gitignore");
  assert_eq!(
    upgraded, converged,
    "upgrade removes the rule and its comment and adds nothing"
  );

  let export: serde_json::Value =
    serde_json::from_str(&alice.intent(&["export"])).expect("export is one JSON document");
  let ops: Vec<&str> = export["events"]
    .as_array()
    .expect("the export carries the log")
    .iter()
    .filter_map(|e| e["op"].as_str())
    .collect();
  assert_eq!(ops, ["init", "st.new"], "every event, in the one document");
}
