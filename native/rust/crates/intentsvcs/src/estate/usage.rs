//! Plan usage for the estate (ST0086 WP-02), normalised into the @1 `usage`
//! shape so the page derives nothing.
//!
//! **THE FALLBACK IS HERE FIRST.** The JSON Claude Code hands a session's
//! statusline command carries an optional `rate_limits` (measured on 2.1.310):
//! `five_hour` and `seven_day`, each `{used_percentage, resets_at}`, the reset
//! in epoch seconds. It is stdin to that command and no file, so a statusline
//! step hands it to [`report`], which keeps the session's latest in
//! `<state>/estate/<session-id>.usage.json`. That is its own file, so the WIP
//! line's writer and this one never share one, and its stamp is its mtime
//! (D42). [`fallback`] reads the newest.
//!
//! **THE PRIMARY IS THE ACCOUNT'S USAGE ENDPOINT, READ BY intentd ALONE**
//! (vc decision 73, 2026-10-24): on a census tick, never per page read, by
//! [`read_endpoint`], which takes Claude Code's OAuth token from the Keychain
//! through `/usr/bin/security` and sends it with `/usr/bin/curl`, the URL,
//! the headers and the token in a config on curl's stdin (`-K -`), so no argv
//! ever carries the token. The token is never written, logged or sent
//! anywhere else. [`resolve`] answers with the endpoint's reading when it has
//! one, else the fallback with the endpoint's failure named.
//!
//! **THE ENDPOINT IS SHARED, SO IT IS READ SPARINGLY AND A REFUSAL KEEPS WHAT
//! WAS READ** (issue 0658). Every reader of the account's token asks the same
//! endpoint, and it answered 429 while the census asked every minute, which
//! blanked the page. So [`next_wait`] spaces the reads [`READ_EVERY`] apart
//! and backs off after a failure, never sooner than a 429's `Retry-After`,
//! and [`carry`] keeps the last good reading, still stamped when it was read,
//! beside the failure that followed it.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{EstateError, Unreadable, estate_dir, iso, stamped_at, valid_session_id};
use crate::userstate::Dirs;
use crate::write_set::{WriteError, write_atomically};

/// The schema of one session's kept `rate_limits`.
pub const REPORT_SCHEMA: &str = "intent/estate-usage@1";

/// What `usage.error` says when this reading was not taken by intentd, the
/// only reader of the endpoint.
pub const NO_ENDPOINT: &str = "the account's usage endpoint is read by intentd's census tick, and this reading was not taken through it";

/// The account's usage endpoint, as Claude Code's menubar readers call it.
pub const ENDPOINT: &str = "https://api.anthropic.com/api/oauth/usage";

const SECURITY: &str = "/usr/bin/security";
const CURL: &str = "/usr/bin/curl";

/// The Keychain item Claude Code keeps its credentials in.
const SERVICE: &str = "Claude Code-credentials";

/// How long curl may take, in whole seconds.
const MAX_TIME: &str = "10";

/// The census reads the endpoint at most this often while the estate is
/// asked for: the five-hour and seven-day windows move slowly, and the
/// endpoint is shared with every other reader of the account's token.
pub const READ_EVERY: Duration = Duration::from_secs(5 * 60);

/// The longest wait a run of failures backs the next read off to.
pub const WAIT_CAP: Duration = Duration::from_secs(60 * 60);

/// Why the endpoint gave no reading, carried into `usage.error`.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum EndpointFailure {
  #[error("`{curl}` could not run: {why}")]
  NoCurl { curl: String, why: String },
  #[error("the Keychain read failed: {why}")]
  Keychain { why: String },
  #[error("curl reached no answer: {why}")]
  Unreached { why: String },
  #[error("the endpoint answered {status}")]
  Status { status: u16 },
  /// A 429, with the `Retry-After` it sent in whole seconds, if it sent one,
  /// which [`next_wait`] waits at least.
  #[error("the endpoint answered 429, too many requests")]
  Throttled { retry_after: Option<u64> },
  #[error("the endpoint's answer would not parse: {why}")]
  Unparsable { why: String },
}

