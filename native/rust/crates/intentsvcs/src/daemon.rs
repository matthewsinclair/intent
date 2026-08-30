//! Reaching `intentd`: where it might be listening, and whether it is there.
//!
//! **THIS IS THE ADDRESS AND THE ROUTING RULE, NOT THE DAEMON.** The daemon is
//! a CLIENT of this crate exactly as the CLI is (D06), so nothing here knows
//! what the daemon does -- only how to find out whether one is answering.
//!
//! **IT LIVES IN `intentsvcs` BECAUSE TWO BINARIES MUST AGREE ON ONE ADDRESS.**
//! The CLI resolves it to connect and `intentd` resolves it to bind. Two homes
//! for that value is a daemon that listens where the CLI never looks, with
//! nothing comparing the two -- and the failure is silent in the worst
//! direction, because a CLI that finds no daemon just works, in-process,
//! forever. `intentsvcs` is the only crate both binaries can depend on.
//!
//! ## The rule
//!
//! design.md D-line 22, quoted rather than restated (design.md is the live
//! text; this is a record of what it said):
//!
//! > if the intentd socket exists and answers, the CLI MUST route to it (never
//! > two sync engines live at once); when absent, it executes in-process
//! > against the same facade.
//!
//! **`EXISTS AND ANSWERS`, AND THE SECOND HALF IS THE WHOLE POINT.** A unix
//! socket file OUTLIVES the process that bound it -- kill the daemon and the
//! path is still there. A rule keyed on existence alone routes every verb at a
//! socket nobody is listening on, which is worse than not routing at all: the
//! in-process engine was available the entire time.
//!
//! **AND A CONNECT IS NOT AN ANSWER EITHER, WHICH IS THE SAME DEFECT ONE LAYER
//! DOWN.** A listening descriptor that leaked into a surviving child keeps the
//! backlog open with nobody accepting, so `connect` succeeds forever against a
//! dead owner. So the probe is a bounded round TRIP -- see
//! [`Endpoint::answers`], which carries the mechanism and the measurement.
//!
//! ## Why the candidates are a LIST
//!
//! **THE TRANSPORT IS A VARIANT THIS BODY DRIVES UNEDITED** (vc's instruction,
//! 2026-08-29, and it is the same mechanism that makes the dual-path
//! conformance harness extensible: routes as a list, so adding one is adding a
//! variant rather than editing the loop). D56 gives the daemon ONE output
//! contract "over the socket and over HTTP", and a browser cannot connect to a
//! unix socket while `URLSession` cannot either -- so a routing rule that can
//! only ever have one listener is wrong on the face of the decision it
//! implements. [`route`] never names a transport. Adding one is a variant plus
//! its [`Endpoint::answers`] arm.
//!
//! **THERE IS NO PORT CONSTANT IN THIS FILE OR ANYWHERE ELSE, AND THAT IS
//! hv's D6 RATHER THAN AN OMISSION.** The daemon binds `127.0.0.1:0`, takes
//! whatever the kernel gives it, and PUBLISHES it; [`candidates`] reads that
//! file. So the TCP candidate exists exactly when a daemon has said where it
//! is, and the question "which port" -- unresolved in canon, where
//! `tui-design.md` writes `http://127.0.0.1:<port>/` with the port literally
//! unwritten -- never has to be answered by anyone. **An earlier draft of this
//! paragraph said the TCP variant existed and `candidates` produced none, which
//! was true when it was written and false a few hours later**; it survived the
//! commit that falsified it, in the file whose own subject is a rule that rots
//! when it is summarised rather than derived. The test drives mixed lists so
//! the multi-transport body is exercised rather than being a loop that has
//! never run twice.
//!
//! ## Where the path comes from
//!
//! [`crate::userstate`], and only there. `$HOME` is confined to that module by
//! path, enforced over the whole shipped surface -- so this module takes an
//! address rather than resolving one, and stays a pure function a test can
//! drive against any temp directory it likes.

use std::fmt;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::os::unix::fs::MetadataExt;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;

use std::time::Duration;
use thiserror::Error;

