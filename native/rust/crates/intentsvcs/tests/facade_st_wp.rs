//! AT-04.1 / AC-04.1: st/wp lifecycle verbs run through the facade with canon,
//! views and DB written transactionally -- a failure mid-write leaves no torn
//! state.
//!
//! The torn-state clause is INJECTED, not reasoned about. A read-only
//! directory makes a real `rename` fail the way the operating system fails it,
//! part-way through a batch that has already written other files. A synthetic
//! seam the production code knows about would prove the seam works.

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::contract::Scope;
use intentsvcs::facade::FacadeError;
use intentsvcs::model::{AtStatus, TShirt, ThreadStatus, WpStatus};

#[test]
fn st_new_creates_canon_and_every_view() {
  let fx = Fixture::new();
  let mut facade = fx.facade();

  let id = facade.st_new("Add a Rust-based CLI").expect("st new");
  assert_eq!(id, "ST0001", "the first thread in an empty project");

  assert!(fx.path("intent/st/ST0001/thread.json").is_file());
  assert!(fx.path("intent/st/ST0001/info.md").is_file());
  assert!(fx.path("intent/st/ST0001/acceptance.md").is_file());
  assert!(fx.path("intent/st/steel_threads.md").is_file());
  assert!(fx.path("intent/todo.md").is_file());

  let thread = facade.st_show("ST0001").expect("show");
  assert_eq!(thread.status, ThreadStatus::Triage);
  assert_eq!(
    thread.created, "2026-08-14",
    "the date comes from the caller"
  );
  assert_eq!(thread.slug.as_deref(), Some("add-a-rust-based-cli"));
}

#[test]
fn ids_continue_from_the_highest_existing_thread() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();
  assert_eq!(facade.st_new("next").expect("st new"), "ST0057");
}

#[test]
fn the_lifecycle_moves_status_and_stamps_completion() {
  let fx = Fixture::new();
  let mut facade = fx.facade();
  facade.st_new("a thread").expect("new");

  facade.st_triage("ST0001").expect("triage");
  facade.st_start("ST0001").expect("start");
  assert_eq!(facade.st_show("ST0001").unwrap().status, ThreadStatus::Wip);
  assert_eq!(
    facade.st_show("ST0001").unwrap().completed,
    None,
    "a thread in flight has no completion date"
  );

  facade
    .st_cancel("ST0001", "superseded by the v3 line")
    .expect("cancel");
  let thread = facade.st_show("ST0001").unwrap();
  assert_eq!(thread.status, ThreadStatus::Cancelled);
  assert_eq!(thread.completed.as_deref(), Some("2026-08-14"));
}

#[test]
fn work_packages_number_from_one_and_move_through_their_states() {
  let fx = Fixture::new();
  let mut facade = fx.facade();
  facade.st_new("a thread").expect("new");

  assert_eq!(facade.wp_new("ST0001", "first", TShirt::M).unwrap(), 1);
  assert_eq!(facade.wp_new("ST0001", "second", TShirt::L).unwrap(), 2);

  facade.wp_start("ST0001", 2).expect("start");
  let wps = facade.wp_list("ST0001").expect("list");
  assert_eq!(wps[0].status, WpStatus::NotStarted);
  assert_eq!(wps[1].status, WpStatus::Wip);
  assert_eq!(wps[1].scope, TShirt::L);
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
    .at_set("ST0056", "AT-03.1", AtStatus::Red)
    .expect("at set");
  facade
    .at_set("ST0056", "AT-03.7", AtStatus::Red)
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

#[test]
fn a_gate_pass_lets_the_thread_close() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();

  assert!(
    facade.gate("ST0056", Scope::Thread).unwrap().is_pass(),
    "precondition: the fixture contract is satisfied"
  );
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

  let canon_before = fx.read("intent/st/ST0056/thread.json");
  let info_before = fx.read("intent/st/ST0056/info.md");
  let index_before = fx.read("intent/st/steel_threads.md");
  let db_before = facade.store().snapshot().expect("snapshot");

  // `intent/` read-only: writes into `intent/st/**` still succeed (they need
  // write permission on their OWN directory), but the rename of
  // `intent/todo.md` fails -- so the batch dies after several writes landed.
  let mode = fx.make_readonly("intent");

  let result = facade.st_cancel("ST0056", "superseded by the v3 line");

  fx.restore_mode("intent", mode);

  let Err(err) = result else {
    // Running as root defeats the injection. Say so rather than passing.
    panic!(
      "the write into a read-only directory SUCCEEDED -- the failure was not injected, so this test proved nothing (running as root?)"
    );
  };
  // D01 REVERSED (hv, 2026-08-15): the DB is the SSOT, so by the time the file
  // write fails the mutation has ALREADY landed in truth. The variant says so
  // -- `Write` would tell the operator the mutation failed, which is now the
  // opposite of what happened, and a retry is the hazard.
  assert!(
    matches!(err, FacadeError::ViewsNotWritten { .. }),
    "expected a projection failure, got: {err}"
  );

  assert_eq!(
    fx.read("intent/st/ST0056/thread.json"),
    canon_before,
    "the canon file is byte-identical -- the write landed and the batch unwound it"
  );
  assert_eq!(
    fx.read("intent/st/ST0056/info.md"),
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
    facade.store().snapshot().expect("snapshot"),
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
  let result = facade.st_new("doomed");
  fx.restore_mode("intent", mode);

  assert!(result.is_err(), "precondition: the write failed");
  assert!(
    !fx.path("intent/st/ST0001").exists(),
    "no directory survives a thread that was never created"
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

#[test]
fn views_are_regenerated_by_every_mutation() {
  let fx = Fixture::new();
  let mut facade = fx.facade();
  facade.st_new("a thread").expect("new");
  let before = fx.read("intent/st/steel_threads.md");
  assert!(before.contains("Triage"));

  facade.st_triage("ST0001").expect("triage");
  facade.st_start("ST0001").expect("start");
  let after = fx.read("intent/st/steel_threads.md");
  assert!(
    after.contains("WIP") && !after.contains("Not Started"),
    "the index reflects the mutation without anyone regenerating it by hand"
  );
}
