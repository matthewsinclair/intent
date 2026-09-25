//! Snapshots of the durable store, and the retention that keeps them bounded
//! (D35).
//!
//! **A snapshot is same-schema rollback and nothing else.** It is restorable
//! only into a binary that speaks its schema, so it is never the recovery path
//! for a store an upgraded binary refuses -- restoring one taken before a
//! schema change reinstates the schema you were escaping. The recovery path
//! for that is the committed extract (D34/D35).
//!
//! Three things this module holds together, and each exists because the
//! obvious implementation gets it wrong:
//!
//! 1. **The copy goes through SQLite**, never the filesystem. The store opens
//!    WAL, so committed transactions live in `intent.db-wal` until something
//!    checkpoints them -- measured on this shape, a live store with 50 rows
//!    yields 50 through `VACUUM INTO` and **0** through a naive copy, and the
//!    bad copy opens cleanly and reports no error.
//! 2. **The attempt is recorded before it can succeed**, so a failure and a
//!    schedule that never ran are different observable states.
//! 3. **Retention buckets in SQL**, so the decision about which snapshots to
//!    keep is made where the stamps are and no time crosses into this process.

use std::path::PathBuf;

use crate::project::Project;
use crate::store::{SnapshotOutcome, Store, StoreError};

/// **The snapshot namespace, and it is NOT configurable** (ic, endorsed).
///
/// `intent upgrade` writes its own rollback artefacts as `backup-<TIMESTAMP>/`
/// under the same `.backup/` root. Two mechanisms in one directory with
/// different retention rules is precisely how a pruner comes to delete the
/// wrong thing, and **a configurable path is how that collision would become
/// reachable through supported configuration rather than through a bug.** So
/// the pruner below is confined to this subdirectory by construction, and
/// there is no setting that can aim it elsewhere.
pub const SNAPSHOT_DIR: &str = "db";

#[derive(Debug, thiserror::Error)]
pub enum BackupError {
  #[error("preparing the snapshot directory {path}: {source}")]
  Directory {
    path: PathBuf,
    #[source]
    source: std::io::Error,
  },
  #[error("recording the backup attempt: {0}")]
  Record(#[source] StoreError),
  #[error("writing the snapshot: {0}")]
  Write(#[source] StoreError),
  #[error("removing an expired snapshot {path}: {source}")]
  Prune {
    path: PathBuf,
    #[source]
    source: std::io::Error,
  },
}

impl crate::remedy::Remedy for BackupError {
  fn remedy(&self) -> String {
    match self {
      Self::Directory { path, .. } => {
        format!("check that {} is writable by you", path.display())
      }
      // **The attempt could not even be recorded**, which is worse than a
      // failed copy: there is nothing for `doctor` to report later, so this is
      // the one backup failure that would otherwise be invisible.
      Self::Record(_) => {
        "the store could not record that a backup was attempted, so nothing will report this later -- run `intent doctor`".to_string()
      }
      Self::Write(_) => {
        "the attempt is recorded as failed and `intent doctor` will report the estate as unbacked until one succeeds".to_string()
      }
      Self::Prune { .. } => {
        "an expired snapshot could not be removed; the backup itself succeeded, so this is disk hygiene rather than data loss".to_string()
      }
    }
  }
}

/// How many snapshots to keep in each bucket.
///
/// Rolling rather than a flat count, because the two failure modes are
/// opposite: a flat "keep 20" on an hourly schedule holds less than a day of
/// history, and on a monthly one holds two years of it. Bucketing by day, week
/// and month makes the retained window a property of the calendar rather than
/// of how often the schedule happens to fire.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Retention {
  pub daily: u32,
  pub weekly: u32,
  pub monthly: u32,
}

impl Default for Retention {
  /// **THE RATIFIED DEFAULTS, WHICH IS NOT WHAT THIS SAID** (cc, 2026-08-30).
  /// `monthly` was 6 against `dispatch-table.md`'s `keys.4.default: 12`, and
  /// because nothing could read a configured value the hardcoded number WAS
  /// the policy -- an estate keeping half the monthly history its own surface
  /// promised, with no setting able to say otherwise.
  fn default() -> Self {
    let declared = crate::project::RetainConfig::default();
    Self {
      daily: declared.daily,
      weekly: declared.weekly,
      monthly: declared.monthly,
    }
  }
}

impl Retention {
  /// Read from the project's `backup.retain` block.
  ///
  /// **IT READ `Config::extra` UNTIL 2026-08-30 AND COULD NOT WORK.** `backup`
  /// is a NAMED field on `Config`, and serde's `flatten` gives `extra` only the
  /// keys no named field claimed -- so from the moment `backup` was typed for
  /// `schedule`'s sake, the whole block stopped arriving in `extra` and this
  /// returned its hardcoded default for every project that has ever run.
  /// **Measured before the fix: `extra` empty, retention `7/4/6`, against a
  /// config file declaring otherwise on the same line.**
  ///
  /// **AND THE KEYS IT LOOKED FOR WERE NEVER THE RATIFIED ONES.** It read
  /// `keep_daily`, `keep_weekly` and `keep_monthly` flat under `backup`; the
  /// surface names `backup.retain.daily`, `.weekly` and `.monthly`, and says
  /// in the ratification that key names are ic's to name and cc's to implement
  /// against. **So the read had two independent reasons to find nothing**, and
  /// each of them alone would have been enough -- which is why no test caught
  /// it: every one of them passed its own `Retention` in by hand.
  ///
  /// **ABSENT IS THE DEFAULT AND ZERO IS A CHOICE** (`keys.4.note`). That
  /// distinction now lives in the serde defaults on
  /// [`crate::project::RetainConfig`], one per field, rather than in a closure
  /// here -- and a value of the wrong TYPE is refused when the config is read
  /// rather than silently becoming the default, which is the same direction
  /// `config set` takes on an uncoercible value.
  pub fn from_project(project: &Project) -> Self {
    let retain = project.config().backup.retain.unwrap_or_default();
    Self {
      daily: retain.daily,
      weekly: retain.weekly,
      monthly: retain.monthly,
    }
  }
}

/// Which rule prunes this project's snapshots: `backup.keep` when it is set,
/// the `backup.retain` tiers when it is not.
///
/// **ONE RULE AT A TIME, NEVER BOTH** (ST0080). Two rules applied together
/// would hold a snapshot by one and remove it by the other, and which won
/// would be an accident of order. A config that sets both is named by
/// `doctor` rather than refused, because every verb reads config and a
/// pruning preference is not worth taking the tool down over.
///
/// **THE FLAT COUNT IS THE OPERATOR'S TRADE TO MAKE.** [`Retention`] says why
/// the tiers are the default: `keep: 20` on an hourly schedule holds less
/// than a day. A project that sets `keep` has chosen that.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pruning {
  /// Keep the newest N good snapshots.
  Newest(u32),
  /// Keep what the daily, weekly and monthly tiers hold.
  Tiers(Retention),
}

