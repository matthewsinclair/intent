//! `intent mcp` -- the stdio MCP server over the tool tier (WP-09).
//!
//! An MCP client speaks newline-delimited JSON-RPC over this process's stdio.
//! `tools/list` answers from [`crate::mcp::tools`] and `tools/call` routes
//! through the per-call facade door to [`crate::mcp::serve`] -- this module is
//! TRANSPORT, and owns no tool schema, no arm, and no facade logic. The
//! client's config entry is `{"command": "intent", "args": ["mcp"]}`: nothing
//! in it moves.
//!
//! # Zero dependencies, and that is a measured choice, not thrift
//!
//! design.md:166 ratified rmcp for "MCP server (stdio + streamable HTTP)".
//! Read at mode granularity, its two proof points split: conflabd proves rmcp
//! for streamable HTTP -- the 3.x multi-agent tier -- while the OTHER pattern
//! the same section cites, Lamplight's `mcp.rs`, is a depless hand loop over
//! exactly the stdio mode that ships now (339 lines, serde_json only, rmcp
//! nowhere in its manifest). The 3.0.0 scope is tools-only stdio: initialize,
//! `notifications/initialized`, `tools/list`, `tools/call`, `ping`. rmcp buys
//! protocol breadth this scope does not use, at the price of tokio and a
//! large subtree entering a lock four nodes build against, and async entering
//! a crate whose facade contract is synchronous open-per-call. The loop goes
//! when the MCP face routes through `dispatch(op)`/intentd -- the same 3.x
//! destination `mcp.rs`'s header already records -- which is where rmcp
//! enters at the tier that earns it.
//!
//! ratified_in: "vc, 2026-08-31, under hv's pen granted 2026-08-22; ruled in
//! ic's channel on ic's fork of 2026-08-30". The deciding fact was the
//! measurement, not the cost argument: the design's OWN cited proof point for
//! the stdio mode is the depless loop, so this reads the grounds at the
//! granularity the design section already uses. rmcp STAYS ratified for
//! streamable HTTP in 3.x -- this defers the mode, it does not retire the
//! row. Discharge: the loop goes when the MCP face routes through
//! `dispatch(op)`/intentd.
//!
//! # Framing is split from calling, and the split is what the tests buy
//!
//! [`serve_frames`] takes any reader/writer pair plus a `call` closure
//! (Lamplight's testable shape): the protocol tests drive real frames with a
//! stub closure and never touch the ambient project, while [`run`] passes the
//! real one -- open a fresh facade through `render.rs`'s one door, serve, and
//! render every refusal through its own remedy. The integration drive spawns
//! the real binary in a fixture project.
//!
//! # Protocol posture
//!
//! - **A notification is answered with silence, decided BY THE ABSENCE OF AN
//!   ID and never by what came back** -- emitting a response to one is a
//!   protocol violation, not noise.
//! - **A request always produces exactly one line**: its result or a typed
//!   error. There is no third arm, which is what makes never-hang structural.
//! - **`initialize` echoes the client's `protocolVersion`.** The tools-only
//!   surface is identical across every published protocol revision, so the
//!   echo is the maximally compatible true answer; when the client names
//!   none, the latest revision this module was written against is offered.
//! - **Tool-execution failures travel as `isError` content, protocol
//!   failures as JSON-RPC errors.** An agent can read `isError` content and
//!   self-correct -- the remedy text exists for exactly that reader -- while
//!   an unknown method or unknown tool is a caller defect the protocol layer
//!   owns.
//! - **Every response line is flushed immediately**: an MCP host reads the
//!   stream incrementally and a buffered reply is an apparent hang.

use std::io::{BufRead, Write};

use serde_json::{Value, json};

use crate::mcp::Tool;
use crate::spine::Failure;

/// JSON-RPC "Parse error" -- stdin carried something that is not JSON.
const JSONRPC_PARSE_ERROR: i64 = -32700;
/// JSON-RPC "Method not found".
const JSONRPC_METHOD_NOT_FOUND: i64 = -32601;
/// JSON-RPC "Invalid params" -- the spec's code for an unknown tool name.
const JSONRPC_INVALID_PARAMS: i64 = -32602;

