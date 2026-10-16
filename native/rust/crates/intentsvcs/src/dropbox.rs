//! The Dropbox mark on `intent/.cache` (ST0083 WP-07, hv's ruling 8 of
//! 2026-10-15).
//!
//! **A STORE UNDER A DROPBOX FOLDER IS A LIVE SQLITE DATABASE AND ITS WAL,
//! SYNCED FILE BY FILE TO EVERY DEVICE THAT SHARES THE FOLDER.** The client
//! uploads each file as it settles, so another device can receive a database
//! whose WAL belongs to a different moment, and a family member who syncs a
//! book's folder receives a tooling file they never asked for. The store
//! rebuilds from committed canon on any machine, as it does on a fresh clone,
//! so the right state for `intent/.cache` under Dropbox is ignored.
//!
//! **THE MARK GOES ON THE DIRECTORY BEFORE THE STORE FILE EXISTS INSIDE IT.**
//! [`make_cache_dir`] is where the directory is born, and `Store::open` calls
//! it before SQLite creates the file, so the client never sees a store it was
//! not told to ignore.
//!
//! **"UNDER DROPBOX" IS ANSWERED ON RESOLVED REAL PATHS.** On a File Provider
//! Mac the client's folder is `~/Library/CloudStorage/Dropbox` and `~/Dropbox`
//! is a symlink to it, so a project opened through the symlink names a path no
//! root in `info.json` is a prefix of until both are resolved.
//!
//! The attributes are the ones Dropbox's own help page gives: on macOS the File
//! Provider's `com.apple.fileprovider.ignore#P` and the legacy client's
//! `com.dropbox.ignored`, both set because which client a Mac runs is not
//! Intent's to know; on Linux `user.com.dropbox.ignored`, which is what
//! `attr -s com.dropbox.ignored` sets. Each takes the value `1`.

use std::path::{Path, PathBuf};

/// The attributes this platform's Dropbox client reads, each set to [`VALUE`].
#[cfg(target_os = "macos")]
pub const ATTRIBUTES: &[&str] = &["com.apple.fileprovider.ignore#P", "com.dropbox.ignored"];
/// The attributes this platform's Dropbox client reads, each set to [`VALUE`].
#[cfg(target_os = "linux")]
pub const ATTRIBUTES: &[&str] = &["user.com.dropbox.ignored"];
/// No Dropbox client this build knows how to tell, so nothing to set.
#[cfg(not(any(target_os = "macos", target_os = "linux")))]
pub const ATTRIBUTES: &[&str] = &[];

/// The value every attribute in [`ATTRIBUTES`] takes.
pub const VALUE: &[u8] = b"1";

/// Why the Dropbox question, or the mark, could not be answered.
#[derive(Debug, thiserror::Error)]
pub enum DropboxError {
  #[error("{} is present and could not be read: {cause}", path.display())]
  InfoUnreadable {
    path: PathBuf,
    #[source]
    cause: std::io::Error,
  },
  #[error("{} is not the JSON the Dropbox client writes: {cause}", path.display())]
  InfoMalformed {
    path: PathBuf,
    #[source]
    cause: serde_json::Error,
  },
  #[error("{} could not be resolved to its real path: {cause}", dir.display())]
  Unresolvable {
    dir: PathBuf,
    #[source]
    cause: std::io::Error,
  },
  #[error("{attribute} could not be read on {}: {cause}", dir.display())]
  AttributeUnread {
    dir: PathBuf,
    attribute: &'static str,
    #[source]
    cause: std::io::Error,
  },
  #[error("{attribute} could not be set on {}: {cause}", dir.display())]
  AttributeUnset {
    dir: PathBuf,
    attribute: &'static str,
    #[source]
    cause: std::io::Error,
  },
}

