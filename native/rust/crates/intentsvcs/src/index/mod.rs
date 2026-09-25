//! The search index: the scope rule, the reconciler, and what each answers.
//!
//! ST0069 WP-18 builds the first of these. The module tree is the estate's PFIC
//! shape -- a pure core and an impure rim -- and it is laid out here in full so
//! that a later package adds a file rather than moving one.

pub mod corpus;
pub mod elixir_tracer;
pub mod freshness;
pub mod reconcile;
pub mod resolved;
pub mod rust_analyzer;
pub mod scip;
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
  /// The [`symbols::EXTRACTOR_VERSION`] that wrote this file's symbols. `None`
  /// where no pass that reads files has recorded one, which includes every
  /// file an older extractor wrote.
  pub symbols_version: Option<i64>,
}

/// What one incremental pass did.
///
/// **PATHS RATHER THAN COUNTS**, for the reason `index status` lists its
/// skipped files: the caller of an incremental refresh is a watcher deciding
/// what to publish, and a number tells it nothing it can name to a subscriber.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Refreshed {
  /// Paths whose rows and content were rewritten.
  pub updated: Vec<String>,
  /// Paths that have left the index.
  pub removed: Vec<String>,
  /// The search table this pass found damaged after its delete and rebuilt in
  /// the same write, or `None` when fts5's check passed or nothing was deleted.
  pub repaired: Option<IndexRepair>,
}

/// A search table a scoped refresh found damaged and rebuilt in its own write.
///
/// **CARRIED TO EVERY DOOR THAT REFRESHES, NEVER SWALLOWED**
/// (`IN-AG-NO-SILENT-001`): a repair that ran is a fault that happened, and each
/// door says so in its own output. See `store::repair_if_damaged` for why a
/// secure delete can leave the table damaged.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct IndexRepair {
  /// The FTS5 table that was rebuilt.
  pub table: String,
  /// What the probes found before the rebuild, as `doctor` phrases it.
  pub found: String,
  /// The docids the index held with no content row before the rebuild.
  pub orphaned: Vec<i64>,
  /// What fts5's check still objected to after the rebuild, or `None` when the
  /// rebuild cleared it.
  pub remaining: Option<String>,
}

impl IndexRepair {
  pub fn new(found: &crate::doctor::SearchIndexReading, remaining: Option<String>) -> Self {
    Self {
      table: found.table.clone(),
      found: found.found(),
      orphaned: match &found.orphaned {
        crate::doctor::Orphans::Docids(docs) => docs.clone(),
        crate::doctor::Orphans::Unreadable(_) => Vec::new(),
      },
      remaining,
    }
  }

  /// The one sentence every door prints for this repair.
  pub fn sentence(&self) -> String {
    let orphaned = match self.orphaned.as_slice() {
      [] => String::new(),
      docs => format!(
        " (orphaned docid(s): {})",
        docs
          .iter()
          .map(i64::to_string)
          .collect::<Vec<_>>()
          .join(", ")
      ),
    };
    let head = format!(
      "the search index's `{}` was damaged -- {}{orphaned}",
      self.table, self.found
    );
    match &self.remaining {
      None => {
        format!("{head}; it was rebuilt in the write that found it, and fts5's check now passes")
      }
      Some(still) => format!(
        "{head}; it was rebuilt in the write that found it and fts5's check still objects ({still}) -- run `intent index rebuild`"
      ),
    }
  }
}

impl Refreshed {
  /// Did this pass change anything at all?
  pub fn is_empty(&self) -> bool {
    self.updated.is_empty() && self.removed.is_empty()
  }
}