/// The protocol revision offered when the client names none.
const FALLBACK_PROTOCOL_VERSION: &str = "2025-06-18";

/// Serve until stdin closes. The host going away is the normal end of life,
/// exit 0 -- which is why the `not_probed` exemption on this row is written in
/// the built tense: bare `intent mcp` serves until its host closes it.
pub fn run() -> Result<(), Failure> {
  let tools = crate::mcp::tools(&crate::dispatch::table()).map_err(|u| {
    Failure::Error(format!(
      "error: the tool table refused `{}`: {}\n  remedy: this is a build defect -- the committed dispatch table and the generator disagree",
      u.path, u.why
    ))
  })?;
  let stdin = std::io::stdin();
  let mut stdout = std::io::stdout();
  serve_frames(&mut stdin.lock(), &mut stdout, &tools, &mut |tool, args| {
    // **OPEN PER CALL, THROUGH THE ONE DOOR.** A fresh facade per request is
    // the contract `mcp.rs`'s header states: an MCP host keeps this process
    // alive for a whole client session, so a facade opened at spawn would
    // serve every later call from the store as it stood at the first one.
    let opened = crate::render::context().and_then(|(project, ctx)| {
      crate::render::engine(project, ctx.clone(), crate::render::StoreNeed::Shared)
        .map(|facade| (facade, ctx))
    });
    match opened {
      Err(e) => Answered::Refused(e.message().unwrap_or("refused").to_string()),
      Ok((mut facade, ctx)) => match crate::mcp::serve(&mut facade, &ctx, &tool.path, args) {
        Ok(value) => Answered::Value(value),
        Err(e) => Answered::Refused(e.render()),
      },
    }
  })
}

/// What one tool call produced, as the wire will carry it: the tool's JSON
/// answer, or the operator-facing refusal that travels back as `isError`
/// content. **A refusal here is an ANSWER for the agent, not a failure of the
/// loop** -- the remedy text exists for exactly that reader -- which is why
/// this is a value and not an error type (IN-RS-CODE-004's third arm: errors
/// explicitly modelled as data, because the loop treats both variants
/// identically and nothing ever propagates one).
pub enum Answered {
  Value(Value),
  Refused(String),
}

/// The loop over any reader/writer pair. `run` is the stdio wiring plus the
/// real facade closure; this is the behaviour the tests drive with real
/// frames. `call` answers one found tool's invocation.
pub fn serve_frames(
  input: &mut impl BufRead,
  output: &mut impl Write,
  tools: &[Tool],
  call: &mut dyn FnMut(&Tool, &Value) -> Answered,
) -> Result<(), Failure> {
  for line in input.lines() {
    let line =
      line.map_err(|e| Failure::Error(format!("error: could not read from stdin: {e}")))?;
    if let Some(response) = answer_line(&line, tools, call) {
      write_frame(output, &response)?;
    }
  }
  Ok(())
}

/// One inbound line -> at most one outbound frame.
fn answer_line(
  line: &str,
  tools: &[Tool],
  call: &mut dyn FnMut(&Tool, &Value) -> Answered,
) -> Option<Value> {
  let trimmed = line.trim();
  if trimmed.is_empty() {
    return None;
  }
  match serde_json::from_str::<Value>(trimmed) {
    Err(source) => Some(error_frame(
      &Value::Null,
      JSONRPC_PARSE_ERROR,
      &format!("this line is not JSON-RPC: {source}"),
    )),
    Ok(message) => answer_message(&message, tools, call),
  }
}

