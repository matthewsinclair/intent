//! `AT-08.3`: the positive case of the routing rule, driven against a daemon
//! that actually exists.
//!
//! **THIS IS THE ONE CASE THAT COULD NOT BE CONSTRUCTED, AND IT IS THE ONLY
//! ONE THIS FILE ADDS.** `cli_routing.rs` drives `AC-08.3` exhaustively and
//! well -- absent, stale, never-accepting, inherited-descriptor,
//! accept-and-close, mixed transports, concurrency. Every one of those is a
//! phantom, and **a phantom is better evidenced by construction than by a real
//! daemon**: the inherited-descriptor race is 1-in-300 and cannot be summoned
//! on demand, while a deterministic fixture reproduces the observable exactly.
//!
//! **BUT THE POSITIVE CASE OWES A REAL SUBJECT, AND UNTIL NOW IT HAD NONE.**
//! The suite was green at 11-of-11 while every "live daemon" in it was a bare
//! listener -- which IS the phantom, so the fixture standing for the good case
//! was an instance of the bad one and the suite's discriminating power was
//! zero at maximal count. No amount of positive-controlling the checker finds
//! that. What finds it is a real daemon, driven once.
//!
//! So: start `intentd`, ask the SHIPPED routing function what it sees, stop it,
//! and ask again. The second half is not decoration -- without it a `Daemon`
//! verdict could come from anything at that address, and the point is that it
//! came from this process.

mod common;

use common::RunningDaemon;
use intentsvcs::daemon::Route;

#[test]
fn a_running_intentd_is_routed_to_and_a_stopped_one_is_not() {
  let mut daemon = RunningDaemon::start();

  let endpoint = match daemon.route() {
    Route::Daemon(endpoint) => endpoint,
    Route::InProcess => panic!(
      "a real intentd is running and answering, and the routing rule sent this invocation in-process. That is the failure AC-08.3 exists to prevent: the in-process engine would open a store the daemon owns, which is two sync engines on one database"
    ),
  };

  // **THE ENDPOINT IS REPORTED, NOT ASSERTED AGAINST A CONSTANT.** There is no
  // port literal anywhere in this estate -- the daemon binds `:0` and publishes
  // what the kernel gave it -- so pinning one here would mint the constant the
  // design went out of its way not to have.
  assert!(
    daemon.home().join(".local/share/intent").exists(),
    "the daemon published under the isolated HOME it was given, not the operator's"
  );

  // **THE CONTROL THAT ATTRIBUTES THE VERDICT ABOVE TO THIS PROCESS.** Without
  // it, `Daemon` could have come from anything reachable at that address, and
  // the test would be evidence that something answered rather than that the
  // daemon did.
  daemon.stop_and_settle();
  assert!(
    matches!(daemon.route(), Route::InProcess),
    "the address kept routing to a daemon after the only process that could have been serving it was killed and reaped. The positive result above cannot then be attributed to it -- endpoint was {endpoint}"
  );
}
