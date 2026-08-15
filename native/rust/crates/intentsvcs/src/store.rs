//! The store (design.md D01 as reversed, 2026-08-15): a per-project SQLite DB
//! that is **the durable SSOT**, not an index of the files. `rm` of it is NOT
//! safe -- it costs whatever the committed extract does not carry -- and **DB
//! MIGRATIONS ARE NORMAL**, so there is no "rebuild instead of migrating" story
//! here to lean on.
//!
//! **[`Store::rebuild`] is the disk -> db sync direction and nothing else.** It
//! wipes threads, wps, criteria, tests and issues and reloads them from an
//! extract: the right operation for reconstituting a machine's DB from the
//! interchange form (D34), the wrong one for every other job, which is why the
//! write path stopped calling it. The event log is not in that set at all
//! (D15): append-only, nothing derives it, and hv ruled it a first-class
//! artefact with its own committed file form (`events.jsonl`), so "losable by
//! design" is struck.

use rusqlite::{Connection, params};
use serde_json::json;

use crate::event::Envelope;
use crate::model::{
  AcceptanceTest, Criterion, ISSUE_SCHEMA, Issue, Legacy, Related, THREAD_SCHEMA, Thread,
  WorkPackage, enum_str,
};
use crate::prose::DocSection;
use crate::sync::FileEntry;

/// The DDL face. Applied verbatim on open; committed under `schema/ddl.sql`
/// by the faces machinery and drift-checked against this constant.
pub const DDL: &str = "\
-- Intent v3 runtime store (GENERATED FACE -- the master is
-- native/rust/crates/intentsvcs/src/store.rs; regenerate via INTENT_BLESS, never edit).
-- The durable SSOT (D01, reversed 2026-08-15). Re-creatable from the committed
-- extract as a CAPABILITY; migrations are normal when the schema moves.
--
-- EVERY TABLE DECLARES HOW ITS DATA LEAVES. `-- openness: carried by <path>`
-- names the file form that holds it losslessly; `-- openness: DERIVED` states
-- why it needs none, and always says why. Absence of a declaration is never
-- the answer -- a table with no line is a table nobody has said how to get
-- data out of, and tests/openness.rs refuses one.
-- **RECORD TIMESTAMPS (AC-02.8, D42), AND THEY ARE A DIFFERENT KIND OF THING
-- FROM THE DATES BESIDE THEM.** Every table carries one, written by the
-- database as part of the write and never passed in by a caller.
--
-- Two kinds of time live in this schema and conflating them is what put eight
-- tables here with no record timestamp at all:
--
--   (a) A RECORD timestamp is a fact about THIS DATABASE -- when this store
--       wrote this row. It is per-machine, it is deliberately NOT carried in
--       the extract, and a rebuild correctly re-stamps it, because the row
--       genuinely was written then. `created_at` / `updated_at` / `written_at`.
--   (b) A DOMAIN date is a fact about the WORLD -- when a thread was created,
--       when an issue was raised. It is carried in the extract, it is NEVER
--       re-stamped, and it is what `st show` prints. `threads.created`,
--       `threads.completed`, `issues.created`.
--
-- Both are needed and they are not interchangeable. A schema carrying a
-- plausible `created` column is exactly how eight tables shipped with no
-- record time and nobody noticed.
--
-- **THE COLUMN IS NAMED FOR WHAT IT CAN HONESTLY RECORD, NEVER FOR UNIFORMITY
-- ACROSS TABLES** (vc, ruling, 2026-08-15). `threads`, `issues` and
-- `file_index` have durable row identity, so they are UPSERTED and their
-- `created_at` fires exactly once while `updated_at` moves with each write.
-- `related`, `wps`, `criteria` and `tests` are deleted and re-inserted with
-- their parent -- a removed WP must vanish -- so a `created_at` there would
-- record the latest write while carrying the name of the first, which is
-- AC-02.8's remedy reintroducing AC-02.8's defect. They get `written_at`:
-- when THIS VERSION of this row was written, which is what the write strategy
-- can actually support.
--
-- **MILLISECONDS, NOT SECONDS, AND THAT WAS MEASURED RATHER THAN PREFERRED.**
-- Every stamp here is `%f`. At second resolution two writes in the same second
-- carry identical stamps, and that is not an edge case -- it is what any script
-- does, and it was found by a MUTATION TEST rather than by reasoning: reverting
-- `threads` to delete-and-reinsert should have moved `created_at`, the test
-- asserting it did not still passed, and the reason was that both writes landed
-- inside one second. A guard blind to the defect it names is worse than no
-- guard, and the same blindness is load-bearing in the product: under D34 two
-- machines MERGE their event logs, and a merge orders records by a time nobody
-- typed. Colliding stamps make that order arbitrary exactly when it is being
-- relied on.
--
-- That is a scope call with a stated reversal, not a claim about the domain
-- (D39): `wps` and `criteria` do have stable ids, so if per-row durable
-- history is wanted the upgrade is delete-missing + upsert-present, and
-- `written_at` does not block it. What is not reversible is shipping a
-- `created_at` on a table that re-stamps it.
-- openness: carried by intent/st/<ID>/thread.json
CREATE TABLE IF NOT EXISTS threads (
  id TEXT PRIMARY KEY,
  title TEXT NOT NULL,
  slug TEXT,
  status TEXT NOT NULL,
  status_reason TEXT,
  created TEXT NOT NULL,
  completed TEXT,
  acceptance TEXT,
  objective TEXT NOT NULL,
  context TEXT NOT NULL,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);
-- openness: carried by intent/st/<ID>/thread.json
CREATE TABLE IF NOT EXISTS related (
  thread_id TEXT NOT NULL REFERENCES threads (id) ON DELETE CASCADE,
  seq INTEGER NOT NULL,
  id TEXT NOT NULL,
  note TEXT,
  written_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  PRIMARY KEY (thread_id, seq)
);
-- openness: carried by intent/st/<ID>/thread.json
CREATE TABLE IF NOT EXISTS wps (
  thread_id TEXT NOT NULL REFERENCES threads (id) ON DELETE CASCADE,
  seq INTEGER NOT NULL,
  title TEXT NOT NULL,
  scope TEXT NOT NULL,
  status TEXT NOT NULL,
  status_reason TEXT,
  objective TEXT NOT NULL,
  body TEXT NOT NULL,
  written_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  PRIMARY KEY (thread_id, seq)
);
-- `state` is the whole recorded AC state as its serde JSON, replacing the
-- `scope`/`evidence`/`satisfied` trio. One column because the state is one
-- value: the trio could hold combinations the model has no meaning for (a
-- descoped row carrying `satisfied`), and a schema that can represent a
-- contradiction eventually stores one. Same treatment `legacy` already gets.
-- The discriminant stays queryable as `json_extract(state, '$.is')`.
-- openness: carried by intent/st/<ID>/thread.json
CREATE TABLE IF NOT EXISTS criteria (
  thread_id TEXT NOT NULL REFERENCES threads (id) ON DELETE CASCADE,
  id TEXT NOT NULL,
  text TEXT NOT NULL,
  kind TEXT NOT NULL,
  state TEXT NOT NULL,
  written_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  PRIMARY KEY (thread_id, id)
);
-- openness: carried by intent/st/<ID>/thread.json
CREATE TABLE IF NOT EXISTS tests (
  thread_id TEXT NOT NULL REFERENCES threads (id) ON DELETE CASCADE,
  id TEXT NOT NULL,
  kind TEXT NOT NULL,
  file TEXT,
  prose TEXT,
  covers TEXT NOT NULL,
  status TEXT NOT NULL,
  note TEXT,
  legacy TEXT,
  written_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  PRIMARY KEY (thread_id, id)
);
-- `created` is AUTHORED -- v2 users write it by hand in frontmatter, so it is a
-- fact about the world and stays, with a DB stamp beside it rather than
-- replaced by one.
-- openness: carried by intent/issues/<NNNN>.json
CREATE TABLE IF NOT EXISTS issues (
  number INTEGER PRIMARY KEY,
  slug TEXT NOT NULL,
  title TEXT NOT NULL,
  status TEXT NOT NULL,
  severity TEXT,
  created TEXT NOT NULL,
  closed TEXT,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);
