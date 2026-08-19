//! AT-08.5 / AC-08.5: **every writable field of every entity is settable
//! through the mutation surface, and a field that cannot be written is
//! reported BY NAME.**
//!
//! The criterion is TWO-SIDED, and dc caught the second side before it
//! ratified:
//!
//! 1. Every writable field is settable **intentionally**.
//! 2. **No verb silently clears a field it was not asked to change.** A
//!    checker asking merely _can every field be set_ passes on `note` while
//!    the verb empties it -- and the sharpest instance is the CLOSING verb,
//!    because it fires exactly when a row carries the most evidence. The rows
//!    nobody can afford to lose are the ones it hits hardest.
//!
//! # The row is RED and this test PASSES, and that is not a contradiction
//!
//! AC-08.5 makes the completeness of the surface the criterion "with the
//! unsettable set as the printed OUTPUT", and the surface is not complete. So
//! AT-08.5 belongs at red.
//!
//! **The test still passes, because that is how this estate carries an
//! unsatisfied criterion.** Checked rather than assumed: six red rows are
//! cargo tests -- `dep_graph_guard`, `mutation_completeness`,
//! `write_moves_only_what_changed`, `generated_views_are_not_formatted`,
//! `organize_five_rows`, `organize_attachment_divergence` -- all six exist and
//! the workspace is green, so not one red row corresponds to a failing cargo
//! test. **The ROW carries "does not pass"; the SUITE stays green.** A
//! deliberately-failing test here would be the first, and it would break the
//! shared suite for three other nodes and for the acceptance verifier.
//!
//! **So the gap is PINNED rather than merely printed**, which is the thing
//! that stops this being the vacuous green the estate keeps paying for. The
//! unsettable set is asserted EXACTLY. Close part of it and this fails, so
//! somebody has to move the row. Widen it and this fails too. A test that
//! merely printed the gap and passed would notice neither.
//!
//! # The roster is checked against the model, not trusted
//!
//! A hand-kept list of fields stops covering on the day someone adds one. So
//! [`writable_field_roster`] is compared to the JSON keys a fully-populated
//! entity actually serialises, and an unrostered field fails rather than being
//! silently omitted from the gap report. That is the same discipline
//! `openness.rs` uses to enumerate its tables from the DDL.

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::model::{AtStatus, Thread};
use serde_json::Value;

/// Every field of an AT row, and the verb that sets it -- `None` where no verb
/// does. Sourced from `facade.rs`'s public surface, read rather than recalled.
fn writable_field_roster() -> Vec<(&'static str, Option<&'static str>)> {
  vec![
    ("id", None),
    ("kind", None),
    ("file", None),
    ("prose", None),
    ("covers", None),
    ("status", Some("at_set")),
    ("note", None),
    ("legacy", None),
  ]
}

fn at_json(thread: &Thread, at: &str) -> Value {
  let row = thread
    .tests
    .iter()
    .find(|t| t.id == at)
    .unwrap_or_else(|| panic!("{at} is in the fixture"));
  serde_json::to_value(row).expect("an AT row serialises")
}

/// **Side 2, and it settles a claim recorded in the criterion itself.**
///
/// AC-08.5 records that "the only verbs that touch the row DESTROY it" and
/// that AT-10.11 was greened by hand-editing canon because `intent at green`
/// would have destroyed the note. **This test is the measurement rather than
/// the recollection**: it populates every field, moves the status, and diffs
/// the whole row.
#[test]
fn at_set_moves_status_and_touches_nothing_else() {
  let fx = Fixture::new();
  let thread = sample_thread("ST0001");
  fx.write_thread(&thread);
  let mut facade = fx.facade();

  let before_row = at_json(
    facade.canon().threads.first().expect("one thread"),
    "AT-03.1",
  );
  let before_note = before_row.get("note").cloned();
  assert!(
    before_note.as_ref().is_some_and(|n| !n.is_null()),
    "precondition: the fixture row must CARRY a note, or this test cannot see\n       \
     the field being cleared and passes vacuously -- got {before_note:?}"
  );
  let before_status = before_row.get("status").cloned();

  facade
    .at_set("ST0001", "AT-03.1", AtStatus::Red)
    .expect("the verb runs");

  let after_row = at_json(
    facade.canon().threads.first().expect("one thread"),
    "AT-03.1",
  );

  assert_ne!(
    after_row.get("status").cloned(),
    before_status,
    "precondition: the status must actually have MOVED, or a verb that did\n       \
     nothing would pass the diff below"
  );

  let mut unexpected: Vec<String> = Vec::new();
  for (key, before_value) in before_row.as_object().expect("an object") {
    if key == "status" {
      continue;
    }
    let after_value = after_row.get(key);
    if after_value != Some(before_value) {
      unexpected.push(format!("  {key}: {before_value:?} -> {after_value:?}"));
    }
  }
  // A field the verb ADDED is a change too, and the loop above only walks the
  // before-keys.
  for key in after_row.as_object().expect("an object").keys() {
    if before_row.get(key).is_none() {
      unexpected.push(format!("  {key}: absent -> {:?}", after_row.get(key)));
    }
  }

  assert!(
    unexpected.is_empty(),
    "`at_set` was asked to change `status` and changed these as well:\n{}\n\n  \
     A verb that clears a field as a side effect fails AC-08.5's second side, and\n  \
     the closing verb is the worst place for it: it fires precisely when a row\n  \
     carries the most evidence.",
    unexpected.join("\n")
  );
}

