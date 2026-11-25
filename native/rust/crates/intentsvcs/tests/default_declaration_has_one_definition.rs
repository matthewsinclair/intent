//! ST0057 **AT-11.3, covering AC-11.3**: `intent init`, the upgrade and
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
//! # Why the corpus mixes statuses, and what it buys
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
//! Every test passed on the first run, which is not evidence -- an assertion
//! nobody has watched go red is a claim about an instrument nobody has tested.
//! Each was then reddened by mutating the source in a detached worktree at
//! HEAD, one mutation at a time:
//!
//! | mutation                                    | expected             | observed                            |
//! | ------------------------------------------- | -------------------- | ----------------------------------- |
//! | `init` writes a literal header              | proof red, writer 1  | red at the writer-1 assertion       |
//! | the upgrade writes a literal                | proof red, writer 2  | red at the writer-2 assertion       |
//! | the verb writes a literal                   | proof red, writer 3  | red at the writer-3 assertion       |
//! | a line added to the function's header       | proof GREEN          | green -- all three moved together   |
//! | the predicate reverted to `!is_closed()`    | corpus control red   | red at the corpus control           |
//! | the predicate replaced by `false`           | both controls red    | both red, and the proof still GREEN |
//! | the upgrade regenerates a present file      | present-arm red      | red at the present-arm assertion    |
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
//! # AC-11.3's upgrade clause -- _realises only those threads_
//!
//! `the_upgrade_realises_only_what_it_declares` is below. The upgrade's plan
//! filters against the declaration it is about to write, so it realises only
//! the threads that declaration names.
//!
//! **Writer 2 was the v2 conversion until v2 went** (`intent upgrade` no longer
//! converts a v2 estate). It is now the upgrade of a 3.4.0 project with no
//! manifest, which folds the same function's answer into the records: the
//! same one definition, reached through the door that still writes it.
//!
//! # ST0096 WP-01: two of the three writers now write the records
//!
//! `init` makes a project born upgraded and writes no manifest, and the
//! upgrade folds its declaration into each record's `realised` and removes
//! the file. So their output is compared as the realised SET the records
//! declare, against the same function's answer read through
//! [`realised_from`]. The verb still writes `.intentfiles` in 3.4.0's format,
//! which is the corpus it is driven on here.

use crate::common::{Fixture, facade_ctx, sample_thread};
use intentsvcs::facade::Facade;
use intentsvcs::intentfiles::{Realised, declared_by_records, default_declaration, realised_from};
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

/// What an upgraded project's records declare realised.
fn records_declare(facade: &Facade) -> Realised {
  declared_by_records(&facade.canon().threads, &facade.canon().issues)
}

/// The project at `project` carries no manifest: an upgraded project keeps
/// realisation on its records (ST0096 WP-01).
fn assert_no_manifest(project: &Project, who: &str) {
  let path = project.intentfiles_path();
  assert!(
    !path.exists(),
    "{who} left {} behind, a second home for what the records carry",
    project.relative(&path)
  );
}

