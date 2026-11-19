//! AT-07.8 / AC-07.6, the intent-cli half: **the doors this crate owns refuse
//! another project's address by name** (hv's ruling 4, 2026-09-15; issue 0338).
//!
//! The facade's doors are driven in `intentsvcs/tests/address_empty_authority.rs`.
//! These are the doors that read an address in THIS crate before any facade door
//! sees it: MCP `resources/read`, and `browse` with its twin `edit --browser`.
//! Driven as built, both answered from this project. `resources/read` on
//! `intent://other/threads/ST0001` returned this project's ST0001 byte for byte,
//! and `browse` resolved the id here and went on to look for a daemon. The
//! terminal `edit <address>` and `set <address>` are driven here too, end to
//! end, because a refusal the facade gets right can still reach the operator in
//! someone else's words.

use std::path::{Path, PathBuf};

use crate::common::{mcp_session, short_dir};

/// Another project's address for the thread id `st`.
fn foreign_thread(st: &str) -> String {
  format!("intent://elsewhere/threads/{st}")
}

/// A `resources/read` frame for `uri`, with the JSON-RPC id `id`.
fn resources_read(id: u32, uri: &str) -> String {
  format!(r#"{{"jsonrpc":"2.0","id":{id},"method":"resources/read","params":{{"uri":"{uri}"}}}}"#)
}

/// A project carrying one thread and one issue, and their ids, so every foreign
/// address below names something this project DOES hold: a door that resolved
/// it here would succeed, which is what makes a refusal mean something.
fn estate() -> (PathBuf, String, String) {
  let root = short_dir("cross-project");
  std::fs::create_dir_all(&root).expect("the estate directory");
  intentsvcs::init::init(&root, "Crossing", "test", env!("CARGO_PKG_VERSION"))
    .expect("a fresh project initialises");
  let project = intentsvcs::project::Project::open(&root).expect("the project opens");
  let ctx = intentsvcs::facade::FacadeContext::for_tests(
    "test".to_string(),
    String::new(),
    env!("CARGO_PKG_VERSION").to_string(),
  );
  let mut facade = intentsvcs::facade::Facade::open(project, ctx).expect("the facade opens");
  let st = facade
    .st_new("A thread this project carries")
    .expect("a thread is created");
  let issue = facade
    .issue_add(
      "An issue this project carries",
      Some("medium"),
      Some("cc"),
      "issue body prose",
    )
    .expect("an issue is created");
  (root, st, intentsvcs::model::issue_id(issue))
}

/// `intent <args>` in `root` under an isolated HOME: its stderr and exit code.
fn cli(root: &Path, home: &Path, args: &[&str]) -> (String, i32) {
  let out = crate::common::intent()
    .args(args)
    .current_dir(root)
    .env("HOME", home)
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("the intent binary runs");
  (
    String::from_utf8_lossy(&out.stderr).to_string(),
    out.status.code().unwrap_or(-1),
  )
}

fn names_the_address_and_the_project(said: &str, url: &str) -> bool {
  said.contains(&format!("`{url}`")) && said.contains("`elsewhere`")
}

#[test]
fn resources_read_refuses_another_projects_address_by_name() {
  let (root, st, issue) = estate();
  let foreign = foreign_thread(&st);
  let foreign_issue = format!("intent://elsewhere/issues/{issue}");
  let frames = [
    r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"cross-project","version":"0"}}}"#.to_string(),
    resources_read(2, &format!("intent:///threads/{st}")),
    resources_read(3, &foreign),
    resources_read(4, &foreign_issue),
  ];
  let sent: Vec<&str> = frames.iter().map(String::as_str).collect();
  let (out, parsed) = mcp_session(&root, None, &sent);
  assert!(
    out.status.success(),
    "the server did not exit cleanly: {}",
    String::from_utf8_lossy(&out.stderr)
  );
  let response = |id: i64| {
    parsed
      .iter()
      .find(|f| f["id"] == serde_json::json!(id))
      .unwrap_or_else(|| panic!("no response with id {id}: {parsed:?}"))
  };

  // The control: this project's own address reads, so a refusal below is about
  // the authority and not about the entity.
  assert!(
    response(2).get("result").is_some(),
    "this project's own {st} did not read: {}",
    response(2)
  );

  for (id, uri) in [(3, foreign.as_str()), (4, foreign_issue.as_str())] {
    let answer = response(id);
    let message = answer["error"]["message"].as_str().unwrap_or_else(|| {
      panic!(
        "reading {uri} ANSWERED rather than refused, so another project's address was served from this one: {answer}"
      )
    });
    assert!(
      names_the_address_and_the_project(message, uri),
      "the refusal for {uri} does not name the address and the project: {message}"
    );
  }
  let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn browse_and_edit_browser_refuse_another_projects_address_before_resolving_it() {
  let (root, st, _) = estate();
  let home = short_dir("cross-project-home");
  std::fs::create_dir_all(&home).expect("an isolated home");
  let foreign = foreign_thread(&st);
  for args in [
    vec!["browse", "st", foreign.as_str()],
    vec!["edit", "st", foreign.as_str(), "--browser"],
  ] {
    let (said, code) = cli(&root, &home, &args);
    let spelled = args.join(" ");
    assert_ne!(code, 0, "`intent {spelled}` succeeded: {said}");
    assert!(
      names_the_address_and_the_project(&said, &foreign),
      "`intent {spelled}` did not refuse the address by name: {said}"
    );
    assert!(
      !said.contains("no `intentd` is answering"),
      "`intent {spelled}` resolved the id in this project and went looking for a daemon: {said}"
    );
  }
  let _ = std::fs::remove_dir_all(&root);
  let _ = std::fs::remove_dir_all(&home);
}

#[test]
fn the_terminal_edit_and_set_doors_refuse_another_projects_address_by_name() {
  let (root, st, _) = estate();
  let home = short_dir("cross-project-home");
  std::fs::create_dir_all(&home).expect("an isolated home");
  let foreign = foreign_thread(&st);
  let absent = "intent://elsewhere/threads/ST0009";
  for (args, url) in [
    (vec!["edit", absent, "design", "--path"], absent),
    (
      vec![
        "set",
        foreign.as_str(),
        "title",
        "Renamed from another project",
      ],
      foreign.as_str(),
    ),
  ] {
    let (said, code) = cli(&root, &home, &args);
    let spelled = args.join(" ");
    assert_ne!(code, 0, "`intent {spelled}` succeeded: {said}");
    assert!(
      names_the_address_and_the_project(&said, url),
      "`intent {spelled}` did not refuse the address by name: {said}"
    );
    assert!(
      said.contains("intent:///"),
      "the remedy for `intent {spelled}` does not spell this project's own address: {said}"
    );
  }
  let _ = std::fs::remove_dir_all(&root);
  let _ = std::fs::remove_dir_all(&home);
}
