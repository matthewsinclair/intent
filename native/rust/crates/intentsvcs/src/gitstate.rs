//! What git says about this clone, as `intent sync` needs it (ST0078 WP-05).
//!
//! The plan `intent sync` prints is a pure function of the store, the tree and
//! git's status. This module is the git half of the reading: whether the branch
//! is behind its upstream, which index entries are unmerged and at which
//! stages, and the bytes a stage or a commit holds at a path.
//!
//! **ONE WRITE, AND IT IS STAGING.** [`stage`] runs `git add` over exactly the
//! paths a caller names, and the caller names only the files it regenerated to
//! resolve a conflict it was asked to resolve (AC-05.4). Without it a person is
//! back to one `git add` per file after every repaired merge.
//!
//! **IT NEVER PULLS, COMMITS OR PUSHES, AND THERE IS NO FUNCTION HERE THAT
//! COULD.** The boundary is this module's surface rather than a promise at each
//! call site: a caller that wanted to pull would have to write the call itself,
//! in a review that would see it.
//!
//! **EVERY PATH IS RELATIVE TO THE PROJECT ROOT**, which is where git runs, so
//! a project that is a subdirectory of its repository reads the same way.
//!
//! **NOT A REPOSITORY, OR NO UPSTREAM, IS AN ANSWER, NEVER AN ERROR.** A project
//! need not live in git, and a branch need not track one. A git that ran and
//! failed is a typed error with a remedy, because a plan built on an index it
//! could not read would say nothing was unmerged exactly when it could not look.

use std::path::Path;
use std::process::Command;

/// A git read or write that ran and failed.
#[derive(Debug, thiserror::Error)]
pub enum GitStateError {
  /// `git` could not be started at all.
  #[error("running `git {args}`: {source}")]
  Spawn {
    args: String,
    #[source]
    source: std::io::Error,
  },
  /// `git` ran and refused.
  #[error("`git {args}` exited {code}: {stderr}")]
  Refused {
    args: String,
    code: i32,
    stderr: String,
  },
  /// An index line that does not have the shape `git ls-files -u` prints.
  #[error("reading the unmerged index: a line did not parse: {line}")]
  Unparsed { line: String },
}

impl crate::remedy::Remedy for GitStateError {
  fn remedy(&self) -> String {
    match self {
      Self::Spawn { .. } => {
        "check that `git` is on PATH -- `intent sync` reads git's status to plan, and plans nothing it could not read".to_string()
      }
      Self::Refused { args, .. } => format!(
        "run `git {args}` yourself to read git's answer, repair what it names, then run `intent sync` again"
      ),
      Self::Unparsed { .. } => {
        "run `git ls-files -u` to see the unmerged entries git reports, and `git status` for the merge in progress".to_string()
      }
    }
  }
}

/// How far the branch is behind the upstream it tracks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Behind {
  /// The upstream, as git names it (`origin/main`).
  pub upstream: String,
  /// Commits the upstream holds that `HEAD` does not, as of the last fetch.
  pub commits: usize,
}

/// One unmerged path in the index, with the stages git holds for it.
///
/// Stage 1 is the common ancestor, 2 is this clone's side (`HEAD`) and 3 is the
/// side being merged in. An add/add has 2 and 3 and no 1; a content conflict
/// has all three; a modify/delete has 1 and one of the others.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unmerged {
  /// Relative to the project root, as git spells it from there.
  pub path: String,
  pub base: bool,
  pub ours: bool,
  pub theirs: bool,
}

impl Unmerged {
  /// Both sides added a file at this path and there was nothing before.
  pub fn is_add_add(&self) -> bool {
    self.ours && self.theirs && !self.base
  }
}

/// Is `root` inside a git work tree? No git on PATH reads as no.
pub fn is_work_tree(root: &Path) -> bool {
  Command::new("git")
    .args(["rev-parse", "--is-inside-work-tree"])
    .current_dir(root)
    .output()
    .map(|out| out.status.success() && out.stdout.starts_with(b"true"))
    .unwrap_or(false)
}

/// How far `HEAD` is behind its upstream, or `None` when the branch tracks
/// none (a detached `HEAD` included). **It reads what the last fetch left and
/// fetches nothing**, so it answers for the upstream this clone has seen.
pub fn behind(root: &Path) -> Result<Option<Behind>, GitStateError> {
  let upstream = Command::new("git")
    .args(["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{u}"])
    .current_dir(root)
    .output()
    .map_err(|source| GitStateError::Spawn {
      args: "rev-parse --abbrev-ref --symbolic-full-name @{u}".to_string(),
      source,
    })?;
  // No upstream is git refusing `@{u}`, which is an answer rather than a fault.
  if !upstream.status.success() {
    return Ok(None);
  }
  let upstream = String::from_utf8_lossy(&upstream.stdout).trim().to_string();
  let count = run(root, &["rev-list", "--count", "HEAD..@{u}"])?;
  let commits = String::from_utf8_lossy(&count)
    .trim()
    .parse::<usize>()
    .map_err(|_| GitStateError::Unparsed {
      line: String::from_utf8_lossy(&count).trim().to_string(),
    })?;
  Ok(Some(Behind { upstream, commits }))
}

