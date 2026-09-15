//! `intent organize` -- the reconciliation verb (D57-3, ST0057 WP-04).
//!
//! Four answers, one refusal, one gate. Given the store, the manifest and a
//! listing of the tree, decide for every path which of D57-3's five rows it
//! falls in, and do the minimum that makes disk agree with the declaration.
//!
//! **PLAN AND APPLY ARE SPLIT, AND THE SPLIT IS THE TESTABILITY.** [`plan`]
//! reads and decides; [`Plan::apply`] is the only thing that writes or removes.
//! All five rows are therefore drivable without a filesystem mutation, which
//! matters because four of the six criteria on this work package are about what
//! `organize` REFUSES to do -- and a refusal is only observable if the thing it
//! refuses can be set up cheaply.
//!
//! **WHAT IS BUILT: plan, gate and apply. WHAT IS NOT: the HANDLER.** Said here
//! rather than left to be inferred, because a module whose docstring describes a
//! verb reads as a verb a user can type. `intent organize` is now a declared
//! command -- hv reclaimed the name for v3 on 2026-08-19 and the dispatch row
//! ships -- but nothing calls into this module, so the verb answers rc=2, `is a
//! known command that is not implemented yet`. Driven in a disposable clone by
//! vc: 744 files under `intent/st/` before, 744 after.
//!
//! **THIS PARAGRAPH SAID "WHAT IS NOT: the dispatch entry" UNTIL THE ENTRY
//! LANDED, AND THEN IT WAS FALSE IN THE OTHER DIRECTION** -- a build-state claim
//! with no revision on it, which is the class I spent the same day writing down
//! twice elsewhere. A docstring about what exists yet is a claim about a moment;
//! it goes stale by being right at the time.
//!
//! **AND WHEN THE HANDLER IS WIRED, THAT IS THE MOMENT THE ESTATE BECOMES
//! DESTRUCTIBLE** (vc, flagged once rather than gated on). Four of D57's seven
//! dehydration preconditions are unbuilt: AC-00.1's gate is `to-write` and its
//! file absent, the conservation verdict is STRANDED, WP-06's text realisation is
//! unbuilt, and opaque-attachment canon is unstarted. `.intentfiles` currently
//! declares NOTHING -- every line a comment, the generated region empty -- against
//! 745 files under `intent/st/`. **A verb that dehydrates what the manifest does
//! not name, wired before the gate that refuses when a precondition is unmet,
//! ships able to do precisely what the gate exists to prevent.** Wire the gate
//! first.
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
use crate::intentfiles::Realised;
use crate::preconditions::{self, Verdict};
use crate::project::{Project, ThreadFile};
use crate::views::{self, RenderContext, View};
use crate::write_set::WriteSet;

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
  /// Declared, carried in the store, absent from disk. Write it from the store's
  /// copy. Attachments are not rendered, so this is a distinct row from
  /// [`Action::Hydrate`] and shares none of its bytes.
  HydrateAttachment,
  /// Present on disk and NOT what the store carries. **Reported, both remedies
  /// named, neither side touched** (AC-04.3).
  AttachmentDiverged,
}

impl Action {
  /// Whether this action removes bytes. Exactly one does, which is why the gate
  /// has exactly one place to stand.
  pub fn is_destructive(&self) -> bool {
    matches!(self, Action::Dehydrate)
  }

  /// What a human is asked to do, for the rows `organize` deliberately will not
  /// resolve itself.
  ///
  /// **BOTH VERBS, NEVER ONE (AC-04.3).** Authority follows AUTHORSHIP: a view
  /// diverging means the FILE is stale, an attachment diverging means the STORE
  /// is. Same observation, opposite remedies -- so naming only one would be this
  /// verb quietly choosing whose work to discard, right most of the time and
  /// catastrophically wrong occasionally, which is the worst profile available.
  pub fn remedy(&self) -> Option<&'static str> {
    match self {
      Action::AttachmentDiverged => Some(
        "an attachment is authored ON DISK, so this means the STORE is stale: `intent sync --to-store` takes the disk copy. If the store is the one you want, restore the file instead. organize will not choose for you.",
      ),
      Action::Unclaimed => Some(
        "nothing renders this and nothing carries it. Leave it, or make it an attachment -- organize is not the thing that decides an unrecognised file is rubbish.",
      ),
      _ => None,
    }
  }
}

/// Whether a run may touch the disk.
///
/// **THE PREVIEW AND THE PERFORMED RUN ARE ONE BODY TAKING THIS PARAMETER, NOT
/// TWO IMPLEMENTATIONS THAT AGREE BY INSPECTION.** A preview computed by its own
/// path is a *promise* about what the other path would do, and the two drift in
/// the direction nobody notices: the preview stays green while the act changes
/// underneath it, so the operator is shown a reassurance rather than a
/// measurement. Here the preview IS the act with three things withheld -- the
/// removal, the write, and the digest guard that stands over the removal -- so
/// "what it said it would do" and "what it does" are not two things the code is
/// able to disagree about.
///
/// **v2 SHIPPED BOTH ANSWERS FOR ONE OPERATION AND THAT IS WHY THIS IS AN ENUM
/// RATHER THAN A `bool`.** `intent organize` took `--dry-run`, `intent st
/// organize` took `--write`: same operation, opposite defaults, recorded in the
/// dispatch table as a live Highlander violation. A `bool` at this seam is a
/// parameter whose meaning has to be remembered at every call site, which is
/// exactly the condition under which two faces come to disagree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
  /// Decide everything; touch nothing. The default spelling of the verb.
  Preview,
  /// Decide everything and perform it. Reached only by `--apply`.
  Apply,
}

impl Mode {
  /// Whether this run is permitted to change the tree.
  pub fn performs(&self) -> bool {
    matches!(self, Mode::Apply)
  }
}

/// One decided path.
#[derive(Debug, Clone)]
pub struct Step {
  pub path: PathBuf,
  pub action: Action,
  /// The bytes this step writes or is gated against, for the rows that have
  /// any: a view's render as UTF-8, an attachment's carried bytes in whichever
  /// form the store holds them. `None` for [`Action::Unclaimed`] -- the renderer
  /// cannot produce those, which is what makes them unclaimed -- and for an
  /// opaque attachment whose bytes were never loaded, which [`Plan::run`]
  /// refuses by path rather than skipping (issue 0338 (i)).
  pub content: Option<Vec<u8>>,
}

/// What [`plan`] needs to know about the tree.
///
/// Passed in rather than walked inside `plan`, so every row can be driven against
/// a constructed population -- which is what makes the four refusal criteria on
/// this work package cheap to exercise.
#[derive(Debug, Clone, Default)]
pub struct TreeState {
  /// Every file present under the realised locations.
  pub present: BTreeSet<PathBuf>,
  /// SHA-256 of the ATTACHMENTS on disk, lowercase hex.
  ///
  /// **Attachments only, and the omission is deliberate.** A view's identity is
  /// its rendered bytes, which the plan already holds in `renderable`; hashing
  /// one would be a second way to ask a question already answered exactly, and
  /// the second way is the one that goes stale.
  pub sha256: BTreeMap<PathBuf, String>,
}

/// A decided run, not yet applied.
#[derive(Debug, Clone)]
pub struct Plan {
  pub steps: Vec<Step>,
  /// What the v2 tree left behind, and whether the store holds it (WP-02).
  ///
  /// **CARRIED ON THE PLAN, NOT RECOMPUTED INSIDE THE APPLY**, for
  /// [`Plan::preconditions`]'s reason: a removal an operator cannot see before
  /// it happens is the one line of this verb nobody can review. It is computed
  /// once, printed by the preview, and acted on by the same body.
  pub leftovers: crate::legacy::Leftovers,
  /// The tree digest as measured while planning. Re-computed immediately before
  /// the irreversible step; any difference refuses the run.
  pub digest: String,
  /// AC-00.1's ship gate, resolved while planning.
  ///
  /// **Carried on the plan rather than consulted inside `apply`, so a plan can
  /// be inspected for what it would be ALLOWED to do and not only for what it
  /// intends.** A gate whose answer exists only inside the destructive call is
  /// one nobody can report on before running it.
  pub preconditions: Verdict,
  /// The estate root, carried so [`Plan::run`] has a FLOOR it cannot prune
  /// above.
  ///
  /// **A directory-removing loop without a declared floor walks to `/`.** The
  /// cascade below removes a parent that its child's removal emptied, and the
  /// only thing stopping that from continuing past `intent/st` is a bound
  /// stated in data rather than inferred from a path shape. Carried on the plan
  /// rather than passed to `run`, so a plan can be inspected for the region it
  /// is allowed to touch, exactly as `preconditions` lets it be inspected for
  /// what it is allowed to do.
  pub estate_root: PathBuf,
  /// Declared threads this plan will not realise, because a v2 status bucket
  /// still holds their files (issue 0209). Their steps are absent from `steps`,
  /// and [`Plan::run`] reports each one as a refusal.
  pub held: Vec<Held>,
}

