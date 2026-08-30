//! AT-04.6 / AC-04.6: mutation completeness (D32).
//!
//! **Every state an entity can enter, it can leave, by a service call.** hv
//! ruled it on a concrete instance -- `intent ac satisfy` was a one-way door,
//! so a verifier whose evidence proved incomplete had to hand-edit
//! `acceptance.md`, the file the CLI exists to own -- and stated it generally:
//! a state that can be entered and not left is a missing mutation, not a
//! missing flag.
//!
//! **This is held mechanically, and the mechanism is the point.** The AC could
//! be satisfied by a test that asserts `ac unsatisfy` exists; that would close
//! the instance and miss the rule, and the next state field to arrive would
//! reopen it. So the QUESTION comes from the committed JSON Schema, which is
//! generated from the model types: this walks it for every field with a closed
//! value domain and REFUSES any the transition table does not classify. A
//! state field added to the model fails on the day it is added, with nobody
//! having to remember either file exists -- which is what makes the rule bind
//! D30's whiteboard entities before they are written.
//!
//! It reads the schema by GENERATING it from the types rather than by reading
//! `schema/`, for the reason `faces::all_faces_banner` gives: the committed
//! file and the types are tied together by `schema_faces_drift.rs`, so
//! generating makes this a second independent witness rather than a reader of
//! a file that could have drifted.

// An integration test is its OWN CRATE, so the crate-level allow in
// `intentsvcs/src/lib.rs` does not reach here and has to be restated. The
// REASON is not restated with it: it lives at that one site, and a second copy
// would be a second home for it -- read it there.
#![allow(clippy::result_large_err)]

mod common;

use common::{Fixture, sample_issue, sample_thread};
use intentsvcs::facade::{Facade, FacadeError, Outcome};
use intentsvcs::model::{
  AcKind, AcState, AcceptanceMode, AtKind, AtStatus, Criterion, Issue, Thread, ThreadStatus,
  WpStatus, enum_str,
};
use intentsvcs::transitions::{
  ABSENT, Disposition, Edge, Entry, FIELDS, Field, Guard, exitless, find, traps, unreachable,
};
use serde_json::Value;

// ---------------------------------------------------------------------------
// The schema walk -- the mechanical driver
// ---------------------------------------------------------------------------

/// One closed-domain field: `(entity, field, values)`. `entity` is the JSON
/// Schema definition name, which is what the transition table joins on.
type DomainField = (String, String, Vec<String>);

fn schema(name: &str) -> Value {
  let text = intentsvcs::faces::face(name).unwrap_or_else(|| panic!("no face named {name}"));
  serde_json::from_str(&text).expect("a generated face is JSON")
}

/// Every field in one schema face whose value domain is closed.
fn closed_domain_fields(schema: &Value) -> Vec<DomainField> {
  let defs = schema.get("$defs").and_then(Value::as_object);
  let mut scopes: Vec<(String, &Value)> = vec![(
    schema
      .get("title")
      .and_then(Value::as_str)
      .expect("a face names its root type")
      .to_string(),
    schema,
  )];
  if let Some(defs) = defs {
    for (name, body) in defs {
      if body.get("properties").is_some() {
        scopes.push((name.clone(), body));
      }
    }
  }

  let mut out = Vec::new();
  for (entity, body) in scopes {
    let Some(props) = body.get("properties").and_then(Value::as_object) else {
      continue;
    };
    for (field, prop) in props {
      if let Some(values) = domain_of(prop, defs) {
        out.push((entity.clone(), field.clone(), values));
      }
    }
  }
  out.sort();
  out
}

/// A property's closed value domain, or `None` if it is open (a string, a
/// number, an array, a nested object).
///
/// Four shapes, because the generator emits four. Each was verified against
/// the real face rather than assumed -- an earlier probe of this same schema
/// checked `type == "boolean"` by scalar equality and silently missed
/// `Criterion.satisfied`, whose type is the ARRAY `["boolean", "null"]`. That
/// probe reported nine fields and stayed quiet about the one the whole ruling
/// came from, which is why `every_boolean_field_in_the_schema_is_found` exists
/// below.
fn domain_of(prop: &Value, defs: Option<&serde_json::Map<String, Value>>) -> Option<Vec<String>> {
  let nullable = mentions_type(prop, "null");
  if let Some(name) = first_ref(prop) {
    let def = defs?.get(&name)?;
    let mut values = def_values(def)?;
    if nullable {
      values.insert(0, ABSENT.to_string());
    }
    return Some(values);
  }
  if mentions_type(prop, "boolean") {
    let mut values = vec!["false".to_string(), "true".to_string()];
    if nullable {
      values.insert(0, ABSENT.to_string());
    }
    return Some(values);
  }
  None
}

/// The values a `$defs` entry admits: a plain string enum, or a `oneOf` whose
/// arms are bare strings (`AtStatus`) or tagged objects (`AcState`).
fn def_values(def: &Value) -> Option<Vec<String>> {
  if let Some(values) = string_list(def.get("enum")) {
    return Some(values);
  }
  let arms = def.get("oneOf")?.as_array()?;
  let mut values = Vec::new();
  for arm in arms {
    if let Some(list) = string_list(arm.get("enum")) {
      values.extend(list);
    } else if let Some(one) = arm.get("const").and_then(Value::as_str) {
      values.push(one.to_string());
    } else if let Some(props) = arm.get("properties").and_then(Value::as_object) {
      // A tagged union: the discriminant carries the state's name.
      //
      // **The tag is DISCOVERED, not guessed from a roster.** This read
      // `for key in ["state", "status"]` -- a hand-kept list of tag names --
      // and the AC collapse renamed a tag to `is`, at which point the walk
      // silently stopped classifying `Criterion.state` and the table's own
      // "every closed-domain field is classified" check reported the field as
      // ABSENT FROM THE SCHEMA. A roster that has to be updated by hand is the
      // enumerate-don't-sniff failure, in the instrument built to catch it.
      //
      // What identifies the tag structurally: serde emits each arm with the
      // discriminant as a single-valued property. Anything shaped that way IS
      // the tag, whatever it is called.
      for tag in props.values() {
        if let Some(one) = tag.get("const").and_then(Value::as_str) {
          values.push(one.to_string());
        } else if let Some(list) = string_list(tag.get("enum")) {
          values.extend(list);
        }
      }
    }
  }
  (!values.is_empty()).then_some(values)
}

fn string_list(value: Option<&Value>) -> Option<Vec<String>> {
  let items = value?.as_array()?;
  let out: Vec<String> = items
    .iter()
    .filter_map(Value::as_str)
    .map(str::to_string)
    .collect();
  (out.len() == items.len() && !out.is_empty()).then_some(out)
}

/// Whether a property's `type` mentions `wanted`, directly or in an `anyOf` /
/// `oneOf` arm. Handles both spellings the generator uses: a bare string and
/// a list.
fn mentions_type(prop: &Value, wanted: &str) -> bool {
  let here = match prop.get("type") {
    Some(Value::String(one)) => one == wanted,
    Some(Value::Array(many)) => many.iter().any(|t| t.as_str() == Some(wanted)),
    _ => false,
  };
  here
    || ["anyOf", "oneOf"].iter().any(|key| {
      prop
        .get(key)
        .and_then(Value::as_array)
        .is_some_and(|arms| arms.iter().any(|arm| mentions_type(arm, wanted)))
    })
}

/// The first `$defs` name this property refers to, following `anyOf` / `oneOf`
/// (which is how an `Option<T>` over an enum is emitted).
fn first_ref(prop: &Value) -> Option<String> {
  if let Some(reference) = prop.get("$ref").and_then(Value::as_str) {
    return reference.rsplit('/').next().map(str::to_string);
  }
  for key in ["anyOf", "oneOf"] {
    if let Some(arms) = prop.get(key).and_then(Value::as_array) {
      for arm in arms {
        if let Some(name) = first_ref(arm) {
          return Some(name);
        }
      }
    }
  }
  None
}

fn all_fields() -> Vec<DomainField> {
  let mut out = Vec::new();
  for face in [
    "thread.schema.json",
    "issue.schema.json",
    "event.schema.json",
  ] {
    out.extend(closed_domain_fields(&schema(face)));
  }
  out
}

// ---------------------------------------------------------------------------
// The walk cannot go blind
// ---------------------------------------------------------------------------

/// Every `$defs` entry that HAS a closed domain is claimed by at least one
/// field.
///
/// The canary for the ref-following half. If the generator starts nesting a
/// reference in a shape `first_ref` does not follow, the def goes unclaimed
/// and this fails -- rather than the walk quietly returning a shorter list and
/// every closure check below passing over a set it never looked at.
#[test]
fn every_enum_in_the_schema_is_claimed_by_a_field() {
  for face in [
    "thread.schema.json",
    "issue.schema.json",
    "event.schema.json",
  ] {
    let doc = schema(face);
    let claimed: Vec<Vec<String>> = closed_domain_fields(&doc)
      .into_iter()
      .map(|(_, _, values)| values)
      .collect();
    let Some(defs) = doc.get("$defs").and_then(Value::as_object) else {
      continue;
    };
    for (name, def) in defs {
      let Some(values) = def_values(def) else {
        continue;
      };
      assert!(
        claimed.iter().any(|c| values.iter().all(|v| c.contains(v))),
        "{face}: `{name}` defines a closed domain {values:?} that no field claims -- the walk is not following the reference to it"
      );
    }
  }
}

/// Every boolean-typed property is found, counted by a second method.
///
/// The canary for the inline half, where there is no `$defs` entry to anchor
/// on. A raw recursive count of boolean schema nodes is compared against what
/// the walk classified: they must agree, so a second `Option<bool>` field
/// added to the model cannot slip past the way the first one slipped past the
/// probe that preceded this file.
#[test]
fn every_boolean_field_in_the_schema_is_found() {
  for face in [
    "thread.schema.json",
    "issue.schema.json",
    "event.schema.json",
  ] {
    let doc = schema(face);
    let found = closed_domain_fields(&doc)
      .into_iter()
      .filter(|(_, _, values)| values.iter().any(|v| v == "true"))
      .count();
    assert_eq!(
      found,
      count_boolean_nodes(&doc),
      "{face}: the walk classified {found} boolean field(s) and the schema carries {} -- one is invisible to the walk",
      count_boolean_nodes(&doc)
    );
  }
}

fn count_boolean_nodes(value: &Value) -> usize {
  match value {
    Value::Object(map) => {
      let here = usize::from(match map.get("type") {
        Some(Value::String(one)) => one == "boolean",
        Some(Value::Array(many)) => many.iter().any(|t| t.as_str() == Some("boolean")),
        _ => false,
      });
      here + map.values().map(count_boolean_nodes).sum::<usize>()
    }
    Value::Array(items) => items.iter().map(count_boolean_nodes).sum(),
    _ => 0,
  }
}

/// The ruled instance, named -- and it now asserts the COLLAPSE rather than the
/// field the collapse removed.
///
/// `Criterion.satisfied` is what hv ruled on: an `Option<bool>` beside an
/// `AcScope`, three representable values for two meanings, one of them written
/// by nothing. It is gone, and "gone" is the assertion -- a schema that still
/// carried it would mean the two-field model had survived somewhere.
#[test]
fn the_ruled_field_is_gone_and_the_collapsed_one_is_walked() {
  let fields = all_fields();
  assert!(
    !fields
      .iter()
      .any(|(e, f, _)| e == "Criterion" && f == "satisfied"),
    "`Criterion.satisfied` is the field the collapse removed; a schema still carrying it means the two-field model survived"
  );
  let (_, _, values) = fields
    .iter()
    .find(|(e, f, _)| e == "Criterion" && f == "state")
    .expect("`Criterion.state` is what replaced it and must be walked");
  let mut sorted = values.clone();
  sorted.sort();
  assert_eq!(
    sorted,
    vec![
      "computed".to_string(),
      "descoped".to_string(),
      "fiat".to_string(),
      "satisfied".to_string(),
      "unsatisfied".to_string(),
      "withdrawn".to_string()
    ],
    "all six recorded states reach the walk, including BOTH entry values -- `computed` for a test-backed criterion and `unsatisfied` for an authored one"
  );
}

// ---------------------------------------------------------------------------
// The table answers the whole question
// ---------------------------------------------------------------------------

/// Neither direction may drift: every closed-domain field is classified, and
/// every classification names a field that exists.
#[test]
fn the_transition_table_classifies_exactly_the_schemas_closed_domain_fields() {
  let fields = all_fields();
  for (entity, field, _) in &fields {
    assert!(
      find(entity, field).is_some(),
      // **The old text said "or as Unbuilt naming the work package that owes
      // it", and D37 deleted the field it was telling the reader to fill.** A
      // failure message is the one piece of prose read at exactly the moment
      // somebody acts on it, so a message prescribing a retired mechanism sends
      // them to write code that will not compile.
      "{entity}.{field} has a closed value domain and the transition table does not classify it -- add it to transitions.rs as a State with its edges, as Unbuilt with a note saying what is unavailable, or as Immutable citing the ruling that says nothing will ever move it (D32)"
    );
  }
  for declared in FIELDS {
    assert!(
      fields
        .iter()
        .any(|(e, f, _)| e == declared.entity && f == declared.field),
      "the transition table declares {}.{} and no such closed-domain field is in the schema",
      declared.entity,
      declared.field
    );
  }
}

/// **hv's ruling, as an assertion.**
#[test]
fn no_state_can_be_entered_and_not_left() {
  for (entity, field, values) in all_fields() {
    let Some(Disposition::State { initial, edges, .. }) =
      find(&entity, &field).map(|f| &f.disposition)
    else {
      continue;
    };
    let stuck = traps(&values, initial, edges);
    assert!(
      stuck.is_empty(),
      "{entity}.{field} can be moved into {stuck:?} and not out again -- every state an entity can enter it must be able to leave, by a service call (D32/AC-04.6). Add the inverse mutation, or say why the state exists"
    );
  }
}

