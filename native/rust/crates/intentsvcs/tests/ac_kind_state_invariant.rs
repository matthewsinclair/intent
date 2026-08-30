//! The `kind`/`state` cross-field invariant, held at the FILE as well as at
//! the API.
//!
//! vc named the cost of the collapse on 2026-08-15, reversing its own ruling in
//! favour of the shape already built: two fields can express nonsense.
//! `{kind: test, state: satisfied}` records a satisfaction nothing computed;
//! `{kind: non-test, state: computed}` claims a derivation with nothing to
//! derive from. `Guard::NonTestOnly` shuts that door at the API -- but under
//! D34 the committed extract is the interchange, so **a combination the extract
//! can carry and ingest will reject is a round-trip failure sitting at the
//! clone boundary**, not a validation nicety.
//!
//! So the rule is expressed in the generated JSON Schema face, which ingest
//! validates against and every non-Intent reader can read. That makes it one
//! artefact rather than two that agree today -- and it makes the block on
//! [`Criterion`] hand-written JSON inside a generated instrument, which is the
//! exact shape of thing this estate has already watched go stale in silence.
//! Hence this file: the JSON block and [`AcState::permitted_for`] are held to
//! each other over every variant the schema declares, and the roster of
//! variants is DISCOVERED from the schema rather than typed here.
//!
//! Proven by mutation, not assumed, and the two halves die differently:
//!
//! - **Deleting the `schemars(extend(...))` block** makes
//!   `the_schema_and_the_model_agree_on_every_pair` name all three pairs it
//!   stopped refusing, and `the_three_nonsense_pairs_...` name the first.
//! - **A sixth `AcState` variant does not reach a test at all** -- it fails to
//!   COMPILE in five places, `AcState::permitted_for` among them, so nobody can
//!   add a state without saying which kinds may hold it.
//! - **The gap between those two** is a variant that has been given its arms
//!   everywhere and not its schema clause. That is what
//!   `every_declared_state_has_a_sample` covers, and it was killed from the
//!   other side: dropping `Withdrawn` from `samples()` fails it by name, which
//!   is the same condition -- the schema declares a state nothing here
//!   exercises.

use intentsvcs::model::{AcKind, AcState, Criterion};
use serde_json::Value;

/// One value per `AcState` variant. Hand-written, and safe to be so ONLY
/// because `every_declared_state_has_a_sample` fails when the schema declares a
/// variant this list does not produce.
fn samples() -> Vec<AcState> {
  vec![
    AcState::Computed,
    AcState::Unsatisfied { note: None },
    AcState::Satisfied {
      evidence: "the render itself".to_string(),
    },
    AcState::Descoped {
      to: "ST0057".to_string(),
      by: Some("hv".to_string()),
      reason: Some("moved with the daemon".to_string()),
    },
    AcState::Withdrawn {
      reason: "the premise did not reproduce".to_string(),
      by: None,
    },
    // Legal on BOTH kinds, like the two states above it and for the same
    // reason: it records a decision about the requirement rather than about
    // its satisfaction. An over-cooked TEST-backed criterion is the case hv
    // described wanting to escape, so a fiat state barred from `test` would
    // miss the population the verb exists for.
    AcState::Fiat(intentsvcs::model::FiatRecord {
      because: "the half it asserts is unobservable by unit test".to_string(),
      by: "hv".to_string(),
      at: "2026-08-28T18:30:00.000Z".to_string(),
      invoker: intentsvcs::model::Invoker {
        tty: true,
        env: "darwin/arm64".to_string(),
      },
      inherited_from: None,
      inherited_event: None,
    }),
  ]
}

const KINDS: [AcKind; 2] = [AcKind::Test, AcKind::NonTest];

fn criterion_schema() -> Value {
  serde_json::to_value(schemars::schema_for!(Criterion))
    .expect("a schemars schema serialises to JSON by construction")
}

/// The tag values the schema declares for `AcState`, read out of its `oneOf`.
///
/// STRUCTURAL, not a list. schemars renders each internally-tagged variant as a
/// branch whose `is` property is a `const`, so the variant roster is a property
/// of the generated schema and cannot be out of step with the type.
fn declared_states(schema: &Value) -> Vec<String> {
  schema["$defs"]["AcState"]["oneOf"]
    .as_array()
    .expect("AcState is rendered as a oneOf of its variants")
    .iter()
    .map(|branch| {
      branch["properties"]["is"]["const"]
        .as_str()
        .expect("each variant branch pins its tag with a const")
        .to_string()
    })
    .collect()
}

fn tag_of(state: &AcState) -> String {
  serde_json::to_value(state).expect("AcState serialises")["is"]
    .as_str()
    .expect("the serde form carries its tag")
    .to_string()
}

