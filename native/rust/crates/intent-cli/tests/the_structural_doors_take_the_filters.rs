//! AT-04.1 / AC-04.1 (ST0076 WP-04): **the structural doors take the filters, a
//! search asked only its filters lists the symbols that pass them, a symbol hit
//! says what it is and where it sits, and every face spells a fact the way the
//! `symbols` column does.**
//!
//! **THE FIRST ARM IS RED ON THE COMMIT BEFORE THIS ONE, AND NOT BECAUSE A FLAG
//! IS NEW.** `--context` read no filter at all, so `--context new --lang elixir`
//! over a Rust-only project answered both Rust `new`s at exit 0: a filter the
//! caller could see on their own command line, dropped without a word.

use std::path::Path;

fn run(args: &[&str], cwd: &Path) -> (String, String, i32) {
  let out = crate::common::intent()
    .args(args)
    .current_dir(cwd)
    .env("HOME", testkit::fixture_home())
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run the v3 binary");
  (
    String::from_utf8_lossy(&out.stdout).into_owned(),
    String::from_utf8_lossy(&out.stderr).into_owned(),
    out.status.code().unwrap_or(-1),
  )
}

const SOURCE: &str = "pub struct AddressError;

impl AddressError {
  pub fn new() -> Self {
    AddressError
  }

  pub fn is_local(&self) -> bool {
    true
  }
}

pub struct Other;

impl Other {
  pub fn new() -> Self {
    Other
  }
}
";

const INITIALIZE: &str = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"drive","version":"0"}}}"#;

fn estate() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  let (_, err, code) = run(&["init", "structural-filters-fixture"], root);
  assert_eq!(code, 0, "fixture init failed: {err}");
  let (_, err, code) = run(&["lang", "init", "rust"], root);
  assert_eq!(code, 0, "fixture lang init failed: {err}");
  std::fs::create_dir_all(root.join("src")).expect("mkdir src");
  std::fs::write(root.join("src/lib.rs"), SOURCE).expect("write the fixture source");
  let (_, err, code) = run(&["index", "rebuild"], root);
  assert_eq!(code, 0, "fixture index failed: {err}");
  dir
}

fn hits(root: &Path, args: &[&str]) -> Vec<serde_json::Value> {
  let mut full = vec!["search"];
  full.extend_from_slice(args);
  full.push("--json");
  let (out, err, code) = run(&full, root);
  assert_eq!(code, 0, "{full:?} failed: {err}");
  let answer: serde_json::Value = serde_json::from_str(&out).expect("the envelope is JSON");
  answer["groups"]
    .as_array()
    .expect("groups")
    .iter()
    .filter(|group| group["tier"] == "structural")
    .flat_map(|group| group["hits"].as_array().cloned().unwrap_or_default())
    .collect()
}

fn containers(hits: &[serde_json::Value]) -> Vec<String> {
  let mut out: Vec<String> = hits
    .iter()
    .map(|hit| hit["container"].as_str().unwrap_or("-").to_string())
    .collect();
  out.sort();
  out
}

