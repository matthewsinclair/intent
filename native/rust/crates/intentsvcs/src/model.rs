//! The reified Intent model -- the single authored master (design.md D01,
//! data-model.md). Everything else is generated from these types: the JSON
//! Schema face via schemars, the DDL the store applies, and (WP-04) the
//! GraphQL SDL.
//!
//! Strictness (D05): every struct IN THIS MODULE is `deny_unknown_fields` --
//! an unknown field in canonical JSON is refused by name at deserialize time,
//! never silently dropped. schemars mirrors that as
//! `additionalProperties: false`.
//!
//! **The scope of that sentence is load-bearing and it used to read as a
//! blanket** (ic, 2026-08-16). It is a rule about CANON -- data whose every
//! field this binary owns -- and it is deliberately NOT the rule for the
//! registers the repository also carries. `intent_cli::dispatch::Table` is
//! lenient on purpose, because a measurement record carrying prose for humans
//! must be able to gain a paragraph without the binary refusing to load its own
//! command surface. Stated here because the blanket form is an invitation to
//! make that one "consistent" and break it.
//!
//! The CANONICAL (JSON) wire form has one authority: serde's rename rules --
//! [`enum_str`] routes through serde so a hand-maintained name table cannot
//! drift into existence. Other transports derive their own casing from the
//! same variant set (GraphQL's SDL says `NA` where serde says `n-a`): two
//! wire conventions over ONE vocabulary, with the correspondence pinned by
//! the faces drift guard rather than by this module. The law is about where
//! names are AUTHORED (here, once), not about every transport spelling them
//! identically.

use async_graphql::{Enum, SimpleObject};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// The `schema` field value for thread canon files.
pub const THREAD_SCHEMA: &str = "intent/thread@3.0";
/// The `schema` field value for issue canon files.
pub const ISSUE_SCHEMA: &str = "intent/issue@3.0";

/// **THE steel-thread id form: `ST` and four digits, in one place.**
///
/// hv retired `st_prefix` (issue 0040): the prefix is FIXED. Retiring it turned
/// out not to be a change of direction -- `st_prefix` appears in no ST0056
/// spec, so the design had already dropped the knob and the type was behind the
/// design rather than ahead of it.
///
/// **What the retirement is really for is here, not in the deletion.** The
/// field existed and nothing read it, while the id form was spelled out FOUR
/// separate times: `facade.rs` allocated with `format!("ST{:04}")` and
/// recognised with `strip_prefix("ST")`, `legacy.rs` recognised with
/// `starts_with("ST")` AND a hardcoded `len() == 6`. That `6` is `"ST".len() +
/// 4` -- the same fact as the prefix, written a second way, in a place that
/// would not move if the first one did. A config knob nobody read was the
/// harmless half; four hand-written copies of what it configured was the rest.
pub const THREAD_PREFIX: &str = "ST";
/// How many digits follow [`THREAD_PREFIX`]. Zero-padded, fixed width.
pub const THREAD_DIGITS: usize = 4;
/// How many digits an issue id carries. Zero-padded, fixed width, no prefix --
/// `intent/.canon/issues/0001.json` is the on-disk form this describes.
pub const ISSUE_DIGITS: usize = 4;

/// The finest interval [`Thread::completed`] can distinguish, in hours.
///
/// **`completed` is a domain DATE -- `YYYY-MM-DD`, no time component, carried
/// from v2 and never re-stamped -- so the smallest gap between two completions
/// this model can tell apart is a day.** That is a property of the data, and
/// code that compares against `completed` has to reason about it rather than
/// assume its own precision.
///
/// **This exists so the rule that depends on it RETIRES ITSELF.**
/// `TodoConfig::window` refuses a window that is not a whole multiple of this,
/// because a sub-day cutoff truncated to a date means different things at
/// different times of day. The moment `completed` gains a time component this
/// becomes `1`, `n % 1 != 0` is false for every `n`, and the refusal is
/// unreachable -- by construction rather than by someone remembering to go and
/// delete a guard whose reason has expired. A guard that cannot outlive its
/// reason is the only kind that does not become folklore.
pub const COMPLETED_RESOLUTION_HOURS: u32 = 24;

/// The canonical id for the nth steel thread.
pub fn thread_id(seq: u32) -> String {
  format!("{THREAD_PREFIX}{seq:0THREAD_DIGITS$}")
}

/// Whether `name` is a steel-thread id.
///
/// The width is DERIVED rather than asserted, so this and [`thread_id`] cannot
/// disagree about what they both describe.
pub fn is_thread_id(name: &str) -> bool {
  name.len() == THREAD_PREFIX.len() + THREAD_DIGITS
    && name.starts_with(THREAD_PREFIX)
    && name[THREAD_PREFIX.len()..]
      .bytes()
      .all(|b| b.is_ascii_digit())
}

/// Whether `name` is an issue id.
///
/// Width DERIVED from [`ISSUE_DIGITS`] for the same reason [`is_thread_id`]
/// derives its own: a validator and a formatter that assert the width
/// separately are two declarations of one fact, and they agree until they do
/// not.
pub fn is_issue_id(name: &str) -> bool {
  name.len() == ISSUE_DIGITS && name.bytes().all(|b| b.is_ascii_digit())
}

/// The sequence number in an issue id, or `None` if it is not one.
pub fn issue_seq(name: &str) -> Option<u32> {
  is_issue_id(name).then(|| name.parse().ok()).flatten()
}

/// The sequence number in a thread id, or `None` if it is not one.
pub fn thread_seq(name: &str) -> Option<u32> {
  is_thread_id(name)
    .then(|| name[THREAD_PREFIX.len()..].parse().ok())
    .flatten()
}

/// Render any model enum as its canonical wire string via serde -- the one
/// naming authority. Panics only if serialisation itself fails, which for
/// these unit enums cannot happen.
pub fn enum_str<T: Serialize>(value: &T) -> String {
  match serde_json::to_value(value) {
    Ok(serde_json::Value::String(s)) => s,
    other => panic!("enum_str called on a non-string-serialising value: {other:?}"),
  }
}

/// Canonical JSON form (data-model.md): 2-space pretty print, LF, trailing
/// newline, keys in declaration order (serde struct order).
pub fn to_canonical_json<T: Serialize>(value: &T) -> serde_json::Result<String> {
  let mut out = serde_json::to_string_pretty(value)?;
  out.push('\n');
  Ok(out)
}

