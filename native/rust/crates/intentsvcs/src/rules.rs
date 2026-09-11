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

  /// Where this install's CANON rules would be, present or not.
  ///
  /// **EXPOSED SO A CALLER CAN TELL AN EMPTY LIBRARY FROM AN ABSENT ONE**
  /// (`0275`). [`Library::files`] returns an empty list for a root that is not a
  /// directory -- correct there, because no rules installed is an ordinary state
  /// -- and that makes the two indistinguishable one layer up, where the
  /// difference is the whole answer. It is the CANON root specifically and not
  /// "the library": an ext pack can supply rules while this is missing, so a
  /// caller asking "is the shipped tree here" must not be answered with a count.
  pub fn canon_root(&self) -> &Path {
    &self.canon
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

  /// Every rule document, UNJUDGED -- canon first, then each extension.
  ///
  /// **UNLIKE [`Library::rules`] THIS DOES NOT REFUSE A FILE WITH NO ID.** That
  /// refusal is right for the read path, where an unaddressable rule is
  /// unusable; it is exactly wrong for the validate path, whose job is to
  /// REPORT that file rather than to fail on it.
  pub fn docs(&self) -> Result<Vec<RuleDoc>, RulesError> {
    let mut out = Vec::new();
    for path in self.files(&self.canon)? {
      out.push(self.doc(&path, Provenance::Canon)?);
    }
    for (name, root) in self.ext_packs()? {
      for path in self.files(&root)? {
        out.push(self.doc(&path, Provenance::Ext(name.clone()))?);
      }
    }
    Ok(out)
  }

  fn doc(&self, path: &Path, provenance: Provenance) -> Result<RuleDoc, RulesError> {
    let text = std::fs::read_to_string(path).map_err(|source| RulesError::Io {
      path: path.to_path_buf(),
      source,
    })?;
    Ok(RuleDoc {
      path: path.to_path_buf(),
      provenance,
      front: parse_front(&text),
    })
  }

  /// Validate the library, optionally reporting only the rules `subject`
  /// selects -- an id, or a path.
  ///
  /// **THE CORPUS IS ALWAYS THE WHOLE LIBRARY EVEN WHEN THE SUBJECT IS ONE
  /// FILE, and that is not an optimisation.** Two of the checks are about
  /// RELATIONSHIPS -- a duplicate id, a `references:` that resolves -- and a
  /// corpus of one cannot answer either. Narrowing the corpus to the subject
  /// would report every citation in a valid rule as unresolved, which is a
  /// confident wrong answer rather than a missing one.
  ///
  /// **THE NARROWING IS THEREFORE ON THE REPORT, NOT ON THE CHECK.**
  pub fn validate(&self, subject: Option<&str>) -> Result<(Vec<Finding>, usize), RulesError> {
    let mut docs = self.docs()?;

    // A subject naming a file OUTSIDE the library still gets validated, and it
    // joins the corpus rather than replacing it -- that is how `rules validate
    // <a fixture>` can report a duplicate id against the shipped rules.
    if let Some(name) = subject {
      let path = Path::new(name);
      if path.is_file() && !docs.iter().any(|d| d.path == path) {
        docs.push(self.doc(path, Provenance::Canon)?);
      }
    }

    let schema = Schema::read(&self.canon)?;
    let attributions = attributions(&self.canon)?;
    let findings = check(&docs, &schema, &attributions);

    let Some(name) = subject else {
      return Ok((findings, docs.len()));
    };
    let selected: Vec<&RuleDoc> = docs
      .iter()
      .filter(|d| d.path == Path::new(name) || d.front.scalars.get("id").is_some_and(|i| i == name))
      .collect();
    let paths: Vec<&PathBuf> = selected.iter().map(|d| &d.path).collect();
    Ok((
      findings
        .into_iter()
        .filter(|f| paths.contains(&&f.path))
        .collect(),
      selected.len(),
    ))
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
/// One frontmatter block, parsed ONCE into both of the views anyone needs.
///
/// **TWO VIEWS, ONE PARSE, AND THAT IS THE WHOLE REASON THIS TYPE EXISTS.**
/// Reading a rule needs the scalars; VALIDATING one needs the block lists too --
/// `references:` followed by indented `- ` items is the shape the
/// `unresolved-reference` fixture uses and the shape the old scalar-only reader
/// silently dropped. A second parser for the second view is how two readers of
/// one format start disagreeing about it.
#[derive(Debug, Default)]
pub struct Front {
  /// `key: value` at column zero.
  pub scalars: BTreeMap<String, String>,
  /// `key:` at column zero followed by indented `- item` lines, and inline
  /// `key: [a, b]`. **Both spellings land here** so a caller never has to know
  /// which one an author used.
  pub lists: BTreeMap<String, Vec<String>>,
  /// Whether the block was present at all. **An ABSENT block and an EMPTY one
  /// are different states**: the first is a file that is not a rule, the second
  /// is a rule with nothing declared, and collapsing them would make
  /// `missing-frontmatter` indistinguishable from a rule whose keys are all
  /// blank.
  pub present: bool,
}

/// Parse a frontmatter block. **Pure: every input is an argument.**
pub fn parse_front(text: &str) -> Front {
  let mut front = Front::default();
  let Some(rest) = text.strip_prefix("---\n") else {
    return front;
  };
  // **AN IMMEDIATELY-CLOSED BLOCK IS A BLOCK.** Searching only for `\n---`
  // cannot see `---\n---\n`, so a file with an EMPTY block read as a file with
  // NO block -- the two states this type exists to keep apart, collapsed in the
  // parser that produces it. The verdict happened to agree (both are errors),
  // which is why it survived: only the message differed.
  let Some(end) = (if rest.starts_with("---") {
    Some(0)
  } else {
    rest.find("\n---")
  }) else {
    return front;
  };
  front.present = true;

  let mut open: Option<String> = None;
  for line in rest[..end].lines() {
    // An indented `- item` continues the key above it. Anything else indented
    // is prose -- a folded scalar's body -- and is not a field.
    if line.starts_with(char::is_whitespace) {
      if let (Some(key), Some(item)) = (open.as_ref(), line.trim().strip_prefix("- ")) {
        front
          .lists
          .entry(key.clone())
          .or_default()
          .push(unquote(item.trim()).to_string());
      }
      continue;
    }
    let Some((key, value)) = line.split_once(':') else {
      open = None;
      continue;
    };
    let key = key.trim().to_string();
    let value = value.trim();

    // An inline `[a, b]` is the same declaration as a block list and is read as
    // one. An empty `[]` declares the key with no members, which is NOT the
    // same as not declaring it.
    if let Some(inner) = value.strip_prefix('[').and_then(|v| v.strip_suffix(']')) {
      let items: Vec<String> = inner
        .split(',')
        .map(|i| unquote(i.trim()).to_string())
        .filter(|i| !i.is_empty())
        .collect();
      front.lists.insert(key.clone(), items);
      front.scalars.insert(key, value.to_string());
      open = None;
      continue;
    }

    // A bare `key:` opens a block list. It is recorded as a scalar too, so a
    // required-field check sees the key was declared.
    if value.is_empty() {
      open = Some(key.clone());
      front.lists.entry(key.clone()).or_default();
    } else {
      open = None;
    }
    front.scalars.insert(key, unquote(value).to_string());
  }
  front
}

fn frontmatter(text: &str) -> BTreeMap<String, String> {
  parse_front(text).scalars
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
  /// **THE SCHEMA DOCUMENT PARSED TO NOTHING, AND THAT REFUSES RATHER THAN
  /// DEGRADING.** An empty vocabulary would pass every rule for having all zero
  /// of its required fields AND fail every rule for declaring keys no table
  /// admits -- two silent, opposite wrongnesses from one missing table.
  #[error("the rule schema at {path} declares no field tables, so no vocabulary could be read")]
  SchemaUnreadable { path: PathBuf },
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
      Self::SchemaUnreadable { path } => format!(
        "reinstall Intent -- {} ships with the tool and its `### Required fields` and `### Optional fields` tables are what the validator reads the vocabulary from",
        path.display()
      ),
    }
  }
}

