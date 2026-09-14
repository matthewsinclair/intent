//! **AT-24.2 / AC-24.2: a tool's description says when to use it and when not
//! to, and the words come from the REGISTER rather than from `mcp.rs`.**
//!
//! A model picks a tool from three things -- the description, the instructions
//! in its context, and what worked last time -- and grep wins all three by
//! default. The half of a description that changes that is **when NOT to reach
//! for this**, which has no place in a one-line `--help` and every place in a
//! tool description. So it is a second field on the row, not a longer `help`:
//! two renderings for two readers, both in the register, and nothing in
//! `mcp.rs` authoring prose.
//!
//! # What this can and cannot hold
//!
//! It detects ABSENCE and PROVENANCE, never quality -- the same limit
//! `dependency_rationale.rs` states about itself. That the text is generated
//! from the row is mechanical and is asserted below. That the words are the
//! right words for a model to match on is REVIEW, and review is vc's.

use std::path::Path;
use std::process::Command;

/// **EVERY DRIVE HERE FIXTURES ITS HOME**, because this file chooses which verbs
/// to run from the dispatch table AT RUN TIME -- so a verb implemented later is
/// a verb it will drive, and `dispatch_ssot` once published this machine's
/// install pointer to a scratch worktree that was then deleted. The guard
/// `table_driven_tests_fixture_their_home` caught this file on its first run and
/// named the remedy; this is it.
fn tools(root: &Path) -> Vec<serde_json::Value> {
  let home = testkit::fixture_home();
  let (out, frames) = crate::common::mcp_session(
    root,
    Some(home),
    &[
      r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"drive","version":"0"}}}"#,
      r#"{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}"#,
    ],
  );
  assert!(
    out.status.success(),
    "the MCP session failed: {}",
    String::from_utf8_lossy(&out.stderr)
  );
  frames
    .iter()
    .find(|f| f["id"] == 2)
    .expect("a response to tools/list")["result"]["tools"]
    .as_array()
    .expect("a tool list")
    .clone()
}

fn estate() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(["init", "tool-description-fixture"])
    .current_dir(dir.path())
    .env("HOME", testkit::fixture_home())
    .stdin(testkit::lifeline_for(&["init"]))
    .output()
    .expect("run the v3 binary");
  assert!(out.status.success(), "fixture init failed");
  dir
}

/// **EVERY ROW THAT DECLARES ONE HAS IT IN THE PUBLISHED DESCRIPTION, VERBATIM**
/// -- which is what makes the register the author. The population is derived
/// from the table rather than listed here, so a row that gains the field is
/// covered on the day it gains it.
#[test]
fn the_declared_when_to_use_reaches_the_published_description() {
  let dir = estate();
  let published = tools(dir.path());
  let table = intent_cli::dispatch::table();

  let declaring: Vec<&intent_cli::dispatch::Entry> = table
    .families
    .iter()
    .flat_map(|f| f.entries.iter())
    .chain(table.new_surface.iter())
    .filter(|e| e.when_to_use.is_some())
    .collect();

  // **THE POPULATION CHECK, because a loop over an empty set passes.** The
  // search surface is what AC-24.2 names, so if nothing declares the field the
  // assertion below is vacuous and this says so instead.
  assert!(
    !declaring.is_empty(),
    "no row declares `when_to_use`, so the loop below asserts nothing"
  );
  assert!(
    declaring.iter().any(|e| e.path == "search"),
    "AC-24.2 is about the search tools: {:?}",
    declaring.iter().map(|e| &e.path).collect::<Vec<_>>()
  );

  let mut checked = 0usize;
  for entry in declaring {
    if !entry.exposed_on_mcp {
      continue;
    }
    let name = format!("intent_{}", entry.path.replace([' ', '-'], "_"));
    let Some(tool) = published.iter().find(|t| t["name"] == name) else {
      continue;
    };
    let description = tool["description"].as_str().expect("a description");
    let when = entry.when_to_use.as_deref().expect("declared above");
    assert!(
      description.contains(when),
      "`{name}`'s description does not carry the row's own words -- so something \
       other than the register is authoring it:\n  description: {description}\n  row: {when}"
    );
    checked += 1;
  }
  assert!(
    checked > 0,
    "no declaring row is exposed on MCP, so nothing was compared"
  );
}

/// **THE CONTROL: a row that declares none is unchanged.** Without it, a
/// renderer that appended the same text to every tool would pass the assertion
/// above, and the field would look load-bearing while meaning nothing.
#[test]
fn a_row_without_the_field_carries_no_when_to_use() {
  let dir = estate();
  let published = tools(dir.path());
  let table = intent_cli::dispatch::table();

  let bare = table
    .families
    .iter()
    .flat_map(|f| f.entries.iter())
    .chain(table.new_surface.iter())
    .find(|e| e.when_to_use.is_none() && e.exposed_on_mcp && e.path == "doctor")
    .expect("`doctor` is exposed and declares no when_to_use");
  let tool = published
    .iter()
    .find(|t| t["name"] == "intent_doctor")
    .expect("the doctor tool is published");
  let description = tool["description"].as_str().expect("a description");

  assert!(
    description.starts_with(bare.help.trim_end_matches('.')),
    "an undeclared row's description is still its help: {description:?}"
  );
  assert!(
    !description.contains("DO NOT USE IT"),
    "the when-and-when-not reached a row that never declared one: {description:?}"
  );
}
