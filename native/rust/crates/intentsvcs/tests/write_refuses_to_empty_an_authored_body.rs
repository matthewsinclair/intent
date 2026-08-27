//! **A WRITE THAT REPLACES AUTHORED PROSE WITH NOTHING REFUSES AND SAYS SO.**
//!
//! Lamplight, 2026-08-26: `intent issues close 5` printed `ok: issue 0005 ->
//! CLOSED` and emptied 4934 bytes of the issue's body. Measured off git --
//! 5323 bytes before, 0 after -- and restored by hand.
//!
//! **NOTHING MALFUNCTIONED, WHICH IS THE WHOLE DIFFICULTY.** The in-memory
//! model held an empty body because the store had never ingested one; the write
//! wrote exactly what it was given; the verb did what it was asked. There was no
//! error anywhere to surface, no rescue swallowing anything, and no branch that
//! could have said "this looks wrong". **A silence with no error in it cannot be
//! fixed by handling errors better -- it needs something that COMPARES.**
//!
//! So the check compares the outgoing body against the FILE ABOUT TO BE
//! OVERWRITTEN, not against the model the write came from. Comparing to the
//! model can only ever agree with itself.
//!
//! **AND IT IS FIELD-LEVEL RATHER THAN BYTE-LEVEL, WHICH IS WHY IT CAN LIVE
//! BESIDE `refuse_if_this_would_empty_a_populated_face` INSTEAD OF DUPLICATING
//! IT.** That guard's byte-shrink arm is deliberately gated on
//! `canon.threads.is_empty()`, and its own comment says why: a file shrinking is
//! ordinary -- an edited-down objective, a removed work package -- and *a guard
//! that refuses the ordinary path is worse than the hole it closes, because it
//! gets disabled rather than fixed*. Emptying an authored body is not ordinary.
//! **Asking about the field distinguishes an author shortening prose from a
//! field being erased; asking about the size never can.**

mod common;

use common::Fixture;

const BODY: &str =
  "# The issue\n\nProse an author wrote, which no verb here was asked to remove.\n";

/// Put an authored body on disk that the open facade does not know about.
///
/// **This is the live sequence rather than a contrivance.** The body reached
/// canon by an edit -- there is no `issues add --body`, so on the estate this
/// is the only route in -- and the running process holds a model that predates
/// it. That gap is the condition; the verb is only what walks into it.
fn issue_with_prose_only_on_disk(fx: &Fixture) -> intentsvcs::facade::Facade {
  let mut f = fx.facade();
  let number = f.issue_add("A thing", None, None, "").expect("issue added");
  assert_eq!(number, 1, "the fixture's numbering");

  let path = fx.path("intent/.canon/issues/0001.json");
  let text = std::fs::read_to_string(&path).expect("canon written");
  let mut issue: serde_json::Value = serde_json::from_str(&text).expect("canon parses");
  issue["body"] = serde_json::Value::String(BODY.to_string());
  std::fs::write(
    &path,
    serde_json::to_string_pretty(&issue).expect("re-serialise"),
  )
  .expect("write the authored body");

  f
}

/// **THE CRITERION: the verb refuses, and the file still has its prose.**
#[test]
fn closing_an_issue_will_not_write_an_empty_body_over_an_authored_one() {
  let fx = Fixture::new();
  let mut f = issue_with_prose_only_on_disk(&fx);

  let err = f
    .issue_close(1)
    .expect_err("this must refuse rather than report ok over the loss");

  let said = err.to_string();
  assert!(
    said.contains("0001"),
    "the refusal names the subject: {said}"
  );
  assert!(
    said.contains(&BODY.len().to_string()),
    "and how much is at stake, so an operator can tell a stray newline from the \
     thing the issue is about: {said}"
  );

  let after = std::fs::read_to_string(fx.path("intent/.canon/issues/0001.json")).expect("canon");
  assert!(
    after.contains("Prose an author wrote"),
    "and the prose is STILL THERE -- a refusal that reports and writes anyway is \
     the defect with a message on top: {after}"
  );
}

/// **THE REMEDY POINTS AT THE ROUTE THAT ACTUALLY RECOVERS.**
///
/// The prose is not lost and the operator's next move is to get it into the
/// store, not to force the write. A remedy naming the wrong route is the false
/// remedy this estate has now filed five of.
#[test]
fn the_refusal_names_the_route_that_recovers_the_prose() {
  let fx = Fixture::new();
  let mut f = issue_with_prose_only_on_disk(&fx);
  let err = f.issue_close(1).expect_err("refused");

  let remedy = intentsvcs::remedy::Remedy::remedy(&err);
  assert!(
    remedy.contains("sync --to-store"),
    "the body has never reached the store, so taking it in is the fix: {remedy}"
  );
  assert!(
    !remedy.contains("--force"),
    "and no route that discards the thing being protected: {remedy}"
  );
}

/// **THE CONTROL, and without it every arm above passes for a verb that refuses
/// everything.**
///
/// An issue whose stored body and disk body agree closes normally. This is the
/// ordinary path, and the guard above must not touch it -- the gate on the
/// byte-shrink arm of the neighbouring guard exists because its first version
/// refused exactly this.
#[test]
fn an_issue_whose_body_is_empty_on_both_sides_closes_normally() {
  let fx = Fixture::new();
  let mut f = fx.facade();
  f.issue_add("A thing", None, None, "").expect("issue added");

  f.issue_close(1)
    .expect("an issue with no authored body anywhere is an ordinary close");
}

/// **AND THE OTHER HALF OF THE CONTROL: a body present on BOTH sides closes
/// normally too.**
///
/// The guard fires on the DISAGREEMENT, not on the presence of prose. Without
/// this arm a guard that refused any issue carrying a body would pass
/// everything above.
#[test]
fn an_issue_whose_body_the_store_already_holds_closes_normally() {
  let fx = Fixture::new();
  let mut f = issue_with_prose_only_on_disk(&fx);
  // Take the authored body in, which is what the refusal tells the operator to
  // do -- so this arm also proves the remedy WORKS rather than merely reads well.
  f.sync_from_disk(&intentsvcs::sync::Scope::All)
    .expect("the authored body is ingested");

  f.issue_close(1)
    .expect("once the store holds the prose, the write carries it and the verb proceeds");

  let after = std::fs::read_to_string(fx.path("intent/.canon/issues/0001.json")).expect("canon");
  assert!(
    after.contains("Prose an author wrote"),
    "and the prose survives the close it was blocking: {after}"
  );
}