/// One reading of the endpoint: its usage, stamped when it was read, or why
/// it gave none.
pub type Endpoint = Result<Usage, EndpointFailure>;

/// What a reader holds of the endpoint (issue 0659). Only intentd reads it,
/// and intentd holds nothing until its census tick first does, which is a
/// reading not taken yet and no failure.
#[derive(Debug, Clone, Default)]
pub enum Reading {
  /// A reader that is not intentd, so never reads the endpoint.
  #[default]
  NotIntentd,
  /// intentd before its census tick first reads the endpoint.
  NotReadYet,
  /// intentd's last reading, as [`carry`] keeps it.
  Read(Endpoint),
}

/// What [`read_endpoint`] runs and asks: the Keychain tool, curl, and the URL.
///
/// **THE HOST'S ARE [`EndpointTools::host`], AND NOTHING ELSE CALLS THE REAL
/// ONES.** A test hands stubs in a temporary directory, so no arm reads the
/// operator's Keychain or reaches the account's endpoint (ST0086 AC-02.6).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndpointTools {
  pub security: PathBuf,
  pub curl: PathBuf,
  pub url: String,
}

impl EndpointTools {
  /// `/usr/bin/security`, `/usr/bin/curl` and [`ENDPOINT`].
  pub fn host() -> Self {
    Self {
      security: PathBuf::from(SECURITY),
      curl: PathBuf::from(CURL),
      url: ENDPOINT.to_string(),
    }
  }
}

/// Where a reading came from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Source {
  /// `endpoint` or `statusline`.
  pub kind: String,
  /// The reporting session's, for the statusline; null for the endpoint.
  pub session_id: Option<String>,
}

/// One window's reading.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Window {
  pub percent: f64,
  pub resets_at: Option<String>,
}

/// One model's weekly cap, from the endpoint only.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ModelCap {
  pub name: String,
  pub percent: f64,
  pub resets_at: Option<String>,
}

/// The @1 `usage` key.
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub struct Usage {
  pub source: Option<Source>,
  /// Why the primary failed, or null when it did not.
  pub error: Option<String>,
  pub read_at: Option<String>,
  pub five_hour: Option<Window>,
  pub seven_day: Option<Window>,
  pub models: Option<Vec<ModelCap>>,
}

/// One session's kept `rate_limits`, as its file holds them.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Report {
  pub schema: String,
  pub session_id: String,
  pub rate_limits: Value,
}

fn report_path(dirs: &Dirs, session_id: &str) -> PathBuf {
  estate_dir(dirs).join(format!("{session_id}.usage.json"))
}

/// Keeps the `rate_limits` of the statusline input `text`, for the session it
/// names. An input with none, which is every one before the session's first
/// API answer, writes nothing and answers `None`.
pub fn report(dirs: &Dirs, text: &str) -> Result<Option<PathBuf>, EstateError> {
  let input: Value = serde_json::from_str(text).map_err(|e| EstateError::BadStatusline {
    why: format!("is not JSON: {e}"),
  })?;
  let session_id = input
    .get("session_id")
    .and_then(Value::as_str)
    .ok_or_else(|| EstateError::BadStatusline {
      why: "names no `session_id`".to_string(),
    })?;
  if !valid_session_id(session_id) {
    return Err(EstateError::BadSessionId {
      session_id: session_id.to_string(),
    });
  }
  let Some(rate_limits) = input.get("rate_limits").filter(|v| !v.is_null()) else {
    return Ok(None);
  };
  let path = report_path(dirs, session_id);
  let unwritable = |cause: WriteError| EstateError::Unwritable {
    path: path.clone(),
    cause,
  };
  let report = Report {
    schema: REPORT_SCHEMA.to_string(),
    session_id: session_id.to_string(),
    rate_limits: rate_limits.clone(),
  };
  let mut body = serde_json::to_string_pretty(&report).map_err(|e| {
    unwritable(WriteError::Io {
      path: path.display().to_string(),
      source: std::io::Error::other(e),
    })
  })?;
  body.push('\n');
  let dir = estate_dir(dirs);
  std::fs::create_dir_all(&dir)
    .map_err(|e| WriteError::Io {
      path: dir.display().to_string(),
      source: e,
    })
    .and_then(|()| write_atomically(&path, body.as_bytes()))
    .map_err(unwritable)?;
  Ok(Some(path))
}

