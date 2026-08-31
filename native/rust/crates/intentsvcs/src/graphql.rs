//! The GraphQL SDL face (AC-02.2) -- the third of the three faces the model
//! types generate (design.md, "one master, three faces").
//!
//! The SDL is EXPORTED from the same authored master the JSON Schema face is
//! exported from: the model types carry `SimpleObject` / `Enum` derives beside
//! their schemars ones, and [`sdl`] renders the schema those derives describe.
//! Nothing here restates the model, so a field added to a type reaches this
//! face without anyone remembering to update it.
//!
//! **The one projection, and why it exists.** GraphQL's type system cannot
//! express [`AcScope`] -- a tagged enum whose variants carry different fields.
//! There is no derive for it: `Enum` takes unit variants only, and `Union`
//! members must be objects, which would mean reshaping the master to suit one
//! face. So [`AcScopeView`] flattens it exactly the way serde's
//! `#[serde(tag = "state")]` already does, which is the shape the JSON face is
//! committed to -- one wire form, described twice, in two type systems.
//!
//! A hand-written projection can drift from the master silently, which is the
//! whole reason it is the only one. `tests/graphql_face_agrees.rs` holds it
//! shut: the projection must carry exactly the fields the serde form carries
//! for every variant, and every property of the JSON Schema face must appear on
//! its own type in the SDL. Neither is a convention anyone has to remember.
//!
//! **The Query root is WIRED, reads only, and executes only in intentd.** The
//! four resolvers -- `thread`, `threads`, `issue`, `issues` -- answer from a
//! [`Snapshot`] taken through the facade's own accessors ([`Facade::st_list`],
//! [`Facade::issue_list`]) by [`Facade::graphql`], the one entry point; a
//! request built without that snapshot is refused by name rather than answered
//! empty. **Mutations are OUT of 3.0.x and `EmptyMutation` is the
//! enforcement**: a mutation document fails the schema's own validation, so
//! the reads-only bound is a property of the face and not a check somewhere in
//! front of it. The bound was ruled by vc under hv's pen, 2026-08-31
//! (`ratified_in: "vc, 2026-08-31, under hv's pen granted 2026-08-22;
//! hv/wip.md, the escape-hatch entry"`): a GraphQL mutation path would be a
//! second home for every mutation the MCP `serve()` roster already maps,
//! arriving in the same release as the first.
//!
//! **NOTHING HERE DRIVES A FUTURE.** [`Facade::graphql`] returns one; the store
//! thread in intentd blocks on it with tokio's own handle. The CLI links no
//! runtime and must not grow one for this -- when no daemon is answering, both
//! its faces refuse and name `intent daemon start` rather than executing here.

use std::future::Future;
use std::sync::OnceLock;

use async_graphql::{
  Context, EmptyMutation, EmptySubscription, Enum, Error, Object, Request, Schema, SimpleObject,
  Variables,
};
use serde_json::Value;

use crate::facade::Facade;
use crate::model::{AcState, Criterion, Issue, Thread};

/// Rendered by [`crate::faces::faces`] as the committed `schema.graphql`.
///
/// async-graphql indents with tabs; the house rule is two spaces in every
/// language, and this artefact is committed and reviewed like any other. Only
/// LEADING tabs are converted -- a tab inside a description string is content,
/// not indentation.
pub fn sdl() -> String {
  let mut out = String::new();
  for line in schema().sdl().lines() {
    let depth = line.len() - line.trim_start_matches('\t').len();
    out.push_str(&"  ".repeat(depth));
    out.push_str(line.trim_start_matches('\t'));
    out.push('\n');
  }
  out
}

/// The schema the SDL face describes and the one intentd executes.
///
/// Built once: a `Schema` is a handle over shared registry state, so cloning
/// it is cheap and rebuilding the registry per request would have been the
/// only cost of executing per request.
pub fn schema() -> Schema<Query, EmptyMutation, EmptySubscription> {
  static SCHEMA: OnceLock<Schema<Query, EmptyMutation, EmptySubscription>> = OnceLock::new();
  SCHEMA
    .get_or_init(|| Schema::build(Query, EmptyMutation, EmptySubscription).finish())
    .clone()
}

// ---------------------------------------------------------------------------
// The one projection: AcState
// ---------------------------------------------------------------------------

