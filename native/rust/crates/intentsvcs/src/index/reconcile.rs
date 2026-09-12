//! The impure rim: what the index has to touch the filesystem to know.
//!
//! **IT DECIDES NOTHING IT CAN ASK [`super::corpus`] INSTEAD** (IN-AG-PFIC-001).
//! The rules -- which corpus a path joins, what counts as binary, what the cap
//! is -- are pure and live one module over, driven on values. What is here is
//! the reading: a `stat` that must not follow a link, a bounded read that can
//! fail, and the ordering between them.

use std::io::Read;
use std::path::Path;

use super::corpus::{BINARY_SAMPLE_BYTES, SkipReason, looks_binary};

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
}