-- The sync engine's git-style index (data-model.md). DB-only and derived from
-- the working tree, not from canon, so `rebuild` does not touch it.
-- `findings` is a JSON array; `state` is clean | changed | unparsed.
-- openness: DERIVED -- rebuilt by re-scanning the working tree, and the files it
-- indexes are the user's own data, already readable without Intent.
-- `mtime` is the FILE's, read from the filesystem -- a fact about the file, not
-- about this row. `created_at` / `updated_at` are the row's own, and the two
-- answer different questions: a file untouched since last scan has a moving
-- `updated_at` and a still `mtime`.
CREATE TABLE IF NOT EXISTS file_index (
  path TEXT PRIMARY KEY,
  size INTEGER NOT NULL,
  mtime TEXT NOT NULL,
  sha256 TEXT NOT NULL,
  state TEXT NOT NULL,
  findings TEXT NOT NULL,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);
-- Prose ingest (data-model.md): bodies stored VERBATIM, never modelled, and
-- FTS5-indexed to power `intent search`. One table, not an external-content
-- pair: the store is rebuilt wholesale from canon, so a shadow content table
-- plus triggers would add a drift hazard to buy nothing. UNINDEXED columns
-- carry the addressing; `heading` and `body` are the searchable surface.
-- openness: DERIVED -- a search index over prose that is already on disk in the
-- files it points at; every row is recomputed by re-reading them.
CREATE VIRTUAL TABLE IF NOT EXISTS doc_sections USING fts5 (
  owner_type UNINDEXED,
  owner_id UNINDEXED,
  file UNINDEXED,
  seq UNINDEXED,
  heading,
  level UNINDEXED,
  body,
  tokenize = 'porter unicode61'
);
-- **D42: THE DB STAMPS THE RECORD, AND THE APPLICATION NEVER SUPPLIES A TIME.**
-- `ts` carries a DEFAULT so the stamp is applied AS PART OF THE INSERT. A
-- caller that read a clock and then wrote the value would hold it across a
-- gap, so a retried, deferred or batched write would be stamped when it was
-- PREPARED rather than when it happened -- invisible by inspection, which is
-- this estate's recurring failure shape. A DEFAULT has no gap: the stamp and
-- the write are one operation.
-- The column is still WRITABLE, and that is not a loophole: restoring the
-- committed extract must carry each envelope's ORIGINAL time, which is a
-- different act from recording that something just happened.
-- **`ts` IS THIS TABLE'S RECORD TIMESTAMP AND THERE IS DELIBERATELY NO SECOND
-- COLUMN.** Stated rather than left as an absence, because a missing
-- measurement must present as a refusal and never as a measurement of nothing
-- -- an unexplained gap here reads as the oversight AC-02.8 was raised to fix
-- and gets re-audited. An event row is append-only and immutable, so it has no
-- `updated_at` to have: nothing ever updates it, and a column recording an act
-- that cannot happen is a guard that passes vacuously.
-- openness: carried by intent/events.jsonl
CREATE TABLE IF NOT EXISTS event_log (
  id TEXT PRIMARY KEY,
  ts TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  principal TEXT NOT NULL,
  project_id TEXT NOT NULL,
  op TEXT NOT NULL,
  subject_type TEXT NOT NULL,
  subject_id TEXT NOT NULL,
  payload TEXT NOT NULL
);
";

/// **The shape of [`DDL`], stamped into every store this binary creates**
/// (`PRAGMA user_version`). Bump it in the same commit as any DDL change --
/// `tests/store_schema_version.rs` fails if the DDL moves and this does not.
///
/// It exists because `CREATE TABLE IF NOT EXISTS` makes applying the DDL to an
/// existing database a NO-OP. Without a stamp, opening a store written by an
/// older binary SUCCEEDS, hands back a connection on the old shape, and defers
/// the failure to whichever query first names a column that is not there --
/// so the distance between "this is broken" and "you find out" is however long
/// it takes to run the right verb. Found by dc dogfooding on 2026-08-15, about
/// forty minutes after the criteria table changed shape underneath it:
///
/// ```text
/// error: could not read the committed canon
///   caused by: sqlite: no such column: state in SELECT id, text, kind, state FROM criteria ...
/// ```
///
/// **1 is the first stamped version, and the unstamped past is deliberately
/// not version 0 of anything.** Databases written before this stamp existed
/// carry `user_version = 0` and no record of which of the day's several shapes
/// they hold, so there is no state to migrate FROM. They are refused, by name,
/// rather than migrated on a guess -- see [`StoreError::SchemaUnstamped`].
pub const SCHEMA_VERSION: i32 = 3;

/// **The record-timestamp columns (AC-02.8, D42), named once.**
///
/// Every one is written by the database and never passed in by a caller. They
/// are a fact about THIS store rather than about the project, so they are
/// deliberately absent from the extract and correctly re-stamped by a rebuild.
///
/// Public and single-sourced because two readers need the same answer and a
/// hand-kept copy in either would rot: [`Store::derived_dump`] excludes them so
/// rebuild-identity compares modelled content, and `tests/record_timestamps.rs`
/// DISCOVERS the stamped columns from the DDL rather than listing them, so a
/// table added tomorrow is covered without anyone remembering to add it here.
pub const RECORD_TIMESTAMPS: &[&str] = &["created_at", "updated_at", "written_at"];

