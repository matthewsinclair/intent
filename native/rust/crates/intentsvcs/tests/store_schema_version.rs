//! AT-02.7 / AC-02.7: the store's schema stamp -- it is written, it is
//! checked, and it cannot be left behind when the DDL moves.
//!
//! **The defect this closes was found by dogfooding, not by reasoning** (dc,
//! 2026-08-15). `CREATE TABLE IF NOT EXISTS` makes applying the DDL to an
//! existing database a no-op, so a store written before a shape change opened
//! CLEANLY, handed back a connection, and failed later at whichever query first
//! named a column that was not there. The open path succeeded on a database it
//! could not read; the only thing standing between that and a wrong answer was
//! which verb the operator happened to run.
//!
//! Under D01 as reversed that is not a development nuisance. The store is
//! durable truth and the committed extract is what travels (D34), so a database
//! the tool half-understands is the one artefact that cannot be shrugged off.
//!
//! Three properties, and the third is the one that decays without a test:
//!
//! 1. A new store is stamped.
//! 2. A store this build does not speak is REFUSED AT OPEN, with a remedy that
//!    says what to do -- distinctly for "wrong version" and "no version".
//! 3. **`SCHEMA_VERSION` moves when `DDL` moves.** The stamp is hand-kept, and
//!    a hand-kept number beside a generated artefact is exactly the thing this
//!    estate keeps catching after the fact.

use intentsvcs::remedy::Remedy;
use intentsvcs::store::{DDL, SCHEMA_VERSION, Store, StoreError};
use rusqlite::Connection;

fn version_of(path: &std::path::Path) -> i32 {
  Connection::open(path)
    .expect("open")
    .pragma_query_value(None, "user_version", |row| row.get(0))
    .expect("user_version is readable")
}

#[test]
fn a_new_store_is_stamped_with_this_builds_schema_version() {
  let dir = tempfile::tempdir().expect("tempdir");
  let path = dir.path().join("intent.db");

  let store = Store::open(&path).expect("a fresh store opens");
  drop(store);

  assert_eq!(
    version_of(&path),
    SCHEMA_VERSION,
    "a store this binary created carries this binary's schema version"
  );
}

#[test]
fn reopening_a_store_of_the_same_version_is_fine() {
  let dir = tempfile::tempdir().expect("tempdir");
  let path = dir.path().join("intent.db");

  drop(Store::open(&path).expect("first open"));
  drop(Store::open(&path).expect("reopening the same shape must not refuse"));

  assert_eq!(version_of(&path), SCHEMA_VERSION);
}

/// **The regression itself.** A database carrying tables and no stamp is what
/// every store written before today looks like, and dc's was one.
#[test]
fn an_unstamped_store_is_refused_at_open_not_at_the_first_query() {
  let dir = tempfile::tempdir().expect("tempdir");
  let path = dir.path().join("intent.db");

  // A pre-stamp store, built the way the old code did: DDL applied, nothing
  // recorded about what shape it is. The old `criteria` shape is used
  // deliberately -- this is the exact database that opened cleanly and then
  // failed on `no such column: state`.
  {
    let conn = Connection::open(&path).expect("create");
    conn
      .execute_batch(
        "CREATE TABLE threads (id TEXT PRIMARY KEY, title TEXT NOT NULL);
         CREATE TABLE criteria (
           thread_id TEXT NOT NULL, id TEXT NOT NULL, text TEXT NOT NULL,
           kind TEXT NOT NULL, scope TEXT NOT NULL, evidence TEXT, satisfied INTEGER
         );",
      )
      .expect("lay down a pre-stamp schema");
  }
  assert_eq!(version_of(&path), 0, "the fixture really is unstamped");

  let err = match Store::open(&path) {
    Ok(_) => panic!("an unstamped store must be refused, and it opened"),
    Err(e) => e,
  };
  assert!(
    matches!(err, StoreError::SchemaUnstamped),
    "got: {err:?} -- an unstamped store is its own case, not a version mismatch"
  );
  // The message an operator reads has to say what happened without them
  // knowing what a pragma is.
  assert!(
    err.to_string().contains("predates schema versioning"),
    "the message names the cause, got: {err}"
  );
  assert!(
    !err.remedy().is_empty(),
    "a refusal with no remedy is where the original defect ended up"
  );
}

