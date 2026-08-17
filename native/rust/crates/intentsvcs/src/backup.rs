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
  fn default() -> Self {
    Self {
      daily: 7,
      weekly: 4,
      monthly: 6,
    }
  }
}

impl Retention {
  /// Read from `intent/.config/config.json`, falling back per key.
  ///
  /// **A malformed or missing value falls back to the default rather than to
  /// zero**, and the direction matters: zero means "keep nothing", so a typo in
  /// a config key would silently delete every snapshot on the next prune. The
  /// safe failure for a retention setting is to keep too much.
  pub fn from_project(project: &Project) -> Self {
    let default = Self::default();
    let Some(table) = project
      .config()
      .extra
      .get("backup")
      .and_then(|v| v.as_object())
    else {
      return default;
    };
    let read = |key: &str, fallback: u32| {
      table
        .get(key)
        .and_then(serde_json::Value::as_u64)
        .and_then(|n| u32::try_from(n).ok())
        .unwrap_or(fallback)
    };
    Self {
      daily: read("keep_daily", default.daily),
      weekly: read("keep_weekly", default.weekly),
      monthly: read("keep_monthly", default.monthly),
    }
  }
}

/// How often a backup is expected, in hours. Default daily.
///
/// **Read separately from [`Retention`] because it answers a different
/// question**: retention is how much history to keep, and this is how often to
/// add to it. Collapsing them would make "keep 7 daily" imply a daily
/// schedule, which is exactly the inference that lets a stopped scheduler look
/// configured.
///
/// **There is deliberately no setting that silences a stale backup.** A switch
/// to turn the warning off is a switch to turn the backup off without noticing,
/// and this is the backup of the durable source of truth.
pub fn schedule_hours(project: &Project) -> u32 {
  project
    .config()
    .extra
    .get("backup")
    .and_then(|v| v.as_object())
    .and_then(|t| t.get("every_hours"))
    .and_then(serde_json::Value::as_u64)
    .and_then(|n| u32::try_from(n).ok())
    .filter(|n| *n > 0)
    .unwrap_or(24)
}

/// Where snapshots live for this project.
pub fn snapshot_dir(project: &Project) -> PathBuf {
  project.intent_dir().join(".backup").join(SNAPSHOT_DIR)
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
