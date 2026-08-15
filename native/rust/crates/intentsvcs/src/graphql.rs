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
//! The Query root exists because a schema cannot be built without one, and its
//! resolvers are wired to the store in WP-04. Until then they REFUSE, by name
//! -- an unwired resolver that answered `null` would be a silent error, and a
//! face that reads as working while returning nothing is worse than one that
//! does not compile.

use async_graphql::{
  Context, EmptyMutation, EmptySubscription, Enum, Object, Schema, SimpleObject,
};

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

/// The schema the SDL face describes, and (from WP-04) the one intentd serves.
pub fn schema() -> Schema<Query, EmptyMutation, EmptySubscription> {
  Schema::build(Query, EmptyMutation, EmptySubscription).finish()
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
}

/// [`AcState`] flattened to `state` + the union of its variants' fields, which
/// is precisely serde's internally-tagged form. A field is `None` exactly when
/// the serde form omits it, so the two representations carry the same data.
#[derive(Debug, Clone, PartialEq, Eq, SimpleObject)]
pub struct AcStateView {
  pub state: AcStateName,
  /// Satisfied only: the named evidence.
  pub evidence: Option<String>,
  /// Descoped only: the thread the requirement moved to.
  pub to: Option<String>,
  /// Descoped or withdrawn: who ruled.
  pub by: Option<String>,
  /// Descoped or withdrawn: why.
  pub reason: Option<String>,
}

impl From<&AcState> for AcStateView {
  fn from(state: &AcState) -> Self {
    let base = Self {
      state: AcStateName::Computed,
      evidence: None,
      to: None,
      by: None,
      reason: None,
    };
    match state {
      AcState::Computed => base,
      AcState::Unsatisfied => Self {
        state: AcStateName::Unsatisfied,
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

/// The read surface. Resolvers land on the intentsvcs facade in WP-04; until
/// then each refuses by name rather than answering emptily.
pub struct Query;

/// The one refusal, so the message cannot drift between resolvers.
///
/// It carried "(ST0056 WP-04)" until D37: a GraphQL client is the furthest
/// thing from this project's backlog that Intent has, and the schedule note
/// belongs in the module doc above, where it is.
fn unwired<T>(field: &str) -> async_graphql::Result<T> {
  Err(async_graphql::Error::new(format!(
    "Query.{field} is not implemented yet; the SDL face is published ahead of the resolvers"
  )))
}

#[Object]
impl Query {
  /// One steel thread by its natural id, eg `ST0056`.
  async fn thread(&self, _ctx: &Context<'_>, id: String) -> async_graphql::Result<Option<Thread>> {
    let _ = id;
    unwired("thread")
  }

  /// Every steel thread in the project.
  async fn threads(&self, _ctx: &Context<'_>) -> async_graphql::Result<Vec<Thread>> {
    unwired("threads")
  }

  /// One issue by its number.
  async fn issue(&self, _ctx: &Context<'_>, number: u32) -> async_graphql::Result<Option<Issue>> {
    let _ = number;
    unwired("issue")
  }

  /// Every issue in the project.
  async fn issues(&self, _ctx: &Context<'_>) -> async_graphql::Result<Vec<Issue>> {
    unwired("issues")
  }
}