/// The mirror question, held to a declared list so the known gaps stay
/// recorded with their evidence and a NEW one fails.
#[test]
fn unreachable_states_are_exactly_the_declared_orphans() {
  for (entity, field, values) in all_fields() {
    let Some(Disposition::State {
      initial,
      edges,
      orphans,
    }) = find(&entity, &field).map(|f| &f.disposition)
    else {
      continue;
    };
    let mut found = unreachable(&values, initial, edges);
    found.sort();
    let mut declared: Vec<String> = orphans.iter().map(|(v, _)| v.to_string()).collect();
    declared.sort();
    assert_eq!(
      found, declared,
      "{entity}.{field}: the values nothing can produce are not the ones transitions.rs declares. A new one is a modelling defect (a state the model admits and the service cannot reach); a disappeared one means the orphan entry is stale and should come out"
    );
  }
}

/// An edgeless row, flattened: how a value gets in, what the row says, and
/// whether a mutation is OWED.
struct Edgeless {
  note: &'static str,
  entry: Entry,
  /// `Unbuilt` owes a mutation; `Immutable` was ruled to owe none and must cite
  /// the ruling. Carried rather than re-matched downstream so the checks below
  /// can still say WHICH claim a failure breaks.
  owed: bool,
  ruled: Option<&'static str>,
}

/// The three immovability checks reach their population through here, and
/// **the reason is a measurement, not a preference.**
///
/// Each used its own `let Disposition::Unbuilt { .. } = .. else { continue }`.
/// Adding a second edgeless variant and moving `Thread.acceptance` into it left
/// **20 passed, 0 failed** with the row gone from all four populations: three
/// `let-else`s skipped it in silence, and the one count guard that could have
/// noticed asserted `unbuilt_fields > 0`, which survives 4 becoming 3. The
/// enumeration-gone-short class, caught only because the drop was performed
/// deliberately to see whether anything would say so.
///
/// So the match here is exhaustive and it is the ONLY one: a third edgeless
/// variant makes the compiler ask this function, and every check inherits the
/// answer. `every_edgeless_row_is_accounted_for` holds the partition total, so a
/// row cannot leave this population without the `State` walks picking it up.
fn edgeless(field: &Field) -> Option<Edgeless> {
  match &field.disposition {
    Disposition::State { .. } => None,
    Disposition::Unbuilt { note, entry } => Some(Edgeless {
      note,
      entry: *entry,
      owed: true,
      ruled: None,
    }),
    Disposition::Immutable { note, entry, ruled } => Some(Edgeless {
      note,
      entry: *entry,
      owed: false,
      ruled: Some(ruled),
    }),
  }
}

/// **An edgeless field must be one NO service call can put a value into.**
///
/// vc's correction, and it is the load-bearing test of the pair: "carries no
/// edges" only says the table is self-consistent, and **edges are the exits,
/// not the entrances**. A field with no exits is harmless only if nothing can
/// enter it; if a service call can put a value in place, the entity is sitting
/// in a state no service call can leave, and the disposition is a label on a
/// defect rather than a description of one.
///
/// **This measures the SERVICE-PATH half only, and the argument that once
/// stood here for stopping at that half was wrong.** It ran: strict ingest
/// accepts any schema-valid canon, so under "enterable by any path" every value
/// of every closed-domain field is enterable, every row fails, and a test that
/// fires on everything is not a test.
///
/// The flaw is that enterability was never the failure -- entry WITHOUT AN EXIT
/// is, and the two are separate questions. Every value of every `State` field
/// has an exiting edge, so the wider reading fires on none of them; the
/// edgeless rows have no edges at all, so it fires on exactly those. It
/// discriminates cleanly, and `the_wider_reading_fires_on_edgeless_rows_only`
/// holds that as a measurement rather than leaving it as the counter-argument
/// to this one. The pair below is the second condition; this is the first.
///
/// It measures by DRIVING the creation verbs rather than by reading them. That
/// is how `WorkPackage.scope` was caught: `wp_new` takes the size from the
/// caller, so all six were entered at creation and none could be left.
///
/// **The requirement is the same for both edgeless variants and the failure
/// means opposite things**, so the message branches on `owed`: for an `Unbuilt`
/// row a settable field is a trap wearing an absence's label and the fix is to
/// build the exit, while for an `Immutable` row it is the RULING that has been
/// contradicted, and the fix is a conversation rather than a verb.
#[test]
fn no_service_call_can_set_an_edgeless_field() {
  for field in FIELDS {
    let Some(row) = edgeless(field) else {
      continue;
    };
    let settable = match (field.entity, field.field) {
      // `st_new` hardcodes `acceptance: None`; the caller has no say.
      //
      // **This is the stronger measurement of the two that would satisfy the
      // ruling, and it is the one available today.** "Immutable after creation"
      // would permit an `st new --exempt` setting it AT creation; nothing sets
      // it at all, which implies it. If that flag ever lands, this arm becomes
      // "created carrying it, and no verb moves it afterwards" -- a change of
      // measurement, not of ruling.
      ("Thread", "acceptance") => {
        let fx = Fixture::new();
        let mut facade = fx.facade();
        let id = facade.st_new("a thread").expect("st new");
        facade.st_show(&id).expect("thread").acceptance.is_some()
      }
      // No service call constructs a criterion or an acceptance test: they
      // arrive only as authored canon. Asserted by the absence of a creation
      // verb on the facade, which is a compile-time fact -- adding one and not
      // adding an arm here leaves this returning false, so the arm below is what
      // has to be kept honest, and the panic is the keeper.
      //
      // **`("Issue", "status")` was listed here and had been DEAD since Machine
      // 4 landed**, because the row became a `State` and the loop stops before
      // the match. A dead arm in a hand-written enumeration reads as coverage,
      // which is the same thing a short enumeration does.
      ("Criterion", "kind") | ("AcceptanceTest", "kind") => false,
      // No service call constructs a fiat record today, so nothing can set the
      // tty flag inside one. **This arm is written knowing it will change**:
      // when `fc` lands it will CREATE the record carrying this value, and the
      // measurement becomes the same one `("Thread", "acceptance")` describes
      // above -- created carrying it, and no verb moves it afterwards. That is
      // a change of measurement rather than of ruling, and the disposition
      // stays `Immutable` either way.
      ("Invoker", "tty") => false,
      other => panic!(
        "{other:?} is edgeless and no arm decides whether a service call can set it -- inertness is measured, never assumed"
      ),
    };
    let consequence = if row.owed {
      "the row is describing a trap rather than an absence. Make it a State and give it the exit"
    } else {
      "the row says nothing moves this field and something does, so the ruling it cites has been contradicted -- revisit the ruling, do not relax the test"
    };
    assert!(
      !settable,
      "{}.{} declares no edges and a service call CAN put a value into it -- so an entity can hold a value nothing can change, and {consequence}",
      field.entity, field.field
    );
  }
}

/// **AC-04.6's second condition: an edgeless field's ENTRY path is measured by
/// driving canon, and the table has to agree with the measurement.**
///
/// The measurement is one shape for every row: author canon carrying a value no
/// service call could have put there, read it back through the facade, and see
/// whether it is held as authored. If ingest normalised it away the field is
/// genuinely `Inert`; if it survives, an entity is holding a value with no verb
/// to move it.
///
/// **What this test does NOT do is discharge the criterion, and the distinction
/// matters because getting it wrong is how a red row goes quietly green.** Every
/// row here measures `Authored`; asserting the declaration matches the
/// measurement makes the position visible and guards against another arriving
/// unnoticed. It does not make an unleaveable value leaveable.
///
/// **`Authored` no longer implies a mutation is owed, and that split is the
/// point of the second variant.** For an `Unbuilt` row the criterion stays red
/// until the verb exists; for `Thread.acceptance` hv ruled authoring IS the
/// interface, so `Authored` is the finished answer rather than a debt. The
/// measurement is identical either way because it measures a property -- canon
/// can put a value here -- and not the classification.
#[test]
fn an_edgeless_fields_entry_path_is_measured_not_declared() {
  for field in FIELDS {
    let Some(Edgeless { entry, .. }) = edgeless(field) else {
      continue;
    };
    let measured = match (field.entity, field.field) {
      // `st_new` hardcodes `acceptance: None`, so `exempt` is a value only
      // canon supplies.
      ("Thread", "acceptance") => {
        let fx = Fixture::new();
        let mut thread = sample_thread("ST0001");
        thread.acceptance = Some(AcceptanceMode::Exempt);
        fx.write_thread(&thread);
        fx.facade()
          .st_show("ST0001")
          .expect("thread")
          .acceptance
          .is_some()
      }
      // **Two distinct authored values surviving distinctly**, which is the
      // form that also catches an ingest path NORMALISING the field: one value
      // read back correctly proves nothing a hardcoded default would not.
      //
      // **`kind` cannot be authored alone, and that is the sharpest thing this
      // arm found.** Flipping it on a `computed` criterion is schema-INVALID:
      // the pairing is enforced, so `Test` implies `computed` and `NonTest`
      // implies one of the four recorded states. The verb owed here therefore
      // has to move `kind` AND `state` in one act -- an unratified field and a
      // ratified one together -- which is more than the row's "needs a
      // hand-edit" suggests and is why it is not a small build.
      ("Criterion", "kind") => {
        let fx = Fixture::new();
        let thread = sample_thread("ST0001");
        fx.write_thread(&thread);
        let read = fx.facade();
        let kinds: Vec<AcKind> = read
          .st_show("ST0001")
          .expect("thread")
          .criteria
          .iter()
          .map(|c| c.kind)
          .collect();
        kinds.contains(&AcKind::Test) && kinds.contains(&AcKind::NonTest)
      }
      // The AT mirror, and it has NO coupled field: `kind` and `status` are
      // independent here, so a single flip is valid canon.
      ("AcceptanceTest", "kind") => {
        let fx = Fixture::new();
        let mut thread = sample_thread("ST0001");
        thread
          .tests
          .first_mut()
          .expect("the fixture carries tests")
          .kind = AtKind::NonTest;
        fx.write_thread(&thread);
        let read = fx.facade();
        let kinds: Vec<AtKind> = read
          .st_show("ST0001")
          .expect("thread")
          .tests
          .iter()
          .map(|t| t.kind)
          .collect();
        kinds.contains(&AtKind::Test) && kinds.contains(&AtKind::NonTest)
      }
      // Driven, not inspected: authored canon carrying a fiat state carries the
      // invoker inside it, so BOTH bool values have to survive the trip for the
      // `Authored` declaration to be true. Written with the two criteria
      // disagreeing, because a fixture that sets one value round-trips
      // vacuously -- it would pass on a reader that hardcoded either bool.
      ("Invoker", "tty") => {
        let fx = Fixture::new();
        let mut thread = sample_thread("ST0001");
        let record = |tty: bool| {
          AcState::Fiat(intentsvcs::model::FiatRecord {
            because: "cut and run".to_string(),
            by: "hv".to_string(),
            at: "2026-08-28T18:30:00.000Z".to_string(),
            invoker: intentsvcs::model::Invoker {
              tty,
              env: "darwin/arm64".to_string(),
            },
            inherited_from: None,
          })
        };
        assert!(
          thread.criteria.len() >= 2,
          "the fixture must carry two criteria for the two tty values to disagree"
        );
        thread.criteria[0].state = record(true);
        thread.criteria[1].state = record(false);
        fx.write_thread(&thread);
        let read = fx.facade();
        let flags: Vec<bool> = read
          .st_show("ST0001")
          .expect("thread")
          .criteria
          .iter()
          .filter_map(|c| match &c.state {
            AcState::Fiat(r) => Some(r.invoker.tty),
            _ => None,
          })
          .collect();
        flags.contains(&true) && flags.contains(&false)
      }
      // **`("Issue", "status")` had an arm here and it had been DEAD since
      // Machine 4 landed**, along with a comment reading "v3 ships the read
      // verbs only" that `issues.close` and `issues.open` had already falsified.
      // The row is a `State`, so the loop stops before the match ever reaches
      // it. Removed on the same grounds as its twin above: an arm that cannot
      // run is a coverage claim nothing honours, and a stale comment inside one
      // is how the claim keeps reading as true.
      other => panic!(
        "{other:?} is edgeless and no arm MEASURES how a value gets into it. Entry is driven, never assumed -- a row asserted inert on inspection is the exact \
         reasoning this test replaced"
      ),
    };
    let expected = matches!(entry, Entry::Authored);
    assert_eq!(
      measured, expected,
      "{}.{} declares {entry:?} and canon says otherwise. Held to the measurement in BOTH directions: a row claiming Inert that canon can populate is an \
       undeclared trap, and a row claiming Authored after the entry path closed is a debt still being reported after it was paid",
      field.entity, field.field
    );
  }
}

