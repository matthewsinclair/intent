//! AT-17.2 / AC-17.2 and AT-17.4 / AC-17.4: **the DSL declares layout and
//! widget, never the field set, and the widget vocabulary is closed.**
//!
//! Both criteria are about REFUSALS, so every arm here drives a declaration
//! that trips one. **A refusal asserted from a green declaration is not
//! measured** -- it is the shape `mutation_every_writable_field.rs` documents
//! shipping for months, where two hand-written literals were compared to each
//! other and observed nothing.
//!
//! # AC-17.2 IS HELD BOTH WAYS AND THE TWO HALVES FAIL AT DIFFERENT TIMES
//!
//! - **Forward** -- a form naming a property the face does not carry is
//!   refused at load. This fires the day a model field is RENAMED, loudly, at
//!   startup.
//! - **Converse** -- a writable property on no form is NAMED. This fires the
//!   day a field is ADDED, and **nothing about a form merely missing a row
//!   looks wrong**. Without it a new writable field is unreachable through
//!   every realiser and the only symptom is that nobody ever edits it.
//!
//! The converse is the one that rots, so it is asserted as an EQUALITY against
//! a declared expectation rather than as *something was reported*. A new
//! writable field fails this file.

use intentsvcs::form::{FormError, Loaded, Reach, face_reach};

/// The shipped declaration loads. **The positive control for every refusal
/// below**: a loader that refused everything would pass each negative arm
/// while being useless, and this is what separates the two.
#[test]
fn the_shipped_declaration_loads() {
  let loaded = Loaded::load().expect("the shipped form declaration resolves against the faces");
  assert!(
    !loaded.forms().is_empty(),
    "a declaration that loads and declares no forms would pass every arm in this file vacuously"
  );
}

/// **AC-17.4: the vocabulary is CLOSED, DECLARED IN THE FILE, and exactly the
/// five the criterion names.**
///
/// Asserted against the criterion's own list rather than against whatever the
/// file happens to say, because the file is the thing under test. A sixth
/// widget arriving without a ruling fails here.
#[test]
fn the_widget_vocabulary_is_the_five_the_criterion_names() {
  let loaded = Loaded::load().expect("loads");
  let mut declared: Vec<&str> = loaded.widgets().iter().map(|w| w.value.as_str()).collect();
  declared.sort_unstable();
  assert_eq!(
    declared,
    ["button", "number", "prose", "select", "text"],
    "AC-17.4 names five widgets and the vocabulary is closed -- a change here is a ruling, \
     not an edit"
  );
  assert!(
    loaded.widgets().iter().all(|w| !w.gloss.trim().is_empty()),
    "a vocabulary value with no gloss is a member nobody can check the meaning of"
  );
}

/// **AC-17.4: an unknown widget is REFUSED BY NAME, not skipped.**
///
/// `AC-17.4`'s own words: *a skipped field renders a form that looks complete
/// and is not*, which is the silent-partial class this thread exists to
/// remove. So the assertion is on the NAME in the error, never merely that
/// loading failed.
#[test]
fn an_unknown_widget_is_refused_by_name() {
  let broken = declaration_with_widget("textarea");
  match Loaded::from_str(&broken) {
    Err(FormError::UnknownWidget {
      widget,
      field,
      declared,
      ..
    }) => {
      assert_eq!(
        widget, "textarea",
        "the refusal names the widget it refused"
      );
      assert_eq!(field, "title", "and the field it was given for");
      assert!(
        declared.contains(&"prose".to_string()),
        "and states the vocabulary, so the author can see what was available: {declared:?}"
      );
    }
    other => panic!("an unknown widget must be refused by name, got {other:?}"),
  }

  // **THE POSITIVE CONTROL.** The identical declaration with a DECLARED widget
  // loads. Without this the arm above passes on a loader that refuses every
  // declaration, which is a green that means nothing.
  Loaded::from_str(&declaration_with_widget("prose"))
    .expect("the same declaration with a declared widget loads");
}

/// **AC-17.2 forward: a field naming a property the face does not carry is
/// REFUSED BY NAME at load.**
#[test]
fn a_field_naming_no_property_is_refused_by_name() {
  let broken = declaration_with_field("objectivve");
  match Loaded::from_str(&broken) {
    Err(FormError::NoSuchProperty {
      field,
      entity,
      face,
      available,
    }) => {
      assert_eq!(
        field, "objectivve",
        "the refusal names the field it refused"
      );
      assert_eq!(entity, "thread");
      assert_eq!(
        face, "thread.schema.json",
        "and the face it resolved against"
      );
      assert!(
        available.contains(&"objective".to_string()),
        "and lists what the face DOES carry, so a rename is visible rather than a puzzle: \
         {available:?}"
      );
    }
    other => panic!("a field naming no property must be refused by name, got {other:?}"),
  }

  // **THE POSITIVE CONTROL**, and it is the same one-character difference that
  // makes the negative arm meaningful.
  Loaded::from_str(&declaration_with_field("objective"))
    .expect("the same declaration naming a real property loads");
}

