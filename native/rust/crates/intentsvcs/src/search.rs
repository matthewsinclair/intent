//! **THE ONE SEARCH ANSWER, AND NOTHING IN IT IS ASSEMBLED TWICE** (AC-19.1,
//! AC-19.2).
//!
//! Every search surface -- the terminal, `--json`, the MCP tool, and the
//! explorer's pane when WP-21 builds it -- renders THIS value, produced by one
//! facade call. The alternative, which is what a CLI usually grows, is each
//! surface assembling its own shape from the same rows: they agree on the day
//! they are written and drift the first time one of them gains a field.
//!
//! The types here are pure. They hold no store, read no disk and ask nothing
//! what time it is; the impure half is `Facade::search_all`, which fills them.
//!
//! **A TIER IS A GROUP AND A CORPUS IS AN ENTRY** -- the design's structural
//! claim, and the reason this file exists before the corpus it will carry.
//! WP-20's structural tier and WP-23's semantic tier each add a GROUP, so the
//! CLI's contract and the MCP tool's schema do not move when they land. cc's
//! source corpus adds an ENTRY to [`IndexFreshness::corpora`] and rows to the
//! lexical group, and moves nothing at all. If adding a corpus ever looks like
//! it needs a group, one of the two halves is wrong and it is an argument
//! rather than a patch.

use crate::index::corpus::{Corpus, SkipReason};
use crate::index::resolved::{self, Row, Run};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// The staleness policy the canon corpus is indexed under (D24).
///
/// **It is a constant HERE only until cc's `file_index` carries the column**
/// (WP-18), at which point this value is read from the row like every other
/// corpus's and this constant goes. It is stated rather than omitted because a
/// freshness block that names the corpus and not how its freshness is decided
/// tells a reader the question was never asked.
pub const CANON_POLICY: &str = "hash";

/// The key a corpus appears under in [`IndexFreshness::corpora`].
///
/// **THE TAXONOMY IS cc's [`crate::index::corpus::Corpus`] AND THIS IS ITS ONLY
/// RENDERING** (vc, 2026-09-12: the `corpus` column's values are the
/// namespace). A `&str` constant per corpus in this file would be a second
/// spelling of a taxonomy that already has a home, and the two would agree
/// until a corpus was added. This match is exhaustive, so a new variant breaks
/// the build HERE, which is the failure that gets fixed rather than the one
/// that ships.
pub fn corpus_key(corpus: &Corpus) -> &'static str {
  match corpus {
    Corpus::Canon => "canon",
    Corpus::Prose => "prose",
    Corpus::Code { .. } => "code",
  }
}

/// One search answer.
///
/// **BOTH DENOMINATORS TRAVEL** (`matched`, `returned`), the `events` page
/// pattern: a capped result that reported only what it returned would be a
/// silent subset, and a caller cannot tell one from a complete answer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchAnswer {
  /// The operator's words, not the FTS5 expression they became.
  pub query: String,
  pub index: IndexFreshness,
  /// One group per tier PRESENT. A tier with no hits is an EMPTY group and
  /// never an absent one: absence would say the tier does not exist.
  pub groups: Vec<TierGroup>,
  /// A tier the caller asked for by name that this project cannot answer, with
  /// the reason (issue 0356).
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub unanswered: Vec<Unanswered>,
  /// What an answer asked by target asked (ST0076 WP-07); absent on every
  /// other answer.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub target: Option<TargetAsked>,
  /// What the index matched across the groups carried, before any cap.
  pub matched: usize,
  /// The rows the answer carries: the length of every group's hits.
  pub returned: usize,
}

impl SearchAnswer {
  /// What a reader of this answer's symbol hits must know before trusting
  /// them, or `None` when there is nothing to say.
  ///
  /// **IT NAMES THE LEVEL THAT ANSWERED** (ST0076 WP-04, AC-04.1): which of
  /// the three levels the hits were read at, what that level means, and for
  /// each language in the answer what its references do not cover yet. A
  /// reference list that did not say so would be read as every caller.
  ///
  /// **AND WHERE LEVEL 3 COULD NOT ANSWER** (ST0076 WP-07, AC-07.1): each
  /// language among the symbol hits whose level 3 is not current, in
  /// [`resolution_words`]. An answer asked by target names every such language
  /// in the index, hits or none, because every hit it could hold is level 3's;
  /// and one whose target no resolved row names says it cannot tell a target
  /// nothing references from a misspelt one.
  pub fn symbol_note(&self) -> Option<String> {
    let symbols: Vec<(&Hit, &SymbolFacts)> = self
      .groups
      .iter()
      .flat_map(|group| group.hits.iter())
      .filter_map(|hit| hit.symbol.as_ref().map(|facts| (hit, facts)))
      .collect();
    let mut levels: Vec<u8> = symbols.iter().map(|(_, facts)| facts.level).collect();
    levels.sort_unstable();
    levels.dedup();
    let mut langs: Vec<&str> = symbols
      .iter()
      .filter_map(|(hit, _)| hit.lang.as_deref())
      .collect();
    langs.sort_unstable();
    langs.dedup();
    let mut parts: Vec<String> = levels
      .iter()
      .map(|level| format!("level {level}: {}", level_words(*level)))
      .collect();
    parts.extend(
      langs
        .iter()
        .filter_map(|lang| crate::index::symbols::what_a_reference_misses(lang))
        .map(str::to_string),
    );
    parts.extend(
      self
        .index
        .resolution
        .iter()
        .filter(|(lang, _)| self.target.is_some() || langs.contains(&lang.as_str()))
        .map(|(lang, state)| resolution_words(lang, state)),
    );
    if let Some(asked) = &self.target
      && !asked.named
    {
      parts.push(format!(
        "no resolved reference names `{}`: {UNKNOWN_TARGET}",
        asked.target
      ));
    }
    if parts.is_empty() {
      None
    } else {
      Some(parts.join("; "))
    }
  }
}

/// What an answer asked by target asked (ST0076 WP-07, AC-07.2).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetAsked {
  /// The target as the caller spelled it.
  pub target: String,
  /// Whether any resolved row names it. An answer about a target none names is
  /// empty, and its note says [`UNKNOWN_TARGET`].
  pub named: bool,
}

/// What an empty answer about a target no resolved row names says, and the
/// register quotes verbatim (vc's refinement of AC-07.2).
pub const UNKNOWN_TARGET: &str = "a target nothing references and a misspelt one read the same";

/// What a symbol row's `level` means, in the words every surface uses.
pub fn level_words(level: u8) -> &'static str {
  match level {
    1 => {
      "read from the file's syntax and matched by name, never resolved to the definition it names"
    }
    2 => {
      "read from the file's syntax with the qualifier it was written with, a Rust path as written or an Elixir module with the file's own aliases and `__MODULE__` expanded, still never resolved to the definition it names"
    }
    3 => "resolved to the definition it names by the language's own toolchain",
    _ => "a level this build does not describe",
  }
}

/// One language's level 3 as a search answer reports it when it is not
/// current (ST0076 WP-07, AC-07.1).
///
/// **NO COUNTS.** What a run matched and dropped is `intent index status`'s to
/// report; an answer carries what a reader must know before trusting a hit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolutionState {
  /// `missing`, `failed`, `stale` or `unresolved`.
  pub state: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub tool: Option<String>,
  /// Where a failed run failed, when the tool named it.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub path: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub line: Option<u32>,
  /// What the tool said, for a run that stored nothing.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub detail: Option<String>,
  /// The files whose resolved rows no longer describe the bytes the index
  /// holds, in path order. A reference in one keeps its syntax level.
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub stale: Vec<String>,
}

impl ResolutionState {
  /// A run's record as an answer reports it, or `None` when the run stored
  /// and nothing it stored has gone stale.
  ///
  /// **A RUN THAT FAILED KEEPS ITS STATE WHATEVER IS STALE**, and lists the
  /// stale files beside it: the failure is why nothing newer was stored, which
  /// is the fact a reader needs first.
  pub fn of(run: &Run) -> Option<Self> {
    let current = run.state == resolved::CURRENT;
    if current && run.stale.is_empty() {
      return None;
    }
    Some(Self {
      state: if current {
        resolved::STALE.to_string()
      } else {
        run.state.clone()
      },
      tool: Some(run.tool.clone()).filter(|tool| !tool.is_empty()),
      path: run.path.clone(),
      line: run.line,
      detail: run.detail.clone(),
      stale: run.stale.clone(),
    })
  }
}

/// A resolver this build carries, as a search answer reads one (ST0076 WP-07).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Carried {
  pub lang: &'static str,
  pub tool: &'static str,
  pub manifest: resolved::Manifest,
}

/// The carried resolvers an answer asks the index about before naming their
/// language: each whose last run did not store, and each the project declares
/// that no run has recorded. Whether the index holds each one's manifest is
/// asked of these alone.
pub fn manifest_questions<'a>(
  runs: &BTreeMap<String, Run>,
  declared: &[String],
  carried: &'a [Carried],
) -> Vec<&'a Carried> {
  carried
    .iter()
    .filter(|reader| match runs.get(reader.lang) {
      Some(run) => run.state != resolved::CURRENT,
      None => declared.iter().any(|d| d == reader.lang),
    })
    .collect()
}

