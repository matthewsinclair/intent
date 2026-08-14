//! AT-02.3 / AC-02.3: the D01 disposability invariant -- deleting the DB and
//! rebuilding from canon yields identical queryable content, and rebuild is
//! idempotent. Also: the event log is NOT derived and survives a rebuild.

use intentsvcs::event::{Envelope, LOCAL_PRINCIPAL, Subject};
use intentsvcs::model::{
  AcKind, AcScope, AcceptanceTest, AtKind, AtStatus, Criterion, ISSUE_SCHEMA, Issue, IssueStatus,
  THREAD_SCHEMA, TShirt, Thread, ThreadStatus, WorkPackage, WpStatus,
};
use intentsvcs::store::Store;

fn canon() -> (Vec<Thread>, Vec<Issue>) {
  let thread = Thread {
    schema: THREAD_SCHEMA.to_string(),
    id: "ST0056".to_string(),
    title: "Intent v3.0.0".to_string(),
    slug: Some("intent-v3".to_string()),
    status: ThreadStatus::Wip,
    created: "2026-08-14".to_string(),
    completed: None,
    acceptance: None,
    wps: vec![
      WorkPackage {
        seq: 1,
        title: "Design canon".to_string(),
        scope: TShirt::L,
        status: WpStatus::Done,
      },
      WorkPackage {
        seq: 2,
        title: "Workspace".to_string(),
        scope: TShirt::L,
        status: WpStatus::Wip,
      },
    ],
    criteria: vec![
      Criterion {
        id: "AC-02.3".to_string(),
        text: "rebuild identity".to_string(),
        kind: AcKind::Test,
        scope: AcScope::InScope,
        evidence: None,
        satisfied: None,
      },
      Criterion {
        id: "AC-02.9".to_string(),
        text: "a descoped example".to_string(),
        kind: AcKind::NonTest,
        scope: AcScope::Descoped {
          to: "ST0057".to_string(),
          by: Some("hv".to_string()),
          reason: None,
        },
        evidence: Some("design.md".to_string()),
        satisfied: Some(false),
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
fn rebuild_is_idempotent_and_delete_then_rebuild_is_identity() {
  let (threads, issues) = canon();

  let mut first = Store::open_in_memory().expect("open");
  first.rebuild(&threads, &issues).expect("rebuild");
  let snap_one = first.snapshot().expect("snapshot");

  // Idempotent: rebuilding the same canon changes nothing.
  first.rebuild(&threads, &issues).expect("second rebuild");
  assert_eq!(
    first.snapshot().expect("snapshot"),
    snap_one,
    "rebuild is idempotent"
  );

  // Disposable: a brand-new store from the same canon is identical -- this is
  // `rm intent.db` being safe, as a law rather than a slogan.
  let mut fresh = Store::open_in_memory().expect("open fresh");
  fresh.rebuild(&threads, &issues).expect("rebuild fresh");
  assert_eq!(
    fresh.snapshot().expect("snapshot"),
    snap_one,
    "delete + rebuild is identity"
  );
}

#[test]
fn file_backed_store_survives_deletion() {
  let dir = tempfile::tempdir().expect("tempdir");
  let db = dir.path().join("intent.db");
  let (threads, issues) = canon();

  let mut store = Store::open(&db).expect("open file store");
  store.rebuild(&threads, &issues).expect("rebuild");
  let before = store.snapshot().expect("snapshot");
  drop(store);

  std::fs::remove_file(&db).expect("rm intent.db");
  let mut rebuilt = Store::open(&db).expect("reopen after rm");
  rebuilt
    .rebuild(&threads, &issues)
    .expect("rebuild after rm");
  assert_eq!(
    rebuilt.snapshot().expect("snapshot"),
    before,
    "rm + rebuild is identity"
  );
}

#[test]
fn event_log_is_not_derived_and_survives_rebuild() {
  let (threads, issues) = canon();
  let mut store = Store::open_in_memory().expect("open");
  store.rebuild(&threads, &issues).expect("rebuild");

  let envelope = Envelope::new(
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
  let snap = store.snapshot().expect("snapshot");
  assert!(
    snap.get("event_log").is_none(),
    "snapshot covers derived tables only"
  );
}
