//! AT-07.2 / AC-07.2 (ST0076 WP-07): **`intent search --target <target>`, the
//! MCP tool's `target` and the SQL door's `resolved.target` ask one question:
//! the written references a current resolved row joins to exactly that target,
//! a key naming other targets too marked as one of N. It narrows and is
//! narrowed by every other filter, lists the references alone when nothing else
//! is asked, keeps no definition, and takes no hit from a stale file, which the
//! note names. A target no resolved row names is refused with the nearest
//! targets when any end the same way, and otherwise its empty answer says that
//! a target nothing references and a misspelt one read the same. An index with
//! no stored resolution is refused with the remedy `intent index resolve` and
//! never answered as an empty list.**
//!
//! **RED ON THE COMMIT BEFORE THIS ONE**: there was no `--target`, so a caller
//! who wanted the references to one definition had name-matched occurrences
//! and nothing else.

use crate::common::level_three::{
  HELPER, INITIALIZE, TWO, both, envelope, estate, resolve, run, structural, tool_text,
};
use intentsvcs::search::UNKNOWN_TARGET;
use serde_json::json;

/// The `path:line` of each structural hit in an envelope, in order.
fn places(answer: &serde_json::Value) -> Vec<String> {
  structural(answer)
    .into_iter()
    .map(|(place, _)| place)
    .collect()
}

#[test]
fn a_target_lists_the_references_resolved_to_it_and_narrows_every_door() {
  let dir = estate();
  let root = dir.path();
  resolve(root, Ok(both()));

  let alone = envelope(root, &["--target", HELPER]);
  assert_eq!(
    places(&alone),
    vec!["src/lib.rs:2", "src/two.rs:2"],
    "the references alone, and never the definition: {alone}"
  );
  assert_eq!(
    (&alone["query"], &alone["target"]),
    (
      &json!(format!("target {HELPER}")),
      &json!({"target": HELPER, "named": true})
    )
  );
  let (out, err, code) = run(&["search", "--target", "crate::b::other()"], root);
  assert_eq!(code, 0, "{err}");
  assert!(
    out.contains("src/lib.rs:3") && out.contains("one of 2: crate::a::other(), crate::b::other()"),
    "a key naming another target too is marked one of N: {out:?}"
  );

  for (args, expected, what) in [
    (
      vec!["--context", "helper"],
      vec!["src/lib.rs:2", "src/two.rs:2"],
      "a context narrowed to the target keeps its references and no definition",
    ),
    (
      vec!["--context", "other"],
      vec![],
      "a name the target's references do not carry",
    ),
    (
      vec!["--outline", "src/lib.rs"],
      vec!["src/lib.rs:2"],
      "an outline lists the file's references to the target",
    ),
    (
      vec!["--path", "src/two.rs"],
      vec!["src/two.rs:2"],
      "a path narrows it",
    ),
    (
      vec!["--subkind", "call"],
      vec!["src/lib.rs:2", "src/two.rs:2"],
      "a subkind narrows it",
    ),
    (vec!["--in", "Nothing"], vec![], "a container narrows it"),
    (vec!["--lang", "elixir"], vec![], "a language narrows it"),
    (
      vec!["--limit", "1"],
      vec!["src/lib.rs:2"],
      "the cap applies",
    ),
    (
      vec!["helper"],
      vec!["src/lib.rs:2", "src/two.rs:2"],
      "a text query keeps the structural hits the target joins",
    ),
  ] {
    let mut asked = args.clone();
    asked.extend(["--target", HELPER]);
    let answer = envelope(root, &asked);
    assert_eq!(places(&answer), expected, "{what}: {asked:?} {answer}");
  }
  let lexical = envelope(root, &["helper", "--target", HELPER]);
  assert!(
    lexical["groups"]
      .as_array()
      .expect("groups")
      .iter()
      .filter(|group| group["tier"] == "lexical")
      .all(|group| group["hits"].as_array().is_some_and(Vec::is_empty)),
    "a text hit names no target, so the lexical group keeps nothing: {lexical}"
  );

  let (out, err, code) = run(&["search", "--target", HELPER, "--kind", "def"], root);
  assert_eq!(
    (code, out.as_str()),
    (1, ""),
    "a kind filter that leaves references out is refused: {err}"
  );
  assert!(
    err.contains("leaves every reference out"),
    "the refusal says why: {err:?}"
  );
  let (_, err, code) = run(&["--daemon", "search", "--target", HELPER], root);
  assert_eq!(code, 2, "{err}");
  assert!(
    err.contains("a search asked by target"),
    "`--daemon` is refused rather than answered in this process: {err:?}"
  );

  let (session, frames) = crate::common::mcp_session(
    root,
    Some(testkit::fixture_home()),
    &[
      INITIALIZE,
      &format!(
        r#"{{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{{"name":"intent_search","arguments":{{"target":"{HELPER}"}}}}}}"#
      ),
      &format!(
        r#"{{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{{"name":"intent_search","arguments":{{"context":"helper","target":"{HELPER}","kind":"def"}}}}}}"#
      ),
    ],
  );
  assert!(
    session.status.success(),
    "the MCP session failed: {}",
    String::from_utf8_lossy(&session.stderr)
  );
  let (refused, text) = tool_text(&frames, 2);
  assert!(!refused, "{text}");
  let from_mcp: serde_json::Value = serde_json::from_str(&text).expect("the envelope is JSON");
  assert_eq!(
    from_mcp["groups"], alone["groups"],
    "the tool's `target` answers what `--target` answers"
  );
  let (refused, text) = tool_text(&frames, 3);
  assert!(
    refused && text.contains("leaves every reference out"),
    "the tool refuses with the terminal's words: {text:?}"
  );

  let (out, err, code) = run(
    &[
      "search",
      "--sql",
      &format!(
        "select path, line, name from resolved where target = '{HELPER}' order by path, line"
      ),
      "--json",
    ],
    root,
  );
  assert_eq!(code, 0, "{err}");
  let page: serde_json::Value = serde_json::from_str(&out).expect("the page is JSON");
  assert_eq!(
    page["rows"],
    json!([
      {"path": "src/lib.rs", "line": 2, "name": "helper"},
      {"path": "src/two.rs", "line": 2, "name": "helper"},
    ]),
    "the SQL door asks the same question of the same rows: {page}"
  );
}

