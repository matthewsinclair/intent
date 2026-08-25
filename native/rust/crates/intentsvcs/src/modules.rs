//! The module registry -- `intent/llm/MODULES.md`, and the population it is
//! measured against.
//!
//! Two verbs read this: `intent modules find <term>` searches the registry, and
//! `intent modules check` compares it against what is actually on disk. Both are
//! reads; nothing here writes.
//!
//! **THE POPULATION IS DERIVED FROM THE PROJECT'S DECLARED `languages`, AND
//! THAT IS THE ONE DELIBERATE DEVIATION FROM v2.** `bin/intent_modules` scans
//! `bin/intent_*`, `lib/**/*.{ex,sh}` and `intent/plugins/**/bin/*` for every
//! project it is pointed at. That is not a general layout -- it is INTENT'S OWN,
//! and `intent_*` is this product's name used as a file prefix. Shipped to a
//! consumer written in Swift, v2 scans for shell scripts with an `intent_`
//! prefix, finds none, and reports a clean registry. **The check could not fire,
//! and a check that cannot fire reads exactly like a check that passed.**
//!
//! So v3 asks the project what it is written in and scans accordingly. The
//! honest consequence is that a project declaring no code language has an empty
//! population -- and [`Report::scanned`] carries that fact out to the renderer
//! so it is stated rather than presented as a clean bill.
//!
//! **THE GLOB SEMANTICS ARE THE CRITIC'S, NOT A SECOND SET.**
//! [`crate::critic::applies_to_file`] is suffix-anchored (ST0038), so
//! `src/**/*.rs` matches `native/rust/crates/intentsvcs/src/modules.rs` and
//! `lib/**/*.ex` matches `apps/control/lib/foo.ex`. Umbrella and workspace
//! layouts therefore work without a second rule, and there is one answer in this
//! estate to "does this path match this glob" rather than two that agree until
//! they do not.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use thiserror::Error;

/// The registry's path within a project. One home for the literal: `find`,
/// `check` and every error message below reach it here.
pub const REGISTRY: &str = "intent/llm/MODULES.md";

/// Directories the population walk never descends into.
///
/// **Build output, vendored dependencies and scratch space.** Without this the
/// walk reads `target/` (hundreds of thousands of files on a Rust project),
/// `deps/` and `_build/` (other people's source, which is not this project's to
/// register) and `tmp/` (which on this very repo holds whole vendored copies of
/// other Intent projects -- every one of which would be reported unregistered).
///
/// Named rather than pattern-matched on a leading dot, for the reason
/// [`crate::rules::LANGUAGES`] gives: a directory must not join or leave this
/// set by how somebody spells it.
pub const NOT_WALKED: [&str; 9] = [
  ".git",
  ".backup",
  "target",
  "_build",
  "deps",
  "node_modules",
  "tmp",
  ".treeindex",
  ".elixir_ls",
];

/// What went wrong reading the registry.
#[derive(Debug, Error)]
pub enum ModulesError {
  /// The project has no `intent/llm/MODULES.md`.
  ///
  /// v2 calls this an error rather than an empty registry, and it is right to:
  /// an absent registry and a registry with no rows are different states, and
  /// treating the first as the second would report every file in the project as
  /// unregistered.
  #[error("no MODULES.md found at {0}")]
  NoRegistry(String),
  /// The registry exists and could not be read.
  #[error("cannot read {path}: {source}")]
  Unreadable {
    /// The path that failed.
    path: String,
    /// The underlying I/O failure.
    #[source]
    source: std::io::Error,
  },
}

