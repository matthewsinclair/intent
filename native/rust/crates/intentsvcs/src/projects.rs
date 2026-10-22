//! The project registry: the Intent projects this machine knows about, in
//! `$XDG_CONFIG_HOME/intent/projects.json` (ST0074 WP-03).
//!
//! **A FILE A HUMAN EDITS AND THREE PROGRAMS READ.** `intent explore` adds the
//! project it opens, `intent discover` adds the compatible projects it finds, an
//! operator adds or removes entries by hand, and `intentd` reads it and never
//! writes it. So the document is the operator's: a key this build does not know
//! is carried through every rewrite, and a schema newer than this build is read
//! and never rewritten.
//!
//! **IT LISTS ROOTS AND NOTHING THAT CHANGES.** No status, counts or times: each
//! project's own store answers those, and a copy here would be a second home
//! that goes stale without saying so.

use std::path::{Path, PathBuf};

use serde_json::{Map, Value};
use thiserror::Error;

use crate::project::{Migration, Project, ProjectError};

/// The schema this build writes, and the newest it will rewrite.
pub const SCHEMA: u64 = 1;

/// How many times [`add`] re-reads a file another writer changed under it.
const ATTEMPTS: usize = 3;

/// Which writer added an entry, recorded in its `added_by`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddedBy {
  Explore,
  Discover,
}

impl AddedBy {
  fn as_str(self) -> &'static str {
    match self {
      Self::Explore => "explore",
      Self::Discover => "discover",
    }
  }
}

#[derive(Debug, Error)]
pub enum ProjectsError {
  #[error("`{}` could not be read", path.display())]
  Unreadable {
    path: PathBuf,
    #[source]
    cause: std::io::Error,
  },
  #[error("`{}` is not a project registry: {why}", path.display())]
  Malformed { path: PathBuf, why: String },
  #[error("`{}` declares schema {schema}, newer than this build's {SCHEMA}", path.display())]
  Newer { path: PathBuf, schema: u64 },
  #[error("`{}` could not be written", path.display())]
  Unwritable {
    path: PathBuf,
    #[source]
    cause: std::io::Error,
  },
  #[error("`{}` kept changing while it was being updated", path.display())]
  Contended { path: PathBuf },
}

impl crate::remedy::Remedy for ProjectsError {
  fn remedy(&self) -> String {
    match self {
      Self::Unreadable { .. } => "check the file's permissions. Nothing was changed.".to_string(),
      Self::Malformed { .. } => "the registry is JSON shaped {\"schema\": 1, \"projects\": [{\"root\": \"/path/to/project\"}]}. Fix it by hand, or move it aside and run `intent discover` to write a new one. Nothing was changed.".to_string(),
      Self::Newer { .. } => "a newer Intent wrote this file, and this build does not rewrite what it cannot fully read. Upgrade Intent, or add the project to the file by hand.".to_string(),
      Self::Unwritable { .. } => "check that the directory is writable. The registry was not changed.".to_string(),
      Self::Contended { .. } => "another writer was changing the file at the same moment. Run the command again.".to_string(),
    }
  }
}

/// A registry document, as read.
///
/// **THE ENTRIES ARE HELD APART FROM THE REST OF THE DOCUMENT** so that
/// [`Registry::parse`] establishes their shape once, and every later step works
/// on a list it knows is a list rather than re-checking a `Value`.
#[derive(Debug, Clone, PartialEq)]
pub struct Registry {
  doc: Map<String, Value>,
  projects: Vec<Map<String, Value>>,
}

impl Registry {
  /// An empty registry, which is what a missing file means.
  pub fn empty() -> Registry {
    let mut doc = Map::new();
    doc.insert("schema".to_string(), Value::from(SCHEMA));
    Registry {
      doc,
      projects: Vec::new(),
    }
  }