/// The lowercase hex SHA-256 of some bytes.
///
/// Here rather than in `sync`, which had the only copy, because [`Attachment`]
/// needs the same answer and two hashers is how two subsystems come to disagree
/// about whether a file changed. `sync::scan` calls this one.
pub fn sha256_hex(bytes: &[u8]) -> String {
  use sha2::{Digest, Sha256};
  Sha256::digest(bytes)
    .iter()
    .map(|b| format!("{b:02x}"))
    .collect()
}

// ---------------------------------------------------------------------------
// Steel thread
// ---------------------------------------------------------------------------

/// A steel thread: `st/<ID>/thread.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, SimpleObject)]
#[serde(deny_unknown_fields)]
pub struct Thread {
  /// Always [`THREAD_SCHEMA`]; lets validators pick the schema.
  pub schema: String,
  // The example is `ST0000` -- the STZero retrofit id, present in every Intent
  // project -- because this line is PUBLISHED into thread.schema.json and the
  // SDL, and a reader cannot look up a thread in our repository (D37).
  /// Natural id, eg `ST0000`. Global identity is `(project_id, id)`.
  pub id: String,
  pub title: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub slug: Option<String>,
  pub status: ThreadStatus,
  /// Why the thread is in [`Thread::status`], for the transitions the ratified
  /// machine guards with "reason recorded" (`st hold`, `st cancel`,
  /// `st reopen`, `st reinstate`).
  ///
  /// **It belongs to the CURRENT status and is cleared by any transition that
  /// does not carry one**, which is the whole reason it is a separate field
  /// rather than a note appended to the title. Without the clear, `st hold
  /// --reason "waiting on the fleet"` followed by `st resume` would leave a
  /// live thread explaining why it was paused -- a reason surviving the
  /// condition it described, which is the same shape as a remedy outliving its
  /// model.
  ///
  /// **The HISTORY is the event log, not this field.** Every guarded verb puts
  /// its reason in the envelope, so the sequence of decisions is durable and
  /// queryable; this carries only the latest one, so `intent st show` can
  /// answer "why is this on hold" without a log query. That is a denormalised
  /// read of the log rather than a second source of truth: both are written by
  /// the same call, and only the log is ever read for history.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status_reason: Option<String>,
  /// ISO 8601 date, `YYYY-MM-DD`.
  pub created: String,
  /// ISO 8601 date, `YYYY-MM-DD`. **No time component**, so the finest interval
  /// two completions can be distinguished by is a day -- a fact any consumer
  /// windowing on this field has to reason about, which is why it is stated
  /// here rather than left to be discovered.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub completed: Option<String>,
  /// `exempt` or absent = acceptance enforced.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub acceptance: Option<AcceptanceMode>,
  /// What this thread ships. Modelled rather than authored prose (vc ruling,
  /// 2026-08-14): `objective` already carries tool opinion -- the 0010
  /// empty-objective warning -- which is the signature of a modelled field.
  /// May be empty; the 0010 warning is COMPUTED from emptiness, never stored.
  #[serde(default)]
  pub objective: String,
  /// Why this thread exists. Markdown, carried verbatim, never reflowed.
  #[serde(default)]
  pub context: String,
  // PUBLISHED (D37): the `///` below becomes a field description in
  // thread.schema.json and the SDL, so the design provenance lives here in a
  // plain comment instead. This is the two-field shape D28 gave the work
  // package, applied one level up -- and the guard `no_pm_state_in_output`
  // caught a `D28` citation in the doc comment on the first build, which is
  // the rule about `///` being shipped output doing its job on the person who
  // wrote that rule down.
  //
  // The drop set is measured against `lib/templates/prj/st/ST####/info.md` at
  // revision `0b1b3b5b`: 35 sections of 283 across this estate, 13,613 bytes --
  // `Acceptance` 12 of 12, `Context for LLM` 20 of 41, `Related Steel Threads`
  // 3 of 55. No substitution is applied, and that is measured rather than
  // assumed by analogy with the work-package template: all ten of
  // `bin/intent_st:353`'s substitutions live outside every `## ` section.
  /// Every OTHER authored section of the thread, verbatim, in the order it was
  /// written.
  ///
  /// `objective` and `context` take the two sections every thread has and this
  /// takes the rest whole. Threads exceed the template freely -- 44 headings
  /// appear exactly once each across this project's own estate -- so a model
  /// naming a fixed set of sections silently drops whatever it did not foresee.
  ///
  /// Sections byte-identical to the template that created the file are NOT
  /// here: byte-identity to that artefact is evidence that no author wrote
  /// them, and carrying scaffolding files it as authored prose that the
  /// renderer then emits forever as though somebody had. Each such removal is
  /// recorded individually rather than counted.
  #[serde(default)]
  pub body: String,
  // PUBLISHED (D37) -- provenance here, contract in the `///` below.
  //
  // A CONSERVATION FIX, not an additive field (vc's ruling, data-model.md,
  // 2026-08-17). `legacy.rs` buffered a section only once a `## ` had been
  // seen, so every byte above the first heading fell on the floor -- and
  // `conservation_check.sh` had been reporting exactly that as LOST-PROSE from
  // the day its arm was written. I proposed the field believing the region was
  // carried and merely unclassified; it was not carried at all.
  //
  // 396 regions / 88,648 bytes across nine projects; 20 on the canary at
  // `42fb5269`, 15 thread-level and 5 work-package, 102 to 1020 bytes each.
  // ST0010's 485 bytes are a cancelled thread's deprecation blockquote and its
  // supersession pointer -- exactly what the cancellation discipline exists to
  // preserve, dropped with no drop record.
  //
  // NOT `body`, and the reason is load-bearing: `wp_info` renders `body` after
  // `## Objective`, so a preamble carried there returns in the wrong place --
  // trading a silent DROP for a silent MOVE, which is harder to see.
  /// Authored prose above the first `## `, minus the `# ` title line, stripped.
  ///
  /// Carried verbatim and never classified. The regions are largely metadata
  /// restatement, and that is the reason no classifier exists for them rather
  /// than a reason to build one: a model naming the shapes it foresaw drops
  /// what it did not, and the unforeseen remainder is the load-bearing half.
  ///
  /// Stored STRIPPED. The surrounding blank lines are markdown layout the
  /// renderer re-emits, so the trim is a normalisation rather than a loss --
  /// reported and counted as such, never silently adopted.
  #[serde(default)]
  pub preamble: String,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub related: Vec<Related>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub wps: Vec<WorkPackage>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub criteria: Vec<Criterion>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub tests: Vec<AcceptanceTest>,
  // PUBLISHED (D37) -- provenance here, contract in the `///` below.
  //
  // **The precondition for disk becoming optional, and it is measured rather
  // than argued.** Of 485 `.md` under the thread estate, 380 are in the store
  // and 52 are not -- one-off documents nobody modelled, found by counting and
  // named by no surface at all. Under the old model that was residue; under an
  // index-plus-render-on-demand disk it is what the first render destroys.
  //
  // **The line is by extension because the population is not what the ask
  // sounds like.** 304 files under thread directories are none of the
  // canonical five: 196 generated TAP baselines, 66 `.md`, 38 executable
  // shell instruments, 2 `.txt`, 2 `.tsv`. Carrying all of it would put
  // generated baselines and executables into the record of intent, and a store
  // holding executables wants mode bits, binary payloads and a merge story --
  // which is a version control system, and there is one a directory up.
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub attachments: Vec<Attachment>,
}

