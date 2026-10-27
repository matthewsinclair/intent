//! Slots, lanes and the controller (ST0086 WP-04 Part A).
//!
//! **ONE SESSION CONTROLS HOW MUCH RUNS AT ONCE, AND THE OTHERS ASK FOR A
//! SHARE.** The tool enforces capacity and records; which request goes first is
//! the controller's judgement, never the tool's (vc decision 88). A lane is a
//! scarce resource, and a run holds every lane it uses, so one `cargo` lane of
//! capacity one is one cargo run on the host however many lanes a run names.
//!
//! **THE FILES ARE THE PROTOCOL**, under `<state>/estate/slots/`: the
//! controller's `controller.json`, `lanes/<lane>.json` and `holds/<id>.json`,
//! and each request a directory of files that each have one writer and are
//! written once. A request's state is the fold of the files present, so no lock
//! exists.
//!
//! **NO RECORD CARRIES A CLOCK READING** (ST0056's D42): every stamp is the
//! file's own mtime, set by the atomic rename that wrote it, as the WIP
//! record's is. The reader reads each file's mtime, never its directory's.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use thiserror::Error;
use time::OffsetDateTime;

use super::{ClaudeSession, Unreadable, estate_dir, iso};
use crate::userstate::Dirs;
use crate::write_set::{WriteError, write_atomically};

/// The schema every slot record carries.
pub const SLOT_SCHEMA: &str = "intent/estate-slots@1";

/// The lane name a negligible run asks for and is granted into: no lane, so no
/// capacity.
pub const NO_LANE: &str = "none";

/// How long a finished request stays in `recent`.
pub(crate) const RECENT_SECONDS: i64 = 3600;

/// How long `slots wait` and `slots watch` pause between reads of the
/// requests (ST0086 AC-04.4).
pub const PAUSE: Duration = Duration::from_millis(500);

#[derive(Debug, Error)]
pub enum SlotError {
  #[error("no Claude Code session file names session `{session_id}`")]
  NoSession { session_id: String },
  #[error("`{verb}` is the controller's, and this session is not it ({controller})")]
  NotController { verb: String, controller: String },
  #[error("control is held by `{name}` (pid {pid}), which is still live")]
  ControllerLive { name: String, pid: u32 },
  #[error(
    "the controller was `{recorded}` and this session is `{caller}`: a take under another name needs hv's words"
  )]
  NeedsHv { recorded: String, caller: String },
  #[error("no request `{id}`")]
  NoRequest { id: String },
  #[error("request `{id}` is {state}, so it cannot be {verb}")]
  WrongState {
    id: String,
    state: String,
    verb: String,
  },
  #[error(
    "request `{id}` was asked by session `{by}`, and only that session starts or releases it"
  )]
  NotAsker { id: String, by: String },
  #[error("no lane `{lane}` is open")]
  NoLane { lane: String },
  #[error("lane `{lane}` holds {held} of its capacity {capacity}, so one more is over it")]
  OverCapacity {
    lane: String,
    held: u32,
    capacity: u32,
  },
  #[error(
    "lane `{lane}` admits a grant only while the 1-minute load is under {under}, and it is {load}"
  )]
  LoadOver { lane: String, load: f64, under: f64 },
  #[error("lane `{lane}` admits by load, and the load could not be read: {why}")]
  LoadUnknown { lane: String, why: String },
  #[error("hold `{id}` stands ({scope}): {condition}")]
  Held {
    id: String,
    scope: String,
    condition: String,
  },
  #[error("request `{id}` is gated not before `{gate}`, which has not ended")]
  GateUnmet { id: String, gate: String },
  #[error("request `{id}` was asked naming {asked}, and this names {given}")]
  NamedMismatch {
    id: String,
    asked: String,
    given: String,
  },
  #[error("lane `{lane}` has {count} request(s) granted or running in it")]
  LaneBusy { lane: String, count: usize },
  #[error("`{name}` is not a {what} name")]
  BadName { what: String, name: String },
  #[error("--place-of `{id}` {why}")]
  BadPlaceOf { id: String, why: String },
  #[error("{why}")]
  BadArg { why: String },
  #[error("no standing hold `{id}`{scope}")]
  NoHold { id: String, scope: String },
  #[error("`{}` already exists, and a slot record is written once", path.display())]
  Exists { path: PathBuf },
  #[error("`{}` could not be read: {why}", path.display())]
  Unreadable { path: PathBuf, why: String },
  #[error("`{}` could not be written", path.display())]
  Unwritable {
    path: PathBuf,
    #[source]
    cause: WriteError,
  },
  #[error("`{}` could not be removed", path.display())]
  Unremovable {
    path: PathBuf,
    #[source]
    cause: std::io::Error,
  },
  #[error("which sessions are alive could not be read: {why}")]
  NoLiveness { why: String },
}

impl crate::remedy::Remedy for SlotError {
  fn remedy(&self) -> String {
    match self {
      Self::NoSession { .. } => "run it from inside a Claude Code session: the caller is the session `CLAUDE_CODE_SESSION_ID` names, read from its own session file. Nothing was written.".to_string(),
      Self::NotController { .. } => "ask the controller to do it, or take control with `intent estate control take` when the controller is gone. Nothing was written.".to_string(),
      Self::ControllerLive { .. } => "ask the controller to `intent estate control release`; a take succeeds over a gone controller only. Nothing was written.".to_string(),
      Self::NeedsHv { .. } => "take it under the recorded name, or pass `--hv \"<hv's words>\"` quoting hv's yes, which is recorded on the take. Nothing was written.".to_string(),
      Self::NoRequest { .. } => "check the id with `intent estate status`. Nothing was written.".to_string(),
      Self::WrongState { .. } => "check the request's state with `intent estate status`; a finished request takes no further step, and a new booking is a new `intent estate slots ask`. Nothing was written.".to_string(),
      Self::NotAsker { .. } => "run it from the session that asked; the controller ends someone else's request with `revoke`. Nothing was written.".to_string(),
      Self::NoLane { .. } => "open it with `intent estate lane open <lane> --type <type> --capacity <n>`, or name an open lane. Nothing was written.".to_string(),
      Self::OverCapacity { .. } => "wait for a holder to release, or raise the capacity with `intent estate lane open` if that is meant. Nothing was written.".to_string(),
      Self::LoadOver { .. } => "wait for the load to fall, or change the lane's admission with `intent estate lane open --load-under`. Nothing was written.".to_string(),
      Self::LoadUnknown { .. } => "the load comes from `sysctl` on macOS and `/proc/loadavg` on Linux; check it can be read. Nothing was written.".to_string(),
      Self::Held { .. } => "release the hold with `intent estate control unhold <id>` or `intent estate lane unhold <lane> <id>` once its condition is met. Nothing was written.".to_string(),
      Self::GateUnmet { .. } => "grant it after the gating request has ended, or revoke the gated one if the gate no longer applies. Nothing was written.".to_string(),
      Self::NamedMismatch { .. } => "start it naming the same set the ask named; different bytes are a new booking, `intent estate slots ask`. Nothing was written.".to_string(),
      Self::LaneBusy { .. } => "wait for those requests to end, or revoke them, before closing the lane. Nothing was written.".to_string(),
      Self::BadName { .. } => "use letters, digits, `-` and `_`: the name is a file name. Nothing was written.".to_string(),
      Self::BadPlaceOf { .. } => "a hand-back keeps its place only from a request that was granted and released before it started, naming the same set; otherwise ask afresh. Nothing was written.".to_string(),
      Self::BadArg { .. } => "check the verb's arguments with `--help`. Nothing was written.".to_string(),
      Self::NoHold { .. } => "list the standing holds with `intent estate status`. Nothing was written.".to_string(),
      Self::Exists { .. } => "the step this writes has already been taken; check the request's state with `intent estate status`. Nothing was written.".to_string(),
      Self::Unreadable { .. } => "check the file; a record that will not parse is listed by `intent estate status` too. Nothing was written.".to_string(),
      Self::Unwritable { .. } => "check that the estate's state directory is writable. The record was not changed.".to_string(),
      Self::Unremovable { .. } => "check the file's permissions. Nothing else was changed.".to_string(),
      Self::NoLiveness { .. } => "a take must read which processes are running (`/bin/ps` on macOS, `/proc` on Linux) to tell a gone controller from a live one. Nothing was written.".to_string(),
    }
  }
}

/// `<state>/estate/slots`.
pub fn slots_dir(dirs: &Dirs) -> PathBuf {
  estate_dir(dirs).join("slots")
}

fn controller_path(dirs: &Dirs) -> PathBuf {
  slots_dir(dirs).join("controller.json")
}

fn lanes_dir(dirs: &Dirs) -> PathBuf {
  slots_dir(dirs).join("lanes")
}

fn holds_dir(dirs: &Dirs) -> PathBuf {
  slots_dir(dirs).join("holds")
}

fn requests_dir(dirs: &Dirs) -> PathBuf {
  slots_dir(dirs).join("requests")
}

/// Who runs a verb: the Claude Code session `CLAUDE_CODE_SESSION_ID` names,
/// read from its own session file, and the node it says it is.
///
/// **THE NAME IS THE SESSION FILE'S, NEVER A FLAG** (vc decision 88): `--node`
/// is self-declared, so the handover's identity check reads the name Claude
/// Code itself recorded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Caller {
  pub session_id: String,
  pub name: String,
  pub pid: u32,
  pub node: Option<String>,
}

