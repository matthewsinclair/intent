//! **`intent lang` -- declaring a language, not installing one (ST0056 WP-06).**
//!
//! Four verbs ship and one is retired. `list` is byte-identical to v2 and
//! derives from somewhere else; `show`, `init` and `remove` are `corrected`
//! because v2's behaviour was to copy two markdown files per language into
//! `intent/llm/`, and issue 0068 measured 10 of 10 of those byte-identical to
//! their templates with zero readers anywhere in Intent's own tree.
//!
//! # The family is MIXED, and the tests are per verb for that reason
//!
//! `list` and `show` answer from the tool's own registry and are correct outside
//! a project. `init` and `remove` write `intent/.config/config.json` and must
//! refuse outside one (INV-03). `plugin` legitimately takes a family-level
//! exemption from the project gate; taking one here would exempt precisely the
//! two verbs that mutate.

use std::process::{Command, Output};

fn run(args: &[&str]) -> Output {
  Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .output()
    .expect("run the v3 binary")
}

/// Run from a directory that is NOT inside an Intent project, for INV-03.
fn run_outside(args: &[&str]) -> Output {
  Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(std::env::temp_dir())
    .output()
    .expect("run the v3 binary outside a project")
}

fn stdout(out: &Output) -> String {
  String::from_utf8_lossy(&out.stdout).into_owned()
}

fn stderr(out: &Output) -> String {
  String::from_utf8_lossy(&out.stderr).into_owned()
}

/// **ONE CAPABILITY, TWO SPELLINGS, IDENTICAL BYTES -- and this test exists
/// because the first implementation failed it.**
///
/// Bare `intent lang` printed `Usage: lang` and advertised a `help` subcommand
/// that exits 1, while `intent lang --help` printed neither. That is the exact
/// defect `intent version` was raised on, reintroduced in the commit after the
/// one that fixed it, by rendering from the authority instead of asking it.
///
/// The vacuity guard is asserted FIRST: two empty outputs compare equal, so the
/// text must be shown to name the binary and a real verb before the equality is
/// believed.
#[test]
fn bare_lang_and_dash_help_are_one_capability() {
  let bare = run(&["lang"]);
  let flag = run(&["lang", "--help"]);
  assert!(bare.status.success(), "bare `lang` should exit 0");
  assert!(flag.status.success(), "`lang --help` should exit 0");

  let text = stdout(&bare);
  assert!(
    text.contains("intent lang"),
    "usage must name the binary, got: {text}"
  );
  assert!(
    text.contains("list"),
    "usage must name a real verb, got: {text}"
  );

  assert_eq!(
    stdout(&bare),
    stdout(&flag),
    "bare `lang` and `lang --help` disagree"
  );
}

/// Nothing in the family's help advertises a verb that does not answer.
///
/// The `help` subcommand is the concrete instance: clap adds one to an unbuilt
/// command, and `intent st help` exits 1, so printing that line hands the reader
/// a verb that fails.
#[test]
fn the_usage_advertises_no_verb_that_refuses() {
  let text = stdout(&run(&["lang"]));
  // **A FLOOR, BECAUSE THIS LOOP CAN GO VACUOUS AND STILL PASS.** The parsing
  // below skips lines it does not recognise; if the help layout changes it could
  // skip every line and report green over zero checks. Counting what it actually
  // drove is the difference between "no verb refuses" and "no verb was tried".
  let mut checked = 0usize;
  for line in text.lines() {
    let Some(verb) = line
      .strip_prefix("  ")
      .map(|l| l.trim())
      .and_then(|l| l.split_whitespace().next())
    else {
      continue;
    };
    if !text.contains(&format!("\n  {verb} ")) || verb.starts_with('-') {
      continue;
    }
    let out = run(&["lang", verb, "--help"]);
    assert!(
      out.status.success(),
      "`lang` help advertises `{verb}`, which exits {:?}",
      out.status.code()
    );
    checked += 1;
  }
  assert!(
    checked >= 4,
    "only {checked} advertised verb(s) were driven; the parser has gone blind to the help layout"
  );
}

/// `lang list` names the languages a project may declare, and does NOT name the
/// two rule packs nobody declares.
#[test]
fn list_names_the_declarable_languages_and_not_the_shared_packs() {
  let out = run(&["lang", "list"]);
  assert!(out.status.success());
  let text = stdout(&out);
  for lang in [
    "elixir", "rust", "shell", "author", "content", "lua", "swift",
  ] {
    assert!(
      text.contains(lang),
      "`{lang}` missing from `lang list`: {text}"
    );
  }
  for pack in ["agnostic", "prose"] {
    assert!(
      !text.contains(pack),
      "`{pack}` is a shared rule pack, not a declarable language, but `lang list` offers it: {text}"
    );
  }
}

