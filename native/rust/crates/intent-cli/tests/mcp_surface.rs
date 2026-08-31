//! **`AT-09.1` / `AT-00.4`: the PUBLISHED tool surface is the GENERATED one,
//! and the escape hatch is on it.** One file, two rows, because both ask the
//! same question of the wire.
//!
//! # What the unit tests already prove, and the hole they leave
//!
//! `mcp.rs`'s own tests prove GENERATION: `tools(&table)` derives one tool per
//! `exposed_on_mcp && facade` row, its schema from the row's authored `args`
//! and `flags` (never from a Rust type -- D37), its description in D45's
//! projection order, and `SERVED` agrees with that population both ways. Every
//! one of those drives `tools(&table)` directly.
//!
//! **None of them proves PUBLICATION.** A generator can be perfect and the
//! server can list something else -- a floor of `> 40` tools, a stale roster
//! compiled in, a name transformed on the way out -- and every unit test stays
//! green because none of them speaks over the wire. That is
//! guard-on-the-wrong-side-of-the-wire (cc's class): an arm driving the
//! composing function proves the sentence and proves nothing about whether the
//! server still calls it. `mcp_stdio_serves.rs` drives a real session but
//! asserts only `tools.len() > 40`, which a wrong roster of the right size
//! passes. **This file closes the wire.**
//!
//! # The two anchors, chosen so neither side certifies itself
//!
//! Publication-equals-generation is an equality, and an equality between two
//! things this file computes proves nothing if both drift together -- the
//! agreement-is-not-derivation trap. So each side is anchored to a DIFFERENT
//! authority the estate already holds:
//!
//! - **The published NAME SET is anchored to the `SERVED` const**, the declared
//!   roster of 60. A tool published that is not in `SERVED`, or a `SERVED` path
//!   the wire does not publish, fails -- so the surface is checked against a
//!   DECLARATION, not against itself. (`SERVED` agreeing with `tools(&table)`
//!   is the unit test's job; this file does not repeat it.)
//! - **Each published tool's SCHEMA and DESCRIPTION are anchored to
//!   `tools(&table)`**, the generator's own output. So the wire's CONTENT is
//!   the generated content byte for byte, not merely the right names -- which
//!   is what carries every unit-tested property (schema-from-table,
//!   D45 projection, no tracker id) onto the surface without restating them.
//!
//! Names from the declaration, content from the generator: the two halves of
//! "the server publishes exactly what the table generates" resting on two
//! authorities rather than on each other.
//!
//! # `AT-00.4`'s escape-hatch clause
//!
//! `AC-00.4` says MCP ships the tiered typed tools PLUS the `intent_graphql`
//! escape hatch, bridging to intentd when it is up. This file witnesses the
//! FIRST half at the surface -- the tier is published and `intent_graphql` is
//! among it. The BRIDGING half is `AT-09.2`'s, driven end to end against a
//! real daemon in `graphql_escape_hatch.rs`; it is cited, not re-driven, so
//! the daemon proof lives in one place.
//!
//! # No daemon, and no project needed for the claim
//!
//! `tools/list` is answered from the compiled-in table, so it is a claim about
//! the BUILD and not about any project or daemon. A project is initialised only
//! because the server is driven as a real host would drive it; nothing here
//! calls a tool.

mod common;

use std::collections::BTreeSet;

use common::{mcp_session, short_dir};
use intent_cli::dispatch;
use intent_cli::mcp::{self, SERVED, Tool};

/// The generator's output, or the build defect that stops it.
fn generated() -> Vec<Tool> {
  mcp::tools(&dispatch::table()).unwrap_or_else(|u| {
    panic!(
      "the committed table refused row `{}`: {} -- a build defect, and there is no surface to test until it is fixed",
      u.path, u.why
    )
  })
}

/// `SERVED`'s declared paths as the tool names the wire must publish.
/// `st list` -> `intent_st_list`; the one legal separator and the space both
/// become `_`, exactly as [`mcp::tool_name`] does it -- but spelled here from
/// the DECLARATION rather than imported, so this anchor cannot drift with the
/// function it checks.
fn served_names() -> BTreeSet<String> {
  SERVED
    .iter()
    .map(|p| format!("intent_{}", p.replace([' ', '-'], "_")))
    .collect()
}

