//! `intentd` -- one daemon per machine, serving N registered projects.
//!
//! **IT ALWAYS RUNS IN THE FOREGROUND, AND THAT IS THE DESIGN RATHER THAN A
//! STAGE IT IS AT.** Nothing here forks, detaches, writes a pidfile or
//! reparents itself: `launchd` does the backgrounding, and `intent daemon run`
//! runs this same binary attached to a terminal. A process that daemonises
//! itself has to reimplement supervision, restart and log redirection that the
//! platform already owns, and does it worse.
//!
//! **`intent daemon run` EXECS THIS BINARY RATHER THAN LINKING ITS BODY**, so
//! `AC-08.9`'s *identical code* is identity of BINARY, not two code paths that
//! agree. The rejected alternative -- the daemon body in `intentsvcs`, called
//! from both -- would put `tokio` and `axum` in the crate every CLI invocation
//! links, and would falsify the workspace manifest's rationale for confining
//! them here. What exec mints instead is a resolution question: an older
//! `intentd` on `PATH`, or a stale sibling from a previous build, serves
//! DIFFERENT code while every identity test stays green, because both faces
//! really are one binary -- just not the one that was meant. The refusal for
//! that lives on the CLI side, where the resolving happens.
//!
//! **WHAT THIS SERVES TODAY IS THE LIVENESS PROBE AND NOTHING ELSE.** The
//! registry, the domain API, watching and the web face are still ahead; a
//! request that is not the probe is refused BY NAME rather than answered with
//! an empty success, because a face that reads as working while returning
//! nothing is worse than one that is honestly absent.

use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::Arc;
use std::time::Duration;

use intentsvcs::daemon::{self, Bound, DaemonError, Published};
use intentsvcs::remedy::Remedy;
use intentsvcs::userstate;
use intentsvcs::wire::{self, Event, Op, Response};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};

mod registry;
mod store;
mod watch;

use registry::Registry;

// NO `SOURCE_COMMIT` CONST HERE, DELIBERATELY, AND THE ASYMMETRY WITH
// `intent-cli` IS THE POINT RATHER THAN AN OVERSIGHT. There it is `pub` in a
// lib, so it is real API something can read. `intentd` has no lib, so a const
// here is unreadable by anything, forever -- `dead_code` said so under
// `-D warnings` and it was right. Silencing that with `#[allow(dead_code)]`
// would have kept a declaration whose only purpose was to look symmetrical.
// The marker below is the whole artefact-facing contract; it is what every
// consumer greps, and `#[used]` is what makes it survive.

/// The string `int macos publish` and `self_provenance_check.sh` grep out of the
/// ARTEFACT.
///
/// SELF-DELIMITING, and that is not cosmetic. Rodata packs string literals with
/// no separator between them, so an unterminated marker runs straight into
/// whatever the linker laid down next -- measured during this row's canary as
/// `intent-source-commit:<sha>unsafe`, with `unsafe` belonging to an unrelated
/// literal. The fix belongs here in the artefact rather than in each consumer's
/// pattern, because hardening one grep only moves the trap to the next consumer.
///
/// `#[used]` because the whole point is that it survives into the binary even
/// though no code path reads it: a provenance marker the linker is free to drop
/// is one that vanishes under `--release`, which is the one build where it
/// matters. IT LIVES IN `main.rs` BECAUSE `intentd` HAS NO LIB, and a lib target
/// is deliberately NOT added to give a static a home -- that would reshape the
/// crate for the sake of where a marker lives (cc's call, and the right one:
/// `intent-cli` having a lib is incidental rather than the pattern).
#[used]
static SOURCE_COMMIT_MARKER: &str = env!("INTENT_SOURCE_COMMIT_MARKER");

/// How long a connection may stay silent before its task is dropped.
///
/// **IT BOUNDS THE WAIT FOR EVERY LINE, NOT JUST THE FIRST.** A connection now
/// serves many requests, so a client that connects, asks once and wanders off
/// would hold a task forever if only the opening read were bounded.
///
/// **THE PROPERTY IS BOUNDEDNESS, NOT THIS NUMBER.** A connection that opens
/// and never speaks holds a task and a descriptor, and a daemon whose task
/// count is set by other people's abandoned sockets is one `ulimit` away from
/// refusing the probe. Any finite value fixes that; this one is long enough
/// that no honest client on a loaded machine meets it.
const IDLE_DEADLINE: Duration = Duration::from_secs(30);