/// How long each step of the probe gets before the endpoint counts as absent.
///
/// **APPLIED PER STEP -- CONNECT, WRITE, READ -- SO THE WORST CASE IS THREE
/// TIMES THIS AND THAT IS DELIBERATE.** It is a bound, not a budget: on a live
/// loopback or unix endpoint every step completes in microseconds, and on a
/// dead one connect is REFUSED immediately rather than timing out. The
/// deadline only bites in the two cases that have no answer at all -- a
/// firewall rule that DROPs rather than rejects, and the inherited listener
/// below -- where the alternative is not a slower CLI but a hung one.
///
/// **`AC-08.3` MAKES EXPIRY MEAN ABSENT, WHICH MOVES THE RISK RATHER THAN
/// REMOVING IT, AND THE DIRECTION IT MOVES IN IS THE WORSE ONE.** A probe that
/// gives up too early on a LIVE daemon routes in-process against a store that
/// daemon owns -- two sync engines, the one thing the rule exists to forbid --
/// whereas the false positive it replaces costs a failed request and no
/// corruption. **So this constant is safe only while answering the probe
/// cannot queue behind request work**, which is an obligation on the daemon
/// and not on this value: the probe is answered on the accept path, before
/// anything per-request. Raising the number is not the fix if that is ever
/// violated, because no number is.
const PROBE_DEADLINE: Duration = Duration::from_millis(250);

/// What the probe sends to make an endpoint prove it is listening.
///
/// **THE SMALLEST THING THAT IS INSIDE D56 AND OUTSIDE THE PROTOCOL.** D56
/// rules the daemon's output contract JSON, so the probe is a JSON object; it
/// is newline-terminated because a reader has to know the request ended, and a
/// newline is the least framing that achieves that without minting a
/// length-prefix format here.
///
/// **THIS DOC USED TO DECLINE TO NAME A REPLY SHAPE, AND ITS REASON EXPIRED
/// RATHER THAN BEING WRONG.** It said naming one *would put half a wire
/// protocol in the routing seam, where the daemon that has to honour it does
/// not yet exist* -- true while nothing could answer. The daemon lands in this
/// work package, so the question is no longer whether to name a reply but
/// WHERE, and the answer is here: **a probe is a request and a response, and a
/// pair split across two crates is two homes for one agreement.** The frame
/// would live in the seam and the answer in `intentd`, each meaningless alone,
/// which is how a predicate this subtle drifts.
///
/// It stays PRIVATE and the daemon reaches [`is_probe_frame`] instead. The
/// daemon needs the RECOGNITION, not the literal, and a test asserting what
/// goes on the wire should carry its own copy rather than import the value it
/// is checking.
const PROBE_FRAME: &[u8] = b"{\"intent_probe\":1}\n";

/// The daemon's answer to a probe.
///
/// **IT CARRIES NO DATA, AND THE EMPTINESS IS THE DESIGN.** [`Endpoint::answers`]
/// requires one byte and parses nothing, deliberately -- so anything put in
/// here would be a field no client reads, which is a promise with no check
/// behind it. A version, a pid or a project count would all look useful and all
/// rot silently. **When the daemon has something to say it says it in a
/// response to a real request, where somebody is reading.**
///
/// JSON and newline-terminated because D56 rules the daemon emits JSON only,
/// over the socket AND over HTTP, and the probe is not an exception to that
/// just because its content is empty.
pub const PROBE_REPLY: &[u8] = b"{\"intent_probe\":\"ack\"}\n";

/// Is this the probe frame, so the daemon can answer it before dispatching?
///
/// **THE DAEMON MUST NOT RE-SPELL THE FRAME TO RECOGNISE IT.** Both ends have
/// to agree on these bytes exactly, and two spellings of one agreement is the
/// failure this seam is least able to survive: a daemon that stopped
/// recognising the probe would answer nothing, every client would route
/// in-process, and two sync engines would land on one store with nothing
/// reporting a fault. So the comparison lives with the frame.
///
/// Trailing whitespace is ignored on both sides because a line-oriented reader
/// usually strips the newline before it asks, and refusing on that would make
/// the recogniser depend on how the caller framed its read rather than on what
/// arrived.
pub fn is_probe_frame(bytes: &[u8]) -> bool {
  fn without_trailing_space(b: &[u8]) -> &[u8] {
    let end = b
      .iter()
      .rposition(|c| !c.is_ascii_whitespace())
      .map_or(0, |i| i + 1);
    &b[..end]
  }
  without_trailing_space(bytes) == without_trailing_space(PROBE_FRAME)
}

/// A connected stream the probe can drive, whatever it is connected over.
///
/// **THE ROUND TRIP IS WRITTEN ONCE AND BOTH TRANSPORTS DRIVE IT.** `std` gives
/// `UnixStream` and `TcpStream` the same three methods and no common trait, so
/// without this the liveness rule would exist twice -- and two copies of a
/// predicate this subtle is how one of them quietly stops matching the other.
/// The per-variant part of [`Endpoint::answers`] is then only the connect,
/// which is the only part that genuinely differs.
trait Probeable: Read + Write {
  fn set_deadline(&self, budget: Duration) -> std::io::Result<()>;
}

impl Probeable for UnixStream {
  fn set_deadline(&self, budget: Duration) -> std::io::Result<()> {
    self.set_write_timeout(Some(budget))?;
    self.set_read_timeout(Some(budget))
  }
}

