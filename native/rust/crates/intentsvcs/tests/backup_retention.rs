//! AC-03.10 (c): rolling retention, in its own namespace, that cannot reach
//! `intent upgrade`'s rollback artefacts.
//!
//! **The discriminating case is not "does it delete old snapshots".** Any
//! plausible pruner does that. It is whether the pruner can reach the OTHER
//! mechanism that writes under `.backup/`: `intent upgrade` leaves
//! `backup-<TIMESTAMP>/` rollback directories there, with different retention
//! rules and a different owner, and **deleting one of those is the loss this
//! criterion exists to prevent.** A pruner that globbed `.backup/` would pass
//! every test about snapshot counts and still be the defect.
//!
//! **Historical stamps are authored directly into the table here, and that is
//! the RESTORE door rather than a confection.** Retention is a question about
//! calendar buckets, so it cannot be exercised without rows from other days --
//! and there is deliberately no API that takes a time, so a fixture reaches
//! past it to SQLite exactly as the migration fixtures do. Carrying a recorded
//! stamp is preserving history; asking what time it is would be the violation,
//! and nothing here asks.

mod common;

use common::Fixture;
use intentsvcs::backup::{self, Retention};
use intentsvcs::store::Store;
use rusqlite::Connection;

/// Write a successful snapshot row with a chosen stamp, and the file it points
/// at. Returns the file's path.
fn historical(fx: &Fixture, stamp: &str) -> std::path::PathBuf {
  let dir = backup::snapshot_dir(&fx.project());
  std::fs::create_dir_all(&dir).expect("snapshot dir");
  let name = format!("{}.db", stamp.replace([':', '.'], "-"));
  let path = dir.join(&name);
  std::fs::write(&path, b"not a real database, and nothing here opens it").expect("write");

  let rel = path
    .strip_prefix(fx.root())
    .expect("under the project")
    .to_string_lossy()
    .to_string();
  let conn = Connection::open(fx.project().db_path()).expect("open the store directly");
  conn
    .execute(
      "INSERT INTO snapshots (path, bytes, outcome, taken_at) VALUES (?1, 1, 'ok', ?2)",
      rusqlite::params![rel, stamp],
    )
    .expect("insert a historical snapshot");
  path
}

fn store_of(fx: &Fixture) -> Store {
  Store::open(&fx.project().db_path()).expect("open the store")
}

#[test]
fn a_snapshot_is_written_into_its_own_namespace_and_recorded() {
  let fx = Fixture::new();
  let store = store_of(&fx);

  let written = backup::take(&fx.project(), &store).expect("take");

  assert!(
    written.is_file(),
    "the snapshot exists: {}",
    written.display()
  );
  assert!(
    written.starts_with(backup::snapshot_dir(&fx.project())),
    "and it is inside the snapshot namespace rather than loose under .backup/: {}",
    written.display()
  );
  // Named from the stamp the database returned, so it sorts the way the rows
  // do and nothing had to ask what time it is.
  let name = written.file_name().unwrap().to_string_lossy().to_string();
  assert!(name.ends_with(".db"), "{name}");
  assert!(
    name.starts_with("20"),
    "the name carries the DB-assigned stamp: {name}"
  );

  let recorded = store.snapshots().expect("read");
  assert_eq!(recorded.len(), 1);
  assert_eq!(recorded[0].outcome, "ok");
  assert!(recorded[0].bytes.unwrap_or(0) > 0, "the size was recorded");
}

