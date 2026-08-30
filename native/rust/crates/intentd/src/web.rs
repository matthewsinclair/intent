//! The HTTP face (`AC-08.9`, D56).
//!
//! **THE DAEMON EMITS JSON ONLY, AND THIS FILE IS NOT AN EXCEPTION TO THAT.**
//! D56 rules one output contract -- GraphQL/JSON over the socket AND over HTTP
//! -- with *no HTML beyond a single static shell page*, and the deciding
//! argument is the menubar app: **a SwiftUI client cannot consume
//! server-rendered markup**, so a daemon that renders domain HTML for browsers
//! ends up serving JSON to the menubar as well, which is two output contracts
//! reached by the door the decision was written to shut. So there is exactly
//! one page here, it carries no domain data, and everything a renderer needs
//! comes back from [`op`] as JSON.
//!
//! **`/op` ANSWERS THE SAME `dispatch` THE SOCKET ANSWERS -- the same function,
//! not a second one that agrees today.** That is the criterion's own clause and
//! it is why this module holds no registry logic, no binding rule and no
//! response shape of its own: it deserialises, calls, and serialises.
//!
//! # WHY THE SHELL IS UNGATED AND `/op` IS NOT
//!
//! **Loopback is not a permission boundary.** Every process on this machine
//! reaches `127.0.0.1`, and so does any page the operator's browser happens to
//! be showing. The estate's data therefore needs the token
//! [`intentsvcs::daemon::Token`] mints; the shell does not, because it is a
//! logo and an empty skeleton. **Gating the page too would mean an operator
//! cannot confirm their daemon is up without first finding a secret**, which
//! buys nothing -- the page discloses nothing a port scan does not.
//!
//! # WHAT IS DELIBERATELY NOT HERE
//!
//! **The UI.** hv, on the day this landed: *your job is: can intentd serve
//! anything, it is ic's job to actually build the UX/UI for it.* The
//! derivations a UI needs are already built and are NOT re-walked here --
//! `intentsvcs::form::triples` emits the generic `{name, label, widget, value,
//! editable}` rows every renderer consumes, and `intentsvcs::nav::View` is the
//! path contract the TUI and the browser must share. **A web face that derived
//! its own would make `AC-17.12` unsatisfiable by construction**, which is the
//! reason `nav.rs` sits in the shared crate at all.

use std::sync::Arc;

use axum::Router;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::{IntoResponse, Response as HttpResponse};
use axum::routing::{get, post};
use intentsvcs::daemon::Token;

use crate::registry::Registry;

/// The turtle.
///
/// **THE PROJECT'S OWN MARK, EMBEDDED FROM THE ONE COPY THE REPOSITORY ALREADY
/// CARRIES** -- `docs/design/intent-logo.svg`, byte-identical to the one
/// `intent.laksa.io` serves. Not copied into this crate: a second copy is a
/// second thing to update when the mark changes, and the failure is the kind
/// nobody notices, because a stale logo still renders.
const TURTLE: &str = include_str!("../../../../../docs/design/intent-logo.svg");

/// The one static page (D56).
const SHELL: &str = include_str!("shell.html");

/// What the HTTP face needs to answer.
#[derive(Clone)]
pub struct Face {
  pub registry: Arc<Registry>,
  pub token: Arc<Token>,
}

/// Every route the HTTP face serves.
///
/// **A FUNCTION RETURNING THE ROUTER RATHER THAN A SERVER**, so the caller owns
/// the listener. `intentd` accepts on a port it bound and published itself, and
/// a `serve()` here would need that port a second time -- which is the address
/// with two homes that `Published` exists to make unexpressible.
pub fn router(face: Face) -> Router {
  Router::new()
    .route("/", get(shell))
    .route("/intent-logo.svg", get(turtle))
    .route("/op", post(op))
    .with_state(face)
}

