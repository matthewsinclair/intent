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
  /// ISO 8601 date, `YYYY-MM-DD`.
  pub created: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub completed: Option<String>,
  /// `exempt` (ST0048) or absent = acceptance enforced.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub acceptance: Option<AcceptanceMode>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub wps: Vec<WorkPackage>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub criteria: Vec<Criterion>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub tests: Vec<AcceptanceTest>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Enum)]
#[serde(rename_all = "kebab-case")]
pub enum ThreadStatus {
  NotStarted,
  Wip,
  Tbc,
  Hold,
  Completed,
  Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Enum)]
#[serde(rename_all = "kebab-case")]
pub enum AcceptanceMode {
  Exempt,
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
  pub kind: AcKind,
  // A plain comment, NOT a doc comment: schemars lifts `///` into the JSON
  // Schema face as a `description`, and why the SDL needs a projection is a
  // GraphQL concern that has no business in the JSON face. The reasoning lives
  // in `graphql::AcScopeView`, which is the thing it describes.
  #[graphql(skip)]
  pub scope: AcScope,
  /// Non-test only: the named evidence reference.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub evidence: Option<String>,
  /// Non-test only. Test-backed satisfaction is COMPUTED from covering green
  /// ATs and never stored -- storing it would be double truth (data-model.md).
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub satisfied: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Enum)]
#[serde(rename_all = "kebab-case")]
pub enum AcKind {
  Test,
  NonTest,
}

/// The four AC states beyond satisfied/unsatisfied (issue 0013): in scope,
/// descoped to a named thread, or withdrawn with its reason on the record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "state", rename_all = "kebab-case", deny_unknown_fields)]
pub enum AcScope {
  InScope,
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
