//! ST0080: a project's `backup.schedule` fires whichever door asks.
//!
//! intentd's sweep reached only the projects it held open, so a project worked
//! through the in-process CLI was never backed up on its schedule. `if_due` is
//! now the one composition of "is it due, then take it", called by intentd and
//! by `intent explore`. These arms hold the three things ST0080 settled: the
//! cadence vocabulary with its tolerance, `backup.keep` as the whole pruning
//! rule when set, and a due backup that fails being an error that is already
//! recorded rather than a quiet nothing.
//!
//! The ST0080 acceptance rows these arms are, by id:
//!
//! - AT-00.1: `a_due_backup_is_taken_and_announced_before_it_starts`, and the
//!   pty drive of `intent explore` recorded on the row.
//! - AT-00.2: `the_three_words_keep_their_periods_and_durations_are_read_beside_them`
//!   and `a_duration_schedule_is_honoured_like_a_word`.
//! - AT-00.3: `a_daily_backup_is_due_at_nine_tenths_of_a_day_and_not_before`.
//! - AT-00.4: `keep_prunes_to_the_newest_n_and_keeps_the_one_just_taken`,
//!   `keep_decides_alone_beside_retain_and_doctor_names_the_pair`,
//!   `without_keep_the_tiers_decide_and_nothing_is_named` and
//!   `a_keep_of_zero_is_refused_when_the_config_is_read`.
//! - AT-00.5: `a_due_backup_that_fails_is_an_error_that_doctor_can_already_see`,
//!   `a_backup_that_is_not_taken_says_why_and_announces_nothing`, and the pty
//!   drive recorded on the row.
//!
//! **Ages are written by SQLite against its own clock**, inside the INSERT
//! that records the row, so no test asks what time it is -- the same way the
//! store's own interval is computed.

use crate::common::{Fixture, ctx};
use intentsvcs::backup::{self, Due, Ran, Schedule};
use intentsvcs::finding::FindingClass;
use intentsvcs::project::Project;
use intentsvcs::store::Store;
use rusqlite::Connection;
use std::cell::Cell;

fn store_of(fx: &Fixture) -> Store {
  Store::open(&fx.project().db_path()).expect("open the store")
}

/// Rewrite the fixture's config with a chosen `backup` block.
fn with_backup_block(fx: &Fixture, block: &str) {
  fx.write_file(
    "intent/.config/config.json",
    &format!(
      "{{\n  \"intent_version\": \"3.0.0\",\n  \"project_name\": \"Fixture\",\n  \"author\": \"ic\",\n  \"intent_dir\": \"intent\",\n  \"languages\": [],\n  \"backup\": {block}\n}}\n"
    ),
  );
}

/// A good snapshot row and its file, `age` old by SQLite's clock (an
/// `strftime` modifier such as `-22 hours`). Returns the file.
///
/// **THE STAMP IS WRITTEN BY THE INSERT, NEVER ASKED FOR.** `strftime` inside
/// an INSERT is the ratified mechanism (`one_clock.rs`); the row's own stamp is
/// read back with `RETURNING` to name the file, so nothing here asks what
/// time it is.
fn good_snapshot_aged(fx: &Fixture, age: &str) -> std::path::PathBuf {
  drop(store_of(fx));
  let conn = Connection::open(fx.project().db_path()).expect("open the store directly");
  let (id, stamp): (i64, String) = conn
    .query_row(
      "INSERT INTO snapshots (path, bytes, outcome, taken_at)
       VALUES ('pending', 1, 'ok', strftime('%Y-%m-%dT%H:%M:%fZ', 'now', ?1))
       RETURNING id, taken_at",
      [age],
      |row| Ok((row.get(0)?, row.get(1)?)),
    )
    .expect("insert an aged snapshot");
  let dir = backup::snapshot_dir(&fx.project());
  std::fs::create_dir_all(&dir).expect("snapshot dir");
  let path = dir.join(format!("{}.db", stamp.replace([':', '.'], "-")));
  std::fs::write(&path, b"not a real database, and nothing here opens it").expect("write");
  let rel = path
    .strip_prefix(fx.root())
    .expect("under the project")
    .to_string_lossy()
    .to_string();
  conn
    .execute(
      "UPDATE snapshots SET path = ?1 WHERE id = ?2",
      rusqlite::params![rel, id],
    )
    .expect("point the row at its file");
  path
}

fn names(paths: &[std::path::PathBuf]) -> Vec<String> {
  paths
    .iter()
    .map(|p| p.file_name().expect("a file").to_string_lossy().to_string())
    .collect()
}

// ---------------------------------------------------------------------------
// (a) The cadence.
// ---------------------------------------------------------------------------

