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
use std::collections::BTreeSet;

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

// ---------------------------------------------------------------------------
// AC-08.5's population, widened from ONE entity to EVERY entity form.
// ---------------------------------------------------------------------------

/// What the mutation surface did when this form was addressed.
///
/// **The discriminator is `has no write path yet`, and it is the right one
/// because it is the surface's own words for *this entity is not reachable at
/// all*.** A body error, an id mismatch or a named refusal all prove the
/// opposite -- that `put` reached the form and had an opinion about the
/// request. Only this answer means the door is absent.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Reached {
  /// `put` has an arm for this form. What it then said about the BODY is a
  /// different question and deliberately not this test's.
  Yes,
  /// `put` fell through to its catch-all: no arm exists.
  NoWritePathYet,
}

/// What we DECLARE about a form, which is a different question from what the
/// surface DID and is therefore a different type.
///
/// **`Reached` is OBSERVED and has two values because the surface can only do
/// two things. `Expected` is AUTHORED and has three, because `no write path`
/// was carrying two meanings that nothing could tell apart** -- *not built
/// yet*, and *never, by a ruling made elsewhere*. Both rendered as
/// `NoWritePathYet`, so the criterion's denominator silently included forms
/// the estate has separately ruled must NEVER be writable.
///
/// # Why that mattered rather than being untidy
///
/// `Event` and `NodeInbox` are append-only by ruling, with a SHIPPED GUARD
/// behind it: `append-only-guard.sh` declares exactly two subjects,
/// `intent/whiteboard/*/.history/**` and the event log, and exists to refuse a
/// truncating write where an append was meant -- written after 492 lines of
/// `.history/` were destroyed on 2026-08-17 and 19 events on 2026-08-19.
///
/// **So while both meanings shared one value, AC-08.5's only routes were to
/// build write paths a shipped guard exists to refuse, or to stay red
/// forever.** That is a population defect and not a criterion defect: those
/// forms are not writable entities, so they were never in this criterion's
/// population. Naming a permanent exclusion WITH ITS REASON is a stronger
/// record than an absent row.
///
/// # THIS SPLIT IS GUARDED IN ONE DIRECTION ONLY, AND SAYING SO IS THE POINT
///
/// **Mutation-verified 2026-08-22, three planted cases.** Declaring a form
/// reachable when it has no arm fails (`declared Reachable, which requires
/// Yes, but observed NoWritePathYet`). Declaring a permanent exclusion with an
/// empty citation fails. **But DEMOTING a permanent exclusion back into the
/// worklist PASSES SILENTLY** -- population 11 -> 12, exclusions 2 -> 1, green.
///
/// That asymmetry is correct and is not a hole to be plugged. The dangerous
/// direction is retiring a form that SHOULD be reachable, because it shrinks
/// the criterion's denominator and reads as a decision somebody made -- and
/// that direction is guarded by the citation clause. The other direction only
/// ever ADDS work to AC-08.5, so an error there is loud by construction: the
/// worklist grows and somebody has to build an arm nobody wanted.
///
/// **What this test therefore does NOT do: it cannot tell you a form SHOULD be
/// excluded.** It can only refuse an exclusion that does not say why. Whether
/// `Event` and `NodeInbox` genuinely belong outside the population is a
/// RULING, and the citations below are where a reader goes to challenge it.
/// An instrument that declares its own blind spot is the argument for the
/// discipline around it; one that implies it has none is the vacuous green.
///
/// # The citation is mandatory, and that clause is inherited rather than minted
///
/// hv ruled exactly this shape for the runner roster's `not-an-instrument`, on
/// cc's grounds: **a bare exclusion costs nothing to write, so a genuine member
/// can be declared out of scope by whoever finds the check inconvenient -- and
/// the check goes blind again WITH A SIGNATURE ON IT, which is worse than a
/// gap, because a declaration reads as a decision someone made.** Same shape,
/// same clause. An empty citation fails below.
#[derive(Debug, PartialEq, Eq)]
enum Expected {
  /// `put` has an arm today.
  Reachable,
  /// No arm yet, and there SHOULD be one. **This is AC-08.5's worklist.**
  NotBuiltYet,
  /// No arm, and there must never be one. Carries the ruling that says so.
  NeverByRuling(&'static str),
}

impl Expected {
  /// What the surface must do for this declaration to hold.
  fn requires(&self) -> Reached {
    match self {
      Expected::Reachable => Reached::Yes,
      Expected::NotBuiltYet | Expected::NeverByRuling(_) => Reached::NoWritePathYet,
    }
  }

