//! Issue `0496`: a root file canon writes -- `CLAUDE.md` the worked instance --
//! could be behind the templates the running Intent reads, and `doctor` said
//! nothing. Driven 2026-09-21 on the 46131238e pair: a line appended to
//! `CLAUDE.md` after `claude upgrade --apply` left `doctor` at `0 finding(s)`
//! while the dry `claude upgrade` listed the file as one it would write.
//!
//! **THE INSTALL ROOT, AND WHERE A FIXTURE DIFFERS FROM A PROJECT.** `doctor`
//! resolves the templates from the running binary's own path. Under `cargo
//! test` that binary is this suite, and with the in-tree target the build
//! rules require, the first ancestor carrying `lib/templates` is this
//! repository -- the root [`testkit::repo_root`] hands every canon test. The
//! first assertion pins that, so a target moved out of the tree fails here by
//! name rather than comparing against templates nobody chose.
//!
//! The fixture is not a git repository, so neither side sees a hook: the arm
//! compares the root files and the settings, and the hooks' half is
//! `canon::apply`'s own, already held by its tests.
//!
//! One fixture, three states: quiet after `--apply`; reported once a file
//! moves off its template, with the verdict and the exit code unchanged; quiet
//! again after `--apply`.

use crate::common::{Fixture, ctx};
use intentsvcs::canon;
use intentsvcs::doctor::{Report, Scope, diagnose};
use intentsvcs::finding::{Finding, FindingClass};

fn apply(fx: &Fixture) {
  canon::apply(
    fx.root(),
    &testkit::repo_root(),
    fx.project().config(),
    &ctx(),
    None,
    canon::Options::default(),
  )
  .expect("canon apply");
}

fn doctor(fx: &Fixture) -> Report {
  diagnose(&fx.project(), &ctx(), None, Scope::All)
}

fn behind(report: &Report) -> Vec<&Finding> {
  report
    .findings
    .iter()
    .filter(|f| f.class == FindingClass::RootFileBehind)
    .collect()
}

fn append(fx: &Fixture, rel: &str, text: &str) {
  let path = fx.root().join(rel);
  let mut body = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {rel}: {e}"));
  body.push_str(text);
  std::fs::write(&path, body).unwrap_or_else(|e| panic!("write {rel}: {e}"));
}

#[test]
fn a_root_file_off_its_template_is_reported_uncounted_until_an_apply_lands_it() {
  let canonical = |p: std::path::PathBuf| std::fs::canonicalize(&p).unwrap_or(p);
  assert_eq!(
    canonical(intentsvcs::install::home().expect("an install root above the test binary")),
    canonical(testkit::repo_root()),
    "doctor's install root under cargo test is not the repository the fixture is applied from -- is CARGO_TARGET_DIR outside the tree?"
  );

  let fx = Fixture::new();
  apply(&fx);
  let quiet = doctor(&fx);
  assert!(
    behind(&quiet).is_empty(),
    "negative control: every root file was just written from its template: {:?}",
    behind(&quiet)
  );

  append(&fx, "CLAUDE.md", "\na line the template does not carry\n");
  append(&fx, ".claude/settings.json", "\n");
  let stale = doctor(&fx);
  let found = behind(&stale);
  let mut files: Vec<&str> = found.iter().map(|f| f.file.as_str()).collect();
  files.sort_unstable();
  assert_eq!(
    files,
    [".claude/settings.json", "CLAUDE.md"],
    "each file off its template is named, project-relative, and nothing else: {found:?}"
  );
  let detail = |file: &str| {
    found
      .iter()
      .find(|f| f.file == file)
      .map(|f| f.detail.as_str())
      .unwrap_or_default()
  };
  assert!(
    detail("CLAUDE.md").contains("`intent claude upgrade --apply --skip-settings`"),
    "CLAUDE.md names the door the project's own CLAUDE.md prescribes: {}",
    detail("CLAUDE.md")
  );
  let settings = detail(".claude/settings.json");
  assert!(
    settings.contains("`intent claude upgrade --apply`") && !settings.contains("--skip-settings"),
    "settings.json names the door that rewrites it, which --skip-settings is not: {settings}"
  );
  assert_eq!(
    (stale.actionable(), stale.exit_code()),
    (quiet.actionable(), quiet.exit_code()),
    "the advisory moves neither the verdict nor the exit code"
  );
  assert!(
    FindingClass::RootFileBehind.is_shown_by_default(),
    "shown without --verbose: a doctor reading 0 while CLAUDE.md is behind is the defect"
  );

  apply(&fx);
  let landed = doctor(&fx);
  assert!(
    behind(&landed).is_empty(),
    "negative control: quiet once an apply lands the templates: {:?}",
    behind(&landed)
  );
}
