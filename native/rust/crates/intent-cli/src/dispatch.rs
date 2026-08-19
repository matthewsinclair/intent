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

use serde::{Deserialize, Serialize};

/// The committed table, compiled into the binary.
///
/// `pub` so the one path literal has one home: `canon_keys_are_read.rs` reads
/// the same bytes this module parses, rather than reaching for the file by a
/// second `include_str!` that agrees until someone moves the table.
pub const TABLE: &str = include_str!("../../../../../surface/dispatch-table.json");

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
///
/// **And it is one of TWO mechanisms, which is why `key_classes` closes only
/// half the class** (cc, 2026-08-16, sent as text rather than edited in because
/// this file was live). The other half is a field that EXISTS, deserializes
/// correctly, and has no consumer: `Config.st_prefix` was the measured
/// instance. **It never lands in a `rest` map, so a key-set check reports
/// agreement**, and `dead_code` does not fire because a `pub` field on a `pub`
/// struct in a lib crate is reachable by definition. **The discriminator that
/// separates them is vc's: not "is this key read" but "does a consumer exist
/// and encode the value another way".**
///
/// That instance is gone -- hv retired `st_prefix`. The MECHANISM is what this
/// note is about, and the next instance will not be called `st_prefix`.
#[derive(Debug, Clone, Deserialize)]
pub struct Table {
  pub schema: String,
  #[serde(default)]
  pub measured_at: String,
  /// The root command's `--help` line (EXP-08).
  ///
  /// **Deliberately NOT `#[serde(default)]`, unlike `measured_at` above.** A
  /// missing `measured_at` degrades a provenance stamp; a missing root help
  /// would render an EMPTY about line on `intent --help` and look like a
  /// styling choice. That is the silent-empty class this contract has spent a
  /// day removing, so a table without this key refuses to load rather than
  /// shipping a blank first impression.
  pub root_help: String,
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
  /// The closed domain of [`Entry::recoverability`], declared beside the other
  /// three rather than written into a doc comment -- which is the mistake both
  /// of those comments made, and neither could tell.
  #[serde(default)]
  pub recoverability_values: Vec<Vocab>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Family {
  pub name: String,
  pub entries: Vec<Entry>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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
  /// The v2 spellings that must keep working, eg `done` for `at green`.
  ///
  /// **Issue 0039: this was authored on five rows and absent from this struct**,
  /// so serde dropped it in silence and `at done` / `at notdone` -- which v2
  /// documents in its own help as "Aliases for green | red" -- did not exist in
  /// the binary at all. Four of the five rows are `disposition: keep`, which is
  /// the one classification that promises the v2 spelling survives, so the
  /// table declared four commands and shipped two.
  ///
  /// Registered by the spine for rows that SHIP, never unconditionally: `st
  /// organise` is an alias on a `retire` row, and registering it would bring a
  /// retired command back through its old spelling.
  #[serde(default)]
  pub aliases: Vec<String>,
  /// Spellings that are ACCEPTED and never SHOWN.
  ///
  /// **A second list rather than a flag on the first, because they are two
  /// dispositions rather than one list with an attribute.** `aliases` above is
  /// registered VISIBLE on the stated ground that v2 documents those spellings
  /// (`done|notdone`) and a hidden one would be undiscoverable -- "a different
  /// way of not shipping it". That reasoning is correct and stays.
  ///
  /// **This list is for the opposite case: a spelling accepted as a COURTESY
  /// that must never become a second name for the verb.** hv, 2026-08-19, on
  /// `organise`: _handle 'organise' and 'organize' but only ever show the 'z'
  /// version to keep things simple._ A courtesy spelling rendered anywhere --
  /// `--help`, the guide, an error message, the table's own view -- stops being
  /// a courtesy and becomes a second documented name, which is the outcome the
  /// ruling exists to prevent.
  ///
  /// **The tell is built in: if one of these ever appears in output, something
  /// is echoing the user's spelling back instead of naming the verb.**
  #[serde(default)]
  pub hidden_aliases: Vec<String>,
  /// Whether WP-09's MCP tool tier exposes this command (AC-09.1).
  ///
  /// **Deliberately NOT `#[serde(default)]`.** All 112 rows carry it and ic's
  /// generator refuses an unclassified key, so a row without it is a broken
  /// table rather than an older one -- and the two plausible defaults are both
  /// wrong to pick silently. `false` would quietly withhold a command from the
  /// agent surface; `true` would quietly offer one. Refusing to load says which
  /// row is missing it, which is the only answer that does not guess.
  pub exposed_on_mcp: bool,
  /// `read` or `mutate` -- whether invoking this command can change the estate.
  ///
  /// Not `default` for the same reason, with a sharper edge: this is the field
  /// an agent tier gates safety on, and an absent value defaulting to `read`
  /// would present an unclassified command as safe to call unattended.
  pub read_or_mutate: String,
  /// `reversible` / `idempotent` / `one-way` -- **can this surface put the
  /// estate back?** Declared on MUTATIONS only; `None` on a `read`, where the
  /// question is vacuous.
  ///
  /// **This field replaced one that was disproved before it shipped, and the
  /// disproof is why it is worth reading twice.** vc ruled that the policy
  /// withholding 13 leaves from MCP earns a declared field, and proposed
  /// `acts_upon` -- one modelled entity / the estate / the environment. The
  /// canary killed it: `lang init` and `lang remove` act upon the IDENTICAL
  /// thing and sit on opposite sides of the partition, as do `agents init`
  /// against `agents generate`, and `claude upgrade` against `claude skills`.
  /// **Any function of one field returns one answer for rows sharing that
  /// field's value**, so no classification of that property could reproduce
  /// the partition -- three families independently, which rules out a bad row.
  ///
  /// Recoverability is the property the policy was always about: nobody
  /// withheld `lang remove` because of what it touches, they withheld it
  /// **because you cannot get back what it deletes.** It survives any ruling
  /// about MCP, and it is the field a `--dry-run`, a confirmation prompt or an
  /// undo stack would each read.
  ///
  /// **CLASSIFIED AGAINST SHIPPED BEHAVIOUR, NEVER AGAINST INTENT** (vc's
  /// ruling). `at green` is `one-way` today because issue 0033 destroys the
  /// row's authored note, so the documented round trip moves the status back
  /// and does not restore the prior state. A field describing what a command is
  /// SUPPOSED to do is the `doctor` failure in advance -- there, `read_or_mutate`
  /// went on describing a `--fix` that had been retired underneath it, and the
  /// reasoning stayed sound about a subject that no longer existed.
  ///
  /// `Option` rather than a required `String` because the honest domain is
  /// mutations. The totality property -- every shipped mutation declares it, no
  /// read does -- is enforced by `gen_dispatch_table.sh`, which can say WHICH
  /// row is missing it. serde can only say that something was.
  #[serde(default)]
  pub recoverability: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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
  /// What the slot means when the caller leaves it out.
  ///
  /// **Deserialized and validated, deliberately not rendered as a clap
  /// `default_value` yet, and the reason is in the data.** Eight rows carry it
  /// and seven are literals, but `init` reads `"the current directory name"`,
  /// which is a DESCRIPTION OF A COMPUTATION rather than a value (ic,
  /// measured). Wiring the field straight through would make `intent init`
  /// name a project `the current directory name`, which is the confidently
  /// wrong behaviour that having the field at all was supposed to prevent.
  ///
  /// The discriminator is the arg's own `type`, not a list of exempt names:
  /// `enum` and `subcommand` have a CLOSED domain, so a default has to name a
  /// member of it and [`check_vocabularies`] checks that it does. `string` has
  /// an open domain, so nothing can tell a value from a description of one --
  /// and the only row that is not a literal is the only `string` row.
  #[serde(default)]
  pub default: Option<String>,
}

impl Arg {
  /// Whether the caller must supply this slot -- ie whether its minimum is at
  /// least one.
  ///
  /// **These two predicates live on the model rather than in the spine because
  /// the spine stopped being their only reader** (ic, 2026-08-16). WP-09's
  /// agent guide renders the same arity into a usage line, and an arity is a
  /// fact about the table, not about clap -- a second reading of `0..n` in a
  /// second module is the Highlander failure with a delay fuse on it, since
  /// the two would agree until the table grew a fifth spelling.
  ///
  /// The vocabulary is four values and it is measured, not assumed: `1` (69),
  /// `0..1` (23), `0..n` (3) and `1..n` (2) across the shipped set.
  ///
  /// **`1..n` IS required, and `spine.rs` does not yet agree** -- a divergence
  /// this extraction found rather than introduced. `positionals` reads
  /// `arity == "1"` inline, so `intent lang init` with NO language PARSES and
  /// falls through to the unimplemented-command path, where v2 refuses it
  /// (`bin/intent_lang:251`, "missing language argument(s)"). Measured against
  /// the built binary: `at green` with its arguments absent is refused at exit
  /// 1, `lang init` with its argument absent is not refused at all. Latent
  /// only because `lang` is unwired; the day WP-07 wires it, the renderer is
  /// handed an empty list.
  ///
  /// **Stated as the SEMANTICS here rather than as the spine's behaviour**, so
  /// the guide tells an agent `<lang>...` -- which is true of the table and
  /// true of v2. The one-line repair is to point `positionals` at this method;
  /// it was deliberately not made in the same change, because `spine.rs` was
  /// held by a peer and a behaviour change split across two commits is worse
  /// than a divergence that is written down.
  pub fn required(&self) -> bool {
    self.arity == "1" || self.arity == "1..n"
  }

