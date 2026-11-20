//! Issue 0729: **SYNC'S SCAN, ITS ATTACHMENT TEST AND THE ATTACHMENT DOOR'S
//! PATH CHECK READ THE PROJECT'S OWN INTENT DIRECTORY**, never a spelled
//! `intent/`. With any other `intent_dir`, the scan walked a directory that
//! does not exist, and the door's repo-relative check missed the spelling it
//! exists to catch.

use crate::common::Fixture;
use intentsvcs::project::{PathFault, attachment_path_fault};
use intentsvcs::sync::{FileState, scan};
use std::path::Path;

#[test]
fn the_scan_is_rooted_at_the_configured_intent_directory() {
  let fx = Fixture::with_intent_dir("work");
  fx.write_file("work/wip.md", "# In flight\n");
  let scanned = scan(fx.root(), &[]).expect("the scan runs");
  assert!(
    scanned.iter().any(|entry| entry.path == "work/wip.md"),
    "the configured directory is the corpus: {scanned:?}"
  );
}

#[test]
fn a_non_utf8_attachment_under_the_configured_directory_is_carried_not_residue() {
  let fx = Fixture::with_intent_dir("work");
  let picture = fx.path("work/st/ST0001/picture.bin");
  std::fs::create_dir_all(picture.parent().expect("the thread dir")).expect("mkdir");
  std::fs::write(&picture, [0xff, 0xfe, 0x00, 0x01]).expect("write bytes");
  let scanned = scan(fx.root(), &[]).expect("the scan runs");
  let entry = scanned
    .iter()
    .find(|entry| entry.path == "work/st/ST0001/picture.bin")
    .expect("the attachment is in the corpus");
  assert_ne!(
    entry.state,
    FileState::Unparsed,
    "an attachment claims nothing about its bytes: {entry:?}"
  );
}

#[test]
fn the_door_names_a_path_relative_to_the_configured_directory() {
  let home = Path::new("work/st/ST0001");
  assert_eq!(
    attachment_path_fault(Path::new("work/st/ST0001/notes.md"), home),
    Some(PathFault::RepoRelative {
      corrected: "notes.md".to_string(),
      would_name: "work/st/ST0001/work/st/ST0001/notes.md".to_string(),
    })
  );
  assert_eq!(
    attachment_path_fault(Path::new("intent/st/notes.md"), home),
    None,
    "`intent/` is an ordinary directory name in a project whose intent directory is `work`"
  );
}
