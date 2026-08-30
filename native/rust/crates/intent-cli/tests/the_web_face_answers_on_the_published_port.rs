//! **`AC-08.9`: one published port, two protocols, and the estate's data behind
//! a token.**
//!
//! **THE THREE CONDITIONS THIS FILE EXISTS TO MEET ARE vc's, RULED BEFORE IT
//! WAS BUILT** (2026-08-30), and each closes a hole the others cannot see:
//!
//! 1. **The recogniser is driven TWO-SIDED on planted input.** An HTTP request
//!    must not be answered as a frame, and a frame must not be answered as
//!    HTTP. **A one-sided witness passes under the very thing it replaced** --
//!    a daemon that had simply stopped speaking frames would satisfy every
//!    HTTP arm here perfectly.
//! 2. **A tokenless HTTP request is REFUSED, as a test rather than a comment.**
//!    Any page the operator's browser is showing can `fetch` a loopback port,
//!    so the token is the whole defence, and a defence that lives only in a
//!    doc comment is not one.
//! 3. **The frame branch is untouched** -- which the probe arm measures rather
//!    than asserts.
//!
//! **THE DAEMON IS A REAL `intentd` UNDER AN ISOLATED `HOME`.** A fixture
//! listener could serve HTTP and would prove nothing about the sorting, which
//! is the whole subject.

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

mod common;
use common::RealDaemon;

/// The loopback address this daemon published.
///
/// **READ FROM THE PUBLISHED FILE, NEVER ASSUMED**, and least of all assumed to
/// be `Published::PREFERRED`. These tests run in parallel: the first daemon to
/// start takes 51737 and every one after it falls back to a kernel-chosen port
/// -- so a test that hardcoded the memorable number would pass alone, fail in
/// the suite, and be blamed on flakiness. **That is the preference-not-a-promise
/// rule meeting its first consumer**, and the consumer obeys it.
///
/// `RealDaemon::endpoint` is deliberately not used: it returns the first
/// candidate, which is the unix socket, and no browser can reach one.
fn published(daemon: &RealDaemon) -> String {
  let path = intentsvcs::userstate::daemon_address_file_under(daemon.home());
  std::fs::read_to_string(&path)
    .unwrap_or_else(|e| panic!("no address published at {}: {e}", path.display()))
    .trim()
    .to_string()
}

/// This run's secret, read the way a client reads it.
fn token(daemon: &RealDaemon) -> String {
  intentsvcs::daemon::Token::read_under(daemon.home()).expect("the daemon published a token")
}

/// One HTTP request over a bare socket, as `(status_line, body)`.
///
/// **HAND-ROLLED RATHER THAN THROUGH A CLIENT CRATE, AND THE REASON IS THE
/// SUBJECT.** This file's claim is about what the daemon does with the FIRST
/// BYTE of a connection; a client library would be free to reuse connections,
/// upgrade, or pipeline, and any of those would put something between the test
/// and the thing it measures. It also adds no dependency to assert a property
/// of a protocol this simple.
fn http(addr: &str, request: &str) -> (String, String) {
  let mut socket = TcpStream::connect(addr).expect("connect to the published port");
  socket
    .write_all(request.as_bytes())
    .expect("write the request");
  socket.flush().expect("flush");
  let mut reader = BufReader::new(socket);
  let mut status = String::new();
  reader.read_line(&mut status).expect("a status line");

  let mut length = 0usize;
  loop {
    let mut header = String::new();
    reader.read_line(&mut header).expect("a header line");
    if header.trim().is_empty() {
      break;
    }
    if let Some(value) = header.to_ascii_lowercase().strip_prefix("content-length:") {
      length = value.trim().parse().unwrap_or(0);
    }
  }
  let mut body = vec![0u8; length];
  std::io::Read::read_exact(&mut reader, &mut body).expect("the whole body");
  (
    status.trim().to_string(),
    String::from_utf8_lossy(&body).to_string(),
  )
}

fn get(addr: &str, path: &str) -> (String, String) {
  http(
    addr,
    &format!("GET {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n"),
  )
}

// ---------------------------------------------------------------------------
// Condition 1, first side: an HTTP request is answered as HTTP.
// ---------------------------------------------------------------------------

/// **THE SHELL IS SERVED AND IT CARRIES THE MARK**, which is the whole of what
/// hv asked for -- *can intentd serve anything* -- and is deliberately all of
/// it. The interface goes on top of `/op`, and building it here would be taking
/// WP-17's criterion.
#[test]
fn a_browser_gets_the_shell_page_and_the_mark() {
  let daemon = RealDaemon::start();
  let addr = published(&daemon);

  let (status, body) = get(&addr, "/");
  assert!(status.contains("200"), "GET / answered {status}");
  assert!(
    body.contains("<img src=\"/intent-logo.svg\""),
    "the shell asks for the mark: {body}"
  );

  let (status, svg) = get(&addr, "/intent-logo.svg");
  assert!(status.contains("200"), "the mark answered {status}");
  assert!(
    svg.starts_with("<?xml") && svg.contains("<svg"),
    "the mark is the SVG the repository carries, served from the binary"
  );
}

