//! The shipped Claude Team Profiles and the seat registry they draw on (ST0092
//! WP-02).
//!
//! **ROOTED AT THE INSTALL, AS THE RULE LIBRARY IS, AND NOTHING ELSE.** A
//! profile is a resource the tool ships, so a project names one and never
//! copies it, and every answer here is the same from any directory, a project
//! or not. Nothing is read from the extension base: a user-authored profile is
//! out of this thread, so [`Teams`] takes no ext root at all rather than one it
//! ignores.
//!
//! The words are defined in `TERMS.md` beside these files and nowhere here.
//! This module reads three things from that directory: `seats.json`, the seat
//! registry, which holds each seat once; one `<name>.md` per profile, whose
//! flat frontmatter names its kind, its mediator and its seats and whose body
//! is the brief; and `TERMS.md` itself, which `wb profile terms` prints whole.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Where an install keeps its Team Profiles, relative to its root.
pub const CANON_TEAMS: &str = "intent/plugins/claude/teams";
/// The seat registry's file name under [`CANON_TEAMS`].
pub const SEATS_FILE: &str = "seats.json";
/// The terms document's file name under [`CANON_TEAMS`]. It is Markdown beside
/// the profiles and is not one, so the profile enumeration leaves it out by
/// this name.
pub const TERMS_FILE: &str = "TERMS.md";

/// The four sections a base's brief is printed from, each an exact H2 heading.
/// The route to hv and the launch steps are written once for the mediator and
/// once for every other seat, and the reader chooses by whether the seat is the
/// declared mediator (WP-06, WP-07). A layer carries none of them.
pub const SECTION_ROUTE_MEDIATOR: &str = "## Route to hv: the mediator";
/// See [`SECTION_ROUTE_MEDIATOR`].
pub const SECTION_ROUTE_SEATS: &str = "## Route to hv: every other seat";
/// See [`SECTION_ROUTE_MEDIATOR`].
pub const SECTION_LAUNCH_MEDIATOR: &str = "## At launch: the mediator";
/// See [`SECTION_ROUTE_MEDIATOR`].
pub const SECTION_LAUNCH_SEATS: &str = "## At launch: every other seat";
/// The four, in the order a base carries them.
pub const SECTIONS: [&str; 4] = [
  SECTION_ROUTE_MEDIATOR,
  SECTION_ROUTE_SEATS,
  SECTION_LAUNCH_MEDIATOR,
  SECTION_LAUNCH_SEATS,
];

/// The seat registry, `seats.json`: each seat once.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SeatRegistry {
  /// Every seat any shipped profile may name.
  pub seats: Vec<Seat>,
}

/// One seat, as the terms document's seats table defines it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Seat {
  /// The two-letter handle: the routing key and the board's directory name.
  pub moniker: String,
  /// The display name, eg Control Claude.
  pub name: String,
  /// One word for what the seat does, eg `control`.
  pub role: String,
  /// Who drives the seat.
  pub driver: Driver,
  /// The default remit: one line saying what the seat is for.
  pub remit: String,
}

/// Who drives a seat.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum Driver {
  /// A Claude session started in the seat.
  Session,
  /// hv, the human. A seat driven by the human is only ever registered.
  Human,
  /// The supervisor's own session, from its hub.
  SupervisorSession,
}

impl Driver {
  /// The words the terms document's "Driven by" column uses for this driver,
  /// so the parity test (AC-02.5) compares the table against the registry
  /// through one mapping.
  pub fn phrase(self) -> &'static str {
    match self {
      Self::Session => "a session",
      Self::Human => "the human",
      Self::SupervisorSession => "the supervisor's session",
    }
  }
}

/// Whether a profile declares a whole roster or adds to one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum ProfileKind {
  /// Declares a whole roster; a whiteboard declares exactly one.
  Base,
  /// Adds seats to a base, and may move the mediator.
  Layer,
}

/// A profile file's frontmatter: flat, three keys.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ProfileFront {
  /// A base or a layer.
  pub kind: ProfileKind,
  /// The seat through which hv is reached. A base names one; a layer names one
  /// only when it moves the mediator.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub mediator: Option<String>,
  /// The monikers of the seats the profile declares or adds.
  pub seats: Vec<String>,
}

