//! The crate's ONE integration-test target.
//!
//! Every `.rs` directly under `tests/` used to be its own autodiscovered target,
//! and so its own separately linked executable against the whole dependency
//! graph. This file is the single target they are now modules of, per hv's
//! estate-wide ruling of 2026-08-27 (TN001).
//!
//! **THE FILES DID NOT MOVE.** `autotests = false` plus one `[[test]]` plus
//! `#[path]` gets the same single binary with every file exactly where it was,
//! so no acceptance-test row citing a test by path goes stale.
//!
//! **A FILE ADDED UNDER `tests/` NO LONGER RUNS ON ITS OWN.** `autotests = false`
//! sees to that, so an undeclared file is silently never compiled. That inverted
//! failure is the trade, and it is guarded -- see `tests/no_orphan_suite_member.rs`,
//! whose declaration below is load-bearing and must not be tidied away.

#[path = "roots_and_fixture_home.rs"]
mod roots_and_fixture_home;

#[path = "no_orphan_suite_member.rs"]
mod no_orphan_suite_member;

#[path = "abandoned_fixtures_are_swept.rs"]
mod abandoned_fixtures_are_swept;