/// **The wider reading of the second condition discriminates, and this is the
/// measurement that says so.**
///
/// The argument for not measuring entry-by-any-path was that it would fire on
/// every row and so distinguish nothing. It fires on the `Unbuilt` rows and on
/// none of the `State` rows, because a `State` field's values all have exiting
/// edges. Held as a test rather than settled as an argument, since the argument
/// is what let the condition sit unmeasured -- and a future field that broke it
/// would break it silently.
///
/// Every class is asserted non-empty AND the three are asserted to partition
/// `FIELDS` exactly. A walk that found no fields of a kind would agree with every
/// claim above, and **a count guard of the form `> 0` is what failed to notice
/// `Thread.acceptance` leaving the `Unbuilt` population**: 4 became 3 and the
/// assertion held. Summing to the total is the version that cannot be satisfied
/// by a row going missing.
///
/// **Mutation testing turned up a second thing this arm protects, and it was not
/// the reason it was written.** `exitless` is now the one computation behind both
/// conditions, so it is a single point of failure whose natural failure mode is
/// returning nothing -- and every OTHER consumer asserts a set is EMPTY, so a
/// neutered `exitless` makes them pass. Stubbing it to `Vec::new()` leaves
/// `no_state_can_be_entered_and_not_left` green. The `Unbuilt` half below is the
/// only assertion in the file that requires the computation to actually FIND
/// something, which makes it the canary for the primitive the emptiness tests
/// cannot watch. Worth knowing before anyone simplifies it away as redundant
/// with the State half.
#[test]
fn the_wider_reading_fires_on_edgeless_rows_only() {
  let mut state_fields = 0;
  let mut owed_fields = 0;
  let mut immutable_fields = 0;

  for (entity, field, values) in all_fields() {
    let Some(disposition) = find(&entity, &field).map(|f| &f.disposition) else {
      continue;
    };
    assert!(
      !values.is_empty(),
      "{entity}.{field} has a closed domain with no values in it, so nothing below is being tested"
    );
    match disposition {
      Disposition::State { edges, .. } => {
        state_fields += 1;
        assert!(
          exitless(&values, edges).is_empty(),
          "{entity}.{field}: these values have no exiting edge, so the wider reading fires on a State field and the discrimination this test claims is gone: \
           {:?}",
          exitless(&values, edges)
        );
      }
      // Both edgeless variants owe the same exitless property, and they are
      // counted apart because the POPULATIONS are what go short, not the
      // property.
      Disposition::Unbuilt { .. } | Disposition::Immutable { .. } => {
        if matches!(disposition, Disposition::Unbuilt { .. }) {
          owed_fields += 1;
        } else {
          immutable_fields += 1;
        }
        assert_eq!(
          exitless(&values, &[]).len(),
          values.len(),
          "{entity}.{field} declares no edges, so EVERY value of it should be exitless. A subset means the row has acquired edges while still declaring \
           nothing moves the field"
        );
      }
    }
  }

  assert!(
    state_fields > 0 && owed_fields > 0 && immutable_fields > 0,
    "the walk found {state_fields} State, {owed_fields} Unbuilt and {immutable_fields} Immutable fields, and it needs all three to be showing a difference \
     between them rather than a property of an empty set"
  );
  assert_eq!(
    state_fields + owed_fields + immutable_fields,
    FIELDS.len(),
    "the walk classified {state_fields}+{owed_fields}+{immutable_fields} fields and the table declares {}. A row reached no arm, so it is being asserted \
     about by nothing -- which is what a row silently changing disposition looks like",
    FIELDS.len()
  );
}

/// An `Unbuilt` field says what is unavailable -- so the day a mutation for it
/// lands, the disposition is contradicted rather than quietly outliving the gap
/// it described.
///
/// **This asserted `owed_by.starts_with("WP-")` until D37, and it was a test
/// REQUIRING the defect.** The field carried which of Intent's work packages
/// owed the verb, in a library crate another project can depend on, and this
/// assertion made removing it fail the build -- so the guard against the leak
/// would have read as a regression. Second instance in two days of a test
/// defending a superseded model, both found by grepping the tests for the old
/// behaviour after a ruling rather than by reading the diff.
///
/// What remains is the half that was always the useful one: a note a READER can
/// act on. `note` is what a refusal is built from, so an empty one is a refusal
/// that says nothing.
#[test]
fn edgeless_fields_say_what_is_unavailable_and_carry_no_edges() {
  for field in FIELDS {
    let Some(row) = edgeless(field) else {
      continue;
    };
    assert!(
      !row.note.is_empty(),
      "{}.{} declares no edges and must say what is unavailable",
      field.entity,
      field.field
    );
    assert!(
      pm_free(row.note),
      "{}.{} declares no edges and its note names Intent's own project-management state, which a \
       consumer of this crate cannot look up: {}",
      field.entity,
      field.field,
      row.note
    );
  }
}

/// **A row claiming no mutation is owed must cite the ruling that closed it.**
///
/// Without this, `Immutable` is an escape hatch rather than a classification:
/// every `Unbuilt` row is a standing red, and relabelling one clears the red at
/// no cost. Requiring the citation makes the reclassification an act somebody
/// signed -- the same posture `orphans` takes, where a known gap stays recorded
/// with its evidence instead of being tolerated.
///
/// The shape asserted is `<node> <YYYY-MM-DD>`, which this file already uses for
/// Machine 4. It is deliberately weak about WHO: checking the moniker against a
/// roster would put the whiteboard's membership inside a library crate, which is
/// the leak D37 is about. What it enforces is that a human decision was recorded
/// and can be found, not who is allowed to make one.
#[test]
fn a_row_that_owes_no_mutation_cites_the_ruling_that_closed_it() {
  let mut cited = 0;
  for field in FIELDS {
    let Some(row) = edgeless(field) else {
      continue;
    };
    if row.owed {
      assert!(
        row.ruled.is_none(),
        "{}.{} owes a mutation and carries a ruling, so the two claims disagree about whether the gap is a debt",
        field.entity,
        field.field
      );
      continue;
    }
    let ruled = row.ruled.unwrap_or_else(|| {
      panic!(
        "{}.{} owes no mutation and cites no ruling -- which is a red cleared by relabelling it",
        field.entity, field.field
      )
    });
    let (node, date) = ruled
      .split_once(' ')
      .unwrap_or_else(|| panic!("{ruled:?} is not `<node> <YYYY-MM-DD>`"));
    assert!(
      !node.is_empty()
        && date.len() == 10
        && date.split('-').count() == 3
        && date.chars().all(|c| c.is_ascii_digit() || c == '-'),
      "{}.{} cites {ruled:?}, which is not `<node> <YYYY-MM-DD>` -- a citation nobody can look up is not evidence",
      field.entity,
      field.field
    );
    cited += 1;
  }
  assert_eq!(
    cited,
    FIELDS
      .iter()
      .filter(|f| matches!(f.disposition, Disposition::Immutable { .. }))
      .count(),
    "the rows reached here and the Immutable rows in the table are different counts, so a row that owes nothing is having its citation checked by nobody"
  );
}

/// **The population function is itself measured, and by an enumerator that fails
/// the OPPOSITE way.**
///
/// Every immovability check reads its population from [`edgeless`], which makes
/// that function a single point of failure whose natural failure mode is
/// returning `None` -- and `None` means "skip", so a shortfall reads as a pass
/// everywhere downstream. This is the shape that let `Thread.acceptance` leave
/// four populations at **20 passed, 0 failed**.
///
/// So the count is taken twice and the two enumerators fail in opposite
/// directions. [`edgeless`] matches each variant by name, so a new one must be
/// added by hand and is OMITTED until it is; the negation below is `!State`, so a
/// new variant is INCLUDED the moment it exists. They agree exactly when
/// `edgeless` is complete, and the day they disagree the missing arm is the
/// reason -- which is the one fact no downstream check can report, because from
/// where it stands the row simply is not there.
#[test]
fn edgeless_reaches_every_row_that_declares_no_edges() {
  let reached = FIELDS.iter().filter(|f| edgeless(f).is_some()).count();
  let declared = FIELDS
    .iter()
    .filter(|f| !matches!(f.disposition, Disposition::State { .. }))
    .count();
  assert_eq!(
    reached, declared,
    "{declared} rows declare no edges and `edgeless` reaches {reached} of them -- the ones it drops are skipped by every check that reads through it, in \
     silence and in green"
  );
  assert!(
    declared > 0,
    "no row declares no edges, so every immovability check above ranges over an empty set"
  );
}

/// Whether a note is free of Intent's own thread and work-package ids.
///
/// Deliberately narrow and deliberately here rather than shared: this is one
/// last check at the point the string is DECLARED, and the emitted surfaces are
/// covered end-to-end by `intent-cli`'s `no_pm_state_in_output.rs`, which
/// drives the binary instead of reading the source.
fn pm_free(note: &str) -> bool {
  !note.contains("WP-") && !note.contains("ST00")
}

// ---------------------------------------------------------------------------
// The table cannot lie: every declared edge is executed
// ---------------------------------------------------------------------------

/// Every edge in the table is a mutation that exists and moves the field to
/// the value the row claims, from every value the row accepts.
///
/// Without this the table would be prose: a closure check over a graph nobody
/// verified is satisfiable by writing the edges you wish you had. The residual
/// direction is the safe one -- an UNDECLARED mutation adds an edge, and extra
/// edges can only make a graph more closed, so the worst it causes is a false
/// alarm.
#[test]
fn every_declared_edge_is_a_mutation_that_exists() {
  let mut executed = 0;
  for (entity, field, values) in all_fields() {
    let Some(Disposition::State { edges, .. }) = find(&entity, &field).map(|f| &f.disposition)
    else {
      continue;
    };
    for edge in *edges {
      // **`leaves` rather than `accepts`, because a self-loop is not a
      // transition to drive** (hv, 2026-08-17). `at.set` and `wp.rescope` declare
      // an empty from-set, so `accepts` admits the edge's OWN target as a
      // starting state -- and driving `at.set -> to-write` from `to-write` moves
      // nothing, then reads `to-write` back and calls it a pass. The walk would
      // have gone green on a verb that did nothing at all. `leaves` is
      // `accepts(v) && to != v`, which is exactly the distinction the ruling
      // draws.
      for from in values.iter().filter(|v| edge.leaves(v)) {
        let observed = execute(&entity, &field, edge, from);
        assert_eq!(
          observed, edge.to,
          "{entity}.{field}: `{}` from `{from}` should land on `{}` and landed on `{observed}`",
          edge.verb, edge.to
        );
        executed += 1;
      }
    }
  }
  assert!(
    executed >= FIELDS.len(),
    "only {executed} edges were executed -- the dispatcher is skipping rows"
  );
}

const ST: &str = "ST0056";

/// The reason every guarded verb is driven with. One constant so a test that
/// asserts a reason lands can name the same value the driver supplied.
const REASON: &str = "the contract grew after the close";

/// Drive one edge: build a thread with `field` at `from`, apply the verb, and
/// **Driving a DECLARED edge must be a real movement, not a self-loop.**
///
/// Added when self-loops became legal (hv, 2026-08-17): the verbs now answer
/// `Outcome::AlreadyThere` for a non-movement at exit 0, so a fixture that
/// accidentally started in the edge's target state would drive nothing and the
/// walk would still read the expected value back. That is a green built out of
/// two facts that never met, and it is cheaper to refuse it here than to notice
/// it later.
fn assert_movement(entity: &str, field: &str, edge: &Edge, from: &str, outcome: Outcome) {
  // **`outcome.moved()` AND NOT `== Outcome::Moved`, SINCE 2026-08-20.** This
  // asks whether anything HAPPENED, and an equality against one variant asks
  // which outcome it was -- the same question only while `Moved` is the sole
  // moving variant. `Outcome::MovedWith` (AC-05.2's closing note) is a
  // movement, and under the old form `st.done` and `st.cancel` reddened this
  // walk for carrying a warning, which is not a no-op by any reading.
  assert!(
    outcome.moved(),
    "{entity}.{field}: `{}` from `{from}` reported a NO-OP ({outcome:?}), so this walk drove no transition and would still read `{}` back from a fixture that began there",
    edge.verb,
    edge.to
  );
}

