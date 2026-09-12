//! The impure rim: what the index has to touch the filesystem to know.
//!
//! **IT DECIDES NOTHING IT CAN ASK [`super::corpus`] INSTEAD** (IN-AG-PFIC-001).
//! The rules -- which corpus a path joins, what counts as binary, what the cap
//! is -- are pure and live one module over, driven on values. What is here is
//! the reading: a `stat` that must not follow a link, a bounded read that can
//! fail, and the ordering between them.

use std::io::Read;
use std::path::Path;

use super::Row;
use super::corpus::{BINARY_SAMPLE_BYTES, SkipReason, corpus_of, looks_binary};
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

/// Every path in the index's scope, as a row per path: the corpus it joins, and
/// the reason the index will not hold it if there is one.
///
/// **THE STORE'S OWN PROJECTIONS ARE NOT IN THE RESULT AT ALL**, which is the
/// one absence here that is correct: a rendered view and the canon extract are
/// the store's prose seen twice, so they are not files the index declined --
/// they are not files the index is about. Everything else in scope gets a row,
/// skipped ones included.
///
/// **A ROW IS NOT PROOF THE FILE WAS READ.** `indexed_sha256` is left `None`
/// here for every row, skipped or not: a survey walks and stats, and the hash
/// is the content indexer's to write when it reads the bytes. Filling it in
/// from a hash taken now would claim the file had been indexed at that content
/// when nothing had indexed it at all.
///
/// `views` is every path the renderer produces for this project and `canon_dir`
/// is the extract's directory; both are the caller's to supply, so this can be
/// driven without a store.
pub fn survey(
  root: &Path,
  views: &[std::path::PathBuf],
  canon_dir: &Path,
  max_bytes: u64,
) -> Result<Vec<Row>, SyncError> {
  rows_under(root, None, views, canon_dir, max_bytes)
}

/// What an event under `under` changes about the index, against the rows the
/// store holds.
///
/// **THE SHAPE IS `sync::changed_under`'s ON PURPOSE, AND THE QUESTION IS NOT.**
/// That one answers which CANON files differ, for a corpus of the named root
/// files plus `intent/`; this answers which INDEX rows differ, for the
/// gitignore-aware repository. A watcher that widened the first instead of
/// calling the second would put every source edit through a canon ingest.
///
/// **THE CORPUS IS ENUMERATED ONCE AND FILTERED, NEVER WALKED FROM `under`**
/// (vc, 2026-09-12, ruling the shape of the same defect in the watcher). A walk
/// from the event's path keeps whatever is under it, which is a SECOND
/// statement of scope and disagrees with the first at exactly the paths that
/// matter -- a root event enumerating the whole tree for a corpus that is three
/// files by name.
///
/// `indexed_sha256` is NOT part of the comparison, and the upserts carry the
/// stored value forward. A survey does not read bytes, so it has no opinion
/// about what the index holds; comparing on it would report every row as
/// changed forever, and writing its `None` would erase the record.
pub fn changed_under(
  root: &Path,
  under: &Path,
  previous: &[Row],
  views: &[std::path::PathBuf],
  canon_dir: &Path,
  max_bytes: u64,
) -> Result<Change, SyncError> {
  let seen = rows_under(root, Some(under), views, canon_dir, max_bytes)?;
  let mut upserts = Vec::new();
  for mut row in seen {
    let before = previous.iter().find(|p| p.path == row.path);
    row.indexed_sha256 = before.and_then(|b| b.indexed_sha256.clone());
    if before != Some(&row) {
      upserts.push(row);
    }
  }
  let under_rel = crate::project::relative(root, under);
  let removed = previous
    .iter()
    .filter(|p| {
      under_rel.is_empty() || p.path == under_rel || p.path.starts_with(&format!("{under_rel}/"))
    })
    .filter(|p| !root.join(&p.path).exists())
    .map(|p| p.path.clone())
    .collect();
  Ok(Change { upserts, removed })
}

/// What a scoped reconcile found: the rows to write, and the paths that have
/// gone.
///
/// **REMOVALS ARE THEIR OWN FIELD RATHER THAN AN ABSENCE**, because the caller
/// is reconciling a SUBTREE: a row missing from `upserts` is a row that did not
/// change, and a row missing from the whole answer would be indistinguishable
/// from one outside the event's path.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Change {
  pub upserts: Vec<Row>,
  pub removed: Vec<String>,
}

