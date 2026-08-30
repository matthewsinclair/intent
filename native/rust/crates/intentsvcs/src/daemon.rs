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
use std::net::{SocketAddr, TcpStream};
use std::os::unix::net::UnixStream;
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
/// length-prefix format here. **Nothing about the RESPONSE is specified**, on
/// purpose -- see [`completes_a_round_trip`]. Naming a reply shape would put
/// half a wire protocol in the routing seam, where the daemon that has to
/// honour it does not yet exist.
const PROBE_FRAME: &[u8] = b"{\"intent_probe\":1}\n";

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
/// **ONE VARIANT, AND IT EXISTS BECAUSE ABSENCE AND UNREADABILITY ARE
/// DIFFERENT ANSWERS.** No address file means no TCP candidate, which is a
/// state and not a fault. An address file that cannot be parsed means the
/// daemon published something and this build could not read it -- and quietly
/// dropping that candidate is the ONE failure direction this whole rule exists
/// to prevent, because a shorter list routes in-process while a daemon holds
/// the store. So it refuses instead.
#[derive(Debug, Error)]
pub enum DaemonError {
  #[error("the daemon address at `{path}` is not an address: {found:?}")]
  UnreadableAddress { path: PathBuf, found: String },
}

impl crate::remedy::Remedy for DaemonError {
  fn remedy(&self) -> String {
    "this file is written by intentd when it starts and holds one loopback address, eg `127.0.0.1:54321`. If no daemon is running, deleting it is safe and this command will run in-process; if one is running, restart it so it republishes its address.".to_string()
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
  // `read_to_string` failing is read as absent on purpose: a file that is not
  // there and a file this process may not read both mean "no address this
  // build can use", and neither is the daemon telling us something we failed
  // to understand. What follows is that case.
  if let Ok(text) = std::fs::read_to_string(&published) {
    let trimmed = text.trim();
    match trimmed.parse::<SocketAddr>() {
      Ok(addr) => found.push(Endpoint::Tcp(addr)),
      Err(_) => {
        return Err(DaemonError::UnreadableAddress {
          path: published,
          found: trimmed.chars().take(80).collect(),
        });
      }
    }
  }
  Ok(found)
}
