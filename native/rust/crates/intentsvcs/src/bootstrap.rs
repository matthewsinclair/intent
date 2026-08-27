//! What it means to set THIS MACHINE up.
//!
//! The third of the three questions `userstate.rs` names, in its operational
//! form. `project.rs` answers *where is the tree I am standing in*,
//! `install.rs` answers *where is the tool I am running*, `userstate.rs`
//! answers *where does this operator's state live* -- and this module is the
//! one that WRITES that last answer down.
//!
//! # Two things, and deliberately only two
//!
//! It publishes the install-root pointer ([`crate::install::publish_home`]),
//! and it records the operator's own config. hv ruled the caller for the
//! publisher on 2026-08-27 (`164d5bce`) and ruled the scope in the same breath:
//! *implementing it minimally -- this pointer plus the author identity it
//! already owns*.
//!
//! # What v2 does that this deliberately does not
//!
//! `bin/intent_bootstrap` has six steps. Two of them must not be ported:
//!
//! **Step 5 prints `export INTENT_HOME=...` and a PATH line.** That advice is
//! FALSE under v3. `install.rs` resolves the root by walking up from the
//! symlink-resolved `current_exe()` and reads no environment at all -- a
//! deliberate refusal, because a stale `INTENT_HOME` exported in somebody's
//! shell would make a v3 binary exec v2's scripts. Printing setup instructions
//! for a variable nothing reads would teach an operator a model of the tool
//! that is wrong, and the pointer this module writes is precisely what replaced
//! it.
//!
//! **Step 6 runs `intent doctor`.** A setup command that invokes a diagnostic
//! makes each answerable for the other's exit code, and it is the mirror of the
//! reason hv rejected `doctor` as the publisher's caller: a diagnostic that
//! writes, and a writer that diagnoses, are the same conflation from opposite
//! ends. `doctor` is one command away and reports for itself.
//!
//! # Why the config is v3-private
//!
//! `~/.intent/config.json`, never v2's `~/.config/intent/config.json`. That is
//! vc's class ruling, hv adopted 2026-08-22: every v3 per-user store gets its
//! own path and never reads or writes v2's. v2 is shipped and can never be
//! taught to branch, so separate paths are the only mechanism that works --
//! a `version` field would be a courtesy only the newer party reads.

use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::install::{self, Published};

