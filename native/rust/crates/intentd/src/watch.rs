//! `AC-08.5`: debounced, gitignore-aware watching that drives ingest on
//! external edits.
//!
//! **THE WATCHER IS A TRIGGER, NOT A SYNC ENGINE, AND KEEPING THAT LINE IS THE
//! WHOLE DESIGN.** It answers one question -- *did anything a sync would read
//! just change?* -- and hands the answer to the project's store thread. It does
//! not decide what changed, does not read files, and has no opinion about
//! canon. Everything downstream of the trigger is `Facade::ingest_from_disk`,
//! the engine `intent sync --to-store` runs, in its non-destructive mode
//! (issue `0216`).
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
use notify_debouncer_full::{DebounceEventResult, Debouncer, NoCache, new_debouncer_opt};

use intentsvcs::wire::{Event, Response};

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
  _debouncer: Debouncer<notify_debouncer_full::notify::RecommendedWatcher, NoCache>,
  /// The INDEX scope's own registration and its own debounced stream.
  ///
  /// **TWO REGISTRATIONS, NOT ONE WIDENED ONE** (vc, 2026-09-12, on a
  /// measurement). The canon watch above keeps exactly the registration its
  /// tests pin; this one covers the gitignore-aware repository and its events
  /// reach only the index. Widening the canon registration instead was measured
  /// at three of four full-suite runs red against none of four, and the second
  /// stream's cost buys the property WP-18 promised: the canon path is
  /// untouched by the index.
  _index_debouncer: Debouncer<notify_debouncer_full::notify::RecommendedWatcher, NoCache>,
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
  let handle_for_index = Arc::clone(&handle);
  let watched_root = root.to_path_buf();
  // **NO FILE-ID CACHE ON THE CANON REGISTRATION EITHER** (vc's ruling on issue
  // 0377, 2026-09-15). The default cache walked what every Create named, a stat
  // per entry and before any ignore rule applied, and a commit's gate and
  // ingest produce exactly that burst: measured at about eight seconds of
  // system time on this loop per commit on the pair of 2026-09-14. What the
  // cache bought is a rename stitched into one event; without it a rename
  // arrives as a removal and a creation, which `on_batch` reconciles as a
  // vanished path and a leaf. The scope is untouched: the same two
  // registrations below.
  let mut debouncer = new_debouncer_opt::<_, notify_debouncer_full::notify::RecommendedWatcher, NoCache>(
    QUIET,
    None,
    move |result: DebounceEventResult| on_batch(&watched_root, &handle, result),
    NoCache::new(),
    notify_debouncer_full::notify::Config::default(),
  )
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

  // **THE INDEX SCOPE'S OWN REGISTRATION.** Recursive from the root, because
  // the index's scope IS the gitignore-aware repository; the events are
  // filtered by `Scanned::in_repository` at the leaf and by the root bound
  // above it, and they reach `Work::IndexRefresh` and nothing else.
  //
  // **THE VOLUME IS AFFORDABLE AND IT WAS MEASURED, NOT ASSUMED** (AC-18.6, dc
  // 2026-09-12). FSEvents coalesces, so the volume tracks ACTIVITY rather than
  // tree size: one entire release build in a watched tree produced 703
  // path-events across 26 debounced batches, caching 447 distinct paths, of
  // which 2 were in scope. The fear was a flood proportional to the 601,783
  // paths under a build directory, and that flood does not arrive.
  //
  // **THE LINUX LEG IS NOT MEASURED AND IS NOT CLAIMED.** inotify does not
  // coalesce and needs a watch per directory; this repository holds 496
  // directories git tracks content under, which is evidence the per-directory
  // strategy FITS under `max_user_watches` and no evidence about its rate.
  let index_root = root.to_path_buf();
  let index_handle = Arc::clone(&handle_for_index);
  //
  // **AND IT KEEPS NO FILE-ID CACHE** (issue 0355; the canon registration
  // followed under 0377). The
  // default cache walks each created directory and, on each removal, retains
  // over every path it holds, so a deletion burst under a build directory held
  // half a core in that bookkeeping alone. What the cache buys is a rename
  // stitched into one event; without it a rename arrives as a removal and a
  // creation, which `index_paths_to_refresh` already hands over as a vanished
  // path and a leaf.
  // Issue 0355.
  let mut index_debouncer = new_debouncer_opt::<_, notify_debouncer_full::notify::RecommendedWatcher, NoCache>(
    QUIET,
    None,
    move |result: DebounceEventResult| on_index_batch(&index_root, &index_handle, result),
    NoCache::new(),
    notify_debouncer_full::notify::Config::default(),
  )
  .map_err(|e| {
    Response::error(
      format!("the index watcher for `{}` could not start: {e}", root.display()),
      "source edits will not reach `intent search` on their own. Run `intent index rebuild` there when you need it caught up, and restart the daemon to retry the watch.",
    )
  })?;
  index_debouncer
    .watch(root, RecursiveMode::Recursive)
    .map_err(|e| {
      Response::error(
        format!("`{}` could not be watched for the index: {e}", root.display()),
        "the project root is not readable, or this process has run out of the descriptors the platform watcher needs. Source edits will not reach `intent search` until the daemon is restarted.",
      )
    })?;

  Ok(Watch {
    _debouncer: debouncer,
    _index_debouncer: index_debouncer,
  })
}

