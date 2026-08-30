//! Issue `0152`: the contention wait is OURS, and the only thing testable is
//! that we still set it.
//!
//! **THIS IS A SOURCE-LEVEL TEST ON PURPOSE, AND THE REASON IS THE SAME ONE
//! THAT MADE THE `rename` WITNESS STRUCTURAL** (vc, 2026-08-30, applying cc's
//! own N2 ruling back at them): *a property enforced by a syscall cannot be
//! witnessed by testing its outcome, because the outcome holds under any
//! implementation that still calls it; what is testable is that you still call
//! it.* Here the property is enforced by a DEPENDENCY DEFAULT, the outcome
//! holds under any implementation that still equals it, and what is testable is
//! that we still set it.
//!
//! **EVERY BEHAVIOURAL ROUTE IS CLOSED, AND NOT FOR WANT OF TRYING.**
//! `busy_timeout` is PER-CONNECTION and is not persisted in the database file,
//! so no second connection can read back what `Store::open` set -- the only
//! observation available is contending for the write lock and TIMING the wait.
//! And a timed wait cannot distinguish our 5000 from `rusqlite`'s 5000: the two
//! are behaviourally identical by construction, **so a timing test would pass
//! with the line deleted.** That is the vacuous instrument, and building one
//! here would have put a false green in the file whose whole subject is a
//! default nobody could see.
//!
//! # The deletion this fires on, which is not malice or accident
//!
//! Somebody reads the line, observes CORRECTLY that it sets the value to what
//! `rusqlite` already supplies, concludes it is redundant, and removes it.
//! **That change is behaviourally invisible today.** It becomes visible only
//! when an upstream bump moves the default -- at which point nothing connects
//! the new behaviour to the deletion that allowed it. This test fires on the
//! deletion itself, which is the only moment the cause and the effect are in
//! the same place.

use std::path::PathBuf;

/// The module that must keep choosing the value.
fn store_source() -> (PathBuf, String) {
  let path = testkit::workspace_root().join("crates/intentsvcs/src/store.rs");
  let body = std::fs::read_to_string(&path).expect("store.rs is readable");
  (path, body)
}

#[test]
fn the_contention_wait_is_set_rather_than_inherited() {
  let (path, body) = store_source();

  assert!(
    body.contains("const BUSY_TIMEOUT_MS"),
    "{}: the contention wait no longer has a named constant. Issue 0152 is about a number nobody could see, and an unnamed literal is the same defect wearing our own spelling",
    path.display()
  );
  assert!(
    body.contains(r#"pragma_update(None, "busy_timeout", Self::BUSY_TIMEOUT_MS)"#),
    "{}: `Store::open` no longer SETS busy_timeout from the constant. Deleting that call is behaviourally invisible today -- rusqlite supplies the same 5000 -- and becomes visible only when an upstream bump moves the default, by which time nothing connects the two. See issue 0152",
    path.display()
  );
}

/// The constant is only meaningful while it is the one the open path uses.
///
/// **A SECOND ASSERTION BECAUSE THE FIRST CAN BE SATISFIED BY A CORPSE**: a
/// declared constant that nothing reads would pass a `contains` check on the
/// declaration alone, and would be exactly the state this issue describes --
/// a value that looks chosen and is not applied.
#[test]
fn the_constant_is_the_one_the_open_path_applies() {
  let (path, body) = store_source();
  let declared = body
    .lines()
    .find(|l| l.trim_start().starts_with("const BUSY_TIMEOUT_MS"))
    .unwrap_or_else(|| panic!("{}: no BUSY_TIMEOUT_MS declaration", path.display()));
  let value: String = declared.chars().filter(|c| c.is_ascii_digit()).collect();
  assert!(
    !value.is_empty(),
    "{}: BUSY_TIMEOUT_MS declares no numeric value: {declared}",
    path.display()
  );
  assert_eq!(
    body.matches("Self::BUSY_TIMEOUT_MS").count(),
    1,
    "{}: the constant should be applied in exactly one place -- the open path. More than one reader means the contention wait has acquired a second home, which is the shape 0152 exists to prevent",
    path.display()
  );
}
