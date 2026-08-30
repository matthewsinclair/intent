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

/// **D56 AND THE ONE-BODY CLAIM `wire::ask` MAKES ABOUT ITSELF.**
///
/// The client's round trip -- the deadline, the write, the newline, what a
/// closed connection means -- is written ONCE and only the connect is behind a
/// `match` on the transport. **That collapse replaced two copies that agreed on
/// the day they were written**, which is the only day duplicated code ever does,
/// and its whole justification is that the two transports can no longer drift.
///
/// **A JUSTIFICATION NOTHING DRIVES IS A COMMENT.** Every other test in this
/// crate reaches the daemon over the unix socket, so before this the loopback
/// arm of `connect` had never been executed by anything -- the transport was
/// PROBED for liveness elsewhere and never asked a question. The two answers
/// must be identical because they are the same store reached two ways, which is
/// D56 stated as a test rather than as a decision.
#[test]
fn one_question_over_two_transports_gets_the_same_answer() {
  let daemon = RunningDaemon::start();
  let alpha = project("Alpha");

  let over_unix = daemon.ask(ask(Op::ThreadList, &alpha));
  let tcp = daemon
    .tcp()
    .expect("the daemon publishes a loopback address as well as a socket");
  let over_tcp = daemon.ask_over(&tcp, ask(Op::ThreadList, &alpha));

  assert_eq!(
    titles(&over_unix),
    vec!["Alpha thread".to_string()],
    "the unix transport answered with something other than this project's content"
  );
  assert_eq!(
    over_unix, over_tcp,
    "the same question over the two published transports returned different answers. \
     They reach one store through one client body, so a difference here is the client \
     behaving differently by transport -- which is the drift collapsing the two round \
     trips into one was supposed to make unrepresentable"
  );

  let _ = std::fs::remove_dir_all(&alpha);
}

/// How many ops the daemon says it has dispatched to `root`.
fn dispatched_for(response: &Response, root: &std::path::Path) -> u64 {
  match response {
    Response::Registry { projects } => {
      projects
        .iter()
        .find(|p| p.root == root)
        .unwrap_or_else(|| panic!("{} is not registered: {projects:?}", root.display()))
        .dispatched
    }
    other => panic!("expected a registry listing, got {other:?}"),
  }
}

/// **THE DISCRIMINATOR `AC-08.2` RESTS ON, AND BOTH ARMS ARE THE POINT.**
///
/// A dual-path harness comparing in-process against daemon agrees with itself
/// **by construction** wherever the CLI falls through -- so `both routes
/// answered` cannot tell a working client from one that routes nothing, and it
/// gets weaker every time the op set grows. Reading this counter around a single
/// verb gives per-verb attribution with no per-verb wire surface.
///
/// **THE SECOND ARM IS THE ONE THAT CLOSES THE PARTITION.** A served op moving
/// the count by 1 says routing happened somewhere; an UNCOUNTED op leaving it
/// UNTOUCHED is what proves fallthrough is fallthrough rather than a silent
/// route. Every op is in exactly one bucket and the buckets sum to the surface.
#[test]
fn a_dispatched_op_is_counted_and_an_uncounted_one_is_not() {
  let daemon = RunningDaemon::start();
  let alpha = project("Alpha");

  // Open the project so it is registered, and take the baseline AFTER that --
  // the first contact is itself a dispatch, and folding it into the delta would
  // measure registration rather than the verb under test.
  assert!(matches!(
    daemon.ask(ask(Op::ThreadList, &alpha)),
    Response::Threads { .. }
  ));
  let before = dispatched_for(
    &daemon.ask(ask(Op::Registry, &alpha)),
    &alpha.canonicalize().unwrap(),
  );

  daemon.ask(ask(Op::ThreadList, &alpha));
  let after = dispatched_for(
    &daemon.ask(ask(Op::Registry, &alpha)),
    &alpha.canonicalize().unwrap(),
  );

  assert_eq!(
    after,
    before + 1,
    "one dispatched op must move the count by exactly one. Any other delta means the counter is \
     measuring something other than dispatches -- connections, or probes -- and a harness resting \
     on it would read every fallthrough as a route"
  );

  // **THE UNCOUNTED ARM, AND IT IS ALSO THIS INSTRUMENT'S SELF-DEFENCE.**
  // `Op::Registry` is how the count is READ, so a registry that counted would
  // move the number it is being asked for -- the observer changing the
  // observable. Two consecutive reads with nothing between them must agree.
  let twice = dispatched_for(
    &daemon.ask(ask(Op::Registry, &alpha)),
    &alpha.canonicalize().unwrap(),
  );
  assert_eq!(
    twice, after,
    "reading the registry changed the count it reports, so every delta measured through it \
     includes the measurement itself. `Op::Registry` is declared UNCOUNTED in intentsvcs::wire \
     precisely so this cannot happen"
  );

  // And the declaration and the behaviour are checked against each other,
  // rather than the test carrying its own idea of which ops are exempt.
  assert!(
    intentsvcs::wire::UNCOUNTED.contains(&Op::Registry),
    "the behaviour above holds of `Op::Registry` while the shipped declaration does not name it, \
     so a harness deriving its expectation from the declaration would expect the opposite"
  );

  let _ = std::fs::remove_dir_all(&alpha);
}
