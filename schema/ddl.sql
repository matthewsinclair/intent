-- INTENT_VER: 3.2.1
-- SCHEMA_DDL_VER: 26
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
  -- The thread's fiat record, as serde JSON, or NULL. Present exactly when the
  -- thread reached `completed` through `st.fc` rather than `st.done`. LAST for
  -- the reason `tests.fiat` is last: a rung that recreates this table carries
  -- the older column list forward, and the new column is the one the SELECT
  -- must not name.
  fiat TEXT,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  -- A per-record write counter, bumped by the change door on every write. Its
  -- scope is the whole thread SUBTREE, not this row: `write_thread` deletes and
  -- re-inserts criteria, tests, related and wps, so every mutation to any of
  -- them passes through the bump below.
  --
  -- **THIS SHIPPED DESCRIBED AS ISSUE 0206's COMPARE-AND-SWAP TOKEN AND IT IS
  -- NOT ONE. CORRECTED HERE RATHER THAN REWRITTEN.** It is not monotonic across
  -- a sync: `rebuild` DELETEs every row before re-inserting it, so the upsert
  -- never hits a conflict, the clause below never fires, and the counter comes
  -- back at this DEFAULT. **It does not go up across a sync; it goes back to
  -- zero** -- measured, after the first version of this argument claimed a bump
  -- and the test's own non-vacuity control said `0 -> 0`.
  --
  -- A CAS on it would therefore FAIL OPEN on the case it was built for: a
  -- facade loaded at 0, a peer write taking the record to 1, a sync resetting
  -- it to 0, and the stale facade seeing `0 == 0` and writing straight over the
  -- peer. `Store::refuse_if_the_record_moved` compares CONTENT and carries the
  -- whole reasoning.
  --
  -- The argument this comment used to make against content -- that it is a
  -- HAND-MAINTAINED POPULATION which fails open for every column added after
  -- it -- was the reason to prefer a counter, and it was **wrong about the
  -- mechanism actually available**: `Thread` derives `PartialEq`, so the
  -- comparison enumerates no more than the counter does. The argument was
  -- sound against a hand-written field list, and nobody was proposing one.
  --
  -- **SO NOTHING READS THIS COLUMN, AND THAT IS SAID HERE RATHER THAN LEFT FOR
  -- THE NEXT READER TO DISCOVER** (vc, 2026-09-02). It is WRITTEN by the clause
  -- below and NAMED by `RECORD_WRITE_METADATA`, whose only use is to EXCLUDE it
  -- from `derived_dump`'s content comparison -- so its one consumer exists in
  -- order to ignore it. There is no `SELECT` of it in any crate. It is retained
  -- rather than dropped because removing a column costs an irreversible
  -- migration rung, which is a worse trade than a harmless counter; **silence
  -- here would send the next reader hunting for a consumer that does not
  -- exist.**
  revision INTEGER NOT NULL DEFAULT 0
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
  -- The package's fiat record, as serde JSON, or NULL. Carries an
  -- `inherited_from` when it was written by a cascade from its thread.
  fiat TEXT,
  -- 0100: a status v2 recorded outside the vocabulary, carried verbatim beside
  -- the status it was read as -- `scope_legacy`'s carry, for `status`. Last
  -- because rung 18 appends it.
  status_legacy TEXT,
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
--
-- **THIS TABLE HAS ONE WRITER AND THE SEARCH INDEX IS NOT IT.** It briefly
-- carried the index's four columns and they moved to `index_file` one rung
-- later: `replace_file_index` deletes every row the sync scan did not produce,
-- so a row the index had written for a source file disappeared on the next
-- sync -- and the two corpora are not nested either way, because the canon
-- corpus carries the rendered views and the extract that the index corpus
-- deliberately excludes. Neither writer can be given the other's delete rule,
-- so neither shares the other's table.
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
-- The SEARCH INDEX's scope table: one row per in-scope path, skipped ones
-- included, so that a file the index does not hold is a ROW SAYING WHY rather
-- than an absence. That is the difference between `intent index status`
-- reporting a skip and a user finding out by not getting a hit.
--
-- The scope is the gitignore-aware repository, which is WIDER than the change
-- detector's corpus above and excludes things that one carries: the rendered
-- views and the canon extract are the store's prose seen twice, and indexing
-- them would return every entity hit beside its own rendering.
--
-- `corpus` is `canon`, `prose` or `code`, and a kind the map does not recognise
-- is `code` with no `lang` rather than absent: a file with no corpus is a file
-- no surface can report. `indexed_sha256` is the content this row was last
-- indexed AT -- NULL where the file has not been read, which includes every
-- skipped row, because nothing hashes a file a skip reason excludes.
-- `skipped_reason` is NULL for a file the index holds.
--
-- `size` and `mtime` are the FILE's, carried so the stat-then-hash policy can
-- compare them without a second table; `created_at` / `updated_at` are the
-- row's own.
-- openness: DERIVED -- rebuilt by re-walking the working tree, and the files it
-- indexes are the user's own data, already readable without Intent.
CREATE TABLE IF NOT EXISTS index_file (
  path TEXT PRIMARY KEY,
  corpus TEXT NOT NULL,
  lang TEXT,
  size INTEGER NOT NULL,
  mtime TEXT NOT NULL,
  indexed_sha256 TEXT,
  skipped_reason TEXT,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  symbols_version INTEGER
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
-- The source half of the search index: code, one row per file at the lexical
-- tier. FTS5 like the prose table beside it, and with a DIFFERENT TOKENISER on
-- purpose: `unicode61` WITHOUT stemming, because stemming mangles identifiers,
-- while unicode61's default token characters already split `snake_case` into
-- its words, so a search for `disabled` finds `parse_disabled`. `CamelCase`
-- stays one token and is reached by a prefix search, which is why `name_parts`
-- exists as a column: the words inside a camel-cased name are searchable
-- because something puts them there.
--
-- `kind`, `name` and `name_parts` are empty for a whole-file row and are filled
-- by the structural tier, which has a grammar and can say what a span IS. An
-- empty column a later pass fills is honest; a guessed one is not.
--
-- `path` is UNINDEXED for the reason `doc_sections` keeps its addressing
-- unindexed: searching for a path is a different question from searching for
-- what is in a file, and one query must not quietly answer both.
-- openness: DERIVED -- recomputed by re-reading the files it points at, which
-- are the user's own and already on disk.
CREATE VIRTUAL TABLE IF NOT EXISTS src_sections USING fts5 (
  path UNINDEXED,
  seq UNINDEXED,
  start_line UNINDEXED,
  end_line UNINDEXED,
  kind UNINDEXED,
  name,
  name_parts,
  body,
  tokenize = 'unicode61'
);
-- The structural half of the search index: what each grammar's own tags query
-- named in a source file. One row per symbol, definitions and name-matched
-- references alike, told apart by `kind`.
--
-- **`kind` IS `def` OR `ref` AND THE SECOND IS A WEAKER CLAIM THAN IT LOOKS.**
-- A reference row says this identifier occurs here; nothing resolves it to the
-- definition it names, so no surface may render it as a call or a caller.
--
-- NOT an FTS5 table, and not for want of searching: the question this answers
-- is `does a thing with this name exist`, which is an equality on `name`, and
-- an inverted index over identifiers would answer a different question less
-- exactly. The lexical tier beside it is where a substring search belongs.
--
-- `lang` is the language whose grammar produced the row, which is not the same
-- fact as the path's language in `index_file`: this one says what actually
-- parsed it.
-- openness: DERIVED -- recomputed by re-parsing the files it points at, which
-- are the user's own and already on disk.
CREATE TABLE IF NOT EXISTS symbols (
  path TEXT NOT NULL,
  lang TEXT NOT NULL,
  name TEXT NOT NULL,
  kind TEXT NOT NULL,
  start_line INTEGER NOT NULL,
  end_line INTEGER NOT NULL,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  subkind TEXT NOT NULL DEFAULT '',
  container TEXT,
  container_kind TEXT,
  trait_name TEXT,
  arity INTEGER,
  arity_min INTEGER,
  qualifier TEXT,
  level INTEGER NOT NULL DEFAULT 1
);
CREATE INDEX IF NOT EXISTS symbols_by_name ON symbols (name);
CREATE INDEX IF NOT EXISTS symbols_by_path ON symbols (path);
-- Level 3: each language's last resolution run, one row per language.
--
-- **`state` SAYS WHAT THE LAST RUN DID, AND THE COUNTS SAY WHAT THE STORE
-- HOLDS.** `state` is `current`, `missing` or `failed`, and `path`, `line` and
-- `detail` are the last run's failure, NULL after a run that stored. The
-- counts, `run` and `resolved_at` belong to the last run that stored, so a
-- failure writes its own record and nothing else: the rows an earlier run
-- resolved still answer, each checked against its file's hash.
--
-- `resolved_at` is written by the database clock in the statement that stores
-- a run, and is NULL until one has. `run` counts the runs that stored, and
-- each `resolved_file` row names the one that wrote it. `symbols_version` is
-- the extractor version whose written rows that run joined against, the name
-- `index_file` uses for the same fact: a build writing another version has
-- re-extracted them, so every resolved row of the language is stale.
-- openness: DERIVED -- recomputed by running the language's own toolchain
-- over the project's files, which are already on disk.
CREATE TABLE IF NOT EXISTS resolution (
  lang TEXT PRIMARY KEY,
  state TEXT NOT NULL,
  tool TEXT NOT NULL,
  path TEXT,
  line INTEGER,
  detail TEXT,
  resolved_at TEXT,
  run INTEGER NOT NULL DEFAULT 0,
  matched INTEGER NOT NULL DEFAULT 0,
  unmatched INTEGER NOT NULL DEFAULT 0,
  dropped INTEGER NOT NULL DEFAULT 0,
  ambiguous INTEGER NOT NULL DEFAULT 0,
  symbols_version INTEGER,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);
-- The last stored run's `dropped` count, by reason: the core's reasons and
-- the ones the language's reader declares. The rows sum to
-- `resolution.dropped`, and a run that stores replaces them.
-- openness: DERIVED -- recomputed by running the language's own toolchain
-- over the project's files, which are already on disk.
CREATE TABLE IF NOT EXISTS resolution_dropped (
  lang TEXT NOT NULL,
  reason TEXT NOT NULL,
  count INTEGER NOT NULL,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  PRIMARY KEY (lang, reason)
);
-- Level 3's rows: a written reference and the definition a toolchain resolved
-- it to, one row per path, line, name and target.
--
-- **A SIDE TABLE AND NOT COLUMNS ON `symbols`**: `replace_symbols_for` deletes and re-inserts a file's rows at every
-- re-extract, so a column there would lose what a run resolved the moment the
-- index next read the file. A reconcile never deletes a row here.
--
-- **NO TIE-BREAK AT WRITE.** A key naming two targets holds both, and the
-- language's `ambiguous` count says how many keys did.
-- openness: DERIVED -- recomputed by running the language's own toolchain
-- over the project's files, which are already on disk.
CREATE TABLE IF NOT EXISTS resolved (
  path TEXT NOT NULL,
  line INTEGER NOT NULL,
  name TEXT NOT NULL,
  target TEXT NOT NULL,
  target_path TEXT,
  target_line INTEGER,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  PRIMARY KEY (path, line, name, target)
);
CREATE INDEX IF NOT EXISTS resolved_by_target ON resolved (target);
-- Level 3's staleness facts, once per file: the hash of the bytes the
-- toolchain read, the language, and the run that wrote the file's
-- rows. A file whose `index_file.indexed_sha256` no longer equals `sha256`
-- holds rows resolved against bytes that have moved, and `index status` names
-- it as stale.
-- openness: DERIVED -- recomputed by running the language's own toolchain
-- over the project's files, which are already on disk.
CREATE TABLE IF NOT EXISTS resolved_file (
  path TEXT PRIMARY KEY,
  lang TEXT NOT NULL,
  sha256 TEXT NOT NULL,
  run INTEGER NOT NULL,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);
-- When the search index was last reconciled. One row, id 1.
-- `reconciled_at` is written by the database clock in the statement that
-- records a reconcile; nothing reads a clock at render.
-- openness: DERIVED -- a fact about this store's own last reconcile.
CREATE TABLE IF NOT EXISTS index_state (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  reconciled_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);
-- The semantic tier's vectors. One row per indexed unit per model.
--
-- **`model` IS PART OF THE KEY BECAUSE TWO MODELS' SPACES ARE UNRELATED.** A
-- cosine between vectors of different models is a number with no meaning, so a
-- reader selects one model and never mixes; the same chunk may carry a vector
-- from each model it has been through.
--
-- `dims` is stored beside the vector rather than inferred from its length so
-- that a truncated BLOB is a refusal rather than a shorter vector that still
-- scores.
--
-- **THE VECTOR IS A BLOB OF LITTLE-ENDIAN f32**, which is the format the
-- writer and reader in `store.rs` agree on and the only place it is stated.
-- Nothing in this build writes a row: the tier is staged and its chunker is a
-- later package, so an empty table is the honest description of every store.
-- openness: DERIVED -- recomputed by re-embedding the corpus it points at,
-- which is the user's own files.
CREATE TABLE IF NOT EXISTS embeddings (
  chunk_id TEXT NOT NULL,
  model TEXT NOT NULL,
  dims INTEGER NOT NULL,
  vector BLOB NOT NULL,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  PRIMARY KEY (chunk_id, model)
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
-- openness: carried by intent/.canon/events/<YYYY>/<MM>/<DD>/<ULID>.json -- one
-- committed file per event, written in the same write set as the act it records,
-- named by its id and never rewritten. Acts that describe one machine only, such
-- as heartbeats and restores, stay in this table. Ingest adds every file whose id
-- this table does not hold and deletes nothing. No verb writes the
-- single-file form, events.jsonl, as a file.
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
-- THE COORDINATION ENTITIES. One board per node.
--
-- **`recorded_at` CARRIES NO `DEFAULT` AND THAT IS THE REQUIREMENT, NOT AN
-- OMISSION.** With the database as truth and sync running both ways, a
-- disk-to-db resync that re-inserts rows would let `DEFAULT CURRENT_TIMESTAMP`
-- re-stamp them -- rewriting history silently and indistinguishably from a
-- correct value, which is the fabricated-stamp failure reintroduced by its own
-- fix. The SERVICE writes it once, at the write, and a sync in either direction
-- CARRIES it rather than re-deriving it.
--
-- `updated_at` is this store's own record stamp and is the opposite kind of
-- value: per-machine, omitted from the extract, correctly re-stamped by a
-- rebuild. The two live side by side deliberately; neither can do the other's
-- job.
--
-- `authored_at` is the stamp a migrated board's markdown CLAIMED, verbatim and
-- untrusted -- the one column in this store whose contents are known to include
-- invented values, kept as text and never read as a time.
--
-- `migrated_at` is when a node's board became the model's: stamped by `wb
-- migrate`, and by registering a node from its arguments, which has no
-- hand-authored board to carry. Null means the markdown on disk is still the
-- board, so every board write refuses until the node is migrated. It sits at
-- the tail, where the rung that added it rebuilds the table to put it.
-- openness: carried by intent/whiteboard/<node>/board.json
CREATE TABLE IF NOT EXISTS wb_node (
  moniker TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  role TEXT NOT NULL,
  session_id TEXT,
  heartbeat_at TEXT NOT NULL,
  status TEXT NOT NULL,
  focus TEXT NOT NULL,
  claims TEXT NOT NULL DEFAULT '[]',
  recorded_at TEXT NOT NULL,
  authored_at TEXT,
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  migrated_at TEXT
);
-- openness: carried by intent/whiteboard/<node>/board.json
CREATE TABLE IF NOT EXISTS wb_item (
  id INTEGER PRIMARY KEY,
  node TEXT NOT NULL,
  kind TEXT NOT NULL,
  seq INTEGER NOT NULL,
  text TEXT NOT NULL,
  state TEXT NOT NULL,
  archived_at TEXT,
  recorded_at TEXT NOT NULL,
  authored_at TEXT,
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  edited_at TEXT
);
-- **`id` IS AN INTEGER PRIMARY KEY SO INSERTION ORDER IS RECOVERABLE**, which
-- is not decoration: every row a migration inserts in one pass shares one
-- `recorded_at`, so that column cannot order them and the board's only
-- cross-node ordering would be lost at the moment it became queryable. The
-- migration inserts in SOURCE FILE ORDER, ties on `recorded_at` order by
-- insertion, and every view orders that way.
-- openness: carried by intent/whiteboard/<node>/board.json
CREATE TABLE IF NOT EXISTS wb_message (
  id INTEGER PRIMARY KEY,
  sender TEXT NOT NULL,
  recipient TEXT NOT NULL,
  body TEXT NOT NULL,
  re TEXT,
  fyi INTEGER NOT NULL DEFAULT 0,
  state TEXT NOT NULL,
  handled_at TEXT,
  recorded_at TEXT NOT NULL,
  authored_at TEXT,
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  edited_at TEXT
);
CREATE INDEX IF NOT EXISTS wb_item_by_node ON wb_item (node, kind, seq);
CREATE INDEX IF NOT EXISTS wb_message_by_recipient ON wb_message (recipient, id);
