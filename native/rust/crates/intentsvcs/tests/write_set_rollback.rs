//! AT-04.1 (rollback half) / AC-04.1: the unwind itself can fail, and that
//! case had no test at all.
//!
//! **Why this matters more after the D01 reversal.** `WriteSet::commit`
//! unwinding is the entire reason a failed projection leaves the tree stale
//! rather than torn -- it is what lets the facade say "the change is recorded,
//! the files are as they were". The one path where that promise does not hold
//! is [`WriteError::TornRollback`], and with no db -> disk direction yet
//! (AC-03.9) a genuinely torn tree has no repair but the next successful
//! mutation. vc found it had zero occurrences under `crates/*/tests/`.
//!
//! **Writing the test is what found the defect, and it was in the reporting
//! rather than in the unwinding.** A prior is recorded before its write is
//! attempted, so the FAILING path had a prior too, and the unwind tried to
//! restore a file that write had never touched. When that restore was refused,
//! the batch reported the estate TORN while it was byte-for-byte intact --
//! the loudest message the write layer has, raised for the calmest state, and
//! under the reversed D01 it points at the one condition with no repair.
//! Fixed by `Prior::written`.
//!
//! **The case vc asked for IS covered, and getting there took correcting an
//! argument this file already contained.** An earlier pass enumerated four
//! constructions, found none of them reached the variant, and concluded it was
//! unreachable single-threaded. Every construction was correct and the
//! conclusion was scoped to one producer: they are all about `WriteSet::commit`
//! unwinding, where the restore follows the failed write with no caller in
//! between. **`Applied::rollback` is the other producer and it has a
//! caller-controlled window BY DESIGN** -- that window is the entire reason
//! `Applied` exists. Both are recorded below, the narrow claim kept where it is
//! true and the wider one withdrawn.

mod common;

use common::Fixture;
use intentsvcs::write_set::{WriteError, WriteSet};

#[cfg(unix)]
fn chmod(path: &std::path::Path, mode: u32) -> u32 {
  use std::os::unix::fs::PermissionsExt;
  let was = std::fs::metadata(path)
    .expect("metadata")
    .permissions()
    .mode();
  std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode)).expect("chmod");
  was
}

/// **The false positive this file was written to find, now FIXED.**
///
/// A prior is recorded BEFORE its write is attempted, so the path that fails
/// has a prior too -- and the unwind used to try to restore a file the failed
/// write had never modified. With the file read-only and its directory
/// read-only, that restore is refused, and the batch reported the estate as
/// TORN while it was byte-for-byte intact: the loudest message the write layer
/// has, raised for the calmest state.
///
/// Under the reversed D01 that is worse than it sounds. `TornRollback` is the
/// one state with no repair -- stale files can be rewritten from truth, torn
/// ones cannot -- and with no db -> disk direction yet (AC-03.9) it points an
/// operator at a problem they cannot fix and do not have.
///
/// `Prior::written` is the fix: nothing is restored for a write that never
/// landed, while directories `record` really did create are still removed.
#[cfg(unix)]
#[test]
fn a_write_that_never_landed_is_not_reported_as_torn() {
  let fx = Fixture::new();
  let dir = fx.path("locked");
  std::fs::create_dir_all(&dir).expect("mkdir");
  let target = dir.join("held.json");
  std::fs::write(&target, "original").expect("seed");

  // File read-only so a restore could not rewrite it; directory read-only so
  // the write cannot land its temp file. Order matters: the file first, since
  // a locked directory would refuse the chmod of something inside it.
  let file_mode = chmod(&target, 0o444);
  let dir_mode = chmod(&dir, 0o555);

  let mut set = WriteSet::new();
  set.add(target.clone(), "replacement".to_string());
  let err = set.commit().expect_err("the write must fail");

  chmod(&dir, dir_mode);
  chmod(&target, file_mode);

  assert!(
    matches!(err, WriteError::Io { .. }),
    "nothing landed, so nothing was torn -- this must be an ordinary write failure: {err}"
  );
  assert_eq!(
    std::fs::read_to_string(&target).expect("read"),
    "original",
    "and the evidence that it is ordinary: the file is untouched"
  );
}

