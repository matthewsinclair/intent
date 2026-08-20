//! AT-09.1 / AC-09.1 -- WP-09: **every act that changes the DISK is recorded,
//! and no act that leaves it alone is.**
//!
//! `Facade::apply` is the door for MODEL mutation and everything reaching it is
//! logged. The realisation verbs do not pass it -- deliberately, because `apply`
//! diffs canon and they change no canon at all -- so until this landed they
//! wrote the filesystem in silence.
//!
//! **The gap was measured rather than supposed.** On 2026-08-19 `organize
//! --apply` removed 423 files from this estate and the log recorded nothing;
//! its 55 events at that moment were every one of them a model mutation. The
//! only act all evening that destroyed anything was the only class of act
//! absent from the one table that cannot be re-derived from anything else on
//! disk.
//!
//! **BOTH POLARITIES ARE DRIVEN, and the silent one is the half that is easy to
//! get wrong.** A verb that emitted on every call would pass a test that only
//! asked "is the act recorded" -- and would fill the one unrecoverable table
//! with records of acts that did not happen, which is how a log stops meaning
//! anything. So each arm below pairs a real act with a call of the same verb
//! that changes nothing.

mod common;

use common::{Fixture, gate_open, sample_thread};
use intentsvcs::address::{Address, Entity};
use intentsvcs::organize;

const MANIFEST: &str = "\
# .intentfiles

# BEGIN INTENT
# END INTENT
";

fn fixture() -> Fixture {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  fx.write_file("intent/.intentfiles", MANIFEST);
  fx
}

fn disk_ops(facade: &intentsvcs::facade::Facade) -> Vec<String> {
  facade
    .store()
    .events()
    .expect("events")
    .into_iter()
    .filter(|e| e.op.starts_with("disk."))
    .map(|e| e.op)
    .collect()
}

fn last_disk_payload(facade: &intentsvcs::facade::Facade, op: &str) -> serde_json::Value {
  facade
    .store()
    .events()
    .expect("events")
    .into_iter()
    .filter(|e| e.op == op)
    .next_back()
    .unwrap_or_else(|| panic!("no {op} event"))
    .payload
}

/// **A PREVIEW DECIDES EVERYTHING AND TOUCHES NOTHING, so it records nothing.**
///
/// The two modes exist to keep exactly this distinction, and an event for a
/// preview would put a decision into the record of acts.
#[test]
fn a_preview_records_nothing_and_an_apply_that_moves_nothing_records_nothing() {
  let fx = fixture();
  let mut facade = fx.facade_on_disk();

  facade.organize(organize::Mode::Preview).expect("preview");
  assert!(
    disk_ops(&facade).is_empty(),
    "a preview changed no file, so it is not an act: {:?}",
    disk_ops(&facade)
  );

  // An apply over a tree that already agrees with the manifest is equally not
  // an act. This is the arm that fails on a verb which emits unconditionally.
  facade.organize(organize::Mode::Apply).expect("apply");
  facade
    .organize(organize::Mode::Apply)
    .expect("second apply");
  assert!(
    disk_ops(&facade).is_empty(),
    "an apply that moved nothing is not an act either: {:?}",
    disk_ops(&facade)
  );
}

/// **A REAL HYDRATION IS RECORDED, AND A SECOND ONE IS NOT.**
///
/// `hydrate` is idempotent and its return value is deliberately *paths that now
/// exist* rather than *paths this run wrote* -- so a naive emission keyed on the
/// return value records an act every time anybody hydrates an already-realised
/// artefact. The second call here is what catches that.
#[test]
fn hydrating_is_recorded_once_and_the_paths_are_named() {
  let fx = fixture();
  let mut facade = fx.facade_on_disk();

  let address = Address {
    authority: None,
    entity: Entity::Thread {
      id: "ST0001".to_string(),
    },
    format: None,
  };
  facade.hydrate(&address).expect("hydrate");

  assert_eq!(
    disk_ops(&facade),
    vec!["disk.hydrate".to_string()],
    "the act is recorded exactly once"
  );

  let payload = last_disk_payload(&facade, "disk.hydrate");
  assert_eq!(payload["id"], "ST0001", "the artefact is named");
  assert_eq!(payload["pinned"], true, "and the pin moved");
  assert!(
    !payload["hydrated"].as_array().expect("hydrated").is_empty(),
    "THE PATH SET IS THE SUBJECT -- an event naming no path answers nothing: {payload}"
  );

  // Idempotent: nothing to pin, nothing to write, nothing to say.
  facade.hydrate(&address).expect("second hydrate");
  assert_eq!(
    disk_ops(&facade).len(),
    1,
    "hydrating an already-hydrated artefact changed no file: {:?}",
    disk_ops(&facade)
  );
}