/// report where the field landed.
fn execute(entity: &str, field: &str, edge: &Edge, from: &str) -> String {
  match (entity, field) {
    ("Thread", "status") => {
      let fx = Fixture::new();
      fx.write_thread(&thread_with(|t| t.status = parse(from)));
      let mut facade = fx.facade();
      let outcome = match edge.verb {
        "st.triage" => facade.st_triage(ST).expect("st triage"),
        "st.start" => facade.st_start(ST).expect("st start"),
        "st.resume" => facade.st_resume(ST).expect("st resume"),
        "st.hold" => facade.st_hold(ST, REASON).expect("st hold"),
        "st.done" => facade.st_done(ST).expect("st done"),
        "st.cancel" => facade.st_cancel(ST, REASON).expect("st cancel"),
        "st.reopen" => facade.st_reopen(ST, REASON).expect("st reopen"),
        "st.reinstate" => facade.st_reinstate(ST, REASON).expect("st reinstate"),
        // `by` is the invoker recorded on the fiat record, not an actor the
        // walk models; any non-empty value drives the edge.
        "st.fc" => facade.st_fc(ST, REASON, "hv").expect("st fc"),
        other => panic!("no arm drives {other} on Thread.status"),
      };
      assert_movement(entity, field, edge, from, outcome);
      enum_str(&facade.st_show(ST).expect("thread").status).to_string()
    }
    ("WorkPackage", "scope") => {
      let fx = Fixture::new();
      // `absent` is a real state of this field now, and it cannot go through
      // `parse` -- there is no `TShirt` variant for "nobody recorded one". It
      // is entered by INGEST rather than by any verb: a v2 work package
      // predating the frontmatter convention has no `scope:` line, and the
      // migration carries that rather than substituting a size.
      let from_state = (from != ABSENT).then(|| parse(from));
      fx.write_thread(&thread_with(|t| t.wps[1].scope = from_state));
      let mut facade = fx.facade();
      let seq = 3;
      let outcome = match edge.verb {
        "wp.rescope" => facade
          .wp_rescope(ST, seq, parse(edge.to))
          .expect("wp rescope"),
        other => panic!("no arm drives {other} on WorkPackage.scope"),
      };
      assert_movement(entity, field, edge, from, outcome);
      // Read back through the same ABSENT convention the domain uses, so a
      // field that landed nowhere reports the state name rather than tripping
      // `enum_str` on a null.
      match &facade.wp_show(ST, seq).expect("wp").scope {
        Some(scope) => enum_str(scope).to_string(),
        None => ABSENT.to_string(),
      }
    }
    ("WorkPackage", "status") => {
      let fx = Fixture::new();
      fx.write_thread(&thread_with(|t| t.wps[1].status = parse::<WpStatus>(from)));
      let mut facade = fx.facade();
      let seq = 3;
      let outcome = match edge.verb {
        "wp.start" => facade.wp_start(ST, seq).expect("wp start"),
        "wp.unstart" => facade.wp_unstart(ST, seq).expect("wp unstart"),
        "wp.done" => facade.wp_done(ST, seq).expect("wp done"),
        "wp.reopen" => facade.wp_reopen(ST, seq, REASON).expect("wp reopen"),
        "wp.cancel" => facade.wp_cancel(ST, seq, REASON).expect("wp cancel"),
        "wp.reinstate" => facade.wp_reinstate(ST, seq, REASON).expect("wp reinstate"),
        "wp.fc" => facade.wp_fc(ST, seq, REASON, "hv").expect("wp fc"),
        other => panic!("no arm drives {other} on WorkPackage.status"),
      };
      assert_movement(entity, field, edge, from, outcome);
      enum_str(&facade.wp_show(ST, seq).expect("wp").status).to_string()
    }
    ("AcceptanceTest", "status") => {
      let fx = Fixture::new();
      fx.write_thread(&thread_with(|t| t.tests[0].status = parse(from)));
      let mut facade = fx.facade();
      let to: AtStatus = parse(edge.to);
      let outcome = match edge.verb {
        "at.set" => facade.at_set(ST, "AT-03.1", to, None).expect("at set"),
        // The reason is supplied because the edge DECLARES `ReasonRecorded`;
        // the refusal without one is the UNMET walk's job, not this one's.
        "at.fc" => facade
          .at_fc(ST, "AT-03.1", "hv closed it on authority", "hv")
          .expect("at fc"),
        other => panic!("no arm drives {other} on AcceptanceTest.status"),
      };
      assert_movement(entity, field, edge, from, outcome);
      let thread = facade.st_show(ST).expect("thread");
      let test = thread.tests.iter().find(|t| t.id == "AT-03.1").expect("AT");
      enum_str(&test.status).to_string()
    }
    ("Criterion", "state") => {
      let fx = Fixture::new();
      // **The fixture is set up from the VALUE alone now**, where it used to
      // need the verb as well. With two fields, `ac.rescope` had a
      // precondition living on a DIFFERENT field from the one the edge moved,
      // so the driver had to know which scope each verb undid. One field, one
      // precondition.
      //
      // The criterion is driven as `(non-test)` for the authored states and as
      // test-backed for `computed`, which is what makes the two entry values
      // reachable at all.
      // **The kind is read from BOTH endpoints, not just the source.** Coming
      // back into scope lands on `AcState::entry(kind)`, so `descoped ->
      // computed` and `descoped -> unsatisfied` are the same verb on
      // criteria of different kinds -- and a driver that guessed from `from`
      // alone would set up a non-test criterion and then assert it landed on
      // the test-backed value.
      let kind = if from == "computed" || edge.to == "computed" {
        AcKind::Test
      } else {
        AcKind::NonTest
      };
      fx.write_thread(&thread_with(|t| {
        t.criteria[1].kind = kind;
        t.criteria[1].state = state_named(from);
      }));
      // `ac.descope` names ST0057 and its ratified guard requires the target to
      // exist, so the fixture provides one. The guard is exercised on its own
      // in `facade_acceptance.rs`; here it is a precondition, not the subject.
      fx.write_thread(&sample_thread("ST0057"));
      let mut facade = fx.facade();
      apply_ac_verb(&mut facade, edge.verb);
      let c = criterion(&facade);
      // **The evidence assertion that used to live here is DELETED, and its
      // deletion is the result rather than a loss of coverage.** It checked
      // that every edge clearing satisfaction also cleared the evidence -- a
      // rule two assignments had to keep. The evidence now lives inside
      // `Satisfied`, so an edge that leaves `Satisfied` takes it with it and
      // there is no state in which the pair can disagree.
      state_name_of(&c.state).to_string()
    }
    ("Issue", "status") => {
      let fx = Fixture::new();
      // **The fixture's issue is authored at `from`, and the number is 21 rather
      // than 1** so that a verb resolving by position instead of by number
      // cannot pass -- `next_issue_number` is also read from this shape.
      fx.write_issue(&Issue {
        status: parse(from),
        ..sample_issue(21)
      });
      let mut facade = fx.facade();
      let outcome = match edge.verb {
        "issues.close" => facade.issue_close(21).expect("issues close"),
        "issues.open" => facade.issue_open(21).expect("issues open"),
        other => panic!("no arm drives {other} on Issue.status"),
      };
      assert_movement(entity, field, edge, from, outcome);
      enum_str(&facade.issue_show(21).expect("issue").status).to_string()
    }
    other => panic!("no arm drives {other:?} -- every State field needs one"),
  }
}

/// The AC verbs all act on `AC-03.2`, the fixture's non-test criterion.
fn apply_ac_verb(facade: &mut intentsvcs::facade::Facade, verb: &str) -> Outcome {
  const AC: &str = "AC-03.2";
  match verb {
    "ac.satisfy" => facade
      .ac_satisfy(ST, AC, "the render itself")
      .expect("satisfy"),
    "ac.unsatisfy" => facade.ac_unsatisfy(ST, AC).expect("unsatisfy"),
    "ac.descope" => facade
      .ac_descope(ST, AC, "ST0057", Some("hv"), Some("moved"))
      .expect("descope"),
    "ac.withdraw" => facade
      .ac_withdraw(ST, AC, "the premise did not reproduce", Some("hv"))
      .expect("withdraw"),
    "ac.rescope" => facade.ac_rescope(ST, AC).expect("rescope"),
    "ac.reinstate" => facade.ac_reinstate(ST, AC).expect("reinstate"),
    "ac.fc" => facade
      .ac_fc(ST, AC, "hv closed it on authority", "hv")
      .expect("fc"),
    other => panic!("no arm drives {other} on a criterion"),
  }
}

fn criterion(facade: &intentsvcs::facade::Facade) -> Criterion {
  facade
    .st_show(ST)
    .expect("thread")
    .criteria
    .iter()
    .find(|c| c.id == "AC-03.2")
    .expect("AC-03.2")
    .clone()
}

fn thread_with(edit: impl FnOnce(&mut Thread)) -> Thread {
  let mut thread = sample_thread(ST);
  // The gate is consulted by `st done` and `wp done`, and a blocked gate fails
  // these for a reason that is not the one under test. The shared fixture
  // already gates clean -- its test-backed criterion has green cover, its
  // non-test one carries evidence, and the other two are descoped and
  // withdrawn, which are non-blocking by design. So nothing here forces the
  // contract; an earlier version of this function pulled every criterion into
  // scope to be safe and blocked the gate by making two of them uncoverable.
  thread.status = ThreadStatus::Wip;
  edit(&mut thread);
  thread
}

/// Parse a schema value name into its model variant, VIA SERDE -- so the
/// mapping is the one the schema itself declares and cannot drift from it.
fn parse<T: serde::de::DeserializeOwned>(name: &str) -> T {
  serde_json::from_value(Value::String(name.to_string()))
    .unwrap_or_else(|e| panic!("`{name}` is not a value of this field: {e}"))
}

fn state_named(name: &str) -> AcState {
  match name {
    "computed" => AcState::Computed,
    "unsatisfied" => AcState::Unsatisfied { note: None },
    "satisfied" => AcState::Satisfied {
      evidence: "the render itself".to_string(),
    },
    "descoped" => AcState::Descoped {
      to: "ST0057".to_string(),
      by: Some("hv".to_string()),
      reason: None,
    },
    "withdrawn" => AcState::Withdrawn {
      reason: "the premise did not reproduce".to_string(),
      by: None,
    },
    "fiat" => AcState::Fiat(intentsvcs::model::FiatRecord {
      because: "the half it asserts is unobservable by unit test".to_string(),
      by: "hv".to_string(),
      at: "2026-08-29T11:00:00.000Z".to_string(),
      invoker: intentsvcs::model::Invoker {
        tty: true,
        env: "darwin/arm64".to_string(),
      },
      inherited_from: None,
    }),
    other => panic!("no such AC state: {other}"),
  }
}

fn state_name_of(state: &AcState) -> &'static str {
  // **DELEGATED, and the copy that stood here is why `AcState::name` exists.**
  // These five words were a vocabulary owned by a test file and unavailable to
  // production, so issue 0050's no-op line could not have used them -- the fix
  // was to move the home rather than take a second copy. Kept as a named helper
  // because the walk reads better for it.
  state.name()
}

// ---------------------------------------------------------------------------
// The instrument detects the defect it was ruled on
// ---------------------------------------------------------------------------

/// **The discriminator.** A closure check that fails on everything proves
/// nothing about its discrimination, and one that passes on everything proves
/// less. So: the graph AS IT STOOD when hv ruled -- `ac.satisfy` with no
/// inverse and scope changes that moved `scope` alone -- must be reported as a
/// trap on exactly `true`, while `Criterion.scope`, which was already closed
/// because descope and withdraw were each built with their inverse, must come
/// back clean from the same function.
///
/// This is a permanent record rather than a moment in the build: it holds the
/// pre-ruling edge set as data, so the instrument's discrimination stays
/// asserted long after the defect is fixed, without anyone having to un-fix
/// the code to see it.
#[test]
fn the_graph_as_hv_ruled_on_it_reports_the_trap_and_nothing_else() {
  let values: Vec<String> = [ABSENT, "false", "true"]
    .iter()
    .map(|s| s.to_string())
    .collect();
  let before = &[Edge::direct("ac.satisfy", &[ABSENT, "false"], "true")];
  assert_eq!(
    traps(&values, &[ABSENT], before),
    vec!["true".to_string()],
    "the instrument must name the state hv ruled on"
  );

  let scope_values: Vec<String> = ["in-scope", "descoped", "withdrawn"]
    .iter()
    .map(|s| s.to_string())
    .collect();
  let closed = &[
    Edge::direct("ac.descope", &["in-scope"], "descoped"),
    Edge::direct("ac.withdraw", &["in-scope"], "withdrawn"),
    Edge::direct("ac.rescope", &["descoped"], "in-scope"),
    Edge::direct("ac.reinstate", &["withdrawn"], "in-scope"),
  ];
  assert!(
    traps(&scope_values, &["in-scope"], closed).is_empty(),
    "a field that WAS closed must come back clean from the same function, or the check is not discriminating"
  );
}

/// **An incidental edge produces a value and does not discharge a trap.**
///
/// Found by mutation-testing this file: with scope changes clearing
/// satisfaction, deleting `ac.unsatisfy` left `satisfied: true` formally
/// leavable via descope-then-rescope, and the closure check went green over
/// the exact defect hv ruled on. The exit was real and useless -- withdrawing
/// a claim of evidence would mean moving the requirement to another thread and
/// bringing it back, recording two false facts to undo one true one.
///
/// So closure is necessary and not sufficient, and this is the sufficiency:
/// **a state you can only leave by changing a different field is still a state
/// you cannot leave.**
#[test]
fn a_side_effect_of_another_verb_is_not_an_exit() {
  let values: Vec<String> = [ABSENT, "true"].iter().map(|s| s.to_string()).collect();
  let only_incidental = &[
    Edge::direct("ac.satisfy", &[ABSENT], "true"),
    Edge::incidental("ac.rescope", &[], ABSENT, "scope"),
  ];
  assert_eq!(
    traps(&values, &[ABSENT], only_incidental),
    vec!["true".to_string()],
    "an exit that is a side effect of a verb about another field must not discharge the trap"
  );

  let with_direct = &[
    Edge::direct("ac.satisfy", &[ABSENT], "true"),
    Edge::direct("ac.unsatisfy", &["true"], ABSENT),
    Edge::incidental("ac.rescope", &[], ABSENT, "scope"),
  ];
  assert!(
    traps(&values, &[ABSENT], with_direct).is_empty(),
    "and the direct inverse must discharge it"
  );
}

/// A verb that accepts a value and lands back on it is not a way out of it.
/// The distinction `Edge::leaves` draws, asserted directly -- without it a
/// self-edge would silently satisfy the whole AC.
#[test]
fn a_self_edge_is_not_an_exit() {
  let values = vec!["only".to_string()];
  let self_edge = &[Edge::direct("x.set", &[], "only")];
  assert_eq!(
    traps(&values, &["only"], self_edge),
    vec!["only".to_string()]
  );
}

// ---------------------------------------------------------------------------
// CONFORMANCE -- the ratified machines, not merely a closed graph
// ---------------------------------------------------------------------------
//
// AC-04.6 changed shape when hv ratified the state machines on 2026-08-15. It
// used to ask "is the graph closed?", which the code answered about a graph it
// had discovered from itself. It now asks "does the code implement THE ratified
// machine?" -- and the difference is not academic, because the graph that
// passed the closure check declared every edge with `from: &[]`, meaning any
// state. A graph where every verb accepts every state cannot have a trap. It
// was closed, and it let `st done` fire from `cancelled`.

/// The ratified steel-thread machine, transcribed from `data-model.md`
/// INDEPENDENTLY of `transitions.rs`.
///
/// **Two transcriptions of one document, compared.** The production table is
/// what the service layer enforces from, so a test that read it would be asking
/// the implementation to confirm itself. This is the second witness: a typo in
/// either copy is a failure, and the only way to make both wrong in the same
/// way is to make the same mistake twice.
/// One ratified edge as data-model.md states it: verb, from-states, to-state,
/// guards. Named because the tuple appears in five places and clippy is right
/// that the nested form stops being readable at the point it is being compared
/// against another list of the same shape.
type RatifiedEdge = (
  &'static str,
  &'static [&'static str],
  &'static str,
  &'static [Guard],
);

