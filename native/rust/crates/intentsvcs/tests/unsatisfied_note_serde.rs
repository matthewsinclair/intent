//! **0133, the model half: widening `AcState::Unsatisfied` must not change one
//! byte of the canon already on disk, and must not switch off the strictness
//! the rest of the enum relies on.**
//!
//! # Why every arm here goes through JSON rather than through the constructor
//!
//! `Unsatisfied` is a UNIT variant today and a struct variant after this change,
//! so `AcState::Unsatisfied` and `AcState::Unsatisfied { note: None }` are
//! different syntax for the same value. **An arm written against either
//! constructor can only ever run on one side of the change, and an arm that
//! cannot run before is not a control -- it is a green collected afterwards.**
//!
//! So the arms below construct from bytes. Every one of them compiles and passes
//! against the UNIT variant, which is what makes them evidence: they were driven
//! to green before the type moved, and any red after the move is the move's
//! doing rather than a new test's. `fiat_state_serde.rs` paid for this lesson
//! from the other direction -- there the risk was newtype-versus-tag, here it is
//! that a variant which already exists in twelve estates' canon must keep
//! reading and writing identically.
//!
//! # The property that actually protects the estate
//!
//! `an_unsatisfied_state_with_no_note_serialises_to_exactly_the_old_bytes`.
//! Every `{"is":"unsatisfied"}` in every committed extract must round-trip to
//! itself. If the widened variant emitted `{"is":"unsatisfied","note":null}`,
//! nothing would fail to compile, no round-trip would break, and **every canon
//! file in the estate would rewrite itself on the next flush** -- a diff against
//! every thread, produced by a change that was supposed to add an optional
//! field. That is what `skip_serializing_if` is carrying, and it is asserted
//! here rather than assumed from the attribute.

use intentsvcs::model::AcState;

const OLD_BYTES: &str = r#"{"is":"unsatisfied"}"#;

#[test]
fn existing_canon_without_a_note_still_deserialises() {
  let parsed = serde_json::from_str::<AcState>(OLD_BYTES);
  assert!(
    parsed.is_ok(),
    "the shape every existing extract already carries stopped parsing: {parsed:?}"
  );
}

#[test]
fn an_unsatisfied_state_with_no_note_serialises_to_exactly_the_old_bytes() {
  // **THE ARM THIS FILE EXISTS FOR.** Not "it round-trips" -- byte equality with
  // the literal already committed across the estate. A `null` payload key would
  // satisfy a round-trip and still rewrite every canon file in the fleet.
  let parsed: AcState = serde_json::from_str(OLD_BYTES).expect("deserialise");
  let out = serde_json::to_string(&parsed).expect("serialise");
  assert_eq!(
    out, OLD_BYTES,
    "an unsatisfied state with nothing to record no longer serialises to the \
     bytes already on disk, so widening the variant rewrites canon it did not \
     change"
  );
}

#[test]
fn the_byte_comparison_can_actually_fail() {
  // The positive control. The assertion above compares a string to a constant,
  // and a comparison that agrees with everything agrees silently -- it would
  // pass just as well if `to_string` were returning its own input.
  let other: AcState =
    serde_json::from_str(r#"{"is":"computed"}"#).expect("deserialise a different state");
  let out = serde_json::to_string(&other).expect("serialise");
  assert_ne!(
    out, OLD_BYTES,
    "a different state serialised to the unsatisfied bytes, so the equality in \
     the arm above proves nothing"
  );
}

#[test]
fn an_unknown_field_is_refused_inside_an_unsatisfied_state() {
  // `deny_unknown_fields` is what makes hand-authored canon safe to trust.
  // Whether it survives a unit variant becoming a struct variant is a question
  // about serde rather than about this code, and the failure is silent in both
  // directions: nothing fails to compile and no round-trip breaks.
  let refused = serde_json::from_str::<AcState>(r#"{"is":"unsatisfied","smuggled":"value"}"#);
  assert!(
    refused.is_err(),
    "an unknown field was ACCEPTED inside an unsatisfied state -- canon carrying \
     a typo would now be ingested in silence"
  );
}

#[test]
fn the_strictness_arm_accepts_the_same_document_without_the_unknown_field() {
  // Without this, `is_err()` above would be satisfied by a document malformed
  // for some unrelated reason, and the strictness check would pass while
  // testing nothing.
  let accepted = serde_json::from_str::<AcState>(OLD_BYTES);
  assert!(
    accepted.is_ok(),
    "the well-formed document was refused too, so the refusal above says nothing \
     about the unknown field: {accepted:?}"
  );
}