  /// A registry from its text. `path` only names the file in a refusal.
  pub fn parse(path: &Path, text: &str) -> Result<Registry, ProjectsError> {
    let malformed = |why: String| ProjectsError::Malformed {
      path: path.to_path_buf(),
      why,
    };
    let Value::Object(mut doc) =
      serde_json::from_str::<Value>(text).map_err(|e| malformed(e.to_string()))?
    else {
      return Err(malformed("the top level is not an object".to_string()));
    };
    if !doc.get("schema").is_some_and(Value::is_u64) {
      return Err(malformed("`schema` is not a whole number".to_string()));
    }
    let Some(Value::Array(entries)) = doc.remove("projects") else {
      return Err(malformed("`projects` is not a list".to_string()));
    };
    let projects = entries
      .into_iter()
      .map(|entry| match entry {
        Value::Object(entry) if entry.get("root").is_some_and(Value::is_string) => Ok(entry),
        _ => Err(malformed(
          "an entry in `projects` is not an object with a `root` string".to_string(),
        )),
      })
      .collect::<Result<Vec<_>, _>>()?;
    Ok(Registry { doc, projects })
  }

  /// The schema the document declares.
  pub fn schema(&self) -> u64 {
    self
      .doc
      .get("schema")
      .and_then(Value::as_u64)
      .unwrap_or(SCHEMA)
  }

  /// Every listed root, as written, in file order.
  pub fn roots(&self) -> Vec<PathBuf> {
    self
      .projects
      .iter()
      .filter_map(|entry| entry.get("root")?.as_str())
      .map(PathBuf::from)
      .collect()
  }

  /// This registry with each of `roots` not already listed appended, and the
  /// roots that were appended. `None` when every one was already listed.
  pub fn with_roots(
    &self,
    roots: &[PathBuf],
    added_by: AddedBy,
  ) -> Option<(Registry, Vec<PathBuf>)> {
    let mut listed = self.roots();
    let mut next = self.clone();
    let mut added = Vec::new();
    for root in roots {
      if listed.contains(root) {
        continue;
      }
      let mut entry = Map::new();
      entry.insert(
        "root".to_string(),
        Value::String(root.to_string_lossy().into_owned()),
      );
      entry.insert(
        "added_by".to_string(),
        Value::String(added_by.as_str().to_string()),
      );
      next.projects.push(entry);
      listed.push(root.clone());
      added.push(root.clone());
    }
    (!added.is_empty()).then_some((next, added))
  }

  /// This registry without each listed entry whose root is one of `roots`, and
  /// the roots that were removed. `None` when none of them was listed.
  pub fn without_roots(&self, roots: &[PathBuf]) -> Option<(Registry, Vec<PathBuf>)> {
    let mut next = self.clone();
    let mut removed = Vec::new();
    next.projects.retain(|entry| {
      let Some(root) = entry.get("root").and_then(Value::as_str).map(PathBuf::from) else {
        return true;
      };
      if !roots.contains(&root) {
        return true;
      }
      if !removed.contains(&root) {
        removed.push(root);
      }
      false
    });
    (!removed.is_empty()).then_some((next, removed))
  }

  /// The document's text: two-space JSON with a trailing newline, so an
  /// operator's editor and this build write the same shape.
  pub fn render(&self) -> String {
    let mut doc = self.doc.clone();
    doc.insert(
      "projects".to_string(),
      Value::Array(self.projects.iter().cloned().map(Value::Object).collect()),
    );
    // A `Value` built from maps, strings and numbers has no failing case in
    // `serde_json`'s serialiser: its errors are for `Serialize` impls and I/O.
    #[allow(
      clippy::expect_used,
      reason = "INVARIANT: a Value built from maps, strings and numbers has no failing case in serde_json's serialiser"
    )]
    let mut text =
      serde_json::to_string_pretty(&Value::Object(doc)).expect("a JSON value serialises");
    text.push('\n');
    text
  }
}

/// The text at `path`, or `None` when there is no file.
fn read(path: &Path) -> Result<Option<String>, ProjectsError> {
  match std::fs::read_to_string(path) {
    Ok(text) => Ok(Some(text)),
    Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
    Err(cause) => Err(ProjectsError::Unreadable {
      path: path.to_path_buf(),
      cause,
    }),
  }
}

