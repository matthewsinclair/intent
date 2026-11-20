//! ST0095 AC-14.2: **ONE FUNCTION ANSWERS WHICH THREAD OR ISSUE OWNS A PATH**,
//! `Project::owner_of`, and every caller asks it.
//!
//! The census for WP-14 found six answers and more: views read canon membership
//! by prefix, organize took the first name under `st/` unchecked (so a v2
//! status bucket came back as a thread), plan split strings, and two of them
//! spelled `intent/` whatever the project configured. These are the answer's
//! own cases: what a thread owns, what an issue owns, what nothing owns, and a
//! configured intent directory.

use crate::common::Fixture;
use intentsvcs::id::{IssueId, Named, ThreadId};
use std::path::Path;

fn thread(id: &str) -> Option<Named> {
  Some(Named::Thread(ThreadId::parse(id).expect("a thread id")))
}

fn issue(id: &str) -> Option<Named> {
  Some(Named::Issue(IssueId::parse(id).expect("an issue id")))
}

#[test]
fn a_thread_owns_its_directory_its_record_and_what_lies_beside_its_record() {
  let project = Fixture::new().project();
  for rel in [
    "intent/st/ST0001",
    "intent/st/ST0001/info.md",
    "intent/st/ST0001/acceptance.md",
    "intent/st/ST0001/WP/01/info.md",
    "intent/st/ST0001/design.md",
    "intent/.canon/st/ST0001.json",
    "intent/.canon/st/ST0001/mangle.sh",
  ] {
    assert_eq!(project.owner_of(Path::new(rel)), thread("ST0001"), "{rel}");
  }
}

#[test]
fn an_issue_owns_its_record_and_its_view() {
  let project = Fixture::new().project();
  for rel in ["intent/.canon/issues/0021.json", "intent/issues/0021.md"] {
    assert_eq!(project.owner_of(Path::new(rel)), issue("0021"), "{rel}");
  }
}

/// A v2 status bucket is legacy's to read, a name that is not an id names
/// nothing, and a file beside a record that is not the record's own spelling
/// is not the record.
#[test]
fn nothing_else_has_an_owner() {
  let project = Fixture::new().project();
  for rel in [
    "intent/steel_threads.md",
    "intent/st/COMPLETED/ST0001/info.md",
    "intent/st/notes/info.md",
    "intent/st/st0001/info.md",
    "intent/.canon/st/ST0001.md",
    "intent/.canon/issues/0021/notes.json",
    "intent/issues/0021.json",
    "intent/whiteboard/dc/wip.md",
    "native/rust/Cargo.toml",
  ] {
    assert_eq!(project.owner_of(Path::new(rel)), None, "{rel}");
  }
}

#[test]
fn an_absolute_path_has_the_owner_its_relative_spelling_has() {
  let fx = Fixture::new();
  let absolute = fx.root().join("intent/st/ST0001/design.md");
  assert_eq!(fx.project().owner_of(&absolute), thread("ST0001"));
}

#[test]
fn a_configured_intent_directory_is_honoured() {
  let fx = Fixture::with_intent_dir("work");
  let project = fx.project();
  assert_eq!(
    project.owner_of(Path::new("work/st/ST0001/design.md")),
    thread("ST0001")
  );
  assert_eq!(
    project.owner_of(Path::new("work/issues/0021.md")),
    issue("0021")
  );
  assert_eq!(
    project.owner_of(Path::new("intent/st/ST0001/design.md")),
    None
  );
}