impl Pruning {
  pub fn from_project(project: &Project) -> Self {
    match project.config().backup.keep {
      Some(n) => Self::Newest(n),
      None => Self::Tiers(Retention::from_project(project)),
    }
  }
}

/// How often a backup is expected, as read from configuration.
///
/// **Read separately from [`Retention`] because it answers a different
/// question**: retention is how much history to keep, and this is how often to
/// add to it. Collapsing them would make "keep 7 daily" imply a daily
/// schedule, which is exactly the inference that lets a stopped scheduler look
/// configured.
///
/// **There is deliberately no setting that silences a stale backup**, and hv
/// reaffirmed that on 2026-08-26 in the same breath that made the period
/// configurable: "I don't want it turned off." A switch to turn the warning off
/// is a switch to turn the backup off without noticing, and this is the backup
/// of the durable source of truth. `surface/dispatch-table.md` draws the same
/// line from the surface side at `deliberately_not_keys.1`.
///
/// **An unrecognised value is CARRIED, not corrected.** The alternative is to
/// fall back to the default, which reports a period the operator did not choose
/// while their actual setting sits in the file looking honoured -- the shape
/// this subsystem's own surface notes refuse at `keys.4.note` (absence and zero
/// must not be the same value) and at the `retian` transposition example.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Schedule {
  /// A recognised period, in hours.
  Hours(u32),
  /// A value outside the closed vocabulary, carried verbatim so that whatever
  /// reports it can name what was actually written.
  Unrecognised(String),
}

/// The forms `backup.schedule` accepts, as every reporter of an unreadable
/// schedule names them. One sentence in one place, because `doctor`, intentd
/// and `intent explore` all say it and three copies drift.
pub const SCHEDULE_FORMS: &str =
  "hourly, daily, weekly, or a whole number of hours or days such as 12h or 7d";