/// A store from a FUTURE build is refused, and the remedy says which end to
/// move.
///
/// Only the newer direction is reachable through `open` today, and the reason
/// is worth stating because the first cut of this test got it wrong by
/// arithmetic: it walked `[SCHEMA_VERSION + 1, SCHEMA_VERSION - 1]`, and at
/// version 1 the second of those is **0, which is not an older version -- it is
/// the absence of one**. SQLite's `user_version` defaults to 0, so 0 can never
/// mean "schema zero"; it is permanently spoken for by the unstamped past. The
/// older-store direction becomes reachable at version 2 and is asserted below
/// against the error value directly until then.
#[test]
fn a_store_from_a_newer_build_is_refused_at_open() {
  let dir = tempfile::tempdir().expect("tempdir");
  let stamp = SCHEMA_VERSION + 1;
  let path = dir.path().join("newer.db");
  {
    let conn = Connection::open(&path).expect("create");
    conn
      .pragma_update(None, "user_version", stamp)
      .expect("stamp");
    conn.execute_batch(DDL).expect("apply");
  }

  let err = match Store::open(&path) {
    Ok(_) => panic!("a store stamped {stamp} must be refused, and it opened"),
    Err(e) => e,
  };
  let StoreError::SchemaMismatch { found, expected } = err else {
    panic!("expected a version mismatch at stamp {stamp}, got: {err:?}");
  };
  assert_eq!((found, expected), (stamp, SCHEMA_VERSION));
}

/// The two directions of a version mismatch read differently.
///
/// A refusal that does not say WHICH END is behind leaves the operator guessing
/// whether to move the tool or the data -- and when the store is ahead, only one
/// of those is even possible: there is no migrating a database backwards into a
/// binary that has never heard of its shape.
#[test]
fn the_mismatch_remedy_points_at_the_end_that_can_actually_move() {
  let ahead = StoreError::SchemaMismatch {
    found: SCHEMA_VERSION + 1,
    expected: SCHEMA_VERSION,
  }
  .remedy();
  let behind = StoreError::SchemaMismatch {
    found: SCHEMA_VERSION,
    expected: SCHEMA_VERSION + 1,
  }
  .remedy();

  assert!(
    ahead.contains("NEWER"),
    "a store ahead of the binary must say so: {ahead}"
  );
  assert!(
    !behind.contains("NEWER"),
    "a store behind the binary is the migration case, not the upgrade case: {behind}"
  );
  assert_ne!(
    ahead, behind,
    "the two directions need different actions and must not share a sentence"
  );
}

/// **The stamp cannot be left behind.**
///
/// A hash of the DDL, pinned here beside the version it describes. Change the
/// schema and this fails; the fix is to bump `SCHEMA_VERSION` and re-pin, in the
/// same commit, which is the moment someone has to think about what an existing
/// database now needs.
///
/// The hash is NOT a version -- it cannot be ordered, so it cannot dispatch a
/// migration. It is only the tripwire that makes the orderable number honest.
/// Both are needed and neither substitutes for the other.
///
/// **It hashes what SQLite acts on, not the file.** The first cut hashed the
/// whole constant, and the first thing that touched the DDL afterwards was a
/// comment -- the openness declarations, which change no table, no column and
/// no constraint. That build demanded a version bump, and obeying it would have
/// refused every existing store to record a change SQLite never sees. A guard
/// that cries wolf on a comment is a guard someone re-pins without reading.
#[test]
fn the_schema_version_is_bumped_whenever_the_ddl_changes() {
  // Whole-line comments only. Stripping `--` wherever it appears would need a
  // SQL tokeniser to avoid eating one inside a string literal -- so instead the
  // assumption that there are none is CHECKED, and a future in-line comment
  // fails here rather than silently changing what this hash means.
  let mut schema = String::new();
  for line in DDL.lines() {
    let t = line.trim();
    if t.starts_with("--") || t.is_empty() {
      continue;
    }
    assert!(
      !t.contains("--"),
      "the DDL grew an in-line comment, so stripping whole lines no longer isolates \
       the schema: {t}"
    );
    schema.push_str(t);
    schema.push('\n');
  }
  assert!(
    schema.contains("CREATE TABLE IF NOT EXISTS threads ("),
    "the strip removed the schema itself"
  );

  // FNV-1a, written out rather than pulled in: the property wanted here is
  // "changes when the input changes", which needs no cryptographic strength
  // and should not add a dependency to the shipped crate to get.
  let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
  for byte in schema.as_bytes() {
    hash ^= u64::from(*byte);
    hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
  }

  // 11 is the ladder learning its own rule: rung 10 was EDITED after a store
  // had run it, so two shapes were stamped 10 and the reader could not tell
  // them apart. A version is a claim about SHAPE -- changing what a rung
  // produces needs a NEW rung. The DDL itself did not move, which is why this
  // hash is unchanged and only the version is.
  //
  // 8 -> 9 -> 10 in one commit, both rungs on the same subject: prose whose
  // only home was a file. `issues.body` is the issue's authored body, which had
  // no column because it had no model field -- the migration read every v2
  // issue file and kept the frontmatter. `attachments` is the authored files
  // beside a thread that no typed document held, and it is the FIRST NEW TABLE
  // since this ladder began: a `CREATE` rather than an `ALTER`, which is the
  // easy rung, because an empty table is a correct representation of a store
  // that never had one and there is nothing to back-fill.
  //
  // 12 is `ingests`: whether the last load from canon finished. A second NEW
  // TABLE, and the same easy rung as `attachments` for the same reason -- an
  // empty table is a correct representation of a store with no load history,
  // and it is the ONLY correct one here. Back-filling a `succeeded` row would
  // assert a load nobody observed; back-filling a `refused` one would block
  // every upgraded project's egest at once. So the rung creates and stops, and
  // `Store::last_ingest` returning `None` carries "no evidence either way"
  // rather than either invented answer.
  const PINNED_SCHEMA_HASH: u64 = 0x2769_d55e_632d_8fd6;
  const PINNED_FOR_VERSION: i32 = 13;

  assert_eq!(
    SCHEMA_VERSION, PINNED_FOR_VERSION,
    "SCHEMA_VERSION moved to {SCHEMA_VERSION}; re-pin PINNED_FOR_VERSION and PINNED_SCHEMA_HASH here \
     in the same commit, and write the migration that gets an existing store from \
     {PINNED_FOR_VERSION} to {SCHEMA_VERSION}"
  );
  assert_eq!(
    hash, PINNED_SCHEMA_HASH,
    "the DDL's SCHEMA changed and SCHEMA_VERSION did not (comments are excluded, so this is \
     a real table, column or constraint).\n\
     `CREATE TABLE IF NOT EXISTS` means an existing store will NOT pick this change up -- it will \
     open cleanly and fail at whichever query first names the new shape.\n\
     Bump SCHEMA_VERSION, re-pin PINNED_SCHEMA_HASH to {hash:#018x}, and ship the migration in the \
     same commit."
  );
}

