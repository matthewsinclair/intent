//! The append-only event log (design.md D15): every mutation writes an
//! envelope. It is the audit trail, the subscription feed the TUI/bus will
//! consume, and the substrate a future intentc sync protocol replays.
//!
//! DB-only state must be losable; the event log is the deliberate exception
//! to derivability and is explicitly NOT durable truth in v3 -- `rm` of the
//! DB loses it, by design (data-model.md).

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

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
  /// Mint an envelope stamped now, with a fresh ULID.
  pub fn new(
    principal: &str,
    project_id: &str,
    op: &str,
    subject: Subject,
    payload: serde_json::Value,
  ) -> Self {
    let ts = OffsetDateTime::now_utc()
      .format(&Rfc3339)
      .expect("RFC 3339 formatting of a UTC timestamp cannot fail");
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
