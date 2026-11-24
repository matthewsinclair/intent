//! `intent mcp` end to end: the real binary serves real frames over real
//! stdio against a real project, and the host closing stdin is the normal
//! end of life at exit 0 -- the property the row's `not_probed` exemption
//! describes, driven instead of asserted.

#[test]
fn the_server_answers_a_whole_session_and_exits_cleanly_when_the_host_leaves() {
  let dir = tempfile::tempdir().expect("tempdir");
  let init = crate::common::intent()
    .arg("init")
    .arg("Fixture")
    .current_dir(dir.path())
    .output()
    .expect("run intent init");
  assert!(
    init.status.success(),
    "init refused: {}",
    String::from_utf8_lossy(&init.stderr)
  );

  // The helper closes stdin after the last frame: EOF is how an MCP host says
  // goodbye.
  let (out, frames) = crate::common::mcp_session(
    dir.path(),
    None,
    &[
      r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"drive","version":"0"}}}"#,
      r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
      r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#,
      r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"intent_st_list","arguments":{"status":"all"}}}"#,
      r#"{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"intent_doctor","arguments":{}}}"#,
    ],
  );
  assert!(
    out.status.success(),
    "the host closing stdin must be exit 0, got {:?}\nstderr: {}",
    out.status.code(),
    String::from_utf8_lossy(&out.stderr)
  );
  let stdout = String::from_utf8_lossy(&out.stdout);
  // Exactly one response per id-bearing request, in order; silence for the
  // notification.
  assert_eq!(
    frames
      .iter()
      .map(|f| f["id"].as_i64().unwrap())
      .collect::<Vec<_>>(),
    vec![1, 2, 3, 4],
    "response ids: {stdout}"
  );

  assert_eq!(frames[0]["result"]["protocolVersion"], "2025-06-18");
  assert_eq!(frames[0]["result"]["serverInfo"]["name"], "intent");

  // The listing names each tool this session goes on to call.
  let tools: Vec<&str> = frames[1]["result"]["tools"]
    .as_array()
    .expect("tools")
    .iter()
    .map(|t| t["name"].as_str().expect("a tool's name"))
    .collect();
  assert!(
    tools.contains(&"intent_st_list") && tools.contains(&"intent_doctor"),
    "tools/list: {tools:?}"
  );

  // st list on a fresh project: a well-formed empty listing, not an error.
  assert_eq!(frames[2]["result"]["isError"], false, "st list: {stdout}");
  let listing: serde_json::Value = serde_json::from_str(
    frames[2]["result"]["content"][0]["text"]
      .as_str()
      .expect("text"),
  )
  .expect("the content text is the tool's JSON answer");
  assert!(listing["threads"].is_array());

  assert_eq!(frames[3]["result"]["isError"], false, "doctor: {stdout}");
  let report: serde_json::Value = serde_json::from_str(
    frames[3]["result"]["content"][0]["text"]
      .as_str()
      .expect("text"),
  )
  .expect("doctor's JSON answer");
  assert!(report.get("healthy").is_some());
}

/// The refusal channels, driven through the same real session: a tool
/// refusing bad arguments answers as `isError` CONTENT (the agent's channel),
/// an unknown tool as a protocol error -- and the session survives both.
#[test]
fn refusals_travel_on_their_declared_channels_and_never_kill_the_session() {
  let dir = tempfile::tempdir().expect("tempdir");
  let init = crate::common::intent()
    .arg("init")
    .arg("Fixture")
    .current_dir(dir.path())
    .output()
    .expect("run intent init");
  assert!(init.status.success());

  let (out, frames) = crate::common::mcp_session(
    dir.path(),
    None,
    &[
      // `title` missing: the serve tier's Args refusal.
      r#"{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"intent_st_new","arguments":{}}}"#,
      // A narrowed parameter: refused BY NAME, not accepted-and-ignored.
      r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"intent_st_list","arguments":{"width":"80"}}}"#,
      // Not a tool at all.
      r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"intent_info"}}"#,
      // Still alive afterwards.
      r#"{"jsonrpc":"2.0","id":4,"method":"ping"}"#,
    ],
  );
  assert!(out.status.success());

  assert_eq!(frames[0]["result"]["isError"], true);
  assert!(
    frames[0]["result"]["content"][0]["text"]
      .as_str()
      .expect("text")
      .contains("`title` is required")
  );
  assert_eq!(frames[1]["result"]["isError"], true);
  assert!(
    frames[1]["result"]["content"][0]["text"]
      .as_str()
      .expect("text")
      .contains("no parameter named `width`")
  );
  assert_eq!(frames[2]["error"]["code"], -32602);
  assert!(
    frames[3]["result"].is_object(),
    "ping after refusals: {frames:?}"
  );
}
