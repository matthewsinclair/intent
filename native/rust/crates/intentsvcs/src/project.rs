//! The project: a root directory, its config, and the canonical location of
//! every artefact under it.
//!
//! **This is the one place paths are decided.** v2's defect class was that
//! every reader reimplemented parsing and path resolution, so every reader
//! answered confidently from partial evidence; the fix is not "be careful", it
//! is having exactly one module that can answer "where does a thread live".
//! Nothing else in intentsvcs joins a path by hand.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::model::Attachment;

/// The file extensions carried as [`crate::model::Attachment`]s.
///
/// **A LIST, so the question is decidable without opening the file**, and
/// extending it is an explicit act rather than a classifier quietly changing
/// its mind. Nothing here inspects content or forms a view about whether a
/// file "feels authored".
///
/// **THE PRINCIPLE THE LIST ENCODES: no tool can make this again, versus a
/// tool made this and can again** (vc, on measuring the estate). It is the
/// authorship axis the view/attachment split already runs on, one level down,
/// which makes it one idea in two places rather than two rules.
///
/// So `.sh` is IN. The shell here is hand-authored and unreproducible -- on
/// this project it is the instruments that verify the migration, including the
/// one whose whole job is to prove content was not lost, and a clone of the
/// canon would not have contained the tools that prove the canon. Generated
/// baselines stay out: a tool's committed output is regenerable, so carrying
/// it buys nothing.
///
/// **One consequence carried openly rather than solved: a mode bit does not
/// survive.** `text` is content, so an executable written back from the store
/// arrives without its `+x`. That was the original reason to exclude
/// executables and it has not stopped being true -- it is now outweighed,
/// because a script that has to be `chmod +x` is recoverable and a script
/// nobody kept is not.
pub const ATTACHMENT_EXTENSIONS: &[&str] = &["md", "txt", "sh"];

/// What a file sitting under a thread's directory is.
///
/// These partition the directory: every file is exactly one, and `Unattached`
/// is the named remainder rather than a silent gap. **`doctor` reports the
/// remainder by path, which is the property the whole scheme rests on** -- a
/// disk that becomes optional destroys whatever nothing said was uncovered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadFile {
  /// Rendered from the model: the thread's cover, its acceptance contract, a
  /// work package's cover.
  GeneratedView,
  /// `thread.json` -- the committed canon itself.
  Canon,
  /// Carried verbatim under [`ATTACHMENT_EXTENSIONS`].
  Attachment,
  /// Everything else. Reported by name, never silently skipped.
  Unattached,
}

/// The per-project config (`intent/.config/config.json`).
///
/// Unknown fields are PERMITTED here, and that is deliberate rather than an
/// oversight of D05. config.json is the one canon file v2 also writes, plugins
/// extend it with their own blocks, and a v3 binary that refused a config it
/// did not fully recognise would brick a project the moment any other tool
/// touched it. Strictness belongs on the artefacts v3 owns outright.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
  pub intent_version: String,
  #[serde(default)]
  pub project_name: String,
  #[serde(default)]
  pub author: String,
  /// Stamped at migration (D15); absent on a v2 project, which is how the
  /// migrator tells "not yet migrated" from "migrated".
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub project_id: Option<String>,
  #[serde(default = "default_intent_dir")]
  pub intent_dir: String,
  #[serde(default)]
  pub languages: Vec<String>,
  /// The `todo` block (D44). Absent in every v2 config, hence the default.
  #[serde(default)]
  pub todo: TodoConfig,
  /// Everything else in the file, carried so a rewrite never drops a block
  /// this version does not know about.
  ///
  /// **`st_prefix` lands here, and that is the retirement working rather than
  /// a leak** (issue 0040, hv). The field is gone from the type -- the prefix
  /// is fixed at `crate::model::THREAD_PREFIX` -- so a v2 project that
  /// declared it keeps the declaration in the file, byte for byte, instead of
  /// having it silently dropped on the first rewrite. v3 does not honour it,
  /// and [`crate::legacy`] says so out loud when the value is not the default.
  /// Retiring a knob nobody uses is fine; retiring it under someone who does,
  /// without telling them, is the silent data change this thread exists to
  /// prevent.
  #[serde(flatten)]
  pub extra: serde_json::Map<String, serde_json::Value>,
}

/// The `todo` block: how much of the DONE bucket a TERMINAL render shows.
///
/// **Configuration, not state, and that is the whole of D44.** v2 kept a
/// watermark inside the generated file and read it back out, which made a
/// disposable view its own database -- deleting it silently resurrected every
/// flushed item. There is nothing durable here: every completed date is
/// already in the model, so the DONE bucket is a QUESTION asked at render time
/// rather than a set anyone maintains.
///
/// **It is config rather than a flag because all six `todo` verbs regenerate
/// the file**, so a flag on any one of them is a silent-revert generator -- the
/// next verb rewrites without it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodoConfig {
  /// How far back the terminal's DONE bucket reaches. Default 24.
  ///
  /// **The unit is hours and the resolution is a DAY, which is a property of
  /// the data rather than of this field.** `steel_thread.completed` is a
  /// domain date -- `YYYY-MM-DD`, no time component, carried from v2 and never
  /// re-stamped -- so a cutoff finer than a day has nothing to bite on. The
  /// unit stays hours because that is what D44 ruled and what a longer window
  /// wants to be expressed in.
  ///
  /// **A value the data cannot honour is REFUSED rather than rounded** -- see
  /// [`TodoConfig::window`], which is the only supported way to read this.
  #[serde(default = "default_window_hours")]
  pub window_hours: u32,
}

fn default_window_hours() -> u32 {
  24
}

impl Default for TodoConfig {
  fn default() -> Self {
    Self {
      window_hours: default_window_hours(),
    }
  }
}

/// A configured window the data cannot honour.
///
/// Its own type rather than a string, because two callers need it -- the render
/// path refuses with it and `doctor` reports it -- and a message assembled
/// twice is a message that drifts once.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error(
  "todo.window_hours is {configured}, which is not a whole number of days -- steel thread completion is recorded as a date with no time component, so a cutoff \
   of {configured}h is truncated to a date and the window means different things at different times of day"
)]
pub struct UnhonourableWindow {
  /// What `config.json` asked for.
  pub configured: u32,
  /// The finest interval the data can distinguish, from
  /// [`crate::model::COMPLETED_RESOLUTION_HOURS`].
  pub resolution: u32,
}

