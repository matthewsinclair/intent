//! `intent organize` -- the reconciliation verb (D57-3, ST0057 WP-04).
//!
//! Four answers, one refusal, one gate. Given the store, the manifest and a
//! listing of the tree, decide for every path which of D57-3's five rows it
//! falls in, and do the minimum that makes disk agree with the declaration.
//!
//! **PLAN AND APPLY ARE SPLIT, AND THE SPLIT IS THE TESTABILITY.** [`plan`]
//! reads and decides; nothing here writes or removes. All five rows are
//! therefore drivable without a filesystem mutation, which matters because four
//! of the six criteria on this work package are about what `organize` REFUSES to
//! do -- and a refusal is only observable if the thing it refuses can be set up
//! cheaply.
//!
//! **WHAT IS BUILT SO FAR: the plan and the gate. THE APPLY PATH IS NOT.** Said
//! here rather than left to be inferred from an absent function, because a module
//! whose docstring describes a verb reads as a verb that exists. `plan` decides,
//! [`gate`] proves a removal safe, and no caller yet turns either into a write --
//! so `intent organize` is not a runnable command at this commit.
//!
//! **IT REUSES `Project::classify` RATHER THAN RESTATING THE ELIGIBILITY
//! CONTRACT.** `ThreadFile::Unattached` already IS D57-3's fifth row. A second
//! expression of "what counts as a view" -- here, or in a shell tool -- would be
//! the fourth list in the estate, and the one that goes stale is always the one
//! nobody is looking at when a new view kind lands.
//!
//! **AND THE APPLY PATH MUST WRITE THROUGH `WriteSet`, WHERE THE MTIME SKIP
//! ALREADY LIVES.** AC-04.4 wants a second run to move zero mtimes. That is not
//! implemented here and must not be: `views::write_all` once grew its own
//! skip-when-unchanged guard, it was correct, and it reached nothing because the
//! production path did not go through it. `view_determinism.rs` drove the guard
//! directly and stayed green while every real verb churned the estate. Idempotence
//! is inherited by using the one write mechanism, or it is a second guard that
//! will diverge from it.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::ingest::Canon;
use crate::intentfiles::{Manifest, Sigil};
use crate::project::{Project, ThreadFile};
use crate::views::{self, RenderContext, View};

/// Views that the renderer produces but which name no single artefact.
///
/// **NAMED, NOT INFERRED (AC-04.6).** `steel_threads.md` and `todo.md` are
/// renderer output, so the fifth row would never claim them -- but they are also
/// not per-artefact views, so no manifest entry implies them and the declared
/// set can never contain them. Without this list they would be DEHYDRATED on the
/// first real run.
///
/// Leaving them to be caught by D57-3's fifth row was considered and refused.
/// UNCLAIMED means *a human put this here*, and using it to shelter something the
/// renderer demonstrably does produce would make the unclaimed report meaningless
/// -- which is the one report in this verb that a human has to act on personally.
const EXEMPT_REASON: &str = "an index view: renderer-produced, but names no single artefact, so no manifest entry can imply it";

/// What `organize` decided about one path. D57-3's table, one variant per row,
/// plus the exempt set above.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
  /// Declared, absent from disk. Write it.
  Hydrate,
  /// Declared and present. Re-render; [`WriteSet`] writes only if bytes differ.
  Verify,
  /// Not declared, present on disk. Remove it -- **subject to the gate**.
  Dehydrate,
  /// A path the renderer does not produce. **Report, never remove.**
  Unclaimed,
  /// A renderer-produced view that no manifest entry can imply. Kept, always.
  Exempt,
}

impl Action {
  /// Whether this action removes bytes. Exactly one does, which is why the gate
  /// has exactly one place to stand.
  pub fn is_destructive(&self) -> bool {
    matches!(self, Action::Dehydrate)
  }
}

/// One decided path.
#[derive(Debug, Clone)]
pub struct Step {
  pub path: PathBuf,
  pub action: Action,
  /// The rendered bytes, for the rows that have any. `None` for [`Action::Unclaimed`]
  /// -- the renderer cannot produce those, which is what makes them unclaimed.
  pub content: Option<String>,
}

/// A decided run, not yet applied.
#[derive(Debug, Clone)]
pub struct Plan {
  pub steps: Vec<Step>,
  /// The tree digest as measured while planning. Re-computed immediately before
  /// the irreversible step; any difference refuses the run.
  pub digest: String,
}

