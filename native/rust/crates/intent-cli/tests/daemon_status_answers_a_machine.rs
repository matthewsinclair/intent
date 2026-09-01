//! `ST0064` `AC-01.2`: the menubar app reaches the daemon's health THROUGH this
//! verb, so this verb owes it a face a decoder can read.
//!
//! **THE SHAPE WAS NAMED BY ITS CONSUMER, NOT MINTED HERE** (ic, 2026-08-31).
//! A bare lowercase `state` discriminator matching the `Health` variant names,
//! `endpoint` present iff live, `pid` present iff stale, nothing else. Their
//! decoder is `enum State: String` plus per-state optionals, which is stable
//! against added fields and brittle against a renamed variant -- so the variant
//! NAMES are what this file pins.
//!
//! **WHY A MACHINE FACE AT ALL, RATHER THAN LETTING THE APP READ THE PROSE.**
//! `AC-01.2` forbids two predicates that agree; a prose parser in Swift is one
//! of them wearing a different hat, and it would break on the day somebody
//! improves a sentence. The roster was narrowed to `terminal` on 2026-08-31
//! ending *widen it again when a projection is built*, and the condition fired
//! when ic began the port.
//!
//! **THE LIMIT, STATED RATHER THAN DISCOVERED: `stale` IS NOT DRIVEN HERE.**
//! Constructing it needs a process holding the lock while not answering, which
//! is a library-level fixture rather than a CLI one. It IS constructed
//! deterministically in `intentsvcs`'s `daemon_health_splits_stale_from_absent`,
//! against the same `Health` value this renders; what is unproven from here is
//! only the rendering of that one variant, and its key is pinned below from the
//! same source as the other two.

use std::path::Path;
use std::process::Command;

use crate::common::RealDaemon;

fn bin() -> std::path::PathBuf {
  std::path::PathBuf::from(env!("CARGO_BIN_EXE_intent"))
}

fn status_json(home: &Path) -> serde_json::Value {
  let out = Command::new(bin())
    .args(["daemon", "status", "--format", "json"])
    .env("HOME", home)
    .output()
    .expect("run intent");
  let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
  assert_eq!(
    out.status.code(),
    Some(0),
    "`daemon status --format json` answered rc={:?}: {text}{}",
    out.status.code(),
    String::from_utf8_lossy(&out.stderr)
  );
  serde_json::from_str(&text)
    .unwrap_or_else(|e| panic!("the machine face did not emit JSON ({e}): {text}"))
}

#[test]
fn absent_is_the_state_and_nothing_else() {
  let home = tempfile::tempdir().expect("tempdir");
  let v = status_json(home.path());

  assert_eq!(v["state"], "absent");
  // **THE ABSENCE OF THE OTHER KEYS IS THE CONTRACT, NOT AN INCIDENTAL.** ic
  // gates on optionals, so a `pid: null` or an empty `endpoint` would decode as
  // present-and-meaningless rather than absent.
  assert!(
    v.get("pid").is_none() && v.get("endpoint").is_none(),
    "absent carries the state alone: {v}"
  );
}

#[test]
fn live_names_the_endpoint_and_no_pid() {
  let daemon = RealDaemon::start();
  let v = status_json(daemon.home());

  assert_eq!(
    v["state"], "live",
    "a daemon that answers must render as live: {v}"
  );
  assert!(
    v["endpoint"].as_str().is_some_and(|e| !e.is_empty()),
    "live carries the endpoint the app shows: {v}"
  );
  assert!(
    v.get("pid").is_none(),
    "`pid` belongs to stale alone -- a live row carrying one invites the app to \
     offer an investigate affordance for a healthy daemon: {v}"
  );
}

/// **THE DISCRIMINATOR VALUES ARE ic's DECODER CONTRACT, SO THEY ARE PINNED AS
/// LITERALS.** A renamed `Health` variant would still compile, still serialise,
/// and silently stop decoding in Swift -- the failure lands in another language
/// in another repository, which is exactly the kind this estate cannot see.
#[test]
fn the_three_state_names_are_the_ones_the_consumer_decodes() {
  let home = tempfile::tempdir().expect("tempdir");
  let absent = status_json(home.path());
  let daemon = RealDaemon::start();
  let live = status_json(daemon.home());

  let seen: Vec<&str> = vec![
    absent["state"].as_str().expect("absent names a state"),
    live["state"].as_str().expect("live names a state"),
  ];
  assert_eq!(
    seen,
    vec!["absent", "live"],
    "the state names must be the bare lowercase variant names ic decodes"
  );
  // `stale` is not drivable from here (see the header); its literal is asserted
  // to exist in the renderer so a rename cannot pass unnoticed.
  let rendered =
    std::fs::read_to_string(testkit::workspace_root().join("crates/intent-cli/src/render.rs"))
      .expect("read the renderer");
  assert!(
    rendered.contains(r#""state": "stale""#),
    "the renderer no longer emits the `stale` discriminator ic decodes"
  );
}