fn rows_under(
  root: &Path,
  under: Option<&Path>,
  views: &[std::path::PathBuf],
  canon_dir: &Path,
  max_bytes: u64,
) -> Result<Vec<Row>, SyncError> {
  let scope = Scanned::for_root(root);
  let mut out = Vec::new();
  for path in repository_files(root, &scope)? {
    if let Some(under) = under
      && !path.starts_with(under)
    {
      continue;
    }
    let Some(corpus) = corpus_of(&path, views, canon_dir) else {
      continue;
    };
    // **A PATH THAT CANNOT BE STAT'D IS STILL A ROW.** It is in scope, so the
    // report owes the operator a line about it, and `unreadable` is exactly
    // what happened. Dropping it would make the one file nobody can read the
    // one file nothing says anything about.
    let stamp = stamp_of(&path);
    let skipped = skip_for(&path, max_bytes);
    out.push(Row {
      path: crate::project::relative(root, &path),
      corpus: corpus.as_str().to_string(),
      lang: match &corpus {
        super::corpus::Corpus::Code { lang } => lang.map(str::to_string),
        _ => None,
      },
      size: stamp.as_ref().map(|s| s.size).unwrap_or_default(),
      mtime: stamp.map(|s| s.mtime).unwrap_or_default(),
      indexed_sha256: None,
      skipped_reason: skipped.map(|r| r.as_str().to_string()),
    });
  }
  Ok(out)
}

/// What the index holds after reading the files it said it would hold.
///
/// **`indexed` IS A PAIR PER FILE AND NOT A FLAG**, because the column it fills
/// records what was read rather than that something was: a later reconcile
/// compares it with the file's current hash to know whether the rows beside it
/// are the bytes on disk.
#[derive(Debug, Clone, Default)]
pub struct Content {
  /// Disk prose, split by the same splitter canon prose goes through.
  pub prose: Vec<crate::prose::DocSection>,
  /// Code, one row per file at the lexical tier.
  pub source: Vec<super::source::Section>,
  /// `(path, sha256)` for every file actually read.
  pub indexed: Vec<(String, String)>,
}