#[test]
fn a_target_no_resolved_row_names_is_refused_near_or_answered_empty() {
  let dir = estate();
  let root = dir.path();
  resolve(root, Ok(both()));

  let (out, err, code) = run(&["search", "--target", "helper"], root);
  assert_eq!((code, out.as_str()), (1, ""), "{err}");
  assert!(
    err.contains("no resolved reference names `helper`")
      && err.contains(&format!("`{HELPER}`"))
      && err.contains("remedy: "),
    "a target that ends the same way is named in the refusal: {err:?}"
  );
  let (_, err, code) = run(&["search", "--target", "other"], root);
  assert_eq!(code, 1, "{err}");
  assert!(
    err.contains("`crate::a::other()`, `crate::b::other()`"),
    "every near target, in order: {err:?}"
  );

  let (out, err, code) = run(&["search", "--target", "nothing::here()"], root);
  assert_eq!(
    (code, out.as_str()),
    (0, ""),
    "a target nothing is near answers empty: {err}"
  );
  assert!(
    err.contains(&format!(
      "no resolved reference names `nothing::here()`: {UNKNOWN_TARGET}"
    )),
    "and says it cannot tell unreferenced from misspelt: {err:?}"
  );
  let empty = envelope(root, &["--target", "nothing::here()"]);
  assert_eq!(
    empty["target"],
    json!({"target": "nothing::here()", "named": false})
  );

  let (session, frames) = crate::common::mcp_session(
    root,
    Some(testkit::fixture_home()),
    &[
      INITIALIZE,
      r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"intent_search","arguments":{"target":"helper"}}}"#,
      r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"intent_search","arguments":{"target":"nothing::here()"}}}"#,
    ],
  );
  assert!(session.status.success());
  let (refused, text) = tool_text(&frames, 2);
  assert!(
    refused && text.contains(&format!("`{HELPER}`")),
    "the tool names the near target too: {text:?}"
  );
  let (refused, text) = tool_text(&frames, 3);
  assert!(!refused, "{text}");
  let answer: serde_json::Value = serde_json::from_str(&text).expect("the envelope is JSON");
  assert!(
    answer["note"]
      .as_str()
      .is_some_and(|note| note.contains(UNKNOWN_TARGET)),
    "the tool's empty answer carries the note: {answer}"
  );

  let table = intent_cli::dispatch::table();
  let search = intent_cli::dispatch::all_entries(&table)
    .find(|entry| entry.path == "search")
    .expect("the search row");
  for (face, text) in [
    (
      "when_to_use",
      search.when_to_use.as_deref().unwrap_or_default(),
    ),
    ("mcp_instructions", table.mcp_instructions.as_str()),
  ] {
    assert!(
      text.contains(UNKNOWN_TARGET),
      "{face} does not say `{UNKNOWN_TARGET}`"
    );
  }
}

