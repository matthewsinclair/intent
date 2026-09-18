//! Issue 0443: **a search that meets a store fault says the search could not
//! be answered, and only an expression FTS5 itself refused is the reader's.**
//!
//! Every `StoreError::Sqlite` used to become `BadQuery`, whose remedy sends the
//! reader after an unbalanced `(`. So a damaged index, a busy database or an
//! I/O fault each told the reader their CORRECT query was wrong, and every
//! retry seemed to confirm it because the query kept failing. The reconcile a
//! search runs first failed as `Store`, which renders "could not update the
//! runtime store" to a reader who asked a question.
//!
//! **THE CLASSIFIER IS PINNED BY DRIVING, NOT BY ITS OWN STRINGS.** The
//! recognised messages are SQLite's text, and SQLite moves (3.46.0 to 3.53.2
//! in 3.1.0). A test that asserted the strings would pass on a bundle whose
//! wording had moved away from them; the malformed expression below is run
//! against the bundled library, so a wording change reds here.
//!
//! **BOTH DIRECTIONS ARE ARMS**, because each fix has an over-correction on its
//! far side: a store fault reported as a bad query is the defect, and a bad
//! query reported as a store fault would be the fix going too far.

use crate::common::Fixture;
use intentsvcs::facade::FacadeError;
use intentsvcs::remedy::Remedy;
use intentsvcs::store::StoreError;

/// A real error from the bundled SQLite, produced by running `sql` against a
/// real fts5 table -- never a constructed message.
fn driven(sql: &str) -> StoreError {
  let db = rusqlite::Connection::open_in_memory().expect("open");
  db.execute_batch("CREATE VIRTUAL TABLE t USING fts5(body);")
    .expect("an fts5 table in the bundled library");
  let err = db
    .prepare(sql)
    // A MATCH refuses at STEP time, so the refusal is one of the rows the
    // query yields -- counting them would call it an answer.
    .and_then(|mut stmt| {
      stmt
        .query_map([], |_| Ok(()))?
        .collect::<Result<Vec<()>, _>>()
    })
    .expect_err("the statement must be refused, or this arm tests nothing");
  StoreError::Sqlite(err)
}

/// A store error carrying `code` and `msg`, for the faults that cannot be
/// driven honestly in a unit test (corruption, a held lock).
fn failure(code: std::os::raw::c_int, msg: &str) -> StoreError {
  StoreError::Sqlite(rusqlite::Error::SqliteFailure(
    rusqlite::ffi::Error::new(code),
    Some(msg.to_string()),
  ))
}

#[test]
fn a_malformed_expression_driven_through_the_bundle_is_the_readers() {
  for expression in [r#"( "hello""#, "hello AND", "hello OR", "NOT"] {
    let err = driven(&format!("SELECT * FROM t WHERE t MATCH '{expression}'"));
    assert!(
      err.is_bad_fts5_expression(),
      "`{expression}` is the reader's fault and must be recognised -- if this reds after a SQLite \
       move, the bundle's wording changed and the allowlist follows it: {err}"
    );
  }
}

/// `hello NEAR` is answered, not refused. It is the reason the old remedy lost
/// its `NEAR` clause, and this is the arm that says so.
#[test]
fn a_trailing_near_is_answered_not_refused() {
  let db = rusqlite::Connection::open_in_memory().expect("open");
  db.execute_batch("CREATE VIRTUAL TABLE t USING fts5(body); INSERT INTO t VALUES ('hello near');")
    .expect("an fts5 table");
  let rows: i64 = db
    .query_row(
      "SELECT count(*) FROM t WHERE t MATCH 'hello NEAR'",
      [],
      |r| r.get(0),
    )
    .expect("`hello NEAR` is a query fts5 answers");
  assert_eq!(rows, 1);
}

/// **THE EXCLUSION A TIDY-UP WOULD UNDO.** The core SQL parser's syntax error
/// reads like fts5's without the prefix; a recogniser that matched on "syntax
/// error" would call every malformed statement on the `search_sql` door a bad
/// search expression, and accuse the reader of the wrong thing.
#[test]
fn the_core_parsers_syntax_error_is_not_an_fts5_expression() {
  let err = driven("SELEC body FROM t");
  let text = err.to_string();
  assert!(
    text.contains("syntax error") && !text.contains("fts5"),
    "the specimen must be the core parser's refusal, or this arm is about nothing: {text}"
  );
  assert!(!err.is_bad_fts5_expression(), "{text}");
}

#[test]
fn a_missing_table_is_not_the_readers() {
  let err = driven("SELECT * FROM absent WHERE absent MATCH 'hello'");
  assert!(!err.is_bad_fts5_expression(), "{err}");
}