/// **The migration ladder: one rung per version step, applied in order.**
///
/// `MIGRATIONS ARE NORMAL` had a stamp and a refusal and no ladder, which was
/// deliberate sequencing (refusing with a remedy is the invariant; migrating
/// is the convenience) and stopped being enough the moment the DDL actually
/// moved. A store at 1 opened by this binary is now MIGRATED rather than
/// refused.
///
/// **A rung can only ever start at 1.** SQLite defaults `user_version` to 0,
/// so 0 is permanently the ABSENCE of a version rather than schema zero: there
/// is no state to migrate FROM, and those stores stay refused by name. The
/// stamp bought the future, not the past.
///
/// Each rung is `(to_version, sql)` and runs in one transaction with the
/// version bump, so an interrupted migration leaves the old version and the
/// old shape rather than a half-migrated store claiming the new one.
const MIGRATIONS: &[(i32, &str)] = &[(
  2,
  // 1 -> 2: `event_log.ts` gains a DEFAULT so the DATABASE stamps the row at
  // INSERT (D42). SQLite cannot alter an existing column's default, so the
  // table is rebuilt -- and every existing row keeps its original `ts`,
  // because the stamps already recorded are history and re-stamping them
  // would move the whole log to the moment of the upgrade.
  "CREATE TABLE event_log_v2 (
     id TEXT PRIMARY KEY,
     ts TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
     principal TEXT NOT NULL,
     project_id TEXT NOT NULL,
     op TEXT NOT NULL,
     subject_type TEXT NOT NULL,
     subject_id TEXT NOT NULL,
     payload TEXT NOT NULL
   );
   INSERT INTO event_log_v2 (id, ts, principal, project_id, op, subject_type, subject_id, payload)
     SELECT id, ts, principal, project_id, op, subject_type, subject_id, payload FROM event_log;
   DROP TABLE event_log;
   ALTER TABLE event_log_v2 RENAME TO event_log;",
), (
  3,
  // 2 -> 3: every table gains its DB-written record timestamp (AC-02.8).
  //
  // Rebuilt rather than ALTERed, and not by preference: SQLite refuses
  // `ADD COLUMN` for a NOT NULL column whose default is non-constant, and
  // `strftime(...)` is non-constant by definition. The alternative -- a
  // nullable column -- would ship a record timestamp that is allowed to be
  // absent, which is the measurement-of-nothing this criterion exists to stop.
  //
  // **EXISTING ROWS TAKE THE MIGRATION'S OWN STAMP, AND THAT IS NOT THE
  // RE-STAMPING RUNG 2 REFUSED TO DO.** There the column already held recorded
  // history and rewriting it would have moved the whole log to the moment of
  // the upgrade. Here there is no prior value to destroy: the column did not
  // exist, so nothing was ever recorded, and the honest answer to "when did
  // this store write this row" is the rebuild that is writing it now.
  //
  // **`event_log` is rebuilt again here, for PRECISION** -- `%S` to `%f`. Its
  // stamps are carried through unchanged; only the DEFAULT that future rows
  // will take moves. See the DDL for why a second is not fine enough.
  "CREATE TABLE threads_v3 (
     id TEXT PRIMARY KEY,
     title TEXT NOT NULL,
     slug TEXT,
     status TEXT NOT NULL,
     status_reason TEXT,
     created TEXT NOT NULL,
     completed TEXT,
     acceptance TEXT,
     objective TEXT NOT NULL,
     context TEXT NOT NULL,
     created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
     updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
   );
   INSERT INTO threads_v3 (id, title, slug, status, status_reason, created, completed, acceptance, objective, context)
     SELECT id, title, slug, status, status_reason, created, completed, acceptance, objective, context FROM threads;
   DROP TABLE threads;
   ALTER TABLE threads_v3 RENAME TO threads;

   CREATE TABLE related_v3 (
     thread_id TEXT NOT NULL REFERENCES threads (id) ON DELETE CASCADE,
     seq INTEGER NOT NULL,
     id TEXT NOT NULL,
     note TEXT,
     written_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
     PRIMARY KEY (thread_id, seq)
   );
   INSERT INTO related_v3 (thread_id, seq, id, note)
     SELECT thread_id, seq, id, note FROM related;
   DROP TABLE related;
   ALTER TABLE related_v3 RENAME TO related;

   CREATE TABLE wps_v3 (
     thread_id TEXT NOT NULL REFERENCES threads (id) ON DELETE CASCADE,
     seq INTEGER NOT NULL,
     title TEXT NOT NULL,
     scope TEXT NOT NULL,
     status TEXT NOT NULL,
     status_reason TEXT,
     objective TEXT NOT NULL,
     body TEXT NOT NULL,
     written_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
     PRIMARY KEY (thread_id, seq)
   );
   INSERT INTO wps_v3 (thread_id, seq, title, scope, status, status_reason, objective, body)
     SELECT thread_id, seq, title, scope, status, status_reason, objective, body FROM wps;
   DROP TABLE wps;
   ALTER TABLE wps_v3 RENAME TO wps;

   CREATE TABLE criteria_v3 (
     thread_id TEXT NOT NULL REFERENCES threads (id) ON DELETE CASCADE,
     id TEXT NOT NULL,
     text TEXT NOT NULL,
     kind TEXT NOT NULL,
     state TEXT NOT NULL,
     written_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
     PRIMARY KEY (thread_id, id)
   );
   INSERT INTO criteria_v3 (thread_id, id, text, kind, state)
     SELECT thread_id, id, text, kind, state FROM criteria;
   DROP TABLE criteria;
   ALTER TABLE criteria_v3 RENAME TO criteria;

   CREATE TABLE tests_v3 (
     thread_id TEXT NOT NULL REFERENCES threads (id) ON DELETE CASCADE,
     id TEXT NOT NULL,
     kind TEXT NOT NULL,
     file TEXT,
     prose TEXT,
     covers TEXT NOT NULL,
     status TEXT NOT NULL,
     note TEXT,
     legacy TEXT,
     written_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
     PRIMARY KEY (thread_id, id)
   );
   INSERT INTO tests_v3 (thread_id, id, kind, file, prose, covers, status, note, legacy)
     SELECT thread_id, id, kind, file, prose, covers, status, note, legacy FROM tests;
   DROP TABLE tests;
   ALTER TABLE tests_v3 RENAME TO tests;

   CREATE TABLE issues_v3 (
     number INTEGER PRIMARY KEY,
     slug TEXT NOT NULL,
     title TEXT NOT NULL,
     status TEXT NOT NULL,
     severity TEXT,
     created TEXT NOT NULL,
     closed TEXT,
     created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
     updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
   );
   INSERT INTO issues_v3 (number, slug, title, status, severity, created, closed)
     SELECT number, slug, title, status, severity, created, closed FROM issues;
   DROP TABLE issues;
   ALTER TABLE issues_v3 RENAME TO issues;

   CREATE TABLE file_index_v3 (
     path TEXT PRIMARY KEY,
     size INTEGER NOT NULL,
     mtime TEXT NOT NULL,
     sha256 TEXT NOT NULL,
     state TEXT NOT NULL,
     findings TEXT NOT NULL,
     created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
     updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
   );
   INSERT INTO file_index_v3 (path, size, mtime, sha256, state, findings)
     SELECT path, size, mtime, sha256, state, findings FROM file_index;
   DROP TABLE file_index;
   ALTER TABLE file_index_v3 RENAME TO file_index;

   CREATE TABLE event_log_v3 (
     id TEXT PRIMARY KEY,
     ts TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
     principal TEXT NOT NULL,
     project_id TEXT NOT NULL,
     op TEXT NOT NULL,
     subject_type TEXT NOT NULL,
     subject_id TEXT NOT NULL,
     payload TEXT NOT NULL
   );
   INSERT INTO event_log_v3 (id, ts, principal, project_id, op, subject_type, subject_id, payload)
     SELECT id, ts, principal, project_id, op, subject_type, subject_id, payload FROM event_log;
   DROP TABLE event_log;
   ALTER TABLE event_log_v3 RENAME TO event_log;",
)];

/// Which of the two write acts is happening (D42).
///
/// Named rather than a `bool`, because `write_event(conn, e, true)` at a call
/// site says nothing about which world it is in, and the two worlds are
/// "record that this is happening now" and "reinstate a record of something
/// that happened then". Getting them the wrong way round rewrites history to
/// the moment of the restore, silently.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Stamp {
  /// The DEFAULT fires: the database stamps the row as part of the INSERT.
  ByTheDatabase,
  /// The envelope's own `ts` is carried verbatim -- transport, not recording.
  CarriedFromTheExtract,
}

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
  #[error("sqlite: {0}")]
  Sqlite(#[from] rusqlite::Error),
  #[error("serialisation: {0}")]
  Serde(#[from] serde_json::Error),
  #[error("creating the runtime cache directory: {0}")]
  Cache(#[source] std::io::Error),
  /// The store holds a schema this binary does not speak. **Refused at open**,
  /// which is the whole point: the alternative is answering questions from a
  /// database whose shape disagrees with the queries.
  #[error("the runtime store holds schema version {found}; this build of intent speaks {expected}")]
  SchemaMismatch { found: i32, expected: i32 },
  /// The store predates schema versioning altogether.
  #[error("the runtime store predates schema versioning and does not record which shape it holds")]
  SchemaUnstamped,
  /// A migration rebuilt a table and left rows pointing at a parent that is no
  /// longer there. Foreign keys are off for the rebuild and re-checked inside
  /// the same transaction, so this is the check firing and rolling the rung
  /// back rather than the damage going unnoticed.
  #[error(
    "migrating the runtime store left {violations} row(s) referencing a parent that is not there"
  )]
  MigrationLeftDanglingRows { violations: i64 },
}

impl StoreError {
  /// What the operator should DO. Distinct per variant -- a remedy that fits
  /// two causes is telling the operator to guess which one they hit.
  ///
  /// The two schema variants are separated because the ACTION differs: one is
  /// "your build is out of step with your database", which is recoverable by
  /// moving either end, and the other is "nothing knows what this database
  /// is", which is not recoverable by the tool at all. Collapsing them would
  /// promise a migration for the case that cannot have one.
  pub fn remedy(&self) -> String {
    match self {
      Self::SchemaMismatch { found, expected } if found > expected => format!(
        "this store was written by a NEWER intent than the one you are running -- upgrade intent rather than migrating the store down; it holds version {found} and this build speaks {expected}"
      ),
      Self::SchemaMismatch { .. } => {
        "run `intent doctor` -- it names the store's version against this build's, and reports whether a migration for it has shipped".to_string()
      }
      Self::SchemaUnstamped => {
        // NO RECOVERY COMMAND, because there is none and inventing one is
        // worse than admitting it. The database was written on the day the
        // schema moved several times without a stamp, so its shape is not
        // knowable and a migration cannot be dispatched for it. What CAN be
        // said honestly is where the work is: the committed extract carries
        // everything that was ever synced out (D34), which for a project under
        // version control is a `git status` away from being checked.
        "this database cannot be migrated -- nothing recorded which shape it holds. Check what your committed extract carries before replacing it; anything never synced out of this store exists only here".to_string()
      }
      // CARRIES THE WARNING THE FACADE USED TO SHOW FOR EVERY STORE FAILURE,
      // because this is the variant it was written for: an unclassified
      // statement failure is the one where an operator starts reaching for
      // the file. The store is truth and the files are an extract that may be
      // older than it, so deleting it is not a reset -- it is the loss.
      Self::Sqlite(_) => {
        "the change was not made. Do NOT delete the store -- it is the source of truth, not a cache, and the committed extract may be older than it. Run `intent doctor` to inspect the estate".to_string()
      }
      Self::Serde(_) => {
        "a stored value could not be read back as its modelled type -- run `intent doctor`, and do not delete the store to clear it".to_string()
      }
      Self::Cache(_) => {
        "check that `intent/.cache/` is writable by you".to_string()
      }
      // The migration ran inside a transaction that has already rolled back,
      // so the store is still at its old version and its old shape -- which is
      // the recoverable case, and worth saying, because "migration failed"
      // reads as damage.
      Self::MigrationLeftDanglingRows { .. } => {
        "the migration was rolled back and the store is untouched at its previous version -- this is a defect in intent rather than in your data; report it with the version `intent doctor` prints".to_string()
      }
    }
  }
}