/// Every unmerged path in the index, in path order.
pub fn unmerged(root: &Path) -> Result<Vec<Unmerged>, GitStateError> {
  let out = run(root, &["ls-files", "-u", "-z"])?;
  let mut found: Vec<Unmerged> = Vec::new();
  for record in out.split(|b| *b == 0).filter(|r| !r.is_empty()) {
    let line = String::from_utf8_lossy(record).to_string();
    // `<mode> <object> <stage>\t<path>`
    let (meta, path) = line
      .split_once('\t')
      .ok_or_else(|| GitStateError::Unparsed { line: line.clone() })?;
    let stage = meta
      .rsplit(' ')
      .next()
      .and_then(|s| s.parse::<u8>().ok())
      .ok_or_else(|| GitStateError::Unparsed { line: line.clone() })?;
    let entry = match found.iter_mut().find(|u| u.path == path) {
      Some(entry) => entry,
      None => {
        found.push(Unmerged {
          path: path.to_string(),
          base: false,
          ours: false,
          theirs: false,
        });
        found
          .last_mut()
          .ok_or_else(|| GitStateError::Unparsed { line: line.clone() })?
      }
    };
    match stage {
      1 => entry.base = true,
      2 => entry.ours = true,
      3 => entry.theirs = true,
      _ => return Err(GitStateError::Unparsed { line }),
    }
  }
  found.sort_by(|a, b| a.path.cmp(&b.path));
  Ok(found)
}

/// Is a merge in progress, so that `MERGE_HEAD` names the side coming in?
pub fn merging(root: &Path) -> bool {
  Command::new("git")
    .args(["rev-parse", "-q", "--verify", "MERGE_HEAD"])
    .current_dir(root)
    .output()
    .is_ok_and(|out| out.status.success())
}

/// The paths a commit holds under `prefix`, relative to the project root.
///
/// `rev` is `HEAD` for this clone's side of a merge and `MERGE_HEAD` for the
/// side coming in.
pub fn files_under(root: &Path, rev: &str, prefix: &str) -> Result<Vec<String>, GitStateError> {
  let out = run(
    root,
    &["ls-tree", "-r", "-z", "--name-only", rev, "--", prefix],
  )?;
  Ok(
    out
      .split(|b| *b == 0)
      .filter(|r| !r.is_empty())
      .map(|r| String::from_utf8_lossy(r).to_string())
      .collect(),
  )
}

/// Is `path` in the index? Outside a work tree nothing is.
pub fn is_tracked(root: &Path, path: &str) -> Result<bool, GitStateError> {
  if !is_work_tree(root) {
    return Ok(false);
  }
  run(root, &["ls-files", "-z", "--", path]).map(|out| !out.is_empty())
}

/// The bytes a commit holds at `path`, or `None` when it holds nothing there.
pub fn blob(root: &Path, rev: &str, path: &str) -> Result<Option<Vec<u8>>, GitStateError> {
  // `./` makes the path relative to the project root rather than to the
  // repository's top level, which is how every other path here is spelled.
  let spec = format!("{rev}:./{path}");
  let exists = Command::new("git")
    .args(["cat-file", "-e", &spec])
    .current_dir(root)
    .stderr(std::process::Stdio::null())
    .status()
    .map_err(|source| GitStateError::Spawn {
      args: format!("cat-file -e {spec}"),
      source,
    })?;
  if !exists.success() {
    return Ok(None);
  }
  run(root, &["cat-file", "blob", &spec]).map(Some)
}

/// Stage exactly `paths`, removals included (`git add -A -- <paths>`).
///
/// **THE ONLY GIT WRITE `intent sync` MAKES.** The caller names the files it
/// regenerated to resolve a conflict it was asked to resolve, and nothing else.
pub fn stage(root: &Path, paths: &[String]) -> Result<(), GitStateError> {
  if paths.is_empty() {
    return Ok(());
  }
  let mut args = vec!["add", "-A", "--"];
  args.extend(paths.iter().map(String::as_str));
  run(root, &args).map(|_| ())
}

fn run(root: &Path, args: &[&str]) -> Result<Vec<u8>, GitStateError> {
  let out = Command::new("git")
    .args(args)
    .current_dir(root)
    .output()
    .map_err(|source| GitStateError::Spawn {
      args: args.join(" "),
      source,
    })?;
  if !out.status.success() {
    return Err(GitStateError::Refused {
      args: args.join(" "),
      code: out.status.code().unwrap_or(-1),
      stderr: String::from_utf8_lossy(&out.stderr).trim().to_string(),
    });
  }
  Ok(out.stdout)
}
