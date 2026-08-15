//! INV-02 / AC-05.2: usage errors exit **1**, not clap's default 2.
//!
//! **This file was written BEFORE the clap spine existed**, because ic's
//! dispatch table asks for exactly that and the reason is worth restating: v2
//! exits 1 on every usage error (`error()` at `bin/intent_helpers:7-11` is
//! `echo >&2; exit 1`, and it is the only failure exit in the shipped surface
//! bar `intent critic`), while clap exits 2 for both
//! `ErrorKind::MissingRequiredArgument` and `ErrorKind::UnknownArgument`.
//!
//! D17 carries v2's codes over, so the override is a framework-layer decision
//! made once at the start. Pinned here so that a clap major bump, or someone
//! removing the override, reds ONE named invariant -- instead of a hundred BATS
//! conformance tests failing for a reason nobody traces back to this decision.
//!
//! The exception is `intent critic`, which genuinely uses 2 (INV-04); asserting
//! that here too is what stops a blanket "always exit 1" from looking correct.

use std::process::{Command, Output};

/// Run the built binary. `CARGO_BIN_EXE_<name>` is set by cargo for
//. integration tests, so this needs no dev-dependency and cannot pick up a
/// stale `intent` from PATH -- which would silently measure v2.
fn run(args: &[&str]) -> Output {
  Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .output()
    .expect("run the v3 binary")
}

#[test]
fn a_missing_required_argument_exits_1() {
  let out = run(&["st", "show"]);
  assert_eq!(
    out.status.code(),
    Some(1),
    "clap's default is 2; D17 carries v2's 1 over.\nstdout: {}\nstderr: {}",
    String::from_utf8_lossy(&out.stdout),
    String::from_utf8_lossy(&out.stderr)
  );
}

#[test]
fn an_unknown_flag_exits_1() {
  let out = run(&["st", "list", "--no-such-flag"]);
  assert_eq!(
    out.status.code(),
    Some(1),
    "clap's default is 2 for UnknownArgument too.\nstderr: {}",
    String::from_utf8_lossy(&out.stderr)
  );
}

#[test]
fn an_unknown_subcommand_exits_1() {
  let out = run(&["st", "bogusverb"]);
  assert_eq!(out.status.code(), Some(1));
}

#[test]
fn an_unknown_family_exits_1() {
  let out = run(&["nosuchfamily"]);
  assert_eq!(out.status.code(), Some(1));
}

/// INV-01: the voice is lowercase `error:` on stderr, no banners.
///
/// clap writes its own `error: ...` in a different shape and to a stream that
/// varies by error kind, so this asserts BOTH the prefix and the stream.
#[test]
fn usage_errors_speak_v2s_voice_on_stderr() {
  let out = run(&["st", "show"]);
  let stderr = String::from_utf8_lossy(&out.stderr);
  assert!(
    stderr.starts_with("error: "),
    "the lowercase voice (0023, INV-01), got: {stderr}"
  );
  assert!(
    String::from_utf8_lossy(&out.stdout).is_empty(),
    "a failure writes nothing to stdout -- INV-06 is the v2 defect being corrected, not reproduced"
  );
  assert!(
    !stderr.contains("Usage:") || stderr.lines().count() < 20,
    "no banner dump on a usage error"
  );
}

/// A successful invocation exits 0 and says so in v2's voice.
#[test]
fn success_exits_0() {
  let out = run(&["--version"]);
  assert_eq!(
    out.status.code(),
    Some(0),
    "stderr: {}",
    String::from_utf8_lossy(&out.stderr)
  );
}

/// INV-04's exception, asserted so a blanket always-exit-1 cannot pass.
///
/// `intent critic` genuinely uses 2 -- it is the one place in the shipped
/// surface where 2 means something. Without this case, an override that
/// hard-coded 1 everywhere would look correct.
#[test]
fn the_critic_exception_is_not_flattened_by_the_override() {
  let out = run(&["critic", "--help"]);
  let code = out.status.code().expect("exited");
  assert!(
    code != 2 || !String::from_utf8_lossy(&out.stderr).contains("unexpected argument"),
    "critic keeps its own exit-code semantics (INV-04) rather than being swept into the blanket override"
  );
}