/// Every language whose level 3 is not current (ST0076 WP-07, AC-07.1): each
/// run on record that did not store or has stale files, and each declared
/// language no run has recorded, as `unresolved`.
///
/// `carried` is every resolver this build carries, and `applies` those of
/// [`manifest_questions`] whose manifest the index holds.
///
/// **A LANGUAGE THE PROJECT HOLDS NOTHING FOR IS NOT NAMED** (vc, 2026-09-17),
/// whether no run is on record or the last one failed. An estate that declares
/// Elixir and holds no `mix.exs` would otherwise say so in every answer, for as
/// long as it had no `mix.exs`, and nothing a person could run would change it:
/// the permanent false alarm the store keeps out, arriving through the envelope
/// instead. The record itself is left as it is, so `intent index status` still
/// shows the failed run. A run of a language this build does not carry has no
/// manifest to ask about, and is named as recorded.
pub fn resolution_states(
  runs: &BTreeMap<String, Run>,
  carried: &[Carried],
  applies: &[&Carried],
) -> BTreeMap<String, ResolutionState> {
  let carries = |lang: &str| carried.iter().any(|reader| reader.lang == lang);
  let holds = |lang: &str| applies.iter().any(|reader| reader.lang == lang);
  let mut out: BTreeMap<String, ResolutionState> = runs
    .iter()
    .filter(|(lang, run)| run.state == resolved::CURRENT || !carries(lang) || holds(lang))
    .filter_map(|(lang, run)| ResolutionState::of(run).map(|state| (lang.clone(), state)))
    .collect();
  for reader in applies
    .iter()
    .filter(|reader| !runs.contains_key(reader.lang))
  {
    out.insert(
      reader.lang.to_string(),
      ResolutionState {
        state: resolved::UNRESOLVED.to_string(),
        tool: Some(reader.tool.to_string()),
        path: None,
        line: None,
        detail: None,
        stale: Vec::new(),
      },
    );
  }
  out
}

/// What each state of a language's level 3 means, in the words every surface
/// uses and the register quotes verbatim (ST0076 WP-07, AC-07.1).
pub fn resolution_phrase(state: &str) -> &'static str {
  match state {
    resolved::MISSING => {
      "its toolchain is not where Intent can run it, so only what an earlier run stored answers at level 3"
    }
    resolved::FAILED => {
      "its last resolution run failed, so only what an earlier run stored answers at level 3"
    }
    resolved::STALE => {
      "the files it lists changed since they were resolved, so their references answer at their syntax level until the next run"
    }
    resolved::UNRESOLVED => {
      "no resolution run has stored its references here, so they answer at their syntax level"
    }
    _ => "its level 3 is in a state this build does not describe",
  }
}

/// How many stale files a note names before it counts the rest. A bound on
/// the note, not a measurement: `index.resolution` lists every one.
const STALE_NAMED: usize = 5;

/// One language's level 3, when it is not current, as the sentence every
/// surface prints: the state, what it means, and the facts that go with it.
///
/// **THE ONE HOME FOR THESE WORDS** (AC-07.1). The terminal's note and the
/// explorer's INFO row both reach it through [`SearchAnswer::symbol_note`], and
/// the register quotes [`resolution_phrase`] for each state.
pub fn resolution_words(lang: &str, state: &ResolutionState) -> String {
  let mut facts: Vec<String> = Vec::new();
  match state.state.as_str() {
    resolved::MISSING | resolved::FAILED => {
      let at = state.path.as_ref().map(|path| match state.line {
        Some(line) => format!("at {path}:{line}"),
        None => format!("at {path}"),
      });
      let head = [state.tool.clone(), at]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(" ");
      let fact = match (&state.detail, head.is_empty()) {
        (Some(detail), true) => detail.clone(),
        (Some(detail), false) => format!("{head}: {detail}"),
        (None, _) => head,
      };
      if !fact.is_empty() {
        facts.push(fact);
      }
    }
    // **THE VERB IS NAMED AS A PERSON'S, NEVER AS A NEXT STEP** (vc,
    // 2026-09-17): it runs the project's build, which is why it is withheld
    // from the MCP tool tier, so an agent reading this is told what resolves
    // the language and not invited to resolve it.
    resolved::UNRESOLVED => {
      if let Some(tool) = &state.tool {
        facts.push(format!(
          "a person resolves it by running `intent index resolve`, which runs the project's build through {tool}"
        ));
      }
    }
    _ => {}
  }
  if !state.stale.is_empty() {
    let named: Vec<&str> = state
      .stale
      .iter()
      .take(STALE_NAMED)
      .map(String::as_str)
      .collect();
    let more = state.stale.len() - named.len();
    facts.push(match more {
      0 => format!("stale: {}", named.join(", ")),
      more => format!("stale: {} and {more} more", named.join(", ")),
    });
  }
  let mut words = format!(
    "level 3 in `{lang}` ({}): {}",
    state.state,
    resolution_phrase(&state.state)
  );
  for fact in facts {
    words.push_str(" -- ");
    words.push_str(&fact);
  }
  words
}

/// A tier asked for by name that did not answer, and why.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Unanswered {
  pub tier: Tier,
  pub reason: String,
}

/// **FRESHNESS IS PART OF EVERY ANSWER** (S5, AC-19.3), not a separate
/// question a caller has to know to ask. A search that cannot say whether its
/// index was complete has answered a different question from the one asked,
/// and the estate's dominant defect class is exactly that: an empty result
/// that reads as an absence.
#[derive(Debug, Clone)]
pub struct IndexFreshness {
  /// When the index was last reconciled, as the reconcile's own write stamped
  /// it.
  ///
  /// **NOTHING HERE READS A CLOCK** (D42, `one_clock.rs`): this is a column,
  /// written by the statement that performed the reconcile, and it is `None`
  /// until WP-18's reconcile exists to write it. A time read at render would
  /// be a plausible value that no event produced.
  pub reconciled_at: Option<String>,
  /// Whether THIS answer's search reconciled the index against the tree before
  /// it queried (issue 0484).
  ///
  /// **`false` IS THE DEFAULT, AND ONLY THE DOOR THAT RECONCILED SETS `true`.**
  /// A plain search skips its reconcile where a daemon is watching the project
  /// (0443 Q1), `--no-reconcile` skips it by request, and the daemon never
  /// reconciles per query -- in all three the answer is the index as it stands,
  /// and a file written since `reconciled_at` may be missing from it while
  /// `complete` still reads `true`, because `complete` is about what the index
  /// read, not about the tree. This is the field that tells a caller which of
  /// the two it got. Defaulting to `false` means no door can claim a reconcile
  /// it did not run.
  pub reconciled: bool,
  /// Keyed by the corpus names the index itself uses (`canon`, and cc's
  /// `prose` and `code` when WP-18 lands). The keys are not enumerated here:
  /// a corpus the index gains appears in the answer the day it is added.
  pub corpora: BTreeMap<String, CorpusState>,
  /// What was in scope and deliberately not indexed, each with its reason.
  /// **A named exclusion, never a silent absence** (AC-13.2's rule, applied to
  /// the answer rather than to the corpus). A skip by policy is listed here
  /// too, and only a skip that [`Skipped::leaves_a_gap`] makes the answer
  /// incomplete (issue 0430).
  pub skipped: Vec<Skipped>,
  /// Paths whose indexed bytes no longer match the disk.
  pub stale: Vec<String>,
  /// Each language a structural answer could not see, and why (issue 0548).
  /// Empty for a question the lexical tier answered, which read every file's
  /// text whether or not a grammar could parse it.
  pub unindexed: Vec<Unindexed>,
  /// Each language whose level 3 is not current, keyed as `index_file.lang`
  /// spells it (ST0076 WP-07, AC-07.1), and empty when every language that
  /// resolves is current.
  ///
  /// **INDEX-WIDE, AND NOT PART OF `complete`.** `complete` says whether the
  /// index read the text an answer covers; level 3 is a claim about what a
  /// reference names, and each symbol hit states its own `level`. Every answer
  /// carries this, because a caller reading a reference at level 1 needs to
  /// know whether level 3 was there to be had.
  pub resolution: BTreeMap<String, ResolutionState>,
}

