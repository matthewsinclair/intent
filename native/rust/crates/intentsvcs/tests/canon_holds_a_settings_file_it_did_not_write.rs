//! **`claude upgrade --apply` overwrote `.claude/settings.json` with no hold
//! and no check, in the same function that holds `CLAUDE.md`.**
//!
//! The rule for `CLAUDE.md` was already written down and already right: a copy
//! carrying no generated marker was authored by a person, and curing variation
//! by overwriting it destroys what the project decided. `.claude/settings.json`
//! sat just above that rule and took none of it, so a project with its
//! own Claude Code settings -- hooks, permissions, a model pin, anything -- lost
//! the file to a verb it ran to update its documentation. hv's batch-4 class:
//! bytes overwritten without being named first.
//!
//! **WHAT COUNTS AS THE MARKER, AND WHY IT IS NOT BYTE-EQUALITY.** JSON carries
//! no comment, so there is no footer to look for. Holding on any difference
//! from the template instead would be worse than the defect in a quieter way:
//! a project that took Intent's hooks under an OLDER template would be held for
//! ever, so a hook fix would silently never reach it and the report would say
//! `held` while everyone assumed it had landed. The discriminator that answers
//! the real question -- *did this file ever carry Intent's hook door* -- is the
//! door itself, `intent claude hook`. Present means Intent wrote it and an
//! upgrade may rewrite it; absent means it is somebody else's file.
//!
//! **AND THE RESIDUAL IS STATED RATHER THAN HIDDEN**: a project that kept
//! Intent's hooks and tuned a timeout still has its tuning rewritten, exactly as
//! a marker-carrying `CLAUDE.md` is regenerated. That is the same trade in the
//! same function, not a gap this file leaves open by accident.

use std::path::Path;

use intentsvcs::canon;

fn home() -> std::path::PathBuf {
  testkit::repo_root()
}

fn hooks_dir(fx: &crate::common::Fixture) -> std::path::PathBuf {
  let hooks = fx.root().join(".git/hooks");
  std::fs::create_dir_all(&hooks).expect("mkdir hooks");
  hooks
}

fn apply(fx: &crate::common::Fixture, hooks: &Path, opts: canon::Options) -> canon::Applied {
  let project = fx.project();
  canon::apply(
    fx.root(),
    &home(),
    project.config(),
    &crate::common::ctx(),
    Some(hooks),
    opts,
  )
  .expect("canon apply")
}

/// Somebody's own Claude Code settings, with no Intent hook door anywhere in
/// them. Recognisable bytes, so the test can read them back rather than merely
/// assert a disposition.
const THEIRS: &str = r#"{
  "permissions": {
    "allow": ["Bash(ls:*)"]
  },
  "model": "the-one-this-project-chose"
}
"#;

fn seed_theirs(fx: &crate::common::Fixture) -> std::path::PathBuf {
  let path = fx.root().join(".claude/settings.json");
  std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir .claude");
  std::fs::write(&path, THEIRS).expect("seed their settings");
  path
}

/// **THE DEFECT.** Their file survives, and the run says it held it.
#[test]
fn a_settings_file_with_no_intent_hook_door_is_held_not_overwritten() {
  let fx = crate::common::Fixture::new();
  fx.git_init();
  let hooks = hooks_dir(&fx);
  let path = seed_theirs(&fx);

  let applied = apply(&fx, &hooks, canon::Options::default());

  let after = std::fs::read_to_string(&path).expect("their settings are gone");
  assert_eq!(
    after, THEIRS,
    "canon overwrote a settings.json it did not write"
  );
  assert!(
    applied.held.contains(&path),
    "the file was left alone and NOT reported as held, which is a silent skip: \
     held={:?} written={:?}",
    applied.held,
    applied.written
  );
  assert!(
    !applied.written.contains(&path),
    "a held file must not also be reported as written"
  );
}