/// The registry at `path`. A missing file is an empty registry.
pub fn load(path: &Path) -> Result<Registry, ProjectsError> {
  match read(path)? {
    Some(text) => Registry::parse(path, &text),
    None => Ok(Registry::empty()),
  }
}

/// Add each of `roots` the registry at `path` does not list, returning the ones
/// added. The roots are stored as given, so the caller passes canonical ones:
/// intentd keys projects by canonical root, and a second spelling of one project
/// would list it twice. Written through `rewrite`, the file's one writer.
pub fn add(
  path: &Path,
  roots: &[PathBuf],
  added_by: AddedBy,
) -> Result<Vec<PathBuf>, ProjectsError> {
  rewrite(path, |registry| registry.with_roots(roots, added_by))
}

/// Remove each of `roots` the registry at `path` lists, returning the ones
/// removed (ST0083 WP-06, `intent deinit --all`). The roots are compared as
/// given, so the caller passes canonical ones, as [`add`] stores them.
pub fn remove(path: &Path, roots: &[PathBuf]) -> Result<Vec<PathBuf>, ProjectsError> {
  if !path.exists() {
    return Ok(Vec::new());
  }
  rewrite(path, |registry| registry.without_roots(roots))
}

/// Apply `change` to the registry at `path` and write the result, returning
/// the roots it reports. **THE ONE WRITER OF THE FILE**, for [`add`] and
/// [`remove`] alike.
///
/// **THE WRITE IS A RENAME, AND IT CHECKS THE FILE DID NOT MOVE UNDER IT.** A
/// temporary file beside the registry is renamed over it, so a reader never sees
/// half a document. If the file changed between the read and the rename,
/// another writer got there first, so this re-reads and re-applies its one
/// change rather than overwriting theirs.
fn rewrite(
  path: &Path,
  change: impl Fn(&Registry) -> Option<(Registry, Vec<PathBuf>)>,
) -> Result<Vec<PathBuf>, ProjectsError> {
  let unwritable = |at: &Path, cause| ProjectsError::Unwritable {
    path: at.to_path_buf(),
    cause,
  };
  for _ in 0..ATTEMPTS {
    let before = read(path)?;
    let registry = match &before {
      Some(text) => Registry::parse(path, text)?,
      None => Registry::empty(),
    };
    if registry.schema() > SCHEMA {
      return Err(ProjectsError::Newer {
        path: path.to_path_buf(),
        schema: registry.schema(),
      });
    }
    let Some((next, changed)) = change(&registry) else {
      return Ok(Vec::new());
    };
    if let Some(dir) = path.parent() {
      std::fs::create_dir_all(dir).map_err(|cause| unwritable(dir, cause))?;
    }
    let temporary = path.with_extension(format!("json.{}.tmp", std::process::id()));
    std::fs::write(&temporary, next.render()).map_err(|cause| unwritable(&temporary, cause))?;
    if read(path)? != before {
      std::fs::remove_file(&temporary).map_err(|cause| unwritable(&temporary, cause))?;
      continue;
    }
    std::fs::rename(&temporary, path).map_err(|cause| unwritable(path, cause))?;
    return Ok(changed);
  }
  Err(ProjectsError::Contended {
    path: path.to_path_buf(),
  })
}

/// Why a directory holding an Intent config was not registered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refusal {
  /// The config is missing, unreadable or not a config.
  ConfigDoesNotParse(String),
  /// v3 would refuse it until it is migrated (`AC-10.7`); carries what it declares.
  NeedsMigration(String),
  /// It sits in a bare git repository (issue 0609), whatever its config holds.
  /// Named apart from `ConfigDoesNotParse`, whose label would send the operator
  /// to a config that parses. It carries no path: the line that reports it
  /// already names the root.
  BareRepository,
}

impl std::fmt::Display for Refusal {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::ConfigDoesNotParse(why) => write!(f, "config does not parse: {why}"),
      Self::NeedsMigration(declared) => {
        write!(
          f,
          "needs migration (declares {declared}); `intent upgrade` there migrates it"
        )
      }
      Self::BareRepository => write!(f, "in {}", crate::project::BARE_REPOSITORY),
    }
  }
}