/// Drive a real `intent mcp` session to `tools/list` and return the published
/// tool frames, keyed by name.
fn published_tools() -> std::collections::BTreeMap<String, serde_json::Value> {
  let root = short_dir("mcp-surface");
  intentsvcs::init::init(&root, "Surface", "test", env!("CARGO_PKG_VERSION"))
    .expect("the shipped initialiser creates a project");

  let (out, frames) = mcp_session(
    &root,
    None,
    &[
      r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"surface","version":"0"}}}"#,
      r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
      r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#,
    ],
  );
  let _ = std::fs::remove_dir_all(&root);
  assert!(
    out.status.success(),
    "the server did not exit cleanly: {:?}\nstderr: {}",
    out.status.code(),
    String::from_utf8_lossy(&out.stderr)
  );

  let list = frames
    .iter()
    .find(|f| f["id"] == serde_json::json!(2))
    .expect("a response to the tools/list request");
  let tools = list["result"]["tools"]
    .as_array()
    .expect("tools/list result carries a tools array");

  tools
    .iter()
    .map(|t| {
      (
        t["name"]
          .as_str()
          .expect("every published tool has a name")
          .to_string(),
        t.clone(),
      )
    })
    .collect()
}

/// **THE WIRE PUBLISHES EXACTLY THE DECLARED ROSTER, BOTH DIRECTIONS.**
#[test]
fn the_published_names_are_the_served_roster() {
  let published: BTreeSet<String> = published_tools().into_keys().collect();
  let served = served_names();

  let unpublished: Vec<&String> = served.difference(&published).collect();
  assert!(
    unpublished.is_empty(),
    "these `SERVED` paths are declared and the server did NOT publish them over `tools/list`: {unpublished:?}"
  );
  let unbacked: Vec<&String> = published.difference(&served).collect();
  assert!(
    unbacked.is_empty(),
    "the server published these tools that are NOT in `SERVED` -- an agent is offered a tool the declaration does not vouch for: {unbacked:?}"
  );
}

/// **THE PUBLISHED CONTENT IS THE GENERATED CONTENT, NOT MERELY THE RIGHT
/// NAMES.** Schema and description byte for byte, so every property the unit
/// tests prove of `tools(&table)` -- schema-from-table, the D45 projection, no
/// tracker id -- reaches the wire without being restated here.
#[test]
fn each_published_tool_carries_its_generated_schema_and_description() {
  let published = published_tools();
  let generated = generated();
  assert!(!generated.is_empty(), "the generator produced no tools");

  for tool in &generated {
    let frame = published.get(&tool.name).unwrap_or_else(|| {
      panic!(
        "the generator names `{}` and the wire did not publish it",
        tool.name
      )
    });
    assert_eq!(
      frame["description"].as_str(),
      Some(tool.description.as_str()),
      "`{}`: the published description is not the generated one -- the wire is composing its own",
      tool.name
    );
    assert_eq!(
      frame["inputSchema"], tool.input_schema,
      "`{}`: the published inputSchema is not the generated one, so a client is typed against something other than the authored args/flags",
      tool.name
    );
  }
}

/// **`AT-00.4`: the escape hatch is on the surface.** The bridging behaviour is
/// `AT-09.2`'s, driven against a real daemon in `graphql_escape_hatch.rs`; this
/// asserts only that `intent_graphql` is published among the tier, which is the
/// half `AC-00.4` states about the SURFACE rather than about a running daemon.
#[test]
fn the_graphql_escape_hatch_is_published_among_the_tier() {
  let published = published_tools();
  assert!(
    published.contains_key("intent_graphql"),
    "`intent_graphql` is not in the published tool list -- `AC-00.4`'s escape-hatch clause is unmet at the surface. Published: {:?}",
    published.keys().collect::<Vec<_>>()
  );
  // And it is one tool among a real tier, not the only thing served -- the
  // `> 1` guards against a degenerate surface that published the hatch alone.
  assert!(
    published.len() > 1,
    "the hatch is published but the tier is not: only {} tool(s) on the wire",
    published.len()
  );
}
