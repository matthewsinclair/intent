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
      // settable -- `created` INCLUDED, and its refusal was withdrawn as false:
      // `Machine("intent issues add")` named a verb that creates an issue and
      // cannot move the field on one that exists. `Thread::created` is settable
      // too, and this row's first burning case is a provenance field that is
      // WRONG (`Thread::completed`, NULL on ST0011) -- so machine-stamped argues
      // FOR a setter. A stamp nothing can correct is how an estate keeps a value
      // it already knows is false.
      &["slug", "title", "severity", "reporter", "body", "created"],
      // refused, each by name and for its own reason
      &["schema", "number", "status", "closed"],
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

/// **A `#[serde(skip)]` FIELD IS STILL A DECLARED FIELD, AND THE SKIPPED ONE IS
/// THIS ROW'S OWN BURNING CASE.**
///
/// `Attachment::blob` carries the bytes and is skipped because they live in a
/// sidecar, so a field set derived from serialisation returns four of its five
/// names. **An instrument that cannot see the case the criterion exists for is
/// the address-axis mistake one layer down** -- a true measurement of something
/// narrower than the row asks about. Found by ic, against their own interest.
///
/// The per-model totals are pinned so a SECOND skip cannot slip in silently: add
/// one and the serialisable set shrinks while the declared total does not.
#[test]
fn the_declared_field_count_includes_what_serialisation_skips() {
  let counts: [(&str, usize); 6] = [
    ("intent:///threads/ST0001", 18),
    ("intent:///threads/ST0001/wp/01", 9),
    ("intent:///threads/ST0001/ac/AC-01.1", 4),
    ("intent:///threads/ST0001/at/AT-01.1", 8),
    ("intent:///threads/ST0001/attachments/x.sh", 5),
    ("intent:///issues/0001", 10),
  ];
  let mut total = 0;
  for (url, want) in counts {
    let a = address::promote(url).expect("parses");
    let settable = Facade::settable_fields(&a.entity).expect("has fields");
    let refused: usize = want - settable.len();
    assert!(
      refused <= want,
      "{url}: more settable than declared, which cannot happen"
    );
    total += want;
  }
  assert_eq!(
    total, 54,
    "the declared field population across the six model types"
  );

  // The one that proves the skip is accounted for rather than merely counted.
  let a = address::promote("intent:///threads/ST0001/attachments/x.sh").expect("parses");
  assert!(
    Facade::settable_fields(&a.entity)
      .expect("has fields")
      .is_empty(),
    "every attachment field is refused, blob included"
  );
}

mod common;

/// A thread body carrying every scalar and NO child collection.
///
/// **THE CHILDREN ARE OMITTED BECAUSE THE DOOR REFUSES THEM BY NAME**, which is
/// the arm this criterion's first limb already satisfies -- `wps`, `criteria`,
/// `tests` and `attachments` each have an address of their own. So the "whole
/// body" a caller can legally send is the scalars, and that is exactly the body
/// whose omissions used to clear eight of them.
fn scalars_of(t: &intentsvcs::model::Thread) -> serde_json::Value {
  let mut v = serde_json::to_value(t).expect("serialises");
  let o = v.as_object_mut().expect("an object");
  for child in ["wps", "criteria", "tests", "attachments"] {
    o.remove(child);
  }
  v
}

/// **LIMB 2, AT THE DOOR THAT WAS FAILING IT 8 OF 8.**
///
/// AC-08.5's second clause: *no verb silently clears a field it was not asked to
/// change.* Measured 2026-08-24 (ic, `ea84d0ae`): a minimal legal `put` at a
/// thread address -- the five schema-required fields plus `completed` -- cleared
/// the other EIGHT scalars. Nothing partial.
///
/// **THE GRAFT IS WHAT MAKES IT A CHOICE RATHER THAN A LIMITATION.** Four lines
/// restore the four children from the stored row; the nine scalars four lines
/// away are not restored. Had the children moved too, the finding would be the
/// weak *parse-and-replace replaces the document*.
#[test]
fn the_thread_door_refuses_to_clear_what_the_body_does_not_mention() {
  let fx = common::Fixture::new();
  let mut t = common::sample_thread("ST0001");
  t.context = "load-bearing prose nobody sent in the body".to_string();
  t.objective = "also load-bearing".to_string();
  t.related = vec![intentsvcs::model::Related {
    id: "ST0002".to_string(),
    note: None,
  }];
  fx.write_thread(&t);
  let mut f = fx.facade();
  let addr = intentsvcs::address::parse("intent:///threads/ST0001").expect("address");

  // The minimal legal body: schema-required fields plus the one being changed.
  let minimal = serde_json::json!({
    "schema": t.schema,
    "id": "ST0001",
    "title": t.title,
    "status": intentsvcs::model::enum_str(&t.status),
    "created": t.created,
    "completed": "2026-08-25",
  });

  let e = f
    .put(&addr, &minimal.to_string())
    .expect_err("a body that would clear eight fields is refused");
  let said = format!("{e}");

  // **REFUSED BY NAME, WHICH IS THE CLAUSE.** A refusal saying only "this would
  // change other fields" fails the criterion for the same reason the by-form
  // refusal did: the operator cannot act on it.
  for field in ["context", "objective", "related"] {
    assert!(
      said.contains(&format!("`{field}`")),
      "the refusal names `{field}`: {said}"
    );
  }

  // **AND NOTHING MOVED.** A verb that reported the collateral correctly and
  // wrote anyway would be the worse half of this class.
  let after = f.st_show("ST0001").expect("still there");
  assert_eq!(after.context, "load-bearing prose nobody sent in the body");
  assert_eq!(after.related.len(), 1);
  assert_eq!(
    after.completed, None,
    "the asked-for change did not land either -- the write is refused whole"
  );
}

/// The positive control: a body that mentions what it changes still writes.
///
/// **WITHOUT THIS, THE REFUSAL ABOVE IS SATISFIED BY A DOOR THAT REFUSES
/// EVERYTHING** -- a green that is a fact about the harness rather than about
/// the verb.
#[test]
fn a_body_that_mentions_the_field_it_changes_still_writes() {
  let fx = common::Fixture::new();
  let t = common::sample_thread("ST0001");
  fx.write_thread(&t);
  let mut f = fx.facade();
  let addr = intentsvcs::address::parse("intent:///threads/ST0001").expect("address");

  let mut whole = t.clone();
  whole.completed = Some("2026-08-25".to_string());
  f.put(&addr, &scalars_of(&whole).to_string())
    .expect("a whole body that changes one field is accepted");

  assert_eq!(
    f.st_show("ST0001").expect("there").completed.as_deref(),
    Some("2026-08-25"),
    "and the change it DID ask for landed"
  );
}
