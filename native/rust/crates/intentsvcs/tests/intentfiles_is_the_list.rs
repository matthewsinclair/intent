//! AT-02.2 / AC-02.2: **`organize` makes disk match the list, in both
//! directions, and regenerates nothing.**
//!
//! AT-02.3 / AC-02.3: **the list wins over status, both directions** -- a
//! `completed` or `cancelled` artefact that IS listed stays through `organize`,
//! and a `wip` artefact that is NOT listed is removed by it. Proven here by
//! [`the_list_wins_over_status_in_both_directions`], and on the live estate at
//! `e7f00e65` across four statuses: 52 completed and 2 cancelled removed while
//! unlisted, 1 not-started and 2 wip kept while listed, plus ST0010
//! (`cancelled`) listed -> hydrated -> kept -> unlisted -> removed. **Status has
//! no vote at `organize` time at all.**
//!
//! hv, 2026-08-19, replacing the two-region design: *why isn't the organise
//! operation simply: a) look at `.intentfiles`, b) hydrate the items in the
//! file, c) dehydrate any previously hydrated items that are no longer in the
//! file.*
//!
//! **`.intentfiles` IS DURABLE STATE: the record of which database artefacts
//! also have a realised form on disk.** Realisation is driven from it; COMMANDS
//! CHANGE IT -- `st start` adds the id, `st done` removes it, `st hydrate` and
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

