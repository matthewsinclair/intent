//! The probe is a request AND a response, and both ends of it live in one home.
//!
//! `AC-08.3` gives the client a predicate -- *exists and ANSWERS* -- and
//! `AC-08.11` gives the daemon the obligation to answer it on the accept path.
//! Between them sits an agreement about bytes that no criterion states, because
//! it is an implementation detail of the pair: what the client writes, and what
//! counts as an answer.
//!
//! **THIS FILE IS THE WITNESS FOR THAT AGREEMENT, AND IT HOLDS ITS OWN
//! LITERALS.** `is_probe_frame` is exported so the daemon can recognise the
//! frame without re-spelling it -- one home for the comparison. But a test that
//! imported the frame and asked whether the recogniser accepted it would be
//! asserting that a value equals itself, true under every wording including a
//! wrong one. So the bytes are written out here, by hand, and this is the file
//! that goes red if the wire form ever moves.
//!
//! **THE FAILURE IT GUARDS IS THE QUIET ONE.** A daemon that stopped
//! recognising the probe would answer nothing; every client would read that as
//! ABSENT and route in-process; and two sync engines would land on one store --
//! which `design.md:22` exists to forbid -- with no error anywhere, because
//! each half is behaving exactly as designed.

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixListener;
use std::path::Path;

use intentsvcs::daemon::{self, Endpoint};

/// The bytes the client puts on the wire, written out rather than imported.
const FRAME_ON_THE_WIRE: &[u8] = b"{\"intent_probe\":1}\n";

#[test]
fn the_recogniser_accepts_the_bytes_the_client_actually_sends() {
  assert!(
    daemon::is_probe_frame(FRAME_ON_THE_WIRE),
    "the daemon no longer recognises the frame the client writes. Every probe would go unanswered, every CLI would route in-process, and a daemon holding the store would be invisible to all of them"
  );
}

#[test]
fn a_line_reader_that_stripped_the_newline_is_still_recognised() {
  // A daemon reading with `read_line` and trimming, or with a framed codec, has
  // not sent a different request -- it has framed the same one. Refusing here
  // would make recognition depend on the reader's habits.
  assert!(daemon::is_probe_frame(b"{\"intent_probe\":1}"));
  assert!(daemon::is_probe_frame(b"{\"intent_probe\":1}\r\n"));
}

#[test]
fn near_misses_are_not_the_probe() {
  // **WITHOUT THIS THE RECOGNISER COULD BE `|_| true` AND EVERY TEST ABOVE
  // WOULD STILL PASS.** A predicate that accepts everything answers the probe
  // correctly and answers a real request with an ack, which is worse than not
  // recognising it at all.
  assert!(!daemon::is_probe_frame(b""));
  assert!(!daemon::is_probe_frame(b"\n"));
  assert!(!daemon::is_probe_frame(b"{\"intent_probe\":2}\n"));
  assert!(!daemon::is_probe_frame(b"{\"intent_probe\": 1}\n"));
  assert!(!daemon::is_probe_frame(
    b"{\"query\":\"{ threads { id } }\"}\n"
  ));
}

#[test]
fn the_reply_is_json_and_newline_terminated() {
  // D56: the daemon emits JSON only, over the socket AND over HTTP. The probe
  // is not exempt just because its content is empty.
  let reply = daemon::PROBE_REPLY;
  assert!(
    reply.ends_with(b"\n"),
    "the reply must be newline-terminated: a reader that framed on newlines would block forever"
  );
  let text = std::str::from_utf8(reply).expect("the reply is UTF-8");
  let parsed: serde_json::Value =
    serde_json::from_str(text.trim_end()).expect("D56: the daemon emits JSON only");
  assert!(
    parsed.is_object(),
    "a bare scalar is JSON but not an output contract: {text}"
  );
}

#[test]
fn the_reply_promises_nothing_a_client_would_have_to_read() {
  // **THE EMPTINESS IS ASSERTED, NOT ASSUMED.** `Endpoint::answers` reads one
  // byte and parses nothing, so any field here would be a promise with no check
  // behind it -- a version or a pid that looks useful and rots in silence. This
  // is what stops the next person adding one without meeting the argument.
  let text = std::str::from_utf8(daemon::PROBE_REPLY).expect("UTF-8");
  let parsed: serde_json::Value = serde_json::from_str(text.trim_end()).expect("JSON");
  let object = parsed.as_object().expect("an object");
  assert_eq!(
    object.len(),
    1,
    "the probe reply carries data a client never reads: {text}. If the daemon has something to say, it says it in a response to a real request, where somebody is reading"
  );
}

// ---------------------------------------------------------------------------
// The two halves are shown to COMPOSE, which is the thing neither half can
// establish alone.
// ---------------------------------------------------------------------------

/// A one-shot listener that answers exactly the way the daemon is required to.
///
/// It accepts once, reads a line, and replies only if the line is the probe.
/// The thread ends after that connection -- the probe opens a fresh connection
/// per call and never reuses one, so a loop would outlive its purpose.
fn responder_that(answers: bool, path: &Path) {
  let listener = UnixListener::bind(path).expect("bind the fixture listener");
  std::thread::spawn(move || {
    let Ok((stream, _)) = listener.accept() else {
      return;
    };
    let mut reader = BufReader::new(&stream);
    let mut line = Vec::new();
    if reader.read_until(b'\n', &mut line).is_err() {
      return;
    }
    if answers && daemon::is_probe_frame(&line) {
      let mut out = &stream;
      let _ = out.write_all(daemon::PROBE_REPLY);
      let _ = out.flush();
    }
  });
}

#[test]
fn a_listener_using_both_halves_is_seen_as_answering() {
  let dir = tempfile::tempdir().expect("tempdir");
  let path = dir.path().join("intentd.sock");
  responder_that(true, &path);

  assert!(
    Endpoint::Unix(path).answers(),
    "a listener that recognises the frame with `is_probe_frame` and replies with `PROBE_REPLY` is not seen as answering by the client's own predicate. The two halves of the probe do not compose, and nothing but this test would say so"
  );
}

#[test]
fn a_listener_that_recognises_but_never_replies_is_seen_as_absent() {
  // **THE CONTROL THAT MAKES THE ONE ABOVE MEAN SOMETHING.** Without it, the
  // green above could come from `answers()` being true for any listener at all
  // -- which is exactly the bare-connect predicate `AC-08.3` replaced, and the
  // fixture would be an instance of the phantom it is standing against.
  let dir = tempfile::tempdir().expect("tempdir");
  let path = dir.path().join("intentd.sock");
  responder_that(false, &path);

  assert!(
    !Endpoint::Unix(path).answers(),
    "a listener that accepts and never answers reported itself as live. A successful connect is not an answer"
  );
}
