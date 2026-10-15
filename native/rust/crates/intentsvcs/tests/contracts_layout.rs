//! ST0083 WP-02, AT-02.1: the `contracts` key places the two agent contracts, and every
//! writer, reader and the sync scope derive the place from it (AC-02.1 to
//! AC-02.3). Each `dotclaude` arm has a `root` control beside it, so a writer
//! that ignored the key would fail one side or the other rather than pass both.

use std::path::{Path, PathBuf};

use crate::common::{Fixture, facade_ctx};
use intentsvcs::canon::Options;
use intentsvcs::facade::Facade;
use intentsvcs::project::{Config, Contracts, Project};
use intentsvcs::rootfiles::AgentsFileState;
use intentsvcs::sync::{Scanned, SyncError};

/// A config carrying `contracts` and nothing else optional.
fn with_contracts(contracts: serde_json::Value) -> Result<Config, serde_json::Error> {
  serde_json::from_value(serde_json::json!({
    "intent_version": "3.2.3",
    "contracts": contracts,
  }))
}

/// A fixture project declaring `layout` through the one config writer, and a
/// facade opened after the declaration, so it reads it.
fn declared(layout: Contracts) -> (Fixture, Facade) {
  let fx = Fixture::new();
  let mut config = fx.project().config().clone();
  config.contracts = layout;
  intentsvcs::project::write_config(fx.root(), &config).expect("the config writes");
  let facade = Facade::open_in_memory(fx.project(), facade_ctx()).expect("the facade opens");
  (fx, facade)
}

fn options() -> Options {
  Options {
    force: false,
    skip_settings: true,
    report: false,
  }
}

fn scanned(root: &Path) -> Vec<String> {
  intentsvcs::sync::scan(root, &[])
    .expect("the scan reads")
    .into_iter()
    .map(|entry| entry.path)
    .collect()
}

#[test]
fn the_key_reads_root_and_dotclaude_and_an_absent_key_reads_as_root() {
  assert_eq!(
    with_contracts(serde_json::json!("root"))
      .expect("root reads")
      .contracts,
    Contracts::Root
  );
  assert_eq!(
    with_contracts(serde_json::json!("dotclaude"))
      .expect("dotclaude reads")
      .contracts,
    Contracts::Dotclaude
  );
  let absent: Config = serde_json::from_value(serde_json::json!({ "intent_version": "3.2.3" }))
    .expect("a config without the key reads");
  assert_eq!(absent.contracts, Contracts::Root);
}

#[test]
fn a_third_value_is_refused_by_name_with_the_two_it_accepts() {
  for (value, shown) in [
    (serde_json::json!("Dotclaude"), "\"Dotclaude\""),
    (serde_json::json!(".claude"), "\".claude\""),
    (serde_json::json!(3), "3"),
  ] {
    let message = with_contracts(value)
      .expect_err("a third value is refused")
      .to_string();
    assert!(
      message.contains(&format!(
        "`contracts` is {shown}, and it accepts \"root\" or \"dotclaude\""
      )),
      "the refusal must name the key, the value and the two it accepts: {message}"
    );
  }
}

#[test]
fn the_root_default_is_never_written_back_and_dotclaude_is() {
  let root = with_contracts(serde_json::json!("root")).expect("root reads");
  assert_eq!(
    serde_json::to_value(&root)
      .expect("serialises")
      .get("contracts"),
    None,
    "root is the default, and a rewrite must leave it out of the file"
  );
  let dotclaude = with_contracts(serde_json::json!("dotclaude")).expect("dotclaude reads");
  assert_eq!(
    serde_json::to_value(&dotclaude).expect("serialises")["contracts"],
    serde_json::json!("dotclaude")
  );
}

#[test]
fn under_dotclaude_agents_sync_and_validate_use_dotclaude_and_never_the_root() {
  let (fx, mut facade) = declared(Contracts::Dotclaude);

  let written = facade.agents_sync().expect("agents sync writes");

  assert_eq!(written, fx.root().join(".claude/AGENTS.md"));
  assert!(
    written.is_file(),
    "the contract is on disk where the key puts it"
  );
  assert!(
    !fx.root().join("AGENTS.md").exists(),
    "no root copy under dotclaude"
  );
  let report = facade.agents_validate();
  assert_eq!(report.path, PathBuf::from(".claude/AGENTS.md"));
  assert_eq!(report.state, AgentsFileState::Regular);
}

