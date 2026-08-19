//! The `ROOT_FILES` generator (ST0057 AC-00.4).
//!
//! `AGENTS.md`, `CLAUDE.md` and `usage-rules.md` sit at the project root, are
//! outside `.intentfiles` scope, and **something emptied `AGENTS.md` on
//! 2026-08-18** -- so until this module existed their derivability was an
//! assumption rather than a mechanism. The criterion asks for the mechanism.
//!
//! # Three deliberate nots
//!
//! **NOT a [`crate::views::View`].** A `GeneratedView` under a thread
//! DEHYDRATES; root files escape `organize` only because `thread_relative`
//! answers `None` for them. Joining `views::render_all` would put three root
//! files one classifier change away from removal -- and `AGENTS.md` is the file
//! this criterion exists because something already emptied it. That is a hazard
//! rather than a preference.
//!
//! **NOT prose in Rust.** The project's own rule is that all generated content
//! comes from `lib/templates/` via substitution, with no inline heredoc
//! duplicating a template. v2's generator predates the rule and carries its
//! sections inline across 835 lines of `intent/plugins/agents/bin/intent_agents`.
//! Porting that into a binary would be wrong twice: it bakes v2's shipped-and-
//! frozen content into v3, and it bakes THIS project's content into a tool every
//! consumer installs.
//!
//! **NO `[[DATE]]`, and it is not an oversight.** [`RenderContext`] is
//! documented as carrying facts about the tool or the project's data and
//! "never about the moment of rendering". A generated file that stamps the
//! moment it was generated **differs from itself on every run**, which is
//! AC-03.17's churn loop with a date in it. D42 says the same thing from the
//! other side: nothing here asks what time it is. The token is therefore
//! REFUSED rather than ignored -- see [`Fault::UnknownToken`] -- because a
//! silently-passed-through `[[DATE]]` would ship to a reader as literal text.
//!
//! # The split
//!
//! [`substitute`] is pure and [`render_all`] does the I/O, the same way
//! `install.rs` splits `resolve` from `home`. That is what lets the
//! language-conditional blocks have real tests instead of one test of whatever
//! tree the suite happens to run in.

use crate::project::Config;
use crate::views::{RenderContext, View};
use std::path::{Path, PathBuf};

/// What went wrong reading or expanding a root-file template.
#[derive(Debug, thiserror::Error)]
pub enum RootFileError {
  #[error("cannot read the template {path}")]
  Unreadable {
    path: String,
    #[source]
    source: std::io::Error,
  },
  #[error("the template {path} is malformed: {fault}")]
  Malformed { path: String, fault: Fault },
  #[error("cannot write {path}")]
  Unwritable {
    path: String,
    #[source]
    source: crate::write_set::WriteError,
  },
}

impl crate::remedy::Remedy for RootFileError {
  fn remedy(&self) -> String {
    match self {
      Self::Unreadable { .. } => {
        "reinstall Intent -- this binary is running against an install whose templates are missing"
          .to_string()
      }
      Self::Malformed { .. } => {
        "fix the template in the Intent install -- the generated file is not written from a template this binary cannot expand".to_string()
      }
      // The rendered document was fine and the disk refused it, so the remedy
      // is about the path rather than about the template. Naming the template
      // here would send the reader to the one place that is working.
      Self::Unwritable { .. } => {
        "check the project root is writable and has space -- the file was rendered and could not be put down".to_string()
      }
    }
  }
}

impl crate::remedy::Remedy for Fault {
  /// **One remedy for all four, and it names the TEMPLATE rather than the
  /// project.** Every variant here is a defect in a file that ships with the
  /// Intent install, so the person who can act on it is whoever maintains that
  /// template -- and telling a project's operator to fix their own tree would
  /// send them looking in the one place the fault is not.
  fn remedy(&self) -> String {
    "fix the root-file template in the Intent install -- the message above names the exact marker or token at fault".to_string()
  }
}

/// A defect in the template text itself, found while expanding it.
///
/// **Every one of these REFUSES rather than degrading.** A template that
/// silently passes an unrecognised `[[TOKEN]]` through ships the literal token
/// to whoever reads the generated file, and the person best placed to notice --
/// the one running the generator -- is the one who never sees it.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Fault {
  #[error("`[[{0}]]` is not a token this generator substitutes")]
  UnknownToken(String),
  #[error("`{0}` was opened and never closed")]
  Unclosed(String),
  #[error("`{0}` closes a block that was never opened")]
  Unopened(String),
  #[error("`{0}` opens a block inside `{1}`, and these blocks do not nest")]
  Nested(String, String),
}

