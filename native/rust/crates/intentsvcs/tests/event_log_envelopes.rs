//! AT-04.5 / AC-04.5: every mutation path writes an event-log envelope
//! carrying principal + project_id -- the D15 cloud seams exist end to end.
//!
//! Renumbered in from AC-02.6 at the WP-02 close, so this is real work in this
//! WP rather than inherited green.
//!
//! The test is written against the VERB SET rather than against a list of
//! calls: it drives every mutating verb the facade exposes and requires an
//! envelope from each. A test that checked three verbs would pass while the
//! fourth quietly wrote nothing, and "every mutation path" is precisely the
//! claim that cannot be sampled.

mod common;

use common::{Fixture, PROJECT_ID, sample_thread};
use intentsvcs::model::{AtStatus, TShirt};

/// Every op recorded, in order.
fn ops(facade: &intentsvcs::facade::Facade) -> Vec<String> {
  facade
    .store()
    .events()
    .expect("read the event log")
    .into_iter()
    .map(|e| e.op)
    .collect()
}

#[test]
fn every_mutating_verb_writes_an_envelope() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();

  assert!(
    ops(&facade).is_empty(),
    "opening a project is a read; it must not log a mutation"
  );

  // Drive the whole mutating surface.
  facade.st_new("a second thread").expect("st.new");
  facade
    .st_triage("ST0057")
    .expect("st.triage -- `st new` enters at triage");
  facade.st_start("ST0057").expect("st.start");
  facade.wp_new("ST0057", "a wp", TShirt::S).expect("wp.new");
  facade.wp_start("ST0057", 1).expect("wp.start");
  facade
    .st_cancel("ST0057", "superseded by the v3 line")
    .expect("st.cancel");

  facade
    .ac_satisfy("ST0056", "AC-03.2", "evidence")
    .expect("ac.satisfy");
  // `rescope` undoes a DESCOPE and `reinstate` undoes a WITHDRAWAL; each
  // refuses the other's state and names it (v2, `bin/intent_acceptance:1241`
  // and `:1246`). They are driven in their own states here, because calling
  // either on the wrong one is now a refusal rather than a no-op -- which is
  // how this guard caught the change.
  facade
    .ac_descope("ST0056", "AC-03.1", "ST0057", None, None)
    .expect("ac.descope");
  facade.ac_rescope("ST0056", "AC-03.1").expect("ac.rescope");
  facade
    .ac_withdraw("ST0056", "AC-03.1", "reason", None)
    .expect("ac.withdraw");
  facade
    .ac_reinstate("ST0056", "AC-03.1")
    .expect("ac.reinstate");
  facade
    .at_set("ST0056", "AT-03.1", AtStatus::Red)
    .expect("at.set");

  let recorded = ops(&facade);
  for expected in [
    "st.new",
    "st.start",
    "st.cancel",
    "wp.new",
    "wp.start",
    "ac.satisfy",
    "ac.descope",
    "ac.rescope",
    "ac.withdraw",
    "ac.reinstate",
    "at.set",
    // Ratified 2026-08-15: `st new` enters at triage, so reaching `wip`
    // is two transitions and the log records both.
    "st.triage",
  ] {
    assert!(
      recorded.iter().any(|op| op == expected),
      "no envelope for {expected}; recorded: {recorded:?}"
    );
  }
  assert_eq!(
    recorded.len(),
    12,
    "one envelope per mutation, no more and no fewer: {recorded:?}"
  );
}

#[test]
fn wp_done_and_st_done_log_their_closes() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();

  facade.wp_done("ST0056", 3).expect("wp.done");
  facade.st_done("ST0056").expect("st.done");

  let recorded = ops(&facade);
  assert_eq!(recorded, vec!["wp.done", "st.done"]);
}

#[test]
fn every_envelope_carries_the_principal_and_project_id() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();
  facade
    .st_hold("ST0056", "waiting on the fleet")
    .expect("a legal mutation from wip");
  facade
    .at_set("ST0056", "AT-03.1", AtStatus::Red)
    .expect("at set");

  let events = facade.store().events().expect("events");
  assert_eq!(events.len(), 2);
  for e in &events {
    assert_eq!(e.principal, "cc", "the D15 principal seam is carried");
    assert_eq!(
      e.project_id, PROJECT_ID,
      "the D15 project_id seam is carried"
    );
    assert!(!e.id.is_empty(), "every envelope has a ULID");
    assert!(
      e.ts.contains('T') && e.ts.ends_with('Z'),
      "the timestamp is RFC 3339 UTC, got: {}",
      e.ts
    );
  }
}

