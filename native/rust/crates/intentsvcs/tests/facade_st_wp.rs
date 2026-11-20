//! AT-04.1 / AC-04.1: st/wp lifecycle verbs run through the facade with canon,
//! views and DB written transactionally -- a failure mid-write leaves no torn
//! state.
//!
//! The torn-state clause is INJECTED, not reasoned about. A read-only
//! directory makes a real `rename` fail the way the operating system fails it,
//! part-way through a batch that has already written other files. A synthetic
//! seam the production code knows about would prove the seam works.

use crate::common::{Fixture, sample_thread, wp};
use intentsvcs::contract::Scope;
use intentsvcs::facade::{FacadeError, ListEdit};
use intentsvcs::model::{AtStatus, TShirt, ThreadStatus, WpStatus};
use intentsvcs::remedy::Remedy;

#[test]
fn st_new_creates_canon_and_every_view() {
  let fx = Fixture::new();
  let mut facade = fx.facade();

  let id = facade.st_new("Add a Rust-based CLI").expect("st new");
  // **THE ID IS THE CREATION EVENT'S, NOT A COUNT** (ST0095 WP-06): taken from
  // the mint and checked against the origin the thread carries, never predicted.
  let origin = facade
    .st_show(&id)
    .expect("show")
    .origin
    .clone()
    .expect("a minted thread carries its origin");
  assert!(
    facade.st_show(&id).expect("show").id.derives_from(&origin),
    "the new thread's id {id} is not the one its creation event mints"
  );

  assert!(fx.canon_path(&id).is_file());
  assert!(fx.project().info_view(&id).is_file());
  assert!(fx.project().acceptance_view(&id).is_file());
  assert!(fx.path("intent/st/steel_threads.md").is_file());
  assert!(fx.path("intent/todo.md").is_file());

  let thread = facade.st_show(&id).expect("show");
  assert_eq!(thread.status, ThreadStatus::Triage);
  // **The SHAPE, and there is nothing left to pin it to.** `created` is filled
  // by SQLite inside the INSERT and handed back (D42), so nothing in this
  // process ever held the value. This used to compare it against
  // `store.today()`, which was a tautology the day it was written -- the same
  // clock on both sides of an equals sign -- and is now not even expressible,
  // because the clock is gone. That the date agrees with the row's OWN record
  // stamp is proved in `record_timestamps.rs`, where the column is readable.
  assert!(
    thread.created.len() == 10 && thread.created.split('-').count() == 3,
    "the tool stamped an ISO date: {}",
    thread.created
  );
  assert_eq!(thread.slug.as_deref(), Some("add-a-rust-based-cli"));
}

/// **A NEW ID IS MINTED FROM ITS CREATION EVENT, WHATEVER IDS THE PROJECT
/// HOLDS** (ST0095 WP-06). This was "ids continue from the highest existing
/// thread": the arithmetic it pinned is retired, and the ordinal beside the new
/// thread is left exactly as it was.
#[test]
fn a_new_thread_is_minted_from_its_creation_event_whatever_ids_exist() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();
  let id = facade.st_new("next").expect("st new");
  let thread = facade.st_show(&id).expect("there");
  let origin = thread
    .origin
    .as_deref()
    .expect("a minted thread carries its origin");
  assert!(
    thread.id.derives_from(origin),
    "the new thread's id {id} is not the one its creation event mints"
  );
  assert!(
    facade.st_show("ST0056").is_ok(),
    "the ordinal the project held is still there"
  );
}

#[test]
fn the_lifecycle_moves_status_and_stamps_completion() {
  let fx = Fixture::new();
  let mut facade = fx.facade();
  let id = facade.st_new("a thread").expect("new");

  facade.st_triage(&id).expect("triage");
  facade.st_start(&id).expect("start");
  assert_eq!(facade.st_show(&id).unwrap().status, ThreadStatus::Wip);
  assert_eq!(
    facade.st_show(&id).unwrap().completed,
    None,
    "a thread in flight has no completion date"
  );

  facade
    .st_cancel(&id, "superseded by the v3 line")
    .expect("cancel");
  let thread = facade.st_show(&id).unwrap();
  assert_eq!(thread.status, ThreadStatus::Cancelled);
  let completed = thread
    .completed
    .as_deref()
    .expect("a close stamps completion");
  assert!(
    completed.len() == 10 && completed.split('-').count() == 3,
    "and the database wrote it as an ISO date: {completed}"
  );
}