/// Where a root file's template lives, given the install root.
///
/// The mapping is DERIVED (`_` + the file's own name), never a second roster.
/// [`crate::sync::ROOT_FILES`] is the one list of which files these are, and a
/// table here pairing each name with its template would be that list wearing a
/// different constant -- with nothing to notice on the day the two disagree.
pub fn template_path(home: &Path, name: &str) -> PathBuf {
  home
    .join(crate::install::MARKER)
    .join("llm")
    .join(format!("_{name}"))
}

/// Expand one template. Pure: every input is an argument.
///
/// Blocks are resolved before tokens, so a token inside a dropped block is
/// never substituted and never has to be substitutable.
pub fn substitute(template: &str, cfg: &Config, ctx: &RenderContext<'_>) -> Result<String, Fault> {
  let kept = resolve_blocks(template, &cfg.languages)?;
  expand_tokens(&kept, cfg, ctx)
}

/// Render one root file by name.
pub fn render(
  home: &Path,
  name: &str,
  cfg: &Config,
  ctx: &RenderContext<'_>,
) -> Result<String, RootFileError> {
  let path = template_path(home, name);
  let template = std::fs::read_to_string(&path).map_err(|source| RootFileError::Unreadable {
    path: path.display().to_string(),
    source,
  })?;
  substitute(&template, cfg, ctx).map_err(|fault| RootFileError::Malformed {
    path: path.display().to_string(),
    fault,
  })
}

/// Render every root file, in [`crate::sync::ROOT_FILES`] order.
///
/// The `View`'s path is project-relative and bare -- these files live at the
/// root by definition, which is the whole reason `sync` names them separately
/// from its walk of `intent/`.
pub fn render_all(
  home: &Path,
  cfg: &Config,
  ctx: &RenderContext<'_>,
) -> Result<Vec<View>, RootFileError> {
  let mut out = Vec::with_capacity(crate::sync::ROOT_FILES.len());
  for name in crate::sync::ROOT_FILES {
    out.push(View {
      path: PathBuf::from(name),
      content: render(home, name, cfg, ctx)?,
    });
  }
  Ok(out)
}

/// Render one root file and put it on disk. The write half of `intent agents
/// sync`.
///
/// **THROUGH [`crate::write_set::WriteSet`], NOT `fs::write`, AND THAT IS THE
/// WHOLE OF THE MECHANISM CHOICE.** `views::write_all` once wrote with a bare
/// loop and became a divergent expression of the db-to-disk direction; the
/// consequence was a skip-when-unchanged guard that was correct and reached
/// nothing. One write path means this file gets the rollback and the mtime skip
/// for free -- and the skip matters here more than most, because a root file
/// whose bytes did not change must not move its mtime and wake every watcher in
/// the tree.
///
/// **NO `.bak` SIBLING, AND THE DEVIATION IS RATIFIED RATHER THAN OVERLOOKED**
/// (hv, 2026-08-19). v2 wrote `AGENTS.md.bak` beside the file -- undeclared
/// until 2026-08-17, and covered by `*.bak` in `.gitignore`, so it never
/// reached git. It therefore guarded against a loss git already prevents, which
/// is the exact test hv used the same evening to withdraw AC-00.3. The
/// alternative was worse than redundant: v3 already carries D35's rolling
/// snapshots in [`crate::backup`], and a second backup mechanism for one file
/// is the Highlander violation this project names first.
pub fn sync(
  root: &Path,
  home: &Path,
  name: &str,
  cfg: &Config,
  ctx: &RenderContext<'_>,
) -> Result<PathBuf, RootFileError> {
  let content = render(home, name, cfg, ctx)?;
  let path = root.join(name);
  let mut set = crate::write_set::WriteSet::new();
  set.add(path.clone(), content);
  set
    .commit()
    .map_err(|source| RootFileError::Unwritable {
      path: path.display().to_string(),
      source,
    })?
    .keep();
  Ok(path)
}

// ---------------------------------------------------------------------------
// Blocks
// ---------------------------------------------------------------------------

