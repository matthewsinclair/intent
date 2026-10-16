//! Issue 0601, (a1) -- **`claude upgrade --apply` puts the prompt gate's
//! releaser on a machine that lacks it, says so, and `doctor` says when the
//! gate is wired with nothing to release it.**
//!
//! The arm is the sequence a fresh user runs, `init`, `bootstrap`, `claude
//! upgrade --apply`, under an empty `HOME`: before 0601 that left a gate in
//! `.claude/settings.json` and no `/in-session` anywhere on the machine. The
//! library tests in intentsvcs pin each disposition of the step; this pins the
//! report lines and doctor's advisory through the shipped binary.
//!
//! **UNDER ITS OWN `HOME`**, a tempdir, so the install lands there and never in
//! the developer's `~/.claude` or the fixture HOME other arms share.

use std::path::Path;
use std::process::Command;

fn run(args: &[&str], cwd: &Path, home: &Path) -> (String, String, i32) {
  let out = crate::common::intent()
    .args(args)
    .current_dir(cwd)
    .env("HOME", home)
    .env_remove("XDG_CONFIG_HOME")
    .env_remove("XDG_DATA_HOME")
    .env_remove("XDG_STATE_HOME")
    .env_remove("XDG_RUNTIME_DIR")
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run the v3 binary");
  (
    String::from_utf8_lossy(&out.stdout).into_owned(),
    String::from_utf8_lossy(&out.stderr).into_owned(),
    out.status.code().unwrap_or(-1),
  )
}

const ADVISORY: &str = "wires the prompt gate, and this machine has no";

#[test]
fn a_fresh_machine_gets_in_session_from_claude_upgrade_and_doctor_names_its_absence() {
  let project = tempfile::tempdir().expect("tempdir");
  let home = tempfile::tempdir().expect("tempdir");
  let skill = home.path().join(".claude/skills/in-session");
  let releaser = skill.join("scripts/release-gate.sh");
  let git = Command::new("git")
    .args(["init", "-q"])
    .current_dir(project.path())
    .status()
    .expect("run git init");
  assert!(git.success(), "git init failed in the fixture");
  let (_, err, code) = run(&["init", "Team"], project.path(), home.path());
  assert_eq!(code, 0, "init failed: {err}");
  let (_, err, code) = run(&["bootstrap"], project.path(), home.path());
  assert_eq!(code, 0, "bootstrap failed: {err}");

  // The dry run says what it would install and installs nothing.
  let (out, err, code) = run(&["claude", "upgrade"], project.path(), home.path());
  assert_eq!(code, 0, "the dry run failed: {err}");
  assert!(
    out.contains(&format!("would install: {}", skill.display())),
    "the dry run did not name the skill it would install:\n{out}"
  );
  assert!(!skill.exists(), "the dry run installed the skill");

  // The run installs it, outside the project, and names it.
  let (out, err, code) = run(
    &["claude", "upgrade", "--apply"],
    project.path(),
    home.path(),
  );
  assert_eq!(code, 0, "claude upgrade --apply failed: {out}{err}");
  assert!(
    out.contains(&format!("installed: {}", skill.display())),
    "the run did not report the install:\n{out}"
  );
  assert!(releaser.is_file(), "no releaser at {}", releaser.display());
  let (out, _, _) = run(&["doctor", "--verbose"], project.path(), home.path());
  assert!(
    !out.contains(ADVISORY),
    "doctor reported a missing releaser that the upgrade just installed:\n{out}"
  );

  // A second run is settled.
  let (out, err, code) = run(
    &["claude", "upgrade", "--apply"],
    project.path(),
    home.path(),
  );
  assert_eq!(code, 0, "the second run failed: {out}{err}");
  assert!(
    out.contains(&format!("unchanged: {}", skill.display())),
    "the second run did not report the releaser as settled:\n{out}"
  );

  // With the skill gone, doctor names the gap and the verb, and exits 0: the
  // advisory is about this machine, not the project. An uncounted advisory is
  // printed under `--verbose`; the default run only counts it.
  std::fs::remove_dir_all(&skill).expect("remove the installed skill");
  let (out, err, code) = run(&["doctor", "--verbose"], project.path(), home.path());
  assert!(
    out.contains(ADVISORY) && out.contains("intent claude skills install in-session"),
    "doctor did not name the missing releaser and its install verb:\n{out}{err}"
  );
  assert_eq!(code, 0, "doctor counted a machine advisory: {out}{err}");
}

/// The line `/in-session` runs to release the gate, exactly as the installed
/// SKILL.md must carry it (issue 0604).
const RELEASE_LINE: &str =
  "bash \"${CLAUDE_CONFIG_DIR:-$HOME/.claude}/skills/in-session/scripts/release-gate.sh\"";

/// Issue 0604: with `CLAUDE_CONFIG_DIR` set, `claude skills install` writes the
/// skill where Claude Code loads it, not under `$HOME/.claude`, and the
/// installed SKILL.md carries the release line byte for byte. The installed
/// file is what Claude Code reads; its own rendering at injection time is not
/// drivable from here, and `skill_renderer_trap.bats` guards the tokens it
/// strips.
#[test]
fn claude_config_dir_moves_the_skill_install_and_the_release_line_arrives_whole() {
  let cwd = tempfile::tempdir().expect("tempdir");
  let home = tempfile::tempdir().expect("tempdir");
  let config = tempfile::tempdir().expect("tempdir");
  let out = crate::common::intent()
    .args(["claude", "skills", "install", "in-session"])
    .current_dir(cwd.path())
    .env("HOME", home.path())
    .env("CLAUDE_CONFIG_DIR", config.path())
    .stdin(testkit::lifeline_for(&["claude", "skills", "install"]))
    .output()
    .expect("run the v3 binary");
  let said = format!(
    "{}{}",
    String::from_utf8_lossy(&out.stdout),
    String::from_utf8_lossy(&out.stderr)
  );
  assert!(out.status.success(), "the install failed: {said}");

  let skill = config.path().join("skills/in-session");
  assert!(
    skill.join("scripts/release-gate.sh").is_file(),
    "no releaser under CLAUDE_CONFIG_DIR:\n{said}"
  );
  assert!(
    !home.path().join(".claude/skills/in-session").exists(),
    "the install also wrote under HOME"
  );
  let body = std::fs::read_to_string(skill.join("SKILL.md")).expect("read the installed SKILL.md");
  assert!(
    body.lines().any(|line| line == RELEASE_LINE),
    "the installed SKILL.md does not carry the release line whole:\n{body}"
  );
}
