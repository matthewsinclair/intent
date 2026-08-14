//! The runtime store (design.md D01): a per-project SQLite DB, derived from
//! committed canon and rebuilt from it at any time. `rm` of the DB file is
//! always safe, so there are NO DB migrations, ever -- a schema bump deletes
//! and rebuilds.
//!
//! Derived tables (threads, wps, criteria, tests, issues) are wiped and
//! reloaded by [`Store::rebuild`]. The event log is the deliberate exception
//! (D15): append-only, not derived, and losable by design.

use rusqlite::{Connection, params};
use serde_json::json;

use crate::event::Envelope;
use crate::model::{Issue, Thread, enum_str};

/// The DDL face. Applied verbatim on open; committed under `schema/ddl.sql`
/// by the faces machinery and drift-checked against this constant.
pub const DDL: &str = "\
-- Intent v3 runtime store (GENERATED FACE -- the master is
-- crates/intentsvcs/src/store.rs; regenerate via INTENT_BLESS, never edit).
-- Derived from committed canon; rebuilt at any time; no migrations ever.
CREATE TABLE IF NOT EXISTS threads (
  id TEXT PRIMARY KEY,
  title TEXT NOT NULL,
  slug TEXT,
  status TEXT NOT NULL,
  created TEXT NOT NULL,
  completed TEXT,
  acceptance TEXT
);
CREATE TABLE IF NOT EXISTS wps (
  thread_id TEXT NOT NULL REFERENCES threads (id) ON DELETE CASCADE,
  seq INTEGER NOT NULL,
  title TEXT NOT NULL,
  scope TEXT NOT NULL,
  status TEXT NOT NULL,
  PRIMARY KEY (thread_id, seq)
);
CREATE TABLE IF NOT EXISTS criteria (
  thread_id TEXT NOT NULL REFERENCES threads (id) ON DELETE CASCADE,
  id TEXT NOT NULL,
  text TEXT NOT NULL,
  kind TEXT NOT NULL,
  scope TEXT NOT NULL,
  evidence TEXT,
  satisfied INTEGER,
  PRIMARY KEY (thread_id, id)
);
CREATE TABLE IF NOT EXISTS tests (
  thread_id TEXT NOT NULL REFERENCES threads (id) ON DELETE CASCADE,
  id TEXT NOT NULL,
  kind TEXT NOT NULL,
  file TEXT,
  prose TEXT,
  covers TEXT NOT NULL,
  status TEXT NOT NULL,
  note TEXT,
  PRIMARY KEY (thread_id, id)
);
CREATE TABLE IF NOT EXISTS issues (
  number INTEGER PRIMARY KEY,
  slug TEXT NOT NULL,
  title TEXT NOT NULL,
  status TEXT NOT NULL,
  severity TEXT,
  created TEXT NOT NULL,
  closed TEXT
);
CREATE TABLE IF NOT EXISTS event_log (
  id TEXT PRIMARY KEY,
  ts TEXT NOT NULL,
  principal TEXT NOT NULL,
  project_id TEXT NOT NULL,
  op TEXT NOT NULL,
  subject_type TEXT NOT NULL,
  subject_id TEXT NOT NULL,
  payload TEXT NOT NULL
);
";

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
  #[error("sqlite: {0}")]
  Sqlite(#[from] rusqlite::Error),
  #[error("serialisation: {0}")]
  Serde(#[from] serde_json::Error),
}

pub struct Store {
  conn: Connection,
}

impl Store {
  /// Open (creating if absent) the DB at `path`, apply the DDL, set WAL +
  /// foreign keys. Reopening an existing DB is a no-op apply (IF NOT EXISTS).
  pub fn open(path: &std::path::Path) -> Result<Self, StoreError> {
    let conn = Connection::open(path)?;
    conn.pragma_update(None, "journal_mode", "WAL")?;
    Self::init(conn)
  }

  /// An in-memory store, for tests.
  pub fn open_in_memory() -> Result<Self, StoreError> {
    Self::init(Connection::open_in_memory()?)
  }

  fn init(conn: Connection) -> Result<Self, StoreError> {
    conn.pragma_update(None, "foreign_keys", "ON")?;
    conn.execute_batch(DDL)?;
    Ok(Self { conn })
  }

  /// Rebuild every derived table from canon: wipe and reload in one
  /// transaction. The event log is untouched (not derived, D15).
  pub fn rebuild(&mut self, threads: &[Thread], issues: &[Issue]) -> Result<(), StoreError> {
    let tx = self.conn.transaction()?;
    tx.execute_batch("DELETE FROM tests; DELETE FROM criteria; DELETE FROM wps; DELETE FROM threads; DELETE FROM issues;")?;

    for t in threads {
      tx.execute(
        "INSERT INTO threads (id, title, slug, status, created, completed, acceptance) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
          t.id,
          t.title,
          t.slug,
          enum_str(&t.status),
          t.created,
          t.completed,
          t.acceptance.as_ref().map(enum_str),
        ],
      )?;
      for wp in &t.wps {
        tx.execute(
          "INSERT INTO wps (thread_id, seq, title, scope, status) VALUES (?1, ?2, ?3, ?4, ?5)",
          params![
            t.id,
            wp.seq,
            wp.title,
            enum_str(&wp.scope),
            enum_str(&wp.status)
          ],
        )?;
      }
      for c in &t.criteria {
        tx.execute(
          "INSERT INTO criteria (thread_id, id, text, kind, scope, evidence, satisfied) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
          params![
            t.id,
            c.id,
            c.text,
            enum_str(&c.kind),
            serde_json::to_string(&c.scope)?,
            c.evidence,
            c.satisfied,
          ],
        )?;
      }
      for at in &t.tests {
        tx.execute(
          "INSERT INTO tests (thread_id, id, kind, file, prose, covers, status, note) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
          params![
            t.id,
            at.id,
            enum_str(&at.kind),
            at.file,
            at.prose,
            serde_json::to_string(&at.covers)?,
            enum_str(&at.status),
            at.note,
          ],
        )?;
      }
    }
    for i in issues {
      tx.execute(
        "INSERT INTO issues (number, slug, title, status, severity, created, closed) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![i.number, i.slug, i.title, enum_str(&i.status), i.severity, i.created, i.closed],
      )?;
    }
    tx.commit()?;
    Ok(())
  }

  /// Append one envelope to the event log.
  pub fn append_event(&self, e: &Envelope) -> Result<(), StoreError> {
    self.conn.execute(
      "INSERT INTO event_log (id, ts, principal, project_id, op, subject_type, subject_id, payload) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
      params![
        e.id,
        e.ts,
        e.principal,
        e.project_id,
        e.op,
        e.subject.kind,
        e.subject.id,
        serde_json::to_string(&e.payload)?,
      ],
    )?;
    Ok(())
  }

  /// A deterministic, ordered dump of the DERIVED tables, for equality
  /// checks (the D01 rebuild-identity invariant). Excludes the event log,
  /// which is not derived.
  pub fn snapshot(&self) -> Result<serde_json::Value, StoreError> {
    let mut out = serde_json::Map::new();
    for table in ["threads", "wps", "criteria", "tests", "issues"] {
      out.insert(table.to_string(), self.dump_table(table)?);
    }
    Ok(serde_json::Value::Object(out))
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
