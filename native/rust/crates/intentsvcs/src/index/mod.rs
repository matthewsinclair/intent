//! The search index: the scope rule, the reconciler, and what each answers.
//!
//! ST0069 WP-18 builds the first of these. The module tree is the estate's PFIC
//! shape -- a pure core and an impure rim -- and it is laid out here in full so
//! that a later package adds a file rather than moving one.

pub mod corpus;
pub mod freshness;
pub mod reconcile;
pub mod source;
pub mod status;
pub mod symbols;

/// One `index_file` row, in the shape the store holds it.
///
/// **THE STORED SHAPE IS STRINGS, AND THE DECISION SHAPE IS ENUMS, AND THIS IS
/// DELIBERATELY THE FIRST.** [`corpus::Corpus`] and [`corpus::SkipReason`] are
/// how the rules are decided and compared; a row is what a column holds. Making
/// the row carry the enums would put a PARSE on the read path -- and a parse
/// has a failure mode, so the store would owe an answer for a value it wrote
/// itself. The survey produces rows, the store persists them, and the status
/// report reads them, with no conversion anywhere in between.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
  /// Relative to the project root, as the store holds a path.
  pub path: String,
  /// `canon`, `prose` or `code`.
  pub corpus: String,
  /// The language for a code row the map recognises; `None` otherwise.
  pub lang: Option<String>,
  pub size: u64,
  /// RFC3339, as `sync::stamp_of` spells it.
  pub mtime: String,
  /// What this row was last indexed AT. `None` where the file has not been
  /// read, which includes every skipped row.
  pub indexed_sha256: Option<String>,
  /// Why the index holds no content for this file; `None` when it holds it.
  pub skipped_reason: Option<String>,
}
