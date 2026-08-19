//! AT-03.12 / AC-03.11: **a cold start restores the event log, and a log that
//! is missing is REPORTED rather than read as "this project has no history".**
//!
//! `ingest::resync` rebuilt seven tables from the extract and skipped the
//! eighth. Every other table is derived, so omitting one is recoverable;
//! `event_log` is derived from nothing (D34), so omitting it lost the data
//! outright. Its reach was the whole cold-store path -- `load_fresh` warms an
//! empty store through `resync`, and **an empty store is the normal state of
//! every fresh clone**, because `intent/.cache/` is gitignored. So a clone
//! answered every question correctly and had no history at all.
//!
//! **The obvious test for this passes on the defect**, which is why the shape
//! below is prescribed rather than left to taste. Populate a store, call
//! `resync`, assert the log survived: green before the fix and after it,
//! because that store already held its events and the defect is in the path
//! that warms an EMPTY one. The fixture has to be a project that never had a
//! database, not one whose database was removed -- `clone_extract()` copies the
//! `intent/` tree and skips `.cache/`, which is what a `git clone` gives you.
//!
//! **Deliberately not satisfied by AT-02.8 or AT-04.5.** Both mention
//! `event_log`, both were green before this defect and after it, and neither is
//! sensitive to it; naming them as coverage is the OR-not-AND trap.

mod common;

use common::Fixture;
use intentsvcs::event;
use intentsvcs::facade::Facade;
use intentsvcs::finding::FindingClass;

/// A project with real history: three mutations, then the whole-estate
/// projection that writes `events.jsonl` beside the canon.
///
/// Through the facade rather than by writing envelopes directly, because the
/// property under test is that history SURVIVES the round trip, and a
/// hand-written log would prove only that the reader reads what the test wrote.
fn estate_with_history() -> Fixture {
  let fixture = Fixture::new();
  {
    let mut f = fixture.facade_on_disk();
    // The ratified path -- v3 enters at `Triage`, so `new -> start` is two
    // edges rather than one. Walking it rather than jumping gives the log more
    // than one envelope to lose.
    //
    // It stops short of `done` on purpose: the close gate blocks a thread with
    // an empty contract, and satisfying it here would put the gate's rules into
    // a fixture whose subject is the event log. Four envelopes is a past.
    let id = f.st_new("A thread with a past").expect("st new");
    f.st_triage(&id).expect("st triage");
    f.st_start(&id).expect("st start");
    f.st_hold(&id, "waiting on a peer").expect("st hold");
    f.sync_to_disk(&intentsvcs::sync::Scope::All)
      .expect("project the estate, event log included");
  }
  fixture
}

/// **The property: a fresh clone knows what happened to it.**
#[test]
fn a_clone_that_never_had_a_database_still_has_its_history() {
  let origin = estate_with_history();
  let before = {
    let f = origin.facade_on_disk();
    f.store().events().expect("read the origin's log")
  };
  assert!(
    before.len() >= 3,
    "the fixture is meant to have a past: {} envelope(s)",
    before.len()
  );

  let clone = origin.clone_extract();
  // The precondition, asserted rather than assumed: if `clone_extract` ever
  // starts carrying `.cache/`, this test would silently become the weaker one
  // it exists to replace -- a store that HAD a database rather than one that
  // never did.
  assert!(
    !clone.path("intent/.cache").exists(),
    "the fixture must be a clone, not a copy: a carried `.cache/` makes this test vacuous"
  );

  // Opening warms the empty store through `load_fresh` -> `resync`. Nothing
  // here calls a restore explicitly, because a user does not either.
  let warmed = clone.facade_on_disk();
  let after = warmed.store().events().expect("read the clone's log");

  assert_eq!(
    after.len(),
    before.len(),
    "the clone restored {} of {} envelopes",
    after.len(),
    before.len()
  );
  let ids = |envelopes: &[event::Envelope]| -> Vec<String> {
    let mut v: Vec<String> = envelopes.iter().map(|e| e.id.clone()).collect();
    v.sort();
    v
  };
  assert_eq!(
    ids(&after),
    ids(&before),
    "the SAME envelopes, by id -- an equal count with different envelopes would be a different \
     defect wearing this one's green"
  );
  let stamps = |envelopes: &[event::Envelope]| -> Vec<String> {
    let mut v: Vec<String> = envelopes.iter().map(|e| e.ts.clone()).collect();
    v.sort();
    v
  };
  assert_eq!(
    stamps(&after),
    stamps(&before),
    "and carrying their ORIGINAL stamps (D42): restoring history is not the same as it happening \
     again, and a re-stamped log would look perfectly valid"
  );
}

