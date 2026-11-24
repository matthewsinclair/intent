//! A project file that repeats a global rule (issue Ia87820b).
//!
//! The user's global Claude files (`CLAUDE.md`, `AGENTS.md` and what they
//! import, eg `house-rules.md`) hold the rules every session loads. A project's
//! `CLAUDE.md`, `AGENTS.md` or memory file that repeats one of them is a second
//! copy of one setting (Highlander), and it drifts when the global copy
//! changes. `doctor` reports each such sentence as `restated-global-rule`.
//!
//! **A LITERAL MATCH, NEVER A PARAPHRASE.** A sentence matches when its
//! normalised text equals a global sentence's: list and heading marks,
//! emphasis, backticks and the closing punctuation go, and case and spaces
//! fold. A rule said again in other words is not found, and nothing here
//! claims it is. A sentence shorter than [`MIN_WORDS`] is never compared, so a
//! common phrase such as "Use the CLI." matches nothing.
//!
//! **THE CORE IS PURE** (`IN-AG-PFIC-001`): [`normalise`], [`sentences`],
//! [`restatements`] and [`memory_slug`] read nothing. The readers,
//! [`global_files`] and [`project_files`], read the files and nothing else.

use std::collections::BTreeSet;
use std::path::{Component, Path, PathBuf};

use crate::project::Project;

/// The fewest words a sentence has before it is compared.
pub const MIN_WORDS: usize = 6;

/// The two agent contracts, by name: a project's, and the global ones every
/// session loads from the Claude dir, which carry the same two names.
const CONTRACTS: &[&str] = &["CLAUDE.md", "AGENTS.md"];

/// One file's text, by the path a finding names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text {
  pub path: PathBuf,
  pub body: String,
}

/// A place in a file: its path and its 1-based line.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct At {
  pub path: PathBuf,
  pub line: usize,
}

/// A project sentence that a global sentence already says.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Restatement {
  pub project: At,
  pub global: At,
}

/// `sentence` as it is compared: marks, emphasis, backticks and closing
/// punctuation removed, case and spaces folded.
pub fn normalise(sentence: &str) -> String {
  let unmarked = sentence
    .trim_start()
    .trim_start_matches(['#', '>'])
    .trim_start();
  let unmarked = unmarked
    .strip_prefix("- ")
    .or_else(|| unmarked.strip_prefix("* "))
    .or_else(|| unmarked.strip_prefix("+ "))
    .unwrap_or_else(|| strip_ordinal(unmarked));
  let plain: String = unmarked
    .chars()
    .filter(|c| !matches!(c, '*' | '`'))
    .collect();
  let folded = plain.split_whitespace().collect::<Vec<_>>().join(" ");
  folded
    .trim_end_matches(['.', ':', ';', '!', '?', ','])
    .to_lowercase()
}

/// A numbered list item's text without its number, or the text as it is.
fn strip_ordinal(text: &str) -> &str {
  let digits = text.len() - text.trim_start_matches(|c: char| c.is_ascii_digit()).len();
  match text[digits..].strip_prefix(". ") {
    Some(rest) if digits > 0 => rest,
    _ => text,
  }
}

/// Each sentence of `body` worth comparing, normalised, with its 1-based line.
///
/// A line is cut after each `.`, `!` or `?` that a space follows. Lines inside
/// a fenced code block are skipped, because code is not a rule.
pub fn sentences(body: &str) -> Vec<(usize, String)> {
  let mut out = Vec::new();
  let mut fenced = false;
  for (index, line) in body.lines().enumerate() {
    if line.trim_start().starts_with("```") {
      fenced = !fenced;
      continue;
    }
    if fenced {
      continue;
    }
    for piece in split_sentences(line) {
      let sentence = normalise(piece);
      if sentence.split_whitespace().count() >= MIN_WORDS {
        out.push((index + 1, sentence));
      }
    }
  }
  out
}

/// `line` cut after each `.`, `!` or `?` that whitespace follows.
fn split_sentences(line: &str) -> Vec<&str> {
  let mut pieces = Vec::new();
  let mut start = 0;
  let bytes = line.as_bytes();
  for (at, &byte) in bytes.iter().enumerate() {
    let ends =
      matches!(byte, b'.' | b'!' | b'?') && bytes.get(at + 1).is_some_and(u8::is_ascii_whitespace);
    if ends {
      pieces.push(&line[start..=at]);
      start = at + 1;
    }
  }
  pieces.push(&line[start..]);
  pieces
}

