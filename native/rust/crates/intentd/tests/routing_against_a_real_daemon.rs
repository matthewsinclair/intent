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

use crate::common::{RunningDaemon, short_dir};
use intentsvcs::daemon::{self, Route};
use intentsvcs::userstate::{self, Dirs};

/// Issue I75b8995: a stale address file that names ANOTHER HOME's live daemon
/// does not route there.
///
/// Home B's daemon is gone and left its token and address behind; the address
/// names the port home A's real intentd now holds, as the shared preferred
/// port makes happen. Before the identity probe, B's CLI routed to A's daemon.
#[test]
fn a_stale_address_naming_another_homes_daemon_does_not_route_there() {
  let a = RunningDaemon::start();
  let a_tcp = a
    .tcp()
    .expect("the control: home A's own daemon keeps its TCP candidate");

  let b = Dirs::at_home(&short_dir("intent-fixture-foreign-home"));
  let _b_token = daemon::Token::mint_under(&b).expect("home B's token, left by its dead daemon");
  let a_address = userstate::daemon_address_file_under(&Dirs::at_home(a.home()));
  let b_address = userstate::daemon_address_file_under(&b);
  std::fs::copy(&a_address, &b_address).expect("home B's stale address names A's port");

  let candidates = daemon::candidates_under(&b).expect("home B's candidates");
  assert!(
    !candidates.contains(&a_tcp),
    "home B kept {a_tcp} as a candidate, but the daemon there named home A: a CLI in B would send its verbs to A's daemon. Candidates: {candidates:?}"
  );
  assert_eq!(
    daemon::route(&candidates),
    Route::InProcess,
    "home B has no daemon of its own, so it runs in-process"
  );
}

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
    daemon.home().join(".local/state/intent/run").exists(),
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
