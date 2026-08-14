//! The generated schema faces (design.md, "one master, three faces").
//!
//! The model types in this crate are the single authored master; this module
//! renders the committed artefacts under `schema/` at the repo root. The
//! JSON Schema face is generated here via schemars; the DDL face is the
//! store's [`crate::store::DDL`] rendered verbatim; the GraphQL SDL face
//! lands with the async-graphql schema (WP-04).
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
  ]
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
