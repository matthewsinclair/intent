//! Issue 0564: **a seeded `.intent_critic.yml` is the template's own bytes.**
//! The seed went through the Markdown expander, whose empty-section filler read
//! every YAML comment line as a bare heading and wrote
//! `_Not configured for this project._` after it, so every project that took the
//! seed held a file a strict YAML reader refuses.

use intentsvcs::canon;

#[test]
fn a_seeded_critic_config_equals_its_template_byte_for_byte() {
  let fx = crate::common::Fixture::new();
  let hooks = fx.root().join(".git/hooks");
  std::fs::create_dir_all(&hooks).expect("mkdir hooks");
  let path = fx.root().join(".intent_critic.yml");
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

  let template =
    std::fs::read_to_string(testkit::repo_root().join("lib/templates/_intent_critic.yml"))
      .expect("read the template");
  let lines: Vec<&str> = template.lines().collect();
  assert!(
    lines
      .windows(2)
      .any(|w| w[0].starts_with('#') && w[1].starts_with('#')),
    "no comment line in the template is followed by another, so the arm cannot see the filler"
  );
  let seeded = std::fs::read_to_string(&path).expect("read the seeded file");
  assert_eq!(seeded, template, "the seed is not the template's bytes");
}
