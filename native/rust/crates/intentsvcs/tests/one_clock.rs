//! **There is no clock in this workspace but the daemon log's** (hv, 2026-08-15:
//! time comes from the DB; hv, 2026-09-15: the daemon log is exempt, for the
//! reason ST0056's D42 records).
//!
//! It said "exactly one clock, and it is the store's", which was the right step
//! and not the destination. A `Store::now()` returning `SELECT strftime('now')`
//! still hands a time to the application, which then holds it across a gap
//! before writing it -- better provenance, same confection. **A record is
//! stamped BY the write that creates it**, so nothing needs to ask.
//!
//! The rule needs a mechanical guard rather than a note, because a second
//! clock is the easiest thing in the world to add by accident: reaching for
//! `OffsetDateTime::now_utc()` is the obvious way to stamp anything, it
//! compiles, it produces a plausible value, and nothing downstream can tell
//! the difference. That is the whiteboard's local-versus-UTC failure one layer
//! down -- a stamp from the wrong clock is indistinguishable from a right one
//! by inspection, so the only place it can be caught is where it is written.
//!
//! Before this rule there were three: the CLI's `today()`, `Envelope::new`'s
//! `now_utc()`, and the caller-supplied `FacadeContext.today` that carried the
//! first into the second. They were collapsed into `Store::now()` /
//! `Store::today()`, and those are now gone too. **Four positions, each better
//! sourced than the last, and only the fourth removes the gap.**
//!
//! **The roster is DISCOVERED, never listed.** Every `src/**/*.rs` in every
//! crate is scanned, so a new file is covered the day it is written and a new
//! crate the day it is added -- the failure mode of a hand-maintained list
//! being that the act which invalidates it (adding a file) is not the act that
//! updates it.
//!
//! ST0069 AT-14.11 cites this file: every whiteboard stamp is the store's,
//! because nothing in the workspace reads a clock except the daemon log's
//! writer, and that writes no record.

use std::path::{Path, PathBuf};
use testkit::workspace_root;

/// Every way to reach a clock that is not the store's. `now_utc` and
/// `SystemTime` are the direct routes; `Instant` measures elapsed time, which
/// is a different question but still an ambient reading.
///
/// **`OffsetDateTime` alone is deliberately NOT banned, and the distinction is
/// the rule rather than a loophole.** `sync.rs` converts a file's `mtime` into
/// RFC 3339 -- it reads a property of a file that the filesystem already
/// recorded, and never asks what time it is. Data about when something
/// happened is not a clock reading. Banning the type wholesale would forbid
/// reading a timestamp that already exists, which is not what "time comes from
/// the DB" means; every needle here is a `::now`, because asking is the act
/// being ruled on.
const CLOCK: &[&str] = &[
  "OffsetDateTime::now",
  "SystemTime::now",
  "Instant::now",
  "Local::now",
  "Utc::now",
  "chrono::",
  // **The store clock, and its absence here was a real hole.** When
  // `Store::now()` existed, every needle above was a call into an external time
  // API and none of them matched a `SELECT strftime(...)` -- so the one clock
  // this workspace actually had was invisible to the guard watching for clocks
  // (vc, 2026-08-15). A standalone SELECT is the banned shape: it hands a time
  // to the application, which then holds it across a gap before writing it.
  // `strftime` INSIDE an INSERT or an UPDATE is the ratified mechanism and is
  // deliberately not matched -- there the stamp and the write are one
  // operation, which is the whole of D42.
  "SELECT strftime(",
];

/// **Files allowed to read a clock: ONE, the daemon log's writer, by ruling.**
///
/// It held `store.rs` until the store clock was deleted. That exemption was
/// correct under the model of the morning -- one well-sourced clock beat three
/// process clocks -- and D42 superseded it: a function returning a time that
/// went through no RECORD is a confection with better provenance. **Time is a
/// property of a write**, so no Rust file needs to ask what time it is, and the
/// exemption shrank to zero rather than moving.
///
/// **It holds one file again, and not because a record needs a time.** hv
/// ruled on 2026-09-15 that the daemon log is exempt, and the reason is
/// recorded once, at ST0056's D42: the log is an operational stream that must
/// write when the store cannot (issue `0321`). The entry is a file rather than
/// a crate, so every other file in `intentd` is still scanned.
const EXEMPT: &[&str] = &["crates/intentd/src/daemon_log.rs"];

