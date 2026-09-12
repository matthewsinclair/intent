//! The impure rim: what the index has to touch the filesystem to know.
//!
//! **IT DECIDES NOTHING IT CAN ASK [`super::corpus`] INSTEAD** (IN-AG-PFIC-001).
//! The rules -- which corpus a path joins, what counts as binary, what the cap
//! is -- are pure and live one module over, driven on values. What is here is
//! the reading: a `stat` that must not follow a link, a bounded read that can
//! fail, and the ordering between them.

use std::io::Read;
use std::path::Path;

use super::corpus::{BINARY_SAMPLE_BYTES, Corpus, SkipReason, corpus_of, looks_binary};
use super::freshness::Stamp;
use crate::sync::{Scanned, SyncError, repository_files};

/// Why the index will hold no content for this in-scope file, or `None` when it
/// will hold it.
///
/// **THE ORDER OF THE FOUR CHECKS IS LOAD-BEARING AND IS NOT A PREFERENCE.**
///
/// - The symlink question comes first because it is the only one that must not
///   follow the link. `metadata` follows; `symlink_metadata` does not, and
///   asking about size or bytes first would silently answer about the TARGET --
///   including a target outside the repository, which is the case the skip
///   exists for.
/// - The cap comes before the read, because the read is what the cap exists to
///   avoid paying for.
/// - Unreadable comes before binary because a file that could not be read has
///   no bytes to judge, and calling it text by default is how an unreadable
///   file becomes a silently empty index entry.
///
/// **AN ERROR HERE IS A REASON AND NEVER A FAILURE** (IN-AG-NO-SILENT-001 read
/// in the right direction). Nothing is swallowed: an I/O error becomes
/// [`SkipReason::Unreadable`], which is reported to the operator by
/// `intent index status` as a row. One unreadable file must not fail a
/// reconcile over thousands, and it must not vanish either.
pub fn skip_for(path: &Path, max_bytes: u64) -> Option<SkipReason> {
  let Ok(meta) = std::fs::symlink_metadata(path) else {
    return Some(SkipReason::Unreadable);
  };
  if meta.file_type().is_symlink() {
    return Some(SkipReason::Symlink);
  }
  if meta.len() > max_bytes {
    return Some(SkipReason::TooLarge);
  }
  let Ok(file) = std::fs::File::open(path) else {
    return Some(SkipReason::Unreadable);
  };
  let mut sample = Vec::with_capacity(BINARY_SAMPLE_BYTES);
  if file
    .take(BINARY_SAMPLE_BYTES as u64)
    .read_to_end(&mut sample)
    .is_err()
  {
    return Some(SkipReason::Unreadable);
  }
  looks_binary(&sample).then_some(SkipReason::Binary)
}

/// What a stat says about this file now, or `None` when it cannot be stat'd.
///
/// **THE SPELLING IS `sync`'s AND IS NOT RESTATED HERE.** The recorded stamp
/// this is compared against was written by the change detector, so a second
/// formatting of the same instant would make every file look modified on the
/// first reconcile after either moved -- and the symptom would be a slow index
/// rather than an error.
pub fn stamp_of(path: &Path) -> Option<Stamp> {
  crate::sync::stamp_of(path)
    .ok()
    .map(|(size, mtime)| Stamp { size, mtime })
}

/// One in-scope path, as the index sees it before it holds any content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Surveyed {
  /// Relative to the project root, as `file_index` stores a path.
  pub path: String,
  pub corpus: Corpus,
  /// `None` when the index will hold this file's content.
  pub skipped: Option<SkipReason>,
}

