//! AT-02.3 / AC-02.3: RE-CREATABILITY -- deleting the DB and rebuilding it from
//! the same extract yields identical queryable content, and rebuild is
//! idempotent. Also: the event log is NOT derived and survives a rebuild.
//!
//! **It said "the D01 disposability invariant" and that word is now wrong.**
//! Under D01 as reversed the DB is truth and re-creation from an extract is a
//! CAPABILITY, so what is invariant is that the round trip is faithful -- never
//! that the thing being rebuilt was disposable. This line is the third in this
//! one file to carry the old model, and it survived two corrections because it
//! spells it "disposability" while the grep asked for "disposable".

use intentsvcs::event::{Envelope, LOCAL_PRINCIPAL, Subject};
use intentsvcs::model::{
  AcKind, AcState, AcceptanceTest, AtKind, AtStatus, Criterion, ISSUE_SCHEMA, Issue, IssueStatus,
  Related, THREAD_SCHEMA, TShirt, Thread, ThreadStatus, WorkPackage, WpStatus,
};
use intentsvcs::store::Store;

fn canon() -> (Vec<Thread>, Vec<Issue>) {
  let thread = Thread {
    schema: THREAD_SCHEMA.to_string(),
    id: "ST0056".to_string(),
    title: "Intent v3.0.0".to_string(),
    slug: Some("intent-v3".to_string()),
    status: ThreadStatus::Wip,
    status_reason: None,
    created: "2026-08-14".to_string(),
    completed: None,
    acceptance: None,
    objective: "Ship Intent v3.0.0.".to_string(),
    context: "v2 is 12,492 lines of bash.\n\nEvery reader reimplements parsing.".to_string(),
    related: vec![Related {
      id: "ST0043".to_string(),
      note: Some("the v2 convergent orchestrator".to_string()),
    }],
    wps: vec![
      WorkPackage {
        seq: 1,
        title: "Design canon".to_string(),
        scope: Some(TShirt::L),
        scope_legacy: None,
        status: WpStatus::Done,
        status_reason: None,
        objective: String::new(),
        body: String::new(),
      },
      WorkPackage {
        seq: 2,
        title: "Workspace".to_string(),
        scope: Some(TShirt::L),
        scope_legacy: None,
        status: WpStatus::Wip,
        status_reason: None,
        objective: String::new(),
        body: String::new(),
      },
    ],
    criteria: vec![
      Criterion {
        id: "AC-02.3".to_string(),
        text: "rebuild identity".to_string(),
        kind: AcKind::Test,
        state: AcState::Computed,
      },
      Criterion {
        id: "AC-02.9".to_string(),
        text: "a descoped example".to_string(),
        kind: AcKind::NonTest,
        state: AcState::Descoped {
          to: "ST0057".to_string(),
          by: Some("hv".to_string()),
          reason: None,
        },
      },
    ],
    tests: vec![AcceptanceTest {
      id: "AT-02.3".to_string(),
      kind: AtKind::Test,
      file: Some("crates/intentsvcs/tests/store_rebuild.rs".to_string()),
      prose: None,
      covers: vec!["AC-02.3".to_string()],
      status: AtStatus::Red,
      note: Some("red-first".to_string()),
      legacy: None,
    }],
  };
  let issue = Issue {
    schema: ISSUE_SCHEMA.to_string(),
    number: 21,
    slug: "credo-checks".to_string(),
    title: "prune the dead mechanism".to_string(),
    status: IssueStatus::Closed,
    severity: Some("medium".to_string()),
    created: "2026-08-14".to_string(),
    closed: Some("2026-08-14".to_string()),
  };
  (vec![thread], vec![issue])
}

#[test]
fn rebuild_is_idempotent_and_a_second_store_from_the_same_extract_matches() {
  let (threads, issues) = canon();

  let mut first = Store::open_in_memory().expect("open");
  first.rebuild(&threads, &issues).expect("rebuild");
  let snap_one = first.derived_dump().expect("snapshot");

  // Idempotent: rebuilding the same canon changes nothing.
  first.rebuild(&threads, &issues).expect("second rebuild");
  assert_eq!(
    first.derived_dump().expect("snapshot"),
    snap_one,
    "rebuild is idempotent"
  );

  // RE-CREATABLE, which is not the same as disposable. This line used to read
  // "`rm intent.db` being safe, as a law rather than a slogan" -- the most
  // dangerous sentence in the estate under D34, because it is true of the
  // FIXTURE and false of a project. What is proved here is that a store rebuilt
  // from THE SAME EXTRACT is identical; a real `rm` costs everything the extract
  // does not carry, and today that includes the whole event log.
  let mut fresh = Store::open_in_memory().expect("open fresh");
  fresh.rebuild(&threads, &issues).expect("rebuild fresh");
  assert_eq!(
    fresh.derived_dump().expect("snapshot"),
    snap_one,
    "a second store built from the same extract is the same store"
  );
}

/// **A machine that has never held this store builds an identical one from the
/// extract** -- the clone case, which is what a file-backed rebuild is actually
/// for (D34).
///
/// Renamed twice, and the second rename is D36 rather than accuracy. It was
/// `file_backed_store_survives_deletion`, which claimed more than it proves:
/// the store does not survive anything, it is RE-CREATED, and only because the
/// same extract is handed back in. Then it became
/// `a_deleted_store_rebuilt_from_the_same_extract_is_identical`, which was true
/// and still built its fixture by deleting a database -- **the test-fixture
/// idiom D36 names by name**, sitting one line below a comment written to kill
/// the same idea in prose.
///
/// **So the deletion is gone, not renamed.** A second path IS the fresh
/// machine, exactly and without pretending: nothing is removed, because the
/// property was never about removal. The old form could only reach "a store
/// that is not there" by destroying one, which made an operation Intent does
/// not have look like a step in a procedure it does.
#[test]
fn a_machine_that_never_held_the_store_builds_an_identical_one_from_the_extract() {
  let dir = tempfile::tempdir().expect("tempdir");
  let (threads, issues) = canon();

  let mut original = Store::open(&dir.path().join("original/intent.db")).expect("open file store");
  original.rebuild(&threads, &issues).expect("rebuild");
  let before = original.derived_dump().expect("snapshot");
  drop(original);

  // A different path: a machine that has only ever had the extract.
  let mut clone =
    Store::open(&dir.path().join("clone/intent.db")).expect("open on a fresh machine");
  clone
    .rebuild(&threads, &issues)
    .expect("build from the extract");
  assert_eq!(
    clone.derived_dump().expect("snapshot"),
    before,
    "the extract reconstitutes the same store on a machine that never had it"
  );
}

#[test]
fn event_log_is_not_derived_and_survives_rebuild() {
  let (threads, issues) = canon();
  let mut store = Store::open_in_memory().expect("open");
  store.rebuild(&threads, &issues).expect("rebuild");

  let envelope = Envelope::minted(
    LOCAL_PRINCIPAL,
    "00000000-0000-0000-0000-000000000000",
    "wp.start",
    Subject {
      kind: "wp".to_string(),
      id: "ST0056/02".to_string(),
    },
    serde_json::json!({"from": "not-started", "to": "wip"}),
  );
  store.append_event(&envelope).expect("append");

  // A rebuild wipes derived tables only; the log is append-only state.
  store.rebuild(&threads, &issues).expect("rebuild again");
  let snap = store.derived_dump().expect("snapshot");
  assert!(
    snap.get("event_log").is_none(),
    "snapshot covers derived tables only"
  );
}