/// **`complete` IS COMPUTED AT SERIALISATION AND IS NOT A FIELD.** It is
/// [`IndexFreshness::complete`] over `skipped`, `stale` and `unindexed`, and holding it as a
/// field beside the lists that determine it would be a second home for one fact whose
/// failure mode is the flag saying complete while the lists say otherwise.
/// This is the whole reason `IndexFreshness` writes its own `Serialize` rather
/// than deriving one.
impl Serialize for IndexFreshness {
  fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    use serde::ser::SerializeStruct;
    let fields = 7 + usize::from(self.reconciled_at.is_some());
    let mut out = serializer.serialize_struct("IndexFreshness", fields)?;
    out.serialize_field("complete", &self.complete())?;
    out.serialize_field("reconciled", &self.reconciled)?;
    if let Some(at) = &self.reconciled_at {
      out.serialize_field("reconciled_at", at)?;
    }
    out.serialize_field("corpora", &self.corpora)?;
    out.serialize_field("skipped", &self.skipped)?;
    out.serialize_field("stale", &self.stale)?;
    out.serialize_field("unindexed", &self.unindexed)?;
    out.serialize_field("resolution", &self.resolution)?;
    out.end()
  }
}

/// **AND `complete` IS RECOMPUTED ON THE WAY IN, NEVER READ.** The envelope now
/// crosses a wire (`Op::Search`, WP-22), so it must come back as well as go
/// out -- and the field that is computed on the way out is exactly the field a
/// deserialiser must not trust. Reading it would let a peer's `complete: true`
/// sit beside a non-empty `stale`, which is the one contradiction this type is
/// shaped to make impossible. Serde ignores an unknown field by default, so
/// `complete` arrives, is not named here, and is derived again from the two
/// lists that determine it.
impl<'de> Deserialize<'de> for IndexFreshness {
  fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    #[derive(Deserialize)]
    struct Carried {
      #[serde(default)]
      reconciled_at: Option<String>,
      #[serde(default)]
      reconciled: bool,
      corpora: BTreeMap<String, CorpusState>,
      skipped: Vec<Skipped>,
      stale: Vec<String>,
      // A daemon built before issue 0548 sends no list, and its answer reads
      // as it did then.
      #[serde(default)]
      unindexed: Vec<Unindexed>,
      #[serde(default)]
      resolution: BTreeMap<String, ResolutionState>,
    }
    let carried = Carried::deserialize(deserializer)?;
    Ok(Self {
      reconciled_at: carried.reconciled_at,
      reconciled: carried.reconciled,
      corpora: carried.corpora,
      skipped: carried.skipped,
      stale: carried.stale,
      unindexed: carried.unindexed,
      resolution: carried.resolution,
    })
  }
}

impl IndexFreshness {
  /// The freshness of an index holding the given corpora, with nothing skipped,
  /// nothing stale, and no language's level 3 to report.
  pub fn new(corpora: BTreeMap<String, CorpusState>) -> Self {
    Self {
      reconciled_at: None,
      reconciled: false,
      corpora,
      skipped: Vec::new(),
      stale: Vec::new(),
      unindexed: Vec::new(),
      resolution: BTreeMap::new(),
    }
  }

  /// Whether this index answered the whole question it was asked: nothing is
  /// stale, no skip left a gap, and no language the question needed symbols
  /// from was one the index could not see (issue 0548).
  ///
  /// **A SKIP BY POLICY IS NOT INCOMPLETENESS** (issue 0430). Every whole-tree
  /// query on this repository answered `complete: false` for its binaries and
  /// one symlink, which no text query could match, and the tool's description
  /// sends a reader of `complete: false` to grep -- so the flag sent every
  /// query to grep while meaning nothing.
  pub fn complete(&self) -> bool {
    self.stale.is_empty() && self.gaps().next().is_none() && self.unindexed.is_empty()
  }

  /// The skips that make this answer partial.
  pub fn gaps(&self) -> impl Iterator<Item = &Skipped> {
    self.skipped.iter().filter(|skip| skip.leaves_a_gap())
  }

  /// Say that this answer's search reconciled the index before it queried.
  /// Called only by a door that ran the reconcile (issue 0484).
  pub fn mark_reconciled(&mut self) {
    self.reconciled = true;
  }

  /// What a reader is owed when this answer did not reconcile first, or `None`
  /// when it did (issue 0484). The terminal prints it on stderr. The explorer's
  /// pane keeps its one info row for the symbol level and the partial-index
  /// notes, which its own tests hold, and carries `reconciled` in the envelope.
  pub fn unreconciled_note(&self) -> Option<String> {
    if self.reconciled {
      return None;
    }
    let since = self
      .reconciled_at
      .as_deref()
      .unwrap_or("the last reconcile");
    Some(format!(
      "answered from the index as it stands, without reconciling it against the tree first -- a file changed since {since} may be missing"
    ))
  }

  /// Record a path whose indexed bytes no longer match the disk.
  pub fn mark_stale(&mut self, path: impl Into<String>) {
    let path = path.into();
    if !self.stale.contains(&path) {
      self.stale.push(path);
    }
  }

  /// Record something in scope that was not indexed, and why.
  pub fn mark_skipped(&mut self, path: impl Into<String>, reason: impl Into<String>) {
    self.skipped.push(Skipped {
      path: path.into(),
      reason: reason.into(),
    });
  }

  /// Record a language a structural answer could not see (issue 0548).
  pub fn mark_unindexed(&mut self, unindexed: Unindexed) {
    if !self.unindexed.contains(&unindexed) {
      self.unindexed.push(unindexed);
    }
  }

  /// **AN INDEX THAT HOLDS NOTHING IS NOT A COMPLETE INDEX.** Without this an
  /// unpopulated store answers every query the way a genuine miss does --
  /// exit 0, zero hits -- which is the confident wrong answer AC-19.3 forbids,
  /// in the one case where it is most believable.
  pub fn is_empty(&self) -> bool {
    self.corpora.values().all(|corpus| corpus.files == 0)
  }
}

/// One corpus's state: how its freshness is decided, and how much of it there
/// is.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorpusState {
  /// The staleness policy, in the index's own words (`hash`,
  /// `stat-then-hash`). Rendered, never parsed.
  pub policy: String,
  /// Files indexed under this corpus. **A count with a reader**: it is what
  /// makes an empty result interpretable, which is the one use a bare number
  /// has.
  pub files: usize,
}

/// Something in scope that was not indexed, and why.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skipped {
  pub path: String,
  pub reason: String,
}

impl Skipped {
  /// Whether this skip makes a text answer partial. **A reason this build
  /// cannot read counts as a gap**: the envelope crosses a wire, and a spelling
  /// nobody here decided is never reported as whole.
  pub fn leaves_a_gap(&self) -> bool {
    SkipReason::parse(&self.reason).is_none_or(SkipReason::leaves_a_gap)
  }
}

/// A language a structural answer could not see, and why (issue 0548).
///
/// **A FILE NO GRAMMAR PARSES IS A GAP IN A STRUCTURAL ANSWER, NOT AN EMPTY
/// FILE.** A def, outline or context question finds nothing in it, and until
/// this the answer said `complete` over that nothing -- so a reader told to fall
/// back to grep only when the index is not complete never did, and read "no
/// definition" as the answer, which is also how a Highlander check concludes
/// that a name is free.
///
/// `lang` is `None` for an outline of a file the index gives no language: it
/// knows a language only by a file's extension
/// ([`crate::index::corpus::lang_of`]), so a script without one is unparsed
/// whatever it is written in.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Unindexed {
  pub lang: Option<String>,
  /// [`crate::index::symbols::Readiness`]'s spelling for a build with no
  /// working grammar, [`Unindexed::NOT_DECLARED`], or
  /// [`Unindexed::NO_LANGUAGE`].
  pub reason: String,
}

impl Unindexed {
  /// A file whose extension names no language the index parses.
  pub const NO_LANGUAGE: &'static str = "no-language";
  /// A language this build has a grammar for and the project does not declare:
  /// the reconcile extracts symbols for declared languages only.
  pub const NOT_DECLARED: &'static str = "not-declared";

  /// What a reader is owed, in the one wording every face prints.
  pub fn words(&self) -> String {
    match &self.lang {
      Some(lang) => format!(
        "the index names no symbols in {lang} ({}), nor in any file without an extension, so a definition there is not in this answer -- search those files as text",
        self.reason
      ),
      None => "the index names no symbols in this file: it knows a language only by a file's extension, and this one names none it parses -- read the file".to_string(),
    }
  }
}

/// Why the index names no symbols in `lang`'s files, or `None` when it does.
fn blind_to(lang: &str, declared: &[String]) -> Option<String> {
  use crate::index::symbols::{Readiness, readiness};
  match readiness(lang) {
    Readiness::Ready if declared.iter().any(|d| d == lang) => None,
    Readiness::Ready => Some(Unindexed::NOT_DECLARED.to_string()),
    other => Some(other.as_str().to_string()),
  }
}

