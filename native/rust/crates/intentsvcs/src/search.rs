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

use crate::index::corpus::Corpus;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

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
  /// What the index matched across the groups carried, before any cap.
  pub matched: usize,
  /// The rows the answer carries: the length of every group's hits.
  pub returned: usize,
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
  /// Keyed by the corpus names the index itself uses (`canon`, and cc's
  /// `prose` and `code` when WP-18 lands). The keys are not enumerated here:
  /// a corpus the index gains appears in the answer the day it is added.
  pub corpora: BTreeMap<String, CorpusState>,
  /// What was in scope and deliberately not indexed, each with its reason.
  /// **A named exclusion, never a silent absence** (AC-13.2's rule, applied to
  /// the answer rather than to the corpus).
  pub skipped: Vec<Skipped>,
  /// Paths whose indexed bytes no longer match the disk.
  pub stale: Vec<String>,
}

/// **`complete` IS COMPUTED AT SERIALISATION AND IS NOT A FIELD.** It is
/// `skipped.is_empty() && stale.is_empty()`, and holding it as a field beside
/// the two lists that determine it would be a second home for one fact whose
/// failure mode is the flag saying complete while the lists say otherwise.
/// This is the whole reason `IndexFreshness` writes its own `Serialize` rather
/// than deriving one.
impl Serialize for IndexFreshness {
  fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    use serde::ser::SerializeStruct;
    let fields = 4 + usize::from(self.reconciled_at.is_some());
    let mut out = serializer.serialize_struct("IndexFreshness", fields)?;
    out.serialize_field("complete", &self.complete())?;
    if let Some(at) = &self.reconciled_at {
      out.serialize_field("reconciled_at", at)?;
    }
    out.serialize_field("corpora", &self.corpora)?;
    out.serialize_field("skipped", &self.skipped)?;
    out.serialize_field("stale", &self.stale)?;
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
      corpora: BTreeMap<String, CorpusState>,
      skipped: Vec<Skipped>,
      stale: Vec<String>,
    }
    let carried = Carried::deserialize(deserializer)?;
    Ok(Self {
      reconciled_at: carried.reconciled_at,
      corpora: carried.corpora,
      skipped: carried.skipped,
      stale: carried.stale,
    })
  }
}

impl IndexFreshness {
  /// The freshness of an index holding the given corpora, with nothing skipped
  /// and nothing stale.
  pub fn new(corpora: BTreeMap<String, CorpusState>) -> Self {
    Self {
      reconciled_at: None,
      corpora,
      skipped: Vec::new(),
      stale: Vec::new(),
    }
  }

  /// Whether this index answered the whole question it was asked.
  pub fn complete(&self) -> bool {
    self.skipped.is_empty() && self.stale.is_empty()
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
}

impl SearchQuery {
  pub fn keeps(&self, hit: &Hit) -> bool {
    if !self.kinds.is_empty() && !self.kinds.contains(&hit.kind) {
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
