//! **`intent plugin` -- what this INSTALL ships (ST0056 WP-06, `as-observed`).**
//!
//! Three entries, no `keep` flags, and a question about the install rather than
//! about a project -- so it answers outside one, exactly as v2 does.
//!
//! # The one deliberate departure, and why it is pinned here
//!
//! v2's `plugin show` ends with `Run 'intent help <name>' for full command
//! documentation.` Porting that line faithfully would ship a remedy pointing at
//! a spelling this binary does not answer -- AC-06.11's class, arriving through
//! `as-observed` FIDELITY rather than through carelessness, which is the
//! direction nobody is watching.
//!
//! # THE PREMISE MOVED ON 2026-08-30 AND THE CONCLUSION DID NOT
//!
//! This file used to say `intent help` is RETIRED and refuses at exit 2. **That
//! stopped being true**: hv ruled `help` into the 3.0.0 cut and `intent help`
//! now answers rc=0, byte-identical to `intent --help`. The coupling below did
//! its job -- the test went red on its own PREMISE and sent someone back to the
//! decision rather than being quietly edited green.
//!
//! **Re-decided, and the line STAYS DROPPED for a narrower reason than before.**
//! hv ruled `help` in at the ROOT ONLY; the per-command `<cmd> help` shape is
//! recorded and POST-TAG. v2's line names `intent help <name>`, which is
//! exactly the spelling that does not exist: `intent help claude` is rc=1
//! *unexpected argument*. So the remedy would still send the reader somewhere
//! that fails -- the verb is no longer retired, the ARGUMENT is what is absent.
//!
//! The assertion below therefore moves to the premise that actually governs the
//! decision. It is not "`help` refuses" -- that is spent -- it is "the spelling
//! v2's line names does not work". **That premise expires the day hv's post-tag
//! `<cmd> help` lands, and this test fails again and sends someone back here
//! again**, which is the whole point of coupling the two halves. A test that
//! only asserted the absence would keep passing after its own premise expired.

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

  for expected in ["Plugin: claude", "Version:", "Description:", "Location:"] {
    assert!(
      printed.contains(expected),
      "`plugin show` lost {expected:?}:\n{printed}"
    );
  }
  assert!(
    printed.contains("intent/plugins/claude"),
    "the location must name where the manifest was actually read from:\n{printed}"
  );

  // **`Commands (` WAS ASSERTED HERE AND IS DELIBERATELY GONE** (hv,
  // 2026-08-30). It required a COUNT LINE, which a manifest declaring nothing
  // cannot print -- and the shipped manifests now declare nothing, because a
  // hand-written command list in a plugin directory is a third home for a
  // surface the binary is built from, and it had rotted: three of the eight
  // entries across both manifests were false when they were removed.
  //
  // **THE ARM IS REPLACED RATHER THAN DROPPED.** What it was really protecting
  // is that this command does not go silent about commands, and that property
  // survives the ruling in a different form -- the reader is sent to the home
  // that IS true instead of being handed a list that is not.
  assert!(
    printed.contains("Commands"),
    "`plugin show` must still say something about commands, even when the \
     manifest declares none:\n{printed}"
  );
  assert!(
    printed.contains("intent --help"),
    "with no declared commands the reader must be pointed at the real surface, \
     not left with an empty heading:\n{printed}"
  );
}

/// **NOTHING `plugin` PRINTS MAY SEND THE READER TO A SPELLING THAT FAILS.**
///
/// Both halves are asserted together on purpose -- see this file's header. The
/// premise is the ARGUMENT form `intent help <name>`, which is what v2's
/// dropped line actually names; bare `intent help` answers rc=0 since
/// 2026-08-30 and is not what this test is about. If the per-command form ever
/// lands, the first assertion fails and the dropped line becomes a decision to
/// revisit rather than a silent absence.
#[test]
fn no_line_sends_the_reader_to_a_spelling_that_fails() {
  // The bare verb is asserted FIRST, as an anti-vacuity arm: if `help` were
  // retired again the argument form would also fail, and this test would pass
  // for a reason that has nothing to do with what it is checking.
  let bare = run(&["help"]);
  assert_eq!(
    bare.status.code(),
    Some(0),
    "anti-vacuity: bare `intent help` must ANSWER, or the argument-form check \
     below passes for the wrong reason.\nstderr: {}",
    String::from_utf8_lossy(&bare.stderr)
  );

  let help = run(&["help", "claude"]);
  assert_ne!(
    help.status.code(),
    Some(0),
    "this test's PREMISE is that `intent help <name>` -- the spelling v2's \
     dropped line names -- does NOT work. It answers now, so the line `plugin \
     show` deliberately drops is portable again -- go and re-decide it rather \
     than editing this assertion.\nstdout: {}\nstderr: {}",
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