/// The languages a structural answer could not see (issue 0548, on the rule
/// vc ruled on 2026-09-24 and recorded in the issue).
///
/// A language counts when the index names no symbols in it and EITHER the
/// project declares it OR the index holds a file of it within the answer's
/// path. **THE DECLARED HALF IS WHAT REACHES A SCRIPT WITHOUT AN EXTENSION**:
/// the index gives such a file no language, so no file of the language is ever
/// held for it, and a project that writes its shell that way -- Devbin's `lib/`
/// and `cmd/`, this repository's `bin/` and `.githooks/` -- would otherwise
/// answer complete over every one. The declared half reads only the names the
/// index gives files, so `author` and `content`, disciplines rather than source
/// languages, never count. A `--lang` filter that leaves a language out drops
/// it from both halves.
pub fn unindexed_for<'a>(
  declared: &[String],
  held: impl IntoIterator<Item = (&'a str, &'a str)>,
  ask: &SearchQuery,
) -> Vec<Unindexed> {
  let mut langs: BTreeSet<String> = declared
    .iter()
    .filter(|lang| crate::critic::HEADLESS_LANGUAGES.contains(&lang.as_str()))
    .cloned()
    .collect();
  langs.extend(
    held
      .into_iter()
      .filter(|(path, _)| {
        ask
          .path
          .as_deref()
          .is_none_or(|glob| glob_matches(glob, path))
      })
      .map(|(_, lang)| lang.to_string()),
  );
  langs
    .into_iter()
    .filter(|lang| ask.langs.is_empty() || ask.langs.contains(lang))
    .filter_map(|lang| {
      blind_to(&lang, declared).map(|reason| Unindexed {
        lang: Some(lang),
        reason,
      })
    })
    .collect()
}

/// Why an outline of `path` names no symbols, or `None` when the index parses
/// it (issue 0548, vc's outline rule): the file has no language the index
/// parses, or one it names no symbols in. **AN EMPTY OUTLINE OF AN UNPARSED
/// FILE IS NOT AN ANSWER.** A `--lang` filter that leaves the file out asks
/// nothing of it, and a file with no language is left out by any.
pub fn unindexed_outline(path: &str, declared: &[String], ask: &SearchQuery) -> Option<Unindexed> {
  let lang = crate::index::corpus::lang_of(std::path::Path::new(path));
  if !ask.langs.is_empty() && !lang.is_some_and(|lang| ask.langs.iter().any(|asked| asked == lang))
  {
    return None;
  }
  match lang {
    None => Some(Unindexed {
      lang: None,
      reason: Unindexed::NO_LANGUAGE.to_string(),
    }),
    Some(lang) => blind_to(lang, declared).map(|reason| Unindexed {
      lang: Some(lang.to_string()),
      reason,
    }),
  }
}

/// The hits of one tier, ranked within it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierGroup {
  pub tier: Tier,
  pub hits: Vec<Hit>,
}

/// **GROUPED BY TIER, RANKED WITHIN A TIER, NEVER BLENDED** (S4). A blended
/// score across tiers is one nobody designed and nobody can debug: lexical
/// relevance and structural exactness are not the same quantity, and adding
/// them produces an order that no rule explains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Tier {
  Lexical,
  Structural,
  Semantic,
}

impl Tier {
  /// Whether this tier was asked for. An empty filter asks for all of them.
  pub fn asked(self, filter: &[Tier]) -> bool {
    filter.is_empty() || filter.contains(&self)
  }

  /// Parse the spelling a caller gives `--tier`.
  pub fn parse(word: &str) -> Option<Self> {
    Some(match word {
      "lexical" => Tier::Lexical,
      "structural" => Tier::Structural,
      "semantic" => Tier::Semantic,
      _ => return None,
    })
  }

  /// Every spelling `--tier` accepts, for the refusal that names them.
  ///
  /// **`semantic` IS IN THE VOCABULARY AND NOT IN THE BUILD** (WP-23). A filter
  /// naming it is accepted and answers nothing, which is honest: the tier is
  /// declared and unbuilt, and refusing the word would say it does not exist.
  pub const ALL: &'static [&'static str] = &["lexical", "structural", "semantic"];

  pub fn as_str(self) -> &'static str {
    match self {
      Tier::Lexical => "lexical",
      Tier::Structural => "structural",
      Tier::Semantic => "semantic",
    }
  }
}

/// What a hit IS. The canon kinds are entities; `file` is a file on disk;
/// `def` and `ref` arrive with WP-20's symbols.
///
/// **`ref` IS A NAME-MATCHED OCCURRENCE AND IS NAMED AS SUCH ON EVERY
/// SURFACE.** Without type resolution nothing here can say *caller*, and a
/// surface that said it would be the confident wrong answer this estate
/// refuses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HitKind {
  Def,
  Ref,
  File,
  Thread,
  Wp,
  Issue,
  Attachment,
  Project,
}

impl HitKind {
  pub fn as_str(self) -> &'static str {
    match self {
      HitKind::Def => "def",
      HitKind::Ref => "ref",
      HitKind::File => "file",
      HitKind::Thread => "thread",
      HitKind::Wp => "wp",
      HitKind::Issue => "issue",
      HitKind::Attachment => "attachment",
      HitKind::Project => "project",
    }
  }

  /// The kind an owner_type in the prose index stands for.
  ///
  /// **AN UNRECOGNISED OWNER IS A `file`, NOT A PANIC AND NOT A DROP.** The
  /// prose index carries whatever the model owns, and a kind this enum has not
  /// met is still a real hit whose path is still the answer.
  pub fn of_owner(owner_type: &str) -> Self {
    match owner_type {
      "thread" | "steel_thread" => HitKind::Thread,
      "wp" | "work_package" => HitKind::Wp,
      "issue" => HitKind::Issue,
      "attachment" => HitKind::Attachment,
      "project" => HitKind::Project,
      _ => HitKind::File,
    }
  }

  /// Parse the spelling a caller gives `--kind`.
  pub fn parse(word: &str) -> Option<Self> {
    Some(match word {
      "def" => HitKind::Def,
      "ref" => HitKind::Ref,
      "file" => HitKind::File,
      "thread" => HitKind::Thread,
      "wp" => HitKind::Wp,
      "issue" => HitKind::Issue,
      "attachment" => HitKind::Attachment,
      "project" => HitKind::Project,
      _ => return None,
    })
  }

  /// Every spelling `--kind` accepts, for the refusal that names them.
  pub const ALL: &'static [&'static str] = &[
    "def",
    "ref",
    "file",
    "thread",
    "wp",
    "issue",
    "attachment",
    "project",
  ];
}

/// One hit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hit {
  pub kind: HitKind,
  /// What to call it: an entity's heading, a symbol's name, a file's path.
  pub name: String,
  /// The entity this belongs to, where one does (`ST0069`, an issue's id).
  ///
  /// **IT IS WHAT A READER NAVIGATES BY**, and the terminal has printed it
  /// beside every hit since search shipped. A file or a symbol has no owner
  /// and says so by absence rather than by an empty string.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub owner: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub lang: Option<String>,
  /// Project-relative.
  pub path: String,
  /// **A SPAN IS A CLAIM ABOUT THE DISK** (issue 0195). It is present only
  /// where the indexed bytes were found in the file as it now stands, so a
  /// reader jumping to it lands on the match. Where they were not, the hit
  /// keeps its path and loses its line -- the file is still the answer, and a
  /// line that was once right would be believed.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub span: Option<Span>,
  /// The tier's own ranking quantity, comparable only WITHIN its group.
  pub score: f64,
  pub snippet: String,
  /// True exactly when the indexed bytes are no longer on the disk where they
  /// were indexed -- the reason a span is absent, stated rather than left for
  /// the reader to infer from the absence.
  ///
  /// **`default` PAIRS WITH THE SKIP, AND ITS ABSENCE MADE THE WHOLE ENVELOPE
  /// WRITE-ONLY** (found by driving `--daemon search`, 2026-09-12). A field
  /// omitted on the way out has to have a value on the way in; serde supplies
  /// one for `Option` without being asked and for nothing else, so a `bool`
  /// that skips `false` cannot be read back at all. Every hit in a normal
  /// answer is fresh, so every normal answer was unreadable -- and nothing
  /// noticed for as long as the envelope only ever went outwards.
  #[serde(default, skip_serializing_if = "std::ops::Not::not")]
  pub stale: bool,
  /// What a symbol hit knows beyond its name; absent on every other hit.
  #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
  pub symbol: Option<SymbolFacts>,
}

/// A symbol hit's facts, **SPELLED AS THE `symbols` COLUMNS THEY ARE READ FROM**
/// (vc, 2026-09-17, ST0076 WP-04): one fact has one name across the SQL door,
/// this envelope and the MCP tool, so a caller who learned `trait_name` in one
/// place reads it in the others.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymbolFacts {
  /// What the symbol is, in its language's own words: `struct`, `method`,
  /// `defp`, `call`.
  pub subkind: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub container: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub container_kind: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub trait_name: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub arity: Option<u32>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub arity_min: Option<u32>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub qualifier: Option<String>,
  /// How much the row knows: 1 a definition or an unqualified reference read
  /// by syntax, 2 a reference with the qualifier it was written with, 3 a
  /// reference resolved by the language's own toolchain.
  pub level: u8,
  /// The one definition a current resolved row joins this reference to, flat
  /// on the hit as `target`, `target_path` and `target_line`, at level 3
  /// (ST0076 WP-07, AC-07.1).
  #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
  pub resolved: Option<ResolvedTarget>,
  /// The definitions current resolved rows join this reference to when there
  /// are several, in order. The hit keeps its syntax level, because the store
  /// keeps no tie-break and neither does an answer.
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub candidates: Vec<ResolvedTarget>,
}