/// The caller, from the session files under `claude_dir`.
pub fn caller(
  claude_dir: &Path,
  session_id: &str,
  node: Option<&str>,
) -> Result<Caller, SlotError> {
  let mut ignored = Vec::new();
  let sessions =
    super::read_sessions(claude_dir, &mut ignored).map_err(|_| SlotError::NoSession {
      session_id: session_id.to_string(),
    })?;
  let found = sessions
    .into_iter()
    .find(|s| s.session_id == session_id)
    .ok_or_else(|| SlotError::NoSession {
      session_id: session_id.to_string(),
    })?;
  Ok(Caller {
    session_id: found.session_id,
    name: found.name,
    pid: found.pid,
    node: node.map(str::to_string),
  })
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Predecessor {
  pub session_id: String,
  pub name: String,
  pub node: Option<String>,
}

/// `controller.json`: replaced whole by each take and by a release.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControllerRecord {
  pub schema: String,
  pub session_id: String,
  pub name: String,
  pub pid: u32,
  pub node: Option<String>,
  pub predecessor: Option<Predecessor>,
  pub hv: Option<String>,
  pub released: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LaneRecord {
  pub schema: String,
  pub name: String,
  #[serde(rename = "type")]
  pub kind: String,
  pub capacity: u32,
  pub load_under: Option<f64>,
  pub node: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Gate {
  /// `not-before`, which the tool enforces, or `hold`, which the controller
  /// judges.
  pub kind: String,
  pub text: String,
  #[serde(rename = "ref")]
  pub reference: Option<String>,
}

pub const GATE_NOT_BEFORE: &str = "not-before";
pub const GATE_HOLD: &str = "hold";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AskRecord {
  pub schema: String,
  pub id: String,
  #[serde(rename = "type")]
  pub kind: String,
  pub what: String,
  /// The lane the asker prefers, `none` for a negligible run, or null.
  pub lane: Option<String>,
  pub named: BTreeMap<String, String>,
  pub tags: Vec<String>,
  pub gates: Vec<Gate>,
  pub place_of: Option<String>,
  pub by: String,
  pub node: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GrantRecord {
  pub schema: String,
  /// Every lane the grant names; empty for a grant into no lane.
  pub lanes: Vec<String>,
  pub node: Option<String>,
}

/// `deny.json` and `revoke.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WhyRecord {
  pub schema: String,
  pub why: String,
  pub node: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartRecord {
  pub schema: String,
  pub named: BTreeMap<String, String>,
  pub relayed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndRecord {
  pub schema: String,
  pub result: Option<String>,
  pub relayed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HoldRecord {
  pub schema: String,
  pub id: String,
  /// `desk` or `lane`.
  pub scope: String,
  pub lane: Option<String>,
  pub condition: String,
  pub node: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnheldRecord {
  pub schema: String,
  pub node: Option<String>,
}

/// A record and the mtime of the file that holds it.
#[derive(Debug, Clone, PartialEq)]
pub struct Stamped<T> {
  pub record: T,
  pub at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ReqState {
  Asked,
  Granted,
  Running,
  Ended,
  Denied,
  Revoked,
}

impl ReqState {
  pub fn as_str(self) -> &'static str {
    match self {
      Self::Asked => "asked",
      Self::Granted => "granted",
      Self::Running => "running",
      Self::Ended => "ended",
      Self::Denied => "denied",
      Self::Revoked => "revoked",
    }
  }
}

/// One request: its directory's files, folded.
#[derive(Debug, Clone, PartialEq)]
pub struct Request {
  pub ask: Stamped<AskRecord>,
  pub grant: Option<Stamped<GrantRecord>>,
  pub deny: Option<Stamped<WhyRecord>>,
  pub start: Option<Stamped<StartRecord>>,
  pub end: Option<Stamped<EndRecord>>,
  pub revoke: Option<Stamped<WhyRecord>>,
}

impl Request {
  /// The fold: revoked, else denied, else ended, else running, else granted,
  /// else asked.
  pub fn state(&self) -> ReqState {
    if self.revoke.is_some() {
      ReqState::Revoked
    } else if self.deny.is_some() {
      ReqState::Denied
    } else if self.end.is_some() {
      ReqState::Ended
    } else if self.start.is_some() {
      ReqState::Running
    } else if self.grant.is_some() {
      ReqState::Granted
    } else {
      ReqState::Asked
    }
  }

  /// Granted or running: counted against every lane its grant names.
  pub fn holding(&self) -> bool {
    matches!(self.state(), ReqState::Granted | ReqState::Running)
  }

  pub fn finished(&self) -> bool {
    matches!(
      self.state(),
      ReqState::Ended | ReqState::Denied | ReqState::Revoked
    )
  }

  /// The lanes its grant names.
  pub fn lanes(&self) -> &[String] {
    self
      .grant
      .as_ref()
      .map_or(&[], |g| g.record.lanes.as_slice())
  }

  /// When it finished: the mtime of its end, deny or revoke file.
  pub fn finished_at(&self) -> Option<OffsetDateTime> {
    match self.state() {
      ReqState::Revoked => self.revoke.as_ref().and_then(|r| r.at),
      ReqState::Denied => self.deny.as_ref().and_then(|r| r.at),
      ReqState::Ended => self.end.as_ref().and_then(|r| r.at),
      _ => None,
    }
  }

  pub fn id(&self) -> &str {
    &self.ask.record.id
  }
}

/// Everything under `slots/`, read.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Slots {
  pub controller: Option<Stamped<ControllerRecord>>,
  /// Why `controller.json` would not parse, when it is there and will not:
  /// then `controller` is `None` and nobody can tell who holds control, which
  /// every face says on its control line rather than NO CONTROLLER (vc's
  /// ruling, 2026-10-26).
  pub controller_unreadable: Option<Unreadable>,
  pub lanes: Vec<LaneRecord>,
  /// Standing holds only, oldest first.
  pub holds: Vec<Stamped<HoldRecord>>,
  /// Every request, in id order.
  pub requests: Vec<Request>,
}

impl Slots {
  /// The controller, when one holds control.
  pub fn live_controller(&self) -> Option<&Stamped<ControllerRecord>> {
    self.controller.as_ref().filter(|c| !c.record.released)
  }

  pub fn request(&self, id: &str) -> Option<&Request> {
    self.requests.iter().find(|r| r.id() == id)
  }

  pub fn lane(&self, name: &str) -> Option<&LaneRecord> {
    self.lanes.iter().find(|l| l.name == name)
  }

  /// The requests granted or running in a lane.
  pub fn held_in(&self, lane: &str) -> usize {
    self
      .requests
      .iter()
      .filter(|r| r.holding() && r.lanes().iter().any(|l| l == lane))
      .count()
  }

  /// The request ids each session holds, granted or running.
  pub fn held_by(&self) -> BTreeMap<String, Vec<String>> {
    let mut out: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for r in self.requests.iter().filter(|r| r.holding()) {
      out
        .entry(r.ask.record.by.clone())
        .or_default()
        .push(r.id().to_string());
    }
    out
  }

  /// The sessions holding a running request.
  pub fn running_by(&self) -> BTreeSet<String> {
    self
      .requests
      .iter()
      .filter(|r| r.state() == ReqState::Running)
      .map(|r| r.ask.record.by.clone())
      .collect()
  }

  pub fn running(&self) -> usize {
    self
      .requests
      .iter()
      .filter(|r| r.state() == ReqState::Running)
      .count()
  }
}

fn mtime(path: &Path) -> Option<OffsetDateTime> {
  std::fs::metadata(path)
    .and_then(|m| m.modified())
    .ok()
    .map(OffsetDateTime::from)
}

fn unread(path: &Path, why: String) -> Unreadable {
  Unreadable {
    path: path.display().to_string(),
    why,
    pid: None,
  }
}

/// One record, or why not. An absent file is `Ok(None)`.
fn load<T: serde::de::DeserializeOwned>(path: &Path) -> Result<Option<Stamped<T>>, Unreadable> {
  let text = match std::fs::read_to_string(path) {
    Ok(text) => text,
    Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
    Err(e) => return Err(unread(path, e.to_string())),
  };
  let doc: Value =
    serde_json::from_str(&text).map_err(|e| unread(path, format!("not JSON: {e}")))?;
  let schema = doc
    .get("schema")
    .and_then(Value::as_str)
    .unwrap_or("(none)");
  if schema != SLOT_SCHEMA {
    return Err(unread(
      path,
      format!("schema `{schema}` is not `{SLOT_SCHEMA}`"),
    ));
  }
  let record =
    serde_json::from_value(doc).map_err(|e| unread(path, format!("not a record: {e}")))?;
  Ok(Some(Stamped {
    record,
    at: mtime(path),
  }))
}

/// The `.json` files directly in `dir`, sorted, or none when it is absent.
fn json_files(dir: &Path, unreadable: &mut Vec<Unreadable>) -> Vec<PathBuf> {
  let entries = match std::fs::read_dir(dir) {
    Ok(entries) => entries,
    Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Vec::new(),
    Err(e) => {
      unreadable.push(unread(dir, format!("could not be listed: {e}")));
      return Vec::new();
    }
  };
  let mut paths: Vec<PathBuf> = entries
    .filter_map(Result::ok)
    .map(|e| e.path())
    .filter(|p| p.extension().is_some_and(|x| x == "json"))
    .collect();
  paths.sort();
  paths
}

/// Reads everything under `slots/`; a file that will not parse is listed and
/// left out, never dropped silently.
pub fn read(dirs: &Dirs, unreadable: &mut Vec<Unreadable>) -> Slots {
  let mut keep = |r: Result<Option<_>, Unreadable>| match r {
    Ok(v) => v,
    Err(u) => {
      unreadable.push(u);
      None
    }
  };
  let mut controller_unreadable = None;
  let controller = keep(
    load::<ControllerRecord>(&controller_path(dirs)).inspect_err(|u| {
      controller_unreadable = Some(u.clone());
    }),
  );
  let mut lane_errors = Vec::new();
  let lanes: Vec<LaneRecord> = json_files(&lanes_dir(dirs), &mut lane_errors)
    .iter()
    .filter_map(|p| match load::<LaneRecord>(p) {
      Ok(v) => v.map(|s| s.record),
      Err(u) => {
        lane_errors.push(u);
        None
      }
    })
    .collect();
  let mut hold_errors = Vec::new();
  let mut holds: Vec<Stamped<HoldRecord>> = json_files(&holds_dir(dirs), &mut hold_errors)
    .iter()
    .filter(|p| !p.to_string_lossy().ends_with(".unheld.json"))
    .filter_map(|p| match load::<HoldRecord>(p) {
      Ok(v) => v,
      Err(u) => {
        hold_errors.push(u);
        None
      }
    })
    .filter(|h| {
      !holds_dir(dirs)
        .join(format!("{}.unheld.json", h.record.id))
        .exists()
    })
    .collect();
  holds.sort_by(|a, b| a.at.cmp(&b.at).then_with(|| a.record.id.cmp(&b.record.id)));
  let mut request_errors = Vec::new();
  let requests = read_requests(&requests_dir(dirs), &mut request_errors);
  unreadable.extend(lane_errors);
  unreadable.extend(hold_errors);
  unreadable.extend(request_errors);
  Slots {
    controller,
    controller_unreadable,
    lanes,
    holds,
    requests,
  }
}

fn read_requests(dir: &Path, unreadable: &mut Vec<Unreadable>) -> Vec<Request> {
  let entries = match std::fs::read_dir(dir) {
    Ok(entries) => entries,
    Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Vec::new(),
    Err(e) => {
      unreadable.push(unread(dir, format!("could not be listed: {e}")));
      return Vec::new();
    }
  };
  let mut dirs: Vec<PathBuf> = entries
    .filter_map(Result::ok)
    .map(|e| e.path())
    .filter(|p| p.is_dir())
    .collect();
  dirs.sort();
  dirs
    .into_iter()
    .filter_map(|d| match read_request(&d) {
      Ok(r) => r,
      Err(u) => {
        unreadable.push(u);
        None
      }
    })
    .collect()
}

fn read_request(dir: &Path) -> Result<Option<Request>, Unreadable> {
  let Some(ask) = load::<AskRecord>(&dir.join("ask.json"))? else {
    return Err(unread(
      dir,
      "a request directory with no `ask.json`".to_string(),
    ));
  };
  Ok(Some(Request {
    ask,
    grant: load(&dir.join("grant.json"))?,
    deny: load(&dir.join("deny.json"))?,
    start: load(&dir.join("start.json"))?,
    end: load(&dir.join("end.json"))?,
    revoke: load(&dir.join("revoke.json"))?,
  }))
}

fn valid_name(name: &str) -> bool {
  !name.is_empty()
    && name
      .chars()
      .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

fn name_ok(what: &str, name: &str) -> Result<(), SlotError> {
  if valid_name(name) {
    Ok(())
  } else {
    Err(SlotError::BadName {
      what: what.to_string(),
      name: name.to_string(),
    })
  }
}

/// A one-line, non-empty text argument.
fn one_line(what: &str, text: &str) -> Result<(), SlotError> {
  if text.trim().is_empty() {
    return Err(SlotError::BadArg {
      why: format!("the {what} is empty"),
    });
  }
  if text.contains(['\n', '\r']) {
    return Err(SlotError::BadArg {
      why: format!("the {what} spans more than one line"),
    });
  }
  Ok(())
}

fn write<T: Serialize>(path: &Path, record: &T) -> Result<(), SlotError> {
  let unwritable = |cause: WriteError| SlotError::Unwritable {
    path: path.to_path_buf(),
    cause,
  };
  let mut text = serde_json::to_string_pretty(record).map_err(|e| {
    unwritable(WriteError::Io {
      path: path.display().to_string(),
      source: std::io::Error::other(e),
    })
  })?;
  text.push('\n');
  if let Some(parent) = path.parent() {
    std::fs::create_dir_all(parent).map_err(|e| {
      unwritable(WriteError::Io {
        path: parent.display().to_string(),
        source: e,
      })
    })?;
  }
  write_atomically(path, text.as_bytes()).map_err(unwritable)
}

/// A record written once: an existing file is refused.
fn write_once<T: Serialize>(path: &Path, record: &T) -> Result<(), SlotError> {
  if path.exists() {
    return Err(SlotError::Exists {
      path: path.to_path_buf(),
    });
  }
  write(path, record)
}

/// Refuses unless the caller is the live controller.
fn ensure_controller(slots: &Slots, caller: &Caller, verb: &str) -> Result<(), SlotError> {
  match slots.live_controller() {
    Some(c) if c.record.session_id == caller.session_id => Ok(()),
    Some(c) => Err(SlotError::NotController {
      verb: verb.to_string(),
      controller: format!("the controller is `{}`", c.record.name),
    }),
    None => Err(SlotError::NotController {
      verb: verb.to_string(),
      controller: "no session holds control".to_string(),
    }),
  }
}

fn read_strict(dirs: &Dirs) -> Slots {
  let mut ignored = Vec::new();
  read(dirs, &mut ignored)
}

fn request_of<'a>(slots: &'a Slots, id: &str) -> Result<&'a Request, SlotError> {
  slots
    .request(id)
    .ok_or_else(|| SlotError::NoRequest { id: id.to_string() })
}

fn wrong_state(r: &Request, verb: &str) -> SlotError {
  SlotError::WrongState {
    id: r.id().to_string(),
    state: r.state().as_str().to_string(),
    verb: verb.to_string(),
  }
}

/// Whether a session is alive by the page's ORPHANED test: its session file is
/// present and its pid is still the process Claude Code recorded.
fn session_live(claude_dir: &Path, session_id: &str) -> Result<bool, SlotError> {
  let mut ignored = Vec::new();
  let sessions: Vec<ClaudeSession> = super::read_sessions(claude_dir, &mut ignored)
    .map_err(|e| SlotError::NoLiveness { why: e.to_string() })?;
  let Some(found) = sessions.iter().find(|s| s.session_id == session_id) else {
    return Ok(false);
  };
  let starts =
    super::read_liveness(&[found.pid]).map_err(|u| SlotError::NoLiveness { why: u.why })?;
  Ok(super::alive(found, &starts))
}

/// What a take wrote.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Taken {
  pub path: PathBuf,
  pub predecessor: Option<Predecessor>,
  /// The caller already held control, and nothing was written.
  pub unchanged: bool,
}

/// `control take`: a handover (vc decision 88, amended on molt-vc's read).
///
/// It succeeds when no controller was ever recorded, when the recorded one
/// released, or when its session is gone. A take under a name other than the
/// recorded controller's needs hv's words, recorded verbatim; a first take
/// needs none. While the recorded controller is live it is refused, naming it.
pub fn control_take(
  dirs: &Dirs,
  claude_dir: &Path,
  caller: &Caller,
  hv: Option<&str>,
) -> Result<Taken, SlotError> {
  if let Some(words) = hv {
    one_line("hv's words", words)?;
  }
  let path = controller_path(dirs);
  let slots = read_strict(dirs);
  let predecessor = match &slots.controller {
    None => None,
    Some(current) => {
      let rec = &current.record;
      if !rec.released {
        if rec.session_id == caller.session_id {
          return Ok(Taken {
            path,
            predecessor: rec.predecessor.clone(),
            unchanged: true,
          });
        }
        if session_live(claude_dir, &rec.session_id)? {
          return Err(SlotError::ControllerLive {
            name: rec.name.clone(),
            pid: rec.pid,
          });
        }
      }
      if rec.name != caller.name && hv.is_none() {
        return Err(SlotError::NeedsHv {
          recorded: rec.name.clone(),
          caller: caller.name.clone(),
        });
      }
      Some(Predecessor {
        session_id: rec.session_id.clone(),
        name: rec.name.clone(),
        node: rec.node.clone(),
      })
    }
  };
  write(
    &path,
    &ControllerRecord {
      schema: SLOT_SCHEMA.to_string(),
      session_id: caller.session_id.clone(),
      name: caller.name.clone(),
      pid: caller.pid,
      node: caller.node.clone(),
      predecessor: predecessor.clone(),
      hv: hv.map(str::to_string),
      released: false,
    },
  )?;
  Ok(Taken {
    path,
    predecessor,
    unchanged: false,
  })
}

/// `control release`: hands over cleanly. The record stays, marked released,
/// so the reader shows NO CONTROLLER from a record and not from an absence.
pub fn control_release(dirs: &Dirs, caller: &Caller) -> Result<PathBuf, SlotError> {
  let slots = read_strict(dirs);
  ensure_controller(&slots, caller, "control release")?;
  let path = controller_path(dirs);
  let Some(current) = slots.controller else {
    return Err(SlotError::NotController {
      verb: "control release".to_string(),
      controller: "no session holds control".to_string(),
    });
  };
  write(
    &path,
    &ControllerRecord {
      released: true,
      node: caller.node.clone().or(current.record.node),
      ..current.record
    },
  )?;
  Ok(path)
}

/// `control hold` (no lane) or `lane hold` (a lane): returns the hold's id
/// and the file written.
pub fn hold(
  dirs: &Dirs,
  caller: &Caller,
  lane: Option<&str>,
  condition: &str,
) -> Result<(String, PathBuf), SlotError> {
  let verb = if lane.is_some() {
    "lane hold"
  } else {
    "control hold"
  };
  let slots = read_strict(dirs);
  ensure_controller(&slots, caller, verb)?;
  one_line("condition", condition)?;
  if let Some(l) = lane
    && slots.lane(l).is_none()
  {
    return Err(SlotError::NoLane {
      lane: l.to_string(),
    });
  }
  let id = crate::event::next_id();
  let path = holds_dir(dirs).join(format!("{id}.json"));
  write_once(
    &path,
    &HoldRecord {
      schema: SLOT_SCHEMA.to_string(),
      id: id.clone(),
      scope: if lane.is_some() { "lane" } else { "desk" }.to_string(),
      lane: lane.map(str::to_string),
      condition: condition.to_string(),
      node: caller.node.clone(),
    },
  )?;
  Ok((id, path))
}

/// `control unhold <id>` or `lane unhold <lane> <id>`.
pub fn unhold(
  dirs: &Dirs,
  caller: &Caller,
  lane: Option<&str>,
  id: &str,
) -> Result<PathBuf, SlotError> {
  let verb = if lane.is_some() {
    "lane unhold"
  } else {
    "control unhold"
  };
  let slots = read_strict(dirs);
  ensure_controller(&slots, caller, verb)?;
  let standing = slots
    .holds
    .iter()
    .find(|h| h.record.id == id && h.record.lane.as_deref() == lane);
  if standing.is_none() {
    return Err(SlotError::NoHold {
      id: id.to_string(),
      scope: lane.map_or(" on the desk".to_string(), |l| format!(" on lane `{l}`")),
    });
  }
  let path = holds_dir(dirs).join(format!("{id}.unheld.json"));
  write_once(
    &path,
    &UnheldRecord {
      schema: SLOT_SCHEMA.to_string(),
      node: caller.node.clone(),
    },
  )?;
  Ok(path)
}

/// `lane open`: opens a lane, or changes its type, capacity or admission.
pub fn lane_open(
  dirs: &Dirs,
  caller: &Caller,
  name: &str,
  kind: &str,
  capacity: u32,
  load_under: Option<f64>,
) -> Result<PathBuf, SlotError> {
  let slots = read_strict(dirs);
  ensure_controller(&slots, caller, "lane open")?;
  name_ok("lane", name)?;
  name_ok("lane type", kind)?;
  if name == NO_LANE {
    return Err(SlotError::BadArg {
      why: format!("`{NO_LANE}` is the name of no lane, for a negligible run"),
    });
  }
  if capacity == 0 {
    return Err(SlotError::BadArg {
      why: "a lane's capacity is at least one; close it to end it".to_string(),
    });
  }
  if let Some(under) = load_under
    && !(under.is_finite() && under > 0.0)
  {
    return Err(SlotError::BadArg {
      why: format!("--load-under {under} is not a positive load"),
    });
  }
  let path = lanes_dir(dirs).join(format!("{name}.json"));
  write(
    &path,
    &LaneRecord {
      schema: SLOT_SCHEMA.to_string(),
      name: name.to_string(),
      kind: kind.to_string(),
      capacity,
      load_under,
      node: caller.node.clone(),
    },
  )?;
  Ok(path)
}

/// `lane close`: refused while anything is granted or running in it.
pub fn lane_close(dirs: &Dirs, caller: &Caller, name: &str) -> Result<PathBuf, SlotError> {
  let slots = read_strict(dirs);
  ensure_controller(&slots, caller, "lane close")?;
  if slots.lane(name).is_none() {
    return Err(SlotError::NoLane {
      lane: name.to_string(),
    });
  }
  let count = slots.held_in(name);
  if count > 0 {
    return Err(SlotError::LaneBusy {
      lane: name.to_string(),
      count,
    });
  }
  let path = lanes_dir(dirs).join(format!("{name}.json"));
  std::fs::remove_file(&path).map_err(|cause| SlotError::Unremovable {
    path: path.clone(),
    cause,
  })?;
  Ok(path)
}

/// What an ask carries beyond its type and what.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AskArgs {
  pub lane: Option<String>,
  pub named: BTreeMap<String, String>,
  pub tags: Vec<String>,
  pub not_before: Vec<String>,
  pub holds: Vec<String>,
  pub place_of: Option<String>,
}

/// `slots ask`: queues a request and returns its id and the file written.
pub fn ask(
  dirs: &Dirs,
  caller: &Caller,
  kind: &str,
  what: &str,
  args: AskArgs,
) -> Result<(String, PathBuf), SlotError> {
  name_ok("request type", kind)?;
  one_line("what", what)?;
  if let Some(l) = &args.lane
    && l != NO_LANE
  {
    name_ok("lane", l)?;
  }
  for tag in &args.tags {
    name_ok("tag", tag)?;
  }
  for (k, v) in &args.named {
    name_ok("--named key", k)?;
    one_line("--named value", v)?;
  }
  for condition in &args.holds {
    one_line("hold condition", condition)?;
  }
  let slots = read_strict(dirs);
  for gate in &args.not_before {
    request_of(&slots, gate)?;
  }
  if let Some(prior) = &args.place_of {
    let r = request_of(&slots, prior)?;
    let why = if r.grant.is_none() || r.end.is_none() || r.start.is_some() || r.revoke.is_some() {
      Some("was not granted and released before it started")
    } else if r.ask.record.named != args.named {
      Some("named a different set")
    } else {
      None
    };
    if let Some(why) = why {
      return Err(SlotError::BadPlaceOf {
        id: prior.clone(),
        why: why.to_string(),
      });
    }
  }
  let gates = args
    .not_before
    .iter()
    .map(|id| Gate {
      kind: GATE_NOT_BEFORE.to_string(),
      text: format!("not before {id}"),
      reference: Some(id.clone()),
    })
    .chain(args.holds.iter().map(|c| Gate {
      kind: GATE_HOLD.to_string(),
      text: c.clone(),
      reference: None,
    }))
    .collect();
  let id = crate::event::next_id();
  let path = requests_dir(dirs).join(&id).join("ask.json");
  write_once(
    &path,
    &AskRecord {
      schema: SLOT_SCHEMA.to_string(),
      id: id.clone(),
      kind: kind.to_string(),
      what: what.to_string(),
      lane: args.lane,
      named: args.named,
      tags: args.tags,
      gates,
      place_of: args.place_of,
      by: caller.session_id.clone(),
      node: caller.node.clone(),
    },
  )?;
  Ok((id, path))
}

/// `slots grant`: into every lane named, or into no lane with `none`.
///
/// Refused while a desk hold stands, a hold on a named lane stands, a
/// `not-before` gate is unmet, a named lane's load admission is not met, or
/// any named lane would go over its capacity. `load1` is asked only when a
/// named lane admits by load.
pub fn grant(
  dirs: &Dirs,
  caller: &Caller,
  id: &str,
  lanes: &[String],
  load1: &dyn Fn() -> Result<f64, Unreadable>,
) -> Result<PathBuf, SlotError> {
  let slots = read_strict(dirs);
  ensure_controller(&slots, caller, "slots grant")?;
  let r = request_of(&slots, id)?;
  if r.state() != ReqState::Asked {
    return Err(wrong_state(r, "granted"));
  }
  if lanes.is_empty() {
    return Err(SlotError::BadArg {
      why: format!("a grant names at least one lane, or `--lane {NO_LANE}`"),
    });
  }
  let none = lanes.iter().any(|l| l == NO_LANE);
  if none && lanes.len() > 1 {
    return Err(SlotError::BadArg {
      why: format!("`--lane {NO_LANE}` grants into no lane, so it names no other"),
    });
  }
  let named: Vec<String> = if none { Vec::new() } else { lanes.to_vec() };
  let mut seen = BTreeSet::new();
  for l in &named {
    if !seen.insert(l.as_str()) {
      return Err(SlotError::BadArg {
        why: format!("lane `{l}` is named twice"),
      });
    }
  }
  if let Some(h) = slots.holds.iter().find(|h| h.record.lane.is_none()) {
    return Err(held(&h.record));
  }
  let mut open = Vec::new();
  for l in &named {
    let lane = slots
      .lane(l)
      .ok_or_else(|| SlotError::NoLane { lane: l.clone() })?;
    if let Some(h) = slots
      .holds
      .iter()
      .find(|h| h.record.lane.as_deref() == Some(l.as_str()))
    {
      return Err(held(&h.record));
    }
    open.push(lane);
  }
  for gate in &r.ask.record.gates {
    if gate.kind == GATE_NOT_BEFORE
      && let Some(gating) = &gate.reference
    {
      let ended = slots.request(gating).is_some_and(|g| g.end.is_some());
      if !ended {
        return Err(SlotError::GateUnmet {
          id: id.to_string(),
          gate: gating.clone(),
        });
      }
    }
  }
  let mut load = None;
  for lane in &open {
    if let Some(under) = lane.load_under {
      let reading = match &load {
        Some(v) => *v,
        None => {
          let v = load1().map_err(|u| SlotError::LoadUnknown {
            lane: lane.name.clone(),
            why: u.why,
          })?;
          load = Some(v);
          v
        }
      };
      if reading >= under {
        return Err(SlotError::LoadOver {
          lane: lane.name.clone(),
          load: reading,
          under,
        });
      }
    }
  }
  for lane in &open {
    let held = slots.held_in(&lane.name);
    if held + 1 > lane.capacity as usize {
      return Err(SlotError::OverCapacity {
        lane: lane.name.clone(),
        held: u32::try_from(held).unwrap_or(u32::MAX),
        capacity: lane.capacity,
      });
    }
  }
  let path = requests_dir(dirs).join(id).join("grant.json");
  write_once(
    &path,
    &GrantRecord {
      schema: SLOT_SCHEMA.to_string(),
      lanes: named,
      node: caller.node.clone(),
    },
  )?;
  Ok(path)
}

fn held(h: &HoldRecord) -> SlotError {
  SlotError::Held {
    id: h.id.clone(),
    scope: h
      .lane
      .as_ref()
      .map_or("the desk".to_string(), |l| format!("lane `{l}`")),
    condition: h.condition.clone(),
  }
}

/// `slots deny`: refuses a request still asked.
pub fn deny(dirs: &Dirs, caller: &Caller, id: &str, why: &str) -> Result<PathBuf, SlotError> {
  let slots = read_strict(dirs);
  ensure_controller(&slots, caller, "slots deny")?;
  one_line("reason", why)?;
  let r = request_of(&slots, id)?;
  if r.state() != ReqState::Asked {
    return Err(wrong_state(r, "denied"));
  }
  let path = requests_dir(dirs).join(id).join("deny.json");
  write_once(
    &path,
    &WhyRecord {
      schema: SLOT_SCHEMA.to_string(),
      why: why.to_string(),
      node: caller.node.clone(),
    },
  )?;
  Ok(path)
}

/// `slots revoke`: ends an asked, granted or running request, freeing what it
/// held. The tool kills no process.
pub fn revoke(dirs: &Dirs, caller: &Caller, id: &str, why: &str) -> Result<PathBuf, SlotError> {
  let slots = read_strict(dirs);
  ensure_controller(&slots, caller, "slots revoke")?;
  one_line("reason", why)?;
  let r = request_of(&slots, id)?;
  if r.finished() {
    return Err(wrong_state(r, "revoked"));
  }
  let path = requests_dir(dirs).join(id).join("revoke.json");
  write_once(
    &path,
    &WhyRecord {
      schema: SLOT_SCHEMA.to_string(),
      why: why.to_string(),
      node: caller.node.clone(),
    },
  )?;
  Ok(path)
}

fn ensure_asker(r: &Request, caller: &Caller) -> Result<(), SlotError> {
  if r.ask.record.by == caller.session_id {
    Ok(())
  } else {
    Err(SlotError::NotAsker {
      id: r.id().to_string(),
      by: r.ask.record.by.clone(),
    })
  }
}

fn show_named(named: &BTreeMap<String, String>) -> String {
  if named.is_empty() {
    "nothing".to_string()
  } else {
    named
      .iter()
      .map(|(k, v)| format!("{k}={v}"))
      .collect::<Vec<_>>()
      .join(" ")
  }
}

/// `slots start`: the holder starts a granted request, naming the set the ask
/// named.
pub fn start(
  dirs: &Dirs,
  caller: &Caller,
  id: &str,
  named: BTreeMap<String, String>,
  relayed: bool,
) -> Result<PathBuf, SlotError> {
  let slots = read_strict(dirs);
  let r = request_of(&slots, id)?;
  ensure_asker(r, caller)?;
  if r.state() != ReqState::Granted {
    return Err(wrong_state(r, "started"));
  }
  if r.ask.record.named != named {
    return Err(SlotError::NamedMismatch {
      id: id.to_string(),
      asked: show_named(&r.ask.record.named),
      given: show_named(&named),
    });
  }
  let path = requests_dir(dirs).join(id).join("start.json");
  write_once(
    &path,
    &StartRecord {
      schema: SLOT_SCHEMA.to_string(),
      named,
      relayed,
    },
  )?;
  Ok(path)
}

/// `slots release`: the holder ends a granted or running request. Before a
/// start, it ends it unrun, which a `--place-of` ask re-asks from.
pub fn release(
  dirs: &Dirs,
  caller: &Caller,
  id: &str,
  result: Option<&str>,
  relayed: bool,
) -> Result<PathBuf, SlotError> {
  if let Some(text) = result {
    one_line("result", text)?;
  }
  let slots = read_strict(dirs);
  let r = request_of(&slots, id)?;
  ensure_asker(r, caller)?;
  if !r.holding() {
    return Err(wrong_state(r, "released"));
  }
  let path = requests_dir(dirs).join(id).join("end.json");
  write_once(
    &path,
    &EndRecord {
      schema: SLOT_SCHEMA.to_string(),
      result: result.map(str::to_string),
      relayed,
    },
  )?;
  Ok(path)
}

fn stamp(at: Option<OffsetDateTime>) -> Value {
  at.and_then(iso).map_or(Value::Null, Value::String)
}

/// The reader's `controller`: null when no session holds control.
pub fn controller_value(slots: &Slots) -> Value {
  slots.live_controller().map_or(Value::Null, |c| {
    json!({
      "session_id": c.record.session_id,
      "name": c.record.name,
      "pid": c.record.pid,
      "node": c.record.node,
      "taken_at": stamp(c.at),
      "predecessor": c.record.predecessor,
      "hv": c.record.hv,
    })
  })
}

/// The reader's `controller_unreadable`: null, or the path and reason of a
/// `controller.json` that is there and will not parse, so a face says on its
/// control line that nobody can tell who holds control, never NO CONTROLLER
/// (vc's ruling, 2026-10-26).
pub fn controller_unreadable_value(slots: &Slots) -> Value {
  slots
    .controller_unreadable
    .as_ref()
    .map_or(Value::Null, |u| json!({ "path": u.path, "why": u.why }))
}

/// The reader's `lanes`, by name.
///
/// **EACH LANE CARRIES THE QUEUE FOR ITS TYPE** (AC-04.6): the ids of the
/// requests asked for the lane's type and not yet answered, oldest first in
/// the queue's own order. A request asks by type, so a heavy ask waits on
/// every heavy lane and shows under each; membership and order are decided
/// here, once, and every face only draws them.
pub fn lanes_value(slots: &Slots) -> Value {
  let asked = waiting(slots);
  Value::Array(
    slots
      .lanes
      .iter()
      .map(|l| {
        let in_lane = |state: ReqState| {
          slots
            .requests
            .iter()
            .filter(|r| r.state() == state && r.lanes().contains(&l.name))
            .count()
        };
        let queue: Vec<&str> = asked
          .iter()
          .filter(|r| r.ask.record.kind == l.kind)
          .map(|r| r.id())
          .collect();
        json!({
          "name": l.name,
          "type": l.kind,
          "capacity": l.capacity,
          "load_under": l.load_under,
          "granted": in_lane(ReqState::Granted),
          "running": in_lane(ReqState::Running),
          "queue": queue,
        })
      })
      .collect(),
  )
}

/// The reader's `holds`: every hold standing, oldest first.
pub fn holds_value(slots: &Slots) -> Value {
  Value::Array(
    slots
      .holds
      .iter()
      .map(|h| {
        json!({
          "id": h.record.id,
          "scope": h.record.scope,
          "lane": h.record.lane,
          "condition": h.record.condition,
          "node": h.record.node,
          "held_at": stamp(h.at),
        })
      })
      .collect(),
  )
}

fn row(r: &Request) -> Value {
  let a = &r.ask.record;
  let relayed = r.start.as_ref().is_some_and(|s| s.record.relayed)
    || r.end.as_ref().is_some_and(|e| e.record.relayed);
  json!({
    "id": a.id,
    "type": a.kind,
    "what": a.what,
    "state": r.state().as_str(),
    "lanes": r.lanes(),
    "named": a.named,
    "tags": a.tags,
    "gates": a.gates,
    "asked_at": stamp(r.ask.at),
    "started_at": stamp(r.start.as_ref().and_then(|s| s.at)),
    "ended_at": stamp(r.finished_at()),
    "relayed": relayed,
    "by": a.by,
  })
}

/// Every request not finished, oldest first by its sort key, which is its own
/// `asked_at` or, for a hand-back, the `--place-of` request's: the one order
/// the queue and every lane's queue share.
fn queue_order(slots: &Slots) -> Vec<&Request> {
  let key = |r: &Request| {
    r.ask
      .record
      .place_of
      .as_deref()
      .and_then(|p| slots.request(p))
      .map_or(r.ask.at, |p| p.ask.at)
  };
  let mut open: Vec<&Request> = slots.requests.iter().filter(|r| !r.finished()).collect();
  open.sort_by(|a, b| key(a).cmp(&key(b)).then_with(|| a.id().cmp(b.id())));
  open
}

/// The reader's `queue`: every request not finished, in `queue_order`.
pub fn queue_value(slots: &Slots) -> Value {
  Value::Array(queue_order(slots).into_iter().map(row).collect())
}

/// The requests asked and not yet answered, oldest first in `queue_order`:
/// what the controller has to judge.
pub fn waiting(slots: &Slots) -> Vec<&Request> {
  queue_order(slots)
    .into_iter()
    .filter(|r| r.state() == ReqState::Asked)
    .collect()
}

/// The reader's `recent`: the requests that ended, were denied or were revoked
/// at a moment `in_window` admits, oldest end first.
///
/// **THE WINDOW IS THE CALLER'S, AND NO TIME IS HANDED IN** (D42: no
/// intentsvcs function takes a time). `estate::join` builds `in_window` from
/// its one reading of the estate's clock, `RECENT_SECONDS` back from it, so
/// this judges only the moments its own records carry.
pub fn recent_value(slots: &Slots, in_window: impl Fn(OffsetDateTime) -> bool) -> Value {
  let mut done: Vec<&Request> = slots
    .requests
    .iter()
    .filter(|r| r.finished_at().is_some_and(&in_window))
    .collect();
  done.sort_by(|a, b| {
    a.finished_at()
      .cmp(&b.finished_at())
      .then_with(|| a.id().cmp(b.id()))
  });
  Value::Array(done.into_iter().map(row).collect())
}

/// The controller's answer to an ask, which `slots wait` waits for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Answer {
  Granted,
  Denied,
  Revoked,
}

/// The answer a request carries, or `None` while it is still asked.
///
/// **A REVOKE ANSWERS WHENEVER IT LANDS**: the controller may end an ask it
/// never granted, and the asker learns it the way it learns a denial. A
/// request that has started or ended was granted first.
pub fn answer(r: &Request) -> Option<Answer> {
  match r.state() {
    ReqState::Asked => None,
    ReqState::Denied => Some(Answer::Denied),
    ReqState::Revoked => Some(Answer::Revoked),
    ReqState::Granted | ReqState::Running | ReqState::Ended => Some(Answer::Granted),
  }
}

/// `slots wait`: re-reads the request every `pause` until the controller has
/// answered it, and returns it as it then stands with the answer; `None` when
/// `limit` passed with it still asked. An answer already there returns at
/// once, and with no limit it waits as long as the ask stands.
///
/// **NO CLOCK IS READ** (D42): the time waited is the pauses counted, as the
/// daemon's lock wait counts its own, so `limit` is reached by counting and
/// the reads in between add a little to it.
pub fn wait(
  dirs: &Dirs,
  id: &str,
  limit: Option<Duration>,
  pause: Duration,
) -> Result<Option<(Request, Answer)>, SlotError> {
  let mut waited = Duration::ZERO;
  loop {
    let slots = read_strict(dirs);
    let r = request_of(&slots, id)?;
    if let Some(said) = answer(r) {
      return Ok(Some((r.clone(), said)));
    }
    if limit.is_some_and(|l| waited >= l) {
      return Ok(None);
    }
    std::thread::sleep(pause);
    waited += pause;
  }
}

/// One state change under `slots/requests/`: the file a step wrote.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
  Asked,
  Granted,
  Denied,
  Started,
  Ended,
  Revoked,
}