impl Probeable for TcpStream {
  fn set_deadline(&self, budget: Duration) -> std::io::Result<()> {
    self.set_write_timeout(Some(budget))?;
    self.set_read_timeout(Some(budget))
  }
}

/// Did something on the other end ACCEPT, read, and write back in time?
///
/// **ONE BYTE IS THE WHOLE TEST, AND ASKING FOR MORE WOULD BE WORSE.** The
/// question is not what the daemon said, it is whether anything is there to
/// say it: a phantom endpoint holds an open listen backlog with nothing behind
/// it, so a `connect` succeeds and no byte ever arrives. Requiring a
/// well-formed reply instead would make this seam a second home for the
/// response format, and would fail against a daemon whose reply shape moved.
///
/// **`Ok(0)` IS A CLEAN EOF AND COUNTS AS ABSENT.** Something accepted and
/// closed without answering -- a listener shutting down, or a server that
/// rejects what it cannot parse. Either way it cannot serve this invocation,
/// which is the only question being asked.
fn completes_a_round_trip<S: Probeable>(mut stream: S) -> bool {
  if stream.set_deadline(PROBE_DEADLINE).is_err() {
    return false;
  }
  if stream.write_all(PROBE_FRAME).is_err() {
    return false;
  }
  if stream.flush().is_err() {
    return false;
  }
  let mut first = [0u8; 1];
  matches!(stream.read(&mut first), Ok(read) if read > 0)
}

/// An address `intentd` may be listening on.
///
/// **PER-VARIANT BEHAVIOUR IS THE EXTENSION POINT, AND THE NEXT THING TO ARRIVE
/// THROUGH IT IS AUTHZ** (vc, 2026-08-29). The two transports do not have one
/// access-control story and must never be made to share one: a unix socket is
/// guarded by filesystem permissions and needs nothing built, while a loopback
/// TCP port is reachable by every process on the machine under any user and
/// needs at least a token. **Uniform treatment fails in both directions** --
/// the socket inherits a check it does not need, or the TCP port inherits the
/// socket's absence of one, and the second is a hole rather than an
/// inefficiency. [`answers`](Endpoint::answers) already matches per variant;
/// whatever authz lands arrives the same way. Nothing in [`route`] may come to
/// assume the two are alike.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Endpoint {
  /// A unix domain socket, addressed by path.
  Unix(PathBuf),
  /// A TCP socket on the loopback interface.
  Tcp(SocketAddr),
}

impl Endpoint {
  /// Is a daemon ANSWERING here?
  ///
  /// **A COMPLETED ROUND TRIP, NEVER A STAT AND NEVER A BARE CONNECT.** The
  /// two weaker tests fail on two different real cases, and each one is the
  /// same defect one layer down from the last:
  ///
  /// - `Path::exists` is wrong because **a unix socket file OUTLIVES the
  ///   process that bound it.** Kill the daemon and the path is still there,
  ///   so a presence test routes every verb at an address nobody is listening
  ///   on -- while the in-process engine that would have served the request was
  ///   available the whole time.
  /// - `connect().is_ok()` is wrong because **a successful connect is not an
  ///   answer.** macOS has no `SOCK_CLOEXEC`, so a listening socket is created
  ///   by one syscall and marked close-on-exec by a second; a `fork` landing
  ///   between them leaks the LISTENING descriptor into the child. If that
  ///   child outlives the daemon, the socket file is on disk and the listen
  ///   backlog is open, so `connect` succeeds against an owner that is dead and
  ///   **nothing ever accepts.** Measured in this estate's own test process at
  ///   1 in 300 with a sibling thread spawning children, 0 in 2000 without.
  ///
  /// So the probe writes [`PROBE_FRAME`] and requires a byte back inside
  /// [`PROBE_DEADLINE`]. **The daemon still closes the window it can** -- set
  /// `FD_CLOEXEC` immediately, keep child spawns off the bind path -- **and
  /// this rule must never come to depend on that having worked**, because the
  /// window is a race the daemon can narrow and cannot eliminate. The client is
  /// where it is made harmless.
  ///
  /// **AN EXPIRED DEADLINE IS ABSENT, AND THAT IS THE DANGEROUS DIRECTION, NOT
  /// THE SAFE ONE.** A CLI that blocks on a dead daemon is the outage the
  /// routing rule exists to prevent, so falling through to in-process is right
  /// -- but it is right at a cost that the earlier bare-connect form did not
  /// carry. Getting this WRONG now means running in-process against a store a
  /// live daemon owns, which is two sync engines; getting the old form wrong
  /// meant a failed request and an intact store. See [`PROBE_DEADLINE`] for the
  /// obligation that keeps the trade sound.
  ///
  /// **FALSE IS THE FAIL-SAFE ANSWER FOR EVERY *ERROR*, WHICH IS A NARROWER
  /// CLAIM THAN IT WAS.** Refused, missing, permission-denied: none of them is
  /// a daemon that can serve a request, and treating an unreadable socket as a
  /// live daemon would refuse work over a file the operator cannot even see. A
  /// TIMEOUT is not in that list on the same footing -- it is the one `false`
  /// here that can be wrong about a daemon that exists.
  pub fn answers(&self) -> bool {
    match self {
      Endpoint::Unix(path) => match UnixStream::connect(path) {
        Ok(stream) => completes_a_round_trip(stream),
        Err(_) => false,
      },
      Endpoint::Tcp(addr) => match TcpStream::connect_timeout(addr, PROBE_DEADLINE) {
        Ok(stream) => completes_a_round_trip(stream),
        Err(_) => false,
      },
    }
  }
}

