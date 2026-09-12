//! **AT-17.1 to AT-17.4 / AC-17.1 to AC-17.4: `intent search --sql` runs ONE read statement, names
//! both denominators, and auto-detects nothing.**
//!
//! The door exists because a join across the model is a question no verb
//! answers, and it is exposed on MCP -- so what it REFUSES is as much the
//! feature as what it returns. Every refusal below is driven against a real
//! store rather than against the gate in isolation: the gate is unit-tested in
//! `intentsvcs::sql_gate`, and this file asks whether the door as assembled
//! actually stops the thing.
//!
//! **THE ESTATE IS CHECKED AFTER EVERY REFUSED WRITE.** A refusal that happened
//! for some other reason -- a typo'd table, a locked file -- passes an
//! assertion about exit codes and proves nothing about read-onlyness. So the
//! write cases read the row count before and after and assert it did not move.

use std::path::Path;
use std::process::Command;

fn run(args: &[&str], cwd: &Path) -> (String, String, i32) {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(cwd)
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run the v3 binary");
  (
    String::from_utf8_lossy(&out.stdout).into_owned(),
    String::from_utf8_lossy(&out.stderr).into_owned(),
    out.status.code().unwrap_or(-1),
  )
}

/// Two threads, so a `--limit 1` has something to cap and `matched` differs
/// from `returned` -- on a one-row estate the two denominators are the same
/// number and AC-17.4's case cannot be exhibited.
fn estate() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  let (_, err, code) = run(&["init", "sql-door-fixture"], root);
  assert_eq!(code, 0, "fixture init failed: {err}");
  for title in ["The first thread", "The second thread"] {
    let (_, err, code) = run(&["st", "new", title], root);
    assert_eq!(code, 0, "fixture st new failed: {err}");
  }
  dir
}

/// How many threads the store holds, through the door itself.
fn threads(root: &Path) -> i64 {
  let (out, err, code) = run(
    &[
      "search",
      "--sql",
      "select count(*) as n from threads",
      "--json",
    ],
    root,
  );
  assert_eq!(code, 0, "counting threads failed: {err}");
  let page: serde_json::Value = serde_json::from_str(&out).expect("the envelope is JSON");
  page["rows"][0]["n"].as_i64().expect("a count came back")
}

#[test]
fn a_read_statement_answers_rows_and_columns() {
  let dir = estate();
  let root = dir.path();
  let (out, err, code) = run(
    &[
      "search",
      "--sql",
      "select id, title from threads order by id",
    ],
    root,
  );
  assert_eq!(code, 0, "the read failed: {err}");
  assert!(
    out.contains("ST0001") && out.contains("The first thread"),
    "{out:?}"
  );
  assert!(
    out
      .lines()
      .next()
      .is_some_and(|line| line.contains("id") && line.contains("title")),
    "the table does not lead with its column names: {out:?}"
  );
}

/// **THE HEADLINE REFUSAL, with the estate as its control** (AT-17.1, AC-17.1).
#[test]
fn a_write_is_refused_and_the_store_does_not_move() {
  let dir = estate();
  let root = dir.path();
  let before = threads(root);

  for statement in [
    "delete from threads",
    "update threads set title = 'gone'",
    "insert into threads (id) values ('ST9999')",
    "create table smuggled (a int)",
    "drop table threads",
  ] {
    let (_, err, code) = run(&["search", "--sql", statement], root);
    assert_eq!(code, 1, "{statement:?} was not refused: {err}");
    assert!(
      err.contains("READ-ONLY"),
      "{statement:?} was refused for some other reason, which proves nothing about the door: {err}"
    );
  }

  assert_eq!(
    threads(root),
    before,
    "a refused write moved the store anyway"
  );
}

/// AT-17.1: a second statement is where a read door becomes a write door --
/// SQLite prepares the first and hands back the rest.
#[test]
fn a_second_statement_is_refused_and_neither_half_runs() {
  let dir = estate();
  let root = dir.path();
  let before = threads(root);

  let (_, err, code) = run(&["search", "--sql", "select 1; delete from threads"], root);
  assert_eq!(code, 1, "the batch was not refused: {err}");
  assert!(err.contains("ONE statement"), "{err:?}");
  assert_eq!(threads(root), before, "the batch's second half ran");
}

/// **THE OVER-REFUSAL CONTROL** (AT-17.1). A door that refused every semicolon
/// would pass the test above while making an ordinary query unreachable.
#[test]
fn a_semicolon_inside_a_literal_is_not_a_second_statement() {
  let dir = estate();
  let root = dir.path();
  let (out, err, code) = run(&["search", "--sql", "select ';' as s"], root);
  assert_eq!(code, 0, "an ordinary query was refused: {err}");
  assert!(out.contains(';'), "the literal did not come back: {out:?}");
}

