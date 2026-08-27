//! ST0057 **AT-11.3, covering AC-11.3**: `intent init`, the migration and
//! `organize --default` are three WRITERS of `.intentfiles` and ONE definition
//! of what the default declares.
//!
//! # The criterion asks for a test that "changes the function and sees all three
//! # callers move", and Rust offers no seam for changing it
//!
//! `default_declaration` cannot be substituted at runtime, so the proof is
//! built the other way round: **every assertion compares the bytes a caller
//! wrote against the bytes the function returns for that estate's own
//! statuses, computed live in this test.** Not one expectation here is a
//! literal. A change to the header, the sigil, the sort or the predicate moves
//! the expectation and all three actuals together -- and a caller that stopped
//! deriving from the function is the one that stops moving, which is the
//! failure the criterion names.
//!
//! **A LITERAL EXPECTATION WOULD HAVE BEEN THE WEAKER INSTRUMENT AND IT LOOKS
//! LIKE THE STRONGER ONE.** It pins the content, so it reddens whenever the
//! function changes, which reads as rigour. The property wanted here is that
//! the three AGREE, not that the text never moves -- and a literal turns every
//! legitimate edit into three failures a person fixes by editing three
//! literals. Three literals is three homes for the definition: the defect the
//! criterion exists to prevent, arriving through the test suite instead of
//! through the source.
//!
//! # What was already covered, measured before a line of this was written
//!
//! Two files already drive `default_declaration` and **neither can see this
//! criterion**, which is why this is a new file rather than three assertions
//! added to one of them. `intentfiles_default_declaration.rs` calls the
//! function directly -- so it pins what the function COMPUTES and is
//! structurally blind to whether anything calls it, the same shape as the
//! `intentfiles::default_declaration` that once landed with zero production
//! callers and was reported done. `organize_default_declaration.rs` drives the
//! binary for AC-11.1, AC-11.2, AC-11.4 and AC-11.5 -- one caller, through one
//! verb. **A subject grep hits both and tells you nothing about which limbs
//! they cover**, and the limb this criterion is about -- three callers reaching
//! one function -- is covered by neither.
//!
//! # Why the corpus has five statuses, and what it buys
//!
//! A comparison against the live function passes for ANY caller on an
//! all-WIP corpus, because every candidate definition agrees there. The corpus
//! carries WIP, Not Started, On Hold, Completed and Cancelled precisely so the
//! candidates disagree, and
//! `control_the_corpus_can_tell_the_candidate_definitions_apart` asserts that
//! it does -- **a comparison that cannot go red is not a comparison**
//! (`IN-AG-RED-CONTROL-001`), and an all-WIP corpus would degrade this file to
//! decoration without changing a line of it.
//!
//! The two candidates are not hypothetical. `!is_closed()` is what this
//! function computed until 2026-08-26, and it is what a reader reaches for
//! when the name is "the open set"; `status == Wip` is hv's ruling. The
//! control states the difference in the only terms that cannot drift: it asks
//! the function itself both questions and requires two answers.
//!
//! # Every assertion here has been SEEN to fail, and the run is recorded
//!
//! All four tests passed on the first run, which is not evidence -- an
//! assertion nobody has watched go red is a claim about an instrument nobody
//! has tested. Each of the four was then reddened by mutating the source in a
//! detached worktree at HEAD, one mutation at a time:
//!
//! | mutation                                    | expected             | observed                            |
//! | ------------------------------------------- | -------------------- | ----------------------------------- |
//! | `init` writes a literal header              | proof red, writer 1  | red at the writer-1 assertion       |
//! | the migration writes a literal              | proof red, writer 2  | red at the writer-2 assertion       |
//! | the verb writes a literal                   | proof red, writer 3  | red at the writer-3 assertion       |
//! | a line added to the function's header       | proof GREEN          | green -- all three moved together   |
//! | the predicate reverted to `!is_closed()`    | corpus control red   | red at the corpus control           |
//! | the predicate replaced by `false`           | both controls red    | both red, and the proof still GREEN |
//! | the migration regenerates a present file    | present-arm red      | red at the present-arm assertion    |
//!
//! **THE `false` ROW IS THE WHOLE ARGUMENT FOR THE TWO CONTROLS, AND IT IS THE
//! ONE I DID NOT PREDICT BEFORE RUNNING IT.** A `default_declaration` that
//! declares NOTHING leaves the proof test green -- correctly, on its own terms:
//! all three writers still derive from the function and all three moved
//! together, which is exactly what the criterion asks and all it asks. The
//! proof alone would have certified a function that realises no thread in any
//! estate. **What catches it is the pair of controls**, and this is why they
//! are not decoration around the real test: the criterion's property is
//! AGREEMENT, and three writers agreeing on nothing is a perfect score.
//!
//! The `header` row is the other half and the only other green: changing the
//! FUNCTION must not redden the proof, or the test would punish every
//! legitimate edit and teach a maintainer to pin literals.
//!
//! # AC-11.3's migration clause -- _realises only those threads_
//!
//! **THIS SECTION USED TO SAY THE CLAUSE WAS NOT COVERED HERE, AND IT IS NOW.**
//! `the_migration_realises_only_what_it_declares` is below. When this file was
//! written the migration declared the WIP set AFTER `migrate::plan` had already
//! realised everything, so the declaration landed over a fully realised tree
//! and the clause was unmet; the note said so and pointed at the gap. The
//! behaviour landed 2026-08-27 (`migrate::plan` filters against the bytes it is
//! about to write) and the assertion follows it.
//!
//! **WHAT THE CLAUSE CANNOT MEAN, AND THE TEST BELOW IS SHAPED BY IT.**
//! `WriteSet` has no remove, so a v2 estate's non-WIP thread directories stay
//! on disk holding their v2 content. "Realises only those threads" is therefore
//! a claim about what the migration WRITES, never about what survives -- and
//! the test asserts both halves: the v3-only view appears for the declared
//! thread and for no other, and every undeclared thread's file is byte-identical
//! to what it was before the run. **The residue is AC-11.4's state, not a
//! hole**, and `organize` reports it as to-remove-and-blocked.

