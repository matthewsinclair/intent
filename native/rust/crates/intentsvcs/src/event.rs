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

/// Every `op` this binary can write into the log.
///
/// **THIS IS A ROSTER, NOT AN ENUM, AND THE DISTINCTION IS DELIBERATE.** There
/// is no type, no parse, no `FromStr`, and nothing refuses an op for being
/// absent from this list. Whether the op vocabulary should become a TYPE is
/// hv's open question (vc's Highlander finding F1), and a type arriving by way
/// of a check nobody asked for would be a ruling nobody made. What this answers
/// is the narrower question hv's is waiting on: **would a parse, if one existed,
/// meet anything in the wild it does not know.**
///
/// # It is a second home for the vocabulary, and that is paid for rather than denied
///
/// The op strings are spelled at their call sites; this repeats them. A second
/// home drifts -- that is the whole of `IN-AG-HIGHLANDER-001` -- so two
/// mechanisms hold it in step, and **each catches a direction the other cannot
/// see**:
///
/// - `known_ops_are_spelled_in_the_source_that_declares_them` reds when an op is
///   RETIRED or RENAMED and left here, which is the direction the live check
///   below cannot detect at all: the code stops producing it, and a log that
///   already carries it goes on matching a roster entry that means nothing.
/// - `every_transition_op_is_in_the_roster` reds when a new `st.*` edge lands
///   and nobody adds it here.
///
/// **NEITHER CATCHES A BRAND-NEW OP IN A FAMILY WITH NO STATE MACHINE** -- a
/// fresh `disk.*` or `issues.*` op added and not listed. That gap is real, it is
/// named here rather than left to be discovered, and it is **self-reporting**:
/// the first time such an op is written, the live check reports it as one this
/// binary does not declare. Loud, in the safe direction, and the fix is one line.
///
/// # The corpus this was sized against, so a later reader does not re-derive it
///
/// Measured 2026-08-27. **11 families HERE** -- ac at attachment disk issue
/// issues st text thread todo wp -- against a live estate store holding 21
/// distinct ops in 425 events, and 22 distinct across the 15 stores on that
/// machine, which fall in 8 families. **The two family counts are different
/// questions and were nearly written down as one:** 8 is how many families have
/// ever been WRITTEN on that machine, 11 is how many this binary can write.
///
/// **THE OP COUNT USED TO BE WRITTEN HERE AND IS NOT ANY MORE, BECAUSE IT WAS A
/// SECOND HOME FOR SOMETHING THE LIST BELOW ALREADY SAYS.** The prose read "43
/// ops" in two places and the test header in a third; `ac.edit` and `at.edit`
/// landed on 2026-08-29 and all three went stale in one commit, silently,
/// because nothing compares a sentence to a list. The FAMILY count is kept
/// because it is the comparison being made and does not move when an op is
/// added to a family that exists. `KNOWN_OPS.len()` is the op count. A vocabulary is always at least as wide as its use, and quoting
/// the corpus figure for the code would have understated the roster by three
/// families that simply have not been exercised.
///
/// Every op in the wild was spelled, so **the compat case is EMPTY TODAY** and
/// the trigger to watch is the first RENAME or RETIREMENT rather than the first
/// event. Six `disk.*` ops have no state machine behind them, which is why the
/// transitions table cannot derive this list and why the roster exists at all.
///
/// # `init` HAS NO DOT, AND THAT IS A FACT ABOUT THE VOCABULARY, NOT A TYPO
///
/// Every other member is `family.verb`. `init` is not: it is the one event a
/// project writes before it has any entity to name, so there is no family for
/// it to belong to. **It was missed by the first draft of this roster**, which
/// was seeded by grepping for `"<word>.<word>"` literals -- a pattern that
/// cannot match it -- and it was the live check that found it, on first contact
/// with a real estate, exactly as designed.
///
/// It is worth carrying into hv's open question rather than filed as trivia:
/// **a vocabulary with an irregular member is a vocabulary whose shape cannot
/// be assumed by whatever parses it.** Any future type has to hold `init`
/// alongside the dotted names, or the shape rule is wrong on its first row.
///
/// The population is bounded and was enumerated by DOOR rather than by pattern
/// after that: there are exactly four `Envelope::minted` call sites in this
/// crate -- `record_disk_act`, the `text.realise` writer, the generic entity
/// recorder, and `init` -- so the vocabulary is the literals reaching those
/// four and nothing else.
pub const KNOWN_OPS: &[&str] = &[
  "ac.descope",
  "ac.edit",
  "ac.put",
  "ac.reinstate",
  "ac.rescope",
  "ac.satisfy",
  "ac.set",
  "ac.unsatisfy",
  "ac.withdraw",
  "at.edit",
  "at.put",
  "at.set",
  "attachment.put",
  "disk.declare_default",
  "disk.dehydrate",
  "disk.hydrate",
  "disk.organize",
  "disk.sync_from_disk",
  "disk.sync_to_disk",
  "init",
  "issue.set",
  "issues.add",
  "issues.close",
  "issues.open",
  "st.cancel",
  "st.done",
  "st.hold",
  "st.new",
  "st.reinstate",
  "st.reopen",
  "st.resume",
  "st.start",
  "st.triage",
  "text.realise",
  "thread.put",
  "thread.set",
  "todo.flush",
  "wp.cancel",
  "wp.done",
  "wp.new",
  "wp.reinstate",
  "wp.reopen",
  "wp.rescope",
  "wp.set",
  "wp.start",
  "wp.unstart",
];

