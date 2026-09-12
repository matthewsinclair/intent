//! The scope rule: which paths the index covers, and which corpus each joins.
//!
//! **PURE. IT DECIDES AND READS NOTHING** (IN-AG-PFIC-001). Every function here
//! answers from a path and the scope object; the walk, the file reads and the
//! staleness comparison are `reconcile`'s, one module over.
//!
//! # Three corpora, and a file is never in none of them
//!
//! ST0069 AC-18.1 and AC-18.5. A path inside the gitignore-aware repository is
//! `canon`, `prose` or `code`, and **a kind this map does not recognise is
//! `code` with no language, never absent** (vc, 2026-09-12): WP-19 routes a
//! file to a table off this column, and a file with no column is a file no
//! surface can report.
//!
//! # What is NOT in the disk corpus, and why it is a rule rather than a path
//!
//! The store's own projections -- the rendered views under `intent/st/**` and
//! the canon extract under `intent/.canon/**` -- are the store's prose seen
//! twice. Indexing them returns every entity hit beside its own rendering.
//! **The views are identified by asking the renderer**, not by matching a path
//! shape, so a view kind added later is excluded on the day it is first
//! rendered and there is nothing to remember.

use std::path::Path;

/// Which index corpus a path belongs to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Corpus {
  /// The store's own entities. Their prose is indexed FROM THE STORE, once.
  Canon,
  /// Prose on disk: markdown and text the repository carries.
  Prose,
  /// Source. `lang` is `None` for a file whose extension this map does not
  /// know -- **it is still in the corpus**, because a file the index cannot
  /// classify is a file it still has to be able to name.
  Code { lang: Option<&'static str> },
}

/// The extension-to-language map.
///
/// **ITS NAMES ARE `critic::HEADLESS_LANGUAGES` AND A TEST PINS THEM THERE.**
/// Two rosters of language names drift in the direction somebody will actually
/// take them, and this estate has already paid for that once, in the pair of
/// literals twenty-six lines apart that `HEADLESS_LANGUAGES` was extracted to
/// end.
///
/// **AND `critic` IS NOT CHANGED TO READ IT** (vc, 2026-09-12, correcting the
/// design). The design said this map already existed where
/// `critic::applies_to_file` reads; measured, it does not -- `applies_to_file`
/// is a glob matcher over a RULE's own `applies_to` list, and a rule's language
/// is its frontmatter's. Those globs are deliberately per-rule, and replacing
/// them with a coarser extension answer would change what every rule applies
/// to. The vocabulary is shared; the mapping is this module's.
pub fn lang_of(path: &Path) -> Option<&'static str> {
  let ext = path.extension()?.to_str()?;
  Some(match ext {
    "rs" => "rust",
    "ex" | "exs" => "elixir",
    "swift" => "swift",
    "lua" => "lua",
    "sh" | "bash" | "zsh" => "shell",
    _ => return None,
  })
}

/// Is this path prose on disk?
fn is_prose(path: &Path) -> bool {
  matches!(
    path.extension().and_then(|e| e.to_str()),
    Some("md" | "markdown" | "txt")
  )
}

/// Which corpus this path joins, or `None` when it is not in the disk corpus at
/// all.
///
/// `views` is every path the renderer produces for this project, which is how
/// the store's projections are excluded by rule. `canon_dir` is the extract's
/// own directory.
pub fn corpus_of(path: &Path, views: &[std::path::PathBuf], canon_dir: &Path) -> Option<Corpus> {
  // **THE PROJECTIONS COME FIRST**, because a rendered view is also markdown
  // and would otherwise be classified as prose by the arm below it.
  if path.starts_with(canon_dir) || views.iter().any(|v| v == path) {
    return None;
  }
  if is_prose(path) {
    return Some(Corpus::Prose);
  }
  Some(Corpus::Code {
    lang: lang_of(path),
  })
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::path::PathBuf;

  #[test]
  fn every_language_this_map_names_is_one_the_estate_declares() {
    for ext in ["rs", "ex", "exs", "swift", "lua", "sh", "bash", "zsh"] {
      let lang = lang_of(&PathBuf::from(format!("a.{ext}"))).expect("a language");
      assert!(
        crate::critic::HEADLESS_LANGUAGES.contains(&lang),
        "`{ext}` maps to `{lang}`, which is not a language this estate declares -- \
         two rosters of language names drift in the direction somebody takes them"
      );
    }
  }

  #[test]
  fn a_kind_this_map_does_not_know_is_still_in_the_corpus() {
    let views: Vec<PathBuf> = Vec::new();
    let canon = PathBuf::from("/p/intent/.canon");
    assert_eq!(
      corpus_of(&PathBuf::from("/p/thing.toml"), &views, &canon),
      Some(Corpus::Code { lang: None }),
      "a file with no language is code with no language, never absent: WP-19 \
       routes off this column and a file with no column is one no surface reports"
    );
    assert_eq!(
      corpus_of(&PathBuf::from("/p/Makefile"), &views, &canon),
      Some(Corpus::Code { lang: None }),
      "and so is a file with no extension at all"
    );
  }

  #[test]
  fn prose_and_code_are_told_apart_by_kind_and_not_by_directory() {
    let views: Vec<PathBuf> = Vec::new();
    let canon = PathBuf::from("/p/intent/.canon");
    assert_eq!(
      corpus_of(&PathBuf::from("/p/intent/wip.md"), &views, &canon),
      Some(Corpus::Prose),
      "a markdown file under `intent/` is prose like any other -- corpus \
       assignment is by kind, not by directory"
    );
    assert_eq!(
      corpus_of(&PathBuf::from("/p/docs/design/note.md"), &views, &canon),
      Some(Corpus::Prose)
    );
    assert_eq!(
      corpus_of(&PathBuf::from("/p/native/rust/src/lib.rs"), &views, &canon),
      Some(Corpus::Code { lang: Some("rust") })
    );
  }

  #[test]
  fn the_stores_own_projections_are_out_by_rule() {
    let view = PathBuf::from("/p/intent/st/ST0001/info.md");
    let views = vec![view.clone()];
    let canon = PathBuf::from("/p/intent/.canon");
    assert_eq!(
      corpus_of(&view, &views, &canon),
      None,
      "a rendered view is the store's prose seen twice"
    );
    assert_eq!(
      corpus_of(
        &PathBuf::from("/p/intent/.canon/st/ST0001.json"),
        &views,
        &canon
      ),
      None,
      "and so is the extract"
    );
    // **THE CONTROL**: the rule is the renderer's answer, not the directory.
    // A file under the same directory that the renderer does not produce stays
    // in the corpus, or the exclusion would be a path-shape hack after all.
    assert_eq!(
      corpus_of(
        &PathBuf::from("/p/intent/st/ST0001/notes.md"),
        &views,
        &canon
      ),
      Some(Corpus::Prose)
    );
  }
}