/// How much of one line will be read before giving up on it.
///
/// The probe frame is nineteen bytes. This bound exists so that a client which
/// never sends a newline cannot make the daemon buffer without limit -- the
/// read stops, the task ends, and the connection closes. **It is reset for
/// every line rather than spent across the connection**, because a connection
/// serves many requests and a budget shared between them would refuse an honest
/// client for the sin of having asked a lot of questions.
const MAX_LINE: u64 = 64 * 1024;

/// Why the daemon could not start.
///
/// **DEFINED HERE RATHER THAN ADDED TO `DaemonError`, WHICH IS THE CLIENT'S
/// TYPE.** Every CLI invocation matches on that enum to decide where to route;
/// widening it with a variant only a daemon can produce would give every
/// consumer an arm that is unreachable for them, and adding variants to an enum
/// consumers match loosely is a change that breaks nothing and means something.
#[derive(Debug)]
enum StartupError {
  /// There is no per-user state directory to bind under.
  NoUserState(userstate::UserStateError),
  /// The socket, the lock or the address file refused.
  Address(DaemonError),
  /// A listener could not be handed to the async runtime.
  Runtime(io::Error),
}

impl std::fmt::Display for StartupError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      StartupError::NoUserState(e) => write!(f, "intentd has no per-user state directory: {e}"),
      StartupError::Address(e) => write!(f, "{e}"),
      StartupError::Runtime(e) => write!(
        f,
        "intentd bound its listeners and could not serve them: {e}"
      ),
    }
  }
}

impl StartupError {
  fn remedy(&self) -> String {
    match self {
      StartupError::NoUserState(_) => {
        "intentd keeps its socket and address file under your per-user state directory, which is derived from $HOME. Run it as a user with a home directory.".to_string()
      }
      StartupError::Address(e) => e.remedy(),
      StartupError::Runtime(_) => {
        "this is an operating-system level failure on a socket that had already bound. Check the process descriptor limit with `ulimit -n`.".to_string()
      }
    }
  }
}

#[tokio::main]
async fn main() -> ExitCode {
  // **NO PROJECT-MANAGEMENT STATE IN SHIPPED OUTPUT** (D37). What a user needs
  // from `--version` is the version; which of our work packages finished the
  // daemon is our business, and it stays in the module note above.
  if std::env::args().any(|arg| arg == "--version" || arg == "-V") {
    println!("intentd {}", env!("CARGO_PKG_VERSION"));
    return ExitCode::SUCCESS;
  }

  // **AN ARGUMENT THIS BINARY DOES NOT UNDERSTAND MUST NOT START A DAEMON, AND
  // THIS COST A LIVE INCIDENT ON THE DEVELOPER'S OWN MACHINE.**
  //
  // Before this, `intentd` inspected argv for `--version` and then served
  // REGARDLESS of what else was there. So `intentd --help` -- which is what
  // anybody types first, and what a peer typed while diagnosing something
  // unrelated -- started a real daemon under the real `$HOME`. It bound, it
  // published, and for three minutes every session on the machine had its store
  // verbs refused at rc=2 by a daemon nobody meant to start.
  //
  // **THE FIXTURES WERE GUARDED AND THE FRONT DOOR WAS NOT.** Every test in this
  // estate goes to some trouble to give the daemon an isolated `HOME`, precisely
  // because a daemon on the real one takes four sessions down together -- and
  // the thing that actually did it was a person typing `--help`. **A guard on
  // the path you expected the danger to arrive by is not a guard on the danger.**
  //
  // Serving is what this binary does with NO arguments, so anything else is a
  // request it cannot honour, and starting anyway is answering a question that
  // was not asked.
  let unknown: Vec<String> = std::env::args().skip(1).collect();
  if !unknown.is_empty() {
    let wants_help = unknown.iter().any(|a| a == "--help" || a == "-h");
    if wants_help {
      println!("intentd -- the Intent daemon. Serves this machine's open projects.");
      println!();
      println!("Usage: intentd            serve until signalled (SIGTERM, SIGINT)");
      println!("       intentd --version  print the version and exit");
      println!("       intentd --help     print this and exit");
      println!();
      println!(
        "It takes no other arguments and no subcommands. `intent daemon run` execs this binary."
      );
      return ExitCode::SUCCESS;
    }
    eprintln!(
      "error: intentd takes no arguments and was given {}",
      unknown.join(" ")
    );
    eprintln!(
      "  remedy: run `intentd` with nothing after it to serve, or `intentd --help`. It is deliberate that an unrecognised argument does NOT fall through to serving: starting a daemon on this machine's real HOME makes every session's store verbs refuse, and that is not a thing to do by accident."
    );
    return ExitCode::FAILURE;
  }

  let root = match userstate::home() {
    Ok(root) => root,
    Err(e) => return refuse(StartupError::NoUserState(e)),
  };

  match serve_under(&root).await {
    Ok(()) => ExitCode::SUCCESS,
    Err(e) => refuse(e),
  }
}