/// **`WriteSet::commit`'s OWN unwind cannot be torn without concurrent
/// interference**, and the four constructions below are why.
///
/// **Scoped to that producer deliberately: this used to claim the VARIANT was
/// unreachable, and it is not.** `a_rollback_that_cannot_restore_reports_the
/// _estate_as_torn` reaches it through `Applied::rollback`. The enumeration
/// here is sound and the conclusion drawn from it was not -- which is this
/// estate's own two-writers rule turned on an argument instead of on code.
///
/// Within `commit`, a restore fails only if the path stops being a writable
/// file between the write and the unwind, and nothing a caller does can arrange
/// that --
///
/// - a read-only DIRECTORY stops the write (temp + rename) rather than the
///   restore (a plain `fs::write` on the path);
/// - a read-only FILE does not stop the write either, because `rename` over it
///   needs permission on the directory, not on the file -- and the renamed
///   temp arrives writable, so the restore then succeeds;
/// - a path that is a directory fails at `record`, before anything lands;
/// - adding the same path twice, or nesting one write under another, both
///   unwind cleanly.
///
/// So WITHIN `commit` the variant guards against another process changing
/// permissions or replacing a path mid-batch -- a real hazard in a tree several
/// agents and a daemon are writing to, and not one a deterministic test can
/// arrange. A racing test would be flaky, which is worse than absent.
#[test]
fn the_commit_unwind_cannot_be_torn_without_concurrent_interference() {
  // The type still has to render its distinct message: a caller that could not
  // tell it from an ordinary failure would retry into the damage.
  let err = WriteError::TornRollback {
    path: "intent/st/ST0001/info.md".to_string(),
    unrestored: 2,
    source: std::io::Error::other("permission denied"),
  };
  let text = err.to_string();
  assert!(text.contains("rolling back left"), "{text}");
  assert!(
    text.contains('2'),
    "the count of unrestored files is named: {text}"
  );
  assert!(
    text.contains("intent/st/ST0001/info.md"),
    "and the path that triggered it: {text}"
  );
}

/// **THE VARIANT HAS TWO PRODUCERS AND THE ANALYSIS ABOVE ONLY ENUMERATED ONE.**
///
/// Everything argued above is about `WriteSet::commit`'s own `unwind`, where
/// the restore follows the failed write with no caller in between -- and for
/// that producer it holds. **`Applied::rollback` is the other producer, and it
/// has a caller-controlled window by design**: `commit` returns `Applied` so
/// the caller can hold the batch while the DB write happens, and roll the files
/// back if that fails. The window between those two calls is the whole reason
/// the type exists.
///
/// So the variant is reachable deterministically, on one thread, and it is not
/// a contrivance: a permission change in that window -- an operator, a peer
/// agent, a deploy touching the tree -- produces exactly this, and under the
/// reversed D01 a torn tree has no repair but the next successful mutation.
///
/// **This is the same shape as the rule this estate keeps re-finding, applied
/// to an ARGUMENT rather than to code: enumerating the ways one of two writers
/// can fail and concluding the failure is unreachable.** The four constructions
/// above are all correct; the conclusion drawn from them was scoped to the
/// producer that happened to be in view.
#[cfg(unix)]
#[test]
fn a_rollback_that_cannot_restore_reports_the_estate_as_torn() {
  let fixture = Fixture::new();
  let dir = fixture.path("intent/st");
  std::fs::create_dir_all(&dir).expect("mkdir");

  let mut set = WriteSet::new();
  set.add(dir.join("info.md"), "the projection".to_string());
  let applied = set.commit().expect("the files land");
  assert!(
    dir.join("info.md").is_file(),
    "precondition: the batch really landed, so the rollback below has something to undo"
  );

  // The window `Applied` exists to create: the DB write is happening, and the
  // tree changes underneath. 0o555 leaves the file readable and the directory
  // untouchable, which is what stops `remove_file` in the restore.
  let was = chmod(&dir, 0o555);
  let torn = applied.rollback();
  chmod(&dir, was);

  match torn {
    Err(WriteError::TornRollback { unrestored, .. }) => {
      assert_eq!(
        unrestored, 1,
        "the count is the operator's whole picture of the damage -- how many files are NOT as they were"
      );
    }
    other => panic!(
      "a rollback that could not restore must report the estate as TORN, not as an ordinary failure and not as success. A caller that cannot tell the two \
       apart retries into the damage, and under the reversed D01 a torn tree has no repair but the next successful mutation. got: {other:?}"
    ),
  }

  assert!(
    dir.join("info.md").is_file(),
    "and the file is still there, which is what TORN means: the estate is not as it was and the tool could not put it back"
  );
}