/// This estate's threads and their statuses, as the tool holds them.
///
/// **The INPUT comes from the tool and the OUTPUT comes from the function, and
/// that is the whole shape of the assertion.** It claims only that the
/// declaration is this function applied to whatever the statuses are.
fn statuses(facade: &Facade) -> Vec<(String, ThreadStatus)> {
  facade
    .st_list()
    .iter()
    .map(|t| (t.id.stored(), t.status))
    .collect()
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

/// A 3.4.0 project with no manifest, one thread at each status the default
/// tells apart, for the upgrade to declare.
///
/// **Triage is left to the other corpus**, so the two corpora's WIP sets
/// differ and the controls below can fail.
fn upgrade_corpus() -> Fixture {
  // 3.4.0's format with no `.intentfiles`, so the upgrade folds the status
  // default into the records (ST0096 WP-01).
  let fx = Fixture::ordinal();
  fx.write_thread(&thread_at("ST0001", ThreadStatus::Wip));
  fx.write_thread(&thread_at("ST0002", ThreadStatus::NotStarted));
  fx.write_thread(&thread_at("ST0003", ThreadStatus::Hold));
  fx.write_thread(&thread_at("ST0004", ThreadStatus::Completed));
  fx.write_thread(&thread_at("ST0005", ThreadStatus::Cancelled));
  fx
}

/// A v3 estate whose WIP set is DIFFERENT from the upgrade corpus's.
///
/// Different on purpose: it is what makes
/// `control_the_three_writers_do_not_all_write_the_same_bytes` able to fail, and
/// so what stops a caller emitting a constant from passing the proof below.
fn v3_corpus() -> Fixture {
  // 3.4.0's format, the one where `organize --default` writes the manifest.
  let fx = Fixture::ordinal();
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
  assert_no_manifest(&initialised, "`init`");
  let born = Facade::open(initialised, facade_ctx()).expect("open the initialised project");
  assert_eq!(
    records_declare(&born),
    realised_from(&default_declaration(&[], &[])),
    "`init` must declare the function's answer for an empty estate: nothing"
  );

  // WRITER 2 -- the upgrade, through `Facade::upgrade` on a 3.4.0 project
  // with no manifest.
  let converted = upgrade_corpus();
  Facade::upgrade(&converted.project(), &facade_ctx()).expect("a 3.4.0 project upgrades");
  let migrated =
    Facade::open(converted.project(), facade_ctx()).expect("open the upgraded project");
  assert_no_manifest(migrated.project(), "the upgrade");
  assert_eq!(
    records_declare(&migrated),
    realised_from(&default_declaration(&statuses(&migrated), &[])),
    "the upgrade must fold the function's answer for the corpus it just \
     upgraded into the records"
  );

  // WRITER 3 -- the `organize --default` verb, on a different corpus again.
  let declared = v3_corpus();
  let mut facade = declared.facade();
  facade
    .declare_default(false)
    .expect("--default writes an absent manifest");
  assert_eq!(
    manifest(facade.project()),
    default_declaration(&statuses(&facade), &[]),
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
  let converted = upgrade_corpus();
  Facade::upgrade(&converted.project(), &facade_ctx()).expect("a 3.4.0 project upgrades");
  let migrated =
    Facade::open(converted.project(), facade_ctx()).expect("open the upgraded project");
  let corpus = statuses(&migrated);

  let wip_only = default_declaration(&corpus, &[]);
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
  let open_set = default_declaration(&every_open, &[]);

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
  let from_init = records_declare(
    &Facade::open(Project::open(fresh.path()).expect("open"), facade_ctx()).expect("open"),
  );

  let converted = upgrade_corpus();
  Facade::upgrade(&converted.project(), &facade_ctx()).expect("a 3.4.0 project upgrades");
  let from_migration = records_declare(
    &Facade::open(converted.project(), facade_ctx()).expect("open the upgraded project"),
  );

  let declared = v3_corpus();
  let mut facade = declared.facade();
  facade
    .declare_default(false)
    .expect("--default writes an absent manifest");
  let from_verb = realised_from(&manifest(facade.project()));

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
/// The arm that fails SILENTLY. An upgrade that regenerated the declaration
/// would look identical on a first run and would quietly discard a hand
/// realisation on every re-run. Once upgraded, the declaration is on the
/// records (ST0096 WP-01), so the hand realisation is `st hydrate`'s, on the
/// record of a COMPLETED thread, which no regeneration from status produces.
#[test]
fn upgrade_over_a_present_manifest_changes_not_one_byte() {
  let converted = upgrade_corpus();
  Facade::upgrade(&converted.project(), &facade_ctx()).expect("a 3.4.0 project upgrades");
  let project = converted.project();

  let mut facade = Facade::open(project.clone(), facade_ctx()).expect("open the upgraded project");
  facade
    .hydrate(&intentsvcs::address::Address {
      authority: None,
      entity: intentsvcs::address::Entity::Thread {
        id: "ST0004".to_string(),
      },
      format: None,
    })
    .expect("a completed thread hydrates by hand");
  assert!(
    facade.st_show("ST0004").expect("ST0004").realised,
    "precondition: the hand hydration is on the record"
  );
  drop(facade);
  let by_hand = std::fs::read_to_string(project.thread_json("ST0004")).expect("ST0004's record");

  Facade::upgrade(&project, &facade_ctx()).expect("an upgraded project re-runs");
  assert_eq!(
    std::fs::read_to_string(project.thread_json("ST0004")).expect("ST0004's record"),
    by_hand,
    "a re-run must leave a record's declaration alone -- regenerating it from \
     status discards every hand realisation the estate had recorded"
  );
}
/// **AC-11.3's UPGRADE CLAUSE: _realises only those threads_.**
///
/// The corpus is one WIP thread and four that are not, so the declared set and
/// the upgraded set genuinely differ -- **on a corpus of WIP threads alone this
/// test would pass against an upgrade that realised everything**, which is the
/// same vacuity `control_the_corpus_can_tell_the_candidate_definitions_apart`
/// exists to refuse one function along.
#[test]
fn the_upgrade_realises_only_what_it_declares() {
  let converted = upgrade_corpus();

  Facade::upgrade(&converted.project(), &facade_ctx()).expect("a 3.4.0 project upgrades");

  let after = crate::common::tree(converted.root());
  let migrated =
    Facade::open(converted.project(), facade_ctx()).expect("open the upgraded project");

  // The declaration is folded into the records (ST0096 WP-01).
  assert_no_manifest(migrated.project(), "the upgrade");
  let declared: Vec<String> = migrated
    .st_list()
    .iter()
    .filter(|t| t.realised)
    .map(|t| t.id.stored())
    .collect();
  assert_eq!(
    declared,
    vec!["ST0001".to_string()],
    "the corpus has exactly one WIP thread, so the upgrade must declare exactly it"
  );

  let project = converted.project();
  assert!(
    after.contains_key(&converted.rel(&project.acceptance_view("ST0001"))),
    "the declared thread was not realised -- the upgrade wrote no view for it"
  );
  for id in ["ST0002", "ST0003", "ST0004", "ST0005"] {
    assert!(
      !after.contains_key(&converted.rel(&project.acceptance_view(id))),
      "{id} is NOT declared and the upgrade realised it anyway"
    );
  }
}