/// Each new package's id is minted from its own creation event (ST0095 WP-06),
/// so the two are told apart by the ids the mint returned, never by position.
#[test]
fn work_packages_are_minted_and_move_through_their_states() {
  let fx = Fixture::new();
  let mut facade = fx.facade();
  let id = facade.st_new("a thread").expect("new");

  let first = facade.wp_new(&id, "first", TShirt::M).unwrap();
  let second = facade.wp_new(&id, "second", TShirt::L).unwrap();
  assert_ne!(first, second, "two packages were given one id");

  facade.wp_start(&id, second).expect("start");
  let wps = facade.wp_list(&id).expect("list");
  let listed = |seq| {
    wps
      .iter()
      .find(|w| w.seq == seq)
      .expect("the package is listed")
  };
  assert_eq!(listed(first).status, WpStatus::NotStarted);
  assert_eq!(listed(second).status, WpStatus::Wip);
  assert_eq!(listed(second).scope, Some(TShirt::L));
  for package in wps {
    let origin = package
      .origin
      .as_deref()
      .expect("a minted package carries its origin");
    assert!(
      package.seq.derives_from(origin),
      "a package's id is not its creation event's"
    );
  }
}

/// `st done` consults the close gate, and there is no path around it.
#[test]
fn closing_is_gated() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();

  // AC-03.1 is satisfied by a green AT; the non-test AC-03.2 is satisfied
  // inline. Turn the AT red and the thread must refuse to close.
  facade
    .at_set("ST0056", "AT-03.1", AtStatus::Red, None)
    .expect("at set");
  facade
    .at_set("ST0056", "AT-03.7", AtStatus::Red, None)
    .expect("at set");

  let err = facade.st_done("ST0056").expect_err("must refuse");
  match &err {
    FacadeError::GateBlocked { verdict, .. } => {
      assert!(verdict.contains("BLOCKED"), "got: {verdict}");
      assert!(
        verdict.contains("AC-03.1"),
        "names what is unsatisfied: {verdict}"
      );
    }
    other => panic!("expected GateBlocked, got: {other}"),
  }
  assert_eq!(
    facade.st_show("ST0056").unwrap().status,
    ThreadStatus::Wip,
    "a refused close changes nothing"
  );
}

/// **A refused close carries the remedy of the gate arm that refused it**
/// (issue 0526). One fixed remedy sat under every block, so closing a thread
/// with no criteria printed "satisfy or formally descope the remaining
/// criteria" beneath a gate line saying it had none.
#[test]
fn an_empty_contracts_refusal_carries_its_own_remedy() {
  let fx = Fixture::new();
  let mut facade = fx.facade();
  let id = facade.st_new("a thread with no criteria").expect("st new");
  facade.st_start(&id).expect("st start");

  let err = facade.st_done(&id).expect_err("an empty contract blocks");
  let FacadeError::GateBlocked {
    verdict, remedy, ..
  } = &err
  else {
    panic!("expected GateBlocked, got: {err}");
  };
  assert!(
    verdict.contains("zero acceptance criteria"),
    "got: {verdict}"
  );
  assert!(
    !remedy.contains("remaining criteria"),
    "the remedy speaks of criteria the thread does not have: {remedy}"
  );
  assert!(
    err.render().contains(&format!("remedy: {remedy}")),
    "the rendered refusal prints the arm's remedy: {}",
    err.render()
  );
}

#[test]
fn a_gate_pass_lets_the_thread_close() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();

  assert!(
    facade.gate("ST0056", Scope::Thread).unwrap().is_pass(),
    "precondition: the fixture contract is satisfied"
  );
  // The fixture leaves WP 3 at `wip`, and a thread does not close over an open
  // package (issue 0324), so it is settled through its own verb first.
  facade.wp_done("ST0056", wp(3)).expect("WP 3 settles");
  facade.st_done("ST0056").expect("done");
  assert_eq!(
    facade.st_show("ST0056").unwrap().status,
    ThreadStatus::Completed
  );
}