/// Which paths an index-scope batch should have reconciled, with the root bound
/// applied.
///
/// **THE ROOT BOUND IS NOT HERE, AND THAT IS DELIBERATE.** An event naming the
/// root names everything and therefore names nothing, so it must reconcile the
/// root's own files without descending -- and that rule lives in
/// `index::reconcile::changed_under` (cc, `cbcd46fad`), which is the one place
/// that enumerates the index corpus. A second copy here would be two statements
/// of one rule, which is the defect this thread has already paid for twice.
///
/// What this decides is only what the DISPATCH hands over: a directory or a
/// vanished path goes as itself only when the index scope reaches it (the root
/// always does), a leaf goes only if the index scope admits it,
/// and the daemon's own store writes are refused before they cost a round trip.
fn index_paths_to_refresh(root: &Path, paths: &[&Path]) -> Vec<std::path::PathBuf> {
  let scope = intentsvcs::sync::Scanned::for_root(root);
  let mut out: Vec<std::path::PathBuf> = Vec::new();
  for path in paths {
    // **A DIRECTORY OR A VANISHED PATH GOES ONLY WHEN THE SCOPE REACHES IT.**
    // Handed over regardless, a deletion burst under an ignored build directory
    // cost the store thread a whole-repository walk per debounced batch.
    // Issue 0355.
    let admitted = if path.is_dir() || !path.exists() {
      *path == root || scope.reaches(path)
    } else {
      scope.in_repository(path)
    };
    if admitted {
      out.push(path.to_path_buf());
    }
  }
  out.sort();
  out.dedup();
  out
}

/// One debounced batch from the INDEX registration.
///
/// **IT CANNOT START A CANON INGEST, WHICH IS THE WHOLE RULING.** The only door
/// it reaches is [`ProjectHandle::index_refresh`], and the index's tables have
/// their own writers, so nothing on this stream can rewrite a view and produce
/// the next event.
fn on_index_batch(root: &Path, handle: &Arc<ProjectHandle>, result: DebounceEventResult) {
  let events = match result {
    Ok(events) => events,
    // **REPORTED, NEVER SWALLOWED** (`IN-AG-NO-SILENT-001`). An index watcher
    // that stopped receiving events and said nothing is indistinguishable from
    // a repository nobody is editing.
    Err(errors) => {
      for error in errors {
        eprintln!(
          "intentd: the index watcher for `{}` reported an error: {error}\n  remedy: source edits may not be reaching `intent search`. Run `intent index rebuild` there, and restart the daemon to re-establish the watch.",
          root.display()
        );
      }
      return;
    }
  };

  let paths: Vec<&Path> = events
    .iter()
    .flat_map(|event| event.paths.iter())
    .map(|p| p.as_path())
    .collect();
  // **THE WHOLE BATCH IS ONE REFRESH.** The facade pays for what the store
  // carries, the stored rows and the walk once per call, so a batch handed over
  // a path at a time paid them once per path.
  // Issue 0354.
  let under = index_paths_to_refresh(root, &paths);
  if under.is_empty() {
    return;
  }
  match handle.index_refresh(under) {
    Ok(()) => {}
    // Rendered, never re-worded, exactly as the canon stream renders its own:
    // the store said what went wrong and what to do about it, and this module
    // did not diagnose it.
    Err(Response::Error { message, remedy }) => {
      eprintln!("intentd: {message}\n  remedy: {remedy}");
    }
    Err(other) => {
      eprintln!(
        "intentd: the store refused an index refresh with {other:?}\n  remedy: this is a fault in intentd rather than in the project. Source edits are not reaching `intent search`."
      );
    }
  }
}

