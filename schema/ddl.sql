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
