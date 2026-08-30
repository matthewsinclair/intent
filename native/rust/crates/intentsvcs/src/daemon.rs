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
//! in-process engine was available the entire time. So the probe is a connect,
//! and a stale socket falls back to in-process, which is both the safe
//! direction and the correct one.
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
//! **[`Endpoint::Tcp`] EXISTS TODAY AND [`candidates`] PRODUCES NONE.** Both
//! halves are deliberate. The variant is not minted here -- D56 names HTTP -- but
//! the PORT is unresolved in canon (`tui-design.md` writes
//! `http://127.0.0.1:<port>/` with the port literally unwritten), and choosing
//! one here would hardcode a member of a population that has not declared
//! itself. The test drives mixed lists so the multi-transport body is exercised
//! rather than being a loop that has never run twice.
//!
//! ## Where the path comes from
//!
//! [`crate::userstate`], and only there. `$HOME` is confined to that module by
//! path, enforced over the whole shipped surface -- so this module takes an
//! address rather than resolving one, and stays a pure function a test can
//! drive against any temp directory it likes.

use std::fmt;
use std::net::{SocketAddr, TcpStream};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;

use std::time::Duration;
use thiserror::Error;

/// How long a TCP candidate gets to answer before it counts as absent.
///
/// The daemon is a LOCAL process on a loopback address, where a connect either
/// completes or is refused in microseconds -- this bound exists for the one
/// case that has neither answer, a firewall rule that DROPs rather than
/// rejects, which would otherwise hang every store-reading verb on a machine
/// whose daemon is not even running. A unix socket needs no equivalent: it has
/// no network stack to disappear into.
const TCP_PROBE_TIMEOUT: Duration = Duration::from_millis(250);

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
  /// **A CONNECT, NEVER A STAT.** `Path::exists` answers a different question
  /// -- see the module note on stale sockets -- and answering it instead would
  /// be a confident wrong answer at exactly the moment a daemon has crashed.
  ///
  /// **A `true` HERE IS NOT QUITE A LIVE DAEMON, AND THE GAP IS `fork`.**
  /// macOS has no `SOCK_CLOEXEC`, so a listening socket is created by one
  /// syscall and marked close-on-exec by a second; a `fork` landing between
  /// them leaks the listening fd into the child, and the socket keeps
  /// accepting until that child exits -- however long ago the daemon died.
  /// Measured in this estate's own test process, where it made a released
  /// socket answer 1 time in 300. **The consequence is a failed request rather
  /// than a corrupted store**: routing to a phantom finds nothing to talk to,
  /// and the invariant this rule exists for -- never two engines on one
  /// database -- still holds, because the in-process engine was not started.
  /// Worth knowing before someone reads a routing refusal as proof a daemon is
  /// running.
  ///
  /// **FALSE IS THE FAIL-SAFE ANSWER AND EVERY ERROR MAPS TO IT.** Refused,
  /// missing, permission-denied, timed out: none of them is a daemon that can
  /// serve a request, and the fallback is the in-process engine that was
  /// working anyway. The opposite bias -- treating an unreadable socket as a
  /// live daemon -- would refuse work over a file the operator cannot even
  /// see.
  pub fn answers(&self) -> bool {
    match self {
      Endpoint::Unix(path) => UnixStream::connect(path).is_ok(),
      Endpoint::Tcp(addr) => TcpStream::connect_timeout(addr, TCP_PROBE_TIMEOUT).is_ok(),
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
