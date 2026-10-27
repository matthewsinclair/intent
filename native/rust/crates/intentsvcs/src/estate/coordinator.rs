//! The coordinator's working directory (ST0086 WP-04 Part B, AC-04.5).
//!
//! **A DEDICATED COORDINATOR RUNS ON INTENT ALONE.** `intent estate init
//! [<dir>]` renders the directory a Claude Code session is started in to
//! control the host's slots: `AGENTS.md` and `CLAUDE.md`, the coordinator's
//! contract, from `lib/templates/estate/` through the root files' expander, and
//! `.claude/settings.json` from the one template every project gets, so the
//! coordinator's hooks, the prompt hook that names its asks among them, are a
//! project's hooks.
//!
//! **READ LIVE FROM THE INSTALL, NOT EMBEDDED**, as the root files and the
//! hooks are: the directory's own hooks run `intent claude hook`, which needs
//! the install's scripts anyway, and the embedded table is pinned to what
//! `intent init` writes.
//!
//! **IT NEVER OVERWRITES** (vc's ruling, 2026-10-26): a file there that
//! differs from what it renders refuses the whole run, writing nothing, and a
//! re-run over files that match writes nothing.

use std::path::{Path, PathBuf};

use crate::project::{Config, Project, ProjectError};
use crate::rootfiles::Fault;
use crate::views::RenderContext;
use crate::write_set::{WriteError, WriteSet};

/// What `intent estate init` writes, each beside its template under the
/// install's templates: the contract from the coordinator's own, and the hooks
/// from the project template, so the hooks have one home.
const FILES: &[(&str, &str)] = &[
  ("AGENTS.md", "estate/_AGENTS.md"),
  ("CLAUDE.md", "estate/_CLAUDE.md"),
  (".claude/settings.json", ".claude/settings.json"),
];