/// An authored file carried under a thread verbatim, because nothing else in
/// the model has a place for it.
///
/// The typed documents are parsed into fields. Everything else an author wrote
/// beside them -- a plan, a reference, a journal -- has no fields to parse
/// into, so it is carried whole rather than classified. **The content is
/// OPAQUE: nothing reads it, splits it or normalises it.**
///
/// Which files qualify is decided by EXTENSION and nothing else, so the
/// question is answerable without opening a file or forming a view about
/// whether it feels authored. A file outside that set is reported by name; it
/// is never silently passed over.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, SimpleObject)]
#[serde(deny_unknown_fields)]
pub struct Attachment {
  /// Path relative to the thread's own directory, eg `reference.md`.
  ///
  /// Relative to the THREAD rather than the project, so a file nested under it
  /// is addressed without a second collection to hold it.
  pub path: String,
  /// The file's content when it is TEXT, byte for byte. `None` when the
  /// attachment is OPAQUE, and that absence is the ONLY marker of which it is.
  ///
  /// **One representation, because a second field saying `opaque: true` would
  /// be a way for the two to disagree** -- the same argument [`Attachment::new`]
  /// already makes below about `bytes` and `sha256`. A reader asks whether the
  /// text is here; there is nothing else to consult and nothing to contradict.
  ///
  /// **Absence is unambiguous rather than merely convenient**: every attachment
  /// in every canon file written before this field became optional carries a
  /// `text`, so no existing artefact reads as opaque by omission. And because
  /// `Some(s)` serialises exactly as the old `String` did, the 98 canon files
  /// on disk do not move a byte.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub text: Option<String>,
  /// The content's length in bytes.
  pub bytes: u64,
  /// Lowercase hex SHA-256 of the content.
  ///
  /// What the file was when it was carried. Comparing it to the file on disk
  /// is how a hand edit is reported rather than silently overwritten.
  pub sha256: String,
  /// An OPAQUE attachment's bytes. **Never serialised into the thread's canon
  /// JSON** -- they travel as a sibling FILE at `intent/.canon/st/<ID>/<path>`,
  /// which is AC-03.2's rule in the direction that a `#[serde(skip)]` can
  /// actually enforce: there is no code path that could write them inline,
  /// rather than a convention that nobody writes them inline.
  ///
  /// **So a canon file parsed on its own yields an attachment with NEITHER
  /// half**, and that intermediate is real rather than hypothetical.
  /// [`crate::ingest::read`] closes it in the same step that opens the JSON,
  /// and a missing sidecar is a REFUSAL there -- canon naming bytes that do not
  /// exist is AC-03.6's invariant one level down, and reporting it as an empty
  /// attachment would be the silent form of exactly that.
  #[serde(skip)]
  #[graphql(skip)]
  pub blob: Option<Vec<u8>>,
}

impl Attachment {
  /// Carry `text` as the attachment at `path`.
  ///
  /// **The one constructor, because `bytes` and `sha256` are FUNCTIONS of
  /// `text` and a second way to set them is a way for them to disagree with
  /// it.** A stored hash that no longer describes the stored content reports
  /// skew against a file that is fine, or stays quiet about one that is not --
  /// and both failures look exactly like the check working.
  pub fn new(path: impl Into<String>, text: impl Into<String>) -> Self {
    let text = text.into();
    Self {
      path: path.into(),
      bytes: text.len() as u64,
      sha256: sha256_hex(text.as_bytes()),
      text: Some(text),
      blob: None,
    }
  }

  /// Carry `bytes` as the OPAQUE attachment at `path`.
  ///
  /// **The same one-constructor argument, and it is stronger here**: an opaque
  /// attachment's `sha256` is the only thing that can ever say whether the
  /// sidecar in canon is the file the author wrote, because nobody can read the
  /// content and notice it is wrong. A hash set by hand beside bytes set
  /// separately is a hash describing whatever was in the other variable.
  pub fn opaque(path: impl Into<String>, bytes: impl Into<Vec<u8>>) -> Self {
    let raw = bytes.into();
    Self {
      path: path.into(),
      bytes: raw.len() as u64,
      sha256: sha256_hex(&raw),
      text: None,
      blob: Some(raw),
    }
  }

  /// Whether this attachment's content is bytes rather than text.
  ///
  /// **Asked of `text`, never of `blob`, and the asymmetry is deliberate.**
  /// `blob` is `#[serde(skip)]`, so a canon file parsed on its own produces
  /// `blob: None` for a text attachment AND for an opaque one -- keying on it
  /// would call every attachment text at exactly the moment the distinction
  /// matters. `text` survives the parse, so it is the half that can answer.
  pub fn is_opaque(&self) -> bool {
    self.text.is_none()
  }

  /// The content as bytes, whichever form it is carried in.
  ///
  /// The one place the two halves rejoin, so a caller that only needs bytes --
  /// hydration, hashing, a byte comparison against disk -- never has to know
  /// which it got, and cannot get it wrong in one of the two arms.
  pub fn as_bytes(&self) -> Option<&[u8]> {
    match (&self.text, &self.blob) {
      (Some(text), _) => Some(text.as_bytes()),
      (None, Some(raw)) => Some(raw),
      // An opaque attachment whose sidecar has not been loaded. Neither half is
      // present and inventing an empty one here is how a missing file becomes a
      // zero-byte write.
      (None, None) => None,
    }
  }
}