/// Whether this build can serve the project at `root`: its config opens, and
/// it is not a project v3 refuses until it is migrated.
pub fn assess(root: &Path) -> Result<(), Refusal> {
  let project = Project::open(root).map_err(|e| match e {
    ProjectError::BareRepository(_) => Refusal::BareRepository,
    other => Refusal::ConfigDoesNotParse(other.to_string()),
  })?;
  match project.migration() {
    Migration::Done => Ok(()),
    Migration::Pending(pending) => Err(Refusal::NeedsMigration(pending.declared)),
  }
}

/// Directory names a walk for projects never enters.
const SKIPPED: &[&str] = &[".git", "target", "node_modules"];

/// Every directory under `from`, down to `depth` levels, holding an Intent
/// config, and every path the walk could not read.
///
/// **A FOUND PROJECT IS NOT DESCENDED INTO.** A project's own tree holds
/// fixtures and vendored checkouts that carry configs of their own, and
/// registering those would list test data as work. The walk honours
/// `.gitignore` the way `sync.rs` and `project.rs` do, and follows no symlinks,
/// so a link back up the tree cannot make it loop.
pub fn find(from: &Path, depth: usize) -> (Vec<PathBuf>, Vec<String>) {
  let mut walk = ignore::WalkBuilder::new(from);
  walk
    .max_depth(Some(depth))
    .follow_links(false)
    .git_global(false)
    .git_exclude(false)
    .filter_entry(|entry| {
      let skipped = SKIPPED.iter().any(|name| entry.file_name() == *name);
      let inside_a_project = entry.depth() > 0
        && entry
          .path()
          .parent()
          .is_some_and(|parent| Project::config_path(parent).is_file());
      !skipped && !inside_a_project
    });
  let mut found = Vec::new();
  let mut unreadable = Vec::new();
  for entry in walk.build() {
    match entry {
      Ok(entry)
        if entry.file_type().is_some_and(|kind| kind.is_dir())
          && Project::config_path(entry.path()).is_file() =>
      {
        found.push(entry.into_path())
      }
      Ok(_) => {}
      Err(e) => unreadable.push(e.to_string()),
    }
  }
  found.sort();
  (found, unreadable)
}

/// What [`discover`] did with each project it found.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Discovered {
  pub registered: Vec<PathBuf>,
  pub already: Vec<PathBuf>,
  pub refused: Vec<(PathBuf, Refusal)>,
  pub unreadable: Vec<String>,
}

/// Find the projects under `from`, and add the compatible ones to the registry
/// at `registry`. `from` is canonical, so every root found under it is too.
///
/// **IT MIGRATES AND REPAIRS NOTHING.** A project that cannot be registered is
/// named with the reason, and fixing it is the operator's act.
pub fn discover(registry: &Path, from: &Path, depth: usize) -> Result<Discovered, ProjectsError> {
  let (found, unreadable) = find(from, depth);
  let mut compatible = Vec::new();
  let mut refused = Vec::new();
  for root in found {
    match assess(&root) {
      Ok(()) => compatible.push(root),
      Err(why) => refused.push((root, why)),
    }
  }
  let registered = add(registry, &compatible, AddedBy::Discover)?;
  let already = compatible
    .into_iter()
    .filter(|root| !registered.contains(root))
    .collect();
  Ok(Discovered {
    registered,
    already,
    refused,
    unreadable,
  })
}

/// One registered project as a refusal lists it: how it can be named, and
/// where it is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidate {
  /// The directory the project lives in, [`Project::directory_name`].
  pub directory: String,
  /// The name the project's config declares.
  pub project_name: String,
  pub root: PathBuf,
}

impl std::fmt::Display for Candidate {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{} (project_name {}) at {}",
      self.directory,
      self.project_name,
      self.root.display()
    )
  }
}

fn listed(candidates: &[Candidate]) -> String {
  candidates.iter().map(|c| format!("\n  - {c}")).collect()
}

fn unopenable(roots: &[(PathBuf, String)]) -> String {
  if roots.is_empty() {
    return String::new();
  }
  let lines: String = roots
    .iter()
    .map(|(root, why)| format!("\n  - {} ({why})", root.display()))
    .collect();
  format!("\nregistered roots that do not open as a project, never matched:{lines}")
}

