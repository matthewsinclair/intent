//! **ST0058 `AC-00.5`: a retired command is ENUMERABLE, so a caller can tell
//! *gone forever* from *coming soon* without parsing prose and without a new
//! exit code.**
//!
//! The criterion states its own falsifier, and this file is that falsifier run
//! as a test rather than transcribed as a drive: *a caller distinguishing a
//! retired command from an unbuilt one requiring either a new exit code or the
//! parsing of message text. The list is the interface; if it cannot be read
//! mechanically, this is unsatisfied.*
//!
//! **A TRANSCRIPT OF A DRIVE PROVES THE PROPERTY ONCE, ON ONE TREE, ON ONE
//! MACHINE.** The estate has spent this week correcting documents that claimed
//! a capability the shipped binary did not have -- so the evidence for a row
//! about mechanical readability should itself be mechanical, and should re-run
//! on every commit rather than sit in a criterion's evidence field ageing.
//!
//! # The no-prose property is STRUCTURAL here, not a promise
//!
//! [`classify`] takes an exit code and a membership set. It has no third
//! parameter, so no arm of this file can quietly begin reading a message body
//! and still compile -- the discipline the criterion demands of a caller is
//! enforced on the test by its own signature. That is deliberate: a comment
//! saying *we do not parse the message* is worth nothing next to a function
//! that cannot.

use std::process::Command;

use intent_cli::{dispatch, spine};

/// Everything a caller can learn about a command it was refused.
#[derive(Debug, PartialEq, Eq)]
enum Verdict {
  /// Retired: it existed, it is gone, and no build will bring it back.
  GoneForever,
  /// Known to the surface and not implemented in this build.
  ComingSoon,
  /// Not a refusal at all -- carried so a control can assert the classifier is
  /// not a constant.
  NotARefusal(i32),
}

/// **The criterion, expressed as a signature.** Two channels in, a verdict
/// out; the message body is not among the parameters.
fn classify(code: i32, roster: &[String], path: &str) -> Verdict {
  if code != 2 {
    return Verdict::NotARefusal(code);
  }
  if roster.iter().any(|p| p == path) {
    Verdict::GoneForever
  } else {
    Verdict::ComingSoon
  }
}

/// Drives the real binary from a directory with no project in it.
///
/// **Outside a project deliberately.** Both refusals under test fire at
/// spelling-match time, ahead of `Facade::open`, so a project fixture would add
/// a variable without adding coverage -- and a caller who typed a retired verb
/// is very often standing outside one.
fn run(argv: &[&str]) -> i32 {
  let dir = tempfile::tempdir().expect("tempdir");
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(argv)
    .current_dir(dir.path())
    .env("HOME", testkit::fixture_home())
    .output()
    .expect("run the v3 binary");
  out.status.code().unwrap_or(-1)
}

/// **A REGISTER PATH IS A SPELLING, NOT AN ARGUMENT LIST.**
///
/// `st repair` is TWO argv elements. Passed as one it is an unrecognized
/// subcommand at rc=1 -- which is indistinguishable from a command that never
/// existed, so the first run of this test reported the retirement refusal as
/// absent on every multi-word row. **The strict `== 2` below is what caught
/// it**; a laxer assertion would have recorded the refusal as working.
fn run_path(path: &str) -> i32 {
  let argv: Vec<&str> = path.split_whitespace().collect();
  run(&argv)
}

/// Drives a register path and returns the code AND what it said.
///
/// **THE ORACLE MAY READ PROSE; THE CLASSIFIER MAY NOT.** [`classify`] models
/// the caller and is held to two channels. This function is the test's own
/// evidence, held to no such rule -- constraining the oracle to the same
/// channels as the instrument would leave nothing to check the instrument
/// against.
fn run_saying(path: &str) -> (i32, String) {
  let argv: Vec<&str> = path.split_whitespace().collect();
  let dir = tempfile::tempdir().expect("tempdir");
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(&argv)
    .current_dir(dir.path())
    .env("HOME", testkit::fixture_home())
    .output()
    .expect("run the v3 binary");
  (
    out.status.code().unwrap_or(-1),
    String::from_utf8_lossy(&out.stderr).to_string(),
  )
}

/// The stderr marker of a verb the surface declares and this build does not
/// implement.
///
/// **THIS IS THE STRING'S THIRD HOME AND THE DEBT IS REAL.** It is built inline
/// at `render.rs:767` and copied again into `flag_reachability.rs`; the right
/// fix is one `pub const` in `render.rs` that all three read. Not taken here on
/// purpose: that edit widens into a file carrying another node's live work
/// while the estate's delivered binary is refusing, and a refactor of a shared
/// source is the wrong thing to spend that moment on. Named rather than
/// silently added.
const UNWIRED_MARKER: &str = "is a known command that is not implemented yet";

/// The roster as a CALLER reads it: off stdout, as JSON, parsed.
fn published_roster() -> Vec<String> {
  let dir = tempfile::tempdir().expect("tempdir");
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(["surface", "retired", "--json"])
    .current_dir(dir.path())
    .env("HOME", testkit::fixture_home())
    .output()
    .expect("run the v3 binary");
  assert_eq!(
    out.status.code(),
    Some(0),
    "`surface retired --json` must answer; a roster that refuses is not an interface"
  );
  let parsed: serde_json::Value =
    serde_json::from_slice(&out.stdout).expect("the roster must be well-formed JSON on stdout");
  parsed
    .as_array()
    .expect("the roster is a JSON array")
    .iter()
    .map(|row| {
      row
        .get("path")
        .and_then(serde_json::Value::as_str)
        .expect("every roster row names a path")
        .to_string()
    })
    .collect()
}