/// The ratified steel-thread machine (data-model.md, hv 2026-08-15). Declared
/// in LIFECYCLE order, entry first, because this list is the schema face's enum
/// order and a reader meets it there before they meet the transition table.
///
/// **`Triage` is not v2's `Tbc` renamed, and the distinction is load-bearing at
/// migration.** v2's `TBC` means "To Be Commenced" -- `bin/intent_helpers:544`
/// maps both `tbc` and `to be commenced` to `Not Started` -- so every v2 `TBC`
/// migrates to [`ThreadStatus::NotStarted`]. `Triage` reuses none of that
/// meaning and begins with zero legacy members; mapping on the string would
/// invent a triage decision nobody made, for every thread that ever carried it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Enum)]
#[serde(rename_all = "kebab-case")]
pub enum ThreadStatus {
  Triage,
  NotStarted,
  Wip,
  Hold,
  Completed,
  Cancelled,
}

impl ThreadStatus {
  /// v2's spelling, for a human reading a table or a generated view.
  ///
  /// **On the type, because the vocabulary belongs to it** (issue 0041). It was
  /// spelled twice -- `views.rs` for the committed markdown and `render.rs` for
  /// the terminal -- byte-identical on all six arms, both private, so neither
  /// crate could call the other's and nothing compared them. **Each copy was
  /// held in place by its own test against hand-written literals, so each test
  /// certified its own copy and no test could see the other one.**
  ///
  /// This is not the wire spelling: serde writes kebab-case into `thread.json`,
  /// and these are the words a person reads. Two vocabularies for two audiences
  /// is correct; two copies of one vocabulary is not.
  ///
  /// The vocabulary is v2's (`canonical_status`, `bin/intent_helpers:535`) with
  /// **one deliberate divergence**: v2 collapsed `TBC` into `Not Started` for
  /// display, so a thread whose file said TBC appeared in the index as
  /// something else. The model distinguishes them, and reproducing the collapse
  /// would be v3 faithfully reproducing a v2 defect -- a `corrected` register
  /// row, not a parity break.
  /// Whether the thread has reached an end state.
  ///
  /// On the type beside [`display`], and for the same reason: it was private
  /// in `views.rs`, so `doctor` could not ask the question and would have
  /// grown a second copy of the answer.
  ///
  /// [`display`]: ThreadStatus::display
  pub fn is_closed(self) -> bool {
    matches!(self, Self::Completed | Self::Cancelled)
  }

  pub fn display(self) -> &'static str {
    match self {
      Self::Triage => "Triage",
      Self::NotStarted => "Not Started",
      Self::Wip => "WIP",
      Self::Hold => "On Hold",
      Self::Completed => "Completed",
      Self::Cancelled => "Cancelled",
    }
  }

  /// The checkbox glyph for the flat work view.
  ///
  /// **On the type beside [`display`], for the third time and the same reason.**
  /// It was not spelled twice here -- it was not spelled at all. `views.rs`
  /// emitted a constant `- [ ]` for every row of every bucket, so the flat view
  /// rendered six states as one, and two CANCELLED threads appeared under
  /// `## DONE` indistinguishable from the 52 completed ones. A glyph computed
  /// from nothing is an instrument whose output is independent of the thing it
  /// measures (ic), and nothing could have caught it downstream: the value was
  /// not wrong for some inputs, it was constant for all of them.
  ///
  /// The vocabulary is v2's (`status_box`, `bin/intent_todo:63`) on the four
  /// states v2 had, and the two v3 added take the remaining ground:
  /// `Triage` claims the `?` that was v2's fallthrough for a status it could
  /// not read, which is what triage now NAMES, and `Hold` takes `!`.
  ///
  /// **Exhaustive, with no wildcard arm, deliberately.** A `_ => '?'` would
  /// make this total by making it silent, and the next variant added would
  /// render as undecided instead of failing to compile. Six states, six
  /// glyphs; a seventh must be decided here.
  ///
  /// [`display`]: ThreadStatus::display
  pub fn glyph(self) -> char {
    match self {
      Self::Triage => '?',
      Self::NotStarted => ' ',
      Self::Wip => '-',
      Self::Hold => '!',
      Self::Completed => 'x',
      Self::Cancelled => '~',
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Enum)]
#[serde(rename_all = "kebab-case")]
pub enum AcceptanceMode {
  Exempt,
}

/// A cross-reference to another steel thread, with the note explaining WHY it
/// is related. v2 carried these as free bullets under `## Related Steel
/// Threads`; the id and the reason are separable facts, so they separate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, SimpleObject)]
#[serde(deny_unknown_fields)]
pub struct Related {
  /// The related thread's natural id, eg `ST0000`.
  pub id: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub note: Option<String>,
}

