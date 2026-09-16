//! The scope rule: which paths the index covers, and which corpus each joins.
//!
//! **AT-18.5** is `the_stores_own_projections_are_out_by_rule`: a rendered view
//! and the canon extract are the store's prose seen twice, so they are not in
//! the disk corpus, and the rule asks the RENDERER rather than a path shape.
//! `index::reconcile`'s survey arm drives the same rule over a real tree.
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
//! **EVERY DOCUMENT THE STORE ALREADY CARRIES PROSE FOR IS OUT**, because the
//! store indexes it once from its own rows and a second copy on disk is the
//! same bytes answering twice. That is the canon extract under
//! `intent/.canon/**`, the rendered views under `intent/st/**`, and the
//! authored documents the store carries as attachments -- a thread's
//! `design.md`, `impl.md` and `tasks.md`.
//!
//! **THE EXCLUSION WAS THE PROJECTIONS ONLY AND THAT WAS NOT WIDE ENOUGH**
//! (issue 0304). A view is a projection: the renderer produces it, so nobody
//! authored it and excluding it loses nothing. An attachment is AUTHORED, so it
//! is not a view -- and the store carries it anyway, which is the fact that
//! decides the question. Neither corpus was wrong about its own scope; the
//! overlap was between the two scopes and belonged to neither, so one document
//! realised on disk answered a search twice, once as `file` and once as
//! `thread`, same path and same line.
//!
//! **THE SET IS OBTAINED BY ASKING THE TWO AUTHORITIES**, not by matching a
//! path shape: the renderer says which views exist, canon's attachment rows say
//! which documents it carries. So a view kind added later, or a newly attached
//! document, is excluded on the day it first exists and there is nothing to
//! remember.
//!
//! **AND THE ANSWER IS NOT WHERE SCOPE IS DECIDED.** The other shape available
//! was to let the corpora overlap and dedupe the doubled row at the answer;
//! ruled against, because a search that has to remember not to say one thing
//! twice is a search whose scope nobody can state.

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

impl Corpus {
  /// The stored spelling, which is also how a report names it.
  pub fn as_str(&self) -> &'static str {
    match self {
      Corpus::Canon => "canon",
      Corpus::Prose => "prose",
      Corpus::Code { .. } => "code",
    }
  }
}

/// The corpus a stored spelling names, or `None` for a word this build does
/// not know.
///
/// **THE STORED STRING IS THE NAMESPACE AND THIS IS THE WAY BACK** (vc,
/// 2026-09-12). A reader that matched on the words itself would be a second
/// roster; a row naming a corpus this build has never heard of is a row no
/// surface can classify, and `None` says so rather than guessing.
///
/// `lang` is not recoverable from the corpus name alone -- it is a column of
/// its own -- so a code corpus comes back with none.
pub fn named(word: &str) -> Option<Corpus> {
  Some(match word {
    "canon" => Corpus::Canon,
    "prose" => Corpus::Prose,
    "code" => Corpus::Code { lang: None },
    _ => return None,
  })
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
/// `carried` is every path the store already carries prose for -- the renderer's
/// views and the documents attached to a thread -- which is how the store's own
/// prose is excluded by rule rather than by path shape. `canon_dir` is the
/// extract's own directory.
pub fn corpus_of(path: &Path, carried: &[std::path::PathBuf], canon_dir: &Path) -> Option<Corpus> {
  // **WHAT THE STORE CARRIES COMES FIRST**, because a rendered view and an
  // attached document are both markdown and would otherwise be classified as
  // prose by the arm below it.
  if path.starts_with(canon_dir) || carried.iter().any(|v| v == path) {
    return None;
  }
  if is_prose(path) {
    return Some(Corpus::Prose);
  }
  Some(Corpus::Code {
    lang: lang_of(path),
  })
}

/// Why the index holds no content for a file that IS in scope.
///
/// **A SKIP IS A ROW, NOT AN ABSENCE** (AC-18.2). Every one of these gets a
/// `file_index` row carrying the reason, and `intent index status` lists them,
/// because the alternative is a user searching for something that is on disk,
/// getting nothing, and having nothing to read that explains it. A silent
/// exclusion is indistinguishable from a broken index.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkipReason {
  /// A NUL byte in the sampled block, which is how grep decides the same
  /// question.
  Binary,
  /// Larger than the cap. See [`DEFAULT_MAX_FILE_BYTES`].
  TooLarge,
  /// A symlink. Its target is either inside the corpus, where it is indexed
  /// once at its real path, or outside it, where following the link would take
  /// the index out of the repository it is a statement about. Neither wants a
  /// second copy under this name.
  Symlink,
  /// The bytes could not be read. **A reason and not an error**: one
  /// unreadable file must not fail a reconcile over thousands.
  Unreadable,
}

