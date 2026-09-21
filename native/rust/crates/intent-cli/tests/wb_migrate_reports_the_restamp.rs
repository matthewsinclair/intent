//! **`wb migrate` says that it restamped the node it carried (issue 0497).**
//!
//! The store writes `heartbeat_at` at the carry and keeps the header's claim in
//! `authored_at` (`Store::carry_header`, AC-14.4 against AC-14.9). That is
//! ruled and it stays. What was missing is that nothing a reader sees said so:
//! on Riffle's carry on 2026-09-21 a board paused since 2026-08-11 rendered as
//! live that morning, and the migrate report was silent about it.
//!
//! Driven through the binary, because the line is the CLI's rendering of the
//! facade's value and a facade arm would not see the rendering.

use std::path::Path;

/// A board whose header claims an old heartbeat, the Riffle shape.
const BOARD: &str = "---\nnode: dc\nname: DevX Claude\nrole: worker\nsession_id: none\nheartbeat_at: 2026-08-11T11:06Z\nstatus: paused\nfocus: \"idle since August\"\nclaims: []\n---\n\n# DevX Claude (dc)\n\n## DOING\n\n- The one thing on the board.\n";

/// A peer whose header claims no heartbeat at all, the negative control.
const PEER: &str = "---\nnode: vc\nname: Validation Claude\nrole: validation\nstatus: active\n---\n\n# Validation Claude (vc)\n\n## TODO\n\n- A queued thing.\n";

fn run(cwd: &Path, args: &[&str]) -> (String, i32) {
  let out = crate::common::intent()
    .args(args)
    .current_dir(cwd)
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run intent");
  (
    format!(
      "{}{}",
      String::from_utf8_lossy(&out.stdout),
      String::from_utf8_lossy(&out.stderr)
    ),
    out.status.code().unwrap_or(-1),
  )
}

/// The value after `label` on the first line starting with it.
fn line_value<'a>(text: &'a str, label: &str) -> Option<&'a str> {
  text
    .lines()
    .find_map(|l| l.trim_start().strip_prefix(label))
    .map(str::trim)
}

fn registered() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let (text, code) = run(dir.path(), &["init", "restamp"]);
  assert_eq!(code, 0, "init: {text}");
  for (node, board) in [("dc", BOARD), ("vc", PEER)] {
    let home = dir.path().join("intent/whiteboard").join(node);
    std::fs::create_dir_all(&home).expect("the node's directory");
    std::fs::write(home.join("wip.md"), board).expect("the board");
  }
  let (text, code) = run(dir.path(), &["wb", "register"]);
  assert_eq!(code, 0, "register: {text}");
  dir
}

#[test]
fn the_report_names_the_authored_stamp_and_the_carry_s_and_says_none_when_the_header_had_none() {
  let dir = registered();

  let (text, code) = run(dir.path(), &["wb", "migrate", "dc"]);
  assert_eq!(code, 0, "migrate dc: {text}");
  let line = line_value(&text, "heartbeat:")
    .unwrap_or_else(|| panic!("the report must carry a heartbeat line: {text}"));
  let (authored, carry) = line
    .strip_suffix(" at carry")
    .and_then(|l| l.split_once(" as authored, "))
    .unwrap_or_else(|| {
      panic!("the line's shape is `<authored> as authored, <carry> at carry`: {line}")
    });
  assert_eq!(
    authored, "2026-08-11T11:06Z",
    "the header's claim is reported verbatim"
  );

  // The carry's value is the store's, so it is read back from the store rather
  // than predicted from a clock this test would have to read.
  let (shown, code) = run(dir.path(), &["wb", "show", "dc"]);
  assert_eq!(code, 0, "show dc: {shown}");
  assert_eq!(
    Some(carry),
    line_value(&shown, "heartbeat"),
    "the reported carry stamp is the heartbeat the store now holds: {shown}"
  );
  assert_ne!(
    authored, carry,
    "the restamp is the thing being reported, so the two values differ"
  );

  // THE NEGATIVE CONTROL: a header that claimed no heartbeat is reported as
  // claiming none, not given one.
  let (text, code) = run(dir.path(), &["wb", "migrate", "vc"]);
  assert_eq!(code, 0, "migrate vc: {text}");
  let line = line_value(&text, "heartbeat:")
    .unwrap_or_else(|| panic!("the report must carry a heartbeat line: {text}"));
  assert!(
    line.starts_with("none as authored, ") && line.ends_with(" at carry"),
    "a header with no stamp reads as none authored: {line}"
  );
}
