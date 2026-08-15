//! The sync engine's change detection -- a git-style index over the project's
//! files (data-model.md `file_index`).
//!
//! **The hash is computed on every scan, and stat is never a gate on it.**
//! design.md sketched "stat scan (mtime/size, SHA-256 rehash on change)", but
//! AC-03.3 requires detecting a same-size same-mtime rewrite, which no amount
//! of stat comparison can see: the whole point of that case is that stat is
//! unchanged while content is not. Where the contract and the architecture
//! narrative disagree, the contract governs (vc ruling, 2026-08-14) -- so size
//! and mtime are carried as reporting metadata, and content identity is
//! decided by SHA-256 alone.
//!
//! The cost is hashing a few hundred files per invocation. If that ever bites,
//! the optimisation is a recorded deviation with its own evidence, never an
//! accident that silently reintroduces the blind spot.

use std::io;
use std::path::{Path, PathBuf};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::finding::{Finding, FindingClass};

/// Root-level files inside the sync scope. Explicit and reviewed, never a
/// glob: the scope is "Intent's own artefacts", and a glob over the repo root
/// would sweep in whatever the project happens to keep there.
///
/// `.gitignore` is deliberately absent -- migration converges it, but nothing
/// ingests it, and a file in the index that no reader consumes is a claim the
/// tool cannot back.
pub const ROOT_FILES: &[&str] = &["AGENTS.md", "CLAUDE.md", "usage-rules.md"];

/// Paths under `intent/` that the scan does not walk.
///
/// `.cache/` holds the rebuildable DB (D21) -- indexing the index is circular.
/// `.treeindex/` is an untracked derived cache (issue 0018).
pub const SKIPPED_DIRS: &[&str] = &[".cache", ".treeindex", ".backup"];

/// What the scan concluded about one file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum FileState {
  /// Content identical to the last indexed state.
  Clean,
  /// Content differs from the last indexed state, or the file is new.
  Changed,
  /// The file is in a modelled location and cannot be read as what it claims
  /// to be. Commands needing it refuse (AC-03.5); nothing reads through it.
  Unparsed,
}

/// One row of the file index.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FileEntry {
  /// Project-relative, forward-slashed.
  pub path: String,
  /// Reporting metadata only -- never a gate on hashing. See the module note.
  pub size: u64,
  /// RFC 3339 UTC. Reporting metadata only.
  pub mtime: String,
  /// Lowercase hex SHA-256 of the file's bytes. The sole identity test.
  pub sha256: String,
  pub state: FileState,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub findings: Vec<Finding>,
}

