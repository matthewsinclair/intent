//! Claude Code's session file is not a published contract (ST0086 WP-01), so
//! this pins the fields the estate reader relies on against real files,
//! scrubbed: one written by Claude Code 2.1.310 on macOS and one on Ubuntu
//! ARM64, every key kept, the identifying values replaced. A Claude Code that
//! renames or drops one of them turns this red rather than the estate page
//! quietly empty.
//!
//! **THE TWO OSES WRITE `procStart` IN DIFFERENT FORMS**, measured 2026-10-24:
//! UTC ctime on macOS, the kernel's start time in clock ticks since boot on
//! Linux. Each fixture is read on every OS, since the parse is the same; only
//! the liveness read that compares against it differs.

use std::path::Path;

use intentsvcs::estate::{PINNED, parse_session};

const MACOS: &str = "tests/fixtures/estate/claude-session-2.1.310-macos.json";
const LINUX: &str = "tests/fixtures/estate/claude-session-2.1.310-linux.json";

fn fixture(rel: &str) -> String {
  std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)).expect("the fixture")
}

#[test]
fn the_macos_session_file_parses_with_every_pinned_field() {
  let session =
    parse_session(Path::new("/c/sessions/12345.json"), &fixture(MACOS)).expect("parses");
  assert_eq!(session.pid, 12345);
  assert_eq!(session.proc_start, "Sat Oct 24 05:28:00 2026");
  assert_eq!(session.session_id, "00000000-0000-4000-8000-000000000000");
  assert_eq!(session.cwd, Path::new("/Users/someone/Devel/prj/Example"));
  assert_eq!(session.name, "example-cc");
  assert_eq!(session.status, "busy");
  assert_eq!(session.started_at, "2026-10-24T05:28:01.447Z");
  assert_eq!(session.updated_at, "2026-10-24T13:56:13.388Z");
}

#[test]
fn the_linux_session_file_parses_with_its_tick_count_procstart() {
  let session =
    parse_session(Path::new("/c/sessions/692155.json"), &fixture(LINUX)).expect("parses");
  assert_eq!(session.pid, 692155);
  assert_eq!(session.proc_start, "12448223");
  assert_eq!(session.cwd, Path::new("/home/someone/Devel/prj/Example"));
  assert_eq!(session.status, "busy");
  assert_eq!(session.started_at, "2026-10-24T05:26:24.834Z");
}

#[test]
fn removing_any_pinned_field_names_it_in_either_form() {
  for (rel, pid) in [(MACOS, 12345), (LINUX, 692155)] {
    for key in PINNED {
      let mut doc: serde_json::Value = serde_json::from_str(&fixture(rel)).unwrap();
      doc.as_object_mut().unwrap().remove(*key);
      let err = parse_session(
        Path::new(&format!("/c/sessions/{pid}.json")),
        &doc.to_string(),
      )
      .unwrap_err();
      assert_eq!(err.why, format!("missing `{key}`"), "{rel}");
      assert_eq!(err.pid, Some(pid), "{rel}");
    }
  }
}