#[test]
fn a_retired_command_and_an_unbuilt_one_are_separable_without_reading_a_message() {
  let roster = published_roster();
  let table = dispatch::table();

  // **BOTH POPULATIONS ARE DERIVED, NEVER LISTED HERE.** A hand-written sample
  // is the defect this estate keeps finding: it ages into a list that no longer
  // describes the surface, and the test goes on passing over whatever is left
  // of it.
  let retired: Vec<String> = roster.clone();

  // Shipped, single-token, argument-free rows are the ones a caller can type
  // bare -- which is what makes them drivable without the placeholder-argument
  // machinery that the unmigrated sweep needs. Any that refuses at 2 and is
  // absent from the roster is an unbuilt command by the caller's own reckoning.
  let unbuilt: Vec<String> = dispatch::shipped_entries(&table)
    .into_iter()
    .filter(|e| !e.path.contains(' ') && e.args.is_empty())
    .map(|e| e.path.clone())
    .filter(|p| run_path(p) == 2 && !roster.contains(p))
    .collect();

  // **NON-VACUITY, ASSERTED BEFORE ANY VERDICT IS CHECKED.** Two empty
  // populations satisfy every assertion below and would report a pass on a
  // binary with no commands in it at all. If the unbuilt arm ever empties
  // legitimately -- every declared verb built -- this row's discrimination
  // stops being testable and someone must come and read it, which is what a red
  // here is asking for.
  assert!(
    !retired.is_empty(),
    "the published roster is empty, so nothing here tests membership"
  );
  assert!(
    !unbuilt.is_empty(),
    "no shipped command refuses at 2 while absent from the roster, so the COMING SOON arm is \
     untested -- if every declared verb is now built, this test needs re-reading rather than \
     re-running"
  );

  // **THE EXIT CODE MUST NOT DISCRIMINATE.** Asserted rather than assumed: if a
  // later change gave retirement its own code, the criterion would be met by
  // the means it explicitly rules out, and every assertion below would still
  // pass.
  for path in retired.iter().chain(unbuilt.iter()) {
    assert_eq!(
      run_path(path),
      2,
      "`intent {path}` must refuse with the shared code -- retirement earning its own exit code \
       is the outcome this criterion was written to prevent"
    );
  }

  for path in &retired {
    assert_eq!(
      classify(2, &roster, path),
      Verdict::GoneForever,
      "`intent {path}` is retired and must read as GONE FOREVER from the code plus the list alone"
    );
  }
  for path in &unbuilt {
    assert_eq!(
      classify(2, &roster, path),
      Verdict::ComingSoon,
      "`intent {path}` is unbuilt and must read as COMING SOON from the code plus the list alone"
    );
  }

  // **THE POPULATION MUST NOT ABSORB A REFUSAL CLASS IT WAS NOT BUILT FOR.**
  // `unbuilt` is defined by a PREDICATE -- refuses at 2, absent from the roster
  // -- and a predicate admits whatever later arrives matching it. That is not
  // hypothetical: while this file was being written, an uncommitted
  // `SCHEMA_VERSION` bump left every older binary refusing on the runtime
  // store, and had that refusal carried 2 instead of 1 a LIVE verb would have
  // entered this population and read as COMING SOON.
  //
  // **`rc=2` CARRIES FOUR MEANINGS IN THIS ESTATE AND ONLY PROSE SEPARATES
  // THEM** -- ST0058 `AC-00.5` says so in as many words, and it made only
  // RETIREMENT enumerable. The other three are still told apart by their
  // message, so the message is the only oracle available and the register
  // cannot help: it carries no built-ness field, by construction.
  // (cc asked for this arm, 2026-08-29, having been bitten twice this week by a
  // predicate-defined population silently gaining a member.)
  for path in &unbuilt {
    let (code, said) = run_saying(path);
    assert!(
      code == 2 && said.contains(UNWIRED_MARKER),
      "`intent {path}` entered the unbuilt population on the predicate alone, and it is not an \
       unbuilt verb -- it exited {code} saying: {said}. A refusal class that is not \
       not-implemented-yet has started answering at 2, and every member it contributes reads as \
       COMING SOON to a caller."
    );
  }

  // **THE CONTROL.** A classifier that answered GONE FOREVER for everything
  // would pass both loops above. `version` is tool-level, answers outside a
  // project, and is neither retired nor unbuilt.
  assert_eq!(
    classify(run(&["version"]), &roster, "version"),
    Verdict::NotARefusal(0),
    "a live command must fall out of both classes, or the classifier is a constant"
  );
}

#[test]
fn the_published_roster_and_the_refusal_path_read_one_definition() {
  // **TWO DOORS ONTO ONE ROSTER IS THE DESIGN; TWO ANSWERS TO *IS THIS
  // RETIRED* IS THE DIVERGENT-COPY SHAPE.** The refusal a caller meets and the
  // list they consult afterwards must agree, or the list is advice about a
  // different binary.
  //
  // It matters more than it looks: `table.retired()` alone names `organize`,
  // which this build RECLAIMED and which RUNS. A roster built from disposition
  // alone would tell an operator to strip a working command out of their
  // scripts.
  let table = dispatch::table();
  let in_process: Vec<String> = spine::retired_and_unreachable(&table)
    .into_iter()
    .map(|(e, _)| e.path.clone())
    .collect();
  assert_eq!(
    published_roster(),
    in_process,
    "the roster on stdout and the one the exec path refuses from must be the same list"
  );
}
