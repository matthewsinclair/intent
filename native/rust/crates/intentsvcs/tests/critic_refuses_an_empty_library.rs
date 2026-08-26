//! A critic run that armed nothing has not passed -- it has ABSTAINED, and the
//! two must not share an exit code.
//!
//! # What this is, measured rather than relayed
//!
//! The shipped `v3.0.0` keg carries no rule library: `SUPPORT_PATHS` copied the
//! marker directory only, and `intent/plugins/claude/rules` was never in the
//! tarball. Driven against the installed keg
//! (`intent 3.0.0 (80d8b2ca...)`) every one of the five headless languages
//! answers `total 0, armed 0`, prints its clean verdict, and exits **0**.
//!
//! `lib/templates/hooks/pre-commit.sh` then branches on that rc entirely
//! correctly and passes. So the gate in sixteen repositories reports a green
//! over an empty denominator, and **nothing in the output distinguishes it from
//! a gate that ran.** devbin hit this first and fixed its own `gate_critic`
//! (`455d3f0`); that commit scopes itself explicitly -- the hook path "stays
//! un-gated until 3.0.1". This is that.
//!
//! # Why the predicate is `total() == 0` and NOT `armed() == 0`
//!
//! devbin refuses on `armed == 0` and is right to, because devbin gates exactly
//! one language. **That predicate does not port to this runner.** Measured
//! against the SOURCE tree -- a full, correct library:
//!
//! | lang   | armed | total |
//! | ------ | ----- | ----- |
//! | elixir | 9     | 19    |
//! | rust   | 4     | 7     |
//! | swift  | **0** | 6     |
//! | lua    | **0** | 7     |
//! | shell  | 2     | 6     |
//!
//! Swift's six rules and lua's seven are all `Undeclared` -- nobody has yet
//! recorded whether they are mechanically checkable -- so **those two languages
//! arm zero in a perfectly healthy install**, and arming is a property of the
//! RULE rather than the run (shell arms 2 against `README.md` and against a real
//! shell script alike). Refusing on `armed == 0` would therefore block swift and
//! lua on every commit of a correct install: a false refusal with a real
//! population, which is the wrong-population error one layer up.
//!
//! `total() == 0` has exactly one cause -- the census is empty, so the library
//! did not load -- and no legitimate population among the declared languages.
//!
//! # Why 2, and why the hook needs no change
//!
//! The runner's codes are `0` clean, `1` findings (BLOCKS), `2` the gate itself
//! is broken (fails open), `3` an armed rule's tool is absent (BLOCKS). An empty
//! library is neither findings nor an armed-but-unenforceable rule; it is our
//! breakage, and the module's governing principle is *a gate should fail open on
//! its own breakage and closed on yours*. Blocking sixteen repositories because
//! the keg shipped without its rules is issue 0043 rebuilt, which the hook's own
//! comment forbids by name.
//!
//! 2 lands in the hook's `*)` arm -- whose comment already uses `did not check
//! (exit 2)` as its worked example -- which records the language as UNENFORCED
//! and prints the digest with its denominator. Five declared languages then read
//! `5 of 5 declared language(s) went UNENFORCED`, which that comment calls "a
//! gate that is not running at all". Loud, and open.
//!
//! # The lever, and the control that proves it reached
//!
//! Per `IN-AG-RED-CONTROL-001`, whose own worked example is a negative control
//! that set `INTENT_HOME` to a directory without templates and never reached the
//! resolver: **`$INTENT_HOME` is deliberately not read here** (`rules.rs:16` --
//! roots come from the install), so the lever is the install path handed to
//! `Library::new`, not the environment.
//!
//! `an_empty_library_produces_an_empty_census` and
//! `the_real_library_produces_a_populated_census` are the two halves of one
//! control and neither is decoration: the first shows the lever can produce a
//! zero, the second shows **this same lever can produce a non-zero**, so the
//! zero is a real zero rather than a mistyped path answering empty for every
//! input. A control that could only ever come back empty would pass against a
//! broken instrument too.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use intentsvcs::critic::{Severity, run};
use intentsvcs::rules::Library;

/// The repo root -- the install root a source-tree run resolves to.
fn repo_root() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../..")
}

fn files() -> Vec<PathBuf> {
  vec![repo_root().join("README.md")]
}

