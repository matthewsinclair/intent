//! ST0083 WP-04, AC-04.1 to AC-04.4: `doctor` reports a contract Intent
//! generated in both layouts (counted), and, shown and not counted, a held
//! `.claude/settings.json` whose hook names a path that does not exist, an
//! empty or placeholder author, and a v2 hook script nothing runs.
//!
//! **EACH ARM READS THE VERDICT BESIDE THE LINE.** Three of the four classes
//! must leave the counted half of the verdict and the exit alone, and the
//! fourth must move them, so every arm compares against the same tree before
//! the one change it makes.

use std::path::Path;

fn run(args: &[&str], cwd: &Path, home: &Path) -> (String, String, i32) {
  let out = crate::common::intent()
    .args(args)
    .current_dir(cwd)
    .env("HOME", home)
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run the v3 binary");
  (
    String::from_utf8_lossy(&out.stdout).into_owned(),
    String::from_utf8_lossy(&out.stderr).into_owned(),
    out.status.code().unwrap_or(-1),
  )
}

/// A project fresh from `init`, its author set to `author`, with its own HOME.
struct Project {
  _tmp: tempfile::TempDir,
  root: std::path::PathBuf,
  home: std::path::PathBuf,
}

fn project(author: &str) -> Project {
  let tmp = tempfile::tempdir().expect("tempdir");
  let root = tmp.path().join("book");
  let home = tmp.path().join("home");
  std::fs::create_dir_all(&root).expect("mkdir");
  std::fs::create_dir_all(&home).expect("mkdir");
  let (out, err, code) = run(&["init", "Book"], &root, &home);
  assert_eq!(code, 0, "init failed: {out}{err}");
  set_author(&root, author);
  Project {
    _tmp: tmp,
    root,
    home,
  }
}

fn set_author(root: &Path, author: &str) {
  let path = root.join("intent/.config/config.json");
  let mut config: serde_json::Value =
    serde_json::from_str(&std::fs::read_to_string(&path).expect("read config"))
      .expect("parse config");
  config["author"] = serde_json::Value::String(author.to_string());
  std::fs::write(
    &path,
    serde_json::to_string_pretty(&config).expect("render"),
  )
  .expect("write config");
}

fn doctor(p: &Project) -> (String, i32) {
  let (out, err, code) = run(&["doctor"], &p.root, &p.home);
  (format!("{out}{err}"), code)
}

/// The counted half of the verdict line, up to its first ` -- `.
fn counted(report: &str) -> Option<String> {
  report
    .lines()
    .find(|line| line.starts_with("doctor:"))
    .and_then(|line| line.split(" -- ").next())
    .map(str::to_string)
}

fn write(root: &Path, rel: &str, body: &str) {
  let path = root.join(rel);
  std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
  std::fs::write(path, body).expect("write");
}

#[test]
fn a_generated_contract_in_both_layouts_is_counted_and_names_both_paths() {
  let p = project("Jo Writer");
  let (before, before_code) = doctor(&p);
  assert!(!before.contains("contracts-doubled"), "{before}");
  let agents = std::fs::read_to_string(p.root.join("AGENTS.md")).expect("AGENTS.md");
  write(&p.root, ".claude/AGENTS.md", &agents);
  let (after, code) = doctor(&p);
  assert!(
    after.contains("contracts-doubled")
      && after.contains(".claude/AGENTS.md")
      && after.contains("AGENTS.md is at AGENTS.md")
      && after.contains("rm .claude/AGENTS.md"),
    "a doubled generated contract was not named with both paths and its removal:\n{after}"
  );
  assert_ne!(
    counted(&after),
    counted(&before),
    "it was not counted:\n{after}"
  );
  assert_ne!(code, before_code, "the exit did not move:\n{after}");
}