/// A declared thread whose realisation is held back (issue 0209).
///
/// **THE PREDICATE IS "THE DISK CARRIES FILES FOR THIS THREAD THAT THE
/// REALISATION WOULD LEAVE BEHIND", NOT A SIZE COMPARISON.** A realisation can
/// be byte-larger and still drop `tasks.md`, so size tracks nothing (laksa-dc's
/// correction, recorded on the issue). Measured on Laksa's ST0106: realising
/// from a store that never carried `design.md`/`impl.md`/`tasks.md` wrote 9493
/// bytes at `intent/st/ST0106/` while the 14789-byte original stayed tracked
/// under `NOT-STARTED/` -- two directories, and the incomplete one looked
/// authoritative.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Held {
  pub thread: String,
  /// The bucket directory holding the files, eg `intent/st/NOT-STARTED/ST0106`.
  pub dir: PathBuf,
  /// Where the thread would have been realised.
  pub home: PathBuf,
  pub files: usize,
}

impl Plan {
  /// Steps carrying a given action, in path order.
  /// This plan narrowed to the removals of the paths `keep` names, and nothing
  /// else: no hydration, no v2 prune, no held-thread refusal (issue 0316).
  ///
  /// **THE NARROWING IS OF THE STEPS, NOT OF THE GATES.** The steps kept run
  /// through [`Plan::run`] exactly as organize's own do -- the preconditions,
  /// the re-observation guard and the per-file [`gate`] -- so a caller that
  /// wants only these removals gets organize's refusals with them. Dropping the
  /// leftovers is what keeps the v2 prune out: an ingest and a removal are not
  /// one run (0319), and a caller of this is not running the prune.
  pub fn only_dehydrating(self, keep: impl Fn(&Path) -> bool) -> Plan {
    Plan {
      steps: self
        .steps
        .into_iter()
        .filter(|s| s.action == Action::Dehydrate && keep(&s.path))
        .collect(),
      leftovers: crate::legacy::Leftovers::default(),
      held: Vec::new(),
      ..self
    }
  }

  pub fn with(&self, action: Action) -> impl Iterator<Item = &Step> {
    self.steps.iter().filter(move |s| s.action == action)
  }

  /// Whether this plan would remove anything. A plan that removes nothing needs
  /// no digest re-check, because there is no irreversible step to guard.
  pub fn is_destructive(&self) -> bool {
    // **THE v2 PRUNE COUNTS, AND LEAVING IT OUT MADE THE DOOR UNREACHABLE.**
    // This predicate decides whether the removal branch runs at all, so a plan
    // whose only removals were v2 leftovers took no removal path, reported an
    // empty prune and left the tree exactly as it found it -- a door that
    // compiles, reports nothing and does nothing. Found by the arm rather than
    // by review, which is what the arm is for.
    //
    // **A REFUSAL IS DESTRUCTIVE-SHAPED TOO**: withheld paths must reach
    // `refused`, and that also happens inside this branch.
    self.steps.iter().any(|s| s.action.is_destructive())
      || !self.leftovers.removable.is_empty()
      || self.leftovers.refuses()
  }
}

/// Why `organize` refused.
///
/// **EVERY VARIANT NAMES THE PATH.** A reconciliation verb that refuses without
/// saying which file sends the operator to diff a whole tree against a
/// description of one, and the refusals here are the entire point of the verb.
#[derive(Debug, Error)]
pub enum OrganizeError {
  /// The dehydration gate (AC-04.2). The file on disk differs from what the
  /// store renders, and removing it would lose a hand edit if that is what
  /// the difference is.
  ///
  /// **A DIFFERENCE IS NOT A DIRECTION** (issue `0283`). This said removing
  /// the file "would destroy N byte(s) the store does not carry", which is
  /// false whenever the STORE is ahead -- until 0283's half B, a change to a
  /// thread `.intentfiles` no longer lists left its views behind, so the disk
  /// copy is a stale render with nothing authored in it. Half B stops new ones
  /// forming; an estate can still carry one from before. Nothing here can
  /// tell the two apart after the fact, so the gate still refuses; what
  /// changed is that it no longer tells the operator their file holds work it
  /// may not.
  #[error(
    "refusing to dehydrate {path}: the file on disk ({bytes} byte(s)) differs from what the store renders -- either a hand edit the store never took in, or a render the store has since moved past (before v3.0.1, a change to a thread `.intentfiles` no longer lists left its views behind). Removing it would destroy the hand edit if there is one -- a wanted edit belongs in canon, made through the CLI -- so organize does not guess."
  )]
  HandEdited { path: PathBuf, bytes: usize },

  /// A thread's other files, withheld because one of them was refused (issue
  /// 0343).
  ///
  /// **A THREAD'S FILES ARE ONE SET.** Each removal used to be gated alone, so a
  /// thread whose cover the gate refused lost its attachments and kept its
  /// views: a tree neither realised nor dehydrated. The refusal beside this one
  /// names the file that stopped it.
  #[error(
    "withheld {} other file(s) of {thread} from dehydration, because a file of the thread was refused: a thread is dehydrated whole or not at all",
    withheld.len()
  )]
  ThreadWithheld {
    thread: String,
    withheld: Vec<PathBuf>,
  },

  /// A v2 leftover whose content the store does not hold (WP-02, AC-02.2).
  ///
  /// **ONE OF THESE REFUSES THE WHOLE PRUNE**, because the usual cause is the
  /// INGEST rather than the file: an ingest that failed for one thread is
  /// evidence about the run, and removing the fifty-three it did carry while
  /// one is unheld is the half-migrated estate the per-file content probe
  /// exists to catch.
  #[error(
    "refusing to prune {path}: {reason}. The v2 tree is removed only once the store holds every one of its files, so this run removes none of them -- re-run `intent upgrade` to ingest what is missing, or move this file out of the v2 tree by hand if it is not the record of any work."
  )]
  LegacyUnheld { path: PathBuf, reason: String },

  /// The moment-of-act digest (AC-04.5). Something wrote to the tree between
  /// planning and applying.
  #[error(
    "refusing to apply: the tree changed between the plan and the act ({detail}). Every read verb materialises the store on access, so a peer running `intent st list` is enough -- re-run and it will re-plan against what is there now."
  )]
  TreeMoved { detail: String },

  /// The dehydration SHIP gate (AC-00.1). The estate has not yet proved it can
  /// put back what dehydration would remove.
  ///
  /// **It names every unmet precondition and prints the denominator**, because
  /// a refusal reporting only the first one trains an operator to fix that one
  /// and re-run -- and a count with no denominator cannot be told from a gate
  /// that checked nothing.
  ///
  /// **AND IT NAMES THE THREADS WHOSE FILES ARE HELD, WHICH A COUNT DOES NOT.**
  /// One refusal for the whole run is right -- the unmet precondition is a
  /// property of the estate -- but the consequence lands on particular threads,
  /// and `would remove 2 file(s)` leaves the operator to work out which two by
  /// diffing the estate against a description of it. The threads are the
  /// smallest thing that turns the refusal into something actionable, and they
  /// are what AC-11.6 asks the refusal to name beside the precondition.
  #[error(
    "refusing to dehydrate: this run would remove {removals} file(s) belonging to {}, and this estate has not proved it can put them back -- {verdict}. Each precondition is an acceptance criterion of this project; `intent ac list` shows the state of every one, and this gate records no answer of its own.",
    if .threads.is_empty() { "no thread this gate could name".to_string() } else { .threads.join(", ") }
  )]
  PreconditionsUnmet {
    removals: usize,
    verdict: Verdict,
    /// The threads owning the held files, deduplicated, in path order.
    threads: Vec<String>,
  },

  /// An attachment diverges from the store (AC-04.3). `organize` reports and
  /// modifies neither side.
  #[error(
    "attachment divergence at {path}: authority follows AUTHORSHIP, and an attachment is authored ON DISK, so this means the STORE is stale -- the opposite remedy from a divergent view. `organize` will not choose for you: run `intent sync --to-store` to take the disk copy, or restore the file if the store is right."
  )]
  AttachmentDiverged { path: PathBuf },

  /// The tree could not be READ. Its own variant rather than folded into
  /// [`OrganizeError::Io`]: a failed walk means the plan was computed against a
  /// population that is not the estate, and every other refusal here is about a
  /// named file the walk succeeded on.
  #[error(
    "refusing to realise {thread}: the v2 status bucket {dir} still holds {files} file(s) for it, and realising from the store would write a second, smaller copy at {home} while that one stays -- two directories, and the incomplete one would look authoritative."
  )]
  LegacyCopyPresent {
    thread: String,
    dir: PathBuf,
    home: PathBuf,
    files: usize,
  },

  #[error("could not read the tree to reconcile it: {source}")]
  Scan {
    #[source]
    source: crate::sync::SyncError,
  },

  /// A file realisation has to write and holds no bytes for (issue 0338 (i)).
  ///
  /// **REFUSED BY PATH RATHER THAN SKIPPED.** The write loop passed over a step
  /// with no bytes while the report still listed it as hydrated, so an opaque
  /// attachment whose sidecar was never loaded read as realised and was never
  /// written. Writing an empty file instead would be worse: present, and wrong.
  #[error("nothing was written to {path}: the store holds no bytes for it")]
  NothingToWrite { path: PathBuf },

  #[error("could not read {path}: {source}")]
  Io {
    path: PathBuf,
    #[source]
    source: std::io::Error,
  },
}