/// The roster must describe the model, not a memory of it. A field added to
/// `AcceptanceTest` with no roster row fails here rather than dropping quietly
/// out of the gap report below.
#[test]
fn the_roster_covers_every_field_the_model_serialises() {
  let thread = sample_thread("ST0001");
  let row = at_json(&thread, "AT-03.1");
  let actual: Vec<&str> = row
    .as_object()
    .expect("an object")
    .keys()
    .map(|s| s.as_str())
    .collect();

  let rostered: Vec<&str> = writable_field_roster().iter().map(|(f, _)| *f).collect();
  let missing: Vec<&&str> = actual.iter().filter(|f| !rostered.contains(f)).collect();
  assert!(
    missing.is_empty(),
    "these AT fields serialise and are not in the roster, so the gap report\n       \
     below cannot see them: {missing:?}"
  );
}

/// **Side 1: the unsettable set, pinned exactly.**
///
/// AC-08.5's criterion is an EMPTY set. It is not empty, so AT-08.5 is red.
/// What this asserts is that the gap is exactly what it is believed to be --
/// which fails if somebody closes part of it without moving the row, and fails
/// if it widens.
#[test]
fn the_unsettable_field_set_is_exactly_what_the_row_records() {
  let unsettable: Vec<&str> = writable_field_roster()
    .into_iter()
    .filter(|(field, verb)| verb.is_none() && !matches!(*field, "id" | "kind" | "legacy"))
    .map(|(field, _)| field)
    .collect();

  assert_eq!(
    unsettable,
    vec!["file", "prose", "covers", "note"],
    "the AT fields with no setter have CHANGED.\n\n  \
     If the set SHRANK, somebody closed part of AC-08.5 -- update this pin and\n  \
     move AT-08.5 toward green.\n  \
     If it GREW, a field was added with no verb and the surface got less\n  \
     complete.\n\n  \
     `note` is the burning one: it is the only place this contract records WHY a\n  \
     row is where it is, several notes are load-bearing paragraphs, and the only\n  \
     route today is a hand-edit of canon plus `sync --to-store`, racy against any\n  \
     peer syncing the other way. `file`, `prose` and `covers` are the rest of the\n  \
     row's authored content, so an AT row cannot be corrected once created."
  );
}

/// **Entity creation is a different axis from field completeness**, and a
/// surface can be field-complete while offering no way to bring the entity
/// into existence. AC-08.5's fourth instance.
///
/// # This test was WRONG when it shipped, and the way it was wrong is the
/// # lesson
///
/// The first version grepped `facade.rs` for `fn at_new`, `fn at_add`,
/// `fn at_create`, `fn ac_new`, `fn ac_add` and asserted none existed. It
/// PASSED -- while `Facade::put` in the same file created both, by
/// insert-if-absent, thirty lines away. **The pin measured a NAME and the
/// criterion is about a CAPABILITY**, so a capability arriving under an
/// unlisted name was invisible to it. Shipped in `53cb3f34`; caught when vc
/// asked whether the commit message's claim superseded the row.
///
/// That is the same defect cc found in the `organize` retirement -- a
/// ratification expressed as a string literal cannot tell a name being
/// reclaimed from a command being resurrected -- and it is the day's recurring
/// shape: an observable that agrees with two different worlds.
///
/// **So it is behavioural now.** It creates rows that do not exist and asserts
/// they land. A creator arriving under any name at all satisfies it, and no
/// renaming can make it lie.
#[test]
fn an_ac_and_an_at_can_be_created_through_the_surface() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();

  let before = facade.canon().threads[0].clone();
  assert!(
    !before.tests.iter().any(|t| t.id == "AT-09.9"),
    "precondition: the AT must be absent or this tests an update"
  );
  assert!(
    !before.criteria.iter().any(|c| c.id == "AC-09.9"),
    "precondition: the AC must be absent"
  );

  facade
    .put(
      &intentsvcs::address::parse("intent:///threads/ST0001/at/AT-09.9").expect("resolves"),
      r#"{"id":"AT-09.9","kind":"test","file":"native/rust/crates/intentsvcs/tests/n.rs",
         "covers":["AC-09.9"],"status":"to-write","note":"created, not transitioned"}"#,
    )
    .expect("an AT is creatable through the address surface");

  let landed = facade.canon().threads[0]
    .tests
    .iter()
    .find(|t| t.id == "AT-09.9")
    .expect("the AT exists now");
  assert_eq!(
    landed.note.as_deref(),
    Some("created, not transitioned"),
    "and it carries a field no transitioning verb could have set"
  );

  // The AC half. Asserted separately because a surface can create one and not
  // the other, and AC-08.5 names both.
  let ac_body = serde_json::to_string(
    facade.canon().threads[0]
      .criteria
      .first()
      .expect("the fixture carries a criterion"),
  )
  .expect("serialises");
  let mut ac: serde_json::Value = serde_json::from_str(&ac_body).expect("parses");
  ac["id"] = serde_json::json!("AC-09.9");
  facade
    .put(
      &intentsvcs::address::parse("intent:///threads/ST0001/ac/AC-09.9").expect("resolves"),
      &serde_json::to_string(&ac).expect("serialises"),
    )
    .expect("an AC is creatable through the address surface");

  assert!(
    facade.canon().threads[0]
      .criteria
      .iter()
      .any(|c| c.id == "AC-09.9"),
    "the AC exists now -- AC-08.5's fourth instance is superseded for the SERVICE
            surface. It remains true at the CLI, which has no create verb wired."
  );
}