impl crate::remedy::Remedy for DropboxError {
  fn remedy(&self) -> String {
    match self {
      Self::InfoUnreadable { .. } | Self::InfoMalformed { .. } => {
        "the Dropbox client writes this file and Intent only reads it; open the Dropbox app so it rewrites the file, then run `intent doctor`, which says whether `intent/.cache` is under Dropbox and marked".to_string()
      }
      Self::Unresolvable { .. } => {
        "check that the project directory exists and that you can read every directory above it, then run `intent doctor`".to_string()
      }
      Self::AttributeUnread { dir, .. } | Self::AttributeUnset { dir, .. } => format!(
        "mark it by hand from the project root: `{}`. The Dropbox client then removes `intent/.cache` from the server and from other devices, which is right for a store each machine rebuilds from committed canon",
        command(&dir.display().to_string())
      ),
    }
  }
}

/// What the store's directory birth did about Dropbox, when there was
/// anything to do.
#[derive(Debug)]
pub enum Marking {
  /// The directory sits under a Dropbox folder and now carries every one of
  /// [`ATTRIBUTES`].
  Marked { dir: PathBuf },
  /// The directory may sit under a Dropbox folder and is not marked, for the
  /// reason `cause` gives; its remedy carries the command that marks it.
  Unmarked { dir: PathBuf, cause: DropboxError },
}

/// Every account's folder, read from the Dropbox client's `info.json`.
///
/// The file is one object keyed by account kind (`personal`, `business`), and
/// each account carries the folder as `path`. A top level that is not an
/// object names no folder.
pub fn roots(info_json: &str) -> Result<Vec<PathBuf>, serde_json::Error> {
  let info: serde_json::Value = serde_json::from_str(info_json)?;
  Ok(
    info
      .as_object()
      .into_iter()
      .flat_map(|accounts| accounts.values())
      .filter_map(|account| account.get("path")?.as_str())
      .map(PathBuf::from)
      .collect(),
  )
}

/// The root `dir` sits under, comparing paths as given: both sides must
/// already be resolved, which is [`under_dropbox`]'s job.
pub fn covering<'a>(dir: &Path, roots: &'a [PathBuf]) -> Option<&'a PathBuf> {
  roots.iter().find(|root| dir.starts_with(root))
}

/// The command that marks `dir` on this platform, as Dropbox's help page
/// gives it.
pub fn command(dir: &str) -> String {
  if cfg!(target_os = "linux") {
    format!("attr -s com.dropbox.ignored -V 1 {dir}")
  } else {
    format!(
      "xattr -w 'com.apple.fileprovider.ignore#P' 1 {dir} && xattr -w com.dropbox.ignored 1 {dir}"
    )
  }
}

/// The folders of the Dropbox client installed for `home`, each resolved.
///
/// **NO `info.json` IS NO CLIENT, NOT AN ERROR**: most machines have none. A
/// root that does not resolve is kept as written, since nothing resolved can
/// sit under a folder that does not exist.
pub fn client_roots(home: &Path) -> Result<Vec<PathBuf>, DropboxError> {
  let path = home.join(".dropbox").join("info.json");
  let text = match std::fs::read_to_string(&path) {
    Ok(text) => text,
    Err(cause) if cause.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
    Err(cause) => return Err(DropboxError::InfoUnreadable { path, cause }),
  };
  let roots = roots(&text).map_err(|cause| DropboxError::InfoMalformed { path, cause })?;
  Ok(
    roots
      .into_iter()
      .map(|root| root.canonicalize().unwrap_or(root))
      .collect(),
  )
}

/// The Dropbox folder `dir` sits under, by its resolved real path, or `None`
/// when no client is installed for `home` or `dir` is outside every folder.
pub fn under_dropbox(dir: &Path, home: &Path) -> Result<Option<PathBuf>, DropboxError> {
  let roots = client_roots(home)?;
  if roots.is_empty() {
    return Ok(None);
  }
  let real = dir
    .canonicalize()
    .map_err(|cause| DropboxError::Unresolvable {
      dir: dir.to_path_buf(),
      cause,
    })?;
  Ok(covering(&real, &roots).cloned())
}

