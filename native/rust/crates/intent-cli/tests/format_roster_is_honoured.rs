//! **Every value a `--format` row DECLARES, that verb ACCEPTS.**
//!
//! `flag.value` in `surface/dispatch-table.json` is a pipe-string roster --
//! `terminal|md|json` -- and it reaches clap as `value_name` and nothing else.
//! Nothing parses it, so it is documentation that `--help` prints verbatim, and
//! `declared_values_are_enforced.rs` cannot see it: that file walks `values`
//! ARRAYS, and this is the other encoding.
//!
//! **So the drift ran unobserved and in both directions at once.** Measured
//! 2026-08-27: five verbs advertised `json` in `--help` and refused it at exit
//! 1 with `this verb has no json projection`, because `Output::table` yields
//! `None` for JSON by design -- a list-of-lists is not the object anyone means,
//! and a verb with a real projection branches earlier. The code was right and
//! the declaration was wrong, and no instrument compared the two.
//!
//! # The fixture must have CONTENT, and that is the whole care in this file
//!
//! `--format` is validated as an ARGUMENT to the renderer, so a verb whose
//! result set is empty returns before it ever looks at the flag. **A drive
//! against a fresh project therefore passes for every value, including values
//! the verb refuses** -- vacuously, silently, and looking exactly like
//! coverage. That is the same class this estate keeps meeting: a subject that
//! cannot exhibit the defect cannot clear it. So the fixture seeds a thread, a
//! work package and an issue first, and `it_can_fail` proves the drive can go
//! red before any green is read as evidence.

use std::path::Path;
use std::process::Command;

use testkit::workspace_root;

fn bin() -> std::path::PathBuf {
  workspace_root().join("target/debug/intent")
}

/// A project with a thread, a work package and an issue in it.
fn seeded() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  run(dir.path(), &["init", "rosters"]);
  run(dir.path(), &["st", "new", "a thread"]);
  run(dir.path(), &["wp", "new", "ST0001", "a package"]);
  run(dir.path(), &["issues", "add", "an issue"]);
  dir
}

fn run(cwd: &Path, args: &[&str]) -> (String, i32) {
  let out = Command::new(bin())
    .args(args)
    .current_dir(cwd)
    .env("HOME", testkit::fixture_home())
    .output()
    .expect("run intent");
  (
    format!(
      "{}{}",
      String::from_utf8_lossy(&out.stdout),
      String::from_utf8_lossy(&out.stderr)
    ),
    out.status.code().unwrap_or(-1),
  )
}

/// The argv for each verb that declares a `--format` roster.
///
/// **An unlisted row FAILS rather than being skipped.** A verb added to the
/// table with a roster and no entry here is a slot nobody decided about, which
/// is the same refusal `declared_values_are_enforced.rs` gives an undeclared
/// disposition.
const ARGV: &[(&str, &[&str])] = &[
  // **`--status all`, AND THE BARE FORM WAS VACUOUS HERE FROM THE DAY THIS FILE
  // WAS WRITTEN.** The fixture seeds a thread, `st new` enters it at `Triage`,
  // and bare `st list` narrows to WIP -- so every `st list --format <value>`
  // drive below ran over an EMPTY result set, which is precisely the vacuity
  // this file's header says the fixture exists to prevent. It rendered as a
  // header-and-separator rather than as a sentence, so the non-empty control
  // below could not see it; issue 0121's disclosure made it visible and the
  // control fired the same hour.
  // **DECLARED RATHER THAN SKIPPED, AND THE DRIVE IS CURRENTLY VACUOUS -- SAY
  // SO.** `daemon status` is DECLARED AND UNWIRED: it returns the `known command
  // that is not implemented yet` marker at rc=2 for every argv, so this drive
  // cannot yet distinguish a format the verb accepts from one it refuses. It
  // passes here because the marker matches neither refusal pattern, which is a
  // pass BY NOT MATCHING rather than by being right. The entry is here because
  // this file refuses to be skipped, and the honest state is recorded rather
  // than left to look like coverage. **It becomes a real drive the moment cc
  // wires the arm** (WP-08) -- nothing here needs changing then.
  ("daemon status", &["daemon", "status"]),
  ("st list", &["st", "list", "--status", "all"]),
  ("st sync", &["st", "sync"]),
  ("wp list", &["wp", "list", "ST0001"]),
  ("issues", &["issues"]),
  ("issues list", &["issues", "list"]),
  ("issues show", &["issues", "show", "0001"]),
  ("todo", &["todo"]),
  ("todo list", &["todo", "list"]),
  ("doctor", &["doctor"]),
  ("critic", &["critic", "rust"]),
];