  /// **Is this form in AC-08.5's denominator?** Permanent exclusions are not:
  /// the criterion is about the forms that SHOULD be reachable.
  fn in_population(&self) -> bool {
    !matches!(self, Expected::NeverByRuling(_))
  }
}

/// **EXHAUSTIVE ON PURPOSE: a fourteenth `Entity` variant does not compile
/// until it is named here**, which is the only mechanism Rust offers for
/// "every variant is accounted for" -- the language cannot enumerate them.
/// The same device carries AT-07.7's collection fence.
///
/// This is NOT a second copy of `Entity::form()`. That answers *what is this
/// called*; this answers *is this form expected to be reachable by `put`*, and
/// nothing else in the tree holds that.
fn declared_reach(entity: &intentsvcs::address::Entity) -> Expected {
  use intentsvcs::address::Entity as E;
  match entity {
    // Arms that exist in `Facade::put` today.
    E::At { .. } | E::Ac { .. } | E::Thread { .. } => Expected::Reachable,
    // Reached, and refused BY NAME with `this id is server-assigned`. That is
    // an opinion about the request, so the door exists.
    E::Threads | E::Issue { .. } => Expected::Reachable,

    // **AC-08.5's LIVE WORKLIST, and the reason this row is not green.**
    // An attachment's canon record has no setter narrower than a thread; the
    // rest have no `put` arm at all. Every one of these SHOULD have one.
    E::Attachment { .. } => Expected::Reachable,
    E::Wp { .. } | E::WpCollection { .. } | E::AcCollection { .. } => Expected::NotBuiltYet,
    E::Issues => Expected::NotBuiltYet,
    E::Node { .. } => Expected::NotBuiltYet,

    // **PERMANENT EXCLUSIONS, EACH CITING THE RULING THAT EXCLUDES IT.**
    // These are not writable entities, so they were never in this criterion's
    // population. Before the split they sat in the worklist, which made
    // AC-08.5 satisfiable only by building what a shipped guard refuses.
    E::NodeInbox { .. } => Expected::NeverByRuling(
      "append-only by the whiteboard protocol's single-writer inbox rule, guarded by \
       `append-only-guard.sh`, whose declared subject `intent/whiteboard/*/.history/**` \
       exists because 492 lines of one node's history were destroyed 2026-08-17. A `put` \
       arm here is a truncating write over an append-only surface: the exact act the \
       guard was written to refuse.",
    ),
    E::Event { .. } => Expected::NeverByRuling(
      "the event log is append-only -- history is not edited. hv ruling D53, 2026-08-20 \
       retired the tracked `intent/events.jsonl` and moved the log into the store, and \
       `append-only-guard.sh` names the log as its second declared subject after 19 \
       events were destroyed 2026-08-19. AC-09.3 covers reading it; nothing covers \
       rewriting it, because nothing should.",
    ),
  }
}

/// One address of every entity form D57-8 defines.
///
/// Spellings taken from the address grammar's own tests rather than composed
/// here, so a form this list gets WRONG fails to parse and is reported as a
/// broken population instead of silently measuring twelve forms as thirteen.
fn one_address_of_every_form() -> Vec<&'static str> {
  vec![
    "intent:///threads",
    "intent:///issues",
    "intent:///threads/ST0001/wp",
    "intent:///threads/ST0001/ac",
    "intent:///threads/ST0001",
    "intent:///threads/ST0001/wp/01",
    "intent:///threads/ST0001/ac/AC-09.9",
    "intent:///threads/ST0001/at/AT-03.1",
    "intent:///threads/ST0001/attachments/design.md",
    "intent:///issues/0001",
    "intent:///nodes/ic",
    "intent:///nodes/ic/inbox/vc/2026-08-19T11:41Z",
    "intent:///events/1234",
  ]
}