/// One statusline window, `{used_percentage, resets_at}`, or `None` when the
/// window is absent or carries no percentage.
fn window(limits: &Value, key: &str) -> Option<Window> {
  let w = limits.get(key)?;
  Some(Window {
    percent: w.get("used_percentage")?.as_f64()?,
    resets_at: w
      .get("resets_at")
      .and_then(Value::as_i64)
      .and_then(|s| time::OffsetDateTime::from_unix_timestamp(s).ok())
      .and_then(iso),
  })
}

/// The usage a kept report says, read at `read_at`. Pure.
pub fn from_report(report: &Report, read_at: Option<String>) -> Usage {
  Usage {
    source: Some(Source {
      kind: "statusline".to_string(),
      session_id: Some(report.session_id.clone()),
    }),
    error: Some(NO_ENDPOINT.to_string()),
    read_at,
    five_hour: window(&report.rate_limits, "five_hour"),
    seven_day: window(&report.rate_limits, "seven_day"),
    models: None,
  }
}

/// The usage the estate reports: the endpoint's reading when there is one,
/// else the fallback with why the endpoint gave none, or [`NO_ENDPOINT`] for
/// a reader that is not intentd. A reading kept beside a later failure (see
/// [`carry`]) stands until a statusline report is newer, and the report then
/// shows, naming the same failure. Before intentd's first read the fallback
/// stands with no error, because nothing failed (issue 0659).
pub fn resolve(endpoint: &Reading, dirs: &Dirs, unreadable: &mut Vec<Unreadable>) -> Usage {
  let why = match endpoint {
    Reading::NotReadYet => {
      return Usage {
        error: None,
        ..fallback(dirs, unreadable)
      };
    }
    Reading::Read(Ok(usage)) if usage.error.is_none() => return usage.clone(),
    Reading::Read(Ok(kept)) => {
      let newer = fallback(dirs, unreadable);
      // ISO stamps of one form sort as the times they name.
      return if newer.source.is_some() && newer.read_at > kept.read_at {
        Usage {
          error: kept.error.clone(),
          ..newer
        }
      } else {
        kept.clone()
      };
    }
    Reading::Read(Err(failure)) => failure.to_string(),
    Reading::NotIntentd => NO_ENDPOINT.to_string(),
  };
  Usage {
    error: Some(why),
    ..fallback(dirs, unreadable)
  }
}

/// What the census keeps after a reading: a good one as it is; a failure
/// after a good reading as that reading, still stamped when it was read, with
/// the failure named in `error`; a failure with nothing good behind it as the
/// failure. Pure.
pub fn carry(kept: Option<&Endpoint>, read: Endpoint) -> Endpoint {
  match (read, kept) {
    (Ok(usage), _) => Ok(usage),
    (Err(failure), Some(Ok(usage))) => Ok(Usage {
      error: Some(failure.to_string()),
      ..usage.clone()
    }),
    (Err(failure), _) => Err(failure),
  }
}

/// How long the census waits after a reading before it reads the endpoint
/// again: [`READ_EVERY`] after a good one; after a failure, twice the wait
/// that led to it, from [`READ_EVERY`] up to [`WAIT_CAP`], and never sooner
/// than a 429's `Retry-After`. Pure.
pub fn next_wait(read: &Endpoint, waited: Duration) -> Duration {
  let Err(failure) = read else {
    return READ_EVERY;
  };
  let doubled = waited.saturating_mul(2).clamp(READ_EVERY, WAIT_CAP);
  match failure {
    EndpointFailure::Throttled {
      retry_after: Some(secs),
    } => doubled.max(Duration::from_secs(*secs)),
    _ => doubled,
  }
}