#[test]
fn a_stale_file_gives_no_hit_and_the_answer_names_it() {
  let dir = estate();
  let root = dir.path();
  resolve(root, Ok(both()));

  // Moved on disk and not yet read: the index still joins its reference.
  std::fs::write(
    root.join("src/two.rs"),
    format!("{TWO}pub fn later() {{}}\n"),
  )
  .expect("edit");
  let answer = envelope(root, &["--target", HELPER]);
  assert_eq!(
    (places(&answer), &answer["index"]["stale"]),
    (vec!["src/lib.rs:2".to_string()], &json!(["src/two.rs"])),
    "a file that moved on disk takes no hit and is named: {answer}"
  );

  // Read again: its resolved rows describe bytes the index no longer holds.
  let (_, err, code) = run(&["index", "rebuild"], root);
  assert_eq!(code, 0, "{err}");
  let answer = envelope(root, &["--target", HELPER]);
  assert_eq!(
    (
      places(&answer),
      &answer["index"]["resolution"]["rust"]["stale"]
    ),
    (vec!["src/lib.rs:2".to_string()], &json!(["src/two.rs"])),
    "{answer}"
  );
  let (_, err, code) = run(&["search", "--target", HELPER], root);
  assert_eq!(code, 0, "{err}");
  assert!(
    err.contains("level 3 in `rust` (stale)") && err.contains("stale: src/two.rs"),
    "the note names the stale file: {err:?}"
  );
}

#[test]
fn an_index_with_no_stored_resolution_is_refused_with_its_remedy() {
  let dir = estate();
  let root = dir.path();

  for args in [
    vec!["search", "--target", HELPER],
    vec!["search", "--context", "helper", "--target", HELPER],
  ] {
    let (out, err, code) = run(&args, root);
    assert_eq!(
      (code, out.as_str()),
      (1, ""),
      "refused, never an empty list: {args:?} {err}"
    );
    assert!(
      err.contains("no language's references have been resolved")
        && err.contains("remedy: ")
        && err.contains("`intent index resolve`"),
      "the refusal carries its remedy: {err:?}"
    );
  }

  let (session, frames) = crate::common::mcp_session(
    root,
    Some(testkit::fixture_home()),
    &[
      INITIALIZE,
      &format!(
        r#"{{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{{"name":"intent_search","arguments":{{"target":"{HELPER}"}}}}}}"#
      ),
    ],
  );
  assert!(session.status.success());
  let (refused, text) = tool_text(&frames, 2);
  assert!(
    refused && text.contains("`intent index resolve`"),
    "the tool refuses with the same remedy: {text:?}"
  );
}
