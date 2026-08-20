//! **AT-10.12: the migrator must not silently shrink the estate it reports on.**
//!
//! AC-10.12 is worded *a verb that reports an estate UNCHANGED must not have
//! changed it*, and its recorded instance -- `intent upgrade` rewriting 40
//! issue bodies while printing *their content is unchanged* -- **is no longer
//! reproducible**: issues were pruned from the disk estate, so `upgrade`
//! reports `0 issue(s)` and there is nothing to rewrite. The row's EVIDENCE
//! expired while its property did not, which is a shape this thread keeps
//! meeting from the other side (a recorded BLOCKER expiring).
//!
//! What is live is worse, and it is what this file drives.
//!
//! # The defect, measured before it was written down
//!
//! On a fresh clone at `183f7342`, binary `ada1ce8c0241e3a9`:
//!
//! ```text
//! fresh clone                steel_threads.md  57 rows
//! intent sync --to-store     "ok: store replaced from the extract, 57 thread(s)"
//! intent upgrade             rc=0, "ok: this project is now Intent v3.0.0-dev"
//! after                      steel_threads.md   3 rows
//! ```
//!
//! **239 lines deleted across two committed views, at rc=0.** Canon is
//! untouched -- all 57 files survive -- so nothing is LOST; the committed
//! INDEX of the estate now names three threads of fifty-seven, and the only
//! thing that said so was a diff nobody was asked to read.
//!
//! **It is not a missing-store problem**, which was the first hypothesis and
//! was checked: loading the store from the extract first (57 threads, "the
//! store and the extract agree") changes nothing.
//!
//! # Why, read at source
//!
//! `legacy::scan` iterates `thread_dirs(project)` -- the DISK directories
//! under `intent/st/`. A dehydrated thread has canon and no directory, so it
//! is invisible to the enumeration, and `views::write_all` then regenerates
//! the project-level index from a model missing it.
//!
//! **THE MIGRATOR IS DISK-DRIVEN IN AN ESTATE WHERE DISK IS A SPARSE
//! PROJECTION.** Every comment in that function is about v2 status buckets and
//! duplicate ids -- the population it was written against was one where every
//! thread had a directory. **Nothing about it is wrong for the world it was
//! written for**, which is why no test caught it: the world changed under it
//! and the enumeration is still correct-looking.
//!
//! # The fixture is synthetic and that is the ruling, not a convenience
//!
//! vc, 2026-08-20: neither the instance, nor the control, nor the predicate may
//! be drawn from the thing under test. A red-first keyed on the live estate's
//! 54 dehydrated threads would make them a fixture, and the estate could not
//! then be repaired without reddening this file.

mod common;

use std::collections::BTreeSet;

use common::{Fixture, facade_ctx, gate_open, sample_thread};
use intentsvcs::model::ThreadStatus;
use intentsvcs::organize::Mode;
use intentsvcs::project::Project;
use intentsvcs::sync::Scope;

/// **THE ESTATE IS DEHYDRATED THROUGH `organize`, NOT BY REMOVING DIRECTORIES**
/// (vc, 2026-08-20, and the recipe is `write_path_canon_always.rs`'s). My first
/// cut called `remove_dir_all` -- which reaches the same on-disk shape and
/// **side-steps the gate that is supposed to produce it**, so it would have
/// passed against a build where `organize` refused to dehydrate anything at
/// all. A fixture that side-steps the gate measures the side-step.
///
/// Returns every id canon holds, which is the population both assertions below
/// compare against.
fn dehydrated_estate(fx: &Fixture) -> Vec<String> {
  fx.write_thread(&gate_open());
  for id in ["ST0001", "ST0002"] {
    let mut closed = sample_thread(id);
    closed.status = ThreadStatus::Completed;
    fx.write_thread(&closed);
  }

  let mut f = fx.facade_on_disk();
  f.sync_to_disk(&Scope::All)
    .expect("realise everything first");
  for id in ["ST0001", "ST0002"] {
    assert!(
      fx.project().info_view(id).exists(),
      "precondition: {id} is on disk before organize is asked to remove it"
    );
  }

  // Declaring nothing keeps nothing -- ABSENT IS NOT EMPTY, so the file must
  // exist and be empty rather than be missing.
  fx.write_file("intent/.intentfiles", "# BEGIN INTENT\n# END INTENT\n");
  let mut f = fx.facade_on_disk();
  f.organize(Mode::Apply)
    .expect("the gate is open, so the removals happen");
  for id in ["ST0001", "ST0002"] {
    assert!(
      !fx.project().info_view(id).exists(),
      "precondition: {id} is DEHYDRATED -- if it is still here the gate refused and \
       every assertion below would pass for the wrong reason"
    );
  }

  fx.project().thread_ids().expect("canon ids")
}