impl UnhonourableWindow {
  /// The rule itself, with the resolution passed in rather than read.
  ///
  /// **Split out so the self-retirement claim is TESTABLE instead of merely
  /// asserted in a comment.** A test can hand this a resolution of `1` -- what
  /// [`crate::model::COMPLETED_RESOLUTION_HOURS`] becomes the day `completed`
  /// gains a time component -- and observe that nothing at all is refused. A
  /// guard that says in prose that it will retire itself is a guard nobody
  /// checks; this one is measured.
  ///
  /// **A resolution of `0` or `1` refuses nothing, and the `0` half is the
  /// guard rather than a courtesy.** `is_multiple_of(0)` is `self == 0`, so
  /// without the short-circuit a resolution of zero would refuse EVERY non-zero
  /// window -- the failure inverted, on the one edit this code exists to
  /// anticipate.
  pub fn check(window_hours: u32, resolution: u32) -> Result<u32, Self> {
    if resolution <= 1 || window_hours.is_multiple_of(resolution) {
      Ok(window_hours)
    } else {
      Err(Self {
        configured: window_hours,
        resolution,
      })
    }
  }

  fn remedy_text(&self) -> String {
    let down = self.configured - (self.configured % self.resolution);
    let up = down + self.resolution;
    format!(
      "set todo.window_hours to a whole multiple of {} -- {down} or {up} -- in intent/.config/config.json",
      self.resolution
    )
  }
}

impl crate::remedy::Remedy for UnhonourableWindow {
  fn remedy(&self) -> String {
    self.remedy_text()
  }
}

impl TodoConfig {
  /// The configured window, or the reason the data cannot honour it.
  ///
  /// **THE ONLY SUPPORTED READ OF `window_hours`, and the refusal is vc's
  /// ruling** (2026-08-17). Two spellings were offered and both rejected:
  /// `window_days`, because it forecloses `completed` gaining a time component,
  /// which is live -- the field is date-resolution because v2 was, and nothing
  /// rules v3 must stay so; and hours-with-a-comment, because that leaves a
  /// config value silently meaning something other than it says.
  ///
  /// **The failure it prevents is worse than rounding, which is how it was
  /// first described.** The cutoff is `date('now', '-Nh')`, truncated to a
  /// date -- so at 02:00 a 6-hour window reaches back into yesterday and at
  /// 12:00 it does not. The same configuration produces different DONE buckets
  /// depending on the hour it is read at, with nothing on screen to say why.
  /// A named refusal at the point of use beats a silent divergence between what
  /// a setting says and what it does, which is the class this thread has now
  /// found six times in two days.
  ///
  /// **It retires itself** -- see [`crate::model::COMPLETED_RESOLUTION_HOURS`].
  ///
  /// Zero is honourable and deliberately not special-cased into "show
  /// everything": it is a whole number of days and it means what it says, a
  /// DONE bucket reaching back to the start of today.
  pub fn window(&self) -> Result<u32, UnhonourableWindow> {
    UnhonourableWindow::check(self.window_hours, crate::model::COMPLETED_RESOLUTION_HOURS)
  }
}

/// The retired `st_prefix` key, by the name it has in `config.json`.
///
/// Named once so the migrator's check and any later reader agree about what
/// they are looking for. There is deliberately no `default_st_prefix()` beside
/// it: a default would be a second place the prefix is decided.
pub const RETIRED_ST_PREFIX_KEY: &str = "st_prefix";

fn default_intent_dir() -> String {
  "intent".to_string()
}

#[derive(Debug, thiserror::Error)]
pub enum ProjectError {
  #[error("no Intent project found at or above {0} (looked for intent/.config/config.json)")]
  NotFound(String),
  #[error("reading {path}: {source}")]
  Io {
    path: String,
    #[source]
    source: std::io::Error,
  },
  #[error("{path} is not valid Intent config: {source}")]
  Config {
    path: String,
    #[source]
    source: serde_json::Error,
  },
}

impl crate::remedy::Remedy for ProjectError {
  fn remedy(&self) -> String {
    match self {
      Self::NotFound(_) => {
        "run `intent init` here, or change to a directory inside an Intent project".to_string()
      }
      Self::Io { path, .. } => {
        format!("check that {path} exists and that this user can read it")
      }
      // The parse error above carries the line and column, which is the whole
      // of the action -- and the second clause matters more than the first:
      // this file is committed, so git holds the version before the edit.
      Self::Config { path, .. } => format!(
        "the parse error above names the position in {path}; if you did not mean to edit it, `git checkout -- {path}` restores the committed version"
      ),
    }
  }
}

/// The v2 release a project must already be at before v3 can migrate it (D09,
/// migration.md). Below it, v2's own `intent upgrade` runs first -- v3 never
/// reimplements the v2 ledger.
pub const MIGRATION_FLOOR: (u64, u64, u64) = (2, 19, 0);

/// One thread's canon path, relative to the intent directory (D57-1).
///
/// **ONE SPELLING, TWO CONSUMERS, AND THE SECOND ONE IS WHY THIS IS A
/// FUNCTION.** [`Project::thread_json`] resolves it against a root; the
/// exporter emits it as the name INSIDE the portable extract, promising that
/// "the paths are the real ones so a refusal names a file the operator can go
/// and look at". Those were two independent `format!` calls, and the issue arm
/// beside it had already shipped the resulting defect -- `issues/46.json`
/// written where every reader opened `issues/0046.json`. Two ends agreeing by
/// convention is how that happens; agreeing by construction is this.
pub fn canon_thread_rel(id: &str) -> String {
  format!(".canon/st/{id}.json")
}

/// One OPAQUE attachment's canon path, relative to the intent directory
/// (ST0057 AC-03.1, D57-7).
///
/// **A function for the same reason [`canon_thread_rel`] is one, and the
/// consumers are further apart here**: the exporter names the file, `ingest`
/// opens it, and `organize` will one day copy it back onto disk. Three
/// `format!` calls agreeing by convention is exactly the shape that shipped
/// `issues/46.json`, and a disagreement here is worse than that one was -- an
/// unreadable path means the bytes are gone, not merely unfindable, because
/// the working copy is what dehydration removed.
///
/// **The thread's canon file and its attachment directory are SIBLINGS with
/// the same stem**: `.canon/st/ST0056.json` beside `.canon/st/ST0056/`. That
/// is not a collision -- one is a file and one is a directory -- and it keeps
/// a thread's whole record in one place a reader can list.
pub fn canon_blob_rel(id: &str, path: &str) -> String {
  format!(".canon/st/{id}/{path}")
}