// ---------------------------------------------------------------------------
// Validation -- `intent claude rules validate`
// ---------------------------------------------------------------------------

/// How bad a finding is.
///
/// **AN UNKNOWN TOP-LEVEL KEY IS AN `Error`, RULED BY vc 2026-09-09**, against
/// `tests/unit/rule_validator.bats`, which asserted a warning. The bats arm was
/// written against a verb that has never existed, so it had never executed --
/// an executable spec that has never executed is prose with a shebang -- while
/// `_schema/rule-schema.md` states the contract in the opposite direction:
/// *`intent claude rules validate` rejects unknown top-level keys*. On the
/// substance: this is a VALIDATOR, and one that warns on an unknown key lets a
/// typo'd field ship silently, which is how schema drift enters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
  Error,
  Warning,
}

/// One thing wrong with one rule.
#[derive(Debug, Clone)]
pub struct Finding {
  pub path: PathBuf,
  pub level: Level,
  pub message: String,
}

/// A rule document as READ, before anything has judged it.
///
/// **THIS IS THE VALIDATOR'S INPUT AND [`Rule`] IS NOT.** `Rule` is the parsed,
/// believed model -- it cannot represent a file with no id, because [`Library::read`]
/// refuses one. A validator whose input type cannot hold the defect it looks for
/// can only ever report that everything is fine.
#[derive(Debug)]
pub struct RuleDoc {
  pub path: PathBuf,
  pub provenance: Provenance,
  pub front: Front,
}