/// Read the configured backup schedule.
///
/// **What this replaced is the reason it exists.** The period used to be
/// `unwrap_or(24)` behind `backup.every_hours` -- a key ic had not ratified,
/// which appeared exactly ONCE in the whole tree, on the line that read it, and
/// which therefore no config file has ever contained. Every project was measured
/// against a number none of them could name, find, or change. The ratified key
/// is `backup.schedule` (D35, hv, 2026-08-15), and 24 is now the declared
/// default of a declared key rather than a literal at the end of a chain.
///
/// **THE THREE WORDS KEEP THEIR MEANINGS AND DURATIONS ARE ADDED BESIDE THEM**
/// (ST0080): `<N>h` and `<N>d` with N a whole number of at least 1. Anything
/// else -- `0h`, `1.5d`, `2w`, `monthly`, `off` -- is carried as
/// [`Schedule::Unrecognised`]. **There is no `off`**: `backup.enabled: false`
/// is the one switch that stops scheduled backups, and a second spelling of
/// it here could say off while the other said daily.
pub fn schedule(project: &Project) -> Schedule {
  parse_schedule(&project.config().backup.schedule)
}

/// [`schedule`]'s reading of one value, apart from the project that holds it.
pub fn parse_schedule(value: &str) -> Schedule {
  let hours = match value {
    "hourly" => Some(1),
    "daily" => Some(24),
    "weekly" => Some(168),
    _ => duration_hours(value),
  };
  match hours {
    Some(hours) => Schedule::Hours(hours),
    None => Schedule::Unrecognised(value.to_string()),
  }
}

/// `<N>h` or `<N>d` as hours; `None` for anything else, including N of 0, a
/// sign, a fraction or a period too long to count in hours.
fn duration_hours(value: &str) -> Option<u32> {
  let (digits, per) = value
    .strip_suffix('h')
    .map(|n| (n, 1))
    .or_else(|| value.strip_suffix('d').map(|n| (n, 24)))?;
  if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
    return None;
  }
  let n: u32 = digits.parse().ok()?;
  (n >= 1).then_some(n)?.checked_mul(per)
}

/// Where snapshots live for this project.
pub fn snapshot_dir(project: &Project) -> PathBuf {
  project.intent_dir().join(".backup").join(SNAPSHOT_DIR)
}

/// The newest snapshot on disk, read from the DIRECTORY rather than the store.
///
/// **FOR THE ONE CALLER THAT CANNOT ASK THE STORE** (issue 0447): a store whose
/// index table cannot be read refuses every verb that opens it, `intent backup
/// --list` included, so the refusal names the file itself. Snapshot names are
/// the stamp with its separators replaced, so they sort in time order, and the
/// greatest name is the newest. `None` when there is no snapshot, or the
/// directory cannot be read -- either way there is nothing to point at.
pub fn newest_snapshot_on_disk(project: &Project) -> Option<PathBuf> {
  std::fs::read_dir(snapshot_dir(project))
    .ok()?
    .filter_map(|entry| entry.ok().map(|e| e.path()))
    .filter(|path| path.extension().is_some_and(|ext| ext == "db"))
    .max()
}

/// Take one snapshot, recording the attempt either way.
///
/// **The file is named from the stamp the DATABASE returned** when the attempt
/// was opened, so nothing here asks what time it is and the name is guaranteed
/// to sort in the order the rows do. Colons and dots are replaced because a
/// timestamp is a poor filename on some filesystems and an awkward one on all
/// of them; the authoritative value stays in the row.
pub fn take(project: &Project, store: &Store) -> Result<PathBuf, BackupError> {
  let dir = snapshot_dir(project);
  std::fs::create_dir_all(&dir).map_err(|source| BackupError::Directory {
    path: dir.clone(),
    source,
  })?;

  let (id, stamp) = store.begin_snapshot().map_err(BackupError::Record)?;
  let dest = dir.join(format!("{}.db", stamp.replace([':', '.'], "-")));

  match store.snapshot_into(&dest) {
    Ok(()) => {
      let bytes = std::fs::metadata(&dest).map(|m| m.len()).ok();
      store
        .finish_snapshot(
          id,
          SnapshotOutcome::Ok,
          Some(&project.relative(&dest)),
          bytes,
          None,
        )
        .map_err(BackupError::Record)?;
      Ok(dest)
    }
    Err(cause) => {
      // **Recorded as failed BEFORE the error is returned.** A backup that
      // fails and says nothing is the silent failure this criterion is about,
      // and the caller propagating the error is not enough on its own -- a
      // scheduled backup's caller is a daemon nobody is watching.
      store
        .finish_snapshot(
          id,
          SnapshotOutcome::Failed,
          None,
          None,
          Some(&cause.to_string()),
        )
        .map_err(BackupError::Record)?;
      Err(BackupError::Write(cause))
    }
  }
}

