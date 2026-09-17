//! Issue 0434, under vc's decision 19 (2026-09-15): **every notice intentd
//! writes with a `remedy:` line opens with intentd's `warning: ` or `error: `
//! token, and says its remedy once.**
//!
//! The Console colours a log line by its token, so a notice without one is
//! coloured by guessing from its words or not coloured at all. A rendered error
//! already ends with its remedy, so a notice that renders the error and then
//! appends `e.remedy()` writes the same remedy line twice at one stamp.
//!
//! **A SOURCE SCAN, BECAUSE EVERY ONE OF THESE NOTICES IS A FAILURE PATH A
//! GREEN RUN NEVER WALKS.** Each fires only when a refresh, a survey, a
//! reconcile, an ingest or a registry parse fails. An arm per notice would
//! have to build each failure, and still could not see the next notice
//! somebody writes without a token, which is the class this closes.

use std::path::Path;

use intentsvcs::remedy::{ERROR_PREFIX, REMEDY_PREFIX};

use crate::common::{daemon_sources, without_comments};

/// intentd's warning token, as its notices spell it and the Console reads it.
/// The error token is the renderer's own, read from `intentsvcs::remedy`.
const WARNING_PREFIX: &str = "warning: ";

/// One `logln!` or `elogln!` call: the file it is in, its format string as
/// written in source, and the whole text of its arguments.
struct Notice {
  file: String,
  format: String,
  args: String,
}

impl Notice {
  /// It carries a remedy: in its own format string, or in a rendered error.
  fn carries_a_remedy(&self) -> bool {
    self.format.contains(REMEDY_PREFIX) || self.args.contains(".render()")
  }

  /// A call whose format string starts with the remedy token continues the
  /// notice the call before it opened, as `refuse` in main.rs does.
  fn continues_a_notice(&self) -> bool {
    self.format.starts_with(REMEDY_PREFIX)
  }
}

fn notices() -> Vec<Notice> {
  let mut out = Vec::new();
  for path in daemon_sources() {
    let body = without_comments(&path);
    let mut rest = body.as_str();
    // `elogln!(` contains `logln!(`, so one search finds both macros.
    while let Some(at) = rest.find("logln!(") {
      let after = &rest[at + "logln!(".len()..];
      let end = closing_paren(after);
      let args = &after[..end];
      if let Some(format) = first_string_literal(args) {
        out.push(Notice {
          file: file_name(&path),
          format,
          args: args.to_string(),
        });
      }
      rest = &after[end..];
    }
  }
  out
}

fn file_name(path: &Path) -> String {
  path
    .file_name()
    .map(|name| name.to_string_lossy().to_string())
    .unwrap_or_default()
}

/// The byte offset of the `)` that closes a macro call whose `(` was just
/// consumed, counting parentheses outside string literals only.
fn closing_paren(args: &str) -> usize {
  let mut depth = 1;
  let mut in_string = false;
  let mut escaped = false;
  for (i, c) in args.char_indices() {
    match (in_string, escaped, c) {
      (true, true, _) => escaped = false,
      (true, false, '\\') => escaped = true,
      (true, false, '"') => in_string = false,
      (true, false, _) => {}
      (false, _, '"') => in_string = true,
      (false, _, '(') => depth += 1,
      (false, _, ')') => {
        depth -= 1;
        if depth == 0 {
          return i;
        }
      }
      (false, _, _) => {}
    }
  }
  args.len()
}

/// The first string literal's text as written in source, escapes included.
fn first_string_literal(args: &str) -> Option<String> {
  let start = args.find('"')? + 1;
  let mut escaped = false;
  for (i, c) in args[start..].char_indices() {
    match (escaped, c) {
      (true, _) => escaped = false,
      (false, '\\') => escaped = true,
      (false, '"') => return Some(args[start..start + i].to_string()),
      (false, _) => {}
    }
  }
  None
}

#[test]
fn every_notice_with_a_remedy_opens_with_a_severity_token() {
  let heads: Vec<Notice> = notices()
    .into_iter()
    .filter(|n| n.carries_a_remedy() && !n.continues_a_notice())
    .collect();
  assert!(
    !heads.is_empty(),
    "the scan found no notice carrying a remedy under crates/intentd/src, so it measured nothing"
  );
  let untokened: Vec<String> = heads
    .iter()
    .filter(|n| !n.format.starts_with(ERROR_PREFIX) && !n.format.starts_with(WARNING_PREFIX))
    .map(|n| format!("{}: {}", n.file, n.format))
    .collect();
  assert!(
    untokened.is_empty(),
    "decision 19: a notice with a remedy opens with `{WARNING_PREFIX}` or `{ERROR_PREFIX}`, so the Console colours it by its token; these do not:\n{}",
    untokened.join("\n")
  );
}

#[test]
fn a_rendered_error_is_not_given_its_remedy_again() {
  let twice: Vec<String> = notices()
    .into_iter()
    .filter(|n| n.args.contains(".render()"))
    .filter(|n| n.format.contains(REMEDY_PREFIX) || n.args.contains(".remedy()"))
    .map(|n| format!("{}: {}", n.file, n.format))
    .collect();
  assert!(
    twice.is_empty(),
    "a rendered error already ends with its remedy line, so a notice that also writes the remedy says it twice at one stamp:\n{}",
    twice.join("\n")
  );
}
