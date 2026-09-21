//! **AT-24.3 / AC-24.3: `--outline <path>` and `--context <name>` answer units
//! rather than lines, each one facade call, in the same envelope.**
//!
//! These are the answers grep cannot give, and they are the whole argument for
//! an index earning a call at all. An agent asking what is in a file reads the
//! file; an agent asking where a name is defined and where it occurs runs a
//! grep, a glob and several reads. Both become one call whose rows carry SPANS,
//! which is what replaces read-the-file with read-this-span.
//!
//! **THE FIXTURE COMPILES A GRAMMAR IN, AND THE ARMS SAY SO RATHER THAN
//! SKIPPING.** Every grammar is off by default until hv rules the binary-size
//! line, so a build without one indexes no symbols and these doors correctly
//! answer nothing. Gated on `lang-rust` and driven with
//! `--features intentsvcs/lang-rust`; the ungated arm below is the one that
//! holds in every build, because refusing two questions is not a property of
//! any grammar.

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

const SOURCE: &str = "pub enum Severity {\n  \
                      Low,\n  \
                      High,\n\
                      }\n\n\
                      impl Severity {\n  \
                      pub fn parse_disabled(text: &str) -> usize {\n    \
                      text.len()\n  \
                      }\n\
                      }\n";

fn estate() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  let (_, err, code) = run(&["init", "structural-doors-fixture"], root);
  assert_eq!(code, 0, "fixture init failed: {err}");
  // **THE PROJECT MUST DECLARE THE LANGUAGE, THROUGH THE VERB THAT DECLARES
  // IT.** A language absent from `languages` parses nothing (AC-20.1), so a
  // fixture that skipped this would index no symbols and the arms below would
  // fail for a reason that has nothing to do with the doors. Driven through
  // `lang init` rather than by writing the config, because the config's shape
  // is that verb's to own.
  let (_, err, code) = run(&["lang", "init", "rust"], root);
  assert_eq!(code, 0, "fixture lang init failed: {err}");
  std::fs::create_dir_all(root.join("src")).expect("mkdir src");
  std::fs::write(root.join("src/lib.rs"), SOURCE).expect("write the fixture source");
  let (_, err, code) = run(&["index", "rebuild"], root);
  assert_eq!(code, 0, "fixture index failed: {err}");
  dir
}

/// **EXACTLY ONE QUESTION PER CALL**, and the refusal names the pair it found.
/// This holds in every build, with or without a grammar, because it is about
/// the doors rather than about what they can see.
#[test]
fn two_doors_at_once_is_a_usage_error_that_names_them() {
  let dir = estate();
  let (_, err, code) = run(
    &["search", "--outline", "src/lib.rs", "--context", "Severity"],
    dir.path(),
  );
  assert_eq!(code, 1, "two questions must be refused: {err}");
  assert!(
    err.contains("`--outline`") && err.contains("`--context`"),
    "the refusal names the pair it found: {err:?}"
  );

  let (_, err, code) = run(&["search", "quokka", "--outline", "src/lib.rs"], dir.path());
  assert_eq!(code, 1, "a text query and a door are two questions: {err}");
  assert!(err.contains("a text query"), "{err:?}");
}

/// AC-24.3 proper: **units with spans, in the envelope, from one call.**
#[cfg(feature = "lang-rust")]
#[test]
fn outline_lists_a_files_symbols_and_context_finds_a_name_with_its_occurrences() {
  let dir = estate();
  let root = dir.path();

  let (out, err, code) = run(&["search", "--outline", "src/lib.rs", "--json"], root);
  assert_eq!(code, 0, "outline failed: {err}");
  let answer: serde_json::Value = serde_json::from_str(&out).expect("the envelope is JSON");
  let groups = answer["groups"].as_array().expect("groups");
  assert_eq!(
    groups.len(),
    1,
    "a path names no words, so there is nothing for the lexical tier to answer \
     and an empty lexical group would claim a text search ran: {answer}"
  );
  assert_eq!(groups[0]["tier"], "structural");
  let hits = groups[0]["hits"].as_array().expect("hits");
  assert!(
    hits.iter().any(|h| h["name"] == "parse_disabled"),
    "the file's function is not in its outline: {answer}"
  );
  assert!(
    hits
      .iter()
      .all(|h| h["span"]["start_line"].as_u64().is_some_and(|n| n > 0)),
    "every unit carries the span that makes read-this-span possible: {answer}"
  );

  let (out, err, code) = run(&["search", "--context", "Severity", "--json"], root);
  assert_eq!(code, 0, "context failed: {err}");
  let answer: serde_json::Value = serde_json::from_str(&out).expect("the envelope is JSON");
  let hits = answer["groups"][0]["hits"].as_array().expect("hits");
  assert!(
    hits.iter().any(|h| h["kind"] == "def"),
    "context answers where the name is DEFINED: {answer}"
  );
  assert!(
    hits.iter().any(|h| h["kind"] == "ref"),
    "and where it OCCURS -- name-matched, never a caller: {answer}"
  );
}

/// **THE MCP TOOL ANSWERS THE SAME ENVELOPE FROM THE SAME CALL** (AC-19.2's
/// rule, holding for the doors added later -- which is the whole claim of one
/// envelope).
#[cfg(feature = "lang-rust")]
#[test]
fn the_tool_and_json_answer_the_same_outline() {
  let dir = estate();
  let root = dir.path();
  let (out, err, code) = run(&["search", "--outline", "src/lib.rs", "--json"], root);
  assert_eq!(code, 0, "outline failed: {err}");
  let from_cli: serde_json::Value = serde_json::from_str(&out).expect("JSON");
  assert!(
    !from_cli["groups"][0]["hits"]
      .as_array()
      .expect("hits")
      .is_empty(),
    "the fixture must produce units, or this compares two empty answers"
  );

  let home = testkit::fixture_home();
  let (out, frames) = crate::common::mcp_session(
    root,
    Some(home),
    &[
      r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"drive","version":"0"}}}"#,
      r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"intent_search","arguments":{"outline":"src/lib.rs"}}}"#,
    ],
  );
  assert!(
    out.status.success(),
    "the MCP session failed: {}",
    String::from_utf8_lossy(&out.stderr)
  );
  let text =
    frames.iter().find(|f| f["id"] == 2).expect("a response")["result"]["content"][0]["text"]
      .as_str()
      .expect("text-wrapped JSON");
  let from_mcp: serde_json::Value = serde_json::from_str(text).expect("JSON");
  assert_eq!(from_cli, from_mcp, "the two faces answer differently");
}