/// Why an attachment path cannot be carried (ST0057 AC-03.3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BadName {
  /// Its canon path would land outside the thread's own canon directory.
  EscapesCanon { would_be: String },
  /// It is not already normalised, so two different attachment paths would
  /// share one canon file and the second write would destroy the first.
  Collides { would_be: String },
  /// It cannot be addressed: the URL built from it does not parse back to the
  /// same path.
  NotAddressable { url: String, reason: String },
}

impl std::fmt::Display for BadName {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::EscapesCanon { would_be } => write!(
        f,
        "its canon copy would be written to {would_be}, outside the thread's own directory"
      ),
      Self::Collides { would_be } => write!(
        f,
        "it is not already normalised: its canon copy would be written to {would_be}, which is \
         also where the attachment actually named that would go -- two paths, one file, and the \
         second write destroys the first"
      ),
      Self::NotAddressable { url, reason } => {
        write!(f, "it cannot be addressed -- {url} {reason}")
      }
    }
  }
}

/// Whether an attachment path can be given BOTH a canon path and a URL
/// (ST0057 AC-03.3, D57-8).
///
/// # It is DERIVED from the two consumers, never a list of bad characters
///
/// A hand-written character list is a second opinion about what those two
/// mechanisms accept, and a second opinion drifts the moment either of them
/// changes -- silently, and in the direction that admits a name one of them
/// cannot take. So each half is answered by ASKING the mechanism:
///
/// - **The canon half** builds the real path through [`canon_blob_rel`] and
///   requires it to stay inside the thread's own canon directory once `.` and
///   `..` are resolved. This is not a tidiness rule: hydration WRITES to that
///   path, so a name that escapes it is a write outside the thread, chosen by
///   whoever named the file.
/// - **The URL half** builds `intent:///threads/{id}/attachments/{path}` and
///   requires [`crate::address::parse`] to return the SAME path. Round-tripping
///   through the real parser is what stops this rule and the addressing layer
///   disagreeing: a path that survives the trip is addressable by construction,
///   and one that does not is rejected for the parser's own reason rather than
///   for a reason invented here.
///
/// # Lexical resolution, deliberately, and it is the stricter answer
///
/// `..` is resolved textually rather than by asking the filesystem. A
/// filesystem answer depends on what exists at the moment of asking -- a
/// symlink, a directory not yet created -- so the same name could be legal on
/// one machine and not another, and the check would pass on the estate that
/// wrote it and refuse on the clone that received it.
///
/// # WHAT THIS DOES NOT CATCH, said out loud
///
/// **The URL half is exactly as strict as [`crate::address::parse`], which is
/// the point and also the limit.** Measured against the accepted set: a name
/// containing a SPACE, a `#`, or a control character such as a newline is
/// ACCEPTED, because the parser takes them. A `#` would truncate the address in
/// any standard URL reader, and a newline breaks every line-oriented tool that
/// ever handles the path.
///
/// **That is not a hole to patch here.** Adding a character list would rebuild
/// the second opinion this function exists to avoid, and it would then disagree
/// with the addressing layer in the direction where a name is refused although
/// the parser would happily take it. **If those names should be refused, the
/// parser is where it belongs -- and then both consumers gain it at once, which
/// is the whole return on deriving the rule.** Reported to whoever owns D57-8
/// rather than worked around.
///
/// **So a green from this function means "canon and the addressing layer can
/// both take this name", and does NOT mean "this name is safe in a shell".**
/// The two are different claims and only the first is being made.
pub fn attachment_name(thread: &str, path: &str) -> Result<(), BadName> {
  let rel = canon_blob_rel(thread, path);
  let prefix = format!(".canon/st/{thread}/");

  // Resolve `.` and `..` lexically. A leading `..` pops past the prefix and the
  // containment test below fails, which is the whole point.
  let mut resolved: Vec<&str> = Vec::new();
  for part in rel.split('/') {
    match part {
      "." | "" => {}
      ".." => {
        resolved.pop();
      }
      other => resolved.push(other),
    }
  }
  let resolved = resolved.join("/");
  // **RESOLUTION MUST BE A NO-OP, not merely land inside the directory, and the
  // difference is a collision rather than a nicety.** `a/./b.md` resolves to
  // `a/b.md`, which is contained -- and is the SAME canon file as the
  // attachment actually named `a/b.md`. Two distinct attachment paths, one
  // sidecar, and the second write silently destroys the first. Requiring the
  // path to be already-normalised catches that, and catches `..`, `//`,
  // a trailing separator and the empty path in the same rule instead of four.
  //
  // Found by probing the accepted set rather than by reading this function: the
  // containment test passed `a/./b.md` and looked correct doing it.
  // **CONTAINMENT FIRST, and the order is not cosmetic.** Both tests fire for
  // `../x.md`, and only one of them describes it: it escapes the thread, and it
  // also happens to be un-normalised. Answering "collision" for a path
  // traversal sends an operator looking for a duplicate that does not exist,
  // and buries the write-outside-the-thread finding under a tidiness one.
  //
  // The first version had these the other way round and every escape in the
  // probe set came back mislabelled -- correct refusals, wrong reasons, which
  // is the shape a reader trusts and acts on.
  if !resolved.starts_with(&prefix) || resolved.len() == prefix.len() {
    return Err(BadName::EscapesCanon { would_be: resolved });
  }
  // **Reported as a COLLISION and not as an escape, because `a/./b.md` resolves
  // INSIDE the directory.** Calling that "outside the thread" would describe a
  // fault the file does not have.
  if resolved != rel {
    return Err(BadName::Collides { would_be: resolved });
  }

  let url = format!("intent:///threads/{thread}/attachments/{path}");
  match crate::address::parse(&url) {
    Ok(address) => match &address.entity {
      crate::address::Entity::Attachment {
        path: round_tripped,
        ..
      } if round_tripped == path => Ok(()),
      crate::address::Entity::Attachment {
        path: round_tripped,
        ..
      } => Err(BadName::NotAddressable {
        url,
        reason: format!("reads back as {round_tripped:?} rather than {path:?}"),
      }),
      other => Err(BadName::NotAddressable {
        url,
        reason: format!("reads back as {other:?} rather than as an attachment"),
      }),
    },
    Err(e) => Err(BadName::NotAddressable {
      url,
      reason: format!("does not parse: {e}"),
    }),
  }
}

/// One issue's canon path, relative to the intent directory. Zero-padded,
/// which is the half that was wrong before.
pub fn canon_issue_rel(number: u32) -> String {
  format!(".canon/issues/{number:04}.json")
}

