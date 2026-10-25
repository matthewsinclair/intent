//! ST0086 WP-02: intentd answers `estate_status` with the reader's own value,
//! without binding the connection to a project.
//!
//! **THE ROOT NAMES NO PROJECT, AND THAT IS THE ASSERTION.** Every bound op is
//! refused for a root that does not exist, so an answer here is the proof the
//! estate was served before any binding, and the registry afterwards proves
//! nothing was opened for it.

use std::path::{Path, PathBuf};

use crate::common::{RunningDaemon, short_dir};
use intentsvcs::wire::{Op, Request, Response};

/// The scrubbed session file intentsvcs pins the reader against, read where
/// it lives rather than copied.
fn session_fixture() -> String {
  let path = Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("../intentsvcs/tests/fixtures/estate/claude-session-2.1.310-macos.json");
  std::fs::read_to_string(&path).expect("the session fixture")
}

#[test]
fn the_estate_is_answered_for_a_root_that_names_no_project() {
  let claude = short_dir("estate-claude");
  std::fs::create_dir_all(claude.join("sessions")).expect("sessions dir");
  std::fs::write(claude.join("sessions/12345.json"), session_fixture()).expect("session file");
  let daemon = RunningDaemon::start_with(&[("CLAUDE_CONFIG_DIR", &claude)]);
  let nowhere = PathBuf::from("/nonexistent/estate-root");

  let status = match daemon.ask(Request {
    root: nowhere.clone(),
    op: Op::EstateStatus,
  }) {
    Response::EstateStatus { status } => status,
    other => panic!("expected the estate, got {other:?}"),
  };
  assert_eq!(status["schema"], intentsvcs::estate::STATUS_SCHEMA);
  let ids: Vec<&str> = status["sessions"]
    .as_array()
    .expect("sessions is a list")
    .iter()
    .filter_map(|row| row["session_id"].as_str())
    .collect();
  assert_eq!(
    ids,
    ["00000000-0000-4000-8000-000000000000"],
    "the daemon read some other Claude directory than the one it was given"
  );

  let Response::Registry { projects } = daemon.ask(Request {
    root: nowhere.clone(),
    op: Op::Registry,
  }) else {
    panic!("expected a registry listing");
  };
  assert!(
    projects.iter().all(|p| p.root != nowhere),
    "answering the estate registered the root it was sent: {projects:?}"
  );

  let _ = std::fs::remove_dir_all(&claude);
}

/// The page asks for the estate's FORM, by both of its paths, and is answered
/// with the status, unbound; a view that is not the estate still binds, so the
/// same nowhere root refuses it.
#[test]
fn the_estate_form_is_answered_unbound_and_no_other_view_is() {
  let claude = short_dir("estate-form");
  std::fs::create_dir_all(claude.join("sessions")).expect("sessions dir");
  std::fs::write(claude.join("sessions/12345.json"), session_fixture()).expect("session file");
  let daemon = RunningDaemon::start_with(&[("CLAUDE_CONFIG_DIR", &claude)]);
  let nowhere = PathBuf::from("/nonexistent/estate-form-root");

  for view in ["/estate/status", "/estate"] {
    match daemon.ask(Request {
      root: nowhere.clone(),
      op: Op::Form {
        view: view.to_string(),
      },
    }) {
      Response::EstateStatus { status } => {
        assert_eq!(
          status["schema"],
          intentsvcs::estate::STATUS_SCHEMA,
          "{view}"
        )
      }
      other => panic!("expected the estate for the form {view}, got {other:?}"),
    }
  }

  let refused = daemon.ask(Request {
    root: nowhere.clone(),
    op: Op::Form {
      view: "/settings".to_string(),
    },
  });
  assert!(
    matches!(refused, Response::Error { .. }),
    "a form that is not the estate was answered for a root that names no project: {refused:?}"
  );

  let Response::Registry { projects } = daemon.ask(Request {
    root: nowhere.clone(),
    op: Op::Registry,
  }) else {
    panic!("expected a registry listing");
  };
  assert!(
    projects.iter().all(|p| p.root != nowhere),
    "answering the estate's form registered the root it was sent: {projects:?}"
  );

  let _ = std::fs::remove_dir_all(&claude);
}