/// **The discriminator, and without it the case above passes on an
/// implementation that calls every rollback torn.**
///
/// Same fixture, same batch, same call -- only the permission change removed.
/// A rollback that CAN restore must report success, or the loudest message the
/// write layer has is raised for the calmest state.
#[cfg(unix)]
#[test]
fn a_rollback_that_can_restore_is_not_reported_as_torn() {
  let fixture = Fixture::new();
  let dir = fixture.path("intent/st");
  std::fs::create_dir_all(&dir).expect("mkdir");

  let mut set = WriteSet::new();
  set.add(dir.join("info.md"), "the projection".to_string());
  let applied = set.commit().expect("the files land");

  applied
    .rollback()
    .expect("an unobstructed rollback restores");
  assert!(
    !dir.join("info.md").exists(),
    "the file did not exist before the batch, so restoring means removing it"
  );
}

/// An ordinary failure -- one the unwind CAN undo -- is not reported as torn.
///
/// The discriminator. Without it the test above would pass on an
/// implementation that called every failure torn, which is the alarm-always-on
/// shape that teaches operators to ignore the one message that matters.
#[cfg(unix)]
#[test]
fn a_failure_the_unwind_can_undo_is_an_ordinary_write_error() {
  let fx = Fixture::new();
  let good = fx.path("writable");
  std::fs::create_dir_all(&good).expect("mkdir");
  let landed = good.join("first.json");

  let locked = fx.path("locked");
  std::fs::create_dir_all(&locked).expect("mkdir");
  let blocked = locked.join("second.json");
  let dir_mode = chmod(&locked, 0o555);

  let mut set = WriteSet::new();
  set.add(landed.clone(), "written then undone".to_string());
  set.add(blocked, "never lands".to_string());
  let err = set.commit().expect_err("the second write must fail");

  chmod(&locked, dir_mode);

  assert!(
    matches!(err, WriteError::Io { .. }),
    "the unwind succeeded, so this is an ordinary failure: {err}"
  );
  assert!(
    !landed.exists(),
    "and the file that HAD landed was removed, because it did not exist before"
  );
}

/// The batch either lands entirely or leaves nothing behind -- including the
/// directories it created on the way.
#[cfg(unix)]
#[test]
fn a_failed_batch_leaves_no_directories_it_created() {
  let fx = Fixture::new();
  let locked = fx.path("locked");
  std::fs::create_dir_all(&locked).expect("mkdir");
  let dir_mode = chmod(&locked, 0o555);

  let mut set = WriteSet::new();
  set.add(fx.path("fresh/deep/a.json"), "a".to_string());
  set.add(locked.join("b.json"), "b".to_string());
  let _ = set.commit().expect_err("the second write must fail");

  chmod(&locked, dir_mode);

  assert!(
    !fx.path("fresh").exists(),
    "the whole created chain is gone, not just the file"
  );
}