#[derive(Debug, thiserror::Error)]
pub enum SyncError {
  #[error("reading {path}: {source}")]
  Io {
    path: String,
    #[source]
    source: io::Error,
  },
  #[error("formatting a timestamp: {0}")]
  Time(#[from] time::error::Format),
}

fn io_err(path: &Path, source: io::Error) -> SyncError {
  SyncError::Io {
    path: path.display().to_string(),
    source,
  }
}

/// Scan the project's sync scope, deciding each file's state against the
/// previously indexed entries.
///
/// `previous` is the stored index; pass an empty slice for a first scan, where
/// every file is [`FileState::Changed`] because nothing has been ingested yet.
/// Output is ordered by path, so two scans of one tree are comparable without
/// the caller sorting.
pub fn scan(root: &Path, previous: &[FileEntry]) -> Result<Vec<FileEntry>, SyncError> {
  let ignored = Ignored::for_root(root);

  let mut paths = Vec::new();
  for name in ROOT_FILES {
    let candidate = root.join(name);
    if candidate.is_file() && !ignored.contains(&candidate) {
      paths.push(candidate);
    }
  }
  let intent_dir = root.join("intent");
  if intent_dir.is_dir() {
    walk(&intent_dir, &ignored, &mut paths)?;
  }
  paths.sort();

  let mut entries = Vec::with_capacity(paths.len());
  for path in paths {
    entries.push(entry_for(root, &path, previous)?);
  }
  Ok(entries)
}

/// The set of paths git would never commit, which are therefore never canon.
///
/// **D29 / AC-03.7.** Ingest walks the filesystem; git does not. On macOS every
/// directory acquires a `.DS_Store`, `.gitignore` excludes them, and strict
/// ingest then correctly refused a corpus containing what it correctly could
/// not parse -- so `intent search` exited 1 having read nothing, on a clean
/// checkout, on every Mac. Because AC-10.2 makes residue a migration BLOCK,
/// that failure propagated to the fleet rollout's first step.
///
/// **D05 is not weakened; the CORPUS is defined.** The rule is derived from
/// D01 rather than picked to fit: durable truth is committed, schema-validated
/// JSON, so a path git can never commit can never be canon, and must never
/// produce residue or block a read.
///
/// **Not a `.DS_Store` special case, deliberately.** The same rule is already
/// load-bearing and currently held by luck: `intent/.cache/intent.db` escapes
/// today through path shape (`SKIPPED_DIRS`) rather than through any rule, and
/// WP-13 widens the corpus to the whole project for search, at which point a
/// binary SQLite file walks into scope. One rule now, or two bugs later.
///
/// Two edges, both of which are worse to get backwards than the original bug:
///
/// - It keys on IGNORED, never on untracked. A `thread.json` you just created
///   and have not committed must still ingest -- that is what most of a
///   working session looks like.
/// - A project with no git has no ignore rules, so nothing is ignored and the
///   corpus degrades to everything-in-scope rather than to nothing. That falls
///   out of the walker's `require_git` default rather than being special-cased.
struct Ignored {
  paths: std::collections::HashSet<PathBuf>,
}

impl Ignored {
  /// Enumerate the ignored paths under `root` by walking WITH ignore rules off
  /// and again with them on, and taking the difference.
  ///
  /// The walker reports what it keeps, not what it drops, so the set is
  /// derived rather than read off. Cheap enough here because the tree is the
  /// project's own and already being scanned.
  fn for_root(root: &Path) -> Self {
    let visible: std::collections::HashSet<PathBuf> = ignore::WalkBuilder::new(root)
      .hidden(false)
      .parents(true)
      .build()
      .filter_map(Result::ok)
      .map(|e| e.into_path())
      .collect();
    let all: std::collections::HashSet<PathBuf> = ignore::WalkBuilder::new(root)
      .hidden(false)
      .standard_filters(false)
      .build()
      .filter_map(Result::ok)
      .map(|e| e.into_path())
      .collect();
    Self {
      paths: all.difference(&visible).cloned().collect(),
    }
  }

  fn contains(&self, path: &Path) -> bool {
    self.paths.contains(path)
  }
}

/// Depth-first, name-ordered walk. Ordering is explicit because `read_dir`
/// order is filesystem-dependent, and an index whose row order varies by
/// machine cannot be compared across them.
fn walk(dir: &Path, ignored: &Ignored, out: &mut Vec<PathBuf>) -> Result<(), SyncError> {
  let mut children: Vec<PathBuf> = std::fs::read_dir(dir)
    .map_err(|e| io_err(dir, e))?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| io_err(dir, e))?
    .into_iter()
    .map(|e| e.path())
    .collect();
  children.sort();

  for child in children {
    let name = child
      .file_name()
      .map(|n| n.to_string_lossy().into_owned())
      .unwrap_or_default();
    if ignored.contains(&child) {
      continue;
    }
    if child.is_dir() {
      if SKIPPED_DIRS.contains(&name.as_str()) {
        continue;
      }
      walk(&child, ignored, out)?;
    } else if child.is_file() {
      out.push(child);
    }
  }
  Ok(())
}