/// The frontmatter keys a profile may carry, read off [`ProfileFront`] so the
/// strict reader and the type cannot disagree about the vocabulary.
const FRONT_KEYS: [&str; 3] = ["kind", "mediator", "seats"];

/// One profile as read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Profile {
  /// Its name: the file name without `.md`.
  pub name: String,
  pub path: PathBuf,
  pub front: ProfileFront,
  /// The body after the frontmatter, printed whole.
  pub brief: String,
}

impl Profile {
  /// The four section headings this profile's brief carries, in file order.
  pub fn sections(&self) -> Vec<&'static str> {
    let headings: Vec<&str> = self.brief.lines().map(str::trim_end).collect();
    SECTIONS
      .iter()
      .filter(|s| headings.contains(s))
      .copied()
      .collect()
  }
}

/// One thing wrong with one shipped file: the file, the field, and what is
/// wrong with it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Finding {
  pub path: PathBuf,
  pub field: String,
  pub message: String,
}

impl std::fmt::Display for Finding {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}: {}: {}",
      self.path.display(),
      self.field,
      self.message
    )
  }
}

#[derive(Debug, Error)]
pub enum TeamsError {
  /// The profiles directory is not in this install, so nothing here can answer.
  #[error("this install carries no Team Profiles at {path}")]
  Absent { path: PathBuf },
  #[error("cannot read {path}: {source}")]
  Io {
    path: PathBuf,
    #[source]
    source: std::io::Error,
  },
  /// A shipped file that does not parse into its type. It names the field, so
  /// a validator can report it in the same terms as a semantic finding.
  #[error("{path}: {field}: {detail}")]
  Malformed {
    path: PathBuf,
    field: String,
    detail: String,
  },
  /// A profile was asked for by a name the install does not ship (INV-04).
  #[error("no Team Profile named `{name}`")]
  UnknownProfile { name: String, shipped: Vec<String> },
  /// `wb init` was named more than one base; a board declares exactly one.
  #[error("`wb init` was named more than one base: {}", bases.join(", "))]
  TwoBases { bases: Vec<String> },
  /// A layer was named on a board that declares no base, and none was named.
  #[error("a layer ({}) adds to a base, and this board declares none and none was named", layers.join(", "))]
  LayerWithoutBase { layers: Vec<String> },
  /// A base other than the declared one, without `--change`.
  #[error("this board declares the base `{declared}`, and `{named}` was named")]
  BaseDiffers { declared: String, named: String },
  /// The reinit (`wb init --apply` with no profile) on a board with no
  /// recorded declaration.
  #[error("this board records no declaration to re-apply")]
  NoDeclaration { shipped: Vec<String> },
}

impl TeamsError {
  /// Whether this build cannot answer at all, as against a refusal of what
  /// was asked: the install's profiles are absent, malformed or unreadable.
  /// [`crate::facade::FacadeError::build_cannot_answer`] reads it, so exit 2
  /// has one definition for a profile fault.
  pub fn build_cannot_answer(&self) -> bool {
    match self {
      Self::Malformed { .. } | Self::Absent { .. } | Self::Io { .. } => true,
      Self::UnknownProfile { .. }
      | Self::TwoBases { .. }
      | Self::LayerWithoutBase { .. }
      | Self::BaseDiffers { .. }
      | Self::NoDeclaration { .. } => false,
    }
  }
}

impl crate::remedy::Remedy for TeamsError {
  fn remedy(&self) -> String {
    match self {
      Self::Absent { .. } | Self::Io { .. } => {
        "the Team Profiles ship with the tool, so this usually means the install is incomplete: `intent info` names the install root this binary resolved, and reinstalling Intent restores the files".to_string()
      }
      Self::Malformed { .. } => {
        "the shipped profiles are the tool's own files, so a malformed one is a defect in this build: `intent wb profile validate` names every finding, and reinstalling Intent restores the shipped set".to_string()
      }
      Self::UnknownProfile { shipped, .. } => format!(
        "name one of the shipped profiles: {}. `intent wb profile list` says which are bases and which are layers",
        shipped.join(", ")
      ),
      Self::TwoBases { .. } => {
        "name one base and any layers after it, eg `intent wb init team supervised`; `intent wb profile list` says which profile is which".to_string()
      }
      Self::LayerWithoutBase { .. } => {
        "name the base first, eg `intent wb init team supervised`; `intent wb profile list` says which profiles are bases".to_string()
      }
      Self::BaseDiffers { declared, named } => format!(
        "to move this board from `{declared}` to `{named}`, pass `--change`, which retires the seats `{named}` does not declare; to keep `{declared}`, name it or name no base"
      ),
      Self::NoDeclaration { shipped } => format!(
        "name the profiles to declare, eg `intent wb init team --apply`; the shipped profiles are {}",
        shipped.join(", ")
      ),
    }
  }
}

