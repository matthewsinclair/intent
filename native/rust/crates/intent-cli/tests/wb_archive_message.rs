//! **Issue 0627: a handled message never left its inbox view, and no verb
//! retired one.** The inbox view is the file a human reads, and `wb clear`
//! marks a message handled in the recipient's own board without touching it.
//! `intent wb archive message` is the sender's retire: one message by its
//! address once its recipient has handled it, or every such message under
//! `--handled`. Each arm drives the CLI as a node would, and each is red on a
//! binary without the form, which refuses `message` as an archive kind.

use std::path::Path;

/// stdout, stderr and the exit code of one `intent` run in `cwd`.
fn run(cwd: &Path, args: &[&str]) -> (String, String, i32) {
  let out = crate::common::intent()
    .args(args)
    .current_dir(cwd)
    .env("HOME", testkit::fixture_home())
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run intent");
  (
    String::from_utf8_lossy(&out.stdout).into_owned(),
    String::from_utf8_lossy(&out.stderr).into_owned(),
    out.status.code().unwrap_or(-1),
  )
}

/// Run a step a test stands on, which must succeed.
fn ok(cwd: &Path, args: &[&str]) -> String {
  let (out, err, code) = run(cwd, args);
  assert_eq!(code, 0, "{args:?}: {out}{err}");
  out
}

/// A project in its own git repository with `cc`, `vc` and `ic` registered.
fn roster() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let git = std::process::Command::new("git")
    .args(["init", "-q"])
    .current_dir(dir.path())
    .status()
    .expect("run git init");
  assert!(git.success(), "git init");
  ok(dir.path(), &["init", "wbarchive"]);
  for (node, name, role) in [
    ("cc", "Control", "control"),
    ("vc", "Validation", "validation"),
    ("ic", "Interface", "interface"),
  ] {
    ok(
      dir.path(),
      &["wb", "register", node, "--name", name, "--role", role],
    );
  }
  dir
}

/// The inbox view `sender` writes for `recipient`.
fn view(dir: &Path, recipient: &str, sender: &str) -> String {
  std::fs::read_to_string(dir.join(format!("intent/whiteboard/{recipient}/inbox.{sender}.md")))
    .expect("the inbox view")
}

/// The address of the first message `sender` sent `recipient`, read off the
/// inbox view where a reader finds it; `#1` names the first of its minute.
fn first_address(dir: &Path, recipient: &str, sender: &str) -> String {
  let view = view(dir, recipient, sender);
  let stamp = view
    .lines()
    .find_map(|line| line.strip_prefix("## ("))
    .and_then(|rest| rest.split(')').next())
    .expect("the view heads a message with its stamp");
  format!("{stamp}#1")
}

/// **AN ARCHIVED MESSAGE LEAVES THE VIEW AND STAYS IN THE RECORD.** The view
/// drops it, `wb show --all` still lists it marked `(archived)`, and the verb
/// names the three files it wrote, all the sender's to commit.
#[test]
fn an_archived_handled_message_leaves_the_inbox_view() {
  let dir = roster();
  let d = dir.path();
  ok(
    d,
    &[
      "wb",
      "ask",
      "vc",
      "the kestrel returns its first argument",
      "--node",
      "cc",
    ],
  );
  ok(d, &["wb", "clear", "cc", "--node", "vc"]);
  assert!(
    view(d, "vc", "cc").contains("the kestrel"),
    "the control: a handled message is still in the view before the archive"
  );

  let address = first_address(d, "vc", "cc");
  let (out, err, code) = run(
    d,
    &[
      "wb", "archive", "message", &address, "--to", "vc", "--node", "cc",
    ],
  );
  assert_eq!(code, 0, "{out}{err}");
  assert!(
    out.contains(&format!("ok: cc archived message {address} for vc")),
    "{out}"
  );
  for path in [
    "intent/whiteboard/cc/board.json",
    "intent/whiteboard/vc/inbox.cc.md",
  ] {
    assert!(out.contains(&format!("wrote: {path}")), "{path}: {out}");
  }
  assert!(
    out
      .lines()
      .any(|l| l.starts_with("wrote: intent/.canon/events/")),
    "the verb's own event: {out}"
  );
  assert!(
    !out.contains("re-rendered:"),
    "everything is the sender's: {out}"
  );

  let after = view(d, "vc", "cc");
  assert!(!after.contains("the kestrel"), "{after}");
  assert!(after.contains("_(empty)_"), "{after}");
  let shown = ok(d, &["wb", "show", "vc", "--all"]);
  assert!(
    shown.contains("cc -> vc (handled) (archived) the kestrel"),
    "{shown}"
  );
}

