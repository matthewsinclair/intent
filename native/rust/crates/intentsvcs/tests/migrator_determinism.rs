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

use common::{Fixture, ctx, facade_ctx, sample_thread};

/// **Dehydrate by removing the DIRECTORY and keeping canon**, which is exactly
/// the state `intent organize --apply` leaves a thread in and exactly the state
/// 54 of this repo's 57 threads are in. Nothing here edits `.intentfiles`: the
/// migrator never reads it, so declaring the absence would test a path this
/// defect does not travel.
fn dehydrate(project: &intentsvcs::project::Project, id: &str) {
  let dir = project.thread_dir(id);
  std::fs::remove_dir_all(&dir).unwrap_or_else(|e| panic!("could not dehydrate {id}: {e}"));
  assert!(
    !dir.exists(),
    "the fixture must actually remove {id}'s directory or the arm below proves nothing"
  );
  assert!(
    project.thread_json(id).exists(),
    "dehydration keeps CANON -- a fixture that removed both would be testing deletion, not sparseness"
  );
}

fn index(project: &intentsvcs::project::Project) -> String {
  let path = project.steel_threads_view();
  std::fs::read_to_string(&path)
    .unwrap_or_else(|e| panic!("could not read {}: {e}", path.display()))
}

/// **PARKED, LOUDLY, AND THE PARK IS THE POINT: THIS ARM IS RED AND THE DEFECT
/// IS REAL.** `#[ignore]` rather than a relaxed assertion, on vc's ruling that
/// relaxing a gate when it stops covering anything converts a refusal into a
/// silent pass -- so this says NOT RUN, in every test run, where a reader sees
/// it.
///
/// **IT IS NOT PARKED BECAUSE ANYONE DOUBTS IT.** It is parked because the fix
/// is SCOPE rather than wiring: it touches the migrator's enumeration and the
/// SSOT rebuild, and landing a red test into a four-node workspace makes every
/// peer's `cargo test` red on a decision that is not theirs.
///
/// **EXPIRY, NAMED: remove `#[ignore]` in the commit that fixes it.** Two shapes
/// were put to vc, and the second is narrow because the blast radius is
/// measured rather than assumed -- `legacy::scan` has exactly TWO callers,
/// `Facade::upgrade` and `Facade::ingest_from_md`, and the second writes
/// nothing. `sync --to-disk` builds from `store.load_canon()`. **So one entry
/// point is destructive and the fix can be scoped to it.**
#[test]
#[ignore = "RED and the defect is real: upgrade rebuilds store and views from a disk-only scan. Parked because the fix is scope, not wiring -- see the doc comment; remove in the commit that fixes it"]
fn upgrade_does_not_shrink_the_index_to_the_threads_that_happen_to_be_realised() {
  let fx = Fixture::new();
  let ids = ["ST0001", "ST0002", "ST0003"];
  for id in ids {
    fx.write_thread(&sample_thread(id));
  }
  let project = fx.project();
  let canon = intentsvcs::ingest::read(&project).expect("fixture canon reads");
  intentsvcs::views::write_all(&project, &canon, &ctx()).expect("write views");

  // **THE CONTROL COMES FIRST AND IT IS NOT CEREMONY.** If the index never
  // named all three, the assertion below would pass on an empty file and this
  // whole test would be a green over nothing.
  let before = index(&project);
  for id in ids {
    assert!(
      before.contains(id),
      "the index must name {id} BEFORE the migration, or nothing after it means anything:\n{before}"
    );
  }

  dehydrate(&project, "ST0002");
  dehydrate(&project, "ST0003");

  intentsvcs::facade::Facade::upgrade(&project, &facade_ctx()).expect("upgrade runs");

  let after = index(&project);
  let lost: Vec<&str> = ids
    .iter()
    .copied()
    .filter(|id| !after.contains(id))
    .collect();
  assert!(
    lost.is_empty(),
    "`upgrade` removed {} thread(s) from the committed index -- {lost:?} -- and reported success. \
     Their canon is intact, so nothing is lost; the INDEX of the estate now names a subset and \
     nothing said so.\n{after}",
    lost.len()
  );
}

/// **PAIRED, so the arm above cannot pass by the migrator writing nothing.**
/// A fully realised estate must survive `upgrade` with its index intact -- if
/// this went red too, the finding would be "upgrade destroys the index" rather
/// than "upgrade cannot see a dehydrated thread", and the remedy would differ.
#[test]
fn upgrade_leaves_a_fully_realised_index_intact() {
  let fx = Fixture::new();
  let ids = ["ST0001", "ST0002"];
  for id in ids {
    fx.write_thread(&sample_thread(id));
  }
  let project = fx.project();
  let canon = intentsvcs::ingest::read(&project).expect("fixture canon reads");
  intentsvcs::views::write_all(&project, &canon, &ctx()).expect("write views");

  intentsvcs::facade::Facade::upgrade(&project, &facade_ctx()).expect("upgrade runs");

  let after = index(&project);
  for id in ids {
    assert!(
      after.contains(id),
      "a fully realised estate lost {id} from its index, so the sparse arm above is measuring \
       something else:\n{after}"
    );
  }
}