/// One machine: entity, field, and its ratified edges.
type RatifiedMachine = (&'static str, &'static str, &'static [RatifiedEdge]);

const RATIFIED_THREAD: &[RatifiedEdge] = &[
  ("st.triage", &["triage"], "not-started", &[]),
  ("st.start", &["triage", "not-started"], "wip", &[]),
  ("st.resume", &["hold"], "wip", &[]),
  (
    "st.hold",
    &["not-started", "wip"],
    "hold",
    &[Guard::ReasonRecorded],
  ),
  ("st.done", &["wip"], "completed", &[Guard::GatePass]),
  // **THE SAME FROM-STATE AND THE SAME LANDING AS `st.done`, AND A DIFFERENT
  // GUARD** (hv, D1, 2026-08-29). The pair is the whole point: two edges into
  // `completed`, one that requires the gate to pass and one that exists because
  // it did not. Relaxing `st.done` to accept either guard was the option hv
  // declined, and this row is what makes the refusal checkable.
  ("st.fc", &["wip"], "completed", &[Guard::ReasonRecorded]),
  (
    "st.cancel",
    &["triage", "not-started", "wip", "hold"],
    "cancelled",
    &[Guard::ReasonRecorded],
  ),
  ("st.reopen", &["completed"], "wip", &[Guard::ReasonRecorded]),
  (
    "st.reinstate",
    &["cancelled"],
    "not-started",
    &[Guard::ReasonRecorded],
  ),
];

/// The ratified acceptance-criterion machine, same discipline.
///
/// **`ac.rescope` and `ac.reinstate` each appear TWICE**, landing on
/// `unsatisfied` or `computed`. That is not a transcription error: coming back
/// into scope lands on `AcState::entry(kind)`, and the ratified table's single
/// "-> Unsatisfied" row is written for the authored criterion it had in mind.
/// The second row is the test-backed case the same rule implies -- stated here
/// rather than left implicit, so a disagreement surfaces as a failing test.
const RATIFIED_CRITERION: &[RatifiedEdge] = &[
  (
    "ac.satisfy",
    &["unsatisfied"],
    "satisfied",
    // **Two guards, and the second is the one the single-valued column could
    // not hold.** Evidence was ruled required and recorded as structural; a
    // `String` field only made it mandatory, so `ac satisfy` with no evidence
    // wrote a satisfaction the gate counted (ic, 2026-08-15). Ratified here as
    // part of the machine so the transcription check holds the code to it.
    &[Guard::NonTestOnly, Guard::EvidenceRecorded],
  ),
  (
    "ac.unsatisfy",
    &["satisfied"],
    "unsatisfied",
    &[Guard::NonTestOnly],
  ),
  (
    "ac.descope",
    &["computed", "unsatisfied", "satisfied"],
    "descoped",
    &[Guard::TargetExists],
  ),
  (
    "ac.withdraw",
    &["computed", "unsatisfied", "satisfied"],
    "withdrawn",
    &[Guard::ReasonRecorded],
  ),
  ("ac.rescope", &["descoped"], "unsatisfied", &[]),
  ("ac.rescope", &["descoped"], "computed", &[]),
  ("ac.reinstate", &["withdrawn"], "unsatisfied", &[]),
  ("ac.reinstate", &["withdrawn"], "computed", &[]),
  // **The fiat exit, ruled by hv 2026-08-29: the SAME verb, not a new one.**
  // Reinstating already means bringing a requirement closed-without-being-met
  // back into play, and a fiat close is closed-without-being-met by definition.
  //
  // **THE TRANSCRIPTION WAS AHEAD OF ITS SOURCE AND IS NOT ANY MORE.** This
  // note used to say `data-model.md`'s Machine 3 still listed eight edges, that
  // the two witnesses therefore disagreed, and that **nothing detected it** --
  // this check reads only the list below, so the divergence was invisible to
  // the very mechanism built to catch divergence. Recording the ruling in the
  // doc was routed rather than taken, and it has since landed: the table now
  // carries both exits and both entries, and `machine_table_check.sh` compares
  // them. The note is rewritten rather than deleted because the mechanism it
  // describes is still real -- **this list is a hand transcription and the
  // check cannot see a disagreement with the document it transcribes.**
  ("ac.reinstate", &["fiat"], "unsatisfied", &[]),
  ("ac.reinstate", &["fiat"], "computed", &[]),
  // **THE ENTRY EDGES, landing with the verb** (`fc`, ic's surface ruling; the
  // machine's op token is `ac.fc`). From the two OPEN states only: `satisfied`
  // is deliberately absent, because a fiat close is how an UNMET requirement is
  // closed and offering it on a met one would let a close-on-authority
  // overwrite a close-on-evidence.
  (
    "ac.fc",
    &["computed", "unsatisfied"],
    "fiat",
    &[Guard::ReasonRecorded],
  ),
];

/// The ratified work-package machine, same discipline.
const RATIFIED_WP: &[RatifiedEdge] = &[
  ("wp.start", &["not-started"], "wip", &[]),
  ("wp.unstart", &["wip"], "not-started", &[]),
  ("wp.done", &["wip"], "done", &[Guard::GatePass]),
  ("wp.fc", &["wip"], "done", &[Guard::ReasonRecorded]),
  ("wp.reopen", &["done"], "wip", &[Guard::ReasonRecorded]),
  // hv, 2026-08-21: `Cancelled` at WP level, after a live consumer hit the
  // deadlock the original ruling created. NOT `GatePass` -- this verb is the
  // announcement that there is no contract to gate on.
  (
    "wp.cancel",
    &["not-started", "wip", "done"],
    "cancelled",
    &[Guard::ReasonRecorded],
  ),
  (
    "wp.reinstate",
    &["cancelled"],
    "not-started",
    &[Guard::ReasonRecorded],
  ),
];

/// The ratified issue machine -- **Machine 4** (hv, 2026-08-17).
///
/// No guards on either edge, and the empty lists are the ratified fact rather
/// than an unfilled column: v2 has none, `intent issues` is `keep`, and hv ruled
/// that inventing one here "would be a parity break wearing a ratification".
const RATIFIED_ISSUE: &[RatifiedEdge] = &[
  ("issues.close", &["open"], "closed", &[]),
  ("issues.open", &["closed"], "open", &[]),
];

/// The ratified acceptance-test machine -- **Machine 5** (hv, 2026-08-29, D1).
///
/// **This machine was ratified in PROSE until 2026-08-29 and is the only one that
/// has ever moved between the two rosters.** It sat in
/// `RATIFIED_WITHOUT_A_TABLE` on the stated ground that its graph was "any value,
/// one verb, any value"; `at.fc` is a second verb with a from-set AND a guard, so
/// the sentence became false and `a_machine_ratified_in_prose_is_actually_trivial`
/// went red on the commit that added the verb. **That is the trigger working, not
/// the trigger being wrong** -- it is a dormant assertion on a dormant assumption,
/// and the whole point of writing it was that somebody would one day make the
/// prose untrue without noticing.
///
/// **`at.set` declares an EMPTY from-set on all four ordinary landings, which is
/// `from: &[]` meaning no restriction rather than an unfilled column.** The
/// ratified table renders it `(any)`. Writing the five states out instead would
/// transcribe a different machine -- one where a sixth value would owe five new
/// rows.
///
/// **The `fiat` landing takes `ReasonRecorded` and never `GatePass`**, which is
/// hv's D1 in one line: a fiat close is BY DEFINITION the case where the gate does
/// not pass.
const RATIFIED_AT: &[RatifiedEdge] = &[
  ("at.set", &[], "to-write", &[]),
  ("at.set", &[], "red", &[]),
  ("at.set", &[], "green", &[]),
  ("at.set", &[], "n-a", &[]),
  (
    "at.fc",
    &["to-write", "red"],
    "fiat",
    &[Guard::ReasonRecorded],
  ),
];

/// **EVERY ratified machine, in ONE array, read by all three walks** (vc's
/// recommendation, 2026-08-17).
///
/// The three walks each carried their own hand-written entity list, and two of
/// them had gone short: Machine 4 was in the first and absent from the other two.
/// **Both omissions were free**, and that is the worst state for them -- walk 3's
/// because Machine 4 is total, and the justification guard's because
/// `RATIFIED_ISSUE` declares no guards. Neither would red when it stopped being
/// free.
///
/// The precedent is in this file: `Criterion` missing from the justification loop
/// left `ac.withdraw`'s ratified `ReasonRecorded` transcribed, conformance-checked
/// and dead. **That repair derived the VERB axis from the tables and left the
/// ENTITY axis hand-written, while its comment reads as though both were done** --
/// and the incident it describes was an entity omission. This is the other axis.
const RATIFIED: &[RatifiedMachine] = &[
  ("Thread", "status", RATIFIED_THREAD),
  ("WorkPackage", "status", RATIFIED_WP),
  ("Criterion", "state", RATIFIED_CRITERION),
  ("Issue", "status", RATIFIED_ISSUE),
  ("AcceptanceTest", "status", RATIFIED_AT),
];

/// **Machines `data-model.md` ratifies IN PROSE rather than as an edge table**,
/// with the sentence that does it.
///
/// Found by binding `RATIFIED` to `transitions::FIELDS`: six State machines exist
/// and five have transcriptions. The one missing is not an omission --
/// `data-model.md` declines it a table on the record, and quoting the sentence is
/// what makes that checkable rather than something a reader has to go and confirm.
///
/// **This list held TWO rows until 2026-08-30 and the second one left by being
/// PROMOTED, which is the outcome it was written to produce.**
/// `AcceptanceTest.status` is now Machine 5, transcribed in `RATIFIED_AT` above.
/// A row leaving here is the success case; a row quietly staying while its machine
/// grows a verb is the failure this roster exists to make impossible.
///
/// **Attributed to the document, not to a node, and that is a correction.** This
/// said "vc declines them"; every commit in this repository is authored `Matthew
/// Sinclair`, so `git log` cannot separate the nodes and the claim was one the
/// repository could not confirm. A ratified sentence's authority comes from its
/// ratification, not from who typed it.
///
/// **It is held to the property the prose states instead**, by
/// `a_machine_ratified_in_prose_is_actually_trivial` below: one verb, no
/// from-restriction, reaching every state. A machine that grows a second verb or a
/// guard has stopped being "any value, one verb, any value" and needs a table --
/// and that test is what says so, rather than the machine quietly becoming
/// non-trivial with nothing comparing it to anything.
const RATIFIED_WITHOUT_A_TABLE: &[(&str, &str, &str)] = &[(
  "WorkPackage",
  "scope",
  "data-model.md: \"six T-shirt values, all six initial ... with `wp rescope` the single exit. \
     The reasoning lives in the code comment; it does not need a table because the graph is 'any \
     value, one verb, any value'.\"",
)];

/// Machines with NO undeclared (verb, state) pair, declared with the reason.
///
/// **A structural zero and an unvisited machine produce the same output -- none
/// -- so the zero is declared rather than inferred** (vc's second point). A
/// machine in this list is required to contribute zero pairs; one absent from it
/// is required to contribute some. Either way the number is checked, which is what
/// makes the vacuity readable instead of something a reader has to recompute by
/// hand.
const TOTAL_MACHINES: &[(&str, &str)] = &[(
  "Issue",
  "two states and two edges, so every (verb, state) pair is either declared or the verb's own \
   target -- there is nothing undeclared left to refuse",
)];

/// **Verbs whose ratified from-set is EMPTY BY DECLARATION** -- `(any)` in the
/// document's From column, `from: &[]` in the code -- with the reason.
///
/// **THE TWO FILES DISAGREED ABOUT WHAT AN EMPTY FROM-SET MEANS, and nothing could
/// see it while no ratified machine used one.** `transitions.rs` says `Edge::accepts`
/// reads `from: &[]` as ANY state; the walk below reads a verb's from-set as the
/// complete list of states it is declared from, so an empty one made every state
/// undeclared and demanded a refusal from all of them. Both readings are defensible
/// and exactly one can be true. Machine 5 is the first ratified machine to carry
/// such an edge, and promoting it is what forced the question -- `at.set` from
/// `fiat` is the documented exit from `fiat`, and the walk asked it to refuse.
///
/// **The resolution is the code's reading, and the zero it produces is DECLARED
/// rather than inferred** -- the discipline `TOTAL_MACHINES` above already applies
/// one level up. A verb with no from-restriction has no undeclared (verb, state)
/// pair, so it contributes nothing to this walk; and a contribution of nothing is
/// indistinguishable from a verb the walk never reached unless the zero is written
/// down. Checked in both directions below: an empty from-set not listed here fails,
/// and a row here whose verb is not actually unrestricted fails too.
const UNRESTRICTED_VERBS: &[(&str, &str, &str)] = &[(
  "AcceptanceTest",
  "at.set",
  "data-model.md Machine 5 renders its From column `(any)`: at.set takes any status from any \
   status, and all four ordinary landings are therefore the exits that keep `fiat` from being a \
   trap. Enumerating the five states instead would declare a DIFFERENT machine -- one where a \
   sixth status would owe five new rows rather than none",
)];

fn declared(entity: &str, field: &str) -> &'static [Edge] {
  match find(entity, field).map(|f| &f.disposition) {
    Some(Disposition::State { edges, .. }) => edges,
    _ => panic!("{entity}.{field} is not a State"),
  }
}

/// **RENAMED, and the old name was the defect** (vc, 2026-08-17).
///
/// This was `the_implemented_graph_is_the_ratified_one_edge_for_edge`, and it
/// compares `declared()` -- the transition TABLE -- against the `RATIFIED_*`
/// consts transcribed from data-model.md. **Neither side is the
/// implementation.** It cannot see code, so code's absence cannot red it: it was
/// green while the table carried Machine 4's edges and no facade verb drove them.
///
/// The property "the implemented graph is the ratified one" holds across three
/// tests and no one of them: this one (table matches ratified),
/// `every_declared_edge_is_a_mutation_that_exists` (every declared edge DRIVES),
/// and `a_transition_the_ratified_machine_does_not_declare_is_refused` (every
/// undeclared pair REFUSES). **They should not be merged** -- three assertions
/// failing for three distinguishable reasons is worth more than one failing for
/// any of them. AT-04.6's closing condition now states the three behaviours
/// rather than naming a test, so a rename here cannot invalidate it.
/// **Every State machine in the code is ratified SOMEHOW, and the two ways are
/// distinguished.** A machine with a table gets one; a machine ratified in prose
/// gets the prose property checked. Nothing gets neither.
///
/// This is the entity axis closed at its root rather than at three loops. Without
/// it, deleting a row from [`RATIFIED`] makes all three walks skip that machine in
/// silence -- the omission this file has now been caught by twice, on `Criterion`
/// and on `Issue`. **And writing it found two more**: six State machines exist and
/// four had transcriptions, so AT-04.6's "the implemented graph matches the
/// ratified machines exactly" was being discharged over two thirds of them.
#[test]
fn every_state_machine_is_ratified_by_a_table_or_by_prose_and_nothing_by_neither() {
  let in_code: std::collections::BTreeSet<(String, String)> = intentsvcs::transitions::FIELDS
    .iter()
    .filter(|f| matches!(f.disposition, Disposition::State { .. }))
    .map(|f| (f.entity.to_string(), f.field.to_string()))
    .collect();
  let tabled: std::collections::BTreeSet<(String, String)> = RATIFIED
    .iter()
    .map(|(entity, field, _)| (entity.to_string(), field.to_string()))
    .collect();
  let prose: std::collections::BTreeSet<(String, String)> = RATIFIED_WITHOUT_A_TABLE
    .iter()
    .map(|(entity, field, _)| (entity.to_string(), field.to_string()))
    .collect();

  // Disjoint, because a machine in both would be checked two ways that could
  // disagree, and the prose check is the WEAKER one -- so the overlap would
  // quietly become the only thing enforced if the table were later dropped.
  let both: Vec<_> = tabled.intersection(&prose).collect();
  assert!(
    both.is_empty(),
    "a machine is ratified by a table OR in prose, not both: {both:?}"
  );

  let covered: std::collections::BTreeSet<(String, String)> =
    tabled.union(&prose).cloned().collect();
  assert_eq!(
    in_code, covered,
    "the State fields in `transitions.rs` and the ratified machines must be the same set -- one \
     missing is a machine every walk skips in silence, and one extra is a rule about a machine that \
     no longer exists"
  );
}

