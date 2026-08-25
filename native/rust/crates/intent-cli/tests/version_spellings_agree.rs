//! **ONE CAPABILITY MUST NOT DISAGREE WITH ITSELF ABOUT WHETHER IT EXISTS.**
//!
//! Driven 2026-08-25: `intent --version` returned rc=0 and a version, while
//! `intent version` returned rc=2 and *`version` is a known command that is not
//! implemented yet* -- the same capability, present by one spelling and absent
//! by the other, in one binary. **The failing spelling is the one a person
//! types first**, which is what made it worth a criterion rather than a fix.
//!
//! # Why this asserts BYTES and not just both-succeed
//!
//! Wiring the subcommand closes the exit-code gap on its own, and it would
//! leave a strictly harder defect behind it: two spellings that both answer,
//! with different text, and nothing saying they differ. A rc-only test passes
//! on that. **The property is that the two are one capability, so the test is
//! byte-identity of stdout** -- the strongest statement that can be made from
//! outside the binary.
//!
//! The renderer closes it by construction (`render_version()` is the same
//! string `spine::parse` prints for clap's `DisplayVersion`), and this file is
//! written anyway, because **a shared SOURCE is not a shared OUTPUT**: a
//! `println!` where the arm has `print!`, or any later decoration, keeps the
//! shared source and breaks the shared bytes, and the type checker is happy
//! with both.
//!
//! # The vacuity guard, stated rather than discovered
//!
//! Two empty outputs are byte-identical. An equality test over a capability
//! that had stopped answering at all would therefore PASS -- the shape this
//! thread keeps catching, most recently a search test whose fixture collapsed
//! the distinction it was written to prove. So equality is asserted only
//! alongside the output being a real version line: non-empty, carrying the
//! package version, and carrying the embedded commit. **Each of those three
//! would independently fail if the answer went hollow.**

use std::process::{Command, Output};

fn run(args: &[&str]) -> Output {
  Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .output()
    .expect("run the v3 binary")
}

fn stdout(out: &Output) -> String {
  String::from_utf8_lossy(&out.stdout).into_owned()
}

/// The flag and its subcommand twin print the same bytes and both succeed.
#[test]
fn the_flag_and_its_subcommand_twin_are_one_capability() {
  let flag = run(&["--version"]);
  let sub = run(&["version"]);

  assert_eq!(
    flag.status.code(),
    Some(0),
    "`--version` must succeed.\nstderr: {}",
    String::from_utf8_lossy(&flag.stderr)
  );
  assert_eq!(
    sub.status.code(),
    Some(0),
    "`version` must succeed -- it refused rc=2 as unimplemented while `--version` \
     answered rc=0, which is the defect this file exists for.\nstderr: {}",
    String::from_utf8_lossy(&sub.stderr)
  );

  // THE VACUITY GUARD COMES FIRST, so a hollow answer fails here rather than
  // sailing through the equality below.
  let printed = stdout(&flag);
  assert!(
    !printed.trim().is_empty(),
    "`--version` printed nothing; two empty outputs would compare equal and this \
     file would certify a capability that had stopped answering"
  );
  assert!(
    printed.contains(env!("CARGO_PKG_VERSION")),
    "the version line does not carry the package version.\nprinted: {printed}"
  );

  assert_eq!(
    printed,
    stdout(&sub),
    "the two spellings answered with DIFFERENT bytes. They are one capability \
     with two names, so this is the same defect as one of them refusing -- only \
     quieter, because both of them answer"
  );
}

/// The `corrected` ratification's own clause: the string carries the build.
///
/// **A version alone does not identify a build in this estate**, where every
/// binary reports the same `3.0.0-dev` and four nodes build from one clone.
/// The commit is what separates them, so its absence is a real regression and
/// not a cosmetic one -- it is the difference between naming a release line and
/// naming an artefact.
///
/// Asserted structurally rather than against a literal sha, which would pin the
/// test to the commit that wrote it: what must hold is that SOMETHING
/// build-shaped is there. `dirty-<sha>` and `unknown` are both legitimate
/// values -- the first when the tree carries uncommitted Rust, the second when
/// git could not answer -- so the assertion admits all three forms and would
/// still fail on an empty or absent parenthetical.
#[test]
fn the_version_string_names_the_build_and_not_only_the_line() {
  let printed = stdout(&run(&["version"]));
  let inside = printed
    .split_once('(')
    .and_then(|(_, rest)| rest.split_once(')'))
    .map(|(inside, _)| inside.to_string())
    .unwrap_or_default();

  assert!(
    !inside.trim().is_empty(),
    "the version line carries no build marker.\nprinted: {printed}"
  );

  let sha_shaped = inside.len() >= 7 && inside.chars().all(|c| c.is_ascii_hexdigit());
  let known_form = sha_shaped
    || inside == "unknown"
    || inside
      .strip_prefix("dirty-")
      .is_some_and(|s| s.len() >= 7 && s.chars().all(|c| c.is_ascii_hexdigit()));
  assert!(
    known_form,
    "the build marker is not a commit, `dirty-<sha>` or `unknown`: {inside:?}"
  );
}

/// v2 accepted `intent version --zzz` silently at exit 0 (INV-08). v3 refuses.
///
/// **The refusal is the ratified correction, so it is asserted rather than
/// left to clap's defaults** -- clap's own default for an unrecognised argument
/// is exit 2, and D17/INV-02 carry v2's 1 across the whole surface. A test that
/// only checked "non-zero" would pass on the unported behaviour.
#[test]
fn an_unknown_argument_is_refused_rather_than_accepted_in_silence() {
  let out = run(&["version", "--zzz"]);
  assert_eq!(
    out.status.code(),
    Some(1),
    "v2 took this at exit 0 and printed the version anyway (INV-08); v3 refuses \
     at 1 per INV-02.\nstdout: {}\nstderr: {}",
    stdout(&out),
    String::from_utf8_lossy(&out.stderr)
  );
  assert!(
    stdout(&out).trim().is_empty(),
    "a refused invocation must not also print the version: {}",
    stdout(&out)
  );
}
