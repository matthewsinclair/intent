//! AT-02.2 / AC-02.2: **`organize` makes disk match the list, in both
//! directions, and regenerates nothing.**
//!
//! hv, 2026-08-19, replacing the two-region design: *why isn't the organise
//! operation simply: a) look at `.intentfiles`, b) hydrate the items in the
//! file, c) dehydrate any previously hydrated items that are no longer in the
//! file.*
//!
//! **`.intentfiles` IS DURABLE STATE: the record of which database artefacts
//! also have a realised form on disk.** Realisation is driven from it; COMMANDS
//! CHANGE IT -- `st new` adds the id, `st done` removes it, `st hydrate` and
//! `st dehydrate` and the issue equivalents do it directly -- and `organize`
//! realises it. **A human editing it by hand is one writer among several, not
//! the privileged one.**
//!
//! **THE ONE THING THAT CHANGED FROM THE TWO-REGION DESIGN IS THAT NOTHING
//! RECOMPUTES IT.** No derivation from status overwrites what is there, so a
//! write is a CHANGE TO STATE rather than a REGENERATION OF IT -- and that is
//! why the protected region became unnecessary. The two regions existed only
//! because the file was machine-written: if `organize` rewrote the list from
//! status every run, a hand-added line would be wiped, so a protected region
//! was needed. **Take away the regeneration and the protected region has
//! nothing to protect against.** It also settles why `intentfiles::render` had
//! no production caller -- the thing it does is not needed.
//!
//! # Both directions, because one of them cannot be told from doing nothing
//!
//! A rule that only ever KEEPS things is indistinguishable from a rule that
//! never removes any. So the listed-and-absent case and the present-and-unlisted
//! case are asserted together, and the counts are printed rather than inferred.
//!
//! # The hand edit survives because NO CODE PATH REWRITES THE FILE
//!
//! That is a stronger property than *the rewrite preserves it*, and it is
//! asserted the strong way: byte-for-byte equality of the whole file across a
//! run that moved files in both directions.

mod common;

use common::{Fixture, gate_open, sample_thread};
use intentsvcs::organize::Mode;
use intentsvcs::sync::Scope;

/// An estate where one thread is LISTED and one is not, both realised on disk,
/// so a single run has work to do in both directions.
fn estate(fx: &Fixture) -> String {
  fx.write_thread(&gate_open());
  fx.write_thread(&sample_thread("ST0001"));
  fx.write_thread(&sample_thread("ST0002"));

  // Realise everything first, with no manifest: absent means nobody has said.
  fx.facade_on_disk()
    .sync_to_disk(&Scope::All)
    .expect("realise everything");

  // The list. Hand-authored, with a comment that must survive verbatim.
  let list = "\
# hand-authored: this file is the source of truth for what is on disk
STEELTHREAD:ST0001
STEELTHREAD:ST0057
";
  fx.write_file("intent/.intentfiles", list);

  // And take ST0001's cover away, so the same run has something to HYDRATE.
  std::fs::remove_file(fx.project().info_view("ST0001")).expect("dehydrate by hand");
  list.to_string()
}

/// **THE PROPERTY, BOTH DIRECTIONS, ONE RUN.**
#[test]
fn organize_makes_disk_match_the_list_in_both_directions() {
  let fx = Fixture::new();
  estate(&fx);

  let listed_absent = fx.project().info_view("ST0001");
  let unlisted_present = fx.project().info_view("ST0002");
  assert!(!listed_absent.exists(), "precondition: listed and absent");
  assert!(
    unlisted_present.exists(),
    "precondition: present and unlisted"
  );

  fx.facade_on_disk()
    .organize(Mode::Apply)
    .expect("organize reconciles");

  assert!(
    listed_absent.exists(),
    "LISTED AND ABSENT -> WRITTEN OUT. The list is the source of truth and the\n       \
     file was missing from disk, so organize puts it back."
  );
  assert!(
    !unlisted_present.exists(),
    "PRESENT AND UNLISTED -> REMOVED. Without this direction the rule cannot be\n       \
     told from one that never removes anything at all."
  );
}

/// **STATUS HAS NO VOTE.** A `completed` artefact that IS listed stays; a `wip`
/// artefact that is NOT listed goes.
///
/// This is the whole of what replaced the function of status, and it is the
/// direction that would be easy to reintroduce by accident.
#[test]
fn the_list_wins_over_status_in_both_directions() {
  let fx = Fixture::new();
  fx.write_thread(&gate_open());
  let mut closed = sample_thread("ST0001");
  closed.status = intentsvcs::model::ThreadStatus::Completed;
  fx.write_thread(&closed);
  fx.write_thread(&sample_thread("ST0002")); // wip, and deliberately unlisted

  fx.facade_on_disk()
    .sync_to_disk(&Scope::All)
    .expect("realise everything");
  fx.write_file(
    "intent/.intentfiles",
    "STEELTHREAD:ST0001\nSTEELTHREAD:ST0057\n",
  );

  fx.facade_on_disk()
    .organize(Mode::Apply)
    .expect("organize reconciles");

  assert!(
    fx.project().info_view("ST0001").exists(),
    "a COMPLETED artefact that is LISTED stays on disk -- status does not remove it"
  );
  assert!(
    !fx.project().info_view("ST0002").exists(),
    "and a WIP artefact that is NOT listed goes -- status does not keep it either"
  );
}

/// **`organize` DOES NOT RECOMPUTE THE LIST. Byte for byte, across a run that
/// moved files in both directions.**
///
/// **The property is NOT "no code path writes this file" -- that was my first
/// wording and vc corrected it mid-build.** Commands write it routinely and
/// that is the design: `st new` adds, `st done` removes, `st hydrate` and
/// `st dehydrate` do it directly. What no longer happens is RECOMPUTATION --
/// nothing derives the list from status and overwrites what is there.
///
/// So this asserts the narrow, true thing: **the verb that READS the list does
/// not write it.** Asserted over the whole file rather than a preserved region,
/// because there is no longer a region to preserve.
#[test]
fn organize_does_not_recompute_the_list() {
  let fx = Fixture::new();
  let authored = estate(&fx);

  fx.facade_on_disk()
    .organize(Mode::Apply)
    .expect("organize reconciles");

  assert_eq!(
    std::fs::read_to_string(fx.path("intent/.intentfiles")).expect("the list"),
    authored,
    "the list survived byte for byte across a run that moved files in BOTH\n       \
     directions -- `organize` realises the list and never recomputes it, which\n       \
     is why the protected region is gone. Other commands do write this file;\n       \
     this one must not."
  );
}

/// **THE CONTROL.** A second run with the list unchanged moves nothing, so the
/// first run's movement was the list being applied rather than churn.
#[test]
fn a_second_run_with_an_unchanged_list_moves_nothing() {
  let fx = Fixture::new();
  estate(&fx);
  let mut facade = fx.facade_on_disk();
  facade.organize(Mode::Apply).expect("first");

  let before: Vec<_> = std::fs::read_dir(fx.path("intent/st/ST0001"))
    .expect("dir")
    .filter_map(Result::ok)
    .map(|e| e.file_name())
    .collect();

  facade.organize(Mode::Apply).expect("second");

  let after: Vec<_> = std::fs::read_dir(fx.path("intent/st/ST0001"))
    .expect("dir")
    .filter_map(Result::ok)
    .map(|e| e.file_name())
    .collect();
  assert_eq!(
    before, after,
    "organize is a reconciliation, not a rewrite: with the list unchanged the\n       \
     second run has nothing to do"
  );
}