/// Read every row the index says it holds, and turn the bytes into rows.
///
/// **IT READS ONLY WHAT THE SURVEY ALREADY DECIDED TO HOLD.** A skipped row is
/// not opened -- that is the whole of what a skip buys -- and a row the survey
/// did not produce is not in scope, so this never makes a scope decision of its
/// own.
///
/// **A FILE WHOSE BYTES ARE NOT UTF-8 IS INDEXED LOSSILY RATHER THAN DROPPED,
/// AND THE ESTATE ALREADY HANDLES WHAT THAT COSTS.** A search index is over
/// text; a Latin-1 `notes.txt` is prose somebody wrote and the corpus rule
/// keeps it, because binary is a NUL and not a failed decode. The lossy body
/// differs from the file on disk, and `SearchHit`'s line is `None` unless the
/// indexed body IS the file byte for byte -- so such a hit is found and carries
/// no line, which is the honest outcome rather than a line pointing at bytes
/// that are not there.
///
/// **A READ THAT FAILS HERE IS PASSED OVER SILENTLY ON PURPOSE.** The row
/// already exists and its `indexed_sha256` stays as it was, so the file is
/// reported by `index status` exactly as it was before; making this a refusal
/// would fail a whole reconcile over one file that changed its permissions
/// between the walk and the read.
pub fn read_content(root: &Path, rows: &[Row]) -> Content {
  let mut out = Content::default();
  for row in rows {
    if row.skipped_reason.is_some() {
      continue;
    }
    let path = root.join(&row.path);
    let Ok(bytes) = std::fs::read(&path) else {
      continue;
    };
    let text = String::from_utf8_lossy(&bytes);
    match row.corpus.as_str() {
      "prose" => out.prose.extend(crate::prose::split(
        crate::prose::FILE_OWNER,
        &row.path,
        &row.path,
        &text,
      )),
      "code" => out.source.push(super::source::whole_file(&row.path, &text)),
      // `canon` is the store's own prose and is indexed FROM the store, so a
      // file carrying that corpus is not this pass's to read. It cannot arise
      // from a survey today and is passed over rather than assumed away.
      _ => continue,
    }
    out
      .indexed
      .push((row.path.clone(), crate::sync::sha256_of(&bytes)));
  }
  out
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

  fn surveyed(rows: &[Row], rel: &str) -> Row {
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

    assert_eq!(surveyed(&rows, "README.md").corpus, "prose");
    assert_eq!(surveyed(&rows, "README.md").skipped_reason, None);
    assert!(
      surveyed(&rows, "README.md").size > 0,
      "a row carries the file's stat, which is what the stat-then-hash policy \
       compares against"
    );
    assert_eq!(surveyed(&rows, "src/lib.rs").corpus, "code");
    assert_eq!(surveyed(&rows, "src/lib.rs").lang.as_deref(), Some("rust"));
    assert_eq!(
      surveyed(&rows, "assets/logo.bin").skipped_reason.as_deref(),
      Some("binary"),
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
      "prose",
      "a file the renderer does not produce, in the same directory, stays in \
       the corpus -- or the exclusion would be a path-shape hack"
    );
    assert!(beside.exists());
  }

  #[test]
  fn a_survey_claims_nothing_about_content_it_has_not_read() {
    // The survey walks and stats; the hash belongs to whatever reads the bytes.
    let dir = repo();
    write(&dir, "README.md", b"# readme\n");
    let canon = dir.path().join("intent/.canon");
    let rows = survey(
      dir.path(),
      &[],
      &canon,
      super::super::corpus::DEFAULT_MAX_FILE_BYTES,
    )
    .expect("survey");
    assert!(
      rows.iter().all(|r| r.indexed_sha256.is_none()),
      "`indexed_sha256` says what this row was last indexed AT, and nothing has \
       indexed anything yet"
    );
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

  fn rows_of(dir: &tempfile::TempDir) -> Vec<Row> {
    survey(
      dir.path(),
      &[],
      &dir.path().join("intent/.canon"),
      super::super::corpus::DEFAULT_MAX_FILE_BYTES,
    )
    .expect("survey")
  }

  fn changes(dir: &tempfile::TempDir, under: &str, previous: &[Row]) -> Change {
    changed_under(
      dir.path(),
      &dir.path().join(under),
      previous,
      &[],
      &dir.path().join("intent/.canon"),
      super::super::corpus::DEFAULT_MAX_FILE_BYTES,
    )
    .expect("changed_under")
  }

  #[test]
  fn an_unchanged_subtree_changes_nothing() {
    // The control that every arm below needs: a reconcile that reported work
    // on a tree nobody touched would wake the daemon forever.
    let dir = repo();
    write(&dir, "src/lib.rs", b"fn main() {}\n");
    write(&dir, "README.md", b"# readme\n");
    let previous = rows_of(&dir);

    assert_eq!(changes(&dir, "src", &previous), Change::default());
  }

  #[test]
  fn an_edit_under_the_event_is_an_upsert_and_one_outside_it_is_not() {
    let dir = repo();
    write(&dir, "src/lib.rs", b"fn main() {}\n");
    write(&dir, "docs/guide.md", b"# guide\n");
    let previous = rows_of(&dir);

    write(&dir, "src/lib.rs", b"fn main() { let longer = 1; }\n");
    write(
      &dir,
      "docs/guide.md",
      b"# a much longer guide than before\n",
    );

    let change = changes(&dir, "src", &previous);
    assert_eq!(
      change
        .upserts
        .iter()
        .map(|r| r.path.as_str())
        .collect::<Vec<_>>(),
      vec!["src/lib.rs"],
      "the reconcile answers about the event's subtree; the edit to `docs/` is \
       real and is not this event's business"
    );
    assert!(change.removed.is_empty());
  }

  #[test]
  fn a_file_that_has_gone_is_removed_and_not_silently_dropped() {
    let dir = repo();
    write(&dir, "src/lib.rs", b"fn main() {}\n");
    write(&dir, "src/old.rs", b"fn gone() {}\n");
    let previous = rows_of(&dir);

    std::fs::remove_file(dir.path().join("src/old.rs")).expect("remove");
    let change = changes(&dir, "src", &previous);

    assert_eq!(change.removed, vec!["src/old.rs".to_string()]);
    assert!(
      change.upserts.is_empty(),
      "and nothing else in the subtree is claimed to have changed"
    );
  }

  #[test]
  fn the_reconcile_carries_forward_what_the_index_holds() {
    // **`indexed_sha256` IS NOT THE SURVEY'S TO KNOW OR TO ERASE.** A survey
    // stats and does not read, so comparing on this column would report every
    // row as changed forever, and writing its `None` would erase the record of
    // what the index actually holds.
    let dir = repo();
    write(&dir, "src/lib.rs", b"fn main() {}\n");
    let mut previous = rows_of(&dir);
    for row in &mut previous {
      row.indexed_sha256 = Some("deadbeef".to_string());
    }

    assert_eq!(
      changes(&dir, "src", &previous),
      Change::default(),
      "a row whose only difference is a hash the survey cannot see is NOT a change"
    );

    write(&dir, "src/lib.rs", b"fn main() { let longer = 1; }\n");
    let change = changes(&dir, "src", &previous);
    assert_eq!(
      change.upserts[0].indexed_sha256.as_deref(),
      Some("deadbeef"),
      "and when the file really does change, the upsert still says what the \
       index holds -- stale content, honestly recorded, until something reads \
       the new bytes"
    );
  }
}