/// **THE RECONCILIATION AS AN EQUALITY OF TWO INDEPENDENTLY DERIVED SETS, WITH
/// BOTH COUNTS PRINTED** -- AC-02.2's actual wording, which the four tests
/// above do not satisfy.
///
/// **vc's objection, and it is right: a sample of two cannot be told from a
/// rule that handles exactly those two ids.** Every assertion above names
/// `ST0001` and `ST0002` and checks one path each. A `organize` that special-
/// cased two ids and ignored the rest would pass all four, and the live estate
/// is 545 removals.
///
/// So: ten threads, an unlisted majority, and the sets derived from two sources
/// that do not consult each other.
///
/// **NEITHER DERIVATION CALLS THE PRODUCTION CODE UNDER TEST.** The listed set
/// is parsed here rather than through `intentfiles::parse`, and the realised
/// set is read off the filesystem rather than through `organize`'s own plan.
/// Deriving either one from the code being tested would make the equality a
/// tautology -- the two sets have to be able to disagree for their agreement to
/// mean anything, which is why the pre-state is asserted to disagree first.
#[test]
fn the_realised_set_equals_the_listed_set() {
  use std::collections::BTreeSet;

  let fx = Fixture::new();
  fx.write_thread(&gate_open()); // ST0057
  for n in 1..=9 {
    fx.write_thread(&sample_thread(&format!("ST{n:04}")));
  }

  // Realise the whole estate: ten threads on disk, nothing said about any.
  fx.facade_on_disk()
    .sync_to_disk(&Scope::All)
    .expect("realise everything");

  // The list declares an odd, non-contiguous minority -- so a rule keyed on
  // "the first two" or "the low ids" is a different set from this one.
  //
  // **THE `ISSUE:` LINE WAS LOAD-BEARING AND THE GRAMMAR TOOK ITS SUBJECT
  // AWAY. THIS IS A REAL LOSS OF COVERAGE AND IT IS STATED, NOT ABSORBED.**
  //
  // It was added because a mutation arm SURVIVED without it: deleting the
  // sigil filter from the derivation below left this test green while the
  // fixture held steel threads only, because the filter had nothing to
  // exclude. An untested branch sat inside the very derivation whose
  // independence is the point.
  //
  // hv retired `ISSUE:` on 2026-08-20, so the grammar has ONE sigil and there
  // is no second one to put here. **A line with an unknown sigil cannot
  // substitute** -- `parse` refuses it before the derivation ever sees it, so
  // it exercises the parser rather than the filter. The mutation this comment
  // was written about is therefore UNREACHABLE from any fixture today, and
  // deleting the sigil filter would once again go unnoticed.
  //
  // **The coverage returns the day a second sigil lands** -- which is queued,
  // not hypothetical: cc's 59 project-content files want one. Whoever adds it
  // must put it in this fixture, and this comment is the reason why.
  fx.write_file(
    "intent/.intentfiles",
    "\
# hand-authored
STEELTHREAD:ST0002
STEELTHREAD:ST0003
STEELTHREAD:ST0005
STEELTHREAD:ST0007
STEELTHREAD:ST0057
",
  );

  // --- derivation one: what the LIST says, parsed here, not by the tool ------
  let listed: BTreeSet<String> = fx
    .read("intent/.intentfiles")
    .lines()
    .map(str::trim)
    .filter(|l| !l.is_empty() && !l.starts_with('#'))
    .filter_map(|l| l.split_once(':'))
    .filter(|(sigil, _)| *sigil == "STEELTHREAD")
    .map(|(_, id)| id.to_string())
    .collect();

  // --- derivation two: what is REALISED, read off the filesystem ------------
  // The estate directory is SCANNED -- the ids are discovered, not supplied --
  // and a thread counts as realised iff its cover view is present. Written as a
  // closure because it is evaluated twice: the disagreement before and the
  // agreement after are the same measurement at two times.
  //
  // **`is a non-empty directory` WAS THE FIRST PREDICATE AND IT IS WRONG.**
  // Measured, not reasoned: after a run that dehydrated 20 files, every one of
  // the five unlisted threads still had a directory holding an empty `WP/`.
  // Dehydration removes FILES; the directory shell it leaves behind is not
  // recorded anywhere as intended or unintended, so this test does not assert
  // on it in either direction -- it just declines to measure realisation with a
  // predicate the residue satisfies. Reported to vc rather than ruled here.
  let realised = || -> BTreeSet<String> {
    std::fs::read_dir(fx.project().st_dir())
      .expect("the estate directory")
      .filter_map(Result::ok)
      .filter(|e| e.path().is_dir())
      .map(|e| e.file_name().to_string_lossy().into_owned())
      .filter(|id| fx.project().info_view(id).exists())
      .collect()
  };

  // **THE PRE-STATE MUST DISAGREE**, or the equality below is satisfied by an
  // estate where nothing ever needed doing.
  let before = realised();
  assert_ne!(
    before,
    listed,
    "PRECONDITION: realised ({}) and listed ({}) must differ before the run,\n       \
     otherwise the equality afterwards is satisfied by an estate that never\n       \
     needed reconciling",
    before.len(),
    listed.len()
  );

  let report = fx
    .facade_on_disk()
    .organize(Mode::Apply)
    .expect("organize reconciles");

  let after = realised();
  assert_eq!(
    after,
    listed,
    "THE RECONCILIATION IS AN EQUALITY OF TWO SETS.\n       \
     realised on disk: {} {:?}\n       \
     listed in .intentfiles: {} {:?}\n       \
     was realised before the run: {} {:?}\n       \
     Each set is derived from its own source and neither consults the other.",
    after.len(),
    after,
    listed.len(),
    listed,
    before.len(),
    before,
  );

  // **THE RUN DID THE WORK, AND THE REPORT SAYS SO IN ITS OWN WORDS.** Set
  // equality alone cannot tell a reconciliation from an estate that was already
  // reconciled -- the pre-state assertion above closes half of that, and this
  // closes the other half by requiring the verb to have MOVED files in both
  // directions on this run.
  assert!(
    !report.dehydrated.is_empty() && !report.hydrated.is_empty(),
    "the equality must be the RESULT of this run: hydrated {} / dehydrated {} / refused {}",
    report.hydrated.len(),
    report.dehydrated.len(),
    report.refused.len()
  );
}
