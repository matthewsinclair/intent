//! ST0092 WP-06: a session is told its seat at pickup and at launch, and issue
//! 0675: a pickup says how old the reader's own last read is.
//!
//! Every arm drives the process surface on a scratch project under the
//! fixture HOME, the board declared through `intent wb init team --apply` from
//! the profiles in this repository. The seats, their names, roles and remits
//! are read from the shipped seat registry and profile, never typed here, so a
//! reworded profile cannot leave an arm asserting the old words.
//!
//! **A PICKUP HERE ALWAYS NAMES ITS SESSION.** An unnamed one defaults to the
//! running Claude Code session's id, and a suite run inside a session would
//! then record that session on a fixture board.

use std::path::Path;

use crate::common::level_three::INITIALIZE;
use intentsvcs::teams::{Seat, Teams};

/// The base profile every arm declares.
const TEAM: &str = "team";

fn run(cwd: &Path, args: &[&str]) -> (String, String, i32) {
  let out = crate::common::intent()
    .args(args)
    .current_dir(cwd)
    .env("HOME", testkit::fixture_home())
    .env_remove("CLAUDE_CODE_SESSION_ID")
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run intent");
  (
    String::from_utf8_lossy(&out.stdout).into_owned(),
    String::from_utf8_lossy(&out.stderr).into_owned(),
    out.status.code().unwrap_or(-1),
  )
}

fn ok(cwd: &Path, args: &[&str]) -> String {
  let (out, err, code) = run(cwd, args);
  assert_eq!(code, 0, "`intent {}`: {out}{err}", args.join(" "));
  out
}

fn json(cwd: &Path, args: &[&str]) -> serde_json::Value {
  let out = ok(cwd, args);
  serde_json::from_str(&out)
    .unwrap_or_else(|e| panic!("`{}` is not JSON: {e}: {out}", args.join(" ")))
}

/// A project with no profile declared.
fn project(name: &str) -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  ok(dir.path(), &["init", name]);
  dir
}

/// A project whose board declares the team profile, every seat registered by
/// the apply and none ever picked up.
fn team_project(name: &str) -> tempfile::TempDir {
  let dir = project(name);
  ok(dir.path(), &["wb", "init", TEAM, "--apply"]);
  dir
}

/// The team profile's seats as the shipped registry holds them.
fn team_seats() -> Vec<Seat> {
  let teams = Teams::new(&testkit::repo_root());
  let registry = teams.registry().expect("the shipped seat registry");
  let profile = teams.profile(TEAM).expect("the shipped team profile");
  profile
    .front
    .seats
    .iter()
    .map(|m| {
      registry
        .seats
        .iter()
        .find(|s| &s.moniker == m)
        .unwrap_or_else(|| panic!("the registry holds no seat `{m}`"))
        .clone()
    })
    .collect()
}

fn mediator() -> String {
  let teams = Teams::new(&testkit::repo_root());
  teams
    .profile(TEAM)
    .expect("the shipped team profile")
    .front
    .mediator
    .expect("a base names its mediator")
}

/// The seat the team profile declares as the human's.
fn human() -> Seat {
  team_seats()
    .into_iter()
    .find(|s| s.driver == intentsvcs::teams::Driver::Human)
    .expect("the team profile declares a human seat")
}

/// A session-driven seat that is not the mediator.
fn worker() -> Seat {
  let mediator = mediator();
  team_seats()
    .into_iter()
    .find(|s| s.driver != intentsvcs::teams::Driver::Human && s.moniker != mediator)
    .expect("the team profile declares a seat other than the mediator")
}