/// The one remedy for a v2 file the store does not hold (issue 0318).
///
/// **ONE HOME, TWO FACES**: the terminal prints it once under the class line,
/// and every per-file `LegacyUnheld` a face lists carries the same sentence.
///
/// **IT NAMES BOTH EXITS, BECAUSE `intent upgrade` ALONE IS NOT ONE FOR EVERY
/// FILE** (vc, 2026-09-13). Since 0319 an upgrade ingests every bucket file it
/// can and reports each one it cannot -- a name the naming gate refuses, a file
/// over the size cap -- with the reason. Re-running it for one of those changes
/// nothing, so a remedy naming only the upgrade sends the operator round a loop.
const LEGACY_UNHELD_REMEDY: &str = "run `intent upgrade`: it ingests every v2 file it can, and names each one it cannot with the reason. A file it reports as not ingested stays unheld until it is renamed or moved out of the v2 tree by hand, which is also the way out for a file that is not the record of any work. Nothing is removed until every one of them is held.";

impl crate::remedy::Remedy for OrganizeError {
  /// **ONE ACTION PER REFUSAL, AND THEY ARE GENUINELY DIFFERENT ACTIONS.**
  /// Four of these are the verb doing its job, so a shared sentence here would
  /// tell an operator nothing on the occasions the verb is most useful: a hand
  /// edit is reconciled, a moved tree is re-run, a divergence is a CHOICE the
  /// verb refuses to make, and an unmet precondition is not the operator's to
  /// fix at all.
  fn remedy(&self) -> String {
    match self {
      // `sync --to-store` was named here for the keep-the-file case, and it
      // does not take a hand edit to a generated view into the model -- only
      // canon and the info covers are read back -- so that remedy did nothing
      // for most of the files this refusal is about (issue `0283`).
      // **THE REFUSAL IS ABOUT THE RUN, SO THE REMEDY NAMES THE RUN** -- and
      // since 0318 it is the class's one remedy, shared with the line the
      // terminal prints once. See `LEGACY_UNHELD_REMEDY`.
      Self::LegacyUnheld { .. } => LEGACY_UNHELD_REMEDY.to_string(),
      Self::HandEdited { path, .. } => format!(
        "decide which copy is right. `intent doctor` names the difference and the command that regenerates it. If nobody edited the file at {}, the store is right: delete it and re-run. If it holds an edit you want, make the change through the CLI so it lands in the model, then re-run.",
        path.display()
      ),
      Self::ThreadWithheld { thread, .. } => format!(
        "clear the refusal naming a file of {thread}, then re-run `intent organize --apply`, and its files are removed together. Nothing of {thread} was removed."
      ),
      // **The action is to run it again, and saying so is only honest because
      // re-running re-plans from scratch.** A guard whose remedy is "retry"
      // trains an operator to retry until it passes, so the sentence has to say
      // what the second run does differently.
      Self::TreeMoved { .. } => "re-run `intent organize`. It re-plans against the tree as it is now rather than resuming the plan it refused, so the second run is a fresh decision and not the first one forced through.".to_string(),
      Self::AttachmentDiverged { path } => format!(
        "choose, because `organize` will not: `intent sync --to-store` takes the copy at {} as authoritative, or restore the file from git if the store is right. An attachment is authored ON DISK, so this means the STORE is stale -- the opposite remedy from a divergent view.",
        path.display()
      ),
      // **NOT THE OPERATOR'S TO FIX, AND THE REMEDY SAYS SO RATHER THAN
      // OFFERING A LEVER.** Every other remedy here names something to do to
      // this estate. This one names work that has to land first, and inventing
      // an override would hand out exactly the bypass the gate exists to
      // refuse.
      Self::PreconditionsUnmet { .. } => "dehydration stays gated until this project's declared preconditions are met, and the refusal above names every one that is not. `intent ac list` shows the state of each. Nothing here needs undoing: hydration and verification in the same run were unaffected, and no file was removed.".to_string(),
      // **NOT `intent st hydrate`**: it realises from the same store, so it
      // would write the same smaller copy. Moving the directory keeps what the
      // store does not model beside the views, where `organize` reports it as
      // unclaimed and never removes it.
      Self::LegacyCopyPresent { dir, home, .. } => format!(
        "bring the bucket copy home first: `git mv {} {}`, then re-run. If the bucket copy is obsolete, delete it instead. Nothing was written for this thread.",
        dir.display(),
        home.display()
      ),
      Self::Scan { .. } => "the tree could not be walked, so nothing was planned and nothing was touched. The cause above names the path -- check it is readable and re-run.".to_string(),
      Self::NothingToWrite { path } => format!(
        "run `intent sync --to-store`, which loads canon again and reads an opaque attachment's bytes from its sidecar under `intent/.canon/st/`; if that sidecar is missing, restore it from git first. Nothing was written to {}.",
        path.display()
      ),
      Self::Io { path, .. } => format!(
        "check that {} exists and is readable. This is a file `organize` had already decided about, so the tree moved or a permission changed between the plan and the act.",
        path.display()
      ),
    }
  }
}

