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

  let from_mcp = crate::common::search_tool_answer(
    root,
    Some(testkit::fixture_home()),
    r#"{"outline":"src/lib.rs"}"#,
  );
  // Each face reconciles before it answers (issue 0669), and each reconcile
  // stamps the index (issue 0369), so the two envelopes carry two stamps. The
  // stamp is when the index was reconciled, not part of the answer's shape.
  assert_eq!(
    without_the_stamp(from_cli),
    without_the_stamp(from_mcp),
    "the two faces answer differently"
  );
}

/// An envelope without its reconcile stamp, which says when the index was
/// reconciled rather than anything about the answer.
#[cfg(feature = "lang-rust")]
fn without_the_stamp(mut envelope: serde_json::Value) -> serde_json::Value {
  assert!(
    envelope["index"]["reconciled_at"].is_string(),
    "the envelope carries no reconcile stamp: {envelope}"
  );
  envelope["index"]
    .as_object_mut()
    .expect("the freshness block is an object")
    .remove("reconciled_at");
  envelope
}

/// Give the fixture's source a function the index has not seen.
#[cfg(feature = "lang-rust")]
fn add_function(root: &Path, name: &str) {
  let source = root.join("src/lib.rs");
  let mut text = std::fs::read_to_string(&source).expect("read the fixture source");
  text.push_str(&format!("\npub fn {name}() {{}}\n"));
  std::fs::write(&source, text).expect("write the fixture source");
}

/// The door reconciled before it answered, so it names `name`, which the tree
/// gained after the index was last reconciled.
#[cfg(feature = "lang-rust")]
fn assert_reconciled_and_names(answer: &serde_json::Value, name: &str) {
  assert_eq!(
    answer["index"]["reconciled"], true,
    "the door answered without reconciling first: {answer}"
  );
  let named = answer["groups"]
    .as_array()
    .expect("groups")
    .iter()
    .filter_map(|group| group["hits"].as_array())
    .flatten()
    .any(|hit| hit["name"] == name);
  assert!(
    named,
    "the door answered about the tree as the last reconcile left it, without `{name}`: {answer}"
  );
}

/// Issue 0669: **EVERY IN-PROCESS DOOR OF `intent search` RECONCILES BEFORE IT
/// ANSWERS.** Each door is asked straight after the tree gains a function the
/// index has not seen, so it can name it only by reconciling for itself. Until
/// 0669 only the text query did, and an outline, a caller list or a
/// filters-only listing answered about the tree as the last reconcile left it.
#[cfg(feature = "lang-rust")]
#[test]
fn every_door_of_the_terminal_reconciles_before_it_answers() {
  let dir = estate();
  let root = dir.path();

  add_function(root, "added_before_the_outline");
  assert_reconciled_and_names(
    &envelope(&["search", "--outline", "src/lib.rs", "--json"], root),
    "added_before_the_outline",
  );

  add_function(root, "added_before_the_context");
  assert_reconciled_and_names(
    &envelope(
      &["search", "--context", "added_before_the_context", "--json"],
      root,
    ),
    "added_before_the_context",
  );

  add_function(root, "added_before_the_listing");
  assert_reconciled_and_names(
    &envelope(&["search", "--subkind", "function", "--json"], root),
    "added_before_the_listing",
  );
}