impl fmt::Display for Endpoint {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Endpoint::Unix(path) => write!(f, "{}", path.display()),
      Endpoint::Tcp(addr) => write!(f, "{addr}"),
    }
  }
}

/// Which engine serves this invocation.
///
/// **TWO VARIANTS, AND THE ABSENCE OF A THIRD IS THE INVARIANT.** "Never two
/// sync engines live at once" is not a runtime check anywhere -- it holds
/// because this type cannot express it and [`route`] is total. A `Both` arm,
/// or an `Option<Endpoint>` that a caller could ignore, would move the
/// guarantee from the type system into everyone's discipline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Route {
  /// A daemon is answering at this address; it owns the store.
  Daemon(Endpoint),
  /// No daemon is answering anywhere; run against the store directly.
  InProcess,
}

/// Apply the routing rule to a list of candidate addresses.
///
/// First candidate that ANSWERS wins; none answering is in-process. **The body
/// names no transport**, which is what makes a third one a variant rather than
/// an edit here.
///
/// **ORDER IS THE CALLER'S AND THIS FUNCTION IMPOSES NONE.** A precedence rule
/// baked in here would be a second place transport policy lives, and
/// [`candidates`] is already the first.
pub fn route(candidates: &[Endpoint]) -> Route {
  for candidate in candidates {
    if candidate.answers() {
      return Route::Daemon(candidate.clone());
    }
  }
  Route::InProcess
}

/// Every address this build knows to look for a daemon on.
///
/// **EMPTY IS A LEGITIMATE ANSWER, NOT AN ERROR.** Without `$HOME` there is no
/// per-user state and so no socket path -- and the honest consequence is that
/// no daemon can be found, not that the command fails. The project verbs do not
/// need `$HOME` (see [`crate::userstate::UserStateError::NoHome`]) and must not
/// start needing it because a routing probe could not resolve a directory.
/// Why a candidate list could not be computed.
///
/// **THREE ANSWERS, NOT TWO, AND CONFLATING THE LAST TWO IS WHAT THIS TYPE
/// EXISTS TO PREVENT.** No address file is a STATE: no TCP candidate, no
/// fault. A file whose CONTENT is not an address is [`Self::MalformedAddress`].
/// A file that could not be READ AT ALL is [`Self::UnreadableAddress`].
/// Quietly dropping the candidate in either of the last two is the ONE failure
/// direction this whole rule exists to prevent, because a shorter list routes
/// in-process while a daemon holds the store.
///
/// **THE SPLIT WAS FOUND BY vc, AND THE REASON IT SURVIVED IS IN THE OLD
/// NAMES.** One variant was called `UnreadableAddress` and handled only the
/// UNPARSEABLE case, while the doc beside it used *unreadable* and *cannot be
/// parsed* as though they were one thing. So the reader below matched `Ok` and
/// dropped every `Err` -- and `NotFound` is the only `Err` that is a state.
/// `PermissionDenied`, `InvalidData` and `EMFILE`/`ENFILE` are all faults, and
/// the fd-pressure ones arrive exactly when a machine is busy, which is when a
/// daemon is worth having.
///
/// **THE TWO REMEDIES MUST DIFFER OR THE SPLIT BUYS NOTHING.** The malformed
/// remedy says the file is safe to delete when no daemon is running. Saying
/// that to someone whose real problem is a descriptor limit would tell them to
/// delete a file a LIVE daemon owns.
#[derive(Debug, Error)]
pub enum DaemonError {
  /// The file was read and its content is not an address.
  #[error("the daemon address at `{path}` is not an address: {found:?}")]
  MalformedAddress { path: PathBuf, found: String },
  /// The file could not be read, for any reason other than not existing.
  ///
  /// **ABSENCE IS A STATE AND UNREADABILITY IS AN ERROR, AND A `let Ok(..)`
  /// CANNOT TELL THEM APART.** Only `NotFound` means no daemon published an
  /// address; every other kind means one may have, and we failed to find out.
  #[error("could not read the daemon address at `{path}`: {source}")]
  UnreadableAddress {
    path: PathBuf,
    #[source]
    source: std::io::Error,
  },
  /// Another daemon is already answering on the socket.
  ///
  /// **REFUSED RATHER THAN TAKEN OVER, AND THAT IS THE WHOLE POINT.** Binding a
  /// unix socket whose path exists requires unlinking it first, so "start a
  /// daemon" and "silently evict the running one" are the same two syscalls.
  /// A second daemon that evicts the first leaves it holding a listener no path
  /// reaches, serving nobody, with every client routed at the newcomer.
  #[error("a daemon is already answering on `{path}`")]
  AlreadyRunning { path: PathBuf },
  /// The daemon bound a port and could not tell anyone where.
  ///
  /// **A HARD FAILURE FOR THE DAEMON, NOT A DEGRADED MODE.** A listener nobody
  /// can find is worse than no listener: every client routes in-process, the
  /// daemon serves nothing, and the only symptom is work silently not going
  /// where the operator arranged for it to go.
  #[error("could not publish the daemon address to `{path}`: {source}")]
  Unpublishable {
    path: PathBuf,
    #[source]
    source: std::io::Error,
  },
}