/// **`--force` IS THE WAY THROUGH, AND IT MUST ACTUALLY GO THROUGH.** A hold
/// with no escape is a different defect from the one being fixed.
#[test]
fn force_overwrites_the_settings_file_it_would_otherwise_hold() {
  let fx = crate::common::Fixture::new();
  fx.git_init();
  let hooks = hooks_dir(&fx);
  let path = seed_theirs(&fx);

  let applied = apply(
    &fx,
    &hooks,
    canon::Options {
      force: true,
      ..canon::Options::default()
    },
  );

  let after = std::fs::read_to_string(&path).expect("read it back");
  assert!(
    after.contains("intent claude hook"),
    "--force did not install the hook door: {after}"
  );
  assert!(
    !applied.held.contains(&path),
    "--force still reported the file as held"
  );
}

/// **THE CONTROL THAT KEEPS THE UPGRADE PATH ALIVE.** A file that already
/// carries the hook door is Intent's own, and an upgrade must still be able to
/// rewrite it -- otherwise a hook fix reaches nobody and the report says `held`
/// while everyone believes it landed.
#[test]
fn a_settings_file_carrying_the_hook_door_is_still_rewritten() {
  let fx = crate::common::Fixture::new();
  fx.git_init();
  let hooks = hooks_dir(&fx);

  let path = fx.root().join(".claude/settings.json");
  std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir .claude");
  // An OLDER Intent-written file: the door is there, the rest is not current.
  std::fs::write(
    &path,
    "{\n  \"hooks\": {\n    \"Stop\": [\n      {\n        \"matcher\": \"\",\n        \"hooks\": [\n          {\n            \"type\": \"command\",\n            \"command\": \"intent claude hook session-finish\"\n          }\n        ]\n      }\n    ]\n  }\n}\n",
  )
  .expect("seed an older intent-written settings file");

  let applied = apply(&fx, &hooks, canon::Options::default());

  assert!(
    !applied.held.contains(&path),
    "an Intent-written settings file was held, so a hook fix would never reach this project"
  );
  let after = std::fs::read_to_string(&path).expect("read it back");
  assert!(
    after.contains("require-in-session"),
    "the current template did not reach an Intent-written file: {after}"
  );
}

/// **`--skip-settings` STILL DECLINES IT, and skipped must not become held.**
/// Two different answers to two different questions, and collapsing them would
/// make issue 0143's escape hatch read as a refusal.
#[test]
fn skip_settings_still_skips_and_does_not_read_as_a_hold() {
  let fx = crate::common::Fixture::new();
  fx.git_init();
  let hooks = hooks_dir(&fx);
  let path = seed_theirs(&fx);

  let applied = apply(
    &fx,
    &hooks,
    canon::Options {
      skip_settings: true,
      ..canon::Options::default()
    },
  );

  assert!(
    applied.skipped.contains(&path),
    "--skip-settings did not report the file as skipped"
  );
  assert!(
    !applied.held.contains(&path),
    "a skipped file was also reported as held"
  );
  assert_eq!(
    std::fs::read_to_string(&path).expect("read it back"),
    THEIRS,
    "--skip-settings wrote to the file it declined"
  );
}

/// **A PROJECT WITH NO SETTINGS FILE AT ALL STILL GETS ONE.** The hold must
/// not turn into "canon stopped installing the hook door", which would disarm
/// every new project silently.
#[test]
fn an_absent_settings_file_is_still_written() {
  let fx = crate::common::Fixture::new();
  fx.git_init();
  let hooks = hooks_dir(&fx);
  let path = fx.root().join(".claude/settings.json");
  assert!(!path.exists(), "the fixture already has a settings file");

  let applied = apply(&fx, &hooks, canon::Options::default());

  assert!(
    applied.written.contains(&path),
    "canon did not write a settings file into a project that had none"
  );
  assert!(
    std::fs::read_to_string(&path)
      .expect("read it back")
      .contains("intent claude hook"),
    "the file canon wrote carries no hook door"
  );
}