/// The body of the profile's `## ` section `heading`: every line after it up to
/// the next `## `, trimmed. Read here from the shipped file, so the arm holds
/// the launch to the profile's words.
fn section(heading: &str) -> String {
  let brief = Teams::new(&testkit::repo_root())
    .profile(TEAM)
    .expect("the shipped team profile")
    .brief;
  let body: Vec<&str> = brief
    .lines()
    .skip_while(|l| l.trim_end() != heading)
    .skip(1)
    .take_while(|l| !l.starts_with("## "))
    .collect();
  let body = body.join("\n").trim().to_string();
  assert!(
    !body.is_empty(),
    "the team profile has no section `{heading}`"
  );
  body
}

fn mcp_tool(root: &Path, tool: &str, arguments: serde_json::Value) -> serde_json::Value {
  let call = serde_json::json!({
    "jsonrpc": "2.0",
    "id": 2,
    "method": "tools/call",
    "params": { "name": tool, "arguments": arguments },
  })
  .to_string();
  let (out, frames) =
    crate::common::mcp_session(root, Some(testkit::fixture_home()), &[INITIALIZE, &call]);
  assert!(
    out.status.success(),
    "{}",
    String::from_utf8_lossy(&out.stderr)
  );
  let answer = frames
    .iter()
    .find(|f| f["id"] == 2)
    .unwrap_or_else(|| panic!("no answer to {tool} in {frames:?}"));
  assert_ne!(
    answer["result"]["isError"], true,
    "{tool} refused: {answer:#}"
  );
  let text = answer["result"]["content"][0]["text"]
    .as_str()
    .unwrap_or_else(|| panic!("{tool} answers text content: {answer:#}"));
  serde_json::from_str(text).unwrap_or_else(|e| panic!("{tool}'s text is JSON: {e}: {text}"))
}

fn status_row<'a>(rows: &'a serde_json::Value, moniker: &str) -> &'a serde_json::Value {
  rows
    .as_array()
    .expect("an array of rows")
    .iter()
    .find(|r| r["moniker"] == moniker || r["node"]["moniker"] == moniker)
    .unwrap_or_else(|| panic!("no row for {moniker}: {rows}"))
}