/// The three words keep their meanings, durations are added beside them, and
/// everything else is carried as written. `off` is refused on purpose:
/// `backup.enabled: false` is the one off switch.
#[test]
fn the_three_words_keep_their_periods_and_durations_are_read_beside_them() {
  for (value, hours) in [
    ("hourly", 1),
    ("daily", 24),
    ("weekly", 168),
    ("1h", 1),
    ("12h", 12),
    ("36h", 36),
    ("1d", 24),
    ("7d", 168),
    ("14d", 336),
  ] {
    assert_eq!(
      backup::parse_schedule(value),
      Schedule::Hours(hours),
      "{value} is {hours}h"
    );
  }
  for value in [
    "0h",
    "0d",
    "1.5d",
    "-1h",
    "+2h",
    "2w",
    "h",
    "d",
    "12",
    "12 h",
    "monthly",
    "off",
    "Daily",
    "",
    "99999999999h",
  ] {
    assert_eq!(
      backup::parse_schedule(value),
      Schedule::Unrecognised(value.to_string()),
      "`{value}` is reported, never rounded"
    );
  }
}

/// **DUE AT NINE TENTHS OF THE PERIOD.** Without the tolerance, `explore`
/// opened at 09:00 each day after a 09:05 snapshot finds 23.9h against 24h and
/// `daily` becomes every other day. 22h is due on `daily` (21.6h is the line);
/// 21h is not.
#[test]
fn a_daily_backup_is_due_at_nine_tenths_of_a_day_and_not_before() {
  let fx = Fixture::new();
  with_backup_block(&fx, "{ \"schedule\": \"daily\" }");
  good_snapshot_aged(&fx, "-21 hours");
  let store = store_of(&fx);
  assert_eq!(
    backup::due(&fx.project(), &store).expect("due"),
    Due::NotYet,
    "21h is under 0.9 x 24h"
  );

  let fx = Fixture::new();
  with_backup_block(&fx, "{ \"schedule\": \"daily\" }");
  good_snapshot_aged(&fx, "-22 hours");
  let store = store_of(&fx);
  assert_eq!(
    backup::due(&fx.project(), &store).expect("due"),
    Due::Now,
    "22h is past 0.9 x 24h, so a once-a-morning habit gets its daily backup"
  );
}

/// A duration schedule is honoured on the same terms as a word.
#[test]
fn a_duration_schedule_is_honoured_like_a_word() {
  let fx = Fixture::new();
  with_backup_block(&fx, "{ \"schedule\": \"12h\" }");
  good_snapshot_aged(&fx, "-11 hours");
  let store = store_of(&fx);
  assert_eq!(
    backup::due(&fx.project(), &store).expect("due"),
    Due::Now,
    "11h is past 0.9 x 12h"
  );
}

// ---------------------------------------------------------------------------
// The one composition: `if_due`.
// ---------------------------------------------------------------------------

/// Due: the backup is taken, and the announcement runs first, once.
#[test]
fn a_due_backup_is_taken_and_announced_before_it_starts() {
  let fx = Fixture::new();
  let store = store_of(&fx);
  let announced = Cell::new(0);
  let ran =
    backup::if_due(&fx.project(), &store, || announced.set(announced.get() + 1)).expect("if_due");
  let Ran::Took(cycle) = ran else {
    panic!("a store with no snapshot is due one: {ran:?}");
  };
  assert!(cycle.written.is_file(), "the snapshot exists");
  assert_eq!(
    announced.get(),
    1,
    "the wait is announced once, before the write"
  );
}

/// Not due, turned off, and unreadable each say which, and none announces a
/// wait that is not coming.
#[test]
fn a_backup_that_is_not_taken_says_why_and_announces_nothing() {
  fn quiet() {
    panic!("nothing is being taken, so nothing is announced");
  }

  let fx = Fixture::new();
  let store = store_of(&fx);
  backup::take(&fx.project(), &store).expect("take");
  assert!(matches!(
    backup::if_due(&fx.project(), &store, quiet).expect("if_due"),
    Ran::NotYet
  ));

  let fx = Fixture::new();
  with_backup_block(&fx, "{ \"enabled\": false }");
  let store = store_of(&fx);
  assert!(matches!(
    backup::if_due(&fx.project(), &store, quiet).expect("if_due"),
    Ran::Disabled
  ));

  let fx = Fixture::new();
  with_backup_block(&fx, "{ \"schedule\": \"off\" }");
  let store = store_of(&fx);
  match backup::if_due(&fx.project(), &store, quiet).expect("if_due") {
    Ran::Unschedulable(value) => assert_eq!(value, "off", "the value as written"),
    other => panic!("`off` is not a schedule: {other:?}"),
  }
}