fn io_err(path: &Path, source: std::io::Error) -> OrganizeError {
  OrganizeError::Io {
    path: path.to_path_buf(),
    source,
  }
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

/// The thread id owning `path`, or `None` for anything not under the estate root.
///
/// **THE FIRST COMPONENT UNDER THE ROOT, WHICH IS THE SAME FRAME
/// [`thread_relative`] DISCARDS.** That one throws the id away to get at the
/// file; this one throws the file away to get at the id. Kept as two functions
/// rather than one returning a pair, because every caller of each wants exactly
/// one half and a pair would have both of them destructuring past the part they
/// do not use.
fn thread_of(estate_root: &Path, path: &Path) -> Option<String> {
  let under = path.strip_prefix(estate_root).ok()?;
  let first = under.components().next()?;
  Some(first.as_os_str().to_string_lossy().into_owned())
}

/// Read the tree as [`plan`] needs it, and fingerprint it in the same pass.
///
/// **THE WALK AND THE DIGEST COME FROM ONE OBSERVATION, WHICH IS THE ONLY WAY
/// THE MOMENT-OF-ACT GUARD MEANS ANYTHING.** AC-04.5 compares the tree as
/// planned against the tree as found immediately before the irreversible step.
/// Two separate walks -- one for the listing, one for the hash -- would compare
/// a fingerprint of one moment against a plan built from another, so the guard
/// could pass while the thing it guards had already moved.
///
/// **It reuses `sync::scan` rather than walking the tree again.** That function
/// already owns what counts as an estate file: the gitignore-derived corpus,
/// `SKIPPED_DIRS`, the name-ordered deterministic walk, and the sha256 of every
/// file. A second walker here would be a fourth answer to "what is in this
/// estate", and the one that goes stale is always the one nobody is looking at
/// when a new ignore rule lands.
///
/// **The hash covers PATH AND CONTENT, not mtime.** A digest that moved when a
/// file was rewritten with identical bytes would refuse `organize`'s own quiet
/// second run -- an alarm that is always on, which is the defect AC-04.4 exists
/// to measure the absence of.
pub fn observe(
  project: &Project,
  previous: &[crate::sync::FileEntry],
) -> Result<(TreeState, String), OrganizeError> {
  let root = project.root();
  let entries =
    crate::sync::scan(root, previous).map_err(|source| OrganizeError::Scan { source })?;

  let mut present = BTreeSet::new();
  let mut sha256 = BTreeMap::new();
  // Sorted by construction: `scan` walks name-ordered, and both collections are
  // ordered maps. The digest below depends on that and must not depend on it
  // silently, so it re-derives its order from the map rather than the walk.
  for entry in &entries {
    let path = root.join(&entry.path);
    present.insert(path.clone());
    sha256.insert(path, entry.sha256.clone());
  }

  let mut hasher = <sha2::Sha256 as sha2::Digest>::new();
  for (path, sha) in &sha256 {
    sha2::Digest::update(&mut hasher, path.to_string_lossy().as_bytes());
    sha2::Digest::update(&mut hasher, b"\0");
    sha2::Digest::update(&mut hasher, sha.as_bytes());
    sha2::Digest::update(&mut hasher, b"\n");
  }
  let digest = sha2::Digest::finalize(hasher)
    .iter()
    .map(|b| format!("{b:02x}"))
    .collect::<String>();

  Ok((TreeState { present, sha256 }, digest))
}

/// Compute a plan without touching the tree.
///
/// `on_disk` is the listing -- passed in rather than walked here so the five
/// rows can be driven against a constructed population. The production caller
/// supplies a real walk.
pub fn plan(
  project: &Project,
  canon: &Canon,
  realised: &Realised,
  ctx: &RenderContext<'_>,
  tree: &TreeState,
  digest: String,
) -> Plan {
  // Everything the renderer can produce, keyed by path. This is BOTH the source
  // of hydration bytes and the denominator for "unclaimed" -- a path absent from
  // this map is one the renderer cannot make, which is the fifth row's exact
  // definition rather than a proxy for it.
  let renderable: BTreeMap<PathBuf, Vec<u8>> = views::render_all(project, canon, ctx)
    .into_iter()
    .map(|View { path, content }| (path, content.into_bytes()))
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
    if !realised.declares(&thread.id) {
      continue;
    }
    declared_paths.insert(project.info_view(&thread.id));
    declared_paths.insert(project.acceptance_view(&thread.id));
    for wp in &thread.wps {
      declared_paths.insert(project.wp_info_view(&thread.id, wp.seq));
    }
  }
  // **ISSUES JOIN BY THE SAME RULE AS THREADS (AC-01.2), WHICH IS WHY THIS IS A
  // SECOND LOOP AND NOT A SECOND RULE.** A declared issue with no file lands in
  // the hydrate row below; a realised file whose issue is undeclared falls
  // through to the dehydrate row, exactly as a closed thread's views do. The
  // OPEN/CLOSED decision is nowhere near here -- it was made when the verb
  // edited the manifest -- so this loop only asks what the manifest says.
  for issue in &canon.issues {
    if !realised.declares_artefact(
      crate::intentfiles::Sigil::Issue,
      &format!("{:04}", issue.number),
    ) {
      continue;
    }
    declared_paths.insert(project.issue_view(issue.number));
  }

  let present = &tree.present;
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

  // THE ATTACHMENT ARM. Attachments are AUTHORED ON DISK, so nothing renders
  // them and `renderable` says nothing about them -- the store carries a copy
  // plus the sha it had when it was carried. Three outcomes, and only one of them
  // is a write.
  let mut attachment_paths: BTreeSet<PathBuf> = BTreeSet::new();
  for thread in &canon.threads {
    let declared_thread = realised.declares(&thread.id);
    for att in &thread.attachments {
      let path = project.st_dir().join(&thread.id).join(&att.path);
      attachment_paths.insert(path.clone());
      if !declared_thread {
        // Undeclared and present: row four, through the same gate. The store
        // carries this file's bytes, so removing it is safe EXACTLY WHEN the
        // gate proves the copy matches -- which is why the attachment's bytes
        // travel on the step, as the view's rendered bytes do.
        if present.contains(&path) {
          steps.push(Step {
            path,
            action: Action::Dehydrate,
            // **THE STORE'S BYTES, WHICHEVER FORM THEY ARE CARRIED IN** (issue
            // 0338 (i)). This was `att.text`, so an opaque attachment reached
            // `gate` with nothing to compare against and its removal was refused
            // as unproven for good. `None` now means only that the bytes were
            // never loaded, and `gate` still refuses that: unproven is not
            // permission, and an empty comparison would pass for an empty file.
            content: att.as_bytes().map(<[u8]>::to_vec),
          });
        }
        continue;
      }
      if !present.contains(&path) {
        steps.push(Step {
          path,
          action: Action::HydrateAttachment,
          // The same bytes, the other direction (issue 0338 (i)). This was
          // `att.text`, and the write loop skipped the `None` an opaque
          // attachment produced while the report called it hydrated. `None` is
          // now only bytes never loaded, and `Plan::run` refuses that step by
          // path: never a zero-byte file, and never a `hydrated:` line over
          // nothing.
          content: att.as_bytes().map(<[u8]>::to_vec),
        });
        continue;
      }
      match tree.sha256.get(&path) {
        // Agrees with what the store carries: nothing to do, and saying so costs
        // a step that means "no action" in a list whose every other member means
        // one.
        Some(on_disk) if *on_disk == att.sha256 => {}
        // **A MISSING HASH IS TREATED AS DIVERGENCE, NOT AS AGREEMENT.** If the
        // caller could not hash the file, whether it matches the store is
        // UNANSWERED -- and reporting an unanswered question as agreement is how
        // a check comes to mean nothing. Reporting costs a line; the other
        // direction costs the file.
        _ => steps.push(Step {
          path,
          action: Action::AttachmentDiverged,
          content: None,
        }),
      }
    }
  }

  // Rows four and five, over what is actually on disk.
  for path in &tree.present {
    if declared_paths.contains(path) || attachment_paths.contains(path) {
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
    // **THE ISSUES ARM THE PARAGRAPH BELOW USED TO DEFER TO (AC-01.2).** A file
    // under `intent/issues/` that the renderer can produce, and that the
    // manifest does not declare, is row four for an issue exactly as a
    // generated view under a thread directory is row four for a thread.
    //
    // **MEMBERSHIP IS `renderable`, NOT THE DIRECTORY**, which is the whole
    // care needed here. `renderable` holds a view for EVERY issue in canon, so
    // a path in this directory that it does not know is a file no issue
    // renders -- somebody else's, and this verb removes nothing it cannot
    // regenerate. That keeps the rule identical to the thread side, where
    // `classify` answers `Unattached` rather than `GeneratedView` for the same
    // case.
    if path.starts_with(project.issues_view_dir()) {
      if renderable.contains_key(path) {
        steps.push(Step {
          path: path.clone(),
          action: Action::Dehydrate,
          content: renderable.get(path).cloned(),
        });
      } else {
        steps.push(Step {
          path: path.clone(),
          action: Action::Unclaimed,
          content: None,
        });
      }
      continue;
    }
    let Some(rel) = thread_relative(project, path) else {
      // Not under `st_dir` at all, and not an issue view either -- both of
      // those are decided above. A thread classifier would be a second, wrong,
      // spelling of whatever layout this path belongs to.
      continue;
    };
    match Project::classify(&rel) {
      // The renderer produces this shape and nothing declares it: row four.
      ThreadFile::GeneratedView => steps.push(Step {
        path: path.clone(),
        action: Action::Dehydrate,
        content: renderable.get(path).cloned(),
      }),
      // Canon is not a view and is not this verb's business.
      ThreadFile::Canon => {}
      // An attachment the STORE DOES NOT CARRY. The arm above decided every
      // attachment canon knows about, so reaching here means disk holds a
      // carryable file with no record -- which is an ingest question, not a
      // realisation one. Reported, never removed: it is the only copy.
      ThreadFile::Attachment => steps.push(Step {
        path: path.clone(),
        action: Action::Unclaimed,
        content: None,
      }),
    }
  }

  let held = held(project, canon, realised, present);
  steps.retain(|s| !held.iter().any(|h| s.path.starts_with(&h.home)));

  steps.sort_by(|a, b| a.path.cmp(&b.path));
  Plan {
    steps,
    leftovers: crate::legacy::leftovers(project, canon),
    digest,
    preconditions: preconditions::check(canon),
    estate_root: project.st_dir(),
    held,
  }
}

/// The declared threads that must not be realised yet (issue 0209).
///
/// **A DECLARED THREAD WITH NOTHING AT ITS OWN DIRECTORY AND FILES IN A v2
/// BUCKET IS HELD.** Only a FIRST realisation is held: a thread already present
/// at its home has no smaller copy left to write, and holding it would freeze a
/// working thread over a stray bucket.
///
/// **ONE PREDICATE, TWO DOORS.** [`plan`] asks it of the tree it observed;
/// `Facade::projection` asks it of [`presence`] on every write, because the
/// write after `st start` realised the thread silently -- driven, and the
/// likelier way Laksa's copy appeared than `organize` itself.
pub fn held(
  project: &Project,
  canon: &Canon,
  realised: &Realised,
  present: &BTreeSet<PathBuf>,
) -> Vec<Held> {
  canon
    .threads
    .iter()
    .filter(|t| realised.declares(&t.id))
    .filter(|t| {
      let home = project.thread_dir(&t.id);
      !present.iter().any(|p| p.starts_with(&home))
    })
    .filter_map(|t| v2_bucket_copy(project, present, &t.id))
    .collect()
}

/// The files [`held`] reads, for a caller with no observed tree: each declared
/// thread's home and its v2 bucket directories, and nothing else.
///
/// **NOT `sync::scan`**, which hashes the whole estate: `Facade::projection`
/// runs on every write, and the question needs a few directory listings.
pub fn presence(project: &Project, canon: &Canon, realised: &Realised) -> BTreeSet<PathBuf> {
  canon
    .threads
    .iter()
    .filter(|t| realised.declares(&t.id))
    .flat_map(|t| {
      std::iter::once(project.thread_dir(&t.id)).chain(
        crate::legacy::V2_STATUS_BUCKETS
          .iter()
          .map(|bucket| project.st_dir().join(bucket).join(&t.id)),
      )
    })
    .flat_map(|dir| crate::realise::walk(&dir))
    .collect()
}

/// The first v2 status bucket holding files for `id`, if any.
fn v2_bucket_copy(project: &Project, present: &BTreeSet<PathBuf>, id: &str) -> Option<Held> {
  crate::legacy::V2_STATUS_BUCKETS.iter().find_map(|bucket| {
    let dir = project.st_dir().join(bucket).join(id);
    let files = present.iter().filter(|p| p.starts_with(&dir)).count();
    (files > 0).then(|| Held {
      thread: id.to_string(),
      dir,
      home: project.thread_dir(id),
      files,
    })
  })
}

/// Remove the directories THIS RUN EMPTIED, and only those.
///
/// **`rmdir` SEMANTICS, NEVER A RECURSIVE DELETE.** [`std::fs::remove_dir`]
/// refuses a directory that still holds anything, so a directory carrying a
/// file this run did not plan to remove survives untouched. That refusal is the
/// whole safety argument, and it is the filesystem's rather than this
/// function's -- which is why the recursive variant must never be substituted
/// here, however similar the name.
///
/// **CANDIDATES ARE ANCESTORS OF REMOVED FILES, DEEPEST FIRST.** Only a
/// directory that held something this run deleted is a candidate, so a
/// directory that was already empty before the run is not touched -- the run
/// did not empty it, and removing it would be this verb doing something nobody
/// asked for. Deepest-first is what makes the cascade work without a second
/// pass: `ST0001/WP/01` is tried and removed before `ST0001/WP` is tried, so
/// the parent is genuinely empty by the time its turn comes.
///
/// **THE FLOOR IS THE ESTATE ROOT AND IT IS A BOUND, NOT A CONVENTION.** A
/// candidate that is not a strict descendant of `root` is skipped, so no
/// cascade can reach `intent/` or above however many levels it climbs.
///
/// # Why this exists
///
/// `organize` removed 423 files from this project's estate and left 54 empty
/// directory shells behind, because the only removal it performed was
/// `remove_file`. vc judged that harmless on the grounds git does not track an
/// empty directory; hv looked at the tree and counted 58 directories where 3
/// threads were declared. **Both readings are correct about different estates,
/// and the one a person opens is the one that matters.** A sweep with `rmdir`
/// cleared them once and would have recurred on the next dehydration, which is
/// what makes this a code change rather than a tidy-up.
/// **THE PREVIEW PREDICTS THE PRUNES RATHER THAN OMITTING THEM** (hv,
/// 2026-09-12: silent deletion). Until this took a `Mode`, pruning happened
/// only under `Mode::Apply`, so a preview reported `0 to prune` and the apply
/// that followed removed directories the plan had not named -- a removal
/// announced nowhere, inside the verb whose preview exists to announce them.
///
/// **ONE BODY WITH THE ACT WITHHELD, WHICH IS THIS MODULE'S OWN RULE** (see
/// [`Mode`]). The candidate set, the floor and the deepest-first order are
/// shared; only the question at the bottom differs -- `remove_dir` SUCCEEDING
/// is the act's test, and *every entry in it is already going* is the
/// preview's. A second function predicting prunes its own way would be a
/// promise about this one, and the two would drift in the direction nobody
/// notices.
fn prune_emptied(root: &Path, removed: &[PathBuf], pruned: &mut Vec<PathBuf>, mode: Mode) {
  // **DEDUPLICATED THROUGH A SET, NOT BY `dedup()` AFTER THE DEPTH SORT.**
  // `sort_by_key` on depth alone leaves equal paths merely at the same depth
  // rather than adjacent, and `Vec::dedup` only collapses NEIGHBOURS -- so the
  // obvious spelling silently keeps duplicates whenever two threads are pruned
  // in one run. It happens to be harmless here, because the second
  // `remove_dir` fails and contributes nothing, but a correctness argument that
  // rests on a later step failing is one that stops holding when the later step
  // changes.
  let unique: BTreeSet<PathBuf> = removed
    .iter()
    .flat_map(|p| {
      p.ancestors()
        .skip(1)
        .map(Path::to_path_buf)
        .collect::<Vec<_>>()
    })
    .filter(|d| d.starts_with(root) && d != root)
    .collect();
  let mut candidates: Vec<PathBuf> = unique.into_iter().collect();
  candidates.sort_by_key(|d| std::cmp::Reverse(d.components().count()));

  // Everything this run is taking away. The act reads it off the filesystem as
  // it goes; the preview has to carry it, because deepest-first order means a
  // directory's fate can depend on a CHILD DIRECTORY decided moments earlier.
  let mut going: BTreeSet<PathBuf> = removed.iter().cloned().collect();
  for dir in candidates {
    let goes = if mode.performs() {
      std::fs::remove_dir(&dir).is_ok()
    } else {
      would_be_emptied(&dir, &going)
    };
    if goes {
      going.insert(dir.clone());
      pruned.push(dir);
    }
  }
}

/// Would this directory be empty once `going` has gone?
///
/// **UNREADABLE MEANS NO.** A directory this process cannot list is one it
/// cannot predict, and answering *yes* would put a path in a preview's removal
/// list that the act may well leave alone -- naming a file that is not going is
/// its own kind of false report, and the conservative direction here is the one
/// that under-promises.
fn would_be_emptied(dir: &Path, going: &BTreeSet<PathBuf>) -> bool {
  let Ok(entries) = std::fs::read_dir(dir) else {
    return false;
  };
  let mut any = false;
  for entry in entries {
    let Ok(entry) = entry else {
      return false;
    };
    any = true;
    if !going.contains(&entry.path()) {
      return false;
    }
  }
  // An ALREADY-empty directory is not this run's prune to claim. `remove_dir`
  // would take it, and it is not a consequence of anything removed here.
  any
}

/// Which directories a removal set WOULD empty, without emptying anything.
///
/// **EXPOSED SO A CALLER CAN NAME THEM BEFORE IT ACTS** (hv, 2026-09-12: silent
/// deletion). `st dehydrate` removes files and then prunes what they emptied,
/// and a caller announcing only the files would announce less than the run
/// takes. It is the same body the act uses, asked in the mode that withholds
/// the act -- see [`prune_emptied`].
pub fn prunes_for(root: &Path, removed: &[PathBuf]) -> Vec<PathBuf> {
  let mut out = Vec::new();
  prune_emptied(root, removed, &mut out, Mode::Preview);
  out
}

/// The reason an index view is kept. Exposed so the report can print it rather
/// than restate it.
pub fn exempt_reason() -> &'static str {
  EXEMPT_REASON
}