#[test]
fn corruption_and_a_held_lock_are_not_the_readers() {
  use rusqlite::ffi::{SQLITE_BUSY, SQLITE_CORRUPT, SQLITE_ERROR};
  let busy = failure(SQLITE_BUSY, "database is locked");
  assert!(busy.is_busy() && !busy.is_bad_fts5_expression());
  assert!(!failure(SQLITE_CORRUPT, "database disk image is malformed").is_bad_fts5_expression());
  // The six `fts5:` messages 3.53.2 adds are all corruption reports. The last
  // is 0442's own damage, which must never be reported as the reader's query.
  for msg in [
    "fts5: checksum mismatch for table \"doc_sections\"",
    "fts5: corrupt structure record for table \"doc_sections\"",
    "fts5: corruption found reading blob 17 from table \"doc_sections\"",
    "fts5: corruption in table \"doc_sections\"",
    "fts5: corruption on page 3, segment 1, table \"doc_sections\"",
    "fts5: missing row 42 from content table \"doc_sections_content\"",
  ] {
    assert!(
      !failure(SQLITE_CORRUPT, msg).is_bad_fts5_expression()
        && !failure(SQLITE_ERROR, msg).is_bad_fts5_expression(),
      "a prefix test on `fts5:` would call this the reader's fault: {msg}"
    );
  }
}

/// The store underneath a LIVE facade loses its prose index -- `Facade::open`
/// lays the schema down again, so breaking the file between two facades is
/// undone by the call meant to meet it.
fn break_the_index(fx: &Fixture) {
  let db = rusqlite::Connection::open(fx.root().join("intent/.cache/intent.db"))
    .expect("open the store directly");
  db.execute_batch(
    "DROP TABLE doc_sections; CREATE TABLE doc_sections (unrelated TEXT);
     DROP TABLE src_sections; CREATE TABLE src_sections (unrelated TEXT);",
  )
  .expect("replace the fts5 tables with plain ones");
}

#[test]
fn a_search_over_a_broken_index_could_not_be_answered_and_blames_nothing_typed() {
  let fx = Fixture::new();
  let facade = fx.facade_on_disk();
  break_the_index(&fx);

  let err = facade
    .search_all("hello", &intentsvcs::search::SearchQuery::default())
    .expect_err("a search over a broken index must fail rather than answer");
  assert!(
    matches!(err, FacadeError::SearchUnanswerable { .. }),
    "a store fault is not a bad query: {err:?}"
  );
  assert_eq!(err.to_string(), "the search `hello` could not be answered");
  let remedy = err.remedy();
  // Rebuild, because it is the remedy that was DRIVEN to cure this fault;
  // `index status` is not named because it reads a damaged index as healthy.
  assert!(remedy.contains("intent index rebuild"), "{remedy}");
  assert!(
    !remedy.contains("unbalanced"),
    "the remedy must not send the reader after their query: {remedy}"
  );
}

/// **THE OVER-CORRECTION ARM.** Without it the change could make every syntax
/// error look like corruption and still pass the arm above.
#[test]
fn a_malformed_expression_is_still_refused_as_the_readers() {
  let fx = Fixture::new();
  let facade = fx.facade_on_disk();

  let err = facade
    .search_all("(hello", &intentsvcs::search::SearchQuery::default())
    .expect_err("an unbalanced paren is malformed under any escaping");
  assert!(matches!(err, FacadeError::BadQuery { .. }), "{err:?}");
  let remedy = err.remedy();
  assert!(remedy.contains("unbalanced"), "{remedy}");
  assert!(
    !remedy.contains("NEAR"),
    "`hello NEAR` is answered, so naming it sends the reader after nothing: {remedy}"
  );
}

/// The reconcile a search runs first is part of ANSWERING it, so its failure
/// is the search's, not an update the reader never asked for.
#[test]
fn a_reconcile_that_fails_for_a_search_is_the_searchs_failure() {
  let fx = Fixture::new();
  fx.write_file("notes.md", "# Notes\n\nhello\n");
  let mut facade = fx.facade_on_disk();
  break_the_index(&fx);

  let err = facade
    .index_refresh_for_search("hello")
    .expect_err("a reconcile into a broken index must fail");
  assert!(
    matches!(err, FacadeError::SearchUnanswerable { .. }),
    "the search path must not say `could not update the runtime store`: {err:?}"
  );
  assert!(!err.to_string().contains("could not update"), "{err}");
}

/// A held lock keeps the store's own remedy: a rebuild offered for a lock
/// another process holds cannot work.
#[test]
fn a_busy_store_keeps_the_busy_remedy_on_the_search_path() {
  let cause = failure(rusqlite::ffi::SQLITE_BUSY, "database is locked");
  let want = cause.remedy();
  let err = FacadeError::SearchUnanswerable {
    query: "hello".to_string(),
    cause,
  };
  assert_eq!(err.remedy(), want);
  assert!(!err.remedy().contains("index rebuild"), "{}", err.remedy());
}