  /// Whether the slot takes more than one value.
  ///
  /// **`0..n` is the table's open-ended spelling and carries neither `+` nor
  /// `*`**, so the obvious check for those two alone reads it as a single
  /// value -- which is the defect this predicate was extracted carrying, and
  /// the reason `ends_with('n')` is here rather than a tidier two-arm test.
  ///
  /// Proven by [`tests::an_arity_is_read_the_same_way_by_every_reader`], which
  /// drives all four declared spellings and asserts the two open-ended ones
  /// come back repeated. Co-located deliberately: a proof that lives only in a
  /// commit message cannot answer "has this ever refused anything" without a
  /// `git log --follow` nobody runs before trusting a green.
  pub fn repeated(&self) -> bool {
    self.arity.contains('+') || self.arity.contains('*') || self.arity.ends_with('n')
  }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Target {
  /// One of [`Table::target_states`]. Not restated here: this comment said five
  /// when the vocabulary had six, missing `new-surface` -- the second-largest
  /// class at 18 rows.
  #[serde(default)]
  pub state: String,
  /// What the command is spelled as instead, where a retirement has a
  /// replacement. Authored on `st_zero` (`intent st bootstrap`) and absent
  /// everywhere else, because most retirements replace nothing.
  ///
  /// **Read for the MESSAGE, never for dispatch, and the distinction is vc's
  /// ruling rather than a nicety.** vc refused to teach the spine to read this
  /// as a rename -- a general rename facility for a population of one reads as
  /// foresight and ships as unused surface, and aliasing the old spelling would
  /// make the row assert `corrected` ("survives, renamed") where hv ratified
  /// `retire` ("the root spelling dies"). Telling someone what to type instead
  /// asserts neither: the command is gone, and this names where the capability
  /// went. That is precisely what issue 0044 asks the retired class to say.
  ///
  /// **It was also declared and undeserialised until now**, which is issue
  /// 0039's class -- a key authored on a row that serde silently dropped.
  #[serde(default)]
  pub spelling: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Invariant {
  pub id: String,
  pub title: String,
}

impl Table {
  /// Every entry the table retires, LONGEST PATH FIRST.
  ///
  /// The order is the whole contract of this function. A caller matching a
  /// command line against these prefixes must try `st organize` before `st`,
  /// or a two-segment retirement is answered by whatever one-segment row
  /// happens to sit above it -- and today that would be silent, because no
  /// retired path is currently a prefix of another. **A sort that is only
  /// correct because the data has not yet reached the case it guards is a sort
  /// worth having before it does.**
  pub fn retired(&self) -> Vec<&Entry> {
    let mut out: Vec<&Entry> = self
      .families
      .iter()
      .flat_map(|f| f.entries.iter())
      .filter(|e| !e.is_shipped())
      .collect();
    out.sort_by_key(|e| std::cmp::Reverse(e.path.split(' ').count()));
    out
  }
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

  /// Every spelling this entry answered to in v2 -- its own path first, then
  /// its aliases, each as whole space-separated segments.
  ///
  /// Whole paths rather than last segments, because the caller here is matching
  /// against argv from the front. `alias_verbs` below answers a different
  /// question (what clap registers on an already-placed subcommand) and the two
  /// must not be confused: `st organise` is one segment to clap and two to
  /// anything reading a command line.
  pub fn spellings(&self) -> Vec<Vec<&str>> {
    std::iter::once(self.path.as_str())
      .chain(self.aliases.iter().map(String::as_str))
      .map(|s| s.split(' ').filter(|seg| !seg.is_empty()).collect())
      .collect()
  }

  /// The alias spellings as clap registers them: each alias's last segment.
  ///
  /// The prefix is validated at load ([`check_vocabularies`]), so by the time
  /// anything calls this an alias is known to belong to the command it sits on.
  pub fn alias_verbs(&self) -> Vec<&str> {
    self
      .aliases
      .iter()
      .map(|alias| {
        alias
          .rsplit_once(' ')
          .map(|(_, last)| last)
          .unwrap_or(alias)
      })
      .collect()
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
  let recoverability = names(&table.recoverability_values);

  // An empty vocabulary would make every check below vacuous, so the absence of
  // a declaration is itself the first thing refused.
  let mut unknown = Vec::new();
  for (what, declared) in [
    ("target_states", &states),
    ("entry_dispositions", &entry_dispositions),
    ("flag_dispositions", &flag_dispositions),
    ("recoverability_values", &recoverability),
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
    // **A `mutate` declares it, a `read` must NOT, and both halves refuse.**
    // The absent half is the one worth stating: a `read` carrying a
    // recoverability is a row whose classification was copied rather than
    // decided, and it would render a line in the agent guide answering a
    // question nobody asked of it.
    match (entry.read_or_mutate.as_str(), entry.recoverability.as_deref()) {
      ("mutate", Some(r)) if !recoverability.contains(&r.to_string()) => unknown.push(format!(
        "`{}` recoverability {r:?} is not in recoverability_values",
        entry.path
      )),
      ("mutate", None) if entry.is_shipped() => unknown.push(format!(
        "`{}` is a shipped mutation and declares no recoverability -- the field an agent reads to know whether this can be undone",
        entry.path
      )),
      ("read", Some(r)) => unknown.push(format!(
        "`{}` is a read and declares recoverability {r:?} -- the question is vacuous for a command that changes nothing",
        entry.path
      )),
      _ => {}
    }
    for flag in &entry.flags {
      if !flag_dispositions.contains(&flag.disposition) {
        unknown.push(format!(
          "`{}` flag {:?} disposition {:?} is not in flag_dispositions",
          entry.path, flag.spellings, flag.disposition
        ));
      }
    }
    // An alias is written as a FULL path (`at done` beside `at green`), so the
    // spelling clap registers is its last segment and everything before it has
    // to be this entry's own prefix. If they disagree the alias names a
    // different command, and registering its last segment here would attach it
    // to this one silently -- an alias that works and points somewhere nobody
    // wrote down.
    for alias in &entry.aliases {
      if prefix(alias) != prefix(&entry.path) {
        unknown.push(format!(
          "`{}` declares the alias {:?}, which is not in its own family",
          entry.path, alias
        ));
      }
    }
  }

  // **A default on a CLOSED domain has to name a member of it.** Run with the
  // family in hand, because a `subcommand` slot's domain is usually its sibling
  // verbs rather than a `values` array -- `todo` declares `default: "list"` and
  // no values at all, and `list` is a sibling entry. Checking only the rows that
  // carry `values` would be the narrower question wearing the wider one's name.
  //
  // `string` args are not checked and cannot be: their domain is open, so
  // `init`'s "the current directory name" is indistinguishable from a literal
  // by anything mechanical. That is why the field is not rendered.
  for family in &table.families {
    let siblings: Vec<&str> = family.entries.iter().filter_map(|e| e.verb()).collect();
    for entry in &family.entries {
      unknown.extend(unreachable_defaults(entry, &siblings));
    }

    // **A family with shipped verbs DECLARES how its verb slot is filled.**
    //
    // The spine reads that arity to decide `subcommand_required`, and it used
    // to fall back to "required" when the declaration was absent -- a default
    // nobody wrote down, which is how v3 answered `intent config` at exit 1
    // where v2 exits 0. **Absence was standing in for a decision**, and the two
    // are indistinguishable by inspection.
    //
    // Refusing it here is what lets the spine assert instead of defaulting.
    // vc measured the residue before this landed: twelve family roots declare
    // no slot, all twelve are single-entry LEAVES the spine's branch never
    // reaches, so **filtered to families with sibling verbs the count is
    // ZERO** -- this refuses nothing that exists and closes the way back in.
    let shipped_verbs = family
      .entries
      .iter()
      .any(|e| e.verb().is_some() && e.is_shipped());
    let declares_slot = family
      .entries
      .iter()
      .filter(|e| e.verb().is_none())
      .any(|e| e.args.iter().any(|a| a.kind == "subcommand"));
    if shipped_verbs && !declares_slot {
      unknown.push(format!(
        "`{}` has shipped verbs and its family row declares no `subcommand` arg -- so whether the bare command is legal would be decided by a default rather \
         than by the table",
        family.name
      ));
    }
  }
  for entry in &table.new_surface {
    unknown.extend(unreachable_defaults(entry, &[]));
  }

  if unknown.is_empty() {
    Ok(())
  } else {
    Err(unknown)
  }
}

/// Any `enum` or `subcommand` default on this entry that names nothing.
fn unreachable_defaults(entry: &Entry, siblings: &[&str]) -> Vec<String> {
  let mut bad = Vec::new();
  for arg in &entry.args {
    let Some(default) = arg.default.as_deref() else {
      continue;
    };
    let domain: Vec<&str> = match arg.kind.as_str() {
      "enum" => arg.values.iter().map(String::as_str).collect(),
      "subcommand" if !arg.values.is_empty() => arg.values.iter().map(String::as_str).collect(),
      "subcommand" => siblings.to_vec(),
      // An open domain. Nothing to check against, and saying so is the point.
      _ => continue,
    };
    if domain.is_empty() {
      bad.push(format!(
        "`{}` arg `{}` defaults to {:?} and its {} domain is empty, so the default names nothing",
        entry.path, arg.name, default, arg.kind
      ));
    } else if !domain.contains(&default) {
      bad.push(format!(
        "`{}` arg `{}` defaults to {:?}, which is not one of its {} values ({})",
        entry.path,
        arg.name,
        default,
        arg.kind,
        domain.join(", ")
      ));
    }
  }
  bad
}

/// Everything in a command path before its last segment; `""` for a bare name.
fn prefix(path: &str) -> &str {
  path.rsplit_once(' ').map(|(head, _)| head).unwrap_or("")
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

  /// **A family with shipped verbs must DECLARE how its verb slot is filled**,
  /// rather than leaving the spine to default it.
  ///
  /// The default was `required`, which is how v3 answered `intent config` at
  /// exit 1 where v2 exits 0 -- and absence standing in for a decision is
  /// indistinguishable by inspection from a decision somebody made. This is the
  /// guarantee half of the pair; `spine::build` asserts the same fact rather
  /// than defaulting, so a drift between them is a panic at build rather than a
  /// silently different surface.
  #[test]
  fn a_family_with_verbs_must_declare_how_its_verb_slot_is_filled() {
    assert!(
      check_vocabularies(&table()).is_ok(),
      "the committed table already satisfies this -- vc measured the residue at ZERO for families with sibling verbs, and if that were false this check would be \
       proposing to break the build rather than to hold a line"
    );

    let mut bad = table();
    let family = bad
      .families
      .iter_mut()
      .find(|f| {
        f.entries
          .iter()
          .any(|e| e.verb().is_some() && e.is_shipped())
          && f
            .entries
            .iter()
            .any(|e| e.verb().is_none() && e.args.iter().any(|a| a.kind == "subcommand"))
      })
      .expect("some shipped family declares a subcommand slot");
    let name = family.name.clone();
    for entry in family.entries.iter_mut().filter(|e| e.verb().is_none()) {
      entry.args.retain(|a| a.kind != "subcommand");
    }

    let err =
      check_vocabularies(&bad).expect_err("a family with verbs and no declared slot is refused");
    assert!(
      err.iter().any(|e| e.contains(&name)),
      "the refusal names the family whose declaration is missing, because the fix is one row: {err:?}"
    );
  }

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
      aliases: vec![],
      exposed_on_mcp: false,
      read_or_mutate: "read".to_string(),
      recoverability: None,
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

  /// **A default that names nothing is refused, and an open domain is not
  /// checked at all** -- driven here rather than by editing the table, which
  /// belongs to another node.
  ///
  /// Both arms matter and only together. Checking the closed domains without
  /// exempting `string` would refuse `init`'s `"the current directory name"`,
  /// which is correct as authored: it is a description of a computation, and
  /// refusing it would push someone to invent a literal that the CLI would then
  /// use as a project name. Exempting `string` without checking the rest would
  /// be an exemption wearing a check's name.
  #[test]
  fn a_default_outside_a_closed_domain_is_refused_and_an_open_one_is_left_alone() {
    let slot = |kind: &str, values: &[&str], default: &str| Arg {
      name: "command".to_string(),
      kind: kind.to_string(),
      arity: "0..1".to_string(),
      values: values.iter().map(|v| v.to_string()).collect(),
      default: Some(default.to_string()),
    };
    let with = |arg: Arg| Entry {
      path: "todo".to_string(),
      help: String::new(),
      args: vec![arg],
      flags: vec![],
      v2: String::new(),
      target: Target::default(),
      disposition: "keep".to_string(),
      owner_wp: String::new(),
      aliases: vec![],
      exposed_on_mcp: false,
      read_or_mutate: "read".to_string(),
      recoverability: None,
    };

    assert!(
      unreachable_defaults(&with(slot("enum", &["info", "design"], "banana")), &[]).len() == 1,
      "an enum default outside its own values names nothing"
    );
    assert!(
      unreachable_defaults(&with(slot("enum", &["info", "design"], "info")), &[]).is_empty(),
      "and one inside them is fine -- a refusal that fires on everything is not a check"
    );
    assert!(
      unreachable_defaults(&with(slot("subcommand", &[], "list")), &["list", "update"]).is_empty(),
      "a subcommand slot with no values takes its domain from the SIBLING VERBS, which is how the \
       four rows that declare `default: list` and no values are legal"
    );
    assert!(
      unreachable_defaults(&with(slot("subcommand", &[], "list")), &["update"]).len() == 1,
      "and the sibling check is real: no `list` sibling, no reachable default"
    );
    assert!(
      unreachable_defaults(
        &with(slot("string", &[], "the current directory name")),
        &[]
      )
      .is_empty(),
      "an OPEN domain is not checked, because nothing can tell a value from a description of one"
    );
  }

  #[test]
  fn retired_entries_are_not_shipped_but_pending_hv_ones_are() {
    let t = table();
    let shipped = shipped_entries(&t);
    // **THE RETIRE IS OF A FACE, NOT OF A WORD** (hv, 2026-08-19). This used to
    // read `!e.path.starts_with("organize")`, which forbade the TOKEN -- and a
    // token check cannot tell a name being reclaimed from a command being
    // resurrected. hv retired `bin/intent_organize` and `st organize`, the pair
    // that moved ST directories into status folders; ST0057's `organize` is a
    // different program that reconciles the tree against `.intentfiles` and
    // shares nothing with it but the spelling.
    //
    // Keyed on the v2 ANTECEDENT, which is the thing that was actually retired
    // and the one field a reclaimed name cannot accidentally satisfy: a
    // new-surface row carries `new-surface` there by construction.
    assert!(
      !shipped.iter().any(|e| e.v2 == "bin/intent_organize"),
      "the v2 `organize` face is a ratified retire (hv, 2026-08-14): a strictly structured model cannot hold data in the wrong place, so the disorder it repaired cannot arise"
    );
    // And the reclamation is ASSERTED, not merely permitted. Loosening the line
    // above without this one would leave the v3 verb free to vanish from the
    // table with nothing noticing -- a check that stops forbidding something is
    // not the same as one that requires it.
    assert!(
      shipped
        .iter()
        .any(|e| e.path == "organize" && e.v2 == "new-surface"),
      "the v3 `organize` verb is reclaimed (hv, 2026-08-19) and must ship"
    );
    assert!(
      shipped.iter().any(|e| e.target.state == "pending-hv"),
      "a pending usage-convention ruling does not remove a command from the surface"
    );
  }

  /// **All three arms of the recoverability rule, including the one that is
  /// about ABSENCE.** A vocabulary check that only rejects bad values passes a
  /// table where the field was never written -- which is the declared-but-not-
  /// deserialized class this register has now produced six instances of, and
  /// the reason the field is `Option` with the totality enforced here instead
  /// of by serde: serde can say something was missing, this can say WHICH.
  #[test]
  fn a_mutation_declares_its_recoverability_and_a_read_must_not() {
    assert!(
      check_vocabularies(&table()).is_ok(),
      "the committed table is conformant, or every case below passes for the wrong reason"
    );

    // Mutating in place rather than through a helper that hands out a
    // reference: a closure returning `&'static mut` needs `unsafe` to satisfy
    // the borrow checker, and reaching for `unsafe` to make a TEST compile is
    // how a test starts proving something about a program that does not exist.
    fn tamper(t: &mut Table, want: &str, value: Option<&str>) {
      let e = t
        .families
        .iter_mut()
        .flat_map(|f| f.entries.iter_mut())
        .find(|e| e.read_or_mutate == want && e.is_shipped())
        .expect("the surface has both kinds");
      e.recoverability = value.map(str::to_string);
    }

    let mut bad = table();
    tamper(&mut bad, "mutate", Some("banana"));
    let err = check_vocabularies(&bad).expect_err("an undeclared value is refused");
    assert!(
      err.iter().any(|e| e.contains("banana")),
      "the refusal names the offending value: {err:?}"
    );

    let mut bad = table();
    tamper(&mut bad, "mutate", None);
    let err = check_vocabularies(&bad).expect_err("a shipped mutation must declare it");
    assert!(
      err.iter().any(|e| e.contains("declares no recoverability")),
      "the refusal says the field is ABSENT rather than merely wrong: {err:?}"
    );

    let mut bad = table();
    tamper(&mut bad, "read", Some("reversible"));
    assert!(
      check_vocabularies(&bad).is_err(),
      "a read changes nothing, so a recoverability on it was copied rather than decided"
    );
  }

  /// The proof for [`Arg::required`] and [`Arg::repeated`], co-located with
  /// the predicates rather than left in a commit message.
  ///
  /// **Driven over the DECLARED vocabulary, and then asserted to BE the
  /// declared vocabulary**, which is the half that does not go stale: a fifth
  /// arity spelling added to the table fails the second assertion by name
  /// instead of being read by whichever arm of `repeated()` happens to catch
  /// it. Four cases passing proves the four cases; only the closure check
  /// proves there are four.
  ///
  /// **The first version of this test asserted `!required()` for `1..n` and
  /// passed** -- because it was written by reading the implementation it was
  /// meant to check, so it agreed with the defect. What caught it was a test
  /// in `guide.rs` written from the MEANING of the delimiters (`<x>` required,
  /// `[x]` optional), which had no way to inherit the mistake. A test derived
  /// from the code under test is a restatement wearing a green tick.
  #[test]
  fn an_arity_is_read_the_same_way_by_every_reader() {
    let arg = |arity: &str| Arg {
      name: "x".to_string(),
      kind: "string".to_string(),
      arity: arity.to_string(),
      values: vec![],
      default: None,
    };

    for required in ["1", "1..n"] {
      assert!(
        arg(required).required(),
        "`{required}` has a minimum of one, so the caller must supply it -- `1..n` is the case spine.rs still reads as optional"
      );
    }
    for optional in ["0..1", "0..n"] {
      assert!(
        !arg(optional).required(),
        "`{optional}` admits zero, which is the whole of what the leading `0` says"
      );
    }

    assert!(
      !arg("1").repeated(),
      "a single required value is not a list"
    );
    assert!(
      !arg("0..1").repeated(),
      "an optional single value is not a list"
    );
    for many in ["0..n", "1..n"] {
      assert!(
        arg(many).repeated(),
        "`{many}` is the table's open-ended spelling and carries neither `+` nor `*` -- the trap"
      );
    }

    let declared: std::collections::BTreeSet<String> = shipped_entries(&table())
      .iter()
      .flat_map(|e| e.args.iter())
      .map(|a| a.arity.clone())
      .filter(|a| !a.is_empty())
      .collect();
    let covered: std::collections::BTreeSet<String> = ["1", "0..1", "0..n", "1..n"]
      .iter()
      .map(|s| s.to_string())
      .collect();
    assert_eq!(
      declared, covered,
      "the table declares an arity these predicates were never driven over -- add the case, do not widen a match arm"
    );
  }
}