/// The declared frontmatter vocabulary, READ FROM THE SCHEMA DOCUMENT.
///
/// **ONE HOME, AND IT IS THE DOCUMENT.** A `const` list of key names in this
/// file would be a second home for a vocabulary `_schema/rule-schema.md` already
/// states in a machine-readable table -- and the two would drift the first time
/// somebody added a field to the doc alone, which is exactly the direction the
/// doc is edited.
#[derive(Debug, Default)]
pub struct Schema {
  pub required: Vec<String>,
  pub optional: Vec<String>,
}

impl Schema {
  /// Parse the two field tables out of the schema document.
  ///
  /// **A SCHEMA THAT PARSES TO NOTHING IS AN ERROR, NEVER AN EMPTY VOCABULARY.**
  /// An empty `required` would pass every rule for having all zero of its
  /// mandatory fields, and an empty `optional` would fail every rule for
  /// declaring keys no table admits. Both are silent, and they are opposite, so
  /// neither would be caught by "the tests still pass".
  pub fn read(canon: &Path) -> Result<Self, RulesError> {
    let path = canon.join("_schema/rule-schema.md");
    let text = std::fs::read_to_string(&path).map_err(|source| RulesError::Io {
      path: path.clone(),
      source,
    })?;
    let mut schema = Self::default();
    let mut into: Option<&mut Vec<String>> = None;
    for line in text.lines() {
      if let Some(heading) = line.strip_prefix("### ") {
        into = match heading.trim() {
          "Required fields" => Some(&mut schema.required),
          "Optional fields" => Some(&mut schema.optional),
          _ => None,
        };
        continue;
      }
      // A field row is `| \`name\` | ...`. The separator row and the header row
      // both fail the backtick test, so neither needs special-casing.
      if let Some(bucket) = into.as_deref_mut()
        && let Some(cell) = line.strip_prefix("| ").and_then(|l| l.split('|').next())
        && let Some(name) = cell
          .trim()
          .strip_prefix('`')
          .and_then(|c| c.strip_suffix('`'))
      {
        bucket.push(name.to_string());
      }
    }
    if schema.required.is_empty() || schema.optional.is_empty() {
      return Err(RulesError::SchemaUnreadable { path });
    }
    Ok(schema)
  }

  fn declares(&self, key: &str) -> bool {
    self.required.iter().any(|k| k == key) || self.optional.iter().any(|k| k == key)
  }
}