/// **AC-08.5's UNSETTABLE SET, DRIVEN ACROSS EVERY ENTITY FORM AND PRINTED BY
/// NAME.**
///
/// The criterion's own words: *the completeness of the surface, with the
/// unsettable set as the printed output*. `the_unsettable_field_set_is_measured_by_driving_the_surface`
/// above measures ONE entity through ONE door and says so in its own failure
/// text -- *an empty list here is NOT AC-08.5 met*. This takes the population
/// the criterion actually names.
///
/// # Membership is DRIVEN, and the declaration is only the expectation
///
/// Every form is addressed and `put` is CALLED. What lands in the set is what
/// the surface did. `declared_reach` says what we believe, and the two are
/// compared -- so this reds in BOTH directions: **a form that loses its write
/// path joins the set, and a form that GAINS one leaves it and forces the
/// declaration to shrink.** An authored list of known gaps can only ever rot in
/// the second direction, silently, which is the shape that turns a criterion
/// into an excuse-list.
///
/// # Why this cannot go green by being written more carefully
///
/// It is not a defect in the test that the set is non-empty. **Red IS the
/// criterion's verdict** until the surface reaches those forms, and the value
/// of the test is that the set is exact and named rather than gestured at.
#[test]
fn the_unsettable_set_is_driven_across_every_entity_form_and_named() {
  let fx = populated_fixture();
  let mut facade = fx.facade();

  let mut observed: Vec<(String, Reached, Expected)> = Vec::new();
  let mut disagreements: Vec<String> = Vec::new();

  for url in one_address_of_every_form() {
    let address = parse(url).unwrap_or_else(|e| {
      panic!("the POPULATION is broken, not the surface: `{url}` does not parse ({e:?})")
    });
    // **The body is deliberately minimal.** This test asks whether `put` has an
    // ARM for the form, never whether a particular write lands -- so a body
    // that would fail parsing inside an arm still proves the arm was reached,
    // because a parse complaint is an opinion about the request.
    let said = match facade.put(&address, "{}") {
      Ok(_) => String::new(),
      Err(why) => format!("{why}"),
    };
    let reached = if said.contains("has no write path yet") {
      Reached::NoWritePathYet
    } else {
      Reached::Yes
    };
    let declared = declared_reach(&address.entity);

    // **THE CITATION CLAUSE, ENFORCED RATHER THAN TRUSTED.** A permanent
    // exclusion carrying no usable reason is precisely cc's roster finding: it
    // costs nothing to write, so a form that SHOULD be reachable can be
    // declared out of the population by whoever finds this test inconvenient,
    // and the criterion goes blind with a signature on it. The length floor is
    // crude on purpose -- it cannot judge a reason, only refuse an absent one,
    // and a check that cannot be satisfied by a shrug is worth more here than
    // one that pretends to grade prose.
    if let Expected::NeverByRuling(why) = &declared {
      assert!(
        why.trim().len() > 40,
        "{url} is declared NeverByRuling with no usable citation.\n  \
         A permanent exclusion MUST name the ruling that excludes it. A bare one costs \
         nothing to write, so it can retire a form that should be reachable and leave a \
         DECLARATION where a gap used to be -- which reads as a decision somebody made \
         and is worse than the gap. hv ruled this clause for the runner roster's \
         `not-an-instrument`; it is inherited here, not minted."
      );
    }

    if reached != declared.requires() {
      disagreements.push(format!(
        "  {url}\n    declared {declared:?}, which requires {:?}, but observed {reached:?}",
        declared.requires()
      ));
    }
    observed.push((url.to_string(), reached, declared));
  }

  // **THE POSITIVE CONTROL: at least one form must be reachable.** Every
  // assertion here is about a refusal, so a facade that refused everything --
  // a broken fixture, an unopened canon -- would satisfy the whole list and
  // report a green that is a fact about the harness rather than the surface.
  let reachable = observed
    .iter()
    .filter(|(_, r, _)| *r == Reached::Yes)
    .count();
  assert!(
    reachable > 0,
    "no form was reachable at all -- this is a fact about the harness, not about \
     the mutation surface"
  );

  assert!(
    disagreements.is_empty(),
    "the mutation surface's reach has MOVED, and the declaration no longer describes it:\n{}\n\n  \
     If a form GAINED a write path, delete it from `declared_reach` -- AC-08.5 got closer.\n  \
     If a form LOST one, that is a regression in the surface, not in this test.",
    disagreements.join("\n")
  );

  // **The unsettable set, printed by name on every run.** AC-08.5 asks for it
  // as OUTPUT, not merely as an assertion, and a set nobody prints is one
  // nobody can act on.
  // **THE DENOMINATOR IS THE FORMS THAT SHOULD BE REACHABLE**, and the
  // permanent exclusions are printed separately WITH their rulings rather than
  // omitted. An exclusion nobody prints is one nobody can challenge, and this
  // list is exactly where a wrong exclusion would hide.
  let population: Vec<&(String, Reached, Expected)> = observed
    .iter()
    .filter(|(_, _, e)| e.in_population())
    .collect();
  let worklist: Vec<&str> = population
    .iter()
    .filter(|(_, r, _)| *r == Reached::NoWritePathYet)
    .map(|(u, _, _)| u.as_str())
    .collect();

  println!(
    "AC-08.5: {} of {} entity form(s) IN THE POPULATION have no write path through `put`:",
    worklist.len(),
    population.len()
  );
  for u in &worklist {
    println!("  {u}");
  }

  let excluded: Vec<(&str, &str)> = observed
    .iter()
    .filter_map(|(u, _, e)| match e {
      Expected::NeverByRuling(why) => Some((u.as_str(), *why)),
      _ => None,
    })
    .collect();
  println!(
    "AC-08.5: {} form(s) are EXCLUDED FROM THE POPULATION BY RULING, not pending:",
    excluded.len()
  );
  for (u, why) in &excluded {
    println!("  {u}\n    {why}");
  }
}