/// The source globs a declared language contributes to the registrable
/// population.
///
/// **EXHAUSTIVE OVER [`crate::rules::LANGUAGES`] BY CONSTRUCTION, and `None` for
/// anything else.** Returning an empty slice for an unknown name would make a
/// typo indistinguishable from a prose language, and both indistinguishable from
/// a language whose globs somebody forgot to add -- three states, one silent
/// answer. `every_language_has_an_answer` in the test module holds the totality;
/// this function is not a fourth hand-written list of language names, because a
/// name that leaves `LANGUAGES` fails that test rather than sitting here
/// unreachable.
///
/// The four empty answers are empty for two different reasons and both are
/// deliberate:
///
/// - `agnostic` and `prose` are [`crate::rules::NON_DECLARABLE`] -- a project
///   never declares them, so reaching them here means a caller passed something
///   `lang init` would have refused.
/// - `author` and `content` are declarable and are PROSE disciplines. A chapter
///   is not a module and does not belong in a Highlander registry.
pub fn source_globs(lang: &str) -> Option<&'static [&'static str]> {
  match lang {
    "elixir" => Some(&["lib/**/*.ex"]),
    "rust" => Some(&["src/**/*.rs"]),
    "shell" => Some(&["bin/*", "lib/**/*.sh"]),
    "lua" => Some(&["src/**/*.lua"]),
    "swift" => Some(&["Sources/**/*.swift"]),
    "author" | "content" => Some(&[]),
    "agnostic" | "prose" => Some(&[]),
    _ => None,
  }
}

/// The paths a registry declares.
///
/// Column 2 -- "THE Module" -- of every table row that is not a header, a
/// separator or an HTML comment, taking the backtick-quoted span inside it.
///
/// **The FIRST span rather than the last, and the choice is currently
/// unobservable**: measured on this repo's own registry, 242 rows carry a
/// backticked column 2 and not one carries two spans. v2's `sed` takes the last
/// through a greedy `.*`; recording the difference matters more than the value,
/// because the first row with two spans is where they would start disagreeing.
pub fn registered_paths(text: &str) -> BTreeSet<String> {
  let mut out = BTreeSet::new();
  for line in text.lines() {
    if !line.contains('|') || line.trim_start().starts_with("<!--") {
      continue;
    }
    let cells: Vec<&str> = line.split('|').collect();
    if cells.len() < 3 {
      continue;
    }
    let cell = cells[2];
    if cell.trim() == "Concern" || cell.trim().starts_with("---") {
      continue;
    }
    // The header row is identified by its own column-1 label rather than by
    // position, because a registry may carry several tables.
    if cells[1].trim() == "Concern" {
      continue;
    }
    if let Some(span) = first_backticked(cell) {
      out.insert(span);
    }
  }
  out
}

/// The first backtick-quoted span in a cell, if any.
fn first_backticked(cell: &str) -> Option<String> {
  let open = cell.find('`')?;
  let rest = &cell[open + 1..];
  let close = rest.find('`')?;
  Some(rest[..close].to_string())
}

/// Registry rows matching a term, case-insensitively.
///
/// v2 greps the whole file and then keeps the lines containing `|`, which is
/// table rows plus any prose line that happens to hold a pipe. Ported as
/// observed: the term is matched against the WHOLE line, not just the module
/// column, so searching for a word that appears only in a row's Notes finds that
/// row -- which is the behaviour a reader looking for "who owns X" wants.
pub fn find_rows(text: &str, term: &str) -> Vec<String> {
  let needle = term.to_lowercase();
  text
    .lines()
    .filter(|l| l.contains('|') && l.to_lowercase().contains(&needle))
    .map(|l| l.to_string())
    .collect()
}

/// Everything `check` found.
pub struct Report {
  /// On disk, matched by a declared language's globs, absent from the registry.
  pub unregistered: Vec<String>,
  /// Declared by the registry, absent from disk.
  pub stale: Vec<String>,
  /// The declared languages that contributed globs, and how many each did.
  ///
  /// **Carried out rather than summed to a boolean** so the renderer can say
  /// what was scanned. A count of zero issues means nothing until the reader
  /// knows whether anything was looked at.
  pub scanned: Vec<(String, usize)>,
}

impl Report {
  /// Did the check find anything to report?
  pub fn clean(&self) -> bool {
    self.unregistered.is_empty() && self.stale.is_empty()
  }

