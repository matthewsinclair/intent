//! Entity to rows: the last mapping between the model and the screen.
//!
//! **THE DERIVATION ITSELF LIVES IN `intentsvcs::form::triples`, ONE CRATE
//! DOWN, AND THIS MODULE ONLY RENDERS WHAT IT RETURNS** (vc, 2026-08-30). It
//! was built here first, which was one crate too high: `intentd` depends on
//! `intentsvcs` and NOT on the CLI, so cc's daemon emitter would have had to
//! write the same walk again -- two homes for one derivation, arriving by the
//! door the argument was meant to shut. The line is DERIVATION shared,
//! RENDERING per face.
//!
//! What follows is the reason the derivation is shaped the way it is, kept
//! here because this is where a reader meets it.
//!
//! **THE FIELD VALUES ARE READ OUT OF THE SERIALISED ENTITY, NOT OUT OF A
//! MATCH.** A `match field { "title" => e.title, "status" => ... }` is a second
//! home for the field set -- exactly what `AC-17.2` refuses one layer up, where
//! the form declaration takes existence and type from the schema instead of
//! listing fields. A hand-written value map would put the list back, in the one
//! place nothing checks it, and it would go stale the day a property is renamed
//! -- silently, because a missing arm looks like an empty value.
//!
//! Serialising and indexing by the declared name has the property the match
//! cannot: **the declaration is the only list, and it is already held against
//! the schema.**
//!
//! **AND IT IS THE SAME MECHANISM THE WEB FACE USES.** `tui-design.md` §10a:
//! the daemon resolves the form declaration server-side and emits a generic
//! `{label, value, widget}` description, and *the JS renders triples, so does
//! SwiftUI, so does the TUI*. Building the TUI's rows any other way would make
//! the two faces agree only by coincidence -- which is what `AC-17.1` exists to
//! refuse.
//!
//! # A collection is a count, never its contents
//!
//! ST0056 carries 297 attachments. Inlining them makes the form 325 rows of
//! which 297 are files, and it breaks the alignment guarantee outright: one
//! aligned name column cannot serve `title` and
//! `parity/tools/conservation_check.sh` at once. **So an array renders as its
//! SIZE and opens its own pane** -- which is also why [`super::nav`] keys
//! descents on arrays.
//!
//! # A field the entity does not carry renders EMPTY, never skipped
//!
//! Skipping would silently shorten the form, and tab order is declaration order
//! (`AC-17.5`) -- so a skipped row moves every row after it and the operator's
//! muscle memory lands on the wrong field. An empty value is visible; a missing
//! row is not.

use intentsvcs::form::{self, Form, Loaded};
use serde_json::Value;

use super::layout::Row;
use super::nav::View;

/// What a view puts on screen, before layout.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rendered {
  /// The APP row's text.
  pub app: String,
  pub rows: Vec<Row>,
}

/// One row per declared field, in declaration order.
///
/// **A MAP, NOT A WALK.** The walk is `intentsvcs::form::triples`; this turns
/// its `{label, value, widget}` into the TUI's row type and does nothing else,
/// which is what keeps the terminal face and the web face agreeing by
/// construction rather than by two people reading the same design section.
pub fn rows_for(form: &Form, entity: &Value) -> Vec<Row> {
  form::triples(form, entity)
    .into_iter()
    .map(|t| Row::named(t.name, t.label, t.value, t.widget))
    .collect()
}

/// The entity kinds, as the root's rows: one per declared form.
pub fn entity_rows(loaded: &Loaded) -> Vec<Row> {
  loaded
    .forms()
    .iter()
    .map(|f| Row::new(f.entity.clone(), String::new(), "button"))
    .collect()
}

/// The APP row's text for a view. **The trail and the exit key belong to the
/// stack, not to this** -- see [`super::nav::Stack::trail`].
pub fn app_line(view: &View) -> String {
  match view {
    View::Entities => "intent".to_string(),
    View::Collection { kind } => kind.clone(),
    View::Item { kind, id } => format!("{kind}  {id}"),
    View::Children { kind, id, field } => format!("{kind}  {id}  {field}"),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde_json::json;

  fn loaded() -> Loaded {
    Loaded::load().expect("the shipped form declaration must load")
  }

  /// A thread-shaped value carrying every kind of JSON the model produces, so
  /// the arms below are driven rather than reasoned about.
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
  fn every_declared_field_gets_exactly_one_row_in_declaration_order() {
    let l = loaded();
    let mut checked = 0usize;
    for form in l.forms() {
      let rows = rows_for(form, &a_thread());
      assert_eq!(
        rows.len(),
        form.fields.len(),
        "{} produced {} rows for {} declared fields",
        form.entity,
        rows.len(),
        form.fields.len()
      );
      for (row, field) in rows.iter().zip(form.fields.iter()) {
        assert_eq!(
          row.title, field.label,
          "row order diverged from declaration order"
        );
        assert_eq!(
          row.kind, field.widget,
          "a row carries a widget the form did not declare"
        );
      }
      checked += 1;
    }
    assert!(
      checked > 0,
      "no form was examined, so this test asserted nothing"
    );
  }

  #[test]
  fn the_root_offers_one_row_per_declared_kind_and_they_are_all_descents() {
    let l = loaded();
    let rows = entity_rows(&l);
    assert!(
      !rows.is_empty(),
      "the root has no rows, so explore opens on nothing"
    );
    assert_eq!(
      rows.len(),
      l.forms().len(),
      "the root is not one row per declared form"
    );
    assert!(
      rows.iter().all(|r| r.kind == "button"),
      "a root row is not a descent"
    );
  }

  /// The APP row says where you are, for every view -- *a way back that is
  /// wired and unlabelled is a way back nobody finds*.
  #[test]
  fn every_view_names_itself_on_the_app_row() {
    let views = [
      View::Entities,
      View::Collection {
        kind: "thread".into(),
      },
      View::Item {
        kind: "thread".into(),
        id: "ST0056".into(),
      },
      View::Children {
        kind: "thread".into(),
        id: "ST0056".into(),
        field: "wps".into(),
      },
    ];
    let mut seen: Vec<String> = Vec::new();
    for v in &views {
      let line = app_line(v);
      assert!(!line.trim().is_empty(), "{v:?} puts nothing on the APP row");
      assert!(
        !seen.contains(&line),
        "{v:?} shares an APP row with another view: {line:?}"
      );
      seen.push(line);
    }
  }
}
