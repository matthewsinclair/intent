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
  AcceptanceTest, BOARD_SCHEMA, Board, Criterion, ISSUE_SCHEMA, Issue, Legacy, Related,
  THREAD_SCHEMA, Thread, WbItem, WbMessage, WbNode, WorkPackage, enum_str,
};
use crate::prose::DocSection;
use crate::sync::FileEntry;

/// The DDL face. Applied verbatim on open; committed under `schema/ddl.sql`
/// by the faces machinery and drift-checked against this constant.
///
/// **EVERY LINE BELOW IS PUBLISHED, INCLUDING THE `--` COMMENTS.** `intent
/// schema ddl.sql` prints this constant into the caller's terminal, so a
/// comment in here is not a note to ourselves -- it is documentation shipped to
/// a stranger, and under D37 it must carry nothing of Intent's own project
/// management. The design rationale that used to live inside the string is
/// therefore in the `//` block below, which no generator lifts and no command
/// prints. When editing, the test is simple: **would this sentence help someone
/// who has never seen our repository?** If not, it belongs up here.
//
// WHY THE TIMESTAMPS ARE SHAPED THIS WAY (AC-02.8, D42), kept out of the face.
//
// **The column is named for what it can honestly record, never for uniformity
// across tables** (vc, ruling, 2026-08-15). `threads`, `issues` and
// `file_index` have durable row identity, so they are UPSERTED and their
// `created_at` fires exactly once while `updated_at` moves with each write.
// `related`, `wps`, `criteria` and `tests` are deleted and re-inserted with
// their parent -- a removed WP must vanish -- so a `created_at` there would
// record the latest write while carrying the name of the first, which is
// AC-02.8's remedy reintroducing AC-02.8's defect. They get `written_at`.
//
// **Milliseconds rather than seconds was MEASURED, not preferred.** At second
// resolution two writes in the same second carry identical stamps, which is
// what any script produces, and it was found by a MUTATION TEST rather than by
// reasoning: reverting `threads` to delete-and-reinsert should have moved
// `created_at`, the test asserting it did not still passed, and the reason was
// that both writes landed inside one second. A guard blind to the defect it
// names is worse than no guard -- and the same blindness is load-bearing in the
// product, because under D34 two machines MERGE their event logs and a merge
// orders records by a time nobody typed.
//
// **`written_at` is a scope call with a stated reversal, not a claim about the
// domain** (D39): `wps` and `criteria` do have stable ids, so if per-row
// durable history is wanted the upgrade is delete-missing + upsert-present, and
// `written_at` does not block it. What is not reversible is shipping a
// `created_at` on a table that re-stamps it.
//
// WHY THE DONE CUTOFF IS A COLUMN AND NOT A QUERY (WP-14, hv 2026-08-26), kept
// out of the face for the same reason.
//
// **Derived from the event log it could not cross a git clone.** D53 took the
// log out of the working tree, so a cutoff read back out of it was absent on a
// fresh clone -- every flushed thread reappeared in DONE (52 completed + 2
// cancelled, measured on this repo) and `doctor` called the committed
// `todo.md` hand-edited, permanently. Filing the cutoff as history is what put
// it on the wrong side of D53.
//
// **NOTHING READS A CUTOFF OUT OF THE LOG.** The migration rung that creates
// the table derives the value once and that is the last time it happens.
// `event::todo_watermark` was DELETED rather than left as a fallback: two homes
// for one value is the defect this keeps finding, not a safety net, and both
// answers look plausible at the call site.
//
// **NULL means never flushed, which v2 could not represent** -- it read the
// cutoff back out of the generated file, so an absent file had to fall back to
// a clock.
pub const DDL: &str = "\
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
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);
CREATE INDEX IF NOT EXISTS symbols_by_name ON symbols (name);
CREATE INDEX IF NOT EXISTS symbols_by_path ON symbols (path);
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
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
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
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);
CREATE INDEX IF NOT EXISTS wb_item_by_node ON wb_item (node, kind, seq);
CREATE INDEX IF NOT EXISTS wb_message_by_recipient ON wb_message (recipient, id);
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
pub const SCHEMA_VERSION: i32 = 25;

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
pub const RECORD_TIMESTAMPS: &[&str] = &["created_at", "updated_at", "written_at", "taken_at"];