/// The newest kept report, as usage; with none, no source and the primary's
/// failure named. Every file it could not read is listed.
pub fn fallback(dirs: &Dirs, unreadable: &mut Vec<Unreadable>) -> Usage {
  let newest = report_paths(dirs)
    .into_iter()
    .filter_map(|path| match read_report(&path) {
      Ok(report) => Some((stamped_at(&path), report)),
      Err(u) => {
        unreadable.push(u);
        None
      }
    })
    // ISO stamps of one form sort as the times they name.
    .max_by(|a, b| a.0.cmp(&b.0));
  match newest {
    Some((read_at, report)) => from_report(&report, read_at),
    None => Usage {
      error: Some(NO_ENDPOINT.to_string()),
      ..Usage::default()
    },
  }
}

/// The Keychain service Claude Code keeps this config dir's credentials
/// under, chosen by Claude Code's own condition: **WHETHER
/// `CLAUDE_CONFIG_DIR` IS SET**, not whether the dir is `~/.claude`. Set, the
/// name is suffixed with `-` and the first eight hex digits of the SHA-256 of
/// the variable's value; unset, it is the bare name (measured 2026-10-24,
/// both items present on one host). The two tests differ when the variable
/// names `~/.claude` explicitly, and a path test would then read no item and
/// report a Keychain failure for a healthy host.
///
/// **ONE SERVICE, NEVER A FALLBACK TO THE BARE NAME.** A dir whose own item
/// is missing has no credentials, and borrowing the default dir's would read
/// another account's usage, or a test's temporary dir the operator's.
pub fn service(claude_dir: &Path, configured: bool) -> String {
  use sha2::{Digest, Sha256};
  if !configured {
    return SERVICE.to_string();
  }
  let digest = Sha256::digest(claude_dir.to_string_lossy().as_bytes());
  let hex: String = digest.iter().take(4).map(|b| format!("{b:02x}")).collect();
  format!("{SERVICE}-{hex}")
}

/// The OAuth access token, read from the config dir's own Keychain item. The
/// value never leaves this process but on curl's stdin.
fn read_token(
  security: &Path,
  claude_dir: &Path,
  configured: bool,
) -> Result<String, EndpointFailure> {
  let service = service(claude_dir, configured);
  let out = Command::new(security)
    .args(["find-generic-password", "-s", &service, "-w"])
    .stderr(Stdio::null())
    .output()
    .map_err(|e| EndpointFailure::Keychain {
      why: format!("`{}` could not run: {e}", security.display()),
    })?;
  if !out.status.success() {
    return Err(EndpointFailure::Keychain {
      why: format!("no item under `{service}`"),
    });
  }
  let creds: Value =
    serde_json::from_slice(&out.stdout).map_err(|_| EndpointFailure::Keychain {
      why: format!("the `{service}` item is not JSON"),
    })?;
  creds
    .pointer("/claudeAiOauth/accessToken")
    .and_then(Value::as_str)
    .map(str::to_string)
    .ok_or_else(|| EndpointFailure::Keychain {
      why: format!("the `{service}` item has no `claudeAiOauth.accessToken`"),
    })
}

/// A curl config line's quoted value.
fn quoted(value: &str) -> String {
  format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}