#[test]
fn context_and_outline_honour_the_filters_and_a_hit_names_its_columns() {
  let dir = estate();
  let root = dir.path();

  let every = hits(root, &["--context", "new", "--kind", "def"]);
  assert_eq!(
    containers(&every),
    vec!["AddressError", "Other"],
    "the control: both definitions of `new` answer with no filter"
  );
  assert!(
    hits(root, &["--context", "new", "--lang", "elixir"]).is_empty(),
    "a language filter narrows `--context` as it narrows a text query"
  );
  assert_eq!(
    hits(root, &["--outline", "src/lib.rs", "--limit", "1"]).len(),
    1,
    "the cap applies to an outline as it does to a text query"
  );
  let (_, err, code) = run(&["search", "--context", "new", "--tier", "lexical"], root);
  assert_eq!(
    code, 1,
    "a tier filter that leaves out the structural tier is refused, never ignored: {err}"
  );
  assert!(
    err.contains("structural tier"),
    "the refusal names the tier that answers: {err:?}"
  );

  let one = hits(root, &["--context", "new", "--in", "AddressError"]);
  assert_eq!(containers(&one), vec!["AddressError"], "{one:#?}");
  assert_eq!(one[0]["subkind"], "assoc_fn");
  assert_eq!(one[0]["container_kind"], "impl");
  assert_eq!(one[0]["arity"], 0);
  assert_eq!(one[0]["level"], 1);

  let methods = hits(root, &["--outline", "src/lib.rs", "--subkind", "method"]);
  let names: Vec<&str> = methods
    .iter()
    .filter_map(|hit| hit["name"].as_str())
    .collect();
  assert_eq!(
    names,
    vec!["is_local"],
    "an outline narrowed to methods: {methods:#?}"
  );

  let text = hits(root, &["new", "--subkind", "assoc_fn", "--in", "Other"]);
  assert_eq!(
    containers(&text),
    vec!["Other"],
    "a text query takes the same filters"
  );

  let (out, frames) = crate::common::mcp_session(
    root,
    Some(testkit::fixture_home()),
    &[
      INITIALIZE,
      r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"intent_search","arguments":{"context":"new","in":"AddressError"}}}"#,
    ],
  );
  assert!(
    out.status.success(),
    "the MCP session failed: {}",
    String::from_utf8_lossy(&out.stderr)
  );
  let text = frames
    .iter()
    .find(|frame| frame["id"] == 2)
    .expect("a response to the call")["result"]["content"][0]["text"]
    .as_str()
    .expect("the tool answers text-wrapped JSON")
    .to_string();
  let from_mcp: serde_json::Value = serde_json::from_str(&text).expect("the envelope is JSON");
  let (cli, err, code) = run(
    &[
      "search",
      "--context",
      "new",
      "--in",
      "AddressError",
      "--json",
    ],
    root,
  );
  assert_eq!(code, 0, "{err}");
  let from_cli: serde_json::Value = serde_json::from_str(&cli).expect("the envelope is JSON");
  assert_eq!(
    from_mcp["groups"], from_cli["groups"],
    "the tool's `in` answers what the terminal's `--in` answers"
  );
}

/// AC-04.1's "names the level that answered", on the terminal: the note goes to
/// stderr beside the rows, and a text query that found no symbol says nothing.
#[test]
fn the_terminal_names_the_level_and_the_row_names_the_subkind() {
  let dir = estate();
  let root = dir.path();
  let (out, err, code) = run(&["search", "--context", "new", "--in", "Other"], root);
  assert_eq!(code, 0, "{err}");
  assert!(
    out.contains("  def assoc_fn  Other  new"),
    "the row carries the subkind and the container: {out:?}"
  );
  assert!(
    err.contains("note: level 1: read from the file's syntax")
      && err.contains("in Rust, inside a macro invocation"),
    "the level that answered and Rust's gap are named: {err:?}"
  );
  let (_, err, code) = run(&["search", "quokka"], root);
  assert_eq!(code, 0, "{err}");
  assert!(
    !err.contains("level 1"),
    "no symbol hit, no level note: {err:?}"
  );
}

