//! **`AC-08.8`: the daemon's scheduled backup and the operator's typed one are
//! ONE call, and the schedule decides from the store rather than from a
//! timer's memory.**
//!
//! The single-home half is `the_backup_cycle_has_one_home.rs`, which reads call
//! sites. This drives the behaviour that composition and the decision in front
//! of it.
//!
//! **NOTHING HERE WAITS FOR A SWEEP, AND THE REASON IS THE SHAPE RATHER THAN
//! THE PATIENCE.** `intentd`'s sweep contributes a `tokio::time::interval` and
//! a channel send; every DECISION it makes is `backup::due`, and every effect
//! is `backup::cycle`. Both are ordinary functions taking a project and a
//! store, which is what makes the daemon's policy testable without a daemon --
//! and is the same split that keeps `intentd` holding no backup policy at all.

mod common;

use common::{Fixture, ctx};
use intentsvcs::backup::{self, Due};
use intentsvcs::finding::FindingClass;
use intentsvcs::store::{SnapshotOutcome, Store};
use rusqlite::Connection;

fn store_of(fx: &Fixture) -> Store {
  Store::open(&fx.project().db_path()).expect("open the store")
}

/// Rewrite the fixture's config with a chosen `backup` block.
fn with_backup_block(fx: &Fixture, block: &str) {
  fx.write_file(
    "intent/.config/config.json",
    &format!(
      "{{\n  \"intent_version\": \"3.0.0\",\n  \"project_name\": \"Fixture\",\n  \"author\": \"cc\",\n  \"intent_dir\": \"intent\",\n  \"languages\": [],\n  \"backup\": {block}\n}}\n"
    ),
  );
}

/// A successful snapshot row with a chosen stamp, and the file it points at.
fn good_snapshot_at(fx: &Fixture, stamp: &str) {
  let dir = backup::snapshot_dir(&fx.project());
  std::fs::create_dir_all(&dir).expect("snapshot dir");
  let path = dir.join(format!("{}.db", stamp.replace([':', '.'], "-")));
  std::fs::write(&path, b"not a real database, and nothing here opens it").expect("write");
  let rel = path
    .strip_prefix(fx.root())
    .expect("under the project")
    .to_string_lossy()
    .to_string();
  Connection::open(fx.project().db_path())
    .expect("open the store directly")
    .execute(
      "INSERT INTO snapshots (path, bytes, outcome, taken_at) VALUES (?1, 1, 'ok', ?2)",
      rusqlite::params![rel, stamp],
    )
    .expect("insert a historical snapshot");
}

fn findings_of(fx: &Fixture, store: &Store, class: FindingClass) -> Vec<String> {
  intentsvcs::doctor::diagnose(&fx.project(), &ctx(), Some(store))
    .findings
    .into_iter()
    .filter(|f| f.class == class)
    .map(|f| f.detail)
    .collect()
}

// ---------------------------------------------------------------------------
// The decision: `backup::due`.
// ---------------------------------------------------------------------------

/// **A store that has never been backed up is DUE, not undecidable.**
///
/// The tempting arm is that with no snapshot there is no age and so no basis
/// to compare -- which would make the project that has never been backed up
/// the one project a schedule never backs up. The state a schedule exists to
/// leave cannot be the state it treats as no answer.
#[test]
fn a_store_with_no_snapshot_is_due_one_now() {
  let fx = Fixture::new();
  let store = store_of(&fx);
  assert_eq!(backup::due(&fx.project(), &store).expect("due"), Due::Now);
}

#[test]
fn a_store_backed_up_moments_ago_is_not_due_another() {
  let fx = Fixture::new();
  let store = store_of(&fx);
  backup::take(&fx.project(), &store).expect("take");
  assert_eq!(
    backup::due(&fx.project(), &store).expect("due"),
    Due::NotYet,
    "a schedule that fired every sweep would back up every five minutes"
  );
}

#[test]
fn a_store_older_than_its_period_is_due() {
  let fx = Fixture::new();
  with_backup_block(&fx, "{ \"schedule\": \"daily\" }");
  let store = store_of(&fx);
  good_snapshot_at(&fx, "2020-01-01T00:00:00.000Z");
  assert_eq!(backup::due(&fx.project(), &store).expect("due"), Due::Now);
}