/// What a run actually did.
///
/// **`unchanged` IS A FIELD AND NOT AN OMISSION.** AC-04.4 measures idempotence
/// as the count of files whose mtime moved, so a second run has to be able to say
/// "I considered these and touched none of them". A report listing only what it
/// wrote cannot distinguish a correctly quiet run from one that examined nothing.
#[derive(Debug, Default)]
pub struct Report {
  pub hydrated: Vec<PathBuf>,
  pub rewritten: Vec<PathBuf>,
  pub unchanged: Vec<PathBuf>,
  pub dehydrated: Vec<PathBuf>,
  pub unclaimed: Vec<PathBuf>,
  pub diverged: Vec<PathBuf>,
  /// Removals the gate refused. **Reported, and the run continues** -- the
  /// criterion refuses the REMOVAL, not the reconciliation. Aborting everything
  /// over one hand-edited file would make every other thread's realisation
  /// hostage to an edit nobody has read yet.
  pub refused: Vec<OrganizeError>,
  /// v2 leftovers this run removed -- the status buckets, the v2 issue estate
  /// and the retired `.treeindex` cache, once the store was proved to hold
  /// every one of their files (WP-02, AC-02.2).
  ///
  /// **A SEPARATE FIELD FROM `dehydrated`, because the two are removed on
  /// different PROOFS.** A view is removed because the renderer reproduced it
  /// byte for byte; a leftover is removed because canon carries its content at
  /// its sha. Folding them together would let one proof be reported as the
  /// other.
  pub pruned_legacy: Vec<PathBuf>,
  /// Authored files naming a v2 bucket path (AC-02.4). **Reported, never
  /// rewritten**, and never a reason to refuse anything.
  pub legacy_pointers: Vec<crate::legacy::Pointer>,
  /// Directories this run emptied and then removed.
  ///
  /// **REPORTED BECAUSE IT IS DESTRUCTIVE.** Removing a directory is a smaller
  /// act than removing a file and it is not a smaller KIND of act, and every
  /// other removal on this report is named. A prune that happened silently
  /// would be the one line of this verb an operator could not review.
  pub pruned: Vec<PathBuf>,
  /// The tree digest of the plan this report came from.
  ///
  /// **IT IS HERE SO A CALLER CAN PIN A LATER RUN TO THE PLAN IT SHOWED A
  /// HUMAN** (hv, 2026-09-12: silent deletion). A preview and the `--apply`
  /// that follows it are two runs, and between them the estate can move --
  /// every read verb materialises the store on access, so a peer running
  /// `intent st list` is enough. Without this the second run re-plans and acts
  /// on a plan NOBODY WAS SHOWN, which is the defect with the preview's
  /// reassurance on top of it. See [`Plan::digest`] for what the value measures.
  pub digest: String,
}

