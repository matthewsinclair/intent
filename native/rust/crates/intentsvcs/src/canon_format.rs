//! The two canon formats a project can be in, and the one codec for each
//! (ST0095 WP-06, hv decision 70).
//!
//! **ONE FACT PER PROJECT.** A project's format is the schema of its project
//! state: `.canon/project.json`, which `intent init` and `intent upgrade` write,
//! and the store's copy, which a load carries in from the file as it carries
//! the DONE cutoff. A project state with no schema, an absent file or an empty
//! store, is [`CanonFormat::V3`] ([`CanonFormat::of_project`]). Every write of a
//! thread, an issue or the project state encodes here by the store's copy, so a
//! canon recreated from the store recreates the file, a project the upgrade has
//! not moved is written exactly as 3.4.0 writes it, and a machine still on
//! 3.4.0 keeps working beside the new build.
//!
//! **A FILE IS READ BY ITS OWN SCHEMA.** A merge can bring a file of the other
//! format into a project. Reading it is what lets `intent upgrade` move it, and
//! the next write of that record puts it in the project's format.
//!
//! **V3 IS FROZEN.** Its wire structs are 3.4.0's, field for field. Each
//! converts to and from the model by destructuring every field, so a field the
//! model gains is a compile error here, never a field a V3 write drops in
//! silence.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::id::{IssueId, ThreadId, WpId};
use crate::model::{
  AcceptanceMode, AcceptanceTest, Attachment, Criterion, FiatRecord, ISSUE_SCHEMA, Issue,
  IssueStatus, Legacy, PROJECT_SCHEMA, ProjectState, Related, THREAD_SCHEMA, TShirt, Thread,
  ThreadStatus, WorkPackage, WpStatus, authored_section_from, to_canonical_json,
};

/// The `schema` of a thread canon file in [`CanonFormat::V3`].
pub const THREAD_SCHEMA_V3: &str = "intent/thread@3.0";
/// The `schema` of an issue canon file in [`CanonFormat::V3`].
pub const ISSUE_SCHEMA_V3: &str = "intent/issue@3.0";
/// The `schema` of the project-state file in [`CanonFormat::V3`].
pub const PROJECT_SCHEMA_V3: &str = "intent/project@3.0";

/// The layout a project's canon is written in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonFormat {
  /// 3.4.0's, the ordinal format: a work package's `seq` and an issue's
  /// `number` are the numbers they have always been, and no record carries an
  /// origin. A project is in it until `intent upgrade` moves it, and its first
  /// mint refuses (hv decision 70).
  V3,
  /// The release's: every id is a string of either family, under `id`, and a
  /// hashed record carries its creation event's id as `origin`.
  V4,
}

/// The canon files whose schema names a format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonKind {
  Thread,
  Issue,
  Project,
}

impl CanonFormat {
  /// The format a new project is born in, and the one `intent upgrade` writes.
  pub const CURRENT: Self = Self::V4;

  /// The schema a `kind` file carries in this format: the one table of them.
  pub fn schema(self, kind: CanonKind) -> &'static str {
    match (self, kind) {
      (Self::V3, CanonKind::Thread) => THREAD_SCHEMA_V3,
      (Self::V3, CanonKind::Issue) => ISSUE_SCHEMA_V3,
      (Self::V3, CanonKind::Project) => PROJECT_SCHEMA_V3,
      (Self::V4, CanonKind::Thread) => THREAD_SCHEMA,
      (Self::V4, CanonKind::Issue) => ISSUE_SCHEMA,
      (Self::V4, CanonKind::Project) => PROJECT_SCHEMA,
    }
  }

  /// The format of a `kind` file whose schema is `schema`, or `None` where this
  /// build reads no such schema.
  pub fn of(kind: CanonKind, schema: &str) -> Option<Self> {
    [Self::V3, Self::V4]
      .into_iter()
      .find(|format| format.schema(kind) == schema)
  }

  /// The format a project state's schema names, where `None` is a project
  /// that carries none: 3.4.0's, whose `project.json` exists only for a cutoff.
  /// The one rule for both holders of a project's format, the file and the
  /// store's copy of it, and for an export bundle's project state.
  pub fn of_project(schema: Option<&str>) -> Result<Self, CanonFormatError> {
    match schema {
      None => Ok(Self::V3),
      Some(schema) => {
        Self::of(CanonKind::Project, schema).ok_or_else(|| CanonFormatError::UnreadProjectSchema {
          schema: schema.to_string(),
        })
      }
    }
  }

  /// Whether a project in this format always has its `project.json`. In the
  /// release's it does, because the file's schema is what makes an older Intent
  /// refuse the project by name rather than misread it. In 3.4.0's the file
  /// exists only for a cutoff (issue 0456).
  pub fn always_has_project_file(self) -> bool {
    match self {
      Self::V3 => false,
      Self::V4 => true,
    }
  }

  /// The schemas a `kind` file may carry, for a refusal that names what this
  /// build reads.
  pub fn readable(kind: CanonKind) -> String {
    format!("{:?} or {:?}", Self::V4.schema(kind), Self::V3.schema(kind))
  }
}

