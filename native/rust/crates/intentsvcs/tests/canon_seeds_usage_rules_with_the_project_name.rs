//! Issue 0336: **a seeded `usage-rules.md` names the project, never
//! `[[PROJECT_NAME]]`.** The template carries the token on its first lines and
//! the seed wrote it raw, so every project that took the file read its own name
//! as a placeholder.

use intentsvcs::canon;

#[test]
fn a_seeded_usage_rules_carries_the_project_name_and_no_token() {
  let fx = crate::common::Fixture::new();
  let hooks = fx.root().join(".git/hooks");
  std::fs::create_dir_all(&hooks).expect("mkdir hooks");
  let path = fx.root().join("usage-rules.md");
  assert!(!path.exists(), "the fixture already had one");

  let project = fx.project();
  canon::apply(
    fx.root(),
    &testkit::repo_root(),
    project.config(),
    &crate::common::ctx(),
    Some(&hooks),
    canon::Options::default(),
  )
  .expect("canon apply");

  let body = std::fs::read_to_string(&path).expect("read the seeded file");
  let name = &project.config().project_name;
  assert!(
    !name.is_empty(),
    "the fixture's project has no name, so the arm would prove nothing"
  );
  assert!(
    body.contains(name.as_str()),
    "the seeded file never names `{name}`:\n{body}"
  );
  assert!(
    !body.contains("[["),
    "the seeded file still carries a token:\n{body}"
  );
}
