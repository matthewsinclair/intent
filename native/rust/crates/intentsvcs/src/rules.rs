//! The rule library -- the thing the whole agentic contract is written against.
//!
//! `CLAUDE.md`, `AGENTS.md`, `usage-rules.md`, `/in-standards`, `/in-essentials`
//! and all five `critic-<lang>` subagents reach the four rules of the road
//! through `intent claude rules show <id>`. Measured on this repo's own
//! machinery: **125 call sites**, against 230 for the whole `claude` family.
//! It is the single highest-traffic verb in the tool and v3 answered `2` for it.
//!
//! **ONE ENUMERATION, ONE PROVENANCE RULE, ONE FRONTMATTER CONTRACT, and v2
//! already proved why.** `rules_lib.sh` exists in v2 for exactly this reason:
//! `intent_claude_rules` and `critic_runner.sh` both need to know what a rule
//! is, and two answers to that question is two rule libraries
//! (IN-AG-HIGHLANDER-001). The headless critic is the next consumer here and it
//! reads this module rather than restating any of it.
//!
//! **ROOTS COME FROM THE INSTALL, NEVER THE ENVIRONMENT.** `$INTENT_HOME` is not
//! read -- AC-11.3, and the reason is specific rather than stylistic: the assets
//! are VERSIONED, so a machine mid-rollout has both trees and a leftover
//! `$INTENT_HOME` pointing at the v2 one. Reading it would make a v3 binary
//! serve v2's rule bodies from a variable the operator set for a different tool.
//! The executable's own location always knows which version is running.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use thiserror::Error;

/// The language subdirectories a rule pack may occupy.
///
/// **AN EXPLICIT LIST, not "every directory under `rules/`", and it is the same
/// list v2 carries.** The root also holds `_schema/` and `_attribution/`, which
/// contain markdown that is not a rule; enumerating everything and filtering by
/// a leading underscore makes the exclusion a naming convention that a future
/// directory silently joins or leaves. Naming the languages makes adding one a
/// deliberate edit in one place.
pub const LANGUAGES: [&str; 9] = [
  "agnostic", "elixir", "rust", "swift", "lua", "shell", "prose", "author", "content",
];

/// The two members of [`LANGUAGES`] a project never DECLARES, and why each one
/// is not there.
///
/// **A closed list is only safe when it declares why the things NOT in it are
/// not in it** (dc, 2026-08-25). Both of these are rule packs that exist to be
/// composed INTO another language's answer, never chosen as a project's own:
///
/// - `agnostic` is the cross-language pack every language pack concretises.
///   Declaring it would declare "this project is written in the rules".
/// - `prose` is the shared base that `author` and `content` both build on. A
///   project declares one of those two; `prose` is what they have in common.
///
/// Naming them here rather than filtering on a leading underscore is the same
/// choice [`LANGUAGES`] makes one doc-comment above, for the same reason: a
/// future pack must not join or leave this set by how somebody spells it.
pub const NON_DECLARABLE: [&str; 2] = ["agnostic", "prose"];

/// The languages `intent lang init` will accept, derived from [`LANGUAGES`].
///
/// **DERIVED, NOT A SECOND LIST.** This estate already carries two deliberately
/// distinct language sets -- [`LANGUAGES`] and [`crate::critic::HEADLESS_LANGUAGES`],
/// whose own doc says in as many words that it must not be collapsed into the
/// first. A third HAND-WRITTEN one is where they start disagreeing, and the
/// disagreement would be invisible: all three are correct-looking lists of
/// language names.
///
/// **v2 answered this question by listing directories under
/// `intent/plugins/agents/templates/`** -- which returns the right seven today
/// and returns them for a reason v3 retired. `intent lang init` no longer
/// installs a template, so enumerating the template directory would be a
/// correct value about a subject that is no longer the question.
pub fn declarable() -> Vec<&'static str> {
  let mut out: Vec<&'static str> = LANGUAGES
    .iter()
    .copied()
    .filter(|l| !NON_DECLARABLE.contains(l))
    .collect();
  out.sort_unstable();
  out
}

/// Whether `lang` is a language a project may declare.
pub fn is_declarable(lang: &str) -> bool {
  LANGUAGES.contains(&lang) && !NON_DECLARABLE.contains(&lang)
}

/// Where a rule came from.
///
/// **`canon` AND `ext:<name>` ARE DISTINCT VALUES AND MUST STAY THAT WAY.** A
/// rule the tool ships and a rule the operator can edit answer differently the
/// moment a critic cites one: the first is a bug report against Intent, the
/// second is a question about this machine. Merging them into "a rule" loses the
/// only fact that tells those two apart.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Provenance {
  Canon,
  Ext(String),
}

impl std::fmt::Display for Provenance {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Provenance::Canon => write!(f, "canon"),
      Provenance::Ext(name) => write!(f, "ext:{name}"),
    }
  }
}