/// Every path in the index's scope, with the corpus it joins and the reason the
/// index will not hold it, if there is one.
///
/// **THE STORE'S OWN PROJECTIONS ARE NOT IN THE RESULT AT ALL**, which is the
/// one absence here that is correct: a rendered view and the canon extract are
/// the store's prose seen twice, so they are not files the index declined --
/// they are not files the index is about. Everything else in scope gets a row,
/// skipped ones included.
///
/// `views` is every path the renderer produces for this project and `canon_dir`
/// is the extract's directory; both are the caller's to supply, so this can be
/// driven without a store.
pub fn survey(
  root: &Path,
  views: &[std::path::PathBuf],
  canon_dir: &Path,
  max_bytes: u64,
) -> Result<Vec<Surveyed>, SyncError> {
  let scope = Scanned::for_root(root);
  let mut out = Vec::new();
  for path in repository_files(root, &scope)? {
    let Some(corpus) = corpus_of(&path, views, canon_dir) else {
      continue;
    };
    out.push(Surveyed {
      path: crate::project::relative(root, &path),
      corpus,
      skipped: skip_for(&path, max_bytes),
    });
  }
  Ok(out)
}

#[cfg(test)]
mod tests {
  use super::super::corpus::DEFAULT_MAX_FILE_BYTES;
  use super::*;

  fn at(dir: &tempfile::TempDir, name: &str) -> std::path::PathBuf {
    dir.path().join(name)
  }

  #[test]
  fn a_file_the_index_can_hold_is_not_skipped() {
    // **THE CONTROL, FIRST.** Without it every arm below passes under a
    // function that skips everything.
    let dir = tempfile::tempdir().expect("tempdir");
    let path = at(&dir, "notes.md");
    std::fs::write(&path, "# a heading\n\nand a paragraph.\n").expect("write");
    assert_eq!(skip_for(&path, DEFAULT_MAX_FILE_BYTES), None);
  }

  #[test]
  fn a_nul_makes_it_binary_and_the_bytes_around_it_do_not() {
    let dir = tempfile::tempdir().expect("tempdir");
    let binary = at(&dir, "artefact.bin");
    std::fs::write(&binary, b"\xff\xfe\x00Bud1\xff\xfe").expect("write");
    assert_eq!(
      skip_for(&binary, DEFAULT_MAX_FILE_BYTES),
      Some(SkipReason::Binary)
    );

    // Not UTF-8 and not binary: the rule is grep's NUL, not decodability. A
    // Latin-1 file is prose somebody wrote and the index can hold it.
    let latin1 = at(&dir, "notes.txt");
    std::fs::write(&latin1, b"caf\xe9 au lait\n").expect("write");
    assert_eq!(skip_for(&latin1, DEFAULT_MAX_FILE_BYTES), None);
  }