/// A `Thread` with every field that is neither REQUIRED nor GRAFTED carrying a
/// distinctive non-default value.
///
/// **This is [`fully_populated_row`]'s argument carried one entity up, and the
/// gap it closes is that nobody had carried it.** That helper exists because
/// `skip_serializing_if` drops `None` fields from the JSON entirely, so a
/// measurement taken against a partially-populated row is blind to exactly the
/// fields nobody has ever set. `sample_thread` says the same thing in its own
/// words -- *a new field the fixture leaves `None` round-trips vacuously* --
/// and then leaves `completed`, `acceptance`, `body` and `preamble` at their
/// defaults, which are four of the eight fields this test is about.
fn fully_populated_thread(id: &str) -> Thread {
  let mut thread = sample_thread(id);
  thread.slug = Some("intent-v3".to_string());
  thread.status_reason = Some("reopened: AC-02.6 was added after the close".to_string());
  thread.completed = Some("2026-08-20".to_string());
  thread.acceptance = Some(intentsvcs::model::AcceptanceMode::Exempt);
  thread.objective = "Ship v3.0.0 with the store as the durable SSOT.".to_string();
  thread.context = "Why this thread exists, in markdown, carried verbatim.".to_string();
  thread.body = "A load-bearing paragraph nothing else records.".to_string();
  thread.preamble = "Front matter prose.".to_string();
  thread.related = vec![intentsvcs::model::Related {
    id: "ST0056".to_string(),
    note: Some("the rewrite this thread gates".to_string()),
  }];
  thread
}

