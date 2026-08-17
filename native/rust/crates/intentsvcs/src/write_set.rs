//! All-or-nothing writes across several files.
//!
//! A mutation touches the thread's canon and every view it feeds -- five or
//! six files -- and AC-04.1 requires that a failure part-way through leaves no
//! torn state. Filesystems give no transaction to borrow, so this builds one:
//! record what every target looked like BEFORE, write each via temp-and-rename,
//! and on any failure put every already-written path back exactly as it was.
//!
//! Rollback is deliberately total rather than best-effort. A partial rollback
//! leaves the estate in a third state that is neither before nor after, and
//! that state has no name, no test, and nobody looking for it.
//!
//! Order matters at the call site, and D01 was REVERSED on 2026-08-15: the DB
//! is the SSOT and the files are re-creatable, so the DB transaction lands
//! FIRST and this batch is the projection that follows. A failure here leaves
//! the tree stale rather than torn, and stale is repairable from truth --
//! which is what makes this unwind the load-bearing part rather than an
//! optimisation.

use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum WriteError {
  #[error("writing {path}: {source}")]
  Io {
    path: String,
    #[source]
    source: io::Error,
  },
  /// A write failed AND the rollback could not fully undo the earlier ones.
  /// Reported distinctly because it is the one case where the estate really is
  /// torn, and a caller that cannot tell it from an ordinary failure would
  /// retry into the damage.
  #[error("writing {path} failed, and rolling back left {unrestored} file(s) modified: {source}")]
  TornRollback {
    path: String,
    unrestored: usize,
    #[source]
    source: io::Error,
  },
}

impl crate::remedy::Remedy for WriteError {
  fn remedy(&self) -> String {
    match self {
      // Nothing is half-written: the write set rolled back, so the estate is
      // where it was. Saying so IS the remedy -- the useful action is to fix
      // the permission and retry, and a caller who does not know the rollback
      // succeeded will go looking for damage first.
      Self::Io { path, .. } => format!(
        "check that {path}'s directory exists and is writable, then retry -- the write set rolled back, so nothing was left half-written"
      ),
      // The one case where that is NOT true, which is why it is its own
      // variant. A retry into a torn estate compounds it.
      Self::TornRollback { unrestored, .. } => format!(
        "DO NOT RETRY YET: {unrestored} file(s) were modified and could not be restored. Inspect with `git status` and restore them before running anything else"
      ),
    }
  }
}

fn io_err(path: &Path, source: io::Error) -> WriteError {
  WriteError::Io {
    path: path.display().to_string(),
    source,
  }
}

/// What one path looked like before the write, so it can be put back.
#[derive(Debug)]
struct Prior {
  path: PathBuf,
  /// `None` = the file did not exist, so rollback deletes it.
  content: Option<Vec<u8>>,
  /// Directories this write created, deepest first, removed on rollback.
  created_dirs: Vec<PathBuf>,
  /// Whether the write for this path actually LANDED.
  ///
  /// A prior is recorded before its write is attempted, so the path that FAILS
  /// has a prior too -- and restoring it is restoring a file the failed write
  /// never modified. Counting that restore's refusal as damage reported the
  /// estate as TORN while it was byte-for-byte intact: the loudest message the
  /// write layer has, raised for the calmest state. Its created directories
  /// are still cleaned up, because `record` really did make those.
  written: bool,
}

/// A batch of file writes that lands completely or not at all.
#[derive(Debug, Default)]
pub struct WriteSet {
  writes: Vec<(PathBuf, String)>,
}

impl WriteSet {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn add(&mut self, path: PathBuf, content: String) {
    self.writes.push((path, content));
  }

  pub fn is_empty(&self) -> bool {
    self.writes.is_empty()
  }

  pub fn len(&self) -> usize {
    self.writes.len()
  }

  /// Apply every write. On failure, restore everything already written and
  /// return the original error -- the estate is as it was.
  pub fn commit(self) -> Result<Applied, WriteError> {
    let mut priors: Vec<Prior> = Vec::with_capacity(self.writes.len());

    for (path, content) in &self.writes {
      let prior = match record(path) {
        Ok(prior) => prior,
        Err(e) => return Err(unwind(priors, path, e)),
      };
      priors.push(prior);
      if let Err(e) = write_atomically(path, content) {
        return Err(unwind(priors, path, e));
      }
      // Only now is there something to undo for this path.
      if let Some(last) = priors.last_mut() {
        last.written = true;
      }
    }
    Ok(Applied { priors })
  }
}

/// A committed batch that can still be undone -- held by the caller until the
/// rest of the mutation (the DB write) has also succeeded.
#[derive(Debug)]
pub struct Applied {
  priors: Vec<Prior>,
}

impl Applied {
  /// Undo the batch. Used when a later step fails after the files landed.
  pub fn rollback(self) -> Result<(), WriteError> {
    match restore_all(self.priors) {
      0 => Ok(()),
      unrestored => Err(WriteError::TornRollback {
        path: "(rollback)".to_string(),
        unrestored,
        source: io::Error::other("one or more files could not be restored"),
      }),
    }
  }

  /// Accept the batch: nothing more to undo.
  pub fn keep(self) {}
}