fn refuse(e: StartupError) -> ExitCode {
  eprintln!("error: {e}");
  eprintln!("  remedy: {}", e.remedy());
  ExitCode::FAILURE
}

/// Bind, publish, and serve until a signal arrives.
///
/// **THE TWO GUARDS ARE HELD FOR THE WHOLE FUNCTION AND THAT IS WHAT MAKES THE
/// CLEANUP REAL.** `Bound` holds the kernel lock that makes this the only
/// daemon (`AC-08.12`) and unlinks the socket on drop; `Published` removes the
/// address file on drop. Both run on the error paths and on unwind, which is
/// the whole reason they are guards rather than a tidy-up at the end -- cleanup
/// written after the serving loop is dead code until the day something returns
/// early, and on that day it does not run.
async fn serve_under(root: &Path) -> Result<(), StartupError> {
  let (unix, bound) = Bound::bind_socket_under(root).map_err(StartupError::Address)?;
  let (tcp, published) = Published::bind_loopback_under(root).map_err(StartupError::Address)?;

  unix.set_nonblocking(true).map_err(StartupError::Runtime)?;
  tcp.set_nonblocking(true).map_err(StartupError::Runtime)?;
  let unix = tokio::net::UnixListener::from_std(unix).map_err(StartupError::Runtime)?;
  let tcp = tokio::net::TcpListener::from_std(tcp).map_err(StartupError::Runtime)?;

  // **ONE REGISTRY FOR THE PROCESS, SHARED BY EVERY CONNECTION.** A registry
  // per connection would open a second store for every client of one project,
  // which is the two-engines failure arrived at from inside the daemon meant to
  // prevent it.
  let registry = Arc::new(Registry::new());

  println!(
    "intentd listening on {} and {}",
    bound.endpoint(),
    published.endpoint()
  );

  loop {
    tokio::select! {
      accepted = unix.accept() => match accepted {
        Ok((stream, _)) => { tokio::spawn(answer(stream, Arc::clone(&registry))); }
        Err(e) => accept_failed("unix", e).await,
      },
      accepted = tcp.accept() => match accepted {
        Ok((stream, _)) => { tokio::spawn(answer(stream, Arc::clone(&registry))); }
        Err(e) => accept_failed("loopback", e).await,
      },
      reason = shutdown() => {
        println!("intentd stopping: {reason}");
        break;
      }
    }
  }

  Ok(())
}

/// **AN ACCEPT FAILURE IS REPORTED AND SURVIVED, WITH A PAUSE THAT IS ABOUT THE
/// LOOP RATHER THAN THE ERROR.** Descriptor exhaustion is transient and common
/// on a busy machine, so dying would turn a passing condition into an outage.
/// But `accept` failing immediately and repeatedly would spin a core at full
/// tilt while logging, so the loop yields for long enough that the log is
/// readable and the machine is usable.
async fn accept_failed(transport: &str, e: io::Error) {
  eprintln!("warning: intentd could not accept a {transport} connection: {e}");
  tokio::time::sleep(Duration::from_millis(100)).await;
}