/// **`ATTACH` IS THE REFUSAL THAT IS ABOUT REACH RATHER THAN WRITING** (AT-17.1). A
/// read-only connection reads another file on the machine perfectly happily,
/// and this door is exposed on MCP.
#[test]
fn attach_and_pragma_are_refused_as_out_of_reach() {
  let dir = estate();
  let root = dir.path();
  let elsewhere = dir.path().join("elsewhere.db");
  let attach = format!("attach database '{}' as x", elsewhere.display());
  for (statement, expect) in [
    (attach.as_str(), "ATTACH"),
    ("pragma user_version", "PRAGMA"),
  ] {
    let (_, err, code) = run(&["search", "--sql", statement], root);
    assert_eq!(code, 1, "{statement:?} was not refused: {err}");
    assert!(
      err.contains(expect),
      "the refusal does not name what it refused: {err:?}"
    );
  }
  assert!(
    !elsewhere.exists(),
    "the refused ATTACH created the file it named"
  );
}

/// AT-17.4 / AC-17.4: a capped result is never a silent subset.
#[test]
fn a_capped_result_reports_both_denominators() {
  let dir = estate();
  let root = dir.path();
  let (out, err, code) = run(
    &[
      "search",
      "--sql",
      "select id from threads",
      "--limit",
      "1",
      "--json",
    ],
    root,
  );
  assert_eq!(code, 0, "the capped read failed: {err}");
  let page: serde_json::Value = serde_json::from_str(&out).expect("the envelope is JSON");
  assert_eq!(page["returned"], 1, "{page}");
  assert_eq!(page["matched"], 2, "{page}");
  assert_eq!(page["truncated"], true, "{page}");

  // The terminal rendering says the same thing in its own words, on stderr so a
  // piped table stays a table.
  let (_, err, code) = run(
    &["search", "--sql", "select id from threads", "--limit", "1"],
    root,
  );
  assert_eq!(code, 0, "the capped read failed: {err}");
  assert!(
    err.contains("1 of 2"),
    "the terminal rendering does not report the cap: {err:?}"
  );
}

/// AT-17.2 / AC-17.2: the envelope carries the store's schema version.
#[test]
fn the_envelope_carries_the_stores_schema_version() {
  let dir = estate();
  let root = dir.path();
  let (out, err, code) = run(&["search", "--sql", "select 1 as one", "--json"], root);
  assert_eq!(code, 0, "the read failed: {err}");
  let page: serde_json::Value = serde_json::from_str(&out).expect("the envelope is JSON");
  let version = page["schema_version"].as_i64().expect("a schema version");
  assert!(version > 0, "a store that opened carries a version: {page}");
  // **THE DOOR IS THE ONLY WAY TO LEARN IT, which is why it is in the
  // envelope**: `pragma user_version` is refused as out of reach.
  let (_, err, code) = run(&["search", "--sql", "pragma user_version"], root);
  assert_eq!(code, 1, "the pragma answered: {err}");
}

/// AT-17.3 / AC-17.3: nothing is auto-detected from the query's first word.
#[test]
fn a_bare_query_is_text_even_when_it_begins_with_select() {
  let dir = estate();
  let root = dir.path();
  // A text search for the word `select` is a search, not a statement: it
  // answers as a search does (exit 0, no rows, the unindexed note) rather than
  // as SQL.
  let (out, err, code) = run(&["search", "select id from threads"], root);
  assert_eq!(code, 0, "the text search failed: {err}");
  assert!(
    !out.contains("ST0001"),
    "the text query was run as SQL: {out:?}"
  );
}

/// AT-17.3: both doors at once, or neither, is a usage error rather than a
/// precedence rule -- any order of preference silently drops half of what was asked for.
#[test]
fn a_query_and_sql_together_or_neither_is_a_usage_error() {
  let dir = estate();
  let root = dir.path();
  let (_, err, code) = run(&["search", "anything", "--sql", "select 1"], root);
  assert_eq!(code, 1, "both doors at once was accepted: {err}");
  assert!(err.contains("two different questions"), "{err:?}");

  let (_, err, code) = run(&["search"], root);
  assert_eq!(code, 1, "neither door was accepted: {err}");
  assert!(err.contains("nothing to search for"), "{err:?}");
}

/// **THE WORK BUDGET IS WHAT STOPS AN AGENT HANGING THE PROCESS** (AT-17.1), and a row
/// cap cannot reach it: this statement returns nothing until it finishes, and
/// it never finishes. The budget is counted in SQLite instructions rather than
/// seconds because nothing in this workspace may read a clock (D42), which is
/// also why this case asserts a REFUSAL rather than a duration.
#[test]
fn a_statement_that_never_finishes_is_stopped() {
  let dir = estate();
  let root = dir.path();

  let (_, err, code) = run(
    &[
      "search",
      "--sql",
      "with recursive forever(x) as (select 1 union all select x + 1 from forever) \
       select count(*) from forever",
    ],
    root,
  );

  assert_eq!(code, 1, "the runaway statement was not stopped: {err}");
  assert!(
    err.contains("more work than the SQL door allows"),
    "it stopped for some other reason: {err:?}"
  );
}

