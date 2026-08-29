//! ST0066 AC-00.2, the model half: **the fiat record survives serde
//! byte-faithfully, and adding it did not quietly switch off the strictness the
//! rest of the enum relies on.**
//!
//! # Why the strictness arm exists, and why it is the one that matters
//!
//! `AcState` is `#[serde(tag = "is", deny_unknown_fields)]`. Every other
//! variant is a unit or a struct variant; `Fiat` is the first NEWTYPE variant,
//! carrying [`FiatRecord`] so the record's shape has one home rather than being
//! spelled out again inside the enum.
//!
//! **Internal tagging and newtype variants interact, and the interaction is not
//! obvious from reading either feature's documentation.** The risk taken
//! knowingly here is that `deny_unknown_fields` stops applying through the
//! newtype -- which would not fail to compile, would not fail a round-trip, and
//! would show up only as an estate quietly accepting canon it should have
//! refused. That is the whole reason `an_unknown_field_is_still_refused_inside a
//! fiat state` is written before anything is built on top of this type: the
//! defect it guards against is invisible to every other test in this file.
//!
//! The alternative shape -- a struct variant repeating `FiatRecord`'s five
//! fields inline -- was available and is what the neighbouring `Descoped` and
//! `Withdrawn` do. It was not taken because the same record also hangs off
//! `Thread` and `WorkPackage`, so spelling it inline would put its shape in
//! three homes. **This file is what makes that choice safe rather than merely
//! tidier.**

use intentsvcs::model::{AcState, FiatRecord, Invoker};

fn record() -> FiatRecord {
  FiatRecord {
    because: "the panel-survival half is unobservable by unit test".to_string(),
    by: "hv".to_string(),
    at: "2026-08-28T18:30:00.000Z".to_string(),
    invoker: Invoker {
      tty: true,
      env: "darwin/arm64".to_string(),
    },
    inherited_from: None,
  }
}

#[test]
fn a_fiat_state_round_trips_byte_faithfully() {
  let state = AcState::Fiat(record());
  let first = serde_json::to_string(&state).expect("serialise");
  let back: AcState = serde_json::from_str(&first).expect("deserialise");
  let second = serde_json::to_string(&back).expect("re-serialise");
  assert_eq!(
    first, second,
    "the fiat state did not survive a serde round-trip unchanged, so the stored \
     record and the extracted one are not the same fact"
  );
  assert_eq!(
    back, state,
    "the value itself changed across the round-trip"
  );
}

#[test]
fn the_round_trip_comparison_can_actually_fail() {
  // THE POSITIVE CONTROL. An equality that agrees with everything agrees
  // silently, and the arm above would pass on a broken round-trip and on a
  // broken comparison identically.
  let mut altered = record();
  altered.because = "a different reason entirely".to_string();
  let a = serde_json::to_string(&AcState::Fiat(record())).expect("serialise");
  let b = serde_json::to_string(&AcState::Fiat(altered)).expect("serialise");
  assert_ne!(
    a, b,
    "two fiat states differing in their reason serialised identically, so the \
     comparison in the arm above proves nothing"
  );
}

#[test]
fn the_serde_form_is_internally_tagged_and_flat_like_its_neighbours() {
  // **The newtype must not introduce a nesting level.** If it did, the extract
  // would read `{"is":"fiat","0":{...}}` or similar, every hand-written
  // consumer of the canon would meet a shape no other state uses, and the
  // published JSON Schema face would disagree with the estate's own files.
  let json: serde_json::Value = serde_json::to_value(AcState::Fiat(record())).expect("serialise");
  let object = json
    .as_object()
    .expect("a fiat state serialises as an object");
  assert_eq!(
    object.get("is").and_then(|v| v.as_str()),
    Some("fiat"),
    "the discriminant is not where every other state puts it: {json}"
  );
  for field in ["because", "by", "at", "invoker"] {
    assert!(
      object.contains_key(field),
      "`{field}` is not flattened alongside the tag, so the newtype introduced a \
       nesting level no other variant has: {json}"
    );
  }
  assert!(
    !object.contains_key("0"),
    "the newtype serialised positionally rather than flattening: {json}"
  );
}

#[test]
fn an_unknown_field_is_still_refused_inside_a_fiat_state() {
  // **THE ARM THIS FILE EXISTS FOR.** `deny_unknown_fields` is the property
  // that makes hand-authored canon safe to trust, and whether it survives
  // through an internally-tagged NEWTYPE variant is a question about serde
  // rather than about this code. Measured here rather than assumed, because
  // the failure is silent in both directions: nothing fails to compile, and
  // nothing else in this file would notice.
  let refused = serde_json::from_str::<AcState>(
    r#"{"is":"fiat","because":"x","by":"hv","at":"2026-08-28T18:30:00.000Z",
        "invoker":{"tty":true,"env":"darwin/arm64"},"smuggled":"value"}"#,
  );
  assert!(
    refused.is_err(),
    "an unknown field was ACCEPTED inside a fiat state -- the newtype variant \
     has switched off the strictness every other state in this enum relies on, \
     and canon carrying a typo would now be ingested in silence"
  );
}

#[test]
fn the_strictness_arm_accepts_the_same_document_without_the_unknown_field() {
  // The other half of the arm above: without this, `is_err()` would be
  // satisfied by a document that is malformed for some unrelated reason, and
  // the strictness check would pass while testing nothing.
  let accepted = serde_json::from_str::<AcState>(
    r#"{"is":"fiat","because":"x","by":"hv","at":"2026-08-28T18:30:00.000Z",
        "invoker":{"tty":true,"env":"darwin/arm64"}}"#,
  );
  assert!(
    accepted.is_ok(),
    "the well-formed document was refused too, so the refusal above says nothing \
     about the unknown field: {accepted:?}"
  );
}

#[test]
fn a_cascaded_record_is_distinguishable_from_a_directly_closed_one() {
  // AC-00.3 rests on this: a cascaded row must never be indistinguishable from
  // one hv judged individually. The field is omitted entirely when absent, so
  // the two are distinct in the extract and not merely in memory.
  let direct = serde_json::to_string(&AcState::Fiat(record())).expect("serialise");
  let mut cascaded = record();
  cascaded.inherited_from = Some("ST0066".to_string());
  let cascaded = serde_json::to_string(&AcState::Fiat(cascaded)).expect("serialise");
  assert!(
    !direct.contains("inherited_from"),
    "an absent ancestor still emitted the field, so every direct close now \
     carries a null a reader has to interpret: {direct}"
  );
  assert!(
    cascaded.contains("\"inherited_from\":\"ST0066\""),
    "the cascade marker did not reach the extract: {cascaded}"
  );
}