/// A record the ordinal format cannot carry, or one that did not serialise.
#[derive(Debug, thiserror::Error)]
pub enum CanonFormatError {
  /// A hashed id, or an origin, in a project still in [`CanonFormat::V3`]. The
  /// verbs never reach it, since that project's first mint refuses (hv
  /// decision 70); it is refused here rather than written as a number it does
  /// not have.
  #[error(
    "{record} has a hashed id, which the ordinal canon of a project `intent upgrade` has not \
     moved cannot carry"
  )]
  Hashed { record: String },
  /// A project state, held by the store or carried in an export bundle, whose
  /// schema this build does not read, so no format can be chosen for the canon.
  #[error(
    "the project state's schema is {schema:?}; this binary reads {}",
    CanonFormat::readable(CanonKind::Project)
  )]
  UnreadProjectSchema { schema: String },
  #[error("serialisation: {0}")]
  Serde(#[from] serde_json::Error),
}

impl crate::remedy::Remedy for CanonFormatError {
  fn remedy(&self) -> String {
    match self {
      Self::Hashed { .. } => "nothing was written. Run `intent upgrade`, which moves the \
        project's canon to the format that carries both id families"
        .to_string(),
      Self::UnreadProjectSchema { .. } => "nothing was written. The project state was written \
        by an Intent other than this one: run the build that wrote it, or move intent forward"
        .to_string(),
      Self::Serde(_) => "nothing was written. A record that does not serialise is a defect in \
        Intent: report it with the command that met it"
        .to_string(),
    }
  }
}

/// A thread's canon bytes in `format`: the one encoder every thread write asks.
pub fn encode_thread(thread: &Thread, format: CanonFormat) -> Result<String, CanonFormatError> {
  Ok(match format {
    CanonFormat::V4 => to_canonical_json(thread)?,
    CanonFormat::V3 => to_canonical_json(&ThreadV3::try_from(thread)?)?,
  })
}

/// An issue's canon bytes in `format`: the one encoder every issue write asks.
pub fn encode_issue(issue: &Issue, format: CanonFormat) -> Result<String, CanonFormatError> {
  Ok(match format {
    CanonFormat::V4 => to_canonical_json(issue)?,
    CanonFormat::V3 => to_canonical_json(&IssueV3::try_from(issue)?)?,
  })
}

/// The project state's canon bytes in `format`.
pub fn encode_project_state(
  todo_watermark: Option<String>,
  format: CanonFormat,
) -> Result<String, CanonFormatError> {
  Ok(to_canonical_json(&ProjectState::new(
    todo_watermark,
    format,
  ))?)
}

/// A thread as 3.4.0 writes it.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ThreadV3 {
  pub schema: String,
  pub id: ThreadId,
  pub title: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub slug: Option<String>,
  pub status: ThreadStatus,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status_reason: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub fiat: Option<FiatRecord>,
  pub created: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub completed: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub acceptance: Option<AcceptanceMode>,
  #[serde(default, deserialize_with = "authored_section_from")]
  #[schemars(with = "String")]
  pub objective: String,
  #[serde(default, deserialize_with = "authored_section_from")]
  #[schemars(with = "String")]
  pub context: String,
  #[serde(default)]
  pub body: String,
  #[serde(default)]
  pub preamble: String,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub related: Vec<Related>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub wps: Vec<WorkPackageV3>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub criteria: Vec<Criterion>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub tests: Vec<AcceptanceTest>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub attachments: Vec<Attachment>,
}

/// A work package as 3.4.0 writes it: its id is the number it has always been.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct WorkPackageV3 {
  #[serde(with = "crate::id::ordinal_face")]
  #[schemars(with = "u32")]
  pub seq: WpId,
  pub title: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub scope: Option<TShirt>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub scope_legacy: Option<Legacy>,
  pub status: WpStatus,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status_legacy: Option<Legacy>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status_reason: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub fiat: Option<FiatRecord>,
  #[serde(default, deserialize_with = "authored_section_from")]
  #[schemars(with = "String")]
  pub objective: String,
  #[serde(default)]
  pub body: String,
  #[serde(default)]
  pub preamble: String,
}

/// An issue as 3.4.0 writes it: its id is the number it has always been.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct IssueV3 {
  pub schema: String,
  #[serde(with = "crate::id::ordinal_face")]
  #[schemars(with = "u32")]
  pub number: IssueId,
  pub slug: String,
  pub title: String,
  pub status: IssueStatus,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub severity: Option<String>,
  pub created: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub closed: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub reporter: Option<String>,
  #[serde(default)]
  pub body: String,
}