/// The Team Profiles one install ships.
#[derive(Debug, Clone)]
pub struct Teams {
  root: PathBuf,
}

impl Teams {
  /// Rooted at an install's [`CANON_TEAMS`] directory.
  pub fn new(install: &Path) -> Self {
    Self::at(&install.join(CANON_TEAMS))
  }

  /// Rooted at a profiles directory named directly, for a test's fixture
  /// install. [`Teams::new`] is this at the install's canon path.
  pub fn at(root: &Path) -> Self {
    Self {
      root: root.to_path_buf(),
    }
  }

  /// The profiles directory this reads.
  pub fn root(&self) -> &Path {
    &self.root
  }

  /// The terms document, whole. It reads no profile.
  pub fn terms(&self) -> Result<String, TeamsError> {
    self.read(&self.root.join(TERMS_FILE))
  }

  /// The seat registry, read strictly: an unknown key or a missing field is a
  /// [`TeamsError::Malformed`] naming the field.
  pub fn registry(&self) -> Result<SeatRegistry, TeamsError> {
    let path = self.root.join(SEATS_FILE);
    let text = self.read(&path)?;
    serde_json::from_str(&text).map_err(|e| TeamsError::Malformed {
      field: serde_field(&e.to_string()),
      detail: e.to_string(),
      path,
    })
  }

  /// The name of every shipped profile, sorted: each `*.md` in the directory
  /// but the terms document.
  pub fn names(&self) -> Result<Vec<String>, TeamsError> {
    if !self.root.is_dir() {
      return Err(TeamsError::Absent {
        path: self.root.clone(),
      });
    }
    let entries = std::fs::read_dir(&self.root).map_err(|source| TeamsError::Io {
      path: self.root.clone(),
      source,
    })?;
    let mut names = Vec::new();
    for entry in entries {
      let entry = entry.map_err(|source| TeamsError::Io {
        path: self.root.clone(),
        source,
      })?;
      let file = entry.file_name().to_string_lossy().into_owned();
      if file == TERMS_FILE {
        continue;
      }
      if let Some(name) = file.strip_suffix(".md") {
        names.push(name.to_string());
      }
    }
    names.sort();
    Ok(names)
  }

  /// One profile by name. A name the install does not ship is refused with the
  /// shipped names, never answered empty.
  pub fn profile(&self, name: &str) -> Result<Profile, TeamsError> {
    let shipped = self.names()?;
    if !shipped.iter().any(|n| n == name) {
      return Err(TeamsError::UnknownProfile {
        name: name.to_string(),
        shipped,
      });
    }
    self.read_profile(name)
  }

  /// Every shipped profile, in name order.
  pub fn profiles(&self) -> Result<Vec<Profile>, TeamsError> {
    self
      .names()?
      .iter()
      .map(|name| self.read_profile(name))
      .collect()
  }

  /// Every finding across the shipped set, and how many profiles were read.
  ///
  /// A file that does not parse is a finding about that file and field, not a
  /// refusal of the whole run, so one broken profile does not hide the
  /// findings in the others. The run itself is refused only when the
  /// directory cannot be listed.
  pub fn validate(&self) -> Result<(Vec<Finding>, usize), TeamsError> {
    let names = self.names()?;
    let mut findings = Vec::new();
    let registry = match self.registry() {
      Ok(registry) => {
        findings.extend(check_registry(&self.root.join(SEATS_FILE), &registry));
        Some(registry)
      }
      Err(e) => {
        findings.push(finding_of(e, &self.root.join(SEATS_FILE)));
        None
      }
    };
    for name in &names {
      match self.read_profile(name) {
        Ok(profile) => findings.extend(check_profile(&profile, registry.as_ref())),
        Err(e) => findings.push(finding_of(e, &self.root.join(format!("{name}.md")))),
      }
    }
    Ok((findings, names.len()))
  }

