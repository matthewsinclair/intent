//! **A write moves mtime on EXACTLY the files whose bytes changed, and no
//! others** -- measured over the FILE ESTATE before and after, at every
//! user-visible verb that writes it.
//!
//! The criterion reached this form after four corrections from three nodes,
//! and each one is a constraint on this file:
//!
//! - **The SUBJECT is the verb, never a type.** An earlier guard sat on
//!   `views::write_all` -- correct, and reached by nothing, because every
//!   caller was a test. `view_determinism.rs` drove that function directly and
//!   was green throughout while the estate churned. **An internal subject lets
//!   the test reach PAST the thing being tested**, so nothing here calls
//!   `WriteSet` or `write_all`; it drives what a user runs.
//! - **The DENOMINATOR is the file estate, not the write set.** `WriteSet` is
//!   internal too, and a denominator taken from it would be blind to any write
//!   that never joins one. Walking the tree needs no internal type and catches
//!   a bypassing write for free.
//! - **The PROPERTY is `moved == changed`, not "the second run writes zero".**
//!   Run-twice-writes-zero is inapplicable to the 27 mutating verbs, where
//!   some files SHOULD change -- so it would exempt the criterion exactly
//!   where the churn is worst. It is a corollary here, not the test.
//! - **The EXTENT is every verb that writes.** One verb under-covers: six
//!   commit sites serve 31 user-visible verbs, and a row naming only `sync`
//!   would leave `todo update`, `upgrade` and the whole `apply` family green
//!   while churning.
//!
//! **No clock (D42).** Every file is aged to a fixed synthetic stamp between
//! the two observations, so a rewritten file carries whatever the filesystem
//! gave it and a skipped file still carries the constant exactly. A
//! sleep-based version passes vacuously on a coarse-resolution filesystem --
//! which is the very failure this criterion detects, so the instrument must
//! not be able to fail that way itself.

mod common;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use common::{Fixture, sample_thread};
use intentsvcs::model::TShirt;

/// 2001-09-09, and obviously synthetic on sight. An absolute constant makes
/// the assertion "the mtime is still exactly this" rather than "it is still
/// roughly where I put it".
fn aged() -> SystemTime {
  UNIX_EPOCH + Duration::from_secs(1_000_000_000)
}

/// Every file under the project root, with its bytes. The store lives in
/// `intent/.cache/` and is not a projected artefact, so it is excluded --
/// including it would report the DB's own writes as estate churn.
fn walk(root: &Path, out: &mut Vec<PathBuf>) {
  let Ok(entries) = std::fs::read_dir(root) else {
    return;
  };
  for entry in entries.flatten() {
    let path = entry.path();
    if path.is_dir() {
      if path.file_name().is_some_and(|n| n == ".cache") {
        continue;
      }
      walk(&path, out);
    } else if path.is_file() {
      out.push(path);
    }
  }
}

fn snapshot(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
  let mut paths = Vec::new();
  walk(root, &mut paths);
  paths
    .into_iter()
    .filter_map(|p| std::fs::read(&p).ok().map(|bytes| (p, bytes)))
    .collect()
}

/// Stamp every file in the estate with the synthetic constant.
fn age_everything(before: &BTreeMap<PathBuf, Vec<u8>>) {
  for path in before.keys() {
    std::fs::File::options()
      .write(true)
      .open(path)
      .expect("open to age")
      .set_modified(aged())
      .expect("age");
  }
}

/// The two sets the criterion compares, as project-relative paths.
struct Verdict {
  moved: Vec<String>,
  changed: Vec<String>,
}

fn verdict(root: &Path, before: &BTreeMap<PathBuf, Vec<u8>>) -> Verdict {
  let after = snapshot(root);
  let rel = |p: &Path| {
    p.strip_prefix(root)
      .unwrap_or(p)
      .to_string_lossy()
      .into_owned()
  };

  // A file the verb CREATED is neither moved nor changed: it has no prior
  // mtime to move and no prior bytes to differ from. Counting a creation as
  // churn would red every verb that legitimately adds a file, so both sets are
  // taken over paths present in BOTH observations.
  let mut moved = Vec::new();
  let mut changed = Vec::new();
  for (path, before_bytes) in before {
    let Some(after_bytes) = after.get(path) else {
      continue;
    };
    let still_aged = std::fs::metadata(path)
      .and_then(|m| m.modified())
      .map(|m| m == aged())
      .unwrap_or(false);
    if !still_aged {
      moved.push(rel(path));
    }
    if before_bytes != after_bytes {
      changed.push(rel(path));
    }
  }
  Verdict { moved, changed }
}