/// **AN UNREADABLE PERIOD REFUSES TO SCHEDULE RATHER THAN FALLING BACK, AND
/// THE OLD SNAPSHOT IS WHAT MAKES THIS A REAL TEST.**
///
/// With a six-year-old snapshot, EVERY recognised period says `Now`. So an
/// implementation that quietly used the 24h default would answer `Now` here
/// and be indistinguishable from a correct one on any fixture with a recent
/// snapshot. The refusal is the whole of `Schedule::Unrecognised`'s reason for
/// existing: backing up on a period the operator did not choose, while their
/// actual setting sits in the file looking honoured, is a config that LOOKS
/// configured.
#[test]
fn an_unreadable_period_is_unschedulable_even_when_a_backup_is_overdue() {
  let fx = Fixture::new();
  with_backup_block(&fx, "{ \"schedule\": \"fortnightly\" }");
  let store = store_of(&fx);
  good_snapshot_at(&fx, "2020-01-01T00:00:00.000Z");

  assert_eq!(
    backup::due(&fx.project(), &store).expect("due"),
    Due::Unschedulable("fortnightly".to_string()),
    "it names what was written rather than rounding to the nearest plausible period"
  );
}

/// **A RUN OF FAILURES DOES NOT DEFER THE NEXT ATTEMPT.**
///
/// The store counts only `ok` rows toward freshness, and this is where that
/// matters most: if a failed attempt reset the schedule, a backup failing
/// every hour would be tried once and then left alone -- the schedule silenced
/// by the thing it exists to detect.
#[test]
fn failed_attempts_do_not_defer_the_next_one() {
  let fx = Fixture::new();
  let store = store_of(&fx);
  let (id, _) = store.begin_snapshot().expect("begin");
  store
    .finish_snapshot(id, SnapshotOutcome::Failed, None, None, Some("no space"))
    .expect("finish");

  assert_eq!(
    backup::due(&fx.project(), &store).expect("due"),
    Due::Now,
    "an attempt that failed a moment ago is not a backup and must not stand in for one"
  );
}

// ---------------------------------------------------------------------------
// The effect: `backup::cycle` is take AND prune, in one call.
// ---------------------------------------------------------------------------

/// **ONE CALL DOES BOTH HALVES, WHICH IS WHY A SECOND CALLER CANNOT DO ONLY
/// ONE OF THEM.** Before the cycle existed the composition lived in the CLI's
/// renderer, so a scheduler had nothing to call: the guard beside this file
/// forbids composing it again, and this is what it would have had to compose.
#[test]
fn one_cycle_takes_a_snapshot_and_prunes_the_expired_ones() {
  let fx = Fixture::new();
  with_backup_block(
    &fx,
    "{ \"schedule\": \"daily\", \"retain\": { \"daily\": 1, \"weekly\": 0, \"monthly\": 0 } }",
  );
  // The store is opened FIRST: it is what creates the database the rows go
  // into, and `good_snapshot_at` writes through a direct connection.
  let store = store_of(&fx);
  good_snapshot_at(&fx, "2020-01-01T00:00:00.000Z");
  good_snapshot_at(&fx, "2020-02-01T00:00:00.000Z");

  let ran = backup::cycle(&fx.project(), &store).expect("cycle");

  assert!(
    ran.written.is_file(),
    "the snapshot half wrote a file: {}",
    ran.written.display()
  );
  assert_eq!(
    ran.removed.len(),
    2,
    "the prune half removed both expired snapshots in the same call: {:?}",
    ran.removed
  );
  for gone in &ran.removed {
    assert!(!gone.exists(), "{} is still on disk", gone.display());
  }
}

// ---------------------------------------------------------------------------
// The report: a failure is met at `doctor`, not only in `backup --list`.
// ---------------------------------------------------------------------------

