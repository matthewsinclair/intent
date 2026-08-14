//! Findings -- the one shape in which intentsvcs refuses.
//!
//! Every refusal in v3 names the artefact and the reason, in one grammar, so a
//! caller never has to tell "the tool said no" from "the tool said nothing".
//! The line format is migration.md's residue line, generalised from migration
//! to every refusing path:
//!
//! ```text
//! residue: <file>:<line> -- <class> -- <detail>
//! ```
//!
//! The class vocabulary is deliberately closed. A new refusal reason is a new
//! variant here, reviewed once, rather than a new string spelled slightly
//! differently at each site -- which is how v2 ended up with the same failure
//! reported five ways (the 0023 voice sweep).

use std::fmt;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Why an artefact was refused. Closed vocabulary; migration.md's residue
/// classes plus the two WP-03 adds ([`ViewSkew`], [`MalformedJson`]).
///
/// [`ViewSkew`]: FindingClass::ViewSkew
/// [`MalformedJson`]: FindingClass::MalformedJson
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum FindingClass {
  /// Not parseable as JSON at all.
  MalformedJson,
  /// Parses as JSON; violates the schema. Includes the D05 unknown-field
  /// refusal -- an unknown field is named, never dropped.
  SchemaInvalid,
  /// Git conflict markers present in an artefact. v2 grepped straight through
  /// these; v3 refuses (design.md).
  ConflictMarkers,
  /// A file in a modelled location the parser cannot classify.
  UnknownFileShape,
  /// Two artefacts claiming one natural id (the 0011 class).
  DuplicateId,
  /// A generated view on disk differs from what the model renders -- a
  /// hand-edit that would otherwise be silently overwritten, or silently
  /// believed.
  ViewSkew,
}

impl FindingClass {
  /// The wire spelling, via serde -- the one naming authority, exactly as
  /// [`crate::model::enum_str`] establishes for the model enums.
  pub fn as_str(&self) -> &'static str {
    match self {
      Self::MalformedJson => "malformed-json",
      Self::SchemaInvalid => "schema-invalid",
      Self::ConflictMarkers => "conflict-markers",
      Self::UnknownFileShape => "unknown-file-shape",
      Self::DuplicateId => "duplicate-id",
      Self::ViewSkew => "view-skew",
    }
  }
}

/// One refusal: what was refused, where, and why.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Finding {
  /// Project-relative path of the offending artefact.
  pub file: String,
  /// 1-indexed line, where the class can point at one.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub line: Option<u32>,
  pub class: FindingClass,
  /// Human-actionable detail. Names the specific thing -- the unknown field,
  /// the duplicate id -- never just restates the class.
  pub detail: String,
}

impl Finding {
  pub fn new(file: impl Into<String>, class: FindingClass, detail: impl Into<String>) -> Self {
    Self {
      file: file.into(),
      line: None,
      class,
      detail: detail.into(),
    }
  }

  pub fn at_line(mut self, line: u32) -> Self {
    self.line = Some(line);
    self
  }
}

impl fmt::Display for Finding {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "residue: {}", self.file)?;
    if let Some(line) = self.line {
      write!(f, ":{line}")?;
    }
    write!(f, " -- {} -- {}", self.class.as_str(), self.detail)
  }
}

/// A refusal carrying every finding, never only the first.
///
/// The report never truncates (migration.md's no-silent-caps rule): a capped
/// list reads as complete when it is not, which sends the reader round the
/// fix-and-rerun loop once per finding instead of once.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub struct Refusal {
  pub findings: Vec<Finding>,
}

impl Refusal {
  pub fn new(findings: Vec<Finding>) -> Self {
    Self { findings }
  }

  /// Count per class, in the class's declaration order -- the per-class totals
  /// migration.md's report prints.
  pub fn totals(&self) -> Vec<(FindingClass, usize)> {
    let classes = [
      FindingClass::MalformedJson,
      FindingClass::SchemaInvalid,
      FindingClass::ConflictMarkers,
      FindingClass::UnknownFileShape,
      FindingClass::DuplicateId,
      FindingClass::ViewSkew,
    ];
    classes
      .into_iter()
      .filter_map(|c| {
        let n = self.findings.iter().filter(|f| f.class == c).count();
        (n > 0).then_some((c, n))
      })
      .collect()
  }
}

impl fmt::Display for Refusal {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for finding in &self.findings {
      writeln!(f, "{finding}")?;
    }
    let totals = self
      .totals()
      .into_iter()
      .map(|(c, n)| format!("{}: {n}", c.as_str()))
      .collect::<Vec<_>>()
      .join(", ");
    write!(
      f,
      "error: refused {} finding(s) -- {totals}",
      self.findings.len()
    )
  }
}
