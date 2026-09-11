//! `claude skills list` names a unit on disk that the roster does not (issue
//! 0150).
//!
//! The listing used to enumerate the ROSTER -- every unit this install carries
//! -- and report each one's install state. A directory under the target that
//! the roster does not name was not a row, so it was never examined and never
//! reported: the only instrument that looks at `~/.claude/skills/` could not
//! see what it was asked about. Two such directories are made here the way
//! they arise in the field -- the empty directory `uninstall` leaves by ruling
//! (`0218`) once canon has retired the skill, and a skill copied in by hand --
//! and the listing must name both, and say which of them the agent can load.

use std::path::Path;
use std::process::Command;

fn run(home: &Path, args: &[&str]) -> (i32, String) {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .env("HOME", home)
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("spawn intent");
  let mut text = String::from_utf8_lossy(&out.stdout).to_string();
  text.push_str(&String::from_utf8_lossy(&out.stderr));
  (out.status.code().unwrap_or(-1), text)
}

/// The listing's lines whose first column is `name`.
fn rows<'a>(listing: &'a str, name: &str) -> Vec<&'a str> {
  listing
    .lines()
    .filter(|l| l.split_whitespace().next() == Some(name))
    .collect()
}

#[test]
fn a_skill_directory_the_roster_does_not_name_is_listed() {
  let home = tempfile::tempdir().expect("temp HOME");
  let h = home.path();

  // A roster unit installed through the verb: the control that the new arm
  // does not report a unit the roster already names a second time.
  let (rc, fresh) = run(h, &["claude", "skills", "list"]);
  assert_eq!(rc, 0, "`claude skills list` failed: {fresh}");
  let canon = fresh
    .lines()
    .find_map(|l| l.split_whitespace().next())
    .expect("the install carries at least one skill")
    .to_string();
  let (rc, out) = run(h, &["claude", "skills", "install", &canon]);
  assert_eq!(rc, 0, "install {canon} failed: {out}");

  let target = h.join(".claude/skills");
  std::fs::create_dir_all(target.join("retired-husk")).expect("husk");
  std::fs::create_dir_all(target.join("hand-made")).expect("hand-made");
  std::fs::write(target.join("hand-made/SKILL.md"), "# hand-made\n").expect("SKILL.md");

  let (rc, listing) = run(h, &["claude", "skills", "list"]);
  assert_eq!(rc, 0, "`claude skills list` failed: {listing}");

  let husk = rows(&listing, "retired-husk");
  assert_eq!(husk.len(), 1, "the husk must be listed once:\n{listing}");
  assert!(
    husk[0].contains("inert") && husk[0].contains("unlisted"),
    "the husk must read as inert and unlisted: {}",
    husk[0]
  );

  let hand = rows(&listing, "hand-made");
  assert_eq!(
    hand.len(),
    1,
    "the hand-made skill must be listed once:\n{listing}"
  );
  assert!(
    hand[0].contains("installed") && hand[0].contains("unlisted"),
    "the hand-made skill must read as installed and unlisted: {}",
    hand[0]
  );

  let known = rows(&listing, &canon);
  assert_eq!(
    known.len(),
    1,
    "{canon} must be listed once, by the roster:\n{listing}"
  );
  assert!(
    !known[0].contains("unlisted"),
    "{canon} is in the roster and must not read as unlisted: {}",
    known[0]
  );
}