/// **ONLY THE RECIPIENT SAYS A MESSAGE IS HANDLED**, so archiving one it still
/// holds live is refused, with the recipient's `wb clear` as the remedy, and
/// writes nothing.
#[test]
fn a_message_its_recipient_holds_live_is_refused() {
  let dir = roster();
  let d = dir.path();
  ok(d, &["wb", "ask", "vc", "not read yet", "--node", "cc"]);
  let board = d.join("intent/whiteboard/cc/board.json");
  let before = std::fs::read(&board).expect("cc's board");

  let address = first_address(d, "vc", "cc");
  let (out, err, code) = run(
    d,
    &[
      "wb", "archive", "message", &address, "--to", "vc", "--node", "cc",
    ],
  );
  assert_eq!(code, 1, "{out}{err}");
  assert!(
    err.contains("`vc` has not handled the message from `cc`"),
    "{err}"
  );
  assert!(err.contains("intent wb clear cc --node vc"), "{err}");
  assert!(!out.contains("wrote:"), "a refusal writes nothing: {out}");
  assert_eq!(
    std::fs::read(&board).expect("cc's board"),
    before,
    "the board is unchanged"
  );
  assert!(view(d, "vc", "cc").contains("not read yet"));
}

/// **AN ANNOUNCE IS ARCHIVED ONE RECIPIENT AT A TIME**: each copy is its own
/// row, and the mark that gates it is that recipient's. `vc` handled its copy
/// and `ic` did not, so `vc`'s leaves and `ic`'s stays and is refused.
#[test]
fn an_announce_is_archived_one_recipient_at_a_time() {
  let dir = roster();
  let d = dir.path();
  ok(d, &["wb", "announce", "the box is free", "--node", "cc"]);
  ok(d, &["wb", "clear", "cc", "--node", "vc"]);

  let to_vc = first_address(d, "vc", "cc");
  ok(
    d,
    &[
      "wb", "archive", "message", &to_vc, "--to", "vc", "--node", "cc",
    ],
  );
  assert!(!view(d, "vc", "cc").contains("the box is free"));
  assert!(
    view(d, "ic", "cc").contains("the box is free"),
    "ic's copy stays: ic has not handled it"
  );

  let to_ic = first_address(d, "ic", "cc");
  let (_, err, code) = run(
    d,
    &[
      "wb", "archive", "message", &to_ic, "--to", "ic", "--node", "cc",
    ],
  );
  assert_eq!(code, 1, "{err}");
  assert!(
    err.contains("`ic` has not handled the message from `cc`"),
    "{err}"
  );
}

/// **THE SWEEP ARCHIVES WHAT IS HANDLED AND NAMES WHAT STAYS**, and a repeat
/// moves nothing and writes nothing: the form a fold runs.
#[test]
fn the_sweep_archives_every_handled_message_and_names_what_stays_live() {
  let dir = roster();
  let d = dir.path();
  let for_vc = ["first for vc", "second for vc"];
  for body in for_vc {
    ok(d, &["wb", "ask", "vc", body, "--node", "cc"]);
  }
  ok(d, &["wb", "ask", "ic", "one for ic", "--node", "cc"]);
  ok(d, &["wb", "clear", "cc", "--node", "vc"]);

  let out = ok(
    d,
    &["wb", "archive", "message", "--handled", "--node", "cc"],
  );
  assert!(
    out.contains(&format!(
      "ok: cc archived {} message(s) its recipients have handled",
      for_vc.len()
    )),
    "{out}"
  );
  assert_eq!(
    out.matches("  archived: vc ").count(),
    for_vc.len(),
    "{out}"
  );
  assert!(out.contains("  still live with ic: "), "{out}");
  assert!(view(d, "vc", "cc").contains("_(empty)_"));
  assert!(view(d, "ic", "cc").contains("one for ic"));

  let again = ok(
    d,
    &["wb", "archive", "message", "--handled", "--node", "cc"],
  );
  assert!(
    again.contains("ok: cc has no handled message to archive"),
    "{again}"
  );
  assert!(
    !again.contains("wrote:"),
    "a repeat writes nothing: {again}"
  );
}