mod common;

use common::{Fixture, facade_ctx, sample_thread, v2_estate, v2_thread};
use intentsvcs::facade::Facade;
use intentsvcs::intentfiles::default_declaration;
use intentsvcs::model::{Thread, ThreadStatus};
use intentsvcs::project::Project;

/// The manifest as it stands on disk, or a panic naming the path.
///
/// **Read through `Project` rather than joined here.** A second spelling of
/// `intent/.intentfiles` in the test would pass against a caller writing to the
/// wrong place, provided the test wrote to the wrong place the same way.
fn manifest(project: &Project) -> String {
  let path = project.intentfiles_path();
  std::fs::read_to_string(&path)
    .unwrap_or_else(|e| panic!("no manifest at {}: {e}", project.relative(&path)))
}

/// This estate's threads and their statuses, as the tool holds them.
///
/// **The INPUT comes from the tool and the OUTPUT comes from the function, and
/// that is the whole shape of the assertion.** The test is not claiming to know
/// which statuses the migrator assigned -- that is AC-10.3's question. It
/// claims only that the manifest is this function applied to whatever they
/// turned out to be.
fn statuses(facade: &Facade) -> Vec<(String, ThreadStatus)> {
  facade
    .st_list()
    .iter()
    .map(|t| (t.id.clone(), t.status))
    .collect()
}

/// A v2 estate carrying one thread of each status v2 can express.
///
/// **Triage is absent because v2 has no such status** -- `legacy::thread_status`
/// maps `tbc` to Not Started and nothing to Triage, deliberately. Corpus B
/// below supplies it.
fn v2_corpus() -> Fixture {
  let fx = v2_estate();
  v2_thread(&fx, "ST0001", "WIP");
  v2_thread(&fx, "ST0002", "Not Started");
  v2_thread(&fx, "ST0003", "On Hold");
  v2_thread(&fx, "ST0004", "Completed");
  v2_thread(&fx, "ST0005", "Cancelled");
  fx
}

/// A canon thread at `id` with `status`, otherwise the sample.
fn thread_at(id: &str, status: ThreadStatus) -> Thread {
  let mut t = sample_thread(id);
  t.status = status;
  if status.is_closed() {
    t.completed = Some("2026-08-26".to_string());
  }
  t
}

