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
-- crates/intentsvcs/src/store.rs; regenerate via INTENT_BLESS, never edit).
-- Derived from committed canon; rebuilt at any time; no migrations ever.
CREATE TABLE IF NOT EXISTS threads (
  id TEXT PRIMARY KEY,
  title TEXT NOT NULL,
  slug TEXT,
  status TEXT NOT NULL,
  created TEXT NOT NULL,
  completed TEXT,
  acceptance TEXT,
  objective TEXT NOT NULL,
  context TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS related (
  thread_id TEXT NOT NULL REFERENCES threads (id) ON DELETE CASCADE,
  seq INTEGER NOT NULL,
  id TEXT NOT NULL,
  note TEXT,
  PRIMARY KEY (thread_id, seq)
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
  legacy TEXT,
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
-- The sync engine's git-style index (data-model.md). DB-only and derived from
-- the working tree, not from canon, so `rebuild` does not touch it.
-- `findings` is a JSON array; `state` is clean | changed | unparsed.
CREATE TABLE IF NOT EXISTS file_index (
  path TEXT PRIMARY KEY,
  size INTEGER NOT NULL,
  mtime TEXT NOT NULL,
  sha256 TEXT NOT NULL,
  state TEXT NOT NULL,
  findings TEXT NOT NULL
);
-- Prose ingest (data-model.md): bodies stored VERBATIM, never modelled, and
-- FTS5-indexed to power `intent search`. One table, not an external-content
-- pair: the store is rebuilt wholesale from canon, so a shadow content table
-- plus triggers would add a drift hazard to buy nothing. UNINDEXED columns
-- carry the addressing; `heading` and `body` are the searchable surface.
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
  #[error("creating the runtime cache directory: {0}")]
  Cache(#[source] std::io::Error),
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

impl Store {
  /// Open (creating if absent) the DB at `path`, apply the DDL, set WAL +
  /// foreign keys. Reopening an existing DB is a no-op apply (IF NOT EXISTS).
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

  fn init(conn: Connection) -> Result<Self, StoreError> {
    conn.pragma_update(None, "foreign_keys", "ON")?;
    conn.execute_batch(DDL)?;
    Ok(Self { conn })
  }

  /// Rebuild every derived table from canon: wipe and reload in one
  /// transaction. The event log is untouched (not derived, D15).
  pub fn rebuild(&mut self, threads: &[Thread], issues: &[Issue]) -> Result<(), StoreError> {
    let tx = self.conn.transaction()?;
    tx.execute_batch("DELETE FROM tests; DELETE FROM criteria; DELETE FROM related; DELETE FROM wps; DELETE FROM threads; DELETE FROM issues;")?;

    for t in threads {
      tx.execute(
        "INSERT INTO threads (id, title, slug, status, created, completed, acceptance, objective, context) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
          t.id,
          t.title,
          t.slug,
          enum_str(&t.status),
          t.created,
          t.completed,
          t.acceptance.as_ref().map(enum_str),
          t.objective,
          t.context,
        ],
      )?;
      for (seq, r) in t.related.iter().enumerate() {
        tx.execute(
          "INSERT INTO related (thread_id, seq, id, note) VALUES (?1, ?2, ?3, ?4)",
          params![t.id, seq as i64, r.id, r.note],
        )?;
      }
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
  /// It does NOT weaken D01. Committed canon is still the durable truth and
  /// the store is still rebuildable from it; what changes is how often we pay
  /// for that rebuild -- on a detected change, rather than on every
  /// invocation.
  ///
  /// The correctness property is round-trip identity: `rebuild` then
  /// `load_canon` must return exactly what went in. Anything this drops is a
  /// question the DB would answer differently from the files, which is the one
  /// failure that would make the whole arrangement unsafe. `store_round_trip`
  /// asserts it against the markup-bearing fixture rather than a tame one.
  pub fn load_canon(&self) -> Result<(Vec<Thread>, Vec<Issue>), StoreError> {
    let mut threads = Vec::new();
    let mut stmt = self.conn.prepare(
      "SELECT id, title, slug, status, created, completed, acceptance, objective, context FROM threads ORDER BY id",
    )?;
    let rows = stmt.query_map([], |row| {
      Ok((
        row.get::<_, String>(0)?,
        row.get::<_, String>(1)?,
        row.get::<_, Option<String>>(2)?,
        row.get::<_, String>(3)?,
        row.get::<_, String>(4)?,
        row.get::<_, Option<String>>(5)?,
        row.get::<_, Option<String>>(6)?,
        row.get::<_, String>(7)?,
        row.get::<_, String>(8)?,
      ))
    })?;

    let mut shells = Vec::new();
    for row in rows {
      shells.push(row?);
    }

    for (id, title, slug, status, created, completed, acceptance, objective, context) in shells {
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
      .prepare("SELECT seq, title, scope, status FROM wps WHERE thread_id = ?1 ORDER BY seq")?;
    let raw = stmt
      .query_map(params![thread], |row| {
        Ok((
          row.get::<_, u32>(0)?,
          row.get::<_, String>(1)?,
          row.get::<_, String>(2)?,
          row.get::<_, String>(3)?,
        ))
      })?
      .collect::<Result<Vec<_>, _>>()?;
    raw
      .into_iter()
      .map(|(seq, title, scope, status)| {
        Ok(WorkPackage {
          seq,
          title,
          scope: enum_from(&scope)?,
          status: enum_from(&status)?,
        })
      })
      .collect()
  }

  fn criteria_of(&self, thread: &str) -> Result<Vec<Criterion>, StoreError> {
    let mut stmt = self.conn.prepare(
      "SELECT id, text, kind, scope, evidence, satisfied FROM criteria WHERE thread_id = ?1 ORDER BY rowid",
    )?;
    let raw = stmt
      .query_map(params![thread], |row| {
        Ok((
          row.get::<_, String>(0)?,
          row.get::<_, String>(1)?,
          row.get::<_, String>(2)?,
          row.get::<_, String>(3)?,
          row.get::<_, Option<String>>(4)?,
          row.get::<_, Option<bool>>(5)?,
        ))
      })?
      .collect::<Result<Vec<_>, _>>()?;
    raw
      .into_iter()
      .map(|(id, text, kind, scope, evidence, satisfied)| {
        Ok(Criterion {
          id,
          text,
          kind: enum_from(&kind)?,
          scope: serde_json::from_str(&scope)?,
          evidence,
          satisfied,
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
  pub fn snapshot(&self) -> Result<serde_json::Value, StoreError> {
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
  pub fn replace_file_index(&mut self, entries: &[FileEntry]) -> Result<(), StoreError> {
    let tx = self.conn.transaction()?;
    tx.execute("DELETE FROM file_index", [])?;
    for e in entries {
      tx.execute(
        "INSERT INTO file_index (path, size, mtime, sha256, state, findings) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
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
    tx.execute("DELETE FROM doc_sections", [])?;
    for s in sections {
      tx.execute(
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
    tx.commit()?;
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