impl Plan {
  /// Steps carrying a given action, in path order.
  pub fn with(&self, action: Action) -> impl Iterator<Item = &Step> {
    self.steps.iter().filter(move |s| s.action == action)
  }

  /// Whether this plan would remove anything. A plan that removes nothing needs
  /// no digest re-check, because there is no irreversible step to guard.
  pub fn is_destructive(&self) -> bool {
    self.steps.iter().any(|s| s.action.is_destructive())
  }
}

/// Why `organize` refused.
///
/// **EVERY VARIANT NAMES THE PATH.** A reconciliation verb that refuses without
/// saying which file sends the operator to diff a whole tree against a
/// description of one, and the refusals here are the entire point of the verb.
#[derive(Debug, Error)]
pub enum OrganizeError {
  /// The dehydration gate (AC-04.2). Disk holds something the store does not,
  /// and removing the file destroys it.
  #[error(
    "refusing to dehydrate {path}: the file on disk differs from what the store renders, so removing it would destroy {bytes} byte(s) the store does not carry. Reconcile it first -- `intent doctor` names the difference, and if the disk copy is the one you want, the edit belongs in canon."
  )]
  HandEdited { path: PathBuf, bytes: usize },

  /// The moment-of-act digest (AC-04.5). Something wrote to the tree between
  /// planning and applying.
  #[error(
    "refusing to apply: the tree changed between the plan and the act ({detail}). Every read verb materialises the store on access, so a peer running `intent st list` is enough -- re-run and it will re-plan against what is there now."
  )]
  TreeMoved { detail: String },

  /// An attachment diverges from the store (AC-04.3). `organize` reports and
  /// modifies neither side.
  #[error(
    "attachment divergence at {path}: authority follows AUTHORSHIP, and an attachment is authored ON DISK, so this means the STORE is stale -- the opposite remedy from a divergent view. `organize` will not choose for you: run `intent sync --to-store` to take the disk copy, or restore the file if the store is right."
  )]
  AttachmentDiverged { path: PathBuf },

  #[error("could not read {path}: {source}")]
  Io {
    path: PathBuf,
    #[source]
    source: std::io::Error,
  },
}

fn io_err(path: &Path, source: std::io::Error) -> OrganizeError {
  OrganizeError::Io {
    path: path.to_path_buf(),
    source,
  }
}

/// The artefacts the manifest declares, as a set of `(sigil, id)`.
///
/// **PINNED AND GENERATED ARE UNIONED, AND THE REGION IS NOT CONSULTED HERE.**
/// The two regions differ in who WRITES them and whether they survive a rewrite
/// -- both declare realisation equally. Filtering to one region would silently
/// dehydrate every pinned thread, which is the exact decision AC-02.3 exists to
/// preserve.
fn declared_artefacts(manifest: &Manifest) -> BTreeSet<(&'static str, String)> {
  manifest
    .entries
    .iter()
    .map(|e| (e.sigil.as_str(), e.id.clone()))
    .collect()
}

/// A path re-based from the project root onto the THREAD directory, which is the
/// frame [`Project::classify`] reads.
///
/// Returns `None` for anything not under `st_dir`, and for `st_dir` itself or a
/// bare thread directory -- there is no file there to classify, and answering
/// anyway would put a directory into a population that is counted as files.
fn thread_relative(project: &Project, path: &Path) -> Option<PathBuf> {
  let under_st = path.strip_prefix(project.st_dir()).ok()?;
  let mut comps = under_st.components();
  comps.next()?; // the thread id
  let rel: PathBuf = comps.collect();
  if rel.as_os_str().is_empty() {
    return None;
  }
  Some(rel)
}

