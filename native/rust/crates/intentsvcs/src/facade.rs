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

use serde_json::json;

use crate::contract::{self, Scope, Verdict};
use crate::event::{self, Envelope, Subject};
use crate::export::{self, ExportRefusal};
use crate::ingest::{self, Canon, IngestError};
use crate::model::{
  AcKind, AcState, AcceptanceTest, AtStatus, Criterion, Issue, TShirt, Thread, ThreadStatus,
  WorkPackage, WpStatus, to_canonical_json,
};
use crate::project::{Migration, Pending, Project};
use crate::store::{Store, StoreError};
use crate::transitions;
use crate::views::{self, RenderContext};
use crate::write_set::{Applied, WriteError, WriteSet};

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

#[derive(Debug, thiserror::Error)]
pub enum FacadeError {
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
  #[error("{ac} is already {state}")]
  ScopeUnchanged { ac: String, state: String },
  #[error("{ac} is in scope, so there is nothing to reinstate")]
  NotOffScope { ac: String },
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
  #[error("could not update the runtime store")]
  Store(#[from] StoreError),
  #[error("could not read the committed canon")]
  Ingest(#[from] IngestError),
  /// Something this build does not do -- NOT a fault in the project.
  #[error("{what} is not available in this build")]
  Unavailable { what: String },
  /// The event log's extract exists and is not readable as one.
  ///
  /// Its own variant rather than an ingest finding: history is the one thing
  /// nothing else can reconstruct, so "your history file is damaged" needs an
  /// action of its own and must not be reported as though a thread were
  /// malformed.
  #[error("the event log extract at {path} could not be read")]
  EventLogUnreadable { path: String, cause: String },
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
}

impl FacadeError {
  /// What the operator should DO. Every variant has one, and no two variants
  /// share a remedy text -- a remedy that fits two different causes is telling
  /// the operator to guess which one they hit (AC-04.4).
  pub fn remedy(&self) -> String {
    match self {
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
      Self::ScopeUnchanged { .. } => {
        "no action needed; the criterion is already in the state you asked for".to_string()
      }
      Self::NotOffScope { .. } => {
        "reinstate applies only to a descoped or withdrawn criterion".to_string()
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
          "give `{verb}` a reason. It is recorded on the entity as the reason for its CURRENT state, and in the event log as part of the decision, which is what lets anyone reconstruct why later"
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
      // Delegated, because the remedy DIFFERS by state: below the v2.19.0
      // floor it is the two-hop, and naming the v3 migrator there would send
      // half the operators who read it to a command that refuses them.
      Self::Unmigrated(pending) => pending.remedy(),
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
      Self::Store(cause) => cause.remedy(),
      Self::Ingest { .. } => {
        "fix the artefacts named above, then retry -- run `intent doctor` to list them".to_string()
      }
      // **Leads with what did NOT happen**, because that is the reader's actual
      // question. Someone who ran a migration and got an error wants to know
      // whether their estate was touched before they want to know why, and this
      // is the one case where the answer is "not at all".
      Self::Unavailable { .. } => {
        "nothing was read and nothing was written, so the project is exactly as it was -- keep using the version of Intent that wrote it until a build offers this".to_string()
      }
      Self::EventLogUnreadable { path, cause } => format!(
        "{cause}. Nothing recomputes history, so do NOT delete {path} to get past this -- repair the named line, from version control if the file is committed"
      ),
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
    }
  }

  /// The operator-facing rendering: the message, the FULL cause chain, and the
  /// remedy.
  ///
  /// The chain is walked rather than summarised. v2's habit of collapsing a
  /// failure to its outermost sentence is what made two different problems
  /// print the same line, and the whole point of typed errors is that they
  /// stop doing that.
  pub fn render(&self) -> String {
    let mut out = format!("error: {self}");
    let mut source = std::error::Error::source(self);
    while let Some(cause) = source {
      out.push_str(&format!("\n  caused by: {cause}"));
      source = cause.source();
    }
    out.push_str(&format!("\n  remedy: {}", self.remedy()));
    out
  }
}

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

/// The facade: a project, its store, and the canon it has loaded.
pub struct Facade {
  project: Project,
  store: Store,
  canon: Canon,
  ctx: FacadeContext,
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

  pub fn canon(&self) -> &Canon {
    &self.canon
  }

  pub fn store(&self) -> &Store {
    &self.store
  }

  fn render_ctx(&self) -> RenderContext<'_> {
    RenderContext {
      version: &self.ctx.version,
      todo_watermark: None,
    }
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
  /// It calls the same `contract_findings` the close gate calls, deliberately.
  /// A lint with its own copy of the rules is a lint that can say clean while
  /// the gate refuses, and an operator who cannot trust the lint runs the gate
  /// instead -- at which point the lint has no reason to exist.
  pub fn at_lint(&self, st: &str) -> Result<Vec<String>, FacadeError> {
    let thread = self.st_show(st)?;
    Ok(contract::contract_findings(
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
  pub fn sync_to_disk(&mut self) -> Result<usize, FacadeError> {
    let (threads, issues) = self.store.load_canon().map_err(FacadeError::Store)?;
    let sections = self.store.doc_sections().map_err(FacadeError::Store)?;
    let canon = Canon {
      threads,
      issues,
      sections,
    };
    let all_threads: Vec<&Thread> = canon.threads.iter().collect();
    let all_issues: Vec<&Issue> = canon.issues.iter().collect();
    let mut set = self.projection(&canon, &all_threads, &all_issues)?;
    // History travels only if something writes it out. Every other entity in
    // this set can be re-derived from a file that is already there; this one
    // cannot be re-derived from anything.
    self.add_event_log(&mut set)?;
    set.commit()?.keep();
    let count = canon.threads.len();
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
  pub fn sync_from_disk(&mut self) -> Result<usize, FacadeError> {
    let canon = ingest::resync(&self.project, &mut self.store)?;
    self.restore_event_log()?;
    let all_threads: Vec<&Thread> = canon.threads.iter().collect();
    let all_issues: Vec<&Issue> = canon.issues.iter().collect();
    let set = self.projection(&canon, &all_threads, &all_issues)?;
    set.commit()?.keep();
    let count = canon.threads.len();
    self.canon = canon;
    Ok(count)
  }

  /// Take into the log whatever the extract carries and the store does not.
  ///
  /// **The one place the destructive direction is NOT destructive, and it has
  /// to be.** `sync_from_disk` replaces the store from the files because for
  /// every other entity the files are a faithful copy. The event log is
  /// append-only (D15) and nothing derives it, so a wipe-and-reload would
  /// destroy exactly the history the extract had not caught up with -- a
  /// restore from yesterday's clone would silently delete today.
  ///
  /// Merging on the ULID makes it idempotent and makes two machines' logs a
  /// union rather than a conflict, so restoring an older extract over a newer
  /// log adds nothing and loses nothing.
  ///
  /// An absent file is not an error: a project that has never synced out has no
  /// extract of its history yet, and refusing here would make the first sync of
  /// an old project impossible.
  fn restore_event_log(&mut self) -> Result<usize, FacadeError> {
    let path = self.project.events_jsonl();
    let text = match std::fs::read_to_string(&path) {
      Ok(text) => text,
      Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(0),
      Err(source) => {
        return Err(FacadeError::Ingest(ingest::IngestError::Io {
          path: self.project.relative(&path),
          source,
        }));
      }
    };
    let incoming = event::from_jsonl(&text).map_err(|e| FacadeError::EventLogUnreadable {
      path: self.project.relative(&path),
      cause: e.to_string(),
    })?;
    let have = self.store.events().map_err(FacadeError::Store)?;
    let missing = event::merge(&have, &incoming);
    for envelope in &missing {
      // **`restore_event`, never `append_event`** (D42). These already
      // happened; the extract carries when. Recording them as happening now
      // would rewrite the whole of an older clone's history to the moment
      // someone restored it -- and every stamp would look perfectly valid.
      self
        .store
        .restore_event(envelope)
        .map_err(FacadeError::Store)?;
    }
    Ok(missing.len())
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
  pub fn sync_overwrite(&self) -> Result<Vec<String>, FacadeError> {
    let (stored_threads, stored_issues) = self.store.load_canon().map_err(FacadeError::Store)?;
    let on_disk = ingest::read(&self.project)?;
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
  pub fn export(&self, format: Option<&str>) -> Result<String, FacadeError> {
    let (threads, issues) = self.store.load_canon().map_err(FacadeError::Store)?;
    let events = self.store.events().map_err(FacadeError::Store)?;
    let bundle = export::Bundle::new(&self.ctx.project_id, threads, issues, events);
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
    })
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
    for view in views::render_all(&self.project, canon, &self.render_ctx()) {
      set.add(view.path, view.content);
    }
    Ok(set)
  }

  /// Add the event log's file form to a write set (D34, AC-02.6).
  ///
  /// **Separate from [`Facade::projection`] because it is a different kind of
  /// artefact, and conflating them would be a real error rather than an
  /// untidiness.** A projection re-derives files from the canon it is handed,
  /// so it is correct to run it over a SUBSET -- a single mutated thread -- and
  /// the mutation path does exactly that. The event log has no subset: it is
  /// the whole log or a truncated one, and rendering it during a per-thread
  /// write would rewrite the file down to whatever that call happened to know
  /// about.
  ///
  /// It therefore joins only the whole-estate direction, and it goes through
  /// the same [`WriteSet`] so the extract lands atomically or not at all
  /// (AC-04.1) -- a partial history is worse than a stale one.
  fn add_event_log(&self, set: &mut WriteSet) -> Result<(), FacadeError> {
    let events = self.store.events().map_err(FacadeError::Store)?;
    set.add(
      self.project.events_jsonl(),
      event::to_jsonl(&events).map_err(|e| FacadeError::Store(StoreError::Serde(e)))?,
    );
    Ok(())
  }

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
  pub fn ingest_from_md(project: &Project) -> Result<crate::ingest::Canon, FacadeError> {
    ingest::from_md(project).map_err(|e| match e {
      IngestError::Unavailable { what } => FacadeError::Unavailable { what },
      other => FacadeError::Ingest(other),
    })
  }

  pub fn doctor(
    project: &Project,
    ctx: &FacadeContext,
    store: Option<&crate::store::Store>,
  ) -> crate::doctor::Report {
    crate::doctor::diagnose(
      project,
      &RenderContext {
        version: &ctx.version,
        todo_watermark: None,
      },
      store,
    )
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
    let id = self.next_thread_id();
    if self.canon.threads.iter().any(|t| t.id == id) {
      return Err(FacadeError::ThreadExists { id });
    }
    let thread = Thread {
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
    Ok(id)
  }

  /// Accept a thread out of triage and into the backlog.
  pub fn st_triage(&mut self, id: &str) -> Result<(), FacadeError> {
    self.set_thread_status(id, ThreadStatus::NotStarted, "st.triage", None)
  }

  pub fn st_start(&mut self, id: &str) -> Result<(), FacadeError> {
    self.set_thread_status(id, ThreadStatus::Wip, "st.start", None)
  }

  /// Pause a thread, recording why.
  ///
  /// **`Hold` was in the vocabulary for two major versions with no verb that
  /// set it** -- v2 recognised it in its status filter and reached it only by
  /// hand-editing frontmatter, which is the defect class hv ruled on, sitting
  /// in the tool's own status enum.
  pub fn st_hold(&mut self, id: &str, reason: &str) -> Result<(), FacadeError> {
    self.set_thread_status(id, ThreadStatus::Hold, "st.hold", Some(reason))
  }

  /// Resume a held thread. **The hold reason is cleared**, because it described
  /// a condition that has ended -- see [`Thread::status_reason`].
  ///
  /// [`Thread::status_reason`]: crate::model::Thread::status_reason
  pub fn st_resume(&mut self, id: &str) -> Result<(), FacadeError> {
    self.set_thread_status(id, ThreadStatus::Wip, "st.resume", None)
  }

  /// Close a thread. Consults the close gate first -- the single authority, so
  /// there is no path that closes without it.
  pub fn st_done(&mut self, id: &str) -> Result<(), FacadeError> {
    let verdict = self.gate(id, Scope::Thread)?;
    if !verdict.is_pass() {
      return Err(FacadeError::GateBlocked {
        scope: id.to_string(),
        verdict: verdict.line(id),
      });
    }
    self.set_thread_status(id, ThreadStatus::Completed, "st.done", None)
  }

  /// Reopen a completed thread.
  ///
  /// **The ratified machines have no terminal states**, and this is one of the
  /// two exits that makes that true. A thread whose contract grows after it
  /// closed was previously repairable only by editing the file the CLI exists
  /// to own -- and the gate then kept saying PASS against a contract that had
  /// moved underneath it.
  pub fn st_reopen(&mut self, id: &str, reason: &str) -> Result<(), FacadeError> {
    self.set_thread_status(id, ThreadStatus::Wip, "st.reopen", Some(reason))
  }

  /// Bring a cancelled thread back, to the backlog rather than to where it was.
  ///
  /// It lands on `not-started` deliberately: a thread that was cancelled mid-
  /// flight has had its work overtaken, and resuming it as `wip` would assert
  /// a continuity nobody checked.
  pub fn st_reinstate(&mut self, id: &str, reason: &str) -> Result<(), FacadeError> {
    self.set_thread_status(id, ThreadStatus::NotStarted, "st.reinstate", Some(reason))
  }

  pub fn st_cancel(&mut self, id: &str, reason: &str) -> Result<(), FacadeError> {
    self.set_thread_status(id, ThreadStatus::Cancelled, "st.cancel", Some(reason))
  }

  fn set_thread_status(
    &mut self,
    id: &str,
    status: ThreadStatus,
    op: &'static str,
    reason: Option<&str>,
  ) -> Result<(), FacadeError> {
    let from = self.st_show(id)?.status;
    Self::check_transition("Thread", "status", op, &crate::model::enum_str(&from), id)?;
    let reason = Self::check_reason("Thread", "status", op, reason)?;
    let mut next = self.canon.clone();
    let thread = find_thread_mut(&mut next, id)?;
    thread.status = status;
    thread.status_reason = reason.clone();
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
    )
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
      seq,
      title: title.to_string(),
      scope,
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

  pub fn wp_start(&mut self, st: &str, seq: u32) -> Result<(), FacadeError> {
    self.set_wp_status(st, seq, WpStatus::Wip, "wp.start", None)
  }

  /// Put a work package back to `not-started` -- the inverse of `wp start`,
  /// for one started by mistake or on the wrong thread.
  pub fn wp_unstart(&mut self, st: &str, seq: u32) -> Result<(), FacadeError> {
    self.set_wp_status(st, seq, WpStatus::NotStarted, "wp.unstart", None)
  }

  /// Close a work package, gated on its own scope.
  pub fn wp_done(&mut self, st: &str, seq: u32) -> Result<(), FacadeError> {
    let label = format!("{st}/{seq:02}");
    let verdict = self.gate(st, Scope::WorkPackage(seq))?;
    if !verdict.is_pass() {
      return Err(FacadeError::GateBlocked {
        scope: label.clone(),
        verdict: verdict.line(&label),
      });
    }
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
  pub fn wp_reopen(&mut self, st: &str, seq: u32, reason: &str) -> Result<(), FacadeError> {
    self.set_wp_status(st, seq, WpStatus::Wip, "wp.reopen", Some(reason))
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
  pub fn wp_rescope(&mut self, st: &str, seq: u32, scope: TShirt) -> Result<(), FacadeError> {
    let from = self
      .st_show(st)?
      .wps
      .iter()
      .find(|w| w.seq == seq)
      .map(|w| w.scope)
      .ok_or_else(|| FacadeError::NoSuchWorkPackage {
        st: st.to_string(),
        seq,
      })?;
    let mut next = self.canon.clone();
    let wp = find_thread_mut(&mut next, st)?
      .wps
      .iter_mut()
      .find(|w| w.seq == seq)
      .ok_or_else(|| FacadeError::NoSuchWorkPackage {
        st: st.to_string(),
        seq,
      })?;
    wp.scope = scope;
    self.apply(
      "wp.rescope",
      Subject {
        kind: "wp".to_string(),
        id: format!("{st}/{seq:02}"),
      },
      json!({"from": crate::model::enum_str(&from), "to": crate::model::enum_str(&scope)}),
      next,
    )
  }

  fn set_wp_status(
    &mut self,
    st: &str,
    seq: u32,
    status: WpStatus,
    op: &'static str,
    reason: Option<&str>,
  ) -> Result<(), FacadeError> {
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
    Self::check_transition(
      "WorkPackage",
      "status",
      op,
      &crate::model::enum_str(&from),
      &label,
    )?;
    let reason = Self::check_reason("WorkPackage", "status", op, reason)?;
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
    wp.status_reason = reason.clone();
    self.apply(
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
  pub fn ac_satisfy(&mut self, st: &str, ac: &str, evidence: &str) -> Result<(), FacadeError> {
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
  pub fn ac_unsatisfy(&mut self, st: &str, ac: &str) -> Result<(), FacadeError> {
    let criterion = self.criterion(st, ac)?;
    if criterion.kind != AcKind::NonTest {
      return Err(FacadeError::ComputedSatisfaction { ac: ac.to_string() });
    }
    if !matches!(criterion.state, AcState::Satisfied { .. }) {
      return Err(FacadeError::NotSatisfied { ac: ac.to_string() });
    }
    self.set_ac_state(st, ac, AcState::Unsatisfied, "ac.unsatisfy", json!({}))
  }

  pub fn ac_descope(
    &mut self,
    st: &str,
    ac: &str,
    to: &str,
    by: Option<&str>,
    reason: Option<&str>,
  ) -> Result<(), FacadeError> {
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
  ) -> Result<(), FacadeError> {
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
  pub fn ac_reinstate(&mut self, st: &str, ac: &str) -> Result<(), FacadeError> {
    let criterion = self.criterion(st, ac)?;
    let entry = AcState::entry(criterion.kind);
    match &criterion.state {
      AcState::Withdrawn { .. } => self.set_ac_state(st, ac, entry, "ac.reinstate", json!({})),
      AcState::Descoped { .. } => Err(FacadeError::WrongOffScopeState {
        ac: ac.to_string(),
        actual: "descoped".to_string(),
        wanted: "withdrawn".to_string(),
        verb: "rescope".to_string(),
      }),
      _ => Err(FacadeError::NotOffScope { ac: ac.to_string() }),
    }
  }

  /// Undo a DESCOPE. The mirror of [`Facade::ac_reinstate`], refusing a
  /// withdrawn criterion the same way.
  pub fn ac_rescope(&mut self, st: &str, ac: &str) -> Result<(), FacadeError> {
    let criterion = self.criterion(st, ac)?;
    let entry = AcState::entry(criterion.kind);
    match &criterion.state {
      AcState::Descoped { .. } => self.set_ac_state(st, ac, entry, "ac.rescope", json!({})),
      AcState::Withdrawn { .. } => Err(FacadeError::WrongOffScopeState {
        ac: ac.to_string(),
        actual: "withdrawn".to_string(),
        wanted: "descoped".to_string(),
        verb: "reinstate".to_string(),
      }),
      _ => Err(FacadeError::NotOffScope { ac: ac.to_string() }),
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
  ) -> Result<(), FacadeError> {
    let current = &self.criterion(st, ac)?.state;
    if *current == state {
      return Err(FacadeError::ScopeUnchanged {
        ac: ac.to_string(),
        state: state_name(&state).to_string(),
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
    self.apply(
      op,
      Subject {
        kind: "ac".to_string(),
        id: format!("{st}/{ac}"),
      },
      payload,
      next,
    )
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
  pub fn at_set(&mut self, st: &str, at: &str, status: AtStatus) -> Result<(), FacadeError> {
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
    let mut next = self.canon.clone();
    find_test_mut(&mut next, st, at)?.status = status;
    self.apply(
      "at.set",
      Subject {
        kind: "at".to_string(),
        id: format!("{st}/{at}"),
      },
      json!({"from": crate::model::enum_str(&from), "to": crate::model::enum_str(&status)}),
      next,
    )
  }

  pub fn at_list(&self, st: &str) -> Result<&[AcceptanceTest], FacadeError> {
    Ok(&self.st_show(st)?.tests)
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
  fn apply(
    &mut self,
    op: &str,
    subject: Subject,
    payload: serde_json::Value,
    mut next: Canon,
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
      })
      .map_err(FacadeError::Store)?;
    drop(changed_threads);
    drop(changed_issues);

    // **THE DATES COME BACK FROM THE WRITE, AND THE FILES ARE RENDERED FROM
    // WHAT LANDED** (D42). `st new` handed in an empty `created`; SQLite filled
    // it as part of the INSERT. Rendering `thread.json` before this point would
    // write the empty string into the extract -- truth and its projection
    // disagreeing on the one field neither of them can recompute.
    for stamped in dates {
      let thread = find_thread_mut(&mut next, &stamped.id)?;
      thread.created = stamped.created;
      thread.completed = stamped.completed;
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

  fn next_thread_id(&self) -> String {
    let highest = self
      .canon
      .threads
      .iter()
      .filter_map(|t| t.id.strip_prefix("ST"))
      .filter_map(|n| n.parse::<u32>().ok())
      .max()
      .unwrap_or(0);
    format!("ST{:04}", highest + 1)
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
fn slugify(title: &str) -> String {
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
  while out.ends_with('-') {
    out.pop();
  }
  out.chars().take(48).collect::<String>()
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

  /// Every variant has a remedy, and no two share one. A remedy that fits two
  /// causes is telling the operator to guess which one they hit.
  #[test]
  fn no_two_error_variants_share_a_remedy() {
    let errors = vec![
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
      FacadeError::ScopeUnchanged {
        ac: "AC-03.1".to_string(),
        state: "descoped".to_string(),
      },
      FacadeError::NotOffScope {
        ac: "AC-03.1".to_string(),
      },
    ];
    let mut remedies: Vec<String> = errors.iter().map(FacadeError::remedy).collect();
    let before = remedies.len();
    remedies.sort();
    remedies.dedup();
    assert_eq!(before, remedies.len(), "two variants share a remedy text");
  }
}