/// A block opener, if this line is one.
///
/// Line-oriented and whole-line: a marker must be the only thing on its line.
/// An inline form would make the marker's own whitespace part of the output and
/// leave no way to write about the syntax in a template's prose.
fn opener(line: &str) -> Option<Block> {
  let t = line.trim();
  if let Some(rest) = t.strip_prefix("[[#lang ").and_then(|r| r.strip_suffix("]]")) {
    return Some(Block::Lang(rest.trim().to_string()));
  }
  if t == "[[#nolang]]" {
    return Some(Block::NoLang);
  }
  None
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Block {
  /// Kept when this language is declared.
  Lang(String),
  /// Kept when NO language is declared. The negation needs its own form: a
  /// template must be able to say what to print for a project that has not
  /// declared any, and there is no language name that means "none".
  NoLang,
}

impl Block {
  fn keeps(&self, languages: &[String]) -> bool {
    match self {
      Self::Lang(name) => languages.iter().any(|l| l == name),
      Self::NoLang => languages.is_empty(),
    }
  }

  fn opened(&self) -> String {
    match self {
      Self::Lang(name) => format!("[[#lang {name}]]"),
      Self::NoLang => "[[#nolang]]".to_string(),
    }
  }
}

fn resolve_blocks(template: &str, languages: &[String]) -> Result<String, Fault> {
  let mut out = String::with_capacity(template.len());
  let mut open: Option<Block> = None;

  // `split_inclusive`, never `lines()`: each piece carries its own terminator, so a
  // template that does not end in a newline does not gain one. Every real
  // template here ends with one and the difference never shows -- which is
  // exactly why a generator that silently adds a byte would never be caught.
  for line in template.split_inclusive('\n') {
    let t = line.trim();

    if let Some(block) = opener(line) {
      if let Some(outer) = &open {
        return Err(Fault::Nested(block.opened(), outer.opened()));
      }
      open = Some(block);
      continue;
    }

    if t == "[[/lang]]" || t == "[[/nolang]]" {
      let Some(block) = open.take() else {
        return Err(Fault::Unopened(t.to_string()));
      };
      // Deliberately NOT checking that `[[/lang]]` closes a `[[#lang]]`
      // specifically. Blocks do not nest, so there is exactly one thing a close
      // can mean, and a mismatch here would refuse a template whose meaning is
      // unambiguous.
      let _ = block;
      continue;
    }

    let keep = open.as_ref().is_none_or(|b| b.keeps(languages));
    if keep {
      out.push_str(line);
    }
  }

  match open {
    Some(block) => Err(Fault::Unclosed(block.opened())),
    None => Ok(out),
  }
}

// ---------------------------------------------------------------------------
// Tokens
// ---------------------------------------------------------------------------

fn expand_tokens(text: &str, cfg: &Config, ctx: &RenderContext<'_>) -> Result<String, Fault> {
  let mut out = String::with_capacity(text.len());
  let mut rest = text;

  while let Some(at) = rest.find("[[") {
    out.push_str(&rest[..at]);
    let after = &rest[at + 2..];
    let Some(end) = after.find("]]") else {
      // An unterminated `[[` is not a token and is not prose either -- the
      // template says nothing about what it meant, so refuse rather than emit
      // it. Named as an unknown token because that is the actionable half.
      return Err(Fault::UnknownToken(after.to_string()));
    };
    let name = &after[..end];
    out.push_str(value(name, cfg, ctx).ok_or_else(|| Fault::UnknownToken(name.to_string()))?);
    rest = &after[end + 2..];
  }
  out.push_str(rest);
  Ok(out)
}

/// The substitution table. **`DATE` is absent on purpose** -- see the module
/// doc; it reaches [`Fault::UnknownToken`] and refuses, which is the point.
fn value<'a>(name: &str, cfg: &'a Config, ctx: &RenderContext<'a>) -> Option<&'a str> {
  match name {
    "PROJECT_NAME" => Some(&cfg.project_name),
    "AUTHOR" => Some(&cfg.author),
    // The RUNNING tool's version, never `config.intent_version`. The banner
    // says what generated the file; the config says what the project is
    // stamped at, and `upgrade` moves one without touching the other. Every
    // other generated banner in this binary reads `ctx.version`, so taking a
    // second source here is how the root files come to disagree with the views
    // about which Intent produced them.
    "INTENT_VERSION" => Some(ctx.version),
    _ => None,
  }
}