/// The torn-state clause. A real filesystem failure part-way through the batch.
#[cfg(unix)]
#[test]
fn a_mid_write_failure_leaves_no_torn_state() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();

  // One successful mutation first, so the views exist on disk and "before" is
  // a real state rather than an absence. Without this the test would be
  // asserting that nothing changed in files that were never there.
  facade
    .st_hold("ST0056", "waiting on the fleet")
    .expect("a legal mutation from wip");

  let canon_before = fx.read_canon("ST0056");
  // The cover's path from the project, never spelled by hand (ST0095 WP-06).
  let info = fx.project().info_view("ST0056");
  let info_before = std::fs::read_to_string(&info).expect("the cover");
  let index_before = fx.read("intent/st/steel_threads.md");
  let db_before = facade.store().derived_dump().expect("snapshot");

  // `intent/` read-only: writes into `intent/st/**` still succeed (they need
  // write permission on their OWN directory), but the rename of
  // `intent/todo.md` fails -- so the batch dies after several writes landed.
  let mode = fx.make_readonly("intent");

  let result = facade.st_cancel("ST0056", "superseded by the v3 line");

  fx.restore_mode("intent", mode);

  // Running as root defeats the injection, and `landed_note` says so rather
  // than passing.
  let outcome = result.expect("the write landed, so the verb reports it rather than refusing");
  let (step, cause, ..) = crate::common::landed_note(outcome.notes());
  // D01 REVERSED (hv, 2026-08-15): the DB is the SSOT, so by the time the file
  // write fails the mutation has ALREADY landed in truth. The note says so,
  // and since 0376 the verb exits clean with it (vc, ruled 2026-09-14): a
  // refusal sent the operator to retry a write that had landed.
  assert_eq!(
    step, "writing the views",
    "expected a projection failure, got: {cause}"
  );
  assert!(
    cause.contains("the change is recorded"),
    "the note leads with what succeeded: {cause}"
  );

  assert_eq!(
    fx.read_canon("ST0056"),
    canon_before,
    "the canon file is byte-identical -- the write landed and the batch unwound it"
  );
  assert_eq!(
    std::fs::read_to_string(&info).expect("the cover"),
    info_before,
    "the view that DID get written was restored, not left half-updated"
  );
  assert_eq!(
    fx.read("intent/st/steel_threads.md"),
    index_before,
    "the index too -- rollback is total, not best-effort on the file that failed"
  );
  // THE INVERSION. The DB is truth and it is written first, so it DID see the
  // mutation, and that is correct rather than torn: the files are the
  // re-creatable side and `intent sync` rewrites them from here.
  //
  // What AC-04.1 asks for survives intact on both stores, which is the point
  // worth keeping. The DB is all-or-nothing because entities, prose index and
  // envelope share one transaction; the FILES are all-or-nothing because
  // `WriteSet::commit` unwinds what it already wrote. So neither store is ever
  // half-applied -- the files are merely allowed to be STALE relative to
  // truth, which is what "re-creatable" means.
  assert_ne!(
    facade.store().derived_dump().expect("snapshot"),
    db_before,
    "the mutation landed in the store -- under D01 as reversed that is truth, not damage"
  );
  assert_eq!(
    facade.st_show("ST0056").unwrap().status,
    ThreadStatus::Cancelled,
    "and the in-memory canon agrees with the store, so the next call builds on what actually happened"
  );
}