  /// Did any declared language contribute a glob?
  ///
  /// False means the unregistered half could not have fired -- the state this
  /// module exists to stop reading as a pass.
  pub fn scanned_anything(&self) -> bool {
    self.scanned.iter().any(|(_, n)| *n > 0)
  }
}

/// Read a project's registry.
pub fn read_registry(root: &Path) -> Result<String, ModulesError> {
  let path = root.join(REGISTRY);
  if !path.is_file() {
    return Err(ModulesError::NoRegistry(REGISTRY.to_string()));
  }
  std::fs::read_to_string(&path).map_err(|source| ModulesError::Unreadable {
    path: REGISTRY.to_string(),
    source,
  })
}

/// Every file under `root` that a declared language's globs admit.
///
/// Returns project-relative paths, sorted, with the per-language contribution
/// counts [`Report::scanned`] carries.
pub fn population(root: &Path, languages: &[String]) -> (BTreeSet<String>, Vec<(String, usize)>) {
  let mut globs: Vec<String> = Vec::new();
  let mut per_lang: Vec<(String, Vec<String>)> = Vec::new();
  for lang in languages {
    let owned: Vec<String> = source_globs(lang)
      .unwrap_or(&[])
      .iter()
      .map(|g| (*g).to_string())
      .collect();
    globs.extend(owned.clone());
    per_lang.push((lang.clone(), owned));
  }

  let mut files: Vec<PathBuf> = Vec::new();
  if !globs.is_empty() {
    walk(root, root, &mut files);
  }

  let mut all = BTreeSet::new();
  let mut counts: Vec<(String, usize)> = Vec::new();
  for (lang, lang_globs) in &per_lang {
    // **EMPTY MEANS NOTHING HERE AND MEANS EVERYTHING THERE, AND THE PREDICATE
    // IS THE THERE.** [`crate::critic::applies_to_file`] documents an absent
    // `applies_to` as UNIVERSAL -- correct for a rule, because a rule that
    // declares no scope checks every file. A language that contributes no
    // source globs must contribute no FILES, which is the exact opposite.
    //
    // Measured before this guard existed: `author` and `content` -- both
    // deliberately empty -- each matched 1634 files, and `intent modules check`
    // on this repo reported 1442 issues including `.DS_Store`. Reusing one glob
    // semantics was still right; what it cost was checking the polarity at the
    // boundary, because the two callers disagree about what "nothing declared"
    // means and both readings are defensible in their own home.
    if lang_globs.is_empty() {
      counts.push((lang.clone(), 0));
      continue;
    }
    let mut n = 0usize;
    for f in &files {
      let rel = match f.strip_prefix(root) {
        Ok(r) => r,
        Err(_) => continue,
      };
      if crate::critic::applies_to_file(lang_globs, rel) {
        all.insert(rel.to_string_lossy().to_string());
        n += 1;
      }
    }
    counts.push((lang.clone(), n));
  }
  (all, counts)
}

/// Depth-first walk, skipping [`NOT_WALKED`] and symlinks.
///
/// Symlinks are skipped rather than followed because a link into a sibling
/// checkout -- which this estate has -- would pull another project's whole tree
/// into this project's population.
fn walk(root: &Path, dir: &Path, out: &mut Vec<PathBuf>) {
  let entries = match std::fs::read_dir(dir) {
    Ok(e) => e,
    // A directory that cannot be read contributes nothing. It is not an error:
    // the walk covers whatever the caller can see, and a permission-denied
    // subtree is not a registry defect.
    Err(_) => return,
  };
  for entry in entries.flatten() {
    let path = entry.path();
    let meta = match entry.file_type() {
      Ok(m) => m,
      Err(_) => continue,
    };
    if meta.is_symlink() {
      continue;
    }
    let name = entry.file_name();
    let name = name.to_string_lossy();
    if meta.is_dir() {
      if NOT_WALKED.contains(&name.as_ref()) {
        continue;
      }
      walk(root, &path, out);
    } else if meta.is_file() {
      out.push(path);
    }
  }
}

