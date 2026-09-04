//! `AT-17.6` / `AC-17.6`: **`intent edit <kind> <id>` and `intent browse <kind>
//! <id>` reach ONE MODEL through ONE SERVICE.**
//!
//! # What "one model, one service" has to mean to be testable
//!
//! It is not *the two verbs look similar*. `AC-17.1` settled the shape of this
//! kind of claim on this thread and the rule is **diff the model, never compare
//! two renderings for similarity** -- so the assertion below takes the rows the
//! BROWSER receives over HTTP and the rows the TERMINAL computes IN PROCESS,
//! for the same entity, and requires them to be the same bytes. One derivation
//! reached two ways, rather than two derivations that agree today.
//!
//! **THAT IS WHY `Op::Form` EXISTS AT ALL, AND WHY IT IS NOT A GraphQL FIELD.**
//! `Op::Graphql` has no in-process twin -- its own doc comment says so, which
//! is why it is `DAEMON_ONLY` and absent from `SERVED_BY_DAEMON` -- and a door
//! with ONE path cannot carry a claim about two paths agreeing. The identity
//! asserted here would have been unassertable through the escape hatch.
//!
//! # INV-09 is the other half, and it was holding by an ABSENCE
//!
//! `browse` and `edit --browser` are `INV-09` twins: every spelling of one
//! capability agrees about whether it exists. **Until 2026-09-04 they agreed by
//! BOTH REFUSING**, which is a real way for that invariant to hold and a
//! fragile one -- wiring either alone would have broken it by NEITHER holding,
//! and it would have read green throughout. They are wired from one function
//! for that reason, and the tests below drive both spellings through the same
//! inputs rather than trusting that they share a callee.
//!
//! # What this row does NOT claim
//!
//! **Nothing here opens a browser.** The last step is handing a URL to `open`
//! or `xdg-open`, which is the desktop's business and not this daemon's; a test
//! that spawned a browser would be measuring the operator's default handler.
//! The claim stops at *the right URL is composed and the page it names is
//! served*, and the two halves either side of that are asserted separately.
//!
//! **Work packages are NOT covered, and that is the row's open half rather than
//! an omission here.** `nav.rs` refuses to produce a `View` for `Entity::Wp` in
//! two separate functions, because `/wp/ST0056/17` parses as a CHILDREN view
//! under the positional grammar -- so the ratified path contract cannot express
//! one. `AC-17.6` requires ST, WP and ISSUE, so the row does not close on this
//! file alone; the refusal is asserted below so the gap is visible rather than
//! silent.

use std::path::{Path, PathBuf};

use crate::common::{RealDaemon, ask_op, get, http, published, short_dir};

/// A project with one thread and one issue in it, at a path short enough for a
/// unix socket -- `SUN_LEN` is 104 bytes and a temp dir under a long scratch
/// path exceeds it, which the daemon refuses by name rather than truncating.
fn an_estate() -> PathBuf {
  let root = short_dir("browse-estate");
  std::fs::create_dir_all(&root).expect("the estate directory");
  intentsvcs::init::init(&root, "Browse", "test", env!("CARGO_PKG_VERSION"))
    .expect("a fresh project initialises");

  let project = intentsvcs::project::Project::open(&root).expect("the project opens");
  let ctx = intentsvcs::facade::FacadeContext {
    principal: "test".to_string(),
    project_id: String::new(),
    version: env!("CARGO_PKG_VERSION").to_string(),
  };
  let mut facade = intentsvcs::facade::Facade::open(project, ctx).expect("the facade opens");
  facade
    .st_new("A thread to open in a browser")
    .expect("a thread is created");
  root
}

/// The rows the TERMINAL would draw, computed in this process.
fn in_process(root: &Path, id: &str) -> Vec<intentsvcs::form::Triple> {
  let project = intentsvcs::project::Project::open(root).expect("the project opens");
  let ctx = intentsvcs::facade::FacadeContext {
    principal: "test".to_string(),
    project_id: String::new(),
    version: env!("CARGO_PKG_VERSION").to_string(),
  };
  let facade = intentsvcs::facade::Facade::open(project, ctx).expect("the facade opens");
  let declaration = intentsvcs::form::Loaded::load().expect("the form declaration loads");
  let form = declaration.form("thread").expect("`thread` is declared");
  let entity = facade
    .entity_json(&intentsvcs::address::Entity::Thread { id: id.to_string() })
    .expect("the thread resolves");
  intentsvcs::form::triples(form, &entity)
}

// ---------------------------------------------------------------------------
// The criterion itself.
// ---------------------------------------------------------------------------

