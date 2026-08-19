//! AT-03.16 / AC-03.15: **a verb that would reduce a population to zero must
//! refuse or name it; succeeding at rc=0 is not available.**
//!
//! The live instance: `sync --to-disk` wrote empty views over a non-empty
//! estate at rc=0 -- `steel_threads.md` 57 rows -> 0, `todo.md` 82 -> 0.
//! **Nothing was refused and nothing malfunctioned in the egest.** The store
//! legitimately held zero threads, because a shared `target/release/` binary
//! built from a reverted WP-01 tree had ingested zero and `sync --to-store` had
//! reported success over it. The egest wrote exactly what it was given,
//! correctly, by its own lights.
//!
//! # Why this is not AC-03.13
//!
//! That criterion requires an ingest to have been REFUSED. Here nothing was
//! refused, which is precisely why the sibling guard cannot reach this and why
//! the two are separate criteria rather than one. Both cases are driven below,
//! and the second is what stops this row being satisfied by the other's
//! machinery.
//!
//! # Why the positive control actually works, which most of the day's did not
//!
//! 57 and 82 are non-zero, observable before the verb runs, and cannot be
//! confused with a correct answer. A zero cannot do any of that -- it is
//! indistinguishable from a legitimately empty population, which is the whole
//! reason the same class one verb upstream (`sync --to-store` printing "the
//! store and the extract agree" over `0 == 0`) went unnoticed for so long.
//!
//! # Driven with the output read
//!
//! The original loss was silent ONLY because the write verb was piped to
//! `/dev/null`. Every case here reads what the verb returned and asserts on the
//! rendered message and its remedy -- a test that reproduced the loss under
//! suppression would be measuring the suppression.

mod common;

use common::{Fixture, sample_issue, sample_thread};
use intentsvcs::remedy::Remedy;
use intentsvcs::store::Store;
use intentsvcs::sync::Scope;

/// A populated estate whose STORE holds issues and no threads.
///
/// **This is how the state is reached without a broken binary, and it is not a
/// contrivance: it is a partial ingest.** A store that holds nothing at all is
/// COLD, and `ingest::load_fresh` warms a cold store from the files -- so an
/// entirely empty store repopulates itself on the next open and can never
/// reach the egest. One non-empty table is what makes the store WARM and wrong
/// at the same time, which is exactly the shape the live instance had.
fn estate_with_a_store_that_lost_its_threads(fx: &Fixture) {
  fx.write_thread(&sample_thread("ST0056"));
  fx.write_thread(&sample_thread("ST0057"));
  fx.write_issue(&sample_issue(21));

  // One good egest first, so the estate has real views to be emptied.
  fx.facade_on_disk()
    .sync_to_disk(&Scope::All)
    .expect("a healthy estate projects");

  let mut store = Store::open(&fx.project().db_path()).expect("open");
  let (_, issues) = store.load_canon().expect("read");
  store.rebuild(&[], &issues).expect("the threads are gone");
}

/// **THE PROPERTY.** rc=0 is not available.
#[test]
fn an_egest_from_a_store_that_holds_no_threads_refuses() {
  let fx = Fixture::new();
  estate_with_a_store_that_lost_its_threads(&fx);

  let err = fx.facade_on_disk().sync_to_disk(&Scope::All).expect_err(
    "the store holds no threads and the estate has two -- writing that out is\n       \
     the data-loss path presenting as a routine sync, and it presented as one",
  );

  let said = err.to_string();
  // **`contains('2')` WAS THE FIRST VERSION OF THIS AND IT WAS WORTHLESS.** Arm
  // two's message is full of byte counts, so a digit proves nothing about WHICH
  // arm fired -- and the two arms are the difference between "your estate has
  // two threads" and "a file would shrink", which are different diagnoses.
  assert!(
    said.contains("the estate has 2"),
    "the refusal states the population it is protecting, by ARM ONE, because a\n       \
     bare `refused` leaves the operator to guess whether the tool or the estate\n       \
     is wrong: {said}"
  );
  assert!(
    err.remedy().contains("still on disk"),
    "and it says WHERE the work is -- the store is empty, the project is not: {}",
    err.remedy()
  );
}

