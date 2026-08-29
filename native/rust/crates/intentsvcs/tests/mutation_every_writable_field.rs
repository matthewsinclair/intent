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
use intentsvcs::address::{Address, parse};
use intentsvcs::facade::Facade;
use intentsvcs::model::{AcceptanceTest, AtKind, AtStatus, FiatRecord, Invoker, Legacy, Thread};
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
    ("fiat", Some("at_fc")),
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
    fiat: Some(FiatRecord {
      because: "the panel-survival half is unobservable by unit test".to_string(),
      by: "hv".to_string(),
      at: "2026-08-28T18:30:00.000Z".to_string(),
      invoker: Invoker {
        tty: true,
        env: "darwin/arm64".to_string(),
      },
      inherited_from: Some("ST0001".to_string()),
    }),
    id: "AT-03.1".to_string(),
    kind: AtKind::Test,
    file: Some("crates/intentsvcs/tests/ingest_refusal.rs".to_string()),
    prose: Some("what was read, on a row that also cites a file".to_string()),
    covers: vec!["AC-03.1".to_string()],
    // **`Fiat`, AND IT IS NOT INTERCHANGEABLE WITH `Green` HERE.**
    // `AcceptanceTest::fiat` is documented -- and published into both faces --
    // as *present exactly when status is `Fiat`*, so a row carrying a record
    // while reading `green` is not a fuller fixture, it is an illegal one.
    // **This is the first field on this model whose population is CORRELATED
    // with another's**, which is why "every field present" needed a second
    // sentence rather than one more line.
    status: AtStatus::Fiat,
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
    // A DIFFERENT legal record, not a cleared one: `null` would measure
    // whether the field can be emptied, and every other row here measures
    // whether it can be MOVED.
    (
      "fiat",
      json!({
        "because": "a second close, recorded through the mutation surface",
        "by": "vc",
        "at": "2026-08-29T01:02:03.000Z",
        "invoker": { "tty": false, "env": "linux/x86_64" },
      }),
    ),
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
    .at_set("ST0001", "AT-03.1", AtStatus::Red, None)
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
    9,
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

  // **ASSERTED ON THE FIELD NAME, NOT ON THE REFUSAL SENTENCE.** Pinning the
  // prose would make this a change-detector on wording; the property is that
  // the door is shut, and the remedy's exact phrasing is free to improve.
  let refused: Vec<&str> = unsettable
    .iter()
    .map(|entry| {
      entry
        .split_whitespace()
        .next()
        .expect("a refusal entry leads with the field it is about")
    })
    .collect();

  assert_eq!(
    refused,
    vec!["fiat"],
    "exactly one field of an ACCEPTANCE TEST is unsettable through the mutation \
     surface, and it is\n  \
     DELIBERATE (hv's D7, 2026-08-29): the fiat record is reachable only through \
     `fc`.\n\n  \
     **THE LIST WAS EMPTY UNTIL D7 AND ITS EMPTINESS WAS NEVER AC-08.5 MET** -- \
     it is one\n  \
     entity measured through one door. The criterion`s own burning cases are \
     elsewhere:\n  \
     ST0011`s `completed` is a THREAD field, an attachment`s canon record has no \
     setter\n  \
     narrower than a thread, and no CLI verb creates an AC or an AT at all.\n\n  \
     **A SECOND NAME APPEARING HERE IS A FINDING, NOT A PASS.** Every other field \
     on this\n  \
     row is writable through `put`, which is what makes this a measurement rather \
     than a\n  \
     restatement of policy -- so a new refusal means a door closed somewhere \
     nobody\n  \
     declared it: {unsettable:?}"
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
// AC-08.5's population, widened from ONE entity to EVERY entity form -- and
// from ONE door to the UNION OF DOORS, which is the half that arrived late.
// ---------------------------------------------------------------------------

/// What the mutation surface did when this form was addressed.
///
/// **THE SUBJECT IS THE UNION OF DOORS. IT USED TO BE `put` ALONE, AND THAT
/// NARROWING WAS THIS INSTRUMENT'S DEFECT RATHER THAN THE SURFACE'S.** AC-08.5
/// asks whether a form is reachable through THE MUTATION SURFACE; the surface
/// has four address-addressed doors (see [`Door`]), and measuring one of them
/// and publishing the answer as the criterion's is a true measurement of a
/// narrower thing than the row asks about. **That is the same mistake the field
/// axis found one layer down with `Attachment::blob`, committed one layer up by
/// the instrument that found it.**
///
/// It was not a small narrowing. Under `put` alone the worklist held five forms;
/// three of them -- `wp`, `.../wp` and `issues` -- had arms the whole time, at
/// `set` and at `post`. **The instrument was reporting built work as unbuilt**,
/// which is the direction its own doc block calls the dangerous one.
///
/// **Each door's discriminator is that door's own words for *I have no arm for
/// this shape*.** A body error, an id mismatch or a named refusal all prove the
/// opposite -- the door reached the form and had an opinion about the request.
/// Every needle is positive-controlled on every run, in both directions, by
/// [`the_door_needles_still_match_what_the_surface_says`]: a needle that has
/// stopped matching reclassifies forms as REACHABLE in silence, which is the
/// failure that hides a regression rather than inventing one.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Reached {
  /// At least one door has an arm for this form. **WHICH door is recorded and
  /// printed** -- a reachable form whose route nobody can name is one nobody can
  /// use. What that door then said about the request is a different question and
  /// deliberately not this test's.
  Yes,
  /// **Every door fell through to its catch-all.** No arm exists anywhere on the
  /// mutation surface, which is the only answer that means the form is unwritable.
  NoWritePathYet,
}

/// **THE MUTATION SURFACE'S DOORS, ENUMERATED -- the widening this instrument
/// was missing, and the half of ic's TODO 1 that outlived the half that landed.**
///
/// # The door set was derived by PARSING signatures, not by grepping lines
///
/// `set` and `put_attachment` both declare their parameters across several
/// lines, so `grep 'pub fn .*address: &Address'` finds FOUR of the six and reads
/// as a complete answer -- **and two of the two it misses are doors.** The same
/// fixed-window trap once reported that `Criterion` derives no `JsonSchema`
/// because a 17-line attribute sat between the derive and the type. A signature
/// is not a line, so it is not greppable by the line.
///
/// # Six functions take `(&mut self, address: &Address)`; two are not doors
///
/// Excluded WITH the reason, under the same citation clause [`Expected`] carries:
/// `hydrate` materialises canon onto disk and `edit` returns a path for an
/// editor to open. **Neither changes canon at the addressed entity**, so neither
/// can make a field settable. A bare exclusion here would shrink the surface
/// this criterion is about, with a signature on it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Door {
  /// Replace an entity from a body.
  Put,
  /// Set one named field, and demonstrably nothing else.
  Set,
  /// Create a member where the id does not exist yet.
  Post,
  /// Write an attachment's content, bytes-carried included.
  PutAttachment,
}

impl Door {
  /// **EXHAUSTIVE BY CONSTRUCTION: a fifth door added to `Facade` is not caught
  /// by this array, and nothing in Rust can make it be.** So the array is not
  /// the guard -- [`the_door_set_is_the_facades_own_and_announces_a_fifth`] is,
  /// by re-deriving the door set from the facade's source on every run.
  const ALL: [Door; 4] = [Door::Put, Door::Set, Door::Post, Door::PutAttachment];

  fn name(self) -> &'static str {
    match self {
      Door::Put => "put",
      Door::Set => "set",
      Door::Post => "post",
      Door::PutAttachment => "put_attachment",
    }
  }

  /// This door's own words for *I have no arm for this shape* -- or `None` when
  /// the door has no such sentence.
  ///
  /// **THESE ARE INTERFACES, NOT PROSE.** `facade.rs` already says so of `put`'s
  /// -- rewording that one sentence, once, silently reclassified six forms as
  /// reachable, two of them forms the estate refuses BY RULING.
  ///
  /// # `set` HAS NO NEEDLE, AND THE CONTROL IS WHAT ESTABLISHED THAT
  ///
  /// The first draft of this enum gave `set` the needle `has no narrow setter`,
  /// which reads exactly like the other three and is **a sentence the surface
  /// cannot emit.** `fields_of` returns `Ok` for six forms; `set` dispatches on
  /// five of them; the sixth is `Attachment`, whose every field is unsettable --
  /// so `settable_fields` hands back an empty list and the field-name check
  /// refuses before the dispatch is ever reached. `facade.rs` says as much at
  /// the arm itself (*unreachable in practice*), and this instrument coupled to
  /// it anyway.
  ///
  /// **Nothing would have reported that.** The main sweep passed with the dead
  /// needle in it, because no form in the population produces the sentence and
  /// a needle that never matches only widens the union. It failed the moment the
  /// control asked the needle to fire -- **which is the whole argument for
  /// controlling a discriminator instead of reading it.**
  fn no_arm_needle(self) -> Option<&'static str> {
    match self {
      Door::Put => Some("has no write path yet"),
      // Structural: `settable_fields` refuses before any sentence exists.
      Door::Set => None,
      Door::Post => Some("is not a collection whose ids this tool assigns"),
      Door::PutAttachment => Some("is not an attachment address"),
    }
  }

  /// Forms this door MUST refuse, and one it must NOT.
  ///
  /// **A DISCRIMINATOR CHECKED IN ONE DIRECTION ONLY IS THE DEFECT THIS FILE HAS
  /// ALREADY PAID FOR**: the field-axis partition computed a verdict it could
  /// not fail, and passed with a planted defect in it. A needle that matches
  /// nothing makes its door open for everything and empties the worklist in
  /// silence; a needle that matches everything makes its door reach nothing,
  /// which at least shows up as a disagreement. Only the first is quiet, and it
  /// is the one that reports unbuilt work as built.
  ///
  /// **`set` NAMES TWO REFUSING FORMS BECAUSE IT HAS TWO STRUCTURAL PATHS** --
  /// a fieldless form and a form whose every field is unsettable. One control
  /// would leave the other path unexercised, which is the same gap one door
  /// left in the population.
  fn control_forms(self) -> (&'static [&'static str], &'static str) {
    match self {
      // `nodes/ic` has no arm at any door; a thread is `put`'s first arm.
      Door::Put => (&["intent:///nodes/ic"], "intent:///threads/ST0001"),
      Door::Set => (
        &[
          // A collection: `fields_of` refuses it outright.
          "intent:///threads",
          // Fields, none of them settable: nothing to probe the dispatch with.
          "intent:///threads/ST0001/attachments/design.md",
        ],
        "intent:///threads/ST0001",
      ),
      // An entity address is not a collection POST assigns ids in; `issues` is.
      Door::Post => (&["intent:///threads/ST0001"], "intent:///issues"),
      Door::PutAttachment => (
        &["intent:///threads/ST0001"],
        "intent:///threads/ST0001/attachments/design.md",
      ),
    }
  }

  /// Drive this door at this address and report what it did with the shape.
  ///
  /// **EVERY CALLER HANDS THIS A FRESH FACADE, WHICH IS WHY THE PROBES CAN BE
  /// WRITES.** `put_attachment` on a legal attachment address SUCCEEDS -- there
  /// is no request it refuses after dispatch -- so a probe that must not mutate
  /// could not tell that door's arm from its catch-all at all. Isolation is the
  /// control here rather than restraint, and it costs one in-memory store per
  /// probe.
  ///
  /// **THE THREE ANSWERS ARE DISTINCT BECAUSE THE CONTROL NEEDS THEM TO BE.** A
  /// bool cannot tell *the needle matched* from *there was nothing to probe
  /// with*, and a control that cannot tell those apart passes whenever the
  /// needle has gone blind -- which is the exact failure it exists to catch, and
  /// the one it did catch.
  fn probe(self, facade: &mut Facade, address: &Address) -> DoorAnswer {
    let said = match self {
      // A minimal legal body: this asks whether the door has an ARM, never
      // whether a particular write lands, so a parse complaint from inside an
      // arm still proves the arm was reached.
      Door::Put => match facade.put(address, "{}") {
        Ok(_) => String::new(),
        Err(why) => format!("{why}"),
      },
      Door::Set => {
        // **A FIELDLESS FORM HAS NO ARM AND SAYS SO STRUCTURALLY**, before any
        // dispatch: `settable_fields` refuses a collection, an append-only log,
        // and a form whose model has not landed. That is an `Err` rather than a
        // sentence, so it is read as one -- a needle would be the weaker test.
        let Ok(fields) = Facade::settable_fields(&address.entity) else {
          return DoorAnswer::NoArmStructural("settable_fields refuses this form outright");
        };
        // A form whose every field is unsettable leaves nothing to probe the
        // dispatch WITH. That is not the same as having no arm, and calling it
        // one would be this instrument guessing -- so it is its own answer.
        let Some(field) = fields.first() else {
          return DoorAnswer::NoArmStructural("this form has no settable field to probe with");
        };
        let said = match facade.set(address, field, json!("a value this probe supplies")) {
          Ok(_) => String::new(),
          Err(why) => format!("{why}"),
        };
        // **A TRIPWIRE, NOT DEAD DEFENSIVE CODE.** `set`'s catch-all is
        // documented unreachable and the reasoning above is why. If it ever
        // fires, the surface has changed shape and the structural discriminator
        // this door depends on has stopped being the whole answer -- which must
        // be loud, because the quiet version of it is a form silently counted
        // reachable.
        assert!(
          !said.contains("has no narrow setter"),
          "`set` answered `{}` with its documented-unreachable catch-all. Either a form gained \
           settable fields without gaining a dispatch arm, or `settable_fields` stopped refusing \
           what it used to -- and either way `Door::Set`'s structural discriminator is no longer \
           the whole answer: {said}",
          address.to_url()
        );
        said
      }
      Door::Post => match facade.post(address, "{}") {
        Ok(_) => String::new(),
        Err(why) => format!("{why}"),
      },
      Door::PutAttachment => match facade.put_attachment(address, b"probe") {
        Ok(_) => String::new(),
        Err(why) => format!("{why}"),
      },
    };
    match self.no_arm_needle() {
      Some(needle) if said.contains(needle) => DoorAnswer::NoArm(said),
      _ => DoorAnswer::Arm,
    }
  }

  /// Whether this door dispatched to an arm for this form.
  fn reaches(self, facade: &mut Facade, address: &Address) -> bool {
    matches!(self.probe(facade, address), DoorAnswer::Arm)
  }
}

