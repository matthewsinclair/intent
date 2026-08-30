//! `AC-08.4`: **the daemon stops when asked over the wire, and answers before
//! it goes.**
//!
//! **THE ORDER IS THE CLAIM, NOT THE STOPPING.** A daemon that shut down and
//! then failed to reply would also stop, and `intent daemon stop` could not
//! tell that apart from the daemon dying mid-request -- so the verb would have
//! to report *it either worked or it crashed*. Receiving [`Response::Stopping`]
//! is what proves the reply was written first, because a closed connection is
//! what the other ordering produces.
//!
//! **AND IT IS THE PRIMARY PATH RATHER THAN THE ONLY ONE** (vc, 2026-08-30).
//! `launchd` sends `SIGTERM` regardless and a wedged daemon will not answer its
//! own socket, which is exactly when stopping it matters most. What the wire
//! buys is the removal of a race rather than the removal of the signal: the
//! daemon stops ITSELF, so the thing acting and the thing acted on are one
//! process and there is no window in which a pid stops meaning what it meant.

mod common;

use common::RunningDaemon;
use intentsvcs::daemon::Route;
use intentsvcs::wire::{Op, Request, Response};

/// How many times the route is asked before the test gives up on it.
///
/// A count rather than a deadline: what this requires is that the loop
/// TERMINATE, which says nothing about the time and keeps D42 true here.
const ATTEMPTS: u32 = 400;
const PAUSE: std::time::Duration = std::time::Duration::from_millis(20);

fn settle_to_in_process(daemon: &RunningDaemon) -> bool {
  for _ in 0..ATTEMPTS {
    if matches!(daemon.route(), Route::InProcess) {
      return true;
    }
    std::thread::sleep(PAUSE);
  }
  false
}

#[test]
fn the_daemon_answers_first_and_then_stops() {
  let daemon = RunningDaemon::start();
  let project = common::project("stopped");

  // **ANTI-VACUITY: IT MUST BE ANSWERING BEFORE IT IS ASKED TO STOP.** A
  // daemon that never came up would satisfy "it is not answering afterwards"
  // completely, and this test would pass having observed nothing.
  assert!(
    matches!(daemon.route(), Route::Daemon(_)),
    "the daemon was not answering before it was asked to stop, so the arm below proves nothing"
  );

  let answer = daemon.ask(Request {
    root: project.clone(),
    op: Op::Shutdown,
  });

  // **THIS IS THE ORDERING ASSERTION.** The fixture's `ask` panics on a closed
  // connection, so reaching a `Stopping` here means the reply was framed,
  // written and flushed while the process was still up.
  assert_eq!(
    answer,
    Response::Stopping,
    "the daemon did not answer the stop request before going"
  );

  assert!(
    settle_to_in_process(&daemon),
    "the daemon answered `Stopping` and was still answering {ATTEMPTS} attempts later, so it agreed to stop and did not"
  );

  let _ = std::fs::remove_dir_all(&project);
}

/// **THE CASE THE ROUTING COMMENT CLAIMS AND WOULD OTHERWISE ONLY ASSERT.**
///
/// `Op::Shutdown` is answered before the connection binds to a project, on the
/// grounds that the operator most likely to stop a daemon is the one whose
/// project will not open. **That is a claim about a code path, and an untested
/// claim about a code path is a comment.** If the ordering in `dispatch` were
/// ever moved below the binding check, this is the arm that notices -- and the
/// symptom in the field would be a daemon that cannot be stopped precisely on
/// the machine where it most needs stopping.
#[test]
fn a_root_that_is_not_a_project_can_still_stop_the_daemon() {
  let daemon = RunningDaemon::start();
  assert!(
    matches!(daemon.route(), Route::Daemon(_)),
    "the daemon was not answering before it was asked to stop"
  );

  let answer = daemon.ask(Request {
    root: std::path::PathBuf::from("/nonexistent/not-a-project"),
    op: Op::Shutdown,
  });

  assert_eq!(
    answer,
    Response::Stopping,
    "a stop request naming an unopenable root was refused rather than served, so a daemon cannot be stopped from a broken project"
  );
  assert!(
    settle_to_in_process(&daemon),
    "the daemon agreed to stop and did not"
  );
}
