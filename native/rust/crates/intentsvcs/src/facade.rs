//! The intentsvcs facade -- the one surface every skin calls (design.md D06).
//!
//! The clap layer, the GraphQL layer and the MCP layer are all thin
//! coordinators over this: parse, call, render. Nothing above this module
//! touches the DB or the file canon, which is what makes the two entry skins
//! incapable of drifting apart.
//!
//! **Every mutation is all-or-nothing across three stores** -- committed canon,
//! generated views, and the DB -- and the order is forced by D01. Canon is
//! durable truth and the DB is rebuildable from it, so files land first and
//! the DB second: a DB failure after a good file write is repaired by
//! rebuilding, where a file failure after a DB write would leave the DB
//! asserting something the canon never said, with nothing to rebuild from.
//!
//! **The facade has no clock.** Dates arrive from the caller in
//! [`FacadeContext::today`]. That is not the renderer's no-clock law (D23) --
//! a mutation genuinely happens at a time -- but it keeps every verb a pure
//! function of its inputs, which is what makes them testable without freezing
//! time. The event log is the one place a real timestamp is minted, because an
//! event log that did not record when things happened would not be one.

use serde_json::json;

use crate::contract::{self, Scope, Verdict};
use crate::event::{Envelope, Subject};
use crate::ingest::{self, Canon, IngestError};
use crate::model::{
  AcKind, AcScope, AcceptanceTest, AtStatus, Criterion, TShirt, Thread, ThreadStatus, WorkPackage,
  WpStatus, to_canonical_json,
};
use crate::project::{Migration, Pending, Project};
use crate::store::{Store, StoreError};
use crate::views::{self, RenderContext};
use crate::write_set::{WriteError, WriteSet};

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
  /// Today, ISO 8601. Supplied by the caller; the facade owns no clock.
  pub today: String,
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
  // NOT `#[source]`-bearing and NOT constructed from anything: the whole value
  // of this variant is that it carries the EVIDENCE, so that a refusal can be
  // told apart from an empty project by reading it.
  #[error("{0}")]
  Unmigrated(Pending),
  #[error("could not write the project files")]
  Write(#[from] WriteError),
  #[error("could not update the runtime store")]
  Store(#[from] StoreError),
  #[error("could not read the committed canon")]
  Ingest(#[from] IngestError),
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
      Self::Store { .. } => {
        "the runtime store is rebuildable: delete `intent/.cache/intent.db` and retry".to_string()
      }
      Self::Ingest { .. } => {
        "fix the artefacts named above, then retry -- run `intent doctor` to list them".to_string()
      }
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
          state: match &c.scope {
            AcScope::Descoped { to, .. } => format!("descoped-to: {to}"),
            AcScope::Withdrawn { reason, .. } => format!("withdrawn: {reason}"),
            AcScope::InScope => format!(
              "satisfied: {}",
              if contract::ac_state(thread, c) == contract::AcState::Satisfied {
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
  pub fn sync(&mut self) -> Result<usize, FacadeError> {
    let canon = ingest::resync(&self.project, &mut self.store)?;
    let views = views::write_all(&self.project, &canon, &self.render_ctx()).map_err(|e| {
      FacadeError::Write(WriteError::Io {
        path: self.project.root().display().to_string(),
        source: e,
      })
    })?;
    let count = canon.threads.len();
    self.canon = canon;
    let _ = views;
    Ok(count)
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
  pub fn doctor(project: &Project, ctx: &FacadeContext) -> crate::doctor::Report {
    crate::doctor::diagnose(
      project,
      &RenderContext {
        version: &ctx.version,
        todo_watermark: None,
      },
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
      status: ThreadStatus::NotStarted,
      created: self.ctx.today.clone(),
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

  pub fn st_start(&mut self, id: &str) -> Result<(), FacadeError> {
    self.set_thread_status(id, ThreadStatus::Wip, "st.start")
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
    self.set_thread_status(id, ThreadStatus::Completed, "st.done")
  }

  pub fn st_cancel(&mut self, id: &str) -> Result<(), FacadeError> {
    self.set_thread_status(id, ThreadStatus::Cancelled, "st.cancel")
  }

  fn set_thread_status(
    &mut self,
    id: &str,
    status: ThreadStatus,
    op: &str,
  ) -> Result<(), FacadeError> {
    let from = self.st_show(id)?.status;
    let mut next = self.canon.clone();
    let thread = find_thread_mut(&mut next, id)?;
    thread.status = status;
    thread.completed = match status {
      ThreadStatus::Completed | ThreadStatus::Cancelled => Some(self.ctx.today.clone()),
      _ => None,
    };
    self.apply(
      op,
      Subject {
        kind: "thread".to_string(),
        id: id.to_string(),
      },
      json!({"from": crate::model::enum_str(&from), "to": crate::model::enum_str(&status)}),
      next,
    )
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
    self.set_wp_status(st, seq, WpStatus::Wip, "wp.start")
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
    self.set_wp_status(st, seq, WpStatus::Done, "wp.done")
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
    op: &str,
  ) -> Result<(), FacadeError> {
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
    self.apply(
      op,
      Subject {
        kind: "wp".to_string(),
        id: format!("{st}/{seq:02}"),
      },
      json!({"from": crate::model::enum_str(&from), "to": crate::model::enum_str(&status)}),
      next,
    )
  }

  // -------------------------------------------------------------------------
  // Acceptance criteria -- the four states (issue 0013)
  // -------------------------------------------------------------------------

  /// Mark a NON-TEST criterion satisfied, with its evidence.
  ///
  /// Refuses on a test-backed criterion, because storing that answer would be
  /// the double truth data-model.md forbids: the model has no field for it and
  /// the gate computes it from covering green ATs.
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
    let mut next = self.canon.clone();
    let c = find_criterion_mut(&mut next, st, ac)?;
    c.satisfied = Some(true);
    c.evidence = Some(evidence.to_string());
    self.apply(
      "ac.satisfy",
      Subject {
        kind: "ac".to_string(),
        id: format!("{st}/{ac}"),
      },
      json!({"evidence": evidence}),
      next,
    )
  }

  pub fn ac_descope(
    &mut self,
    st: &str,
    ac: &str,
    to: &str,
    by: Option<&str>,
    reason: Option<&str>,
  ) -> Result<(), FacadeError> {
    self.set_scope(
      st,
      ac,
      AcScope::Descoped {
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
    self.set_scope(
      st,
      ac,
      AcScope::Withdrawn {
        reason: reason.to_string(),
        by: by.map(str::to_string),
      },
      "ac.withdraw",
      json!({"reason": reason}),
    )
  }

  /// Bring a descoped or withdrawn criterion back into scope.
  /// Undo a WITHDRAWAL: back in scope, unsatisfied.
  ///
  /// It refuses a descoped criterion and names `rescope` instead, because v2
  /// does (`bin/intent_acceptance:1246`) and because the two are genuinely
  /// different acts: a descoped requirement still exists somewhere else, and a
  /// withdrawn one does not exist at all. Treating them as one verb would make
  /// the tool answer "done" to a question it had not been asked.
  pub fn ac_reinstate(&mut self, st: &str, ac: &str) -> Result<(), FacadeError> {
    match self.criterion(st, ac)?.scope {
      AcScope::Withdrawn { .. } => {
        self.set_scope(st, ac, AcScope::InScope, "ac.reinstate", json!({}))
      }
      AcScope::Descoped { .. } => Err(FacadeError::WrongOffScopeState {
        ac: ac.to_string(),
        actual: "descoped".to_string(),
        wanted: "withdrawn".to_string(),
        verb: "rescope".to_string(),
      }),
      AcScope::InScope => Err(FacadeError::NotOffScope { ac: ac.to_string() }),
    }
  }

  /// Undo a DESCOPE: back in scope, unsatisfied. The mirror of
  /// [`Facade::ac_reinstate`], refusing a withdrawn criterion the same way.
  pub fn ac_rescope(&mut self, st: &str, ac: &str) -> Result<(), FacadeError> {
    match self.criterion(st, ac)?.scope {
      AcScope::Descoped { .. } => self.set_scope(st, ac, AcScope::InScope, "ac.rescope", json!({})),
      AcScope::Withdrawn { .. } => Err(FacadeError::WrongOffScopeState {
        ac: ac.to_string(),
        actual: "withdrawn".to_string(),
        wanted: "descoped".to_string(),
        verb: "reinstate".to_string(),
      }),
      AcScope::InScope => Err(FacadeError::NotOffScope { ac: ac.to_string() }),
    }
  }

  /// Reopen a non-test criterion: unsatisfied, and its evidence cleared.
  ///
  /// **The inverse `ac.satisfy` never had.** hv ruled on this instance
  /// directly (D32, AC-04.6): satisfy was a one-way door, so a verifier whose
  /// evidence proved incomplete had to hand-edit `acceptance.md` -- the file
  /// this command exists to own. A state that can be entered and not left is a
  /// missing mutation, not a missing flag.
  ///
  /// **The evidence goes with it, and that is the whole design content.**
  /// Clearing satisfaction while leaving the evidence string behind would
  /// produce a criterion that reads as unsatisfied and still cites the proof
  /// that was withdrawn -- a worse lie than the one-way door, because it looks
  /// like a record. v2 takes the same position for the same reason on a scope
  /// change: "re-satisfying is a fresh, stated act, not something inherited"
  /// (bin/intent_acceptance:1252-1255).
  pub fn ac_unsatisfy(&mut self, st: &str, ac: &str) -> Result<(), FacadeError> {
    let criterion = self.criterion(st, ac)?;
    if criterion.kind != AcKind::NonTest {
      return Err(FacadeError::ComputedSatisfaction { ac: ac.to_string() });
    }
    if criterion.satisfied != Some(true) {
      return Err(FacadeError::NotSatisfied { ac: ac.to_string() });
    }
    let mut next = self.canon.clone();
    let c = find_criterion_mut(&mut next, st, ac)?;
    c.satisfied = None;
    c.evidence = None;
    self.apply(
      "ac.unsatisfy",
      Subject {
        kind: "ac".to_string(),
        id: format!("{st}/{ac}"),
      },
      json!({}),
      next,
    )
  }

  /// A criterion that has left scope refuses every verb that would record
  /// something about its satisfaction, and the refusal names the undo.
  fn refuse_if_off_scope(criterion: &Criterion, ac: &str, verb: &str) -> Result<(), FacadeError> {
    let (state, undo) = match &criterion.scope {
      AcScope::InScope => return Ok(()),
      AcScope::Descoped { to, .. } => (format!("descoped to {to}"), "rescope"),
      AcScope::Withdrawn { .. } => ("withdrawn".to_string(), "reinstate"),
    };
    Err(FacadeError::OffScope {
      ac: ac.to_string(),
      state,
      undo: undo.to_string(),
      verb: verb.to_string(),
    })
  }

  fn set_scope(
    &mut self,
    st: &str,
    ac: &str,
    scope: AcScope,
    op: &str,
    payload: serde_json::Value,
  ) -> Result<(), FacadeError> {
    if self.criterion(st, ac)?.scope == scope {
      return Err(FacadeError::ScopeUnchanged {
        ac: ac.to_string(),
        state: scope_name(&scope).to_string(),
      });
    }
    let mut next = self.canon.clone();
    let c = find_criterion_mut(&mut next, st, ac)?;
    c.scope = scope;
    // A SCOPE CHANGE CLEARS SATISFACTION, in both directions. v2 does this for
    // all four verbs -- `ac_strip_tail_expr` removes `evidence:` and
    // `satisfied:` along with the scope markers, and descope calls it on the
    // way out (bin/intent_acceptance:1191) exactly as rescope does on the way
    // back (:1250). v3 changed `scope` alone, so a satisfied criterion that
    // was descoped and rescoped came back still carrying the evidence for a
    // claim that had been withdrawn -- while the verb's own help string, in
    // v2's words and in the dispatch table, said "back in scope, unsatisfied".
    //
    // This is also the second edge out of `satisfied` (transitions.rs), and
    // the reason one verb declares an edge on two fields.
    c.satisfied = None;
    c.evidence = None;
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
    next: Canon,
  ) -> Result<(), FacadeError> {
    let mut set = WriteSet::new();

    // What gets written is DIFFED, not declared. The caller used to hand in a
    // list of touched ids, which made "the mutation did not persist" reachable
    // by naming the wrong id -- a silent failure, since the DB and the return
    // value would both say it worked. Comparing against the loaded canon
    // cannot forget, and it generalises to issues for free.
    for thread in &next.threads {
      let unchanged = self
        .canon
        .threads
        .iter()
        .any(|current| current.id == thread.id && current == thread);
      if !unchanged {
        set.add(
          self.project.thread_json(&thread.id),
          to_canonical_json(thread).map_err(|e| FacadeError::Store(StoreError::Serde(e)))?,
        );
      }
    }
    for issue in &next.issues {
      let unchanged = self
        .canon
        .issues
        .iter()
        .any(|current| current.number == issue.number && current == issue);
      if !unchanged {
        set.add(
          self.project.issue_json(issue.number),
          to_canonical_json(issue).map_err(|e| FacadeError::Store(StoreError::Serde(e)))?,
        );
      }
    }
    for view in views::render_all(&self.project, &next, &self.render_ctx()) {
      set.add(view.path, view.content);
    }

    // Files first (D01): the DB is rebuildable from canon, canon is not
    // rebuildable from the DB.
    let applied = set.commit()?;

    let envelope = Envelope::new(
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

    let db = self
      .store
      .rebuild(&next.threads, &next.issues)
      .and_then(|()| self.store.replace_doc_sections(&sections))
      .and_then(|()| self.store.append_event(&envelope));

    match db {
      Ok(()) => {
        applied.keep();
        self.canon = next;
        Ok(())
      }
      Err(e) => {
        applied.rollback()?;
        Err(FacadeError::Store(e))
      }
    }
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

fn scope_name(scope: &AcScope) -> &'static str {
  match scope {
    AcScope::InScope => "in scope",
    AcScope::Descoped { .. } => "descoped",
    AcScope::Withdrawn { .. } => "withdrawn",
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