/// **A machine ratified in prose is held to what the prose says**: one verb, no
/// from-restriction, reaching every state.
///
/// That is "any value, one verb, any value" stated mechanically. It is a weaker
/// check than a transcription and it is the RIGHT weaker check: the document's
/// reason for declining a table is that the graph carries no information, so the
/// thing to verify is precisely that claim. **Attributed to the document rather
/// than to a node, for the reason `RATIFIED_WITHOUT_A_TABLE` states above** -- this
/// read "vc's reason", and it is the same claim the repository cannot confirm,
/// one paragraph down from where it was already corrected. **The moment one grows a second verb, a
/// guard, or a from-restriction the claim is false, the machine has information in
/// its graph again, and this says so** -- which is the trigger for writing the
/// table rather than a thing to notice later.
#[test]
fn a_machine_ratified_in_prose_is_actually_trivial() {
  for (entity, field, sentence) in RATIFIED_WITHOUT_A_TABLE {
    let edges = declared(entity, field);
    let verbs: std::collections::BTreeSet<&str> = edges.iter().map(|e| e.verb).collect();
    assert_eq!(
      verbs.len(),
      1,
      "{entity}.{field} is ratified in prose as ONE verb and declares {verbs:?}. It now needs an \
       edge table in data-model.md -- the sentence standing in for one says: {sentence}"
    );
    for edge in edges {
      assert!(
        edge.from.is_empty(),
        "{entity}.{field}: `{}` -> `{}` is declared from {:?}, so the graph restricts something and \
         is no longer 'any value, one verb, any value'. It needs a table: {sentence}",
        edge.verb,
        edge.to,
        edge.from
      );
    }
    let targets: std::collections::BTreeSet<&str> = edges.iter().map(|e| e.to).collect();
    let states = initial_states(entity, field);
    let unreachable: Vec<&&str> = states
      .iter()
      .filter(|s| !targets.contains(**s) && **s != "absent")
      .collect();
    assert!(
      unreachable.is_empty(),
      "{entity}.{field}: the one verb must reach every state, and {unreachable:?} cannot be reached \
       -- which is the trap condition AC-04.6 forbids, hiding inside a machine nobody transcribed: \
       {sentence}"
    );
  }
}

/// The states a field can be entered in, from the declaration.
fn initial_states(entity: &str, field: &str) -> &'static [&'static str] {
  match find(entity, field).map(|f| &f.disposition) {
    Some(Disposition::State { initial, .. }) => initial,
    _ => panic!("{entity}.{field} is not a State"),
  }
}

#[test]
fn the_transition_table_transcribes_the_ratified_machines_edge_for_edge() {
  for (entity, field, ratified) in RATIFIED {
    let implemented = declared(entity, field);
    assert_eq!(
      implemented.len(),
      ratified.len(),
      "{entity}.{field} declares {} edges and the ratified machine has {} -- an EXTRA edge is as much a divergence as a missing one, because it is a transition nobody approved",
      implemented.len(),
      ratified.len()
    );
    for (verb, from, to, guard) in ratified.iter() {
      // **THE KEY IS (verb, to, FROM), AND IT USED TO BE (verb, to) BY
      // ACCIDENT RATHER THAN BY DESIGN.** No two ratified edges shared a verb
      // and a target until `ac.reinstate` gained a second source, so the
      // two-part lookup was unique for reasons nothing here asserted. The
      // moment it was not, `find` returned the FIRST match and the `from`
      // assertion below compared a ratified edge against a different declared
      // one -- reporting a divergence that did not exist while silently not
      // checking the edge it was actually looking at.
      //
      // A property holding because another mechanism happened to make its key
      // unique is the class `IN-AG-RED-CONTROL-001` names: enforcement
      // inherited from an accident expires silently when the accident does.
      // Matching on all three makes existence the check that `from` used to be,
      // and the guard assertion below stays the part a success path never sees.
      let found = implemented
        .iter()
        .find(|e| e.verb == *verb && e.to == *to && e.from == *from)
        .unwrap_or_else(|| {
          panic!("{entity}.{field}: the ratified machine has `{verb}` from {from:?} -> `{to}` and the code declares no such edge")
        });
      assert_eq!(
        found.guard, *guard,
        "{entity}.{field}: `{verb}` is ratified with {guard:?} and declared with {:?} -- a guard is the half a success-path test never sees",
        found.guard
      );
    }
  }
}

// ---------------------------------------------------------------------------
// GUARDS UNMET -- the half of every transition a success-path walk never sees
// ---------------------------------------------------------------------------

/// The descope target used to make [`Guard::TargetExists`] unmet.
///
/// A thread id no fixture writes. It perturbs the ARGUMENT rather than the
/// estate, which is what keeps the refusal attributable: the fixture still
/// carries `ST0057`, so nothing else about it has changed.
const MISSING_THREAD: &str = "ST9999";

/// **How a declared guard is made UNMET, so a refusal can be demanded of it.**
///
/// One variant per MECHANISM rather than one per verb, so two verbs declaring
/// the same guard are made to fail the same way and a reader can see that they
/// are.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Unmet {
  /// Hand the verb a justification that is empty, and again one that is only
  /// spaces. **Two probes, because a shell makes `--reason ""` and
  /// `--reason "  "` the same gesture**, so a guard that refuses one and stores
  /// the other teaches its own bypass.
  BlankJustification,
  /// Leave one in-scope criterion unsatisfied, so the close gate BLOCKS.
  GateBlocked,
  /// Point the verb at a TEST-BACKED criterion, which the guard excludes.
  TestBackedCriterion,
  /// Name a descope target that is not a thread in this project.
  MissingTarget,
  /// Hand the descope a BLANK target, which is a different mistake from naming
  /// one that does not exist -- see [`MODES`].
  BlankTarget,
}

/// **The unmet-fixture table: how every declared `(verb, guard)` pair is driven
/// with its guard unmet, and why that makes the REFUSAL ATTRIBUTABLE TO THE
/// GUARD.**
///
/// This table is the reviewable artefact, not the test that reads it. The test
/// is a loop; the judgement is here, one row at a time, in a form somebody can
/// disagree with.
///
/// **A `(verb, guard)` pair with no row REFUSES rather than skipping** -- the
/// same construction `execute` uses for a `State` field with no drive arm. A
/// skip would mean a guard nobody can test, reported as a pass, which is the
/// shape the whole file exists to refuse.
///
/// **THE KEY IS `(verb, guard, mode)`, NOT `(verb, guard)`, AND THAT IS vc's
/// CORRECTION TO THE DESIGN** (2026-08-17). A guard can be unmet in more than
/// one way, with a different refusal for each, and the declaration table cannot
/// see that -- `transitions.rs` knows `ac.descope` is guarded by
/// `TargetExists` and nothing more. **So the two sources are the declaration
/// table for WHICH pairs exist and the error enum for HOW MANY WAYS each is
/// unmet, and neither completes this table alone.** The mode counts are declared
/// in [`MODES`], because a count nobody wrote down is a count nobody can be
/// wrong about.
///
/// **The fourth column is load-bearing and it is not documentation.** Requiring
/// a refusal is not enough on its own: drive `ac.satisfy` from a state it is not
/// declared from and it refuses for the FROM-STATE, and a walk asking only
/// `is_err()` would credit that to the guard. So each row says what makes its
/// refusal the guard's, and the assertion checks the error variant rather than
/// the mere fact of failure.
const UNMET: &[(&str, Guard, Unmet, &str)] = &[
  (
    "st.hold",
    Guard::ReasonRecorded,
    Unmet::BlankJustification,
    "driven from `not-started`, a declared from-state, so the transition check cannot be what refuses",
  ),
  (
    "st.cancel",
    Guard::ReasonRecorded,
    Unmet::BlankJustification,
    "driven from `triage`, a declared from-state",
  ),
  (
    "st.reopen",
    Guard::ReasonRecorded,
    Unmet::BlankJustification,
    "driven from `completed`, its only declared from-state",
  ),
  (
    "st.fc",
    Guard::ReasonRecorded,
    Unmet::BlankJustification,
    "driven from `wip`, its only declared from-state, so the transition check cannot be what refuses. \
     **This is the guard whose absence would be worst**: a fiat close with no reason is a close on \
     human authority with nothing recording whose, and the attribution posture is the whole design",
  ),
  (
    "st.reinstate",
    Guard::ReasonRecorded,
    Unmet::BlankJustification,
    "driven from `cancelled`, its only declared from-state",
  ),
  (
    "wp.cancel",
    Guard::ReasonRecorded,
    Unmet::BlankJustification,
    "driven from `wip`, a declared from-state, so the transition check cannot be what refuses",
  ),
  (
    "wp.fc",
    Guard::ReasonRecorded,
    Unmet::BlankJustification,
    "driven from `wip`, its only declared from-state; same reasoning as `st.fc`",
  ),
  (
    "wp.reinstate",
    Guard::ReasonRecorded,
    Unmet::BlankJustification,
    "driven from `cancelled`, its only declared from-state",
  ),
  (
    "wp.reopen",
    Guard::ReasonRecorded,
    Unmet::BlankJustification,
    "driven from `done`, its only declared from-state",
  ),
  (
    "ac.withdraw",
    Guard::ReasonRecorded,
    Unmet::BlankJustification,
    "the reason lives INSIDE `Withdrawn`, so a blank one is the only way to ask for the state without it",
  ),
  (
    "ac.fc",
    Guard::ReasonRecorded,
    Unmet::BlankJustification,
    "the reason lives INSIDE `Fiat`, exactly as it does inside `Withdrawn`, so a blank `because` is the only way to ask for the state without one -- and it is the case AC-00.1 names: invoked without a reason, `fc` refuses and writes nothing",
  ),
  (
    "at.fc",
    Guard::ReasonRecorded,
    Unmet::BlankJustification,
    "the AT's record is a FIELD beside its status, not a payload inside it, so a blank `--because` is the whole of the unmet case -- and the refusal must be the reason guard rather than the transition check, which is why the fixture sits at `to-write`, a declared from-state",
  ),
  (
    "ac.satisfy",
    Guard::EvidenceRecorded,
    Unmet::BlankJustification,
    "blank evidence on a non-test criterion at `unsatisfied` -- and the refusal must say EVIDENCE, not reason, or the two guards are one guard with two names",
  ),
  (
    "st.done",
    Guard::GatePass,
    Unmet::GateBlocked,
    "the thread is `wip` and the gate blocks, so the self-loop test and the transition check both pass first and the gate is what is left",
  ),
  (
    "wp.done",
    Guard::GatePass,
    Unmet::GateBlocked,
    "the same contract seen through the WP-03 scope, which is the other of the two call sites where this guard was decorative",
  ),
  (
    "ac.satisfy",
    Guard::NonTestOnly,
    Unmet::TestBackedCriterion,
    "ORDER IS WHAT MAKES THIS DRIVABLE: a test-backed criterion is always `computed`, which `ac.satisfy` is not declared from, so BOTH checks would refuse -- the kind check runs first and `ComputedSatisfaction` is the proof of which one fired",
  ),
  (
    "ac.unsatisfy",
    Guard::NonTestOnly,
    Unmet::TestBackedCriterion,
    "same ordering argument, and the reason `ComputedSatisfaction` is not delegated to this guard: there is no self-loop for it to shadow, because a test-backed criterion can never be at this verb's target",
  ),
  (
    "ac.descope",
    Guard::TargetExists,
    Unmet::MissingTarget,
    "the ARGUMENT is perturbed and the estate is not, so the fixture still carries `ST0057` and only the named target is missing",
  ),
  (
    "ac.descope",
    Guard::TargetExists,
    Unmet::BlankTarget,
    "the SECOND mode of the same guard, and the one a `(verb, guard)` key would have marked covered: a blank target is a different mistake and gets a different refusal",
  ),
];

/// **Pairs whose guard has more than one unmet MODE, with the reason there are
/// that many.** Anything absent from here has exactly one.
///
/// The count is declared rather than inferred for the reason `TOTAL_MACHINES` is:
/// **one row per pair and several modes per guard produce the same output from
/// the declaration side -- a covered pair -- and only one of them is complete.**
/// The number here is the authored claim, and the driver checks the rows against
/// it, so a guard that grows a third refusal is a one-line edit somebody has to
/// make rather than a gap nothing reports.
const MODES: &[(&str, Guard, usize, &str)] = &[(
  "ac.descope",
  Guard::TargetExists,
  2,
  "`facade.rs` splits them deliberately -- an ABSENT target is not a target that does not exist, and reporting one as the other produces a message with a hole \
   in it. Clap refuses a truly missing `--to`, so what reaches the facade is blank or named, and each gets its own refusal",
)];

/// The justifications one unmet mechanism is driven with.
///
/// Exhaustive on [`Unmet`], so a new mechanism has to say how many probes it
/// takes rather than silently taking the default.
fn probes(unmet: Unmet) -> &'static [&'static str] {
  match unmet {
    Unmet::BlankJustification => &["", "   "],
    Unmet::GateBlocked | Unmet::TestBackedCriterion | Unmet::MissingTarget | Unmet::BlankTarget => {
      &[REASON]
    }
  }
}