impl Step {
  /// The word a `slots watch` line opens with, so a reader filters on it.
  pub fn as_str(self) -> &'static str {
    match self {
      Self::Asked => "asked",
      Self::Granted => "granted",
      Self::Denied => "denied",
      Self::Started => "started",
      Self::Ended => "ended",
      Self::Revoked => "revoked",
    }
  }
}

/// The steps a request's files record, in the order they are written.
fn taken(r: &Request) -> Vec<Step> {
  let mut out = vec![Step::Asked];
  for (present, step) in [
    (r.grant.is_some(), Step::Granted),
    (r.deny.is_some(), Step::Denied),
    (r.start.is_some(), Step::Started),
    (r.end.is_some(), Step::Ended),
    (r.revoke.is_some(), Step::Revoked),
  ] {
    if present {
      out.push(step);
    }
  }
  out
}

/// Every state change between two reads of `slots/`: each file that appeared
/// is one step, in request id order and, within a request, in the order its
/// files are written (vc's ruling, 2026-10-26: `slots watch` reports every
/// change, the controller's own grants, denials and revokes among them, and a
/// reader filters by the step).
pub fn steps<'a>(before: &Slots, after: &'a Slots) -> Vec<(Step, &'a Request)> {
  let mut out = Vec::new();
  for r in &after.requests {
    let had = before.request(r.id()).map(taken).unwrap_or_default();
    for step in taken(r) {
      if !had.contains(&step) {
        out.push((step, r));
      }
    }
  }
  out
}

