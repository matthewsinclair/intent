//! The project: a root directory, its config, and the canonical location of
//! every artefact under it.
//!
//! **This is the one place paths are decided.** v2's defect class was that
//! every reader reimplemented parsing and path resolution, so every reader
//! answered confidently from partial evidence; the fix is not "be careful", it
//! is having exactly one module that can answer "where does a thread live".
//! Nothing else in intentsvcs joins a path by hand.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// The per-project config (`intent/.config/config.json`).
///
/// Unknown fields are PERMITTED here, and that is deliberate rather than an
/// oversight of D05. config.json is the one canon file v2 also writes, plugins
/// extend it with their own blocks, and a v3 binary that refused a config it
/// did not fully recognise would brick a project the moment any other tool
/// touched it. Strictness belongs on the artefacts v3 owns outright.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
  pub intent_version: String,
  #[serde(default)]
  pub project_name: String,
  #[serde(default)]
  pub author: String,
  /// Stamped at migration (D15); absent on a v2 project, which is how the
  /// migrator tells "not yet migrated" from "migrated".
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub project_id: Option<String>,
  #[serde(default = "default_intent_dir")]
  pub intent_dir: String,
  #[serde(default = "default_st_prefix")]
  pub st_prefix: String,
  #[serde(default)]
  pub languages: Vec<String>,
  /// Everything else in the file, carried so a rewrite never drops a block
  /// this version does not know about.
  #[serde(flatten)]
  pub extra: serde_json::Map<String, serde_json::Value>,
}

fn default_intent_dir() -> String {
  "intent".to_string()
}

fn default_st_prefix() -> String {
  "ST".to_string()
}

#[derive(Debug, thiserror::Error)]
pub enum ProjectError {
  #[error("no Intent project found at or above {0} (looked for intent/.config/config.json)")]
  NotFound(String),
  #[error("reading {path}: {source}")]
  Io {
    path: String,
    #[source]
    source: std::io::Error,
  },
  #[error("{path} is not valid Intent config: {source}")]
  Config {
    path: String,
    #[source]
    source: serde_json::Error,
  },
}

/// A resolved project.
#[derive(Debug, Clone)]
pub struct Project {
  root: PathBuf,
  config: Config,
}

impl Project {
  /// Open the project rooted exactly at `root`.
  pub fn open(root: &Path) -> Result<Self, ProjectError> {
    let path = root.join("intent").join(".config").join("config.json");
    let text = std::fs::read_to_string(&path).map_err(|source| ProjectError::Io {
      path: path.display().to_string(),
      source,
    })?;
    let config = serde_json::from_str(&text).map_err(|source| ProjectError::Config {
      path: path.display().to_string(),
      source,
    })?;
    Ok(Self {
      root: root.to_path_buf(),
      config,
    })
  }

  /// Walk up from `start` to the first directory holding an Intent config.
  ///
  /// The marker is the config file's own presence -- never an environment
  /// variable. A generic env var answers "did someone set a variable?" while
  /// meaning "am I in a project?", and those agree often enough to look correct
  /// and differ exactly when it matters (issue 0025).
  pub fn discover(start: &Path) -> Result<Self, ProjectError> {
    for dir in start.ancestors() {
      if dir
        .join("intent")
        .join(".config")
        .join("config.json")
        .is_file()
      {
        return Self::open(dir);
      }
    }
    Err(ProjectError::NotFound(start.display().to_string()))
  }

  pub fn root(&self) -> &Path {
    &self.root
  }

  /// This project's spelling of [`relative`].
  pub fn relative(&self, path: &Path) -> String {
    relative(&self.root, path)
  }

  pub fn config(&self) -> &Config {
    &self.config
  }

  /// `intent/` -- configurable in v2, so read rather than assumed.
  pub fn intent_dir(&self) -> PathBuf {
    self.root.join(&self.config.intent_dir)
  }

  pub fn st_dir(&self) -> PathBuf {
    self.intent_dir().join("st")
  }

  /// `intent/st/<ID>/` -- a thread's directory, whether or not it exists.
  pub fn thread_dir(&self, id: &str) -> PathBuf {
    self.st_dir().join(id)
  }

  /// The committed structured canon for one thread.
  pub fn thread_json(&self, id: &str) -> PathBuf {
    self.thread_dir(id).join("thread.json")
  }

  pub fn issues_dir(&self) -> PathBuf {
    self.intent_dir().join("issues")
  }

  /// The committed structured canon for one issue, `issues/<nnnn>.json`.
  pub fn issue_json(&self, number: u32) -> PathBuf {
    self.issues_dir().join(format!("{number:04}.json"))
  }