/// The descope target one unmet mechanism is driven with.
///
/// `BlankTarget` uses SPACES rather than the empty string, deliberately: clap's
/// `required` already refuses an absent `--to`, so the case that reaches the
/// facade is the trimmable one, and an empty string would exercise a shorter
/// path than the operator can actually take.
fn target(unmet: Option<Unmet>) -> &'static str {
  match unmet {
    Some(Unmet::MissingTarget) => MISSING_THREAD,
    Some(Unmet::BlankTarget) => "   ",
    _ => "ST0057",
  }
}

/// **Whether a refusal is the one THIS guard, unmet THIS way, owes.**
///
/// The whole strength of the driver is here. A guard-unmet walk asserting
/// `is_err()` is satisfiable by any refusal at all -- including the from-state
/// refusal, which is present on some of these fixtures by construction -- so it
/// would go green on a guard that had been deleted from the code and left in the
/// table. That is the decorative-declaration failure `Guard::GatePass` was
/// caught in, one layer along. **vc's sharpening: every guard already has its own
/// `FacadeError` variant, one to one, because somebody went out of their way to
/// keep the remedies apart -- so asserting refusal-ness discards exactly the
/// distinction the code was written to preserve.**
///
/// **Exhaustive on [`Guard`], and that is the point of writing the outer match on
/// the guard rather than on the pair.** A guard variant added to the model is a
/// compile error HERE, which is the opposite failure direction from [`UNMET`]:
/// that table is matched by name and omits a new pair until somebody adds a row,
/// while this includes it immediately and refuses to build. Two enumerators that
/// go short in opposite directions is the construction; one of them alone is a
/// threshold.
fn names_the_guard(guard: Guard, unmet: Unmet, err: &FacadeError) -> bool {
  match guard {
    Guard::ReasonRecorded => matches!(err, FacadeError::ReasonRequired { .. }),
    Guard::EvidenceRecorded => matches!(err, FacadeError::EvidenceRequired { .. }),
    Guard::GatePass => matches!(err, FacadeError::GateBlocked { .. }),
    Guard::NonTestOnly => matches!(err, FacadeError::ComputedSatisfaction { .. }),
    // The one guard with two modes and two refusals. The inner match names both
    // rather than defaulting, so a mode paired with this guard by mistake says so
    // instead of being scored against whichever variant happened to be the
    // fallback.
    Guard::TargetExists => match unmet {
      Unmet::MissingTarget => matches!(err, FacadeError::DescopeTargetMissing { .. }),
      Unmet::BlankTarget => matches!(err, FacadeError::DescopeTargetRequired { .. }),
      other => panic!(
        "{other:?} is not a way to make `TargetExists` unmet -- the row pairs a guard with a mechanism that cannot break it, so whatever it refused with was about \
         something else"
      ),
    },
  }
}

/// The fixture half of an unmet guard, applied after the from-state is set.
///
/// Exhaustive, including the `None` and argument-only cases, so a new mechanism
/// cannot land in a silent arm and quietly drive a CLEAN fixture -- which would
/// pass or fail for reasons having nothing to do with the guard.
fn perturb(thread: &mut Thread, unmet: Option<Unmet>) {
  match unmet {
    // These perturb the ARGUMENTS; the estate they are driven against is the
    // clean one.
    None
    | Some(Unmet::BlankJustification)
    | Some(Unmet::MissingTarget)
    | Some(Unmet::BlankTarget) => {}
    Some(Unmet::GateBlocked) => {
      // **AC-03.2 is the fixture's only criterion whose satisfaction is STORED**,
      // so unsatisfying it reaches a `Blocked` verdict without touching the AT
      // roster. Reddening the tests covering AC-03.1 would also block, and it
      // could block as an AT CONTRACT FINDING instead -- a refusal with the right
      // variant and the wrong cause, which is the failure this column exists to
      // rule out.
      thread.criteria[1].state = AcState::Unsatisfied { note: None };
    }
    Some(Unmet::TestBackedCriterion) => {
      // Both fields together, because the pairing is enforced in the schema face:
      // flipping `kind` alone is invalid canon and the fixture would be refused
      // by ingest rather than by the guard.
      thread.criteria[1].kind = AcKind::Test;
      thread.criteria[1].state = AcState::Computed;
    }
  }
}

/// Drive `verb` from `from`, and report the refusal if there was one.
///
/// `unmet` is `None` for the undeclared-transition walk, which wants a clean
/// fixture and a refusal that can only be about the STATE, and `Some` for the
/// guard walk, which wants exactly one precondition broken.
fn attempt(
  entity: &str,
  verb: &str,
  from: &str,
  justification: &str,
  unmet: Option<Unmet>,
) -> Result<Outcome, FacadeError> {
  let fx = Fixture::new();
  let mut facade: Facade = match entity {
    "Thread" => {
      fx.write_thread(&thread_with(|t| {
        t.status = parse(from);
        perturb(t, unmet);
      }));
      fx.facade()
    }
    "WorkPackage" => {
      fx.write_thread(&thread_with(|t| {
        t.wps[1].status = parse::<WpStatus>(from);
        perturb(t, unmet);
      }));
      fx.facade()
    }
    "Criterion" => {
      let kind = if from == "computed" {
        AcKind::Test
      } else {
        AcKind::NonTest
      };
      fx.write_thread(&thread_with(|t| {
        t.criteria[1].kind = kind;
        t.criteria[1].state = state_named(from);
        perturb(t, unmet);
      }));
      fx.write_thread(&sample_thread("ST0057"));
      fx.facade()
    }
    "AcceptanceTest" => {
      fx.write_thread(&thread_with(|t| {
        t.tests[0].status = parse(from);
        perturb(t, unmet);
      }));
      fx.facade()
    }
    other => panic!("no fixture for {other}"),
  };
  let seq = 3;
  const AC: &str = "AC-03.2";
  match verb {
    // **THE JUSTIFICATION IS THE `because`, AND THAT IS WHAT MAKES THE BLANK
    // PROBE REACH THIS GUARD.** An AT's fiat record sits BESIDE its status
    // rather than inside it, so unlike `ac.fc` there is no state payload to
    // leave empty -- the argument is the only channel, and a blank one is
    // exactly the AC-00.1 case: invoked without a reason, `fc` refuses and
    // writes nothing.
    "at.fc" => facade.at_fc(ST, "AT-03.1", justification, "hv"),
    "ac.satisfy" => facade.ac_satisfy(ST, AC, justification),
    "ac.unsatisfy" => facade.ac_unsatisfy(ST, AC),
    "ac.descope" => facade.ac_descope(ST, AC, target(unmet), Some("hv"), Some("moved")),
    "ac.withdraw" => facade.ac_withdraw(ST, AC, justification, Some("hv")),
    "ac.rescope" => facade.ac_rescope(ST, AC),
    "ac.reinstate" => facade.ac_reinstate(ST, AC),
    "ac.fc" => facade.ac_fc(ST, AC, justification, "hv"),
    "st.triage" => facade.st_triage(ST),
    "st.start" => facade.st_start(ST),
    "st.resume" => facade.st_resume(ST),
    "st.hold" => facade.st_hold(ST, justification),
    "st.done" => facade.st_done(ST),
    "st.cancel" => facade.st_cancel(ST, justification),
    "st.reopen" => facade.st_reopen(ST, justification),
    "st.reinstate" => facade.st_reinstate(ST, justification),
    "st.fc" => facade.st_fc(ST, justification, "hv"),
    "wp.start" => facade.wp_start(ST, seq),
    "wp.unstart" => facade.wp_unstart(ST, seq),
    "wp.done" => facade.wp_done(ST, seq),
    "wp.reopen" => facade.wp_reopen(ST, seq, justification),
    "wp.cancel" => facade.wp_cancel(ST, seq, justification),
    "wp.reinstate" => facade.wp_reinstate(ST, seq, justification),
    "wp.fc" => facade.wp_fc(ST, seq, justification, "hv"),
    other => panic!("no arm drives {other}"),
  }
}

/// **THE conformance test, and the one the old shape could not have written.**
///
/// Every (verb, state) pair the ratified machine does NOT declare must be
/// REFUSED. Without this, a graph could declare the right edges and still
/// accept every other transition on top of them -- which is exactly what
/// `from: &[]` was: the declared edges were all present and correct, and so was
/// every edge nobody declared.
#[test]
fn a_transition_the_ratified_machine_does_not_declare_is_refused() {
  let mut checked = 0;
  let mut per_machine: Vec<(&str, usize)> = Vec::new();
  let mut unrestricted_seen: Vec<(&str, &str)> = Vec::new();
  for (entity, _, ratified) in RATIFIED {
    let states: Vec<&str> = ratified
      .iter()
      .flat_map(|(_, from, to, _)| from.iter().copied().chain(std::iter::once(*to)))
      .collect::<std::collections::BTreeSet<_>>()
      .into_iter()
      .collect();
    let verbs: Vec<&str> = ratified
      .iter()
      .map(|(verb, _, _, _)| *verb)
      .collect::<std::collections::BTreeSet<_>>()
      .into_iter()
      .collect();
    let mut here = 0;
    // **Grouped by VERB, not by edge, and that is a correctness fix rather than a
    // tidy-up.** A verb can declare several edges, and the filter below has to
    // exclude the whole union of its targets -- so iterating edges asked
    // `ac.rescope` from `computed` to refuse, on the strength of the edge whose
    // target is `unsatisfied`, while `computed` is that same verb's OTHER target.
    // The self-loop predicate is "the current state equals the verb's TARGET"
    // (data-model.md, hv 2026-08-17), and for a kind-dependent verb the target is
    // a SET. Testing one member at a time gets a well-formed wrong answer.
    //
    // It was invisible until 2026-08-17 because `ac_rescope` and `ac_reinstate`
    // hand-checked the from-state ahead of the shared setter and refused
    // everything else -- so this walk passed on a refusal that was itself the
    // defect of issue 0053. Removing that check unmasked the filter.
    for verb in &verbs {
      let from: std::collections::BTreeSet<&str> = ratified
        .iter()
        .filter(|(v, _, _, _)| v == verb)
        .flat_map(|(_, from, _, _)| from.iter().copied())
        .collect();
      // **Excluding the verb's own TARGETS, because a self-loop is not an
      // undeclared transition -- it is not a transition** (data-model.md, hv
      // 2026-08-17: "asking a verb for the state an entity is already in is not a
      // movement, so it is not a transition to declare and not an illegal one to
      // refuse"). `st.triage` targets `not-started`, so asking it of a thread
      // already `not-started` is accepted at exit 0, and demanding a refusal
      // there would be this test requiring the behaviour the ruling retired.
      let targets: std::collections::BTreeSet<&str> = ratified
        .iter()
        .filter(|(v, _, _, _)| v == verb)
        .map(|(_, _, to, _)| *to)
        .collect();
      // **AN EMPTY FROM-SET IS A DECLARATION, NOT AN OMISSION, AND THIS WALK MUST
      // NOT GUESS WHICH.** See `UNRESTRICTED_VERBS`. Reading it as "declared from
      // nothing" is what asked `at.set` to refuse from `fiat` -- the one state
      // whose exit `at.set` IS.
      let unrestricted = UNRESTRICTED_VERBS
        .iter()
        .find(|(e, v, _)| e == entity && v == verb);
      if from.is_empty() {
        assert!(
          unrestricted.is_some(),
          "{entity}: `{verb}` declares an EMPTY from-set. If that means ANY state, say so in \
           UNRESTRICTED_VERBS with the reason and this walk will contribute nothing for it. If it \
           means the from-set was never filled in, fill it in -- that is the `from: &[]` hole this \
           test was written to catch, and the two are indistinguishable from here"
        );
        unrestricted_seen.push((*entity, *verb));
        continue;
      }
      assert!(
        unrestricted.is_none(),
        "{entity}: `{verb}` is declared unrestricted in UNRESTRICTED_VERBS and its ratified \
         from-set is {from:?}, not empty. The declaration is stale and is now excusing a walk it \
         no longer describes"
      );

      for state in states
        .iter()
        .filter(|s| !from.contains(*s) && !targets.contains(*s))
      {
        let outcome = attempt(
          entity,
          verb,
          state,
          "a reason, so the refusal is about the STATE",
          // No guard broken, for the same reason the justification is supplied:
          // this walk's subject is the from-state, and a fixture with a second
          // thing wrong cannot say which one the refusal was for.
          None,
        );
        assert!(
          outcome.is_err(),
          "{entity}: `{verb}` from `{state}` is not in the ratified machine and was NOT refused -- got {outcome:?}"
        );
        // **The AC verbs are held to refusal, not to a particular variant, and
        // that is a deliberate asymmetry rather than a weaker check.** The
        // thread and work-package verbs enforce from the declared graph and so
        // raise one error; the AC verbs predate the table and raise richer,
        // per-cause ones -- `NotSatisfied` says there is nothing to unsatisfy,
        // `WrongOffScopeState` names the verb that DOES undo the state it is
        // actually in. AC-04.4 wants a remedy per cause, so collapsing those
        // into `IllegalTransition` would trade a better message for a tidier
        // test. What must hold either way is that the transition does not
        // happen, and that is what is asserted.
        if *entity != "Criterion" {
          assert!(
            matches!(outcome, Err(FacadeError::IllegalTransition { .. })),
            "{entity}: `{verb}` from `{state}` must be refused AS an illegal transition -- got {outcome:?}"
          );
        }
        checked += 1;
        here += 1;
      }
    }
    per_machine.push((entity, here));
  }

  // **Every machine's contribution is recorded, and a ZERO is declared** -- so a
  // machine this walk never visited cannot look like a machine it visited and
  // found nothing to refuse in. Both produce no assertions; only one is correct.
  for (entity, count) in &per_machine {
    let total = TOTAL_MACHINES.iter().find(|(name, _)| name == entity);
    match total {
      Some((_, why)) => assert_eq!(
        *count, 0,
        "{entity} is declared total ({why}), so this walk must find nothing undeclared in it -- \
         finding {count} means the machine grew and the declaration is stale"
      ),
      None => assert!(
        *count > 0,
        "{entity} contributed NO undeclared pairs and is not declared total. Either the machine is \
         total -- say so in TOTAL_MACHINES with the reason -- or this walk is not reaching it, \
         which is the omission it cannot otherwise see. Counts: {per_machine:?}"
      ),
    }
  }
  // And the other direction: a machine declared total must have been VISITED.
  // Without this, deleting its row from `RATIFIED` removes it from `per_machine`
  // and the loop above has nothing to check -- the declaration would then be
  // covering an absence instead of a zero.
  for (entity, why) in TOTAL_MACHINES {
    assert!(
      per_machine.iter().any(|(name, _)| name == entity),
      "{entity} is declared total ({why}) and this walk never visited it, so the zero above is an \
       absence rather than a measurement. Counts: {per_machine:?}"
    );
  }
  // The other direction, exactly as for `TOTAL_MACHINES`: a row here that no
  // machine exercised is covering an absence rather than a declared zero -- it
  // would go on excusing a verb that had been renamed or deleted.
  for (entity, verb, why) in UNRESTRICTED_VERBS {
    assert!(
      unrestricted_seen
        .iter()
        .any(|(e, v)| e == entity && v == verb),
      "{entity}.`{verb}` is declared unrestricted ({why}) and this walk never met it, so the \
       exemption is covering an absence rather than a measurement. Seen: {unrestricted_seen:?}"
    );
  }
  assert!(
    checked >= 20,
    "only {checked} undeclared pairs were exercised -- the enumeration is collapsing, and a check that examines nothing passes. Counts: {per_machine:?}"
  );
}