/// **THE THREAD FIXTURE'S COMPLETENESS, ASSERTED RATHER THAN NAMED.**
///
/// [`thread_put_clears_the_fields_it_was_not_asked_to_change`] derives its whole
/// population from a PROSE COMMENT -- *eighteen fields: five required, four
/// children, nine remaining* -- and asserts none of it. **A nineteenth `Thread`
/// field leaves that comment stale and the collateral list silently measuring
/// eight of ten.** That is the identical `skip_serializing_if` blindness
/// [`both_lists_cover_every_field_the_model_serialises`] documents and guards
/// for the AT row, thirty lines above, and `fully_populated_thread` is fully
/// populated only BY ITS NAME.
///
/// **This is the third time this file has had to record _the fix was applied to
/// one of the two_** -- after the create pin, and after the unsettable roster.
///
/// # It asserts a PARTITION, not a count, because a count cannot name what moved
///
/// The three roles the thread test plays -- schema-required, grafted child,
/// collateral -- must cover exactly what the fixture SERIALISES, in both
/// directions. A new field is unclassified until someone places it, and the
/// failure prints its name; a field that stops serialising is caught by the
/// second half even though the count would be equally wrong. **`len() == 18`
/// reds on the same changes and says only that a number moved.**
#[test]
fn the_thread_fixture_serialises_every_field_and_the_three_roles_partition_it() {
  let json = serde_json::to_value(fully_populated_thread("ST0001")).expect("a thread serialises");
  let serialised: BTreeSet<&str> = json
    .as_object()
    .expect("an object")
    .keys()
    .map(|s| s.as_str())
    .collect();

  // The three roles, spelled exactly as the thread test uses them. `completed`
  // is the field that test ASKS to move, so it is neither required nor
  // collateral -- it is its own role, and naming it here is what stops it being
  // quietly counted as one of the eight.
  const REQUIRED: [&str; 5] = ["schema", "id", "title", "status", "created"];
  const GRAFTED_CHILDREN: [&str; 4] = ["wps", "criteria", "tests", "attachments"];
  const ASKED_FOR: [&str; 1] = ["completed"];
  const COLLATERAL: [&str; 8] = [
    "slug",
    "status_reason",
    "acceptance",
    "objective",
    "context",
    "body",
    "preamble",
    "related",
  ];

  let classified: BTreeSet<&str> = REQUIRED
    .iter()
    .chain(GRAFTED_CHILDREN.iter())
    .chain(ASKED_FOR.iter())
    .chain(COLLATERAL.iter())
    .copied()
    .collect();

  let unclassified: Vec<&&str> = serialised
    .iter()
    .filter(|f| !classified.contains(*f))
    .collect();
  assert!(
    unclassified.is_empty(),
    "these `Thread` fields serialise and no role in the thread test accounts for \n       \
     them, so a `put` could clear them and nothing would report it: {unclassified:?}"
  );

  let unserialised: Vec<&&str> = classified
    .iter()
    .filter(|f| !serialised.contains(*f))
    .collect();
  assert!(
    unserialised.is_empty(),
    "the fixture is called `fully_populated_thread` and these fields do NOT \n       \
     serialise, so every assertion about them compares a default to a default: \n       \
     {unserialised:?}"
  );
}

