//! What the CLI and `intentd` say to each other.
//!
//! **IT LIVES HERE FOR THE REASON [`crate::daemon`] LIVES HERE: TWO BINARIES
//! MUST AGREE, AND `intentsvcs` IS THE ONLY CRATE BOTH DEPEND ON.** The CLI
//! serialises a request and the daemon deserialises it; two homes for that
//! shape is a client asking for something the server does not recognise, with
//! nothing comparing the two until a user meets it. The address had this
//! argument first and the protocol inherits it unchanged.
//!
//! **AND IT COSTS NO CRATE A DEPENDENCY, WHICH IS NOT WHY IT WAS PUT HERE BUT
//! IS WHY THE ALTERNATIVE WAS WORSE.** Defining these types in `intentd` would
//! have meant adding `serde` and `serde_json` there -- two more workspace
//! firsts, each owing a written rationale under `AC-08.10` -- to describe a
//! contract the CLI has to understand as well. The dependency question was the
//! symptom; the Highlander violation was the disease.
//!
//! **D56: JSON ONLY, OVER THE SOCKET AND OVER HTTP.** One object per line, the
//! same framing the liveness probe uses, because a daemon that framed its
//! probe one way and its requests another would need two readers on one
//! connection.
//!
//! ## What this is NOT
//!
//! **NOT THE GraphQL FACE.** `AC-08.2`'s dual-path conformance across the whole
//! verb surface is served by a GraphQL skin over the facade
//! ([`crate::graphql`]), and that is still ahead. This is the envelope that
//! carries a request to a project and an answer back -- addressing, binding and
//! failure -- and it is deliberately separable from what is being asked.
//!
//! **AND NOT A SECOND RENDERER.** Nothing here formats anything for a human. A
//! response carries values; the CLI renders them, exactly as it renders the
//! in-process answer, which is what makes the two paths incapable of drifting
//! in their output.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::model::ThreadStatus;

/// One request from a client to the daemon.
///
/// **EVERY REQUEST NAMES ITS PROJECT ROOT, EVEN THOUGH A CONNECTION IS BOUND TO
/// ONE.** The redundancy is deliberate: it lets the daemon REFUSE a request
/// that has wandered to the wrong project rather than serve it against
/// whichever store the connection happened to open. A field that is usually
/// the same as the last one is how a mis-addressed request is caught; omitting
/// it would make the binding unverifiable from the request itself.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
  /// The project this request is about, as the client resolved it.
  ///
  /// The daemon canonicalises it before use -- two clients naming one project
  /// by different paths must reach one store, and a symlinked or relative root
  /// that opened a second store would be two engines on one database wearing
  /// different names.
  pub root: PathBuf,
  /// What is being asked.
  ///
  /// **FLATTENED, SO THE TAG LANDS AT THE TOP LEVEL AND THE LINE READS THE WAY
  /// A CLIENT WOULD WRITE IT BY HAND:** `{"root":"/p","op":"thread_list"}`.
  /// Without this the internally-tagged enum nests inside its own field and the
  /// real wire form is `{"root":"/p","op":{"op":"thread_list"}}` -- which is
  /// what this actually emitted until a hand-written client tried it. **Both
  /// tests passed while it was wrong**, because both sides of the round trip
  /// used serde, and a comparison whose two ends share a source is an identity
  /// rather than a measurement. The literal below is the control that was
  /// missing.
  #[serde(flatten)]
  pub op: Op,
}

/// What a client is asking for.
///
/// **A NEW OPERATION IS A VARIANT, NEVER AN EDIT.** The daemon matches on this
/// exhaustively, so an operation the server does not know cannot be silently
/// dropped -- it fails to deserialise and is refused by name, which is the
/// behaviour a version skew needs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum Op {
  /// Every steel thread in the project, in the order the model holds them.
  ThreadList,
  /// The projects this daemon has opened, and whether their roots still exist.
  ///
  /// **NOT SCOPED TO ONE PROJECT, WHICH IS WHY IT IS ANSWERED WITHOUT BINDING
  /// TO ONE.** `AC-08.1` requires a moved or deleted root to surface rather
  /// than crash, and something has to be able to ask across projects for that
  /// to be visible at all.
  Registry,
}

