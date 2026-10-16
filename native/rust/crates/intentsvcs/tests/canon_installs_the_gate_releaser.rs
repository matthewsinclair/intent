//! **`claude upgrade --apply` wired a prompt gate on a machine where nothing
//! could release it** (issue 0601, (a1)).
//!
//! The settings file wires a cooperative gate: it blocks a session's first
//! prompt until `/in-session` runs, and `/in-session` is a skill. Nothing the
//! upgrade did put that skill on the machine, so a fresh user's first session
//! was locked with no way out from inside it. hv ruled the strict gate the
//! default for a fresh user, so the upgrade installs the skill, through the
//! installer `intent claude skills install` uses, BEFORE it writes the
//! settings.
//!
//! Every arm runs against a temp skills directory and manifest, so no arm can
//! touch the operator's `~/.claude`.

use std::path::{Path, PathBuf};

use intentsvcs::canon::{self, GATE_RELEASER, GATE_SKILL, Releaser};
use intentsvcs::payload::{Kind, Payload};

/// A machine: a skills directory and a manifest, both temporary.
struct Machine {
  dir: tempfile::TempDir,
}

impl Machine {
  fn new() -> Self {
    Self {
      dir: tempfile::tempdir().expect("machine tempdir"),
    }
  }

  fn skills(&self) -> PathBuf {
    self.dir.path().join("claude/skills")
  }

  fn manifest(&self) -> PathBuf {
    self
      .dir
      .path()
      .join("data/intent/skills/installed-skills.v3.json")
  }

  fn releaser(&self) -> PathBuf {
    self.skills().join(GATE_SKILL).join(GATE_RELEASER)
  }

  /// The payload, whose canon is `install`'s.
  fn payload(&self, install: &Path) -> Payload {
    Payload::new(Kind::Skills, install, None, self.skills(), self.manifest())
  }
}

fn hooks_dir(fx: &crate::common::Fixture) -> PathBuf {
  let hooks = fx.root().join(".git/hooks");
  std::fs::create_dir_all(&hooks).expect("mkdir hooks");
  hooks
}

fn apply(fx: &crate::common::Fixture, opts: canon::Options, skills: &Payload) -> canon::Applied {
  fx.git_init();
  let hooks = hooks_dir(fx);
  let project = fx.project();
  canon::apply(
    fx.root(),
    &testkit::repo_root(),
    project.config(),
    &crate::common::ctx(),
    Some(&hooks),
    opts,
    Some(skills),
  )
  .expect("canon apply")
}

/// **THE DEFECT.** A machine with no `in-session` gets it, its releaser on
/// disk and its manifest naming it, and the settings that wire the gate are
/// written in the same run.
#[test]
fn a_machine_without_in_session_gets_it_before_the_gate_is_wired() {
  let fx = crate::common::Fixture::new();
  let machine = Machine::new();
  let skills = machine.payload(&testkit::repo_root());

  let applied = apply(&fx, canon::Options::default(), &skills);

  assert_eq!(
    applied.releaser,
    Some(Releaser::Installed {
      dir: machine.skills().join(GATE_SKILL)
    })
  );
  assert!(
    machine.releaser().is_file(),
    "the step reported an install and left no releaser at {}",
    machine.releaser().display()
  );
  let manifest = std::fs::read_to_string(machine.manifest()).expect("the manifest was not written");
  assert!(
    manifest.contains(&format!("\"{GATE_SKILL}\"")),
    "the manifest does not name {GATE_SKILL}, so skills sync and uninstall cannot see it: {manifest}"
  );
  let settings = fx.root().join(".claude/settings.json");
  assert!(
    applied.written.contains(&settings),
    "the settings were not written beside the install"
  );
}

/// **A SECOND RUN SAYS IT IS SETTLED** and writes nothing outside the project.
#[test]
fn a_second_run_reports_the_releaser_present() {
  let fx = crate::common::Fixture::new();
  let machine = Machine::new();
  let skills = machine.payload(&testkit::repo_root());
  apply(&fx, canon::Options::default(), &skills);
  let manifest = std::fs::read(machine.manifest()).expect("first run's manifest");

  let applied = apply(&fx, canon::Options::default(), &skills);

  assert_eq!(
    applied.releaser,
    Some(Releaser::Present {
      dir: machine.skills().join(GATE_SKILL)
    })
  );
  assert_eq!(
    std::fs::read(machine.manifest()).expect("manifest"),
    manifest,
    "a settled run rewrote the manifest"
  );
}

