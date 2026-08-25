//! **`intent modules` -- the Highlander registry's own verbs (ST0056 WP-06).**
//!
//! Two verbs ship and one flag is retired. `find` and `check` are both reads;
//! `--register` is gone, and its going is what earns this family the simple
//! shape the tests below can assume.
//!
//! # The two findings this family was built on
//!
//! **`find` on no match exits 1, and two documents said 0.** The dispatch
//! table's `observed` block carried "found, or no match -> 0" with an evidence
//! class of `read`, and issue 0067 records `rc=0` as MEASURED. Driven against
//! the frozen v2.19.0 install on a fixture, a non-matching term is rc=1 --
//! `cmd_find` returns 1 on the empty branch and always has. Two documents
//! agreed with each other and neither agreed with the program.
//!
//! **`check`'s population is derived from the declared `languages`.** v2 scans
//! `bin/intent_*` for every project it is pointed at, which is Intent's own
//! layout with this product's name as a file prefix. On a project written in
//! anything else the check could not fire, and a check that cannot fire reads
//! exactly like one that passed.

use std::process::{Command, Output};

fn run(args: &[&str]) -> Output {
  Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .output()
    .expect("run the v3 binary")
}

fn run_in(dir: &std::path::Path, args: &[&str]) -> Output {
  Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(dir)
    .output()
    .expect("run the v3 binary")
}

fn stdout(out: &Output) -> String {
  String::from_utf8_lossy(&out.stdout).into_owned()
}

/// A migrated v3 project with a registry, a source file and a declared language.
fn project(languages: &str, registry: &str) -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  std::fs::create_dir_all(root.join("intent/.config")).expect("mkdir");
  std::fs::create_dir_all(root.join("intent/llm")).expect("mkdir");
  std::fs::write(
    root.join("intent/.config/config.json"),
    format!(
      r#"{{"intent_version":"3.0.0","project_name":"Fixture","author":"cc","intent_dir":"intent","languages":[{languages}]}}"#
    ),
  )
  .expect("write config");
  std::fs::write(root.join("intent/llm/MODULES.md"), registry).expect("write registry");
  dir
}

const EMPTY_REGISTRY: &str = "| Concern | THE Module | Notes |\n| --- | --- | --- |\n";

/// **ONE CAPABILITY, TWO SPELLINGS, IDENTICAL BYTES.**
///
/// The `version` defect, which `lang` then reintroduced one commit after it was
/// fixed. The vacuity guard runs first: two empty strings compare equal, so the
/// text must be shown to name a real verb before the equality means anything.
#[test]
fn bare_modules_and_dash_help_are_one_capability() {
  let bare = run(&["modules"]);
  let flag = run(&["modules", "--help"]);
  assert!(bare.status.success(), "bare `modules` should exit 0");
  assert!(flag.status.success(), "`modules --help` should exit 0");
  let text = stdout(&bare);
  assert!(text.contains("find"), "usage names no verb: {text}");
  assert!(text.contains("check"), "usage names no verb: {text}");
  assert_eq!(text, stdout(&flag), "bare and --help disagree");
}

/// Every verb the usage advertises must answer. `lang`'s first implementation
/// advertised a `help` subcommand that exits 1.
#[test]
fn the_usage_advertises_no_verb_that_refuses() {
  let text = stdout(&run(&["modules"]));
  let mut checked = 0usize;
  for line in text.lines() {
    let trimmed = line.trim_start();
    for verb in ["find", "check", "help"] {
      if trimmed.starts_with(verb) {
        // `find` needs an argument; ask it for its own help instead, which is
        // the question "does this verb exist" without the argument noise.
        let out = run(&["modules", verb, "--help"]);
        assert!(
          out.status.success(),
          "usage advertises `{verb}`, which exits {:?}",
          out.status.code()
        );
        checked += 1;
      }
    }
  }
  assert!(
    checked >= 2,
    "the scan matched {checked} verbs; it is vacuous"
  );
}

