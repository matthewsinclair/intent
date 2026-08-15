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
  objective TEXT NOT NULL,
  body TEXT NOT NULL,
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
