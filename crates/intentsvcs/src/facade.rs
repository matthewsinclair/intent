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
use crate::project::Project;
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

/// The facade: a project, its store, and the canon it has loaded.
pub struct Facade {
  project: Project,
  store: Store,
  canon: Canon,
  ctx: FacadeContext,
}

impl Facade {
  /// Open a project, loading and validating its whole canon.
  pub fn open(project: Project, ctx: FacadeContext) -> Result<Self, FacadeError> {
    let mut store = Store::open(&project.db_path()).map_err(FacadeError::Store)?;
    let canon = ingest::load(&project, &mut store)?;
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

  pub fn st_list(&self) -> Vec<&Thread> {
    let mut threads: Vec<&Thread> = self.canon.threads.iter().collect();
    threads.sort_by(|a, b| a.id.cmp(&b.id));
    threads
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
      std::slice::from_ref(&id),
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
      &[id.to_string()],
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
      &[st.to_string()],
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
      &[st.to_string()],
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
      &[st.to_string()],
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
  pub fn ac_reinstate(&mut self, st: &str, ac: &str) -> Result<(), FacadeError> {
    if matches!(self.criterion(st, ac)?.scope, AcScope::InScope) {
      return Err(FacadeError::NotOffScope { ac: ac.to_string() });
    }
    self.set_scope(st, ac, AcScope::InScope, "ac.reinstate", json!({}))
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
    find_criterion_mut(&mut next, st, ac)?.scope = scope;
    self.apply(
      op,
      Subject {
        kind: "ac".to_string(),
        id: format!("{st}/{ac}"),
      },
      payload,
      next,
      &[st.to_string()],
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
      &[st.to_string()],
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
    touched: &[String],
  ) -> Result<(), FacadeError> {
    let mut set = WriteSet::new();
    for id in touched {
      if let Some(thread) = next.threads.iter().find(|t| &t.id == id) {
        set.add(
          self.project.thread_json(id),
          to_canonical_json(thread).map_err(|e| FacadeError::Store(StoreError::Serde(e)))?,
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
    let db = self
      .store
      .rebuild(&next.threads, &next.issues)
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