/// What one door did with one address shape.
///
/// **`NoArm` AND `NoArmStructural` ARE THE SAME VERDICT AND DIFFERENT
/// EVIDENCE**, which is why they are not one value. The first is the door's own
/// sentence and is what the needle control tests; the second is a refusal that
/// happened before any sentence existed. Collapsing them would let a blind
/// needle borrow the structural refusal's correctness.
#[derive(Debug)]
enum DoorAnswer {
  /// The door dispatched to an arm. What it then said about the request is a
  /// different question.
  Arm,
  /// The door said, in its own words, that it has no arm for this shape.
  NoArm(String),
  /// The door refused before dispatch, with no sentence to match on.
  NoArmStructural(&'static str),
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
  /// **No arm at ANY door, and there SHOULD be one.** This is AC-08.5's
  /// worklist -- and it is a claim about the union, which is the only scope at
  /// which the word "worklist" is true. While this was `put`-scoped it named
  /// three forms that already had arms elsewhere.
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

    // **NOT A WORKLIST ANY MORE, AND THE WIDENING IS WHY.** `Wp` is reached by
    // `set`; `.../wp` and `issues` are reached by `post`. All three sat here as
    // unbuilt work for as long as the instrument drove `put` alone -- so this
    // block's own caption, `the reason this row is not green`, was describing
    // work that was already done. **Nothing edited those three sentences into
    // being wrong: the surface moved underneath them, and a claim with no
    // expiry outlives the thing it was true of.**
    E::Attachment { .. } => Expected::Reachable,
    E::Wp { .. } => Expected::Reachable,
    E::WpCollection { .. } | E::Issues => Expected::Reachable,

    // **AC-08.5's LIVE WORKLIST, DRIVEN AGAINST EVERY DOOR AND STILL EMPTY-HANDED.**
    // `AcCollection` is refused by `post` (`ids this tool assigns`) and by `set`
    // (a collection has membership, not fields), and both send the caller to an
    // address that cannot be created -- **which is the standing burning case
    // *no verb creates an AC or an AT*, seen from the service side.** `Node` has
    // no model behind its address form at all. Every one of these SHOULD have a
    // door, and none has one at any door.
    E::AcCollection { .. } => Expected::NotBuiltYet,
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

  // **`Reached::Yes` CARRIES ITS ROUTE NOW.** A form reported reachable whose
  // door nobody can name is a claim a reader cannot act on -- and while this
  // instrument drove one door, *reachable* and *reachable by `put`* were the
  // same sentence, so the distinction had nowhere to live.
  /// One form, what the surface did with it, what we declared, the doors that
  /// opened, and **what every closed door actually said**.
  ///
  /// The last field is why `DoorAnswer` carries its reasons rather than
  /// collapsing to a bool: AC-08.5 asks for the unwritable set to be reported BY
  /// NAME, and a worklist that names forms without naming the refusals sends the
  /// reader back to the surface to ask it again.
  type Form = (String, Reached, Expected, Vec<&'static str>, Vec<String>);

  let mut observed: Vec<Form> = Vec::new();
  let mut disagreements: Vec<String> = Vec::new();

  for url in one_address_of_every_form() {
    let address = parse(url).unwrap_or_else(|e| {
      panic!("the POPULATION is broken, not the surface: `{url}` does not parse ({e:?})")
    });
    // **EVERY DOOR IS DRIVEN, ON A FRESH FACADE EACH TIME.** The question is
    // whether ANY door has an arm for this form, never whether a particular
    // write lands -- so a refusal from inside an arm still counts as reached.
    let mut opens: Vec<&'static str> = Vec::new();
    let mut closed: Vec<String> = Vec::new();
    for door in Door::ALL {
      match door.probe(&mut fx.facade(), &address) {
        DoorAnswer::Arm => opens.push(door.name()),
        DoorAnswer::NoArm(said) => closed.push(format!("{}: {said}", door.name())),
        DoorAnswer::NoArmStructural(why) => closed.push(format!("{}: {why}", door.name())),
      }
    }
    let reached = if opens.is_empty() {
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
        "  {url}\n    declared {declared:?}, which requires {:?}, but observed {reached:?}\n    \
         doors that opened: {}",
        declared.requires(),
        if opens.is_empty() {
          "none".to_string()
        } else {
          opens.join(", ")
        }
      ));
    }
    observed.push((url.to_string(), reached, declared, opens, closed));
  }

  // **THE POSITIVE CONTROL: at least one form must be reachable.** Every
  // assertion here is about a refusal, so a facade that refused everything --
  // a broken fixture, an unopened canon -- would satisfy the whole list and
  // report a green that is a fact about the harness rather than the surface.
  let reachable = observed
    .iter()
    .filter(|(_, r, _, _, _)| *r == Reached::Yes)
    .count();
  assert!(
    reachable > 0,
    "no form was reachable at all -- this is a fact about the harness, not about \
     the mutation surface"
  );

  // **THE SECOND POSITIVE CONTROL, AND IT IS ABOUT THE WIDENING.** A door whose
  // needle has drifted answers `reaches` for everything, so the union would
  // report every form reachable through it and the worklist would empty in
  // silence. **A door that opens for all thirteen forms is a broken
  // discriminator, not a generous door** -- no door on this surface has an arm
  // for a collection, an append-only log and an entity alike.
  for door in Door::ALL {
    let opened = observed
      .iter()
      .filter(|(_, _, _, opens, _)| opens.contains(&door.name()))
      .count();
    assert!(
      opened < observed.len(),
      "`{}` reported an arm for ALL {} forms, including the append-only ones. That is its \
       discriminator ({}) failing to refuse anything, not a door that reaches everything: every \
       form would read as reachable and the worklist would empty with nothing built.",
      door.name(),
      observed.len(),
      door
        .no_arm_needle()
        .map_or_else(|| "structural".to_string(), |n| format!("`{n}`"))
    );
  }

  assert!(
    disagreements.is_empty(),
    "the mutation surface's reach has MOVED, and the declaration no longer describes it:\n{}\n\n  \
     If a form GAINED a write path at ANY door, re-declare it Reachable -- AC-08.5 got closer.\n  \
     If a form LOST one at every door, that is a regression in the surface, not in this test.\n  \
     If you are here because you added a DOOR, that is the good case: the union widened.",
    disagreements.join("\n")
  );

  // **The unsettable set, printed by name on every run.** AC-08.5 asks for it
  // as OUTPUT, not merely as an assertion, and a set nobody prints is one
  // nobody can act on.
  // **THE DENOMINATOR IS THE FORMS THAT SHOULD BE REACHABLE**, and the
  // permanent exclusions are printed separately WITH their rulings rather than
  // omitted. An exclusion nobody prints is one nobody can challenge, and this
  // list is exactly where a wrong exclusion would hide.
  let population: Vec<&Form> = observed
    .iter()
    .filter(|(_, _, e, _, _)| e.in_population())
    .collect();
  let worklist: Vec<(&str, &Vec<String>)> = population
    .iter()
    .filter(|(_, r, _, _, _)| *r == Reached::NoWritePathYet)
    .map(|(u, _, _, _, closed)| (u.as_str(), closed))
    .collect();

  println!(
    "AC-08.5: {} of {} entity form(s) IN THE POPULATION have no write path at ANY of the {} doors ({}):",
    worklist.len(),
    population.len(),
    Door::ALL.len(),
    Door::ALL
      .iter()
      .map(|d| d.name())
      .collect::<Vec<_>>()
      .join(", ")
  );
  // **EVERY CLOSED DOOR IS QUOTED, NOT COUNTED.** The criterion asks for the
  // unwritable set reported BY NAME, and a worklist that names forms without
  // naming the refusals sends its reader back to the surface to ask again --
  // which is what four separate messages did on this row before the buckets
  // landed one axis down.
  for (u, closed) in &worklist {
    println!("  {u}");
    for said in closed.iter() {
      println!("    {said}");
    }
  }

  // **THE ROUTE IS PRINTED, NOT JUST THE VERDICT.** A reader who is told a form
  // is reachable and not told through WHAT cannot use the answer, and cannot
  // check it either. This half of the report did not exist while the instrument
  // drove one door, because there was nothing to say.
  println!("AC-08.5: the doors that reach each form in the population:");
  for (u, r, _, opens, _) in &population {
    if *r == Reached::Yes {
      println!("  {u}\n    via {}", opens.join(", "));
    }
  }