/// **A STORE BUILT BY AN EARLIER LADDER OPENS AND READS -- the case a suite
/// that always starts fresh cannot see.**
///
/// Found by hv driving `intent st list` against this project's own store, not
/// by any test. Rung 10 was EDITED after that store had already run it: it
/// reached version 10 with an `attachments` table that had no `seq`, the rung
/// is version-gated so it can never run again, and `attachments_of` then
/// learned `ORDER BY seq`. Every read of the canon failed with
/// `no such column: seq`.
///
/// **Every other test here builds its store from the current `DDL`, so every
/// other test gets the current shape and passes.** The defect is reachable only
/// from a store that predates the change -- ours, and any real user's. This is
/// the only test in the file whose fixture is a store this binary did not
/// create, and that is the whole point of it.
///
/// **The rule the failure earned: a version number is a claim about SHAPE, so
/// once any store has run a rung, changing what that rung produces needs a NEW
/// rung.** The old rung's output is already stamped and unreachable.
#[test]
fn a_store_stamped_by_an_earlier_draft_of_a_rung_is_walked_forward_not_refused() {
  let dir = tempfile::tempdir().expect("tempdir");
  let path = dir.path().join("intent.db");

  // Version 10 exactly as the earlier draft of rung 10 left it: `attachments`
  // with no `seq`, keyed on (thread_id, path). Written by hand because no
  // binary that still exists produces this shape.
  {
    let conn = Connection::open(&path).expect("create");
    conn.execute_batch(DDL).expect("apply the current DDL");
    conn
      .execute_batch(
        "DROP TABLE attachments;
         CREATE TABLE attachments (
           thread_id TEXT NOT NULL REFERENCES threads (id) ON DELETE CASCADE,
           path TEXT NOT NULL,
           text TEXT NOT NULL,
           bytes INTEGER NOT NULL,
           sha256 TEXT NOT NULL,
           written_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
           PRIMARY KEY (thread_id, path)
         );
         INSERT INTO threads (id, title, status, created, objective, context, body, preamble)
           VALUES ('ST0001', 't', 'wip', '2026-08-18', '', '', '', '');
         INSERT INTO attachments (thread_id, path, text, bytes, sha256)
           VALUES ('ST0001', 'reference.md', '# R\n', 4, 'deadbeef'),
                  ('ST0001', 'parity/cmd-st.md', '# s\n', 4, 'cafebabe');",
      )
      .expect("lay down the pre-seq shape");
    conn.pragma_update(None, "user_version", 10).expect("stamp");
  }

  let store = Store::open(&path).expect("a store from an earlier ladder must open, not refuse");
  drop(store);

  assert_eq!(
    version_of(&path),
    SCHEMA_VERSION,
    "and it is walked all the way forward"
  );

  let conn = Connection::open(&path).expect("reopen");
  let mut stmt = conn
    .prepare("SELECT seq, path FROM attachments WHERE thread_id = 'ST0001' ORDER BY seq")
    .expect("the column the reader needs now exists");
  let rows: Vec<(i64, String)> = stmt
    .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))
    .expect("query")
    .collect::<Result<_, _>>()
    .expect("rows");

  // **Rows are CARRIED, not dropped.** This store's real counterpart happened
  // to hold none, and a rung that quietly relied on that would be correct once.
  assert_eq!(
    rows,
    vec![
      (0, "reference.md".to_string()),
      (1, "parity/cmd-st.md".to_string())
    ],
    "both rows survive, and `seq` is INSERTION order rather than path order -- \
     a store that already recorded an order keeps it instead of being re-sorted"
  );
}