/// The discriminant of [`AcState`], as GraphQL's flat enum.
///
/// **`Computed` is one of the values, and exposing it is the point.** A client
/// asking a test-backed criterion for its state gets the honest answer -- that
/// nothing is stored and the answer comes from covering tests -- rather than a
/// fabricated `unsatisfied` that would be indistinguishable from an authored
/// one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
pub enum AcStateName {
  Computed,
  Unsatisfied,
  Satisfied,
  Descoped,
  Withdrawn,
  Fiat,
}

/// [`AcState`] flattened to `state` + the union of its variants' fields, which
/// is precisely serde's internally-tagged form. A field is `None` exactly when
/// the serde form omits it, so the two representations carry the same data.
#[derive(Debug, Clone, PartialEq, Eq, SimpleObject)]
pub struct AcStateView {
  pub state: AcStateName,
  /// Satisfied only: the named evidence.
  ///
  /// **Still satisfied-only after unsatisfied criteria gained prose of their
  /// own, and that is a ruling rather than an oversight (hv, 2026-08-29).** A
  /// note on an unsatisfied criterion is not proof that it was met, so it is
  /// published as [`AcStateView::note`] and never folded in here. A consumer
  /// reading `evidence` is asking whether the criterion was satisfied and gets
  /// an answer to that question only.
  pub evidence: Option<String>,
  /// Unsatisfied only: what was measured, what is blocking, or what would
  /// discharge the criterion.
  pub note: Option<String>,
  /// Descoped only: the thread the requirement moved to.
  pub to: Option<String>,
  /// Descoped or withdrawn: who ruled.
  pub by: Option<String>,
  /// Descoped or withdrawn: why.
  pub reason: Option<String>,
  /// Fiat only: why the close was made against the evidence.
  ///
  /// Deliberately NOT folded into [`AcStateView::reason`]: this view is the
  /// serde form flattened, and the serde form spells it `because`. Two names
  /// for one field would make the two representations disagree, which is the
  /// one property this type promises.
  pub because: Option<String>,
  /// Fiat only: when the close was recorded, RFC 3339 UTC.
  pub at: Option<String>,
  /// Fiat only: evidence about the invocation.
  pub invoker: Option<crate::model::Invoker>,
  /// Fiat only, and only on a CASCADED row: the ancestor whose fiat close
  /// reached it.
  pub inherited_from: Option<String>,
}

impl From<&AcState> for AcStateView {
  fn from(state: &AcState) -> Self {
    let base = Self {
      state: AcStateName::Computed,
      evidence: None,
      note: None,
      to: None,
      by: None,
      reason: None,
      because: None,
      at: None,
      invoker: None,
      inherited_from: None,
    };
    match state {
      AcState::Computed => base,
      AcState::Unsatisfied { note } => Self {
        state: AcStateName::Unsatisfied,
        note: note.clone(),
        ..base
      },
      AcState::Satisfied { evidence } => Self {
        state: AcStateName::Satisfied,
        evidence: Some(evidence.clone()),
        ..base
      },
      AcState::Descoped { to, by, reason } => Self {
        state: AcStateName::Descoped,
        to: Some(to.clone()),
        by: by.clone(),
        reason: reason.clone(),
        ..base
      },
      AcState::Withdrawn { reason, by } => Self {
        state: AcStateName::Withdrawn,
        by: by.clone(),
        reason: Some(reason.clone()),
        ..base
      },
      AcState::Fiat(record) => Self {
        state: AcStateName::Fiat,
        by: Some(record.by.clone()),
        because: Some(record.because.clone()),
        at: Some(record.at.clone()),
        invoker: Some(record.invoker.clone()),
        inherited_from: record.inherited_from.clone(),
        ..base
      },
    }
  }
}

#[async_graphql::ComplexObject]
impl Criterion {
  /// The recorded AC state, flattened -- see [`AcStateView`].
  async fn state(&self) -> AcStateView {
    AcStateView::from(&self.state)
  }
}

// ---------------------------------------------------------------------------
// Query root
// ---------------------------------------------------------------------------