/// GETs `url` with the bearer token, the config on curl's stdin. Answers the
/// status, the body and the `Retry-After` in whole seconds when the answer
/// sent one; any status, since the caller judges it.
fn fetch(
  curl: &Path,
  url: &str,
  token: &str,
) -> Result<(u16, String, Option<u64>), EndpointFailure> {
  let config = format!(
    "url = {}\nheader = {}\nheader = {}\n",
    quoted(url),
    quoted(&format!("Authorization: Bearer {token}")),
    quoted("anthropic-beta: oauth-2025-04-20"),
  );
  let mut child = Command::new(curl)
    .args([
      "-sS",
      "--fail-with-body",
      "--max-time",
      MAX_TIME,
      "-K",
      "-",
      "-w",
      "\n%{http_code} %header{retry-after}",
    ])
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .map_err(|e| EndpointFailure::NoCurl {
      curl: curl.display().to_string(),
      why: e.to_string(),
    })?;
  if let Some(mut stdin) = child.stdin.take() {
    stdin
      .write_all(config.as_bytes())
      .map_err(|e| EndpointFailure::Unreached {
        why: format!("its config could not be written: {e}"),
      })?;
  }
  let out = child
    .wait_with_output()
    .map_err(|e| EndpointFailure::Unreached { why: e.to_string() })?;
  let stdout = String::from_utf8_lossy(&out.stdout);
  // The last line is the status, then the `Retry-After`, which is empty when
  // the answer sent none and is kept only as whole seconds.
  let (body, trailer) = stdout.rsplit_once('\n').unwrap_or(("", &stdout));
  let mut fields = trailer.split_whitespace();
  match fields.next().and_then(|code| code.parse::<u16>().ok()) {
    Some(status) if status != 0 => Ok((
      status,
      body.to_string(),
      fields.next().and_then(|secs| secs.parse::<u64>().ok()),
    )),
    _ => Err(EndpointFailure::Unreached {
      why: String::from_utf8_lossy(&out.stderr).trim().to_string(),
    }),
  }
}

/// A row's `resets_at` in the estate's form: `+00:00` as `Z`, and the
/// fraction to milliseconds. Any other offset is kept as sent. It takes the
/// row, a record that holds the time, not the time (D42).
fn resets_at(row: &Value) -> Option<String> {
  let sent = row.get("resets_at")?.as_str()?;
  let Some(base) = sent.strip_suffix("+00:00") else {
    return Some(sent.to_string());
  };
  let (secs, frac) = base.split_once('.').unwrap_or((base, ""));
  let ms: String = frac.chars().chain(std::iter::repeat('0')).take(3).collect();
  Some(format!("{secs}.{ms}Z"))
}

/// One endpoint window, `{utilization, resets_at}`.
fn endpoint_window(answer: &Value, key: &str) -> Option<Window> {
  let w = answer.get(key)?;
  Some(Window {
    percent: w.get("utilization")?.as_f64()?,
    resets_at: resets_at(w),
  })
}

/// The usage an endpoint answer says. Pure. `models` are the `limits` rows
/// whose scope names a model.
pub fn from_endpoint(body: &str) -> Result<Usage, EndpointFailure> {
  let answer: Value =
    serde_json::from_str(body).map_err(|e| EndpointFailure::Unparsable { why: e.to_string() })?;
  if !answer.is_object() {
    return Err(EndpointFailure::Unparsable {
      why: "it is not an object".to_string(),
    });
  }
  let models = answer
    .get("limits")
    .and_then(Value::as_array)
    .map(|limits| {
      limits
        .iter()
        .filter_map(|l| {
          Some(ModelCap {
            name: l
              .pointer("/scope/model/display_name")?
              .as_str()?
              .to_string(),
            percent: l.get("percent")?.as_f64()?,
            resets_at: resets_at(l),
          })
        })
        .collect()
    });
  Ok(Usage {
    source: Some(Source {
      kind: "endpoint".to_string(),
      session_id: None,
    }),
    error: None,
    read_at: None,
    five_hour: endpoint_window(&answer, "five_hour"),
    seven_day: endpoint_window(&answer, "seven_day"),
    models,
  })
}