/// AT-17.2, AC-17.2's second half: **`--json` and the MCP tool answer the SAME envelope
/// for the same statement.** Two spellings of one answer is two things to keep
/// true, and the MCP side is the one nobody reads by eye.
#[test]
fn the_mcp_tool_and_json_answer_the_same_envelope() {
  let dir = estate();
  let root = dir.path();
  let statement = "select id, title from threads order by id";

  let (out, err, code) = run(&["search", "--sql", statement, "--json"], root);
  assert_eq!(code, 0, "the CLI read failed: {err}");
  let from_cli: serde_json::Value = serde_json::from_str(&out).expect("the envelope is JSON");

  let call = format!(
    r#"{{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{{"name":"intent_search","arguments":{{"sql":{}}}}}}}"#,
    serde_json::Value::from(statement)
  );
  let (out, frames) = crate::common::mcp_session(
    root,
    None,
    &[
      r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"drive","version":"0"}}}"#,
      &call,
    ],
  );
  assert!(
    out.status.success(),
    "the MCP session failed: {}",
    String::from_utf8_lossy(&out.stderr)
  );
  let answer = frames
    .iter()
    .find(|f| f["id"] == 2)
    .expect("a response to the call");
  let text = answer["result"]["content"][0]["text"]
    .as_str()
    .expect("the tool answers text-wrapped JSON");
  let from_mcp: serde_json::Value =
    serde_json::from_str(text).expect("the tool's envelope is JSON");

  assert_eq!(
    from_cli, from_mcp,
    "the two faces answer differently for one statement"
  );
}

/// **THE DOOR READS THE INDEX, WHICH IS HALF THE QUESTION IT EXISTS FOR.**
///
/// It did not, from the day it shipped until 2026-09-12: SQLite's fts5 module
/// issues `PRAGMA data_version` of its own when a virtual table is initialised,
/// and the authorizer refused pragmas wholesale -- so `select path from
/// src_sections limit 2` came back `the SQL door refuses PRAGMA`, blaming the
/// operator for a statement they did not write. The door's stated purpose is a
/// question that crosses the MODEL and the INDEX, and the index half is exactly
/// what it could not read.
#[test]
fn an_fts_table_can_be_read_through_the_door() {
  let dir = estate();
  let root = dir.path();
  let (_, err, code) = run(&["index", "rebuild"], root);
  assert_eq!(code, 0, "the fixture index failed: {err}");

  for table in ["doc_sections", "src_sections"] {
    let (out, err, code) = run(
      &[
        "search",
        "--sql",
        &format!("select count(*) as n from {table}"),
        "--json",
      ],
      root,
    );
    assert_eq!(code, 0, "`{table}` is not readable through the door: {err}");
    let page: serde_json::Value = serde_json::from_str(&out).expect("the envelope is JSON");
    assert!(
      page["rows"][0]["n"].as_i64().is_some(),
      "`{table}` answered no count: {page}"
    );
  }

  // A MATCH, not just a plain select: the match is what an agent joining the
  // model to the index actually writes, and it initialises the tokenizer too.
  let (out, err, code) = run(
    &[
      "search",
      "--sql",
      "select file from doc_sections where doc_sections match 'thread' limit 3",
    ],
    root,
  );
  assert_eq!(code, 0, "an FTS match is refused: {err}");
  let _ = out;
}

/// **THE NARROWING DID NOT OPEN THE DOOR**, and this is the arm that proves it
/// rather than the one that assumes it: the allowlist is one pragma BY NAME and
/// only as a read, so a pragma that would change the connection is still
/// refused -- and the connection is the store's, so one that got through would
/// outlive the statement that set it.
#[test]
fn a_state_changing_pragma_is_still_refused_and_the_store_stays_read_only() {
  let dir = estate();
  let root = dir.path();
  let before = threads(root);

  let (_, err, code) = run(&["search", "--sql", "PRAGMA query_only = 0"], root);
  assert_eq!(code, 1, "a state-changing pragma must be refused: {err}");
  assert!(
    err.contains("PRAGMA"),
    "the refusal names what it refused: {err:?}"
  );

  // **AND THE NEXT STATEMENT STILL CANNOT WRITE**, which is the half a refusal
  // alone does not establish: the refusal could have fired after the pragma
  // took effect.
  let (_, err, code) = run(
    &["search", "--sql", "delete from threads where id = 'ST0001'"],
    root,
  );
  assert_eq!(code, 1, "the write was not refused: {err}");
  assert_eq!(
    threads(root),
    before,
    "the estate moved -- the door is not read-only"
  );
}
