//! The estate: every Claude Code session on this host, joined to the Intent
//! project, node, focus and claims it stands in (ST0086 WP-01).
//!
//! **ONE READER, THREE RENDERERS.** [`status`] returns one serialisable
//! [`EstateStatus`] (schema [`STATUS_SCHEMA`]), and `intent estate status`,
//! intentd's `/op` and the TUI all render that value. The join is pure
//! ([`join`]); [`status`] only gathers what it joins.
//!
//! **CLAUDE CODE'S OWN SESSION FILES ARE THE PRESENCE RECORD, READ AND NEVER
//! WRITTEN** (hv's ruling, 2026-10-24). Claude Code keeps
//! `<claude dir>/sessions/<pid>.json` current for every session. Its format is
//! not a published contract, so [`PINNED`] names exactly the fields read, a
//! test pins them against a scrubbed real fixture, and a file that lacks one is
//! listed as unreadable with the field named, never dropped.
//!
//! **NOTHING IT CAN JOIN IS STORED.** Node, focus, claims and paused come from
//! the board header whose session id matches the session's, so one fact keeps
//! one home. Intent writes only what nothing else holds: a WIP line for a
//! session with no board, in `<state>/estate/<session-id>.json`.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use thiserror::Error;
use time::OffsetDateTime;

use crate::project::{Project, ProjectError};
use crate::userstate::{Dirs, UserStateError};
use crate::write_set::{WriteError, write_atomically};

pub mod census;
pub mod clock;
pub mod usage;

/// The schema of the value [`status`] returns.
pub const STATUS_SCHEMA: &str = "intent/estate-status@1";

/// The schema of Intent's own per-session record.
pub const RECORD_SCHEMA: &str = "intent/estate-session@1";

/// The board schemas whose node header this build reads. The header carries
/// the same keys under both, so one narrow read serves a fleet that has not
/// all migrated.
const BOARD_SCHEMAS: &[&str] = &["intent/board@3.0", "intent/board@4.0"];

/// The session-file fields the reader relies on, and nothing else.
pub const PINNED: &[&str] = &[
  "pid",
  "procStart",
  "sessionId",
  "cwd",
  "name",
  "status",
  "startedAt",
  "updatedAt",
];

/// `ps` by absolute path: a judging run's PATH holds only the tools it links.
#[cfg(not(target_os = "linux"))]
const PS: &str = "/bin/ps";

/// `uname` by absolute path, for the same reason.
const UNAME: &str = "/usr/bin/uname";

#[derive(Debug, Error)]
pub enum EstateError {
  #[error(transparent)]
  Dirs(#[from] UserStateError),
  #[error("`{}` could not be listed", path.display())]
  Unlistable {
    path: PathBuf,
    #[source]
    cause: std::io::Error,
  },
  #[error("`{session_id}` is not a session id")]
  BadSessionId { session_id: String },
  #[error("the WIP line {why}")]
  BadLine { why: String },
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
  #[error("`{class}` is not a census class name")]
  BadClass { class: String },
  #[error("`{pattern}` is not a pattern: {why}")]
  BadPattern { pattern: String, why: String },
  #[error("the statusline input {why}")]
  BadStatusline { why: String },
}

impl crate::remedy::Remedy for EstateError {
  fn remedy(&self) -> String {
    match self {
      Self::Dirs(e) => e.remedy(),
      Self::Unlistable { .. } => "check the directory's permissions. Nothing was read or written.".to_string(),
      Self::BadSessionId { .. } => "pass the session's own id, as Claude Code records it in `sessionId`: letters, digits and dashes. Nothing was written.".to_string(),
      Self::BadLine { .. } => "give one non-empty line, or `--clear` to remove the record. Nothing was written.".to_string(),
      Self::Unwritable { .. } => "check that the estate directory is writable. The record was not changed.".to_string(),
      Self::Unremovable { .. } => "check the file's permissions and remove it by hand if it should go. Nothing else was removed.".to_string(),
      Self::BadClass { .. } => "name the class with letters, digits, `-` and `_` only: it is the watch file's name. Nothing was written.".to_string(),
      Self::BadPattern { .. } => "give a regular expression in Rust's `regex` syntax, matched against the process's whole command line. Nothing was written.".to_string(),
      Self::BadStatusline { .. } => "hand it the JSON Claude Code gives a statusline command, unchanged, on stdin. Nothing was written.".to_string(),
      Self::NoLiveness { .. } => "prune must read which processes are running (`/bin/ps` on macOS, `/proc` on Linux) to tell a gone session from a live one, so nothing was removed. Run `intent estate status` to see what `ps` reported.".to_string(),
    }
  }
}

/// One Claude Code session file, as far as the reader relies on it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaudeSession {
  pub path: PathBuf,
  pub pid: u32,
  /// UTC ctime, as Claude Code writes it.
  pub proc_start: String,
  pub session_id: String,
  pub cwd: PathBuf,
  pub name: String,
  /// Kept as found: `busy` and `idle`, and `shell` on 2026-10-24.
  pub status: String,
  pub started_at: String,
  pub updated_at: String,
}

/// A file or a tool the reader could not read, listed rather than dropped.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Unreadable {
  pub path: String,
  pub why: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub pid: Option<u32>,
}

/// Intent's own record for one session: what nothing else holds.
///
/// **ITS STAMP IS THE FILE'S MTIME, NOT A FIELD** (D42): the record is stamped
/// BY the atomic rename that writes it, so nothing asks what time it is.
/// `wip_at` is filled from that mtime when the record is read, and never
/// written.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WipRecord {
  pub schema: String,
  pub session_id: String,
  pub wip: String,
  #[serde(skip)]
  pub wip_at: Option<String>,
}

/// The node header of one board, read narrowly.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct BoardHeader {
  pub moniker: String,
  #[serde(default)]
  pub session_id: Option<String>,
  pub status: String,
  pub focus: String,
  #[serde(default)]
  pub claims: Vec<String>,
}

