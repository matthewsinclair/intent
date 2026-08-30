//! **`AC-08.8`, end to end: a REAL `intentd` takes the snapshot, through the
//! same `intentsvcs::backup::cycle` the typed verb reaches.**
//!
//! **THE PIECES ARE UNIT-TESTED ELSEWHERE AND THAT IS NOT THE SAME CLAIM.**
//! `a_scheduled_backup_is_the_same_call.rs` drives `backup::due` and
//! `backup::cycle` directly, and `the_backup_cycle_has_one_home.rs` reads call
//! sites -- between them a tree can have a correct cycle, a correct decision,
//! a single home, **and nothing wiring the daemon to any of it.** That shape
//! is this crate's oldest scar: its suite once ran 11 of 11 green while every
//! "live daemon" in it was a bare listener. So the daemon here is a real
//! `intentd` started through `intent daemon run`, and what is asserted is a
//! file on disk that only it could have written.
//!
//! **NOTHING WAITS FOR THE SWEEP, AND THAT IS A PROPERTY RATHER THAN A
//! SHORTCUT.** A project considers a backup when its store thread opens --
//! before that thread serves its first queued op -- so by the time the
//! registering request has been ANSWERED the decision has already been made.
//! A test that polled for five minutes would be testing `tokio::time::interval`.
//!
//! **`RealDaemon` ISOLATES `HOME`, WHICH IS THE ONE LINE HERE THAT MUST NOT BE
//! WRONG.** A daemon started under the real one answers every peer session's
//! probe at once; it happened on this machine on 2026-08-30 from an
//! `intentd --help`.

use std::path::Path;
use std::process::Command;

mod common;
use common::RealDaemon;

use intentsvcs::wire::{self, Op, Request, Response};

/// A project with the minimum a v3 verb needs to resolve its root.
///
/// Deliberately the same shape `dual_path_conformance.rs` and
/// `cli_end_to_end.rs` use -- one fixture idiom in this crate, not two.
fn project() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let config = dir.path().join("intent").join(".config");
  std::fs::create_dir_all(&config).expect("mkdir");
  std::fs::write(
    config.join("config.json"),
    "{\n  \"intent_version\": \"3.0.0\",\n  \"project_name\": \"BackedUp\",\n  \"author\": \"cc\",\n  \"intent_dir\": \"intent\",\n  \"languages\": [\"rust\"]\n}\n",
  )
  .expect("write config");
  dir
}

/// The snapshots this project holds on disk, by filename.
///
/// **READ OFF THE FILESYSTEM RATHER THAN OUT OF THE STORE**, so the assertion
/// is about a restorable artefact existing. A row saying a backup happened is
/// what a broken backup writes too -- `backup::take` records the attempt
/// before it can succeed, deliberately -- and this criterion is about the
/// snapshot, not the bookkeeping.
fn snapshots_on_disk(root: &Path) -> Vec<String> {
  let dir = root.join("intent").join(".backup").join("db");
  let Ok(entries) = std::fs::read_dir(&dir) else {
    return Vec::new();
  };
  let mut out: Vec<String> = entries
    .flatten()
    .map(|e| e.file_name().to_string_lossy().to_string())
    .filter(|name| name.ends_with(".db"))
    .collect();
  out.sort();
  out
}

/// Make the daemon open this project, by routing one real op at it.
///
/// **REGISTRATION IS A SIDE EFFECT OF BEING USED AND THERE IS NO `register`
/// VERB** -- `Registry`'s own note, and the reason this is an `Op::ThreadList`
/// rather than a setup call. It is also what makes the assertion below
/// meaningful: nothing here asks for a backup.
fn make_the_daemon_open(daemon: &RealDaemon, root: &Path) -> Response {
  let endpoint = daemon
    .endpoint()
    .expect("the daemon was answering when this test started");
  wire::ask(
    &endpoint,
    &Request {
      root: root.to_path_buf(),
      op: Op::ThreadList,
    },
  )
  .expect("the daemon answers a real op")
}

#[test]
fn a_project_the_daemon_opens_gets_the_backup_it_is_owed() {
  let daemon = RealDaemon::start();
  let project = project();
  let root = project.path();

  assert!(
    snapshots_on_disk(root).is_empty(),
    "the fixture starts with no snapshot, or the assertion below proves nothing"
  );

  let answered = make_the_daemon_open(&daemon, root);
  assert!(
    matches!(answered, Response::Threads { .. }),
    "the op that registers the project has to have been ANSWERED, or the store \
     thread never reached the point this test is about: {answered:?}"
  );

  let after = snapshots_on_disk(root);
  assert_eq!(
    after.len(),
    1,
    "a store the daemon opened and has never backed up is due one, and the \
     daemon takes it: {after:?}"
  );
}

/// **THE DAEMON'S SNAPSHOT IS IN THE STORE THE CLI READS, WHICH IS THE HALF A
/// FILE ON DISK CANNOT SHOW.** `AC-08.8` is that the scheduled path and the
/// manual path are one implementation; a daemon writing a `.db` somewhere
/// proves it wrote a file, and this proves the RECORD landed where
/// `intent backup --list` looks -- same table, same row shape, same project.
#[test]
fn the_cli_can_see_what_the_daemon_backed_up() {
  let daemon = RealDaemon::start();
  let project = project();
  let root = project.path();

  make_the_daemon_open(&daemon, root);
  let written = snapshots_on_disk(root);
  assert_eq!(written.len(), 1, "the daemon took one: {written:?}");

  // **`--list` runs LOCALLY and beside the daemon, on purpose.** Routing is
  // opt-in, so this is an ordinary in-process read of a store another process
  // has open -- which the store serialises, and which is the arrangement
  // `StoreNeed::Shared` exists to permit.
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(["backup", "--list"])
    .current_dir(root)
    .env("HOME", daemon.home())
    .output()
    .expect("the intent binary runs");
  let listing = String::from_utf8_lossy(&out.stdout).to_string();

  assert!(
    out.status.success(),
    "`backup --list` failed: {}",
    String::from_utf8_lossy(&out.stderr)
  );
  assert!(
    listing.contains(&written[0]),
    "the snapshot the daemon wrote is the one the CLI lists. wrote {written:?}, listing was:\n{listing}"
  );
}

/// **A SECOND CONTACT DOES NOT TAKE A SECOND BACKUP**, because the decision is
/// read from the store rather than made on arrival. The first open backs up a
/// store that has never been backed up; the second finds one minutes old
/// against a daily schedule and leaves it alone.
///
/// Without this arm, an implementation that snapshotted on every registration
/// would pass the test above perfectly -- and would fill `.backup/db/` at the
/// rate its clients reconnect.
#[test]
fn opening_a_project_twice_does_not_take_two_backups() {
  let daemon = RealDaemon::start();
  let project = project();
  let root = project.path();

  make_the_daemon_open(&daemon, root);
  make_the_daemon_open(&daemon, root);

  assert_eq!(
    snapshots_on_disk(root).len(),
    1,
    "the second contact finds a fresh snapshot and is not due another"
  );
}
