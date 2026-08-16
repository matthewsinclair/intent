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

/// One declared value of a closed-domain field.
///
/// `target_states` spells the key `state` and the two disposition lists spell it
/// `value`; the alias takes both rather than making the reader care which list
/// they are holding.
#[derive(Debug, Clone, Deserialize)]
pub struct Vocab {
  #[serde(alias = "state")]
  pub value: String,
}

/// The dispatch table, as the binary reads it.
///
/// **DELIBERATELY NOT `deny_unknown_fields`, against a rule stated as a
/// blanket** (ic, 2026-08-16). `model.rs` opens with "Strictness (D05): every
/// struct is `deny_unknown_fields`", and that is right for the CANON types --
/// an unknown field in a `thread.json` is a defect and must be refused by name.
/// This is not a canon type. It is a REGISTER: a measurement record that also
/// carries `about` blocks, glosses, `mcp_review` notes, `field_overlap` and the
/// pair matrix, none of which the binary needs and all of which exist to be
/// read by people.
///
/// So the asymmetry is the design. Strict deserialisation here would mean a
/// Rust field for every prose block anyone adds to the register, and the first
/// time someone documented a decision in it the binary would stop loading its
/// own surface. **The note is here because the exemption was undiscoverable**:
/// a correctness-minded reader who has met the blanket rule adds
/// `deny_unknown_fields` for consistency and breaks canon that was never meant
/// to be typed. Newly-added keys deserializing away silently is the intended
/// behaviour, not an oversight -- `legal_pairs` landed exactly that way.
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
  /// **The declared vocabularies, read rather than restated.**
  ///
  /// Each of the three closed-domain string fields below used to be documented
  /// by a doc comment listing its values, and both of those comments were wrong:
  /// the entry disposition said three where the vocabulary has five, and
  /// `target.state` said five where it has six (ic, 2026-08-15). Neither was
  /// noticed, because the act that adds a value is not the act that updates a
  /// comment -- and `banana` on `st start` passed every check in the repo.
  ///
  /// So the values are not written down here at all. They are read from the
  /// table that declares them, and [`table`] refuses any row carrying a value
  /// none of them lists.
  #[serde(default)]
  pub target_states: Vec<Vocab>,
  #[serde(default)]
  pub entry_dispositions: Vec<Vocab>,
  #[serde(default)]
  pub flag_dispositions: Vec<Vocab>,
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
  /// One of [`Table::entry_dispositions`], which is where the values live and
  /// what [`table`] validates against. Not restated here: this comment said
  /// three when the vocabulary had five, and nothing could tell.
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
  /// One of [`Table::flag_dispositions`]. This one happens to be RIGHT today,
  /// and it is going anyway: its two siblings were both wrong, and a
  /// hand-written copy that is currently accurate is the same mechanism a
  /// beat earlier.
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
  /// One of [`Table::target_states`]. Not restated here: this comment said five
  /// when the vocabulary had six, missing `new-surface` -- the second-largest
  /// class at 18 rows.
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
  let table: Table = serde_json::from_str(TABLE).expect(
    "the compiled-in dispatch table parses; a failure here means the committed table is malformed, which is a build defect rather than anything a user did",
  );
  if let Err(unknown) = check_vocabularies(&table) {
    panic!(
      "the dispatch table carries values no vocabulary declares:\n  {}\n\
       Each is a closed domain declared in the table itself (`entry_dispositions`, `target_states`, \
       `flag_dispositions`); a value outside one is a typo or an undeclared addition, and either is \
       a build defect.",
      unknown.join("\n  ")
    );
  }
  table
}

