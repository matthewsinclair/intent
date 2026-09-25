//! AT-07.1 / AC-07.1 (ST0076 WP-07): **every search answer carries
//! `index.resolution`; a reference one current resolved row names answers at
//! level 3 with its target flat on the hit, one several rows name keeps its
//! syntax level and lists them as `candidates`, and one in a stale file keeps
//! its syntax level; and the terminal, the explorer and the MCP tool say the
//! same, in words one function owns and the register quotes verbatim.**
//!
//! **RED ON THE COMMIT BEFORE THIS ONE, AND NOT BECAUSE A FIELD IS NEW.** The
//! level-3 core stored what a run resolved and every door answered around it: a
//! reference the run had joined to its definition still read level 1, and no
//! answer said whether level 3 was there to be had.

use crate::common::level_three::{
  CARGO_TOML, HELPER, Handed, INITIALIZE, TWO, both, envelope, estate, facade, resolve, resolve_as,
  run, structural, tool_text,
};
use intent_cli::tui::app::App;
use intent_cli::tui::{commands, run as tui, views};
use intentsvcs::index::resolved::{FAILED, MISSING, Resolver, STALE, UNRESOLVED, Unresolved};
use intentsvcs::search::{SearchQuery, level_words, resolution_phrase};
use serde_json::json;

/// The hit at `place` in an envelope's structural group.
fn hit_at(answer: &serde_json::Value, place: &str) -> serde_json::Value {
  structural(answer)
    .into_iter()
    .find(|(at, _)| at == place)
    .map(|(_, hit)| hit)
    .unwrap_or_else(|| panic!("no structural hit at {place}: {answer}"))
}

/// An explorer whose palette is the real vocabulary, as the pane's own arms
/// build it.
fn explorer() -> App {
  let mut app = App::explore();
  app.commands = commands::vocabulary(&intent_cli::spine::build(&intent_cli::dispatch::table()));
  app
}

#[test]
fn a_reference_one_row_names_is_level_three_and_one_several_name_lists_them() {
  let dir = estate();
  let root = dir.path();
  resolve(root, Ok(both()));

  let helper = envelope(root, &["--context", "helper"]);
  assert_eq!(
    helper["index"]["resolution"],
    json!({}),
    "every answer carries the key, and a current run with nothing stale names nothing: {helper}"
  );
  let call = hit_at(&helper, "src/lib.rs:2");
  assert_eq!(
    (
      &call["level"],
      &call["target"],
      &call["target_path"],
      &call["target_line"]
    ),
    (&json!(3), &json!(HELPER), &json!("src/lib.rs"), &json!(6)),
    "one resolved row: level 3, with the definition flat on the hit: {call}"
  );
  let definition = hit_at(&helper, "src/lib.rs:6");
  assert!(
    definition["level"] == 1 && definition.get("target").is_none(),
    "a definition resolves to nothing: {definition}"
  );

  let other = envelope(root, &["--context", "other"]);
  let call = hit_at(&other, "src/lib.rs:3");
  assert_eq!(
    (&call["level"], call.get("target"), &call["candidates"]),
    (
      &json!(1),
      None,
      &json!([{"target": "crate::a::other()"}, {"target": "crate::b::other()"}])
    ),
    "two resolved rows keep the syntax level and list both: {call}"
  );

  let (out, err, code) = run(&["search", "--context", "helper"], root);
  assert_eq!(code, 0, "{err}");
  assert!(
    out.contains(&format!("helper();  -> {HELPER}  src/lib.rs:6")),
    "the terminal row says where the reference points: {out:?}"
  );
  assert!(
    err.contains(&format!("level 3: {}", level_words(3))),
    "the note names the level that answered: {err:?}"
  );
  let (out, err, code) = run(&["search", "--context", "other"], root);
  assert_eq!(code, 0, "{err}");
  assert!(
    out.contains("other();  one of 2: crate::a::other(), crate::b::other()"),
    "the terminal row lists the candidates: {out:?}"
  );

  let (session, frames) = crate::common::mcp_session(
    root,
    Some(testkit::fixture_home()),
    &[
      INITIALIZE,
      r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"intent_search","arguments":{"context":"helper"}}}"#,
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
    (&from_mcp["groups"], &from_mcp["index"]["resolution"]),
    (&helper["groups"], &helper["index"]["resolution"]),
    "the tool answers what the terminal answers"
  );

  let answer = facade(root)
    .context("helper", &SearchQuery::default())
    .expect("the door answers in process");
  let rows = views::search_rows(&answer);
  assert!(
    rows
      .iter()
      .any(|row| row.value.contains(&format!("-> {HELPER}  src/lib.rs:6"))),
    "the explorer's row says where the reference points: {rows:?}"
  );
}