/// Per-machine write metadata that is NOT modelled content, and is not a
/// timestamp either.
///
/// **A SEPARATE LIST RATHER THAN A FOURTH ENTRY IN [`RECORD_TIMESTAMPS`], and
/// the reason is that the other list's NAME is load-bearing.** Two tests read
/// it as "the columns that are record timestamps" and assert a property that
/// only timestamps have -- `one_clock` checks each carries a `strftime`
/// DEFAULT. `revision` is an INTEGER counter with a constant default, so
/// adding it there would either break those tests or force them to special-case
/// a member, which is how a list stops meaning its own name.
///
/// The EXCLUSION reasoning is shared, and it is [`RECORD_TIMESTAMPS`]'s
/// verbatim: `derived_dump` answers "is the modelled CONTENT identical", and a
/// value that is re-stamped by the act of writing makes that property false by
/// construction. `revision` moves on every write through the change door,
/// including a write that changes nothing else. **A sync does not bump it, it
/// RESETS it** -- `rebuild` deletes before it inserts, so the conflict clause
/// never runs. Either way the value is not modelled content.
pub const RECORD_WRITE_METADATA: &[&str] = &["revision"];

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
), (
  4,
  // 3 -> 4: the backup log. A new table, so this is an ADD rather than a
  // rebuild -- nothing existing changes shape and no row moves.
  "CREATE TABLE IF NOT EXISTS snapshots (
     id INTEGER PRIMARY KEY,
     path TEXT,
     bytes INTEGER,
     outcome TEXT NOT NULL DEFAULT 'attempted',
     detail TEXT,
     taken_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
     updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
   );",
), (
  5,
  // 4 -> 5: `wps.scope` loses NOT NULL and `scope_legacy` arrives beside it,
  // for the marked-legacy carry form (vc's ruling, data-model.md).
  //
  // **A REBUILD rather than an ALTER, and only the first half needs one.**
  // Adding a nullable column is a legal `ALTER TABLE ADD COLUMN`; DROPPING a
  // NOT NULL is not expressible in SQLite at all, so the table is rebuilt and
  // both changes ride together rather than leaving the column constrained
  // against the model that no longer is.
  //
  // Every existing row keeps its scope. Nothing is re-derived and nothing is
  // guessed: a store built before this rung has no legacy scopes in it, so
  // `scope_legacy` is correctly NULL everywhere and the column arrives empty.
  "CREATE TABLE wps_v5 (
     thread_id TEXT NOT NULL REFERENCES threads (id) ON DELETE CASCADE,
     seq INTEGER NOT NULL,
     title TEXT NOT NULL,
     scope TEXT,
     scope_legacy TEXT,
     status TEXT NOT NULL,
     status_reason TEXT,
     objective TEXT NOT NULL,
     body TEXT NOT NULL,
     written_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
     PRIMARY KEY (thread_id, seq)
   );
   INSERT INTO wps_v5 (thread_id, seq, title, scope, status, status_reason, objective, body, written_at)
     SELECT thread_id, seq, title, scope, status, status_reason, objective, body, written_at FROM wps;
   DROP TABLE wps;
   ALTER TABLE wps_v5 RENAME TO wps;",
), (
  6,
  // 5 -> 6: `issues.reporter` arrives, for the one v2 issue key that had no
  // column (WP-10; 40 of 40 issues in this estate carry it).
  //
  // **An `ALTER` rather than a rebuild, and the difference is the constraint
  // rather than the change.** A nullable column with no default is the one
  // shape SQLite's `ADD COLUMN` accepts, so nothing is dropped, no row moves,
  // and no foreign key is momentarily dangling.
  //
  // Every existing row gets NULL, which is correct and not a gap to fill
  // later: a store built before this rung was built by a binary that never
  // read the field, so there is no reporter it could have known and declined
  // to record. The values arrive with the migration that reads them.
  "ALTER TABLE issues ADD COLUMN reporter TEXT;",
), (
  7,
  // 6 -> 7: `threads.body` -- every authored section that is not the objective
  // or the context, the same two-field shape D28 gave the work package.
  //
  // **An ALTER rather than a rebuild, and legally so**: SQLite refuses
  // `ADD COLUMN` for a NOT NULL column only when its default is non-constant,
  // and `''` is constant. Rung 5 rebuilt because it DROPPED a constraint;
  // nothing here changes shape.
  //
  // **Existing rows take `''` and that is the honest value, not a placeholder.**
  // A store at version 6 was written by an ingest that never read these
  // sections, so there is no prior content to preserve and no author's prose
  // being overwritten. The sections are re-read on the next ingest.
  "ALTER TABLE threads ADD COLUMN body TEXT NOT NULL DEFAULT '';",
), (
  8,
  // 7 -> 8: `preamble` on BOTH tables -- the region above the first heading,
  // which was not carried anywhere and was being reported as LOST-PROSE.
  //
  // **Two statements in one rung, because one field arrived at two levels.**
  // 15 of the canary's 20 regions are thread-level and 5 are work-package, so
  // shipping only the thread half would close 75% of a conservation hole and
  // leave the rest reporting as lost with no record of why.
  //
  // Same ALTER argument as rung 7: `''` is a constant default, so SQLite
  // permits it on a NOT NULL column, and nothing here changes shape.
  //
  // **APPENDED AT THE TAIL, and rung 7 is why that is written down twice.**
  // The walk runs this array in order; placed at the head, this would run
  // before rung 3 rebuilds `threads` by selecting the columns it knew about --
  // silently dropping the column just added, then stamping the new version over
  // a store with the old shape.
  "ALTER TABLE threads ADD COLUMN preamble TEXT NOT NULL DEFAULT '';
   ALTER TABLE wps ADD COLUMN preamble TEXT NOT NULL DEFAULT '';",
), (
  9,
  // 8 -> 9: `issues.body` -- the issue's authored prose, which had no home in
  // the model at all and lived only in the v2 files the migration reads.
  //
  // Same ALTER argument as rungs 7 and 8, and appended at the tail for rung
  // 8's reason: the walk runs this array in order, and a rung placed ahead of
  // rung 3's rebuild is dropped by it and then stamped over as though it had
  // landed.
  //
  // **`''` on every existing row is the honest value here for a sharper reason
  // than on rung 7.** A store at version 8 was written by a binary whose
  // `Issue` had no `body` field, so no ingest could have read one and no
  // author's prose is being overwritten. The content arrives when the estate
  // is re-read -- and on an already-migrated estate that means re-running the
  // v2 conversion, because the v3 canon written before this rung never carried
  // it either. That is a data question, not a schema one, and it is why the
  // migrator's carry is the same commit as this column.
  "ALTER TABLE issues ADD COLUMN body TEXT NOT NULL DEFAULT '';",
), (
  10,
  // 9 -> 10: `attachments` -- the authored files under a thread that no typed
  // document has a place for.
  //
  // **A CREATE rather than an ALTER, which is the first new table since the
  // ladder began, and it is the easy case**: an empty table is a correct
  // representation of a store that never read one, so there is nothing to
  // back-fill and no existing row to reshape.
  //
  // Appended at the tail for rung 8's reason -- the walk runs this array in
  // order, and anything placed ahead of rung 3's rebuild is silently dropped
  // by it and then stamped over as though it had landed.
  //
  // **`CREATE TABLE` here is NOT redundant with the `IF NOT EXISTS` in `DDL`,
  // and the difference is which stores each one reaches.** `DDL` runs on a
  // store being created; this runs on one that already exists and is being
  // walked forward. A store at version 9 has been through `DDL` once, at a
  // revision that did not contain this table.
  "CREATE TABLE IF NOT EXISTS attachments (
     thread_id TEXT NOT NULL REFERENCES threads (id) ON DELETE CASCADE,
     seq INTEGER NOT NULL,
     path TEXT NOT NULL,
     text TEXT NOT NULL,
     bytes INTEGER NOT NULL,
     sha256 TEXT NOT NULL,
     written_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
     PRIMARY KEY (thread_id, seq),
     UNIQUE (thread_id, path)
   );",
), (
  11,
  // 10 -> 11: `attachments` gains `seq` -- and this rung exists because I
  // EDITED RUNG 10 AFTER A STORE HAD ALREADY RUN IT.
  //
  // **THE RULE, and it belongs here rather than in the commit that learned it:
  // a version number is a CLAIM ABOUT SHAPE, so once any store has run a rung,
  // changing what that rung PRODUCES requires a new rung and never an edit to
  // the old one.** The old rung's output is already stamped and permanently
  // unreachable -- version-gated so it cannot run again, and `CREATE TABLE IF
  // NOT EXISTS` would skip it even if it did. It is the ladder's form of not
  // rewriting published history.
  //
  // What actually happened: this project's own store ran rung 10 at a revision
  // before `seq` existed, reached 10, and kept a table with no `seq` column.
  // Then `attachments_of` learned `ORDER BY seq` and every read of the canon
  // failed with `no such column: seq` -- found by hv driving `st list` through
  // the CLI, not by any test.
  //
  // **AND NO TEST COULD HAVE FOUND IT, which is the part worth keeping.** Every
  // test starts from a fresh store, so every test gets the current `DDL` with
  // `seq` and passes. The defect is reachable only from a store that existed
  // BEFORE the change -- ours, and any real user's. A suite that always starts
  // fresh cannot see a migration defect at all.
  //
  // **A REBUILD, and it reads only the columns BOTH shapes have**, because two
  // different tables are stamped 10 and nothing in the store can tell them
  // apart. `seq` is synthesised from `rowid`, which is insertion order -- so a
  // store that already had the column keeps the order it recorded rather than
  // being silently re-sorted, and one that never had it gets the order its rows
  // arrived in. Rows are carried; the live store happened to hold none, and a
  // rung must not assume that.
  "CREATE TABLE attachments_v11 (
     thread_id TEXT NOT NULL REFERENCES threads (id) ON DELETE CASCADE,
     seq INTEGER NOT NULL,
     path TEXT NOT NULL,
     text TEXT NOT NULL,
     bytes INTEGER NOT NULL,
     sha256 TEXT NOT NULL,
     written_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
     PRIMARY KEY (thread_id, seq),
     UNIQUE (thread_id, path)
   );
   INSERT INTO attachments_v11 (thread_id, seq, path, text, bytes, sha256, written_at)
     SELECT thread_id,
            ROW_NUMBER() OVER (PARTITION BY thread_id ORDER BY rowid) - 1,
            path, text, bytes, sha256, written_at
       FROM attachments;
   DROP TABLE attachments;
   ALTER TABLE attachments_v11 RENAME TO attachments;",
),(
  12,
  // 11 -> 12: `ingests` arrives. Purely additive -- no existing table is read
  // or rewritten -- so the rung is the bare `CREATE`.
  //
  // **The rung is not redundant with the `IF NOT EXISTS` in [`DDL`], and this
  // is the trap [`SCHEMA_VERSION`]'s own doc names from the other side.** The
  // DDL apply after the ladder would indeed create the table; what it would not
  // do is move `user_version`, so an 11-stamped store would migrate on every
  // open forever, correct in shape and permanently unstamped. The rung exists
  // to carry the version, and the `CREATE` is here so the two never disagree
  // about which rung created what.
  //
  // **Existing stores arrive with the table EMPTY, and that is the right
  // starting state rather than a gap.** An empty history means nothing was
  // recorded, which is exactly true of every store written before this shipped
  // -- back-filling a `succeeded` row would be asserting a load nobody
  // observed, and back-filling a `refused` one would block the egest on every
  // upgraded project at once. `Store::last_ingest` returning `None` is the
  // honest answer and the egest guard reads it as "no evidence either way".
  "CREATE TABLE IF NOT EXISTS ingests (
     id INTEGER PRIMARY KEY,
     outcome TEXT NOT NULL DEFAULT 'attempted',
     detail TEXT,
     started_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
     updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
   );",
), (
  13,
  // 12 -> 13: `attachments.text` becomes NULLABLE and `blob` arrives, so an
  // OPAQUE attachment has somewhere to be (ST0057 AC-03.1). The CHECK is
  // AC-03.2 -- form follows content, both ways -- stated where the database
  // enforces it rather than where a writer promises it.
  //
  // **THE CRITERION IDS LIVE HERE, IN A RUST COMMENT, AND NOT IN THE `DDL`
  // STRING ABOVE.** `intent schema ddl.sql` PRINTS that string, so a `-- ...
  // (ST0057 AC-03.2)` inside it ships this project's own work tracking to every
  // consumer of the published contract. `no_pm_state_in_output` caught exactly
  // that, on this table, in this commit. The SQL comments say what the column
  // means; the reasoning about which row required it says so out here.
  //
  // **A REBUILD rather than two `ALTER`s, because SQLite cannot drop a NOT NULL
  // constraint in place.** `ADD COLUMN blob BLOB` alone would leave `text NOT
  // NULL` standing, and every opaque insert would then fail at the database
  // with a constraint error rather than being impossible to express -- which is
  // the failure that looks like a bug in the caller.
  //
  // **Rung 12 is ic's `ingests` table and this is 13, deliberately.** ic bumped
  // 11 -> 12 in the same working tree on the same afternoon; two rungs sharing
  // a number is precisely what rung 11's own note is about -- two shapes
  // stamped one version, and nothing in the store able to tell them apart.
  //
  // Every existing row is text by construction: `text NOT NULL` was the old
  // shape, so no row can carry bytes and none needs a `blob`. The CHECK is
  // therefore satisfied by the carried rows without inspecting them, and if it
  // were not the rung would fail LOUDLY at the constraint rather than quietly
  // dropping whatever offended it.
  "CREATE TABLE attachments_v13 (
     thread_id TEXT NOT NULL REFERENCES threads (id) ON DELETE CASCADE,
     seq INTEGER NOT NULL,
     path TEXT NOT NULL,
     text TEXT,
     blob BLOB,
     bytes INTEGER NOT NULL,
     sha256 TEXT NOT NULL,
     written_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
     PRIMARY KEY (thread_id, seq),
     UNIQUE (thread_id, path),
     CHECK ((text IS NULL) <> (blob IS NULL))
   );
   INSERT INTO attachments_v13 (thread_id, seq, path, text, blob, bytes, sha256, written_at)
     SELECT thread_id, seq, path, text, NULL, bytes, sha256, written_at
       FROM attachments;
   DROP TABLE attachments;
   ALTER TABLE attachments_v13 RENAME TO attachments;",
), (
  14,
  // 13 -> 14: the `project` singleton arrives, and it is an ADD -- nothing
  // existing changes shape and no row moves.
  //
  // **THE SEED IS THE WHOLE POINT OF THE RUNG AND IT RUNS EXACTLY ONCE.** An
  // existing store already holds its flush history, so the cutoff is derivable
  // from it right here -- and after this statement nothing derives a cutoff
  // from `event_log` ever again. The alternative, leaving the read in place as
  // a fallback, is two homes for one value.
  //
  // **A STORE WITH NO FLUSHES STILL GETS ITS ROW, CARRYING NULL.** `MAX` over
  // an empty set is NULL and an aggregate with no GROUP BY still returns one
  // row, so the singleton exists either way -- and NULL is the correct reading
  // of a project that has never flushed, not a gap to paper over.
  //
  // The fraction is dropped to match what the cutoff is compared against: the
  // log stamps milliseconds, `Thread.completed` is a date, and a cutoff is a
  // value people read and retype.
  //
  // **BOTH STATEMENTS ARE REPLAY-SAFE, AND THAT IS A REQUIREMENT OF THE LADDER
  // RATHER THAN CAUTION.** A store can carry the CURRENT DDL and still be
  // stamped at an older version -- that is exactly the fixture
  // `a_store_stamped_by_an_earlier_draft_of_a_rung_is_walked_forward_not_refused`
  // builds -- so this rung meets a `project` table that the DDL already
  // created. A bare `CREATE TABLE` refused with `table project already exists`
  // and the whole walk-forward failed on it. `OR IGNORE` guards the row for the
  // same reason: the singleton may already be there, and a rung that cannot be
  // replayed is a rung that strands every store that took a partial ladder.
  "CREATE TABLE IF NOT EXISTS project (
     id INTEGER PRIMARY KEY CHECK (id = 1),
     todo_watermark TEXT,
     updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
   );
   INSERT OR IGNORE INTO project (id, todo_watermark)
     SELECT 1, substr(MAX(ts), 1, 19) || 'Z' FROM event_log WHERE op = 'todo.flush';",
  ),
  (
    15,
    // 14 -> 15: ST0066. `tests` gains a home for the AT's fiat record.
    //
    // **A COLUMN RATHER THAN ZERO DDL, AND THE RULING'S PREMISE IS WHY THE
    // DIFFERENCE IS EASY TO MISS.** hv's 2026-08-28 ruling records that ATs
    // cost zero DDL, measured on `tests.status` being unconstrained TEXT with
    // no CHECK -- correct, and it is a measurement about the STATUS VALUE.
    // `fiat` is a separate FIELD, forced there because `AtStatus` derives
    // `Copy` and async-graphql `Enum` and so cannot carry a payload the way
    // `AcState::Fiat` does. A criterion's record rides inside `criteria.state`;
    // an AT's had nowhere to live.
    //
    // **A REBUILD RATHER THAN AN `ALTER TABLE ADD COLUMN`, AND THE TEST THAT
    // FORCED IT IS THE ONE WORTH READING.** The obvious form is
    // `ALTER TABLE tests ADD COLUMN fiat TEXT`, it is additive and nullable, and
    // it would have been the FIRST rung on this ladder that is not a rebuild.
    // `a_store_stamped_by_an_earlier_draft_of_a_rung_is_walked_forward_not_refused`
    // reds it with `duplicate column name: fiat`, because that fixture builds
    // its store from the CURRENT `DDL` -- which already carries the column --
    // and then stamps an older version onto it. Every sibling rung survives that
    // fixture by recreating the table outright rather than depending on what the
    // old one had.
    //
    // **The load-bearing detail is that the `SELECT` never names `fiat`.** That
    // is what makes one statement correct against both worlds: a real store at
    // 14 has no such column to carry, and the test's store has one whose
    // contents are irrelevant because nothing has been able to write it yet.
    // `written_at` is not carried for the same reason its siblings do not carry
    // it -- a record timestamp is per-machine and re-stamped by a rebuild by
    // design.
    "CREATE TABLE tests_v15 (
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
       fiat TEXT,
       PRIMARY KEY (thread_id, id)
     );
     INSERT INTO tests_v15 (thread_id, id, kind, file, prose, covers, status, note, legacy)
       SELECT thread_id, id, kind, file, prose, covers, status, note, legacy FROM tests;
     DROP TABLE tests;
     ALTER TABLE tests_v15 RENAME TO tests;",
  ),
  (
    16,
    // 15 -> 16: ST0066 again, for the ST and WP kinds. `threads` and `wps` each
    // gain a home for a fiat record.
    //
    // **TWO TABLES IN ONE RUNG BECAUSE IT IS ONE RULING.** hv's D1 declared
    // `st.fc` and `wp.fc` together, from the same from-states as their `done`
    // siblings and landing on the same states. Splitting them across two rungs
    // would let a store exist at a version where a thread can be fiat-closed and
    // its cascade has nowhere to land -- a half-applied ruling, reachable only
    // by interrupting a migration, and the ladder is one-way so it would stay
    // there.
    //
    // **REBUILDS RATHER THAN `ALTER TABLE ADD COLUMN`, for the reason rung 15
    // records**: `a_store_stamped_by_an_earlier_draft_of_a_rung_is_walked_forward_not_refused`
    // builds its store from the CURRENT `DDL`, which already carries both
    // columns, then stamps an older version onto it -- so an `ADD COLUMN` reds
    // with `duplicate column name: fiat`. Neither `SELECT` names `fiat`, which
    // is what makes one statement correct against both worlds: a real store at
    // 15 has no such column, and the test's store has one whose contents are
    // irrelevant because nothing has been able to write it yet.
    //
    // `created_at` and `updated_at` ARE carried on `threads`, unlike `written_at`
    // on the child tables. They are entity dates rather than record stamps: when
    // this thread was created is a fact about the thread, and re-stamping it to
    // the moment of a migration would silently rewrite the age of every thread
    // in the estate.
    "CREATE TABLE threads_v16 (
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
       body TEXT NOT NULL DEFAULT '',
       preamble TEXT NOT NULL DEFAULT '',
       fiat TEXT,
       created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
       updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
     );
     INSERT INTO threads_v16 (id, title, slug, status, status_reason, created, completed, acceptance, objective, context, body, preamble, created_at, updated_at)
       SELECT id, title, slug, status, status_reason, created, completed, acceptance, objective, context, body, preamble, created_at, updated_at FROM threads;
     DROP TABLE threads;
     ALTER TABLE threads_v16 RENAME TO threads;
     CREATE TABLE wps_v16 (
       thread_id TEXT NOT NULL REFERENCES threads (id) ON DELETE CASCADE,
       seq INTEGER NOT NULL,
       title TEXT NOT NULL,
       scope TEXT,
       scope_legacy TEXT,
       status TEXT NOT NULL,
       status_reason TEXT,
       objective TEXT NOT NULL,
       body TEXT NOT NULL,
       preamble TEXT NOT NULL DEFAULT '',
       written_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
       fiat TEXT,
       PRIMARY KEY (thread_id, seq)
     );
     INSERT INTO wps_v16 (thread_id, seq, title, scope, scope_legacy, status, status_reason, objective, body, preamble)
       SELECT thread_id, seq, title, scope, scope_legacy, status, status_reason, objective, body, preamble FROM wps;
     DROP TABLE wps;
     ALTER TABLE wps_v16 RENAME TO wps;",
  ),
  (
    17,
    // 16 -> 17: `threads.revision`, a per-record write counter. It shipped
    // named as issue 0206's compare-and-swap token; see the DDL for why it is
    // not one -- it is not monotonic across a sync -- and what replaced it. The
    // RUNG is unaffected either way: the column is still there and still counts
    // writes between rebuilds.
    //
    // **NO TABLE REBUILD, because the default is CONSTANT.** Every rung above
    // that added a column rebuilt its table, and the reason was always the
    // same: SQLite refuses `ADD COLUMN` with a non-constant default, and those
    // columns defaulted to `strftime(...)`. `0` is constant, so the cheap form
    // is available here and the rebuild would be ceremony -- which matters
    // beyond tidiness, since a rebuild carries a column list that the NEXT
    // rung's author has to remember not to name.
    //
    // **EXISTING ROWS START AT 0 AND THAT LOSES NOTHING.** The column did not
    // exist, so no write has ever been counted; 0 is the honest statement that
    // this store has recorded no revisions rather than a claim that no writes
    // happened. The first write through the change door takes every row it
    // touches to 1.
    "ALTER TABLE threads ADD COLUMN revision INTEGER NOT NULL DEFAULT 0;",
  ),
  (
    18,
    // 17 -> 18: `wps.status_legacy` (0100), the spelling v2 wrote for a
    // work-package status outside the vocabulary. The migrator defaulted it to
    // `not-started` and kept nothing of what v2 said, while `scope_legacy`
    // beside it carried the same class of value verbatim.
    //
    // No table rebuild: NULL is a constant default, as rung 17's `0` is.
    // Existing rows arrive NULL, which is true of every store written before
    // this -- none holds a carried status, because no migrator carried one.
    "ALTER TABLE wps ADD COLUMN status_legacy TEXT;",
  ),
  (
    19,
    // 18 -> 19: `file_index` gains the search index's four columns (ST0069
    // WP-18). See the DDL for what each NULL means; the short form is that the
    // five columns above them are the change detector's, over the canon corpus
    // `sync::scan` walks, and these four are the index's, over the repository.
    //
    // **A REBUILD RATHER THAN FOUR `ADD COLUMN`s, FOR RUNG 15's REASON AND NOT
    // BY PREFERENCE.** The cheap form was written first and
    // `a_store_stamped_by_an_earlier_draft_of_a_rung_is_walked_forward_not_
    // refused` red it with `duplicate column name: corpus`: that fixture builds
    // its store from the CURRENT `DDL`, which already carries the columns, and
    // stamps an older version onto it. Rungs 17 and 18 take the cheap form only
    // because rung 16 rebuilds the two tables they alter, which puts those
    // tables back to their pre-17 shape inside the same walk; `file_index` was
    // last rebuilt at rung 3, so nothing does that for it.
    //
    // **The load-bearing detail is that the `SELECT` never names the four.** A
    // real store at 18 has no such columns to carry, and the fixture's store
    // has them holding nothing, because nothing has been able to write them.
    //
    // **EVERY CARRIED ROW ARRIVES WITH FOUR NULLS AND THAT IS THE TRUE
    // DESCRIPTION OF IT.** This table is DERIVED, so dropping it outright was
    // available and loses nothing durable -- and it would make the next scan
    // report every file in the project as changed. Carrying the rows keeps the
    // change detector's answer about what moved, and the index fills its own
    // columns on its first reconcile rather than inheriting a guess from here.
    "CREATE TABLE file_index_v19 (
       path TEXT PRIMARY KEY,
       size INTEGER NOT NULL,
       mtime TEXT NOT NULL,
       sha256 TEXT NOT NULL,
       state TEXT NOT NULL,
       findings TEXT NOT NULL,
       corpus TEXT,
       lang TEXT,
       indexed_sha256 TEXT,
       skipped_reason TEXT,
       created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
       updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
     );
     INSERT INTO file_index_v19 (path, size, mtime, sha256, state, findings)
       SELECT path, size, mtime, sha256, state, findings FROM file_index;
     DROP TABLE file_index;
     ALTER TABLE file_index_v19 RENAME TO file_index;",
  ),
  (
    20,
    // 19 -> 20: the index gets its own table and gives back the four columns
    // rung 19 put on `file_index`.
    //
    // **ONE WRITER PER TABLE, MEASURED RATHER THAN PREFERRED.**
    // `replace_file_index` deletes every row the sync scan did not produce, so
    // the first row a reconcile wrote for a source file vanished on the next
    // sync. The two corpora are not nested either way -- the canon corpus
    // carries the rendered views and the extract, which the index corpus
    // deliberately excludes -- so neither writer can be handed the other's
    // delete rule, and the NOT NULL columns made it worse: an index row for a
    // source file would have had to supply a `sha256` for a file the cap says
    // not to read, and a `state` from a vocabulary that is about canon ingest.
    //
    // **RUNG 19 IS NOT EDITED, AND THAT IS THE LADDER'S OWN RULE.** A version
    // is a claim about SHAPE, so once a store has run a rung, changing what it
    // produces needs a NEW rung; 19 shipped, so 20 undoes it in the open.
    //
    // A rebuild rather than `DROP COLUMN`, for the reason rung 19 gives about
    // the fixture that stamps an old version onto the current DDL -- there the
    // columns are already gone, and `DROP COLUMN` would fail on a table that
    // never had them. The `SELECT` names only what survives.
    //
    // The new table starts EMPTY and that is correct: no reconcile has ever
    // run, so there is nothing to carry, and an empty index is the honest
    // description of a store that has never built one.
    "CREATE TABLE IF NOT EXISTS index_file (
       path TEXT PRIMARY KEY,
       corpus TEXT NOT NULL,
       lang TEXT,
       size INTEGER NOT NULL,
       mtime TEXT NOT NULL,
       indexed_sha256 TEXT,
       skipped_reason TEXT,
       created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
       updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
     );
     CREATE TABLE file_index_v20 (
       path TEXT PRIMARY KEY,
       size INTEGER NOT NULL,
       mtime TEXT NOT NULL,
       sha256 TEXT NOT NULL,
       state TEXT NOT NULL,
       findings TEXT NOT NULL,
       created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
       updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
     );
     INSERT INTO file_index_v20 (path, size, mtime, sha256, state, findings)
       SELECT path, size, mtime, sha256, state, findings FROM file_index;
     DROP TABLE file_index;
     ALTER TABLE file_index_v20 RENAME TO file_index;",
  ),
  (
    21,
    // 20 -> 21: the source half of the search index, `src_sections`.
    //
    // A new table, which is the easy rung for the reason rung 12's note gives:
    // an empty table is a correct representation of a store that has never
    // indexed a line of source, and it is the only correct one. There is
    // nothing to back-fill, because nothing has ever read a source file.
    //
    // **THE STATEMENT IS REPEATED FROM THE DDL RATHER THAN SHARED, which the
    // ladder's shape requires**: a rung is a claim about the shape at a
    // VERSION, so a rung that read the current DDL would silently change what
    // it did the next time the DDL moved.
    "CREATE VIRTUAL TABLE IF NOT EXISTS src_sections USING fts5 (
       path UNINDEXED,
       seq UNINDEXED,
       start_line UNINDEXED,
       end_line UNINDEXED,
       kind UNINDEXED,
       name,
       name_parts,
       body,
       tokenize = 'unicode61'
     );",
  ),
  (
    22,
    // 21 -> 22: `symbols`, the structural half of the search index.
    //
    // A new table and its two indexes, which is the easy rung for rung 12's
    // reason: an empty table is the correct and only representation of a store
    // that has never parsed a source file.
    //
    // **THE INDEXES ARE PART OF THE SHAPE AND SO THEY ARE PART OF THE RUNG.**
    // A store that reached this version without them would answer the same
    // questions by scanning, correctly and slowly, and nothing would say why.
    "CREATE TABLE IF NOT EXISTS symbols (
       path TEXT NOT NULL,
       lang TEXT NOT NULL,
       name TEXT NOT NULL,
       kind TEXT NOT NULL,
       start_line INTEGER NOT NULL,
       end_line INTEGER NOT NULL,
       created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
       updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
     );
     CREATE INDEX IF NOT EXISTS symbols_by_name ON symbols (name);
     CREATE INDEX IF NOT EXISTS symbols_by_path ON symbols (path);",
  ),
  (
    23,
    // 22 -> 23: `embeddings`, the semantic tier's vectors.
    //
    // A new table, the easy rung. Nothing writes it in this cut -- the tier is
    // staged and its chunker is a later package -- so an empty table is the
    // honest description of every store that reaches this version, and there is
    // nothing to back-fill.
    "CREATE TABLE IF NOT EXISTS embeddings (
       chunk_id TEXT NOT NULL,
       model TEXT NOT NULL,
       dims INTEGER NOT NULL,
       vector BLOB NOT NULL,
       created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
       updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
       PRIMARY KEY (chunk_id, model)
     );",
  ),
  (
    24,
    // 23 -> 24: the coordination entities (D30, WP-14).
    //
    // Three new tables, the easy rung: no existing column changes shape and
    // there is nothing to back-fill, because the boards are still hand-authored
    // markdown on disk at this version. **An empty set of tables is the honest
    // description of every store that reaches 24** -- the live board migrates
    // as a separate deliberate act (AC-14.9), at a cutover, and not as a side
    // effect of opening a store.
    //
    // `recorded_at` carries no DEFAULT here for the same reason it carries none
    // in the DDL: AC-14.11 refuses a DB-side default as the mechanism, because
    // a disk-to-db resync re-inserting rows would re-stamp them and rewrite
    // history indistinguishably from a correct value.
    "CREATE TABLE IF NOT EXISTS wb_node (
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
       updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
     );
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
       updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
     );
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
       updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
     );
     CREATE INDEX IF NOT EXISTS wb_item_by_node ON wb_item (node, kind, seq);
     CREATE INDEX IF NOT EXISTS wb_message_by_recipient ON wb_message (recipient, id);",
  ),
  (
    25,
    // 24 -> 25: `wb_node.migrated_at` (0317).
    //
    // **A BOARD WRITE LANDS A RENDER, SO ON A NODE WHOSE BOARD IS STILL ITS
    // MARKDOWN IT WOULD ERASE THE BOARD.** The column says which nodes are past
    // that point, and every board write refuses a node that is not.
    //
    // **THE BACK-FILL MARKS THE NODES THAT ALREADY CARRY MIGRATED CONTENT**: a
    // header claim in `authored_at`, or any item or message row. Registration
    // writes none of those, so a node registered from its header and never
    // carried stays null and keeps its hand-authored board. The value is the
    // rung's own clock read, because when an earlier binary migrated a node was
    // never recorded and inventing it would be the fabricated stamp.
    //
    // **A REBUILD RATHER THAN `ALTER TABLE ADD COLUMN`, for rung 15's reason.**
    // The cheap form was written first and
    // `a_store_stamped_by_an_earlier_draft_of_a_rung...` red it with `duplicate
    // column name: migrated_at`: that fixture builds the current DDL and stamps
    // an older version, and nothing rebuilds `wb_node` after rung 24. The
    // `SELECT` names only the columns rung 24 made, so it reads either shape.
    "CREATE TABLE wb_node_rebuilt (
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
     INSERT INTO wb_node_rebuilt (moniker, name, role, session_id, heartbeat_at, status, focus,
       claims, recorded_at, authored_at, updated_at, migrated_at)
       SELECT moniker, name, role, session_id, heartbeat_at, status, focus, claims, recorded_at,
         authored_at, updated_at,
         CASE WHEN authored_at IS NOT NULL
           OR EXISTS (SELECT 1 FROM wb_item WHERE wb_item.node = wb_node.moniker)
           OR EXISTS (SELECT 1 FROM wb_message WHERE wb_message.recipient = wb_node.moniker)
         THEN strftime('%Y-%m-%dT%H:%M:%fZ', 'now') END
       FROM wb_node;
     DROP TABLE wb_node;
     ALTER TABLE wb_node_rebuilt RENAME TO wb_node;",
  ),
];

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
  /// An in-memory store was asked for the read-only door `intent search --sql`
  /// reads through. There is no file to open a second connection to, and a
  /// fresh in-memory database opened beside it would be EMPTY -- every query
  /// answering zero rows, which is indistinguishable from a true miss.
  #[error("this store is in memory, so there is no file to open a read-only connection to")]
  NoReadOnlyDoor,
  /// The store holds a schema this binary does not speak. **Refused at open**,
  /// which is the whole point: the alternative is answering questions from a
  /// database whose shape disagrees with the queries.
  #[error("{store} holds schema version {found}; this build of intent speaks {expected}")]
  SchemaMismatch {
    store: String,
    found: i32,
    expected: i32,
  },
  /// The store predates schema versioning altogether.
  #[error("{store} predates schema versioning and does not record which shape it holds")]
  SchemaUnstamped { store: String },
  /// A migration rebuilt a table and left rows pointing at a parent that is no
  /// longer there. Foreign keys are off for the rebuild and re-checked inside
  /// the same transaction, so this is the check firing and rolling the rung
  /// back rather than the damage going unnoticed.
  #[error(
    "migrating the runtime store left {violations} row(s) referencing a parent that is not there"
  )]
  MigrationLeftDanglingRows { violations: i64 },
  /// **A CREATE LANDED ON A KEY THAT ALREADY EXISTS** (issue 0131, hv ruled
  /// 2026-08-28: a verb named `add`/`new` must FAIL on an existing key rather
  /// than replace it).
  ///
  /// This is a REFUSAL, not a crash. It is raised from the UNIQUE constraint
  /// itself rather than from a preceding `SELECT`, so there is no window
  /// between the check and the write for a second writer to enter -- which is
  /// the whole defect: two nodes filed concurrently, both were told
  /// `created: ...0126.json`, and one filing reached neither the store nor the
  /// extract with nothing saying so.
  #[error("{kind} {key} already exists, and a create must not replace it")]
  CreateHitAnExistingKey { kind: EntityKind, key: String },
  /// **THE RECORD THIS WRITE WAS DERIVED FROM HAS MOVED** (issue 0206, vc
  /// ruled 2026-09-01: refuse and name, never retry).
  ///
  /// A canon verb is a read-modify-write over a snapshot loaded when the facade
  /// opened. Two sessions editing DIFFERENT fields of one thread each write the
  /// whole record back, and the second silently carries the first's field at
  /// its pre-edit value. Measured on the shipping binary: **9 of 15 concurrent
  /// pairs lost a write, every one of them `rc=0` with no error text.**
  ///
  /// **RAISED INSIDE THE MUTATION'S OWN TRANSACTION**, which is what makes it a
  /// compare-and-swap rather than a check. Comparing in the facade before the
  /// call would narrow the window and leave it open; shipping that as a CAS
  /// would overclaim.
  ///
  /// **IT CARRIES NO WRITE COUNT, AND THE OMISSION IS DELIBERATE.** The obvious
  /// field is "how many writes landed under you", which needs the revision this
  /// session LOADED at -- state the facade would have to carry and keep in step
  /// at five assignment sites. The count is worth less than that invariant
  /// costs, and a number nobody can derive is how a message starts lying.
  #[error(
    "{kind} {key} changed while this command was running, and this write was derived from the copy it held before that"
  )]
  RecordMovedUnderTheWrite { kind: EntityKind, key: String },
}

