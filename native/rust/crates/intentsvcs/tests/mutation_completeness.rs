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

mod common;

use common::{Fixture, sample_issue, sample_thread};
use intentsvcs::facade::{Facade, FacadeError};
use intentsvcs::model::{
  AcKind, AcState, AcceptanceMode, AtKind, AtStatus, Criterion, IssueStatus, Thread, ThreadStatus,
  WpStatus, enum_str,
};
use intentsvcs::transitions::{
  ABSENT, Disposition, Edge, Entry, FIELDS, Guard, exitless, find, traps, unreachable,
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
      "satisfied".to_string(),
      "unsatisfied".to_string(),
      "withdrawn".to_string()
    ],
    "all five recorded states reach the walk, including BOTH entry values -- `computed` for a test-backed criterion and `unsatisfied` for an authored one"
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
      "{entity}.{field} has a closed value domain and the transition table does not classify it -- add it to transitions.rs as a State with its edges, or as Unbuilt naming the work package that owes it (D32)"
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

/// **An `Unbuilt` field must be one NO service call can put a value into.**
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
/// `Unbuilt` rows have no edges at all, so it fires on exactly those. It
/// discriminates cleanly, and `the_wider_reading_fires_on_unbuilt_rows_only`
/// holds that as a measurement rather than leaving it as the counter-argument
/// to this one. The pair below is the second condition; this is the first.
///
/// It measures by DRIVING the creation verbs rather than by reading them. That
/// is how `WorkPackage.scope` was caught: `wp_new` takes the size from the
/// caller, so all six were entered at creation and none could be left.
#[test]
fn an_unbuilt_field_is_one_no_service_call_can_set() {
  for field in FIELDS {
    let Disposition::Unbuilt { .. } = &field.disposition else {
      continue;
    };
    let settable = match (field.entity, field.field) {
      // `st_new` hardcodes `acceptance: None`; the caller has no say.
      ("Thread", "acceptance") => {
        let fx = Fixture::new();
        let mut facade = fx.facade();
        let id = facade.st_new("a thread").expect("st new");
        facade.st_show(&id).expect("thread").acceptance.is_some()
      }
      // No service call constructs a criterion, an acceptance test or an
      // issue: they arrive only as authored canon. Asserted by the absence of
      // a creation verb on the facade, which is a compile-time fact -- adding
      // one and not adding an arm here leaves this returning false, so the
      // arm below is what has to be kept honest, and the panic is the keeper.
      ("Criterion", "kind") | ("AcceptanceTest", "kind") | ("Issue", "status") => false,
      other => panic!(
        "{other:?} is Unbuilt and no arm decides whether a service call can set it -- inertness is measured, never assumed"
      ),
    };
    assert!(
      !settable,
      "{}.{} is declared Unbuilt and a service call CAN put a value into it -- so an entity can hold a value nothing can change, and the row is describing a trap rather than an absence. Make it a State and give it the exit",
      field.entity, field.field
    );
  }
}