impl crate::remedy::Remedy for DaemonError {
  fn remedy(&self) -> String {
    match self {
      DaemonError::MalformedAddress { .. } => "this file is written by intentd when it starts and holds one loopback address, eg `127.0.0.1:54321`. If no daemon is running, deleting it is safe and this command will run in-process; if one is running, restart it so it republishes its address.".to_string(),
      // DELIBERATELY DOES NOT SAY "DELETE IT". The content was never read, so
      // nothing here knows whether a live daemon owns this file -- and under a
      // descriptor limit one almost certainly does.
      DaemonError::UnreadableAddress { path, .. } => format!(
        "the file exists and could not be read, so its contents are unknown and a daemon may be running. Check that `{}` is readable by you, and if this process is out of file descriptors that is the cause rather than the file. Do not delete it until you know no daemon is running.",
        path.display()
      ),
      DaemonError::AlreadyRunning { .. } => {
        "this machine already has an intentd serving these projects, and a second one would evict it. Stop the running daemon first if you mean to replace it.".to_string()
      }
      DaemonError::Unpublishable { path, .. } => format!(
        "intentd bound a port and could not record it, so no client could have found it. Check that `{}` is writable -- it is created by intentd under your own per-user state directory.",
        path.parent().map(|p| p.display().to_string()).unwrap_or_else(|| "its directory".to_string())
      ),
    }
  }
}

/// Every address this build knows to look for a daemon on.
///
/// **DERIVED FROM STATE, NEVER FROM CONSTANTS** (hv's D6): the daemon binds
/// `127.0.0.1:0` and publishes what the kernel gave it, so there is no port
/// literal here to go stale, collide, or be minted ahead of a ruling.
///
/// **THE SOCKET COMES FIRST, AND THAT ORDER IS POLICY THIS FUNCTION OWNS**
/// rather than something [`route`] knows. A unix socket carries its own authz
/// in filesystem permissions and needs no token, while a loopback port is
/// reachable by every process on the machine under any user -- so where both
/// answer, the cheaper and better-guarded one wins.
///
/// **NO `$HOME` IS A LEGITIMATE EMPTY LIST, NOT AN ERROR.** Without it there is
/// no per-user state and so no daemon to find, and the project verbs must not
/// start requiring a variable they have never needed because a routing probe
/// could not resolve a directory.
pub fn candidates() -> Result<Vec<Endpoint>, DaemonError> {
  let Ok(root) = crate::userstate::home() else {
    return Ok(Vec::new());
  };
  candidates_under(&root)
}