/// **`intent upgrade`'s rollback artefacts are untouchable, and this is the
/// case the whole namespace rule exists for.**
///
/// Two mechanisms write under `.backup/` with different retention rules and
/// different owners. A pruner that reasoned about `.backup/` rather than about
/// `.backup/db/` would delete a rollback a user is one bad upgrade away from
/// needing -- and would look correct in every test that only counted
/// snapshots.
#[test]
fn pruning_cannot_reach_the_upgrade_rollback_namespace() {
  let fx = Fixture::new();
  let store = store_of(&fx);

  // An `intent upgrade` rollback, exactly where the v2 upgrader puts it.
  let rollback = fx
    .project()
    .intent_dir()
    .join(".backup")
    .join("backup-20260101120000");
  std::fs::create_dir_all(&rollback).expect("rollback dir");
  let precious = rollback.join("config.json");
  std::fs::write(&precious, b"{}").expect("write");

  // Enough snapshots that everything but the newest is expired.
  for stamp in [
    "2026-01-01T00:00:00.000Z",
    "2026-02-01T00:00:00.000Z",
    "2026-03-01T00:00:00.000Z",
    "2026-04-01T00:00:00.000Z",
  ] {
    historical(&fx, stamp);
  }

  let removed = backup::prune(
    &fx.project(),
    &store,
    Retention {
      daily: 1,
      weekly: 1,
      monthly: 1,
    },
  )
  .expect("prune");

  assert!(!removed.is_empty(), "the prune did something at all");
  assert!(
    precious.is_file(),
    "`intent upgrade`'s rollback survived the snapshot pruner"
  );
  assert!(
    rollback.is_dir(),
    "and so did its directory -- a pruner that removed it would look correct in every test that \
     only counted snapshots"
  );
  for path in &removed {
    assert!(
      path.starts_with(backup::snapshot_dir(&fx.project())),
      "the pruner only ever removed files from its own namespace: {}",
      path.display()
    );
  }

  // **The second arm, and it is the one that tests the confinement rather than
  // the bookkeeping.** Above, the rollback survives because the pruner acts on
  // ROWS and no row names it -- which is true and is not the same as being
  // unable to reach it. Here a row DOES name it, which is what a corrupted
  // table or a future writer with a bug looks like, and the pruner must still
  // refuse: the path is outside its namespace, whatever the row says.
  let conn = Connection::open(fx.project().db_path()).expect("open the store directly");
  conn
    .execute(
      "INSERT INTO snapshots (path, bytes, outcome, taken_at)
       VALUES (?1, 1, 'ok', '2020-01-01T00:00:00.000Z')",
      rusqlite::params!["intent/.backup/backup-20260101120000/config.json"],
    )
    .expect("insert a row pointing outside the namespace");

  backup::prune(
    &fx.project(),
    &store,
    Retention {
      daily: 0,
      weekly: 0,
      monthly: 0,
    },
  )
  .expect("prune");

  assert!(
    precious.is_file(),
    "a row naming a file outside the snapshot namespace must not make it deletable -- the \
     directory is the boundary, not the table"
  );
}

/// The newest of each bucket survives, and the window rolls.
///
/// Four monthly snapshots with `monthly: 2` keeps two and removes two. The
/// point is not the arithmetic but that a snapshot is retained by ANY bucket
/// that still wants it -- which is what makes an old snapshot age out of the
/// day rule and be held by the month rule instead.
#[test]
fn retention_keeps_the_newest_of_each_bucket_and_drops_the_rest() {
  let fx = Fixture::new();
  let store = store_of(&fx);

  let jan = historical(&fx, "2026-01-15T00:00:00.000Z");
  let feb = historical(&fx, "2026-02-15T00:00:00.000Z");
  let mar = historical(&fx, "2026-03-15T00:00:00.000Z");
  let apr = historical(&fx, "2026-04-15T00:00:00.000Z");

  backup::prune(
    &fx.project(),
    &store,
    Retention {
      daily: 0,
      weekly: 0,
      monthly: 2,
    },
  )
  .expect("prune");

  assert!(apr.is_file(), "the newest month is kept");
  assert!(mar.is_file(), "and the one before it");
  assert!(!feb.is_file(), "february is outside a two-month window");
  assert!(!jan.is_file(), "and so is january");

  let left = store.snapshots().expect("read");
  assert_eq!(
    left.len(),
    2,
    "the rows for removed files are gone too -- a row pointing at nothing would make `backup \
     --list` report snapshots nobody can restore from"
  );
}

/// **A failed attempt is never pruned**, because it is the evidence.
#[test]
fn a_failed_attempt_survives_pruning() {
  let fx = Fixture::new();
  let store = store_of(&fx);
  historical(&fx, "2026-01-15T00:00:00.000Z");

  let (id, _) = store.begin_snapshot().expect("begin");
  store
    .finish_snapshot(
      id,
      intentsvcs::store::SnapshotOutcome::Failed,
      None,
      None,
      Some("disk full"),
    )
    .expect("finish");

  backup::prune(
    &fx.project(),
    &store,
    Retention {
      daily: 0,
      weekly: 0,
      monthly: 0,
    },
  )
  .expect("prune");

  let left = store.snapshots().expect("read");
  assert_eq!(
    left.len(),
    1,
    "everything with a file was expired, and the failure record stayed"
  );
  assert_eq!(
    left[0].outcome, "failed",
    "pruning must not consume the evidence that backups have been failing"
  );
}

/// **A missing or malformed setting falls back to keeping MORE, never less.**
///
/// Zero means "keep nothing", so a typo in a config key must not be the thing
/// that deletes every snapshot on the next prune.
#[test]
fn retention_settings_fall_back_to_the_default_rather_than_to_zero() {
  let fx = Fixture::new();
  let default = Retention::default();
  assert_eq!(
    Retention::from_project(&fx.project()),
    default,
    "a project with no backup block gets the defaults"
  );
  assert!(
    default.daily > 0 && default.weekly > 0 && default.monthly > 0,
    "and no default is zero, or an unconfigured project would prune everything"
  );
}