fn answer_message(
  message: &Value,
  tools: &[Tool],
  call: &mut dyn FnMut(&Tool, &Value) -> Answered,
) -> Option<Value> {
  let id = message.get("id").cloned();
  let method = message.get("method").and_then(Value::as_str).unwrap_or("");
  let params = message.get("params").cloned().unwrap_or(Value::Null);

  // A message with no id is a notification: acted on where one means
  // something, answered with silence ALWAYS.
  let Some(id) = id else {
    return None;
  };

  let answered = match method {
    "initialize" => Ok(initialize_result(&params)),
    "ping" => Ok(json!({})),
    "tools/list" => Ok(json!({ "tools": tools.iter().map(tool_frame).collect::<Vec<_>>() })),
    "tools/call" => tool_call(&params, tools, call),
    other => Err((
      JSONRPC_METHOD_NOT_FOUND,
      format!("no method named `{other}` on this server"),
    )),
  };
  Some(match answered {
    Ok(result) => json!({ "jsonrpc": "2.0", "id": id, "result": result }),
    Err((code, text)) => error_frame(&id, code, &text),
  })
}

/// The handshake. Capabilities name exactly what is served: tools, no
/// list-changed notifications (the population is the committed table's, so it
/// cannot change within a process's lifetime).
fn initialize_result(params: &Value) -> Value {
  let version = params
    .get("protocolVersion")
    .and_then(Value::as_str)
    .unwrap_or(FALLBACK_PROTOCOL_VERSION);
  json!({
    "protocolVersion": version,
    "capabilities": { "tools": {} },
    "serverInfo": { "name": "intent", "version": env!("CARGO_PKG_VERSION") },
  })
}

/// One tool as the wire advertises it.
fn tool_frame(tool: &Tool) -> Value {
  json!({
    "name": tool.name,
    "description": tool.description,
    "inputSchema": tool.input_schema,
  })
}

/// `tools/call`: find the tool, run the closure, wrap the answer.
///
/// The split of failure channels is deliberate and stated in the module doc:
/// an unknown NAME is the protocol layer's refusal (`-32602`), while a found
/// tool's refusal -- bad arguments, a facade refusal, an unopenable project --
/// is `isError` CONTENT, because the agent reading it is the one who can act
/// on the remedy.
fn tool_call(
  params: &Value,
  tools: &[Tool],
  call: &mut dyn FnMut(&Tool, &Value) -> Answered,
) -> Result<Value, (i64, String)> {
  let name = params.get("name").and_then(Value::as_str).unwrap_or("");
  let Some(tool) = tools.iter().find(|t| t.name == name) else {
    return Err((
      JSONRPC_INVALID_PARAMS,
      format!(
        "no tool named `{name}` -- `tools/list` names the {} this server serves",
        tools.len()
      ),
    ));
  };
  let arguments = params.get("arguments").cloned().unwrap_or(Value::Null);
  Ok(match call(tool, &arguments) {
    Answered::Value(value) => json!({
      "content": [ { "type": "text", "text": pretty(&value) } ],
      "isError": false,
    }),
    Answered::Refused(text) => json!({
      "content": [ { "type": "text", "text": text } ],
      "isError": true,
    }),
  })
}

/// Pretty output for the agent-facing text block. `Value`'s own Display
/// cannot fail, so there is no branch that could emit nothing.
fn pretty(value: &Value) -> String {
  format!("{value:#}")
}

fn error_frame(id: &Value, code: i64, message: &str) -> Value {
  json!({
    "jsonrpc": "2.0",
    "id": id,
    "error": { "code": code, "message": message },
  })
}