/// One thread, as much of it as a listing needs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThreadSummary {
  pub id: String,
  pub title: String,
  /// **THE TYPE, NOT A STRING, SO THE WIRE MINTS NO THIRD SPELLING.**
  /// [`ThreadStatus`] already owns two vocabularies for two audiences -- serde's
  /// kebab-case for `thread.json` and `display()` for a person -- and that
  /// separation was earned: the human spelling used to exist twice, in
  /// `views.rs` and `render.rs`, each held in place by its own test against its
  /// own literals so neither could see the other. A `String` here would be a
  /// third home with the same defect available, and the renderer at the far end
  /// would have to parse words back into a type it could have been handed.
  pub status: ThreadStatus,
}

/// One project the daemon has opened.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegisteredProject {
  /// The canonical root, as the daemon holds it.
  pub root: PathBuf,
  /// **DOES THE ROOT STILL EXIST?** A registered project whose directory has
  /// been moved or deleted is a STATE the operator needs reported, never a
  /// panic and never a silent omission from the list. `AC-08.1` names exactly
  /// this, and a listing that quietly dropped the missing ones would satisfy
  /// the words while hiding the thing they were written for.
  pub root_exists: bool,
}

/// What the daemon answers.
///
/// **FAILURE IS A VARIANT AND NOT A SEPARATE CHANNEL**, because a client
/// reading one line has to get an answer or nothing, and "nothing" is already
/// spoken for -- it is what a dead daemon looks like. An error that arrived as
/// a closed connection would be indistinguishable from the daemon dying
/// mid-request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "result", rename_all = "snake_case")]
pub enum Response {
  /// The threads a [`Op::ThreadList`] found.
  Threads { threads: Vec<ThreadSummary> },
  /// The projects a [`Op::Registry`] found.
  Registry { projects: Vec<RegisteredProject> },
  /// The request was understood and could not be served.
  ///
  /// **IT CARRIES A REMEDY BECAUSE EVERY OTHER REFUSAL IN THIS ESTATE DOES.**
  /// A daemon-side failure that reached the operator without one would be the
  /// only error in the tool that says what went wrong and not what to do, and
  /// it would arrive at exactly the moment they have least context: something
  /// they cannot see failed in a process they did not start.
  Error { message: String, remedy: String },
}

impl Response {
  /// Build a refusal, so the two fields are never assembled ad hoc at a call
  /// site and one of them forgotten.
  pub fn error(message: impl Into<String>, remedy: impl Into<String>) -> Response {
    Response::Error {
      message: message.into(),
      remedy: remedy.into(),
    }
  }
}

/// Render a value as one newline-terminated JSON line.
///
/// **THE FRAMING IS WRITTEN ONCE AND BOTH ENDS CALL IT.** A newline appended by
/// hand at one end and forgotten at the other is a reader that blocks forever
/// on a response that was fully sent -- the failure looks like a hung daemon
/// and is a missing byte.
pub fn frame<T: Serialize>(value: &T) -> Result<Vec<u8>, serde_json::Error> {
  let mut out = serde_json::to_vec(value)?;
  out.push(b'\n');
  Ok(out)
}

/// Read one framed request, or a refusal that can be sent straight back.
///
/// **THE PARSE LIVES WITH THE FRAMING, FOR THE FRAMING'S REASON.** A caller
/// doing its own `serde_json::from_slice` would have to invent the refusal
/// message too, and the message for an unrecognised operation is the one place
/// a version skew becomes legible to a human -- so it is written once, here,
/// beside the enum whose variants define what is recognised.
///
/// **AND IT KEEPS `serde_json` OUT OF THE DAEMON'S MANIFEST**, which is not the
/// reason but is the confirmation: a crate that never names the format is a
/// crate that cannot grow a second opinion about it.
pub fn parse_request(line: &[u8]) -> Result<Request, Response> {
  serde_json::from_slice(line.trim_ascii_end()).map_err(|e| {
    Response::error(
      format!("this request could not be read: {e}"),
      "the daemon accepts one JSON object per line. An operation this daemon does not recognise is refused rather than guessed at -- if the client is newer than the daemon, restart the daemon so the pair match.",
    )
  })
}

/// The daemon said something this build cannot read.
///
/// **A TYPE RATHER THAN A `String`, WHICH THE RUST CRITIC IS RIGHT ABOUT AND
/// THE FIRST DRAFT GOT WRONG.** `IN-RS-CODE-004` refuses a stringly-typed error
/// in a library API, and the reason bites here specifically: the caller is the
/// CLI, which has to decide an exit code and a remedy, and a `String` forces it
/// to either re-parse prose or invent its own advice. The sibling
/// [`parse_request`] already returned a structured [`Response`]; this returning
/// text was an asymmetry with no argument behind it.
#[derive(Debug, Error)]
#[error("the daemon's answer could not be read: {source}")]
pub struct UnreadableResponse {
  #[source]
  source: serde_json::Error,
}

