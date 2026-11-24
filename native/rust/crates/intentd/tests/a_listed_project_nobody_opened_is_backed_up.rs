//! Issue I53c0c22: intentd's backup sweep reaches every project the registry
//! lists, and not only the stores it holds open, and it never changes the
//! shape of a store it did not open.
//!
//! **DRIVEN AGAINST A REAL DAEMON, BECAUSE THE DEFECT WAS A PROCESS'S REACH.**
//! The sweep asked only the projects first contact had opened, so after a
//! restart a project nobody routed to the daemon was never asked: Webber had 8
//! snapshots in 16 days. Both projects here are listed before the daemon
//! starts and neither is ever routed to it. The sweep's first round runs at
//! start, so the wait below is for that round and not for a period.

use std::path::{Path, PathBuf};

use crate::common::{ARM_ATTEMPTS, RunningDaemon, poll_until, project, short_dir};
use intentsvcs::projects::{self, AddedBy};
use intentsvcs::userstate::{self, Dirs};

/// The snapshots the project at `root` holds. A snapshot directory that is
/// not there holds none, and one that cannot be read fails the test, so an
/// absence below is never an unread directory.
fn snapshots(root: &Path) -> Vec<PathBuf> {
  let project = intentsvcs::project::Project::open(root).expect("the project opens");
  let dir = intentsvcs::backup::snapshot_dir(&project);
  match std::fs::read_dir(&dir) {
    Err(e) if e.kind() == std::io::ErrorKind::NotFound => Vec::new(),
    listing => listing
      .unwrap_or_else(|e| panic!("read {}: {e}", dir.display()))
      .map(|entry| entry.expect("read a snapshot entry").path())
      .collect(),
  }
}

#[test]
fn a_listed_project_is_backed_up_and_one_at_another_schema_is_left_as_it_is() {
  let fresh = project("Fresh").canonicalize().expect("canonical root");
  let other = project("Other").canonicalize().expect("canonical root");
  let db = intentsvcs::project::Project::open(&other)
    .expect("the project opens")
    .db_path();
  testkit::assert_checkpointed(&db);
  let older = intentsvcs::store::SCHEMA_VERSION - 1;
  let stamped = testkit::stamp_schema(&db, older);
  assert!(
    snapshots(&fresh).is_empty() && snapshots(&other).is_empty(),
    "precondition: neither project holds a snapshot"
  );

  let home = short_dir("intentd-home");
  let file = userstate::project_registry_under(&Dirs::at_home(&home));
  projects::add(&file, &[fresh.clone(), other.clone()], AddedBy::Discover)
    .expect("list both projects");
  let err_log = short_dir("intentd-log").join("intentd.err.log");
  let _daemon = RunningDaemon::start_logging(home, &err_log);

  let skip_line = |log: &str| {
    log.lines().any(|line| {
      line.contains(&format!("`{}` is NOT being backed up", other.display()))
        && line.contains(&format!("is at schema {older}"))
        && line.contains(&format!(
          "speaks schema {}",
          intentsvcs::store::SCHEMA_VERSION
        ))
    }) && log.contains("remedy: run `intent backup` in the project")
  };
  let swept = poll_until(ARM_ATTEMPTS, || {
    !snapshots(&fresh).is_empty()
      && skip_line(&std::fs::read_to_string(&err_log).unwrap_or_default())
  });
  assert!(
    swept,
    "the first sweep did not back up {} and say why it left {} in {ARM_ATTEMPTS} attempts. stderr:\n{}",
    fresh.display(),
    other.display(),
    std::fs::read_to_string(&err_log).unwrap_or_default()
  );

  assert_eq!(
    std::fs::read(&db).expect("read the skipped store"),
    stamped,
    "the sweep changed the bytes of a store at another schema"
  );
  testkit::assert_no_wal_frames(&db, "the sweep");
  assert!(
    snapshots(&other).is_empty(),
    "the sweep took a snapshot of a store it must leave unopened"
  );
  let _ = std::fs::remove_dir_all(&fresh);
  let _ = std::fs::remove_dir_all(&other);
}