/// **One property, one row.** Two rows rendering one property is not a layout
/// choice: the value appears twice and an edit through one leaves the other
/// stale on screen, which is the divergent-copy shape at render time.
#[test]
fn one_property_cannot_occupy_two_rows() {
  let doubled = declaration_with_fields(&["title", "title"]);
  match Loaded::from_str(&doubled) {
    Err(FormError::DuplicateProperty { field, .. }) => assert_eq!(field, "title"),
    other => panic!("a repeated property must be refused by name, got {other:?}"),
  }
  Loaded::from_str(&declaration_with_fields(&["title", "objective"]))
    .expect("two DIFFERENT properties are the ordinary case");
}

/// A declaration carrying one thread field with the given widget.
fn declaration_with_widget(widget: &str) -> String {
  declaration(&format!(
    r#"{{ "name": "title", "label": "title", "widget": "{widget}", "editable": true }}"#
  ))
}

/// A declaration carrying one thread field with the given property name.
fn declaration_with_field(name: &str) -> String {
  declaration(&format!(
    r#"{{ "name": "{name}", "label": "objective", "widget": "prose", "editable": true }}"#
  ))
}

fn declaration_with_fields(names: &[&str]) -> String {
  let rows: Vec<String> = names
    .iter()
    .map(|n| format!(r#"{{ "name": "{n}", "label": "{n}", "widget": "text", "editable": true }}"#))
    .collect();
  declaration(&rows.join(","))
}

/// A minimal well-formed declaration around the supplied field rows.
///
/// **Built here rather than read from a fixture file** so the broken shapes
/// this file drives cannot be mistaken for something the tool ships.
fn declaration(fields: &str) -> String {
  format!(
    r#"{{
      "about": "test fixture",
      "widgets": [
        {{ "value": "text",   "gloss": "g" }},
        {{ "value": "number", "gloss": "g" }},
        {{ "value": "select", "gloss": "g" }},
        {{ "value": "button", "gloss": "g" }},
        {{ "value": "prose",  "gloss": "g" }}
      ],
      "forms": [
        {{ "entity": "thread", "fields": [{fields}] }}
      ]
    }}"#
  )
}

/// **AC-17.2's CONVERSE, ASSERTED AS AN EQUALITY OVER ALL THREE ENTITIES.**
///
/// The gap set is exactly three properties, each out by a recorded decision:
///
/// - **`thread.body`, `thread.preamble`** -- `tui-design.md` §11, *empty
///   everywhere measured*, recommended out of v1.
/// - **`wp.scope_legacy`** -- the model's own words are *carried, NEVER
///   interpreted*: a v2 size outside the vocabulary, kept verbatim, that
///   nothing reads to answer a question about size. **A form offering to edit
///   it would invite an operator to author into a field whose whole contract
///   is that nobody reads it** -- and a live thread carrying one is itself a
///   defect `doctor` reports, so the route out is `doctor`'s, not a form row.
///
/// **The point is that each omission is NAMED rather than silent**, and it is
/// asserted as `==`, never as `is_empty` or `contains`. A new writable field
/// on any of the three models fails this test, which is the whole reason the
/// converse arm exists: it fires the day a field is ADDED, and nothing about a
/// form merely missing a row looks wrong.
#[test]
fn the_only_changeable_properties_off_a_form_are_the_three_ruled_out() {
  let loaded = Loaded::load().expect("loads");
  let gaps: Vec<String> = loaded
    .changeable_and_not_on_any_form()
    .into_iter()
    .map(|(entity, property)| format!("{entity}.{property}"))
    .collect();
  assert_eq!(
    gaps,
    ["thread.body", "thread.preamble", "wp.scope_legacy"],
    "a changeable property with no row is unreachable through EVERY realiser, and the only \
     symptom is that nobody ever edits it. These three are out by a recorded decision; \
     anything else here is a field that arrived after the forms were written"
  );
}