/// **THE ROWS A BROWSER GETS AND THE ROWS THE TERMINAL COMPUTES ARE THE SAME
/// BYTES.** This is `AC-17.6` in one assertion.
#[test]
fn the_browser_and_the_terminal_receive_one_derivation() {
  let root = an_estate();
  let daemon = RealDaemon::start();
  daemon.wait_until_it_answers_a_real_op();

  let (status, body) = ask_op(
    &daemon,
    &format!(
      r#"{{"root":"{}","op":"form","view":"/thread/ST0001"}}"#,
      root.display()
    ),
  );
  assert!(
    status.contains("200"),
    "the form op answered {status}: {body}"
  );

  let answer: serde_json::Value = serde_json::from_str(&body).expect("a JSON answer");
  assert!(
    answer.get("error").is_none(),
    "the daemon refused the form op: {body}"
  );
  assert_eq!(
    answer["entity"], "ST0001",
    "the answer names the instance, not the kind"
  );

  let over_the_wire: Vec<intentsvcs::form::Triple> =
    serde_json::from_value(answer["fields"].clone()).expect("the fields are triples");
  let in_this_process = in_process(&root, "ST0001");

  // **THE VACUITY GUARD, AND IT IS NOT DECORATION.** Two empty vectors are
  // equal, so without this the assertion below passes for a daemon that
  // answered with nothing and a facade that resolved nothing -- which is the
  // exact failure `Op::Form` was built to make impossible.
  assert!(
    !in_this_process.is_empty(),
    "the in-process derivation produced no rows, so the comparison below proves nothing"
  );
  assert_eq!(
    over_the_wire, in_this_process,
    "the browser's rows and the terminal's rows must be ONE derivation reached two ways"
  );
}

/// **A DEEP LINK IS SERVED, OR CLIENT-SIDE ROUTING IS A FICTION.**
///
/// The client reads `location.pathname` and asks for that view, so the daemon
/// has to answer a path it has no route for -- a reload, a bookmark, or the URL
/// `intent browse` itself opens would otherwise 404 on a page the client would
/// have rendered perfectly.
#[test]
fn a_deep_link_gets_the_shell_and_only_on_get() {
  let daemon = RealDaemon::start();
  daemon.wait_until_it_answers_a_real_op();
  let addr = published(&daemon);

  let (status, body) = get(&addr, "/thread/ST0001");
  assert!(status.contains("200"), "a deep link answered {status}");
  assert!(
    body.contains("id=\"entity\""),
    "the deep link is served the shell, which is the client that will render it"
  );

  // **THE FALLBACK IS `get(shell)` AND NOT `shell`, AND THIS IS THE ARM THAT
  // SAYS SO.** A bare handler answers every method, so a mistyped `POST /opp`
  // would receive HTML at 200 -- an answer that reads as success to a client
  // that asked for JSON.
  let (status, _) = http(
    &addr,
    "POST /thread/ST0001 HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
  );
  assert!(
    status.contains("405"),
    "the fallback must answer GET only, and POST answered {status}"
  );

  // The routes that existed before the fallback still win, which is a property
  // of the router rather than of the order they were written in.
  let (status, svg) = get(&addr, "/intent-logo.svg");
  assert!(
    status.contains("200") && svg.contains("<svg"),
    "the fallback must not shadow a declared route"
  );
}

// ---------------------------------------------------------------------------
// INV-09: the two spellings agree, including about what they cannot do.
// ---------------------------------------------------------------------------

fn cli(root: &Path, home: &Path, args: &[&str]) -> (String, i32) {
  let out = std::process::Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(root)
    .env("HOME", home)
    .output()
    .expect("the intent binary runs");
  (
    String::from_utf8_lossy(&out.stderr).to_string(),
    out.status.code().unwrap_or(-1),
  )
}

/// **BOTH SPELLINGS REFUSE IDENTICALLY WHEN NO DAEMON ANSWERS**, which is the
/// `INV-09` property asserted against behaviour rather than against presence.
/// `tui-design.md` §9 is the rule being kept: the verb does not spawn a process
/// the operator did not ask for.
#[test]
fn neither_spelling_starts_a_daemon_and_both_say_the_same_thing() {
  let root = an_estate();
  let home = short_dir("browse-nodaemon");
  std::fs::create_dir_all(&home).expect("an isolated home");

  let (verb, verb_code) = cli(&root, &home, &["browse", "st", "ST0001"]);
  let (flag, flag_code) = cli(&root, &home, &["edit", "st", "ST0001", "--browser"]);

  assert!(
    verb.contains("no `intentd` is answering"),
    "the subcommand must name what is missing, and said: {verb}"
  );
  assert!(
    verb.contains("intent daemon start"),
    "the refusal must name the remedy, and said: {verb}"
  );
  assert_eq!(
    verb, flag,
    "INV-09: the two spellings of one capability must not refuse differently"
  );
  assert_eq!(verb_code, flag_code, "and must not exit differently");
}