/// A v3 estate whose WIP set is DIFFERENT from the v2 corpus's.
///
/// Different on purpose: it is what makes
/// `control_the_three_writers_do_not_all_write_the_same_bytes` able to fail, and
/// so what stops a caller emitting a constant from passing the proof below.
fn v3_corpus() -> Fixture {
  let fx = Fixture::new();
  fx.write_thread(&thread_at("ST0010", ThreadStatus::Wip));
  fx.write_thread(&thread_at("ST0011", ThreadStatus::Wip));
  fx.write_thread(&thread_at("ST0012", ThreadStatus::Triage));
  fx.write_thread(&thread_at("ST0013", ThreadStatus::Completed));
  fx
}

/// **THE PROOF: one definition, three writers, and they move together.**
///
/// One test rather than three, because the criterion's property is the
/// AGREEMENT -- three separate greens each say "this caller matches the
/// function" and none of them says "and so do the other two".
#[test]
fn the_three_writers_all_derive_their_content_from_the_one_function() {
  // WRITER 1 -- `intent init`, on a directory with no threads at all.
  let fresh = tempfile::tempdir().expect("tempdir");
  intentsvcs::init::init(fresh.path(), "Probe", "ic", "3.0.0").expect("init");
  let initialised = Project::open(fresh.path()).expect("open the initialised project");
  assert_eq!(
    manifest(&initialised),
    default_declaration(&[]),
    "`init` must write the function's answer for an empty estate -- header \
     present, no declarations. A mismatch here means init spells the header \
     itself, which is a second home for it."
  );

  // WRITER 2 -- the migration, through `Facade::upgrade` on a v2 estate.
  let converted = v2_corpus();
  Facade::upgrade(&converted.project(), &facade_ctx()).expect("a clean v2 estate converts");
  let migrated =
    Facade::open(converted.project(), facade_ctx()).expect("open the converted estate");
  assert_eq!(
    manifest(migrated.project()),
    default_declaration(&statuses(&migrated)),
    "the migration must write the function's answer for the corpus it just \
     converted"
  );

  // WRITER 3 -- the `organize --default` verb, on a different corpus again.
  let declared = v3_corpus();
  let mut facade = declared.facade();
  facade
    .declare_default(false)
    .expect("--default writes an absent manifest");
  assert_eq!(
    manifest(facade.project()),
    default_declaration(&statuses(&facade)),
    "the verb must write the function's answer for this estate's statuses"
  );
}

/// **CONTROL: the corpus can tell the two candidate definitions apart.**
///
/// Asked of the function itself, twice, so there is no second implementation of
/// either candidate to drift. Promoting every OPEN thread to WIP yields what
/// `!is_closed()` would have declared; if the corpus held no open non-WIP
/// thread the two answers would be identical and every assertion in the proof
/// above would hold for a caller using either rule.
#[test]
fn control_the_corpus_can_tell_the_candidate_definitions_apart() {
  let converted = v2_corpus();
  Facade::upgrade(&converted.project(), &facade_ctx()).expect("a clean v2 estate converts");
  let migrated =
    Facade::open(converted.project(), facade_ctx()).expect("open the converted estate");
  let corpus = statuses(&migrated);

  let wip_only = default_declaration(&corpus);
  let every_open: Vec<(String, ThreadStatus)> = corpus
    .iter()
    .map(|(id, status)| {
      let promoted = if status.is_closed() {
        *status
      } else {
        ThreadStatus::Wip
      };
      (id.clone(), promoted)
    })
    .collect();
  let open_set = default_declaration(&every_open);

  assert_ne!(
    wip_only, open_set,
    "TWO CAUSES REACH THIS FAILURE AND THEY WANT OPPOSITE FIXES. Either the \
     corpus above lost its open non-WIP threads, in which case the proof test \
     is asserting nothing about WHICH definition a caller used and the fix is \
     to the corpus -- or `default_declaration` has gone back to `!is_closed()`, \
     in which case the two candidates agree by construction, hv\'s WIP-only \
     ruling has been reverted, and the fix is to the function. Read the \
     predicate before touching this file. (Both reachable: mutating the \
     predicate is how this control was proven able to go red.)"
  );
}

