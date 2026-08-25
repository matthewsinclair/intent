//! Plugin discovery: which plugins this INSTALL ships, and what they declare.
//!
//! Serves `intent plugin list` and `intent plugin show <name>` (ST0056 WP-06,
//! both `as-observed`). A plugin is a directory under `<install>/intent/plugins`
//! carrying a `plugin.json` that names it, versions it, describes it, and lists
//! the commands it contributes.
//!
//! # Roots come from the INSTALL, never the environment
//!
//! The same rule `rules.rs` and `skills.rs` follow, and for the same reason: a
//! v2 `$INTENT_HOME` left in an operator's environment would otherwise point a
//! v3 binary at v2's manifests, and the answer would look entirely normal. The
//! caller passes the install root it resolved; nothing here reads a variable.
//!
//! # A manifest that cannot be parsed is REPORTED, never skipped
//!
//! v2 does `[ -f "$plugin_json" ] || continue` and then trusts `jq`, so a
//! malformed manifest degrades to a plugin that is simply missing from the
//! list. **That answers "there are two plugins" with the same bytes whether the
//! third is absent or broken** -- the unpopulated-versus-empty defect in a
//! directory listing, which is the shape this thread keeps finding one artefact
//! at a time. So a directory WITHOUT a manifest is not a plugin and is skipped
//! silently, which is a real distinction and matches v2; a directory WITH a
//! manifest that will not read or parse is an error naming the file.
//!
//! That is a deliberate departure from v2 on an input v2's probe matrix never
//! covered (bare / `--help` / unknown flag / outside a project), so it deviates
//! from no observed behaviour.

use std::path::{Path, PathBuf};

use serde::Deserialize;

/// One plugin, as its manifest declares it.
#[derive(Debug, Clone, Deserialize)]
pub struct Plugin {
  pub name: String,
  #[serde(default)]
  pub version: String,
  #[serde(default)]
  pub description: String,
  #[serde(default)]
  pub commands: Vec<PluginCommand>,
  /// The directory the manifest was read from.
  ///
  /// Filled in by the walk rather than by serde -- it is where the plugin WAS
  /// FOUND, not something the manifest gets to claim about itself. A manifest
  /// that could name its own location could name someone else's.
  #[serde(skip)]
  pub root: PathBuf,
}

/// One command a plugin contributes, as `intent plugin show` prints it.
#[derive(Debug, Clone, Deserialize)]
pub struct PluginCommand {
  pub syntax: String,
  #[serde(default)]
  pub description: String,
}

#[derive(Debug, thiserror::Error)]
pub enum PluginError {
  #[error("cannot read the plugin directory {dir}")]
  Dir {
    dir: String,
    #[source]
    source: std::io::Error,
  },
  #[error("cannot read the plugin manifest {file}")]
  Read {
    file: String,
    #[source]
    source: std::io::Error,
  },
  #[error("the plugin manifest {file} is not valid JSON")]
  Parse {
    file: String,
    #[source]
    source: serde_json::Error,
  },
}

impl crate::remedy::Remedy for PluginError {
  fn remedy(&self) -> String {
    match self {
      // Not "reinstall": the directory was reachable enough to be named and
      // then refused, which is a permissions or filesystem fault rather than a
      // broken install.
      Self::Dir { dir, .. } => {
        format!("check that {dir} is readable by the account running intent")
      }
      Self::Read { file, .. } => {
        format!("check that {file} is readable by the account running intent")
      }
      // The file is the plugin author's, so the remedy names the file rather
      // than sending the operator to a verb -- there is no `intent` command
      // that repairs a third party's manifest, and offering one would be a
      // remedy naming something this binary cannot do.
      Self::Parse { file, .. } => {
        format!("repair the JSON in {file}, or remove the plugin directory that holds it")
      }
    }
  }
}

/// Where plugins live, relative to an install root.
pub fn root(install: &Path) -> PathBuf {
  install.join("intent").join("plugins")
}

/// Every plugin this install ships, ordered by directory name.
///
/// **A missing plugins directory is an empty list, not an error.** An install
/// that ships no plugins is a legitimate install, and v2 answers it with
/// `No plugins found.` at exit 0. A directory that EXISTS and cannot be read is
/// the opposite case and is refused -- the difference between "there are none"
/// and "I could not look" is the whole point.
///
/// Ordered by DIRECTORY name rather than by the name inside the manifest, which
/// is what v2's glob does. The two agree today and are not the same key: a
/// manifest is free to call itself anything.
pub fn discover(install: &Path) -> Result<Vec<Plugin>, PluginError> {
  let dir = root(install);
  let entries = match std::fs::read_dir(&dir) {
    Ok(entries) => entries,
    Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
    Err(source) => {
      return Err(PluginError::Dir {
        dir: dir.display().to_string(),
        source,
      });
    }
  };

  let mut dirs: Vec<PathBuf> = Vec::new();
  for entry in entries {
    let entry = entry.map_err(|source| PluginError::Dir {
      dir: dir.display().to_string(),
      source,
    })?;
    if entry.path().is_dir() {
      dirs.push(entry.path());
    }
  }
  dirs.sort();

  let mut out = Vec::new();
  for plugin_dir in dirs {
    let manifest = plugin_dir.join("plugin.json");
    // No manifest means this directory is not a plugin. That is a real
    // distinction and not a swallowed error, so it is skipped in silence
    // exactly as v2 skips it.
    if !manifest.is_file() {
      continue;
    }
    let text = std::fs::read_to_string(&manifest).map_err(|source| PluginError::Read {
      file: manifest.display().to_string(),
      source,
    })?;
    let mut plugin: Plugin = serde_json::from_str(&text).map_err(|source| PluginError::Parse {
      file: manifest.display().to_string(),
      source,
    })?;
    plugin.root = plugin_dir;
    out.push(plugin);
  }
  Ok(out)
}