fn criterion(kind: AcKind, state: AcState) -> Value {
  // Serialised from the real type rather than hand-built, so the shape under
  // test is the shape the tool actually writes.
  serde_json::to_value(Criterion {
    id: "AC-03.1".to_string(),
    text: "a requirement".to_string(),
    kind,
    state,
  })
  .expect("Criterion serialises")
}

/// Every variant the schema declares is exercised below.
///
/// This is the guard on the hand-written half. A sixth `AcState` variant makes
/// `permitted_for` fail to compile (its match is exhaustive) AND fails here,
/// so the JSON block on `Criterion` cannot be left un-taught while the type
/// moves on.
#[test]
fn every_declared_state_has_a_sample() {
  let declared = declared_states(&criterion_schema());
  let sampled: Vec<String> = samples().iter().map(tag_of).collect();

  assert!(
    declared.len() >= 5,
    "only {} states declared -- the probe is not reading the schema: {declared:?}",
    declared.len()
  );
  let missing: Vec<&String> = declared.iter().filter(|d| !sampled.contains(d)).collect();
  assert!(
    missing.is_empty(),
    "the schema declares {missing:?} and this file has no sample for it -- \
     add one, and teach the `schemars(extend(...))` block on `Criterion` which kinds may hold it"
  );
}

/// **The two witnesses, over the whole product.** For every (kind, state) pair
/// the JSON Schema's verdict must be the model's verdict.
#[test]
fn the_schema_and_the_model_agree_on_every_pair() {
  let schema = criterion_schema();
  let validator = jsonschema::validator_for(&schema).expect("the generated schema compiles");

  let mut disagreements = Vec::new();
  let mut checked = 0;
  for kind in KINDS {
    for state in samples() {
      checked += 1;
      let by_model = state.permitted_for(kind);
      let by_schema = validator.is_valid(&criterion(kind, state.clone()));
      if by_model != by_schema {
        disagreements.push(format!(
          "{:?} + {}: model says {by_model}, schema says {by_schema}",
          kind,
          tag_of(&state)
        ));
      }
    }
  }

  // 2 kinds x 6 states. Left as a literal rather than computed from
  // `samples().len()`: deriving it from the same source the loop walks would
  // make the assertion agree with any sample set including an empty one, and
  // this estate uses a hand-kept count as a deliberate tripwire elsewhere for
  // the same reason (`SCHEMA_VER_KEYS.len()`, "this count is the thing that
  // notices"). The independent witness is the sibling test, which reads the
  // declared states out of the SCHEMA rather than out of this file.
  assert_eq!(checked, 12, "the product was not walked");
  assert!(
    disagreements.is_empty(),
    "the schema face and `AcState::permitted_for` disagree:\n  {}\n\
     one of them is the published contract and the other is what the tool does; \
     they cannot differ",
    disagreements.join("\n  ")
  );
}

/// And the invariant is not vacuously satisfied by everything being legal:
/// exactly the three nonsense pairs are refused, by name.
#[test]
fn the_three_nonsense_pairs_are_refused_and_the_refusal_names_the_value() {
  let schema = criterion_schema();
  let validator = jsonschema::validator_for(&schema).expect("the generated schema compiles");

  let nonsense = [
    (
      AcKind::Test,
      AcState::Satisfied {
        evidence: "hand-authored".to_string(),
      },
    ),
    (AcKind::Test, AcState::Unsatisfied { note: None }),
    (AcKind::NonTest, AcState::Computed),
  ];

  for (kind, state) in nonsense {
    let tag = tag_of(&state);
    let instance = criterion(kind, state);
    let messages: Vec<String> = validator
      .iter_errors(&instance)
      .map(|e| format!("at {}: {e}", e.instance_path()))
      .collect();

    assert!(
      !messages.is_empty(),
      "{kind:?} + {tag} validated, and it is not a thing a criterion can be"
    );
    // A refusal that does not name the offending value leaves whoever hit it
    // reading the schema to find out what happened.
    assert!(
      messages.iter().any(|m| m.contains(&tag)),
      "{kind:?} + {tag} was refused without naming the value: {messages:?}"
    );
  }
}

/// The legal pairs really are legal -- otherwise the test above passes on a
/// schema that refuses everything.
#[test]
fn every_legal_pair_validates() {
  let schema = criterion_schema();
  let validator = jsonschema::validator_for(&schema).expect("the generated schema compiles");

  for kind in KINDS {
    for state in samples() {
      if !state.permitted_for(kind) {
        continue;
      }
      let tag = tag_of(&state);
      let instance = criterion(kind, state);
      let errors: Vec<String> = validator
        .iter_errors(&instance)
        .map(|e| e.to_string())
        .collect();
      assert!(
        errors.is_empty(),
        "{kind:?} + {tag} is a legal pair and the schema refused it: {errors:?}"
      );
    }
  }
}