/// **THE HARM.** The views survive.
#[test]
fn the_populated_views_are_still_populated_afterwards() {
  let fx = Fixture::new();
  estate_with_a_store_that_lost_its_threads(&fx);
  // Resolved rather than spelled -- the path is `Project`'s to know, and a
  // second spelling here is the Highlander problem one level below the one
  // AC-01.6 fixes.
  let face = fx.project().steel_threads_view();
  let before = std::fs::read_to_string(&face).expect("the face exists");
  assert!(
    before.contains("ST0056") && before.contains("ST0057"),
    "precondition: the face carries both threads"
  );

  let _ = fx.facade_on_disk().sync_to_disk(&Scope::All);

  assert_eq!(
    std::fs::read_to_string(&face).expect("the face exists"),
    before,
    "THE LOSS. The face carried two threads, the store carried none, and the\n       \
     egest wrote the store over the face at rc=0. Nothing was refused and\n       \
     nothing malfunctioned, which is what made it silent."
  );
}

/// **THE CONTROL, and it is the one that matters most here.**
///
/// Without it every case above passes on a `sync --to-disk` that refuses
/// whenever a population is zero -- which would refuse the first egest of every
/// freshly initialised project, where zero is the correct and only answer.
#[test]
fn a_genuinely_empty_estate_egests_without_complaint() {
  let fx = Fixture::new();
  let mut facade = fx.facade_on_disk();
  assert_eq!(
    facade
      .sync_to_disk(&Scope::All)
      .expect("an empty project has nothing to lose and must not be refused"),
    0
  );
  // And twice, because the second run has views on disk to compare against.
  assert_eq!(facade.sync_to_disk(&Scope::All).expect("still fine"), 0);
}

/// A healthy estate is not refused either -- the ordinary path stays ordinary.
#[test]
fn a_populated_store_projects_normally() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  fx.write_issue(&sample_issue(21));
  assert_eq!(
    fx.facade_on_disk()
      .sync_to_disk(&Scope::All)
      .expect("nothing about this estate is empty"),
    1
  );
}

/// **THE DISCRIMINATING CONTROL FOR ARM TWO, AND ITS FIRST VERSION PASSED FOR
/// THE WRONG REASON.**
///
/// A healthy estate whose content genuinely SHRINKS -- an objective edited
/// down, a note removed -- must still egest. The first version of the guard ran
/// the byte comparison whenever EITHER population was zero, and **most projects
/// have no issues at all**, so on those it ran on every egest and refused every
/// ordinary shrink. A guard that refuses the ordinary path is worse than the
/// hole it closes, because what happens next is that somebody turns it off.
///
/// **The first version of this test drove the edit through `sync --to-store`
/// and then measured `sync --to-disk`, and a mutation arm proved it vacuous.**
/// `sync_from_disk` projects at the end of its own run, so the views were
/// ALREADY SHORT before the verb under test was called: nothing shrank, arm two
/// had nothing to look at, and removing the gate it exists to justify changed
/// no outcome. **The arm survived, which is the only reason this was found** --
/// the test was green throughout and reads exactly like a control.
///
/// So the store is moved directly instead. That is the only way to hold the
/// store short while the disk is still long, which is precisely the state a
/// shrinking egest is defined by.
#[test]
fn a_healthy_estate_whose_content_shrinks_still_egests() {
  let fx = Fixture::new();
  let mut thread = sample_thread("ST0056");
  thread.objective = "a long objective, written out at length so that removing it is unmistakably a shrink rather than a rounding difference in the rendered view".to_string();
  fx.write_thread(&thread);
  // NO ISSUES, which is the ordinary shape and was the false positive's home.
  fx.facade_on_disk()
    .sync_to_disk(&Scope::All)
    .expect("the first egest");
  let face = fx.project().info_view("ST0056");
  let long = std::fs::read_to_string(&face).expect("the view exists");

  // The store shortens; the disk does not hear about it yet.
  {
    let mut store = Store::open(&fx.project().db_path()).expect("open");
    let (mut threads, issues) = store.load_canon().expect("read");
    threads[0].objective = "short".to_string();
    store.rebuild(&threads, &issues).expect("the store shrinks");
  }

  assert_eq!(
    fx.facade_on_disk()
      .sync_to_disk(&Scope::All)
      .expect("an estate that shrank legitimately must still project"),
    1
  );

  let now = std::fs::read_to_string(&face).expect("the view exists");
  assert!(
    now.len() < long.len(),
    "the control is vacuous unless the write ACTUALLY SHRANK the file: {} -> {}",
    long.len(),
    now.len()
  );
  assert!(
    now.contains("short"),
    "and the shorter content is what landed"
  );
}

