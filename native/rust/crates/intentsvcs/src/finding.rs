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
  /// The project's canon is not in a form this binary can read -- v2 canon
  /// not yet migrated, or a config declaring a pre-v3 version. FIRST, because
  /// every other finding on such a project is downstream of this one and
  /// would send the operator after the wrong thing.
  Unmigrated,
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
  /// The canon parses and validates, but says two things that cannot both be
  /// true -- an acceptance test covering a criterion that does not exist, a
  /// completed thread with no completion date. The schema cannot catch these:
  /// every one of them is individually well-formed, and only the RELATIONSHIP
  /// is wrong. This is what `doctor`'s model half reports (AC-06.2).
  ModelInconsistent,
}

impl FindingClass {
  /// Rank, wire spelling and REMEDY, from ONE exhaustive match.
  ///
  /// Exhaustive because the compiler must refuse a new variant that forgets
  /// any of them -- an omission here is a class that reports under the wrong
  /// name, sorts arbitrarily, or tells an operator nothing to do, and none of
  /// the three announces itself.
  ///
  /// **The remedy is carried because `doctor --fix` was WITHDRAWN** (hv,
  /// 2026-08-15), and the ruling generalises past that flag: a diagnostic that
  /// NAMES the exact remedy is strictly better than one that performs it. The
  /// operator sees what will happen, decides whether it is what they meant,
  /// and keeps the blast radius in their own hands. A repair verb claims the
  /// tool understands the fault well enough to act unattended; a named remedy
  /// claims only that it understands it well enough to describe it -- and the
  /// second is the claim `doctor` can actually make.
  ///
  /// Two rules bind every string below. **No remedy proposes an operation
  /// whose blast radius exceeds the fault it repairs** (vc, 2026-08-15), which
  /// is why none of them reaches for `sync --to-store`: it replaces the whole
  /// store, and `event_log` is durable truth no file can reconstruct. And **no
  /// remedy names deleting the store** (D36) -- it is the source of truth, not
  /// a cache.
  fn meta(self) -> (u8, &'static str, &'static str) {
    match self {
      // The detail already carries `Migration::remedy()`, which names the
      // version and the command. This says the part that is true of the class:
      // nothing else is worth reading until it is done.
      Self::Unmigrated => (
        0,
        "unmigrated",
        "migrate the project first -- every other finding on it is downstream of this one",
      ),
      Self::MalformedJson => (
        1,
        "malformed-json",
        "repair the file's JSON, or restore that one file from version control",
      ),
      Self::SchemaInvalid => (
        2,
        "schema-invalid",
        "correct the field named above; `intent schema` prints the shape the file must match",
      ),
      Self::ConflictMarkers => (
        3,
        "conflict-markers",
        "finish the merge in the named file -- Intent will not read around a conflict marker",
      ),
      Self::UnknownFileShape => (
        4,
        "unknown-file-shape",
        "move or rename it -- a modelled directory carries only the artefacts Intent writes",
      ),
      Self::DuplicateId => (
        5,
        "duplicate-id",
        "two artefacts claim one id; rename or remove one of them",
      ),
      // The one remedy that is a command, and it is bounded on purpose: it
      // rewrites artefacts that are re-creatable from the store by
      // definition, so nothing authored is at risk. It says what it costs
      // anyway, because the finding exists BECAUSE someone hand-edited the
      // view, and regenerating is precisely what discards that edit.
      Self::ViewSkew => (
        6,
        "view-skew",
        "`intent sync --to-disk` regenerates the views from the store, DISCARDING the hand edit -- copy anything you meant to keep out first",
      ),
      Self::ModelInconsistent => (
        7,
        "model-inconsistent",
        "the canon says two things that cannot both be true; correct the artefact named above",
      ),
    }
  }

  /// What an operator should do about it, in words they can act on.
  pub fn remedy(&self) -> &'static str {
    self.meta().2
  }

  /// The wire spelling. Asserted against serde's by test rather than routed
  /// through it: the return is `&'static str`, and serde's is an owned
  /// `String`, so the two cannot be the same function. The test is what makes
  /// this a single authority in practice.
  pub fn as_str(&self) -> &'static str {
    self.meta().1
  }

  /// Declaration order, for a stable totals line.
  fn rank(&self) -> u8 {
    self.meta().0
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
    write!(f, " -- {} -- {}", self.class.as_str(), self.detail)?;
    // The two-line refusal grammar the rest of the estate uses: what is wrong,
    // then what to do about it. `doctor --fix` is withdrawn, so this line is
    // the whole of the tool's repair offer -- and it has to be runnable.
    write!(f, "\n  remedy: {}", self.class.remedy())
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
  ///
  /// **Counted from the findings present, never from a list of classes.** The
  /// list version was a hand-maintained array the compiler could not check, so
  /// a class added to the enum would simply never appear in the totals line --
  /// a silent undercount, in the function that exists to honour the
  /// no-silent-caps rule. Ordering comes from [`FindingClass::rank`], which is
  /// an exhaustive match, so a new variant cannot be silently dropped OR
  /// silently unordered.
  pub fn totals(&self) -> Vec<(FindingClass, usize)> {
    let mut out: Vec<(FindingClass, usize)> = Vec::new();
    for finding in &self.findings {
      match out.iter_mut().find(|(c, _)| *c == finding.class) {
        Some((_, n)) => *n += 1,
        None => out.push((finding.class, 1)),
      }
    }
    out.sort_by_key(|(c, _)| c.rank());
    out
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
