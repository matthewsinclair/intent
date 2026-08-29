-- INTENT_VER: 3.0.0
-- SCHEMA_DDL_VER: 12
-- Intent v3 runtime store (GENERATED FACE -- the master is
-- native/rust/crates/intentsvcs/src/store.rs; regenerate via INTENT_BLESS, never edit).
-- The durable source of truth for a project, not an index of its files.
-- Re-creatable from the committed extract as a CAPABILITY; migrations are
-- normal when the schema moves.
--
-- EVERY TABLE DECLARES HOW ITS DATA LEAVES. `-- openness: carried by <path>`
-- names the file form that holds it losslessly; `-- openness: DERIVED` states
-- why it needs none, and always says why; `-- openness: ON DEMAND <path>` names
-- a file form that is PRODUCED rather than projected, and says why it is not
-- kept in the tree. Absence of a declaration is never the answer -- a table
-- with no line is a table nobody has said how to get data out of, and
-- tests/openness.rs refuses one.
--
-- THE THIRD FORM IS NOT A LOOPHOLE FOR THE SECOND. DERIVED
-- means the data is reconstructible from something else that IS on disk, which
-- `event_log` is not -- it is the one table derived from nothing, so it can
-- never take that exemption however convenient it looks. ON DEMAND says the
-- opposite: the file form is real, lossless and standard, and the estate simply
-- does not keep a copy of it lying in the working tree. **Its evidence is
-- STRONGER than the second form's, not weaker**: `carried by` is proved by a
-- path existing, while ON DEMAND is proved by driving the exporter and watching
-- the bytes come out, which is the property hv's requirement actually asks for
-- -- that the data can LEAVE, not that a file is sitting there.
--
-- TWO KINDS OF TIME LIVE HERE AND THEY ARE NOT INTERCHANGEABLE. Every table
-- carries a record timestamp, written by the database as part of the write and
-- never passed in by a caller:
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
-- A schema carrying a plausible `created` column is exactly how a table comes
-- to ship with no record time at all and nobody notices.
--
-- Tables with durable row identity are upserted and carry `created_at` +
-- `updated_at`. Tables whose rows are deleted and re-inserted with their parent
-- carry `written_at` -- when THIS VERSION of the row was written -- because a
-- `created_at` there would record the latest write under the name of the first.
--
-- Every stamp is millisecond resolution. At second resolution two writes in the
-- same second collide, and these stamps are what orders records when two
-- machines merge their event logs.
-- openness: carried by intent/.canon/st/<ID>.json
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
  -- Every other authored section, verbatim and in authored order. Sections
  -- byte-identical to the template that created the file are not here: no
  -- author wrote them, and carrying them files scaffolding as authored prose.
  body TEXT NOT NULL DEFAULT '',
  -- Authored prose ABOVE the first heading, minus the `# ` title, STRIPPED.
  -- Its own column and not part of `body`: `body` renders below the objective,
  -- so a preamble carried there comes back in the wrong place -- bytes kept,
  -- position moved, which is harder to see than a drop.
  preamble TEXT NOT NULL DEFAULT '',
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);
-- openness: carried by intent/.canon/st/<ID>.json
CREATE TABLE IF NOT EXISTS related (
  thread_id TEXT NOT NULL REFERENCES threads (id) ON DELETE CASCADE,
  seq INTEGER NOT NULL,
  id TEXT NOT NULL,
  note TEXT,
  written_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  PRIMARY KEY (thread_id, seq)
);
-- Authored files under a thread that no typed document has a place for -- a
-- plan, a reference, a journal. Carried whole; nothing here is parsed.
-- `path` is relative to the THREAD's directory, which is why a file nested
-- under it needs no second table to hold it, and why it is the key.
-- `bytes` and `sha256` DESCRIBE `text` -- they are written by one constructor
-- and never set independently, so a stored hash cannot come to disagree with
-- the content it describes.
-- openness: carried by intent/.canon/st/<ID>.json
-- `seq` is the ORDER THE PRODUCER CHOSE, carried rather than re-derived. The
-- store gives back what it was given: a read that sorted by `path` would
-- reorder a thread whose attachments arrived any other way, and canon compared
-- against its own round trip would differ for a reason nothing in the data
-- explains. `path` is still unique, so it is a UNIQUE rather than the key.
CREATE TABLE IF NOT EXISTS attachments (
  thread_id TEXT NOT NULL REFERENCES threads (id) ON DELETE CASCADE,
  seq INTEGER NOT NULL,
  path TEXT NOT NULL,
  -- NULLABLE, and its absence is what OPAQUE means. An attachment carries text
  -- or it carries bytes, never both and never neither, which the CHECK below
  -- states so the table cannot hold a shape the model forbids.
  text TEXT,
  -- An opaque attachment's bytes. The store is the authoritative record, so
  -- these live HERE as well as in the committed extract's sidecar file -- a
  -- store that held only the hash could report divergence and never hydrate
  -- the file back.
  blob BLOB,
  bytes INTEGER NOT NULL,
  sha256 TEXT NOT NULL,
  written_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  PRIMARY KEY (thread_id, seq),
  UNIQUE (thread_id, path),
  -- **EXACTLY ONE, enforced by the database rather than by every writer.** The
  -- model's constructors already guarantee it, and the model is not the only
  -- thing that has ever written this table -- a migration rung is a writer too,
  -- and rung 11 exists because one of them produced a shape nobody checked.
  CHECK ((text IS NULL) <> (blob IS NULL))
);
-- openness: carried by intent/.canon/st/<ID>.json
-- `scope` is NULLABLE and `scope_legacy` sits beside it, exactly as `file` and
-- `legacy` do on `tests`. v2 read scope as free text and one work package in
-- the corpus carries `Medium-Large`, which sits BETWEEN two enum members: the
-- ratified carry policy forbids normalising it (a guess), blocking it (it is
-- in a CLOSED thread) and dropping it (loss), so it is carried as legacy and
-- the enum column holds nothing rather than a lie.
CREATE TABLE IF NOT EXISTS wps (
  thread_id TEXT NOT NULL REFERENCES threads (id) ON DELETE CASCADE,
  seq INTEGER NOT NULL,
  title TEXT NOT NULL,
  scope TEXT,
  scope_legacy TEXT,
  status TEXT NOT NULL,
  status_reason TEXT,
  objective TEXT NOT NULL,
  body TEXT NOT NULL,
  -- As `threads.preamble`; 5 of the canary's 20 regions are work-package ones.
  preamble TEXT NOT NULL DEFAULT '',
  written_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  PRIMARY KEY (thread_id, seq)
);
-- `state` is the whole recorded AC state as its serde JSON, replacing the
-- `scope`/`evidence`/`satisfied` trio. One column because the state is one
-- value: the trio could hold combinations the model has no meaning for (a
-- descoped row carrying `satisfied`), and a schema that can represent a
-- contradiction eventually stores one. Same treatment `legacy` already gets.
-- The discriminant stays queryable as `json_extract(state, '$.is')`.
-- openness: carried by intent/.canon/st/<ID>.json
CREATE TABLE IF NOT EXISTS criteria (
  thread_id TEXT NOT NULL REFERENCES threads (id) ON DELETE CASCADE,
  id TEXT NOT NULL,
  text TEXT NOT NULL,
  kind TEXT NOT NULL,
  state TEXT NOT NULL,
  written_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  PRIMARY KEY (thread_id, id)
);
-- openness: carried by intent/.canon/st/<ID>.json
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
  -- The AT's fiat record, as serde JSON, or NULL. **LAST ON PURPOSE: `ALTER
  -- TABLE ... ADD COLUMN` appends, so a store migrated up rung 15 gets it here
  -- and a store created fresh from this DDL has to agree, or the two shapes
  -- differ by column order for the rest of their lives.**
  fiat TEXT,
  PRIMARY KEY (thread_id, id)
);
-- `created` is AUTHORED -- v2 users write it by hand in frontmatter, so it is a
-- fact about the world and stays, with a DB stamp beside it rather than
-- replaced by one.
-- `closed` is NULL on every issue converted from a v2 estate, and that is the
-- older format rather than a gap: its issue frontmatter carried six keys and a
-- closed date was not one of them. There is nothing to back-fill it from, and a
-- filesystem mtime is a fact about a file rather than about the world, so it
-- stays NULL. All-NULL here means converted data, never a reader that failed.
-- `reporter` is free text, and it is the one converted key that had no column
-- until the estate was measured. It is modelled rather than carried as legacy
-- because a name is not a value outside a vocabulary -- there is no enum for it
-- to sit between, so `scope_legacy`'s shape would buy nothing. An issue is a
-- report against a released version, which is what makes who filed it
-- load-bearing rather than incidental.
-- `body` is the issue's authored prose, carried whole and never parsed. It is
-- here rather than in a sibling `<nnnn>.md` because hv ruled that disk becomes
-- optional: prose whose only home is a file is destroyed by the first render,
-- which is the defect this column exists to close rather than a style choice
-- about where markdown lives.
-- openness: carried by intent/.canon/issues/<NNNN>.json
CREATE TABLE IF NOT EXISTS issues (
  number INTEGER PRIMARY KEY,
  slug TEXT NOT NULL,
  title TEXT NOT NULL,
  status TEXT NOT NULL,
  severity TEXT,
  created TEXT NOT NULL,
  closed TEXT,
  reporter TEXT,
  body TEXT NOT NULL DEFAULT '',
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
-- **THE DB STAMPS THE RECORD, AND THE APPLICATION NEVER SUPPLIES A TIME.**
-- `ts` carries a DEFAULT so the stamp is applied AS PART OF THE INSERT. A
-- caller that read a clock and then wrote the value would hold it across a
-- gap, so a retried, deferred or batched write would be stamped when it was
-- PREPARED rather than when it happened -- invisible by inspection. A DEFAULT
-- has no gap: the stamp and the write are one operation.
-- The column is still WRITABLE, and that is not a loophole: restoring the
-- committed extract must carry each envelope's ORIGINAL time, which is a
-- different act from recording that something just happened.
-- **`ts` IS THIS TABLE'S RECORD TIMESTAMP AND THERE IS DELIBERATELY NO SECOND
-- COLUMN.** Stated rather than left as an absence, because a missing
-- measurement must present as a refusal and never as a measurement of nothing:
-- an unexplained gap here reads as an oversight and gets re-audited. An event
-- row is append-only and immutable, so it has no `updated_at` to have: nothing
-- ever updates it, and a column recording an act that cannot happen is a guard
-- that passes vacuously.
-- **THE BACKUP LOG RECORDS ATTEMPTS, NOT SUCCESSES**, and that is the whole
-- reason it is a table rather than a directory listing.
--
-- A directory of snapshot files can only answer what EXISTS. It cannot tell
-- a schedule that has never run from one that runs and fails every time, and
-- those need different actions from a user. A row is written BEFORE the copy
-- is attempted and updated after, so a crashed or failed attempt leaves a row
-- saying so -- a backup that fails is not allowed to be indistinguishable from
-- a backup that was never due.
--
-- `taken_at` is the row's record timestamp under its own name: there is one
-- event here and it is the attempt, so `created_at` and a separate `taken_at`
-- would be two columns for one moment. It is what retention buckets on and
-- what staleness is measured from, and BOTH of those are computed in SQL --
-- the database compares its own stamp against its own `now` and returns a
-- verdict, so no time is ever handed to the application to hold.
-- openness: DERIVED -- an operations log about files on THIS machine's disk.
-- It describes nothing about the project, and the snapshots it points at are
-- plain SQLite databases that any tool can open without Intent.
CREATE TABLE IF NOT EXISTS snapshots (
  id INTEGER PRIMARY KEY,
  path TEXT,
  bytes INTEGER,
  outcome TEXT NOT NULL DEFAULT 'attempted',
  detail TEXT,
  taken_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);
-- **WHETHER THE LAST LOAD FROM CANON FINISHED.**
--
-- A reader of this face needs it because the answer decides whether the store
-- can be projected back over the files: a store whose last load was refused may
-- be older than the canon beside it, and writing it out would overwrite
-- authored work the store never took.
--
-- A row is written BEFORE the load is attempted and updated after, which is the
-- `snapshots` shape and is here for the same reason plus a sharper one: the
-- refusal this exists to record is a SQLite failure INSIDE the rebuild
-- transaction, so anything written in that transaction rolls back with it. The
-- attempt row has to be committed before the rebuild opens, or the store
-- forgets it was ever asked.
--
-- An unfinished row therefore reads `attempted`, which is not `succeeded`, so a
-- crash mid-ingest fails the safe way without anything having to catch it.
-- openness: DERIVED -- an operations log about THIS machine's store, recording
-- whether the store is currently older than the canon beside it. It describes
-- nothing about the project: a clone's copy of the estate cannot inherit
-- another machine's load history, and would be wrong if it did.
CREATE TABLE IF NOT EXISTS ingests (
  id INTEGER PRIMARY KEY,
  outcome TEXT NOT NULL DEFAULT 'attempted',
  detail TEXT,
  started_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);
-- openness: ON DEMAND events.jsonl -- produced by `intent export`, not projected
-- into the working tree. The tracked extract was deleted: it was the sole carrier of
-- history across a clone, and git already is that carrier for everything the canon
-- describes. The file form itself is unchanged and still lossless.
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
-- PROJECT-LEVEL RECORDED STATE. A singleton -- `CHECK (id = 1)` -- because there
-- is one project per store and a table that could hold two would need a rule
-- about which one counts.
--
-- `todo_watermark` is the DONE cutoff: the instant of the last
-- `intent todo done --flush`/`--prune`. It is STATE rather than history, which
-- is why it is a column here and not a query over `event_log`. A flush
-- HAPPENING at T is an event and belongs in the log; the current cutoff BEING T
-- is a fact about the project now, so it is recorded here and travels with the
-- project's committed files rather than with its history.
--
-- NULL means never flushed.
-- openness: carried by intent/.canon/project.json
CREATE TABLE IF NOT EXISTS project (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  todo_watermark TEXT,
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);