// ---------------------------------------------------------------------------
// Work package
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, SimpleObject)]
#[serde(deny_unknown_fields)]
pub struct WorkPackage {
  /// Unique within the thread, and rendered zero-padded as `WP-<seq>`.
  pub seq: u32,
  pub title: String,
  /// The size, absent only when [`scope_legacy`] carries a v2 value the enum
  /// cannot express.
  ///
  /// [`scope_legacy`]: WorkPackage::scope_legacy
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub scope: Option<TShirt>,
  // PUBLISHED (D37): schemars lifts a `///` here into thread.schema.json and
  // async-graphql into the SDL, and `intent schema` prints both -- so the
  // design reasoning is in `//` and the `///` below says only what a consumer
  // of the format needs. The guard caught this comment carrying an internal
  // decision number on its first run, which is the second time that rule has
  // paid for itself in the same file.
  //
  // v2 read `scope` as free text, and this repository's own corpus carries
  // eleven spellings for six sizes. Ten of them are the same six values written
  // differently -- `Extra Small` and `XS` say the identical thing -- so
  // rendering those canonically loses nothing. The eleventh decides the rule:
  // `Medium-Large`, one work package, in a CLOSED thread. It sits BETWEEN two
  // enum members, and the ratified carry policy forbids all three obvious moves
  // at once -- normalising it to `M` or `L` is a guess, blocking violates
  // lossless-by-carrying for a closed thread, and dropping it is loss outright.
  //
  // So it is carried AS legacy, reusing the same shape the model already sets
  // for a v2 acceptance-test row: the value stays visible as what it was, and
  // the enum stays honest for everything new. The general form is the model's
  // strictness posture one level down -- an unknown enum VALUE is marked by
  // name, exactly as an unknown FIELD is.
  //
  // Carried, NEVER interpreted. Nothing reads it to answer a question about
  // size; anything that did would rebuild v2's answer-confidently-from-partial-
  // evidence habit inside v3. And a live thread never produces one -- it stays
  // blocked until clean -- so a carried scope on a live thread is itself a
  // defect, which `doctor` reports.
  /// A scope recorded by an older Intent that is outside the size vocabulary,
  /// carried verbatim. Present only when `scope` is absent.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub scope_legacy: Option<Legacy>,
  pub status: WpStatus,
  /// Why the work package is in [`WorkPackage::status`] -- `wp reopen` is the
  /// one WP transition the ratified machine guards with "reason recorded".
  /// Same rule as [`Thread::status_reason`]: it belongs to the current status
  /// and any transition without a reason clears it.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status_reason: Option<String>,
  // PUBLISHED (D37): the two `///` blocks below become field descriptions in
  // thread.schema.json and the SDL, so the design provenance that used to be in
  // them -- which decision modelled what, which of our work packages would have
  // destroyed which of our own files -- is here instead. D22 modelled
  // `objective` and `context` on the thread and stopped there; `WP/<NN>/info.md`
  // is the same mixed authored/generated file one level down, and nothing
  // carried its prose. Without `objective` + `body` the WP-10 migration would
  // have destroyed it -- ST0056's own `WP/13/info.md` is the search work
  // package's spec, hundreds of lines -- and hv has ratified that a migration is
  // never lossy. Two fields rather than three is vc's call under D28.
  // `deliverables` is deliberately NOT modelled as an array: this thread already
  // demoted it when WP-02 closed with `intent schema` unbuilt, and structuring
  // it would re-privilege what the acceptance contract replaced.
  /// What this work package ships: the one section every work package has.
  #[serde(default)]
  pub objective: String,
  /// Every OTHER authored section of the work package, verbatim.
  ///
  /// Work packages exceed the template freely, so `objective` takes the one
  /// guaranteed section and this takes the rest whole. That is lossless by
  /// construction, where a fixed set of named sections would silently drop
  /// anything unforeseen.
  #[serde(default)]
  pub body: String,
  // Same field one level down, and it is not a symmetry argument: 5 of the
  // canary's 20 regions are work-package ones, measured. See `Thread::preamble`
  // for the ruling and the population.
  /// Authored prose above the first `## `, minus the `# ` title line, stripped.
  ///
  /// Carried verbatim and never classified, and rendered ABOVE the generated
  /// sections -- putting it in `body` would return it below `## Objective`,
  /// which preserves the bytes and moves them.
  #[serde(default)]
  pub preamble: String,
}

/// T-shirt sizes -- the only sizing vocabulary in Intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Enum)]
pub enum TShirt {
  XS,
  S,
  M,
  L,
  XL,
  XXL,
}

impl TShirt {
  /// The six sizes, smallest first.
  ///
  /// Ordered because two callers read it as a list a human sees: the CLI's
  /// refusal names the permitted set, and a set printed in declaration order
  /// reads as a scale while an arbitrary order reads as a bag.
  pub const ALL: [TShirt; 6] = [
    TShirt::XS,
    TShirt::S,
    TShirt::M,
    TShirt::L,
    TShirt::XL,
    TShirt::XXL,
  ];

  /// A caller's spelling of a size, or `None`.
  ///
  /// **Derived from the serialisation rather than from a second table of
  /// spellings.** A `match` on six string literals here would be the canonical
  /// vocabulary written twice -- once in the enum and once in the parse -- and
  /// the two would part company at the first rename, silently, because a rename
  /// updates the variant and a literal has nothing pointing at it. So this asks
  /// each variant how it serialises and compares against that.
  ///
  /// **Case-insensitive, and that is not a seventh value.** `l` and `L` are the
  /// same one of the six the dispatch table declares; requiring the shift key
  /// would refuse a correct answer over its typography. Anything outside the six
  /// is `None` -- v2's long forms (`Small`, `Medium`) are a FOREIGN vocabulary
  /// read at ingest, and [`crate::legacy`] adds them there rather than here,
  /// because the set an operator may type and the set v2 may have written are
  /// different questions that happen to overlap.
  pub fn parse(raw: &str) -> Option<Self> {
    let want = raw.trim().to_ascii_lowercase();
    Self::ALL
      .into_iter()
      .find(|size| enum_str(size).to_ascii_lowercase() == want)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Enum)]
#[serde(rename_all = "kebab-case")]
pub enum WpStatus {
  NotStarted,
  Wip,
  Done,
}

impl WorkPackage {
  /// The size as a human reads it in a table or a generated view.
  ///
  /// **One home for three states, because the states are the whole point.** A
  /// recorded size renders as itself; a carried v2 value renders AS ITSELF AND
  /// MARKED, which is what "the value stays visible as legacy rather than
  /// being silently canonicalised into a lie" means at the point a reader
  /// meets it; a scope nobody ever recorded renders empty, because inventing a
  /// size for it is the substitution this whole form exists to remove.
  ///
  /// On the type for the reason issue 0041 gives: `render.rs` had a private
  /// match and `views.rs` called `enum_str`, so the terminal and the committed
  /// markdown reached the same answer by two routes that nothing compared.
  pub fn scope_display(&self) -> String {
    match (&self.scope, &self.scope_legacy) {
      (Some(scope), _) => enum_str(scope).to_string(),
      (None, Some(legacy)) => format!("{} (legacy)", legacy.raw),
      (None, None) => String::new(),
    }
  }
}

impl WpStatus {
  /// v2's spelling, for a human. Same one-home rule as
  /// [`ThreadStatus::display`], and it was the same defect: two private copies
  /// in two crates, each pinned by its own test.
  pub fn display(self) -> &'static str {
    match self {
      Self::NotStarted => "Not Started",
      Self::Wip => "WIP",
      Self::Done => "Done",
    }
  }

  /// The checkbox glyph, same one-home rule as [`ThreadStatus::glyph`] and the
  /// same defect: the flat view emitted a constant for work packages too.
  ///
  /// v2 folded a work package's `Done` into `Completed` before asking for a
  /// glyph, so `x` is v2's answer as well as this one.
  pub fn glyph(self) -> char {
    match self {
      Self::NotStarted => ' ',
      Self::Wip => '-',
      Self::Done => 'x',
    }
  }
}

