//! The `agents` payload: the second kind `Payload` serves, and the first one
//! whose source shape and target shape differ.
//!
//! **THIS IS THE KIND v3 NEVER HAD, AND THE GAP WAS LIVE RATHER THAN
//! THEORETICAL.** Measured on the machine this was written on: nine subagents
//! in canon, eight in `~/.claude/agents/`, every one of them installed by v2 in
//! April and none since. `critic-prose` was the missing one -- dispatched by
//! four shipped skills including `in-session`, absent from the operator's
//! agents directory, and reachable by no v3 verb at all.
//!
//! **EVERY ARM HERE IS ABOUT THE SHAPE DIFFERENCE, BECAUSE EVERYTHING ELSE IS
//! ALREADY COVERED BY `skills_sync.rs` OVER THE SAME CODE.** The lifecycle --
//! baselines, conflict detection, forced discards, the `Outcome` vocabulary --
//! is one implementation serving both kinds, so re-asserting it here would test
//! the same lines twice and drift the day one copy was edited. What is genuinely
//! new is: a unit whose canon holds more than it installs, a target that is a
//! file rather than a directory, and a name that has to survive a rename in
//! both directions.

use std::fs;
use std::path::PathBuf;

use intentsvcs::payload::{Kind, Outcome, Payload, Shape};

struct Fixture {
  _tmp: tempfile::TempDir,
  install: PathBuf,
  target: PathBuf,
  manifest: PathBuf,
}

impl Fixture {
  fn new() -> Self {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().to_path_buf();
    let install = root.join("install");
    fs::create_dir_all(install.join("intent/plugins/claude/subagents")).unwrap();
    // `install::MARKER` -- what makes a tree an install.
    fs::create_dir_all(install.join("lib/templates")).unwrap();
    Self {
      install,
      target: root.join("home/.claude/agents"),
      manifest: root.join("home/.intent/subagents/installed-subagents.v3.json"),
      _tmp: tmp,
    }
  }

  fn agents(&self) -> Payload {
    Payload::new(
      Kind::Agents,
      &self.install,
      None,
      self.target.clone(),
      self.manifest.clone(),
    )
  }

  fn canon(&self) -> PathBuf {
    self.install.join("intent/plugins/claude/subagents")
  }

  /// A canon subagent as the estate actually stores one: `agent.md` beside a
  /// `metadata.json` that is never installed.
  fn plant(&self, name: &str, body: &str, metadata: &str) {
    let dir = self.canon().join(name);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("agent.md"), body).unwrap();
    fs::write(dir.join("metadata.json"), metadata).unwrap();
  }

  fn installed_bytes(&self, name: &str) -> Option<String> {
    fs::read_to_string(self.target.join(format!("{name}.md"))).ok()
  }
}

fn names(v: &[&str]) -> Vec<String> {
  v.iter().map(|s| s.to_string()).collect()
}