impl SymbolFacts {
  /// Whether a current resolved row joins this reference to `target`, as the
  /// one it resolves to or as one of its candidates.
  pub fn names_target(&self, target: &str) -> bool {
    self
      .resolved
      .as_ref()
      .is_some_and(|one| one.target == target)
      || self
        .candidates
        .iter()
        .any(|candidate| candidate.target == target)
  }

  /// Where this reference points, in the words the terminal's row and the
  /// explorer's row both print, or `None` when level 3 says nothing about it.
  pub fn points_to(&self) -> Option<String> {
    if let Some(one) = &self.resolved {
      return Some(match (&one.target_path, one.target_line) {
        (Some(path), Some(line)) => format!("-> {}  {path}:{line}", one.target),
        (Some(path), None) => format!("-> {}  {path}", one.target),
        _ => format!("-> {}", one.target),
      });
    }
    if self.candidates.is_empty() {
      return None;
    }
    Some(format!(
      "one of {}: {}",
      self.candidates.len(),
      self
        .candidates
        .iter()
        .map(|candidate| candidate.target.as_str())
        .collect::<Vec<_>>()
        .join(", ")
    ))
  }
}

/// A definition a resolved reference names, spelled as the `resolved` columns
/// it is read from (ST0076 WP-07).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ResolvedTarget {
  /// The definition's printable name, as the language's reader prints it.
  pub target: String,
  /// Where it is defined, when the run could place it.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub target_path: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub target_line: Option<u32>,
}

/// Give a reference hit what level 3 knows about it (ST0076 WP-07, AC-07.1).
///
/// `line` is the `symbols` row's start line, which is the resolved key's. It is
/// passed rather than read from the hit because a hit whose file moved on disk
/// carries no span. `rows` are resolved rows of the hit's file, and `stale` the
/// files whose rows no longer describe the bytes the index holds.
///
/// **ONE ROW IS LEVEL 3, SEVERAL ARE CANDIDATES AT THE SYNTAX LEVEL.** A hit in
/// a stale file, or one whose file moved on disk since it was indexed, is left
/// as it was: its rows describe bytes that are not the ones it came from.
pub fn resolve_hit(hit: &mut Hit, line: u32, rows: &[Row], stale: &BTreeSet<String>) {
  if hit.kind != HitKind::Ref || hit.stale || stale.contains(&hit.path) {
    return;
  }
  let mut joined: Vec<ResolvedTarget> = rows
    .iter()
    .filter(|row| row.path == hit.path && row.line == line && row.name == hit.name)
    .map(|row| ResolvedTarget {
      target: row.target.clone(),
      target_path: row.target_path.clone(),
      target_line: row.target_line,
    })
    .collect();
  joined.sort();
  joined.dedup();
  let Some(facts) = hit.symbol.as_mut() else {
    return;
  };
  if joined.len() == 1 {
    facts.level = 3;
    facts.resolved = joined.pop();
  } else {
    facts.candidates = joined;
  }
}

/// Whether a resolved row joins the reference at `path`, `line` and `name` to
/// `target`, whatever its file's staleness. A search by target reads it to
/// name a moved file it takes no hit from.
pub fn joins_target(rows: &[Row], path: &str, line: u32, name: &str, target: &str) -> bool {
  rows
    .iter()
    .any(|row| row.path == path && row.line == line && row.name == name && row.target == target)
}

/// The resolved targets a target no row names most likely meant (vc's
/// refinement of AC-07.2), in order: each that ENDS with the typed text at a
/// segment boundary, the start of the target or a `.`, `:` or `/` before it.
///
/// **THREE SPELLINGS OF EACH TARGET ARE COMPARED**: as it prints, without a
/// trailing arity (Elixir's `Map.get/2`), and without a trailing `()` (a
/// callable Rust target, `crate::store::Store::open()`), so `get`,
/// `Store::open` and `Repo.get/2` each reach what they name.
pub fn near_targets<'a>(typed: &str, targets: impl IntoIterator<Item = &'a str>) -> Vec<String> {
  if typed.is_empty() {
    return Vec::new();
  }
  let mut near: Vec<String> = targets
    .into_iter()
    .filter(|target| {
      let without_arity = target
        .rsplit_once('/')
        .filter(|(_, arity)| !arity.is_empty() && arity.bytes().all(|b| b.is_ascii_digit()))
        .map(|(head, _)| head);
      [Some(*target), without_arity, target.strip_suffix("()")]
        .into_iter()
        .flatten()
        .any(|spelling| {
          spelling
            .strip_suffix(typed)
            .is_some_and(|head| head.is_empty() || head.ends_with(['.', ':', '/']))
        })
    })
    .map(str::to_string)
    .collect();
  near.sort();
  near.dedup();
  near
}

/// A line range in a file, 1-indexed and inclusive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
  pub start_line: u32,
  pub end_line: u32,
}

impl Span {
  pub fn line(line: u32) -> Self {
    Self {
      start_line: line,
      end_line: line,
    }
  }
}

/// What a caller asked for, apart from the words.
///
/// **THE FILTERS THIS CARRIES ARE THE ONES A CORPUS EXISTS TO HONOUR.**
/// `--tier`, `--lang` and `--no-reconcile` are in the design's usage block and
/// are deliberately NOT here: the tiers they select do not exist until WP-20,
/// the languages until WP-18's source corpus, and the reconcile until WP-18's
/// walk. A flag accepted and then ignored is the silent-subset defect wearing
/// a flag's clothes, so each lands with the thing it filters.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchQuery {
  /// Empty means every kind.
  pub kinds: Vec<HitKind>,
  /// Which tiers to ask. Empty means every tier this build answers.
  ///
  /// **A TIER FILTER SELECTS WHICH TIERS ARE ASKED, IT DOES NOT EMPTY THEM.**
  /// An unasked tier is ABSENT from the answer rather than present with no
  /// hits, because an empty group says *this tier ran and found nothing* -- a
  /// claim nobody made. It is the same rule `--outline` follows by carrying one
  /// group and no lexical one.
  pub tiers: Vec<Tier>,
  /// Empty means every language. A hit with no language -- prose, canon -- is
  /// excluded by ANY language filter, because it is not of that language.
  pub langs: Vec<String>,
  /// A glob over the project-relative path.
  pub path: Option<String>,
  /// The row cap. `None` is the caller's "all of it".
  pub limit: Option<usize>,
  /// Empty means every subkind. A hit that is not a symbol has none, so any
  /// subkind filter leaves it out.
  #[serde(default)]
  pub subkinds: Vec<String>,
  /// The container a symbol sits in, matched by [`container_matches`]. A hit
  /// with no container is left out by any container filter.
  #[serde(default)]
  pub container: Option<String>,
  /// The definition a reference must resolve to, matched exactly against a
  /// resolved row's `target` (ST0076 WP-07, AC-07.2). A definition, a hit
  /// that is not a symbol, and a reference no current row joins to it are
  /// left out.
  #[serde(default)]
  pub target: Option<String>,
}

impl SearchQuery {
  /// Whether these filters are a question on their own: a search with a
  /// subkind, a container or a target and nothing else to ask lists the
  /// symbols that pass them (ST0076 WP-04, AC-04.1; WP-07, AC-07.2).
  ///
  /// **THE RULE FOR EVERY FACE, HERE ONCE.** The terminal and the MCP tool both
  /// ask it, so neither can list on a filter the other refuses as nothing to
  /// search for. Kind, language and path are left out on purpose: each narrows
  /// every tier, so alone they would list the whole index.
  pub fn lists_symbols(&self) -> bool {
    !self.subkinds.is_empty() || self.container.is_some() || self.target.is_some()
  }

  /// Whether only the structural tier can answer what these filters keep: the
  /// lexical tier was not asked, the filters list symbols, or every kind asked
  /// is a symbol's (issue 0548). **A TEXT QUESTION WHOSE LEXICAL TIER RAN HAS
  /// READ EVERY FILE'S TEXT**, a script no grammar parses included, so a
  /// language the structural tier cannot see leaves it whole. Marking it
  /// otherwise would send every text query in a project that declares shell to
  /// grep, which is 0430's meaningless flag again.
  pub fn needs_symbols(&self) -> bool {
    Tier::Structural.asked(&self.tiers)
      && (!Tier::Lexical.asked(&self.tiers)
        || self.lists_symbols()
        || (!self.kinds.is_empty()
          && self
            .kinds
            .iter()
            .all(|kind| matches!(kind, HitKind::Def | HitKind::Ref))))
  }