/// What the resolvers answer from: the project's threads and issues, cloned out
/// of the facade at the moment the request was made.
///
/// **A SNAPSHOT RATHER THAN THE FACADE ITSELF, FOR A REASON THE TYPE SYSTEM
/// STATES.** Schema data must be `Send + Sync + 'static`, and a `Facade` is a
/// mutable store handle that lives on one thread; the two cannot meet. What
/// CAN move into the future is what the facade ANSWERS -- and that keeps the
/// seam where the bound puts it: the data reaches this face through
/// [`Facade::st_list`] and [`Facade::issue_list`], the same accessors `st list`
/// and `issues list` render, never through a second reader beside them.
///
/// Reads only, so a snapshot cannot go stale in a way one request can observe:
/// the document sees one consistent estate from its first field to its last.
pub struct Snapshot {
  threads: Vec<Thread>,
  issues: Vec<Issue>,
}

impl Snapshot {
  fn of(facade: &Facade) -> Snapshot {
    Snapshot {
      threads: facade.st_list().into_iter().cloned().collect(),
      issues: facade.issue_list().into_iter().cloned().collect(),
    }
  }
}

/// The snapshot this request carries, or a refusal that names the only door.
///
/// async-graphql's own message for missing data (`Data \`Snapshot\` does not
/// exist`) is true and useless to a caller; this one says what to call.
fn snapshot<'a>(ctx: &'a Context<'_>) -> async_graphql::Result<&'a Snapshot> {
  ctx.data::<Snapshot>().map_err(|_| {
    Error::new(
      "this schema executes only through `Facade::graphql`, which attaches the project's snapshot; a request built without it cannot be answered",
    )
  })
}

impl Facade {
  /// Execute one GraphQL document against this project: the escape hatch's
  /// facade seam (`AC-00.4`, `AC-09.2`).
  ///
  /// **DEFINED BESIDE THE SCHEMA IT EXECUTES, ON THE TYPE THAT OWNS THE DATA.**
  /// The MCP tier's dispatch row names its facade method by string
  /// (`facade: "graphql"`) and the two-sided gate over that tier holds only
  /// while the string names something real, so this is a method and not a free
  /// function. It lives in this module rather than `facade.rs` because
  /// everything it touches -- the schema, the snapshot, the variables form --
  /// is GraphQL's, and the facade's part is the two reads.
  ///
  /// **THE FACADE IS TOUCHED SYNCHRONOUSLY, BEFORE THE FUTURE EXISTS, AND
  /// NEVER BY IT.** The snapshot is taken here; the returned future owns it
  /// and borrows nothing, which is what lets intentd's store thread hand it to
  /// a runtime without the facade leaving the thread. **This function drives
  /// nothing** -- intentd blocks on the future with tokio's handle, and the CLI
  /// never calls this at all: it bridges to intentd or refuses (vc,
  /// 2026-08-31).
  ///
  /// The answer is the spec's `{data, errors}` object. Serialising it does not
  /// fail for any value async-graphql produces, but the `Result` is kept rather
  /// than unwrapped: a fault here belongs in the caller's remedy, not in a
  /// panic on the store thread.
  pub fn graphql(
    &self,
    query: &str,
    variables: Option<Value>,
  ) -> impl Future<Output = Result<Value, serde_json::Error>> + Send + 'static {
    let mut request = Request::new(query).data(Snapshot::of(self));
    if let Some(variables) = variables {
      request = request.variables(Variables::from_json(variables));
    }
    let schema = schema();
    async move { serde_json::to_value(schema.execute(request).await) }
  }
}

/// The read surface -- four roots over the [`Snapshot`].
pub struct Query;

#[Object]
impl Query {
  /// One steel thread by its natural id, eg `ST0000`; `null` when there is
  /// none, which is an answer and not an error.
  async fn thread(&self, ctx: &Context<'_>, id: String) -> async_graphql::Result<Option<Thread>> {
    Ok(snapshot(ctx)?.threads.iter().find(|t| t.id == id).cloned())
  }

  /// Every steel thread in the project, in the order `st list` renders them.
  async fn threads(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Thread>> {
    Ok(snapshot(ctx)?.threads.clone())
  }

  /// One issue by its number; `null` when there is none.
  async fn issue(&self, ctx: &Context<'_>, number: u32) -> async_graphql::Result<Option<Issue>> {
    Ok(
      snapshot(ctx)?
        .issues
        .iter()
        .find(|i| i.number == number)
        .cloned(),
    )
  }

  /// Every issue in the project, in number order.
  async fn issues(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Issue>> {
    Ok(snapshot(ctx)?.issues.clone())
  }
}
