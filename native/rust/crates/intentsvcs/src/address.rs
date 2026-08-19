//! `intent://` -- the address of a piece of data (D57-8, WP-07).
//!
//! **Hydration makes a file path a statement about a moment.**
//! `intent/st/ST0034/design.md` exists or does not depending on what
//! `organize` last did, so every reference to it is conditional. Measured at
//! `ce532a97`: 80 citations of `intent/st/ST####/<file>.md` in tracked estate
//! prose, the most-cited being `ST0034/design.md` at 23 -- a COMPLETED thread,
//! and therefore among the first to dehydrate.
//!
//! **So references name the ENTITY, never the file.**
//!
//! # Resolution has ONE home (AC-07.1)
//!
//! This module. The CLI calls it in-process; intentd calls THE SAME function
//! and serves it over GraphQL. Neither implements resolution. **The failure to
//! guard is intentd growing its own resolver because GraphQL wants different
//! shapes** -- two resolvers agreeing exactly until one moves, with nothing
//! watching. That is asserted by there being a single implementation, not by
//! comparing two outputs, because comparing two outputs is what you do once
//! you have already lost.
//!
//! # Views get no address (AC-07.2)
//!
//! A view is derivable from its entity, so a reference to a view is a
//! reference to its source. Giving views addresses would re-create, INSIDE the
//! scheme, the exact conditionality the scheme exists to remove.
//!
//! `?format=` selects a REPRESENTATION of the addressed entity; a path segment
//! would name a separate thing. `/threads/ST0056` is the thread and
//! `?format=md` is its cover; there is no `/threads/ST0056/info.md`.
//!
//! # One rendering per address (AC-07.3)
//!
//! If an entity has more than one rendering it is UNDER-ADDRESSED. The format
//! set is held at exactly `json` and `md` -- two formats with a ratified
//! meaning each beats four that drift -- and an entity with two renderings
//! gains a distinct ADDRESS rather than a `?view=`.

use crate::model;
use crate::remedy::Remedy;
use thiserror::Error;

/// The scheme, including its separator. Written once.
pub const SCHEME: &str = "intent://";

/// A representation of the addressed entity.
///
/// **Exactly two, closed for 3.0.0.** `GET` accepts both; `PUT` accepts `json`
/// only, because writing markdown to an address would promote a stale
/// rendering into canon.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
  Json,
  Md,
}

impl Format {
  pub fn as_str(&self) -> &'static str {
    match self {
      Format::Json => "json",
      Format::Md => "md",
    }
  }

  pub fn parse(s: &str) -> Option<Self> {
    match s {
      "json" => Some(Format::Json),
      "md" => Some(Format::Md),
      _ => None,
    }
  }
}

/// What an address names.
///
/// **D57-8's nine ARTEFACT forms, plus the two COLLECTION addresses its prose
/// names.** The distinction is load-bearing and the comment here was wrong
/// before vc ruled on it (2026-08-19): a tenth ARTEFACT form is a design
/// change and must be one, while a collection address is not -- the nine-form
/// list enumerates artefacts, it is not a closed grammar.
///
/// Both collections are required rather than convenient. D57-8 says
/// `/threads/{id}` returns the cover and `/threads/{id}/ac` returns the
/// acceptance view, and says server-assigned ids are a `POST` to the
/// COLLECTION address. **Without `AcCollection` the acceptance view has no
/// address at all**, and AC-07.4 requires `?format=md` to serve what
/// `organize` would hydrate -- which includes `acceptance.md`. The only other
/// route is giving the VIEW an address, which is exactly what AC-07.2 forbids
/// in the sentence that exists to stop the scheme becoming a path alias.
///
/// **The collection is the addressee and the view is its representation.**
/// That applies D57-8's entity-versus-representation split rather than
/// extending it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Entity {
  /// The threads COLLECTION. D57-8 gives collections addresses because
  /// server-assigned ids are a `POST` to one -- you cannot address `ST0058`
  /// before the tool has decided it is `ST0058`.
  Threads,
  Thread {
    id: String,
  },
  /// A thread's acceptance collection. **This is how the acceptance view is
  /// reachable without giving a view an address**: D57-8 says
  /// `/threads/ST0056` returns the cover and `/threads/ST0056/ac` returns the
  /// acceptance view, so the addressee is the COLLECTION and the view is its
  /// markdown representation. That is the distinction AC-07.2 rests on.
  AcCollection {
    thread: String,
  },
  Wp {
    thread: String,
    wp: String,
  },
  Ac {
    thread: String,
    ac: String,
  },
  At {
    thread: String,
    at: String,
  },
  Attachment {
    thread: String,
    path: String,
  },
  Issue {
    id: String,
  },
  Node {
    moniker: String,
  },
  NodeInbox {
    moniker: String,
    sender: String,
    stamp: String,
  },
  Event {
    id: String,
  },
}