/// Run `verb` against an aged estate and require `moved == changed`.
///
/// **`expect_change` is a POSITIVE CONTROL and it is not optional.** The first
/// run of this file had `st start` PASSING against a fixture whose threads
/// were already `Wip`: the gate refused the transition, the verb wrote
/// nothing, and `moved == changed == {}` held perfectly. **Emptiness from a
/// verb that never fired is indistinguishable from a verb that fired
/// cleanly** (ic's formulation), so a mutating verb must PROVE it mutated
/// before its own quietness counts for anything.
fn assert_moves_only_what_changed(
  fx: &Fixture,
  label: &str,
  expect_change: bool,
  verb: impl FnOnce(&Fixture),
) {
  let before = snapshot(fx.root());
  assert!(
    !before.is_empty(),
    "{label}: precondition -- the estate is not empty"
  );
  age_everything(&before);

  verb(fx);

  let Verdict { moved, changed } = verdict(fx.root(), &before);
  assert_eq!(
    !changed.is_empty(),
    expect_change,
    "{label}: POSITIVE CONTROL failed -- expected the verb to change \
     {} file(s), and it changed {}. A quiet verb that was REFUSED proves \
     nothing about a writer.",
    if expect_change { "some" } else { "no" },
    changed.len()
  );
  assert_eq!(
    moved,
    changed,
    "{label}: {} file(s) moved mtime, {} changed bytes, over an estate of {}.\n  \
     moved-but-unchanged: {:?}",
    moved.len(),
    changed.len(),
    before.len(),
    moved
      .iter()
      .filter(|p| !changed.contains(p))
      .collect::<Vec<_>>()
  );
}

/// A fixture carrying enough estate for the denominator to be interesting:
/// several threads, so a one-thread mutation leaves most views untouched.
fn seeded() -> Fixture {
  let fx = Fixture::new();
  for id in ["ST0001", "ST0002", "ST0003"] {
    fx.write_thread(&sample_thread(id));
  }
  let mut facade = fx.facade();
  facade.sync_from_disk().expect("ingest the seeded canon");
  facade.sync_to_disk().expect("project it back");
  fx
}

// ---------------------------------------------------------------------------
// The whole-estate direction.

#[test]
fn sync_to_disk_moves_only_what_changed() {
  let fx = seeded();
  // A re-projection of unchanged canon: nothing SHOULD change, and the
  // corollary test below covers the same shape deliberately.
  assert_moves_only_what_changed(&fx, "sync --to-disk", false, |fx| {
    fx.facade().sync_to_disk().expect("sync to disk");
  });
}

#[test]
fn sync_from_disk_moves_only_what_changed() {
  let fx = seeded();
  assert_moves_only_what_changed(&fx, "sync --to-store", false, |fx| {
    fx.facade().sync_from_disk().expect("sync from store");
  });
}

#[test]
fn todo_update_moves_only_what_changed() {
  let fx = seeded();
  assert_moves_only_what_changed(&fx, "todo update", false, |fx| {
    fx.facade().todo_update().expect("todo update");
  });
}

// ---------------------------------------------------------------------------
// The `apply` family -- 27 user-visible verbs through nine sites. These are
// the ones "run twice writes zero" could never have graded: each SHOULD change
// two or three files, and the defect is the other 260-odd it also rewrites.

/// **`st hold`, NOT `st start`.** The fixture threads are seeded `Wip`, so
/// `st start` is REFUSED and writes nothing -- which is how this test passed
/// vacuously on its first run. `hold` is a real transition out of `Wip`, and
/// the positive control now proves it happened.
#[test]
fn a_status_transition_moves_only_what_changed() {
  let fx = seeded();
  assert_moves_only_what_changed(&fx, "st hold", true, |fx| {
    fx.facade()
      .st_hold("ST0001", "parked behind the churn fix")
      .expect("st hold");
  });
}

#[test]
fn wp_new_moves_only_what_changed() {
  let fx = seeded();
  assert_moves_only_what_changed(&fx, "wp new", true, |fx| {
    fx.facade()
      .wp_new("ST0002", "a work package", TShirt::S)
      .expect("wp new");
  });
}

#[test]
fn st_new_moves_only_what_changed() {
  let fx = seeded();
  assert_moves_only_what_changed(&fx, "st new", true, |fx| {
    fx.facade().st_new("a new thread").expect("st new");
  });
}

// ---------------------------------------------------------------------------

/// The corollary, kept because it is the cheapest form to read and because it
/// is what the estate was originally measured with -- 20 of 20 views moving on
/// a second sync. It is NOT the criterion: it grades only the no-op case.
#[test]
fn a_second_sync_writes_nothing_at_all() {
  let fx = seeded();
  assert_moves_only_what_changed(&fx, "sync --to-disk, twice", false, |fx| {
    fx.facade().sync_to_disk().expect("second sync");
  });

  let before = snapshot(fx.root());
  age_everything(&before);
  fx.facade().sync_to_disk().expect("third sync");
  let Verdict { moved, changed } = verdict(fx.root(), &before);
  assert!(
    moved.is_empty() && changed.is_empty(),
    "a no-op sync wrote {} file(s) and changed {}: {:?}",
    moved.len(),
    changed.len(),
    moved
  );
}
