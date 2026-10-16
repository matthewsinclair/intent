//! ST0083 WP-06, AC-06.1: `deinit`'s plan keeps what a person wrote and acts on
//! what canon can prove is Intent's, and its applier acts on exactly the plan.
//!
//! **THE INSTALL IS THIS REPOSITORY**, whose `lib/templates/` is what canon
//! writes from, so a seed planted from [`canon::canon_bytes`] is byte for byte
//! what a real project carries.

use intentsvcs::canon::{self, Artefact};
use intentsvcs::deinit::{self, Act, DeinitError, Plan, Scope};
use intentsvcs::project::Project;
use intentsvcs::views::RenderContext;
use std::path::{Path, PathBuf};

const CTX: RenderContext<'static> = RenderContext {
  version: env!("CARGO_PKG_VERSION"),
  todo_watermark: None,
};

struct Fixture {
  _tmp: tempfile::TempDir,
  root: PathBuf,
  hooks: PathBuf,
}

fn fixture() -> Fixture {
  let tmp = tempfile::tempdir().expect("tempdir");
  let root = tmp.path().join("book");
  std::fs::create_dir_all(&root).expect("mkdir");
  intentsvcs::init::init(&root, "Book", "Jo Writer", env!("CARGO_PKG_VERSION")).expect("init");
  let hooks = tmp.path().join("hooks");
  std::fs::create_dir_all(&hooks).expect("hooks dir");
  Fixture {
    _tmp: tmp,
    root,
    hooks,
  }
}

fn plan_of(fx: &Fixture) -> Plan {
  let project = Project::open(&fx.root).expect("open");
  let install = testkit::repo_root();
  deinit::plan(&Scope {
    project: &project,
    install: &install,
    hooks: Some(&fx.hooks),
    ctx: &CTX,
    all: None,
  })
  .expect("plan")
}

fn seed(fx: &Fixture, artefact: Artefact) -> String {
  let project = Project::open(&fx.root).expect("open");
  canon::canon_bytes(artefact, &testkit::repo_root(), project.config(), &CTX).expect("render")
}

fn removes(plan: &Plan, path: &Path) -> bool {
  plan
    .acts
    .iter()
    .any(|act| matches!(act, Act::Remove { path: p, .. } if p == path))
}

fn rewrites(plan: &Plan, path: &Path) -> Option<String> {
  plan.acts.iter().find_map(|act| match act {
    Act::Rewrite { path: p, after, .. } if p == path => Some(after.clone()),
    _ => None,
  })
}

fn keeps(plan: &Plan, path: &Path) -> bool {
  plan.kept.iter().any(|kept| kept.path == path)
}

/// Every file under `dir` with its bytes, for a before-and-after comparison.
fn tree(dir: &Path) -> Vec<(PathBuf, Vec<u8>)> {
  let mut out = Vec::new();
  let mut stack = vec![dir.to_path_buf()];
  while let Some(d) = stack.pop() {
    for entry in std::fs::read_dir(&d).expect("read_dir") {
      let path = entry.expect("entry").path();
      if path.is_dir() {
        stack.push(path);
      } else {
        out.push((path.clone(), std::fs::read(&path).expect("read")));
      }
    }
  }
  out.sort();
  out
}

#[test]
fn a_fresh_project_plans_its_contracts_and_planning_writes_nothing() {
  let fx = fixture();
  let before = tree(fx._tmp.path());
  let plan = plan_of(&fx);
  assert_eq!(tree(fx._tmp.path()), before, "planning wrote to the tree");
  assert!(removes(&plan, &fx.root.join("AGENTS.md")), "{plan:?}");
  assert!(removes(&plan, &fx.root.join("CLAUDE.md")), "{plan:?}");
  assert!(
    plan.kept.is_empty(),
    "a fresh project kept something: {:?}",
    plan.kept
  );
}

#[test]
fn a_contract_a_person_wrote_or_wrote_into_is_kept() {
  let fx = fixture();
  std::fs::create_dir_all(fx.root.join(".claude")).expect("mkdir");
  std::fs::write(fx.root.join(".claude/CLAUDE.md"), "# our own notes\n").expect("write");
  let claude = std::fs::read_to_string(fx.root.join("CLAUDE.md")).expect("read");
  let filled = claude.replacen(
    "<!-- user:end -->",
    "Always answer in French.\n<!-- user:end -->",
    1,
  );
  assert_ne!(filled, claude, "the template has no user block to fill");
  std::fs::write(fx.root.join("CLAUDE.md"), filled).expect("write");
  let plan = plan_of(&fx);
  assert!(keeps(&plan, &fx.root.join(".claude/CLAUDE.md")), "{plan:?}");
  assert!(keeps(&plan, &fx.root.join("CLAUDE.md")), "{plan:?}");
  assert!(removes(&plan, &fx.root.join("AGENTS.md")), "{plan:?}");
}

