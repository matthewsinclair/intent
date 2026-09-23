//! The intentsvcs facade -- the one surface every skin calls (design.md D06).
//!
//! The clap layer, the GraphQL layer and the MCP layer are all thin
//! coordinators over this: parse, call, render. Nothing above this module
//! touches the DB or the file canon, which is what makes the two entry skins
//! incapable of drifting apart.
//!
//! **The DB is the mutation; the files are its projection.** D01 was REVERSED
//! by hv on 2026-08-15 -- the DB is the SSOT and the files are re-creatable --
//! and the order here follows: one transaction writes the entities, the prose
//! index and the event envelope together, and only then are the files
//! rewritten. If the transaction fails, nothing was written anywhere. If the
//! FILE write fails, the change is already safe, the batch unwinds so the tree
//! is left STALE BUT CONSISTENT rather than half-applied, and the failure is
//! reported through its own error variant that leads with what succeeded.
//!
//! It used to be the other way round, and under the old model that was
//! correct: canon was durable, the DB was rebuildable from it, so files landed
//! first and a DB failure rolled them back. The reversal put the recoverable
//! half second, where it belongs.
//!
//! **Sync has BOTH directions now, and they are not interchangeable** --
//! [`Facade::sync_to_disk`] rewrites the extract from truth and is the repair
//! for a stale tree; [`Facade::sync_from_disk`] replaces truth from the extract
//! and is a RESTORE that loses anything newer. [`Facade::sync_overwrite`]
//! prices the second one before it is paid. The paragraph that stood here said
//! the db -> disk direction did not exist, which was true when it was written
//! and stopped being true the same day AC-03.9 landed -- so it is recorded as
//! the second instance of a doc outliving its own subject, alongside the three
//! remedies that named a command after the reasoning behind it had moved.
//!
//! **The facade has no clock.** Dates arrive from the caller in
//! `FacadeContext::today`. That is not the renderer's no-clock law (D23) --
//! a mutation genuinely happens at a time -- but it keeps every verb a pure
//! function of its inputs, which is what makes them testable without freezing
//! time. The event log is the one place a real timestamp is minted, because an
//! event log that did not record when things happened would not be one.

use serde_json::{Value, json};

use crate::address::{Address, Entity as AddrEntity, Format as AddrFormat};
use crate::contract::{self, Scope, Verdict};
// **Aliased because this crate has two `Scope` types and they are different
// questions.** `contract::Scope` selects an AC GROUP WITHIN one thread
// (`Thread` or `WorkPackage(seq)`); `sync::Scope` selects WHICH THREADS a sync
// takes from its source. Importing the second bare would shadow nothing and
// resolve silently to the first -- which is exactly what happened while this
// was being written, and the compiler caught it only because the two have no
// methods in common. Two types one word apart deserve the alias at the seam
// rather than a reader inferring which is meant.
use crate::event::{Envelope, Subject};
use crate::export::{self, ExportRefusal};
use crate::ingest::{self, Canon, IngestError};
use crate::intentfiles::{Realised, Sigil};
use crate::model::{
  AcKind, AcState, AcceptanceTest, AtKind, AtStatus, Attachment, Board, Criterion, Issue,
  IssueStatus, TShirt, Thread, ThreadStatus, WbItemKind, WorkPackage, WpStatus, to_canonical_json,
};
use crate::project::{EditDisposition, Migration, Pending, Project, ThreadFile};
use crate::realise;
use crate::store::{Store, StoreError};
use crate::sync::Scope as SyncScope;
use crate::transitions;
use crate::views::{self, RenderContext};
use crate::write_set::{WriteError, WriteSet};
use crate::{intentfiles, organize};

/// How a door opens the store: the ordinary way, or the way `intent index
/// rebuild` must when the search index itself may be what cannot be read
/// (issue 0453).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opening {
  /// Load the model through the store's ordinary reads.
  Ordinary,
  /// Drop and recreate the two derived search-index tables before the first
  /// read of them, then put the model's prose back. Every other table is read
  /// first and never dropped; see [`Store::recreate_index_tables`].
  RepairingIndex,
}

/// Ambient facts a facade call runs with. Explicit rather than discovered, so
/// a verb's result is a function of its arguments.
#[derive(Debug, Clone)]
pub struct FacadeContext {
  /// Who is acting, which every event records: the author
  /// [`FacadeContext::for_project`] resolves (ST0078 P1).
  pub principal: String,
  /// The project's UUID (D15). Stamped at migration; empty on a pre-migration
  /// project, which the event log records honestly rather than inventing one.
  pub project_id: String,
  /// The Intent version, for generated banners.
  pub version: String,
}

impl FacadeContext {
  /// The context a verb runs with on `project`, its principal the author.
  ///
  /// **ONE CONSTRUCTOR FOR EVERY DOOR THAT ACTS FOR A PERSON** -- the CLI --
  /// so the rule for who an event names has one home. The rule itself is
  /// [`crate::event::author`]: git's `user.name` and `user.email`, else the
  /// project's configured `author`, else `local`. The daemon does not use it:
  /// its own writes are ingests, and its events name the mechanism.
  pub fn for_project(project: &Project, version: &str) -> Self {
    Self {
      principal: resolve_author(project),
      project_id: project.config().project_id.clone().unwrap_or_default(),
      version: version.to_string(),
    }
  }
}

/// The author of this project's acts: git's identity at its root, else its
/// config's author.
pub fn resolve_author(project: &Project) -> String {
  author_at(project.root(), &project.config().author)
}

/// [`crate::event::author`] for a tree at `root` whose config names
/// `configured`: the one reading both the CLI's context and `init` use.
///
/// **One git process, reading both keys.** Git comes first in the rule, so it
/// is asked on every verb; a single `--get-regexp` keeps that to one spawn. A
/// git that is absent, or a repository with no identity, is the absent
/// identity the rule falls back through -- the fallback is the ruled answer for
/// that case, not a swallowed failure.
pub(crate) fn author_at(root: &std::path::Path, configured: &str) -> String {
  let identity = std::process::Command::new("git")
    .arg("-C")
    .arg(root)
    .args(["config", "--get-regexp", r"^user\.(name|email)$"])
    .output()
    .ok()
    .filter(|out| out.status.success())
    .and_then(|out| String::from_utf8(out.stdout).ok())
    .unwrap_or_default();
  let value = |key: &str| {
    identity
      .lines()
      .filter_map(|line| line.split_once(' '))
      .filter(|(k, _)| *k == key)
      .map(|(_, v)| v.to_string())
      .next_back()
  };
  crate::event::author(
    value("user.name").as_deref(),
    value("user.email").as_deref(),
    configured,
  )
}

/// What a completed migration did, for the door to report.
///
/// **`already_migrated` is a count of threads whose SOURCE changed, not of work
/// skipped.** Those threads are in the plan like any other and re-emit
/// byte-identical canon and views, so they stay inside every conservation
/// denominator; what the number says is how much canon a previous run had
/// already produced. On a first run it is zero, and on a re-run after an
/// interruption it is exactly how far the interrupted run got.
#[derive(Debug)]
pub struct Upgraded {
  pub threads: usize,
  pub issues: usize,
  /// Files written -- canon plus generated views.
  pub files: usize,
  /// Phase A's carried findings: reported so the counts reconcile, never so
  /// that anyone acts on them.
  pub carried: Vec<crate::finding::Finding>,
  /// Thread ids read from committed canon rather than converted from markdown.
  pub already_migrated: Vec<String>,
  /// Issue numbers read from committed canon rather than converted from a v2
  /// estate. **Zero on a first run, and on a re-run it is every issue in the
  /// project** -- because a re-run's v2 estate is empty and the whole issue
  /// population comes back through the union (intent#0070).
  pub already_migrated_issues: Vec<u32>,
  /// Sections dropped as template scaffolding, one record each, so a declared
  /// drop can be reconciled against the estate's census rather than inferred
  /// from a total that happens to be short.
  pub dispositions: Vec<crate::legacy::Disposition>,
  /// The v2 leftovers this conversion removed (WP-02, AC-02.2), named rather
  /// than counted.
  ///
  /// **A DESTRUCTIVE ACT THAT REPORTS ONLY A TOTAL IS ONE NOBODY CAN REVIEW.**
  /// Empty when the prune refused, and then [`Upgraded::prune_withheld`] says
  /// which file stopped it -- and empty when it was deferred, and then
  /// [`Upgraded::prune_deferred`] names what stays.
  pub pruned: Vec<std::path::PathBuf>,
  /// v2 bucket files this run ingested into an already-migrated thread's canon
  /// (issue 0319), named rather than counted.
  pub ingested: Vec<std::path::PathBuf>,
  /// Bucket files this run did not ingest, each with its reason. A report and
  /// never a refusal: none of them stops the run.
  pub not_ingested: Vec<crate::legacy::Withheld>,
  /// v2 leftovers the store now holds and this run did NOT remove, because it
  /// ingested some of them.
  ///
  /// **AN INGEST AND A REMOVAL ARE NOT ONE RUN** (vc, 2026-09-13, on hv's
  /// no-silent-deletion ruling of 2026-09-12). Pruning straight after the
  /// ingest would delete files whose only reviewable record is the line saying
  /// they were just carried, and the operator would read both after the fact.
  /// Removal stays with `intent organize`, which names every path before it
  /// goes. A conversion, and a re-run that ingested nothing, prune as before.
  pub prune_deferred: Vec<std::path::PathBuf>,
  /// Why the prune removed nothing: one entry per file the store does not hold.
  /// Empty on a conversion that pruned.
  pub prune_withheld: Vec<crate::legacy::Withheld>,
  /// Authored files naming a v2 bucket path -- a worklist, never a refusal, and
  /// never rewritten (AC-02.4).
  pub pointers: Vec<crate::legacy::Pointer>,
  /// Views of threads the manifest leaves undeclared that this run removed
  /// through organize's own gate (issue 0316), named rather than counted.
  pub dehydrated: Vec<std::path::PathBuf>,
  /// Why a view this run would have removed was kept, each in organize's own
  /// words with its remedy -- a hand edit, an unmet precondition, or a run
  /// that could not ask.
  pub dehydrate_refused: Vec<String>,
  /// How many event files this run wrote for project events the store held
  /// and the tree did not (ST0078 P1's backfill), so a project's history from
  /// before its events travelled travels too. Zero on every later run.
  pub events_backfilled: usize,
  /// What this run did with the `intent/events.jsonl` an earlier v3 upgrade
  /// left in the tree (issue 0459), or `None` when there was none.
  pub event_log_leftover: Option<EventLogLeftover>,
}

/// The single-file event log an earlier v3 upgrade left behind, and what this
/// upgrade did with it (issue 0459).
///
/// **Every 3.0.x upgrade wrote an empty `intent/events.jsonl`, hidden by an
/// ignore rule that this release retires**, so on an existing project the rule
/// goes and the file surfaces as untracked beside what the upgrade asks the
/// operator to commit. An upgrade removes what an upgrade left, and nothing it
/// cannot prove is that.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventLogLeftover {
  /// Zero bytes and untracked, so it held nothing and git never had it: removed.
  Removed(std::path::PathBuf),
  /// Left where it is, with why.
  Kept {
    path: std::path::PathBuf,
    why: String,
  },
}

/// Ensure the runtime store's directory is gitignored.
///
/// **The store is per-machine and must never enter history (D34), and this is
/// the one moment a project acquires one** -- so the ignore rule lands with the
/// database rather than being a thing sixteen fleet projects each remember to
/// add. On a project that already ignores it this is a no-op, which is every
/// project the canary included.
///
/// **A PATH RULE AND DELIBERATELY NOT A CLASS RULE.** `*.db` would silently
/// swallow a database a user genuinely wants tracked, in a tool whose whole
/// promise is that it does not touch what it was not asked to.
/// Write `.intentfiles` from the converted corpus when there is none -- the
/// `upgrade` third of **AC-11.3**.
///
/// **ABSENT ONLY, AND NEVER OVER AN EXISTING FILE.** A v2 estate has no
/// manifest, so this is what gives a converted project a declaration at all. A
/// project that already has one has SAID something, and an upgrade is not the
/// place to overrule it -- regenerating from status is `organize --default
/// --force`, which asks a human first.
///
/// **IT RUNS AFTER THE REALISATION, AND THE TWO NOW AGREE BY CONSTRUCTION.**
/// `migrate::plan` decides what to realise by reading back the very text this
/// function is about to write -- `realised_from(&default_declaration(..))` --
/// so the declared set and the realised set are the same set, derived once.
///
/// **THIS PARAGRAPH USED TO SAY THE OPPOSITE AND IT IS REWRITTEN RATHER THAN
/// DELETED, BECAUSE THE OLD TEXT WAS A PROMISE.** It read: *"So this ADDS a
/// declaration and changes not one file on disk. The other half of AC-11.3 --
/// the migration realising ONLY the declared threads -- is ... held to 3.0.2
/// (vc, 2026-08-26)."* That was true when written, and a reader arriving here
/// to trace why a converted estate had non-WIP threads on disk would have been
/// told it was known, deferred, and someone else's problem. **hv has ruled
/// there is no 3.0.2**, so a clause parked there was parked nowhere; vc lifted
/// the hold on 2026-08-27 once the residue it worried about was measured
/// rather than argued (see `migrate::plan`, where the measurement is recorded).
///
/// **The content is `intentfiles::default_declaration` and nothing else**, so a
/// change to what "open" means moves this caller, `init` and the verb together.
/// Since ST0069 WP-01 that content is WIP threads AND open issues, so this
/// takes both populations rather than growing a second declaration for one.
/// Where one issue's realised view lives, given the id a manifest line carries.
///
/// **THE PARSE CANNOT FAIL THROUGH EITHER DOOR THAT REACHES HERE, AND IT IS
/// STILL REPORTED RATHER THAN DEFAULTED.** `Sigil::accepts` gates a manifest
/// line on `model::is_issue_id`, and `Entity::Issue` is minted behind the same
/// predicate, so a four-digit id is the only thing that arrives today.
/// "Cannot happen" is a claim about today's callers: an `unwrap_or(0)` here
/// would quietly resolve a malformed id to issue `0000`'s view and realise or
/// remove the wrong file, which is the silent-wrong-answer class rather than a
/// crash.
/// Prove the artefact an id names exists, whichever kind it is.
///
/// **ONE RESOLVER PER KIND, DISPATCHED HERE, SO A THIRD KIND ADDS AN ARM
/// RATHER THAN A CALL SITE.** The resolution itself still lives once per kind
/// -- `st_show` and `issue_show` -- and this is a second CALLER of each, not a
/// second answer, which is the distinction `edit`'s own doc draws about
/// `st_show`.
impl Facade {
  fn resolve_artefact(&self, sigil: Sigil, id: &str) -> Result<(), FacadeError> {
    match sigil {
      Sigil::SteelThread => {
        self.st_show(id)?;
      }
      Sigil::Issue => {
        let number = id
          .parse::<u32>()
          .map_err(|_| FacadeError::MalformedIssueId { id: id.to_string() })?;
        self.issue_show(number)?;
      }
    }
    Ok(())
  }
}

fn issue_home(project: &Project, id: &str) -> Result<std::path::PathBuf, FacadeError> {
  id.parse::<u32>()
    .map(|number| project.issue_view(number))
    .map_err(|_| FacadeError::MalformedIssueId { id: id.to_string() })
}

fn declare_default_if_absent(
  project: &Project,
  threads: &[Thread],
  issues: &[crate::model::Issue],
) -> Result<(), std::io::Error> {
  let path = project.intentfiles_path();
  if path.exists() {
    return Ok(());
  }
  let open: Vec<(String, crate::model::ThreadStatus)> =
    threads.iter().map(|t| (t.id.clone(), t.status)).collect();
  let open_issues: Vec<(u32, crate::model::IssueStatus)> =
    issues.iter().map(|i| (i.number, i.status)).collect();
  std::fs::write(&path, intentfiles::default_declaration(&open, &open_issues))
}

/// The per-machine artefacts a converged project ignores, each a PATH under the
/// intent directory with the line that says why -- the same class Intent
/// ignores in its own tree.
///
/// **`.backup/` joined for issue `0120`, and with it the table is the WHOLE of
/// the class, enumerated rather than patched.** v3 writes per-machine artefacts
/// nowhere else: `backup` and `export --text` both write under
/// `<intent>/.backup/`. The ROOT `/.backup/` Intent's own `.gitignore` also
/// ignores is v2's upgrade-rollback namespace, which v3 does not write, so it is
/// not here. `.backup/` is the one that dirtied a consumer's tree through
/// doctor's own remedy: `intent backup` left a 46MB snapshot untracked on
/// Conflab.
///
/// **`events.jsonl` LEFT THE TABLE FOR ST0078 P1**, which reverses D53: the log
/// travels as one committed file per event under `.canon/events/`, which is
/// already tracked, so there is nothing new to ignore. It is in [`RETIRED`], so
/// a converge removes the rule rather than leaving it behind.
const IGNORED: &[(&str, &str)] = &[
  (
    ".cache/",
    "The Intent runtime store: per-machine, rebuilt from the committed extract.",
  ),
  (
    ".backup/",
    "Intent's store snapshots and text exports: per-machine, never committed.",
  ),
];

/// Rules a converge REMOVES, each with the comment line an earlier converge
/// wrote above it, which goes with it.
///
/// **THE COMMENT IS REMOVED ONLY WHEN IT IS THE ONE THIS CODE WROTE**, word for
/// word. `.gitignore` is the operator's file, so a comment somebody wrote above
/// the rule in their own words is theirs, and stays.
///
/// **SO EACH COMMENT HERE IS A MATCH KEY, NEVER PRINTED, AND KEEPS ITS OLD
/// WORDS.** It is the exact line earlier converges wrote into operators'
/// `.gitignore` files, internal decision id included; rewording it here would
/// leave that line behind in every project that has it.
const RETIRED: &[(&str, &str)] = &[(
  "events.jsonl",
  "The event log lives in the store (D53); its file form is produced by `intent export`.",
)];

/// Where an event's committed file lives and what it holds: the one rendering
/// of an envelope as a file, for the act that wrote it and for the backfill.
fn event_file_write(
  project: &Project,
  event: &crate::event::Envelope,
) -> Result<(std::path::PathBuf, String), FacadeError> {
  let unserialisable = |why: String| FacadeError::EntityUnserialisable {
    form: "event file".to_string(),
    why,
  };
  let path = project
    .event_file(event)
    .map_err(|e| unserialisable(e.to_string()))?;
  let body = crate::event::to_file(event).map_err(|e| unserialisable(e.to_string()))?;
  Ok((path, body))
}

/// Write a committed file for every PROJECT event the store holds and the tree
/// lacks, and return how many were written (ST0078 P1's backfill, AC-01.4).
///
/// **ONCE AND IDEMPOTENTLY, BY CONSTRUCTION.** A file whose path exists is
/// skipped and never compared: an event file is never rewritten, so after the
/// first run every later one writes nothing. A machine-scoped event is skipped
/// as it is at the moment of the act ([`crate::event::travels`]), so the
/// backfill carries off the machine exactly what the act would have.
///
/// What it wrote is recorded in the file index like any file the store wrote,
/// so a watching daemon does not read the backfill back as an edit.
pub(crate) fn backfill_event_files(
  project: &Project,
  store: &mut Store,
) -> Result<usize, FacadeError> {
  let mut set = WriteSet::new();
  for event in store.events()? {
    if !crate::event::travels(&event.op) {
      continue;
    }
    let (path, body) = event_file_write(project, &event)?;
    if path.exists() {
      continue;
    }
    set.add(path, body);
  }
  if set.is_empty() {
    return Ok(0);
  }
  let applied = set.commit()?;
  let landed: Vec<std::path::PathBuf> = applied.written().map(std::path::PathBuf::from).collect();
  applied.keep();
  ingest::record_canon_files(project, store, &landed)?;
  Ok(landed.len())
}

pub(crate) fn converge_gitignore(project: &Project) -> Result<(), std::io::Error> {
  let dir = project
    .intent_dir()
    .file_name()
    .map(|n| n.to_string_lossy().into_owned())
    .unwrap_or_else(|| "intent".to_string());
  let path = project.root().join(".gitignore");
  let current = std::fs::read_to_string(&path).unwrap_or_default();
  let mut next = current.clone();
  for (member, why) in IGNORED {
    let rule = format!("{dir}/{member}");
    if next.lines().any(|l| l.trim() == rule) {
      continue;
    }
    if !next.is_empty() && !next.ends_with('\n') {
      next.push('\n');
    }
    next.push_str(&format!("\n# {why}\n{rule}\n"));
  }
  for (member, why) in RETIRED {
    next = without_rule(&next, &format!("{dir}/{member}"), &format!("# {why}"));
  }
  if next == current {
    return Ok(());
  }
  std::fs::write(&path, next)
}

/// Remove the empty, untracked `intent/events.jsonl` an earlier upgrade left
/// (issue 0459), or name why a file there is left alone.
///
/// **ONLY THE ONE SHAPE AN UPGRADE WROTE IS REMOVED**: a regular file of zero
/// bytes that git does not track. A tracked file is the project's history, and
/// removing it would be a change for its owner to commit; a file with content
/// holds bytes no build of Intent wrote there. Both are named with why. A git
/// that cannot answer leaves the file too, because not knowing is not untracked.
/// A later step that halts the upgrade does not bring the file back, and that
/// loses nothing: it held no bytes and git never had it.
pub(crate) fn remove_event_log_leftover(
  project: &Project,
) -> Result<Option<EventLogLeftover>, std::io::Error> {
  let path = project.intent_dir().join(crate::event::JSONL);
  let meta = match std::fs::symlink_metadata(&path) {
    Ok(meta) => meta,
    Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
    Err(e) => return Err(e),
  };
  let kept = |why: String| {
    Ok(Some(EventLogLeftover::Kept {
      path: path.clone(),
      why,
    }))
  };
  if !meta.is_file() {
    return kept(
      "it is not a regular file, so it is not the empty log an earlier upgrade wrote".to_string(),
    );
  }
  match crate::gitstate::is_tracked(project.root(), &project.relative(&path)) {
    Err(e) => {
      return kept(format!(
        "git could not say whether it is tracked ({e}), so it is left for you to read"
      ));
    }
    Ok(true) => {
      return kept(
        "it is tracked in git, so removing it is a change to your history -- `git rm` it yourself if you mean to".to_string(),
      );
    }
    Ok(false) => {}
  }
  if meta.len() > 0 {
    return kept(format!(
      "it holds {} byte(s), which no build of Intent wrote there -- read it, and delete it yourself if you do not need it",
      meta.len()
    ));
  }
  std::fs::remove_file(&path)?;
  Ok(Some(EventLogLeftover::Removed(path)))
}

/// `text` without the lines that equal `rule`, and for each one the `comment`
/// directly above it and the blank line that separated the pair from what came
/// before, so a removal leaves the file as a converge that never added the rule
/// would have.
fn without_rule(text: &str, rule: &str, comment: &str) -> String {
  let lines: Vec<&str> = text.lines().collect();
  let mut drop = vec![false; lines.len()];
  for (i, line) in lines.iter().enumerate() {
    if line.trim() != rule {
      continue;
    }
    drop[i] = true;
    let mut top = i;
    if i > 0 && lines[i - 1].trim() == comment {
      drop[i - 1] = true;
      top = i - 1;
    }
    let blank_after = lines.get(i + 1).is_none_or(|l| l.trim().is_empty());
    if top > 0 && lines[top - 1].trim().is_empty() && blank_after {
      drop[top - 1] = true;
    }
  }
  if !drop.contains(&true) {
    return text.to_string();
  }
  let mut out: String = lines
    .iter()
    .zip(&drop)
    .filter(|(_, gone)| !**gone)
    .map(|(line, _)| format!("{line}\n"))
    .collect();
  if !text.ends_with('\n') {
    out.pop();
  }
  out
}

/// Ensure the project's formatter leaves generated views alone (AC-07.6).
///
/// **A GENERATED VIEW HAS ONE WRITER AND IT IS THE RENDERER** (D02 applied to
/// writers rather than to content, hv's ruling 2026-08-19). A markdown
/// formatter run from a pre-commit hook is a second writer, and a silent one:
/// it rewrites markup the author wrote -- `*emphasis*` becomes `_emphasis_` --
/// the skew check then compares canon to the committed view and reports drift
/// on a file nobody edited, regenerating restores the renderer's bytes, and the
/// next commit rewrites them again. Forever, with every pass looking like a
/// legitimate repair. Measured in this estate before the exclusion existed: 7
/// diverged rows, and three consecutive commits reporting "1 file changed"
/// because at stage time the view matched HEAD again.
///
/// **THIS EXISTS BECAUSE THE ESTATE FIXED ITSELF BY HAND AND THEREBY STOPPED
/// BEING ABLE TO SEE THE BUG.** Intent's own `.prettierignore` is 40 lines
/// somebody sat down and wrote, so this repository has been immune since
/// 2026-08-19 while every consumer stayed exposed -- and from inside, the two
/// are indistinguishable. That is the whole content of AC-07.6's closing
/// clause, *in a consumer repo as well as this one*.
///
/// **ADDITIVE, NEVER AUTHORITATIVE.** `.prettierignore` is the operator's file.
/// Missing patterns are appended and everything already there is left alone, so
/// a consumer who has tuned theirs keeps it and a second run changes nothing.
/// Intent excludes what IT generates and has no business switching off a
/// consumer's formatter anywhere else -- which is why this writes only the
/// generated views' patterns and never `*`.
///
/// The patterns come from [`Project::generated_view_patterns`], beside the
/// methods that produce the real paths, so this function holds no roster of its
/// own to drift.
pub(crate) fn converge_formatter_exclusion(
  project: &Project,
) -> Result<Vec<String>, std::io::Error> {
  let path = project.root().join(".prettierignore");
  let current = std::fs::read_to_string(&path).unwrap_or_default();
  let missing = formatter_exclusion_missing(project);
  if missing.is_empty() {
    return Ok(missing);
  }
  let mut next = current;
  if !next.is_empty() && !next.ends_with('\n') {
    next.push('\n');
  }
  next.push_str(
    "\n# Generated views have ONE writer, and it is the renderer. A formatter\n\
     # editing these rewrites markup the author wrote, and `intent doctor` then\n\
     # reports the drift as a hand-edit on a file nobody touched.\n",
  );
  for pattern in &missing {
    next.push_str(pattern);
    next.push('\n');
  }
  std::fs::write(&path, next)?;
  Ok(missing)
}

/// The generated views' formatter-ignore patterns a project's `.prettierignore`
/// does not carry: what [`converge_formatter_exclusion`] appends, asked without
/// writing so a dry run can report it (0378).
pub(crate) fn formatter_exclusion_missing(project: &Project) -> Vec<String> {
  let current = std::fs::read_to_string(project.root().join(".prettierignore")).unwrap_or_default();
  project
    .generated_view_patterns()
    .into_iter()
    .filter(|pattern| !current.lines().any(|l| l.trim() == pattern))
    .collect()
}

/// Write `intent_version` into `config.json`. **THE LAST ACT OF THE
/// MIGRATION** -- see [`Facade::upgrade`] for the three reasons.
///
/// The file is rewritten from its parsed form with only this key replaced, so
/// an unknown key a project carries survives: `config.json` is the operator's
/// file and the migration has no business pruning it.
fn stamp_version(project: &Project) -> Result<(), std::io::Error> {
  let path = Project::config_path(project.root());
  let text = std::fs::read_to_string(&path)?;
  let mut value: serde_json::Value = serde_json::from_str(&text).map_err(std::io::Error::other)?;
  let Some(map) = value.as_object_mut() else {
    return Err(std::io::Error::other(format!(
      "{} is not a JSON object, so there is no version field to stamp",
      project.relative(&path)
    )));
  };
  map.insert(
    "intent_version".to_string(),
    serde_json::Value::String(crate::faces::INTENT_VER.to_string()),
  );

  // **`project_id`, MINTED ONCE AND NEVER RE-MINTED** (design.md D15 and the
  // four cloud seams; vc ruled the value a UUID 2026-08-20). The natural keys
  // stay human-legible and the UUID namespaces them, so `(project_id,
  // natural_id)` is the global identity.
  //
  // **THIS IS WHERE THE STAMP GOES AND IT IS NOT WHERE THE COMMENT SAID IT
  // WAS.** `migrate.rs` read *"the facade mints and stamps it, last"* -- above
  // the `Bundle::new(&ctx.project_id, ..)` whose id is empty on a pre-migration
  // project -- and the facade did no such thing. **The comment promised the fix
  // immediately above the call that depended on it**, so a reader tracing the
  // empty id was told the next step handled it and stopped. Three sites knew
  // about `project_id`: one assumed it (`project.rs`, ruling it out as the
  // migration marker BY REASONING THAT MIGRATED PROJECTS HAVE ONE), one
  // commented on it being empty, one mandated it. **None wrote it**, and
  // Intent's own self-hosted config carried no such field.
  //
  // **MINT-IF-ABSENT IS LOAD-BEARING, NOT DEFENSIVE.** `upgrade` is re-runnable
  // by the fix-forward ruling, and `running_it_twice_leaves_the_tree_byte_
  // identical` asserts a second run changes nothing. A fresh UUID per run would
  // red that test -- correctly, because it would mean a project's identity was
  // whatever the last migration happened to generate.
  //
  // An EMPTY string counts as absent. `Config::project_id` is `Option<String>`
  // and every read site does `.unwrap_or_default()`, so `""` is the value an
  // unstamped project already presents; treating it as present would stamp the
  // field and leave the identity empty forever.
  let unstamped = !map
    .get("project_id")
    .and_then(serde_json::Value::as_str)
    .is_some_and(|id| !id.is_empty());
  if unstamped {
    map.insert(
      "project_id".to_string(),
      serde_json::Value::String(crate::project::mint_project_id()),
    );
  }
  // **THE SCHEDULE IS BACK-FILLED SO THAT IT IS SOMETHING TO READ AND EDIT**
  // (hv, 2026-08-26: "back-fill the config"). A default that lives only in the
  // binary is a value the operator cannot find, and that IS the defect this
  // fixes: `backup.every_hours` was a real 24h default behind a key that
  // appeared in no config file anywhere, so every project was measured against
  // a number none of them could name. Writing the ratified key with its
  // declared value puts it where an editor and `intent config` both reach it.
  //
  // **INSERT-IF-ABSENT, on the `project_id` precedent above, and for the same
  // reason:** `running_it_twice_leaves_the_tree_byte_identical` asserts a second
  // upgrade changes nothing, and it must stay true. An operator who has already
  // set `weekly` keeps `weekly` -- back-filling is how an absent key acquires a
  // visible default, never how a set one acquires ours.
  //
  // The block is serialised FROM [`crate::project::BackupConfig`] rather than
  // written as a literal here, so a key added to that type back-fills with it
  // instead of quietly missing from every upgraded project.
  if !map.contains_key("backup") {
    map.insert(
      "backup".to_string(),
      serde_json::to_value(crate::project::BackupConfig::default())
        .map_err(std::io::Error::other)?,
    );
  }

  let mut out = serde_json::to_string_pretty(&value).map_err(std::io::Error::other)?;
  out.push('\n');
  std::fs::write(&path, out)
}

/// What `Facade::export` produced -- **a document, or a realised tree**.
///
/// Two shapes because the artefacts have two shapes, and `intent export`
/// therefore stops meaning "the artefact on stdout" in every case. hv took that
/// knowingly on 2026-08-20 rather than keep the signature uniform, because the
/// only way to keep it uniform was a SECOND markdown producer -- and the one
/// that drifts is always the one nobody is looking at.
///
/// **A verb returning a destination for tree-shaped output and a document for
/// document-shaped output is describing reality, not compromising** (vc).
#[derive(Debug, Clone)]
pub enum Exported {
  /// The artefact, verified to re-derive the canon byte for byte.
  Document(String),
  /// The realisation happened; this says what and where. There is nothing to
  /// print as an artefact, so the destination and the denominator ARE the
  /// answer to the operator's question.
  Realised(realise::Realisation),
}

/// Directories that still exist under `dir` and have no surviving subdirectory,
/// `dir` itself included when it is one.
///
/// Recursive rather than iterative because the depth is a thread's own file
/// tree -- units of tens, never unbounded -- and the recursive spelling is the
/// one whose correctness is readable. A directory that cannot be read is
/// SKIPPED rather than raised: this runs after the act, purely to describe what
/// is left, and a describe step must never turn a completed run into a failure.
fn surviving_leaves(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
  if !dir.is_dir() {
    return;
  }
  let Ok(entries) = std::fs::read_dir(dir) else {
    return;
  };
  let children: Vec<std::path::PathBuf> = entries
    .flatten()
    .map(|e| e.path())
    .filter(|p| p.is_dir())
    .collect();
  if children.is_empty() {
    out.push(dir.to_path_buf());
    return;
  }
  for child in children {
    surviving_leaves(&child, out);
  }
}

/// What a `todo done --flush` did, in the operator's terms.
///
/// **`remaining` is the field that stops the command lying.** `--flush`
/// promises to clear the DONE view and cannot clear same-day completions --
/// `Thread.completed` is date-granular, so there is no fact separating "finished
/// this morning, before the flush" from "finished this afternoon, after it". A
/// flush reporting only what it removed reads as a success on a view the
/// operator can see is not empty. Reporting both numbers turns a puzzling no-op
/// into a stated limit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoFlush {
  /// The watermark now in force, as a date.
  pub watermark: Option<String>,
  /// Items that were in DONE before the flush.
  pub cleared: Vec<String>,
  /// Items still in DONE after it -- those completed on the watermark's own
  /// date, which no watermark can exclude.
  pub remaining: Vec<String>,
}

/// What [`Facade::declare_default`] did.
///
/// **`wrote` AND `was_present` ARE BOTH CARRIED, BECAUSE THREE OUTCOMES SHARE
/// TWO WORDS.** Absent-and-written, present-and-left-alone, and
/// present-and-regenerated are different events, and a report carrying only
/// "did something" collapses the first and third -- which are the safe one and
/// the destructive one.
#[derive(Debug, Clone)]
pub struct Declared {
  pub path: std::path::PathBuf,
  /// Whether this run wrote the file.
  pub wrote: bool,
  /// Whether a manifest was already there when the run started.
  pub was_present: bool,
  /// How many entries the file declares NOW -- after the write when there was
  /// one, and as found when there was not.
  pub declares: usize,
}

/// What [`Facade::hydration`] did (0083).
#[derive(Debug, Clone)]
pub struct Hydration {
  /// Every file the artefact owns that NOW exists -- [`Facade::hydrate`]'s
  /// answer, the same whether or not this call had anything to do.
  pub paths: Vec<std::path::PathBuf>,
  /// The subset of `paths` this call wrote: created, or rewritten because its
  /// bytes differed. Empty on a hydrate that found everything in place.
  pub wrote: Vec<std::path::PathBuf>,
}

/// What [`Facade::dehydrate`] did.
///
/// **`unlisted` IS A FACT ABOUT THIS RUN, NOT A RESTATEMENT OF `removed`.** The
/// two are independently reachable: a thread can be listed with no files on
/// disk, and present on disk while never listed. Collapsing them into one count
/// would make the ordinary re-run -- nothing listed, nothing present -- read
/// identically to the run that did the work (AC-00.6).
#[derive(Debug, Clone)]
pub struct Dehydrated {
  /// Files removed from disk, in the order the plan held them.
  pub removed: Vec<std::path::PathBuf>,
  /// Directories that became empty and were pruned. Never the estate root --
  /// `organize::prune_emptied` carries that floor and proves it directly.
  pub pruned: Vec<std::path::PathBuf>,
  /// Whether the manifest actually changed. `false` when the id was not listed,
  /// which is an ordinary state and not an error.
  pub unlisted: bool,
  /// Directories this run emptied of everything it was allowed to remove and
  /// still could not delete, because content outside the corpus remains in them.
  ///
  /// **NAMED RATHER THAN REFUSED ON, AND NEITHER IS THE SAME AS SILENT** (vc,
  /// 2026-08-26). `prune_emptied` skips a failed `remove_dir` through an
  /// `is_ok()`, which is correct as a floor and silent as a report. Ignored
  /// paths are outside the corpus by D29/AC-03.7 -- never counted, never
  /// removed -- so their presence must not refuse the run. But it leaves the
  /// manifest saying dehydrated while a directory tree remains, and **we have a
  /// manifest to contradict, which is exactly what git does not**: `git rm -r`
  /// leaves ignored files too and says nothing, because nothing it keeps claims
  /// otherwise. So the verdict names them and never reads a bare `dehydrated`.
  pub left_in_place: Vec<std::path::PathBuf>,
}

#[derive(Debug, thiserror::Error)]
pub enum FacadeError {
  /// A write the address scheme can express and this surface will not perform.
  ///
  /// One variant carrying a REASON rather than four variants, because every
  /// case is "you addressed something real and asked for a write that is not
  /// available here" -- and the operator needs the rule that sent them away,
  /// not a taxonomy of refusals.
  #[error("`{url}` cannot be written: {why}")]
  WriteNotAddressable { url: String, why: String },
  /// An acceptance-test row that would break its thread's contract (issue
  /// 0325). **ITS OWN VARIANT BECAUSE THE REMEDY IS.** It borrowed
  /// `WriteNotAddressable`, whose remedy is the `put` door's -- `PUT` json,
  /// `POST` to the collection -- which tells a terminal caller nothing about a
  /// `--covers` naming a criterion that does not exist.
  #[error(
    "`intent:///threads/{st}/at/{at}` cannot be written: the row would not satisfy the acceptance-test contract: {findings}"
  )]
  RowBreaksContract {
    st: String,
    at: String,
    findings: String,
  },
  /// The SQL door was handed more than one statement (AC-17.1).
  ///
  /// **ITS OWN VARIANT BECAUSE THE OPERATOR'S NEXT MOVE IS DIFFERENT.** A batch
  /// is a person expecting a shell; a write is a person expecting a different
  /// contract; and a door that answered both with one sentence would send the
  /// first of them looking for a permission they do not need.
  #[error("the SQL door runs ONE statement and this text carries more than one")]
  SqlMoreThanOneStatement,
  /// The SQL door was handed nothing but whitespace and comments.
  #[error("the SQL door was given no statement to run")]
  SqlNoStatement,
  /// A quote or block comment that never closes -- refused by the gate rather
  /// than passed to SQLite, because that is where a scanner and a parser
  /// disagree about how many statements there are.
  #[error("the SQL door was given a statement with an unterminated quote or comment")]
  SqlUnterminated,
  /// The statement would change the store.
  #[error("the SQL door is READ-ONLY and this statement would change the store")]
  SqlWouldWrite,
  /// The statement asked for something outside this store -- `ATTACH`, a
  /// pragma, a transaction.
  ///
  /// **SEPARATE FROM A WRITE, and `ATTACH` is why.** A read-only connection
  /// reads ANOTHER file on the machine perfectly happily, so this is the one
  /// refusal that is about reach rather than about writing, and this door is
  /// exposed on MCP.
  #[error("the SQL door refuses `{action}` -- it answers about THIS store and nothing else")]
  SqlOutOfReach { action: String },
  /// The statement spent the door's whole work budget and was stopped.
  ///
  /// **THE REFUSAL SAYS "WORK", NOT "TIME", because the bound is work.** A
  /// message promising a time bound would be describing a mechanism this
  /// workspace does not have: nothing here may read a clock (D42).
  #[error("the statement did more work than the SQL door allows and was stopped")]
  SqlOverBudget,
  /// A `--limit` above the door's ceiling.
  #[error("`--limit {asked}` is above the SQL door's ceiling of {ceiling}")]
  SqlLimitAboveCeiling { asked: usize, ceiling: usize },
  /// A structural door asked with a tier filter that leaves the structural
  /// tier out (ST0076 WP-04).
  ///
  /// **ITS OWN VARIANT BECAUSE THE QUESTION IS UNANSWERABLE, NOT MALFORMED.**
  /// `--outline`, `--context` and a search asked only its filters are answered
  /// by the structural tier alone, so the remedy is to widen the tiers or ask a
  /// text query, and no neighbour's sentence says that.
  #[error(
    "this question is answered by the structural tier alone, and the tier filter leaves it out"
  )]
  StructuralTierNotAsked,
  /// SQLite refused the statement for a reason of its own -- a syntax error, an
  /// unknown table. Carried as itself: the operator wrote the SQL, and SQLite's
  /// own words are the most useful thing anybody can say about it.
  #[error("the statement did not run: {detail}")]
  SqlDidNotRun { detail: String },
  /// A verdict (`green` / `red`) on a row whose cited file is not there (`0270`).
  ///
  /// **SEPARATE BECAUSE THE REMEDY HAS TO NAME A CONSEQUENCE, NOT A FIX.** The
  /// operator is not doing anything malformed -- `at red` on a failing test is
  /// exactly right -- and what they need told is that this particular row's
  /// citation does not exist yet, that the transition is one-way, and that the
  /// finding it creates refuses every commit in the repository rather than
  /// theirs.
  #[error(
    "{at} cites `{path}`, which does not exist, so `{status}` would make it a live absent_at finding"
  )]
  VerdictCitesAbsentFile {
    st: String,
    at: String,
    path: String,
    status: String,
  },
  /// An attachment path that names nowhere inside the thread (`0262`).
  ///
  /// **SEPARATE FROM `WriteNotAddressable` BECAUSE THE REMEDY IS.** That
  /// variant's remedy routes a caller to the right DOOR; this one has to hand
  /// the operator the right PATH, and the two are not the same sentence. The
  /// first build of this check reused that variant and printed *`PUT` json to a
  /// caller-assigned id* at someone who had mistyped a file path.
  #[error("{}", fault.why(path, thread))]
  AttachmentPathNotInThread {
    url: String,
    path: String,
    thread: String,
    fault: crate::project::PathFault,
  },
  /// A detach naming an attachment the thread does not carry (issue 0394).
  ///
  /// Its own variant for `AttachmentPathNotInThread`'s reason: the remedy is a
  /// place to read the paths the thread DOES carry, and neither neighbour's
  /// sentence is that.
  #[error("{thread} carries no attachment at `{path}`, so there is nothing to detach")]
  NoSuchAttachment {
    url: String,
    path: String,
    thread: String,
  },
  /// `st relate` naming a target no thread in the project carries (issue 0460).
  ///
  /// **A LINK TO NOTHING IS THE DEFECT THIS VERB EXISTS TO REPAIR**, so the verb
  /// that writes links does not write one. `st unrelate` does not ask the same
  /// question, because dropping a dangling link is its whole reason to exist.
  #[error("{target} is not a steel thread in this project, so {thread} cannot be related to it")]
  NoSuchRelatedTarget { thread: String, target: String },
  /// `st relate` naming the thread as its own target (issue 0460).
  #[error("{thread} cannot be related to itself")]
  RelatedToItself { thread: String },
  /// `st unrelate` naming a link the thread does not carry (issue 0460).
  #[error("{thread} carries no related link to {target}, so there is nothing to unrelate")]
  NoSuchRelated { thread: String, target: String },
  /// A read naming a document the thread does not carry (issue 0398).
  ///
  /// **BESIDE `NoSuchAttachment` AND `NoSuchEditable`, AND NEITHER, BECAUSE THE
  /// REMEDY IS NEITHER.** A detach that misses wants the paths it could have
  /// named, and an edit that misses lists what the artefact carries; a read that
  /// misses is asking for a document that does not exist yet, and the door that
  /// makes one exist is `st attach`.
  #[error("{thread} carries no `{path}`, so there is nothing to show")]
  NotCarried { thread: String, path: String },
  /// An address naming ANOTHER project, handed to a door of this one (issue
  /// 0338 (i); hv's ruling 4 on AC-07.6, 2026-09-15).
  ///
  /// **ITS OWN VARIANT BECAUSE THE REMEDY IS.** The doors borrowed
  /// `NotHydratable`, whose remedy sends the operator to address an artefact --
  /// which a thread address already is -- and `WriteNotAddressable`, whose
  /// remedy is the `put` door's and whose reason promised a project registry
  /// that does not exist. What someone holding another project's address needs
  /// is this project's own spelling of it, which [`require_local`] builds.
  #[error(
    "`{url}` names the project `{authority}`, and a door of this project acts on this project only"
  )]
  CrossProjectAddress {
    url: String,
    authority: String,
    local: String,
  },
  /// A field the narrow setter will not write, and the door that does.
  ///
  /// **SEPARATE FROM `WriteNotAddressable` BECAUSE THE SUBJECT IS DIFFERENT.**
  /// That one is about an ADDRESS this surface declines to write; this is about
  /// a FIELD of an entity it writes happily. AC-08.5's own words are that an
  /// unwritable field is *reported BY NAME* -- and a variant carrying only a
  /// url cannot do that, because the name would live inside prose where no
  /// caller can read it back out.
  #[error("`{field}` cannot be set on `{url}`: {why}")]
  FieldNotWritable {
    url: String,
    field: String,
    why: String,
  },
  /// A value the CALLER STATED that the model cannot record.
  ///
  /// **Distinct from [`FacadeError::FieldNotWritable`], and the difference is
  /// which of the two things the operator has to change.** `FieldNotWritable`
  /// says *this field is not yours to set, go to the door that owns it*; this
  /// says *the field is yours and that value is not one of its values* -- so a
  /// remedy pointing at another door is actively wrong here.
  ///
  /// Driven before this existed: `st done --date 14/02/2026` refused
  /// correctly, then told the operator to `PUT json to a caller-assigned id`,
  /// which is nothing they can do about a slash.
  #[error("`{field}` was given `{given}`, which this field cannot record: {why}")]
  ValueNotRecordable {
    field: String,
    given: String,
    why: String,
  },
  /// A `file` written onto a non-test row (0146).
  ///
  /// **ITS OWN VARIANT BECAUSE THE REMEDY IS THE POINT.** `ValueNotRecordable`
  /// says the value is wrong; here the value is fine and the row's KIND is
  /// what cannot hold it, so the way out is a re-kind, which re-enters a row
  /// at `n-a` at `to-write` in the same call (issues 0324 and 0337).
  #[error(
    "{st} {at} is a non-test row, which asserts prose INSTEAD of a file, so `{file}` cannot be cited on it"
  )]
  FileOnANonTestRow {
    st: String,
    at: String,
    file: String,
  },
  /// A status that does not fit the row's kind (issue 0337): `n/a` on a test
  /// row, or `to-write`/`red`/`green` on a non-test row -- recorded by a verdict
  /// on an existing row, or named to `at new` for a row it would create.
  #[error("{st} {at} is a {kind} row, and `{status}` is not a verdict that row can hold")]
  VerdictWrongForKind {
    st: String,
    at: String,
    kind: String,
    status: String,
  },
  /// `st done` on a thread whose work packages are not all settled (issue 0324).
  #[error("{st} cannot close while work packages are still open: {}", .open.join(", "))]
  OpenWorkPackages { st: String, open: Vec<String> },
  /// A `--date` on a thread already in the state the closing verb moves to
  /// (issue 0503).
  ///
  /// **ITS OWN VARIANT BECAUSE THE VALUE IS FINE AND THE DOOR IS NOT**, which
  /// is [`FacadeError::NoteWouldBeLost`]'s distinction two variants down:
  /// `ValueNotRecordable` says the field cannot hold that value, and this says
  /// it can, and that a self-loop is not where it is written. A self-loop means
  /// *nothing to do*, so a date that differs from the one on record would be a
  /// RESTATEMENT of a close that already happened, and `intent set` is the door
  /// that restates a field. The remedy names it; this says what is on record.
  #[error(
    "{st} is already {state} and records {}, so `--date {given}` would restate the completion date rather than record it",
    match .recorded { Some(on) => on.clone(), None => "no completion date".to_string() }
  )]
  CompletionDateNotRestated {
    st: String,
    state: String,
    recorded: Option<String>,
    given: String,
  },
  /// A `--note` that would DESTROY an existing note rather than extend it.
  ///
  /// **DISTINCT FROM [`FacadeError::ValueNotRecordable`], and the difference is
  /// that the value is perfectly recordable.** That one says *the field is
  /// yours and that value is not one of its values*; this says *the value is
  /// fine and writing it discards something you did not mention*. An operator
  /// told their text was invalid would go and fix the text, which is the one
  /// thing that cannot help here.
  ///
  /// Issue 0207. On this estate an AT's note is where the row's adjudication
  /// history lives, and `--note` replaced it wholesale and silently -- on the
  /// three verbs you reach for AT CLOSE, which is exactly when someone writes
  /// down why. `AT-00.12` went 7803 bytes to 683 and the author did not notice.
  ///
  /// **THE DOOR IS IN `remedy()`, NOT IN THIS MESSAGE.** A refusal that names
  /// no way forward is a worse defect than the one it prevents -- but spelling
  /// the door in both places is the doubled rendering `IngestError::Refused`
  /// documents. This says what is at risk; `remedy()` says where to go.
  #[error(
    "`{url}` already carries a note of {existing_bytes} byte(s) and this `--note` is {incoming_bytes}, \
     so it would drop the note rather than extend it. What would be lost begins: {opening}"
  )]
  NoteWouldBeLost {
    url: String,
    existing_bytes: usize,
    incoming_bytes: usize,
    opening: String,
  },
  #[error("no steel thread {id} in this project")]
  NoSuchThread { id: String },
  #[error("steel thread {id} already exists")]
  ThreadExists { id: String },
  #[error("{st} has no WP-{seq:02}")]
  NoSuchWorkPackage { st: String, seq: u32 },
  #[error("no acceptance criterion {ac} in {st}")]
  NoSuchCriterion { st: String, ac: String },
  #[error("no acceptance test {at} in {st}")]
  NoSuchTest { st: String, at: String },
  /// **A CREATE LANDED ON A CRITERION ID THAT ALREADY EXISTS** (hv,
  /// 2026-08-28: a verb named `add`/`new` must FAIL on an existing key rather
  /// than replace it). The third member of the family with
  /// [`Self::ThreadExists`] and [`Self::IssueExists`], per-entity for the same
  /// reason they are -- the operator's next move differs, and this one sends
  /// them to `intent ac edit`.
  ///
  /// **UNLIKE ITS TWO SIBLINGS THIS IS RAISED IN THE FACADE RATHER THAN LIFTED
  /// FROM A STORE REFUSAL, AND THAT IS FORCED RATHER THAN PREFERRED.** A thread
  /// and an issue are rows with a UNIQUE key, so `store::Door::Create` can hand
  /// the collision to SQLite INSIDE the transaction, which is what makes that
  /// refusal hold against a facade whose canon is stale. A criterion is a CHILD
  /// row and `write_thread` replaces the child set wholesale, so there is no
  /// per-child constraint for a create door to fire on: the check has to be
  /// made against loaded canon before the write, and it inherits that read's
  /// staleness. **The window this leaves open is stated on [`Facade::ac_new`]
  /// and filed, not discovered later.**
  #[error("{st} already has criterion {ac}, and a create must not replace it")]
  CriterionExists { st: String, ac: String },
  /// The AT-side sibling of [`Self::CriterionExists`], carrying the same
  /// mechanism and the same window. Separate because the remedy names a
  /// different verb, and because what a replace destroyed here was measured:
  /// six ST0061 notes, eaten by a re-cite through `at new`.
  #[error("{st} already has acceptance test {at}, and a create must not replace it")]
  TestExists {
    st: String,
    at: String,
    /// The kind of the row already sitting on that id, carried SO THE REMEDY
    /// CAN NAME THE FLAG THAT APPLIES TO IT.
    ///
    /// **Not decoration: this field is the fix for issue 0146.** `at edit`
    /// takes `--file` on a test row and `--prose` on a non-test one and
    /// enforces neither, so a remedy naming `--file` unconditionally returns
    /// rc=0 on a non-test row and leaves it carrying BOTH -- a state whose
    /// vocabulary says the two are alternatives, and which no verb can undo
    /// because `kind` is settable at mint and changeable nowhere. **The
    /// operator most likely to reach it is the one who did what this sentence
    /// told them to.** The kind is in hand at the only construction site, so
    /// carrying it costs a field and not carrying it is what made the advice
    /// wrong.
    kind: AtKind,
  },
  /// **AN EDIT THAT NAMED NOTHING TO EDIT.**
  ///
  /// Its own small class, and not a member of the `*Required` family beside it:
  /// those name ONE field a call left out, and this one says that none of
  /// several was given when at least one had to be. Collapsing it into
  /// `ReasonRequired`'s shape would force it to pick a field to blame, which
  /// is a guess about what the caller meant.
  ///
  /// **It exists because the alternative is a silent success.** An edit with no
  /// field named would otherwise return "unchanged" at exit 0 to a caller who
  /// believes they changed something -- IN-AG-NO-SILENT-001, reached through a
  /// door that reports no error at all.
  #[error("nothing was named to change on {subject}")]
  NothingToChange {
    subject: String,
    /// The fields this door can move, so the refusal answers the question it
    /// raises rather than sending the caller to `--help`.
    offered: Vec<String>,
  },
  #[error("{scope} is not ready to close -- {verdict}")]
  GateBlocked {
    scope: String,
    verdict: String,
    /// The remedy of the gate arm that blocked (issue 0526), carried because
    /// only the verdict knows which arm that was.
    remedy: String,
  },
  #[error(
    "{ac} is test-backed, so its satisfaction is computed from covering green acceptance tests and cannot be set directly"
  )]
  ComputedSatisfaction { ac: String },
  /// **Carries the verb the caller actually typed** (ic, issue 0053). One
  /// hardcoded `reinstate` served both entry points, so `intent ac rescope` on an
  /// in-scope criterion was answered with advice about a different command --
  /// and v2 gets this right, so it was a regression rather than a gap.
  /// `WrongOffScopeState` next door already carried a `verb`; this is the same
  /// field on its sibling.
  #[error("{ac} is in scope, so there is nothing to {verb}")]
  NotOffScope {
    ac: String,
    verb: String,
    /// The one state this verb undoes -- `descoped` for `rescope`, `withdrawn`
    /// for `reinstate`. The old remedy named both, which is the union of the two
    /// verbs' preconditions and true of neither of them.
    wanted: String,
  },
  #[error("{ac} is not satisfied, so there is nothing to unsatisfy")]
  NotSatisfied { ac: String },
  /// A fiat close met a requirement that already carries one.
  ///
  /// **Not `OffScope`, and the distinction is vocabulary rather than
  /// tidiness.** A fiat-closed criterion has not left scope -- it is in scope
  /// and closed on authority -- so answering it with a refusal whose remedy
  /// talks about "a requirement nobody is working on" would describe a
  /// different act. `OffScope`'s remedy is hardcoded to the descope/withdraw
  /// story and would have been wrong here in exactly the way a reused error
  /// usually is: right shape, wrong sentence.
  ///
  /// **It carries the STANDING reason**, because the operator is about to
  /// replace one human judgement with another and the one already on the
  /// record is the thing they need to see before they do.
  //
  // **`undo` IS A FIELD BECAUSE THE REMEDY WAS HARDCODED TO ONE KIND.** This
  // variant was minted for criteria and its remedy named `ac reinstate`
  // literally; ST0066's AT kind reaches the same state by a different door and
  // leaves it by another one again. Hardcoding a second sentence beside the
  // first is the failure the doc above describes -- right shape, wrong sentence
  // -- so the caller supplies the way out and the sentence stays single.
  #[error("{subject} is already fiat-closed: {because}")]
  AlreadyFiatClosed {
    subject: String,
    because: String,
    /// The full command that reverses it, ready to run.
    undo: String,
  },
  #[error("{ac} is {state}, so it cannot be {verb}")]
  OffScope {
    ac: String,
    state: String,
    /// The verb that brings it back into scope first.
    undo: String,
    verb: String,
  },
  #[error("{ac} is {actual}, not {wanted}")]
  WrongOffScopeState {
    ac: String,
    actual: String,
    wanted: String,
    /// The verb that DOES undo the state it is actually in.
    verb: String,
  },
  #[error("the search query `{query}` was refused")]
  BadQuery {
    query: String,
    #[source]
    cause: StoreError,
  },
  /// A store fault met while ANSWERING a search: the query is not at fault.
  ///
  /// **Lifted out of [`Self::BadQuery`] and [`Self::Store`] both** (issue
  /// 0443). As `BadQuery` a malformed index or a busy database told the reader
  /// their correct query was wrong and sent them after an unbalanced `(` that
  /// was never there -- a task that cannot succeed, which every retry seems to
  /// confirm. As `Store` it renders "could not update the runtime store", a
  /// sentence about a WRITE, to a reader who asked a question.
  #[error("the search `{query}` could not be answered")]
  SearchUnanswerable {
    query: String,
    #[source]
    cause: StoreError,
  },
  #[error("no schema face named `{face}`")]
  NoSuchFace { face: String },
  #[error("no issue {number:04} in this project")]
  NoSuchIssue { number: u32 },
  /// A manifest or address id that is not an issue number at all.
  ///
  /// **Its own variant rather than [`FacadeError::NoSuchIssue`] with a zero**,
  /// which is an idiom already in this file and reads as "no issue 0000 in
  /// this project" -- a sentence about a project's contents, for a fault in
  /// the id's SHAPE. The two send an operator to different places.
  #[error("`{id}` is not an issue number")]
  MalformedIssueId { id: String },
  /// An address whose form names no entity a renderer can resolve.
  ///
  /// **THE ADDRESS GRAMMAR IS WIDER THAN THE FORM DECLARATION, AND THAT IS
  /// DELIBERATE ON BOTH SIDES.** `Entity::form()` answers thirteen names;
  /// `surface/forms.json` declares three. A criterion, a test and an
  /// attachment are ROWS INSIDE a collection a renderer draws, not entities
  /// with forms of their own, and a collection is not a thing with fields at
  /// all -- so the gap is not a backlog, it is the two vocabularies meaning
  /// different things.
  ///
  /// **REFUSED BY NAME RATHER THAN ANSWERED EMPTY**, which is the same rule
  /// `form::Loaded` applies to an unknown widget. An empty field list is a
  /// legal answer -- it means *this entity has no declared fields* -- so
  /// returning one here would make *no form for this kind* indistinguishable
  /// from *a form with nothing in it*, at the one layer that could still tell
  /// them apart.
  #[error("`{form}` is not an entity this project renders a form for")]
  NoFormForEntity { form: String },
  /// A stored entity that will not serialise.
  ///
  /// **SEPARATE FROM [`FacadeError::NoFormForEntity`] BECAUSE THE TWO SEND THE
  /// READER TO DIFFERENT PLACES.** *This kind has no form* is a fact about the
  /// declaration and the operator can act on it; *this entity will not
  /// serialise* is a fault in this process, and the model types derive
  /// `Serialize`, so reaching it means something structural broke. Folding it
  /// into the first would hand someone a *check which kinds are declared*
  /// remedy for a defect no spelling of the address can avoid.
  #[error("the stored `{form}` will not serialise: {why}")]
  EntityUnserialisable { form: String, why: String },
  // The ratified machines have no terminal states, so every refusal here is
  // about ORDER rather than about a dead end -- there is always a route, and
  // the remedy names where it starts.
  #[error("`{verb}` is not a legal transition for {subject}, which is `{from}`")]
  IllegalTransition {
    verb: &'static str,
    subject: String,
    from: String,
    /// The states the declared graph accepts this verb from.
    legal: String,
  },
  #[error("`{verb}` requires a reason and was given none")]
  ReasonRequired { verb: &'static str },
  // Its own variant rather than `ReasonRequired` with a different word in it:
  // the two are owed for different reasons and the remedy has to say which. A
  // reason explains a decision; evidence is the whole substitute for a test
  // result on a criterion that has none.
  #[error("{ac} is a non-test criterion, so satisfying it requires evidence and none was given")]
  EvidenceRequired { ac: String },
  #[error("cannot descope {ac} to {to}, which is not a steel thread in this project")]
  DescopeTargetMissing { ac: String, to: String },
  #[error("descoping {ac} moves it to another steel thread, and no thread was named")]
  DescopeTargetRequired { ac: String },
  // NOT `#[source]`-bearing and NOT constructed from anything: the whole value
  // of this variant is that it carries the EVIDENCE, so that a refusal can be
  // told apart from an empty project by reading it.
  #[error("{0}")]
  Unmigrated(Pending),
  /// The estate is below the migration floor, so the migrator refuses it.
  ///
  /// **A DISTINCT VARIANT FROM `Unmigrated` although both carry a `Pending`,
  /// because the two are told to operators in opposite situations.**
  /// `Unmigrated` answers someone who asked a question about an estate v3
  /// cannot read, and *this project has not been migrated* is news to them.
  /// Here the operator RAN the migrator, so that sentence restates their own
  /// intent back at them and says nothing about why it stopped. What they need
  /// is the version they have and the floor they are under.
  ///
  /// **The remedy is delegated, not copied**: the two-hop text already lives on
  /// [`Pending`], and a second spelling of it is a second thing to keep true.
  #[error(
    "this project declares Intent {} and is below the migration floor, so it cannot be converted directly",
    .0.declared
  )]
  BelowMigrationFloor(Pending),
  /// The conversion path met a project git reports no work tree for.
  ///
  /// **`migration.md` states this refusal and states its own reason for it:**
  /// *rollback is git; migrating without an undo is a lossy operation by
  /// construction.* The Rollback section then rests its entire cost argument on
  /// the same property -- *cheap because the migration is ONE named commit over
  /// a v2 estate git holds whole.* **The refusal was documented and absent**
  /// (issue 0271, measured on the genuine conversion path): a project with no
  /// `.git` converted, exit 0, and the word `git` did not appear once in the
  /// output. The operator got a converted estate, a success line, and no undo,
  /// with nothing naming the missing property.
  ///
  /// **Scoped to `Migration::Pending`, so it cannot reach the convergent
  /// re-run**, and additive by measurement rather than by hope: all 18 fleet
  /// members carry a `.git`, so this refuses none of them today.
  ///
  /// **It says what was OBSERVED, never which of the two causes it was.** No
  /// repository and no runnable git are indistinguishable from here, and a
  /// message picking one would be wrong half the time; the remedy names both.
  #[error("git reports no work tree at this project, so a migration here would have no undo")]
  MigrationWithoutGit,
  /// The conversion path met a working tree carrying work that is not committed.
  ///
  /// **The dirt that matters is measured against HEAD, not the index** -- see
  /// [`crate::sync::tree_state`]. `git commit` records the index as it stands,
  /// so staged work rides the migration commit, and a revert then takes it too.
  ///
  /// **Scoped to the conversion path and NEVER to the verb** (vc's ruling on
  /// cc's measurement, 2026-09-05). `intent upgrade` is the convergent
  /// orchestrator and gets run routinely: hung on the verb this refuses 11 of
  /// 18 fleet members today, hung on the conversion it refuses 1 of 1 -- the
  /// only unmigrated member, correctly, and remediably by committing. **A
  /// precondition written for a one-shot conversion, applied to a verb people
  /// run all day, is a migration wearing a fix's clothes.** Git-presence is
  /// stable; dirt is the normal state of an active repo.
  #[error(
    "this project has {} uncommitted change(s), and a migration commit assembled over them could not be reverted without taking them too",
    .paths.len()
  )]
  MigrationOverDirtyTree {
    /// Every offending path, sorted and NEVER truncated -- `migration.md`'s
    /// no-silent-caps rule, which exists because a capped list reads as
    /// complete when it is not.
    paths: Vec<crate::sync::Uncommitted>,
  },
  #[error("could not write the project files")]
  Write(#[from] WriteError),
  // NOT a failed mutation, and the text says so. Under D01 as reversed the DB
  // is the truth, so by the time this is returned the change IS recorded --
  // what failed is the projection of it onto disk. A caller that read this as
  // "the mutation failed" and retried would be acting on the opposite of what
  // happened, so the message leads with what succeeded.
  #[error("the change is recorded, but the files on disk could not be rewritten")]
  ViewsNotWritten {
    #[source]
    cause: WriteError,
  },
  /// Phase A found live-thread residue, so no migration was planned.
  ///
  /// **Nothing has been written when this is returned** (AC-10.2), and that is
  /// structural rather than careful: `migrate::plan` builds a `WriteSet` and
  /// does not commit it, so a refusal cannot have touched the estate.
  ///
  /// **`transparent` rather than `{0}`, and the rule it restores was already
  /// written down one level below.** `Blocked::Residue` deliberately does NOT
  /// carry its `Refusal` as a `#[source]`, and says why at `migrate.rs:110`: a
  /// source there renders the whole list twice and every residue count reads
  /// double. **`#[from]` implies `#[source]`**, so pairing it with `{0}`
  /// recreated exactly that -- the entire classed report printed once as the
  /// message and once as its own cause, summary line included. Measured on a
  /// two-finding estate: ten lines for two findings, with `refused 2
  /// finding(s)` appearing twice, the first occurrence mid-output where it
  /// reads as the end of the list.
  ///
  /// **The rule survived being written down because the violation used a
  /// different spelling.** A reader checking this variant against the comment
  /// below looks for `#[source]` and does not find one. ic found it by reading
  /// the operator's output rather than the type.
  ///
  /// `transparent` forwards Display AND source to the inner error, so the
  /// chain now reaches what `Blocked` itself declares -- the serde error under
  /// `Canon`, nothing under `Residue` -- instead of restating `Blocked`.
  #[error(transparent)]
  MigrationBlocked(#[from] crate::migrate::Blocked),
  /// The migration stopped part way through, at a NAMED step.
  ///
  /// **The step is carried because the remedy depends on it and the operator
  /// cannot see where it stopped.** Under hv's fix-forward ruling the recovery
  /// is to run it again, and whether that is safe is a different answer before
  /// the version stamp than after it -- so a bare IO error here would leave the
  /// one question that matters unanswerable.
  #[error("the migration stopped while {step}")]
  MigrationHalted {
    step: &'static str,
    #[source]
    cause: std::io::Error,
  },
  /// `organize` refused, or could not read the tree.
  ///
  /// **Delegated rather than given a remedy here**, for the reason `Store` is:
  /// a hand-edited view, a moved tree, an attachment divergence and an unmet
  /// ship precondition are four different problems with four different actions,
  /// and one sentence covering all of them is the collapse `error_remedies.rs`
  /// exists to refuse.
  #[error("could not reconcile the tree")]
  Organize(#[from] organize::OrganizeError),
  /// The text realisation could not be written (AC-06.1 / AC-06.2).
  ///
  /// **Its own variant rather than folded into an IO error, because its two
  /// causes have opposite subjects**: a missing sidecar means CANON names bytes
  /// it does not carry, which is a repair to the estate; a write failure means
  /// the destination is unusable, which is a repair to the machine. Delegated
  /// for the same reason `Organize` is.
  #[error("could not write the text realisation")]
  Realise(#[from] realise::RealiseError),
  /// `.intentfiles` could not be parsed. Its own variant because the manifest
  /// error already carries the LINE NUMBER, and folding it into a generic read
  /// failure would drop the one field that makes it actionable.
  #[error("could not read the realisation manifest: {0}")]
  Intentfiles(#[from] intentfiles::IntentfilesError),
  /// `.intentfiles` is not there, or is not readable.
  ///
  /// **Distinct from a parse failure, because absent and malformed have
  /// opposite remedies** -- one is created, the other is corrected -- and a
  /// verb whose whole subject is what the manifest declares must not report
  /// "no manifest" as though the file were broken.
  #[error("could not read {path}")]
  ManifestUnreadable {
    path: String,
    #[source]
    source: std::io::Error,
  },
  /// `.intentfiles` exists and will not parse, raised by the door that OPENED
  /// it (AC-04.7 arm (b)).
  ///
  /// **NOT A SECOND SPELLING OF [`FacadeError::Intentfiles`], AND THE
  /// DIFFERENCE IS WHO KNOWS THE PATH.** `Intentfiles` is raised where the
  /// CALLER supplied the text -- `pin`, `unpin` -- and is about the edit being
  /// expressible; the path is that caller's own business and naming it would
  /// be a guess made twice. This one is raised by [`Facade::manifest_for_
  /// action`], the only reader that opened a file and therefore the only one
  /// that can say which. **AC-04.7 arm (b) requires the refusal to name the
  /// path**, and it requires it precisely because the fix for arm (a) removes
  /// the absent case from the refusal entirely: what survives has to be
  /// actionable on its own.
  #[error("could not read {path}: {cause}")]
  ManifestMalformed {
    path: String,
    #[source]
    cause: intentfiles::IntentfilesError,
  },
  /// The address names something realisation cannot make exist.
  ///
  /// **It names the FORM, and the form is why it is counted rather than
  /// dropped.** AC-05.1 wants a denominator over the forms the verb dispatches
  /// on, and a denominator is only honest if the refusals are IN it -- a verb
  /// that silently skips four of eleven forms reports the same number as one
  /// that handles all eleven.
  #[error("`{form}` is not something that can be realised to disk: {why}")]
  NotHydratable { form: &'static str, why: String },
  /// `st dehydrate` was pointed at a project with no `intent/.intentfiles`
  /// (ST0061 AC-00.4).
  ///
  /// **ABSENT MEANS NOBODY HAS SAID, SO EVERYTHING IS REALISED -- AND WRITING
  /// ONE HERE WOULD DECLARE THAT EVERYTHING EXCEPT THIS THREAD IS.** That is an
  /// estate-wide assertion, arrived at through a verb that names one thread,
  /// and made by nobody. It is the exact mirror of the hazard `hydrate` already
  /// refuses in the other direction, where creating a manifest to hold one
  /// entry would declare that entry to be the WHOLE of what is realised.
  ///
  /// Distinct from the ordinary `the thread was not listed`, which is an exit 0
  /// no-op: one of those two states needs fixing and the other does not, so
  /// they must not share an answer.
  #[error(
    "refusing to dehydrate {id}: this project has no {path}, so nothing has declared what is realised -- and absent means EVERYTHING is, not nothing"
  )]
  NoManifestToUnlistFrom { id: String, path: String },

  /// A realisation would have written over a view whose bytes on disk are not
  /// what the store renders (hv, 2026-09-12: silent deletion).
  ///
  /// **THE DIFFERENCE IS NOT A DIRECTION, WHICH IS WHY THIS REFUSES RATHER THAN
  /// CHOOSING.** A view that differs is either a hand edit the store never took
  /// in, or a render the store has moved past. Nothing on disk tells the two
  /// apart, and `dehydrate` has refused this exact signature since it was
  /// written -- `organize::gate` will not REMOVE such a file. Writing over it is
  /// the same loss by a different verb.
  #[error(
    "refusing to realise {id}: {} view(s) on disk are not what the store renders, and writing them would destroy the difference -- {}",
    paths.len(),
    paths.iter().map(|p| p.display().to_string()).collect::<Vec<_>>().join(", ")
  )]
  HydrationWouldOverwrite {
    id: String,
    paths: Vec<std::path::PathBuf>,
  },

  /// A realisation would have REMOVED a file under the artefact it was asked to
  /// realise (hv, 2026-09-12: silent deletion).
  ///
  /// **`hydrate` RUNS A WHOLE-ESTATE PLAN NARROWED TO ONE DIRECTORY, AND THE
  /// PLAN HAS REMOVALS IN IT.** A view the store no longer carries is a
  /// `Dehydrate` step, and a scoped run performed it -- so a verb whose entire
  /// job is to MAKE FILES EXIST could delete one, silently, on its way past.
  /// `edit` and `st edit` reach the same body, so the same removal arrived
  /// under a verb that prints a path and nothing else.
  ///
  /// **REFUSED RATHER THAN ANNOUNCED, WHICH IS THE ONE DECISION HERE.** The
  /// verb that reconciles an estate is `organize`, and it now names every
  /// removal before it makes it and confirms on a terminal. A realisation verb
  /// performing a removal the operator did not ask for is the wrong ACT, not an
  /// under-reported one, and announcing it would make it look intended.
  #[error(
    "refusing to realise {id}: the plan would REMOVE {} file(s) under it, and realising is not a verb that removes -- {}",
    paths.len(),
    paths.iter().map(|p| p.display().to_string()).collect::<Vec<_>>().join(", ")
  )]
  RealisationWouldRemove {
    id: String,
    paths: Vec<std::path::PathBuf>,
  },
  /// One or more of an artefact's realised files could not be shown to be in the
  /// store, so NONE of them was removed (ST0061 AC-00.2).
  ///
  /// **IT CARRIES EVERY REFUSAL, NOT THE FIRST.** A refusal naming one file
  /// trains an operator to fix that one and re-run, and a count with no detail
  /// cannot be told from a gate that checked nothing.
  #[error(
    "refusing to dehydrate {id}: {count} of its file(s) cannot be shown to be in the store, so none was removed --\n  {detail}"
  )]
  DehydrationRefused {
    id: String,
    count: usize,
    detail: String,
  },
  /// `intent edit` was pointed at a file the model writes (AC-05.1, hv
  /// 2026-08-19).
  ///
  /// **THE REFUSAL IS THE FEATURE AND THE DESTINATION IS WHY.** Handing over a
  /// generated view's path lets an operator author into a file the next render
  /// overwrites, and the skew check catches that AFTER the work is gone --
  /// **detection is not prevention.** A refusal that only refuses is barely
  /// better, because the operator still has a real edit to make, so
  /// `author_with` travels with it.
  #[error("`{path}` is generated from the model, so an edit here is lost at the next render")]
  NotEditable {
    path: String,
    author_with: &'static str,
  },
  /// The artefact does not carry that file.
  ///
  /// **v2's `st edit` PRINTED THE PATH ANYWAY** (`bin/intent_st:1101-1144`,
  /// _the thread DIRECTORY must exist; the file need not_). AC-05.1 asks for a
  /// path that EXISTS after the call, so this is a deliberate deviation: a path
  /// to nothing sends an editor to create an untracked file beside the
  /// artefact, which is the `Unclaimed` population `organize` already reports
  /// and nobody wants more of.
  #[error("`{path}` is not a file this artefact carries")]
  NoSuchEditable { path: String, present: Vec<String> },
  /// **A CREATE LANDED ON AN ISSUE NUMBER THAT ALREADY EXISTS** (issue 0131,
  /// hv ruled 2026-08-28: a verb named `add`/`new` must FAIL on an existing key
  /// rather than replace it).
  ///
  /// The sibling of [`Self::ThreadExists`], and per-entity for the same reason
  /// the whole of this vocabulary is: `intent issues add` and `intent st new`
  /// send their operator to different places.
  ///
  /// Neither travels as [`Self::Store`], which renders "could not update the
  /// runtime store" and drops its source. **An operator meeting a refusal has
  /// to be able to tell "the key is taken" from "the database broke"** -- the
  /// first is answered by taking the next key, the second by looking at the
  /// machine, and one message for both tells them to guess.
  #[error("issue {number:04} already exists, and a create must not replace it")]
  IssueExists { number: u32 },
  /// **A RENUMBER NAMED AN ID SOMETHING ALREADY HOLDS** (ST0078 WP-02).
  ///
  /// One variant for both kinds and both holders, because the next move is the
  /// same: pick another id. `held_by` says which holder, since a canon file or
  /// directory the store does not know is usually a pull not yet loaded, and
  /// that has its own remedy. Not [`Self::ThreadExists`] or
  /// [`Self::IssueExists`], whose remedies send the operator to a create verb.
  #[error("{subject} is already taken, held by {held_by}")]
  RenumberTargetTaken { subject: String, held_by: String },
  /// A renumber could not move a path on disk (ST0078 WP-02).
  ///
  /// Raised before the write, after every earlier move is put back, so nothing
  /// is renumbered. After the write the same failure is a note carrying its own
  /// remedy, because the store has already moved.
  #[error("could not {step}")]
  RenumberDiskStep {
    step: String,
    #[source]
    source: std::io::Error,
  },
  /// `sync --apply --plan <digest>` named a plan the tree no longer matches
  /// (ST0078 WP-05, AC-05.1). Refused before any step runs.
  #[error(
    "the plan you were shown read the tree as {shown} and it now reads {now}, so the steps about to run are not the ones that were printed"
  )]
  SyncPlanMoved { shown: String, now: String },
  /// An id minted twice, with no merge in progress to hold the pulled side
  /// (ST0078 WP-05). The mid-merge renumber keeps the pulled record at the old
  /// id, and with no `MERGE_HEAD` there is no pulled record to keep.
  #[error("{id} was minted on both sides, but no merge is in progress")]
  RenumberNotMerging { id: String },
  /// A disk step of `sync --apply`'s repair of a merge failed (ST0078 WP-05).
  #[error("could not {step}")]
  SyncDiskStep {
    step: String,
    #[source]
    source: std::io::Error,
  },
  /// git ran and failed while `sync` read or staged (ST0078 WP-05).
  #[error(transparent)]
  Git(#[from] crate::gitstate::GitStateError),
  /// **THE WRITE WAS DERIVED FROM A RECORD THAT HAS SINCE MOVED** (issue 0206,
  /// vc ruled 2026-09-01: refuse and name, never retry).
  ///
  /// Lifted out of `StoreError` for the reason the whole of this vocabulary
  /// exists: as [`Self::Store`] it would render "could not update the runtime
  /// store" and drop its source, telling an operator to look at their disk over
  /// what is in fact two sessions working the same thread.
  #[error("{subject} changed while this command was running -- nothing was written")]
  RecordMovedUnderTheWrite { subject: String },
  /// A configured embedder failed. **Carried rather than folded into an empty
  /// answer**: a semantic query that silently becomes a lexical one tells an
  /// operator their question was answered.
  #[error(transparent)]
  Embed(#[from] crate::embed::EmbedError),
  #[error("could not update the runtime store")]
  Store(#[from] StoreError),
  // Issue 0447: the headline is the inner error's to choose, because one of
  // its variants is the store's and says the canon is intact.
  #[error("{}", .0.headline())]
  Ingest(#[from] IngestError),
  #[error("no export format named `{format}`")]
  NoSuchFormat {
    format: String,
    /// What the operator may actually ask for.
    emits: Vec<String>,
    /// Names the roster knows and declines -- reported so the next guess is
    /// not one of them, and NEVER offered as a choice.
    refused: Vec<String>,
  },
  /// A format the roster carries and deliberately will not emit.
  ///
  /// **Its own variant rather than [`FacadeError::NoSuchFormat`], because the
  /// two are opposite answers to the same question.** "There is no such
  /// format" invites the operator to look for the right spelling; this one
  /// says the spelling was right and the answer is still no. Collapsing them
  /// would send someone hunting for a name that does not exist to find.
  #[error("`{format}` cannot carry the canon back, so it is refused rather than written")]
  LossyFormat {
    format: String,
    because: &'static str,
    instead: &'static str,
  },
  /// A format that claims to round-trip and did not, on this estate.
  ///
  /// **This one is ours, and the message says so.** Every other refusal here
  /// tells an operator something to do; this tells them they have found a
  /// defect in the exporter, and it exists at all because the alternative was
  /// handing them a file that silently is not their data.
  #[error("`{format}` did not survive its own round-trip, so nothing was written")]
  ExportRoundTripFailed { format: String, detail: String },
  /// **The store's last load from canon did not finish, so the store may be
  /// older than the canon it is about to overwrite** (AC-03.13).
  ///
  /// Its own variant rather than folded into [`FacadeError::Ingest`], because
  /// the two are opposite in time and in subject. `Ingest` says "this canon is
  /// not readable, now"; this says "canon read fine, and the LAST attempt to
  /// take it did not land" -- reported by a verb reading in the other
  /// direction, which never touched the canon at all. An operator told the
  /// second when the first was meant would go looking for a defect in the file
  /// they are pointing at.
  #[error(
    "the store's last ingest was not accepted (recorded at {at}), so it may be older than the canon this would overwrite: {detail}"
  )]
  EgestFromRefusedIngest { at: String, detail: String },
  /// **A write that would reduce a populated face to zero** (AC-03.15).
  ///
  /// Separate from [`FacadeError::EgestFromRefusedIngest`] because the two are
  /// reached by disjoint routes and the criteria say so: that one requires an
  /// ingest to have been REFUSED, and in the live instance here **nothing was
  /// refused and nothing malfunctioned**. The store legitimately held zero,
  /// because a binary built from a reverted tree had ingested zero and reported
  /// success over it. The egest wrote exactly what it was given, correctly, by
  /// its own lights -- which is why a guard on the ingest's outcome cannot see
  /// this and why it needs a variant of its own.
  #[error("this would write an empty estate over one that is not empty: {evidence}")]
  EgestWouldEmptyTheEstate { evidence: String },
  /// **A canon file moved after the store last wrote or read it** (0260).
  ///
  /// Separate from [`FacadeError::EgestFromRefusedIngest`] because nothing was
  /// refused: the store was warm and correct, and then the canon moved under it
  /// -- a pull, a peer's commit -- with no intentd running to take it in. The
  /// daily driver never looks at the files, so only the egest can see it.
  #[error(
    "the canon on disk for {subjects} changed after the store last wrote or read it, so this would write the store's version over it"
  )]
  EgestFromStaleStore { subjects: String },
  /// An ingest pass found another connection's commit after each of its
  /// `INGEST_RENDERS` snapshots, so it wrote none of its renders (issue
  /// `0441`). `paths` names what the last one would have changed.
  #[error(
    "the store moved under each of this sync's {renders} renders, so it wrote none of them; the last would have changed {paths}"
  )]
  IngestOutpacedByWrites { renders: usize, paths: String },
  /// **A write that would replace an authored body with nothing** (Lamplight,
  /// 2026-08-26).
  ///
  /// `issues close 5` returned `ok:` and emptied 4934 bytes of the issue's
  /// prose. Nothing malfunctioned on the way: the model held an empty body, the
  /// write wrote what it was given, and **there was no error to suppress --
  /// which is why it was silent and why "be careful" could not have prevented
  /// it.**
  ///
  /// **Separate from [`FacadeError::EgestWouldEmptyTheEstate`], and the
  /// distinction is the reason this variant exists rather than reusing it.**
  /// That one asks whether a POPULATION went to zero, and its byte-shrink arm
  /// is deliberately gated on `canon.threads.is_empty()` -- because a file
  /// shrinking is ordinary (an edited-down objective, a removed work package)
  /// and a guard that refuses the ordinary path gets disabled rather than
  /// fixed. **This one is FIELD-level, so it can tell an author shortening
  /// prose from a field being emptied, which a byte comparison never can.**
  #[error(
    "this would replace the authored body of {subject} with nothing -- {had} byte(s) on disk, none in what is being written"
  )]
  WriteWouldEmptyAnAuthoredBody { subject: String, had: usize },
  /// The Intent install could not be located, so a template-reading verb has
  /// nothing to read. Added for [`Facade::agents_generate`] -- a facade gap
  /// closed on vc's 2026-08-30 ruling (c) -- and a NEW variant rather than a
  /// widened one, because every existing arm names a subject that is not this.
  #[error("cannot locate the Intent install")]
  Install(#[from] crate::install::InstallError),
  /// A root-file template could not be read or expanded. Same provenance as
  /// A board was asked for by a moniker the roster does not carry.
  ///
  /// **IT NAMES WHO IS REGISTERED RATHER THAN SAYING NO.** The two ways to
  /// reach this are a typo and an unregistered node, and the operator cannot
  /// tell them apart from a refusal that only repeats what they typed -- one is
  /// fixed by retyping and the other by registering, which are not the same
  /// next move. The roster is small by construction, so listing it costs a line
  /// and removes the guess.
  #[error("no node `{node}` is registered on this board; the roster carries {known}")]
  WbNodeNotRegistered { node: String, known: String },
  /// An entry body over the configured bound.
  ///
  /// **THE BOUND IS IN THE MESSAGE BECAUSE THE WRITER HAS TO ACT ON IT.** A
  /// refusal saying only "too long" leaves the author guessing how much to cut,
  /// and the guess is what produces a second refusal.
  #[error("the body is {bytes} bytes and the bound for `{node}` is {bound}")]
  WbBodyOverBound {
    node: String,
    bytes: usize,
    bound: usize,
  },
  /// One inbox already holds as many live messages as it may.
  ///
  /// **IT NAMES THE INBOX RATHER THAN THE RECIPIENT**, because the bound is per
  /// ordered pair: a full `cc -> vc` says nothing about `dc -> vc`, and a
  /// refusal naming only `vc` would send the reader to clear the wrong thing.
  #[error(
    "the inbox `{sender}` -> `{recipient}` holds {live} live message(s) and the bound is {bound}"
  )]
  WbInboxFull {
    sender: String,
    recipient: String,
    live: usize,
    bound: usize,
  },
  /// One node already holds as many live items of one kind as it may.
  ///
  /// **IT NAMES THE KIND, because the bound is per kind.** A full watch-out list
  /// says nothing about DOING, and a refusal naming only the node would send a
  /// reader to archive the wrong section.
  #[error("`{node}` holds {live} live `{kind}` item(s) and the bound is {bound}")]
  WbItemsFull {
    node: String,
    kind: String,
    live: usize,
    bound: usize,
  },
  /// A claim that is not a steel thread or work package address.
  #[error("`{claim}` is not a steel thread or work package address")]
  WbClaimMalformed { claim: String },
  /// A kind this verb will not write, because another verb owns it.
  #[error("`{kind}` items are not written by this verb")]
  WbKindHasItsOwnVerb { kind: String, verb: String },
  /// A standing directive written on a board that is not the hypervisor's.
  #[error("`directive` items are written on `hv`'s board, and `{node}` is not `hv`")]
  WbDirectiveOffHv { node: String },
  /// A hand-authored board that is not the hypervisor's carries a
  /// `## Standing directives` section.
  #[error(
    "`{node}`'s board carries a `## Standing directives` section, at {at}, and only `hv`'s board carries one"
  )]
  WbDirectivesOnAnotherBoard { node: String, at: String },
  /// A board write on a node whose board is still its hand-authored markdown.
  #[error("`{node}` is registered and not migrated, so its board is still the markdown on disk")]
  WbNotMigrated { node: String },
  /// A moniker already registered, named again with other values.
  #[error("`{node}` is registered as `{held_name}` ({held_role}), not `{name}` ({role})")]
  WbRegisteredDifferently {
    node: String,
    name: String,
    role: String,
    held_name: String,
    held_role: String,
  },
  /// A registration whose arguments disagree with the header of the board it
  /// found on disk (issue 0410).
  #[error(
    "`{node}`'s board at {file} names it `{header_name}` ({header_role}), not `{name}` ({role}); nothing was registered"
  )]
  WbRegisterDisagreesWithHeader {
    node: String,
    name: String,
    role: String,
    header_name: String,
    header_role: String,
    file: String,
  },
  /// A correction of a moniker that is not registered (issue 0417).
  #[error("`{node}` is not registered, so there is no name or role to correct")]
  WbCorrectUnregistered { node: String },
  /// A migration into a board that already holds rows.
  ///
  /// **THERE IS NO MERGE HERE THAT IS NOT A GUESS.** A second run cannot tell
  /// the rows its own first run wrote from a live write made since, so carrying
  /// again would either duplicate a board or silently skip a node's real work.
  /// It says what is standing, because that is what the operator has to look at
  /// before deciding which of the two happened.
  #[error("`{node}` already holds {items} item(s) and {messages} message(s)")]
  WbAlreadyCarried {
    node: String,
    items: usize,
    messages: usize,
  },
  /// A migration that meets an inbox from a sender the roster does not carry.
  ///
  /// **REFUSED BEFORE ANYTHING IS WRITTEN, SO THE RE-RUN STAYS OPEN.** Carrying
  /// the rest and naming the inbox would leave a board [`Self::WbAlreadyCarried`]
  /// refuses to carry again, with entries no command could then carry.
  #[error("`{node}`'s board holds inboxes from senders the roster does not carry: {inboxes}")]
  WbSendersNotRegistered {
    node: String,
    inboxes: String,
    senders: Vec<String>,
  },
  /// A roster registration that meets a board header lacking a field it reads.
  ///
  /// **REFUSED BEFORE ANYTHING IS WRITTEN, FOR THE WHOLE ROSTER** (issue 0424).
  /// The header form skipped such a board at exit 0, so the roster came out one
  /// node short with nothing to say so -- indistinguishable from a node nobody
  /// has created, which is the thing [`Facade::register_roster`] refuses an
  /// unreadable header to avoid. Every such board is named with the fields it
  /// lacks, so the refusal is the worklist.
  #[error("board header(s) lack a field `wb register` reads: {listing}")]
  WbHeaderIncomplete { listing: String },
  /// A migration that meets board content the model cannot carry.
  ///
  /// **REFUSED BEFORE ANYTHING IS WRITTEN, UNLESS THE DROP IS ASKED FOR BY
  /// NAME** (vc decision 20, issues 0403, 0406, 0407 and 0408). A carry that
  /// wrote the rest and exited 0 left a script unable to tell a complete carry
  /// from a lossy one, and [`Self::WbAlreadyCarried`] then refused the re-run
  /// that would have fixed it. Every unit is carried in `units`, with its
  /// address and its reason, so the refusal is the worklist.
  #[error(
    "`{node}`'s board offers {} unit(s) the model cannot carry, at {}",
    .units.len(),
    .units.iter().map(|u| u.at.as_str()).collect::<Vec<_>>().join(", ")
  )]
  WbUncarried {
    node: String,
    units: Vec<crate::wbmigrate::Uncarried>,
    /// Where `--drop-uncarried` would keep a verbatim copy: of the board's
    /// `wip.md`, and of every inbox holding one of the units (issue 0438).
    snapshots: Vec<String>,
  },
  /// A pre-migration snapshot already on disk that is not the file the carry
  /// keeps there.
  ///
  /// **THE CARRY WRITES EACH VERBATIM COPY AND NEVER OVERWRITES ONE.** A file
  /// at that path with other bytes is either an earlier board or inbox or a
  /// hand-placed document, and replacing it would lose exactly the record the
  /// snapshot exists to keep.
  #[error(
    "`{at}` already holds a pre-migration snapshot of `{node}`'s board, and it is not the file this carry keeps there"
  )]
  WbSnapshotInTheWay { node: String, at: String },
  /// No acting node: nothing said who is writing.
  ///
  /// **IT REFUSES RATHER THAN GUESSING, AND THE GUESS IT WILL NOT MAKE IS THE
  /// DANGEROUS ONE.** Defaulting to any node would write one node's words under
  /// another node's name, which is the single-writer invariant broken by the
  /// mechanism built to serve it, and it would be invisible to everyone
  /// afterwards.
  #[error("no acting node: nothing said which node is writing")]
  WbNoActingNode,
  /// `intent index resolve` was asked for level 3 where this build carries no
  /// resolver (ST0076 WP-05): for the language named, or for every language
  /// the project declares. `built` names the languages it does carry, or says
  /// there are none.
  #[error("this build has no level-3 resolver for {asked}")]
  NoResolver { asked: String, built: String },
  /// A search by target asked of an index where no language has stored a
  /// resolution (ST0076 WP-07, AC-07.2): an empty list would read as a target
  /// nothing references.
  #[error(
    "no language's references have been resolved in this index, so nothing can say which references name `{target}`"
  )]
  NothingResolved { target: String },
  /// A search by a target no resolved row names, where resolved targets end
  /// the same way (vc's refinement of AC-07.2). `near` is those targets, named.
  #[error(
    "no resolved reference names `{target}`; the resolved targets ending the same way are {near}"
  )]
  NoSuchTarget { target: String, near: String },
  /// [`Self::Install`], same add-don't-widen rule.
  #[error("could not render the root file")]
  RootFile(#[from] crate::rootfiles::RootFileError),
  /// `intent claude upgrade`'s canon apply failed, once the verb reached the
  /// facade (0351). By message: `CanonError` carries no `source` chain.
  #[error("could not apply canon: {0}")]
  Canon(crate::canon::CanonError),
}

/// What [`Facade::wb_migrate`] carried off one node's board, and what it would
/// not carry.
///
/// **THE CARRIED HALF IS PER ITEM AND SO IS THE REFUSED HALF**, which is the
/// whole of AC-14.9's accounting: a migration that reported `41 carried, 3
/// uncarried` would reconcile arithmetically while telling nobody which three
/// lines are now only in a markdown file somebody is about to stop reading.
/// Every entry on both sides carries its `<file>:<line>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WbMigration {
  pub node: String,
  /// Every item carried, in source order, each with its address; an item whose
  /// `coerced` is set was section prose rather than a bullet.
  pub items: Vec<crate::wbmigrate::SourceItem>,
  /// Inbox entries carried. They are not listed per item because each is
  /// already addressed by its own `authored_at` heading and lands whole.
  pub messages: usize,
  /// The `.history/` documents carried, by project-relative path: the node's
  /// fold archives AND the pre-migration copies below, every `*.md` under its
  /// `.history/`. A term in [`WbMigration::reconciles`], so it counts what it
  /// counts; the report labels it by the directory (issue 0499).
  pub snapshots: Vec<String>,
  /// The pre-migration copies this carry keeps, byte for byte, by
  /// project-relative path: the board always, and each inbox holding a unit the
  /// model cannot carry (issue 0438). Each was written by this run or was
  /// already present with the same bytes from an earlier attempt.
  ///
  /// **NAMED ON THEIR OWN BECAUSE THEY ANSWER THE CARRIER'S QUESTION** (issue
  /// 0499): a dropped unit is in one of these files and nowhere else. The report said `N snapshot(s)` for every `.history` document, while
  /// the refusal's remedy calls these copies snapshots, so on Conflab a node
  /// with one copy read `29 snapshot(s)` and nobody could tell where the
  /// dropped units went. They are already inside `snapshots`, so they are not a
  /// second term in the sum.
  pub kept: Vec<String>,
  /// Everything the source offered and this did not carry, named. **Non-empty
  /// only when the drop was asked for**: without it these units refuse the carry
  /// through [`FacadeError::WbUncarried`] before anything is written.
  pub uncarried: Vec<crate::wbmigrate::Uncarried>,
  /// `.history/` files the carry cannot read as documents and leaves where they
  /// are.
  ///
  /// **A SEPARATE LIST BECAUSE THEY ARE NOT A LOSS** (vc decision 20, issue
  /// 0409). Reported as `uncarried:`, eighteen files still on disk and tracked
  /// read as eighteen losses where there were none.
  pub left_in_place: Vec<crate::wbmigrate::Uncarried>,
  /// The node's files that are views Intent rendered and offer no unit, by
  /// project-relative path (issue 0439): named, and not kept, because every
  /// byte of them was written from rows. **NOT IN `offered`**, since a view the
  /// renderer wrote is the model's statement and not a line the board offered.
  pub rendered: Vec<String>,
  /// The heartbeat the node's hand-authored header CLAIMED, verbatim, or `None`
  /// where it claimed none: the row's `authored_at` after the carry.
  pub heartbeat_authored: Option<String>,
  /// The heartbeat the carry wrote, read back from the row.
  ///
  /// **A CARRY RESTAMPS THE NODE, AND THE REPORT SAYS SO** (issue 0497). The
  /// store stamps `heartbeat_at` at the write and keeps the header's claim in
  /// `authored_at` (`Store::carry_header`, AC-14.4 against AC-14.9). So a board
  /// untouched for weeks reads as live at its carry. Both values are reported
  /// because a reader of the board sees only this one.
  pub heartbeat_at_carry: String,
  /// Every unit the source offered: item-shaped board lines, inbox entries and
  /// `.history/` files. **Counted where each unit is dispatched**, never
  /// re-derived by a second walk that would be free to disagree.
  pub offered: usize,
}

impl WbMigration {
  /// Did everything the source offered end up on one side or the other?
  ///
  /// **THIS IS THE CHEAP HALF OF AC-14.9 AND IT IS NOT THE PROOF.** It holds by
  /// construction today, so what it guards is a later edit that dispatches a
  /// unit without counting it. The proof that nothing is dropped is an arm
  /// reading a fixture whose every line is known, finding each one either
  /// carried with its address or refused by name.
  pub fn reconciles(&self) -> bool {
    self.items.len()
      + self.messages
      + self.snapshots.len()
      + self.uncarried.len()
      + self.left_in_place.len()
      == self.offered
  }
}

/// What [`Facade::wb_pickup`] hands back: the acting node's own board, the
/// standing content every node honours, and every peer's header state with what
/// it does not show.
///
/// **THE PEERS ARE HEADERS AND NOT WHOLE BOARDS, DELIBERATELY.** A node at
/// session start needs to know who is active, on what, and how recently they
/// said so; handing it every peer's items and messages would make the cheap
/// question expensive and would put another node's inbox in front of a reader
/// who asked where everybody is. `wb show` is the door for one whole board.
///
/// **BUT A HEADER ALONE READ AS THE WHOLE SURFACE** (issue 0416). A node saw its
/// own watch-outs in full and no peer's at all, so the read looked complete; a
/// node booted, missed `hv`'s ruling that a flake was known, and spent a morning
/// diagnosing it. So `hv`'s standing kinds come back in full in
/// [`Pickup::standing`], and every peer carries the counts of what this read
/// leaves out, so the omission is stated rather than invisible.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Pickup {
  pub board: BoardRead,
  /// `hv`'s live items of the [`STANDING_KINDS`], in board order, and empty
  /// when `hv` holds none. **ABSENT when there is no `hv` board to read from, or
  /// `hv` is the acting node** and its own board above already carries them: an
  /// empty list says `hv` has no standing content, and it must not be the answer
  /// for a board this read never looked at.
  pub standing: Option<Vec<crate::model::WbItem>>,
  pub peers: Vec<PeerRead>,
}

/// The item kinds a booting node is obliged to read on `hv`'s board: what `hv`
/// has directed, ruled and warned of (issue 0416).
pub const STANDING_KINDS: [WbItemKind; 3] = [
  WbItemKind::Directive,
  WbItemKind::Watchout,
  WbItemKind::Decision,
];

/// One peer as `wb pickup` answers it: its header, and the live items this read
/// does not show, counted by kind.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct PeerRead {
  #[serde(flatten)]
  pub node: crate::model::WbNode,
  /// Live items on this peer's board that the pickup leaves out, one entry per
  /// kind present, in the board's section order. `wb show <peer>` lists them.
  pub unshown: Vec<KindCount>,
}

/// How many items of one kind.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct KindCount {
  pub kind: WbItemKind,
  pub count: usize,
}

/// One node's board as `wb show` and `wb pickup` answer it: the messages still
/// LIVE, and a count of the handled ones, unless the caller asks for all.
///
/// **THE HANDLED MESSAGES ARE COUNTED AND NOT LISTED BY DEFAULT**, for the
/// reason archived items are: a status read asks what still needs an answer,
/// and an inbox that only grows answers it with the archive. The count stays
/// in the answer so an empty archive and a left-out one read differently, and
/// `all` puts every message back with its state.
///
/// **A READ SHAPE, NOT A MODEL TYPE.** [`Board`] is the published schema and
/// the lossless extract, and it carries every row; this is what one read shows
/// of it, so it sits beside [`Pickup`] rather than in the model.
// Issue 0322: one `wb show` measured at hv's terminal was mostly the bodies of
// handled messages, with no live message on the board.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct BoardRead {
  #[serde(flatten)]
  pub board: Board,
  pub handled_count: usize,
}

impl BoardRead {
  pub fn of(mut board: Board, all: bool) -> Self {
    let handled_count = board
      .messages
      .iter()
      .filter(|m| m.state == crate::model::WbMessageState::Handled)
      .count();
    if !all {
      board
        .messages
        .retain(|m| m.state == crate::model::WbMessageState::Live);
    }
    Self {
      board,
      handled_count,
    }
  }
}

/// Refuse an address naming another project, before a door resolves anything
/// against this one (issue 0338 (i); hv's ruling 4 on AC-07.6, 2026-09-15).
///
/// **ONE CHECK, RUN FIRST BY EVERY DOOR THAT TAKES AN ADDRESS, AND A PURE
/// FUNCTION OF THE ADDRESS.** A refusal that ran after a lookup depended on
/// what this project carries: `edit` answered `no steel thread ST0009 in this
/// project` for `intent://other/threads/ST0009`, a true sentence about the wrong
/// project. intent-cli's `mcp::resource_read` and `browser_url` call it too, so
/// the facade, MCP and the terminal refuse in one voice.
pub fn require_local(address: &Address) -> Result<(), FacadeError> {
  match &address.authority {
    None => Ok(()),
    Some(authority) => Err(FacadeError::CrossProjectAddress {
      url: address.to_url(),
      authority: authority.clone(),
      local: Address {
        authority: None,
        ..address.clone()
      }
      .to_url(),
    }),
  }
}

impl crate::remedy::Remedy for FacadeError {
  /// What the operator should DO. Every variant has one, and no two variants
  /// share a remedy text -- a remedy that fits two different causes is telling
  /// the operator to guess which one they hit (AC-04.4).
  fn remedy(&self) -> String {
    match self {
      Self::SqlMoreThanOneStatement => "send a single SELECT -- the door is read-only and stateless, so a batch has nothing to sequence".to_string(),
      Self::SqlNoStatement => "give `--sql` a statement, eg `intent search --sql 'select id, title from thread'`".to_string(),
      Self::SqlUnterminated => "close the quote or the comment -- the door refuses text it cannot read the same way SQLite would".to_string(),
      Self::SqlWouldWrite => "read with SELECT; every change goes through a verb, which records what it did and why".to_string(),
      Self::SqlOutOfReach { .. } => "name the tables you want -- `intent schema` publishes what this store holds".to_string(),
      Self::SqlOverBudget => "narrow it -- add a WHERE, or a smaller --limit -- or ask the question with a verb. The door stops a statement that runs long rather than letting it hold the process".to_string(),
      Self::SqlLimitAboveCeiling { ceiling, .. } => format!(
        "ask for {ceiling} or fewer. A door that streamed everything would hand an agent a result nothing can hold"
      ),
      Self::StructuralTierNotAsked => "drop the tier filter or add `structural` to it; the lexical and semantic tiers answer a text query".to_string(),
      Self::SqlDidNotRun { .. } => "the words above are SQLite's own -- `intent schema` publishes the tables and columns this store holds".to_string(),
      Self::WbNodeNotRegistered { .. } => "check the spelling against the roster above; a node that is genuinely missing is put on the board by `intent wb register`, which reads the roster from each node's own `wip.md` header".to_string(),
      Self::WbBodyOverBound { bound, .. } => format!(
        "say it in {bound} bytes or fewer, or put the long form in the artefact it is about and leave a pointer here. A board carries the pointer; the account belongs where it will still be read next week"
      ),
      Self::WbInboxFull { sender, recipient, .. } => format!(
        "`{recipient}` clears it with `intent wb clear {sender}` once the messages are handled. The bound is per inbox, so this says nothing about anyone else's"
      ),
      Self::WbItemsFull { node, kind, .. } => format!(
        "`intent wb archive --node {node} {kind} <seq>` moves one to archived: the state change IS the archival, the row is never deleted, and what it said stays readable after it stops counting"
      ),
      Self::WbAlreadyCarried { node, .. } => format!(
        "read what is there first -- `intent wb show {node}` -- because this refuses rather than guessing whether those rows are an earlier carry or work written since. A board carried by mistake is emptied by rebuilding the store from canon; one carrying real work is already past the markdown era and needs no migration"
      ),
      Self::WbSendersNotRegistered { node, senders, .. } => format!(
        "nothing was carried. Register each sender that is a node on this project -- {} -- then re-run `intent wb migrate {node}`; an inbox from a node that is not on this project moves out of `intent/whiteboard/{node}/` first",
        senders
          .iter()
          .map(|s| format!("`{}`", crate::model::register_form(s)))
          .collect::<Vec<_>>()
          .join(", ")
      ),
      Self::WbHeaderIncomplete { .. } => "nothing was registered. Give each named header the `key: value` line it lacks -- `node`, `name` and `role` are all read from the header block -- or move a directory that is not a node out of `intent/whiteboard/`, then re-run `intent wb register`".to_string(),
      Self::WbClaimMalformed { .. } => format!(
        "claim {} -- the spelling `.intentfiles` already uses. A claim names what the board can point at, so free text here would be a claim nothing can resolve",
        crate::model::CLAIM_ADDRESS_FORMS
      ),
      Self::WbRegisteredDifferently {
        node,
        name,
        role,
        held_name,
        held_role,
      } => format!(
        "registering again never changes a node. If `{node}` is this node and `{held_name}` ({held_role}) is wrong, `intent wb register {node} --name \"{name}\" --role {role} --correct` replaces it with `{name}` ({role}) and keeps its board; a participant who is not that node needs a moniker of its own"
      ),
      Self::WbRegisterDisagreesWithHeader {
        node,
        header_name,
        header_role,
        file,
        ..
      } => format!(
        "if the header is right, register with its values: `intent wb register {node} --name \"{header_name}\" --role {header_role}`. If the arguments are right, correct `name:` and `role:` in {file} first, then register again"
      ),
      Self::WbCorrectUnregistered { node } => format!(
        "`--correct` changes a node that exists and never creates one: `{}` registers it",
        crate::model::register_form(node)
      ),
      Self::WbNotMigrated { node } => format!(
        "`intent wb migrate {node}` carries its hand-authored board into the model first. A board write renders the board from the store, so writing before the carry would replace the markdown with a render of a board that holds none of it"
      ),
      Self::WbKindHasItsOwnVerb { kind, verb } => format!(
        "`{verb}` writes a `{kind}`. One door per kind is deliberate: what a decision is FOR is stated once, beside the verb that writes one"
      ),
      Self::WbDirectiveOffHv { .. } => "a standing directive is the hypervisor's: an instruction every node honours, kept under the protocol's `## Standing directives` section on `hv`'s board, where `intent wb add directive <text> --node hv` writes it on hv's word. A call this node made itself is a decision: `intent wb decide <text>`".to_string(),
      Self::WbUncarried {
        node, snapshots, ..
      } => format!(
        "nothing was written. Each unit is named on an `uncarried:` line with its reason: give it a home the model carries -- a bullet under a section the protocol names, or a heading with nothing after its kind word -- and re-run `intent wb migrate {node}`. Or carry the rest without them: `intent wb migrate {node} --drop-uncarried`, which first keeps byte-for-byte copies, as snapshots, at {}",
        snapshots
          .iter()
          .map(|at| format!("`{at}`"))
          .collect::<Vec<_>>()
          .join(", ")
      ),
      Self::WbSnapshotInTheWay { node, at } => format!(
        "nothing was written. Read `{at}`: if it is an earlier copy worth keeping, move it to another name under `.history/`, where the carry takes it as a snapshot of its own, then re-run `intent wb migrate {node}`"
      ),
      Self::WbDirectivesOnAnotherBoard { node, .. } => format!(
        "nothing was written, so this carry can run again once those lines have a home. A directive still in force belongs on `hv`'s board: under `## Standing directives` in `intent/whiteboard/hv/wip.md` before `hv` is carried, or through `intent wb add directive <text> --node hv` after. A call `{node}` made itself belongs under `## Decisions` on its own board"
      ),
      Self::WbNoActingNode => "say who is writing: `--node <moniker>`. `intent wb status` lists the roster".to_string(),
      Self::NoResolver { built, .. } if built.is_empty() => "nothing was run. This build carries no level-3 resolver for any language yet, so references answer at levels 1 and 2: `intent search --kind ref <name>`".to_string(),
      Self::NoResolver { built, .. } => format!(
        "nothing was run. Name a language this build resolves: `intent index resolve --lang <lang>`, for {built}"
      ),
      Self::NothingResolved { .. } => "nothing was answered. A person resolves the declared languages' references by running `intent index resolve`, which runs the project's build; the search answers once a run has stored".to_string(),
      Self::NoSuchTarget { .. } => "ask for one of them by its whole name, as a hit's `target` prints it: `intent search --target <target>`".to_string(),
      // The `why` already carries the rule that refused; a remedy repeating it
      // would be the doubled rendering `IngestError::Refused` documents.
      // **THE REMEDY IS THE CORRECTED PATH WHERE ONE EXISTS**, because the
      // operator who typed the wrong spelling is the operator who has to type
      // the right one, and a remedy that describes the rule leaves them to
      // derive it. Where no corrected form exists the remedy says what a
      // well-formed path IS rather than restating the fault.
      // **THE REMEDY NAMES THE VERB THAT REMOVES, AND IT IS NOT THIS ONE.**
      // `organize` reconciles an estate: it names every removal before making
      // it and asks on a terminal. Offering a flag here would put a removal
      // inside a verb whose contract is that files come into existence.
      Self::RealisationWouldRemove { .. } => (
        "`intent organize` previews what the estate's declaration implies, and `intent organize --apply` performs it -- naming every file it will remove first. If the file is yours and nothing in the store carries it, move it out of the thread's directory"
      )
      .to_string(),
      // **THE REMEDY IS A SEQUENCE, NOT A FLAG.** Naming `--overwrite` alone
      // would hand an operator the destructive route as the answer to a
      // question they have not looked at yet; the copy comes first.
      Self::HydrationWouldOverwrite { id, .. } => format!(
        "copy the version on disk somewhere else first -- then `intent st hydrate {id} --overwrite` discards it and writes what the store renders. If the version on disk is the one you want, it belongs in canon: make the change through the CLI so the store carries it"
      ),
      Self::NoSuchAttachment { thread, .. } => format!(
        "the paths {thread} carries are its canon's `attachments`, in `intent/.canon/st/{thread}.json` -- name one of those, relative to the thread's own directory"
      ),
      Self::NoSuchRelatedTarget { thread, .. } => format!(
        "run `intent st list` to see the threads this project has, and relate {thread} to one of those -- a link to a thread that does not exist is the dangling reference `intent doctor` reports"
      ),
      Self::RelatedToItself { thread } => format!(
        "name a different thread as the target -- {thread}'s links name the threads beside it"
      ),
      Self::NoSuchRelated { thread, .. } => format!(
        "the links {thread} carries are its canon's `related`, in `intent/.canon/st/{thread}.json`, and its `info.md` lists them under `## Related Steel Threads` -- name one of those"
      ),
      Self::NotCarried { thread, path } => format!(
        "a thread carries a document once it is attached -- `intent st attach {thread} {path} --from <file>` writes one into the store"
      ),
      Self::CrossProjectAddress {
        authority, local, ..
      } => format!(
        "address this project's own artefact as `{local}` -- the same address with no authority -- and reach `{authority}`'s artefacts from inside that project; no door here resolves another project's address"
      ),
      Self::AttachmentPathNotInThread { thread, fault, .. } => {
        use crate::project::PathFault;
        match fault {
          PathFault::RepoRelative { corrected, .. } | PathFault::Unnormalised { corrected } => {
            format!("write `{corrected}` -- an attachment path is relative to the thread's own directory, not to the repository root")
          }
          PathFault::Empty => format!(
            "name the file's place inside {thread}, as `intent st attach {thread} <path-inside-the-thread> --from <file>`"
          ),
          PathFault::Absolute | PathFault::Escapes => format!(
            "give the path the file should have INSIDE {thread} -- `--from` is where the bytes are read from, and this argument is where they are recorded"
          ),
        }
      }
      // **THE REMEDY NAMES THE CONSEQUENCE AND THE WAY OUT, IN THAT ORDER**,
      // because the operator's instinct here is that the tool is being fussy
      // about a status they are entitled to set. What makes it not fussy is the
      // blast radius: an `absent_at` finding refuses EVERY node's commit, and
      // there is no verb that returns the row to `to-write` afterwards.
      Self::VerdictCitesAbsentFile { st, at, path, .. } => {
        format!(
          "write `{path}` first, then set the verdict -- or point the row at the file that exists with `intent at edit {st} {at} --file <path>`. \
           This is refused rather than warned because the finding it would create refuses EVERY commit in this repository, not just yours, \
           and `intent at` has no spelling that returns a row to `to-write` afterwards."
        )
      }
      Self::RowBreaksContract { st, at, .. } => {
        format!(
          "`intent at lint {st}` names each finding; `intent ac list {st}` shows the criteria that exist. \
           Create a missing criterion first with `intent ac new {st} <AC-ID> --text \"...\"`, or point the row at one that exists with `intent at edit {st} {at} --covers <AC-ID>`"
        )
      }
      Self::WriteNotAddressable { .. } => {
        "`PUT` json to a caller-assigned id (an AC or an AT); everything else is a \
         `POST` to the collection address"
          .to_string()
      }
      // **THE ASYMMETRY IS THE MECHANISM, NOT AN OVERSIGHT** (issue 0207, vc
      // ruled (c) 2026-09-02, rider 1). `at edit --note` is the explicit door
      // and it carries NO shrinkage refusal, because a door that refuses is
      // not a door: guarding it too would leave no legal way to fold a bloated
      // note, which is the build 0207's own evidence disproves. `at edit`'s
      // published contract is already *a field you do not name is a field it
      // does not change*, so naming `--note` there IS the deliberate act.
      Self::NoteWouldBeLost { .. } => {
        "extend the note instead -- pass a `--note` that contains the existing text -- or, to \
         replace it deliberately, name the field on the verb that exists for it: `intent at edit \
         <ST> <AT> --note \"...\"`"
          .to_string()
      }
      Self::ValueNotRecordable { .. } => {
        "restate the value in the form the message names -- the verb and the field are right, \
         and nothing was written"
          .to_string()
      }
      // **THE DOOR, NOT THE VALUE** (issue 0503): the date is recordable and
      // the closing verb is not where a closed thread's date is restated, so
      // the remedy is the setter, with the date the caller already typed.
      Self::CompletionDateNotRestated { st, given, .. } => format!(
        "`intent set {st} completed {given}` restates the date on a thread that is already closed \
         -- a closing verb records one only on the move that closes it, and nothing was written"
      ),
      Self::VerdictWrongForKind { st, at, .. } => {
        format!(
          "a test row holds to-write, red or green and a non-test row holds n/a. Creating it, `intent at new` \
           starts the row at its kind's entry: to-write, or n/a with `--kind non-test`. On an existing row, a test row takes \
           `intent at red|green {st} {at}` and a non-test row takes `intent at na {st} {at}`; if the row's kind is \
           what is wrong, re-kind it with `intent at edit {st} {at} --kind <test|non-test>`"
        )
      }
      Self::OpenWorkPackages { st, .. } => {
        format!(
          "finish each with `intent wp done {st}/<NN>` or drop it with `intent wp cancel {st}/<NN> --reason \"...\"`, then close {st} again"
        )
      }
      Self::FileOnANonTestRow { st, at, file } => format!(
        "re-kind it in the same call, where a row recording n/a re-enters at to-write: `intent at edit {st} {at} --kind test --file {file}`"
      ),
      Self::FieldNotWritable { .. } => {
        "go to the door the refusal names: a lifecycle verb for a field a state machine owns, \
         the member's own address for a collection, and the list's own verbs for a list that has them"
          .to_string()
      }
      Self::NoSuchThread { .. } => {
        "run `intent st list` to see the threads this project has".to_string()
      }
      Self::ThreadExists { id } => {
        format!("pick a different id, or work on the existing one with `intent st show {id}`")
      }
      Self::NoSuchWorkPackage { st, .. } => contract::see_the_work_packages(st),
      Self::NoSuchCriterion { st, .. } => {
        format!("run `intent ac list {st}` to see the criteria in its contract")
      }
      Self::NoSuchTest { st, .. } => {
        format!("run `intent at list {st}` to see the tests in its contract")
      }
      Self::CriterionExists { st, ac } => format!(
        "nothing was written and {ac} still holds the text, kind and state it held. To reword \
         it, `intent ac edit {st} {ac} --text \"...\"` -- which changes the sentence and leaves \
         its satisfaction alone, where a create would have reset both. To add a new one, \
         `intent ac list {st}` shows which ids are taken"
      ),
      Self::TestExists { st, at, kind } => {
        // **THE REMEDY READS THE ROW'S KIND BECAUSE `at edit` DOES NOT.** Both
        // halves of that sentence are load-bearing: `at edit` accepts either
        // field on either kind and warns about neither (0146), so the only
        // thing standing between an operator and a row carrying both `file`
        // and `prose` is whether this string named the right flag.
        let (held, flag) = match kind {
          AtKind::Test => ("its file", "--file <path>"),
          AtKind::NonTest => ("its prose", "--prose \"...\""),
        };
        format!(
          "nothing was written and {at} still holds {held}, coverage, status and note. To \
           re-cite it, `intent at edit {st} {at} {flag}` -- which keeps the note that a create \
           would have eaten. To add a new one, `intent at list {st}` shows which ids are taken"
        )
      }
      // **THE ARM THAT BLOCKED SAYS WHAT TO DO, NOT THIS VARIANT** (issue
      // 0526). A fixed remedy here spoke of "the remaining criteria" under an
      // empty contract, beneath a gate line saying there were none.
      Self::GateBlocked { remedy, .. } => remedy.clone(),
      Self::ComputedSatisfaction { ac } => format!(
        // **`at set` NEVER EXISTED IN v3.** The family is
        // list/lint/green/red/na/new/edit, and every one of those verbs takes
        // `<STID> <ATID>` -- so the old string was wrong twice, naming a verb
        // the binary does not have and a grammar carrying neither argument.
        // Reached from `ac satisfy` on any test-backed criterion, which is a
        // shipped path rather than dead prose. Found by dc and verified
        // independently by cc, 2026-09-01; the corpus gap that let it stand is
        // AC-06.11's and is cc's to close.
        "set the covering test green instead -- `intent at green <STID> <ATID>` -- or make \
         {ac} a non-test criterion with named evidence"
      ),
      Self::NotOffScope { verb, wanted, .. } => {
        format!("{verb} applies only to a {wanted} criterion")
      }
      Self::ViewsNotWritten { .. } => {
        // NOT bare `intent sync`, and that instruction was in this remedy
        // until it was checked. The disk -> db direction reads canon from the
        // files and replaces the store from them, so running it here would
        // overwrite truth with the stale projection and destroy the very
        // change this error says is safe. A remedy that names a data-loss
        // command is worse than no remedy.
        //
        // It now names a REPAIR rather than a wait, and that is a second edit
        // for a second reason: until AC-03.9 landed `sync_to_disk` there was
        // no db -> disk direction, so "the files are rewritten by the next
        // successful mutation" was the honest answer. It stopped being the
        // best one the same day, which is the same class as the first edit --
        // a remedy outliving the estate it was written against.
        RERENDER_REMEDY.to_string()
      }
      Self::IllegalTransition { verb, legal, .. } => {
        format!(
          "`{verb}` is declared only from: {legal}. The machine has no terminal states, so there IS a route from here -- move through the states rather than around them"
        )
      }
      Self::DescopeTargetMissing { to, .. } => {
        format!(
          "descoping moves a requirement to a thread that will hold it, so {to} has to exist first -- create it with `intent st new`, or use `intent ac withdraw` if the requirement is going away rather than moving"
        )
      }
      Self::ReasonRequired { verb } => {
        format!(
          // **THE EVENT-LOG CLAUSE IS GONE, BY hv's RULING (AC-03.12).**
          //
          // It read "and in the event log as part of the decision", and before
          // that "which is what lets anyone reconstruct why later" -- a
          // justification citing a store NO SHIPPED VERB CAN READ. `intent
          // --help` declares no `events`, no `log`, no `history`, and
          // `ingest.rs` never mentions the field so `search` does not reach it
          // either. **A refusal that argues from a capability the operator
          // cannot exercise is arguing from nothing**, and the operator cannot
          // tell the difference from the message.
          //
          // The remaining sentence is the one the tool can keep: the reason IS
          // on the entity, and it IS rendered.
          "give `{verb}` a reason. It is recorded on the entity as the reason for its CURRENT state"
        )
      }
      Self::DescopeTargetRequired { ac } => {
        format!(
          "run `intent ac descope <thread> {ac} --to <thread>` with the thread that will hold the requirement -- use `intent ac withdraw` instead if it is going away rather than moving"
        )
      }
      Self::EvidenceRequired { ac } => {
        format!(
          "run `intent ac satisfy <thread> {ac} --evidence \"<what you checked>\"` -- a non-test criterion has no test to run, so the evidence IS the verification: cite the commit, the command, or the review that settled it"
        )
      }
      Self::NotSatisfied { .. } => {
        "run `intent ac list <thread>` to see which criteria carry evidence -- only a non-test criterion that was satisfied can be unsatisfied".to_string()
      }
      Self::AlreadyFiatClosed { undo, .. } => {
        format!(
          "run `{undo}` first if you mean to close it again for a different reason -- a fiat close is reversible and the record of the first one stays in the event log, so reinstating does not erase that it happened"
        )
      }
      Self::OffScope { undo, ac, verb, .. } => {
        format!("run `intent ac {undo} <thread> {ac}` first if you mean to {verb} it -- recording evidence for a requirement nobody is working on is the bookkeeping descope replaced")
      }
      Self::WrongOffScopeState { verb, ac, .. } => {
        format!("run `intent ac {verb} <thread> {ac}` instead -- a descoped requirement still exists on another thread, and a withdrawn one does not exist at all")
      }
      // `NEAR` is not in the list: `hello NEAR` is answered, not refused (driven
      // on the pair and on a bare fts5 table, issue 0443), and naming it sent
      // the reader after a fault that is not one.
      Self::BadQuery { .. } => {
        "bare words and punctuation are searched literally, so this was refused as an FTS5 EXPRESSION -- check for an unbalanced `(` or `)`, or an `AND`/`OR`/`NOT` with nothing on one side of it".to_string()
      }
      // **IT NAMES ONLY WHAT WAS DRIVEN TO WORK.** Over an fts5 leaf page
      // overwritten with random bytes, `intent index rebuild` cured the search
      // and `intent index status` reported sizes at rc=0 without a word about
      // the damage -- so pointing at status for the index's "health" would have
      // been a remedy that cannot deliver, the defect this variant removes.
      //
      // **THE INDEX REMEDY ONLY WHERE THE INDEX IS THE SUSPECT.** A busy store
      // is not a damaged one (issue 0436) and a schema mismatch is not fixed by
      // a rebuild, so every cause the store already has a remedy for keeps it:
      // a rebuild offered for a lock another process holds is a remedy that
      // cannot work, which is the defect this variant exists to remove.
      Self::SearchUnanswerable { cause, .. } => match cause {
        StoreError::Sqlite(_) if !cause.is_busy() => "the query is not at fault -- the index could not be read to answer it. `intent index rebuild` rebuilds it from the tree, and the query will answer once it has".to_string(),
        other => other.remedy(),
      },
      // The alternatives are NAMED here rather than pointed at: the faces are
      // generated from the types and cost nothing to list, and a remedy that
      // says "run the command again differently" when it could say the answer
      // is a round trip for nothing.
      Self::NoSuchFace { .. } => format!(
        "one of: {} -- or run `intent schema` with no argument to print every face under its own banner",
        crate::faces::face_names().join(", ")
      ),
      // `--kind all` rather than a bare list: the default bucket is OPEN, so a
      // remedy without it sends someone looking for a closed issue in a list
      // that cannot contain one, and they conclude it is gone.
      Self::NoSuchIssue { .. } => {
        "run `intent issues list --kind all` to see every issue this project has, closed ones included".to_string()
      }
      Self::MalformedIssueId { .. } => {
        "an issue id is four digits, eg ISSUE:0001 in intent/.intentfiles".to_string()
      }
      // The remedy names the three that DO resolve rather than the ten that
      // do not: a caller here asked for something reasonable and needs the
      // set they can ask for, and the refusing set is both longer and less
      // useful. Derived from the declaration so it cannot drift from it.
      Self::EntityUnserialisable { .. } => {
        "this is a fault in Intent rather than in the address or the project; the id is not the problem and retyping it will not help"
          .to_string()
      }
      Self::NoFormForEntity { .. } => format!(
        "this project declares forms for {}. A criterion, a test or an attachment is read through its thread",
        crate::form::Loaded::load()
          .map(|l| l
            .forms()
            .iter()
            .map(|f| format!("`{}`", f.entity))
            .collect::<Vec<_>>()
            .join(", "))
          .unwrap_or_else(|_| "no entity, because the form declaration itself will not load".to_string())
      ),
      // Delegated for the same reason the message is: the arithmetic that
      // names the two honourable values either side belongs with the rule.
      // Delegated, because the remedy DIFFERS by state: below the v2.19.0
      // floor it is the two-hop, and naming the v3 migrator there would send
      // half the operators who read it to a command that refuses them.
      Self::Unmigrated(pending) => pending.remedy(),
      // Same delegation and the same reason: `Pending` owns the two-hop, and
      // this variant differs from `Unmigrated` in its MESSAGE, not its cure.
      Self::BelowMigrationFloor(pending) => pending.remedy(),
      // NOT delegated and NOT shared with the dirty arm: the two states have
      // different cures, and one of them names a command that would not help
      // the other. Both causes are named because this variant deliberately
      // does not claim to know which one it met.
      Self::MigrationWithoutGit => {
        "put the project under git before migrating -- `git init && git add -A && git commit` -- or install git if it is missing. The documented rollback is `git revert <the migration commit>`, and there is nothing to revert without a repository".to_string()
      }
      Self::MigrationOverDirtyTree { paths } => {
        let each: Vec<String> = paths.iter().map(ToString::to_string).collect();
        format!(
          "commit or stash these first, then re-run -- the migration is one visible commit and a revert of it must not take your work with it:\n  {}",
          each.join("\n  ")
        )
      }
      Self::Write { .. } => {
        "check permissions and free space on the project directory, then retry -- nothing was changed".to_string()
      }
      // THIS REMEDY USED TO SAY "delete `intent/.cache/intent.db` and retry".
      // Under the reversed D01 that instructs the operator to delete the
      // SOURCE OF TRUTH, on any store error at all -- the third data-loss
      // instruction found in this estate today, and the one shown most often.
      // It was true when the DB was a rebuildable index; it became a
      // destructive default the moment the DB stopped being one.
      //
      // It deliberately names no recovery COMMAND: `intent events ingest` is
      // ruled and unbuilt, and naming a command that does not exist is the
      // same defect one step further on.
      //
      // **AND IT NO LONGER SPEAKS FOR EVERY STORE FAILURE.** One remedy for
      // the whole of `StoreError` is the same collapse this method exists to
      // prevent, one level down: a schema-version refusal and a failed
      // statement are different problems with different actions, and both were
      // getting this sentence. `StoreError::remedy` distinguishes them, and
      // this variant now asks rather than answers -- the store knows which of
      // its failures happened and this does not.
      Self::NotEditable { author_with, .. } => format!("author it with {author_with}"),
      Self::RenumberTargetTaken { .. } => {
        "nothing was renumbered. Pick an id nothing holds: `intent st list --status all` and \
         `intent issues list` show what the store holds, and a canon file or directory on disk \
         that the store does not hold is usually a pull not yet loaded, which `intent sync \
         --apply` loads without discarding anything the store holds"
          .to_string()
      }
      Self::RenumberDiskStep { .. } => {
        "nothing was renumbered: the store, canon, the realised files and `.intentfiles` are as \
         they were. Clear the filesystem cause and run the renumber again"
          .to_string()
      }
      Self::SyncPlanMoved { .. } => {
        "nothing was run. Run `intent sync` to read the plan as the tree stands now, then \
         `intent sync --apply --plan <digest>` with the digest it prints"
          .to_string()
      }
      Self::RenumberNotMerging { id } => format!(
        "nothing was renumbered. With no merge in progress `intent st renumber {id} <new>` (or \
         `intent issues renumber`) moves this clone's record before you merge again"
      ),
      Self::SyncDiskStep { .. } => {
        "the merge is partly repaired and the steps before this one stand. Clear the filesystem \
         cause, then run `intent sync` to read what is still unmerged and plan from there; `git \
         status` shows what has been staged"
          .to_string()
      }
      Self::Git(cause) => crate::remedy::Remedy::remedy(cause),
      Self::IssueExists { number } => format!(
        "nothing was written and issue {number:04} still holds what it held. Re-run `intent \
         issues add` to take the next free number, or `intent issues show {number:04}` to see \
         what is already there -- a create that overwrote would have destroyed that record \
         silently, which is what this refuses"
      ),
      // **NO RETRY FLAG IS OFFERED, AND THAT IS THE RULING RATHER THAN AN
      // OMISSION** (vc, 2026-09-01). Re-applying the edit automatically would
      // derive it from a record the operator never saw, which is issue 0206
      // wearing a success message -- a fix must not be observationally
      // identical to the defect.
      //
      // **AND IT NAMES NO COMMAND TO INSPECT THE RECORD, DELIBERATELY.** The
      // subject reads "thread ST0056", and the verb for that is `intent st
      // show` -- a mapping this arm would have to keep, and get wrong for every
      // entity added later. A refusal owes the operator their next MOVE, which
      // is here, not a command it half-remembers.
      Self::RecordMovedUnderTheWrite { subject } => format!(
        "nothing was written, so nothing needs undoing -- {subject} still holds what the other \
         write put there. Re-run the command: it reads the current record first"
      ),
      Self::NothingToChange { offered, .. } => format!(
        "name at least one of {} -- nothing was written, so nothing needs undoing",
        offered.join(", ")
      ),
      Self::NoSuchEditable { present, .. } => {
        format!("this artefact carries: {}", present.join(", "))
      }
      // **TWO ROUTES REACH THIS VARIANT AND THEY NEED OPPOSITE REMEDIES**
      // (0514, found by ic driving `intent edit intent:///issues/0514 --path`,
      // 2026-09-22). An ISSUE is an artefact -- `.intentfiles` names it and
      // `hydrate` realises it -- so it arrives only from `edit`, which refuses
      // it because its one file is a generated view. Sending it to "address an
      // artefact instead" told the operator to type what they had just typed,
      // and "has no files of its own" contradicted the `why` printed one line
      // above. Every OTHER form arrives because it names no artefact at all --
      // measured, the collection `threads`, a whiteboard `node` or `node-inbox`,
      // and an `event`; a wp, a criterion or an attachment realises its thread
      // and never reaches here, so the remedy must not speak to them.
      //
      // The remedy this replaced said an issue had NO realised form, and that
      // was true until ST0069 AC-01 gave it one; it was written when the
      // offered target was a thread alone.
      Self::NotHydratable { form: "issue", .. } => "an issue's prose is changed through the record its view is rendered from: read it with `intent issues show <NNNN>`, and replace its body with `intent set intent:///issues/<NNNN> body --from <file>`".to_string(),
      Self::NotHydratable { form, .. } => format!(
        "address an ARTEFACT instead -- a steel thread or an issue, the two things `.intentfiles` names. The form `{form}` names neither, so there is nothing for realisation to create."
      ),
      Self::NoManifestToUnlistFrom { path, .. } => format!(
        "write one first with `intent organize --default`, which declares the open threads; then re-run. Removing a thread's files is a change to a list that has to exist before it can be changed, and {path} does not."
      ),
      Self::DehydrationRefused { .. } => "each line above names a file whose bytes the store cannot be shown to hold. `intent doctor` names the difference; if the copy on disk is the one you want, take it into canon with `intent sync --to-store`, and if the store is right, the file is a hand edit to discard. Nothing was removed, so nothing needs undoing.".to_string(),
      // The embedder's own remedy, which names the configuration or the
      // endpoint; restating it here would be a second answer to one question.
      Self::Embed(cause) => cause.remedy(),
      Self::Store(cause) => cause.remedy(),
      // Delegated for the same reason: `organize` knows which of its four
      // refusals happened and this does not.
      Self::Organize(cause) => cause.remedy(),
      Self::Realise(cause) => cause.remedy(),
      // **DELEGATED, LIKE EVERY NEIGHBOUR, AND THE HAND-WRITTEN VERSION HERE
      // WAS A DEFECT I DEFENDED IN A COMMENT.** `Store`, `Organize` and
      // `Realise` all delegate on the stated ground that the source knows which
      // of its refusals happened and this does not. This arm did not, and its
      // source is the most specific of the four: every `IntentfilesError`
      // carries the LINE NUMBER and the offending text.
      //
      // **THE `#[error]` STRING WAS FIXED, SO EVERY MANIFEST FAULT RENDERED
      // IDENTICALLY** -- an unknown sigil on line 12 and a malformed id on
      // line 40 both came out as `could not read the realisation manifest`.
      // The doc comment on the variant itself says folding it into a generic
      // read failure "would drop the one field that makes it actionable", and
      // the variant did precisely that, one line below the sentence forbidding
      // it. `.intentfiles`'s own header promises the line number.
      //
      // **AND I MADE IT WORSE BY UNDERSCORING THE BINDING TO SILENCE THE
      // WARNING.** `unused variable: cause` was not noise; it was the compiler
      // reporting that the cause reached nothing, which is the defect. I wrote
      // a comment claiming the cause was "already the DISPLAY body of this
      // variant" so interpolating it would double the line -- **that was false
      // by inspection: the format string contained no `{0}`.** Third time today
      // a comment asserted what the code did not do, and the first two were
      // other people's.
      Self::Intentfiles(cause) => cause.remedy(),
      Self::Install(cause) => cause.remedy(),
      Self::RootFile(cause) => cause.remedy(),
      Self::Canon(cause) => match cause {
        crate::canon::CanonError::Unreadable { path, .. } => format!(
          "check that {} is readable, then re-run `intent claude upgrade --apply`",
          path.display()
        ),
        crate::canon::CanonError::Unwritable { path, .. } => format!(
          "check that {}'s directory is writable, then re-run `intent claude upgrade --apply`; what it already wrote is canonical, so the re-run converges",
          path.display()
        ),
        crate::canon::CanonError::RootFile(_) => "a root-file template in the Intent install could not be rendered; restore the install's `lib/templates/`, then re-run `intent claude upgrade --apply`".to_string(),
      },
      // **THE REMEDY THIS REPLACES STATED hv's RULE BACKWARDS, AND IT WAS THE
      // FIRST MESSAGE A NEW v3 PROJECT SHOWED ANYBODY** (AC-04.7 arm (c)). It
      // read *"an absent manifest declares nothing, so `organize` would read
      // the whole estate as undeclared"* -- the PRE-REVERSAL reading, four
      // files from `Realised::declares`, whose own comment is **ABSENT IS NOT
      // EMPTY** and whose code answers `true` for everything. Absent means
      // realise everything; the message said absent means realise nothing.
      // **So it did not merely fail to consult the model -- it TAUGHT the
      // reversed rule to the one person with no other source.**
      //
      // **AND THE TEXT IS NOW HONEST BECAUSE THE STATE IS UNREACHABLE, NOT
      // BECAUSE IT WAS REWORDED.** No site maps `NotFound` here any more:
      // `manifest_for_action` answers `NothingSaid`, `edit`'s pin step and
      // `edit_list` both no-op. What is left is a file that IS there
      // and cannot be read -- a permissions fault, a directory in its place, a
      // bad mount -- which is a repair to the MACHINE and never to the estate.
      // A remedy telling that operator to `create` the file would send them to
      // overwrite something they cannot currently read.
      Self::ManifestUnreadable { path, .. } => format!(
        "`{path}` is there and could not be read -- check its permissions and the mount it sits on. This is not the absent case: a MISSING manifest is not an error at all, because nobody having said means everything stays realised."
      ),
      // Delegated to the parse error, which knows WHICH line and WHY, exactly
      // as `Intentfiles` does -- the path this variant adds is context for the
      // message, not a substitute for the remedy.
      Self::ManifestMalformed { cause, .. } => cause.remedy(),
      // Delegated, for the same reason `Store` is: `Blocked` knows whether the
      // estate needs repairing under v2 or whether the migrator itself failed,
      // and those are different actions for different people.
      Self::MigrationBlocked(cause) => cause.remedy(),
      // **THE REMEDY IS TO RUN IT AGAIN, AND SAYING SO IS ONLY HONEST BECAUSE
      // RE-RUNNING IS NOW IDEMPOTENT.** It was not until canon-wins landed: a
      // re-run absorbed the renderer's own output and the estate grew every
      // time, so this sentence would have sent an operator to corrupt their
      // project by following it. It is safe because the version stamp is
      // written LAST -- an interrupted migration still declares v2, so v3 sees
      // an estate to migrate and every already-converted thread is read from
      // its canon rather than re-parsed.
      Self::MigrationHalted { .. } => {
        "run `intent upgrade` again -- the migration is re-runnable, and threads already converted are read from their canon rather than converted twice".to_string()
      }
      // **Delegated by INNER variant, because one remedy for the whole of
      // `IngestError` would tell someone whose history file is damaged to fix
      // their steel threads.** History is the one thing nothing recomputes, so
      // it gets the one remedy that says do NOT delete the file.
      Self::Ingest(IngestError::EventLogUnreadable { path, cause }) => format!(
        "{cause}. Nothing recomputes history, so do NOT delete {path} to get past this -- repair the named line, from version control if the file is committed"
      ),
      // **A STORE BUSY PAST ITS WAIT IS THE STORE'S TO NAME** (issue 0436).
      // The artefacts remedy below would send someone to repair files nothing
      // is wrong with, and `intent doctor` would find nothing to list. Only
      // the busy cause is delegated: a store cause an artefact can produce, a
      // constraint a malformed canon breaks, keeps the artefacts remedy.
      Self::Ingest(IngestError::Store(cause)) if cause.is_busy() => cause.remedy(),
      // Issue 0447: the canon is not at fault, so the artefacts remedy below
      // would name nothing -- the store's own sentence goes instead.
      Self::Ingest(inner @ IngestError::IndexUnreadable { .. }) => inner.remedy(),
      Self::Ingest { .. } => {
        "fix the artefacts named above, then retry -- run `intent doctor` to list them".to_string()
      }
      // **"one of:" lists only what can be HAD.** Offering a refused format as
      // the remedy for a refusal spends the operator's next command on a
      // second one; the declined names are reported after, as a warning rather
      // than a menu.
      Self::NoSuchFormat { emits, refused, .. } => {
        let mut out = format!("one of: {}", emits.join(", "));
        if !refused.is_empty() {
          out.push_str(&format!(
            ". `{}` are also recognised and deliberately refused -- ask for one to see why",
            refused.join("` and `")
          ));
        }
        out
      }
      // The reason and the route, both from the roster, because a refusal that
      // withholds either is a wall. `because` answers "why not", `instead`
      // answers "then what", and the operator asked both.
      Self::LossyFormat {
        because, instead, ..
      } => format!("{because}. {instead}"),
      // **It does NOT suggest retrying, and it does not offer another format
      // as though this were a preference.** The estate is fine and the export
      // is not; a second attempt produces the same refusal, and a different
      // format would hide the defect rather than route around it.
      Self::ExportRoundTripFailed { detail, .. } => format!(
        "{detail}. Nothing was written and the project is untouched -- this is a defect in the exporter, and it refused rather than hand you an artefact that cannot be read back"
      ),
      // **NAMES A COMMAND THAT ACTUALLY CLEARS IT.** A refusal whose remedy
      // does not lift the refusal is a dead end, and the operator who finds
      // that out is already in the failure -- so `refused_ingest_blocks_egest`
      // drives the repair through to a working egest rather than asserting the
      // wording.
      //
      // It does NOT offer a way to proceed anyway. The store being older than
      // the canon is exactly the condition under which proceeding destroys the
      // authored work, so "run it with a flag" would be an escape hatch onto
      // the one path this exists to close.
      // **DOES NOT NAME A COMMAND, BECAUSE THERE IS NO SAFE ONE TO NAME.** The
      // store being empty over a populated estate means something upstream
      // already went wrong, and every verb that could "fix" it writes. Saying
      // where the data still IS -- on disk, in the commit -- is the honest help;
      // sending the operator to a verb would be sending them to a second write
      // over a state nobody has diagnosed.
      Self::EgestWouldEmptyTheEstate { .. } => {
        "the store is empty, not the project. Your work is still on disk and in the commit; find out why the store holds nothing -- a `sync --to-store` that read zero and reported success is the usual cause -- before writing in either direction".to_string()
      }
      // Both directions are named because the bytes cannot say which side is
      // right, and the operator can: a pull they meant, or an edit they did
      // not. Neither route proceeds over the disagreement.
      Self::EgestFromStaleStore { .. } => {
        "nothing was written. If the canon on disk is right -- a pull or a peer's commit -- run `intent sync --to-store` to take it into the store. If the store is right, restore the file from git when the change is an uncommitted edit, or remove it: an absent canon file is re-created from the store. Then run this again".to_string()
      }
      // Nothing to repair by hand: every write that moved the store rendered
      // its own subject, and what this pass left behind is what it read from
      // disk, which the next pass reads again.
      Self::IngestOutpacedByWrites { .. } => {
        "nothing was lost: the store holds every write, and each write that moved it rendered its own files. The next sync, which the next edit in this project starts, renders these again; until then `intent doctor` names any file that is behind the store".to_string()
      }
      Self::EgestFromRefusedIngest { .. } => {
        "fix what the ingest refused and run `intent sync --to-store` again -- a load that succeeds clears this. Your canon holds authored work the store has never taken, so writing the store over it now is the loss, not the repair".to_string()
      }
      Self::WriteWouldEmptyAnAuthoredBody { .. } => {
        "the prose on disk has never reached the store, so this write would destroy it rather than record it. Run `intent sync --to-store` to take the authored body in, confirm it arrived, then run this verb again -- the file is intact until you do".to_string()
      }
    }
  }
}

// `render` is DELETED here, not moved: its body is now the `Remedy` trait's
// default, so this type gets it by implementing the trait. It was the only
// rendering in the workspace, and leaving it as an inherent method would mean
// the one type that already had it kept a private copy while every other error
// used the shared one -- which is how two renderings become normal.

/// One row of `intent ac list`: the criterion, its computed state, and the
/// tests that cover it.
#[derive(Debug, Clone)]
pub struct AcRow {
  pub id: String,
  pub text: String,
  /// Computed, never read off the criterion -- see [`Facade::ac_list`].
  pub state: String,
  pub covered_by: Vec<String>,
}

/// What a mutating verb DID -- because "it worked" and "there was nothing to do"
/// are different answers and a caller has to be able to say which.
///
/// **Self-loops are legal, accepted and reported at exit 0, and they do not
/// re-run the guard** (data-model.md, hv 2026-08-17, across all four machines).
/// Asking a verb for the state an entity is already in is not a movement, so it
/// is not a transition to declare and not an illegal one to refuse -- and it
/// brings v3 back to v2's measured `already CLOSED`.
///
/// **[`Outcome::AlreadyThere`] is a NO-OP and not a repeated write, which is the
/// half a reader is most likely to get wrong.** Recording an event for a
/// non-movement would stamp a second `st.done` at a second time, and under D42
/// the record is stamped BY the write -- so history would show a thread closed
/// twice. Nothing is written, nothing is re-rendered, and nothing is stamped.
/// **Deliberately NOT `#[must_use]`, and the measurement is the reason.** It was
/// annotated first, on the argument that a caller ignoring this cannot tell a
/// movement from a no-op. It fired on 65 sites, nearly all of them tests putting
/// a fixture into a state, where ignoring the outcome is exactly right. The fix
/// would have been 65 `let _ =` annotations added to silence a warning, which is
/// how an annotation stops carrying information -- the same reason
/// `exit_code_consumers.rs` excludes markdown by construction rather than firing
/// on every documentation edit. Where the outcome MUST be reported is the CLI,
/// which is a handful of sites, and that is held by tests that read what the
/// command printed.
/// **`AlreadyThere` CARRIES THE STATE, and it carries it because two verbs
/// cannot name their own target** (issue 0050). A self-loop means the entity is
/// at the verb's target, so seventeen of the nineteen arms could have printed a
/// literal -- but `ac rescope` and `ac reinstate` land on `AcState::entry(kind)`,
/// which is `Unsatisfied` or `Computed` depending on the criterion, and the
/// renderer does not know the kind. One mechanism for all of them beats fifteen
/// literals and two special cases.
///
/// **And the payload is the spelling the entity's own display source gives**, so
/// the no-op line is not a new home for a status vocabulary. That is issue 0047's
/// lesson applied before it can recur: seventeen hard-coded state words in
/// `render.rs` would be seventeen spellings a rename could not reach.
/// Something a transition has to say beyond the fact that it happened.
///
/// **A NOTE IS NOT A REFUSAL AND MUST NOT GROW INTO ONE** (AC-05.2, and vc's
/// 2026-08-19 correction of its own wording). The closing verbs delete nothing:
/// `organize` holds the only line in the tool that removes an estate file, and
/// a second gate over a destructive act this verb does not perform would refuse
/// work the real authority allows -- **and it would disagree with that
/// authority BY CONSTRUCTION rather than by drift, because the two answer
/// different questions.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Note {
  /// The artefact holds on-disk bytes no commit contains, and closing it puts
  /// its files in line for the next `organize` to remove. The strings are the
  /// paths, ready to print.
  UnsyncedAttachments(Vec<String>),
  /// This fiat close was the ONLY test covering the named criteria, which
  /// therefore stay unsatisfied with nothing left that could satisfy them.
  ///
  /// **THE VERB WAS SEMANTICALLY CORRECT AND SILENT, AND SILENCE WAS THE
  /// DEFECT** (issue 0158). `at fc` records that one test is abandoned; it is
  /// deliberately NOT a satisfaction, because an AC may have several covering
  /// tests and closing one on authority must not close the criterion a level
  /// above where anyone reads it. But when it was the LAST cover, the operator
  /// had moved the only thing that could ever answer the criterion and every
  /// observable surface reported rc=0 and no change. A verb can be correct
  /// about what it did and wrong about what it said.
  FiatClosedSoleCover(Vec<String>),
  /// **THE QUESTION COULD NOT BE ASKED** -- no repository, or git did not run.
  ///
  /// Carried as its own variant rather than as an empty list, because "nothing
  /// is uncommitted" and "I could not look" are the two answers an operator
  /// most needs to tell apart, and collapsing them prints a clean bill of
  /// health nobody earned.
  UnsyncedUnknown,
  /// The thread this verb just declared is NOT realised: a v2 status bucket
  /// still holds its files, so every write skips its views and `organize` /
  /// `hydrate` refuse it (issue 0209). The strings are project-relative paths.
  ///
  /// **SAID BY THE VERB THAT CREATED THE STATE**, because that is the one
  /// write whose success implies the thread is on its way to disk. A later
  /// write about another thread claims nothing about this one, so its skip
  /// hides nothing (vc, 2026-09-11).
  HeldByV2Bucket {
    thread: String,
    dir: String,
    home: String,
    files: usize,
  },
  /// The projection overwrote bytes that were NOT the store's own render of
  /// those paths before this mutation: a hand edit, or some other writer's
  /// work, going under. Project-relative, ready to print.
  ///
  /// **THE PREDICATE IS SKEW, NOT CHANGE, AND THE DIFFERENCE IS THE WHOLE
  /// VALUE OF THE NOTE** (vc, 2026-09-12). Every mutation rewrites the
  /// subject's own views -- that is what a projection is FOR, nobody loses
  /// anything, and a receipt printed on every mutation is one finding printed
  /// so many times that the reader learns to skip the line that matters. The
  /// line that matters is the one where the bytes on disk were not what the
  /// store would have rendered a moment ago, which is the same question
  /// `doctor`'s view-skew asks, narrowed to the paths this write is about to
  /// land on and asked BEFORE it lands.
  ///
  /// **IT IS A RECORD AND NOT A REFUSAL** (hv's silent-deletion gate,
  /// 2026-09-12, via vc). A generated view has ONE writer and the store is the
  /// SSOT (D01, reversed 2026-08-15): rewriting it is the correct act. What was
  /// missing is the record of it, and a refusal would make the correct act
  /// unavailable to anyone who had ever typed in a generated file.
  OverwroteForeignBytes(Vec<String>),
  /// Closing this artefact UNLISTS it, and these are the paths the next
  /// `organize --apply` will remove because of that. Project-relative.
  ///
  /// **THE COMMITTED FILES ARE THE POINT, WHICH IS WHY THIS IS NOT
  /// [`Note::UnsyncedAttachments`].** That warning names the bytes that reach
  /// no commit -- the sharpest case, and a strict SUBSET. Everything else armed
  /// by the same close disappeared from the operator's working tree without
  /// ever being named, and "it was in git" is a recovery route rather than a
  /// reason not to say it is going.
  ///
  /// **SAID BEFORE THE REMOVAL IS ARMED, not after it happens** (AC-03.9, and
  /// the same argument `Facade::closing_notes` already makes): the close
  /// itself removes nothing, so this is the moment the operator can still
  /// decide otherwise.
  DehydratesOnNextOrganize(Vec<String>),
  /// A unit closed while its objective was still unwritten (issue 0337). It
  /// carries the address `intent set` takes for that unit.
  UnwrittenObjective(String),
  /// The write landed, and a step after it did not.
  ///
  /// **A WARNING AND NOT A REFUSAL, BECAUSE THE WRITE HAPPENED** (vc, ruled
  /// 2026-09-14). The store holds the change, so the exit code says so. A
  /// refusal here sent the caller to retry a write that had already landed, and
  /// `wb add`, `wb ask` and `wb decide` each append a row, so the retry doubled
  /// it. The note names the step that did not run, why, and what to run.
  ///
  /// **A ROLLBACK THAT TORE THE FILES IS NOT THIS NOTE** (vc, ruled
  /// 2026-09-14): the files are then neither the old render nor the new, and
  /// the verb returns [`FacadeError::ViewsNotWritten`] naming them.
  StepFailedAfterWrite {
    step: String,
    cause: String,
    /// The cause's own chain, each link as `Remedy::render` prints it.
    caused_by: Vec<String>,
    remedy: String,
  },
}

impl Note {
  /// The note for a step that failed after the store committed.
  fn after_write(step: &str, cause: &FacadeError, remedy: &str) -> Self {
    Self::StepFailedAfterWrite {
      step: step.to_string(),
      cause: cause.to_string(),
      caused_by: std::iter::successors(std::error::Error::source(cause), |link| link.source())
        .map(ToString::to_string)
        .collect(),
      remedy: remedy.to_string(),
    }
  }
}

/// The remedy when the views a landed write renders could not be written:
/// [`FacadeError::ViewsNotWritten`]'s and the landed-write note's, in one home
/// (0376).
const RERENDER_REMEDY: &str = "the change is safe in the store -- do NOT retry it. Clear the filesystem cause, then run `intent st sync` to rewrite the files from the store. Do NOT reach for the disk -> db direction, which reads the FILES into the database and would overwrite the change with the stale copy";

/// The remedy when the views a landed BOARD write renders could not be
/// written (0487).
///
/// **SEPARATE FROM [`RERENDER_REMEDY`] BECAUSE A BOARD HAS A DIFFERENT DOOR,
/// AND IT IS NOT A HIGHLANDER COPY.** Two view families, two doors, two
/// remedies: `RERENDER_REMEDY` sends a reader to `intent st sync`, which is
/// correct for a thread's or an issue's views and cannot land a board's. Driven
/// 2026-09-20: `.intentfiles` carries no whiteboard row and `organize.rs`
/// carries no board code, so `intent organize --apply` renders no board either
/// -- it reports a clean run and leaves the tree behind the store, which is
/// worse than saying nothing. A board view is landed by a BOARD WRITE and by
/// nothing else: every one of [`Facade::land_board_write_noting`]'s callers is a
/// `wb_*` verb, and `wb touch` is the cheapest, being a heartbeat stamp that
/// changes no content.
///
/// **SO THIS NAMES THE TWO NON-DOORS AS WELL AS THE DOOR.** A node that has
/// read decision 16 -- "organize is the one door for stale views" -- reaches
/// for organize here, and that ruling is true of thread and issue views and
/// false of boards. Naming only the right command would leave the wrong one
/// looking untested.
pub(crate) const BOARD_RERENDER_REMEDY: &str = "the row is safe in the store and the TREE is behind it -- do NOT retry the write. Clear the filesystem cause, then run `intent wb touch --node <you>`: a board's views are landed by a board write and by nothing else. NOT `intent organize`, which renders no board, and NOT `intent st sync`, which rewrites a thread's views only -- either one reports a clean run and leaves the board stale";

/// The remedy when a landed write's views are on disk and the file index was not told.
const UNINDEXED_REMEDY: &str = "do not retry the write: the store holds it and its views are on disk. The file index was not told about them, so `intent sync --to-disk` records them; until then the daemon can read them back as an edit";

/// The remedy when an organize run's acts landed and the event log did not record them.
const UNRECORDED_REMEDY: &str = "do not re-run to record it: the acts listed above are on disk, and `git status` shows them. The event log has no entry for this run, and no command writes one after the fact";

/// How many times one ingest pass renders before it gives up on a store that
/// keeps moving under it (issue `0441`).
///
/// **A BOUND, BECAUSE EVERY WRITE THAT MOVED THE STORE RENDERED ITS OWN
/// SUBJECT.** What a pass renders again is what it took in from disk, and a
/// write to a canon file or a view starts another pass with its own file
/// events; three renders outlast a burst without turning a busy estate into a
/// loop.
const INGEST_RENDERS: usize = 3;

/// What `intent claude upgrade` did: canon's dispositions, and the
/// formatter-ignore patterns it added to `.prettierignore` -- or, on a dry run,
/// would add (0378).
#[derive(Debug)]
pub struct ClaudeUpgraded {
  pub applied: crate::canon::Applied,
  pub excluded: Vec<String>,
}

/// What a mutation's projection hands back to the verb that asked for it.
///
/// **THE OVERWRITES AND THE LANDED-WRITE NOTE TRAVEL TOGETHER** (0376), so the
/// one fold every transition reports through, [`Outcome::with_overwrites`],
/// carries both, and a verb that returns no `Outcome` parks its note for
/// [`Facade::take_notes`] rather than dropping it.
#[derive(Debug, Default)]
pub struct Applied {
  foreign: Vec<String>,
  after_write: Option<Note>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
  /// The field moved, and the movement is recorded.
  Moved,
  /// The entity was already in the requested state. Nothing was written.
  ///
  /// `state` is that state, spelled as the surface spells it.
  AlreadyThere { state: String },
  /// The field moved, and there is something to say about it.
  ///
  /// **THE NOTES TRAVEL WITH THE ACT RATHER THAN BEING FETCHED BESIDE IT**, and
  /// that is the whole reason this variant exists instead of the renderer
  /// asking `sync_uncommitted` itself before calling the verb. A renderer-side
  /// warning is one every OTHER caller of the facade skips in silence -- the
  /// library API, and whatever MCP surface arrives next -- so the operator's
  /// protection would be a property of the door they came through.
  MovedWith { notes: Vec<Note> },
}

impl Outcome {
  /// The no-op state, for a caller that only wants to report one.
  ///
  /// Named rather than matched at nineteen call sites: a renderer arm needs
  /// exactly "did it move, and if not what is it", and `match` on a struct
  /// variant at every one of them is the shape that invites a `..` and then a
  /// dropped payload.
  pub fn already(&self) -> Option<&str> {
    match self {
      Self::Moved | Self::MovedWith { .. } => None,
      Self::AlreadyThere { state } => Some(state),
    }
  }

  /// What this transition has to say, if anything.
  ///
  /// **PRINTED FROM THE ONE REPORTER, WHICH IS WHAT MAKES A NOTE UNDROPPABLE.**
  /// Adding a variant would NOT have forced nineteen renderer arms to handle it
  /// -- they all go through [`Outcome::already`], which is a method and absorbs
  /// a new variant silently. What actually prevents a dropped note is that
  /// every one of those arms reports through a single function, so the note is
  /// printed once, there, for all of them.
  /// Did the field move?
  ///
  /// **THE COMPLEMENT OF [`Outcome::already`], AND IT EXISTS BECAUSE A THIRD
  /// VARIANT MADE `== Outcome::Moved` A LIE.** An equality against one variant
  /// asks *which outcome is this* when the caller means *did anything happen*,
  /// and the compiler cannot tell those apart -- adding `MovedWith` broke a
  /// walk over every declared edge that had been correct for months, silently
  /// in the sense that nothing about the assertion looked wrong.
  pub fn moved(&self) -> bool {
    self.already().is_none()
  }

  pub fn notes(&self) -> &[Note] {
    match self {
      Self::Moved | Self::AlreadyThere { .. } => &[],
      Self::MovedWith { notes } => notes,
    }
  }

  /// Fold a mutation's overwrites, and the note for a step that failed after its
  /// write landed, into this outcome.
  ///
  /// **ONE FOLD RATHER THAN A NOTE BUILT AT EVERY VERB.** Thirteen call sites
  /// turn the projection's result into an `Outcome`, and a note assembled at
  /// each of them is thirteen places for the next verb to forget -- the same
  /// argument [`Outcome::notes`] makes for printing from one reporter, applied
  /// one layer earlier.
  ///
  /// **EMPTY IN MEANS UNCHANGED OUT, which is the ordinary case and not an
  /// edge.** A mutation whose every target held exactly what the store last
  /// rendered has overwritten nobody's work, so there is nothing to say --
  /// the same objection the event log makes to recording a no-op.
  #[must_use]
  pub fn with_overwrites(self, applied: Applied) -> Self {
    let mut folded = Vec::new();
    if !applied.foreign.is_empty() {
      folded.push(Note::OverwroteForeignBytes(applied.foreign));
    }
    folded.extend(applied.after_write);
    if folded.is_empty() {
      return self;
    }
    match self {
      // Nothing moved, so nothing was projected and these paths are not this
      // verb's to claim. Unreachable today -- the no-op returns before the
      // write -- and stated rather than left to `unreachable!`, which would
      // turn a future refactor's harmless case into a panic.
      Self::AlreadyThere { state } => Self::AlreadyThere { state },
      Self::Moved => Self::MovedWith { notes: folded },
      Self::MovedWith { mut notes } => {
        notes.extend(folded);
        Self::MovedWith { notes }
      }
    }
  }
}

/// What a mutating verb DID, in the projection the board ruled:
/// `moved` / `already` / `notes[]`, each note structural rather than prose.
///
/// **THE NOTES TRAVEL, AND THAT IS WHY THIS LIVES BESIDE `Outcome` RATHER THAN
/// INSIDE ONE FACE.** [`Outcome::MovedWith`] exists precisely so a non-CLI
/// caller cannot skip them in silence -- and a projection that lives in one
/// face protects only that face's callers. It was written in `mcp.rs` and
/// moved here when the wire became the SECOND machine face needing it: two
/// machine faces answering the same question from two mappings is how they
/// come to disagree about what a note is.
///
/// **THE HUMAN RENDERING IS NOT A THIRD COPY OF THIS AND MUST NOT BE FOLDED
/// IN.** `render.rs` turns the same notes into sentences with remedies in
/// them; that is a different concern with a different reader, not a divergent
/// spelling of this one.
/// What a renumber did (ST0078 WP-02): the write's outcome, what moved with
/// the id, and the prose that still names the old one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Renumbering {
  pub from: String,
  pub to: String,
  pub outcome: Outcome,
  /// Structured references outside the record that now name `to`.
  pub rewritten: Vec<String>,
  /// Paths and manifest rows the renumber moved or removed.
  pub moved: Vec<String>,
  /// Where the index found `from` in text somebody wrote. Left alone.
  pub prose: Vec<ProseMention>,
}

/// One place the index found an id in authored text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProseMention {
  pub path: String,
  pub line: Option<u32>,
}

pub fn outcome_json(outcome: &Outcome, subject: &str) -> serde_json::Value {
  serde_json::json!({
    "subject": subject,
    "moved": outcome.moved(),
    "already": outcome.already(),
    "notes": notes_json(outcome.notes()),
  })
}

/// Notes as the machine faces carry them: one mapping for [`outcome_json`] and
/// for the verbs that hand theirs over through [`Facade::take_notes`].
pub fn notes_json(notes: &[Note]) -> serde_json::Value {
  let notes: Vec<serde_json::Value> = notes
    .iter()
    .map(|note| match note {
      Note::UnsyncedAttachments(paths) => serde_json::json!({
        "kind": "unsynced-attachments", "paths": paths,
      }),
      Note::FiatClosedSoleCover(acs) => serde_json::json!({
        "kind": "fiat-closed-sole-cover", "criteria": acs,
      }),
      Note::UnsyncedUnknown => serde_json::json!({ "kind": "unsynced-unknown" }),
      Note::OverwroteForeignBytes(paths) => serde_json::json!({
        "kind": "overwrote-foreign-bytes", "paths": paths,
      }),
      Note::DehydratesOnNextOrganize(paths) => serde_json::json!({
        "kind": "dehydrates-on-next-organize", "paths": paths,
      }),
      Note::UnwrittenObjective(unit) => serde_json::json!({
        "kind": "unwritten-objective", "unit": unit,
      }),
      Note::HeldByV2Bucket {
        thread,
        dir,
        home,
        files,
      } => serde_json::json!({
        "kind": "held-by-v2-bucket", "thread": thread, "dir": dir, "home": home, "files": files,
      }),
      Note::StepFailedAfterWrite {
        step,
        cause,
        caused_by,
        remedy,
      } => serde_json::json!({
        "kind": "step-failed-after-write", "step": step, "cause": cause, "caused_by": caused_by, "remedy": remedy,
      }),
    })
    .collect();
  serde_json::Value::from(notes)
}

/// What the SQL door denied, in the words a person types.
///
/// **THE NAME AND NOT THE DEBUG SPELLING.** `{:?}` on the action renders a
/// Rust variant with its fields, which reads as an internal error rather than
/// as the rule the operator met.
fn action_name(action: &rusqlite::hooks::AuthAction<'_>) -> String {
  use rusqlite::hooks::AuthAction;
  match action {
    AuthAction::Attach { .. } => "ATTACH",
    AuthAction::Detach { .. } => "DETACH",
    AuthAction::Pragma { .. } => "PRAGMA",
    AuthAction::Transaction { .. } => "a transaction",
    AuthAction::Insert { .. } => "INSERT",
    AuthAction::Update { .. } => "UPDATE",
    AuthAction::Delete { .. } => "DELETE",
    AuthAction::CreateTable { .. } => "CREATE TABLE",
    AuthAction::DropTable { .. } => "DROP TABLE",
    _ => "that statement",
  }
  .to_string()
}

/// One SQLite value as JSON.
///
/// **A BLOB IS DESCRIBED, NEVER RENDERED.** The store holds attachment bytes,
/// and a door that inlined them would answer a `select *` with a megabyte of
/// mangled text -- and `--json` would not be valid UTF-8 to begin with.
fn sql_value(value: rusqlite::types::ValueRef<'_>) -> serde_json::Value {
  use rusqlite::types::ValueRef;
  match value {
    ValueRef::Null => serde_json::Value::Null,
    ValueRef::Integer(i) => serde_json::Value::from(i),
    ValueRef::Real(f) => serde_json::Value::from(f),
    ValueRef::Text(t) => serde_json::Value::from(String::from_utf8_lossy(t).into_owned()),
    ValueRef::Blob(b) => serde_json::Value::from(format!("<{} byte blob>", b.len())),
  }
}

/// Whether a lifecycle transition makes its declared edit to `.intentfiles`,
/// or the operator has suppressed it (AC-05.2).
///
/// **ONE TYPE FOR TWO FLAGS, BECAUSE THEY ARE ONE INSTRUCTION.** `st new
/// --dehydrate` and `st done --keep` read as opposites -- one withholds an
/// entry, the other retains one -- and both say exactly *do not make this op's
/// declared change to the list*. A `bool` per verb would have been two
/// spellings of one concept, and the third verb to grow a flag would have
/// invented a third.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListEdit {
  /// Do what the op declares. Every call but the two flagged ones.
  AsDeclared,
  /// The operator said not to. Reached only from `--dehydrate` and `--keep`.
  Suppressed,
}

/// The edit a lifecycle op declares against `.intentfiles`, if any.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ListAction {
  Add,
  Remove,
}

/// What a lifecycle op declares about `.intentfiles`.
///
/// **KEYED ON THE OP AND NEVER ON THE STATUS, AND HERE THAT IS ARITHMETIC
/// RATHER THAN PRINCIPLE.** hv ruled that nothing derives this file from
/// status; in this function the target status could not carry the answer even
/// if it were allowed to. `st.triage` and `st.reinstate` both land on
/// `NotStarted`, and `st.start`, `st.resume` and `st.reopen` all land on
/// `Wip` -- so a match on the destination would make `st triage` start adding
/// entries and `st start` re-add one a human had deliberately removed. **Two
/// collisions in a vocabulary of eight ops**, which is why the op string is
/// the key and the five members are listed by name.
///
/// **AND `None` IS A DECISION RATHER THAN A FALLTHROUGH.** `st.triage`,
/// `st.start`, `st.hold` and `st.resume` change what a thread IS and say
/// nothing about whether it is on disk. **A held thread stays realised** --
/// that is the whole content of "no function of status".
/// **THE SIGIL TRAVELS WITH THE ACTION (ST0069 WP-01), BECAUSE THE TABLE IS
/// THE ONLY THING THAT KNOWS WHICH ARTEFACT AN OP IS ABOUT.** `edit_list` used
/// to hardcode `Sigil::SteelThread`, which was true while threads were the
/// only declarable kind; an issue op reaching that code would have pinned
/// `STEELTHREAD:0001`. Returning the pair keeps the op -> (kind, action)
/// mapping in one place instead of pairing this table with a second match on
/// the op prefix somewhere else.
fn declared_list_edit(op: &str) -> Option<(Sigil, ListAction)> {
  match op {
    // **`st.new` LEFT THIS SET ON hv's RULING OF 2026-08-27 16:30Z** (hv's
    // board `1d0ce157`, first-hand in vc's session, chosen from options vc
    // authored -- the CHOICE hv's, the FRAMING vc's): *`st new` stops
    // declaring the Triage thread it just made*. A thread is created at
    // `triage`, and the realised set is WIP alone, so declaring it on creation
    // put a non-WIP thread in the manifest at the one moment nobody could yet
    // be working on it.
    //
    // **THE CONSEQUENCE, MEASURED RATHER THAN REASONED: nothing now declares a
    // thread when it BECOMES wip.** `st.start`, `st.resume` and `st.reopen`
    // all land on `wip` and only `reopen` is in this set, so a
    // `st new` + `st start` thread is WIP and undeclared until an explicit
    // `st hydrate` or `organize --default`. That is a real workflow change and
    // it is NOT a defect of this edit -- it is the question of whether the
    // transition INTO wip should declare, which is three ops hv has not ruled
    // on. vc is putting them to hv separately; whatever comes back, it changes
    // this table and not the predicate.
    // **AND THE MEMBERSHIP MOVED AGAIN ON hv's RULING OF 2026-08-27 16:43Z**
    // (hv's board `dfd07cfe`, first-hand in vc's session, chosen from options
    // vc authored): `st.start` and `st.resume` ADD because both land on `wip`;
    // `st.reinstate` STOPS adding because it lands on `not-started`. **hv was
    // told the cost -- `st reinstate` stops adding and nobody asked for that.**
    //
    // Ruling 3 was not implementable without this in EITHER direction, which is
    // why it was put to hv rather than chosen here. **The table is still keyed
    // on the OP: only its membership changed, and the predicate is untouched.**
    // Every member now lands on `wip`, so declared-iff-WIP is a property of the
    // mechanism rather than of four sites agreeing.
    //
    // **AND THE ONE-DIRECTIONAL GAP THAT PARAGRAPH REPORTED IS CLOSED BY hv's
    // RULING OF 2026-08-27 17:10Z** (hv's board `3e5e620c`, first-hand in vc's
    // session, chosen from options vc authored): `st.hold` AND `st.triage`
    // BOTH REMOVE. Both move a thread off `wip`, so both leave the realised
    // set, and declared-iff-WIP now holds in BOTH directions rather than on
    // the ADD side alone. The superseded paragraph reported the gap as *what
    // is still not true of the mechanism*; it was hv's to close and hv closed
    // it.
    //
    // **hv WAS TOLD THE COST IN THESE TERMS:** a thread put on hold LEAVES the
    // manifest and `st resume` re-adds it. The round trip works; what changes
    // is that the entries visibly vanish while the thread is held, and *a held
    // thread stays realised* was the entire content of the old design.
    //
    // **AND THE REMOVAL DRAGS THE UNSYNCED-ATTACHMENTS WARNING WITH IT**, via
    // `closing_notes` keying on this same `Remove`. `st hold` now warns where
    // it was silent. That is the documented *tied to the removal, not to the
    // verb* rule reaching a new member of its class rather than a side effect,
    // and it is asserted next door rather than left to be met in the field.
    //
    // **THE TABLE IS STILL KEYED ON THE OP, AND AFTER THIS RULING THAT IS
    // LOAD-BEARING AGAIN.** `dfd07cfe` had left every destination mapping to
    // exactly one action, so op-keying was true but no longer forced; this
    // ruling puts a second op on `not-started` taking a DIFFERENT action --
    // `st.triage` REMOVES, `st.reinstate` does nothing -- so a status-keyed
    // rewrite cannot express both. **Measured, not argued:** adding
    // `"st.reinstate"` to the arm below is the minimal faithful spelling of a
    // status-keyed table, and it reds exactly two tests in
    // `lifecycle_verbs_edit_the_list.rs` and nothing else in the crate. hv has
    // not ruled on the FORM, so a status-keyed rewrite would be an unruled
    // change wearing a refactor.
    "st.start" | "st.resume" | "st.reopen" => Some((Sigil::SteelThread, ListAction::Add)),
    // **`st.fc` REMOVES BECAUSE IT LANDS WHERE `st.done` LANDS.** The membership
    // of this table is a property of the LANDING STATE -- declared iff `wip` --
    // and a fiat close reaches `completed` exactly as an ordinary close does.
    // The difference between the two edges is the guard and the record beside
    // the status, neither of which the manifest has an opinion about: a thread
    // that is finished is off the realised set however it got there.
    "st.done" | "st.fc" | "st.cancel" | "st.hold" | "st.triage" => {
      Some((Sigil::SteelThread, ListAction::Remove))
    }
    // **ISSUES, AND THE MEMBERSHIP IS THE WHOLE OF WHAT WP-01's OBJECTIVE
    // RULED: _`issue new` adds the id and closing the issue removes it_.**
    //
    // `issues.open` IS INFERRED rather than ruled, and it is the one arm here
    // that nobody wrote down. The objective also says the default declaration
    // is *every open thread and every open issue*, and `issues.open` is the
    // only op that makes a closed issue open again -- so leaving it out would
    // put the mechanism in permanent contradiction with the declaration the
    // same paragraph specifies. It matches `st.reopen`, which adds.
    //
    // **THE OP-KEYING ARGUMENT ABOVE DOES NOT BITE HERE AND THAT IS WORTH
    // SAYING RATHER THAN LEAVING TO BE NOTICED.** An issue has exactly two
    // statuses, so op-keyed and status-keyed agree for every issue op; the
    // table stays op-keyed because threads need it to be, not because issues
    // prove it.
    "issues.add" | "issues.open" => Some((Sigil::Issue, ListAction::Add)),
    "issues.close" => Some((Sigil::Issue, ListAction::Remove)),
    // **THE TWO LEGITIMATE `None`s, NAMED SO THE WILDCARD STOPS ANSWERING FOR
    // THEM** (vc's Highlander finding F1, 2026-08-27). This vocabulary has two
    // consumers and they fail in OPPOSITE directions: an op missing from
    // `transitions.rs` is REFUSED at `check_transition`, loudly; an op missing
    // from here falls through the wildcard and SILENTLY MAKES NO LIST EDIT --
    // the transition works, the declaration quietly does not follow, and the
    // only detector is someone noticing in the field. That is what happened
    // three times on 2026-08-27, and `cce816a4`, `6ff37c0f` and `26111785` are
    // the cost already paid for it.
    //
    // `st.new` creates a thread at `triage`, which is not the realised set
    // (hv, 16:30Z). `st.reinstate` lands on `not-started`, which is not either
    // (hv, 16:43Z). Both belong here rather than in a `_` that cannot tell a
    // ruled `None` from an op nobody wired.
    "st.new" | "st.reinstate" => None,
    // **A RENUMBER MOVES THE ROW RATHER THAN ADDING OR REMOVING ONE** (ST0078
    // WP-02): the old id's line becomes the new id's, and only if it was there.
    // `land_renumber` does that edit itself, so this table has nothing to add.
    "st.renumber" | "issues.renumber" => None,
    // **A LINK MOVES NO STATUS** (issue 0460): `st relate` and `st unrelate`
    // change a thread's `related` list and nothing the declaration is keyed on.
    "st.relate" | "st.unrelate" => None,
    // **AND THE WILDCARD NOW ANSWERS FOR OTHER ENTITIES ONLY.** `wp.*`, `ac.*`,
    // `at.*` and the rest never edit a thread's declaration, and
    // `every_st_op_has_a_declared_list_answer.rs` holds it to that: every
    // `st.*` op is named above, derived from the edge table rather than from a
    // list anyone maintains.
    _ => None,
  }
}

/// The facade: a project, its store, and the canon it has loaded.
pub struct Facade {
  project: Project,
  store: Store,
  canon: Canon,
  ctx: FacadeContext,
  /// What turns a query into a vector.
  ///
  /// **BUILT FROM THE PROJECT'S CONFIGURATION AND REPLACEABLE BY THE CALLER**
  /// ([`Facade::with_embedder`]). A project with no `embed` block gets the one
  /// that refuses, which is the normal case in this cut; the seam is here so
  /// that the semantic tier plugs in without the surfaces above it changing,
  /// which is the claim AC-23.1 makes about staged tiers.
  embedder: Box<dyn crate::embed::Embedder>,
  /// The level-3 resolvers this build carries (ST0076 WP-07): a search answer
  /// names a declared language no run has recorded as `unresolved` only where
  /// the build could resolve it and the index holds its manifest.
  ///
  /// **BUILT FROM [`crate::index::resolved::readers`] AND REPLACEABLE BY THE
  /// CALLER** ([`Facade::with_resolvers`]), the seam `embedder` has.
  resolvers: Vec<crate::search::Carried>,
  /// Notes from steps that failed after a write landed, held for a verb that
  /// returns no [`Outcome`] until [`Facade::take_notes`] hands them over.
  after_write: Vec<Note>,
  /// This connection's `PRAGMA data_version` when `canon` was last read from
  /// the store: what [`Facade::catch_up`] compares against (issue 0520).
  seen: i64,
}

/// Each resolver as a search answer reads it: its language, its tool and its
/// manifest.
fn carried(readers: &[Box<dyn crate::index::resolved::Resolver>]) -> Vec<crate::search::Carried> {
  readers
    .iter()
    .map(|reader| crate::search::Carried {
      lang: reader.lang(),
      tool: reader.tool(),
      manifest: reader.manifest(),
    })
    .collect()
}

/// What level 3 says about the index for one search answer (ST0076 WP-07).
struct LevelThree {
  /// The answer's `index.resolution`.
  states: std::collections::BTreeMap<String, crate::search::ResolutionState>,
  /// The files whose resolved rows no longer describe the bytes the index
  /// holds.
  stale: std::collections::BTreeSet<String>,
  /// Whether any language has stored, which is whether there are rows to read.
  stored: bool,
  /// What an answer asked by target asked.
  target: Option<crate::search::TargetAsked>,
}

/// How many near targets a refusal names before it counts the rest: a bound on
/// the message, which a trait method with many impls would otherwise run on.
const NEAR_TARGETS_NAMED: usize = 10;

/// What [`Facade::projection`] builds: the writes, and which of them are canon.
///
/// The canon files are named because an egest has to ask about them before it
/// writes (0260) and the index has to record them after, and the projection is
/// the one place that knows which paths it added as canon.
struct Projection {
  set: WriteSet,
  /// Each thread and issue canon file in `set`, with the subject a refusal
  /// names: the thread id, or `issue 0260`.
  canon_files: Vec<(std::path::PathBuf, String)>,
}

/// One ingest pass between its snapshot and its file commit (issue `0441`):
/// what [`Facade::ingest_render`] took and rendered, for
/// [`Facade::ingest_commit`] to land or discard.
///
/// **TWO STEPS, BECAUSE THE STORE CAN MOVE BETWEEN THEM, AND THAT GAP WAS THE
/// DEFECT.** A command-line write that committed after the snapshot had its
/// canon file and view written back to this render. The fields are private,
/// so the one thing a caller can do with a render is hand it to the commit.
pub struct IngestRender {
  /// This connection's `PRAGMA data_version`, read before the snapshot.
  baseline: i64,
  canon: Canon,
  count: usize,
  projection: Projection,
  /// What this render took from the disk: see [`Ingested::taken`].
  taken: Vec<String>,
}

/// What an ingest pass that landed did (ST0078 WP-03).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ingested {
  /// The threads the pass read.
  pub threads: usize,
  /// Every thread and issue whose stored value this pass CHANGED, by subject
  /// (`ST0002`, `issue 0003`), in id order: taken from the disk, or removed
  /// because the store had written its canon file and the file is gone, which
  /// is marked `ST0003 (removed)`.
  ///
  /// **A CHANGE IN THE STORE AND NOT A CHANGE ON DISK.** A file that differs
  /// from the one the store recorded and parses to the same value is not
  /// listed, so an empty list means the store already answered what the disk
  /// says. `sync --apply` and the git hooks print on this and on `events`, and
  /// on nothing else.
  pub taken: Vec<String>,
  /// The ids of the committed event files this pass took into the store
  /// (ST0078 P1), which the store did not hold.
  pub events: Vec<String>,
}

/// What `intent sync --apply` did (ST0078 WP-05).
#[derive(Debug, Default)]
pub struct SyncApplied {
  /// One line per step that says something and runs nothing: a branch behind
  /// its upstream, unmerged paths that are not Intent's.
  pub said: Vec<String>,
  /// One line per step that ran, in order.
  pub done: Vec<String>,
  /// The steps that did not run, and why.
  pub left: Vec<crate::plan::Left>,
  /// What the ingest took, empty when it took nothing or did not run.
  pub taken: Vec<String>,
  /// The committed event files taken, by id, whichever step took them.
  pub events: Vec<String>,
  /// `doctor`, run last. Its verdict is the exit code.
  pub doctor: Option<crate::doctor::Report>,
}

/// What [`Facade::ingest_commit`] did with a render.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngestCommit {
  /// The files landed and the file index recorded them, under one hold of the
  /// writer lock.
  Written(Ingested),
  /// Another connection committed after the snapshot, so nothing was written
  /// and nothing recorded. `unwritten` is what the render would have changed,
  /// project-relative.
  StoreMoved { unwritten: Vec<String> },
}

/// The threads and issues whose value differs between `held` and `canon`, by
/// subject, in id order: what an ingest pass changed in the store. A subject
/// the pass removed says so, because `took ST0003` read as an arrival when
/// driven on a pulled deletion.
fn changed_subjects(held: &(Vec<Thread>, Vec<Issue>), canon: &Canon) -> Vec<String> {
  fn changed<T: PartialEq, K: Ord + Clone>(
    before: &[T],
    after: &[T],
    key: impl Fn(&T) -> K,
  ) -> Vec<(K, bool)> {
    let mut keys: Vec<K> = before.iter().chain(after).map(&key).collect();
    keys.sort();
    keys.dedup();
    keys
      .into_iter()
      .filter_map(|k| {
        let now = after.iter().find(|t| key(t) == k);
        (before.iter().find(|t| key(t) == k) != now).then_some((k, now.is_none()))
      })
      .collect()
  }
  let named = |subject: String, removed: bool| match removed {
    true => format!("{subject} (removed)"),
    false => subject,
  };
  let threads = changed(&held.0, &canon.threads, |t| t.id.clone());
  let issues = changed(&held.1, &canon.issues, |i| i.number);
  threads
    .into_iter()
    .map(|(id, removed)| named(id, removed))
    .chain(
      issues
        .into_iter()
        .map(|(n, removed)| named(format!("issue {n:04}"), removed)),
    )
    .collect()
}

/// A canon file that exists and cannot be read. Reported, never read past: an
/// egest that could not see the file cannot know whether it moved.
fn canon_file_unreadable(path: &std::path::Path, source: std::io::Error) -> FacadeError {
  FacadeError::Ingest(IngestError::Io {
    path: path.display().to_string(),
    source,
  })
}

/// A narrowing of the history. Every field NARROWS; none widens.
///
/// **Filters are the point rather than a convenience.** `event_log` is
/// append-only and monotone, so an unfiltered dump is the one output
/// guaranteed to become unusable with time -- and a reader who has to pipe it
/// through `grep` has been handed the file back with extra steps, which is the
/// artefact the ruling removed.
#[derive(Debug, Default, Clone)]
pub struct EventFilter {
  pub op: Option<String>,
  pub subject: Option<String>,
  pub limit: Option<usize>,
}

/// What one read of the SQL door answers (AC-17.2, AC-17.4).
///
/// **BOTH DENOMINATORS TRAVEL**, the `events` page pattern: `matched` is every
/// row the statement produced and `returned` is how many came back. A capped
/// result that reported only what it returned is a silent subset, and the
/// reader has no way to tell it from the whole answer.
///
/// **THE SCHEMA VERSION IS THE STORE'S, READ OFF THE DATABASE** rather than
/// `store::SCHEMA_VERSION`, which is what this BINARY believes. They agree
/// whenever the store opened at all, and the envelope is a claim about the
/// database the caller just queried, so it comes from the database.
#[derive(Debug, Clone)]
pub struct SqlPage {
  pub columns: Vec<String>,
  pub rows: Vec<Vec<serde_json::Value>>,
  pub matched: usize,
  pub returned: usize,
  pub schema_version: i32,
}

/// How many rows the SQL door returns when nobody says.
///
/// **A DEFAULT AND NOT A MEASUREMENT**, and it is a constant rather than a
/// config key on purpose: a key nobody asked for is a second place for the
/// answer to live, which is what D42 warns about. It is named in the refusal
/// so an operator meeting the bound reads the number rather than guessing it.
pub const SQL_ROWS_DEFAULT: usize = 200;

/// The most rows the SQL door will return for any `--limit`.
pub const SQL_ROWS_CEILING: usize = 10_000;

/// How much WORK somebody else's statement may do inside this process, counted
/// in SQLite virtual-machine instructions.
///
/// **IT IS A WORK BUDGET AND NOT A TIME BOUND, AND THAT IS THE ESTATE'S RULE
/// RATHER THAN A PREFERENCE.** The specification said five seconds; a deadline
/// needs `Instant::now`, and `one_clock.rs` bans every ambient clock in this
/// workspace with an exemption list that is empty and says it must stay empty
/// (D42: time is a property of a write). A budget answers the same question --
/// *has this gone on too long* -- by counting the work rather than asking what
/// time it is, and it has a property a deadline does not: the same statement on
/// the same store is refused at the same point on a fast machine and a slow
/// one.
///
/// Measured on this machine, a runaway recursive CTE trips it in a few seconds.
/// That figure is an OBSERVATION about one machine and is not the contract; the
/// contract is the budget.
pub const SQL_WORK_BUDGET: u64 = 20_000;

/// How many virtual-machine instructions pass between two checks of the budget.
pub const SQL_WORK_INTERVAL: i32 = 10_000;

/// A page of history WITH ITS DENOMINATOR.
///
/// **`matched` and `total` are carried separately and neither is the length of
/// `rows`.** A count of what was printed reported as a count of what exists is
/// this estate's most-repeated defect; here the three numbers can differ
/// legitimately -- a limit truncates `rows`, a filter narrows `matched`, and
/// `total` is what the store holds -- so a caller that prints one of them can
/// say which.
#[derive(Debug)]
pub struct EventPage {
  pub rows: Vec<Envelope>,
  pub matched: usize,
  pub total: usize,
}

impl Facade {
  /// Refuse a project whose canon this binary cannot read (AC-10.7).
  ///
  /// **Here, and not in [`ingest`], because this is the boundary where a
  /// question gets answered.** `ingest::read` is also what `doctor` and the
  /// WP-10 migrator call, and both of those must be able to look at an
  /// unmigrated project -- a gate that stopped them would take away the two
  /// tools whose entire job is this state.
  ///
  /// It also runs BEFORE the store is opened, so the refusal never depends on
  /// a DB that an unmigrated project has no reason to have.
  ///
  /// # `critic` MUST NOT BE BUILT ON THIS, and the ground is a new one
  ///
  /// **Issue 0045 (vc), and it does not reproduce today** -- which is exactly
  /// why it is written at the point of temptation rather than in a backlog.
  ///
  /// The two exemptions above share a ground: their job IS the unmigrated
  /// state. `critic`'s ground is different and this comment did not contemplate
  /// it -- **its consumer fails CLOSED on the refusal code.** The shipped
  /// pre-commit gate reads `1` from `intent critic` as FINDINGS and blocks the
  /// commit; every refusal here becomes `Unmigrated -> Failure::Error -> 1`. So
  /// a `critic` opened through this function blocks every commit in every
  /// unmigrated project, printing a remedy about findings that do not exist
  /// while the true remedy -- run `intent upgrade` -- sits on screen above it,
  /// overridden by one that cannot be followed.
  ///
  /// **Moving the refusal to 2 is NOT the fix and was considered.** It clears
  /// git and breaks Claude Code, whose `UserPromptSubmit` reads 2 as BLOCK.
  /// That is issue 0043 rebuilt one consumer over. `critic` needs to reach the
  /// canon without asking this question at all -- the same route `doctor`
  /// takes -- rather than a different number.
  ///
  /// Held mechanically by `an_unmigrated_project_can_still_commit`, which drives
  /// the SHIPPED hook in an unmigrated fixture and reds the day this is wired
  /// the obvious way.
  fn readable(project: &Project) -> Result<(), FacadeError> {
    match project.migration() {
      Migration::Done => Ok(()),
      Migration::Pending(pending) => Err(FacadeError::Unmigrated(pending)),
    }
  }

  /// Open a project, loading and validating its whole canon.
  pub fn open(project: Project, ctx: FacadeContext) -> Result<Self, FacadeError> {
    Self::open_as(project, ctx, Opening::Ordinary)
  }

  /// Open a project the way `opening` says: [`Opening::Ordinary`] is
  /// [`Facade::open`], and [`Opening::RepairingIndex`] is the door `intent
  /// index rebuild` takes (issue 0453).
  pub fn open_as(
    project: Project,
    ctx: FacadeContext,
    opening: Opening,
  ) -> Result<Self, FacadeError> {
    Self::readable(&project)?;
    let mut store = Store::open(&project.db_path()).map_err(FacadeError::Store)?;
    if opening == Opening::RepairingIndex {
      store.recreate_index_tables().map_err(FacadeError::Store)?;
    }
    // **READ BEFORE THE LOAD** (issue 0520), so a commit that lands while the
    // canon is read moves the version past it and the next catch-up reloads.
    let seen = store.data_version().map_err(FacadeError::Store)?;
    // The daily-driver path: answer from the store unless the tree moved.
    let canon = ingest::load_fresh(&project, &mut store)?;
    let mut facade = Self::over(project, ctx, store, canon, seen);
    if opening == Opening::RepairingIndex {
      facade.rederive_prose()?;
    }
    Ok(facade)
  }

  /// Put the model's prose back into the recreated `doc_sections`: canon's
  /// half from the records the store holds, and each migrated board's carried
  /// `.history/` from the files it was carried from (issue 0453).
  ///
  /// **THE FILES' HALF AND THE SOURCE TABLE ARE NOT HERE**: they are
  /// [`Facade::index_rebuild`]'s, which the verb runs next, and a second
  /// writer of them here would be a second home for the walk.
  fn rederive_prose(&mut self) -> Result<(), FacadeError> {
    self.canon.sections = ingest::sections_of(
      &self.project,
      &self.canon.threads,
      &self.canon.issues,
      &self.canon.boards,
    );
    self
      .store
      .replace_doc_sections(&self.canon.sections)
      .map_err(FacadeError::Store)?;
    let migrated: Vec<String> = self
      .canon
      .boards
      .iter()
      .filter(|b| b.node.migrated_at.is_some())
      .map(|b| b.node.moniker.clone())
      .collect();
    for node in migrated {
      let home = self.project.whiteboard_dir().join(&node);
      let (_, mut sections, _) = self.read_snapshots(&node, &home)?;
      sections.sort_by(|a, b| (&a.file, a.seq).cmp(&(&b.file, b.seq)));
      self
        .store
        .replace_wb_sections_for(&node, &sections)
        .map_err(FacadeError::Store)?;
    }
    Ok(())
  }

  /// A facade over a store and the canon read from it: the one place the
  /// struct is assembled, for both opens and for [`Facade::sync_plan`]'s
  /// shadow.
  fn over(project: Project, ctx: FacadeContext, store: Store, canon: Canon, seen: i64) -> Self {
    let embedder = crate::embed::from_config(&project.config().embed);
    Self {
      project,
      store,
      canon,
      ctx,
      embedder,
      resolvers: carried(&crate::index::resolved::readers()),
      after_write: Vec::new(),
      seen,
    }
  }

  /// Open against an in-memory store, for callers that do not want the DB on
  /// disk (tests, and the daemonless read paths).
  pub fn open_in_memory(project: Project, ctx: FacadeContext) -> Result<Self, FacadeError> {
    Self::readable(&project)?;
    let mut store = Store::open_in_memory().map_err(FacadeError::Store)?;
    let seen = store.data_version().map_err(FacadeError::Store)?;
    let canon = ingest::load(&project, &mut store)?;
    Ok(Self::over(project, ctx, store, canon, seen))
  }

  /// Reload the canon if another connection has committed since it was read,
  /// and say whether it did (issue 0520).
  ///
  /// **A FACADE ANSWERS FROM THE CANON IT LOADED, AND THAT COPY MOVES ONLY FOR
  /// ITS OWN WRITES.** A verb's facade lives for one command, so that is the
  /// store as it stood when the command began. The explorer's lives for the
  /// whole session: a write from another terminal or node, intentd's ingest of a
  /// hand edit and the explorer's own `/` commands, which run through a second
  /// facade, all commit through other connections, and none of them reached the
  /// explorer until it was restarted. `intent mcp` opens per request for the
  /// same reason; a session cannot.
  ///
  /// **THE STORE SAYS WHETHER IT MOVED, SO NO WRITER HAS TO.**
  /// [`Store::data_version`] moves when another connection commits and never
  /// for this one's own writes (issue 0441), so when nothing moved this costs
  /// one pragma read. The version is read BEFORE the reload, so a commit that
  /// lands during it moves the version again and is caught next time.
  ///
  /// **NOT BEFORE A WRITE THAT IS TO BE JUDGED.** [`Store::commit_mutation`]'s
  /// compare-and-swap refuses a write whose record moved since the canon was
  /// read. Catching up first would make the record as it is now the write's
  /// baseline, and wave through an edit made against the one the operator saw.
  ///
  /// It reloads through [`ingest::load_fresh`], the door [`Facade::open`] takes,
  /// so a facade that caught up holds what a fresh open would.
  pub fn catch_up(&mut self) -> Result<bool, FacadeError> {
    let now = self.store.data_version().map_err(FacadeError::Store)?;
    if now == self.seen {
      return Ok(false);
    }
    self.canon = ingest::load_fresh(&self.project, &mut self.store)?;
    self.seen = now;
    Ok(true)
  }

  pub fn project(&self) -> &Project {
    &self.project
  }

  /// Answer semantic questions with THIS embedder rather than the one the
  /// project's configuration names.
  ///
  /// **THE SEAM, AND IT IS PRODUCTION CODE RATHER THAN A TEST HOOK.** A caller
  /// that already holds an embedder -- a daemon with one warm, a tool told to
  /// use a different model -- supplies it here; a test supplying a fixed one is
  /// the same caller. What it must never become is a way to reach past the
  /// configuration silently: the embedder a project configured is what it gets
  /// unless somebody says otherwise in as many words.
  pub fn with_embedder(mut self, embedder: Box<dyn crate::embed::Embedder>) -> Self {
    self.embedder = embedder;
    self
  }

  /// Answer searches as a build carrying THESE level-3 resolvers would (ST0076
  /// WP-07).
  ///
  /// **THE SEAM `with_embedder` IS, FOR THE SAME REASON.** What a build carries
  /// is part of a search answer, since a declared language is `unresolved` only
  /// where the build could resolve it, so a caller holding other readers says
  /// so in as many words.
  pub fn with_resolvers(mut self, readers: &[Box<dyn crate::index::resolved::Resolver>]) -> Self {
    self.resolvers = carried(readers);
    self
  }

  /// **The migration door, and the one entry point that deliberately does NOT
  /// go through [`Facade::open`].**
  ///
  /// `open` calls `readable`, which refuses an unmigrated project -- and the
  /// migrator runs on an unmigrated project by definition, so routing it
  /// through the usual door would make the operation refuse the only estate it
  /// exists for. It is an associated function rather than a method for the same
  /// reason: there is no `Facade` to be had until this has run.
  ///
  /// **THE ORDER IS THE CONTRACT AND THE STAMP GOES LAST.** Plan (writes
  /// nothing) -> commit the files -> rebuild the store -> converge gitignore ->
  /// stamp the version. Three separate arguments land on that last step and
  /// they are worth keeping apart:
  ///
  /// 1. A stamp written before the canon leaves a project claiming to be v3
  ///    with no canon, and `readable`'s gate stops firing -- so the state that
  ///    tells an operator what is wrong is the state the premature stamp
  ///    destroyed. (v2 learned this in ST0043.)
  /// 2. **v2 now REFUSES a project declaring a newer version than itself**, so
  ///    between a premature stamp and complete canon the estate is locked out of
  ///    BOTH toolchains at once -- v2 because the project is from the future, v3
  ///    because the canon is not there. There is no tool left to fix it with.
  /// 3. **It is what makes the re-run possible at all**, which is what hv's
  ///    fix-forward ruling depends on: while the config still says v2, an
  ///    interrupted estate is one v3 will migrate again. Stamp early and a
  ///    half-migrated project is one v3 believes is finished, so fix-forward has
  ///    nothing to fix forward with.
  ///
  /// **Nothing is written when this refuses** (AC-10.2). Before the commit that
  /// is structural -- `plan` builds a `WriteSet` and does not apply it. After
  /// it, a failure rolls the files back, so a halted migration leaves the estate
  /// as it found it rather than half-converted.
  pub fn upgrade(project: &Project, ctx: &FacadeContext) -> Result<Upgraded, FacadeError> {
    // **THE FLOOR IS ENFORCED HERE, AND ITS ABSENCE WAS A REAL DEFECT.**
    //
    // `readable()` does TWO things -- it refuses an unmigrated project AND it
    // enforces the migration floor -- and this door bypasses it on purpose,
    // because the migrator runs on an unmigrated project by definition. That
    // reasoning covers the first job and is silent on the second, so bypassing
    // the function bypassed both. **The floor was checked on the door that
    // READS and not on the door that WRITES, and the writing one is the one the
    // floor exists to stop.** Measured by vc across the fleet: Utilz declares
    // 2.18.0 and was converted clean -- 61 files, 9 threads, stamped -- with no
    // refusal and nothing looking wrong.
    //
    // It matters because 2.19.0 is where the acceptance-test row grammar
    // landed. A sub-floor estate converted directly skips that migration, so
    // its rows arrive in v3 in a grammar v3 was never told about, silently,
    // because nothing on this path is looking.
    //
    // **One arm, and deliberately not `readable()` wholesale.**
    // `Migration::Done` MUST proceed -- that is the re-run after an interrupted
    // migration, and idempotence rests on it -- and `Pending` at or above the
    // floor is the ordinary estate this door exists for.
    // **THE OTHER TWO PRECONDITIONS `migration.md` DOCUMENTS, WHICH DID NOT
    // EXIST** (issue 0271). The section opens *refused by name, not worked
    // around* and states three refusals; until this commit the floor above was
    // the only one implemented, and both of the others were driven on the
    // GENUINE conversion path -- a dirty tree converted 349 threads at exit 0,
    // and a tree with no `.git` at all converted at exit 0 with the word `git`
    // absent from the whole run.
    //
    // **THEY LIVE INSIDE THE `Pending` ARM, WHICH IS THE SCOPING RATHER THAN A
    // COMMENT ABOUT IT.** `Migration::Done` is the convergent re-run, and
    // neither of these may reach it: measured across 18 fleet members, the dirt
    // check refuses 11 of them on the verb and 1 of 1 on the conversion.
    if let crate::project::Migration::Pending(pending) = project.migration() {
      if pending.below_floor {
        return Err(FacadeError::BelowMigrationFloor(pending));
      }
      match crate::sync::tree_state(project.root()) {
        crate::sync::TreeState::NoWorkTree => return Err(FacadeError::MigrationWithoutGit),
        crate::sync::TreeState::Dirty(paths) => {
          return Err(FacadeError::MigrationOverDirtyTree { paths });
        }
        crate::sync::TreeState::Clean => {}
      }
    }

    let scan = crate::legacy::scan(project).map_err(|cause| FacadeError::MigrationHalted {
      step: "reading the v2 estate",
      cause,
    })?;
    let plan = crate::migrate::plan(project, ctx, scan)?;
    let crate::migrate::Plan {
      writes,
      threads,
      issues,
      carried,
      already_migrated,
      already_migrated_issues,
      dispositions,
      bucket_ingested,
      bucket_not_ingested,
    } = plan;

    let files = writes.len();
    let applied = writes.commit()?;

    // Everything past here has files on disk, so a failure unwinds them rather
    // than leaving a half-converted estate. `keep()` is reached only once the
    // stamp has landed.
    // Returns what the prune did, because the caller reports it and a closure
    // that swallowed it would make the removal unreviewable.
    // What the prune removed, what it deferred, the leftovers it judged, the
    // events backfilled, and the old single-file log's fate (issue 0459).
    type Finished = (
      Vec<std::path::PathBuf>,
      Vec<std::path::PathBuf>,
      crate::legacy::Leftovers,
      usize,
      Option<EventLogLeftover>,
    );
    let finish = || -> Result<Finished, FacadeError> {
      let mut store = Store::open(&project.db_path())?;
      store.rebuild(&threads, &issues)?;
      // **AND THE PROJECT STATE, WHICH THE REBUILD DOES NOT CARRY** (issue
      // 0485). On a cold store the rebuild left no `project` row, so every
      // later render read no DONE watermark from the store while `doctor` read
      // one from canon, and `todo.md` disagreed with doctor until a
      // `sync --to-store`. `load` and `resync` carry it; this is the third door.
      ingest::carry_project_state(project, &mut store)?;
      // The store has just been built from the canon these writes landed, so
      // it records their bytes (0260); without a baseline, the first egest
      // after a hop could not tell a peer's committed change from its own.
      ingest::record_canon_files(
        project,
        &mut store,
        &ingest::canon_paths(project, &threads, &issues),
      )?;
      // **THE v2 PRUNE, AT THE SECOND DOOR** (WP-02, AC-02.2). It runs AFTER
      // the store rebuild above, deliberately: "does the store hold this file"
      // is only answerable once the store holds anything, and asking it of the
      // canon we are about to write would be asking the plan rather than the
      // estate.
      //
      // **ONE DERIVATION, AND `organize --apply` IS THE OTHER DOOR.** A second
      // implementation of "may this go?" is how one door removes what the other
      // would have kept, in the direction that cannot be taken back.
      let converted = Canon {
        threads: threads.clone(),
        issues: issues.clone(),
        sections: Vec::new(),
        boards: Vec::new(),
      };
      let leftovers = crate::legacy::leftovers(project, &converted);
      let mut pruned = Vec::new();
      let mut deferred = Vec::new();
      // **A RUN THAT INGESTED BUCKET FILES DEFERS THE PRUNE** (issue 0319; see
      // `Upgraded::prune_deferred`). The refusal still comes first: an estate
      // with an unheld file removes nothing whatever this run carried.
      match (leftovers.refuses(), bucket_ingested.is_empty()) {
        (true, _) => {}
        (false, false) => deferred.clone_from(&leftovers.removable),
        (false, true) => {
          for path in &leftovers.removable {
            std::fs::remove_file(path).map_err(|cause| FacadeError::MigrationHalted {
              step: "removing the v2 tree the store now holds",
              cause,
            })?;
            pruned.push(path.clone());
          }
        }
      }
      converge_gitignore(project).map_err(|cause| FacadeError::MigrationHalted {
        step: "adding the per-machine artefacts to .gitignore",
        cause,
      })?;
      let event_log_leftover =
        remove_event_log_leftover(project).map_err(|cause| FacadeError::MigrationHalted {
          step: "removing the empty event log an earlier upgrade left",
          cause,
        })?;
      let events_backfilled = backfill_event_files(project, &mut store)?;
      converge_formatter_exclusion(project).map_err(|cause| FacadeError::MigrationHalted {
        step: "keeping the formatter off generated views",
        cause,
      })?;
      declare_default_if_absent(project, &threads, &issues).map_err(|cause| {
        FacadeError::MigrationHalted {
          step: "writing the realisation manifest",
          cause,
        }
      })?;
      stamp_version(project).map_err(|cause| FacadeError::MigrationHalted {
        step: "stamping the project version",
        cause,
      })?;
      Ok((
        pruned,
        deferred,
        leftovers,
        events_backfilled,
        event_log_leftover,
      ))
    };
    match finish() {
      Ok((pruned, prune_deferred, leftovers, events_backfilled, event_log_leftover)) => {
        applied.keep();
        // **THE VIEWS OF EVERY UNDECLARED THREAD GO, THROUGH ORGANIZE'S OWN
        // PLAN AND GATE** (issue 0316, vc's ruling 2026-09-14). The manifest
        // renders no undeclared thread's views and the gate removed only a
        // byte-identical one, so a view an earlier binary rendered for a thread
        // this run left undeclared had no owner, and an upgraded estate handed
        // its operator a hand deletion. The migration has LANDED by this line,
        // so a failure here is reported beside the refusals rather than
        // returned: an error now would say the upgrade was not made when it was.
        let (dehydrated, dehydrate_refused) = match Facade::open(project.clone(), ctx.clone())
          .and_then(|mut f| f.dehydrate_undeclared_thread_views())
        {
          Ok(report) => (
            report.dehydrated,
            report
              .refused
              .iter()
              .map(crate::remedy::Remedy::render)
              .collect(),
          ),
          Err(cause) => (
            Vec::new(),
            vec![format!(
              "the views of undeclared threads were not examined: {cause}"
            )],
          ),
        };
        Ok(Upgraded {
          dehydrated,
          dehydrate_refused,
          pruned,
          ingested: bucket_ingested,
          not_ingested: bucket_not_ingested,
          prune_deferred,
          prune_withheld: leftovers.withheld,
          pointers: leftovers.pointers,
          threads: threads.len(),
          issues: issues.len(),
          files,
          carried,
          already_migrated,
          already_migrated_issues,
          dispositions,
          events_backfilled,
          event_log_leftover,
        })
      }
      Err(halted) => {
        // The rollback's own failure is not allowed to hide the reason we are
        // rolling back: it is reported as the cause of a halt at this step, and
        // the original error is what the operator is told about.
        applied.rollback()?;
        Err(halted)
      }
    }
  }

  pub fn canon(&self) -> &Canon {
    &self.canon
  }

  pub fn store(&self) -> &Store {
    &self.store
  }

  /// Everything a render is allowed to know, assembled from the store.
  ///
  /// **Fallible because the DONE cutoff is a read of the `project` table.** It
  /// used to be derived from the event log, which made it history -- and D53
  /// took history out of the working tree, so it could not cross a clone. The
  /// cutoff is STATE and the store holds it; the log still records every flush
  /// and nothing reads a cutoff out of it.
  fn render_ctx(&self) -> Result<RenderContext<'_>, FacadeError> {
    Ok(RenderContext {
      version: &self.ctx.version,
      todo_watermark: self.store.todo_watermark().map_err(FacadeError::Store)?,
    })
  }

  // -------------------------------------------------------------------------
  // Reads
  // -------------------------------------------------------------------------

  /// Threads in the order the index renders them -- open first, newest id
  /// first. Ascending id would have been the obvious choice and is wrong:
  /// v2 lists newest-first, and `st list` has to agree with the generated
  /// index byte for byte.
  pub fn st_list(&self) -> Vec<&Thread> {
    views::index_order(&self.canon.threads)
  }

  /// Every issue, in number order.
  ///
  /// **Ordered here rather than at the call site**, so the terminal table, the
  /// generated view and any later consumer agree without each sorting -- which
  /// is the shape `views::index_order` already holds for threads.
  pub fn issue_list(&self) -> Vec<&crate::model::Issue> {
    let mut out: Vec<&crate::model::Issue> = self.canon.issues.iter().collect();
    out.sort_by_key(|i| i.number);
    out
  }

  /// The outstanding threads, work packages and issues of the kinds in `show`
  /// (ST0079), read through [`Self::st_list`] and [`Self::issue_list`] so each
  /// kind keeps the order its own list verb prints.
  pub fn outstanding(&self, show: &[crate::outstanding::Kind]) -> crate::outstanding::Outstanding {
    crate::outstanding::outstanding(&self.st_list(), &self.issue_list(), show)
  }

  /// Every issue in the explorer's collection order: open first, then newest
  /// first ([`views::issue_index_order`]), beside [`Self::issue_list`]'s
  /// number order.
  pub fn issue_index(&self) -> Vec<&crate::model::Issue> {
    views::issue_index_order(&self.canon.issues)
  }

  /// One issue by number.
  ///
  /// The NUMBER rather than the rendered id, because zero-padding is a display
  /// decision: `21`, `0021` and `issues/0021.json` are one issue, and the
  /// widening from a string to a number belongs at the surface where the
  /// operator's spelling arrives.
  pub fn issue_show(&self, number: u32) -> Result<&crate::model::Issue, FacadeError> {
    self
      .canon
      .issues
      .iter()
      .find(|i| i.number == number)
      .ok_or(FacadeError::NoSuchIssue { number })
  }

  pub fn st_show(&self, id: &str) -> Result<&Thread, FacadeError> {
    self
      .canon
      .threads
      .iter()
      .find(|t| t.id == id)
      .ok_or_else(|| FacadeError::NoSuchThread { id: id.to_string() })
  }

  /// One attachment a thread carries, by its path relative to the thread's own
  /// directory (issue 0398).
  ///
  /// **THE RECORD, NEVER THE FILE.** An attachment's content is in the store
  /// whether or not its file is on disk -- a dehydrated thread has none -- so a
  /// read answering from the tree would answer for fewer threads than the store
  /// holds, and for the thread the finder met not at all.
  pub fn st_attachment(&self, id: &str, path: &str) -> Result<&Attachment, FacadeError> {
    self
      .st_show(id)?
      .attachments
      .iter()
      .find(|a| a.path == path)
      .ok_or_else(|| FacadeError::NotCarried {
        thread: id.to_string(),
        path: path.to_string(),
      })
  }

  /// A thread's acceptance contract: the bytes its `acceptance.md` is hydrated
  /// with, whether or not that file is on disk (issue 0398).
  ///
  /// **SELECTED FROM [`views::render_all`], NEVER RENDERED HERE**, for the reason
  /// [`crate::address::serve_md`] gives: calling the per-view renderer would
  /// produce bytes that merely equal the hydrated ones rather than being them.
  /// And because `render_all` renders a contract for every thread canon holds,
  /// a thread with none is a thread this estate does not hold.
  pub fn st_contract(&self, id: &str) -> Result<String, FacadeError> {
    let want = self.project.acceptance_view(id);
    let ctx = self.render_ctx()?;
    views::render_all(&self.project, &self.canon, &ctx)
      .into_iter()
      .find(|view| view.path == want)
      .map(|view| view.content)
      .ok_or_else(|| FacadeError::NoSuchThread { id: id.to_string() })
  }

  pub fn wp_list(&self, st: &str) -> Result<&[WorkPackage], FacadeError> {
    Ok(&self.st_show(st)?.wps)
  }

  pub fn wp_show(&self, st: &str, seq: u32) -> Result<&WorkPackage, FacadeError> {
    self
      .st_show(st)?
      .wps
      .iter()
      .find(|w| w.seq == seq)
      .ok_or_else(|| FacadeError::NoSuchWorkPackage {
        st: st.to_string(),
        seq,
      })
  }

  /// **ONE HOME FOR *WHICH WORK PACKAGE DOES THIS ENTITY NAME*.**
  ///
  /// A `Wp` entity carries its sequence as TEXT, because an address is text;
  /// every caller that wants the work package has to parse it and decide what
  /// an unparseable one means. That decision was about to exist twice --
  /// [`Self::entity_json`] had it and [`Self::edit`] needed the same answer --
  /// so it lives here and both call it.
  ///
  /// **A NUMBER THAT WILL NOT PARSE IS *NO SUCH ENTITY*, NOT A THIRD ERROR.**
  /// `intent://.../wp/banana` names no work package for the same reason
  /// `.../wp/9999` names none, and an operator who mistyped needs the same
  /// sentence either way.
  fn wp_of(&self, thread: &str, wp: &str) -> Result<&WorkPackage, FacadeError> {
    let seq = wp
      .parse::<u32>()
      .map_err(|_| FacadeError::NoSuchWorkPackage {
        st: thread.to_string(),
        seq: 0,
      })?;
    self.wp_show(thread, seq)
  }

  /// The entity behind one address, as the JSON a form is resolved against.
  ///
  /// **THE OTHER HALF OF THE SHARED DERIVATION, AND IT WAS THE HALF LEFT
  /// BEHIND.** [`crate::form::triples`] was moved down here in 2026-08-30
  /// because `intentd` depends on `intentsvcs` and NOT on the CLI, so a daemon
  /// emitter would have had to write the walk a second time. **The entity
  /// LOOKUP that feeds it stayed in `intent-cli`, where the daemon still could
  /// not reach it** -- so the correction was half made, and the missing half
  /// only became visible when something outside the CLI first needed a form.
  ///
  /// **AND IT HAD ALREADY GROWN ITS SECOND HOME BEFORE ANYONE LOOKED.** The
  /// CLI resolved `thread` and `issue` in one function and work packages in a
  /// DIFFERENT one -- an inline `serde_json::to_value` inside the children
  /// walk -- so two spellings of *turn this entity into form JSON* were live in
  /// one file. Both now call this.
  ///
  /// **THE `wp` ARM IS THE POINT OF THE MOVE AND NOT A BONUS.** The function
  /// this replaces answered `_ => None` for a work package while
  /// `surface/forms.json` declared a `wp` form, and `nav.rs` records what that
  /// pairing produces: *a form whose every value is blank, for every work
  /// package, not just a missing one.* It was unreachable, so it was latent --
  /// and `AC-17.6` requires both verbs to cover ST, WP and ISSUE, which is
  /// precisely what makes it reachable. Carrying the catch-all down here would
  /// have moved the bug rather than fixed it.
  ///
  /// **THE `wp` ARM RESOLVES THROUGH `Self::wp_of`**, which is also what
  /// [`Self::edit`] checks existence with, so a work package that this door
  /// can describe and a work package that door will open are the same set.
  pub fn entity_json(
    &self,
    entity: &crate::address::Entity,
  ) -> Result<serde_json::Value, FacadeError> {
    use crate::address::Entity;
    let value = match entity {
      Entity::Thread { id } => serde_json::to_value(self.st_show(id)?),
      Entity::Wp { thread, wp } => serde_json::to_value(self.wp_of(thread, wp)?),
      Entity::Issue { id } => {
        let number = id
          .parse::<u32>()
          .map_err(|_| FacadeError::NoSuchIssue { number: 0 })?;
        serde_json::to_value(self.issue_show(number)?)
      }
      other => {
        return Err(FacadeError::NoFormForEntity {
          form: other.form().to_string(),
        });
      }
    };
    // **A SERIALISATION FAULT IS THIS PROCESS'S, NOT THE OPERATOR'S**, and it
    // gets its own variant rather than being folded into `NoFormForEntity` --
    // the model types derive `Serialize`, so reaching here means something
    // structural broke, and a *check which kinds are declared* remedy would
    // send someone hunting a typo that is not there.
    value.map_err(|why| FacadeError::EntityUnserialisable {
      form: entity.form().to_string(),
      why: why.to_string(),
    })
  }

  /// Every criterion with its COMPUTED state and its covering tests.
  ///
  /// The state is computed here rather than read off the criterion, because
  /// for a test-backed AC it is not stored anywhere: satisfaction comes from a
  /// covering green test, and storing it too would be the double truth
  /// data-model.md forbids.
  /// The criteria scoped to one work package, as `ac list` composes them
  /// (issue 0310). A criterion belongs to a package by its id's group --
  /// `AC-03.x` to package 3 -- which is the same grouping the close gate's
  /// `Scope::WorkPackage` reads, so `wp show` lists exactly what `wp gate` and
  /// `wp done` judge. Refuses a package the thread does not have.
  pub fn wp_criteria(&self, st: &str, seq: u32) -> Result<Vec<AcRow>, FacadeError> {
    self.wp_show(st, seq)?;
    let group = format!("{seq:02}");
    Ok(
      self
        .ac_list(st)?
        .into_iter()
        .filter(|row| crate::contract::group_of(&row.id) == group)
        .collect(),
    )
  }

  pub fn ac_list(&self, st: &str) -> Result<Vec<AcRow>, FacadeError> {
    let thread = self.st_show(st)?;
    Ok(thread.criteria.iter().map(|c| ac_row(thread, c)).collect())
  }

  /// One criterion, for `intent ac show` (0168): the criterion as stored --
  /// its kind, its state's payload and its whole text -- and its row as
  /// [`Facade::ac_list`] computes it, so the two verbs share one state
  /// vocabulary. A missing id is the same `NoSuchCriterion` every AC verb
  /// refuses with.
  pub fn ac_show(&self, st: &str, ac: &str) -> Result<(&Criterion, AcRow), FacadeError> {
    let criterion = self.criterion(st, ac)?;
    Ok((criterion, ac_row(self.st_show(st)?, criterion)))
  }

  /// Check the acceptance-test rows against the grammar the GATE enforces.
  ///
  /// It calls the same `contract_report` the close gate calls, deliberately.
  /// A lint with its own copy of the rules is a lint that can say clean while
  /// the gate refuses, and an operator who cannot trust the lint runs the gate
  /// instead -- at which point the lint has no reason to exist.
  ///
  /// The row count comes back with the findings so the report can say what it
  /// examined; see [`contract::ContractReport`].
  pub fn at_lint(&self, st: &str) -> Result<contract::ContractReport, FacadeError> {
    let thread = self.st_show(st)?;
    Ok(contract::contract_report(
      thread,
      None,
      &contract::RepoFiles(self.project.root()),
    ))
  }

  /// `intent search <query>` -- the whole answer: hits grouped by tier, the
  /// index's freshness, and both denominators (AC-19.1, AC-19.2, AC-19.3,
  /// AC-19.5).
  ///
  /// **ONE CALL, AND EVERY SURFACE RENDERS WHAT IT RETURNS.** The terminal,
  /// `--json`, the MCP tool and the explorer's pane differ in how they DRAW
  /// this value and in nothing else. **It is the ONLY search door on the
  /// facade**: the flat section list that used to sit beside it, and the
  /// section COUNT that told a caller whether an empty result meant anything,
  /// both went with WP-21 when the explorer's pane moved onto this envelope --
  /// the count because `index.corpora` already carries it, and the list because
  /// two doors onto one question is how two surfaces come to disagree.
  ///
  /// **THE ONLY TIER TODAY IS LEXICAL, AND IT IS STILL A GROUP.** WP-20's
  /// structural tier is a new group and moves nothing here; cc's source corpus
  /// is a new ENTRY in `index.corpora` and new rows in this same group. That
  /// asymmetry is the design's claim, and building the envelope before the
  /// corpus arrives is what tests it.
  pub fn search_all(
    &self,
    query: &str,
    ask: &crate::search::SearchQuery,
  ) -> Result<crate::search::SearchAnswer, FacadeError> {
    use crate::search::{
      Hit, HitKind, IndexFreshness, Located, SearchAnswer, Span, Tier, TierGroup, Unanswered,
      snippet,
    };

    // ST0076 WP-07: a search by target is refused here, before any tier runs,
    // when the store cannot answer it.
    let level_three = self.level_three(ask)?;

    // **AN UNASKED TIER IS NOT QUERIED AT ALL** -- narrowing the question
    // narrows the work, and a tier whose rows are computed and then discarded
    // would make `--tier` a rendering filter rather than part of the question.
    let expression = crate::fts::expression(query);
    let rows = if Tier::Lexical.asked(&ask.tiers) {
      self.store.search_hits(&expression)
    } else {
      Ok(Vec::new())
    }
    .map_err(|cause| {
      // **A STORE FAULT IS NOT A BAD QUERY** (issue 0443). This mapped every
      // `StoreError::Sqlite` to `BadQuery`, so a malformed index, a busy
      // database or an I/O fault each told the reader their correct query was
      // wrong. Only an expression FTS5 itself refused is the reader's.
      if cause.is_bad_fts5_expression() {
        FacadeError::BadQuery {
          query: query.to_string(),
          cause,
        }
      } else {
        FacadeError::SearchUnanswerable {
          query: query.to_string(),
          cause,
        }
      }
    })?;

    let mut index = IndexFreshness::new(self.corpora()?);
    // Issue 0369: the age of the index this answer read, as its reconcile
    // stamped it.
    index.reconciled_at = self.store.reconciled_at().map_err(FacadeError::Store)?;
    index.resolution = level_three.states.clone();

    // Issue 0370: a file the index holds as skipped is part of the answer's
    // freshness, scoped by the filters as staleness is, so a miss inside one is
    // never a confident miss.
    if Tier::Lexical.asked(&ask.tiers)
      && (ask.kinds.is_empty() || ask.kinds.contains(&HitKind::File))
    {
      for row in self.store.index_files().map_err(FacadeError::Store)? {
        let Some(reason) = row.skipped_reason else {
          continue;
        };
        let in_path = ask
          .path
          .as_deref()
          .is_none_or(|glob| crate::search::glob_matches(glob, &row.path));
        let in_lang = ask.langs.is_empty()
          || row
            .lang
            .as_deref()
            .is_some_and(|lang| ask.langs.iter().any(|asked| asked == lang));
        if in_path && in_lang {
          index.mark_skipped(row.path, reason.as_str());
        }
      }
    }

    let mut hits = Vec::new();
    for row in rows {
      let section = row.section;
      let located = self.locate(&section, row.at);
      let hit = Hit {
        kind: HitKind::of_owner(&section.owner_type),
        name: section
          .heading
          .clone()
          .unwrap_or_else(|| section.file.clone()),
        owner: Some(section.owner_id.clone()).filter(|id| !id.is_empty()),
        lang: None,
        path: section.file.clone(),
        span: match located {
          Located::At(line) => Some(Span::line(line)),
          _ => None,
        },
        score: row.rank,
        snippet: snippet(&section.body, row.at),
        stale: matches!(located, Located::Moved),
        symbol: None,
      };
      // **FRESHNESS DESCRIBES THE ANSWER, NOT THE WHOLE INDEX.** A stale row
      // the filters excluded is not part of what was returned, and warning
      // about it would print a caution about hits the reader cannot see --
      // `--kind issue` reporting that a thread's file moved.
      if ask.keeps(&hit) {
        if hit.stale {
          index.mark_stale(hit.path.clone());
        }
        hits.push(hit);
      }
    }

    // **CODE JOINS THE LEXICAL GROUP AND DOES NOT MAKE ONE OF ITS OWN.** A tier
    // is a group and a corpus is an entry: the source table is a second corpus
    // answered by the same lexical question, with a tokeniser chosen for
    // identifiers rather than prose.
    //
    // Issue 0357: this loop ran whatever `--tier` said, so `--tier structural`
    // counted the code rows into `matched` and then dropped the lexical group
    // that held them.
    let source_rows = if Tier::Lexical.asked(&ask.tiers) {
      self
        .store
        .search_source(&expression)
        .map_err(FacadeError::Store)?
    } else {
      Vec::new()
    };
    for row in source_rows {
      let section = row.section;
      let located = self.locate_body(&section.path, &section.body, row.at);
      let hit = Hit {
        kind: HitKind::File,
        name: section.path.clone(),
        // A file's owner is its path, which is already in `path`.
        owner: None,
        lang: self.lang_of(&section.path),
        path: section.path.clone(),
        span: match located {
          Located::At(line) => Some(Span::line(line)),
          _ => None,
        },
        score: row.rank,
        snippet: snippet(&section.body, row.at),
        stale: matches!(located, Located::Moved),
        symbol: None,
      };
      if ask.keeps(&hit) {
        if hit.stale {
          index.mark_stale(hit.path.clone());
        }
        hits.push(hit);
      }
    }

    // **THE STRUCTURAL TIER IS A GROUP AND NOT MORE ENTRIES IN THE LEXICAL
    // ONE** (the design's claim, and ic's on the envelope side): a tier is a
    // group, a corpus is an entry in `index.corpora`. The group is present
    // whether or not it has hits, because the tier exists in this build -- the
    // store can answer structurally -- and an absent group would read as a tier
    // that is not built.
    let symbols = if Tier::Structural.asked(&ask.tiers) {
      self
        .store
        .symbols_named(query)
        .map_err(FacadeError::Store)?
    } else {
      Vec::new()
    };
    let structural = self.structural_hits(symbols, ask, &level_three, &mut index)?;
    // Issue 0356: a semantic tier asked for BY NAME that cannot answer says why,
    // rather than leaving an absent group to be read as "found nothing".
    let (semantic, unanswered) = if Tier::Semantic.asked(&ask.tiers) {
      match self.semantic_hits(query, ask)? {
        Ok(hits) => (Some(hits), Vec::new()),
        Err(reason) if ask.tiers.contains(&Tier::Semantic) => (
          None,
          vec![Unanswered {
            tier: Tier::Semantic,
            reason: reason.to_string(),
          }],
        ),
        Err(_) => (None, Vec::new()),
      }
    } else {
      (None, Vec::new())
    };

    // **THE SEMANTIC GROUP APPEARS WHEN THE TIER DOES, WHICH IS WHEN THIS
    // PROJECT HAS AN EMBEDDER AND SOMETHING HAS BEEN EMBEDDED.** The other two
    // tiers are built into every binary, so their groups are there whenever
    // they were asked for, hits or not; this one is not built for a project
    // with no `embed` block, and an empty group would claim a tier answered and
    // found nothing.
    //
    // **THAT IS A DIFFERENT ABSENCE FROM `--tier`'s, AND BOTH APPLY**: a tier
    // nobody asked about is absent because the question was narrowed, and a
    // tier this project does not have is absent because there is nothing to
    // ask. The two compose here rather than one shadowing the other.
    let mut groups: Vec<TierGroup> = [
      (Tier::Lexical, Some(hits)),
      (Tier::Structural, Some(structural)),
      (Tier::Semantic, semantic),
    ]
    .into_iter()
    .filter_map(|(tier, hits)| hits.map(|hits| (tier, hits)))
    .filter(|(tier, _)| tier.asked(&ask.tiers))
    .map(|(tier, hits)| TierGroup { tier, hits })
    .collect();

    let (matched, returned) = crate::search::cap(&mut groups, ask.limit);
    Ok(SearchAnswer {
      query: query.to_string(),
      index,
      groups,
      unanswered,
      target: level_three.target,
      matched,
      returned,
    })
  }

  /// Level 3's part of a search answer, and the refusals a search by target
  /// meets (ST0076 WP-07, AC-07.1, AC-07.2).
  ///
  /// **STORED ROWS ANSWER, WHATEVER THIS BUILD'S READERS ARE** (vc, 2026-09-17):
  /// the rows carry their own staleness, so the one refusal on the store's
  /// state is an index where no language has stored, which would otherwise
  /// answer an empty list that reads as a target nothing references. A target
  /// no resolved row names is refused only when resolved targets end the same
  /// way, and names them; otherwise it is answered empty, and the answer says
  /// it cannot tell a target nothing references from a misspelt one.
  fn level_three(&self, ask: &crate::search::SearchQuery) -> Result<LevelThree, FacadeError> {
    let runs = self
      .store
      .resolution(crate::index::symbols::EXTRACTOR_VERSION)
      .map_err(FacadeError::Store)?;
    // **WHETHER A LANGUAGE APPLIES IS ASKED NOW, OF THE INDEX, AND NEVER STORED**
    // (vc, 2026-09-17): a stored verdict would go stale the day the manifest is
    // added, and only a carried language no run has stored for needs asking.
    let mut applies = Vec::new();
    for reader in
      crate::search::manifest_questions(&runs, &self.project.config().languages, &self.resolvers)
    {
      if self
        .store
        .holds_manifest(&reader.manifest)
        .map_err(FacadeError::Store)?
      {
        applies.push(reader);
      }
    }
    let states = crate::search::resolution_states(&runs, &self.resolvers, &applies);
    let stale = runs
      .values()
      .flat_map(|run| run.stale.iter().cloned())
      .collect();
    let stored = runs.values().any(|run| run.resolved_at.is_some());
    let target = match &ask.target {
      None => None,
      Some(target) => {
        if !stored {
          return Err(FacadeError::NothingResolved {
            target: target.clone(),
          });
        }
        let named = self
          .store
          .names_resolved_target(target)
          .map_err(FacadeError::Store)?;
        if !named {
          let resolved = self.store.resolved_targets().map_err(FacadeError::Store)?;
          let near = crate::search::near_targets(target, resolved.iter().map(String::as_str));
          if !near.is_empty() {
            let listed: Vec<String> = near
              .iter()
              .take(NEAR_TARGETS_NAMED)
              .map(|one| format!("`{one}`"))
              .collect();
            let more = near.len() - listed.len();
            return Err(FacadeError::NoSuchTarget {
              target: target.clone(),
              near: match more {
                0 => listed.join(", "),
                more => format!("{} and {more} more", listed.join(", ")),
              },
            });
          }
        }
        Some(crate::search::TargetAsked {
          target: target.clone(),
          named,
        })
      }
    };
    Ok(LevelThree {
      states,
      stale,
      stored,
      target,
    })
  }

  /// The structural hits some symbol rows answer: each given what level 3 says
  /// about it and judged by the query, with the answer's freshness marked for
  /// what it keeps (ST0076 WP-04, WP-07).
  ///
  /// **ONE LOOP FOR BOTH BUILDERS OF AN ANSWER**, `search_all`'s structural
  /// group and [`Self::structural_answer`], which each held a copy of it until
  /// level 3 gave the loop a decision to make.
  ///
  /// **A FILE THAT MOVED ON DISK TAKES NO HIT FROM A SEARCH BY TARGET, AND IS
  /// NAMED** (AC-07.2): its reference joins the target in the index, so leaving
  /// it out without a word would be a silent subset. A file whose resolved rows
  /// went stale is named by `index.resolution` already.
  fn structural_hits(
    &self,
    symbols: Vec<crate::index::symbols::Symbol>,
    ask: &crate::search::SearchQuery,
    level_three: &LevelThree,
    index: &mut crate::search::IndexFreshness,
  ) -> Result<Vec<crate::search::Hit>, FacadeError> {
    let mut rows: std::collections::BTreeMap<String, Vec<crate::index::resolved::Row>> =
      std::collections::BTreeMap::new();
    if level_three.stored {
      for symbol in &symbols {
        if symbol.kind == crate::index::symbols::SymbolKind::Ref && !rows.contains_key(&symbol.path)
        {
          let read = self
            .store
            .resolved_in(&symbol.path)
            .map_err(FacadeError::Store)?;
          rows.insert(symbol.path.clone(), read);
        }
      }
    }
    let untargeted = crate::search::SearchQuery {
      target: None,
      ..ask.clone()
    };
    let mut hits = Vec::new();
    for symbol in symbols {
      let mut hit = self.structural_hit(&symbol);
      let rows = rows.get(&symbol.path).map_or(&[][..], Vec::as_slice);
      crate::search::resolve_hit(&mut hit, symbol.span.start_line, rows, &level_three.stale);
      if ask.keeps(&hit) {
        if hit.stale {
          index.mark_stale(hit.path.clone());
        }
        hits.push(hit);
      } else if hit.stale
        && untargeted.keeps(&hit)
        && ask.target.as_deref().is_some_and(|target| {
          crate::search::joins_target(
            rows,
            &symbol.path,
            symbol.span.start_line,
            &symbol.name,
            target,
          )
        })
      {
        index.mark_stale(hit.path.clone());
      }
    }
    Ok(hits)
  }

  /// The semantic tier's hits, or the reason this project has no such tier.
  ///
  /// **A REASON AND AN EMPTY VECTOR ARE DIFFERENT ANSWERS.** The reason is
  /// "there is no semantic tier here" -- no embedder configured, or nothing
  /// embedded -- and it produces no group at all. An empty vector would be "the
  /// tier ran and matched nothing", which is a claim this build cannot make yet.
  ///
  /// **AND A CONFIGURED EMBEDDER THAT FAILS IS A REFUSAL, NEVER A QUIET
  /// ABSENCE** (IN-AG-NO-SILENT-001). An operator who configured an endpoint
  /// and gets a silently lexical answer has been told their semantic query
  /// found nothing, which is the confident-subset defect AC-19.3 names.
  fn semantic_hits(
    &self,
    query: &str,
    ask: &crate::search::SearchQuery,
  ) -> Result<Result<Vec<crate::search::Hit>, &'static str>, FacadeError> {
    use crate::embed::{EmbedError, cosine};
    use crate::search::{Hit, HitKind};

    let asked = match self.embedder.embed(&[query.to_string()]) {
      Ok(vectors) => vectors,
      Err(EmbedError::NotConfigured) => {
        return Ok(Err("no embedder is configured for this project"));
      }
      Err(other) => return Err(FacadeError::Embed(other)),
    };
    let Some(asked) = asked.first() else {
      return Ok(Err("the embedder returned no vector for the query"));
    };
    let stored = self
      .store
      .embeddings_of(self.embedder.model())
      .map_err(FacadeError::Store)?;
    if stored.is_empty() {
      return Ok(Err(
        "nothing has been embedded for this project's model yet",
      ));
    }

    let mut hits: Vec<Hit> = stored
      .into_iter()
      .filter_map(|row| {
        // **AN INCOMPARABLE VECTOR IS DROPPED, NOT SCORED AT ZERO.** Zero is a
        // real cosine, so a row of the wrong width would land mid-ranking.
        let score = f64::from(cosine(asked, &row.vector)?);
        Some(Hit {
          kind: HitKind::File,
          name: row.chunk_id.clone(),
          owner: None,
          lang: self.lang_of(&row.chunk_id),
          path: row.chunk_id,
          // A vector covers a unit, and nothing chunks yet, so there is no span
          // to claim and no line to show. Both arrive with the chunker.
          span: None,
          score,
          snippet: String::new(),
          stale: false,
          symbol: None,
        })
      })
      .filter(|hit| ask.keeps(hit))
      .collect();
    // **RANKED WITHIN THE TIER, HIGH COSINE FIRST**, and comparable only here:
    // the lexical tier publishes FTS5's rank, where lower is better, which is
    // why the envelope never blends tiers.
    hits.sort_by(|a, b| b.score.total_cmp(&a.score));
    Ok(Ok(hits))
  }

  /// The language the index recorded for a path.
  fn lang_of(&self, path: &str) -> Option<String> {
    self.store.index_lang(path).ok().flatten()
  }

  /// Every corpus the index holds, with how its freshness is decided and how
  /// many files it covers.
  ///
  /// **THE POLICY IS READ FROM THE RULE RATHER THAN SPELLED HERE** -- the
  /// corpus name comes off the row, `corpus::named` turns it back into the
  /// taxonomy, and `freshness::policy_for` says which policy that corpus is
  /// under. A freshness block that named a corpus without saying how its
  /// freshness is decided would tell a reader the question was never asked, and
  /// a second spelling of the two policies would drift from the rule.
  ///
  /// **CANON IS COUNTED FROM THE PROSE TABLE AND THE REST FROM `index_file`**,
  /// because they are different populations: canon's prose belongs to entities
  /// the store holds, and a corpus row is a path on disk.
  ///
  /// **CANON IS NAMED EVEN WHEN IT HOLDS NOTHING, AND A DISK CORPUS IS NOT.**
  /// Canon is intrinsic -- every store has an entity corpus, so a block that
  /// omitted it would read as a build without one, which is the case
  /// `an_empty_index_is_named_in_the_envelope_and_is_not_a_miss` exists to
  /// tell apart from a miss. A disk corpus appears when the index has walked
  /// and found files of that kind; a project with no code has no code corpus,
  /// and naming it with a zero would invent one.
  fn corpora(
    &self,
  ) -> Result<std::collections::BTreeMap<String, crate::search::CorpusState>, FacadeError> {
    use crate::index::corpus::Corpus;
    use crate::index::freshness::policy_for;
    use crate::search::{CorpusState, corpus_key};

    let mut out = std::collections::BTreeMap::new();
    let canon = self
      .store
      .canon_doc_file_count()
      .map_err(FacadeError::Store)?;
    out.insert(
      corpus_key(&Corpus::Canon).to_string(),
      CorpusState {
        policy: policy_for(&Corpus::Canon).as_str().to_string(),
        files: canon,
      },
    );
    for row in self.store.index_files().map_err(FacadeError::Store)? {
      let Some(corpus) = crate::index::corpus::named(&row.corpus) else {
        continue;
      };
      let entry = out
        .entry(corpus_key(&corpus).to_string())
        .or_insert_with(|| CorpusState {
          policy: policy_for(&corpus).as_str().to_string(),
          files: 0,
        });
      entry.files += 1;
    }
    Ok(out)
  }

  /// What a symbol row may still claim about the file it came from: its span,
  /// whether it is stale, and the line to show.
  ///
  /// **THE SPAN IS A CLAIM ABOUT THE DISK AND IS KEPT ONLY WHERE THE BYTES ARE
  /// THE BYTES THAT WERE PARSED** (issue 0195, the same rule the prose tier
  /// takes). A symbol's line came from a parse; if the file has moved since, the
  /// line names something else, and a wrong line is worse than none because it
  /// is believed. The comparison is against the hash the index recorded when it
  /// read the file, which is what that column is for.
  ///
  /// A file that cannot be read makes no claim either way: it is not evidence
  /// the symbol is stale, and `NoClaim` is what the prose tier returns in the
  /// same situation.
  fn symbol_claim(
    &self,
    symbol: &crate::index::symbols::Symbol,
  ) -> (Option<crate::search::Span>, bool, String) {
    let span = crate::search::Span {
      start_line: symbol.span.start_line,
      end_line: symbol.span.end_line,
    };
    let Ok(text) = std::fs::read_to_string(self.project.root().join(&symbol.path)) else {
      return (None, false, String::new());
    };
    let recorded = self.store.index_files().ok().and_then(|rows| {
      rows
        .into_iter()
        .find(|r| r.path == symbol.path)
        .and_then(|r| r.indexed_sha256)
    });
    if recorded.is_some_and(|sha| sha != crate::sync::sha256_of(text.as_bytes())) {
      return (None, true, String::new());
    }
    let line = text
      .lines()
      .nth(symbol.span.start_line.saturating_sub(1) as usize)
      .unwrap_or_default()
      .trim()
      .to_string();
    (Some(span), false, line)
  }

  /// One `symbols` row as a hit in the envelope.
  ///
  /// **ONE PLACE, THREE CALLERS** -- `search_all`'s structural group,
  /// `outline` and `context`. It was inline in the first of those until the
  /// other two arrived; three copies of a mapping that carries decisions (a
  /// symbol has no owner because its path already says where it is; the score
  /// is not a rank and says so) is three places for those decisions to drift.
  fn structural_hit(&self, symbol: &crate::index::symbols::Symbol) -> crate::search::Hit {
    use crate::search::{Hit, HitKind};
    let (span, stale, snippet_line) = self.symbol_claim(symbol);
    Hit {
      kind: match symbol.kind {
        crate::index::symbols::SymbolKind::Def => HitKind::Def,
        crate::index::symbols::SymbolKind::Ref => HitKind::Ref,
      },
      name: symbol.name.clone(),
      // A symbol belongs to a file, and the file is already in `path`.
      owner: None,
      lang: Some(symbol.lang.to_string()).filter(|l| !l.is_empty()),
      path: symbol.path.clone(),
      span,
      // **NOT A RANK, AND SAYING SO.** FTS5 ranks the lexical tier and lower is
      // better; an exact name match has no gradation to report, so every
      // structural hit carries the same best score rather than an invented
      // ordering.
      score: 0.0,
      snippet: snippet_line,
      stale,
      symbol: Some(crate::search::SymbolFacts {
        subkind: symbol.subkind.clone(),
        container: symbol.container.clone(),
        container_kind: symbol.container_kind.clone(),
        trait_name: symbol.trait_name.clone(),
        arity: symbol.arity,
        arity_min: symbol.arity_min,
        qualifier: symbol.qualifier.clone(),
        level: symbol.level,
        resolved: None,
        candidates: Vec::new(),
      }),
    }
  }

  /// `intent search --outline <path>` -- a file's symbols with their spans
  /// (AC-24.3).
  ///
  /// **THIS IS THE ANSWER GREP CANNOT GIVE, AND IT IS WHY THE TIER EXISTS.** An
  /// agent asks what is in a file and today reads the whole file to find out;
  /// this replaces read-the-file with read-this-span, which is where the saving
  /// is. Racing ripgrep on text was never the point.
  ///
  /// **ONE STRUCTURAL GROUP AND NO LEXICAL ONE, DELIBERATELY.** The question
  /// names a PATH, not words, so there is nothing for the lexical tier to
  /// answer -- and an empty lexical group here would say that a text search ran
  /// and found nothing, which is a different and false claim.
  ///
  /// Issue 0358: an outline is what the file DEFINES; the names it merely uses
  /// are listed only when `ask.kinds` names `ref`, which here widens the answer
  /// rather than narrowing it.
  ///
  /// **THE OTHER FILTERS NARROW IT AS THEY NARROW A TEXT QUERY** (ST0076
  /// WP-04): subkind, container, language and path go through
  /// [`crate::search::SearchQuery::keeps`], the one predicate every door uses.
  /// A target asks for the file's references resolved to it, so it lists
  /// references as `ref` does (WP-07).
  pub fn outline(
    &self,
    path: &str,
    ask: &crate::search::SearchQuery,
  ) -> Result<crate::search::SearchAnswer, FacadeError> {
    let refs = ask.kinds.contains(&crate::search::HitKind::Ref) || ask.target.is_some();
    let narrowing = crate::search::SearchQuery {
      kinds: Vec::new(),
      ..ask.clone()
    };
    let symbols = self
      .store
      .symbols_in(path)
      .map_err(FacadeError::Store)?
      .into_iter()
      .filter(|s| refs || s.kind == crate::index::symbols::SymbolKind::Def)
      .collect();
    self.structural_answer(path, symbols, &narrowing)
  }

  /// `intent search --context <name>` -- a definition and its name-matched
  /// references, as source spans (AC-24.3).
  ///
  /// **IT IS THE THING AN AGENT DOES TODAY WITH A GREP, A GLOB AND SEVERAL
  /// READS**, in one call: where this is defined, and where the name occurs.
  ///
  /// **AND THE REFERENCES ARE NAME-MATCHED, WHICH THIS DOOR SAYS RATHER THAN
  /// IMPLIES** (AC-20.5). Nothing here resolves a name to the definition it
  /// points at, so a `ref` is an occurrence of the name and never a CALLER. A
  /// door that answered "callers" would be the confident wrong answer, and the
  /// agent asking has no way to check it.
  ///
  /// **AND IT HONOURS THE FILTERS** (ST0076 WP-04). It ignored every one of them
  /// until then, so `--context new --in AddressError` answered every `new` in
  /// the tree and said nothing about the filter it had dropped.
  pub fn context(
    &self,
    name: &str,
    ask: &crate::search::SearchQuery,
  ) -> Result<crate::search::SearchAnswer, FacadeError> {
    let symbols = self.store.symbols_named(name).map_err(FacadeError::Store)?;
    self.structural_answer(name, symbols, ask)
  }

  /// `intent search --subkind <subkind>` or `--in <container>` with nothing
  /// else to ask -- the symbols that pass the filters, wherever they are
  /// (ST0076 WP-04, AC-04.1).
  ///
  /// **THE FILTERS ARE THE QUESTION.** The methods of a type are asked without
  /// knowing the file it lives in, which is the one thing `--outline` needs, and
  /// `--context` cannot ask them without a name. The store read narrows by
  /// subkind and container, and [`crate::search::SearchQuery::keeps`] judges
  /// every row, as it does for the other structural doors.
  ///
  /// **A TARGET IS THE NARROWEST READ** (ST0076 WP-07, AC-07.2): the written
  /// references a resolved row joins to it, which every other filter then
  /// narrows.
  pub fn filtered(
    &self,
    ask: &crate::search::SearchQuery,
  ) -> Result<crate::search::SearchAnswer, FacadeError> {
    let symbols = match &ask.target {
      Some(target) => self.store.references_to(target),
      None => self
        .store
        .symbols_filtered(&ask.subkinds, ask.container.as_deref()),
    }
    .map_err(FacadeError::Store)?;
    // The envelope's `query` names what was asked, because there is no text to
    // name and an empty one would print as nothing in the empty-index note.
    let mut asked = Vec::new();
    if let Some(target) = &ask.target {
      asked.push(format!("target {target}"));
    }
    if !ask.subkinds.is_empty() {
      asked.push(format!("subkind {}", ask.subkinds.join(" or ")));
    }
    if let Some(container) = &ask.container {
      asked.push(format!("in {container}"));
    }
    self.structural_answer(&asked.join(", "), symbols, ask)
  }

  /// The envelope for a question only the structural tier answers.
  ///
  /// **EVERY FILTER IS HONOURED HERE, THE TIER AND THE CAP INCLUDED** (ST0076
  /// WP-04). A tier filter that leaves the structural tier out is refused
  /// rather than ignored: answering would hand back a tier nobody asked for,
  /// and an empty answer would claim a search ran that this door cannot run.
  fn structural_answer(
    &self,
    query: &str,
    symbols: Vec<crate::index::symbols::Symbol>,
    ask: &crate::search::SearchQuery,
  ) -> Result<crate::search::SearchAnswer, FacadeError> {
    use crate::search::{IndexFreshness, SearchAnswer, Tier, TierGroup};
    if !Tier::Structural.asked(&ask.tiers) {
      return Err(FacadeError::StructuralTierNotAsked);
    }
    let level_three = self.level_three(ask)?;
    // **cc's `corpora()` IS THE ONE HOME AND IT LANDED FIRST.** I had extracted
    // the same block as `freshness()` in the same hour; two methods answering
    // "what does this index hold" is the duplication the extraction was for, so
    // mine went and this calls theirs.
    let mut index = IndexFreshness::new(self.corpora()?);
    index.reconciled_at = self.store.reconciled_at().map_err(FacadeError::Store)?;
    index.resolution = level_three.states.clone();
    let hits = self.structural_hits(symbols, ask, &level_three, &mut index)?;
    let mut groups = vec![TierGroup {
      tier: Tier::Structural,
      hits,
    }];
    let (matched, returned) = crate::search::cap(&mut groups, ask.limit);
    Ok(SearchAnswer {
      query: query.to_string(),
      index,
      groups,
      unanswered: Vec::new(),
      target: level_three.target,
      matched,
      returned,
    })
  }

  /// Where a section's indexed body sits in the file as it now stands
  /// (AC-19.5, issue 0195).
  ///
  /// **THREE OUTCOMES, AND COLLAPSING ANY TWO OF THEM TELLS A LIE.** A file
  /// that is not on disk is not stale: canon realises lazily, so an entity
  /// whose markdown has never been written is exactly as fresh as the store
  /// says. A file whose bytes no longer carry the indexed section IS stale,
  /// and that is the one the reader must be told about. A body that is found
  /// gives a line that is right by construction, because it is found rather
  /// than computed from an offset recorded when it was indexed.
  ///
  /// **WHERE THE INDEXED BYTES OCCUR MORE THAN ONCE, THE FIRST OCCURRENCE IS
  /// NAMED** (vc, 2026-09-12). Either line contains the matched bytes, so the
  /// claim is true of both, and the first is the deterministic choice.
  fn locate(
    &self,
    section: &crate::prose::DocSection,
    at: Option<usize>,
  ) -> crate::search::Located {
    use crate::search::Located;
    if section.body.is_empty() {
      return Located::NoClaim;
    }
    // **A CANON SECTION'S BODY IS A FIELD, NOT A BYTE RANGE**, so the file on
    // disk never contains it verbatim and its absence says nothing about
    // freshness. Asking the question at all here would answer it wrongly for
    // every healthy project.
    let canon = self
      .project
      .relative(&self.project.canon_dir())
      .replace('\\', "/");
    if section.file.starts_with(canon.trim_end_matches('/')) {
      return Located::NoClaim;
    }
    self.locate_body(&section.file, &section.body, at)
  }

  /// **THE 0195 RULE ITSELF**, asked of any indexed body: where it sits in the
  /// file as it now stands, or that it has moved, or that nothing can be
  /// claimed. Both tiers ask it here rather than each carrying its own copy,
  /// because a line is the one thing a search says about the disk and two
  /// statements of when it may be said would differ in the direction that
  /// prints a number.
  fn locate_body(&self, file: &str, body: &str, at: Option<usize>) -> crate::search::Located {
    use crate::search::{Located, find};
    if body.is_empty() {
      return Located::NoClaim;
    }
    let Ok(bytes) = std::fs::read(self.project.root().join(file)) else {
      return Located::NoClaim;
    };
    let Some(found) = find(&bytes, body.as_bytes()) else {
      return Located::Moved;
    };
    let upto = found + at.unwrap_or(0);
    let line = bytes[..upto.min(bytes.len())]
      .iter()
      .filter(|byte| **byte == b'\n')
      .count() as u32
      + 1;
    Located::At(line)
  }

  /// `intent agents generate` -- render `AGENTS.md` from current project
  /// state, writing nothing. A facade GAP closed by a method (vc ruling (c),
  /// 2026-08-30): the renderer composed `install::home` + `rootfiles::render`
  /// inline, so the CLI face owned an operation every other face would have
  /// had to re-compose.
  ///
  /// **NOT `Self::render_ctx`, AND THE DIFFERENCE IS A STORE READ.** Nothing
  /// on this path renders `todo.md`, so there is no watermark to carry and
  /// asking the store for one would be a read with no reader.
  pub fn agents_generate(&self) -> Result<String, FacadeError> {
    let home = crate::install::home()?;
    let ctx = RenderContext {
      version: &self.ctx.version,
      todo_watermark: None,
    };
    Ok(crate::rootfiles::render(
      &home,
      "AGENTS.md",
      self.project.config(),
      &ctx,
    )?)
  }

  /// `intent agents validate` -- the well-formedness census over the file on
  /// disk. Infallible by design: "the file is missing" is a RESULT here, not
  /// an error, because the verb exists precisely to answer that question.
  /// The check body lives in [`crate::rootfiles::validate`] -- validating
  /// generated content is a services concern (vc ruling (c), 2026-08-30).
  pub fn agents_validate(&self) -> crate::rootfiles::AgentsValidation {
    crate::rootfiles::validate(self.project.root())
  }

  /// `intent agents sync` and `intent agents init` -- write `AGENTS.md` from
  /// current project state and record it in the file index, in one act.
  ///
  /// **THE WRITER INDEXES WHAT IT WRITES** (issue 0351). Written by the CLI
  /// alone, the file reached disk and not the index, so a running daemon's
  /// canon watch read the rewrite as an external edit and ingested it.
  pub fn agents_sync(&mut self) -> Result<std::path::PathBuf, FacadeError> {
    let path = crate::rootfiles::generate(
      self.project.root(),
      "AGENTS.md",
      self.project.config(),
      &self.ctx.version,
    )?;
    self.record_written(std::slice::from_ref(&path))?;
    Ok(path)
  }

  /// `intent claude upgrade` -- apply canon, then record in the file index
  /// what it wrote into the corpus the index covers, in one act (0351). A dry
  /// run writes nothing, so it records nothing. It also converges the
  /// formatter exclusion and names each pattern it adds (0378).
  pub fn claude_upgrade(
    &mut self,
    git_hooks: Option<&std::path::Path>,
    opts: crate::canon::Options,
  ) -> Result<ClaudeUpgraded, FacadeError> {
    let home = crate::install::home()?;
    let ctx = RenderContext {
      version: &self.ctx.version,
      todo_watermark: None,
    };
    let applied = crate::canon::apply(
      self.project.root(),
      &home,
      self.project.config(),
      &ctx,
      git_hooks,
      opts,
    )
    .map_err(FacadeError::Canon)?;
    // Issue 0378: a consumer that took canon before the exclusion covered every generated view never received the rest, and no verb converged it.
    let excluded = if opts.report {
      formatter_exclusion_missing(&self.project)
    } else {
      self.record_written(&applied.written)?;
      converge_formatter_exclusion(&self.project).map_err(|source| {
        FacadeError::Canon(crate::canon::CanonError::Unwritable {
          path: self.project.root().join(".prettierignore"),
          source,
        })
      })?
    };
    Ok(ClaudeUpgraded { applied, excluded })
  }

  /// Record in the file index what a writer outside the projection put on
  /// disk, keeping only the paths the canon corpus covers
  /// ([`crate::sync::Scanned::includes`]): canon also writes hooks and
  /// settings the index holds no row for.
  fn record_written(&mut self, written: &[std::path::PathBuf]) -> Result<(), FacadeError> {
    let scope = crate::sync::Scanned::for_root(self.project.root());
    let paths: Vec<std::path::PathBuf> = written
      .iter()
      .filter(|path| scope.includes(path))
      .cloned()
      .collect();
    ingest::record_canon_files(&self.project, &mut self.store, &paths).map_err(FacadeError::Ingest)
  }

  /// Re-read committed canon and rebuild the store from it -- `intent sync`.
  ///
  /// The expensive, infrequent half of the daily-driver split (hv,
  /// 2026-08-14). Ordinary commands answer from the store and never scan the
  /// tree; this is what makes the store agree with the files again after a
  /// `git pull`, a hand edit, or anything else that moved canon behind the
  /// tool's back. WP-08's intentd runs it in the background, at which point
  /// the operator stops needing to.
  ///
  /// It also refreshes the generated views, because a resync that fixed the
  /// store and left the views stale would swap one disagreement for another.
  /// **db -> disk. The ROUTINE direction: rewrite every projected file from
  /// the store.**
  ///
  /// This did not exist until AC-03.9, and its absence was the actual defect
  /// rather than a missing convenience: with the DB as the SSOT (D01, reversed
  /// 2026-08-15) the files are the re-creatable side, so re-creating them is
  /// the operation the model is built around -- and `sync` had only its
  /// dangerous half. Everything else about the old verb was a symptom of that.
  ///
  /// Safe by construction: it reads the source of truth and overwrites
  /// artefacts derived from it. Nothing authored can be lost, because nothing
  /// it writes is authored -- prose lives in modelled fields (D22, D28), which
  /// is what makes the whole projection disposable.
  /// Refuse a scope that names a thread the estate does not have.
  ///
  /// **A scope selecting nothing must not report success.** An id is typed by
  /// hand, so a typo is the common case, and a sync that "completed" over an
  /// empty selection is indistinguishable from one that landed the work --
  /// which leaves the operator believing their thread is saved. Every named id
  /// is checked, not just the first, so `sync ST0056 ST9999` refuses on the
  /// one that is wrong rather than half-running.
  fn check_scope(&self, scope: &SyncScope, threads: &[Thread]) -> Result<(), FacadeError> {
    let Some(named) = scope.named() else {
      return Ok(());
    };
    for id in named {
      if !threads.iter().any(|t| &t.id == id) {
        return Err(FacadeError::NoSuchThread { id: id.clone() });
      }
    }
    Ok(())
  }

  /// **A write path whose input was REFUSED must not then be used as a source
  /// of truth** (AC-03.13).
  ///
  /// The two verbs are opposite directions of one operation and nothing carried
  /// the failure of one into the other, so a refused `sync --to-store`
  /// correctly rolled the store back and a routine `sync --to-disk` then wrote
  /// that stale store over the canon it had declined -- at rc=0, destroying an
  /// authored criterion twice (vc, 2026-08-18).
  ///
  /// **`None` is not a refusal.** A store with no recorded load has no
  /// evidence either way, which is the state of every store written before the
  /// `ingests` table shipped; reading absence as failure would block the egest
  /// across the fleet for something nobody observed.
  ///
  /// It refuses rather than warning, which is the stricter of the two the
  /// criterion allows. A warning on a path that has already succeeded silently
  /// once is a line of output above a completed data loss.
  fn refuse_if_the_last_ingest_was_refused(&self) -> Result<(), FacadeError> {
    let Some(last) = self.store.last_ingest().map_err(FacadeError::Store)? else {
      return Ok(());
    };
    if last.succeeded() {
      return Ok(());
    }
    Err(FacadeError::EgestFromRefusedIngest {
      at: last.updated_at,
      detail: last
        .detail
        .filter(|d| !d.is_empty())
        // An `attempted` row that nothing ever closed: the process died inside
        // the load. There is no detail to give because nothing survived to
        // write one, and saying so is the honest answer -- inventing a cause
        // here would put a guess in the operator's hands under the tool's name.
        .unwrap_or_else(|| "the load did not finish and recorded no cause".to_string()),
    })
  }

  /// **A VERB THAT WOULD REDUCE A POPULATION TO ZERO MUST REFUSE OR NAME IT**
  /// (AC-03.15).
  ///
  /// `sync --to-disk` wrote empty views over a non-empty estate at rc=0:
  /// `steel_threads.md` 57 rows -> 0, `todo.md` 82 -> 0. The store held zero
  /// legitimately, so nothing in the egest was wrong -- which is exactly what
  /// makes the silence possible, and why the check has to be a comparison
  /// against the estate rather than a sanity check inside the write.
  ///
  /// **Compared against the STORE'S canon, not the scope-filtered subset, and
  /// that is deliberate.** A scope narrowing the write to nothing is the
  /// operator's own instruction; an empty STORE empties every view under any
  /// scope, because views render from full canon. So the question is whether
  /// truth is empty, and a scope cannot make it less so.
  ///
  /// **The zero on the disk side is what makes this measurable at all** (and
  /// it is the reason this row is worth more than most): 57 and 82 are non-zero,
  /// observable before the verb runs, and cannot be confused with a correct
  /// answer -- unlike a zero, which is indistinguishable from a legitimately
  /// empty population.
  fn refuse_if_this_would_empty_a_populated_face(
    &self,
    canon: &Canon,
    set: &crate::write_set::WriteSet,
  ) -> Result<(), FacadeError> {
    // **ARM ONE: the canon files this binary can see.** Direct, cheap, and the
    // clearest thing to tell an operator -- "the estate has 57 and the store
    // holds none" needs no interpretation.
    if canon.threads.is_empty() {
      let on_disk = self
        .project
        .thread_ids()
        .map_err(|e| FacadeError::Ingest(e.into()))?
        .len();
      if on_disk > 0 {
        return Err(FacadeError::EgestWouldEmptyTheEstate {
          evidence: format!("the store holds no steel threads and the estate has {on_disk}"),
        });
      }
    }
    if canon.issues.is_empty() {
      let on_disk = self
        .project
        .issue_numbers()
        .map_err(|e| FacadeError::Ingest(e.into()))?
        .len();
      if on_disk > 0 {
        return Err(FacadeError::EgestWouldEmptyTheEstate {
          evidence: format!("the store holds no issues and the estate has {on_disk}"),
        });
      }
    }

    // **ARM TWO, AND ARM ONE DOES NOT SUBSUME IT -- IT IS BLIND TO THE LIVE
    // INSTANCE.**
    //
    // The binary that caused the episode was built from a REVERTED WP-01 TREE,
    // so its canon resolver pointed at the old location: it read zero threads
    // from disk for the same reason it had ingested zero. Canon zero, disk
    // zero, no refusal. **Right verb, right depth, a population that cannot
    // contain the failure** -- an instrument reading its subject through the
    // very assumption that is broken.
    //
    // **What survives a stale resolver is the FACE**, because `steel_threads.md`
    // and `todo.md` did not move in WP-01. So compare the bytes about to be
    // written against the bytes already there: a file that SHRINKS is the
    // estate saying it holds more than the store does, in the one place a wrong
    // resolver cannot have misread.
    //
    // **GATED ON THREADS SPECIFICALLY, AND THE FIRST VERSION OF THIS WAS A
    // FALSE-POSITIVE GENERATOR.** It ran whenever EITHER population was zero --
    // and most projects have no issues at all, so on those it ran on every
    // egest and refused any legitimate shrink: an edited-down objective, a
    // removed work package, a shortened note. **A guard that refuses the
    // ordinary path is worse than the hole it closes**, because it gets
    // disabled rather than fixed.
    //
    // Threads are the right gate because they are the population with a FACE.
    // Issues have no index view -- measured: the write set for a store that
    // lost its issues is seven paths, every one byte-identical and not one of
    // them about issues -- so there is nothing for a shrink to be observed in.
    // A population with no face cannot be protected this way, and pretending
    // otherwise is what produced the false positive.
    if !canon.threads.is_empty() {
      return Ok(());
    }
    //
    // **AND THE SHRINK IS MEASURED IN AUTHORED TEXT** (issue 0446). Measured in
    // bytes, a store holding no threads refused to rewrite an EMPTY estate's
    // `steel_threads.md` over a 2-byte footer change -- an older renderer's
    // backticks -- and called it "the store is behind the estate". Text the
    // renderer owns ([`views::authored_text`]) says nothing about what the
    // estate holds, so it is not counted; a file with no banner is counted
    // whole, because none of it is known to be the renderer's.
    for (path, content) in set.writes() {
      let Ok(disk) = std::fs::read(path) else {
        continue;
      };
      let on_disk = disk.len();
      let authored = |bytes: &[u8]| {
        let text = String::from_utf8_lossy(bytes);
        views::authored_text(&text).map_or(text.len(), |masked| masked.len())
      };
      if authored(&disk) > authored(content) {
        return Err(FacadeError::EgestWouldEmptyTheEstate {
          evidence: format!(
            "{} would go from {on_disk} bytes to {} -- the file on disk carries more than the store does, so the store is behind the estate rather than the other way round",
            path.display(),
            content.len()
          ),
        });
      }
    }
    Ok(())
  }

  /// Reconcile the tree with `.intentfiles` (D57-3, ST0057 WP-04).
  ///
  /// **THIS IS A THIN COORDINATOR AND IT MUST STAY ONE.** Observe, plan, apply.
  /// Every decision -- which of D57-3's five rows a path falls in, whether a
  /// removal is safe, whether the estate may dehydrate at all -- lives in
  /// [`organize`], because a reconciliation rule expressed here would be a
  /// second answer beside the one the acceptance tests drive.
  ///
  /// **THE DIGEST IS RE-OBSERVED THROUGH THE SAME FUNCTION THAT PRODUCED IT.**
  /// `apply` takes the re-observation as a closure so the moment-of-act guard
  /// can be driven in tests without racing a real process; production hands it
  /// the real walk. Passing anything else -- a cached value, a cheaper proxy --
  /// would make the guard compare the tree against itself.
  ///
  /// **AND THE RE-WALK IS PAID FOR ONLY WHEN SOMETHING IRREVERSIBLE IS ABOUT TO
  /// HAPPEN.** `apply` calls the closure exclusively on a plan that will remove,
  /// so a pure hydration walks the tree once.
  /// Reconcile the tree with `.intentfiles` (D57-3).
  ///
  /// **THE MODE IS A PARAMETER RATHER THAN A DEFAULT HERE, AND THE SURFACE OWNS
  /// THE POLARITY.** ic ruled preview-by-default at the command (AC-05.1); this
  /// layer refuses to hold a second opinion about it, because a default living
  /// in two places is how v2 came to ship `--dry-run` on one face and `--write`
  /// on the other.
  /// Record an act that changed the DISK (ST0057 WP-09, AC-09.1).
  ///
  /// **`Facade::apply` is the door for MODEL mutation and this is deliberately
  /// NOT it.** `apply` diffs `next` against loaded canon; the realisation verbs
  /// change no canon at all -- disk is their subject -- so routing them through
  /// it would make the diff a no-op and the event a lie about its own
  /// mechanism. They emit against the same `Store::append_event` instead, and
  /// the subject is the PATH SET rather than an artefact id.
  ///
  /// **The gap this closes was measured rather than supposed**: on 2026-08-19
  /// `organize --apply` removed 423 files from this estate and the log recorded
  /// nothing. Its 55 events at that moment were all model mutations -- `at.set`,
  /// `wp.new`, `st.start` and their kin. **The only act all evening that
  /// destroyed anything was the only class of act absent from the one table
  /// that cannot be re-derived from anything else on disk.**
  ///
  /// **Silent on a no-op, and that is the contract rather than an optimisation.**
  /// An `organize` that moved nothing did not change the disk, so an event for
  /// it would be a record of an act that did not happen -- and a log padded with
  /// non-acts is one a reader stops trusting to mean anything.
  ///
  /// **RECORDED AFTER THE ACT, and the trade is named rather than hidden.**
  /// `realise` records BEFORE its write precisely so a half-finished directory
  /// is still findable; here the act set is not known until the run returns, so
  /// recording early would record a PLAN and call it an act. The cost is real
  /// and bounded: a crash mid-run leaves the partial change unrecorded. Naming
  /// the paths that actually moved is worth more than covering that window with
  /// a claim about paths that might have.
  fn record_disk_act(&mut self, op: &str, payload: serde_json::Value) -> Result<(), FacadeError> {
    let envelope = Envelope::minted(
      &self.ctx.principal,
      &self.ctx.project_id,
      op,
      Subject {
        kind: "paths".to_string(),
        id: self.ctx.project_id.clone(),
      },
      payload,
    );
    self
      .store
      .append_event(&envelope)
      .map_err(FacadeError::Store)?;
    // The act's files have already landed, so its record lands as a set of its
    // own (ST0078 P1): a disk act has no projection to ride in.
    self.land_event_files()
  }

  /// Put the committed file of every event the store has written since the
  /// last call into `set` (ST0078 P1).
  ///
  /// **THE SAME WRITE SET AS THE ACT'S CANON AND VIEWS**, so an act and its
  /// record land together or not at all. The events come from
  /// `Store::take_landed_events`, which is fed by the one place an envelope
  /// becomes a row, so a door that writes an event cannot forget its file; an
  /// event whose set did not land waits for the next act's.
  fn add_event_files(&self, set: &mut WriteSet) -> Result<(), FacadeError> {
    for event in self.store.take_landed_events() {
      if !crate::event::travels(&event.op) {
        continue;
      }
      let (path, body) = event_file_write(&self.project, &event)?;
      set.add(path, body);
    }
    Ok(())
  }

  /// Land the pending events' files as a set of their own, for a door whose act
  /// has no projection to carry them: a disk act, a realisation, a roster read.
  /// What landed is recorded like any written file, so a watching daemon does
  /// not read the store's own write back as an edit.
  fn land_event_files(&mut self) -> Result<(), FacadeError> {
    let mut set = WriteSet::new();
    self.add_event_files(&mut set)?;
    if set.is_empty() {
      return Ok(());
    }
    let applied = set
      .commit()
      .map_err(|cause| FacadeError::ViewsNotWritten { cause })?;
    let landed: Vec<std::path::PathBuf> = applied.written().map(std::path::PathBuf::from).collect();
    applied.keep();
    self.record_landed(&[], &landed)
  }

  /// The paths an act really changed.
  ///
  /// **What LANDED, not what was asked for.** `commit` skips a path whose bytes
  /// already match, so the [`WriteSet`] is a second opinion about what a sync
  /// writes -- it answers what WOULD be written, which is the right question
  /// before a commit and the wrong one after it.
  ///
  /// **It carried an event-log exclusion until D53 and no longer needs one.**
  /// `sync_to_disk` used to project the log inside its own write set, so a run
  /// recording its own act staled the file and handed the next sync real work
  /// -- each sync manufacturing the next one's, forever. The filter terminated
  /// that regress; deleting the tracked file removed its cause, which is the
  /// better repair and is why the filter is gone rather than kept as defence.
  fn estate_paths(&self, applied: &crate::write_set::Applied) -> Vec<String> {
    applied
      .written()
      .map(|path| self.project.relative(path))
      .collect()
  }

  /// **ONE READ-ONLY SQL STATEMENT OVER THIS STORE** (AC-17.1 to AC-17.4).
  ///
  /// The door the agent guide cannot replace: `intent search` answers text, and
  /// this answers a join across the model that no verb has. It is `&self`
  /// because it cannot write, and the signature says so before any mechanism
  /// does.
  ///
  /// **FIVE THINGS STAND BETWEEN A STATEMENT AND THE STORE, and each one has a
  /// hole the others cover.**
  ///
  /// 1. The connection is opened READ-ONLY and is not the live one, so the
  ///    guarantee does not depend on anybody restoring a pragma.
  /// 2. `query_only` is set on it, so the refusal an operator meets is about
  ///    the rule rather than an errno about a file.
  /// 3. An AUTHORIZER allows `Select`, `Read`, `Function` and `Recursive` and
  ///    denies everything else -- which is what refuses `ATTACH`. A read-only
  ///    connection reads ANOTHER file on this machine perfectly happily, and
  ///    this door is exposed on MCP.
  /// 4. ONE statement, checked before prepare by [`crate::sql_gate`], because
  ///    SQLite prepares the first and hands back the rest as a tail.
  /// 5. A row cap and a time bound, so a cross join costs a refusal rather than
  ///    the process.
  ///
  /// **THE SCHEMA VERSION IS READ BEFORE THE AUTHORIZER IS INSTALLED**, and
  /// that ordering is load-bearing: `PRAGMA user_version` is a pragma, and an
  /// authorizer that denies pragmas would deny this door's own read of the
  /// version it has to report.
  ///
  /// **BOTH DENOMINATORS COME FROM ONE EXECUTION.** Rows stream, the first
  /// `cap` are kept and the rest are counted. Wrapping the statement in
  /// `select count(*) from (...)` would run somebody else's SQL twice, and two
  /// executions are two truths.
  pub fn search_sql(&self, statement: &str, limit: Option<usize>) -> Result<SqlPage, FacadeError> {
    use rusqlite::hooks::{AuthAction, Authorization};

    crate::sql_gate::single_statement(statement).map_err(|refusal| match refusal {
      crate::sql_gate::GateRefusal::MoreThanOneStatement => FacadeError::SqlMoreThanOneStatement,
      crate::sql_gate::GateRefusal::Empty => FacadeError::SqlNoStatement,
      crate::sql_gate::GateRefusal::Unterminated => FacadeError::SqlUnterminated,
    })?;

    let cap = limit.unwrap_or(SQL_ROWS_DEFAULT);
    if cap > SQL_ROWS_CEILING {
      return Err(FacadeError::SqlLimitAboveCeiling {
        asked: cap,
        ceiling: SQL_ROWS_CEILING,
      });
    }

    let conn = self
      .store
      .read_only_connection()
      .map_err(FacadeError::Store)?;
    let schema_version: i32 = conn
      .pragma_query_value(None, "user_version", |row| row.get(0))
      .map_err(|e| FacadeError::SqlDidNotRun {
        detail: e.to_string(),
      })?;

    // **WHAT THE AUTHORIZER DENIED, KEPT, so the refusal can name it.** The
    // error SQLite returns says only "not authorized"; which door the statement
    // reached for is the operator's whole question, and only the callback knows.
    let denied: std::sync::Arc<std::sync::Mutex<Option<String>>> =
      std::sync::Arc::new(std::sync::Mutex::new(None));
    let recorder = std::sync::Arc::clone(&denied);
    // **A GUARD THAT DID NOT INSTALL REFUSES THE STATEMENT.** In rusqlite 0.40
    // registering a hook can fail (it checks that the connection is owned), and
    // running somebody else's SQL without the authorizer or the budget is the
    // one outcome this door exists to prevent, so both registrations below
    // propagate.
    conn
      .authorizer(Some(
        move |ctx: rusqlite::hooks::AuthContext<'_>| match ctx.action {
          AuthAction::Select | AuthAction::Read { .. } | AuthAction::Function { .. } => {
            Authorization::Allow
          }
          AuthAction::Recursive => Authorization::Allow,
          // **ONE PRAGMA, BY NAME, AND ONLY AS A READ** -- the door refused every
          // FTS5 table until this existed, including a plain `select path from
          // src_sections limit 2`, with a message blaming the operator for a
          // PRAGMA they did not write. **SQLite's fts5 module issues
          // `PRAGMA data_version` itself** when a virtual table is initialised,
          // so the blanket refusal below shut the door on exactly the tables the
          // door was built to join against: the whole stated purpose of `--sql`
          // is a question that crosses the model and the INDEX.
          //
          // **THE NAME WAS READ OFF A LOGGING AUTHORIZER AGAINST THE REAL TABLE,
          // NOT GUESSED** (ic, 2026-09-12, driven on `src_sections`): the set is
          // `data_version` and nothing else. A CATEGORY -- "allow read pragmas"
          // -- was the obvious fix and is the wrong one: this connection is the
          // store's and outlives the statement, so a pragma that CHANGED
          // something would outlive it too, and the read-only open flag would be
          // the only thing left standing. Hence a name, and hence
          // `pragma_value.is_none()`: `PRAGMA data_version` is a read and
          // `PRAGMA data_version = x` is not the same act, whatever it would do.
          AuthAction::Pragma {
            pragma_name: "data_version",
            pragma_value: None,
            ..
          } => Authorization::Allow,
          other => {
            if let Ok(mut slot) = recorder.lock() {
              slot.get_or_insert_with(|| action_name(&other));
            }
            Authorization::Deny
          }
        },
      ))
      .map_err(|e| FacadeError::SqlDidNotRun {
        detail: e.to_string(),
      })?;

    // **THE BUDGET IS SPENT IN WORK, NOT IN SECONDS** -- see [`SQL_WORK_BUDGET`]
    // for why this workspace cannot read a clock. The unit is SQLite
    // virtual-machine instructions rather than rows, which is the case a row cap
    // cannot reach: a cross join that returns nothing runs forever and returns
    // no rows at all.
    let mut spent: u64 = 0;
    conn
      .progress_handler(
        SQL_WORK_INTERVAL,
        Some(move || {
          spent += 1;
          spent > SQL_WORK_BUDGET
        }),
      )
      .map_err(|e| FacadeError::SqlDidNotRun {
        detail: e.to_string(),
      })?;

    let refused = |e: rusqlite::Error| -> FacadeError {
      if let Some(name) = denied.lock().ok().and_then(|slot| slot.clone()) {
        return match name.as_str() {
          "INSERT" | "UPDATE" | "DELETE" | "CREATE TABLE" | "DROP TABLE" | "ALTER TABLE" => {
            FacadeError::SqlWouldWrite
          }
          other => FacadeError::SqlOutOfReach {
            action: other.to_string(),
          },
        };
      }
      match e.sqlite_error_code() {
        Some(rusqlite::ErrorCode::ReadOnly) => FacadeError::SqlWouldWrite,
        Some(rusqlite::ErrorCode::OperationInterrupted) => FacadeError::SqlOverBudget,
        _ => FacadeError::SqlDidNotRun {
          detail: e.to_string(),
        },
      }
    };

    let mut stmt = conn.prepare(statement).map_err(&refused)?;
    let columns: Vec<String> = stmt
      .column_names()
      .into_iter()
      .map(str::to_string)
      .collect();
    let width = columns.len();
    let mut kept: Vec<Vec<serde_json::Value>> = Vec::new();
    let mut matched = 0usize;
    let mut rows = stmt.query([]).map_err(&refused)?;
    while let Some(row) = rows.next().map_err(&refused)? {
      matched += 1;
      if kept.len() < cap {
        let mut out = Vec::with_capacity(width);
        for i in 0..width {
          out.push(sql_value(row.get_ref(i).map_err(&refused)?));
        }
        kept.push(out);
      }
    }

    Ok(SqlPage {
      columns,
      returned: kept.len(),
      rows: kept,
      matched,
      schema_version,
    })
  }

  /// Read the history. **The store is the only home it has**, so this touches
  /// no disk at all -- the tracked extract was deleted and the file form is
  /// produced by `export` rather than kept projected.
  ///
  /// **Newest LAST and the limit takes from the END.** The rows are ULID-
  /// ordered, so a limit that took the first N would answer with the oldest
  /// history in the store, which is the opposite of what anyone asking for
  /// "the last 20" means -- and it would look perfectly plausible.
  pub fn events(&self, filter: &EventFilter) -> Result<EventPage, FacadeError> {
    let all = self.store.events().map_err(FacadeError::Store)?;
    let total = all.len();
    let matched: Vec<Envelope> = all
      .into_iter()
      .filter(|e| filter.op.as_ref().is_none_or(|op| &e.op == op))
      .filter(|e| filter.subject.as_ref().is_none_or(|id| &e.subject.id == id))
      .collect();
    let count = matched.len();
    let rows = match filter.limit {
      Some(n) if n < count => matched[count - n..].to_vec(),
      _ => matched,
    };
    Ok(EventPage {
      rows,
      matched: count,
      total,
    })
  }

  /// Open `.intentfiles` for a verb that is about to ACT on what it says.
  ///
  /// **THREE STATES, KEPT APART, BECAUSE TWO OF THEM WERE THE SAME ONE AND THE
  /// COLLAPSE WAS THE DEFECT (AC-04.7).** Both callers below used to open the
  /// file with a bare `read_to_string` mapped to `ManifestUnreadable`, so an
  /// ABSENT manifest -- **the shipped initial condition of every project
  /// `intent init` creates** -- refused to run and reported the estate broken.
  /// `intentfiles::realised()` had modelled hv's rule completely and correctly
  /// the whole time, four files away, and nothing that ACTED consulted it:
  /// **one rule, one correct model, three readers of which one used it.**
  ///
  /// - **ABSENT: not an error.** Nobody has said, so everything is realised.
  ///   The verb proceeds and removes nothing.
  /// - **PRESENT AND UNPARSEABLE: refused, with the line AND the path.**
  ///   Fail-open belongs to reporters; a verb about to remove files must never
  ///   act on a declaration it could not read.
  /// - **PRESENT AND UNREADABLE: refused.** A permissions fault is not an
  ///   absence, and folding it into one would let a broken mount read as
  ///   "nobody has said" and silently dehydrate the estate.
  ///
  /// **THE SAME RULE, INLINE, IS AT [`Facade::edit_list`]** (ic,
  /// AC-05.2) -- and it is not extracted into this helper because the two ask
  /// different questions: that one needs the manifest's TEXT in order to
  /// rewrite it, this one needs its DECLARATION in order to plan against it.
  /// Sharing a return type would make one of them convert back.
  fn manifest_for_action(&self) -> Result<intentfiles::Realised, FacadeError> {
    let path = self.project.intentfiles_path();
    match std::fs::read_to_string(&path) {
      Ok(raw) => {
        intentfiles::realised_for_action(&raw).map_err(|cause| FacadeError::ManifestMalformed {
          path: path.display().to_string(),
          cause,
        })
      }
      Err(source) if source.kind() == std::io::ErrorKind::NotFound => {
        Ok(intentfiles::Realised::NothingSaid)
      }
      Err(source) => Err(FacadeError::ManifestUnreadable {
        path: path.display().to_string(),
        source,
      }),
    }
  }

  /// Reconcile the tree against the declaration.
  ///
  /// **UNPINNED, WHICH IS RIGHT FOR A CALLER THAT IS NOT SHOWING A PLAN TO
  /// ANYBODY.** A caller that DOES show one -- every interactive face, by hv's
  /// 2026-09-12 ruling on silent deletion -- must use
  /// [`Facade::organize_as_shown`] and pin the act to the plan it rendered.
  pub fn organize(&mut self, mode: organize::Mode) -> Result<organize::Report, FacadeError> {
    self.organize_as_shown(mode, None)
  }

  /// The same reconciliation, pinned to a plan that has already been RENDERED.
  ///
  /// **A PREVIEW AND THE `--apply` THAT FOLLOWS IT ARE TWO RUNS, AND THE SECOND
  /// ONE RE-PLANS** (hv, 2026-09-12: *silent deletion ... THEY NEED
  /// IDENTIFYING, TRIAGING, AND FIXING, AS A MATTER OF URGENCY*). Between them
  /// the estate can move -- every read verb materialises the store on access,
  /// so a peer running `intent st list` is enough -- and the act would then
  /// perform a plan NOBODY WAS SHOWN, with the preview's reassurance standing
  /// over it. That is worse than no preview at all: it is a specific promise
  /// about which files go, made about a different run.
  ///
  /// `shown` is the digest of the plan the caller rendered (`Report::digest`).
  /// If this run's plan does not carry the same digest, the run REFUSES and
  /// removes nothing rather than acting on the difference.
  ///
  /// **THIS IS NOT THE MOMENT-OF-ACT GUARD AND DOES NOT REPLACE IT.** That one
  /// stands between THIS run's plan and its own irreversible step, inside
  /// `Plan::run`; this one stands between the plan a HUMAN READ and the plan
  /// about to be performed. Two different windows, and only the second is
  /// closed by a person having looked.
  pub fn organize_as_shown(
    &mut self,
    mode: organize::Mode,
    shown: Option<&str>,
  ) -> Result<organize::Report, FacadeError> {
    self.organize_run(mode, shown, false)
  }

  /// Remove the views of every thread the manifest leaves undeclared, through
  /// organize's own plan, gate and record, and nothing else (issue 0316).
  ///
  /// `upgrade` is the caller: it is the one verb that knows it has just changed
  /// the declaration, and the one place a view an earlier binary rendered for a
  /// now-undeclared thread can be given an owner without a new rule. Its
  /// refusals are organize's -- a hand edit, an unmet precondition -- and are
  /// reported, never bypassed.
  pub fn dehydrate_undeclared_thread_views(&mut self) -> Result<organize::Report, FacadeError> {
    self.organize_run(organize::Mode::Apply, None, true)
  }

  /// organize's plan for the tree as it stands, rendered from `canon`, with
  /// the file index it was observed against.
  ///
  /// **ONE HOME FOR THE OBSERVATION AND THE PLAN**, shared by every organize
  /// run and by `intent sync`'s preview of the views, which renders from the
  /// model as it will stand after the ingest rather than from the store.
  fn organize_plan_over(
    &self,
    canon: &Canon,
  ) -> Result<(organize::Plan, Vec<crate::sync::FileEntry>), FacadeError> {
    let realised = self.manifest_for_action()?;
    let previous = self.store.file_index().map_err(FacadeError::Store)?;
    let (tree, digest) =
      organize::observe(&self.project, &previous).map_err(FacadeError::Organize)?;
    let ctx = self.render_ctx()?;
    let plan = organize::plan(&self.project, canon, &realised, &ctx, &tree, digest);
    Ok((plan, previous))
  }

  /// One run of organize, shared by [`Facade::organize_as_shown`] and
  /// [`Facade::dehydrate_undeclared_thread_views`], so the second is the first
  /// with its steps narrowed and never a second implementation of either.
  fn organize_run(
    &mut self,
    mode: organize::Mode,
    shown: Option<&str>,
    undeclared_thread_views_only: bool,
  ) -> Result<organize::Report, FacadeError> {
    // **NOTHING REGENERATES `.intentfiles`, BY hv's RULING (`d2b63bc3`).** organize
    // is: read the list, hydrate what is in it, dehydrate what is on disk and
    // is not. **Status has no vote here at all.**
    //
    // A previous version of this function rewrote a generated region from a
    // declared function of status, and it was removed rather than fixed. The
    // two-region design existed ONLY because the file was machine-written: if
    // organize rewrote the list every run, a hand-added line would be wiped, so
    // a protected region was needed. **Take away the regeneration and the
    // protected region has nothing to protect against.**
    //
    // It also settles a mystery this estate spent an evening on:
    // `intentfiles::render` had no production caller because **the thing it
    // does is not needed**, not because anybody forgot to wire it.
    let (plan, previous) = self.organize_plan_over(&self.canon)?;
    // Issue 0316: a thread's generated view reaches a Dehydrate step only when
    // the thread is undeclared, so "a view with a thread owner" is exactly the
    // undeclared threads' views -- attachments and issues stay out.
    let plan = if undeclared_thread_views_only {
      let views: std::collections::BTreeSet<std::path::PathBuf> = {
        let ctx = self.render_ctx()?;
        views::render_all(&self.project, &self.canon, &ctx)
          .into_iter()
          .map(|v| v.path)
          .collect()
      };
      let canon = &self.canon;
      plan.only_dehydrating(|p| views.contains(p) && self.owning_thread(p, canon).is_some())
    } else {
      plan
    };

    // **THE PIN IS TESTED BEFORE THE RUN, NOT INSIDE IT.** `Plan::run` is
    // reached by callers who showed nobody anything; the question *is this the
    // plan that was rendered* is the CALLER's, and answering it here keeps
    // `run` answering only its own.
    if let Some(shown) = shown
      && plan.digest != shown
    {
      return Err(FacadeError::Organize(organize::OrganizeError::TreeMoved {
        detail: format!(
          "the plan you were shown measured the tree as {shown} and it now measures {}, so the removals about to run are not the ones that were printed",
          plan.digest
        ),
      }));
    }

    let project = &self.project;
    let report = plan
      .run(mode, &|| {
        organize::observe(project, &previous)
          .map(|(_, digest)| digest)
          // **A FAILED RE-OBSERVATION MUST NOT READ AS AN UNCHANGED TREE.** The
          // guard compares this against the planned digest, so returning the
          // planned value on error would say "nothing moved" precisely when
          // nothing is known -- and the removals would proceed. A sentinel that
          // can never equal a sha256 refuses instead.
          .unwrap_or_else(|_| "tree-could-not-be-re-read".to_string())
      })
      .map_err(FacadeError::Organize)?;

    // **A PREVIEW CHANGED NOTHING, SO IT RECORDS NOTHING.** `Mode::Preview`
    // decides everything and touches the tree not at all; an event for it would
    // put a decision in the record of acts, which is the one distinction this
    // verb's two modes exist to keep.
    if mode.performs() {
      let rel = |ps: &[std::path::PathBuf]| -> Vec<String> {
        let mut v: Vec<String> = ps.iter().map(|p| self.project.relative(p)).collect();
        v.sort();
        v
      };
      // The four ACTS. `unchanged`, `unclaimed` and `diverged` are findings
      // about the tree rather than changes to it, and a log that carried them
      // would answer "what happened" with a list of things that did not.
      let hydrated = rel(&report.hydrated);
      let rewritten = rel(&report.rewritten);
      let dehydrated = rel(&report.dehydrated);
      let pruned = rel(&report.pruned);
      if !(hydrated.is_empty()
        && rewritten.is_empty()
        && dehydrated.is_empty()
        && pruned.is_empty())
      {
        // **A RECEIPT THAT FAILS AFTER A CORRECT ACT DOES NOT UNDO THE ACT**
        // (vc, ruled 2026-09-14). The removals and hydrations above are done,
        // so this run reports them, and the event log's failure is the note.
        // Issue 0376: a locked event log reported "the change was not made" over removals already on disk.
        if let Err(cause) = self.record_disk_act(
          "disk.organize",
          serde_json::json!({
            "hydrated": hydrated,
            "rewritten": rewritten,
            "dehydrated": dehydrated,
            "pruned": pruned,
            "refused": report.refused.len(),
          }),
        ) {
          self.after_write.push(Note::after_write(
            "recording this run in the event log",
            &cause,
            UNRECORDED_REMEDY,
          ));
        }
      }
    }
    Ok(report)
  }

  /// Make an addressed artefact's files exist on disk, and say which ones do.
  ///
  /// **TWO INDEPENDENT IDEMPOTENT STEPS, NEITHER GUARDING THE OTHER** (ic's
  /// correction, and it killed a defect before it existed). Materialise if
  /// absent, then list if not listed. The obvious shape -- return early when
  /// the files are already there -- skips the LIST step in an ordinary case: a
  /// thread `st done` has just delisted keeps its files until the next
  /// `organize`, so they are present and the list does not name them. **Files
  /// on disk and a line in the list are different facts, and `organize`
  /// answers only to the second.** Until issue 0338 carried out D57-9 the trap
  /// had a second form, an id in the generated region that was present and
  /// not pinned; the flat list removed that form and left this one.
  ///
  /// **Listing is what makes the decision outlive the run.** A line in the list
  /// is a durable statement that this artefact stays on disk. Hydrating without
  /// listing hands the files straight back to the next `organize`.
  ///
  /// **IT DISPATCHES ON `entity` AND IGNORES `format`** (ic). `?format=json` and
  /// `?format=md` name the SAME artefact and must realise identically, so a verb
  /// that read the format would have acquired an opinion about REPRESENTATION
  /// that AC-05.1 never asked for. A non-empty `authority` is refused rather
  /// than ignored: that names a DIFFERENT PROJECT, and realising another
  /// project's artefact into this tree is not a representation question.
  ///
  /// **AND IT REUSES `organize`'s PLAN RATHER THAN RESTATING WHAT AN ARTEFACT
  /// OWNS.** The plan is computed for the whole estate and then FILTERED to this
  /// artefact's steps, so the classification -- which paths a thread owns, which
  /// are renderable, which are attachments the store carries -- has exactly one
  /// expression. A second list here would be the fourth answer to "what files
  /// does this artefact have", and the one that goes stale is always the one
  /// nobody is looking at when a new view kind lands.
  pub fn hydrate(&mut self, address: &Address) -> Result<Vec<std::path::PathBuf>, FacadeError> {
    self.hydration(address).map(|h| h.paths)
  }

  /// Every file a thread carries, whether or not it is on disk, sorted.
  ///
  /// **ONE EXPRESSION, FILTERED** -- [`Facade::carried_paths`] asks the two
  /// owners of the answer for the whole estate (`views::render_all` is THE
  /// renderer; canon's `attachments` is THE store's own list), and this is that
  /// set narrowed to one artefact's directory. It used to ask them a second
  /// time here, which is two homes for "what files does this artefact have" and
  /// the index's own scope now reads the same answer. A pure read, so `edit`
  /// can refuse on it before `hydrate` writes anything (0145).
  fn carried(&self, id: &str) -> Result<Vec<std::path::PathBuf>, FacadeError> {
    let home = self.project.thread_dir(id);
    Ok(
      self
        .carried_paths()?
        .into_iter()
        .filter(|p| p.starts_with(&home))
        .collect(),
    )
  }

  /// [`Facade::hydrate`], with which of its paths THIS CALL WROTE (0083).
  ///
  /// `hydrate`'s answer is the paths that now exist, and it is idempotent, so
  /// a restore and a no-op returned the same set and a report built on it
  /// labelled a file it had just recreated `exists:`. The written subset is
  /// what the organize run already knows -- created or rewritten -- so the
  /// report reads it here rather than guessing from the disk.
  pub fn hydration(&mut self, address: &Address) -> Result<Hydration, FacadeError> {
    self.hydration_overwriting(address, false, &mut |_| {})
  }

  /// The same realisation, with the one question that decides whether it may
  /// destroy work: **may it write over a view whose bytes on disk are not what
  /// the store renders?**
  ///
  /// **IT REFUSED NOTHING AND ASKED NOTHING UNTIL 2026-09-12** (hv: silent
  /// deletion). Every `Verify` step went into the write set unconditionally, so
  /// a hand edit to `info.md` -- or any work an unregistered writer had put
  /// there -- was replaced by the render, reported afterwards as `wrote`, at
  /// exit 0. **`dehydrate` has refused exactly this since it was written**:
  /// `organize::gate` will not REMOVE a file whose bytes differ from the render
  /// because the difference may be a hand edit. Overwriting it is the same loss
  /// by a different verb, and the two must not disagree.
  ///
  /// `overwrite` is the operator saying they know. It is not a default and
  /// there is no environment variable for it.
  ///
  /// `announce` is handed every path the run is ABOUT TO WRITE OVER, before it
  /// writes one, for the reason `dehydrate_announcing` takes a callback: what
  /// to say is the renderer's and what is about to go is this layer's. It fires
  /// only when `overwrite` is letting a divergence through -- an ordinary
  /// realisation writes over nothing anybody could miss.
  pub fn hydration_overwriting(
    &mut self,
    address: &Address,
    overwrite: bool,
    announce: &mut dyn FnMut(&[std::path::PathBuf]),
  ) -> Result<Hydration, FacadeError> {
    require_local(address)?;
    let Some((sigil, id)) = address.entity.artefact() else {
      return Err(FacadeError::NotHydratable {
        form: address.entity.form(),
        why: "only an artefact -- a steel thread or an issue -- is named by `.intentfiles`, so it is the smallest thing realisation can address".to_string(),
      });
    };
    let id = id.to_string();

    // **STEP ZERO: RESOLVE. An id naming no thread is not realised, and this
    // runs before the pin because the pin is a WRITE to a TRACKED file.**
    // `intent st edit ST9997 design` appended `STEELTHREAD:ST9997` to
    // `.intentfiles` and then refused, so a typo entered a shared file to be
    // committed by whoever committed that path next, under their authorship,
    // in a commit about something else (`intent#0144`).
    //
    // **IT SITS HERE RATHER THAN IN `edit` BECAUSE THE DIRECT DOOR IS THE
    // WORSE HALF.** `intent st hydrate ST9997` planted the same line and
    // exited **0**, printing `hydrated -- listed in intent/.intentfiles` about
    // a thread that does not exist; `edit` at least refused afterwards.
    // Measured on both doors before choosing, because a guarded door usually
    // has an unguarded twin one command away -- and guarding `edit` alone
    // would have left the silent one open while looking complete.
    //
    // **AND IT IS WHAT MAKES A TYPO DIAGNOSABLE.** Neither door could say the
    // id was unknown: `edit` answered `is not a file this artefact carries`
    // for an authored file and `is generated from the model` for a view, both
    // describing a FILE in a thread that was never there. `NoSuchThread` names
    // the thing the operator actually got wrong.
    //
    // **THE SECOND ARTEFACT KIND ARRIVED, AND THIS IS THE ARM THE PARAGRAPH
    // ABOVE ASKED FOR.** It read *artefact is steel thread today ... a second
    // artefact kind would need its own arm here rather than falling through to
    // this one*, and falling through is exactly what an issue did the moment
    // `Entity::artefact` started answering for one: `st_show("0021")` refused
    // with `NoSuchThread { id: "0021" }`, naming the wrong KIND of thing in a
    // message whose whole purpose is telling an operator what they got wrong.
    self.resolve_artefact(sigil, &id)?;

    // STEP ONE: PIN. First, and unconditionally, because it is the step the
    // obvious ordering skips.
    let path = self.project.intentfiles_path();
    let pinned = match std::fs::read_to_string(&path) {
      Ok(before) => Some(before),
      // **AN ABSENT MANIFEST IS LEFT ABSENT, AND THE ALTERNATIVE IS
      // DESTRUCTIVE RATHER THAN MERELY DIFFERENT.** Creating one here to hold
      // this single entry would declare that this id is THE WHOLE of what is
      // realised, and the next `organize` would remove every other thread's
      // files on the strength of one `intent edit`. Nobody has said, so
      // everything is already realised and there is nothing for a pin to add
      // -- the no-op is the rule applying, not a case being skipped. hv ruled
      // this for the lifecycle verbs; `edit_list` is the same arm,
      // written by ic, and AC-04.7 states expressly that it does not decide
      // this one beyond requiring that absence not be REPORTED as unreadable.
      Err(source) if source.kind() == std::io::ErrorKind::NotFound => None,
      // Any other IO fault is raised. A file that exists and cannot be read is
      // not a file that does not exist, and letting a permissions error answer
      // as "nobody has said" is the silent swallow this estate forbids.
      Err(source) => {
        return Err(FacadeError::ManifestUnreadable {
          path: path.display().to_string(),
          source,
        });
      }
    };
    // **`false` FOR THE ABSENT CASE IS A FACT ABOUT THE RUN, NOT A DEFAULT.**
    // This value is what `disk.hydrate` records as `pinned`, and nothing was
    // pinned: no manifest was read, none was written, and none was created.
    // Reporting `true` because the artefact is realised would record an act
    // that did not happen, in the log this estate treats as the durable one.
    let pin_moved = match pinned {
      None => false,
      Some(before) => {
        let after =
          intentfiles::pin(&before, sigil, &id, None).map_err(FacadeError::Intentfiles)?;
        if after != before {
          let mut set = WriteSet::new();
          set.add(path, after);
          set.commit()?.keep();
          true
        } else {
          false
        }
      }
    };

    // STEP TWO: MATERIALISE. Independent of the first -- it runs whether or not
    // the pin moved, and the pin ran whether or not this will write anything.
    let realised = self.manifest_for_action()?;
    let previous = self.store.file_index().map_err(FacadeError::Store)?;
    let (tree, digest) =
      organize::observe(&self.project, &previous).map_err(FacadeError::Organize)?;
    let whole = {
      let ctx = self.render_ctx()?;
      organize::plan(&self.project, &self.canon, &realised, &ctx, &tree, digest)
    };

    // **SCOPED TO THIS ARTEFACT'S DIRECTORY, AND THE FILTER IS WHY THIS IS NOT
    // AN `organize`.** `intent edit ST0001` must not reconcile the estate; it
    // must make one artefact's files exist. The plan is whole-estate because
    // classification needs the whole estate as its denominator, and the ACT is
    // narrow.
    // **TWO ARMS AGAIN, AND THE ISSUE ONE NOW ADDRESSES THE ESTATE RATHER THAN
    // CANON -- which is the whole of what was wrong with it.** It matched
    // `Sigil::Issue` to `issues_dir()`, `intent/.canon/issues/`, so a
    // realisation verb's home resolved into CANON for one of its two inputs.
    // It was inert only because `organize::plan` emits no step under
    // `intent/.canon/`, which is a property of the plan and not a bound this
    // code stated. hv retired `ISSUE:` on 2026-08-20; ST0069 WP-01 gives an
    // issue a realised form, so the arm returns `issue_view`, which is a FILE
    // rather than a directory. `starts_with` is still the right filter: a path
    // starts with itself, so the narrowing selects exactly that one view.
    let home = match sigil {
      intentfiles::Sigil::SteelThread => self.project.thread_dir(&id),
      intentfiles::Sigil::Issue => issue_home(&self.project, &id)?,
    };
    // **A HELD THREAD IS REFUSED HERE, NOT REPORTED** (issue 0209). `run`
    // reports a hold as one refusal among a run's findings, which suits the
    // whole-estate verb; this verb names one artefact, so nothing it could
    // write means the call failed. The pin above stands, and the next
    // `organize` names the same refusal until the bucket copy is moved.
    if let Some(h) = whole.held.iter().find(|h| h.thread == id) {
      return Err(FacadeError::Organize(
        organize::OrganizeError::LegacyCopyPresent {
          thread: h.thread.clone(),
          dir: h.dir.clone(),
          home: h.home.clone(),
          files: h.files,
        },
      ));
    }
    let mine: Vec<_> = whole
      .steps
      .iter()
      .filter(|s| s.path.starts_with(&home))
      .cloned()
      .collect();
    let scoped = organize::Plan {
      // **A SCOPED PLAN PRUNES NO v2 LEFTOVER.** This narrows an estate-wide
      // plan to one artefact's paths; the v2 prune is an estate-wide act with
      // an estate-wide refusal, so it belongs to the whole-tree run and never
      // rides along with a single thread's realisation.
      leftovers: crate::legacy::Leftovers::default(),
      steps: mine,
      digest: whole.digest.clone(),
      preconditions: whole.preconditions.clone(),
      estate_root: whole.estate_root.clone(),
      held: Vec::new(),
    };
    // **THE BYTES THAT WOULD GO ARE READ BEFORE ANY ARE WRITTEN.** A `Verify`
    // step is a view this artefact owns that is already on disk; the plan
    // carries what the store renders for it, so the comparison is in hand here
    // and costs one read per view. **The refusal names every path**, because a
    // verb that refuses without saying which file sends the operator to diff a
    // whole thread against a description of it.
    // **A REALISATION VERB DOES NOT REMOVE.** The scoped plan is the estate's
    // plan narrowed to this artefact's directory, so it can carry `Dehydrate`
    // steps -- and `Plan::run` performs them. Checked before the divergence
    // below because it is the coarser fault: a run that would remove files is
    // refused whole, not file by file.
    {
      let removing: Vec<std::path::PathBuf> = scoped
        .with(organize::Action::Dehydrate)
        .map(|step| self.project.relative(&step.path))
        .map(std::path::PathBuf::from)
        .collect();
      if !removing.is_empty() {
        return Err(FacadeError::RealisationWouldRemove {
          id: id.clone(),
          paths: removing,
        });
      }
    }

    {
      let diverged: Vec<std::path::PathBuf> = scoped
        .with(organize::Action::Verify)
        .filter(|step| {
          step.content.as_ref().is_some_and(|rendered| {
            // **UNREADABLE IS NOT DIVERGED.** A view this process cannot read
            // is not evidence that somebody's work is under it, and refusing
            // there would block a realisation over a permissions problem the
            // operator would then have to diagnose from the wrong message.
            std::fs::read(&step.path).is_ok_and(|disk| disk != *rendered)
          })
        })
        .map(|step| self.project.relative(&step.path))
        .map(std::path::PathBuf::from)
        .collect();
      if !diverged.is_empty() {
        if !overwrite {
          return Err(FacadeError::HydrationWouldOverwrite {
            id: id.clone(),
            paths: diverged,
          });
        }
        // **`--overwrite` STILL NAMES WHAT IT DISCARDS, AND NAMES IT FIRST.**
        // The flag is the operator saying they know; it is not permission to
        // stop reporting. A run that destroys work and prints `wrote:` after
        // the fact is the defect this batch exists for, flag or no flag.
        announce(&diverged);
      }
    }

    // **`hydrate` IS ALWAYS `Mode::Apply`, AND IT NEEDS NO FLAG TO BE.** The
    // preview/apply split exists because `organize` REMOVES; `hydrate` only
    // ever writes, and a caller naming an address has already said what they
    // want to happen to it. Making the safe verb ask twice would teach the
    // reflex that makes the dangerous one's question invisible.
    let run = scoped
      .run(organize::Mode::Apply, &|| {
        organize::observe(&self.project, &previous)
          .map(|(_, digest)| digest)
          .unwrap_or_else(|_| "tree-could-not-be-re-read".to_string())
      })
      .map_err(FacadeError::Organize)?;

    // **PATHS THAT NOW EXIST, NOT PATHS THIS RUN HAD A STEP FOR, AND THE
    // DIFFERENCE IS A DEFECT MY OWN TEST CAUGHT.** Returning the plan's steps
    // looks right and is not: `plan` deliberately emits NO step for an
    // attachment already agreeing with the store, so the first call returned six
    // paths and the second returned four -- the same artefact, the same tree,
    // two different answers. A caller asking "does my file exist now" would have
    // been told no on the run where nothing needed doing.
    //
    // **So the set is asked of the two owners rather than reconstructed** --
    // see [`Facade::carried`].
    let mut owned = self.carried(&id)?;
    owned.retain(|p| p.exists());
    // **WHAT THIS RUN CHANGED, NOT WHAT NOW EXISTS -- and the two differ on the
    // ordinary path, which is what makes it worth guarding.** `owned` is
    // deliberately *paths that now exist* so a caller can ask "is my file
    // there"; gating the event on it would record an act every time anyone
    // hydrated an already-realised artefact, and a log of non-acts is one a
    // reader stops trusting. **The two steps are independently idempotent**, so
    // either can be the real change: the pin can move over an already-present
    // tree, and the tree can be written under an already-pinned id.
    let wrote = !(run.hydrated.is_empty() && run.rewritten.is_empty());
    if pin_moved || wrote {
      self.record_disk_act(
        "disk.hydrate",
        serde_json::json!({
          // The artefact is named by sigil and id rather than by a rendered
          // address: `?format=` names a REPRESENTATION and `hydrate` dispatches
          // on entity alone, so carrying the format would record a distinction
          // the act does not make.
          "sigil": sigil.as_str(),
          "id": id,
          "pinned": pin_moved,
          "hydrated": run
            .hydrated
            .iter()
            .map(|p| self.project.relative(p))
            .collect::<Vec<_>>(),
          "rewritten": run
            .rewritten
            .iter()
            .map(|p| self.project.relative(p))
            .collect::<Vec<_>>(),
        }),
      )?;
    }
    let wrote: Vec<std::path::PathBuf> = owned
      .iter()
      .filter(|p| run.hydrated.contains(p) || run.rewritten.contains(p))
      .cloned()
      .collect();
    // **A FILE THE RUN COULD NOT WRITE IS A REFUSAL, NOT A QUIET GAP** (issue
    // 0338 (i)). The run refuses by path a step it holds no bytes for -- an
    // opaque attachment whose sidecar was never loaded -- and this door dropped
    // the run's refusals, so `st hydrate` reported a thread realised with a file
    // missing. What the run did write is recorded above first: a refusal must
    // not erase the record of an act that completed.
    if let Some(refusal) = run.refused.into_iter().next() {
      return Err(FacadeError::Organize(refusal));
    }
    Ok(Hydration {
      paths: owned,
      wrote,
    })
  }

  /// Remove one artefact's realised files and unlist it from the manifest --
  /// the inverse of [`Facade::hydrate`], and **deliberately not its mirror
  /// image.**
  ///
  /// # The asymmetry IS the design
  ///
  /// `hydrate` pins FIRST and materialises second, because its second step only
  /// ever writes: a pin over a tree that fails to materialise leaves a declared
  /// thread with missing files, which the next `organize` fixes by writing them.
  ///
  /// **Dehydrating in that order would convert a REFUSAL into a DEFERRED
  /// DELETION.** Unlist first, refuse the removal, and the thread is left
  /// undeclared and present -- so the next `organize --apply` run by anyone
  /// removes exactly the files this run refused to remove, with nobody having
  /// decided anything and no refusal in sight. So the plan here is computed
  /// against a HYPOTHETICAL manifest: [`intentfiles::unpin`] returns text,
  /// [`intentfiles::realised_for_action`] reads it, and **nothing on disk moves
  /// until the removal has been permitted and performed** (AC-00.3).
  ///
  /// # All or nothing, for one thread
  ///
  /// Every destructive step is gated BEFORE any of them runs, and one refusal
  /// refuses the whole artefact. `organize` may sensibly remove what it can and
  /// report the rest, because it is reconciling an estate; this verb was handed
  /// ONE id, and half-removing that id's files while leaving it declared is a
  /// state no operator asked for and none would expect to have to repair.
  /// Write `.intentfiles` from status -- **AC-11.1, AC-11.2, AC-11.4.**
  ///
  /// **THE CONTENT COMES FROM `intentfiles::default_declaration` AND NOWHERE
  /// ELSE (AC-11.3).** `init`, `upgrade` and this verb each decide their own
  /// present/absent POLICY -- they genuinely differ: init writes into a
  /// directory where the file cannot already exist, upgrade writes only when it
  /// is absent, and this one writes when absent or when a human has confirmed.
  /// What they must not each decide is what "the open set" MEANS. Three callers
  /// deriving that separately is three chances to disagree, and the one that
  /// drifts is the one nobody runs.
  ///
  /// **`force` MEANS "A HUMAN HAS ALREADY SAID YES", NOT "SKIP THE ASKING".**
  /// The tty and the confirmation belong to the renderer: whether a person is
  /// present is a property of the invocation, not of the estate, and a facade
  /// that reached for `/dev/tty` would make every caller -- tests included --
  /// depend on a terminal. So this takes the ANSWER and never asks the
  /// question.
  ///
  /// **IT NEVER REMOVES ANYTHING, IN EITHER ARM (AC-11.4).** The only write is
  /// the manifest itself. Realising and dehydrating against the new declaration
  /// is `organize`'s, and keeping them apart is what lets `--default` be safe
  /// to run on any estate at any time.
  pub fn declare_default(&mut self, force: bool) -> Result<Declared, FacadeError> {
    let path = self.project.intentfiles_path();

    let existing = match std::fs::read_to_string(&path) {
      Ok(text) => Some(text),
      Err(source) if source.kind() == std::io::ErrorKind::NotFound => None,
      // A file that is there and cannot be READ is not a file that is absent,
      // and letting a permissions error answer as "nobody has said" would write
      // an estate-wide declaration over something unreadable.
      Err(source) => {
        return Err(FacadeError::ManifestUnreadable {
          path: path.display().to_string(),
          source,
        });
      }
    };

    if let Some(before) = &existing {
      // **A MALFORMED MANIFEST REFUSES RATHER THAN COUNTING ZERO.** The count
      // below is what the operator is told the file declares, and a parse
      // failure reported as `0 declared` is the silent zero this estate keeps
      // paying for -- it reads exactly like an empty declaration, which is the
      // one state that means "remove everything".
      let declares = intentfiles::parse(before)
        .map_err(FacadeError::Intentfiles)?
        .entries
        .len();
      if !force {
        return Ok(Declared {
          path,
          wrote: false,
          was_present: true,
          declares,
        });
      }
    }

    let threads: Vec<(String, crate::model::ThreadStatus)> = self
      .st_list()
      .iter()
      .map(|t| (t.id.clone(), t.status))
      .collect();
    let issues: Vec<(u32, crate::model::IssueStatus)> = self
      .issue_list()
      .iter()
      .map(|i| (i.number, i.status))
      .collect();
    let text = intentfiles::default_declaration(&threads, &issues);
    let declares = intentfiles::parse(&text)
      .map_err(FacadeError::Intentfiles)?
      .entries
      .len();

    let mut set = WriteSet::new();
    set.add(path.clone(), text);
    set.commit()?.keep();

    self.record_disk_act(
      "disk.declare_default",
      serde_json::json!({
        "path": self.project.relative(&path),
        "declares": declares,
        "replaced": existing.is_some(),
      }),
    )?;

    Ok(Declared {
      path,
      wrote: true,
      was_present: existing.is_some(),
      declares,
    })
  }

  /// Delist a thread and remove its realised files.
  ///
  /// **THE ANNOUNCING FORM IS THE ONE EVERY INTERACTIVE FACE OWES ITS
  /// OPERATOR** (hv, 2026-09-12: silent deletion). This one announces to
  /// nobody, which is right for a caller that shows no output at all.
  pub fn dehydrate(&mut self, address: &Address) -> Result<Dehydrated, FacadeError> {
    self.dehydrate_announcing(address, &mut |_, _| {})
  }

  /// The same removal, with every path handed to `announce` BEFORE the first
  /// byte goes.
  ///
  /// `st dehydrate` listed what it had removed AFTER removing it, so the first
  /// time a path reached the screen it was already gone. The plan is in hand
  /// here -- the scoped `Plan` is built before it is run -- so the caller is
  /// handed the files and the directories the run will take, and prints them in
  /// the future tense.
  ///
  /// **THE FACADE DOES NOT PRINT, WHICH IS WHY THIS IS A CALLBACK**
  /// (IN-AG-THIN-COORD-001). What to say and where to say it is the renderer's;
  /// what is about to go is this layer's, and only this layer knows it.
  pub fn dehydrate_announcing(
    &mut self,
    address: &Address,
    announce: &mut dyn FnMut(&[std::path::PathBuf], &[std::path::PathBuf]),
  ) -> Result<Dehydrated, FacadeError> {
    require_local(address)?;
    let Some((sigil, id)) = address.entity.artefact() else {
      return Err(FacadeError::NotHydratable {
        form: address.entity.form(),
        why: "only an artefact -- a steel thread or an issue -- is named by `.intentfiles`, so it is the smallest thing realisation can address".to_string(),
      });
    };
    let id = id.to_string();
    let path = self.project.intentfiles_path();

    // **AN ABSENT MANIFEST REFUSES (AC-00.4).** See
    // [`FacadeError::NoManifestToUnlistFrom`] for why creating one here is the
    // destructive answer rather than the convenient one.
    let before = match std::fs::read_to_string(&path) {
      Ok(text) => text,
      Err(source) if source.kind() == std::io::ErrorKind::NotFound => {
        return Err(FacadeError::NoManifestToUnlistFrom {
          id,
          path: self.project.relative(&path),
        });
      }
      Err(source) => {
        return Err(FacadeError::ManifestUnreadable {
          path: path.display().to_string(),
          source,
        });
      }
    };

    // THE HYPOTHETICAL. Text, not a write. `unpin` refuses a malformed id and
    // returns the original unchanged when the id is simply not listed -- which
    // is an ordinary state here, not an error: a thread can be present on disk
    // and absent from the list, and finishing that job is exactly what this verb
    // is for.
    let after = intentfiles::unpin(&before, sigil, &id).map_err(FacadeError::Intentfiles)?;
    let was_listed = after != before;
    let realised_after =
      intentfiles::realised_for_action(&after).map_err(FacadeError::Intentfiles)?;

    let previous = self.store.file_index().map_err(FacadeError::Store)?;
    let (tree, digest) =
      organize::observe(&self.project, &previous).map_err(FacadeError::Organize)?;
    let whole = {
      let ctx = self.render_ctx()?;
      organize::plan(
        &self.project,
        &self.canon,
        &realised_after,
        &ctx,
        &tree,
        digest,
      )
    };

    // Whole-estate plan, narrow act -- the same split `hydrate` documents:
    // classification needs the estate as its denominator, and this verb was
    // handed one id.
    let home = match sigil {
      intentfiles::Sigil::SteelThread => self.project.thread_dir(&id),
      intentfiles::Sigil::Issue => issue_home(&self.project, &id)?,
    };
    let mine: Vec<_> = whole
      .steps
      .iter()
      .filter(|s| s.path.starts_with(&home))
      .cloned()
      .collect();

    // **THE RAIL, AND IT IS READ BEFORE ANYTHING IS REMOVED (AC-00.2).**
    // `organize::gate` is the one answer to "can the store put this file back",
    // and its refusing arm is a wildcard: an opaque attachment carrying `None`
    // and a hand-edited file whose bytes differ both land there, by the same
    // match arm rather than by two cases that could drift apart.
    // **EVERY STEP UNDER THE THREAD, NOT ONLY THE DESTRUCTIVE ONES -- and the
    // widening is vc's, from a measurement on Lamplight ST0306.** Ten untracked
    // gifs, 152.9 MB, under one thread's `WP/11/images/`: in no git object, on
    // one machine. They are not renderable and the store does not carry them,
    // so `plan` files them `Unclaimed` -- *report, never remove*.
    //
    // **NOTHING HERE WOULD EVER HAVE DELETED THEM**, and it is worth saying why
    // rather than implying the fix was a near miss: `run` removes only
    // `Dehydrate` steps, and `prune_emptied` calls `remove_dir`, which fails on
    // a non-empty directory. That is a PHYSICAL floor, not a rule to be kept.
    //
    // What the narrow gate got wrong was the REPORT. It would remove the
    // generated views, delist the thread, and answer `dehydrated` with 152.9 MB
    // still on disk and the declaration already gone -- **the manifest saying
    // one thing and the disk another, which is the exact divergence this verb
    // is ordered to prevent.** So a step this run cannot remove refuses the run,
    // naming the file and what made it unremovable.
    let refusals: Vec<String> = mine
      .iter()
      .filter_map(|s| match s.action {
        // The one removable action; the gate alone decides whether the store
        // can put these bytes back.
        organize::Action::Dehydrate => organize::gate(s).err().map(|e| e.to_string()),
        // Kept by design -- and keeping it means the thread does not leave the
        // tree, which is what this verb was asked for.
        other => Some(format!(
          "{} is `{other:?}` -- this run cannot remove it, so the thread would keep a realised form after being delisted",
          self.project.relative(&s.path)
        )),
      })
      .collect();
    if !refusals.is_empty() {
      return Err(FacadeError::DehydrationRefused {
        id: id.clone(),
        count: refusals.len(),
        detail: refusals.join("\n  "),
      });
    }

    let scoped = organize::Plan {
      // **A SCOPED PLAN PRUNES NO v2 LEFTOVER.** This narrows an estate-wide
      // plan to one artefact's paths; the v2 prune is an estate-wide act with
      // an estate-wide refusal, so it belongs to the whole-tree run and never
      // rides along with a single thread's realisation.
      leftovers: crate::legacy::Leftovers::default(),
      steps: mine,
      digest: whole.digest.clone(),
      preconditions: whole.preconditions.clone(),
      estate_root: whole.estate_root.clone(),
      held: Vec::new(),
    };
    // **ANNOUNCED HERE, WHICH IS THE LAST MOMENT BEFORE ANYTHING IS
    // IRREVERSIBLE.** Every refusal above has already been taken, so what is
    // handed over is what the run will actually attempt -- announcing earlier
    // would name files a refusal then spares, which is its own false report.
    let going: Vec<std::path::PathBuf> = scoped
      .with(organize::Action::Dehydrate)
      .map(|s| s.path.clone())
      .collect();
    let prunes = organize::prunes_for(&scoped.estate_root, &going);
    announce(&going, &prunes);

    let run = scoped
      .run(organize::Mode::Apply, &|| {
        organize::observe(&self.project, &previous)
          .map(|(_, digest)| digest)
          .unwrap_or_else(|_| "tree-could-not-be-re-read".to_string())
      })
      .map_err(FacadeError::Organize)?;

    // **`run` RETURNS `Ok` WITH ITS REFUSALS INSIDE THE REPORT, AND READING
    // ONLY THE `Err` ARM IS A SILENT SWALLOW.** This comment used to say the
    // estate preconditions were "read inside `run`, which refuses the whole run
    // before touching anything" -- **which I asserted without driving, and it
    // was false in the direction that matters.** `Plan::run` pushes
    // `PreconditionsUnmet` onto `report.refused` and returns `Ok`; the `?`
    // above sees nothing. So on an estate that had declared no preconditions at
    // all, `st dehydrate` removed no file, delisted the thread anyway, and
    // reported success -- **the manifest saying dehydrated over a full
    // directory tree, which is the one divergence this verb exists to
    // prevent.** Found by `an_estate_with_no_declaration_refuses_this_door_too`
    // on its first run, not by review.
    //
    // A per-thread verb must not be a door around the estate gate: if it were,
    // the gate would protect only the operator who reached for `organize`, and
    // the NARROWER verb would be the one that deletes.
    if let Some(refusal) = run.refused.into_iter().next() {
      return Err(FacadeError::Organize(refusal));
    }

    // **ONLY NOW.** Every removal has been permitted and performed, so the
    // declaration can follow. Dying between these two leaves files removed and
    // the thread still declared -- which the next `organize` repairs by writing
    // them back from the store. The other order has no such recovery.
    if was_listed {
      let mut set = WriteSet::new();
      set.add(path, after);
      set.commit()?.keep();
    }

    let removed: Vec<std::path::PathBuf> = run.dehydrated.clone();
    if was_listed || !removed.is_empty() {
      self.record_disk_act(
        "disk.dehydrate",
        serde_json::json!({
          "sigil": sigil.as_str(),
          "id": id,
          "unlisted": was_listed,
          "removed": removed
            .iter()
            .map(|p| self.project.relative(p))
            .collect::<Vec<_>>(),
          "pruned": run
            .pruned
            .iter()
            .map(|p| self.project.relative(p))
            .collect::<Vec<_>>(),
        }),
      )?;
    }

    // **WALKED FROM THE THREAD DIRECTORY, NOT DERIVED FROM WHAT WAS REMOVED --
    // and the first spelling of this was wrong in a way its own motivating case
    // would have exposed.** Deriving the candidates from the ancestors of
    // removed files is what `prune_emptied` does, and it is right for pruning:
    // a directory nothing was removed from was never going to become empty. It
    // is wrong for REPORTING, because the directory that survives is typically
    // a SIBLING of everything removed -- `WP/11/images` holding review aids,
    // while every removal happened in the thread root. Nothing there is an
    // ancestor of anything removed, so the report would have been silent about
    // precisely the directory it exists to name.
    //
    // Only the LEAVES are named: a directory with no surviving subdirectory.
    // The chain above it is implied by its own path, and printing four nested
    // lines for one leftover would be a count of containers standing in for a
    // count of contents.
    let mut left_in_place: Vec<std::path::PathBuf> = Vec::new();
    surviving_leaves(&home, &mut left_in_place);
    left_in_place.sort();

    Ok(Dehydrated {
      removed,
      pruned: run.pruned.clone(),
      unlisted: was_listed,
      left_in_place,
    })
  }

  /// Register the node roster from each node's own `wip.md` header.
  ///
  /// **THE HEADER IS THE SOURCE, AND THE README ROSTER TABLE IS NOT.** Three
  /// reasons, and the first is decisive on its own: the table has no `role`
  /// column -- its third column is a charter sentence -- so two of the three
  /// fields would have to come from somewhere else anyway. The headers are
  /// single-writer, one node each, and the README is not: that file says of
  /// itself that it has no single writer and goes stale, having described one
  /// node's lane wrongly through an entire reorganisation with nobody owning
  /// the correction. And its own proposed fix is this one, recorded there
  /// before this existed -- generate the roster from each node's header so it
  /// cannot disagree with the boards it describes.
  ///
  /// **THIS REGISTERS CONFIGURATION; IT DOES NOT MIGRATE A BOARD.** The rows it
  /// writes carry no items and no messages, and the markdown beside them stays
  /// hand-authored and authoritative. Carrying the boards' CONTENT across is a
  /// separate deliberate act at a cutover, and reading a thin board file as a
  /// half-finished migration would get both of them wrong.
  pub fn register_roster(&mut self) -> Result<usize, FacadeError> {
    let mut nodes = Vec::new();
    let Ok(entries) = std::fs::read_dir(self.project.whiteboard_dir()) else {
      return Ok(0);
    };
    let mut dirs: Vec<std::path::PathBuf> = entries
      .filter_map(Result::ok)
      .map(|e| e.path())
      .filter(|p| p.join("wip.md").is_file())
      .collect();
    dirs.sort();
    let mut incomplete = Vec::new();
    for dir in dirs {
      // **AN UNREADABLE OR INCOMPLETE HEADER REFUSES RATHER THAN SKIPPING THE
      // NODE.** A silently skipped node is a roster that is quietly short by
      // one, which is indistinguishable from a node nobody has created yet.
      // Until issue 0424 this held for an unreadable header only: a header
      // lacking a field fell through and was skipped at exit 0.
      let text = std::fs::read_to_string(dir.join("wip.md")).map_err(|e| {
        FacadeError::Ingest(IngestError::Io {
          path: dir.join("wip.md").display().to_string(),
          source: e,
        })
      })?;
      let field = |key: &str| -> Option<String> {
        // **The header block is line-oriented `key: value`, not YAML**, and it
        // is read the way it is written: the value is everything after the
        // first `: ` to the end of the line, with one pair of surrounding
        // double quotes stripped for display and quotes inside left alone.
        text
          .lines()
          .take_while(|l| l.trim() != "---" || l.trim().is_empty())
          .chain(text.lines())
          .find_map(|line| line.strip_prefix(&format!("{key}: ")))
          .map(|v| {
            let v = v.trim();
            v.strip_prefix('"')
              .and_then(|r| r.strip_suffix('"'))
              .unwrap_or(v)
              .to_string()
          })
      };
      match (field("node"), field("name"), field("role")) {
        (Some(node), Some(name), Some(role)) => nodes.push((node, name, role)),
        (node, name, role) => {
          let lacks: Vec<&str> = [("node", node), ("name", name), ("role", role)]
            .into_iter()
            .filter(|(_, v)| v.is_none())
            .map(|(k, _)| k)
            .collect();
          let path = dir.join("wip.md");
          let shown = path.strip_prefix(self.project.root()).unwrap_or(&path);
          incomplete.push(format!("{} (lacks {})", shown.display(), lacks.join(", ")));
        }
      }
    }
    // **EVERY HEADER IS READ BEFORE THE FIRST WRITE**, so a refusal names every
    // incomplete board at once and leaves no partial roster behind.
    if !incomplete.is_empty() {
      return Err(FacadeError::WbHeaderIncomplete {
        listing: incomplete.join(", "),
      });
    }
    let event = Envelope::minted(
      &self.ctx.principal,
      &self.ctx.project_id,
      "wb.register",
      Subject {
        kind: "roster".to_string(),
        id: self.ctx.project_id.clone(),
      },
      json!({ "nodes": nodes, "from": "headers" }),
    );
    let registered = self
      .store
      .wb_write(&event, |w| w.register_nodes(&nodes, false))
      .map_err(FacadeError::Store)?;
    // Index only: the headers this read stay hand-authored until a migration.
    self.reindex_boards()?;
    self.land_event_files()?;
    Ok(registered)
  }

  /// Every registered node's board, in roster order.
  ///
  /// **THE READ IS UNSCOPED BY DESIGN, and AC-14.7 is why**: any workstream can
  /// read any node's board. The single-writer invariant the protocol turns on
  /// is about WRITES -- it never made a board private, and the markdown form it
  /// replaces was world-readable in the checkout.
  /// Register one node from its arguments, and say how many rows that wrote.
  ///
  /// **HOW A NODE JOINS ONCE NO HEADER IS HAND-WRITTEN** (vc, 2026-09-13, on
  /// ic's finding). [`Self::register_roster`] reads each node's `wip.md` header,
  /// and after the cutover those headers are rendered FROM the rows -- so
  /// reading them can create nothing, and a new participant would have no door.
  ///
  /// **THE SAME VALUES TWICE WRITE NOTHING; DIFFERENT VALUES ARE REFUSED.** A
  /// silent overwrite would rename a node under every peer reading its board,
  /// and a silent no-op would tell a caller who asked for new values that they
  /// hold. A deliberate change is [`Self::wb_correct`], which the refusal names.
  pub fn wb_register(
    &mut self,
    moniker: &str,
    name: &str,
    role: &str,
  ) -> Result<usize, FacadeError> {
    if let Some(held) = self
      .boards()?
      .into_iter()
      .find(|b| b.node.moniker == moniker)
    {
      if held.node.name == name && held.node.role == role {
        return Ok(0);
      }
      return Err(FacadeError::WbRegisteredDifferently {
        node: moniker.to_string(),
        name: name.to_string(),
        role: role.to_string(),
        held_name: held.node.name,
        held_role: held.node.role,
      });
    }
    // **A NODE WITH A HAND-AUTHORED BOARD IS NEITHER BORN MIGRATED NOR
    // RENDERED.** Its row is not the board while markdown stands beside it:
    // `wb migrate` carries that, and until then every board write refuses
    // rather than rendering the empty row over it. A node with no board IS its
    // row, and lands its view now, because doctor reads a migrated node's
    // absent view as a missing one.
    // Issue 0379: this rendered every registration, so a register before the migrate erased the board the migrate reads.
    let board_file = self.project.whiteboard_dir().join(moniker).join("wip.md");
    let hand_authored = board_file.is_file();
    // **THE HEADER THE VERB FOUND IS READ AND COMPARED** (issue 0410). The
    // board was evidence of who this node is, and the verb used it only as a
    // render flag, so arguments contradicting it were written without a word.
    // It is read the way `wb migrate` reads it, and a field the header does
    // not carry is no evidence either way.
    if hand_authored {
      let file = self.project.relative(&board_file);
      let header =
        crate::wbmigrate::read_board(moniker, &Self::read_board_file(&board_file)?, &file);
      if (!header.name.is_empty() && header.name != name)
        || (!header.role.is_empty() && header.role != role)
      {
        return Err(FacadeError::WbRegisterDisagreesWithHeader {
          node: moniker.to_string(),
          name: name.to_string(),
          role: role.to_string(),
          header_name: header.name,
          header_role: header.role,
          file,
        });
      }
    }
    let event = self.wb_event(
      "wb.register",
      moniker,
      json!({ "name": name, "role": role }),
    );
    let written = self
      .store
      .wb_write(&event, |w| {
        w.register_nodes(
          &[(moniker.to_string(), name.to_string(), role.to_string())],
          !hand_authored,
        )
      })
      .map_err(FacadeError::Store)?;
    if hand_authored {
      self.reindex_boards()?;
      self.land_event_files()?;
    } else {
      self.land_board_write_noting()?;
    }
    Ok(written)
  }

  /// Correct a registered node's name and role, and say whether they moved
  /// (issue 0417, vc decision 21 (1)).
  ///
  /// **A FLAG ON THE ONE IDENTITY DOOR, NOT A SECOND WAY IN.** It refuses a
  /// moniker that is not registered rather than creating it, so `register`
  /// stays the only thing that puts a node on the board; it writes name and
  /// role and nothing else, so the board, its items and its messages stay
  /// attached. Until it, a node registered wrong could be repaired only by a
  /// hand `DELETE` on `wb_node`.
  ///
  /// **THE VALUES ALREADY HELD MOVE NOTHING AND RECORD NOTHING**, and the
  /// caller reports the node unchanged.
  pub fn wb_correct(&mut self, moniker: &str, name: &str, role: &str) -> Result<bool, FacadeError> {
    let Some(held) = self
      .boards()?
      .into_iter()
      .find(|b| b.node.moniker == moniker)
    else {
      return Err(FacadeError::WbCorrectUnregistered {
        node: moniker.to_string(),
      });
    };
    if held.node.name == name && held.node.role == role {
      return Ok(false);
    }
    let event = self.wb_event(
      "wb.correct",
      moniker,
      json!({
        "name": name,
        "role": role,
        "held_name": held.node.name,
        "held_role": held.node.role,
      }),
    );
    self
      .store
      .wb_write(&event, |w| w.set_identity(moniker, name, role))
      .map_err(FacadeError::Store)?;
    // A node whose board is still its markdown is indexed and not rendered,
    // for the reason `wb_register` gives.
    if self
      .store
      .wb_node_migrated(moniker)
      .map_err(FacadeError::Store)?
    {
      self.land_board_write_noting()?;
    } else {
      self.reindex_boards()?;
      self.land_event_files()?;
    }
    Ok(true)
  }

  /// The event one whiteboard verb records, naming the acting node and the
  /// verb's arguments (issue 0411).
  ///
  /// **THE ACTING NODE IS THE SUBJECT**, so a node's history is one filter on
  /// the log. For `ask` and `announce` that is the sender: sending is the
  /// sender's act, and the recipient is an argument.
  fn wb_event(&self, op: &str, node: &str, payload: serde_json::Value) -> Envelope {
    Envelope::minted(
      &self.ctx.principal,
      &self.ctx.project_id,
      op,
      Subject {
        kind: "node".to_string(),
        id: node.to_string(),
      },
      payload,
    )
  }

  pub fn boards(&self) -> Result<Vec<Board>, FacadeError> {
    self.store.hydrate_boards().map_err(FacadeError::Store)
  }

  /// Bring the model's boards and the prose index up to what a board write
  /// just stored. Every whiteboard write ends here.
  ///
  /// **THE SAME DERIVATION AND THE SAME WRITER A THREAD MUTATION USES**:
  /// `sections_of` over the whole model and `replace_doc_sections`, never a
  /// board-only patch of the index. A mutation clones `self.canon` and indexes
  /// what the clone carries, so the boards are refreshed there too, or the next
  /// thread mutation in a long-lived facade would put a stale board back.
  ///
  /// **NOT IN THE BOARD WRITE'S OWN TRANSACTION**, which a thread mutation's
  /// index is. A failure between the two leaves the row written and the index
  /// one write behind, reported as this call's error rather than swallowed, and
  /// the next board write or rebuild derives the whole index again.
  fn reindex_boards(&mut self) -> Result<(), FacadeError> {
    self.canon.boards = self.boards()?;
    let sections = ingest::sections_of(
      &self.project,
      &self.canon.threads,
      &self.canon.issues,
      &self.canon.boards,
    );
    self
      .store
      .replace_doc_sections(&sections)
      .map_err(FacadeError::Store)?;
    self.canon.sections = sections;
    Ok(())
  }

  /// The notes steps raised after a write landed, taken so each prints once.
  ///
  /// **FOR THE VERBS THAT RETURN NO [`Outcome`]** (0376): the board verbs,
  /// `organize`, `issues add`, `wp new` and `todo flush`. A verb that returns
  /// an `Outcome` carries its notes in it, through [`Outcome::with_overwrites`].
  pub fn take_notes(&mut self) -> Vec<Note> {
    std::mem::take(&mut self.after_write)
  }

  /// Keep a landed write's note for [`Self::take_notes`], for a verb with no
  /// [`Outcome`] to carry it.
  fn park(&mut self, applied: Applied) {
    self.after_write.extend(applied.after_write);
  }

  /// Land a board write's views, keeping a failure as a note rather than a
  /// refusal, save a torn rollback, which stays one.
  ///
  /// **EVERY CALLER HAS COMMITTED ITS ROW BEFORE THIS RUNS** (vc, ruled
  /// 2026-09-14). Returning the failure as the verb's own sent the caller to
  /// retry a write the store already held, and `wb add` doubled a row that way.
  fn land_board_write_noting(&mut self) -> Result<(), FacadeError> {
    // Issue 0376: this failure was returned as the write's own.
    match self.land_board_write() {
      Ok(()) => Ok(()),
      Err(
        torn @ FacadeError::ViewsNotWritten {
          cause: WriteError::TornRollback { .. },
        },
      ) => Err(torn),
      Err(cause) => {
        // Issue 0487: this carried RERENDER_REMEDY, which names `intent st
        // sync` -- a door that rewrites a thread's views and cannot land a
        // board's. The board's own remedy says why, and names the two commands
        // a reader would otherwise reach for.
        self.after_write.push(Note::after_write(
          "landing the board's views",
          &cause,
          BOARD_RERENDER_REMEDY,
        ));
        Ok(())
      }
    }
  }

  /// Refresh the index for a board write, then land the write's views on disk.
  ///
  /// **THE WRITE LANDS THROUGH THE PROJECTION A MUTATION USES** (0317). From the
  /// cutover on a board is a generated view, so a board write that moved the
  /// row and the index and not the files left every view stale at the first
  /// verb, and the one repair was a sync the running daemon refuses. The
  /// projection is narrowed to the whiteboard, because a board write changes no
  /// thread; `WriteSet::commit` skips a board whose bytes already match, so only
  /// the boards this write changed are written, and what landed is recorded so
  /// the daemon does not read it back as an edit.
  ///
  /// **A HAND EDIT OF A GENERATED BOARD IS OVERWRITTEN, AS A MUTATION OVERWRITES
  /// ONE.** Skipping a view whose bytes differ from the prior render wedges: a
  /// facade whose boards are a write behind the store reads its own last render
  /// as a hand edit and never writes that board again.
  ///
  /// **A REGISTRATION BESIDE HAND-AUTHORED MARKDOWN DOES NOT CALL THIS, AND
  /// THE MIGRATION CALLS IT ONLY AFTER ITS CARRY.** [`Self::register_roster`]
  /// reads every node's header, and [`Self::wb_register`] on a node that has a
  /// `wip.md` stands beside a board nobody has carried; a render from either
  /// would erase the board the migration exists to carry, so they refresh the
  /// index only. [`Self::wb_migrate`] reads every file first and lands its
  /// views once the carry is written, which is when the tree and the store can
  /// agree.
  fn land_board_write(&mut self) -> Result<(), FacadeError> {
    self.reindex_boards()?;
    let Projection { set, canon_files } = self.projection(&self.canon, &[], &[], None, None)?;
    let whiteboard = self.project.whiteboard_dir();
    let mut boards = WriteSet::new();
    for (path, content) in set
      .writes()
      .filter(|(path, _)| path.starts_with(&whiteboard))
    {
      boards.add_bytes(path.to_path_buf(), content.to_vec());
    }
    // The board write's event travels with the boards it changed (ST0078 P1).
    self.add_event_files(&mut boards)?;
    let applied = boards
      .commit()
      .map_err(|cause| FacadeError::ViewsNotWritten { cause })?;
    let landed: Vec<std::path::PathBuf> = applied.written().map(std::path::PathBuf::from).collect();
    applied.keep();
    let canon_files: Vec<(std::path::PathBuf, String)> = canon_files
      .into_iter()
      .filter(|(path, _)| path.starts_with(&whiteboard))
      .collect();
    self.record_landed(&canon_files, &landed)
  }

  /// Refuse an entry body the acting node's bound does not admit.
  ///
  /// **ONE HOME FOR THE CHECK, and every write path asks it.** Two spellings of
  /// the same bound is how one door ends up enforcing a rule the other does
  /// not, and the silent direction is the one that accepts.
  fn check_body_bound(&self, node: &str, body: &str) -> Result<(), FacadeError> {
    let cfg = &self.project.config().whiteboard;
    if !cfg.bounds_apply_to(node) {
      return Ok(());
    }
    // Compared with `>`, so a body exactly at the bound is accepted -- the same
    // reading `IndexConfig::max_file_bytes` takes of its own cap.
    if body.len() > cfg.body_bytes {
      return Err(FacadeError::WbBodyOverBound {
        node: node.to_string(),
        bytes: body.len(),
        bound: cfg.body_bytes,
      });
    }
    Ok(())
  }

  /// Refuse a moniker the roster does not carry, with the roster named.
  fn require_registered(&self, node: &str) -> Result<(), FacadeError> {
    if self
      .store
      .wb_node_exists(node)
      .map_err(FacadeError::Store)?
    {
      return Ok(());
    }
    let known = self.store.wb_monikers().map_err(FacadeError::Store)?;
    Err(FacadeError::WbNodeNotRegistered {
      node: node.to_string(),
      known: if known.is_empty() {
        "no nodes at all".to_string()
      } else {
        known.join(", ")
      },
    })
  }

  /// Refuse a board write on a node whose board is still hand-authored.
  ///
  /// **A BOARD WRITE LANDS A RENDER, SO ON AN UNMIGRATED NODE IT WOULD ERASE THE
  /// BOARD** (vc, 2026-09-13, on 0317). A node registered from its header holds a
  /// row and none of its markdown, and rendering that row over `wip.md` replaces
  /// the board a migration exists to carry with an empty one. Registration and
  /// the migration are the two board writes that do not ask this.
  fn require_migrated(&self, node: &str) -> Result<(), FacadeError> {
    self.require_registered(node)?;
    if self
      .store
      .wb_node_migrated(node)
      .map_err(FacadeError::Store)?
    {
      return Ok(());
    }
    Err(FacadeError::WbNotMigrated {
      node: node.to_string(),
    })
  }

  /// Stamp the acting node's heartbeat.
  ///
  /// **THE SERVICE READS THE CLOCK AND NO CALLER OFFERS A TIME.** A heartbeat is
  /// the one field whose entire meaning is "this node was alive at this
  /// moment", so a caller-supplied value would be the fabricated stamp with the
  /// model's blessing -- the class this whole model exists to close.
  pub fn wb_touch(&mut self, node: &str) -> Result<(), FacadeError> {
    self.require_migrated(node)?;
    let event = self.wb_event("wb.touch", node, json!({}));
    self
      .store
      .wb_write(&event, |w| w.touch(node))
      .map_err(FacadeError::Store)?;
    self.land_board_write_noting()?;
    Ok(())
  }

  /// Pause the acting node, and stamp its heartbeat on the way out.
  ///
  /// **IT TOUCHES AS WELL, because the last thing a paused node says is WHEN it
  /// stopped.** A pause that left the heartbeat where it was would make a node
  /// that released cleanly indistinguishable from one that died mid-turn, and
  /// telling those apart is the whole point of a heartbeat on a board peers
  /// read.
  pub fn wb_release(&mut self, node: &str) -> Result<(), FacadeError> {
    self.require_migrated(node)?;
    let event = self.wb_event("wb.release", node, json!({}));
    self
      .store
      .wb_write(&event, |w| {
        w.set_status(
          node,
          &crate::model::enum_str(&crate::model::WbNodeStatus::Paused),
        )?;
        w.touch(node)
      })
      .map_err(FacadeError::Store)?;
    self.land_board_write_noting()?;
    Ok(())
  }

  /// What a node needs at the start of a session: it is marked active with its
  /// heartbeat moved, the session and focus it names are recorded, and it gets
  /// back its own board and its peers' state.
  ///
  /// **A COMPOSITE OF THE READS WITH ONE WRITE OF ITS OWN** (vc's rulings,
  /// 2026-09-12 and 2026-09-13). The reads are [`Self::boards`]; the write is
  /// what a session start states, which is what makes a released node active
  /// again. An unnamed session or focus keeps what the header holds.
  ///
  /// **THE WRITE HAPPENS BEFORE THE READ, DELIBERATELY.** A node's own board is
  /// part of what this returns, so reading first would hand back a header this
  /// very call is about to change -- a value that was true when it was read and
  /// false by the time it was printed.
  ///
  /// The board comes back as [`BoardRead`] does it: live messages unless `all`.
  pub fn wb_pickup(
    &mut self,
    node: &str,
    session_id: Option<&str>,
    focus: Option<&str>,
    all: bool,
  ) -> Result<Pickup, FacadeError> {
    self.require_migrated(node)?;
    let event = self.wb_event(
      "wb.pickup",
      node,
      json!({ "session_id": session_id, "focus": focus }),
    );
    self
      .store
      .wb_write(&event, |w| {
        w.pick_up(
          node,
          &crate::model::enum_str(&crate::model::WbNodeStatus::Active),
          session_id,
          focus,
        )
      })
      .map_err(FacadeError::Store)?;
    self.land_board_write_noting()?;
    let boards = self.canon.boards.clone();
    #[allow(
      clippy::expect_used,
      reason = "INVARIANT: require_registered passed above, and a registered node always has a board in canon"
    )]
    let board = boards
      .iter()
      .find(|b| b.node.moniker == node)
      .cloned()
      .expect("require_registered passed, so this node has a board");
    let shown = |b: &crate::model::Board, i: &crate::model::WbItem| {
      b.node.moniker == crate::model::HYPERVISOR && STANDING_KINDS.contains(&i.kind)
    };
    // In the board's section order, as `wb show hv` prints them.
    let standing = boards
      .iter()
      .find(|b| b.node.moniker == crate::model::HYPERVISOR && b.node.moniker != node)
      .map(|b| {
        crate::views::BOARD_SECTIONS
          .iter()
          .flat_map(|(kind, _)| {
            b.items.iter().filter(move |i| {
              i.kind == *kind && i.state == crate::model::WbItemState::Live && shown(b, i)
            })
          })
          .cloned()
          .collect()
      });
    let peers = boards
      .into_iter()
      .filter(|b| b.node.moniker != node)
      .map(|b| {
        let mut unshown: Vec<KindCount> = Vec::new();
        for (kind, _) in crate::views::BOARD_SECTIONS {
          let count = b
            .items
            .iter()
            .filter(|i| {
              i.kind == kind && i.state == crate::model::WbItemState::Live && !shown(&b, i)
            })
            .count();
          if count > 0 {
            unshown.push(KindCount { kind, count });
          }
        }
        PeerRead {
          node: b.node,
          unshown,
        }
      })
      .collect();
    Ok(Pickup {
      board: BoardRead::of(board, all),
      standing,
      peers,
    })
  }

  /// Add one item of a WRITABLE kind to the acting node's own board.
  ///
  /// **`decision` IS REFUSED HERE AND `wb_decide` IS WHY** (vc, 2026-09-12).
  /// One door per kind: a decision is broadcast by sitting on a board its peers
  /// read, and a second way to write one is a second place the rule about what
  /// a decision is for would have to be stated. The refusal names the verb, so
  /// the caller is redirected rather than told no.
  ///
  /// **THE VERB EXISTS BECAUSE THE GENERATED VIEW CLOSES THE OTHER DOOR.** Until
  /// the cutover a node writes its DOING and its watch-outs by editing
  /// `wip.md`; afterwards that file is rendered, and a hand edit is skew doctor
  /// reports. Without this verb a node would have no way to record its next
  /// piece of work at all -- the model would have taken the board away and
  /// given nothing back.
  pub fn wb_add(&mut self, node: &str, kind: WbItemKind, text: &str) -> Result<u32, FacadeError> {
    if kind == WbItemKind::Decision {
      return Err(FacadeError::WbKindHasItsOwnVerb {
        kind: crate::model::enum_str(&kind),
        verb: "intent wb decide".to_string(),
      });
    }
    // **A DIRECTIVE IS WRITTEN ON `hv`'s BOARD AND NOWHERE ELSE** (vc, 2026-09-15,
    // issue 0375). A standing directive is the hypervisor's instruction to every
    // node, so the same row on any other board would be a node issuing one.
    if kind == WbItemKind::Directive && node != crate::model::HYPERVISOR {
      return Err(FacadeError::WbDirectiveOffHv {
        node: node.to_string(),
      });
    }
    self.wb_add_item(node, kind, text)
  }

  /// Record a decision on the acting node's own board.
  ///
  /// **A DECISION IS AN ITEM RATHER THAN A MESSAGE, and the difference is who
  /// it is FOR.** A message is addressed and expects handling; a decision is
  /// broadcast by being on a board its peers read at pickup, which is how the
  /// markdown protocol already works. Giving it a recipient would turn one
  /// durable statement into four copies that can diverge.
  pub fn wb_decide(&mut self, node: &str, text: &str) -> Result<u32, FacadeError> {
    self.wb_add_item(node, WbItemKind::Decision, text)
  }

  /// Append one item of one kind to the acting node's own board.
  ///
  /// **ONE DOOR FOR EVERY KIND, so the bound and the stamp rule are stated
  /// once.** `wb_decide` and `wb_add` are its callers, and a second insert
  /// path would be the place one of them quietly skipped the bound.
  pub fn wb_add_item(
    &mut self,
    node: &str,
    kind: WbItemKind,
    text: &str,
  ) -> Result<u32, FacadeError> {
    self.require_migrated(node)?;
    self.check_body_bound(node, text)?;
    let wire = crate::model::enum_str(&kind);
    let cfg = self.project.config().whiteboard.clone();
    if cfg.bounds_apply_to(node) {
      let live = self
        .store
        .wb_live_item_count(node, &wire)
        .map_err(FacadeError::Store)?;
      if live >= cfg.live_items {
        return Err(FacadeError::WbItemsFull {
          node: node.to_string(),
          kind: wire,
          live,
          bound: cfg.live_items,
        });
      }
    }
    // The verb the caller ran: `wb_decide` and `wb_add` share this door.
    let op = if kind == WbItemKind::Decision {
      "wb.decide"
    } else {
      "wb.add"
    };
    let event = self.wb_event(op, node, json!({ "kind": wire, "text": text }));
    let seq = self
      .store
      .wb_write(&event, |w| w.insert_item(node, &wire, text, None))
      .map_err(FacadeError::Store)?;
    self.land_board_write_noting()?;
    Ok(seq)
  }

  /// Carry one node's hand-authored board into the model -- AC-14.9.
  ///
  /// **IT READS THE DISK AND IT IS THE ONLY WHITEBOARD VERB THAT DOES.** Every
  /// other verb in this family reads and writes rows; this one exists to end the
  /// markdown era for one node, so it takes that node's `wip.md`, every
  /// `inbox.<sender>.md` it owns, and every `.history/` snapshot, and it writes
  /// what it found.
  ///
  /// **NOTHING IS DROPPED SILENTLY, AND THE ACCOUNTING IS PER ITEM ON BOTH
  /// SIDES** ([`WbMigration::reconciles`]). A count that reconciles arithmetically
  /// tells nobody WHICH line went, so every carried item and every refused one
  /// comes back with its `<file>:<line>`, and the two halves are asserted to
  /// account for every item-shaped line the source offered.
  ///
  /// **IT APPLIES NO LIVE BOUND AND MARKS NOTHING HANDLED** (hv, ruled
  /// 2026-09-12). The bounds are a refusal on `ask`, `announce` and `add` -- a
  /// board's own author being told to prune before writing more -- and applying
  /// them here would make a node's history refuse to be carried because it is
  /// long, which is the one moment pruning is not available.
  ///
  /// **AFTER THE CARRY THE TWO BOUNDS DIFFER, AND THIS SENTENCE ONCE SAID THEY
  /// DID NOT** (issue 0444). A migrated BOARD over its item bound refuses its
  /// owner's next `add` until something is archived, because carried items
  /// count. A migrated INBOX refuses no sender: a carried message holds an
  /// `authored_at` and stays out of
  /// [`crate::store::Store::wb_live_message_count`] until the recipient's first
  /// `wb clear` (vc, ruled 2026-09-14; issue 0374).
  /// **THE ASYMMETRY IS THE DESIGN, SO DO NOT TIDY IT INTO AGREEMENT:** the
  /// message bound protects a SENDER from someone else's backlog, while the
  /// item bound is the owner's own board, and archiving is the owner's act.
  ///
  /// **A NODE THAT ALREADY HOLDS ROWS IS REFUSED BY NAME rather than carried
  /// twice.** A second run cannot tell its own earlier work from a live write
  /// made since, so there is no version of "merge" here that is not a guess; the
  /// remedy names what to do about it.
  ///
  /// **THE ACTING NODE IS THE OPERATOR'S ARGUMENT, NOT THE BOARD'S OWNER**,
  /// which is the one deliberate exception to the single-writer convention this
  /// family otherwise holds: a cutover is performed on every node's board by
  /// whoever is running it, and that is a human act with a human behind it.
  ///
  /// **WHAT THE MODEL CANNOT CARRY REFUSES THE CARRY, UNLESS `drop_uncarried`**
  /// (vc decision 20). Every such unit is named before the first write, the way
  /// an unregistered sender is, so the re-run stays open. Whichever way it
  /// goes, the board's `wip.md` and every inbox holding a dropped line are kept
  /// byte for byte as snapshots (issue 0438), so a dropped line is out of the
  /// model and still in the store's prose.
  ///
  /// **A VIEW INTENT RENDERED IS NOT READ AS A BOARD'S LINES** (issue 0439).
  /// Its banner is the renderer's, and a registered node's views offer no unit
  /// once it is set aside, so each such file is named rather than kept.
  pub fn wb_migrate(
    &mut self,
    node: &str,
    drop_uncarried: bool,
  ) -> Result<WbMigration, FacadeError> {
    self.require_registered(node)?;
    let standing = self.board(node)?;
    if !standing.items.is_empty() || !standing.messages.is_empty() {
      return Err(FacadeError::WbAlreadyCarried {
        node: node.to_string(),
        items: standing.items.len(),
        messages: standing.messages.len(),
      });
    }

    let home = self.project.whiteboard_dir().join(node);

    // **INBOXES IN SENDER ORDER AND ENTRIES IN SOURCE FILE ORDER**, because
    // every message migrated in one pass shares one `recorded_at`: the stamp
    // cannot order them, so insertion order is the board's only surviving
    // ordering and it has to be a decided one rather than whatever the
    // filesystem hands back.
    let mut inboxes: Vec<(String, String, std::path::PathBuf)> = Vec::new();
    for entry in Self::read_node_dir(&home)? {
      let name = entry.file_name().to_string_lossy().to_string();
      if let Some(sender) = name
        .strip_prefix("inbox.")
        .and_then(|rest| rest.strip_suffix(".md"))
      {
        inboxes.push((sender.to_string(), name.clone(), entry.path()));
      }
    }
    inboxes.sort();

    // **AN INBOX FROM A SENDER THE ROSTER DOES NOT KNOW REFUSES THE WHOLE
    // CARRY, BEFORE ANYTHING IS READ OR WRITTEN** (vc, ruled 2026-09-14). A
    // message row names its sender, so those entries cannot be written, and
    // carrying the rest would leave a board this verb refuses to carry a second
    // time: the skipped entries could then never be carried by any command.
    // Refusing first keeps the re-run open once the sender is registered.
    // Issue 0381: this skipped the inbox and told the operator to register the sender and re-run, which the already-carried refusal then refused.
    let mut strangers: Vec<(String, String)> = Vec::new();
    for (sender, _, path) in &inboxes {
      if !self
        .store
        .wb_node_exists(sender)
        .map_err(FacadeError::Store)?
      {
        strangers.push((sender.clone(), self.project.relative(path)));
      }
    }
    if !strangers.is_empty() {
      return Err(FacadeError::WbSendersNotRegistered {
        node: node.to_string(),
        inboxes: strangers
          .iter()
          .map(|(_, at)| at.as_str())
          .collect::<Vec<_>>()
          .join(", "),
        senders: strangers.into_iter().map(|(sender, _)| sender).collect(),
      });
    }

    let wip = home.join("wip.md");
    let text = Self::read_board_file(&wip)?;
    let wip_rel = self.project.relative(&wip);
    let source = crate::wbmigrate::read_board(node, &text, &wip_rel);

    // **STANDING DIRECTIVES ON ANY BOARD BUT `hv`'s REFUSE THE WHOLE CARRY, BEFORE
    // ANYTHING IS WRITTEN** (vc, 2026-09-15, issue 0375), for the reason an
    // unregistered sender does: carrying the rest would leave a board this verb
    // refuses to carry a second time, and those lines could then reach the model
    // by no command at all.
    if node != crate::model::HYPERVISOR {
      let at: Vec<&str> = source
        .items
        .iter()
        .filter(|i| i.kind == WbItemKind::Directive)
        .map(|i| i.at.as_str())
        .collect();
      if !at.is_empty() {
        return Err(FacadeError::WbDirectivesOnAnotherBoard {
          node: node.to_string(),
          at: at.join(", "),
        });
      }
    }

    // **THE FILES THE CARRY KEEPS VERBATIM, BY NAME AND BYTES** (vc decision 20,
    // issue 0438). The carry renders `wip.md` and every inbox from the rows it
    // writes, so without a copy the markdown each uncoerced, uncarried line came
    // from would be gone from the tree the moment the carry landed. The board is
    // kept whole, and so is each inbox holding a line the model cannot carry;
    // an inbox holding none is carried entry by entry, with nothing left over.
    let mut keep: Vec<(String, String)> = Vec::new();
    // **A VIEW INTENT RENDERED, OFFERING NO UNIT, IS NAMED AND NOT KEPT** (issue
    // 0439). The readers set its banner aside, and what is left is a header and
    // the renderer's empty sections, every byte written from rows, so a copy
    // would keep no board. A rendered view that DOES offer a unit is carried as
    // any board is: rows it shows that this store does not hold are exactly the
    // lines a carry must not lose.
    let mut rendered: Vec<String> = Vec::new();
    if crate::views::view_body(&text).is_some() && source.source_items == 0 {
      rendered.push(wip_rel);
    } else {
      keep.push(("wip.md".to_string(), text));
    }

    let mut messages: Vec<crate::wbmigrate::SourceMessage> = Vec::new();
    let mut message_uncarried: Vec<crate::wbmigrate::Uncarried> = Vec::new();
    for (sender, name, path) in &inboxes {
      let text = Self::read_board_file(path)?;
      let rel = self.project.relative(path);
      let read = crate::wbmigrate::read_inbox(sender, node, &text, &rel);
      if crate::views::view_body(&text).is_some()
        && read.messages.is_empty()
        && read.uncarried.is_empty()
      {
        rendered.push(rel);
      }
      if !read.uncarried.is_empty() {
        keep.push((name.clone(), text));
      }
      messages.extend(read.messages);
      message_uncarried.extend(read.uncarried);
    }

    let (mut snapshots, mut sections, left_in_place) = self.read_snapshots(node, &home)?;

    let uncarried: Vec<crate::wbmigrate::Uncarried> = source
      .uncarried
      .iter()
      .cloned()
      .chain(message_uncarried.iter().cloned())
      .collect();
    let keep: Vec<(std::path::PathBuf, String, String)> = keep
      .into_iter()
      .map(|(name, text)| {
        let path = self.project.wb_pre_migration_snapshot(node, &name);
        let rel = self.project.relative(&path);
        (path, rel, text)
      })
      .collect();
    if !uncarried.is_empty() && !drop_uncarried {
      return Err(FacadeError::WbUncarried {
        node: node.to_string(),
        units: uncarried,
        snapshots: keep.into_iter().map(|(_, rel, _)| rel).collect(),
      });
    }

    let mut kept: Vec<String> = keep.iter().map(|(_, rel, _)| rel.clone()).collect();
    kept.sort();

    // **EVERY COPY IS CHECKED BEFORE ANY IS WRITTEN** (issue 0438). A copy
    // already there with these bytes is an earlier attempt and the walk above
    // has carried it; with other bytes it is refused rather than overwritten,
    // and a refusal over the second copy must not leave the first one written.
    let mut pending: Vec<(std::path::PathBuf, String, String)> = Vec::new();
    for (path, rel, text) in keep {
      if !snapshots.contains(&rel) {
        pending.push((path, rel, text));
      } else if Self::read_board_file(&path)? != text {
        return Err(FacadeError::WbSnapshotInTheWay {
          node: node.to_string(),
          at: rel,
        });
      }
    }

    // Nothing is written until every file has been read, so a refusal on the
    // third inbox does not leave a board half carried.
    for (path, rel, text) in pending {
      let io = |source| {
        FacadeError::Ingest(IngestError::Io {
          path: path.display().to_string(),
          source,
        })
      };
      if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(io)?;
      }
      std::fs::write(&path, &text).map_err(io)?;
      sections.extend(crate::wbmigrate::snapshot_sections(node, &rel, &text));
      snapshots.push(rel);
    }
    snapshots.sort();
    // **ONE EVENT FOR THE CARRY, NOT ONE PER ROW.** The verb is the act; what
    // it carried is in the rows, and the counts say how many.
    let event = self.wb_event(
      "wb.migrate",
      node,
      json!({ "items": source.items.len(), "messages": messages.len() }),
    );
    self
      .store
      .wb_write(&event, |w| {
        w.carry_header(
          node,
          &source.name,
          &source.role,
          source.session_id.as_deref(),
          &crate::model::enum_str(&source.status.unwrap_or(crate::model::WbNodeStatus::Paused)),
          &source.focus,
          &source.claims,
          source.authored_at.as_deref(),
        )?;
        for item in &source.items {
          // **A MIGRATED ITEM CARRIES NO `authored_at`, AND THAT IS THE SOURCE
          // SPEAKING RATHER THAN A FIELD BEING SKIPPED.** A board's markdown
          // stamps its header and its inbox entries; an item is a line in a
          // section and has never claimed a time. Giving it the header's
          // heartbeat would invent a per-item stamp out of a per-board one,
          // which is the fabrication this pair of fields exists to make
          // impossible.
          w.insert_item(node, &crate::model::enum_str(&item.kind), &item.text, None)?;
        }
        for message in &messages {
          w.insert_message(
            &message.sender,
            &message.recipient,
            &message.body,
            message.re.as_deref(),
            message.fyi,
            message.authored_at.as_deref(),
          )?;
        }
        Ok(())
      })
      .map_err(FacadeError::Store)?;
    sections.sort_by(|a, b| (&a.file, a.seq).cmp(&(&b.file, b.seq)));
    self
      .store
      .replace_wb_sections_for(node, &sections)
      .map_err(FacadeError::Store)?;
    // **THE CARRY LANDS ITS OWN VIEWS, NOW THAT NOTHING IT READ IS LEFT TO
    // PROTECT.** Every file was read before the first write, and the header
    // write above stamped the node migrated, so the projection renders this
    // board and its inboxes and leaves every unmigrated peer's markdown alone.
    // Issue 0380: this refreshed the index only, so the carried board stayed hand-authored and doctor refused every commit as skew.
    self.land_board_write_noting()?;
    let (heartbeat_at_carry, heartbeat_authored) = self
      .store
      .wb_node_heartbeats(node)
      .map_err(FacadeError::Store)?;

    Ok(WbMigration {
      heartbeat_authored,
      heartbeat_at_carry,
      node: node.to_string(),
      offered: source.source_items
        + messages.len()
        + message_uncarried.len()
        + snapshots.len()
        + left_in_place.len(),
      items: source.items,
      messages: messages.len(),
      snapshots,
      kept,
      uncarried,
      left_in_place,
      rendered,
    })
  }

  /// One whiteboard file, read with its path in the failure.
  ///
  /// **THE PATH IS THE WHOLE VALUE OF THIS WRAPPER.** A migration reads a
  /// directory's worth of files and `No such file or directory` on its own
  /// names none of them.
  fn read_board_file(path: &std::path::Path) -> Result<String, FacadeError> {
    std::fs::read_to_string(path).map_err(|source| {
      FacadeError::Ingest(IngestError::Io {
        path: path.display().to_string(),
        source,
      })
    })
  }

  /// One node's directory, in a stable order.
  fn read_node_dir(home: &std::path::Path) -> Result<Vec<std::fs::DirEntry>, FacadeError> {
    let mut entries: Vec<std::fs::DirEntry> = std::fs::read_dir(home)
      .map_err(|source| {
        FacadeError::Ingest(IngestError::Io {
          path: home.display().to_string(),
          source,
        })
      })?
      .filter_map(Result::ok)
      .collect();
    entries.sort_by_key(std::fs::DirEntry::file_name);
    Ok(entries)
  }

  /// Every `.history/` file, as verbatim snapshot documents.
  ///
  /// **A FOLD'S ARCHIVE IS A DOCUMENT AND NEVER A SOURCE OF ITEMS** (hv, ruled
  /// 2026-09-12): splitting one into items would put every archived DOING line
  /// back on the board as live work and manufacture a second history of the same
  /// node. The walk is recursive because folds archive by date directory.
  ///
  /// **A FILE THIS CANNOT CARRY IS NAMED RATHER THAN PASSED OVER**, as left in
  /// place: it is not read, not moved and not lost. `.gitkeep` is the exception
  /// and it is a git artefact rather than a document: it exists because git does
  /// not track an empty directory.
  #[allow(clippy::type_complexity)]
  fn read_snapshots(
    &self,
    node: &str,
    home: &std::path::Path,
  ) -> Result<
    (
      Vec<String>,
      Vec<crate::prose::DocSection>,
      Vec<crate::wbmigrate::Uncarried>,
    ),
    FacadeError,
  > {
    let mut carried = Vec::new();
    let mut sections = Vec::new();
    let mut left_in_place = Vec::new();
    let mut pending = vec![home.join(".history")];
    while let Some(dir) = pending.pop() {
      if !dir.is_dir() {
        continue;
      }
      for entry in Self::read_node_dir(&dir)? {
        let path = entry.path();
        if path.is_dir() {
          pending.push(path);
          continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        if name == ".gitkeep" {
          continue;
        }
        let rel = self.project.relative(&path);
        if !name.ends_with(".md") {
          left_in_place.push(crate::wbmigrate::Uncarried {
            at: rel.clone(),
            text: name,
            reason: "a `.history/` file that is not markdown: the snapshot carry splits a \
                     document into prose sections and nothing would read these bytes back, so \
                     the file stays on disk untouched"
              .to_string(),
          });
          continue;
        }
        let text = Self::read_board_file(&path)?;
        sections.extend(crate::wbmigrate::snapshot_sections(node, &rel, &text));
        carried.push(rel);
      }
    }
    carried.sort();
    left_in_place.sort_by(|a, b| a.at.cmp(&b.at));
    Ok((carried, sections, left_in_place))
  }

  /// Move one of the acting node's live items to archived, and say whether it
  /// moved.
  ///
  /// **THE STATE CHANGE IS THE SCHEDULE** (vc's ruling, 2026-09-12, on cc's
  /// finding that the roll had no source population). AC-14.6 forbids a second
  /// act after the fact is stated -- the fold, the sweep, the run somebody has
  /// to remember -- not the statement itself. Handled and done are facts only
  /// the node can state, so an item leaves the live count the moment it is
  /// stated, deterministically, with no timer and nothing swept. `wb clear` is
  /// the same transition for a message.
  ///
  /// **IT TAKES A KIND AS WELL AS A `seq`, BECAUSE `seq` ALONE IS AMBIGUOUS.**
  /// [`crate::store::WbWrite::insert_item`] numbers within (node, kind), so a node can hold a
  /// `doing` 1 and a `decision` 1 at once. The pair is what a reader already
  /// sees: `wb show` prints `[decision] 1`.
  ///
  /// **ONE VERB FOR ONE TRANSITION, AND IT READS TWO WAYS BY DESIGN.** For a
  /// `doing` or `todo` item, archiving is what DONE means; for a `decision` or
  /// `watchout` it is retirement. Splitting it into two verbs would be two doors
  /// on one state change, which is where they drift.
  pub fn wb_archive(
    &mut self,
    node: &str,
    kind: WbItemKind,
    seq: u32,
  ) -> Result<bool, FacadeError> {
    self.require_migrated(node)?;
    let wire = crate::model::enum_str(&kind);
    let event = self.wb_event("wb.archive", node, json!({ "kind": wire, "seq": seq }));
    let moved = self
      .store
      .wb_write(&event, |w| w.archive_item(node, &wire, seq))
      .map_err(FacadeError::Store)?;
    self.land_board_write_noting()?;
    Ok(moved)
  }

  /// Add one claim to the acting node's own board, and say whether it moved.
  ///
  /// **IDEMPOTENT, AND IT REPORTS WHAT MOVED RATHER THAN WHAT IS THERE.**
  /// Claiming something already claimed is not an error -- a node re-asserting
  /// its own lane is the normal case at pickup -- but reporting success either
  /// way would say a write happened when none did, which is the rule
  /// `wb register` and `wb clear` already answer to.
  pub fn wb_claim(&mut self, node: &str, claim: &str) -> Result<bool, FacadeError> {
    self.require_migrated(node)?;
    if !crate::model::is_claim_address(claim) {
      return Err(FacadeError::WbClaimMalformed {
        claim: claim.to_string(),
      });
    }
    let mut claims = self.store.wb_claims(node).map_err(FacadeError::Store)?;
    if claims.iter().any(|c| c == claim) {
      return Ok(false);
    }
    claims.push(claim.to_string());
    let event = self.wb_event("wb.claim", node, json!({ "claim": claim }));
    self
      .store
      .wb_write(&event, |w| w.set_claims(node, &claims))
      .map_err(FacadeError::Store)?;
    self.land_board_write_noting()?;
    Ok(true)
  }

  /// Drop one claim from the acting node's own board, and say whether it moved.
  ///
  /// **AN UNCLAIM OF SOMETHING UNCLAIMED IS NOT A REFUSAL.** The end state the
  /// caller asked for is the end state they get, and refusing would make the
  /// obvious cleanup -- unclaim everything, whatever the board says -- a script
  /// that has to check first.
  pub fn wb_unclaim(&mut self, node: &str, claim: &str) -> Result<bool, FacadeError> {
    self.require_migrated(node)?;
    let claims = self.store.wb_claims(node).map_err(FacadeError::Store)?;
    let kept: Vec<String> = claims.iter().filter(|c| *c != claim).cloned().collect();
    if kept.len() == claims.len() {
      return Ok(false);
    }
    let event = self.wb_event("wb.unclaim", node, json!({ "claim": claim }));
    self
      .store
      .wb_write(&event, |w| w.set_claims(node, &kept))
      .map_err(FacadeError::Store)?;
    self.land_board_write_noting()?;
    Ok(true)
  }

  /// Send one message from `sender` into `recipient`'s board.
  ///
  /// **THE ACTING NODE COMES FROM THE CALLER, SO AC-14.5's SINGLE-WRITER RULE IS
  /// A CONVENTION AT THIS LAYER AND NOT A GUARANTEE** (ic, measured against the
  /// surface rather than against this signature). It is tempting to say the door
  /// has no parameter for writing as somebody else, and it is false: `sender` IS
  /// that parameter, and `--node dc` above it lands a message in a board as dc
  /// from any session. The markdown form held the rule by the filesystem -- you
  /// wrote your own file -- and moving it here moved it to whoever types the
  /// flag. **Making it structural means taking the acting node from somewhere
  /// the caller does not choose, which is a larger design than this cut**, and
  /// saying so is the only honest state to leave it in: nothing above this layer
  /// can tell an honest `--node` from a dishonest one.
  ///
  /// What this door DOES hold is narrower and real: a message can only be
  /// addressed to a registered node, and it lands on the RECIPIENT's board.
  ///
  /// **NO PARAMETER TAKES A TIMESTAMP.** The fabricated-stamp class closes by
  /// construction rather than by detection: there is nothing for a caller to
  /// supply and therefore nothing to validate, and the store reads the clock at
  /// the write. **THE MIGRATION IS THE ONE WRITER THAT DOES TAKE A STAMP, BY
  /// DESIGN** (ic): `authored_at` carries verbatim what a board's markdown
  /// claimed, which is a doorway rather than a hole -- the untrusted claim is
  /// kept as a claim, typed as text and never read as a time.
  pub fn wb_ask(
    &mut self,
    sender: &str,
    recipient: &str,
    body: &str,
    re: Option<&str>,
    fyi: bool,
  ) -> Result<(), FacadeError> {
    self.require_registered(sender)?;
    self.require_migrated(recipient)?;
    self.check_body_bound(sender, body)?;
    let cfg = self.project.config().whiteboard.clone();
    if cfg.bounds_apply_to(sender) {
      let live = self
        .store
        .wb_live_message_count(sender, recipient)
        .map_err(FacadeError::Store)?;
      if live >= cfg.live_messages {
        return Err(FacadeError::WbInboxFull {
          sender: sender.to_string(),
          recipient: recipient.to_string(),
          live,
          bound: cfg.live_messages,
        });
      }
    }
    let event = self.wb_event(
      "wb.ask",
      sender,
      json!({ "recipient": recipient, "body": body, "re": re, "fyi": fyi }),
    );
    self
      .store
      .wb_write(&event, |w| {
        w.insert_message(sender, recipient, body, re, fyi, None)
      })
      .map_err(FacadeError::Store)?;
    self.land_board_write_noting()?;
    Ok(())
  }

  /// Send one message to every OTHER registered node, and say how many boards
  /// it reached.
  ///
  /// **IT IS `wb_ask` IN A LOOP RATHER THAN A SECOND WRITE PATH.** An announce
  /// that inserted rows itself would be a second place the bounds, the roster
  /// check and the stamp rule are stated, and the one that drifts is whichever
  /// is edited second.
  ///
  /// **A BOUND HIT PART-WAY THROUGH REFUSES THE WHOLE ANNOUNCE.** Every
  /// recipient is checked before any row is written, so a broadcast never
  /// half-lands -- half an announce is worse than none, because the nodes that
  /// received it and the nodes that did not both believe they know what was
  /// said.
  pub fn wb_announce(&mut self, sender: &str, body: &str) -> Result<usize, FacadeError> {
    self.require_registered(sender)?;
    self.check_body_bound(sender, body)?;
    let recipients: Vec<String> = self
      .store
      .wb_monikers()
      .map_err(FacadeError::Store)?
      .into_iter()
      .filter(|m| m != sender)
      .collect();
    // Every recipient is asked before any row is written, for the reason the
    // bounds below are: a broadcast never half-lands.
    for r in &recipients {
      self.require_migrated(r)?;
    }
    let cfg = self.project.config().whiteboard.clone();
    if cfg.bounds_apply_to(sender) {
      for r in &recipients {
        let live = self
          .store
          .wb_live_message_count(sender, r)
          .map_err(FacadeError::Store)?;
        if live >= cfg.live_messages {
          return Err(FacadeError::WbInboxFull {
            sender: sender.to_string(),
            recipient: r.clone(),
            live,
            bound: cfg.live_messages,
          });
        }
      }
    }
    let event = self.wb_event(
      "wb.announce",
      sender,
      json!({ "recipients": recipients, "body": body }),
    );
    self
      .store
      .wb_write(&event, |w| {
        for r in &recipients {
          w.insert_message(sender, r, body, None, true, None)?;
        }
        Ok(())
      })
      .map_err(FacadeError::Store)?;
    self.land_board_write_noting()?;
    Ok(recipients.len())
  }

  /// Mark every live message `sender` sent this node handled.
  ///
  /// **ONLY THE RECIPIENT CLEARS, and the recipient is the acting node** -- so
  /// the ownership half of AC-14.5 stands exactly as far as the sender half
  /// does, and no further. `--node cc` clears cc's inbox from any session. See
  /// [`Self::wb_ask`] for why that is a convention at this layer and what
  /// making it a guarantee would cost.
  pub fn wb_clear(&mut self, recipient: &str, sender: &str) -> Result<usize, FacadeError> {
    self.require_migrated(recipient)?;
    self.require_registered(sender)?;
    let event = self.wb_event("wb.clear", recipient, json!({ "sender": sender }));
    let cleared = self
      .store
      .wb_write(&event, |w| w.clear_inbox(sender, recipient))
      .map_err(FacadeError::Store)?;
    self.land_board_write_noting()?;
    Ok(cleared)
  }

  /// One node's board, refused BY NAME when the moniker is not on the roster.
  ///
  /// **AN ABSENT NODE IS A REFUSAL AND NEVER AN EMPTY BOARD.** They render
  /// almost identically -- no items, no messages -- and they mean opposite
  /// things: one is a node with nothing to say, the other is a question about
  /// somebody who is not here. A reader that answered both with the same empty
  /// shape would let a typo look like a quiet colleague.
  pub fn board(&self, node: &str) -> Result<Board, FacadeError> {
    let boards = self.boards()?;
    boards
      .iter()
      .find(|b| b.node.moniker == node)
      .cloned()
      .ok_or_else(|| FacadeError::WbNodeNotRegistered {
        node: node.to_string(),
        known: if boards.is_empty() {
          "no nodes at all".to_string()
        } else {
          boards
            .iter()
            .map(|b| b.node.moniker.clone())
            .collect::<Vec<_>>()
            .join(", ")
        },
      })
  }

  pub fn sync_to_disk(&mut self, scope: &SyncScope) -> Result<usize, FacadeError> {
    self.refuse_if_the_last_ingest_was_refused()?;
    let (threads, issues) = self.store.load_canon().map_err(FacadeError::Store)?;
    let sections = self.store.doc_sections().map_err(FacadeError::Store)?;
    let boards = self.store.hydrate_boards().map_err(FacadeError::Store)?;
    let canon = Canon {
      threads,
      issues,
      sections,
      boards,
    };
    self.check_scope(scope, &canon.threads)?;
    let all_threads: Vec<&Thread> = canon
      .threads
      .iter()
      .filter(|t| scope.selects(&t.id))
      .collect();
    // **Issues are not threads, so a thread scope names none of them and none
    // of them are written.** Projecting them anyway would mean `sync --to-disk
    // ST0056` rewrote forty issue files nobody asked about, which is the
    // estate-wide write this exists to stop wearing a narrower name.
    let all_issues: Vec<&Issue> = match scope.named() {
      None => canon.issues.iter().collect(),
      Some(_) => Vec::new(),
    };
    let count = all_threads.len();
    let Projection {
      mut set,
      canon_files,
    } = self.projection(&canon, &all_threads, &all_issues, None, None)?;
    for (path, content) in self.attachments_the_disk_lacks(&canon, scope)? {
      set.add_bytes(path, content);
    }
    self.refuse_if_this_would_empty_a_populated_face(&canon, &set)?;
    self.refuse_if_canon_moved_under_the_store(&set, &canon_files)?;
    let applied = set.commit()?;
    let landed: Vec<std::path::PathBuf> = applied.written().map(std::path::PathBuf::from).collect();
    self.record_landed(&canon_files, &landed)?;
    // **WHAT LANDED, NOT WHAT WAS ASKED FOR.** `commit` skips a path whose
    // bytes already match, so a sync over an estate that already agrees writes
    // nothing -- and recording the SET would put an act that did not happen
    // into the one table that cannot be re-derived. `WriteSet::writes` answers
    // the right question before a commit and the wrong one after it.
    let wrote = self.estate_paths(&applied);
    applied.keep();
    // **THIS EVENT IS NEVER IN THE FILE IT DESCRIBES, AND THAT IS A FIXED
    // POINT RATHER THAN A GAP.** `add_event_log` put the store's log into the
    // set four lines up, so the projection was computed before this act
    // finished. Recording earlier would record a PLAN and call it an act --
    // the trade `record_disk_act` already names and refuses. So `sync
    // --to-disk` leaves the store exactly one event ahead of the file, every
    // time, and `doctor`'s unsynced count has a floor of ONE rather than zero.
    //
    // **A log that projects itself cannot contain the record of its own
    // writing.** The alternative is a second write that nothing records, which
    // buys a clean number by MOVING the unrecorded act rather than removing
    // it -- and an unrecorded write is the whole defect AC-09.1 exists for.
    if !wrote.is_empty() {
      self.record_disk_act(
        "disk.sync_to_disk",
        serde_json::json!({
          "scope": scope.named(),
          "threads": count,
          "wrote": wrote,
        }),
      )?;
    }
    self.canon = canon;
    Ok(count)
  }

  /// **disk -> db. The DESTRUCTIVE direction: replace the store from the
  /// files.**
  ///
  /// Under the reversed D01 this is a RESTORE, not a refresh. It reads the
  /// re-creatable side and overwrites the source of truth with it, so a change
  /// that is in the store and not yet projected is destroyed -- which is
  /// exactly the situation the file-write failure leaves behind, and why no
  /// remedy may send an operator here to recover.
  ///
  /// Callers are expected to have shown [`Facade::sync_overwrite`] first. This
  /// method does not print, because the facade renders nothing; it refuses
  /// nothing either, because a service call with a stated direction has
  /// already been chosen. The REFUSAL belongs on the bare verb (AC-03.9).
  pub fn sync_from_disk(&mut self, scope: &SyncScope) -> Result<usize, FacadeError> {
    let (canon, count, Projection { set, canon_files }) =
      self.render_from_disk(scope, ingest::Load::Restore)?;
    let applied = set.commit()?;
    let landed: Vec<std::path::PathBuf> = applied.written().map(std::path::PathBuf::from).collect();
    self.record_landed(&canon_files, &landed)?;
    self.finish_from_disk(scope, canon, count, applied)
  }

  /// **disk -> db for intentd's background pass after an external edit: the
  /// disk wins only where it says something the store did not write** (issue
  /// `0216`).
  ///
  /// The same engine as [`Facade::sync_from_disk`] under a different
  /// `ingest::Load`, so the daemon runs no second sync implementation (D32).
  /// It is NOT the restore: nobody typed it and nobody was shown an OVERWRITES
  /// preview, so it must not destroy a write whose commit has landed and whose
  /// canon file has not -- which the restore's wholesale rebuild did, ~1s after
  /// the writer printed `ok`.
  ///
  /// **AND ITS FILES LAND ONLY WHERE THE STORE HAS NOT MOVED SINCE ITS
  /// SNAPSHOT** (issue `0441`). The pass renders every thread, issue, board and
  /// declared view from one snapshot, and a command-line write can commit while
  /// it renders. Written anyway, the render put that write's canon file and view
  /// back as they were before it and recorded them as the store's own, and
  /// nothing said so until the next write to the subject warned. So a render
  /// lands only under a held writer lock that finds no other connection's commit
  /// since the snapshot; a pass that finds one renders again, at most
  /// `INGEST_RENDERS` times, and then refuses, naming what it did not write.
  pub fn ingest_from_disk(&mut self, scope: &SyncScope) -> Result<Ingested, FacadeError> {
    let mut unwritten = Vec::new();
    for _ in 0..INGEST_RENDERS {
      let render = self.ingest_render(scope)?;
      match self.ingest_commit(scope, render)? {
        IngestCommit::Written(ingested) => return Ok(ingested),
        IngestCommit::StoreMoved { unwritten: last } => unwritten = last,
      }
    }
    Err(FacadeError::IngestOutpacedByWrites {
      renders: INGEST_RENDERS,
      paths: match unwritten.is_empty() {
        true => "no file".to_string(),
        false => unwritten.join(", "),
      },
    })
  }

  /// The plan bare `intent sync` prints: what `--apply` would do to this
  /// clone, computed WITHOUT WRITING (ST0078, hv's ruling of 2026-09-18).
  ///
  /// **A PURE FUNCTION OF THE STORE, THE TREE AND GIT'S STATUS** (P5, AC-05.1
  /// and 05.2): git is read for the upstream and the unmerged index,
  /// [`crate::plan::classify`] sorts the unmerged paths, and the steps are laid
  /// out in the order [`crate::plan`] documents, each with its recoverability,
  /// under the tree's digest. Outside a git repository there are no git steps.
  ///
  /// **THE INGEST STEP'S PREVIEW RUNS THE DAEMON'S OWN ENGINE**, against a
  /// shadow: an in-memory store holding a copy of this store's estate and file
  /// index, which is everything the ingest's decision reads. A second
  /// implementation of the decision, written to answer without writing, would
  /// be the drift D32 forbids; a copy of the whole database would cost this
  /// project's 192 MB store on every bare `sync`. The shadow's render is never
  /// committed, so no file moves and this store is only read. **The views step
  /// is previewed against the model that render produced**, so it names what
  /// the pull changes rather than what the ingest is about to fix. While canon
  /// holds conflict markers neither can be read, and both say they wait.
  ///
  /// The estate and the index are read one after the other, not under one
  /// transaction, so a peer's write landing between them can show in the plan.
  /// The apply decides again under the lock, which is where correctness lives.
  pub fn sync_plan(&self, scope: &SyncScope) -> Result<crate::plan::Plan, FacadeError> {
    use crate::plan::Step;
    let root = self.project.root();
    let mut steps = Vec::new();
    let conflicts = if crate::gitstate::is_work_tree(root) {
      // **NOT WHILE A MERGE IS IN PROGRESS**: mid-pull the upstream's commits
      // are the ones being merged, and telling a person to pull them in the
      // middle of their pull is the one wrong thing to say.
      if !crate::gitstate::merging(root)
        && let Some(behind) = crate::gitstate::behind(root)?
        && behind.commits > 0
      {
        steps.push(Step::behind(behind.upstream, behind.commits));
      }
      crate::plan::classify(&self.project, &crate::gitstate::unmerged(root)?)
    } else {
      crate::plan::Conflicts::default()
    };
    if !conflicts.unowned.is_empty() {
      steps.push(Step::unowned(conflicts.unowned.clone()));
    }
    let mut minted: Vec<String> = Vec::new();
    for (kind, from) in &conflicts.minted_twice {
      let to = self.next_free_id(*kind, &minted)?;
      minted.push(to.clone());
      steps.push(Step::renumber(*kind, from.clone(), to));
    }
    for path in &conflicts.take_sides {
      steps.push(Step::take_side(path.clone()));
    }

    let previous = self.store.file_index().map_err(FacadeError::Store)?;
    let (_, digest) = organize::observe(&self.project, &previous).map_err(FacadeError::Organize)?;
    // **AN UNMERGED VIEW STOPS THE INGEST TOO**: it refuses a tree holding
    // conflict markers, so neither it nor the views step can be previewed
    // until the apply has written over them.
    if conflicts.canon_is_unmerged() || !conflicts.views.is_empty() {
      // **THE TREE HOLDS CONFLICT MARKERS, SO NOTHING CAN BE READ FROM IT
      // YET.** The steps that read the merged canon are listed, and say so.
      steps.push(Step::ingest_after_conflicts());
      if !conflicts.views.is_empty() {
        steps.push(Step::resolve_views(conflicts.views.clone()));
      }
      steps.push(Step::regenerate_views(None));
    } else {
      let render = self.shadow_ingest(scope)?;
      let views = self.organize_preview_over(&render.canon)?;
      steps.push(Step::ingest(render.taken));
      if !conflicts.views.is_empty() {
        steps.push(Step::resolve_views(conflicts.views.clone()));
      }
      steps.push(Step::regenerate_views(Some(views)));
    }
    steps.push(Step::events(self.event_files_waiting()?));
    let change = self.index_change(None)?;
    steps.push(Step::reindex(change.upserts.len() + change.removed.len()));
    steps.push(Step::doctor());
    Ok(crate::plan::Plan { steps, digest })
  }

  /// The ingest's preview: the daemon's own engine against a shadow store.
  ///
  /// **THE SHADOW HOLDS A COPY OF THIS STORE'S ESTATE AND FILE INDEX**, which is
  /// everything the ingest's decision reads, and its render is never
  /// committed, so no file moves and this store is only read.
  fn shadow_ingest(&self, scope: &SyncScope) -> Result<IngestRender, FacadeError> {
    let (threads, issues) = self.store.load_canon().map_err(FacadeError::Store)?;
    let index = self.store.file_index().map_err(FacadeError::Store)?;
    let mut shadow = Store::open_in_memory().map_err(FacadeError::Store)?;
    shadow
      .rebuild(&threads, &issues)
      .map_err(FacadeError::Store)?;
    shadow
      .replace_file_index(&index)
      .map_err(FacadeError::Store)?;
    let seen = shadow.data_version().map_err(FacadeError::Store)?;
    let mut shadow = Self::over(
      self.project.clone(),
      self.ctx.clone(),
      shadow,
      self.canon.clone(),
      seen,
    );
    shadow.ingest_render(scope)
  }

  /// How many committed event files the store does not hold, by NAME: a
  /// directory listing, no file read (ST0078 P1).
  fn event_files_waiting(&self) -> Result<usize, FacadeError> {
    let held = self.store.event_ids().map_err(FacadeError::Store)?;
    Ok(
      ingest::event_files_to_take(&self.project, &held)
        .map_err(FacadeError::Ingest)?
        .len(),
    )
  }

  /// Take the committed event files the store does not hold, and return their
  /// ids. The sync plan's `events` step, for when no ingest pass took them.
  fn take_event_files(&mut self) -> Result<Vec<String>, FacadeError> {
    ingest::take_event_files(&self.project, &mut self.store).map_err(FacadeError::Ingest)
  }

  /// What `organize --apply` would write and remove, rendered from `canon`:
  /// the model as it will stand after the ingest, so the plan names the views
  /// the pull changes rather than the ones the ingest is about to fix.
  fn organize_preview_over(&self, canon: &Canon) -> Result<crate::plan::ViewWork, FacadeError> {
    let (plan, _) = self.organize_plan_over(canon)?;
    let report = plan
      .run(organize::Mode::Preview, &|| plan.digest.clone())
      .map_err(FacadeError::Organize)?;
    let rel = |paths: &[std::path::PathBuf]| -> Vec<String> {
      paths.iter().map(|p| self.project.relative(p)).collect()
    };
    Ok(crate::plan::ViewWork {
      writes: [rel(&report.hydrated), rel(&report.rewritten)].concat(),
      removes: [
        rel(&report.dehydrated),
        rel(&report.pruned_legacy),
        rel(&report.pruned),
      ]
      .concat(),
    })
  }

  /// The next id free in the store AND the tree, past every one in `taken`.
  ///
  /// **THE TREE IS ASKED AS WELL AS THE STORE** (ic, measured building P2):
  /// mid-merge the tree holds the pulled side's canon and directories, which the
  /// store has not loaded, and an id only the store calls free is one the
  /// renumber would then refuse as taken.
  fn next_free_id(
    &self,
    kind: crate::plan::Minted,
    taken: &[String],
  ) -> Result<String, FacadeError> {
    use crate::plan::Minted;
    let names = |dir: std::path::PathBuf| -> Result<Vec<String>, FacadeError> {
      match std::fs::read_dir(&dir) {
        Ok(entries) => Ok(
          entries
            .flatten()
            .map(|e| e.file_name().to_string_lossy().to_string())
            .collect(),
        ),
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(source) => Err(FacadeError::Ingest(IngestError::Io {
          path: dir.display().to_string(),
          source,
        })),
      }
    };
    let stem = |name: &str| name.split('.').next().unwrap_or_default().to_string();
    let highest = match kind {
      Minted::Thread => {
        let mut seen: Vec<u32> = self
          .canon
          .threads
          .iter()
          .filter_map(|t| crate::model::thread_seq(&t.id))
          .collect();
        for name in [
          names(self.project.canon_st_dir())?,
          names(self.project.st_dir())?,
        ]
        .concat()
        {
          seen.extend(crate::model::thread_seq(&stem(&name)));
        }
        seen.extend(taken.iter().filter_map(|id| crate::model::thread_seq(id)));
        seen.into_iter().max().unwrap_or(0)
      }
      Minted::Issue => {
        let mut seen: Vec<u32> = self.canon.issues.iter().map(|i| i.number).collect();
        for name in [
          names(self.project.issues_dir())?,
          names(self.project.issues_view_dir())?,
        ]
        .concat()
        {
          seen.extend(crate::model::issue_seq(&stem(&name)));
        }
        seen.extend(taken.iter().filter_map(|id| crate::model::issue_seq(id)));
        seen.into_iter().max().unwrap_or(0)
      }
    };
    Ok(match kind {
      Minted::Thread => crate::model::thread_id(highest + 1),
      Minted::Issue => crate::model::issue_id(highest + 1),
    })
  }

  /// `intent sync --apply`: the plan, applied (ST0078 WP-05, AC-05.1 to 05.4).
  ///
  /// **THE TREE IS PINNED FIRST.** `shown` is the digest of the plan a person
  /// read; a tree that has moved since is refused before any step runs, as
  /// `organize --apply --plan` refuses.
  ///
  /// **[`crate::plan::gate`] DECIDES EACH STEP AND `ask` IS CALLED ONLY WHEN IT
  /// SAYS ASK**, so the rule lives in the library and the caller owns nothing
  /// but the question. A step that resolves canon and is not run leaves canon
  /// holding conflict markers, so every later step that reads it is left too,
  /// marked as waiting.
  ///
  /// **A STEP IS DECIDED AGAIN WHEN IT RUNS.** The ingest, the views and the
  /// index are recomputed at that moment, and a view regeneration that turns
  /// out to remove a file is gated again as the reversible step it now is,
  /// whatever the plan said before the conflicts were resolved.
  pub fn sync_apply(
    &mut self,
    scope: &SyncScope,
    asking: crate::plan::Asking,
    ask: &mut dyn FnMut(&crate::plan::Step) -> crate::plan::Decision,
  ) -> Result<SyncApplied, FacadeError> {
    use crate::plan::{Action, Decision, Gate, Left, LeftBecause, Step};
    let plan = self.sync_plan(scope)?;
    if let Some(shown) = asking.shown.as_deref()
      && shown != plan.digest
    {
      return Err(FacadeError::SyncPlanMoved {
        shown: shown.to_string(),
        now: plan.digest.clone(),
      });
    }
    let unmerged_views: Vec<String> = plan
      .steps
      .iter()
      .flat_map(|s| match &s.action {
        Action::ResolveViews { paths } => paths.clone(),
        _ => Vec::new(),
      })
      .collect();
    let mut applied = SyncApplied::default();
    let mut canon_unresolved = false;
    for step in plan.steps.iter().filter(|s| s.has_work()) {
      if step.reports_only() {
        applied
          .said
          .push(format!("{}: {}", step.name(), step.describe()));
        continue;
      }
      if step.needs_merged_canon() && canon_unresolved {
        applied.left.push(Left {
          step: step.clone(),
          because: LeftBecause::Waits,
        });
        continue;
      }
      // A regeneration planned before the conflicts were resolved could not
      // see what it would remove, so it is decided on what it finds now.
      let step = match &step.action {
        Action::RegenerateViews { would: None } => {
          Step::regenerate_views(Some(self.organize_preview_over(&self.canon)?))
        }
        // The ingest step takes the event files too, so by now there is
        // usually nothing left for this one; it runs on its own when the
        // ingest had no canon to take or was left behind a conflict.
        Action::Events { .. } => Step::events(self.event_files_waiting()?),
        _ => step.clone(),
      };
      if !step.has_work() {
        continue;
      }
      let decision = match crate::plan::gate(step.recoverability, asking.yes, asking.terminal) {
        Gate::Run => Decision::Run,
        Gate::Ask => ask(&step),
        Gate::Leave => {
          canon_unresolved |= step.resolves_canon();
          applied.left.push(Left {
            step,
            because: LeftBecause::NeedsAPerson,
          });
          continue;
        }
      };
      let said = match (&step.action, decision) {
        (_, Decision::Decline) | (Action::TakeSide { .. }, Decision::Run) => {
          canon_unresolved |= step.resolves_canon();
          applied.left.push(Left {
            step,
            because: LeftBecause::Declined,
          });
          continue;
        }
        (Action::TakeSide { path }, Decision::Take(side)) => self.take_side(path, side)?,
        (Action::Renumber { minted, from, to }, _) => self.renumber_in_merge(*minted, from, to)?,
        (Action::Ingest { .. }, _) => {
          self.write_views_from_store(&unmerged_views)?;
          let ingested = self.ingest_from_disk(scope)?;
          let said = crate::sync::ingested(&ingested.taken, &ingested.events);
          applied.taken = ingested.taken;
          applied.events.extend(ingested.events);
          said
        }
        (Action::ResolveViews { paths }, _) => self.resolve_views(paths)?,
        (Action::RegenerateViews { .. }, _) => {
          let report = self.organize(organize::Mode::Apply)?;
          format!(
            "views: wrote {} and removed {}",
            report.hydrated.len() + report.rewritten.len(),
            report.dehydrated.len() + report.pruned_legacy.len() + report.pruned.len()
          )
        }
        (Action::Events { .. }, _) => {
          let events = self.take_event_files()?;
          let said = crate::sync::ingested(&[], &events);
          applied.events.extend(events);
          said
        }
        (Action::Reindex { .. }, _) => {
          let refreshed = self.index_refresh(None)?;
          format!(
            "index: brought {} file(s) up to date",
            refreshed.updated.len() + refreshed.removed.len()
          )
        }
        (Action::Behind { .. } | Action::Unowned { .. } | Action::Doctor, _) => continue,
      };
      applied.done.push(said);
    }
    applied.doctor = Some(Self::doctor(
      &self.project,
      &self.ctx,
      Some(&self.store),
      crate::doctor::Scope::default(),
    ));
    Ok(applied)
  }

  /// Take one side of a canon file both sides changed, and stage it.
  ///
  /// **THE BYTES COME FROM GIT'S OWN STAGE**, 2 for ours and 3 for theirs, so
  /// the file is exactly one side's and never a mix. A side that deleted the
  /// file removes it. The ingest that follows takes it into the store.
  fn take_side(&mut self, path: &str, side: crate::plan::Side) -> Result<String, FacadeError> {
    let root = self.project.root().to_path_buf();
    let stage = match side {
      crate::plan::Side::Ours => ":2",
      crate::plan::Side::Theirs => ":3",
    };
    let target = root.join(path);
    match crate::gitstate::blob(&root, stage, path)? {
      Some(bytes) => std::fs::write(&target, bytes),
      None => std::fs::remove_file(&target),
    }
    .map_err(|source| FacadeError::SyncDiskStep {
      step: format!("write {} side at {path}", side.as_str()),
      source,
    })?;
    crate::gitstate::stage(&root, &[path.to_string()])?;
    Ok(format!("took {} for {path} and staged it", side.as_str()))
  }

  /// Regenerate unmerged generated views from the store and stage them.
  ///
  /// **ONLY A VIEW THE STORE RENDERS IS WRITTEN AND STAGED.** One it does not
  /// render is named and left unmerged, because staging bytes nothing
  /// regenerated would be resolving a conflict by guessing.
  fn resolve_views(&mut self, paths: &[String]) -> Result<String, FacadeError> {
    let root = self.project.root().to_path_buf();
    let (written, unrendered) = self.write_views_from_store(paths)?;
    crate::gitstate::stage(&root, &written)?;
    let mut said = format!(
      "regenerated and staged {} view(s): {}",
      written.len(),
      written.join(", ")
    );
    if !unrendered.is_empty() {
      said.push_str(&format!(
        "; left unmerged, because the store renders no such view: {}",
        unrendered.join(", ")
      ));
    }
    Ok(said)
  }

  /// Write `paths` as the store renders them now, and return the ones written
  /// and the ones the store renders no view at.
  ///
  /// **ALSO RUN BEFORE THE INGEST, OVER THE UNMERGED VIEWS, AND THAT IS WHY IT
  /// IS ITS OWN STEP.** The ingest refuses a tree holding git's conflict
  /// markers, views included, so an unmerged view would stop the very pass
  /// whose store it must be regenerated from. The store's render before the
  /// ingest is a generated file standing in for a generated file; the one
  /// written after the ingest is what gets staged.
  fn write_views_from_store(
    &mut self,
    paths: &[String],
  ) -> Result<(Vec<String>, Vec<String>), FacadeError> {
    let root = self.project.root().to_path_buf();
    let ctx = self.render_ctx()?;
    let rendered: std::collections::BTreeMap<String, String> =
      views::render_all(&self.project, &self.canon, &ctx)
        .into_iter()
        .map(|v| (self.project.relative(&v.path), v.content))
        .collect();
    drop(ctx);
    let mut set = WriteSet::new();
    let mut written = Vec::new();
    let mut unrendered = Vec::new();
    for path in paths {
      match rendered.get(path) {
        Some(content) => {
          set.add(root.join(path), content.clone());
          written.push(path.clone());
        }
        None => unrendered.push(path.clone()),
      }
    }
    if !set.is_empty() {
      let applied = set.commit()?;
      let landed: Vec<std::path::PathBuf> =
        applied.written().map(std::path::PathBuf::from).collect();
      applied.keep();
      self.record_landed(&[], &landed)?;
    }
    Ok((written, unrendered))
  }

  /// Renumber an id both sides minted, in the middle of the merge (AC-05.2).
  ///
  /// **NOT THE PLAIN VERB, AND ic MEASURED WHY** building P2: mid-merge the old
  /// id's paths hold the OTHER side's canon file and directory, so `st
  /// renumber` would move the pulled record to the new id and remove it from
  /// the old one. Here this clone's record moves in the store, its attachments
  /// are written at the new id from `HEAD`, the new id's canon and views are
  /// written from the store by the one write path, and the old id's paths are
  /// restored to the pulled side from `MERGE_HEAD`. Every path touched is
  /// staged, and nothing else.
  fn renumber_in_merge(
    &mut self,
    minted: crate::plan::Minted,
    from: &str,
    to: &str,
  ) -> Result<String, FacadeError> {
    use crate::plan::Minted;
    let root = self.project.root().to_path_buf();
    if !crate::gitstate::merging(&root) {
      return Err(FacadeError::RenumberNotMerging {
        id: from.to_string(),
      });
    }
    // A clone, so the closure holds no borrow of `self` across the write.
    let project = self.project.clone();
    let rel = |p: std::path::PathBuf| project.relative(&p);
    // The old id's paths, and where this clone's files under each go.
    let (prefixes, sigil, model) = match minted {
      Minted::Thread => {
        let model = crate::renumber::thread(&self.canon, from, to).ok_or_else(|| {
          FacadeError::NoSuchThread {
            id: from.to_string(),
          }
        })?;
        (
          vec![
            (
              rel(self.project.thread_json(from)),
              rel(self.project.thread_json(to)),
            ),
            (
              rel(self.project.canon_st_dir().join(from)),
              rel(self.project.canon_st_dir().join(to)),
            ),
            (
              rel(self.project.thread_dir(from)),
              rel(self.project.thread_dir(to)),
            ),
          ],
          Sigil::SteelThread,
          model,
        )
      }
      Minted::Issue => {
        // Both are issue ids by construction: the plan read them from canon
        // paths and minted `to` itself. A spelling that is not one names no
        // issue, which is the refusal it gets.
        let (old, new) = (
          crate::model::issue_seq(from).unwrap_or(0),
          crate::model::issue_seq(to).unwrap_or(0),
        );
        let model = crate::renumber::issue(&self.canon, old, new)
          .ok_or(FacadeError::NoSuchIssue { number: old })?;
        (
          vec![
            (
              rel(self.project.issue_json(old)),
              rel(self.project.issue_json(new)),
            ),
            (
              rel(self.project.issue_view(old)),
              rel(self.project.issue_view(new)),
            ),
          ],
          Sigil::Issue,
          model,
        )
      }
    };

    // Each side's files under the old id, from the two commits being merged.
    let mut ours: Vec<String> = Vec::new();
    let mut theirs: Vec<String> = Vec::new();
    for (old, _) in &prefixes {
      ours.extend(crate::gitstate::files_under(&root, "HEAD", old)?);
      theirs.extend(crate::gitstate::files_under(&root, "MERGE_HEAD", old)?);
    }
    let theirs_declares =
      match crate::gitstate::blob(&root, "MERGE_HEAD", &rel(self.project.intentfiles_path()))? {
        Some(bytes) => {
          let text = String::from_utf8_lossy(&bytes).to_string();
          intentfiles::unpin(&text, sigil, from)? != text
        }
        None => false,
      };

    // This clone's authored files move to the new id, from `HEAD`. Its canon
    // file and its views are the store's to write, below.
    let thread_dir = (minted == Minted::Thread).then(|| rel(self.project.thread_dir(from)));
    let mut staged: Vec<String> = Vec::new();
    for path in &ours {
      let Some((old, new)) = prefixes
        .iter()
        .find(|(old, _)| path.starts_with(old.as_str()))
      else {
        continue;
      };
      // A prefix that is a FILE -- the canon file, an issue's view -- is the
      // store's to write at the new id. Under the realised directory a
      // generated view is too; everything else there, and every canon sidecar,
      // is this clone's authored bytes.
      if path == old {
        continue;
      }
      let within = path[old.len()..].trim_start_matches('/');
      if Some(old) == thread_dir.as_ref()
        && Project::classify(std::path::Path::new(within)) == ThreadFile::GeneratedView
      {
        continue;
      }
      let target = format!("{new}/{within}");
      let bytes = crate::gitstate::blob(&root, "HEAD", path)?.unwrap_or_default();
      let target_path = root.join(&target);
      if let Some(parent) = target_path.parent() {
        std::fs::create_dir_all(parent).map_err(|source| FacadeError::SyncDiskStep {
          step: format!("create {}", self.project.relative(parent)),
          source,
        })?;
      }
      std::fs::write(&target_path, bytes).map_err(|source| FacadeError::SyncDiskStep {
        step: format!("write {target}"),
        source,
      })?;
      staged.push(target);
    }

    let envelope = Envelope::minted(
      &self.ctx.principal,
      &self.ctx.project_id,
      match minted {
        Minted::Thread => "st.renumber",
        Minted::Issue => "issues.renumber",
      },
      Subject {
        kind: match minted {
          Minted::Thread => "thread",
          Minted::Issue => "issue",
        }
        .to_string(),
        id: to.to_string(),
      },
      json!({ "from": from, "to": to, "mid_merge": true }),
    );
    let renumbering = self.land_renumber(RenumberPlan {
      from: from.to_string(),
      to: to.to_string(),
      sigil,
      moves: Vec::new(),
      stale: Vec::new(),
      rerender: None,
      envelope,
      model,
      foreign: Vec::new(),
    })?;
    // The pulled side declared the old id too, so it stays declared.
    if theirs_declares {
      self.repin(sigil, from)?;
    }

    // The old id's paths go back to the pulled side, file by file.
    let mut old_paths: Vec<String> = ours.iter().chain(theirs.iter()).cloned().collect();
    old_paths.sort();
    old_paths.dedup();
    for path in &old_paths {
      let target = root.join(path);
      let outcome = match crate::gitstate::blob(&root, "MERGE_HEAD", path)? {
        Some(bytes) => target
          .parent()
          .map_or(Ok(()), std::fs::create_dir_all)
          .and_then(|()| std::fs::write(&target, bytes)),
        None => match std::fs::remove_file(&target) {
          Err(source) if source.kind() == std::io::ErrorKind::NotFound => Ok(()),
          other => other,
        },
      };
      outcome.map_err(|source| FacadeError::SyncDiskStep {
        step: format!("restore the pulled side at {path}"),
        source,
      })?;
      staged.push(path.clone());
    }

    // Everything the store wrote at the new id, and the manifest.
    for (_, new) in &prefixes {
      let path = root.join(new);
      if path.is_file() {
        staged.push(new.clone());
      } else if path.is_dir() {
        for file in Project::files_in(&path) {
          staged.push(format!("{new}/{}", file.display()));
        }
      }
    }
    if renumbering
      .moved
      .iter()
      .any(|m| m.starts_with(&rel(self.project.intentfiles_path())))
      || theirs_declares
    {
      staged.push(rel(self.project.intentfiles_path()));
    }
    staged.sort();
    staged.dedup();
    crate::gitstate::stage(&root, &staged)?;
    Ok(format!(
      "renumbered this clone's {from} to {to}, kept the pulled {from}, and staged {} path(s)",
      staged.len()
    ))
  }

  /// Declare `id` in `.intentfiles` again.
  fn repin(&mut self, sigil: Sigil, id: &str) -> Result<(), FacadeError> {
    let manifest = self.project.intentfiles_path();
    let text =
      std::fs::read_to_string(&manifest).map_err(|source| FacadeError::ManifestUnreadable {
        path: manifest.display().to_string(),
        source,
      })?;
    let pinned = intentfiles::pin(&text, sigil, id, None)?;
    if pinned != text {
      let mut set = WriteSet::new();
      set.add(manifest, pinned);
      set.commit()?.keep();
    }
    Ok(())
  }

  /// An ingest pass's first step: read the store's version, take the snapshot
  /// and render it (issue `0441`).
  ///
  /// **THE VERSION IS READ BEFORE THE SNAPSHOT**, so a commit that lands while
  /// the snapshot is being taken moves it as well. That can discard a render
  /// that was already current, at the price of one more, and it never lets a
  /// stale one through.
  ///
  /// **WHAT THE STORE HELD IS READ AFTER THE VERSION TOO**, for the same
  /// reason: a peer's commit between the two moves the version, so the render
  /// is discarded rather than counting that peer's write as something this
  /// pass took.
  pub fn ingest_render(&mut self, scope: &SyncScope) -> Result<IngestRender, FacadeError> {
    let baseline = self.store.data_version().map_err(FacadeError::Store)?;
    let held = self.store.load_canon().map_err(FacadeError::Store)?;
    let (canon, count, projection) = self.render_from_disk(scope, ingest::Load::Ingest)?;
    let taken = changed_subjects(&held, &canon);
    Ok(IngestRender {
      baseline,
      canon,
      count,
      projection,
      taken,
    })
  }

  /// An ingest pass's second step: land `render` only if no other connection
  /// has committed since its snapshot (issue `0441`).
  ///
  /// **THE CHECK, THE FILES AND THEIR RECORD SHARE ONE HOLD OF THE WRITER
  /// LOCK**, so no write can commit between finding the store unmoved and the
  /// files that assume it. When the store has moved, nothing is written and
  /// nothing recorded: the render is older than the store, and the write that
  /// moved it rendered its own subject.
  pub fn ingest_commit(
    &mut self,
    scope: &SyncScope,
    render: IngestRender,
  ) -> Result<IngestCommit, FacadeError> {
    let IngestRender {
      baseline,
      canon,
      count,
      projection: Projection { set, canon_files },
      taken,
    } = render;
    // **THE COMMITTED EVENT FILES LAND UNDER THE SAME HOLD AS THE CANON**
    // (ST0078 P1), read here, before it, because a read needs no lock. The
    // render never touched them (see `ingest::resync_inner`), and the insert
    // skips any id a writer added in between, so reading early loses nothing.
    let events = ingest::read_events_to_take(
      &self.project,
      &self.store.event_ids().map_err(FacadeError::Store)?,
    )
    .map_err(FacadeError::Ingest)?;
    let Some(held) = self
      .store
      .hold_unless_moved(baseline)
      .map_err(FacadeError::Store)?
    else {
      let unwritten = set
        .writes()
        .filter(|(path, content)| {
          !std::fs::read(path).is_ok_and(|disk| disk.as_slice() == *content)
        })
        .map(|(path, _)| self.project.relative(path))
        .collect();
      return Ok(IngestCommit::StoreMoved { unwritten });
    };
    let applied = set.commit()?;
    let landed: Vec<std::path::PathBuf> = applied.written().map(std::path::PathBuf::from).collect();
    let entries =
      ingest::canon_file_entries(&self.project, &Self::landed_paths(&canon_files, &landed))
        .map_err(FacadeError::Ingest)?;
    held
      .record_file_entries(&entries)
      .map_err(FacadeError::Store)?;
    let events = held.ingest_events(&events).map_err(FacadeError::Store)?;
    held.release().map_err(FacadeError::Store)?;
    self
      .finish_from_disk(scope, canon, count, applied)
      .map(|threads| {
        IngestCommit::Written(Ingested {
          threads,
          taken,
          events,
        })
      })
  }

  /// Take a disk load's snapshot and render it: what both directions do before
  /// either writes a file.
  fn render_from_disk(
    &mut self,
    scope: &SyncScope,
    load: ingest::Load,
  ) -> Result<(Canon, usize, Projection), FacadeError> {
    // **Validated against DISK, because disk is this direction's SOURCE.**
    // Checking the store instead would refuse a thread that exists only on
    // disk, which is the one case a restore is most obviously for. The extra
    // read costs a canon parse on scoped runs only, and it happens before
    // anything is written -- a refusal after `resync` would arrive with the
    // store already rebuilt.
    if scope.named().is_some() {
      let on_disk = ingest::read(&self.project)?;
      self.check_scope(scope, &on_disk.threads)?;
    }
    // **THE RECORDING SPANS THE WHOLE DISK -> STORE OPERATION, NOT JUST
    // `resync`.** `resync` writes the store and returns Ok, and the three steps
    // after it can still refuse -- so an inner recording would have closed the
    // record `succeeded` and left the outer refusal invisible, which is the
    // exact silence AC-03.13 exists to break, reached through the machinery
    // built to break it. `ingest::recording` is re-entrant for this: the
    // outermost open load owns the row and the one inside `resync` joins it.
    //
    // **The projection is deliberately OUTSIDE it, and the reason is that the
    // remedy has to stay true.** A failure there is `ViewsNotWritten` -- the
    // store holds the change and the files do not -- and the documented repair
    // for that is `sync --to-disk`. Recording it as a refused ingest would
    // block the very verb the error tells the operator to run.
    let project = &self.project;
    let canon = ingest::recording(&mut self.store, |store| {
      let mut canon = ingest::resync_as(project, store, scope, load)?;

      // **The disk-to-attachments carry, and this is the only caller** (D57-6's
      // second consumer; 5.1b). Until this landed the sole producer of an
      // `Attachment` was the migrator, so a file a person wrote into a thread
      // directory reached neither canon nor the index. Measured on this estate
      // before the fix: 57 threads, 0 carrying attachments, with the files
      // themselves sitting on disk.
      //
      // It runs after `resync` rather than inside it because `resync` also warms
      // a COLD store, and a cold warm must reproduce the committed extract
      // rather than let disk quietly outvote it. The second `rebuild` is the
      // price of keeping that boundary honest, on a path that is explicit,
      // infrequent and already declared destructive.
      let before: std::collections::HashMap<String, Vec<crate::model::Attachment>> = canon
        .threads
        .iter()
        .map(|t| (t.id.clone(), t.attachments.clone()))
        .collect();
      let refused = ingest::collect_attachments_into(project, &mut canon);
      if !refused.is_empty() {
        return Err(IngestError::from(crate::finding::Refusal::new(refused)));
      }
      if load == ingest::Load::Ingest {
        // **ON THE DAEMON'S PASS, ONLY WHAT THE CARRY CHANGED, ONTO THE ESTATE
        // AS IT STANDS UNDER THE LOCK** (issue `0216`). The wholesale second
        // rebuild below, from canon decided in `resync`'s own transaction a
        // moment earlier, deleted a writer committing between the two: the
        // event log shows `ST0082` minted at .315, gone by .325 when a
        // contender minted the same id, and the ingest logged at .331. When
        // the carry changed nothing there is no second write at all.
        let carried: std::collections::HashMap<String, Vec<crate::model::Attachment>> = canon
          .threads
          .iter()
          .filter(|t| before.get(&t.id) != Some(&t.attachments))
          .map(|t| (t.id.clone(), t.attachments.clone()))
          .collect();
        if !carried.is_empty() {
          let (threads, issues) = store.rebuild_deciding(|mut held_threads, held_issues, _| {
            for thread in &mut held_threads {
              if let Some(attachments) = carried.get(&thread.id) {
                thread.attachments = attachments.clone();
              }
            }
            (held_threads, held_issues)
          })?;
          canon.threads = threads;
          canon.issues = issues;
        }
      } else {
        store.rebuild(&canon.threads, &canon.issues)?;
      }

      // **The index is derived during `read`, which by design has not seen the
      // attachments yet, so it has to be re-derived here or the carry lands in
      // canon and reaches the FTS index nowhere.** That is AC-06.4's failure
      // shape exactly -- content present and findable by nothing -- so rebuilding
      // the thread sections is part of the carry rather than a tidy-up after it.
      if !canon.threads.is_empty() {
        canon.sections = ingest::sections_of(project, &canon.threads, &canon.issues, &canon.boards);
        store.replace_doc_sections(&canon.sections)?;
      }

      Ok(canon)
    })?;
    // **The projection narrows with the scope too, and forgetting that is the
    // subtle half.** A restore also rewrites the views of what it read; left
    // unfiltered it would regenerate all 266 views from a store whose unscoped
    // threads it deliberately did not touch -- churning files the operator
    // did not name, which is the estate-wide write this exists to stop.
    let all_threads: Vec<&Thread> = canon
      .threads
      .iter()
      .filter(|t| scope.selects(&t.id))
      .collect();
    let all_issues: Vec<&Issue> = match scope.named() {
      None => canon.issues.iter().collect(),
      Some(_) => Vec::new(),
    };
    let count = all_threads.len();
    // **A SCOPED RESTORE RENDERS ONLY THE VIEWS OF THE THREADS IT TOOK (0259).**
    // The rest keep the store's value, so re-rendering their covers changes
    // nothing but a hand edit a peer is holding -- which it discards.
    let projection = self.projection(&canon, &all_threads, &all_issues, Some(scope), None)?;
    drop(all_threads);
    drop(all_issues);
    Ok((canon, count, projection))
  }

  /// Finish a disk load whose files are on disk and in the file index: what
  /// both directions do after they land.
  fn finish_from_disk(
    &mut self,
    scope: &SyncScope,
    canon: Canon,
    count: usize,
    applied: crate::write_set::Applied,
  ) -> Result<usize, FacadeError> {
    let wrote = self.estate_paths(&applied);
    // **THE INDEX RECORDS THE VIEWS THIS WROTE, AND UNTIL ISSUE `0311` IT
    // RECORDED ONLY THE CANON.** The store knew what it had just put on disk
    // for one half of its own write and not the other, so a running daemon
    // rewrote `todo.md` and `steel_threads.md`, its own watcher read them as an
    // external edit, and it published them to every subscriber and ingested
    // again -- the feedback loop scope is supposed to close, arriving through
    // the one door that had no baseline to compare against.
    //
    // **IT IS THE SAME ACT AS `record_landed`, WHICH EACH DIRECTION RUNS BEFORE
    // THIS, APPLIED TO THE OTHER HALF OF WHAT THE PROJECTION WROTE**, which is why it uses that path rather
    // than a second mechanism. A HAND edit to a view still differs from these
    // bytes and is still reported: this records what the store wrote, never a
    // claim about who may write next.
    applied.keep();
    // **THE DISK ACT OF A RESTORE IS THE RE-PROJECTION, AND IT IS THE HALF
    // NOTHING ELSE RECORDS.** The store side is already covered by
    // `ingest::recording` above (AC-03.13); this is the other side, and it is
    // the one AC-09.1 names -- the restore direction rewrites the views of what
    // it read, so files move under a verb whose name says nothing about
    // writing any.
    //
    // **THE RESTORE TAKES THE COMMITTED EVENT FILES AND DELETES NO EVENT**
    // (ST0078 P1). The canon is replaced from the files; the log only gains
    // the files' events the store lacked, because an event is history and a
    // local act never committed is still a fact. This event is machine-scoped
    // (`event::MACHINE_SCOPED_OPS`), so it stays in this store.
    if !wrote.is_empty() {
      self.record_disk_act(
        "disk.sync_from_disk",
        serde_json::json!({
          "scope": scope.named(),
          "threads": count,
          "wrote": wrote,
        }),
      )?;
    }
    self.canon = canon;
    Ok(count)
  }

  /// What a [`Facade::sync_from_disk`] would overwrite, computed BEFORE it
  /// runs.
  ///
  /// AC-03.9 requires the destructive direction to state what it will
  /// overwrite rather than report what it did. The difference is the whole
  /// point: a summary afterwards is a receipt for a loss, and the operator
  /// needed it one moment earlier.
  ///
  /// It compares the store against the files by VALUE, so an entity present in
  /// both and identical is not listed -- the usual case is an empty answer,
  /// and an empty answer is what makes a non-empty one worth reading.
  /// **The warning narrows with the scope, and a warning that over-reports is
  /// not merely noisy.** It would name files the run will not touch, so the
  /// operator either stops for a loss that is not coming or learns to skim the
  /// list. Both end with the warning unread, which is the state it exists to
  /// prevent -- and it is the same failure as an under-report, arrived at from
  /// the other side.
  /// Attachments whose worktree bytes the git index does not hold (ST0057
  /// AC-03.5).
  ///
  /// **Reported BEFORE the store is written, because after it the report is a
  /// receipt.** `sync --to-store` reads the WORKTREE, so an uncommitted edit
  /// sitting in the tree is carried into canon by whoever syncs next -- and
  /// canon then records an artefact whose bytes exist in no commit, which is
  /// indistinguishable on inspection from a correct one. dc measured that
  /// happening twice in one afternoon, once to the node who wrote the
  /// commit-yours-first rule. **A rule that lives in a peer message is followed
  /// until somebody is mid-task.**
  ///
  /// **It REPORTS and does not refuse, deliberately.** The harm is not at sync
  /// -- canon holding uncommitted bytes in a working tree is a dirty tree:
  /// normal, reversible, nobody's problem. It becomes permanent at the COMMIT,
  /// which is AC-03.6's gate and dc's `canon_commit_check.sh`. Refusing here
  /// would block the ordinary act of saving your own in-flight work, which is
  /// the guard nobody keeps.
  ///
  /// **`None` means the question could not be asked** -- no repository, or git
  /// did not run -- and is not an empty list. The caller must be able to say "I
  /// do not know" rather than print a clean bill of health it did not earn.
  pub fn sync_uncommitted(&self, scope: &SyncScope) -> Result<Option<Vec<String>>, FacadeError> {
    let canon = ingest::read(&self.project)?;
    let mut paths = Vec::new();
    for thread in canon.threads.iter().filter(|t| scope.selects(&t.id)) {
      for att in &thread.attachments {
        paths.push(
          self
            .project
            .relative(&self.project.thread_dir(&thread.id).join(&att.path)),
        );
      }
    }
    Ok(
      crate::sync::uncommitted(self.project.root(), &paths)
        .map(|found| found.iter().map(ToString::to_string).collect()),
    )
  }

  pub fn sync_overwrite(&self, scope: &SyncScope) -> Result<Vec<String>, FacadeError> {
    let (stored_threads, stored_issues) = self.store.load_canon().map_err(FacadeError::Store)?;
    let mut on_disk = ingest::read(&self.project)?;
    // **THE WRITE CARRIES DISK ATTACHMENTS INTO CANON, SO THE PREVIEW MUST SEE
    // WHAT IT WILL CARRY** (0276). `ingest::read` never reads them, so a
    // committed attachment whose bytes had moved was ingested under "nothing
    // the store already held was overwritten". This is the write's own
    // function, so the preview and the act cannot disagree about what changes.
    //
    // Its refusals are not reported here: the write refuses on exactly these
    // findings and names them itself, so a run that proceeds never carries
    // them. This preview answers what a run that PROCEEDS would change.
    let _refused_by_the_write = ingest::collect_attachments_into(&self.project, &mut on_disk);
    let stored_threads: Vec<Thread> = stored_threads
      .into_iter()
      .filter(|t| scope.selects(&t.id))
      .collect();
    // Issues are not threads, so a thread scope names none of them and none of
    // them can be overwritten by the run this is warning about.
    let stored_issues: Vec<Issue> = match scope.named() {
      None => stored_issues,
      Some(_) => Vec::new(),
    };
    let mut out = Vec::new();
    for thread in &stored_threads {
      match on_disk.threads.iter().find(|t| t.id == thread.id) {
        Some(same) if same == thread => {}
        Some(same) => {
          // Named per attachment, the way a thread is named, because a reader
          // told only "differs" goes looking in the JSON, and the change is in
          // a markdown file beside it.
          let diverged: Vec<&str> = thread
            .attachments
            .iter()
            .filter(|held| {
              same
                .attachments
                .iter()
                .any(|disk| disk.path == held.path && disk != *held)
            })
            .map(|held| held.path.as_str())
            .collect();
          let mut rest = same.clone();
          rest.attachments = thread.attachments.clone();
          if rest != *thread || diverged.is_empty() {
            out.push(format!("{}: differs on disk", thread.id));
          }
          for path in diverged {
            out.push(format!("{}: attachment {path} differs on disk", thread.id));
          }
        }
        None => out.push(format!("{}: absent from disk, would be DELETED", thread.id)),
      }
    }
    for issue in &stored_issues {
      match on_disk.issues.iter().find(|i| i.number == issue.number) {
        Some(same) if same == issue => {}
        Some(_) => out.push(format!("issue {}: differs on disk", issue.number)),
        None => out.push(format!(
          "issue {}: absent from disk, would be DELETED",
          issue.number
        )),
      }
    }
    // **THE BOARDS ARE PART OF WHAT A WHOLE-PROJECT RESTORE WRITES, SO THEY ARE
    // PART OF WHAT IT WARNS ABOUT** (issue 0414). Only an unscoped run carries
    // them, and the answer is the restore's own diff, so this list and the rows
    // the write moves are the same set.
    if scope.named().is_none() {
      for orphan in self.store.wb_orphans().map_err(FacadeError::Store)? {
        out.push(format!(
          "board {}: {} item(s) and {} message(s) whose node is not on the roster, would be DELETED",
          orphan.node, orphan.items, orphan.messages
        ));
      }
      out.extend(Self::board_differences(
        &self.store.hydrate_boards().map_err(FacadeError::Store)?,
        &on_disk.boards,
      ));
    }
    Ok(out)
  }

  /// One line per board row a restore of `offered` would change, named the way
  /// `wb show` names it.
  fn board_differences(held: &[Board], offered: &[Board]) -> Vec<String> {
    use crate::model::{BoardRows, RowChange};
    let changes = crate::model::board_changes(held, offered);
    let (was, now) = (BoardRows::of(held), BoardRows::of(offered));
    let verdict = |change: &RowChange| match change {
      RowChange::Added(_) => "on disk only, would be ADDED",
      RowChange::Changed { .. } => "differs on disk",
      RowChange::Removed(_) => "absent from disk, would be DELETED",
    };
    let pick = |change: &RowChange| match *change {
      RowChange::Added(o) | RowChange::Changed { offered: o, .. } => (false, o),
      RowChange::Removed(h) => (true, h),
    };
    let mut out = Vec::new();
    for change in &changes.nodes {
      let (from_held, at) = pick(change);
      let n = if from_held {
        was.nodes[at]
      } else {
        now.nodes[at]
      };
      out.push(format!("board {}: node {}", n.moniker, verdict(change)));
    }
    for change in &changes.items {
      let (from_held, at) = pick(change);
      let i = if from_held {
        was.items[at]
      } else {
        now.items[at]
      };
      out.push(format!(
        "board {}: [{}] {} {}",
        i.node,
        crate::model::enum_str(&i.kind),
        i.seq,
        verdict(change)
      ));
    }
    for change in &changes.messages {
      let (from_held, at) = pick(change);
      let m = if from_held {
        was.messages[at]
      } else {
        now.messages[at]
      };
      out.push(format!(
        "board {}: message from {} recorded {} {}",
        m.recipient,
        m.sender,
        m.recorded_at,
        verdict(change)
      ));
    }
    out
  }

  /// The flat DOING / TODO / DONE view, as markdown -- exactly the bytes
  /// `intent/todo.md` holds.
  ///
  /// **Rendered from the store rather than read off disk**, which is where v2
  /// and v3 differ on this command. v2's `todo` showed the file and generated
  /// it if absent, so a stale file was shown as though it were current; here
  /// the file is an extract and the answer comes from truth.
  ///
  /// **ONE RENDERING FOR BOTH SURFACES AGAIN.** D44 replaced the watermark with
  /// a display WINDOW and vc ruled the window terminal-only, so the file and
  /// the terminal answered differently: the terminal was bounded and the
  /// committed file carried every completion ever. Measured on Lamplight,
  /// 2026-08-26: 2236 lines, 84% of them a DONE section. hv's ruling restores
  /// the watermark and with it the single answer -- **what a person sees and
  /// what the file holds are the same bytes, which is what v2 did.**
  pub fn todo_view(&self) -> Result<String, FacadeError> {
    Ok(views::todo(&self.canon.threads, &self.render_ctx()?))
  }

  /// A thread's file as the model holds it, with nothing written (issue 0399):
  /// the explorer's pane reads an attached document through this.
  pub fn read_thread_file(
    &self,
    id: &str,
    file: &str,
  ) -> Result<views::ThreadFileRead, FacadeError> {
    let thread = self.st_show(id)?;
    Ok(views::read_thread_file(thread, file, &self.render_ctx()?))
  }

  /// The same three buckets, structured, for `intent todo --json`.
  pub fn todo_buckets(&self) -> Result<views::TodoBuckets, FacadeError> {
    Ok(views::todo_buckets(
      &self.canon.threads,
      &self.render_ctx()?,
    ))
  }

  /// Advance the DONE watermark to now -- `intent todo done --flush`.
  ///
  /// **It changes the VIEW and never a thread's status.** v2 said the same in
  /// its own comment: the completion record stays where it is; this moves the
  /// line below which finished work stops being shown.
  ///
  /// **The watermark is an EVENT, not a settings row**, which is what fixes
  /// v2's real defect. v2 wrote the flush instant into `todo.md` and grepped it
  /// back out, so deleting a file the model calls disposable reset the flush and
  /// resurrected everything ever flushed. As an event it survives `rm
  /// intent.db`, travels in `events.jsonl`, MERGES across two machines rather
  /// than conflicting, keeps every flush rather than only the last, and needs no
  /// new table -- so no schema version moves for it.
  pub fn todo_flush(&mut self) -> Result<TodoFlush, FacadeError> {
    let cleared: Vec<String> = self
      .todo_buckets()?
      .done
      .into_iter()
      .map(|i| i.label)
      .collect();
    let next = self.canon.clone();
    // **BOTH RECORDS, AND THEY ARE DIFFERENT THINGS -- BUT THEY LAND TOGETHER
    // OR NOT AT ALL (AC-14.7).** The event is HISTORY, a flush happened, and
    // stays in the log D53 keeps out of the working tree; the watermark is
    // STATE, the cutoff is now this, and state is what canon carries and git
    // moves. Deriving one from the other is what left a fresh clone with no
    // cutoff at all, so they stay two records -- in ONE transaction.
    //
    // **THIS USED TO BE TWO CALLS AND THE CRITERION SAID IT WAS ONE.** `apply`
    // committed with the event in it and a separate unwrapped INSERT followed,
    // so a failure between them left a `todo.flush` in the log that no cutoff
    // reflected -- and AC-14.2 had removed the fallback that would have hidden
    // it, so the flush simply appeared not to have happened while history said
    // it did. Measured and repaired 2026-08-27; the criterion was not
    // uncovered, it was unbuilt.
    let applied = self.apply_with_state(
      crate::event::TODO_FLUSH,
      Subject {
        kind: "todo".to_string(),
        id: "watermark".to_string(),
      },
      serde_json::json!({ "cleared": cleared.len() }),
      next,
      crate::store::ProjectStateEdit::SetTodoWatermark,
    )?;
    self.park(applied);
    // Re-read AFTER the event, so `remaining` is measured rather than assumed.
    let after = self.todo_buckets()?;
    Ok(TodoFlush {
      watermark: after.watermark,
      cleared,
      remaining: after.done.into_iter().map(|i| i.label).collect(),
    })
  }

  /// Write `intent/todo.md` from current status.
  ///
  /// One file rather than the whole projection, because that is what the verb
  /// says. It goes through a [`WriteSet`] like every other write, so it is
  /// atomic and leaves nothing half-written; the CONTENT comes from
  /// [`views::todo`], so this selects which files to write and never re-decides
  /// what they say.
  pub fn todo_update(&mut self) -> Result<(), FacadeError> {
    let content = self.todo_view()?;
    let mut set = WriteSet::new();
    set.add(self.project.todo_view(), content);
    set.commit()?.keep();
    Ok(())
  }

  /// Project the whole estate into a named format, or refuse (AC-06.6).
  ///
  /// **A READ, and it writes nothing** -- not the artefact, not a temp file,
  /// not the store. It returns the bytes and lets the caller decide where they
  /// go, which is what makes `intent export --format json > estate.json` the
  /// operator's choice rather than ours, and what makes a refusal cost nothing.
  ///
  /// It reads the STORE rather than [`Facade::canon`], because the store is
  /// truth (D01 as reversed) and an export is exactly the operation where
  /// answering from a cached view would put stale data in an artefact that
  /// then travels.
  ///
  /// `None` takes [`export::DEFAULT_FORMAT`]. The default is declared with the
  /// roster rather than here, so the surface, the help and this agree by
  /// construction.
  /// Write the whole estate as readable files under `intent/.backup/text/<stamp>/`
  /// (ST0057 WP-06, AC-06.1 / AC-06.2).
  ///
  /// **THE STAMP IS THE DATABASE'S AND THIS IS THE ONLY PLACE IT IS OBTAINED**
  /// (D42). The envelope is minted without a time, `append_event` returns what
  /// the INSERT actually wrote, and that value names the directory -- the same
  /// mechanism `backup.rs` uses for a snapshot filename. `realise::realise`
  /// therefore takes a DESTINATION and never a time, so no signature below the
  /// facade accepts one.
  ///
  /// **The event is recorded BEFORE the files are written, and that ordering is
  /// deliberate.** A realisation that fails half way has still happened, and
  /// its directory has to be named and findable; recording afterwards would
  /// leave the partial tree anonymous -- which is the one state a human
  /// consulting the fallback must never meet.
  pub fn realise(&mut self) -> Result<realise::Realisation, FacadeError> {
    let envelope = Envelope::minted(
      &self.ctx.principal,
      &self.ctx.project_id,
      "text.realise",
      Subject {
        kind: "project".to_string(),
        id: self.ctx.project_id.clone(),
      },
      serde_json::Value::Null,
    );
    let stamp = self
      .store
      .append_event(&envelope)
      .map_err(FacadeError::Store)?;
    // `text.realise` is machine-scoped and writes no file of its own, but this
    // drains the landed queue, so a project event left pending by a failed set
    // lands here rather than riding on the next act.
    self.land_event_files()?;
    // Colons and dots are replaced for the same reason `backup.rs` replaces
    // them: an ISO timestamp is a poor filename on some filesystems and an
    // awkward one on all of them. The ORDER is preserved, because the
    // substitution is character-for-character.
    let root = self
      .project
      .intent_dir()
      .join(".backup")
      .join("text")
      .join(stamp.replace([':', '.'], "-"));
    let ctx = self.render_ctx()?;
    realise::realise(&self.project, &self.canon, &ctx, &root).map_err(FacadeError::Realise)
  }

  /// **`&mut self` SINCE 2026-08-20, AND THE FORMAT ROSTER IS WHY** (AC-06.3).
  /// `md` is [`Projection::Realises`](export::Projection::Realises), whose
  /// artefact is a directory tree rather than a document, and realising one
  /// mints a database stamp. So `export` can no longer promise to be a pure
  /// read for every format it accepts. Taken knowingly rather than worked
  /// around: the alternative was a second markdown producer, and nothing
  /// would have kept the two in agreement.
  pub fn export(&mut self, format: Option<&str>) -> Result<Exported, FacadeError> {
    let (threads, issues) = self.store.load_canon().map_err(FacadeError::Store)?;
    let events = self.store.events().map_err(FacadeError::Store)?;
    let bundle = export::Bundle::new(&self.ctx.project_id, threads, issues, events)
      .with_project_state(crate::model::ProjectState::new(
        self.store.todo_watermark().map_err(FacadeError::Store)?,
      ));
    let projected =
      export::project(&bundle, format.unwrap_or(export::DEFAULT_FORMAT)).map_err(|refusal| {
        // Mapped one-to-one and exhaustively rather than wrapped in a single
        // variant: these three want three different remedies, and one variant
        // for all of them is the same-text-for-different-causes collapse
        // AC-04.4 forbids.
        match refusal {
          ExportRefusal::Unknown {
            name,
            emits,
            refused,
          } => FacadeError::NoSuchFormat {
            format: name,
            emits,
            refused,
          },
          ExportRefusal::Lossy {
            name,
            because,
            instead,
          } => FacadeError::LossyFormat {
            format: name,
            because,
            instead,
          },
          ExportRefusal::RoundTripFailed { name, detail } => FacadeError::ExportRoundTripFailed {
            format: name,
            detail,
          },
        }
      })?;
    // **THE ROSTER DECLARED AND THIS PERFORMS**, which is the split that keeps
    // `export::project` pure. It returns an instruction for a tree-shaped
    // format because it has neither the store nor a clock, and both are
    // required: `realise` mints its directory name from the database (D42).
    match projected {
      export::Projected::Document(text) => Ok(Exported::Document(text)),
      export::Projected::Realise => self.realise().map(Exported::Realised),
    }
  }

  /// What `.intentfiles` says about which threads are realised.
  ///
  /// **THREE INPUTS, THREE STATES, AND THE THIRD ONE HAD TO BE ARGUED FOR.**
  /// The first version returned `Option<BTreeSet<String>>` and wrote
  /// `.ok()?` twice -- which collapsed an UNPARSEABLE manifest into the same
  /// `None` as an ABSENT one (vc, and it is my own finding an hour old wearing
  /// a different hat: `cargo test` returns 101 for a build failure and a test
  /// failure alike, and the discriminator had to be added there too).
  /// **Two-valued returns are the default shape, so the third state has to be
  /// argued for every single time.**
  ///
  /// The two absent-ish states behave IDENTICALLY here -- both realise
  /// everything -- so collapsing them costs nothing at this call site and
  /// everything at the next one. They are separate values so that a reader
  /// which needs to tell them apart does not have to re-derive the distinction
  /// from the filesystem.
  fn realised_threads(&self) -> Realised {
    crate::intentfiles::realised(&self.project.intentfiles_path())
  }

  /// Which thread's directory a view lives under, if any.
  ///
  /// Delegates to [`views::owning_thread`], which `doctor` also consults. **One
  /// answer to which artefact owns a view**: two would let the write path and
  /// the diagnostic path disagree about whether a file should exist.
  fn owning_thread(&self, path: &std::path::Path, canon: &Canon) -> Option<String> {
    views::owning_thread(&self.project, path, canon)
  }

  /// Every file the model projects onto disk, as one batch.
  ///
  /// THE ONE PLACE THE db -> disk DIRECTION IS EXPRESSED. `apply` and both
  /// sync directions go through it, so a mutation and a resync cannot disagree
  /// about what the tree should look like -- which would be a divergent copy
  /// of the projection rules with a filesystem in between.
  fn projection(
    &self,
    canon: &Canon,
    threads: &[&Thread],
    issues: &[&Issue],
    views_of: Option<&SyncScope>,
    before: Option<&Canon>,
  ) -> Result<Projection, FacadeError> {
    let mut set = WriteSet::new();
    let mut canon_files: Vec<(std::path::PathBuf, String)> = Vec::new();
    for thread in threads {
      let path = self.project.thread_json(&thread.id);
      canon_files.push((path.clone(), thread.id.clone()));
      set.add(
        path,
        to_canonical_json(thread).map_err(|e| FacadeError::Store(StoreError::Serde(e)))?,
      );
      // **THE BYTES THE CANON FILE NAMES LAND BESIDE IT (0084).** Canon names an
      // opaque attachment by hash and size and keeps its bytes in a sidecar;
      // without this write the next read of canon found the name and not the
      // file, and refused `broken-reference`.
      for (rel, raw) in crate::export::thread_blobs(thread) {
        set.add_bytes(self.project.intent_dir().join(rel), raw);
      }
    }
    for issue in issues {
      let path = self.project.issue_json(issue.number);
      canon_files.push((path.clone(), format!("issue {:04}", issue.number)));
      set.add(
        path,
        to_canonical_json(issue).map_err(|e| FacadeError::Store(StoreError::Serde(e)))?,
      );
    }
    // **ONE BOARD FILE PER NODE, AND NOT NARROWED BY A THREAD SCOPE**, for the
    // reason project state is not: a board is not a thread, so a thread scope
    // names none of them, and `WriteSet::commit` skips a path whose bytes
    // already match -- so a sync over an estate that already agrees writes
    // nothing here at all.
    //
    // **THE MARKDOWN BESIDE IT IS NOT TOUCHED.** `wip.md` and
    // `inbox.<sender>.md` stay hand-authored and authoritative until the
    // cutover migrates them; this projects only the file the store carries, so
    // the two can coexist without either overwriting the other.
    for board in &canon.boards {
      let path = self.project.board_json(&board.node.moniker);
      canon_files.push((path.clone(), format!("board {}", board.node.moniker)));
      set.add(
        path,
        to_canonical_json(board).map_err(|e| FacadeError::Store(StoreError::Serde(e)))?,
      );
    }
    // **PROJECT STATE, WRITTEN ON EVERY SYNC AND NOT NARROWED BY A THREAD
    // SCOPE.** Issues are skipped by a scoped sync because forty issue files
    // nobody asked about is the estate-wide write a scope exists to prevent.
    // This is ONE file holding one scalar, and `WriteSet::commit` skips a path
    // whose bytes already match -- so writing it always costs nothing when
    // nothing moved, and skipping it would let `sync --to-disk ST0056` leave a
    // stale cutoff on disk after a flush.
    //
    // **BUT IT IS NEVER CREATED TO SAY NOTHING** (issue 0456). With no cutoff
    // the file would hold only its schema, which the reader takes exactly as
    // it takes an absent file. Creating it anyway made every write on a branch
    // that never committed it leave it untracked -- the post-checkout hook did
    // so on every switch -- and the next merge then refused over it. So it is
    // written when there is a cutoff to record, or when it is already there.
    let watermark = self.store.todo_watermark().map_err(FacadeError::Store)?;
    let project_json = self.project.project_json();
    if watermark.is_some() || project_json.exists() {
      set.add(
        project_json,
        to_canonical_json(&crate::model::ProjectState::new(watermark))
          .map_err(|e| FacadeError::Store(StoreError::Serde(e)))?,
      );
    }
    // **VIEWS IF MARKED, AND CANON REGARDLESS** (AC-08.1). The canon writes
    // above are unconditional; only the RENDERED views narrow.
    //
    // Without this, every mutation re-rendered every thread's views from full
    // canon -- so a dehydrated artefact came back the moment anybody touched
    // anything, and `organize` was undone by the next verb anyone ran. **The
    // sparse projection was not a state the estate could hold**, which is
    // ST0057's whole subject.
    //
    // **An ABSENT manifest realises everything, deliberately.** A project that
    // has never run `organize` has no `.intentfiles`, and reading that absence
    // as "nothing is declared" would silently stop rendering every view in
    // every project that has not opted in. Absence means nobody has said, not
    // that the answer is none.
    let realised = self.realised_threads();
    // **A HELD THREAD'S VIEWS ARE SKIPPED, NOT REFUSED** (issue 0209). The
    // write after `st start` realised the thread here, beside the v2 bucket
    // copy it would leave behind, and that write can be any verb about any
    // thread -- so refusing would make every write hostage to one bucket. The
    // named refusal belongs to the realising verbs, `organize` and `hydrate`.
    let held: std::collections::BTreeSet<String> = organize::held(
      &self.project,
      canon,
      &realised,
      &organize::presence(&self.project, canon, &realised),
    )
    .into_iter()
    .map(|h| h.thread)
    .collect();
    // **AN UNDECLARED THREAD'S VIEW IS REFRESHED ONLY WHEN THE STORE IS AHEAD
    // OF IT (0283).** A closed thread drops out of `.intentfiles` while its
    // files stay on disk, so a mutation on it left them stale -- and doctor then
    // called the gap a hand edit. Disk bytes equal to the render of canon BEFORE
    // this change are what the store last put there, so the store is ahead by
    // construction; anything else is a hand edit and is left for doctor. No
    // file index is consulted: it holds scan rows, not renderer writes, so it
    // cannot tell direction. A missing file stays missing, and the thread is
    // never re-declared -- realising is `st hydrate`'s act, not a mutation's.
    let changed: std::collections::BTreeSet<&str> = threads.iter().map(|t| t.id.as_str()).collect();
    let changed_issues: std::collections::BTreeSet<u32> = issues.iter().map(|i| i.number).collect();
    // Rendered only when a changed thread is undeclared -- the ordinary
    // mutation on a declared thread pays nothing for this.
    let undeclared_changed = match &realised {
      // **ASKED THROUGH `Realised`, NOT BY READING ITS SET.** A caller that
      // reaches into `Declared` and spells the key itself is a second site the
      // compiler cannot see -- which is how `ISSUE:0001` was dropped on the way
      // in once already.
      Realised::Declared(_) => {
        changed.iter().any(|id| !realised.declares(id))
          || changed_issues
            .iter()
            .any(|n| !realised.declares_artefact(Sigil::Issue, &format!("{n:04}")))
      }
      // Neither skips a view below, so neither needs the prior render.
      Realised::NothingSaid | Realised::Unreadable => false,
    };
    let rendered_before: Vec<views::View> = match before {
      Some(prior) if undeclared_changed => {
        views::render_all(&self.project, prior, &self.render_ctx()?)
      }
      _ => Vec::new(),
    };
    for view in views::render_all(&self.project, canon, &self.render_ctx()?) {
      // **ONE SKIP FOR BOTH ARTEFACT KINDS, AND IT WAS TWO NEAR-IDENTICAL
      // BLOCKS UNTIL THE SECOND ONE'S QUESTION WAS FOUND TO BE THE FIRST
      // ONE'S.** The thread block read `owning_thread`, which answers `None`
      // for `intent/issues/<nnnn>.md`; `None` makes the whole `&&` false, so
      // the skip never fired for an issue and every issue view was written
      // whatever the manifest said -- **284 of them on the live estate the day
      // WP-01 landed, against a manifest declaring none**, which
      // `organize --apply` would then remove: the projection and the plan
      // disagreeing about the same file.
      //
      // **`store_ahead` APPLIES TO BOTH, AND A COMMENT HERE ONCE SAID AN ISSUE
      // NEEDED NO SUCH THING.** That was wrong within the hour and a test said
      // so: `issues close` moves the record AND undeclares it in one breath, so
      // a plain skip leaves the OPEN render on disk while the store holds the
      // closed one -- and `organize`'s dehydration gate then refuses to remove
      // it, correctly, because it cannot tell a stale render from a hand edit.
      // The thread side has solved exactly this since 0079.
      if let Some(undeclared) = views::undeclared_owner(&self.project, &view.path, canon, &realised)
      {
        let changed_here = match &undeclared {
          views::Undeclared::Thread(id) => changed.contains(id.as_str()),
          views::Undeclared::Issue(number) => changed_issues.contains(number),
          // A board has no manifest entry to change, so nothing is ahead of it.
          views::Undeclared::Board(_) => false,
        };
        let store_ahead = changed_here
          && rendered_before
            .iter()
            .find(|v| v.path == view.path)
            .is_some_and(|prior| {
              std::fs::read_to_string(&view.path).is_ok_and(|disk| disk == prior.content)
            });
        if !store_ahead {
          continue;
        }
      }
      if let Some(owner) = self.owning_thread(&view.path, canon)
        && held.contains(&owner)
      {
        continue;
      }
      if let Some(scope) = views_of
        && let Some(owner) = self.owning_thread(&view.path, canon)
        && !scope.selects(&owner)
      {
        continue;
      }
      set.add(view.path, view.content);
    }
    Ok(Projection { set, canon_files })
  }

  /// **The attachments the store carries and the disk lacks, for an egest to
  /// write** (0082).
  ///
  /// `st attach` writes the store and canon and never the disk, so an
  /// attachment authored canon-first reached nobody who opens files, and
  /// `--to-disk` reported `ok` over it. This asks `organize::plan` -- the
  /// classifier `hydrate` already acts on -- for its `HydrateAttachment` steps
  /// rather than deciding again what is absent.
  ///
  /// **ABSENT ONLY, NEVER A PRESENT ONE.** Attachments are authored on disk, so
  /// one that is present and differs is the author's newer work, and the plan
  /// already calls that `AttachmentDiverged` and names both remedies. Writing
  /// the store's copy over it would be 0260's loss arriving by another door.
  ///
  /// Declared threads only, as the views are, and only within `scope`. A step
  /// with no bytes -- an opaque attachment whose sidecar was never loaded -- is
  /// REFUSED by path, as `organize` refuses it (issue 0338 (i)): skipping it
  /// here projected an estate with the file missing and said nothing.
  fn attachments_the_disk_lacks(
    &self,
    canon: &Canon,
    scope: &SyncScope,
  ) -> Result<Vec<(std::path::PathBuf, Vec<u8>)>, FacadeError> {
    let realised = self.manifest_for_action()?;
    let previous = self.store.file_index().map_err(FacadeError::Store)?;
    let (tree, digest) =
      organize::observe(&self.project, &previous).map_err(FacadeError::Organize)?;
    let ctx = self.render_ctx()?;
    let plan = organize::plan(&self.project, canon, &realised, &ctx, &tree, digest);
    plan
      .steps
      .into_iter()
      .filter(|step| step.action == organize::Action::HydrateAttachment)
      .filter(|step| {
        self
          .owning_thread(&step.path, canon)
          .is_some_and(|id| scope.selects(&id))
      })
      .map(|step| match step.content {
        Some(content) => Ok((step.path, content)),
        None => Err(FacadeError::Organize(
          organize::OrganizeError::NothingToWrite { path: step.path },
        )),
      })
      .collect()
  }

  /// **Record the canon files a projection just landed** (0260), through
  /// [`ingest::record_canon_files`], the one recorder.
  ///
  /// Called only after a commit SUCCEEDS. A refused projection leaves the old
  /// bytes and their old record in place, which is exactly store-ahead.
  fn record_landed(
    &mut self,
    canon_files: &[(std::path::PathBuf, String)],
    written: &[std::path::PathBuf],
  ) -> Result<(), FacadeError> {
    // **THE STORE RECORDS EVERYTHING IT PUT ON DISK, AND UNTIL ISSUE `0311` IT
    // RECORDED ONLY THE CANON HALF.** A projection writes canon, the generated
    // views and `.canon/project.json`; the index learned about the first and
    // not the rest. **The consequence was the feedback loop scope is supposed
    // to close**: a running daemon rewrote `todo.md` and `steel_threads.md`,
    // its own watcher compared them against an index that had never seen those
    // bytes, published them to every subscriber as an external edit, and
    // ingested again -- twice per write, traced on a socket client.
    //
    // **BOTH LISTS, BECAUSE NEITHER CONTAINS THE OTHER.** `commit` skips a path
    // whose bytes already match, so a canon file that was already correct is
    // absent from `written` and must still be recorded -- that baseline is what
    // `refuse_if_canon_moved_under_the_store` reads. And the views are in
    // `written` and were never in `canon_files`. The union is the honest answer
    // to *what does the store now know it wrote*.
    ingest::record_canon_files(
      &self.project,
      &mut self.store,
      &Self::landed_paths(canon_files, written),
    )
    .map_err(FacadeError::Ingest)
  }

  /// What [`Facade::record_landed`] records: every canon file the projection
  /// carried and every path it wrote, sorted, each once.
  ///
  /// **ONE UNION FOR BOTH RECORDERS** (issue `0441`). The ingest pass records
  /// the same set inside its held lock, and a second spelling of it there would
  /// be free to disagree about which files the store now knows it wrote.
  fn landed_paths(
    canon_files: &[(std::path::PathBuf, String)],
    written: &[std::path::PathBuf],
  ) -> Vec<std::path::PathBuf> {
    let mut paths: Vec<std::path::PathBuf> = canon_files.iter().map(|(p, _)| p.clone()).collect();
    paths.extend(written.iter().cloned());
    paths.sort();
    paths.dedup();
    paths
  }

  /// **An egest must not overwrite a canon file that moved since the store
  /// last wrote or read it** (0260).
  ///
  /// Content alone cannot answer this. A store AHEAD of its files -- a
  /// projection refused after the DB committed, which `--to-disk` exists to
  /// repair -- and files AHEAD of their store -- a pull, a peer's commit, with
  /// no intentd to take it in -- disagree identically. Provenance separates
  /// them: a file whose bytes still match the index is where the store left it,
  /// so the store's version is newer; one that does not has moved under it.
  ///
  /// It reads this write's own canon files, so a scope narrows it for free and
  /// another thread's state never refuses this one (issue 0259's shape).
  ///
  /// - **Absent**: re-created. The files are re-creatable (D01).
  /// - **Already equal to what would be written**: nothing to overwrite.
  /// - **No recorded bytes**: written, as before this check existed. A file
  ///   with no baseline has nothing it could have moved from -- the rule
  ///   `ingest::resync` states for covers -- and refusing would block the
  ///   first egest from every store written before the index learned to
  ///   record writes.
  fn refuse_if_canon_moved_under_the_store(
    &self,
    set: &WriteSet,
    canon_files: &[(std::path::PathBuf, String)],
  ) -> Result<(), FacadeError> {
    let index = self.store.file_index().map_err(FacadeError::Store)?;
    let writes: std::collections::HashMap<&std::path::Path, &[u8]> = set.writes().collect();
    let mut moved: Vec<String> = Vec::new();
    for (path, subject) in canon_files {
      let bytes = match std::fs::read(path) {
        Ok(bytes) => bytes,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
        Err(e) => return Err(canon_file_unreadable(path, e)),
      };
      if writes
        .get(path.as_path())
        .is_some_and(|w| *w == bytes.as_slice())
      {
        continue;
      }
      let now = crate::sync::entry_for(self.project.root(), path, &[])
        .map_err(|e| canon_file_unreadable(path, std::io::Error::other(e.to_string())))?;
      if index
        .iter()
        .any(|recorded| recorded.path == now.path && recorded.sha256 != now.sha256)
      {
        moved.push(subject.clone());
      }
    }
    match moved.is_empty() {
      true => Ok(()),
      false => Err(FacadeError::EgestFromStaleStore {
        subjects: moved.join(", "),
      }),
    }
  }

  /// Add the event log's file form to a write set (D34, AC-02.6).
  ///
  /// Run every health check (AC-06.2). A read: it reports, and repairs
  /// nothing.
  ///
  /// **An associated function, not a method, and that is the design.** A
  /// method would require an opened facade, which is precisely the
  /// precondition doctor must not have: the first version went through
  /// [`Facade::open`], so a duplicate criterion id tripped a UNIQUE constraint
  /// during the DB load and the command died before it could report the thing
  /// it exists to report -- while the tool advised running `intent doctor`.
  /// The skin still calls only the facade (D06); the facade knows that this
  /// one verb has to work on a project nothing else can open.
  ///
  /// Reporting-only is likewise deliberate. A doctor that fixed what it found
  /// would change the thing it was measuring, and the operator would be
  /// reading a report about a state that no longer existed -- which is how
  /// `at lint --fix` came to half-migrate rows.
  /// Read an estate by parsing its MARKDOWN -- the v2 migrator's door, and the
  /// seam WP-10 plugs its frozen legacy parser into.
  ///
  /// **Associated rather than a method, and the reason is the whole point of
  /// the operation.** A method would need an opened facade, which needs canon
  /// this crate can read -- and an estate that has to be ingested from markdown
  /// is precisely one that has no such canon. Requiring a facade to migrate a
  /// project would mean requiring the project to already be migrated. Same
  /// shape as [`Facade::doctor`] above, for the same kind of reason: both run
  /// where the ordinary preconditions do not hold.
  ///
  /// **It is the DOOR rather than the implementation** -- one line today,
  /// because `ingest::from_md` refuses until WP-10 lands the parser. It
  /// exists now so the CLI has one entry point that does not move when the body
  /// arrives, and so the layer the parser plugs into is settled before there is
  /// a parser arguing for a different one.
  /// **`Unavailable` is mapped rather than wrapped, and that is not a
  /// nicety.** `FacadeError::Ingest` reads "could not read the committed
  /// canon", with the remedy "fix the artefacts named above, then retry" --
  /// true of every other ingest failure and false of this one, where nothing
  /// was read and nothing is wrong. An unbuilt feature reported as a damaged
  /// estate sends a user to repair files that are fine.
  pub fn ingest_from_md(project: &Project) -> Result<crate::legacy::Scan, FacadeError> {
    crate::legacy::scan(project).map_err(|source| {
      FacadeError::Ingest(IngestError::Io {
        path: project.relative(project.root()),
        source,
      })
    })
  }

  /// The canon extract's own directory.
  ///
  /// One expression, because two spellings of it would put the extract in the
  /// index's corpus on whichever side went stale -- and the extract is the
  /// store's prose seen twice.
  fn canon_dir(&self) -> std::path::PathBuf {
    self.project.canon_st_dir().parent().map_or_else(
      || self.project.root().join("intent").join(".canon"),
      std::path::Path::to_path_buf,
    )
  }

  /// Every path the store already carries prose for, sorted.
  ///
  /// The index excludes the store's own prose by ASKING THE TWO AUTHORITIES
  /// rather than by matching a path shape: `views::render_all` says which views
  /// exist, and canon's attachment rows say which authored documents the store
  /// carries. So a view kind added later, or a newly attached document, is
  /// excluded on the day it first exists and there is nothing to remember.
  ///
  /// **THE ATTACHMENTS ARE HERE BECAUSE A PROJECTION IS NOT THE WHOLE SET**
  /// (issue 0304). A thread's `design.md` is authored, so it is not a view, and
  /// the store carries it anyway -- which is the fact that decides whether the
  /// disk corpus should hold it. When it was only the views, one document
  /// realised on disk answered a search twice, once as `file` and once as
  /// `thread`.
  ///
  /// [`Facade::carried`] is this same answer filtered to one thread, and reads
  /// it from here rather than asking the two authorities a second time.
  fn carried_paths(&self) -> Result<Vec<std::path::PathBuf>, FacadeError> {
    let ctx = self.render_ctx()?;
    let mut paths: Vec<std::path::PathBuf> = views::render_all(&self.project, &self.canon, &ctx)
      .into_iter()
      .map(|v| v.path)
      .collect();
    for thread in &self.canon.threads {
      let home = self.project.thread_dir(&thread.id);
      for attachment in &thread.attachments {
        paths.push(home.join(&attachment.path));
      }
    }
    // **A `.history/` SNAPSHOT THE STORE CARRIES LEAVES THE DISK CORPUS**, by
    // the rule 0304 settled: a document held as sections in the store and
    // indexed again as a file is one document answering a search twice, from
    // two halves of one table, with nothing on either hit saying so.
    for file in self
      .store
      .section_files(crate::prose::WB_OWNER)
      .map_err(FacadeError::Store)?
    {
      paths.push(self.project.root().join(&file));
    }
    // **A BOARD'S EXTRACT IS CANON THAT LIVES OUTSIDE `.canon/`**, so the
    // directory rule does not reach it, and the store indexes the board from
    // its rows. Left in, one board answered a search twice: its section, and
    // `board.json` as a file.
    for board in &self.canon.boards {
      paths.push(self.project.board_json(&board.node.moniker));
    }
    paths.sort();
    paths.dedup();
    Ok(paths)
  }

  /// Walk the index's scope and record what it holds and what it does not.
  ///
  /// **THE SURVEY IS THE WHOLE WRITE, and it claims nothing about content.**
  /// Every row lands with `indexed_sha256` unset, because a walk stats files
  /// and does not read them; the hash is written by whatever reads the bytes.
  /// So this answers "what is in scope, and what will never be held, and why",
  /// which is what `intent index status` reports and what AC-18.2 requires.
  pub fn index_rebuild(&mut self) -> Result<crate::index::status::Status, FacadeError> {
    let carried = self.carried_paths()?;
    let rows = crate::index::reconcile::survey(
      self.project.root(),
      &carried,
      &self.canon_dir(),
      // **THE PROJECT'S CAP, NOT THE CONSTANT.** The default is a measurement
      // of one estate's own tree; a project whose documents are larger is not
      // wrong, and a cap it could not move would drop them with a row saying
      // `too-large` and no way to disagree.
      self.project.config().index.max_file_bytes,
    )
    // **THE SAME CARRIER `refresh_index` USES FOR THE SAME FAILURE**, two
    // functions over: a walk of the working tree failed at a path, and the
    // remedy for that is the one already written. A variant of its own would
    // owe a new remedy for a case that is not a new case.
    .map_err(|e| {
      FacadeError::Ingest(IngestError::Io {
        path: self.project.root().display().to_string(),
        source: std::io::Error::other(e.to_string()),
      })
    })?;
    // **AND THEN IT READS WHAT IT SAID IT WOULD HOLD.** The survey decides the
    // scope and the skips; this turns the bytes of everything left into rows.
    // Doing it in one door is deliberate: a rebuild that recorded a corpus and
    // indexed none of it would leave `index status` reporting files as held
    // when nothing could be found in them.
    let content = crate::index::reconcile::read_content(
      self.project.root(),
      &rows,
      &self.project.config().languages,
    );
    let mut rows = rows;
    // The mark goes in BEFORE the rows are written, so `index_file` and the
    // section tables land in one pass and the columns say what the rows beside
    // them were read from.
    crate::index::reconcile::mark_read(&mut rows, &content);
    self
      .store
      .replace_index_files(&rows)
      .map_err(FacadeError::Store)?;
    self
      .store
      .replace_file_sections(&content.prose)
      .map_err(FacadeError::Store)?;
    self
      .store
      .replace_src_sections(&content.source)
      .map_err(FacadeError::Store)?;
    self
      .store
      .replace_symbols_for(
        &rows.iter().map(|r| r.path.clone()).collect::<Vec<_>>(),
        &content.symbols,
      )
      .map_err(FacadeError::Store)?;
    // Issue 0369: a rebuild is a whole-scope reconcile, and stamps.
    self.store.stamp_reconciled().map_err(FacadeError::Store)?;
    let mut status = crate::index::status::summarise(&rows);
    status.grammars = crate::index::status::grammars(&self.project.config().languages);
    // Issue 0373: what the index costs, measured.
    status.sizes = self.store.index_sizes().map_err(FacadeError::Store)?;
    status.resolution = self
      .store
      .resolution(crate::index::symbols::EXTRACTOR_VERSION)
      .map_err(FacadeError::Store)?;
    Ok(status)
  }

  /// Bring the index up to date under the paths one batch named, touching
  /// nothing outside them.
  ///
  /// **`None` MEANS THE WHOLE SCOPE**, which is the daemonless query's case: it
  /// reconciles everything before it answers, and under the staleness policies
  /// that is a stat pass over source and a hash pass over canon rather than a
  /// re-read of the tree. Each path means its subtree, except the project root,
  /// which means depth one -- see `index::reconcile`'s `names`.
  ///
  /// **THIS IS THE DOOR A WATCHER CALLS, and it is not `index_rebuild` with a
  /// filter.** A rebuild is told the whole scope and deletes every row it was
  /// not told about; a refresh is told about a subtree, so it upserts what
  /// moved, deletes what has gone from under that path, and leaves the rest of
  /// the index alone. Reconciling one file through the rebuild door would
  /// unindex the project on every keystroke.
  ///
  /// **IT REPORTS PATHS RATHER THAN A COUNT**, because its caller is deciding
  /// what to publish to a subscriber, and a number is not something anyone can
  /// name.
  ///
  /// **THE PATHS ARE ONE SLICE BECAUSE THE WORK AHEAD OF THE FILTER IS PAID PER
  /// CALL.** What the store carries, the stored rows and the walk cost the same
  /// whatever the call names, so a batch handed over a path at a time paid them
  /// once per path.
  // Issue 0354.
  pub fn index_refresh(
    &mut self,
    under: Option<&[std::path::PathBuf]>,
  ) -> Result<crate::index::Refreshed, FacadeError> {
    let change = self.index_change(under)?;
    if change == crate::index::reconcile::Change::default() {
      // Issue 0369: a WHOLE-SCOPE reconcile that found nothing still ran, and
      // says so. A scoped one that found nothing writes nothing: the daemon's
      // watcher hands over `intent/` when the OS coalesces the store's own
      // writes, and a stamp there would be the next event (issues 0354, 0355).
      if under.is_none() {
        self.store.stamp_reconciled().map_err(FacadeError::Store)?;
      }
      return Ok(crate::index::Refreshed::default());
    }

    let content = crate::index::reconcile::read_content(
      self.project.root(),
      &change.upserts,
      &self.project.config().languages,
    );
    let mut upserts = change.upserts;
    crate::index::reconcile::mark_read(&mut upserts, &content);
    // **EVERY PATH THIS PASS TOUCHED, INCLUDING THE ONES THAT HAVE GONE.** A
    // removed file contributes no sections, so a call that took its scope from
    // the sections it was handed could never empty anything.
    let touched: Vec<String> = upserts
      .iter()
      .map(|r| r.path.clone())
      .chain(change.removed.iter().cloned())
      .collect();

    self
      .store
      .apply_index_changes(&upserts, &change.removed)
      .map_err(FacadeError::Store)?;
    self
      .store
      .replace_sections_for(&touched, &content.prose, &content.source)
      .map_err(FacadeError::Store)?;
    self
      .store
      .replace_symbols_for(&touched, &content.symbols)
      .map_err(FacadeError::Store)?;

    self.store.stamp_reconciled().map_err(FacadeError::Store)?;
    Ok(crate::index::Refreshed {
      updated: upserts.into_iter().map(|r| r.path).collect(),
      removed: change.removed,
    })
  }

  /// Bring the whole index up to date IN ORDER TO ANSWER a search.
  ///
  /// **The same work as [`Self::index_refresh`] and a different error**, which
  /// is the whole reason it exists (issue 0443). A reconcile that fails on a
  /// read verb is not a failed update from the reader's side -- it is a search
  /// that cannot be answered, and "could not update the runtime store" sends
  /// them to look at a write they did not ask for. Every search door that
  /// reconciles first calls this, so the sentence has one home.
  pub fn index_refresh_for_search(&mut self, query: &str) -> Result<(), FacadeError> {
    self.index_refresh(None).map(|_| ()).map_err(|e| match e {
      FacadeError::Store(cause) => FacadeError::SearchUnanswerable {
        query: query.to_string(),
        cause,
      },
      other => other,
    })
  }

  /// The paths whose refresh would bring the whole index up to date: the
  /// project root, for its own files, and each top-level directory holding a
  /// file the index has not caught up with. Empty when the index is current.
  ///
  /// **ONE SURVEY AND NO WRITES**, so a caller holding a current index pays a
  /// stat pass and nothing else. A caller that refreshes what this names gets
  /// the whole-scope reconcile `index_refresh(None)` performs, in pieces it
  /// can put other work between -- which is what a daemon opening a project
  /// needs, because its clients queue behind whatever its store thread is
  /// doing.
  // Issue 0366: the daemon indexed only what its watcher happened to name, so a
  // project opened over a quiet tree was indexed at the root and nowhere below.
  pub fn index_stale_roots(&self) -> Result<Vec<std::path::PathBuf>, FacadeError> {
    let change = self.index_change(None)?;
    let root = self.project.root();
    let mut roots: Vec<std::path::PathBuf> = change
      .upserts
      .iter()
      .map(|row| row.path.as_str())
      .chain(change.removed.iter().map(String::as_str))
      .map(|rel| match rel.split_once('/') {
        Some((top, _)) => root.join(top),
        None => root.to_path_buf(),
      })
      .collect();
    roots.sort();
    roots.dedup();
    Ok(roots)
  }

  /// What a reconcile under `under` would change, read against the index as it
  /// stands. `None` is the whole scope, as it is for [`Facade::index_refresh`].
  fn index_change(
    &self,
    under: Option<&[std::path::PathBuf]>,
  ) -> Result<crate::index::reconcile::Change, FacadeError> {
    let carried = self.carried_paths()?;
    let previous = self.store.index_files().map_err(FacadeError::Store)?;
    crate::index::reconcile::changed_under(
      self.project.root(),
      under,
      &previous,
      &carried,
      &self.canon_dir(),
      self.project.config().index.max_file_bytes,
    )
    .map_err(|e| {
      FacadeError::Ingest(IngestError::Io {
        path: match under {
          Some([one]) => one.display().to_string(),
          _ => self.project.root().display().to_string(),
        },
        source: std::io::Error::other(e.to_string()),
      })
    })
  }

  /// What the index holds and what it does not hold, READ FROM THE STORE.
  ///
  /// **IT DOES NOT WALK**, and the difference is the whole of the report's
  /// meaning: a status that surveyed the tree would describe the world, not
  /// the index, and would say a file was held on the run before anything held
  /// it. An empty answer is a store nobody has built an index in, which
  /// [`crate::index::status::Status::is_empty`] states in its own words.
  pub fn index_status(&self) -> Result<crate::index::status::Status, FacadeError> {
    let rows = self.store.index_files().map_err(FacadeError::Store)?;
    let mut status = crate::index::status::summarise(&rows);
    status.grammars = crate::index::status::grammars(&self.project.config().languages);
    // Issue 0373: what the index costs, measured.
    status.sizes = self.store.index_sizes().map_err(FacadeError::Store)?;
    status.resolution = self
      .store
      .resolution(crate::index::symbols::EXTRACTOR_VERSION)
      .map_err(FacadeError::Store)?;
    Ok(status)
  }

  /// Resolve references to the definitions they name with each language's own
  /// toolchain, and store what joins a written reference (ST0076 WP-05, vc
  /// decision 25).
  ///
  /// **AN EXPLICIT VERB AND NEVER A RECONCILE.** A toolchain runs the project's
  /// own code -- rust-analyzer's export runs build scripts and proc macros, and
  /// nothing turns that off (measured 2026-09-17) -- so nothing calls this
  /// unasked: not intentd, not a reconcile, not a hook, and not the MCP tool
  /// tier.
  ///
  /// **ONE LANGUAGE'S FAILURE IS ITS OWN.** Each language is stored or
  /// recorded as unresolved on its own, and the answer carries every language
  /// the call covered, so a missing tool for one never hides another's result.
  ///
  /// `readers` is what this build can run: the CLI passes
  /// [`crate::index::resolved::readers`], and a test passes its own. `lang`
  /// names one language; `None` runs every reader for a language the project
  /// declares, and skips, by name, one whose project holds nothing for its
  /// tool.
  pub fn index_resolve(
    &mut self,
    lang: Option<&str>,
    full: bool,
    readers: &[Box<dyn crate::index::resolved::Resolver>],
  ) -> Result<crate::index::resolved::Outcome, FacadeError> {
    use crate::index::resolved::{self, Unresolved};
    let extractor = crate::index::symbols::EXTRACTOR_VERSION;
    let declared = &self.project.config().languages;
    let chosen: Vec<&dyn resolved::Resolver> = readers
      .iter()
      .map(Box::as_ref)
      .filter(|reader| match lang {
        Some(asked) => reader.lang() == asked,
        None => declared.iter().any(|d| d == reader.lang()),
      })
      .collect();
    if chosen.is_empty() {
      let quoted = |langs: Vec<&str>| {
        langs
          .iter()
          .map(|l| format!("`{l}`"))
          .collect::<Vec<_>>()
          .join(", ")
      };
      return Err(FacadeError::NoResolver {
        asked: match lang {
          Some(asked) => format!("`{asked}`"),
          None if declared.is_empty() => {
            "any language, because this project declares none".to_string()
          }
          None => format!(
            "any language this project declares ({})",
            quoted(declared.iter().map(String::as_str).collect())
          ),
        },
        built: quoted(readers.iter().map(|r| r.lang()).collect()),
      });
    }

    let mut outcome = resolved::Outcome::default();
    for reader in &chosen {
      let previous = self
        .store
        .resolution(extractor)
        .map_err(FacadeError::Store)?
        .remove(reader.lang());
      let full = full || resolved::must_run_full(previous.as_ref(), extractor);
      // **A BUILD DIRECTORY THAT CANNOT BE MADE FAILS THIS LANGUAGE'S RUN**,
      // recorded like any other failure, rather than refusing the verb with an
      // I/O error that names no language and leaves every other one unrun.
      let cache = self.project.resolve_cache_dir(reader.lang());
      if let Err(e) = std::fs::create_dir_all(&cache) {
        let why = Unresolved::Failed {
          path: None,
          line: None,
          detail: format!(
            "Intent could not create its build directory {}: {e}",
            cache.display()
          ),
        };
        self
          .store
          .record_unresolved(reader.lang(), reader.tool(), &why)
          .map_err(FacadeError::Store)?;
        continue;
      }

      let indexed: Vec<String> = self
        .store
        .index_files()
        .map_err(FacadeError::Store)?
        .into_iter()
        .filter(|row| row.skipped_reason.is_none())
        .map(|row| row.path)
        .collect();
      let scope = resolved::Scope {
        root: self.project.root(),
        cache: &cache,
        full,
        indexed: &indexed,
      };
      let traced = reader.trace(&scope).and_then(|trace| {
        let undeclared = resolved::undeclared(&trace, reader.excludes())
          .iter()
          .map(|r| format!("`{r}`"))
          .collect::<Vec<_>>();
        if undeclared.is_empty() {
          Ok(trace)
        } else {
          Err(Unresolved::Failed {
            path: None,
            line: None,
            detail: format!(
              "the {} reader excluded references for {}, which it does not declare",
              reader.tool(),
              undeclared.join(", ")
            ),
          })
        }
      });
      // **THE INDEX CATCHES UP AFTER THE TOOL HAS READ, AND NEVER BEFORE.** The
      // join keeps only files whose indexed bytes are the bytes the tool read.
      // Refreshed first, a file saved between the refresh and the tool's read
      // would never join, and an incremental tool would never retrace it, since
      // its own state already holds those bytes. Refreshed after, an edit
      // before the tool's read joins, and an edit after it is dropped as moved
      // and retraced next run, because the disk and the tool's state disagree.
      // A later tidy-up that moves this refresh up reopens that hole.
      //
      // **AND A REFRESH THAT FAILS FAILS THE RUN** (dc, on the reader
      // contract): joined against an index that did not catch up, an edit made
      // before the tool's read would drop as moved and never be retraced.
      let traced = traced.and_then(|trace| match self.index_refresh(None) {
        Ok(_) => Ok(trace),
        Err(e) => Err(Unresolved::Failed {
          path: None,
          line: None,
          detail: format!(
            "the index could not catch up with the files {} read: {e}",
            reader.tool()
          ),
        }),
      });

      match traced {
        Ok(trace) => {
          let (indexed, written) = self
            .store
            .resolution_basis(reader.lang())
            .map_err(FacadeError::Store)?;
          let defs = self
            .store
            .definitions(reader.lang())
            .map_err(FacadeError::Store)?;
          let located = resolved::locations(&defs, *reader);
          let joined = resolved::join(&trace, &indexed, &written, &located);
          self
            .store
            .replace_resolved(reader.lang(), reader.tool(), extractor, &joined)
            .map_err(FacadeError::Store)?;
        }
        Err(Unresolved::NotApplicable { detail }) if lang.is_none() => {
          outcome
            .not_applicable
            .insert(reader.lang().to_string(), detail);
        }
        Err(why) => self
          .store
          .record_unresolved(reader.lang(), reader.tool(), &why)
          .map_err(FacadeError::Store)?,
      }
    }

    outcome.resolution = self
      .store
      .resolution(extractor)
      .map_err(FacadeError::Store)?;
    outcome.resolution.retain(|l, _| {
      chosen.iter().any(|reader| reader.lang() == l) && !outcome.not_applicable.contains_key(l)
    });
    Ok(outcome)
  }

  pub fn doctor(
    project: &Project,
    ctx: &FacadeContext,
    store: Option<&crate::store::Store>,
    scope: crate::doctor::Scope,
  ) -> crate::doctor::Report {
    // **This used to read the event log, from the store or from the extract,
    // and D44 took away its only reason to.** The watermark was the one thing
    // a render needed that lived in the log, so sourcing it from both places
    // **THE CUTOFF COMES FROM THE COMMITTED FILE, WHICH IS WHY `doctor` CAN
    // ANSWER AT ALL ON A MACHINE WITH NO STORE** -- the normal state of a fresh
    // clone, and the moment someone reaches for this command.
    //
    // `doctor` re-renders every view to detect a hand-edited one, so it must
    // compute the SAME cutoff the writer used. It used to read the event log,
    // which D53 removed from the working tree: with no store there was no
    // cutoff, so this re-render put every completed thread back in DONE and
    // reported `todo.md` as hand-edited on every project that had ever flushed,
    // permanently, with nothing wrong.
    //
    // **A DAMAGED FILE READS AS ABSENT HERE, and only here.** `doctor` is the
    // command you run when the estate is broken; refusing to diagnose because
    // one canon file will not parse would withhold the report naming that very
    // file. The finding still comes out of the checks below.
    let todo_watermark = crate::ingest::read_project_state(project)
      .ok()
      .flatten()
      .and_then(|state| state.todo_watermark);
    crate::doctor::diagnose(
      project,
      &RenderContext {
        version: &ctx.version,
        todo_watermark,
      },
      store,
      scope,
    )
  }

  /// AC-05.1: **the path to open for an addressed artefact, realised first so
  /// that the path EXISTS when it is printed.**
  ///
  /// **IT DELEGATES TO [`Facade::hydrate`] RATHER THAN REALISING ANYTHING
  /// ITSELF**, which is AC-05.3 in one line: path-printing has ONE home, and
  /// this is `st edit`'s behaviour learning to hydrate first rather than a
  /// second implementation of it. Every refusal `hydrate` makes -- a foreign
  /// authority, a non-artefact entity -- is inherited whole and none is
  /// restated here.
  ///
  /// **THE EXISTENCE CHECK IS MEMBERSHIP IN WHAT `hydrate` RETURNED, NOT A
  /// `Path::exists`.** `hydrate` documents its return as the paths that NOW
  /// EXIST, so asking the filesystem again would be a second answer to a
  /// question already answered -- and a worse one, because it could not say
  /// what IS there when the answer is no.
  ///
  /// **A GENERATED VIEW IS REFUSED AND THE REFUSAL CARRIES THE DESTINATION**
  /// (hv, 2026-08-19). The disposition comes from [`Project::edit_disposition`]
  /// so there is no second answer to what a file is.
  pub fn edit(&mut self, address: &Address, file: &str) -> Result<std::path::PathBuf, FacadeError> {
    // **ANOTHER PROJECT'S ADDRESS IS REFUSED BEFORE ANY CHECK BELOW** (issue
    // 0338 (i)): each of them answers about THIS project, and `edit` told
    // `intent://other/threads/ST0009` that this project has no ST0009 -- a true
    // answer to a question nobody asked.
    require_local(address)?;
    let rel = std::path::PathBuf::from(format!("{file}.md"));

    // **THIS REFUSAL IS DECIDED BEFORE ANYTHING IS WRITTEN, AND THE ORDER IS
    // THE POINT.** It used to sit BELOW `hydrate`, so the default invocation --
    // `intent st edit ST0001`, where `file` defaults to `info`, the one file
    // this verb refuses -- realised the thread's views and appended
    // `STEELTHREAD:<id>` to the TRACKED `.intentfiles`, and THEN exited 1.
    // Driven at `21ea0e8f` in a disposable project: two files created, one line
    // appended, rc=1. Reported by ic; the one affected project is this one.
    //
    // **THE EXIT CODE AND THE EFFECT DISAGREED, WHICH IS THE ARM
    // IN-AG-NO-SILENT-001 NEVER NAMES.** The rule is written against a
    // swallowed error; here the error is surfaced correctly and the EFFECT is
    // hidden. A verb that hydrates and says so is fine; a verb that reports it
    // did nothing and appends to a tracked file is not.
    //
    // **THIS SCOPES THE NO-ROLLBACK RULING RATHER THAN OVERTURNING IT** (vc,
    // 2026-08-22): a refusal must not roll back a completed act, and **a ruling
    // about rollback cannot reach an act that was never performed.** Its test
    // moved onto the `NoSuchEditable` arm then, and was deleted with 0145, when
    // that refusal moved ahead of `hydrate` too and left it no act to govern.
    //
    // `Project::edit_disposition` is a pure function of the FILENAME: it
    // consults no disk and no store, so nothing was ever gained by deciding it
    // late. A non-artefact address falls through deliberately -- `hydrate`
    // owns that refusal and makes it before its own first write.
    // **THE ID OUTRANKS THE FILENAME, AND BOTH CHECKS ARE PURE READS SO PUTTING
    // THE ID FIRST COSTS NOTHING.** Without this the filename check answered
    // first and described a file inside a thread that was never there: `st edit
    // ST9997 info` said `info.md is generated from the model` and `st edit
    // ST9997 design` said `is not a file this artefact carries`. **The file
    // argument decided which wrong story the operator got, and neither could
    // say the id was unknown** -- so a typo was the one fault this verb could
    // not diagnose (`intent#0144`).
    //
    // `hydrate` resolves again below and that is deliberate rather than
    // redundant: it is a public door in its own right, and a rule that only
    // holds when you arrive through `edit` is not a rule. The resolution lives
    // once, in `st_show`; this is a second CALLER, not a second answer.
    // **AN ISSUE IS REFUSED HERE, BY NAME, AND THE REFUSAL IS THE HONEST
    // ANSWER RATHER THAN A GAP.** `edit` resolves a file THE ARTEFACT CARRIES
    // -- `design.md`, an attachment -- and every line below it reaches for
    // `thread_dir` and `carried`. An issue carries no authored files: the one
    // file it has is the generated view, which `edit` refuses for a thread too
    // (`EditDisposition::Refuse`). So there is nothing here for an issue to
    // name, and falling through would ask the store for a THREAD `0021`.
    //
    // **WHAT THIS DELIBERATELY DOES NOT DO IS INVENT `intent edit <issue>`.**
    // ST0069 AC-01.1 to AC-01.3 give an issue a realised form and put it under
    // `organize`; none of them says an issue is editable through this door,
    // and building it here would be scope this package was not given.
    if let Some((Sigil::Issue, id)) = address.entity.artefact() {
      return Err(FacadeError::NotHydratable {
        form: address.entity.form(),
        why: format!(
          "issue {id}'s only file is its generated view, which is rendered from the store rather than authored -- `intent issues edit {id}` corrects the record it is rendered from"
        ),
      });
    }
    if let Some((_, id)) = address.entity.artefact() {
      self.st_show(id)?;

      // **AND THE ARTEFACT IS NOT THE ENTITY, WHICH IS THE WHOLE OF `0238`.**
      // `Entity::artefact()` collapses SIX variants onto their THREAD -- a
      // `Wp`, an `Ac`, an `At`, an `Attachment` and both collections all answer
      // with the thread that carries them -- so the line above verifies the
      // THREAD and never the thing the operator actually named.
      //
      // **THE HOLE IS IN THIS DOOR, NOT IN ONE ARM.** `--path`, `--editor` and
      // the bare spelling all arrive here, and all three printed the thread's
      // `info.md` at rc=0 for `wp ST0056/99`. It was reported as a `--path`
      // defect; `--path` is just where someone happened to be standing.
      //
      // **THE ANSWER WAS INVARIANT OVER THE ID, WHICH IS WHY NOTHING CAUGHT
      // IT.** A real WP-01 and an absent WP-99 printed the SAME BYTES -- so no
      // comparison of outputs could ever have separated them, and refusing
      // before answering is the only available fix rather than the chosen one.
      // **THE PATH WAS WRONG TOO, AND THIS COMMENT SAID IT WAS NOT** (issue
      // 0291). It read *`artefact()` rules that a work package has no files of
      // its own*. `artefact()` rules which ARTEFACT is realised, which is the
      // thread; the work package's view is `WP/<NN>/info.md` inside it, and it
      // is on disk. So the file is resolved under the WP's own directory below.
      //
      // **`Ac` AND `At` ARE CHECKED HERE TOO, BEFORE ANYTHING IS WRITTEN**
      // (issue 0240). They reach this door through the `intent://` address
      // grammar, which bypasses the `kind` enum that bounds the `<kind> <id>`
      // spelling, so bounding the surface by that enum misses them. Driven:
      // `edit intent:///threads/ST0001/ac/1 --path` printed the thread's
      // `info.md` at rc=0 for a criterion that does not exist. `Attachment` is
      // checked by the membership test below instead, because its path IS the
      // file it names.
      //
      // **THE TWO COLLECTION VARIANTS ARE DELIBERATELY ABSENT.** The `ac` or
      // `wp` collection of a thread that exists also exists, empty or not, so
      // `st_show` above is already the right and complete check for them.
      match &address.entity {
        crate::address::Entity::Wp { thread, wp } => {
          self.wp_of(thread, wp)?;
        }
        crate::address::Entity::Ac { thread, ac } => {
          self.criterion(thread, ac)?;
        }
        crate::address::Entity::At { thread, at } => {
          self.acceptance_test(thread, at)?;
        }
        _ => {}
      }
    }

    // **A WORK PACKAGE'S FILE IS UNDER ITS OWN DIRECTORY, THREAD-RELATIVE**
    // (issue 0291). `edit wp ST0064/01 --path` resolved and validated the WP,
    // then printed the THREAD's `info.md` at rc=0 while `WP/01/info.md` was on
    // disk -- a caller editing the result edited the thread. The directory
    // comes from `Project::wp_info_view`, the one spelling of that layout, and
    // everything below -- the disposition, the refusal's path, the membership
    // check -- already reads `rel` against the thread directory.
    let rel = match &address.entity {
      crate::address::Entity::Wp { thread, wp } => {
        let seq = self.wp_of(thread, wp)?.seq;
        let view = self.project.wp_info_view(thread, seq);
        // Never a fallback to the thread-relative `rel`: that fallback IS the
        // defect, so a layout that broke this would have to fail loudly.
        #[allow(
          clippy::expect_used,
          reason = "INVARIANT: wp_info_view is under thread_dir by construction, and falling back to the thread-relative path is the defect"
        )]
        let under = view
          .parent()
          .and_then(|dir| dir.strip_prefix(self.project.thread_dir(thread)).ok())
          .expect("wp_info_view is under thread_dir by construction");
        under.join(&rel)
      }
      // **AN ATTACHMENT ADDRESS NAMES ITS OWN FILE** (issue 0240), already
      // thread-relative, as `Attachment.path` is. It printed the thread's
      // `info.md` at rc=0 for `attachments/notes.md`, which was never created.
      // Resolved here, an absent one fails the membership check below, which
      // names what the thread does carry.
      crate::address::Entity::Attachment { path, .. } => std::path::PathBuf::from(path),
      // **A CRITERION AND A TEST ROW RENDER INTO THE THREAD'S `acceptance.md`, SO
      // THAT IS THE FILE THEIR ADDRESS NAMES** (issue 0334). `edit
      // intent:///threads/ST0001/ac/AC-01.1 --path` printed the thread's `info.md`
      // at rc=0, a file the row is not in. Resolved here, the disposition below
      // refuses the generated view and names `intent ac` and `intent at` -- the
      // answer `edit st <id> acceptance` already gets -- so no spelling prints a
      // view the next render overwrites.
      crate::address::Entity::Ac { .. } | crate::address::Entity::At { .. } => {
        std::path::PathBuf::from(crate::views::ThreadView::Acceptance.file())
      }
      _ => rel,
    };

    if let Some((_, id)) = address.entity.artefact()
      && let crate::project::EditDisposition::Refuse { author_with } =
        Project::edit_disposition(&rel)
    {
      return Err(FacadeError::NotEditable {
        path: self
          .project
          .relative(&self.project.thread_dir(id).join(&rel)),
        author_with,
      });
    }

    // **MEMBERSHIP IS DECIDED BEFORE `hydrate` TOO, ON THE SAME GROUNDS AS THE
    // FILENAME ABOVE** (0145). It sat below `hydrate`, so `st edit ST0001 impl`
    // on a known thread realised its views and grew the TRACKED `.intentfiles`,
    // then exited 1 -- and the remedy was built from what that realisation
    // happened to write, so on a thread with nothing to write it named nothing.
    // `carried` is a pure read of the model, so the refusal names what the
    // thread carries whether or not any of it is on disk.
    if let Some((_, id)) = address.entity.artefact() {
      let carried = self.carried(id)?;
      let wanted = self.project.thread_dir(id).join(&rel);
      if !carried.contains(&wanted) {
        return Err(FacadeError::NoSuchEditable {
          path: self.project.relative(&wanted),
          present: self.carried_names(id, &carried),
        });
      }
    }

    let realised = self.hydrate(address)?;
    // `hydrate` refuses every non-artefact form before this point, so the
    // address is known to name one.
    let (_, id) = address
      .entity
      .artefact()
      .ok_or_else(|| FacadeError::NotHydratable {
        form: address.entity.form(),
        why: "the address names no artefact, so there is no file of it to open".to_string(),
      })?;

    let wanted = self.project.thread_dir(id).join(&rel);
    if !realised.contains(&wanted) {
      return Err(FacadeError::NoSuchEditable {
        path: self.project.relative(&wanted),
        present: self.carried_names(id, &realised),
      });
    }
    Ok(wanted)
  }

  /// The remedy's list for [`FacadeError::NoSuchEditable`].
  ///
  /// **WHAT IS THERE, NOT MERELY THAT THIS IS NOT.** The operator asked for a
  /// file this artefact does not carry, and the set that answers the follow-up
  /// question is the one already in hand.
  ///
  /// **THREAD-RELATIVE AND DEDUPED, BECAUSE THE BASENAME IS NOT THE ANSWER.**
  /// Taking `file_name()` printed `info.md` ten times on a thread with nine
  /// work packages -- every `WP/<NN>/info.md` collapsing onto the cover's name.
  /// A remedy that repeats one word ten times is read as a rendering fault and
  /// stops being read at all, and it also told the operator that `info` was
  /// available when `info` is the one thing this verb refuses.
  fn carried_names(&self, id: &str, paths: &[std::path::PathBuf]) -> Vec<String> {
    let dir = self.project.thread_dir(id);
    let mut names: Vec<String> = paths
      .iter()
      .filter_map(|p| p.strip_prefix(&dir).ok())
      .map(|p| p.to_string_lossy().into_owned())
      .collect();
    names.sort();
    names.dedup();
    names
  }

  /// Run the close gate. A read: it changes nothing and refuses nothing.
  pub fn gate(&self, st: &str, scope: Scope) -> Result<Verdict, FacadeError> {
    Ok(contract::gate(
      self.st_show(st)?,
      scope,
      &contract::RepoFiles(self.project.root()),
    ))
  }

  // -------------------------------------------------------------------------
  // Steel-thread lifecycle
  // -------------------------------------------------------------------------

  /// Create a thread. The id is the next free `ST<nnnn>`.
  ///
  /// **Entry is `Triage`, ratified** -- it used to be `NotStarted`. Every
  /// thread is now triaged rather than assumed wanted, and `st triage` is the
  /// verb that accepts it into the backlog.
  pub fn st_new(&mut self, title: &str) -> Result<String, FacadeError> {
    self.st_new_listing(title, ListEdit::AsDeclared)
  }

  /// Create a thread, saying whether it is also listed in `.intentfiles`.
  ///
  /// `st new --dehydrate` passes [`ListEdit::Suppressed`]; everything else
  /// reaches this through [`Facade::st_new`]. **A wrapper for the same reason
  /// [`Facade::st_done_listing`] is one** -- seventeen call sites across nine
  /// shared test files, and nothing to say in any of them.
  ///
  /// **WHAT `--dehydrate` DOES NOT DO, AND ITS HELP TEXT CLAIMS IT DOES:**
  /// suppress the FILES. `Facade::apply` projects every changed thread
  /// unconditionally and consults no manifest, so the views are written either
  /// way and the next `organize` is what removes them. The flag's real and only
  /// effect is on the list. Reported rather than worked around: filtering
  /// `apply` by the manifest is a change to the core write path, not to this
  /// verb.
  /// [`Facade::st_new_listing`] WITH what its projection had to say.
  ///
  /// **A CREATE PROJECTS THE ESTATE EXACTLY AS A TRANSITION DOES**, so it can
  /// overwrite a generated view somebody had edited -- and it returned an id,
  /// which is not a channel anything can be said through. Measured before this
  /// existed: `st new` overwrote a hand-edited `steel_threads.md` and printed
  /// `created: ST0002` and nothing else.
  ///
  /// **THE SIBLING RATHER THAN A CHANGED RETURN TYPE, and the reason is the
  /// caller set.** `st_new_listing` has callers across three crates and most of
  /// them want the id and nothing more; widening its return would edit all of
  /// them to say `.0`, which is churn that hides the one call site that
  /// actually changed. This is additive: the face that reports takes this one,
  /// and every other caller stays exactly as it was.
  pub fn st_new_listing_reported(
    &mut self,
    title: &str,
    list: ListEdit,
  ) -> Result<(String, Vec<Note>), FacadeError> {
    self.st_new_inner(title, list)
  }

  pub fn st_new_listing(&mut self, title: &str, list: ListEdit) -> Result<String, FacadeError> {
    self.st_new_inner(title, list).map(|(id, _)| id)
  }

  fn st_new_inner(
    &mut self,
    title: &str,
    list: ListEdit,
  ) -> Result<(String, Vec<Note>), FacadeError> {
    // **THE PRE-CHECK THAT USED TO BE HERE COULD NOT FIRE, AND IT IS GONE**
    // (issue 0131). It asked whether `self.canon` already held the id that
    // `next_thread_id()` had just computed as `max() + 1` over that same canon
    // -- false by construction, on every call, for the whole life of the verb.
    // `error_remedies.rs` had already recorded the symptom without the cause,
    // exempting `ThreadExists` from its drive as "needs a colliding id, which
    // `st new` allocates around".
    //
    // The condition it was reaching for is real and it is what the ruling is
    // about: a SECOND facade, opened before this one wrote, computes the same
    // id. No check against an in-memory canon can see that, because the canon
    // is the stale read. `ThreadExists` is now raised where the collision is
    // actually detectable -- by the UNIQUE constraint, inside the transaction
    // -- and is reachable for the first time.
    let id = self.next_thread_id();
    let thread = Thread {
      // A thread created here has no files beside it yet; the walk that finds
      // them runs at ingest, not at creation.
      attachments: Vec::new(),
      // A thread created by v3 has no authored sections beyond the two the
      // model names; anything else arrives when a human writes it. The
      // preamble is the same case: v2 estates carry one, a thread this tool
      // creates does not until somebody writes above the first heading.
      body: String::new(),
      preamble: String::new(),
      schema: crate::model::THREAD_SCHEMA.to_string(),
      id: id.clone(),
      title: title.to_string(),
      slug: Some(slugify(title)),
      status: ThreadStatus::Triage,
      status_reason: None,
      // A creation path: only `st.fc` writes this, so `None` is the fact and
      // not a placeholder. A brand-new thread has not been closed at all.
      fiat: None,
      // **EMPTY, AND THAT IS THE POINT** (D42). Nothing here knows what day it
      // is, and nothing needs to: the store fills this inside the INSERT and
      // hands back what it wrote. Same idiom as `Envelope::minted`, which mints
      // an event with no `ts` for the same reason.
      created: String::new(),
      completed: None,
      acceptance: None,
      objective: String::new(),
      context: String::new(),
      related: Vec::new(),
      wps: Vec::new(),
      criteria: Vec::new(),
      tests: Vec::new(),
    };
    let mut next = self.canon.clone();
    next.threads.push(thread);
    let foreign = self.apply(
      "st.new",
      Subject {
        kind: "thread".to_string(),
        id: id.clone(),
      },
      json!({"title": title}),
      next,
    )?;
    self.edit_list("st.new", &id, list)?;
    // The same fold every transition uses, so a create cannot report its
    // overwrites in a second spelling.
    let notes = Outcome::Moved.with_overwrites(foreign).notes().to_vec();
    Ok((id, notes))
  }

  /// Accept a thread out of triage and into the backlog.
  pub fn st_triage(&mut self, id: &str) -> Result<Outcome, FacadeError> {
    self.set_thread_status(
      id,
      ThreadStatus::NotStarted,
      "st.triage",
      None,
      ListEdit::AsDeclared,
      None,
    )
  }

  pub fn st_start(&mut self, id: &str) -> Result<Outcome, FacadeError> {
    self.set_thread_status(
      id,
      ThreadStatus::Wip,
      "st.start",
      None,
      ListEdit::AsDeclared,
      None,
    )
  }

  /// Pause a thread, recording why.
  ///
  /// **`Hold` was in the vocabulary for two major versions with no verb that
  /// set it** -- v2 recognised it in its status filter and reached it only by
  /// hand-editing frontmatter, which is the defect class hv ruled on, sitting
  /// in the tool's own status enum.
  pub fn st_hold(&mut self, id: &str, reason: &str) -> Result<Outcome, FacadeError> {
    self.set_thread_status(
      id,
      ThreadStatus::Hold,
      "st.hold",
      Some(reason),
      ListEdit::AsDeclared,
      None,
    )
  }

  /// Resume a held thread. **The hold reason is cleared**, because it described
  /// a condition that has ended -- see [`Thread::status_reason`].
  ///
  /// [`Thread::status_reason`]: crate::model::Thread::status_reason
  pub fn st_resume(&mut self, id: &str) -> Result<Outcome, FacadeError> {
    self.set_thread_status(
      id,
      ThreadStatus::Wip,
      "st.resume",
      // **`st resume` SPENDS THE REASON, AND THIS IS A RATIFIED BEHAVIOUR RATHER
      // THAN THE DEFAULT IT USED TO RIDE ON.** `mutation_completeness.rs:2324`
      // asserts it and states why: *a resumed thread must not still be
      // explaining why it was paused -- the reason belongs to the state it was
      // given for.* **THAT TEST COVERS EXACTLY THIS ONE EDGE**, `st hold` ->
      // `st resume`, and says nothing about the other six verbs that were
      // clearing the field on their way past.
      //
      // So the ruled shape does not reverse it, it makes it VISIBLE: `Some("")`
      // is this call site declaring that the reason is spent, where `None` was
      // the shared tail deciding it for seven verbs at once. **The behaviour is
      // unchanged and the silence is gone**, which is the whole of what limb 2
      // asks for.
      Some(""),
      ListEdit::AsDeclared,
      None,
    )
  }

  /// Close a thread. Consults the close gate first -- the single authority, so
  /// there is no path that closes without it.
  /// Close a thread. The gate is a DECLARED guard and is run by the shared
  /// setter, after the self-loop test -- see `Facade::check_gate`.
  pub fn st_done(&mut self, id: &str) -> Result<Outcome, FacadeError> {
    self.st_done_listing(id, ListEdit::AsDeclared, None)
  }

  /// `st done --keep`: close the thread and LEAVE its `.intentfiles` entry, so
  /// its files stay on disk (AC-05.2).
  ///
  /// **A WRAPPER RATHER THAN A PARAMETER ON [`Facade::st_done`], AND THE
  /// REASON IS THE SHARED CHECKOUT RATHER THAN TASTE.** Threading the argument
  /// through would have rewritten thirteen call sites across seven test files
  /// that four sessions edit concurrently -- a mechanical diff whose only
  /// content is `ListEdit::AsDeclared`, in exactly the files a peer is most
  /// likely to be holding. **The delegation is one line and there is one
  /// implementation**, so this is a second door and never a second answer.
  pub fn st_done_listing(
    &mut self,
    id: &str,
    list: ListEdit,
    on: Option<&str>,
  ) -> Result<Outcome, FacadeError> {
    self.set_thread_status(id, ThreadStatus::Completed, "st.done", None, list, on)
  }

  /// `intent fc <ST>`: close the thread on human authority against the evidence,
  /// recording who, when and why (hv, D1, 2026-08-29).
  ///
  /// **THE GUARD IS `ReasonRecorded` AND NEVER `GatePass`**, which is the whole
  /// of the ruling in one line: this verb exists for the case where the gate does
  /// NOT pass, so gating it would make it unreachable exactly when it is needed.
  /// The gate is a no-op here rather than skipped -- `check_gate` is keyed on the
  /// DECLARED guard, so the edge declaring `ReasonRecorded` is what makes the
  /// gate not run, and no second place holds that decision.
  ///
  /// **The status lands on `completed`, the same value `st.done` reaches**, and
  /// the record beside it on [`Thread::fiat`] is the only thing that
  /// distinguishes them. That is hv's 2026-08-28 shape, and it is why this is an
  /// edge rather than a status variant.
  pub fn st_fc(&mut self, id: &str, because: &str, by: &str) -> Result<Outcome, FacadeError> {
    let standing = self.st_show(id)?.fiat.as_ref().map(|r| r.because.clone());

    // **NOT AN `unwrap`, AND NOT UNREACHABLE**, for the reason `at_fc` records:
    // `check_reason` returns `None` when the verb declares no `ReasonRecorded`
    // guard, so this arm is what happens if that declaration is ever dropped
    // from the table -- and the consequence would be a `FiatRecord` stored with
    // an empty `because`. Refusing is the honest answer to a table and a verb
    // that disagree.
    let because = Self::check_reason("Thread", "status", "st.fc", Some(because))?
      .ok_or(FacadeError::ReasonRequired { verb: "st.fc" })?;

    let record = crate::model::FiatRecord {
      because: because.clone(),
      by: by.to_string(),
      // D42: no function here takes a time. The stamp is applied BY the write.
      at: String::new(),
      invoker: crate::model::Invoker::collected(),
      inherited_from: None,
      inherited_event: None,
    };
    self
      .set_thread_status_fiat(
        id,
        ThreadStatus::Completed,
        "st.fc",
        Some(&because),
        ListEdit::AsDeclared,
        None,
        Some(record),
      )
      .map_err(|cause| match (cause, standing) {
        // **The standing reason is what the operator needs before replacing one
        // human judgement with another**, so it is carried rather than summarised.
        (FacadeError::IllegalTransition { .. }, Some(because)) => FacadeError::AlreadyFiatClosed {
          subject: id.to_string(),
          because,
          undo: format!("intent st reopen {id} --reason <why>"),
        },
        (other, _) => other,
      })
  }

  /// `intent fc <ST>/<NN>`: close the work package on human authority against the
  /// evidence. Same ruling, same guard and same reasoning as [`Facade::st_fc`].
  pub fn wp_fc(
    &mut self,
    st: &str,
    seq: u32,
    because: &str,
    by: &str,
  ) -> Result<Outcome, FacadeError> {
    let standing = self
      .st_show(st)?
      .wps
      .iter()
      .find(|w| w.seq == seq)
      .and_then(|w| w.fiat.as_ref().map(|r| r.because.clone()));

    let because = Self::check_reason("WorkPackage", "status", "wp.fc", Some(because))?
      .ok_or(FacadeError::ReasonRequired { verb: "wp.fc" })?;

    let record = crate::model::FiatRecord {
      because: because.clone(),
      by: by.to_string(),
      at: String::new(),
      invoker: crate::model::Invoker::collected(),
      inherited_from: None,
      inherited_event: None,
    };
    self
      .set_wp_status_fiat(
        st,
        seq,
        WpStatus::Done,
        "wp.fc",
        Some(&because),
        Some(record),
      )
      .map_err(|cause| match (cause, standing) {
        (FacadeError::IllegalTransition { .. }, Some(because)) => FacadeError::AlreadyFiatClosed {
          subject: format!("{st}/{seq:02}"),
          because,
          undo: format!("intent wp reopen {st}/{seq:02} --reason <why>"),
        },
        (other, _) => other,
      })
  }

  /// Reopen a completed thread.
  ///
  /// **The ratified machines have no terminal states**, and this is one of the
  /// two exits that makes that true. A thread whose contract grows after it
  /// closed was previously repairable only by editing the file the CLI exists
  /// to own -- and the gate then kept saying PASS against a contract that had
  /// moved underneath it.
  pub fn st_reopen(&mut self, id: &str, reason: &str) -> Result<Outcome, FacadeError> {
    self.set_thread_status(
      id,
      ThreadStatus::Wip,
      "st.reopen",
      Some(reason),
      ListEdit::AsDeclared,
      None,
    )
  }

  /// Bring a cancelled thread back, to the backlog rather than to where it was.
  ///
  /// It lands on `not-started` deliberately: a thread that was cancelled mid-
  /// flight has had its work overtaken, and resuming it as `wip` would assert
  /// a continuity nobody checked.
  pub fn st_reinstate(&mut self, id: &str, reason: &str) -> Result<Outcome, FacadeError> {
    self.set_thread_status(
      id,
      ThreadStatus::NotStarted,
      "st.reinstate",
      Some(reason),
      ListEdit::AsDeclared,
      None,
    )
  }

  /// `intent st renumber <old> <new>` (ST0078 WP-02, AC-02.1): move a thread
  /// to a free id, with everything that names it structurally.
  ///
  /// **THE REPAIR FOR TWO CLONES THAT MINTED ONE ID.** Git refuses that merge
  /// loudly, so the collision stays git's to catch; this is what the person on
  /// the losing side runs before merging again.
  ///
  /// **`new` IS TAKEN IF THE STORE OR THE TREE HOLDS IT.** A canon file or a
  /// directory the store does not know is most often a pull not yet loaded,
  /// and renumbering onto it would put two threads in one place.
  pub fn st_renumber(&mut self, old: &str, new: &str) -> Result<Renumbering, FacadeError> {
    self.st_show(old)?;
    let subject = format!("steel thread {new}");
    if self.canon.threads.iter().any(|t| t.id == new) {
      return Err(FacadeError::RenumberTargetTaken {
        subject,
        held_by: "the store".to_string(),
      });
    }
    self.refuse_a_taken_path(
      &subject,
      &[
        self.project.thread_json(new),
        self.project.thread_dir(new),
        self.project.canon_st_dir().join(new),
      ],
    )?;
    let model =
      crate::renumber::thread(&self.canon, old, new).ok_or_else(|| FacadeError::NoSuchThread {
        id: old.to_string(),
      })?;
    let foreign = self.foreign_under(&self.project.thread_dir(old))?;
    let envelope = Envelope::minted(
      &self.ctx.principal,
      &self.ctx.project_id,
      "st.renumber",
      Subject {
        kind: "thread".to_string(),
        id: new.to_string(),
      },
      json!({ "from": old, "to": new }),
    );
    self.land_renumber(RenumberPlan {
      from: old.to_string(),
      to: new.to_string(),
      sigil: Sigil::SteelThread,
      // The realised directory carries the attachments, the views and any file
      // nobody modelled; the canon directory carries the opaque sidecars.
      moves: vec![
        (self.project.thread_dir(old), self.project.thread_dir(new)),
        (
          self.project.canon_st_dir().join(old),
          self.project.canon_st_dir().join(new),
        ),
      ],
      stale: vec![self.project.thread_json(old)],
      rerender: Some(self.project.thread_dir(new)),
      envelope,
      model,
      foreign,
    })
  }

  /// `intent issues renumber <old> <new>` (ST0078 WP-02, AC-02.2): the same
  /// move for an issue, whose canon file and view are named by its number.
  pub fn issue_renumber(&mut self, old: u32, new: u32) -> Result<Renumbering, FacadeError> {
    self.issue_show(old)?;
    let subject = format!("issue {}", crate::model::issue_id(new));
    if self.canon.issues.iter().any(|i| i.number == new) {
      return Err(FacadeError::RenumberTargetTaken {
        subject,
        held_by: "the store".to_string(),
      });
    }
    self.refuse_a_taken_path(
      &subject,
      &[self.project.issue_json(new), self.project.issue_view(new)],
    )?;
    let model = crate::renumber::issue(&self.canon, old, new)
      .ok_or(FacadeError::NoSuchIssue { number: old })?;
    let foreign = self.foreign_under(&self.project.issue_view(old))?;
    let (from, to) = (crate::model::issue_id(old), crate::model::issue_id(new));
    let envelope = Envelope::minted(
      &self.ctx.principal,
      &self.ctx.project_id,
      "issues.renumber",
      Subject {
        kind: "issue".to_string(),
        id: to.clone(),
      },
      json!({ "from": from, "to": to }),
    );
    self.land_renumber(RenumberPlan {
      from,
      to,
      sigil: Sigil::Issue,
      moves: Vec::new(),
      stale: vec![self.project.issue_json(old), self.project.issue_view(old)],
      rerender: None,
      envelope,
      model,
      foreign,
    })
  }

  fn refuse_a_taken_path(
    &self,
    subject: &str,
    paths: &[std::path::PathBuf],
  ) -> Result<(), FacadeError> {
    match paths.iter().find(|p| p.exists()) {
      Some(path) => Err(FacadeError::RenumberTargetTaken {
        subject: subject.to_string(),
        held_by: self.project.relative(path),
      }),
      None => Ok(()),
    }
  }

  /// The paths under `root` holding bytes the store did not render, asked
  /// BEFORE a renumber moves or removes them -- the same predicate a write
  /// asks of the paths it overwrites.
  fn foreign_under(&self, root: &std::path::Path) -> Result<Vec<String>, FacadeError> {
    let ctx = self.render_ctx()?;
    let mut set = WriteSet::new();
    for view in views::render_all(&self.project, &self.canon, &ctx) {
      if view.path.starts_with(root) {
        set.add(view.path, view.content);
      }
    }
    self.foreign_bytes(&set)
  }

  /// Land a renumber: the manifest row and the disk moves first, because the
  /// projection inside the write reads the manifest and renders into the moved
  /// directory; then the one write; then what follows it.
  ///
  /// **A REFUSED WRITE PUTS BACK EVERYTHING IT MOVED**, in reverse, so a
  /// refusal means nothing was renumbered. After the write the store has
  /// moved, so every later failure is a note with its own remedy.
  fn land_renumber(&mut self, plan: RenumberPlan) -> Result<Renumbering, FacadeError> {
    let RenumberPlan {
      from,
      to,
      sigil,
      moves,
      stale,
      rerender,
      envelope,
      model,
      foreign,
    } = plan;
    let mut moved = Vec::new();
    let manifest = self.project.intentfiles_path();
    let before = match std::fs::read_to_string(&manifest) {
      Ok(text) => Some(text),
      Err(source) if source.kind() == std::io::ErrorKind::NotFound => None,
      Err(source) => {
        return Err(FacadeError::ManifestUnreadable {
          path: manifest.display().to_string(),
          source,
        });
      }
    };
    if let Some(text) = &before {
      let unpinned = intentfiles::unpin(text, sigil, &from)?;
      if &unpinned != text {
        let after = intentfiles::pin(&unpinned, sigil, &to, None)?;
        let mut set = WriteSet::new();
        set.add(manifest.clone(), after);
        set.commit()?.keep();
        moved.push(format!(
          "{}: {} -> {}",
          self.project.relative(&manifest),
          intentfiles::declared_key(sigil, &from),
          intentfiles::declared_key(sigil, &to)
        ));
      }
    }
    let mut done: Vec<(std::path::PathBuf, std::path::PathBuf)> = Vec::new();
    for (source_path, target) in moves {
      if !source_path.exists() {
        continue;
      }
      if let Err(source) = std::fs::rename(&source_path, &target) {
        self.undo_renumber_moves(&done)?;
        self.restore_manifest(before)?;
        return Err(FacadeError::RenumberDiskStep {
          step: format!(
            "move {} to {}",
            self.project.relative(&source_path),
            self.project.relative(&target)
          ),
          source,
        });
      }
      moved.push(format!(
        "{} -> {}",
        self.project.relative(&source_path),
        self.project.relative(&target)
      ));
      done.push((source_path, target));
    }
    let applied = match self.apply_envelopes(
      vec![envelope],
      model.canon,
      crate::store::ProjectStateEdit::Unchanged,
    ) {
      Ok(applied) => applied,
      Err(refused) => {
        self.undo_renumber_moves(&done)?;
        self.restore_manifest(before)?;
        return Err(refused);
      }
    };
    let mut notes = Vec::new();
    for path in stale {
      match std::fs::remove_file(&path) {
        Ok(()) => moved.push(format!("{} removed", self.project.relative(&path))),
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => {}
        Err(source) => {
          let rel = self.project.relative(&path);
          notes.push(Note::after_write(
            &format!("removing {rel}"),
            &FacadeError::RenumberDiskStep {
              step: format!("remove {rel}"),
              source,
            },
            &format!(
              "the renumber is in the store and {to} is written -- do NOT run it again. Delete \
               {rel} by hand; until it is gone, `intent sync --to-store` would load {from} back"
            ),
          ));
        }
      }
    }
    // **A MOVED DIRECTORY'S VIEWS STILL NAME THE OLD ID.** The write re-renders
    // them for a declared thread; an undeclared one keeps its files and is
    // never re-rendered by a write, so this does it, for exactly the views the
    // directory already held.
    if let Some(root) = rerender
      && let Err(cause) = self.rerender_existing_under(&root)
    {
      notes.push(Note::after_write(
        "re-rendering the moved views",
        &cause,
        RERENDER_REMEDY,
      ));
    }
    let mut rewritten = model.rewritten;
    if !model.claims.is_empty() {
      // Only this call's board notes: a long-lived facade may hold others.
      let mark = self.after_write.len();
      for (node, claims) in &model.claims {
        let event = self.wb_event("st.renumber", node, json!({ "from": from, "to": to }));
        if let Err(cause) = self
          .store
          .wb_write(&event, |w| w.set_claims(node, claims))
          .map_err(FacadeError::Store)
        {
          rewritten.retain(|line| !line.starts_with(&format!("{node}'s claims:")));
          notes.push(Note::after_write(
            &format!("moving {node}'s claims"),
            &cause,
            &format!(
              "the renumber is in the store -- do NOT run it again. Move the claim by hand with \
               `intent wb unclaim {from} --node {node}` and `intent wb claim {to} --node {node}`"
            ),
          ));
        }
      }
      self.land_board_write_noting()?;
      notes.extend(self.after_write.drain(mark..));
    }
    let prose = match self.search_all(&from, &crate::search::SearchQuery::default()) {
      Ok(answer) => prose_mentions(&answer),
      Err(cause) => {
        notes.push(Note::after_write(
          &format!("asking the index for the prose that names {from}"),
          &cause,
          &format!("the renumber is done; `intent search {from}` lists what still names it"),
        ));
        Vec::new()
      }
    };
    let mut outcome = Outcome::Moved.with_overwrites(Applied {
      foreign: [foreign, applied.foreign].concat(),
      after_write: applied.after_write,
    });
    if !notes.is_empty() {
      outcome = match outcome {
        Outcome::MovedWith { notes: mut had } => {
          had.extend(notes);
          Outcome::MovedWith { notes: had }
        }
        _ => Outcome::MovedWith { notes },
      };
    }
    Ok(Renumbering {
      from,
      to,
      outcome,
      rewritten,
      moved,
      prose,
    })
  }

  /// Put back a renumber's disk moves, last first.
  fn undo_renumber_moves(
    &self,
    done: &[(std::path::PathBuf, std::path::PathBuf)],
  ) -> Result<(), FacadeError> {
    for (source_path, target) in done.iter().rev() {
      std::fs::rename(target, source_path).map_err(|source| FacadeError::RenumberDiskStep {
        step: format!(
          "put {} back at {} after the renumber was refused",
          self.project.relative(target),
          self.project.relative(source_path)
        ),
        source,
      })?;
    }
    Ok(())
  }

  /// Re-render the views under `root` that are on disk, from the store.
  fn rerender_existing_under(&mut self, root: &std::path::Path) -> Result<(), FacadeError> {
    let ctx = self.render_ctx()?;
    let mut set = WriteSet::new();
    for view in views::render_all(&self.project, &self.canon, &ctx) {
      if view.path.starts_with(root) && view.path.exists() {
        set.add(view.path, view.content);
      }
    }
    drop(ctx);
    if set.is_empty() {
      return Ok(());
    }
    let applied = set.commit()?;
    let written: Vec<std::path::PathBuf> =
      applied.written().map(std::path::PathBuf::from).collect();
    applied.keep();
    self.record_landed(&[], &written)
  }

  pub fn st_cancel(&mut self, id: &str, reason: &str) -> Result<Outcome, FacadeError> {
    self.st_cancel_listing(id, reason, ListEdit::AsDeclared, None)
  }

  /// `st cancel --keep`: cancel the thread and LEAVE its `.intentfiles` entry,
  /// so its files stay on disk.
  ///
  /// **THE SYMMETRY IS hv's RULING, 2026-08-20, AND IT REVERSES A GUESS OF
  /// MINE.** AC-05.2 named `st done --keep` and said nothing about `st cancel`,
  /// so the surface shipped with the opt-out on one of two identical acts. I
  /// read the silence as deliberate -- cancelling is the stronger statement, so
  /// you are less likely to want the files -- and hv ruled the other way.
  ///
  /// **The guess was plausible and wrong for a reason worth keeping: `--keep`
  /// is not about how sure you are that the work is over, it is about whether
  /// you still need to READ the files** -- and a cancelled thread is at least
  /// as likely to be one you are still mining for what it decided. Both verbs
  /// remove the entry, so both take the same override under the same word.
  /// **`on` THREADED RATHER THAN GIVEN A THIRD DOOR, AND THE PRECEDENT ABOVE
  /// IS WHY THAT NEEDS SAYING.** `st_cancel`'s note chose a wrapper over a
  /// parameter because threading it would have rewritten thirteen call sites
  /// across seven test files that four sessions edit concurrently. **That
  /// reasoning is about BREADTH, not about parameters** -- measured here, the
  /// two `_listing` doors have twelve call sites in THREE files, eight of them
  /// in one test file and the rest being this definition and the CLI. A third
  /// door for a second optional knob would have been the cost the precedent
  /// was avoiding, paid to avoid a diff it was not describing.
  pub fn st_cancel_listing(
    &mut self,
    id: &str,
    reason: &str,
    list: ListEdit,
    on: Option<&str>,
  ) -> Result<Outcome, FacadeError> {
    self.set_thread_status(
      id,
      ThreadStatus::Cancelled,
      "st.cancel",
      Some(reason),
      list,
      on,
    )
  }

  /// `list` is [`ListEdit::AsDeclared`] for every caller but `st done --keep`.
  /// The wording for a [`crate::model::DateError`].
  ///
  /// **Here rather than on the type, mirroring `IdError` and `render.rs`'s
  /// `id_refusal`.** The model states what is wrong; a caller says it in the
  /// terms of the field it was asked about, and a second caller with a different
  /// field would need different words for the same fact.
  /// The completion date a thread may record, for every door that writes one
  /// (issues 0503 and 0504).
  ///
  /// **ONE HOME, BECAUSE EVERY DOOR OWES THE SAME TWO CHECKS.** A stated date
  /// is ISO 8601 `YYYY-MM-DD`: a malformed one reaching canon is unrecoverable
  /// by inspection, because the field has no time component, so `2026-02-30`
  /// reads as data rather than as an error for as long as it sits there. And a
  /// thread records a completion date only where it is Completed or Cancelled:
  /// a date under any other status is a claim about a close that never
  /// happened. The status writer's arm had both checks, [`Facade::set`] had
  /// neither, and the self-loop reached neither -- three doors, one rule, and
  /// this is where it lives.
  ///
  /// **THE REFUSAL NAMES THE STATUS RATHER THAN THE MOVE**, which is the one
  /// wording this cost: the status writer said *a move to wip records no
  /// completion date*, and one sentence has to serve a door that moves a thread
  /// and a door that edits one standing still.
  fn recordable_completion(
    status: crate::model::ThreadStatus,
    stated: &str,
  ) -> Result<String, FacadeError> {
    let refuse = |why: String| FacadeError::ValueNotRecordable {
      field: "completed".to_string(),
      given: stated.to_string(),
      why,
    };
    match status {
      ThreadStatus::Completed | ThreadStatus::Cancelled => {
        crate::model::parse_domain_date(stated).map_err(|e| refuse(Self::date_refusal(e)))
      }
      _ => Err(refuse(format!(
        "a {} thread records no completion date",
        crate::model::enum_str(&status)
      ))),
    }
  }

  fn date_refusal(e: crate::model::DateError) -> String {
    match e {
      crate::model::DateError::NotADate => {
        "this field records ISO 8601 `YYYY-MM-DD`, with no time component".to_string()
      }
      crate::model::DateError::NoSuchMonth { named } => {
        format!("it names month {named}, and there are twelve")
      }
      crate::model::DateError::NotADay => "that is not a day in the calendar".to_string(),
    }
  }

  /// Realise the files of a thread a state verb has just listed again, through
  /// the one realise path (issue 0367).
  ///
  /// **AFTER THE WRITE, SO A FAILURE IS A NOTE** (0376's rule): the status and
  /// the list edit have landed, and `intent organize --apply` writes what this
  /// could not.
  fn realise_relisted(&mut self, id: &str) -> Option<Note> {
    let address = Address {
      authority: None,
      entity: AddrEntity::Thread { id: id.to_string() },
      format: None,
    };
    self.hydration(&address).err().map(|cause| {
      Note::after_write(
        "realising the thread's attachments",
        &cause,
        "the status and the list edit are recorded; `intent organize --apply` writes the thread's files",
      )
    })
  }

  fn set_thread_status(
    &mut self,
    id: &str,
    status: ThreadStatus,
    op: &'static str,
    reason: Option<&str>,
    list: ListEdit,
    on: Option<&str>,
  ) -> Result<Outcome, FacadeError> {
    self.set_thread_status_fiat(id, status, op, reason, list, on, None)
  }

  /// The one implementation, taking what happens to the fiat record beside the
  /// status (ST0066).
  ///
  /// **A WRAPPER OVER ONE IMPLEMENTATION, for the reason [`Facade::st_done`] and
  /// [`Facade::st_done_listing`] already record**: threading the argument through
  /// would have rewritten eight call sites in a file four sessions edit
  /// concurrently, for a diff whose entire content is the word `None`. The
  /// delegation is one line and there is one body, so this is a second door and
  /// never a second answer.
  ///
  /// **THE INVARIANT LIVES HERE BECAUSE THIS IS WHERE EVERY THREAD STATUS WRITE
  /// PASSES.** `Thread::fiat` is beside the status and no type holds the two in
  /// agreement, so something must, and a rule kept by every verb separately is a
  /// rule kept until somebody adds the next verb. **The clear is unconditional
  /// and is not "reset a field on the way past"**: a fiat record explains the
  /// state the thread was IN, so a thread that has left `completed` has left the
  /// record's subject behind. That is the same argument `status_reason` makes,
  /// with the opposite conclusion about defaults -- a reason is written only when
  /// the caller says something, and a fiat record is cleared unless the caller
  /// says otherwise -- because a stale reason is confusing and a stale fiat
  /// record is a false claim about a person.
  #[allow(clippy::too_many_arguments)]
  fn set_thread_status_fiat(
    &mut self,
    id: &str,
    status: ThreadStatus,
    op: &'static str,
    reason: Option<&str>,
    list: ListEdit,
    on: Option<&str>,
    fiat: Option<crate::model::FiatRecord>,
  ) -> Result<Outcome, FacadeError> {
    let from = self.st_show(id)?.status;

    // **THE SELF-LOOP TEST IS FIRST, AND ITS POSITION IS THE RULING.** It sits
    // ahead of the transition check, ahead of the reason guard and -- the half
    // that matters -- ahead of the gate below. `st done` on an already-completed
    // thread must not re-run the gate, or a criterion added AFTER the close
    // blocks a thread that is legitimately finished. That is not hypothetical:
    // AC-04.6 was added under closed units in this very thread. A self-loop must
    // not be able to fail for a reason that did not exist when the state was
    // entered.
    //
    // **A STATED DATE IS ANSWERED HERE, AND THE GATE STILL STAYS OUT** (issue
    // 0503, ruled by vc under hv's pen, 2026-09-22). That ruling is about the
    // GATE, and a date the caller has just typed is not "a reason that did not
    // exist when the state was entered" -- it is this call's own argument.
    // Dropping it under an `ok:` line was the silent default this estate
    // refuses everywhere else. So the date is validated exactly as a close
    // validates it; the same date as the one on record is still nothing to do;
    // and any other date is a RESTATEMENT, which `intent set` is the door for.
    if from == status {
      if let Some(stated) = on {
        let stated = Self::recordable_completion(status, stated)?;
        let recorded = self.st_show(id)?.completed.clone();
        if recorded.as_deref() != Some(stated.as_str()) {
          return Err(FacadeError::CompletionDateNotRestated {
            st: id.to_string(),
            state: from.display().to_string(),
            recorded,
            given: stated,
          });
        }
      }
      return Ok(Outcome::AlreadyThere {
        state: from.display().to_string(),
      });
    }

    Self::check_transition("Thread", "status", op, &crate::model::enum_str(&from), id)?;
    self.check_gate(("Thread", "status", op), id, id, Scope::Thread)?;
    // **`st done` READS THE PACKAGES** (issue 0324). The gate is over criteria
    // only, so a thread closed with a work package still `not-started`. `st fc`
    // is the human's override and does not pass through here.
    // After the gate, so a thread whose criteria are unsatisfied is told that
    // first, and the packages are the last thing between a passing gate and
    // the close.
    if op == "st.done" {
      let open: Vec<String> = self
        .st_show(id)?
        .wps
        .iter()
        .filter(|w| matches!(w.status, WpStatus::NotStarted | WpStatus::Wip))
        .map(|w| format!("{id}/{:02} ({})", w.seq, w.status.display()))
        .collect();
      if !open.is_empty() {
        return Err(FacadeError::OpenWorkPackages {
          st: id.to_string(),
          open,
        });
      }
    }
    // The guard still refuses a verb that REQUIRES a reason and was given
    // none; its return value is no longer the thing that writes the field.
    Self::check_reason("Thread", "status", op, reason)?;
    // Read BEFORE anything is written -- see [`Facade::closing_notes`].
    let mut notes = self.closing_notes(op, id, list)?;
    // Issue 0337: a close on an unwritten objective warns rather than passing in silence.
    if status == ThreadStatus::Completed && self.st_show(id)?.objective.trim().is_empty() {
      notes.push(Note::UnwrittenObjective(id.to_string()));
    }
    let mut next = self.canon.clone();
    let thread = find_thread_mut(&mut next, id)?;
    thread.status = status;
    // See this function's own doc: unconditional, because the record describes
    // the state being left.
    thread.fiat = fiat.clone();
    // **WRITTEN ONLY WHEN THE CALLER SAID SOMETHING, WHICH MAKES CLEARING AN ACT
    // RATHER THAN A DEFAULT** (vc's ruling, 2026-08-25). It read
    // `thread.status_reason = reason.clone()` unconditionally, and `check_reason`
    // returns `Ok(None)` for any verb whose guard does not require one -- so
    // SEVEN verbs across two entities cleared the field on their way past:
    // `st triage|start|resume|done` and `wp start|unstart|done`.
    //
    // **THE SPLIT IS ONE DESIGN DECISION MADE TWICE, NOT A COINCIDENCE: the
    // verbs that CLEAR are the ones that move work FORWARD, and the ones that
    // pass a reason are the ones that stop or reverse it.** A reason is
    // something you supply when you interrupt, and nobody asked what happens to
    // the reason already there when you resume.
    //
    // **AND THE WORST INSTANCE IS THE CLOSING VERB, ON BOTH ENTITIES** -- this
    // row's own words: *the verb that records the outcome erases the
    // reasoning*, and *the rows nobody can afford to lose are the ones the
    // closing verb hits hardest*. Two live records were exposed, each the
    // account of a decision rather than incidental text.
    //
    // **THIS DOES NOT DECIDE THE SEMANTICS AND MUST NOT.** Whether a resumed
    // thread's hold reason is spent stays open; under this shape each call site
    // declares its own answer and can be argued one at a time. A verb that
    // genuinely spends the reason passes `Some("")` explicitly. **A fix that
    // answered the semantics on the way past would become the ruling by
    // default**, which is the failure mode this criterion produced twice today.
    if let Some(given) = reason {
      let given = given.trim();
      thread.status_reason = (!given.is_empty()).then(|| given.to_string());
    }
    // `Some("")` is "completed, and the database says when" -- the third state
    // the CREATE door recognises. `None` stays null; a date already recorded is
    // carried. The facade never holds a time in any of the three.
    // **`Some("")` IS STILL "the database says when", AND A STATED DATE RIDES
    // THE SAME FIELD.** The store writes
    // `COALESCE(NULLIF(?7, ''), strftime('%Y-%m-%d', 'now'))`, so a non-empty
    // value is recorded as given and only the empty one reaches the clock --
    // which is why `--date` needs no second clock and no new column. The
    // facade still holds no time in any of the three states.
    //
    // **Validated HERE rather than at the door, because every door owes it.**
    // A malformed date reaching canon is unrecoverable by inspection: the field
    // has no time component, so `2026-02-30` reads as data rather than as an
    // error for as long as it sits there.
    thread.completed = match status {
      ThreadStatus::Completed | ThreadStatus::Cancelled => Some(match on {
        Some(stated) => Self::recordable_completion(status, stated)?,
        None => String::new(),
      }),
      // **A NON-TERMINAL MOVE WITH A DATE IS A MISTAKE WORTH NAMING.** Silently
      // dropping it would let `st start --date` read as accepted and change
      // nothing, which is the shape of every silent default in this estate.
      // The refusal is [`Facade::recordable_completion`]'s, which is why the
      // `?` below is the whole arm: on any status but the two terminal ones it
      // never returns a date, so nothing reaches the `None`.
      _ => {
        if let Some(stated) = on {
          Self::recordable_completion(status, stated)?;
        }
        None
      }
    };
    // **MINTED HERE RATHER THAN INSIDE `apply`, so a cascade can carry this
    // envelope's ID into every child it closes.** `Envelope::minted` generates
    // the ULID in Rust, so the id exists before any write and the whole cascade
    // rides ONE transaction.
    let envelope = Envelope::minted(
      &self.ctx.principal,
      &self.ctx.project_id,
      op,
      Subject {
        kind: "thread".to_string(),
        id: id.to_string(),
      },
      json!({
        "from": crate::model::enum_str(&from),
        "to": crate::model::enum_str(&status),
        "reason": reason,
      }),
    );
    let mut envelopes = vec![envelope];
    // **THE CASCADE IS KEYED ON THE RECORD, NOT ON THE OP, and that is the point
    // of putting it here.** A fiat close is the only thread transition that
    // carries one, so any verb that ever does will cascade by construction
    // rather than by remembering to -- the same reason the CLEAR lives in this
    // door rather than in each verb.
    if let Some(record) = fiat.clone() {
      let ancestor_event = envelopes[0].id.clone();
      let ctx = self.ctx.clone();
      let thread = find_thread_mut(&mut next, id)?;
      envelopes.extend(Self::cascade_fiat(
        &ctx,
        thread,
        None,
        id,
        &ancestor_event,
        &record,
      ));
    }
    // **A REMOVAL IS EDITED AFTER `apply` AND AN ADDITION BEFORE IT** (issue
    // 0079) -- see [`Facade::edit_list`] for why each order is chosen for its
    // failure mode. Pinned after, `st start` was the one write that missed its
    // own thread: the projection inside `apply` reads the manifest, so the
    // thread was listed with no files until the next write by anyone. Pinned
    // before, this write renders it exactly as every write renders every
    // declared thread -- no second realiser, and attachments stay `organize`'s.
    let adds =
      list == ListEdit::AsDeclared && matches!(declared_list_edit(op), Some((_, ListAction::Add)));
    let manifest = self.project.intentfiles_path();
    let before = adds
      .then(|| std::fs::read_to_string(&manifest).ok())
      .flatten();
    if adds {
      self.edit_list(op, id, list)?;
    }
    let foreign =
      match self.apply_envelopes(envelopes, next, crate::store::ProjectStateEdit::Unchanged) {
        Ok(foreign) => foreign,
        Err(refused) => {
          // The status did not move, so neither may the list it follows.
          self.restore_manifest(before)?;
          return Err(refused);
        }
      };
    if !adds {
      self.edit_list(op, id, list)?;
    }
    // Issue 0367: a re-listed thread's views were realised and its attachments left to organize, so the canon gate refused the next commit.
    if adds {
      notes.extend(self.realise_relisted(id));
    }
    // **ASKED AFTER THE PIN, BECAUSE THE PIN IS WHAT MAKES IT HELD** (issue
    // 0209). A thread this verb has just declared, with a v2 bucket copy and
    // nothing at its home, will be skipped by every write -- and this verb is
    // the one whose success reads as "on its way to disk".
    let realised = self.realised_threads();
    let present = organize::presence(&self.project, &self.canon, &realised);
    if let Some(h) = organize::held(&self.project, &self.canon, &realised, &present)
      .into_iter()
      .find(|h| h.thread == id)
    {
      notes.push(Note::HeldByV2Bucket {
        thread: h.thread,
        dir: self.project.relative(&h.dir),
        home: self.project.relative(&h.home),
        files: h.files,
      });
    }
    Ok(
      if notes.is_empty() {
        Outcome::Moved
      } else {
        Outcome::MovedWith { notes }
      }
      .with_overwrites(foreign),
    )
  }

  /// Enforce a declared [`transitions::Guard::GatePass`], and only where it is
  /// declared.
  ///
  /// **This existed as hand-written code at two call sites and the declaration
  /// was decorative for it.** `Edge::guarded("st.done", .., &[Guard::GatePass])`
  /// said the gate was a precondition, while `st_done` and `wp_done` ran the gate
  /// themselves before delegating -- so deleting `GatePass` from the table
  /// changed nothing, which is the same declaration-versus-implementation split
  /// AC-04.6 exists to find. `Guard::ReasonRecorded` was already enforced from
  /// its declaration; this makes the pair consistent.
  ///
  /// **And the ordering falls out rather than having to be remembered.** Run by
  /// the shared setter, the gate now sits AFTER the self-loop test by
  /// construction, which is what the self-loop ruling requires. Hoisted at the
  /// call site it ran first, and any later verb copying that shape would have
  /// reintroduced a gate that can fail for a reason postdating the state.
  ///
  /// The SCOPE stays local because only the caller knows it -- a thread gates on
  /// itself, a work package on its own sequence -- so the declaration decides
  /// WHETHER and the caller decides ABOUT WHAT.
  /// `thread` is what the gate is RUN against and `label` is what a refusal
  /// NAMES -- they differ for a work package, which gates on its thread and
  /// reports as `ST0001/01`.
  fn check_gate(
    &mut self,
    field: (&'static str, &'static str, &'static str),
    thread: &str,
    label: &str,
    scope: Scope,
  ) -> Result<(), FacadeError> {
    let (entity, name, verb) = field;
    if !transitions::guard_for(entity, name, verb).contains(&transitions::Guard::GatePass) {
      return Ok(());
    }
    let verdict = self.gate(thread, scope)?;
    match &verdict {
      Verdict::Pass { .. } | Verdict::Exempt { .. } => Ok(()),
      Verdict::Blocked { remedy, .. } => Err(FacadeError::GateBlocked {
        scope: label.to_string(),
        verdict: verdict.line(label),
        remedy: remedy.clone(),
      }),
    }
  }

  /// Refuse a transition the ratified machine does not have.
  ///
  /// **It asks [`crate::transitions`] rather than carrying its own copy of the
  /// from-states**, so there is one machine rather than a declaration and an
  /// implementation that can disagree. That disagreement is precisely what
  /// AC-04.6 exists to find, and the cheapest way to never find it is to make
  /// it unconstructible.
  fn check_transition(
    entity: &'static str,
    field: &'static str,
    verb: &'static str,
    from: &str,
    subject: &str,
  ) -> Result<(), FacadeError> {
    if transitions::permits(entity, field, verb, from) {
      return Ok(());
    }
    Err(FacadeError::IllegalTransition {
      verb,
      subject: subject.to_string(),
      from: from.to_string(),
      legal: transitions::accepted_from(entity, field, verb).join(", "),
    })
  }

  /// Require a reason exactly where the declared guard says one is required.
  ///
  /// **The clearing of a stale reason is NOT done here, and saying it was is a
  /// claim mutation-testing refused.** Replacing the `None` arm below with a
  /// pass-through changed no test, because every unguarded verb passes `None`
  /// anyway -- so the declaration was not what cleared anything, the caller's
  /// signature was. The clearing lives in the unconditional
  /// `status_reason = reason` assignment at each call site, and THAT is what a
  /// test kills a mutant on. Recorded rather than quietly reworded, because a
  /// comment claiming the wrong mechanism is how the next person builds on a
  /// guarantee that is not there.
  ///
  /// What it is guarding against is real: `st hold --reason "waiting on the
  /// fleet"` followed by `st resume` must not leave a running thread explaining
  /// why it was paused -- a reason outliving the condition it described, which
  /// is this estate's remedy-outliving-its-model class in different clothes.
  fn check_reason(
    entity: &'static str,
    field: &'static str,
    verb: &'static str,
    reason: Option<&str>,
  ) -> Result<Option<String>, FacadeError> {
    if transitions::guard_for(entity, field, verb).contains(&transitions::Guard::ReasonRecorded) {
      return reason
        .map(str::trim)
        .filter(|r| !r.is_empty())
        .map(|r| Some(r.to_string()))
        .ok_or(FacadeError::ReasonRequired { verb });
    }
    Ok(None)
  }

  /// The prose a criterion's state carries to justify itself, for the guards
  /// that require one.
  ///
  /// **One function for both, because they are one question asked of two
  /// states**: a withdrawal explains a decision, a satisfaction stands in for a
  /// test result, and in each case the state is worthless without it. The
  /// guard decides WHICH is owed and the refusal says so; this only knows where
  /// to look.
  fn justification(state: &AcState) -> Option<&str> {
    match state {
      AcState::Satisfied { evidence } => Some(evidence),
      AcState::Withdrawn { reason, .. } => Some(reason),
      // **`--because` is enforced HERE and nowhere else**, through the same
      // `Guard::ReasonRecorded` the machine declares for `ac.withdraw`. A
      // second check written beside `ac_fc` would be a second home for the
      // requirement `FiatRecord.because`'s `length(min = 1)` already states.
      AcState::Fiat(record) => Some(&record.because),
      _ => None,
    }
  }

  /// Enforce the guards the AC machine DECLARES for this verb.
  ///
  /// **The guard column existed for criteria and nothing read it.**
  /// `set_ac_state` consulted the declaration for the FROM-STATE only, so
  /// `ac.withdraw`'s ratified `ReasonRecorded` was declared, transcribed,
  /// checked for faithfulness by `mutation_completeness.rs` -- and never
  /// enforced, because the enforcement path for reasons ran through
  /// [`Self::check_reason`], which only Thread and WorkPackage verbs call.
  /// Found while confirming ic's evidence defect; the two are one defect seen
  /// at two verbs.
  ///
  /// Blank counts as absent. A shell makes `--reason ""` and `--reason "  "`
  /// the same gesture, so a guard that refuses one and stores the other is a
  /// guard that teaches its own bypass.
  fn check_ac_guards(verb: &'static str, ac: &str, state: &AcState) -> Result<(), FacadeError> {
    let supplied = Self::justification(state)
      .map(str::trim)
      .is_some_and(|j| !j.is_empty());
    if supplied {
      return Ok(());
    }
    for guard in transitions::guard_for("Criterion", "state", verb) {
      match guard {
        transitions::Guard::ReasonRecorded => {
          return Err(FacadeError::ReasonRequired { verb });
        }
        transitions::Guard::EvidenceRecorded => {
          return Err(FacadeError::EvidenceRequired { ac: ac.to_string() });
        }
        // `NonTestOnly` and `TargetExists` are enforced by the verbs, which
        // have the criterion and the canon this static check does not. They
        // are named rather than swept into a wildcard so that a guard added to
        // the table cannot land in a silent arm -- the failure that produced
        // this function.
        transitions::Guard::NonTestOnly
        | transitions::Guard::TargetExists
        | transitions::Guard::GatePass => {}
      }
    }
    Ok(())
  }

  // -------------------------------------------------------------------------
  // Work packages
  // -------------------------------------------------------------------------

  pub fn wp_new(&mut self, st: &str, title: &str, scope: TShirt) -> Result<u32, FacadeError> {
    let seq = self
      .st_show(st)?
      .wps
      .iter()
      .map(|w| w.seq)
      .max()
      .unwrap_or(0)
      + 1;
    let mut next = self.canon.clone();
    find_thread_mut(&mut next, st)?.wps.push(WorkPackage {
      objective: String::new(),
      body: String::new(),
      preamble: String::new(),
      seq,
      title: title.to_string(),
      // A work package created through v3 always has a real size: the legacy
      // form exists only for values that arrived from a v2 estate.
      scope: Some(scope),
      scope_legacy: None,
      status: WpStatus::NotStarted,
      status_legacy: None,
      status_reason: None,
      // A creation path: only `wp.fc` writes this, so `None` is the fact and
      // not a placeholder. A brand-new package has not been closed at all.
      fiat: None,
    });
    let applied = self.apply(
      "wp.new",
      Subject {
        kind: "wp".to_string(),
        id: format!("{st}/{seq:02}"),
      },
      json!({"title": title, "scope": crate::model::enum_str(&scope)}),
      next,
    )?;
    self.park(applied);
    Ok(seq)
  }

  pub fn wp_start(&mut self, st: &str, seq: u32) -> Result<Outcome, FacadeError> {
    self.set_wp_status(st, seq, WpStatus::Wip, "wp.start", None)
  }

  /// Put a work package back to `not-started` -- the inverse of `wp start`,
  /// for one started by mistake or on the wrong thread.
  pub fn wp_unstart(&mut self, st: &str, seq: u32) -> Result<Outcome, FacadeError> {
    self.set_wp_status(st, seq, WpStatus::NotStarted, "wp.unstart", None)
  }

  /// Close a work package, gated on its own scope.
  /// Close a work package. The gate is a DECLARED guard, run by the shared
  /// setter after the self-loop test -- see `Facade::check_gate`.
  pub fn wp_done(&mut self, st: &str, seq: u32) -> Result<Outcome, FacadeError> {
    self.set_wp_status(st, seq, WpStatus::Done, "wp.done", None)
  }

  /// Reopen a closed work package, recording why.
  ///
  /// **This is the verb whose absence was doing live damage.** `wp done`
  /// consults the gate on the way in and nothing re-checks afterwards, so a
  /// work package that was legitimately `Done` becomes a false green the moment
  /// an AC is added to it -- and with no inverse, the only repair was editing
  /// the file the CLI exists to own. Measured on this thread on 2026-08-15:
  /// three of five work packages carried a status that disagreed with their own
  /// gate, two of them written by the verifier enforcing the rule that names
  /// the class.
  pub fn wp_reopen(&mut self, st: &str, seq: u32, reason: &str) -> Result<Outcome, FacadeError> {
    self.set_wp_status(st, seq, WpStatus::Wip, "wp.reopen", Some(reason))
  }

  /// Cancel a work package whose scope was removed, recording why.
  ///
  /// **The state the model could not express, and the gap was ruled rather
  /// than overlooked.** `data-model.md`'s Machine 2 proposed no `Cancelled` at
  /// WP level -- _"a WP that stops mattering is a scope change on the thread,
  /// not a state on the package"_ -- and flagged it _"Open for hv if that is
  /// wrong"_. It was wrong, and hv ruled so on 2026-08-21 after a live consumer
  /// hit it: scope removed, every AC withdrawn, and `wp done` refused forever
  /// because [`crate::contract::gate`] correctly declines to infer an exemption
  /// from an emptied contract. The only announced exemption was thread-scoped,
  /// so closing one unit would have discarded the standing of all 37 ACs.
  ///
  /// **A reason is REQUIRED, mirroring `st cancel`.** A cancelled unit is the
  /// one status a reader cannot interpret without knowing why -- `Done` says
  /// delivered, `Cancelled` says nothing at all on its own.
  ///
  /// **Deliberately NOT gated.** Every other close consults the contract; this
  /// one is the announcement that there is no contract to consult, so gating it
  /// would reproduce the deadlock it exists to break.
  pub fn wp_cancel(&mut self, st: &str, seq: u32, reason: &str) -> Result<Outcome, FacadeError> {
    self.set_wp_status(st, seq, WpStatus::Cancelled, "wp.cancel", Some(reason))
  }

  /// Reinstate a cancelled work package, recording why.
  ///
  /// Lands on `NotStarted` rather than restoring the pre-cancellation status,
  /// mirroring `st reinstate`: the previous status is not recorded anywhere, so
  /// restoring it would be a guess wearing the authority of a verb.
  pub fn wp_reinstate(&mut self, st: &str, seq: u32, reason: &str) -> Result<Outcome, FacadeError> {
    self.set_wp_status(st, seq, WpStatus::NotStarted, "wp.reinstate", Some(reason))
  }

  /// Re-size a work package.
  ///
  /// **`wp new` lets the caller choose a size and nothing could ever change
  /// it.** Neither v2 nor v3 had this verb, so a work package mis-sized at
  /// creation -- or, more usually, correctly sized and then understood better
  /// -- could only be corrected by hand-editing the file the tool owns. That
  /// is the shape hv ruled on, one entity over from the criterion it was ruled
  /// on, and it was found by vc's discriminating test rather than by the
  /// closure check: a value the caller supplies at creation is ENTERED, so
  /// having no exit makes every one of the six sizes a trap.
  pub fn wp_rescope(&mut self, st: &str, seq: u32, scope: TShirt) -> Result<Outcome, FacadeError> {
    let from = self
      .st_show(st)?
      .wps
      .iter()
      .find(|w| w.seq == seq)
      // The DISPLAY form, because `from` can be three things and only one of
      // them is a size: a recorded value, a v2 value carried verbatim, or a
      // scope nobody ever wrote. The envelope records what was actually there.
      .map(|w| w.scope_display())
      .ok_or_else(|| FacadeError::NoSuchWorkPackage {
        st: st.to_string(),
        seq,
      })?;

    // **A rescope to the SAME size is only a self-loop when there is no carried
    // legacy value, and that is not a technicality.** `scope_legacy` is a v2
    // string nobody has decided about yet; rescoping resolves it. So `wp rescope
    // L` on a package already recorded `L` does nothing, while the same call on
    // one carrying `Medium-Large` alongside `L` clears the carry -- a real
    // movement of the field, with the same from and to.
    let settled = self
      .st_show(st)?
      .wps
      .iter()
      .find(|w| w.seq == seq)
      .is_some_and(|w| w.scope == Some(scope) && w.scope_legacy.is_none());
    if settled {
      // The SIZE, not the status -- this verb's field is `scope`, and reporting
      // a work package's status here would answer a question nobody asked.
      return Ok(Outcome::AlreadyThere {
        state: crate::model::enum_str(&scope),
      });
    }

    let mut next = self.canon.clone();
    let wp = find_thread_mut(&mut next, st)?
      .wps
      .iter_mut()
      .find(|w| w.seq == seq)
      .ok_or_else(|| FacadeError::NoSuchWorkPackage {
        st: st.to_string(),
        seq,
      })?;
    wp.scope = Some(scope);
    // **Rescoping RESOLVES a carried legacy value, so the carry is cleared.**
    // Leaving it would keep `Medium-Large` beside a deliberate `L` forever,
    // and a reader could not tell which one the project meant. The carry
    // exists because nobody had decided; someone just did.
    wp.scope_legacy = None;
    self
      .apply(
        "wp.rescope",
        Subject {
          kind: "wp".to_string(),
          id: format!("{st}/{seq:02}"),
        },
        // `from` is the state the work package was IN, and that can be "nobody
        // recorded one" or a carried v2 value -- so the envelope records what was
        // actually there rather than a size that would have to be invented to
        // fill the field. The event log is history; a guess in it is permanent.
        json!({"from": from, "to": crate::model::enum_str(&scope)}),
        next,
      )
      .map(|foreign| Outcome::Moved.with_overwrites(foreign))
  }

  fn set_wp_status(
    &mut self,
    st: &str,
    seq: u32,
    status: WpStatus,
    op: &'static str,
    reason: Option<&str>,
  ) -> Result<Outcome, FacadeError> {
    self.set_wp_status_fiat(st, seq, status, op, reason, None)
  }

  /// The one implementation, taking what happens to the fiat record beside the
  /// status. Same split and same reasoning as
  /// [`Facade::set_thread_status_fiat`]; the invariant lives here because this
  /// is where every work-package status write passes.
  fn set_wp_status_fiat(
    &mut self,
    st: &str,
    seq: u32,
    status: WpStatus,
    op: &'static str,
    reason: Option<&str>,
    fiat: Option<crate::model::FiatRecord>,
  ) -> Result<Outcome, FacadeError> {
    let label = format!("{st}/{seq:02}");
    let from = self
      .st_show(st)?
      .wps
      .iter()
      .find(|w| w.seq == seq)
      .map(|w| w.status)
      .ok_or_else(|| FacadeError::NoSuchWorkPackage {
        st: st.to_string(),
        seq,
      })?;

    // First, and ahead of the gate -- see `set_thread_status`.
    if from == status {
      return Ok(Outcome::AlreadyThere {
        state: from.display().to_string(),
      });
    }

    Self::check_transition(
      "WorkPackage",
      "status",
      op,
      &crate::model::enum_str(&from),
      &label,
    )?;
    self.check_gate(
      ("WorkPackage", "status", op),
      st,
      &label,
      Scope::WorkPackage(seq),
    )?;
    // The guard still refuses a verb that REQUIRES a reason and was given
    // none; its return value is no longer the thing that writes the field.
    Self::check_reason("WorkPackage", "status", op, reason)?;
    let mut next = self.canon.clone();
    let wp = find_thread_mut(&mut next, st)?
      .wps
      .iter_mut()
      .find(|w| w.seq == seq)
      .ok_or_else(|| FacadeError::NoSuchWorkPackage {
        st: st.to_string(),
        seq,
      })?;
    wp.status = status;
    // Unconditional, because the record describes the state being left -- see
    // [`Facade::set_thread_status_fiat`].
    wp.fiat = fiat.clone();
    // **WRITTEN ONLY WHEN THE CALLER SAID SOMETHING, WHICH MAKES CLEARING AN ACT
    // RATHER THAN A DEFAULT** (vc's ruling, 2026-08-25). It read
    // `wp.status_reason = reason.clone()` unconditionally, and `check_reason`
    // returns `Ok(None)` for any verb whose guard does not require one -- so
    // SEVEN verbs across two entities cleared the field on their way past:
    // `st triage|start|resume|done` and `wp start|unstart|done`.
    //
    // **THE SPLIT IS ONE DESIGN DECISION MADE TWICE, NOT A COINCIDENCE: the
    // verbs that CLEAR are the ones that move work FORWARD, and the ones that
    // pass a reason are the ones that stop or reverse it.** A reason is
    // something you supply when you interrupt, and nobody asked what happens to
    // the reason already there when you resume.
    //
    // **AND THE WORST INSTANCE IS THE CLOSING VERB, ON BOTH ENTITIES** -- this
    // row's own words: *the verb that records the outcome erases the
    // reasoning*, and *the rows nobody can afford to lose are the ones the
    // closing verb hits hardest*. Two live records were exposed, each the
    // account of a decision rather than incidental text.
    //
    // **THIS DOES NOT DECIDE THE SEMANTICS AND MUST NOT.** Whether a resumed
    // thread's hold reason is spent stays open; under this shape each call site
    // declares its own answer and can be argued one at a time. A verb that
    // genuinely spends the reason passes `Some("")` explicitly. **A fix that
    // answered the semantics on the way past would become the ruling by
    // default**, which is the failure mode this criterion produced twice today.
    if let Some(given) = reason {
      let given = given.trim();
      wp.status_reason = (!given.is_empty()).then(|| given.to_string());
    }
    // Minted here for the reason [`Facade::set_thread_status_fiat`] gives: a
    // cascade needs this envelope's id before its children's records exist.
    let envelope = Envelope::minted(
      &self.ctx.principal,
      &self.ctx.project_id,
      op,
      Subject {
        kind: "wp".to_string(),
        id: label,
      },
      json!({
        "from": crate::model::enum_str(&from),
        "to": crate::model::enum_str(&status),
        "reason": reason,
      }),
    );
    let mut envelopes = vec![envelope];
    // **SCOPED TO THIS PACKAGE'S OWN GROUP, so a package close does not reach a
    // sibling's criteria.** `Some(seq)` selects the AC group exactly as
    // [`crate::contract::Scope::WorkPackage`] does for the gate, which is why
    // this needed no new scoping rule -- the question *which criteria belong to
    // this package* was already answered and had one home.
    if let Some(record) = fiat.clone() {
      let ancestor_event = envelopes[0].id.clone();
      let ctx = self.ctx.clone();
      let ancestor = format!("{st}/{seq:02}");
      let thread = find_thread_mut(&mut next, st)?;
      envelopes.extend(Self::cascade_fiat(
        &ctx,
        thread,
        Some(seq),
        &ancestor,
        &ancestor_event,
        &record,
      ));
    }
    // Issue 0337: a close on an unwritten objective warns rather than passing in silence.
    let unwritten = status == WpStatus::Done
      && self
        .st_show(st)?
        .wps
        .iter()
        .find(|w| w.seq == seq)
        .is_some_and(|w| w.objective.trim().is_empty());
    self
      .apply_envelopes(envelopes, next, crate::store::ProjectStateEdit::Unchanged)
      .map(|foreign| {
        let moved = if unwritten {
          Outcome::MovedWith {
            notes: vec![Note::UnwrittenObjective(format!(
              "intent:///threads/{st}/wp/{seq:02}"
            ))],
          }
        } else {
          Outcome::Moved
        };
        moved.with_overwrites(foreign)
      })
  }

  // -------------------------------------------------------------------------
  // Acceptance criteria -- the four states (issue 0013)
  // -------------------------------------------------------------------------

  /// Mark a NON-TEST criterion satisfied, with its evidence.
  //
  // **This comment used to say two of the three guards were structural rather
  // than enforced, and one half of that was wrong.** The test-backed half
  // holds: a criterion in scope records [`AcState::Computed`] and there is no
  // satisfaction field to write. The evidence half did not. `Satisfied {
  // evidence: String }` makes the FIELD mandatory, not the evidence present, so
  // "satisfied with nothing to show" stayed representable -- and because this
  // comment said otherwise, no guard was written for it, the renderer reached
  // for `unwrap_or_default()`, and an AC could record Satisfied with empty
  // evidence and count toward the close gate (ic, 2026-08-15).
  //
  // It is now declared as `Guard::EvidenceRecorded` on the `ac.satisfy` edge
  // and enforced by `set_ac_state` from that declaration, with `minLength` on
  // the model carrying the same rule into the schema face. A comment asserting
  // a property is not the property, and this one was cited as the reason not to
  // build the thing that would have made it true.
  /// **CREATE A CRITERION THROUGH THE ADDRESSED SURFACE (AC-08.6).**
  ///
  /// Before this, the only route to a new criterion was a hand-edit of
  /// `.canon/st/<ID>.json` followed by `sync --to-store` -- which is how
  /// `AC-08.6` ITSELF reached canon, and how `AC-14.12` reached ST0056 the same
  /// evening. `ac` had nine subcommands and `at` five, and not one of the
  /// fourteen created anything: every arm is a transition on a row that already
  /// exists.
  ///
  /// **THIS DELEGATES TO [`Facade::put`] RATHER THAN INSERTING, AND THAT IS
  /// HIGHLANDER RATHER THAN INDIRECTION.** `put` already carries the create
  /// path, the id-agreement refusal, and the idempotence `AC-08.6` names as a
  /// falsifier; a second insert here would be a divergent copy of all three,
  /// and the one that drifts is the one the CLI actually calls. So this method
  /// owns exactly what `put` cannot know -- what an EMPTY criterion of each
  /// kind is -- and nothing else.
  ///
  /// **THE ID ORIGIN IS CALLER-ASSIGNED (WP-08), so the address is the entity's
  /// and never the collection's.** You cannot `POST` here: the caller already
  /// knows the id, and a server-assigned one would make `AC-08.6`
  /// unaddressable before the write.
  ///
  /// **AND IT IS NO LONGER IDEMPOTENT, WHICH REVERSES HALF OF ITS OWN
  /// RATIFICATION** (hv, 2026-08-28: a verb named `add`/`new` must FAIL on an
  /// existing key rather than replace it; ic ratified the idempotent reading on
  /// 2026-08-26 and a later first-hand ruling from hv supersedes it). **The
  /// caller-assigned half of that ratification is untouched and still correct**
  /// -- what changed is what a second call does, not where the id comes from.
  ///
  /// **THE REFUSAL IS HERE AND NOT IN [`Facade::put`], AND THE TWO DOORS ARE
  /// DELIBERATELY DIFFERENT.** `put` at an entity address is a `PUT`: replacing
  /// what is there is its contract, it is the shape the HTTP and GraphQL faces
  /// expose, and `ac.put` is a declared op. This is a CREATE door, and a create
  /// that replaces is the defect hv ruled on. Putting the check inside `put`
  /// would close this door by breaking the other one.
  ///
  /// **THE WINDOW THIS CHECK CANNOT CLOSE IS CLOSED BY THE THREAD WRITE**
  /// (0135). The check reads `self.canon`, loaded when the facade opened, so
  /// two facades opened before either writes both find `AC-01.1` free -- and
  /// `store::Door::Create` cannot help, with no per-child UNIQUE key to fire
  /// on. What refuses the second is `commit_mutation`'s compare-and-swap
  /// (`5ef0667c`): the thread it derived from has changed, so the write is
  /// `RecordMovedUnderTheWrite`. This check still earns its place: it names
  /// the taken key for the whole single-writer population, which is every
  /// operator at a terminal.
  ///
  /// The state is DERIVED FROM THE KIND rather than taken as an argument,
  /// because the two are not independent: a test-backed criterion in scope
  /// records [`AcState::Computed`] -- satisfaction is derived from covering
  /// green ATs and there is no field to write -- while a non-test one starts
  /// [`AcState::Unsatisfied`]. Letting a caller pass `Satisfied` here would
  /// mint a criterion that was born already met, with no evidence and no
  /// decision behind it.
  pub fn ac_new(
    &mut self,
    st: &str,
    ac: &str,
    text: &str,
    kind: AcKind,
  ) -> Result<Outcome, FacadeError> {
    if self.st_show(st)?.criteria.iter().any(|c| c.id == ac) {
      return Err(FacadeError::CriterionExists {
        st: st.to_string(),
        ac: ac.to_string(),
      });
    }
    let row = Criterion {
      id: ac.to_string(),
      text: text.to_string(),
      kind,
      state: AcState::entry(kind),
    };
    let body = serde_json::to_string(&row).map_err(|e| FacadeError::WriteNotAddressable {
      url: format!("intent:///threads/{st}/ac/{ac}"),
      why: format!("the criterion could not be serialised: {e}"),
    })?;
    self.put(
      &Address {
        authority: None,
        entity: AddrEntity::Ac {
          thread: st.to_string(),
          ac: ac.to_string(),
        },
        format: None,
      },
      &body,
    )
  }

  /// **CREATE AN ACCEPTANCE TEST THROUGH THE ADDRESSED SURFACE (AC-08.7).**
  ///
  /// Recorded separately from [`Facade::ac_new`] rather than folded into it
  /// because an AT row carries `file`, `covers` and `status`, so its create has
  /// a validity question a criterion's does not.
  ///
  /// **THE CREATED ROW IS HELD TO THE GRAMMAR `at lint` ENFORCES ON EVERY
  /// OTHER ROW, AND THAT IS THE HALF THAT MAKES THIS MORE THAN A `push`.**
  /// `AC-08.7` names bypassing it as a falsifier, and the hole is real rather
  /// than theoretical: L2 refuses a row citing a file that does not exist, L3 a
  /// file that does not carry the row's literal id, L4 a `covers` naming no
  /// criterion, and L5 a non-test AT covering a test-backed criterion. A create
  /// that skipped them could mint, in one call, a row that `at lint` and the
  /// close-gate would then both refuse -- so the verb would report `ok:` and
  /// leave the thread ungateable.
  ///
  /// **THE CHECK RUNS ON THE PROSPECTIVE THREAD, BEFORE THE WRITE.** Writing
  /// first and linting after would leave the bad row in canon on refusal, which
  /// is the shape `issues hydrate` was retired for: a durable claim left behind
  /// by a call that reported failure.
  ///
  /// `to-write` is exempt from L2/L3 by the contract's own rule, not by an
  /// exception here -- a row whose test has not been written yet cites a file
  /// that is SUPPOSED not to exist, and this repository's commit gate checks
  /// for the opposite case.
  #[allow(clippy::too_many_arguments)]
  pub fn at_new(
    &mut self,
    st: &str,
    at: &str,
    kind: AtKind,
    file: Option<String>,
    prose: Option<String>,
    covers: Vec<String>,
    note: Option<String>,
  ) -> Result<Outcome, FacadeError> {
    // **THE REFUSAL, AND IT RETIRES A MITIGATION RATHER THAN JOINING IT.** This
    // verb used to READ the stored row and carry its `note` and `legacy`
    // forward, because a create on an existing id was a full replacement and
    // every field the verb did not set was a field it destroyed -- measured on
    // this repository as six ST0061 notes eaten by a single re-cite and
    // recovered from a git blob.
    //
    // **That carry is now unreachable and is gone.** hv ruled that a create
    // must fail on an existing key, so there is no stored row here to carry
    // anything from; leaving the read in would be dead code that reads as live
    // protection. **The preservation moved to [`Facade::at_edit`], where it is
    // the verb's whole purpose rather than a patch over the wrong door** -- and
    // that is the half that matters, because the trap was never the replacement
    // on its own. It was that the safe route (a canon edit plus
    // `sync --to-store`) was the undocumented one and the verb built for the
    // job was the one that lost the note.
    if let Some(existing) = self.st_show(st)?.tests.iter().find(|t| t.id == at) {
      return Err(FacadeError::TestExists {
        st: st.to_string(),
        at: at.to_string(),
        kind: existing.kind,
      });
    }

    // **A ROW IS CREATED AT ITS KIND'S ENTRY, AND NO OTHER STATUS CAN BE ASKED
    // FOR** (vc, 2026-09-14 and 2026-09-15, issues 0324, 0337 and 0339, the
    // mirror of `ac_new` taking `AcState::entry`). A create that could name a
    // status could name `green`, and green is reachable only from red: a row
    // nobody had seen fail would read as passing. So the status is not a
    // parameter, and a verdict is recorded afterwards with `at red|green|na`.
    let status = AtStatus::entry(kind);

    let row = AcceptanceTest {
      id: at.to_string(),
      kind,
      file,
      prose,
      covers,
      status,
      // `None` for the same reason `legacy` is, one line down: a row created
      // here has no past. A fiat close is an act performed ON an existing AT,
      // and `at.fc` is the only thing that writes this field.
      fiat: None,
      note,
      // **`None` because this row is NEW, which is now a fact rather than an
      // assumption.** `legacy` marks a reference carried whole from a v2 estate
      // under the closed-thread carry policy; a row created here has no v2
      // ancestor to carry. It used to be carried from the stored row for the
      // same reason `note` was -- to survive a replace -- and the refusal above
      // means there is no stored row to reach.
      legacy: None,
    };

    self.refuse_unless_the_row_holds_the_contract(st, &row)?;

    let body = serde_json::to_string(&row).map_err(|e| FacadeError::WriteNotAddressable {
      url: format!("intent:///threads/{st}/at/{at}"),
      why: format!("the acceptance test could not be serialised: {e}"),
    })?;
    self.put(
      &Address {
        authority: None,
        entity: AddrEntity::At {
          thread: st.to_string(),
          at: at.to_string(),
        },
        format: None,
      },
      &body,
    )
  }

  /// **DOES THIS ROW INTRODUCE AN ACCEPTANCE-CONTRACT FINDING THAT IS NOT
  /// THERE ALREADY?**
  ///
  /// The check `at_new` was born with, extracted the moment `at_edit` needed
  /// the same question asked -- one home rather than two, because a copy would
  /// drift toward whichever door was exercised more and show up as a row one
  /// verb accepts and the other refuses.
  ///
  /// **IT ASKS ABOUT THE PROSPECTIVE THREAD -- what canon WOULD hold if this
  /// write landed** -- so a refusal costs nothing and a pass is a statement
  /// about the row being written rather than about the one before it. Writing
  /// first and linting after would leave the bad row in canon on refusal, which
  /// is the shape `issues hydrate` was retired for: a durable claim left behind
  /// by a call that reported failure.
  ///
  /// The grammar is `at lint`'s, not a second copy of it: L2 refuses a row
  /// citing a file that does not exist, L3 a file that does not carry the row's
  /// literal id, L4 a `covers` naming no criterion, L5 a non-test AT covering a
  /// test-backed criterion. `to-write` is exempt from L2/L3 by the contract's
  /// own rule -- a row whose test is not written yet cites a file that is
  /// SUPPOSED not to exist.
  ///
  /// # It compares BEFORE against AFTER, and the create door is why that is one
  /// rule rather than two
  ///
  /// `at_new` refused any finding naming the row, which is right for a create
  /// and **would have made `at_edit` unable to perform the repair it exists
  /// for.** A row whose test file has been deleted already carries an L2
  /// finding; under a refuse-any rule, every field on that row becomes
  /// uneditable at exactly the moment someone needs to fix it -- the "a verb
  /// that refuses on somebody else's defect is one nobody can use" problem, one
  /// row down and pointed at the row's own inherited breakage.
  ///
  /// So the rule is **you may not make this row worse**, and it needs no door
  /// parameter: a create's before-set is empty by construction, because
  /// [`FacadeError::TestExists`] has already refused the call if the id is
  /// taken. The create keeps the behaviour it had, the edit gets the one it
  /// needs, and there is one predicate to reason about.
  ///
  /// **What this gives up, stated rather than discovered:** an edit may leave a
  /// row still carrying a finding it arrived with. That is deliberate -- the
  /// alternative is refusing a partial repair, and `at lint` is the door that
  /// reports inherited breakage.
  fn refuse_unless_the_row_holds_the_contract(
    &self,
    st: &str,
    row: &AcceptanceTest,
  ) -> Result<(), FacadeError> {
    let current = self.st_show(st)?.clone();
    let mut prospective = current.clone();
    match prospective.tests.iter_mut().find(|t| t.id == row.id) {
      Some(existing) => *existing = row.clone(),
      None => prospective.tests.push(row.clone()),
    }
    prospective.tests.sort_by(|a, b| a.id.cmp(&b.id));

    // NARROWED TO THE ROW BEING WRITTEN, and the narrowing is the point rather
    // than politeness: a thread carrying a finding about a DIFFERENT row would
    // otherwise make every write on it impossible.
    let about_this_row = |t: &Thread| -> Vec<String> {
      contract::contract_report(t, None, &contract::RepoFiles(self.project.root()))
        .findings
        .into_iter()
        .filter(|f| f.contains(&row.id))
        .collect()
    };
    let before = about_this_row(&current);
    let introduced: Vec<String> = about_this_row(&prospective)
      .into_iter()
      .filter(|f| !before.contains(f))
      .collect();

    if !introduced.is_empty() {
      return Err(FacadeError::RowBreaksContract {
        st: st.to_string(),
        at: row.id.clone(),
        findings: introduced.join("; "),
      });
    }
    Ok(())
  }

  /// **REWORD A CRITERION, AND CHANGE NOTHING ELSE ABOUT IT.**
  ///
  /// The other half of hv's 2026-08-28 ruling, and it ships in the same change
  /// as the refusal rather than after it. **Refusing a create on an existing id
  /// without this verb would leave an AC sentence unwritable by any door in the
  /// tool** -- `ac` had nine subcommands and not one of them could touch the
  /// text -- so the two halves are one change or the estate is stranded. ic
  /// measured that gap; hv took the re-raise.
  ///
  /// **`text` IS THE WHOLE SURFACE, AND `kind` IS DELIBERATELY ABSENT.**
  /// Changing a criterion between test-backed and non-test moves where its
  /// satisfaction comes from -- computed from covering green ATs, or stored
  /// with evidence -- so it edits the contract graph, not a sentence. That is a
  /// state change wearing an edit verb's clothes, and it belongs in a ruling
  /// rather than in a flag added while building something else. **So a kind
  /// change is still stuck after this lands**, and the transition table already
  /// says so: `Criterion.kind` is `Unbuilt`, owed, and named.
  ///
  /// **THE STATE IS UNTOUCHED, WHICH IS THE PROPERTY THAT MAKES THIS A REPAIR
  /// RATHER THAN A SECOND WAY TO LOSE WORK.** `ac new` on an existing row reset
  /// the state to the kind's entry value, so repairing one sentence of a
  /// satisfied criterion silently discarded its evidence. This reaches one
  /// field and cannot reach that one.
  ///
  /// **`note` IS THE ONE PART OF A STATE IT WRITES, AND ONLY ON AN UNSATISFIED
  /// ROW (0140).** An unsatisfied note was published on both faces and written
  /// only by the v2 ingest, so a v3-native estate could not reach a state a
  /// migrated one arrives in. It is [`Facade::at_edit`]'s `--note`: it replaces
  /// the note outright, and a field not named is not changed. The variant never
  /// moves; a satisfied, descoped, withdrawn or computed row carries its own
  /// record rather than a note, so a note there is refused by name and the
  /// lifecycle verbs are the way to it.
  pub fn ac_edit(
    &mut self,
    st: &str,
    ac: &str,
    text: Option<String>,
    note: Option<String>,
  ) -> Result<Outcome, FacadeError> {
    // Refused here rather than in the renderer, for `at_edit`'s reason: a
    // caller naming no field believes they changed something, and a library
    // caller passing two `None`s makes the same mistake.
    if text.is_none() && note.is_none() {
      return Err(FacadeError::NothingToChange {
        subject: format!("{st} {ac}"),
        offered: vec!["--text".to_string(), "--note".to_string()],
      });
    }
    // `criterion` raises `NoSuchCriterion`, which is the refusal an edit owes:
    // an edit that CREATED on a missing id would be the create door wearing the
    // other name, and both doors would then be able to make a row.
    let existing = self.criterion(st, ac)?.clone();
    if note.is_some() && !matches!(existing.state, AcState::Unsatisfied { .. }) {
      let state = existing.state.name();
      return Err(FacadeError::FieldNotWritable {
        url: format!("intent:///threads/{st}/ac/{ac}"),
        field: "note".to_string(),
        why: format!(
          "{ac} is {state}, and only an unsatisfied criterion carries a note -- a {state} row \
           keeps its own record, so move it with `intent ac unsatisfy|rescope|reinstate` first"
        ),
      });
    }
    let mut row = existing.clone();
    if let Some(text) = &text {
      row.text = text.clone();
    }
    if let Some(note) = &note {
      row.state = AcState::Unsatisfied {
        note: Some(note.clone()),
      };
    }
    if row == existing {
      return Ok(Outcome::AlreadyThere {
        state: "unchanged".to_string(),
      });
    }
    let mut payload = serde_json::Map::new();
    if let Some(text) = text {
      payload.insert("text".to_string(), json!(text));
    }
    if let Some(note) = note {
      payload.insert("note".to_string(), json!(note));
    }
    let mut next = self.canon.clone();
    *find_criterion_mut(&mut next, st, ac)? = row;
    self
      .apply(
        "ac.edit",
        Subject {
          kind: "ac".to_string(),
          id: format!("{st}/{ac}"),
        },
        Value::Object(payload),
        next,
      )
      .map(|foreign| Outcome::Moved.with_overwrites(foreign))
  }

  /// **RE-CITE AN ACCEPTANCE TEST: CHANGE WHAT YOU NAME, KEEP WHAT YOU DO
  /// NOT.**
  ///
  /// **THIS IS THE VERB THE SIX ST0061 NOTES NEEDED.** A re-cite through
  /// `at new` was a full replacement, so moving a test to its new file ate the
  /// row's `note` in the same call -- and the safe route was a hand-edit of
  /// canon plus `sync --to-store`, ie the undocumented one. The verb built for
  /// the job was the one that lost the work. Here absence is silence: a field
  /// nobody named is a field nobody changed.
  ///
  /// **`status` IS ABSENT AND `kind` IS NOT, AND THE SPLIT IS hv's RULING OF
  /// 2026-09-07 RATHER THAN A DRIFT.** `status` has a door already -- `at
  /// green` / `at red` / `at na`, which is a declared state machine with an
  /// envelope per movement -- and a second way to write it would be a
  /// divergent copy of a transition. That reason is untouched.
  ///
  /// **`kind` USED TO BE ABSENT FOR [`Facade::ac_edit`]'s REASON -- it moves
  /// the contract graph -- AND THAT REASONING WAS SOUND WHILE ITS COST WAS
  /// UNMEASURED.** The fleet audit measured it: the v2 migrator reads
  /// `- AT-01.1 (legacy) [non-test: capture-spec doc review]` and records
  /// `kind: Test`, because the bracket form is not the `(non-test)` marker the
  /// v3 grammar looks for. The row's own `legacy.raw` then says `non-test`
  /// while its `kind` says otherwise, `doctor` reports the disagreement
  /// forever, and **there was no legal spelling anywhere in the tool that could
  /// correct it**: this verb excluded `kind`, `at lint --fix` is deliberately
  /// not carried over from v2, and `sync --to-store` reads the canon extract
  /// rather than the markdown. Six rows on Baize, permanently.
  ///
  /// **A STATE THE TOOL CAN CREATE AND CANNOT REPAIR IS THE DEFECT**, and it
  /// outweighs the graph-motion argument -- which is answered rather than
  /// ignored: the prospective row still goes through
  /// `refuse_unless_the_row_holds_the_contract` like every other field, so a
  /// re-kind that introduces a contract finding is refused with nothing
  /// written.
  ///
  /// **AND THAT CHECK IS NOT ENOUGH ON ITS OWN, WHICH A NEGATIVE CONTROL
  /// CAUGHT RATHER THAN A READING.** `contract_report` does not ask whether a
  /// row's status suits its kind -- `AtStatus::permitted_for` is enforced in
  /// `doctor` alone, which that check's own comment calls "ONE of its three
  /// enforcement points, not three ... it reports rather than prevents". So
  /// the contract check let `--kind test` land on an `n-a` row: the exact
  /// defect this flag exists to REMOVE, reachable through the flag itself.
  ///
  /// **THE PRECONDITION BELOW IS SCOPED TO THIS FLAG DELIBERATELY, AND IS NOT
  /// THE GENERAL GUARD.** That same comment records why the general one is a
  /// TABLE amendment rather than more code: `at_set` writes any status onto
  /// any row, the guard belongs on Machine 5's four `at.set` edges, hv
  /// ratified that table on 2026-08-29 with `--` in every Guard cell, and
  /// "enforcing it in the verb body without the declaration is the shape
  /// `data-model.md` already names and rejects". **That ruling stands and the
  /// `at.set` hole stays open -- it is hv's to close.** What is refused here
  /// is narrower and needs no table: a NEW door declining to manufacture a
  /// state its own estate calls a defect. Nothing existing becomes stricter.
  ///
  /// **THE CONTRACT IS RE-ASKED ON THE PROSPECTIVE ROW**, so a re-cite to a
  /// file that does not exist, or to one not carrying the row's id, is refused
  /// with nothing written -- the same grammar a create is held to, through the
  /// same one check.
  // **SUPPRESSED WITH A REASON, AND THE REASON IS THAT THE REAL FIX IS NOT A
  // LINT FIX.** `clippy::too_many_arguments` fires at 8/7 and it is not wrong:
  // five optional edit fields in a positional list is a shape that invites a
  // transposed call. The honest repair is an `AtEdits` struct, which touches
  // THIRTEEN call sites and changes a WP-04 facade signature -- a refactor
  // wearing a lint fix's clothes, and the way a small change becomes
  // unreviewable. Routed as a proposal to whoever owns the at/ac facade rather
  // than taken here while clearing a CI gate. Remove this attribute with that
  // change, not before.
  #[allow(clippy::too_many_arguments)]
  pub fn at_edit(
    &mut self,
    st: &str,
    at: &str,
    file: Option<String>,
    prose: Option<String>,
    covers: Option<Vec<String>>,
    note: Option<String>,
    kind: Option<AtKind>,
  ) -> Result<Outcome, FacadeError> {
    // **REFUSED RATHER THAN TREATED AS A NO-OP, AND IT IS REFUSED HERE RATHER
    // THAN IN THE RENDERER.** `at edit ST0001 AT-01.1` with no field named is a
    // caller who believes they changed something; answering `unchanged` at exit
    // 0 tells them they succeeded. The check lives at the facade because the
    // facade is the contract -- a library caller passing three `None`s makes
    // the identical mistake, and a renderer-side guard would protect only the
    // operators who came through the CLI.
    if file.is_none() && prose.is_none() && covers.is_none() && note.is_none() && kind.is_none() {
      return Err(FacadeError::NothingToChange {
        subject: format!("{st} {at}"),
        offered: vec![
          "--file".to_string(),
          "--prose".to_string(),
          "--covers".to_string(),
          "--note".to_string(),
          "--kind".to_string(),
        ],
      });
    }

    let existing = self
      .st_show(st)?
      .tests
      .iter()
      .find(|t| t.id == at)
      .ok_or_else(|| FacadeError::NoSuchTest {
        st: st.to_string(),
        at: at.to_string(),
      })?;

    // The prospective row: the stored one with the named fields moved over it.
    // **Every field not named is READ OFF THE STORED ROW rather than defaulted**
    // -- which is the whole difference between this verb and the create it
    // replaces, and the reason `note` and `legacy` need no special handling.
    // **`note` CARRIES NO SHRINKAGE REFUSAL HERE, AND THE ASYMMETRY IS THE
    // MECHANISM RATHER THAN AN OVERSIGHT** (issue 0207, vc ruled (c)
    // 2026-09-02, rider 1). **A door that refuses is not a door.** The status
    // verbs refuse a note that does not contain the one it replaces, so this
    // is the only way to fold a bloated note deliberately -- guarding it too
    // would leave no legal spelling at all, which is the build 0207's own
    // evidence disproves (it records one destructive shrink AND three benign
    // folds the same day).
    //
    // **NAMING THE FIELD IS THE DELIBERATE ACT**, which is this verb's
    // published contract already: *a field you do not name is a field it does
    // not change*. 0207 records that `at edit` exists precisely so a re-cite
    // does not eat a row's note -- **the reasoning was applied to this verb
    // and stopped one field short.**
    // **AND `legacy` IS THE ONE FIELD THAT CANNOT BE READ OFF THE STORED ROW,
    // BECAUSE A RE-CITE IS WHAT RETIRES IT** (issue `0314`, ruled by vc
    // 2026-09-13). `doctor`'s advisory on a migrated row asks for exactly this
    // rewrite -- *worth rewriting in the v3 grammar next time the thread is
    // touched* -- and carrying `legacy` across it left the row citing a test
    // file AND holding a legacy reference, which `doctor` then reported as
    // BLOCKING: *these are alternatives, not a pair*. There is no `at rm` and
    // `at new` refuses a taken id, so a migrated row could not be expressed in
    // the v3 grammar at all: the verb the remedy named was the one that made
    // the estate worse.
    //
    // **NAMING `--file` OR `--prose` IS THE DELIBERATE ACT**, which is this
    // verb's own contract read the right way round: the citation is what
    // `legacy` holds an older spelling of, so re-citing it REPLACES it, while a
    // `--note` or `--covers` edit touches neither and leaves it exactly where
    // it is. **The `--kind` re-entry below serves the same rule:
    // a verb must not create the disagreement `doctor` reports.**
    let recited = file.is_some() || prose.is_some();
    let mut row = AcceptanceTest {
      file: file.or_else(|| existing.file.clone()),
      prose: prose.or_else(|| existing.prose.clone()),
      covers: covers.unwrap_or_else(|| existing.covers.clone()),
      note: note.or_else(|| existing.note.clone()),
      kind: kind.unwrap_or(existing.kind),
      legacy: if recited {
        None
      } else {
        existing.legacy.clone()
      },
      ..existing.clone()
    };

    // **A RE-KIND RE-ENTERS THE STATUS AT `AtStatus::entry`** (vc, 2026-09-14,
    // issues 0324 and 0337, the mirror of `Facade::rekinded_state`). This used
    // to refuse a kind the row's status could not hold and send the caller to a
    // verdict first; once a verdict had to fit its kind, that route closed in
    // both directions. `AtStatus::permitted_for` decides, as `doctor` does.
    //
    // Only a caller who NAMED `--kind` moves the status: a row already carrying
    // the disagreement is left as it is under a `--note` or `--covers` edit.
    if kind.is_some() && !row.status.permitted_for(row.kind) {
      row.status = AtStatus::entry(row.kind);
    }
    Self::refuse_a_file_written_onto_a_non_test_row(st, existing, &row)?;
    if &row == existing {
      return Ok(Outcome::AlreadyThere {
        state: "unchanged".to_string(),
      });
    }

    self.refuse_unless_the_row_holds_the_contract(st, &row)?;

    let mut next = self.canon.clone();
    *find_test_mut(&mut next, st, at)? = row;
    self
      .apply(
        "at.edit",
        Subject {
          kind: "at".to_string(),
          id: format!("{st}/{at}"),
        },
        json!({ "via": "edit" }),
        next,
      )
      .map(|foreign| Outcome::Moved.with_overwrites(foreign))
  }

  /// **A WRITE THAT PUTS A `file` ON A NON-TEST ROW IS REFUSED** (0146), and
  /// this is the one home both `at edit` and `intent set` call.
  ///
  /// A non-test row asserts prose INSTEAD of a file, and `at edit --file`
  /// answered rc=0 and left one carrying both. **Judged on what the call
  /// WRITES -- the file moving -- never on the row as found**, so a row already
  /// carrying both stays editable, and `--kind test --file` in one call passes
  /// because the row it leaves is a test row. `--prose` on a test row is not
  /// refused: it is in deliberate use as the note on a row with no file yet.
  fn refuse_a_file_written_onto_a_non_test_row(
    st: &str,
    before: &AcceptanceTest,
    after: &AcceptanceTest,
  ) -> Result<(), FacadeError> {
    let file = after.file.as_deref().unwrap_or_default();
    if matches!(after.kind, AtKind::NonTest) && after.file != before.file && !file.is_empty() {
      return Err(FacadeError::FileOnANonTestRow {
        st: st.to_string(),
        at: after.id.clone(),
        file: file.to_string(),
      });
    }
    Ok(())
  }

  pub fn ac_satisfy(&mut self, st: &str, ac: &str, evidence: &str) -> Result<Outcome, FacadeError> {
    let criterion = self.criterion(st, ac)?;
    if criterion.kind != AcKind::NonTest {
      return Err(FacadeError::ComputedSatisfaction { ac: ac.to_string() });
    }
    // v2 refuses this and v3 had stopped: on a descoped criterion, satisfy
    // printed `ok:`, exited 0, and wrote a row that read as both descoped and
    // satisfied, while `ac list` and the gate went on correctly reporting it
    // descoped. Reported success, no effect -- the issue-0006 shape, reachable
    // through the verbs added to fix issue 0013 (bin/intent_acceptance:117-127).
    Self::refuse_if_off_scope(criterion, ac, "satisfied")?;
    self.set_ac_state(
      st,
      ac,
      AcState::Satisfied {
        evidence: evidence.to_string(),
      },
      "ac.satisfy",
      json!({"evidence": evidence}),
    )
  }

  /// Reopen a non-test criterion: unsatisfied, and its evidence gone with it.
  ///
  /// **The inverse `ac.satisfy` never had.** hv ruled on this instance directly
  /// (D32, AC-04.6): satisfy was a one-way door, so a verifier whose evidence
  /// proved incomplete had to hand-edit `acceptance.md` -- the file this command
  /// exists to own. A state that can be entered and not left is a missing
  /// mutation, not a missing flag.
  ///
  /// **The evidence going with it used to be the design content here, and the
  /// collapse absorbed it.** Clearing satisfaction while leaving the evidence
  /// behind would produce a criterion that reads as unsatisfied and still cites
  /// the proof that was withdrawn -- a worse lie than the one-way door, because
  /// it looks like a record. That was a rule two assignments had to keep; now
  /// the evidence lives INSIDE `Satisfied`, so leaving it behind is not a thing
  /// the type can do.
  /// Withdraw a satisfaction, clearing the evidence with it.
  ///
  /// **This REFUSED A LEGAL SELF-LOOP until 2026-08-17, and the mechanism is
  /// 0051's, not a typo.** It carried its own from-state check --
  /// `if !matches!(state, Satisfied { .. }) { NotSatisfied }` -- ahead of
  /// delegating, so `ac unsatisfy` on an already-unsatisfied criterion exited 1
  /// while hv's ruling makes it a self-loop at 0. **A hand-written copy of a
  /// from-state the table already declares, running ahead of the shared setter's
  /// self-loop test**, which is exactly what `Guard::GatePass` was doing at two
  /// call sites. Found by driving the verb twice through the real binary
  /// (`self_loop_voice.rs`); reading the code did not find it, and neither did
  /// `mutation_completeness.rs`, whose walk only ever drives an edge from a state
  /// it IS declared from.
  ///
  /// **The refusal is preserved rather than lost, and mapped rather than
  /// duplicated.** `ac.unsatisfy` is declared from `satisfied` and nowhere else,
  /// so every `IllegalTransition` this verb can produce means "not satisfied" --
  /// the mapping is equivalent by construction, and `NotSatisfied`'s remedy names
  /// where to look where `IllegalTransition`'s names only the state.
  ///
  /// **The kind check STAYS ahead of the delegation, and it cannot mis-refuse a
  /// self-loop.** A test-backed criterion is always `Computed` (the kind/state
  /// pairing is enforced in the schema face), so it can never be at this verb's
  /// target -- meaning there is no self-loop for this guard to shadow.
  /// `ComputedSatisfaction` is a better answer for that case than either
  /// alternative, which is why it is not delegated to `Guard::NonTestOnly`.
  pub fn ac_unsatisfy(&mut self, st: &str, ac: &str) -> Result<Outcome, FacadeError> {
    let criterion = self.criterion(st, ac)?;
    if criterion.kind != AcKind::NonTest {
      return Err(FacadeError::ComputedSatisfaction { ac: ac.to_string() });
    }
    self
      .set_ac_state(
        st,
        ac,
        AcState::Unsatisfied { note: None },
        "ac.unsatisfy",
        json!({}),
      )
      .map_err(|cause| match cause {
        FacadeError::IllegalTransition { .. } => FacadeError::NotSatisfied { ac: ac.to_string() },
        other => other,
      })
  }

  pub fn ac_descope(
    &mut self,
    st: &str,
    ac: &str,
    to: &str,
    by: Option<&str>,
    reason: Option<&str>,
  ) -> Result<Outcome, FacadeError> {
    // **The ratified machine guards this with "target thread exists", and it
    // was declared and unenforced.** `doctor` already reports the resulting
    // state -- "descoped to X, which is not a steel thread in this project" --
    // so the estate has been DETECTING a condition it could refuse, which is
    // the reminder-shaped thing D33 rules against. Refusing costs one
    // workflow: descoping to a thread you intend to create next. That is a
    // real cost and it is flagged to vc rather than absorbed silently.
    // **An ABSENT target is not a target that does not exist**, and reporting
    // it as one produces a message with a hole in it: "cannot descope AC-01.1
    // to , which is not a steel thread in this project", with the same gap
    // repeated in the remedy. Different mistakes need different refusals, which
    // is the same reason `EvidenceRequired` is not `ReasonRequired` with a word
    // swapped. Clap now refuses an absent `--to` from the declared `required`,
    // so what reaches here is an empty or blank one -- the "empty is not
    // absent" case that the whole evidence defect turned on, one verb over.
    if to.trim().is_empty() {
      return Err(FacadeError::DescopeTargetRequired { ac: ac.to_string() });
    }
    if !self.canon.threads.iter().any(|t| t.id == to) {
      return Err(FacadeError::DescopeTargetMissing {
        ac: ac.to_string(),
        to: to.to_string(),
      });
    }
    self.set_ac_state(
      st,
      ac,
      AcState::Descoped {
        to: to.to_string(),
        by: by.map(str::to_string),
        reason: reason.map(str::to_string),
      },
      "ac.descope",
      json!({"to": to}),
    )
  }

  pub fn ac_withdraw(
    &mut self,
    st: &str,
    ac: &str,
    reason: &str,
    by: Option<&str>,
  ) -> Result<Outcome, FacadeError> {
    self.set_ac_state(
      st,
      ac,
      AcState::Withdrawn {
        reason: reason.to_string(),
        by: by.map(str::to_string),
      },
      "ac.withdraw",
      json!({"reason": reason}),
    )
  }

  /// **Close a criterion on human authority, against the evidence** -- the
  /// entry door to `AcState::Fiat`, and the one place in this facade that
  /// records a close nobody claims was earned.
  ///
  /// **`at` GOES IN EMPTY AND COMES BACK FILLED** (D42), exactly as `st new`'s
  /// `created` does. Nothing here knows what time it is: the database stamps
  /// the event inside the INSERT and `Facade::apply_with_state` patches the
  /// value that landed back into the record before the extract is rendered.
  /// Reading a process clock here is the defect hv ruled out on 2026-08-15 --
  /// it would make the log's ordering an accident of which machine ran the
  /// command.
  ///
  /// **`inherited_from` is `None` at this door because this door is the
  /// individual judgement.** A cascaded close is a different act and carries
  /// the ancestor that reached it; that is what stops a cascaded row reading as
  /// one somebody actually looked at.
  ///
  /// **The empty-`because` refusal is NOT written here.** It is the machine's
  /// `Guard::ReasonRecorded`, read through `Self::justification`, so the
  /// guard, the schema's `length(min = 1)` and this verb cannot drift apart --
  /// there is only one of them.
  pub fn ac_fc(
    &mut self,
    st: &str,
    ac: &str,
    because: &str,
    by: &str,
  ) -> Result<Outcome, FacadeError> {
    // **`refuse_if_off_scope` IS DELIBERATELY NOT CALLED HERE, AND THE REASON
    // IS RECORDED BECAUSE THE OMISSION LOOKS LIKE ONE.** Every sibling that
    // writes about satisfaction calls it, so a fiat close on a `withdrawn`
    // criterion answering with a bare legal-transition list is a real
    // inconsistency -- it just cannot be fixed from here. `OffScope`'s remedy
    // hardcodes the evidence story ("recording evidence for a requirement
    // nobody is working on is the bookkeeping descope replaced"), which is a
    // sentence about `ac satisfy` and false about this verb, and its `{verb}`
    // slot wants an infinitive that "fiat-closed" is not.
    //
    // **Measured, not predicted: wiring it produced `if you mean to
    // fiat-closed it` followed by a paragraph about evidence** -- worse output
    // than the raw refusal it replaced. The remedy's wording is the surface's
    // to own, so the gap is reported to ic rather than patched here with a
    // second nearly-identical error. Right shape, wrong sentence, which is the
    // usual way a reused error fails.
    self
      .set_ac_state(
        st,
        ac,
        AcState::Fiat(crate::model::FiatRecord {
          because: because.to_string(),
          by: by.to_string(),
          at: String::new(),
          invoker: crate::model::Invoker::collected(),
          inherited_from: None,
          inherited_event: None,
        }),
        "ac.fc",
        json!({"because": because, "by": by}),
      )
      // **MAPPED AFTER THE SETTER, NEVER CHECKED BEFORE IT** -- issues 0051 and
      // 0053, whose mechanism was a hand-written from-state `match` placed
      // ahead of `set_ac_state`, making the shared self-loop arm unreachable
      // for that verb. `ac_reinstate` carries the same shape for the same
      // reason. With `descoped` and `withdrawn` handled above and the machine
      // declaring `ac.fc` from the two open states, an `IllegalTransition`
      // surviving to here means the criterion is already fiat-closed or already
      // satisfied, and only the first has an undo to name.
      .map_err(
        |cause| match (cause, &self.criterion(st, ac).map(|c| c.state.clone())) {
          (FacadeError::IllegalTransition { .. }, Ok(AcState::Fiat(record))) => {
            FacadeError::AlreadyFiatClosed {
              subject: ac.to_string(),
              because: record.because.clone(),
              undo: format!("intent ac reinstate <thread> {ac}"),
            }
          }
          (other, _) => other,
        },
      )
  }

  /// Undo a WITHDRAWAL: back in scope, and back to whatever "in scope" MEANS
  /// for this criterion's kind.
  ///
  /// It refuses a descoped criterion and names `rescope` instead, because v2
  /// does (`bin/intent_acceptance:1246`) and because the two are genuinely
  /// different acts: a descoped requirement still exists somewhere else, and a
  /// withdrawn one does not exist at all. Treating them as one verb would make
  /// the tool answer "done" to a question it had not been asked.
  /// **The `_` arm DELEGATES rather than refusing ahead of the setter** -- issue
  /// 0053, and this is 0051's mechanism in its third and fourth instances. A
  /// hand-written from-state check placed before `set_ac_state` makes the shared
  /// self-loop test unreachable for that verb, so `intent ac reinstate` on an
  /// in-scope criterion exited 1 where hv's ruling makes it a self-loop at 0.
  /// Both survivors sat twenty lines below the copy fixed in `ac_unsatisfy`, in
  /// the same file, in the same commit -- found by ic driving the binary twice
  /// rather than by reading the enumeration.
  ///
  /// **The mapping is equivalent by construction.** `ac.reinstate` declares its
  /// edges only from `withdrawn`, and `Descoped` is handled above, so every
  /// remaining `IllegalTransition` means exactly "in scope" -- while a criterion
  /// already AT the entry state now reaches the self-loop arm instead of being
  /// refused before it.
  pub fn ac_reinstate(&mut self, st: &str, ac: &str) -> Result<Outcome, FacadeError> {
    let criterion = self.criterion(st, ac)?;
    let entry = AcState::entry(criterion.kind);
    match &criterion.state {
      AcState::Descoped { .. } => Err(FacadeError::WrongOffScopeState {
        ac: ac.to_string(),
        actual: "descoped".to_string(),
        wanted: "withdrawn".to_string(),
        verb: "rescope".to_string(),
      }),
      _ => self
        .set_ac_state(st, ac, entry, "ac.reinstate", json!({}))
        .map_err(|cause| Self::in_scope(cause, ac, "reinstate", "withdrawn")),
    }
  }

  /// Undo a DESCOPE. The mirror of [`Facade::ac_reinstate`], refusing a
  /// withdrawn criterion the same way -- and self-looping the same way.
  pub fn ac_rescope(&mut self, st: &str, ac: &str) -> Result<Outcome, FacadeError> {
    let criterion = self.criterion(st, ac)?;
    let entry = AcState::entry(criterion.kind);
    match &criterion.state {
      AcState::Withdrawn { .. } => Err(FacadeError::WrongOffScopeState {
        ac: ac.to_string(),
        actual: "withdrawn".to_string(),
        wanted: "descoped".to_string(),
        verb: "reinstate".to_string(),
      }),
      _ => self
        .set_ac_state(st, ac, entry, "ac.rescope", json!({}))
        .map_err(|cause| Self::in_scope(cause, ac, "rescope", "descoped")),
    }
  }

  /// The refusal the two undo verbs used to raise before the setter, raised
  /// AFTER it instead so the self-loop is reachable.
  ///
  /// One home for both, because the two call sites differ only in two words and
  /// the mapping argument is identical: with the sibling off-scope state handled
  /// by its own arm, an `IllegalTransition` from an undo verb can only mean the
  /// criterion never left scope.
  fn in_scope(cause: FacadeError, ac: &str, verb: &str, wanted: &str) -> FacadeError {
    match cause {
      FacadeError::IllegalTransition { .. } => FacadeError::NotOffScope {
        ac: ac.to_string(),
        verb: verb.to_string(),
        wanted: wanted.to_string(),
      },
      other => other,
    }
  }

  /// A criterion that has left scope refuses every verb that would record
  /// something about its satisfaction, and the refusal names the undo.
  fn refuse_if_off_scope(criterion: &Criterion, ac: &str, verb: &str) -> Result<(), FacadeError> {
    let (state, undo) = match &criterion.state {
      AcState::Descoped { to, .. } => (format!("descoped to {to}"), "rescope"),
      AcState::Withdrawn { .. } => ("withdrawn".to_string(), "reinstate"),
      _ => return Ok(()),
    };
    Err(FacadeError::OffScope {
      ac: ac.to_string(),
      state,
      undo: undo.to_string(),
      verb: verb.to_string(),
    })
  }

  /// The one writer of a criterion's state.
  ///
  /// **"A scope change clears satisfaction" used to be a RULE here, kept by two
  /// assignments, and it is now a consequence of assigning one value.** v3 had
  /// changed `scope` alone, so a satisfied criterion that was descoped and
  /// rescoped came back still carrying the evidence for a claim that had been
  /// withdrawn -- while the verb's own help string, in v2's words, said "back in
  /// scope, unsatisfied". v2 got it right by stripping the row's whole tail on
  /// every scope change, on the way out (bin/intent_acceptance:1191) and on the
  /// way back (:1250). One enum makes both correct by construction: there is no
  /// pair of fields to leave inconsistent.
  ///
  /// It also removes the reason `transitions.rs` needed `EdgeKind::Incidental`
  /// -- with one field, a scope verb no longer moves a second one as a side
  /// effect.
  fn set_ac_state(
    &mut self,
    st: &str,
    ac: &str,
    state: AcState,
    op: &'static str,
    payload: serde_json::Value,
  ) -> Result<Outcome, FacadeError> {
    let current = &self.criterion(st, ac)?.state;

    // **This REFUSED a self-loop as `ScopeUnchanged` until hv's 2026-08-17
    // ruling, and the variant is pruned rather than deprecated.**
    //
    // The equality here is payload-inclusive, because `AcState` carries its
    // evidence, reason and target -- so this arm is "nothing whatsoever would
    // change", which is the strongest reading of a non-movement. **Same state
    // with a DIFFERENT payload deliberately falls through to the machine below**,
    // where it is refused because no verb targeting a state is declared from that
    // same state. That matters: it means asking to re-satisfy with new evidence
    // gets an explicit refusal rather than a silent no-op, so this ruling does
    // not open a reported-success-with-no-effect path.
    if *current == state {
      return Ok(Outcome::AlreadyThere {
        state: current.name().to_string(),
      });
    }
    // **The AC verbs now enforce from the same declared graph the thread and
    // work-package verbs do, and adding it found a live defect.** `ac descope`
    // succeeded on an ALREADY-descoped criterion whenever the new target
    // differed from the old, because the only check was equality: a
    // requirement could be moved from thread to thread without ever coming back
    // into scope, so the audit trail recorded a chain of moves with no decision
    // between them. The ratified machine declares `ac.descope` only from the
    // in-scope states, and now so does the code.
    Self::check_transition("Criterion", "state", op, current.name(), ac)?;
    Self::check_ac_guards(op, ac, &state)?;
    let mut next = self.canon.clone();
    let c = find_criterion_mut(&mut next, st, ac)?;
    c.state = state;
    self
      .apply(
        op,
        Subject {
          kind: "ac".to_string(),
          id: format!("{st}/{ac}"),
        },
        payload,
        next,
      )
      .map(|foreign| Outcome::Moved.with_overwrites(foreign))
  }

  fn criterion(&self, st: &str, ac: &str) -> Result<&Criterion, FacadeError> {
    self
      .st_show(st)?
      .criteria
      .iter()
      .find(|c| c.id == ac)
      .ok_or_else(|| FacadeError::NoSuchCriterion {
        st: st.to_string(),
        ac: ac.to_string(),
      })
  }

  /// [`Self::criterion`]'s twin for an acceptance test: the row, or
  /// `NoSuchTest`.
  fn acceptance_test(&self, st: &str, at: &str) -> Result<&AcceptanceTest, FacadeError> {
    self
      .st_show(st)?
      .tests
      .iter()
      .find(|t| t.id == at)
      .ok_or_else(|| FacadeError::NoSuchTest {
        st: st.to_string(),
        at: at.to_string(),
      })
  }

  // -------------------------------------------------------------------------
  // Acceptance tests
  // -------------------------------------------------------------------------

  /// Set an acceptance test's status. This is how a test-backed AC becomes
  /// satisfied -- transitively, and only by a test actually going green.
  pub fn at_set(
    &mut self,
    st: &str,
    at: &str,
    status: AtStatus,
    note: Option<String>,
  ) -> Result<Outcome, FacadeError> {
    let row_now = self
      .st_show(st)?
      .tests
      .iter()
      .find(|t| t.id == at)
      .map(|t| (t.status, t.file.clone(), t.kind))
      .ok_or_else(|| FacadeError::NoSuchTest {
        st: st.to_string(),
        at: at.to_string(),
      })?;
    let (from, cited, kind) = row_now;

    // **A VERDICT NEEDS ITS EVIDENCE TO EXIST, AND THE FORWARD STEP IS A ONE-WAY
    // DOOR** (issue 0270, vc ruled option 2, 2026-09-05).
    //
    // `to-write` citing a file that does not exist yet is the NORMAL state and is
    // exempt from the `absent_at` finding. The identical row at `green` or `red`
    // is a live finding, and that finding **refuses every commit in the
    // repository** -- not just the committer's, and not just this thread's. So
    // `to-write -> red` on a test nobody has written yet converts a quiet, legal
    // row into an estate-wide block.
    //
    // **AND THERE IS NO STEP BACK.** `intent at` offers `green`, `red`, `na`,
    // `new` and `edit`; no spelling returns a row to `to-write`. Giving the
    // machine that inverse is the wider fix and is not ruled out. This is the
    // narrower one, and it defends every row rather than the one somebody
    // remembered to annotate.
    //
    // **BOTH VERDICT STATES, NOT ONLY `red`.** The ruling names `at red` because
    // that is the door that was walked into, but `absent_at` treats a green row
    // citing a missing file identically -- guarding one and not the other would
    // leave the same hole one verb over.
    //
    // **RADIUS MEASURED BEFORE THE CHOICE**, independently of the row that asked
    // for it: 303 green/red rows in this estate carry a citation and **zero**
    // cite a missing file, so this refuses nothing that exists. It is additive.
    // The exposure it closes is larger than the row's one instance: **36
    // `to-write` rows cite a file that has never existed**, and every one of them
    // is this door, armed.
    if matches!(status, AtStatus::Green | AtStatus::Red)
      && let Some(path) = cited.as_deref()
      && !self.project.root().join(path).exists()
    {
      return Err(FacadeError::VerdictCitesAbsentFile {
        st: st.to_string(),
        at: at.to_string(),
        path: path.to_string(),
        status: status.display().to_string(),
      });
    }

    // **`at.set` is declared with an EMPTY from-set, so without this a self-loop
    // was a real write.** Every value legitimately reaches every other here, which
    // means `from` never refuses anything -- and `at set green` on an
    // already-green row wrote an envelope recording a movement that did not
    // happen. Under D42 the record is stamped by the write, so history gained a
    // second transition at a second time for one event.
    //
    // **AND IT HAD TO LEARN THE DIFFERENCE BETWEEN NOTHING-TO-DO AND
    // NOTHING-TO-DO-ABOUT-THE-STATUS.** `at green --note "..."` on an
    // already-green row is the COMMON case for annotating a row -- nobody
    // annotates a status they are about to change -- so short-circuiting on
    // `from == status` alone would make the flag inert exactly where it is most
    // wanted, with `--help` listing it and the exit code 0.
    if from == status && note.is_none() {
      return Ok(Outcome::AlreadyThere {
        // `display()`, not `enum_str` -- `AlreadyThere` is a state name a HUMAN
        // reads, and the wire form spells `Na` as `n-a` (issue 0056). This was
        // the second of the three spellings one command produced.
        state: from.display().to_string(),
      });
    }

    // **A VERDICT MUST FIT THE ROW'S KIND** (issue 0337). `n/a` belongs to a
    // non-test row, which asserts prose instead of a file, and `red`/`green`
    // belong to a test row, whose verdict is a test's. `at_set` read neither,
    // so `at na` on a test row and `at green` on a non-test row both landed.
    let fits = match status {
      AtStatus::Na => kind == AtKind::NonTest,
      AtStatus::Red | AtStatus::Green => kind == AtKind::Test,
      _ => true,
    };
    if !fits {
      return Err(FacadeError::VerdictWrongForKind {
        st: st.to_string(),
        at: at.to_string(),
        kind: crate::model::enum_str(&kind).to_string(),
        status: status.display().to_string(),
      });
    }
    // **GREEN ONLY FROM RED** (issue 0337): every `at.set` edge shares one
    // verb, so only the target tells them apart -- see `transitions::permits_to`.
    if from != status {
      let (from_s, to_s) = (
        crate::model::enum_str(&from),
        crate::model::enum_str(&status),
      );
      if !transitions::permits_to("AcceptanceTest", "status", "at.set", &from_s, &to_s) {
        return Err(FacadeError::IllegalTransition {
          verb: "at.set",
          subject: format!("{st} {at}"),
          from: from_s.to_string(),
          legal: transitions::accepted_from_to("AcceptanceTest", "status", "at.set", &to_s)
            .join(", "),
        });
      }
    }

    let mut next = self.canon.clone();
    {
      let row = find_test_mut(&mut next, st, at)?;
      row.status = status;
      // **THE RECORD LIVES EXACTLY AS LONG AS THE CLOSE DOES, SO MOVING OFF
      // `Fiat` DISCARDS IT.** Two published `///`s already promise this --
      // `AcceptanceTest::fiat` says *present exactly when status is Fiat* and
      // `FiatRecord` says the record lives exactly as long as the fiat close --
      // and both are lifted verbatim into the committed JSON Schema and SDL, so
      // leaving the record behind published a guarantee the model did not keep.
      //
      // **The AC half gets this for free and the AT half cannot.** `AcState`
      // carries the record INSIDE its `Fiat` variant, so `ac_reinstate` discards
      // it structurally -- there is nowhere in the entry states to put it. The
      // record sits BESIDE the status here (forced: `AtStatus` derives `Copy`
      // and async-graphql `Enum`), so nothing structural drops it and the
      // discard has to be written. hv's exit ruling of 2026-08-29 settles which
      // way it goes: **the history is the event log, not this field.**
      if status != AtStatus::Fiat {
        row.fiat = None;
      }
      if let Some(text) = note.as_ref() {
        // **THE NOTE IS EXTENDED OR IT IS REFUSED** (issue 0207). `--note`
        // replaced this field wholesale and silently, on the three verbs
        // someone reaches for at close -- which is the moment they write down
        // why, and the moment the row's adjudication history is longest.
        //
        // **THE TEST IS CONTAINMENT, AND LENGTH IS NOT IN IT** (vc ruled
        // 2026-09-02 under hv's pen). The contract in one line: **the status
        // verbs EXTEND; `at edit` REPLACES.** Containment is *extend*
        // mechanised.
        //
        // **A LENGTH TEST FAILS OPEN ON A SAME-LENGTH REWRITE**, which is why
        // it was refused. Replace 400 bytes of adjudication record with 400
        // bytes of something else and `shorter` is false, so the guard passes
        // and the record is gone -- a check failing open on a case inside its
        // own subject. Containment DOMINATES rather than trades off: anything
        // longer-by-appending contains the original, so there is nothing a
        // length test catches that this misses, and it needs no threshold.
        //
        // **AND NO THRESHOLD WAS AVAILABLE TO BUILD, WHICH IS A MEASUREMENT
        // AND NOT AN OPINION.** 0207's remedy sketch said *materially* longer.
        // Across 313 AT notes carrying content the sizes run near-continuously
        // -- median 178, p75 1184, max 17485, and the largest gap anywhere
        // between 150 and 1200 bytes is about 30. There is no valley between
        // short labels and adjudication records to put a line in, so any
        // number would have been arbitrary while being described as
        // data-driven, and its failure mode is the silent one: a 400-byte
        // record destroyed below the line with the guard green.
        //
        // **`materially longer` WAS ITSELF INSTANCE-SHAPED.** It was written
        // looking at a 7803 -> 683 destruction, where length was the visible
        // symptom. The mechanism was never length; it is CONTENT LOSS.
        if let Some(existing) = row.note.as_ref()
          && !text.contains(existing.as_str())
        {
          let opening: String = existing.chars().take(120).collect();
          return Err(FacadeError::NoteWouldBeLost {
            url: format!("{st}/{at}"),
            existing_bytes: existing.len(),
            incoming_bytes: text.len(),
            opening,
          });
        }
        row.note = Some(text.clone());
      }
    }
    self
      .apply(
        "at.set",
        Subject {
          kind: "at".to_string(),
          id: format!("{st}/{at}"),
        },
        // **`note` IS IN THE PAYLOAD SO A SELF-LOOP ENVELOPE IS NOT A LIE.** The
        // guard above exists because `at set green` on a green row once recorded
        // a movement that did not happen. A note-only write reaches here with
        // `from == to`, so the envelope must say what DID change or it
        // reintroduces exactly that reading.
        json!({"from": crate::model::enum_str(&from), "to": crate::model::enum_str(&status), "note": note.is_some()}),
        next,
      )
      .map(|foreign| Outcome::Moved.with_overwrites(foreign))
  }

  /// **Close an acceptance test on human authority, against the evidence** --
  /// the AT-side door onto a fiat close, and the mirror of [`Facade::ac_fc`].
  ///
  /// **THE STATE AND ITS EVIDENCE ARE TWO FIELDS HERE AND ONE FIELD FOR A
  /// CRITERION**, so this verb writes both and the invariant holds them
  /// together. `AcState::Fiat` carries its `FiatRecord`; `AtStatus` derives
  /// `Copy` and async-graphql `Enum` and cannot, so the record lives on
  /// [`AcceptanceTest::fiat`]. A `Fiat` status with no record, or a record on a
  /// row that is not `Fiat`, are both representable and both defects.
  ///
  /// **`at` GOES IN EMPTY AND COMES BACK FILLED** (D42), exactly as `ac_fc`'s
  /// does. Nothing here knows what time it is: the database stamps the event
  /// inside the INSERT and `Facade::apply_with_state` patches the value that
  /// landed back into the record before the extract is rendered.
  ///
  /// **THE MACHINE IS CONSULTED BEFORE THE REASON, AND THE ORDER IS THE POINT.**
  /// `at_set` beside this reads no declaration at all -- its edges carry
  /// `from: &[]`, so there was never a from-state to check. This verb has one,
  /// and it goes through `Self::check_transition` rather than a hand-written
  /// `match`: issues 0051 and 0053 are four instances of a hand-rolled
  /// from-state check placed ahead of the shared setter, each making the common
  /// arm unreachable for one verb. The refusal for an already-closed row is
  /// therefore MAPPED from what the declaration said, never decided here.
  pub fn at_fc(
    &mut self,
    st: &str,
    at: &str,
    because: &str,
    by: &str,
  ) -> Result<Outcome, FacadeError> {
    // Scoped so the immutable borrow ends before `self.canon` is cloned; both
    // values are owned copies of what the refusal below needs.
    let (from, standing) = {
      let row = self
        .st_show(st)?
        .tests
        .iter()
        .find(|t| t.id == at)
        .ok_or_else(|| FacadeError::NoSuchTest {
          st: st.to_string(),
          at: at.to_string(),
        })?;
      (row.status, row.fiat.as_ref().map(|r| r.because.clone()))
    };

    Self::check_transition(
      "AcceptanceTest",
      "status",
      "at.fc",
      &crate::model::enum_str(&from),
      at,
    )
    .map_err(|cause| match (cause, standing) {
      // **The standing reason is what the operator needs before replacing one
      // human judgement with another**, which is why it is carried rather than
      // summarised. `undo` names `at red` rather than a reinstate verb because
      // an AT leaves `fiat` through the ordinary setter -- the asymmetry with
      // `ac reinstate` is recorded on the machine's own edge list.
      (FacadeError::IllegalTransition { .. }, Some(because)) => FacadeError::AlreadyFiatClosed {
        subject: at.to_string(),
        because,
        undo: format!("intent at red <thread> {at}"),
      },
      (other, _) => other,
    })?;

    // **NOT AN `unwrap`, AND NOT UNREACHABLE.** `check_reason` returns `None`
    // when the verb declares no `ReasonRecorded` guard. `at.fc` declares one
    // today, so the `None` arm is what happens if that declaration is ever
    // dropped from the table -- and the consequence would be a `FiatRecord`
    // stored with an empty `because`, which `length(min = 1)` forbids in the
    // file and nothing would forbid in the store. Refusing is the honest answer
    // to a table and a verb that disagree.
    let because = Self::check_reason("AcceptanceTest", "status", "at.fc", Some(because))?
      .ok_or(FacadeError::ReasonRequired { verb: "at.fc" })?;

    let mut next = self.canon.clone();
    {
      let row = find_test_mut(&mut next, st, at)?;
      row.status = AtStatus::Fiat;
      row.fiat = Some(crate::model::FiatRecord {
        because: because.clone(),
        by: by.to_string(),
        at: String::new(),
        invoker: crate::model::Invoker::collected(),
        inherited_from: None,
        inherited_event: None,
      });
    }
    // Computed from `next` -- the estate as this close leaves it -- and BEFORE
    // the apply, because `next` is moved into it.
    let sole = next
      .threads
      .iter()
      .find(|t| t.id == st)
      .map(|t| crate::contract::solely_covered_by(t, at))
      .unwrap_or_default();
    self
      .apply(
        "at.fc",
        Subject {
          kind: "at".to_string(),
          id: format!("{st}/{at}"),
        },
        json!({"because": because, "by": by}),
        next,
      )
      .map(|foreign| {
        if sole.is_empty() {
          Outcome::Moved
        } else {
          Outcome::MovedWith {
            notes: vec![Note::FiatClosedSoleCover(sole)],
          }
        }
        .with_overwrites(foreign)
      })
  }

  /// **Write by address** -- `PUT` an entity's json form (AC-08.3, AC-08.4).
  ///
  /// The mutation format IS the interchange format: `GET ?format=json`,
  /// modify, `PUT` the same shape back. That is what gives AC-02.6 its second
  /// job -- a field that does not round-trip is a field that cannot be
  /// WRITTEN.
  ///
  /// **`PUT` is for CALLER-ASSIGNED ids only** (AC-08.4). An AC or an AT is
  /// named by its author, so the address exists before the row does and `PUT`
  /// creates it. Threads, issues and WP sequences are server-assigned -- you
  /// cannot address `ST0058` before the tool has decided it is `ST0058` -- so
  /// those are a `POST` to the collection and are refused here rather than
  /// half-supported.
  ///
  /// **json only.** Writing markdown to an address would promote a stale
  /// rendering into canon. The attachment exception is not an exception:
  /// an attachment is AUTHORED on disk, so authority runs the other way and
  /// text-in is correct. Authorship decides direction.
  /// **POST to a COLLECTION address: the tool assigns the id and hands back the
  /// address it assigned** (AC-08.4).
  ///
  /// **The other half of `put`, and the split is not a REST convention borrowed
  /// for its own sake.** You cannot address `ST0058` before the tool has
  /// decided it is `ST0058`, so a create whose id the SERVER chooses has no
  /// entity address for the caller to `PUT` to -- there is nothing to name yet.
  /// The collection is the only address that exists before the id does.
  ///
  /// **It returns an `Address` rather than an id, and that is the criterion's
  /// wording rather than decoration.** A caller handed `ST0058` has to build
  /// the address itself to do anything with it, which is a second spelling of
  /// the scheme at every call site; handing back the address means the one
  /// resolver stays the one resolver.
  ///
  /// **Every refusal names the FORM and is counted rather than dropped.** A
  /// surface that silently ignored the collections it cannot create into would
  /// report the same success as one that handles them all.
  pub fn post(&mut self, address: &Address, body: &str) -> Result<Address, FacadeError> {
    require_local(address)?;
    match &address.entity {
      AddrEntity::Threads => {
        let value = Self::posted_json(address, body)?;
        let title = Self::posted_title(address, &value)?;
        let id = self.st_new(&title)?;
        Ok(Address {
          authority: None,
          entity: AddrEntity::Thread { id },
          format: address.format,
        })
      }
      // **THE SECOND AND THIRD OF D57-8's THREE SERVER-ASSIGNED POPULATIONS.**
      // `_threads, issues, WP seq_` is the design's own list; until 2026-08-20
      // the grammar could express one of them, so this verb read complete while
      // covering a third of its subject.
      AddrEntity::Issues => {
        let value = Self::posted_json(address, body)?;
        let title = Self::posted_title(address, &value)?;
        let number = self.issue_add(
          &title,
          value.get("severity").and_then(|v| v.as_str()),
          value.get("reporter").and_then(|v| v.as_str()),
          value
            .get("body")
            .and_then(|v| v.as_str())
            .unwrap_or_default(),
        )?;
        Ok(Address {
          authority: None,
          entity: AddrEntity::Issue {
            id: format!("{number:04}"),
          },
          format: address.format,
        })
      }
      AddrEntity::WpCollection { thread } => {
        let value = Self::posted_json(address, body)?;
        let title = Self::posted_title(address, &value)?;
        // **A SIZE THE CALLER DID NOT CHOOSE COMES FROM ONE PLACE**, shared with
        // the CLI's `wp new`. A size they DID choose and spelled wrongly is
        // refused by name rather than defaulted: silently sizing someone else's
        // work package is the kind of help that is indistinguishable from a bug.
        let scope = match value.get("scope").and_then(|v| v.as_str()) {
          None => crate::model::DEFAULT_WP_SCOPE,
          Some(raw) => TShirt::parse(raw).ok_or_else(|| FacadeError::WriteNotAddressable {
            url: address.to_url(),
            why: format!(
              "`{raw}` is not a size -- the six are {}",
              TShirt::spellings()
            ),
          })?,
        };
        let seq = self.wp_new(thread, &title, scope)?;
        Ok(Address {
          authority: None,
          entity: AddrEntity::Wp {
            thread: thread.clone(),
            wp: format!("{seq:02}"),
          },
          format: address.format,
        })
      }
      // **REFUSED BY NAME, WITH THE REASON THAT DECIDES IT: the id is already
      // known, so the entity address exists and `PUT` is the verb that reaches
      // it.** This is not "unsupported"; it is the other side of AC-08.4's
      // split, and saying so sends the caller to a door that opens.
      other => Err(FacadeError::WriteNotAddressable {
        url: address.to_url(),
        why: format!(
          "`{}` is not a collection whose ids this tool assigns -- POST creates only where the id does not exist yet. Its id is already known, so `PUT` to the entity address instead",
          other.form()
        ),
      }),
    }
  }

  /// The title a POST to `threads` carries, or a refusal naming what is missing.
  ///
  /// **A blank title is refused rather than defaulted.** `st new` takes a title
  /// because a thread without one is unfindable in every view that lists it,
  /// and a create arriving through a different door must not be able to make
  /// the entity that verb refuses to make.
  /// A posted body, parsed once.
  ///
  /// Separate from [`Facade::posted_title`] because three collections read
  /// different fields out of the same body, and parsing per field would report
  /// "the body is not JSON" once for each of them.
  fn posted_json(address: &Address, body: &str) -> Result<serde_json::Value, FacadeError> {
    serde_json::from_str(body).map_err(|e| FacadeError::WriteNotAddressable {
      url: address.to_url(),
      why: format!("the body is not JSON: {e}"),
    })
  }

  fn posted_title(address: &Address, value: &serde_json::Value) -> Result<String, FacadeError> {
    let refuse = |why: &str| FacadeError::WriteNotAddressable {
      url: address.to_url(),
      why: why.to_string(),
    };
    let title = value
      .get("title")
      .and_then(|t| t.as_str())
      .ok_or_else(|| refuse("a posted thread needs a `title` -- a thread without one is unfindable in every view that lists it"))?;
    if title.trim().is_empty() {
      return Err(refuse(
        "`title` is blank, and a blank title is refused rather than defaulted",
      ));
    }
    Ok(title.to_string())
  }

  /// # `put` IS A LOW-LEVEL UPSERT AND DOES NOT CONSULT THE TRANSITION MACHINES
  ///
  /// **hv ruled this on 2026-08-21 (relayed by vc, on ST0057 AC-08.5): the
  /// VERBS own the machines and `put` stays what it is.** `st done` and
  /// `todo done` are the enforcement point -- a body carrying
  /// `"status": "done"` lands here whether or not that transition is legal,
  /// and `todo done` refused exactly that transition on a `triage` thread the
  /// same afternoon.
  ///
  /// The alternative -- every write path honouring the machine -- was put to hv
  /// and declined, on the ground that **what needed fixing was that nothing on
  /// the record said so.** This paragraph is that fix, and it is the only place
  /// it can live: the dispatch table describes VERBS, and `put` is not one.
  ///
  /// **THE ARGUMENT THAT CARRIED IT WAS CONSISTENCY, NOT SAFETY.** `at.put`
  /// already writes an AT row's `status` this way and the estate has accepted
  /// it -- ic's AC-08.5 measurement drives exactly that -- so guarding the
  /// thread door alone would make one entity's `put` mean something different
  /// from another's.
  ///
  /// # The exposure is zero because nothing calls it, NOT because the write is checked
  ///
  /// **Read that sentence before concluding this is safe.** The machine is
  /// enforced one layer ABOVE the SSOT, so anything reaching the store directly
  /// goes around it. Driven 2026-08-21 (vc): `intent put` is not a subcommand
  /// at all, and of **17 `.put(` call sites across `native/rust`, every one is
  /// a test** -- there is no caller in any `src/` outside this definition.
  ///
  /// **So a future reader who finds a production caller has found a LIVE GAP,
  /// not a curiosity.** The absence of callers is the whole of the containment;
  /// the day one appears, this contract needs re-deciding rather than
  /// re-reading.
  pub fn put(&mut self, address: &Address, body: &str) -> Result<Outcome, FacadeError> {
    require_local(address)?;
    let is_attachment = matches!(address.entity, AddrEntity::Attachment { .. });
    if address.format == Some(AddrFormat::Md) && !is_attachment {
      return Err(FacadeError::WriteNotAddressable {
        url: address.to_url(),
        why: "PUT accepts json; markdown would promote a stale rendering into canon".to_string(),
      });
    }

    match &address.entity {
      AddrEntity::At { thread, at } => {
        let row: AcceptanceTest =
          serde_json::from_str(body).map_err(|e| FacadeError::WriteNotAddressable {
            url: address.to_url(),
            why: format!("the body is not an acceptance test: {e}"),
          })?;
        if &row.id != at {
          return Err(FacadeError::WriteNotAddressable {
            url: address.to_url(),
            why: format!("the body names `{}` and the address names `{at}`", row.id),
          });
        }
        let mut next = self.canon.clone();
        let holder = find_thread_mut(&mut next, thread)?;
        let outcome = match holder.tests.iter_mut().find(|t| &t.id == at) {
          Some(existing) => {
            if *existing == row {
              return Ok(Outcome::AlreadyThere {
                state: "unchanged".to_string(),
              });
            }
            // **hv's D7 (2026-08-29): THE RECORD IS REACHABLE ONLY THROUGH `fc`.**
            //
            // **CHANGED, not PRESENT, and the difference is load-bearing.**
            // `put` is a whole-row write, so a caller doing an ordinary
            // read-modify-write on an already fiat-closed row sends the record
            // back untouched -- refusing `Some` outright would make such a row
            // unputtable, and its `note` could never be edited again. `id` is
            // guarded exactly this way one refusal above.
            //
            // **AND IT HAS TO LIVE HERE RATHER THAN IN THE `Unsettable`
            // ROSTER.** That roster is read by `set` and by the form layer's
            // editability; `put` consults neither and writes the whole row.
            // The first draft of this ruling went into the roster, the suite
            // stayed green, and the green was the tell -- a refusal that moves
            // nothing has not been added.
            if existing.fiat != row.fiat {
              return Err(FacadeError::WriteNotAddressable {
                url: address.to_url(),
                why: "the fiat record is evidence about a person and `put` cannot author it: \
             use `intent fc <target> --because \"<why>\"`, which records who closed it, \
             when, and the invocation's own evidence that no caller supplies"
                  .to_string(),
              });
            }
            *existing = row;
            Outcome::Moved
          }
          None => {
            // A row cannot ARRIVE fiat-closed: there is no prior close for this
            // to be the unchanged record of.
            if row.fiat.is_some() {
              return Err(FacadeError::WriteNotAddressable {
                url: address.to_url(),
                why: "the fiat record is evidence about a person and `put` cannot author it: \
             use `intent fc <target> --because \"<why>\"`, which records who closed it, \
             when, and the invocation's own evidence that no caller supplies"
                  .to_string(),
              });
            }
            holder.tests.push(row);
            holder.tests.sort_by(|a, b| a.id.cmp(&b.id));
            Outcome::Moved
          }
        };
        self
          .apply(
            "at.put",
            Subject {
              kind: "at".to_string(),
              id: format!("{thread}/{at}"),
            },
            json!({ "via": "address" }),
            next,
          )
          .map(|foreign| outcome.with_overwrites(foreign))
      }
      AddrEntity::Ac { thread, ac } => {
        let row: Criterion =
          serde_json::from_str(body).map_err(|e| FacadeError::WriteNotAddressable {
            url: address.to_url(),
            why: format!("the body is not a criterion: {e}"),
          })?;
        if &row.id != ac {
          return Err(FacadeError::WriteNotAddressable {
            url: address.to_url(),
            why: format!("the body names `{}` and the address names `{ac}`", row.id),
          });
        }
        let mut next = self.canon.clone();
        let holder = find_thread_mut(&mut next, thread)?;
        // **THE SAME RULING REACHES THE CRITERION KIND BY A DIFFERENT ROUTE.**
        // The AC record lives INSIDE `AcState::Fiat` rather than beside the
        // state, so there is no `fiat` field here to compare -- the guard is on
        // the state's variant instead. Same property, same remedy, and the two
        // shapes are why hv's ruling had to be applied twice rather than once.
        let fiat_in = |c: &Criterion| matches!(c.state, AcState::Fiat(_));
        let outcome = match holder.criteria.iter_mut().find(|c| &c.id == ac) {
          Some(existing) => {
            if *existing == row {
              return Ok(Outcome::AlreadyThere {
                state: "unchanged".to_string(),
              });
            }
            if fiat_in(&row) != fiat_in(existing) || (fiat_in(&row) && existing.state != row.state)
            {
              return Err(FacadeError::WriteNotAddressable {
                url: address.to_url(),
                why: "the fiat record is evidence about a person and `put` cannot author it: \
             use `intent fc <target> --because \"<why>\"`, which records who closed it, \
             when, and the invocation's own evidence that no caller supplies"
                  .to_string(),
              });
            }
            *existing = row;
            Outcome::Moved
          }
          None => {
            if fiat_in(&row) {
              return Err(FacadeError::WriteNotAddressable {
                url: address.to_url(),
                why: "the fiat record is evidence about a person and `put` cannot author it: \
             use `intent fc <target> --because \"<why>\"`, which records who closed it, \
             when, and the invocation's own evidence that no caller supplies"
                  .to_string(),
              });
            }
            holder.criteria.push(row);
            holder.criteria.sort_by(|a, b| a.id.cmp(&b.id));
            Outcome::Moved
          }
        };
        self
          .apply(
            "ac.put",
            Subject {
              kind: "ac".to_string(),
              id: format!("{thread}/{ac}"),
            },
            json!({ "via": "address" }),
            next,
          )
          .map(|foreign| outcome.with_overwrites(foreign))
      }
      // **CREATE AND UPDATE ARE DIFFERENT OPERATIONS AND THIS ARM USED TO
      // DECLINE BOTH WITH A CREATE-SHAPED REASON** (hv ruling, 2026-08-21,
      // ST0057 AC-08.5).
      //
      // `this id is server-assigned -- POST to the collection address` is a
      // true statement about CREATING a thread with a chosen id, and it was
      // being returned for UPDATING one that already exists -- where the id is
      // not being assigned by anybody, it is being addressed. The consequence
      // was concrete: ST0011's `completed` is the estate's one genuinely wrong
      // row and had **no write path at all**, because no field-setter verb
      // reaches it and the one addressable door refused on grounds that did not
      // apply.
      //
      // So the arm splits on EXISTENCE. Create-by-id stays refused, with the
      // same words, which are correct for it.
      AddrEntity::Thread { id } => {
        let refuse = |why: String| FacadeError::WriteNotAddressable {
          url: address.to_url(),
          why,
        };
        let value = Self::posted_json(address, body)?;

        // **CHILD COLLECTIONS ARE REFUSED BY NAME, NEVER SILENTLY DROPPED AND
        // NEVER SILENTLY APPLIED.** `Thread` carries `wps`, `criteria`,
        // `tests` and `attachments`, and every one of them is `#[serde(default)]`
        // -- so the obvious implementation, parse-and-replace, turns a body
        // that simply did not mention `tests` into a thread with none.
        //
        // **That is the defect AC-08.5 exists to name, arriving inside the
        // change meant to satisfy it**: the criterion's second limb is that no
        // verb silently clears a field it was not asked to change, and the
        // rows it would hit hardest are the ones carrying the most evidence.
        //
        // Ignoring the keys instead would be the same failure pointed the
        // other way -- a caller who sent `tests` and got a success back would
        // have been told their write landed. **Each child has its own address
        // and that is where it is written**, so the refusal can say where to
        // go. `related` is deliberately NOT in this list: it has no address of
        // its own, so a whole-thread write carries it, and its one-link doors
        // are `st relate` and `st unrelate` (issue 0460).
        // **THE FIELD NAME IS NOT THE ADDRESS SEGMENT, AND INTERPOLATING ONE
        // AS THE OTHER PRINTS A REMEDY THAT DOES NOT PARSE.** The model calls
        // them `wps`/`criteria`/`tests`; the grammar spells them `wp`/`ac`/`at`
        // (`address.rs:445-467`). The first draft of this refusal named
        // `threads/<id>/tests/<AT>` -- correct-looking, and an operator
        // following it gets a parse error from the tool that just told them to
        // go there. Mapped explicitly, and each pair is driven.
        for (field, segment) in CHILD_COLLECTIONS {
          if value.get(field).is_some() {
            return Err(refuse(format!(
              "`{field}` is not written through the thread address -- PUT each one at its own address (`{}/{segment}/<id>`), because a thread PUT that accepted this would have to either apply it or drop it, and both are silent about the other",
              address.to_url()
            )));
          }
        }

        let Some(existing) = self.canon.threads.iter().find(|t| &t.id == id).cloned() else {
          return Err(refuse(
            "this id is server-assigned -- POST to the collection address".to_string(),
          ));
        };

        // **THE KEYS THE CALLER ACTUALLY SENT, CAPTURED BEFORE THE PARSE
        // CONSUMES THEM.** Everything below turns on the difference between a
        // field the body OMITTED and a field it set to the same value serde
        // would have defaulted to, and after `from_value` those are identical.
        let sent: std::collections::BTreeSet<String> = value
          .as_object()
          .map(|o| o.keys().cloned().collect())
          .unwrap_or_default();

        let mut row: Thread = serde_json::from_value(value)
          .map_err(|e| refuse(format!("the body is not a thread: {e}")))?;
        if &row.id != id {
          return Err(refuse(format!(
            "the body names `{}` and the address names `{id}`",
            row.id
          )));
        }

        // The children the body was refused permission to carry are the
        // children the thread keeps. Grafted from the CLONE taken above, so
        // the row written is the authored scalars over the stored children and
        // never a partially-defaulted document.
        row.wps = existing.wps.clone();
        row.criteria = existing.criteria.clone();
        row.tests = existing.tests.clone();
        row.attachments = existing.attachments.clone();

        // **LIMB 2 AS AN INVARIANT OF THE VERB, WHICH IS WHERE `set` ALREADY
        // KEEPS IT.** Every field this model declares is `#[serde(default)]` or
        // `Option`, so a body that simply does not MENTION `context` parses to a
        // thread whose `context` is empty -- and the graft above restores the
        // four CHILDREN and nothing else. Measured 2026-08-24 (ic, `ea84d0ae`):
        // a minimal legal body carrying the five required fields plus
        // `completed` silently cleared the other EIGHT -- `slug`,
        // `status_reason`, `acceptance`, `objective`, `context`, `body`,
        // `preamble`, `related`. 8 of 8, nothing partial.
        //
        // **THE GRAFT IS WHAT MAKES THAT A CHOICE RATHER THAN A LIMITATION**, and
        // a choice is what a criterion can be failed against: four lines restore
        // the children, and the nine scalars four lines away are not restored.
        //
        // **`related` IS THE SHARPEST AND THE ARM ABOVE SUPPLIES THE REASON** --
        // it has no address of its own, so when this was measured the thread
        // door was its ONLY door, and it was the door that emptied it.
        //
        // Refusing BY NAME rather than merging silently: a caller who omitted
        // `context` may have meant *leave it* or may have meant *clear it*, and
        // guessing either way is the silence this criterion exists to name. The
        // remedy is a field the caller can act on.
        let before = serde_json::to_value(&existing)
          .map_err(|e| refuse(format!("the stored thread does not serialise: {e}")))?;
        let after = serde_json::to_value(&row)
          .map_err(|e| refuse(format!("the posted thread does not serialise: {e}")))?;
        if let (Some(before), Some(after)) = (before.as_object(), after.as_object()) {
          // **THE UNION OF BOTH KEY SETS, AND THE FIRST DRAFT USED ONLY
          // `after`'s.** `Thread::related` is `skip_serializing_if`, so a row
          // whose `related` has just been cleared to `[]` does not carry the key
          // at all -- and iterating the written side alone made the CLEARED
          // fields exactly the ones the check could not see. **It reported
          // `context` and `objective` and stayed silent about `related`, which
          // is the sharpest field of the eight**: the arm above records that
          // `related` has no address of its own, so the thread door is its ONLY
          // door, and the door was the thing emptying it. Caught by this
          // change's own test, which is the whole argument for writing the
          // refusal and the assertion in the same sitting.
          let keys: std::collections::BTreeSet<&String> =
            before.keys().chain(after.keys()).collect();
          let collateral: Vec<&String> = keys
            .into_iter()
            .filter(|k| !sent.contains(*k) && before.get(*k) != after.get(*k))
            .collect();
          if !collateral.is_empty() {
            let named = collateral
              .iter()
              .map(|k| format!("`{k}`"))
              .collect::<Vec<_>>()
              .join(", ");
            return Err(refuse(format!(
              "this write would change {named}, which the body does not mention -- send each \
               field you mean to change, or set one at a time with the narrow setter. A PUT that \
               applied this would clear fields you did not ask about, and the rows carrying the \
               most evidence are the ones it would hit hardest"
            )));
          }
        }

        // **hv's D7 EXTENDED TO THIS KIND: `put` REFUSES TO AUTHOR A FIAT
        // RECORD.** The fiat record is evidence about a PERSON -- who closed
        // this against the evidence, when, and from what invocation -- and no
        // other field on this row is. Prevention in general is unachievable and
        // that is exactly why the posture is attribution, **so a door that lets
        // the record be fabricated without the guard holes the only half that
        // works.**
        //
        // **AND IT RUNS AFTER THE COLLATERAL-CLEAR CHECK, WHICH IS AN ORDERING
        // DECISION AND NOT AN ACCIDENT.** Placed before it, this refusal fired
        // on a body that simply omitted `fiat` -- true, and the wrong thing to
        // tell that caller, because the same body had omitted eight other
        // fields and the general refusal names all of them. **A narrower
        // refusal that preempts a broader one hides the broader one**, so the
        // specific message is only reached once the general one is satisfied.
        //
        // **CHANGED, not PRESENT.** `put` is a whole-row write, so an ordinary
        // read-modify-write on an already fiat-closed thread sends the record
        // back untouched; refusing `Some` outright would make such a thread
        // unputtable and its `objective` uneditable forever.
        //
        // **AND IT LIVES HERE RATHER THAN IN THE `Unsettable` ROSTER**, which
        // `set` and the form layer read and `put` consults neither -- the first
        // draft of this ruling went into that roster, the suite stayed green,
        // and the green was the tell.
        if existing.fiat != row.fiat {
          return Err(refuse(
            "the fiat record is evidence about a person and `put` cannot author it: \
             use `intent fc <target> --because \"<why>\"`, which records who closed it, \
             when, and the invocation's own evidence that no caller supplies"
              .to_string(),
          ));
        }

        if row == existing {
          return Ok(Outcome::AlreadyThere {
            state: "unchanged".to_string(),
          });
        }

        let mut next = self.canon.clone();
        *find_thread_mut(&mut next, id)? = row;
        self
          .apply(
            "thread.put",
            Subject {
              kind: "thread".to_string(),
              id: id.to_string(),
            },
            json!({ "via": "address" }),
            next,
          )
          .map(|foreign| Outcome::Moved.with_overwrites(foreign))
      }
      // **AN ATTACHMENT IS THE ONE ADDRESS WHERE TEXT-IN IS CORRECT, AND THAT
      // IS A RULING RATHER THAN A CONVENIENCE** (`design.md:271`, hv
      // 2026-08-18): _an ATTACHMENT is authored on disk, so for attachments the
      // authority runs the other way and text-in is correct._ Every other
      // entity refuses a markdown body because writing a rendering into canon
      // promotes a stale view; an attachment HAS no rendering, so the body is
      // the content and there is nothing for it to be stale about.
      //
      // **THE SAME RULING NAMES `Project::classify` AS THE SINGLE ANSWER TO
      // WHAT A FILE IS, AND THIS ARM ASKS IT RATHER THAN RE-DECIDING.** A
      // filename check here would be the second-opinion defect AC-02.5 names,
      // and it would drift the day somebody adds a view.
      AddrEntity::Attachment { thread, path } => {
        let refuse = |why: String| FacadeError::WriteNotAddressable {
          url: address.to_url(),
          why,
        };

        // **`?format=json` IS REFUSED, AND IT IS THE ROUND-TRIP THAT MAKES IT
        // DANGEROUS RATHER THAN MERELY REDUNDANT.** The mutation format is the
        // interchange format -- `GET ?format=json`, modify, `PUT` the same
        // shape back -- so a caller who has learnt that habit on every other
        // address would, at this one, write the attachment's own RECORD (path,
        // bytes, sha256) into the file as its CONTENT. Every guard below would
        // pass while it happened, and the sha256 would correctly describe the
        // wrong thing.
        if address.format == Some(AddrFormat::Json) {
          return Err(refuse(
            "an attachment's body is its content, so this address takes text -- `?format=json` would write the record into the file".to_string(),
          ));
        }

        // **THE NAMING GATE, AND IT IS THE ONE INGEST ALREADY USES.** A path
        // that escapes the thread, or that normalises onto another
        // attachment's canon sidecar and destroys it, is refused before
        // anything is written. Sharing `attachment_name` rather than checking
        // locally is what keeps `put` and `--to-store` accepting the same set:
        // two answers to "is this name storable" agree exactly until one moves.
        crate::project::attachment_name(thread, path).map_err(|bad| refuse(bad.to_string()))?;

        let rel = std::path::PathBuf::from(path);
        // A generated view, or a stray canon file, wearing an attachment's
        // address. **Asked through `edit_disposition` because a refusal here
        // owes the caller a DESTINATION** -- the operator has a real edit to
        // make and this address is not where it goes -- and because the remedy
        // strings then have one author, exactly as the classification does.
        // **ONLY THE `Canon` LIMB IS REACHABLE HERE, AND THE OTHER ONE BEING
        // DEAD IS A DEFENCE RATHER THAN A GAP.** `address::parse` refuses a
        // view's name a layer lower -- `acceptance.md` and `info.md` come back
        // `ViewAddressed` and never arrive -- so what this catches in practice
        // is a stray `thread.json` from a v2 tree wearing an attachment's
        // address. Asked through `edit_disposition` anyway, because hand-rolling
        // a canon-only check would be a second answer to what a file is, and
        // because a refusal here owes the caller a DESTINATION.
        if let EditDisposition::Refuse { author_with } = Project::edit_disposition(&rel) {
          return Err(refuse(format!(
            "`{path}` is generated from the model rather than authored on disk -- author it with {author_with}"
          )));
        }
        // **`edit_disposition` OPENS A PATH CANON DOES NOT CARRY, AND `put`
        // MUST NOT.** They are the same answer to different questions: `edit` may
        // open any file in the directory, because the estate holds files Intent
        // does not model and never claimed to. Canon CARRIES only the
        // attachment extensions -- so writing an unattached path here would put
        // a row into canon that `--to-store` would never have produced, and
        // that the next carry would not sustain.
        if Project::classify(&rel) != ThreadFile::Attachment {
          return Err(refuse(format!(
            "`{path}` is canon or a generated view rather than an authored file, so it has no attachment record to write"
          )));
        }

        // **AND THE SIZE, FOR THE SAME REASON AS THE ARM ABOVE.** The carrier
        // refuses a file over `ATTACHMENT_CAP_BYTES`, so a row written here for
        // a larger body is a row `--to-store` would never have produced and the
        // next carry could not sustain. **An artefact the owning pipeline
        // cannot reproduce is already drifting the moment it lands** -- which
        // is what this door's original extension check was protecting, and the
        // property survived the list that used to enforce it.
        if !crate::project::within_attachment_cap(body.len() as u64) {
          return Err(refuse(format!(
            "`{path}` is {} bytes, over the {}-byte cap, so canon has no record to write for it -- the carry would refuse it on the next pass",
            body.len(),
            crate::project::ATTACHMENT_CAP_BYTES
          )));
        }

        let row = Attachment::new(path.clone(), body);

        // **THE TEXT DOOR'S OWN REFUSAL, AND IT STAYS HERE RATHER THAN MOVING

        // DOWN.** It is about what THIS door can represent -- a caller PUTting

        // text over an opaque attachment would destroy bytes the door has no way

        // to carry. A bytes-capable door has no such limit, so pushing this into

        // the shared placement would impose one door's constraint on the other.

        if let Some(existing) = self
          .canon
          .threads
          .iter()
          .find(|t| &t.id == thread)
          .and_then(|t| t.attachments.iter().find(|a| &a.path == path))
          && existing.is_opaque()
        {
          return Err(refuse(format!(
            "`{path}` is carried as bytes and this address takes text -- rewriting it here would destroy content that nothing in this door can represent"
          )));
        }

        self.place_attachment(thread, path, row)
      }
      // Server-assigned ids. Named individually rather than falling into a
      // catch-all, so the refusal can say WHICH rule sent them away.
      AddrEntity::Threads | AddrEntity::Issue { .. } => Err(FacadeError::WriteNotAddressable {
        url: address.to_url(),
        why: "this id is server-assigned -- POST to the collection address".to_string(),
      }),
      // **REPORTED BY THE NAME THE GRAMMAR USES, NOT BY `{:?}`.** The Debug
      // repr leaks Rust struct syntax into an operator-facing message --
      // `Wp { thread: "ST0057", seq: 3 }` -- and AC-08.5 asks for the
      // unwritable thing to be reported BY NAME. `form()` is the name, and the
      // POST arm forty lines up was already using it.
      other => Err(FacadeError::WriteNotAddressable {
        url: address.to_url(),
        // **THE WORDING IS LOAD-BEARING AND NOT MINE TO IMPROVE.** AT-08.5's
        // entity sweep discriminates on the literal `has no write path yet`
        // and says so at its own line 451. Rewording it -- which this arm did
        // for one build -- silently reclassified SIX forms as reachable,
        // including two the estate refuses BY RULING. The Debug leak was the
        // defect; the sentence around it is an interface.
        why: format!("{} has no write path yet", other.form()),
      }),
    }
  }

  /// The fields of an addressed entity that [`Facade::set`] will write.
  ///
  /// **PUBLIC BECAUSE LIMB 1 ASKS FOR IT.** AC-08.5 wants *the completeness of
  /// the surface, with the unsettable set as the printed output* -- so a caller
  /// has to be able to ASK, rather than discover the boundary one refusal at a
  /// time. Derived from the model's own schema, so the answer cannot drift from
  /// what [`Facade::set`] will actually do.
  pub fn settable_fields(entity: &AddrEntity) -> Result<Vec<String>, FacadeError> {
    let declared = fields_of(entity).map_err(|fieldless| FacadeError::WriteNotAddressable {
      url: format!("{} {} address", article_for(entity.form()), entity.form()),
      why: fieldless.why(),
    })?;
    Ok(
      declared
        .into_iter()
        .filter(|field| unsettable(entity, field).is_none())
        .collect(),
    )
  }

  /// **THE ONE PLACE AN ATTACHMENT ROW REACHES CANON.**
  ///
  /// Extracted when `st attach` needed to carry bytes: a second door carries a
  /// different FORM, never a second implementation. vc named that distinction
  /// when authorising the verb -- *one capability, two doors, one of them
  /// currently fenced, is the normal shape; a second opaque-carrying
  /// implementation would be the violation.*
  fn place_attachment(
    &mut self,
    thread: &str,
    path: &str,
    row: Attachment,
  ) -> Result<Outcome, FacadeError> {
    let mut next = self.canon.clone();
    let holder = find_thread_mut(&mut next, thread)?;
    let outcome = match holder.attachments.iter_mut().find(|a| a.path == path) {
      Some(existing) => {
        if *existing == row {
          return Ok(Outcome::AlreadyThere {
            state: "unchanged".to_string(),
          });
        }
        *existing = row;
        Outcome::Moved
      }
      None => {
        holder.attachments.push(row);
        holder.attachments.sort_by(|a, b| a.path.cmp(&b.path));
        Outcome::Moved
      }
    };
    self
      .apply(
        "attachment.put",
        Subject {
          kind: "attachment".to_string(),
          id: format!("{thread}/{path}"),
        },
        json!({ "via": "address" }),
        next,
      )
      .map(|foreign| outcome.with_overwrites(foreign))
  }

  /// Write an attachment's content, carrying **the form the bytes decide**.
  ///
  /// **THIS CLOSES AC-08.5's LAST FIELD-AXIS GAP AND IT DOES NOT TOUCH THE
  /// SHADOW.** `Attachment.blob` had no route on the mutation surface, so the
  /// criterion's first clause failed on a field whose refusal correctly said so.
  /// A separate defect -- `sync.rs`'s `inspect` still refusing a non-UTF-8 file
  /// as residue while `project.rs:889` deliberately retired the same refusal --
  /// keeps the OTHER door to `Attachment::opaque` fenced. **That is a question
  /// about what `sync` classifies as residue, not about whether a field is
  /// settable, and it is filed rather than folded in** (vc's ruling): holding a
  /// row red on a defect its criterion does not describe is the loose-condition
  /// error pointed the other way.
  ///
  /// **FORM FOLLOWS CONTENT, DECIDED BY DECODING, and that rule is not mine** --
  /// `project.rs` states it as the single place the decision is made (ST0057
  /// AC-03.2), *decided by DECODING, never by the extension*. This agrees with
  /// it rather than restating it: valid UTF-8 is carried inline, anything else
  /// as bytes.
  pub fn put_attachment(
    &mut self,
    address: &Address,
    bytes: &[u8],
  ) -> Result<Outcome, FacadeError> {
    require_local(address)?;
    let AddrEntity::Attachment { thread, path } = &address.entity else {
      return Err(FacadeError::WriteNotAddressable {
        url: address.to_url(),
        why: format!(
          "{} is not an attachment address, and this door writes an attachment's content",
          address.entity.form()
        ),
      });
    };
    let refuse = |why: String| FacadeError::WriteNotAddressable {
      url: address.to_url(),
      why,
    };

    let rel = std::path::PathBuf::from(path);
    // **ASKED BEFORE `whose file is this`, because a path that names nowhere
    // has no owner to look up.** The refusals below classify a well-formed
    // path; this one refuses a path that never was (`0262`).
    if let Some(fault) = crate::project::attachment_path_fault(&rel, thread) {
      return Err(FacadeError::AttachmentPathNotInThread {
        url: address.to_url(),
        path: path.clone(),
        thread: thread.clone(),
        fault,
      });
    }
    // **THE NAMING GATE, AND IT IS THE ONE `put` AND THE INGEST ALREADY USE**
    // (`0490`). This door built its own set of checks and did not ask it, so
    // `intent st attach <ID> todo.md` wrote an attachment that `address::parse`
    // refuses to name: canon held a row no address could reach, by the door an
    // operator actually uses. The checks below classify the FILE; this one asks
    // whether the NAME is one the addressing layer will give back, which is a
    // different question and the one that was missing.
    //
    // **ASKED AFTER `attachment_path_fault` AND BEFORE THE REST, WHICH IS
    // `put`'s ORDER WITH `0262`'s MESSAGES KEPT.** An empty, absolute,
    // `..`-bearing or repo-relative path has a better message there and
    // `attachment_path_fault` says so in its own comment, so it keeps those; a
    // well-formed path that the parser will not round-trip is this one's, and
    // it precedes the view and canon refusals exactly as it does at `put`.
    // Both doors now answer the same name the same way, which is the whole
    // return on sharing the function rather than checking locally.
    crate::project::attachment_name(thread, path).map_err(|bad| refuse(bad.to_string()))?;
    if let EditDisposition::Refuse { author_with } = Project::edit_disposition(&rel) {
      return Err(refuse(format!(
        "`{path}` is generated from the model rather than authored on disk -- author it with {author_with}"
      )));
    }
    if Project::classify(&rel) != ThreadFile::Attachment {
      return Err(refuse(format!(
        "`{path}` is canon or a generated view rather than an authored file, so it has no attachment record to write"
      )));
    }
    // The same cap as the text door and as the carrier, asked of the same
    // function. A row written above it is one `--to-store` would never have
    // produced, so it drifts on the next carry rather than at the moment it is
    // noticed.
    if !crate::project::within_attachment_cap(bytes.len() as u64) {
      return Err(refuse(format!(
        "`{path}` is {} bytes, over the {}-byte cap, so canon has no record to write for it -- the carry would refuse it on the next pass",
        bytes.len(),
        crate::project::ATTACHMENT_CAP_BYTES
      )));
    }

    let row = match std::str::from_utf8(bytes) {
      Ok(text) => Attachment::new(path.clone(), text),
      Err(_) => Attachment::opaque(path.clone(), bytes.to_vec()),
    };
    let (thread, path) = (thread.clone(), path.clone());
    self.place_attachment(&thread, &path, row)
  }

  /// Remove an attachment from its thread: the record leaves the store and
  /// canon, and the file on disk is left where it is (issue 0394).
  ///
  /// **THE FILE'S FATE IS THE CALLER'S TO STATE, NOT THIS DOOR'S TO DECIDE.**
  /// Until this door, an attachment could leave a thread only by a hand edit of
  /// canon and a `sync --to-store`, which is exactly what the CLI exists to
  /// spare an operator. Deleting the file here as well would destroy authored
  /// bytes on a verb whose subject is the record, so the file stays and the
  /// caller names it -- and says that an authored file left under a thread is
  /// carried back in by the next ingest, which is the one fact that makes
  /// "left on disk" an instruction rather than a reassurance.
  ///
  /// Written through the same `Facade::apply` as [`Facade::put_attachment`],
  /// so the canon, the store and the event log move together.
  pub fn detach_attachment(&mut self, address: &Address) -> Result<Outcome, FacadeError> {
    require_local(address)?;
    let AddrEntity::Attachment { thread, path } = &address.entity else {
      return Err(FacadeError::WriteNotAddressable {
        url: address.to_url(),
        why: format!(
          "{} is not an attachment address, and this door removes an attachment",
          address.entity.form()
        ),
      });
    };
    let mut next = self.canon.clone();
    let holder = find_thread_mut(&mut next, thread)?;
    let before = holder.attachments.len();
    holder.attachments.retain(|a| a.path != *path);
    if holder.attachments.len() == before {
      return Err(FacadeError::NoSuchAttachment {
        url: address.to_url(),
        path: path.clone(),
        thread: thread.clone(),
      });
    }
    let (thread, path) = (thread.clone(), path.clone());
    self
      .apply(
        "attachment.detach",
        Subject {
          kind: "attachment".to_string(),
          id: format!("{thread}/{path}"),
        },
        json!({ "via": "address" }),
        next,
      )
      .map(|foreign| Outcome::Moved.with_overwrites(foreign))
  }

  /// `intent st relate <ID> <TARGET> [--note <text>]` (issue 0460): link a
  /// thread to another, or re-note a link it already carries.
  ///
  /// **A LINK IS A VALUE, THE TARGET AND ITS NOTE TOGETHER.** Relating a target
  /// the thread already links replaces the note, a missing `--note` included,
  /// and relating it with the note it already has writes nothing and records
  /// nothing. Repointing a link is `st_unrelate` of the old target and
  /// `st_relate` of the new one: two acts, two events.
  ///
  /// Written through the same `Facade::apply` as every other thread write, so
  /// the store, canon, the realised `info.md` and the event log move together.
  pub fn st_relate(
    &mut self,
    id: &str,
    target: &str,
    note: Option<&str>,
  ) -> Result<Outcome, FacadeError> {
    self.st_show(id)?;
    if id == target {
      return Err(FacadeError::RelatedToItself {
        thread: id.to_string(),
      });
    }
    if !self.canon.threads.iter().any(|t| t.id == target) {
      return Err(FacadeError::NoSuchRelatedTarget {
        thread: id.to_string(),
        target: target.to_string(),
      });
    }
    let note = note.map(str::to_string);
    let mut next = self.canon.clone();
    let holder = find_thread_mut(&mut next, id)?;
    match holder.related.iter_mut().find(|r| r.id == target) {
      Some(link) if link.note == note => {
        return Ok(Outcome::AlreadyThere {
          state: format!("related to {target}"),
        });
      }
      Some(link) => link.note = note.clone(),
      None => holder.related.push(crate::model::Related {
        id: target.to_string(),
        note: note.clone(),
      }),
    }
    self
      .apply(
        "st.relate",
        Subject {
          kind: "thread".to_string(),
          id: id.to_string(),
        },
        json!({ "target": target, "note": note }),
        next,
      )
      .map(|foreign| Outcome::Moved.with_overwrites(foreign))
  }

  /// `intent st unrelate <ID> <TARGET>` (issue 0460): drop a thread's link.
  ///
  /// **THE TARGET NEED NOT EXIST**, because a link to a thread that is gone is
  /// the case this verb is for: a thread adopted under a new id leaves links
  /// naming the old one, and until this verb nothing could remove them but a
  /// hand edit of canon.
  pub fn st_unrelate(&mut self, id: &str, target: &str) -> Result<Outcome, FacadeError> {
    let mut next = self.canon.clone();
    let holder = find_thread_mut(&mut next, id)?;
    let before = holder.related.len();
    holder.related.retain(|r| r.id != target);
    if holder.related.len() == before {
      return Err(FacadeError::NoSuchRelated {
        thread: id.to_string(),
        target: target.to_string(),
      });
    }
    self
      .apply(
        "st.unrelate",
        Subject {
          kind: "thread".to_string(),
          id: id.to_string(),
        },
        json!({ "target": target }),
        next,
      )
      .map(|foreign| Outcome::Moved.with_overwrites(foreign))
  }

  /// **THE NARROW FIELD-SETTER: one named field, on one addressed entity, and
  /// demonstrably nothing else** (AC-08.5).
  ///
  /// # Why this exists when `put` already writes
  ///
  /// **DC-1 (hv via vc, 2026-08-24) ruled that the standard is a FIELD-SETTER,
  /// not any path that changes the bytes.** A whole-document parse-plus-graft is
  /// not a setter and a whole-document authored replace is not a setter, so
  /// [`Facade::put`] closes limb 1 for nothing at all -- it is the door for
  /// *here is the document*, and this is the door for *set this field*.
  ///
  /// The four gaps it was built for were one shape: `Thread::completed` -- NULL
  /// on ST0011, the estate's one genuinely wrong row -- plus `WorkPackage`'s
  /// `objective`, `body` and `preamble`. **The work-package three were worse off
  /// than `completed` and that decided the design.** `put` has no `Wp` arm, and
  /// the thread door refuses `wps` BY NAME and sends the caller to that very
  /// address: two doors pointing at each other, neither opening. The only route
  /// to a work package's prose was a hand-edit of markdown and a whole-estate
  /// `sync --to-store`.
  ///
  /// **So this is generic rather than four bespoke verbs.** A named
  /// `wp objective` verb closes one gap and leaves the identical hole one field
  /// over; this closes them by construction and keeps closing them, because a
  /// field added to any of these models is settable the day it lands.
  ///
  /// # Limb 2 is an INVARIANT here, not a property some test asserts elsewhere
  ///
  /// The write is re-serialised and diffed against what was read, and **the verb
  /// REFUSES if any key other than the addressed one moved.** A serde attribute
  /// that caused collateral movement would make this fail loudly rather than
  /// leaving a test as the only thing between that field and a silent clear --
  /// which is the shape the criterion's second limb names.
  ///
  /// # `Value::Null` clears; it is not a gap
  ///
  /// An optional field's null is how a caller says *remove this*. Without it
  /// `status_reason` could be written and never unwritten -- half a setter, and
  /// the half nobody notices missing. On a REQUIRED field the typed re-parse
  /// refuses it by name, which is the same answer `put` gives.
  pub fn set(
    &mut self,
    address: &Address,
    field: &str,
    value: Value,
  ) -> Result<Outcome, FacadeError> {
    require_local(address)?;

    let url = address.to_url();
    let refuse = |field: &str, why: String| FacadeError::FieldNotWritable {
      url: url.clone(),
      field: field.to_string(),
      why,
    };

    // **THE NAME IS CHECKED BEFORE THE VALUE**, so a caller who misspells a
    // field is told they misspelled it rather than being handed a type error
    // about a field they never meant.
    let settable = Self::settable_fields(&address.entity)?;
    if !schema_properties_of(&address.entity).contains(field) {
      return Err(refuse(
        field,
        format!(
          "not a field of this entity -- the ones it will set are {}",
          settable.join(", ")
        ),
      ));
    }
    if let Some(why) = unsettable(&address.entity, field) {
      return Err(refuse(field, why.explain(&url)));
    }

    let mut next = self.canon.clone();
    let (op, subject) = match &address.entity {
      AddrEntity::Thread { id } => {
        let existing = find_thread_mut(&mut next, id)?;
        let Some(mut row) = Self::splice_one_field(existing, field, value, &refuse)? else {
          return Ok(Outcome::AlreadyThere {
            state: "unchanged".to_string(),
          });
        };
        // **THE COMPLETION DATE OBEYS THE STATUS WRITER'S RULE AT THIS DOOR
        // TOO** (issue 0504). The splice re-parses the row by the model's own
        // types, and `completed` is declared a string, so every string passed
        // and nothing here read the thread's status: `not-a-date` landed in
        // canon, and so did a completion date on a thread in Triage. This is
        // the Ac arm's post-splice shape, one field over.
        if field == "completed" {
          match row.completed.as_deref() {
            Some(stated) => row.completed = Some(Self::recordable_completion(row.status, stated)?),
            // **A NULL CLEARS A STRAY DATE AND NEVER A REAL ONE.** On a thread
            // that closed, clearing is how ST0011's NULL-completed row came
            // about, and this door exists to repair that row rather than to
            // make another; under any other status the date should never have
            // been there, and clearing it is the repair.
            None
              if matches!(
                row.status,
                ThreadStatus::Completed | ThreadStatus::Cancelled
              ) =>
            {
              return Err(refuse(
                field,
                format!(
                  "a {} thread keeps its completion date -- restate it with a date rather than clearing it",
                  crate::model::enum_str(&row.status)
                ),
              ));
            }
            None => {}
          }
        }
        *existing = row;
        (
          "thread.set",
          Subject {
            kind: "thread".to_string(),
            id: id.to_string(),
          },
        )
      }
      AddrEntity::Wp { thread, wp } => {
        let seq = Self::wp_seq(address, wp)?;
        let existing = find_wp_mut(&mut next, thread, seq)?;
        let Some(row) = Self::splice_one_field(existing, field, value, &refuse)? else {
          return Ok(Outcome::AlreadyThere {
            state: "unchanged".to_string(),
          });
        };
        *existing = row;
        (
          "wp.set",
          Subject {
            kind: "wp".to_string(),
            id: format!("{thread}/{seq:02}"),
          },
        )
      }
      AddrEntity::Ac { thread, ac } => {
        let existing = find_criterion_mut(&mut next, thread, ac)?;
        let Some(mut row) = Self::splice_one_field(existing, field, value, &refuse)? else {
          return Ok(Outcome::AlreadyThere {
            state: "unchanged".to_string(),
          });
        };
        if field == "kind" && !row.state.permitted_for(row.kind) {
          row.state = Self::rekinded_state(ac, &row, &refuse)?;
        }
        *existing = row;
        (
          "ac.set",
          Subject {
            kind: "ac".to_string(),
            id: format!("{thread}/{ac}"),
          },
        )
      }
      AddrEntity::At { thread, at } => {
        let existing = find_test_mut(&mut next, thread, at)?;
        let Some(row) = Self::splice_one_field(existing, field, value, &refuse)? else {
          return Ok(Outcome::AlreadyThere {
            state: "unchanged".to_string(),
          });
        };
        Self::refuse_a_file_written_onto_a_non_test_row(thread, existing, &row)?;
        *existing = row;
        (
          "at.set",
          Subject {
            kind: "at".to_string(),
            id: format!("{thread}/{at}"),
          },
        )
      }
      AddrEntity::Issue { id } => {
        let n =
          crate::model::normalise_issue_id(id).map_err(|_| FacadeError::WriteNotAddressable {
            url: address.to_url(),
            why: format!("`{id}` is not an issue id"),
          })?;
        let existing = find_issue_mut(&mut next, n)?;
        let Some(row) = Self::splice_one_field(existing, field, value, &refuse)? else {
          return Ok(Outcome::AlreadyThere {
            state: "unchanged".to_string(),
          });
        };
        *existing = row;
        (
          "issue.set",
          Subject {
            kind: "issue".to_string(),
            id: format!("{n:04}"),
          },
        )
      }
      // Unreachable in practice -- `settable_fields` above refuses every other
      // form first. Named rather than `unreachable!()` so a fifteenth entity
      // form that someone teaches `settable_fields` cannot reach a panic here.
      other => {
        return Err(FacadeError::WriteNotAddressable {
          url: address.to_url(),
          why: format!("`{}` has no narrow setter", other.form()),
        });
      }
    };

    self
      .apply(
        op,
        subject,
        json!({ "via": "address", "field": field }),
        next,
      )
      .map(|foreign| Outcome::Moved.with_overwrites(foreign))
  }

  /// The seq a `wp` address segment names.
  ///
  /// **Refused by name rather than defaulted.** `address.rs` mints the segment
  /// as `{seq:02}` but the grammar accepts what a caller types, so a
  /// non-numeric segment reaches here -- and silently choosing a work package
  /// for somebody is worse than telling them the address is wrong.
  fn wp_seq(address: &Address, wp: &str) -> Result<u32, FacadeError> {
    wp.parse::<u32>()
      .map_err(|_| FacadeError::WriteNotAddressable {
        url: address.to_url(),
        why: format!("`{wp}` is not a work-package sequence number"),
      })
  }

  /// Replace ONE key of a serialised entity and return the row that results, or
  /// `None` when the value asked for is the value it already holds.
  ///
  /// **THE COLLATERAL CHECK IS THE REASON THIS GOES THROUGH JSON** rather than
  /// matching on field names and assigning to struct members. A hand-written
  /// match sets exactly what it names, which sounds like the safer construction
  /// and is the one that cannot be AUDITED: nothing about it can observe that a
  /// second field moved. Going out to `Value` and back means the before and
  /// after are directly comparable, so limb 2 is checked on every single call
  /// rather than asserted about the code by a reader.
  /// **THE STATE A CRITERION TAKES WHEN ITS KIND FLIPS UNDER IT** (issue 0346).
  ///
  /// `set <ac> kind` wrote the kind and kept the state, so a `computed` row
  /// flipped to `non-test` became the pair `AcState::permitted_for` forbids:
  /// doctor refused the canon and `ac satisfy` refused the row, leaving it stuck.
  /// A state the new kind cannot hold re-enters at `AcState::entry`, which is
  /// the state a criterion of that kind is created in -- but only where nothing
  /// is lost. A recorded satisfaction or an unsatisfied note has no home on a
  /// test-backed criterion, so those flips are refused with the verb that
  /// clears them, rather than dropped without a word.
  fn rekinded_state(
    ac: &str,
    row: &Criterion,
    refuse: &dyn Fn(&str, String) -> FacadeError,
  ) -> Result<AcState, FacadeError> {
    match &row.state {
      AcState::Computed {} | AcState::Unsatisfied { note: None } => Ok(AcState::entry(row.kind)),
      AcState::Satisfied { .. } => Err(refuse(
        "kind",
        format!(
          "{ac} is satisfied, and a test-backed criterion's satisfaction is computed rather than recorded -- \
           reopen it with `intent ac unsatisfy` first, then set its kind"
        ),
      )),
      AcState::Unsatisfied { note: Some(note) } => Err(refuse(
        "kind",
        format!(
          "{ac} carries the note `{note}`, and a test-backed criterion has nowhere to keep it -- \
           record it elsewhere and clear it with `intent ac edit` first, then set its kind"
        ),
      )),
      other => Ok(other.clone()),
    }
  }

  fn splice_one_field<T>(
    current: &T,
    field: &str,
    value: Value,
    refuse: &dyn Fn(&str, String) -> FacadeError,
  ) -> Result<Option<T>, FacadeError>
  where
    T: serde::Serialize + serde::de::DeserializeOwned + PartialEq,
  {
    let before = serde_json::to_value(current)
      .map_err(|e| refuse(field, format!("this entity does not serialise: {e}")))?;
    let mut spliced = before.clone();
    let object = spliced
      .as_object_mut()
      .ok_or_else(|| refuse(field, "this entity is not a JSON object".to_string()))?;
    if value.is_null() {
      object.remove(field);
    } else {
      object.insert(field.to_string(), value);
    }

    // **A REQUIRED FIELD CLEARED, A WRONG TYPE, AND AN ENUM SPELLED WRONGLY ALL
    // LAND HERE**, which is why the typed re-parse is the validator rather than
    // a hand-written check per field. `deny_unknown_fields` and the model's own
    // enums do the work, and they cannot fall behind the model.
    let next: T = serde_json::from_value(spliced)
      .map_err(|e| refuse(field, format!("`{field}` will not take that value: {e}")))?;

    if next == *current {
      return Ok(None);
    }

    let after = serde_json::to_value(&next)
      .map_err(|e| refuse(field, format!("the result does not serialise: {e}")))?;
    let moved: Vec<&String> = before
      .as_object()
      .into_iter()
      .flat_map(|o| o.keys())
      .chain(after.as_object().into_iter().flat_map(|o| o.keys()))
      .collect::<std::collections::BTreeSet<_>>()
      .into_iter()
      .filter(|key| before.get(*key) != after.get(*key))
      .collect();
    if moved != vec![&field.to_string()] {
      return Err(refuse(
        field,
        format!(
          "setting it would also have moved {} -- refused rather than written. A write that \
           changes a field you did not name would clear it without saying so; set each field \
           in its own call",
          moved
            .iter()
            .filter(|key| **key != field)
            .map(|key| format!("`{key}`"))
            .collect::<Vec<_>>()
            .join(", ")
        ),
      ));
    }

    Ok(Some(next))
  }

  pub fn at_list(&self, st: &str) -> Result<&[AcceptanceTest], FacadeError> {
    Ok(&self.st_show(st)?.tests)
  }

  // -------------------------------------------------------------------------
  // Issue lifecycle -- MACHINE 4
  //
  // **These three were blocked on a ratification for two days, not on effort.**
  // `Issue.status` was `Disposition::Unbuilt`: `Closed` was a value authored
  // canon could put there with no verb to leave it, which is what AC-04.6's
  // second condition names. Building `close` and `open` meant declaring the
  // `open <-> closed` edges, and declaring edges is declaring a machine -- so
  // it went to hv rather than being written on my own authority, however
  // obvious the two edges look. hv ratified Machine 4 on 2026-08-17.
  //
  // **v2 HAD these verbs, which makes this a REGRESSION being closed rather
  // than a feature.** That distinction decided the parity posture: the row is
  // `keep` with `target.state: as-observed`, so the graph and the strings both
  // come from `bin/intent_issues` and nothing here is a design choice.
  // -------------------------------------------------------------------------

  /// Raise an issue. The number is the next free one.
  ///
  /// **`created` goes in EMPTY and comes back filled** (D42) -- same idiom as
  /// [`Facade::st_new`]. Nothing here knows what day it is, the store fills the
  /// date inside the INSERT, and `apply` renders the extract from what landed.
  ///
  /// **The severity DEFAULT is the caller's, not this function's.** v2 defaults
  /// `--severity` to `medium` in its flag parsing, and the dispatch row carries
  /// that default, so the flag's default belongs to the surface. `None` here
  /// means nobody said -- which `issues list` already renders as `?` rather than
  /// as a blank, deliberately.
  ///
  /// **`reporter` is the caller's for the same reason `severity` is, and it is
  /// the CREATE door for a field whose RESTORE door arrived with WP-10.** The
  /// migration carries a reporter v2 recorded; this records the one raising it
  /// now. Building only the restore half is the defect this estate already paid
  /// for once, one field over -- `write_issue` was `write_thread` with the
  /// create door missing, and it was correct only because every caller was
  /// `rebuild`. **The door is a property of the ACT, not of the entity.**
  ///
  /// It is NOT taken from `Ctx::principal`, which is the hard-coded `local`
  /// until the 3.2 agent bus gives principals meaning. Writing that here would
  /// assert every issue was reported by somebody called `local` -- a wrong
  /// value where `None` at least reads as nobody said.
  pub fn issue_add(
    &mut self,
    title: &str,
    severity: Option<&str>,
    reporter: Option<&str>,
    body: &str,
  ) -> Result<u32, FacadeError> {
    let number = self.next_issue_number();
    let issue = crate::model::Issue {
      schema: crate::model::ISSUE_SCHEMA.to_string(),
      number,
      slug: slugify(title),
      title: title.to_string(),
      status: IssueStatus::Open,
      severity: severity.map(str::to_string),
      created: String::new(),
      closed: None,
      reporter: reporter.map(str::to_string),
      // **THE CREATE DOOR, RULED GO BY hv 2026-08-27.** This field was declared
      // in the model, carried through canon, and reachable by NO verb: `issues
      // add` took a title and a severity, so the whole of an issue's prose had
      // to be written by editing the file -- a route that stops existing under
      // the disk-optional model. Measured on this estate at the time of the
      // ruling: 57 of 78 issues carry a non-empty body.
      //
      // Still empty when nobody passes one, which is a state and not a gap --
      // inventing a template here would put prose in the record that no author
      // wrote, the same reasoning that keeps template-identical sections out of
      // `Thread.body`.
      body: body.to_string(),
    };
    // **THE MANIFEST EDIT COMES FIRST, WHICH IS `edit_list`'s OWN DOCUMENTED
    // ORDER FOR AN ADDITION** -- *manifest first for an addition leaves an
    // artefact listed whose status did not move ... and one that deletes
    // nothing*. It was second here, and `apply` runs the projection: so the
    // projection asked the manifest whether this issue was declared BEFORE the
    // line declaring it existed, decided no, and skipped the view. Nothing
    // failed; the file simply never appeared.
    let before = std::fs::read_to_string(self.project.intentfiles_path()).ok();
    self.edit_list("issues.add", &format!("{number:04}"), ListEdit::AsDeclared)?;
    let mut next = self.canon.clone();
    next.issues.push(issue);
    // Issue 0376: a refused add left its ISSUE row in the manifest, and a landed add's failed step went unreported.
    match self.apply(
      "issues.add",
      Subject {
        kind: "issue".to_string(),
        id: format!("{number:04}"),
      },
      json!({"title": title, "severity": severity}),
      next,
    ) {
      Ok(applied) => self.park(applied),
      Err(refused) => {
        self.restore_manifest(before)?;
        return Err(refused);
      }
    }
    Ok(number)
  }

  /// Correct an issue's authored record: its prose, its title, its severity.
  ///
  /// **THE VERB hv RULED ON 2026-08-28/29 AND vc RE-DISCOVERED AS A DEFECT FOUR
  /// DAYS LATER.** `issues add` could WRITE a body from the day the create door
  /// landed, and nothing could ever correct one: an issue body was write-once,
  /// so a filing with a wrong premise stayed wrong permanently. vc filed that as
  /// `0179` -- *an issue body is write-once* -- as a fresh finding, unable to see
  /// hv's ruling because a fold had moved it to `.history/`.
  ///
  /// **cc MET THE SAME WALL WHILE BUILDING THE FIX'S NEIGHBOUR**: `0183` was
  /// filed with a remedy that measurement later proved wrong, and the body could
  /// not be corrected. Two instances in one session, both in the record of a
  /// finding about the thing this verb is. That is the evidence hv re-sequenced
  /// the package on.
  ///
  /// # Title and severity, and why they arrived a release later than the body
  ///
  /// **THE BODY DOOR SHIPPED ALONE AND LEFT TWO FIELDS STILL WRITE-ONCE.**
  /// `issues add` sets a title (a required positional) and a severity (a flag),
  /// and until this method took them nothing could change either: both were
  /// writable exactly once, at creation, forever. `0154` predicted precisely
  /// this half-fix in advance -- it asked for an edit door covering title AND
  /// body, and warned that a one-field fix *leaves the case that prompted the
  /// filing exactly where it is* -- and `0151` owns the title row.
  ///
  /// **EACH FIELD IS `Option` BECAUSE NOT ADDRESSING A FIELD AND EMPTYING IT
  /// ARE DIFFERENT ACTS.** `None` leaves the field alone; `Some("")` is an
  /// erasure and is refused. A signature taking three `&str` could not tell
  /// them apart, which would make "correct the title" silently erase the prose.
  ///
  /// # An empty value is REFUSED, and that is not symmetry with `add`
  ///
  /// `issue_add` leaves the body empty when nobody passes one, because an
  /// unwritten body is a STATE. Here an empty body is an ERASURE: it destroys
  /// authored prose with nothing to recover it from, since the previous value
  /// exists only in the event log. **The two verbs face opposite directions over
  /// the same field** -- one has nothing to lose and the other has everything --
  /// so they get opposite defaults, and this refuses rather than treating
  /// "no prose given" as "make it have no prose".
  ///
  /// A title has no such state in either direction: `add` requires one, so there
  /// is no issue anywhere carrying an empty title and no act that should create
  /// the first. **The two refusals name their own field**, because `AC-04.4`
  /// forbids one message for two causes and an operator who emptied the title
  /// learns nothing from a sentence about prose.
  ///
  /// # `AlreadyThere` rather than a write, when nothing given would move
  ///
  /// The same discipline `set_issue_status` uses: re-writing identical bytes
  /// would emit an event saying something changed, and a reader of the log
  /// cannot tell a real correction from a no-op after the fact. With three
  /// fields the test is that NONE of the ones addressed would move -- a call
  /// that corrects a title and passes the body it already has is a real
  /// correction, not a no-op.
  pub fn issue_edit(
    &mut self,
    number: u32,
    body: Option<&str>,
    title: Option<&str>,
    severity: Option<&str>,
  ) -> Result<Outcome, FacadeError> {
    if let Some(b) = body
      && b.trim().is_empty()
    {
      return Err(FacadeError::ValueNotRecordable {
        field: "body".to_string(),
        given: b.to_string(),
        why: "an empty body would ERASE the issue's prose, and the only copy of what it said \
              is the event log. `issues add` may leave a body empty because nothing is lost; \
              correcting one to empty is a different act"
          .to_string(),
      });
    }
    if let Some(t) = title
      && t.trim().is_empty()
    {
      return Err(FacadeError::ValueNotRecordable {
        field: "title".to_string(),
        given: t.to_string(),
        why: "an issue with no title cannot be told from any other in `issues list`, and \
              `issues add` takes the title as a required positional for that reason. Unlike a \
              body, there is no state in which this field is legitimately empty"
          .to_string(),
      });
    }

    // Read once, into OWNED values. `issue_show` hands back a borrow of
    // `self.canon`, and everything below this needs `&mut self` -- so the
    // comparison has to finish before the mutation starts, rather than the
    // borrow being held across it.
    let (was_body, was_title, was_severity) = {
      let c = self.issue_show(number)?;
      (c.body.clone(), c.title.clone(), c.severity.clone())
    };

    // **WHAT WAS ADDRESSED IS NOT WHAT WOULD CHANGE**, and only the second is
    // recordable. A caller passing the value a field already holds has asked
    // for nothing, and the event log must not say otherwise.
    let body_change = body.filter(|b| *b != was_body.as_str());
    let title_change = title.filter(|t| *t != was_title.as_str());
    let severity_change = severity.filter(|s| was_severity.as_deref() != Some(*s));

    if body_change.is_none() && title_change.is_none() && severity_change.is_none() {
      // Names what was already true rather than a fixed sentence: the caller may
      // have addressed three fields and needs to know it was all three.
      let mut held: Vec<&str> = Vec::new();
      if body.is_some() {
        held.push("that prose");
      }
      if title.is_some() {
        held.push("that title");
      }
      if severity.is_some() {
        held.push("that severity");
      }
      return Ok(Outcome::AlreadyThere {
        state: format!("carrying exactly {}", held.join(" and ")),
      });
    }

    let mut next = self.canon.clone();
    let issue = next
      .issues
      .iter_mut()
      .find(|i| i.number == number)
      .ok_or(FacadeError::NoSuchIssue { number })?;
    if let Some(b) = body_change {
      issue.body = b.to_string();
    }
    if let Some(t) = title_change {
      // **THE SLUG IS DERIVED FROM THE TITLE AND HAS TO BE RE-DERIVED WITH IT.**
      // `issue_add` computes it once through `slugify` and nothing recomputes it
      // afterwards, so a retitle that left it alone would ship a row whose slug
      // describes a title the issue no longer has. That is the divergent-copy
      // shape at FIELD scale rather than module scale, and it is worse than the
      // usual case because both copies live inside one record and no reader
      // compares them.
      issue.title = t.to_string();
      issue.slug = slugify(t);
    }
    if let Some(s) = severity_change {
      issue.severity = Some(s.to_string());
    }

    // **THE PAYLOAD CARRIES A SIZE AND NOT THE PROSE**, matching `issues.add`,
    // which records title and severity and never the body. An event log that
    // copied every body would become a second home for the prose -- and the one
    // that grows without bound while nothing reads it. Title and severity ARE
    // carried, because `issues.add` carries them, and a log recording a retitle
    // without the new title could not answer what it was retitled to.
    //
    // **ONLY WHAT CHANGED APPEARS**, built as a map rather than a fixed shape
    // with nulls: an absent key says the field was not moved, where
    // `"title": null` would say it was moved to nothing.
    let mut payload = serde_json::Map::new();
    if let Some(b) = body_change {
      payload.insert("bytes".to_string(), json!(b.len()));
    }
    if let Some(t) = title_change {
      payload.insert("title".to_string(), json!(t));
    }
    if let Some(s) = severity_change {
      payload.insert("severity".to_string(), json!(s));
    }

    // Issue 0376: this reported `Moved` whatever the projection said, so a landed edit's failed step went unreported.
    let applied = self.apply(
      "issues.edit",
      Subject {
        kind: "issue".to_string(),
        id: format!("{number:04}"),
      },
      serde_json::Value::Object(payload),
      next,
    )?;
    Ok(Outcome::Moved.with_overwrites(applied))
  }

  /// Close an issue.
  pub fn issue_close(&mut self, number: u32) -> Result<Outcome, FacadeError> {
    self.set_issue_status(number, IssueStatus::Closed, "issues.close")
  }

  /// Reopen a closed issue.
  pub fn issue_open(&mut self, number: u32) -> Result<Outcome, FacadeError> {
    self.set_issue_status(number, IssueStatus::Open, "issues.open")
  }

  /// The shared setter, in the same shape as [`Facade::set_thread_status`] and
  /// for the same reasons -- self-loop test FIRST, then the declared graph.
  ///
  /// **The self-loop here is not a nicety, it is the parity case.** v2's
  /// `move_issue` looks in the source bucket and, finding nothing, looks in the
  /// TARGET before erroring: an already-closed issue gets `already CLOSED` at
  /// exit 0 and an absent one gets a refusal. That behaviour is where hv's
  /// self-loop ruling took its citation from, so reproducing it is what the
  /// `keep` disposition means -- and the two conditions v2 tells apart, this
  /// must also tell apart.
  ///
  /// **No guard call, and the absence is checked rather than assumed.** Machine
  /// 4 declares none; `Guard::ReasonRecorded` is the only variant that could
  /// apply to an issue and `Issue` has no field to record one in. See the row in
  /// `transitions.rs` for what a guard added here would cost.
  fn set_issue_status(
    &mut self,
    number: u32,
    status: IssueStatus,
    op: &'static str,
  ) -> Result<Outcome, FacadeError> {
    let from = self.issue_show(number)?.status;
    if from == status {
      return Ok(Outcome::AlreadyThere {
        state: from.display().to_string(),
      });
    }
    Self::check_transition(
      "Issue",
      "status",
      op,
      &crate::model::enum_str(&from),
      &format!("{number:04}"),
    )?;
    let mut next = self.canon.clone();
    let issue = next
      .issues
      .iter_mut()
      .find(|i| i.number == number)
      .ok_or(FacadeError::NoSuchIssue { number })?;
    issue.status = status;
    // The same three-state sentinel `thread.completed` uses: `Some("")` asks
    // the database for today, `None` clears it. Reopening an issue drops the
    // close date because it describes a state that has ended -- the same
    // reasoning as `st resume` clearing a hold reason.
    issue.closed = match status {
      IssueStatus::Closed => Some(String::new()),
      IssueStatus::Open => None,
    };
    // Before `apply` for the reason `issue_add` records: the projection runs
    // inside it and reads the manifest, so an edit made afterwards is invisible
    // to the very write it is supposed to govern.
    let before = std::fs::read_to_string(self.project.intentfiles_path()).ok();
    self.edit_list(op, &format!("{number:04}"), ListEdit::AsDeclared)?;
    // Issue 0376: closes refused on a locked store had already taken their ISSUE rows out of the manifest.
    let applied = match self.apply(
      op,
      Subject {
        kind: "issue".to_string(),
        id: format!("{number:04}"),
      },
      json!({
        "from": crate::model::enum_str(&from),
        "to": crate::model::enum_str(&status),
      }),
      next,
    ) {
      Ok(applied) => applied,
      Err(refused) => {
        self.restore_manifest(before)?;
        return Err(refused);
      }
    };
    Ok(Outcome::Moved.with_overwrites(applied))
  }

  /// The next free issue number.
  ///
  /// **Highest-plus-one, not count-plus-one**, for the reason
  /// [`Facade::next_thread_id`] is: a project whose issues are not contiguous --
  /// and Intent's own are not, once anything is ever removed -- would otherwise
  /// be handed a number that is already taken.
  fn next_issue_number(&self) -> u32 {
    self
      .canon
      .issues
      .iter()
      .map(|i| i.number)
      .max()
      .unwrap_or(0)
      + 1
  }

  // -------------------------------------------------------------------------
  // The one write path
  // -------------------------------------------------------------------------

  /// Land a mutation across canon, views and the DB -- completely or not at
  /// all -- and record its envelope.
  ///
  /// EVERY mutating verb routes through here. That is not tidiness: AC-04.5
  /// requires an event-log envelope on every mutation path, and a second write
  /// path is how one of them would come to be missing it.
  /// Apply a mutation that touches no project-level state.
  ///
  /// **THE PLAIN SPELLING IS THE ONE FOUR CALLERS WANT, AND IT DELEGATES** --
  /// the same shape as `st_new` beside `st_new_listing`. One body underneath,
  /// so there is no second mutation path to drift, and the callers that care
  /// about project state are the only ones that have to say so.
  fn apply(
    &mut self,
    op: &str,
    subject: Subject,
    payload: serde_json::Value,
    next: Canon,
  ) -> Result<Applied, FacadeError> {
    self.apply_with_state(
      op,
      subject,
      payload,
      next,
      crate::store::ProjectStateEdit::Unchanged,
    )
  }

  fn apply_with_state(
    &mut self,
    op: &str,
    subject: Subject,
    payload: serde_json::Value,
    next: Canon,
    project_state: crate::store::ProjectStateEdit,
  ) -> Result<Applied, FacadeError> {
    let envelope = Envelope::minted(
      &self.ctx.principal,
      &self.ctx.project_id,
      op,
      subject,
      payload,
    );
    self.apply_envelopes(vec![envelope], next, project_state)
  }

  /// Apply an ancestor's fiat close DOWNWARD, to exactly the children the
  /// ratified machines say can take one.
  ///
  /// **THE RULE IS THE TABLES READ DOWNWARD, NOT A POLICY** (hv ruled the cascade
  /// 2026-08-28; vc confirmed 2026-08-30 that this half needs no ruling because
  /// it is derived). A child is closed iff its own machine declares an `fc` edge
  /// from the state it is IN -- `transitions::permits` is the whole predicate,
  /// so the from-sets have one home and this cannot drift from them.
  ///
  /// **Every skip it produces is one that would have been argued for separately**,
  /// which is the evidence the derivation is right rather than merely convenient:
  /// a `satisfied` AC is skipped because a fiat close is how an UNMET requirement
  /// is closed and offering it on a met one would let close-on-authority
  /// overwrite close-on-evidence; a `green` AT for the same reason; a `done` or
  /// `cancelled` package because it is already closed; `descoped` and `withdrawn`
  /// because they are off scope entirely.
  ///
  /// **A SKIPPED CHILD WRITES NO EVENT, AND THAT IS ANSWERABLE RATHER THAN LOST**
  /// -- but only under the one-event-per-entity ruling. *Was this child considered
  /// and skipped?* is derivable by reading the machine against the child's state
  /// AT THE TIME, and its state at the time is recoverable only because every
  /// state change writes its own event. **So the event-count ruling is what makes
  /// the skip derivable, and one-event-per-cascade would have left it genuinely
  /// unanswerable.**
  ///
  /// `scope` is `None` for a thread close and `Some(seq)` for a work package,
  /// selecting the AC group the way [`crate::contract::Scope`] already does.
  /// Returns one envelope per child MOVED, to ride the ancestor's transaction.
  fn cascade_fiat(
    ctx: &FacadeContext,
    thread: &mut Thread,
    scope: Option<u32>,
    ancestor: &str,
    ancestor_event: &str,
    record: &crate::model::FiatRecord,
  ) -> Vec<Envelope> {
    let inherited = |extra: Option<&str>| crate::model::FiatRecord {
      because: record.because.clone(),
      by: record.by.clone(),
      // D42: the write stamps it, here as everywhere.
      at: String::new(),
      invoker: record.invoker.clone(),
      inherited_from: Some(extra.unwrap_or(ancestor).to_string()),
      inherited_event: Some(ancestor_event.to_string()),
    };
    let group = scope.map(|seq| format!("{seq:02}"));
    let in_scope = |id: &str| match &group {
      // `AC-03.2` belongs to WP-03: the group is between the dash and the dot.
      Some(g) => id
        .split_once('-')
        .and_then(|(_, rest)| rest.split_once('.'))
        .is_some_and(|(prefix, _)| prefix == g),
      None => true,
    };
    let thread_id = thread.id.clone();
    let mut out = Vec::new();
    let mut mint = |op: &str, kind: &str, id: String| {
      out.push(Envelope::minted(
        &ctx.principal,
        &ctx.project_id,
        op,
        Subject {
          kind: kind.to_string(),
          id,
        },
        json!({
          "because": record.because,
          "by": record.by,
          "inherited_from": ancestor,
          "inherited_event": ancestor_event,
        }),
      ));
    };

    // Work packages, thread close only -- a package close does not reach its
    // siblings.
    if scope.is_none() {
      for wp in &mut thread.wps {
        if transitions::permits(
          "WorkPackage",
          "status",
          "wp.fc",
          &crate::model::enum_str(&wp.status),
        ) {
          wp.status = WpStatus::Done;
          wp.fiat = Some(inherited(None));
          mint("wp.fc", "wp", format!("{thread_id}/{:02}", wp.seq));
        }
      }
    }

    // Criteria in scope.
    let mut closed_acs: Vec<String> = Vec::new();
    for ac in &mut thread.criteria {
      if in_scope(&ac.id) && transitions::permits("Criterion", "state", "ac.fc", ac.state.name()) {
        ac.state = crate::model::AcState::Fiat(inherited(None));
        closed_acs.push(ac.id.clone());
        mint("ac.fc", "ac", format!("{thread_id}/{}", ac.id));
      }
    }

    // Acceptance tests covering a criterion this cascade just closed. **Reached
    // through `covers` rather than through the id group**, because an AT's
    // number does not have to match the AC it covers and the coverage list is
    // the modelled relationship.
    for at in &mut thread.tests {
      let covers_closed = at.covers.iter().any(|c| closed_acs.contains(c));
      if covers_closed
        && transitions::permits(
          "AcceptanceTest",
          "status",
          "at.fc",
          &crate::model::enum_str(&at.status),
        )
      {
        at.status = AtStatus::Fiat;
        at.fiat = Some(inherited(None));
        mint("at.fc", "at", format!("{thread_id}/{}", at.id));
      }
    }

    out
  }

  /// The one implementation, taking envelopes ALREADY MINTED.
  ///
  /// **A WRAPPER OVER ONE IMPLEMENTATION, and the reason is the fiat cascade**
  /// (vc, 2026-08-30). A cascade writes one event per entity it moves, and every
  /// child's record carries the ANCESTOR'S EVENT ID -- so the ancestor's envelope
  /// must exist before the children's records are built, which it cannot if this
  /// function mints it.
  ///
  /// **`Envelope::minted` generates the ULID in Rust, so the id IS knowable before
  /// the write, and nothing here had to start returning it.** That is consistent
  /// with D42 rather than an exception to it: the ULID is an IDENTITY and the `ts`
  /// is the STAMP, and it is only the stamp the write applies. The id being
  /// available up front is what makes one transaction possible for the whole
  /// cascade; had the database assigned it, the children would have needed a
  /// second transaction to learn it.
  ///
  /// The FIRST envelope is the act the caller invoked; the rest are what it
  /// reached, and they share its transaction.
  /// **Returns the paths whose bytes were NOT the store's own render**, project-
  /// relative -- the write set's members that this mutation overwrote and that
  /// held somebody else's work when it did.
  ///
  /// It returns them rather than printing or logging them because a library
  /// says what happened and a face decides how to show it; the caller folds
  /// them into its [`Outcome`] through [`Outcome::with_overwrites`], so the
  /// note reaches every face rather than only the CLI (issue 0158's argument,
  /// one note along).
  fn apply_envelopes(
    &mut self,
    envelopes: Vec<Envelope>,
    mut next: Canon,
    project_state: crate::store::ProjectStateEdit,
  ) -> Result<Applied, FacadeError> {
    // What gets written is DIFFED, not declared. The caller used to hand in a
    // list of touched ids, which made "the mutation did not persist" reachable
    // by naming the wrong id -- a silent failure, since the DB and the return
    // value would both say it worked. Comparing against the loaded canon
    // cannot forget, and it generalises to issues for free.
    //
    // **The diff is kept as IDS rather than as references** (D42). The
    // database fills a new thread's `created` and hands it back, so `next` has
    // to be patched with what actually landed before the files are rendered
    // from it -- and a `Vec<&Thread>` borrowed from `next` would still be alive
    // at that point. Ids re-resolve at each use site and cost nothing here.
    let changed_thread_ids: Vec<String> = next
      .threads
      .iter()
      .filter(|t| {
        !self
          .canon
          .threads
          .iter()
          .any(|current| current.id == t.id && current == *t)
      })
      .map(|t| t.id.clone())
      .collect();
    let changed_issue_numbers: Vec<u32> = next
      .issues
      .iter()
      .filter(|i| {
        !self
          .canon
          .issues
          .iter()
          .any(|current| current.number == i.number && current == *i)
      })
      .map(|i| i.number)
      .collect();
    // **AN AUTHORED BODY IS NEVER EMPTIED BY A WRITE THAT WAS NOT ASKED TO
    // EMPTY IT, AND THE COMPARISON IS AGAINST THE FILE ABOUT TO BE
    // OVERWRITTEN.**
    //
    // Lamplight, 2026-08-26: `issues close 5` reported `ok:` and took 4934
    // bytes of prose with it. Every step was correct by its own lights -- the
    // model held an empty body, the write wrote what it was given -- so there
    // was no error anywhere to surface. **A silence with no error in it cannot
    // be fixed by handling errors better; it needs something that compares.**
    //
    // **Against the DISK rather than against `self.canon`, and that is the
    // whole point.** `self.canon` is what this write is derived from, so
    // comparing to it can only ever agree with itself. The file is the thing
    // that loses the prose, so the file is what gets asked.
    //
    // Field-level rather than byte-level, which is what lets it live beside
    // `refuse_if_this_would_empty_a_populated_face` instead of duplicating it:
    // that guard's byte-shrink arm is gated off on a healthy estate because a
    // shrinking file is ordinary. **Emptying an authored body is not ordinary,
    // and asking about the field says so where asking about the size cannot.**
    for issue in next
      .issues
      .iter()
      .filter(|i| changed_issue_numbers.contains(&i.number))
      .filter(|i| i.body.is_empty())
    {
      let path = self.project.issue_json(issue.number);
      // Unreadable or absent is not evidence of loss. A new issue has no file
      // yet, and a file this cannot parse is a different report from a
      // different surface -- inventing a refusal from either would refuse the
      // ordinary path, which is how a guard gets disabled rather than fixed.
      let Ok(text) = std::fs::read_to_string(&path) else {
        continue;
      };
      let Ok(on_disk) = serde_json::from_str::<Issue>(&text) else {
        continue;
      };
      if !on_disk.body.is_empty() {
        return Err(FacadeError::WriteWouldEmptyAnAuthoredBody {
          subject: format!("issue {:04}", issue.number),
          had: on_disk.body.len(),
        });
      }
    }

    let removed_threads: Vec<String> = self
      .canon
      .threads
      .iter()
      .filter(|current| !next.threads.iter().any(|t| t.id == current.id))
      .map(|t| t.id.clone())
      .collect();
    let removed_issues: Vec<u32> = self
      .canon
      .issues
      .iter()
      .filter(|current| !next.issues.iter().any(|i| i.number == current.number))
      .map(|i| i.number)
      .collect();

    // **No time is read here, and that is D42.** The envelope is minted
    // without one and the database stamps it as part of the INSERT, inside
    // the same transaction as the rows it describes. Reading a clock and
    // writing the value would hold it across a gap the write could be
    // retried or deferred inside.
    // The prose index is refreshed with the derived tables, not left behind.
    //
    // Prose is DERIVED FROM CANON (D28), so a mutation that rebuilt the model
    // and left `doc_sections` alone would leave `intent search` answering from
    // the previous model -- silently, since a search that finds nothing looks
    // exactly like a search with no matches.
    //
    // **THE WHOLE INDEX, FROM THE WHOLE POST-MUTATION MODEL** (issue 0234).
    // This used to keep part of the previous index and recompute the rest, and
    // naming the part to keep is what went wrong: the filter excluded
    // `work-package` while the emitter also produced two kinds of `thread`
    // section, so every mutation left one more copy of a thread's prose
    // behind, and issue prose was never derived here at all. `sections_of` has
    // no set to name.
    let sections = ingest::sections_of(&self.project, &next.threads, &next.issues, &next.boards);

    // THE DATABASE IS THE MUTATION (D01, reversed by hv 2026-08-15: the DB is
    // the SSOT and the files are re-creatable).
    //
    // This used to write FILES first and roll them back if the DB write
    // failed, which was right while canon was durable and the DB was a
    // rebuildable index. Under the reversed model it is backwards in a way
    // that matters: the file write is the one that can be redone from the
    // other side, so committing it first put the recoverable half in front of
    // the unrecoverable one.
    //
    // One transaction covers the entities, the prose index and the envelope
    // together -- see [`Mutation`]. If it fails, nothing has been written
    // anywhere and truth is untouched.
    let changed_threads: Vec<&Thread> = next
      .threads
      .iter()
      .filter(|t| changed_thread_ids.contains(&t.id))
      .collect();
    let changed_issues: Vec<&Issue> = next
      .issues
      .iter()
      .filter(|i| changed_issue_numbers.contains(&i.number))
      .collect();

    // **WHICH OF THOSE ARE CREATES** (issue 0131, hv ruled 2026-08-28: a verb
    // named `add`/`new` must FAIL on an existing key rather than replace it).
    //
    // Diffed here rather than declared by the verb, for the reason stated at
    // the top of this function about the write set: a declaration can name the
    // wrong id and comparing cannot forget. It also means every create verb is
    // covered at once, present and future, rather than the ones that remembered
    // to opt in -- which is how `issues add` and `at new` came to have the same
    // defect and one fix between them.
    //
    // **A STALE `self.canon` RESOLVES THE RIGHT WAY, AND THAT IS THE WHOLE
    // FIX.** If the number was taken since this facade loaded, the diff still
    // says create, the UNIQUE constraint still fires inside the transaction,
    // and the caller is REFUSED. Before this, `next_issue_number()` read a
    // stale max, the landing was `ON CONFLICT DO UPDATE SET ... body =
    // excluded.body`, and the second writer was told `created:` while the first
    // writer s issue was overwritten -- measured 2026-08-28, two nodes, one
    // surviving filing, nothing reporting the loss.
    let created_threads: Vec<String> = changed_threads
      .iter()
      .filter(|t| !self.canon.threads.iter().any(|c| c.id == t.id))
      .map(|t| t.id.clone())
      .collect();
    let created_issues: Vec<u32> = changed_issues
      .iter()
      .filter(|i| !self.canon.issues.iter().any(|c| c.number == i.number))
      .map(|i| i.number)
      .collect();

    // **AND WHAT EACH CHANGE WAS DERIVED FROM** (issue 0206, vc ruled
    // 2026-09-01). The store compares these against what it holds, inside the
    // mutation's own transaction, and refuses a write whose record has moved.
    //
    // **TAKEN FROM `self.canon`, WHICH IS THE DEFINITION OF "DERIVED FROM".**
    // Every canon verb clones this snapshot, edits a field and writes the whole
    // record back, so `self.canon` is exactly the copy the write assumes is
    // still true -- and the copy that is stale when somebody else has written
    // since this facade opened.
    //
    // **FILTERING `self.canon` IS ALSO WHAT EXCLUDES THE CREATES, WITHOUT
    // SAYING SO TWICE.** A created id is by construction one `self.canon` does
    // not hold -- that is how `created_threads` above is computed -- so it
    // cannot appear here. A second `!created_threads.contains(...)` clause
    // would read as a safety net and would in fact be a second definition of
    // "new", free to disagree with the first.
    let expected_threads: Vec<&Thread> = self
      .canon
      .threads
      .iter()
      .filter(|c| changed_thread_ids.contains(&c.id))
      .collect();
    let expected_issues: Vec<&crate::model::Issue> = self
      .canon
      .issues
      .iter()
      .filter(|c| changed_issue_numbers.contains(&c.number))
      .collect();

    let dates = self
      .store
      .commit_mutation(crate::store::Mutation {
        threads: &changed_threads,
        issues: &changed_issues,
        removed_threads: &removed_threads,
        removed_issues: &removed_issues,
        created_threads: &created_threads,
        created_issues: &created_issues,
        expected_threads: &expected_threads,
        expected_issues: &expected_issues,
        sections: &sections,
        envelopes: &envelopes.iter().collect::<Vec<_>>(),
        project_state,
      })
      // **The refusal is LIFTED rather than wrapped.** `FacadeError::Store`
      // renders "could not update the runtime store" and drops its source, so
      // travelling as one would turn "issue 0126 already exists" into a
      // sentence the operator cannot act on -- the same class as the raw
      // `UNIQUE constraint failed` the store already translates away.
      .map_err(|e| match e {
        crate::store::StoreError::CreateHitAnExistingKey { kind, key } => match kind {
          crate::store::EntityKind::Thread => FacadeError::ThreadExists { id: key },
          crate::store::EntityKind::Issue => FacadeError::IssueExists {
            number: key.parse().unwrap_or_default(),
          },
        },
        // **ONE VARIANT FOR BOTH ENTITIES, WHICH IS A DEPARTURE FROM THE
        // SIBLINGS ABOVE AND IS REASONED RATHER THAN LAZY.** `ThreadExists` and
        // `IssueExists` are split because the operator's NEXT MOVE differs --
        // one sends them to `intent st new`, the other to `intent issues add`.
        // A record that moved has one next move whichever kind it is: re-run,
        // and the command reads the current record. Splitting on a distinction
        // that changes nothing is how a vocabulary grows without meaning more.
        crate::store::StoreError::RecordMovedUnderTheWrite { kind, key } => {
          FacadeError::RecordMovedUnderTheWrite {
            subject: format!("{kind} {key}"),
          }
        }
        other => FacadeError::Store(other),
      })?;
    drop(changed_threads);
    drop(changed_issues);

    // **THE DATES COME BACK FROM THE WRITE, AND THE FILES ARE RENDERED FROM
    // WHAT LANDED** (D42). `st new` handed in an empty `created`; SQLite filled
    // it as part of the INSERT. Rendering `thread.json` before this point would
    // write the empty string into the extract -- truth and its projection
    // disagreeing on the one field neither of them can recompute.
    for stamped in dates.threads {
      let thread = find_thread_mut(&mut next, &stamped.id)?;
      thread.created = stamped.created;
      thread.completed = stamped.completed;
    }
    // **The issues arm is the same seam and it was missing until Machine 4
    // needed it.** `issues.created` is a domain date the DDL names alongside
    // `threads.created`, `issues add` hands it in empty, and before this loop
    // existed there was no channel to bring the stamp back -- so the extract
    // would have carried `""` for the one field a rebuild cannot recompute.
    for stamped in dates.issues {
      let issue = next
        .issues
        .iter_mut()
        .find(|i| i.number == stamped.number)
        .ok_or(FacadeError::NoSuchIssue {
          number: stamped.number,
        })?;
      issue.created = stamped.created;
      issue.closed = stamped.closed;
    }

    // **THE THIRD SEAM, AND IT IS NOT AN ENTITY DATE** -- `FiatRecord.at` is
    // nested inside a criterion's state, so no column fills it and the two
    // loops above have no shape that fits. The value still comes back from the
    // write: `write_event` returns the stamp the DB assigned, which
    // `commit_mutation` now carries in `StoredDates::event_ts`.
    //
    // **EMPTY IS THE SELECTOR, and that is what makes this safe to run over
    // every criterion on every mutation.** `ac_fc` is the only door that
    // constructs a `FiatRecord` with an empty `at`; a fiat close already
    // carrying a stamp is either a row this mutation did not touch or one
    // ingested from canon, and re-stamping either would rewrite when it
    // happened to the moment of an unrelated write. That is the same
    // distinction `Stamp::CarriedFromTheExtract` exists to draw one layer down.
    //
    // Rendering the extract before this point would write `""` into the one
    // field neither truth nor its projection can recompute -- the exact failure
    // the threads and issues loops were built to prevent.
    //
    // **AND THE AT's RECORD SITS BESIDE ITS STATUS RATHER THAN INSIDE IT**, so
    // it is a second walk rather than a wider pattern. `AtStatus` derives `Copy`
    // and async-graphql `Enum` and cannot carry a payload; the selector is the
    // same empty `at`, for the same reason, with the same safety over rows this
    // mutation never touched.
    //
    // **THIS LOOP FILLS THE RENDER; `Store::stamp_fiat` FILLS THE WRITE, AND
    // BEFORE 0159 ONLY THIS ONE EXISTED.** That is why the extract carried the
    // time and durable truth carried a blank: the patch ran after the rows were
    // already serialised, so the store kept the empty string and the next `sync
    // --to-disk` put it back over the good value. The two now agree because the
    // stamp exists before either consumes it, not because one copies the other.
    //
    // **AND IT WALKS ALL FOUR KINDS NOW.** Criteria and tests were the only two
    // reached, so a thread or package closed on authority rendered with no time
    // at all -- the two kinds a cascade STARTS from.
    for thread in &mut next.threads {
      if let Some(record) = thread.fiat.as_mut()
        && record.at.is_empty()
      {
        record.at = dates.event_ts.clone();
      }
      for wp in &mut thread.wps {
        if let Some(record) = wp.fiat.as_mut()
          && record.at.is_empty()
        {
          record.at = dates.event_ts.clone();
        }
      }
      for criterion in &mut thread.criteria {
        if let AcState::Fiat(record) = &mut criterion.state
          && record.at.is_empty()
        {
          record.at = dates.event_ts.clone();
        }
      }
      for test in &mut thread.tests {
        if let Some(record) = test.fiat.as_mut()
          && record.at.is_empty()
        {
          record.at = dates.event_ts.clone();
        }
      }
    }

    // The SAME projection both sync directions use, so a mutation and a
    // resync cannot render the tree differently.
    let changed_threads: Vec<&Thread> = next
      .threads
      .iter()
      .filter(|t| changed_thread_ids.contains(&t.id))
      .collect();
    let changed_issues: Vec<&Issue> = next
      .issues
      .iter()
      .filter(|i| changed_issue_numbers.contains(&i.number))
      .collect();
    let projected = self.projection(
      &next,
      &changed_threads,
      &changed_issues,
      None,
      Some(&self.canon),
    );
    drop(changed_threads);
    drop(changed_issues);
    // Issue 0376: every step below returned its failure as the write's own, and a failed render left the canon behind the store.
    let Projection {
      mut set,
      canon_files,
    } = match projected {
      Ok(projection) => projection,
      Err(cause) => {
        self.canon = next;
        return Ok(Applied {
          foreign: Vec::new(),
          after_write: Some(Note::after_write(
            "rendering the views",
            &cause,
            RERENDER_REMEDY,
          )),
        });
      }
    };
    // The act's events travel in the act's own write set (ST0078 P1).
    if let Err(cause) = self.add_event_files(&mut set) {
      self.canon = next;
      return Ok(Applied {
        foreign: Vec::new(),
        after_write: Some(Note::after_write(
          "rendering the event files",
          &cause,
          RERENDER_REMEDY,
        )),
      });
    }

    // Truth has landed. The files are a projection of it, so a failure here is
    // REPORTED AND RECOVERABLE rather than corrupting: `intent sync` writes
    // them again from the DB. Reporting it is not optional -- silently
    // returning success would leave the tree disagreeing with truth and say
    // nothing, which is the No Silent Errors case this whole apparatus exists
    // to refuse.
    // In-memory canon follows the STORE, not the files, so the next call in
    // this process builds on what actually happened either way.
    // **ASKED BEFORE THE WRITE, WHICH IS THE HALF THE OBVIOUS ORDERING GETS
    // WRONG** -- the same argument `closing_notes` makes, and AC-03.9's before
    // `that`: once the bytes are gone, the prior contents cannot be compared
    // against anything. A list assembled afterwards could say WHICH paths were
    // written and never whose work was on them.
    let foreign = match self.foreign_bytes(&set) {
      Ok(foreign) => foreign,
      Err(cause) => {
        self.canon = next;
        return Ok(Applied {
          foreign: Vec::new(),
          after_write: Some(Note::after_write(
            "rendering the views",
            &cause,
            RERENDER_REMEDY,
          )),
        });
      }
    };
    let projected = set.commit();
    self.canon = next;
    let applied = match projected {
      Ok(applied) => applied,
      // A torn rollback stays the verb's error (vc, ruled 2026-09-14): the
      // files are neither the old render nor the new, so a warning would
      // report a state that is not the one on disk.
      Err(cause @ WriteError::TornRollback { .. }) => {
        return Err(FacadeError::ViewsNotWritten { cause });
      }
      Err(cause) => {
        return Ok(Applied {
          foreign,
          after_write: Some(Note::after_write(
            "writing the views",
            &FacadeError::ViewsNotWritten { cause },
            RERENDER_REMEDY,
          )),
        });
      }
    };
    // **READ BEFORE `keep`, BECAUSE `keep` CONSUMES IT.** The mutation path is
    // the one a fixture and a user both take, so a view written here that the
    // index never learned about is the same feedback loop by the commonest door.
    let landed: Vec<std::path::PathBuf> = applied.written().map(std::path::PathBuf::from).collect();
    applied.keep();
    let after_write = self
      .record_landed(&canon_files, &landed)
      .err()
      .map(|cause| {
        Note::after_write(
          "recording the written views in the file index",
          &cause,
          UNINDEXED_REMEDY,
        )
      });
    Ok(Applied {
      foreign,
      after_write,
    })
  }

  /// The write set's paths that hold bytes the store did not render.
  ///
  /// **THE PREDICATE IS SKEW AND NOT CHANGE.** Every mutation re-renders the
  /// subject's own views, and those files differing from the NEW render is the
  /// projection working. What matters is a path whose bytes differ from what
  /// the store would have rendered a MOMENT AGO, because the store is the only
  /// writer these files have: anything else there was typed by somebody, or
  /// left by a tool nobody registered, and this write is about to end it.
  ///
  /// **THE CANDIDATES ARE FOUND BEFORE THE RENDER, so the ordinary mutation
  /// pays nothing.** A path that does not exist loses nothing, and a path whose
  /// bytes already match what is about to be written is not written at all
  /// (`WriteSet::commit` skips it). Only when something survives both is the
  /// prior estate rendered to ask what it SHOULD have held.
  ///
  /// **WHAT IT CANNOT SPEAK FOR, it says nothing about.** The prior render and
  /// the prior canon cover views and canon files; a write to a path neither can
  /// produce -- an attachment's opaque sidecar, a blob -- has no store-side
  /// "should" to compare against, so it is left out rather than reported on a
  /// comparison that was never made.
  fn foreign_bytes(&self, set: &WriteSet) -> Result<Vec<String>, FacadeError> {
    let candidates: Vec<(std::path::PathBuf, Vec<u8>)> = set
      .writes()
      .filter_map(|(path, content)| {
        let disk = std::fs::read(path).ok()?;
        (disk != content).then(|| (path.to_path_buf(), disk))
      })
      .collect();
    if candidates.is_empty() {
      return Ok(Vec::new());
    }
    let ctx = self.render_ctx()?;
    let mut before: std::collections::BTreeMap<std::path::PathBuf, Vec<u8>> =
      views::render_all(&self.project, &self.canon, &ctx)
        .into_iter()
        .map(|view| (view.path, view.content.into_bytes()))
        .collect();
    for thread in &self.canon.threads {
      if let Ok(json) = to_canonical_json(thread) {
        before.insert(self.project.thread_json(&thread.id), json.into_bytes());
      }
    }
    for issue in &self.canon.issues {
      if let Ok(json) = to_canonical_json(issue) {
        before.insert(self.project.issue_json(issue.number), json.into_bytes());
      }
    }
    Ok(
      candidates
        .into_iter()
        // **A VIEW AN OLDER INTENT RENDERED IS NOT A HAND EDIT** (issue 0309's
        // predicate, the one doctor asks, widened by 0446). Only text the
        // renderer owns differs, so nobody's work is under it. Issue 0385: the
        // first write after an upgrade warned for every such view.
        .filter(|(path, disk)| {
          before.get(path).is_some_and(|prior| {
            prior != disk
              && !views::differs_only_in_renderer_owned_text(
                &String::from_utf8_lossy(disk),
                &String::from_utf8_lossy(prior),
              )
          })
        })
        .map(|(path, _)| self.project.relative(&path))
        .collect(),
    )
  }

  /// What a CLOSING transition has to say before it happens (AC-05.2).
  ///
  /// **COMPUTED BEFORE THE WRITE, WHICH IS THE HALF THE OBVIOUS ORDERING GETS
  /// WRONG.** `sync --to-store` states what it will overwrite rather than what
  /// it overwrote, on AC-03.9's ground that a summary afterwards is a receipt
  /// for a loss the operator needed one moment earlier. The same argument
  /// reaches here one step removed: the close itself destroys nothing, but it
  /// puts the artefact's files in line for the next `organize`, and the moment
  /// to say so is while the operator is still holding the decision.
  ///
  /// **TIED TO THE REMOVAL AND NOT TO THE VERB.** `st done --keep` closes the
  /// thread and leaves it listed, so no dehydration is coming and there is
  /// nothing to warn about. Keying on the verb would have warned anyway --
  /// correct-looking, and a warning about a consequence that is not coming is
  /// how an operator learns to skim the ones that are.
  ///
  /// **AN ATTACHMENT-LESS THREAD ASKS NOTHING AND SO CANNOT BE UNCERTAIN.**
  /// The uncertainty this reports is git's, and with no paths to ask about
  /// there is no question for git to fail to answer -- zero attachments hold
  /// zero uncommitted bytes by arithmetic, in a repository or out of one. That
  /// is not a clean bill of health taken on credit; it is the one case where
  /// the answer does not depend on the check.
  fn closing_notes(&self, op: &str, id: &str, list: ListEdit) -> Result<Vec<Note>, FacadeError> {
    if list == ListEdit::Suppressed
      || !matches!(declared_list_edit(op), Some((_, ListAction::Remove)))
    {
      return Ok(Vec::new());
    }
    // **EVERY PATH THE REMOVAL WOULD TAKE, COMMITTED ONES INCLUDED.** The
    // uncommitted-attachment warning below is the sharpest case and a strict
    // subset; a committed view is still a file that leaves the operator's
    // working tree, and it left without being named.
    let mut notes = match self.dehydration_paths(id)? {
      paths if paths.is_empty() => Vec::new(),
      paths => vec![Note::DehydratesOnNextOrganize(paths)],
    };
    // **THE ATTACHMENT QUESTION IS ASKED OF A THREAD THAT HAS ATTACHMENTS**, and
    // an attachment-less thread cannot be uncertain about them -- see this
    // function's own doc. It is asked SECOND now, because the removal list above
    // is about files on disk and does not depend on the store carrying any.
    if self.st_show(id)?.attachments.is_empty() {
      return Ok(notes);
    }
    let scope = SyncScope::Threads(vec![id.to_string()]);
    match self.sync_uncommitted(&scope)? {
      // **NOT FOLDED INTO SILENCE.** A close that says nothing is read as "no
      // uncommitted bytes" by anyone who knows it warns, so silence here IS the
      // clean bill of health `sync_uncommitted` refuses to let a caller print.
      None => notes.push(Note::UnsyncedUnknown),
      Some(found) if found.is_empty() => {}
      Some(found) => notes.push(Note::UnsyncedAttachments(found)),
    }
    Ok(notes)
  }

  /// The paths the next `organize --apply` would remove because this close
  /// unlists `id`.
  ///
  /// **ASKED OF `organize::plan` RATHER THAN COMPUTED HERE**, with the manifest
  /// it is ABOUT to have: the thread struck from the declared set, everything
  /// else as it stands. A second opinion about what dehydration removes is a
  /// divergent copy of the rule that does the removing, and the two would
  /// disagree exactly when it mattered -- which is the objection `projection`
  /// already records against a second renderer.
  ///
  /// **A MANIFEST THAT IS NOT `Declared` ARMS NOTHING.** Absent or unparseable
  /// realises everything and `edit_list` writes no entry, so no removal is
  /// coming and there is nothing to warn about -- the same fail-open direction
  /// `Realised::declares` takes, read from the same place rather than assumed.
  /// A thread the manifest does not list is likewise already unlisted, so this
  /// close arms nothing new.
  ///
  /// **IT READS THE MANIFEST THROUGH [`Facade::realised_threads`] AND NOT
  /// THROUGH `manifest_for_action`, AND THE DIFFERENCE IS WHICH ERROR THE
  /// OPERATOR SEES.** The strict reader REFUSES an unparseable manifest, and a
  /// note that refuses is a diagnostic deciding the verb's outcome: measured
  /// here as two error-remedy arms going red, because a close carrying a bad
  /// date on an estate with a bad manifest started reporting the manifest --
  /// the wrong one of its two faults, chosen by which diagnostic ran first. A
  /// note may say less than it would like; it may not change what the verb
  /// answers.
  fn dehydration_paths(&self, id: &str) -> Result<Vec<String>, FacadeError> {
    let Realised::Declared(mut declared) = self.realised_threads() else {
      return Ok(Vec::new());
    };
    if !declared.remove(&intentfiles::declared_key(Sigil::SteelThread, id)) {
      return Ok(Vec::new());
    }
    let previous = self.store.file_index().map_err(FacadeError::Store)?;
    let (tree, digest) =
      organize::observe(&self.project, &previous).map_err(FacadeError::Organize)?;
    let ctx = self.render_ctx()?;
    let plan = organize::plan(
      &self.project,
      &self.canon,
      &Realised::Declared(declared),
      &ctx,
      &tree,
      digest,
    );
    Ok(
      plan
        .steps
        .into_iter()
        .filter(|step| step.action.is_destructive())
        .filter(|step| self.owning_thread(&step.path, &self.canon).as_deref() == Some(id))
        .map(|step| self.project.relative(&step.path))
        .collect(),
    )
  }

  /// Make a lifecycle op's declared edit to `.intentfiles` (AC-05.2).
  ///
  /// **A REMOVAL RUNS AFTER `apply`, AND THE ORDER IS CHOSEN FOR ITS FAILURE MODE.**
  /// Both orders can be interrupted between the two writes, so the question is
  /// only which half-done state is survivable. Manifest first, store second,
  /// leaves the list saying NOT REALISED while the thread is still open -- and
  /// the next `organize` removes a live thread's files on the strength of it.
  /// Store first leaves a closed thread still listed, which is precisely what
  /// `--keep` asks for on purpose. **One order degrades into a supported
  /// outcome and the other into a deletion nobody asked for.**
  ///
  /// **THAT ARGUMENT IS ABOUT A REMOVAL, AND AN ADDITION RUNS BEFORE `apply`**
  /// (issue 0079). Manifest first for an addition leaves a thread listed whose
  /// status did not move -- the state `st hydrate` leaves on purpose, and one
  /// that deletes nothing -- and the caller restores the list when `apply`
  /// refuses. Store first left the thread listed with no files, because the
  /// projection inside `apply` had already read the manifest.
  ///
  /// **THIS IS THE OPPOSITE ORDER FROM [`Facade::hydrate`], WHICH PINS FIRST,
  /// AND THE TWO ARE NOT IN TENSION.** Hydrate's ordering answers a different
  /// question -- that the pin must not be skipped when the files already exist
  /// -- and it has no second write to be interrupted between.
  fn edit_list(&self, op: &str, id: &str, list: ListEdit) -> Result<(), FacadeError> {
    if list == ListEdit::Suppressed {
      return Ok(());
    }
    let Some((sigil, action)) = declared_list_edit(op) else {
      return Ok(());
    };
    let path = self.project.intentfiles_path();
    let before = match std::fs::read_to_string(&path) {
      Ok(text) => text,
      // **ABSENT IS NOT EMPTY, SO AN ABSENT MANIFEST IS LEFT ABSENT** -- and
      // this is the arm where that rule earns its keep. A missing file means
      // nobody has said, and everything is realised. Creating one here to hold
      // this single entry would declare that this id is **the whole of what is
      // realised**, and the next `organize` would remove every other thread's
      // files on the strength of one `st new`. The no-op is the rule applying,
      // not a case being skipped.
      Err(source) if source.kind() == std::io::ErrorKind::NotFound => return Ok(()),
      // **AND ANY OTHER IO ERROR IS RAISED RATHER THAN FOLDED INTO THE ABOVE.**
      // A manifest that exists and cannot be read is not a manifest that does
      // not exist: treating them alike would let a permissions fault silently
      // stop `st done` maintaining the list, which is the No Silent Errors case
      // exactly -- an unreadable file answering as "nobody has said".
      Err(source) => {
        return Err(FacadeError::ManifestUnreadable {
          path: path.display().to_string(),
          source,
        });
      }
    };
    let after = match action {
      ListAction::Add => intentfiles::pin(&before, sigil, id, None),
      ListAction::Remove => intentfiles::unpin(&before, sigil, id),
    }
    .map_err(FacadeError::Intentfiles)?;
    // Both primitives are idempotent, so an unchanged file is the ordinary
    // outcome of closing an already-unlisted thread rather than a miss.
    if after != before {
      let mut set = WriteSet::new();
      set.add(path, after);
      set.commit()?.keep();
    }
    Ok(())
  }

  /// Put `.intentfiles` back as it was, when a write that edited it first did
  /// not land.
  ///
  /// **A REFUSAL SAYS THE CHANGE WAS NOT MADE, SO NOTHING OF IT MAY STAY.** A
  /// manifest edit made ahead of the store write, because the projection inside
  /// that write reads it, is the one piece of such a write already on disk when
  /// the store refuses. `None` is a manifest that was not there to edit.
  fn restore_manifest(&self, before: Option<String>) -> Result<(), FacadeError> {
    let manifest = self.project.intentfiles_path();
    if let Some(text) = before
      && std::fs::read_to_string(&manifest).ok().as_deref() != Some(text.as_str())
    {
      let mut set = WriteSet::new();
      set.add(manifest, text);
      set.commit()?.keep();
    }
    Ok(())
  }

  fn next_thread_id(&self) -> String {
    let highest = self
      .canon
      .threads
      .iter()
      .filter_map(|t| crate::model::thread_seq(&t.id))
      .max()
      .unwrap_or(0);
    crate::model::thread_id(highest + 1)
  }
}

/// The child collections that are never written through their parent's
/// address, paired with the address SEGMENT that reaches each one.
///
/// **ONE TABLE, TWO READERS, AND THAT IS THE WHOLE POINT.** [`Facade::put`]
/// refuses a thread body carrying any of these; [`Facade::set`] refuses the
/// same names as fields. Two copies would agree exactly until a fifth child was
/// added to one of them, and the reader who noticed would be an operator
/// holding a remedy that does not parse.
///
/// **THE FIELD NAME IS NOT THE ADDRESS SEGMENT.** The model calls them
/// `wps`/`criteria`/`tests`; the grammar spells them `wp`/`ac`/`at`
/// (`address.rs:445-467`). Interpolating one as the other prints a remedy that
/// sends an operator to a parse error from the tool that just told them to go
/// there.
const CHILD_COLLECTIONS: [(&str, &str); 4] = [
  ("wps", "wp"),
  ("criteria", "ac"),
  ("tests", "at"),
  ("attachments", "attachments"),
];

/// Every property name a model declares, ABSENT OPTIONALS INCLUDED.
///
/// **DERIVED FROM THE TYPE AND NEVER FROM AN INSTANCE, AND THE BURNING CASE IS
/// EXACTLY WHY.** `Thread::completed` is `skip_serializing_if =
/// "Option::is_none"`, so on ST0011 -- the estate's one genuinely wrong row,
/// and the row this criterion was written for -- it serialises to nothing at
/// all. A field set read off an instance would answer *not a field of this
/// entity* for `completed` on precisely the row that needs it, and the refusal
/// would look perfectly correct on its way past.
/// `a` or `an`, so a refusal cannot say *a issue*.
///
/// Issue 0081: seven of `Entity::form()`'s fourteen values are vowel-initial and
/// two shipped messages glued a bare `a` in front of them.
fn article_for(form: &str) -> &'static str {
  match form.chars().next() {
    Some('a' | 'e' | 'i' | 'o' | 'u') => "an",
    _ => "a",
  }
}

fn schema_properties<T: schemars::JsonSchema>() -> std::collections::BTreeSet<String> {
  #[allow(
    clippy::expect_used,
    reason = "INVARIANT: a schemars schema serialises to JSON by construction"
  )]
  let schema = serde_json::to_value(schemars::schema_for!(T))
    .expect("a schemars schema serialises to JSON by construction");
  schema
    .get("properties")
    .and_then(Value::as_object)
    .map(|properties| properties.keys().cloned().collect())
    .unwrap_or_default()
}

/// Why an address form contributes NO fields to AC-08.5's surface.
///
/// **A FORM WITH NO FIELDS IS NOT A GAP, BUT IT OWES A REASON.** The criterion
/// asks for an unwritable field to be reported BY NAME; a form with no fields at
/// all has nothing to name, so what it owes instead is why -- and the reason has
/// to be TRUE of the form it is given for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fieldless {
  /// Membership, not fields. Writing one would mean applying or dropping the
  /// whole collection, and both are silent about the other.
  Collection,
  /// Rows are appended whole (D53), so there is no field to set.
  AppendOnly,
  /// The address form was minted in the grammar ahead of its model, so there is
  /// no type to derive fields FROM. **Not a decision that it should have none.**
  NoModelYet(&'static str),
}

impl Fieldless {
  /// The reason, TRUE of the form it is given for.
  ///
  /// **THIS REPLACES A SENTENCE THAT WAS FALSE FOR TWO OF THE FORMS IT
  /// EXPLAINED.** `settable_fields` refused everything outside its four arms
  /// with *an attachment's body is its content, and the rest are collections or
  /// append-only logs* -- and `Issue` is neither a collection nor a log, nor is
  /// `Node`. So an operator refused on `intent:///issues/0081` was told they had
  /// addressed a collection. **A wrong reason is worse than no reason: it reads
  /// as a considered explanation and sends the reader somewhere real.** Derived
  /// per form now, so it cannot describe a set it was not given.
  pub fn why(&self) -> String {
    match self {
      Self::Collection => "a collection has membership rather than fields -- address a member \
         and set the field there"
        .to_string(),
      Self::AppendOnly => "this is an append-only log (D53): rows are added whole and never \
         edited, so there is no field to set"
        .to_string(),
      // **THE OWNER IS A CODE COMMENT, NOT A SHIPPED STRING.** The first draft
      // interpolated `ST0056/WP-14` into the operator's refusal, and
      // `no_pm_state_in_output` refused it: a consumer reading this learns
      // nothing they can act on from OUR work-package id, and Intent's own
      // project-management state has no business in Intent's output. Same rule
      // that moved a worked example off a real thread id an hour ago.
      Self::NoModelYet(what) => format!(
        "{what} has no model type yet, so there are no fields to set here -- the address form \
         exists in the grammar ahead of the type behind it, and until that lands there is \
         nothing to derive a field list from"
      ),
    }
  }
}

/// **THE mapping from an address form to the model behind it, and the ONLY one.**
///
/// It was written twice -- here and inside `settable_fields` -- which is the
/// two-homes defect sitting inside the criterion about surface completeness.
///
/// # The match is exhaustive ON PURPOSE, and that is the announcement mechanism
///
/// vc's ruling (2026-08-25): **the population is DERIVED from the model, never
/// enumerated in prose**, so a type arriving later joins without anybody
/// remembering. A `_` arm here would swallow a fourteenth `Entity` variant into
/// whichever bucket it happened to fall in; naming every variant means adding
/// one **fails to compile** until someone says which side it is on. A guard whose
/// scope silently excludes the case it will later need is the class this
/// criterion keeps producing, and the fix for it must not commit it.
/// Fields the model DECLARES that serialisation cannot see.
///
/// **A `#[serde(skip)]` FIELD IS STILL A FIELD, AND AC-08.5 COUNTS DECLARED
/// FIELDS RATHER THAN SERIALISABLE ONES.** `Attachment::blob` is skipped because
/// the bytes live in a sidecar, so `schema_properties::<Attachment>()` returns
/// FOUR of its five names -- and `blob` is this row's own bytes-carried burning
/// case. **An instrument derived from serialisation cannot see the case the
/// criterion exists for**, which is the address-axis mistake one layer down: a
/// true measurement of a narrower thing than the row asks about.
///
/// Declared here rather than derived because schemars honours the same serde
/// attribute, so there is nothing to derive FROM. The pinned totals in
/// `ac_08_5_field_axis` are what stop a second skip slipping in silently: add
/// one and the schema set shrinks while the declared total does not, and the
/// arithmetic reds.
///
/// Found by ic, against their own interest -- it moves the denominator they are
/// measured on.
const SERDE_SKIPPED: [(&str, &[&str]); 1] = [("attachment", &["blob"])];

/// The skipped names for an address form, if any.
fn skipped_fields(entity: &AddrEntity) -> &'static [&'static str] {
  SERDE_SKIPPED
    .iter()
    .find(|(form, _)| *form == entity.form())
    .map_or(&[], |(_, fields)| *fields)
}

fn fields_of(entity: &AddrEntity) -> Result<std::collections::BTreeSet<String>, Fieldless> {
  let declared = |mut set: std::collections::BTreeSet<String>| {
    set.extend(skipped_fields(entity).iter().map(|f| (*f).to_string()));
    Ok(set)
  };
  match entity {
    AddrEntity::Thread { .. } => declared(schema_properties::<Thread>()),
    AddrEntity::Wp { .. } => declared(schema_properties::<WorkPackage>()),
    AddrEntity::Ac { .. } => declared(schema_properties::<Criterion>()),
    AddrEntity::At { .. } => declared(schema_properties::<AcceptanceTest>()),
    AddrEntity::Attachment { .. } => declared(schema_properties::<crate::model::Attachment>()),
    AddrEntity::Issue { .. } => declared(schema_properties::<crate::model::Issue>()),
    AddrEntity::Threads
    | AddrEntity::Issues
    | AddrEntity::WpCollection { .. }
    | AddrEntity::AcCollection { .. } => Err(Fieldless::Collection),
    AddrEntity::Event { .. } | AddrEntity::NodeInbox { .. } => Err(Fieldless::AppendOnly),
    // **HELD WITH hv, NOT SETTLED HERE.** Whether the model should carry a
    // `Node` type at all is ST0056/WP-14's scope. What is measurable today, and
    // needs nobody's ruling, is that `schema_properties::<Node>()` cannot be
    // written because no such type exists in any crate.
    AddrEntity::Node { .. } => Err(Fieldless::NoModelYet("a whiteboard node")),
  }
}

/// [`schema_properties`] for whichever model an address names.
fn schema_properties_of(entity: &AddrEntity) -> std::collections::BTreeSet<String> {
  fields_of(entity).unwrap_or_default()
}

/// Why a field is not settable through the narrow setter.
///
/// **EVERY VARIANT CARRIES THE DOOR THAT IS OPEN.** AC-08.5 asks for an
/// unwritable field to be *reported BY NAME*, and a name with no remedy sends
/// the operator to a hand-edit of canon -- which is the route this criterion
/// exists to retire. "You cannot" is not what the criterion asked for.
enum Unsettable {
  /// The value IS the entity's address. Changing it through a write to the old
  /// address is a rename wearing an update's clothes, and D57-8 gives renames
  /// no verb.
  Identity,
  /// A ratified state machine owns the field. A raw write would land the value
  /// without the transition check, the gate, or the recorded reason -- three
  /// guarantees the lifecycle verb exists to provide, lost in silence.
  Machine(&'static str),
  /// The field has an address of its own; the segment that reaches it.
  Child(&'static str),
  /// **A LIST WITH VERBS OF ITS OWN**, which add and drop one member at a time
  /// and check each one (issue 0460). Not `Child`: the members have no address,
  /// so there is no URL to name. Not `Machine`: no state machine owns the list.
  /// Carries the verbs.
  OwnVerbs(&'static str),
  /// **COMPUTED FROM ANOTHER FIELD, so writing it independently is how a record
  /// comes to describe something it does not.** An attachment's `sha256` set by
  /// hand would correctly describe the wrong bytes -- the exact hazard `put`
  /// already refuses `?format=json` for, one field at a time instead of all at
  /// once. Carries the field it follows.
  Derived(&'static str),
  /// **THE SERVICE LAYER WRITES IT ONCE, AT THE EVENT, AND NO VERB MOVES IT
  /// AFTERWARDS -- WHICH IS WHY THE REMEDY IS NOT A VERB.**
  ///
  /// D42 as amended (dc, 2026-08-25): **no caller authors a stamp.** The rule
  /// used to state itself as a test on SIGNATURES -- *no cli or intentsvcs
  /// function takes a time* -- and `Facade::set(&addr, field, Value)` genuinely
  /// takes no time, **so the letter of the rule was satisfied while the property
  /// it exists to protect was not.** A rule stated as a mechanism, evaded by a
  /// different mechanism; the signature test is one sufficient condition and
  /// never the definition.
  ///
  /// **IT IS NOT TIDINESS.** ST0057 AC-14.11 forbids re-stamping *in either
  /// direction* because with the DB as truth and sync running both ways, a
  /// resync that re-inserts rows lets a default re-stamp them -- the
  /// fabricated-stamp failure reintroduced by its own fix. A settable `created`
  /// gives you that by hand instead of by accident, **and the two are
  /// indistinguishable afterwards.**
  ///
  /// **`completed` IS NOT THIS AND MUST STAY SETTABLE.** It is a domain DATE the
  /// operator asserts, carried from v2 and never re-stamped, and it is this
  /// criterion's FIRST burning case -- NULL on ST0011, the estate's one
  /// genuinely wrong row. A stamp is written by the service from the clock; a
  /// date is authored. **Refusing both because they look alike would close the
  /// gap the row was opened for.**
  ///
  /// **AND THIS SAYS NOTHING ABOUT INGEST OR MIGRATION.** A migrator carrying a
  /// v2 thread's real creation date is a caller CARRYING a stamp, not authoring
  /// one. dc owns D42 and declined that boundary; it is hv's if it is anyone's,
  /// and nothing here closes it in either direction.
  Stamped,
  /// **THIS ADDRESS TAKES THE DOCUMENT ITSELF, NOT ITS FIELDS.** An attachment's
  /// content is its body: `put` at the attachment address writes the file, and
  /// the record's other fields are computed from what lands.
  ///
  /// **THE ROUTE IS REAL AT THE SERVICE LAYER AND ABSENT AT THE CLI, AND THE
  /// EXPLANATION SAYS SO RATHER THAN IMPLYING A VERB.** Driven: `intent put`
  /// answers *unrecognized subcommand*, and no dispatch-table row writes an
  /// attachment's content. The first draft of this remedy read *PUT the text to
  /// `{url}`*, which names an operation an operator cannot perform -- the same
  /// class as the `why` string this change exists to correct, committed inside
  /// the correction. **This is the AC/AT-creation instance's shape, not a new
  /// one: a service-layer route with no CLI surface**, so it NARROWS the gap
  /// rather than closing it, and the wording must not pretend otherwise.
  WholeBody {
    /// **WHETHER THE SERVICE-LAYER `put` CAN ALSO CARRY THIS FORM.**
    ///
    /// `Facade::put` takes a `&str`, so it carries text and CANNOT carry bytes.
    /// **The two fields shared one sentence and its fallback read *or PUT the
    /// text to `<url>`* for `blob` — the wrong noun, and worse, a route that
    /// cannot reach the outcome** (vc found it after the row was already green).
    ///
    /// **A SHARED SENTENCE IS HOW A REMEDY BECOMES WRONG FOR ONE OF ITS
    /// MEMBERS**, which is `finding.rs:267`'s defect at a smaller scale: there a
    /// remedy attached to a whole finding CLASS went on being emitted for a
    /// member that had stopped belonging. Here it was two fields in one arm.
    /// **The distinction is carried in the type so the sentence cannot be right
    /// for one member and wrong for the other.**
    put_carries: bool,
  },
  /// **A FIELD `transitions.rs` DECLARES `Immutable`: FIXED WHEN ITS ENTITY IS
  /// AUTHORED AND MOVED BY NO VERB AFTERWARDS** (issue 0334). `set` wrote
  /// `Thread.acceptance` at rc=0 while the table declared it immutable, so the
  /// declaration and the setter were two answers to one question. The refusal
  /// READS the table rather than naming the field here, so a row declared
  /// immutable later is refused with no second edit. It carries the row's note,
  /// which says what the value is; the row's `ruled` is provenance and is never
  /// printed.
  Immutable(&'static str),
  /// **A FIELD WITH NO ROUTE AT ALL, SAID PLAINLY.**
  ///
  /// This row's own doc argues that a name with no remedy sends the operator to
  /// a hand-edit, so *you cannot* is not what the criterion asked for. **It is
  /// still the only honest answer when there is genuinely nothing to name**, and
  /// the alternative is worse: three false remedies were authored in one day,
  /// each inside the fix for the previous, and a fourth was drafted for THIS
  /// field claiming `sync --to-store` picks up a dropped file. It does not --
  /// `ingest` fills bytes from a sidecar canon ALREADY records.
  ///
  /// **So this variant exists to stop the pressure to invent one.** Naming the
  /// gap is a report; naming a route that does not reach the outcome is a defect
  /// the operator acts on.
  #[allow(
    dead_code,
    reason = "NotYet must stay representable so a report can print 0 rather \
                               than drop the line; blob was its only member and st attach \
                               closed it"
  )]
  NoRouteYet(&'static str),
}

/// **THE READER'S QUESTION, NOT THE REFUSAL'S REASON** (vc's ruling,
/// 2026-08-25).
///
/// ic and I proposed `by-design | unbuilt | shadowed` -- a taxonomy of WHY the
/// refusal exists. vc ruled a different axis: *can I change this, and if so
/// how*. **Our axis put the majority case with its opposite**: a reader asking
/// after `Thread.criteria` and after `Thread.id` got one answer for two
/// situations that are nothing alike -- *yes, over there* and *no, ever*.
///
/// **`shadowed` DISSOLVED RATHER THAN SURVIVING AS A FOURTH TOKEN.** A
/// capability fenced behind something is still NOT YET to whoever is trying to
/// use it; that it exists is a repair instruction, which belongs in the remedy
/// text and not in the answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnsettableKind {
  /// A route exists and the remedy names it.
  Elsewhere,
  /// No route, by design, and there never will be one.
  Never,
  /// No route, and that is a gap rather than a decision.
  NotYet,
}

impl UnsettableKind {
  /// **EVERY KIND, INCLUDING ONES WITH NO MEMBERS TODAY.**
  ///
  /// A report must be able to print `0 not-yet` rather than drop the line
  /// (ic's constraint, and the estate already ships it as `ABSENT is not
  /// EMPTY`). **A vanished category reads as a clean result and is
  /// indistinguishable from one nobody measured** -- so the kinds are
  /// enumerable rather than merely returnable per field.
  ///
  /// **THIS IS LIVE RATHER THAN HYPOTHETICAL: `NotYet` HAS NO MEMBERS AS OF
  /// `st attach` CARRYING BYTES.** `blob` was its only one.
  pub const ALL: [UnsettableKind; 3] = [Self::Elsewhere, Self::Never, Self::NotYet];

  pub fn as_str(&self) -> &'static str {
    match self {
      Self::Elsewhere => "elsewhere",
      Self::Never => "never",
      Self::NotYet => "not-yet",
    }
  }
}

/// Which of the three answers this refusal gives the reader.
///
/// **DERIVED FROM THE VARIANT AND LIVING BESIDE IT**, so it cannot drift from
/// the refusal it describes -- the same reason `fields_of` is one exhaustive
/// match. A second literal enumerating fields by kind would be the two-homes
/// defect inside the criterion about surface completeness, which is the thing
/// this file already had to have removed from it once today.
pub fn unsettable_kind(entity: &AddrEntity, field: &str) -> Option<UnsettableKind> {
  unsettable(entity, field).map(|u| u.kind())
}

impl Unsettable {
  fn kind(&self) -> UnsettableKind {
    match self {
      // A route exists and `explain` names it.
      Self::Machine(_)
      | Self::Child(_)
      | Self::OwnVerbs(_)
      | Self::Derived(_)
      | Self::WholeBody { .. } => UnsettableKind::Elsewhere,
      // Constitutive: the value IS the address, or the service owns the stamp.
      Self::Identity | Self::Stamped | Self::Immutable(_) => UnsettableKind::Never,
      // **NO CONSTRUCTOR REACHES THIS TODAY AND THE VARIANT STAYS.** `blob` was
      // its only member and `st attach` closed it. **Deleting it would make the
      // next gap unrepresentable and quietly retire the bucket** -- and a bucket
      // that cannot exist prints nothing rather than zero, which is the
      // difference this kind exists to preserve.
      Self::NoRouteYet(_) => UnsettableKind::NotYet,
    }
  }

  fn explain(&self, url: &str) -> String {
    match self {
      Self::Identity => {
        "the id is the ADDRESS of this entity rather than a field of the document at it -- \
         POST to the collection address to create a new one. There is no rename"
          .to_string()
      }
      Self::Machine(verbs) => format!(
        "a ratified state machine owns this field -- move it with `{verbs}`, which checks the \
         transition, runs the gate and records the reason. A raw write would land the value \
         and none of the three"
      ),
      Self::Child(segment) => format!(
        "this collection has an address of its own -- set each member at `{url}/{segment}/<id>`, \
         because a write here would have to either apply the whole collection or drop it, and \
         both are silent about the other"
      ),
      Self::OwnVerbs(verbs) => format!(
        "this list has verbs of its own -- `{verbs}` -- which change one link at a time, refuse \
         a link to a thread that does not exist and record each change. A write here would \
         replace the whole list with neither check"
      ),
      Self::Derived(from) => format!(
        "this field is COMPUTED from `{from}` -- write `{from}` at `{url}` and this follows. \
         Setting it by hand would leave the record correctly describing the wrong content, \
         which is exactly what nothing downstream can detect"
      ),
      Self::Stamped => "this is a machine stamp: the service layer writes it once, from the \
         clock, at the moment of the event, and nothing moves it afterwards. There is no verb \
         for this and there should not be -- a caller authoring a stamp is indistinguishable \
         later from a resync having re-stamped the row"
        .to_string(),
      Self::WholeBody { put_carries } => {
        let mut said = "an attachment's body IS its content, so it is written whole rather than \
             field by field, and the record's other fields are computed from what lands -- write \
             it with `intent st attach <thread> <path> --from <file>`"
          .to_string();
        if *put_carries {
          said.push_str(&format!(", or PUT the text to `{url}`"));
        }
        said
      }
      Self::Immutable(note) => format!(
        "{note} -- so there is no verb for it, and a write here would be the one door that moved it"
      ),
      Self::NoRouteYet(what) => format!(
        "{what}, and there is no route on this surface today. Canon must already record the \
         attachment as opaque with its sidecar beside it -- `intent st attach` writes TEXT, and \
         `ingest` fills an opaque attachment's bytes from a sidecar canon already names rather \
         than from a file you drop in"
      ),
    }
  }
}

/// The note of a field `transitions.rs` declares `Immutable`, when this one is.
///
/// **KEYED BY THE MODEL'S SCHEMA NAME, ASKED OF THE MODEL**, because that name
/// is what the table's `entity` column holds -- a literal here would be a second
/// spelling of it. The forms that carry no fields name no model.
fn immutable_note(entity: &AddrEntity, field: &str) -> Option<&'static str> {
  use schemars::JsonSchema;
  let model = match entity {
    AddrEntity::Thread { .. } => Thread::schema_name(),
    AddrEntity::Wp { .. } => WorkPackage::schema_name(),
    AddrEntity::Ac { .. } => Criterion::schema_name(),
    AddrEntity::At { .. } => AcceptanceTest::schema_name(),
    AddrEntity::Attachment { .. } => crate::model::Attachment::schema_name(),
    AddrEntity::Issue { .. } => crate::model::Issue::schema_name(),
    _ => return None,
  };
  match &transitions::find(&model, field)?.disposition {
    transitions::Disposition::Immutable { note, .. } => Some(*note),
    _ => None,
  }
}

/// Which of the three refusals, if any, covers this field.
fn unsettable(entity: &AddrEntity, field: &str) -> Option<Unsettable> {
  if let Some((_, segment)) = CHILD_COLLECTIONS.iter().find(|(name, _)| *name == field) {
    return Some(Unsettable::Child(segment));
  }
  // **AN IMMUTABLE DECLARATION IS READ, NOT RESTATED** (issue 0334): ahead of
  // the per-entity arms, so no arm below can answer for a field the table has
  // already fixed.
  if let Some(note) = immutable_note(entity, field) {
    return Some(Unsettable::Immutable(note));
  }
  // **THE VERB SPELLINGS ARE DRIVEN FROM THE SHIPPED CLI, not recalled.** A
  // remedy naming a verb that does not exist is worse than no remedy: it costs
  // the operator a round trip and reads as authoritative while it does.
  match entity {
    AddrEntity::Thread { .. } => match field {
      "schema" | "id" => Some(Unsettable::Identity),
      "status" => Some(Unsettable::Machine(
        "intent st start|done|fc|hold|resume|cancel|reopen|reinstate",
      )),
      // **`Machine` and therefore `Elsewhere`, not `Never`** -- the record HAS a
      // route and this names it, which is the distinction the kind exists to
      // make. `fc` is in the status list above for the same reason: it is one of
      // the edges that reaches `completed`, and a route list that named only the
      // gated one would send a reader to a verb that refuses them.
      "fiat" => Some(Unsettable::Machine("intent fc <ST-id> --because")),
      "related" => Some(Unsettable::OwnVerbs(
        "intent st relate <ST-id> <target> [--note <text>], intent st unrelate <ST-id> <target>",
      )),
      // `completed` is deliberately NOT here -- see [`Unsettable::Stamped`].
      "created" => Some(Unsettable::Stamped),
      _ => None,
    },
    AddrEntity::Wp { .. } => match field {
      "seq" => Some(Unsettable::Identity),
      "status" => Some(Unsettable::Machine(
        "intent wp start|done|fc|unstart|reopen|cancel|reinstate",
      )),
      "fiat" => Some(Unsettable::Machine("intent fc <ST-id>/<NN> --because")),
      _ => None,
    },
    AddrEntity::Ac { .. } => match field {
      "id" => Some(Unsettable::Identity),
      "state" => Some(Unsettable::Machine(
        "intent ac satisfy|unsatisfy|withdraw|reinstate",
      )),
      _ => None,
    },
    AddrEntity::At { .. } => match field {
      "id" => Some(Unsettable::Identity),
      // `fc` joins the route list because it genuinely moves this field now.
      // A remedy that omits a real route is incomplete in the same way one
      // that names an unreal route is wrong -- and the arms above list every
      // verb, not a representative sample.
      "status" => Some(Unsettable::Machine(
        "intent at green|red|na, or intent fc <thread> <AT-id> --because",
      )),
      // **hv's D7 (2026-08-29): THE RECORD IS REACHABLE ONLY THROUGH `fc`.**
      //
      // `put` writes the whole row, so without this the record could be
      // FABRICATED -- written with any `because`, any `by`, any `at`, and an
      // `invoker` the party being described chose -- with none of `at_fc`'s
      // guards: no transition check, no reason requirement, no service-collected
      // evidence.
      //
      // **Why this field and not the ordinary "put can write anything" case:
      // the fiat record is evidence about a PERSON, and no other field on this
      // row is.** ST0066's ruled posture is DETECTION AND ATTRIBUTION rather
      // than prevention -- on a machine where the tool and its operator share a
      // uid there is no boundary to enforce -- so a door that lets the record be
      // authored without the guard holes the only half of that posture that
      // works.
      "fiat" => Some(Unsettable::Machine("intent fc <thread> <AT-id> --because")),
      _ => None,
    },
    // **EVERY ISSUE FIELD WAS UNSETTABLE AND UNREPORTED UNTIL NOW.** `Issue`
    // fell to `settable_fields`'s `other` arm and was refused BY FORM, so the
    // ten fields below were invisible: not one of them could be written and not
    // one of them was named. `body` is the burning case -- declared in the
    // model, carried through canon, and `issues add` takes `<TITLE>` and
    // `--severity` only, so the whole of an issue's prose had no door at all.
    AddrEntity::Issue { .. } => match field {
      "schema" | "number" => Some(Unsettable::Identity),
      "status" | "closed" => Some(Unsettable::Machine("intent issues close|open")),
      "created" => Some(Unsettable::Stamped),
      // **`created` WAS SETTABLE FOR AN HOUR AND THE ARGUMENT FOR IT WAS WRONG
      // ABOUT WHICH FIELD IT WAS MAKING.** I withdrew a false remedy --
      // `Machine("intent issues add")`, a verb that creates an issue and cannot
      // move the field on one that exists -- and reached for *this row's first
      // burning case is a provenance field that is wrong*. **That case is
      // `completed`, which is an authored DATE, and I generalised it onto
      // `created`, which is a machine STAMP.** dc ruled it under D42; the two
      // are now distinguished at [`Unsettable::Stamped`], and `completed` stays
      // settable because the gap this row was opened for is exactly that field.
      // **`body` IS DELIBERATELY SETTABLE AND THE FIRST DRAFT OF THIS ARM HAD IT
      // REFUSED.** The plan was `Machine` naming a route -- and there is no
      // route: `issues` is list|add|show|close|open, `add` takes `<TITLE>` and
      // `--severity`, and nothing writes `body` at all. **A remedy naming a verb
      // that does not set the field is the `collections or append-only logs`
      // defect authored deliberately instead of arriving by drift, and it is
      // worse, because an operator ACTS on a remedy.** vc withdrew the
      // suggestion; this row's own doc settles it -- a field with no remedy is a
      // GAP, not a category.
      _ => None,
    },
    // **ALL FIVE ARE UNSETTABLE AND EACH FOR ITS OWN REASON**, which is what
    // the criterion asks for -- a refusal per field, not a refusal per form.
    // `settable_fields` therefore returns an EMPTY set here, and that is a
    // complete answer rather than a missing one.
    AddrEntity::Attachment { .. } => match field {
      "path" => Some(Unsettable::Identity),
      // **THESE TWO DIVERGED THE MOMENT `st attach` SHIPPED, AND BEFORE THAT THEY
      // SHARED A REFUSAL SAYING NEITHER HAD A ROUTE.** `text` now has one;
      // `blob` still does not, and collapsing them again would make one of the
      // two remedies false whichever way it collapsed.
      "text" => Some(Unsettable::WholeBody { put_carries: true }),
      // **`blob` MOVED FROM `NoRouteYet` TO `WholeBody` WHEN `st attach` LEARNED
      // TO CARRY BYTES, AND LEAVING IT WOULD HAVE BEEN THE SEVENTH FALSE
      // REMEDY.** It said *there is no route on this surface today* -- true when
      // written, false one commit later, and **a remedy describing a limit the
      // code no longer has is the same defect as one describing a route the code
      // never had.** The `today` in it was the honest word and it is also the
      // word that dates.
      // `put` takes a `&str` and cannot carry bytes, so the PUT clause is
      // withheld here rather than reworded.
      "blob" => Some(Unsettable::WholeBody { put_carries: false }),
      "sha256" | "bytes" => Some(Unsettable::Derived("the attachment's content")),
      _ => None,
    },
    _ => None,
  }
}

fn find_wp_mut<'a>(
  canon: &'a mut Canon,
  st: &str,
  seq: u32,
) -> Result<&'a mut WorkPackage, FacadeError> {
  find_thread_mut(canon, st)?
    .wps
    .iter_mut()
    .find(|w| w.seq == seq)
    .ok_or_else(|| FacadeError::NoSuchWorkPackage {
      st: st.to_string(),
      seq,
    })
}

fn find_issue_mut(canon: &mut Canon, number: u32) -> Result<&mut crate::model::Issue, FacadeError> {
  canon
    .issues
    .iter_mut()
    .find(|i| i.number == number)
    .ok_or(FacadeError::NoSuchIssue { number })
}

/// Everything [`Facade::land_renumber`] needs, gathered by the verb that knows
/// its kind's paths.
struct RenumberPlan {
  from: String,
  to: String,
  sigil: Sigil,
  /// Directories that move whole, each skipped when it is not there.
  moves: Vec<(std::path::PathBuf, std::path::PathBuf)>,
  /// Files named by the old id that the write does not replace.
  stale: Vec<std::path::PathBuf>,
  /// Where views that moved with a directory are re-rendered.
  rerender: Option<std::path::PathBuf>,
  envelope: Envelope,
  model: crate::renumber::Renumbered,
  foreign: Vec<String>,
}

/// Each place an answer found the id, once, in the order the index gave.
fn prose_mentions(answer: &crate::search::SearchAnswer) -> Vec<ProseMention> {
  let mut out: Vec<ProseMention> = Vec::new();
  for hit in answer.groups.iter().flat_map(|g| &g.hits) {
    let mention = ProseMention {
      path: hit.path.clone(),
      line: hit.span.as_ref().map(|s| s.start_line),
    };
    if !out.contains(&mention) {
      out.push(mention);
    }
  }
  out
}

fn find_thread_mut<'a>(canon: &'a mut Canon, id: &str) -> Result<&'a mut Thread, FacadeError> {
  canon
    .threads
    .iter_mut()
    .find(|t| t.id == id)
    .ok_or_else(|| FacadeError::NoSuchThread { id: id.to_string() })
}

/// One criterion's `ac list` row. The ONE home for its computed state, shared
/// by [`Facade::ac_list`] and [`Facade::ac_show`].
fn ac_row(thread: &Thread, c: &Criterion) -> AcRow {
  AcRow {
    id: c.id.clone(),
    text: c.text.clone(),
    // v2's own vocabulary (`bin/intent_acceptance:904-907`). The state
    // is COMPUTED -- for a test-backed AC it is stored nowhere, because
    // satisfaction comes from a covering green test and storing it too
    // would be the double truth data-model.md forbids.
    state: match &c.state {
      AcState::Descoped { to, .. } => format!("descoped-to: {to}"),
      AcState::Withdrawn { reason, .. } => format!("withdrawn: {reason}"),
      // **ISSUE 0137, AND IT WAS A WILDCARD SWALLOWING A STATE THAT HAS
      // TWO EXPLICIT NEIGHBOURS.** `Fiat` fell into the `_` arm below and
      // rendered `satisfied: no` -- indistinguishable from an ordinary
      // open criterion, which is the one outcome hv's ruling exists to
      // prevent: a fiat-closed row must never read as an ordinarily
      // judged one, in either direction.
      //
      // **It was demoted rather than escalated on a census showing ZERO
      // fiat rows store-wide, watched with the census as its trigger.
      // The trigger fired the moment `fc` could write one.** A defect
      // whose only defence is that nothing can reach the state stops
      // being defended by the change that reaches it.
      //
      // **The spelling is this line's vocabulary, not `fiat_marker`'s,
      // and that is the ruling rather than a shortcut.** It now has ONE
      // HOME -- `model::fiat_status`, hv's required composer -- shared
      // with the AT kind, whose record sits beside its status and so has
      // nothing structural forcing a renderer to look at it at all.
      // `fiat_marker` composes the
      // GENERATED VIEW's form; the census in
      // `fiat_close_is_visible_on_every_surface.rs` records that "one
      // composer" is the goal for surfaces that render a marker, and that
      // the property actually held is the weaker true one -- a surface
      // reporting a fiat-closed criterion must make the close visible, by
      // whatever spelling suits it. This line's vocabulary is
      // `<state>: <why>`, set by the two arms above it.
      //
      // **The cascade marker leads**, for the reason `fiat_marker` gives:
      // a reader must not miss that this row was never individually
      // judged, and a marker after the reason reads as a footnote to a
      // decision nobody made about it.
      AcState::Fiat(record) => crate::model::fiat_status("fiat-closed", Some(record)),
      _ => format!(
        "satisfied: {}",
        if contract::resolve(thread, c) == contract::Resolved::Satisfied {
          "yes"
        } else {
          "no"
        }
      ),
    },
    covered_by: thread
      .tests
      .iter()
      .filter(|t| t.covers.iter().any(|covered| covered == &c.id))
      .map(|t| t.id.clone())
      .collect(),
  }
}

fn find_criterion_mut<'a>(
  canon: &'a mut Canon,
  st: &str,
  ac: &str,
) -> Result<&'a mut Criterion, FacadeError> {
  find_thread_mut(canon, st)?
    .criteria
    .iter_mut()
    .find(|c| c.id == ac)
    .ok_or_else(|| FacadeError::NoSuchCriterion {
      st: st.to_string(),
      ac: ac.to_string(),
    })
}

fn find_test_mut<'a>(
  canon: &'a mut Canon,
  st: &str,
  at: &str,
) -> Result<&'a mut AcceptanceTest, FacadeError> {
  find_thread_mut(canon, st)?
    .tests
    .iter_mut()
    .find(|t| t.id == at)
    .ok_or_else(|| FacadeError::NoSuchTest {
      st: st.to_string(),
      at: at.to_string(),
    })
}

/// The v2 slug shape: lowercase, non-alphanumerics to hyphens, collapsed,
/// trimmed, capped at 48 characters.
///
/// **THE SLUG IS A VIEW OF THE TITLE, NOT A SECOND PIECE OF DATA** (hv,
/// 2026-08-27): *"We don't need two pieces of data that get out of sync. The
/// title is the SSOT. The slug is just a way to show the title in an escaped
/// URI friendly way."* So this is the one home, called wherever a slug is
/// wanted, and the stored `Thread::slug` column is vestigial rather than
/// authoritative.
///
/// **THE ST ID REMAINS THE UNIQUE IDENTIFIER AND THE SLUG NEVER RESOLVES**
/// (hv, same ruling): the slug exists so a listing is legible to a human or an
/// LLM where a bare id is opaque. That is why nothing here enforces
/// uniqueness -- two threads may legitimately slug alike, and the 48-char cap
/// below is therefore a display choice rather than an address hazard. It would
/// be one the moment a slug resolved.
///
/// **IDEMPOTENT: `slugify(slugify(t)) == slugify(t)`, and it was not before.**
/// The cap used to be applied AFTER the trailing-hyphen trim, so a title whose
/// 48th character landed on a hyphen kept it -- `ST0020` produced
/// `modernizing-intent-s-elixir-support-for-agentic-`, and feeding that back in
/// returned something different. Truncating first and trimming second is the
/// whole fix, and the order is the property: a slug that changes when it is
/// re-slugged cannot survive a round trip through a URI.
pub fn slugify(title: &str) -> String {
  let mut out = String::new();
  let mut last_hyphen = true;
  for ch in title.chars() {
    if ch.is_ascii_alphanumeric() {
      out.push(ch.to_ascii_lowercase());
      last_hyphen = false;
    } else if !last_hyphen {
      out.push('-');
      last_hyphen = true;
    }
  }
  // TRUNCATE FIRST, THEN TRIM. Reversing these is the non-idempotence above.
  let mut out: String = out.chars().take(48).collect();
  while out.ends_with('-') {
    out.pop();
  }
  out
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn slugs_are_lowercase_hyphenated_and_trimmed() {
    assert_eq!(
      slugify("Add a Rust-based CLI!"),
      "add-a-rust-based-cli",
      "punctuation collapses and no trailing hyphen survives"
    );
    assert_eq!(slugify("  spaced  out  "), "spaced-out");
  }

  /// **hv's stated requirement, and the estate's own data is the fixture.**
  ///
  /// `ST0020`'s real title is the case that failed: its 48th character landed
  /// on a hyphen, and the cap used to be applied after the trim, so the slug
  /// kept it. A slug that changes when re-slugged cannot round-trip through a
  /// URI, which is what the slug is for.
  #[test]
  fn slugifying_a_slug_returns_it_unchanged() {
    for title in [
      "Modernizing Intent's Elixir support for agentic development",
      "Add a Rust-based CLI with a local SQLite DB with bidirectional sync to/from .md files",
      "Add a Rust-based CLI!",
      "  spaced  out  ",
      "help",
      "-- leading and trailing punctuation --",
    ] {
      let once = slugify(title);
      assert_eq!(
        slugify(&once),
        once,
        "slugify is not idempotent for {title:?}"
      );
    }
  }

  /// The cap is enforced, and enforcing it must not reintroduce the hyphen.
  #[test]
  fn a_capped_slug_never_ends_on_a_hyphen() {
    let long = "Modernizing Intent's Elixir support for agentic development";
    let s = slugify(long);
    assert!(s.len() <= 48, "cap not enforced: {s:?}");
    assert!(
      !s.ends_with('-'),
      "the cap must not leave the hyphen the trim removed: {s:?}"
    );
  }

  /// Every variant has a remedy, and no two share one. A remedy that fits two
  /// causes is telling the operator to guess which one they hit.
  #[test]
  fn no_two_error_variants_share_a_remedy() {
    let errors = [
      FacadeError::NoSuchThread {
        id: "ST0099".to_string(),
      },
      FacadeError::ThreadExists {
        id: "ST0056".to_string(),
      },
      FacadeError::NoSuchWorkPackage {
        st: "ST0056".to_string(),
        seq: 9,
      },
      FacadeError::NoSuchCriterion {
        st: "ST0056".to_string(),
        ac: "AC-09.9".to_string(),
      },
      FacadeError::NoSuchTest {
        st: "ST0056".to_string(),
        at: "AT-09.9".to_string(),
      },
      FacadeError::GateBlocked {
        scope: "ST0056".to_string(),
        verdict: "x".to_string(),
        remedy: contract::SATISFY_THE_REMAINING.to_string(),
      },
      FacadeError::ComputedSatisfaction {
        ac: "AC-03.1".to_string(),
      },
      FacadeError::NotOffScope {
        ac: "AC-03.1".to_string(),
        verb: "reinstate".to_string(),
        wanted: "withdrawn".to_string(),
      },
    ];
    let mut remedies: Vec<String> = errors.iter().map(crate::remedy::Remedy::remedy).collect();
    let before = remedies.len();
    remedies.sort();
    remedies.dedup();
    assert_eq!(before, remedies.len(), "two variants share a remedy text");
  }
}
