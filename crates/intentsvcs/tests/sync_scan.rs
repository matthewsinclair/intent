//! AT-03.3 / AC-03.3: the scan detects external edits by content hash,
//! including a same-size same-mtime rewrite.
//!
//! That last case is the whole test. design.md sketched "SHA-256 rehash on
//! change", gating the hash on a stat comparison -- and a rewrite that
//! preserves size and mtime is invisible to stat by construction, so the gated
//! design cannot see it. The contract governs over the narrative (vc ruling),
//! and this file is what holds the hash-always decision in place: gate the
//! hash on stat again and `a_same_size_same_mtime_rewrite_is_detected` goes
//! red immediately.

mod common;

use std::fs::{File, FileTimes};

use common::Fixture;
use intentsvcs::sync::{self, FileState};

fn state_of(entries: &[sync::FileEntry], path: &str) -> FileState {
  entries
    .iter()
    .find(|e| e.path == path)
    .unwrap_or_else(|| {
      panic!(
        "{path} not in the index; indexed: {:?}",
        entries.iter().map(|e| &e.path).collect::<Vec<_>>()
      )
    })
    .state
}

#[test]
fn a_first_scan_reports_everything_changed() {
  let fx = Fixture::new();
  fx.write_file("intent/wip.md", "work in progress\n");

  let entries = sync::scan(fx.root(), &[]).expect("scan");
  assert_eq!(state_of(&entries, "intent/wip.md"), FileState::Changed);
}

#[test]
fn an_unmodified_file_is_clean_on_rescan() {
  let fx = Fixture::new();
  fx.write_file("intent/wip.md", "work in progress\n");

  let first = sync::scan(fx.root(), &[]).expect("first scan");
  let second = sync::scan(fx.root(), &first).expect("second scan");
  assert_eq!(state_of(&second, "intent/wip.md"), FileState::Clean);
}

/// The named case: content changes, stat does not.
#[test]
fn a_same_size_same_mtime_rewrite_is_detected() {
  let fx = Fixture::new();
  fx.write_file("intent/wip.md", "aaaa\n");
  let first = sync::scan(fx.root(), &[]).expect("first scan");

  let path = fx.path("intent/wip.md");
  let meta = std::fs::metadata(&path).expect("metadata");
  let (modified, accessed) = (
    meta.modified().expect("mtime"),
    meta
      .accessed()
      .unwrap_or_else(|_| meta.modified().expect("mtime")),
  );

  // Same byte count, different bytes.
  fx.write_file("intent/wip.md", "bbbb\n");
  File::options()
    .write(true)
    .open(&path)
    .expect("reopen")
    .set_times(
      FileTimes::new()
        .set_modified(modified)
        .set_accessed(accessed),
    )
    .expect("restore mtime");

  let after = std::fs::metadata(&path).expect("metadata");
  assert_eq!(after.len(), meta.len(), "precondition: size is unchanged");
  assert_eq!(
    after.modified().expect("mtime"),
    modified,
    "precondition: mtime is unchanged -- without this the test proves nothing, because an ordinary stat-visible edit would pass a hash-gated scan too"
  );

  let second = sync::scan(fx.root(), &first).expect("second scan");
  assert_eq!(
    state_of(&second, "intent/wip.md"),
    FileState::Changed,
    "content identity is decided by SHA-256, never by stat"
  );
}

#[test]
fn the_scan_is_ordered_and_scoped() {
  let fx = Fixture::new();
  fx.write_file("intent/b.md", "b\n");
  fx.write_file("intent/a.md", "a\n");
  fx.write_file("intent/.cache/intent.db", "binary-ish\n");
  fx.write_file("intent/.treeindex/bin/.treeindex", "cache\n");
  fx.write_file("AGENTS.md", "root canon\n");
  fx.write_file("README.md", "not in scope\n");

  let entries = sync::scan(fx.root(), &[]).expect("scan");
  let paths: Vec<&str> = entries.iter().map(|e| e.path.as_str()).collect();

  let mut sorted = paths.clone();
  sorted.sort_unstable();
  assert_eq!(
    paths, sorted,
    "the index is path-ordered, so two scans are comparable without the caller sorting"
  );

  assert!(
    paths.contains(&"AGENTS.md"),
    "named root files are in scope"
  );
  assert!(paths.contains(&"intent/a.md"));
  assert!(
    !paths.iter().any(|p| p.starts_with("intent/.cache/")),
    "the rebuildable DB is not indexed -- indexing the index is circular: {paths:?}"
  );
  assert!(
    !paths.iter().any(|p| p.starts_with("intent/.treeindex/")),
    "the derived treeindex cache is not indexed: {paths:?}"
  );
  assert!(
    !paths.contains(&"README.md"),
    "the root-file list is explicit, not a glob over the repo root: {paths:?}"
  );
}

#[test]
fn the_hash_is_the_content_hash() {
  let fx = Fixture::new();
  fx.write_file("intent/wip.md", "hello\n");
  let entries = sync::scan(fx.root(), &[]).expect("scan");
  let entry = entries
    .iter()
    .find(|e| e.path == "intent/wip.md")
    .expect("indexed");
  // sha256("hello\n") -- a fixed, externally checkable value, so the test
  // pins the algorithm rather than merely pinning it to itself.
  assert_eq!(
    entry.sha256,
    "5891b5b522d5df086d0ff0b110fbd9d21bb4fc7163af34d08286a2e846f6be03"
  );
  assert_eq!(entry.size, 6);
}
