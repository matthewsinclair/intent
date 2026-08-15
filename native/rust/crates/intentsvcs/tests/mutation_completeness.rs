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

use common::{Fixture, sample_thread};
use intentsvcs::model::{AcScope, AtStatus, Criterion, Thread, ThreadStatus, WpStatus, enum_str};
use intentsvcs::transitions::{ABSENT, Disposition, Edge, FIELDS, find, traps, unreachable};
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
/// arms are bare strings (`AtStatus`) or tagged objects (`AcScope`).
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
      for key in ["state", "status"] {
        if let Some(tag) = props.get(key) {
          if let Some(one) = tag.get("const").and_then(Value::as_str) {
            values.push(one.to_string());
          } else if let Some(list) = string_list(tag.get("enum")) {
            values.extend(list);
          }
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

/// The ruled instance, named. `Criterion.satisfied` is the field hv ruled on
/// and the one shape with no `$defs` entry behind it, so it is asserted by
/// name as well as by count.
#[test]
fn the_ruled_field_is_in_the_walk() {
  let fields = all_fields();
  let (_, _, values) = fields
    .iter()
    .find(|(e, f, _)| e == "Criterion" && f == "satisfied")
    .expect("Criterion.satisfied is the field AC-04.6 was ruled on and must be walked");
  assert_eq!(values, &[ABSENT, "false", "true"]);
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
/// The reading held here is the SERVICE-PATH one, stated because the
/// alternative was considered and discriminates nothing: strict ingest accepts
/// any schema-valid canon, so under "reachable by any path including ingest"
/// every value of every closed-domain field is enterable, every `Unbuilt` row
/// fails, and the test fires on everything. A test that fires on everything is
/// not a test. D32 is a statement about what SERVICES expose, so that is what
/// this measures -- and the residue (a value that can only arrive by authoring
/// canon by hand) is real, named in each row's note, and owed to WP-06.
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

/// An `Unbuilt` field owes a work package and declares no edges -- so the day
/// a mutation for it lands, the disposition is contradicted rather than
/// quietly outliving the gap it described.
#[test]
fn unbuilt_fields_name_their_work_package_and_carry_no_edges() {
  for field in FIELDS {
    if let Disposition::Unbuilt { owed_by, note } = &field.disposition {
      assert!(
        owed_by.starts_with("WP-"),
        "{}.{} is Unbuilt and must name the work package that owes it",
        field.entity,
        field.field
      );
      assert!(
        !note.is_empty(),
        "{}.{} is Unbuilt and must say why",
        field.entity,
        field.field
      );
    }
  }
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

/// Drive one edge: build a thread with `field` at `from`, apply the verb, and
/// report where the field landed.
fn execute(entity: &str, field: &str, edge: &Edge, from: &str) -> String {
  match (entity, field) {
    ("Thread", "status") => {
      let fx = Fixture::new();
      fx.write_thread(&thread_with(|t| t.status = parse(from)));
      let mut facade = fx.facade();
      match edge.verb {
        "st.start" => facade.st_start(ST).expect("st start"),
        "st.done" => facade.st_done(ST).expect("st done"),
        "st.cancel" => facade.st_cancel(ST).expect("st cancel"),
        other => panic!("no arm drives {other} on Thread.status"),
      }
      enum_str(&facade.st_show(ST).expect("thread").status).to_string()
    }
    ("WorkPackage", "scope") => {
      let fx = Fixture::new();
      fx.write_thread(&thread_with(|t| t.wps[1].scope = parse(from)));
      let mut facade = fx.facade();
      let seq = 3;
      match edge.verb {
        "wp.rescope" => facade
          .wp_rescope(ST, seq, parse(edge.to))
          .expect("wp rescope"),
        other => panic!("no arm drives {other} on WorkPackage.scope"),
      }
      enum_str(&facade.wp_show(ST, seq).expect("wp").scope).to_string()
    }
    ("WorkPackage", "status") => {
      let fx = Fixture::new();
      fx.write_thread(&thread_with(|t| t.wps[1].status = parse::<WpStatus>(from)));
      let mut facade = fx.facade();
      let seq = 3;
      match edge.verb {
        "wp.start" => facade.wp_start(ST, seq).expect("wp start"),
        "wp.done" => facade.wp_done(ST, seq).expect("wp done"),
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
    ("Criterion", "scope") => {
      let fx = Fixture::new();
      fx.write_thread(&thread_with(|t| t.criteria[1].scope = scope_named(from)));
      let mut facade = fx.facade();
      apply_ac_verb(&mut facade, edge.verb);
      scope_name(&criterion(&facade).scope).to_string()
    }
    ("Criterion", "satisfied") => {
      let fx = Fixture::new();
      // A scope verb needs the criterion in the scope it undoes, so the
      // fixture is set up from the VERB as well as from the value -- the one
      // place an edge's precondition lives on a different field.
      let scope = match edge.verb {
        "ac.rescope" => "descoped",
        "ac.reinstate" => "withdrawn",
        _ => "in-scope",
      };
      fx.write_thread(&thread_with(|t| {
        t.criteria[1].satisfied = satisfied_named(from);
        t.criteria[1].evidence = Some("the render itself".to_string());
        t.criteria[1].scope = scope_named(scope);
      }));
      let mut facade = fx.facade();
      apply_ac_verb(&mut facade, edge.verb);
      let c = criterion(&facade);
      // The evidence goes with the satisfaction, on every edge that removes
      // it. A criterion that reads unsatisfied while still citing the proof
      // that was withdrawn is a worse record than the one-way door this AC
      // removed, because it looks like a record. Edges that ADD satisfaction
      // are exempt -- setting evidence is what they are for.
      if edge.to == ABSENT {
        assert_eq!(
          c.evidence, None,
          "`{}` cleared satisfaction and left the evidence behind",
          edge.verb
        );
      }
      satisfied_name(c.satisfied).to_string()
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

fn scope_named(name: &str) -> AcScope {
  match name {
    "in-scope" => AcScope::InScope,
    "descoped" => AcScope::Descoped {
      to: "ST0057".to_string(),
      by: Some("hv".to_string()),
      reason: None,
    },
    "withdrawn" => AcScope::Withdrawn {
      reason: "the premise did not reproduce".to_string(),
      by: None,
    },
    other => panic!("no such AC scope: {other}"),
  }
}

fn scope_name(scope: &AcScope) -> &'static str {
  match scope {
    AcScope::InScope => "in-scope",
    AcScope::Descoped { .. } => "descoped",
    AcScope::Withdrawn { .. } => "withdrawn",
  }
}

fn satisfied_named(name: &str) -> Option<bool> {
  match name {
    ABSENT => None,
    "true" => Some(true),
    "false" => Some(false),
    other => panic!("no such satisfied value: {other}"),
  }
}

fn satisfied_name(value: Option<bool>) -> &'static str {
  match value {
    None => ABSENT,
    Some(true) => "true",
    Some(false) => "false",
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