/// Compute a plan without touching the tree.
///
/// `on_disk` is the listing -- passed in rather than walked here so the five
/// rows can be driven against a constructed population. The production caller
/// supplies a real walk.
pub fn plan(
  project: &Project,
  canon: &Canon,
  manifest: &Manifest,
  ctx: &RenderContext<'_>,
  on_disk: &[PathBuf],
  digest: String,
) -> Plan {
  let declared = declared_artefacts(manifest);

  // Everything the renderer can produce, keyed by path. This is BOTH the source
  // of hydration bytes and the denominator for "unclaimed" -- a path absent from
  // this map is one the renderer cannot make, which is the fifth row's exact
  // definition rather than a proxy for it.
  let renderable: BTreeMap<PathBuf, String> = views::render_all(project, canon, ctx)
    .into_iter()
    .map(|View { path, content }| (path, content))
    .collect();

  let exempt: BTreeSet<PathBuf> = [project.steel_threads_view(), project.todo_view()]
    .into_iter()
    .collect();

  // Which renderable paths belong to a DECLARED artefact. Built by asking the
  // canon which thread owns each view, rather than by parsing the path -- a path
  // parser here would be a second spelling of the layout that `Project`'s view
  // helpers already own.
  let mut declared_paths: BTreeSet<PathBuf> = BTreeSet::new();
  for thread in &canon.threads {
    if !declared.contains(&(Sigil::SteelThread.as_str(), thread.id.clone())) {
      continue;
    }
    declared_paths.insert(project.info_view(&thread.id));
    declared_paths.insert(project.acceptance_view(&thread.id));
    for wp in &thread.wps {
      declared_paths.insert(project.wp_info_view(&thread.id, wp.seq));
    }
  }

  let present: BTreeSet<&PathBuf> = on_disk.iter().collect();
  let mut steps = Vec::new();

  // Rows one and two: declared. Absent -> hydrate, present -> verify.
  for path in &declared_paths {
    let content = renderable.get(path).cloned();
    let action = if present.contains(path) {
      Action::Verify
    } else {
      Action::Hydrate
    };
    steps.push(Step {
      path: path.clone(),
      action,
      content,
    });
  }

  // Rows four and five, over what is actually on disk.
  for path in on_disk {
    if declared_paths.contains(path) {
      continue; // already decided above
    }
    if exempt.contains(path) {
      steps.push(Step {
        path: path.clone(),
        action: Action::Exempt,
        content: renderable.get(path).cloned(),
      });
      continue;
    }
    // **`classify` TAKES A PATH RELATIVE TO THE THREAD DIRECTORY, NOT TO
    // `st_dir`.** Its view arms are `depth == 1` for `info.md` and `depth == 3`
    // for `WP/NN/info.md`, so handing it `ST0001/info.md` makes every view read
    // as depth 2 and fall through to `Unattached` -- which would report every
    // generated view in the estate as something a human put there, and dehydrate
    // none of them. Silent, and in the direction that looks safe.
    let Some(rel) = thread_relative(project, path) else {
      // Not under `st_dir` at all. Issues have their own layout and their own
      // criteria; deciding them with a thread classifier would be a second,
      // wrong, spelling of that layout. Left for the issues arm rather than
      // guessed at here.
      continue;
    };
    match Project::classify(&rel) {
      // The renderer produces this shape and nothing declares it: row four.
      ThreadFile::GeneratedView => steps.push(Step {
        path: path.clone(),
        action: Action::Dehydrate,
        content: renderable.get(path).cloned(),
      }),
      // Canon and attachments are not this verb's business. Canon is not a view,
      // and an attachment is authored on disk -- AC-04.3 keeps both out.
      ThreadFile::Canon | ThreadFile::Attachment => {}
      // Row five. Reported by name, never removed, and it carries no content
      // because there is nothing that could render it.
      ThreadFile::Unattached => steps.push(Step {
        path: path.clone(),
        action: Action::Unclaimed,
        content: None,
      }),
    }
  }

  steps.sort_by(|a, b| a.path.cmp(&b.path));
  Plan { steps, digest }
}

/// The reason an index view is kept. Exposed so the report can print it rather
/// than restate it.
pub fn exempt_reason() -> &'static str {
  EXEMPT_REASON
}

/// The dehydration gate (AC-04.2).
///
/// Re-render into memory, compare to the bytes on disk, refuse on ANY difference
/// and name the path. **Fail-safe by construction rather than by discipline:**
/// the only way to remove a view is to have proved first that the store can
/// reproduce it exactly, so a hand edit cannot be destroyed by an operator who
/// forgot to check.
///
/// A view the renderer cannot reproduce at all is refused too, on the same
/// ground: if there are no bytes to compare against, the claim "the store
/// carries this" is unproven, and unproven is not permission.
pub fn gate(step: &Step) -> Result<(), OrganizeError> {
  debug_assert!(step.action.is_destructive(), "gate is for removals only");
  let on_disk = std::fs::read_to_string(&step.path).map_err(|e| io_err(&step.path, e))?;
  match &step.content {
    Some(rendered) if *rendered == on_disk => Ok(()),
    _ => Err(OrganizeError::HandEdited {
      path: step.path.clone(),
      bytes: on_disk.len(),
    }),
  }
}