/// [`candidates`] against any root: the one ambient read stays above, and the
/// policy below is a pure mapping a test can drive against a temp directory.
///
/// The same split [`crate::userstate::daemon_state_dir_under`] uses, for the
/// same reason -- without it the only way to test this is to mutate `$HOME`,
/// which is process-global and races every sibling test.
pub fn candidates_under(root: &std::path::Path) -> Result<Vec<Endpoint>, DaemonError> {
  let mut found = vec![Endpoint::Unix(crate::userstate::daemon_socket_under(root))];

  let published = crate::userstate::daemon_address_file_under(root);
  // **ONLY `NotFound` IS ABSENCE. EVERY OTHER KIND IS A FAULT.** Matching on
  // the kind rather than on `Ok` is the whole of vc's finding: the previous
  // form dropped the candidate on `PermissionDenied`, `InvalidData` and
  // `EMFILE`/`ENFILE` alike, so under descriptor pressure a live daemon's
  // address vanished, `route` answered `InProcess`, and the CLI wrote the
  // store while the daemon owned it. Not `metadata()` first -- that is two
  // syscalls with a race between them, answering a different question.
  match std::fs::read_to_string(&published) {
    Ok(text) => {
      let trimmed = text.trim();
      match trimmed.parse::<SocketAddr>() {
        Ok(addr) => found.push(Endpoint::Tcp(addr)),
        Err(_) => {
          return Err(DaemonError::MalformedAddress {
            path: published,
            found: trimmed.chars().take(80).collect(),
          });
        }
      }
    }
    Err(source) if source.kind() == std::io::ErrorKind::NotFound => {}
    Err(source) => {
      return Err(DaemonError::UnreadableAddress {
        path: published,
        source,
      });
    }
  }
  Ok(found)
}

/// A published daemon address, removed when this value is dropped.
///
/// **THE WRITE SIDE OF hv's D6, AND THE READER ([`candidates`]) ALREADY
/// EXISTED WITHOUT IT.** The daemon binds `127.0.0.1:0`, takes whatever port
/// the kernel gives it, and publishes that -- so no port constant exists
/// anywhere to mint, hardcode or let go stale. The question canon leaves
/// unwritten (`tui-design.md` has `http://127.0.0.1:<port>/` with the port
/// literally blank) never has to be answered by anyone.
///
/// **BINDING AND PUBLISHING ARE ONE CALL SO THAT PUBLISHING AN ADDRESS NOBODY
/// IS LISTENING ON IS UNEXPRESSIBLE.** A `publish(addr)` taking any
/// `SocketAddr` would let a caller advertise a port it never bound, which is
/// the stale-address failure created deliberately rather than by a crash. Same
/// reasoning as [`Route`] having no `Both` arm: the invariant is held by the
/// type rather than by everyone's discipline.
///
/// **REMOVED ON `Drop`, INCLUDING ON PANIC AND ON `?`, AND THAT IS THE WHOLE
/// REASON IT IS A GUARD RATHER THAN A WRITE PLUS A TIDY-UP.** A published
/// address is a CLAIM THAT A DAEMON IS ALIVE, so it must not outlive the
/// process that made it. Cleanup written at the end of a run is dead code
/// until the day something returns early, and on that day it does not run.
///
/// **THIS FILE IS A PUBLICATION, NOT A MUTEX, AND MUST NOT BE ALLOWED TO
/// BECOME ONE** (vc, 2026-08-30). It says where a daemon is; it does not say
/// only one may exist. If single-daemon is a requirement it needs its own
/// mechanism and its own criterion -- **"two daemons would fight over the
/// address file" must never become the de facto reason there is only one**,
/// because that is a safety property resting on a side effect. Same fictional
/// guarantee `AC-08.11` nearly acquired before the store measurement showed
/// the store already serialises writes.
///
/// **A HARD KILL STILL LEAVES A STALE FILE, AND THAT IS DESIGNED FOR RATHER
/// THAN DEFENDED AGAINST.** `SIGKILL` runs no destructor, so this guard
/// narrows the window and cannot close it -- which is exactly why `AC-08.3`'s
/// probe refuses to trust a published address and demands a round trip
/// instead. The two halves are built against each other on purpose: this one
/// tries to be tidy, and the reader assumes it failed.
#[derive(Debug)]
pub struct Published {
  path: PathBuf,
  addr: SocketAddr,
}