/// Whether this project's canon is in a form THIS binary can read.
///
/// The question exists because "no threads found" and "threads this binary
/// cannot see" are the same empty vector, and answering the second as though
/// it were the first is a confident lie about someone's work (AC-10.7).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Migration {
  /// v3 canon, or a genuinely empty project.
  Done,
  Pending(Pending),
}

/// A project v3 must not answer questions about yet, and everything the
/// operator needs to tell this state from an empty estate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pending {
  /// What `config.json` declares, verbatim -- reported as it stands rather
  /// than normalised, because the operator has to find it in the file.
  pub declared: String,
  /// Below [`MIGRATION_FLOOR`], so the remedy is the two-hop rather than a v3
  /// migration that would refuse.
  pub below_floor: bool,
  /// Thread ids carrying v2 canon this binary cannot read, sorted.
  pub legacy_threads: Vec<String>,
}

impl Pending {
  /// What the operator should DO -- and the two states must not share a text,
  /// because one of them names a command that will refuse them.
  pub fn remedy(&self) -> String {
    if self.below_floor {
      let (major, minor, patch) = MIGRATION_FLOOR;
      format!(
        "this project is below the v{major}.{minor}.{patch} migration floor -- run `install intent@2 && intent upgrade` first, then migrate it with v3"
      )
    } else {
      "run `intent upgrade` to migrate this project to Intent v3".to_string()
    }
  }
}

impl std::fmt::Display for Pending {
  /// One line, inside the existing refusal grammar. A second report shape
  /// would be a second thing to learn for the same job.
  ///
  /// It deliberately does NOT name `config.json`. The remedy is a command, so
  /// the path buys the operator nothing -- and it invites the one repair that
  /// makes things worse: hand-editing `intent_version` to 3.0.0 produces a
  /// config claiming v3 over canon that is not, which is the half-migrated
  /// estate this whole check exists to catch. `doctor` still points at the
  /// file, because pointing at artefacts is what `doctor` is for and its
  /// finding carries the path in its own field.
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "this project has not been migrated to Intent v3 -- it declares Intent {}",
      self.declared
    )?;
    match self.legacy_threads.len() {
      0 => Ok(()),
      // Named, not just counted: a count alone is indistinguishable from a
      // count of nothing, and the ids are how the operator recognises their
      // own work. Capped, because this repository has 56 of them at f7434f1
      // and an error message is not a listing.
      n => write!(
        f,
        ", and {n} steel thread{} carr{} v2 canon this binary cannot read ({})",
        if n == 1 { "" } else { "s" },
        if n == 1 { "ies" } else { "y" },
        summarise(&self.legacy_threads)
      ),
    }
  }
}

fn summarise(ids: &[String]) -> String {
  const SHOWN: usize = 3;
  if ids.len() <= SHOWN {
    return ids.join(", ");
  }
  format!(
    "{}, and {} more",
    ids[..SHOWN].join(", "),
    ids.len() - SHOWN
  )
}

/// Walk `dir` for v2 thread directories, descending exactly one level into
/// anything that is not itself a thread -- which is what v2's `st/<STATUS>/`
/// archive directories are.
///
/// One level, not arbitrary depth: v2 has exactly this shape, and a full walk
/// would start reporting thread-shaped directories from unrelated trees
/// (a vendored checkout, a fixture directory) as this project's own canon.
fn collect_legacy(dir: &std::path::Path, descend: bool, out: &mut Vec<String>) {
  let Ok(entries) = std::fs::read_dir(dir) else {
    return;
  };
  for path in entries.filter_map(Result::ok).map(|e| e.path()) {
    if !path.is_dir() || path.join("thread.json").is_file() {
      continue;
    }
    match path.join("info.md").is_file() {
      true => out.extend(
        path
          .file_name()
          .and_then(|s| s.to_str())
          .map(str::to_string),
      ),
      false if descend => collect_legacy(&path, false, out),
      false => {}
    }
  }
}

/// `major.minor.patch`, ignoring any pre-release suffix.
///
/// Returns `None` rather than guessing. An unparseable version means the
/// declaration ABSTAINS -- the evidence check still applies -- because a typo
/// in one config field must not brick the tool, and a version we cannot read
/// is not evidence of anything in either direction.
fn parse_version(text: &str) -> Option<(u64, u64, u64)> {
  let core = text.split(['-', '+']).next()?;
  let mut parts = core.split('.').map(str::parse::<u64>);
  match (parts.next(), parts.next(), parts.next()) {
    (Some(Ok(major)), Some(Ok(minor)), Some(Ok(patch))) => Some((major, minor, patch)),
    _ => None,
  }
}

/// A resolved project.
#[derive(Debug, Clone)]
pub struct Project {
  root: PathBuf,
  config: Config,
}

impl Project {
  /// `intent/.config/config.json` under `root`. The marker file's location is
  /// FIXED -- it is what identifies a project, so it cannot itself be read
  /// from the project's config.
  pub fn config_path(root: &Path) -> PathBuf {
    root.join("intent").join(".config").join("config.json")
  }

  /// Open the project rooted exactly at `root`.
  pub fn open(root: &Path) -> Result<Self, ProjectError> {
    let path = Self::config_path(root);
    let text = std::fs::read_to_string(&path).map_err(|source| ProjectError::Io {
      path: path.display().to_string(),
      source,
    })?;
    let config = serde_json::from_str(&text).map_err(|source| ProjectError::Config {
      path: path.display().to_string(),
      source,
    })?;
    Ok(Self {
      root: root.to_path_buf(),
      config,
    })
  }

  /// Walk up from `start` to the first directory holding an Intent config.
  ///
  /// The marker is the config file's own presence -- never an environment
  /// variable. A generic env var answers "did someone set a variable?" while
  /// meaning "am I in a project?", and those agree often enough to look correct
  /// and differ exactly when it matters (issue 0025).
  pub fn discover(start: &Path) -> Result<Self, ProjectError> {
    for dir in start.ancestors() {
      if Self::config_path(dir).is_file() {
        return Self::open(dir);
      }
    }
    Err(ProjectError::NotFound(start.display().to_string()))
  }

  pub fn root(&self) -> &Path {
    &self.root
  }

  /// This project's spelling of [`relative`].
  pub fn relative(&self, path: &Path) -> String {
    relative(&self.root, path)
  }

  pub fn config(&self) -> &Config {
    &self.config
  }

