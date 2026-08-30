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

use intentsvcs::wire::{Event, RegisteredProject, Response};
use tokio::sync::{Mutex, broadcast};

use crate::store::ProjectHandle;
use crate::watch::{self, Watch};

/// How many events a subscriber may fall behind before it is disconnected.
///
/// **BOUNDED, AND A SUBSCRIBER THAT OVERRUNS IT IS DROPPED RATHER THAN QUIETLY
/// SKIPPED** (`IN-AG-NO-SILENT-001`). `broadcast` reports the overrun as
/// `RecvError::Lagged(n)`, and the tempting arm is to log it and carry on --
/// which hands the client a feed with a HOLE in it that looks exactly like a
/// feed without one. **A subscription that ended is recoverable: the client
/// reconnects and re-reads. A subscription that silently skipped is not**,
/// because nothing downstream ever learns which state it is missing.
///
/// The size is a judgement about burst versus liveness: a `git checkout` of a
/// large branch is the realistic burst, and a subscriber that cannot keep up
/// with 256 events is not going to keep up with 4096 either.
const EVENT_BACKLOG: usize = 256;

/// One opened project: the door to its store, and the watch on its tree.
///
/// **THE WATCH IS HELD HERE BECAUSE ITS LIFETIME IS THE REGISTRATION'S.**
/// Dropping a `Watch` stops the watcher thread, so keeping it beside the handle
/// makes *this project is served* and *this project is watched* one fact rather
/// than two that can disagree. The alternative -- a second map keyed on the
/// same roots -- is two homes for one lifetime, and the failure is silent in
/// the worse direction: a project still served and no longer watched looks
/// completely healthy and ingests nothing.
struct Registered {
  handle: Arc<ProjectHandle>,
  /// `None` when the watch could not be started.
  ///
  /// **A PROJECT THAT CANNOT BE WATCHED IS STILL SERVED, DELIBERATELY.**
  /// Refusing to open it would turn a degraded feature into a total outage for
  /// that project -- and watching is an ENHANCEMENT to a store that is correct
  /// without it, since `intent sync --to-store` does the same work on demand.
  /// The failure is printed when it happens rather than swallowed.
  watch: Option<Watch>,
}

/// **HAND-WRITTEN RATHER THAN DERIVED, AND IT REPORTS THE THING WORTH KNOWING.**
/// `Watch` wraps a `Debouncer`, which is not `Debug`, so the derive was never
/// available -- and the useful answer is not the watcher's internals but
/// WHETHER THIS PROJECT IS BEING WATCHED AT ALL, which is precisely the state a
/// failed `watch::start` leaves behind and the one an operator debugging *why
/// did my edit not ingest* needs to see.
impl std::fmt::Debug for Registered {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Registered")
      .field("dispatched", &self.handle.dispatched())
      .field("watched", &self.watch.is_some())
      .finish()
  }
}

/// Every project this daemon has opened, by canonical root.
///
/// **A `tokio::sync::Mutex` RATHER THAN A `std` ONE, BECAUSE IT IS HELD ACROSS
/// AN `await`.** Opening a project is asynchronous -- the store thread starts
/// and reports back -- and holding a blocking mutex across that would park a
/// runtime worker on a lock, which is the family of defect this daemon's whole
/// store arrangement exists to avoid.
#[derive(Debug, Default)]
pub struct Registry {
  projects: Mutex<HashMap<PathBuf, Registered>>,
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
      return Ok(Arc::clone(&existing.handle));
    }
    // **ONE FEED PER PROJECT, CREATED HERE BECAUSE HERE IS WHERE A PROJECT
    // BECOMES ONE.** Both publishers -- the store thread after an ingest, and
    // the watcher on a file change -- need this sender, and so does every
    // subscriber; creating it anywhere else would mean a second channel for one
    // project, on which half the events would arrive.
    let (events, _) = broadcast::channel::<Event>(EVENT_BACKLOG);
    let handle = Arc::new(ProjectHandle::open(canonical.clone(), events).await?);

    // **THE WATCH STARTS WHEN THE PROJECT OPENS, WHICH IS THE SAME ANSWER THIS
    // REGISTRY ALREADY GIVES TO EVERY OTHER LIFECYCLE QUESTION.** There is no
    // `register` verb because registration is a side effect of being used; a
    // separate `watch` verb would reintroduce exactly the two-states-and-a-way-
    // to-be-in-the-wrong-one this type was shaped to avoid. A project the
    // daemon is answering for is a project the daemon is watching.
    let watching = match watch::start(&canonical, Arc::clone(&handle)) {
      Ok(watching) => Some(watching),
      // Reported, never swallowed: the project is served and its external
      // edits will not be ingested, and only this line says so.
      // **THE REFUSAL IS RENDERED, NOT RE-WORDED.** `watch::start` returns a
      // `Response` carrying its own message and remedy, so printing them is the
      // whole job -- and a second sentence composed here would be a second
      // opinion about a failure this module did not diagnose.
      Err(Response::Error { message, remedy }) => {
        eprintln!(
          "intentd: `{}` is being SERVED and NOT WATCHED: {message}\n  remedy: {remedy}",
          canonical.display()
        );
        None
      }
      // `watch::start` only ever refuses. An `Ok`-shaped response here would be
      // a routing fault inside this crate, and it says so rather than being
      // silently treated as success.
      Err(other) => {
        eprintln!(
          "intentd: the watcher for `{}` answered a refusal with {other:?}\n  remedy: this is a fault in intentd rather than in the project. The project is served and not watched.",
          canonical.display()
        );
        None
      }
    };

    projects.insert(
      canonical,
      Registered {
        handle: Arc::clone(&handle),
        watch: watching,
      },
    );
    Ok(handle)
  }

  /// Every registered project, and whether its root is still there.
  pub async fn snapshot(&self) -> Response {
    let projects = self.projects.lock().await;
    let mut listed: Vec<RegisteredProject> = projects
      .iter()
      .map(|(root, registered)| RegisteredProject {
        root: root.clone(),
        dispatched: registered.handle.dispatched(),
        ingested: registered.handle.ingested(),
        // Read from the watch's PRESENCE rather than from a second flag set
        // when it started: one fact, one home, and they cannot disagree about
        // a project that was served after `watch::start` refused.
        watched: registered.watch.is_some(),
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

  /// A live feed for a project, opening it if this is first contact.
  ///
  /// **IT GOES THROUGH `handle_for` RATHER THAN AROUND IT**, so a subscription
  /// registers and watches a project exactly as any other first contact does.
  /// A subscribe path that skipped registration would give a client a feed on a
  /// project nothing was watching -- a subscription that is correct, connected,
  /// and permanently silent.
  pub async fn feed_for(
    &self,
    root: &Path,
  ) -> Result<(String, broadcast::Receiver<Event>), Response> {
    let handle = self.handle_for(root).await?;
    Ok((handle.project_id().to_string(), handle.subscribe()))
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
