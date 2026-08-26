//! AC-03.10 (c) and the reporting half of (d): rolling retention in its own
//! namespace, and `doctor` telling a stale backup from one that never ran.
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

// ---------------------------------------------------------------------------
// AC-03.10 (d), second half: `doctor` reports STALENESS.
//
// A failure report cannot cover this. A schedule that never fires produces no
// failure, so waiting for an error leaves a user unable to tell a working
// backup from one that silently never started -- the ambiguity that was living
// inside the clause written to prevent it.
// ---------------------------------------------------------------------------

use common::ctx;
use intentsvcs::finding::FindingClass;

fn backup_findings(fx: &Fixture, store: &Store) -> Vec<String> {
  intentsvcs::doctor::diagnose(&fx.project(), &ctx(), Some(store))
    .findings
    .into_iter()
    .filter(|f| f.class == FindingClass::BackupStale)
    .map(|f| f.detail)
    .collect()
}

/// **Never-taken is its own message, not a very large number.**
///
/// "nothing has ever been backed up" and "the last backup is old" call for
/// different actions -- the first says the mechanism has never run, the second
/// says it has stopped -- and a check that reported an enormous age for both
/// would lose exactly the distinction it was added for.
#[test]
fn doctor_reports_a_store_that_has_never_been_backed_up() {
  let fx = Fixture::new();
  let store = store_of(&fx);

  let found = backup_findings(&fx, &store);
  assert_eq!(found.len(), 1, "one finding, got {found:?}");
  assert!(
    found[0].contains("ever been taken") && !found[0].contains("schedule"),
    "it says the mechanism has never run rather than quoting an age against a schedule: {}",
    found[0]
  );
}

#[test]
fn doctor_is_quiet_when_a_backup_was_taken_recently() {
  let fx = Fixture::new();
  let store = store_of(&fx);
  backup::take(&fx.project(), &store).expect("take");

  assert!(
    backup_findings(&fx, &store).is_empty(),
    "a backup taken moments ago is not stale, and a check that said so would be turned off"
  );
}

/// An old snapshot is reported, and the message carries the two numbers a user
/// needs: how old it is and what was expected.
#[test]
fn doctor_reports_a_backup_older_than_the_schedule() {
  let fx = Fixture::new();
  let store = store_of(&fx);
  historical(&fx, "2020-01-01T00:00:00.000Z");

  let found = backup_findings(&fx, &store);
  assert_eq!(found.len(), 1, "one finding, got {found:?}");
  assert!(
    found[0].contains("24h schedule"),
    "the message names what was expected, or the age is a number with no meaning: {}",
    found[0]
  );
  assert!(
    !found[0].contains("ever been taken"),
    "and it is NOT the never-ran message -- something was taken, it is just old: {}",
    found[0]
  );
}

/// **A FAILING schedule reads as stale, which is the truth.**
///
/// This is the case a naive "when did we last try" gets wrong: something IS
/// happening every hour, so a check keyed on attempts reports a healthy recent
/// number while nothing restorable exists.
#[test]
fn doctor_reports_a_schedule_that_runs_and_fails_as_unbacked() {
  let fx = Fixture::new();
  let store = store_of(&fx);
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

  let found = backup_findings(&fx, &store);
  assert_eq!(
    found.len(),
    1,
    "a backup that ran and failed leaves nothing to restore from, so it is reported: {found:?}"
  );
  assert!(
    found[0].contains("ever been taken"),
    "a failed attempt leaves nothing restorable, so it reads as never-backed rather than as a \
     recent one: {}",
    found[0]
  );
}

// ---------------------------------------------------------------------------
// The schedule is CONFIGURATION; the report is not.
//
// hv, 2026-08-26, on finding `doctor` measuring against a period nobody had
// chosen: "Who came up with a rule that the intent db had to be backed up every
// 24h?" and "this _has_ to be a configuration param" and "But it can't be
// hardcoded" -- then, asked whether the check itself should be switchable, "I
// don't want it turned off."
//
// So these arms test BOTH halves, and the second half is the one with no
// natural control: a period that cannot be read must not become a project that
// is not reported on.
// ---------------------------------------------------------------------------