  pub fn keeps(&self, hit: &Hit) -> bool {
    if !self.kinds.is_empty() && !self.kinds.contains(&hit.kind) {
      return false;
    }
    if !self.subkinds.is_empty()
      && !hit
        .symbol
        .as_ref()
        .is_some_and(|facts| self.subkinds.contains(&facts.subkind))
    {
      return false;
    }
    if let Some(asked) = &self.container
      && !hit
        .symbol
        .as_ref()
        .and_then(|facts| facts.container.as_deref())
        .is_some_and(|container| container_matches(asked, container))
    {
      return false;
    }
    if let Some(target) = &self.target
      && !hit
        .symbol
        .as_ref()
        .is_some_and(|facts| facts.names_target(target))
    {
      return false;
    }
    if !self.langs.is_empty()
      && !hit
        .lang
        .as_deref()
        .is_some_and(|lang| self.langs.iter().any(|asked| asked == lang))
    {
      return false;
    }
    match &self.path {
      Some(glob) => glob_matches(glob, &hit.path),
      None => true,
    }
  }
}

/// Whether a symbol's `container` is the one a caller named: the whole of it,
/// or its last segment after `::` or `.`, so `AddressError` finds a method whose
/// container is `errors::AddressError`.
pub fn container_matches(asked: &str, container: &str) -> bool {
  container == asked
    || container
      .strip_suffix(asked)
      .is_some_and(|head| head.ends_with("::") || head.ends_with('.'))
}

/// An answer's `matched` and `returned`, with `limit` applied to each group.
///
/// **BOTH DENOMINATORS, AND `matched` IS COUNTED AFTER THE FILTERS.** The
/// filters are part of the question -- `--kind issue` asks how many ISSUES
/// matched -- while the limit is a cap on the answer. Counting before them
/// would report a denominator for a question nobody asked.
///
/// Issue 0357: both are taken from the groups the answer carries, so
/// `returned` is the length of the body. The cap applies per group because
/// tiers are ranked within themselves and never blended.
///
/// **ONE HOME FOR EVERY DOOR** (ST0076 WP-04): the structural doors counted
/// their own hits and never applied the cap, so `--outline <path> --limit 1`
/// returned the whole file.
pub fn cap(groups: &mut [TierGroup], limit: Option<usize>) -> (usize, usize) {
  let mut matched = 0;
  let mut returned = 0;
  for group in groups {
    matched += group.hits.len();
    if let Some(limit) = limit {
      group.hits.truncate(limit);
    }
    returned += group.hits.len();
  }
  (matched, returned)
}

/// The filter words a caller gave, before they are checked.
///
/// **THE WORDS ARE CHECKED HERE, ONCE, FOR EVERY FACE** (ST0076 WP-04). The
/// terminal and the MCP tool each parsed `kind` and `tier` with their own copy
/// of the refusal, and a subkind refusal needs the roster of the languages in
/// scope, which is this crate's to know.
#[derive(Debug, Clone, Default)]
pub struct FilterWords {
  pub kinds: Vec<String>,
  pub tiers: Vec<String>,
  pub subkinds: Vec<String>,
  pub langs: Vec<String>,
  pub target: Option<String>,
}

/// A filter word that names nothing this search has.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilterRefusal {
  /// What was wrong, naming the word.
  pub problem: String,
  /// The words that would have been accepted.
  pub choices: Vec<String>,
}

impl FilterWords {
  /// The kinds, tiers and subkinds as a query, or the first word that names
  /// nothing. Path, limit and container are the faces' to set.
  ///
  /// **AN UNKNOWN SUBKIND IS REFUSED AGAINST THE LANGUAGES IN SCOPE**: the
  /// `lang` filter's languages when it has any, otherwise every language this
  /// build parses. A subkind filter that silently kept no rows would answer a
  /// typo with a confident empty result.
  pub fn check(self) -> Result<SearchQuery, FilterRefusal> {
    let mut ask = SearchQuery {
      langs: self.langs,
      ..SearchQuery::default()
    };
    for word in self.kinds {
      match HitKind::parse(&word) {
        Some(kind) => ask.kinds.push(kind),
        None => {
          return Err(FilterRefusal {
            problem: format!("`{word}` is not a kind of thing this index holds"),
            choices: HitKind::ALL.iter().map(|k| k.to_string()).collect(),
          });
        }
      }
    }
    for word in self.tiers {
      match Tier::parse(&word) {
        Some(tier) => ask.tiers.push(tier),
        None => {
          return Err(FilterRefusal {
            problem: format!("`{word}` is not a tier this search has"),
            choices: Tier::ALL.iter().map(|t| t.to_string()).collect(),
          });
        }
      }
    }
    if !self.subkinds.is_empty() {
      let roster = subkind_roster(&ask.langs);
      for word in &self.subkinds {
        if !roster.contains(word) {
          return Err(FilterRefusal {
            problem: format!(
              "`{word}` is not a subkind the index writes for {}",
              if ask.langs.is_empty() {
                "any language this build parses".to_string()
              } else {
                ask.langs.join(", ")
              }
            ),
            choices: roster,
          });
        }
      }
      ask.subkinds = self.subkinds;
    }
    // **A TARGET ASKS FOR REFERENCES** (ST0076 WP-07): nothing but a reference
    // resolves to a definition, so a kind filter that leaves references out
    // could only answer empty, and is refused by name.
    if self.target.is_some() && !ask.kinds.is_empty() && !ask.kinds.contains(&HitKind::Ref) {
      return Err(FilterRefusal {
        problem: format!(
          "a target asks for the references resolved to it, and the kind filter ({}) leaves every reference out",
          ask
            .kinds
            .iter()
            .map(|kind| kind.as_str())
            .collect::<Vec<_>>()
            .join(", ")
        ),
        choices: vec![HitKind::Ref.as_str().to_string()],
      });
    }
    ask.target = self.target;
    Ok(ask)
  }
}

/// Every subkind the given languages' queries write, or every parsed
/// language's when none is given, in roster order with none repeated.
pub fn subkind_roster(langs: &[String]) -> Vec<String> {
  let mut out: Vec<String> = Vec::new();
  let every: Vec<&str> = crate::index::symbols::LANGUAGES
    .iter()
    .map(|(lang, _)| *lang)
    .collect();
  let asked: Vec<&str> = if langs.is_empty() {
    every
  } else {
    langs.iter().map(String::as_str).collect()
  };
  for lang in asked {
    for word in crate::index::symbols::subkinds(lang) {
      if !out.contains(&word) {
        out.push(word);
      }
    }
  }
  out
}

/// A path glob: `*` matches within a segment, `**` across segments, `?` one
/// character. Pure, and the only pattern language the search surfaces speak.
pub fn glob_matches(pattern: &str, path: &str) -> bool {
  matches_from(pattern.as_bytes(), path.as_bytes(), pattern.contains("**"))
}

fn matches_from(pattern: &[u8], path: &[u8], double: bool) -> bool {
  if pattern.is_empty() {
    return path.is_empty();
  }
  match pattern[0] {
    b'*' => {
      // `**` crosses `/`; a single `*` stops at one.
      let (rest, crosses) = if double && pattern.starts_with(b"**") {
        let mut rest = &pattern[2..];
        if rest.first() == Some(&b'/') {
          // `a/**/b` must also match `a/b`.
          if matches_from(&rest[1..], path, double) {
            return true;
          }
          rest = &rest[1..];
        }
        (rest, true)
      } else {
        (&pattern[1..], false)
      };
      for split in 0..=path.len() {
        if !crosses && path[..split].contains(&b'/') {
          break;
        }
        if matches_from(rest, &path[split..], double) {
          return true;
        }
      }
      false
    }
    b'?' if !path.is_empty() => matches_from(&pattern[1..], &path[1..], double),
    c if !path.is_empty() && path[0] == c => matches_from(&pattern[1..], &path[1..], double),
    _ => false,
  }
}

/// Where an indexed body was found in the file that now stands on disk.
///
/// **`NoClaim` AND `Moved` ARE DIFFERENT ANSWERS AND ARE KEPT APART.** A hit
/// with no line is not a stale hit, and three separate things produce one: a
/// file that canon has not realised, a section whose body was never a byte
/// range of any file, and a file whose bytes have genuinely moved on. Only the
/// third is staleness.
///
/// **THE SECOND CASE IS THE ONE THAT ALMOST SHIPPED AS A BUG.** A canon
/// section's body comes from a FIELD in `thread.json`, where it sits JSON
/// escaped, so searching the file for those bytes fails for a perfectly fresh
/// project -- and calling that `Moved` would have reported every canon hit as
/// stale, turned `complete` false everywhere, and printed a warning per hit.
/// The discriminator is whether the file's own bytes were what was indexed,
/// which is a property of WHERE the section came from and not of what the
/// comparison returns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Located {
  /// The 1-indexed line in the file where the match sits.
  At(u32),
  /// The indexed bytes are not in that file any more.
  Moved,
  /// There is nothing on disk to make a claim about.
  NoClaim,
}

/// The first position of `needle` in `haystack`, byte for byte.
pub fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
  if needle.is_empty() || needle.len() > haystack.len() {
    return None;
  }
  haystack
    .windows(needle.len())
    .position(|window| window == needle)
}