#[test]
fn the_envelope_names_its_subject_and_carries_the_transition() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();
  facade
    .wp_reopen("ST0056", 2, "AC-02.6 was added after the close")
    .expect("wp reopen");

  let events = facade.store().events().expect("events");
  let e = events.last().expect("one event");
  assert_eq!(e.subject.kind, "wp");
  assert_eq!(e.subject.id, "ST0056/02");
  assert_eq!(e.payload["from"], "done", "the fixture WP-02 was done");
  assert_eq!(
    e.payload["reason"], "AC-02.6 was added after the close",
    "a guarded verb puts its reason in the envelope, which is where the HISTORY of decisions lives -- `status_reason` on the entity only ever carries the latest"
  );
  assert_eq!(e.payload["to"], "wip");
}

/// A refused mutation writes NOTHING -- not the change, and not an envelope
/// saying it happened. An event log that records attempts as events is a log
/// that lies about what the estate did.
#[test]
fn a_refused_mutation_writes_no_envelope() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();

  let _ = facade
    .ac_satisfy("ST0056", "AC-03.1", "x")
    .expect_err("refused");
  let _ = facade.st_show("ST9999").expect_err("refused");
  let _ = facade.wp_start("ST0056", 99).expect_err("refused");

  assert!(
    ops(&facade).is_empty(),
    "refusals are not events: {:?}",
    ops(&facade)
  );
}

/// The event log survives a rebuild (D15: it is the deliberate exception to
/// derivability), so a mutation's envelope is not wiped by the next mutation's
/// rebuild of the derived tables.
#[test]
fn envelopes_accumulate_across_mutations_rather_than_being_rebuilt_away() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();

  facade
    .st_hold("ST0056", "waiting on the fleet")
    .expect("a legal mutation from wip");
  facade
    .wp_reopen("ST0056", 2, "AC-02.6 was added after the close")
    .expect("wp reopen -- WP-02 is Done in the fixture, so `wp start` is not a legal edge");
  facade
    .at_set("ST0056", "AT-03.1", AtStatus::Red)
    .expect("at set");

  assert_eq!(
    ops(&facade).len(),
    3,
    "each mutation writes its own entities and the log is append-only -- three mutations, three envelopes, and no rebuild that could wipe them"
  );
}

/// **A FILE-WRITE FAILURE STILL RECORDS THE ENVELOPE, because the mutation
/// happened.**
///
/// This test asserted the exact opposite until hv reversed D01 on 2026-08-15
/// -- "the envelope is minted only after the files land" -- and that was
/// correct while committed canon was durable truth. Now the DB is the SSOT, so
/// the envelope is written in the SAME TRANSACTION as the change and the files
/// are a projection written afterwards. A failure to project cannot unmake the
/// event.
///
/// The inverted assertion is the one AC-04.5 actually needs. "Every mutation
/// path writes an event-log envelope" is a claim about mutations that HAPPEN,
/// and under the old order a mutation could land in the DB and be denied its
/// envelope by an unrelated filesystem error -- an unaudited change to what is
/// now the source of truth.
#[cfg(unix)]
#[test]
fn a_file_write_failure_still_records_the_envelope() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();
  facade
    .st_hold("ST0056", "waiting on the fleet")
    .expect("a legal mutation from wip");
  let before = ops(&facade).len();

  let mode = fx.make_readonly("intent");
  let result = facade.st_cancel("ST0056", "superseded by the v3 line");
  fx.restore_mode("intent", mode);
  assert!(result.is_err(), "precondition: the file write failed");

  assert_eq!(
    ops(&facade).len(),
    before + 1,
    "the change landed in the store, so it is audited there -- a projection failure does not erase an event that occurred"
  );
  assert_eq!(
    ops(&facade).last().map(String::as_str),
    Some("st.cancel"),
    "and the envelope is the one for the mutation that actually ran"
  );

  // The entity moved with its envelope, which is what makes them one
  // transaction rather than two writes that happened to both succeed.
  assert_eq!(
    facade.st_show("ST0056").expect("thread").status,
    intentsvcs::model::ThreadStatus::Cancelled,
    "the mutation is in the store, not merely logged"
  );
}

/// The other direction -- a failed DB transaction writing no envelope -- is
/// NOT reachable through the facade in a fixture, and that is a property
/// rather than a gap in the test.
///
/// `commit_mutation` puts the entities, the prose index and the envelope in
/// one transaction, so the only way to write an entity without its envelope is
/// for SQLite to fail mid-transaction, which rolls back all three. Reaching it
/// would mean feeding the store data the typed API cannot express -- and that
/// is exactly hv's stated guarantee for the reversal: "the typed API ensures
/// the only data that goes into the db conforms by construction to the
/// schema". Recorded here rather than left as an untested branch nobody
/// noticed was untestable.
#[test]
fn the_envelope_and_the_change_share_one_transaction() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();
  let before = ops(&facade).len();
  facade
    .st_hold("ST0056", "waiting on the fleet")
    .expect("a legal mutation from wip");
  assert_eq!(ops(&facade).len(), before + 1);
  assert_eq!(
    facade.st_show("ST0056").expect("thread").status,
    intentsvcs::model::ThreadStatus::Hold
  );
}