/// Rewrite the fixture's config with a chosen `backup.schedule`, or with no
/// `backup` block at all when `value` is `None`.
fn with_schedule(fx: &Fixture, value: Option<&str>) {
  let block = match value {
    Some(v) => format!(",\n  \"backup\": {{ \"schedule\": {v:?} }}"),
    None => String::new(),
  };
  fx.write_file(
    "intent/.config/config.json",
    &format!(
      "{{\n  \"intent_version\": \"3.0.0\",\n  \"project_name\": \"Fixture\",\n  \"author\": \"dc\",\n  \"intent_dir\": \"intent\",\n  \"languages\": []{block}\n}}\n"
    ),
  );
}

fn findings_of(fx: &Fixture, store: &Store, class: FindingClass) -> Vec<String> {
  intentsvcs::doctor::diagnose(&fx.project(), &ctx(), Some(store))
    .findings
    .into_iter()
    .filter(|f| f.class == class)
    .map(|f| f.detail)
    .collect()
}

/// **The period comes out of the ratified key, and 24 is the declared default
/// of a declared key rather than a literal at the end of a read chain.**
///
/// The mutation this exists to catch is any implementation that answers 24 to
/// everything -- which is what shipped, because `backup.every_hours` appeared
/// exactly once in the tree, on the line that read it, so no config file has
/// ever contained it and `unwrap_or(24)` answered every project.
#[test]
fn the_schedule_is_read_from_the_ratified_key_and_every_word_maps() {
  let fx = Fixture::new();
  for (word, hours) in [("hourly", 1), ("daily", 24), ("weekly", 168)] {
    with_schedule(&fx, Some(word));
    assert_eq!(
      backup::schedule(&fx.project()),
      backup::Schedule::Hours(hours),
      "{word} is a {hours}h period"
    );
  }
}

/// **An absent block is the declared default, never an off switch.**
///
/// hv: "I don't want it turned off." A project that predates the key -- which
/// on the day this landed was every project in the estate -- is still reported
/// on, and absence never acquires a meaning of its own. That is the same
/// refusal the surface makes at `keys.4.note`, in the one subsystem where
/// reading absence as a value costs data.
#[test]
fn an_absent_backup_block_is_the_default_and_not_an_off_switch() {
  let fx = Fixture::new();
  with_schedule(&fx, None);
  assert_eq!(
    backup::schedule(&fx.project()),
    backup::Schedule::Hours(24),
    "absent reads as the declared default"
  );

  let store = store_of(&fx);
  let found = backup_findings(&fx, &store);
  assert_eq!(
    found.len(),
    1,
    "the report survives a config that has never heard of the key: {found:?}"
  );
}

/// **A value outside the closed vocabulary is CARRIED and REPORTED, never
/// rounded to the nearest plausible period.**
///
/// Rounding would report a schedule the operator did not choose while their
/// actual setting sat in the file looking honoured -- a config that LOOKS
/// configured, which is the failure the surface spells out at the `retian`
/// transposition.
#[test]
fn an_unrecognised_schedule_is_named_rather_than_rounded() {
  let fx = Fixture::new();
  with_schedule(&fx, Some("fortnightly"));
  assert_eq!(
    backup::schedule(&fx.project()),
    backup::Schedule::Unrecognised("fortnightly".to_string())
  );

  let store = store_of(&fx);
  let invalid = findings_of(&fx, &store, FindingClass::SchemaInvalid);
  assert_eq!(invalid.len(), 1, "the bad value is reported: {invalid:?}");
  assert!(
    invalid[0].contains("fortnightly"),
    "the report quotes what was actually written: {}",
    invalid[0]
  );
}

/// **An unreadable period silences the COMPARISON and nothing else.**
///
/// "no restorable snapshot has ever been taken" is not a lateness claim and
/// needs no period to be true, so gating it on reading one would be the switch
/// that silences backup failure -- arrived at by accident rather than by a key,
/// which is the only way that switch was ever going to get built.
#[test]
fn an_unreadable_schedule_never_silences_the_never_taken_report() {
  let fx = Fixture::new();
  with_schedule(&fx, Some("fortnightly"));
  let store = store_of(&fx);

  let stale = backup_findings(&fx, &store);
  assert_eq!(stale.len(), 1, "never-taken still fires: {stale:?}");
  assert!(
    stale[0].contains("ever been taken"),
    "and it is still the never-taken message: {}",
    stale[0]
  );
}