  #[test]
  fn the_cap_is_a_cap_and_the_file_at_it_is_held() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = at(&dir, "big.md");
    std::fs::write(&path, vec![b'x'; 64]).expect("write");
    assert_eq!(
      skip_for(&path, 63),
      Some(SkipReason::TooLarge),
      "one byte over the cap is over the cap"
    );
    assert_eq!(
      skip_for(&path, 64),
      None,
      "and the file exactly AT the cap is held, or the cap would be off by one \
       in the direction that quietly drops content"
    );
  }

  #[test]
  fn a_symlink_is_skipped_and_its_target_is_not() {
    let dir = tempfile::tempdir().expect("tempdir");
    let real = at(&dir, "real.md");
    std::fs::write(&real, "# real\n").expect("write");
    let link = at(&dir, "link.md");
    std::os::unix::fs::symlink(&real, &link).expect("symlink");

    assert_eq!(
      skip_for(&link, DEFAULT_MAX_FILE_BYTES),
      Some(SkipReason::Symlink),
      "the link is skipped -- indexing it would hold the same bytes twice, \
       under a name that is not where they live"
    );
    assert_eq!(
      skip_for(&real, DEFAULT_MAX_FILE_BYTES),
      None,
      "and the target is held once, at its real path"
    );
  }

  #[test]
  fn an_unreadable_file_is_a_reason_rather_than_an_error() {
    use std::os::unix::fs::PermissionsExt;
    let dir = tempfile::tempdir().expect("tempdir");
    let path = at(&dir, "sealed.md");
    std::fs::write(&path, "# sealed\n").expect("write");
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o000)).expect("chmod");

    // **THE PRECONDITION IS ASSERTED, because a run as root reads it anyway**
    // and this arm would then be measuring nothing while passing.
    assert!(
      std::fs::read(&path).is_err(),
      "precondition: this file must actually be unreadable to this process"
    );
    assert_eq!(
      skip_for(&path, DEFAULT_MAX_FILE_BYTES),
      Some(SkipReason::Unreadable),
      "an I/O error is reported as a reason, not raised as a failure and not \
       swallowed into `None`"
    );

    // Left readable so the tempdir can clean itself up.
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).expect("chmod back");
  }

  #[test]
  fn an_absent_path_is_unreadable_rather_than_indexable() {
    let dir = tempfile::tempdir().expect("tempdir");
    assert_eq!(
      skip_for(&at(&dir, "never-written.md"), DEFAULT_MAX_FILE_BYTES),
      Some(SkipReason::Unreadable),
      "a path that vanished between the walk and the read is the same class of \
       answer as one that cannot be opened"
    );
  }

  #[test]
  fn a_same_size_same_mtime_rewrite_is_caught_by_one_policy_and_missed_by_the_other() {
    // **THE MISSED-EDIT PAIR, DRIVEN ON A REAL FILE RATHER THAN ON TWO
    // STRUCTS.** The pure arms in `freshness` assert the decision; this asserts
    // that the case they describe is one a filesystem actually produces --
    // same length, mtime put back, different bytes.
    use super::super::freshness::{Policy, must_read};

    let dir = tempfile::tempdir().expect("tempdir");
    let path = at(&dir, "src.rs");
    std::fs::write(&path, "fn a() {}\n").expect("write");
    let before = stamp_of(&path).expect("stamp");
    let was = crate::sync::file_sha256(&path).expect("hash");
    let times = std::fs::FileTimes::new().set_modified(
      std::fs::metadata(&path)
        .expect("meta")
        .modified()
        .expect("mtime"),
    );

    std::fs::write(&path, "fn b() {}\n").expect("rewrite, same length");
    std::fs::File::options()
      .write(true)
      .open(&path)
      .expect("open")
      .set_times(times)
      .expect("put the mtime back");

    let after = stamp_of(&path).expect("stamp");
    assert_eq!(
      before, after,
      "precondition: the rewrite must leave the stat untouched, or this arm is \
       measuring an ordinary edit"
    );
    assert_ne!(
      was,
      crate::sync::file_sha256(&path).expect("hash"),
      "precondition: the bytes must actually differ"
    );

    assert!(
      must_read(Policy::HashAlways, Some(&before), &after),
      "canon catches it, which is what D24 and AC-03.3 require"
    );
    assert!(
      !must_read(Policy::StatThenHash, Some(&before), &after),
      "and source misses it, which is the accepted cost: one stale hit, against \
       hashing every source file on every reconcile"
    );
  }

  /// A project root that is a real repository, because the survey's scope is
  /// git's answer and a fixture without git has no rules at all.
  fn repo() -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    let ok = std::process::Command::new("git")
      .args(["init", "-q"])
      .current_dir(dir.path())
      .status()
      .expect("run git")
      .success();
    assert!(ok, "git init failed");
    std::fs::write(dir.path().join(".gitignore"), "build/\n").expect("gitignore");
    dir
  }

  fn write(dir: &tempfile::TempDir, rel: &str, bytes: &[u8]) -> std::path::PathBuf {
    let path = dir.path().join(rel);
    std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
    std::fs::write(&path, bytes).expect("write");
    path
  }

  fn surveyed(rows: &[Surveyed], rel: &str) -> Surveyed {
    rows
      .iter()
      .find(|r| r.path == rel)
      .unwrap_or_else(|| panic!("no row for `{rel}`; rows: {rows:?}"))
      .clone()
  }

  #[test]
  fn every_in_scope_path_gets_a_row_and_a_skipped_one_is_still_a_row() {
    let dir = repo();
    write(&dir, "README.md", b"# readme\n");
    write(&dir, "src/lib.rs", b"fn main() {}\n");
    write(&dir, "assets/logo.bin", b"\x00\x01binary");
    write(&dir, "build/out.o", b"ignored\n");
    let canon = dir.path().join("intent/.canon");

    let rows = survey(
      dir.path(),
      &[],
      &canon,
      super::super::corpus::DEFAULT_MAX_FILE_BYTES,
    )
    .expect("survey");

    assert_eq!(surveyed(&rows, "README.md").corpus, Corpus::Prose);
    assert_eq!(surveyed(&rows, "README.md").skipped, None);
    assert_eq!(
      surveyed(&rows, "src/lib.rs").corpus,
      Corpus::Code { lang: Some("rust") }
    );
    assert_eq!(
      surveyed(&rows, "assets/logo.bin").skipped,
      Some(SkipReason::Binary),
      "a file the index will not hold is a ROW SAYING WHY, which is the whole \
       of AC-18.2 -- it is not missing from the survey"
    );
    assert!(
      !rows.iter().any(|r| r.path.starts_with("build/")),
      "and an IGNORED file is not in scope at all, so it gets no row: the \
       corpus is the gitignore-aware repository, not everything on disk"
    );
  }

  #[test]
  fn the_stores_own_projections_are_absent_rather_than_skipped() {
    // **THE ONE CORRECT ABSENCE.** A view is not a file the index declined; it
    // is the store's prose seen twice, and a row saying `skipped` would invite
    // an operator to go looking for the reason.
    let dir = repo();
    let view = write(&dir, "intent/st/ST0001/info.md", b"# a view\n");
    write(&dir, "intent/.canon/st/ST0001.json", b"{}\n");
    let beside = write(&dir, "intent/st/ST0001/notes.md", b"# not a view\n");
    let canon = dir.path().join("intent/.canon");

    let rows = survey(
      dir.path(),
      &[view],
      &canon,
      super::super::corpus::DEFAULT_MAX_FILE_BYTES,
    )
    .expect("survey");

    assert!(!rows.iter().any(|r| r.path == "intent/st/ST0001/info.md"));
    assert!(
      !rows
        .iter()
        .any(|r| r.path == "intent/.canon/st/ST0001.json")
    );
    // The control: the rule is the renderer's answer, not the directory.
    assert_eq!(
      surveyed(&rows, "intent/st/ST0001/notes.md").corpus,
      Corpus::Prose,
      "a file the renderer does not produce, in the same directory, stays in \
       the corpus -- or the exclusion would be a path-shape hack"
    );
    assert!(beside.exists());
  }

  #[test]
  fn the_survey_and_the_predicate_cannot_disagree() {
    // One scope object answers both the enumeration and the question. Without
    // this, a survey that walked its own way would drift from
    // `Scanned::in_repository` in whichever direction nobody was looking.
    let dir = repo();
    write(&dir, "README.md", b"# readme\n");
    write(&dir, "src/lib.rs", b"fn main() {}\n");
    write(&dir, "build/out.o", b"ignored\n");
    write(&dir, "intent/.cache/intent.db", b"SQLite format 3\x00");
    let canon = dir.path().join("intent/.canon");

    let rows = survey(
      dir.path(),
      &[],
      &canon,
      super::super::corpus::DEFAULT_MAX_FILE_BYTES,
    )
    .expect("survey");
    let scope = Scanned::for_root(dir.path());

    for row in &rows {
      assert!(
        scope.in_repository(&dir.path().join(&row.path)),
        "the survey enumerated `{}` and the predicate excludes it",
        row.path
      );
    }
    assert!(
      rows.len() >= 2,
      "the fixture surveyed {} row(s), too few for either direction to mean \
       anything: {rows:?}",
      rows.len()
    );
    for out in ["build/out.o", "intent/.cache/intent.db"] {
      assert!(
        !rows.iter().any(|r| r.path == out),
        "`{out}` is out of scope and the survey named it anyway"
      );
    }
  }
}
