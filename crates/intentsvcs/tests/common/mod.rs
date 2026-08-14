//! Shared fixtures for the WP-03 acceptance tests.
//!
//! One home for "make me a project on disk" (IN-EX-TEST-007 generalised): six
//! AT files need a fixture project, and six private copies would drift until
//! two tests disagreed about what a valid estate looks like -- which is the
//! failure mode where a test keeps passing because its fixture quietly stopped
//! resembling the thing under test.

#![allow(dead_code)]

use std::path::{Path, PathBuf};

use intentsvcs::model::{
  AcKind, AcScope, AcceptanceTest, AtKind, AtStatus, Criterion, ISSUE_SCHEMA, Issue, IssueStatus,
  Related, THREAD_SCHEMA, TShirt, Thread, ThreadStatus, WorkPackage, WpStatus, to_canonical_json,
};
use intentsvcs::project::Project;
use intentsvcs::views::RenderContext;

/// The tool version the fixtures render with. Fixed, never read from the
/// build: a view that changed bytes because the crate version bumped would
/// make every determinism test a version tripwire.
pub const VERSION: &str = "3.0.0-test";

pub fn ctx() -> RenderContext<'static> {
  RenderContext {
    version: VERSION,
    todo_watermark: None,
  }
}

pub struct Fixture {
  pub dir: tempfile::TempDir,
}

impl Fixture {
  /// A project with config.json and nothing else.
  pub fn new() -> Self {
    let dir = tempfile::tempdir().expect("tempdir");
    let config = dir.path().join("intent").join(".config");
    std::fs::create_dir_all(&config).expect("mkdir .config");
    std::fs::write(
      config.join("config.json"),
      "{\n  \"intent_version\": \"3.0.0\",\n  \"project_name\": \"Fixture\",\n  \"author\": \"cc\",\n  \"intent_dir\": \"intent\",\n  \"languages\": [\"rust\"]\n}\n",
    )
    .expect("write config");
    Self { dir }
  }

  pub fn root(&self) -> &Path {
    self.dir.path()
  }

  pub fn project(&self) -> Project {
    Project::open(self.root()).expect("open fixture project")
  }

  pub fn path(&self, rel: &str) -> PathBuf {
    self.root().join(rel)
  }

  /// Write a thread's canonical JSON.
  pub fn write_thread(&self, thread: &Thread) {
    let path = self.project().thread_json(&thread.id);
    std::fs::create_dir_all(path.parent().expect("thread dir")).expect("mkdir st");
    std::fs::write(&path, to_canonical_json(thread).expect("render canon")).expect("write thread");
  }

  /// Write raw bytes as a thread's canon -- for the cases where the point is
  /// that the bytes are NOT valid.
  pub fn write_raw_thread(&self, id: &str, json: &str) {
    let path = self.project().thread_json(id);
    std::fs::create_dir_all(path.parent().expect("thread dir")).expect("mkdir st");
    std::fs::write(&path, json).expect("write raw thread");
  }

  pub fn write_issue(&self, issue: &Issue) {
    let path = self.project().issue_json(issue.number);
    std::fs::create_dir_all(path.parent().expect("issues dir")).expect("mkdir issues");
    std::fs::write(&path, to_canonical_json(issue).expect("render canon")).expect("write issue");
  }

  /// Write an authored prose file beside a thread's canon.
  pub fn write_prose(&self, id: &str, name: &str, text: &str) {
    let path = self.project().thread_dir(id).join(name);
    std::fs::create_dir_all(path.parent().expect("thread dir")).expect("mkdir st");
    std::fs::write(&path, text).expect("write prose");
  }

  pub fn write_file(&self, rel: &str, text: &str) {
    let path = self.path(rel);
    std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
    std::fs::write(&path, text).expect("write file");
  }

  pub fn read(&self, rel: &str) -> String {
    std::fs::read_to_string(self.path(rel)).expect("read file")
  }
}