#[derive(Debug, Error)]
pub enum ResolveError {
  #[error(transparent)]
  Registry(#[from] ProjectsError),
  #[error("`{wanted}` names more than one registered project:{}", listed(.candidates))]
  Ambiguous {
    wanted: String,
    candidates: Vec<Candidate>,
  },
  #[error("`{wanted}` names no registered project{}{}", if .registered.is_empty() { " (the registry lists none)".to_string() } else { format!(". The registry lists:{}", listed(.registered)) }, unopenable(.stale))]
  NoMatch {
    wanted: String,
    registered: Vec<Candidate>,
    stale: Vec<(PathBuf, String)>,
  },
}

impl crate::remedy::Remedy for ResolveError {
  fn remedy(&self) -> String {
    match self {
      Self::Registry(e) => e.remedy(),
      Self::Ambiguous { .. } => "name it by its registered root or its project_id, which are unique. Nothing was read or written.".to_string(),
      Self::NoMatch { .. } => "name a project by its root, its project_id, its directory name or its project_name. `intent discover <dir>` registers the projects under a directory. Nothing was read or written.".to_string(),
    }
  }
}

/// The registered project `wanted` names (issue 0590).
///
/// **TWO TIERS, AND THE FIRST CANNOT BE SHADOWED BY THE SECOND.** An exact
/// name, a path (one spelled with a `/`) to the root or the project's `project_id`,
/// wins outright when one project carries it. Only then is `wanted` read as a
/// name, case-insensitive, against each project's directory name and its
/// `project_name`, because the directory is what tells two checkouts of one
/// project apart (hv, 2026-09-03: `Intentv2` declares itself `Intent`).
///
/// **NEVER THE WORKING DIRECTORY'S PROJECT.** Two matches are refused naming
/// both and none is refused listing the registry, so an issue can never be
/// filed in an estate the caller did not name. A registered root that no
/// longer opens is listed in the refusal and never matched.
pub fn resolve(registry: &Path, wanted: &str) -> Result<Project, ResolveError> {
  let mut open = Vec::new();
  let mut stale = Vec::new();
  for root in load(registry)?.roots() {
    match Project::open(&root) {
      Ok(project) => open.push(project),
      Err(e) => stale.push((root, e.to_string())),
    }
  }
  let candidate = |p: &Project| Candidate {
    directory: p.directory_name().unwrap_or_default(),
    project_name: p.config().project_name.clone(),
    root: p.root().to_path_buf(),
  };
  // A value is read as a path only when it is spelled as one, so a bare name
  // that happens to be a directory under the working directory stays a name.
  let wanted_path = wanted
    .contains('/')
    .then(|| std::fs::canonicalize(wanted).ok())
    .flatten();
  let exact = |p: &Project| {
    (wanted_path.is_some() && std::fs::canonicalize(p.root()).ok() == wanted_path)
      || p.config().identity() == Some(wanted)
  };
  let folded = wanted.to_lowercase();
  let named = |p: &Project| {
    p.directory_name().map(|d| d.to_lowercase()) == Some(folded.clone())
      || p.config().project_name.to_lowercase() == folded
  };
  for tier in [&exact as &dyn Fn(&Project) -> bool, &named] {
    let mut hits: Vec<Project> = open.iter().filter(|p| tier(p)).cloned().collect();
    match hits.len() {
      0 => continue,
      1 => return Ok(hits.remove(0)),
      _ => {
        return Err(ResolveError::Ambiguous {
          wanted: wanted.to_string(),
          candidates: hits.iter().map(candidate).collect(),
        });
      }
    }
  }
  Err(ResolveError::NoMatch {
    wanted: wanted.to_string(),
    registered: open.iter().map(candidate).collect(),
    stale,
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  fn project(root: &Path, version: &str) {
    std::fs::create_dir_all(root.join("intent/.config")).expect("config dir");
    std::fs::write(
      Project::config_path(root),
      format!("{{\"intent_version\": \"{version}\"}}"),
    )
    .expect("config");
  }

  /// `AT-03.1` (ST0074 WP-03): a top-level key, an entry key and a note this
  /// build does not know all survive a rewrite, beside the root it added.
  #[test]
  fn a_rewrite_keeps_what_the_operator_wrote() {
    let text = r#"{"schema": 1, "mine": {"kept": true}, "projects": [{"root": "/a", "note": "the old one", "colour": "red"}]}"#;
    let registry = Registry::parse(Path::new("projects.json"), text).expect("parses");
    let (next, added) = registry
      .with_roots(
        &[PathBuf::from("/a"), PathBuf::from("/b")],
        AddedBy::Discover,
      )
      .expect("one root is new");
    assert_eq!(added, [PathBuf::from("/b")]);

    let rendered = next.render();
    let reread = Registry::parse(Path::new("projects.json"), &rendered).expect("round-trips");
    assert_eq!(reread.roots(), [PathBuf::from("/a"), PathBuf::from("/b")]);
    let doc: Value = serde_json::from_str(&rendered).expect("json");
    assert_eq!(doc["mine"]["kept"], Value::Bool(true));
    assert_eq!(doc["projects"][0]["note"], "the old one");
    assert_eq!(doc["projects"][0]["colour"], "red");
    assert_eq!(doc["projects"][1]["added_by"], "discover");
  }

  /// `AT-03.2`: a root already listed writes nothing, and a registry from a
  /// newer schema is refused and left byte for byte as it was.
  #[test]
  fn a_root_is_added_once_and_a_newer_schema_is_left_alone() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("intent/projects.json");
    let root = dir.path().join("work");

    assert_eq!(
      add(&path, std::slice::from_ref(&root), AddedBy::Explore).expect("first"),
      std::slice::from_ref(&root)
    );
    let written = std::fs::read_to_string(&path).expect("written");
    assert!(
      add(&path, std::slice::from_ref(&root), AddedBy::Explore)
        .expect("second")
        .is_empty()
    );
    assert_eq!(std::fs::read_to_string(&path).expect("reread"), written);

    let newer = r#"{"schema": 2, "projects": []}"#;
    std::fs::write(&path, newer).expect("newer");
    assert!(matches!(
      add(&path, &[root], AddedBy::Explore),
      Err(ProjectsError::Newer { schema: 2, .. })
    ));
    assert_eq!(std::fs::read_to_string(&path).expect("reread"), newer);
  }

  /// `AT-03.3`: discover registers the project v3 serves, names the unmigrated
  /// and the unparseable ones with the reason, finds nothing inside a project
  /// or under a skipped directory, and on a second run registers nothing new.
  #[test]
  fn discover_registers_the_compatible_and_names_the_rest() {
    let dir = tempfile::tempdir().expect("tempdir");
    let from = dir.path().canonicalize().expect("canonical");
    project(&from.join("current"), "3.0.2");
    project(&from.join("current/fixtures/inner"), "3.0.2");
    project(&from.join("old"), "2.9.0");
    project(&from.join("node_modules/vendored"), "3.0.2");
    std::fs::create_dir_all(from.join("broken/intent/.config")).expect("broken dir");
    std::fs::write(Project::config_path(&from.join("broken")), "not json").expect("broken config");
    let registry = from.join("config/projects.json");

    let first = discover(&registry, &from, 4).expect("discover");
    assert_eq!(first.registered, [from.join("current")]);
    assert!(first.already.is_empty());
    match first.refused.as_slice() {
      [
        (broken, Refusal::ConfigDoesNotParse(_)),
        (old, Refusal::NeedsMigration(declared)),
      ] => {
        assert_eq!(*broken, from.join("broken"));
        assert_eq!(*old, from.join("old"));
        assert_eq!(declared, "2.9.0");
      }
      other => panic!("expected broken and old refused with their reasons, got {other:?}"),
    }
    assert_eq!(
      load(&registry).expect("load").roots(),
      [from.join("current")]
    );

    let again = discover(&registry, &from, 4).expect("again");
    assert!(again.registered.is_empty());
    assert_eq!(again.already, [from.join("current")]);
  }
}