/// **AC-08.5's SECOND LIMB, DRIVEN AT THE THREAD DOOR: does `put` clear a field
/// it was not asked to change?**
///
/// # Why this had to be written, and why the existing pair could not see it
///
/// AC-08.5 has TWO AXES and the file covered them asymmetrically.
/// `the_unsettable_set_is_driven_across_every_entity_form_and_named` drives all
/// thirteen FORMS and asks only *does an arm exist*;
/// `the_unsettable_field_set_is_measured_by_driving_the_surface` drives FIELDS
/// for exactly one entity, the AT row. **`E::Thread => Reachable` proves the
/// form has an arm and says nothing about which of the thread's own fields that
/// arm writes** -- so a refutation of *`ST0011.completed` has no write path*
/// can be entirely correct and leave this criterion unmet.
///
/// # The population is stated, because a collateral set over an unstated
/// denominator is the vacuous green this row keeps paying for
///
/// `Thread` has eighteen fields. FIVE are required by the schema, so a legal
/// body always carries them. FOUR are children the arm grafts off the stored
/// row on purpose, with a comment naming this very limb. **The remaining NINE
/// are neither, and this test asks for exactly one of them -- `completed`, the
/// criterion's own first burning case -- so the collateral population is the
/// other EIGHT.**
///
/// # The subject is the SURFACE, never the estate's data
///
/// The fixture is SYNTHESISED. ST0011's real `completed` was repaired by hand
/// at `608e9721` on 2026-08-20, the day BEFORE the ruling that created this
/// write path, so reading the live estate answers a question about a hand-edit
/// and reports it as a fact about the surface. **When a measurement is correct
/// and its SUBJECT is wrong, nothing inside the measurement can report it.**
#[test]
fn thread_put_clears_the_fields_it_was_not_asked_to_change() {
  let fx = Fixture::new();
  let before = fully_populated_thread("ST0001");
  fx.write_thread(&before);
  let mut facade = fx.facade();

  // **THE MINIMAL LEGAL BODY.** The five schema-required fields, plus the one
  // field we are asking to move. The four children are omitted because the arm
  // refuses them BY NAME -- naming them here would measure that refusal
  // instead, which is a different and already-covered question.
  let body = json!({
    "schema": before.schema,
    "id": before.id,
    "title": before.title,
    "status": before.status,
    "created": before.created,
    "completed": "2026-08-24",
  });
  let address = parse("intent:///threads/ST0001").expect("the address parses");
  let outcome = facade
    .put(&address, &body.to_string())
    .expect("the thread arm accepts a legal body for an existing thread");

  let after = facade
    .canon()
    .threads
    .iter()
    .find(|t| t.id == "ST0001")
    .cloned()
    .expect("the thread survives the write");

  // **POSITIVE CONTROL: the field we ASKED for must have moved.** Without it a
  // facade that refused the write outright would leave every other field
  // untouched and satisfy every assertion below -- a green that is a fact about
  // the harness rather than about the surface.
  assert_eq!(
    after.completed.as_deref(),
    Some("2026-08-24"),
    "the field this test asked to move did not move -- the measurement below is \
     about a write that never happened ({outcome:?})"
  );

  // The eight fields nobody asked about, each compared to what it was.
  let collateral: Vec<&str> = [
    ("slug", before.slug != after.slug),
    ("status_reason", before.status_reason != after.status_reason),
    ("acceptance", before.acceptance != after.acceptance),
    ("objective", before.objective != after.objective),
    ("context", before.context != after.context),
    ("body", before.body != after.body),
    ("preamble", before.preamble != after.preamble),
    ("related", before.related != after.related),
  ]
  .into_iter()
  .filter_map(|(name, moved)| moved.then_some(name))
  .collect();

  // **GRAFTED CHILDREN ARE THE CONTROL THAT SAYS THE ARM CAN PROTECT A FIELD.**
  // If these moved too, the finding would be "the arm replaces the document",
  // which is a weaker and less interesting claim. They do not move, which is
  // what makes the eight above a CHOICE about which fields get grafted rather
  // than an inevitability of parse-and-replace.
  assert_eq!(
    (
      before.wps.len(),
      before.criteria.len(),
      before.tests.len(),
      before.attachments.len()
    ),
    (
      after.wps.len(),
      after.criteria.len(),
      after.tests.len(),
      after.attachments.len()
    ),
    "the grafted children moved -- the arm is not protecting what it says it protects"
  );

  println!(
    "AC-08.5 limb 2 at the thread door: {} of 8 unasked field(s) were cleared by a minimal legal `put`:",
    collateral.len()
  );
  for name in &collateral {
    println!("  {name}");
  }

  assert_eq!(
    collateral,
    [
      "slug",
      "status_reason",
      "acceptance",
      "objective",
      "context",
      "body",
      "preamble",
      "related"
    ],
    "the thread arm's collateral set has MOVED.\n  \
     If it SHRANK, a graft was added and AC-08.5's second limb got closer -- move this \
     literal and say so on the row.\n  \
     If it GREW, a field lost its protection, which is a regression in the surface."
  );
}