// ---------------------------------------------------------------------------
// Acceptance (the 0013 four-state AC model + the 0017 AT grammar, reified)
// ---------------------------------------------------------------------------

// PLAIN COMMENTS, NOT DOC COMMENTS -- for the reason stated three fields down,
// and it caught me writing this block. schemars lifts `///` into the JSON
// Schema face as a `description` and async-graphql lifts it into the SDL, both
// of which `intent schema` prints to a consumer's terminal. The first cut of
// this block was a doc comment and put "AC-02.6", a node name, a date, and a
// test file path into two published faces -- **a D37 violation authored while
// closing a different hole**, in the one file that already carries the warning.
//
// **The `kind`/`state` invariant is carried in the JSON Schema face, not only
// in Rust** (vc, 2026-08-15). Two fields can express nonsense --
// `{kind: test, state: satisfied}` records a satisfaction nothing computed,
// `{kind: non-test, state: computed}` claims a derivation with nothing to
// derive from -- and `AcState::permitted_for` is the one place that says so.
// The facade's `NonTestOnly` guard shuts the door at the API, which is the gate
// that matters under D01; this shuts it at the FILE.
//
// It has to be in the FACE rather than only in ingest because of what the
// extract is for. Under D34 the committed extract is the interchange, and an
// external reader validating a `thread.json` against the published face must
// reach the same verdict Intent does -- a rule that lives only in this crate is
// a rule every other reader has to reimplement, which is the thing openness
// exists to prevent. Expressing it here also means ingest enforces it for free:
// ingest validates against this generated schema before deserialising, so the
// file refusal and the published contract are one artefact rather than two that
// agree today.
//
// `tests/ac_kind_state_invariant.rs` holds the two sides to each other over
// every variant, and fails if a variant appears that this block has not been
// taught -- because the block is hand-written JSON, which is exactly the kind
// of hand-kept roster that goes stale in silence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, SimpleObject)]
#[serde(deny_unknown_fields)]
#[graphql(complex)]
#[schemars(extend("allOf" = [
  {
    "if": { "properties": { "kind": { "const": "test" } }, "required": ["kind"] },
    "then": { "properties": { "state": { "properties": {
      "is": { "enum": ["computed", "descoped", "withdrawn"] }
    } } } }
  },
  {
    "if": { "properties": { "kind": { "const": "non-test" } }, "required": ["kind"] },
    "then": { "properties": { "state": { "properties": {
      "is": { "enum": ["unsatisfied", "satisfied", "descoped", "withdrawn"] }
    } } } }
  }
]))]
pub struct Criterion {
  /// `AC-<gg>.<n>`; group `00` is ST-level, otherwise the WP seq.
  pub id: String,
  pub text: String,
  /// Test-backed or not. **Authored on the AC line and independent of the
  /// state**, which is why it stays its own field: `(non-test)` is a literal
  /// the author writes (`bin/intent_acceptance:90`), not something derived from
  /// AT coverage, so the type can carry it soundly (vc, 2026-08-15).
  pub kind: AcKind,
  // A plain comment, NOT a doc comment: schemars lifts `///` into the JSON
  // Schema face as a `description`, and why the SDL needs a projection is a
  // GraphQL concern that has no business in the JSON face. The reasoning lives
  // in `graphql::AcStateView`, which is the thing it describes.
  #[graphql(skip)]
  pub state: AcState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Enum)]
#[serde(rename_all = "kebab-case")]
pub enum AcKind {
  Test,
  NonTest,
}

/// **What a criterion RECORDS.** One enum, replacing the `satisfied:
/// Option<bool>` + `scope: AcScope` pair that produced "three stored values,
/// two meanings, one of them never written" (hv ruling, 2026-08-15).
///
/// The ratified machine is `Satisfied | Unsatisfied | Descoped | Withdrawn`.
/// This type has a fifth variant, [`AcState::Computed`], and it is what makes
/// the ratified asymmetry STRUCTURAL rather than a rule somebody enforces:
///
/// - **A test-backed criterion in scope records `Computed`** -- nothing about
///   satisfaction is stored, because it is derived from covering green ATs.
///   There is no field for `ac satisfy` to write and no method that could,
///   where before there was a `satisfied: Option<bool>` that the linter's L5
///   was the only thing keeping empty.
/// - **`Satisfied` carries its evidence, which must not be empty** (hv,
///   2026-08-15): a criterion with no test to run has nothing BUT its evidence,
///   so a satisfaction with none behind it is the one state this type exists to
///   rule out. The rule is published here as `minLength` so a reader of this
///   face reaches the same verdict Intent does, and the service refuses the
///   same call at its API.
/// - **`Descoped` and `Withdrawn` apply to BOTH kinds** and are always stored:
///   they are decisions about the requirement, not about its satisfaction.
///
/// `Descoped` and `Withdrawn` stay distinct with no direct edge between them --
/// descoped means the requirement still exists on a named thread and is a
/// pointer you can follow; withdrawn means it does not exist at all. Moving
/// between them routes through `Unsatisfied` so the audit trail records the
/// intermediate decision instead of smearing two facts into one.
///
/// The tag is `is` rather than `state` so the extract reads
/// `"state": {"is": "satisfied", ...}` rather than doubling the word. Nesting
/// rather than `#[serde(flatten)]` is forced: flatten and `deny_unknown_fields`
/// do not compose in serde, and strict rejection of unknown fields is the
/// property that must win.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "is", rename_all = "kebab-case", deny_unknown_fields)]
pub enum AcState {
  /// Test-backed and in scope: satisfaction is computed from covering ATs and
  /// **nothing is stored**. Carries no payload -- a test-backed criterion's
  /// evidence IS the AT relation and must not be copied into a state field
  /// (hv, 2026-08-15).
  Computed,
  /// Non-test and in scope, not yet satisfied. The entry state for an authored
  /// criterion.
  Unsatisfied,
  /// Non-test and satisfied, carrying the evidence that settled it.
  //
  // **The evidence must be NON-EMPTY, and `String` does not say that.** The
  // ruling was recorded as structural -- "unconstructible without evidence" --
  // and a required field of type `String` delivers a narrower property than the
  // one that was ruled: it makes the field mandatory, not the evidence present.
  // `evidence: String::new()` builds it. Three separate pieces of reasoning
  // rested on the stronger reading (this variant's doc, `AcState`'s, and
  // `Facade::ac_satisfy`'s "structural rather than enforced"), which is why no
  // guard was written and why ic could trace an empty evidence from the CLI all
  // the way to the close gate counting it (2026-08-15).
  //
  // So the constraint is written down where it is checkable, on the three
  // enforcement points this estate already uses for the kind/state invariant:
  // `minLength` here refuses the FILE (ingest validates against this generated
  // schema, and an external reader of the published face reaches the same
  // verdict under D34), `Guard::EvidenceRecorded` refuses the API call, and
  // `doctor` reports an estate that already carries one.
  Satisfied {
    #[schemars(length(min = 1))]
    evidence: String,
  },
  Descoped {
    /// The thread the requirement moved to, eg `ST0000`.
    to: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
  },
  Withdrawn {
    // Non-empty for the same reason and by the same three points: a withdrawal
    // whose reason is blank records that a requirement was dropped and nothing
    // about why, which is the state `withdraw` exists to prevent being reached
    // by deletion.
    #[schemars(length(min = 1))]
    reason: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    by: Option<String>,
  },
}