#[test]
fn under_dotclaude_claude_upgrade_writes_both_contracts_there_and_seeds_no_usage_rules() {
  let (fx, mut facade) = declared(Contracts::Dotclaude);

  let upgraded = facade
    .claude_upgrade(None, options())
    .expect("canon applies");

  for name in ["AGENTS.md", "CLAUDE.md"] {
    assert!(
      fx.root().join(".claude").join(name).is_file(),
      "{name} is written under .claude/"
    );
    assert!(!fx.root().join(name).exists(), "{name} has no root copy");
  }
  let usage_rules = fx.root().join("usage-rules.md");
  assert!(
    !usage_rules.exists(),
    "dotclaude seeds no usage-rules.md (hv's ruling 9 on ST0083)"
  );
  assert!(!upgraded.applied.written.contains(&usage_rules));
}

/// vc's rider (i) on ST0083 WP-02: ruling 9 says not SEEDED, never removed.
#[test]
fn under_dotclaude_an_authored_usage_rules_at_the_root_is_preserved_and_reported() {
  let (fx, mut facade) = declared(Contracts::Dotclaude);
  let usage_rules = fx.root().join("usage-rules.md");
  std::fs::write(&usage_rules, "# Ours\n").expect("an authored usage-rules.md");

  let upgraded = facade
    .claude_upgrade(None, options())
    .expect("canon applies");

  assert!(
    upgraded.applied.preserved.contains(&usage_rules),
    "an authored usage-rules.md is reported as preserved: {:?}",
    upgraded.applied.preserved
  );
  assert_eq!(
    std::fs::read_to_string(&usage_rules).expect("it reads"),
    "# Ours\n"
  );
}

/// The control for the three arms above: the same calls under `root` write at
/// the root and seed `usage-rules.md`, as 3.2.3 did.
#[test]
fn under_root_claude_upgrade_writes_the_contracts_at_the_root_and_seeds_usage_rules() {
  let (fx, mut facade) = declared(Contracts::Root);

  let upgraded = facade
    .claude_upgrade(None, options())
    .expect("canon applies");

  for name in ["AGENTS.md", "CLAUDE.md"] {
    assert!(fx.root().join(name).is_file(), "{name} is at the root");
    assert!(
      !fx.root().join(".claude").join(name).exists(),
      "{name} has no .claude copy under root"
    );
  }
  assert!(
    upgraded
      .applied
      .written
      .contains(&fx.root().join("usage-rules.md")),
    "root seeds usage-rules.md as it always has"
  );
}

/// AC-02.3: the scope `sync --to-store` and the daemon both ask takes the
/// contracts from where the key puts them, and a stray copy elsewhere is out.
#[test]
fn the_scope_takes_the_contracts_where_the_key_puts_them_and_a_copy_elsewhere_is_out() {
  for (layout, inside, outside) in [
    (Contracts::Dotclaude, ".claude/", ""),
    (Contracts::Root, "", ".claude/"),
  ] {
    let (fx, _facade) = declared(layout);
    std::fs::create_dir_all(fx.root().join(".claude")).expect("mkdir .claude");
    for name in ["AGENTS.md", "CLAUDE.md"] {
      std::fs::write(fx.root().join(format!("{inside}{name}")), "x\n").expect("the contract");
      std::fs::write(fx.root().join(format!("{outside}{name}")), "x\n").expect("a stray copy");
    }

    let scope = Scanned::for_root(fx.root()).expect("the scope builds");
    let paths = scanned(fx.root());

    for name in ["AGENTS.md", "CLAUDE.md"] {
      let kept = format!("{inside}{name}");
      let left = format!("{outside}{name}");
      assert!(
        scope.includes(&fx.root().join(&kept)),
        "{layout:?}: {kept} is in scope"
      );
      assert!(
        !scope.includes(&fx.root().join(&left)),
        "{layout:?}: {left} is out of scope"
      );
      assert!(
        paths.contains(&kept),
        "{layout:?}: the scan reads {kept}: {paths:?}"
      );
      assert!(
        !paths.contains(&left),
        "{layout:?}: the scan skips {left}: {paths:?}"
      );
    }
  }
}

#[test]
fn a_config_naming_a_third_layout_refuses_the_scope_rather_than_assuming_root() {
  let (fx, _facade) = declared(Contracts::Dotclaude);
  let config = Project::config_path(fx.root());
  let text = std::fs::read_to_string(&config).expect("the config reads");
  std::fs::write(&config, text.replace("\"dotclaude\"", "\"elsewhere\""))
    .expect("the config writes");

  match Scanned::for_root(fx.root()) {
    Ok(_) => panic!("a config naming a third layout must not build a scope"),
    Err(error) => {
      assert!(matches!(error, SyncError::Project(_)), "{error:?}");
      assert!(
        error.to_string().contains("`contracts` is \"elsewhere\""),
        "{error}"
      );
    }
  }
}