/// **CONTROL: the three writers do not all write the same bytes.**
///
/// The proof compares each caller to the function. A caller that emitted one
/// constant would satisfy that comparison if the function's answer were the
/// same everywhere, so the three estates are built to have three different
/// answers and this says so out loud.
#[test]
fn control_the_three_writers_do_not_all_write_the_same_bytes() {
  let fresh = tempfile::tempdir().expect("tempdir");
  intentsvcs::init::init(fresh.path(), "Probe", "ic", "3.0.0").expect("init");
  let from_init = manifest(&Project::open(fresh.path()).expect("open"));

  let converted = v2_corpus();
  Facade::upgrade(&converted.project(), &facade_ctx()).expect("a clean v2 estate converts");
  let from_migration = manifest(&converted.project());

  let declared = v3_corpus();
  let mut facade = declared.facade();
  facade
    .declare_default(false)
    .expect("--default writes an absent manifest");
  let from_verb = manifest(facade.project());

  assert_ne!(
    from_init, from_migration,
    "an empty estate and a corpus with a WIP thread must not declare the same"
  );
  assert_ne!(
    from_migration, from_verb,
    "two corpora with different WIP sets must not declare the same -- if they \
     do, the proof test would pass against a caller writing a constant"
  );
}

/// AC-11.3's other half for the re-run: **present means touch nothing.**
///
/// The arm that fails SILENTLY. A migration that regenerated the manifest would
/// look identical on a fresh conversion and would quietly discard a hand
/// dehydration on every re-run, which is the one operation `.intentfiles`
/// exists to record.
#[test]
fn upgrade_over_a_present_manifest_changes_not_one_byte() {
  let converted = v2_corpus();
  Facade::upgrade(&converted.project(), &facade_ctx()).expect("a clean v2 estate converts");
  let project = converted.project();

  // A declaration no regeneration from status would ever produce: it names a
  // COMPLETED thread, which is what a hand hydration looks like on disk.
  let by_hand = format!("{}STEELTHREAD:ST0004\n", default_declaration(&[]));
  std::fs::write(project.intentfiles_path(), &by_hand).expect("write the hand manifest");

  Facade::upgrade(&project, &facade_ctx()).expect("a converted estate re-runs");
  assert_eq!(
    manifest(&project),
    by_hand,
    "a re-run must leave a present manifest alone -- regenerating it discards \
     every hand realisation the estate had recorded"
  );
}
/// **AC-11.3's MIGRATION CLAUSE: _realises only those threads_.**
///
/// The corpus is one WIP thread and four that are not, so the declared set and
/// the converted set genuinely differ -- **on a corpus of WIP threads alone this
/// test would pass against a migration that realised everything**, which is the
/// same vacuity `control_the_corpus_can_tell_the_candidate_definitions_apart`
/// exists to refuse one function along.
///
/// **`acceptance.md` IS THE INSTRUMENT BECAUSE v2 HAD NO SUCH FILE.** Its
/// presence is unambiguous evidence that THIS run rendered a v3 view for that
/// thread; asking whether `info.md` exists could not tell a view the migration
/// wrote from the v2 file that was always there.
#[test]
fn the_migration_realises_only_what_it_declares() {
  let converted = v2_corpus();
  let before = common::tree(converted.root());

  Facade::upgrade(&converted.project(), &facade_ctx()).expect("a clean v2 estate converts");

  let after = common::tree(converted.root());
  let migrated =
    Facade::open(converted.project(), facade_ctx()).expect("open the converted estate");

  let declared: Vec<String> = manifest(migrated.project())
    .lines()
    .filter_map(|l| l.trim().strip_prefix("STEELTHREAD:"))
    .map(|id| id.to_string())
    .collect();
  assert_eq!(
    declared,
    vec!["ST0001".to_string()],
    "the corpus has exactly one WIP thread, so the migration must declare exactly it"
  );

  assert!(
    after.contains_key("intent/st/ST0001/acceptance.md"),
    "the declared thread was not realised -- the migration wrote no v3 view for it"
  );

  for id in ["ST0002", "ST0003", "ST0004", "ST0005"] {
    assert!(
      !after.contains_key(&format!("intent/st/{id}/acceptance.md")),
      "{id} is NOT declared and the migration realised it anyway"
    );
    // **AND IT WAS NOT REMOVED EITHER.** Both halves matter: a migration that
    // deleted the undeclared threads would satisfy the line above and break
    // AC-11.4, which is the criterion that says every pre-existing file is
    // still there byte for byte.
    let rel = format!("intent/st/{id}/info.md");
    assert_eq!(
      before.get(&rel),
      after.get(&rel),
      "{id} is undeclared, so the migration must neither refresh nor remove {rel}"
    );
  }
}