/// Read a model enum back from its stored wire spelling.
///
/// The mirror of [`enum_str`], and routed through serde for the same reason:
/// the wire names have ONE authority (the serde attributes on the model), so a
/// hand-written parse table here could disagree with the one that wrote them.
fn enum_from<T: serde::de::DeserializeOwned>(wire: &str) -> Result<T, StoreError> {
  Ok(serde_json::from_value(serde_json::Value::String(
    wire.to_string(),
  ))?)
}

pub struct Store {
  conn: Connection,
}

/// Everything ONE mutation changes, written in ONE transaction.
///
/// Grouped into a struct rather than passed as six arguments because the whole
/// point is that they are indivisible: under D01 as reversed, the DB is the
/// truth, so a mutation that recorded the entity and lost its envelope would
/// be an unaudited change to the source of truth -- and AC-04.5 requires the
/// envelope end to end. Before this they were three separate calls, one of
/// which did not open a transaction at all.
/// What the database stored for one thread's DOMAIN dates.
///
/// Handed back from the write rather than predicted before it, which is the
/// whole of D42 at this seam: the caller had no time to give, so the only way
/// it can know the date is to be told by the write that set it.
#[derive(Debug, Clone)]
pub struct ThreadDates {
  pub id: String,
  pub created: String,
  pub completed: Option<String>,
}

pub struct Mutation<'a> {
  pub threads: &'a [&'a Thread],
  pub issues: &'a [&'a Issue],
  pub removed_threads: &'a [String],
  pub removed_issues: &'a [u32],
  pub sections: &'a [DocSection],
  pub envelope: &'a Envelope,
}

impl Store {
  /// Open (creating if absent) the DB at `path`, set WAL + foreign keys, and
  /// **check the schema stamp before handing back a usable store**.
  pub fn open(path: &std::path::Path) -> Result<Self, StoreError> {
    // SQLite creates the FILE but not its directory, and `intent/.cache/` is
    // gitignored (D21) so it is absent on every fresh clone and every fresh
    // project. Without this, the first command in a new project fails with
    // "unable to open database file" -- a confusing message for a directory
    // the tool owns and can simply make.
    if let Some(parent) = path.parent() {
      std::fs::create_dir_all(parent).map_err(StoreError::Cache)?;
    }
    let conn = Connection::open(path)?;
    conn.pragma_update(None, "journal_mode", "WAL")?;
    Self::init(conn)
  }

  /// An in-memory store, for tests.
  pub fn open_in_memory() -> Result<Self, StoreError> {
    Self::init(Connection::open_in_memory()?)
  }

  /// Apply the schema and refuse a store whose shape this binary does not
  /// speak.
  ///
  /// **The stamp is written BEFORE the DDL, inside one transaction, and the
  /// order is not arbitrary.** A crash between the two must leave a state the
  /// next open can repair rather than one it must refuse. Stamp-then-DDL leaves
  /// `version = N` with tables missing, and the next open re-applies the
  /// (idempotent) DDL and completes the job. DDL-then-stamp would leave tables
  /// at `version = 0` -- indistinguishable from the unstamped past, and refused
  /// forever for a crash that cost nothing.
  fn init(mut conn: Connection) -> Result<Self, StoreError> {
    conn.pragma_update(None, "foreign_keys", "ON")?;
    let found: i32 = conn.pragma_query_value(None, "user_version", |row| row.get(0))?;

    match found {
      // A fresh database: nothing has been written and nothing can be lost.
      0 if !Self::has_tables(&conn)? => {
        let tx = conn.transaction()?;
        tx.pragma_update(None, "user_version", SCHEMA_VERSION)?;
        tx.execute_batch(DDL)?;
        tx.commit()?;
      }
      // Written before the stamp existed, so its shape is not knowable.
      0 => return Err(StoreError::SchemaUnstamped),
      v if v == SCHEMA_VERSION => {
        // Same shape. The apply is a genuine no-op here, and it is what
        // finishes an interrupted create -- see the ordering note above.
        conn.execute_batch(DDL)?;
      }
      // An OLDER store is migrated, not refused: migrations are normal.
      v if v < SCHEMA_VERSION => {
        Self::migrate(&mut conn, v)?;
        conn.execute_batch(DDL)?;
      }
      // A NEWER store is refused, and there is no rung that could help. The
      // shape was written by a binary this one has never heard of, so the
      // remedy is to move the TOOL forward, never the data back.
      found => {
        return Err(StoreError::SchemaMismatch {
          found,
          expected: SCHEMA_VERSION,
        });
      }
    }
    Ok(Self { conn })
  }

  /// Every table this database actually has, including FTS shadow tables.
  ///
  /// A read the store owns rather than a `Connection` handed out: intentsvcs is
  /// the sole owner of the DB (D06), and "what tables are there" is a question
  /// about the store, not a reason to lend out the connection that would let a
  /// caller answer any other question too.
  pub fn table_names(&self) -> Result<Vec<String>, StoreError> {
    let mut stmt = self
      .conn
      .prepare("SELECT name FROM sqlite_master WHERE type = 'table' ORDER BY name")?;
    let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
    Ok(rows.collect::<Result<Vec<String>, _>>()?)
  }

  /// Whether this database holds anything at all.
  ///
  /// `sqlite_master` rather than a probe for a known table name: the question
  /// is "has anyone written here", and asking after a specific table would
  /// answer "no" for every past shape that did not happen to contain it.
  /// Walk the ladder from `from` up to [`SCHEMA_VERSION`].
  ///
  /// Each rung runs with its version bump in ONE transaction. A crash between
  /// rungs leaves a store that is validly at some intermediate version, which
  /// the next open resumes from -- never a store stamped with a shape it does
  /// not have.
  /// **A rung may REBUILD a table, so foreign keys come off around the ladder**
  /// -- SQLite's own documented recipe for a schema change it cannot express as
  /// an `ALTER`. Three pragmas, and each is load-bearing:
  ///
  /// - `foreign_keys = OFF`, because a rebuild drops the parent while children
  ///   still reference it. **It has to be set OUTSIDE a transaction** -- inside
  ///   one it is silently a no-op, which would leave the guard looking applied
  ///   and doing nothing.
  /// - `legacy_alter_table = ON`, because modern `RENAME TO` re-parses every
  ///   table that references the one being renamed, and mid-rebuild those
  ///   references point at a table that momentarily does not exist.
  /// - `foreign_key_check` inside each rung's own transaction, so turning the
  ///   enforcement off cannot quietly leave a violation behind it. Off for the
  ///   rebuild is not off for the result.
  fn migrate(conn: &mut Connection, from: i32) -> Result<(), StoreError> {
    conn.pragma_update(None, "foreign_keys", "OFF")?;
    conn.pragma_update(None, "legacy_alter_table", "ON")?;
    let walked = Self::walk_ladder(conn, from);
    conn.pragma_update(None, "legacy_alter_table", "OFF")?;
    conn.pragma_update(None, "foreign_keys", "ON")?;
    walked
  }