#[derive(Debug, Error)]
pub enum BootstrapError {
  #[error("{0}")]
  Install(#[from] install::InstallError),
  #[error("cannot locate your per-user Intent directory: {0}")]
  UserState(#[from] crate::userstate::UserStateError),
  /// **The path is in the message.** A bare io error here names no file, and
  /// the two candidates -- the directory and the config inside it -- fail for
  /// different reasons and need different repairs.
  #[error("cannot write {path}: {source}")]
  Write {
    path: String,
    #[source]
    source: std::io::Error,
  },
}

impl crate::remedy::Remedy for BootstrapError {
  fn remedy(&self) -> String {
    match self {
      BootstrapError::Install(e) => crate::remedy::Remedy::remedy(e),
      BootstrapError::UserState(e) => crate::remedy::Remedy::remedy(e),
      BootstrapError::Write { path, .. } => format!(
        "bootstrap writes only inside your own per-user Intent directory. Check that {path} is writable and that the filesystem is not full or read-only."
      ),
    }
  }
}

/// What the config file was found to be, so the caller can say so.
///
/// **`Unchanged` IS A DISTINCT OUTCOME RATHER THAN A QUIET SUCCESS**, and it is
/// the one an operator re-running `bootstrap` will hit. Collapsing it into
/// `Written` would report a write that did not happen; collapsing it into
/// nothing would leave `--force`'s effect invisible.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Config {
  /// The file already existed and was left exactly as it was.
  Kept { path: PathBuf },
  /// There was no config; this one is new.
  Created {
    path: PathBuf,
    author: Option<String>,
  },
  /// A config existed and `--force` replaced it.
  Replaced {
    path: PathBuf,
    author: Option<String>,
  },
}

/// Everything one `bootstrap` run did.
#[derive(Debug)]
pub struct Report {
  pub pointer: Published,
  pub config: Config,
}

/// Set this machine up.
///
/// **THE POINTER IS PUBLISHED FIRST, AND THE ORDER IS LOAD-BEARING.**
/// [`install::publish_home`] refuses before it writes if the root it resolved
/// is not an install -- so on a broken installation this fails having written
/// NOTHING, rather than leaving a config file behind that claims a setup which
/// did not happen. The cheap check that can refuse goes first.
pub fn run(force: bool) -> Result<Report, BootstrapError> {
  let pointer = install::publish_home()?;
  let path = crate::userstate::global_config()?;
  let config = write_config(&path, crate::userstate::author(), force)?;
  Ok(Report { pointer, config })
}

/// The config half, with the path and identity handed in.
///
/// Split from [`run`] for the reason `install.rs` splits `resolve` from `home`
/// and `userstate.rs` keeps its one ambient read in a single function: every
/// arm below can then be driven against a fixture, rather than against
/// whatever `$HOME` the suite happens to run under.
pub fn write_config(
  path: &Path,
  author: Option<String>,
  force: bool,
) -> Result<Config, BootstrapError> {
  if path.exists() && !force {
    return Ok(Config::Kept {
      path: path.to_path_buf(),
    });
  }
  let replacing = path.exists();

  if let Some(dir) = path.parent() {
    std::fs::create_dir_all(dir).map_err(|source| BootstrapError::Write {
      path: dir.display().to_string(),
      source,
    })?;
  }

  // **NO BACKUP FILE, UNLIKE v2.** `bin/intent_bootstrap` copies the old config
  // to `config.json.bak.<timestamp>` under `--force`, which accumulates one
  // file per run in a directory nothing ever prunes. The content is four
  // fields an operator can retype, `--force` is explicit about replacing, and
  // this estate's own rule is that migrations prune rather than preserve.
  std::fs::write(path, render(author.as_deref())).map_err(|source| BootstrapError::Write {
    path: path.display().to_string(),
    source,
  })?;

  Ok(if replacing {
    Config::Replaced {
      path: path.to_path_buf(),
      author,
    }
  } else {
    Config::Created {
      path: path.to_path_buf(),
      author,
    }
  })
}

/// The config's bytes.
///
/// **Hand-rendered rather than serialised, and the reason is measured rather
/// than stylistic:** `serde_json` writes a map in whatever order it is given
/// and offers no trailing newline, and this file is one an operator opens in an
/// editor. Four fixed fields in a fixed order, with a newline, is a file that
/// looks the same every time it is written -- and a serialiser is a second
/// writer nobody declared.
///
/// **AN ABSENT AUTHOR IS OMITTED, NOT WRITTEN AS `null` OR `""`.** A key whose
/// value is empty reads as a decision someone made; an absent key reads as a
/// question nobody answered, which is what it is.
fn render(author: Option<&str>) -> String {
  let mut out = String::from("{\n");
  out.push_str(&format!(
    "  \"intent_version\": \"{}\",\n",
    env!("CARGO_PKG_VERSION")
  ));
  if let Some(a) = author {
    out.push_str(&format!("  \"author\": \"{}\",\n", a.replace('"', "\\\"")));
  }
  out.push_str("  \"intent_dir\": \"intent\"\n");
  out.push_str("}\n");
  out
}

/// The author this machine has recorded, if `bootstrap` has run.
///
/// **THE READER LIVES BESIDE THE WRITER ON PURPOSE.** [`render`] decides the
/// config's shape; a second module parsing that shape is two definitions of one
/// file kept in step by hand, which is the Highlander case this estate keeps
/// meeting. `init` calls this rather than reaching for the path itself.
///
/// **EVERY FAILURE IS `None`, AND THAT IS NOT SWALLOWING AN ERROR.** The
/// question asked is *is an author recorded*, and "no config", "unreadable
/// config", "config without the key" are all honestly answered no. The caller's
/// behaviour is identical in each case -- it proceeds without an author and
/// says so -- so distinguishing them would produce a branch nothing takes.
/// Nothing here is a write, so there is no half-done state to hide.
pub fn recorded_author() -> Option<String> {
  let path = crate::userstate::global_config().ok()?;
  let text = std::fs::read_to_string(path).ok()?;
  let parsed: serde_json::Value = serde_json::from_str(&text).ok()?;
  let author = parsed.get("author")?.as_str()?.trim();
  (!author.is_empty()).then(|| author.to_string())
}

#[cfg(test)]
mod tests {
  use super::*;

