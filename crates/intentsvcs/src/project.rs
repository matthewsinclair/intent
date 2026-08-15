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

/// The v2 release a project must already be at before v3 can migrate it (D09,
/// migration.md). Below it, v2's own `intent upgrade` runs first -- v3 never
/// reimplements the v2 ledger.
pub const MIGRATION_FLOOR: (u64, u64, u64) = (2, 19, 0);

/// Whether this project's canon is in a form THIS binary can read.
///
/// The question exists because "no threads found" and "threads this binary
/// cannot see" are the same empty vector, and answering the second as though
/// it were the first is a confident lie about someone's work (AC-10.7).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Migration {
  /// v3 canon, or a genuinely empty project.
  Done,
  Pending(Pending),
}

/// A project v3 must not answer questions about yet, and everything the
/// operator needs to tell this state from an empty estate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pending {
  /// What `config.json` declares, verbatim -- reported as it stands rather
  /// than normalised, because the operator has to find it in the file.
  pub declared: String,
  /// Below [`MIGRATION_FLOOR`], so the remedy is the two-hop rather than a v3
  /// migration that would refuse.
  pub below_floor: bool,
  /// Thread ids carrying v2 canon this binary cannot read, sorted.
  pub legacy_threads: Vec<String>,
}

impl Pending {
  /// What the operator should DO -- and the two states must not share a text,
  /// because one of them names a command that will refuse them.
  pub fn remedy(&self) -> String {
    if self.below_floor {
      let (major, minor, patch) = MIGRATION_FLOOR;
      format!(
        "this project is below the v{major}.{minor}.{patch} migration floor -- run `install intent@2 && intent upgrade` first, then migrate it with v3"
      )
    } else {
      "run `intent upgrade` to migrate this project to Intent v3".to_string()
    }
  }
}

impl std::fmt::Display for Pending {
  /// One line, inside the existing refusal grammar. A second report shape
  /// would be a second thing to learn for the same job.
  ///
  /// It deliberately does NOT name `config.json`. The remedy is a command, so
  /// the path buys the operator nothing -- and it invites the one repair that
  /// makes things worse: hand-editing `intent_version` to 3.0.0 produces a
  /// config claiming v3 over canon that is not, which is the half-migrated
  /// estate this whole check exists to catch. `doctor` still points at the
  /// file, because pointing at artefacts is what `doctor` is for and its
  /// finding carries the path in its own field.
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "this project has not been migrated to Intent v3 -- it declares Intent {}",
      self.declared
    )?;
    match self.legacy_threads.len() {
      0 => Ok(()),
      // Named, not just counted: a count alone is indistinguishable from a
      // count of nothing, and the ids are how the operator recognises their
      // own work. Capped, because this repository has 56 of them at f7434f1
      // and an error message is not a listing.
      n => write!(
        f,
        ", and {n} steel thread{} carr{} v2 canon this binary cannot read ({})",
        if n == 1 { "" } else { "s" },
        if n == 1 { "ies" } else { "y" },
        summarise(&self.legacy_threads)
      ),
    }
  }
}

fn summarise(ids: &[String]) -> String {
  const SHOWN: usize = 3;
  if ids.len() <= SHOWN {
    return ids.join(", ");
  }
  format!(
    "{}, and {} more",
    ids[..SHOWN].join(", "),
    ids.len() - SHOWN
  )
}

/// Walk `dir` for v2 thread directories, descending exactly one level into
/// anything that is not itself a thread -- which is what v2's `st/<STATUS>/`
/// archive directories are.
///
/// One level, not arbitrary depth: v2 has exactly this shape, and a full walk
/// would start reporting thread-shaped directories from unrelated trees
/// (a vendored checkout, a fixture directory) as this project's own canon.
fn collect_legacy(dir: &std::path::Path, descend: bool, out: &mut Vec<String>) {
  let Ok(entries) = std::fs::read_dir(dir) else {
    return;
  };
  for path in entries.filter_map(Result::ok).map(|e| e.path()) {
    if !path.is_dir() || path.join("thread.json").is_file() {
      continue;
    }
    match path.join("info.md").is_file() {
      true => out.extend(
        path
          .file_name()
          .and_then(|s| s.to_str())
          .map(str::to_string),
      ),
      false if descend => collect_legacy(&path, false, out),
      false => {}
    }
  }
}