/// **`--skip-settings` DECLINES THE GATE, SO IT INSTALLS NOTHING.** The skill
/// exists to release a gate this run does not wire.
#[test]
fn skip_settings_installs_nothing() {
  let fx = crate::common::Fixture::new();
  let machine = Machine::new();
  let skills = machine.payload(&testkit::repo_root());

  let applied = apply(
    &fx,
    canon::Options {
      skip_settings: true,
      ..canon::Options::default()
    },
    &skills,
  );

  assert_eq!(applied.releaser, None);
  assert!(
    !machine.dir.path().join("claude").exists(),
    "--skip-settings wrote under the machine's skills directory"
  );
  assert!(
    !machine.manifest().exists(),
    "--skip-settings wrote a manifest"
  );
}

/// **A DRY RUN SAYS IT WOULD INSTALL, AND DOES NOT.**
#[test]
fn report_mode_reports_the_install_and_writes_nothing() {
  let fx = crate::common::Fixture::new();
  let machine = Machine::new();
  let skills = machine.payload(&testkit::repo_root());

  let applied = apply(
    &fx,
    canon::Options {
      report: true,
      ..canon::Options::default()
    },
    &skills,
  );

  assert_eq!(
    applied.releaser,
    Some(Releaser::Installed {
      dir: machine.skills().join(GATE_SKILL)
    })
  );
  assert!(
    !machine.dir.path().join("claude").exists(),
    "a dry run installed the skill"
  );
}

/// **AN INSTALLED `in-session` WITHOUT ITS RELEASER IS HELD, NEVER FORCED.**
/// What is there is the operator's; the step names it and leaves it alone.
#[test]
fn an_in_session_without_its_releaser_is_held_untouched() {
  let fx = crate::common::Fixture::new();
  let machine = Machine::new();
  let theirs = machine.skills().join(GATE_SKILL);
  std::fs::create_dir_all(&theirs).expect("mkdir their in-session");
  std::fs::write(theirs.join("SKILL.md"), "their own in-session\n").expect("seed SKILL.md");
  let skills = machine.payload(&testkit::repo_root());

  let applied = apply(&fx, canon::Options::default(), &skills);

  assert_eq!(
    applied.releaser,
    Some(Releaser::Held {
      dir: theirs.clone()
    })
  );
  assert_eq!(
    std::fs::read_to_string(theirs.join("SKILL.md")).expect("their SKILL.md"),
    "their own in-session\n",
    "the held skill was overwritten"
  );
  assert!(
    !machine.releaser().exists(),
    "the held skill gained a releaser"
  );
}

/// **A FAILED INSTALL IS REPORTED AND DOES NOT ABORT THE REST OF CANON**: the
/// gate fails open without its releaser (0601 (b)), so the settings are still
/// worth writing. Driven with a canon that carries no `in-session`.
#[test]
fn a_failed_install_is_reported_and_the_settings_are_still_written() {
  let fx = crate::common::Fixture::new();
  let machine = Machine::new();
  let empty_install = tempfile::tempdir().expect("an install with no skills");
  let skills = machine.payload(empty_install.path());

  let applied = apply(&fx, canon::Options::default(), &skills);

  match &applied.releaser {
    Some(Releaser::Failed { cause }) => assert!(
      cause.contains(GATE_SKILL),
      "the failure does not name the skill: {cause}"
    ),
    other => panic!("expected a reported failure, got {other:?}"),
  }
  assert!(
    applied
      .written
      .contains(&fx.root().join(".claude/settings.json")),
    "a failed install aborted the settings write"
  );
}

/// **THE CONTROL: A HELD SETTINGS FILE THAT DOES NOT WIRE THE GATE ASKS FOR NO
/// SKILL.** It is the project's own file, and nothing in it needs releasing.
#[test]
fn a_held_settings_file_without_the_gate_installs_nothing() {
  let fx = crate::common::Fixture::new();
  let machine = Machine::new();
  let settings = fx.root().join(".claude/settings.json");
  std::fs::create_dir_all(settings.parent().expect("parent")).expect("mkdir .claude");
  std::fs::write(&settings, "{\n  \"model\": \"theirs\"\n}\n").expect("seed their settings");
  let skills = machine.payload(&testkit::repo_root());

  let applied = apply(&fx, canon::Options::default(), &skills);

  assert!(
    applied.held.contains(&settings),
    "their settings were not held"
  );
  assert_eq!(applied.releaser, None);
  assert!(
    !machine.manifest().exists(),
    "a skill was installed for a gate nobody wired"
  );
}

/// **DOCTOR'S PATH IS THE INSTALLER'S PATH.** Doctor has no payload to ask, so it
/// joins the skills directory itself through `releaser_under`; this pins that
/// join to the payload's own layout, so the advisory and the install cannot
/// look in two places.
#[test]
fn doctors_releaser_path_is_the_payloads() {
  let machine = Machine::new();
  let skills = machine.payload(&testkit::repo_root());
  assert_eq!(
    canon::releaser_under(&machine.skills()),
    skills.installed_dir(GATE_SKILL).join(GATE_RELEASER)
  );
}
