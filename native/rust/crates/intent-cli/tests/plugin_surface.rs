//! **`intent plugin` -- what this INSTALL ships (ST0056 WP-06, `as-observed`).**
//!
//! Three entries, no `keep` flags, and a question about the install rather than
//! about a project -- so it answers outside one, exactly as v2 does.
//!
//! # The one deliberate departure, and why it is pinned here
//!
//! v2's `plugin show` ends with `Run 'intent help <name>' for full command
//! documentation.` **`intent help` is RETIRED in v3** and refuses at exit 2
//! with *there is no v3 replacement*. Porting that line faithfully would ship a
//! remedy pointing at a verb this binary answers by refusing -- AC-06.11's
//! class, arriving through `as-observed` FIDELITY rather than through
//! carelessness, which is the direction nobody is watching.
//!
//! So the line is dropped, and `no_line_sends_the_reader_to_a_retired_verb`
//! asserts BOTH halves: that `intent help` really does refuse, and that nothing
//! `plugin` prints points at it. **Coupling the two is deliberate.** The reason
//! the line was dropped is that `help` is retired; if that ever stops being
//! true, this test fails and sends someone back to the decision. A test that
//! only asserted the absence would keep passing after its own premise expired,
//! which is the failure mode this thread has paid for repeatedly -- a remedy
//! that was true when written and is caught by nobody when it stops being.

use std::process::{Command, Output};

fn run(args: &[&str]) -> Output {
  Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .output()
    .expect("run the v3 binary")
}

fn stdout(out: &Output) -> String {
  String::from_utf8_lossy(&out.stdout).into_owned()
}

/// Bare `plugin` IS `plugin list` -- v2's behaviour, not a convenience.
///
/// Guarded against vacuity before the equality is believed: two empty outputs
/// compare equal, so the listing must first name a plugin this install actually
/// ships and print the header.
#[test]
fn bare_plugin_is_plugin_list_and_the_listing_is_not_empty() {
  let bare = run(&["plugin"]);
  let list = run(&["plugin", "list"]);

  assert_eq!(bare.status.code(), Some(0), "bare `plugin` must succeed");
  assert_eq!(list.status.code(), Some(0), "`plugin list` must succeed");

  let printed = stdout(&list);
  assert!(
    printed.contains("Intent Plugins"),
    "the listing lost its header: {printed}"
  );
  assert!(
    printed.contains("claude"),
    "this install ships a `claude` plugin and the listing does not name it -- an \
     empty listing would make the equality below vacuous: {printed}"
  );

  assert_eq!(
    stdout(&bare),
    printed,
    "bare `plugin` and `plugin list` must be the same answer"
  );
}

/// `plugin show` reports the manifest's own fields, and locates the plugin.
#[test]
fn show_reports_the_manifest_and_where_it_was_found() {
  let out = run(&["plugin", "show", "claude"]);
  assert_eq!(
    out.status.code(),
    Some(0),
    "`plugin show claude` must succeed"
  );
  let printed = stdout(&out);

  for expected in [
    "Plugin: claude",
    "Version:",
    "Description:",
    "Location:",
    "Commands (",
  ] {
    assert!(
      printed.contains(expected),
      "`plugin show` lost {expected:?}:\n{printed}"
    );
  }
  assert!(
    printed.contains("intent/plugins/claude"),
    "the location must name where the manifest was actually read from:\n{printed}"
  );
}

/// **NOTHING `plugin` PRINTS MAY SEND THE READER TO A VERB THAT REFUSES.**
///
/// Both halves are asserted together on purpose -- see this file's header. If
/// `intent help` is ever un-retired, the first assertion fails and the dropped
/// line becomes a decision to revisit rather than a silent absence.
#[test]
fn no_line_sends_the_reader_to_a_retired_verb() {
  let help = run(&["help"]);
  assert_eq!(
    help.status.code(),
    Some(2),
    "this test's PREMISE is that `intent help` is retired. It is not refusing any \
     more, so the line `plugin show` deliberately drops may be portable again -- \
     go and re-decide it rather than editing this assertion.\nstdout: {}\nstderr: {}",
    stdout(&help),
    String::from_utf8_lossy(&help.stderr)
  );

  for args in [vec!["plugin", "show", "claude"], vec!["plugin", "list"]] {
    let printed = stdout(&run(&args));
    assert!(
      !printed.contains("intent help"),
      "`intent {}` points the reader at `intent help`, which this build refuses:\n{printed}",
      args.join(" ")
    );
  }
}

/// An unknown plugin is refused, and the refusal names a route that exists.
#[test]
fn an_unknown_plugin_is_refused_and_the_remedy_is_reachable() {
  let out = run(&["plugin", "show", "nosuchplugin"]);
  assert_eq!(
    out.status.code(),
    Some(1),
    "v2 exits 1 on a plugin it cannot find"
  );

  let said = String::from_utf8_lossy(&out.stderr).into_owned();
  assert!(
    said.contains("nosuchplugin"),
    "the refusal must name what was asked for: {said}"
  );
  assert!(
    said.contains("plugin list"),
    "the remedy must name a route: {said}"
  );
  // The remedy is DRIVEN, not read. A remedy naming a verb that refuses is the
  // defect this family already carried once, in the line above.
  assert_eq!(
    run(&["plugin", "list"]).status.code(),
    Some(0),
    "the remedy names `intent plugin list`, so that must actually work"
  );
}

/// A plugin is a property of the INSTALL, so there is no project to be in.
#[test]
fn plugin_answers_outside_a_project() {
  let dir = tempfile::tempdir().expect("tempdir");
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(["plugin", "list"])
    .current_dir(dir.path())
    .output()
    .expect("run the v3 binary");
  assert_eq!(
    out.status.code(),
    Some(0),
    "v2 lists outside a project at exit 0.\nstderr: {}",
    String::from_utf8_lossy(&out.stderr)
  );
  assert!(
    stdout(&out).contains("Intent Plugins"),
    "and it lists rather than half-answering: {}",
    stdout(&out)
  );
}
