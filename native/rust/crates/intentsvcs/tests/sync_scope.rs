//! ST0056 WP-03: **`intent sync` takes a SCOPE, so a node can land its own
//! thread without ingesting whatever three peers happen to be holding.**
//!
//! # The defect, measured rather than anticipated
//!
//! `--to-store` and `--to-disk` were whole-estate only. On a four-node board
//! every node needs an estate-wide write to land its own work, so **the
//! routine act of saving your thread reads every other node's uncommitted
//! files and takes them into the store.** vc measured it happening twice in
//! one day -- both times while holding the pen and warning the others -- and
//! ran thirteen estate-wide `--to-store` writes in a single session, each one
//! carrying whatever was on disk at that instant.
//!
//! dc's framing is the one that makes it structural rather than annoying: **a
//! workflow whose correct form requires an operation only safe for one actor
//! is a single-writer bottleneck wearing a per-node procedure's clothes.** And
//! vc's half is the reason care cannot fix it: staying off the estate protects
//! a node from WRITING, and the hazard here is a READ THAT WRITES.
//!
//! # Scope filters WHICH THREADS TAKE THEIR VALUE FROM DISK
//!
//! It is deliberately not "read less". `sync_from_disk` ends in
//! `store.rebuild(...)`, which replaces the whole store -- so narrowing the
//! READ and calling rebuild would DELETE every thread not named, which is a
//! far worse defect than the one being fixed.
//!
//! So a scoped restore composes: the named threads take their value from
//! canon, every other thread keeps the value the store already holds, and
//! `rebuild` runs unchanged over the union. **One write path, no second store
//! surface**, and "unscoped threads are unchanged" becomes a claim the tests
//! below actually check rather than a property of the plumbing nobody looks
//! at.
//!
//! # The unscoped arm is the load-bearing one
//!
//! A scoped restore that landed NOTHING would satisfy "only ST0056 changed"
//! completely. So the discriminating pair is the same fixture run both ways:
//! unscoped must move BOTH threads, scoped must move exactly one. Neither arm
//! means anything without the other -- this is the estate's own rule that a
//! zero is not a result until the check has produced a non-zero, applied to a
//! feature rather than to a guard.

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::sync::Scope;

const MINE: &str = "ST0056";
const PEERS: &str = "ST0057";

/// Two threads, on disk and in the store, agreeing.
fn two_threads(fx: &Fixture) -> intentsvcs::facade::Facade {
  fx.write_thread(&sample_thread(MINE));
  fx.write_thread(&sample_thread(PEERS));
  let mut facade = fx.facade();
  facade
    .sync_from_disk(&Scope::All)
    .expect("seed the store from both threads");
  facade
}

/// Change a thread ON DISK ONLY, the way an editor or a peer's in-flight work
/// does. The title is the observable because it is carried verbatim in both
/// directions and needs no state machine to be legal.
fn retitle_on_disk(fx: &Fixture, id: &str, title: &str) {
  let mut thread = sample_thread(id);
  thread.title = title.to_string();
  fx.write_thread(&thread);
}

fn title_in_store(facade: &mut intentsvcs::facade::Facade, id: &str) -> String {
  facade
    .st_show(id)
    .expect("thread is in the store")
    .title
    .clone()
}

#[test]
fn an_unscoped_restore_still_lands_every_thread_on_disk() {
  let fx = Fixture::new();
  let mut facade = two_threads(&fx);

  retitle_on_disk(&fx, MINE, "my work");
  retitle_on_disk(&fx, PEERS, "a peer's uncommitted holding");

  let landed = facade
    .sync_from_disk(&Scope::All)
    .expect("an unscoped restore");

  assert_eq!(landed, 2, "an unscoped restore reports both threads");
  assert_eq!(
    title_in_store(&mut facade, MINE),
    "my work",
    "precondition for the scoped arm: an unscoped restore DOES land a disk change"
  );
  assert_eq!(
    title_in_store(&mut facade, PEERS),
    "a peer's uncommitted holding",
    "THE DEFECT, asserted rather than guarded against: the whole-estate restore takes a peer's \
     uncommitted file into the store, and the node running it asked only to save their own work. \
     This arm is what makes the scoped one below mean something -- without it, a scoped restore \
     that did nothing at all would pass"
  );
}