/// Remove snapshots outside the retention window, returning what was removed.
///
/// **The buckets are computed in SQL and the pruner only ever removes files it
/// has a ROW for**, inside [`SNAPSHOT_DIR`]. Both halves matter: a pruner that
/// globbed the directory would delete a file some other mechanism put there,
/// and `intent upgrade`'s rollback artefacts live one level up.
pub fn prune(
  project: &Project,
  store: &Store,
  retention: Retention,
) -> Result<Vec<PathBuf>, BackupError> {
  let expired = store
    .expired_snapshots(retention.daily, retention.weekly, retention.monthly)
    .map_err(BackupError::Record)?;
  remove(project, store, expired)
}

/// Remove every good snapshot but the newest `keep` -- `backup.keep`'s rule,
/// with [`prune`]'s confinement.
pub fn prune_newest(
  project: &Project,
  store: &Store,
  keep: u32,
) -> Result<Vec<PathBuf>, BackupError> {
  let expired = store
    .snapshots_beyond_newest(keep)
    .map_err(BackupError::Record)?;
  remove(project, store, expired)
}

/// Delete the files of expired snapshot rows and forget the rows, confined to
/// [`SNAPSHOT_DIR`] whichever rule chose them.
fn remove(
  project: &Project,
  store: &Store,
  expired: Vec<(i64, String)>,
) -> Result<Vec<PathBuf>, BackupError> {
  let dir = snapshot_dir(project);
  let mut removed = Vec::new();
  for (id, rel) in expired {
    let path = project.root().join(&rel);
    if !path.starts_with(&dir) {
      continue;
    }
    // Confined by construction rather than by trusting the stored path: a row
    // whose path points outside the snapshot directory is not something to act
    // on, whatever wrote it.
    match std::fs::remove_file(&path) {
      Ok(()) => {}
      // Already gone is the outcome we wanted; the row still needs clearing.
      Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
      Err(source) => return Err(BackupError::Prune { path, source }),
    }
    store.forget_snapshot(id).map_err(BackupError::Record)?;
    removed.push(path);
  }
  Ok(removed)
}

/// What one backup cycle did.
///
/// **A RETURN VALUE RATHER THAN PRINTED OUTPUT, BECAUSE IT HAS TWO CALLERS AND
/// ONLY ONE OF THEM HAS A TERMINAL.** The CLI renders these lines for whoever
/// typed the verb; the daemon has nobody to render to and needs the same facts
/// to log them. A `cycle` that printed would have forced the daemon to build a
/// second implementation to get a value back, which is the exact split this
/// criterion exists to close.
#[derive(Debug)]
pub struct Cycle {
  /// The snapshot that was written.
  pub written: PathBuf,
  /// Snapshots retention removed, in the order they were removed.
  pub removed: Vec<PathBuf>,
}

/// Take a snapshot and prune to the retention window -- **the whole backup
/// cycle, as ONE function, which is the whole of `AC-08.8`.**
///
/// **WHAT THIS REPLACED IS THE REASON IT EXISTS.** Until this landed the cycle
/// was not a function at all: `intent backup` composed [`take`],
/// [`Retention::from_project`] and [`prune`] inline in the CLI's render layer,
/// so the *policy* -- that a backup is followed by a prune, against the
/// retention this project declares -- lived in a renderer. A daemon scheduling
/// a backup would have had nothing to call and would have composed the same
/// three itself: **not two processes racing, but two implementations that agree
/// today**, which is the failure the criterion names in those words.
///
/// **THE PRUNE IS PART OF THE CYCLE RATHER THAN A SEPARATE CHORE, AND THE
/// ASYMMETRY IS WHY.** A caller that took and did not prune leaves the window
/// growing without bound and nothing reports it, because retention has no
/// staleness check the way freshness does; a caller that pruned and did not
/// take is a no-op on a healthy store. So the composition is what has to be
/// single-homed, not the halves.
///
/// **THE RETENTION IS READ HERE, FROM THE PROJECT, AND IS NOT A PARAMETER.**
/// [`prune`] takes one because the tests need to drive the buckets directly;
/// this does not, because a scheduled cycle and a typed one applying different
/// retention is precisely the drift the row forbids -- and a parameter is a
/// standing invitation to supply a different one.
///
/// Fails the way [`take`] and [`prune`] fail: a failed snapshot is recorded as
/// failed before the error is returned, so the store carries the evidence even
/// when the caller is a daemon nobody is watching.
pub fn cycle(project: &Project, store: &Store) -> Result<Cycle, BackupError> {
  let written = take(project, store)?;
  let removed = match Pruning::from_project(project) {
    Pruning::Newest(keep) => prune_newest(project, store, keep)?,
    Pruning::Tiers(retention) => prune(project, store, retention)?,
  };
  Ok(Cycle { written, removed })
}

