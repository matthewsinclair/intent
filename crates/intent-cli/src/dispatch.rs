//! The dispatch table -- the command surface's single source of truth
//! (AC-05.1).
//!
//! ic authored `surface/dispatch-table.json` from the v2 surface: 27 families,
//! 92 entries, each carrying its help text, arguments, flags, observed exit
//! codes, the v2 antecedent as `file:line`, and a target state.
//! `surface/dispatch-table.md` is GENERATED from it; this module reads the JSON.
//!
//! **`surface/` is the authored mirror of `schema/`** (vc ruling, 2026-08-14).
//! `schema/` holds faces generated FROM the Rust types; `surface/` holds the
//! authored table those faces are generated FROM. Same committed-and-drift-
//! checked discipline, opposite direction -- which is why they are two
//! directories rather than one, and the authored-vs-generated line D02 exists
//! to hold stays visible in the layout.
//!
//! It lives at the workspace root rather than in the ST tree because
//! `intent st done` relocates a completed thread (`mv "$CURRENT_DIR"
//! "$NEW_DIR"`, `bin/intent_st:392`) into `intent/st/COMPLETED/`. Compiling
//! the table in from there would have broken the build the moment ST0056 was
//! marked Completed -- which happens in WP-12, the release itself.
//!
//! Root rather than inside this crate because its consumers span crates: the
//! clap surface here, and WP-09's MCP typed tool list and `intent llm` agent
//! guide (AC-09.1, AC-09.4). A crate-local SSOT would make two later work
//! packages reach sideways into a peer crate for their own source of truth.
//!
//! **It is compiled in with `include_str!`, so there is exactly one copy.**
//! Not a markdown parser over the generated view, and not a second
//! hand-authored list beside it -- either would be the drift AC-05.1 exists to
//! prevent, and Intent has paid for a second surface description before.
//!
//! **Deliberate exception to D05.** Unknown fields are PERMITTED here. The
//! table is a specification document that carries far more than the CLI
//! consumes -- provenance, coverage findings, per-entry defect references --
//! and it will grow as ic's parity work continues. Strict rejection belongs on
//! canon the tool owns and writes; this is an artefact the tool reads.

use serde::Deserialize;

/// The committed table, compiled into the binary.
const TABLE: &str = include_str!("../../../surface/dispatch-table.json");

#[derive(Debug, Clone, Deserialize)]
pub struct Table {
  pub schema: String,
  #[serde(default)]
  pub measured_at: String,
  pub families: Vec<Family>,
  #[serde(default)]
  pub invariants: Vec<Invariant>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Family {
  pub name: String,
  pub entries: Vec<Entry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Entry {
  /// Space-separated, eg `st` or `st new`. The family entry is the bare name.
  pub path: String,
  #[serde(default)]
  pub help: String,
  #[serde(default)]
  pub args: Vec<Arg>,
  #[serde(default)]
  pub flags: Vec<Flag>,
  /// The v2 antecedent, as `file:line`.
  #[serde(default)]
  pub v2: String,
  #[serde(default)]
  pub target: Target,
  /// `keep` · `retire` · `pending`.
  #[serde(default)]
  pub disposition: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Arg {
  pub name: String,
  #[serde(rename = "type", default)]
  pub kind: String,
  #[serde(default)]
  pub arity: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Flag {
  #[serde(default)]
  pub spellings: Vec<String>,
  #[serde(rename = "type", default)]
  pub kind: String,
  #[serde(default)]
  pub help: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Target {
  /// `as-observed` · `corrected` · `pending-hv` · `retire` · `undefined`.
  #[serde(default)]
  pub state: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Invariant {
  pub id: String,
  pub title: String,
}

impl Entry {
  /// The family this entry belongs to -- the first path segment.
  pub fn family(&self) -> &str {
    self.path.split(' ').next().unwrap_or(&self.path)
  }

  /// The verb within its family, or `None` for the family entry itself.
  pub fn verb(&self) -> Option<&str> {
    self.path.split_once(' ').map(|(_, verb)| verb)
  }

  /// Whether this entry belongs in the shipped surface.
  ///
  /// `retire` is out by ratification (`organize` and friends are vestigial by
  /// construction). **`pending-hv` is IN**: the surface exists and works; what
  /// awaits hv is a usage-convention question about how it REPORTS, not
  /// whether it is dispatched. Dropping those 17 entries from the spine would
  /// be designing around an open question rather than leaving it open.
  pub fn is_shipped(&self) -> bool {
    self.disposition != "retire" && self.target.state != "retire"
  }
}

/// Parse the compiled-in table. Panics on a malformed table because the table
/// is compiled in: a failure here is a broken build, never bad user input.
pub fn table() -> Table {
  serde_json::from_str(TABLE).expect(
    "the compiled-in dispatch table parses; a failure here means the committed table is malformed, which is a build defect rather than anything a user did",
  )
}

/// Every shipped entry, in table order.
pub fn shipped_entries(table: &Table) -> Vec<&Entry> {
  table
    .families
    .iter()
    .flat_map(|f| f.entries.iter())
    .filter(|e| e.is_shipped())
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn the_compiled_table_parses_and_is_the_expected_shape() {
    let t = table();
    assert!(t.schema.starts_with("intent/dispatch-table@"));
    assert_eq!(t.families.len(), 27, "27 v2 families");
    assert!(
      t.families.iter().flat_map(|f| f.entries.iter()).count() >= 85,
      "the table carries the full entry set"
    );
  }

  #[test]
  fn paths_decompose_into_family_and_verb() {
    let st = Entry {
      path: "st new".to_string(),
      help: String::new(),
      args: vec![],
      flags: vec![],
      v2: String::new(),
      target: Target::default(),
      disposition: "keep".to_string(),
    };
    assert_eq!(st.family(), "st");
    assert_eq!(st.verb(), Some("new"));

    let family = Entry {
      path: "st".to_string(),
      ..st.clone()
    };
    assert_eq!(family.family(), "st");
    assert_eq!(family.verb(), None);
  }

  #[test]
  fn retired_entries_are_not_shipped_but_pending_hv_ones_are() {
    let t = table();
    let shipped = shipped_entries(&t);
    assert!(
      !shipped.iter().any(|e| e.path.starts_with("organize")),
      "organize is a ratified retire (hv, 2026-08-14): vestigial by construction"
    );
    assert!(
      shipped.iter().any(|e| e.target.state == "pending-hv"),
      "a pending usage-convention ruling does not remove a command from the surface"
    );
  }
}