#[test]
fn a_seed_goes_only_while_it_is_byte_identical() {
  let fx = fixture();
  std::fs::write(
    fx.root.join("usage-rules.md"),
    seed(&fx, Artefact::UsageRules),
  )
  .expect("write");
  std::fs::write(
    fx.root.join(".intent_critic.yml"),
    seed(&fx, Artefact::CriticConfig) + "# ours\n",
  )
  .expect("write");
  let plan = plan_of(&fx);
  assert!(removes(&plan, &fx.root.join("usage-rules.md")), "{plan:?}");
  assert!(
    keeps(&plan, &fx.root.join(".intent_critic.yml")),
    "{plan:?}"
  );
}

#[test]
fn a_hook_loses_its_chain_block_and_keeps_every_foreign_line() {
  let fx = fixture();
  let foreign = "#!/bin/sh\nset -e\n./our-guard.sh\n";
  let wired = canon::insert_chain_block("pre-commit", foreign).expect("wire");
  std::fs::write(fx.hooks.join("pre-commit"), &wired).expect("write");
  let only = canon::insert_chain_block("post-merge", "").expect("wire");
  std::fs::write(fx.hooks.join("post-merge"), &only).expect("write");
  std::fs::write(
    fx.hooks.join("post-checkout"),
    format!(
      "{only}{}",
      canon::insert_chain_block("post-checkout", "").expect("wire")
    ),
  )
  .expect("write a doubled hook");
  std::fs::write(
    fx.hooks.join("pre-commit.intent"),
    seed(&fx, Artefact::PreCommitCarrier),
  )
  .expect("write");
  std::fs::write(fx.hooks.join("post-merge.intent"), "#!/bin/sh\n# edited\n").expect("write");
  let plan = plan_of(&fx);
  assert_eq!(
    rewrites(&plan, &fx.hooks.join("pre-commit")).as_deref(),
    Some(foreign)
  );
  assert!(removes(&plan, &fx.hooks.join("post-merge")), "{plan:?}");
  assert!(
    keeps(&plan, &fx.hooks.join("post-checkout")),
    "a doubled block was not kept: {plan:?}"
  );
  assert!(
    removes(&plan, &fx.hooks.join("pre-commit.intent")),
    "{plan:?}"
  );
  assert!(
    keeps(&plan, &fx.hooks.join("post-merge.intent")),
    "{plan:?}"
  );
}

#[test]
fn settings_and_mcp_lose_only_intents_entries_and_an_unparseable_one_is_kept() {
  let fx = fixture();
  std::fs::create_dir_all(fx.root.join(".claude")).expect("mkdir");
  let mut settings: serde_json::Value =
    serde_json::from_str(&seed(&fx, Artefact::Settings)).expect("the template parses");
  settings["model"] = serde_json::json!("opus");
  std::fs::write(fx.root.join(".claude/settings.json"), settings.to_string()).expect("write");
  std::fs::write(fx.root.join(".mcp.json"), seed(&fx, Artefact::Mcp)).expect("write");
  let plan = plan_of(&fx);
  let kept: serde_json::Value = serde_json::from_str(
    &rewrites(&plan, &fx.root.join(".claude/settings.json")).expect("settings are edited"),
  )
  .expect("the edit is JSON");
  assert_eq!(kept, serde_json::json!({ "model": "opus" }));
  assert!(removes(&plan, &fx.root.join(".mcp.json")), "{plan:?}");

  std::fs::write(fx.root.join(".claude/settings.json"), "{ not json").expect("write");
  assert!(keeps(&plan_of(&fx), &fx.root.join(".claude/settings.json")));
}

#[test]
fn apply_acts_on_the_plan_and_refuses_a_file_that_moved_since() {
  let fx = fixture();
  let plan = plan_of(&fx);
  std::fs::write(fx.root.join("AGENTS.md"), "moved\n").expect("write");
  let refused = deinit::apply(&plan);
  assert!(
    matches!(refused, Err(DeinitError::Changed { ref path }) if path.ends_with("AGENTS.md")),
    "a file that moved after the plan was not refused: {refused:?}"
  );
  assert!(
    fx.root.join("AGENTS.md").exists(),
    "the moved file was removed"
  );

  let plan = plan_of(&fx);
  deinit::apply(&plan).expect("apply");
  assert!(!fx.root.join("CLAUDE.md").exists());
  assert!(
    fx.root.join("AGENTS.md").exists(),
    "a kept file was removed"
  );
}
