//! `Op::Graphql` is answered by the store thread through the facade, like
//! `ThreadList`, and it counts as one dispatch (`AC-00.4`, `AC-09.2`).
//!
//! **THE DISCRIMINATOR IS THE COUNTER, NOT THE ANSWER.** An empty project
//! answers `{"threads":[]}` from anywhere, so the project carries a minted
//! thread and the test reads `dispatched` before and after: a served op moves
//! it by exactly one, which is the same proof `AC-08.2`'s harness rests on.

mod common;

use std::path::Path;

use common::{RunningDaemon, project};
use intentsvcs::wire::{Op, Request, Response};

const MINTED: &str = "Minted for the hatch";

/// A project with one thread the daemon can be asked about.
fn project_with_a_thread() -> (std::path::PathBuf, String) {
  let root = project("Hatch");
  let opened = intentsvcs::project::Project::open(&root).expect("the project just created");
  let ctx = intentsvcs::facade::FacadeContext {
    principal: "test".to_string(),
    project_id: opened.config().project_id.clone().unwrap_or_default(),
    version: env!("CARGO_PKG_VERSION").to_string(),
  };
  let mut facade = intentsvcs::facade::Facade::open(opened, ctx).expect("open it");
  let id = facade.st_new(MINTED).expect("mint one thread");
  (root, id)
}

fn dispatched(daemon: &RunningDaemon, root: &Path) -> u64 {
  let canonical = root.canonicalize().expect("the root exists");
  match daemon.ask(Request {
    root: root.to_path_buf(),
    op: Op::Registry,
  }) {
    Response::Registry { projects } => projects
      .iter()
      .find(|p| p.root == canonical)
      .map(|p| p.dispatched)
      .unwrap_or(0),
    other => panic!("the registry answered {other:?}"),
  }
}

/// Variables stay `None` here on purpose: naming the JSON crate in this
/// crate's tests would put a format into a manifest the wire module keeps it
/// out of, and variables are driven where the crate can name it -- the
/// intentsvcs resolver test and the CLI's end-to-end drive.
fn graphql(query: &str) -> Op {
  Op::Graphql {
    query: query.to_string(),
    variables: None,
  }
}

#[test]
fn a_document_is_answered_from_the_project_and_counted_as_one_dispatch() {
  let daemon = RunningDaemon::start();
  let (root, id) = project_with_a_thread();

  // First contact registers the project; read the baseline AFTER it so the
  // delta below is the document's alone.
  let _ = daemon.ask(Request {
    root: root.clone(),
    op: Op::ThreadList,
  });
  let before = dispatched(&daemon, &root);

  let answer = daemon.ask(Request {
    root: root.clone(),
    op: graphql("{ threads { id title } }"),
  });
  let Response::Graphql { response } = answer else {
    panic!("a GraphQL request was answered with {answer:?}");
  };
  assert!(
    response["errors"].is_null(),
    "a valid read carries no errors: {response}"
  );
  let threads = response["data"]["threads"]
    .as_array()
    .expect("threads is a list");
  assert!(
    threads
      .iter()
      .any(|t| t["id"] == id.as_str() && t["title"] == MINTED),
    "the minted thread reached the answer through the daemon's facade: {response}"
  );
  assert_eq!(
    dispatched(&daemon, &root),
    before + 1,
    "one document is one dispatch"
  );
}

#[test]
fn a_mutation_is_refused_inside_the_answer_and_never_as_a_wire_error() {
  // **THE SCHEMA'S REFUSAL TRAVELS ON THE SPEC'S CHANNEL.** `EmptyMutation`
  // ships, so the document fails validation; the wire still answers
  // `Response::Graphql`, with the refusal in `errors` where every GraphQL
  // client already looks. `Response::Error` is for a daemon that could not
  // serve, and this daemon served.
  let daemon = RunningDaemon::start();
  let (root, _) = project_with_a_thread();

  let answer = daemon.ask(Request {
    root: root.clone(),
    op: graphql("mutation { anything }"),
  });
  let Response::Graphql { response } = answer else {
    panic!("a refused mutation must still be a GraphQL response, got {answer:?}");
  };
  assert!(response["data"].is_null(), "{response}");
  assert!(
    response["errors"].as_array().is_some_and(|e| !e.is_empty()),
    "{response}"
  );
}
