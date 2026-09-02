//! The crate's ONE integration-test target.
//!
//! Every `.rs` directly under `tests/` used to be its own autodiscovered target, and
//! so its own separately linked executable against the whole dependency graph. This
//! file is the single target they are now modules of, per hv's estate-wide ruling of
//! 2026-08-27. Laksa took it on 2026-08-27 and Lamplight partially; Intent, which made
//! the ruling, had not -- and had grown to 257 targets, 201 of them in that same month.
//!
//! **THE FILES DID NOT MOVE, AND THAT IS THE WHOLE REASON THIS WAS CHEAP.** The obvious
//! consolidation relocates everything under `tests/suite/`, which breaks every
//! acceptance-test row citing a test by path -- and `intent at` has no verb that
//! retargets a row's file. `autotests = false` plus one `[[test]]` plus `#[path]` gets
//! the same single binary with every file exactly where it was, so no citation goes
//! stale because nothing it cites ever moves.
//!
//! **AND THE COST THAT IS REAL: these were separate PROCESSES and are now threads in
//! one.** Anything touching process-global state -- `set_current_dir`, `env::set_var`,
//! a fixed port, a shared socket -- stops failing cleanly and starts being flaky, which
//! is worse because it gets blamed elsewhere. Measured before the merge rather than
//! hoped for afterwards: across all 257 files exactly ONE mutates process state
//! (`intent-cli/tests/dual_path_conformance.rs`, `set_current_dir` at :199), and it
//! keeps its own `[[test]]` target for that reason. No test spawns cargo, so the
//! inner-build deadlock cannot arise; the one fixed port is written to a file and
//! parsed, never bound; and every `intentd.sock` path is per-test under a tempdir.
//!
//! **A FILE ADDED UNDER `tests/` NO LONGER RUNS ON ITS OWN.** `autotests = false` sees
//! to that, so an undeclared file is silently never compiled. That inverted failure is
//! the trade, and it is guarded -- see `tests/no_orphan_suite_member.rs`.

#[path = "common/mod.rs"]
mod common;

#[path = "arguments_do_not_start_a_daemon.rs"]
mod arguments_do_not_start_a_daemon;
#[path = "daemon_stops_when_asked.rs"]
mod daemon_stops_when_asked;
#[path = "daemon_watch.rs"]
mod daemon_watch;
#[path = "graphql_answers_through_the_store_door.rs"]
mod graphql_answers_through_the_store_door;
#[path = "no_orphan_suite_member.rs"]
mod no_orphan_suite_member;
#[path = "one_declaration_two_realisers.rs"]
mod one_declaration_two_realisers;
#[path = "one_store_door.rs"]
mod one_store_door;
#[path = "registry_serves_n_projects.rs"]
mod registry_serves_n_projects;
#[path = "routing_against_a_real_daemon.rs"]
mod routing_against_a_real_daemon;
