//! The append-only event log (design.md D15): every mutation writes an
//! envelope. It is the audit trail, the subscription feed the TUI/bus will
//! consume, and the record of what happened that travels between clones.
//!
//! **Nothing derives it, which makes it the sharpest case in the truth model.**
//! Under D01 as reversed the DB is truth and the files are an extract; for
//! every other entity that extract is a faithful copy, so losing the DB costs
//! the work of rebuilding and nothing else. History is where that stops being
//! true: nothing recomputes what happened, so a committed file form is the
//! only thing that can carry it off this machine.
//!
//! **ONE COMMITTED FILE PER EVENT, UNDER `.canon/events/<YYYY>/<MM>/<DD>/`**
//! (ST0078 P1, which reverses D53). The file is written in the same write set
//! as the canon and views of the act that produced it, so an act and its record
//! land together. One file per event is merge-free by construction: the name is
//! the event's ULID, so no two writers ever touch one file, and a file is never
//! rewritten. Ingest is additive -- [`from_file`] reads one back, and the store
//! inserts an id it does not hold and skips one it does.
//!
//! **THE BOUNDARY THAT KEEPS D01 INTACT.** Canon is the STATE extract and these
//! files are the ACT record. Nothing rebuilds state by replaying events, and
//! doctor never reconciles the two, so there is no second truth.
//!
//! The single-file form is [`JSONL`] -- one envelope per line, in log order --
//! and `intent export` still produces it on demand. JSON Lines rather than a
//! JSON array because a new envelope is a new line; an array would turn every
//! append into a whole-file rewrite and make a truncated write
//! indistinguishable from a corrupt one. [`merge`] keys on the envelope's ULID
//! and adds what is missing, so restoring an older extract over a newer log is
//! a no-op rather than a loss.

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
/// after that: the `Envelope::minted` call sites in this crate are
/// `record_disk_act`, the `text.realise` writer, the generic entity recorder,
/// `init`, and the whiteboard's two (`Facade::wb_event` and the roster read) --
/// so the vocabulary is the literals reaching those doors and nothing else.
pub const KNOWN_OPS: &[&str] = &[
  "ac.descope",
  "ac.edit",
  "ac.fc",
  "at.fc",
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
  "attachment.detach",
  "disk.declare_default",
  "disk.dehydrate",
  "disk.hydrate",
  "disk.organize",
  "disk.sync_from_disk",
  "disk.sync_to_disk",
  "init",
  // **`issue.set` IS SINGULAR AND THE REST OF ITS FAMILY IS PLURAL. THAT IS
  // TWO NAMING SOURCES, NOT A HALF-FINISHED RENAME**, and it is recorded here
  // because the asymmetry reads exactly like one. `issue.set` is minted by the
  // generic address-based setter, which names the ENTITY (`AddrEntity::Issue`)
  // -- the same shape as `ac.set`. The verbs below are minted by the issue
  // family, which names the CLI FAMILY. The two sources AGREE for `ac` and
  // DISAGREE for `issue`/`issues`, so nothing is broken and nothing is
  // mid-migration. Renaming either half to match would break the source it
  // actually comes from.
  "issue.set",
  "issues.add",
  "issues.close",
  // **ADDED 2026-09-07: written since it shipped and never declared.**
  // `facade.rs`'s issue-edit door mints it, and the live check found it exactly
  // as designed -- 10 rows on Devbin, 135 on Intent, all reported as an op
  // "this build does not declare". The roster's own note says the trigger to
  // watch is the first RENAME or RETIREMENT; this was neither, just a door
  // whose literal never reached the list.
  "issues.edit",
  "issues.open",
  "issues.renumber",
  "st.cancel",
  "st.done",
  "st.fc",
  "st.hold",
  "st.new",
  "st.reinstate",
  "st.renumber",
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
  "wp.fc",
  "wp.new",
  "wp.reinstate",
  "wp.reopen",
  "wp.rescope",
  "wp.set",
  "wp.start",
  "wp.unstart",
  // **THE WHITEBOARD FAMILY, ADDED 2026-09-16 (issue 0411).** Every board verb
  // wrote rows and no event, so the log could not say who wrote a board row.
  // One op per verb, minted by `Facade::wb_event` and by the roster read.
  "wb.add",
  "wb.announce",
  "wb.archive",
  "wb.ask",
  "wb.claim",
  "wb.clear",
  "wb.correct",
  "wb.decide",
  "wb.migrate",
  "wb.pickup",
  "wb.register",
  "wb.release",
  "wb.touch",
  "wb.unclaim",
];

/// The principal written when nothing names an author: no `author` in the
/// project's config and no git identity.
pub const LOCAL_PRINCIPAL: &str = "local";

/// The `author` `intent init` records when no author is known (nothing
/// bootstrapped on this machine). It is a placeholder, not a name, so
/// [`author`] reads it as absent.
pub const UNKNOWN_AUTHOR: &str = "unknown";