/// **EVERY DECLARED GUARD IS DRIVEN WITH THE GUARD UNMET AND MUST REFUSE, AND
/// THE REFUSAL MUST BE THE ONE THAT GUARD OWES.**
///
/// The other half of the walk. `every_declared_edge_is_a_mutation_that_exists`
/// drives each edge with every precondition SATISFIED, which is the case a guard
/// does not exist for: an edge that lands on the right value from the right state
/// says nothing at all about what happens when the precondition fails. **Two
/// guards hid in exactly that blind spot** -- `Guard::GatePass` was hand-written
/// at two call sites, so deleting it from the table changed nothing, and
/// `ac.withdraw`'s `ReasonRecorded` was transcribed, conformance-checked and
/// dead, because `set_ac_state` read the table for the from-state and never for
/// the guard column.
///
/// **It replaces `a_verb_declared_to_record_a_justification_refuses_a_blank_one`,
/// which was this test over two of the five guards.** That one covered the two
/// prose guards and skipped `GatePass`, `NonTestOnly` and `TargetExists` in
/// silence -- so the guards enforced by hand were exactly the guards nothing
/// drove unmet. Its two probes for a blank justification are carried into
/// [`probes`]; nothing it asserted is gone.
///
/// **THE POPULATION IS THE DECLARATION TABLE, NOT [`RATIFIED`], AND THE FIRST
/// VERSION OF THIS TEST GOT THAT WRONG.** vc's self-loop finding names the two
/// edges at risk -- `at.set` and `wp.rescope`, the two carrying `from: &[]` -- and
/// both live in machines ratified IN PROSE, which are absent from `RATIFIED`. So
/// the refusal written to catch that case could not see the edges it was written
/// for. **Third time this file has gone short on the entity axis**, and this time
/// inside the instrument built to close the second. Reading
/// `transitions::FIELDS` costs nothing:
/// `the_transition_table_transcribes_the_ratified_machines_edge_for_edge` already
/// ties the tabled machines to the ratified ones, so the declaration table is the
/// ratified graph plus the two machines `RATIFIED` deliberately omits.
///
/// Four completeness properties, and they fail in different directions:
///
/// 1. **A declared pair with no [`UNMET`] row PANICS** rather than skipping. A
///    pair nobody can construct a fixture for is a guard nobody can test, and
///    skipping it reports as a pass.
/// 2. **An [`UNMET`] row naming no declared pair fails too**, so a fixture left
///    behind by a retired guard is a red rather than dead weight.
/// 3. **The row COUNT per pair is checked against [`MODES`]**, because a guard
///    with two unmet modes and one row looks identical, from the declaration
///    side, to a guard with one mode.
/// 4. **The machines examined are counted twice**, once by the exhaustive match
///    below and once as the complement of [`edgeless`] -- and the count is
///    required to EXCEED `RATIFIED`, so populating this from the ratified tables
///    again is a red rather than a silently smaller walk.
///
/// # What this actually added, measured rather than claimed
///
/// **Four of the five guards it drives were ALREADY witnessed somewhere, and
/// saying so is the point of measuring.** Each guard was neutered in `facade.rs`
/// and the whole workspace run with THIS TEST SKIPPED, in a sacrificial worktree,
/// against a green baseline of 73 legs:
///
/// | guard, unmet | caught elsewhere by |
/// | --- | --- |
/// | `GatePass` | 7 tests, `closing_is_gated` and `the_facade_routes_closes_through_the_gate` among them |
/// | `NonTestOnly` on `ac.satisfy` | 2, including `satisfying_a_test_backed_criterion_directly_is_refused` |
/// | **`NonTestOnly` on `ac.unsatisfy`** | **NOTHING -- 73 legs, 0 failed** |
/// | `TargetExists`, target missing | 4, including a direct one |
/// | `TargetExists`, target blank | 1, and only the generic variant check |
///
/// **So this is a second witness four times over and a hole closed once** -- which
/// is a materially weaker claim than the one that was available before the
/// measurement, and it is the true one.
///
/// **THE STRUCTURE IS WORTH MORE THAN THE TALLY.**
/// `error_remedies::every_variant_is_provoked_or_declared_elsewhere` is the
/// estate's REFUSAL-side enumerator -- every `FacadeError` variant must be
/// provoked by something -- and it caught four of the five, because deleting a
/// guard orphans its variant. **It cannot catch the fifth, and the reason is
/// exact: `ComputedSatisfaction` stays provoked at `ac.satisfy`, so removing the
/// identical check at `ac.unsatisfy` orphans nothing.** A variant-completeness
/// walk is blind to the SECOND site of a guard whose variant is provoked at the
/// first.
///
/// **That is the pair, and the estate already had one half of it.** This walk
/// enumerates from the DECLARATION side and goes short when a guard is missing
/// from [`UNMET`]; that one enumerates from the REFUSAL side and goes short when a
/// variant is provoked anywhere at all. Neither is redundant with the other and
/// **neither should be simplified away as covering the same ground**, because the
/// ground they cover differs by exactly the case above.
#[test]
fn every_declared_guard_refuses_when_it_is_unmet() {
  // (entity, verb, guard, a legal from-state).
  let mut pairs: Vec<(&str, &str, Guard, &str)> = Vec::new();
  let mut machines = 0;
  for row in FIELDS {
    let edges = match &row.disposition {
      Disposition::State { edges, .. } => {
        machines += 1;
        *edges
      }
      // Named rather than caught by a `_`, so a third edgeless variant asks here
      // instead of being skipped -- which is how `Thread.acceptance` left four
      // populations at 20 passed, 0 failed.
      Disposition::Unbuilt { .. } | Disposition::Immutable { .. } => continue,
    };
    let entity = row.entity;
    for edge in edges {
      let (verb, from, to, guards) = (edge.verb, edge.from, edge.to, edge.guard);
      if guards.is_empty() {
        continue;
      }
      // **A GUARDED EDGE THAT ACCEPTS ITS OWN TARGET IS REFUSED BY THIS TABLE
      // RATHER THAN DRIVEN** (vc, 2026-08-17, measured: 11 guarded edges, 12
      // pairs, `to` in `from` on zero of them -- so this is green today and is
      // here for the day it stops being).
      //
      // The expectation would FLIP. A self-loop is legal, accepted at exit 0, and
      // deliberately does not re-run the guard -- so on such an edge the correct
      // answer for a self-loop fixture is `Ok(AlreadyThere)`, and this test
      // demanding a refusal would be demanding the behaviour hv's ruling retired.
      // **And the plausible repair is the destructive one**: red on correct code
      // invites hoisting the guard ahead of the self-loop test, at which point
      // `st done` starts re-running the gate on a thread that closed before the
      // criterion existed. Refusing here puts the argument where somebody would
      // otherwise make that edit.
      //
      // An empty `from` is the live risk rather than a hypothetical: `at.set` and
      // `wp.rescope` declare `from: &[]`, meaning any value reaches any other, so
      // the target is always accepted and a guard on either one lands in exactly
      // this case.
      assert!(
        !from.is_empty() && !from.contains(&to),
        "{entity}: `{verb}` -> `{to}` is guarded {guards:?} and accepts its own target (from {from:?}), so a fixture in `{to}` is a legal SELF-LOOP that must \
         return Ok at exit 0 without re-running the guard. This driver cannot express that, and moving the guard ahead of the self-loop test to make it red \
         would make `st done` re-run the gate on an already-closed thread. Split the fixture population by `Edge::leaves` before relaxing anything"
      );
      for guard in guards.iter() {
        if pairs.iter().any(|(_, v, g, _)| *v == verb && g == guard) {
          continue;
        }
        pairs.push((entity, verb, *guard, from[0]));
      }
    }
  }
  assert!(
    !pairs.is_empty(),
    "no declared edge carries a guard, so this test asserts a property of an empty set -- either the guard column has been emptied or FIELDS is not being read"
  );
  // The machines are counted a second time as the complement of `edgeless`, which
  // is the by-NAME enumerator: it omits a new disposition until an arm is added,
  // while the match above includes one immediately -- as a compile error, so the
  // compiler asks before this ever runs.
  assert_eq!(
    machines,
    FIELDS.len() - FIELDS.iter().filter(|f| edgeless(f).is_some()).count(),
    "the walk examined {machines} State machine(s) and the edgeless complement says otherwise, so a row reached neither arm"
  );
  // And the population is the DECLARATION table rather than the ratified ones.
  // Reverting to `RATIFIED` makes the self-loop refusal above blind to `at.set`
  // and `wp.rescope`, which are the only two edges it exists for.
  assert!(
    machines > RATIFIED.len(),
    "the walk examined {machines} machines and RATIFIED holds {} -- this must read `transitions::FIELDS`, or the two machines ratified in prose are invisible \
     here and the self-loop refusal cannot see the two `from: &[]` edges it was written for",
    RATIFIED.len()
  );

  let mut driven = 0;
  for (entity, verb, guard, from) in &pairs {
    let rows: Vec<&(&str, Guard, Unmet, &str)> = UNMET
      .iter()
      .filter(|(v, g, _, _)| v == verb && g == guard)
      .collect();
    assert!(
      !rows.is_empty(),
      "`{verb}` declares {guard:?} and UNMET says nothing about how to make it unmet, so this walk would SKIP the only case the guard exists for. A (verb, guard) \
       pair with no fixture is a guard nobody can test: add the row, or take the guard off the edge"
    );
    let modes = MODES
      .iter()
      .find(|(v, g, _, _)| v == verb && g == guard)
      .map_or(1, |(_, _, count, _)| *count);
    assert_eq!(
      rows.len(),
      modes,
      "`{verb}` / {guard:?} carries {} fixture row(s) and {modes} mode(s) are declared. A guard unmet more ways than it is driven is covered on paper only; \
       declare the count in MODES with the reason, or write the missing row",
      rows.len()
    );

    for (_, _, unmet, why) in rows {
      assert!(
        !why.is_empty(),
        "`{verb}` / {guard:?} / {unmet:?} says nothing about what makes its refusal THIS guard's, which is the one thing a reader cannot re-derive from the loop"
      );
      for justification in probes(*unmet) {
        let outcome = attempt(entity, verb, from, justification, Some(*unmet));
        let err = match &outcome {
          Err(err) => err,
          Ok(outcome) => panic!(
            "{entity}: `{verb}` is declared {guard:?} and ACCEPTED the transition with that guard unmet ({unmet:?}) -- got {outcome:?}. The declaration says the \
             precondition is enforced and nothing enforces it, which is a decorative guard: {why}"
          ),
        };
        assert!(
          names_the_guard(*guard, *unmet, err),
          "{entity}: `{verb}` refused with {err:?}, which is not the refusal {guard:?} owes. A refusal from the WRONG cause is the failure this assertion \
           exists for -- it means the guard may not be enforced at all and something else got there first: {why}"
        );
        driven += 1;
      }
    }
  }

  // The opposite direction: a fixture row for a pair no ratified edge declares.
  for (verb, guard, unmet, _) in UNMET {
    assert!(
      pairs.iter().any(|(_, v, g, _)| v == verb && g == guard),
      "UNMET carries a fixture for `{verb}` / {guard:?} / {unmet:?} and no declared edge carries that pair. Either the guard came off the edge and this row \
       outlived it, or the verb is misspelled and the row has been driving nothing"
    );
  }
  for (verb, guard, count, why) in MODES {
    assert!(
      pairs.iter().any(|(_, v, g, _)| v == verb && g == guard),
      "MODES declares {count} unmet mode(s) for `{verb}` / {guard:?} and no declared edge carries that pair, so the count covers an absence rather than a \
       population: {why}"
    );
  }
  assert!(
    driven > pairs.len(),
    "{driven} drives across {} pairs -- at least one pair must contribute more than one probe or the blank/whitespace pair of the prose guards has been lost",
    pairs.len()
  );
}

/// And the mirror: an UNGUARDED transition CLEARS a reason left by a guarded
/// one, so a thread never explains itself with a condition that has ended.
#[test]
fn an_unguarded_transition_clears_the_reason_a_guarded_one_left() {
  let fx = Fixture::new();
  fx.write_thread(&thread_with(|t| t.status = ThreadStatus::Wip));
  let mut facade = fx.facade();

  facade.st_hold(ST, "waiting on the fleet").expect("hold");
  assert_eq!(
    facade.st_show(ST).expect("thread").status_reason.as_deref(),
    Some("waiting on the fleet"),
    "precondition: the guarded verb recorded it"
  );

  facade.st_resume(ST).expect("resume");
  assert_eq!(
    facade.st_show(ST).expect("thread").status_reason,
    None,
    "a resumed thread must not still be explaining why it was paused -- the reason belongs to the state it was given for"
  );
}