#[test]
fn doctor_names_the_reason_the_newest_attempt_failed() {
  let fx = Fixture::new();
  let store = store_of(&fx);
  let (id, _) = store.begin_snapshot().expect("begin");
  store
    .finish_snapshot(
      id,
      SnapshotOutcome::Failed,
      None,
      None,
      Some("disk I/O error"),
    )
    .expect("finish");

  let failing = findings_of(&fx, &store, FindingClass::BackupFailing);
  assert_eq!(failing.len(), 1, "one failure finding, got {failing:?}");
  assert!(
    failing[0].contains("disk I/O error"),
    "the report carries what the attempt recorded, or the operator goes to the log for it: {}",
    failing[0]
  );
}

/// **THE FAILURE REPORT IS NOT THE STALENESS REPORT, AND THIS IS THE CASE THAT
/// SEPARATES THEM.** A store backed up successfully an hour ago is not stale by
/// any reading -- and if every attempt since has failed, it is on its way to
/// being so with the evidence already on disk. An instrument that waited for
/// the staleness threshold would stay silent through exactly the window in
/// which the cause is knowable and the symptom is not yet here.
#[test]
fn a_failing_backup_is_reported_before_the_store_goes_stale() {
  let fx = Fixture::new();
  let store = store_of(&fx);
  let conn = Connection::open(fx.project().db_path()).expect("open the store directly");
  // An hour ago, computed by SQLite against its own clock inside the INSERT
  // that records it -- the ratified shape (D42), and the only way to say
  // "recent" without this process learning what the time is.
  conn
    .execute(
      "INSERT INTO snapshots (path, bytes, outcome, taken_at)
       VALUES ('intent/.backup/db/an-hour-ago.db', 1, 'ok',
               strftime('%Y-%m-%dT%H:%M:%fZ', 'now', '-1 hour'))",
      [],
    )
    .expect("a good snapshot an hour ago");
  conn
    .execute(
      "INSERT INTO snapshots (outcome, detail) VALUES ('failed', 'permission denied')",
      [],
    )
    .expect("a failure since");

  assert_eq!(
    findings_of(&fx, &store, FindingClass::BackupStale).len(),
    0,
    "an hour-old snapshot is not stale against a daily schedule"
  );
  let failing = findings_of(&fx, &store, FindingClass::BackupFailing);
  assert_eq!(
    failing.len(),
    1,
    "the failure is reported anyway: {failing:?}"
  );
  assert!(failing[0].contains("permission denied"));
}

/// **A FAILURE FOLLOWED BY A SUCCESS IS HISTORY, NOT A FINDING.** A lifetime
/// count would make every estate that ever had a bad day report one forever,
/// which is how a true finding becomes one nobody reads.
#[test]
fn doctor_is_quiet_about_failures_a_later_backup_recovered_from() {
  let fx = Fixture::new();
  let store = store_of(&fx);
  let (id, _) = store.begin_snapshot().expect("begin");
  store
    .finish_snapshot(id, SnapshotOutcome::Failed, None, None, Some("was full"))
    .expect("finish");
  backup::take(&fx.project(), &store).expect("a later backup succeeds");

  assert_eq!(
    findings_of(&fx, &store, FindingClass::BackupFailing).len(),
    0,
    "the store is backed up; what happened before the good snapshot is the history command's"
  );
}

/// **THE COUNT DISTINGUISHES AN INCIDENT FROM A PATTERN**, and phrases them
/// differently because they mean different things to whoever reads it.
#[test]
fn a_run_of_failures_is_counted_and_the_newest_reason_is_the_one_shown() {
  let fx = Fixture::new();
  let store = store_of(&fx);
  for reason in ["first", "second", "third"] {
    let (id, _) = store.begin_snapshot().expect("begin");
    store
      .finish_snapshot(id, SnapshotOutcome::Failed, None, None, Some(reason))
      .expect("finish");
  }

  let failing = findings_of(&fx, &store, FindingClass::BackupFailing);
  assert_eq!(failing.len(), 1, "one finding for the run: {failing:?}");
  assert!(
    failing[0].contains('3') && failing[0].contains("third"),
    "it counts the run and shows the newest reason: {}",
    failing[0]
  );
  assert!(
    !failing[0].contains("first"),
    "every reason would be a log, and `intent backup --list` is already that: {}",
    failing[0]
  );
}

