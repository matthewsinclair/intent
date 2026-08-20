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
//! # WHAT THIS FILE MEASURED WRONG, AND HOW LONG IT PASSED WHILE DOING IT
//!
//! Until 2026-08-20 the unsettable set was a hand-written literal asserted
//! equal to a SECOND hand-written literal, and **two literals compared to each
//! other observe nothing.** No setter arriving for any field in it could have
//! moved that test; only a human editing both halves could.
//!
//! It named `file`, `prose`, `covers` and `note`. **Driven the same day,
//! `Facade::put` set all four -- one call, `Outcome::Moved`, values read back
//! changed.**
//!
//! And it was measuring the wrong SUBJECT. The criterion says *settable
//! through the MUTATION SURFACE*; the roster was of named VERBS, and `put` is
//! on the surface. **Those are two findings and they are now two lists.**
//! [`no_named_verb_sets`] records that no CLI verb spells these fields, which
//! is true and is a statement about the CLI; the unsettable set is measured by
//! driving the surface.
//!
//! **This is the identical defect the create pin had, and that one is
//! explained forty lines below in this same file** -- it measured a NAME while
//! `put` created both rows thirty lines away in `facade.rs`. The fix was
//! applied to one of the two.
//!
//! # THE MEASURED SET IS EMPTY, AND THE DENOMINATOR IS WHAT STOPS THAT READING
//! # AS SATISFACTION
//!
//! Over an acceptance-test row every field lands. **That is ONE entity through
//! ONE door**, and AC-08.5 says *every writable field of EVERY entity*. The
//! criterion's own burning cases are not AT-row fields at all: **ST0011's
//! `completed` is a THREAD field, an attachment's canon record has no setter
//! narrower than a thread, and no CLI verb creates an AC or an AT.** None of
//! those three is touched here and none is refuted by this file's empty set.
//!
//! **So an empty gap here is not the criterion met.** It is one population
//! measured and the rest unmeasured, and that distinction is stated in the
//! assertion itself rather than left for a reader to reconstruct -- an empty
//! gap over an unstated denominator is the vacuous green this estate keeps
//! paying for.
//!
//! # The probe is checked against the model, not trusted
//!
//! A hand-kept list of fields stops covering on the day someone adds one, so
//! the probe's field names are compared to the JSON keys a fully-populated
//! entity actually serialises, and an unprobed field fails rather than being
//! silently omitted from the gap report. That is the same discipline
//! `openness.rs` uses to enumerate its tables from the DDL.
//!
//! **And the row it measures against is SYNTHESISED.** `sample_thread`'s
//! `AT-03.1` carries six of the eight fields -- `prose` and `legacy` are
//! `None` and `skip_serializing_if` drops them from the JSON -- so a
//! measurement taken against the live fixture is blind to exactly the two
//! fields nobody has ever set.

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::address::parse;
use intentsvcs::model::{AcceptanceTest, AtKind, AtStatus, Legacy, Thread};
use serde_json::{Value, json};