/// Every project sentence that a global sentence says, each named with the
/// first global place that says it, in path and line order.
pub fn restatements(global: &[Text], project: &[Text]) -> Vec<Restatement> {
  let mut said: std::collections::BTreeMap<String, At> = std::collections::BTreeMap::new();
  for text in global {
    for (line, sentence) in sentences(&text.body) {
      said.entry(sentence).or_insert_with(|| At {
        path: text.path.clone(),
        line,
      });
    }
  }
  let mut found = BTreeSet::new();
  for text in project {
    for (line, sentence) in sentences(&text.body) {
      if let Some(global) = said.get(&sentence) {
        found.insert(Restatement {
          project: At {
            path: text.path.clone(),
            line,
          },
          global: global.clone(),
        });
      }
    }
  }
  found.into_iter().collect()
}

/// The directory name Claude Code keeps a project's memory under: the root
/// with every character that is not an ASCII letter or digit made `-`.
pub fn memory_slug(root: &Path) -> String {
  root
    .to_string_lossy()
    .chars()
    .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
    .collect()
}

/// What a reader found: the files it read, and each file that is there and
/// could not be read, so a check that read nothing does not read as one that
/// found nothing.
#[derive(Debug, Default)]
pub struct Read {
  pub texts: Vec<Text>,
  pub unread: Vec<Unread>,
}

/// A file that is there and could not be read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unread {
  pub path: PathBuf,
  pub cause: String,
}

impl Read {
  /// Read `path` into `self` through `shape`; a file that is not there is
  /// nothing, and any other failure is kept as [`Unread`].
  fn take(&mut self, path: PathBuf, shape: impl FnOnce(String) -> String) -> Option<&str> {
    match std::fs::read_to_string(&path) {
      Ok(body) => {
        self.texts.push(Text {
          body: shape(body),
          path,
        });
        self.texts.last().map(|text| text.body.as_str())
      }
      Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
      Err(e) => {
        self.unread.push(Unread {
          path,
          cause: e.to_string(),
        });
        None
      }
    }
  }

  /// The entries of `dir` that `keep` admits, sorted; a directory that is not
  /// there holds none, and any other failure is kept as [`Unread`].
  fn list(&mut self, dir: &Path, keep: impl Fn(&Path) -> bool) -> Vec<PathBuf> {
    let entries = match std::fs::read_dir(dir) {
      Ok(entries) => entries,
      Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Vec::new(),
      Err(e) => {
        self.unread.push(Unread {
          path: dir.to_path_buf(),
          cause: e.to_string(),
        });
        return Vec::new();
      }
    };
    let mut paths = Vec::new();
    for entry in entries {
      match entry {
        Ok(entry) if keep(&entry.path()) => paths.push(entry.path()),
        Ok(_) => {}
        Err(e) => self.unread.push(Unread {
          path: dir.to_path_buf(),
          cause: e.to_string(),
        }),
      }
    }
    paths.sort();
    paths
  }
}

/// The global files under `claude_dir`: the two `CONTRACTS` and each file they
/// import with an `@<file>` line, read once each.
///
/// **AN IMPORT IS FOLLOWED ONLY INSIDE THE CLAUDE DIR**: a relative path with
/// no `..`, taken from the importing file's directory. A file that is not
/// there is not read, because Claude Code reads nothing there either.
pub fn global_files(claude_dir: &Path) -> Read {
  let mut queue: Vec<PathBuf> = CONTRACTS.iter().map(|n| claude_dir.join(n)).collect();
  let mut seen = BTreeSet::new();
  let mut read = Read::default();
  while let Some(path) = queue.pop() {
    if !seen.insert(path.clone()) {
      continue;
    }
    let dir = path.parent().unwrap_or(claude_dir).to_path_buf();
    let targets: Vec<PathBuf> = read
      .take(path, |body| body)
      .map(|body| imports(body).map(|target| dir.join(target)).collect())
      .unwrap_or_default();
    queue.extend(targets);
  }
  read.texts.sort_by(|a, b| a.path.cmp(&b.path));
  read
}

/// The files `body` imports: each line that is `@` and a relative path with
/// no `..`, and nothing else.
fn imports(body: &str) -> impl Iterator<Item = &Path> {
  body.lines().filter_map(|line| {
    let target = Path::new(line.trim().strip_prefix('@')?);
    let inside = !target.as_os_str().is_empty()
      && target
        .components()
        .all(|c| matches!(c, Component::Normal(_) | Component::CurDir));
    inside.then_some(target)
  })
}

/// The text a person wrote in the project's two contracts, where its
/// `contracts` key puts them, and each `*.md` in its Claude Code memory
/// directory under `claude_dir`.
///
/// **NEVER THE TEXT INTENT'S TEMPLATES GENERATE** (hv decision 91). Intent
/// ships that text to users who do not have hv's global files, so a generated
/// line that a global file also says is not a restatement anyone can remove.
/// A contract is read through [`crate::canon::authored_lines`]: a generated
/// one's user block, line for line.
pub fn project_files(project: &Project, claude_dir: &Path) -> Read {
  let contracts = project.config().contracts;
  let mut read = Read::default();
  for name in CONTRACTS {
    let path = project
      .root()
      .join(crate::sync::root_file_path(name, contracts));
    read.take(path, |body| crate::canon::authored_lines(name, &body));
  }
  let memory = claude_dir
    .join("projects")
    .join(memory_slug(project.root()))
    .join("memory");
  for note in read.list(&memory, |path| path.extension().is_some_and(|e| e == "md")) {
    read.take(note, |body| body);
  }
  read
}