/// ST0092 AT-06.1
///
/// ST0092 AC-06.1 and the INV-09 arm: a pickup on a declared board prints the
/// seat's brief AHEAD of the board, and its `--json`, MCP's pickup and
/// `wb brief --json` carry the same brief, built by one library function. A
/// worker's brief names the mediator and its route; the mediator's says it is
/// the mediator and reaches hv directly; every other seat is listed with its
/// name, role and remit.
#[test]
fn a_pickup_on_a_declared_board_carries_the_seat_brief_on_four_faces() {
  let dir = team_project("briefpickup");
  let root = dir.path();
  let me = worker();
  let mediator = mediator();

  let cli = json(
    root,
    &[
      "wb",
      "pickup",
      "--node",
      &me.moniker,
      "--session",
      "s-worker",
      "--json",
    ],
  );
  let brief = &cli["brief"];
  assert_eq!(brief["seat"]["moniker"], me.moniker.as_str(), "{cli:#}");
  assert_eq!(brief["seat"]["name"], me.name.as_str());
  assert_eq!(brief["seat"]["role"], me.role.as_str());
  assert_eq!(brief["seat"]["remit"], me.remit.as_str());
  assert_eq!(brief["team"]["base"], TEAM);
  assert_eq!(brief["team"]["mediator"]["moniker"], mediator.as_str());
  assert_eq!(brief["team"]["is_mediator"], false);
  let others: Vec<&str> = brief["team"]["seats"]
    .as_array()
    .expect("the other seats")
    .iter()
    .map(|s| s["moniker"].as_str().expect("a moniker"))
    .collect();
  let expected: Vec<String> = team_seats()
    .into_iter()
    .filter(|s| s.moniker != me.moniker)
    .map(|s| s.moniker)
    .collect();
  assert_eq!(others, expected, "the other seats, in the profile's order");
  for seat in brief["team"]["seats"].as_array().expect("seats") {
    for field in ["name", "role", "remit"] {
      assert!(
        seat[field].as_str().is_some_and(|v| !v.is_empty()),
        "a seat line lacks its {field}: {seat}"
      );
    }
  }
  let route = section(intentsvcs::teams::SECTION_ROUTE_SEATS);
  assert!(
    brief["team"]["prose"]
      .as_str()
      .is_some_and(|p| p.contains(&route)),
    "a worker's brief does not carry its route to hv: {brief:#}"
  );

  let mcp = mcp_tool(
    root,
    "intent_wb_pickup",
    serde_json::json!({ "node": me.moniker, "session": "s-worker" }),
  );
  assert_eq!(&mcp["brief"], brief, "MCP's pickup carries the CLI's brief");
  let shown = json(root, &["wb", "brief", &me.moniker, "--json"]);
  assert_eq!(&shown, brief, "`wb brief --json` is the pickup's brief");

  // The prose: the brief is printed ahead of the board, and it is `wb brief`'s
  // own text.
  let text = ok(
    root,
    &[
      "wb",
      "pickup",
      "--node",
      &me.moniker,
      "--session",
      "s-worker",
    ],
  );
  let brief_text = ok(root, &["wb", "brief", &me.moniker]);
  let at = text.find(brief_text.trim_end()).unwrap_or_else(|| {
    panic!("the pickup does not print `wb brief`'s text:\n{text}\n---\n{brief_text}")
  });
  let board = text
    .find("peers (")
    .unwrap_or_else(|| panic!("the pickup printed no peers:\n{text}"));
  assert!(at < board, "the brief is not ahead of the board:\n{text}");

  let own = json(
    root,
    &[
      "wb",
      "pickup",
      "--node",
      &mediator,
      "--session",
      "s-mediator",
      "--json",
    ],
  );
  assert_eq!(own["brief"]["team"]["is_mediator"], true, "{own:#}");
  let route = section(intentsvcs::teams::SECTION_ROUTE_MEDIATOR);
  assert!(
    own["brief"]["team"]["prose"]
      .as_str()
      .is_some_and(|p| p.contains(&route)),
    "the mediator's brief does not carry the mediator's route: {own:#}"
  );
}

/// ST0092 AT-06.2
///
/// ST0092 AC-06.2: a seat no session has picked up is marked never started in
/// `wb pickup` and `wb status`, from the absence of a recorded session; its
/// registration is not printed as a heartbeat; the human seat carries no mark;
/// and the mark goes once the seat is picked up.
#[test]
fn a_seat_no_session_has_picked_up_is_marked_never_started() {
  let dir = team_project("briefnever");
  let root = dir.path();
  let me = worker();
  let human = human();
  let other = team_seats()
    .into_iter()
    .find(|s| s.driver != intentsvcs::teams::Driver::Human && s.moniker != me.moniker)
    .expect("a second session seat");

  let rows = json(root, &["wb", "status", "--json"]);
  assert_eq!(
    status_row(&rows, &other.moniker)["never_started"],
    true,
    "{rows:#}"
  );
  assert_eq!(
    status_row(&rows, &human.moniker)["never_started"],
    false,
    "{rows:#}"
  );
  let text = ok(root, &["wb", "status"]);
  let line = text
    .lines()
    .find(|l| l.starts_with(&format!("{} ", other.moniker)))
    .unwrap_or_else(|| panic!("no status line for {}:\n{text}", other.moniker));
  assert!(
    line.contains("never started") && !line.contains("heartbeat"),
    "a never-started seat's line: {line}"
  );

  let up = json(
    root,
    &[
      "wb",
      "pickup",
      "--node",
      &me.moniker,
      "--session",
      "s-worker",
      "--json",
    ],
  );
  let seat = |m: &str| {
    up["brief"]["team"]["seats"]
      .as_array()
      .expect("seats")
      .iter()
      .find(|s| s["moniker"] == m)
      .cloned()
      .unwrap_or_else(|| panic!("no brief line for {m}: {up:#}"))
  };
  assert_eq!(seat(&other.moniker)["never_started"], true);
  assert_eq!(seat(&human.moniker)["never_started"], false);

  ok(
    root,
    &[
      "wb",
      "pickup",
      "--node",
      &other.moniker,
      "--session",
      "s-other",
    ],
  );
  let rows = json(root, &["wb", "status", "--json"]);
  assert_eq!(
    status_row(&rows, &other.moniker)["never_started"],
    false,
    "{rows:#}"
  );
  assert_eq!(
    status_row(&rows, &me.moniker)["never_started"],
    false,
    "{rows:#}"
  );
}