/// **AN UNSCHEDULABLE PROJECT IS TOLD IT IS NOT BEING BACKED UP, NOT MERELY
/// THAT ITS AGE CANNOT BE JUDGED.** The old wording described the INSTRUMENT's
/// difficulty; since the daemon schedules from the same key, the operator's
/// actual exposure is that no backup is being taken at all.
#[test]
fn doctor_says_an_unreadable_schedule_means_no_backup_is_being_taken() {
  let fx = Fixture::new();
  with_backup_block(&fx, "{ \"schedule\": \"fortnightly\" }");
  let store = store_of(&fx);

  let invalid = findings_of(&fx, &store, FindingClass::SchemaInvalid);
  assert_eq!(invalid.len(), 1, "one finding, got {invalid:?}");
  assert!(
    invalid[0].contains("fortnightly") && invalid[0].contains("no scheduled backup"),
    "it names the value AND the consequence: {}",
    invalid[0]
  );
}

// ---------------------------------------------------------------------------
// The policy the cycle prunes on is the one the project DECLARES.
// ---------------------------------------------------------------------------

/// **THE RETENTION KEYS ARE READ, AND UNTIL 2026-08-30 NONE OF THEM WAS.**
///
/// `Retention::from_project` looked in `Config::extra` -- which serde's
/// `flatten` fills with only the keys no NAMED field claimed, and `backup`
/// became a named field the day `schedule` was typed. So the whole block
/// stopped arriving there and every project in the estate pruned on the
/// hardcoded default. **It also looked for the wrong names**, flat
/// `keep_daily` rather than nested `backup.retain.daily`, which the surface
/// ratified and recorded that cc must not invent.
///
/// **NO EXISTING TEST COULD HAVE CAUGHT IT, AND THAT IS THE INTERESTING PART.**
/// `backup_retention.rs` drives the buckets hard and passes its own
/// `Retention` in by hand every time -- correct unit coverage of the pruner,
/// and structurally blind to whether the value ever comes from the config.
/// **The path from the file to the pruner had no test at all**, so the
/// function returned a plausible answer to every call.
#[test]
fn the_declared_retention_reaches_the_pruner() {
  let fx = Fixture::new();
  with_backup_block(
    &fx,
    "{ \"schedule\": \"daily\", \"retain\": { \"daily\": 3, \"weekly\": 2, \"monthly\": 1 } }",
  );
  assert_eq!(
    backup::Retention::from_project(&fx.project()),
    backup::Retention {
      daily: 3,
      weekly: 2,
      monthly: 1
    },
    "the declared tiers reach the pruner rather than the hardcoded default"
  );
}

/// **AN ABSENT `retain` BLOCK IS THE RATIFIED DEFAULT, AND `monthly` IS 12.**
///
/// The code held 6 while `dispatch-table.md` `keys.4.default` said 12, and
/// because no configured value could be read the hardcoded number WAS the
/// policy -- an estate keeping half the monthly history its own surface
/// promised, with nothing able to say otherwise.
#[test]
fn an_absent_retain_block_is_the_ratified_default() {
  let fx = Fixture::new();
  with_backup_block(&fx, "{ \"schedule\": \"daily\" }");
  assert_eq!(
    backup::Retention::from_project(&fx.project()),
    backup::Retention {
      daily: 7,
      weekly: 4,
      monthly: 12
    }
  );
}

/// **ZERO IS A CHOICE AND ABSENCE IS THE DEFAULT** (`keys.4.note`: *absence
/// and zero must not be the same value in a retention policy, because one of
/// them deletes backups*). A per-field default is what keeps them apart; a
/// block read all-or-nothing would make a lone `"daily": 0` reset the other
/// two to their defaults, which reads as honoured and is not.
#[test]
fn a_zero_tier_is_honoured_and_its_siblings_keep_their_defaults() {
  let fx = Fixture::new();
  with_backup_block(&fx, "{ \"retain\": { \"daily\": 0 } }");
  assert_eq!(
    backup::Retention::from_project(&fx.project()),
    backup::Retention {
      daily: 0,
      weekly: 4,
      monthly: 12
    },
    "the tier set to zero is zero, and the tiers not mentioned are the defaults"
  );
}