/// A complete, valid thread exercising every modelled shape: both AC kinds,
/// all three AC scopes, a legacy-carried AT, related links, prose fields with
/// blank lines.
pub fn sample_thread(id: &str) -> Thread {
  Thread {
    schema: THREAD_SCHEMA.to_string(),
    id: id.to_string(),
    title: "Intent v3.0.0".to_string(),
    slug: Some("intent-v3".to_string()),
    status: ThreadStatus::Wip,
    created: "2026-08-14".to_string(),
    completed: None,
    acceptance: None,
    objective: "Ship Intent v3.0.0.\n\nOne major release, patched by 3.0.x.".to_string(),
    context: "v2 is 12,492 lines of bash where every reader reimplements parsing.".to_string(),
    related: vec![
      Related {
        id: "ST0043".to_string(),
        note: Some("the v2 convergent orchestrator".to_string()),
      },
      Related {
        id: "ST0044".to_string(),
        note: None,
      },
    ],
    wps: vec![
      WorkPackage {
        seq: 2,
        title: "Workspace and reified model".to_string(),
        scope: TShirt::L,
        status: WpStatus::Done,
      },
      WorkPackage {
        seq: 3,
        title: "Ingest, views and sync".to_string(),
        scope: TShirt::L,
        status: WpStatus::Wip,
      },
    ],
    criteria: vec![
      Criterion {
        id: "AC-03.1".to_string(),
        text: "strict ingest refuses schema-invalid canon".to_string(),
        kind: AcKind::Test,
        scope: AcScope::InScope,
        evidence: None,
        satisfied: None,
      },
      Criterion {
        id: "AC-03.2".to_string(),
        text: "view rendering is deterministic".to_string(),
        kind: AcKind::NonTest,
        scope: AcScope::InScope,
        evidence: Some("the render itself".to_string()),
        satisfied: Some(true),
      },
      Criterion {
        id: "AC-03.9".to_string(),
        text: "a descoped requirement".to_string(),
        kind: AcKind::Test,
        scope: AcScope::Descoped {
          to: "ST0057".to_string(),
          by: Some("hv".to_string()),
          reason: Some("moved with the daemon".to_string()),
        },
        evidence: None,
        satisfied: None,
      },
      Criterion {
        id: "AC-03.8".to_string(),
        text: "a withdrawn requirement".to_string(),
        kind: AcKind::Test,
        scope: AcScope::Withdrawn {
          reason: "the premise did not reproduce".to_string(),
          by: None,
        },
        evidence: None,
        satisfied: None,
      },
    ],
    tests: vec![
      AcceptanceTest {
        id: "AT-03.1".to_string(),
        kind: AtKind::Test,
        file: Some("crates/intentsvcs/tests/ingest_refusal.rs".to_string()),
        prose: None,
        covers: vec!["AC-03.1".to_string()],
        status: AtStatus::Green,
        note: Some("red-first".to_string()),
        legacy: None,
      },
      AcceptanceTest {
        id: "AT-03.2".to_string(),
        kind: AtKind::NonTest,
        file: None,
        prose: Some("the render was eyeballed".to_string()),
        covers: vec!["AC-03.2".to_string()],
        status: AtStatus::Na,
        note: None,
        legacy: None,
      },
      AcceptanceTest {
        id: "AT-03.7".to_string(),
        kind: AtKind::Test,
        file: None,
        prose: None,
        covers: vec!["AC-03.1".to_string()],
        status: AtStatus::Green,
        note: None,
        legacy: Some(intentsvcs::model::Legacy {
          raw: "test/some_test.exs::\"the named case\"".to_string(),
        }),
      },
    ],
  }
}

pub fn sample_issue(number: u32) -> Issue {
  Issue {
    schema: ISSUE_SCHEMA.to_string(),
    number,
    slug: "credo-checks".to_string(),
    title: "prune the dead mechanism".to_string(),
    status: IssueStatus::Closed,
    severity: Some("medium".to_string()),
    created: "2026-08-14".to_string(),
    closed: Some("2026-08-14".to_string()),
  }
}
