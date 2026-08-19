//! AT-01.4 / ST0057 AC-01.4: two threads edited produce diffs in two distinct
//! files -- the property that rejected the consolidated `threads.jsonl`
//! (D57-1 option B).
//!
//! **THE ASSERTION IS DISJOINTNESS, NOT A COUNT OF TWO.** The criterion says
//! it is checked "by editing two threads and observing the changed-path set,
//! not by inspecting the layout", and `changed.len() == 2` IS an inspection of
//! the layout: it hard-codes flat one-file-per-thread and would go red on a
//! future nested canon that is perfectly separable. Disjointness is the
//! property itself -- under `threads.jsonl` both edits touch the same path, so
//! the two sets are IDENTICAL rather than disjoint, and the test refutes the
//! rejected design by construction rather than by counting.
//!
//! **BOTH SETS ARE REQUIRED NON-EMPTY, AND THAT GUARD IS THE LOAD-BEARING
//! HALF.** The empty set is disjoint from everything, so if either edit
//! silently wrote no canon at all this test would pass at its loudest while
//! measuring nothing -- the "a zero is not a result until the check has
//! produced a non-zero" class, in set form, where it is easy to miss because
//! the disjointness assertion still reads as meaningful.
//!
//! **No git.** The changed-path set is computed from content hashes over the
//! canon tree, so this needs no repository, takes no index lock, and cannot
//! contend with anything else reading the estate.

mod common;

use common::Fixture;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;

/// Content hash of every canon file, keyed by path relative to the canon root.
///
/// **Walked recursively.** Scoping this to `st/` and `issues/` would make the
/// test blind to a SHARED file elsewhere under `.canon/` -- an index, a
/// manifest -- which is the very shape the criterion rejects: both edits would
/// touch it, the sets would overlap, and the test would never look.
fn canon_snapshot(root: &Path) -> BTreeMap<String, u64> {
  let canon_root = root.join("intent/.canon");
  let mut seen = BTreeMap::new();
  walk(&canon_root, &canon_root, &mut seen);
  seen
}

fn walk(base: &Path, dir: &Path, seen: &mut BTreeMap<String, u64>) {
  let Ok(entries) = std::fs::read_dir(dir) else { return };
  for entry in entries.flatten() {
    let path = entry.path();
    if path.is_dir() {
      walk(base, &path, seen);
    } else {
      let bytes = std::fs::read(&path).expect("read canon file");
      let rel = path.strip_prefix(base).expect("under canon root");
      seen.insert(rel.to_string_lossy().into_owned(), hash_of(&bytes));
    }
  }
}

/// Any stable content hash; the value is never compared across runs, only
/// before-versus-after within one run.
fn hash_of(bytes: &[u8]) -> u64 {
  use std::hash::{Hash, Hasher};
  let mut h = std::collections::hash_map::DefaultHasher::new();
  bytes.hash(&mut h);
  h.finish()
}

/// The set of canon paths whose bytes differ between two snapshots, including
/// paths that appeared or vanished.
fn changed(before: &BTreeMap<String, u64>, after: &BTreeMap<String, u64>) -> BTreeSet<String> {
  let mut moved = BTreeSet::new();
  for (path, hash) in after {
    if before.get(path) != Some(hash) {
      moved.insert(path.clone());
    }
  }
  for path in before.keys() {
    if !after.contains_key(path) {
      moved.insert(path.clone());
    }
  }
  moved
}

/// Mint two threads in a pristine fixture, edit exactly ONE of them, and
/// report which canon files moved.
///
/// Each call gets its own fixture so both measurements start from an identical
/// estate; the only difference between the two runs is which thread was
/// edited, which is what makes the comparison mean anything.
fn canon_touched_by_editing(which: usize) -> BTreeSet<String> {
  let fx = Fixture::new();
  let ids = [
    fx.facade().st_new("first thread").expect("mint first"),
    fx.facade().st_new("second thread").expect("mint second"),
  ];
  fx.facade().sync_to_disk().expect("settle canon before measuring");

  let before = canon_snapshot(fx.root());
  fx.facade()
    .st_hold(&ids[which], "edited to observe the changed-path set")
    .expect("st hold");
  fx.facade().sync_to_disk().expect("sync to disk");
  changed(&before, &canon_snapshot(fx.root()))
}

#[test]
fn two_threads_edited_produce_diffs_in_distinct_files() {
  let first = canon_touched_by_editing(0);
  let second = canon_touched_by_editing(1);

  // The guard without which disjointness is vacuous.
  assert!(!first.is_empty(), "editing the first thread moved no canon file at all -- nothing was measured");
  assert!(!second.is_empty(), "editing the second thread moved no canon file at all -- nothing was measured");

  let shared: Vec<&String> = first.intersection(&second).collect();
  assert!(
    shared.is_empty(),
    "editing the first thread and editing the second both write {} shared canon file(s), so their diffs are not \
     separable -- this is the consolidated-file shape D57-1 rejected.\n  first  -> {:?}\n  second -> {:?}\n  shared -> {:?}",
    shared.len(),
    first,
    second,
    shared
  );
}