/// One rule, as the list face needs it.
///
/// The body is deliberately NOT here. `list` reads 64 files to print 64 lines
/// and `show` reads one file to print all of it; carrying every body through
/// the list path would make the cheap operation pay for the expensive one.
#[derive(Debug, Clone)]
pub struct Rule {
  pub id: String,
  pub language: String,
  pub category: String,
  pub severity: String,
  pub title: String,
  pub provenance: Provenance,
  pub path: PathBuf,
}

/// The rule roots this binary serves.
#[derive(Debug, Clone)]
pub struct Library {
  canon: PathBuf,
  ext: Option<PathBuf>,
}

impl Library {
  /// Rooted at an install, with the ext base resolved separately.
  ///
  /// **`ext` IS AN `Option` RATHER THAN A PATH THAT MIGHT NOT EXIST**, because
  /// "extensions are switched off" and "the extensions directory is empty" are
  /// different states and only one of them is a decision. v2 collapses them by
  /// returning an empty string and then has to special-case it, or the `case`
  /// pattern degenerates to a bare slash-star and swallows every absolute path
  /// -- a comment in `rules_lib.sh:81` records that exact bug.
  ///
  /// **The glob is spelled out in words rather than quoted, and that is not
  /// fastidiousness.** `no_pm_state_in_output`'s scanner skips line comments and
  /// refuses a file containing a block-comment opener, because it would read the
  /// comment body as code. Quoting the token here made a comment about a
  /// pattern-matching bug into a pattern-matching bug, which is funnier than it
  /// is useful.
  pub fn new(install: &Path, ext: Option<PathBuf>) -> Self {
    Self {
      canon: install.join("intent/plugins/claude/rules"),
      ext,
    }
  }

  /// Every rule, canon first, then each extension.
  ///
  /// **A MISSING ROOT IS EMPTY, A PRESENT-BUT-UNREADABLE ROOT IS AN ERROR.** No
  /// rules installed is an ordinary state; a rules directory that cannot be read
  /// is a broken install, and returning an empty list for it would report "no
  /// rules" to a critic whose whole job is to apply them
  /// (IN-AG-NO-SILENT-001).
  pub fn rules(&self) -> Result<Vec<Rule>, RulesError> {
    let mut out = Vec::new();
    for path in self.files(&self.canon)? {
      out.push(self.read(&path, Provenance::Canon)?);
    }
    for (name, root) in self.ext_packs()? {
      for path in self.files(&root)? {
        out.push(self.read(&path, Provenance::Ext(name.clone()))?);
      }
    }
    Ok(out)
  }

  /// One rule and its whole body, by id.
  ///
  /// **THE BODY IS RETURNED VERBATIM, FRONTMATTER AND ALL.** v2's `show` prints
  /// the file; the frontmatter is part of what a reader is being shown, and
  /// stripping it would mean the agent reading the rule cannot see its severity
  /// or the principles it concretises without a second command.
  pub fn show(&self, id: &str) -> Result<Option<(Rule, String)>, RulesError> {
    let Some(rule) = self.rules()?.into_iter().find(|r| r.id == id) else {
      return Ok(None);
    };
    let body = std::fs::read_to_string(&rule.path).map_err(|source| RulesError::Io {
      path: rule.path.clone(),
      source,
    })?;
    Ok(Some((rule, body)))
  }

  /// Every `RULE.md` under one pack root, in a deterministic order.
  ///
  /// **SORTED, because `read_dir` is not.** The list face prints in this order
  /// and a command whose output depends on filesystem iteration order produces a
  /// different diff on every machine -- which is the class
  /// `corpus_machine_independence` exists to catch.
  fn files(&self, root: &Path) -> Result<Vec<PathBuf>, RulesError> {
    let mut found = Vec::new();
    if !root.is_dir() {
      return Ok(found);
    }
    for language in LANGUAGES {
      let dir = root.join(language);
      if !dir.is_dir() {
        continue;
      }
      collect(&dir, &mut found)?;
    }
    found.sort();
    Ok(found)
  }

  /// The extension packs, by name.
  ///
  /// Dotfiles and `_`-prefixed directories are skipped, matching v2: the first
  /// is the operator's own hidden state, the second is the same reservation the
  /// canon root uses for `_schema` and `_attribution`.
  fn ext_packs(&self) -> Result<Vec<(String, PathBuf)>, RulesError> {
    let Some(base) = &self.ext else {
      return Ok(Vec::new());
    };
    if !base.is_dir() {
      return Ok(Vec::new());
    }
    let entries = std::fs::read_dir(base).map_err(|source| RulesError::Io {
      path: base.clone(),
      source,
    })?;
    let mut packs = Vec::new();
    for entry in entries {
      let entry = entry.map_err(|source| RulesError::Io {
        path: base.clone(),
        source,
      })?;
      let name = entry.file_name().to_string_lossy().to_string();
      if name.starts_with('.') || name.starts_with('_') {
        continue;
      }
      let root = entry.path().join("rules");
      if root.is_dir() {
        packs.push((name, root));
      }
    }
    packs.sort();
    Ok(packs)
  }