/// `major.minor.patch`, ignoring any pre-release suffix.
///
/// Returns `None` rather than guessing. An unparseable version means the
/// declaration ABSTAINS -- the evidence check still applies -- because a typo
/// in one config field must not brick the tool, and a version we cannot read
/// is not evidence of anything in either direction.
fn parse_version(text: &str) -> Option<(u64, u64, u64)> {
  let core = text.split(['-', '+']).next()?;
  let mut parts = core.split('.').map(str::parse::<u64>);
  match (parts.next(), parts.next(), parts.next()) {
    (Some(Ok(major)), Some(Ok(minor)), Some(Ok(patch))) => Some((major, minor, patch)),
    _ => None,
  }
}

/// A resolved project.
#[derive(Debug, Clone)]
pub struct Project {
  root: PathBuf,
  config: Config,
}

impl Project {
  /// `intent/.config/config.json` under `root`. The marker file's location is
  /// FIXED -- it is what identifies a project, so it cannot itself be read
  /// from the project's config.
  pub fn config_path(root: &Path) -> PathBuf {
    root.join("intent").join(".config").join("config.json")
  }

  /// Open the project rooted exactly at `root`.
  pub fn open(root: &Path) -> Result<Self, ProjectError> {
    let path = Self::config_path(root);
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
      if Self::config_path(dir).is_file() {
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

  /// A work package's generated cover, `st/<ID>/WP/<NN>/info.md`.
  pub fn wp_info_view(&self, id: &str, seq: u32) -> PathBuf {
    self
      .thread_dir(id)
      .join("WP")
      .join(format!("{seq:02}"))
      .join("info.md")
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

  /// Whether this project's canon is in a form this binary can read.
  ///
  /// TWO independent signals, and each closes a hole the other cannot see:
  ///
  /// - the **declaration** -- `intent_version` below 3 -- catches a project
  ///   with nothing in it yet, which has no evidence to find and is still not
  ///   a v3 project, because migration is what stamps the version;
  /// - the **evidence** -- a thread directory with a v2 `info.md` and no v3
  ///   `thread.json` -- catches a config claiming 3.0.0 over canon that is
  ///   not, which is a HALF-migrated estate and strictly worse than an
  ///   unmigrated one: half of it answers and half is invisible.
  ///
  /// `project_id` is deliberately not the marker, though D15's wording makes
  /// it look like the obvious candidate. It is a migration-PROVENANCE stamp,
  /// and a project created natively under v3 was never migrated at all, so
  /// gating on it would refuse every project that never needed migrating.
  ///
  /// The evidence check is a `read_dir` plus two `stat`s per entry, and it is
  /// paid on every read. Measured A/B on a 200-thread v3 project, debug build,
  /// `intent st list` x60 interleaved: **+0.61 ms** against a 13.74 ms floor.
  /// Interleaved and taking the minimum because the first attempt at this
  /// measured the unchecked build as SLOWER -- process-spawn noise is larger
  /// than the effect, so a before/after pair of runs says nothing.
  ///
  /// That is the price of never mistaking "cannot see it" for "it is not
  /// there", and it is the right side of hv's daily-driver ruling: sync is
  /// allowed to be expensive, reads are not, and half a millisecond is not
  /// where reads become expensive.
  pub fn migration(&self) -> Migration {
    let parsed = parse_version(&self.config.intent_version);
    let declared_pre_v3 = parsed.is_some_and(|(major, _, _)| major < 3);
    let legacy_threads = self.legacy_thread_ids();

    if !declared_pre_v3 && legacy_threads.is_empty() {
      return Migration::Done;
    }
    Migration::Pending(Pending {
      declared: self.config.intent_version.clone(),
      below_floor: parsed.is_some_and(|v| v < MIGRATION_FLOOR),
      legacy_threads,
    })
  }

  /// Thread directories carrying v2 canon and no v3 canon, sorted.
  ///
  /// The discriminator is `thread.json` being ABSENT, never `info.md` being
  /// present: v3 renders `info.md` as a generated view, so every healthy
  /// migrated thread has both, and a rule keyed on `info.md` alone would flag
  /// every project in existence.
  ///
  /// **Two levels, because v2 has two.** `intent st done` RELOCATES a thread
  /// to `st/<STATUS>/<ID>/`, so a one-level scan sees only the threads that
  /// are still open. Measured on this repository when this was one level: it
  /// found ST0056 and missed the 55 threads under `COMPLETED/`, `CANCELLED/`
  /// and `NOT-STARTED/`. The declaration signal happened to catch it anyway,
  /// which is exactly how a hole like this survives -- the case it fails on is
  /// a project whose live threads are migrated and whose ARCHIVE is not, and
  /// that project would have read as fully migrated.
  fn legacy_thread_ids(&self) -> Vec<String> {
    let mut ids = Vec::new();
    collect_legacy(&self.st_dir(), true, &mut ids);
    ids.sort();
    ids.dedup();
    ids
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
