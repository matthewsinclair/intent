//! **Issue 0612: a sender commits its own message, and main is right.**
//!
//! `wb ask` stored the message in the RECIPIENT's `board.json`, so no commit
//! the sender could make left main right (vc's measurement, 2026-10-22). With
//! the event and the recipient's inbox view, a fresh checkout's doctor counted
//! view skew on that inbox (arm M). With the event alone, a fresh store held no
//! message (arm I).
//!
//! The fix gives each canon file and each view one writer, so the commit rule
//! is mechanical: a node commits what its own verbs wrote, which is its own
//! `board.json` and `wip.md`, every `<peer>/inbox.<node>.md`, and its own event
//! files. This arm commits exactly that set after one ask and reads the commit
//! the way a teammate's clone does: a cold store, then doctor.

use crate::common::{Fixture, ctx};
use intentsvcs::finding::FindingClass;
use intentsvcs::sync::Scope;

const BODY: &str = "hello from aa";

/// Run git at `root` and return what it printed, RAISING on failure: a git
/// that failed quietly would leave the commit this arm reads unmade.
fn git(root: &std::path::Path, args: &[&str]) -> String {
  let out = std::process::Command::new("git")
    .args(args)
    .current_dir(root)
    .output()
    .expect("run git");
  assert!(
    out.status.success(),
    "git {args:?} failed at {}: {}",
    root.display(),
    String::from_utf8_lossy(&out.stderr)
  );
  String::from_utf8(out.stdout).expect("git prints utf-8")
}

/// What a teammate's clone of `fx`'s HEAD reads: how many messages from aa to
/// bb carrying [`BODY`] its store holds, and every view-skew finding its
/// doctor counts. The clone has the committed tree and no store until its
/// first open warms one from the extract.
fn a_clone_reads(fx: &Fixture) -> (i64, Vec<String>) {
  let clone = Fixture {
    dir: tempfile::tempdir().expect("tempdir"),
  };
  git(
    clone.root(),
    &["clone", "-q", &fx.root().to_string_lossy(), "."],
  );
  drop(clone.facade_on_disk());
  let held: i64 = rusqlite::Connection::open(clone.project().db_path())
    .expect("the clone's store")
    .query_row(
      "SELECT count(*) FROM wb_message WHERE sender = 'aa' AND recipient = 'bb' AND body = ?1",
      [BODY],
      |r| r.get(0),
    )
    .expect("count the message");
  let skew = intentsvcs::doctor::diagnose(
    &clone.project(),
    &ctx(),
    None,
    intentsvcs::doctor::Scope::All,
  )
  .findings
  .into_iter()
  .filter(|f| f.class == FindingClass::ViewSkew)
  .map(|f| format!("{}: {}", f.file, f.detail))
  .collect();
  (held, skew)
}

#[test]
fn a_fresh_checkout_of_what_the_sender_committed_holds_the_message_and_reads_clean() {
  let fx = Fixture::new();
  fx.write_file(".gitignore", "intent/.cache/\n");
  fx.git_init();
  let mut f = fx.facade_on_disk();
  f.wb_register("aa", "Node A", "worker")
    .expect("register aa");
  f.wb_register("bb", "Node B", "worker")
    .expect("register bb");
  // Every view the estate renders, the aggregate ones included, so the
  // baseline is a whole estate and doctor's only subject is the ask.
  f.sync_to_disk(&Scope::All).expect("render every view");
  drop(f);
  fx.git_commit_all();
  assert_eq!(
    a_clone_reads(&fx),
    (0, Vec::<String>::new()),
    "control: a clone of the baseline holds no message and its doctor counts no view skew"
  );

  let mut f = fx.facade_on_disk();
  f.wb_ask("aa", "bb", BODY, None, false).expect("aa asks bb");
  let written = f.take_written("aa");
  drop(f);

  // aa's set by the mechanical rule: its `board.json`, its inbox view in bb's
  // directory, its event, and its `wip.md` only if the ask changed it, which
  // `git add` of an unchanged tracked file decides by staging nothing. Its
  // event file is the one untracked event, because aa is the only node that
  // acted after the baseline.
  let project = fx.project();
  let events = git(
    fx.root(),
    &[
      "ls-files",
      "--others",
      "--exclude-standard",
      "--",
      &project.relative(&project.events_dir()),
    ],
  );
  let events: Vec<&str> = events.lines().collect();
  assert_eq!(
    events.len(),
    1,
    "precondition: the ask recorded one event file: {events:?}"
  );
  let mut mine = vec![
    project.relative(&project.board_json("aa")),
    project.relative(&project.wb_board_view("aa")),
    project.relative(&project.wb_inbox_view("bb", "aa")),
  ];
  mine.extend(events.iter().map(|e| e.to_string()));
  let mut add = vec!["add", "--"];
  add.extend(mine.iter().map(String::as_str));
  git(fx.root(), &add);
  git(fx.root(), &["commit", "-q", "-m", "aa asks bb"]);

  assert_eq!(
    a_clone_reads(&fx),
    (1, Vec::<String>::new()),
    "a fresh checkout of what aa's own verbs wrote holds aa's message, and its doctor counts no view skew"
  );

  // The verb's `wrote:` lines name exactly what was committed, and nothing it
  // landed is anyone else's.
  let mut committed: Vec<String> = git(fx.root(), &["show", "--name-only", "--format=", "HEAD"])
    .lines()
    .map(str::to_string)
    .collect();
  committed.sort();
  let mut wrote = written.wrote.clone();
  wrote.sort();
  assert_eq!(
    (wrote, written.re_rendered),
    (committed, Vec::new()),
    "the ask's wrote: lines are the set aa committed"
  );
}