impl crate::remedy::Remedy for UnreadableResponse {
  fn remedy(&self) -> String {
    "this build and the running intentd disagree about the response format, which happens when one of the pair has been upgraded and the other has not. Stop the daemon and start it again from the same build as this CLI.".to_string()
  }
}

/// Read one framed response.
///
/// The client's half of [`parse_request`]. A daemon that answered something
/// this build cannot read is a version skew, and it is reported as one rather
/// than as a parse error the operator has to interpret.
pub fn parse_response(line: &[u8]) -> Result<Response, UnreadableResponse> {
  serde_json::from_slice(line.trim_ascii_end()).map_err(|source| UnreadableResponse { source })
}

/// Ask a daemon one question and read its answer.
///
/// **THE ROUND TRIP LIVES BESIDE THE FRAMING, WHICH IS THE SAME ARGUMENT THAT
/// PUT THE FRAMING HERE.** A caller doing its own connect-write-read would own
/// a third opinion about the wire -- the deadline, the newline, what a closed
/// connection means -- and the failure when those drift is a client that hangs
/// rather than one that errors.
///
/// **A CLOSED CONNECTION IS A FAULT, NOT AN EMPTY ANSWER.** Reading zero bytes
/// means the daemon accepted and went away, which is exactly what a crash looks
/// like; treating it as "no result" would report a dead daemon as a project
/// with nothing in it.
pub fn ask(endpoint: &crate::daemon::Endpoint, request: &Request) -> Result<Response, AskError> {
  use std::io::{BufRead, BufReader, Write};

  let framed = frame(request).map_err(AskError::Unsendable)?;
  let mut line = Vec::new();

  match endpoint {
    crate::daemon::Endpoint::Unix(path) => {
      let stream = std::os::unix::net::UnixStream::connect(path).map_err(AskError::Unreachable)?;
      stream
        .set_read_timeout(Some(REQUEST_DEADLINE))
        .map_err(AskError::Unreachable)?;
      (&stream)
        .write_all(&framed)
        .map_err(AskError::Unreachable)?;
      (&stream).flush().map_err(AskError::Unreachable)?;
      BufReader::new(&stream)
        .read_until(b'\n', &mut line)
        .map_err(AskError::Unreachable)?;
    }
    crate::daemon::Endpoint::Tcp(addr) => {
      let stream = std::net::TcpStream::connect(*addr).map_err(AskError::Unreachable)?;
      stream
        .set_read_timeout(Some(REQUEST_DEADLINE))
        .map_err(AskError::Unreachable)?;
      (&stream)
        .write_all(&framed)
        .map_err(AskError::Unreachable)?;
      (&stream).flush().map_err(AskError::Unreachable)?;
      BufReader::new(&stream)
        .read_until(b'\n', &mut line)
        .map_err(AskError::Unreachable)?;
    }
  }

  if line.is_empty() {
    return Err(AskError::ClosedWithoutAnswering);
  }
  parse_response(&line).map_err(AskError::Unreadable)
}

/// How long a request may take before the client gives up.
///
/// **MUCH LONGER THAN THE LIVENESS PROBE'S, AND THE DIFFERENCE IS THE POINT.**
/// The probe's 250ms asks *is anything there*, where a wrong answer costs one
/// redundant in-process run. This asks *do the work*, where giving up early
/// abandons a request the daemon may already have committed. A read this slow
/// means the store is genuinely busy, and waiting is the correct response to
/// that.
const REQUEST_DEADLINE: std::time::Duration = std::time::Duration::from_secs(120);