/// One reading of the endpoint now, for intentd's census tick, through
/// `tools`. `read_at` is the one clock reading of that tick.
pub fn read_endpoint(tools: &EndpointTools, claude_dir: &Path, configured: bool) -> Endpoint {
  let read_at = iso(super::clock::now());
  read_token(&tools.security, claude_dir, configured)
    .and_then(|token| fetch(&tools.curl, &tools.url, &token))
    .and_then(|(status, body, retry_after)| match status {
      200 => from_endpoint(&body),
      429 => Err(EndpointFailure::Throttled { retry_after }),
      status => Err(EndpointFailure::Status { status }),
    })
    .map(|usage| Usage { read_at, ..usage })
}

fn read_report(path: &Path) -> Result<Report, Unreadable> {
  let unread = |why: String| Unreadable {
    path: path.display().to_string(),
    why,
    pid: None,
  };
  let text = std::fs::read_to_string(path).map_err(|e| unread(e.to_string()))?;
  let report: Report =
    serde_json::from_str(&text).map_err(|e| unread(format!("not a usage report: {e}")))?;
  if report.schema != REPORT_SCHEMA {
    return Err(unread(format!(
      "usage report schema `{}` is not `{REPORT_SCHEMA}`",
      report.schema
    )));
  }
  Ok(report)
}

/// `<session-id>.usage.json` under the estate directory.
fn report_paths(dirs: &Dirs) -> Vec<PathBuf> {
  let Ok(entries) = std::fs::read_dir(estate_dir(dirs)) else {
    return Vec::new();
  };
  let mut paths: Vec<PathBuf> = entries
    .filter_map(Result::ok)
    .map(|e| e.path())
    .filter(|p| {
      p.file_name()
        .and_then(|n| n.to_str())
        .and_then(|n| n.strip_suffix(".usage.json"))
        .is_some_and(valid_session_id)
    })
    .collect();
  paths.sort();
  paths
}

#[cfg(test)]
mod tests {
  use super::*;

  const SESSION: &str = "00000000-0000-4000-8000-000000000000";

