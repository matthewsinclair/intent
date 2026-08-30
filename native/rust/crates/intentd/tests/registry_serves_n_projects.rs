//! `AT-08.1` / `AC-08.1`: N projects, per-connection binding, and a moved root
//! that surfaces rather than crashing.
//!
//! **DRIVEN AGAINST A REAL DAEMON WITH REAL PROJECTS, BECAUSE THE THREE CLAIMS
//! ARE ABOUT A PROCESS RATHER THAN A FUNCTION.** Serving two projects at once,
//! refusing a connection that wanders, and surviving a directory that moves are
//! all properties of the registry's state over time, and none of them is
//! visible to a unit test of any single call.

mod common;

use common::{RunningDaemon, project};
use intentsvcs::wire::{Op, Request, Response};

fn ask(op: Op, root: &std::path::Path) -> Request {
  Request {
    root: root.to_path_buf(),
    op,
  }
}

fn titles(response: &Response) -> Vec<String> {
  match response {
    Response::Threads { threads } => threads.iter().map(|t| t.title.clone()).collect(),
    other => panic!("expected a thread listing, got {other:?}"),
  }
}

fn thread_ids(response: &Response) -> Vec<String> {
  match response {
    Response::Threads { threads } => threads.iter().map(|t| t.id.clone()).collect(),
    other => panic!("expected a thread listing, got {other:?}"),
  }
}

#[test]
fn one_daemon_serves_two_projects_and_keeps_them_apart() {
  let daemon = RunningDaemon::start();
  let alpha = project("Alpha");
  let beta = project("Beta");

  let from_alpha = daemon.ask(ask(Op::ThreadList, &alpha));
  let from_beta = daemon.ask(ask(Op::ThreadList, &beta));

  // **THE DISCRIMINATING ASSERTION IS THAT THEY DIFFER, NOT THAT EACH
  // ANSWERED.** A registry that opened one store and served it to everybody
  // would answer both requests successfully and identically, which is the
  // failure this row exists to prevent -- and "both returned threads" cannot
  // see it.
  assert_eq!(titles(&from_alpha), vec!["Alpha thread".to_string()]);
  assert_eq!(
    titles(&from_beta),
    vec!["Beta thread".to_string()],
    "the second project answered with the FIRST project's content, so one store is being served to everybody"
  );
  assert_ne!(
    titles(&from_alpha),
    titles(&from_beta),
    "the two answers must differ, or this test cannot tell a registry from a single shared store"
  );
  assert_eq!(
    thread_ids(&from_alpha),
    thread_ids(&from_beta),
    "both are ST0001, which is exactly why the ID cannot be the discriminator and the title has to be"
  );
  let listed = daemon.ask(ask(Op::Registry, &alpha));
  match listed {
    Response::Registry { projects } => {
      let roots: Vec<_> = projects.iter().map(|p| p.root.clone()).collect();
      assert_eq!(
        roots.len(),
        2,
        "the daemon opened two distinct projects, not one: {roots:?}"
      );
      assert!(
        projects.iter().all(|p| p.root_exists),
        "both roots are present: {projects:?}"
      );
    }
    other => panic!("expected a registry listing, got {other:?}"),
  }

  let _ = std::fs::remove_dir_all(&alpha);
  let _ = std::fs::remove_dir_all(&beta);
}

#[test]
fn a_connection_is_bound_to_one_project_and_refuses_to_wander() {
  let daemon = RunningDaemon::start();
  let alpha = project("Alpha");
  let beta = project("Beta");

  let answers = daemon.conversation(&[
    ask(Op::ThreadList, &alpha),
    ask(Op::ThreadList, &beta),
    ask(Op::ThreadList, &alpha),
  ]);

  assert!(
    matches!(answers[0], Response::Threads { .. }),
    "the first request binds the connection and is served: {:?}",
    answers[0]
  );
  match &answers[1] {
    Response::Error { message, remedy } => {
      assert!(
        message.contains(&alpha.canonicalize().unwrap().display().to_string()),
        "the refusal names what the connection is bound to: {message}"
      );
      assert!(!remedy.is_empty(), "and what to do about it");
    }
    other => panic!(
      "a second project on a bound connection was SERVED. Every answer on this connection now depends on which request came first, which the client cannot see: {other:?}"
    ),
  }
  assert!(
    matches!(answers[2], Response::Threads { .. }),
    "the refusal did not break the binding: the connection still serves its own project"
  );

  let _ = std::fs::remove_dir_all(&alpha);
  let _ = std::fs::remove_dir_all(&beta);
}