/// Every `(path, declared value)` pair the table carries for `--format`.
fn declared() -> Vec<(String, String)> {
  let table: serde_json::Value =
    serde_json::from_str(intent_cli::dispatch::TABLE).expect("the table parses");
  let mut out = Vec::new();
  for fam in table["families"].as_array().expect("families") {
    for e in fam["entries"].as_array().into_iter().flatten() {
      let path = e["path"].as_str().unwrap_or_default().to_string();
      for f in e["flags"].as_array().into_iter().flatten() {
        let names = f["spellings"].as_array();
        let is_format = names.is_some_and(|n| n.iter().any(|s| s.as_str() == Some("--format")));
        let Some(value) = f["value"].as_str() else {
          continue;
        };
        if is_format && value.contains('|') {
          for v in value.split('|') {
            out.push((path.clone(), v.to_string()));
          }
        }
      }
    }
  }
  out
}

fn argv_for(path: &str) -> &'static [&'static str] {
  ARGV
    .iter()
    .find(|(p, _)| *p == path)
    .map(|(_, a)| *a)
    .unwrap_or_else(|| {
      panic!("`{path}` declares a --format roster and this file has no argv for it -- decide about it rather than skipping it")
    })
}

/// **THE CONTROL, AND IT RUNS FIRST.** A drive that cannot go red would report
/// the whole roster green whatever the binary did.
#[test]
fn it_can_fail() {
  let p = seeded();
  let (out, code) = run(p.path(), &["issues", "list", "--format", "bogus"]);
  assert_ne!(code, 0, "an undeclared value must be refused: {out}");
  assert!(out.contains("is not a format"), "{out}");
}

/// The fixture is genuinely non-empty, so the drives below reach the renderer
/// rather than returning early. Without this the suite is vacuous.
#[test]
fn the_fixture_has_content_to_render() {
  let p = seeded();
  for (what, args) in [
    // The same `--status all` as the roster drive uses, and for the same
    // reason: the bare form narrows away the one thread the fixture seeds.
    ("a thread", &["st", "list", "--status", "all"][..]),
    ("a package", &["wp", "list", "ST0001"][..]),
    ("an issue", &["issues", "list"][..]),
  ] {
    let (out, code) = run(p.path(), args);
    assert_eq!(code, 0, "{what}: {out}");
    assert!(
      !out.contains("no ") || out.contains("ST0001"),
      "{what}: the fixture rendered nothing, so a --format drive over it proves nothing: {out}"
    );
  }
}

#[test]
fn every_declared_format_value_is_accepted_by_the_verb_that_declares_it() {
  let p = seeded();
  let mut refused = Vec::new();
  let pairs = declared();
  assert!(
    !pairs.is_empty(),
    "no --format rosters found -- the walk is broken, not the surface"
  );

  for (path, value) in &pairs {
    let mut args: Vec<&str> = argv_for(path).to_vec();
    args.push("--format");
    args.push(value);
    let (out, code) = run(p.path(), &args);
    if code != 0 && (out.contains("is not a format") || out.contains("no json projection")) {
      refused.push(format!("  `{path} --format {value}` -> {}", out.trim()));
    }
  }

  assert!(
    refused.is_empty(),
    "{} of {} declared --format value(s) are advertised by `--help` and refused by the verb:\n{}",
    refused.len(),
    pairs.len(),
    refused.join("\n")
  );
}