  fn input(rate_limits: &str) -> String {
    format!(r#"{{"session_id":"{SESSION}","model":{{"id":"m"}},"rate_limits":{rate_limits}}}"#)
  }

  #[test]
  fn a_report_normalises_both_windows_and_names_the_missing_primary() {
    let report = Report {
      schema: REPORT_SCHEMA.to_string(),
      session_id: SESSION.to_string(),
      // 1792857600 is 2026-10-24T16:00:00Z (`date -u -r 1792857600`).
      rate_limits: serde_json::json!({
        "five_hour": {"used_percentage": 42.5, "resets_at": 1792857600},
        "seven_day": {"used_percentage": 7}
      }),
    };
    let usage = from_report(&report, Some("2026-10-24T15:00:00.000Z".to_string()));
    assert_eq!(
      usage.source,
      Some(Source {
        kind: "statusline".to_string(),
        session_id: Some(SESSION.to_string())
      })
    );
    assert_eq!(usage.error.as_deref(), Some(NO_ENDPOINT));
    assert_eq!(
      usage.five_hour,
      Some(Window {
        percent: 42.5,
        resets_at: Some("2026-10-24T16:00:00.000Z".to_string())
      })
    );
    assert_eq!(
      usage.seven_day,
      Some(Window {
        percent: 7.0,
        resets_at: None
      })
    );
    assert_eq!(usage.models, None);
  }

  #[test]
  fn report_keeps_the_latest_and_fallback_reads_it_back() {
    let home = std::env::temp_dir().join(format!("estate-usage-{}", std::process::id()));
    let dirs = Dirs::at_home(&home);
    let mut unreadable = Vec::new();
    assert_eq!(
      fallback(&dirs, &mut unreadable),
      Usage {
        error: Some(NO_ENDPOINT.to_string()),
        ..Usage::default()
      }
    );
    assert_eq!(
      report(&dirs, &format!(r#"{{"session_id":"{SESSION}"}}"#)).unwrap(),
      None
    );
    let written = report(&dirs, &input(r#"{"five_hour":{"used_percentage":10}}"#))
      .unwrap()
      .expect("a report with rate_limits is kept");
    assert!(written.ends_with(format!("estate/{SESSION}.usage.json")));
    report(&dirs, &input(r#"{"five_hour":{"used_percentage":20}}"#)).unwrap();
    let usage = fallback(&dirs, &mut unreadable);
    assert!(unreadable.is_empty(), "{unreadable:?}");
    assert_eq!(usage.five_hour.map(|w| w.percent), Some(20.0));
    assert!(usage.read_at.is_some());
    let _ = std::fs::remove_dir_all(&home);
  }

  /// The fixture is one real answer of 2026-10-24, trimmed to the keys the
  /// reader reads (`five_hour`, `seven_day`, `seven_day_sonnet`, `limits`):
  /// it carries no account, organisation or user identifier, the session
  /// fixtures' standard, and `+00:00` stamps with microseconds, which the
  /// reader must turn into the estate's `Z` form.
  #[test]
  fn the_endpoint_answer_normalises_windows_and_model_caps() {
    let body = std::fs::read_to_string(
      Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/estate/usage-endpoint-2026-10-24.json"),
    )
    .expect("the endpoint fixture");
    let usage = from_endpoint(&body).expect("the real answer parses");
    assert_eq!(
      usage.source,
      Some(Source {
        kind: "endpoint".to_string(),
        session_id: None
      })
    );
    assert_eq!(usage.error, None);
    assert_eq!(
      usage.five_hour,
      Some(Window {
        percent: 15.0,
        resets_at: Some("2026-10-24T19:50:00.361Z".to_string())
      })
    );
    assert_eq!(usage.seven_day.map(|w| w.percent), Some(4.0));
    assert_eq!(
      usage.models,
      Some(vec![ModelCap {
        name: "Fable".to_string(),
        percent: 3.0,
        resets_at: Some("2026-10-29T21:00:00.361Z".to_string())
      }])
    );
    assert!(matches!(
      from_endpoint("[]"),
      Err(EndpointFailure::Unparsable { .. })
    ));
  }

  #[test]
  fn a_failed_endpoint_is_named_over_the_fallback() {
    let dirs = Dirs::at_home(Path::new("/nonexistent/estate-usage-resolve"));
    let mut unreadable = Vec::new();
    let usage = resolve(
      &Reading::Read(Err(EndpointFailure::Status { status: 401 })),
      &dirs,
      &mut unreadable,
    );
    assert_eq!(usage.error.as_deref(), Some("the endpoint answered 401"));
    assert_eq!(usage.source, None);
  }

  #[test]
  fn a_reader_that_is_not_intentd_names_no_endpoint() {
    let dirs = Dirs::at_home(Path::new("/nonexistent/estate-usage-resolve"));
    let mut unreadable = Vec::new();
    let usage = resolve(&Reading::NotIntentd, &dirs, &mut unreadable);
    assert_eq!(usage.error.as_deref(), Some(NO_ENDPOINT));
  }

  /// Issue 0659: intentd before its first endpoint read reports no failure,
  /// with no fallback and with one, whose own [`NO_ENDPOINT`] it drops.
  #[test]
  fn intentd_before_its_first_read_names_no_failure() {
    let empty = Dirs::at_home(Path::new("/nonexistent/estate-usage-resolve"));
    let mut unreadable = Vec::new();
    assert_eq!(
      resolve(&Reading::NotReadYet, &empty, &mut unreadable),
      Usage::default()
    );
    let home = std::env::temp_dir().join(format!("estate-usage-0659-{}", std::process::id()));
    let dirs = Dirs::at_home(&home);
    report(&dirs, &input(r#"{"five_hour":{"used_percentage":20}}"#)).unwrap();
    let usage = resolve(&Reading::NotReadYet, &dirs, &mut unreadable);
    assert!(unreadable.is_empty(), "{unreadable:?}");
    assert_eq!(usage.error, None);
    assert_eq!(usage.source.map(|s| s.kind), Some("statusline".to_string()));
    assert_eq!(usage.five_hour.map(|w| w.percent), Some(20.0));
    let _ = std::fs::remove_dir_all(&home);
  }

  #[test]
  fn intentd_after_a_good_read_reports_the_endpoint() {
    let dirs = Dirs::at_home(Path::new("/nonexistent/estate-usage-resolve"));
    let mut unreadable = Vec::new();
    let good = Usage {
      source: Some(Source {
        kind: "endpoint".to_string(),
        session_id: None,
      }),
      read_at: Some("2026-10-29T16:00:00.000Z".to_string()),
      ..Usage::default()
    };
    assert_eq!(
      resolve(&Reading::Read(Ok(good.clone())), &dirs, &mut unreadable),
      good
    );
  }

  #[test]
  fn the_keychain_service_carries_the_config_dirs_digest() {
    assert_eq!(
      service(Path::new("/Users/someone/.config/claude"), true),
      "Claude Code-credentials-f6ab3f41"
    );
    // Claude Code's condition is the variable, so the default dir named
    // explicitly is suffixed too.
    assert_ne!(
      service(Path::new("/Users/someone/.claude"), true),
      "Claude Code-credentials"
    );
    assert_eq!(
      service(Path::new("/Users/someone/.claude"), false),
      "Claude Code-credentials"
    );
  }

  /// Every process's command line, as `ps` shows anyone on the host.
  fn every_argv() -> String {
    let out = Command::new("/bin/ps")
      .args(["-axo", "args="])
      .output()
      .expect("ps runs");
    String::from_utf8_lossy(&out.stdout).into_owned()
  }

  #[test]
  fn the_token_reaches_the_server_and_no_command_line() {
    use std::io::{BufRead, BufReader};
    let token = format!("dummy-token-{}", std::process::id());

    // The instrument first: a token that IS in an argv is seen.
    let mut control = Command::new("/bin/sh")
      .args(["-c", "sleep 5; :", &token])
      .spawn()
      .expect("the control runs");
    let seen = (0..50).any(|_| {
      std::thread::sleep(std::time::Duration::from_millis(20));
      every_argv().contains(&token)
    });
    let _ = control.kill();
    let _ = control.wait();
    assert!(
      seen,
      "ps never showed a token carried in argv, so the check below proves nothing"
    );

    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("a local listener");
    let url = format!("http://{}/api/oauth/usage", listener.local_addr().unwrap());
    let expected = token.clone();
    let server = std::thread::spawn(move || {
      let (stream, _) = listener.accept().expect("curl connects");
      let mut lines = BufReader::new(&stream);
      let mut header = String::new();
      let mut authorised = false;
      while lines.read_line(&mut header).is_ok_and(|n| n > 2) {
        authorised |= header.trim() == format!("Authorization: Bearer {expected}");
        header.clear();
      }
      // curl is waiting on this answer, so it is running now.
      let leaked = every_argv().contains(&expected);
      let mut out = &stream;
      out
        .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{}")
        .expect("answer");
      (authorised, leaked)
    });
    let (status, body, retry_after) =
      fetch(Path::new(CURL), &url, &token).expect("curl reaches the listener");
    let (authorised, leaked) = server.join().expect("the server thread");
    assert_eq!((status, body.as_str(), retry_after), (200, "{}", None));
    assert!(authorised, "the bearer header never reached the server");
    assert!(!leaked, "the token was on a command line while curl ran");
  }

  #[test]
  fn report_refuses_input_it_cannot_attribute() {
    let dirs = Dirs::at_home(Path::new("/nonexistent/estate-usage"));
    assert!(matches!(
      report(&dirs, "not json"),
      Err(EstateError::BadStatusline { .. })
    ));
    assert!(matches!(
      report(&dirs, r#"{"rate_limits":{}}"#),
      Err(EstateError::BadStatusline { .. })
    ));
    assert!(matches!(
      report(&dirs, r#"{"session_id":"../x","rate_limits":{}}"#),
      Err(EstateError::BadSessionId { .. })
    ));
  }
}
