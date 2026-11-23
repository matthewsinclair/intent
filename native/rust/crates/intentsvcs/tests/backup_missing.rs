//! Issue I4da7c22: a snapshot whose file is gone keeps its row, and nothing
//! said so. `--list` marks the row, `doctor` reports it as an advisory that
//! never moves the verdict, and `prune_missing` drops the row.
//!
//! **The discriminating case is the verdict.** Every attendee repo upgraded to
//! 3.5.1 must still get doctor 0 with no action, so the advisory is proved
//! against the same project's own report before the file went, not against an
//! exit code written down here.

use crate::common::{Fixture, ctx};
use intentsvcs::backup;
use intentsvcs::doctor::{Report, Scope, diagnose};
use intentsvcs::finding::FindingClass;
use intentsvcs::store::Store;
use rusqlite::Connection;

fn store_of(fx: &Fixture) -> Store {
  Store::open(&fx.project().db_path()).expect("open the store")
}

/// A good snapshot row with a chosen stamp, and its file. Returns the file.
fn recorded(fx: &Fixture, stamp: &str) -> std::path::PathBuf {
  let dir = backup::snapshot_dir(&fx.project());
  std::fs::create_dir_all(&dir).expect("snapshot dir");
  let path = dir.join(format!("{}.db", stamp.replace([':', '.'], "-")));
  std::fs::write(&path, b"not a real database, and nothing here opens it").expect("write");
  let rel = fx.project().relative(&path);
  Connection::open(fx.project().db_path())
    .expect("open the store directly")
    .execute(
      "INSERT INTO snapshots (path, bytes, outcome, taken_at) VALUES (?1, 1, 'ok', ?2)",
      rusqlite::params![rel, stamp],
    )
    .expect("insert a snapshot row");
  path
}

/// A failed attempt: a row with no file, which is never missing.
fn failed(fx: &Fixture, stamp: &str) {
  Connection::open(fx.project().db_path())
    .expect("open the store directly")
    .execute(
      "INSERT INTO snapshots (outcome, detail, taken_at) VALUES ('failed', 'disk full', ?1)",
      rusqlite::params![stamp],
    )
    .expect("insert a failed attempt");
}

fn report(fx: &Fixture, store: &Store) -> Report {
  diagnose(&fx.project(), &ctx(), Some(store), Scope::All)
}

fn missing_advisories(report: &Report) -> Vec<String> {
  report
    .findings
    .iter()
    .filter(|f| f.class == FindingClass::Advisory && f.detail.contains("missing from disk"))
    .map(|f| f.detail.clone())
    .collect()
}

#[test]
fn a_row_whose_file_is_gone_is_missing_and_pruning_drops_only_it() {
  let fx = Fixture::new();
  let store = store_of(&fx);
  let kept = recorded(&fx, "2026-01-01T00:00:00.000Z");
  let gone = recorded(&fx, "2026-02-01T00:00:00.000Z");
  failed(&fx, "2026-03-01T00:00:00.000Z");
  std::fs::remove_file(&gone).expect("remove by hand, as a prune script did");

  let missing = backup::missing(&fx.project(), &store).expect("read");
  assert_eq!(
    missing
      .iter()
      .map(|s| s.taken_at.as_str())
      .collect::<Vec<_>>(),
    ["2026-02-01T00:00:00.000Z"],
    "only the row whose file went is missing; the live one and the failed attempt are not"
  );

  let dropped = backup::prune_missing(&fx.project(), &store).expect("prune");
  assert_eq!(
    dropped
      .iter()
      .map(|s| s.taken_at.as_str())
      .collect::<Vec<_>>(),
    ["2026-02-01T00:00:00.000Z"],
    "it returns what it dropped, so the verb can print it"
  );
  let left: Vec<(String, String)> = store
    .snapshots()
    .expect("read")
    .into_iter()
    .map(|s| (s.taken_at, s.outcome))
    .collect();
  assert_eq!(
    left,
    [
      ("2026-03-01T00:00:00.000Z".to_string(), "failed".to_string()),
      ("2026-01-01T00:00:00.000Z".to_string(), "ok".to_string()),
    ],
    "the live row and the failure record stay"
  );
  assert!(
    kept.is_file(),
    "and no file was touched: {}",
    kept.display()
  );
  assert!(
    backup::prune_missing(&fx.project(), &store)
      .expect("prune again")
      .is_empty(),
    "a second run finds nothing to drop"
  );
}

#[test]
fn doctor_advises_on_a_missing_row_and_the_verdict_does_not_move() {
  let fx = Fixture::new();
  let store = store_of(&fx);
  backup::take(&fx.project(), &store).expect("a fresh snapshot, so nothing is stale");
  let gone = recorded(&fx, "2026-02-01T00:00:00.000Z");
  let before = report(&fx, &store);
  assert!(
    missing_advisories(&before).is_empty(),
    "a registry whose every row has its file says nothing: {:?}",
    missing_advisories(&before)
  );

  std::fs::remove_file(&gone).expect("remove by hand");
  let after = report(&fx, &store);

  let advisories = missing_advisories(&after);
  assert_eq!(advisories.len(), 1, "one advisory, got {advisories:?}");
  assert!(
    advisories[0].contains("2026-02-01T00:00:00.000Z")
      && advisories[0].contains("intent backup --prune-missing"),
    "it names the newest missing row and the verb that drops it: {}",
    advisories[0]
  );
  assert_eq!(
    (after.exit_code(), after.actionable()),
    (before.exit_code(), before.actionable()),
    "the advisory is uncounted, so the exit and the count are what they were before the file went"
  );
}
