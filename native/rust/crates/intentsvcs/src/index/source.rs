//! Code as rows: what the source table holds for one file.
//!
//! **PURE. IT TAKES TEXT AND RETURNS ROWS** (IN-AG-PFIC-001). Reading the file
//! is `reconcile`'s, and the store is the store's.
//!
//! # One row per file, and that is the design's T1 rather than a simplification
//!
//! The lexical tier indexes a code file whole: there is no splitter for code
//! that does not need a grammar, and inventing one -- fixed line windows, or a
//! brace heuristic -- would put a chunking scheme in the way of the structural
//! tier that replaces it. T2 adds the symbol rows beside these, with the spans
//! a grammar can actually give; `kind` and `name` stay empty here because a
//! lexical pass has nothing true to put in them, and an empty column a later
//! pass fills is honest where a guessed one is not.

/// One `src_sections` row.
///
/// **`name_parts` IS THE CAMEL-CASED NAME BROKEN UP, AND IT IS A COLUMN RATHER
/// THAN A TOKENISER SETTING.** `unicode61` splits `snake_case` into its words
/// already, so `disabled` finds `parse_disabled`; it does not split
/// `CamelCase`, which stays one token and is reached by a prefix search. The
/// words inside a camel-cased name are searchable because something puts them
/// in this column, and for a whole-file row there is no name to break up.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Section {
  /// Project-relative path.
  pub path: String,
  /// 0-indexed position within the file's rows. `0` is the whole file at T1.
  pub seq: u32,
  /// 1-indexed, inclusive.
  pub start_line: u32,
  /// 1-indexed, inclusive. Equal to `start_line` for a one-line file, and `0`
  /// for an empty one, which has no lines rather than one empty line.
  pub end_line: u32,
  /// What the structural tier calls this row -- `fn`, `struct`, and so on.
  /// Empty for a whole-file row.
  pub kind: String,
  /// The symbol's name. Empty for a whole-file row.
  pub name: String,
  /// [`Section::name`] broken into its words. Empty for a whole-file row.
  pub name_parts: String,
  /// The bytes indexed, verbatim.
  pub body: String,
}

/// The whole file as one row.
pub fn whole_file(path: &str, text: &str) -> Section {
  Section {
    path: path.to_string(),
    seq: 0,
    start_line: 1,
    // **LINES, NOT NEWLINES.** A file with no trailing newline still has a last
    // line, and an empty file has no lines at all rather than one empty one --
    // reporting `1` there would make a span that names nothing.
    end_line: u32::try_from(text.lines().count()).unwrap_or(u32::MAX),
    kind: String::new(),
    name: String::new(),
    name_parts: String::new(),
    body: text.to_string(),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_whole_file_row_spans_the_lines_the_file_has() {
    assert_eq!(whole_file("a.rs", "fn a() {}\nfn b() {}\n").end_line, 2);
    assert_eq!(
      whole_file("a.rs", "fn a() {}").end_line,
      1,
      "a file with no trailing newline still has its last line"
    );
    assert_eq!(
      whole_file("a.rs", "").end_line,
      0,
      "and an empty file has no lines, rather than one empty line that a span \
       would name and a reader would go looking for"
    );
  }

  #[test]
  fn the_body_is_the_bytes_and_nothing_is_trimmed() {
    let text = "\n  indented\n\nfn a() {}\n";
    assert_eq!(
      whole_file("a.rs", text).body,
      text,
      "the indexed body is the file byte for byte, which is what lets a hit \
       claim a line at all"
    );
  }
}
