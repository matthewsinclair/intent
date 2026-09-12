//! What `intent index status` answers, computed from rows.
//!
//! **PURE. IT WALKS NOTHING AND OPENS NOTHING** (IN-AG-PFIC-001). The rows
//! arrive from a survey or from the store; this decides what a reader is told
//! about them, so the same summary can be driven on values and cannot differ
//! between the two sources.
//!
//! # Why the skipped files are listed and not counted
//!
//! AC-18.2 says nothing is skipped silently, and a COUNT is silence with a
//! number on it: an operator who reads `3 skipped` still cannot tell whether
//! the file they are looking for is one of them. The paths are the answer; the
//! count is a property of the list a reader can see for themselves.

use std::collections::BTreeMap;

use super::Row;
use super::corpus::SkipReason;

/// What the index holds, and what it does not hold and why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Status {
  /// Files the index holds, by corpus, in the corpus's own spelling.
  pub held: BTreeMap<String, usize>,
  /// Files the index does not hold, by reason, each with its paths in path
  /// order. A reason with nothing under it is not carried.
  pub skipped: BTreeMap<String, Vec<String>>,
}

impl Status {
  /// Does this project's index hold nothing at all?
  ///
  /// **A SEPARATE QUESTION FROM "NOTHING IS SKIPPED", and a reader needs both.**
  /// An index that holds nothing and skipped nothing has not been built; one
  /// that holds nothing and skipped everything is a project of binaries.
  pub fn is_empty(&self) -> bool {
    self.held.values().all(|n| *n == 0)
  }
}

/// Summarise rows into the answer.
///
/// **A SKIPPED FILE IS NOT COUNTED AS HELD**, which is the one arithmetic
/// mistake available here: the corpus it belongs to is known for a skipped file
/// too, so adding it to that corpus's tally would report an index holding
/// content it has never read.
pub fn summarise(rows: &[Row]) -> Status {
  let mut held: BTreeMap<String, usize> = BTreeMap::new();
  let mut skipped: BTreeMap<String, Vec<String>> = BTreeMap::new();
  for row in rows {
    match &row.skipped_reason {
      Some(reason) => skipped
        .entry(reason.clone())
        .or_default()
        .push(row.path.clone()),
      None => *held.entry(row.corpus.clone()).or_default() += 1,
    }
  }
  for paths in skipped.values_mut() {
    paths.sort();
  }
  Status { held, skipped }
}

/// Every reason, in the order a report lists them, so that a reason with
/// nothing under it can be shown as absent rather than forgotten.
pub const REASONS: &[SkipReason] = &[
  SkipReason::Binary,
  SkipReason::TooLarge,
  SkipReason::Symlink,
  SkipReason::Unreadable,
];

#[cfg(test)]
mod tests {
  use super::super::corpus::Corpus;
  use super::*;

  fn row(path: &str, corpus: Corpus, skipped: Option<SkipReason>) -> Row {
    Row {
      path: path.to_string(),
      corpus: corpus.as_str().to_string(),
      lang: None,
      size: 0,
      mtime: String::new(),
      indexed_sha256: None,
      skipped_reason: skipped.map(|r| r.as_str().to_string()),
    }
  }

  #[test]
  fn a_skipped_file_is_listed_by_reason_and_not_counted_as_held() {
    let rows = vec![
      row("README.md", Corpus::Prose, None),
      row("docs/guide.md", Corpus::Prose, None),
      row("src/lib.rs", Corpus::Code { lang: Some("rust") }, None),
      row(
        "assets/logo.bin",
        Corpus::Code { lang: None },
        Some(SkipReason::Binary),
      ),
      row(
        "vendor/huge.json",
        Corpus::Code { lang: None },
        Some(SkipReason::TooLarge),
      ),
    ];

    let status = summarise(&rows);

    assert_eq!(status.held.get("prose"), Some(&2));
    assert_eq!(
      status.held.get("code"),
      Some(&1),
      "the binary and the oversized file belong to the code corpus and are NOT \
       held -- counting them would report an index holding content it never read"
    );
    assert_eq!(
      status.skipped.get("binary"),
      Some(&vec!["assets/logo.bin".to_string()]),
      "the PATHS are the answer: `1 skipped` is silence with a number on it"
    );
    assert_eq!(
      status.skipped.get("too-large"),
      Some(&vec!["vendor/huge.json".to_string()])
    );
    assert!(
      status.skipped.get("symlink").is_none(),
      "a reason with nothing under it is absent"
    );
    assert!(!status.is_empty());
  }

  #[test]
  fn an_index_that_holds_nothing_says_so_separately_from_skipping_nothing() {
    assert!(summarise(&[]).is_empty());
    let all_skipped = vec![row(
      "a.bin",
      Corpus::Code { lang: None },
      Some(SkipReason::Binary),
    )];
    let status = summarise(&all_skipped);
    assert!(
      status.is_empty(),
      "an index that holds nothing and skipped everything is a project of \
       binaries, and a reader needs to be able to tell that from one that was \
       never built"
    );
    assert_eq!(status.skipped.get("binary").map(Vec::len), Some(1));
  }

  #[test]
  fn the_listed_paths_are_in_path_order() {
    let rows = vec![
      row(
        "b.bin",
        Corpus::Code { lang: None },
        Some(SkipReason::Binary),
      ),
      row(
        "a.bin",
        Corpus::Code { lang: None },
        Some(SkipReason::Binary),
      ),
    ];
    assert_eq!(
      summarise(&rows).skipped.get("binary"),
      Some(&vec!["a.bin".to_string(), "b.bin".to_string()]),
      "a report whose order varies by filesystem cannot be compared between two \
       runs, which is the first thing anyone does with it"
    );
  }
}