#[test]
fn a_hand_written_dotclaude_claude_md_under_root_is_not_counted() {
  let p = project("Jo Writer");
  write(&p.root, ".claude/CLAUDE.md", "# our own notes for Claude\n");
  let (out, _) = doctor(&p);
  assert!(
    !out.contains("contracts-doubled"),
    "a hand-written copy was counted:\n{out}"
  );
}

#[test]
fn a_held_settings_file_naming_a_dead_path_is_shown_and_not_counted() {
  let p = project("Jo Writer");
  let live = p.root.join("live.sh");
  write(&p.root, "live.sh", "#!/bin/sh\n");
  let held = |command: &str| {
    serde_json::json!({ "hooks": { "SessionStart": [{ "hooks": [{ "type": "command", "command": command }] }] } })
      .to_string()
  };
  write(
    &p.root,
    ".claude/settings.json",
    &held(&format!("bash {}", live.display())),
  );
  let (before, before_code) = doctor(&p);
  assert!(
    !before.contains("settings-hook-missing"),
    "a live path was reported:\n{before}"
  );

  write(
    &p.root,
    ".claude/settings.json",
    &held("bash /no/such/machine/session-context.sh"),
  );
  let (after, code) = doctor(&p);
  assert!(
    after.contains("settings-hook-missing")
      && after.contains("/no/such/machine/session-context.sh"),
    "a held settings file with a dead hook path was not reported:\n{after}"
  );
  assert_eq!(
    (counted(&after), code),
    (counted(&before), before_code),
    "{after}"
  );

  write(
    &p.root,
    ".claude/settings.json",
    &held("intent claude hook session-context /no/such/path"),
  );
  let (owned, _) = doctor(&p);
  assert!(
    !owned.contains("settings-hook-missing"),
    "a file canon owns was reported:\n{owned}"
  );
}

#[test]
fn an_empty_or_placeholder_author_is_shown_naming_the_field_and_not_counted() {
  let p = project("Jo Writer");
  let (before, before_code) = doctor(&p);
  assert!(
    !before.contains("author-placeholder"),
    "a real author was reported:\n{before}"
  );
  for author in ["", "Author Unknown"] {
    set_author(&p.root, author);
    let (after, code) = doctor(&p);
    assert!(
      after.contains("author-placeholder")
        && after.contains("`author` in intent/.config/config.json"),
      "author {author:?} was not reported naming the field:\n{after}"
    );
    assert_eq!(
      (counted(&after), code),
      (counted(&before), before_code),
      "{after}"
    );
  }
}

#[test]
fn an_intent_hook_script_nothing_runs_is_shown_with_its_removal_and_not_counted() {
  let p = project("Jo Writer");
  let header = "#!/bin/bash\n#\n# session-context.sh -- Intent SessionStart hook\n";
  write(
    &p.root,
    ".claude/scripts/ours.sh",
    "#!/bin/bash\n# a hook of our own\n",
  );
  write(&p.root, ".claude/scripts/wired.sh", header);
  write(
    &p.root,
    ".claude/settings.local.json",
    &serde_json::json!({ "hooks": { "Stop": [{ "hooks": [{ "type": "command", "command": "bash .claude/scripts/wired.sh" }] }] } })
      .to_string(),
  );
  let (before, before_code) = doctor(&p);
  assert!(
    !before.contains("hook-script-orphaned"),
    "a run or foreign script was reported:\n{before}"
  );

  write(&p.root, ".claude/scripts/session-context.sh", header);
  let (after, code) = doctor(&p);
  assert!(
    after.contains("hook-script-orphaned")
      && after.contains("rm .claude/scripts/session-context.sh"),
    "an orphaned Intent hook script was not named with its removal:\n{after}"
  );
  assert!(
    !after.contains("scripts/wired.sh"),
    "a referenced script was reported:\n{after}"
  );
  assert!(
    !after.contains("scripts/ours.sh"),
    "a script not naming Intent was reported:\n{after}"
  );
  assert_eq!(
    (counted(&after), code),
    (counted(&before), before_code),
    "{after}"
  );
}