  /// The authored issue body, `issues/<nnnn>.md`.
  pub fn issue_md(&self, number: u32) -> PathBuf {
    self.issues_dir().join(format!("{number:04}.md"))
  }

  /// The runtime DB (D21) -- gitignored, disposable, rebuilt from canon.
  pub fn db_path(&self) -> PathBuf {
    self.intent_dir().join(".cache").join("intent.db")
  }

  /// The thread index view. Beside the threads it indexes, at v2's path (vc
  /// ruling, 2026-08-14): a rewrite that already moves plenty does not need to
  /// move this too.
  pub fn steel_threads_view(&self) -> PathBuf {
    self.st_dir().join("steel_threads.md")
  }

  /// The flat DOING / TODO / DONE view.
  pub fn todo_view(&self) -> PathBuf {
    self.intent_dir().join("todo.md")
  }

  /// A thread's generated cover.
  pub fn info_view(&self, id: &str) -> PathBuf {
    self.thread_dir(id).join("info.md")
  }

  /// A thread's generated acceptance contract + coverage map.
  pub fn acceptance_view(&self, id: &str) -> PathBuf {
    self.thread_dir(id).join("acceptance.md")
  }

  /// Every thread id with committed canon, sorted. Absent `st/` is an empty
  /// project, not an error -- `intent init` creates the directory lazily.
  pub fn thread_ids(&self) -> Result<Vec<String>, ProjectError> {
    let dir = self.st_dir();
    let Ok(entries) = std::fs::read_dir(&dir) else {
      return Ok(Vec::new());
    };
    let mut ids: Vec<String> = entries
      .filter_map(Result::ok)
      .filter(|e| e.path().join("thread.json").is_file())
      .filter_map(|e| e.file_name().to_str().map(str::to_string))
      .collect();
    ids.sort();
    Ok(ids)
  }

  /// Every issue number with committed canon, sorted.
  pub fn issue_numbers(&self) -> Result<Vec<u32>, ProjectError> {
    let Ok(entries) = std::fs::read_dir(self.issues_dir()) else {
      return Ok(Vec::new());
    };
    let mut numbers: Vec<u32> = entries
      .filter_map(Result::ok)
      .map(|e| e.path())
      .filter(|p| p.extension().is_some_and(|x| x == "json"))
      .filter_map(|p| {
        p.file_stem()
          .and_then(|s| s.to_str())
          .and_then(|s| s.parse().ok())
      })
      .collect();
    numbers.sort_unstable();
    Ok(numbers)
  }
}

/// A path as Intent names it: relative to the project root, forward-slashed on
/// every platform.
///
/// The one home for this. Findings, the file index and the skew check all
/// report paths, and three private copies of "make it relative" is precisely
/// the shape of drift the Highlander rule exists to prevent -- the copies
/// agree until one of them handles a prefix mismatch differently, and then two
/// subsystems disagree about what to call the same file.
pub fn relative(root: &Path, path: &Path) -> String {
  path
    .strip_prefix(root)
    .unwrap_or(path)
    .components()
    .map(|c| c.as_os_str().to_string_lossy())
    .collect::<Vec<_>>()
    .join("/")
}

#[cfg(test)]
mod tests {
  use super::*;

  fn fixture() -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    let config = dir.path().join("intent").join(".config");
    std::fs::create_dir_all(&config).expect("mkdir");
    std::fs::write(
      config.join("config.json"),
      r#"{"intent_version":"3.0.0","project_name":"Fixture","author":"cc","intent_dir":"intent","languages":["rust"],"plugins":{"claude":{}}}"#,
    )
    .expect("write config");
    dir
  }

  #[test]
  fn discover_walks_up_to_the_root() {
    let dir = fixture();
    let deep = dir.path().join("intent").join("st").join("ST0001");
    std::fs::create_dir_all(&deep).expect("mkdir");
    let project = Project::discover(&deep).expect("discover");
    assert_eq!(project.root(), dir.path());
    assert_eq!(project.config().project_name, "Fixture");
  }

  #[test]
  fn an_unknown_config_block_is_carried_not_dropped() {
    let dir = fixture();
    let project = Project::open(dir.path()).expect("open");
    assert!(
      project.config().extra.contains_key("plugins"),
      "a block this version does not model must survive a read"
    );
  }

  #[test]
  fn paths_are_derived_from_the_configured_intent_dir() {
    let dir = fixture();
    let project = Project::open(dir.path()).expect("open");
    assert_eq!(
      project.thread_json("ST0056"),
      dir.path().join("intent/st/ST0056/thread.json")
    );
    assert_eq!(
      project.issue_json(21),
      dir.path().join("intent/issues/0021.json")
    );
    assert_eq!(
      project.db_path(),
      dir.path().join("intent/.cache/intent.db")
    );
  }
}