/// Wrap the triggering error, restoring what already landed.
fn unwind(priors: Vec<Prior>, path: &Path, source: WriteError) -> WriteError {
  let unrestored = restore_all(priors);
  if unrestored == 0 {
    return source;
  }
  let inner = match source {
    WriteError::Io { source, .. } => source,
    other => io::Error::other(other.to_string()),
  };
  WriteError::TornRollback {
    path: path.display().to_string(),
    unrestored,
    source: inner,
  }
}

/// Put every recorded path back, newest first. Returns how many could not be
/// restored -- zero means the estate is exactly as it was.
fn restore_all(priors: Vec<Prior>) -> usize {
  let mut failed = 0;
  for prior in priors.into_iter().rev() {
    if !prior.written {
      // Nothing to put back. Directories below are still ours to remove.
      for dir in &prior.created_dirs {
        let _ = std::fs::remove_dir(dir);
      }
      continue;
    }
    let restored = match &prior.content {
      Some(bytes) => std::fs::write(&prior.path, bytes).is_ok(),
      None => match std::fs::remove_file(&prior.path) {
        Ok(()) => true,
        // Already absent is the state we wanted.
        Err(e) => e.kind() == io::ErrorKind::NotFound,
      },
    };
    if !restored {
      failed += 1;
    }
    // Deepest first, and only if empty -- never remove a directory that
    // something else has since put a file into.
    for dir in &prior.created_dirs {
      let _ = std::fs::remove_dir(dir);
    }
  }
  failed
}

/// Capture a path's prior state, creating any missing parent directories and
/// noting which ones we made.
fn record(path: &Path) -> Result<Prior, WriteError> {
  let content = match std::fs::read(path) {
    Ok(bytes) => Some(bytes),
    Err(e) if e.kind() == io::ErrorKind::NotFound => None,
    Err(e) => return Err(io_err(path, e)),
  };

  let mut created_dirs = Vec::new();
  if let Some(parent) = path.parent() {
    // Deepest-last on the way up, so reversing gives deepest-first removal.
    let mut missing: Vec<PathBuf> = Vec::new();
    let mut cursor = Some(parent);
    while let Some(dir) = cursor {
      if dir.exists() {
        break;
      }
      missing.push(dir.to_path_buf());
      cursor = dir.parent();
    }
    if !missing.is_empty() {
      std::fs::create_dir_all(parent).map_err(|e| io_err(parent, e))?;
      created_dirs = missing;
    }
  }

  Ok(Prior {
    path: path.to_path_buf(),
    content,
    created_dirs,
    written: false,
  })
}

/// Write via a sibling temp file and a rename, so a reader never observes a
/// half-written file. The temp file sits in the destination directory because
/// a rename across filesystems is not atomic and may not even be a rename.
fn write_atomically(path: &Path, content: &str) -> Result<(), WriteError> {
  let dir = path.parent().unwrap_or(Path::new("."));
  let name = path
    .file_name()
    .map(|n| n.to_string_lossy().into_owned())
    .unwrap_or_else(|| "intent".to_string());
  let temp = dir.join(format!(".{name}.intent-tmp"));

  std::fs::write(&temp, content).map_err(|e| io_err(&temp, e))?;
  if let Err(e) = std::fs::rename(&temp, path) {
    let _ = std::fs::remove_file(&temp);
    return Err(io_err(path, e));
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_committed_set_lands_every_write() {
    let dir = tempfile::tempdir().expect("tempdir");
    let mut set = WriteSet::new();
    set.add(dir.path().join("a.md"), "a".to_string());
    set.add(dir.path().join("deep/b.md"), "b".to_string());
    set.commit().expect("commit").keep();

    assert_eq!(
      std::fs::read_to_string(dir.path().join("a.md")).unwrap(),
      "a"
    );
    assert_eq!(
      std::fs::read_to_string(dir.path().join("deep/b.md")).unwrap(),
      "b"
    );
  }

  #[test]
  fn rollback_restores_prior_content_and_removes_new_files() {
    let dir = tempfile::tempdir().expect("tempdir");
    let existing = dir.path().join("a.md");
    std::fs::write(&existing, "original").expect("seed");

    let mut set = WriteSet::new();
    set.add(existing.clone(), "changed".to_string());
    set.add(dir.path().join("new.md"), "new".to_string());
    set.commit().expect("commit").rollback().expect("rollback");

    assert_eq!(std::fs::read_to_string(&existing).unwrap(), "original");
    assert!(
      !dir.path().join("new.md").exists(),
      "a file that did not exist before must not exist after"
    );
  }

  #[test]
  fn rollback_removes_directories_it_created() {
    let dir = tempfile::tempdir().expect("tempdir");
    let mut set = WriteSet::new();
    set.add(dir.path().join("made/up/c.md"), "c".to_string());
    set.commit().expect("commit").rollback().expect("rollback");

    assert!(
      !dir.path().join("made").exists(),
      "rollback leaves no empty scaffolding behind"
    );
  }

  #[test]
  fn no_temp_files_survive_a_successful_commit() {
    let dir = tempfile::tempdir().expect("tempdir");
    let mut set = WriteSet::new();
    set.add(dir.path().join("a.md"), "a".to_string());
    set.commit().expect("commit").keep();

    let strays: Vec<String> = std::fs::read_dir(dir.path())
      .expect("read_dir")
      .filter_map(Result::ok)
      .map(|e| e.file_name().to_string_lossy().into_owned())
      .filter(|n| n.contains("intent-tmp"))
      .collect();
    assert!(strays.is_empty(), "temp files left behind: {strays:?}");
  }
}