/// A parsed `intent://` address.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address {
  /// `None` is the EMPTY AUTHORITY and means THIS project (AC-07.6).
  ///
  /// Nearly every reference is intra-project, and one that hard-codes the
  /// project name breaks on rename or fork -- so the empty form is the one
  /// intra-project prose must use, and `Some` is the deliberate exception
  /// rather than the neutral default.
  pub authority: Option<String>,
  pub entity: Entity,
  /// **`None` means the address did not ask**, which is not the same as asking
  /// for a default. A parser that substituted one here would make
  /// `?format=json` and a bare address indistinguishable downstream, and the
  /// choice of default belongs to the caller that knows whether it is
  /// answering a `GET` or writing a file.
  pub format: Option<Format>,
}

/// Why an address could not be read.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum AddressError {
  #[error("`{input}` does not start with `{SCHEME}`")]
  NotAnIntentUrl { input: String },
  #[error("`{segment}` is not an addressable collection")]
  UnknownCollection { segment: String },
  #[error("`{input}` names no entity -- an address needs a collection and an id")]
  Empty { input: String },
  #[error("`{id}` is not a valid {kind} id")]
  MalformedId { kind: &'static str, id: String },
  #[error("`{input}` has trailing segments after a complete address")]
  TrailingSegments { input: String },
  #[error("`{found}` is not a format -- `?format=` accepts exactly json and md")]
  UnknownFormat { found: String },
  #[error("`{found}` is not a query this scheme accepts -- only `format=` exists")]
  UnknownQuery { found: String },
  /// AC-07.2, and its own variant rather than a generic refusal: an operator
  /// who wrote a view path needs to be told the entity is the address, not
  /// that they mistyped something.
  #[error("`{segment}` is a VIEW, and views have no address")]
  ViewAddressed { segment: String },
}

impl Remedy for AddressError {
  fn remedy(&self) -> String {
    match self {
      AddressError::NotAnIntentUrl { .. } => {
        format!("an address begins `{SCHEME}`; an empty authority means this project")
      }
      AddressError::UnknownCollection { .. } => {
        "the collections are threads, issues, nodes and events".into()
      }
      AddressError::Empty { .. } => {
        format!("name an entity, eg `{SCHEME}/threads/ST0000`")
      }
      AddressError::MalformedId { kind, .. } => match *kind {
        "thread" => "a thread id is ST followed by four digits, eg ST0000".into(),
        "issue" => "an issue id is four digits, eg 0042".into(),
        _ => format!("check the {kind} id against the form the estate uses"),
      },
      AddressError::TrailingSegments { .. } => {
        "an address ends at the entity it names; use `?format=` for a representation".into()
      }
      AddressError::UnknownFormat { .. } | AddressError::UnknownQuery { .. } => {
        "`?format=json` or `?format=md`, and nothing else".into()
      }
      AddressError::ViewAddressed { .. } => {
        "address the ENTITY and select a representation: `/threads/ST0000?format=md`".into()
      }
    }
  }
}

/// View filenames, refused as path segments.
///
/// **Named here rather than detected by extension**, because `.md` is also how
/// an attachment is spelled and an attachment IS addressable. The distinction
/// is authorship, not suffix: `Project::classify` is the single answer to what
/// a file is, and these are the names it answers `GeneratedView` for.
const VIEW_NAMES: &[&str] = &[
  "info.md",
  "acceptance.md",
  "todo.md",
  "steel_threads.md",
  "info",
  "acceptance",
];