/// The project a session's cwd stands in, with its boards' headers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectAt {
  pub root: PathBuf,
  pub name: String,
  /// The cwd was in a git worktree, folded to its main checkout.
  pub worktree: bool,
  pub boards: Vec<BoardHeader>,
}

/// What [`join`] joins: everything [`status`] read, and nothing it computed.
#[derive(Debug, Clone, Default)]
pub struct Gathered {
  pub host: Option<String>,
  pub sessions: Vec<ClaudeSession>,
  /// Each live pid's start time, or why `ps` could not say.
  pub liveness: Option<Result<BTreeMap<u32, String>, Unreadable>>,
  /// Keyed by the session's cwd; `None` where it stands in no project.
  pub projects: BTreeMap<PathBuf, Option<ProjectAt>>,
  pub records: BTreeMap<String, WipRecord>,
  pub census: census::Census,
  pub usage: usage::Usage,
  pub unreadable: Vec<Unreadable>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum State {
  Gone,
  Paused,
  Heavy,
  Busy,
  Idle,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProjectRef {
  pub root: PathBuf,
  pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SessionRow {
  pub session_id: String,
  pub name: String,
  pub pid: u32,
  pub project: Option<ProjectRef>,
  pub worktree: bool,
  pub node: Option<String>,
  pub claude_status: String,
  pub state: State,
  pub started_at: String,
  pub updated_at: String,
  /// The slots this session holds (WP-04); empty until then.
  pub slots: Vec<String>,
  pub claims: Vec<String>,
  pub line: Option<String>,
  pub line_is_wip: bool,
  pub line_at: Option<String>,
}

/// The estate, as every renderer receives it.
///
/// **EVERY KEY IS PRESENT FROM WP-01**, so a renderer never branches on a
/// missing one: `controller`, `lanes` and `queue` hold the agreed empty
/// shapes until WP-04 fills and types them.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EstateStatus {
  pub schema: String,
  pub host: Option<String>,
  pub version: String,
  pub taken_at: String,
  pub census: census::Census,
  pub controller: Value,
  pub lanes: Value,
  pub queue: Value,
  pub usage: usage::Usage,
  pub sessions: Vec<SessionRow>,
  pub unreadable: Vec<Unreadable>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WipWritten {
  pub path: PathBuf,
  pub cleared: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Pruned {
  pub removed: Vec<PathBuf>,
}

/// `<state>/estate`: Intent's own records, one per session.
pub fn estate_dir(dirs: &Dirs) -> PathBuf {
  dirs.state.join("estate")
}

/// `<claude dir>/sessions`: Claude Code's own session files.
pub fn sessions_dir(claude_dir: &Path) -> PathBuf {
  claude_dir.join("sessions")
}

/// An epoch-millisecond stamp as ISO-8601 UTC with a `Z`.
fn iso_ms(ms: i64) -> Option<String> {
  let at = OffsetDateTime::from_unix_timestamp_nanos(i128::from(ms) * 1_000_000).ok()?;
  iso(at)
}

fn iso(at: OffsetDateTime) -> Option<String> {
  at.format(time::macros::format_description!(
    "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3]Z"
  ))
  .ok()
}

/// When a file was last written, as the filesystem recorded it: data about
/// when something happened, not a clock reading (D42).
fn stamped_at(path: &Path) -> Option<String> {
  let modified = std::fs::metadata(path).and_then(|m| m.modified()).ok()?;
  iso(OffsetDateTime::from(modified))
}

/// Runs of whitespace as one space, so `Oct  4` and `Oct 4` compare equal.
fn squeeze(text: &str) -> String {
  text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// One session file, or why it is unreadable, naming the field.
pub fn parse_session(path: &Path, text: &str) -> Result<ClaudeSession, Unreadable> {
  let pid_from_name = path
    .file_stem()
    .and_then(|s| s.to_str())
    .and_then(|s| s.parse().ok());
  let unreadable = |why: String| Unreadable {
    path: path.display().to_string(),
    why,
    pid: pid_from_name,
  };
  let doc: Value = serde_json::from_str(text).map_err(|e| unreadable(format!("not JSON: {e}")))?;
  let field = |key: &str| {
    doc
      .get(key)
      .ok_or_else(|| unreadable(format!("missing `{key}`")))
  };
  let string = |key: &str| {
    field(key)?
      .as_str()
      .map(str::to_string)
      .ok_or_else(|| unreadable(format!("`{key}` is not a string")))
  };
  let stamp = |key: &str| {
    field(key)?
      .as_i64()
      .and_then(iso_ms)
      .ok_or_else(|| unreadable(format!("`{key}` is not a time in epoch milliseconds")))
  };
  let pid = field("pid")?
    .as_u64()
    .and_then(|p| u32::try_from(p).ok())
    .ok_or_else(|| unreadable("`pid` is not a process id".to_string()))?;
  Ok(ClaudeSession {
    path: path.to_path_buf(),
    pid,
    proc_start: string("procStart")?,
    session_id: string("sessionId")?,
    cwd: PathBuf::from(string("cwd")?),
    name: string("name")?,
    status: string("status")?,
    started_at: stamp("startedAt")?,
    updated_at: stamp("updatedAt")?,
  })
}

/// `ps -o pid=,lstart=` output as each pid's start time: macOS's form.
///
/// **MEASURED ON BOTH OSES, AND THE FORMS DIFFER** (2026-10-24, Claude Code
/// 2.1.310). On macOS `procStart` is UTC ctime and equalled `ps -o lstart=`
/// under `LC_ALL=C` and `TZ=UTC`, spaces squeezed, for every session on the
/// host; under local time or another locale it never does. On Linux it is the
/// kernel's start time in clock ticks since boot, read by [`parse_proc_stat`].
pub fn parse_ps(out: &str) -> BTreeMap<u32, String> {
  out
    .lines()
    .filter_map(|line| {
      let line = line.trim();
      let (pid, start) = line.split_once(char::is_whitespace)?;
      Some((pid.parse().ok()?, squeeze(start)))
    })
    .collect()
}

/// Field 22 of a `/proc/<pid>/stat` line, the start time in clock ticks since
/// boot: Linux's form of `procStart`.
///
/// **READ FROM AFTER THE LAST `)`**, because field 2, the command name, sits in
/// parentheses and may itself carry spaces and parentheses. Field 3 is the
/// first after it, so field 22 is the twentieth.
pub fn parse_proc_stat(line: &str) -> Option<String> {
  let (_, after) = line.rsplit_once(')')?;
  after.split_whitespace().nth(22 - 3).map(str::to_string)
}

/// This OS's form of `procStart` for one pid, or `None` when it is gone. One
/// owner of the form, so a test that builds a session file for a live pid
/// writes exactly what the reader compares.
pub fn proc_start_of(pid: u32) -> Result<Option<String>, EstateError> {
  read_liveness(&[pid])
    .map(|mut starts| starts.remove(&pid))
    .map_err(|u| EstateError::NoLiveness { why: u.why })
}

/// Each pid's start time, in this OS's form. On Linux one `/proc` read per pid,
/// an absent entry being a gone pid.
#[cfg(target_os = "linux")]
fn read_liveness(pids: &[u32]) -> Result<BTreeMap<u32, String>, Unreadable> {
  let mut starts = BTreeMap::new();
  for pid in pids {
    let path = format!("/proc/{pid}/stat");
    match std::fs::read_to_string(&path) {
      Ok(line) => {
        let start = parse_proc_stat(&line)
          .ok_or_else(|| liveness_unreadable(format!("`{path}` has no start time")))?;
        starts.insert(*pid, start);
      }
      Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
      Err(e) => {
        return Err(liveness_unreadable(format!(
          "`{path}` could not be read: {e}"
        )));
      }
    }
  }
  Ok(starts)
}

/// Each pid's start time, in this OS's form. Elsewhere one `ps` call however
/// many pids there are.
#[cfg(not(target_os = "linux"))]
fn read_liveness(pids: &[u32]) -> Result<BTreeMap<u32, String>, Unreadable> {
  if pids.is_empty() {
    return Ok(BTreeMap::new());
  }
  let pids: Vec<String> = pids.iter().map(u32::to_string).collect();
  let out = Command::new(PS)
    .args(["-o", "pid=,lstart=", "-p", &pids.join(",")])
    .env("LC_ALL", "C")
    .env("TZ", "UTC")
    .output()
    .map_err(|e| liveness_unreadable(format!("`{PS}` could not run: {e}")))?;
  // `ps` exits 1 with nothing on stderr when no listed pid is alive, which is
  // an answer; anything on stderr is not.
  let stderr = String::from_utf8_lossy(&out.stderr);
  if !out.status.success() && !stderr.trim().is_empty() {
    return Err(liveness_unreadable(format!(
      "`{PS}` failed: {}",
      stderr.trim()
    )));
  }
  Ok(parse_ps(&String::from_utf8_lossy(&out.stdout)))
}

/// What a liveness failure is listed against.
#[cfg(target_os = "linux")]
const LIVENESS_SOURCE: &str = "/proc";
#[cfg(not(target_os = "linux"))]
const LIVENESS_SOURCE: &str = PS;

/// A liveness read that could not answer, listed against its source.
fn liveness_unreadable(why: String) -> Unreadable {
  Unreadable {
    path: LIVENESS_SOURCE.to_string(),
    why,
    pid: None,
  }
}

/// Whether the pid is alive and still the process Claude Code recorded.
pub fn alive(session: &ClaudeSession, starts: &BTreeMap<u32, String>) -> bool {
  starts.get(&session.pid) == Some(&squeeze(&session.proc_start))
}

/// One board's node header, under a schema this build reads.
pub fn parse_board_header(path: &Path, text: &str) -> Result<BoardHeader, Unreadable> {
  let unreadable = |why: String| Unreadable {
    path: path.display().to_string(),
    why,
    pid: None,
  };
  let doc: Value = serde_json::from_str(text).map_err(|e| unreadable(format!("not JSON: {e}")))?;
  let schema = doc
    .get("schema")
    .and_then(Value::as_str)
    .unwrap_or("(none)");
  if !BOARD_SCHEMAS.contains(&schema) {
    return Err(unreadable(format!(
      "board schema `{schema}` is not one this build reads ({})",
      BOARD_SCHEMAS.join(", ")
    )));
  }
  let node = doc
    .get("node")
    .cloned()
    .ok_or_else(|| unreadable("missing `node`".to_string()))?;
  serde_json::from_value(node).map_err(|e| unreadable(format!("`node`: {e}")))
}

/// The row's state, most decisive first.
///
/// **ANY STATUS BUT `idle` IS BUSY**, so a status Claude Code adds later still
/// renders, and the row keeps it verbatim beside this.
pub fn state_of(alive: bool, paused: bool, heavy: bool, claude_status: &str) -> State {
  if !alive {
    State::Gone
  } else if paused {
    State::Paused
  } else if heavy {
    State::Heavy
  } else if claude_status == "idle" {
    State::Idle
  } else {
    State::Busy
  }
}

/// The estate from what was gathered. Pure.
pub fn join(gathered: Gathered, now: OffsetDateTime) -> EstateStatus {
  let Gathered {
    host,
    sessions,
    liveness,
    projects,
    records,
    census,
    usage,
    mut unreadable,
  } = gathered;
  let starts = match liveness {
    Some(Ok(starts)) => Some(starts),
    Some(Err(u)) => {
      unreadable.push(u);
      None
    }
    None => None,
  };
  let mut rows: Vec<SessionRow> = sessions
    .into_iter()
    .map(|session| {
      // A liveness read that could not answer leaves liveness unknown, which renders
      // as alive: the failure is listed, and nobody is called gone on it.
      let is_alive = starts.as_ref().is_none_or(|s| alive(&session, s));
      let at = projects.get(&session.cwd).cloned().flatten();
      let board = at.as_ref().and_then(|p| {
        p.boards
          .iter()
          .find(|b| b.session_id.as_deref() == Some(session.session_id.as_str()))
      });
      let record = records.get(&session.session_id);
      let (line, line_is_wip, line_at) = match (board, record) {
        (Some(b), _) => (Some(b.focus.clone()), false, None),
        (None, Some(r)) => (Some(r.wip.clone()), true, r.wip_at.clone()),
        (None, None) => (None, false, None),
      };
      SessionRow {
        state: state_of(
          is_alive,
          board.is_some_and(|b| b.status == "paused"),
          false,
          &session.status,
        ),
        node: board.map(|b| b.moniker.clone()),
        claims: board.map(|b| b.claims.clone()).unwrap_or_default(),
        worktree: at.as_ref().is_some_and(|p| p.worktree),
        project: at.map(|p| ProjectRef {
          root: p.root,
          name: p.name,
        }),
        session_id: session.session_id,
        name: session.name,
        pid: session.pid,
        claude_status: session.status,
        started_at: session.started_at,
        updated_at: session.updated_at,
        slots: Vec::new(),
        line,
        line_is_wip,
        line_at,
      }
    })
    .collect();
  rows.sort_by(|a, b| {
    let key = |r: &SessionRow| {
      (
        r.project.is_none(),
        r.project.as_ref().map(|p| p.name.to_lowercase()),
      )
    };
    key(a).cmp(&key(b)).then_with(|| a.name.cmp(&b.name))
  });
  EstateStatus {
    schema: STATUS_SCHEMA.to_string(),
    host,
    version: crate::VERSION.to_string(),
    taken_at: iso(now).unwrap_or_default(),
    census,
    controller: Value::Null,
    lanes: json!([]),
    queue: json!([]),
    usage,
    sessions: rows,
    unreadable,
  }
}

/// The main checkout a git worktree at `root` belongs to, read from its `.git`
/// file and the git dir's `commondir`. Reads files only; runs no git.
pub fn main_checkout(root: &Path) -> Option<PathBuf> {
  let dotgit = std::fs::read_to_string(root.join(".git")).ok()?;
  let gitdir = root.join(dotgit.strip_prefix("gitdir:")?.trim());
  let common = std::fs::read_to_string(gitdir.join("commondir")).ok()?;
  let common = std::fs::canonicalize(gitdir.join(common.trim())).ok()?;
  common.parent().map(Path::to_path_buf)
}

/// Every board header under the project, and every board it could not read.
fn read_boards(root: &Path, unreadable: &mut Vec<Unreadable>) -> Vec<BoardHeader> {
  let Ok(entries) = std::fs::read_dir(root.join("intent").join("whiteboard")) else {
    return Vec::new();
  };
  let mut paths: Vec<PathBuf> = entries
    .filter_map(Result::ok)
    .map(|e| e.path().join("board.json"))
    .filter(|p| p.is_file())
    .collect();
  paths.sort();
  paths
    .into_iter()
    .filter_map(|path| {
      let read = std::fs::read_to_string(&path).map_err(|e| Unreadable {
        path: path.display().to_string(),
        why: e.to_string(),
        pid: None,
      });
      match read.and_then(|text| parse_board_header(&path, &text)) {
        Ok(header) => Some(header),
        Err(u) => {
          unreadable.push(u);
          None
        }
      }
    })
    .collect()
}

/// The project a cwd stands in, folded out of a worktree, with whether it
/// was one; `Err` when a project is there and could not be opened.
fn locate(cwd: &Path) -> Result<Option<(Project, bool)>, ProjectError> {
  let project = match Project::discover(cwd) {
    Ok(project) => project,
    Err(ProjectError::NotFound(_)) => return Ok(None),
    Err(e) => return Err(e),
  };
  Ok(Some(
    match main_checkout(project.root()).map(|main| Project::open(&main)) {
      Some(Ok(main)) => (main, true),
      _ => (project, false),
    },
  ))
}

fn name_of(project: &Project) -> String {
  project
    .directory_name()
    .unwrap_or_else(|| project.root().display().to_string())
}

/// The name of the project a cwd stands in, for the census. A project that
/// cannot be opened counts as none: the session rows list it unreadable.
fn project_name_at(cwd: &Path) -> Option<String> {
  locate(cwd).ok().flatten().map(|(p, _)| name_of(&p))
}

/// The project a cwd stands in, folded out of a worktree.
fn project_at(cwd: &Path, unreadable: &mut Vec<Unreadable>) -> Option<ProjectAt> {
  let (project, worktree) = match locate(cwd) {
    Ok(found) => found?,
    Err(e) => {
      unreadable.push(Unreadable {
        path: cwd.display().to_string(),
        why: e.to_string(),
        pid: None,
      });
      return None;
    }
  };
  let root = project.root().to_path_buf();
  Some(ProjectAt {
    name: name_of(&project),
    boards: read_boards(&root, unreadable),
    worktree,
    root,
  })
}

/// Every session file under `claude_dir`, and every one it could not read.
fn read_sessions(
  claude_dir: &Path,
  unreadable: &mut Vec<Unreadable>,
) -> Result<Vec<ClaudeSession>, EstateError> {
  let dir = sessions_dir(claude_dir);
  let entries = match std::fs::read_dir(&dir) {
    Ok(entries) => entries,
    Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
    Err(cause) => return Err(EstateError::Unlistable { path: dir, cause }),
  };
  let mut paths: Vec<PathBuf> = entries
    .filter_map(Result::ok)
    .map(|e| e.path())
    .filter(|p| p.extension().is_some_and(|x| x == "json"))
    .collect();
  paths.sort();
  Ok(
    paths
      .into_iter()
      .filter_map(|path| {
        let read = std::fs::read_to_string(&path).map_err(|e| Unreadable {
          path: path.display().to_string(),
          why: e.to_string(),
          pid: None,
        });
        match read.and_then(|text| parse_session(&path, &text)) {
          Ok(session) => Some(session),
          Err(u) => {
            unreadable.push(u);
            None
          }
        }
      })
      .collect(),
  )
}

fn pids_of(sessions: &[ClaudeSession]) -> Vec<u32> {
  sessions.iter().map(|s| s.pid).collect()
}

fn read_host() -> Result<String, Unreadable> {
  let unreadable = |why: String| Unreadable {
    path: UNAME.to_string(),
    why,
    pid: None,
  };
  let out = Command::new(UNAME)
    .arg("-n")
    .output()
    .map_err(|e| unreadable(format!("`{UNAME}` could not run: {e}")))?;
  let host = String::from_utf8_lossy(&out.stdout).trim().to_string();
  if out.status.success() && !host.is_empty() {
    Ok(host)
  } else {
    Err(unreadable(format!("`{UNAME} -n` named no host")))
  }
}

/// Intent's own records, keyed by session id, and every one it could not read.
fn read_records(dirs: &Dirs, unreadable: &mut Vec<Unreadable>) -> BTreeMap<String, WipRecord> {
  record_paths(dirs)
    .into_iter()
    .filter_map(|(session_id, path)| {
      let unread = |why: String| Unreadable {
        path: path.display().to_string(),
        why,
        pid: None,
      };
      let parsed = std::fs::read_to_string(&path)
        .map_err(|e| unread(e.to_string()))
        .and_then(|text| {
          serde_json::from_str::<WipRecord>(&text).map_err(|e| unread(format!("not a record: {e}")))
        })
        .and_then(|r| {
          if r.schema == RECORD_SCHEMA {
            Ok(WipRecord {
              wip_at: stamped_at(&path),
              ..r
            })
          } else {
            Err(unread(format!(
              "record schema `{}` is not `{RECORD_SCHEMA}`",
              r.schema
            )))
          }
        });
      match parsed {
        Ok(record) => Some((session_id, record)),
        Err(u) => {
          unreadable.push(u);
          None
        }
      }
    })
    .collect()
}

/// `<session-id>.json` under the estate directory, skipping a write's temp file.
fn record_paths(dirs: &Dirs) -> Vec<(String, PathBuf)> {
  let Ok(entries) = std::fs::read_dir(estate_dir(dirs)) else {
    return Vec::new();
  };
  let mut paths: Vec<(String, PathBuf)> = entries
    .filter_map(Result::ok)
    .map(|e| e.path())
    .filter_map(|path| {
      let id = path
        .file_name()?
        .to_str()?
        .strip_suffix(".json")?
        .to_string();
      valid_session_id(&id).then_some((id, path))
    })
    .collect();
  paths.sort();
  paths
}

fn valid_session_id(id: &str) -> bool {
  !id.is_empty() && id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
}

/// The estate on this host. Reads only: no store is opened and nothing is
/// written, so a reader never disturbs the sessions it reports.
pub fn status(dirs: &Dirs, claude_dir: &Path) -> Result<EstateStatus, EstateError> {
  status_with(dirs, claude_dir, Held::default())
}

/// What only a long-lived reader holds, and so only intentd hands over: the
/// census ring and the last endpoint reading, both taken on its census tick.
#[derive(Debug, Clone, Default)]
pub struct Held {
  pub history: Vec<census::Sample>,
  pub endpoint: Option<usage::Endpoint>,
}

/// [`status`] with what a long-lived caller held.
pub fn status_with(
  dirs: &Dirs,
  claude_dir: &Path,
  held: Held,
) -> Result<EstateStatus, EstateError> {
  let mut unreadable = Vec::new();
  let host = read_host().map_err(|u| unreadable.push(u)).ok();
  let sessions = read_sessions(claude_dir, &mut unreadable)?;
  let liveness = Some(read_liveness(&pids_of(&sessions)));
  let cwds: BTreeSet<PathBuf> = sessions.iter().map(|s| s.cwd.clone()).collect();
  let projects = cwds
    .into_iter()
    .map(|cwd| {
      let at = project_at(&cwd, &mut unreadable);
      (cwd, at)
    })
    .collect();
  let records = read_records(dirs, &mut unreadable);
  let census = census::read(dirs, held.history, &mut unreadable);
  let usage = usage::resolve(held.endpoint.as_ref(), dirs, &mut unreadable);
  Ok(join(
    Gathered {
      host,
      sessions,
      liveness,
      projects,
      records,
      census,
      usage,
      unreadable,
    },
    clock::now(),
  ))
}

/// [`status`] over the operator's own directories: the one place the CLI's
/// facade and intentd's [`crate::wire::Op::EstateStatus`] both resolve them,
/// so the two faces cannot read two different estates.
pub fn host_status() -> Result<EstateStatus, EstateError> {
  host_status_with(Held::default())
}

/// [`host_status`] with what intentd held.
pub fn host_status_with(held: Held) -> Result<EstateStatus, EstateError> {
  let dirs = crate::userstate::dirs()?;
  let claude = crate::userstate::claude_dir()?;
  status_with(&dirs, &claude, held)
}

/// [`prune`] over the operator's own directories, for intentd's census tick
/// (vc decision 73: a gone session's WIP record is reaped there).
pub fn host_prune() -> Result<Pruned, EstateError> {
  let dirs = crate::userstate::dirs()?;
  let claude = crate::userstate::claude_dir()?;
  prune(&dirs, &claude)
}

/// [`wip`] under the operator's own directories, for the CLI's verb and the
/// facade, so neither resolves them.
pub fn host_wip(session_id: &str, line: Option<&str>) -> Result<WipWritten, EstateError> {
  wip(&crate::userstate::dirs()?, session_id, line)
}

/// [`census::watch`] under the operator's own directories.
pub fn host_census_watch(class: &str, pattern: &str) -> Result<PathBuf, EstateError> {
  census::watch(&crate::userstate::dirs()?, class, pattern)
}

/// [`census::unwatch`] under the operator's own directories.
pub fn host_census_unwatch(class: &str) -> Result<bool, EstateError> {
  census::unwatch(&crate::userstate::dirs()?, class)
}

/// [`usage::report`] under the operator's own directories.
pub fn host_statusline_report(text: &str) -> Result<Option<PathBuf>, EstateError> {
  usage::report(&crate::userstate::dirs()?, text)
}

/// One endpoint reading over the operator's Claude directory, for intentd's
/// census tick.
pub fn host_endpoint() -> Result<usage::Endpoint, EstateError> {
  Ok(usage::read_endpoint(
    &crate::userstate::claude_dir()?,
    crate::userstate::claude_dir_configured(),
  ))
}

/// Writes this session's WIP line, or with `None` removes its record.
///
/// Written whole by atomic rename, by the session it names alone.
pub fn wip(dirs: &Dirs, session_id: &str, line: Option<&str>) -> Result<WipWritten, EstateError> {
  if !valid_session_id(session_id) {
    return Err(EstateError::BadSessionId {
      session_id: session_id.to_string(),
    });
  }
  let path = estate_dir(dirs).join(format!("{session_id}.json"));
  let Some(line) = line else {
    return match std::fs::remove_file(&path) {
      Ok(()) => Ok(WipWritten {
        path,
        cleared: true,
      }),
      Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(WipWritten {
        path,
        cleared: true,
      }),
      Err(cause) => Err(EstateError::Unremovable { path, cause }),
    };
  };
  if line.trim().is_empty() {
    return Err(EstateError::BadLine {
      why: "is empty".to_string(),
    });
  }
  if line.contains(['\n', '\r']) {
    return Err(EstateError::BadLine {
      why: "spans more than one line".to_string(),
    });
  }
  let record = WipRecord {
    schema: RECORD_SCHEMA.to_string(),
    session_id: session_id.to_string(),
    wip: line.to_string(),
    wip_at: None,
  };
  let mut text = serde_json::to_string_pretty(&record).map_err(|e| EstateError::Unwritable {
    path: path.clone(),
    cause: WriteError::Io {
      path: path.display().to_string(),
      source: std::io::Error::other(e),
    },
  })?;
  text.push('\n');
  let written = estate_dir(dirs);
  std::fs::create_dir_all(&written)
    .map_err(|e| WriteError::Io {
      path: written.display().to_string(),
      source: e,
    })
    .and_then(|()| write_atomically(&path, text.as_bytes()))
    .map_err(|cause| EstateError::Unwritable {
      path: path.clone(),
      cause,
    })?;
  Ok(WipWritten {
    path,
    cleared: false,
  })
}

/// The session ids whose records must stay: every live session's, and every
/// session whose file could not be read but names an id, since nobody can say
/// it is gone.
pub fn kept_ids(
  sessions: &[ClaudeSession],
  starts: &BTreeMap<u32, String>,
  unparsed: &[String],
) -> BTreeSet<String> {
  sessions
    .iter()
    .filter(|s| alive(s, starts))
    .map(|s| s.session_id.clone())
    .chain(unparsed.iter().cloned())
    .collect()
}

/// Removes Intent's own records for sessions that are gone. Claude Code's
/// files are never touched.
pub fn prune(dirs: &Dirs, claude_dir: &Path) -> Result<Pruned, EstateError> {
  let mut unreadable = Vec::new();
  let sessions = read_sessions(claude_dir, &mut unreadable)?;
  let starts =
    read_liveness(&pids_of(&sessions)).map_err(|u| EstateError::NoLiveness { why: u.why })?;
  let unparsed: Vec<String> = unreadable
    .iter()
    .filter_map(|u| std::fs::read_to_string(&u.path).ok())
    .filter_map(|text| serde_json::from_str::<Value>(&text).ok())
    .filter_map(|doc| {
      doc
        .get("sessionId")
        .and_then(Value::as_str)
        .map(str::to_string)
    })
    .collect();
  let kept = kept_ids(&sessions, &starts, &unparsed);
  let mut removed = Vec::new();
  for (session_id, path) in record_paths(dirs) {
    if kept.contains(&session_id) {
      continue;
    }
    std::fs::remove_file(&path).map_err(|cause| EstateError::Unremovable {
      path: path.clone(),
      cause,
    })?;
    removed.push(path);
  }
  Ok(Pruned { removed })
}

#[cfg(test)]
mod tests {
  use super::*;

  fn session(pid: u32, id: &str, cwd: &str, status: &str) -> ClaudeSession {
    ClaudeSession {
      path: PathBuf::from(format!("/c/sessions/{pid}.json")),
      pid,
      proc_start: "Sat Oct 24 05:28:00 2026".to_string(),
      session_id: id.to_string(),
      cwd: PathBuf::from(cwd),
      name: format!("s-{pid}"),
      status: status.to_string(),
      started_at: "2026-10-24T05:28:00.000Z".to_string(),
      updated_at: "2026-10-24T13:00:00.000Z".to_string(),
    }
  }

  fn board(moniker: &str, id: &str, status: &str) -> BoardHeader {
    BoardHeader {
      moniker: moniker.to_string(),
      session_id: Some(id.to_string()),
      status: status.to_string(),
      focus: format!("{moniker}'s focus"),
      claims: vec!["ST0086/01".to_string()],
    }
  }

  fn project(root: &str, boards: Vec<BoardHeader>) -> Option<ProjectAt> {
    Some(ProjectAt {
      root: PathBuf::from(root),
      name: Path::new(root)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned(),
      worktree: false,
      boards,
    })
  }

  fn starts(pids: &[u32]) -> Option<Result<BTreeMap<u32, String>, Unreadable>> {
    Some(Ok(
      pids
        .iter()
        .map(|p| (*p, "Sat Oct 24 05:28:00 2026".to_string()))
        .collect(),
    ))
  }

  fn now() -> OffsetDateTime {
    time::macros::datetime!(2026-10-24 14:00:00 UTC)
  }

  fn row<'a>(status: &'a EstateStatus, id: &str) -> &'a SessionRow {
    status
      .sessions
      .iter()
      .find(|r| r.session_id == id)
      .expect("row")
  }

  #[test]
  fn a_session_file_missing_a_pinned_field_is_unreadable_and_names_it() {
    let text = r#"{"pid": 7, "sessionId": "a", "cwd": "/p", "name": "n", "status": "idle", "startedAt": 1, "updatedAt": 2}"#;
    let err = parse_session(Path::new("/c/sessions/7.json"), text).unwrap_err();
    assert_eq!(err.why, "missing `procStart`");
    assert_eq!(err.pid, Some(7));
  }

  #[test]
  fn epoch_milliseconds_become_iso_utc_stamps() {
    assert_eq!(
      iso_ms(1792850200406).as_deref(),
      Some("2026-10-24T13:56:40.406Z")
    );
  }

  #[test]
  fn ps_output_is_read_by_pid_with_spaces_squeezed() {
    let parsed = parse_ps("89875 Sat Oct 24 05:27:40 2026    \n 9208 Sat Oct  4 01:02:03 2026\n");
    assert_eq!(
      parsed.get(&89875).map(String::as_str),
      Some("Sat Oct 24 05:27:40 2026")
    );
    assert_eq!(
      parsed.get(&9208).map(String::as_str),
      Some("Sat Oct 4 01:02:03 2026")
    );
  }

  #[test]
  fn the_proc_stat_start_time_is_read_after_the_last_paren() {
    let line = "692155 (claude (x) y) S 1 692155 692155 0 -1 4194560 1 2 3 4 5 6 7 8 20 0 12 0 12448223 1000 99";
    assert_eq!(parse_proc_stat(line).as_deref(), Some("12448223"));
    assert_eq!(parse_proc_stat("no paren here"), None);
  }

  #[test]
  fn this_process_reads_as_alive_through_its_own_start_time() {
    let pid = std::process::id();
    let start = proc_start_of(pid)
      .unwrap()
      .expect("this process is running");
    let mut me = session(pid, "me", "/x", "busy");
    me.proc_start = start.clone();
    assert!(alive(&me, &BTreeMap::from([(pid, start)])));
  }

  #[test]
  fn a_reused_pid_is_gone() {
    let s = session(7, "a", "/p", "busy");
    let other_start = BTreeMap::from([(7, "Fri Oct 23 01:00:00 2026".to_string())]);
    assert!(!alive(&s, &other_start));
  }

  #[test]
  fn the_state_order_is_gone_paused_heavy_busy_idle() {
    assert_eq!(state_of(false, true, true, "busy"), State::Gone);
    assert_eq!(state_of(true, true, true, "busy"), State::Paused);
    assert_eq!(state_of(true, false, true, "idle"), State::Heavy);
    assert_eq!(state_of(true, false, false, "idle"), State::Idle);
    assert_eq!(state_of(true, false, false, "shell"), State::Busy);
  }

  #[test]
  fn a_board_whose_session_id_matches_supplies_node_focus_and_claims() {
    let status = join(
      Gathered {
        sessions: vec![session(7, "a", "/p/Intent", "idle")],
        liveness: starts(&[7]),
        projects: BTreeMap::from([(
          PathBuf::from("/p/Intent"),
          project("/p/Intent", vec![board("cc", "a", "active")]),
        )]),
        ..Gathered::default()
      },
      now(),
    );
    let r = row(&status, "a");
    assert_eq!(r.node.as_deref(), Some("cc"));
    assert_eq!(r.line.as_deref(), Some("cc's focus"));
    assert!(!r.line_is_wip);
    assert_eq!(r.claims, vec!["ST0086/01".to_string()]);
    assert_eq!(r.state, State::Idle);
  }

  #[test]
  fn a_paused_board_renders_paused() {
    let status = join(
      Gathered {
        sessions: vec![session(7, "a", "/p/Intent", "busy")],
        liveness: starts(&[7]),
        projects: BTreeMap::from([(
          PathBuf::from("/p/Intent"),
          project("/p/Intent", vec![board("cc", "a", "paused")]),
        )]),
        ..Gathered::default()
      },
      now(),
    );
    assert_eq!(row(&status, "a").state, State::Paused);
  }

  #[test]
  fn an_unknown_claude_status_is_busy_and_kept_verbatim() {
    let status = join(
      Gathered {
        sessions: vec![session(7, "a", "/x", "shell")],
        liveness: starts(&[7]),
        ..Gathered::default()
      },
      now(),
    );
    let r = row(&status, "a");
    assert_eq!(r.state, State::Busy);
    assert_eq!(r.claude_status, "shell");
  }

  #[test]
  fn a_boardless_session_shows_its_wip_record() {
    let record = WipRecord {
      schema: RECORD_SCHEMA.to_string(),
      session_id: "a".to_string(),
      wip: "sorting the estate".to_string(),
      wip_at: Some("2026-10-24T13:00:00.000Z".to_string()),
    };
    let status = join(
      Gathered {
        sessions: vec![session(7, "a", "/x", "idle")],
        liveness: starts(&[7]),
        records: BTreeMap::from([("a".to_string(), record)]),
        ..Gathered::default()
      },
      now(),
    );
    let r = row(&status, "a");
    assert_eq!(r.line.as_deref(), Some("sorting the estate"));
    assert!(r.line_is_wip);
    assert_eq!(r.line_at.as_deref(), Some("2026-10-24T13:00:00.000Z"));
    assert!(r.project.is_none());
  }

  #[test]
  fn a_failed_ps_is_unreadable_and_calls_nobody_gone() {
    let status = join(
      Gathered {
        sessions: vec![session(7, "a", "/x", "idle")],
        liveness: Some(Err(liveness_unreadable(
          "`/bin/ps` could not run".to_string(),
        ))),
        ..Gathered::default()
      },
      now(),
    );
    assert_eq!(row(&status, "a").state, State::Idle);
    assert_eq!(status.unreadable[0].path, LIVENESS_SOURCE);
  }

  #[test]
  fn a_dead_pid_is_gone() {
    let status = join(
      Gathered {
        sessions: vec![session(7, "a", "/x", "busy")],
        liveness: starts(&[]),
        ..Gathered::default()
      },
      now(),
    );
    assert_eq!(row(&status, "a").state, State::Gone);
  }

  #[test]
  fn rows_sort_by_project_then_name_with_no_project_last() {
    let status = join(
      Gathered {
        sessions: vec![
          session(3, "c", "/x", "idle"),
          session(2, "b", "/p/Laksa", "idle"),
          session(1, "a", "/p/Devbin", "idle"),
        ],
        liveness: starts(&[1, 2, 3]),
        projects: BTreeMap::from([
          (PathBuf::from("/p/Laksa"), project("/p/Laksa", vec![])),
          (PathBuf::from("/p/Devbin"), project("/p/Devbin", vec![])),
          (PathBuf::from("/x"), None),
        ]),
        ..Gathered::default()
      },
      now(),
    );
    let order: Vec<&str> = status
      .sessions
      .iter()
      .map(|r| r.session_id.as_str())
      .collect();
    assert_eq!(order, vec!["a", "b", "c"]);
  }

  #[test]
  fn every_boundary_key_is_present_from_wp_01() {
    let value = serde_json::to_value(join(Gathered::default(), now())).unwrap();
    for key in [
      "schema",
      "host",
      "version",
      "taken_at",
      "census",
      "controller",
      "lanes",
      "queue",
      "usage",
      "sessions",
      "unreadable",
    ] {
      assert!(value.get(key).is_some(), "missing `{key}`");
    }
    assert_eq!(value["schema"], STATUS_SCHEMA);
    assert_eq!(value["taken_at"], "2026-10-24T14:00:00.000Z");
    assert_eq!(
      value["census"],
      json!({ "load": [], "procs": [], "history": [] })
    );
    assert_eq!(
      value["usage"],
      json!({
        "source": null, "error": null, "read_at": null,
        "five_hour": null, "seven_day": null, "models": null
      })
    );
  }

  #[test]
  fn a_board_in_schema_3_0_is_read_and_another_schema_is_unreadable() {
    let node = r#""node": {"moniker": "vc", "name": "V", "role": "validation", "session_id": "a", "status": "active", "focus": "f", "claims": [], "heartbeat_at": "x"}"#;
    let old = format!(r#"{{"schema": "intent/board@3.0", "items": [], "messages": [], {node}}}"#);
    assert_eq!(
      parse_board_header(Path::new("b.json"), &old)
        .unwrap()
        .moniker,
      "vc"
    );
    let new = format!(r#"{{"schema": "intent/board@5.0", {node}}}"#);
    let err = parse_board_header(Path::new("b.json"), &new).unwrap_err();
    assert!(err.why.contains("`intent/board@5.0`"), "{}", err.why);
  }

  #[test]
  fn a_worktree_folds_to_its_main_checkout() {
    let tmp = tempfile::tempdir().unwrap();
    let main = tmp.path().join("Intent");
    let gitdir = main.join(".git/worktrees/wt-x");
    let wt = main.join("tmp/wt-x");
    std::fs::create_dir_all(&gitdir).unwrap();
    std::fs::create_dir_all(&wt).unwrap();
    std::fs::write(gitdir.join("commondir"), "../..\n").unwrap();
    std::fs::write(wt.join(".git"), format!("gitdir: {}\n", gitdir.display())).unwrap();
    assert_eq!(
      main_checkout(&wt),
      Some(std::fs::canonicalize(&main).unwrap())
    );
    assert_eq!(main_checkout(&main), None);
  }

  #[test]
  fn wip_writes_a_whole_record_and_clear_removes_it() {
    let tmp = tempfile::tempdir().unwrap();
    let dirs = Dirs::at_home(tmp.path());
    let written = wip(&dirs, "abc-1", Some("measuring")).unwrap();
    let record: WipRecord =
      serde_json::from_str(&std::fs::read_to_string(&written.path).unwrap()).unwrap();
    assert_eq!(record.schema, RECORD_SCHEMA);
    assert_eq!(record.wip, "measuring");
    let text = std::fs::read_to_string(&written.path).unwrap();
    assert!(
      !text.contains("wip_at"),
      "the stamp is the mtime, never a field: {text}"
    );
    let mut unreadable = Vec::new();
    let read = read_records(&dirs, &mut unreadable);
    assert_eq!(read["abc-1"].wip_at, stamped_at(&written.path));
    assert!(read["abc-1"].wip_at.is_some());
    assert!(wip(&dirs, "abc-1", None).unwrap().cleared);
    assert!(!written.path.exists());
  }

  #[test]
  fn wip_refuses_a_path_as_a_session_id_and_a_second_line() {
    let tmp = tempfile::tempdir().unwrap();
    let dirs = Dirs::at_home(tmp.path());
    assert!(matches!(
      wip(&dirs, "../x", Some("l")),
      Err(EstateError::BadSessionId { .. })
    ));
    assert!(matches!(
      wip(&dirs, "a", Some("one\ntwo")),
      Err(EstateError::BadLine { .. })
    ));
  }

  #[test]
  fn prune_keeps_live_and_unparsed_sessions_records() {
    let live = session(7, "live", "/x", "idle");
    let gone = session(8, "gone", "/x", "idle");
    let kept = kept_ids(
      &[live, gone],
      &BTreeMap::from([(7, squeeze("Sat Oct 24 05:28:00 2026"))]),
      &["unparsed".to_string()],
    );
    assert_eq!(
      kept,
      BTreeSet::from(["live".to_string(), "unparsed".to_string()])
    );
  }

  #[test]
  fn prune_removes_only_records_and_never_a_claude_code_file() {
    let tmp = tempfile::tempdir().unwrap();
    let dirs = Dirs::at_home(tmp.path());
    let claude = tmp.path().join(".claude");
    std::fs::create_dir_all(sessions_dir(&claude)).unwrap();
    let theirs = sessions_dir(&claude).join("999999.json");
    std::fs::write(&theirs, "{}").unwrap();
    let mine = wip(&dirs, "nobody", Some("left behind")).unwrap().path;
    let pruned = prune(&dirs, &claude).unwrap();
    assert_eq!(pruned.removed, vec![mine.clone()]);
    assert!(!mine.exists());
    assert!(theirs.exists());
  }
}