  /// `intent/` -- configurable in v2, so read rather than assumed.
  pub fn intent_dir(&self) -> PathBuf {
    self.root.join(&self.config.intent_dir)
  }

  pub fn st_dir(&self) -> PathBuf {
    self.intent_dir().join("st")
  }

  /// `intent/st/<ID>/` -- a thread's directory, whether or not it exists.
  pub fn thread_dir(&self, id: &str) -> PathBuf {
    self.st_dir().join(id)
  }

  /// `intent/.intentfiles` -- the realisation manifest (AC-02.1..02.5).
  ///
  /// **Under `intent_dir`, so it MOVES with a configured one**, which is the
  /// difference between it and [`Project::config_path`]. That one is a fixed
  /// bootstrap point by design -- something has to be findable before anything
  /// is configured. Nothing has to find the manifest before the project is
  /// open, so it has no claim on a fixed location and takes the ordinary rule
  /// instead. `Fixture::with_intent_dir` is what catches the alternative: a
  /// site spelling `intent/.intentfiles` itself lands somewhere else entirely.
  pub fn intentfiles_path(&self) -> PathBuf {
    self.intent_dir().join(".intentfiles")
  }

  /// `intent/.canon/` -- every artefact's committed structured canon (D57-1).
  ///
  /// **A DOT DIRECTORY THAT MUST BE COMMITTED, AND IT IS THE ONLY ONE.** The
  /// three siblings beside it -- `.treeindex/`, `.cache/`, `.backup/` -- are
  /// every one of them gitignored, so `intent/.<x>/` currently reads as "local,
  /// never travels". This breaks that pattern deliberately: hv's requirement is
  /// that `intent/st` stop holding a bajillion files, and D29 requires canon to
  /// travel. **A future tidy-up adding `intent/.*/` to `.gitignore` would
  /// silently un-commit the entire estate**, which is why AC-01.2 checks by
  /// CLONING rather than by reading the ignore file: the question is what git
  /// DOES, not what a rule appears to say.
  pub fn canon_dir(&self) -> PathBuf {
    self.intent_dir().join(".canon")
  }

  /// The committed structured canon for one thread, `.canon/st/<ID>.json`.
  ///
  /// **One file per artefact, not one consolidated file** (D57-1 rejects
  /// option B). Four nodes commit into this estate; a single `threads.jsonl`
  /// is a merge-conflict generator, and per-artefact means two nodes editing
  /// two threads touch two paths and never collide.
  pub fn thread_json(&self, id: &str) -> PathBuf {
    self.intent_dir().join(canon_thread_rel(id))
  }

  /// `.canon/st/` -- thread canon, flat, one file per thread.
  pub fn canon_st_dir(&self) -> PathBuf {
    self.canon_dir().join("st")
  }

  /// `.canon/issues/` -- issue canon.
  ///
  /// **The whole directory moved, because the whole directory WAS canon.**
  /// `intent/issues/` held nothing but `<nnnn>.json`; unlike a thread, an
  /// issue has no realised markdown to leave behind.
  pub fn issues_dir(&self) -> PathBuf {
    self.canon_dir().join("issues")
  }

  /// What a file under a thread's directory IS, given its path relative to
  /// that directory.
  ///
  /// **One classifier, because the alternative is three lists that drift.**
  /// Ingest needs to know what to carry, `doctor` needs to know what is
  /// uncovered, and the renderer needs to know what it owns -- and the moment
  /// those are three separate answers, a file can be an attachment to one and
  /// a view to another. Every caller asks here.
  pub fn classify(rel: &Path) -> ThreadFile {
    let name = rel.file_name().and_then(|n| n.to_str()).unwrap_or_default();
    let depth = rel.components().count();

    // A generated view is checked FIRST and by its full shape, not its name.
    // `info.md` at the thread root is the generated cover; `WP/01/info.md` is
    // the work package's. A file called `info.md` three levels down under a
    // parity directory is neither, and matching on the bare name would take
    // an author's file and call it ours.
    if depth == 1 && (name == "info.md" || name == "acceptance.md") {
      return ThreadFile::GeneratedView;
    }
    if depth == 3 && name == "info.md" && rel.starts_with("WP") {
      return ThreadFile::GeneratedView;
    }
    // **NOT dead after D57-1's relocation, and worth saying so.** Canon now
    // lives at `.canon/st/<ID>.json`, so no healthy thread directory holds a
    // `thread.json` -- but a v2 tree does, and so does a tree caught mid-move.
    // Classifying it as canon is what keeps it out of the attachment carry;
    // deleting the arm would make a stale canon file `Unattached` and invite
    // some later reader to treat it as an author's.
    if depth == 1 && name == "thread.json" {
      return ThreadFile::Canon;
    }

    let ext = rel
      .extension()
      .and_then(|e| e.to_str())
      .unwrap_or_default()
      .to_ascii_lowercase();
    if ATTACHMENT_EXTENSIONS.contains(&ext.as_str()) {
      ThreadFile::Attachment
    } else {
      ThreadFile::Unattached
    }
  }

  /// The committed structured canon for one issue, `.canon/issues/<nnnn>.json`.
  pub fn issue_json(&self, number: u32) -> PathBuf {
    self.intent_dir().join(canon_issue_rel(number))
  }

  /// The SQLite db (D21) -- gitignored, and the durable SSOT rather than a
  /// cache. Gitignored is about TRANSPORT, not authority: the committed
  /// extract is how the estate travels, and the store is re-created from it on
  /// a fresh clone.
  pub fn db_path(&self) -> PathBuf {
    self.intent_dir().join(".cache").join("intent.db")
  }

  /// The event log's committed file form (D34, AC-02.6).
  /// The thread index view. Beside the threads it indexes, at v2's path (vc
  /// ruling, 2026-08-14): a rewrite that already moves plenty does not need to
  /// move this too.
  pub fn steel_threads_view(&self) -> PathBuf {
    self.st_dir().join("steel_threads.md")
  }

  /// The flat DOING / TODO / DONE view.
  pub fn todo_view(&self) -> PathBuf {
    self.intent_dir().join("todo.md")
  }

  /// A thread's generated cover.
  pub fn info_view(&self, id: &str) -> PathBuf {
    self.thread_dir(id).join("info.md")
  }

  /// A work package's generated cover, `st/<ID>/WP/<NN>/info.md`.
  pub fn wp_info_view(&self, id: &str, seq: u32) -> PathBuf {
    self
      .thread_dir(id)
      .join("WP")
      .join(format!("{seq:02}"))
      .join("info.md")
  }