/// Which estate a key belongs to.
///
/// **An enum rather than a string, because the facade MATCHES on it** to lift
/// this refusal into its own per-entity vocabulary (`ThreadExists`,
/// `IssueExists`) -- and a string would make a typo there a silent fall-through
/// to the generic arm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityKind {
  Thread,
  Issue,
}

impl std::fmt::Display for EntityKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(match self {
      Self::Thread => "thread",
      Self::Issue => "issue",
    })
  }
}

impl crate::remedy::Remedy for StoreError {
  /// What the operator should DO. Distinct per variant -- a remedy that fits
  /// two causes is telling the operator to guess which one they hit.
  ///
  /// The two schema variants are separated because the ACTION differs: one is
  /// "your build is out of step with your database", which is recoverable by
  /// moving either end, and the other is "nothing knows what this database
  /// is", which is not recoverable by the tool at all. Collapsing them would
  /// promise a migration for the case that cannot have one.
  fn remedy(&self) -> String {
    match self {
      // **IT NAMES THE FILE AND BOTH VERSIONS, AND THEN POINTS AT `--version`
      // FOR THE BUILD RATHER THAN INLINING IT.** A refusal that says only "the
      // runtime store" leaves an operator with two projects to guess between,
      // and a version number alone does not say WHICH BINARY is the old one --
      // `intent --version` names the commit the running binary was built from,
      // which is the question, and it now has exactly one home. Inlining the
      // commit here would be a second.
      Self::SchemaMismatch {
        store,
        found,
        expected,
      } if found > expected => format!(
        "{store} was written by a NEWER intent than the one you are running -- upgrade intent \
         rather than migrating the store down; it holds version {found} and this build speaks \
         {expected}. `intent --version` names the build you are on"
      ),
      // **IT NAMES THE CONDITION, NOT A COMMAND, because there is no command
      // that turns an in-memory store into a file.** A remedy proposing one
      // would send an operator looking for a flag that does not exist.
      Self::NoReadOnlyDoor => "the read-only door reads a store on disk, and this process opened one in memory. Run the command in a project, where the store is a file".to_string(),
      Self::CreateHitAnExistingKey { kind, key } => format!(
        "nothing was written and nothing was replaced. If you meant to CREATE, re-run and take the \
         next free key. If you meant to CHANGE {kind} {key}, use the verb that changes it -- a \
         create that silently overwrote would have destroyed the record already there, which is \
         what this refuses"
      ),
      // **NAMES THE OTHER WRITE AS A FACT ABOUT THE WORLD, NOT AS A FAULT.**
      // Two people working one thread is what this project does; the remedy is
      // to re-read and re-apply, and it says so in those words rather than
      // implying the operator did something wrong.
      //
      // **IT DOES NOT OFFER A RETRY FLAG, DELIBERATELY** (vc, 2026-09-01). An
      // automatic retry re-derives the edit from a snapshot the operator never
      // saw, which is the original defect wearing a success message.
      Self::RecordMovedUnderTheWrite { kind, key } => format!(
        "nothing was written. Somebody else changed {kind} {key} while this command was running -- \
         re-run it and it will read the current record first. If you are running two sessions \
         against one project, this is that, working"
      ),
      Self::SchemaMismatch { store, .. } => format!(
        "run `intent doctor` -- it names {store}'s version against this build's, and reports \
         whether a migration for it has shipped"
      ),
      Self::SchemaUnstamped { store } => {
        // NO RECOVERY COMMAND, because there is none and inventing one is
        // worse than admitting it. The database was written on the day the
        // schema moved several times without a stamp, so its shape is not
        // knowable and a migration cannot be dispatched for it. What CAN be
        // said honestly is where the work is: the committed extract carries
        // everything that was ever synced out (D34), which for a project under
        // version control is a `git status` away from being checked.
        format!(
          "{store} cannot be migrated -- nothing recorded which shape it holds. Check what your \
           committed extract carries before replacing it; anything never synced out of this store \
           exists only here"
        )
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

/// One row out of [`Store::search_hits`]: the section, where the engine
/// matched inside its body, and the rank that ordered it.
///
/// **NAMED RATHER THAN A TUPLE** because the rank made a third anonymous
/// position, and `(section, at, rank)` states nothing at a call site about
/// which is which.
pub struct SearchRow {
  pub section: DocSection,
  /// Byte offset of the first match within `section.body`, when the engine
  /// located one (issue 0195).
  pub at: Option<usize>,
  /// FTS5's own rank for this row. Negative, lower is better, and comparable
  /// only against other rows of the same query.
  pub rank: f64,
}
/// One source row a lexical query matched.
#[derive(Debug, Clone)]
pub struct SourceRow {
  pub section: crate::index::source::Section,
  /// Where in the body the match begins, when the highlighter could say.
  pub at: Option<usize>,
  /// FTS5's own rank, carried rather than recomputed, and comparable only
  /// within this tier.
  pub rank: f64,
}

/// The mark [`Store::search_hits`] asks `highlight()` to put before each
/// matched token. Private-use, so authored prose does not carry it.
const MATCH_MARK: char = '\u{E000}';

/// Which half of the prose table a write owns.
///
/// **THE PROSE TABLE IS THE ONE PLACE SEVERAL WRITERS SHARE A TABLE, and they
/// can because the row says which is which.** `owner_type` is `file` for the
/// repository's own prose, `wb_node` for a whiteboard node's carried history
/// and an entity kind for canon's, so each writer's delete-missing names its
/// own half in SQL. See [`Store::replace_doc_sections`] for why the index's
/// other tables are not arranged this way.
///
/// **THE WHITEBOARD HALF IS SCOPED TO ONE NODE AND THE OTHER TWO ARE NOT**,
/// because it is written one node at a time by `wb migrate` while the others
/// are re-derived whole from a model. A migration of dc that emptied cc's
/// history would be the estate-wide write the verb's own scope exists to
/// prevent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProseHalf<'a> {
  Canon,
  Files,
  Whiteboard(&'a str),
}

/// One `doc_sections` row, from its first seven columns in declaration order.
fn section_from(row: &rusqlite::Row<'_>) -> rusqlite::Result<DocSection> {
  Ok(DocSection {
    owner_type: row.get(0)?,
    owner_id: row.get(1)?,
    file: row.get(2)?,
    seq: row.get::<_, i64>(3)? as u32,
    heading: row.get(4)?,
    level: row.get::<_, i64>(5)? as u8,
    body: row.get(6)?,
  })
}

/// The one upsert into `file_index`, shared by the whole-index replace and the
/// per-projection record. `created_at` is the row's own, so a conflict keeps it.
/// THE ONE PLACE AN INDEX ROW BECOMES A ROW, so the full rebuild and the
/// incremental pass cannot disagree about what a column holds -- including the
/// `COALESCE` that keeps a hash a writer did not supply.
fn upsert_index_row(conn: &rusqlite::Connection, r: &crate::index::Row) -> Result<(), StoreError> {
  conn.execute(
    "INSERT INTO index_file (path, corpus, lang, size, mtime, indexed_sha256, skipped_reason)
       VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
     ON CONFLICT (path) DO UPDATE SET
       corpus = excluded.corpus,
       lang = excluded.lang,
       size = excluded.size,
       mtime = excluded.mtime,
       -- **PRESERVED WHERE THE WRITER DID NOT SUPPLY ONE.** A survey
       -- stats files and does not read them, so it arrives with this
       -- unset for every row; taking it literally would erase, on every
       -- reconcile, the record of what the index actually holds. NULL
       -- from a writer means `I do not know`, and the column already has
       -- a way to say `nothing`: a row that has never been read has NULL
       -- here and no writer has claimed otherwise.
       indexed_sha256 = COALESCE(excluded.indexed_sha256, index_file.indexed_sha256),
       skipped_reason = excluded.skipped_reason,
       updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')",
    params![
      r.path,
      r.corpus,
      r.lang,
      r.size as i64,
      r.mtime,
      r.indexed_sha256,
      r.skipped_reason,
    ],
  )?;
  Ok(())
}

/// THE ONE PLACE A PROSE SECTION BECOMES A ROW.
fn insert_doc_section(conn: &rusqlite::Connection, s: &DocSection) -> Result<(), StoreError> {
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
  Ok(())
}

/// THE ONE PLACE A SOURCE SECTION BECOMES A ROW.
fn insert_src_section(
  conn: &rusqlite::Connection,
  s: &crate::index::source::Section,
) -> Result<(), StoreError> {
  conn.execute(
    "INSERT INTO src_sections (path, seq, start_line, end_line, kind, name, name_parts, body)
       VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
    params![
      s.path,
      s.seq as i64,
      s.start_line as i64,
      s.end_line as i64,
      s.kind,
      s.name,
      s.name_parts,
      s.body,
    ],
  )?;
  Ok(())
}

fn upsert_file_entries(
  tx: &rusqlite::Transaction<'_>,
  entries: &[FileEntry],
) -> Result<(), StoreError> {
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
  Ok(())
}

pub struct Store {
  conn: Connection,
  /// Where this store LIVES, or `None` for an in-memory one.
  ///
  /// **KEPT BECAUSE A SECOND CONNECTION NEEDS IT** (WP-17). `init` already took
  /// the path and used it only to name the store in a refusal, so the one fact
  /// a reader-opener requires was read and dropped. The read-only door asks
  /// THIS type to open its own second connection rather than rebuilding the
  /// path itself: a second opener is a divergent copy of how this store is
  /// opened, and the two would disagree the first time either moved.
  path: Option<std::path::PathBuf>,
  /// How many loads-from-canon are open on this store right now.
  ///
  /// **Re-entrancy, because the operation that must be recorded is not the one
  /// that writes.** `Facade::sync_from_disk` calls `ingest::resync`, which
  /// rebuilds the store, and then does more work that can still refuse. If the
  /// inner call closed the record, the outer refusal would land on a store
  /// whose history said `succeeded` -- the exact silence this table exists to
  /// break, arrived at through the machinery built to break it.
  ///
  /// So the OUTERMOST open load owns the row, and an inner one joins it.
  ingest_depth: u32,
  /// The row the outermost open load is recording into.
  ingest_attempt: Option<i64>,
}

/// Everything ONE mutation changes, written in ONE transaction.
///
/// Grouped into a struct rather than passed as six arguments because the whole
/// point is that they are indivisible: under D01 as reversed, the DB is the
/// truth, so a mutation that recorded the entity and lost its envelope would
/// be an unaudited change to the source of truth -- and AC-04.5 requires the
/// envelope end to end. Before this they were three separate calls, one of
/// which did not open a transaction at all.
/// How a backup attempt ended.
///
/// Named rather than a `bool`, for the reason [`Stamp`] is: `finish(id, true)`
/// at a call site says nothing about which world it is in, and the two worlds
/// are "a snapshot exists" and "a snapshot was supposed to exist".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotOutcome {
  Ok,
  Failed,
}

/// One recorded backup attempt.
///
/// `path` and `bytes` are optional because a row exists from the moment the
/// attempt STARTS -- an attempt that died before producing a file is a real
/// record with nothing to point at, and that is the state this table was added
/// to make visible.
#[derive(Debug, Clone)]
pub struct SnapshotRecord {
  pub id: i64,
  pub path: Option<String>,
  pub bytes: Option<u64>,
  /// `attempted` · `ok` · `failed`.
  pub outcome: String,
  pub detail: Option<String>,
  pub taken_at: String,
}

/// Failed backup attempts since the newest good snapshot.
///
/// **A COUNT AND THE NEWEST REASON, WHICH IS WHAT A REPORT NEEDS AND ALL OF
/// IT.** The count says whether this is a blip or a pattern; the newest detail
/// says what to fix. Every failure's detail would be a log, and `intent backup
/// --list` is already the place that prints one.
///
/// **`attempts: 0` IS A VALUE, NOT AN ABSENCE.** *Nothing has failed since the
/// last good backup* is a real and useful answer -- it is what a healthy store
/// says -- so this is not an `Option`. The distinction this subsystem's surface
/// notes draw at `keys.4.note` is between a setting that is unset and one set
/// to zero; here zero was measured.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailedAttempts {
  /// How many attempts have failed since the newest good snapshot -- or ever,
  /// if there has never been a good one.
  pub attempts: u32,
  /// What the newest of those failures recorded, if it recorded anything.
  ///
  /// `None` when there are no failures, and ALSO when the newest failure
  /// stored no detail. **Those are the same to a reporter** -- both mean
  /// there is no reason to print -- which is why they are not distinguished
  /// here; `attempts` already carries whether any failure happened.
  pub newest_detail: Option<String>,
}

/// How a load from canon ended (AC-03.13).
///
/// Named rather than a `bool` for the reason [`Stamp`] and [`SnapshotOutcome`]
/// are: the two worlds here are "the store now holds what canon holds" and "the
/// store is older than the canon beside it", and a `true` at a call site says
/// neither.
///
/// **There is no `Attempted` variant, and its absence is the mechanism.** That
/// value is written by [`Store::begin_ingest`] and spelled by no caller, so it
/// can only ever mean "nothing reported an outcome" -- a crash, a panic, a
/// process killed mid-rebuild. A caller able to write it could report the
/// unfinished state deliberately, and then an unfinished state would stop being
/// evidence of anything.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IngestOutcome {
  Succeeded,
  Refused,
}

impl IngestOutcome {
  fn as_str(self) -> &'static str {
    match self {
      Self::Succeeded => "succeeded",
      Self::Refused => "refused",
    }
  }
}

/// One recorded load of canon into this store.
///
/// `detail` is optional because a row exists from the moment the load STARTS,
/// and an attempt that died before anything could describe it is a real record
/// with nothing to say -- which is the state this table was added to make
/// visible.
#[derive(Debug, Clone)]
pub struct IngestRecord {
  pub id: i64,
  /// `attempted` · `succeeded` · `refused`.
  pub outcome: String,
  pub detail: Option<String>,
  pub started_at: String,
  pub updated_at: String,
}

impl IngestRecord {
  /// **The single home of what counts as a finished, accepted load.**
  ///
  /// Spelled once rather than at each reader, so the egest guard and `doctor`
  /// cannot come to disagree about whether `attempted` is good enough.
  ///
  /// It tests for SUCCESS rather than against failure, and that is the whole
  /// safety of it: `attempted`, and any string a future rung might add, read
  /// false. A `!= "refused"` test would call a crashed load healthy, and a
  /// crashed load leaves exactly the stale store AC-03.13 is about.
  pub fn succeeded(&self) -> bool {
    self.outcome == IngestOutcome::Succeeded.as_str()
  }
}

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

/// What the database stored for one issue's DOMAIN dates.
///
/// **Separate from [`ThreadDates`] rather than a generic pair, because the two
/// second dates are different facts.** A thread's `completed` and an issue's
/// `closed` are both "the day it stopped being open" and they belong to
/// different entities with different keys, so one struct carrying `id: String`
/// would force an issue's number through a string and lose the type that makes
/// the call sites unmixable.
#[derive(Debug, Clone)]
pub struct IssueDates {
  pub number: u32,
  pub created: String,
  pub closed: Option<String>,
}

/// Every domain date a mutation's writes actually landed, by entity kind (D42).
///
/// **This exists because `issues` was the second entity to carry domain dates
/// and the channel only had room for the first.** `commit_mutation` returned
/// `Vec<ThreadDates>`, so an issue created by a mutation had no way to tell the
/// caller what date the database put on it -- and the caller renders the
/// committed extract from what it holds. The extract would have carried the
/// empty string it handed in while truth carried the date, which is truth and
/// its projection disagreeing on the one field neither of them can recompute.
#[derive(Debug, Clone, Default)]
pub struct StoredDates {
  pub threads: Vec<ThreadDates>,
  pub issues: Vec<IssueDates>,
  /// The stamp the database put on THIS mutation's event.
  ///
  /// **The third arm, and the doc above predicted it.** `issues` was the second
  /// entity to carry a domain date and the channel only had room for the first;
  /// a fiat close is the third case, and it is not an entity date at all -- it
  /// is a value nested inside a criterion's state, which no column can fill.
  ///
  /// [`Store::write_event`] has always RETURNED the stamp it wrote, precisely
  /// because "the caller has no other way to learn it and must never compute
  /// it". `commit_mutation` discarded that return for as long as nothing needed
  /// it. Carrying it here is what lets `FiatRecord.at` obey D42 by the same
  /// mechanism `threads.created` does, rather than by a clock read in Rust.
  ///
  /// **One stamp per mutation is the CORRECT semantics for a cascade, not a
  /// simplification of it.** A fiat close that reaches an ancestor and its
  /// children is one act at one instant recorded by one event, so every row it
  /// closes sharing that event's time is what actually happened.
  pub event_ts: String,
}

pub struct Mutation<'a> {
  pub threads: &'a [&'a Thread],
  pub issues: &'a [&'a Issue],
  pub removed_threads: &'a [String],
  pub removed_issues: &'a [u32],
  /// The entities this mutation brings into EXISTENCE, as opposed to changes
  /// it makes to entities that already have rows.
  ///
  /// Diffed by the caller against the canon it loaded, never declared by the
  /// verb -- see [`Door`]. Anything named here is written through
  /// [`Door::Create`], so the DATABASE decides, inside this transaction,
  /// whether the key was really free (issue 0131).
  pub created_threads: &'a [String],
  pub created_issues: &'a [u32],
  /// The stored records this mutation's changes were DERIVED FROM, for the
  /// entities it is CHANGING rather than creating (issue 0206).
  ///
  /// **THIS IS THE OTHER HALF OF [`Door`], and the split is exact.** A create
  /// asks the database whether the key was free; a change asks it whether the
  /// record still says what the writer thought it said. Anything named in
  /// `created_*` has no prior copy by definition and appears in neither of
  /// these.
  ///
  /// Diffed by the caller against the canon it loaded, never declared by the
  /// verb -- the same rule `created_threads` states, for the same reason.
  pub expected_threads: &'a [&'a Thread],
  pub expected_issues: &'a [&'a Issue],
  pub sections: &'a [DocSection],
  /// The events this mutation records, written inside its own transaction.
  ///
  /// **A SLICE RATHER THAN ONE, BECAUSE A FIAT CASCADE MOVES SEVERAL ENTITIES IN
  /// ONE HUMAN ACT AND EACH MOVE OWES AN EVENT** (vc's ruling, 2026-08-30). The
  /// alternative -- one event for the whole cascade -- is not merely less
  /// detailed: a consumer replaying the log would get a thread closed and
  /// children sitting in states nothing in the log explains, so **replay would
  /// produce an estate that differs from the real one.**
  ///
  /// **They ride in ONE transaction for the same reason the envelope and the rows
  /// always have.** N transactions would put a window between the ancestor's
  /// close and its children's, and a cascade interrupted there is exactly the
  /// state-without-an-event defect one layer along.
  ///
  /// The FIRST is the mutation's own subject; the rest are what it reached.
  pub envelopes: &'a [&'a Envelope],
  /// Project-level state this mutation also sets, inside the same transaction.
  ///
  /// **THE POINT IS THE WORD `also`.** AC-14.7 requires that a flush's history
  /// and its state cannot land separately, and until 2026-08-27 they could:
  /// `todo_flush` committed the event through this door and then wrote the
  /// watermark through a second, unwrapped statement afterwards. Two
  /// transactions, in that order, with a window between them -- so a failure
  /// there left a `todo.flush` in the log that no cutoff reflected, and
  /// AC-14.2 had deliberately removed the fallback that would have hidden it.
  pub project_state: ProjectStateEdit,
}

/// Whether a write is a CREATE or a CHANGE.
///
/// **NOT A READER OF THE OP STRING**, for exactly the reason [`ProjectStateEdit`]
/// is not: the op vocabulary already has two consumers that fail in opposite
/// directions, and inferring the door from `op.ends_with(".add")` would be a
/// third table to forget a member in. `intent st new` and `intent issues add`
/// are not the only ways a row comes into existence.
///
/// It is DIFFED against the canon the facade loaded -- an entity `next` holds
/// and the loaded canon does not is a create -- which is the same rule
/// `apply_with_state` already uses to decide what gets WRITTEN, and it is
/// chosen there over a caller-supplied list for a stated reason: a declaration
/// can name the wrong id, and comparing cannot forget.
///
/// **THE STALE-READ CASE RESOLVES THE RIGHT WAY, AND THAT IS THE POINT.** If
/// the loaded canon is out of date and the key was taken since, the diff still
/// says `Create`, the constraint still fires, and the caller gets a refusal --
/// which is the correct answer to "I thought this number was free". The
/// alternative reading of a stale canon is the defect itself.
///
/// The distinction is not cosmetic -- it selects the SQL. `Change` upserts,
/// because `issues close` clones canon, edits one field and writes the whole
/// row back. `Create` uses a plain INSERT, so a key that already exists raises
/// the UNIQUE constraint and the write is refused.
///
/// **WHY THE CONSTRAINT AND NOT A PRECEDING `SELECT`.** A check-then-write has
/// a window, and a window is the defect: issue 0131 records two nodes filing
/// concurrently, both told `created: ...0126.json`, one filing gone with
/// nothing reporting it. The constraint is evaluated by the database as part of
/// the insert, so there is no gap for a second writer to arrive in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Door {
  /// This write brings the row into existence. An existing key REFUSES.
  Create,
  /// This write changes a row that already exists (or restores one from the
  /// extract). Upserts.
  Change,
}

