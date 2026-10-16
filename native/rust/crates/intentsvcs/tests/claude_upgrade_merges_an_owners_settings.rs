//! Issue 0606: `claude upgrade --apply` merges Intent's entries into a
//! `.claude/settings.json` that carries its hook door, keeping every key the
//! owner added, rather than writing the template over the file.

use std::path::Path;

use intentsvcs::canon;

fn template_text() -> String {
  std::fs::read_to_string(testkit::repo_root().join("lib/templates/.claude/settings.json"))
    .expect("the settings template")
}

fn apply(fx: &crate::common::Fixture) -> canon::Applied {
  let hooks = fx.root().join(".git/hooks");
  std::fs::create_dir_all(&hooks).expect("mkdir hooks");
  let project = fx.project();
  canon::apply(
    fx.root(),
    &testkit::repo_root(),
    project.config(),
    &crate::common::ctx(),
    Some(&hooks),
    canon::Options::default(),
    None,
  )
  .expect("canon apply")
}

fn seed(fx: &crate::common::Fixture, body: &str) -> std::path::PathBuf {
  let path = fx.root().join(".claude/settings.json");
  std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir .claude");
  std::fs::write(&path, body).expect("seed the settings");
  path
}

fn json(path: &Path) -> serde_json::Value {
  serde_json::from_str(&std::fs::read_to_string(path).expect("read")).expect("JSON")
}

/// An older Intent's entries beside the owner's own: a model pin, a
/// permission, a deny rule and a hook of theirs, and a stale Intent hook.
fn owners_and_an_old_intents() -> String {
  serde_json::json!({
    "model": "opus",
    "permissions": { "allow": ["Bash(ls:*)"], "deny": ["Read(./secrets)"] },
    "hooks": {
      "SessionStart": [{ "matcher": "startup", "hooks": [{ "type": "command", "command": "intent claude hook session-context" }] }],
      "PreToolUse": [{ "matcher": "Bash", "hooks": [{ "type": "command", "command": "./our-check.sh" }] }],
      "Notification": [{ "matcher": "", "hooks": [{ "type": "command", "command": "intent claude hook retired-hook" }] }]
    }
  })
  .to_string()
}

#[test]
fn the_owners_keys_and_hook_survive_and_intents_entries_are_the_templates() {
  let fx = crate::common::Fixture::new();
  fx.git_init();
  let path = seed(&fx, &owners_and_an_old_intents());
  apply(&fx);
  let after = json(&path);
  assert_eq!(
    after["model"], "opus",
    "the owner's model pin was lost:\n{after:#}"
  );
  assert_eq!(
    after["permissions"]["allow"],
    serde_json::json!(["Bash(ls:*)"]),
    "{after:#}"
  );
  let deny = after["permissions"]["deny"].as_array().expect("deny");
  assert!(
    deny.contains(&serde_json::json!("Read(./secrets)")),
    "{after:#}"
  );
  assert_eq!(
    after["hooks"]["PreToolUse"][0]["hooks"][0]["command"], "./our-check.sh",
    "the owner's hook was lost:\n{after:#}"
  );
  assert!(
    after["hooks"].get("Notification").is_none(),
    "a hook entry Intent no longer ships survived:\n{after:#}"
  );
  // Intent's entries are now exactly the template's: stripping the owner's
  // keys back out leaves the template.
  let template: serde_json::Value = serde_json::from_str(&template_text()).expect("template");
  for (event, matchers) in template["hooks"].as_object().expect("hooks") {
    for matcher in matchers.as_array().expect("matchers") {
      assert!(
        after["hooks"][event]
          .as_array()
          .expect("event")
          .contains(matcher),
        "the template's {event} entry is missing:\n{after:#}"
      );
    }
  }
  for rule in template["permissions"]["deny"].as_array().expect("deny") {
    assert!(
      deny.contains(rule),
      "the template's deny rule {rule} is missing:\n{after:#}"
    );
  }
}

#[test]
fn a_second_upgrade_changes_nothing() {
  let fx = crate::common::Fixture::new();
  fx.git_init();
  let path = seed(&fx, &owners_and_an_old_intents());
  apply(&fx);
  let once = std::fs::read(&path).expect("read");
  let applied = apply(&fx);
  assert_eq!(
    std::fs::read(&path).expect("read"),
    once,
    "the merge is not idempotent"
  );
  assert!(
    !applied.written.contains(&path),
    "a no-op merge was reported as written"
  );
}

#[test]
fn a_pristine_template_file_stays_byte_identical() {
  let fx = crate::common::Fixture::new();
  fx.git_init();
  let path = seed(&fx, &template_text());
  apply(&fx);
  assert_eq!(
    std::fs::read_to_string(&path).expect("read"),
    template_text()
  );
}

#[test]
fn an_unparseable_file_carrying_the_door_is_held_and_named() {
  let fx = crate::common::Fixture::new();
  fx.git_init();
  let broken = "{ \"hooks\": \"intent claude hook session-context\" ";
  let path = seed(&fx, broken);
  let applied = apply(&fx);
  assert_eq!(
    std::fs::read_to_string(&path).expect("read"),
    broken,
    "an unparseable file was overwritten"
  );
  assert!(
    applied.unparseable.contains(&path),
    "the unparseable file was not named: {applied:?}"
  );
  assert!(!applied.written.contains(&path));
}