  /// A thread's generated acceptance contract + coverage map.
  pub fn acceptance_view(&self, id: &str) -> PathBuf {
    self.thread_dir(id).join("acceptance.md")
  }

  /// Every file under a thread's directory, sorted, each as a path RELATIVE to
  /// that directory -- the address [`Attachment::path`] carries.
  ///
  /// Recursive, so `parity/cmd-st.md` and `WP/01/notes.md` are reached without
  /// a per-level collection. **Gitignored paths are excluded (D29): a path git
  /// does not carry cannot be canon**, which is also what keeps a stray
  /// `.DS_Store` out of the report by rule rather than by a special case.
  /// Every carried attachment under a thread's directory, plus the files that
  /// could not be carried and why.
  ///
  /// **THE ONE COLLECTOR, and "one" is the requirement rather than a tidiness
  /// preference** (vc, condition 2). The migrator grew this walk first and
  /// `sync` needed the same one; two walks over the same directory answering
  /// the same question drift, and the drift would land exactly where it just
  /// cost us -- content carried when it arrives by migration and dropped when
  /// a person types it, with nothing comparing the two paths.
  ///
  /// **Refusals are RETURNED, not recorded**, because the two callers classify
  /// them differently: the migrator files them against a thread's open/closed
  /// disposition, and `sync` has no such axis. Returning `(path, reason)`
  /// pairs lets each build its own finding without this function knowing about
  /// either. A file that cannot be read is NEVER silently skipped -- it comes
  /// back named, which is the posture `sync` already takes on undecodable
  /// content.
  pub fn collect_attachments(&self, id: &str) -> (Vec<Attachment>, Vec<(String, String)>) {
    let dir = self.thread_dir(id);
    let mut carried = Vec::new();
    let mut refused = Vec::new();
    for rel in self.thread_files(id) {
      // Consumed by the parsers -- carrying them here as well would give one
      // file two homes in the model.
      if Project::classify(&rel) != ThreadFile::Attachment {
        continue;
      }
      let path = dir.join(&rel);
      let name = self.relative(&path);
      // **THE NAMING GATE, AT INGEST** (ST0057 AC-03.3). A path that cannot be
      // given both a canon path and a URL is refused HERE, which is the only
      // door into the model -- so a name nothing could store or address never
      // becomes canon in the first place.
      //
      // **REJECTION IS NOT RETROACTIVE, and that falls out rather than being
      // arranged.** Refusing here means the file is not carried; `organize`
      // then meets a `ThreadFile::Attachment` the store does not hold and
      // reports it UNCLAIMED at row five, which it never removes because it is
      // the only copy. So an existing violator is named on every run and left
      // exactly where its author put it.
      let attachment_rel = rel.to_string_lossy().replace('\\', "/");
      if let Err(bad) = crate::project::attachment_name(id, &attachment_rel) {
        refused.push((name, bad.to_string()));
        continue;
      }
      match std::fs::read(&path) {
        // **FORM FOLLOWS CONTENT, and this is the only place that decides it**
        // (ST0057 AC-03.2). Valid UTF-8 is carried inline as `text`; anything
        // else is carried as bytes, in a sidecar file under canon.
        //
        // **It is decided by DECODING, never by the extension.** The extension
        // already answered a different question one step above -- whether the
        // file is carried at all -- and reusing it here would make `.sh` mean
        // "text" for a shell script with one Latin-1 byte in a comment, which
        // is precisely the file that would be silently mangled.
        //
        // **This arm used to REFUSE**, with "not valid UTF-8, so it cannot be
        // carried as text", and the refusal was right for as long as canon had
        // nowhere to put the bytes. It now has one, and a refusal here would
        // leave the file uncarried, therefore unrecoverable, therefore pinned
        // to disk forever -- the condition `.intentfiles` exists to end.
        Ok(raw) => match String::from_utf8(raw) {
          Ok(text) => carried.push(Attachment::new(rel.to_string_lossy(), text)),
          Err(not_text) => carried.push(Attachment::opaque(
            rel.to_string_lossy(),
            not_text.into_bytes(),
          )),
        },
        // Still a refusal, and a different kind: unreadable is not the same as
        // unrepresentable. Nothing here can carry bytes it could not obtain,
        // and carrying an empty attachment in their place would record a file
        // that never existed.
        Err(e) => refused.push((name, format!("could not be read: {e}"))),
      }
    }
    carried.sort_by(|a, b| a.path.cmp(&b.path));
    (carried, refused)
  }

  pub fn thread_files(&self, id: &str) -> Vec<PathBuf> {
    let dir = self.thread_dir(id);
    let mut out = Vec::new();
    let mut walk = ignore::WalkBuilder::new(&dir);
    walk
      .hidden(false)
      .git_ignore(true)
      .parents(true)
      .git_global(false)
      .git_exclude(false);
    for entry in walk.build().filter_map(Result::ok) {
      let path = entry.into_path();
      if path.is_file()
        && let Ok(rel) = path.strip_prefix(&dir)
      {
        out.push(rel.to_path_buf());
      }
    }
    out.sort();
    out
  }

  /// Every thread id with committed canon, sorted. Absent `st/` is an empty
  /// project, not an error -- `intent init` creates the directory lazily.
  pub fn thread_ids(&self) -> Result<Vec<String>, ProjectError> {
    let Ok(entries) = std::fs::read_dir(self.canon_st_dir()) else {
      return Ok(Vec::new());
    };
    let mut ids: Vec<String> = entries
      .filter_map(Result::ok)
      .map(|e| e.path())
      .filter(|p| p.extension().is_some_and(|x| x == "json"))
      .filter_map(|p| p.file_stem().and_then(|s| s.to_str()).map(str::to_string))
      .collect();
    ids.sort();
    Ok(ids)
  }