  fn read_profile(&self, name: &str) -> Result<Profile, TeamsError> {
    let path = self.root.join(format!("{name}.md"));
    let text = self.read(&path)?;
    let (front, brief) = parse_profile(&path, &text)?;
    Ok(Profile {
      name: name.to_string(),
      path,
      front,
      brief,
    })
  }

  fn read(&self, path: &Path) -> Result<String, TeamsError> {
    if !self.root.is_dir() {
      return Err(TeamsError::Absent {
        path: self.root.clone(),
      });
    }
    std::fs::read_to_string(path).map_err(|source| TeamsError::Io {
      path: path.to_path_buf(),
      source,
    })
  }
}

/// A loader error as a validation finding about the file it names.
fn finding_of(e: TeamsError, path: &Path) -> Finding {
  match e {
    TeamsError::Malformed {
      path,
      field,
      detail,
    } => Finding {
      path,
      field,
      message: detail,
    },
    other => Finding {
      path: path.to_path_buf(),
      field: "file".to_string(),
      message: other.to_string(),
    },
  }
}

/// The field a serde_json error names, read off its message: `unknown field
/// `x``, `missing field `x``, or an `unknown variant` of the field a value
/// failed in. Anything else is about the file as a whole.
fn serde_field(message: &str) -> String {
  for lead in ["unknown field `", "missing field `"] {
    if let Some(rest) = message.split(lead).nth(1)
      && let Some(field) = rest.split('`').next()
    {
      return field.to_string();
    }
  }
  if message.contains("unknown variant") {
    return "driver".to_string();
  }
  "file".to_string()
}

/// Split a profile file into its strict frontmatter and its brief. **PURE.**
///
/// The block is read by [`crate::rules::parse_front`], the one frontmatter
/// reader, then held to [`ProfileFront`]: an unknown key, a missing key or a
/// kind that is neither `base` nor `layer` is [`TeamsError::Malformed`].
pub fn parse_profile(path: &Path, text: &str) -> Result<(ProfileFront, String), TeamsError> {
  let malformed = |field: &str, detail: String| TeamsError::Malformed {
    path: path.to_path_buf(),
    field: field.to_string(),
    detail,
  };
  let front = crate::rules::parse_front(text);
  if !front.present {
    return Err(malformed(
      "frontmatter",
      "the file opens with no `---` frontmatter block".to_string(),
    ));
  }
  if let Some(key) = front
    .scalars
    .keys()
    .find(|k| !FRONT_KEYS.contains(&k.as_str()))
  {
    return Err(malformed(
      key,
      format!(
        "`{key}` is not a profile key; a profile's frontmatter carries {}",
        FRONT_KEYS.join(", ")
      ),
    ));
  }
  let kind = match front.scalars.get("kind").map(String::as_str) {
    Some("base") => ProfileKind::Base,
    Some("layer") => ProfileKind::Layer,
    Some(other) => {
      return Err(malformed(
        "kind",
        format!("`{other}` is neither `base` nor `layer`"),
      ));
    }
    None => return Err(malformed("kind", "no `kind:` is declared".to_string())),
  };
  let Some(seats) = front.lists.get("seats").cloned() else {
    return Err(malformed(
      "seats",
      "no `seats:` list is declared".to_string(),
    ));
  };
  let mediator = front
    .scalars
    .get("mediator")
    .filter(|m| !m.is_empty())
    .cloned();
  Ok((
    ProfileFront {
      kind,
      mediator,
      seats,
    },
    body_after_front(text),
  ))
}

/// The text after the closing `---` of a frontmatter block, its leading blank
/// lines dropped.
fn body_after_front(text: &str) -> String {
  let Some(rest) = text.strip_prefix("---\n") else {
    return text.to_string();
  };
  let body = if let Some(after) = rest.strip_prefix("---\n") {
    after
  } else {
    rest
      .find("\n---\n")
      .map_or("", |end| &rest[end + "\n---\n".len()..])
  };
  body.trim_start_matches('\n').to_string()
}

