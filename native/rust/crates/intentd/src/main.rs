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
use std::path::Path;
use std::process::ExitCode;
use std::time::Duration;

use intentsvcs::daemon::{self, Bound, DaemonError, Published};
use intentsvcs::remedy::Remedy;
use intentsvcs::userstate;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};

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
/// **THE PROPERTY IS BOUNDEDNESS, NOT THIS NUMBER.** A connection that opens
/// and never speaks holds a task and a descriptor, and a daemon whose task
/// count is set by other people's abandoned sockets is one `ulimit` away from
/// refusing the probe. Any finite value fixes that; this one is long enough
/// that no honest client on a loaded machine meets it.
const FIRST_LINE_DEADLINE: Duration = Duration::from_secs(5);

/// How much of a first line will be read before giving up on it.
///
/// The probe frame is nineteen bytes. This bound exists so that a client which
/// never sends a newline cannot make the daemon buffer without limit -- the
/// read stops, the task ends, and the connection closes.
const MAX_FIRST_LINE: u64 = 64 * 1024;

/// What a caller gets for anything that is not the probe.
///
/// D56: JSON only, over the socket AND over HTTP. It names no work package and
/// no criterion -- D37 keeps our own project-management state out of anything a
/// user can see.
const NO_REQUEST_API_YET: &[u8] =
  b"{\"error\":\"this intentd answers the liveness probe only and serves no request API yet\"}\n";

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

  println!(
    "intentd listening on {} and {}",
    bound.endpoint(),
    published.endpoint()
  );

  loop {
    tokio::select! {
      accepted = unix.accept() => match accepted {
        Ok((stream, _)) => { tokio::spawn(answer(stream)); }
        Err(e) => accept_failed("unix", e).await,
      },
      accepted = tcp.accept() => match accepted {
        Ok((stream, _)) => { tokio::spawn(answer(stream)); }
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

/// Answer one connection.
///
/// **THE PROBE IS ANSWERED BEFORE ANYTHING ELSE HAPPENS ON THIS CONNECTION, AND
/// THE ACCEPT LOOP IS NEVER THE THING WAITING** (`AC-08.11`). Each connection
/// gets its own task, so a slow or silent caller delays nobody: the loop above
/// returns to `accept` the instant it has a stream. That ordering is what makes
/// the client's bounded deadline sound -- a liveness answer that could queue
/// behind request work would turn `AC-08.3` into a false NEGATIVE on a healthy
/// daemon, and the CLI would then run in-process against a store this process
/// owns.
///
/// **THE STRONGER FORM OF THAT OBLIGATION IS STRUCTURAL AND IS NOT VISIBLE
/// HERE, WHICH IS WHY IT IS WRITTEN DOWN.** Per-connection tasks order things
/// WITHIN a connection; starvation happens BETWEEN them, when blocking work
/// occupies every async worker and the accept loop is a task that never gets
/// polled. Nothing in this function may ever call the facade directly: when the
/// domain API lands, its store access goes through a handle that does the
/// blocking hop internally, so a handler cannot reach a blocking call even by
/// accident.
async fn answer<S>(stream: S)
where
  S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
{
  let (readable, mut writable) = tokio::io::split(stream);
  let mut reader = BufReader::new(readable.take(MAX_FIRST_LINE));
  let mut line = Vec::new();

  let read = tokio::time::timeout(FIRST_LINE_DEADLINE, reader.read_until(b'\n', &mut line)).await;
  match read {
    Ok(Ok(count)) if count > 0 => {}
    // A silent, closed or over-long caller gets no reply and no log line. It is
    // an ordinary condition -- a port scanner, a dropped connection, a probe
    // whose client gave up -- and logging it would let anyone fill the log.
    _ => return,
  }

  let reply = if daemon::is_probe_frame(&line) {
    daemon::PROBE_REPLY
  } else {
    NO_REQUEST_API_YET
  };

  let _ = writable.write_all(reply).await;
  let _ = writable.flush().await;
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