#[derive(Debug, thiserror::Error)]
pub enum InitError {
  #[error(
    "`{}` is inside an Intent project ({project}), and the coordinator stands in no project",
    dir.display()
  )]
  InProject { dir: PathBuf, project: String },
  #[error("`{}` is there and is not a directory", path.display())]
  NotADirectory { path: PathBuf },
  #[error(
    "these are there and differ from what `intent estate init` renders: {}",
    shown(paths)
  )]
  Differs { paths: Vec<PathBuf> },
  #[error("the template `{}` could not be read", path.display())]
  Template {
    path: PathBuf,
    #[source]
    cause: std::io::Error,
  },
  #[error("the template `{}` is malformed: {fault}", path.display())]
  Malformed { path: PathBuf, fault: Fault },
  #[error("the coordinator's configuration could not be built")]
  Config(#[source] serde_json::Error),
  #[error("`{}` could not be read", path.display())]
  Unreadable {
    path: PathBuf,
    #[source]
    cause: std::io::Error,
  },
  #[error("`{}` could not be made", path.display())]
  Unmakeable {
    path: PathBuf,
    #[source]
    cause: std::io::Error,
  },
  #[error("the coordinator's files could not be written")]
  Unwritable(#[source] WriteError),
  #[error(transparent)]
  Install(#[from] crate::install::InstallError),
}

impl crate::remedy::Remedy for InitError {
  fn remedy(&self) -> String {
    match self {
      Self::InProject { .. } => "give it a directory outside every Intent project, eg `intent estate init ~/coordinator`. A project's own session that is assigned control needs no init: it takes control with `intent estate control take`, and the prompt hook names its asks. Nothing was written.".to_string(),
      Self::NotADirectory { .. } => "name a directory, or a path where one can be made. Nothing was written.".to_string(),
      Self::Differs { .. } => "move each aside, or remove it once you have kept what you need, then run `intent estate init` again; a re-run over files that match writes nothing. Nothing was written.".to_string(),
      Self::Template { .. } | Self::Malformed { .. } => "the Intent install is incomplete or its estate templates are malformed; reinstall it. Nothing was written.".to_string(),
      Self::Config(_) => "this is a defect in this build; report it with the command you ran. Nothing was written.".to_string(),
      Self::Unreadable { .. } => "check the file's permissions. Nothing was written.".to_string(),
      Self::Unmakeable { .. } => "check that the directory can be made where you named it. Nothing was written.".to_string(),
      Self::Unwritable(_) => "check that the directory is writable; what this run wrote was rolled back.".to_string(),
      Self::Install(e) => crate::remedy::Remedy::remedy(e),
    }
  }
}

fn shown(paths: &[PathBuf]) -> String {
  paths
    .iter()
    .map(|p| format!("`{}`", p.display()))
    .collect::<Vec<_>>()
    .join(", ")
}

/// One file `intent estate init` rendered: where, and whether this run wrote
/// it or found it already as rendered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rendered {
  pub path: PathBuf,
  pub wrote: bool,
}

/// The configuration the expander reads, for a directory that is no project:
/// the running tool's version, and every other key at the type's own default,
/// so no project's name, author or languages reach the coordinator's files.
fn coordinator_config(version: &str) -> Result<Config, InitError> {
  serde_json::from_value(serde_json::json!({ "intent_version": version }))
    .map_err(InitError::Config)
}

/// `intent estate init`: renders the coordinator's working directory at `dir`
/// from the templates under the install root `home`, `version` being the
/// running tool's.
///
/// **EVERY REFUSAL COMES BEFORE ANY WRITE**: inside a project, a path that is
/// not a directory, or any of the three files there and differing, and the
/// run writes nothing. The rest goes through one [`WriteSet`], which leaves a
/// file whose bytes already match untouched and says so.
pub fn init(dir: &Path, home: &Path, version: &str) -> Result<Vec<Rendered>, InitError> {
  let dir = std::path::absolute(dir).map_err(|cause| InitError::Unreadable {
    path: dir.to_path_buf(),
    cause,
  })?;
  if dir.exists() && !dir.is_dir() {
    return Err(InitError::NotADirectory { path: dir });
  }
  match Project::discover(&dir) {
    Err(ProjectError::NotFound(_)) => {}
    Ok(project) => {
      return Err(InitError::InProject {
        project: project.root().display().to_string(),
        dir,
      });
    }
    Err(other) => {
      return Err(InitError::InProject {
        project: other.to_string(),
        dir,
      });
    }
  }
  let cfg = coordinator_config(version)?;
  let ctx = RenderContext {
    version,
    todo_watermark: None,
  };
  let mut planned: Vec<(PathBuf, String)> = Vec::with_capacity(FILES.len());
  let mut differs = Vec::new();
  for (name, template) in FILES {
    let source = home.join(crate::install::MARKER).join(template);
    let text = std::fs::read_to_string(&source).map_err(|cause| InitError::Template {
      path: source.clone(),
      cause,
    })?;
    // The one expander for both: the Markdown rule for the contract, and the
    // expander alone for the JSON, as `rootfiles::expand` says.
    let rendered = if name.ends_with(".md") {
      crate::rootfiles::substitute(&text, &cfg, &ctx)
    } else {
      crate::rootfiles::expand(&text, &cfg, &ctx)
    };
    let content = rendered.map_err(|fault| InitError::Malformed {
      path: source.clone(),
      fault,
    })?;
    let path = dir.join(name);
    match std::fs::read_to_string(&path) {
      Ok(there) if there != content => differs.push(path),
      Ok(_) => planned.push((path, content)),
      Err(e) if e.kind() == std::io::ErrorKind::NotFound => planned.push((path, content)),
      Err(cause) => return Err(InitError::Unreadable { path, cause }),
    }
  }
  if !differs.is_empty() {
    return Err(InitError::Differs { paths: differs });
  }
  for (path, _) in &planned {
    if let Some(parent) = path.parent() {
      std::fs::create_dir_all(parent).map_err(|cause| InitError::Unmakeable {
        path: parent.to_path_buf(),
        cause,
      })?;
    }
  }
  let mut set = WriteSet::new();
  for (path, content) in &planned {
    set.add(path.clone(), content.clone());
  }
  let applied = set.commit().map_err(InitError::Unwritable)?;
  let written: Vec<PathBuf> = applied.written().map(Path::to_path_buf).collect();
  applied.keep();
  Ok(
    planned
      .into_iter()
      .map(|(path, _)| Rendered {
        wrote: written.contains(&path),
        path,
      })
      .collect(),
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  /// An install root holding the three templates, and a directory under it
  /// that is no project.
  struct Fixture {
    _dir: tempfile::TempDir,
    home: PathBuf,
    target: PathBuf,
  }

  fn fixture() -> Fixture {
    let dir = tempfile::tempdir().expect("tempdir");
    let home = dir.path().join("install");
    let templates = home.join(crate::install::MARKER);
    std::fs::create_dir_all(templates.join("estate")).expect("mkdir estate");
    std::fs::create_dir_all(templates.join(".claude")).expect("mkdir .claude");
    std::fs::write(
      templates.join("estate/_AGENTS.md"),
      "# The coordinator\n\nIntent v[[INTENT_VERSION]]\n",
    )
    .expect("write _AGENTS.md");
    std::fs::write(templates.join("estate/_CLAUDE.md"), "@AGENTS.md\n").expect("write _CLAUDE.md");
    std::fs::write(templates.join(".claude/settings.json"), "{\"hooks\": {}}\n")
      .expect("write settings.json");
    let target = dir.path().join("coordinator");
    Fixture {
      _dir: dir,
      home,
      target,
    }
  }

  /// ST0086 AT-04.5
  ///
  /// AC-04.5: the coordinator's directory is rendered from the install's
  /// templates through the expander, the hooks from the project template, and
  /// a re-run over what it rendered writes nothing.
  #[test]
  fn init_renders_the_contract_and_the_hooks_and_a_rerun_writes_nothing() {
    let fx = fixture();
    let first = init(&fx.target, &fx.home, "3.2.7").expect("init");
    assert!(first.iter().all(|r| r.wrote), "{first:?}");
    assert_eq!(
      std::fs::read_to_string(fx.target.join("AGENTS.md")).expect("AGENTS.md"),
      "# The coordinator\n\nIntent v3\n"
    );
    assert_eq!(
      std::fs::read_to_string(fx.target.join(".claude/settings.json")).expect("settings"),
      "{\"hooks\": {}}\n"
    );
    let again = init(&fx.target, &fx.home, "3.2.7").expect("init again");
    assert!(again.iter().all(|r| !r.wrote), "{again:?}");
  }

  /// ST0086 AT-04.5
  ///
  /// vc's ruling: a file there that differs refuses the whole run and nothing
  /// is written, and the coordinator is refused inside an Intent project.
  #[test]
  fn init_refuses_over_a_differing_file_and_inside_a_project() {
    let fx = fixture();
    std::fs::create_dir_all(&fx.target).expect("mkdir target");
    std::fs::write(fx.target.join("CLAUDE.md"), "my own\n").expect("plant CLAUDE.md");
    match init(&fx.target, &fx.home, "3.2.7") {
      Err(InitError::Differs { paths }) => {
        assert_eq!(paths, vec![fx.target.join("CLAUDE.md")]);
      }
      other => panic!("a differing file refuses the run: {other:?}"),
    }
    assert!(
      !fx.target.join("AGENTS.md").exists(),
      "nothing was written beside it"
    );

    let project = fx.target.with_file_name("project");
    std::fs::create_dir_all(project.join("intent/.config")).expect("mkdir config");
    std::fs::write(
      project.join("intent/.config/config.json"),
      "{\"intent_version\": \"3.2.7\"}\n",
    )
    .expect("write a project config");
    assert!(matches!(
      init(&project.join("desk"), &fx.home, "3.2.7"),
      Err(InitError::InProject { .. })
    ));
    assert!(!project.join("desk").exists(), "nothing was made");
  }
}