/// One plugin by name, or `None` where no plugin answers to it.
///
/// Resolves through [`discover`] rather than joining the name onto the root,
/// so a plugin is found by the name it DECLARES. Joining the path would find it
/// by its directory name and then print a different one, and the two keys are
/// only incidentally equal.
pub fn find(install: &Path, name: &str) -> Result<Option<Plugin>, PluginError> {
  Ok(discover(install)?.into_iter().find(|p| p.name == name))
}

#[cfg(test)]
mod tests {
  use super::*;

  /// Lay down `<root>/intent/plugins/<name>/plugin.json` with the given body.
  fn manifest(root: &Path, name: &str, body: &str) {
    let dir = super::root(root).join(name);
    std::fs::create_dir_all(&dir).expect("create plugin dir");
    std::fs::write(dir.join("plugin.json"), body).expect("write manifest");
  }

  fn valid(name: &str) -> String {
    format!(
      r#"{{"name":"{name}","version":"1.0.0","description":"d","commands":[{{"syntax":"intent {name} x","description":"c"}}]}}"#
    )
  }

  /// **AN INSTALL WITH NO PLUGINS DIRECTORY HAS NO PLUGINS, AND THAT IS NOT AN
  /// ERROR.** v2 answers it at exit 0 with `No plugins found.`, and an install
  /// that ships none is legitimate.
  #[test]
  fn a_missing_plugins_directory_is_an_empty_list_rather_than_a_failure() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let found = discover(tmp.path()).expect("a missing directory is not an error");
    assert!(found.is_empty(), "expected no plugins, got {found:?}");
  }

  /// A directory with no manifest is not a plugin -- a real distinction, and
  /// the ONLY thing skipped in silence here.
  #[test]
  fn a_directory_without_a_manifest_is_not_a_plugin() {
    let tmp = tempfile::tempdir().expect("tempdir");
    std::fs::create_dir_all(super::root(tmp.path()).join("notaplugin")).expect("mkdir");
    manifest(tmp.path(), "real", &valid("real"));

    let found = discover(tmp.path()).expect("discover");
    let names: Vec<&str> = found.iter().map(|p| p.name.as_str()).collect();
    assert_eq!(names, vec!["real"]);
  }

  /// **THE ONE THAT MATTERS: A BROKEN MANIFEST IS REPORTED, NOT DROPPED.**
  ///
  /// v2 trusts `jq` and degrades a malformed plugin into an absent one, so the
  /// list answers "there are two plugins" with the same bytes whether the third
  /// is missing or broken. The failure is named, and it names the FILE, because
  /// the operator's next move is to open it.
  #[test]
  fn a_manifest_that_will_not_parse_is_named_rather_than_skipped() {
    let tmp = tempfile::tempdir().expect("tempdir");
    manifest(tmp.path(), "good", &valid("good"));
    manifest(tmp.path(), "broken", "{ this is not json");

    let err =
      discover(tmp.path()).expect_err("a broken manifest must not vanish into a shorter list");
    let rendered = format!("{err}");
    assert!(
      rendered.contains("broken") && rendered.contains("plugin.json"),
      "the refusal must name the file to open: {rendered}"
    );
  }

  /// Ordering is by directory, which is what v2's glob does.
  #[test]
  fn plugins_come_back_in_directory_order() {
    let tmp = tempfile::tempdir().expect("tempdir");
    for name in ["zeta", "alpha", "mid"] {
      manifest(tmp.path(), name, &valid(name));
    }
    let names: Vec<String> = discover(tmp.path())
      .expect("discover")
      .into_iter()
      .map(|p| p.name)
      .collect();
    assert_eq!(names, vec!["alpha", "mid", "zeta"]);
  }

  /// `find` resolves by the name the manifest DECLARES, not by the directory.
  ///
  /// The two are only incidentally equal. Joining the path would locate a
  /// plugin by one key and then print the other, which is the quiet kind of
  /// wrong -- the output looks right until the two disagree.
  #[test]
  fn find_resolves_the_declared_name_and_not_the_directory_name() {
    let tmp = tempfile::tempdir().expect("tempdir");
    manifest(tmp.path(), "on-disk", &valid("declared"));

    assert!(
      find(tmp.path(), "declared").expect("find").is_some(),
      "the declared name must resolve"
    );
    assert!(
      find(tmp.path(), "on-disk").expect("find").is_none(),
      "the directory name must NOT resolve -- it is not what the listing prints"
    );
  }
}