impl Report {
  /// How many removals the ship gate is holding back.
  ///
  /// **THE REPORT ANSWERS THIS, NOT THE RENDERER**, because it is a property of
  /// what happened and every face that shows a run has the same question to
  /// answer. A fold living inline in one renderer is a fold the next one gets
  /// subtly different (IN-AG-HIGHLANDER-001).
  ///
  /// **IT IS NOT `refused.len()`, AND THE GAP BETWEEN THE TWO IS THE WHOLE
  /// REASON THIS EXISTS.** `PreconditionsUnmet` is deliberately ONE refusal for
  /// the entire run rather than one per file -- the unmet precondition is a
  /// property of the estate, so N copies of an identical sentence would bury the
  /// per-file refusals that ARE about their file. The consequence is that a gate
  /// holding four hundred removals counts as `1`. Accurate, and three orders of
  /// magnitude too small for the only question a reader is asking.
  ///
  /// Per-file refusals contribute nothing here: they refused a removal that was
  /// individually gated, which the caller sees named, one line each.
  /// A digest over the SORTED unclaimed set -- membership, not size.
  ///
  /// **THE COUNT AND THIS ANSWER DIFFERENT QUESTIONS, AND ONLY TOGETHER DO THEY
  /// COVER THE SET.** Measured by vc on the live tree: ADDING an unclaimed file
  /// moves `199 unclaimed` to `200` and is already visible; SWAPPING one file
  /// for another inside one directory leaves the summary BYTE-IDENTICAL, with
  /// the changed entry at position 2 of 199. **The defect is constant
  /// cardinality**, and a count cannot see it by construction.
  ///
  /// **THIS IS WHY A DIRECTORY BREAKDOWN IS NOT ENOUGH EITHER.** Grouping 199
  /// paths into their directories with per-directory counts was the first fix
  /// and it fails the same swap: same directory, same count, same output. Both
  /// quantities a grouped report carries are exactly the two a same-directory
  /// swap preserves.
  ///
  /// Truncated to twelve hex characters, which is a deliberate choice and not a
  /// habit: this is a CHANGE DETECTOR for a human reading one line, not an
  /// identity anyone pins against, so collision resistance past "did this set
  /// move" buys nothing and costs the readability the whole change is for.
  pub fn unclaimed_digest(&self) -> String {
    let mut sorted: Vec<String> = self
      .unclaimed
      .iter()
      .map(|p| p.to_string_lossy().to_string())
      .collect();
    // **SORTED HERE RATHER THAN TRUSTED FROM THE WALK.** The digest must answer
    // *is this the same SET*, so a report whose paths arrived in a different
    // order must hash the same -- otherwise the detector fires on the walk and
    // not on the estate, and a reader learns to ignore it within a day.
    sorted.sort();
    let mut hasher = <sha2::Sha256 as sha2::Digest>::new();
    for path in &sorted {
      sha2::Digest::update(&mut hasher, path.as_bytes());
      sha2::Digest::update(&mut hasher, b"\n");
    }
    format!("{:x}", sha2::Digest::finalize(hasher))[..12].to_string()
  }

  pub fn blocked(&self) -> usize {
    self
      .refused
      .iter()
      .map(|refusal| match refusal {
        OrganizeError::PreconditionsUnmet { removals, .. } => *removals,
        _ => 0,
      })
      .sum()
  }

  /// The v2 prune refusal as ONE class for the whole run, or `None` when the
  /// prune withheld nothing (issue 0318).
  ///
  /// **THE FOLD BELONGS TO THE REPORT, FOR `blocked`'s REASON.** `refused` keeps
  /// one `LegacyUnheld` per file, so the summary count and every face that lists
  /// refusals per path are unchanged; what a terminal prints once is a question
  /// about the run, and a fold living inline in one renderer is the one the next
  /// renderer gets subtly different.
  pub fn legacy_unheld(&self) -> Option<LegacyUnheldClass<'_>> {
    let files: Vec<(&std::path::Path, &str)> = self
      .refused
      .iter()
      .filter_map(|refusal| match refusal {
        OrganizeError::LegacyUnheld { path, reason } => Some((path.as_path(), reason.as_str())),
        _ => None,
      })
      .collect();
    (!files.is_empty()).then_some(LegacyUnheldClass { files })
  }

  /// Every refusal that is not folded into [`Report::legacy_unheld`]'s class,
  /// in report order, for a face that prints the class once and the rest one
  /// by one.
  pub fn refused_outside_classes(&self) -> impl Iterator<Item = &OrganizeError> {
    self
      .refused
      .iter()
      .filter(|refusal| !matches!(refusal, OrganizeError::LegacyUnheld { .. }))
  }
}