/// ST0092 AT-06.5
///
/// ST0092 AC-06.5: `wb brief <moniker>` prints the seat's brief, and with
/// `--launch` its launch steps from the profile, and writes nothing: no
/// heartbeat moves, no session is recorded, no file changes. Its edges: the
/// human seat's line prints and `--launch` on it refuses at exit 1; an unknown
/// moniker refuses at exit 1 naming the registered seats.
#[test]
fn wb_brief_prints_a_seat_and_its_launch_steps_and_writes_nothing() {
  let dir = team_project("briefread");
  let root = dir.path();
  let me = worker();
  let mediator = mediator();
  let human = human();
  let snapshot = || {
    let mut files = Vec::new();
    for entry in walkdir(&root.join("intent")) {
      if entry.components().any(|c| c.as_os_str() == ".cache") {
        continue;
      }
      files.push((entry.clone(), std::fs::read(&entry).expect("read a file")));
    }
    files.sort();
    files
  };
  let shown = |m: &str| json(root, &["wb", "show", m, "--json"]);
  let before = (snapshot(), shown(&me.moniker));

  let text = ok(root, &["wb", "brief", &me.moniker]);
  assert!(
    text.contains(&me.name) && text.contains(&me.remit),
    "the brief does not name the seat and its remit:\n{text}"
  );
  let launch = ok(root, &["wb", "brief", &me.moniker, "--launch"]);
  assert!(
    launch.contains(&section(intentsvcs::teams::SECTION_LAUNCH_SEATS)),
    "a worker's launch steps are not the profile's:\n{launch}"
  );
  let launch = ok(root, &["wb", "brief", &mediator, "--launch"]);
  assert!(
    launch.contains(&section(intentsvcs::teams::SECTION_LAUNCH_MEDIATOR)),
    "the mediator's launch steps are not the profile's:\n{launch}"
  );
  assert_eq!(
    (snapshot(), shown(&me.moniker)),
    before,
    "`wb brief` wrote something"
  );

  let (out, err, code) = run(root, &["wb", "brief", &human.moniker]);
  assert_eq!(code, 0, "the human seat's line: {out}{err}");
  assert!(out.contains(&human.name), "{out}");
  let (out, err, code) = run(root, &["wb", "brief", &human.moniker, "--launch"]);
  assert_eq!(code, 1, "--launch on the human seat: {out}{err}");
  assert!(err.contains("error:") && err.contains("remedy:"), "{err}");

  let (out, err, code) = run(root, &["wb", "brief", "zz"]);
  assert_eq!(code, 1, "an unknown moniker: {out}{err}");
  assert!(
    err.contains("error:") && err.contains(&me.moniker) && err.contains(&mediator),
    "the refusal does not name the registered seats: {err}"
  );
}

/// The edge that keeps an unprofiled project's launch as it was: on a board
/// that declares no profile, `wb brief` answers at exit 0 with the node's name
/// and role and today's two launch steps, and never refuses.
#[test]
fn wb_brief_on_an_unprofiled_board_answers_with_todays_steps() {
  let dir = project("briefplain");
  let root = dir.path();
  ok(
    root,
    &[
      "wb",
      "register",
      "cc",
      "--name",
      "Control Claude",
      "--role",
      "control",
    ],
  );
  let text = ok(root, &["wb", "brief", "cc"]);
  assert!(
    text.contains("Control Claude") && text.contains("control"),
    "{text}"
  );
  let launch = ok(root, &["wb", "brief", "cc", "--launch"]);
  assert!(
    launch.contains("show me a detailed plan for today's work")
      && launch.contains("STOP and wait for my instructions"),
    "an unprofiled launch lost today's two steps:\n{launch}"
  );
}

