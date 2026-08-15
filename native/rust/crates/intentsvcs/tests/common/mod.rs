//! Shared fixtures for the WP-03 and WP-04 acceptance tests.
//!
//! One home for "make me a project on disk" (IN-EX-TEST-007 generalised): six
//! AT files need a fixture project, and six private copies would drift until
//! two tests disagreed about what a valid estate looks like -- which is the
//! failure mode where a test keeps passing because its fixture quietly stopped
//! resembling the thing under test.

#![allow(dead_code)]

use std::path::{Path, PathBuf};

use intentsvcs::model::{
  AcKind, AcState, AcceptanceTest, AtKind, AtStatus, Criterion, ISSUE_SCHEMA, Issue, IssueStatus,
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

/// A fixed project UUID, so envelope assertions compare against a constant
/// rather than against whatever was generated.
pub const PROJECT_ID: &str = "00000000-0000-0000-0000-00000000cc03";

/// The facade's ambient facts. `today` is FIXED: the facade owns no clock, so
/// a test never has to freeze one.
pub fn facade_ctx() -> intentsvcs::facade::FacadeContext {
  intentsvcs::facade::FacadeContext {
    principal: "cc".to_string(),
    project_id: PROJECT_ID.to_string(),
    version: VERSION.to_string(),
    today: "2026-08-14".to_string(),
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

  /// A facade over this fixture, against an in-memory store.
  pub fn facade(&self) -> intentsvcs::facade::Facade {
    intentsvcs::facade::Facade::open_in_memory(self.project(), facade_ctx())
      .expect("open fixture facade")
  }

  /// Make a directory unwritable, returning its previous mode so the caller
  /// can restore it -- a tempdir that cannot be written also cannot be
  /// cleaned up.
  ///
  /// This is how AC-04.1's mid-write failure is INJECTED rather than reasoned
  /// about: a real `rename` into a read-only directory, failing the way the
  /// operating system fails it, not a synthetic seam the production code knows
  /// about.
  #[cfg(unix)]
  pub fn make_readonly(&self, rel: &str) -> u32 {
    use std::os::unix::fs::PermissionsExt;
    let path = self.path(rel);
    let mode = std::fs::metadata(&path)
      .expect("metadata")
      .permissions()
      .mode();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o555)).expect("chmod");
    mode
  }

  #[cfg(unix)]
  pub fn restore_mode(&self, rel: &str, mode: u32) {
    use std::os::unix::fs::PermissionsExt;
    let _ = std::fs::set_permissions(self.path(rel), std::fs::Permissions::from_mode(mode));
  }

  pub fn path(&self, rel: &str) -> PathBuf {
    self.root().join(rel)
  }

  /// Write a thread's canonical JSON, and stub every test file its contract
  /// cites.
  ///
  /// The stubs are not decoration. The close gate reproduces v2's lint rules
  /// L2 and L3 -- an AT whose cited file does not exist BLOCKS, and so does one
  /// whose cited file does not carry the AT's own id, however green the row
  /// claims to be. A fixture thread citing paths that are not there is not a
  /// valid estate and would block every gate test for the wrong reason, so the
  /// stub carries its id exactly as a real test would.
  pub fn write_thread(&self, thread: &Thread) {
    let path = self.project().thread_json(&thread.id);
    std::fs::create_dir_all(path.parent().expect("thread dir")).expect("mkdir st");
    std::fs::write(&path, to_canonical_json(thread).expect("render canon")).expect("write thread");
    for test in &thread.tests {
      if let Some(cited) = test.file.as_deref() {
        self.write_file(cited, &format!("// {}: a cited test file\n", test.id));
      }
    }
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
    // A `wip` thread CARRYING a reason is the realistic shape, not a
    // contradiction: `st reopen` lands on `wip` and records why. Set here so
    // the round-trip tests actually carry the field -- a new field the fixture
    // leaves `None` round-trips vacuously, which is how a field that does not
    // survive the trip would go unnoticed by the very test written to catch it.
    status_reason: Some("reopened: AC-02.6 was added after the close".to_string()),
    created: "2026-08-14".to_string(),
    completed: None,
    acceptance: None,
    // MARKUP-BEARING BY DESIGN. `objective` and `context` are markdown under
    // D22, AT notes carry backticked paths routinely, and titles carry
    // underscores -- so the fixture carries every delimiter that could
    // interleave with markup the renderer adds.
    //
    // A tame fixture proves the LAYOUT half (column widths, blank runs,
    // trailing space) and passes while the DATA half is wide open: a value
    // containing markdown, wrapped in inline markup by the renderer, resolves
    // differently from how anyone meant it and the formatter re-emits its own
    // reading. Found live in ic's `dispatch-table.md`, where two spaces
    // vanished from `` `stzero`against`bin/intent_st_zero` `` and regeneration
    // put them back -- permanent oscillation.
    objective: "Ship `intent` v3.0.0 -- see `bin/intent_st_zero`.\n\nOne _major_ release, patched by 3.0.x, with snake_case_names and a | pipe.".to_string(),
    context: "v2 is 12,492 lines of bash where `every reader` reimplements parsing (NAME DRIFT: `stzero` against `bin/intent_st_zero`)."
      .to_string(),
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
        status_reason: None,
        objective: "Stand up the cargo workspace and reify the model.".to_string(),
        body: String::new(),
      },
      WorkPackage {
        seq: 3,
        title: "Ingest, views and sync -- `intent_st_zero` | strict".to_string(),
        scope: TShirt::L,
        status: WpStatus::Wip,
        status_reason: None,
        // MARKUP-BEARING AND DISTINCTIVE, for the same reason the thread's
        // prose fields are (D28). `quokka` appears in NO other field of this
        // fixture, so AC-06.7's "search finds a phrase that exists only in a
        // work package body" is asserted by a word that cannot match a title.
        objective: "Land strict ingest, deterministic views and the sync engine.".to_string(),
        body: "## Why the incumbents go\n\nThe quokka clause: a section the template never named, carried verbatim.\n\n## The seams\n\nA `pipe | inside` prose, and _emphasis_ the formatter rewrites.".to_string(),
      },
    ],
    criteria: vec![
      Criterion {
        id: "AC-03.1".to_string(),
        text: "strict ingest refuses schema-invalid canon".to_string(),
        kind: AcKind::Test,
        state: AcState::Computed,
      },
      Criterion {
        id: "AC-03.2".to_string(),
        text: "view rendering is deterministic".to_string(),
        kind: AcKind::NonTest,
        state: AcState::Satisfied {
          evidence: "the render itself".to_string(),
        },
      },
      Criterion {
        id: "AC-03.9".to_string(),
        text: "a descoped requirement".to_string(),
        kind: AcKind::Test,
        state: AcState::Descoped {
          to: "ST0057".to_string(),
          by: Some("hv".to_string()),
          reason: Some("moved with the daemon".to_string()),
        },
      },
      Criterion {
        id: "AC-03.8".to_string(),
        text: "a withdrawn requirement".to_string(),
        kind: AcKind::Test,
        state: AcState::Withdrawn {
          reason: "the premise did not reproduce".to_string(),
          by: None,
        },
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
        note: Some("red-first; see `bin/intent_st_zero` and _not_ `bin/intent`".to_string()),
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
