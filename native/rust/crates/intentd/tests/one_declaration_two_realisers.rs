//! `AT-17.1` / `AC-17.1`: the same edit, made through EACH realiser, reaches
//! an identical store state.
//!
//! **THE TWO REALISERS ARE THE TWO TRANSPORTS, NOT TWO RENDERERS.** The
//! criterion says the assertion is made *by driving both against the same
//! fixture and diffing the model, never by inspecting the two renderers for
//! similarity* -- so this file drives the IN-PROCESS door (what the TUI
//! realiser reaches) and the WIRE door (what a browser realiser reaches
//! through `intentd`) and compares the `Thread` each leaves behind.
//!
//! **WHY THIS IS NOT VACUOUS EVEN THOUGH BOTH ARMS END AT `Facade::set`.**
//! One door reached two ways is the design, and it is exactly what makes the
//! criterion assertable rather than a claim about two implementations
//! agreeing. The two ways are still different in ways that can diverge: the
//! wire arm serialises the value through JSON and back, parses an address out
//! of a URL STRING rather than receiving one typed, and lands on a facade the
//! daemon opened on its own store thread with its own canon snapshot. Any of
//! those three can carry a value that in-process never touches -- which is
//! what `no_realiser_is_a_second_opinion_about_what_was_written` drives.
//!
//! **AND THE INSTRUMENT IS CONTROLLED.** `the_diff_can_actually_fail` makes
//! the two arms disagree ON PURPOSE and asserts the comparison catches it. A
//! model diff that passes for two different edits would pass for everything,
//! and this file's whole verdict would mean nothing.

use crate::common::{RunningDaemon, project};
use intentsvcs::facade::{Facade, FacadeContext};
use intentsvcs::model::Thread;
use intentsvcs::wire::{Request, Response};

const MINTED: &str = "Minted for two realisers";
const EDITED: &str = "Edited through a realiser";

/// A project carrying one thread, and that thread's id.
fn project_with_a_thread(tag: &str) -> (std::path::PathBuf, String) {
  let root = project(tag);
  let mut facade = open(&root);
  let id = facade.st_new(MINTED).expect("mint one thread");
  (root, id)
}

/// A facade on this root.
///
/// **A FRESH ONE PER READ, DELIBERATELY.** A facade holds a canon snapshot, so
/// reading back through the one that performed the write would report what
/// that process believes rather than what the store holds -- and the wire arm
/// writes through a facade this test never touches.
fn open(root: &std::path::Path) -> Facade {
  let opened = intentsvcs::project::Project::open(root).expect("the project just created");
  let ctx = FacadeContext {
    principal: "test".to_string(),
    project_id: opened.config().project_id.clone().unwrap_or_default(),
    version: env!("CARGO_PKG_VERSION").to_string(),
  };
  Facade::open(opened, ctx).expect("open it")
}

/// The thread as the STORE holds it, read through a facade that did no writing.
fn stored(root: &std::path::Path, id: &str) -> Thread {
  open(root).st_show(id).expect("the thread is there").clone()
}

/// The edit, made the way the TUI realiser makes it: straight at the facade.
fn through_the_in_process_door(root: &std::path::Path, id: &str, title: &str) {
  let address =
    intentsvcs::address::parse(&format!("intent:///threads/{id}")).expect("an addressable thread");
  open(root)
    .set(&address, "title", title.into())
    .expect("the title is settable");
}

/// The edit, made the way a browser realiser makes it: as a line on the wire.
///
/// **THE LINE IS WRITTEN OUT AS A CLIENT WOULD SEND IT AND READ BACK BY THE
/// SHIPPED READER**, never assembled with `serde_json` here. A test that built
/// the request through the JSON crate directly would be a second opinion about
/// what a request IS, and would keep passing while a real client could not be
/// understood at all -- the discipline `daemon_subscriptions.rs` already
/// records for the event feed, applied to the request side.
fn through_the_wire_door(daemon: &RunningDaemon, root: &std::path::Path, id: &str, title: &str) {
  let line = format!(
    r#"{{"root":"/unused","op":"set","url":"intent:///threads/{id}","field":"title","value":"{title}"}}"#
  );
  let parsed = intentsvcs::wire::parse_request(line.as_bytes())
    .unwrap_or_else(|refusal| panic!("the shipped reader refused the line: {refusal:?}"));
  let response = daemon.ask(Request {
    root: root.to_path_buf(),
    op: parsed.op,
  });
  match response {
    Response::Set { outcome } => {
      assert_eq!(
        outcome["moved"], true,
        "the wire door answered without moving the field: {outcome}"
      );
    }
    other => panic!("the wire door answered {other:?}"),
  }
}

#[test]
fn no_realiser_is_a_second_opinion_about_what_was_written() {
  let daemon = RunningDaemon::start();
  let (in_process_root, in_process_id) = project_with_a_thread("RealiserA");
  let (wire_root, wire_id) = project_with_a_thread("RealiserB");

  through_the_in_process_door(&in_process_root, &in_process_id, EDITED);
  through_the_wire_door(&daemon, &wire_root, &wire_id, EDITED);

  let a = stored(&in_process_root, &in_process_id);
  let b = stored(&wire_root, &wire_id);

  // **THE WHOLE RECORD, NOT THE EDITED FIELD.** A realiser that wrote the
  // title correctly and cleared a neighbouring field would pass a check that
  // only read the title back, and that is precisely the divergence the
  // criterion is about.
  assert_eq!(
    a, b,
    "the same edit through two realisers left two different threads"
  );
  assert_eq!(a.title, EDITED, "neither realiser made the edit at all");
}

#[test]
fn the_diff_can_actually_fail() {
  let daemon = RunningDaemon::start();
  let (in_process_root, in_process_id) = project_with_a_thread("ControlA");
  let (wire_root, wire_id) = project_with_a_thread("ControlB");

  // The SAME machinery, driven to two DIFFERENT values. If the comparison
  // above can pass for this, it can pass for anything.
  through_the_in_process_door(&in_process_root, &in_process_id, EDITED);
  through_the_wire_door(&daemon, &wire_root, &wire_id, "A different title");

  let a = stored(&in_process_root, &in_process_id);
  let b = stored(&wire_root, &wire_id);

  assert_ne!(
    a, b,
    "two different edits compared EQUAL, so the diff in the row above proves nothing"
  );
}