  /// Parse one `RULE.md` into its list row.
  fn read(&self, path: &Path, provenance: Provenance) -> Result<Rule, RulesError> {
    let text = std::fs::read_to_string(path).map_err(|source| RulesError::Io {
      path: path.to_path_buf(),
      source,
    })?;
    let fm = frontmatter(&text);
    let field = |key: &str| fm.get(key).cloned().unwrap_or_default();
    let id = field("id");
    if id.is_empty() {
      return Err(RulesError::Unidentified {
        path: path.to_path_buf(),
      });
    }
    Ok(Rule {
      id,
      language: field("language"),
      category: field("category"),
      severity: field("severity"),
      title: field("title"),
      provenance,
      path: path.to_path_buf(),
    })
  }
}

/// Every `RULE.md` at or below `dir`.
fn collect(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), RulesError> {
  let entries = std::fs::read_dir(dir).map_err(|source| RulesError::Io {
    path: dir.to_path_buf(),
    source,
  })?;
  for entry in entries {
    let entry = entry.map_err(|source| RulesError::Io {
      path: dir.to_path_buf(),
      source,
    })?;
    let path = entry.path();
    if path.is_dir() {
      collect(&path, out)?;
    } else if path.file_name().is_some_and(|n| n == "RULE.md") {
      out.push(path);
    }
  }
  Ok(())
}

/// The frontmatter block's TOP-LEVEL scalar keys.
///
/// **TOP-LEVEL-ONLY IS THE CORRECTNESS PROPERTY, NOT A SHORTCUT, and the rule
/// bodies are what make it one.** Rule frontmatter carries folded scalars
/// (`summary: >`) and block lists (`applies_when:`, `applies_to:`) whose items
/// are prose in quotes -- and prose contains colons. A reader that splits every
/// line on its first colon inserts those list items as keys, and the day one of
/// them begins with `title:` or `severity:` it SHADOWS the real field with a
/// fragment of a sentence. Requiring column zero makes a list item structurally
/// unable to be mistaken for a key, which is stronger than any amount of care.
///
/// **`legacy.rs` HAS A PRIVATE FLAT READER AND IT IS DELIBERATELY NOT REUSED.**
/// It parses v2 ST and issue frontmatter, which is flat, so it splits every line
/// including indented ones. Folding the two is the right end state -- top-level-
/// only is strictly more correct for flat frontmatter too -- but it changes the
/// v2 migrator, and doing that inside a hosting change would put a migration
/// regression and a new command in one diff.
///
/// Values are trimmed and a single pair of surrounding quotes is stripped, so
/// `title: "Foo"` and `title: Foo` read the same. A colon INSIDE a value
/// survives: the split is on the first colon only, so `title: Foo: bar` yields
/// `Foo: bar`.
fn frontmatter(text: &str) -> BTreeMap<String, String> {
  let mut map = BTreeMap::new();
  let Some(rest) = text.strip_prefix("---\n") else {
    return map;
  };
  let Some(end) = rest.find("\n---") else {
    return map;
  };
  for line in rest[..end].lines() {
    // Column zero, or it is not a key. An indented `- "...: ..."` is a list
    // item and a continuation line is prose; neither is a field.
    if line.starts_with(char::is_whitespace) {
      continue;
    }
    let Some((key, value)) = line.split_once(':') else {
      continue;
    };
    map.insert(key.trim().to_string(), unquote(value.trim()).to_string());
  }
  map
}

/// One surrounding pair of quotes, removed. Never an inner pair.
fn unquote(value: &str) -> &str {
  for q in ['"', '\''] {
    if let Some(inner) = value.strip_prefix(q).and_then(|v| v.strip_suffix(q)) {
      return inner;
    }
  }
  value
}

#[derive(Debug, Error)]
pub enum RulesError {
  /// **A RULE WITH NO ID IS REFUSED, NOT SKIPPED.** Every consumer addresses a
  /// rule by its id -- `intent claude rules show <id>`, a critic's finding, a
  /// `concretised_by:` reference -- so a file without one is unreachable by
  /// construction. Dropping it quietly means the library reports 63 of 64 and
  /// nothing says which one went.
  #[error("the rule at {path} declares no `id:` in its frontmatter, so nothing can address it")]
  Unidentified { path: PathBuf },
  #[error("cannot read the rule library at {path}: {source}")]
  Io {
    path: PathBuf,
    #[source]
    source: std::io::Error,
  },
}

impl crate::remedy::Remedy for RulesError {
  fn remedy(&self) -> String {
    match self {
      Self::Unidentified { .. } => {
        "add an `id:` to that file's frontmatter, or move it out of the rule tree if it is not a rule -- `_schema/` and `_attribution/` are where the library keeps markdown that is not one.".to_string()
      }
      Self::Io { path, .. } => format!(
        "check that {} exists and is readable. The rule library ships with the tool, so this usually means the install is incomplete rather than that anything is misconfigured -- `intent info` names the install root this binary resolved.",
        path.display()
      ),
    }
  }
}