/// Compare a registry against the filesystem.
pub fn check(root: &Path, languages: &[String]) -> Result<Report, ModulesError> {
  let text = read_registry(root)?;
  let declared = registered_paths(&text);
  let (found, scanned) = population(root, languages);

  // **A DIRECTORY ROW REGISTERS ITS WHOLE SUBTREE**, so a file under one is
  // registered even though its own path appears nowhere. v2 builds the same
  // `registered_dirs` list and skips beneath it; dropping that reported every
  // one of this repo's `lib/templates/ext-seeds/` seed files as unregistered --
  // template source shipped INTO other projects, which is exactly what a
  // directory row exists to cover in one line instead of forty.
  //
  // Applied to the whole population rather than to v2's two `lib/**` arms only.
  // A subtree is registered or it is not; making that depend on which glob found
  // the file would mean the same path answered differently by route.
  let dirs: Vec<String> = declared
    .iter()
    .filter(|p| p.ends_with('/'))
    .cloned()
    .collect();
  let unregistered: Vec<String> = found
    .iter()
    .filter(|f| !declared.contains(*f))
    .filter(|f| !dirs.iter().any(|d| f.starts_with(d.as_str())))
    .cloned()
    .collect();

  let mut stale: Vec<String> = Vec::new();
  for path in &declared {
    // A directory row registers a whole subtree and is satisfied by the
    // directory existing.
    if path.ends_with('/') {
      if !root.join(path.trim_end_matches('/')).is_dir() {
        stale.push(path.clone());
      }
      continue;
    }
    // `file::function` registers one function inside a file, so BOTH halves
    // must hold -- v2.11.12's rule, ported because a helper registered against
    // a function name is otherwise reported missing.
    if let Some((file, func)) = path.split_once("::") {
      let target = root.join(file);
      let defined = std::fs::read_to_string(&target)
        .map(|body| function_is_defined(&body, func))
        .unwrap_or(false);
      if !defined {
        stale.push(path.clone());
      }
      continue;
    }
    if !root.join(path).exists() {
      stale.push(path.clone());
    }
  }

  Ok(Report {
    unregistered,
    stale,
    scanned,
  })
}

/// Is `func` defined in this file?
///
/// v2 greps `^<name>\(\)`, which is shell's definition syntax. Kept exactly:
/// widening it to other languages would change which rows go stale, and that is
/// a behaviour change wearing a generalisation's clothes.
fn function_is_defined(body: &str, func: &str) -> bool {
  let needle = format!("{func}()");
  body.lines().any(|l| l.starts_with(&needle))
}