/// Answer one connection: the probe, then requests, until it goes quiet.
///
/// **THE PROBE IS ANSWERED BEFORE ANYTHING ELSE HAPPENS ON THIS CONNECTION, AND
/// THE ACCEPT LOOP IS NEVER THE THING WAITING** (`AC-08.11`). Each connection
/// gets its own task, so a slow or silent caller delays nobody: the accept loop
/// returns to `accept` the instant it has a stream. That ordering is what makes
/// the client's bounded deadline sound -- a liveness answer that could queue
/// behind request work would turn `AC-08.3` into a false NEGATIVE on a healthy
/// daemon, and the CLI would then run in-process against a store this process
/// owns.
///
/// **AND THE STRONGER FORM IS HELD BY THE TYPE THIS FUNCTION IS GIVEN.** Per-
/// connection tasks order things WITHIN a connection; starvation happens
/// BETWEEN them, when blocking work occupies every async worker and the accept
/// loop is a task that never gets polled. Nothing here can cause that, because
/// nothing here is handed anything that blocks: a [`Registry`] yields a
/// `ProjectHandle`, which is a channel sender. The `Facade` lives on a thread
/// the runtime does not schedule and never leaves `store.rs`.
///
/// **A PROBE CONNECTION IS ONE-SHOT AND A REQUEST CONNECTION IS NOT.** The
/// client opens a fresh connection to probe and never reuses it, so answering
/// and closing is what its caller expects; a request connection stays open so a
/// client can ask more than one question without paying for a connect each time.
async fn answer<S>(stream: S, registry: Arc<Registry>)
where
  S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
{
  let (readable, mut writable) = tokio::io::split(stream);
  let mut reader = BufReader::new(readable.take(MAX_LINE));

  // **THE CONNECTION'S BINDING, WHICH IS `AC-08.1`'s *PER-CONNECTION* HALF.**
  // It is set by the first request that names a project and never changes: a
  // later request naming a different one is refused rather than served, because
  // a connection that could wander between projects makes every response's
  // subject depend on history the client cannot see.
  let mut bound: Option<PathBuf> = None;

  loop {
    // Reset per line: the budget is against one over-long request, not against
    // a client that asks many questions.
    reader.get_mut().set_limit(MAX_LINE);
    let mut line = Vec::new();
    let read = tokio::time::timeout(IDLE_DEADLINE, reader.read_until(b'\n', &mut line)).await;
    match read {
      Ok(Ok(count)) if count > 0 => {}
      // A silent, closed or over-long caller gets no reply and no log line. It
      // is an ordinary condition -- a port scanner, a dropped connection, a
      // client that finished -- and logging it would let anyone fill the log.
      _ => return,
    }

    if daemon::is_probe_frame(&line) {
      let _ = writable.write_all(daemon::PROBE_REPLY).await;
      let _ = writable.flush().await;
      return;
    }

    let (response, feed) = match dispatch(&registry, &mut bound, &line).await {
      Served::Reply(response) => (response, None),
      Served::Feed { project_id, events } => (Response::Subscribed { project_id }, Some(events)),
    };
    // A response that cannot be serialised is a fault in the daemon, not in the
    // request, and there is no honest way to report it in a format the client
    // parses -- so the connection closes rather than sending something the
    // client would read as an answer.
    let Ok(framed) = wire::frame(&response) else {
      eprintln!("warning: intentd could not serialise a response and closed the connection");
      return;
    };
    if writable.write_all(&framed).await.is_err() || writable.flush().await.is_err() {
      return;
    }

    // **A SUBSCRIPTION ENDS THE QUESTION-AND-ANSWER LOOP RATHER THAN JOINING
    // IT** (`AC-08.6`). From here the daemon writes and never reads, so a
    // client that kept asking would be talking to nobody -- which is why
    // `Op::Subscribe` says so on the request type rather than only here.
    if let Some(events) = feed {
      deliver(events, &mut writable).await;
      return;
    }
  }
}

/// What one request turned into.
///
/// **TWO SHAPES BECAUSE A SUBSCRIPTION IS NOT AN ANSWER**, and collapsing them
/// -- returning a `Response` and having the caller sniff it for a subscribed
/// variant -- would make the connection's MODE a property of a value's contents
/// rather than of the routing decision that produced it.
enum Served {
  Reply(Response),
  Feed {
    project_id: String,
    events: tokio::sync::broadcast::Receiver<Event>,
  },
}

/// Write events to a subscriber until one side goes away (`AC-08.6`).
///
/// **A SUBSCRIBER THAT FELL BEHIND IS DISCONNECTED, NEVER QUIETLY SKIPPED**
/// (`IN-AG-NO-SILENT-001`). `broadcast` reports an overrun as
/// `RecvError::Lagged(n)` and the tempting arm is to log it and carry on --
/// which hands the client a feed with a HOLE in it that is indistinguishable
/// from a feed without one. **A subscription that ENDED is recoverable: the
/// client reconnects and re-reads. One that silently skipped is not**, because
/// nothing downstream ever learns which state it is missing.
async fn deliver<W>(mut events: tokio::sync::broadcast::Receiver<Event>, writable: &mut W)
where
  W: tokio::io::AsyncWrite + Unpin,
{
  loop {
    let event = match events.recv().await {
      Ok(event) => event,
      // The project's feed is gone: the daemon is shutting down, or the handle
      // was dropped. Either way there is nothing further to send.
      Err(tokio::sync::broadcast::error::RecvError::Closed) => return,
      Err(tokio::sync::broadcast::error::RecvError::Lagged(missed)) => {
        eprintln!(
          "intentd: a subscriber fell {missed} event(s) behind and was disconnected\n  remedy: this is backpressure rather than a fault. The client should reconnect and re-read the project, because the feed it had is now missing events it cannot enumerate."
        );
        return;
      }
    };
    let Ok(framed) = wire::frame(&event) else {
      eprintln!("warning: intentd could not serialise an event and closed the subscription");
      return;
    };
    if writable.write_all(&framed).await.is_err() || writable.flush().await.is_err() {
      return;
    }
  }
}