impl From<ThreadV3> for Thread {
  fn from(v3: ThreadV3) -> Self {
    let ThreadV3 {
      schema: _,
      id,
      title,
      slug,
      status,
      status_reason,
      fiat,
      created,
      completed,
      acceptance,
      objective,
      context,
      body,
      preamble,
      related,
      wps,
      criteria,
      tests,
      attachments,
    } = v3;
    Thread {
      schema: THREAD_SCHEMA.to_string(),
      id,
      origin: None,
      // A 3.4.0 project's realisation is its `.intentfiles`, never a record's.
      realised: false,
      title,
      slug,
      status,
      status_reason,
      fiat,
      created,
      completed,
      acceptance,
      objective,
      context,
      body,
      preamble,
      related,
      wps: wps.into_iter().map(WorkPackage::from).collect(),
      criteria,
      tests,
      attachments,
    }
  }
}

impl TryFrom<&Thread> for ThreadV3 {
  type Error = CanonFormatError;

  fn try_from(thread: &Thread) -> Result<Self, Self::Error> {
    let Thread {
      schema: _,
      id,
      origin,
      // The ordinal format has no field for it: `.intentfiles` carries it.
      realised: _,
      title,
      slug,
      status,
      status_reason,
      fiat,
      created,
      completed,
      acceptance,
      objective,
      context,
      body,
      preamble,
      related,
      wps,
      criteria,
      tests,
      attachments,
    } = thread;
    if id.is_hashed() || origin.is_some() {
      return Err(CanonFormatError::Hashed {
        record: format!("thread {}", id.shown()),
      });
    }
    Ok(ThreadV3 {
      schema: THREAD_SCHEMA_V3.to_string(),
      id: id.clone(),
      title: title.clone(),
      slug: slug.clone(),
      status: *status,
      status_reason: status_reason.clone(),
      fiat: fiat.clone(),
      created: created.clone(),
      completed: completed.clone(),
      acceptance: *acceptance,
      objective: objective.clone(),
      context: context.clone(),
      body: body.clone(),
      preamble: preamble.clone(),
      related: related.clone(),
      wps: wps
        .iter()
        .map(|wp| WorkPackageV3::in_thread(id, wp))
        .collect::<Result<_, _>>()?,
      criteria: criteria.clone(),
      tests: tests.clone(),
      attachments: attachments.clone(),
    })
  }
}

impl From<WorkPackageV3> for WorkPackage {
  fn from(v3: WorkPackageV3) -> Self {
    let WorkPackageV3 {
      seq,
      title,
      scope,
      scope_legacy,
      status,
      status_legacy,
      status_reason,
      fiat,
      objective,
      body,
      preamble,
    } = v3;
    WorkPackage {
      seq,
      origin: None,
      title,
      scope,
      scope_legacy,
      status,
      status_legacy,
      status_reason,
      fiat,
      objective,
      body,
      preamble,
    }
  }
}

impl WorkPackageV3 {
  /// `wp` of `thread` as 3.4.0 writes it, refused where it is hashed.
  fn in_thread(thread: &ThreadId, wp: &WorkPackage) -> Result<Self, CanonFormatError> {
    let WorkPackage {
      seq,
      origin,
      title,
      scope,
      scope_legacy,
      status,
      status_legacy,
      status_reason,
      fiat,
      objective,
      body,
      preamble,
    } = wp;
    if seq.is_hashed() || origin.is_some() {
      return Err(CanonFormatError::Hashed {
        record: format!("work package {}", crate::id::wp_address_shown(thread, *seq)),
      });
    }
    Ok(Self {
      seq: *seq,
      title: title.clone(),
      scope: *scope,
      scope_legacy: scope_legacy.clone(),
      status: *status,
      status_legacy: status_legacy.clone(),
      status_reason: status_reason.clone(),
      fiat: fiat.clone(),
      objective: objective.clone(),
      body: body.clone(),
      preamble: preamble.clone(),
    })
  }
}

impl From<IssueV3> for Issue {
  fn from(v3: IssueV3) -> Self {
    let IssueV3 {
      schema: _,
      number,
      slug,
      title,
      status,
      severity,
      created,
      closed,
      reporter,
      body,
    } = v3;
    Issue {
      schema: ISSUE_SCHEMA.to_string(),
      number,
      origin: None,
      realised: false,
      slug,
      title,
      status,
      severity,
      created,
      closed,
      reporter,
      body,
    }
  }
}

impl TryFrom<&Issue> for IssueV3 {
  type Error = CanonFormatError;

  fn try_from(issue: &Issue) -> Result<Self, Self::Error> {
    let Issue {
      schema: _,
      number,
      origin,
      realised: _,
      slug,
      title,
      status,
      severity,
      created,
      closed,
      reporter,
      body,
    } = issue;
    if number.is_hashed() || origin.is_some() {
      return Err(CanonFormatError::Hashed {
        record: format!("issue {}", number.shown()),
      });
    }
    Ok(IssueV3 {
      schema: ISSUE_SCHEMA_V3.to_string(),
      number: *number,
      slug: slug.clone(),
      title: title.clone(),
      status: *status,
      severity: severity.clone(),
      created: created.clone(),
      closed: closed.clone(),
      reporter: reporter.clone(),
      body: body.clone(),
    })
  }
}
