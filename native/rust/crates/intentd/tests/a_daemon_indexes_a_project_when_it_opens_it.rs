//! AT-22.4.
//!
//! `AC-22.4`: **a daemon builds its index when it opens a project**, so a
//! daemon-served search finds a file below the root that no watcher event ever
//! named.
//!
//! **THE FILE IS WRITTEN BEFORE ANY DAEMON EXISTS, AND THAT IS THE WHOLE TEST.**
//! Until issue `0366` the daemon's index grew only as its watcher named paths,
//! so a project opened over a quiet tree was indexed at the root and nowhere
//! below it -- measured on a fresh clone, where every row the index held was a
//! root-level file. A search answered from that index reads as an absence. A
//! file written after the daemon started would reach the index through the
//! watcher and pass whether or not the open builds anything.

use std::path::Path;

use crate::common::{ATTEMPTS, PAUSE, RunningDaemon};
use intentsvcs::search::SearchQuery;
use intentsvcs::wire::{Op, Request, Response};

/// A word nothing else in a fresh project contains.
const MARKER: &str = "zanzibarquux";
const BELOW_THE_ROOT: &str = "notes/deep/marker.md";

/// Does a search served by this daemon name `rel` among its hits?
fn daemon_search_names(daemon: &RunningDaemon, root: &Path, rel: &str) -> bool {
  let response = daemon.ask(Request {
    root: root.to_path_buf(),
    op: Op::Search {
      query: MARKER.to_string(),
      ask: SearchQuery {
        kinds: Vec::new(),
        tiers: Vec::new(),
        langs: Vec::new(),
        path: None,
        limit: None,
        subkinds: Vec::new(),
        container: None,
      },
    },
  });
  let Response::Search { answer } = response else {
    panic!("intentd answered Op::Search with something else: {response:?}");
  };
  answer["groups"]
    .as_array()
    .into_iter()
    .flatten()
    .flat_map(|group| group["hits"].as_array().into_iter().flatten())
    .any(|hit| hit["path"] == rel)
}

#[test]
fn a_daemon_indexes_a_project_when_it_opens_it() {
  let root = crate::common::project("OpenIndexed");
  std::fs::create_dir_all(root.join("notes/deep")).expect("mkdir notes/deep");
  std::fs::write(
    root.join(BELOW_THE_ROOT),
    format!("# Marker\n\nThe {MARKER} line is below the root.\n"),
  )
  .expect("write the marker");

  let daemon = RunningDaemon::start();

  // **POLLED, BECAUSE THE BUILD IS INTERLEAVED WITH CLIENTS.** The first search
  // is also the op that opens the project, and it is answered before the build
  // has reached `notes/`; the claim is that the build reaches it, not that it
  // wins that race.
  let mut found = false;
  for _ in 0..ATTEMPTS {
    if daemon_search_names(&daemon, &root, BELOW_THE_ROOT) {
      found = true;
      break;
    }
    std::thread::sleep(PAUSE);
  }
  assert!(
    found,
    "a daemon-served search never found `{BELOW_THE_ROOT}`, which was on disk before the daemon started: the daemon did not build its index when it opened the project"
  );

  let _ = std::fs::remove_dir_all(&root);
}