/// Who an event says acted: git's `user.name` and `user.email`, else the
/// project's configured `author`, else [`LOCAL_PRINCIPAL`] (ST0078 P1).
///
/// **The principal is what makes a travelling event mean anything.** Until the
/// log travelled, every row read `local` and nobody could tell; once an act on
/// one clone is read on another, `local` says only that somebody did something.
///
/// **GIT FIRST, BECAUSE `config.json` IS COMMITTED** (vc, 2026-09-18, reversing
/// the order first given). A committed author is one name for every clone of the
/// project, so on a team it would sign every teammate's act with whoever ran
/// `init`; git's identity is the one that belongs to the person at this clone.
/// The config's author is the fallback for a machine with no git identity.
///
/// Pure: the caller reads git and the config and hands the values in, so the
/// order of preference has one home and is driven without a repository. A blank
/// value counts as absent, because an empty `user.name` or `author` is a field
/// nobody filled in, not a name, and so does [`UNKNOWN_AUTHOR`] in the config.
pub fn author(git_name: Option<&str>, git_email: Option<&str>, config_author: &str) -> String {
  fn present(v: Option<&str>) -> Option<&str> {
    v.map(str::trim).filter(|v| !v.is_empty())
  }
  match (present(git_name), present(git_email)) {
    (Some(name), Some(email)) => format!("{name} <{email}>"),
    (Some(name), None) => name.to_string(),
    (None, Some(email)) => format!("<{email}>"),
    (None, None) => present(Some(config_author))
      .filter(|a| *a != UNKNOWN_AUTHOR)
      .unwrap_or(LOCAL_PRINCIPAL)
      .to_string(),
  }
}

/// The ops that describe one machine and are false on every other clone, so
/// their events stay in the store and are never written as files (hv,
/// 2026-09-18, AC-01.1; the classification is vc's, under the pen).
///
/// **The test is whether the act changes the model or re-derives from it.** An
/// op that changes the model -- a thread, package, criterion, test, issue,
/// field, attachment, board, claim or the register -- travels. An op that
/// re-derives files or store state from the model on this machine stays:
///
/// - `disk.organize`, `disk.sync_to_disk` and `text.realise` render files from
///   this machine's store. The files they write that git carries are explained
///   by the commit that carries them, not by an event.
/// - `disk.sync_from_disk` is the destructive restore, `sync --to-store`: this
///   clone's store was replaced from its own tree.
/// - `wb.touch` and `wb.pickup` stamp a heartbeat: this session, on this
///   machine, was alive at this moment.
///
/// **What looks close and is not here, so it travels:** `wb.release` sets a
/// node's status, which is board state every clone reads; `disk.hydrate`,
/// `disk.dehydrate` and `disk.declare_default` change the register in
/// `.intentfiles`; `todo.flush` moves the watermark in `project.json`.
///
/// **Two classes the ruling names mint no event at all, so they have no entry
/// here.** An ingest is recorded in `ingest_log`, not `event_log`, and an index
/// rebuild records nothing in either. If one of them ever mints an event, its
/// op joins this list.
///
/// Every entry is also in [`KNOWN_OPS`], and a test holds that, so a rename
/// cannot leave an entry here that matches nothing and quietly start carrying
/// the renamed op off the machine.
pub const MACHINE_SCOPED_OPS: &[&str] = &[
  "disk.organize",
  "disk.sync_from_disk",
  "disk.sync_to_disk",
  "text.realise",
  "wb.pickup",
  "wb.touch",
];

/// Does an event of this `op` travel -- is it written as a committed file?
///
/// **PROJECT ACTS TRAVEL AND MACHINE-SCOPED ACTS DO NOT** (hv, 2026-09-18). The
/// rule is whether the act describes one machine and is false on another clone;
/// [`MACHINE_SCOPED_OPS`] is that rule applied to [`KNOWN_OPS`], and this is the
/// one place it is read. An op outside the roster travels: an act nobody has
/// classified is a project act until someone says otherwise, and a record that
/// travels and was not needed costs less than one that was needed and stayed
/// behind. An event that does not travel is still written to the store.
pub fn travels(op: &str) -> bool {
  !MACHINE_SCOPED_OPS.contains(&op)
}

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