/// **THE TABLE SAID 0 AND THE PROGRAM SAYS 1.**
///
/// The message is on stdout and the failure is silent, so nothing on stderr
/// contradicts the line on stdout. v2's shape exactly, and grep's convention.
#[test]
fn a_term_that_matches_nothing_exits_one_and_says_so_on_stdout() {
  let dir = project(
    r#""shell""#,
    "| Concern | THE Module | Notes |\n| --- | --- | --- |\n| Helpers | `bin/helpers` | shared |\n",
  );
  let out = run_in(dir.path(), &["modules", "find", "zzzznope"]);
  assert_eq!(out.status.code(), Some(1), "no match must exit 1");
  assert!(
    stdout(&out).contains("no matches for 'zzzznope'"),
    "stdout was {:?}",
    stdout(&out)
  );
  assert!(
    out.stderr.is_empty(),
    "the verdict is the rc; stderr should be silent, got {:?}",
    String::from_utf8_lossy(&out.stderr)
  );

  // The positive control: the same fixture, a term that IS there. Without this
  // the test above passes for a `find` that can never match anything.
  let hit = run_in(dir.path(), &["modules", "find", "helpers"]);
  assert!(hit.status.success(), "a matching term must exit 0");
  assert!(stdout(&hit).contains("bin/helpers"));
}

/// **A CHECK THAT COULD NOT FIRE MUST NOT READ AS ONE THAT PASSED.**
///
/// A project declaring only prose languages has no source population. The
/// registry matches, so there is genuinely nothing to report -- and reporting
/// only `ok:` would be the estate's recurring defect in one line.
#[test]
fn check_states_what_it_scanned_before_it_states_a_verdict() {
  let prose = project(r#""author""#, EMPTY_REGISTRY);
  let out = run_in(prose.path(), &["modules", "check"]);
  assert!(out.status.success(), "nothing to find, so nothing is wrong");
  let text = stdout(&out);
  assert!(
    text.contains("no declared language contributes a source population"),
    "an empty population was not stated: {text}"
  );

  // And the other half: a real population says what it scanned, so the reader
  // has the denominator before the count.
  let code = project(r#""shell""#, EMPTY_REGISTRY);
  std::fs::create_dir_all(code.path().join("bin")).expect("mkdir");
  std::fs::write(code.path().join("bin/thing"), "#!/bin/bash\n").expect("write");
  let out = run_in(code.path(), &["modules", "check"]);
  assert_eq!(
    out.status.code(),
    Some(1),
    "an unregistered file is an issue"
  );
  let text = stdout(&out);
  assert!(text.contains("note: scanned"), "no denominator: {text}");
  assert!(text.contains("shell (1)"), "wrong denominator: {text}");
  assert!(
    text.contains("+ bin/thing"),
    "the finding is not named: {text}"
  );
}

/// `--register` is retired, so the flag the surface once carried must not be
/// reachable -- and the family must stay a pure read because of it.
#[test]
fn the_interactive_flag_is_retired_in_both_places() {
  let table: serde_json::Value =
    serde_json::from_str(intent_cli::dispatch::TABLE).expect("parse the dispatch table");
  let mut seen = false;
  for family in table["families"].as_array().expect("families") {
    for entry in family["entries"].as_array().expect("entries") {
      if entry["path"] == "modules check" {
        seen = true;
        for flag in entry["flags"].as_array().expect("flags") {
          let spellings = flag["spellings"].as_array().expect("spellings");
          if spellings.iter().any(|s| s == "--register") {
            assert_eq!(
              flag["disposition"], "retire",
              "`--register` is still shipped by the table"
            );
          }
        }
      }
    }
  }
  assert!(seen, "`modules check` is missing from the table");

  // The binary is the other place, and it is the one a caller meets.
  let dir = project(r#""shell""#, EMPTY_REGISTRY);
  let out = run_in(dir.path(), &["modules", "check", "--register"]);
  assert!(
    !out.status.success(),
    "`--register` still parses: {:?}",
    stdout(&out)
  );
}

/// Both verbs read a PROJECT file, so neither takes the migration exemption
/// `plugin` and `lang list` take. The fallback is written down: `CLAUDE.md`
/// tells an agent to drive `intent modules find` and to fall back to a grep if
/// it does not answer.
#[test]
fn a_modules_verb_refuses_an_unmigrated_project() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  std::fs::create_dir_all(root.join("intent/.config")).expect("mkdir");
  std::fs::create_dir_all(root.join("intent/llm")).expect("mkdir");
  std::fs::write(
    root.join("intent/.config/config.json"),
    r#"{"intent_version":"2.19.0","project_name":"Legacy"}"#,
  )
  .expect("write config");
  std::fs::write(root.join("intent/llm/MODULES.md"), EMPTY_REGISTRY).expect("write registry");

  for args in [
    vec!["modules", "find", "anything"],
    vec!["modules", "check"],
  ] {
    let out = run_in(root, &args);
    assert!(
      !out.status.success(),
      "{args:?} answered from an unmigrated project"
    );
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(
      err.contains("upgrade"),
      "{args:?} refused without naming the remedy: {err}"
    );
  }
}