/// **THE 423-FILE CASE, IN MINIATURE: A REMOVAL IS RECORDED AND NAMES WHAT IT
/// REMOVED.**
///
/// This is the arm the whole work package exists for. A reader asking *what
/// happened to ST0001* previously got `st.new` and nothing since, while the
/// thread's files had left the tree an hour earlier.
#[test]
fn dehydrating_is_recorded_and_names_every_file_it_removed() {
  // **The ship gate is REAL here and is not the subject.** `gate_open` builds a
  // thread whose AC-00.1 carries a genuine precondition declaration with its one
  // entry satisfied -- nothing bypasses the gate, because a helper that handed
  // out a permitting verdict would make it enforced by everyone remembering not
  // to call it. Without this the fixture removes nothing and the test proves
  // nothing, which is what the assertion below exists to catch.
  let fx = Fixture::new();
  fx.write_thread(&gate_open());
  fx.write_file("intent/.intentfiles", MANIFEST);
  let mut facade = fx.facade_on_disk();

  let address = Address {
    authority: None,
    entity: Entity::Thread {
      id: "ST0057".to_string(),
    },
    format: None,
  };
  facade.hydrate(&address).expect("hydrate");

  // Undeclare it, exactly as an operator editing the list would.
  fx.write_file("intent/.intentfiles", MANIFEST);
  let report = facade.organize(organize::Mode::Apply).expect("apply");
  assert!(
    !report.dehydrated.is_empty(),
    "the fixture must actually remove something or this test proves nothing -- \
     dehydrated={:?} refused={:?} unchanged={:?} unclaimed={:?}",
    report.dehydrated,
    report.refused,
    report.unchanged.len(),
    report.unclaimed.len()
  );

  let payload = last_disk_payload(&facade, "disk.organize");
  let recorded = payload["dehydrated"].as_array().expect("dehydrated").len();
  assert_eq!(
    recorded,
    report.dehydrated.len(),
    "EVERY removed file is in the record, not a count standing in for them: {payload}"
  );
  assert!(
    payload["dehydrated"]
      .as_array()
      .expect("dehydrated")
      .iter()
      .any(|p| p.as_str().is_some_and(|s| s.contains("ST0057"))),
    "and they are named rather than counted: {payload}"
  );
}

/// **THE TWO SYNC VERBS ARE THE OTHER HALF OF AC-09.1, AND THE FIRST THING
/// THEY NEEDED WAS A DEFINITION OF "ACT" THAT TERMINATES.**
///
/// `sync_to_disk` projects the event log as part of its own write set. So a
/// naive reading -- *this run wrote a file, therefore it acted* -- makes the
/// verb record itself, which leaves the file one event behind, which gives the
/// NEXT sync a real write to do, which records itself. **Each sync creates the
/// work for the next one and the log grows by one event per sync with nobody
/// touching the estate.** Reasoned first and then driven, because a regress
/// that needs two calls to appear is invisible to any arm that makes one.
///
/// The line that terminates it: **the event log is not part of the estate the
/// log describes.** A run that wrote only the log has not acted on anything --
/// it is the record catching up with itself.
#[test]
fn consecutive_syncs_over_an_unchanged_estate_converge() {
  let fx = fixture();
  let mut facade = fx.facade_on_disk();

  facade
    .sync_to_disk(&intentsvcs::sync::Scope::All)
    .expect("first sync");
  let after_first = disk_ops(&facade).len();

  facade
    .sync_to_disk(&intentsvcs::sync::Scope::All)
    .expect("second sync");
  let after_second = disk_ops(&facade).len();

  facade
    .sync_to_disk(&intentsvcs::sync::Scope::All)
    .expect("third sync");
  let after_third = disk_ops(&facade).len();

  assert_eq!(
    after_second, after_first,
    "the second sync over an unchanged estate wrote only the event log and must be silent; \
     recording it stales the file again and the third sync inherits the work"
  );
  assert_eq!(
    after_third, after_second,
    "and it must STAY silent -- a regress that only shows on the third call is the one a \
     two-call test misses"
  );
}

/// **A SYNC THAT WRITES REAL FILES IS RECORDED AND NAMES THEM.**
///
/// The positive control for the arm above: without it, "silent on an unchanged
/// estate" is satisfied by a verb that is silent always, which is the state
/// this work package exists to end.
#[test]
fn syncing_to_disk_records_the_files_it_actually_wrote() {
  // **DECLARED, or the assertion below is checking the wrong thing.** Under the
  // disk model an undeclared thread has no realised form, so a sync over the
  // default fixture correctly writes `steel_threads.md`, `todo.md` and the log
  // and nothing of ST0001 at all. The first version of this arm asserted the
  // thread was named and failed -- **on the code being right**, which is the
  // only kind of red worth having early.
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  fx.write_file(
    "intent/.intentfiles",
    "# .intentfiles\n\n# BEGIN INTENT\nSTEELTHREAD:ST0001\n# END INTENT\n",
  );
  let mut facade = fx.facade_on_disk();

  facade
    .sync_to_disk(&intentsvcs::sync::Scope::All)
    .expect("sync");

  let ops = disk_ops(&facade);
  assert!(
    ops.contains(&"disk.sync_to_disk".to_string()),
    "a sync that wrote the estate's views for the first time is an act; ops were {ops:?}"
  );

  let payload = last_disk_payload(&facade, "disk.sync_to_disk");
  let wrote = payload["wrote"].as_array().expect("wrote");
  assert!(
    !wrote.is_empty(),
    "the act names the PATHS it wrote, not a count of them"
  );
  assert!(
    wrote
      .iter()
      .any(|p| p.as_str().is_some_and(|s| s.contains("ST0001"))),
    "the thread whose views were written is named in the record; got {wrote:?}"
  );
}