impl AcState {
  /// The state's NAME, without its payload.
  ///
  /// **`enum_str` cannot answer this**: three variants carry payloads, so serde
  /// renders them as objects and `enum_str` panics on a non-string. That is why
  /// the only copy of these five words lived in a test helper
  /// (`mutation_completeness.rs::state_name_of`) -- a vocabulary owned by a test
  /// and unavailable to production, which is the direction this reverses. The
  /// helper now delegates here.
  ///
  /// **Distinct from `AcRow.state`, which is `ac list`'s composed line** --
  /// `descoped-to: ST0057`, `satisfied: yes`. That answers "where does this
  /// criterion stand", and this answers "which state is it in". Issue 0050's
  /// no-op message needs the second and would read as a rendering fault with the
  /// first.
  ///
  /// Exhaustive on purpose, for the reason [`AcState::permitted_for`] is: a sixth
  /// variant should not compile until someone names it.
  pub fn name(&self) -> &'static str {
    match self {
      Self::Computed => "computed",
      Self::Unsatisfied => "unsatisfied",
      Self::Satisfied { .. } => "satisfied",
      Self::Descoped { .. } => "descoped",
      Self::Withdrawn { .. } => "withdrawn",
    }
  }

  /// The state a criterion of `kind` starts in.
  ///
  /// **The entry state differs by kind, which is the collapse's whole point**:
  /// an authored criterion starts `Unsatisfied` and a test-backed one starts
  /// `Computed`, so the pair (kind, entry) can never be the inconsistent
  /// combination the two-field model made representable.
  pub fn entry(kind: AcKind) -> Self {
    match kind {
      AcKind::Test => Self::Computed,
      AcKind::NonTest => Self::Unsatisfied,
    }
  }

  /// **Whether a criterion of `kind` can hold this state -- the one home for
  /// the cross-field invariant.**
  ///
  /// The pair is checked in three places and decided in exactly one: the
  /// facade's `NonTestOnly` guard refuses the transition, the JSON Schema face
  /// on [`Criterion`] refuses the file, and `doctor` reports an estate that
  /// already carries the mismatch. Before this, the rule was a `match` inside
  /// `doctor` with a `_ => None` arm, which meant a sixth variant would have
  /// been silently consistent with everything.
  ///
  /// The match is exhaustive on purpose. **A new variant does not compile until
  /// someone says which kinds may hold it**, which is the property a fallthrough
  /// arm gives away.
  pub fn permitted_for(&self, kind: AcKind) -> bool {
    match self {
      // Derived from covering ATs, so there must be ATs to derive from.
      Self::Computed => kind == AcKind::Test,
      // A recorded satisfaction, which is double truth on a test-backed
      // criterion: its satisfaction is computed and cannot also be asserted.
      Self::Unsatisfied | Self::Satisfied { .. } => kind == AcKind::NonTest,
      // Decisions about the REQUIREMENT rather than about its satisfaction, so
      // both kinds hold them and both must store them -- an AT status cannot
      // recompute a scope decision (vc, 2026-08-15).
      Self::Descoped { .. } | Self::Withdrawn { .. } => true,
    }
  }

  /// Whether the requirement is still being asked for.
  pub fn in_scope(&self) -> bool {
    matches!(
      self,
      Self::Computed | Self::Unsatisfied | Self::Satisfied { .. }
    )
  }

  /// The evidence, when there is any to have.
  pub fn evidence(&self) -> Option<&str> {
    match self {
      Self::Satisfied { evidence } => Some(evidence),
      _ => None,
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, SimpleObject)]
#[serde(deny_unknown_fields)]
pub struct AcceptanceTest {
  /// `AT-<gg>.<n>`.
  pub id: String,
  pub kind: AtKind,
  /// Test kind: the repo-relative test file (the 0017 reference rules --
  /// at least one `/`, no `:`).
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub file: Option<String>,
  /// Non-test kind: what was read / eyeballed.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub prose: Option<String>,
  /// Covered AC ids; at least one.
  pub covers: Vec<String>,
  pub status: AtStatus,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub note: Option<String>,
  /// Present on rows carried from a v2 estate under the closed-thread carry
  /// policy (migration.md, hv-ruled 2026-08-14). When present, [`file`] may be
  /// absent: the legacy reference could be a `::name` citation or a multi-file
  /// list, neither of which the 0017 grammar can express, and neither of which
  /// is guessed at. See [`Legacy`].
  ///
  /// [`file`]: AcceptanceTest::file
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub legacy: Option<Legacy>,
}

/// A v2 AT reference carried whole into the model -- marked legacy, nothing
/// guessed, nothing dropped, nothing reformatted.
///
/// The distinction this type encodes (migration.md): carrying a row into a
/// richer model destroys nothing, where the 0017 `--fix` destroyed one end of
/// a two-ended migration. [`raw`] is never parsed and never rewritten; it is
/// evidence, not data.
///
/// [`raw`]: Legacy::raw
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, SimpleObject)]
#[serde(deny_unknown_fields)]
pub struct Legacy {
  /// The verbatim v2 reference, exactly as it appeared on the row.
  pub raw: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Enum)]