/// **A DUE BACKUP THAT FAILS IS AN ERROR, AND ALREADY A RECORD.** The snapshot
/// directory is made unwritable, so `VACUUM INTO` fails after the attempt is
/// opened; the caller gets `Err` and `doctor` has the failed row to report.
#[cfg(unix)]
#[test]
fn a_due_backup_that_fails_is_an_error_that_doctor_can_already_see() {
  use std::os::unix::fs::PermissionsExt;
  let fx = Fixture::new();
  let store = store_of(&fx);
  let dir = backup::snapshot_dir(&fx.project());
  std::fs::create_dir_all(&dir).expect("snapshot dir");
  std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o555)).expect("read-only");

  let outcome = backup::if_due(&fx.project(), &store, || {});
  std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o755)).expect("restore");

  assert!(
    matches!(outcome, Err(backup::BackupError::Write(_))),
    "the failed write reaches the caller: {outcome:?}"
  );
  let failing: Vec<String> = intentsvcs::doctor::diagnose(
    &fx.project(),
    &ctx(),
    Some(&store),
    intentsvcs::doctor::Scope::All,
  )
  .findings
  .into_iter()
  .filter(|f| f.class == FindingClass::BackupFailing)
  .map(|f| f.detail)
  .collect();
  assert_eq!(
    failing.len(),
    1,
    "doctor reports the recorded failure: {failing:?}"
  );
}

// ---------------------------------------------------------------------------
// (b) keep versus retain.
// ---------------------------------------------------------------------------

/// `keep: 3` over five older snapshots: the cycle writes a sixth and keeps the
/// newest three, the one just taken among them.
#[test]
fn keep_prunes_to_the_newest_n_and_keeps_the_one_just_taken() {
  let fx = Fixture::new();
  with_backup_block(&fx, "{ \"keep\": 3 }");
  let older: Vec<_> = ["-5 days", "-4 days", "-3 days", "-2 days", "-1 days"]
    .iter()
    .map(|age| good_snapshot_aged(&fx, age))
    .collect();
  let store = store_of(&fx);

  let ran = backup::cycle(&fx.project(), &store).expect("cycle");

  assert_eq!(
    names(&ran.removed),
    names(&older[..3]),
    "the three oldest go, oldest first"
  );
  for kept in &older[3..] {
    assert!(
      kept.is_file(),
      "{} is among the newest three",
      kept.display()
    );
  }
  assert!(ran.written.is_file(), "the snapshot just taken is kept");
}

/// With `retain` also set, `keep` still decides alone, and `doctor` names the
/// pair so the tiers do not look configured while pruning nothing.
#[test]
fn keep_decides_alone_beside_retain_and_doctor_names_the_pair() {
  let fx = Fixture::new();
  with_backup_block(
    &fx,
    "{ \"keep\": 1, \"retain\": { \"daily\": 30, \"weekly\": 0, \"monthly\": 0 } }",
  );
  let older = good_snapshot_aged(&fx, "-1 days");
  let store = store_of(&fx);

  let ran = backup::cycle(&fx.project(), &store).expect("cycle");
  assert_eq!(
    names(&ran.removed),
    names(&[older]),
    "keep 1 removes yesterday's, whatever retain says"
  );

  let named: Vec<String> = intentsvcs::doctor::diagnose(
    &fx.project(),
    &ctx(),
    Some(&store),
    intentsvcs::doctor::Scope::All,
  )
  .findings
  .into_iter()
  .filter(|f| f.class == FindingClass::UnhonourableSetting)
  .map(|f| f.detail)
  .collect();
  assert_eq!(named.len(), 1, "{named:?}");
  assert!(named[0].contains("retain is ignored"), "{named:?}");
}

/// Without `keep`, the tiers decide and doctor says nothing about the pair.
#[test]
fn without_keep_the_tiers_decide_and_nothing_is_named() {
  let fx = Fixture::new();
  with_backup_block(&fx, "{ \"retain\": { \"daily\": 7 } }");
  let store = store_of(&fx);
  assert_eq!(
    backup::Pruning::from_project(&fx.project()),
    backup::Pruning::Tiers(backup::Retention {
      daily: 7,
      weekly: 4,
      monthly: 12
    })
  );
  let named = intentsvcs::doctor::diagnose(
    &fx.project(),
    &ctx(),
    Some(&store),
    intentsvcs::doctor::Scope::All,
  )
  .findings
  .into_iter()
  .filter(|f| f.class == FindingClass::UnhonourableSetting)
  .count();
  assert_eq!(named, 0);
}

/// `keep: 0` would remove the snapshot just taken, so the config is refused
/// on read, naming the key.
#[test]
fn a_keep_of_zero_is_refused_when_the_config_is_read() {
  let fx = Fixture::new();
  with_backup_block(&fx, "{ \"keep\": 0 }");
  let err = Project::open(fx.root()).expect_err("keep 0 is refused");
  assert!(
    err.to_string().contains("backup.keep must be at least 1"),
    "the refusal names the key: {err}"
  );
}