impl Published {
  /// Bind a loopback listener on an OS-chosen port and publish where it landed.
  ///
  /// The listener is returned rather than kept: this type owns the FILE, and
  /// the daemon owns the socket. Dropping the listener without dropping this
  /// would leave the address published, which is the state
  /// [`Endpoint::answers`] exists to survive.
  pub fn bind_loopback_under(root: &std::path::Path) -> Result<(TcpListener, Self), DaemonError> {
    let path = crate::userstate::daemon_address_file_under(root);
    let listener = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0)).map_err(|source| {
      DaemonError::Unpublishable {
        path: path.clone(),
        source,
      }
    })?;
    let addr = listener
      .local_addr()
      .map_err(|source| DaemonError::Unpublishable {
        path: path.clone(),
        source,
      })?;
    Self::write_atomically(&path, &addr)?;
    Ok((listener, Published { path, addr }))
  }

  /// The address this guard published.
  ///
  /// Answered from the field rather than by re-reading the file: the file is
  /// not this value's source of truth. Another daemon may have replaced it, and
  /// this method answers what THIS process published.
  pub fn endpoint(&self) -> Endpoint {
    Endpoint::Tcp(self.addr)
  }

  /// Write, then rename over the target.
  ///
  /// **A READER MUST NEVER SEE HALF AN ADDRESS.** `candidates_under` REFUSES an
  /// address it cannot parse rather than dropping it -- correctly, since a
  /// dropped candidate routes in-process while a daemon holds the store -- so a
  /// torn write would turn every concurrent command into a hard error. `rename`
  /// within one directory is atomic, so a reader sees the old address or the
  /// new one and never a prefix of either.
  fn write_atomically(path: &std::path::Path, addr: &SocketAddr) -> Result<(), DaemonError> {
    let fail = |source| DaemonError::Unpublishable {
      path: path.to_path_buf(),
      source,
    };
    if let Some(parent) = path.parent() {
      std::fs::create_dir_all(parent).map_err(fail)?;
    }
    // The pid keeps two daemons racing to publish from colliding on the
    // temporary, which would otherwise be the one unsynchronised write here.
    let staging = path.with_extension(format!("tmp-{}", std::process::id()));
    std::fs::write(&staging, format!("{addr}\n")).map_err(fail)?;
    std::fs::rename(&staging, path).map_err(fail)
  }
}

impl Drop for Published {
  /// **A FAILED REMOVAL IS DELIBERATELY SILENT.** This runs on the way out,
  /// often while unwinding, and there is no caller left to tell. The state it
  /// leaves -- a stale address file -- is one the routing rule already handles,
  /// so the honest thing is to leave the reader's guarantee to do its job
  /// rather than to panic inside a destructor.
  /// **COMPARE AND DELETE, NEVER DELETE** (vc, 2026-08-30, caught before this
  /// was built rather than after). An unconditional remove deletes whatever is
  /// there, including SOMEBODY ELSE'S claim: daemon A publishes; daemon B
  /// starts, binds a different port and atomically replaces the file; A exits
  /// and its destructor removes **B's** address, leaving a live daemon nobody
  /// can find and every client routing in-process. **That is the inverse of the
  /// failure this guard exists for, and the worse direction** -- a stale file is
  /// a false positive the probe already refuses, while a missing file is a
  /// false negative no probe can correct, because there is nothing left to
  /// probe. The read-compare-unlink is itself not atomic, and that residual is
  /// accepted: a small window rather than a structural inversion, and closing
  /// it needs a lock this file must not become.
  fn drop(&mut self) {
    if let Ok(text) = std::fs::read_to_string(&self.path) {
      if text.trim() == self.addr.to_string() {
        let _ = std::fs::remove_file(&self.path);
      }
    }
  }
}

/// A bound unix socket, unlinked when this value is dropped.
///
/// **THE SOCKET'S TWIN OF [`Published`], AND ITS CONCURRENCY STORY IS
/// GENUINELY DIFFERENT.** An address file can be replaced by a second daemon
/// with one atomic `rename`, so [`Published`] compares CONTENT before removing
/// it. A socket path cannot be replaced that way at all: binding requires the
/// path to be free, so a second daemon must UNLINK the first one's socket to
/// take it -- which means "start" and "silently evict whoever is running" are
/// the same two syscalls. So the two guards differ where the mechanism differs:
/// this one refuses to evict, and compares IDENTITY rather than content on the
/// way out, because a socket file has no content to compare.
///
/// **THE START DECISION IS A LOCK, NOT A PROBE** (`AC-08.12`). **The same
/// predicate is sound for routing and unsound for eviction, and the difference
/// is the COST of being wrong rather than the accuracy of the answer.** A
/// routing false negative is one redundant in-process run. An eviction false
/// negative leaves a live daemon holding a listener on an unlinked inode,
/// reachable by no path, serving nobody, silently -- and `SIGSTOP`, a suspended
/// VM or merely a loaded machine defeat any probe. **A lock has neither
/// failure**: the kernel releases it on death by any means, so a crash cannot
/// leave it held and a slow daemon cannot be mistaken for a dead one. It
/// DOMINATES both alternatives rather than trading between them -- refusing
/// whenever the path exists makes one crash permanent, and probing evicts the
/// living.
///
/// **WHERE THE LOCK'S GUARANTEE DOES NOT HOLD, THE PROBE STILL GUARDS THE
/// DESTRUCTIVE HALF.** `flock` is unreliable over NFS, so an answering socket
/// is never unlinked whatever the lock said.
///
/// The superseded reasoning, kept because it is right about the half it
/// covers: **THE DAEMON USES THE CLIENT'S OWN ROUTING RULE TO DECIDE.** A crash leaves
/// the socket file behind -- `AC-08.3` case 1, the case the whole probe exists
/// for -- and that stale file would otherwise block every restart forever,
/// which is a worse outage than the one it came from. So before binding, an
/// existing path is PROBED with [`Endpoint::answers`]: answering means a daemon
/// is live and this one refuses; silent means the file is a corpse and is
/// removed. **One definition of "a daemon is there", used by the client to
/// decide where to send work and by the daemon to decide whether to start** --
/// two answers to that question is how a daemon evicts a healthy peer because
/// it used a weaker test than the clients do.
#[derive(Debug)]
pub struct Bound {
  path: PathBuf,
  /// The lock whose HOLDING means "a daemon is running here" (`AC-08.12`).
  ///
  /// **HELD FOR THIS DAEMON'S WHOLE LIFE AND NEVER CONSULTED AGAIN.** Its value
  /// is that the kernel releases it on process death by ANY means, `SIGKILL`
  /// included -- so it cannot go stale the way a pid file or a socket file
  /// does, and it cannot report a live-but-busy daemon as absent the way a
  /// liveness probe does. Underscore-prefixed because nothing reads it: the
  /// FIELD existing is the mechanism.
  _lock: std::fs::File,
  /// `(dev, ino)` of the socket THIS process created.
  ///
  /// The analogue of [`Published`]'s content comparison: it is what makes the
  /// unlink on drop a compare-and-delete rather than a delete, so a daemon that
  /// was evicted cannot take its evictor's socket down on the way out.
  identity: (u64, u64),
}