/// The thread ids the committed index NAMES, read back out of the rendered
/// table rather than counted.
fn index_ids(project: &Project) -> BTreeSet<String> {
  let text = std::fs::read_to_string(project.steel_threads_view()).expect("index reads");
  text
    .lines()
    .filter_map(|l| l.strip_prefix("| ST"))
    .filter_map(|rest| rest.split('|').next())
    .map(|id| format!("ST{}", id.trim()))
    .collect()
}

/// The thread ids the STORE holds.
fn store_ids(project: &Project) -> BTreeSet<String> {
  let store = intentsvcs::store::Store::open(&project.db_path()).expect("store opens");
  let (threads, _issues) = store.load_canon().expect("store loads");
  threads.into_iter().map(|t| t.id).collect()
}

/// **EQUALITY OF TWO POPULATIONS, NOT A ROW COUNT** (vc's specification, and
/// both halves of it earn their place).
///
/// A COUNT-based assertion passes on a migrator that names 57 of the WRONG
/// threads. A VIEWS-only assertion passes on a fix that repairs the rendering
/// and still rebuilds the SSOT from three -- **and that half is the one no
/// `git status` would ever show**, because the store is gitignored.
#[test]
fn upgrade_covers_every_thread_canon_holds_in_both_the_index_and_the_store() {
  let fx = Fixture::new();
  let canon_ids: BTreeSet<String> = dehydrated_estate(&fx).into_iter().collect();
  let project = fx.project();

  assert!(
    canon_ids.len() > 1,
    "the fixture must leave more than one thread in canon or the equalities below are trivial"
  );

  intentsvcs::facade::Facade::upgrade(&project, &facade_ctx()).expect("upgrade runs");

  assert_eq!(
    index_ids(&project),
    canon_ids,
    "the committed index and canon name different populations after `upgrade`"
  );
  assert_eq!(
    store_ids(&project),
    canon_ids,
    "the STORE and canon hold different populations after `upgrade` -- and the store is \
     gitignored, so nothing in a diff would have shown this"
  );
}

/// **THE THIRD ARM, WHICH vc's SPECIFICATION DID NOT ASK FOR AND MY FIRST FIX
/// NEEDED.** Covering the model is only half: the first cut of the union fed
/// the SAME full population to the per-thread view writer and HYDRATED 54
/// threads onto a disk whose manifest declared 3 -- 326 files written against
/// 38, and 54 untracked directories under an `ok:`.
///
/// **The two defects are opposite and a test for either passes on the other.**
#[test]
fn upgrade_realises_only_what_the_manifest_declares() {
  let fx = Fixture::new();
  dehydrated_estate(&fx);
  let project = fx.project();

  intentsvcs::facade::Facade::upgrade(&project, &facade_ctx()).expect("upgrade runs");

  let realised: Vec<String> = project
    .thread_ids()
    .expect("canon ids")
    .into_iter()
    .filter(|id| project.info_view(id).exists())
    .collect();
  assert!(
    realised.is_empty(),
    "the manifest declares nothing realised and `upgrade` put {realised:?} back on disk"
  );
}