/// `slots watch`: hands `emit` every state change under `slots/requests/`,
/// re-reading every `pause`, and returns when `emit` says to stop.
///
/// **IT OPENS WITH AN `Asked` STEP FOR EACH ASK ALREADY WAITING**, oldest
/// first, so a watch armed after an ask still hears of it; from then on every
/// file a step writes is one call, in [`steps`] order. `emit` returns `false`
/// when its reader has gone, which is the one way a watch ends by itself.
pub fn watch(dirs: &Dirs, pause: Duration, mut emit: impl FnMut(Step, &Request) -> bool) {
  let mut before = read_strict(dirs);
  for r in waiting(&before) {
    if !emit(Step::Asked, r) {
      return;
    }
  }
  loop {
    std::thread::sleep(pause);
    let after = read_strict(dirs);
    for (step, r) in steps(&before, &after) {
      if !emit(step, r) {
        return;
      }
    }
    before = after;
  }
}

/// The asks waiting for the controller, oldest first, when `session_id` holds
/// control; `None` when it does not. A file that will not parse is pushed to
/// `unreadable`, as [`read`] does.
///
/// **IT READS `controller.json` AND STOPS THERE FOR ANY OTHER SESSION** (vc's
/// ruling, 2026-10-26): the prompt hook asks this on every prompt of every
/// session on the host, so a session that is not the controller pays one small
/// read and no more. It takes no lock and asks no daemon.
pub fn pending(
  dirs: &Dirs,
  session_id: &str,
  unreadable: &mut Vec<Unreadable>,
) -> Option<Vec<Request>> {
  let controller = match load::<ControllerRecord>(&controller_path(dirs)) {
    Ok(controller) => controller,
    Err(u) => {
      unreadable.push(u);
      return None;
    }
  };
  if !controller.is_some_and(|c| !c.record.released && c.record.session_id == session_id) {
    return None;
  }
  let slots = read(dirs, unreadable);
  Some(waiting(&slots).into_iter().cloned().collect())
}

