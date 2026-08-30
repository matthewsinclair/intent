//! Which projects this daemon serves, and whether their roots are still there.
//!
//! **`AC-08.1`: N PROJECTS, REGISTERED ON FIRST CONTACT, WITH A MOVED OR
//! DELETED ROOT SURFACING RATHER THAN CRASHING.**
//!
//! **ROOTS ARE CANONICALISED BEFORE THEY BECOME KEYS, AND THAT IS A CORRECTNESS
//! REQUIREMENT RATHER THAN TIDINESS.** Two clients naming one project by
//! different paths -- a symlink, a relative path, `/var` versus `/private/var`
//! on macOS -- must reach ONE store. Keying on the string a client happened to
//! send would open a second `Facade` on the same database under a different
//! name, which is two engines on one store wearing a disguise: exactly what the
//! routing rule forbids, arrived at from inside the daemon that was supposed to
//! be preventing it.
//!
//! **A ROOT THAT HAS GONE IS A STATE, NOT A FAULT, AND NOT AN OMISSION EITHER.**
//! The obvious two failures are both wrong. Panicking loses every other project
//! the daemon serves for a directory somebody moved. Quietly dropping it from
//! the listing satisfies the criterion's words while hiding the thing they were
//! written for -- the operator's question is *why is this project not working*,
//! and an entry missing from a list cannot answer it. So a registered project
//! is always listed, with whether its root still exists.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use intentsvcs::wire::{RegisteredProject, Response};
use tokio::sync::Mutex;

use crate::store::ProjectHandle;

/// Every project this daemon has opened, by canonical root.
///
/// **A `tokio::sync::Mutex` RATHER THAN A `std` ONE, BECAUSE IT IS HELD ACROSS
/// AN `await`.** Opening a project is asynchronous -- the store thread starts
/// and reports back -- and holding a blocking mutex across that would park a
/// runtime worker on a lock, which is the family of defect this daemon's whole
/// store arrangement exists to avoid.
#[derive(Debug, Default)]
pub struct Registry {
  projects: Mutex<HashMap<PathBuf, Arc<ProjectHandle>>>,
}

impl Registry {
  pub fn new() -> Registry {
    Registry::default()
  }

  /// The handle for a root, opening the project the first time it is asked for.
  ///
  /// **REGISTRATION IS A SIDE EFFECT OF BEING USED, WHICH IS WHY THERE IS NO
  /// `register` VERB.** A daemon with a registration step has two states a
  /// project can be in and a way to be in the wrong one; a daemon that opens on
  /// first contact has one. The cost is that an unopenable project is
  /// discovered at request time, which is also when somebody is there to be
  /// told.
  pub async fn handle_for(&self, root: &Path) -> Result<Arc<ProjectHandle>, Response> {
    let canonical = canonicalise(root)?;

    // **THE LOCK IS HELD ACROSS THE OPEN, DELIBERATELY.** Releasing it to open
    // and re-taking it to insert would let two concurrent first contacts for
    // one project each start a store thread, and the loser's `Facade` would sit
    // on the same database as the winner's until it was dropped. Serialising
    // first contact costs one project's callers a wait and removes the race
    // entirely.
    let mut projects = self.projects.lock().await;
    if let Some(existing) = projects.get(&canonical) {
      return Ok(Arc::clone(existing));
    }
    let handle = Arc::new(ProjectHandle::open(canonical.clone()).await?);
    projects.insert(canonical, Arc::clone(&handle));
    Ok(handle)
  }

  /// Every registered project, and whether its root is still there.
  pub async fn snapshot(&self) -> Response {
    let projects = self.projects.lock().await;
    let mut listed: Vec<RegisteredProject> = projects
      .iter()
      .map(|(root, handle)| RegisteredProject {
        root: root.clone(),
        dispatched: handle.dispatched(),
        // Asked at REPORT time rather than remembered from registration: the
        // whole point of the field is to notice a change that happened after
        // the daemon last looked.
        root_exists: root.exists(),
      })
      .collect();
    // A stable order, so an operator comparing two runs is reading a real
    // difference rather than a hash iteration order.
    listed.sort_by(|a, b| a.root.cmp(&b.root));
    Response::Registry { projects: listed }
  }

  /// The canonical form of a root, WITHOUT opening or registering anything.
  ///
  /// **THE SEPARATION EXISTS SO A CONNECTION CAN CHECK ITS BINDING BEFORE THE
  /// SIDE EFFECT THE BINDING WAS MEANT TO PREVENT.** The first build asked
  /// [`Registry::handle_for`] and compared afterwards -- which OPENED and
  /// REGISTERED the wrong project, started a store thread for it, and only then
  /// refused. The refusal was accurate and everything it was protecting had
  /// already happened. A check that runs after its subject is a report.
  pub fn canonical(&self, root: &Path) -> Result<PathBuf, Response> {
    canonicalise(root)
  }
}

/// The canonical form of a root, or a refusal that says why not.
///
/// **A ROOT THAT CANNOT BE CANONICALISED IS REFUSED RATHER THAN USED AS GIVEN.**
/// Falling back to the raw path is the tempting arm and it is the one that
/// opens a second store: the fallback key differs from the canonical key for
/// the same project, so a client whose path stopped resolving would silently
/// get its own private `Facade` on a database another handle already owns.
fn canonicalise(root: &Path) -> Result<PathBuf, Response> {
  root.canonicalize().map_err(|e| {
    Response::error(
      format!("`{}` could not be resolved: {e}", root.display()),
      "the daemon resolves every project root to its canonical form so that two names for one project cannot open two stores. This path does not resolve -- it has been moved or deleted, or a component of it is not readable.",
    )
  })
}