/// Why a request to the daemon did not produce an answer.
#[derive(Debug, Error)]
pub enum AskError {
  /// The request could not be serialised. A fault in this build.
  #[error("this request could not be serialised: {0}")]
  Unsendable(#[source] serde_json::Error),
  /// The daemon could not be reached, or went silent mid-request.
  #[error("intentd could not be reached: {0}")]
  Unreachable(#[source] std::io::Error),
  /// The daemon accepted the connection and closed it without answering.
  #[error("intentd accepted the request and closed the connection without answering")]
  ClosedWithoutAnswering,
  /// The daemon answered something this build cannot read.
  #[error("{0}")]
  Unreadable(#[source] UnreadableResponse),
}

impl crate::remedy::Remedy for AskError {
  fn remedy(&self) -> String {
    match self {
      AskError::Unsendable(_) => {
        "this is a fault in the CLI rather than in the project or the daemon.".to_string()
      }
      AskError::Unreachable(_) => {
        "the daemon answered the liveness probe a moment ago and is not answering now, so it has stopped or is no longer reachable. Run `intent daemon status`; with no daemon running, this command executes in-process.".to_string()
      }
      AskError::ClosedWithoutAnswering => {
        "the daemon took the request and died before replying, which its log will name. Restart it with `intent daemon run`, and note the request may or may not have been applied -- check before retrying anything that writes.".to_string()
      }
      AskError::Unreadable(inner) => inner.remedy(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_framed_value_ends_in_exactly_one_newline() {
    let framed = frame(&Op::Registry).expect("serialisable");
    assert_eq!(framed.iter().filter(|b| **b == b'\n').count(), 1);
    assert!(framed.ends_with(b"\n"));
  }

  #[test]
  fn a_hand_written_line_is_what_this_actually_accepts() {
    // **THE ONE TEST HERE THAT DOES NOT USE `serde` ON BOTH SIDES.** Everything
    // else round-trips through the derive, so all of it passed while the real
    // wire form was `{"root":"/p","op":{"op":"thread_list"}}` -- serde was
    // agreeing with itself about a shape no client could produce. This types
    // the line out the way a person would and requires it to parse.
    let line = br#"{"root":"/tmp/project","op":"thread_list"}"#;
    let parsed: Request = serde_json::from_slice(line).expect("the documented wire form parses");
    assert_eq!(parsed.op, Op::ThreadList);
    assert_eq!(parsed.root, PathBuf::from("/tmp/project"));

    let registry = br#"{"root":"/tmp/project","op":"registry"}"#;
    let parsed: Request = serde_json::from_slice(registry).expect("parses");
    assert_eq!(parsed.op, Op::Registry);
  }

  #[test]
  fn what_is_emitted_is_what_is_accepted() {
    // The other half: the bytes this crate WRITES must be bytes it reads. A
    // format that parses a hand-written line and emits a different one would
    // fail only between two versions of ourselves.
    let sent = Request {
      root: PathBuf::from("/tmp/project"),
      op: Op::ThreadList,
    };
    let emitted = String::from_utf8(frame(&sent).expect("serialisable")).expect("utf8");
    assert_eq!(
      emitted.trim_end(),
      r#"{"root":"/tmp/project","op":"thread_list"}"#,
      "the emitted form drifted from the documented one"
    );
  }

  #[test]
  fn an_unknown_operation_is_refused_rather_than_defaulted() {
    // **A VERSION SKEW MUST NOT DEGRADE INTO A DIFFERENT REQUEST.** If an
    // unknown tag deserialised into some default, a newer client asking for
    // something this daemon cannot do would be answered as though it had asked
    // for something else -- and the answer would look valid.
    let unknown = br#"{"root":"/x","op":"undo_everything"}"#;
    let parsed: Result<Request, _> = serde_json::from_slice(unknown);
    assert!(
      parsed.is_err(),
      "an operation this build does not know deserialised into one it does"
    );
  }

  #[test]
  fn an_unreadable_request_comes_back_as_a_sendable_refusal() {
    // **THE REFUSAL IS A `Response`, NOT AN ERROR TYPE THE CALLER MUST DRESS.**
    // A daemon that had to compose its own message for this would be composing
    // it at the one moment a human has least context: something they cannot see
    // rejected something they did not send by hand.
    let refusal = parse_request(b"{ not json").expect_err("unreadable");
    match refusal {
      Response::Error { message, remedy } => {
        assert!(!message.is_empty(), "the refusal says what happened");
        assert!(!remedy.is_empty(), "and what to do about it");
      }
      other => panic!("an unreadable request produced {other:?}"),
    }
  }

  #[test]
  fn the_round_trip_preserves_what_was_sent() {
    let sent = Request {
      root: PathBuf::from("/tmp/somewhere"),
      op: Op::ThreadList,
    };
    let framed = frame(&sent).expect("serialisable");
    let back: Request = serde_json::from_slice(framed.trim_ascii_end()).expect("parses");
    assert_eq!(back, sent);
  }
}