/// Project-level state a mutation carries alongside its rows.
///
/// **AN EXPLICIT PARAMETER RATHER THAN A THIRD READER OF THE OP STRING.** The
/// obvious implementation is for the mutation path to notice `op ==
/// "todo.flush"` and act, and that is exactly the shape vc's Highlander finding
/// F1 is about: the op vocabulary already has two consumers that fail in
/// opposite directions, and a third would be one more table to forget a member
/// in. The caller knows what it is doing; it says so.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectStateEdit {
  /// The ordinary case: rows and an event, nothing project-level.
  Unchanged,
  /// Set the DONE cutoff to the moment this transaction commits.
  ///
  /// **THE DATABASE STAMPS IT (D42), so no clock is read in this process** --
  /// the same rule the event log's `ts` DEFAULT follows, for the same reason.
  ///
  /// Second resolution rather than the log's milliseconds, because this value
  /// is compared against `Thread.completed`, which is a DATE, and is read and
  /// retyped by people. **It is not derived from the flush EVENT**: deriving it
  /// would put a cutoff back in the log, which is the one thing this table
  /// exists to stop.
  SetTodoWatermark,
}

impl Store {
  /// How long a contended write waits before it is refused (issue `0152`).
  ///
  /// **SET EXPLICITLY, AND SETTING IT TO THE NUMBER WE ALREADY HAD IS NOT A
  /// NO-OP.** `rusqlite` 0.32.1 calls `sqlite3_busy_timeout(db, 5000)` on every
  /// open, in `inner_connection.rs:119`. Nothing in this workspace set it,
  /// nothing documented it, and **no reader of this file could see it** -- so
  /// the estate had a five-second block in its write path that nobody chose.
  ///
  /// **IT PRODUCED A CONFIDENT WRONG ANSWER FROM A CORRECT SOURCE READ, WHICH
  /// IS HOW IT WAS FOUND.** Reading this module and observing that no
  /// `busy_timeout` is set anywhere leads to "a second writer fails
  /// immediately" -- wrong by five seconds, and only a constructed contended
  /// write corrected it. **A dependency's default is invisible to source
  /// review by construction**, so this class is found by driving the path or
  /// not at all. Measured 2026-08-30: a second writer waits 5.22s and is then
  /// refused cleanly; readers never block; the store is intact afterwards.
  ///
  /// **THE VALUE IS UNCHANGED DELIBERATELY.** Whether a shorter wait suits an
  /// interactive CLI better is a real question -- five silent seconds looks
  /// like a hang -- and it is a separate judgement `0152` explicitly does not
  /// pre-empt. This constant closes the defect that the number was UNSEEABLE,
  /// which had to come first, because the judgement cannot be made while the
  /// number cannot be read.
  ///
  /// **THERE IS NO TEST FOR THIS LINE AND THE REASON IS STRUCTURAL, NOT
  /// LAZINESS.** Two things make it unwitnessable while the value equals the
  /// inherited default. `busy_timeout` is PER-CONNECTION and is not persisted
  /// in the file, so no second connection can read back what this one set --
  /// a test can only observe the timeout by CONTENDING and timing the wait.
  /// And a timed wait cannot distinguish our 5000 from `rusqlite`'s 5000: the
  /// two implementations are behaviourally identical by construction, so any
  /// test would pass with this line deleted. **A test that passes under the
  /// mutation it exists to catch is the vacuous instrument, and writing one
  /// here would convert an honest gap into a false green.**
  ///
  /// **THE PROPERTY THIS LINE BUYS IS THEREFORE A DOCUMENTARY ONE, AND THAT IS
  /// WORTH SAYING PLAINLY**: a reader of this module can now see the number,
  /// and a `rusqlite` bump that moves the default can no longer move OUR
  /// behaviour silently. The first is what issue `0152` asked for; the second
  /// is the one a test could never have given us anyway.
  const BUSY_TIMEOUT_MS: i32 = 5000;

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
    Self::wait_for_contention(&conn)?;
    Self::init(conn, Some(path))
  }

  /// Apply the contention wait -- **the ONE place the constant is read**.
  ///
  /// Both connections take it and neither names the value, so the wait cannot
  /// acquire a second home (issue 0152, held by
  /// `contention_wait_is_chosen.rs`).
  fn wait_for_contention(conn: &Connection) -> Result<(), StoreError> {
    conn.pragma_update(None, "busy_timeout", Self::BUSY_TIMEOUT_MS)?;
    Ok(())
  }

  /// A SECOND connection to this same database, opened read-only.
  ///
  /// **THE DOOR `intent search --sql` READS THROUGH** (AC-17.1). It is a
  /// separate connection rather than the live one because the guarantee has to
  /// be structural: the live connection carries the estate's writes, and a
  /// `query_only` pragma toggled around a statement on it is a guarantee that
  /// lasts exactly as long as nobody returns early.
  ///
  /// **`query_only` IS SET ANYWAY, ON TOP OF THE READ-ONLY OPEN FLAG.** The
  /// flag refuses the write at the file layer and the pragma refuses it at the
  /// statement layer, and the door's contract is the statement layer -- an
  /// operator reading the refusal should meet the rule that was broken rather
  /// than an errno about a file.
  ///
  /// **AN IN-MEMORY STORE HAS NO SECOND DOOR AND SAYS SO.** There is no path to
  /// open, and a fresh in-memory database opened alongside would be an EMPTY
  /// one -- which would answer every query with zero rows and look exactly like
  /// a true miss.
  pub fn read_only_connection(&self) -> Result<Connection, StoreError> {
    let Some(path) = self.path.as_ref() else {
      return Err(StoreError::NoReadOnlyDoor);
    };
    let conn = Connection::open_with_flags(
      path,
      rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY | rusqlite::OpenFlags::SQLITE_OPEN_URI,
    )?;
    // The same wait the live connection takes, through the same applier: naming
    // the constant a second time here is the second home
    // `contention_wait_is_chosen.rs` exists to refuse, and it counts the name
    // rather than the reads -- so this comment must not spell it either.
    Self::wait_for_contention(&conn)?;
    conn.pragma_update(None, "query_only", true)?;
    Ok(conn)
  }

  /// An in-memory store, for tests.
  pub fn open_in_memory() -> Result<Self, StoreError> {
    Self::init(Connection::open_in_memory()?, None)
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
  fn init(mut conn: Connection, at: Option<&std::path::Path>) -> Result<Self, StoreError> {
    // **THE NAME OF THE THING BEING REFUSED, RESOLVED ONCE AT THE ONLY PLACE
    // THAT KNOWS IT.** Both schema refusals travel out of a process that has
    // long since forgotten which project it was in -- an operator with two
    // checkouts open reads "the runtime store" and has to guess.
    //
    // The in-memory rendering is currently UNREACHABLE and is written honestly
    // rather than omitted: an in-memory database is always fresh, so it takes
    // the create arm below and can reach neither refusal. That may stop being
    // true, and a `None` that panicked or lied would be worse than a sentence.
    let store = at.map_or_else(
      || "an in-memory store".to_string(),
      |p| p.display().to_string(),
    );
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
      0 => return Err(StoreError::SchemaUnstamped { store }),
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
          store,
          found,
          expected: SCHEMA_VERSION,
        });
      }
    }
    Ok(Self {
      conn,
      path: at.map(std::path::Path::to_path_buf),
      ingest_depth: 0,
      ingest_attempt: None,
    })
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
  /// `commit_mutation` (the mutation write path) both go through it, so the two
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
    door: Door,
  ) -> Result<(String, Option<String>), StoreError> {
    tx.execute("DELETE FROM tests WHERE thread_id = ?1", params![t.id])?;
    tx.execute("DELETE FROM criteria WHERE thread_id = ?1", params![t.id])?;
    tx.execute("DELETE FROM related WHERE thread_id = ?1", params![t.id])?;
    tx.execute(
      "DELETE FROM attachments WHERE thread_id = ?1",
      params![t.id],
    )?;
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
    // **THE CONFLICT CLAUSE IS THE WHOLE OF THE DOOR** (issue 0131, hv ruled
    // 2026-08-28). `Change` upserts, because every ordinary verb clones canon,
    // edits a field and writes the whole row back. `Create` omits the clause,
    // so an id that already exists raises the UNIQUE constraint and the
    // transaction is refused rather than quietly overwriting every column of a
    // thread somebody else is holding. (That sentence said "all twelve columns"
    // until ST0066 added a thirteenth -- a count in prose beside the list that
    // holds it.)
    let on_conflict = match door {
      Door::Change => {
        "ON CONFLICT (id) DO UPDATE SET
           title = excluded.title,
           slug = excluded.slug,
           status = excluded.status,
           status_reason = excluded.status_reason,
           created = excluded.created,
           completed = excluded.completed,
           acceptance = excluded.acceptance,
           objective = excluded.objective,
           context = excluded.context,
           body = excluded.body,
           preamble = excluded.preamble,
           fiat = excluded.fiat,
           updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
           revision = threads.revision + 1"
      }
      Door::Create => "",
    };
    let stored = tx.query_row(
      &format!(
        "INSERT INTO threads (id, title, slug, status, status_reason, created, completed, acceptance, objective, context, body, preamble, fiat) VALUES (?1, ?2, ?3, ?4, ?5, {dates}, ?8, ?9, ?10, ?11, ?12, ?13)
         {on_conflict}
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
        t.body,
        t.preamble,
        t.fiat.as_ref().map(serde_json::to_string).transpose()?,
      ],
      |row| Ok((row.get::<_, String>(0)?, row.get::<_, Option<String>>(1)?)),
    );
    // Translated here rather than left to surface as `sqlite: UNIQUE constraint
    // failed: threads.id`, which names a table and a column to somebody who
    // typed `intent st new`. Surfacing a failure the caller cannot act on is
    // IN-AG-NO-SILENT half-done.
    let stored = stored.map_err(|e| match &e {
      rusqlite::Error::SqliteFailure(f, _)
        if f.code == rusqlite::ErrorCode::ConstraintViolation && door == Door::Create =>
      {
        StoreError::CreateHitAnExistingKey {
          kind: EntityKind::Thread,
          key: t.id.clone(),
        }
      }
      _ => StoreError::from(e),
    })?;
    for (seq, r) in t.related.iter().enumerate() {
      tx.execute(
        "INSERT INTO related (thread_id, seq, id, note) VALUES (?1, ?2, ?3, ?4)",
        params![t.id, seq as i64, r.id, r.note],
      )?;
    }
    for (seq, a) in t.attachments.iter().enumerate() {
      tx.execute(
        "INSERT INTO attachments (thread_id, seq, path, text, blob, bytes, sha256) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![t.id, seq as i64, a.path, a.text, a.blob, a.bytes, a.sha256],
      )?;
    }
    for wp in &t.wps {
      tx.execute(
        "INSERT INTO wps (thread_id, seq, title, scope, scope_legacy, status, status_reason, objective, body, preamble, fiat, status_legacy) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        params![
          t.id,
          wp.seq,
          wp.title,
          wp.scope.as_ref().map(enum_str),
          wp.scope_legacy.as_ref().map(|l| l.raw.clone()),
          enum_str(&wp.status),
          wp.status_reason,
          wp.objective,
          wp.body,
          wp.preamble,
          wp.fiat.as_ref().map(serde_json::to_string).transpose()?,
          wp.status_legacy.as_ref().map(|l| l.raw.clone()),
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
        "INSERT INTO tests (thread_id, id, kind, file, prose, covers, status, note, legacy, fiat) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
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
          at.fiat.as_ref().map(serde_json::to_string).transpose()?,
        ],
      )?;
    }
    Ok(stored)
  }

  /// Write ONE issue inside an open transaction. Same Highlander reason as
  /// [`Store::write_thread`].
  ///
  /// **THE TWO DOORS, and this function had neither until `issues add` existed
  /// to need them** (D42). `issues.created` and `issues.closed` are DOMAIN
  /// dates -- the DDL comment above the table says so, in the same breath as
  /// `threads.created` -- and a domain date is either being SET by this write or
  /// REINSTATED by it. Carrying the caller's value unconditionally was correct
  /// only while every caller was `rebuild`, ie while the only act was restore.
  ///
  /// So the doors are exactly [`Store::write_thread`]'s, for exactly its reason:
  /// [`Stamp::ByTheDatabase`] reads the empty string as "I have no date, you
  /// have the clock", and [`Stamp::CarriedFromTheExtract`] takes what it is
  /// given. Returns what landed, because the caller cannot otherwise know.
  ///
  /// The empty-string sentinel is only ever produced by the CREATE verb: every
  /// other mutation clones canon and edits one field, so `created` arrives
  /// already filled and `COALESCE` carries it through untouched. A `closed` of
  /// `Some("")` is `issues close` asking for today; `None` is open.
  fn write_issue(
    tx: &rusqlite::Transaction<'_>,
    i: &Issue,
    stamp: Stamp,
    door: Door,
  ) -> Result<(String, Option<String>), StoreError> {
    let dates = match stamp {
      Stamp::ByTheDatabase => {
        "COALESCE(NULLIF(?6, ''), strftime('%Y-%m-%d', 'now')),
         CASE WHEN ?7 IS NULL THEN NULL
              WHEN ?7 = '' THEN strftime('%Y-%m-%d', 'now')
              ELSE ?7 END"
      }
      Stamp::CarriedFromTheExtract => "?6, ?7",
    };
    // Upserted for the same reason as a thread: durable identity, so
    // `created_at` must fire once rather than on every write.
    //
    // **A new column APPENDS to this statement rather than slotting in beside
    // the field it belongs next to**, because `{dates}` above interpolates
    // `?6` / `?7` by number: inserting a placeholder ahead of them silently
    // renumbers what those two fragments bind to, and the result is a store
    // whose dates are somebody else's column with no error anywhere.
    // **THE CONFLICT CLAUSE IS THE WHOLE OF THE DOOR** (issue 0131). `Change`
    // keeps the upsert; `Create` omits it, so an existing number raises the
    // UNIQUE constraint and this transaction is refused rather than silently
    // rewriting every field of somebody else s issue.
    //
    // Built by `format!` alongside `{dates}` rather than as two statements: one
    // column list, one params list, one home. Two spellings of this insert is
    // how the columns come to disagree.
    let on_conflict = match door {
      Door::Change => {
        "ON CONFLICT (number) DO UPDATE SET
         slug = excluded.slug,
         title = excluded.title,
         status = excluded.status,
         severity = excluded.severity,
         created = excluded.created,
         closed = excluded.closed,
         reporter = excluded.reporter,
         body = excluded.body,
         updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')"
      }
      Door::Create => "",
    };
    let stored = tx.query_row(
      &format!(
        "INSERT INTO issues (number, slug, title, status, severity, created, closed, reporter, body) VALUES (?1, ?2, ?3, ?4, ?5, {dates}, ?8, ?9)
       {on_conflict}
       RETURNING created, closed"
      ),
      params![i.number, i.slug, i.title, enum_str(&i.status), i.severity, i.created, i.closed, i.reporter, i.body],
      |row| Ok((row.get::<_, String>(0)?, row.get::<_, Option<String>>(1)?)),
    );
    // **The constraint is translated HERE, not left to surface as `sqlite:
    // UNIQUE constraint failed: issues.number`.** That string names a table and
    // a column to somebody who typed `intent issues add`, and IN-AG-NO-SILENT
    // is about surfacing failures in terms the caller can act on -- a raw
    // driver error is surfaced and still unactionable.
    let stored = stored.map_err(|e| match &e {
      rusqlite::Error::SqliteFailure(f, _)
        if f.code == rusqlite::ErrorCode::ConstraintViolation && door == Door::Create =>
      {
        StoreError::CreateHitAnExistingKey {
          kind: EntityKind::Issue,
          key: format!("{:04}", i.number),
        }
      }
      _ => StoreError::from(e),
    })?;
    Ok(stored)
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
  /// **Returns what the database actually stored for the domain dates of BOTH
  /// entity kinds** (D42). A mutation that creates a thread or raises an issue
  /// hands in an empty `created`; SQLite fills it as part of the INSERT and the
  /// value comes back here, so the caller learns the date from the write instead
  /// of reading a clock and predicting it.
  /// Fill every empty [`crate::model::FiatRecord::at`] on this thread with the
  /// stamp the database just wrote, returning `None` when there is nothing to
  /// fill.
  ///
  /// **EMPTY IS THE SELECTOR, which is what makes this safe to run over every
  /// entity on every mutation.** A fiat close arriving with a stamp is either a
  /// row this mutation did not touch or one ingested from canon, and re-stamping
  /// either would rewrite when it happened to the moment of an unrelated write.
  /// Same distinction `Stamp::CarriedFromTheExtract` draws for entity dates.
  ///
  /// **ALL FOUR KINDS, and two of them had never been reached at all.** The
  /// caller-side loop this replaces walked criteria and tests only, so a thread
  /// or work package closed on human authority carried no record of WHEN on any
  /// surface, ever -- and those are the two kinds a cascade starts from.
  fn stamp_fiat(t: &crate::model::Thread, event_ts: &str) -> Option<crate::model::Thread> {
    let empty = |r: &Option<crate::model::FiatRecord>| r.as_ref().is_some_and(|r| r.at.is_empty());
    let wants = empty(&t.fiat)
      || t.wps.iter().any(|w| empty(&w.fiat))
      || t.tests.iter().any(|x| empty(&x.fiat))
      || t
        .criteria
        .iter()
        .any(|c| matches!(&c.state, crate::model::AcState::Fiat(r) if r.at.is_empty()));
    if !wants {
      return None;
    }
    let mut t = t.clone();
    let fill = |r: &mut Option<crate::model::FiatRecord>| {
      if let Some(r) = r.as_mut()
        && r.at.is_empty()
      {
        r.at = event_ts.to_string();
      }
    };
    fill(&mut t.fiat);
    for w in &mut t.wps {
      fill(&mut w.fiat);
    }
    for x in &mut t.tests {
      fill(&mut x.fiat);
    }
    for c in &mut t.criteria {
      if let crate::model::AcState::Fiat(r) = &mut c.state
        && r.at.is_empty()
      {
        r.at = event_ts.to_string();
      }
    }
    Some(t)
  }

  pub fn commit_mutation(&mut self, change: Mutation<'_>) -> Result<StoredDates, StoreError> {
    let tx = self.conn.transaction()?;
    // **THE COMPARE-AND-SWAP, AND ITS BEING INSIDE THIS TRANSACTION IS THE
    // WHOLE DIFFERENCE BETWEEN A CAS AND A CHECK** (issue 0206, vc ruled
    // 2026-09-01). The same comparison in `apply_envelopes` would narrow the
    // window and leave it open, and shipping that as a compare-and-swap would
    // overclaim. Here there is no window: the read and the write are one
    // serialised transaction.
    Self::refuse_if_the_record_moved(&tx, &change)?;
    for id in change.removed_threads {
      tx.execute("DELETE FROM tests WHERE thread_id = ?1", params![id])?;
      tx.execute("DELETE FROM criteria WHERE thread_id = ?1", params![id])?;
      tx.execute("DELETE FROM related WHERE thread_id = ?1", params![id])?;
      tx.execute("DELETE FROM attachments WHERE thread_id = ?1", params![id])?;
      tx.execute("DELETE FROM wps WHERE thread_id = ?1", params![id])?;
      tx.execute("DELETE FROM threads WHERE id = ?1", params![id])?;
    }
    for number in change.removed_issues {
      tx.execute("DELETE FROM issues WHERE number = ?1", params![number])?;
    }
    // The mutation's own event: the DB stamps it inside the same
    // transaction as the rows it describes (D42).
    // **THE FIRST ENVELOPE'S STAMP IS THE MUTATION'S, and the rest share the
    // transaction rather than a separate reading.** `StoredDates.event_ts` is
    // what the caller reports for the act it invoked, which is the first one;
    // a cascade's children are the same act reaching further, so a second clock
    // read for them would be a different time for one decision.
    //
    // **WRITTEN BEFORE THE ROWS RATHER THAN AFTER, AND THE ORDER IS NOW
    // LOAD-BEARING.** `FiatRecord.at` is nested inside a criterion's state
    // rather than sitting in a column, so no `DEFAULT` can fill it and the only
    // way the row can carry the database's stamp is for the stamp to EXIST
    // before the row is serialised. It did not: the event was written last, the
    // caller patched its in-memory copy afterwards, and the store kept the empty
    // string the writer handed in -- so the extract carried the time and durable
    // truth carried a blank, until the next `sync --to-disk` put the blank back
    // over the extract. Issue 0159.
    //
    // Ordering INSIDE a transaction is invisible outside it, so nothing about
    // D42 or "the same transaction as the rows it describes" changes here.
    // `event_log` carries no foreign key to `threads`, so an event may precede
    // the row it names.
    let mut event_ts = String::new();
    for (i, envelope) in change.envelopes.iter().enumerate() {
      let ts = Self::write_event(&tx, envelope, Stamp::ByTheDatabase)?;
      if i == 0 {
        event_ts = ts;
      }
    }
    let mut dates = StoredDates::default();
    for t in change.threads {
      // **THE STAMP GOES IN BEFORE THE BYTES DO.** Only a thread actually
      // carrying an unstamped fiat close is cloned, so the ordinary mutation
      // pays nothing for this.
      let stamped = Self::stamp_fiat(t, &event_ts);
      let t = stamped.as_ref().unwrap_or(t);
      // The CREATE door: this write is the thing happening, so the database
      // stamps it.
      // An id the caller DIFFED as new goes through the create door, where an
      // existing key refuses. Everything else is a change and upserts.
      let door = if change.created_threads.iter().any(|id| id == &t.id) {
        Door::Create
      } else {
        Door::Change
      };
      let (created, completed) = Self::write_thread(&tx, t, Stamp::ByTheDatabase, door)?;
      dates.threads.push(ThreadDates {
        id: t.id.clone(),
        created,
        completed,
      });
    }
    for i in change.issues {
      // The same door, for the same reason. `issues add` hands in an empty
      // `created` and `issues close` an empty `closed`; both come back filled.
      // A number the caller DECLARED new goes through the create door, where an
      // existing key refuses. Everything else -- `issues close`, a migration
      // top-up -- is a change and upserts as it always did.
      let door = if change.created_issues.contains(&i.number) {
        Door::Create
      } else {
        Door::Change
      };
      let (created, closed) = Self::write_issue(&tx, i, Stamp::ByTheDatabase, door)?;
      dates.issues.push(IssueDates {
        number: i.number,
        created,
        closed,
      });
    }
    // The canon half: this is the rebuild from canon, and the repository's own
    // prose is not its to delete.
    Self::write_doc_sections(&tx, change.sections, ProseHalf::Canon)?;
    // **AND THE PROJECT STATE, IN THIS TRANSACTION, WHICH IS THE WHOLE OF
    // AC-14.7.** The clock is the database's, read inside the statement, so
    // this keeps D42 for the same reason the envelope does: a time read in Rust
    // and then written is a value held across a gap the write can be retried
    // inside.
    if change.project_state == ProjectStateEdit::SetTodoWatermark {
      tx.execute(
        "INSERT INTO project (id, todo_watermark)
           VALUES (1, strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
         ON CONFLICT (id) DO UPDATE SET
           todo_watermark = strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
           updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')",
        [],
      )?;
    }
    tx.commit()?;
    Ok(StoredDates { event_ts, ..dates })
  }

  /// **DID THE RECORD MOVE UNDER THE WRITE?** (issue 0206.)
  ///
  /// # It compares CONTENT, and `threads.revision` is not the test
  ///
  /// `revision` shipped at `544a83d3` described as the compare-and-swap token,
  /// and **driving the rest of the path showed it cannot be one.**
  ///
  /// The first version of this argument said a sync BUMPS every revision in the
  /// estate, which would have made a counter-based CAS merely noisy. **The
  /// test's own non-vacuity control said `0 -> 0`, and the real mechanism is
  /// worse.** `rebuild` DELETEs every row before re-inserting it, so the upsert
  /// never hits a conflict and [`Door::Change`]'s `revision + 1` never fires:
  /// **the counter RESETS to its default on every sync.**
  ///
  /// A CAS on it would then fail OPEN on the case it exists for -- a facade
  /// loaded at 0, a peer write taking the record to 1, a sync putting it back
  /// to 0, and the stale facade seeing `0 == 0` and writing over the peer.
  /// `a_write_refuses_a_record_that_moved_under_it.rs` drives that reset. That
  /// commit's framing is corrected here rather than rewritten; the column stays
  /// as what it honestly is, a per-record write counter.
  ///
  /// # Comparing content DOMINATES comparing a counter -- it is not a trade
  ///
  /// It does not fire when nothing was lost: a rebuild that rewrites identical
  /// bytes leaves the comparison equal and the write proceeds. It fires
  /// whenever something would be lost: any real divergence between the stored
  /// record and the snapshot this write derives from means the write is about
  /// to carry a stale field back over a newer one. And it is right about ABA,
  /// where a counter is wrong -- a record edited and then restored is, at the
  /// moment of this write, exactly what the writer assumed, so there is nothing
  /// to refuse.
  ///
  /// # Why the comparison cannot rot
  ///
  /// [`Thread`] and [`Issue`] derive `PartialEq`, so this compares the whole
  /// model rather than a list somebody maintains. A field added tomorrow is
  /// compared tomorrow, with nobody remembering to add it -- which matters
  /// because the drift direction here is the dangerous one: a comparison
  /// blind to a column calls the record unchanged and lets the write through.
  ///
  /// # What it does NOT cover, said rather than implied
  ///
  /// **Deletions.** `removed_threads` and `removed_issues` are not checked, so
  /// deleting a record somebody else has just edited still succeeds. That is a
  /// scope line, not an oversight: 0206 is about a read-modify-write silently
  /// carrying a stale field, and an operator who asked to delete a record is
  /// not surprised that it is gone. Refusing a delete on a concurrent edit is
  /// a different ruling and nobody has made it.
  fn refuse_if_the_record_moved(
    tx: &rusqlite::Transaction<'_>,
    change: &Mutation<'_>,
  ) -> Result<(), StoreError> {
    for expected in change.expected_threads {
      // ABSENT IS NOT MOVED. A record that is gone is either being removed by
      // this same mutation or was removed by somebody else, and inventing a
      // refusal from a missing row would refuse the ordinary path -- which is
      // how a guard gets disabled rather than fixed.
      let Some(stored) = Self::hydrate_threads(tx, Some(&expected.id))?.pop() else {
        continue;
      };
      if &stored != *expected {
        return Err(StoreError::RecordMovedUnderTheWrite {
          kind: EntityKind::Thread,
          key: expected.id.clone(),
        });
      }
    }
    for expected in change.expected_issues {
      let Some(stored) = Self::hydrate_issues(tx, Some(expected.number))?.pop() else {
        continue;
      };
      if &stored != *expected {
        return Err(StoreError::RecordMovedUnderTheWrite {
          kind: EntityKind::Issue,
          key: format!("{:04}", expected.number),
        });
      }
    }
    Ok(())
  }

  /// Rebuild the whole store from canon -- the DISK -> DB sync direction.
  ///
  /// Wholesale by design: this is the operation that makes the DB agree with
  /// the tree, so replacing everything is what it means. It is no longer on
  /// the mutation path (see [`Store::commit_mutation`]).
  pub fn rebuild(&mut self, threads: &[Thread], issues: &[Issue]) -> Result<(), StoreError> {
    let tx = self.conn.transaction()?;
    Self::replace_estate(&tx, threads, issues)?;
    tx.commit()?;
    Ok(())
  }

  /// **Warm an EMPTY store from canon, and do nothing to one that is not**
  /// (0131). Returns whether it warmed.
  ///
  /// The cold path used to decide "empty" in `load_fresh` and rebuild much
  /// later, inside `resync`. Between the two a peer could commit its first
  /// write -- and the rebuild, which deletes every row before re-inserting from
  /// a disk the peer has not reached yet, took that row with it. The number was
  /// free again, the next create landed on it, and both writers were told
  /// `created`. Measured on a fresh project: 3 of 3 runs of ten paired `issues
  /// add` rounds lost one filing, always `0001`; 0 of 3 on a store warmed first.
  ///
  /// So the check and the rebuild are one step, under the write lock
  /// (`IMMEDIATE`): a store that holds anything by the time the lock is held
  /// was warmed or written by someone else, and is left exactly as it is.
  pub fn warm_if_cold(&mut self, threads: &[Thread], issues: &[Issue]) -> Result<bool, StoreError> {
    let tx = self
      .conn
      .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
    let held: i64 = tx.query_row(
      "SELECT (SELECT count(*) FROM threads) + (SELECT count(*) FROM issues)",
      [],
      |row| row.get(0),
    )?;
    if held > 0 {
      return Ok(false);
    }
    Self::replace_estate(&tx, threads, issues)?;
    tx.commit()?;
    Ok(true)
  }

  /// Rebuild the store from canon CHOSEN UNDER THE WRITE LOCK (issue `0216`).
  ///
  /// **`decide` is handed the store's current estate and its file index and
  /// returns the estate to write, and all of it happens inside ONE IMMEDIATE
  /// TRANSACTION.** A disk ingest that read the store, decided, and then called
  /// [`Store::rebuild`] would leave a window in which another writer commits
  /// -- and that commit would be replaced by an estate that never saw it,
  /// which is 0216 exactly. Taking the write lock before the read closes the
  /// window: a writer arriving meanwhile waits, or is refused (0226), and is
  /// never silently lost.
  ///
  /// The decision itself is not the store's to make, so it arrives as a
  /// closure; the store only guarantees that what `decide` saw is still true
  /// when its answer is written.
  pub fn rebuild_deciding(
    &mut self,
    decide: impl FnOnce(Vec<Thread>, Vec<Issue>, Vec<FileEntry>) -> (Vec<Thread>, Vec<Issue>),
  ) -> Result<(Vec<Thread>, Vec<Issue>), StoreError> {
    let tx = self
      .conn
      .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
    let held_threads = Self::hydrate_threads(&tx, None)?;
    let held_issues = Self::hydrate_issues(&tx, None)?;
    let index = Self::read_file_index(&tx)?;
    let (threads, issues) = decide(held_threads, held_issues, index);
    Self::replace_estate(&tx, &threads, &issues)?;
    tx.commit()?;
    Ok((threads, issues))
  }

  /// The body both [`Store::rebuild`] and [`Store::warm_if_cold`] run inside
  /// their own transaction: every modelled row out, the given estate in.
  fn replace_estate(
    tx: &rusqlite::Transaction<'_>,
    threads: &[Thread],
    issues: &[Issue],
  ) -> Result<(), StoreError> {
    tx.execute_batch("DELETE FROM tests; DELETE FROM criteria; DELETE FROM related; DELETE FROM attachments; DELETE FROM wps; DELETE FROM threads; DELETE FROM issues;")?;
    for t in threads {
      // The RESTORE door: these dates were recorded before, and rebuilding a
      // store is not the project happening again.
      // `rebuild` REPLACES the estate wholesale, so every row here is a change
      // by construction -- see the issues arm below for why the create door
      // would refuse every re-sync there is.
      Self::write_thread(tx, t, Stamp::CarriedFromTheExtract, Door::Change)?;
    }
    for i in issues {
      // The RESTORE door, same as the threads above. **v2 users AUTHOR an
      // issue's `date` by hand in frontmatter**, so re-stamping it here would
      // overwrite a fact about the world with a fact about this rebuild -- and
      // `rm intent.db` is not an operation (D36) precisely because a rebuild
      // must not change what the estate says.
      // `rebuild` is the disk -> db reload and REPLACES the estate wholesale, so
      // every row here is a change by construction. Sending these through the
      // create door would refuse every re-sync of an issue that already exists,
      // which is every re-sync there is.
      Self::write_issue(tx, i, Stamp::CarriedFromTheExtract, Door::Change)?;
    }
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
    Ok((
      Self::hydrate_threads(&self.conn, None)?,
      Self::hydrate_issues(&self.conn, None)?,
    ))
  }

  /// Hydrate threads out of the store -- the whole estate, or exactly one.
  ///
  /// **ONE HYDRATION PATH SCOPED BY ARGUMENT, AND THE COLUMN LIST IS THE WHOLE
  /// REASON.** [`Store::commit_mutation`]'s compare-and-swap reads one stored
  /// thread and compares it against the snapshot the write was derived from. A
  /// second SELECT written beside this one would agree with it on the day it
  /// was written and then **FAIL OPEN for every column added afterwards** --
  /// the comparison would not see the new field, would call the record
  /// unchanged, and would wave through the exact write it exists to refuse.
  /// **That is the failing direction, so it does not get a second home.** A
  /// scope argument cannot drift from itself.
  fn hydrate_threads(
    conn: &rusqlite::Connection,
    only: Option<&str>,
  ) -> Result<Vec<Thread>, StoreError> {
    let mut threads = Vec::new();
    // `?1 IS NULL OR id = ?1` rather than two statements, for the reason in
    // the doc above: two statements is two column lists.
    let mut stmt = conn.prepare(
      "SELECT id, title, slug, status, status_reason, created, completed, acceptance, objective, context, body, preamble, fiat FROM threads WHERE (?1 IS NULL OR id = ?1) ORDER BY id",
    )?;
    let rows = stmt.query_map(params![only], |row| {
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
        row.get::<_, String>(10)?,
        row.get::<_, String>(11)?,
        row.get::<_, Option<String>>(12)?,
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
      body,
      preamble,
      fiat,
    ) in shells
    {
      threads.push(Thread {
        schema: THREAD_SCHEMA.to_string(),
        body,
        preamble,
        related: Self::related_of(conn, &id)?,
        attachments: Self::attachments_of(conn, &id)?,
        wps: Self::wps_of(conn, &id)?,
        criteria: Self::criteria_of(conn, &id)?,
        tests: Self::tests_of(conn, &id)?,
        id,
        title,
        slug,
        status: enum_from(&status)?,
        status_reason,
        fiat: fiat.map(|raw| serde_json::from_str(&raw)).transpose()?,
        created,
        completed,
        acceptance: acceptance.as_deref().map(enum_from).transpose()?,
        objective,
        context,
      });
    }
    Ok(threads)
  }

  /// Hydrate issues out of the store -- the whole estate, or exactly one.
  ///
  /// The sibling of [`Store::hydrate_threads`], scoped the same way and for the
  /// same reason.
  fn hydrate_issues(
    conn: &rusqlite::Connection,
    only: Option<u32>,
  ) -> Result<Vec<Issue>, StoreError> {
    let mut stmt = conn.prepare(
      "SELECT number, slug, title, status, severity, created, closed, reporter, body FROM issues WHERE (?1 IS NULL OR number = ?1) ORDER BY number",
    )?;
    let issues = stmt
      .query_map(params![only], |row| {
        Ok((
          row.get::<_, u32>(0)?,
          row.get::<_, String>(1)?,
          row.get::<_, String>(2)?,
          row.get::<_, String>(3)?,
          row.get::<_, Option<String>>(4)?,
          row.get::<_, String>(5)?,
          row.get::<_, Option<String>>(6)?,
          row.get::<_, Option<String>>(7)?,
          row.get::<_, String>(8)?,
        ))
      })?
      .collect::<Result<Vec<_>, _>>()?;

    issues
      .into_iter()
      .map(
        |(number, slug, title, status, severity, created, closed, reporter, body)| {
          Ok(Issue {
            schema: ISSUE_SCHEMA.to_string(),
            number,
            slug,
            title,
            status: enum_from(&status)?,
            severity,
            created,
            closed,
            reporter,
            body,
          })
        },
      )
      .collect::<Result<Vec<_>, StoreError>>()
  }
  /// One thread's attachments, in the order they were written.
  ///
  /// `bytes` and `sha256` are read back rather than recomputed from `text`.
  /// Recomputing would make the round trip agree with itself by construction
  /// and pin nothing -- the point of storing them is that a later read can
  /// disagree with the content and say so.
  fn attachments_of(
    conn: &rusqlite::Connection,
    thread: &str,
  ) -> Result<Vec<crate::model::Attachment>, StoreError> {
    let mut stmt = conn.prepare(
      "SELECT path, text, bytes, sha256, blob FROM attachments WHERE thread_id = ?1 ORDER BY seq",
    )?;
    let rows = stmt.query_map(params![thread], |row| {
      Ok(crate::model::Attachment {
        path: row.get(0)?,
        text: row.get(1)?,
        bytes: row.get(2)?,
        sha256: row.get(3)?,
        // Read back as stored, never re-derived from `text`, for the reason
        // the doc above already gives about `bytes` and `sha256`: a value the
        // reader reconstructs agrees with the reader by construction and can
        // never disagree with what was written.
        blob: row.get(4)?,
      })
    })?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
  }

  fn related_of(conn: &rusqlite::Connection, thread: &str) -> Result<Vec<Related>, StoreError> {
    let mut stmt =
      conn.prepare("SELECT id, note FROM related WHERE thread_id = ?1 ORDER BY seq")?;
    let rows = stmt.query_map(params![thread], |row| {
      Ok(Related {
        id: row.get(0)?,
        note: row.get(1)?,
      })
    })?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
  }

  fn wps_of(conn: &rusqlite::Connection, thread: &str) -> Result<Vec<WorkPackage>, StoreError> {
    let mut stmt = conn
      .prepare("SELECT seq, title, scope, scope_legacy, status, status_reason, objective, body, preamble, fiat, status_legacy FROM wps WHERE thread_id = ?1 ORDER BY seq")?;
    let raw = stmt
      .query_map(params![thread], |row| {
        Ok((
          row.get::<_, u32>(0)?,
          row.get::<_, String>(1)?,
          row.get::<_, Option<String>>(2)?,
          row.get::<_, Option<String>>(3)?,
          row.get::<_, String>(4)?,
          row.get::<_, Option<String>>(5)?,
          row.get::<_, String>(6)?,
          row.get::<_, String>(7)?,
          row.get::<_, String>(8)?,
          row.get::<_, Option<String>>(9)?,
          row.get::<_, Option<String>>(10)?,
        ))
      })?
      .collect::<Result<Vec<_>, _>>()?;
    raw
      .into_iter()
      .map(
        |(
          seq,
          title,
          scope,
          scope_legacy,
          status,
          status_reason,
          objective,
          body,
          preamble,
          fiat,
          status_legacy,
        )| {
          Ok(WorkPackage {
            seq,
            title,
            scope: scope.as_deref().map(enum_from).transpose()?,
            scope_legacy: scope_legacy.map(|raw| crate::model::Legacy { raw }),
            status: enum_from(&status)?,
            status_legacy: status_legacy.map(|raw| crate::model::Legacy { raw }),
            status_reason,
            fiat: fiat.map(|raw| serde_json::from_str(&raw)).transpose()?,
            objective,
            body,
            preamble,
          })
        },
      )
      .collect()
  }

  fn criteria_of(conn: &rusqlite::Connection, thread: &str) -> Result<Vec<Criterion>, StoreError> {
    let mut stmt = conn
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

  fn tests_of(
    conn: &rusqlite::Connection,
    thread: &str,
  ) -> Result<Vec<AcceptanceTest>, StoreError> {
    let mut stmt = conn.prepare(
      "SELECT id, kind, file, prose, covers, status, note, legacy, fiat FROM tests WHERE thread_id = ?1 ORDER BY rowid",
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
          row.get::<_, Option<String>>(8)?,
        ))
      })?
      .collect::<Result<Vec<_>, _>>()?;
    raw
      .into_iter()
      .map(
        |(id, kind, file, prose, covers, status, note, legacy, fiat)| {
          Ok(AcceptanceTest {
            id,
            kind: enum_from(&kind)?,
            file,
            prose,
            covers: serde_json::from_str(&covers)?,
            status: enum_from(&status)?,
            fiat: fiat.map(|raw| serde_json::from_str(&raw)).transpose()?,
            note,
            legacy: legacy.map(|raw| Legacy { raw }),
          })
        },
      )
      .collect()
  }

  /// Every prose section in the index, in a total order.
  pub fn doc_sections(&self) -> Result<Vec<DocSection>, StoreError> {
    self.doc_sections_query(
      "SELECT owner_type, owner_id, file, seq, heading, level, body FROM doc_sections ORDER BY file, seq",
      [],
    )
  }

  /// Every file holding a section of one owner type, in no order.
  ///
  /// **NO BODY AND NO `ORDER BY`, AND THAT IS THE WHOLE OF WHY IT EXISTS.**
  /// [`Store::doc_sections`] reads every body and sorts the table by file,
  /// which SQLite does on a real store as an external merge sort spilling to
  /// temporary files. The table is FTS5, so no index can sit under this; not
  /// asking for the bodies is what keeps it cheap.
  // Issue 0354: `carried_paths` paid that sort on every index refresh.
  pub fn section_files(&self, owner_type: &str) -> Result<Vec<String>, StoreError> {
    let mut stmt = self
      .conn
      .prepare("SELECT DISTINCT file FROM doc_sections WHERE owner_type = ?1")?;
    let rows = stmt.query_map(params![owner_type], |row| row.get::<_, String>(0))?;
    let mut out = Vec::new();
    for row in rows {
      out.push(row?);
    }
    Ok(out)
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
  /// Open a load-from-canon and record that it started (AC-03.13).
  ///
  /// **Committed on its own, before the rebuild transaction opens, and that
  /// ordering is the whole correctness of the table.** The refusal this exists
  /// to record is a SQLite failure INSIDE `Store::rebuild`'s transaction, so
  /// anything written there rolls back with it -- a store that recorded the
  /// attempt in the same transaction would forget it had ever been asked, and
  /// report the stale contents as though nothing had happened. Which is the
  /// original defect, reproduced by the fix for it.
  ///
  /// Re-entrant: an inner load joins the outer one and does not open a second
  /// row. See [`Store::ingest_depth`]'s note for why the outermost caller has
  /// to be the one that owns the outcome.
  pub fn begin_ingest(&mut self) -> Result<(), StoreError> {
    if self.ingest_depth == 0 {
      self
        .conn
        .execute("INSERT INTO ingests (outcome) VALUES ('attempted')", [])?;
      self.ingest_attempt = Some(self.conn.last_insert_rowid());
    }
    self.ingest_depth += 1;
    Ok(())
  }

  /// Close the load opened by [`Store::begin_ingest`], recording how it ended.
  ///
  /// Only the OUTERMOST close writes; an inner one just unwinds the depth. A
  /// close with nothing open is a no-op rather than an error, because the
  /// caller that would see that error is the error path of an ingest, and
  /// failing there would replace the operator's real refusal with a
  /// book-keeping one.
  pub fn finish_ingest(&mut self, outcome: IngestOutcome, detail: &str) -> Result<(), StoreError> {
    self.ingest_depth = self.ingest_depth.saturating_sub(1);
    if self.ingest_depth > 0 {
      return Ok(());
    }
    let Some(id) = self.ingest_attempt.take() else {
      return Ok(());
    };
    self.conn.execute(
      "UPDATE ingests
          SET outcome = ?1,
              detail = ?2,
              updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
        WHERE id = ?3",
      rusqlite::params![outcome.as_str(), detail, id],
    )?;
    Ok(())
  }

  /// The most recent load of canon into this store, or `None` if none has been
  /// recorded.
  ///
  /// **`None` means no evidence, and readers must not read it as failure.** A
  /// store written before this table shipped has no rows, and so does a store
  /// warmed by a path that predates the recording; treating either as a refusal
  /// would block the egest on every upgraded project at once, for something
  /// nobody observed.
  pub fn last_ingest(&self) -> Result<Option<IngestRecord>, StoreError> {
    let mut stmt = self.conn.prepare(
      "SELECT id, outcome, detail, started_at, updated_at
         FROM ingests ORDER BY id DESC LIMIT 1",
    )?;
    let mut rows = stmt.query([])?;
    let Some(row) = rows.next()? else {
      return Ok(None);
    };
    Ok(Some(IngestRecord {
      id: row.get(0)?,
      outcome: row.get(1)?,
      detail: row.get(2)?,
      started_at: row.get(3)?,
      updated_at: row.get(4)?,
    }))
  }

  /// Open a backup attempt and return its id and the stamp the DATABASE gave
  /// it.
  ///
  /// **The row is written BEFORE the copy is attempted**, which is what lets a
  /// failure be distinguishable from a backup that was never due. It also
  /// solves the naming problem without a clock: the snapshot file is named from
  /// the stamp this returns, so the filename is a value the database produced
  /// rather than one the application asked for.
  pub fn begin_snapshot(&self) -> Result<(i64, String), StoreError> {
    Ok(self.conn.query_row(
      "INSERT INTO snapshots DEFAULT VALUES RETURNING id, taken_at",
      [],
      |row| Ok((row.get(0)?, row.get(1)?)),
    )?)
  }

  /// Close a backup attempt, succeeded or failed.
  ///
  /// One method for both outcomes on purpose: two would make "forgot to record
  /// the failure" reachable, and an attempt left open forever is exactly the
  /// silent state this table exists to remove.
  pub fn finish_snapshot(
    &self,
    id: i64,
    outcome: SnapshotOutcome,
    path: Option<&str>,
    bytes: Option<u64>,
    detail: Option<&str>,
  ) -> Result<(), StoreError> {
    self.conn.execute(
      "UPDATE snapshots SET outcome = ?2, path = ?3, bytes = ?4, detail = ?5,
         updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?1",
      params![
        id,
        match outcome {
          SnapshotOutcome::Ok => "ok",
          SnapshotOutcome::Failed => "failed",
        },
        path,
        bytes.map(|b| b as i64),
        detail
      ],
    )?;
    Ok(())
  }

  /// Every recorded backup attempt, newest first.
  pub fn snapshots(&self) -> Result<Vec<SnapshotRecord>, StoreError> {
    let mut stmt = self.conn.prepare(
      "SELECT id, path, bytes, outcome, detail, taken_at FROM snapshots ORDER BY taken_at DESC, id DESC",
    )?;
    let rows = stmt.query_map([], |row| {
      Ok(SnapshotRecord {
        id: row.get(0)?,
        path: row.get(1)?,
        bytes: row.get::<_, Option<i64>>(2)?.map(|b| b as u64),
        outcome: row.get(3)?,
        detail: row.get(4)?,
        taken_at: row.get(5)?,
      })
    })?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
  }

  /// **How many hours have passed since the newest SUCCESSFUL backup, computed
  /// entirely inside SQLite.** `None` when none has ever succeeded.
  ///
  /// The comparison is the database's, not the application's, and that is the
  /// design rather than a flourish. hv permits reading a clock to make a
  /// decision, but the cheapest way to keep that permission from eroding is to
  /// not need it: SQLite compares its own stamp against its own `now` and
  /// returns an INTERVAL. **An interval is not a time** -- it cannot be written
  /// into a record, it cannot be mistaken for one, and there is no moment at
  /// which this process knows what the time is.
  ///
  /// Only `ok` rows count. A failed attempt is not a backup, and letting one
  /// reset the staleness clock would make a schedule that runs and fails every
  /// hour look healthier than one that has never run.
  pub fn hours_since_last_good_snapshot(&self) -> Result<Option<f64>, StoreError> {
    Ok(self.conn.query_row(
      "SELECT (julianday('now') - julianday(max(taken_at))) * 24.0
         FROM snapshots WHERE outcome = 'ok'",
      [],
      |row| row.get::<_, Option<f64>>(0),
    )?)
  }

  /// Backup attempts that FAILED since the newest good snapshot.
  ///
  /// **THE COMPANION TO [`Store::hours_since_last_good_snapshot`], AND IT
  /// EXISTS BECAUSE THAT ONE ANSWERS THE SYMPTOM AND THIS ONE ANSWERS THE
  /// CAUSE.** Staleness says *your newest restorable snapshot is 168h old*,
  /// which is true of a store nothing has tried to back up and equally true of
  /// one that has tried and failed every hour for a week -- **and those call
  /// for opposite actions.** The first says the mechanism is not running; the
  /// second says it is running and something is stopping it, and only the
  /// second has a `detail` naming what.
  ///
  /// **SINCE THE NEWEST GOOD ONE, NOT EVER, BECAUSE FAILURES THAT WERE
  /// FOLLOWED BY A SUCCESS ARE HISTORY.** A store backed up successfully an
  /// hour ago is healthy whatever happened last March, and a lifetime count
  /// would make every estate that ever had a bad day permanently report one.
  ///
  /// **A STORE THAT HAS NEVER SUCCEEDED COUNTS ALL OF THEM.** `coalesce(..,
  /// '')` sorts before every real stamp, so *since the last good snapshot* and
  /// *since the beginning* are one expression rather than a branch -- which is
  /// what keeps the never-succeeded case, the one that most needs reporting,
  /// off a path of its own.
  ///
  /// **`attempted` ROWS ARE NOT COUNTED AND THAT IS A STATED LIMIT.** A row
  /// left open is a process that died between [`Store::begin_snapshot`] and
  /// [`Store::finish_snapshot`] -- **or a backup running right now**, and
  /// telling those apart needs to know how long it has been open, which is a
  /// clock read this estate does not make (D42). Counting them would report a
  /// healthy in-flight backup as a failure.
  pub fn failures_since_last_good_snapshot(&self) -> Result<FailedAttempts, StoreError> {
    Ok(self.conn.query_row(
      "WITH last_good(at) AS (
         SELECT coalesce(max(taken_at), '') FROM snapshots WHERE outcome = 'ok'
       )
       SELECT
         (SELECT count(*) FROM snapshots, last_good
            WHERE outcome = 'failed' AND taken_at > last_good.at),
         (SELECT detail FROM snapshots, last_good
            WHERE outcome = 'failed' AND taken_at > last_good.at
            ORDER BY taken_at DESC, id DESC LIMIT 1)",
      [],
      |row| {
        Ok(FailedAttempts {
          attempts: row.get::<_, i64>(0)? as u32,
          newest_detail: row.get::<_, Option<String>>(1)?,
        })
      },
    )?)
  }

  /// Snapshots outside the retention window, as `(id, path)`.
  ///
  /// **The bucketing is SQL, so the retention decision is made where the
  /// stamps are.** Rust never learns a date, a week number or the current
  /// time; it receives a list of rows to forget.
  ///
  /// The rule is "keep the newest snapshot in each of the most recent N day
  /// buckets, M week buckets and K month buckets". A snapshot can be kept by
  /// any of the three, which is what makes the window roll: today's newest is
  /// held by the day rule, and as it ages out of that it is still the newest
  /// of its week, then of its month.
  ///
  /// **Only successful snapshots with a file are candidates.** A failed
  /// attempt has nothing to delete and is the audit trail this table exists
  /// for, so pruning is not allowed to quietly consume the evidence that
  /// backups have been failing.
  pub fn expired_snapshots(
    &self,
    daily: u32,
    weekly: u32,
    monthly: u32,
  ) -> Result<Vec<(i64, String)>, StoreError> {
    let mut stmt = self.conn.prepare(
      "WITH good AS (
         SELECT id, path, taken_at,
                date(taken_at) AS d,
                strftime('%Y-%W', taken_at) AS w,
                strftime('%Y-%m', taken_at) AS m
           FROM snapshots
          WHERE outcome = 'ok' AND path IS NOT NULL
       ),
       keep_day AS (
         SELECT max(taken_at) AS t FROM good GROUP BY d ORDER BY d DESC LIMIT ?1
       ),
       keep_week AS (
         SELECT max(taken_at) AS t FROM good GROUP BY w ORDER BY w DESC LIMIT ?2
       ),
       keep_month AS (
         SELECT max(taken_at) AS t FROM good GROUP BY m ORDER BY m DESC LIMIT ?3
       ),
       keep AS (
         SELECT t FROM keep_day
         UNION SELECT t FROM keep_week
         UNION SELECT t FROM keep_month
       )
       SELECT id, path FROM good
        WHERE taken_at NOT IN (SELECT t FROM keep)
        ORDER BY taken_at",
    )?;
    let rows = stmt.query_map(params![daily, weekly, monthly], |row| {
      Ok((row.get(0)?, row.get(1)?))
    })?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
  }

  /// Drop a snapshot's row, once its file is gone.
  ///
  /// Deleted rather than marked, because this row's whole subject is a file
  /// that exists; once it does not, a retained row would make `backup --list`
  /// report snapshots nobody can restore from. The failure rows are what stay.
  pub fn forget_snapshot(&self, id: i64) -> Result<(), StoreError> {
    self
      .conn
      .execute("DELETE FROM snapshots WHERE id = ?1", params![id])?;
    Ok(())
  }

  /// How many distinct FILES of CANON the prose index holds.
  ///
  /// **SCOPED TO CANON'S HALF, because the table now holds two.** The
  /// repository's own prose lives here too, under `owner_type = file`, and
  /// counting it as canon would make the freshness block report one corpus's
  /// size as another's.
  ///
  /// Sections are what the index stores and files are what an operator counts,
  /// so the freshness block reports files: "3 sections" says nothing about
  /// whether the tree was read, and one file split six ways is one file.
  pub fn canon_doc_file_count(&self) -> Result<usize, StoreError> {
    let count: i64 = self.conn.query_row(
      "SELECT count(DISTINCT file) FROM doc_sections WHERE owner_type <> ?1",
      params![crate::prose::FILE_OWNER],
      |row| row.get(0),
    )?;
    Ok(count as usize)
  }

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

  /// The DONE cutoff, or `None` when the project has never flushed.
  ///
  /// **THE ONE HOME FOR THIS VALUE.** It used to be derived from the maximum
  /// `todo.flush` stamp in the log, which made it history -- and D53 took
  /// history out of the working tree, so it could not cross a clone. Every
  /// flushed thread came back on a fresh clone and `doctor` reported the
  /// committed `todo.md` as hand-edited, permanently.
  ///
  /// `None` covers both an absent singleton and a NULL column, deliberately:
  /// they are the same fact, and distinguishing them would make a caller reason
  /// about whether the row had been created yet.
  pub fn todo_watermark(&self) -> Result<Option<String>, StoreError> {
    let mut stmt = self
      .conn
      .prepare("SELECT todo_watermark FROM project WHERE id = 1")?;
    let mut rows = stmt.query([])?;
    match rows.next()? {
      Some(row) => Ok(row.get::<_, Option<String>>(0)?),
      None => Ok(None),
    }
  }

  /// Record the DONE cutoff.
  ///
  /// An UPSERT rather than an UPDATE, so the singleton does not have to exist
  /// first. A store created by [`DDL`] has the table and no row; a store that
  /// came up the migration ladder has the row already. **Two creation paths and
  /// one write** -- an UPDATE would silently affect zero rows on the first of
  /// them, which is a flush that reports success and changes nothing.
  pub fn set_todo_watermark(&self, mark: Option<&str>) -> Result<(), StoreError> {
    self.conn.execute(
      "INSERT INTO project (id, todo_watermark) VALUES (1, ?1)
       ON CONFLICT (id) DO UPDATE SET
         todo_watermark = excluded.todo_watermark,
         updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')",
      params![mark],
    )?;
    Ok(())
  }

  /// Every envelope, oldest first.
  ///
  /// Ordered by id rather than by `ts`: a ULID is lexically sortable by its
  /// own timestamp prefix, so it gives a total order even for two events
  /// minted inside the same millisecond -- which `ts` alone does not.
  /// Every distinct `op` the log holds, with how many rows carry it.
  ///
  /// **A `GROUP BY` RATHER THAN [`Self::events`] AND A FOLD, BECAUSE THE CALLER
  /// IS `doctor`.** Doctor has to be the command that still works when the
  /// others have stopped, and reading every envelope to learn 21 strings makes
  /// its cost grow with history for an answer whose size does not. The count
  /// rides along because a report naming an op nobody recognises is asked "how
  /// much of it is there" immediately, and the answer separates one stray row
  /// from a decade of them.
  pub fn op_census(&self) -> Result<Vec<(String, usize)>, StoreError> {
    let mut stmt = self
      .conn
      .prepare("SELECT op, COUNT(*) FROM event_log GROUP BY op ORDER BY op")?;
    let rows = stmt.query_map([], |row| {
      Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)? as usize))
    })?;
    let mut out = Vec::new();
    for row in rows {
      out.push(row?);
    }
    Ok(out)
  }

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
  /// Hydrate every node's board out of the store, one [`Board`] per node.
  ///
  /// **Items and messages are ordered by their ROWID, not by a stamp**, because
  /// every row a migration inserts in one pass shares one `recorded_at` and a
  /// stamp cannot order them. Insertion order is the board's only cross-node
  /// ordering for migrated content, so it is what the read preserves and what
  /// every view then renders.
  ///
  /// A message belongs to its RECIPIENT's board: that is the shape the disk
  /// already has, so one file is a whole readable board.
  pub fn hydrate_boards(&self) -> Result<Vec<Board>, StoreError> {
    let mut stmt = self.conn.prepare(
      "SELECT moniker, name, role, session_id, heartbeat_at, status, focus, claims, recorded_at, \
       authored_at, migrated_at FROM wb_node ORDER BY moniker",
    )?;
    let nodes = stmt
      .query_map([], |row| {
        Ok((
          row.get::<_, String>(0)?,
          row.get::<_, String>(1)?,
          row.get::<_, String>(2)?,
          row.get::<_, Option<String>>(3)?,
          row.get::<_, String>(4)?,
          row.get::<_, String>(5)?,
          row.get::<_, String>(6)?,
          row.get::<_, String>(7)?,
          row.get::<_, String>(8)?,
          row.get::<_, Option<String>>(9)?,
          row.get::<_, Option<String>>(10)?,
        ))
      })?
      .collect::<Result<Vec<_>, _>>()?;

    let mut boards = Vec::new();
    for (
      moniker,
      name,
      role,
      session_id,
      heartbeat_at,
      status,
      focus,
      claims,
      recorded_at,
      authored_at,
      migrated_at,
    ) in nodes
    {
      let node = WbNode {
        moniker: moniker.clone(),
        name,
        role,
        session_id,
        heartbeat_at,
        status: enum_from(&status)?,
        focus,
        claims: serde_json::from_str(&claims)?,
        recorded_at,
        authored_at,
        migrated_at,
      };
      boards.push(Board {
        schema: BOARD_SCHEMA.to_string(),
        items: Self::hydrate_items(&self.conn, &moniker)?,
        messages: Self::hydrate_messages(&self.conn, &moniker)?,
        node,
      });
    }
    Ok(boards)
  }

  fn hydrate_items(conn: &rusqlite::Connection, node: &str) -> Result<Vec<WbItem>, StoreError> {
    let mut stmt = conn.prepare(
      "SELECT node, kind, seq, text, state, archived_at, recorded_at, authored_at FROM wb_item \
       WHERE node = ?1 ORDER BY id",
    )?;
    let rows = stmt
      .query_map(params![node], |row| {
        Ok((
          row.get::<_, String>(0)?,
          row.get::<_, String>(1)?,
          row.get::<_, u32>(2)?,
          row.get::<_, String>(3)?,
          row.get::<_, String>(4)?,
          row.get::<_, Option<String>>(5)?,
          row.get::<_, String>(6)?,
          row.get::<_, Option<String>>(7)?,
        ))
      })?
      .collect::<Result<Vec<_>, _>>()?;
    rows
      .into_iter()
      .map(
        |(node, kind, seq, text, state, archived_at, recorded_at, authored_at)| {
          Ok(WbItem {
            node,
            kind: enum_from(&kind)?,
            seq,
            text,
            state: enum_from(&state)?,
            archived_at,
            recorded_at,
            authored_at,
          })
        },
      )
      .collect()
  }

  fn hydrate_messages(
    conn: &rusqlite::Connection,
    recipient: &str,
  ) -> Result<Vec<WbMessage>, StoreError> {
    let mut stmt = conn.prepare(
      "SELECT sender, recipient, body, re, fyi, state, handled_at, recorded_at, authored_at \
       FROM wb_message WHERE recipient = ?1 ORDER BY id",
    )?;
    let rows = stmt
      .query_map(params![recipient], |row| {
        Ok((
          row.get::<_, String>(0)?,
          row.get::<_, String>(1)?,
          row.get::<_, String>(2)?,
          row.get::<_, Option<String>>(3)?,
          row.get::<_, i64>(4)?,
          row.get::<_, String>(5)?,
          row.get::<_, Option<String>>(6)?,
          row.get::<_, String>(7)?,
          row.get::<_, Option<String>>(8)?,
        ))
      })?
      .collect::<Result<Vec<_>, _>>()?;
    rows
      .into_iter()
      .map(
        |(sender, recipient, body, re, fyi, state, handled_at, recorded_at, authored_at)| {
          Ok(WbMessage {
            sender,
            recipient,
            body,
            re,
            fyi: fyi != 0,
            state: enum_from(&state)?,
            handled_at,
            recorded_at,
            authored_at,
          })
        },
      )
      .collect()
  }

  /// Register the node roster: one `wb_node` row per participant, with no
  /// items and no messages.
  ///
  /// **THE CLOCK IS READ BY THE DATABASE AT THE WRITE, AS A VALUE AND NEVER AS
  /// A COLUMN DEFAULT.** Those are different mechanisms with the same spelling
  /// and only one of them is safe here: a default fires again on every
  /// re-insert, and a disk-to-db resync re-inserts every row, so the whole
  /// board would be re-stamped on each sync -- history rewritten silently and
  /// indistinguishably from a correct value. Written as a value it fires once,
  /// here, and [`Store::replace_boards`] carries the result forward verbatim.
  /// No caller supplies a time, which is the other half of the same rule.
  ///
  /// `heartbeat_at` takes the same instant: a registered node has not reported
  /// itself alive yet, and inventing an earlier time for it would be the
  /// fabrication this model exists to make impossible.
  ///
  /// Idempotent by moniker, so registering twice is not two rows -- but an
  /// existing row is LEFT ALONE rather than refreshed, because the roster is
  /// the starting state and everything after it belongs to whoever wrote it.
  ///
  /// **`migrated` SAYS WHETHER THE ROW IS THE BOARD.** A node named from its
  /// arguments has no hand-authored board, so its row is stamped migrated at
  /// the insert; a node read off its header is not, until `wb migrate` carries
  /// the markdown the header sits on.
  pub fn register_nodes(
    &mut self,
    nodes: &[(String, String, String)],
    migrated: bool,
  ) -> Result<usize, StoreError> {
    let tx = self.conn.transaction()?;
    let mut written = 0;
    for (moniker, name, role) in nodes {
      written += tx.execute(
        "INSERT INTO wb_node (moniker, name, role, session_id, heartbeat_at, status, focus, \
         claims, recorded_at, authored_at, migrated_at) \
         SELECT ?1, ?2, ?3, NULL, strftime('%Y-%m-%dT%H:%M:%fZ', 'now'), 'paused', '', '[]', \
         strftime('%Y-%m-%dT%H:%M:%fZ', 'now'), NULL, \
         CASE WHEN ?4 THEN strftime('%Y-%m-%dT%H:%M:%fZ', 'now') END \
         WHERE NOT EXISTS (SELECT 1 FROM wb_node WHERE moniker = ?1)",
        params![moniker, name, role, migrated],
      )?;
    }
    tx.commit()?;
    Ok(written)
  }

  /// Replace the whiteboard tables from a set of boards.
  ///
  /// **DELETE-THEN-INSERT, DELIBERATELY, AND IT IS WHY `recorded_at` MUST NOT
  /// BE A COLUMN DEFAULT.** A restore re-inserts every row, so a
  /// `DEFAULT CURRENT_TIMESTAMP` would re-stamp the whole board on each
  /// disk-to-db sync -- rewriting history silently and indistinguishably from a
  /// correct value. The value is carried from the board being written, which is
  /// what makes the round trip lossless in both directions.
  ///
  /// Rows are inserted in the order the board holds them, so the ROWIDs that
  /// order the next read are the order this write was given.
  /// Is this node's board the model's, rather than its markdown on disk?
  pub fn wb_node_migrated(&self, moniker: &str) -> Result<bool, StoreError> {
    let migrated: i64 = self.conn.query_row(
      "SELECT count(*) FROM wb_node WHERE moniker = ?1 AND migrated_at IS NOT NULL",
      params![moniker],
      |row| row.get(0),
    )?;
    Ok(migrated > 0)
  }

  /// Is this moniker on the roster?
  pub fn wb_node_exists(&self, moniker: &str) -> Result<bool, StoreError> {
    let n: i64 = self.conn.query_row(
      "SELECT count(*) FROM wb_node WHERE moniker = ?1",
      params![moniker],
      |r| r.get(0),
    )?;
    Ok(n > 0)
  }

  /// Every registered moniker, in roster order.
  pub fn wb_monikers(&self) -> Result<Vec<String>, StoreError> {
    let mut stmt = self
      .conn
      .prepare("SELECT moniker FROM wb_node ORDER BY rowid")?;
    let rows = stmt.query_map([], |r| r.get::<_, String>(0))?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
  }

  /// How many LIVE messages one inbox holds -- an inbox being one ordered
  /// (sender, recipient) pair, which is the shape the file form already has.
  pub fn wb_live_message_count(&self, sender: &str, recipient: &str) -> Result<usize, StoreError> {
    let n: i64 = self.conn.query_row(
      "SELECT count(*) FROM wb_message WHERE sender = ?1 AND recipient = ?2 AND state = 'live'",
      params![sender, recipient],
      |r| r.get(0),
    )?;
    Ok(n as usize)
  }

  /// Stamp a node's heartbeat from the clock at the write.
  ///
  /// **NO CALLER SUPPLIES THE TIME, HERE LEAST OF ALL.** A heartbeat is the one
  /// field whose whole meaning is "this node was alive at this moment", so a
  /// caller-supplied value would be the fabricated stamp with the model's
  /// blessing. The database reads the clock as a VALUE, never as a column
  /// default, for the reason the tables record.
  pub fn wb_touch(&mut self, node: &str) -> Result<(), StoreError> {
    self.conn.execute(
      "UPDATE wb_node SET heartbeat_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE moniker = ?1",
      params![node],
    )?;
    Ok(())
  }

  /// A node picking up: its status, its heartbeat, and the session and focus
  /// it names, in one statement.
  ///
  /// **AN UNNAMED SESSION OR FOCUS KEEPS WHAT THE ROW HOLDS.** A pickup that
  /// says nothing about its focus has not said the focus is empty, and writing
  /// it empty would erase the one line a peer reads to know what this node is
  /// on. The clock is read as a value, as `wb_touch` reads it.
  pub fn wb_pick_up(
    &mut self,
    node: &str,
    status: &str,
    session_id: Option<&str>,
    focus: Option<&str>,
  ) -> Result<(), StoreError> {
    self.conn.execute(
      "UPDATE wb_node SET status = ?2, session_id = coalesce(?3, session_id), focus = coalesce(?4, focus), heartbeat_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE moniker = ?1",
      params![node, status, session_id, focus],
    )?;
    Ok(())
  }

  /// Set a node's status.
  pub fn wb_set_status(&mut self, node: &str, status: &str) -> Result<(), StoreError> {
    self.conn.execute(
      "UPDATE wb_node SET status = ?2 WHERE moniker = ?1",
      params![node, status],
    )?;
    Ok(())
  }

  /// Carry one node's HEADER BLOCK off its hand-authored board, for `wb
  /// migrate` and for nothing else.
  ///
  /// **THE SERVICE STAMPS TAKE THE INGEST INSTANT AND THE BOARD'S CLAIM GOES TO
  /// `authored_at`**, which is the ruling that resolves AC-14.4 against AC-14.9:
  /// a migration run through the ordinary API turns every historical stamp into
  /// `now` and loses the claim, and one run around the API puts a hole in the
  /// refusal on its first day. So `heartbeat_at` is re-read from the clock here
  /// -- what a board CLAIMED about its own liveness is the value we know may be
  /// invented -- while that claim survives verbatim beside it.
  ///
  /// `recorded_at` is left exactly as registration wrote it: that is when this
  /// node entered the model, and a migration is not a second birth.
  #[allow(clippy::too_many_arguments)]
  pub fn wb_carry_header(
    &mut self,
    node: &str,
    name: &str,
    role: &str,
    session_id: Option<&str>,
    status: &str,
    focus: &str,
    claims: &[String],
    authored_at: Option<&str>,
  ) -> Result<(), StoreError> {
    self.conn.execute(
      "UPDATE wb_node SET name = ?2, role = ?3, session_id = ?4, status = ?5, focus = ?6, \
       claims = ?7, heartbeat_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now'), authored_at = ?8, \
       migrated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE moniker = ?1",
      params![
        node,
        name,
        role,
        session_id,
        status,
        focus,
        serde_json::to_string(claims)?,
        authored_at
      ],
    )?;
    Ok(())
  }

  /// Move one live item to archived, and say whether it moved.
  ///
  /// **ARCHIVED IS A STATE AND NEVER A DELETION.** The row keeps its `seq` and
  /// its text, so what an item said stays readable after it stops counting
  /// against the bound -- which is what lets the bound be enforced by refusal
  /// without costing anybody their record.
  pub fn wb_archive_item(&mut self, node: &str, kind: &str, seq: u32) -> Result<bool, StoreError> {
    let moved = self.conn.execute(
      "UPDATE wb_item SET state = 'archived', archived_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') \
       WHERE node = ?1 AND kind = ?2 AND seq = ?3 AND state = 'live'",
      params![node, kind, seq],
    )?;
    Ok(moved > 0)
  }

  /// How many LIVE items one node holds of one kind.
  pub fn wb_live_item_count(&self, node: &str, kind: &str) -> Result<usize, StoreError> {
    let n: i64 = self.conn.query_row(
      "SELECT count(*) FROM wb_item WHERE node = ?1 AND kind = ?2 AND state = 'live'",
      params![node, kind],
      |r| r.get(0),
    )?;
    Ok(n as usize)
  }

  /// Append one item to a node's board, and say which `seq` it was given.
  ///
  /// **`seq` IS ASSIGNED BY THE SERVICE AND NEVER BY A CALLER.** It is the
  /// board's within-kind ordering, and a caller-chosen one collides the moment
  /// two writes race -- the same reasoning that keeps the clock out of callers'
  /// hands one function down. It counts LIVE and ARCHIVED alike, so archiving an
  /// item never frees its number for reuse and a `seq` refers to one item for
  /// the life of the board.
  ///
  /// **`authored_at` IS A CLAIM THE CALLER CARRIES, NEVER A TIME THE CALLER
  /// CHOOSES, and it is `None` for every write but a migration's.** The service
  /// still reads the clock for `recorded_at` here, in this statement, as a value
  /// and never as a column default -- that half has no parameter and will not
  /// get one. What this takes is the stamp a hand-authored board's markdown
  /// PRINTED, verbatim, for the one door that has such a stamp to carry.
  ///
  /// **ONE INSERT PER TABLE, WHICH IS WHY THE PARAMETER IS HERE RATHER THAN IN A
  /// SECOND WRITER BESIDE IT.** A migration-only insert would be the second
  /// spelling of this row, free to drift on `seq`, on `state`, or on the
  /// clock-as-a-value rule, and the drift would be invisible because each door
  /// would be self-consistent.
  pub fn wb_insert_item(
    &mut self,
    node: &str,
    kind: &str,
    text: &str,
    authored_at: Option<&str>,
  ) -> Result<u32, StoreError> {
    let tx = self.conn.transaction()?;
    let next: i64 = tx.query_row(
      "SELECT coalesce(max(seq), 0) + 1 FROM wb_item WHERE node = ?1 AND kind = ?2",
      params![node, kind],
      |r| r.get(0),
    )?;
    tx.execute(
      "INSERT INTO wb_item (node, kind, seq, text, state, archived_at, recorded_at, authored_at) \
       VALUES (?1, ?2, ?3, ?4, 'live', NULL, strftime('%Y-%m-%dT%H:%M:%fZ', 'now'), ?5)",
      params![node, kind, next, text, authored_at],
    )?;
    tx.commit()?;
    Ok(next as u32)
  }

  /// Read one node's claims.
  pub fn wb_claims(&self, node: &str) -> Result<Vec<String>, StoreError> {
    let raw: String = self.conn.query_row(
      "SELECT claims FROM wb_node WHERE moniker = ?1",
      params![node],
      |r| r.get(0),
    )?;
    Ok(serde_json::from_str(&raw)?)
  }

  /// Replace one node's claims.
  ///
  /// **THE CLAIMS LIST IS REPLACED WHOLE RATHER THAN APPENDED TO**, because the
  /// caller has already read it to decide what it should become: an
  /// append-and-a-remove pair would be two doors holding one invariant, and the
  /// ordering between them would be the thing nobody tested.
  pub fn wb_set_claims(&mut self, node: &str, claims: &[String]) -> Result<(), StoreError> {
    self.conn.execute(
      "UPDATE wb_node SET claims = ?2 WHERE moniker = ?1",
      params![node, serde_json::to_string(claims)?],
    )?;
    Ok(())
  }

  /// Append one message to the recipient's board.
  ///
  /// **NO CALLER SUPPLIES THE SERVICE'S STAMP, AND THERE IS NO PARAMETER FOR
  /// ONE.** The clock is read by the database at the write, as a VALUE and never
  /// as a column default -- a default fires again on every re-insert and a
  /// disk-to-db resync re-inserts every row, so the board would be re-stamped on
  /// each sync, which is history rewritten silently.
  ///
  /// **`authored_at` IS THE OTHER THING AND IS NOT AN EXCEPTION TO IT**: what
  /// the entry's `## (...)` heading CLAIMED, verbatim, `None` for everything
  /// born through this door live and non-null only on a migration's write. It is
  /// the field that deliberately carries the class of value the clock guard
  /// exists to refuse -- stamps measured fabricated, an hour out, and ordered
  /// before the message they answer -- so it is stored as text and never read as
  /// a time. See [`Store::wb_insert_item`] for why the parameter sits on this
  /// writer rather than on a second one beside it.
  pub fn wb_insert_message(
    &mut self,
    sender: &str,
    recipient: &str,
    body: &str,
    re: Option<&str>,
    fyi: bool,
    authored_at: Option<&str>,
  ) -> Result<(), StoreError> {
    self.conn.execute(
      "INSERT INTO wb_message (sender, recipient, body, re, fyi, state, handled_at, recorded_at, \
       authored_at) \
       VALUES (?1, ?2, ?3, ?4, ?5, 'live', NULL, strftime('%Y-%m-%dT%H:%M:%fZ', 'now'), ?6)",
      params![sender, recipient, body, re, fyi, authored_at],
    )?;
    Ok(())
  }

  /// Mark every live message from `sender` to `recipient` handled, and say how
  /// many moved.
  ///
  /// **THE COUNT IS WHAT MOVED, NOT WHAT WAS THERE.** Clearing an inbox that
  /// was already clear moves nothing, and reporting its size either way would
  /// say a write happened when none did -- the same rule `wb register` answers
  /// to.
  pub fn wb_clear_inbox(&mut self, sender: &str, recipient: &str) -> Result<usize, StoreError> {
    let moved = self.conn.execute(
      "UPDATE wb_message SET handled_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now'), state = \
       'handled' WHERE sender = ?1 AND recipient = ?2 AND state = 'live'",
      params![sender, recipient],
    )?;
    Ok(moved)
  }

  pub fn replace_boards(&mut self, boards: &[Board]) -> Result<(), StoreError> {
    let tx = self.conn.transaction()?;
    tx.execute("DELETE FROM wb_message", [])?;
    tx.execute("DELETE FROM wb_item", [])?;
    tx.execute("DELETE FROM wb_node", [])?;
    for board in boards {
      let n = &board.node;
      tx.execute(
        "INSERT INTO wb_node (moniker, name, role, session_id, heartbeat_at, status, focus, \
         claims, recorded_at, authored_at, migrated_at) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![
          n.moniker,
          n.name,
          n.role,
          n.session_id,
          n.heartbeat_at,
          enum_str(&n.status),
          n.focus,
          serde_json::to_string(&n.claims)?,
          n.recorded_at,
          n.authored_at,
          n.migrated_at,
        ],
      )?;
      for item in &board.items {
        tx.execute(
          "INSERT INTO wb_item (node, kind, seq, text, state, archived_at, recorded_at, \
           authored_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
          params![
            item.node,
            enum_str(&item.kind),
            item.seq,
            item.text,
            enum_str(&item.state),
            item.archived_at,
            item.recorded_at,
            item.authored_at,
          ],
        )?;
      }
      for message in &board.messages {
        tx.execute(
          "INSERT INTO wb_message (sender, recipient, body, re, fyi, state, handled_at, \
           recorded_at, authored_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
          params![
            message.sender,
            message.recipient,
            message.body,
            message.re,
            i64::from(message.fyi),
            enum_str(&message.state),
            message.handled_at,
            message.recorded_at,
            message.authored_at,
          ],
        )?;
      }
    }
    tx.commit()?;
    Ok(())
  }

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
    upsert_file_entries(&tx, entries)?;
    tx.commit()?;
    Ok(())
  }

  /// Replace the whole index scope table in one transaction.
  ///
  /// **DELETE-MISSING THEN UPSERT-PRESENT**, for the reason
  /// [`Store::replace_file_index`] gives about its own table: a path has
  /// durable identity across reconciles, so wiping would re-fire `created_at`
  /// every time and the column would silently mean `updated_at`.
  ///
  /// **AND THE DELETE IS UNCONDITIONAL HERE BECAUSE THIS TABLE HAS ONE
  /// WRITER.** That is the whole of rung 20: the index and the change detector
  /// each own a table, so neither has to know the other's population to know
  /// which rows are stale. A row this survey did not produce is a path that has
  /// left the index's scope, full stop.
  pub fn replace_index_files(&mut self, rows: &[crate::index::Row]) -> Result<(), StoreError> {
    let tx = self.conn.transaction()?;
    let keep = serde_json::to_string(&rows.iter().map(|r| &r.path).collect::<Vec<_>>())?;
    tx.execute(
      "DELETE FROM index_file WHERE path NOT IN (SELECT value FROM json_each(?1))",
      params![keep],
    )?;
    for r in rows {
      upsert_index_row(&tx, r)?;
    }
    tx.commit()?;
    Ok(())
  }

  /// Apply one subtree's worth of index changes: upsert what moved, delete
  /// what has gone, and leave every other row alone.
  ///
  /// **NOT [`Store::replace_index_files`], AND THE DIFFERENCE IS THE WHOLE
  /// POINT OF AN INCREMENTAL PASS.** That one is told the entire scope and
  /// deletes what it was not told about; this one is told about a subtree, so
  /// deleting the rest would unindex the project every time one file changed.
  /// The removals are named by the caller rather than inferred here, because
  /// only the caller knows which subtree it asked about.
  pub fn apply_index_changes(
    &mut self,
    upserts: &[crate::index::Row],
    removed: &[String],
  ) -> Result<(), StoreError> {
    let tx = self.conn.transaction()?;
    for path in removed {
      tx.execute("DELETE FROM index_file WHERE path = ?1", params![path])?;
    }
    for r in upserts {
      upsert_index_row(&tx, r)?;
    }
    tx.commit()?;
    Ok(())
  }

  /// Replace the indexed content of NAMED PATHS, in one transaction.
  ///
  /// **THE PATHS ARE PASSED SEPARATELY FROM THE SECTIONS, and that is what
  /// makes a removal expressible.** A file that has gone contributes no
  /// sections, so a call that derived its own scope from the sections it was
  /// given could never delete anything; the caller names every path it is
  /// reconciling, and whatever it did not supply rows for is emptied.
  ///
  /// The FTS5 `rebuild` is [`Store::write_doc_sections`]'s, for the reason
  /// recorded there -- a scoped delete leaves a tombstone per row exactly as a
  /// wholesale one does.
  pub fn replace_sections_for(
    &mut self,
    paths: &[String],
    prose: &[DocSection],
    source: &[crate::index::source::Section],
  ) -> Result<(), StoreError> {
    let tx = self.conn.transaction()?;
    for path in paths {
      tx.execute(
        "DELETE FROM doc_sections WHERE owner_type = ?1 AND file = ?2",
        params![crate::prose::FILE_OWNER, path],
      )?;
      tx.execute("DELETE FROM src_sections WHERE path = ?1", params![path])?;
    }
    tx.execute(
      "INSERT INTO doc_sections(doc_sections) VALUES('rebuild')",
      [],
    )?;
    tx.execute(
      "INSERT INTO src_sections(src_sections) VALUES('rebuild')",
      [],
    )?;
    for s in prose {
      insert_doc_section(&tx, s)?;
    }
    for s in source {
      insert_src_section(&tx, s)?;
    }
    tx.commit()?;
    Ok(())
  }

  /// Every row the index holds, in path order.
  pub fn index_files(&self) -> Result<Vec<crate::index::Row>, StoreError> {
    let mut stmt = self.conn.prepare(
      "SELECT path, corpus, lang, size, mtime, indexed_sha256, skipped_reason
         FROM index_file ORDER BY path",
    )?;
    let rows = stmt.query_map([], |row| {
      Ok(crate::index::Row {
        path: row.get(0)?,
        corpus: row.get(1)?,
        lang: row.get(2)?,
        size: row.get::<_, i64>(3)? as u64,
        mtime: row.get(4)?,
        indexed_sha256: row.get(5)?,
        skipped_reason: row.get(6)?,
      })
    })?;
    let mut out = Vec::new();
    for row in rows {
      out.push(row?);
    }
    Ok(out)
  }

  /// Record the files a projection just LANDED, leaving every other row alone
  /// (0260).
  ///
  /// The index answered "what did the store last read"; with this it answers
  /// "what did the store last read OR WRITE", which is the provenance a canon
  /// file needs before an egest may overwrite it. Without it a file the store
  /// wrote after its last ingest looks moved, and store-ahead -- the state
  /// `sync --to-disk` exists to repair -- would be refused.
  ///
  /// Not [`Store::replace_file_index`]: a projection writes a handful of files,
  /// so deleting the rest would unindex files nobody touched.
  pub fn record_file_entries(&mut self, entries: &[FileEntry]) -> Result<(), StoreError> {
    let tx = self.conn.transaction()?;
    upsert_file_entries(&tx, entries)?;
    tx.commit()?;
    Ok(())
  }

  /// Every indexed file, ordered by path.
  pub fn file_index(&self) -> Result<Vec<FileEntry>, StoreError> {
    Self::read_file_index(&self.conn)
  }

  /// The one reader of the file index, on whatever connection or transaction
  /// the caller holds -- [`Store::rebuild_deciding`] reads it under its lock.
  fn read_file_index(conn: &rusqlite::Connection) -> Result<Vec<FileEntry>, StoreError> {
    let mut stmt = conn
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

  /// Replace the CANON half of the prose index in one transaction.
  ///
  /// **THE PROSE TABLE HAS TWO WRITERS AND THEY DELETE ONLY THEIR OWN ROWS.**
  /// The canon ingest owns every section that belongs to an entity; the search
  /// index owns the sections that belong to a FILE on disk
  /// ([`Store::replace_file_sections`]). Both write here on purpose -- one
  /// table means one query answers over canon prose and repository prose alike,
  /// which is the point of widening the corpus at all.
  ///
  /// **THAT IS NOT THE ARRANGEMENT `file_index` AND `index_file` REFUSED, and
  /// the difference is what makes it safe.** There, neither corpus was a subset
  /// of the other and no row said which writer had produced it, so a
  /// delete-missing could not be scoped and the writer that ran last deleted
  /// the other's rows. Here the row carries its own answer in `owner_type`, so
  /// each writer names its own half in SQL and knows nothing about the other's.
  pub fn replace_doc_sections(&mut self, sections: &[DocSection]) -> Result<(), StoreError> {
    let tx = self.conn.transaction()?;
    Self::write_doc_sections(&tx, sections, ProseHalf::Canon)?;
    tx.commit()?;
    Ok(())
  }

  /// Replace ONE NODE's whiteboard prose: the `.history/` snapshots its folds
  /// left behind, carried as documents by `wb migrate`.
  ///
  /// **SCOPED TO THE NODE, so migrating one board cannot empty another's
  /// history**, and re-running the verb over the same node replaces what it
  /// wrote rather than doubling it -- which is the one part of a migration that
  /// is safe to repeat, because the source file is still the authority for it.
  pub fn replace_wb_sections_for(
    &mut self,
    node: &str,
    sections: &[DocSection],
  ) -> Result<(), StoreError> {
    for s in sections {
      debug_assert_eq!(
        s.owner_type,
        crate::prose::WB_OWNER,
        "a whiteboard section's owner_type is what tells the writers apart"
      );
      debug_assert_eq!(
        s.owner_id, node,
        "a section written under this node's scope belongs to this node"
      );
    }
    let tx = self.conn.transaction()?;
    Self::write_doc_sections(&tx, sections, ProseHalf::Whiteboard(node))?;
    tx.commit()?;
    Ok(())
  }

  /// Replace the FILE half of the prose index: the repository's own prose,
  /// which the search index owns. See [`Store::replace_doc_sections`].
  pub fn replace_file_sections(&mut self, sections: &[DocSection]) -> Result<(), StoreError> {
    for s in sections {
      debug_assert_eq!(
        s.owner_type,
        crate::prose::FILE_OWNER,
        "a file section's owner_type is what tells the two writers apart"
      );
    }
    let tx = self.conn.transaction()?;
    Self::write_doc_sections(&tx, sections, ProseHalf::Files)?;
    tx.commit()?;
    Ok(())
  }

  /// THE ONLY PLACE SECTIONS BECOME ROWS, for the same Highlander reason as
  /// [`Store::write_thread`].
  fn write_doc_sections(
    conn: &rusqlite::Connection,
    sections: &[DocSection],
    half: ProseHalf,
  ) -> Result<(), StoreError> {
    match half {
      ProseHalf::Canon => conn.execute(
        "DELETE FROM doc_sections WHERE owner_type NOT IN (?1, ?2)",
        params![crate::prose::FILE_OWNER, crate::prose::WB_OWNER],
      ),
      ProseHalf::Files => conn.execute(
        "DELETE FROM doc_sections WHERE owner_type = ?1",
        params![crate::prose::FILE_OWNER],
      ),
      ProseHalf::Whiteboard(node) => conn.execute(
        "DELETE FROM doc_sections WHERE owner_type = ?1 AND owner_id = ?2",
        params![crate::prose::WB_OWNER, node],
      ),
    }?;
    // **THE `DELETE` EMPTIES THE ROWS AND LEAVES THE INDEX BEHIND** (issue
    // 0234, second mechanism). Deleting from an FTS5 table does not remove a
    // row's terms from the inverted index; it writes a DELETE MARKER for each
    // one into `doc_sections_data`. Wholesale replacement is this table's only
    // write pattern, so those markers accumulate on every mutation for the
    // life of the store, and nothing reports it: the row count is correct, the
    // content table is the right size, `VACUUM` reclaims nothing because
    // tombstones are live data rather than free pages, and searches keep
    // answering correctly the whole time.
    //
    // Measured on the worst project in the estate: 859 sections holding 5 MB
    // of content, against 589 MB of `doc_sections_data`. After the rebuild
    // below the same store is 14.8 MB, down from 2.3 GB, with search intact.
    //
    // **`'rebuild'` RATHER THAN `'delete-all'`, AND THIS WAS DRIVEN, NOT
    // ASSUMED.** `delete-all` is the idiom that first suggests itself and
    // SQLite refuses it here -- *"'delete-all' may only be used with a
    // contentless or external content fts5 table"* -- so it would have failed
    // at runtime on a path with no test. `rebuild` re-derives the index from
    // the content table, which the line above just emptied, so it costs
    // nothing here and truncates what it replaces.
    //
    // **IT STILL CLEARS THEM NOW THAT THE DELETE IS SCOPED TO ONE HALF**, which
    // is why it is still here rather than left over: `rebuild` re-derives the
    // index from the content table as it stands, so it takes in the OTHER
    // writer's rows as well as this one's. A scoped delete leaves tombstones
    // exactly as a wholesale one does, and this is what clears them either way.
    conn.execute(
      "INSERT INTO doc_sections(doc_sections) VALUES('rebuild')",
      [],
    )?;
    for s in sections {
      insert_doc_section(conn, s)?;
    }
    Ok(())
  }

  /// Replace the source half of the search index in one transaction.
  ///
  /// **ONE WRITER, SO THE DELETE IS UNCONDITIONAL**, unlike the prose table it
  /// sits beside: nothing else writes code rows, so a row this pass did not
  /// produce is a file that has left the corpus.
  ///
  /// The `rebuild` is [`Store::write_doc_sections`]'s, for the reason recorded
  /// there: a `DELETE` from an FTS5 table leaves a tombstone per row in the
  /// inverted index, and wholesale replacement is this table's only write
  /// pattern, so without this they accumulate for the life of the store with
  /// nothing reporting it.
  pub fn replace_src_sections(
    &mut self,
    sections: &[crate::index::source::Section],
  ) -> Result<(), StoreError> {
    let tx = self.conn.transaction()?;
    tx.execute("DELETE FROM src_sections", [])?;
    tx.execute(
      "INSERT INTO src_sections(src_sections) VALUES('rebuild')",
      [],
    )?;
    for s in sections {
      insert_src_section(&tx, s)?;
    }
    tx.commit()?;
    Ok(())
  }

  /// Replace the symbols of NAMED PATHS, in one transaction.
  ///
  /// The paths are passed separately from the symbols for
  /// [`Store::replace_sections_for`]'s reason: a file the parser found nothing
  /// in, or one that has gone, contributes no rows, so a call deriving its
  /// scope from its input could never empty anything.
  pub fn replace_symbols_for(
    &mut self,
    paths: &[String],
    symbols: &[crate::index::symbols::Symbol],
  ) -> Result<(), StoreError> {
    let tx = self.conn.transaction()?;
    for path in paths {
      tx.execute("DELETE FROM symbols WHERE path = ?1", params![path])?;
    }
    for s in symbols {
      tx.execute(
        "INSERT INTO symbols (path, lang, name, kind, start_line, end_line)
           VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
          s.path,
          s.lang,
          s.name,
          s.kind.as_str(),
          s.span.start_line as i64,
          s.span.end_line as i64,
        ],
      )?;
    }
    tx.commit()?;
    Ok(())
  }

  /// The language the index recorded for a path, if it recorded one.
  ///
  /// **ONE HOME FOR A PATH'S LANGUAGE.** `src_sections` deliberately does not
  /// carry it: a fact in two tables disagrees the first time a file is
  /// reclassified, and this is a lookup on a primary key.
  pub fn index_lang(&self, path: &str) -> Result<Option<String>, StoreError> {
    match self.conn.query_row(
      "SELECT lang FROM index_file WHERE path = ?1",
      params![path],
      |row| row.get::<_, Option<String>>(0),
    ) {
      Ok(lang) => Ok(lang),
      // A path the index does not hold has no language, which is an answer and
      // not a failure: a search may name a file the index has never read.
      Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
      Err(e) => Err(e.into()),
    }
  }

  /// Record vectors, replacing any this chunk and model already had.
  ///
  /// **THE WIDTH IS CHECKED AGAINST THE ROW'S OWN `dims`**, because a vector
  /// stored at the wrong width is not a smaller vector: it scores, at a
  /// position nobody can account for.
  pub fn record_embeddings(&mut self, rows: &[crate::embed::Stored]) -> Result<(), StoreError> {
    let tx = self.conn.transaction()?;
    for row in rows {
      let mut bytes = Vec::with_capacity(row.vector.len() * 4);
      for value in &row.vector {
        bytes.extend_from_slice(&value.to_le_bytes());
      }
      tx.execute(
        "INSERT INTO embeddings (chunk_id, model, dims, vector) VALUES (?1, ?2, ?3, ?4)
         ON CONFLICT (chunk_id, model) DO UPDATE SET
           dims = excluded.dims,
           vector = excluded.vector,
           updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')",
        params![row.chunk_id, row.model, row.vector.len() as i64, bytes],
      )?;
    }
    tx.commit()?;
    Ok(())
  }

  /// Every vector one model produced, in chunk order.
  ///
  /// **ONE MODEL AT A TIME, BY THE SIGNATURE.** A reader that could ask for
  /// "all the vectors" would get a set it must not compare, and the mistake
  /// would be a ranking rather than an error.
  pub fn embeddings_of(&self, model: &str) -> Result<Vec<crate::embed::Stored>, StoreError> {
    let mut stmt = self.conn.prepare(
      "SELECT chunk_id, model, dims, vector FROM embeddings WHERE model = ?1 ORDER BY chunk_id",
    )?;
    let rows = stmt.query_map(params![model], |row| {
      Ok((
        row.get::<_, String>(0)?,
        row.get::<_, String>(1)?,
        row.get::<_, i64>(2)? as usize,
        row.get::<_, Vec<u8>>(3)?,
      ))
    })?;
    let mut out = Vec::new();
    for row in rows {
      let (chunk_id, model, dims, bytes) = row?;
      // **A BLOB THAT IS NOT A WHOLE NUMBER OF f32s, OR NOT `dims` OF THEM, IS
      // SKIPPED RATHER THAN TRUNCATED.** A short vector still has a cosine, so
      // reading one would put a damaged row in the ranking at a plausible
      // position; leaving it out is the only answer that does not lie.
      if bytes.len() != dims * 4 {
        continue;
      }
      let vector = bytes
        .chunks_exact(4)
        .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
        .collect();
      out.push(crate::embed::Stored {
        chunk_id,
        model,
        vector,
      });
    }
    Ok(out)
  }

  /// Every symbol with this exact name, ordered by path then line.
  ///
  /// **EXACT, BECAUSE THE QUESTION IS EXACT.** `--kind def <name>` asks whether
  /// a thing with this name already exists -- the Highlander check -- and a
  /// prefix or substring match answers a different and softer question. The
  /// lexical tier beside it is where an approximate search belongs, and it
  /// answers the same query in the same envelope.
  /// Every symbol in one FILE, in the order a reader meets them (AC-24.3's
  /// `--outline`).
  ///
  /// **THE SIBLING OF [`Self::symbols_named`] AND IT ASKS THE OTHER QUESTION.**
  /// `symbols_named` asks *where is this name*, exactly, because the Highlander
  /// question is exact; this asks *what is in this file*, which is the question
  /// an agent asks INSTEAD of reading the whole file -- and answering it from
  /// the index is where the saving is, not in racing grep.
  pub fn symbols_in(&self, path: &str) -> Result<Vec<crate::index::symbols::Symbol>, StoreError> {
    self.symbol_rows(
      "SELECT path, lang, name, kind, start_line, end_line
         FROM symbols WHERE path = ?1 ORDER BY start_line, name",
      path,
    )
  }

  pub fn symbols_named(
    &self,
    name: &str,
  ) -> Result<Vec<crate::index::symbols::Symbol>, StoreError> {
    self.symbol_rows(
      "SELECT path, lang, name, kind, start_line, end_line
         FROM symbols WHERE name = ?1 ORDER BY path, start_line",
      name,
    )
  }

  /// The one place a `symbols` row becomes a [`crate::index::symbols::Symbol`].
  ///
  /// **EXTRACTED RATHER THAN COPIED when the second query arrived** -- the
  /// mapping carries a real decision (an unknown language comes back as the
  /// empty name rather than being invented), and a second copy of it would hold
  /// that decision in two places for exactly as long as nobody changed one.
  fn symbol_rows(
    &self,
    sql: &str,
    bind: &str,
  ) -> Result<Vec<crate::index::symbols::Symbol>, StoreError> {
    let mut stmt = self.conn.prepare(sql)?;
    let rows = stmt.query_map(params![bind], |row| {
      Ok((
        row.get::<_, String>(0)?,
        row.get::<_, String>(1)?,
        row.get::<_, String>(2)?,
        row.get::<_, String>(3)?,
        row.get::<_, i64>(4)?,
        row.get::<_, i64>(5)?,
      ))
    })?;
    let mut out = Vec::new();
    for row in rows {
      let (path, lang, name, kind, start_line, end_line) = row?;
      out.push(crate::index::symbols::Symbol {
        path,
        // **THE STORED LANGUAGE IS MATCHED TO THE ROSTER RATHER THAN LEAKED AS
        // A `String`.** `Symbol::lang` is `&'static str` because the roster is
        // the vocabulary; a row naming a language this build has never heard of
        // is a row no surface can render, and it comes back as the empty name
        // rather than being invented.
        lang: crate::index::symbols::LANGUAGES
          .iter()
          .find(|(known, _)| *known == lang)
          .map_or("", |(known, _)| *known),
        name,
        kind: match kind.as_str() {
          "ref" => crate::index::symbols::SymbolKind::Ref,
          _ => crate::index::symbols::SymbolKind::Def,
        },
        span: crate::index::symbols::Span {
          start_line: start_line as u32,
          end_line: end_line as u32,
        },
      });
    }
    Ok(out)
  }

  /// Every source row, ordered by path then position.
  pub fn src_sections(&self) -> Result<Vec<crate::index::source::Section>, StoreError> {
    let mut stmt = self.conn.prepare(
      "SELECT path, seq, start_line, end_line, kind, name, name_parts, body
         FROM src_sections ORDER BY path, seq",
    )?;
    let rows = stmt.query_map([], |row| {
      Ok(crate::index::source::Section {
        path: row.get(0)?,
        seq: row.get::<_, i64>(1)? as u32,
        start_line: row.get::<_, i64>(2)? as u32,
        end_line: row.get::<_, i64>(3)? as u32,
        kind: row.get(4)?,
        name: row.get(5)?,
        name_parts: row.get(6)?,
        body: row.get(7)?,
      })
    })?;
    let mut out = Vec::new();
    for row in rows {
      out.push(row?);
    }
    Ok(out)
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
    Ok(
      self
        .search_hits(query)?
        .into_iter()
        .map(|row| row.section)
        .collect(),
    )
  }

  /// [`Self::search`], with each hit's first match in its BODY located by
  /// the engine: the byte offset in `body` where FTS5 marked a matched
  /// token, or `None` when the match is in the heading alone (issue 0195).
  ///
  /// **THE ENGINE LOCATES IT, THROUGH `highlight()` ON THE SAME QUERY.** A
  /// Rust re-match of the operator's expression would be a second matcher
  /// that agrees with FTS5's tokenizer, stemming and operators only until it
  /// does not. `highlight` inserts a mark before each matched token and
  /// changes nothing else, so the text before the FIRST mark is exactly
  /// `body`'s prefix and its length is the offset. The mark is a
  /// private-use character; a body that already carries one gets no offset
  /// rather than a guessed one.
  pub fn search_hits(&self, query: &str) -> Result<Vec<SearchRow>, StoreError> {
    let mut stmt = self.conn.prepare(
      "SELECT owner_type, owner_id, file, seq, heading, level, body, \
       highlight(doc_sections, 6, ?2, ''), rank FROM doc_sections WHERE doc_sections MATCH ?1 \
       ORDER BY rank, file, seq",
    )?;
    let rows = stmt.query_map(params![query, MATCH_MARK.to_string()], |row| {
      let section = section_from(row)?;
      let marked: String = row.get(7)?;
      let at = marked
        .find(MATCH_MARK)
        .filter(|_| !section.body.contains(MATCH_MARK));
      // **FTS5's OWN RANK, CARRIED RATHER THAN RECOMPUTED.** The rows were
      // already ordered by it; publishing the quantity that did the ordering
      // is what lets a surface show a score without inventing one. It is
      // negative and lower is better, and it is comparable only within this
      // tier -- which is why the envelope never blends tiers (S4).
      let rank: f64 = row.get(8)?;
      Ok(SearchRow { section, at, rank })
    })?;
    let mut out = Vec::new();
    for row in rows {
      out.push(row?);
    }
    Ok(out)
  }

  /// One `src_sections` row a query matched, with where it matched and its
  /// rank.
  ///
  /// **THE SOURCE TIER'S ROWS ARE THE SAME LEXICAL ANSWER AS PROSE'S, from a
  /// different table with a different tokeniser.** They join the LEXICAL group
  /// rather than forming one of their own: a tier is a group and a corpus is an
  /// entry, so code arriving in the corpus does not add a tier.
  pub fn search_source(&self, query: &str) -> Result<Vec<SourceRow>, StoreError> {
    let mut stmt = self.conn.prepare(
      "SELECT path, seq, start_line, end_line, kind, name, name_parts, body, \
       highlight(src_sections, 7, ?2, ''), rank FROM src_sections \
       WHERE src_sections MATCH ?1 ORDER BY rank, path, seq",
    )?;
    let rows = stmt.query_map(params![query, MATCH_MARK.to_string()], |row| {
      let section = crate::index::source::Section {
        path: row.get(0)?,
        seq: row.get::<_, i64>(1)? as u32,
        start_line: row.get::<_, i64>(2)? as u32,
        end_line: row.get::<_, i64>(3)? as u32,
        kind: row.get(4)?,
        name: row.get(5)?,
        name_parts: row.get(6)?,
        body: row.get(7)?,
      };
      let marked: String = row.get(8)?;
      let at = marked
        .find(MATCH_MARK)
        .filter(|_| !section.body.contains(MATCH_MARK));
      let rank: f64 = row.get(9)?;
      Ok(SourceRow { section, at, rank })
    })?;
    let mut out = Vec::new();
    for row in rows {
      out.push(row?);
    }
    Ok(out)
  }

  fn doc_sections_query(
    &self,
    sql: &str,
    args: impl rusqlite::Params,
  ) -> Result<Vec<DocSection>, StoreError> {
    let mut stmt = self.conn.prepare(sql)?;
    let rows = stmt.query_map(args, section_from)?;
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
        if RECORD_TIMESTAMPS.contains(&name.as_str())
          || RECORD_WRITE_METADATA.contains(&name.as_str())
        {
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