  /// Whether this project's canon is in a form this binary can read.
  ///
  /// TWO independent signals, and each closes a hole the other cannot see:
  ///
  /// - the **declaration** -- `intent_version` below 3 -- catches a project
  ///   with nothing in it yet, which has no evidence to find and is still not
  ///   a v3 project, because migration is what stamps the version;
  /// - the **evidence** -- a thread directory with a v2 `info.md` and no v3
  ///   `thread.json` -- catches a config claiming 3.0.0 over canon that is
  ///   not, which is a HALF-migrated estate and strictly worse than an
  ///   unmigrated one: half of it answers and half is invisible.
  ///
  /// `project_id` is deliberately not the marker, though D15's wording makes
  /// it look like the obvious candidate. It is a migration-PROVENANCE stamp,
  /// and a project created natively under v3 was never migrated at all, so
  /// gating on it would refuse every project that never needed migrating.
  ///
  /// The evidence check is a `read_dir` plus two `stat`s per entry, and it is
  /// paid on every read. Measured A/B on a 200-thread v3 project, debug build,
  /// `intent st list` x60 interleaved: **+0.61 ms** against a 13.74 ms floor.
  /// Interleaved and taking the minimum because the first attempt at this
  /// measured the unchecked build as SLOWER -- process-spawn noise is larger
  /// than the effect, so a before/after pair of runs says nothing.
  ///
  /// That is the price of never mistaking "cannot see it" for "it is not
  /// there", and it is the right side of hv's daily-driver ruling: sync is
  /// allowed to be expensive, reads are not, and half a millisecond is not
  /// where reads become expensive.
  pub fn migration(&self) -> Migration {
    let parsed = parse_version(&self.config.intent_version);
    let declared_pre_v3 = parsed.is_some_and(|(major, _, _)| major < 3);
    let legacy_threads = self.legacy_thread_ids();

    if !declared_pre_v3 && legacy_threads.is_empty() {
      return Migration::Done;
    }
    Migration::Pending(Pending {
      declared: self.config.intent_version.clone(),
      below_floor: parsed.is_some_and(|v| v < MIGRATION_FLOOR),
      legacy_threads,
    })
  }

  /// Thread directories carrying v2 canon and no v3 canon, sorted.
  ///
  /// The discriminator is `thread.json` being ABSENT, never `info.md` being
  /// present: v3 renders `info.md` as a generated view, so every healthy
  /// migrated thread has both, and a rule keyed on `info.md` alone would flag
  /// every project in existence.
  ///
  /// **Two levels, because v2 has two.** `intent st done` RELOCATES a thread
  /// to `st/<STATUS>/<ID>/`, so a one-level scan sees only the threads that
  /// are still open. Measured on this repository when this was one level: it
  /// found ST0056 and missed the 55 threads under `COMPLETED/`, `CANCELLED/`
  /// and `NOT-STARTED/`. The declaration signal happened to catch it anyway,
  /// which is exactly how a hole like this survives -- the case it fails on is
  /// a project whose live threads are migrated and whose ARCHIVE is not, and
  /// that project would have read as fully migrated.
  ///
  /// **And the question is PER THREAD, not per directory, which is the half
  /// that was missing and which made a successful migration unreadable.**
  /// `collect_legacy` can only ask whether `thread.json` sits BESIDE the
  /// `info.md` it found. After a migration it never does: canon is written to
  /// `st/<ID>/` and the v2 original stays where v2 left it, so every archived
  /// thread answers "no `thread.json` here" while its canon sits one directory
  /// up. Measured on this estate: 56 threads migrated, exit 0, 311 files
  /// written -- and then 55 ids reported unmigrated, every one of them with
  /// `st/<ID>/thread.json` present. **The detector was flagging the SOURCE of
  /// a migration that had already succeeded** (ic), and every verb but `info`
  /// refused, writes included, with a remedy that loops: a second `upgrade`
  /// exits 0, reports 311 files, and changes nothing.
  ///
  /// So an id with canon anywhere is SUPERSEDED wherever else it appears, not
  /// pending. That is the same rule `7628a02b` gave the ingest walk when canon
  /// began winning on re-read; this is the one reader that never got it.
  ///
  /// **The retain does NOT weaken the case the two-level scan exists for**, and
  /// the two are told apart by the same fact rather than by a special case: a
  /// genuinely unmigrated archive has no `st/<ID>/thread.json` to find, so it
  /// survives the filter and still convicts. Both directions are driven, in
  /// one fixture, in `unmigrated_project.rs` -- a filter tested only on the
  /// estate it is meant to clear is indistinguishable from deleting the check.
  fn legacy_thread_ids(&self) -> Vec<String> {
    let mut ids = Vec::new();
    collect_legacy(&self.st_dir(), true, &mut ids);
    ids.sort();
    ids.dedup();
    ids.retain(|id| !self.thread_json(id).is_file());
    ids
  }

  /// Every issue number with committed canon, sorted.
  pub fn issue_numbers(&self) -> Result<Vec<u32>, ProjectError> {
    let Ok(entries) = std::fs::read_dir(self.issues_dir()) else {
      return Ok(Vec::new());
    };
    let mut numbers: Vec<u32> = entries
      .filter_map(Result::ok)
      .map(|e| e.path())
      .filter(|p| p.extension().is_some_and(|x| x == "json"))
      .filter_map(|p| {
        p.file_stem()
          .and_then(|s| s.to_str())
          .and_then(|s| s.parse().ok())
      })
      .collect();
    numbers.sort_unstable();
    Ok(numbers)
  }
}

/// A path as Intent names it: relative to the project root, forward-slashed on
/// every platform.
///
/// The one home for this. Findings, the file index and the skew check all
/// report paths, and three private copies of "make it relative" is precisely
/// the shape of drift the Highlander rule exists to prevent -- the copies
/// agree until one of them handles a prefix mismatch differently, and then two
/// subsystems disagree about what to call the same file.
pub fn relative(root: &Path, path: &Path) -> String {
  path
    .strip_prefix(root)
    .unwrap_or(path)
    .components()
    .map(|c| c.as_os_str().to_string_lossy())
    .collect::<Vec<_>>()
    .join("/")
}

/// What `intent edit <path>` should do with a file (AC-05.1, hv 2026-08-19).
///
/// **A generated view is REFUSED and the refusal names where to author
/// instead.** Canon had this deferred at `surface/dispatch-table.json:834` on
/// the argument that emitting the path unchanged is defensible if the skew
/// check catches the edit. AC-03.4 is green and does catch it -- **but it
/// catches it AFTER it is written, and the user's work is already gone.
/// Detection is not prevention**, and this estate has lost work to exactly
/// this twice: a row authored into `acceptance.md` dies at the next
/// `--to-disk`, and AC-03.16 exists because a generated view was inviting
/// authoring in itself.
///
/// **And a refusal that only refuses is worse than useless here**, because the
/// user has a real edit to make and no idea where it goes. So the disposition
/// carries the destination.
///
/// Measured, so the refusal is not read as broader than it is: ST0056's thread
/// directory holds 13 files at depth 1 and **only two are generated views**.
/// The other eleven are attachments, authored on disk, and `edit` hands those
/// over without hesitation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditDisposition {
  /// The file on disk IS the authoring surface. Hand over the path.
  Open,
  /// Generated from the model. Refuse, and say what authors it.
  Refuse { author_with: &'static str },
}