/// The one line of `body` the match sits on, trimmed and capped.
///
/// **A SNIPPET IS AN EXCERPT AND NEVER A SUMMARY**: it is the operator's own
/// bytes, cut at a character boundary, so nothing in the answer is text the
/// corpus does not contain.
pub fn snippet(body: &str, at: Option<usize>) -> String {
  let at = at.unwrap_or(0).min(body.len());
  let start = body[..at].rfind('\n').map_or(0, |nl| nl + 1);
  let end = body[at..].find('\n').map_or(body.len(), |nl| at + nl);
  let line = body[start..end].trim();
  let line = if line.is_empty() {
    body
      .lines()
      .find(|l| !l.trim().is_empty())
      .unwrap_or("")
      .trim()
  } else {
    line
  };
  match line.char_indices().nth(SNIPPET_CHARS) {
    Some((cut, _)) => format!("{}...", &line[..cut]),
    None => line.to_string(),
  }
}

/// How much of a matched line a snippet carries. A bound, not a measurement:
/// it says what this surface does, and no reader is told a number about the
/// corpus.
const SNIPPET_CHARS: usize = 160;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn complete_is_derived_from_the_lists_beside_it() {
    let mut freshness = IndexFreshness::new(BTreeMap::from([(
      corpus_key(&Corpus::Canon).to_string(),
      CorpusState {
        policy: CANON_POLICY.to_string(),
        files: 3,
      },
    )]));
    assert!(freshness.complete());
    freshness.mark_stale("intent/wip.md");
    assert!(!freshness.complete(), "a stale path means incomplete");
    assert_eq!(freshness.stale, vec!["intent/wip.md".to_string()]);
  }

  /// Issue 0548: a language a structural answer could not see makes it
  /// incomplete, the list crosses the wire, and an envelope from a daemon built
  /// before the list existed still reads.
  #[test]
  fn a_language_the_answer_could_not_see_makes_it_incomplete() {
    let mut freshness = IndexFreshness::new(BTreeMap::new());
    freshness.mark_unindexed(Unindexed {
      lang: Some("shell".to_string()),
      reason: "no-grammar".to_string(),
    });
    assert!(!freshness.complete());
    let wire = serde_json::to_string(&freshness).expect("serialise");
    let back: IndexFreshness = serde_json::from_str(&wire).expect("deserialise");
    assert_eq!(back.unindexed, freshness.unindexed);
    assert!(!back.complete());
    let older = r#"{"complete":true,"reconciled":false,"corpora":{},"skipped":[],"stale":[],"resolution":{}}"#;
    let read: IndexFreshness = serde_json::from_str(older).expect("an envelope from before 0548");
    assert!(read.unindexed.is_empty() && read.complete());
  }

  /// Issue 0548's rule as vc ruled it: a language counts when it is declared or
  /// held in the question's path, never when it is a discipline, and never
  /// when `--lang` leaves it out; an outline is judged by its own file.
  #[cfg(feature = "lang-rust")]
  #[test]
  fn the_languages_a_structural_answer_cannot_see_follow_the_ruling() {
    let declared = ["rust", "shell", "author", "content"]
      .map(String::from)
      .to_vec();
    let every = SearchQuery::default();
    let langs =
      |found: Vec<Unindexed>| -> Vec<String> { found.into_iter().filter_map(|u| u.lang).collect() };
    assert_eq!(
      langs(unindexed_for(&declared, [], &every)),
      ["shell"],
      "declared shell counts with no file of it held, because its scripts may \
       carry no extension; author and content never count"
    );
    let held = [("tools/gen.lua", "lua"), ("src/lib.rs", "rust")];
    assert_eq!(
      langs(unindexed_for(&declared, held, &every)),
      ["lua", "shell"],
      "a lua file the project does not declare is parsed by nothing"
    );
    let under_src = SearchQuery {
      path: Some("src/**".to_string()),
      ..SearchQuery::default()
    };
    assert_eq!(
      langs(unindexed_for(&declared, held, &under_src)),
      ["shell"],
      "a held file outside the path is out of the question; the declared half \
       has no path to judge"
    );
    let rust_only = SearchQuery {
      langs: vec!["rust".to_string()],
      ..SearchQuery::default()
    };
    assert!(
      unindexed_for(&declared, held, &rust_only).is_empty(),
      "`--lang rust` leaves both halves out"
    );

    assert_eq!(
      unindexed_outline("lib/install", &declared, &every).map(|u| u.reason),
      Some(Unindexed::NO_LANGUAGE.to_string())
    );
    assert_eq!(
      unindexed_outline("lib/install", &declared, &rust_only),
      None,
      "a file with no language is left out by any `--lang`"
    );
    assert_eq!(
      unindexed_outline("hooks/pre.sh", &declared, &every).and_then(|u| u.lang),
      Some("shell".to_string())
    );
    assert_eq!(unindexed_outline("src/lib.rs", &declared, &every), None);
  }

  /// Issue 0430: a skip by policy is listed and leaves the answer whole; a
  /// skip that hid text, or a reason this build does not know, does not.
  #[test]
  fn only_a_skip_that_hid_text_makes_the_answer_incomplete() {
    let mut freshness = IndexFreshness::new(BTreeMap::new());
    freshness.mark_skipped("assets/logo.png", SkipReason::Binary.as_str());
    freshness.mark_skipped("bin/int", SkipReason::Symlink.as_str());
    assert!(
      freshness.complete(),
      "a binary and a symlink hold no text a query could match: {:?}",
      freshness.skipped
    );
    assert_eq!(freshness.skipped.len(), 2, "both are still listed");

    for reason in [
      SkipReason::TooLarge.as_str(),
      SkipReason::Unreadable.as_str(),
      "a-later-reason",
    ] {
      let mut partial = freshness.clone();
      partial.mark_skipped("vendor/huge.json", reason);
      assert!(!partial.complete(), "`{reason}` leaves a gap");
      assert_eq!(
        partial
          .gaps()
          .map(|skip| skip.path.as_str())
          .collect::<Vec<_>>(),
        vec!["vendor/huge.json"]
      );
    }
  }

  /// ST0076 WP-04: a symbol's facts travel flat, under the column names, and
  /// survive the daemon's wire both ways; a hit that is not a symbol carries
  /// none of them.
  #[test]
  fn a_symbol_hit_carries_its_columns_flat_and_reads_back() {
    let symbol = Hit {
      kind: HitKind::Def,
      name: "new".to_string(),
      owner: None,
      lang: Some("rust".to_string()),
      path: "src/lib.rs".to_string(),
      span: Some(Span::line(3)),
      score: 0.0,
      snippet: "pub fn new() -> Self".to_string(),
      stale: false,
      symbol: Some(SymbolFacts {
        subkind: "assoc_fn".to_string(),
        container: Some("AddressError".to_string()),
        container_kind: Some("impl".to_string()),
        trait_name: None,
        arity: Some(0),
        arity_min: Some(0),
        qualifier: None,
        level: 1,
        resolved: None,
        candidates: Vec::new(),
      }),
    };
    let json = serde_json::to_value(&symbol).expect("serialise");
    assert_eq!(json["subkind"], "assoc_fn");
    assert_eq!(json["container"], "AddressError");
    assert_eq!(json["level"], 1);
    assert!(json.get("symbol").is_none() && json.get("trait_name").is_none());
    let back: Hit = serde_json::from_value(json).expect("deserialise");
    assert_eq!(back.symbol, symbol.symbol);

    let file = Hit {
      kind: HitKind::File,
      symbol: None,
      ..symbol
    };
    let json = serde_json::to_value(&file).expect("serialise");
    assert!(json.get("subkind").is_none() && json.get("level").is_none());
    let back: Hit = serde_json::from_value(json).expect("deserialise");
    assert_eq!(
      back.symbol, None,
      "a file hit reads back with no symbol facts"
    );
  }

  fn reference(path: &str, name: &str) -> Hit {
    Hit {
      kind: HitKind::Ref,
      name: name.to_string(),
      owner: None,
      lang: Some("rust".to_string()),
      path: path.to_string(),
      span: Some(Span::line(2)),
      score: 0.0,
      snippet: String::new(),
      stale: false,
      symbol: Some(SymbolFacts {
        subkind: "call".to_string(),
        container: None,
        container_kind: None,
        trait_name: None,
        arity: None,
        arity_min: None,
        qualifier: None,
        level: 1,
        resolved: None,
        candidates: Vec::new(),
      }),
    }
  }

  fn row(path: &str, line: u32, name: &str, target: &str) -> Row {
    Row {
      path: path.to_string(),
      line,
      name: name.to_string(),
      target: target.to_string(),
      target_path: Some("src/lib.rs".to_string()),
      target_line: Some(9),
    }
  }

  /// ST0076 WP-07: a key naming one target is level 3 with the target flat on
  /// the hit and read back from the wire; a key naming two keeps its level and
  /// lists both; a stale file's rows are not read.
  #[test]
  fn a_reference_takes_one_target_flat_and_several_as_candidates() {
    let rows = [
      row("src/lib.rs", 2, "helper", "crate::helper()"),
      row("src/lib.rs", 2, "other", "crate::b::other()"),
      row("src/lib.rs", 2, "other", "crate::a::other()"),
    ];
    let mut one = reference("src/lib.rs", "helper");
    resolve_hit(&mut one, 2, &rows, &BTreeSet::new());
    let json = serde_json::to_value(&one).expect("serialise");
    assert_eq!(
      (
        &json["level"],
        &json["target"],
        &json["target_path"],
        &json["target_line"]
      ),
      (
        &serde_json::json!(3),
        &serde_json::json!("crate::helper()"),
        &serde_json::json!("src/lib.rs"),
        &serde_json::json!(9)
      ),
      "{json}"
    );
    assert!(json.get("resolved").is_none() && json.get("candidates").is_none());
    let back: Hit = serde_json::from_value(json).expect("deserialise");
    assert_eq!(back.symbol, one.symbol, "a level-3 hit survives the wire");

    let mut two = reference("src/lib.rs", "other");
    resolve_hit(&mut two, 2, &rows, &BTreeSet::new());
    let facts = two.symbol.as_ref().expect("facts");
    assert_eq!(facts.level, 1, "several targets keep the syntax level");
    assert_eq!(
      facts.points_to().as_deref(),
      Some("one of 2: crate::a::other(), crate::b::other()")
    );
    assert!(facts.names_target("crate::b::other()"));
    let json = serde_json::to_value(&two).expect("serialise");
    assert!(json.get("target").is_none(), "{json}");
    let back: Hit = serde_json::from_value(json).expect("deserialise");
    assert_eq!(back.symbol, two.symbol, "candidates survive the wire");

    let mut stale = reference("src/lib.rs", "helper");
    resolve_hit(
      &mut stale,
      2,
      &rows,
      &BTreeSet::from(["src/lib.rs".to_string()]),
    );
    assert_eq!(stale.symbol, reference("src/lib.rs", "helper").symbol);
  }

  /// vc's refinement of AC-07.2, with dc's Elixir spellings and vc's ruled Rust
  /// ones: a near target ends with the typed text at a segment boundary, with
  /// or without its arity or its `()`.
  #[test]
  fn a_near_target_ends_with_what_was_typed_at_a_boundary() {
    let targets = [
      "Map.get/2",
      "Shop.Repo.get/2",
      ":ets.lookup/2",
      "Kernel.to_string/1",
      "intentsvcs::store::Store::open()",
      "intentsvcs::store::Store",
      "intentsvcs::address::<AddressError as Remedy>::remedy()",
      "crate::forget()",
    ];
    let near = |typed: &str| near_targets(typed, targets.iter().copied());
    assert_eq!(near("get"), vec!["Map.get/2", "Shop.Repo.get/2"]);
    assert_eq!(near("Repo.get/2"), vec!["Shop.Repo.get/2"]);
    assert_eq!(near("lookup"), vec![":ets.lookup/2"]);
    assert_eq!(near("ets.lookup/2"), vec![":ets.lookup/2"]);
    assert_eq!(
      near("Store::open"),
      vec!["intentsvcs::store::Store::open()"]
    );
    assert_eq!(near("Store"), vec!["intentsvcs::store::Store"]);
    assert_eq!(
      near("remedy"),
      vec!["intentsvcs::address::<AddressError as Remedy>::remedy()"]
    );
    assert!(
      near("tore").is_empty(),
      "inside a segment is not a boundary"
    );
    assert!(near("").is_empty());
  }

  /// ST0076 WP-07: a run's record projects to the state an answer names, and
  /// a declared language this build resolves with no record is unresolved.
  #[test]
  fn a_language_whose_level_three_is_not_current_is_named_with_its_state() {
    let run = |state: &str, stale: &[&str]| Run {
      state: state.to_string(),
      tool: "rust-analyzer".to_string(),
      path: None,
      line: None,
      detail: None,
      resolved_at: Some("2026-09-17T00:00:00Z".to_string()),
      run: 1,
      files: 0,
      joined: 0,
      symbols_version: Some(3),
      tally: resolved::Tally::default(),
      stale: stale.iter().map(|s| s.to_string()).collect(),
    };
    assert_eq!(ResolutionState::of(&run(resolved::CURRENT, &[])), None);
    let stale = ResolutionState::of(&run(resolved::CURRENT, &["src/a.rs"])).expect("stale");
    assert_eq!(
      (stale.state.as_str(), stale.stale.clone()),
      ("stale", vec!["src/a.rs".to_string()])
    );
    assert_eq!(
      resolution_words("rust", &stale),
      format!(
        "level 3 in `rust` (stale): {} -- stale: src/a.rs",
        resolution_phrase(resolved::STALE)
      )
    );

    let runs = BTreeMap::from([
      ("rust".to_string(), run(resolved::CURRENT, &[])),
      ("lua".to_string(), run(resolved::FAILED, &[])),
      ("swift".to_string(), run(resolved::FAILED, &[])),
    ]);
    let carried = |lang: &'static str, tool: &'static str| Carried {
      lang,
      tool,
      manifest: resolved::Manifest {
        name: "manifest",
        root_only: true,
      },
    };
    let carried = [
      carried("rust", "rust-analyzer"),
      carried("elixir", "mix"),
      carried("swift", "sourcekit"),
    ];
    let asked = manifest_questions(&runs, &["rust".to_string(), "elixir".to_string()], &carried);
    assert_eq!(
      asked.iter().map(|reader| reader.lang).collect::<Vec<_>>(),
      vec!["elixir", "swift"],
      "a declared language no run recorded, and a carried one whose run failed, declared or not; \
       never a current run, and never a language this build does not carry"
    );
    let named = |applies: &[&Carried]| {
      resolution_states(&runs, &carried, applies)
        .into_iter()
        .map(|(lang, state)| (lang, state.state))
        .collect::<Vec<_>>()
    };
    let pair = |lang: &str, state: &str| (lang.to_string(), state.to_string());
    assert_eq!(
      named(&asked),
      vec![
        pair("elixir", "unresolved"),
        pair("lua", "failed"),
        pair("swift", "failed")
      ],
      "with each manifest held: current is not named, and the rest are"
    );
    assert_eq!(
      named(&[]),
      vec![pair("lua", "failed")],
      "with no manifest held, neither the unrecorded language nor the carried failed run is named; \
       a run of a language this build does not carry is named as recorded"
    );
    let states = resolution_states(&runs, &carried, &asked);
    assert_eq!(
      resolution_words("elixir", &states["elixir"]),
      format!(
        "level 3 in `elixir` (unresolved): {} -- a person resolves it by running `intent index resolve`, which runs the project's build through mix",
        resolution_phrase(resolved::UNRESOLVED)
      )
    );
  }

  #[test]
  fn a_container_matches_whole_or_by_its_last_segment() {
    assert!(container_matches("AddressError", "AddressError"));
    assert!(container_matches("AddressError", "errors::AddressError"));
    assert!(container_matches("Store", "Intent.Store"));
    assert!(!container_matches("Error", "errors::AddressError"));
    assert!(!container_matches("errors", "errors::AddressError"));
  }

  #[test]
  fn an_empty_corpus_is_reported_as_empty() {
    let freshness = IndexFreshness::new(BTreeMap::from([(
      corpus_key(&Corpus::Canon).to_string(),
      CorpusState {
        policy: CANON_POLICY.to_string(),
        files: 0,
      },
    )]));
    assert!(
      freshness.is_empty(),
      "nothing indexed is not a genuine miss"
    );
  }

  #[test]
  fn a_glob_stops_at_a_segment_unless_it_is_doubled() {
    assert!(glob_matches("*.rs", "lib.rs"));
    assert!(
      !glob_matches("*.rs", "src/lib.rs"),
      "one star holds a segment"
    );
    assert!(glob_matches("**/*.rs", "native/rust/src/lib.rs"));
    assert!(glob_matches("intent/**", "intent/st/ST0069/info.md"));
    assert!(
      glob_matches("intent/**/info.md", "intent/info.md"),
      "**/ also matches nothing"
    );
    assert!(!glob_matches("docs/**", "intent/docs/x.md"));
  }

  #[test]
  fn a_snippet_is_the_matched_line_and_nothing_else() {
    let body = "first line\nthe match is here\nthird line\n";
    let at = body.find("match").expect("fixture");
    assert_eq!(snippet(body, Some(at)), "the match is here");
  }

  #[test]
  fn a_snippet_falls_back_to_the_first_line_with_text() {
    assert_eq!(snippet("\n\n  body text\n", None), "body text");
  }

  #[test]
  fn an_owner_this_enum_has_not_met_is_a_file_and_not_a_drop() {
    assert_eq!(HitKind::of_owner("thread"), HitKind::Thread);
    assert_eq!(HitKind::of_owner("whiteboard_note"), HitKind::File);
  }
}