impl Bound {
  /// Bind the daemon socket, refusing to evict a daemon that is answering.
  pub fn bind_socket_under(root: &std::path::Path) -> Result<(UnixListener, Self), DaemonError> {
    let path = crate::userstate::daemon_socket_under(root);
    let fail = |source| DaemonError::Unpublishable {
      path: path.clone(),
      source,
    };
    if let Some(parent) = path.parent() {
      std::fs::create_dir_all(parent).map_err(fail)?;
    }
    // **THE LOCK DECIDES, NOT THE PROBE** (`AC-08.12`, vc 2026-08-30). The
    // first build of this asked `Endpoint::answers` whether to evict, which is
    // the CLIENT's routing predicate -- sound there and unsound here, because
    // the two decisions have opposite blast radii.
    let lock_path = crate::userstate::daemon_lock_under(root);
    let lock = std::fs::File::options()
      .create(true)
      .append(true)
      .open(&lock_path)
      .map_err(|source| DaemonError::Unpublishable {
        path: lock_path.clone(),
        source,
      })?;
    match lock.try_lock() {
      Err(std::fs::TryLockError::WouldBlock) => {
        return Err(DaemonError::AlreadyRunning { path });
      }
      Err(std::fs::TryLockError::Error(source)) => {
        return Err(DaemonError::Unpublishable {
          path: lock_path,
          source,
        });
      }
      Ok(()) => {}
    }

    if path.exists() {
      // **DEFENCE WHERE THE LOCK'S GUARANTEE DOES NOT HOLD, NOT A SECOND
      // OPINION.** `flock` is unreliable over NFS and a network home directory
      // is not impossible, so two daemons could both believe they hold it. The
      // probe cannot make that correct; it does make the DESTRUCTIVE half
      // safer, because an endpoint that is ANSWERING is never unlinked whatever
      // the lock said. Where the lock works this branch is unreachable, which
      // is the point.
      if Endpoint::Unix(path.clone()).answers() {
        return Err(DaemonError::AlreadyRunning { path });
      }
      // Unlocked AND silent, so it is a corpse rather than a peer. Removing it
      // is what stops one crash from making every future start impossible.
      std::fs::remove_file(&path).map_err(fail)?;
    }
    let listener = UnixListener::bind(&path).map_err(fail)?;
    let meta = std::fs::metadata(&path).map_err(fail)?;
    Ok((
      listener,
      Bound {
        path,
        _lock: lock,
        identity: (meta.dev(), meta.ino()),
      },
    ))
  }

  /// The endpoint clients will reach this daemon on.
  pub fn endpoint(&self) -> Endpoint {
    Endpoint::Unix(self.path.clone())
  }
}

impl Drop for Bound {
  /// **COMPARE AND UNLINK, NEVER UNLINK**, for [`Published`]'s reason and by a
  /// different means: a socket file carries no content, so identity is `(dev,
  /// ino)`. A daemon whose socket was replaced under it must not remove its
  /// successor's on the way out -- that would leave a live daemon unreachable,
  /// which is the false negative no probe can correct.
  fn drop(&mut self) {
    if let Ok(meta) = std::fs::metadata(&self.path) {
      if (meta.dev(), meta.ino()) == self.identity {
        let _ = std::fs::remove_file(&self.path);
      }
    }
  }
}
