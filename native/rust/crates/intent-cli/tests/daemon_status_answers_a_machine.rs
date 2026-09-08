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
    v.get("pid").is_none() && v.get("endpoint").is_none() && v.get("url").is_none(),
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

/// **`ST0064`: THE MENUBAR ITEM'S TITLE IS THIS FIELD AND ITS ACTION IS OPENING
/// IT, SO THE TWO CLAIMS ARE ASSERTED TOGETHER.** A `url` that is well-formed
/// and serves nothing would render a menu item that looks right and does
/// nothing when clicked -- the failure lands in another language, in another
/// build, in a click nobody scripts.
///
/// **THE UNAUTHENTICATED GET IS THE POINT, NOT AN INCIDENTAL.** The HTTP face
/// requires a bearer token for `/op` (`D56`), and a browser sent to a bare
/// address carries no `Authorization` header. If `/` ever starts demanding the
/// secret, the honest consequence is that this menu item cannot exist without
/// putting a secret in a URL -- which is an `hv` decision, not a patch. This
/// test is what would surface that day, rather than an operator finding a login
/// wall in their browser.
#[test]
fn live_names_a_browser_address_that_serves_a_page_without_a_token() {
  let daemon = RealDaemon::start();
  let v = status_json(daemon.home());

  let url = v["url"]
    .as_str()
    .unwrap_or_else(|| panic!("a live daemon publishes a loopback address to open: {v}"));
  // **NO TRAILING SLASH, BECAUSE THIS STRING IS THE MENU ITEM'S TITLE VERBATIM.**
  // The app renders what it is given and derives nothing, so any tidying of the
  // address has to happen here or it happens twice.
  let authority = url
    .strip_prefix("http://")
    .unwrap_or_else(|| panic!("`url` is what a browser is handed, scheme and all: {url}"));
  authority
    .parse::<std::net::SocketAddr>()
    .unwrap_or_else(|e| panic!("`url`'s authority must be the published address ({e}): {url}"));

  // **A RAW GET RATHER THAN A CLIENT DEPENDENCY.** `dependency_rationale.rs`
  // audits this workspace's manifest, and one request with no headers does not
  // earn an HTTP stack.
  let status_line = plain_get(authority, "/");
  assert!(
    status_line.starts_with("HTTP/1.1 200"),
    "a browser with no token must get the status page, not a refusal: {status_line}"
  );
}

/// One unauthenticated `GET`, returning the status line.
fn plain_get(authority: &str, path: &str) -> String {
  use std::io::{Read, Write};

  let mut stream = std::net::TcpStream::connect(authority).expect("connect to the published port");
  stream
    .set_read_timeout(Some(std::time::Duration::from_secs(5)))
    .expect("set a deadline");
  write!(
    stream,
    "GET {path} HTTP/1.1\r\nHost: {authority}\r\nConnection: close\r\n\r\n"
  )
  .expect("write the request");

  let mut body = Vec::new();
  stream.read_to_end(&mut body).expect("read the response");
  String::from_utf8_lossy(&body)
    .lines()
    .next()
    .unwrap_or_default()
    .to_string()
}