/// Take a scheduled backup if one is due -- [`due`] then [`cycle`], as ONE
/// call, so every door that honours `backup.schedule` composes them the same
/// way (ST0080).
///
/// **THE DOORS ARE intentd's SWEEP AND `intent explore`.** intentd reaches only
/// the projects it holds open, so a project worked through the in-process CLI
/// was never backed up on its schedule -- Lamplight's newest snapshot was 433h
/// old against `daily`. `explore` is the door hv named; a session hook was
/// considered and left out, because hooks run `intent` in every Claude session
/// and a snapshot is seconds of writing in front of each one.
///
/// **A CALLER MUST NOT RUN THIS WHERE IT CAN BE ABANDONED.** A snapshot is a
/// write: a thread given up on mid-`VACUUM INTO` leaves a partial file and an
/// attempt row with no outcome. `explore`'s loader abandons its thread on
/// `Esc`, which is right for its reads, so `explore` calls this after the load
/// and not inside it.
///
/// **`before_taking` RUNS ONLY WHEN A BACKUP IS DUE, JUST BEFORE IT STARTS**,
/// so a door with a person in front of it can say why the next few seconds
/// are a wait. The daemon has nobody to tell and passes `|| {}`.
///
/// A failed snapshot is `Err`, and [`take`] has already recorded it as failed,
/// so `doctor` reports it whether or not the caller says anything.
pub fn if_due(
  project: &Project,
  store: &Store,
  before_taking: impl FnOnce(),
) -> Result<Ran, BackupError> {
  Ok(match due(project, store)? {
    Due::Now => {
      before_taking();
      Ran::Took(cycle(project, store)?)
    }
    Due::NotYet => Ran::NotYet,
    Due::Disabled => Ran::Disabled,
    Due::Unschedulable(value) => Ran::Unschedulable(value),
  })
}

/// What [`if_due`] did.
#[derive(Debug)]
pub enum Ran {
  /// A backup was due and was taken; the cycle's outcome.
  Took(Cycle),
  /// The newest good snapshot is younger than the period allows for.
  NotYet,
  /// `backup.enabled` is false.
  Disabled,
  /// `backup.schedule` could not be read, so nothing is being taken. Carries
  /// the value as written.
  Unschedulable(String),
}

