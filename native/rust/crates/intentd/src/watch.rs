//! `AC-08.5`: debounced, gitignore-aware watching that drives ingest on
//! external edits.
//!
//! **THE WATCHER IS A TRIGGER, NOT A SYNC ENGINE, AND KEEPING THAT LINE IS THE
//! WHOLE DESIGN.** It answers one question -- *did anything a sync would read
//! just change?* -- and hands the answer to the project's store thread. It does
//! not decide what changed, does not read files, and has no opinion about
//! canon. Everything downstream of the trigger is `Facade::sync_from_disk`,
//! which is the same call `intent sync --to-store` makes.
//!
//! **SCOPE COMES FROM `intentsvcs::sync::Scanned` AND IS NEVER RE-DERIVED
//! HERE.** That predicate and `sync::scan` are the same object, so the watcher
//! and the sync engine cannot disagree about which paths matter. **A second
//! statement of scope inside the daemon would drift in the direction that
//! loops**, which is the next paragraph.
//!
//! **THE FEEDBACK LOOP IS THE REASON `AC-08.5` SAYS `gitignore-aware`, AND IT
//! IS NOT A TIDINESS ARGUMENT.** The store lives at `intent/.cache/intent.db`
//! -- INSIDE the tree this watches -- and every ingest writes it. A watcher
//! that triggered on any change under `intent/` would trigger on the write its
//! own ingest just made, and would do so forever, on an idle machine, in a
//! process nobody is looking at. `.cache` is in `sync::SKIPPED_DIRS` and the
//! whole directory is gitignored, so honouring scope is what stops it.
//!
//! **AND THE DEBOUNCE IS NOT A PERFORMANCE TWEAK EITHER.** One editor save is a
//! burst -- a write, an attribute change, sometimes a chmod -- and an ingest is
//! one whole transaction over the project's scanned corpus. Raw events would
//! run a full ingest three or four times per saved file, and hundreds of times
//! during a `git checkout` while the tree is still moving underneath it.
//! design.md:83 requires the debouncer for this reason: *never raw notify
//! events*.

use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use notify_debouncer_full::notify::RecursiveMode;
use notify_debouncer_full::{DebounceEventResult, Debouncer, RecommendedCache, new_debouncer};

use intentsvcs::wire::Response;

use crate::store::ProjectHandle;

/// How long the tree must be quiet before an ingest runs.
///
/// **LONG ENOUGH THAT ONE SAVE IS ONE INGEST, SHORT ENOUGH THAT A PERSON DOES
/// NOT NOTICE.** The lower bound is the burst an atomic-save editor produces;
/// the upper bound is that `AC-08.6`'s subscribers are waiting on the other
/// side of this. A quarter of a second clears the first with room and is below
/// the threshold where a UI feels stale.
const QUIET: Duration = Duration::from_millis(250);

/// A running watch on one project, stopped when this value is dropped.
///
/// **THE `Debouncer` IS HELD RATHER THAN LEAKED, SO A PROJECT'S WATCH DIES WITH
/// ITS REGISTRATION.** Dropping it stops the watcher thread; leaking it would
/// leave a thread holding a descriptor on a directory the daemon has stopped
/// serving, which on a long-lived process is a slow descriptor leak whose only
/// symptom is the daemon eventually failing to watch anything new.
pub struct Watch {
  _debouncer: Debouncer<notify_debouncer_full::notify::RecommendedWatcher, RecommendedCache>,
}