impl crate::remedy::Remedy for ModulesError {
  fn remedy(&self) -> String {
    match self {
      // The registry is laid down by the canon, so the actionable answer is
      // almost always "this project has not had canon applied", not "write the
      // file by hand". Naming `upgrade` first keeps the operator on the path the
      // tool already owns.
      ModulesError::NoRegistry(path) => format!(
        "`{path}` is the module registry the canon lays down. Run `intent upgrade` if this project predates it, or create the file if this project has never carried one."
      ),
      ModulesError::Unreadable { path, .. } => format!(
        "`{path}` exists and could not be opened -- check its permissions with `ls -l {path}`."
      ),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  /// The registry used by the parse tests. Two tables, a comment, a directory
  /// row and a function-qualified row -- the shapes the parser must survive.
  const FIXTURE: &str = r#"# Modules

<!-- | Commented | `bin/never` | not a row | -->

| Concern     | THE Module              | Notes            |
| ----------- | ----------------------- | ---------------- |
| Helpers     | `bin/intent_helpers`    | shared helpers   |
| A function  | `bin/intent_helpers::error` | one function |
| A subtree   | `lib/templates/seeds/`  | whole directory  |

| Concern     | THE Module              | Notes            |
| ----------- | ----------------------- | ---------------- |
| Second table| `lib/other.sh`          | still a row      |
"#;

  #[test]
  fn the_parse_takes_column_two_and_skips_headers_separators_and_comments() {
    let got = registered_paths(FIXTURE);
    let want: BTreeSet<String> = [
      "bin/intent_helpers",
      "bin/intent_helpers::error",
      "lib/templates/seeds/",
      "lib/other.sh",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    assert_eq!(got, want);
    // The commented row is the one a `grep '|'` port lets through.
    assert!(!got.contains("bin/never"));
  }

  /// **THE TOTALITY ARM.** A language that leaves [`crate::rules::LANGUAGES`],
  /// or joins it without an arm here, fails HERE rather than answering `None`
  /// somewhere a caller reads as "no files".
  #[test]
  fn every_language_has_an_answer() {
    for lang in crate::rules::LANGUAGES {
      assert!(
        source_globs(lang).is_some(),
        "`{lang}` is in rules::LANGUAGES and has no source_globs arm"
      );
    }
  }

  #[test]
  fn a_name_that_is_not_a_language_is_none_rather_than_empty() {
    // The discriminator that keeps a typo distinguishable from a prose
    // language: both would be "no files" if this returned an empty slice.
    assert!(source_globs("rusty").is_none());
    assert!(source_globs("").is_none());
    assert_eq!(source_globs("author"), Some(&[][..]));
  }

  #[test]
  fn the_non_declarable_packs_contribute_nothing() {
    for lang in crate::rules::NON_DECLARABLE {
      assert_eq!(
        source_globs(lang),
        Some(&[][..]),
        "`{lang}` is non-declarable and must contribute no population"
      );
    }
  }

  /// **THE REGRESSION THAT PAID FOR ITSELF ON THE FIRST DRIVE.**
  /// [`crate::critic::applies_to_file`] treats an empty glob list as UNIVERSAL,
  /// which is right for a rule and inverted for a population. Before the guard,
  /// `author` and `content` each matched every file in the project: `intent
  /// modules check` reported 1442 issues on this repo, `.DS_Store` among them.
  #[test]
  fn a_language_with_no_globs_matches_no_files() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    std::fs::create_dir_all(root.join("lib")).expect("mkdir");
    std::fs::write(root.join("lib/a.ex"), "x").expect("write");
    std::fs::write(root.join("README.md"), "x").expect("write");

    // **`author` IS DECLARED ALONGSIDE A LANGUAGE THAT POPULATES, AND THAT IS
    // THE WHOLE TEST.** With only `author` declared, the union of globs is empty,
    // `population` never walks, and the guard has nothing to filter -- so the
    // first version of this test passed with the guard DELETED and rebuilt.
    // The live defect needed exactly this shape: Intent declares
    // `elixir, author, content, rust, shell`, the code languages fill `files`,
    // and the prose ones then matched every entry in it.
    let (files, counts) = population(root, &["elixir".to_string(), "author".to_string()]);
    assert_eq!(
      counts,
      vec![("elixir".to_string(), 1), ("author".to_string(), 0)],
      "author matched files a code language put in the walk"
    );
    assert!(files.contains("lib/a.ex"));
    assert!(
      !files.contains("README.md"),
      "a prose language pulled in {files:?}"
    );
  }

  #[test]
  fn a_declared_language_finds_its_own_sources_at_any_depth() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    std::fs::create_dir_all(root.join("apps/control/lib")).expect("mkdir");
    std::fs::create_dir_all(root.join("lib")).expect("mkdir");
    std::fs::write(root.join("lib/top.ex"), "x").expect("write");
    std::fs::write(root.join("apps/control/lib/deep.ex"), "x").expect("write");
    std::fs::write(root.join("lib/notelixir.md"), "x").expect("write");

    let (files, counts) = population(root, &["elixir".to_string()]);
    // ST0038's umbrella rule, inherited from the critic's suffix anchoring.
    assert!(files.contains("apps/control/lib/deep.ex"));
    assert!(files.contains("lib/top.ex"));
    assert!(!files.contains("lib/notelixir.md"));
    assert_eq!(counts, vec![("elixir".to_string(), 2)]);
  }

  #[test]
  fn build_output_is_not_walked() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    std::fs::create_dir_all(root.join("target/debug/lib")).expect("mkdir");
    std::fs::write(root.join("target/debug/lib/built.ex"), "x").expect("write");
    let (files, _) = population(root, &["elixir".to_string()]);
    assert!(files.is_empty(), "walked build output: {files:?}");
  }