#[test]
fn a_scoped_restore_lands_only_the_named_thread() {
  let fx = Fixture::new();
  let mut facade = two_threads(&fx);

  retitle_on_disk(&fx, MINE, "my work");
  retitle_on_disk(&fx, PEERS, "a peer's uncommitted holding");

  let landed = facade
    .sync_from_disk(&Scope::Threads(vec![MINE.to_string()]))
    .expect("a scoped restore");

  assert_eq!(landed, 1, "a scoped restore reports only what it landed");
  assert_eq!(
    title_in_store(&mut facade, MINE),
    "my work",
    "the named thread took its value from disk, which is the whole point of running it"
  );
  assert_eq!(
    title_in_store(&mut facade, PEERS),
    sample_thread(PEERS).title,
    "the UNNAMED thread kept the store's value. Its file on disk says otherwise and was not read \
     -- that is the difference between landing your own work and ingesting the board"
  );
}

#[test]
fn a_scoped_restore_does_not_delete_the_threads_it_did_not_name() {
  let fx = Fixture::new();
  let mut facade = two_threads(&fx);

  facade
    .sync_from_disk(&Scope::Threads(vec![MINE.to_string()]))
    .expect("a scoped restore");

  assert!(
    facade.st_show(PEERS).is_ok(),
    "the unnamed thread is GONE from the store. Scope narrowed the read and the rebuild then \
     replaced the whole store with the narrowed set -- which is a worse defect than the one \
     scope exists to fix, and it is silent: the node that ran it was saving its own work"
  );
}

#[test]
fn a_scope_naming_no_such_thread_refuses_rather_than_succeeding_over_nothing() {
  let fx = Fixture::new();
  let mut facade = two_threads(&fx);

  let result = facade.sync_from_disk(&Scope::Threads(vec!["ST9999".to_string()]));

  assert!(
    result.is_err(),
    "a scope naming a thread that does not exist reported success over an empty selection. A \
     typo in an id would then be indistinguishable from a completed sync, and the operator would \
     believe their work had landed"
  );
}

#[test]
fn a_scoped_projection_writes_only_the_named_thread() {
  let fx = Fixture::new();
  let mut facade = two_threads(&fx);

  let wrote = facade
    .sync_to_disk(&Scope::Threads(vec![MINE.to_string()]))
    .expect("a scoped projection");

  assert_eq!(
    wrote, 1,
    "a scoped projection reports only the threads it wrote"
  );

  let all = facade
    .sync_to_disk(&Scope::All)
    .expect("an unscoped projection");
  assert_eq!(
    all, 2,
    "precondition: the unscoped projection covers both threads, so the 1 above is a NARROWING \
     rather than the only number this can return"
  );
}

/// The warning `--to-store` prints before it runs must narrow with the scope.
///
/// **A warning that over-reports is not merely noisy: it names files this run
/// will not touch, so the operator either stops for a loss that is not coming
/// or learns to skip the list.** Both outcomes end with the warning unread,
/// which is the state it exists to prevent.
#[test]
fn the_overwrite_warning_narrows_with_the_scope() {
  let fx = Fixture::new();
  let facade = two_threads(&fx);

  // **Divergence is made on DISK, not in the store, and the first version of
  // this arm got that backwards.** It mutated through the facade -- which
  // projects as it writes -- so store and disk agreed and the warning
  // correctly had nothing to say. The precondition below caught it rather
  // than the assertion passing over two empty lists, which is the whole
  // reason it is written as a precondition.
  retitle_on_disk(&fx, MINE, "my work");
  retitle_on_disk(&fx, PEERS, "a peer's uncommitted holding");

  let wide = facade
    .sync_overwrite(&Scope::All)
    .expect("unscoped warning");
  let narrow = facade
    .sync_overwrite(&Scope::Threads(vec![MINE.to_string()]))
    .expect("scoped warning");

  assert!(
    !wide.is_empty(),
    "precondition: the unscoped warning has something to say, or the comparison below is between \
     two empty lists and proves nothing"
  );
  assert!(
    narrow.len() < wide.len(),
    "the scoped warning did not narrow: it reports {} line(s) against the unscoped {}. It is \
     naming files this run will not touch",
    narrow.len(),
    wide.len()
  );
  assert!(
    narrow.iter().all(|line| !line.contains(PEERS)),
    "the scoped warning still names the thread the scope excludes: {narrow:?}"
  );
}
