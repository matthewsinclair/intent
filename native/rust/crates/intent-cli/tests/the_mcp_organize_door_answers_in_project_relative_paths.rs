//! Issue 0513: **the MCP `organize` door emits PROJECT-RELATIVE paths**, driven
//! over the real wire rather than against the closure that composes them.
//!
//! **THE DEFECT SURVIVED BECAUSE OF ITS NAME, and that is what decides where
//! this arm belongs.** The door's closure is called `rel` and its body did not
//! relativise, so every call site -- `rel(&report.hydrated)` and six more --
//! read as correct and disclosed nothing. A unit test over the closure would
//! have had to suspect the closure first, which is exactly what the name
//! prevented. So the question is asked of the PAYLOAD, where a consumer meets
//! it, and it is asked of every path-bearing field at once rather than of the
//! one field that happened to be noticed.
//!
//! **AND IT IS THE WIRE RATHER THAN `mcp::serve`.** `serve` takes a facade a
//! caller opens, so an in-process arm would build its own project and prove the
//! function relativises against a root the test chose. The class this file is
//! in -- guard on the wrong side of the wire -- is not closed by that: the
//! server has to still be calling it, with the project IT opened.

use std::io::Write;
use std::process::Stdio;

/// Every field of the `organize` answer that carries paths. Named exhaustively
/// rather than sampled, because the defect was one closure feeding ALL of them
/// and an arm reading `unclaimed` alone would pass a fix applied to one field.
const PATH_FIELDS: &[&str] = &[
  "hydrated",
  "rewritten",
  "unchanged",
  "dehydrated",
  "unclaimed",
  "diverged",
  "pruned",
];

#[test]
fn the_mcp_organize_door_answers_in_project_relative_paths() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  let run = |args: &[&str]| {
    let out = crate::common::intent()
      .args(args)
      .current_dir(root)
      .output()
      .unwrap_or_else(|e| panic!("run intent {args:?}: {e}"));
    assert!(
      out.status.success(),
      "intent {args:?} refused: {}",
      String::from_utf8_lossy(&out.stderr)
    );
    out
  };
  run(&["init", "Fixture"]);
  run(&["st", "new", "A thread to hang an unclaimed file on"]);

  // **A FILE NO RENDERER PRODUCES AND NO CANON CLAIMS**, which is organize's
  // fifth row: reported, never acted on. It is the cheapest population that
  // puts a real walked path into the answer.
  let thread = root.join("intent").join("st").join("ST0001");
  std::fs::create_dir_all(&thread).expect("thread dir");
  std::fs::write(thread.join("diagram.png"), b"not a render\n").expect("unclaimed file");

  let mut child = crate::common::intent()
    .arg("mcp")
    .current_dir(root)
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .expect("spawn intent mcp");
  {
    let stdin = child.stdin.as_mut().expect("stdin");
    for frame in [
      r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"drive","version":"0"}}}"#,
      r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
      r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"intent_organize","arguments":{}}}"#,
    ] {
      writeln!(stdin, "{frame}").expect("write frame");
    }
  }
  drop(child.stdin.take());
  let out = child.wait_with_output().expect("wait for the server");
  let stdout = String::from_utf8(out.stdout).expect("utf8 stdout");
  let frames: Vec<serde_json::Value> = stdout
    .lines()
    .map(|l| serde_json::from_str(l).expect("every output line is one JSON frame"))
    .collect();
  let call = frames
    .iter()
    .find(|f| f["id"].as_i64() == Some(2))
    .unwrap_or_else(|| panic!("no answer to the organize call: {stdout}"));
  assert_eq!(call["result"]["isError"], false, "organize: {stdout}");
  let answer: serde_json::Value =
    serde_json::from_str(call["result"]["content"][0]["text"].as_str().expect("text"))
      .expect("the content text is the tool's JSON answer");

  // **THE DENOMINATOR FIRST.** Every assertion below is satisfied vacuously by
  // an answer carrying no paths at all, and a fixture that stopped producing an
  // unclaimed file would do exactly that -- quietly, for a reason having
  // nothing to do with this door.
  let unclaimed = answer["unclaimed"]
    .as_array()
    .unwrap_or_else(|| panic!("no `unclaimed` array in the answer: {answer}"));
  assert!(
    !unclaimed.is_empty(),
    "the fixture must put at least one path in the answer, or nothing below is being measured: {answer}"
  );

  // **AND THE ROOT MUST BE ABSOLUTE**, or "no emitted path contains the root"
  // is true for a reason that has nothing to do with the door.
  assert!(
    root.is_absolute(),
    "the fixture root must be absolute for the containment check to mean anything: {root:?}"
  );
  let root_str = root.to_string_lossy().to_string();

  for field in PATH_FIELDS {
    let Some(values) = answer[*field].as_array() else {
      panic!("`{field}` is missing or not an array: {answer}");
    };
    for v in values {
      let p = v
        .as_str()
        .unwrap_or_else(|| panic!("`{field}` holds a non-string: {v}"));
      assert!(
        !p.contains(&root_str),
        "`{field}` carries the PROJECT ROOT: {p}\nA consumer receives the operator's home directory and cannot compare this against anything the CLI prints for the same run."
      );
      assert!(
        !p.starts_with('/'),
        "`{field}` carries an absolute path: {p}"
      );
    }
  }

  // **THE POSITIVE CONTROL ON THE CHECK ITSELF.** Everything above is a run of
  // NEGATIVE assertions, which a door answering empty strings would also pass.
  // This requires the relative form to still name the file.
  assert!(
    unclaimed
      .iter()
      .any(|v| v.as_str() == Some("intent/st/ST0001/diagram.png")),
    "the relative path must still IDENTIFY the file -- relativising bought by discarding the path is not relativising: {answer}"
  );
}