  #[test]
  fn a_directory_row_registers_its_subtree() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    std::fs::create_dir_all(root.join("intent/llm")).expect("mkdir");
    std::fs::create_dir_all(root.join("lib/seeds")).expect("mkdir");
    std::fs::write(root.join("lib/seeds/one.sh"), "x").expect("write");
    std::fs::write(root.join("lib/loose.sh"), "x").expect("write");
    std::fs::write(
      root.join("intent/llm/MODULES.md"),
      "| Concern | THE Module | Notes |\n| --- | --- | --- |\n| Seeds | `lib/seeds/` | subtree |\n",
    )
    .expect("write");

    let report = check(root, &["shell".to_string()]).expect("check");
    assert!(
      !report.unregistered.iter().any(|f| f.contains("seeds")),
      "a file under a directory row was reported unregistered: {:?}",
      report.unregistered
    );
    assert!(report.unregistered.contains(&"lib/loose.sh".to_string()));
  }

  #[test]
  fn a_function_qualified_row_needs_the_file_and_the_function() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    std::fs::create_dir_all(root.join("intent/llm")).expect("mkdir");
    std::fs::create_dir_all(root.join("bin")).expect("mkdir");
    std::fs::write(root.join("bin/helpers"), "error() {\n  exit 1\n}\n").expect("write");
    std::fs::write(
      root.join("intent/llm/MODULES.md"),
      "| Concern | THE Module | Notes |\n| --- | --- | --- |\n| Live | `bin/helpers::error` | here |\n| Gone | `bin/helpers::vanished` | not here |\n",
    )
    .expect("write");

    let report = check(root, &[]).expect("check");
    assert!(report.stale.contains(&"bin/helpers::vanished".to_string()));
    assert!(!report.stale.contains(&"bin/helpers::error".to_string()));
  }

  #[test]
  fn an_absent_registry_is_an_error_rather_than_an_empty_one() {
    let dir = tempfile::tempdir().expect("tempdir");
    // An empty registry would report every source file unregistered; the
    // distinction is the whole reason this is a typed error.
    assert!(matches!(
      check(dir.path(), &["shell".to_string()]),
      Err(ModulesError::NoRegistry(_))
    ));
  }

  #[test]
  fn find_matches_the_whole_row_case_insensitively() {
    let rows = find_rows(FIXTURE, "SHARED HELPERS");
    assert_eq!(rows.len(), 1, "{rows:?}");
    assert!(rows[0].contains("bin/intent_helpers"));
    // The term is matched against the row, not just the module column -- a
    // reader searching the Notes for who owns something is the point.
    assert_eq!(find_rows(FIXTURE, "whole directory").len(), 1);
    assert!(find_rows(FIXTURE, "zzzznope").is_empty());
  }

  #[test]
  fn a_report_that_scanned_nothing_says_so() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    std::fs::create_dir_all(root.join("intent/llm")).expect("mkdir");
    std::fs::write(
      root.join("intent/llm/MODULES.md"),
      "| Concern | THE Module | Notes |\n| --- | --- | --- |\n",
    )
    .expect("write");

    let report = check(root, &["author".to_string()]).expect("check");
    assert!(report.clean(), "nothing to find, so nothing is found");
    // **AND `clean()` ALONE WOULD BE A LIE HERE.** The pair is the contract:
    // clean AND scanned-nothing is a stated no-op, not a pass.
    assert!(!report.scanned_anything());
  }
}