/// **THE STAGED-BEFORE-THE-EDIT WARNING NAMES THE SENDER'S BOARD**, which holds
/// the body since 0612. It checked each recipient's board until 0627's bank, so
/// a staged sender board holding the old text went unnamed.
#[test]
fn an_edit_of_a_message_names_the_senders_staged_board() {
  let dir = roster();
  let d = dir.path();
  ok(d, &["wb", "ask", "vc", "a body to correct", "--node", "cc"]);
  let staged = std::process::Command::new("git")
    .args(["add", "intent/whiteboard/cc/board.json"])
    .current_dir(d)
    .status()
    .expect("run git add");
  assert!(staged.success(), "stage the sender's board");

  let address = first_address(d, "vc", "cc");
  let (out, err, code) = run(
    d,
    &[
      "wb",
      "edit",
      "message",
      &address,
      "the corrected body",
      "--to",
      "vc",
      "--node",
      "cc",
    ],
  );
  assert_eq!(code, 0, "{out}{err}");
  assert!(
    err.contains("warning: these were staged before the edit"),
    "{err}"
  );
  assert!(err.contains("  intent/whiteboard/cc/board.json"), "{err}");
}

/// One `intent_wb_archive` call over MCP in `dir`: the tool's own answer, the
/// JSON document inside the frame's text, and the whole frame, which carries a
/// refusal as well as an answer.
fn mcp_archive(dir: &Path, args: &serde_json::Value) -> (serde_json::Value, String) {
  let frames = [
    r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"drive","version":"0"}}}"#.to_string(),
    r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#.to_string(),
    serde_json::json!({
      "jsonrpc": "2.0", "id": 2, "method": "tools/call",
      "params": { "name": "intent_wb_archive", "arguments": args },
    })
    .to_string(),
  ];
  let mut child = crate::common::intent()
    .arg("mcp")
    .current_dir(dir)
    .env("HOME", testkit::fixture_home())
    .stdin(std::process::Stdio::piped())
    .stdout(std::process::Stdio::piped())
    .stderr(std::process::Stdio::piped())
    .spawn()
    .expect("spawn intent mcp");
  {
    use std::io::Write as _;
    let stdin = child.stdin.as_mut().expect("stdin");
    for frame in &frames {
      writeln!(stdin, "{frame}").expect("write frame");
    }
  }
  drop(child.stdin.take());
  let out = child.wait_with_output().expect("wait");
  let frame: serde_json::Value = String::from_utf8_lossy(&out.stdout)
    .lines()
    .filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok())
    .find(|v| v.get("id").and_then(|i| i.as_u64()) == Some(2))
    .expect("an answer to the tool call");
  let text = frame
    .pointer("/result/content/0/text")
    .and_then(|t| t.as_str())
    .unwrap_or_default()
    .to_string();
  let payload = serde_json::from_str::<serde_json::Value>(&text).unwrap_or(serde_json::Value::Null);
  (payload, frame.to_string())
}

/// **ISSUE 0631: THE ADDRESS IS `id` OVER MCP, AS `wb edit`'s IS.** A message
/// is archived by `id` and `to`, and the old key `seq` meets the refusal every
/// unknown key meets: no alias is kept. Red at a base whose table names the
/// positional `seq`, where `id` is the unknown key.
#[test]
fn mcp_archives_a_message_by_id_and_refuses_seq() {
  let dir = roster();
  let d = dir.path();
  ok(
    d,
    &[
      "wb",
      "ask",
      "vc",
      "the kestrel returns its first argument",
      "--node",
      "cc",
    ],
  );
  ok(d, &["wb", "clear", "cc", "--node", "vc"]);
  let address = first_address(d, "vc", "cc");

  let (_, refused) = mcp_archive(
    d,
    &serde_json::json!({ "node": "cc", "kind": "message", "seq": address, "to": "vc" }),
  );
  assert!(refused.contains("no parameter named `seq`"), "{refused}");
  assert!(
    view(d, "vc", "cc").contains("the kestrel"),
    "a refusal archives nothing"
  );

  let (answer, frame) = mcp_archive(
    d,
    &serde_json::json!({ "node": "cc", "kind": "message", "id": address, "to": "vc" }),
  );
  assert_eq!(answer["moved"], serde_json::json!(true), "{frame}");
  assert!(!view(d, "vc", "cc").contains("the kestrel"), "{frame}");
}

/// **AN ITEM IS ARCHIVED BY `id` OVER MCP TOO**, its number within its kind.
/// Red at the base for the same reason as the message arm.
#[test]
fn mcp_archives_an_item_by_id() {
  let dir = roster();
  let d = dir.path();
  ok(
    d,
    &["wb", "add", "todo", "the item to retire", "--node", "cc"],
  );

  let (answer, frame) = mcp_archive(
    d,
    &serde_json::json!({ "node": "cc", "kind": "todo", "id": "1" }),
  );
  assert_eq!(answer["moved"], serde_json::json!(true), "{frame}");
  let shown = ok(d, &["wb", "show", "cc"]);
  assert!(!shown.contains("the item to retire"), "{shown}");
}