/// **AC-04.6's second condition: an `Unbuilt` field's ENTRY path is measured by
/// driving canon, and the table has to agree with the measurement.**
///
/// The measurement is one shape for every row: author canon carrying a value no
/// service call could have put there, read it back through the facade, and see
/// whether it is held as authored. If ingest normalised it away the field is
/// genuinely `Inert`; if it survives, an entity is holding a value with no verb
/// to move it.
///
/// **What this test does NOT do is discharge the criterion, and the distinction
/// matters because getting it wrong is how a red row goes quietly green.** All
/// four rows measure `Authored`, so all four are mutations owed; asserting the
/// declaration matches the measurement makes the debt visible and guards
/// against a fifth arriving unnoticed. It does not make an unleaveable value
/// leaveable. The criterion stays red until the verbs exist, and each of the
/// four needs a ratified machine first -- which is the same block `issues
/// add|close|open` already stands behind, one row wider.
#[test]
fn an_unbuilt_fields_entry_path_is_measured_not_declared() {
  for field in FIELDS {
    let Disposition::Unbuilt { entry, .. } = &field.disposition else {
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
      // v2 had `issues close`; v3 ships the read verbs only. So a closed issue
      // in the estate is a value authored canon put there and nothing can undo.
      ("Issue", "status") => {
        let fx = Fixture::new();
        fx.write_issue(&sample_issue(7));
        // `closed` is only meaningful on a closed issue, so reopening the
        // fixture means clearing the date with it -- the same two-fields-in-one-
        // act shape the criterion arm found, one entity over.
        let mut open = sample_issue(8);
        open.status = IssueStatus::Open;
        open.closed = None;
        fx.write_issue(&open);
        let read = fx.facade();
        read.issue_show(7).expect("issue").status == IssueStatus::Closed
          && read.issue_show(8).expect("issue").status == IssueStatus::Open
      }
      other => panic!(
        "{other:?} is Unbuilt and no arm MEASURES how a value gets into it. Entry is driven, never assumed -- a row asserted inert on inspection is the exact \
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
/// Both halves are asserted non-empty. A walk that found no fields of either
/// kind would agree with every claim above.
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
fn the_wider_reading_fires_on_unbuilt_rows_only() {
  let mut state_fields = 0;
  let mut unbuilt_fields = 0;

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
      Disposition::Unbuilt { .. } => {
        unbuilt_fields += 1;
        assert_eq!(
          exitless(&values, &[]).len(),
          values.len(),
          "{entity}.{field} is Unbuilt, so EVERY value of it should be exitless. A subset means the row has acquired edges while still declaring the work \
           unbuilt"
        );
      }
    }
  }

  assert!(
    state_fields > 0 && unbuilt_fields > 0,
    "the walk found {state_fields} State and {unbuilt_fields} Unbuilt fields, and it needs both to be showing a difference between them rather than a property \
     of an empty set"
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
fn unbuilt_fields_say_what_is_unavailable_and_carry_no_edges() {
  for field in FIELDS {
    if let Disposition::Unbuilt { note, .. } = &field.disposition {
      assert!(
        !note.is_empty(),
        "{}.{} is Unbuilt and must say what is unavailable",
        field.entity,
        field.field
      );
      assert!(
        pm_free(note),
        "{}.{} is Unbuilt and its note names Intent's own project-management state, which a \
         consumer of this crate cannot look up: {note}",
        field.entity,
        field.field
      );
    }
  }
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
      for from in values.iter().filter(|v| edge.accepts(v)) {
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
/// report where the field landed.
fn execute(entity: &str, field: &str, edge: &Edge, from: &str) -> String {
  match (entity, field) {
    ("Thread", "status") => {
      let fx = Fixture::new();
      fx.write_thread(&thread_with(|t| t.status = parse(from)));
      let mut facade = fx.facade();
      match edge.verb {
        "st.triage" => facade.st_triage(ST).expect("st triage"),
        "st.start" => facade.st_start(ST).expect("st start"),
        "st.resume" => facade.st_resume(ST).expect("st resume"),
        "st.hold" => facade.st_hold(ST, REASON).expect("st hold"),
        "st.done" => facade.st_done(ST).expect("st done"),
        "st.cancel" => facade.st_cancel(ST, REASON).expect("st cancel"),
        "st.reopen" => facade.st_reopen(ST, REASON).expect("st reopen"),
        "st.reinstate" => facade.st_reinstate(ST, REASON).expect("st reinstate"),
        other => panic!("no arm drives {other} on Thread.status"),
      }
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
      match edge.verb {
        "wp.rescope" => facade
          .wp_rescope(ST, seq, parse(edge.to))
          .expect("wp rescope"),
        other => panic!("no arm drives {other} on WorkPackage.scope"),
      }
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
      match edge.verb {
        "wp.start" => facade.wp_start(ST, seq).expect("wp start"),
        "wp.unstart" => facade.wp_unstart(ST, seq).expect("wp unstart"),
        "wp.done" => facade.wp_done(ST, seq).expect("wp done"),
        "wp.reopen" => facade.wp_reopen(ST, seq, REASON).expect("wp reopen"),
        other => panic!("no arm drives {other} on WorkPackage.status"),
      }
      enum_str(&facade.wp_show(ST, seq).expect("wp").status).to_string()
    }
    ("AcceptanceTest", "status") => {
      let fx = Fixture::new();
      fx.write_thread(&thread_with(|t| t.tests[0].status = parse(from)));
      let mut facade = fx.facade();
      let to: AtStatus = parse(edge.to);
      match edge.verb {
        "at.set" => facade.at_set(ST, "AT-03.1", to).expect("at set"),
        other => panic!("no arm drives {other} on AcceptanceTest.status"),
      }
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
    other => panic!("no arm drives {other:?} -- every State field needs one"),
  }
}

/// The AC verbs all act on `AC-03.2`, the fixture's non-test criterion.
fn apply_ac_verb(facade: &mut intentsvcs::facade::Facade, verb: &str) {
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
    "unsatisfied" => AcState::Unsatisfied,
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
    other => panic!("no such AC state: {other}"),
  }
}

fn state_name_of(state: &AcState) -> &'static str {
  match state {
    AcState::Computed => "computed",
    AcState::Unsatisfied => "unsatisfied",
    AcState::Satisfied { .. } => "satisfied",
    AcState::Descoped { .. } => "descoped",
    AcState::Withdrawn { .. } => "withdrawn",
  }
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
const RATIFIED_THREAD: &[(&str, &[&str], &str, &[Guard])] = &[
  ("st.triage", &["triage"], "not-started", &[]),
  ("st.start", &["not-started"], "wip", &[]),
  ("st.resume", &["hold"], "wip", &[]),
  (
    "st.hold",
    &["not-started", "wip"],
    "hold",
    &[Guard::ReasonRecorded],
  ),
  ("st.done", &["wip"], "completed", &[Guard::GatePass]),
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
const RATIFIED_CRITERION: &[(&str, &[&str], &str, &[Guard])] = &[
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
];

/// The ratified work-package machine, same discipline.
const RATIFIED_WP: &[(&str, &[&str], &str, &[Guard])] = &[
  ("wp.start", &["not-started"], "wip", &[]),
  ("wp.unstart", &["wip"], "not-started", &[]),
  ("wp.done", &["wip"], "done", &[Guard::GatePass]),
  ("wp.reopen", &["done"], "wip", &[Guard::ReasonRecorded]),
];

fn declared(entity: &str, field: &str) -> &'static [Edge] {
  match find(entity, field).map(|f| &f.disposition) {
    Some(Disposition::State { edges, .. }) => edges,
    _ => panic!("{entity}.{field} is not a State"),
  }
}

#[test]
fn the_implemented_graph_is_the_ratified_one_edge_for_edge() {
  for (entity, field, ratified) in [
    ("Thread", "status", RATIFIED_THREAD),
    ("WorkPackage", "status", RATIFIED_WP),
    ("Criterion", "state", RATIFIED_CRITERION),
  ] {
    let implemented = declared(entity, field);
    assert_eq!(
      implemented.len(),
      ratified.len(),
      "{entity}.{field} declares {} edges and the ratified machine has {} -- an EXTRA edge is as much a divergence as a missing one, because it is a transition nobody approved",
      implemented.len(),
      ratified.len()
    );
    for (verb, from, to, guard) in ratified {
      let found = implemented
        .iter()
        .find(|e| e.verb == *verb && e.to == *to)
        .unwrap_or_else(|| {
          panic!("{entity}.{field}: the ratified machine has `{verb}` -> `{to}` and the code declares no such edge")
        });
      assert_eq!(
        found.from, *from,
        "{entity}.{field}: `{verb}` is ratified from {from:?} and declared from {:?}",
        found.from
      );
      assert_eq!(
        found.guard, *guard,
        "{entity}.{field}: `{verb}` is ratified with {guard:?} and declared with {:?} -- a guard is the half a success-path test never sees",
        found.guard
      );
    }
  }
}

/// Drive `verb` from `from`, and report the refusal if there was one.
fn attempt(entity: &str, verb: &str, from: &str, justification: &str) -> Result<(), FacadeError> {
  let fx = Fixture::new();
  let mut facade: Facade = match entity {
    "Thread" => {
      fx.write_thread(&thread_with(|t| t.status = parse(from)));
      fx.facade()
    }
    "WorkPackage" => {
      fx.write_thread(&thread_with(|t| t.wps[1].status = parse::<WpStatus>(from)));
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
      }));
      fx.write_thread(&sample_thread("ST0057"));
      fx.facade()
    }
    other => panic!("no fixture for {other}"),
  };
  let seq = 3;
  const AC: &str = "AC-03.2";
  match verb {
    "ac.satisfy" => facade.ac_satisfy(ST, AC, justification),
    "ac.unsatisfy" => facade.ac_unsatisfy(ST, AC),
    "ac.descope" => facade.ac_descope(ST, AC, "ST0057", Some("hv"), Some("moved")),
    "ac.withdraw" => facade.ac_withdraw(ST, AC, justification, Some("hv")),
    "ac.rescope" => facade.ac_rescope(ST, AC),
    "ac.reinstate" => facade.ac_reinstate(ST, AC),
    "st.triage" => facade.st_triage(ST),
    "st.start" => facade.st_start(ST),
    "st.resume" => facade.st_resume(ST),
    "st.hold" => facade.st_hold(ST, justification),
    "st.done" => facade.st_done(ST),
    "st.cancel" => facade.st_cancel(ST, justification),
    "st.reopen" => facade.st_reopen(ST, justification),
    "st.reinstate" => facade.st_reinstate(ST, justification),
    "wp.start" => facade.wp_start(ST, seq),
    "wp.unstart" => facade.wp_unstart(ST, seq),
    "wp.done" => facade.wp_done(ST, seq),
    "wp.reopen" => facade.wp_reopen(ST, seq, justification),
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
  for (entity, ratified) in [
    ("Thread", RATIFIED_THREAD),
    ("WorkPackage", RATIFIED_WP),
    ("Criterion", RATIFIED_CRITERION),
  ] {
    let states: Vec<&str> = ratified
      .iter()
      .flat_map(|(_, from, to, _)| from.iter().copied().chain(std::iter::once(*to)))
      .collect::<std::collections::BTreeSet<_>>()
      .into_iter()
      .collect();
    for (verb, from, _, _) in ratified {
      for state in states.iter().filter(|s| !from.contains(s)) {
        let outcome = attempt(
          entity,
          verb,
          state,
          "a reason, so the refusal is about the STATE",
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
        if entity != "Criterion" {
          assert!(
            matches!(outcome, Err(FacadeError::IllegalTransition { .. })),
            "{entity}: `{verb}` from `{state}` must be refused AS an illegal transition -- got {outcome:?}"
          );
        }
        checked += 1;
      }
    }
  }
  assert!(
    checked >= 20,
    "only {checked} undeclared pairs were exercised -- the enumeration is collapsing, and a check that examines nothing passes"
  );
}

/// A verb declared to record a justification REFUSES a blank one -- for every
/// entity, and for both kinds of justification.
///
/// The guards are declared in the table, so this reads the declaration rather
/// than a list of verbs somebody has to keep in step with it.
///
/// **It used to loop `Thread` and `WorkPackage` and stop, and that omission is
/// how two defects lived here at once.** `Criterion` was the one entity whose
/// declared guards nothing enforced -- `set_ac_state` consulted the table for
/// the from-state and never for the guard column -- and it was also the one
/// entity this check did not visit. So `ac.withdraw` carried a ratified
/// `ReasonRecorded` that was transcribed, conformance-checked, and dead, while
/// `ac.satisfy` could not even declare the evidence rule it needed. **An
/// instrument that enumerates its subjects by hand can be wrong in exactly the
/// place its subject is wrong, and it reports green.** The list is now derived
/// from the ratified tables themselves.
#[test]
fn a_verb_declared_to_record_a_justification_refuses_a_blank_one() {
  let mut checked = 0;
  for (entity, ratified) in [
    ("Thread", RATIFIED_THREAD),
    ("WorkPackage", RATIFIED_WP),
    ("Criterion", RATIFIED_CRITERION),
  ] {
    for (verb, from, _, guards) in ratified {
      // Both prose guards, together, because they are one rule asked of two
      // fields and a check that knew only about reasons is what let the
      // evidence half go unwritten.
      let reason = guards.contains(&Guard::ReasonRecorded);
      let evidence = guards.contains(&Guard::EvidenceRecorded);
      if !reason && !evidence {
        continue;
      }
      for blank in ["", "   "] {
        let outcome = attempt(entity, verb, from[0], blank);
        let refused = match &outcome {
          Err(FacadeError::ReasonRequired { .. }) => reason,
          Err(FacadeError::EvidenceRequired { .. }) => evidence,
          _ => false,
        };
        assert!(
          refused,
          "{entity}: `{verb}` is declared {guards:?} and accepted the justification {blank:?} -- got {outcome:?}"
        );
        checked += 1;
      }
    }
  }
  assert!(
    checked >= 12,
    "only {checked} guarded verbs were exercised -- the enumeration is collapsing, and a check that examines nothing passes"
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
