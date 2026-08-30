//! `AC-17.1`'s foundation: one derivation from declaration + entity to
//! `{label, value, widget}`, shared by every renderer.
//!
//! **THESE TESTS MOVED DOWN A CRATE WITH THE CODE THEY COVER** (2026-08-30).
//! The walk was first written in `intent-cli/src/tui/views.rs`, which is one
//! crate too high: `intentd` depends on `intentsvcs` and NOT on the CLI, so the
//! daemon's JSON emitter would have had to write it again. `tui-design.md`
//! §10a says the JS renders triples, so does SwiftUI, so does the TUI --
//! **which is only true by construction if there is one function.** Two walks
//! agreeing today is exactly what `AC-17.1` refuses to accept as agreement.
//!
//! Moving code without moving its tests is how a shared home ends up less
//! covered than the private one it replaced, so this file exists at the moment
//! of the move rather than after it.

use intentsvcs::form::{Loaded, triples};
use serde_json::{Value, json};

fn loaded() -> Loaded {
  Loaded::load().expect("the shipped form declaration must load")
}

/// A thread-shaped value carrying every kind of JSON the model produces, so the
/// arms below are driven rather than reasoned about.
fn a_thread() -> Value {
  json!({
    "title": "Add a Rust-based CLI",
    "status": "wip",
    "objective": "line one\nline two\n\nline four",
    "created": "2026-08-14",
    "seq": 56,
    "wps": [1, 2, 3],
    "criteria": [],
    "fiat": { "because": "hv said so" },
    "slug": null
  })
}

#[test]
fn the_declaration_is_not_empty_and_neither_are_its_forms() {
  let l = loaded();
  assert!(
    !l.forms().is_empty(),
    "no forms, so every walk below is over nothing"
  );
  assert!(
    l.forms().iter().all(|f| !f.fields.is_empty()),
    "a form declares no fields, so a walk over it asserts nothing"
  );
}

#[test]
fn every_declared_field_yields_exactly_one_triple_in_declaration_order() {
  let l = loaded();
  let mut checked = 0usize;
  for form in l.forms() {
    let out = triples(form, &a_thread());
    assert_eq!(
      out.len(),
      form.fields.len(),
      "{} lost or gained a row",
      form.entity
    );
    for (t, f) in out.iter().zip(form.fields.iter()) {
      assert_eq!(t.name, f.name, "order diverged from declaration order");
      assert_eq!(t.label, f.label);
      assert_eq!(
        t.widget, f.widget,
        "a triple carries a widget the form did not declare"
      );
      assert_eq!(
        t.editable, f.editable,
        "a triple disagrees with the declaration on editability"
      );
    }
    checked += 1;
  }
  assert!(
    checked > 0,
    "no form was examined, so this test asserted nothing"
  );
}

/// **A MISSING PROPERTY MUST NOT SHORTEN THE FORM.** Tab order is declaration
/// order (`AC-17.5`), so a skipped row moves every row after it and the
/// operator's muscle memory lands on the wrong field. An empty value is
/// visible; a missing row is not.
#[test]
fn a_field_the_entity_does_not_carry_yields_an_empty_value_rather_than_vanishing() {
  let l = loaded();
  let form = l.form("thread").expect("the thread form must be declared");
  let full = triples(form, &a_thread());
  let empty = triples(form, &json!({}));
  assert_eq!(
    empty.len(),
    full.len(),
    "an entity carrying nothing produced a shorter form"
  );
  assert!(
    empty.iter().all(|t| t.value.is_empty()),
    "a value appeared for an entity that carries no properties at all"
  );
  // The control: the populated case must actually differ, or the assertion
  // above is comparing two empty forms and proving nothing.
  assert!(
    full.iter().any(|t| !t.value.is_empty()),
    "the populated fixture produced no values, so the comparison above is vacuous"
  );
}

/// A collection is its SIZE. Inlining ST0056's 297 attachments makes the form
/// 325 rows of which 297 are files, and breaks the alignment guarantee outright.
#[test]
fn an_array_yields_its_size_and_never_its_contents() {
  let l = loaded();
  let form = l.form("thread").expect("the thread form must be declared");
  let big: Vec<u32> = (0..297).collect();
  let out = triples(
    form,
    &json!({ "wps": [1, 2, 3], "criteria": [], "attachments": big }),
  );
  let by = |n: &str| out.iter().find(|t| t.name == n).map(|t| t.value.clone());
  assert_eq!(by("wps"), Some("3".into()));
  assert_eq!(by("criteria"), Some("0".into()));
  assert_eq!(
    by("attachments"),
    Some("297".into()),
    "a large collection must still be one number"
  );
}

/// **A VALUE CANNOT BECOME TWO ROWS.** Criterion prose reaches 59,061
/// characters with paragraph breaks in it; each face clips to its own width,
/// but only after this has made the value one line. Collapsing in the renderer
/// instead would have to be done identically in three places.
#[test]
fn no_value_can_carry_a_line_break() {
  let l = loaded();
  let form = l.form("thread").expect("the thread form must be declared");
  let out = triples(form, &a_thread());
  let objective = out
    .iter()
    .find(|t| t.name == "objective")
    .expect("the fixture must reach it");
  assert_eq!(objective.value, "line one line two line four");
  for t in &out {
    assert!(
      !t.value.contains('\n') && !t.value.contains('\r') && !t.value.contains('\t'),
      "{t:?} carries a line break"
    );
  }
}

/// **THE POINT OF THE MOVE, ASSERTED RATHER THAN ASSUMED.** If this compiles
/// and passes from a crate that knows nothing about any renderer, then the TUI,
/// the daemon and the menubar app can all reach it -- which is what makes
/// `AC-17.1`'s agreement structural. A copy back up into a face would still
/// pass its own tests and quietly reintroduce the second home.
#[test]
fn the_derivation_needs_no_renderer_and_no_terminal() {
  let l = loaded();
  let form = l.form("issue").expect("the issue form must be declared");
  let out = triples(form, &json!({ "title": "a thing", "number": 157 }));
  assert!(!out.is_empty());
  assert_eq!(
    out
      .iter()
      .find(|t| t.name == "title")
      .map(|t| t.value.as_str()),
    Some("a thing")
  );
  assert_eq!(
    out
      .iter()
      .find(|t| t.name == "number")
      .map(|t| t.value.as_str()),
    Some("157")
  );
}