  fn walk_ladder(conn: &mut Connection, from: i32) -> Result<(), StoreError> {
    for (to, sql) in MIGRATIONS {
      if *to <= from {
        continue;
      }
      let tx = conn.transaction()?;
      tx.execute_batch(sql)?;
      // **CHECKED INSIDE THE RUNG'S TRANSACTION, BEFORE THE VERSION MOVES.** A
      // migration that left dangling children is a corrupt store that opens
      // cleanly -- the failure class the schema stamp exists to stop. Checking
      // after the commit would report damage that had already landed; checking
      // here means the rung rolls back and the store stays validly at its
      // previous version and previous shape.
      let violations: i64 =
        tx.query_row("SELECT count(*) FROM pragma_foreign_key_check", [], |r| {
          r.get(0)
        })?;
      if violations > 0 {
        return Err(StoreError::MigrationLeftDanglingRows { violations });
      }
      tx.pragma_update(None, "user_version", to)?;
      tx.commit()?;
    }
    Ok(())
  }

  fn has_tables(conn: &Connection) -> Result<bool, rusqlite::Error> {
    conn
      .query_row(
        "SELECT count(*) FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%'",
        [],
        |row| row.get::<_, i64>(0),
      )
      .map(|n| n > 0)
  }

  /// Rebuild every derived table from canon: wipe and reload in one
  /// transaction. The event log is untouched (not derived, D15).
  /// Write ONE thread and its child rows inside an open transaction.
  ///
  /// THE ONLY PLACE A THREAD BECOMES ROWS. `rebuild` (disk -> db sync) and
  /// `apply_changes` (the mutation write path) both go through it, so the two
  /// cannot drift into disagreeing about what a thread looks like in the
  /// store -- which is the divergent-copy failure with a transaction wrapped
  /// round it.
  /// **The two doors, on the thread's DOMAIN dates** (D42) -- the same split
  /// `write_event` already makes, for the same reason.
  ///
  /// [`Stamp::ByTheDatabase`] is the CREATE door: an empty `created` means
  /// "this is happening now, and the database says when", so SQLite fills it
  /// inside the INSERT and the caller never holds a time. `completed` follows
  /// the same rule with a third state -- `None` stays null, `Some("")` is
  /// stamped, `Some(date)` is carried.
  ///
  /// [`Stamp::CarriedFromTheExtract`] is the RESTORE door: whatever the extract
  /// recorded is written verbatim. **Re-stamping here would move every thread's
  /// creation date to the moment someone cloned the repository, and every date
  /// would still look valid.**
  ///
  /// Both forms `RETURNING` what was actually stored, so the caller learns the
  /// value from the write rather than predicting it. `'now'` is constant within
  /// one SQL statement in SQLite, so `created` and `created_at` here are the
  /// same instant by construction rather than by luck.
  fn write_thread(
    tx: &rusqlite::Transaction<'_>,
    t: &Thread,
    stamp: Stamp,
  ) -> Result<(String, Option<String>), StoreError> {
    tx.execute("DELETE FROM tests WHERE thread_id = ?1", params![t.id])?;
    tx.execute("DELETE FROM criteria WHERE thread_id = ?1", params![t.id])?;
    tx.execute("DELETE FROM related WHERE thread_id = ?1", params![t.id])?;
    tx.execute("DELETE FROM wps WHERE thread_id = ?1", params![t.id])?;
    // **UPSERT, not delete-and-reinsert, and the record timestamps are the
    // whole reason** (AC-02.8). A thread has durable identity, so destroying
    // the row and building a new one would re-fire `created_at` on every
    // mutation -- a column recording the LATEST write while carrying the name
    // of the FIRST. The child rows above have no identity across writes (a
    // removed WP must vanish), which is why they keep the delete and take
    // `written_at` instead.
    //
    // `updated_at` moves DB-side in the conflict clause. It is not a trigger:
    // nothing in this store issues a bare UPDATE, so an `ON UPDATE` trigger
    // would never fire and would pass vacuously forever.
    // The only difference between the doors is how `created` and `completed`
    // reach the row: filled by SQLite when the caller has nothing, or carried
    // verbatim from the extract.
    let dates = match stamp {
      Stamp::ByTheDatabase => {
        "COALESCE(NULLIF(?6, ''), strftime('%Y-%m-%d', 'now')),
         CASE WHEN ?7 IS NULL THEN NULL
              WHEN ?7 = '' THEN strftime('%Y-%m-%d', 'now')
              ELSE ?7 END"
      }
      Stamp::CarriedFromTheExtract => "?6, ?7",
    };
    let stored = tx.query_row(
      &format!(
        "INSERT INTO threads (id, title, slug, status, status_reason, created, completed, acceptance, objective, context) VALUES (?1, ?2, ?3, ?4, ?5, {dates}, ?8, ?9, ?10)
         ON CONFLICT (id) DO UPDATE SET
           title = excluded.title,
           slug = excluded.slug,
           status = excluded.status,
           status_reason = excluded.status_reason,
           created = excluded.created,
           completed = excluded.completed,
           acceptance = excluded.acceptance,
           objective = excluded.objective,
           context = excluded.context,
           updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
         RETURNING created, completed"
      ),
      params![
        t.id,
        t.title,
        t.slug,
        enum_str(&t.status),
        t.status_reason,
        t.created,
        t.completed,
        t.acceptance.as_ref().map(enum_str),
        t.objective,
        t.context,
      ],
      |row| Ok((row.get::<_, String>(0)?, row.get::<_, Option<String>>(1)?)),
    )?;
    for (seq, r) in t.related.iter().enumerate() {
      tx.execute(
        "INSERT INTO related (thread_id, seq, id, note) VALUES (?1, ?2, ?3, ?4)",
        params![t.id, seq as i64, r.id, r.note],
      )?;
    }
    for wp in &t.wps {
      tx.execute(
        "INSERT INTO wps (thread_id, seq, title, scope, status, status_reason, objective, body) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
          t.id,
          wp.seq,
          wp.title,
          enum_str(&wp.scope),
          enum_str(&wp.status),
          wp.status_reason,
          wp.objective,
          wp.body
        ],
      )?;
    }
    for c in &t.criteria {
      tx.execute(
        "INSERT INTO criteria (thread_id, id, text, kind, state) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
          t.id,
          c.id,
          c.text,
          enum_str(&c.kind),
          serde_json::to_string(&c.state)?,
        ],
      )?;
    }
    for at in &t.tests {
      tx.execute(
        "INSERT INTO tests (thread_id, id, kind, file, prose, covers, status, note, legacy) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
          t.id,
          at.id,
          enum_str(&at.kind),
          at.file,
          at.prose,
          serde_json::to_string(&at.covers)?,
          enum_str(&at.status),
          at.note,
          at.legacy.as_ref().map(|l| l.raw.clone()),
        ],
      )?;
    }
    Ok(stored)
  }

  /// Write ONE issue inside an open transaction. Same Highlander reason as
  /// [`Store::write_thread`].
  fn write_issue(tx: &rusqlite::Transaction<'_>, i: &Issue) -> Result<(), StoreError> {
    // Upserted for the same reason as a thread: durable identity, so
    // `created_at` must fire once rather than on every write.
    tx.execute(
      "INSERT INTO issues (number, slug, title, status, severity, created, closed) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
       ON CONFLICT (number) DO UPDATE SET
         slug = excluded.slug,
         title = excluded.title,
         status = excluded.status,
         severity = excluded.severity,
         created = excluded.created,
         closed = excluded.closed,
         updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')",
      params![i.number, i.slug, i.title, enum_str(&i.status), i.severity, i.created, i.closed],
    )?;
    Ok(())
  }

  /// THE MUTATION WRITE PATH (D01 as reversed 2026-08-15): apply exactly the
  /// entities a mutation changed, transactionally.
  ///
  /// `rebuild` is deliberately NOT used here any more. It DELETEs the whole
  /// estate and re-inserts it, which was defensible while the DB was a
  /// rebuildable index of the files and is not defensible now that the DB is
  /// the truth: reloading truth from a derived artefact inverts the model, and
  /// it made every keystroke O(estate). `rebuild` survives unchanged as the
  /// disk -> db sync direction, which is the one place a wholesale reload is
  /// the correct operation.
  ///
  /// **Returns what the database actually stored for each thread's domain
  /// dates** (D42). A mutation that creates a thread hands in an empty
  /// `created`; SQLite fills it as part of the INSERT and the value comes back
  /// here, so the caller learns the date from the write instead of reading a
  /// clock and predicting it.
  pub fn commit_mutation(&mut self, change: Mutation<'_>) -> Result<Vec<ThreadDates>, StoreError> {
    let tx = self.conn.transaction()?;
    for id in change.removed_threads {
      tx.execute("DELETE FROM tests WHERE thread_id = ?1", params![id])?;
      tx.execute("DELETE FROM criteria WHERE thread_id = ?1", params![id])?;
      tx.execute("DELETE FROM related WHERE thread_id = ?1", params![id])?;
      tx.execute("DELETE FROM wps WHERE thread_id = ?1", params![id])?;
      tx.execute("DELETE FROM threads WHERE id = ?1", params![id])?;
    }
    for number in change.removed_issues {
      tx.execute("DELETE FROM issues WHERE number = ?1", params![number])?;
    }
    let mut dates = Vec::new();
    for t in change.threads {
      // The CREATE door: this write is the thing happening, so the database
      // stamps it.
      let (created, completed) = Self::write_thread(&tx, t, Stamp::ByTheDatabase)?;
      dates.push(ThreadDates {
        id: t.id.clone(),
        created,
        completed,
      });
    }
    for i in change.issues {
      Self::write_issue(&tx, i)?;
    }
    Self::write_doc_sections(&tx, change.sections)?;
    // The mutation's own event: the DB stamps it inside the same
    // transaction as the rows it describes (D42).
    Self::write_event(&tx, change.envelope, Stamp::ByTheDatabase)?;
    tx.commit()?;
    Ok(dates)
  }

  /// Rebuild the whole store from canon -- the DISK -> DB sync direction.
  ///
  /// Wholesale by design: this is the operation that makes the DB agree with
  /// the tree, so replacing everything is what it means. It is no longer on
  /// the mutation path (see [`Store::apply_changes`]).
  pub fn rebuild(&mut self, threads: &[Thread], issues: &[Issue]) -> Result<(), StoreError> {
    let tx = self.conn.transaction()?;
    tx.execute_batch("DELETE FROM tests; DELETE FROM criteria; DELETE FROM related; DELETE FROM wps; DELETE FROM threads; DELETE FROM issues;")?;
    for t in threads {
      // The RESTORE door: these dates were recorded before, and rebuilding a
      // store is not the project happening again.
      Self::write_thread(&tx, t, Stamp::CarriedFromTheExtract)?;
    }
    for i in issues {
      Self::write_issue(&tx, i)?;
    }
    tx.commit()?;
    Ok(())
  }

  /// Reconstruct the whole model FROM the store -- the read half of
  /// [`Store::rebuild`].
  ///
  /// **This is what makes the DB the daily driver** (hv, 2026-08-14: "all cli
  /// commands are going to go to the intentsvcs -- db route, not to/from the
  /// file versions"). Without it the store could only be written to, so every
  /// command had to re-parse every `thread.json` before it could answer
  /// anything, and the DB was a scratch index rather than the thing being
  /// queried.
  ///
  /// **Under D01 as reversed this is the ordinary read of truth, not a cache
  /// warmed from the files.** The paragraph that stood here said "committed
  /// canon is still the durable truth and the store is still rebuildable from
  /// it" -- true while the DB was an index, backwards now. Nothing is rebuilt
  /// on a read: `load_canon` returns what the DB holds, and the committed
  /// extract is written FROM it (D34).
  ///
  /// **The correctness property survives the reversal and gains a name.**
  /// Round-trip identity -- `rebuild` then `load_canon` returns exactly what
  /// went in -- is what makes the extract LOSSLESS, which is AC-02.6's openness
  /// requirement measured at one table instead of across the schema. Anything
  /// this drops is a fact that leaves the machine (D34) and does not come back.
  /// `store_round_trip` asserts it against the markup-bearing fixture rather
  /// than a tame one.
  pub fn load_canon(&self) -> Result<(Vec<Thread>, Vec<Issue>), StoreError> {
    let mut threads = Vec::new();
    let mut stmt = self.conn.prepare(
      "SELECT id, title, slug, status, status_reason, created, completed, acceptance, objective, context FROM threads ORDER BY id",
    )?;
    let rows = stmt.query_map([], |row| {
      Ok((
        row.get::<_, String>(0)?,
        row.get::<_, String>(1)?,
        row.get::<_, Option<String>>(2)?,
        row.get::<_, String>(3)?,
        row.get::<_, Option<String>>(4)?,
        row.get::<_, String>(5)?,
        row.get::<_, Option<String>>(6)?,
        row.get::<_, Option<String>>(7)?,
        row.get::<_, String>(8)?,
        row.get::<_, String>(9)?,
      ))
    })?;

    let mut shells = Vec::new();
    for row in rows {
      shells.push(row?);
    }

    for (
      id,
      title,
      slug,
      status,
      status_reason,
      created,
      completed,
      acceptance,
      objective,
      context,
    ) in shells
    {
      threads.push(Thread {
        schema: THREAD_SCHEMA.to_string(),
        related: self.related_of(&id)?,
        wps: self.wps_of(&id)?,
        criteria: self.criteria_of(&id)?,
        tests: self.tests_of(&id)?,
        id,
        title,
        slug,
        status: enum_from(&status)?,
        status_reason,
        created,
        completed,
        acceptance: acceptance.as_deref().map(enum_from).transpose()?,
        objective,
        context,
      });
    }

    let mut stmt = self.conn.prepare(
      "SELECT number, slug, title, status, severity, created, closed FROM issues ORDER BY number",
    )?;
    let issues = stmt
      .query_map([], |row| {
        Ok((
          row.get::<_, u32>(0)?,
          row.get::<_, String>(1)?,
          row.get::<_, String>(2)?,
          row.get::<_, String>(3)?,
          row.get::<_, Option<String>>(4)?,
          row.get::<_, String>(5)?,
          row.get::<_, Option<String>>(6)?,
        ))
      })?
      .collect::<Result<Vec<_>, _>>()?;

    let issues = issues
      .into_iter()
      .map(|(number, slug, title, status, severity, created, closed)| {
        Ok(Issue {
          schema: ISSUE_SCHEMA.to_string(),
          number,
          slug,
          title,
          status: enum_from(&status)?,
          severity,
          created,
          closed,
        })
      })
      .collect::<Result<Vec<_>, StoreError>>()?;

    Ok((threads, issues))
  }

  fn related_of(&self, thread: &str) -> Result<Vec<Related>, StoreError> {
    let mut stmt = self
      .conn
      .prepare("SELECT id, note FROM related WHERE thread_id = ?1 ORDER BY seq")?;
    let rows = stmt.query_map(params![thread], |row| {
      Ok(Related {
        id: row.get(0)?,
        note: row.get(1)?,
      })
    })?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
  }

  fn wps_of(&self, thread: &str) -> Result<Vec<WorkPackage>, StoreError> {
    let mut stmt = self
      .conn
      .prepare("SELECT seq, title, scope, status, status_reason, objective, body FROM wps WHERE thread_id = ?1 ORDER BY seq")?;
    let raw = stmt
      .query_map(params![thread], |row| {
        Ok((
          row.get::<_, u32>(0)?,
          row.get::<_, String>(1)?,
          row.get::<_, String>(2)?,
          row.get::<_, String>(3)?,
          row.get::<_, Option<String>>(4)?,
          row.get::<_, String>(5)?,
          row.get::<_, String>(6)?,
        ))
      })?
      .collect::<Result<Vec<_>, _>>()?;
    raw
      .into_iter()
      .map(
        |(seq, title, scope, status, status_reason, objective, body)| {
          Ok(WorkPackage {
            seq,
            title,
            scope: enum_from(&scope)?,
            status: enum_from(&status)?,
            status_reason,
            objective,
            body,
          })
        },
      )
      .collect()
  }

  fn criteria_of(&self, thread: &str) -> Result<Vec<Criterion>, StoreError> {
    let mut stmt = self
      .conn
      .prepare("SELECT id, text, kind, state FROM criteria WHERE thread_id = ?1 ORDER BY rowid")?;
    let raw = stmt
      .query_map(params![thread], |row| {
        Ok((
          row.get::<_, String>(0)?,
          row.get::<_, String>(1)?,
          row.get::<_, String>(2)?,
          row.get::<_, String>(3)?,
        ))
      })?
      .collect::<Result<Vec<_>, _>>()?;
    raw
      .into_iter()
      .map(|(id, text, kind, state)| {
        Ok(Criterion {
          id,
          text,
          kind: enum_from(&kind)?,
          state: serde_json::from_str(&state)?,
        })
      })
      .collect()
  }

  fn tests_of(&self, thread: &str) -> Result<Vec<AcceptanceTest>, StoreError> {
    let mut stmt = self.conn.prepare(
      "SELECT id, kind, file, prose, covers, status, note, legacy FROM tests WHERE thread_id = ?1 ORDER BY rowid",
    )?;
    let raw = stmt
      .query_map(params![thread], |row| {
        Ok((
          row.get::<_, String>(0)?,
          row.get::<_, String>(1)?,
          row.get::<_, Option<String>>(2)?,
          row.get::<_, Option<String>>(3)?,
          row.get::<_, String>(4)?,
          row.get::<_, String>(5)?,
          row.get::<_, Option<String>>(6)?,
          row.get::<_, Option<String>>(7)?,
        ))
      })?
      .collect::<Result<Vec<_>, _>>()?;
    raw
      .into_iter()
      .map(|(id, kind, file, prose, covers, status, note, legacy)| {
        Ok(AcceptanceTest {
          id,
          kind: enum_from(&kind)?,
          file,
          prose,
          covers: serde_json::from_str(&covers)?,
          status: enum_from(&status)?,
          note,
          legacy: legacy.map(|raw| Legacy { raw }),
        })
      })
      .collect()
  }

  /// Every prose section in the index, in a total order.
  pub fn doc_sections(&self) -> Result<Vec<DocSection>, StoreError> {
    self.doc_sections_query(
      "SELECT owner_type, owner_id, file, seq, heading, level, body FROM doc_sections ORDER BY file, seq",
      [],
    )
  }

  /// **D35's SNAPSHOT: a byte-image of the store, taken through SQLite.**
  ///
  /// `VACUUM INTO`, and the choice is the whole arm rather than a preference
  /// (AC-03.10a). The store opens in WAL mode, so a committed transaction
  /// lives in `intent.db-wal` until something checkpoints it -- and a file
  /// copy of `intent.db` alone silently omits every such transaction. Measured
  /// on this shape: a live store with 50 rows yields 50 through `VACUUM INTO`
  /// and **0** through a naive copy, and the bad copy OPENS CLEANLY and
  /// reports no error. `fs::copy`, `cp` and a directory tar are all defects
  /// here, not slower alternatives.
  ///
  /// SQLite writes the destination itself and REFUSES a path that already
  /// exists, which is the behaviour we want: a snapshot that silently replaced
  /// an earlier one would make retention a lie. The caller creates the parent
  /// directory; nothing else about the destination is this method's business,
  /// because the layout under `.backup/` belongs to whoever owns that
  /// namespace.
  ///
  /// **What it is NOT for.** A snapshot is restorable only into a binary that
  /// speaks its schema. It is same-schema rollback, never the recovery path
  /// for a store an upgraded binary refuses -- restoring a snapshot from
  /// before a schema change reinstates the schema you were escaping. The
  /// recovery path for that is the committed extract (D35, as vc sharpened it).
  pub fn snapshot_into(&self, dest: &std::path::Path) -> Result<(), StoreError> {
    self
      .conn
      .execute("VACUUM INTO ?1", [dest.to_string_lossy().as_ref()])?;
    Ok(())
  }

  /// How many prose sections the index holds.
  ///
  /// **A COUNT rather than `doc_sections().len()`**, because the only caller
  /// runs it on the empty-result path of a search: loading every section's body
  /// to discover there are none would make the answer most expensive exactly
  /// when it is least informative.
  ///
  /// It exists so that "no hits" can be told apart from "nothing to hit". An
  /// unpopulated index answers every query the same way a genuine miss does,
  /// and a caller cannot tell those apart without asking this question
  /// (AC-06.4).
  pub fn doc_section_count(&self) -> Result<usize, StoreError> {
    let n: i64 = self
      .conn
      .query_row("SELECT count(*) FROM doc_sections", [], |row| row.get(0))?;
    Ok(n as usize)
  }

  /// Append one envelope to the event log.
  /// Record that something just happened. **The DB assigns the time** (D42);
  /// the stamp it assigned is returned, because the caller has no other way to
  /// learn it and must never compute it.
  pub fn append_event(&self, e: &Envelope) -> Result<String, StoreError> {
    Self::write_event(&self.conn, e, Stamp::ByTheDatabase)
  }

  /// Take an envelope back from the committed extract, carrying the time it
  /// was originally written with.
  ///
  /// **A different act from [`Store::append_event`], and the difference is the
  /// whole of D42.** Recording that something happens NOW is the database's
  /// job. Reinstating a record of something that happened THEN is transport,
  /// and re-stamping it would rewrite history to the moment of the restore --
  /// turning a clone of yesterday's extract into a log that claims everything
  /// happened today.
  pub fn restore_event(&self, e: &Envelope) -> Result<String, StoreError> {
    Self::write_event(&self.conn, e, Stamp::CarriedFromTheExtract)
  }

  /// THE ONLY PLACE AN ENVELOPE BECOMES A ROW. Takes anything that derefs to a
  /// `Connection`, so the standalone append and the one inside a mutation's
  /// transaction are the same code.
  fn write_event(
    conn: &rusqlite::Connection,
    e: &Envelope,
    stamp: Stamp,
  ) -> Result<String, StoreError> {
    let payload = serde_json::to_string(&e.payload)?;
    // ONE insert in two forms, and the only difference is whether `ts` is
    // named. Omitting the column is what lets the DEFAULT fire, which is the
    // mechanism D42 asks for -- there is no application-side expression to
    // get wrong, and `RETURNING` reads back what the database actually wrote
    // rather than what we hoped it would.
    let ts = match stamp {
      Stamp::ByTheDatabase => conn.query_row(
        "INSERT INTO event_log (id, principal, project_id, op, subject_type, subject_id, payload) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7) RETURNING ts",
        params![e.id, e.principal, e.project_id, e.op, e.subject.kind, e.subject.id, payload],
        |row| row.get(0),
      )?,
      Stamp::CarriedFromTheExtract => conn.query_row(
        "INSERT INTO event_log (id, ts, principal, project_id, op, subject_type, subject_id, payload) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8) RETURNING ts",
        params![e.id, e.ts, e.principal, e.project_id, e.op, e.subject.kind, e.subject.id, payload],
        |row| row.get(0),
      )?,
    };
    Ok(ts)
  }

  /// Every envelope, oldest first.
  ///
  /// Ordered by id rather than by `ts`: a ULID is lexically sortable by its
  /// own timestamp prefix, so it gives a total order even for two events
  /// minted inside the same millisecond -- which `ts` alone does not.
  pub fn events(&self) -> Result<Vec<Envelope>, StoreError> {
    let mut stmt = self.conn.prepare(
      "SELECT id, ts, principal, project_id, op, subject_type, subject_id, payload FROM event_log ORDER BY id",
    )?;
    let rows = stmt.query_map([], |row| {
      Ok((
        row.get::<_, String>(0)?,
        row.get::<_, String>(1)?,
        row.get::<_, String>(2)?,
        row.get::<_, String>(3)?,
        row.get::<_, String>(4)?,
        row.get::<_, String>(5)?,
        row.get::<_, String>(6)?,
        row.get::<_, String>(7)?,
      ))
    })?;
    let mut out = Vec::new();
    for row in rows {
      let (id, ts, principal, project_id, op, subject_type, subject_id, payload) = row?;
      out.push(Envelope {
        id,
        ts,
        principal,
        project_id,
        op,
        subject: crate::event::Subject {
          kind: subject_type,
          id: subject_id,
        },
        payload: serde_json::from_str(&payload)?,
      });
    }
    Ok(out)
  }

  /// A deterministic, ordered dump of the DERIVED tables, for equality
  /// checks (the D01 rebuild-identity invariant). Excludes the event log,
  /// which is not derived.
  ///
  /// **It was called `snapshot` and the name is now spoken for.** D35 gives
  /// "snapshot" a precise meaning -- a byte-image of the store at a schema,
  /// good for same-schema rollback and nothing else -- and this is a LOGICAL
  /// dump of six tables in JSON, which is a different object with different
  /// properties: it survives a schema change, it cannot be restored from, and
  /// it deliberately omits data. Two referents for one word on one type is how
  /// a reader connects a ratified decision to the wrong method.
  pub fn derived_dump(&self) -> Result<serde_json::Value, StoreError> {
    let mut out = serde_json::Map::new();
    for table in ["threads", "related", "wps", "criteria", "tests", "issues"] {
      out.insert(table.to_string(), self.dump_table(table)?);
    }
    Ok(serde_json::Value::Object(out))
  }

  // -------------------------------------------------------------------------
  // The file index (sync) and doc sections (prose ingest)
  //
  // Both are DB-only and derived from the WORKING TREE rather than from canon,
  // so `rebuild` leaves them alone and `snapshot` excludes them: they answer
  // "what is on disk right now", which is a different question from "what does
  // the committed canon say", and conflating the two is how a stale index gets
  // mistaken for truth.
  // -------------------------------------------------------------------------

  /// Replace the whole file index in one transaction.
  ///
  /// **Delete-missing then upsert-present, rather than wipe-and-reload**
  /// (AC-02.8). A path has durable identity across scans, so wiping the table
  /// would re-fire `created_at` on every sync and the column would silently
  /// mean `updated_at`. The observable content is identical either way -- what
  /// changes is whether "when did this store first index this path" survives
  /// the next scan.
  pub fn replace_file_index(&mut self, entries: &[FileEntry]) -> Result<(), StoreError> {
    let tx = self.conn.transaction()?;
    let keep = serde_json::to_string(&entries.iter().map(|e| &e.path).collect::<Vec<_>>())?;
    tx.execute(
      "DELETE FROM file_index WHERE path NOT IN (SELECT value FROM json_each(?1))",
      params![keep],
    )?;
    for e in entries {
      tx.execute(
        "INSERT INTO file_index (path, size, mtime, sha256, state, findings) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
         ON CONFLICT (path) DO UPDATE SET
           size = excluded.size,
           mtime = excluded.mtime,
           sha256 = excluded.sha256,
           state = excluded.state,
           findings = excluded.findings,
           updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')",
        params![
          e.path,
          e.size as i64,
          e.mtime,
          e.sha256,
          enum_str(&e.state),
          serde_json::to_string(&e.findings)?,
        ],
      )?;
    }
    tx.commit()?;
    Ok(())
  }

  /// Every indexed file, ordered by path.
  pub fn file_index(&self) -> Result<Vec<FileEntry>, StoreError> {
    let mut stmt = self
      .conn
      .prepare("SELECT path, size, mtime, sha256, state, findings FROM file_index ORDER BY path")?;
    let rows = stmt.query_map([], |row| {
      Ok((
        row.get::<_, String>(0)?,
        row.get::<_, i64>(1)?,
        row.get::<_, String>(2)?,
        row.get::<_, String>(3)?,
        row.get::<_, String>(4)?,
        row.get::<_, String>(5)?,
      ))
    })?;
    let mut out = Vec::new();
    for row in rows {
      let (path, size, mtime, sha256, state, findings) = row?;
      out.push(FileEntry {
        path,
        size: size as u64,
        mtime,
        sha256,
        state: serde_json::from_value(serde_json::Value::String(state))?,
        findings: serde_json::from_str(&findings)?,
      });
    }
    Ok(out)
  }

  /// Replace the whole prose index in one transaction.
  pub fn replace_doc_sections(&mut self, sections: &[DocSection]) -> Result<(), StoreError> {
    let tx = self.conn.transaction()?;
    Self::write_doc_sections(&tx, sections)?;
    tx.commit()?;
    Ok(())
  }

  /// THE ONLY PLACE SECTIONS BECOME ROWS, for the same Highlander reason as
  /// [`Store::write_thread`].
  fn write_doc_sections(
    conn: &rusqlite::Connection,
    sections: &[DocSection],
  ) -> Result<(), StoreError> {
    conn.execute("DELETE FROM doc_sections", [])?;
    for s in sections {
      conn.execute(
        "INSERT INTO doc_sections (owner_type, owner_id, file, seq, heading, level, body) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
          s.owner_type,
          s.owner_id,
          s.file,
          s.seq as i64,
          s.heading,
          s.level as i64,
          s.body,
        ],
      )?;
    }
    Ok(())
  }

  /// Every section of one file, in document order.
  pub fn doc_sections_for(&self, file: &str) -> Result<Vec<DocSection>, StoreError> {
    self.doc_sections_query(
      "SELECT owner_type, owner_id, file, seq, heading, level, body FROM doc_sections WHERE file = ?1 ORDER BY seq",
      params![file],
    )
  }

  /// Full-text search across headings and bodies -- what `intent search` runs
  /// (design.md). Results are ordered by FTS relevance, then by address so the
  /// ordering is total rather than merely mostly-determined.
  pub fn search(&self, query: &str) -> Result<Vec<DocSection>, StoreError> {
    self.doc_sections_query(
      "SELECT owner_type, owner_id, file, seq, heading, level, body FROM doc_sections WHERE doc_sections MATCH ?1 ORDER BY rank, file, seq",
      params![query],
    )
  }

  fn doc_sections_query(
    &self,
    sql: &str,
    args: impl rusqlite::Params,
  ) -> Result<Vec<DocSection>, StoreError> {
    let mut stmt = self.conn.prepare(sql)?;
    let rows = stmt.query_map(args, |row| {
      Ok(DocSection {
        owner_type: row.get(0)?,
        owner_id: row.get(1)?,
        file: row.get(2)?,
        seq: row.get::<_, i64>(3)? as u32,
        heading: row.get(4)?,
        level: row.get::<_, i64>(5)? as u8,
        body: row.get(6)?,
      })
    })?;
    let mut out = Vec::new();
    for row in rows {
      out.push(row?);
    }
    Ok(out)
  }

  fn dump_table(&self, table: &str) -> Result<serde_json::Value, StoreError> {
    // Table names come from the fixed list in `snapshot`, never from input.
    let mut stmt = self
      .conn
      .prepare(&format!("SELECT * FROM {table} ORDER BY 1, 2"))?;
    let names: Vec<String> = stmt.column_names().iter().map(|s| s.to_string()).collect();
    let rows = stmt.query_map([], |row| {
      let mut obj = serde_json::Map::new();
      for (idx, name) in names.iter().enumerate() {
        // **RECORD TIMESTAMPS ARE EXCLUDED, AND THE ALTERNATIVE IS A FLAKY
        // TEST RATHER THAN A FAILING ONE.** This dump answers "is the modelled
        // content identical", which is what rebuild-identity and the openness
        // round trip assert. A record timestamp is per-machine and re-stamped
        // on rebuild BY DESIGN, so including it makes those properties false
        // by construction -- and at one-second granularity two rebuilds inside
        // the same test usually land in the same second, so it would pass on
        // this machine and fail on a slow one. Excluded here, once, rather
        // than worked around at each assertion.
        if RECORD_TIMESTAMPS.contains(&name.as_str()) {
          continue;
        }
        let value = match row.get_ref(idx)? {
          rusqlite::types::ValueRef::Null => serde_json::Value::Null,
          rusqlite::types::ValueRef::Integer(i) => json!(i),
          rusqlite::types::ValueRef::Real(f) => json!(f),
          rusqlite::types::ValueRef::Text(t) => json!(String::from_utf8_lossy(t)),
          rusqlite::types::ValueRef::Blob(b) => json!(b.to_vec()),
        };
        obj.insert(name.clone(), value);
      }
      Ok(serde_json::Value::Object(obj))
    })?;
    let mut list = Vec::new();
    for row in rows {
      list.push(row?);
    }
    Ok(serde_json::Value::Array(list))
  }
}