/// A rolled-back batch leaves no scaffolding: no temp files, and no
/// directories created for a thread that was never written.
#[cfg(unix)]
#[test]
fn a_failed_creation_leaves_no_directory_behind() {
  let fx = Fixture::new();
  let mut facade = fx.facade();
  let mode = fx.make_readonly("intent");
  let result = facade.st_new_listing_reported("doomed", ListEdit::AsDeclared);
  fx.restore_mode("intent", mode);

  // Issue 0376: the store holds the thread, so the verb reports it with a note.
  let (id, notes) = result.expect("the write landed, so the verb reports it rather than refusing");
  crate::common::landed_note(&notes);
  assert!(
    !fx.project().thread_dir(&id).exists(),
    "no directory survives the write the batch unwound"
  );
  let strays: Vec<String> = walk(fx.root())
    .into_iter()
    .filter(|p| p.contains("intent-tmp"))
    .collect();
  assert!(strays.is_empty(), "temp files left behind: {strays:?}");
}

#[cfg(unix)]
fn walk(dir: &std::path::Path) -> Vec<String> {
  let mut out = Vec::new();
  let Ok(entries) = std::fs::read_dir(dir) else {
    return out;
  };
  for entry in entries.flatten() {
    let path = entry.path();
    if path.is_dir() {
      out.extend(walk(&path));
    } else {
      out.push(path.display().to_string());
    }
  }
  out
}

#[test]
fn an_unknown_thread_is_refused_by_name() {
  let fx = Fixture::new();
  let facade = fx.facade();
  match facade.st_show("ST9999") {
    Err(FacadeError::NoSuchThread { id }) => assert_eq!(id, "ST9999"),
    other => panic!("expected NoSuchThread, got: {other:?}"),
  }
}

/// The index tracks a mutation without anyone regenerating it by hand.
///
/// **This is a STALENESS check and not a vocabulary one, and saying so is the
/// point (issue 0047).** The `!contains("Not Started")` arm asserts the index no
/// longer shows the state the thread passed THROUGH, which is a real property. It
/// is not evidence that `Not Started` is spelled correctly: a negative assertion
/// about a string is satisfied by that string not existing, so any rename
/// satisfies it. vc measured exactly that -- renaming
/// `ThreadStatus::NotStarted`'s spelling reddened no test in the estate -- and
/// this assertion is one of the two that read as though it would have caught it.
/// The spellings are pinned positively in `status_vocabulary.rs`; what is checked
/// here is regeneration.
#[test]
fn views_are_regenerated_by_every_mutation() {
  let fx = Fixture::new();
  let mut facade = fx.facade();
  let id = facade.st_new("a thread").expect("new");
  let before = fx.read("intent/st/steel_threads.md");
  assert!(before.contains("Triage"));

  facade.st_triage(&id).expect("triage");
  facade.st_start(&id).expect("start");
  let after = fx.read("intent/st/steel_threads.md");
  assert!(
    after.contains("WIP") && !after.contains("Not Started"),
    "the index reflects the mutation without anyone regenerating it by hand"
  );
}

/// **A SECOND mutation on a thread carrying attachments must replace them, not
/// duplicate them.**
///
/// The defect this pins, which I shipped and the suite caught: `attachments`
/// was wired into the write, the wholesale rebuild and the read, and NOT into
/// either of the two per-thread clears that every other child collection has.
/// So the first mutation succeeded and the second died on
/// `UNIQUE constraint failed: attachments.thread_id, attachments.path`.
///
/// **A new child collection has FIVE sites, not three**, and the two easy to
/// miss are the ones spelled the same way four times in a row -- a shape the
/// eye reads as one thing. Grep for the rule, do not read for it.
#[test]
fn a_thread_carrying_attachments_survives_being_mutated_twice() {
  let fx = Fixture::new();
  let mut thread = sample_thread("ST0056");
  thread.attachments = vec![intentsvcs::model::Attachment::new(
    "reference.md",
    "# Reference\n",
  )];
  fx.write_thread(&thread);

  let mut f = fx.facade();
  f.wp_start("ST0056", wp(3)).expect("first mutation");
  f.wp_done("ST0056", wp(3))
    .expect("second mutation on the same thread");

  let (threads, _) = f.store().load_canon().expect("read back");
  let carried = &threads
    .iter()
    .find(|t| t.id == "ST0056")
    .unwrap()
    .attachments;
  assert_eq!(
    carried.len(),
    1,
    "one attachment in, one out -- not two, and not none: {carried:?}"
  );
  assert_eq!(carried[0].path, "reference.md");
}