/// **The half the restore alone does not give: history that would not survive
/// a clone is REPORTED.**
///
/// **REPORTED, not refused, and narrowed twice -- both times by measurement.**
/// The first version refused to open an estate with entities and no history,
/// arguing that under D34 every mutation writes an envelope. The suite refuted
/// it in one run: a hand-authored `thread.json` is an entity that never came
/// from a mutation, and that is precisely what WP-10's migration produces, so
/// the refusal would have refused every migrated estate. The second version
/// reported the same condition, and two doctor fixtures fired it immediately --
/// correctly, which was the problem: the per-thread mutation path does not
/// rewrite the log extract, so a normally-used project is in that state
/// routinely, and a finding that fires routinely is the one nobody reads.
///
/// The condition that survives is two artefacts disagreeing: **this store holds
/// envelopes the repository does not**. That is not a guess about intent, it is
/// history that exists on one machine only -- and it is reported to the person
/// who still HAS it, rather than to whoever clones it afterwards and can no
/// longer do anything.
#[test]
fn history_this_machine_holds_and_the_repository_does_not_is_reported() {
  let fixture = estate_with_history();
  let log = fixture.path("intent/events.jsonl");
  assert!(
    log.exists(),
    "the fixture must carry a log before removing it"
  );
  std::fs::remove_file(&log).expect("remove the committed extract");

  let project = fixture.project();
  let f = Facade::open(fixture.project(), common::facade_ctx())
    .expect("the project still OPENS -- its history is here, it is just not committed");
  let report = Facade::doctor(&project, &common::facade_ctx(), Some(f.store()));

  let found = report
    .findings
    .iter()
    .find(|finding| finding.class == FindingClass::EventLogAbsent)
    .expect("doctor must report it, because nothing else in the estate reveals it");
  assert!(
    found.detail.contains("would arrive with no history"),
    "and say what the consequence is, since the project itself looks fine: {}",
    found.detail
  );
  assert!(
    found.class.remedy().contains("nothing recomputes history"),
    "with the remedy that matters -- no rebuild restores it: {}",
    found.class.remedy()
  );
}

/// **A HEALTHY estate does not carry the finding**, which is what makes the one
/// above mean anything. Same estate, one file left alone.
#[test]
fn an_estate_that_kept_its_log_is_not_reported() {
  let fixture = estate_with_history();
  let project = fixture.project();
  let f = Facade::open(fixture.project(), common::facade_ctx()).expect("open");
  let report = Facade::doctor(&project, &common::facade_ctx(), Some(f.store()));
  assert!(
    !report
      .findings
      .iter()
      .any(|finding| finding.class == FindingClass::EventLogAbsent),
    "the log is right there: {:?}",
    report.findings
  );
}

/// **A project whose history was never recorded is NOT reported**, and this is
/// the arm that keeps the check honest rather than merely quiet.
///
/// A hand-authored estate -- which is what a migration produces -- has entities
/// and no envelopes. Nothing local can tell that from history that was lost, so
/// the check says nothing rather than guessing, and the case is with vc: it
/// needs a ruling on how current the committed extract must be, which is a D34
/// question rather than a diagnostic one.
#[test]
fn an_estate_that_never_recorded_history_is_not_accused_of_losing_it() {
  let fixture = Fixture::new();
  fixture.write_thread(&common::sample_thread("ST0001"));

  let project = fixture.project();
  let f = Facade::open(fixture.project(), common::facade_ctx())
    .expect("a hand-authored estate opens -- this is the shape WP-10 produces");
  let report = Facade::doctor(&project, &common::facade_ctx(), Some(f.store()));
  assert!(
    !report
      .findings
      .iter()
      .any(|finding| finding.class == FindingClass::EventLogAbsent),
    "no envelopes exist to be missing, so there is nothing to report: {:?}",
    report.findings
  );
}

/// **An empty log FILE answers the same as a missing one.**
///
/// The check asks the file's SIZE rather than whether the path exists, so a
/// zero-byte `events.jsonl` -- which a truncating write or a bad merge leaves
/// behind, and which `exists()` calls healthy -- lands in the same place.
#[test]
fn an_empty_log_file_is_the_same_answer_as_an_absent_one() {
  let fixture = estate_with_history();
  std::fs::write(fixture.path("intent/events.jsonl"), "").expect("truncate the log");

  let project = fixture.project();
  let f = Facade::open(fixture.project(), common::facade_ctx()).expect("open");
  assert!(
    Facade::doctor(&project, &common::facade_ctx(), Some(f.store()))
      .findings
      .iter()
      .any(|finding| finding.class == FindingClass::EventLogAbsent),
    "a truncated extract carries no history, whatever its path says"
  );
}