/// **THE WHOLE POINT, IN ONE ARM: `<name>/agent.md` LANDS AS `<name>.md`.**
#[test]
fn a_subagent_installs_as_one_renamed_file_and_leaves_its_metadata_behind() {
  let fx = Fixture::new();
  fx.plant("critic-prose", "# critic-prose\nbody\n", r#"{"v":1}"#);

  let report = fx
    .agents()
    .install(&names(&["critic-prose"]), false)
    .unwrap();
  assert_eq!(report.steps.len(), 1);
  assert!(
    matches!(report.steps[0].outcome, Outcome::Installed { files: 1 }),
    "expected a one-file install, got {:?}",
    report.steps[0].outcome
  );

  assert_eq!(
    fx.installed_bytes("critic-prose").as_deref(),
    Some("# critic-prose\nbody\n"),
    "the installed file must carry the canon agent.md bytes"
  );

  // **THE NEGATIVE IS THE HALF THAT MATTERS.** A tree-shaped install would have
  // put a directory here carrying metadata.json, which Claude Code does not
  // read and would not recognise as an agent.
  assert!(
    !fx.target.join("critic-prose").is_dir(),
    "a subagent must not install as a directory"
  );
  assert!(
    !fx.target.join("metadata.json").exists() && !fx.target.join("critic-prose.md").is_dir(),
    "nothing but the renamed agent.md may reach the target"
  );
}

/// The manifest describes what the operator would find, not what canon holds.
#[test]
fn the_manifest_records_the_installed_name_and_not_the_source_name() {
  let fx = Fixture::new();
  fx.plant("diogenes", "a\n", "{}");
  fx.agents().install(&names(&["diogenes"]), false).unwrap();

  let entry = fx
    .agents()
    .manifest()
    .unwrap()
    .installed
    .into_iter()
    .find(|e| e.name == "diogenes")
    .expect("the install is recorded");

  assert_eq!(
    entry.files,
    vec!["diogenes.md".to_string()],
    "recording `agent.md` would name a file that is not in the target"
  );
}

/// **THE CHECKSUM SCOPE DECISION, DRIVEN IN BOTH DIRECTIONS.**
///
/// This is the arm that would have been silently wrong under a tree hash of the
/// canon directory: `metadata.json` moving is not a change to anything the
/// operator has, so a sync must report `UpToDate` -- while `agent.md` moving is
/// exactly the change a sync exists to carry.
#[test]
fn only_the_installed_file_decides_whether_a_sync_has_work() {
  let fx = Fixture::new();
  fx.plant("socrates", "one\n", r#"{"version":1}"#);
  fx.agents().install(&names(&["socrates"]), false).unwrap();

  // Control: nothing moved at all.
  let quiet = fx.agents().sync(false).unwrap();
  assert!(
    matches!(quiet.steps[0].outcome, Outcome::UpToDate),
    "an untouched pair must be up to date, got {:?}",
    quiet.steps[0].outcome
  );

  // The metadata moves. Nothing the operator holds has changed.
  fs::write(
    fx.canon().join("socrates/metadata.json"),
    r#"{"version":2,"note":"reworded"}"#,
  )
  .unwrap();
  let after_metadata = fx.agents().sync(false).unwrap();
  assert!(
    matches!(after_metadata.steps[0].outcome, Outcome::UpToDate),
    "metadata is not installed, so moving it is not work: got {:?}",
    after_metadata.steps[0].outcome
  );

  // The agent body moves. This is the change a sync is for.
  fs::write(fx.canon().join("socrates/agent.md"), "two\n").unwrap();
  let after_body = fx.agents().sync(false).unwrap();
  assert!(
    matches!(after_body.steps[0].outcome, Outcome::Updated { .. }),
    "a changed agent.md must sync, got {:?}",
    after_body.steps[0].outcome
  );
  assert_eq!(fx.installed_bytes("socrates").as_deref(), Some("two\n"));
}

/// The name survives the rename in both directions.
#[test]
fn an_installed_agent_is_discovered_under_the_name_its_verbs_accept() {
  let fx = Fixture::new();
  fx.plant("critic-rust", "r\n", "{}");
  fx.plant("critic-shell", "s\n", "{}");
  fx.agents()
    .install(&names(&["critic-rust", "critic-shell"]), false)
    .unwrap();

  assert_eq!(
    fx.agents().installed().unwrap(),
    names(&["critic-rust", "critic-shell"]),
    "installed() must strip the .md this tool appended, or it reports names no verb takes"
  );
  assert!(fx.agents().is_installed("critic-rust"));
  assert!(!fx.agents().is_installed("critic-rust.md"));
}

/// A directory in canon without the marker is not a unit -- the same rule
/// skills already hold, keyed on this kind's marker.
#[test]
fn a_canon_directory_without_an_agent_md_is_not_a_subagent() {
  let fx = Fixture::new();
  fs::create_dir_all(fx.canon().join("not-an-agent")).unwrap();
  fs::write(fx.canon().join("not-an-agent/README.md"), "x").unwrap();
  fx.plant("real", "r\n", "{}");

  let available: Vec<String> = fx
    .agents()
    .available()
    .unwrap()
    .into_iter()
    .map(|o| o.name)
    .collect();
  assert_eq!(available, names(&["real"]));
}

/// **UNINSTALL REMOVES ONLY WHAT THIS BUILD RECORDED WRITING, AND THE CASE THAT
/// PROVES IT IS THE ONE ON EVERY REAL MACHINE TODAY.** Every kind's units share
/// one directory here, so an unrecorded `<name>.md` is the operator's own agent
/// or one v2 installed -- and matching on the name alone would delete it.
#[test]
fn uninstall_leaves_an_agent_this_build_did_not_install() {
  let fx = Fixture::new();
  fx.plant("intent", "i\n", "{}");

  // As v2 left it: the file is there, the v3 manifest has never seen it.
  fs::create_dir_all(&fx.target).unwrap();
  fs::write(fx.target.join("intent.md"), "installed by v2\n").unwrap();

  let report = fx.agents().uninstall(&names(&["intent"])).unwrap();
  match &report.steps[0].outcome {
    Outcome::Removed { removed, left } => {
      assert!(
        removed.is_empty(),
        "nothing was recorded, so nothing may go"
      );
      assert_eq!(left, &names(&["intent.md"]));
    }
    other => panic!("expected Removed, got {other:?}"),
  }
  assert_eq!(
    fx.installed_bytes("intent").as_deref(),
    Some("installed by v2\n"),
    "the operator's file must survive"
  );

  // Control: one this build DID install goes.
  fx.agents().install(&names(&["intent"]), true).unwrap();
  fx.agents().uninstall(&names(&["intent"])).unwrap();
  assert!(
    fx.installed_bytes("intent").is_none(),
    "a recorded install must be removable"
  );
}

/// The two kinds really do differ where this module says they do, and agree
/// everywhere else. A guard on the enum itself, so a third kind added later
/// cannot quietly reuse a path that is already spoken for.
#[test]
fn the_kinds_declare_distinct_paths_and_only_one_installs_as_a_file() {
  assert_ne!(Kind::Skills.canon_subdir(), Kind::Agents.canon_subdir());
  assert_ne!(Kind::Skills.target_subdir(), Kind::Agents.target_subdir());
  assert_ne!(Kind::Skills.marker(), Kind::Agents.marker());
  assert_ne!(
    Kind::Skills.manifest_relative(),
    Kind::Agents.manifest_relative()
  );
  assert_ne!(Kind::Skills.scope_token(), Kind::Agents.scope_token());
  assert_eq!(Kind::Skills.shape(), Shape::Tree);
  assert_eq!(Kind::Agents.shape(), Shape::SingleFile);
}