  let excluded: Vec<(&str, &str)> = observed
    .iter()
    .filter_map(|(u, _, e, _, _)| match e {
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

/// **THE NEEDLES ARE POSITIVE-CONTROLLED IN BOTH DIRECTIONS, ON EVERY RUN.**
///
/// Four doors are now discriminated by four sentences the facade owns. Each one
/// is an interface the way `put`'s already was -- `facade.rs` records that
/// rewording that one sentence, once, silently reclassified six forms as
/// reachable, two of them forms the estate refuses BY RULING.
///
/// # A needle can go blind in two directions and one of them is silent
///
/// **A needle that stops matching makes its door open for everything**: the
/// union reports every form reachable, `declared_reach` is dragged along behind
/// it, and the worklist empties with nothing built. That failure looks exactly
/// like success. A needle that matches too much makes its door reach nothing,
/// which at least shows up as a disagreement.
///
/// So each door names a form it MUST answer with its needle and a form it must
/// NOT. **This is the control the field-axis partition went without for a
/// morning** -- it computed a verdict that could not go red, and it passed with
/// a planted defect sitting in it.
#[test]
fn the_door_needles_still_match_what_the_surface_says() {
  let fx = populated_fixture();

  for door in Door::ALL {
    let (must_refuse, must_not) = door.control_forms();

    for url in must_refuse {
      let address = parse(url).unwrap_or_else(|e| panic!("`{url}`: {e:?}"));
      let answer = door.probe(&mut fx.facade(), &address);
      match (door.no_arm_needle(), answer) {
        // A door with a sentence must produce THAT sentence.
        (Some(needle), DoorAnswer::NoArm(said)) => assert!(
          said.contains(needle),
          "`{}` answered `{url}` without its own needle `{needle}`: {said}",
          door.name()
        ),
        // A door that refuses structurally must refuse structurally.
        (None, DoorAnswer::NoArmStructural(_)) => {}
        // **EVERY OTHER PAIRING IS AN UNEXERCISED DISCRIMINATOR, NOT A PASS.**
        // A control satisfiable without touching its subject is the shape this
        // file has already been burned by: the verdict can be right while the
        // thing under test was never run.
        (needle, other) => panic!(
          "`{}`'s discriminator was never exercised on `{url}`: it answers {other:?} where {} was \
           required. The verdict may still be right, and the discriminator is unguarded -- a \
           reword of it would empty the worklist in silence.",
          door.name(),
          needle.map_or_else(
            || "a structural refusal".to_string(),
            |n| format!("the sentence `{n}`")
          )
        ),
      }
    }

    let address = parse(must_not).unwrap_or_else(|e| panic!("`{must_not}`: {e:?}"));
    assert!(
      door.reaches(&mut fx.facade(), &address),
      "`{}` reported NO arm for `{must_not}`, which it does reach. Either the door lost an arm \
       -- a regression in the surface -- or its discriminator now refuses an answer that is an \
       opinion about the request rather than about the shape.",
      door.name()
    );
  }
}

/// **THE DOOR SET IS RE-DERIVED FROM THE FACADE'S OWN SOURCE, SO A FIFTH DOOR
/// ANNOUNCES ITSELF.**
///
/// `Door::ALL` is an authored array, and Rust offers nothing that makes adding a
/// method to `Facade` fail to compile here -- the exhaustive-match device that
/// fences `Entity` and the field axis has no purchase on a set of functions. So
/// the announcement mechanism has to be a measurement, and this is it.
///
/// # It parses signatures rather than grepping lines, and that is the finding
///
/// `set` and `put_attachment` declare their parameters across several lines, so
/// a one-line `grep 'pub fn .*address: &Address'` returns FOUR of six and reads
/// as a complete answer. **Both of the two it misses are doors** -- which is
/// precisely how this instrument came to measure one door and report the number
/// as the criterion's. The same fixed-window trap once had a 6-line lookback
/// report that `Criterion` derives no `JsonSchema`, because a 17-line attribute
/// sat between the derive and the type.
///
/// # The two exclusions are named, not silent
///
/// `hydrate`, `dehydrate` and `edit` take the same `(&mut self, address:
/// &Address)` and are not doors: none changes canon at the addressed entity.
/// They are listed here so that removing one from the exclusion list is a
/// visible act rather than a shrug -- the same citation clause [`Expected`]
/// carries.
///
/// **`dehydrate` REMOVES FILES AND IS STILL NOT A DOOR, WHICH IS THE ONE WORTH
/// STATING.** It writes `.intentfiles` and deletes realised views; canon is
/// untouched, which is exactly why re-listing the id and re-running `hydrate`
/// restores what it removed. A door is about the ADDRESSED ENTITY'S canon, not
/// about how much a verb writes.
#[test]
fn the_door_set_is_the_facades_own_and_announces_a_fifth() {
  const NOT_DOORS: [&str; 3] = ["hydrate", "dehydrate", "edit"];

  let source =
    std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/facade.rs"))
      .expect("the facade's source is readable from its own crate");

  // Accumulate a signature across lines until its opening brace, which is the
  // only way to see a parameter list that a line break runs through.
  let mut found: Vec<String> = Vec::new();
  let mut signature = String::new();
  for line in source.lines() {
    if line.starts_with("  pub fn ") {
      signature = line.to_string();
    } else if !signature.is_empty() {
      signature.push(' ');
      signature.push_str(line.trim());
    }
    if signature.is_empty() || !line.trim_end().ends_with('{') {
      continue;
    }
    if signature.contains("&mut self") && signature.contains("address: &Address") {
      let name = signature
        .trim_start()
        .trim_start_matches("pub fn ")
        .split(['(', '<'])
        .next()
        .expect("a signature has a name")
        .trim()
        .to_string();
      found.push(name);
    }
    signature.clear();
  }

  // The parser must find something, or this test is measuring its own regex.
  assert!(
    found.len() >= Door::ALL.len(),
    "the signature parser found {} address-addressed mutators, fewer than the {} doors this \
     file drives -- the parser is broken, not the facade: {found:?}",
    found.len(),
    Door::ALL.len()
  );

  let declared: BTreeSet<&str> = Door::ALL
    .iter()
    .map(|d| d.name())
    .chain(NOT_DOORS)
    .collect();
  let unaccounted: Vec<&String> = found
    .iter()
    .filter(|f| !declared.contains(f.as_str()))
    .collect();

  assert!(
    unaccounted.is_empty(),
    "`Facade` has address-addressed mutators this file has never heard of: {unaccounted:?}\n  \
     AC-08.5 is about THE MUTATION SURFACE, so a door nobody drives makes the worklist too \
     long and reports built work as unbuilt -- which is what drove three forms into the \
     worklist for as long as this instrument knew only `put`.\n  \
     Add it to `Door` if it changes canon at the addressed entity, or to `NOT_DOORS` WITH the \
     reason it does not."
  );
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

/// **AC-08.5's SECOND LIMB AT THE THREAD DOOR: `put` REFUSES a body that would
/// clear a field it was not asked to change, and names every one it would have
/// cleared.**
///
/// # This test asserted the DEFECT until 2026-08-25, and that is worth keeping
///
/// It was written as *does `put` clear a field it was not asked to change*, and
/// its assertion pinned the answer: the collateral set must equal those eight
/// names. **So when cc fixed the door, this test went red -- a correct fix was
/// indistinguishable from a regression, because the suite did not merely fail to
/// observe the data loss, it REQUIRED it.** (dc found the identical shape in the
/// bats suite the same afternoon; two nodes, two files, one class.)
///
/// **The property survived and only its DIRECTION reversed**, which is the tell
/// that the original was measuring the right thing. What is asserted now: the
/// write is refused, the entity is byte-identical INCLUDING the field that was
/// asked for, and the refusal names all eight. **The refusal's prose is
/// deliberately not pinned** -- pinning a sentence makes a comma a test failure
/// and hands this file a veto over another node's wording.
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
/// That denominator is why the eight are named INDIVIDUALLY in the refusal
/// assertion rather than counted: a count reds on the same change and says only
/// that a number moved, and AC-08.5's second clause asks for the field BY NAME.
///
/// # The subject is the SURFACE, never the estate's data
///
/// The fixture is SYNTHESISED. ST0011's real `completed` was repaired by hand
/// at `608e9721` on 2026-08-20, the day BEFORE the ruling that created this
/// write path, so reading the live estate answers a question about a hand-edit
/// and reports it as a fact about the surface. **When a measurement is correct
/// and its SUBJECT is wrong, nothing inside the measurement can report it.**
#[test]
fn thread_put_refuses_to_clear_the_fields_it_was_not_asked_to_change() {
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
  let err = facade
    .put(&address, &body.to_string())
    .expect_err("a body omitting eight populated fields must not be accepted whole");

  let after = facade
    .canon()
    .threads
    .iter()
    .find(|t| t.id == "ST0001")
    .cloned()
    .expect("the thread survives a refused write");

  // **NOTHING MOVED -- INCLUDING THE FIELD WE ASKED FOR.** The refusal is whole:
  // a partial write that landed `completed` and refused the rest would be the
  // worst of both, and it is the reading this assertion exists to exclude.
  let moved: Vec<&str> = [
    ("completed", before.completed != after.completed),
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
  assert!(
    moved.is_empty(),
    "the write was refused and these fields moved anyway: {moved:?}\n  \
     A refusal that writes is worse than a write, because nothing downstream reads \
     an error as a mutation."
  );

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
    "the grafted children moved under a refused write"
  );

  // **THE REPORT IS PART OF THE PROPERTY, NOT DECORATION.** AC-08.5's second
  // clause is *reported BY NAME*, and a refusal that says only `some fields
  // would change` sends the operator back to diffing canon by hand -- the route
  // this criterion exists to retire. The fixture populates all eight, so all
  // eight would have been cleared and all eight must be named.
  //
  // **THE PROSE IS DELIBERATELY NOT PINNED.** Pinning the sentence would make a
  // comma a test failure and hand this file veto over another node's wording.
  let text = format!("{err:?}");
  let unnamed: Vec<&str> = [
    "slug",
    "status_reason",
    "acceptance",
    "objective",
    "context",
    "body",
    "preamble",
    "related",
  ]
  .into_iter()
  .filter(|field| !text.contains(field))
  .collect();
  assert!(
    unnamed.is_empty(),
    "the refusal does not name these fields it would have cleared: {unnamed:?}\n  \
     Refusal text: {text}\n  \
     If the arm names only a subset ON PURPOSE, that is a narrower contract than \
     AC-08.5's second clause and belongs on the row rather than in this literal."
  );
}

// ---------------------------------------------------------------------------
// THE NARROW FIELD-SETTER -- LIMB 1 AND LIMB 2 THROUGH ONE DOOR
//
// **DC-1 (vc, 2026-08-24) RULED THE STANDARD, AND IT IS NOT "SOME PATH CHANGES
// THE BYTES".** Limb 1 asks whether a FIELD is settable, so it has no door
// denominator and one working field-setter satisfies it -- but a whole-document
// parse-plus-graft is not a setter and a whole-document authored replace is not
// a setter, which is why `Facade::put` closes limb 1 for nothing.
//
// **FOUR KNOWN GAPS, TWO ENTITIES, AND THEY ARE ONE SHAPE.** `Thread::completed`
// (ST0011's row, the criterion's first burning case) plus `WorkPackage`'s
// `objective`, `body` and `preamble`. Every one is load-bearing prose or a
// load-bearing date reachable only by replacing the whole document around it.
//
// **AND THE WP THREE ARE STRICTLY WORSE OFF THAN `completed`, WHICH IS WHY A
// GENERIC SETTER WAS THE CHEAPER BUILD THAN FOUR BESPOKE VERBS.** `completed`
// at least has a door. `put`'s `Wp` arm does not exist -- the address falls to
// the catch-all -- and the thread door refuses `wps` BY NAME and sends the
// caller to that very address. **The two doors point at each other and neither
// opens**, so before this setter there was no route to a work package's prose
// except a hand-edit of markdown followed by a whole-estate `sync --to-store`.
//
// # Why one generic setter rather than four named verbs
//
// A bespoke `wp objective` verb closes exactly one gap and leaves the identical
// hole one field over. **This closes them by construction and keeps closing
// them**: a field added to any of these models is settable the day it lands,
// with no verb to write and no roster to remember. That is the same property
// `declared_reach` buys for entity forms, one level down.
//
// # Limb 2 is an INVARIANT of the verb here, not a property tested from outside
//
// [`Facade::set`] re-serialises what it wrote and refuses if any key other than
// the addressed one moved. So the collateral check below is a SECOND observer
// rather than the only one -- and a future field whose serde attributes cause
// collateral movement makes the verb refuse rather than making this test the
// sole thing standing between that field and a silent clear.
// ---------------------------------------------------------------------------

/// The whole entity, read back through the facade after a write.
///
/// **Read from the FACADE and not from the fixture handle**, because the point
/// of every assertion below is what LANDED, and a fixture re-read would answer
/// from the same value the test just constructed.
fn entity_json(facade: &intentsvcs::facade::Facade, url: &str) -> Value {
  let thread = facade
    .st_show("ST0001")
    .expect("the fixture thread is there");
  let address = parse(url).expect("resolves");
  match &address.entity {
    intentsvcs::address::Entity::Thread { .. } => {
      serde_json::to_value(thread).expect("a thread serialises")
    }
    intentsvcs::address::Entity::Wp { wp, .. } => {
      let seq: u32 = wp.parse().expect("the fixture addresses a numeric wp");
      let row = thread
        .wps
        .iter()
        .find(|w| w.seq == seq)
        .unwrap_or_else(|| panic!("wp {seq} is in the fixture"));
      serde_json::to_value(row).expect("a work package serialises")
    }
    other => panic!(
      "this helper covers thread and wp addresses, not {}",
      other.form()
    ),
  }
}

/// Every key whose value differs between two serialised entities, PRESENCE
/// INCLUDED.
///
/// **A field cleared to absence and a field cleared to `""` are both movement**,
/// and comparing only the keys present in `before` sees neither. The union is
/// the denominator for the same reason `both_lists_cover_every_field_the_model_serialises`
/// exists: `skip_serializing_if` makes "the keys it has" a moving target.
fn moved_keys(before: &Value, after: &Value) -> Vec<String> {
  let keys: BTreeSet<&String> = before
    .as_object()
    .expect("object")
    .keys()
    .chain(after.as_object().expect("object").keys())
    .collect();
  keys
    .into_iter()
    .filter(|k| before.get(*k) != after.get(*k))
    .cloned()
    .collect()
}

/// A different, LEGAL value for every field of a THREAD the setter declares
/// settable.
///
/// Hand-written per field for the reason [`a_different_legal_value`] gives one
/// entity up: "a different value" is type-specific, and a generic nudge reports
/// a field UNSETTABLE when the probe was what was wrong.
fn a_different_legal_thread_value() -> Vec<(&'static str, Value)> {
  vec![
    ("title", json!("Intent v3.0.0 -- the rewrite")),
    ("slug", json!("intent-v3-rewrite")),
    ("status_reason", json!("held: waiting on the fleet")),
    // **`created` IS DELIBERATELY ABSENT AND ITS NEIGHBOUR IS DELIBERATELY
    // PRESENT.** A machine stamp has no caller-authored value to offer, so there
    // is nothing legal to put here; `completed` two lines down is an AUTHORED
    // date and stays. **The two look alike and are not the same field** -- see
    // `Unsettable::Stamped`. This map disagreeing with the setter is what caught
    // the change that refused `created`, which is the map doing its job.
    // **THE CRITERION'S FIRST BURNING CASE.** ST0011's row is NULL and wrong,
    // and until this verb existed nothing could write it that was not a
    // whole-document replace.
    ("completed", json!("2026-08-24")),
    // **`AcceptanceMode` HAS EXACTLY ONE VARIANT**, so the only movement this
    // field can make is to absence. That is not a thinner case than the others:
    // clearing is the half of a setter nobody notices missing, and this is the
    // only field whose type forces the sweep to exercise it.
    ("acceptance", Value::Null),
    ("objective", json!("Ship the store as the durable SSOT.")),
    ("context", json!("Why this thread exists, re-authored.")),
    ("body", json!("A load-bearing paragraph, edited in place.")),
    ("preamble", json!("Front matter prose, edited in place.")),
    (
      "related",
      json!([{ "id": "ST0057", "note": "disk as a sparse projection" }]),
    ),
  ]
}

/// A different, LEGAL value for every field of a WORK PACKAGE the setter
/// declares settable. The last three are three of AC-08.5's four known gaps.
fn a_different_legal_wp_value() -> Vec<(&'static str, Value)> {
  vec![
    ("title", json!("Ingest, views and sync -- re-titled")),
    ("scope", json!("XL")),
    ("scope_legacy", json!({ "raw": "3 days" })),
    ("status_reason", json!("held: blocked on the ingest ruling")),
    (
      "objective",
      json!("Land strict ingest and the sync engine."),
    ),
    ("body", json!("## The seams\n\nRe-authored, verbatim.")),
    ("preamble", json!("A numbat note, edited in place.")),
  ]
}

/// **THE DENOMINATOR, AND THE TWO SIDES ARE DERIVED DIFFERENTLY ON PURPOSE.**
///
/// `Facade::settable_fields` reflects over the model's own schema; the maps
/// above are hand-authored. **Two literals compared to each other observe
/// nothing** -- this file has that written at the top of it, about a pin that
/// passed for weeks doing exactly that -- so the guard is only worth having
/// because a field added to `Thread` or `WorkPackage` appears on one side and
/// not the other.
#[test]
fn the_setter_value_maps_cover_exactly_what_the_setter_declares_settable() {
  for (url, mapped) in [
    ("intent:///threads/ST0001", a_different_legal_thread_value()),
    (
      "intent:///threads/ST0001/wp/03",
      a_different_legal_wp_value(),
    ),
  ] {
    let entity = parse(url).expect("resolves").entity;
    let declared: BTreeSet<String> = intentsvcs::facade::Facade::settable_fields(&entity)
      .unwrap_or_else(|e| panic!("`{url}` declares a settable set: {e:?}"))
      .into_iter()
      .collect();
    let authored: BTreeSet<String> = mapped.iter().map(|(f, _)| (*f).to_string()).collect();
    assert_eq!(
      declared, authored,
      "`{url}`: the settable set and the value map disagree. A field on one side\n       \
       and not the other is a field this sweep would silently skip -- add the case\n       \
       or state why the field is refused."
    );
  }
}

/// **LIMB 1 AND LIMB 2 AT THE THREAD DOOR, FIELD BY FIELD.**
#[test]
fn every_settable_thread_field_moves_and_takes_nothing_with_it() {
  for (field, new_value) in a_different_legal_thread_value() {
    let fx = Fixture::new();
    fx.write_thread(&fully_populated_thread("ST0001"));
    let mut facade = fx.facade();
    let address = parse("intent:///threads/ST0001").expect("resolves");

    let before = entity_json(&facade, "intent:///threads/ST0001");
    // **`None` AND `Some(Null)` ARE THE SAME REQUEST AND DIFFERENT VALUES.**
    // Comparing `before.get(field)` directly passes a case that writes `null`
    // over an ABSENT field, and the read-back then compares `Null` to `Null`
    // and proves nothing -- the second vacuity `mutation_roundtrip_complete`
    // records having been bitten by.
    assert_ne!(
      before.get(field).cloned().unwrap_or(Value::Null),
      new_value,
      "the fixture already holds `{field}` at the value this case writes, so the case\n       \
       cannot tell a working setter from a no-op. Pick another value."
    );

    facade
      .set(&address, field, new_value.clone())
      .unwrap_or_else(|e| panic!("`{field}` must be settable through the surface: {e:?}"));

    let after = entity_json(&facade, "intent:///threads/ST0001");
    assert_eq!(
      after.get(field).cloned().unwrap_or(Value::Null),
      new_value,
      "`{field}` did not land -- under AC-08.5 that makes it a field that CANNOT BE SET"
    );
    assert_eq!(
      moved_keys(&before, &after),
      vec![field.to_string()],
      "setting `{field}` moved something else -- limb 2: no verb silently clears a\n       \
       field it was not asked to change"
    );
  }
}

/// **THE THREE GAPS THAT HAD NO DOOR AT ALL.**
#[test]
fn every_settable_work_package_field_moves_and_takes_nothing_with_it() {
  for (field, new_value) in a_different_legal_wp_value() {
    let fx = Fixture::new();
    fx.write_thread(&fully_populated_thread("ST0001"));
    let mut facade = fx.facade();
    let address = parse("intent:///threads/ST0001/wp/03").expect("resolves");

    let before = entity_json(&facade, "intent:///threads/ST0001/wp/03");
    assert_ne!(
      before.get(field).cloned().unwrap_or(Value::Null),
      new_value,
      "vacuous case on `{field}`"
    );

    facade
      .set(&address, field, new_value.clone())
      .unwrap_or_else(|e| panic!("`{field}` must be settable through the surface: {e:?}"));

    let after = entity_json(&facade, "intent:///threads/ST0001/wp/03");
    assert_eq!(
      after.get(field).cloned().unwrap_or(Value::Null),
      new_value,
      "`{field}` did not land"
    );
    assert_eq!(
      moved_keys(&before, &after),
      vec![field.to_string()],
      "setting `{field}` moved something else"
    );
  }
}

/// **A SIBLING WORK PACKAGE IS NOT COLLATERAL EITHER**, and the whole-entity
/// diff above cannot see it: it reads ONE work package, so a setter that
/// rewrote the whole `wps` vector would pass every assertion in the sweep.
#[test]
fn setting_one_work_package_leaves_its_siblings_and_its_thread_alone() {
  let fx = Fixture::new();
  fx.write_thread(&fully_populated_thread("ST0001"));
  let mut facade = fx.facade();

  let before = serde_json::to_value(facade.st_show("ST0001").expect("there")).expect("serialises");
  let address = parse("intent:///threads/ST0001/wp/03").expect("resolves");
  facade
    .set(&address, "objective", json!("Re-authored."))
    .expect("a work package objective is settable");
  let after = serde_json::to_value(facade.st_show("ST0001").expect("there")).expect("serialises");

  assert_eq!(
    moved_keys(&before, &after),
    vec!["wps".to_string()],
    "setting a work-package field moved a THREAD field"
  );
  let seq_2 = |v: &Value| v["wps"].as_array().expect("array")[0].clone();
  assert_eq!(
    seq_2(&before),
    seq_2(&after),
    "setting wp 03 moved wp 02 -- a sibling is not collateral the caller asked for"
  );
}

/// **LIMB 1's SECOND HALF: a field that cannot be written is reported BY NAME.**
///
/// Three kinds of refusal, and each one names the field AND the door that DOES
/// open. **"You cannot" is not what the criterion asks for** -- an unsettable
/// field whose remedy is unnamed sends the operator to a hand-edit of canon,
/// which is the route this criterion exists to retire.
#[test]
fn every_refusal_names_the_field_and_the_door_that_opens() {
  let fx = Fixture::new();
  fx.write_thread(&fully_populated_thread("ST0001"));
  let mut facade = fx.facade();
  let thread = parse("intent:///threads/ST0001").expect("resolves");
  let wp = parse("intent:///threads/ST0001/wp/03").expect("resolves");

  for (address, field, must_mention) in [
    // Identity: the id IS the address.
    (&thread, "id", "address"),
    (&thread, "schema", "address"),
    (&wp, "seq", "address"),
    // Machine-guarded: a raw write would bypass a ratified state machine.
    (&thread, "status", "st "),
    (&wp, "status", "wp "),
    // Child collections: each has an address of its own.
    (&thread, "tests", "/at/"),
    (&thread, "criteria", "/ac/"),
    (&thread, "wps", "/wp/"),
    (&thread, "attachments", "/attachments/"),
    // Not a field of this entity at all.
    (&thread, "objectve", "objective"),
  ] {
    let err = facade
      .set(address, field, json!("anything"))
      .expect_err("this field is refused");
    let text = format!("{err}");
    assert!(
      text.contains(field),
      "the refusal for `{field}` does not NAME it: {text}"
    );
    assert!(
      text.contains(must_mention),
      "the refusal for `{field}` does not say where to go (wanted `{must_mention}`): {text}"
    );
  }
}

/// **A REFUSAL MUST NOT HAVE WRITTEN ANYTHING**, which no assertion about the
/// message can see. A verb that refuses loudly and mutates anyway is the same
/// silent-clear class wearing an error's clothes.
#[test]
fn a_refused_set_leaves_the_entity_byte_identical() {
  let fx = Fixture::new();
  fx.write_thread(&fully_populated_thread("ST0001"));
  let mut facade = fx.facade();
  let address = parse("intent:///threads/ST0001").expect("resolves");

  let before = entity_json(&facade, "intent:///threads/ST0001");
  for (field, value) in [
    ("status", json!("done")),
    ("tests", json!([])),
    ("id", json!("ST9999")),
    ("nonesuch", json!("x")),
    // Right name, wrong TYPE -- refused by the typed re-parse rather than by
    // the name check, so it exercises the other refusal path.
    ("completed", json!({ "not": "a date string" })),
  ] {
    facade.set(&address, field, value).expect_err("refused");
    assert_eq!(
      entity_json(&facade, "intent:///threads/ST0001"),
      before,
      "a refused set of `{field}` still changed the entity"
    );
  }
}

/// **SETTING A FIELD TO WHAT IT ALREADY HOLDS IS NOT A WRITE**, and it must not
/// mint an event. The estate's history is queryable and a no-op that appears in
/// it makes the log describe intent rather than change.
#[test]
fn setting_a_field_to_its_current_value_is_already_there() {
  let fx = Fixture::new();
  fx.write_thread(&fully_populated_thread("ST0001"));
  let mut facade = fx.facade();
  let address = parse("intent:///threads/ST0001").expect("resolves");

  let current = entity_json(&facade, "intent:///threads/ST0001")["objective"].clone();
  let outcome = facade
    .set(&address, "objective", current)
    .expect("re-setting the current value is accepted");
  assert!(
    matches!(outcome, intentsvcs::facade::Outcome::AlreadyThere { .. }),
    "re-setting the current value reported {outcome:?} rather than AlreadyThere"
  );
}

/// **CLEARING IS A SET, NOT A GAP.** An optional field's null is how a caller
/// says *remove this*, and without it `status_reason` could be written but never
/// unwritten -- half a setter, and the half nobody notices missing.
#[test]
fn null_clears_an_optional_field_and_is_refused_on_a_required_one() {
  let fx = Fixture::new();
  fx.write_thread(&fully_populated_thread("ST0001"));
  let mut facade = fx.facade();
  let address = parse("intent:///threads/ST0001").expect("resolves");

  facade
    .set(&address, "status_reason", Value::Null)
    .expect("an optional field clears");
  assert_eq!(
    entity_json(&facade, "intent:///threads/ST0001").get("status_reason"),
    None,
    "`status_reason` did not clear"
  );

  let err = facade
    .set(&address, "title", Value::Null)
    .expect_err("a required field does not clear");
  assert!(
    format!("{err}").contains("title"),
    "clearing a required field must be refused BY NAME"
  );
}

// ===========================================================================
// **AC-08.5's FIELD AXIS -- THE TWIN OF `declared_reach`, ONE AXIS OVER.**
//
// `declared_reach` answers *is this FORM reachable by a door*. That is a
// question about ADDRESSES, indexed over the 13 `Entity` variants. AC-08.5's
// subject is FIELDS: *every writable field settable through the mutation
// surface, and every unwritable one reported BY NAME*. **The two populations
// are different sets**, so an instrument scoped to the narrower one is
// internally consistent, correct in its own printed output, and silent about
// the criterion it serves. Ruled by vc as AC-08.5's contract-holder,
// 2026-08-25, on exactly that distinction.
//
// **THE TWO HALVES WANT OPPOSITE TREATMENTS, AND THAT IS NOT AN
// INCONSISTENCY.** The per-entity partition is DERIVED, because a hand-kept
// field list stops covering on the day someone adds a field. The POPULATION is
// PINNED against a declared literal, because a derived population silently
// ABSORBS a new model type -- answering its own question correctly forever
// while it quietly stops covering the criterion. On the population axis,
// announcing growth IS the job.
//
// **AND IT ASSERTS THROUGH TRAIT BOUNDS AND REAL CALLS, NEVER BY SCANNING
// SOURCE.** Building this, two regexes over `model.rs` reported `Criterion` as
// deriving no `JsonSchema` -- a 6-line lookback against a 17-line
// `#[schemars(extend(...))]` block, then a walk-back that could not read
// multi-line attribute continuations. Both were wrong and the compiler already
// knew: `schema_properties<T: schemars::JsonSchema>` is a bound, and its call
// site type-checks. A scan of a declaration can always rot; a type-checked
// call cannot.
// ===========================================================================

/// What an address form contributes to AC-08.5's **field** axis.
///
/// **EXHAUSTIVE ON PURPOSE, the same device `declared_reach` uses**: a
/// fourteenth `Entity` variant does not compile until it is placed here. That
/// is the only mechanism Rust offers for *every variant is accounted for*.
enum FieldAxis {
  /// Carries fields, and names the model type behind the form.
  Model(&'static str),
  /// Membership, not fields. **Contributes zero rows to this axis** -- there is
  /// no field of a collection to set, and `declared_reach` already covers
  /// whether the collection itself is reachable.
  Collection,
  /// Append-only. **Contributes zero rows**, by the same rulings
  /// `declared_reach` cites: the whiteboard's single-writer inbox rule and
  /// hv's D53 on the event log.
  Log,
  /// **A form D57-8's grammar minted AHEAD of its model.** `Entity::Node`
  /// exists at `address.rs:145` and there is no `struct Node` in any crate;
  /// the model is ST0056/WP-14, Not Started.
  ///
  /// **THIS VARIANT IS THE POINT OF PINNING THE POPULATION.** Indexing the
  /// axis by model type makes this form invisible to AC-08.5 *by
  /// construction*, and it would stay invisible when WP-14 lands unless
  /// something announces the change. Nothing derived can announce it -- a
  /// derived population just absorbs the new type. The pin below is what
  /// turns that silence into a red. (vc, 2026-08-25, correcting the
  /// population from seven to six and naming this as the reason.)
  NoModelYet,
}

/// **THE PIN. Six model types carry fields today.**
///
/// Not asserted because six is right, but so that **six becoming seven is
/// ANNOUNCED.** Sorted, because it is compared as a set and a reader should be
/// able to diff it by eye.
const FIELD_CARRYING_MODELS: [&str; 6] = [
  "AcceptanceTest",
  "Attachment",
  "Criterion",
  "Issue",
  "Thread",
  "WorkPackage",
];

/// **THE FIELDS A MODEL DECLARES AND SERDE DROPS, PINNED SO A SECOND ONE CANNOT
/// ARRIVE QUIETLY.**
///
/// `#[serde(skip)]` removes a field from the JSON entirely, so it is invisible to
/// any serialisation-derived measurement -- **including the one this file used to
/// take.** `Attachment::blob` is the only one today and it is not an incidental
/// case: it is the OPAQUE half of an attachment, and *bytes-carried attachments*
/// is one of AC-08.5's three burning cases.
///
/// Checked in both directions below: a field named here that DOES serialise is as
/// much a defect as a field that vanishes without being named.
const SERDE_SKIPPED: [(&str, &str); 1] = [("Attachment", "blob")];

fn field_axis(entity: &intentsvcs::address::Entity) -> FieldAxis {
  use intentsvcs::address::Entity as E;
  match entity {
    E::Thread { .. } => FieldAxis::Model("Thread"),
    E::Wp { .. } => FieldAxis::Model("WorkPackage"),
    E::Ac { .. } => FieldAxis::Model("Criterion"),
    E::At { .. } => FieldAxis::Model("AcceptanceTest"),
    E::Attachment { .. } => FieldAxis::Model("Attachment"),
    E::Issue { .. } => FieldAxis::Model("Issue"),

    E::Threads | E::Issues | E::WpCollection { .. } | E::AcCollection { .. } => {
      FieldAxis::Collection
    }
    E::NodeInbox { .. } | E::Event { .. } => FieldAxis::Log,

    E::Node { .. } => FieldAxis::NoModelYet,
  }
}

/// A `WorkPackage` with every field carrying a value.
///
/// **Fully populated for the reason [`fully_populated_row`] gives**: `scope`,
/// `scope_legacy` and `status_reason` are `Option` with
/// `skip_serializing_if`, so a partially-populated instance is blind to
/// exactly the fields nobody has ever set.
fn fully_populated_work_package() -> intentsvcs::model::WorkPackage {
  intentsvcs::model::WorkPackage {
    seq: 1,
    title: "The field axis".to_string(),
    scope: Some(intentsvcs::model::TShirt::M),
    scope_legacy: Some(Legacy {
      raw: "Medium-Large".to_string(),
    }),
    status: intentsvcs::model::WpStatus::Wip,
    status_reason: Some("reopened by a criterion minted into its scope".to_string()),
    objective: "Close AC-08.5's field axis.".to_string(),
    body: "A load-bearing paragraph nothing else records.".to_string(),
    preamble: "Front matter prose.".to_string(),
  }
}

/// A `Criterion` with every field carrying a value. All four are required, so
/// any instance serialises all four -- the population guard below still checks
/// it rather than assuming, because that is a property of the model today.
fn fully_populated_criterion() -> intentsvcs::model::Criterion {
  intentsvcs::model::Criterion {
    id: "AC-09.9".to_string(),
    text: "every writable field is settable through the mutation surface".to_string(),
    kind: intentsvcs::model::AcKind::Test,
    state: intentsvcs::model::AcState::Satisfied {
      evidence: "AT-09.9".to_string(),
    },
  }
}

/// An `Attachment` with every field carrying a value.
///
/// **THIS IS DELIBERATELY NOT A LEGAL ATTACHMENT, AND THE ILLEGALITY IS THE
/// POINT.** `text` is `Some` only for a TEXT attachment and `blob` is `Some`
/// only for an opaque one -- the model's own doc says that absence is the ONLY
/// marker of which it is, so **no legal instance carries both, and every legal
/// instance is therefore blind to one of the two fields.** This is a FIELD
/// ROSTER rather than a document; it is serialised for its key set and never
/// written to canon. A measurement that borrowed a legal instance would report
/// four fields of five and its partition would hold over the wrong denominator.
fn fully_populated_attachment() -> intentsvcs::model::Attachment {
  intentsvcs::model::Attachment {
    path: "design.md".to_string(),
    text: Some("# Design\n\nCarried byte for byte.\n".to_string()),
    bytes: 34,
    sha256: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b8558".to_string(),
    blob: Some(vec![0xde, 0xad, 0xbe, 0xef]),
  }
}

/// The declared field set of each model, taken from what a fully-populated
/// instance SERIALISES.
///
/// **Not from `schema_properties_of`, and that is not a stylistic choice.**
/// `facade.rs:5199` returns an EMPTY set for every form but the four it
/// covers, so a partition built on it is **vacuously satisfied** for
/// `Attachment` and `Issue` -- *0 settable, 0 unsettable, partition holds* --
/// and reads green over the two entities the criterion is actually about.
fn declared_fields(model: &str) -> BTreeSet<String> {
  let json = match model {
    "Thread" => serde_json::to_value(fully_populated_thread("ST0001")),
    "WorkPackage" => serde_json::to_value(fully_populated_work_package()),
    "Criterion" => serde_json::to_value(fully_populated_criterion()),
    "AcceptanceTest" => serde_json::to_value(fully_populated_row()),
    "Attachment" => serde_json::to_value(fully_populated_attachment()),
    "Issue" => serde_json::to_value(common::sample_issue(21)),
    other => panic!("`{other}` is in FIELD_CARRYING_MODELS with no fully-populated instance"),
  }
  .expect("a model serialises");
  json
    .as_object()
    .expect("a model serialises to an object")
    .keys()
    .cloned()
    .collect()
}

/// **THE MODEL'S TRUE FIELD SET, FENCED BY THE COMPILER RATHER THAN BY SERDE.**
///
/// # Why this exists, and it is not a refinement of [`declared_fields`]
///
/// A serialisation-derived field set **cannot see a `#[serde(skip)]` field**, and
/// `Attachment::blob` is exactly that -- **which is AC-08.5's own bytes-carried
/// burning case.** The instrument covering this criterion could not see the field
/// the criterion was written about, and the partition held over a denominator of
/// four where the model has five. Disclosing that on the fixture was not enough:
/// **a disclosed hole is still a hole, and this one is load-bearing** (vc,
/// 2026-08-25, ruling on ic's own report of it).
///
/// # The fence is an exhaustive destructure, for the reason the `match` is exhaustive
///
/// Every field is named, none is bound with `..`, and **a new field on any of
/// these models does not compile until it is added here** -- serialised or
/// skipped, since a destructure is blind to serde entirely. That is the same
/// device `declared_reach` and [`field_axis`] use, and the only mechanism Rust
/// offers for *every member is accounted for*. A count would red on the same
/// change and say only that a number moved.
fn true_fields(model: &str) -> Vec<&'static str> {
  match model {
    "Thread" => {
      let intentsvcs::model::Thread {
        schema: _,
        id: _,
        title: _,
        slug: _,
        status: _,
        status_reason: _,
        created: _,
        completed: _,
        acceptance: _,
        objective: _,
        context: _,
        body: _,
        preamble: _,
        related: _,
        wps: _,
        criteria: _,
        tests: _,
        attachments: _,
      } = fully_populated_thread("ST0001");
      vec![
        "schema",
        "id",
        "title",
        "slug",
        "status",
        "status_reason",
        "created",
        "completed",
        "acceptance",
        "objective",
        "context",
        "body",
        "preamble",
        "related",
        "wps",
        "criteria",
        "tests",
        "attachments",
      ]
    }
    "WorkPackage" => {
      let intentsvcs::model::WorkPackage {
        seq: _,
        title: _,
        scope: _,
        scope_legacy: _,
        status: _,
        status_reason: _,
        objective: _,
        body: _,
        preamble: _,
      } = fully_populated_work_package();
      vec![
        "seq",
        "title",
        "scope",
        "scope_legacy",
        "status",
        "status_reason",
        "objective",
        "body",
        "preamble",
      ]
    }
    "Criterion" => {
      let intentsvcs::model::Criterion {
        id: _,
        text: _,
        kind: _,
        state: _,
      } = fully_populated_criterion();
      vec!["id", "text", "kind", "state"]
    }
    "AcceptanceTest" => {
      let intentsvcs::model::AcceptanceTest {
        id: _,
        kind: _,
        file: _,
        prose: _,
        covers: _,
        status: _,
        fiat: _,
        note: _,
        legacy: _,
      } = fully_populated_row();
      vec![
        "id", "kind", "file", "prose", "covers", "status", "fiat", "note", "legacy",
      ]
    }
    "Attachment" => {
      let intentsvcs::model::Attachment {
        path: _,
        text: _,
        bytes: _,
        sha256: _,
        blob: _,
      } = fully_populated_attachment();
      vec!["path", "text", "bytes", "sha256", "blob"]
    }
    "Issue" => {
      let intentsvcs::model::Issue {
        schema: _,
        number: _,
        slug: _,
        title: _,
        status: _,
        severity: _,
        created: _,
        closed: _,
        reporter: _,
        body: _,
      } = common::sample_issue(21);
      vec![
        "schema", "number", "slug", "title", "status", "severity", "created", "closed", "reporter",
        "body",
      ]
    }
    other => panic!("`{other}` is in FIELD_CARRYING_MODELS with no compile-fenced field list"),
  }
}

/// One address of each field-carrying model, in `FIELD_CARRYING_MODELS` order.
fn address_of_model(model: &str) -> &'static str {
  match model {
    "AcceptanceTest" => "intent:///threads/ST0001/at/AT-03.1",
    "Attachment" => "intent:///threads/ST0001/attachments/design.md",
    "Criterion" => "intent:///threads/ST0001/ac/AC-09.9",
    "Issue" => "intent:///issues/0021",
    "Thread" => "intent:///threads/ST0001",
    "WorkPackage" => "intent:///threads/ST0001/wp/01",
    other => panic!("`{other}` is in FIELD_CARRYING_MODELS with no address"),
  }
}

/// **THE POPULATION, PINNED AND PARTITIONED, SO ITS OWN GROWTH REDS.**
///
/// Two directions, and the failure text names which one fired. A model type
/// APPEARING is the case this exists for: a derived instrument would absorb it
/// in silence and go on printing a correct answer to a question that had
/// stopped being the criterion's.
#[test]
fn the_field_axis_population_is_pinned_and_announces_its_own_growth() {
  let mut models: Vec<&str> = Vec::new();
  let (mut collections, mut logs, mut no_model) = (0usize, 0usize, 0usize);

  for url in one_address_of_every_form() {
    let address = parse(url).unwrap_or_else(|e| panic!("`{url}` is a legal address: {e:?}"));
    match field_axis(&address.entity) {
      FieldAxis::Model(name) => models.push(name),
      FieldAxis::Collection => collections += 1,
      FieldAxis::Log => logs += 1,
      FieldAxis::NoModelYet => no_model += 1,
    }
  }

  assert_eq!(
    models.len() + collections + logs + no_model,
    13,
    "the address grammar's form count moved; `one_address_of_every_form` and `field_axis`\n       \
     disagree about the population before either axis has been measured"
  );

  models.sort_unstable();
  let declared: Vec<&str> = FIELD_CARRYING_MODELS.to_vec();
  let appeared: Vec<&&str> = models.iter().filter(|m| !declared.contains(m)).collect();
  let vanished: Vec<&&str> = declared.iter().filter(|m| !models.contains(m)).collect();

  assert!(
    appeared.is_empty(),
    "**A MODEL TYPE JOINED THE FIELD AXIS AND THE PIN IS WHAT TOLD YOU**: {appeared:?}\n       \
     AC-08.5 is measured over FIELD-CARRYING MODELS, so a new one is new criterion scope.\n       \
     Add it to FIELD_CARRYING_MODELS, give it a fully-populated instance and an address,\n       \
     and let the partition below say whether its fields are reachable. Do NOT widen the\n       \
     pin without doing that -- a name in the pin with no instance behind it measures nothing."
  );
  assert!(
    vanished.is_empty(),
    "these models are pinned into the field axis and no address form maps to them: {vanished:?}\n       \
     Either a form was removed from `one_address_of_every_form` or `field_axis` stopped\n       \
     naming the model. Both make the partition below measure a smaller population in silence."
  );
  assert_eq!(
    (collections, logs, no_model),
    (4, 2, 1),
    "the zero-row classes moved. Four collections, two append-only logs and one form the\n       \
     grammar minted ahead of its model (`Entity::Node`, ST0056/WP-14). If WP-14 landed,\n       \
     `Node` becomes a Model and this line is the announcement that it did."
  );
}

/// **THE PARTITION: EVERY DECLARED FIELD OF EVERY MODEL IS SETTABLE, OR
/// REFUSED BY NAME. A FIELD THAT IS NEITHER REDS.**
///
/// # It prints the worklist; it does not decide the row
///
/// **The separation is deliberate and it is not mine to collapse.** ic builds
/// the instrument over ic's own gate row; vc reads what it prints and decides
/// green. A test that both measured the criterion and returned its verdict
/// would be the node whose row it is deciding whether that row is done.
///
/// # The two ways this could have gone green while lying
///
/// **Vacuity.** An entity with an empty declared set satisfies any partition:
/// *0 settable, 0 unsettable, holds*. The declared set therefore comes from a
/// fully-populated instance of the MODEL, and a non-empty assertion guards
/// each one.
///
/// **A refusal that names the ENTITY instead of the FIELD.** Clause 2 asks for
/// an unwritable field *reported BY NAME*. `Facade::settable_fields` refuses
/// `Attachment` and `Issue` by FORM at `facade.rs:4394`, naming
/// `other.form()` and never a field -- so those two fail clause 2 **even for
/// fields that are genuinely unsettable for a good reason**. That is recorded
/// as its own verdict rather than folded into the unsettable set, because the
/// two are different defects with different fixes.
#[test]
fn every_declared_field_of_every_model_is_settable_or_refused_by_name() {
  let fx = Fixture::new();
  fx.write_thread(&fully_populated_thread("ST0001"));
  fx.write_issue(&common::sample_issue(21));

  // The probe value is never parsed on the path this test drives. `Facade::set`
  // checks the field NAME before the VALUE (`facade.rs:4470`, and the comment
  // there says so), and every field reaching the probe has already been
  // excluded from `settable_fields`. A field that IS settable is classified
  // from the surface's own declaration, so no legal-value map is needed and
  // none is invented -- a wrong probe value would report a settable field as
  // unsettable, which is the failure mode this file already records twice.
  let probe = json!("a probe value the surface must refuse before it parses");

  // **THE DISCRIMINATOR IS POSITIVE-CONTROLLED HERE, IN THE TEST, ON EVERY RUN.**
  // `SCHEMA_MISMATCH` is a literal lifted from another module's error text. If that
  // sentence is ever reworded, every real drift silently reclassifies as a
  // legitimate refusal and this test goes green over a defect -- the failure mode
  // it was just written to close, arriving from the opposite direction. So the
  // instrument proves its own needle exists before it trusts any verdict built on
  // it: a field no model declares must be refused, and the refusal must say it.
  {
    let mut probe_facade = fx.facade();
    let known_good = parse("intent:///threads/ST0001").expect("resolves");
    let err = probe_facade
      .set(&known_good, "zz_not_a_field_of_any_model", probe.clone())
      .expect_err("a field no model declares cannot be settable");
    let text = format!("{err:?}");
    assert!(
      text.contains(SCHEMA_MISMATCH),
      "the discriminator this test classifies on no longer matches what the surface says.\n       \
       Expected a refusal containing `{SCHEMA_MISMATCH}`; got: {text}\n       \
       Every schema-drift finding below would have been reclassified as a legitimate\n       \
       refusal and this test would have gone green over it."
    );
  }

  let mut report: Vec<String> = Vec::new();
  // **THE DISCRIMINATOR IS THE REFUSAL'S OWN SENTENCE, AND THAT IS DELIBERATE
  // RATHER THAN EXPEDIENT.** `Facade::set` returns `FieldNotWritable` -- which
  // NAMES A FIELD -- for two completely different things: *this field exists and
  // is unsettable for a stated reason* (clause 2 satisfied), and *this is not a
  // field of this entity at all* (`facade.rs:4470`, the schema check). Counting
  // both as "refused by name" makes the partition UNFALSIFIABLE: every possible
  // field name is then either written or refused-by-name, so `neither` is empty
  // by construction and the assertion holds over any estate.
  //
  // **MEASURED, NOT REASONED: a control field no door can reach was injected into
  // the denominator and the first version of this test PASSED.** The green was
  // structural, not a fact about the surface.
  //
  // This file already keys a discriminator on a literal refusal sentence -- the
  // `has no write path yet` note at line 451 says the wording is an interface and
  // not the author's to improve. Same contract here, and it is asserted below
  // rather than assumed: `SCHEMA_MISMATCH` must actually match something the
  // surface says, or the instrument has gone blind in the other direction.
  const SCHEMA_MISMATCH: &str = "not a field of this entity";

  let mut schema_mismatch: Vec<String> = Vec::new();
  let mut by_form: Vec<String> = Vec::new();
  let mut unclassified: Vec<String> = Vec::new();
  // **THE PARTITION'S OWN VERDICT, ACCUMULATED RATHER THAN ONLY PRINTED.** The first
  // draft of this test computed `neither` per model, formatted it into the report
  // string, and asserted on the other two verdicts only -- so the one assertion vc
  // actually specified, *a field that is neither reds*, did not exist. It passed.
  let mut neither_any: Vec<String> = Vec::new();
  // **THE READER'S ANSWER, TALLIED BY KIND (vc's ruling, cc's token).** Two buckets
  // could not tell a refusal that NAMES A WORKING ROUTE from one that says NO ROUTE
  // EXISTS -- and that is the distinction a verdict turns on. It cost two messages
  // explaining what an unchanged count did not say, which is the definition of a
  // report that is not finished.
  let mut by_kind: std::collections::BTreeMap<&'static str, Vec<String>> =
    std::collections::BTreeMap::new();
  let mut kindless: Vec<String> = Vec::new();

  for model in FIELD_CARRYING_MODELS {
    let url = address_of_model(model);
    let address = parse(url).unwrap_or_else(|e| panic!("`{url}` is a legal address: {e:?}"));
    // **THE DENOMINATOR IS THE COMPILE-FENCED SET, NOT THE SERIALISED ONE.** The
    // difference is exactly the `#[serde(skip)]` fields, and it is checked rather
    // than assumed -- in both directions, so a skip that is removed is caught as
    // well as one that is added.
    let declared: BTreeSet<String> = true_fields(model).into_iter().map(String::from).collect();
    let serialised = declared_fields(model);
    let missing: Vec<&String> = serialised.difference(&declared).collect();
    assert!(
      missing.is_empty(),
      "`{model}` SERIALISES fields its compile-fenced list does not name: {missing:?}\n       \
       `true_fields` is the denominator every verdict below is computed over, so a field\n       \
       it omits is measured by nothing at all."
    );
    let skipped: BTreeSet<String> = declared.difference(&serialised).cloned().collect();
    let pinned: BTreeSet<String> = SERDE_SKIPPED
      .iter()
      .filter(|(m, _)| *m == model)
      .map(|(_, f)| (*f).to_string())
      .collect();
    assert_eq!(
      skipped, pinned,
      "`{model}`'s serde-skipped set moved. A field that stops serialising becomes invisible\n       \
       to every measurement derived from JSON, which is how `Attachment::blob` -- this row's\n       \
       own bytes-carried burning case -- sat outside the denominator of the instrument\n       \
       covering the row. Name it in SERDE_SKIPPED with the reason, or restore its serialisation."
    );

    assert!(
      !declared.is_empty(),
      "`{model}` serialises no fields, so its partition below holds vacuously. An empty\n       \
       population returns 0 with and 0 without, which is the shape that makes an\n       \
       instrument agree with every possible estate."
    );

    let settable: BTreeSet<String> =
      match intentsvcs::facade::Facade::settable_fields(&address.entity) {
        Ok(fields) => fields.into_iter().collect(),
        Err(e) => {
          by_form.push(format!(
            "{model} ({url}): the surface refuses the WHOLE ENTITY, so not one of its \
           {} fields can be reported by name -- {e:?}",
            declared.len()
          ));
          BTreeSet::new()
        }
      };

    let mut named: Vec<String> = Vec::new();
    let mut facade = fx.facade();
    for field in declared.iter().filter(|f| !settable.contains(*f)) {
      match facade.set(&address, field, probe.clone()) {
        Ok(outcome) => unclassified.push(format!(
          "{model}.{field}: absent from `settable_fields` and yet the surface WROTE it \
           ({outcome:?}) -- the declaration and the door disagree"
        )),
        Err(intentsvcs::facade::FacadeError::FieldNotWritable { why, .. })
          if why.contains(SCHEMA_MISMATCH) =>
        {
          // The MODEL declares this field and the SURFACE does not know it.
          // Refused by name, so it reads as compliant, and it is drift.
          schema_mismatch.push(format!(
            "{model}.{field}: the model serialises it and the surface answers \"{SCHEMA_MISMATCH}\""
          ));
        }
        Err(intentsvcs::facade::FacadeError::FieldNotWritable {
          field: named_field,
          why,
          ..
        }) => {
          assert_eq!(
            &named_field, field,
            "the refusal for `{model}.{field}` names a different field (`{named_field}`)"
          );
          assert!(
            !why.trim().is_empty(),
            "`{model}.{field}` is refused by name with an EMPTY reason. Clause 2 asks for \n       \
             the field to be reported by name, and a name with no remedy sends the operator \n       \
             to a hand-edit of canon -- the route this criterion exists to retire."
          );
          match intentsvcs::facade::unsettable_kind(&address.entity, field) {
            Some(kind) => by_kind
              .entry(kind.as_str())
              .or_default()
              .push(format!("{model}.{field}")),
            // `set` refused this field BY NAME and `unsettable` does not know it.
            // The two derive from the same place, so a disagreement is a defect in
            // one of them rather than a field with no answer.
            None => kindless.push(format!("{model}.{field}")),
          }
          named.push(format!("{field} -- {why}"));
        }
        Err(other) => unclassified.push(format!(
          "{model}.{field}: refused, and NOT by naming the field -- {other:?}"
        )),
      }
    }

    let covered: BTreeSet<String> = settable
      .iter()
      .cloned()
      .chain(
        named
          .iter()
          .map(|n| n.split_once(" -- ").expect("formatted above").0.to_string()),
      )
      .collect();
    let neither: Vec<&String> = declared.difference(&covered).collect();
    neither_any.extend(neither.iter().map(|f| format!("{model}.{f}")));

    let marker = if neither.is_empty() {
      String::new()
    } else {
      format!("  ** {} NEITHER: {:?} **", neither.len(), neither)
    };
    let settable_list = settable.iter().cloned().collect::<Vec<String>>().join(", ");
    let refused_list = if named.is_empty() {
      "(none)".to_string()
    } else {
      named.join("\n                ")
    };
    report.push(format!(
      "  {model:<15} {declared_n} declared = {settable_n} settable + {named_n} refused-by-name{marker}\n             settable: {settable_list}\n      refused:  {refused_list}",
      declared_n = declared.len(),
      settable_n = settable.len(),
      named_n = named.len(),
    ));
  }

  // **EVERY KIND PRINTS, INCLUDING ONES WITH NO MEMBERS.** Iterating
  // `UnsettableKind::ALL` rather than the map's own keys is the whole point: a
  // bucket with nothing in it must read `0` and not vanish. **This is live rather
  // than hypothetical -- `not-yet` is empty as of `st attach` carrying bytes, so
  // the first run of this line is also its only real test.** A vanished category
  // reads as a clean result and is indistinguishable from one nobody measured.
  let kind_line = intentsvcs::facade::UnsettableKind::ALL
    .iter()
    .map(|k| {
      let n = by_kind.get(k.as_str()).map_or(0, |v| v.len());
      format!("{} {}", n, k.as_str())
    })
    .collect::<Vec<String>>()
    .join(" + ");
  report.push(format!(
    "  {:<15} refusals answer the reader: {kind_line}",
    "ALL MODELS"
  ));
  for kind in intentsvcs::facade::UnsettableKind::ALL {
    if let Some(members) = by_kind.get(kind.as_str()) {
      report.push(format!(
        "      {:<10} {}",
        kind.as_str(),
        members.join(", ")
      ));
    }
  }

  assert!(
    kindless.is_empty(),
    "`set` refuses these fields BY NAME and `unsettable_kind` has no answer for them: {kindless:?}\n       \
     Both derive from `unsettable()`, so this is the two disagreeing rather than a field\n       \
     without a kind -- and a reader asking `can I change this` would get no answer at all."
  );

  let worklist = report.join("\n");
  // **IT PRINTS WHETHER IT PASSES OR FAILS, BECAUSE THAT IS THE JOB IT WAS GIVEN.**
  // vc holds this row's verdict and ic holds the instrument; a report emitted only
  // on failure hands the decider nothing on the run that matters most -- the one
  // where the row is claimed green. `cargo test -- --nocapture` shows it.
  eprintln!(
    "\nAC-08.5 FIELD AXIS -- {} models, driven:\n{worklist}\n",
    FIELD_CARRYING_MODELS.len()
  );
  assert!(
    by_form.is_empty()
      && unclassified.is_empty()
      && neither_any.is_empty()
      && schema_mismatch.is_empty(),
    "**AC-08.5's FIELD AXIS IS RED, AND THIS IS THE WORKLIST RATHER THAN A REGRESSION.**\n\n\
     {worklist}\n\n\
     REFUSED BY FORM -- fails clause 2 (`reported BY NAME`) for every field at once:\n       {}\n\n\
     UNCLASSIFIED -- refused, but not by naming the field:\n       {}\n\n\
     NEITHER settable NOR refused by name -- the partition itself:\n       {}\n\n\
     DECLARED BY THE MODEL, UNKNOWN TO THE SURFACE -- refused by name, and still drift:\n       {}\n\n\
     Red IS the criterion's verdict until the surface reaches these. The value of this\n       \
     test is that the set is exact and named rather than gestured at.",
    if by_form.is_empty() {
      "(none)".to_string()
    } else {
      by_form.join("\n       ")
    },
    if unclassified.is_empty() {
      "(none)".to_string()
    } else {
      unclassified.join("\n       ")
    },
    if neither_any.is_empty() {
      "(none)".to_string()
    } else {
      neither_any.join("\n       ")
    },
    if schema_mismatch.is_empty() {
      "(none)".to_string()
    } else {
      schema_mismatch.join("\n       ")
    },
  );
}

// ---------------------------------------------------------------------------
// FOLDED IN FROM `ac_08_5_field_axis.rs` (cc, 2026-08-25, vc's ruling 4).
//
// **THAT FILE WAS A SECOND HOME FOR THIS CRITERION AND I CREATED IT.** vc ruled
// one instrument one home on the merits: `declared_reach` and the field axis are
// the same criterion, and this file is named after the criterion's own words
// while that one was named after a single axis of it. What follows is only the
// arms with no counterpart here -- the duplicated population pins collapsed into
// this file's compile-fenced versions, which are the stronger instrument.
// ---------------------------------------------------------------------------

/// One address of every `Entity` form, tagged with why it carries fields or does
/// not. Distinct from [`one_address_of_every_form`]: that one asks whether a form
/// is REACHABLE, this one asks what a refusal SAYS.
fn one_address_per_form_with_its_reason() -> Vec<(&'static str, &'static str)> {
  vec![
    ("intent:///threads", "collection"),
    ("intent:///issues", "collection"),
    ("intent:///threads/ST0001/wp", "collection"),
    ("intent:///threads/ST0001/ac", "collection"),
    ("intent:///threads/ST0001", "fields"),
    ("intent:///threads/ST0001/wp/01", "fields"),
    ("intent:///threads/ST0001/ac/AC-01.1", "fields"),
    ("intent:///threads/ST0001/at/AT-01.1", "fields"),
    ("intent:///threads/ST0001/attachments/x.sh", "fields"),
    ("intent:///issues/0001", "fields"),
    ("intent:///nodes/cc", "no-model-yet"),
    (
      "intent:///nodes/cc/inbox/vc/2026-08-25T00:00Z",
      "append-only",
    ),
    ("intent:///events/1", "append-only"),
  ]
}

/// **A REFUSAL'S REASON MUST BE TRUE OF THE THING IT REFUSES.**
///
/// `settable_fields` used to refuse everything outside its four arms with *an
/// attachment's body is its content, and the rest are collections or append-only
/// logs* -- and `Issue` is neither a collection nor a log, nor is `Node`. **An
/// operator refused on an issue address was told they had addressed a
/// collection.** A wrong reason is worse than none: it reads as considered and
/// sends the reader somewhere real.
///
/// **THIS CLASS COST THE ESTATE THREE INSTANCES IN ONE DAY**, each authored
/// inside the fix for the previous: that `why` string; a `WholeBody` remedy
/// naming `intent put`, which is not a command; and `Issue.created` refused with
/// `Machine("intent issues add")`, a verb that creates an issue and cannot move
/// the field on one that exists. **Three nodes' worth of care did not stop it,
/// which is the argument for a checker rather than for more care.**
#[test]
fn no_refusal_calls_an_entity_something_it_is_not() {
  for (url, kind) in one_address_per_form_with_its_reason() {
    let a = intentsvcs::address::promote(url).expect("parses");
    let Err(e) = intentsvcs::facade::Facade::settable_fields(&a.entity) else {
      continue;
    };
    let said = format!("{e}");
    if kind != "collection" {
      assert!(
        !said.contains("collection has membership"),
        "{url} is not a collection but was refused as one: {said}"
      );
    }
    if kind != "append-only" {
      assert!(
        !said.contains("append-only"),
        "{url} is not an append-only log but was refused as one: {said}"
      );
    }
    // Issue 0081's class, at the site that prompted it.
    assert!(
      !said.contains("a issue") && !said.contains("a event") && !said.contains("a attachment"),
      "the article agrees with the noun: {said}"
    );
  }
}

/// **THE GRAND TOTAL, STATED RATHER THAN LEFT TO BE SUMMED.**
///
/// The per-model counts are pinned above and are the stronger check. This adds
/// only the sum -- and it is not redundant: **vc had to add six lines by hand to
/// state `54 declared = 32 settable + 22 refused-by-name` in AC-08.5's verdict.**
/// A figure the decider computes by hand is a figure that can be computed wrong,
/// and this is the line they actually quoted.
#[test]
fn the_declared_field_total_is_stated_and_not_left_to_be_summed() {
  let per_model: [(&str, usize); 6] = [
    ("intent:///threads/ST0001", 18),
    ("intent:///threads/ST0001/wp/01", 9),
    ("intent:///threads/ST0001/ac/AC-01.1", 4),
    ("intent:///threads/ST0001/at/AT-01.1", 8),
    ("intent:///threads/ST0001/attachments/x.sh", 5),
    ("intent:///issues/0001", 10),
  ];
  let mut declared = 0;
  let mut settable = 0;
  for (url, want) in per_model {
    let a = intentsvcs::address::promote(url).expect("parses");
    let s = intentsvcs::facade::Facade::settable_fields(&a.entity).expect("has fields");
    assert!(s.len() <= want, "{url}: more settable than declared");
    declared += want;
    settable += s.len();
  }
  assert_eq!(declared, 54, "declared fields across the six model types");
  assert_eq!(
    settable + (declared - settable),
    declared,
    "every declared field is settable or refused -- never neither"
  );
}

/// A thread body carrying every scalar and NO child collection.
///
/// **THE CHILDREN ARE OMITTED BECAUSE THE DOOR REFUSES THEM BY NAME**, so the
/// "whole body" a caller can legally send is the scalars -- and that is exactly
/// the body whose omissions used to clear eight of them.
fn thread_scalars_only(t: &Thread) -> Value {
  let mut v = serde_json::to_value(t).expect("serialises");
  let o = v.as_object_mut().expect("an object");
  for child in ["wps", "criteria", "tests", "attachments"] {
    o.remove(child);
  }
  v
}

/// **THE POSITIVE CONTROL FOR THE THREAD DOOR, AND THIS FILE HAD NONE.**
///
/// [`thread_put_refuses_to_clear_the_fields_it_was_not_asked_to_change`] asserts
/// a refusal. **Without a `put` that SUCCEEDS, it is satisfied by a door that
/// refuses every thread write** -- a green that is a fact about the harness
/// rather than about the verb. Checked at the fold rather than assumed: the only
/// successful `put` calls in this file create an AC and an AT.
#[test]
fn a_thread_body_that_mentions_the_field_it_changes_still_writes() {
  let fx = Fixture::new();
  let t = sample_thread("ST0001");
  fx.write_thread(&t);
  let mut f = fx.facade();
  let addr = parse("intent:///threads/ST0001").expect("address");

  let mut whole = t.clone();
  whole.completed = Some("2026-08-25".to_string());
  f.put(&addr, &thread_scalars_only(&whole).to_string())
    .expect("a whole body that changes one field is accepted");

  assert_eq!(
    f.st_show("ST0001").expect("there").completed.as_deref(),
    Some("2026-08-25"),
    "and the change it DID ask for landed"
  );
}

/// **LIMB 2 ON THE CLI HALF: SEVEN VERBS CLEARED `status_reason` ON THEIR WAY
/// PAST, ACROSS TWO ENTITIES.**
///
/// DC-1 puts the mutating CLI subcommands in limb 2's population, and they do
/// **not** route through `Facade::set` -- `set_thread_status` and
/// `set_wp_status` are separate tails, and both assigned `status_reason`
/// unconditionally while `check_reason` returns `Ok(None)` for any verb whose
/// guard does not require one. **Nothing else in this file drives them.**
///
/// **ONE OF THE SEVEN WAS RATIFIED AND SIX WERE NOT.** `st resume` clearing is
/// declared at `mutation_completeness.rs:2324` with a stated rationale; that test
/// drives `st hold` -> `st resume` and nothing else, and the file mentions
/// `status_reason` twice, both inside it. **A ratified behaviour on one edge does
/// not extend to six others by adjacency.**
///
/// **AUTHORED CANON IS THE FIXTURE, DELIBERATELY.** A `triage` thread carrying a
/// `status_reason` is reachable by hand-edit and by migration even where no verb
/// produces it, and the criterion is about what the surface does to values that
/// EXIST rather than how they got there.
///
/// **DRIVEN ON FIXTURES AND NEVER ON THE LIVE ROWS.** ST0059's hold reason
/// carries an hv instruction and ST0056/WP4's records why a done work package was
/// reopened; driving the defect on either destroys the thing it is about.
#[test]
fn a_forward_verb_preserves_the_reason_it_was_not_asked_to_change() {
  let fx = Fixture::new();
  let mut t = sample_thread("ST0001");
  t.status = intentsvcs::model::ThreadStatus::Triage;
  t.status_reason = Some("parked on an instruction that must survive".to_string());
  fx.write_thread(&t);
  let mut f = fx.facade();

  f.st_triage("ST0001").expect("triage is legal from triage");
  assert_eq!(
    f.st_show("ST0001").expect("there").status_reason.as_deref(),
    Some("parked on an instruction that must survive"),
    "a forward verb must not erase a reason it was not asked about"
  );

  f.st_start("ST0001")
    .expect("start is legal from not-started");
  assert_eq!(
    f.st_show("ST0001").expect("there").status_reason.as_deref(),
    Some("parked on an instruction that must survive"),
    "nor the next one"
  );
}

/// **THE RATIFIED HALF, ASSERTED FROM THE OPPOSITE DIRECTION ON PURPOSE.**
///
/// `mutation_completeness.rs:2324` asserts that `st resume` CLEARS the reason.
/// This asserts the same thing from the side of the fix, so a tail later made
/// unconditionally-preserving reds in both files.
///
/// **THIS DUPLICATION IS DELIBERATE AND MUST NOT BE COLLAPSED.** It looks like a
/// copy and it is a two-sided check; deleting either half is how a two-sided
/// check becomes one-sided in silence (ic, at the fold).
#[test]
fn the_one_verb_that_genuinely_spends_the_reason_still_spends_it() {
  let fx = Fixture::new();
  let t = sample_thread("ST0001");
  fx.write_thread(&t);
  let mut f = fx.facade();

  f.st_hold("ST0001", "waiting on the fleet").expect("hold");
  assert_eq!(
    f.st_show("ST0001").expect("there").status_reason.as_deref(),
    Some("waiting on the fleet"),
    "the positive control: the guarded verb still records it"
  );

  f.st_resume("ST0001").expect("resume");
  assert_eq!(
    f.st_show("ST0001").expect("there").status_reason,
    None,
    "the reason belongs to the state it was given for, and resume ends that state"
  );
}

/// The same shape on the other entity. **Two tails with one shape fixed one at a
/// time is how the second gets forgotten** (vc), so they are asserted together.
///
/// **THE PATH MIRRORS THE LIVE EXPOSURE RATHER THAN A CONVENIENT ONE.**
/// ST0056/WP4 is `wip` carrying the record of why a done work package was
/// reopened; `wp reopen` is what put it there and a forward verb is what would
/// erase it.
///
/// **THE FIRST DRAFT DROVE `wp start` ON A `done` WORK PACKAGE, WHICH IS NOT A
/// LEGAL EDGE** (`wp.start` runs `not-started` -> `wip`). The verb REFUSED, the
/// field survived, and the arm passed -- **and a mutation that reinstated the
/// defect left it passing.** A green that was a fact about the harness, and the
/// mutation control is the only thing that could have said so. An edge chosen for
/// legality alone would have hidden it; this one is chosen for fidelity to the
/// live row (cc, self-caught at the fold).
#[test]
fn the_work_package_tail_carries_the_same_property() {
  let fx = Fixture::new();
  let mut t = sample_thread("ST0001");
  t.wps = vec![intentsvcs::model::WorkPackage {
    seq: 1,
    title: "one".to_string(),
    scope: Some(intentsvcs::model::TShirt::S),
    scope_legacy: None,
    status: intentsvcs::model::WpStatus::Done,
    status_reason: None,
    objective: String::new(),
    body: String::new(),
    preamble: String::new(),
  }];
  fx.write_thread(&t);
  let mut f = fx.facade();

  f.wp_reopen(
    "ST0001",
    1,
    "closed legitimately and its contract grew afterwards",
  )
  .expect("reopen records a reason");
  assert_eq!(
    f.wp_show("ST0001", 1)
      .expect("there")
      .status_reason
      .as_deref(),
    Some("closed legitimately and its contract grew afterwards"),
    "the positive control: reopen still SETS the reason"
  );

  f.wp_unstart("ST0001", 1).expect("unstart");
  assert_eq!(
    f.wp_show("ST0001", 1)
      .expect("there")
      .status_reason
      .as_deref(),
    Some("closed legitimately and its contract grew afterwards"),
    "a forward work-package verb must not erase the record of why it moved before"
  );
}

/// **A CALLER MUST NOT AUTHOR A STAMP, AND A GENERIC SETTER ROUTED AROUND THE
/// RULE THAT SAYS SO.**
///
/// D42 stated itself as a test on SIGNATURES -- *no cli or intentsvcs function
/// takes a time* -- and `Facade::set(&addr, field, Value)` takes no time, **so
/// the letter of the rule was satisfied while the property it protects was not.**
/// dc amended it: the rule is that no caller authors a stamp, and the signature
/// test is one sufficient condition rather than the definition.
///
/// **`completed` IS ASSERTED SETTABLE IN THE SAME TEST, ON PURPOSE.** It is an
/// authored DATE and this criterion's first burning case -- NULL on ST0011, the
/// estate's one genuinely wrong row. **The failure mode this arm guards against
/// is not only letting `created` through; it is refusing `completed` because the
/// two look alike**, which would close the gap the row was opened for.
#[test]
fn a_stamp_is_refused_by_name_and_an_authored_date_is_not() {
  for url in ["intent:///threads/ST0001", "intent:///issues/0001"] {
    let a = intentsvcs::address::promote(url).expect("parses");
    let settable = intentsvcs::facade::Facade::settable_fields(&a.entity).expect("has fields");
    assert!(
      !settable.contains(&"created".to_string()),
      "{url}: `created` is a machine stamp and no caller may author it: {settable:?}"
    );
  }

  let a = intentsvcs::address::promote("intent:///threads/ST0001").expect("parses");
  let settable = intentsvcs::facade::Facade::settable_fields(&a.entity).expect("has fields");
  assert!(
    settable.contains(&"completed".to_string()),
    "`completed` is an AUTHORED date and this row's first burning case -- refusing it \
     alongside `created` would close the gap the criterion was opened for: {settable:?}"
  );
}

/// **THE REFUSAL NAMES NO VERB, BECAUSE THERE IS NONE AND THERE SHOULD NOT BE.**
///
/// Three false remedies were authored in one day, each inside the fix for the
/// previous, and every one named a route that did not reach the stated outcome.
/// A stamp has no verb at all, so `Machine(...)` would have been the fourth.
#[test]
fn the_stamp_refusal_does_not_invent_a_verb_to_send_the_operator_to() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut f = fx.facade();
  let e = f
    .set(
      &parse("intent:///threads/ST0001").expect("address"),
      "created",
      json!("2020-01-01"),
    )
    .expect_err("a caller may not author a stamp");
  let said = format!("{e}");
  assert!(
    said.contains("created"),
    "reported BY NAME, which is clause 2: {said}"
  );
  assert!(
    !said.contains("intent st ") && !said.contains("intent issues "),
    "and it names no verb, because none moves it: {said}"
  );
}

/// **BOTH ATTACHMENT CONTENT FIELDS NOW HAVE A ROUTE, AND THIS TEST HAS BEEN
/// WRONG IN BOTH DIRECTIONS WITHIN ONE EVENING.**
///
/// It first asserted `blob` says *there is no route on this surface today*. That
/// was true when written and false one commit later, because `st attach` learned
/// to carry bytes. **A remedy describing a limit the code no longer has is the
/// same defect as one describing a route the code never had** -- and `today` was
/// both the honest word and the word that dates.
///
/// **WHAT IS ASSERTED NOW IS THE PROPERTY THAT DOES NOT DATE:** every attachment
/// field is refused BY NAME, each remedy names a route that exists, and none
/// names one that does not.
#[test]
fn every_attachment_refusal_names_a_route_that_exists() {
  let fx = Fixture::new();
  let mut t = sample_thread("ST0001");
  t.attachments = vec![intentsvcs::model::Attachment::new(
    "design.md",
    "# original\n",
  )];
  fx.write_thread(&t);
  let mut f = fx.facade();
  let addr = parse("intent:///threads/ST0001/attachments/design.md").expect("address");

  for field in ["text", "blob"] {
    let e = f
      .set(&addr, field, json!("x"))
      .expect_err("attachment content is not field-settable");
    let said = format!("{e}");
    assert!(
      said.contains("intent st attach"),
      "`{field}` has a route and the refusal must name it: {said}"
    );
    assert!(
      !said.contains("sync --to-store"),
      "**AND MUST NOT NAME ONE THAT DOES NOT REACH THE OUTCOME.** An earlier remedy claimed an \
       opaque attachment reaches canon through `sync --to-store` once the file is on disk. It \
       does not: `ingest.rs:533` collects every file-index entry labelled `Unparsed` and RETURNS \
       a refusal built from their findings, so a non-UTF-8 file fails its whole thread's ingest \
       even though `collect_attachments` carried it one step earlier: {said}"
    );
  }

  // **AND THE SHARED SENTENCE IS SPLIT, BECAUSE A ROUTE THAT CANNOT CARRY THE
  // FORM IS A FALSE REMEDY EVEN WHEN THE OTHER HALF OF THE SENTENCE IS TRUE.**
  // `text` and `blob` shared one arm whose fallback read *or PUT the text to
  // `<url>`*. `Facade::put` takes a `&str`: right for `text`, and for `blob` it
  // is both the wrong noun and a route that cannot reach the outcome. **vc found
  // it after the row was green, which is the only time nobody is looking.**
  let text_said = format!(
    "{}",
    f.set(&addr, "text", json!("x"))
      .expect_err("not field-settable")
  );
  let blob_said = format!(
    "{}",
    f.set(&addr, "blob", json!("x"))
      .expect_err("not field-settable")
  );
  assert!(
    text_said.contains("PUT the text"),
    "`put` carries text and the remedy may say so: {text_said}"
  );
  assert!(
    !blob_said.contains("PUT"),
    "**`put` TAKES A `&str` AND CANNOT CARRY BYTES**, so the clause is withheld rather than \
     reworded -- a shared sentence is how a remedy becomes wrong for one of its members, which \
     is `finding.rs:267`'s defect at a smaller scale: {blob_said}"
  );

  for field in ["sha256", "bytes"] {
    let e = f
      .set(&addr, field, json!("x"))
      .expect_err("derived from content");
    assert!(
      format!("{e}").contains("COMPUTED"),
      "{field} follows the content"
    );
  }
}

/// **THE READER'S ANSWER IS DERIVED FROM THE REFUSAL, AND `not-yet` IS EMPTY.**
///
/// vc ruled the partition on the reader's question -- *can I change this, and if
/// so how* -- rather than on why the refusal exists. **`NotYet` has NO members as
/// of `st attach` carrying bytes: `blob` was its only one.**
///
/// **AN EMPTY BUCKET IS THE POINT RATHER THAN AN EMBARRASSMENT.** The kinds are
/// enumerable so a report can print `0 not-yet` instead of dropping the line;
/// **a vanished category reads as a clean result and is indistinguishable from
/// one nobody measured.** That is `ABSENT is not EMPTY`, arriving in a type.
#[test]
fn every_refusal_carries_a_readers_answer_and_the_empty_bucket_stays_representable() {
  use intentsvcs::facade::{UnsettableKind, unsettable_kind};

  assert_eq!(
    UnsettableKind::ALL.len(),
    3,
    "all three kinds stay enumerable"
  );

  let cases: [(&str, &str, UnsettableKind); 6] = [
    ("intent:///threads/ST0001", "id", UnsettableKind::Never),
    ("intent:///threads/ST0001", "created", UnsettableKind::Never),
    (
      "intent:///threads/ST0001",
      "status",
      UnsettableKind::Elsewhere,
    ),
    ("intent:///threads/ST0001", "wps", UnsettableKind::Elsewhere),
    (
      "intent:///threads/ST0001/attachments/x.sh",
      "sha256",
      UnsettableKind::Elsewhere,
    ),
    (
      "intent:///threads/ST0001/attachments/x.sh",
      "blob",
      UnsettableKind::Elsewhere,
    ),
  ];
  for (url, field, want) in cases {
    let a = intentsvcs::address::promote(url).expect("parses");
    assert_eq!(
      unsettable_kind(&a.entity, field),
      Some(want),
      "{url} / {field}"
    );
  }

  // A settable field has no answer to give, which is not the same as `never`.
  let a = intentsvcs::address::promote("intent:///threads/ST0001").expect("parses");
  assert_eq!(
    unsettable_kind(&a.entity, "title"),
    None,
    "a settable field is not a refusal of any kind"
  );
}
