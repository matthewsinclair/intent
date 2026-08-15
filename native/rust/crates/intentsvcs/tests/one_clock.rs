//! **There is exactly one clock in this workspace, and it is the store's**
//! (hv, 2026-08-15: time comes from the DB).
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
//! first into the second. All three are gone.
//!
//! **The roster is DISCOVERED, never listed.** Every `src/**/*.rs` in every
//! crate is scanned, so a new file is covered the day it is written and a new
//! crate the day it is added -- the failure mode of a hand-maintained list
//! being that the act which invalidates it (adding a file) is not the act that
//! updates it.

use std::path::{Path, PathBuf};

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
];

/// The ONE file allowed to read a clock -- and it does not read a process
/// clock either; it asks SQLite.
const THE_CLOCK: &str = "crates/intentsvcs/src/store.rs";

fn workspace_root() -> PathBuf {
  // `crates/intentsvcs` -> `native/rust`
  Path::new(env!("CARGO_MANIFEST_DIR"))
    .parent()
    .and_then(Path::parent)
    .expect("the crate sits two levels under the workspace root")
    .to_path_buf()
}

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
fn only_the_store_reads_a_clock() {
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
    if rel == THE_CLOCK {
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
    "time comes from the DB (hv, 2026-08-15): `Store::now` / `Store::today` are the one clock, and these reached for another --\n  {}",
    offenders.join("\n  ")
  );
}

/// The guard is only worth anything if the file it exempts actually holds a
/// clock. An exemption pointing at a file that stopped being the clock would
/// pass forever while the workspace had none.
#[test]
fn the_exempt_file_is_actually_the_clock() {
  let code = code_of(&workspace_root().join(THE_CLOCK));
  assert!(
    code.contains("fn now(") && code.contains("fn today("),
    "{THE_CLOCK} is exempt because it IS the clock; if these moved, move the exemption with them"
  );
  assert!(
    code.contains("'now'"),
    "and it asks SQLite rather than the process: the DB is the clock, not merely the owner of one"
  );
}
