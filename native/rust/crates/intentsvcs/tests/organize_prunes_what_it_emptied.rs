//! **`organize` REMOVES THE DIRECTORIES IT EMPTIED, AND ONLY THOSE.**
//!
//! # Why this exists
//!
//! `organize --apply` removed 423 files from this project's estate and left 54
//! empty directory shells behind: the only removal it performed was
//! `remove_file`. vc ruled that harmless because git does not track an empty
//! directory. hv opened the tree and counted **58 directories where 3 threads
//! were declared**, and reversed it.
//!
//! **Both readings are correct about different estates.** Git's view and a
//! person's view of "what is in this project" disagree exactly on empty
//! directories, and the one a person opens is the one that matters. A `rmdir`
//! sweep cleared them once; the next dehydration would have recreated them,
//! which is what makes this a code change rather than a tidy-up.
//!
//! # The four things asserted, and why none of them is the obvious one alone
//!
//! Asserting only "the directory is gone" would pass for a verb that deletes
//! the whole estate. So the dangerous directions are asserted too: a directory
//! holding an unrelated file SURVIVES, the estate root itself survives, and the
//! prune cascades no further than the run's own removals reach.

mod common;

use common::{Fixture, gate_open, sample_thread};
use intentsvcs::organize::Mode;
use intentsvcs::sync::Scope;

/// Realise two threads, then declare neither, so a single run empties both.
fn dehydrating_estate(fx: &Fixture) {
  fx.write_thread(&gate_open());
  fx.write_thread(&sample_thread("ST0001"));
  fx.write_thread(&sample_thread("ST0002"));
  fx.facade_on_disk()
    .sync_to_disk(&Scope::All)
    .expect("realise everything");
  fx.write_file("intent/.intentfiles", "STEELTHREAD:ST0057\n");
}

/// **THE PROPERTY.** A thread whose every file this run removed leaves no
/// directory behind.
#[test]
fn a_thread_this_run_emptied_leaves_no_directory() {
  let fx = Fixture::new();
  dehydrating_estate(&fx);

  let dir = fx.project().thread_dir("ST0001");
  assert!(dir.is_dir(), "precondition: the thread is realised");

  let report = fx
    .facade_on_disk()
    .organize(Mode::Apply)
    .expect("organize reconciles");

  assert!(
    !dir.exists(),
    "the run removed every file under {} and must not leave the shell behind -- \n       \
     pruned: {:?}",
    dir.display(),
    report.pruned
  );
  assert!(
    report.pruned.iter().any(|p| p == &dir),
    "and the removal must be REPORTED: a destructive act nobody can review is \n       \
     the one line of this verb an operator cannot check. pruned: {:?}",
    report.pruned
  );
}

/// **THE CASCADE.** `WP/01` going empties `WP`, which must go too -- in one run,
/// without a second pass.
#[test]
fn an_ancestor_emptied_by_its_child_goes_in_the_same_run() {
  let fx = Fixture::new();
  fx.write_thread(&gate_open());
  let mut with_wp = sample_thread("ST0001");
  with_wp.wps = sample_thread("ST0001").wps;
  fx.write_thread(&with_wp);
  fx.facade_on_disk()
    .sync_to_disk(&Scope::All)
    .expect("realise everything");
  fx.write_file("intent/.intentfiles", "STEELTHREAD:ST0057\n");

  let wp_dir = fx.project().thread_dir("ST0001").join("WP");
  assert!(
    wp_dir.is_dir(),
    "precondition: the WP directory is realised"
  );

  fx.facade_on_disk()
    .organize(Mode::Apply)
    .expect("organize reconciles");

  assert!(
    !wp_dir.exists(),
    "`WP/` was emptied by the removal of its children and must go in the SAME \n       \
     run -- deepest-first ordering is what makes that true without a second pass"
  );
}

/// **THE DANGEROUS DIRECTION: A DIRECTORY HOLDING SOMETHING SURVIVES.**
///
/// Without this, every assertion above is satisfied by a verb that deletes the
/// estate recursively. The stranger file is one `organize` never planned to
/// remove, so `remove_dir`'s refusal is the whole safety argument -- and it is
/// the filesystem's refusal, not this code's.
#[test]
fn a_directory_still_holding_something_is_not_removed() {
  let fx = Fixture::new();
  dehydrating_estate(&fx);

  let dir = fx.project().thread_dir("ST0001");
  let stranger = dir.join("notes-nobody-declared.md");
  std::fs::write(&stranger, "authored by hand, declared by nobody").expect("write the stranger");

  fx.facade_on_disk()
    .organize(Mode::Apply)
    .expect("organize reconciles");

  assert!(
    stranger.exists(),
    "a file this run never planned to remove must survive -- and it is what \n       \
     makes the pruning safe at all"
  );
  assert!(
    dir.is_dir(),
    "so its directory must survive too. `remove_dir` refuses a non-empty \n       \
     directory; the recursive variant must never be substituted here"
  );
}

/// **THE FLOOR.** The estate root is never pruned, even when the run empties it
/// completely.
#[test]
fn the_estate_root_is_never_removed() {
  let fx = Fixture::new();
  dehydrating_estate(&fx);

  let report = fx
    .facade_on_disk()
    .organize(Mode::Apply)
    .expect("organize reconciles");

  let root = fx.project().st_dir();
  assert!(
    root.is_dir(),
    "the estate root survives a run that emptied everything under it -- a \n       \
     cascade without a declared floor walks to `/`"
  );
  assert!(
    !report
      .pruned
      .iter()
      .any(|p| p == &root || root.starts_with(p)),
    "and nothing at or above the root may even be attempted. pruned: {:?}",
    report.pruned
  );
}

/// **PREVIEW PRUNES NOTHING**, for the same reason it removes no files.
#[test]
fn a_preview_removes_no_directory() {
  let fx = Fixture::new();
  dehydrating_estate(&fx);
  let dir = fx.project().thread_dir("ST0001");

  let report = fx
    .facade_on_disk()
    .organize(Mode::Preview)
    .expect("organize previews");

  assert!(
    dir.is_dir(),
    "a preview decides everything and touches nothing, directories included"
  );
  assert!(
    report.pruned.is_empty(),
    "and it reports no prune it did not perform. pruned: {:?}",
    report.pruned
  );
}
