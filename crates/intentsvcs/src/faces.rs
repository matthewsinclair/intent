//! The generated schema faces (design.md, "one master, three faces").
//!
//! The model types in this crate are the single authored master; this module
//! renders the committed artefacts under `schema/` at the repo root. The
//! JSON Schema face is generated here via schemars; the DDL face is the
//! store's [`crate::store::DDL`] rendered verbatim; the GraphQL SDL face is
//! exported from [`crate::graphql`].
//!
//! `tests/schema_faces_drift.rs` regenerates these and fails on any diff
//! against the committed files. Regenerate with `INTENT_BLESS=1 cargo test
//! -p intentsvcs --test schema_faces_drift` -- the committed face changes
//! only when a type change is deliberate, and always in the same commit.

use schemars::schema_for;

use crate::event::Envelope;
use crate::model::{Issue, Thread};

/// The committed faces, as `(relative path under schema/, content)` pairs.
pub fn faces() -> Vec<(&'static str, String)> {
  vec![
    ("thread.schema.json", schema_json::<Thread>()),
    ("issue.schema.json", schema_json::<Issue>()),
    ("event.schema.json", schema_json::<Envelope>()),
    ("ddl.sql", crate::store::DDL.to_string()),
    ("schema.graphql", crate::graphql::sdl()),
  ]
}

/// One face by name, for `intent schema <face>`. `None` if no face has that
/// name -- the caller reports it; this module does not know the error type.
pub fn face(name: &str) -> Option<String> {
  faces()
    .into_iter()
    .find(|(path, _)| *path == name)
    .map(|(_, content)| content)
}

/// The names of the committed faces, in print order.
pub fn face_names() -> Vec<&'static str> {
  faces().into_iter().map(|(path, _)| path).collect()
}

/// Every face, banner-separated, for a bare `intent schema` (AC-06.5).
///
/// **It GENERATES rather than reading `schema/`.** That is the whole property:
/// AC-06.5 asks that what the command prints be byte-identical to the
/// committed files, and a command that printed the files would satisfy that
/// vacuously -- it would be `cat` with extra steps, and would keep passing
/// after the model and the committed face had drifted apart. Printing from the
/// types makes the command a second, independent witness to the same drift
/// `schema_faces_drift.rs` guards.
pub fn all_faces_banner() -> String {
  let mut out = String::new();
  for (path, content) in faces() {
    out.push_str(&format!("== {path} ==\n"));
    out.push_str(&content);
    if !content.ends_with('\n') {
      out.push('\n');
    }
  }
  out
}

/// Render one type's JSON Schema in canonical form (2-space pretty, trailing
/// newline).
fn schema_json<T: schemars::JsonSchema>() -> String {
  let schema = schema_for!(T);
  let mut out = serde_json::to_string_pretty(&schema)
    .expect("a schemars schema serialises to JSON by construction");
  out.push('\n');
  out
}