/// The registry's own refusals. **PURE.**
pub fn check_registry(path: &Path, registry: &SeatRegistry) -> Vec<Finding> {
  let mut out = Vec::new();
  let mut seen = BTreeSet::new();
  let at = |field: &str, message: String| Finding {
    path: path.to_path_buf(),
    field: field.to_string(),
    message,
  };
  for seat in &registry.seats {
    if !seen.insert(seat.moniker.as_str()) {
      out.push(at(
        "moniker",
        format!(
          "`{}` is held twice; the registry holds each seat once",
          seat.moniker
        ),
      ));
    }
    if seat.role == seat.moniker {
      out.push(at(
        "role",
        format!(
          "`{}`'s role is its moniker; a role is one word for what the seat does, and a moniker is never written as a role",
          seat.moniker
        ),
      ));
    }
    if seat.name.ends_with(&format!("-{}", seat.moniker)) {
      out.push(at(
        "name",
        format!(
          "`{}`'s name `{}` is written as a session name; a name is the seat's display name, eg Control Claude",
          seat.moniker, seat.name
        ),
      ));
    }
  }
  out
}

/// One profile's refusals, against the registry when it could be read. **PURE.**
pub fn check_profile(profile: &Profile, registry: Option<&SeatRegistry>) -> Vec<Finding> {
  let mut out = Vec::new();
  let at = |field: &str, message: String| Finding {
    path: profile.path.clone(),
    field: field.to_string(),
    message,
  };
  let front = &profile.front;
  let base = front.kind == ProfileKind::Base;

  match &front.mediator {
    None if base => out.push(at(
      "mediator",
      "a base names its mediator, the one seat through which hv is reached".to_string(),
    )),
    Some(m) if !front.seats.contains(m) => out.push(at(
      "mediator",
      format!("the mediator `{m}` is not one of this profile's seats"),
    )),
    _ => {}
  }

  let mut seen = BTreeSet::new();
  for moniker in &front.seats {
    if !seen.insert(moniker.as_str()) {
      out.push(at("seats", format!("`{moniker}` is declared twice")));
    }
  }

  if let Some(registry) = registry {
    let mut human = Vec::new();
    for moniker in &front.seats {
      match registry.seats.iter().find(|s| &s.moniker == moniker) {
        None => out.push(at(
          "seats",
          format!("`{moniker}` is not a seat the registry ({SEATS_FILE}) holds"),
        )),
        Some(seat) if seat.driver == Driver::Human => human.push(moniker.as_str()),
        Some(_) => {}
      }
    }
    if !base && let Some(first) = human.first() {
      out.push(at(
        "seats",
        format!("a layer declares no human seat, and `{first}` is driven by the human"),
      ));
    }
    if human.len() > 1 {
      out.push(at(
        "seats",
        format!(
          "{} are each driven by the human; a profile has one human seat",
          human.join(" and ")
        ),
      ));
    }
  }

  let carried = profile.sections();
  if base {
    for missing in SECTIONS.iter().filter(|s| !carried.contains(s)) {
      out.push(at(
        "brief",
        format!("a base carries the section `{missing}`, and this one does not"),
      ));
    }
    if let Some(m) = &front.mediator {
      for section in section_texts(&profile.brief) {
        if section
          .split(|c: char| !c.is_alphanumeric())
          .any(|w| w == m)
        {
          out.push(at(
            "brief",
            format!(
              "a section names the mediator as `{m}`; it names \"the mediator\", because a layer can move it"
            ),
          ));
          break;
        }
      }
    }
  } else {
    for present in &carried {
      out.push(at(
        "brief",
        format!("a layer carries none of the four sections, and this one carries `{present}`"),
      ));
    }
  }
  out
}

/// The text under each of the four headings, to the next H2 or the end.
fn section_texts(brief: &str) -> Vec<String> {
  let mut out = Vec::new();
  let mut open: Option<String> = None;
  for line in brief.lines() {
    if line.starts_with("## ") {
      if let Some(done) = open.take() {
        out.push(done);
      }
      if SECTIONS.contains(&line.trim_end()) {
        open = Some(String::new());
      }
      continue;
    }
    if let Some(text) = open.as_mut() {
      text.push_str(line);
      text.push('\n');
    }
  }
  out.extend(open);
  out
}

// ---------------------------------------------------------------------------
// `wb init`: what a board's profiles declare, and what an apply would do
// (ST0092 WP-03). Pure: the profiles, the declaration and the roster are
// arguments, so every verdict and every refusal is driven without a store.
// ---------------------------------------------------------------------------