/// The next event id, MONOTONIC within a millisecond.
///
/// **`Ulid::new()` IS NOT ENOUGH, AND A CASCADE IS WHAT PROVED IT.** A ULID's
/// first 48 bits are the millisecond and the remaining 80 are random, so two
/// minted in the same millisecond sort by their RANDOM half -- in any order.
/// The whole of a fiat cascade is minted inside one transaction, so measured on
/// a real close (2026-08-30): five events, one timestamp, and `at.fc` sorting
/// BEFORE the `st.fc` that caused it. **A child ahead of its ancestor in a log
/// whose ids are documented as lexically sortable.**
///
/// That matters exactly as much as the ruling it undermines: one-event-per-entity
/// was chosen so a consumer replaying the log rebuilds the estate, and a replay
/// that meets `at.fc` first is applying a cascade before the close that
/// justifies it. **`inherited_event` recovers the GROUPING and cannot recover
/// the ORDER.**
///
/// A shared generator makes ids monotonic within a millisecond, so insertion
/// order and lexical order agree. **Applied to every event rather than to
/// cascades**, because the property the `event_log` row claims -- *lexically
/// sortable* -- is claimed for all of them, and a fix scoped to the one caller
/// that noticed would leave the claim false everywhere else.
///
/// **Falls back to a fresh random ULID rather than failing**: `generate` can
/// refuse only when the random half has been exhausted inside a single
/// millisecond, and a monotonic id is a nicety where an id at all is not.
fn next_id() -> String {
  use std::sync::Mutex;
  static GEN: Mutex<Option<ulid::Generator>> = Mutex::new(None);
  let mut guard = match GEN.lock() {
    Ok(g) => g,
    // A poisoned lock means another thread panicked mid-mint; an id is still
    // owed, and refusing here would turn someone else's panic into ours.
    Err(poisoned) => poisoned.into_inner(),
  };
  let generator = guard.get_or_insert_with(ulid::Generator::new);
  match generator.generate() {
    Ok(id) => id.to_string(),
    Err(_) => ulid::Ulid::new().to_string(),
  }
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
      id: next_id(),
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
// The committed file form: one file per event (ST0078 P1)
// ---------------------------------------------------------------------------

/// The directory under `.canon/` that holds the committed event files.
pub const EVENTS_DIR: &str = "events";

/// Where one written event lives, relative to [`EVENTS_DIR`]:
/// `<YYYY>/<MM>/<DD>/<ulid>.json`, dated by the event's own `ts`.
///
/// **Dated by `ts` and not by the ULID's time prefix.** The two agree for an
/// event this binary wrote, but `ts` is the database's stamp (D42) and the one
/// every reader orders by, so the directory a person opens for a day holds the
/// events that say they happened that day.
///
/// Refuses an envelope with no `ts`: it was never written, so it is not
/// history (the same refusal [`to_jsonl`] makes), and it has no day to live in.
pub fn file_rel(e: &Envelope) -> Result<std::path::PathBuf, EventFileError> {
  let day = e
    .ts
    .get(..10)
    .filter(|d| {
      let b = d.as_bytes();
      b[4] == b'-' && b[7] == b'-' && d.chars().filter(|c| c.is_ascii_digit()).count() == 8
    })
    .ok_or_else(|| EventFileError::Unstamped { id: e.id.clone() })?;
  Ok(
    std::path::PathBuf::from(&day[..4])
      .join(&day[5..7])
      .join(&day[8..10])
      .join(format!("{}.json", e.id)),
  )
}

/// One event's committed bytes: 2-space pretty JSON with a trailing newline,
/// the form every other canon file takes.
pub fn to_file(e: &Envelope) -> Result<String, serde_json::Error> {
  crate::model::to_canonical_json(e)
}

/// Read one committed event file back, refusing one whose name disagrees with
/// the id it carries.
///
/// `stem` is the file name without `.json`. **The name IS the key additive
/// ingest dedups on, so a file whose name and id disagree is two claims about
/// one record** -- ingest would take the id and a reader would find it under
/// another name -- and neither can be picked without guessing.
pub fn from_file(stem: &str, text: &str) -> Result<Envelope, EventFileError> {
  let value: serde_json::Value = serde_json::from_str(text).map_err(EventFileError::NotJson)?;
  let e: Envelope = serde_json::from_value(value).map_err(EventFileError::NotAnEnvelope)?;
  if e.id != stem {
    return Err(EventFileError::NameDisagrees {
      name: stem.to_string(),
      id: e.id,
    });
  }
  if e.ts.is_empty() {
    return Err(EventFileError::Unstamped { id: e.id });
  }
  Ok(e)
}

#[derive(Debug, thiserror::Error)]
pub enum EventFileError {
  /// Not JSON at all. The tree scan already reports this as `malformed-json`
  /// for every `.json` file, so a reader that also reports it says it twice.
  #[error("not JSON: {0}")]
  NotJson(#[source] serde_json::Error),
  #[error("not an event envelope: {0}")]
  NotAnEnvelope(#[source] serde_json::Error),
  #[error("the file is named {name} and carries the id {id}")]
  NameDisagrees { name: String, id: String },
  #[error("event {id} has no timestamp, so it was never written to a store")]
  Unstamped { id: String },
}

impl crate::remedy::Remedy for EventFileError {
  /// **Restore the file from git; never edit it into agreement.** An event file
  /// is written once and never rewritten, so a damaged one is repaired by
  /// putting back the bytes that were committed.
  fn remedy(&self) -> String {
    "restore the file from git (`git checkout -- <path>`) -- an event file is written once and never rewritten, so the committed bytes are the record".to_string()
  }
}

// ---------------------------------------------------------------------------
// The todo DONE watermark, which is an EVENT rather than a setting
// ---------------------------------------------------------------------------

/// The op recorded by `intent todo done --flush`.
pub const TODO_FLUSH: &str = "todo.flush";
