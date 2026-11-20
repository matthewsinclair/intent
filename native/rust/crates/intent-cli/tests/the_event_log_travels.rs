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
//!   `upgrade_backfills_the_history_the_store_held_once`,
//!   `upgrade_on_a_migrated_estate_writes_no_event_log_file`,
//!   `upgrade_removes_the_empty_event_log_an_earlier_upgrade_left` and
//!   `upgrade_names_an_event_log_with_content_and_leaves_it`.

use std::path::{Path, PathBuf};

use crate::common::Forge;

/// One person's clone of the forge, with its own git identity, under the
/// forge's isolated HOME, so no bootstrap on the machine running the suite
/// names the author.
struct Clone<'f> {
  forge: &'f Forge,
  root: PathBuf,
}

impl Clone<'_> {
  /// The binary in this clone: stdout, stderr and the exit code.
  fn run(&self, args: &[&str]) -> (String, String, i32) {
    self.forge.intent(&self.root, args)
  }

  fn intent(&self, args: &[&str]) -> String {
    self.forge.intent_ok(&self.root, args)
  }

  /// Doctor's whole output, whatever its verdict: a finding exits non-zero.
  fn doctor(&self, args: &[&str]) -> String {
    let mut argv = vec!["doctor"];
    argv.extend_from_slice(args);
    self.forge.intent_said(&self.root, &argv).text
  }

  fn git(&self, args: &[&str]) -> String {
    self.forge.git(&self.root, args)
  }

  fn commit(&self, message: &str) {
    self.forge.commit_all(&self.root, message);
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

/// `clone`'s own git identity, `name`, which its repository's config carries
/// over the forge's HOME's.
fn identify(clone: &Clone, name: &str) {
  clone.git(&["config", "user.name", name]);
  clone.git(&[
    "config",
    "user.email",
    &format!("{}@example.com", name.to_lowercase()),
  ]);
}

/// Alice's project, initialised with one thread, committed and pushed to the
/// forge, Bob's clone of it, and that thread's id. Neither machine has
/// bootstrapped, so the config's author is `init`'s placeholder and each event
/// names the git identity of the clone it was made on.
fn two_clones(forge: &Forge) -> (Clone<'_>, Clone<'_>, String) {
  let alice = Clone {
    forge,
    root: forge.clone_as("alice"),
  };
  identify(&alice, "Alice");
  alice.intent(&["init", "Team"]);
  let st = crate::common::created(&alice.intent(&["st", "new", "Onboarding guide"]));
  alice.commit(&format!("alice: init and {st}"));
  alice.git(&["push", "-q", "origin", "HEAD:main"]);

  let bob = Clone {
    forge,
    root: forge.clone_as("bob"),
  };
  identify(&bob, "Bob");
  (alice, bob, st)
}

/// **AC-01.1: an act on one clone is readable on another after a pull, with
/// its author**, and its record is a committed file named by its id in its
/// day's directory, landing in the act's own write set.
#[test]
fn an_act_on_one_clone_is_read_on_another_with_its_author() {
  let forge = Forge::new();
  let (alice, bob, st) = two_clones(&forge);

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
  let read = bob.intent(&["events", "--subject", st.as_str()]);
  assert!(
    read.contains(&format!("st.new  {st}  by Alice <alice@example.com>")),
    "Alice's act is read on Bob's clone with its author: {read}"
  );

  // Bob acts; the event file is in the same working-tree change as the canon
  // it records, so one commit carries both.
  bob.intent(&["st", "start", st.as_str()]);
  let status = bob.git(&["status", "--porcelain", "--untracked-files=all"]);
  assert!(
    status.contains(&format!("intent/.canon/st/{st}.json")),
    "the act's canon changed: {status}"
  );
  let new_event = status
    .lines()
    .find(|l| l.starts_with("??") && l.contains("intent/.canon/events/"))
    .unwrap_or_else(|| panic!("the act's event file lands beside its canon: {status}"));
  bob.commit(&format!("bob: start {st}"));

  // Alice pulls; after the store takes the tree, Bob's act is read with Bob's name.
  alice.git(&[
    "pull",
    "-q",
    "--no-rebase",
    bob.root.to_str().expect("utf8"),
    "HEAD",
  ]);
  alice.intent(&["sync", "--apply"]);
  let read = alice.intent(&["events", "--subject", st.as_str()]);
  assert!(
    read.contains(&format!("st.start  {st}  by Bob <bob@example.com>")),
    "Bob's act ({new_event}) is read on Alice's clone with its author: {read}"
  );
  assert!(
    read.contains(&format!("st.new  {st}  by Alice <alice@example.com>")),
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
  let forge = Forge::new();
  let (alice, _bob, _) = two_clones(&forge);
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
  let forge = Forge::new();
  let (alice, bob, st) = two_clones(&forge);
  let wp = crate::common::created(&bob.intent(&["wp", "new", st.as_str(), "Write it"]));
  bob.commit(&format!(
    "bob: {}",
    crate::common::package_of(&wp).heading()
  ));
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
  let forge = Forge::new();
  let (alice, _bob, _) = two_clones(&forge);
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
  let forge = Forge::new();
  let (alice, _bob, _) = two_clones(&forge);
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

  let (_, said, code) = alice.run(&["upgrade"]);
  assert_eq!(code, 0, "upgrade: {said}");
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
  let (_, said, _) = alice.run(&["upgrade"]);
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
  let forge = Forge::new();
  let (alice, _bob, _) = two_clones(&forge);

  let (_, said, code) = alice.run(&["upgrade"]);
  assert_eq!(code, 0, "upgrade: {said}");
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

/// An estate as a 3.0.x upgrade left it: the old ignore rule in `.gitignore`,
/// committed, and `intent/events.jsonl` holding `body` beside it, ignored.
fn an_estate_from_before_st0078(alice: &Clone, body: &str) -> String {
  let gitignore = alice.root.join(".gitignore");
  let converged = std::fs::read_to_string(&gitignore).expect("read .gitignore");
  std::fs::write(
    &gitignore,
    format!(
      "{converged}\n# The event log lives in the store (D53); its file form is produced by `intent export`.\nintent/events.jsonl\n"
    ),
  )
  .expect("write .gitignore");
  std::fs::write(alice.root.join("intent/events.jsonl"), body).expect("plant the log");
  alice.commit("an estate from before ST0078");
  assert!(
    !alice
      .git(&["ls-files", "--", "intent/events.jsonl"])
      .contains("events.jsonl"),
    "the planted log is ignored, as a 3.0.x upgrade left it"
  );
  converged
}

/// **Issue 0459: `upgrade` removes the empty `intent/events.jsonl` a 3.0.x
/// upgrade left**, once it has retired the rule that hid it. Otherwise every
/// existing project meets the file as untracked beside what the upgrade asks
/// its operator to commit. The status read is the whole tree's.
#[test]
fn upgrade_removes_the_empty_event_log_an_earlier_upgrade_left() {
  let forge = Forge::new();
  let (alice, _bob, _) = two_clones(&forge);
  let converged = an_estate_from_before_st0078(&alice, "");

  let (_, said, code) = alice.run(&["upgrade"]);
  assert_eq!(code, 0, "upgrade: {said}");
  assert!(
    said.contains("removed: ")
      && said.contains("intent/events.jsonl -- the empty single-file event log"),
    "the removal is said: {said}"
  );
  assert!(
    !alice.root.join("intent/events.jsonl").exists(),
    "the empty log is gone"
  );
  assert_eq!(
    std::fs::read_to_string(alice.root.join(".gitignore")).expect("read .gitignore"),
    converged,
    "and so is the rule that hid it"
  );
  let status = alice.git(&["status", "--short", "--untracked-files=all"]);
  assert!(
    !status.contains("events.jsonl"),
    "an unscoped status shows no event log:\n{status}"
  );
}

/// **Issue 0459's other half: a log with content is not an upgrade's to
/// remove.** It is left where it is and named with why.
#[test]
fn upgrade_names_an_event_log_with_content_and_leaves_it() {
  let forge = Forge::new();
  let (alice, _bob, _) = two_clones(&forge);
  let body = "{\"op\":\"st.new\"}\n";
  an_estate_from_before_st0078(&alice, body);

  let (_, said, code) = alice.run(&["upgrade"]);
  assert_eq!(code, 0, "upgrade: {said}");
  assert!(
    said.contains("kept: ") && said.contains(&format!("it holds {} byte(s)", body.len())),
    "the log is named with why it stays: {said}"
  );
  assert_eq!(
    std::fs::read_to_string(alice.root.join("intent/events.jsonl")).expect("the log stays"),
    body,
    "byte for byte"
  );
}

/// **AC-01.3: doctor checks the files and nothing else.** A file that is not an
/// envelope, and one whose name is not its id, are reported by path; a committed
/// event the store lacks is store-stale; and an event naming a thread canon
/// does not have builds no thread and draws no reconciliation finding.
#[test]
fn doctor_reports_bad_event_files_and_events_the_store_lacks() {
  let forge = Forge::new();
  let (alice, _bob, _) = two_clones(&forge);
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
  let (_, _, shown) = alice.run(&["st", "show", "ST0099"]);
  assert_ne!(
    shown, 0,
    "nothing rebuilds state from events, so ST0099 exists only in the log"
  );
  let out = alice.doctor(&["--verbose"]);
  assert!(
    !out.contains("ST0099"),
    "doctor never reconciles canon against events: {out}"
  );
}

/// **Issue 0735: an event file whose `ts` is not a stamp is refused at the
/// boundary.** Every file the store wrote reads back, which is the control;
/// one carrying a stamp the store never writes is reported by doctor by path,
/// naming what it carries, and the take that sync runs writes nothing of it.
#[test]
fn an_event_file_whose_stamp_is_not_one_is_refused_and_never_taken() {
  let forge = Forge::new();
  let (alice, _bob, _) = two_clones(&forge);
  let written = alice.event_files();
  assert!(!written.is_empty(), "the clone wrote events");
  for path in &written {
    let stem = path.file_stem().expect("a file name").to_string_lossy();
    let text = std::fs::read_to_string(path).expect("read");
    intentsvcs::event::from_file(&stem, &text).unwrap_or_else(|e| {
      panic!(
        "{} was written by the store and reads back: {e}",
        path.display()
      )
    });
  }
  let day = written[0].parent().expect("day dir").to_path_buf();
  let good: serde_json::Value =
    serde_json::from_str(&std::fs::read_to_string(&written[0]).expect("read")).expect("json");

  // Two stamps the store never writes, neither a substring of the other, and
  // beside them one it does, which the same take must carry: the control that
  // the take ran at all.
  let planted = [
    ("01M2SXFFFFFFFFFFFFFFFFFFFF", "20261120"),
    ("01M2SXGGGGGGGGGGGGGGGGGGGG", "2026-11-20T17:00:00Z"),
  ];
  let taken = "01M2SXHHHHHHHHHHHHHHHHHHHH";
  let plant = |id: &str, ts: &serde_json::Value| {
    let mut event = good.clone();
    event["id"] = id.into();
    event["ts"] = ts.clone();
    std::fs::write(
      day.join(format!("{id}.json")),
      serde_json::to_string_pretty(&event).expect("json"),
    )
    .expect("write");
  };
  for (id, ts) in planted {
    plant(id, &ts.into());
  }
  plant(taken, &good["ts"]);

  let out = alice.doctor(&[]);
  let rel = day
    .strip_prefix(&alice.root)
    .expect("the day is in the clone");
  // Doctor groups findings under their class's heading, an unindented line,
  // and prints each one indented beneath it as `<file> -- <detail>`.
  let lines: Vec<&str> = out.lines().collect();
  for (id, ts) in planted {
    let file = rel.join(format!("{id}.json")).display().to_string();
    let at = lines
      .iter()
      .position(|line| {
        line.trim_start().starts_with(&format!("{file} -- ")) && line.contains(&format!("`{ts}`"))
      })
      .unwrap_or_else(|| panic!("doctor reports {file}, naming the `{ts}` it carries: {out}"));
    let heading = lines[..at]
      .iter()
      .rev()
      .find(|line| !line.starts_with(char::is_whitespace))
      .unwrap_or_else(|| panic!("{file}'s finding sits under a class heading: {out}"));
    assert!(
      heading.contains("schema-invalid"),
      "{file} is reported as schema-invalid, under `{heading}`: {out}"
    );
  }

  // `sync --apply` ends with doctor and exits with its verdict, which the two
  // misstamped files keep red; what it took is read from what it printed and
  // from the store, not from its exit code.
  let (said, _, _) = alice.run(&["sync", "--apply"]);
  assert!(
    said.contains(&format!("{} event file(s) to take", planted.len() + 1)),
    "the plan names every file the store does not hold: {said}"
  );
  assert!(
    said.contains("took 1 event file(s)"),
    "the take carries the one well-stamped file and neither misstamped one: {said}"
  );
  let events = alice.events_json();
  let rows = events["events"].as_array().expect("rows");
  assert!(
    rows.iter().any(|row| row["id"] == taken),
    "control: the well-stamped event beside them is taken: {events}"
  );
  for (id, ts) in planted {
    assert!(
      !rows.iter().any(|row| row["id"] == id),
      "the event carrying `{ts}` is never taken into the store: {events}"
    );
  }
}

/// **Issue 0738: an event-file finding gives one remedy, its own.** A
/// misstamped file and an unstamped one are each reported under a remedy line
/// that is the event error's restore-from-git remedy, and the remedies of the
/// classes they carry, which would have a person edit history by hand, are
/// printed nowhere.
#[test]
fn an_event_file_finding_gives_its_own_remedy_and_not_its_class() {
  use intentsvcs::event::EventFileError;
  use intentsvcs::finding::FindingClass;
  use intentsvcs::remedy::Remedy;
  let forge = Forge::new();
  let (alice, _bob, _) = two_clones(&forge);
  let first = alice.event_files().remove(0);
  let day = first.parent().expect("day dir").to_path_buf();
  let good: serde_json::Value =
    serde_json::from_str(&std::fs::read_to_string(&first).expect("read")).expect("json");

  let misstamped = "01M2SXJJJJJJJJJJJJJJJJJJJJ";
  let unstamped = "01M2SXKKKKKKKKKKKKKKKKKKKK";
  let cases = [
    (
      misstamped,
      "20261120",
      EventFileError::Misstamped {
        id: misstamped.to_string(),
        found: "20261120".to_string(),
      }
      .remedy(),
    ),
    (
      unstamped,
      "",
      EventFileError::Unstamped {
        id: unstamped.to_string(),
      }
      .remedy(),
    ),
  ];
  for (id, ts, _) in &cases {
    let mut event = good.clone();
    event["id"] = (*id).into();
    event["ts"] = (*ts).into();
    std::fs::write(
      day.join(format!("{id}.json")),
      serde_json::to_string_pretty(&event).expect("json"),
    )
    .expect("write");
  }

  let out = alice.doctor(&[]);
  let lines: Vec<&str> = out.lines().collect();
  let rel = day
    .strip_prefix(&alice.root)
    .expect("the day is in the clone");
  for (id, _, remedy) in &cases {
    let file = rel.join(format!("{id}.json")).display().to_string();
    let at = lines
      .iter()
      .position(|line| line.trim_start().starts_with(&format!("{file} -- ")))
      .unwrap_or_else(|| panic!("doctor reports {file}: {out}"));
    let given = lines[..at]
      .iter()
      .rev()
      .find_map(|line| line.trim_start().strip_prefix("remedy: "))
      .unwrap_or_else(|| panic!("{file}'s group carries a remedy line: {out}"));
    assert_eq!(
      given,
      remedy.as_str(),
      "{file}'s one remedy is its own: {out}"
    );
  }
  for class in [FindingClass::SchemaInvalid, FindingClass::UnknownFileShape] {
    assert!(
      !out.contains(class.remedy()),
      "`{}`'s remedy is printed nowhere, since every finding of it here has its own: {out}",
      class.as_str()
    );
  }
}

/// **AC-01.4: `upgrade` removes the retired ignore rule and adds nothing, and
/// `export` still produces the single-file form.**
#[test]
fn upgrade_retires_the_ignore_rule_and_export_keeps_one_file() {
  let forge = Forge::new();
  let (alice, _bob, _) = two_clones(&forge);
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