/// **Every face partitions: each property is on its form or is `Never`.**
///
/// This is what stops the two arms above being satisfiable by a bad
/// declaration. A form that made every row read-only, or that dropped rows
/// wholesale, passes `offers_an_edit_that_cannot_land` trivially and passes
/// the gap arm as soon as the expectation is updated to match. **Asserting the
/// partition over the WHOLE face, for every entity, is the check neither of
/// those can be talked out of.**
///
/// **Driven over all three kinds, and `wp` is the one that matters.**
/// `WorkPackage` is not a schema root -- it lives at `$defs/WorkPackage`
/// inside `thread.schema.json` -- and an earlier loader resolved it to that
/// FILE, handing a work-package form the THREAD's nineteen fields. Six of
/// `WorkPackage`'s nine share a name with a `Thread` property, so nothing
/// refused. **A test over `thread` alone cannot see that.**
#[test]
fn every_property_of_every_face_is_accounted_for() {
  let loaded = Loaded::load().expect("loads");
  let expected: &[(&str, usize, &[&str])] = &[
    (
      // 18 -> 19: `fiat` arrived with ST0066's fiat close (dc, `d4526c1b`).
      // **THIS IS THE ARM THAT CAUGHT IT**, and it caught it the way it was
      // built to: the converse half of AC-17.2 fires the day a field is ADDED,
      // and nothing about a form merely missing a row looks wrong.
      "thread",
      19,
      &["body (NarrowSetter)", "preamble (NarrowSetter)"],
    ),
    // 9 -> 10: `fiat` landed on WorkPackage in the same commit as the
    // Thread one, so both faces moved together and both are re-read here
    // rather than bumped.
    ("wp", 10, &["scope_legacy (NarrowSetter)"]),
    ("issue", 10, &[]),
  ];

  for (kind, size, exceptions) in expected {
    let reach = face_reach(kind).unwrap_or_else(|| panic!("{kind} resolves to a face"));
    assert_eq!(
      reach.len(),
      *size,
      "the {kind} face changed size -- every arm in this file is then measuring a different \
       population than it was written against, which is a re-read rather than a bump"
    );

    let form = loaded
      .form(kind)
      .unwrap_or_else(|| panic!("a {kind} form is declared"));
    let rows: Vec<&str> = form.fields.iter().map(|f| f.name.as_str()).collect();
    let unaccounted: Vec<String> = reach
      .iter()
      .filter(|(property, r)| !rows.contains(&property.as_str()) && *r != Reach::Never)
      .map(|(property, r)| format!("{property} ({r:?})"))
      .collect();
    assert_eq!(
      unaccounted, *exceptions,
      "every {kind} property is either on the form or unchangeable by design -- the listed \
       exceptions are recorded decisions and nothing else may join them silently"
    );
  }
}

/// **The shipped form promises no edit that cannot land -- AND THE CHECKER CAN
/// FIRE.**
///
/// The first assertion is an empty list, which is the vacuous-green shape this
/// estate keeps paying for: a checker that returned `vec![]` unconditionally
/// would pass it. **So the second half drives a declaration that trips it**,
/// over a property measured `Never` rather than one chosen for looking
/// read-only.
#[test]
fn no_row_offers_an_edit_that_cannot_land_and_the_check_is_not_vacuous() {
  let loaded = Loaded::load().expect("loads");
  assert!(
    loaded.offers_an_edit_that_cannot_land().is_empty(),
    "a row offering an edit the store rejects teaches the operator the tool is unreliable,      and the refusal arrives after they have typed: {:?}",
    loaded.offers_an_edit_that_cannot_land()
  );

  // **THE NON-VACUITY ARM.** `id` is measured `Never` -- the value IS the
  // entity's address, and D57-8 gives renames no verb.
  assert_eq!(
    face_reach("thread")
      .expect("a face")
      .into_iter()
      .find(|(p, _)| p == "id")
      .map(|(_, kind)| kind),
    Some(Reach::Never),
    "this arm rests on `id` being unchangeable; if that moved, the probe below proves nothing"
  );
  let promising = Loaded::from_str(&declaration(
    r#"{ "name": "id", "label": "id", "widget": "text", "editable": true }"#,
  ))
  .expect("a form MAY declare a read-only property -- what it may not do is offer to edit it");
  assert_eq!(
    promising.offers_an_edit_that_cannot_land(),
    vec![("thread".to_string(), "id".to_string())],
    "the checker must actually fire on a row that offers an impossible edit"
  );
}

/// **THE CONVERSE ARM'S POPULATION, DERIVED RATHER THAN LISTED.**
///
/// Prints what the mutation surface says is writable and what the shipped
/// forms reach, so the expectation is set from a measurement rather than from
/// a guess. Run with `--ignored --nocapture`.
#[test]
#[ignore = "derivation aid, not an assertion"]
fn show_the_writable_population() {
  let loaded = Loaded::load().expect("loads");
  for kind in ["thread", "wp", "issue"] {
    println!("REACH, {kind}:");
    for (property, reach) in face_reach(kind).expect("a face") {
      println!("  {kind}.{property:<16} {reach:?}");
    }
  }
  println!("CHANGEABLE AND NOT ON ANY FORM:");
  for (entity, property) in loaded.changeable_and_not_on_any_form() {
    println!("  {entity}.{property}");
  }
  println!("OFFERS AN EDIT THAT CANNOT LAND:");
  for (entity, field) in loaded.offers_an_edit_that_cannot_land() {
    println!("  {entity}.{field}");
  }
}