/// Every closed-domain value in the table is one the table declares.
///
/// **This exists because the two readers of those fields fail in opposite
/// directions and neither one is safe alone.** `Entry::is_shipped` is
/// `disposition != "retire"`, which fails OPEN: `retre` ships a retired
/// command. `Flag::ships` is `disposition == "keep"`, which fails CLOSED and
/// silently -- a typo drops a flag from the surface with nothing in the build
/// to say so, and only an external check nobody runs on a push reports it.
///
/// Measured by ic: 25 of 111 rows carry one fact in two fields (`disposition`
/// and `target.state` move in perfect lockstep on all 19 `new-surface` and all
/// 6 `retire` rows), and that UNDECLARED redundancy was the only thing stopping
/// a single hand-edit from shipping a retired command.
///
/// Refusing at load is stronger than either polarity, and it makes the choice
/// between them stop mattering: an unrecognised value never reaches a reader at
/// all. It is also where the strictness belongs -- the table is compiled in, so
/// this is a build defect and never something a user did. vc found the hole by
/// putting `banana` on `st start` and watching every check in the repo pass.
fn check_vocabularies(table: &Table) -> Result<(), Vec<String>> {
  let names = |v: &[Vocab]| -> Vec<String> { v.iter().map(|x| x.value.clone()).collect() };
  let states = names(&table.target_states);
  let entry_dispositions = names(&table.entry_dispositions);
  let flag_dispositions = names(&table.flag_dispositions);

  // An empty vocabulary would make every check below vacuous, so the absence of
  // a declaration is itself the first thing refused.
  let mut unknown = Vec::new();
  for (what, declared) in [
    ("target_states", &states),
    ("entry_dispositions", &entry_dispositions),
    ("flag_dispositions", &flag_dispositions),
  ] {
    if declared.is_empty() {
      unknown.push(format!(
        "{what} declares no values at all, so nothing below it could be checked"
      ));
    }
  }
  if !unknown.is_empty() {
    return Err(unknown);
  }

  for entry in table
    .families
    .iter()
    .flat_map(|f| f.entries.iter())
    .chain(table.new_surface.iter())
  {
    if !entry_dispositions.contains(&entry.disposition) {
      unknown.push(format!(
        "`{}` disposition {:?} is not in entry_dispositions",
        entry.path, entry.disposition
      ));
    }
    if !states.contains(&entry.target.state) {
      unknown.push(format!(
        "`{}` target.state {:?} is not in target_states",
        entry.path, entry.target.state
      ));
    }
    for flag in &entry.flags {
      if !flag_dispositions.contains(&flag.disposition) {
        unknown.push(format!(
          "`{}` flag {:?} disposition {:?} is not in flag_dispositions",
          entry.path, flag.spellings, flag.disposition
        ));
      }
    }
  }

  if unknown.is_empty() {
    Ok(())
  } else {
    Err(unknown)
  }
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

  /// **`banana` on `st start` passed every check in the repo** (vc's probe,
  /// 2026-08-15), because these are bare `String`s with `#[serde(default)]` and
  /// nothing compared them to the vocabulary that declares them.
  ///
  /// Driven with that exact value, on all three fields, and paired with the
  /// real table coming back clean -- a refusal that fires on everything is not
  /// a check.
  #[test]
  fn a_value_no_vocabulary_declares_is_refused() {
    assert!(
      check_vocabularies(&table()).is_ok(),
      "the committed table is conformant, or every case below passes for the wrong reason"
    );

    let mut bad = table();
    bad.families[0].entries[0].disposition = "banana".to_string();
    let err = check_vocabularies(&bad).expect_err("an undeclared disposition is refused");
    assert!(
      err.iter().any(|e| e.contains("banana")),
      "the refusal names the offending value: {err:?}"
    );

    let mut bad = table();
    bad.families[0].entries[0].target.state = "banana".to_string();
    assert!(
      check_vocabularies(&bad).is_err(),
      "target.state is checked too -- it is the field that said five when it had six"
    );

    let mut bad = table();
    let flagged = bad
      .families
      .iter_mut()
      .flat_map(|f| f.entries.iter_mut())
      .find(|e| !e.flags.is_empty())
      .expect("some entry declares a flag");
    flagged.flags[0].disposition = "banana".to_string();
    assert!(
      check_vocabularies(&bad).is_err(),
      "and flag dispositions, which fail CLOSED and silently when unrecognised"
    );

    // An empty vocabulary must refuse rather than accept everything: a check
    // whose declared set is missing would otherwise pass on every row.
    let mut hollow = table();
    hollow.entry_dispositions.clear();
    assert!(
      check_vocabularies(&hollow).is_err(),
      "an absent vocabulary makes every row vacuously conformant"
    );
  }

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