/// Every field of an AT row, and the NAMED VERB that sets it -- `None` where no
/// named verb does.
///
/// **THIS IS A ROSTER OF VERBS AND IT IS NO LONGER READ AS A ROSTER OF THE
/// SURFACE.** Those are different subjects and the file used to conflate them:
/// AC-08.5 says *settable through the MUTATION SURFACE*, and `Facade::put` is
/// on that surface and writes the whole row. **Measured 2026-08-20 by driving
/// it**: a `put` to `intent:///threads/ST0001/at/AT-03.1` changed `note`,
/// `file` and `covers` in one call, returning `Outcome::Moved`.
///
/// So the gap this list describes is *no verb spells this field* -- which is a
/// real and separate finding about the CLI -- and the unsettable set is
/// measured next door by driving the surface instead.
fn no_named_verb_sets() -> Vec<(&'static str, Option<&'static str>)> {
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

/// The row every field-completeness measurement runs against -- **every field
/// present, so nothing is measured against a `None` that serde dropped.**
///
/// **SYNTHESISED, NOT BORROWED.** `sample_thread`'s `AT-03.1` carries six of
/// the eight fields: `prose` and `legacy` are `None` and `skip_serializing_if`
/// removes them from the JSON entirely, so a measurement taken against that row
/// is blind to exactly the two fields nobody has ever set. **An instrument that
/// borrows a live instance has made the estate's current shape part of its own
/// denominator** (cc's ruling), and the estate is then not free to change it.
fn fully_populated_row() -> AcceptanceTest {
  AcceptanceTest {
    id: "AT-03.1".to_string(),
    kind: AtKind::Test,
    file: Some("crates/intentsvcs/tests/ingest_refusal.rs".to_string()),
    prose: Some("what was read, on a row that also cites a file".to_string()),
    covers: vec!["AC-03.1".to_string()],
    status: AtStatus::Green,
    note: Some("the note this criterion keeps calling the burning case".to_string()),
    legacy: Some(Legacy {
      raw: "AT-03.1 -- carried from a v2 estate".to_string(),
    }),
  }
}

/// A different, LEGAL value for every field of an AT row except its id.
///
/// **Hand-written per field because "a different value" is type-specific.**
/// `kind` and `status` are enums and `legacy` is a struct; a generic nudge
/// produces bytes that will not deserialise, and a field would then be reported
/// UNSETTABLE when the probe was what was wrong. **The NAMES are checked
/// against what the model serialises**, so a field added to `AcceptanceTest`
/// fails that check rather than dropping quietly out of the measurement.
///
/// `id` is absent and that is not an omission: **the id IS the address**, and a
/// `put` whose body renamed the row is addressing a different entity -- which
/// `put` refuses by name. Measuring it here would record the refusal as a gap.
fn a_different_legal_value() -> Vec<(&'static str, Value)> {
  vec![
    ("kind", json!("non-test")),
    ("file", json!("crates/intentsvcs/tests/moved.rs")),
    ("prose", json!("re-read, and this is what it said")),
    ("covers", json!(["AC-09.9"])),
    ("status", json!("red")),
    ("note", json!("set through the mutation surface")),
    ("legacy", json!({ "raw": "a different v2 reference" })),
  ]
}

/// A fixture whose `AT-03.1` is [`fully_populated_row`].
fn populated_fixture() -> Fixture {
  let fx = Fixture::new();
  let mut thread = sample_thread("ST0001");
  let row = thread
    .tests
    .iter_mut()
    .find(|t| t.id == "AT-03.1")
    .expect("the fixture carries AT-03.1");
  *row = fully_populated_row();
  fx.write_thread(&thread);
  fx
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

/// **BOTH LISTS MUST DESCRIBE THE MODEL, NOT A MEMORY OF IT** -- and the row
/// they are checked against is the FULLY POPULATED one.
///
/// The old version compared the roster to `sample_thread`'s `AT-03.1`, which
/// carries six of the eight fields. `prose` and `legacy` are `None` there and
/// `skip_serializing_if` removes them from the JSON, **so the check could not
/// see the two fields it most needed to** -- a new `Option` field would have
/// been invisible to it in exactly the same way.
#[test]
fn both_lists_cover_every_field_the_model_serialises() {
  let row = serde_json::to_value(fully_populated_row()).expect("an AT row serialises");
  let actual: Vec<&str> = row
    .as_object()
    .expect("an object")
    .keys()
    .map(|s| s.as_str())
    .collect();
  assert_eq!(
    actual.len(),
    8,
    "precondition: the row must serialise EVERY field, or both checks below \n       \
     are measuring a serde skip: {actual:?}"
  );

  let rostered: Vec<&str> = no_named_verb_sets().iter().map(|(f, _)| *f).collect();
  let unrostered: Vec<&&str> = actual.iter().filter(|f| !rostered.contains(f)).collect();
  assert!(
    unrostered.is_empty(),
    "these AT fields serialise and no_named_verb_sets does not list them: {unrostered:?}"
  );

  // `id` is deliberately absent from the probe -- it IS the address, and a
  // `put` whose body renamed the row addresses a different entity.
  let probed: Vec<&str> = a_different_legal_value().iter().map(|(f, _)| *f).collect();
  let unprobed: Vec<&&str> = actual
    .iter()
    .filter(|f| **f != "id" && !probed.contains(f))
    .collect();
  assert!(
    unprobed.is_empty(),
    "these AT fields serialise and the surface measurement never tries to set \n       \
     them, so they cannot appear in its gap report: {unprobed:?}"
  );
}

/// **SIDE 1: THE UNSETTABLE SET, MEASURED BY DRIVING THE SURFACE.**
///
/// # What this replaced, and why it could never have gone red
///
/// The previous test built a list of fields from a hand-written literal and
/// asserted it equalled a second hand-written literal. **Two literals compared
/// to each other cannot observe the estate at all**: a setter arriving for
/// every field in the list would not have moved it, and the only thing that
/// could was a human editing both halves.
///
/// It was also measuring the wrong subject. AC-08.5 says *settable through the
/// MUTATION SURFACE*; the roster was of named VERBS. **`Facade::put` is on the
/// surface and writes the whole row** -- driven 2026-08-20, one call changed
/// `note`, `file` and `covers` together and returned `Outcome::Moved` -- so
/// three of the four fields the pin called unsettable were settable while it
/// passed.
///
/// **That is the identical defect this file's own next docstring explains**:
/// the create pin measured a NAME while `put` created both rows thirty lines
/// away. The fix was applied to one of the two.
#[test]
fn the_unsettable_field_set_is_measured_by_driving_the_surface() {
  let mut unsettable: Vec<String> = Vec::new();

  for (field, value) in a_different_legal_value() {
    let fx = populated_fixture();
    let mut facade = fx.facade();

    let before = at_json(&facade.canon().threads[0], "AT-03.1");
    assert!(
      before.get(field).is_some(),
      "precondition: `{field}` must be PRESENT on the row, or this measures a \n       \
       serde skip rather than the surface"
    );
    assert_ne!(
      before.get(field),
      Some(&value),
      "precondition: the probe value for `{field}` must DIFFER from what is \n       \
       there, or a surface that did nothing would read as success"
    );

    let mut body = before.clone();
    body[field] = value.clone();
    let address = parse("intent:///threads/ST0001/at/AT-03.1").expect("the row has an address");

    match facade.put(&address, &body.to_string()) {
      Ok(_) => {
        let after = at_json(&facade.canon().threads[0], "AT-03.1");
        if after.get(field) != Some(&value) {
          unsettable.push(format!("{field} (accepted, did not land)"));
        }
      }
      Err(why) => unsettable.push(format!("{field} (refused: {why})")),
    }
  }

  assert_eq!(
    unsettable,
    Vec::<String>::new(),
    "these fields of an ACCEPTANCE TEST cannot be set through the mutation \
     surface.\n\n  \
     An empty list here is NOT AC-08.5 met: it is one entity measured through \
     one door.\n  \
     The criterion's own burning cases are elsewhere -- ST0011's `completed` \
     is a THREAD\n  \
     field, an attachment's canon record has no setter narrower than a thread, \
     and no\n  \
     CLI verb creates an AC or an AT at all."
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
