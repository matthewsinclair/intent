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
//! [`FacadeContext::today`]. That is not the renderer's no-clock law (D23) --
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
use crate::intentfiles::Realised;
use crate::model::{
  AcKind, AcState, AcceptanceTest, AtKind, AtStatus, Attachment, Criterion, Issue, IssueStatus,
  TShirt, Thread, ThreadStatus, WorkPackage, WpStatus, to_canonical_json,
};
use crate::project::{EditDisposition, Migration, Pending, Project, ThreadFile};
use crate::realise;
use crate::store::{Store, StoreError};
use crate::sync::Scope as SyncScope;
use crate::transitions;
use crate::views::{self, RenderContext};
use crate::write_set::{Applied, WriteError, WriteSet};
use crate::{intentfiles, organize};

/// Ambient facts a facade call runs with. Explicit rather than discovered, so
/// a verb's result is a function of its arguments.
#[derive(Debug, Clone)]
pub struct FacadeContext {
  /// Who is acting. `local` until the 3.2 agent bus gives principals meaning.
  pub principal: String,
  /// The project's UUID (D15). Stamped at migration; empty on a pre-migration
  /// project, which the event log records honestly rather than inventing one.
  pub project_id: String,
  /// The Intent version, for generated banners.
  pub version: String,
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
fn declare_default_if_absent(project: &Project, threads: &[Thread]) -> Result<(), std::io::Error> {
  let path = project.intentfiles_path();
  if path.exists() {
    return Ok(());
  }
  let open: Vec<(String, crate::model::ThreadStatus)> =
    threads.iter().map(|t| (t.id.clone(), t.status)).collect();
  std::fs::write(&path, intentfiles::default_declaration(&open))
}

fn converge_gitignore(project: &Project) -> Result<(), std::io::Error> {
  let rule = format!(
    "{}/.cache/",
    project
      .intent_dir()
      .file_name()
      .map(|n| n.to_string_lossy().into_owned())
      .unwrap_or_else(|| "intent".to_string())
  );
  let path = project.root().join(".gitignore");
  let current = std::fs::read_to_string(&path).unwrap_or_default();
  if current.lines().any(|l| l.trim() == rule) {
    return Ok(());
  }
  let mut next = current;
  if !next.is_empty() && !next.ends_with('\n') {
    next.push('\n');
  }
  next.push_str("\n# The Intent runtime store: per-machine, rebuilt from the committed extract.\n");
  next.push_str(&rule);
  next.push('\n');
  std::fs::write(&path, next)
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
/// consumer's formatter anywhere else -- which is why this writes five patterns
/// and never `*`.
///
/// The patterns come from [`Project::generated_view_patterns`], beside the
/// methods that produce the real paths, so this function holds no roster of its
/// own to drift.
pub(crate) fn converge_formatter_exclusion(project: &Project) -> Result<(), std::io::Error> {
  let path = project.root().join(".prettierignore");
  let current = std::fs::read_to_string(&path).unwrap_or_default();
  let missing: Vec<String> = project
    .generated_view_patterns()
    .into_iter()
    .filter(|pattern| !current.lines().any(|l| l.trim() == pattern))
    .collect();
  if missing.is_empty() {
    return Ok(());
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
  for pattern in missing {
    next.push_str(&pattern);
    next.push('\n');
  }
  std::fs::write(&path, next)
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
      serde_json::Value::String(uuid::Uuid::new_v4().to_string()),
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
  #[error("{scope} is not ready to close -- {verdict}")]
  GateBlocked { scope: String, verdict: String },
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
  #[error("no schema face named `{face}`")]
  NoSuchFace { face: String },
  #[error("no issue {number:04} in this project")]
  NoSuchIssue { number: u32 },
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
  #[error("could not update the runtime store")]
  Store(#[from] StoreError),
  #[error("could not read the committed canon")]
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
}

impl crate::remedy::Remedy for FacadeError {
  /// What the operator should DO. Every variant has one, and no two variants
  /// share a remedy text -- a remedy that fits two different causes is telling
  /// the operator to guess which one they hit (AC-04.4).
  fn remedy(&self) -> String {
    match self {
      // The `why` already carries the rule that refused; a remedy repeating it
      // would be the doubled rendering `IngestError::Refused` documents.
      Self::WriteNotAddressable { .. } => {
        "`PUT` json to a caller-assigned id (an AC or an AT); everything else is a \
         `POST` to the collection address"
          .to_string()
      }
      Self::FieldNotWritable { .. } => {
        "go to the door the refusal names: a lifecycle verb for a field a state machine owns, \
         and the member's own address for a collection"
          .to_string()
      }
      Self::NoSuchThread { .. } => {
        "run `intent st list` to see the threads this project has".to_string()
      }
      Self::ThreadExists { id } => {
        format!("pick a different id, or work on the existing one with `intent st show {id}`")
      }
      Self::NoSuchWorkPackage { st, .. } => {
        format!("run `intent wp list {st}` to see its work packages")
      }
      Self::NoSuchCriterion { st, .. } => {
        format!("run `intent ac list {st}` to see the criteria in its contract")
      }
      Self::NoSuchTest { st, .. } => {
        format!("run `intent at list {st}` to see the tests in its contract")
      }
      Self::GateBlocked { .. } => {
        "satisfy or formally descope the remaining criteria, then close again".to_string()
      }
      Self::ComputedSatisfaction { ac } => format!(
        "set the covering test green instead -- `intent at set <AT> green` -- or make {ac} a non-test criterion with named evidence"
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
        "the change is safe in the store -- do NOT retry it. Clear the filesystem cause, then run `intent st sync` to rewrite the files from the store. Do NOT reach for the disk -> db direction, which reads the FILES into the database and would overwrite the change with the stale copy".to_string()
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
      Self::OffScope { undo, ac, verb, .. } => {
        format!("run `intent ac {undo} <thread> {ac}` first if you mean to {verb} it -- recording evidence for a requirement nobody is working on is the bookkeeping descope replaced")
      }
      Self::WrongOffScopeState { verb, ac, .. } => {
        format!("run `intent ac {verb} <thread> {ac}` instead -- a descoped requirement still exists on another thread, and a withdrawn one does not exist at all")
      }
      Self::BadQuery { .. } => {
        "search takes an FTS5 expression -- quote a phrase, and escape or drop bare punctuation like `:` and `*`".to_string()
      }
      Self::NoSuchFace { .. } => {
        "run `intent schema` with no argument to print every face, which also names them".to_string()
      }
      // `--kind all` rather than a bare list: the default bucket is OPEN, so a
      // remedy without it sends someone looking for a closed issue in a list
      // that cannot contain one, and they conclude it is gone.
      Self::NoSuchIssue { .. } => {
        "run `intent issues list --kind all` to see every issue this project has, closed ones included".to_string()
      }
      // Delegated for the same reason the message is: the arithmetic that
      // names the two honourable values either side belongs with the rule.
      // Delegated, because the remedy DIFFERS by state: below the v2.19.0
      // floor it is the two-hop, and naming the v3 migrator there would send
      // half the operators who read it to a command that refuses them.
      Self::Unmigrated(pending) => pending.remedy(),
      // Same delegation and the same reason: `Pending` owns the two-hop, and
      // this variant differs from `Unmigrated` in its MESSAGE, not its cure.
      Self::BelowMigrationFloor(pending) => pending.remedy(),
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
      Self::NoSuchEditable { present, .. } => {
        format!("this artefact carries: {}", present.join(", "))
      }
      Self::NotHydratable { form, .. } => format!(
        "address an ARTEFACT instead -- a thread or an issue. A `{form}` has no files of its own, so there is nothing for realisation to create; if you meant the thread that carries it, address the thread."
      ),
      Self::NoManifestToUnlistFrom { path, .. } => format!(
        "write one first with `intent organize --default`, which declares the open threads; then re-run. Removing a thread's files is a change to a list that has to exist before it can be changed, and {path} does not."
      ),
      Self::DehydrationRefused { .. } => "each line above names a file whose bytes the store cannot be shown to hold. `intent doctor` names the difference; if the copy on disk is the one you want, take it into canon with `intent sync --to-store`, and if the store is right, the file is a hand edit to discard. Nothing was removed, so nothing needs undoing.".to_string(),
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
      // IDENTICALLY** -- an unknown sigil on line 12 and an unterminated region
      // on line 40 both came out as `could not read the realisation manifest`.
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
  /// **THE QUESTION COULD NOT BE ASKED** -- no repository, or git did not run.
  ///
  /// Carried as its own variant rather than as an empty list, because "nothing
  /// is uncommitted" and "I could not look" are the two answers an operator
  /// most needs to tell apart, and collapsing them prints a clean bill of
  /// health nobody earned.
  UnsyncedUnknown,
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
fn declared_list_edit(op: &str) -> Option<ListAction> {
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
    "st.start" | "st.resume" | "st.reopen" => Some(ListAction::Add),
    "st.done" | "st.cancel" | "st.hold" | "st.triage" => Some(ListAction::Remove),
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
    Self::readable(&project)?;
    let mut store = Store::open(&project.db_path()).map_err(FacadeError::Store)?;
    // The daily-driver path: answer from the store unless the tree moved.
    let canon = ingest::load_fresh(&project, &mut store)?;
    Ok(Self {
      project,
      store,
      canon,
      ctx,
    })
  }

  /// Open against an in-memory store, for callers that do not want the DB on
  /// disk (tests, and the daemonless read paths).
  pub fn open_in_memory(project: Project, ctx: FacadeContext) -> Result<Self, FacadeError> {
    Self::readable(&project)?;
    let mut store = Store::open_in_memory().map_err(FacadeError::Store)?;
    let canon = ingest::load(&project, &mut store)?;
    Ok(Self {
      project,
      store,
      canon,
      ctx,
    })
  }

  pub fn project(&self) -> &Project {
    &self.project
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
    if let crate::project::Migration::Pending(pending) = project.migration()
      && pending.below_floor
    {
      return Err(FacadeError::BelowMigrationFloor(pending));
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
    } = plan;

    let files = writes.len();
    let applied = writes.commit()?;

    // Everything past here has files on disk, so a failure unwinds them rather
    // than leaving a half-converted estate. `keep()` is reached only once the
    // stamp has landed.
    let finish = || -> Result<(), FacadeError> {
      let mut store = Store::open(&project.db_path())?;
      store.rebuild(&threads, &issues)?;
      converge_gitignore(project).map_err(|cause| FacadeError::MigrationHalted {
        step: "adding the store to .gitignore",
        cause,
      })?;
      converge_formatter_exclusion(project).map_err(|cause| FacadeError::MigrationHalted {
        step: "keeping the formatter off generated views",
        cause,
      })?;
      declare_default_if_absent(project, &threads).map_err(|cause| {
        FacadeError::MigrationHalted {
          step: "writing the realisation manifest",
          cause,
        }
      })?;
      stamp_version(project).map_err(|cause| FacadeError::MigrationHalted {
        step: "stamping the project version",
        cause,
      })
    };
    match finish() {
      Ok(()) => {
        applied.keep();
        Ok(Upgraded {
          threads: threads.len(),
          issues: issues.len(),
          files,
          carried,
          already_migrated,
          already_migrated_issues,
          dispositions,
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

  /// Every criterion with its COMPUTED state and its covering tests.
  ///
  /// The state is computed here rather than read off the criterion, because
  /// for a test-backed AC it is not stored anywhere: satisfaction comes from a
  /// covering green test, and storing it too would be the double truth
  /// data-model.md forbids.
  pub fn ac_list(&self, st: &str) -> Result<Vec<AcRow>, FacadeError> {
    let thread = self.st_show(st)?;
    Ok(
      thread
        .criteria
        .iter()
        .map(|c| AcRow {
          id: c.id.clone(),
          text: c.text.clone(),
          // v2's own vocabulary (`bin/intent_acceptance:904-907`). The state
          // is COMPUTED -- for a test-backed AC it is stored nowhere, because
          // satisfaction comes from a covering green test and storing it too
          // would be the double truth data-model.md forbids.
          state: match &c.state {
            AcState::Descoped { to, .. } => format!("descoped-to: {to}"),
            AcState::Withdrawn { reason, .. } => format!("withdrawn: {reason}"),
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
        })
        .collect(),
    )
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

  /// Full-text search across every authored section -- thread prose, issue
  /// bodies, work-package text (AC-06.4).
  ///
  /// The query goes to FTS5 as written, so `foo OR bar` and `"a phrase"` work.
  /// A malformed expression comes back as [`FacadeError::BadQuery`] carrying
  /// SQLite's own complaint in its cause chain: the remedy names the likely
  /// fix, and the chain still says exactly what happened, so a genuinely
  /// unhealthy store is not disguised as a typo.
  pub fn search(&self, query: &str) -> Result<Vec<crate::prose::DocSection>, FacadeError> {
    self.store.search(query).map_err(|cause| {
      if matches!(cause, StoreError::Sqlite(_)) {
        FacadeError::BadQuery {
          query: query.to_string(),
          cause,
        }
      } else {
        FacadeError::Store(cause)
      }
    })
  }

  /// How many prose sections the index holds -- the question that makes an
  /// empty search result interpretable.
  ///
  /// **A search over an unpopulated index returns exactly what a genuine miss
  /// returns**, so a caller reading zero hits cannot tell "the phrase is not
  /// there" from "nothing has been indexed, so the question was never asked"
  /// (AC-06.4). Every caller that reports an empty result to a human owes them
  /// that distinction, and it cannot be derived from the result itself.
  pub fn prose_sections_indexed(&self) -> Result<usize, FacadeError> {
    self.store.doc_section_count().map_err(FacadeError::Store)
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
    for (path, content) in set.writes() {
      let Ok(meta) = std::fs::metadata(path) else {
        continue;
      };
      let on_disk = meta.len() as usize;
      if on_disk > content.len() {
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
  fn record_disk_act(&self, op: &str, payload: serde_json::Value) -> Result<(), FacadeError> {
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
    Ok(())
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

  pub fn organize(&mut self, mode: organize::Mode) -> Result<organize::Report, FacadeError> {
    let realised = self.manifest_for_action()?;
    // **NOTHING REGENERATES THIS FILE, BY hv's RULING (`d2b63bc3`).** organize
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
    let previous = self.store.file_index().map_err(FacadeError::Store)?;

    let (tree, digest) =
      organize::observe(&self.project, &previous).map_err(FacadeError::Organize)?;
    let plan = {
      let ctx = self.render_ctx()?;
      organize::plan(&self.project, &self.canon, &realised, &ctx, &tree, digest)
    };

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
        self.record_disk_act(
          "disk.organize",
          serde_json::json!({
            "hydrated": hydrated,
            "rewritten": rewritten,
            "dehydrated": dehydrated,
            "pruned": pruned,
            "refused": report.refused.len(),
          }),
        )?;
      }
    }
    Ok(report)
  }

  /// Make an addressed artefact's files exist on disk, and say which ones do.
  ///
  /// **TWO INDEPENDENT IDEMPOTENT STEPS, NEITHER GUARDING THE OTHER** (ic's
  /// correction, and it killed a defect before it existed). Materialise if
  /// absent, then pin if not pinned. The obvious shape -- return early when the
  /// files are already there -- skips the PIN in exactly the ordinary case: the
  /// artefact is realised because it is currently `wip`, its id sits in the
  /// GENERATED region, and it is not pinned. **Presence is true and pinned-ness
  /// is false, and they disagree on the common path rather than in a corner.**
  /// `pin_writes_to_the_list.rs` already reds that, so the estate had the test
  /// before it had this caller. (It was `edit_writes_pinned_region.rs` until
  /// 2026-08-20; the file kept the assertion and lost a name that described a
  /// design hv had deleted.)
  ///
  /// **Pinning is what makes the decision outlive STATUS.** The generated region
  /// is a function of today's board; a pin is a durable statement that this
  /// artefact stays on disk. Hydrating without pinning hands the files straight
  /// back to the next `organize`.
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
    if let Some(authority) = &address.authority {
      return Err(FacadeError::NotHydratable {
        form: address.entity.form(),
        why: format!(
          "the address names the project `{authority}` rather than this one, and realising another project's artefact into this tree is not something an empty authority would have meant"
        ),
      });
    }
    let Some((sigil, id)) = address.entity.artefact() else {
      return Err(FacadeError::NotHydratable {
        form: address.entity.form(),
        why: "only an artefact -- a steel thread -- is named by `.intentfiles`, so it is the smallest thing realisation can address. An ISSUE lives only in canon and the store and has no realised form, so there is nothing to realise it INTO".to_string(),
      });
    };
    let id = id.to_string();

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
    // **ONE ARM, AND IT IS ONE ARM BECAUSE THE OTHER ONE ADDRESSED THE WRONG
    // LAYER.** This matched `Sigil::Issue` to `issues_dir()`, which is
    // `intent/.canon/issues/` -- so a realisation verb's home resolved into
    // CANON for one of its two inputs. It was inert only because
    // `organize::plan` emits no step under `intent/.canon/`, which is a
    // property of the plan and not a bound this code stated. hv retired
    // `ISSUE:` from the grammar on 2026-08-20, and `Address::artefact` now
    // answers `None` for an issue, so the case is refused above rather than
    // handled wrongly here.
    let home = match sigil {
      intentfiles::Sigil::SteelThread => self.project.thread_dir(&id),
    };
    let mine: Vec<_> = whole
      .steps
      .iter()
      .filter(|s| s.path.starts_with(&home))
      .cloned()
      .collect();
    let scoped = organize::Plan {
      steps: mine,
      digest: whole.digest.clone(),
      preconditions: whole.preconditions.clone(),
      estate_root: whole.estate_root.clone(),
    };
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
    // **So the set is asked of the two owners rather than reconstructed.**
    // `views::render_all` is THE renderer and says which views exist for this
    // artefact; canon's `attachments` is THE store's own list. Neither is a
    // restatement of what an artefact owns -- both are the authority for their
    // half, filtered to this artefact's directory.
    let mut owned: Vec<std::path::PathBuf> = {
      let ctx = self.render_ctx()?;
      views::render_all(&self.project, &self.canon, &ctx)
        .into_iter()
        .map(|v| v.path)
        .filter(|p| p.starts_with(&home))
        .collect()
    };
    if let Some(thread) = self.canon.threads.iter().find(|t| t.id == id) {
      for attachment in &thread.attachments {
        owned.push(self.project.thread_dir(&id).join(&attachment.path));
      }
    }
    owned.retain(|p| p.exists());
    owned.sort();
    owned.dedup();
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
    Ok(owned)
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
    let text = intentfiles::default_declaration(&threads);
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

  pub fn dehydrate(&mut self, address: &Address) -> Result<Dehydrated, FacadeError> {
    if let Some(authority) = &address.authority {
      return Err(FacadeError::NotHydratable {
        form: address.entity.form(),
        why: format!(
          "the address names the project `{authority}` rather than this one, and removing another project's files from this tree is not something an empty authority would have meant"
        ),
      });
    }
    let Some((sigil, id)) = address.entity.artefact() else {
      return Err(FacadeError::NotHydratable {
        form: address.entity.form(),
        why: "only an artefact -- a steel thread -- is named by `.intentfiles`, so it is the smallest thing realisation can address. An ISSUE lives only in canon and the store and has no realised form, so there is nothing to remove".to_string(),
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
      steps: mine,
      digest: whole.digest.clone(),
      preconditions: whole.preconditions.clone(),
      estate_root: whole.estate_root.clone(),
    };
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

  pub fn sync_to_disk(&mut self, scope: &SyncScope) -> Result<usize, FacadeError> {
    self.refuse_if_the_last_ingest_was_refused()?;
    let (threads, issues) = self.store.load_canon().map_err(FacadeError::Store)?;
    let sections = self.store.doc_sections().map_err(FacadeError::Store)?;
    let canon = Canon {
      threads,
      issues,
      sections,
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
    let set = self.projection(&canon, &all_threads, &all_issues)?;
    self.refuse_if_this_would_empty_a_populated_face(&canon, &set)?;
    let applied = set.commit()?;
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
      let mut canon = ingest::resync(project, store, scope)?;

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
      let refused = ingest::collect_attachments_into(project, &mut canon);
      if !refused.is_empty() {
        return Err(IngestError::from(crate::finding::Refusal::new(refused)));
      }
      store.rebuild(&canon.threads, &canon.issues)?;

      // **The index is derived during `read`, which by design has not seen the
      // attachments yet, so it has to be re-derived here or the carry lands in
      // canon and reaches the FTS index nowhere.** That is AC-06.4's failure
      // shape exactly -- content present and findable by nothing -- so rebuilding
      // the thread sections is part of the carry rather than a tidy-up after it.
      if !canon.threads.is_empty() {
        canon
          .sections
          .retain(|s| s.owner_type != "thread" && s.owner_type != "work-package");
        let mut rebuilt = Vec::new();
        for thread in &canon.threads {
          ingest::collect_wp_text(project, &mut rebuilt, thread);
        }
        canon.sections.append(&mut rebuilt);
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
    let set = self.projection(&canon, &all_threads, &all_issues)?;
    let applied = set.commit()?;
    let wrote = self.estate_paths(&applied);
    applied.keep();
    // **THE DISK ACT OF A RESTORE IS THE RE-PROJECTION, AND IT IS THE HALF
    // NOTHING ELSE RECORDS.** The store side is already covered by
    // `ingest::recording` above (AC-03.13); this is the other side, and it is
    // the one AC-09.1 names -- the restore direction rewrites the views of what
    // it read, so files move under a verb whose name says nothing about
    // writing any.
    //
    // **RECORDED AFTER `restore_event_log` HAS ALREADY REPLACED THE STORE'S LOG
    // FROM THE FILE**, so an event the file did not carry is gone before this
    // one is appended. That is the declared destructive direction behaving as
    // declared, and it is stated here because it is the one place where the
    // table that cannot be re-derived from disk IS re-derived from disk.
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
    let on_disk = ingest::read(&self.project)?;
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
        Some(_) => out.push(format!("{}: differs on disk", thread.id)),
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
    Ok(out)
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
    self.apply_with_state(
      crate::event::TODO_FLUSH,
      Subject {
        kind: "todo".to_string(),
        id: "watermark".to_string(),
      },
      serde_json::json!({ "cleared": cleared.len() }),
      next,
      crate::store::ProjectStateEdit::SetTodoWatermark,
    )?;
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
  ) -> Result<WriteSet, FacadeError> {
    let mut set = WriteSet::new();
    for thread in threads {
      set.add(
        self.project.thread_json(&thread.id),
        to_canonical_json(thread).map_err(|e| FacadeError::Store(StoreError::Serde(e)))?,
      );
    }
    for issue in issues {
      set.add(
        self.project.issue_json(issue.number),
        to_canonical_json(issue).map_err(|e| FacadeError::Store(StoreError::Serde(e)))?,
      );
    }
    // **PROJECT STATE, WRITTEN ON EVERY SYNC AND NOT NARROWED BY A THREAD
    // SCOPE.** Issues are skipped by a scoped sync because forty issue files
    // nobody asked about is the estate-wide write a scope exists to prevent.
    // This is ONE file holding one scalar, and `WriteSet::commit` skips a path
    // whose bytes already match -- so writing it always costs nothing when
    // nothing moved, and skipping it would let `sync --to-disk ST0056` leave a
    // stale cutoff on disk after a flush.
    set.add(
      self.project.project_json(),
      to_canonical_json(&crate::model::ProjectState::new(
        self.store.todo_watermark().map_err(FacadeError::Store)?,
      ))
      .map_err(|e| FacadeError::Store(StoreError::Serde(e)))?,
    );
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
    for view in views::render_all(&self.project, canon, &self.render_ctx()?) {
      if let Realised::Declared(ref declared) = realised
        && let Some(owner) = self.owning_thread(&view.path, canon)
        && !declared.contains(&owner)
      {
        continue;
      }
      set.add(view.path, view.content);
    }
    Ok(set)
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
  /// because [`ingest::from_md`] refuses until WP-10 lands the parser. It
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

  pub fn doctor(
    project: &Project,
    ctx: &FacadeContext,
    store: Option<&crate::store::Store>,
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
    // 2026-08-22). `a_refused_view_is_still_realised_because_the_refusal_is_
    // about_authoring` argues that a refusal must not roll back a completed
    // act, and it is right; **a ruling about rollback cannot reach an act that
    // was never performed.** That test is AMENDED onto the `NoSuchEditable`
    // arm below -- a real refusal after a real hydrate -- where its argument
    // still bites. It was moved, never deleted.
    //
    // `Project::edit_disposition` is a pure function of the FILENAME: it
    // consults no disk and no store, so nothing was ever gained by deciding it
    // late. A non-artefact address falls through deliberately -- `hydrate`
    // owns that refusal and makes it before its own first write.
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

    let realised = self.hydrate(address)?;
    // `hydrate` refuses every non-artefact form before this point, so the
    // address is known to name one.
    let (_, id) = address
      .entity
      .artefact()
      .expect("hydrate refuses any address without an artefact");

    let wanted = self.project.thread_dir(id).join(&rel);
    if !realised.contains(&wanted) {
      return Err(FacadeError::NoSuchEditable {
        path: self.project.relative(&wanted),
        // **WHAT IS THERE, NOT MERELY THAT THIS IS NOT.** The operator asked
        // for a file this artefact does not carry, and the set that answers
        // the follow-up question is the one already in hand.
        //
        // **THREAD-RELATIVE AND DEDUPED, BECAUSE THE BASENAME IS NOT THE
        // ANSWER.** Taking `file_name()` printed `info.md` ten times on a
        // thread with nine work packages -- every `WP/<NN>/info.md` collapsing
        // onto the cover's name. A remedy that repeats one word ten times is
        // read as a rendering fault and stops being read at all, and it also
        // told the operator that `info` was available when `info` is the one
        // thing this verb refuses.
        present: {
          let dir = self.project.thread_dir(id);
          let mut names: Vec<String> = realised
            .iter()
            .filter_map(|p| p.strip_prefix(&dir).ok())
            .map(|p| p.to_string_lossy().into_owned())
            .collect();
          names.sort();
          names.dedup();
          names
        },
      });
    }
    Ok(wanted)
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
  /// suppress the FILES. [`Facade::apply`] projects every changed thread
  /// unconditionally and consults no manifest, so the views are written either
  /// way and the next `organize` is what removes them. The flag's real and only
  /// effect is on the list. Reported rather than worked around: filtering
  /// `apply` by the manifest is a change to the core write path, not to this
  /// verb.
  pub fn st_new_listing(&mut self, title: &str, list: ListEdit) -> Result<String, FacadeError> {
    let id = self.next_thread_id();
    if self.canon.threads.iter().any(|t| t.id == id) {
      return Err(FacadeError::ThreadExists { id });
    }
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
    self.apply(
      "st.new",
      Subject {
        kind: "thread".to_string(),
        id: id.clone(),
      },
      json!({"title": title}),
      next,
    )?;
    self.edit_list("st.new", &id, list)?;
    Ok(id)
  }

  /// Accept a thread out of triage and into the backlog.
  pub fn st_triage(&mut self, id: &str) -> Result<Outcome, FacadeError> {
    self.set_thread_status(
      id,
      ThreadStatus::NotStarted,
      "st.triage",
      None,
      ListEdit::AsDeclared,
    )
  }

  pub fn st_start(&mut self, id: &str) -> Result<Outcome, FacadeError> {
    self.set_thread_status(
      id,
      ThreadStatus::Wip,
      "st.start",
      None,
      ListEdit::AsDeclared,
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
    )
  }

  /// Close a thread. Consults the close gate first -- the single authority, so
  /// there is no path that closes without it.
  /// Close a thread. The gate is a DECLARED guard and is run by the shared
  /// setter, after the self-loop test -- see [`Facade::check_gate`].
  pub fn st_done(&mut self, id: &str) -> Result<Outcome, FacadeError> {
    self.st_done_listing(id, ListEdit::AsDeclared)
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
  pub fn st_done_listing(&mut self, id: &str, list: ListEdit) -> Result<Outcome, FacadeError> {
    self.set_thread_status(id, ThreadStatus::Completed, "st.done", None, list)
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
    )
  }

  pub fn st_cancel(&mut self, id: &str, reason: &str) -> Result<Outcome, FacadeError> {
    self.st_cancel_listing(id, reason, ListEdit::AsDeclared)
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
  pub fn st_cancel_listing(
    &mut self,
    id: &str,
    reason: &str,
    list: ListEdit,
  ) -> Result<Outcome, FacadeError> {
    self.set_thread_status(id, ThreadStatus::Cancelled, "st.cancel", Some(reason), list)
  }

  /// `list` is [`ListEdit::AsDeclared`] for every caller but `st done --keep`.
  fn set_thread_status(
    &mut self,
    id: &str,
    status: ThreadStatus,
    op: &'static str,
    reason: Option<&str>,
    list: ListEdit,
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
    if from == status {
      return Ok(Outcome::AlreadyThere {
        state: from.display().to_string(),
      });
    }

    Self::check_transition("Thread", "status", op, &crate::model::enum_str(&from), id)?;
    self.check_gate(("Thread", "status", op), id, id, Scope::Thread)?;
    // The guard still refuses a verb that REQUIRES a reason and was given
    // none; its return value is no longer the thing that writes the field.
    Self::check_reason("Thread", "status", op, reason)?;
    // Read BEFORE anything is written -- see [`Facade::closing_notes`].
    let notes = self.closing_notes(op, id, list)?;
    let mut next = self.canon.clone();
    let thread = find_thread_mut(&mut next, id)?;
    thread.status = status;
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
    thread.completed = match status {
      ThreadStatus::Completed | ThreadStatus::Cancelled => Some(String::new()),
      _ => None,
    };
    self.apply(
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
      next,
    )?;
    // **AFTER `apply`, DELIBERATELY** -- see [`Facade::edit_list`] for why the
    // interrupted-between state has to be the one that degrades into `--keep`.
    self.edit_list(op, id, list)?;
    Ok(if notes.is_empty() {
      Outcome::Moved
    } else {
      Outcome::MovedWith { notes }
    })
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
    if verdict.is_pass() {
      return Ok(());
    }
    Err(FacadeError::GateBlocked {
      scope: label.to_string(),
      verdict: verdict.line(label),
    })
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
      status_reason: None,
    });
    self.apply(
      "wp.new",
      Subject {
        kind: "wp".to_string(),
        id: format!("{st}/{seq:02}"),
      },
      json!({"title": title, "scope": crate::model::enum_str(&scope)}),
      next,
    )?;
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
  /// setter after the self-loop test -- see [`Facade::check_gate`].
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
      .map(|()| Outcome::Moved)
  }

  fn set_wp_status(
    &mut self,
    st: &str,
    seq: u32,
    status: WpStatus,
    op: &'static str,
    reason: Option<&str>,
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
    self
      .apply(
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
        next,
      )
      .map(|()| Outcome::Moved)
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
  /// **THE ID ORIGIN IS CALLER-ASSIGNED (WP-08), so the shape is an idempotent
  /// `PUT` to the entity address and never a `POST` to the collection.** You
  /// cannot `POST` here: the caller already knows the id, and a server-assigned
  /// one would make `AC-08.6` unaddressable before the write.
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
    let row = Criterion {
      id: ac.to_string(),
      text: text.to_string(),
      kind,
      state: match kind {
        AcKind::Test => AcState::Computed,
        AcKind::NonTest => AcState::Unsatisfied,
      },
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
    status: AtStatus,
    note: Option<String>,
  ) -> Result<Outcome, FacadeError> {
    // **A CREATE ON AN EXISTING ID IS A FULL REPLACEMENT, SO EVERY FIELD THIS
    // VERB DOES NOT SET IS A FIELD IT DESTROYS.** `note` and `legacy` are the
    // two, and both were being written as `None` and then applied over the
    // stored row -- so `at new` on a row that already existed did not merely
    // fail to set a note, it ATE one that was there.
    //
    // **AND THE SAFE PATH WAS THE UNDOCUMENTED ONE, WHICH IS WHAT MADE IT A
    // TRAP RATHER THAN A WALL.** A re-cite done as a canon edit plus
    // `intent sync --to-store` carries `file` and keeps `note`; the verb built
    // for the job was the one that lost it. Measured on this repository: six
    // ST0061 notes destroyed by a single re-cite and recovered from a git blob.
    //
    // Carried rather than defaulted. When `--note` lands it overrides this;
    // ABSENCE of a note argument can only mean "not saying", never "clear it",
    // because a verb with no way to express a value has no way to express
    // erasing one either.
    let carried = self
      .st_show(st)?
      .tests
      .iter()
      .find(|t| t.id == at)
      .map(|t| (t.note.clone(), t.legacy.clone()))
      .unwrap_or((None, None));

    let row = AcceptanceTest {
      id: at.to_string(),
      kind,
      file,
      prose,
      covers,
      status,
      // An explicit `--note` OVERRIDES what was carried; absence carries. The
      // asymmetry is the point: a verb with no way to express a value has no
      // way to express erasing one, so silence can only mean "not saying".
      note: note.or(carried.0),
      legacy: carried.1,
    };

    // The prospective thread: what canon WOULD hold if this write landed. The
    // contract is asked about that, so a refusal costs nothing and a pass is a
    // statement about the row being created rather than about the one before it.
    let mut prospective = self.st_show(st)?.clone();
    match prospective.tests.iter_mut().find(|t| t.id == row.id) {
      Some(existing) => *existing = row.clone(),
      None => prospective.tests.push(row.clone()),
    }
    prospective.tests.sort_by(|a, b| a.id.cmp(&b.id));
    let report = contract::contract_report(
      &prospective,
      None,
      &contract::RepoFiles(self.project.root()),
    );
    // NARROWED TO THE ROW BEING WRITTEN, and the narrowing is the point rather
    // than politeness: a thread carrying a pre-existing finding elsewhere would
    // otherwise make every create on it impossible, and a verb that refuses on
    // somebody else's defect is one nobody can use. Inherited breakage is
    // `at lint`'s to report, not this verb's to block on.
    let mine: Vec<&String> = report.findings.iter().filter(|f| f.contains(at)).collect();
    if !mine.is_empty() {
      return Err(FacadeError::WriteNotAddressable {
        url: format!("intent:///threads/{st}/at/{at}"),
        why: format!(
          "the row would not satisfy the acceptance-test contract: {}",
          mine
            .iter()
            .map(|f| f.as_str())
            .collect::<Vec<_>>()
            .join("; ")
        ),
      });
    }

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
      .set_ac_state(st, ac, AcState::Unsatisfied, "ac.unsatisfy", json!({}))
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
    Self::check_transition("Criterion", "state", op, state_name(current), ac)?;
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
      .map(|()| Outcome::Moved)
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
    let from = self
      .st_show(st)?
      .tests
      .iter()
      .find(|t| t.id == at)
      .map(|t| t.status)
      .ok_or_else(|| FacadeError::NoSuchTest {
        st: st.to_string(),
        at: at.to_string(),
      })?;

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

    let mut next = self.canon.clone();
    {
      let row = find_test_mut(&mut next, st, at)?;
      row.status = status;
      if let Some(text) = note.as_ref() {
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
      .map(|()| Outcome::Moved)
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
    if !address.is_local() {
      return Err(FacadeError::WriteNotAddressable {
        url: address.to_url(),
        why: "a cross-project write resolves against intentd's project registry".to_string(),
      });
    }
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
    if !address.is_local() {
      return Err(FacadeError::WriteNotAddressable {
        url: address.to_url(),
        why: "a cross-project write resolves against intentd's project registry".to_string(),
      });
    }
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
            *existing = row;
            Outcome::Moved
          }
          None => {
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
          .map(|()| outcome)
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
        let outcome = match holder.criteria.iter_mut().find(|c| &c.id == ac) {
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
          .map(|()| outcome)
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
        // its own, so the thread door is the only door it has.
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
        // it has no address of its own, so the thread door is its ONLY door, and
        // it was the door that emptied it.
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
          .map(|()| Outcome::Moved)
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
        // **`edit_disposition` HANDS OVER `Unattached` TOO, AND `put` MUST
        // NOT.** They are the same answer to different questions: `edit` may
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
      .map(|()| outcome)
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
    if !address.is_local() {
      return Err(FacadeError::WriteNotAddressable {
        url: address.to_url(),
        why: "a cross-project write resolves against intentd's project registry".to_string(),
      });
    }

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
        let Some(row) = Self::splice_one_field(existing, field, value, &refuse)? else {
          return Ok(Outcome::AlreadyThere {
            state: "unchanged".to_string(),
          });
        };
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
        let Some(row) = Self::splice_one_field(existing, field, value, &refuse)? else {
          return Ok(Outcome::AlreadyThere {
            state: "unchanged".to_string(),
          });
        };
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
      .map(|()| Outcome::Moved)
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
  /// It is NOT taken from [`Ctx::principal`], which is the hard-coded `local`
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
    let mut next = self.canon.clone();
    next.issues.push(issue);
    self.apply(
      "issues.add",
      Subject {
        kind: "issue".to_string(),
        id: format!("{number:04}"),
      },
      json!({"title": title, "severity": severity}),
      next,
    )?;
    Ok(number)
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
    self
      .apply(
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
      )
      .map(|()| Outcome::Moved)
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
  ) -> Result<(), FacadeError> {
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
    mut next: Canon,
    project_state: crate::store::ProjectStateEdit,
  ) -> Result<(), FacadeError> {
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
    let envelope = Envelope::minted(
      &self.ctx.principal,
      &self.ctx.project_id,
      op,
      subject,
      payload,
    );
    // The prose index is refreshed with the derived tables, not left behind.
    //
    // Work-package text is DERIVED FROM CANON (D28), so a mutation that
    // rebuilt the model and left `doc_sections` alone would leave `intent
    // search` answering from the previous model -- silently, since a search
    // that finds nothing looks exactly like a search with no matches. The
    // file-derived sections are kept as they were; only the canon-derived ones
    // are recomputed, because nothing here read a file.
    let mut sections: Vec<crate::prose::DocSection> = self
      .canon
      .sections
      .iter()
      .filter(|s| s.owner_type != "work-package")
      .cloned()
      .collect();
    for thread in &next.threads {
      ingest::collect_wp_text(&self.project, &mut sections, thread);
    }

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
    let dates = self
      .store
      .commit_mutation(crate::store::Mutation {
        threads: &changed_threads,
        issues: &changed_issues,
        removed_threads: &removed_threads,
        removed_issues: &removed_issues,
        sections: &sections,
        envelope: &envelope,
        project_state,
      })
      .map_err(FacadeError::Store)?;
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
    let set = self.projection(&next, &changed_threads, &changed_issues)?;
    drop(changed_threads);
    drop(changed_issues);

    // Truth has landed. The files are a projection of it, so a failure here is
    // REPORTED AND RECOVERABLE rather than corrupting: `intent sync` writes
    // them again from the DB. Reporting it is not optional -- silently
    // returning success would leave the tree disagreeing with truth and say
    // nothing, which is the No Silent Errors case this whole apparatus exists
    // to refuse.
    // In-memory canon follows the STORE, not the files, so the next call in
    // this process builds on what actually happened either way.
    let projected = set.commit().map(Applied::keep);
    self.canon = next;
    projected.map_err(|cause| FacadeError::ViewsNotWritten { cause })
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
    if list == ListEdit::Suppressed || declared_list_edit(op) != Some(ListAction::Remove) {
      return Ok(Vec::new());
    }
    if self.st_show(id)?.attachments.is_empty() {
      return Ok(Vec::new());
    }
    let scope = SyncScope::Threads(vec![id.to_string()]);
    Ok(match self.sync_uncommitted(&scope)? {
      // **NOT FOLDED INTO SILENCE.** A close that says nothing is read as "no
      // uncommitted bytes" by anyone who knows it warns, so silence here IS the
      // clean bill of health `sync_uncommitted` refuses to let a caller print.
      None => vec![Note::UnsyncedUnknown],
      Some(found) if found.is_empty() => Vec::new(),
      Some(found) => vec![Note::UnsyncedAttachments(found)],
    })
  }

  /// Make a lifecycle op's declared edit to `.intentfiles` (AC-05.2).
  ///
  /// **IT RUNS AFTER `apply`, AND THE ORDER IS CHOSEN FOR ITS FAILURE MODE.**
  /// Both orders can be interrupted between the two writes, so the question is
  /// only which half-done state is survivable. Manifest first, store second,
  /// leaves the list saying NOT REALISED while the thread is still open -- and
  /// the next `organize` removes a live thread's files on the strength of it.
  /// Store first leaves a closed thread still listed, which is precisely what
  /// `--keep` asks for on purpose. **One order degrades into a supported
  /// outcome and the other into a deletion nobody asked for.**
  ///
  /// **THIS IS THE OPPOSITE ORDER FROM [`Facade::hydrate`], WHICH PINS FIRST,
  /// AND THE TWO ARE NOT IN TENSION.** Hydrate's ordering answers a different
  /// question -- that the pin must not be skipped when the files already exist
  /// -- and it has no second write to be interrupted between.
  fn edit_list(&self, op: &str, id: &str, list: ListEdit) -> Result<(), FacadeError> {
    if list == ListEdit::Suppressed {
      return Ok(());
    }
    let Some(action) = declared_list_edit(op) else {
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
      ListAction::Add => intentfiles::pin(&before, intentfiles::Sigil::SteelThread, id, None),
      ListAction::Remove => intentfiles::unpin(&before, intentfiles::Sigil::SteelThread, id),
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

/// The recorded state's name, for a refusal that says which one it already is.
fn state_name(state: &AcState) -> &'static str {
  match state {
    AcState::Computed => "computed",
    AcState::Unsatisfied => "unsatisfied",
    AcState::Satisfied { .. } => "satisfied",
    AcState::Descoped { .. } => "descoped",
    AcState::Withdrawn { .. } => "withdrawn",
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
      Self::Machine(_) | Self::Child(_) | Self::Derived(_) | Self::WholeBody { .. } => {
        UnsettableKind::Elsewhere
      }
      // Constitutive: the value IS the address, or the service owns the stamp.
      Self::Identity | Self::Stamped => UnsettableKind::Never,
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
      Self::NoRouteYet(what) => format!(
        "{what}, and there is no route on this surface today. Canon must already record the \
         attachment as opaque with its sidecar beside it -- `intent st attach` writes TEXT, and \
         `ingest` fills an opaque attachment's bytes from a sidecar canon already names rather \
         than from a file you drop in"
      ),
    }
  }
}

/// Which of the three refusals, if any, covers this field.
fn unsettable(entity: &AddrEntity, field: &str) -> Option<Unsettable> {
  if let Some((_, segment)) = CHILD_COLLECTIONS.iter().find(|(name, _)| *name == field) {
    return Some(Unsettable::Child(segment));
  }
  // **THE VERB SPELLINGS ARE DRIVEN FROM THE SHIPPED CLI, not recalled.** A
  // remedy naming a verb that does not exist is worse than no remedy: it costs
  // the operator a round trip and reads as authoritative while it does.
  match entity {
    AddrEntity::Thread { .. } => match field {
      "schema" | "id" => Some(Unsettable::Identity),
      "status" => Some(Unsettable::Machine(
        "intent st start|done|hold|resume|cancel|reopen|reinstate",
      )),
      // `completed` is deliberately NOT here -- see [`Unsettable::Stamped`].
      "created" => Some(Unsettable::Stamped),
      _ => None,
    },
    AddrEntity::Wp { .. } => match field {
      "seq" => Some(Unsettable::Identity),
      "status" => Some(Unsettable::Machine(
        "intent wp start|done|unstart|reopen|cancel|reinstate",
      )),
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
      "status" => Some(Unsettable::Machine("intent at green|red|na")),
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

fn find_thread_mut<'a>(canon: &'a mut Canon, id: &str) -> Result<&'a mut Thread, FacadeError> {
  canon
    .threads
    .iter_mut()
    .find(|t| t.id == id)
    .ok_or_else(|| FacadeError::NoSuchThread { id: id.to_string() })
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