#[serde(rename_all = "kebab-case")]
pub enum AtKind {
  Test,
  NonTest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Enum)]
#[serde(rename_all = "kebab-case")]
pub enum AtStatus {
  ToWrite,
  Red,
  Green,
  // **`n-a` is the WIRE form and `n/a` is the printed one** -- see
  // [`AtStatus::display`]. Deliberately a `//` and not a `///`: this is reasoning
  // for a maintainer, and a `///` here is lifted verbatim into the committed JSON
  // Schema and SDL faces. Putting it above the variant as documentation drifted
  // both faces and reddened three tests, which is the rule this project already
  // holds -- doc comments are shipped output, plain comments are reasoning -- and
  // it was broken within hours of being written down.
  /// Non-test rows only -- the doc / eyeball / gate status. `n-a` is not
  /// green; satisfaction lives on the AC's own line.
  #[serde(rename = "n-a")]
  Na,
}

impl AtStatus {
  /// The status as a human reads it -- in a printed line, in `at list`, and in
  /// the generated `acceptance.md` row.
  ///
  /// **This is `enum_str`'s wire form leaking into human output, and it leaked
  /// three ways at once** (ic, issue 0056). `Na` serialises as `n-a` because that
  /// is its JSON tag, and the authored form is `n/a`: measured across this
  /// estate's `acceptance.md` files, every authored AT row spells it `n/a` and
  /// none spells it `n-a`. So the view was one projection away from rewriting
  /// every one of them into a
  /// spelling v2's own linter rejects at L1 -- a migration hazard rather than a
  /// preference, and the same shape as `wp show` printing `wip` where every other
  /// surface printed `WIP`.
  ///
  /// The other three agree with `enum_str` today, which is exactly why this was
  /// hard to see: two of the three surface verbs are byte-identical to v2's
  /// tokens, so echoing the wrong source is correct twice and wrong once. **The
  /// coincidence is the hiding mechanism, not the absence of one.**
  pub fn display(self) -> &'static str {
    match self {
      Self::ToWrite => "to-write",
      Self::Red => "red",
      Self::Green => "green",
      Self::Na => "n/a",
    }
  }
}

// ---------------------------------------------------------------------------
// Issue
// ---------------------------------------------------------------------------

/// An issue: `issues/<n>.json` (structure) + `issues/<n>.md` (authored body).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, SimpleObject)]
#[serde(deny_unknown_fields)]
pub struct Issue {
  /// Always [`ISSUE_SCHEMA`].
  pub schema: String,
  /// Rendered zero-padded, eg `0021`.
  pub number: u32,
  pub slug: String,
  pub title: String,
  pub status: IssueStatus,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub severity: Option<String>,
  pub created: String,
  // **Absent for every issue migrated from v2, and that is the format rather
  // than a gap**: v2's issue frontmatter carries six keys and a closed date is
  // not one of them, so there is nothing to carry and nothing to back-fill it
  // from -- the filesystem mtime is a fact about a file, not about the world
  // (D42). All-`None` here is v2 provenance, never a scanner that failed.
  //
  // The reasoning is a `//` and the doc is one line, because the `///` ships:
  // schemars lifts it into the published JSON Schema face, where a consumer
  // needs to know what the field means and not what our own estate contains.
  /// When it was closed, if it is closed.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub closed: Option<String>,
  /// Who reported it, free text, exactly as recorded.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub reporter: Option<String>,
  // PUBLISHED (D37) -- provenance here, contract in the `///` below.
  //
  // **THE FIELD `legacy.rs` REFUSED TO INVENT MID-WALK.** Its `issues()` doc
  // says so in as many words -- "THE BODY HAS NO HOME AND THIS DOES NOT INVENT
  // ONE" -- and reported the hole to vc to price rather than quietly dropping
  // it or growing the model from inside a converter. vc specced it; this is
  // the other end of that report.
  //
  // Measured on this estate: 40 issue files, 443,643 bytes, each of them the
  // one markdown file in a v2 issue directory and nowhere else. Under the old
  // model that was residue. **Under hv's disk-optional ruling it is data loss
  // at the first render**, which is what moved it from a TODO to a gate.
  //
  // ONE FIELD, NOT A PARSE, and the estate is why: 503 sections over 30
  // distinct headings of which 21 appear exactly once. A model naming the
  // shapes it foresaw drops the rest, and the unforeseen remainder is the
  // load-bearing half -- the argument `Thread.body` settled one entity over.
  //
  // **The `# <nnnn>: <title>` line is CARRIED, not reconstructed, and that is
  // measured rather than assumed**: it reconstructs from `number` + `title` on
  // 37 of 40 and NOT on 0011, 0014 and 0035, whose v2 frontmatter quotes the
  // title. Dropping it would have been correct-looking on 37 files and wrong
  // on 3 -- and silent on all 40.
  /// The issue's authored prose: everything below the frontmatter, verbatim.
  ///
  /// Carried whole and never parsed. An issue's headings are its author's, not
  /// a template's, so there are no fields for a parse to land in -- and
  /// parsing into nothing is how prose becomes a drop that no surface reports.
  ///
  /// **No normalisation at all**, including the blank line the format puts
  /// between the frontmatter and the first heading. Rendering the frontmatter
  /// and then this reproduces the file byte for byte, which is what makes the
  /// round trip lossless without anything having to compensate for it.
  #[serde(default)]
  pub body: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Enum)]
#[serde(rename_all = "kebab-case")]
pub enum IssueStatus {
  Open,
  Closed,
}

impl IssueStatus {
  /// v2's spelling, for a human: `OPEN` / `CLOSED`.
  ///
  /// **On the type for the reason [`ThreadStatus::display`] is**, and it was the
  /// same defect one entity over: `render.rs` spelled this as
  /// `enum_str(&status).to_ascii_uppercase()` at two sites, so the uppercase
  /// convention lived in the CLI crate while every other status vocabulary lived
  /// here. Machine 4's no-op line needed the same spelling from the facade, and a
  /// third copy is what this replaces.
  pub fn display(self) -> &'static str {
    match self {
      Self::Open => "OPEN",
      Self::Closed => "CLOSED",
    }
  }
}
