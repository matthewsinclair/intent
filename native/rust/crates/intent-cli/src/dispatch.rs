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
const TABLE: &str = include_str!("../../../../../surface/dispatch-table.json");

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
  /// Which work package owes the command. **Carried, not read** -- it is the
  /// table author's bookkeeping, and the CLI had one consumer for it (the
  /// unbuilt-verb message) until D37 ruled that our work-package numbers do not
  /// reach a user's terminal. Kept because the field is theirs and dropping it
  /// would make the table unparseable for a reason that is not the table's.
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
  /// `keep` · `intrinsic` · `retire` · `pending`.
  #[serde(default)]
  pub disposition: String,
  /// The authored placeholder for the value, eg `<text>` -- what the usage line
  /// should show where clap otherwise falls back to the argument's internal id.
  ///
  /// **35 rows declare it and none of them reached the surface** (ic's
  /// measurement, issue 0035): `intent ac satisfy --help` read `--evidence
  /// <evidence>` where the table says `<ref>`.
  #[serde(default)]
  pub value: Option<String>,
  /// Whether the flag must be supplied.
  #[serde(default)]
  pub required: bool,
  /// The value used when the flag is absent.
  #[serde(default)]
  pub default: Option<String>,
  // `accepts` is deliberately NOT here, and that is a decision rather than an
  // omission. The four rows carrying it are PROSE, in four different shapes --
  // "eg `--lang elixir` or `--lang elixir,rust,shell`", "footgun (default),
  // worked, failed", a `|`-separated list, and a `->` mapping table of
  // case-insensitive synonyms. There is no parse that turns those four into one
  // machine-readable thing, and a `value_parser` built from the two that happen
  // to look like enums would refuse input the other two describe as valid.
  // Two of the four also restate what `value` already carries. It is row
  // documentation for a reader of the table; if any of it needs to reach the
  // surface it belongs in `help`, which is ic's to write.
}

impl Flag {
  /// Whether this flag belongs in the shipped surface.
  ///
  /// **The disposition was honoured at the command level and ignored one level
  /// down**, which is the gap ic raised as EXP-05 and measured: a retired
  /// command is absent from the surface, and a retired FLAG on a shipped
  /// command was built anyway. `--help` advertised what no renderer would
  /// answer, and the table and the binary disagreed with nothing to say so.
  ///
  /// The four values split two ways, and the reason `pending` sits with
  /// `retire` rather than with `keep` is the whole point of the value existing:
  ///
  /// - **`keep`** ships and something must read it.
  /// - **`intrinsic`** ships because CLAP supplies it -- `--help` and friends.
  ///   The spine must not declare these itself or it collides with clap, so
  ///   they are false here and the spelling check above is what lets them
  ///   through clap's own machinery.
  /// - **`retire`** is out by ratification.
  /// - **`pending`** is UNDECIDED, and an undecided flag must not ship. Offering
  ///   it commits the surface to it by fait accompli, which is the failure this
  ///   declaration exists to prevent -- it would answer an open question by
  ///   making one answer true in the binary while the ruling is still open.
  ///
  /// **`ships()` deliberately does not default-allow.** An unrecognised or
  /// empty disposition is out, so a typo or a new value drops a flag from the
  /// surface where ic's check reports it as MISSING, rather than shipping
  /// something nobody classified.
  pub fn ships(&self) -> bool {
    self.disposition == "keep"
  }
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