  fn tmp(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("intent-bootstrap-{name}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("fixture dir");
    dir
  }

  #[test]
  fn a_fresh_machine_gets_a_config_naming_the_author() {
    let path = tmp("fresh").join("config.json");
    let made = write_config(&path, Some("matts".into()), false).expect("write");
    assert!(matches!(made, Config::Created { .. }), "{made:?}");
    let text = std::fs::read_to_string(&path).expect("read back");
    assert!(text.contains("\"author\": \"matts\""), "{text}");
    assert!(text.ends_with("}\n"), "trailing newline: {text:?}");
  }

  /// **The idempotence an operator relies on when re-running setup.**
  #[test]
  fn a_second_run_without_force_keeps_the_existing_config_byte_for_byte() {
    let path = tmp("keep").join("config.json");
    write_config(&path, Some("first".into()), false).expect("write");
    let before = std::fs::read_to_string(&path).expect("read");

    let again = write_config(&path, Some("second".into()), false).expect("second");
    assert!(matches!(again, Config::Kept { .. }), "{again:?}");
    assert_eq!(
      before,
      std::fs::read_to_string(&path).expect("read"),
      "a run without --force must not rewrite the file"
    );
  }

  #[test]
  fn force_replaces_and_says_that_is_what_it_did() {
    let path = tmp("force").join("config.json");
    write_config(&path, Some("first".into()), false).expect("write");
    let again = write_config(&path, Some("second".into()), true).expect("force");
    assert!(matches!(again, Config::Replaced { .. }), "{again:?}");
    assert!(
      std::fs::read_to_string(&path)
        .expect("read")
        .contains("\"author\": \"second\"")
    );
  }

  /// **v2 leaves a `.bak.<timestamp>` behind on every forced run. This does
  /// not, and the absence is asserted rather than assumed.**
  #[test]
  fn force_leaves_no_backup_litter() {
    let dir = tmp("nolitter");
    let path = dir.join("config.json");
    write_config(&path, Some("first".into()), false).expect("write");
    write_config(&path, Some("second".into()), true).expect("force");
    let found: Vec<String> = std::fs::read_dir(&dir)
      .expect("read dir")
      .flatten()
      .map(|e| e.file_name().to_string_lossy().into_owned())
      .collect();
    assert_eq!(found, vec!["config.json".to_string()], "{found:?}");
  }

  /// An unset `USER` is a normal answer, so the config is still written.
  #[test]
  fn an_unknown_author_omits_the_key_rather_than_writing_an_empty_one() {
    let path = tmp("noauthor").join("config.json");
    write_config(&path, None, false).expect("write");
    let text = std::fs::read_to_string(&path).expect("read");
    assert!(!text.contains("author"), "{text}");
    assert!(text.contains("\"intent_version\""), "{text}");
    // It must still be parseable -- an omitted key is only safe if the comma
    // it was carrying went with it.
    let parsed: serde_json::Value = serde_json::from_str(&text).expect("valid json without author");
    assert!(parsed.get("author").is_none());
  }

  /// The rendered file is JSON in both directions, with and without the
  /// optional key -- the comma placement is the thing that breaks.
  #[test]
  fn both_shapes_are_valid_json() {
    for author in [Some("matts"), None] {
      let text = render(author);
      serde_json::from_str::<serde_json::Value>(&text)
        .unwrap_or_else(|e| panic!("invalid json for author={author:?}: {e}\n{text}"));
    }
  }

  #[test]
  fn the_written_directory_is_created_when_absent() {
    let path = tmp("mkdir").join("nested").join("config.json");
    write_config(&path, Some("matts".into()), false).expect("write");
    assert!(path.exists());
  }
}
