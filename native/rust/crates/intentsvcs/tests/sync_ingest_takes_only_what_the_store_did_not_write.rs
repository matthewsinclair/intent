//! ST0078 AT-03.3 (AC-03.3): **`intent sync --apply` is the daemon's pass
//! from the command line, and it takes the files only where they say something
//! the store did not write.**
//!
//! The door is `Facade::ingest_from_disk`, the call `intentd` makes after a
//! watched change, so these arms hold the one engine both callers run (D32).
//! The rule, as vc worded AC-03.3 on 2026-09-18: a file that differs from what
//! the store recorded writing is taken, a recorded file the pull removed
//! included; a row whose file was never written is an unprojected local act and
//! survives; and a peer's write that lands mid-pass is neither reverted nor
//! reported as something this pass took.
//!
//! **WHAT THE PASS REPORTS IS PART OF THE CONTRACT**, because the git hooks
//! print on it and on nothing else: `taken` lists the subjects whose stored
//! value changed, and an empty list means the store already answered what the
//! files say.

use crate::common::{Fixture, sample_thread};
use intentsvcs::facade::IngestCommit;
use intentsvcs::sync::Scope;

/// A project with ST0001 on disk and in a warm on-disk store, which recorded
/// writing its canon file when it warmed.
fn warm_with_one_thread() -> Fixture {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  fx.facade_on_disk();
  fx
}

fn stored_ids(fx: &Fixture) -> Vec<String> {
  let store = intentsvcs::store::Store::open(&fx.project().db_path()).expect("the store opens");
  let (threads, _) = store.load_canon().expect("the store reads back");
  threads.into_iter().map(|t| t.id).collect()
}

fn stored_title(fx: &Fixture, id: &str) -> String {
  let store = intentsvcs::store::Store::open(&fx.project().db_path()).expect("the store opens");
  let (threads, _) = store.load_canon().expect("the store reads back");
  threads
    .into_iter()
    .find(|t| t.id == id)
    .unwrap_or_else(|| panic!("{id} is not in the store"))
    .title
}

#[test]
fn a_file_the_store_did_not_write_is_taken_and_named() {
  let fx = warm_with_one_thread();
  let mut pulled = sample_thread("ST0001");
  pulled.title = "the title a pull brought".to_string();
  fx.write_thread(&pulled);
  fx.write_thread(&sample_thread("ST0002"));

  let ingested = fx
    .facade_on_disk()
    .ingest_from_disk(&Scope::All)
    .expect("the pass runs");

  assert_eq!(
    ingested.taken,
    vec!["ST0001".to_string(), "ST0002".to_string()],
    "the changed file and the new one are the pass's changes, named"
  );
  assert_eq!(stored_title(&fx, "ST0001"), "the title a pull brought");
  assert!(stored_ids(&fx).contains(&"ST0002".to_string()));
}

#[test]
fn a_second_pass_over_the_same_files_takes_nothing() {
  let fx = warm_with_one_thread();
  fx.write_thread(&sample_thread("ST0002"));
  let mut f = fx.facade_on_disk();
  f.ingest_from_disk(&Scope::All)
    .expect("the first pass runs");

  let again = f
    .ingest_from_disk(&Scope::All)
    .expect("the second pass runs");
  assert!(
    again.taken.is_empty(),
    "the store already holds what the files say, so the hooks must print nothing: {:?}",
    again.taken
  );
  assert!(
    intentsvcs::sync::ingested(&again.taken).starts_with("nothing"),
    "and the confirmation says so without the word the hooks print on"
  );
}

#[test]
fn a_recorded_file_the_pull_removed_takes_the_removal() {
  let fx = warm_with_one_thread();
  fx.write_thread(&sample_thread("ST0002"));
  fx.facade_on_disk()
    .ingest_from_disk(&Scope::All)
    .expect("ST0002 arrives, and the store records writing its file");

  // A peer's `git rm`, or a renumber arriving: the file the store wrote is
  // gone, and so are the views git carried beside it. A thread directory left
  // with no canon reads as an unmigrated v2 thread, which a pull never leaves.
  std::fs::remove_file(fx.canon_path("ST0002")).expect("the pull removes the file");
  let _ = std::fs::remove_dir_all(fx.project().thread_dir("ST0002"));
  let ingested = fx
    .facade_on_disk()
    .ingest_from_disk(&Scope::All)
    .expect("the pass runs");

  assert_eq!(
    ingested.taken,
    vec!["ST0002 (removed)".to_string()],
    "a removal is a change the pass made, and it says it was a removal"
  );
  assert_eq!(
    stored_ids(&fx),
    vec!["ST0001".to_string()],
    "the store answers from the merged canon, which no longer has ST0002"
  );
}