/// `(intent rule id, upstream slug)` pairs from the attribution tables.
///
/// **THE PAIR, NOT THE ID.** `attribution-policy.md` requires a rule carrying
/// `upstream_id:` to have a MATCHING row, and a row matches on both halves: an
/// id present under the wrong slug is a mis-filed attribution, which is the
/// thing a licence audit would care about and the thing an id-only check cannot
/// see.
pub fn attributions(canon: &Path) -> Result<Vec<(String, String)>, RulesError> {
  let dir = canon.join("_attribution");
  let mut out = Vec::new();
  if !dir.is_dir() {
    return Ok(out);
  }
  let entries = std::fs::read_dir(&dir).map_err(|source| RulesError::Io {
    path: dir.clone(),
    source,
  })?;
  for entry in entries {
    let path = entry
      .map_err(|source| RulesError::Io {
        path: dir.clone(),
        source,
      })?
      .path();
    if path.extension().and_then(|e| e.to_str()) != Some("md") {
      continue;
    }
    let text = std::fs::read_to_string(&path).map_err(|source| RulesError::Io {
      path: path.clone(),
      source,
    })?;
    for line in text.lines() {
      let cells: Vec<&str> = line.split('|').map(str::trim).collect();
      // `| `IN-EX-TEST-002` | `no-process-sleep` | ...` -- leading empty cell,
      // then the two backticked halves. Anything else is not a derived-rule row.
      if cells.len() < 4 {
        continue;
      }
      let ticked = |c: &str| {
        c.strip_prefix('`')
          .and_then(|v| v.strip_suffix('`'))
          .map(str::to_string)
      };
      if let (Some(id), Some(slug)) = (ticked(cells[1]), ticked(cells[2]))
        && id.starts_with("IN-")
      {
        out.push((id, slug));
      }
    }
  }
  Ok(out)
}

/// Is this a well-formed rule id?
///
/// **THE CATEGORY MAY CONTAIN DASHES AND THAT IS WHY THIS IS NOT A SEGMENT
/// COUNT.** `IN-AG-THIN-COORD-001` is real and has five segments where
/// `IN-EX-CODE-001` has four, so the shape is: the `IN-` prefix, a three-digit
/// tail, and at least two segments between them.
fn id_is_well_formed(id: &str) -> bool {
  let Some(rest) = id.strip_prefix("IN-") else {
    return false;
  };
  let segments: Vec<&str> = rest.split('-').collect();
  if segments.len() < 3 {
    return false;
  }
  let Some((tail, head)) = segments.split_last() else {
    return false;
  };
  tail.len() == 3
    && tail.chars().all(|c| c.is_ascii_digit())
    && head.iter().all(|s| {
      !s.is_empty()
        && s
          .chars()
          .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
    })
}

/// Every key whose value is a list of rule ids, so a broken citation is caught
/// wherever it is spelled rather than only in `references`.
const ID_LISTS: &[&str] = &[
  "references",
  "concretised_by",
  "related_rules",
  "conflicts_with",
];

/// **PURE. Every input is an argument, and that is what lets a caller drive
/// both verdicts without a filesystem.**
///
/// The enumeration -- which roots, which files, what the schema says -- is the
/// caller's, in [`Library::validate`]. This function decides.
pub fn check(docs: &[RuleDoc], schema: &Schema, attributions: &[(String, String)]) -> Vec<Finding> {
  let mut findings = Vec::new();
  let known: Vec<&str> = docs
    .iter()
    .filter_map(|d| d.front.scalars.get("id").map(String::as_str))
    .filter(|id| !id.is_empty())
    .collect();

  for doc in docs {
    let mut say = |level: Level, message: String| {
      findings.push(Finding {
        path: doc.path.clone(),
        level,
        message,
      })
    };

    // A file with no block is not a rule at all, and every check below would
    // report a separate missing field for the same one cause. Reported once.
    if !doc.front.present {
      say(
        Level::Error,
        "no YAML frontmatter -- a rule file opens with a `---` block".to_string(),
      );
      continue;
    }

    for key in &schema.required {
      match doc.front.scalars.get(key) {
        None => say(
          Level::Error,
          format!("required frontmatter key `{key}` is missing"),
        ),
        Some(v) if v.is_empty() && !doc.front.lists.contains_key(key) => say(
          Level::Error,
          format!("required frontmatter key `{key}` is empty"),
        ),
        _ => {}
      }
    }

    for key in doc.front.scalars.keys() {
      if !schema.declares(key) {
        say(
          Level::Error,
          format!("frontmatter key `{key}` is not in the declared schema"),
        );
      }
    }

    if let Some(id) = doc.front.scalars.get("id") {
      if !id.is_empty() && !id_is_well_formed(id) {
        say(
          Level::Error,
          format!("id `{id}` does not match `IN-<LANG>-<CAT>-<NNN>`"),
        );
      }
      let twins = docs
        .iter()
        .filter(|o| o.path != doc.path && o.front.scalars.get("id") == Some(id))
        .count();
      if twins > 0 && !id.is_empty() {
        say(
          Level::Error,
          format!("id `{id}` is declared by {} other rule file(s)", twins),
        );
      }
    }

    for key in ID_LISTS {
      for cited in doc.front.lists.get(*key).into_iter().flatten() {
        if !known.contains(&cited.as_str()) {
          say(
            Level::Error,
            format!("`{key}` cites `{cited}`, which no rule in this corpus declares"),
          );
        }
      }
    }

    // `null` is how the fixtures spell "no upstream", so it is absence and not
    // a slug. Treating it as one would demand an attribution row for the word.
    if let (Some(up), Some(id)) = (
      doc.front.scalars.get("upstream_id"),
      doc.front.scalars.get("id"),
    ) && !up.is_empty()
      && up != "null"
      && !attributions.iter().any(|(a, s)| a == id && s == up)
    {
      say(
        Level::Error,
        format!("`upstream_id: {up}` has no matching row in `_attribution/` for `{id}`"),
      );
    }
  }
  findings
}