/// Issue 0675: a pickup prints how long ago the acting node last picked up on
/// this machine, read before it writes its own, and a store with no pickup of
/// the node, as a fresh clone's, says so on a `note:` at exit 0 and prints no
/// age. `wb status` carries each session seat's last pickup, and each peer's
/// line in a pickup and each status row count live messages, labelled live.
#[test]
fn a_pickup_says_how_old_the_readers_own_last_read_is() {
  let dir = team_project("briefage");
  let root = dir.path();
  let me = worker();
  let mediator = mediator();

  let first = ok(
    root,
    &[
      "wb",
      "pickup",
      "--node",
      &me.moniker,
      "--session",
      "s-worker",
    ],
  );
  assert!(
    first
      .lines()
      .any(|l| l.starts_with("note: ") && l.contains("no pickup")),
    "a first pickup does not say none is recorded:\n{first}"
  );
  let up = json(
    root,
    &[
      "wb",
      "pickup",
      "--node",
      &me.moniker,
      "--session",
      "s-worker",
      "--json",
    ],
  );
  assert!(
    up["last_pickup"].as_str().is_some_and(|s| s.ends_with('Z')),
    "a second pickup does not carry the first one's stamp: {up:#}"
  );
  let second = ok(
    root,
    &[
      "wb",
      "pickup",
      "--node",
      &me.moniker,
      "--session",
      "s-worker",
    ],
  );
  assert!(
    second
      .lines()
      .any(|l| l.starts_with("last pickup: ") && l.contains(" ago")),
    "a later pickup does not print the age of the last one:\n{second}"
  );

  ok(
    root,
    &[
      "wb",
      "ask",
      &mediator,
      "a question for the mediator",
      "--node",
      &me.moniker,
    ],
  );
  let up = json(
    root,
    &[
      "wb",
      "pickup",
      "--node",
      &me.moniker,
      "--session",
      "s-worker",
      "--json",
    ],
  );
  let peer = up["peers"]
    .as_array()
    .expect("peers")
    .iter()
    .find(|p| p["moniker"] == mediator.as_str())
    .cloned()
    .unwrap_or_else(|| panic!("no peer line for the mediator: {up:#}"));
  assert_eq!(peer["live_messages"], 1, "{peer:#}");
  let text = ok(
    root,
    &[
      "wb",
      "pickup",
      "--node",
      &me.moniker,
      "--session",
      "s-worker",
    ],
  );
  assert!(
    text
      .lines()
      .any(|l| l.contains(&format!("  {mediator} ")) && l.contains("1 live message")),
    "the mediator's peer line does not count its live message:\n{text}"
  );

  let rows = json(root, &["wb", "status", "--json"]);
  let row = status_row(&rows, &me.moniker);
  assert!(
    row["last_pickup"]
      .as_str()
      .is_some_and(|s| s.ends_with('Z')),
    "{row:#}"
  );
  assert_eq!(status_row(&rows, &mediator)["live_messages"], 1, "{rows:#}");
  let mcp = mcp_tool(root, "intent_wb_status", serde_json::json!({}));
  let row = status_row(&mcp, &mediator);
  assert_eq!(row["live_messages"], 1, "{mcp:#}");
  assert!(row.get("live_items").is_some(), "{mcp:#}");
  assert!(
    row.get("messages").is_none() && row.get("items").is_none(),
    "MCP's status still answers the old names: {mcp:#}"
  );

  // A fresh clone: the store ingested from canon holds no machine-scoped
  // pickup, so the line says none is recorded and prints no age.
  std::fs::remove_file(root.join("intent/.cache/intent.db")).expect("remove the store");
  let fresh = ok(
    root,
    &[
      "wb",
      "pickup",
      "--node",
      &me.moniker,
      "--session",
      "s-worker",
    ],
  );
  assert!(
    fresh
      .lines()
      .any(|l| l.starts_with("note: ") && l.contains("no pickup")),
    "a fresh clone's pickup does not say none is recorded:\n{fresh}"
  );
  assert!(
    !fresh.lines().any(|l| l.starts_with("last pickup: ")),
    "a fresh clone printed an age:\n{fresh}"
  );
}