fn entry_for(root: &Path, path: &Path, previous: &[FileEntry]) -> Result<FileEntry, SyncError> {
  let rel = crate::project::relative(root, path);
  let bytes = std::fs::read(path).map_err(|e| io_err(path, e))?;
  let meta = std::fs::metadata(path).map_err(|e| io_err(path, e))?;
  let sha256 = hex(Sha256::digest(&bytes).as_slice());

  let findings = inspect(&rel, &bytes);
  let state = if !findings.is_empty() {
    FileState::Unparsed
  } else if previous
    .iter()
    .any(|p| p.path == rel && p.sha256 == sha256 && p.state != FileState::Unparsed)
  {
    FileState::Clean
  } else {
    FileState::Changed
  };

  Ok(FileEntry {
    path: rel,
    size: meta.len(),
    mtime: OffsetDateTime::from(meta.modified().map_err(|e| io_err(path, e))?).format(&Rfc3339)?,
    sha256,
    state,
    findings,
  })
}

/// Everything that makes a file unreadable-as-what-it-claims-to-be.
fn inspect(rel: &str, bytes: &[u8]) -> Vec<Finding> {
  let Ok(text) = std::str::from_utf8(bytes) else {
    return vec![Finding::new(
      rel,
      FindingClass::UnknownFileShape,
      "not valid UTF-8",
    )];
  };

  let mut findings = conflict_markers(rel, text);
  if findings.is_empty()
    && rel.ends_with(".json")
    && let Err(e) = serde_json::from_str::<serde_json::Value>(text)
  {
    findings
      .push(Finding::new(rel, FindingClass::MalformedJson, e.to_string()).at_line(e.line() as u32));
  }
  findings
}

/// Git conflict markers.
///
/// Keyed on the `<<<<<<<` and `>>>>>>>` lines and requiring BOTH, never on the
/// `=======` divider: a run of `=` at the start of a line is also a setext H1
/// underline, which is ordinary markdown. Keying on the divider would have
/// reported Intent's own prose as conflicted.
fn conflict_markers(rel: &str, text: &str) -> Vec<Finding> {
  let mut opens = Vec::new();
  let mut closes = Vec::new();
  for (idx, line) in text.lines().enumerate() {
    let line_no = idx as u32 + 1;
    if is_marker(line, '<') {
      opens.push(line_no);
    } else if is_marker(line, '>') {
      closes.push(line_no);
    }
  }
  if opens.is_empty() || closes.is_empty() {
    return Vec::new();
  }
  opens
    .into_iter()
    .map(|line| {
      Finding::new(
        rel,
        FindingClass::ConflictMarkers,
        "git conflict markers present; resolve the merge before Intent can read this file",
      )
      .at_line(line)
    })
    .collect()
}

/// Exactly seven of `c`, then end-of-line or a space -- git's marker shape.
fn is_marker(line: &str, c: char) -> bool {
  let run = line.chars().take_while(|&ch| ch == c).count();
  run == 7 && line[run..].chars().next().is_none_or(|ch| ch == ' ')
}

fn hex(bytes: &[u8]) -> String {
  bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_setext_underline_is_not_a_conflict_marker() {
    let text = "Title\n=======\n\nbody\n";
    assert!(
      conflict_markers("intent/wip.md", text).is_empty(),
      "a run of = under a heading is markdown, not a merge conflict"
    );
  }

  #[test]
  fn an_open_marker_without_a_close_is_not_reported() {
    // A lone `<<<<<<<` line is far more likely to be prose about conflicts
    // than a conflict; requiring both ends is what makes the check safe to
    // run over documentation that discusses git.
    let text = "<<<<<<< HEAD\nno close marker\n";
    assert!(conflict_markers("intent/wip.md", text).is_empty());
  }

  #[test]
  fn a_real_conflict_is_reported_with_its_line() {
    let text = "a\n<<<<<<< HEAD\nours\n=======\ntheirs\n>>>>>>> branch\n";
    let found = conflict_markers("intent/wip.md", text);
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].line, Some(2));
    assert_eq!(found[0].class, FindingClass::ConflictMarkers);
  }

  #[test]
  fn eight_angle_brackets_is_not_a_marker() {
    assert!(!is_marker("<<<<<<<< eight", '<'));
    assert!(is_marker("<<<<<<< seven", '<'));
    assert!(is_marker("<<<<<<<", '<'));
  }
}