/// Which FILES a batch's paths say changed.
///
/// **SEPARATED FROM [`on_batch`] SO IT CAN BE DRIVEN WITH A PLANTED
/// DIRECTORY-GRANULARITY EVENT.** The defect this exists to close is reachable
/// only when the OS coalesces, which is a property of load rather than of the
/// code, so a test that waits for macOS to coalesce is a test that passes for
/// the wrong reason most of the time. Given the paths, this function is
/// decidable against a real tree with no daemon, no runtime and no waiting
/// (`IN-AG-PFIC-001`: the decision is here, the publishing and the ingest are
/// the caller's).
///
/// `index` is a closure rather than a value because fetching it is a round trip
/// to the store thread, and a batch of leaf events -- the ordinary case -- must
/// not pay for it.
fn files_that_changed(
  root: &Path,
  paths: &[&Path],
  index: &mut dyn FnMut() -> Vec<intentsvcs::sync::FileEntry>,
) -> Vec<std::path::PathBuf> {
  let scope = intentsvcs::sync::Scanned::for_root(root);
  let mut recorded: Option<Vec<intentsvcs::sync::FileEntry>> = None;
  let mut changed: Vec<std::path::PathBuf> = Vec::new();
  for path in paths {
    if path.is_dir() || !path.exists() {
      let previous = recorded.get_or_insert_with(&mut *index);
      match intentsvcs::sync::changed_under(root, path, previous) {
        Ok(files) => changed.extend(files),
        // **REPORTED, NEVER SWALLOWED** (`IN-AG-NO-SILENT-001`). A subtree this
        // cannot read is a subtree whose edits stop reaching the store, and
        // silence there is indistinguishable from nobody editing.
        Err(error) => eprintln!(
          "intentd: could not reconcile `{}` after a directory-level change: {error}\n  remedy: external edits under that path may not be reaching the store. Run `intent sync --to-store` to catch it up.",
          path.display()
        ),
      }
    } else if scope.includes(path) {
      // **A LEAF IS RECONCILED AGAINST THE STORE TOO, AND UNTIL ISSUE `0311` IT
      // WAS NOT** -- which made the invariant three paragraphs above true on one
      // of its two branches and false on the other. A directory event
      // reconciled its subtree and published only files whose bytes differ; a
      // leaf event was published on scope alone, unread.
      //
      // **THE CONSEQUENCE WAS THE FEEDBACK LOOP THIS MODULE SAYS SCOPE
      // PREVENTS, REACHED THROUGH THE OTHER DOOR.** An ingest writes canon and
      // the generated views INSIDE the watched scope; the watcher saw those
      // leaves, published them, and ingested again. Traced on a socket client:
      // one hand-written canon file produced `ST0079.json`, the barrier file,
      // `steel_threads.md`, `todo.md`, `project_changed` -- and then a SECOND
      // round of the same. The daemon was telling subscribers about its own
      // writes and waking itself up to do it again.
      //
      // Asking the same question the other door asks closes it by construction:
      // the store recorded those bytes when it wrote them, so the echo
      // reconciles to nothing, `changed` is empty, and the caller returns before
      // it publishes OR ingests. **A real external edit is unaffected** -- its
      // bytes are not what the store holds, which is what makes this a
      // reconciliation and not a filter on who wrote it.
      let previous = recorded.get_or_insert_with(&mut *index);
      match intentsvcs::sync::differs_from_recorded(root, path, previous) {
        Ok(true) => changed.push(path.to_path_buf()),
        Ok(false) => {}
        // **REPORTED, NEVER SWALLOWED, AND PUBLISHED ANYWAY.** A file this
        // cannot read is a file whose change cannot be judged, and treating
        // unreadable as unchanged would drop a real edit silently -- the one
        // outcome worse than a duplicate event.
        Err(error) => {
          eprintln!(
            "intentd: could not compare `{}` against the store's index: {error}\n  remedy: the event is being published unjudged. If this repeats, that file's edits may be reaching subscribers twice.",
            path.display()
          );
          changed.push(path.to_path_buf());
        }
      }
    }
  }
  changed.sort();
  changed.dedup();
  changed
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

  // **A DIRECTORY-GRANULARITY EVENT IS A QUESTION, NEVER AN ANSWER, AND
  // ANSWERING IT BY ASKING `includes` IS WHAT BROKE THIS** (vc's ruling,
  // 2026-09-12). macOS coalesces a burst into one event naming the PARENT --
  // and a deleted path cannot be classified at all -- so `intent/` itself
  // arrived here as the changed path. `Scanned::includes` returned true for it,
  // the daemon published `fileChanged` naming a DIRECTORY, and then ingested
  // the write its own store had just made: the feedback loop this module's
  // header says scope prevents, reached because scope was asked the wrong
  // question. The header's claim held only while the OS happened to report the
  // leaf.
  //
  // So a path that is a directory, or no longer exists, is RECONCILED: the
  // in-scope files under it, each compared against what the store last
  // recorded. A leaf that names a file is unchanged from before.
  //
  // **THE CONTRACT D20 AND `AC-08.6` PROMISE NOW HOLDS BY CONSTRUCTION:** a
  // `fileChanged` names a file whose bytes differ, or it is not published. And
  // the daemon's own `intent/.cache/` write reconciles to an EMPTY set,
  // because the skip list speaks at the leaf as the walk descends -- so the
  // loop is closed by the shape of the answer rather than by whether the OS
  // reported the leaf.
  let paths: Vec<&Path> = events
    .iter()
    .flat_map(|event| event.paths.iter())
    .map(|p| p.as_path())
    .collect();
  let changed = files_that_changed(root, &paths, &mut || handle.file_index());
  if changed.is_empty() {
    return;
  }

  // **`fileChanged` IS EMITTED HERE BECAUSE HERE IS THE ONLY PLACE THAT KNOWS
  // WHICH FILES** (`D20`, `AC-08.6`). It says the DISK moved; the store thread
  // emits `projectChanged` after the re-read, which says the MODEL moved. They
  // are published in that order for the same reason they are two events: a
  // subscriber mirroring files acts on the first, and one redrawing from the
  // store must not act until the second.
  //
  // **A SEND WITH NO SUBSCRIBERS IS AN `Err` AND IS NOT A FAILURE.** It is the
  // ordinary case for a daemon nobody has subscribed to.
  for path in &changed {
    let _ = handle.publish(Event::FileChanged {
      project_id: handle.project_id().to_string(),
      path: path.clone(),
    });
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

#[cfg(test)]
mod tests {
  use super::*;

  /// A project with one in-scope file under `intent/`, and the index a store
  /// would have recorded for it.
  fn project() -> (tempfile::TempDir, Vec<intentsvcs::sync::FileEntry>) {
    let dir = tempfile::tempdir().expect("tempdir");
    intentsvcs::init::init(dir.path(), "watched", "dc", "3.0.2").expect("init a project");
    let recorded = intentsvcs::sync::scan(dir.path(), &[]).expect("scan the fresh project");
    (dir, recorded)
  }

  /// **THE DEFECT, PLANTED RATHER THAN WAITED FOR.** macOS coalesces a write to
  /// `intent/.cache/` into one event naming `intent/`, and that event used to
  /// pass `Scanned::includes` -- so the daemon published `fileChanged` for a
  /// directory and ingested the write its own store had just made. Reconciling
  /// the subtree answers it with an empty set, because the skip list speaks at
  /// the leaf as the walk descends. Empty means the caller returns before it
  /// publishes anything and before it ingests.
  #[test]
  fn a_directory_event_over_the_daemons_own_store_write_names_nothing() {
    let (dir, recorded) = project();
    let root = dir.path();
    std::fs::create_dir_all(root.join("intent/.cache")).expect("mkdir");
    std::fs::write(
      root.join("intent/.cache/scratch"),
      b"the daemon's own write",
    )
    .expect("write");

    let intent_dir = root.join("intent");
    let changed = files_that_changed(root, &[intent_dir.as_path()], &mut || recorded.clone());

    assert!(
      changed.is_empty(),
      "a write inside `intent/.cache/` reconciled to {changed:?}, so the daemon would publish and then ingest its own store write"
    );
  }

  /// **AND THE SAME SHAPE OVER A REAL EDIT MUST STILL NAME THE LEAF.** A fix
  /// that answered every directory event with nothing would close the loop and
  /// stop delivering external edits, which is the worse failure and the one
  /// nobody notices.
  #[test]
  fn a_directory_event_over_a_real_edit_names_the_file_that_changed() {
    let (dir, recorded) = project();
    let root = dir.path();
    let edited = root.join("intent/wip.md");
    let before = std::fs::read_to_string(&edited).expect("the fixture has no wip.md");
    std::fs::write(&edited, format!("{before}\nan external edit\n")).expect("edit");

    let intent_dir = root.join("intent");
    let changed = files_that_changed(root, &[intent_dir.as_path()], &mut || recorded.clone());

    assert_eq!(
      changed,
      vec![edited.clone()],
      "the reconciliation must name the LEAF that changed and nothing else"
    );
  }

  /// **THE INDEX IS FETCHED AT MOST ONCE PER BATCH, WHATEVER THE BATCH HOLDS.**
  /// It is a round trip to the store thread and the ordinary case is a batch of
  /// leaf events, so the bound that protects that thread is per BATCH -- a
  /// batch of five hundred leaves costs one fetch, not five hundred.
  ///
  /// **IT ASSERTED ZERO UNTIL ISSUE `0311`, AND THE TRADE IS WORTH STATING.** A
  /// leaf now has a baseline because without one the daemon republished its own
  /// writes: an ingest writes canon and the generated views inside the watched
  /// scope, the leaf branch published them unread, and the daemon woke every
  /// subscriber with its own work and ingested again. **The second in-memory
  /// record of what the daemon wrote was refused** (vc, 2026-09-12) -- it would
  /// have kept the zero and put a fact the store already holds in a second
  /// home, which is the one that drifts.
  #[test]
  fn a_batch_of_leaf_events_costs_at_most_one_store_round_trip() {
    let (dir, recorded) = project();
    let root = dir.path();
    // Several leaves, because the bound is per batch: one of them cannot tell a
    // per-event fetch from a per-batch one, and this arm is about exactly that
    // difference.
    let leaves = [
      root.join("intent/wip.md"),
      root.join("intent/llm/RULES.md"),
      root.join("intent/llm/ARCHITECTURE.md"),
    ];
    let paths: Vec<&Path> = leaves.iter().map(std::path::PathBuf::as_path).collect();
    let mut asked = 0;

    let changed = files_that_changed(root, &paths, &mut || {
      asked += 1;
      recorded.clone()
    });

    assert!(
      changed.is_empty(),
      "the fixture's own files are what the store recorded, so a leaf event over them is the \
       daemon hearing its own write back: {changed:?}"
    );
    assert!(
      asked <= 1,
      "the store was asked for its index {asked} times for one batch; the bound is one fetch per \
       batch, shared across every path in it"
    );
  }

  /// **A DIRECTORY EVENT ON THE PROJECT ROOT MUST NOT NAME A FILE THE CORPUS
  /// DOES NOT CONTAIN.** `walk` keeps every file git would commit under the
  /// directory it is handed; outside `intent/` that is the whole repository,
  /// and the corpus out there is [`ROOT_FILES`] by name and nothing else. So a
  /// root-granularity event reconciled a `.prettierignore`, a `README.md` and
  /// every source file against the store's index -- and `Scanned::includes`,
  /// asked about the same `.prettierignore` as a LEAF, answers false. **Two
  /// answers about one file from one scope object, decided by which door the
  /// event came through**, which is the drift `Scanned` exists to prevent.
  #[test]
  fn a_root_event_names_only_what_the_corpus_contains() {
    let (dir, recorded) = project();
    let root = dir.path();

    // A file git commits, inside the tree, that the corpus does not hold --
    // and differing from the index, so only membership can keep it out.
    let outsider = root.join(".prettierignore");
    let before = std::fs::read_to_string(&outsider).unwrap_or_default();
    std::fs::write(
      &outsider,
      format!("{before}\na line the index has never seen\n"),
    )
    .expect("write the outsider");

    let on_root = files_that_changed(root, &[root], &mut || recorded.clone());
    let as_leaf = files_that_changed(root, &[outsider.as_path()], &mut || recorded.clone());

    assert!(
      as_leaf.is_empty(),
      "the control is void: `includes` already admits {outsider:?} as a leaf, so this test cannot show a disagreement"
    );
    assert!(
      on_root.is_empty(),
      "a directory event on the root named {on_root:?}, which the same scope object refuses as a leaf"
    );
  }

  /// **AND THE ROOT EVENT MUST STILL NAME A ROOT FILE THAT REALLY CHANGED.** A
  /// fix that answered every root event with nothing would drop the canon files
  /// that live out there, which is the quieter failure.
  #[test]
  fn a_root_event_still_names_a_root_file_that_changed() {
    let (dir, recorded) = project();
    let root = dir.path();
    let agents = root.join("AGENTS.md");
    let before = std::fs::read_to_string(&agents).expect("the fixture has no AGENTS.md");
    std::fs::write(&agents, format!("{before}\nan external edit\n")).expect("edit");

    let changed = files_that_changed(root, &[root], &mut || recorded.clone());

    assert_eq!(
      changed,
      vec![agents],
      "a root file that changed must still be named"
    );
  }

  /// **WHAT THE INDEX DISPATCH HANDS OVER, WHICH IS NOT THE SAME QUESTION AS
  /// WHAT THE RECONCILE THEN DOES WITH IT.** The root bound -- an event naming
  /// the root reconciles root-level files and never descends -- lives in
  /// `index::reconcile::changed_under` and is asserted there. This pins the
  /// dispatch: the daemon's own store write never costs a round trip, and a
  /// path the index scope refuses never reaches the door.
  #[test]
  fn the_daemons_own_store_write_never_reaches_the_index_door() {
    let (dir, _recorded) = project();
    let root = dir.path();
    let db = root.join("intent/.cache/intent.db");
    std::fs::create_dir_all(db.parent().expect("parent")).expect("mkdir");
    std::fs::write(&db, b"not source").expect("write");

    assert!(
      index_paths_to_refresh(root, &[db.as_path()]).is_empty(),
      "the daemon's own store write reached the index reconcile, which is the loop scope exists to close"
    );

    // **THE CONTROL, WITHOUT WHICH AN EMPTY ANSWER PASSES FOR A CORRECT ONE.**
    // A real source file in the same tree must still be handed over.
    std::fs::create_dir_all(root.join("src")).expect("mkdir");
    let leaf = root.join("src/main.rs");
    std::fs::write(&leaf, b"fn main() {}\n").expect("write");
    assert_eq!(
      index_paths_to_refresh(root, &[leaf.as_path()]),
      vec![leaf.clone()],
      "a source file in the index scope did not reach the door"
    );
  }

  /// **AND A DIRECTORY BELOW THE ROOT IS HANDED OVER WHOLE**, because there the
  /// reconcile's filter means what it says. A fix that bounded every directory
  /// event would stop source edits reaching the index at all, which is the
  /// quieter failure.
  #[test]
  fn a_directory_below_the_root_is_handed_to_the_index_reconcile_as_itself() {
    let (dir, _recorded) = project();
    let root = dir.path();
    std::fs::create_dir_all(root.join("src")).expect("mkdir");
    std::fs::write(root.join("src/main.rs"), b"fn main() {}\n").expect("write");

    let src = root.join("src");
    assert_eq!(
      index_paths_to_refresh(root, &[src.as_path()]),
      vec![src.clone()],
      "a directory below the root must be reconciled as itself"
    );
    let leaf = root.join("src/main.rs");
    assert_eq!(
      index_paths_to_refresh(root, &[leaf.as_path()]),
      vec![leaf.clone()],
      "a leaf in the index scope must reach the reconcile"
    );
  }

  #[test]
  fn a_removal_under_an_ignored_directory_and_a_directory_under_git_reach_no_refresh() {
    // Issue 0355: a deletion burst under `target/` reached the store thread as
    // a whole-repository walk per debounced batch.
    let (dir, _recorded) = project();
    let root = dir.path();
    let ok = std::process::Command::new("git")
      .args(["init", "-q"])
      .current_dir(root)
      .status()
      .expect("run git")
      .success();
    assert!(ok, "git init failed");
    std::fs::write(root.join(".gitignore"), b"target/\n").expect("gitignore");
    std::fs::create_dir_all(root.join("native/rust/target/debug/deps")).expect("mkdir");
    let removed = root.join("native/rust/target/debug/deps/lib.rcgu.o");
    let created = root.join(".git/refs/scratch");
    std::fs::create_dir_all(&created).expect("mkdir under .git");

    assert!(
      index_paths_to_refresh(root, &[removed.as_path(), created.as_path()]).is_empty(),
      "a removal under an ignored directory or a directory under `.git` reached the index refresh"
    );
  }

  /// A path that has been deleted cannot be classified, so it is reconciled
  /// like a directory -- and a deleted subtree honestly holds nothing in scope.
  #[test]
  fn a_vanished_path_is_reconciled_rather_than_guessed_at() {
    let (dir, recorded) = project();
    let root = dir.path();
    let gone = root.join("intent/st/ST0404");

    let changed = files_that_changed(root, &[gone.as_path()], &mut || recorded.clone());

    assert!(
      changed.is_empty(),
      "a path that is not there named {changed:?}"
    );
  }
}