#[test]
fn a_row_whose_file_was_never_written_survives() {
  let fx = warm_with_one_thread();
  // A row only the store has: nothing ever wrote its canon file, and the file
  // index has no record of one. That is an unprojected local act.
  {
    let mut store =
      intentsvcs::store::Store::open(&fx.project().db_path()).expect("the store opens");
    let (mut threads, issues) = store.load_canon().expect("the store reads back");
    threads.push(sample_thread("ST0009"));
    store
      .rebuild(&threads, &issues)
      .expect("the row lands in the store alone");
  }
  assert!(
    !fx.canon_path("ST0009").exists(),
    "the fixture wrote no file"
  );

  let ingested = fx
    .facade_on_disk()
    .ingest_from_disk(&Scope::All)
    .expect("the pass runs");

  assert!(
    stored_ids(&fx).contains(&"ST0009".to_string()),
    "a row whose file was never written is not a deletion, and the pass kept it"
  );
  assert!(
    !ingested.taken.iter().any(|s| s.starts_with("ST0009")),
    "and it is not reported as taken: {:?}",
    ingested.taken
  );
}

#[test]
fn a_peers_write_inside_the_pass_is_neither_reverted_nor_counted_as_taken() {
  let fx = warm_with_one_thread();
  let mut peer = fx.facade_on_disk();
  let mut pass = fx.facade_on_disk();

  let render = pass.ingest_render(&Scope::All).expect("the pass renders");
  let number = peer
    .issue_add("filed mid-pass", None, Some("tester"), "the peer's body")
    .expect("the peer's write lands while the pass holds its render");
  assert!(
    matches!(
      pass
        .ingest_commit(&Scope::All, render)
        .expect("the commit runs"),
      IngestCommit::StoreMoved { .. }
    ),
    "a render older than the store lands nothing (issue 0441)"
  );

  let ingested = pass
    .ingest_from_disk(&Scope::All)
    .expect("the pass renders again and lands");
  assert!(
    ingested.taken.is_empty(),
    "the peer's issue is the peer's write, not something this pass took: {:?}",
    ingested.taken
  );
  assert_eq!(
    fx.facade_on_disk()
      .issue_show(number)
      .expect("the peer's issue is still in the store")
      .body,
    "the peer's body"
  );
}

/// **THE PLAN NAMES WHAT THE APPLY TAKES, AND TAKES NOTHING ITSELF** (hv's
/// ruling of 2026-09-18: bare `intent sync` is the plan). Both halves run the
/// one engine -- the plan against a shadow of the held estate -- so a pull's
/// arrival, its edit and its removal are named the same way by both.
#[test]
fn the_plan_predicts_the_apply_and_writes_nothing() {
  let fx = warm_with_one_thread();
  fx.write_thread(&sample_thread("ST0002"));
  fx.write_thread(&sample_thread("ST0003"));
  fx.facade_on_disk()
    .ingest_from_disk(&Scope::All)
    .expect("ST0002 and ST0003 arrive and are recorded");

  let mut edited = sample_thread("ST0001");
  edited.title = "the title a pull brought".to_string();
  fx.write_thread(&edited);
  std::fs::remove_file(fx.canon_path("ST0003")).expect("the pull removes ST0003");
  let _ = std::fs::remove_dir_all(fx.project().thread_dir("ST0003"));
  let before = crate::common::tree(fx.root());
  let stored_before = stored_ids(&fx);

  let plan = fx
    .facade_on_disk()
    .sync_plan(&Scope::All)
    .expect("the plan is computed");
  let ingest = plan
    .steps
    .iter()
    .find(|s| s.name() == "ingest")
    .expect("the plan has an ingest step");
  assert_eq!(
    ingest.recoverability,
    intentsvcs::plan::Recoverability::Quiet,
    "the ingest never asks, which is what lets a hook run it"
  );
  let intentsvcs::plan::Action::Ingest {
    would_take: Some(would_take),
  } = &ingest.action
  else {
    panic!("the ingest was previewed: {ingest:?}");
  };
  assert_eq!(
    would_take,
    &vec!["ST0001".to_string(), "ST0003 (removed)".to_string()]
  );

  let after = crate::common::tree(fx.root());
  let moved: Vec<_> = after
    .keys()
    .chain(before.keys())
    .filter(|k| !k.contains(".cache/") && after.get(*k) != before.get(*k))
    .collect();
  assert!(moved.is_empty(), "the plan wrote files: {moved:?}");
  assert_eq!(
    stored_ids(&fx),
    stored_before,
    "and moved nothing in the store"
  );
  assert_eq!(stored_title(&fx, "ST0001"), sample_thread("ST0001").title);

  let applied = fx
    .facade_on_disk()
    .ingest_from_disk(&Scope::All)
    .expect("the ingest runs");
  assert_eq!(
    &applied.taken, would_take,
    "the apply took what the plan named"
  );
}