/// Parse an `intent://` address. **The one implementation** (AC-07.1).
pub fn parse(input: &str) -> Result<Address, AddressError> {
  let rest = input
    .strip_prefix(SCHEME)
    .ok_or_else(|| AddressError::NotAnIntentUrl {
      input: input.to_string(),
    })?;

  let (location, query) = match rest.split_once('?') {
    Some((l, q)) => (l, Some(q)),
    None => (rest, None),
  };

  // `intent:///threads/...` -- the empty authority is the leading empty
  // segment before the first `/`. `intent://other/threads/...` carries a slug.
  let (authority, path) = match location.split_once('/') {
    Some((a, p)) => (a, p),
    None => (location, ""),
  };
  let authority = (!authority.is_empty()).then(|| authority.to_string());

  let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
  if segments.is_empty() {
    return Err(AddressError::Empty {
      input: input.to_string(),
    });
  }
  for s in &segments {
    if VIEW_NAMES.contains(s) {
      return Err(AddressError::ViewAddressed {
        segment: s.to_string(),
      });
    }
  }

  let entity = parse_entity(&segments, input)?;
  let format = parse_format(query)?;

  Ok(Address {
    authority,
    entity,
    format,
  })
}

fn parse_format(query: Option<&str>) -> Result<Option<Format>, AddressError> {
  let Some(q) = query else { return Ok(None) };
  if q.is_empty() {
    return Ok(None);
  }
  let (key, value) = q
    .split_once('=')
    .ok_or_else(|| AddressError::UnknownQuery {
      found: q.to_string(),
    })?;
  if key != "format" {
    return Err(AddressError::UnknownQuery {
      found: key.to_string(),
    });
  }
  Format::parse(value)
    .map(Some)
    .ok_or_else(|| AddressError::UnknownFormat {
      found: value.to_string(),
    })
}

fn thread(id: &str) -> Result<String, AddressError> {
  model::is_thread_id(id)
    .then(|| id.to_string())
    .ok_or_else(|| AddressError::MalformedId {
      kind: "thread",
      id: id.to_string(),
    })
}

fn parse_entity(segments: &[&str], input: &str) -> Result<Entity, AddressError> {
  let trailing = || AddressError::TrailingSegments {
    input: input.to_string(),
  };
  match segments {
    ["threads"] => Ok(Entity::Threads),
    ["threads", id] => Ok(Entity::Thread { id: thread(id)? }),
    ["threads", id, "ac"] => Ok(Entity::AcCollection {
      thread: thread(id)?,
    }),
    ["threads", id, "wp", wp] => Ok(Entity::Wp {
      thread: thread(id)?,
      wp: wp.to_string(),
    }),
    ["threads", id, "ac", ac] => Ok(Entity::Ac {
      thread: thread(id)?,
      ac: ac.to_string(),
    }),
    ["threads", id, "at", at] => Ok(Entity::At {
      thread: thread(id)?,
      at: at.to_string(),
    }),
    // The attachment path is the REMAINDER, because an attachment lives at a
    // relative path with its own separators. Rejoined rather than kept as
    // segments so the value is what `Project::classify` takes.
    ["threads", id, "attachments", rest @ ..] if !rest.is_empty() => Ok(Entity::Attachment {
      thread: thread(id)?,
      path: rest.join("/"),
    }),
    ["issues", id] => model::is_issue_id(id)
      .then(|| Entity::Issue { id: id.to_string() })
      .ok_or_else(|| AddressError::MalformedId {
        kind: "issue",
        id: id.to_string(),
      }),
    ["nodes", moniker] => Ok(Entity::Node {
      moniker: moniker.to_string(),
    }),
    ["nodes", moniker, "inbox", sender, stamp] => Ok(Entity::NodeInbox {
      moniker: moniker.to_string(),
      sender: sender.to_string(),
      stamp: stamp.to_string(),
    }),
    ["events", id] => Ok(Entity::Event { id: id.to_string() }),
    // A known collection with the wrong arity is a trailing-segment problem,
    // not an unknown collection -- telling an operator "threads is not a
    // collection" when they wrote one extra segment sends them the wrong way.
    ["threads", ..] | ["issues", ..] | ["nodes", ..] | ["events", ..] => Err(trailing()),
    [first, ..] => Err(AddressError::UnknownCollection {
      segment: first.to_string(),
    }),
    [] => Err(AddressError::Empty {
      input: input.to_string(),
    }),
  }
}

