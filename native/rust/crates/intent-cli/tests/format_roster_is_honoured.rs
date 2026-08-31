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

fn bin() -> std::path::PathBuf {
  std::path::PathBuf::from(env!("CARGO_BIN_EXE_intent"))
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
  // **THIS ENTRY WAS VACUOUS FOR FOURTEEN DAYS AND ITS OWN NOTE SAID SO WHILE
  // BEING WRONG ABOUT WHY.** The note read: *`daemon status` is DECLARED AND
  // UNWIRED, it returns the unwired marker at rc=2 for every argv ... it
  // becomes a real drive the moment cc wires the arm -- nothing here needs
  // changing then.* **The wiring landed at `e6aba646` and nothing here
  // changed, because nothing had to**: the verb then answered `ok:` at rc=0
  // for EVERY value of `--format`, having never read the flag. The pass
  // condition is *matches neither refusal pattern*, and a wired verb ignoring
  // its flag matches neither pattern either -- **so the arm went
  // vacuous-because-UNWIRED to vacuous-because-UNREAD with the same green and
  // no signal.** The promise did not fire because the sentence *nothing here
  // needs changing then* was the part that was wrong.
  //
  // **IT IS A REAL DRIVE NOW** (cc, 2026-08-31): `daemon_status` reads the
  // flag through `enum_flag`, the roster is narrowed to `terminal`, and
  // `the_daemon_status_lookup_resolves_and_refuses_by_name` below drives the
  // refusal rather than leaving acceptance to stand for both verdicts.
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

/// **THE `daemon status` ROSTER LOOKUP RESOLVES, AND THE REFUSAL NAMES THE ROW.**
///
/// `render.rs` spells the literal `"daemon status"` to look its roster up:
/// `enum_flag(a, "daemon status", "--format")`. **That literal is the first
/// MULTI-token command path the roster scanner has ever met** -- `doctor`'s is
/// a single token and goes unscanned -- so it is declared in
/// `command_rosters_are_derived_or_declared.rs`, and it is declared
/// `CheckedBy` **this arm**.
///
/// **WHICH MAKES THE EXIT CODE THE ASSERTION, NOT THE MESSAGE.** `enum_flag`
/// has two failure branches and they are not the same fact:
///
/// - the label does NOT resolve in the table -> `Failure::Unavailable`, **rc
///   2**, *the dispatch table declares no values for `--format` on ...*. That
///   is the typo case, and it is what this arm exists to catch.
/// - the label resolves and the VALUE is undeclared -> `Failure::Error`, **rc
///   1**, naming the row and what it takes.
///
/// So asserting rc 1 is asserting that the lookup found the row. **A test that
/// only asserted "it refused" would pass on the typo**, because a mistyped
/// label refuses too -- more loudly, and about the wrong thing. That
/// distinction is the whole reason this can be classified `CheckedBy` rather
/// than merely tolerated: vc's condition was that a runtime check nothing
/// exercises is a guard nobody runs.
#[test]
fn the_daemon_status_lookup_resolves_and_refuses_by_name() {
  let p = seeded();
  let (out, code) = run(p.path(), &["daemon", "status", "--format", "zzz"]);

  assert_eq!(
    code, 1,
    "an undeclared `--format` value must refuse at 1 -- a 2 here means the literal \
     `daemon status` in `render.rs` no longer resolves to a table row, so the roster lookup \
     is broken rather than the value being wrong: {out}"
  );
  assert!(
    !out.contains("declares no values"),
    "`enum_flag` took its label-did-not-resolve branch, so `render.rs` and the dispatch table \
     disagree about how this row is spelled: {out}"
  );
  assert!(
    out.contains("daemon status") && out.contains("zzz"),
    "the refusal must name the row it refused for and the value it refused: {out}"
  );

  // **AND THE DECLARED VALUE STILL WORKS**, so the arm above is not passing
  // because the verb refuses everything -- which is the shape a broken
  // `enum_flag` would produce and is indistinguishable from correctness
  // without this line.
  let (_, ok_code) = run(p.path(), &["daemon", "status", "--format", "terminal"]);
  assert_eq!(
    ok_code, 0,
    "the one declared value must still be accepted, or the refusal above proves nothing"
  );
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
