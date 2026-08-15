//! The append-only event log (design.md D15): every mutation writes an
//! envelope. It is the audit trail, the subscription feed the TUI/bus will
//! consume, and the substrate a future intentc sync protocol replays.
//!
//! **Nothing derives it, which makes it the sharpest case in the truth model.**
//! Under D01 as reversed the DB is truth and the files are an extract; for
//! every other entity that extract is a faithful copy, so losing the DB costs
//! the work of rebuilding and nothing else. History is where that stops being
//! true: nothing recomputes what happened, so the log's committed file form is
//! **the only thing that can carry it off this machine** (D34).
//!
//! That file form is [`JSONL`] -- one envelope per line, in log order -- and it
//! is built here. It was owed for exactly as long as the log had no way to
//! travel, and while it was owed the DB was the only copy of history there was.
//!
//! **JSON Lines rather than a JSON array, and the reason is the append-only
//! property.** A new envelope is a new line; an array would require rewriting
//! the closing bracket, which turns every append into a whole-file rewrite and
//! makes a truncated write indistinguishable from a corrupt one. Line-oriented
//! also means the artefact stays greppable, diffable per event, and readable by
//! anything that can read a line -- which is what openness asks of a file form.
//!
//! **Merged on the way in, never replaced.** The log is append-only, so a
//! restore that wiped it would destroy history that the extract simply had not
//! caught up with. [`merge`] keys on the envelope's ULID and adds what is
//! missing, so restoring an older extract over a newer log is a no-op rather
//! than a loss.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// The principal a facade call runs as. `local` until the 3.2 agent bus
/// gives principals meaning (vc/cc/hv) and intentc federates them (v4).
pub const LOCAL_PRINCIPAL: &str = "local";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Envelope {
  /// ULID -- lexically sortable, globally unique.
  pub id: String,
  /// RFC 3339 UTC timestamp.
  pub ts: String,
  pub principal: String,
  pub project_id: String,
  /// The facade operation, eg `st.done`.
  pub op: String,
  pub subject: Subject,
  /// Operation-specific detail; opaque to the log.
  pub payload: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Subject {
  /// Entity type, eg `thread`, `wp`, `issue`.
  #[serde(rename = "type")]
  pub kind: String,
  /// Natural id, eg `ST0056`, `ST0056/02`, `0021`.
  pub id: String,
}

impl Envelope {
  /// Mint an envelope that has NOT been written yet, and therefore has NO
  /// time.
  ///
  /// **It used to read the process clock, and under hv's ruling that is the
  /// defect** (2026-08-15): time comes from the DB. The event log is the
  /// durable record of when things happened, so a `ts` taken from whichever
  /// machine's process happened to write it made the log's ordering an
  /// accident of who ran the command. Two nodes syncing their logs together
  /// (D34) would interleave by two unreconciled clocks.
  ///
  /// **D42: `ts` is left EMPTY and the database fills it at the point of
  /// INSERT.** An envelope in this state is a record of nothing until it is
  /// written, which is exactly what it is -- and there is no argument here for
  /// a caller to supply a time through, because a caller has none to give.
  ///
  /// The emptiness is load-bearing rather than a placeholder, and
  /// [`to_jsonl`] refuses it: an unwritten envelope serialised into the
  /// history file would be a record claiming to have happened at the start of
  /// time.
  pub fn minted(
    principal: &str,
    project_id: &str,
    op: &str,
    subject: Subject,
    payload: serde_json::Value,
  ) -> Self {
    let ts = String::new();
    Self {
      id: ulid::Ulid::new().to_string(),
      ts,
      principal: principal.to_string(),
      project_id: project_id.to_string(),
      op: op.to_string(),
      subject,
      payload,
    }
  }
}

// ---------------------------------------------------------------------------
// The committed file form (D34, AC-02.6)
// ---------------------------------------------------------------------------

/// The extract's name for the event log, under the project's intent directory.
pub const JSONL: &str = "events.jsonl";

/// Render the log as JSONL: one envelope per line, in the order given.
///
/// **Each line is COMPACT, not pretty-printed**, which is the one place the
/// event log's canonical form differs from every other artefact's. A canonical
/// `thread.json` is 2-space pretty because a human reads and reviews it whole;
/// a line here is a record in a stream, and pretty-printing it would put a
/// newline inside a line, which is the one thing the format cannot survive.
pub fn to_jsonl(events: &[Envelope]) -> Result<String, serde_json::Error> {
  let mut out = String::new();
  for e in events {
    // **An envelope with no `ts` was never written** (D42: the database
    // stamps at INSERT), so it is not history and must not enter the history
    // file. Serialising it would publish a record claiming to have happened
    // at the start of time -- and an empty string sorts before every real
    // stamp, so it would also land first in any log a peer merged it into.
    debug_assert!(
      !e.ts.is_empty(),
      "envelope {} has no timestamp, so it was never written to the store; only a written record is history",
      e.id
    );
    out.push_str(&serde_json::to_string(e)?);
    out.push('\n');
  }
  Ok(out)
}

/// Parse JSONL back into envelopes, refusing a bad line BY NUMBER.
///
/// Strict, like every other read of the extract (D05): a line that is not an
/// envelope is refused rather than skipped. Skipping is how a log silently
/// loses the record someone will later look for, and a log with a hole in it is
/// worse than a log that says it has one.
///
/// Blank lines are tolerated and only blank lines -- a trailing newline is a
/// property of the format, not a record.
pub fn from_jsonl(text: &str) -> Result<Vec<Envelope>, JsonlError> {
  let mut out = Vec::new();
  for (n, line) in text.lines().enumerate() {
    if line.trim().is_empty() {
      continue;
    }
    out.push(serde_json::from_str(line).map_err(|source| JsonlError {
      line: n + 1,
      source,
    })?);
  }
  Ok(out)
}

#[derive(Debug, thiserror::Error)]
#[error("{JSONL} line {line}: {source}")]
pub struct JsonlError {
  pub line: usize,
  #[source]
  pub source: serde_json::Error,
}

/// The envelopes in `incoming` that `have` does not already carry, in incoming
/// order.
///
/// **Keyed on the ULID, which is why the id is minted rather than derived.**
/// Two machines appending concurrently produce disjoint ids, so a merge is the
/// union and never a conflict -- and re-restoring the same extract adds
/// nothing, which is what makes the operation safe to repeat.
pub fn merge<'a>(have: &[Envelope], incoming: &'a [Envelope]) -> Vec<&'a Envelope> {
  let known: std::collections::HashSet<&str> = have.iter().map(|e| e.id.as_str()).collect();
  incoming
    .iter()
    .filter(|e| !known.contains(e.id.as_str()))
    .collect()
}