/// Each session's name by its id, from Claude Code's own session files: how
/// `slots wait`, `slots watch` and `slots pending` name an asker. A session
/// file that will not parse is left out, and its session's lines show its id.
pub fn names(claude_dir: &Path) -> Result<BTreeMap<String, String>, super::EstateError> {
  let mut ignored = Vec::new();
  Ok(
    super::read_sessions(claude_dir, &mut ignored)?
      .into_iter()
      .map(|s| (s.session_id, s.name))
      .collect(),
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Host {
    _dir: tempfile::TempDir,
    dirs: Dirs,
    claude: PathBuf,
  }

  fn host() -> Host {
    let dir = tempfile::tempdir().expect("tempdir");
    let dirs = Dirs::at_home(dir.path());
    let claude = dir.path().join("claude");
    std::fs::create_dir_all(claude.join("sessions")).expect("mkdir sessions");
    Host {
      _dir: dir,
      dirs,
      claude,
    }
  }

  fn who(session_id: &str, name: &str, pid: u32) -> Caller {
    Caller {
      session_id: session_id.to_string(),
      name: name.to_string(),
      pid,
      node: Some("cc".to_string()),
    }
  }

  /// A Claude Code session file for `pid`, its start read through the estate
  /// module so liveness agrees with the reader on this OS.
  fn session_file(h: &Host, pid: u32, session_id: &str, name: &str) {
    let start = super::super::proc_start_of(pid)
      .expect("read a start")
      .unwrap_or_else(|| "not running".to_string());
    let body = json!({
      "pid": pid,
      "sessionId": session_id,
      "cwd": "/",
      "name": name,
      "status": "idle",
      "procStart": start,
      "startedAt": 1_792_820_660_805_u64,
      "updatedAt": 1_792_848_120_372_u64,
    });
    std::fs::write(
      h.claude.join("sessions").join(format!("{pid}.json")),
      serde_json::to_vec(&body).expect("json"),
    )
    .expect("write a session file");
  }

  fn dead_pid() -> u32 {
    let mut child = std::process::Command::new("/usr/bin/true")
      .spawn()
      .expect("spawn true");
    let pid = child.id();
    child.wait().expect("wait for true");
    pid
  }

  fn quiet() -> Result<f64, Unreadable> {
    Ok(1.0)
  }

  /// The controller on a host with `heavy`, `cargo` and `bats` lanes of
  /// capacity one.
  fn desk(h: &Host) -> Caller {
    let ctl = who("ctl", "molt-vc", std::process::id());
    session_file(h, ctl.pid, "ctl", "molt-vc");
    control_take(&h.dirs, &h.claude, &ctl, None).expect("first take");
    for lane in ["heavy", "cargo", "bats"] {
      lane_open(&h.dirs, &ctl, lane, "heavy", 1, None).expect("open a lane");
    }
    ctl
  }

  fn named(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
    pairs
      .iter()
      .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
      .collect()
  }

  fn state(h: &Host, id: &str) -> ReqState {
    read_strict(&h.dirs)
      .request(id)
      .map(Request::state)
      .expect("the request")
  }

  fn lanes(names: &[&str]) -> Vec<String> {
    names.iter().map(|s| (*s).to_string()).collect()
  }

  /// ST0086 AT-04.3
  /// ST0086 AT-04.8
  /// ST0086 AT-04.12
  ///
  /// AC-04.3, AC-04.8 and AC-04.12: a request folds through its files, a grant
  /// is checked against every lane it names, and a start must name the set the
  /// ask named.
  #[test]
  fn a_request_folds_through_its_files_and_capacity_holds_in_every_lane_named() {
    let h = host();
    let ctl = desk(&h);
    let me = who("me", "intent-cc", 1);
    let set = named(&[("tip", "7a3552f1e")]);
    let (a, _) = ask(
      &h.dirs,
      &me,
      "heavy",
      "0648 judge",
      AskArgs {
        named: set.clone(),
        ..AskArgs::default()
      },
    )
    .expect("ask a");
    let (b, _) = ask(&h.dirs, &me, "heavy", "dc's judge", AskArgs::default()).expect("ask b");
    assert_eq!(state(&h, &a), ReqState::Asked);

    grant(
      &h.dirs,
      &ctl,
      &a,
      &lanes(&["heavy", "cargo", "bats"]),
      &quiet,
    )
    .expect("grant a");
    assert_eq!(state(&h, &a), ReqState::Granted);
    match grant(&h.dirs, &ctl, &b, &lanes(&["cargo"]), &quiet) {
      Err(SlotError::OverCapacity {
        lane,
        held,
        capacity,
      }) => assert_eq!((lane.as_str(), held, capacity), ("cargo", 1, 1)),
      other => panic!("a second cargo grant must be over capacity: {other:?}"),
    }

    let wrong = named(&[("tip", "deadbeef")]);
    match start(&h.dirs, &me, &a, wrong, false) {
      Err(e @ SlotError::NamedMismatch { .. }) => assert_eq!(
        e.to_string(),
        format!("request `{a}` was asked naming tip=7a3552f1e, and this names tip=deadbeef")
      ),
      other => panic!("a start on another set must be refused naming both: {other:?}"),
    }
    start(&h.dirs, &me, &a, set, false).expect("start a");
    assert_eq!(state(&h, &a), ReqState::Running);
    let slots = read_strict(&h.dirs);
    assert_eq!(slots.running_by(), BTreeSet::from(["me".to_string()]));
    let lanes_now = lanes_value(&slots);
    assert_eq!(lanes_now[1]["name"], "cargo");
    assert_eq!(lanes_now[1]["running"], 1);

    release(&h.dirs, &me, &a, Some("fmt rc=0"), false).expect("release a");
    assert_eq!(state(&h, &a), ReqState::Ended);
    grant(&h.dirs, &ctl, &b, &lanes(&["cargo"]), &quiet).expect("a freed lane grants");
    assert!(matches!(
      release(&h.dirs, &me, &a, None, false),
      Err(SlotError::WrongState { .. })
    ));
  }

  /// ST0086 AT-04.8
  ///
  /// AC-04.8: `--lane none` grants a negligible run into no lane, counting
  /// against no capacity.
  #[test]
  fn a_grant_into_no_lane_counts_against_no_capacity() {
    let h = host();
    let ctl = desk(&h);
    let me = who("me", "intent-cc", 1);
    let (busy, _) = ask(&h.dirs, &me, "heavy", "busy", AskArgs::default()).expect("ask");
    grant(&h.dirs, &ctl, &busy, &lanes(&["cargo"]), &quiet).expect("fill cargo");
    let (small, _) = ask(&h.dirs, &me, "negligible", "a landing", AskArgs::default()).expect("ask");
    grant(&h.dirs, &ctl, &small, &lanes(&[NO_LANE]), &quiet).expect("grant into none");
    let slots = read_strict(&h.dirs);
    assert!(slots.request(&small).expect("small").lanes().is_empty());
    assert_eq!(slots.held_in("cargo"), 1);
    assert!(matches!(
      grant(&h.dirs, &ctl, &busy, &lanes(&[NO_LANE, "cargo"]), &quiet),
      Err(SlotError::WrongState { .. } | SlotError::BadArg { .. })
    ));
  }

  /// ST0086 AT-04.9
  ///
  /// AC-04.9: a desk hold refuses every grant and a lane hold every grant
  /// naming its lane, each naming the hold, until released.
  #[test]
  fn a_hold_refuses_grants_naming_it_until_it_is_released() {
    let h = host();
    let ctl = desk(&h);
    let me = who("me", "intent-cc", 1);
    let (a, _) = ask(&h.dirs, &me, "heavy", "a", AskArgs::default()).expect("ask");
    let (desk_hold, _) = hold(&h.dirs, &ctl, None, "until hv says continue").expect("hold");
    match grant(&h.dirs, &ctl, &a, &lanes(&["bats"]), &quiet) {
      Err(SlotError::Held { id, condition, .. }) => {
        assert_eq!(id, desk_hold);
        assert_eq!(condition, "until hv says continue");
      }
      other => panic!("a desk hold refuses every grant: {other:?}"),
    }
    assert_eq!(holds_value(&read_strict(&h.dirs))[0]["scope"], "desk");
    unhold(&h.dirs, &ctl, None, &desk_hold).expect("unhold");
    assert_eq!(holds_value(&read_strict(&h.dirs)), json!([]));

    let (lane_hold, _) = hold(&h.dirs, &ctl, Some("bats"), "bats is broken").expect("hold bats");
    assert!(matches!(
      grant(&h.dirs, &ctl, &a, &lanes(&["bats"]), &quiet),
      Err(SlotError::Held { .. })
    ));
    grant(&h.dirs, &ctl, &a, &lanes(&["cargo"]), &quiet).expect("another lane grants");
    assert!(matches!(
      unhold(&h.dirs, &ctl, None, &lane_hold),
      Err(SlotError::NoHold { .. })
    ));
    unhold(&h.dirs, &ctl, Some("bats"), &lane_hold).expect("unhold the lane");
  }

  /// ST0086 AT-04.13
  ///
  /// AC-04.13: a not-before gate refuses a grant until its request has ended.
  #[test]
  fn a_not_before_gate_refuses_a_grant_until_its_request_has_ended() {
    let h = host();
    let ctl = desk(&h);
    let me = who("me", "intent-cc", 1);
    let (first, _) = ask(&h.dirs, &me, "heavy", "build all", AskArgs::default()).expect("ask");
    let (after, _) = ask(
      &h.dirs,
      &me,
      "light",
      "the drive",
      AskArgs {
        not_before: vec![first.clone()],
        ..AskArgs::default()
      },
    )
    .expect("ask gated");
    match grant(&h.dirs, &ctl, &after, &lanes(&["bats"]), &quiet) {
      Err(SlotError::GateUnmet { gate, .. }) => assert_eq!(gate, first),
      other => panic!("an unmet gate refuses: {other:?}"),
    }
    grant(&h.dirs, &ctl, &first, &lanes(&["heavy"]), &quiet).expect("grant first");
    release(&h.dirs, &me, &first, None, false).expect("end first");
    grant(&h.dirs, &ctl, &after, &lanes(&["bats"]), &quiet).expect("the gate is met");
  }

  /// ST0086 AT-04.1
  /// ST0086 AT-04.10
  ///
  /// AC-04.1 and AC-04.10: the controller's writes are refused from any other
  /// session, and a holder's from any session but the asker.
  #[test]
  fn only_the_controller_grants_and_only_the_asker_starts() {
    let h = host();
    let ctl = desk(&h);
    let me = who("me", "intent-cc", 1);
    let other = who("other", "intent-dc", 2);
    let (a, _) = ask(&h.dirs, &me, "heavy", "a", AskArgs::default()).expect("ask");
    assert!(matches!(
      grant(&h.dirs, &other, &a, &lanes(&["cargo"]), &quiet),
      Err(SlotError::NotController { .. })
    ));
    assert!(matches!(
      deny(&h.dirs, &other, &a, "not yours"),
      Err(SlotError::NotController { .. })
    ));
    assert!(matches!(
      revoke(&h.dirs, &other, &a, "not yours"),
      Err(SlotError::NotController { .. })
    ));
    assert!(matches!(
      lane_open(&h.dirs, &other, "cargo", "heavy", 9, None),
      Err(SlotError::NotController { .. })
    ));
    assert!(matches!(
      lane_close(&h.dirs, &other, "cargo"),
      Err(SlotError::NotController { .. })
    ));
    assert!(matches!(
      hold(&h.dirs, &other, None, "mine now"),
      Err(SlotError::NotController { .. })
    ));
    grant(&h.dirs, &ctl, &a, &lanes(&["cargo"]), &quiet).expect("grant");
    assert!(matches!(
      start(&h.dirs, &other, &a, BTreeMap::new(), false),
      Err(SlotError::NotAsker { .. })
    ));
    start(&h.dirs, &me, &a, BTreeMap::new(), false).expect("the asker starts");
    assert!(matches!(
      release(&h.dirs, &other, &a, None, false),
      Err(SlotError::NotAsker { .. })
    ));
  }

  /// ST0086 AT-04.2
  ///
  /// AC-04.2: a repeated open changes the lane's capacity, a close is refused
  /// while anything is granted in the lane and naming the count, a close of an
  /// idle lane removes it, and a lane's type is whatever the controller names.
  #[test]
  fn a_lane_reopens_to_a_new_capacity_and_closes_only_when_idle() {
    let h = host();
    let ctl = desk(&h);
    let me = who("me", "intent-cc", 1);
    let lane_named = |name: &str| -> Value {
      let all = lanes_value(&read_strict(&h.dirs));
      all
        .as_array()
        .expect("lanes is an array")
        .iter()
        .find(|l| l["name"] == name)
        .cloned()
        .unwrap_or(Value::Null)
    };

    lane_open(&h.dirs, &ctl, "cargo", "heavy", 2, None).expect("re-open at two");
    assert_eq!(lane_named("cargo")["capacity"], 2);

    lane_open(&h.dirs, &ctl, "zebra", "zebra", 1, None).expect("open a lane of any type");
    assert_eq!(lane_named("zebra")["type"], "zebra");

    let (a, _) = ask(&h.dirs, &me, "heavy", "a", AskArgs::default()).expect("ask");
    grant(&h.dirs, &ctl, &a, &lanes(&["cargo"]), &quiet).expect("grant into cargo");
    match lane_close(&h.dirs, &ctl, "cargo") {
      Err(SlotError::LaneBusy { lane, count }) => assert_eq!((lane.as_str(), count), ("cargo", 1)),
      other => panic!("a busy lane must refuse its close: {other:?}"),
    }
    assert_eq!(
      lane_named("cargo")["capacity"],
      2,
      "a refused close writes nothing"
    );

    release(&h.dirs, &me, &a, None, false).expect("end a");
    lane_close(&h.dirs, &ctl, "cargo").expect("an idle lane closes");
    assert_eq!(lane_named("cargo"), Value::Null);
    assert!(matches!(
      lane_close(&h.dirs, &ctl, "cargo"),
      Err(SlotError::NoLane { .. })
    ));
  }

  /// ST0086 AT-04.1
  /// ST0086 AT-04.11
  ///
  /// AC-04.1 and AC-04.11: a take is refused while the controller is live,
  /// passes over a gone one under its own name, and needs hv's words under
  /// another; the record names its predecessor.
  #[test]
  fn a_take_is_a_handover_over_a_gone_controller_under_its_own_name() {
    let h = host();
    let gone = dead_pid();
    let old = who("old", "molt-vc", gone);
    session_file(&h, gone, "old", "molt-vc");
    control_take(&h.dirs, &h.claude, &old, None).expect("a first take needs no words");

    let fresh = who("fresh", "molt-vc", std::process::id());
    session_file(&h, fresh.pid, "fresh", "molt-vc");
    let peer = who("peer", "intent-dc", 3);
    assert!(matches!(
      control_take(&h.dirs, &h.claude, &peer, None),
      Err(SlotError::NeedsHv { .. })
    ));
    let taken =
      control_take(&h.dirs, &h.claude, &fresh, None).expect("same name over a gone controller");
    assert_eq!(taken.predecessor.expect("a predecessor").session_id, "old");

    match control_take(&h.dirs, &h.claude, &peer, Some("hv: dc takes the desk")) {
      Err(SlotError::ControllerLive { name, .. }) => assert_eq!(name, "molt-vc"),
      other => panic!("a live controller refuses every take: {other:?}"),
    }

    control_release(&h.dirs, &fresh).expect("release");
    assert_eq!(controller_value(&read_strict(&h.dirs)), Value::Null);
    assert!(matches!(
      control_take(&h.dirs, &h.claude, &peer, None),
      Err(SlotError::NeedsHv { .. })
    ));
    control_take(&h.dirs, &h.claude, &peer, Some("hv: dc takes the desk")).expect("hv's words");
    let ctl = controller_value(&read_strict(&h.dirs));
    assert_eq!(ctl["name"], "intent-dc");
    assert_eq!(ctl["hv"], "hv: dc takes the desk");
    assert_eq!(ctl["predecessor"]["name"], "molt-vc");
  }

  /// A hand-back keeps its place: `--place-of` re-asks from a request granted
  /// and released before it started, and sorts by its `asked_at`.
  #[test]
  fn a_hand_back_keeps_its_place_only_from_a_request_released_unrun() {
    let h = host();
    let ctl = desk(&h);
    let me = who("me", "intent-cc", 1);
    let (first, _) = ask(&h.dirs, &me, "heavy", "first", AskArgs::default()).expect("ask");
    let (later, _) = ask(&h.dirs, &me, "heavy", "later", AskArgs::default()).expect("ask");
    assert!(matches!(
      ask(
        &h.dirs,
        &me,
        "heavy",
        "too soon",
        AskArgs {
          place_of: Some(first.clone()),
          ..AskArgs::default()
        }
      ),
      Err(SlotError::BadPlaceOf { .. })
    ));
    grant(&h.dirs, &ctl, &first, &lanes(&["heavy"]), &quiet).expect("grant");
    release(&h.dirs, &me, &first, Some("handed back"), false).expect("hand back");
    let (again, _) = ask(
      &h.dirs,
      &me,
      "heavy",
      "first again",
      AskArgs {
        place_of: Some(first.clone()),
        ..AskArgs::default()
      },
    )
    .expect("re-ask in place");
    let queue = queue_value(&read_strict(&h.dirs));
    let order: Vec<&str> = queue
      .as_array()
      .expect("rows")
      .iter()
      .filter_map(|r| r["id"].as_str())
      .collect();
    assert_eq!(order, vec![again.as_str(), later.as_str()]);
    let since = super::super::clock::now() - time::Duration::seconds(RECENT_SECONDS);
    let recent = recent_value(&read_strict(&h.dirs), |at| at >= since);
    assert_eq!(recent[0]["id"], first.as_str());
    assert_eq!(recent[0]["state"], "ended");
  }

  /// ST0086 AT-04.6
  ///
  /// AC-04.6, in the reader: every lane carries the queue for its type, the
  /// asks not yet answered, oldest first in the queue's own order, so a heavy
  /// ask waits on every heavy lane and on no lane of another type, and a
  /// granted request has left every queue.
  #[test]
  fn every_lane_carries_the_queue_for_its_type_oldest_first() {
    let h = host();
    let ctl = desk(&h);
    lane_open(&h.dirs, &ctl, "light", "light", 1, None).expect("open a light lane");
    let me = who("me", "intent-cc", 1);
    let (first, _) = ask(&h.dirs, &me, "heavy", "first", AskArgs::default()).expect("ask");
    let (drive, _) = ask(&h.dirs, &me, "light", "a drive", AskArgs::default()).expect("ask");
    let (second, _) = ask(&h.dirs, &me, "heavy", "second", AskArgs::default()).expect("ask");
    let (held, _) = ask(&h.dirs, &me, "heavy", "held", AskArgs::default()).expect("ask");
    grant(&h.dirs, &ctl, &held, &lanes(&["bats"]), &quiet).expect("grant");

    let lanes_now = lanes_value(&read_strict(&h.dirs));
    let queue_of = |name: &str| -> Vec<String> {
      lanes_now
        .as_array()
        .expect("lanes")
        .iter()
        .find(|l| l["name"] == name)
        .expect("the lane")["queue"]
        .as_array()
        .expect("a queue")
        .iter()
        .map(|id| id.as_str().expect("an id").to_string())
        .collect()
    };
    for heavy in ["heavy", "cargo", "bats"] {
      assert_eq!(
        queue_of(heavy),
        vec![first.clone(), second.clone()],
        "{heavy}"
      );
    }
    assert_eq!(queue_of("light"), vec![drive]);
  }

  /// ST0086 AT-04.4
  ///
  /// AC-04.4: every file a step writes is one state change between two reads,
  /// the controller's own among them, and a request's answer is its grant,
  /// denial or revoke, which a start or an end does not undo.
  #[test]
  fn every_file_a_step_writes_is_one_state_change_and_answers_follow_the_fold() {
    let h = host();
    let ctl = desk(&h);
    let me = who("me", "intent-cc", 1);
    let mut before = read_strict(&h.dirs);
    let mut changes = |h: &Host| -> Vec<(Step, String)> {
      let after = read_strict(&h.dirs);
      let out = steps(&before, &after)
        .into_iter()
        .map(|(step, r)| (step, r.id().to_string()))
        .collect();
      before = after;
      out
    };

    let (a, _) = ask(&h.dirs, &me, "heavy", "a judge", AskArgs::default()).expect("ask a");
    assert_eq!(changes(&h), vec![(Step::Asked, a.clone())]);
    let asked = read_strict(&h.dirs);
    assert_eq!(answer(asked.request(&a).expect("a")), None);

    grant(&h.dirs, &ctl, &a, &lanes(&["cargo"]), &quiet).expect("grant a");
    assert_eq!(changes(&h), vec![(Step::Granted, a.clone())]);
    start(&h.dirs, &me, &a, BTreeMap::new(), false).expect("start a");
    assert_eq!(changes(&h), vec![(Step::Started, a.clone())]);
    release(&h.dirs, &me, &a, Some("rc 0"), false).expect("release a");
    assert_eq!(changes(&h), vec![(Step::Ended, a.clone())]);

    let (b, _) = ask(&h.dirs, &me, "heavy", "b", AskArgs::default()).expect("ask b");
    deny(&h.dirs, &ctl, &b, "not today").expect("deny b");
    let (c, _) = ask(&h.dirs, &me, "heavy", "c", AskArgs::default()).expect("ask c");
    revoke(&h.dirs, &ctl, &c, "the holder is gone").expect("revoke c");
    assert_eq!(
      changes(&h),
      vec![
        (Step::Asked, b.clone()),
        (Step::Denied, b.clone()),
        (Step::Asked, c.clone()),
        (Step::Revoked, c.clone()),
      ]
    );
    assert_eq!(
      changes(&h),
      Vec::<(Step, String)>::new(),
      "nothing new, nothing reported"
    );

    let now = read_strict(&h.dirs);
    let answer_of = |id: &str| answer(now.request(id).expect("the request"));
    assert_eq!(
      answer_of(a.as_str()),
      Some(Answer::Granted),
      "an ended run was granted"
    );
    assert_eq!(answer_of(b.as_str()), Some(Answer::Denied));
    assert_eq!(answer_of(c.as_str()), Some(Answer::Revoked));
  }

  /// ST0086 AT-04.4
  ///
  /// AC-04.4: `wait` returns at the limit with the ask still standing, and at
  /// once with the request when the controller has answered it.
  #[test]
  fn wait_returns_at_the_limit_unanswered_and_at_once_answered() {
    let h = host();
    let ctl = desk(&h);
    let me = who("me", "intent-cc", 1);
    let (a, _) = ask(&h.dirs, &me, "heavy", "a judge", AskArgs::default()).expect("ask");
    let tick = Duration::from_millis(1);
    assert_eq!(
      wait(&h.dirs, &a, Some(Duration::ZERO), tick).expect("wait"),
      None,
      "still asked at a zero limit"
    );
    grant(&h.dirs, &ctl, &a, &lanes(&["heavy"]), &quiet).expect("grant");
    let (answered, said) = wait(&h.dirs, &a, None, tick)
      .expect("wait")
      .expect("answered");
    assert_eq!(said, Answer::Granted);
    assert_eq!(answered.state(), ReqState::Granted);
    assert_eq!(answered.lanes().to_vec(), vec!["heavy".to_string()]);
    assert!(matches!(
      wait(&h.dirs, "01NOSUCHREQUEST", Some(Duration::ZERO), tick),
      Err(SlotError::NoRequest { .. })
    ));
  }

  /// ST0086 AT-04.4
  ///
  /// AC-04.4: `watch` opens with the asks already waiting, reports the next
  /// change it reads, and returns when its reader says stop.
  #[test]
  fn watch_opens_with_the_waiting_asks_and_ends_when_its_reader_stops() {
    let h = host();
    let ctl = desk(&h);
    let me = who("me", "intent-cc", 1);
    let (granted, _) = ask(&h.dirs, &me, "heavy", "running", AskArgs::default()).expect("ask");
    grant(&h.dirs, &ctl, &granted, &lanes(&["cargo"]), &quiet).expect("grant");
    let (a, _) = ask(&h.dirs, &me, "heavy", "a judge", AskArgs::default()).expect("ask");

    let mut heard: Vec<(Step, String)> = Vec::new();
    watch(&h.dirs, Duration::from_millis(5), |step, r| {
      heard.push((step, r.id().to_string()));
      if step == Step::Asked {
        // A change made while the watch runs, which its next read reports.
        grant(&h.dirs, &ctl, &a, &lanes(&["heavy"]), &quiet).expect("grant a");
        return true;
      }
      false
    });
    assert_eq!(heard, vec![(Step::Asked, a.clone()), (Step::Granted, a)]);
  }

  /// ST0086 AT-04.4
  ///
  /// AC-04.4, the prompt hook's question: the waiting asks go to the session
  /// holding control and to no other, and a controller record that will not
  /// parse is reported and names nothing.
  #[test]
  fn pending_names_the_waiting_asks_to_the_controller_alone() {
    let h = host();
    let ctl = desk(&h);
    let me = who("me", "intent-cc", 1);
    let (a, _) = ask(&h.dirs, &me, "heavy", "a judge", AskArgs::default()).expect("ask");
    let mut unreadable = Vec::new();
    let to_ctl = pending(&h.dirs, &ctl.session_id, &mut unreadable).expect("the controller");
    assert_eq!(
      to_ctl.iter().map(Request::id).collect::<Vec<_>>(),
      [a.as_str()]
    );
    assert_eq!(pending(&h.dirs, &me.session_id, &mut unreadable), None);
    assert!(unreadable.is_empty(), "{unreadable:?}");
    assert_eq!(
      names(&h.claude).expect("names").get(&ctl.session_id),
      Some(&"molt-vc".to_string())
    );

    std::fs::write(controller_path(&h.dirs), "not json").expect("break the record");
    assert_eq!(pending(&h.dirs, &ctl.session_id, &mut unreadable), None);
    assert_eq!(unreadable.len(), 1, "{unreadable:?}");
  }

  /// ST0086 AT-04.6
  ///
  /// vc's condition on the prompt hook: a `controller.json` that will not
  /// parse is carried as `controller_unreadable`, so the control line names it,
  /// while `controller` stays null and the record is still listed unreadable.
  #[test]
  fn an_unparsable_controller_record_is_carried_as_unreadable() {
    let h = host();
    desk(&h);
    assert_eq!(
      controller_unreadable_value(&read_strict(&h.dirs)),
      Value::Null
    );
    std::fs::write(controller_path(&h.dirs), "not json").expect("break the record");
    let mut unreadable = Vec::new();
    let slots = read(&h.dirs, &mut unreadable);
    assert!(slots.controller.is_none());
    let u = controller_unreadable_value(&slots);
    assert!(
      u["path"]
        .as_str()
        .is_some_and(|p| p.ends_with("controller.json")),
      "{u}"
    );
    assert!(
      u["why"].as_str().is_some_and(|w| w.contains("not JSON")),
      "{u}"
    );
    assert_eq!(unreadable.len(), 1, "{unreadable:?}");
  }
}