/// LEVER PROOF, half one: an install with no rules tree yields an empty census.
///
/// This is the keg's shape reproduced without a keg. `Library::files` returns
/// empty for a missing root by ruling -- *no rules installed is an ordinary
/// state* -- which is correct for a library that also serves `rules list`, and
/// is exactly why the refusal belongs to the critic rather than to `Library`.
#[test]
fn an_empty_library_produces_an_empty_census() {
  let empty = tempfile::tempdir().expect("tempdir");
  let lib = Library::new(empty.path(), None);
  let report = run(&lib, "shell", &files(), Severity::Warning, &BTreeSet::new())
    .expect("an absent rules tree is an ordinary state, not an error");

  assert_eq!(
    report.total(),
    0,
    "an install carrying no rules tree must yield an empty census -- if this is \
     non-zero the lever never reached the library and every other assertion here \
     is meaningless"
  );
}

/// LEVER PROOF, half two, and the assertion that makes half one mean something.
///
/// **Without this, a mistyped install path would answer empty for every input
/// and the refusal test would pass for the wrong reason.** This is the positive
/// control on the INSTRUMENT rather than on the subject.
#[test]
fn the_real_library_produces_a_populated_census() {
  let lib = Library::new(&repo_root(), None);
  let report = run(&lib, "shell", &files(), Severity::Warning, &BTreeSet::new())
    .expect("the repo's own rule library must load");

  assert!(
    report.total() > 0,
    "the repo's own library must load rules for `shell` -- if this is zero the \
     install path is wrong and the empty-library test proves nothing"
  );
}

/// THE DEFECT. Red before the fix: the pre-fix `exit_code` falls through an
/// empty census to 0, because `unenforced()` and `findings` are both vacuously
/// empty when nothing loaded.
#[test]
fn a_run_that_armed_nothing_refuses_rather_than_reporting_clean() {
  let empty = tempfile::tempdir().expect("tempdir");
  let lib = Library::new(empty.path(), None);
  let report = run(&lib, "shell", &files(), Severity::Warning, &BTreeSet::new())
    .expect("an absent rules tree is an ordinary state, not an error");

  assert_eq!(
    report.exit_code(),
    2,
    "a critic that loaded no rules must refuse: it examined the files against \
     nothing, and 0 would seal a clean verdict over an empty denominator"
  );
}

/// CONTROL, green before and after the fix. Its job is to catch an over-broad
/// fix -- one that refuses whenever nothing was ARMED, or refuses
/// unconditionally -- rather than to demonstrate the defect.
///
/// `shell` arms 2 of 6 in the real library, so a correct run here is 0 or 1
/// depending on findings, and never 2.
#[test]
fn a_loaded_library_never_reports_the_empty_library_refusal() {
  let lib = Library::new(&repo_root(), None);
  let report = run(&lib, "shell", &files(), Severity::Warning, &BTreeSet::new())
    .expect("the repo's own rule library must load");

  assert_ne!(
    report.exit_code(),
    2,
    "a library that loaded {} rule(s) must never produce the empty-library \
     refusal -- if it does, the predicate is reading arming rather than the \
     census",
    report.total()
  );
}

/// The swift and lua case, asserted rather than left to prose: a language whose
/// rules all sit `Undeclared` loads a census and arms nothing, and **that is a
/// healthy install** -- so it must not refuse.
///
/// This is the assertion that would go red if anyone later "simplifies" the
/// predicate to `armed() == 0`.
#[test]
fn a_language_that_arms_nothing_from_a_loaded_library_still_does_not_refuse() {
  let lib = Library::new(&repo_root(), None);
  for lang in ["swift", "lua"] {
    let report = run(&lib, lang, &files(), Severity::Warning, &BTreeSet::new())
      .expect("the repo's own rule library must load");

    assert!(
      report.total() > 0,
      "`{lang}` must load rules from the real library, or this proves nothing"
    );
    assert_eq!(
      report.armed(),
      0,
      "`{lang}`'s rules are all Undeclared today -- if this changes, the test \
       below stops covering the armed-zero-but-healthy case and needs a new \
       subject rather than deletion"
    );
    assert_ne!(
      report.exit_code(),
      2,
      "`{lang}` arms 0 of {} in a CORRECT install -- refusing here would block \
       every commit in every project that declares it",
      report.total()
    );
  }
}