/// **THE ARM THAT CATCHES THE LIVE INSTANCE, AND THE OBVIOUS CHECK DOES NOT.**
///
/// The binary that caused the episode was built from a reverted WP-01 tree, so
/// its canon resolver pointed at the OLD location: it read zero from disk for
/// the same reason it had ingested zero. **Canon zero, store zero, no refusal.**
/// A guard comparing the store against the canon files it can see is reading
/// its subject through the very assumption that is broken, so it agrees with
/// the store and says nothing.
///
/// **What survives is the FACE.** `steel_threads.md` did not move in WP-01, so
/// it still carries what the estate holds, and that is evidence a wrong
/// resolver cannot have misread. The check is a comparison of the bytes about
/// to be written against the bytes already there.
///
/// # The substitution, stated rather than hidden
///
/// The state needed is *both enumerations reading zero while the face says
/// otherwise*. It is reached here by removing the thread canon AND the thread
/// directories, leaving the estate view standing -- which reproduces the
/// OBSERVABLE the guard consumes, not the cause. **A stale resolver cannot see
/// any of the thread files either; the difference is that it is looking
/// elsewhere rather than at an empty place, and no code here can tell those
/// apart.**
///
/// # Two failed substitutions, recorded because each found something
///
/// **Removing only `intent/.canon/st` does not work**: with canon gone and the
/// rendered `info.md` files still present, `Project` reads the estate as an
/// UNMIGRATED v2 project and `Facade::open` refuses first
/// (`Unmigrated(Pending { legacy_threads: [...] })`). A second, independent
/// guard over the same evidence happens to cover that shape -- which is worth
/// knowing, and is why the thread directories go too.
///
/// **Doing it on ISSUES does not work either, and the reason is a hole rather
/// than a quirk**: zero issues in the store produces no shrink because ISSUES
/// HAVE NO INDEX VIEW. Measured -- the write set for that state is seven paths,
/// every one of them byte-identical, and not one of them about issues. So an
/// egest that drops an issue writes nothing at all and leaves the stale canon
/// file standing. **That is not this criterion's subject and it is not
/// asserted here**, but it is the shape of a population with no face, which is
/// a population this row's check cannot protect.
#[test]
fn a_face_that_would_shrink_is_refused_when_both_enumerations_read_zero() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  fx.write_issue(&sample_issue(21));
  fx.facade_on_disk()
    .sync_to_disk(&Scope::All)
    .expect("a healthy estate projects");

  let face = fx.project().steel_threads_view();
  let populated = std::fs::read_to_string(&face).expect("the face exists");
  assert!(populated.contains("ST0056"), "precondition: the face carries it");

  // The store loses its threads and keeps its issues -- one non-empty table is
  // what makes it WARM and wrong, rather than cold and self-healing.
  {
    let mut store = Store::open(&fx.project().db_path()).expect("open");
    let (_, issues) = store.load_canon().expect("read");
    store.rebuild(&[], &issues).expect("the threads are gone");
  }
  // And canon loses them too, so ARM ONE has nothing to fire on.
  std::fs::remove_dir_all(fx.path("intent/.canon/st")).expect("canon goes");
  std::fs::remove_dir_all(fx.path("intent/st/ST0056")).expect("and the rendered directory with it");
  assert!(
    fx.project().thread_ids().expect("enumerate").is_empty(),
    "precondition: ARM ONE HAS NOTHING TO FIRE ON -- canon agrees with the store"
  );
  assert!(
    std::fs::read_to_string(&face)
      .expect("the face is still there")
      .contains("ST0056"),
    "precondition: and the FACE is the only thing left that disagrees"
  );

  let err = fx
    .facade_on_disk()
    .sync_to_disk(&Scope::All)
    .expect_err("a write that shrinks a face while the population is zero is refused");
  let said = err.to_string();
  assert!(
    said.contains("bytes"),
    "the refusal names the file and both sizes, so the operator sees the size of\n       \
     what was about to be lost rather than being told that a rule fired: {said}"
  );
  assert_eq!(
    std::fs::read_to_string(&face).expect("still there"),
    populated,
    "and the face is untouched -- a refusal that had already written is a receipt"
  );
}