/// Whether a scheduled backup should run now.
///
/// **THE DECISION LIVES HERE AND NOT IN THE DAEMON, SO THAT `intentd` HOLDS NO
/// BACKUP POLICY AT ALL** (`IN-AG-THIN-COORD-001`). The daemon's whole part is
/// to ask this on a sweep and call [`cycle`] when the answer is [`Due::Now`];
/// every rule about periods, never-taken stores and unreadable settings is in
/// this file beside the rest of D35.
///
/// **NOTHING HERE READS A CLOCK, AND IT IS THE STORE THAT MAKES THAT POSSIBLE**
/// (D42). [`crate::store::Store::hours_since_last_good_snapshot`] returns an
/// INTERVAL computed by SQLite against its own stamps -- so this compares two
/// numbers and there is no moment at which this process knows what the time is.
/// A scheduler that remembered when it last ran would need one, and would also
/// forget across a restart.
///
/// **DERIVED FROM THE STORE RATHER THAN FROM AN IN-MEMORY TIMER, WHICH IS WHAT
/// MAKES A RESTART HARMLESS.** A daemon that started a fresh interval on boot
/// would, on a machine rebooted daily, never reach a daily period -- the
/// backup would be permanently one boot away. Asking the store how old the
/// newest good snapshot is gives the same answer to a daemon that has run for
/// a month and to one that started ten seconds ago.
///
/// **ONLY GOOD SNAPSHOTS COUNT, WHICH THE STORE ALREADY ENFORCES**: a run of
/// failures does not defer the next attempt, because a failed backup is not a
/// backup and a schedule that failed hourly for a week must keep trying.
pub fn due(project: &Project, store: &Store) -> Result<Due, BackupError> {
  // **THE ONE READER OF `backup.enabled`, AND IT GATES THIS DECISION ONLY.**
  // `cycle` is deliberately NOT gated: `intent backup` is a typed instruction
  // and a setting that made the daemon skip a project must not also refuse the
  // operator standing in front of it. Nor is `doctor` gated -- a stale backup
  // is still stale when you asked for it to stop being taken.
  if !project.config().backup.enabled {
    return Ok(Due::Disabled);
  }
  let every = match schedule(project) {
    Schedule::Hours(hours) => f64::from(hours),
    // **NOT A FALLBACK TO THE DEFAULT, DELIBERATELY.** Backing up on a period
    // the operator did not choose, while their actual setting sits in the file
    // looking honoured, is the failure [`schedule`] was rewritten to remove.
    // Refusing to schedule is visible; a silent 24 hours is not.
    Schedule::Unrecognised(value) => return Ok(Due::Unschedulable(value)),
  };

  // **DUE AT NINE TENTHS OF THE PERIOD, NOT AT THE PERIOD** (ST0080, vc under
  // hv's go, 2026-09-25). A door opened by habit rather than by a timer would
  // otherwise miss by minutes: `explore` opened at 09:00 each day after a
  // 09:05 snapshot finds 23.9h against 24h, and `daily` becomes every other
  // day. So "daily" means "once in each day's use", never "exactly 24h". The
  // daemon backs up at most a tenth early, which costs nothing; `doctor`'s
  // staleness check keeps its own grace and does not read this.
  let every = every * DUE_AT;
  match store
    .hours_since_last_good_snapshot()
    .map_err(BackupError::Record)?
  {
    // **NEVER TAKEN IS DUE, NOT UNKNOWN.** A store with no restorable snapshot
    // is the state the schedule exists to leave, and treating the absence as
    // "no basis to decide" would make a project that has never been backed up
    // the one project that never gets backed up.
    None => Ok(Due::Now),
    Some(hours) if hours >= every => Ok(Due::Now),
    Some(_) => Ok(Due::NotYet),
  }
}

/// The fraction of the configured period after which a backup is due.
pub const DUE_AT: f64 = 0.9;

/// The answer [`due`] gives.
///
/// **`Unschedulable` IS A THIRD STATE RATHER THAN A `NotYet`, BECAUSE THE TWO
/// CALL FOR OPPOSITE ACTIONS.** *Not yet* resolves itself by waiting; *no
/// period could be read* resolves only when somebody edits the config, and
/// collapsing them would mean a project silently never backed up looked
/// exactly like one backed up an hour ago.
///
/// **IT CARRIES THE VALUE THAT WAS ACTUALLY WRITTEN**, for the reason
/// [`Schedule::Unrecognised`] does: whatever reports this has to be able to
/// name the setting the operator typed, or the report is *something is wrong
/// with your config* and the operator goes looking.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Due {
  /// `backup.enabled` is false, so the daemon takes no scheduled snapshot.
  ///
  /// **CHECKED BEFORE THE SCHEDULE, SO AN INERT `backup.schedule` IS NOT
  /// ANNOUNCED AS A DEFECT.** A project that has turned the sweep off has no
  /// period to be wrong about, and a daemon complaining about the spelling of
  /// a setting it is not using is noise. Nothing is hidden by the ordering:
  /// `doctor` reads `backup.schedule` on its own and reports a value outside
  /// the vocabulary either way.
  ///
  /// **AND THIS IS NOT `NotYet` WITH A DIFFERENT CAUSE.** `NotYet` means ask
  /// again later; this means do not ask. Collapsing them would leave the
  /// daemon unable to say WHY it is not backing up, which is the one thing an
  /// operator wanting that answer is looking for.
  Disabled,
  /// The period has elapsed, or no snapshot has ever been taken.
  Now,
  /// The newest good snapshot is younger than the configured period.
  ///
  /// **CARRIES NO NUMBERS, AND THAT IS TO KEEP AN INTERVAL FROM BECOMING A
  /// TIME.** A caller given "3.5 hours ago" would be one subtraction away from
  /// a timestamp, and D42's whole shape is that this estate never holds one it
  /// did not get from a record.
  NotYet,
  /// `backup.schedule` is outside the closed vocabulary, so no period exists
  /// to schedule against. Carries the value as written.
  Unschedulable(String),
}