impl SkipReason {
  /// The stored spelling. Kebab-case, as every other enumerated value the
  /// store holds is.
  pub fn as_str(self) -> &'static str {
    match self {
      SkipReason::Binary => "binary",
      SkipReason::TooLarge => "too-large",
      SkipReason::Symlink => "symlink",
      SkipReason::Unreadable => "unreadable",
    }
  }

  /// The reason a stored spelling names, or `None` for a spelling this build
  /// does not write.
  pub fn parse(stored: &str) -> Option<Self> {
    Some(match stored {
      "binary" => SkipReason::Binary,
      "too-large" => SkipReason::TooLarge,
      "symlink" => SkipReason::Symlink,
      "unreadable" => SkipReason::Unreadable,
      _ => return None,
    })
  }

  /// **WHETHER A TEXT ANSWER THAT SKIPPED THIS FILE IS PARTIAL** (issue 0430).
  /// A binary file holds no text a query could match, and a symlink's target
  /// is indexed at its real path or lies outside the repository, so both are
  /// skipped by policy and the answer is still whole. An oversized or
  /// unreadable file holds text the index never read, so an answer that
  /// skipped one is not. The match is exhaustive so a new reason is decided
  /// here rather than defaulted.
  pub fn leaves_a_gap(self) -> bool {
    match self {
      SkipReason::Binary | SkipReason::Symlink => false,
      SkipReason::TooLarge | SkipReason::Unreadable => true,
    }
  }
}

/// How much of a file is read to decide whether it is binary.
///
/// One block. A file whose first 8 KiB are NUL-free and which turns binary
/// later is indexed as text, and the cost of that is bounded and small -- a
/// few junk tokens in one file's rows -- where the cost of reading every byte
/// of every file to be sure is paid on every reconcile of every project.
pub const BINARY_SAMPLE_BYTES: usize = 8 * 1024;

/// The default size cap, above which a file is in scope and not indexed.
///
/// **MEASURED ON THIS REPOSITORY RATHER THAN CHOSEN** (2026-09-12). Over the
/// tracked tree outside the canon extract -- the extract is not in the disk
/// corpus -- the largest file is `intent/llm/MODULES.md` at 846,473 bytes, and
/// five files exceed 512 KiB. Nothing reaches 1 MiB. The default is roughly
/// five times the largest real file, which keeps every legitimate document in
/// the index while still stopping the case the cap exists for: one vendored
/// bundle or generated blob whose FTS rows cost more than the rest of the
/// project put together (D34: the index is roughly twice its corpus).
///
/// It is a config value because the measurement is of THIS estate and a
/// project with larger documents is not wrong.
pub const DEFAULT_MAX_FILE_BYTES: u64 = 4 * 1024 * 1024;

/// Does this sample say the file is binary?
///
/// The rule is grep's: a NUL byte in the block read. Kept pure and separate
/// from the read so the decision can be driven on bytes rather than on a
/// fixture's filesystem.
pub fn looks_binary(sample: &[u8]) -> bool {
  sample.contains(&0)
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

  #[test]
  fn binary_is_a_nul_and_not_merely_a_high_byte() {
    assert!(looks_binary(b"\xff\xfe\x00Bud1"), "a NUL says binary");
    assert!(
      !looks_binary("a caf\u{e9}, an emoji \u{1f600}, a tab\t and a CR\r".as_bytes()),
      "high bytes, punctuation and control characters are TEXT -- an earlier \
       fixture in this estate used \\x00\\x01 as `not UTF-8` and proved nothing \
       because those are valid UTF-8 control characters"
    );
    assert!(!looks_binary(b""), "an empty file is text, not binary");
  }

  #[test]
  fn every_skip_reason_has_its_own_stored_spelling() {
    let all = [
      SkipReason::Binary,
      SkipReason::TooLarge,
      SkipReason::Symlink,
      SkipReason::Unreadable,
    ];
    for reason in all {
      assert_eq!(
        SkipReason::parse(reason.as_str()),
        Some(reason),
        "the spelling reads back"
      );
    }
    let mut seen: Vec<&str> = all.iter().map(|r| r.as_str()).collect();
    seen.sort_unstable();
    seen.dedup();
    assert_eq!(
      seen.len(),
      all.len(),
      "two reasons sharing a spelling would make `intent index status` unable \
       to tell them apart, which is the whole of what it reports"
    );
  }
}
