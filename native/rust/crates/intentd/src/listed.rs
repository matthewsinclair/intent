//! The project registry, as the daemon reads it (`AC-03.4`).
//!
//! **READ, WATCHED AND NEVER WRITTEN.** The operator, `intent explore` and
//! `intent discover` write `projects.json`; this keeps the daemon's list of
//! known projects in step with the file without a restart. It changes which
//! projects the registry LISTS and never which stores are open: opening stays
//! first contact, so a project somebody lists is not opened until somebody uses
//! it.

use crate::daemon_log::elogln;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use intentsvcs::projects;
use intentsvcs::remedy::Remedy;
use intentsvcs::wire::Response;
use notify_debouncer_full::notify::{RecommendedWatcher, RecursiveMode};
use notify_debouncer_full::{DebounceEventResult, Debouncer, RecommendedCache, new_debouncer};

use crate::registry::Registry;

/// The same quiet period the tree watcher uses, for the same reason: an editor
/// saves in several writes, and one reload per save is the goal.
const QUIET: Duration = Duration::from_millis(250);

/// What every refusal to watch leaves the operator with.
const UNWATCHED: &str = "intentd lists the project registry as it read it at start and will not see later edits to it. Restart the daemon once the directory is readable.";

/// Held for as long as the file is watched; dropping it stops the watch.
pub struct Listed {
  _debouncer: Debouncer<RecommendedWatcher, RecommendedCache>,
}

/// Read the registry at `path` into `registry`, then keep it in step.
///
/// **THE DIRECTORY IS WATCHED, NOT THE FILE.** Every writer replaces the file
/// by renaming a new one over it, and a watch on the first file's inode sees
/// nothing after the first write.
///
/// **A REFUSAL IS A `Response`, AS `watch::start`'s IS**, so the two fallible
/// watch constructors in this crate report a failure the same way: what went
/// wrong, and what to do.
pub fn start(path: PathBuf, registry: Arc<Registry>) -> Result<Listed, Response> {
  reload(&path, &registry);
  let Some(dir) = path.parent().map(Path::to_path_buf) else {
    return Err(Response::error(
      format!("`{}` has no directory to watch", path.display()),
      UNWATCHED,
    ));
  };
  std::fs::create_dir_all(&dir).map_err(|e| {
    Response::error(
      format!("`{}` could not be created to watch it: {e}", dir.display()),
      UNWATCHED,
    )
  })?;
  let watched = path.clone();
  let mut debouncer = new_debouncer(
    QUIET,
    None,
    move |result: DebounceEventResult| match result {
      Ok(_) => reload(&watched, &registry),
      Err(errors) => {
        for e in errors {
          elogln!(
            "warning: intentd could not watch `{}`: {e}\n  remedy: {UNWATCHED}",
            watched.display()
          );
        }
      }
    },
  )
  .map_err(|e| {
    Response::error(
      format!("the watcher for `{}` could not start: {e}", dir.display()),
      UNWATCHED,
    )
  })?;
  debouncer
    .watch(&dir, RecursiveMode::NonRecursive)
    .map_err(|e| {
      Response::error(
        format!("`{}` could not be watched: {e}", dir.display()),
        UNWATCHED,
      )
    })?;
  Ok(Listed {
    _debouncer: debouncer,
  })
}

/// **A FILE THAT DOES NOT PARSE KEEPS THE LAST GOOD LIST, AND SAYS SO.** A
/// half-typed edit in an operator's editor must not empty the list every
/// client reads. A missing file is an empty list, because that is what it says.
fn reload(path: &Path, registry: &Registry) {
  match projects::load(path) {
    Ok(listed) => registry.set_listed(listed.roots()),
    Err(e) => elogln!(
      "warning: intentd: kept the last good project list\n{}",
      e.render()
    ),
  }
}