#[test]
fn a_stale_file_keeps_its_syntax_level_and_each_state_is_named() {
  let dir = estate();
  let root = dir.path();
  resolve(root, Ok(both()));

  std::fs::write(
    root.join("src/two.rs"),
    format!("{TWO}pub fn later() {{}}\n"),
  )
  .expect("edit");
  let (_, err, code) = run(&["index", "rebuild"], root);
  assert_eq!(code, 0, "{err}");
  let answer = envelope(root, &["--context", "helper"]);
  let moved = hit_at(&answer, "src/two.rs:2");
  assert!(
    moved["level"] == 1 && moved.get("target").is_none(),
    "a reference in a file changed since it was resolved keeps its syntax level: {moved}"
  );
  assert_eq!(
    hit_at(&answer, "src/lib.rs:2")["level"],
    3,
    "the file that did not change still answers at level 3"
  );
  assert_eq!(
    answer["index"]["resolution"],
    json!({"rust": {"state": STALE, "tool": "fixture-analyzer", "stale": ["src/two.rs"], "stale_total": 1}}),
    "{answer}"
  );
  let stale_words = format!(
    "level 3 in `rust` (stale): {} -- stale: src/two.rs",
    resolution_phrase(STALE)
  );
  let (_, err, code) = run(&["search", "--context", "helper"], root);
  assert_eq!(code, 0, "{err}");
  assert!(err.contains(&stale_words), "the terminal names it: {err:?}");
  let in_process = facade(root)
    .context("helper", &SearchQuery::default())
    .expect("the door answers in process");
  let rows = views::search_rows(&in_process);
  let note = views::freshness_note(&in_process).expect("a symbol answer has a note");
  let mut app = explorer();
  tui::arrive(&mut app, &rows, None, Some(note));
  let hint = tui::screen_for(&app, &rows, 240).hint;
  assert!(
    hint.contains(&stale_words),
    "the explorer's INFO row names it: {hint:?}"
  );

  resolve(
    root,
    Err(Unresolved::Failed {
      path: Some("src/lib.rs".to_string()),
      line: Some(3),
      detail: "the build script panicked".to_string(),
    }),
  );
  let answer = envelope(root, &["--context", "helper"]);
  assert_eq!(
    answer["index"]["resolution"]["rust"],
    json!({
      "state": FAILED,
      "tool": "fixture-analyzer",
      "path": "src/lib.rs",
      "line": 3,
      "detail": "the build script panicked",
      "stale": ["src/two.rs"],
      "stale_total": 1,
    }),
    "{answer}"
  );
  assert_eq!(
    hit_at(&answer, "src/lib.rs:2")["level"],
    3,
    "what the earlier run stored still answers at level 3"
  );
  let (_, err, _) = run(&["search", "--context", "helper"], root);
  assert!(
    err.contains(&format!(
      "level 3 in `rust` (failed): {} -- fixture-analyzer at src/lib.rs:3: the build script panicked -- stale: src/two.rs",
      resolution_phrase(FAILED)
    )),
    "{err:?}"
  );

  resolve(
    root,
    Err(Unresolved::Missing {
      detail: "fixture-analyzer is not on PATH".to_string(),
    }),
  );
  let answer = envelope(root, &["--context", "helper"]);
  assert_eq!(
    (
      &answer["index"]["resolution"]["rust"]["state"],
      &answer["index"]["resolution"]["rust"]["detail"]
    ),
    (&json!(MISSING), &json!("fixture-analyzer is not on PATH")),
    "{answer}"
  );
  let (_, err, _) = run(&["search", "--context", "helper"], root);
  assert!(
    err.contains(&format!(
      "level 3 in `rust` (missing): {}",
      resolution_phrase(MISSING)
    )),
    "{err:?}"
  );
}

