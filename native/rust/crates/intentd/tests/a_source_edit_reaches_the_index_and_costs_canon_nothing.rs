//! `AC-18.4`'s daemon half: **a source edit reaches the index, and the canon
//! path does not notice.**
//!
//! **THE TWO CLAIMS ARE THE RULING** (vc, 2026-09-12). WP-18's index scope is
//! the gitignore-aware repository, which is far wider than the canon corpus of
//! the named root files plus `intent/`. Serving it by WIDENING the canon
//! watcher's registration was built, measured and rejected: three of four full
//! daemon-suite runs went red against none of four without it, because a
//! coalesced root event made the canon reconcile answer against a lagging
//! `file_index`, publish, and ingest -- and the ingest's own writes coalesced
//! back to the root.
//!
//! So there are TWO registrations. This file holds the property that makes that
//! shape worth its second debounced stream: a file that only the index cares
//! about reaches the index, and costs the canon path ZERO ingests.

use std::path::Path;

use crate::common::{ATTEMPTS, PAUSE, RunningDaemon};
use intentsvcs::wire::{Op, Request, Response};

/// The count of ingests the daemon has run for this project.
fn ingested(daemon: &RunningDaemon, root: &Path) -> u64 {
  let response = daemon.ask(Request {
    root: root.to_path_buf(),
    op: Op::Registry,
  });
  let Response::Registry { projects } = response else {
    panic!("intentd answered Op::Registry with something else: {response:?}");
  };
  let wanted = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
  projects
    .iter()
    .find(|p| p.root.canonicalize().unwrap_or_else(|_| p.root.clone()) == wanted)
    .map(|p| p.ingested)
    .unwrap_or(0)
}

/// Does the index hold a row for this project-relative path?
///
/// **READ FROM THE STORE THE DAEMON IS WRITING, WHICH IS WHAT MAKES IT A
/// DAEMON TEST.** `one_store_door` forbids a store handle in `intentd`'s own
/// source, not in a test observing it, and SQLite's WAL admits a reader beside
/// the writer.
fn index_holds(root: &Path, rel: &str) -> bool {
  let Ok(store) = intentsvcs::store::Store::open(&root.join("intent/.cache/intent.db")) else {
    return false;
  };
  store
    .index_files()
    .map(|rows| rows.iter().any(|r| r.path == rel))
    .unwrap_or(false)
}

#[test]
fn a_source_edit_reaches_the_index_and_costs_canon_nothing() {
  let daemon = RunningDaemon::start();
  let root = crate::common::project("Indexed");

  // **ASKING THE REGISTRY DOES NOT REGISTER THE PROJECT, AND A WATCH IS ONLY
  // STARTED WHEN IT IS.** `Op::Registry` LISTS what the daemon holds; routing a
  // project-scoped op is what makes it hold this one. Without this the test
  // waits for events from a watcher that was never started, and reports the
  // dispatch as broken -- which is exactly what it did on its first run.
  let registered = daemon.ask(Request {
    root: root.to_path_buf(),
    op: Op::ThreadList,
  });
  assert!(
    matches!(registered, Response::Threads { .. }),
    "the project did not register: {registered:?}"
  );

  // Let the project's own creation settle, so the baseline is a resting count
  // rather than one still climbing. Without this the zero-ingest claim below
  // would be measuring the fixture rather than the edit.
  for _ in 0..50 {
    std::thread::sleep(PAUSE);
  }
  let before = ingested(&daemon, &root);

  // A file NO canon corpus contains: source, outside `intent/`, not a root file.
  std::fs::create_dir_all(root.join("src")).expect("mkdir src");
  std::fs::write(root.join("src/lib.rs"), b"pub fn answer() -> u32 { 42 }\n").expect("write");

  let mut reached = false;
  for _ in 0..ATTEMPTS {
    if index_holds(&root, "src/lib.rs") {
      reached = true;
      break;
    }
    std::thread::sleep(PAUSE);
  }
  assert!(
    reached,
    "a source edit never reached the index. The index registration is not watching the repository, or its events are not reaching `index_refresh`"
  );

  // **AND THE CANON PATH DID NOT MOVE.** This is the half that the single
  // widened registration could not deliver: a source file is not canon, so an
  // ingest over it is the daemon doing work nobody asked for -- and it is the
  // first step of the loop that was measured at three of four.
  assert_eq!(
    ingested(&daemon, &root),
    before,
    "a source edit cost the canon path an ingest. `src/lib.rs` is in no canon corpus, so the index stream has reached the canon dispatch"
  );

  let _ = std::fs::remove_dir_all(&root);
}