/// Issue 0669, the tool's face: **THE SAME THREE DOORS OF `intent_search`
/// RECONCILE THROUGH THE SAME STEP.**
#[cfg(feature = "lang-rust")]
#[test]
fn every_door_of_the_search_tool_reconciles_before_it_answers() {
  let dir = estate();
  let root = dir.path();
  let home = Some(testkit::fixture_home());

  add_function(root, "added_before_the_tool_outline");
  assert_reconciled_and_names(
    &crate::common::search_tool_answer(root, home, r#"{"outline":"src/lib.rs"}"#),
    "added_before_the_tool_outline",
  );

  add_function(root, "added_before_the_tool_context");
  assert_reconciled_and_names(
    &crate::common::search_tool_answer(
      root,
      home,
      r#"{"context":"added_before_the_tool_context"}"#,
    ),
    "added_before_the_tool_context",
  );

  add_function(root, "added_before_the_tool_listing");
  assert_reconciled_and_names(
    &crate::common::search_tool_answer(root, home, r#"{"subkind":"function"}"#),
    "added_before_the_tool_listing",
  );
}

/// Issue 0548's script: bash with no extension, which is how Devbin ships every
/// command under `lib/` and `cmd/`, so the index gives it no language at all.
const SCRIPT: &str =
  "#!/usr/bin/env bash\nset -euo pipefail\n\nvendored_paths() {\n  echo lib\n}\n";

/// A project that declares shell, whose grammar this build does not carry, and
/// rust, whose grammar it does.
fn unparsed_estate() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  let (_, err, code) = run(&["init", "unparsed-script-fixture"], root);
  assert_eq!(code, 0, "fixture init failed: {err}");
  for lang in ["rust", "shell"] {
    let (_, err, code) = run(&["lang", "init", lang], root);
    assert_eq!(code, 0, "fixture lang init {lang} failed: {err}");
  }
  std::fs::create_dir_all(root.join("lib")).expect("mkdir lib");
  std::fs::write(root.join("lib/tool"), SCRIPT).expect("write the fixture script");
  std::fs::create_dir_all(root.join("src")).expect("mkdir src");
  std::fs::write(root.join("src/lib.rs"), SOURCE).expect("write the fixture source");
  let (_, err, code) = run(&["index", "rebuild"], root);
  assert_eq!(code, 0, "fixture index failed: {err}");
  dir
}

fn envelope(args: &[&str], root: &Path) -> serde_json::Value {
  let (out, err, code) = run(args, root);
  assert_eq!(code, 0, "{args:?} failed: {err}");
  serde_json::from_str(&out).expect("the envelope is JSON")
}

/// Issue 0548: **A STRUCTURAL ANSWER THAT COULD NOT SEE A DECLARED LANGUAGE
/// SAYS SO.** The script defines `vendored_paths` and no grammar names a symbol
/// in it, so a def query over its directory answering `complete` sent the
/// reader's grep fallback nowhere. The same words as a TEXT question read the
/// script's text, so that answer is whole.
#[test]
fn a_def_query_over_an_unparsed_script_is_incomplete_and_names_the_language() {
  let dir = unparsed_estate();
  let root = dir.path();
  let def = [
    "search",
    "vendored_paths",
    "--kind",
    "def",
    "--path",
    "lib/**",
  ];

  let answer = envelope(&[&def[..], &["--json"]].concat(), root);
  assert_eq!(answer["index"]["complete"], false, "{answer}");
  assert!(
    answer["index"]["unindexed"]
      .as_array()
      .is_some_and(|langs| langs.iter().any(|u| u["lang"] == "shell")),
    "the envelope names the language it could not see: {answer}"
  );

  let (_, err, code) = run(&def, root);
  assert_eq!(code, 0, "{err}");
  assert!(
    err.contains("shell"),
    "the terminal names it too, on stderr: {err:?}"
  );

  let answer = envelope(&["search", "vendored_paths", "--json"], root);
  assert_eq!(
    answer["index"]["complete"], true,
    "a text question read the script: {answer}"
  );
  assert!(
    answer["groups"][0]["hits"]
      .as_array()
      .is_some_and(|hits| hits.iter().any(|h| h["path"] == "lib/tool")),
    "and found it: {answer}"
  );
}

/// Issue 0548: **AN EMPTY OUTLINE OF A FILE THE INDEX CANNOT PARSE IS NOT AN
/// ANSWER**, and the envelope says which of the two reasons it is.
#[test]
fn an_outline_of_a_file_the_index_cannot_parse_is_incomplete_and_says_why() {
  let dir = unparsed_estate();
  let answer = envelope(&["search", "--outline", "lib/tool", "--json"], dir.path());
  assert_eq!(answer["index"]["complete"], false, "{answer}");
  assert_eq!(
    answer["index"]["unindexed"][0]["reason"], "no-language",
    "a file with no extension has no language the index parses: {answer}"
  );
}

/// Issue 0548's boundary: **A QUESTION THAT CANNOT REACH THE UNPARSED LANGUAGE
/// IS STILL WHOLE**, or `complete` would mean nothing in every project that
/// declares shell.
#[cfg(feature = "lang-rust")]
#[test]
fn a_structural_answer_that_leaves_the_unparsed_language_out_stays_complete() {
  let dir = unparsed_estate();
  let root = dir.path();
  let answer = envelope(&["search", "--outline", "src/lib.rs", "--json"], root);
  assert_eq!(
    answer["index"]["complete"], true,
    "an outline of a parsed file: {answer}"
  );
  let answer = envelope(
    &[
      "search",
      "parse_disabled",
      "--kind",
      "def",
      "--lang",
      "rust",
      "--json",
    ],
    root,
  );
  assert_eq!(
    answer["index"]["complete"], true,
    "`--lang rust` asks nothing of shell: {answer}"
  );
}
