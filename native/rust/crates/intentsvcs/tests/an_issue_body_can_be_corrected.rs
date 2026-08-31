//! **AN ISSUE BODY WAS WRITE-ONCE, AND TWO FINDINGS IN ONE SESSION WERE FILED
//! WRONG BECAUSE OF IT.**
//!
//! `issues add --body/--from` landed on 2026-08-27 and gave the create door its
//! prose. Nothing could correct one afterwards: under the disk-optional model
//! there is no file to edit, so a filing with a wrong premise stayed wrong
//! permanently.
//!
//! **BOTH INSTANCES LANDED IN THE RECORD OF A FINDING ABOUT THE MISSING VERB.**
//! vc filed `0179` -- *an issue body is write-once* -- as a fresh discovery,
//! four days after hv had ruled this package built, because a fold had moved
//! the ruling to `.history/`. cc filed `0183` with a remedy that measurement
//! then proved wrong, and could not correct the body. hv re-sequenced the
//! package on those two, recorded at `b1bf4cea`.
//!
//! # What this file pins, and why the refusals are the interesting half
//!
//! A verb that writes prose is easy to get right and easy to make destructive.
//! **The two refusals below are not symmetric with `issues add` and must not
//! become so**: `add` leaves a body empty because an unwritten body is a STATE,
//! and editing one to empty is an ERASURE whose only other copy is the event
//! log. So the arms that matter most are the ones asserting that a refused edit
//! left the prose exactly where it was.
//!
//! **NOT DRIVEN HERE, STATED INSTEAD:** that the event payload records a byte
//! count and never the prose itself. That is a property of the `apply` call and
//! this fixture has no reader for the event log, so asserting it would mean
//! asserting this file's own idea of the source. It is declared in
//! `Facade::issue_edit` beside the code that holds it.

mod common;

use common::Fixture;

const ORIGINAL: &str = "the original premise, which measurement later disproved";

fn with_an_issue(fx: &Fixture) {
  fx.facade()
    .issue_add("a filing with a wrong premise", None, None, ORIGINAL)
    .expect("the create door works");
}

#[test]
fn a_correction_replaces_the_prose() {
  let fx = Fixture::new();
  with_an_issue(&fx);

  fx.facade()
    .issue_edit(1, "the corrected premise, measured")
    .expect("an issue body can be corrected");

  assert_eq!(
    fx.facade().issue_show(1).expect("the issue is there").body,
    "the corrected premise, measured",
    "the correction did not reach the store, so the verb reports a write it did not do"
  );
}

/// **THE ARM THAT MATTERS: A REFUSED EDIT MUST NOT HAVE ALREADY WRITTEN.**
///
/// A refusal that mutates first is worse than no refusal at all -- it destroys
/// the prose AND tells the operator nothing happened, so nobody goes looking.
#[test]
fn an_empty_body_is_refused_and_the_prose_survives() {
  let fx = Fixture::new();
  with_an_issue(&fx);

  let refused = fx
    .facade()
    .issue_edit(1, "   \n  ")
    .expect_err("an empty body erases prose and must be refused");

  let said = refused.to_string();
  assert!(
    said.contains("ERASE"),
    "the refusal must say what it is protecting, or it reads as a validation quibble: {said}"
  );

  assert_eq!(
    fx.facade().issue_show(1).expect("still there").body,
    ORIGINAL,
    "the refusal fired AFTER mutating, so the prose is gone and the message says it is not"
  );
}

/// Whitespace-only is empty for this purpose, which is why the arm above passes
/// spaces and a newline rather than `""` -- a body of blanks erases exactly as
/// completely and would slip a check written against the empty string.
#[test]
fn the_emptiness_test_is_not_fooled_by_whitespace() {
  let fx = Fixture::new();
  with_an_issue(&fx);
  assert!(
    fx.facade().issue_edit(1, "\t \n").is_err(),
    "a body of blanks erases the prose as completely as an empty one"
  );
}

/// **RE-WRITING IDENTICAL BYTES REPORTS `already` AND WRITES NOTHING**, the same
/// discipline `set_issue_status` uses. Without it the event log shows a
/// correction that never happened, and a later reader cannot tell a real one
/// from a re-run.
#[test]
fn identical_prose_is_not_a_write() {
  let fx = Fixture::new();
  with_an_issue(&fx);

  let outcome = fx
    .facade()
    .issue_edit(1, ORIGINAL)
    .expect("re-writing the same bytes is not an error");

  assert!(
    matches!(outcome, intentsvcs::facade::Outcome::AlreadyThere { .. }),
    "identical bytes reported as a move, so the log now claims a correction nobody made: \
     {outcome:?}"
  );
}

#[test]
fn an_absent_issue_is_refused_by_number() {
  let fx = Fixture::new();
  with_an_issue(&fx);
  let said = fx
    .facade()
    .issue_edit(99, "prose for an issue that does not exist")
    .expect_err("there is no issue 99")
    .to_string();
  assert!(
    said.contains("99"),
    "the refusal must name the number asked for: {said}"
  );
}