/// Start watching a project, driving ingest through its store handle.
///
/// **THE TWO WATCH REGISTRATIONS ARE THE SCOPE, EXPRESSED AS REGISTRATIONS.**
/// `intent/` recursively and the root at depth one is exactly what
/// `sync::scan` reads, so nothing outside the corpus is ever watched in the
/// first place. **That is the same narrowing `Ignored::for_root` made and for
/// the same measured reason**: watching the root recursively would descend into
/// a cargo build directory whose size depends on who has compiled what, which
/// is 601,783 paths on this repository against the ~1,500 the scan can ask
/// about.
///
/// **A PROJECT WITH NO `intent/` DIRECTORY IS NOT AN ERROR.** It is a project
/// that has not been initialised yet, and `notify` refuses a path that does not
/// exist -- so the root watch alone is registered and the daemon keeps serving.
/// **THE FAILURE TYPE IS `Response`, NOT `String`, AND NOT A BESPOKE ENUM
/// EITHER** (`IN-RS-CODE-004`, caught by the pre-commit critic on the first
/// version of this function). The rule forbids `String` as an error type and
/// offers `thiserror` or `anyhow`; both would be a new dependency on `intentd`,
/// which under `AC-08.10` means a new written rationale for two variants.
///
/// `Response::error(message, remedy)` is already this crate's refusal shape --
/// `ProjectHandle::open` returns it for the same reason -- and it carries
/// exactly the pair a report needs: what went wrong, and what to do. **Reusing
/// it adds nothing and makes the two fallible constructors in this crate agree
/// about what a failure looks like**, which is the thing a bespoke enum here
/// would quietly cost.
pub fn start(root: &Path, handle: Arc<ProjectHandle>) -> Result<Watch, Response> {
  let watched_root = root.to_path_buf();
  let mut debouncer = new_debouncer(QUIET, None, move |result: DebounceEventResult| {
    on_batch(&watched_root, &handle, result)
  })
  .map_err(|e| {
    Response::error(
      format!("the filesystem watcher for `{}` could not start: {e}", root.display()),
      "external edits to this project will not reach the store on their own. Run `intent sync --to-store` there when you need it caught up, and restart the daemon to retry the watch.",
    )
  })?;

  // The root at depth one, for `sync::ROOT_FILES`.
  debouncer
    .watch(root, RecursiveMode::NonRecursive)
    .map_err(|e| {
      Response::error(
        format!("`{}` could not be watched: {e}", root.display()),
        "the project root is not readable, or this process has run out of the descriptors the platform watcher needs. External edits will not be ingested until the daemon is restarted.",
      )
    })?;

  let intent_dir = root.join("intent");
  if intent_dir.is_dir() {
    debouncer
      .watch(&intent_dir, RecursiveMode::Recursive)
      .map_err(|e| {
        Response::error(
          format!("`{}` could not be watched: {e}", intent_dir.display()),
          "the project's `intent/` directory is not readable, or this process has run out of the descriptors the platform watcher needs. Canon edits will not be ingested until the daemon is restarted.",
        )
      })?;
  }

  Ok(Watch {
    _debouncer: debouncer,
  })
}

/// Decide whether one debounced batch is worth an ingest.
///
/// **THE SCOPE OBJECT IS BUILT ONCE PER BATCH, NOT ONCE PER PATH.**
/// `Scanned::for_root` runs two directory walks; asking it per event would
/// make a `git checkout` quadratic in a directory tree.
///
/// **AND IT IS BUILT FRESH EACH TIME, DELIBERATELY.** Caching it would hold a
/// stale answer across the one edit most likely to change it -- somebody
/// editing `.gitignore`. That file is itself in scope, so the batch carrying
/// its change is the batch whose ignore rules have just moved.
fn on_batch(root: &Path, handle: &Arc<ProjectHandle>, result: DebounceEventResult) {
  let events = match result {
    Ok(events) => events,
    // **REPORTED, NEVER SWALLOWED** (`IN-AG-NO-SILENT-001`). A watcher that
    // stopped receiving events and said nothing is indistinguishable from a
    // project where nobody is editing -- the daemon would look healthy and
    // ingest nothing, forever.
    Err(errors) => {
      for error in errors {
        eprintln!(
          "intentd: watching `{}` failed: {error}\n  remedy: external edits are no longer being ingested for this project. Restart the daemon, or run `intent sync --to-store` when you need the store caught up.",
          root.display()
        );
      }
      return;
    }
  };

  let scope = intentsvcs::sync::Scanned::for_root(root);
  let touched = events
    .iter()
    .flat_map(|event| event.paths.iter())
    .any(|path| scope.includes(path));
  if !touched {
    return;
  }

  // **A DUPLICATE INGEST IS HARMLESS AND A MISSED ONE IS NOT, WHICH IS WHY THIS
  // BLOCKS RATHER THAN SKIPPING WHEN THE QUEUE IS FULL.** The sync is driven by
  // content hashes, so an ingest with nothing to do costs a scan and writes
  // nothing; dropping a trigger, by contrast, leaves the store behind the disk
  // until somebody happens to edit again. This thread is the debouncer's own,
  // so blocking it delays the next batch for THIS project and nothing else.
  match handle.ingest() {
    Ok(()) => {}
    // Rendered, never re-worded: the store said what went wrong and what to do
    // about it, and this module did not diagnose it.
    Err(Response::Error { message, remedy }) => {
      eprintln!("intentd: {message}\n  remedy: {remedy}")
    }
    Err(other) => eprintln!(
      "intentd: the store refused an ingest with {other:?}\n  remedy: this is a fault in intentd rather than in the project. External edits are not reaching the store."
    ),
  }
}
