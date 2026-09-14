//! Issue 0323: **`intent init` keeps the store out of git from the first
//! commit.** A fresh project had no `.gitignore` rule for `intent/.cache/`, so
//! `git add .` staged `intent.db`, which D34 says never enters history. The
//! migration converged the rule and init did not, so a project BORN on v3 was
//! the one shape that never got it.
//!
//! The rules come from the one converger both doors call, so this asserts the
//! same three path rules `migrate_v2_project.rs` asserts after a migration --
//! and then asks GIT, because the property is that git ignores the store, and a
//! line in a file is only evidence of that until something asks.

use std::process::Command;

#[test]
fn a_fresh_init_ignores_the_store_by_path_as_a_migration_does() {
  let fresh = tempfile::tempdir().expect("tempdir");
  let root = fresh.path();
  intentsvcs::init::init(root, "Probe", "dc", "3.0.0").expect("init");

  let ignored = std::fs::read_to_string(root.join(".gitignore"))
    .expect("init wrote no .gitignore, so the store is staged by the first `git add .`");
  for rule in ["intent/.cache/", "intent/events.jsonl", "intent/.backup/"] {
    assert!(
      ignored.lines().any(|l| l.trim() == rule),
      "a fresh init does not ignore `{rule}`: {ignored:?}"
    );
  }
  assert!(
    !ignored.contains("*.db"),
    "the rule is a PATH rule, as the migration's is -- `*.db` would swallow a database the \
     operator wants tracked: {ignored:?}"
  );

  // Git's own answer, on the file the defect staged.
  let git = |args: &[&str]| {
    Command::new("git")
      .args(args)
      .current_dir(root)
      .output()
      .expect("run git")
  };
  assert!(git(&["init", "-q"]).status.success(), "git init");
  std::fs::create_dir_all(root.join("intent/.cache")).expect("mkdir");
  std::fs::write(root.join("intent/.cache/intent.db"), b"a store").expect("write");
  assert!(
    git(&["check-ignore", "-q", "intent/.cache/intent.db"])
      .status
      .success(),
    "git does not ignore intent/.cache/intent.db in a freshly initialised project"
  );
}