impl Address {
  /// Render back to a string. Round-trips with [`parse`].
  pub fn to_url(&self) -> String {
    let authority = self.authority.as_deref().unwrap_or("");
    let path = match &self.entity {
      Entity::Threads => "threads".to_string(),
      Entity::Thread { id } => format!("threads/{id}"),
      Entity::AcCollection { thread } => format!("threads/{thread}/ac"),
      Entity::Wp { thread, wp } => format!("threads/{thread}/wp/{wp}"),
      Entity::Ac { thread, ac } => format!("threads/{thread}/ac/{ac}"),
      Entity::At { thread, at } => format!("threads/{thread}/at/{at}"),
      Entity::Attachment { thread, path } => format!("threads/{thread}/attachments/{path}"),
      Entity::Issue { id } => format!("issues/{id}"),
      Entity::Node { moniker } => format!("nodes/{moniker}"),
      Entity::NodeInbox {
        moniker,
        sender,
        stamp,
      } => format!("nodes/{moniker}/inbox/{sender}/{stamp}"),
      Entity::Event { id } => format!("events/{id}"),
    };
    let query = match self.format {
      Some(f) => format!("?format={}", f.as_str()),
      None => String::new(),
    };
    format!("{SCHEME}{authority}/{path}{query}")
  }

  /// Whether this address names something in THIS project (AC-07.6).
  pub fn is_local(&self) -> bool {
    self.authority.is_none()
  }
}

// ---------------------------------------------------------------------------
// Serving a representation -- AC-07.4
// ---------------------------------------------------------------------------

use crate::ingest::Canon;
use crate::project::Project;
use crate::views::{self, RenderContext, View};

/// Why an address has no markdown representation.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ServeError {
  #[error("`{url}` has no markdown rendering -- it is data, not a document")]
  NoMarkdownRendering { url: String },
  #[error("`{url}` names something this estate does not hold")]
  NotFound { url: String },
}

impl Remedy for ServeError {
  fn remedy(&self) -> String {
    match self {
      ServeError::NoMarkdownRendering { .. } => {
        "ask for `?format=json`; only threads, their acceptance collection, work \
         packages and the thread collection render as markdown"
          .into()
      }
      ServeError::NotFound { .. } => "check the id against `intent st list --status all`".into(),
    }
  }
}

/// Where an entity's markdown WOULD be hydrated, or `None` if it has none.
///
/// **Answered by the `Project` resolvers and nothing else**, which is what
/// makes the serve path a SELECTION rather than a second rendering. A function
/// that built this path from string parts would be the independent spelling
/// AC-01.6 exists to catch.
fn view_path_of(project: &Project, entity: &Entity) -> Option<std::path::PathBuf> {
  match entity {
    Entity::Threads => Some(project.steel_threads_view()),
    Entity::Thread { id } => Some(project.info_view(id)),
    Entity::AcCollection { thread } => Some(project.acceptance_view(thread)),
    Entity::Wp { thread, wp } => wp
      .parse::<u32>()
      .ok()
      .map(|seq| project.wp_info_view(thread, seq)),
    _ => None,
  }
}

/// Serve `?format=md` for an address: **the exact bytes `organize` would
/// hydrate** (AC-07.4).
///
/// **This selects from [`views::render_all`]; it never renders.** That is the
/// whole of the criterion and it is why the guarantee is BY CONSTRUCTION
/// rather than by test: `View.path` is literally where the file would land, so
/// the served bytes and the hydrated bytes are the same `String` from the same
/// call, not two renderings someone compares afterwards.
///
/// A second renderer here would be the two-declarations defect -- it would
/// agree with `render_all` on the day it was written, and a test comparing
/// them would pass for exactly as long as that lasted. One renderer, three
/// jobs: what `sync` writes (`facade.rs`), the skew check (`doctor.rs`), and
/// this.
pub fn serve_md(
  project: &Project,
  canon: &Canon,
  ctx: &RenderContext<'_>,
  address: &Address,
) -> Result<View, ServeError> {
  let want =
    view_path_of(project, &address.entity).ok_or_else(|| ServeError::NoMarkdownRendering {
      url: address.to_url(),
    })?;
  views::render_all(project, canon, ctx)
    .into_iter()
    .find(|v| v.path == want)
    .ok_or_else(|| ServeError::NotFound {
      url: address.to_url(),
    })
}
