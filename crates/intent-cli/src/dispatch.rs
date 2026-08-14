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
  /// Commands v3 ADDS, with no v2 antecedent to port or deviate from.
  ///
  /// A separate array rather than more `families` rows, because the two are
  /// different kinds of claim: a `families` entry asserts something measurable
  /// about v2 at `measured_at`, and one of these asserts a design intention.
  /// Merging them would make the table's own provenance unreadable -- there
  /// would be no way to ask "what did v2 offer?" without first filtering out
  /// rows that describe no v2 at all.
  ///
  /// They reach the surface through the same [`shipped_entries`] as everything
  /// else, because from the operator's side there is no difference: `intent
  /// search` is a command or it is not.
  #[serde(default)]
  pub new_surface: Vec<Entry>,
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
  /// The v2 antecedent, as `file:line`, or `new-surface` where there is none.
  #[serde(default)]
  pub v2: String,
  #[serde(default)]
  pub target: Target,
  /// `keep` · `retire` · `pending`.
  #[serde(default)]
  pub disposition: String,
  /// The work package that owes this command, eg `WP-06`. Carried on
  /// `new_surface` rows; empty on ported ones, whose owner is WP-06 by default.
  ///
  /// This is what lets an unbuilt verb name the work package that owes it
  /// instead of a hardcoded number. The first version of that message said
  /// WP-06 for everything, which would have been a lie for `daemon` and `mcp`
  /// the moment anyone read it.
  #[serde(default)]
  pub owner_wp: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Arg {
  pub name: String,
  #[serde(rename = "type", default)]
  pub kind: String,
  #[serde(default)]
  pub arity: String,
  /// For a `subcommand`-kind arg: the verbs that fill the slot.
  ///
  /// This is how the table expresses the surface's THIRD level -- `intent
  /// claude skills install` is `claude skills` with `install` in its verb slot,
  /// not a `claude skills install` row of its own. A spine that ignored these
  /// dropped 20-odd real commands and, where a free-form positional sat beside
  /// the slot, silently accepted invented ones.
  #[serde(default)]
  pub values: Vec<String>,
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

  /// The work package that owes this command. Ported entries default to WP-06,
  /// the CLI parity long tail; `new_surface` rows carry their own.
  pub fn owner(&self) -> &str {
    if self.owner_wp.is_empty() {
      "WP-06"
    } else {
      &self.owner_wp
    }
  }
}

/// Parse the compiled-in table. Panics on a malformed table because the table
/// is compiled in: a failure here is a broken build, never bad user input.
pub fn table() -> Table {
  serde_json::from_str(TABLE).expect(
    "the compiled-in dispatch table parses; a failure here means the committed table is malformed, which is a build defect rather than anything a user did",
  )
}

/// Every shipped entry, ported and added alike, in table order.
pub fn shipped_entries(table: &Table) -> Vec<&Entry> {
  table
    .families
    .iter()
    .flat_map(|f| f.entries.iter())
    .chain(table.new_surface.iter())
    .filter(|e| e.is_shipped())
    .collect()
}

/// Find one entry by its full path, eg `st new` or `search`.
pub fn entry<'a>(table: &'a Table, path: &str) -> Option<&'a Entry> {
  shipped_entries(table).into_iter().find(|e| e.path == path)
}

/// The work package that owes `path`, for a verb that parses but does not run.
///
/// Falls back to WP-06 for a path the table does not carry at all -- which
/// cannot normally happen, since the spine is built FROM the table, and is a
/// build defect rather than anything the operator can act on.
pub fn owner_of(table: &Table, path: &str) -> String {
  entry(table, path).map_or("WP-06", Entry::owner).to_string()
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
      owner_wp: String::new(),
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