/// AC-04.1's query-less form, on both faces: **a search asked only its filters
/// lists the symbols that pass them, and a search asked nothing names that form
/// in its remedy.** Red on the bank before it: the terminal and the tool both
/// refused `--subkind method --in AddressError` as nothing to search for, so the
/// methods of a type could not be listed without already knowing its file.
#[test]
fn a_search_asked_only_its_filters_lists_the_symbols_that_pass_them() {
  let dir = estate();
  let root = dir.path();

  let methods = hits(root, &["--subkind", "method", "--in", "AddressError"]);
  let names: Vec<&str> = methods
    .iter()
    .filter_map(|hit| hit["name"].as_str())
    .collect();
  assert_eq!(
    names,
    vec!["is_local"],
    "each of its methods, once: {methods:#?}"
  );
  let inside = hits(root, &["--in", "AddressError", "--kind", "def"]);
  let names: Vec<&str> = inside
    .iter()
    .filter_map(|hit| hit["name"].as_str())
    .collect();
  assert_eq!(
    names,
    vec!["new", "is_local"],
    "a container alone lists what sits in it: {inside:#?}"
  );

  let (out, err, code) = run(
    &["search", "--subkind", "method", "--in", "AddressError"],
    root,
  );
  assert_eq!(code, 0, "{err}");
  assert!(
    out.contains("  def method  AddressError  is_local"),
    "the terminal prints the row: {out:?}"
  );

  let (_, err, code) = run(&["search"], root);
  assert_eq!(code, 1, "a search asked nothing is refused: {err}");
  assert!(
    err.contains("nothing to search for") && err.contains("--subkind") && err.contains("--in"),
    "the remedy names the query-less form: {err:?}"
  );

  let (out, frames) = crate::common::mcp_session(
    root,
    Some(testkit::fixture_home()),
    &[
      INITIALIZE,
      r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"intent_search","arguments":{"subkind":"method","in":"AddressError"}}}"#,
      r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"intent_search","arguments":{}}}"#,
    ],
  );
  assert!(
    out.status.success(),
    "the MCP session failed: {}",
    String::from_utf8_lossy(&out.stderr)
  );
  let result = |id: i64| {
    frames
      .iter()
      .find(|frame| frame["id"] == id)
      .expect("a response to the call")["result"]
      .clone()
  };
  let listed = result(2);
  assert_ne!(listed["isError"], true, "{listed}");
  let from_mcp: serde_json::Value = serde_json::from_str(
    listed["content"][0]["text"]
      .as_str()
      .expect("the tool answers text-wrapped JSON"),
  )
  .expect("the envelope is JSON");
  let (cli, err, code) = run(
    &[
      "search",
      "--subkind",
      "method",
      "--in",
      "AddressError",
      "--json",
    ],
    root,
  );
  assert_eq!(code, 0, "{err}");
  let from_cli: serde_json::Value = serde_json::from_str(&cli).expect("the envelope is JSON");
  assert_eq!(
    from_mcp["groups"], from_cli["groups"],
    "the tool lists what the terminal lists for the same filters"
  );
  let refused = result(3);
  assert_eq!(refused["isError"], true, "{refused}");
  let text = refused["content"][0]["text"].as_str().unwrap_or_default();
  assert!(
    text.contains("nothing to search for") && text.contains("`subkind`") && text.contains("`in`"),
    "the tool's refusal names the query-less form in its own parameter names: {text:?}"
  );
}

#[test]
fn an_unknown_subkind_is_refused_by_both_faces_with_the_same_words() {
  let dir = estate();
  let root = dir.path();
  let (_, err, code) = run(&["search", "--context", "new", "--subkind", "strcut"], root);
  assert_eq!(code, 1, "an unknown subkind is refused: {err}");
  assert!(
    err.contains("`strcut` is not a subkind") && err.contains("assoc_fn"),
    "the refusal names the word and the roster: {err:?}"
  );

  let (out, frames) = crate::common::mcp_session(
    root,
    Some(testkit::fixture_home()),
    &[
      INITIALIZE,
      r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"intent_search","arguments":{"context":"new","subkind":"strcut"}}}"#,
    ],
  );
  assert!(out.status.success());
  let result = &frames
    .iter()
    .find(|frame| frame["id"] == 2)
    .expect("a response to the call")["result"];
  assert_eq!(result["isError"], true, "{result}");
  let text = result["content"][0]["text"].as_str().unwrap_or_default();
  assert!(
    text.contains("`strcut` is not a subkind") && text.contains("assoc_fn"),
    "the tool refuses with the terminal's words: {text:?}"
  );
}

#[test]
fn the_sql_door_and_the_ddl_face_carry_the_symbol_columns() {
  let dir = estate();
  let root = dir.path();
  let (out, err, code) = run(
    &[
      "search",
      "--sql",
      "select name, subkind, container, container_kind, trait_name, arity, arity_min, qualifier, level from symbols where name = 'new' order by container",
      "--json",
    ],
    root,
  );
  assert_eq!(code, 0, "the SQL door refused the columns: {err}");
  let page: serde_json::Value = serde_json::from_str(&out).expect("the page is JSON");
  assert_eq!(page["returned"], 2, "{page}");
  assert_eq!(page["rows"][0]["container"], "AddressError", "{page}");
  assert_eq!(page["rows"][0]["subkind"], "assoc_fn", "{page}");

  let (out, err, code) = run(&["schema", "ddl.sql"], root);
  assert_eq!(code, 0, "{err}");
  for column in [
    "subkind",
    "container",
    "container_kind",
    "trait_name",
    "arity",
    "arity_min",
    "qualifier",
    "level",
  ] {
    assert!(
      out.contains(&format!("  {column} ")),
      "the published DDL names `{column}`"
    );
  }
}