/// Every v2 file one run's prune withheld, as one refusal (issue 0318).
///
/// **hv met the per-file form on Laksa**: a PREVIEW printed an `error:` line per
/// unheld file, each carrying the same remedy, so a run that changed nothing
/// read as a run that failed, over and over, until the lines that mattered had
/// scrolled away. The cause is one fact about the estate -- the prune is all or
/// nothing (AC-02.2) -- so it is said once, with its count and one remedy.
#[derive(Debug)]
pub struct LegacyUnheldClass<'a> {
  /// `(path, reason)` for each withheld file, in report order. **Named, never
  /// dropped**: a face that does not list them says that it narrowed.
  pub files: Vec<(&'a std::path::Path, &'a str)>,
}

impl LegacyUnheldClass<'_> {
  /// The class line, in the tense of the run.
  ///
  /// **A PREVIEW IS NOT AN `error:`.** It refused nothing; it is saying what the
  /// apply would refuse, and an error from a run that did what it was asked is
  /// the always-on alarm this verb's exit code already declines to raise. The
  /// apply did refuse, so it says so in the past tense, as an error.
  pub fn line(&self, performed: bool) -> String {
    let files = self.files.len();
    match performed {
      true => format!(
        "{}refused to prune the v2 tree: the store does not hold {files} file(s) under it, so this run removed none of the v2 tree",
        crate::remedy::ERROR_PREFIX
      ),
      false => format!(
        "would refuse to prune the v2 tree: the store does not hold {files} file(s) under it, so `intent organize --apply` would remove none of the v2 tree"
      ),
    }
  }

  /// The class's one remedy line.
  pub fn remedy_line(&self) -> String {
    format!("{}{LEGACY_UNHELD_REMEDY}", crate::remedy::REMEDY_PREFIX)
  }
}

impl Plan {
  /// Apply this plan.
  ///
  /// **REMOVALS HAPPEN FIRST, AND THE ORDER IS THE WHOLE CORRECTNESS OF THE
  /// DIGEST GUARD.** AC-04.5 wants the digest re-computed immediately before the
  /// irreversible step. Writing first and re-digesting after would compare a tree
  /// this function had just changed against the one it measured, so the guard
  /// would fire on `organize`'s OWN writes -- an alarm that is always on, which is
  /// the always-set-marker defect wearing different clothes. Removals are the
  /// irreversible half, and doing them while the tree still matches what was
  /// planned is what lets the guard mean "somebody else wrote here".
  ///
  /// `digest_now` is supplied by the caller rather than computed here, so the
  /// guard can be driven without racing a real process against a test. In
  /// [`Mode::Preview`] it is never called -- there is no moment of acting to
  /// stand in front of.
  ///
  /// **`Mode::Preview` CLASSIFIES EXACTLY AS `Mode::Apply` DOES, INCLUDING THE
  /// PER-FILE GATE.** A preview that skipped `gate` would report 544 removals
  /// where the run would perform 517 and refuse 27, which is the one number the
  /// operator is consulting the preview for.
  pub fn run(&self, mode: Mode, digest_now: &dyn Fn() -> String) -> Result<Report, OrganizeError> {
    // **THE REPORT SAYS WHICH PLAN IT IS OF**, so a caller that rendered one can
    // pin the act to it. See [`Report::digest`].
    let mut report = Report {
      digest: self.digest.clone(),
      ..Report::default()
    };

    // **REPORTED IN BOTH MODES, AND THE RUN CONTINUES** (issue 0209), for the
    // reason `refused` gives: one held thread must not make every other
    // thread's realisation hostage to it. On `--apply` a refusal moves the exit
    // code, so a script does not carry on believing the thread is realised.
    for h in &self.held {
      report.refused.push(OrganizeError::LegacyCopyPresent {
        thread: h.thread.clone(),
        dir: h.dir.clone(),
        home: h.home.clone(),
        files: h.files,
      });
    }

    // **GUARDED ONLY WHEN THERE IS SOMETHING IRREVERSIBLE TO GUARD.** A plan that
    // removes nothing has no step worth refusing over, and refusing a pure
    // hydration because a peer touched an unrelated file would train operators to
    // re-run until it passes -- which is how a guard stops being one.
    // **THE SHIP GATE IS CONSULTED BEFORE THE DIGEST GUARD, AND THE ORDER IS
    // NOT COSMETIC.** If AC-00.1 refuses, this run removes nothing, so there is
    // no irreversible step for the digest to protect -- and `TreeMoved` is a
    // hard `Err` that would abort the hydration half too. A run that is already
    // forbidden from removing anything must not also lose its safe work to a
    // guard standing over a step it is not going to take.
    //
    // **THE NAME IS `removals_permitted` RATHER THAN `will_remove`, BECAUSE A
    // PREVIEW ENTERS THE SAME BRANCH AND REMOVES NOTHING.** The condition is a
    // property of the ESTATE -- is there anything to remove, and does the ship
    // gate allow it -- and it has to be answered identically in both modes, or
    // the preview reports on a different question than the one the run asks.
    // `Mode` is consulted separately, at each of the three points that actually
    // touch the world.
    let removals_permitted = self.is_destructive() && self.preconditions.permits();

    if self.is_destructive() && !self.preconditions.permits() {
      // ONE refusal for the whole run, not one per file. The unmet precondition
      // is a property of the estate, so N copies of an identical sentence would
      // bury the per-file refusals that ARE about their file.
      // **DERIVED FROM THE HELD STEPS THEMSELVES, NOT FROM THE DECLARATION.**
      // The question the operator is asking is which threads lost their
      // removal, and only the steps know that. A `BTreeSet` because the same
      // thread contributes one step per file and the refusal wants threads.
      let threads: std::collections::BTreeSet<String> = self
        .with(Action::Dehydrate)
        .filter_map(|s| thread_of(&self.estate_root, &s.path))
        .collect();
      report.refused.push(OrganizeError::PreconditionsUnmet {
        removals: self.with(Action::Dehydrate).count(),
        verdict: self.preconditions.clone(),
        threads: threads.into_iter().collect(),
      });
    }

    // **THE RE-OBSERVATION GUARD STANDS ON THE APPLY PATH AND NOWHERE ELSE.**
    // `TreeMoved` is about the moment of acting -- it asks whether the tree is
    // still the one that was planned against, immediately before the step that
    // cannot be taken back. A preview takes no such step, so there is nothing
    // for it to guard, and refusing a preview because a peer wrote a file would
    // deny the operator the one reading that is always safe to take.
    if removals_permitted && mode.performs() {
      let now = digest_now();
      if now != self.digest {
        return Err(OrganizeError::TreeMoved {
          detail: format!("planned against {}, found {}", self.digest, now),
        });
      }
    }

    if removals_permitted {
      // Issue 0343: each removal was gated alone, so a thread whose cover was refused lost its attachments and kept its views.
      // Every step is gated first, and one refusal withholds the rest of its thread; an issue view stays per file.
      let gated: Vec<(&Step, Result<(), OrganizeError>)> = self
        .with(Action::Dehydrate)
        .map(|step| (step, gate(step)))
        .collect();
      let refused_threads: BTreeSet<String> = gated
        .iter()
        .filter(|(_, verdict)| verdict.is_err())
        .filter_map(|(step, _)| thread_of(&self.estate_root, &step.path))
        .collect();
      let mut withheld: BTreeMap<String, Vec<PathBuf>> = BTreeMap::new();
      for (step, verdict) in gated {
        match verdict {
          Err(refusal) => report.refused.push(refusal),
          Ok(()) => {
            match thread_of(&self.estate_root, &step.path).filter(|t| refused_threads.contains(t)) {
              Some(thread) => withheld.entry(thread).or_default().push(step.path.clone()),
              None => {
                if mode.performs() {
                  std::fs::remove_file(&step.path).map_err(|e| io_err(&step.path, e))?;
                }
                report.dehydrated.push(step.path.clone());
              }
            }
          }
        }
      }
      for (thread, withheld) in withheld {
        report
          .refused
          .push(OrganizeError::ThreadWithheld { thread, withheld });
      }
      prune_emptied(
        &self.estate_root,
        &report.dehydrated,
        &mut report.pruned,
        mode,
      );

      // **THE v2 PRUNE, AND ITS PROOF IS THE INGEST RATHER THAN A RE-RENDER.**
      // `gate` above asks the renderer to reproduce a view; nothing renders a
      // v2 bucket file, so the question `leftovers` already answered -- does
      // canon carry this file's content, at its sha -- IS the gate here.
      //
      // **ALL OR NOTHING** (AC-02.2): a prune that ingested nothing removes
      // nothing, so one unheld path refuses every removal and names every one
      // of them.
      if self.leftovers.refuses() {
        for withheld in &self.leftovers.withheld {
          report.refused.push(OrganizeError::LegacyUnheld {
            path: withheld.path.clone(),
            reason: withheld.reason.clone(),
          });
        }
      } else {
        for path in &self.leftovers.removable {
          if mode.performs() {
            std::fs::remove_file(path).map_err(|e| io_err(path, e))?;
          }
          report.pruned_legacy.push(path.clone());
        }
        prune_emptied(
          &self.estate_root,
          &report.pruned_legacy,
          &mut report.pruned,
          mode,
        );
      }
    }
    // **REPORTED WHATEVER THE PRUNE DID, INCLUDING WHEN IT REFUSED** (AC-02.4).
    // A worklist of authored files naming a bucket path is for a human to read
    // and act on in their own words; it is not a precondition of anything and
    // nothing here rewrites one.
    report.legacy_pointers.clone_from(&self.leftovers.pointers);

    // Every write goes through ONE `WriteSet`, which is where the
    // skip-when-unchanged already lives. Deciding here which files "need"
    // writing would be a second such guard, and the first one to be written
    // beside the real path reached nothing at all.
    let mut set = WriteSet::new();
    for step in &self.steps {
      let writes = matches!(step.action, Action::Hydrate | Action::HydrateAttachment);
      if !writes && step.action != Action::Verify {
        continue;
      }
      let Some(content) = &step.content else {
        // **A STEP THAT MUST WRITE AND HOLDS NO BYTES IS REFUSED BY PATH**
        // (issue 0338 (i)). It was skipped here and then reported `hydrated`
        // below, so `organize --apply` and `st hydrate` said an opaque attachment
        // was realised and wrote nothing. A `Verify` with no render writes
        // nothing and is reported as nothing, as before; it is not this case.
        if writes {
          report.refused.push(OrganizeError::NothingToWrite {
            path: step.path.clone(),
          });
        }
        continue;
      };
      // **A `Verify` IS CLASSIFIED HERE, BEFORE THE WRITE, AND THE ORDER IS THE
      // WHOLE MEASUREMENT.** The first version read the file AFTER the commit and
      // asked whether it matched the render -- by which point every file matches,
      // because the commit had just made it so. Every rewrite reported itself as
      // unchanged, and AC-04.4 is measured on exactly that distinction. Caught by
      // the positive control in AT-04.4, not by review: the quiet arm was green
      // and the arm that MUST see movement was the one that failed.
      if step.action == Action::Verify {
        match std::fs::read(&step.path) {
          Ok(disk) if disk == *content => report.unchanged.push(step.path.clone()),
          _ => report.rewritten.push(step.path.clone()),
        }
      }
      set.add_bytes(step.path.clone(), content.clone());
    }
    if !set.is_empty() && mode.performs() {
      set
        .commit()
        .map_err(|e| OrganizeError::Io {
          path: PathBuf::from("<write set>"),
          source: std::io::Error::other(e),
        })?
        .keep();
    }

    for step in &self.steps {
      match step.action {
        // Only what was WRITTEN: a write step with no bytes was refused above,
        // by path, and listing it here as well is the defect 0338 (i) found.
        Action::Hydrate | Action::HydrateAttachment if step.content.is_some() => {
          report.hydrated.push(step.path.clone())
        }
        Action::Hydrate | Action::HydrateAttachment => {}
        Action::Unclaimed => report.unclaimed.push(step.path.clone()),
        Action::AttachmentDiverged => report.diverged.push(step.path.clone()),
        // `Verify` was classified above, against the bytes as they were BEFORE
        // the write. Doing it here as well would overwrite a true answer with a
        // tautological one.
        Action::Verify | Action::Dehydrate | Action::Exempt => {}
      }
    }

    Ok(report)
  }
}