/// **`lang show` MUST NOT CLAIM TO INSTALL A FILE.**
///
/// v2 prints "Files installed by 'intent lang init <lang>'" over two paths. This
/// is the `plugin show` defect in the other direction: not a pointer at a retired
/// verb, but a durable promise that two files will appear when nothing writes
/// them. Both halves are asserted -- that the promise is absent, and that the
/// replacement names the command which actually serves the rules.
#[test]
fn show_promises_no_file_it_does_not_write() {
  let out = run(&["lang", "show", "elixir"]);
  assert!(out.status.success(), "`lang show elixir` should exit 0");
  let text = stdout(&out);
  assert!(
    !text.contains("RULES-elixir.md") && !text.contains("ARCHITECTURE-elixir.md"),
    "`lang show` still promises files that v3 does not write: {text}"
  );
  assert!(
    text.contains("intent claude rules list --lang elixir"),
    "`lang show` must name where the rules actually come from: {text}"
  );
}

/// A pack that exists but cannot be declared is refused BY NAME, and the refusal
/// lists what can be declared instead.
#[test]
fn a_shared_pack_is_refused_rather_than_shown() {
  let out = run(&["lang", "show", "agnostic"]);
  assert_eq!(
    out.status.code(),
    Some(1),
    "an undeclarable name should exit 1"
  );
  let text = stderr(&out);
  assert!(
    text.contains("agnostic"),
    "the refusal must name what was refused: {text}"
  );
  assert!(
    text.contains("lang list"),
    "the refusal must say where the answer is: {text}"
  );
}

/// **`lang sync` IS RETIRED: absent from help AND refused at exit 2.**
///
/// Both halves, because either alone is survivable. A verb missing from help but
/// still answering is undiscoverable rather than retired; a verb that refuses
/// while help still advertises it sends the reader at a wall.
#[test]
fn sync_is_retired_in_both_places() {
  let help = stdout(&run(&["lang"]));
  assert!(
    !help.contains("sync"),
    "`lang sync` is retired but still advertised: {help}"
  );

  let out = run(&["lang", "sync"]);
  assert_eq!(
    out.status.code(),
    Some(2),
    "a retired verb exits 2 (unavailable), not 1 (a negative verdict about the caller's work)"
  );
  assert!(
    stderr(&out).contains("retired"),
    "the refusal should say so: {}",
    stderr(&out)
  );
}

/// **THE MUTATING HALF IS GATED AND THE READING HALF IS NOT.**
///
/// This is the one property a family-level exemption would destroy, so it is
/// asserted as a contrast rather than as two separate facts.
#[test]
fn only_the_verbs_that_write_require_a_project() {
  for read in [vec!["lang", "list"], vec!["lang", "show", "rust"]] {
    let out = run_outside(&read);
    assert!(
      out.status.success(),
      "`{}` reads the tool's own registry and must answer outside a project",
      read.join(" ")
    );
  }
  for write in [vec!["lang", "init", "rust"], vec!["lang", "remove", "rust"]] {
    let out = run_outside(&write);
    assert!(
      !out.status.success(),
      "`{}` writes config.json and must refuse outside a project (INV-03)",
      write.join(" ")
    );
  }
}

/// **A v3 BINARY MUST NOT HALF-MIGRATE A v2 PROJECT FROM A COMMAND ABOUT
/// LANGUAGES -- and this test exists because the first wiring did exactly that.**
///
/// `lang init` / `lang remove` do not need a facade, so the first version reached
/// for `context()`, which discovers a project and stops. The migration gate lives
/// in `Facade::open`, so skipping the facade skipped the gate: on an unmigrated
/// v2 project `intent lang init rust` exited 0 and rewrote `config.json` into v3
/// shape -- `author`, `intent_dir` and the `todo` block added, while
/// `intent_version` still read `2.19.0`.
///
/// **THE FILE IS COMPARED, NOT JUST THE EXIT CODE.** A refusal that still wrote
/// would pass an exit-code assertion, and the write was the whole of the harm.
#[test]
fn a_language_command_does_not_migrate_a_v2_project_behind_the_operator() {
  let dir = std::env::temp_dir().join(format!("intent-lang-unmigrated-{}", std::process::id()));
  let cfg_dir = dir.join("intent").join(".config");
  std::fs::create_dir_all(&cfg_dir).expect("fixture dirs");
  let cfg = cfg_dir.join("config.json");
  let before = "{\n  \"intent_version\": \"2.19.0\",\n  \"languages\": [\"elixir\"]\n}\n";
  std::fs::write(&cfg, before).expect("write the v2 config");

  for args in [
    vec!["lang", "init", "rust"],
    vec!["lang", "remove", "elixir"],
  ] {
    let out = Command::new(env!("CARGO_BIN_EXE_intent"))
      .args(&args)
      .current_dir(&dir)
      .output()
      .expect("run the v3 binary in an unmigrated project");
    assert!(
      !out.status.success(),
      "`{}` answered over an unmigrated project",
      args.join(" ")
    );
    assert!(
      stderr(&out).contains("has not been migrated"),
      "the refusal must be the migration gate's, not an incidental error: {}",
      stderr(&out)
    );
    let after = std::fs::read_to_string(&cfg).expect("read the config back");
    assert_eq!(
      before,
      after,
      "`{}` refused and wrote anyway -- the exit code is not the harm, the write is",
      args.join(" ")
    );
  }
  let _ = std::fs::remove_dir_all(&dir);
}
