//! **AC-08.5's LIMB 1, MEASURED ON THE FIELD AXIS RATHER THAN THE ADDRESS AXIS.**
//!
//! The row asks that *every writable field of every entity is settable through
//! the mutation surface, and a field that cannot be written is reported BY
//! NAME*. The instrument that existed answered an ADDRESS question -- can this
//! form be reached -- and read green while three entities refused every field
//! they had, by FORM, naming none of them.
//!
//! # The population is DERIVED, never enumerated (vc's ruling, 2026-08-25)
//!
//! A rule that excludes an entity because its model does not exist YET is a
//! guard whose scope excludes the case it will later need to catch. So
//! `fields_of` is an EXHAUSTIVE match -- a fourteenth `Entity` variant fails to
//! compile until someone says which side it is on -- and the partition SIZES are
//! pinned here so `Node` gaining a model under ST0056/WP-14 **reds this test and
//! announces itself** instead of being silently absorbed.

use intentsvcs::address;
use intentsvcs::facade::Facade;

/// Every `Entity` form, one of each, so the partition below is over the whole
/// enum rather than over whatever happened to be convenient.
fn every_form() -> Vec<(&'static str, &'static str)> {
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

/// **THE POPULATION, PINNED IN BOTH DIRECTIONS.**
///
/// 6 model types carry fields; 4 collections and 2 append-only logs contribute
/// zero rows; 1 form has no model type yet. **13 total, and the total is
/// asserted too** -- a partition whose parts are checked and whose sum is not
/// can lose a form without any part changing.
#[test]
fn the_field_axis_population_is_six_of_thirteen_forms() {
  let mut counts = std::collections::BTreeMap::new();
  for (url, _) in every_form() {
    let a = address::promote(url).unwrap_or_else(|e| panic!("{url} parses: {e}"));
    let bucket = match Facade::settable_fields(&a.entity) {
      Ok(_) => "fields",
      Err(e) => {
        let said = format!("{e}");
        if said.contains("collection has membership") {
          "collection"
        } else if said.contains("append-only") {
          "append-only"
        } else if said.contains("no model type yet") {
          "no-model-yet"
        } else {
          panic!("{url} refused for an unclassified reason: {said}")
        }
      }
    };
    *counts.entry(bucket).or_insert(0) += 1;
  }

  assert_eq!(
    counts.get("fields"),
    Some(&6),
    "field-carrying forms: {counts:?}"
  );
  assert_eq!(
    counts.get("collection"),
    Some(&4),
    "collections: {counts:?}"
  );
  assert_eq!(
    counts.get("append-only"),
    Some(&2),
    "append-only logs: {counts:?}"
  );
  assert_eq!(
    counts.get("no-model-yet"),
    Some(&1),
    "**`Node` GAINING A MODEL MUST RED THIS TEST RATHER THAN JOIN SILENTLY.** \
     ST0056/WP-14 owns reifying it, and whether it should be reified at all is \
     HELD WITH hv -- this assertion is what makes deferring that question safe. {counts:?}"
  );
  assert_eq!(
    counts.values().sum::<u32>(),
    13,
    "every form is in exactly one bucket: {counts:?}"
  );
}

/// **EVERY FIELD IS SETTABLE OR REFUSED BY NAME -- NEVER NEITHER.** This is the
/// clause the address-axis instrument could not see: `Issue` and `Attachment`
/// were refused by FORM, so all fifteen of their fields were simultaneously
/// unsettable and unreported.
#[test]
fn the_two_new_arms_account_for_every_declared_field() {
  let cases: [(&str, &[&str], &[&str]); 2] = [
    (
      "intent:///issues/0001",
      // settable
      &["slug", "title", "severity", "reporter", "body"],
      // refused, each by name and for its own reason
      &["schema", "number", "status", "closed", "created"],
    ),
    (
      // **AN EMPTY SETTABLE SET IS A COMPLETE ANSWER, NOT A MISSING ONE.** Every
      // attachment field has a real reason: the path is the address, the content
      // is written whole, and the checksum and length FOLLOW the content.
      "intent:///threads/ST0001/attachments/x.sh",
      &[],
      &["path", "text", "blob", "sha256", "bytes"],
    ),
  ];

  for (url, settable, refused) in cases {
    let a = address::promote(url).expect("parses");
    let got = Facade::settable_fields(&a.entity).unwrap_or_else(|e| panic!("{url}: {e}"));
    for f in settable {
      assert!(
        got.contains(&f.to_string()),
        "{url}: `{f}` must be settable, got {got:?}"
      );
    }
    for f in refused {
      assert!(
        !got.contains(&f.to_string()),
        "{url}: `{f}` must not be settable, got {got:?}"
      );
    }
    assert_eq!(
      got.len(),
      settable.len(),
      "{url}: the settable set is exactly the declared one -- a field added to the model \
       joins this list deliberately or reds here. got {got:?}"
    );
  }
}

/// **A REFUSAL'S REASON MUST BE TRUE OF THE THING IT REFUSES.**
///
/// The arm this replaces refused everything outside its four cases with *an
/// attachment's body is its content, and the rest are collections or append-only
/// logs* -- and `Issue` is neither, nor is `Node`. **An operator refused on an
/// issue address was told they had addressed a collection.** A wrong reason is
/// worse than none: it reads as considered and sends the reader somewhere real.
#[test]
fn no_refusal_calls_an_entity_something_it_is_not() {
  for (url, kind) in every_form() {
    let a = address::promote(url).expect("parses");
    let Err(e) = Facade::settable_fields(&a.entity) else {
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
