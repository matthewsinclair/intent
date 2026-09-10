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
use intentsvcs::launchagent;
use intentsvcs::remedy::Remedy;
use intentsvcs::userstate;
use intentsvcs::wire::{self, Event, Op, Response};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};

mod registry;
mod store;
mod watch;
mod web;

use registry::Registry;

// **THIS CONST'S ABSENCE WAS CORRECT UNTIL A READER EXISTED, AND ONE NOW DOES.**
//
// The reasoning it replaces stands on its own terms and is kept, because the
// reason it expired is more useful than the conclusion: *`intentd` has no lib,
// so a const here is unreadable by anything, FOREVER -- `dead_code` said so
// under `-D warnings` and it was right.* Every word of that was true when
// written. **The `forever` is what expired**: the shell page now reports its
// own build, so `web.rs` reads this and `dead_code` is satisfied by a consumer
// rather than by an `#[allow]`.
//
// That is the class this estate keeps meeting from the other side -- a stated
// blocker whose REASON expires while its VERDICT still reads as current. The
// verdict here was "no const"; the reason was "nothing can read it"; only the
// reason was ever load-bearing.
//
// The marker below remains the artefact-facing contract and is NOT replaced by
// this: a const is readable by code in this binary, and the marker is readable
// by a tool with only the file. Two readers, two mechanisms, one value.
pub(crate) const SOURCE_COMMIT: &str = env!("INTENT_SOURCE_COMMIT");

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
/// The artefact's own VERSION, embedded for the same reason as the commit
/// beside it: a reader holding a SIBLING binary can otherwise only substitute
/// its own `CARGO_PKG_VERSION`, which is a claim about the reader. `intent` and
/// `intentd` are separately-built artefacts that have been measured
/// forty-two hours apart, so a surface naming both must read both.
///
/// A SEPARATE MARKER rather than a wider commit one -- five parsers capture
/// `[intent-source-commit:...]` with `[^]]*`, and widening it would change what
/// every one of them captures.
#[used]
static SOURCE_VERSION_MARKER: &str = env!("INTENT_SOURCE_VERSION_MARKER");

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
  //
  // **THE COMMIT IS PART OF THE VERSION, NOT PROJECT-MANAGEMENT STATE, AND
  // `intent` HAS SAID SO SINCE THE `corrected` RATIFICATION** (hv 2026-08-14).
  // Its reasoning applies here unchanged: every binary in this estate reports
  // the same `CARGO_PKG_VERSION`, so the version answers WHICH LINE and only
  // the commit answers WHICH BUILD. `SOURCE_COMMIT` carries its own dirt inside
  // the value -- `dirty-<sha>`, or `unknown` when git could not answer -- so
  // this cannot report a dirty build as a clean one.
  //
  // **IT WAS EMBEDDED AND UNPRINTED, WHICH IS THE WORST OF THE THREE STATES.**
  // `build.rs` has embedded it all along and the pre-commit self-provenance arm
  // reads it out with `strings`, so the datum existed and only the operator
  // could not reach it: `intent --version` named its build and `intentd
  // --version` did not. **This binary needs it MORE than its sibling by its own
  // build script's argument** -- it is the one measured forty-two hours older
  // than the commit it was recorded under, and an operator diagnosing a daemon
  // asks the daemon.
  if std::env::args().any(|arg| arg == "--version" || arg == "-V") {
    println!("intentd {} ({})", env!("CARGO_PKG_VERSION"), SOURCE_COMMIT);
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

  // **ASKED BEFORE ANYTHING IS BOUND, AND BEFORE ANYTHING COULD HAVE CONSUMED
  // STDIN.** The answer is a property of the descriptor this process was handed,
  // so it is read once, at the top, rather than re-derived somewhere that a
  // reader has already advanced.
  let lifeline = Lifeline::observed();

  let root = match userstate::home() {
    Ok(root) => root,
    Err(e) => return refuse(StartupError::NoUserState(e)),
  };

  match serve_under(&root, lifeline).await {
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
async fn serve_under(root: &Path, lifeline: Lifeline) -> Result<(), StartupError> {
  let (unix, bound) = Bound::bind_socket_under(root).map_err(StartupError::Address)?;

  // **THE TOKEN IS MINTED BEFORE THE PORT IS PUBLISHED, AND THE ORDER IS D6's
  // ARGUMENT RATHER THAN TIDINESS** (vc's condition, 2026-08-30). Publishing an
  // address whose token is not yet readable advertises an endpoint nobody can
  // legitimately use -- the same bad state `Published` makes unexpressible for
  // the address itself, arriving one file later. `?` here means a daemon that
  // cannot write its secret does not come up: the alternative is a published
  // port whose only protection failed silently.
  let token = Arc::new(intentsvcs::daemon::Token::mint_under(root).map_err(StartupError::Address)?);

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

  // **ONE HANDLE, CLONED PER CONNECTION, AND IT IS THE ONLY WAY A REQUEST CAN
  // STOP THIS PROCESS.** A connection cannot reach the accept loop directly --
  // it runs in its own task -- so `Op::Shutdown` needs a channel back. `Notify`
  // rather than a flag the loop polls: a flag would only be noticed on the next
  // accept, so a daemon nobody else connects to would sit there having agreed
  // to stop.
  heal_the_policy_stamp();

  let stop = Arc::new(tokio::sync::Notify::new());

  tokio::spawn(sweep_backups(Arc::clone(&registry)));

  // **ONE PUBLISHED PORT SERVING BOTH PROTOCOLS** (`AC-08.9`, vc's ruling
  // 2026-08-30). The HTTP half is a `Router` fed by a channel rather than a
  // second listener, because the alternative readings both lose: giving the
  // port to HTTP alone strands the TCP entry's whole purpose -- `candidates`
  // puts unix FIRST and appends TCP, so it is the socket's UNDERSTUDY, reached
  // exactly when the socket is what is broken -- and a second published port
  // reintroduces *which port* one level up, which D6's bind-and-publish exists
  // to make unaskable.
  let (http_tx, http_rx) = tokio::sync::mpsc::channel::<tokio::net::TcpStream>(HTTP_BACKLOG);
  {
    let face = web::Face {
      registry: Arc::clone(&registry),
      token: Arc::clone(&token),
    };
    let here = published.endpoint();
    tokio::spawn(async move {
      if let Err(e) = axum::serve(web::HandedOver::new(http_rx, here), web::router(face)).await {
        eprintln!(
          "intentd: the HTTP face stopped answering: {e}\n  remedy: framed clients on this port and the unix socket are unaffected. Restart the daemon to bring the web face back."
        );
      }
    });
  }

  // Built once and pinned: see `Lifeline::closed`.
  let lifeline_closed = lifeline.closed();
  tokio::pin!(lifeline_closed);

  // Built once and pinned for the same reason the lifeline is.
  let state_dir_gone = state_dir_removed(userstate::daemon_state_dir_under(root));
  tokio::pin!(state_dir_gone);

  loop {
    tokio::select! {
      accepted = unix.accept() => match accepted {
        Ok((stream, _)) => { tokio::spawn(answer(stream, Arc::clone(&registry), Arc::clone(&stop))); }
        Err(e) => accept_failed("unix", e).await,
      },
      accepted = tcp.accept() => match accepted {
        // **SORTED IN ITS OWN TASK, NEVER IN THIS ARM.** Deciding which protocol
        // a connection speaks means reading a byte from it, and a client that
        // connects and sends nothing would hold this loop -- so every other
        // project's first contact, the unix socket and the shutdown arms would
        // all wait on one silent socket. That is a whole-daemon stall reachable
        // by `nc` and a newline nobody types.
        Ok((stream, _)) => {
          tokio::spawn(sort_by_first_byte(
            stream,
            Arc::clone(&registry),
            Arc::clone(&stop),
            http_tx.clone(),
          ));
        }
        Err(e) => accept_failed("loopback", e).await,
      },
      reason = shutdown() => {
        println!("intentd stopping: {reason}");
        break;
      }
      // **THE SAME EXIT AS A SIGNAL, DELIBERATELY.** It breaks the same loop
      // and unwinds through the same guards, so `Op::Shutdown` inherits every
      // property `SIGTERM` already has -- the socket is unlinked, the lock is
      // released -- rather than becoming a second way to stop that has to be
      // kept in step with the first.
      () = stop.notified() => {
        println!("intentd stopping: asked over the wire");
        break;
      }
      // **THE SAME EXIT AS A SIGNAL, FOR THE SAME REASON THE ARM ABOVE IS.**
      // It breaks this loop and unwinds through `Bound` and `Published`, so an
      // owner's death releases the lock and unlinks the socket exactly as
      // `SIGTERM` does. A lifeline that called `process::exit` would leave the
      // stale socket that the whole guard arrangement exists to prevent.
      reason = &mut lifeline_closed => {
        println!("intentd stopping: {reason}");
        break;
      }
      // **THE SAME EXIT AS A SIGNAL, FOR THE THIRD TIME AND FOR THE SAME
      // REASON** (`ST0073` `AC-02.1`). It breaks the loop and unwinds through
      // `Bound` and `Published`, so a removed state directory releases the lock
      // and unlinks whatever is left of the socket exactly as `SIGTERM` does.
      reason = &mut state_dir_gone => {
        println!("intentd stopping: {reason}");
        break;
      }
    }
  }

  Ok(())
}

/// Resolve when the daemon's own state directory is gone.
///
/// # What this is for, and it is NOT the leak
///
/// **DRIVEN BEFORE IT WAS BUILT, because a row whose subject cannot be
/// constructed is decoration.** A daemon whose home is removed under it keeps
/// running: measured 2026-09-10 with the positive control the row demands --
/// `daemon status` answering immediately before the removal, the process still
/// alive 8s after it, and its owner's lifeline still held throughout so the
/// survival is attributable to the directory and not to the pipe.
///
/// **AND THE SHARPER STATEMENT IS THAT IT CANNOT SERVE AND DOES NOT KNOW.** The
/// socket lives INSIDE that tree, so after removal the daemon holds a listening
/// socket whose path no longer exists: alive, holding the store, unreachable to
/// any new client because there is nothing left to connect to. That is the
/// orphaned-listener shape `AC-01.6` and `AC-08.3` already name, reached from a
/// direction neither names -- an unlinked path under a live listener rather
/// than an inherited descriptor.
///
/// **SO THIS ARM IS ABOUT CONTENTION, NOT SURVIVAL.** The lifeline already
/// stops these outliving their run. What it cannot do is stop one accumulating
/// DURING a run, because the owner is alive and the lifeline fires on the
/// owner's death. An unreachable daemon is still a writer on the store, and
/// concurrent writers are `0216`'s reproduced variable.
///
/// # Why this one is an interval when the lifeline next door refuses to be
///
/// [`Lifeline::closed`] argues against polling in terms that apply here too --
/// *a poll leaves a window proportional to its period* -- and the two sitting
/// side by side with no explanation is how the next reader concludes the
/// lifeline could have been polled too. **It could not, and the difference is
/// that the lifeline HAD AN EXACT ALTERNATIVE and this has none.**
///
/// A pipe delivers EOF from the kernel on a descriptor that cannot be recycled
/// while it is open, so the lifeline's event is raceless and free. **A
/// directory has no such primitive.** `notify` is the alternative and it
/// REFUSES A PATH THAT DOES NOT EXIST (`watch.rs`, which already records this
/// for the `intent/` case) -- so a watcher must be re-registered to notice the
/// very event it exists for, which is an interval wearing a watcher's name. It
/// would also put a SECOND watcher in a process that already runs a debouncer.
///
/// **The honest trade is a stated bound against a silent miss, and a stated
/// bound wins.** A missed filesystem event is invisible; a period is written
/// here, testable, and small against what it bounds -- one tick versus the
/// remaining minutes of a test run.
///
/// # Two consecutive observations, not one
///
/// **THE ASYMMETRY IS THE ONE `Lifeline::observed` ALREADY MAKES.** Exiting
/// wrongly stops `launchd`'s daemon until the next login, because the plist is
/// `KeepAlive false` with no socket activation; staying wrongly leaves one
/// orphan that the next tick collects. Those errors are not the same size, so a
/// single unlucky `stat` must not be able to stop a production daemon.
async fn state_dir_removed(dir: PathBuf) -> &'static str {
  const PERIOD: Duration = Duration::from_secs(2);
  let mut misses = 0u8;
  loop {
    tokio::time::sleep(PERIOD).await;
    if dir.is_dir() {
      misses = 0;
    } else {
      misses += 1;
      if misses >= 2 {
        return "its state directory was removed";
      }
    }
  }
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
/// How often the backup sweep comes round.
///
/// **A FLOOR ON LATENESS, NOT A PERIOD** (`AC-08.8`). Nothing about a backup's
/// schedule is decided here: this is only how often each project is ASKED, and
/// the answer comes from `intentsvcs::backup::due` reading that project's own
/// configured period against the age of its newest good snapshot. So the
/// number's whole meaning is *how late a due backup can be*, and against the
/// finest schedule the vocabulary offers -- hourly -- five minutes is under a
/// tenth of a period.
///
/// **IT IS NOT SMALLER BECAUSE A SWEEP IS NOT FREE AND NOT LARGER BECAUSE
/// LATENESS COMPOUNDS.** Each round sends one message to every open project's
/// store thread, and each costs one indexed query; the cost is trivial and it
/// is paid whether or not anything is due, which is the argument against one
/// second. Against one hour, an hourly schedule would drift by up to an hour,
/// which is a second period.
const BACKUP_SWEEP: std::time::Duration = std::time::Duration::from_secs(300);

/// Ask every open project whether it is due a backup (`AC-08.8`).
///
/// **ONE SWEEP FOR THE DAEMON RATHER THAN A TIMER PER PROJECT, AND THE REASON
/// IS A LIFETIME THAT WOULD LIE.** A per-project timer would naturally be held
/// beside `Registered.watch`, where it would LOOK like the watch and behave
/// oppositely: dropping a `Watch` stops its thread, and dropping a
/// `JoinHandle` does not stop its task. A project removed from the registry
/// would go on backing itself up, from a handle nothing holds -- and the field
/// would read, to anybody maintaining it, as though that had been handled.
///
/// **AND THE SWEEP PICKS UP PROJECTS REGISTERED AFTER IT STARTED, FOR FREE.**
/// It re-reads the registry every round rather than capturing a list, so a
/// project opened on first contact five minutes ago is considered on the next
/// pass with nothing to wire.
///
/// **IT ENDS WHEN THE PROCESS DOES AND NEEDS NO SHUTDOWN PATH.** The accept
/// loop breaking drops the runtime, which drops this task mid-sleep; there is
/// no in-flight state a stop could protect, because the decision and the work
/// both happen on the store thread rather than here.
async fn sweep_backups(registry: Arc<Registry>) {
  let mut sweep = tokio::time::interval(BACKUP_SWEEP);
  // **A MISSED TICK IS SKIPPED, NEVER MADE UP.** The default behaviour bursts
  // to catch up after the runtime has been busy, which for this task would
  // mean several sweeps back to back -- and since the due decision is read
  // from the store each time, every one after the first would answer `NotYet`.
  // Work with no effect, at the moment the machine is already loaded.
  sweep.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
  loop {
    sweep.tick().await;
    for handle in registry.handles().await {
      handle.consider_backup().await;
    }
  }
}

/// How many sorted HTTP connections may wait for the web face.
///
/// **A BOUND, BECAUSE AN UNBOUNDED CHANNEL TURNS A SLOW PAGE INTO MEMORY
/// GROWTH.** The sorting tasks are what fill it and they are cheap to keep
/// waiting: a full channel parks one connection's task, not the accept loop,
/// so the back-pressure lands on the client that caused it.
const HTTP_BACKLOG: usize = 64;

/// How long a freshly accepted TCP client has to say what it is.
///
/// **A CONNECTION THAT SAYS NOTHING IS DROPPED RATHER THAN SORTED**, because
/// there is nothing to sort it by. It is an ordinary condition -- a port
/// scanner, a health check, a dropped connection -- so it costs a task and a
/// timeout and produces no log line, exactly as the framed reader already
/// treats a silent caller.
const SORT_DEADLINE: std::time::Duration = std::time::Duration::from_secs(5);

/// Send this connection to the protocol it is speaking (`AC-08.9`).
///
/// **THE DISCRIMINATOR IS BYTE 0 AND IT IS EXACT RATHER THAN A HEURISTIC.**
/// `wire::frame` is `serde_json::to_vec` plus a newline, so **every framed
/// request on this port begins with `{`** -- the liveness probe included, since
/// `PROBE_FRAME` is `{"intent_probe":1}`. **Every HTTP request begins with a
/// method token**, which is ASCII letters, and a browser cannot reach the other
/// branch because `fetch` cannot help sending a method first. One byte, no
/// lookahead, no ambiguity in either direction.
///
/// **IT EXTENDS A RECOGNISER RATHER THAN INTRODUCING THE IDEA.** This listener
/// already reads content before deciding what to do with it: `is_probe_frame`
/// has always branched on the bytes that arrived.
///
/// **`peek` RATHER THAN `read`, WHICH IS WHY THE FRAME BRANCH IS UNTOUCHED.**
/// The kernel keeps the byte, so `answer` receives a stream that has had
/// nothing taken from it and needs no way to be handed a prefix back. A `read`
/// here would have made every framed path carry a pushback buffer for a
/// decision it does not participate in.
async fn sort_by_first_byte(
  stream: tokio::net::TcpStream,
  registry: Arc<Registry>,
  stop: Arc<tokio::sync::Notify>,
  http: tokio::sync::mpsc::Sender<tokio::net::TcpStream>,
) {
  let mut first = [0u8; 1];
  let peeked = tokio::time::timeout(SORT_DEADLINE, stream.peek(&mut first)).await;
  match peeked {
    Ok(Ok(1)) => {}
    // Silent, closed, or slower than the deadline. No reply and no log line.
    _ => return,
  }

  if first[0] == b'{' {
    answer(stream, registry, stop).await;
    return;
  }

  // **A FULL CHANNEL WAITS RATHER THAN DROPS.** The web face being busy is a
  // reason for this request to be slow, not a reason for it to fail -- and the
  // wait is this task's, which is why the sort happens off the accept loop.
  let _ = http.send(stream).await;
}

async fn answer<S>(stream: S, registry: Arc<Registry>, stop: Arc<tokio::sync::Notify>)
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

    let (response, feed, stopping) = match dispatch(&registry, &mut bound, &line).await {
      Served::Reply(response) => (response, None, false),
      Served::ReplyThenStop(response) => (response, None, true),
      Served::Feed { project_id, events } => {
        (Response::Subscribed { project_id }, Some(events), false)
      }
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

    // **THE STOP FIRES HERE, AFTER THE FLUSH, AND NOWHERE EARLIER.** This line
    // is the contract `Served::ReplyThenStop` exists to make unmissable: the
    // client has its answer before the accept loop is told anything, so a
    // successful stop cannot be mistaken for the daemon dying mid-request.
    if stopping {
      stop.notify_one();
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
  /// Answer, and THEN stop the daemon.
  ///
  /// **A SEPARATE VARIANT RATHER THAN A NOTIFY INSIDE `dispatch`, BECAUSE THE
  /// ORDER IS THE CONTRACT.** Firing the stop where the request is understood
  /// would race the reply: the accept loop can break and `main` return while
  /// the task that owes the client an answer is still writing it. **The client
  /// would then see a closed connection, which is what a daemon dying
  /// mid-request looks like** -- so a successful stop and a crash would be
  /// reported identically. Carrying the intent back to the writer makes the
  /// sequence impossible to get wrong: reply, flush, then notify.
  ReplyThenStop(Response),
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

  // **ANSWERED WITHOUT BINDING, FOR A SHARPER VERSION OF `Op::Registry`'s
  // REASON.** Stopping the daemon is not a question for any project, and the
  // operator most likely to ask it is the one whose project will not open --
  // so binding first would refuse the very request that fixes their machine.
  if matches!(request.op, Op::Shutdown) {
    return Served::ReplyThenStop(Response::Stopping);
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

/// Who owns this process's lifetime, DERIVED from stdin rather than declared.
///
/// **A DAEMON'S LIFETIME IS OWNED BY SOMEBODY, AND UNTIL NOW THIS ONE ASSUMED
/// THE OWNER EXISTS.** `launchd` owns the production daemon and stops it; in a
/// test the owner is a `cargo` process that may be killed, and this binary
/// could not tell the two apart. It served on, holding the store, answering
/// nobody -- 64 of them on one machine by 2026-09-10, 64.9 CPU-hours between
/// them (`ST0073`, discharging `0284`).
///
/// # WHY DERIVED AND NOT AN ENVIRONMENT VARIABLE
///
/// **THE FIRST BUILD OF THIS READ `INTENT_DAEMON_LIFELINE` AND
/// `the_shipped_surface_reads_exactly_one_environment_variable` REFUSED IT**,
/// correctly: `AC-11.3`'s invariant is hv's, a second variable needs an hv
/// ruling and a row in `ALLOWED`, and **every machine here would have had it
/// set, so nothing else would have failed.** The guard is not an obstacle that
/// was routed around -- it named a real cost, and the cost bought nothing that
/// the kernel does not already report.
///
/// **THE DISCRIMINATOR IS A FACT ABOUT THE DESCRIPTOR, NOT A CONVENTION
/// BETWEEN CALLERS**, which is the same reason `lib_currency.sh` derives its
/// exclusion instead of hand-listing it: a declared marker rots, and a `stat`
/// cannot. Measured on this platform rather than assumed:
///
///   pipe          `is_fifo`          -> Owned      (a harness passed one)
///   `/dev/null`   `is_char_device`   -> Supervised (what `launchd` hands us)
///   terminal      `is_char_device`   -> Supervised (`intent daemon run`)
///   regular file  `is_file`          -> Supervised (a redirect)
///
/// **THE PRODUCTION PATH IS THEREFORE UNREACHABLE BY ACCIDENT.** `launchd`
/// never hands a daemon a pipe, so `Owned` is not a state the LaunchAgent can
/// enter. That matters more than the leak it fixes: the plist is
/// `KeepAlive false` with no socket activation, so a daemon that exited when it
/// should not have would stay exited until the next login, and nothing in the
/// failure would name this code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Lifeline {
  /// Something else owns it: `launchd`, or a terminal running
  /// `intent daemon run`. Serve until signalled.
  Supervised,
  /// The process holding the other end of stdin owns it, and its death is
  /// observed as EOF.
  ///
  /// **THE OWNER DOES NOT HAVE TO DO ANYTHING, WHICH IS THE ENTIRE DESIGN
  /// REQUIREMENT.** The code that would run at teardown is precisely the code
  /// that does not run when a process is killed -- so the kernel closing the
  /// write end is the only mechanism that survives the case this exists for.
  ///
  /// **AND IT IS EVENT-DRIVEN, NOT SWEPT.** No timer, no interval, no periodic
  /// `getppid`. A poll leaves a window proportional to its period during which
  /// an orphan still holds the store, and the window is invisible in a test
  /// generous enough to pass. The rejected alternatives were both
  /// non-portable: `PR_SET_PDEATHSIG` is Linux-only, and kqueue `NOTE_EXIT`
  /// keys on a pid, which can be recycled underneath it. **A descriptor cannot
  /// be recycled while it is open, so a pipe is the form with no race.**
  Owned,
}

impl Lifeline {
  /// Ask the kernel what stdin is.
  ///
  /// **AN UNREADABLE STDIN IS `Supervised`, AND THE ASYMMETRY IS DELIBERATE.**
  /// Guessing `Owned` and being wrong stops `launchd`'s daemon until the next
  /// login; guessing `Supervised` and being wrong leaves one orphan that the
  /// next reap collects. The two errors are not the same size. It is reported
  /// rather than swallowed, because a daemon quietly declining to notice its
  /// owner is the defect this whole type exists to remove.
  fn observed() -> Lifeline {
    // **THE PREDICATE LIVES IN `intentsvcs`, NOT HERE, BECAUSE TWO PROCESSES
    // ASK IT.** `intent daemon start` asks the same question to decide whether
    // to RELAY the lifeline it was handed. Two copies would be two answers to
    // the one question that decides whether a daemon can be left running for
    // ever.
    if intentsvcs::daemon::stdin_is_a_lifeline() {
      Lifeline::Owned
    } else {
      Lifeline::Supervised
    }
  }

  /// Resolve when the owner is gone. Never resolves when `Supervised`.
  ///
  /// **THE FUTURE IS BUILT ONCE AND POLLED, NEVER REBUILT INSIDE THE SELECT.**
  /// `tokio::select!` re-evaluates its expressions on every pass of the loop,
  /// so calling this in the arm would spawn a fresh reader thread per accepted
  /// connection. It is pinned outside the loop for that reason.
  async fn closed(self) -> &'static str {
    match self {
      Lifeline::Supervised => {
        std::future::pending::<()>().await;
        unreachable!("pending never resolves")
      }
      Lifeline::Owned => {
        let (tx, rx) = tokio::sync::oneshot::channel::<()>();
        // **A PLAIN OS THREAD, NOT `spawn_blocking`.** A blocking-pool task
        // that never returns is one the runtime waits for at shutdown, so the
        // daemon would hang on the way out for every OTHER stop reason. This
        // thread is detached: it blocks in the kernel until the write end
        // closes, and it dies with the process.
        std::thread::spawn(move || {
          use std::io::Read;
          let mut byte = [0u8; 1];
          loop {
            match std::io::stdin().read(&mut byte) {
              // EOF. Every copy of the write end is closed, which is what the
              // kernel does for a process that dies by any means at all.
              Ok(0) => break,
              // The owner wrote something. Nothing here reads stdin for
              // meaning -- this daemon takes no input -- so it is discarded and
              // the wait continues. **A LIFELINE IS NOT A MESSAGE CHANNEL**,
              // and exiting on readability rather than on EOF would kill the
              // daemon the moment anyone wrote a byte for any reason.
              Ok(_) => continue,
              Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
              // Unreadable is treated as gone. The alternative -- serving on --
              // is the orphan this exists to prevent, arriving through an error
              // path instead of through a missing feature.
              Err(_) => break,
            }
          }
          let _ = tx.send(());
        });
        let _ = rx.await;
        "the lifeline closed, so whoever started this daemon is gone"
      }
    }
  }
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

/// Regenerate this machine's LaunchAgent if an older build wrote it
/// (`AC-08.7`).
///
/// **THE STAMP IS THE TRIGGER AND THE CONTENT IS WHAT HEALS.** A version bump
/// alone rewrites a byte-identical file, which is a no-op and fine; what the
/// marker buys is that when the content DOES change -- a new log path, a key we
/// start emitting, a binary that moved -- the operator's plist is brought up to
/// date without them running anything. **That is the whole of "an old install
/// heals without a migration": the healing is a property of booting, so there
/// is no step anyone can skip.**
///
/// **IT REGENERATES THE FILE AND DOES NOT RELOAD THE JOB, DELIBERATELY.**
/// `launchctl unload` + `load` on our own label would stop THIS process -- the
/// one doing the healing -- so a daemon that reloaded its own job would exit
/// every time it healed, and `KeepAlive` is false, so nothing would bring it
/// back. Writing the file is enough: `launchd` reads it at the next login,
/// which is exactly when a LaunchAgent's contents matter.
///
/// **REPORTED AND NEVER FATAL.** A daemon that refused to start because it
/// could not rewrite a plist would turn a cosmetic staleness into an outage,
/// and the plist is not needed for this process to serve anything -- it is only
/// needed for the NEXT one to start itself.
fn heal_the_policy_stamp() {
  let Ok(home) = intentsvcs::userstate::home() else {
    // No home means no per-user state at all, which the daemon reports
    // elsewhere when it matters. There is no plist to heal.
    return;
  };
  if !launchagent::is_stale(&home) {
    return;
  }
  let was = launchagent::stamped_version(&home).unwrap_or_else(|| "unstamped".to_string());
  // **THE PLIST NAMES THIS BINARY, WHICH IS THE ONE FACT AN OLD PLIST IS MOST
  // LIKELY TO HAVE WRONG.** Resolved from `current_exe` rather than from
  // anything ambient, on `install.rs`'s precedent: a path read from the
  // environment would heal the plist to whatever the environment happened to
  // say, which is how a self-healing artefact starts healing itself wrong.
  let Ok(binary) = std::env::current_exe() else {
    eprintln!(
      "warning: this machine's LaunchAgent was written by {was} and could not be regenerated: this process cannot resolve its own path\n  remedy: `intent daemon start --at-login` rewrites it."
    );
    return;
  };
  match launchagent::write_plist(&home, &binary) {
    Ok(path) => println!(
      "intentd: regenerated the LaunchAgent at {} (was {was}, now {})",
      path.display(),
      intentsvcs::faces::INTENT_VER
    ),
    Err(e) => eprintln!(
      "warning: this machine's LaunchAgent was written by {was} and could not be regenerated: {e}\n  remedy: `intent daemon start --at-login` rewrites it. The daemon is running and unaffected; the stale plist only matters at the next login."
    ),
  }
}