/// The dehydration gate (AC-04.2).
///
/// Re-render into memory, compare to the bytes on disk, refuse on any difference
/// but the footer's version, and name the path.
///
/// **THE FOOTER'S VERSION IS THE ONE DIFFERENCE THAT CARRIES NO HAND EDIT**
/// (issue 0316, vc's ruling 2026-09-14). A view an earlier Intent rendered
/// differs from today's render in the version its banner names and nowhere
/// else, and refusing it left a closed, undeclared thread's views with no
/// owner: nothing re-rendered them and nothing removed them. The question is
/// asked through [`crate::views::differs_only_in_banner_version`], the one
/// predicate doctor (0309) and the overwrite check (0385) already ask. **Fail-safe by construction rather than by discipline:**
/// the only way to remove a view is to have proved first that the store can
/// reproduce it exactly, so a hand edit cannot be destroyed by an operator who
/// forgot to check.
///
/// A view the renderer cannot reproduce at all is refused too, on the same
/// ground: if there are no bytes to compare against, the claim "the store
/// carries this" is unproven, and unproven is not permission.
pub fn gate(step: &Step) -> Result<(), OrganizeError> {
  debug_assert!(step.action.is_destructive(), "gate is for removals only");
  // **READ AS BYTES** (issue 0338 (i)). `read_to_string` failed a non-UTF-8
  // working copy as an I/O error before any comparison, which told the operator
  // the file could not be read when the question was whether it matched.
  let on_disk = std::fs::read(&step.path).map_err(|e| io_err(&step.path, e))?;
  match &step.content {
    Some(carried) if *carried == on_disk => Ok(()),
    Some(carried) if only_the_banner_moved(&on_disk, carried) => Ok(()),
    _ => Err(OrganizeError::HandEdited {
      path: step.path.clone(),
      bytes: on_disk.len(),
    }),
  }
}

/// Whether two byte strings are one view but for the version its banner names.
/// Asked only where both sides are text, since a banner is a line of prose.
fn only_the_banner_moved(on_disk: &[u8], carried: &[u8]) -> bool {
  match (std::str::from_utf8(on_disk), std::str::from_utf8(carried)) {
    (Ok(disk), Ok(rendered)) => crate::views::differs_only_in_banner_version(disk, rendered),
    _ => false,
  }
}

#[cfg(test)]
mod prune_floor {
  //! **THE FLOOR IS EXERCISED HERE AND NOT IN THE INTEGRATION TEST, BECAUSE THE
  //! INTEGRATION TEST CANNOT REACH IT.**
  //!
  //! `organize_prunes_what_it_emptied.rs` asserts the estate root survives a run
  //! that emptied everything under it, and that assertion **passed with the
  //! floor deleted** -- measured, as a surviving mutation arm. The reason is
  //! that a real estate root always holds `steel_threads.md`, so `remove_dir`
  //! refuses it whatever the candidate filter says. **The root was protected by
  //! the index happening to live there, not by the bound this code claims to
  //! have**, and a test that cannot tell those apart is not testing the bound.
  //!
  //! So the floor is driven directly, against a root that IS empty and would
  //! therefore be removed by any code that considered it a candidate.

  use super::{Mode, prune_emptied};
  use std::path::PathBuf;

  /// A root holding nothing but one prunable child. Without the `d != root`
  /// filter, the cascade reaches the root and removes it.
  #[test]
  fn an_empty_estate_root_is_never_a_candidate() {
    let tmp = std::env::temp_dir().join(format!("intent-prune-floor-{}", std::process::id()));
    let root = tmp.join("st");
    let child = root.join("ST0001");
    std::fs::create_dir_all(&child).expect("lay out the fixture");
    let file = child.join("info.md");
    std::fs::write(&file, "realised").expect("write");
    std::fs::remove_file(&file).expect("the run removed it");

    let mut pruned: Vec<PathBuf> = Vec::new();
    prune_emptied(&root, std::slice::from_ref(&file), &mut pruned, Mode::Apply);

    assert!(
      !child.exists(),
      "precondition of the real property: the emptied child IS pruned"
    );
    assert!(
      root.is_dir(),
      "AND THE ROOT SURVIVES with nothing left in it. This is the assertion the \n       \
       integration test could not make: here the root is genuinely empty, so \n       \
       `remove_dir` would succeed on it and only the declared floor stops the \n       \
       cascade. A bound that is never reached is not a bound that was tested."
    );
    assert!(
      !pruned.iter().any(|p| p == &root),
      "and the root is not reported as pruned. pruned: {pruned:?}"
    );

    std::fs::remove_dir_all(&tmp).ok();
  }
}