impl Project {
  /// Whether `intent edit` may hand this path to an editor.
  ///
  /// **Derived from [`Project::classify`] rather than from a list of
  /// filenames**, because a second answer to "what is this file" is the
  /// two-declarations defect AC-02.5 names -- and it would drift the day
  /// somebody adds a view.
  pub fn edit_disposition(rel: &Path) -> EditDisposition {
    match Project::classify(rel) {
      ThreadFile::GeneratedView => {
        let name = rel.file_name().and_then(|n| n.to_str()).unwrap_or_default();
        EditDisposition::Refuse {
          author_with: match name {
            // Named per view rather than one generic message: an operator who
            // wanted to add a criterion and one who wanted to retitle a thread
            // need different verbs, and "this is generated" sends both to the
            // same dead end.
            "acceptance.md" => "`intent ac` for criteria and `intent at` for test rows",
            _ => "`intent st` for thread fields and `intent wp` for work packages",
          },
        }
      }
      // **EVERY VALUE OF THIS FIELD MUST COMPLETE THE SENTENCE `author it with
      // ...`, AND THIS ONE DID NOT.** Both consumers interpolate it that way --
      // `FacadeError::NotEditable`'s remedy (`facade.rs:686`) and the
      // attachment `put` arm -- so a value that is a CLAUSE rather than a
      // phrase prints "author it with canon is written by the verbs".
      //
      // **NOTHING CAUGHT IT BECAUSE EACH DOOR REACHES ONLY ONE VALUE.** `st
      // edit` appends `.md` to its argument, so it can never classify a file as
      // `Canon` and only ever prints the view arm, which composes; the
      // attachment address only ever reaches this arm, because `address::parse`
      // refuses a view name one layer lower. Two values, two consumers, and
      // each pairing exercised the half that happened to read correctly.
      ThreadFile::Canon => EditDisposition::Refuse {
        author_with: "the verbs that write canon; `intent st`, `intent wp`, `intent ac`, `intent at`",
      },
      // An attachment is AUTHORED on disk -- authority runs the other way, and
      // `--to-store` ingests what you wrote. Unattached files are not ours to
      // refuse: the estate holds files Intent does not model and never claimed
      // to, and refusing them would make `edit` narrower than the directory.
      ThreadFile::Attachment | ThreadFile::Unattached => EditDisposition::Open,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  /// hv's ruling (2026-08-19): `intent edit` refuses a generated view and
  /// names the authoring surface instead. Detection is not prevention -- the
  /// skew check catches a hand-edited view AFTER the work is gone.
  #[test]
  fn edit_refuses_generated_views_and_opens_authored_files() {
    // The two generated views at a thread's root, and a WP's cover.
    for (view, expect_verb) in [("info.md", "intent st"), ("acceptance.md", "intent ac")] {
      match Project::edit_disposition(Path::new(view)) {
        EditDisposition::Refuse { author_with } => assert!(
          author_with.contains(expect_verb),
          "{view} must name the verb that authors it, said `{author_with}`"
        ),
        EditDisposition::Open => panic!("{view} is GENERATED and must be refused"),
      }
    }
    assert!(matches!(
      Project::edit_disposition(&PathBuf::from("WP").join("01").join("info.md")),
      EditDisposition::Refuse { .. }
    ));
    assert!(matches!(
      Project::edit_disposition(Path::new("thread.json")),
      EditDisposition::Refuse { .. }
    ));

    // **The discriminating half, and it is the larger population.** ST0056
    // holds 13 files at depth 1 and only two are views; refusing the rest
    // would make `edit` narrower than the directory it serves.
    for authored in [
      "design.md",
      "data-model.md",
      "parity.md",
      "impl.md",
      "tasks.md",
      "migration.md",
      "install.md",
      "deferred.md",
      "critic-gate.md",
      "output-contracts.md",
      "realisation.md",
    ] {
      assert_eq!(
        Project::edit_disposition(Path::new(authored)),
        EditDisposition::Open,
        "{authored} is AUTHORED on disk -- the file is the authoring surface"
      );
    }

    // `info.md` deep under a parity directory is an author's file, not our
    // cover -- classify decides by shape, and edit inherits that.
    assert_eq!(
      Project::edit_disposition(&PathBuf::from("parity").join("fixtures").join("info.md")),
      EditDisposition::Open
    );
  }

  fn fixture() -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    let config = dir.path().join("intent").join(".config");
    std::fs::create_dir_all(&config).expect("mkdir");
    std::fs::write(
      config.join("config.json"),
      r#"{"intent_version":"3.0.0","project_name":"Fixture","author":"cc","intent_dir":"intent","languages":["rust"],"plugins":{"claude":{}}}"#,
    )
    .expect("write config");
    dir
  }

  #[test]
  fn discover_walks_up_to_the_root() {
    let dir = fixture();
    let deep = dir.path().join("intent").join("st").join("ST0001");
    std::fs::create_dir_all(&deep).expect("mkdir");
    let project = Project::discover(&deep).expect("discover");
    assert_eq!(project.root(), dir.path());
    assert_eq!(project.config().project_name, "Fixture");
  }

  #[test]
  fn an_unknown_config_block_is_carried_not_dropped() {
    let dir = fixture();
    let project = Project::open(dir.path()).expect("open");
    assert!(
      project.config().extra.contains_key("plugins"),
      "a block this version does not model must survive a read"
    );
  }

  #[test]
  fn paths_are_derived_from_the_configured_intent_dir() {
    let dir = fixture();
    let project = Project::open(dir.path()).expect("open");
    assert_eq!(
      project.thread_json("ST0056"),
      dir.path().join("intent/.canon/st/ST0056.json")
    );
    assert_eq!(
      project.issue_json(21),
      dir.path().join("intent/.canon/issues/0021.json")
    );
    // **The negative arm, beside the positive one (AC-01.6).** Canon moved and
    // the VIEWS did not: `thread_dir` still answers the directory a reader
    // browses, because `info.md` and `acceptance.md` hang off it. Asserting
    // only the first two would pass for a wholesale relocation that emptied
    // `intent/st/` entirely.
    assert_eq!(
      project.thread_dir("ST0056"),
      dir.path().join("intent/st/ST0056")
    );
    assert_eq!(
      project.db_path(),
      dir.path().join("intent/.cache/intent.db")
    );
  }
}