/// What a board's profiles declare, resolved: the base and its layers, the
/// mediator, the human seat, and every seat with the registry's identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resolved {
  pub base: String,
  pub layers: Vec<String>,
  pub mediator: String,
  pub human: String,
  /// The base's seats, then each layer's new ones, in that order.
  pub seats: Vec<Seat>,
}

impl Resolved {
  /// The declaration this resolution records, keeping a recorded README
  /// baseline (WP-07 writes it, and an apply never moves it).
  pub fn declaration(
    &self,
    held: Option<&crate::model::TeamDeclaration>,
  ) -> crate::model::TeamDeclaration {
    crate::model::TeamDeclaration {
      schema: crate::model::DECLARATION_SCHEMA.to_string(),
      base: self.base.clone(),
      layers: self.layers.clone(),
      mediator: self.mediator.clone(),
      human: self.human.clone(),
      readme_baseline: held.and_then(|d| d.readme_baseline.clone()),
    }
  }
}

/// What an apply does to one seat.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
  /// The seat is declared and not registered.
  Register,
  /// The seat is registered with another name or role.
  Correct {
    held_name: String,
    held_role: String,
  },
  /// The seat is registered as the profile declares it.
  Unchanged,
  /// A registered node no profile names. It is left as it is.
  Outside,
  /// A retired node, declared or not (ST0092 WP-04). An apply leaves it
  /// retired: it comes back only through `wb register` (vc's ruling (2)), and
  /// a profile is reconciled without it.
  Retired { reason: String },
  /// A seat of the old base that the new one does not declare, which
  /// `--change` retires (ST0092 WP-04). Nothing else retires a seat.
  Retire,
}

/// One seat's line in the plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeatPlan {
  pub moniker: String,
  /// The profile's name and role, or the node's own for one outside it.
  pub name: String,
  pub role: String,
  pub verdict: Verdict,
  /// Whether the seat shows its profile's default remit, having none of its
  /// own. `false` for a node outside the profile, which has no default.
  pub default_remit: bool,
}

/// One registered node as the plan reads it: moniker, name, role and the
/// remit the project set, if any.
pub type RosterRow<'a> = (&'a str, &'a str, &'a str, Option<&'a str>);

impl Teams {
  /// Resolve the profiles `wb init` was named against the board's recorded
  /// declaration (ST0092 WP-03). The library classifies each name by the kind
  /// its profile declares, and every refusal is here: an unknown name, more
  /// than one base, a layer with no base named or declared, a base other than
  /// the declared one without `change`, and no name on a board that records
  /// no declaration.
  pub fn resolve(
    &self,
    names: &[String],
    declared: Option<&crate::model::TeamDeclaration>,
    change: bool,
  ) -> Result<Resolved, TeamsError> {
    let (base, layers) = if names.is_empty() {
      let Some(d) = declared else {
        return Err(TeamsError::NoDeclaration {
          shipped: self.names()?,
        });
      };
      (d.base.clone(), d.layers.clone())
    } else {
      let mut bases = Vec::new();
      let mut layers = Vec::new();
      for name in names {
        match self.profile(name)?.front.kind {
          ProfileKind::Base => bases.push(name.clone()),
          ProfileKind::Layer => layers.push(name.clone()),
        }
      }
      if bases.len() > 1 {
        return Err(TeamsError::TwoBases { bases });
      }
      let base = match (bases.pop(), declared) {
        (Some(named), Some(d)) if named != d.base && !change => {
          return Err(TeamsError::BaseDiffers {
            declared: d.base.clone(),
            named,
          });
        }
        (Some(named), _) => named,
        (None, Some(d)) => d.base.clone(),
        (None, None) => return Err(TeamsError::LayerWithoutBase { layers }),
      };
      (base, layers)
    };

    let registry = self.registry()?;
    let base_profile = self.profile(&base)?;
    let mut mediator = base_profile.front.mediator.clone().unwrap_or_default();
    let mut monikers: Vec<String> = base_profile.front.seats.clone();
    for layer in &layers {
      let p = self.profile(layer)?;
      if let Some(m) = p.front.mediator {
        mediator = m;
      }
      for seat in p.front.seats {
        if !monikers.contains(&seat) {
          monikers.push(seat);
        }
      }
    }
    let seats: Vec<Seat> = monikers
      .iter()
      .filter_map(|m| registry.seats.iter().find(|s| &s.moniker == m).cloned())
      .collect();
    let human = seats
      .iter()
      .find(|s| s.driver == Driver::Human)
      .map(|s| s.moniker.clone())
      .unwrap_or_default();
    Ok(Resolved {
      base,
      layers,
      mediator,
      human,
      seats,
    })
  }
}