#[test]
fn a_root_that_moves_surfaces_in_the_listing_and_does_not_take_the_daemon_with_it() {
  let mut daemon = RunningDaemon::start();
  let alpha = project("Alpha");
  let beta = project("Beta");

  // Both must be OPEN before one is moved: a project the daemon never opened
  // is not registered, so moving it would prove nothing about the registry.
  assert!(matches!(
    daemon.ask(ask(Op::ThreadList, &alpha)),
    Response::Threads { .. }
  ));
  assert!(matches!(
    daemon.ask(ask(Op::ThreadList, &beta)),
    Response::Threads { .. }
  ));

  // **CANONICALISED BEFORE THE MOVE, BECAUSE AFTERWARDS IT CANNOT BE.** The
  // daemon keys on the canonical form; once the directory is gone
  // `canonicalize` fails and a fallback to the raw path compares
  // `/tmp/...` against the daemon's `/private/tmp/...` on macOS -- a mismatch
  // that is entirely the test's and would read as a registry defect.
  let beta_canonical = beta
    .canonicalize()
    .expect("beta resolves while it still exists");
  let moved = beta.with_extension("moved");
  std::fs::rename(&beta, &moved).expect("move a registered project out from under the daemon");

  match daemon.ask(ask(Op::Registry, &alpha)) {
    Response::Registry { projects } => {
      assert_eq!(
        projects.len(),
        2,
        "**THE MOVED PROJECT IS STILL LISTED.** Dropping it would satisfy the criterion's words and hide the thing they were written for: the operator's question is why this project stopped working, and an entry missing from a list cannot answer it"
      );
      let gone = projects
        .iter()
        .find(|p| !p.root_exists)
        .expect("exactly the moved root reports itself absent");
      assert_eq!(
        gone.root, beta_canonical,
        "the absent entry is the one that moved"
      );
    }
    other => panic!("expected a registry listing, got {other:?}"),
  }

  // **AND THE DAEMON IS STILL SERVING EVERYTHING ELSE**, which is the half of
  // "never as a crash" that a listing cannot show.
  assert!(
    matches!(
      daemon.ask(ask(Op::ThreadList, &alpha)),
      Response::Threads { .. }
    ),
    "a moved root took the daemon's other projects with it"
  );
  daemon.stop_and_settle();

  let _ = std::fs::remove_dir_all(&alpha);
  let _ = std::fs::remove_dir_all(&moved);
}

#[test]
fn a_root_that_was_never_a_project_is_refused_with_a_remedy() {
  let daemon = RunningDaemon::start();
  let empty = common::short_dir("intentd-notaproject");

  match daemon.ask(ask(Op::ThreadList, &empty)) {
    Response::Error { message, remedy } => {
      assert!(!message.is_empty(), "it says what happened");
      assert!(
        remedy.contains("intent init"),
        "and names the thing that would fix it: {remedy}"
      );
    }
    other => panic!("a directory that is not a project was served: {other:?}"),
  }

  // The daemon survives it: a refusal is not a fault.
  let alpha = project("Alpha");
  assert!(matches!(
    daemon.ask(ask(Op::ThreadList, &alpha)),
    Response::Threads { .. }
  ));

  let _ = std::fs::remove_dir_all(&empty);
  let _ = std::fs::remove_dir_all(&alpha);
}