/// **THE OPEN HALF OF `AC-17.6`, ASSERTED SO THAT IT CANNOT GO QUIET.**
///
/// A work package is refused by BOTH spellings, identically, and the refusal
/// names the reason rather than reporting a not-found. When `nav.rs` grows a
/// shape for `Entity::Wp` this test fails, which is the intended signal: the
/// criterion's WP limb is then live and wants a real assertion instead.
#[test]
fn a_work_package_is_refused_by_both_spellings_and_says_why() {
  let root = an_estate();
  let home = short_dir("browse-wp");
  std::fs::create_dir_all(&home).expect("an isolated home");

  let (verb, _) = cli(&root, &home, &["browse", "wp", "ST0001/01"]);
  let (flag, _) = cli(&root, &home, &["edit", "wp", "ST0001/01", "--browser"]);

  assert!(
    verb.contains("cannot open a work package in a browser"),
    "the refusal must name what it cannot do, and said: {verb}"
  );
  assert_eq!(
    verb, flag,
    "INV-09 covers the refusals too, or the twins drift on the case neither can serve"
  );
}

/// **AN ENTITY THAT DOES NOT EXIST IS REFUSED AT THE TERMINAL, NOT IN A
/// BROWSER** -- the browse half of `0238`.
///
/// Before this, nothing on the browse path asked whether the addressed entity
/// existed: a URL was composed for `ST9999` and a browser opened at a page that
/// then errored, so the operator left the terminal they were standing in to be
/// told by a different program about a typo they had just made.
///
/// **THE `no daemon` ARM IS A VACUITY GUARD AND IT IS THE POINT OF THE TEST.**
/// No daemon runs here either, so *it refused* is true with or without the
/// check and an assertion that stopped there would pass against the defect it
/// exists to catch. What discriminates is WHICH refusal arrives: a missing
/// entity must be reported as a missing entity, and never as a missing daemon,
/// because a fact about the machine must not stand in for a fact about the
/// argument.
///
/// **THE THIRD ARM IS THE CONTROL AGAINST OVER-REFUSAL.** A thread that DOES
/// exist has to get past the check and reach the daemon probe -- so a check
/// that refused everything would fail here rather than read as a pass.
#[test]
fn an_entity_that_does_not_exist_is_refused_before_a_browser_is_opened() {
  let root = an_estate();
  let home = short_dir("browse-absent");
  std::fs::create_dir_all(&home).expect("an isolated home");

  let (verb, verb_code) = cli(&root, &home, &["browse", "st", "ST9999"]);
  let (flag, flag_code) = cli(&root, &home, &["edit", "st", "ST9999", "--browser"]);

  assert!(
    verb.contains("no steel thread ST9999"),
    "a missing thread must be named as missing, and said: {verb}"
  );
  assert!(
    !verb.contains("no `intentd` is answering"),
    "the entity is absent whatever the daemon is doing -- reporting the daemon \
     here makes a fact about the machine stand in for a fact about the \
     argument, and said: {verb}"
  );
  assert_eq!(
    verb, flag,
    "INV-09: the two spellings must not refuse differently for a missing entity"
  );
  assert_eq!(verb_code, flag_code, "and must not exit differently");

  // The control: a real thread must reach PAST the check.
  let (real, _) = cli(&root, &home, &["browse", "st", "ST0001"]);
  assert!(
    real.contains("no `intentd` is answering"),
    "a thread that exists must pass the existence check and reach the daemon \
     probe, or the check is refusing things it should admit, and said: {real}"
  );
}

/// **THE WORK-PACKAGE REFUSAL STILL COMES FIRST, AND THE ORDER IS A DECISION.**
///
/// `0238`'s check sits AFTER the `Entity::Wp` refusal deliberately. A real
/// WP-01 cannot be browsed either, so answering *no such work package* for
/// WP-99 would tell the operator that WP-01 would have worked -- which is
/// false. Both a present and an absent work package must therefore get the
/// same answer, and it must be the one naming what this build cannot do.
#[test]
fn an_absent_work_package_is_refused_as_a_work_package_and_not_as_a_typo() {
  let root = an_estate();
  let home = short_dir("browse-wp-absent");
  std::fs::create_dir_all(&home).expect("an isolated home");

  let (absent, _) = cli(&root, &home, &["browse", "wp", "ST0001/99"]);
  let (present, _) = cli(&root, &home, &["browse", "wp", "ST0001/01"]);

  assert!(
    absent.contains("cannot open a work package in a browser"),
    "an absent work package must be refused as a work package, and said: {absent}"
  );
  assert_eq!(
    absent, present,
    "a present and an absent work package must get the SAME answer here, or the \
     refusal implies the present one would have opened"
  );
}