// ---------------------------------------------------------------------------
// Condition 1, second side: a frame is STILL answered as a frame.
// ---------------------------------------------------------------------------

/// **THE ARM THAT MAKES THE OTHERS MEAN SOMETHING.** Every HTTP arm above
/// passes on a daemon that has stopped speaking frames on this port
/// altogether -- which would be a silent removal of the TCP entry's whole
/// purpose. `candidates` puts the unix socket FIRST and APPENDS this port, so
/// it is the socket's understudy, reached exactly when the socket is what is
/// broken.
#[test]
fn a_framed_client_is_still_answered_on_the_same_port() {
  let daemon = RealDaemon::start();
  let addr = published(&daemon);

  let mut socket = TcpStream::connect(&addr).expect("connect to the published port");
  socket
    .write_all(b"{\"intent_probe\":1}\n")
    .expect("write a frame");
  socket.flush().expect("flush");
  let mut reply = String::new();
  BufReader::new(socket)
    .read_line(&mut reply)
    .expect("a framed reply");

  assert_eq!(
    reply.trim(),
    "{\"intent_probe\":\"ack\"}",
    "the frame branch answers exactly as it did before the port learned HTTP"
  );
}

// ---------------------------------------------------------------------------
// Condition 2: the estate is behind the token.
// ---------------------------------------------------------------------------

/// **ANY PAGE THE OPERATOR HAS OPEN CAN `fetch` THIS PORT.** Loopback is not a
/// permission boundary -- it is reachable by every local process and by the
/// browser itself -- so an ungated `/op` is an unauthenticated read of the
/// estate from a tab the operator did not think was theirs.
#[test]
fn a_tokenless_request_for_the_estate_is_refused() {
  let daemon = RealDaemon::start();
  let addr = published(&daemon);
  let body = "{\"root\":\"/tmp\",\"op\":\"registry\"}";

  let (status, answer) = http(
    &addr,
    &format!(
      "POST /op HTTP/1.1\r\nHost: localhost\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
      body.len()
    ),
  );
  assert!(status.contains("401"), "an untokened /op answered {status}");
  assert!(
    !answer.contains("\"projects\""),
    "the refusal must not carry the answer it refused: {answer}"
  );
  assert!(
    answer.contains("intentd.token"),
    "the refusal names where the operator's token lives: {answer}"
  );
}

/// **AND THE TOKEN ADMITS, or the arm above would pass on a door that is simply
/// shut.** This is the same two-sidedness condition 1 asks for, applied to the
/// lock rather than to the sorter.
#[test]
fn the_published_token_admits_and_the_answer_is_the_socket_s_answer() {
  let daemon = RealDaemon::start();
  let addr = published(&daemon);
  let body = "{\"root\":\"/tmp\",\"op\":\"registry\"}";

  let (status, answer) = http(
    &addr,
    &format!(
      "POST /op HTTP/1.1\r\nHost: localhost\r\nAuthorization: Bearer {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
      token(&daemon),
      body.len()
    ),
  );
  assert!(status.contains("200"), "a tokened /op answered {status}");
  assert!(
    answer.contains("\"result\":\"registry\""),
    "the body is a `wire::Response`, framed by the same function the socket \
     frames with -- not a second serialiser: {answer}"
  );
}

/// **THE MODE IS THE MECHANISM, SO THE MODE IS THE ASSERTION** (vc's
/// condition). The file IS the authorisation: anything that can read it can
/// drive the daemon. A world-readable token would make the check theatre --
/// present, checked, and passed by everyone -- which is worse than no check,
/// because the port would then look guarded to anyone reading the code.
#[test]
fn the_token_is_readable_only_by_its_owner() {
  use std::os::unix::fs::PermissionsExt;
  let daemon = RealDaemon::start();
  let path = intentsvcs::userstate::daemon_token_file_under(daemon.home());

  let mode = std::fs::metadata(&path)
    .expect("the daemon published a token")
    .permissions()
    .mode()
    & 0o777;
  assert_eq!(
    mode,
    0o600,
    "the token is 0600, got {mode:o} at {}",
    path.display()
  );
}

/// **THE TOKEN EXISTS BEFORE THE ADDRESS DOES** (vc's ordering condition).
/// Publishing an address whose token is not yet readable advertises an endpoint
/// nobody can legitimately use -- the state `Published` makes unexpressible for
/// the address itself, arriving one file later. `RealDaemon` waits on a real
/// op, so by the time it returns the port is answering; the token being present
/// at that moment is the observable form of the ordering.
#[test]
fn a_published_port_always_has_a_token_behind_it() {
  let daemon = RealDaemon::start();
  assert!(
    intentsvcs::daemon::Token::read_under(daemon.home()).is_ok(),
    "the daemon is answering, so its token was written before it published"
  );
}