/// Every `.rs` under every crate's `src/` AND `tests/`, discovered by walking.
///
/// **`tests/` is in scope on hv's instruction, and it is where the rule is
/// most likely to be broken.** A clock in a fixture looks harmless -- nobody
/// ships a test -- but a fixture is exactly where "I only need a time for
/// setup" gets written, and the value then becomes the thing the assertion
/// trusts. A test that confects a time proves the system agrees with a
/// confection.
///
/// It bans ASKING, not dates. A fixture may still author a literal like
/// `"2026-08-14"`, because under D42 those arrive through the RESTORE door --
/// carrying a recorded stamp is preserving history, not confecting it.
fn sources(root: &Path) -> Vec<PathBuf> {
  fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
      return;
    };
    for entry in entries.flatten() {
      let path = entry.path();
      if path.is_dir() {
        walk(&path, out);
      } else if path.extension().is_some_and(|e| e == "rs") {
        out.push(path);
      }
    }
  }
  let mut out = Vec::new();
  let crates = root.join("crates");
  for entry in std::fs::read_dir(&crates)
    .expect("read the crates dir")
    .flatten()
  {
    for dir in ["src", "tests"] {
      let d = entry.path().join(dir);
      if d.is_dir() {
        walk(&d, &mut out);
      }
    }
  }
  out.sort();
  out
}

/// Strip comments, so this file's own prose and store.rs's explanation of why
/// it owns the clock cannot trip the scan.
fn code_of(path: &Path) -> String {
  std::fs::read_to_string(path)
    .unwrap_or_default()
    .lines()
    .filter(|l| {
      let t = l.trim_start();
      !t.starts_with("//")
    })
    .collect::<Vec<_>>()
    .join("\n")
}

#[test]
fn nothing_but_the_daemon_log_reads_a_clock() {
  let root = workspace_root();
  let files = sources(&root);
  assert!(
    files.len() > 10,
    "precondition: the walk found the workspace ({} files)",
    files.len()
  );

  let mut offenders = Vec::new();
  for path in &files {
    let rel = path
      .strip_prefix(&root)
      .expect("under the root")
      .to_string_lossy()
      .replace('\\', "/");
    if EXEMPT.contains(&rel.as_str()) {
      continue;
    }
    // This file lists the banned needles in code (a `const`, not a comment),
    // so it would always find itself.
    if rel.ends_with("tests/one_clock.rs") {
      continue;
    }
    let code = code_of(path);
    for needle in CLOCK {
      if code.contains(needle) {
        offenders.push(format!("{rel} reaches for {needle}"));
      }
    }
  }

  assert!(
    offenders.is_empty(),
    "time comes from the DB (hv, 2026-08-15). There is no clock in this workspace but the daemon log's writer, D42's one exemption: a record is stamped BY the write that creates it, so nothing needs to ask. These asked --\n  {}",
    offenders.join("\n  ")
  );
}

/// **The exemption list names the daemon log's writer and nothing else, and the
/// clock every record uses lives in the SCHEMA.**
///
/// Inverted rather than deleted, and the reason is the failure it used to
/// guard: an exemption that stops describing reality passes forever. The old
/// form asserted `store.rs` still held `fn now()` and `fn today()`, so it would
/// have failed the build the moment those were deleted -- **a guard enforcing
/// the superseded model, whose failure text argued for keeping the thing being
/// removed** (vc, who caught it before it was hit).
///
/// **The one entry is pinned in both directions** (hv, 2026-09-15, recorded at
/// ST0056's D42; issue `0321`). A second file cannot join it without this test
/// changing, and the file it names must still be the one that reads the clock,
/// so the exemption cannot outlive its reason. Every record is still stamped by
/// a column DEFAULT.
#[test]
fn only_the_daemon_log_is_exempt_and_the_stamping_lives_in_the_schema() {
  assert_eq!(
    EXEMPT,
    ["crates/intentd/src/daemon_log.rs"],
    "the clock ban exempts the daemon log's writer and nothing else (hv, 2026-09-15, for the reason ST0056's D42 records). Another file wants its own ruling, with its reason recorded at D42, rather than an entry here"
  );
  assert!(
    code_of(&workspace_root().join(EXEMPT[0])).contains("OffsetDateTime::now"),
    "`{}` is exempt from the clock ban and is missing or no longer reads the clock, so the exemption has outlived its reason: remove the entry",
    EXEMPT[0]
  );

  // Where the clock went. Every record-timestamp column is filled by SQLite as
  // part of the write, which is why no Rust file has to ask.
  let defaults = intentsvcs::store::RECORD_TIMESTAMPS
    .iter()
    .filter(|c| {
      intentsvcs::store::DDL.contains(&format!(
        "{c} TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))"
      ))
    })
    .count();
  assert_eq!(
    defaults,
    intentsvcs::store::RECORD_TIMESTAMPS.len(),
    "every record-timestamp column must be stamped by a column DEFAULT -- if one is not, the \
     application is filling it and the clock came back somewhere this scan cannot see"
  );
}