#[cfg(test)]
mod validate_tests {
  use super::*;

  /// **EVERY ARM BELOW IS TWO-SIDED, AND THAT IS THE WHOLE REASON [`check`] IS
  /// PURE.** A validator arm that has only ever been seen to pass is
  /// indistinguishable from one that cannot fail, and the enumeration is the
  /// caller's, so none of this needs a filesystem to drive both verdicts.
  fn doc(name: &str, front: &str) -> RuleDoc {
    RuleDoc {
      path: PathBuf::from(name),
      provenance: Provenance::Canon,
      front: parse_front(front),
    }
  }

  fn schema() -> Schema {
    Schema {
      required: ["id", "title", "language"]
        .iter()
        .map(|s| s.to_string())
        .collect(),
      optional: ["references", "upstream_id", "tags"]
        .iter()
        .map(|s| s.to_string())
        .collect(),
    }
  }

  const CLEAN: &str = "---\nid: IN-AG-FIXT-001\ntitle: A rule\nlanguage: agnostic\n---\nbody\n";

  #[test]
  fn a_clean_rule_produces_nothing() {
    assert!(check(&[doc("a", CLEAN)], &schema(), &[]).is_empty());
  }

  #[test]
  fn an_absent_frontmatter_block_is_reported_once_and_not_as_eight_missing_fields() {
    let f = check(&[doc("a", "# just a heading\n")], &schema(), &[]);
    assert_eq!(f.len(), 1, "one cause, one finding: {f:?}");
    assert!(f[0].message.contains("no YAML frontmatter"), "{:?}", f[0]);
  }

  #[test]
  fn a_missing_required_key_is_named_and_a_present_one_is_not() {
    let f = check(
      &[doc("a", "---\nid: IN-AG-FIXT-001\ntitle: A rule\n---\n")],
      &schema(),
      &[],
    );
    assert_eq!(f.len(), 1, "{f:?}");
    assert!(f[0].message.contains("`language`"), "{:?}", f[0]);
  }

  #[test]
  fn an_undeclared_key_is_an_error_and_a_declared_one_is_silent() {
    let with = doc(
      "a",
      "---\nid: IN-AG-FIXT-001\ntitle: A rule\nlanguage: agnostic\nnope: x\n---\n",
    );
    let f = check(&[with], &schema(), &[]);
    assert_eq!(f.len(), 1, "{f:?}");
    assert_eq!(f[0].level, Level::Error, "vc ruled REJECT, not warn");
    assert!(f[0].message.contains("`nope`"), "{:?}", f[0]);

    // The control: `tags` IS declared, and the same shape must stay silent, or
    // the arm above is firing on "has an extra key" rather than on "undeclared".
    let ok = doc(
      "b",
      "---\nid: IN-AG-FIXT-002\ntitle: A rule\nlanguage: agnostic\ntags: [x]\n---\n",
    );
    assert!(check(&[ok], &schema(), &[]).is_empty());
  }

