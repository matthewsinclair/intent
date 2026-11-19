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
//! `doctor` helper pins that before every call, so a target moved out of the
//! tree fails here by name rather than comparing against templates nobody
//! chose, and a quiet answer always means the comparison ran.
//!
//! The fixture is not a git repository, so neither side sees a hook: the arm
//! compares the root files and the settings, and the hooks' half is
//! `canon::apply`'s own, already held by its tests.
//!
//! One fixture, three states: quiet after `--apply`; reported once a file
//! moves off its template, with the verdict and the exit code unchanged; quiet
//! again after `--apply`.
//!
//! Issue `0706`: each file's line says what the upgrade replaces in it and
//! what it keeps, and a `.claude/settings.json` whose only difference is the
//! project's own entries is not behind, because the merge keeps them.

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
    None,
  )
  .expect("canon apply");
}

/// doctor on `fx`, after proving it compares against this repository's
/// templates, so a quiet answer is "nothing behind" and never "never looked".
fn doctor(fx: &Fixture) -> Report {
  let canonical = |p: std::path::PathBuf| std::fs::canonicalize(&p).unwrap_or(p);
  assert_eq!(
    canonical(intentsvcs::install::home().expect("an install root above the test binary")),
    canonical(testkit::repo_root()),
    "doctor's install root under cargo test is not the repository the fixture is applied from -- is CARGO_TARGET_DIR outside the tree?"
  );
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

const SETTINGS: &str = ".claude/settings.json";

/// Rewrite the fixture's settings file through `edit`, as compact JSON on one
/// line: a layout of its own, which is not what the template or the merge
/// writes. Returns the bytes written.
fn edit_settings(fx: &Fixture, edit: impl FnOnce(&mut serde_json::Value)) -> String {
  let path = fx.root().join(SETTINGS);
  let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {SETTINGS}: {e}"));
  let mut doc: serde_json::Value = serde_json::from_str(&text).expect("the settings parse");
  edit(&mut doc);
  let body = format!("{doc}\n");
  std::fs::write(&path, &body).unwrap_or_else(|e| panic!("write {SETTINGS}: {e}"));
  body
}

#[test]
fn a_root_file_off_its_template_is_reported_uncounted_until_an_apply_lands_it() {
  let fx = Fixture::new();
  apply(&fx);
  let quiet = doctor(&fx);
  assert!(
    behind(&quiet).is_empty(),
    "negative control: every root file was just written from its template: {:?}",
    behind(&quiet)
  );

  append(&fx, "CLAUDE.md", "\na line the template does not carry\n");
  // One of Intent's own hook entries as an older template wrote it.
  edit_settings(&fx, |doc| {
    doc["hooks"]["SessionStart"][0]["hooks"][0]["timeout"] = serde_json::Value::from(5_u64);
  });
  let stale = doctor(&fx);
  let found = behind(&stale);
  let mut files: Vec<&str> = found.iter().map(|f| f.file.as_str()).collect();
  files.sort_unstable();
  assert_eq!(
    files,
    [SETTINGS, "CLAUDE.md"],
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
  let settings = detail(SETTINGS);
  assert!(
    settings.contains("`intent claude upgrade --apply`") && !settings.contains("--skip-settings"),
    "settings.json names the door that rewrites it, which --skip-settings is not: {settings}"
  );
  assert!(
    settings.ends_with(canon::Keeps::OwnEntries.clause()) && !settings.contains("overwritten"),
    "settings.json's line says the upgrade keeps the project's own entries, never that a hand edit is lost (issue 0706): {settings}"
  );
  assert!(
    detail("CLAUDE.md").ends_with(canon::Keeps::ProjectBlock.clause()),
    "CLAUDE.md's line says its project block is carried across: {}",
    detail("CLAUDE.md")
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

/// Issue `0706`, as measured on brew 3.4.0: three deny rules of the project's
/// own, after Intent's two and in a layout of the project's, put the advisory
/// on every run, though the upgrade keeps them. The merge would only reorder
/// the file, so doctor is quiet and `--apply` leaves the file byte for byte.
#[test]
fn a_settings_file_differing_only_by_the_projects_own_rules_is_not_behind() {
  let fx = Fixture::new();
  apply(&fx);
  let own = ["Bash(rm -rf:*)", "Read(./.env)", "WebFetch"];
  let ours = edit_settings(&fx, |doc| {
    let deny = doc["permissions"]["deny"]
      .as_array_mut()
      .expect("the template carries deny rules");
    deny.extend(own.map(serde_json::Value::from));
  });

  let report = doctor(&fx);
  assert!(
    behind(&report).is_empty(),
    "the project's own rules are not Intent's to report: {:?}",
    behind(&report)
  );
  apply(&fx);
  assert_eq!(
    std::fs::read_to_string(fx.root().join(SETTINGS)).expect("read the settings"),
    ours,
    "`--apply` rewrote a file whose entries it had nothing to change in"
  );
}
