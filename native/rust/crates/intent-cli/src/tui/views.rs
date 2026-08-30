//! Entity to rows: the last mapping between the model and the screen.
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

use intentsvcs::form::{Form, Loaded};
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

/// One row per declared field, in declaration order, values read from `entity`.
pub fn rows_for(form: &Form, entity: &Value) -> Vec<Row> {
  form
    .fields
    .iter()
    .map(|f| {
      let value = entity.get(&f.name).map(render_value).unwrap_or_default();
      Row::new(f.label.clone(), value, f.widget.clone())
    })
    .collect()
}

/// One JSON value as one line.
///
/// **Every arm answers on ONE line**, because the layout's guarantee is one row
/// per field and a value carrying a newline would break it at the printer
/// rather than here, where the reason is visible.
fn render_value(v: &Value) -> String {
  match v {
    Value::Null => String::new(),
    Value::Bool(b) => b.to_string(),
    Value::Number(n) => n.to_string(),
    // A collection is its size. See the module note.
    Value::Array(a) => a.len().to_string(),
    // An object is present-or-not. `fiat` is the live case: what matters on the
    // form row is that there IS one, and its content is a descent away.
    Value::Object(_) => "set".to_string(),
    Value::String(s) => one_line(s),
  }
}

/// Collapse newlines so a value cannot become two rows.
///
/// Criterion prose reaches 59,061 characters with paragraph breaks in it; the
/// layout clips to the viewport, but only after this has made it one line.
fn one_line(s: &str) -> String {
  let flat: String = s
    .chars()
    .map(|c| {
      if c == '\n' || c == '\r' || c == '\t' {
        ' '
      } else {
        c
      }
    })
    .collect();
  flat.split_whitespace().collect::<Vec<_>>().join(" ")
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

  /// **A MISSING PROPERTY MUST NOT SHORTEN THE FORM.** Tab order is declaration
  /// order, so a skipped row moves every row after it and the operator's muscle
  /// memory lands on the wrong field.
  #[test]
  fn a_field_the_entity_does_not_carry_renders_empty_rather_than_vanishing() {
    let l = loaded();
    let form = l.form("thread").expect("the thread form must be declared");
    let full = rows_for(form, &a_thread());
    let empty = rows_for(form, &json!({}));
    assert_eq!(
      empty.len(),
      full.len(),
      "an entity carrying nothing produced a shorter form"
    );
    assert!(
      empty.iter().all(|r| r.value.is_empty()),
      "a value appeared for an entity that carries no properties at all"
    );
    // The control: the populated case must actually differ, or the assertion
    // above is comparing two empty forms and proving nothing.
    assert!(
      full.iter().any(|r| !r.value.is_empty()),
      "the populated fixture produced no values, so the comparison above is vacuous"
    );
  }

  /// A collection is its SIZE. Inlining 297 attachments makes the form 325 rows
  /// and breaks the one guarantee the layout makes.
  #[test]
  fn an_array_renders_as_its_size_and_never_as_its_contents() {
    assert_eq!(render_value(&json!([1, 2, 3])), "3");
    assert_eq!(render_value(&json!([])), "0");
    let big: Vec<u32> = (0..297).collect();
    assert_eq!(
      render_value(&json!(big)),
      "297",
      "a large collection must still be one number"
    );
  }

  /// **A VALUE CANNOT BECOME TWO ROWS.** Real criterion prose carries paragraph
  /// breaks; the layout clips to the viewport, but only after this has made the
  /// value one line.
  #[test]
  fn no_rendered_value_can_contain_a_line_break() {
    let l = loaded();
    let form = l.form("thread").expect("the thread form must be declared");
    let rows = rows_for(form, &a_thread());
    let multiline: Vec<&Row> = rows.iter().filter(|r| r.title == "objective").collect();
    assert_eq!(
      multiline.len(),
      1,
      "the fixture must reach the objective row for this to mean anything"
    );
    assert!(
      !multiline[0].value.contains('\n'),
      "a newline survived into a row: {:?}",
      multiline[0].value
    );
    assert_eq!(multiline[0].value, "line one line two line four");
    for r in &rows {
      assert!(
        !r.value.contains('\n') && !r.value.contains('\r'),
        "{r:?} carries a line break"
      );
    }
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