/// One frame per line, flushed immediately.
fn write_frame(out: &mut impl Write, frame: &Value) -> Result<(), Failure> {
  writeln!(out, "{frame}")
    .map_err(|e| Failure::Error(format!("error: could not write to stdout: {e}")))?;
  out
    .flush()
    .map_err(|e| Failure::Error(format!("error: could not flush stdout: {e}")))
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::io::Cursor;

  fn tools() -> Vec<Tool> {
    crate::mcp::tools(&crate::dispatch::table()).expect("the committed table generates")
  }

  /// Drive real frames through the real loop with a stub caller; one output
  /// Value per response line.
  fn drive(frames: &[&str], call: &mut dyn FnMut(&Tool, &Value) -> Answered) -> Vec<Value> {
    let input = frames.join("\n");
    let mut out: Vec<u8> = Vec::new();
    let all = tools();
    serve_frames(&mut Cursor::new(input), &mut out, &all, call).expect("the loop completes");
    String::from_utf8(out)
      .expect("utf8 output")
      .lines()
      .map(|l| serde_json::from_str(l).expect("every output line is JSON"))
      .collect()
  }

  fn never(_: &Tool, _: &Value) -> Answered {
    panic!("this frame must not reach the caller");
  }

  #[test]
  fn initialize_echoes_the_clients_protocol_version_and_names_the_server() {
    let out = drive(
      &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{}}}"#,
      ],
      &mut never,
    );
    assert_eq!(out.len(), 1);
    assert_eq!(out[0]["result"]["protocolVersion"], "2024-11-05");
    assert_eq!(out[0]["result"]["serverInfo"]["name"], "intent");
    assert!(out[0]["result"]["capabilities"]["tools"].is_object());
  }

  #[test]
  fn a_notification_is_answered_with_silence_and_a_blank_line_too() {
    let out = drive(
      &[
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        "",
        "   ",
      ],
      &mut never,
    );
    assert!(out.is_empty(), "a notification produced output: {out:?}");
  }

  #[test]
  fn tools_list_advertises_the_whole_generated_population() {
    let out = drive(
      &[r#"{"jsonrpc":"2.0","id":7,"method":"tools/list"}"#],
      &mut never,
    );
    let listed = out[0]["result"]["tools"].as_array().expect("tools array");
    assert_eq!(listed.len(), tools().len());
    for t in listed {
      assert!(t["name"].as_str().unwrap_or("").starts_with("intent_"));
      assert!(t["inputSchema"]["properties"].is_object());
      assert!(!t["description"].as_str().unwrap_or("").is_empty());
    }
  }

  #[test]
  fn a_call_routes_to_the_named_tool_and_wraps_its_answer_as_content() {
    let mut seen = Vec::new();
    let out = drive(
      &[
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"intent_st_list","arguments":{"status":"all"}}}"#,
      ],
      &mut |tool, args| {
        seen.push((tool.path.clone(), args.clone()));
        Answered::Value(serde_json::json!({ "threads": [] }))
      },
    );
    assert_eq!(
      seen,
      vec![("st list".to_string(), serde_json::json!({"status":"all"}))]
    );
    assert_eq!(out[0]["result"]["isError"], false);
    let text = out[0]["result"]["content"][0]["text"]
      .as_str()
      .expect("text");
    assert!(text.contains("\"threads\""));
  }

  #[test]
  fn a_tool_refusal_is_iserror_content_the_agent_can_read() {
    let out = drive(
      &[
        r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"intent_st_new","arguments":{}}}"#,
      ],
      &mut |_, _| Answered::Refused("error: `title` is required".to_string()),
    );
    assert_eq!(out[0]["result"]["isError"], true);
    assert!(
      out[0]["result"]["content"][0]["text"]
        .as_str()
        .expect("text")
        .contains("`title` is required")
    );
  }

  #[test]
  fn an_unknown_tool_is_a_protocol_error_not_content() {
    let out = drive(
      &[r#"{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"intent_no_such"}}"#],
      &mut never,
    );
    assert_eq!(out[0]["error"]["code"], JSONRPC_INVALID_PARAMS);
  }

  #[test]
  fn an_unknown_method_and_a_parse_error_answer_with_their_codes() {
    let out = drive(
      &[
        r#"{"jsonrpc":"2.0","id":5,"method":"resources/list"}"#,
        r#"this is not json"#,
      ],
      &mut never,
    );
    assert_eq!(out[0]["error"]["code"], JSONRPC_METHOD_NOT_FOUND);
    assert_eq!(out[1]["error"]["code"], JSONRPC_PARSE_ERROR);
    assert_eq!(out[1]["id"], Value::Null);
  }

  /// A request always produces exactly one line -- the whole conversation, in
  /// order, one response per id-bearing frame.
  #[test]
  fn one_request_one_line_in_order() {
    let out = drive(
      &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"ping"}"#,
        r#"{"jsonrpc":"2.0","id":3,"method":"tools/list"}"#,
      ],
      &mut never,
    );
    let ids: Vec<i64> = out.iter().map(|v| v["id"].as_i64().expect("id")).collect();
    assert_eq!(ids, vec![1, 2, 3]);
  }
}