#[cfg(test)]
mod tests {
  use super::*;

  /// A sentence of `n` words, so a test builds its sentence from
  /// [`MIN_WORDS`] rather than writing one by hand.
  fn words(n: usize) -> String {
    let mut text = (0..n)
      .map(|i| format!("word{i}"))
      .collect::<Vec<_>>()
      .join(" ");
    text.push('.');
    text
  }

  /// The 1-based line of `body` that holds `text`.
  fn line_of(body: &str, text: &str) -> usize {
    body
      .lines()
      .position(|line| line.contains(text))
      .expect("the fixture holds the text")
      + 1
  }

  fn write(path: &Path, body: &str) {
    std::fs::create_dir_all(path.parent().expect("a parent")).expect("mkdir");
    std::fs::write(path, body).expect("write");
  }

  /// Issue Ia87820b: a sentence the user block of the project's generated
  /// `CLAUDE.md` or a memory note restates from a global file, one reached
  /// through an `@` import, is found at its line and named with the global
  /// line it repeats. A paraphrase, a sentence shorter than [`MIN_WORDS`] and
  /// a generated line the global file says word for word are not (hv
  /// decision 91). The project's directory name carries a `.` and a `_`, so
  /// its memory is found only under Claude Code's own slug.
  #[test]
  fn a_restated_sentence_is_found_and_a_paraphrase_a_short_one_or_a_generated_one_is_not() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let claude = tmp.path().join("claude");
    let root = tmp.path().join("my_proj.v2");
    crate::init::init(&root, "restated", "test", "3.0.0").expect("init a project");
    let project = Project::open(&root).expect("open the project");
    let claude_md = root.join("CLAUDE.md");
    let generated = std::fs::read_to_string(&claude_md).expect("init writes CLAUDE.md");
    let (_, generated_sentence) = sentences(&generated)
      .into_iter()
      .next()
      .expect("the template renders a sentence long enough to compare");
    let rule = "Never wrap, unwrap or reflow Markdown by hand.";
    let short = words(MIN_WORDS - 1);
    write(&claude.join("CLAUDE.md"), "# Rules\n\n@house-rules.md\n");
    let house = format!("# House\n\n- {rule}\n- {short}\n- {generated_sentence}\n");
    write(&claude.join("house-rules.md"), &house);
    let block = crate::canon::USER_START;
    let edited = generated.replacen(
      block,
      &format!(
        "{block}\n- **{}**\n- Do not hard-wrap Markdown text yourself, ever.\n- {short}",
        rule.to_uppercase()
      ),
      1,
    );
    assert_ne!(
      edited, generated,
      "precondition: the template has a user block"
    );
    write(&claude_md, &edited);
    let note = claude
      .join("projects")
      .join(memory_slug(&root))
      .join("memory/writing.md");
    let noted = format!("{rule}\n");
    write(&note, &noted);

    let global = At {
      path: claude.join("house-rules.md"),
      line: line_of(&house, rule),
    };
    let (global_read, project_read) = (global_files(&claude), project_files(&project, &claude));
    assert!(
      global_read.unread.is_empty() && project_read.unread.is_empty(),
      "precondition: every file was read: {:?} {:?}",
      global_read.unread,
      project_read.unread
    );
    assert_eq!(
      restatements(&global_read.texts, &project_read.texts),
      vec![
        Restatement {
          project: At {
            path: note,
            line: line_of(&noted, rule),
          },
          global: global.clone(),
        },
        Restatement {
          project: At {
            path: claude_md,
            line: line_of(&edited, &rule.to_uppercase()),
          },
          global,
        },
      ],
      "in path order: the memory note under the Claude dir, then the contract's user block"
    );
  }

  /// A sentence of [`MIN_WORDS`] words is compared and one a word short is not.
  #[test]
  fn the_minimum_is_the_constant() {
    assert_eq!(sentences(&words(MIN_WORDS)).len(), 1);
    assert!(sentences(&words(MIN_WORDS - 1)).is_empty());
  }

  /// Claude Code's slug makes every character that is not a letter or a digit
  /// a `-`, not only the `/`.
  #[test]
  fn the_memory_slug_dashes_every_character_that_is_not_a_letter_or_a_digit() {
    assert_eq!(
      memory_slug(Path::new("/Users/m/Devel/prj/Intent")),
      "-Users-m-Devel-prj-Intent"
    );
    assert_eq!(
      memory_slug(Path::new("/Users/m/my_proj.v2")),
      "-Users-m-my-proj-v2"
    );
  }
}