/// Every file under `dir`, recursively.
fn walkdir(dir: &Path) -> Vec<std::path::PathBuf> {
  let mut out = Vec::new();
  let mut stack = vec![dir.to_path_buf()];
  while let Some(d) = stack.pop() {
    for entry in std::fs::read_dir(&d).expect("read a directory") {
      let path = entry.expect("an entry").path();
      if path.is_dir() {
        stack.push(path);
      } else {
        out.push(path);
      }
    }
  }
  out
}

/// ST0092 WP-06's retired-seat edge (WP-06's info.md, and vc's ruling of
/// 2026-11-11 on the seats a brief lists): a retired seat's own brief refuses
/// on exit 1, naming the retirement and its reason with `wb register` as the
/// remedy, with and without `--launch`; and a retired seat is left out of
/// every other seat's brief, in `wb brief` and in the pickup, as `wb status`
/// leaves it out without `--all`. The mediator stays listed, so the omission is
/// of the retired seat and not of the list.
#[test]
fn a_retired_seat_is_refused_its_brief_and_left_out_of_the_others() {
  let dir = team_project("briefretired");
  let root = dir.path();
  let me = worker();
  let mediator = mediator();
  let human = human();
  let gone = team_seats()
    .into_iter()
    .find(|s| s.moniker != me.moniker && s.moniker != mediator && s.moniker != human.moniker)
    .expect("a third session seat to retire");
  let reason = "its remit moved to another seat";
  ok(root, &["wb", "retire", &gone.moniker, "--reason", reason]);

  for launch in [false, true] {
    let mut args = vec!["wb", "brief", gone.moniker.as_str()];
    if launch {
      args.push("--launch");
    }
    let (out, err, code) = run(root, &args);
    assert_eq!(code, 1, "`intent {}`: {out}{err}", args.join(" "));
    assert!(out.is_empty(), "a refused brief printed something: {out}");
    assert!(
      err.contains("retired") && err.contains(reason) && err.contains("intent wb register"),
      "the refusal does not name the retirement and `wb register`: {err}"
    );
  }

  let listed = |brief: &serde_json::Value| -> Vec<String> {
    brief["team"]["seats"]
      .as_array()
      .expect("seats")
      .iter()
      .filter_map(|s| s["moniker"].as_str().map(str::to_string))
      .collect()
  };
  let brief = json(root, &["wb", "brief", &me.moniker, "--json"]);
  let up = json(
    root,
    &[
      "wb",
      "pickup",
      "--node",
      &me.moniker,
      "--session",
      "s-worker",
      "--json",
    ],
  );
  for (face, seats) in [
    ("wb brief", listed(&brief)),
    ("wb pickup", listed(&up["brief"])),
  ] {
    assert!(
      !seats.contains(&gone.moniker),
      "{face} lists the retired seat {}: {seats:?}",
      gone.moniker
    );
    assert!(
      seats.contains(&mediator),
      "{face} lost the mediator too, so the list itself is gone: {seats:?}"
    );
  }
  let text = ok(root, &["wb", "brief", &me.moniker]);
  assert!(
    !text.contains(&format!("    {} (", gone.moniker)),
    "the brief's text lists the retired seat:\n{text}"
  );
}