/// What an apply would do to each seat, in the profile's order, then each
/// registered node no profile names. **PURE.**
pub fn plan(
  resolved: &Resolved,
  roster: &[RosterRow<'_>],
  retired: &[(&str, &str)],
  retiring: &[&str],
) -> Vec<SeatPlan> {
  let retired_reason = |m: &str| {
    retired
      .iter()
      .find(|(r, _)| *r == m)
      .map(|(_, reason)| (*reason).to_string())
  };
  let mut out: Vec<SeatPlan> = resolved
    .seats
    .iter()
    .map(|seat| {
      let held = roster.iter().find(|(m, ..)| *m == seat.moniker);
      let verdict = match held {
        Some(_) if retired_reason(&seat.moniker).is_some() => Verdict::Retired {
          reason: retired_reason(&seat.moniker).unwrap_or_default(),
        },
        None => Verdict::Register,
        Some((_, name, role, _)) if *name == seat.name && *role == seat.role => Verdict::Unchanged,
        Some((_, name, role, _)) => Verdict::Correct {
          held_name: (*name).to_string(),
          held_role: (*role).to_string(),
        },
      };
      SeatPlan {
        moniker: seat.moniker.clone(),
        name: seat.name.clone(),
        role: seat.role.clone(),
        verdict,
        default_remit: held.is_none_or(|(.., remit)| remit.is_none()),
      }
    })
    .collect();
  for (moniker, name, role, _) in roster {
    if !resolved.seats.iter().any(|s| s.moniker == *moniker) {
      out.push(SeatPlan {
        moniker: (*moniker).to_string(),
        name: (*name).to_string(),
        role: (*role).to_string(),
        verdict: match retired_reason(moniker) {
          Some(reason) => Verdict::Retired { reason },
          None if retiring.contains(moniker) => Verdict::Retire,
          None => Verdict::Outside,
        },
        default_remit: false,
      });
    }
  }
  out
}

/// The identity a declared moniker must register with, or `None` when the
/// declaration does not name it (ST0092 WP-03, AC-03.6): `register` honours a
/// board's declaration through this.
pub fn declared_identity(resolved: &Resolved, moniker: &str) -> Option<(String, String)> {
  resolved
    .seats
    .iter()
    .find(|s| s.moniker == moniker)
    .map(|s| (s.name.clone(), s.role.clone()))
}

impl crate::facade::Facade {
  /// The `wb profile list` row on the MCP tier. It touches no store: the
  /// profiles belong to the install, and [`Teams`] is the one home.
  pub fn wb_profile_list(&self) -> Result<Vec<Profile>, crate::facade::FacadeError> {
    Ok(installed()?.profiles()?)
  }

  /// The `wb profile show` row on the MCP tier, with the registry rows for its
  /// seats.
  pub fn wb_profile_show(
    &self,
    name: &str,
  ) -> Result<(Profile, Vec<Seat>), crate::facade::FacadeError> {
    let teams = installed()?;
    let profile = teams.profile(name)?;
    let registry = teams.registry()?;
    Ok((profile.clone(), seats_of(&profile, &registry)))
  }

  /// The `wb profile validate` row on the MCP tier.
  pub fn wb_profile_validate(&self) -> Result<(Vec<Finding>, usize), crate::facade::FacadeError> {
    Ok(installed()?.validate()?)
  }

  /// The `wb profile terms` row on the MCP tier.
  pub fn wb_profile_terms(&self) -> Result<String, crate::facade::FacadeError> {
    Ok(installed()?.terms()?)
  }
}

/// The profiles of the install this binary runs from.
pub fn installed() -> Result<Teams, crate::install::InstallError> {
  Ok(Teams::new(&crate::install::home()?))
}

/// The registry rows for a profile's seats, in the profile's order. A moniker
/// the registry does not hold is left out here; `validate` names it.
pub fn seats_of(profile: &Profile, registry: &SeatRegistry) -> Vec<Seat> {
  profile
    .front
    .seats
    .iter()
    .filter_map(|m| registry.seats.iter().find(|s| &s.moniker == m).cloned())
    .collect()
}