/// Whether `dir` carries every one of [`ATTRIBUTES`] with [`VALUE`].
pub fn is_marked(dir: &Path) -> Result<bool, DropboxError> {
  for attribute in ATTRIBUTES {
    let held = xattr::get(dir, attribute).map_err(|cause| DropboxError::AttributeUnread {
      dir: dir.to_path_buf(),
      attribute,
      cause,
    })?;
    if held.as_deref() != Some(VALUE) {
      return Ok(false);
    }
  }
  Ok(true)
}

/// Set every one of [`ATTRIBUTES`] on `dir`.
pub fn mark(dir: &Path) -> Result<(), DropboxError> {
  for attribute in ATTRIBUTES {
    xattr::set(dir, attribute, VALUE).map_err(|cause| DropboxError::AttributeUnset {
      dir: dir.to_path_buf(),
      attribute,
      cause,
    })?;
  }
  Ok(())
}

/// Mark `dir` when it sits under a Dropbox folder of the client installed for
/// `home`, and say what happened; `None` when Dropbox has no claim on it.
pub fn mark_if_under_dropbox(dir: &Path, home: &Path) -> Option<Marking> {
  let dir = dir.to_path_buf();
  match under_dropbox(&dir, home) {
    Ok(None) => None,
    Ok(Some(_)) => Some(match mark(&dir) {
      Ok(()) => Marking::Marked { dir },
      Err(cause) => Marking::Unmarked { dir, cause },
    }),
    Err(cause) => Some(Marking::Unmarked { dir, cause }),
  }
}

/// Create the store's directory and, when this call is what created it, mark
/// it before anything is written inside it.
///
/// **ONLY AT BIRTH.** A directory that already exists was born under an
/// earlier call, or by hand, and `intent doctor` is what reports one left
/// unmarked; marking on every open would put an attribute write and a read of
/// `info.json` on every command.
///
/// **A PROCESS WITH NO `$HOME` HAS NO DROPBOX CLIENT TO ASK ABOUT**, since the
/// client's `info.json` lives under it, so it creates the directory and says
/// nothing. `doctor`, run where `$HOME` is set, reports it if it matters.
pub fn make_cache_dir(dir: &Path) -> std::io::Result<Option<Marking>> {
  if dir.is_dir() {
    return Ok(None);
  }
  std::fs::create_dir_all(dir)?;
  let Ok(home) = crate::userstate::home() else {
    return Ok(None);
  };
  Ok(mark_if_under_dropbox(dir, &home))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn roots_reads_every_accounts_path() {
    let info = r#"{"personal": {"path": "/Users/a/Library/CloudStorage/Dropbox", "host": 1, "is_team": false},
      "business": {"path": "/Users/a/Library/CloudStorage/Dropbox-Firm", "is_team": true}}"#;
    let mut got = roots(info).expect("parses");
    got.sort();
    assert_eq!(
      got,
      vec![
        PathBuf::from("/Users/a/Library/CloudStorage/Dropbox"),
        PathBuf::from("/Users/a/Library/CloudStorage/Dropbox-Firm"),
      ]
    );
  }

  #[test]
  fn roots_refuses_what_is_not_json_and_names_nothing_for_a_non_object() {
    assert!(roots("not json").is_err());
    assert_eq!(roots("[]").expect("parses"), Vec::<PathBuf>::new());
  }

  #[test]
  fn covering_matches_whole_components_only() {
    let roots = vec![PathBuf::from("/d/Dropbox")];
    assert_eq!(
      covering(Path::new("/d/Dropbox/book/intent/.cache"), &roots),
      Some(&roots[0])
    );
    assert_eq!(covering(Path::new("/d/Dropbox-Firm/book"), &roots), None);
    assert_eq!(covering(Path::new("/elsewhere/book"), &roots), None);
  }
}