/// Hold the project's `Cargo.toml`, or not, as the index reads it.
fn manifest_held(root: &std::path::Path, held: bool) {
  let path = root.join("Cargo.toml");
  if held {
    std::fs::write(&path, CARGO_TOML).expect("write Cargo.toml");
  } else {
    std::fs::remove_file(&path).expect("remove Cargo.toml");
  }
  let (_, err, code) = run(&["index", "rebuild"], root);
  assert_eq!(code, 0, "{err}");
}

/// A carried language is named, as `unresolved` with no run on record or as its
/// failed run, only where the index holds its reader's manifest (vc,
/// 2026-09-17). The arm hands the facade its reader rather than depending on
/// what this build's `readers()` holds.
#[test]
fn a_carried_language_is_named_only_where_the_index_holds_its_manifest() {
  let handed: Vec<Box<dyn Resolver>> = vec![Box::new(Handed(Ok(both())))];
  let asked = |root: &std::path::Path| {
    facade(root)
      .with_resolvers(&handed)
      .context("helper", &SearchQuery::default())
      .expect("the door answers")
  };

  let dir = estate();
  let root = dir.path();
  manifest_held(root, false);
  assert!(
    asked(root).index.resolution.is_empty(),
    "no run on record and no Cargo.toml: the reader does not apply, and Rust is not named"
  );
  resolve_as(
    root,
    Some("rust"),
    Err(Unresolved::NotApplicable {
      detail: "the index holds no Cargo.toml".to_string(),
    }),
  );
  let (out, err, code) = run(&["index", "status"], root);
  assert_eq!(code, 0, "{err}");
  assert!(
    out.contains("resolution: rust  failed  fixture-analyzer"),
    "a run asked for by name over a project with no Cargo.toml records its failure: {out}"
  );
  assert!(
    asked(root).index.resolution.is_empty(),
    "and the answer does not name a failed run the project holds nothing for"
  );
  manifest_held(root, true);
  assert_eq!(
    asked(root).index.resolution["rust"].state,
    FAILED,
    "once the index holds the manifest, the failed run is named"
  );

  let other = estate();
  let root = other.path();
  let answer = asked(root);
  let state = &answer.index.resolution["rust"];
  assert_eq!(
    (state.state.as_str(), state.tool.as_deref()),
    (UNRESOLVED, Some("fixture-analyzer")),
    "with its manifest held and no run on record, the language is unresolved"
  );
  let note = answer.symbol_note().expect("a symbol answer has a note");
  assert!(
    note.contains(&format!(
      "level 3 in `rust` (unresolved): {} -- a person resolves it by running `intent index resolve`, which runs the project's build through fixture-analyzer",
      resolution_phrase(UNRESOLVED)
    )),
    "the verb is named as a person's, with what it runs: {note}"
  );

  let none: Vec<Box<dyn Resolver>> = Vec::new();
  assert!(
    facade(root)
      .with_resolvers(&none)
      .context("helper", &SearchQuery::default())
      .expect("the door answers")
      .index
      .resolution
      .is_empty(),
    "a build that resolves nothing has nothing unresolved"
  );
}

/// The register quotes the one home's words, so a change to a state's meaning
/// that leaves the published text behind fails here.
#[test]
fn the_register_quotes_level_threes_words_verbatim() {
  let table = intent_cli::dispatch::table();
  let search = intent_cli::dispatch::all_entries(&table)
    .find(|entry| entry.path == "search")
    .expect("the search row");
  let when = search
    .when_to_use
    .as_deref()
    .expect("search declares when_to_use");
  for (face, text) in [
    ("when_to_use", when),
    ("mcp_instructions", table.mcp_instructions.as_str()),
  ] {
    for words in [
      level_words(3),
      resolution_phrase(MISSING),
      resolution_phrase(FAILED),
      resolution_phrase(STALE),
      resolution_phrase(UNRESOLVED),
    ] {
      assert!(
        text.contains(words),
        "{face} does not say `{words}`: {text}"
      );
    }
  }
}