/// Route one request: the registry answers for itself, everything else needs a
/// project.
async fn dispatch(registry: &Registry, bound: &mut Option<PathBuf>, line: &[u8]) -> Served {
  let request = match wire::parse_request(line) {
    Ok(request) => request,
    Err(refusal) => return Served::Reply(refusal),
  };

  // **THE REGISTRY IS ANSWERED WITHOUT BINDING TO A PROJECT**, because it is a
  // question ABOUT the projects rather than one for any of them -- and because
  // the operator most likely to ask it is the one whose project stopped
  // resolving, which is exactly when binding to it would fail.
  if matches!(request.op, Op::Registry) {
    return Served::Reply(registry.snapshot().await);
  }

  // **THE BINDING IS CHECKED BEFORE ANYTHING IS OPENED.** Canonicalising is a
  // pure question about a path; opening starts a store thread and registers a
  // project. Asking for the handle first and comparing afterwards would refuse
  // the request accurately, having already done the thing the refusal exists to
  // prevent -- which is a report rather than a check.
  let canonical = match registry.canonical(&request.root) {
    Ok(canonical) => canonical,
    Err(refusal) => return Served::Reply(refusal),
  };

  match bound {
    None => *bound = Some(canonical.clone()),
    Some(already) if *already != canonical => {
      return Served::Reply(Response::error(
        format!(
          "this connection is bound to `{}` and the request names `{}`",
          already.display(),
          canonical.display()
        ),
        "one connection serves one project. Open a second connection for the other project -- a connection that changed project mid-stream would make every answer depend on which request came first.",
      ));
    }
    Some(_) => {}
  }

  // **THE SUBSCRIPTION IS ROUTED AFTER THE BINDING CHECK AND BEFORE THE STORE**,
  // which is exactly where it belongs: it names a project, so it binds the
  // connection like any other request -- and it never reaches a store, so it is
  // in `wire::UNCOUNTED` and must not go through `handle.call`, which counts.
  if matches!(request.op, Op::Subscribe) {
    return match registry.feed_for(&canonical).await {
      Ok((project_id, events)) => Served::Feed { project_id, events },
      Err(refusal) => Served::Reply(refusal),
    };
  }

  let handle = match registry.handle_for(&canonical).await {
    Ok(handle) => handle,
    Err(refusal) => return Served::Reply(refusal),
  };

  Served::Reply(handle.call(request.op).await)
}

/// Resolve when the platform asks this process to stop.
///
/// `SIGTERM` is what `launchd` sends and what `intent daemon stop` will send;
/// `SIGINT` is what a terminal sends to `intent daemon run`. Both unwind
/// normally so the guards run -- which is the difference between a restart that
/// works and one that meets a stale socket. `SIGKILL` runs no destructor and is
/// designed for on the reading side instead: the client's probe never trusts a
/// published address.
async fn shutdown() -> &'static str {
  use tokio::signal::unix::{SignalKind, signal};

  let mut term = match signal(SignalKind::terminate()) {
    Ok(s) => s,
    // **A DAEMON THAT CANNOT HEAR `SIGTERM` MUST NOT PRETEND IT CAN.** Failing
    // to install the handler leaves the default disposition, which kills the
    // process without unwinding -- so the honest thing is to say so once, at
    // start, rather than to look like a clean shutdown that never comes.
    Err(e) => {
      eprintln!(
        "warning: intentd could not listen for SIGTERM and will not shut down cleanly: {e}"
      );
      std::future::pending::<()>().await;
      unreachable!("pending never resolves");
    }
  };
  let mut interrupt = match signal(SignalKind::interrupt()) {
    Ok(s) => s,
    Err(e) => {
      eprintln!("warning: intentd could not listen for SIGINT: {e}");
      term.recv().await;
      return "SIGTERM";
    }
  };

  tokio::select! {
    _ = term.recv() => "SIGTERM",
    _ = interrupt.recv() => "SIGINT",
  }
}