async fn shell() -> impl IntoResponse {
  (
    [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
    SHELL.replace("{{VERSION}}", env!("CARGO_PKG_VERSION")),
  )
}

async fn turtle() -> impl IntoResponse {
  (
    [(header::CONTENT_TYPE, "image/svg+xml; charset=utf-8")],
    TURTLE,
  )
}

/// One request, answered by the SAME dispatch the socket answers.
///
/// **THE BINDING IS PER REQUEST HERE AND PER CONNECTION THERE, AND THAT IS
/// FAITHFUL RATHER THAN A RELAXATION.** `AC-08.1`'s per-connection binding
/// exists so a connection cannot WANDER between projects -- *the subject of a
/// response must not depend on history the client cannot see.* An HTTP request
/// names its project and is answered; there is no history for it to depend on,
/// so a fresh binding per request has the property the rule was written for.
/// **Sharing one binding across HTTP requests would be strictly worse**: two
/// browser tabs on two projects are one client by every measure this process
/// can take, and the second would be refused for the first's choice.
///
/// **A SUBSCRIPTION IS REFUSED RATHER THAN HALF-SERVED.** `Op::Subscribe` turns
/// a socket connection into a one-way feed, and a JSON response body has no
/// such mode. Answering it with the first event, or with an empty body, would
/// be a subscription that looks connected and delivers nothing -- so it is
/// declined by name, and the eventual browser mechanism (SSE or a websocket) is
/// a route rather than a reinterpretation of this one.
async fn op(State(face): State<Face>, headers: HeaderMap, body: String) -> HttpResponse {
  if !authorised(&face, &headers) {
    // **THE REFUSAL NAMES THE FILE AND NOT THE VALUE.** An operator who cannot
    // authenticate needs to know where their token lives; anybody else learns
    // only that a file exists whose contents they already cannot read.
    return json(
      StatusCode::UNAUTHORIZED,
      &intentsvcs::wire::Response::error(
        "this request carried no valid token".to_string(),
        "the HTTP face requires the secret intentd published at `~/.local/share/intent/intentd.token`, sent as `Authorization: Bearer <secret>`. The unix socket needs no token because filesystem permissions are its authorisation.".to_string(),
      ),
    );
  }

  let mut bound = None;
  let served = crate::dispatch(&face.registry, &mut bound, body.as_bytes()).await;
  let response = match served {
    crate::Served::Reply(response) => response,
    // Stopping over HTTP would let any page the browser is showing end the
    // daemon on a token it read from a link. The socket keeps the verb, where
    // the caller has already proved filesystem access.
    crate::Served::ReplyThenStop(_) => intentsvcs::wire::Response::error(
      "the HTTP face does not stop the daemon".to_string(),
      "use `intent daemon stop`, which asks over the unix socket -- reaching that socket already requires the filesystem permission this port cannot check.".to_string(),
    ),
    crate::Served::Feed { .. } => intentsvcs::wire::Response::error(
      "a subscription cannot be answered over this route".to_string(),
      "`Op::Subscribe` turns a connection into a one-way feed and an HTTP response body has no such mode. Subscribe over the unix socket.".to_string(),
    ),
  };

  json(StatusCode::OK, &response)
}

/// One [`intentsvcs::wire::Response`] as an HTTP body.
///
/// **THE BODY IS `wire::frame`'s BYTES, WHICH IS THE SAME FRAMING THE SOCKET
/// SENDS.** D56 gives the daemon ONE output contract over both transports, and
/// this is where that stops being a claim: an HTTP client and a socket client
/// receive the identical bytes for the identical answer, because one function
/// produced them. **A `serde_json::to_string` here would have been a second
/// serialiser** -- and `serde_json` is deliberately absent from this crate's
/// manifest for exactly that reason, in `wire.rs`'s own words: *a crate that
/// never names the format is a crate that cannot grow a second opinion about
/// it.*
///
/// **A RESPONSE THAT WILL NOT SERIALISE IS ANSWERED IN PLAIN TEXT, NEVER IN
/// HAND-WRITTEN JSON.** Composing `{"error": ...}` by hand at the one moment
/// the serialiser has failed is a second opinion about the format arriving
/// precisely where the first one broke; the client gets a 500 and a sentence,
/// and the daemon's log carries the rest. The socket path closes the
/// connection here for the same reason.
fn json(status: StatusCode, response: &intentsvcs::wire::Response) -> HttpResponse {
  match intentsvcs::wire::frame(response) {
    Ok(bytes) => (status, [(header::CONTENT_TYPE, "application/json")], bytes).into_response(),
    Err(_) => {
      eprintln!("warning: intentd could not serialise a response for the HTTP face");
      (
        StatusCode::INTERNAL_SERVER_ERROR,
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        "intentd could not serialise its own response\n",
      )
        .into_response()
    }
  }
}

/// Whether this request carried the run's secret.
///
/// **`Authorization: Bearer` RATHER THAN A QUERY PARAMETER**, because a query
/// string is the one place a secret reliably leaks: it lands in browser
/// history, in a `Referer` header on every outbound link, and in any proxy log
/// between here and nowhere. A header is not private either, but it does not
/// travel by itself.
fn authorised(face: &Face, headers: &HeaderMap) -> bool {
  headers
    .get(header::AUTHORIZATION)
    .and_then(|value| value.to_str().ok())
    .and_then(|value| value.strip_prefix("Bearer "))
    .is_some_and(|presented| face.token.admits(presented.trim()))
}

/// A [`axum::serve::Listener`] fed by the connection sorter rather than by a
/// socket of its own.
///
/// **THE POINT IS THAT THERE IS NO SECOND LISTENER**, and this type is what
/// makes one port serve two protocols without a second `accept` on it. The
/// daemon accepts once, decides at byte 0, and hands the HTTP ones here; axum
/// takes them exactly as it would take them from a `TcpListener`, because a
/// `TcpStream` is a `TcpStream` however it arrived.
///
/// **`local_addr` ANSWERS FROM THE PUBLISHED ENDPOINT, NEVER FROM A SOCKET
/// THIS TYPE DOES NOT HOLD.** It has no listener to ask, and inventing an
/// address would be a second answer to the question `Published` exists to have
/// exactly one answer to.
pub struct HandedOver {
  incoming: tokio::sync::mpsc::Receiver<tokio::net::TcpStream>,
  here: intentsvcs::daemon::Endpoint,
}

impl HandedOver {
  pub fn new(
    incoming: tokio::sync::mpsc::Receiver<tokio::net::TcpStream>,
    here: intentsvcs::daemon::Endpoint,
  ) -> Self {
    HandedOver { incoming, here }
  }
}

impl axum::serve::Listener for HandedOver {
  type Io = tokio::net::TcpStream;
  type Addr = std::net::SocketAddr;

  /// **A CLOSED CHANNEL PENDS FOREVER RATHER THAN RETURNING**, because the
  /// trait has nowhere to report *there will be no more connections*: its
  /// signature promises a connection. Pending is the honest encoding -- the
  /// task parks and dies with the runtime, which is when the daemon is
  /// stopping anyway. Returning a fabricated connection is the only
  /// alternative the signature allows and it would be a lie to a server.
  async fn accept(&mut self) -> (Self::Io, Self::Addr) {
    loop {
      match self.incoming.recv().await {
        Some(stream) => {
          // A peer address the kernel will not name does not cost us the
          // connection: axum wants an `Addr` for logging, and the request is
          // servable without one.
          let peer = stream
            .peer_addr()
            .unwrap_or_else(|_| std::net::SocketAddr::from(([127, 0, 0, 1], 0)));
          return (stream, peer);
        }
        None => std::future::pending::<()>().await,
      }
    }
  }

  fn local_addr(&self) -> std::io::Result<Self::Addr> {
    match self.here {
      intentsvcs::daemon::Endpoint::Tcp(addr) => Ok(addr),
      // The web face is only ever handed loopback connections, so this arm is
      // a routing fault inside this crate rather than a condition to handle.
      ref other => Err(std::io::Error::other(format!(
        "the HTTP face was published at {other}, which is not a loopback address"
      ))),
    }
  }
}
