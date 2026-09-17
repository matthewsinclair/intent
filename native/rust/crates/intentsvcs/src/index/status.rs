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
use super::corpus::{Corpus, SkipReason};

/// What the index holds, and what it does not hold and why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Status {
  /// Files the index holds, by corpus, in the corpus's own spelling.
  pub held: BTreeMap<String, usize>,
  /// What this build can do for each language the project declares.
  ///
  /// **A LANGUAGE THAT NAMES NO SYMBOLS SAYS WHY.** Three different facts
  /// produce an empty structural answer -- no grammar compiled in, a grammar
  /// with no tags query, a language nothing supports -- and a reader who is
  /// told none of them concludes the index is broken or that their code has no
  /// definitions in it.
  pub grammars: BTreeMap<String, String>,
  /// Files the index does not hold, by reason, each with its paths in path
  /// order. A reason with nothing under it is not carried.
  pub skipped: BTreeMap<String, Vec<String>>,
  /// The measured size in bytes of each index table family and of the whole
  /// store (issue 0373). Read, never estimated; empty until the store fills it.
  pub sizes: BTreeMap<String, u64>,
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
/// The bucket a `code` row with no language is counted under.
///
/// **NOT A CORPUS AND DELIBERATELY NOT SPELLED LIKE ONE.** Nothing in the store
/// carries this word; it exists so a count a person reads means what the word
/// says.
pub const OTHER: &str = "other";

pub fn summarise(rows: &[Row]) -> Status {
  let mut held: BTreeMap<String, usize> = BTreeMap::new();
  let mut skipped: BTreeMap<String, Vec<String>> = BTreeMap::new();
  for row in rows {
    match &row.skipped_reason {
      Some(reason) => skipped
        .entry(reason.clone())
        .or_default()
        .push(row.path.clone()),
      // **`code` MEANS A FILE WHOSE LANGUAGE THIS BUILD KNOWS, AND EVERYTHING
      // ELSE IN THAT CORPUS IS REPORTED AS `other`** (vc, 2026-09-12, on ic's
      // end-to-end drive). The CORPUS is right and its note says why -- a file
      // the index cannot classify is still a file it must be able to name -- but
      // the WORD misleads a reader: a project with two source files was told
      // `code 5`, the other three being `.prettierignore`, a config file and an
      // `.intentfiles`. The store's column is untouched; this is the summary
      // saying what it counted.
      None => {
        let bucket = if row.corpus == (Corpus::Code { lang: None }).as_str() && row.lang.is_none() {
          OTHER
        } else {
          row.corpus.as_str()
        };
        *held.entry(bucket.to_string()).or_default() += 1;
      }
    }
  }
  for paths in skipped.values_mut() {
    paths.sort();
  }
  Status {
    held,
    grammars: BTreeMap::new(),
    skipped,
    sizes: BTreeMap::new(),
  }
}

/// What this build can do for each language a project declares.
///
/// **PURE, AND A FUNCTION OF THE BUILD RATHER THAN OF THE STORE.** Nothing is
/// recorded about a grammar: whether one is compiled in is a fact about this
/// binary, so a status read back from the store and one returned by a rebuild
/// give the same answer, which is the property that lets the two renderings be
/// compared.
pub fn grammars(declared: &[String]) -> BTreeMap<String, String> {
  declared
    .iter()
    .map(|lang| {
      (
        lang.clone(),
        super::symbols::readiness(lang).as_str().to_string(),
      )
    })
    .collect()
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
      // **THE FIXTURE CARRIES THE LANGUAGE IT WAS HANDED.** It discarded it and
      // wrote `None` on every row, which was invisible while nothing read the
      // field -- and the moment `summarise` split `code` from `other` by it, a
      // row built as `Code { lang: Some("rust") }` counted as a file whose
      // language nothing knows. A fixture that quietly drops an argument tests
      // a shape the caller cannot produce.
      lang: match &corpus {
        Corpus::Code { lang } => lang.map(str::to_string),
        _ => None,
      },
      size: 0,
      mtime: String::new(),
      indexed_sha256: None,
      skipped_reason: skipped.map(|r| r.as_str().to_string()),
      symbols_version: None,
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
      !status.skipped.contains_key("symlink"),
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
