//! Prose ingest (data-model.md `doc_section`).
//!
//! Authored markdown bodies are stored VERBATIM and never modelled. The
//! splitter's only job is to give full-text search an address to return: a
//! section per heading, in document order, each carrying the bytes that sat
//! under that heading. Nothing here interprets, normalises or reflows -- a
//! body that goes in comes out identical, which is the property AC-03.6 pins.
//!
//! Fenced code blocks are respected: a `#` inside a fence is content, not a
//! heading. That is not a nicety -- Intent's own prose is full of shell and
//! markdown samples, and a splitter that broke on them would silently shred
//! the bodies it exists to preserve.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// One addressable chunk of authored prose.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DocSection {
  /// The entity the file belongs to, eg `thread`, `issue`, `project`.
  pub owner_type: String,
  /// That entity's natural id, eg `ST0056`.
  pub owner_id: String,
  /// Project-relative path.
  pub file: String,
  /// 0-indexed position in the file; `0` is the preamble above the first
  /// heading, which exists whether or not it has content.
  pub seq: u32,
  /// The heading text, without its `#` marks. Absent for the preamble.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub heading: Option<String>,
  /// Heading depth, 1-6. `0` for the preamble.
  pub level: u8,
  /// The bytes under this heading, verbatim, excluding the heading line.
  pub body: String,
}

/// Split one markdown document into sections.
///
/// The concatenation law, which the round-trip test pins: rebuilding the file
/// by emitting each section's heading line followed by its body reproduces the
/// input byte for byte.
pub fn split(owner_type: &str, owner_id: &str, file: &str, text: &str) -> Vec<DocSection> {
  let mut sections = Vec::new();
  let mut heading: Option<(String, u8)> = None;
  let mut body = String::new();
  let mut in_fence: Option<String> = None;

  for line in text.split_inclusive('\n') {
    let trimmed = line.trim_end_matches(['\n', '\r']);

    // Fence tracking first: a heading inside a fence is content.
    match &in_fence {
      Some(marker) if trimmed.trim_start().starts_with(marker.as_str()) => in_fence = None,
      Some(_) => {}
      None => {
        let lead = trimmed.trim_start();
        if lead.starts_with("```") || lead.starts_with("~~~") {
          in_fence = Some(lead.chars().take(3).collect());
        }
      }
    }

    if in_fence.is_none()
      && let Some((text, level)) = parse_heading(trimmed)
    {
      sections.push(build(
        owner_type,
        owner_id,
        file,
        sections.len() as u32,
        heading.take(),
        std::mem::take(&mut body),
      ));
      heading = Some((text, level));
      continue;
    }
    body.push_str(line);
  }
  sections.push(build(
    owner_type,
    owner_id,
    file,
    sections.len() as u32,
    heading,
    body,
  ));
  sections
}

/// Rebuild a document from its sections -- the inverse of [`split`], and the
/// half that makes "verbatim" checkable rather than merely claimed.
pub fn join(sections: &[DocSection]) -> String {
  let mut out = String::new();
  for s in sections {
    if let Some(heading) = &s.heading {
      out.push_str(&"#".repeat(s.level as usize));
      out.push(' ');
      out.push_str(heading);
      out.push('\n');
    }
    out.push_str(&s.body);
  }
  out
}

/// An ATX heading: 1-6 `#` then a space. `#hashtag` is not a heading, and
/// neither is a seventh `#`.
fn parse_heading(line: &str) -> Option<(String, u8)> {
  let hashes = line.len() - line.trim_start_matches('#').len();
  if hashes == 0 || hashes > 6 {
    return None;
  }
  let rest = &line[hashes..];
  let text = rest.strip_prefix(' ')?;
  Some((text.to_string(), hashes as u8))
}

fn build(
  owner_type: &str,
  owner_id: &str,
  file: &str,
  seq: u32,
  heading: Option<(String, u8)>,
  body: String,
) -> DocSection {
  let (heading, level) = match heading {
    Some((text, level)) => (Some(text), level),
    None => (None, 0),
  };
  DocSection {
    owner_type: owner_type.to_string(),
    owner_id: owner_id.to_string(),
    file: file.to_string(),
    seq,
    heading,
    level,
    body,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn split_then_join_is_identity() {
    let text = "preamble\n\n# Title\n\nbody one\n\n## Sub\n\nbody two\n";
    let sections = split("thread", "ST0056", "design.md", text);
    assert_eq!(join(&sections), text);
    assert_eq!(sections.len(), 3);
    assert_eq!(sections[0].heading, None);
    assert_eq!(sections[1].heading.as_deref(), Some("Title"));
    assert_eq!(sections[2].level, 2);
  }

  #[test]
  fn a_hash_inside_a_fence_is_content_not_a_heading() {
    let text = "# Real\n\n```sh\n# not a heading\n```\n\nafter\n";
    let sections = split("thread", "ST0056", "design.md", text);
    assert_eq!(join(&sections), text);
    assert_eq!(
      sections.len(),
      2,
      "the fenced comment must not open a section"
    );
  }

  #[test]
  fn a_file_with_no_trailing_newline_round_trips() {
    let text = "# Title\nno trailing newline";
    let sections = split("thread", "ST0056", "impl.md", text);
    assert_eq!(join(&sections), text);
  }

  #[test]
  fn seven_hashes_is_not_a_heading() {
    let text = "####### seven\n";
    let sections = split("thread", "ST0056", "impl.md", text);
    assert_eq!(sections.len(), 1);
    assert_eq!(sections[0].heading, None);
    assert_eq!(join(&sections), text);
  }
}