  #[test]
  fn a_malformed_id_is_caught_and_a_dashed_category_is_not() {
    let bad = doc(
      "a",
      "---\nid: BAD-ID-999\ntitle: A rule\nlanguage: agnostic\n---\n",
    );
    let f = check(&[bad], &schema(), &[]);
    assert!(
      f.iter().any(|x| x.message.contains("does not match")),
      "{f:?}"
    );

    // **THE CONTROL THAT MATTERS: a category may contain dashes.**
    // `IN-AG-THIN-COORD-001` is a real shipped id, and a segment-count check
    // would refuse it while passing everything this test cares about.
    let dashed = doc(
      "b",
      "---\nid: IN-AG-THIN-COORD-001\ntitle: A rule\nlanguage: agnostic\n---\n",
    );
    assert!(
      check(&[dashed], &schema(), &[]).is_empty(),
      "a dashed category must not be read as a malformed id"
    );
  }

  #[test]
  fn a_duplicate_id_names_both_and_two_distinct_ids_do_not() {
    let f = check(&[doc("a", CLEAN), doc("b", CLEAN)], &schema(), &[]);
    assert_eq!(f.len(), 2, "both files are told: {f:?}");
    assert!(
      f.iter().all(|x| x.message.contains("is declared by")),
      "{f:?}"
    );

    let other = CLEAN.replace("FIXT-001", "FIXT-002");
    assert!(check(&[doc("a", CLEAN), doc("b", &other)], &schema(), &[]).is_empty());
  }

  #[test]
  fn a_reference_resolves_against_the_whole_corpus_and_a_broken_one_is_named() {
    let citing = doc(
      "a",
      "---\nid: IN-AG-FIXT-001\ntitle: A rule\nlanguage: agnostic\nreferences:\n  - IN-AG-FIXT-002\n---\n",
    );
    let target = doc("b", &CLEAN.replace("FIXT-001", "FIXT-002"));

    // **THE CORPUS IS WHAT MAKES THE CITATION RESOLVE**, which is why
    // `Library::validate` never narrows it to the subject.
    assert!(check(&[citing, target], &schema(), &[]).is_empty());

    let orphan = doc(
      "a",
      "---\nid: IN-AG-FIXT-001\ntitle: A rule\nlanguage: agnostic\nreferences:\n  - IN-AG-GONE-999\n---\n",
    );
    let f = check(&[orphan], &schema(), &[]);
    assert_eq!(f.len(), 1, "{f:?}");
    assert!(f[0].message.contains("IN-AG-GONE-999"), "{:?}", f[0]);
  }

  #[test]
  fn an_upstream_id_needs_a_matching_attribution_row_and_null_needs_none() {
    let borrowed =
      "---\nid: IN-AG-FIXT-001\ntitle: A rule\nlanguage: agnostic\nupstream_id: a-slug\n---\n";
    let f = check(&[doc("a", borrowed)], &schema(), &[]);
    assert_eq!(f.len(), 1, "{f:?}");
    assert!(f[0].message.contains("a-slug"), "{:?}", f[0]);

    let rows = vec![("IN-AG-FIXT-001".to_string(), "a-slug".to_string())];
    assert!(check(&[doc("a", borrowed)], &schema(), &rows).is_empty());

    // **THE PAIR, NOT THE ID.** The same id filed under a different slug is a
    // mis-filed attribution and must still be reported.
    let wrong = vec![("IN-AG-FIXT-001".to_string(), "another-slug".to_string())];
    assert_eq!(check(&[doc("a", borrowed)], &schema(), &wrong).len(), 1);

    // `null` is how the fixtures spell "no upstream", so it is absence.
    let none =
      "---\nid: IN-AG-FIXT-001\ntitle: A rule\nlanguage: agnostic\nupstream_id: null\n---\n";
    assert!(check(&[doc("a", none)], &schema(), &[]).is_empty());
  }

  #[test]
  fn a_block_list_and_an_inline_list_are_read_as_the_same_declaration() {
    let block = parse_front("---\nreferences:\n  - A\n  - B\n---\n");
    let inline = parse_front("---\nreferences: [A, B]\n---\n");
    assert_eq!(
      block.lists.get("references"),
      inline.lists.get("references")
    );
    assert_eq!(
      block.lists["references"],
      vec!["A".to_string(), "B".to_string()]
    );
  }

  #[test]
  fn an_absent_block_and_an_empty_one_are_different_states() {
    assert!(!parse_front("# no block\n").present);
    assert!(parse_front("---\n---\n").present);
  }
}