/// The principal a facade call runs as. `local` until the 3.2 agent bus
/// gives principals meaning (vc/cc/hv) and intentc federates them (v4).
pub const LOCAL_PRINCIPAL: &str = "local";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Envelope {
  /// ULID -- lexically sortable, globally unique.
  pub id: String,
  /// RFC 3339 UTC, millisecond precision: `YYYY-MM-DDTHH:MM:SS.sssZ`.
  // **Everything below is `//` on purpose: a `///` here is SHIPPED OUTPUT.**
  // schemars lifts it into `event.schema.json` and async-graphql into the SDL,
  // so the first draft of this comment published its own reasoning -- and an
  // internal criterion id with it, straight into a face our own rules say must
  // never carry one. The line above is what a consumer needs; the rest is why.
  //
  // **The exact shape is PUBLISHED rather than merely described.** The stamp
  // moved from second to millisecond precision, and a consumer parsing this
  // field could not have seen that: the face said `"type": "string"` and
  // nothing else, so a format change to the one field the interchange is
  // ORDERED BY was invisible in the contract describing it. The `pattern`
  // makes precision part of what a consumer compiles against, so the next such
  // change moves the schema version instead of passing in silence.
  //
  // Milliseconds rather than seconds because two machines MERGE their event
  // logs (D34) and order them by this value; at second resolution two writes
  // in one second collide, which is what any script does.
  #[schemars(
    extend("format" = "date-time"),
    extend("pattern" = r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z$")
  )]
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
  /// Natural id, eg `ST0000`, `ST0000/02`, `0021`.
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

impl crate::remedy::Remedy for JsonlError {
  /// **Restore the line; never delete it, and never truncate the file at it.**
  ///
  /// The log is the one artefact in the estate nothing recomputes -- the store
  /// is rebuildable from the extract (D36) and history is rebuildable from
  /// nothing -- so the instinct that works everywhere else, throw away the bad
  /// part and regenerate, silently destroys the only copy here.
  fn remedy(&self) -> String {
    format!(
      "restore line {} from git rather than deleting it -- the event log is the one artefact nothing recomputes, so a truncated log is permanent loss and looks exactly like a project that did less work",
      self.line
    )
  }
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

// ---------------------------------------------------------------------------
// The todo DONE watermark, which is an EVENT rather than a setting
// ---------------------------------------------------------------------------

/// The op recorded by `intent todo done --flush`.
pub const TODO_FLUSH: &str = "todo.flush";
