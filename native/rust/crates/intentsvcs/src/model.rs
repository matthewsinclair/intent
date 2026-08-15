//! The reified Intent model -- the single authored master (design.md D01,
//! data-model.md). Everything else is generated from these types: the JSON
//! Schema face via schemars, the DDL the store applies, and (WP-04) the
//! GraphQL SDL.
//!
//! Strictness (D05): every struct is `deny_unknown_fields` -- an unknown
//! field in canonical JSON is refused by name at deserialize time, never
//! silently dropped. schemars mirrors that as `additionalProperties: false`.
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

// ---------------------------------------------------------------------------
// Steel thread
// ---------------------------------------------------------------------------

/// A steel thread: `st/<ID>/thread.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, SimpleObject)]
#[serde(deny_unknown_fields)]
pub struct Thread {
  /// Always [`THREAD_SCHEMA`]; lets validators pick the schema.
  pub schema: String,
  /// Natural id, eg `ST0056`. Global identity is `(project_id, id)` (D15).
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
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub completed: Option<String>,
  /// `exempt` (ST0048) or absent = acceptance enforced.
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
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub related: Vec<Related>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub wps: Vec<WorkPackage>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub criteria: Vec<Criterion>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub tests: Vec<AcceptanceTest>,
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
  /// The related thread's natural id, eg `ST0043`.
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
  /// Rendered `WP-01`; unique within the thread.
  pub seq: u32,
  pub title: String,
  pub scope: TShirt,
  pub status: WpStatus,
  /// Why the work package is in [`WorkPackage::status`] -- `wp reopen` is the
  /// one WP transition the ratified machine guards with "reason recorded".
  /// Same rule as [`Thread::status_reason`]: it belongs to the current status
  /// and any transition without a reason clears it.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status_reason: Option<String>,
  /// What this work package ships. The guaranteed authored section (D28).
  ///
  /// D22 modelled `objective` and `context` on the thread and stopped there;
  /// `WP/<NN>/info.md` is the same mixed file one level down, and nothing
  /// carried its prose. Without this the WP-10 migration would have destroyed
  /// it -- ST0056's own `WP/13/info.md` is the search work package's spec,
  /// hundreds of lines -- and hv has ratified that a migration is never lossy.
  #[serde(default)]
  pub objective: String,
  /// Every OTHER authored section of the work package, verbatim.
  ///
  /// Two fields rather than three, deliberately (vc, D28). Real work packages
  /// exceed the template freely -- WP-13 carries "Why the incumbents go", "The
  /// tiers", "The seams" -- so `objective` takes the one guaranteed section
  /// and `body` takes the rest whole. That is lossless by construction, where
  /// a fixed set of named sections would silently drop anything unforeseen.
  ///
  /// `deliverables` is deliberately NOT modelled as an array: it is the
  /// artefact this thread already demoted when WP-02 closed with `intent
  /// schema` unbuilt, and structuring it would re-privilege what ACs replaced.
  #[serde(default)]
  pub body: String,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Enum)]
#[serde(rename_all = "kebab-case")]
pub enum WpStatus {
  NotStarted,
  Wip,
  Done,
}

// ---------------------------------------------------------------------------
// Acceptance (the 0013 four-state AC model + the 0017 AT grammar, reified)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, SimpleObject)]
#[serde(deny_unknown_fields)]
#[graphql(complex)]
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
/// - **`Satisfied` carries its evidence and cannot be constructed without it**
///   (hv, 2026-08-15), so "satisfied with no evidence" stops being a state the
///   model can represent, rather than one a guard has to refuse.
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
/// do not compose in serde, and D05's strictness is the thing that must win.
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
  /// Non-test and satisfied. **Unconstructible without evidence.**
  Satisfied { evidence: String },
  Descoped {
    /// The thread the requirement moved to, eg `ST0057`.
    to: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
  },
  Withdrawn {
    reason: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    by: Option<String>,
  },
}

impl AcState {
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
  /// Non-test rows only -- the doc / eyeball / gate status. `n-a` is not
  /// green; satisfaction lives on the AC's own line.
  #[serde(rename = "n-a")]
  Na,
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
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub closed: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Enum)]
#[serde(rename_all = "kebab-case")]
pub enum IssueStatus {
  Open,
  Closed,
}
