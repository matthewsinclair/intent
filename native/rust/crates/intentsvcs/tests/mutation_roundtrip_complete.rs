//! AT-08.2 / AC-08.2: **`GET ?format=json`, modify, `PUT` the same shape back
//! is lossless for every field of every entity** -- the interchange format IS
//! the mutation format, denominator printed over the field set.
//!
//! **This gives ST0056's AC-02.6 its second job.** A lossless 1-1 mapping used
//! to be a durability guarantee at the clone boundary; now it is also the
//! completeness guarantee for the whole mutation surface, because **a field
//! that does not round-trip is a field that cannot be WRITTEN.** The two stop
//! being separate properties.
//!
//! # The denominator is the model's own keys
//!
//! [`mutations`] maps every field to a DIFFERENT value, and
//! [`the_mutation_map_covers_every_serialised_field`] asserts that map covers
//! exactly what a populated row serialises. A field added to the model with no
//! entry fails there rather than dropping silently out of the sweep below --
//! which is the only thing that makes "for every field" a claim rather than a
//! hope.
//!
//! # Why each field is changed to a DIFFERENT value
//!
//! A round-trip test that writes back what it read passes against a `PUT` that
//! does nothing at all. **The value has to move**, and the read-back has to see
//! the moved value -- so each case asserts three things: the field changed,
//! it changed to the value asked for, and no sibling field moved with it.

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::address::parse;
use serde_json::{Value, json};

/// Every AT field and a value it does not currently hold.
///
/// `id` is absent deliberately and it is not an omission: the id is the
/// ADDRESS. Changing it through a `PUT` to the old address would be a rename
/// wearing an update's clothes, and D57-8 gives renames no verb.
fn mutations() -> Vec<(&'static str, Value)> {
  vec![
    ("kind", json!("non-test")),
    (
      "file",
      json!("native/rust/crates/intentsvcs/tests/moved.rs"),
    ),
    ("prose", json!("read the migration guide end to end")),
    ("covers", json!(["AC-03.2"])),
    ("status", json!("red")),
    ("note", json!("rewritten through the address surface")),
    ("legacy", json!({"raw": "AT-03.1 :: bin/intent_st_zero"})),
  ]
}

fn row_json(facade: &intentsvcs::facade::Facade, at: &str) -> Value {
  let row = facade.canon().threads[0]
    .tests
    .iter()
    .find(|t| t.id == at)
    .unwrap_or_else(|| panic!("{at} is present"));
  serde_json::to_value(row).expect("serialises")
}

/// The denominator. A field the model serialises and this file does not know
/// about would otherwise pass the sweep by not being in it.
#[test]
fn the_mutation_map_covers_every_serialised_field() {
  let thread = sample_thread("ST0001");
  let row = serde_json::to_value(
    thread
      .tests
      .iter()
      .find(|t| t.id == "AT-03.1")
      .expect("fixture row"),
  )
  .expect("serialises");

  let mapped: Vec<&str> = mutations().iter().map(|(f, _)| *f).collect();
  let unmapped: Vec<&String> = row
    .as_object()
    .expect("object")
    .keys()
    .filter(|k| k.as_str() != "id" && !mapped.contains(&k.as_str()))
    .collect();

  assert!(
    unmapped.is_empty(),
    "these fields serialise and have no mutation case, so the round-trip sweep\n       \
     below cannot see them: {unmapped:?}"
  );
}

/// **The sweep: every field, changed, written back, read out.**
#[test]
fn every_field_survives_get_modify_put() {
  let mut checked = 0usize;

  for (field, new_value) in mutations() {
    let fx = Fixture::new();
    fx.write_thread(&sample_thread("ST0001"));
    let mut facade = fx.facade();

    let before = row_json(&facade, "AT-03.1");
    let mut body = before.clone();

    // A field the fixture already holds at the target value would make the
    // read-back pass without the write doing anything.
    // **Two ways a case can be vacuous, and the second is the one that bit me.**
    // The obvious one is writing the value the row already holds. The other is
    // writing `null` for a field that is ABSENT: `before.get(field)` is `None`
    // rather than `Some(Null)`, so a naive equality check passes it through and
    // the read-back then compares `Null` to `Null` and proves nothing.
    let already = before.get(field).unwrap_or(&Value::Null);
    if already == &new_value {
      panic!(
        "the fixture already holds `{field}` at the value this case writes, so the\n       \
         case cannot distinguish a working PUT from a no-op. Pick another value."
      );
    }
    if new_value.is_null() {
      body.as_object_mut().expect("object").remove(field);
    } else {
      body[field] = new_value.clone();
    }

    let address = parse("intent:///threads/ST0001/at/AT-03.1?format=json").expect("resolves");
    facade
      .put(&address, &serde_json::to_string(&body).expect("serialises"))
      .unwrap_or_else(|e| panic!("PUT of a modified `{field}` must succeed: {e}"));

    let after = row_json(&facade, "AT-03.1");

    let landed = after.get(field).cloned().unwrap_or(Value::Null);
    assert_eq!(
      landed, new_value,
      "`{field}` did not survive the round trip -- and under AC-08.2 that makes it\n       \
       a field that CANNOT BE WRITTEN, not merely one that reads back oddly"
    );

    let mut siblings_moved: Vec<String> = Vec::new();
    for key in before.as_object().expect("object").keys() {
      if key == field {
        continue;
      }
      if before.get(key) != after.get(key) {
        siblings_moved.push(key.clone());
      }
    }
    assert!(
      siblings_moved.is_empty(),
      "writing `{field}` also moved {siblings_moved:?} -- a PUT is the whole row and\n       \
       must land exactly the row it was given"
    );
    checked += 1;
  }

  assert_eq!(
    checked,
    mutations().len(),
    "every mapped field must have been exercised"
  );
}

/// **The identity round trip.** `GET` then `PUT` unchanged must be a no-op --
/// if it is not, something in the serialisation is not stable and every diff
/// this surface produces would carry noise.
#[test]
fn an_unmodified_round_trip_changes_nothing() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();

  let before = row_json(&facade, "AT-03.1");
  let address = parse("intent:///threads/ST0001/at/AT-03.1?format=json").expect("resolves");
  facade
    .put(
      &address,
      &serde_json::to_string(&before).expect("serialises"),
    )
    .expect("writing back what was read is accepted");

  assert_eq!(
    row_json(&facade, "AT-03.1"),
    before,
    "GET then PUT unchanged must be an identity"
  );
}

/// A body missing a REQUIRED field is refused rather than defaulted. A `PUT`
/// is the whole row, so a missing field means the caller lost it somewhere --
/// silently filling it in would write a row nobody authored.
#[test]
fn a_body_missing_a_required_field_is_refused() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();

  let mut body = row_json(&facade, "AT-03.1");
  body.as_object_mut().expect("object").remove("covers");

  let address = parse("intent:///threads/ST0001/at/AT-03.1").expect("resolves");
  assert!(
    facade
      .put(&address, &serde_json::to_string(&body).expect("serialises"))
      .is_err(),
    "`covers` is required -- a PUT that drops it must refuse, not default it"
  );
}
