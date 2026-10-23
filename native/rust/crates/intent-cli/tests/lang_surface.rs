//! **`intent lang` -- declaring a language, not installing one (ST0056 WP-06).**
//!
//! The family's verbs ship bar `sync`, which is retired. `list` is
//! byte-identical to v2 and derives from somewhere else; `show`, `init` and
//! `remove` are `corrected` because v2's behaviour was to copy markdown files
//! per language into `intent/llm/`, and issue 0068 measured every one of those
//! byte-identical to its template with no readers anywhere in Intent's own
//! tree.
//!
//! # The family is MIXED, and the tests are per verb for that reason
//!
//! `list` and `show` answer from the tool's own registry and are correct outside
//! a project. `init` and `remove` write `intent/.config/config.json` and must
//! refuse outside one (INV-03). `plugin` legitimately takes a family-level
//! exemption from the project gate; taking one here would exempt precisely the
//! two verbs that mutate.

use std::process::Output;

fn run(args: &[&str]) -> Output {
  crate::common::intent()
    .args(args)
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run the v3 binary")
}

/// Run from a directory that is NOT inside an Intent project, for INV-03.
fn run_outside(args: &[&str]) -> Output {
  crate::common::intent()
    .args(args)
    .current_dir(std::env::temp_dir())
    .stdin(testkit::lifeline_for(args))
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
/// asserted as a contrast rather than as two separate facts. `declared` sits
/// outside the contrast: it reads the project's config, so it needs a project
/// and writes nothing, and its own arm below holds its refusal.
#[test]
fn only_the_verbs_that_write_require_a_project() {
  for read in [
    vec!["lang", "list"],
    vec!["lang", "show", "rust"],
    vec!["lang", "of", "foo.rs"],
  ] {
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
    let out = crate::common::intent()
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

/// A v3 project in a temp directory declaring `languages` as written (a JSON
/// array), with a subdirectory to run from.
fn declaring(languages: &str) -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let config = dir.path().join("intent").join(".config");
  std::fs::create_dir_all(&config).expect("mkdir");
  std::fs::create_dir_all(dir.path().join("src").join("deep")).expect("mkdir the subdirectory");
  std::fs::write(
    config.join("config.json"),
    format!(
      "{{\n  \"intent_version\": \"3.0.0\",\n  \"project_name\": \"Declared\",\n  \"author\": \"cc\",\n  \"intent_dir\": \"intent\",\n  \"languages\": {languages}\n}}\n"
    ),
  )
  .expect("write config");
  dir
}

fn declared_from(dir: &std::path::Path) -> Output {
  crate::common::intent()
    .args(["lang", "declared"])
    .current_dir(dir)
    .stdin(testkit::lifeline_for(&["lang", "declared"]))
    .output()
    .expect("run the v3 binary")
}

/// **`lang declared` ANSWERS FROM ANY DIRECTORY IN THE PROJECT, IN CONFIG
/// ORDER** (issue 0618). The five jq readers it replaces each read
/// `intent/.config/config.json` relative to the working directory, so each
/// worked from the root alone; discovery walks up. The order is not
/// alphabetical, so a sorted answer fails.
#[test]
fn declared_prints_the_configs_languages_in_order_from_a_subdirectory() {
  let dir = declaring(r#"["shell", "rust", "elixir"]"#);
  let out = declared_from(&dir.path().join("src").join("deep"));
  assert!(out.status.success(), "stderr: {}", stderr(&out));
  assert_eq!(stdout(&out), "shell\nrust\nelixir\n");
}

/// `languages: []` is a legitimate state (decision 62(1)), so it prints nothing
/// at 0 rather than refusing.
#[test]
fn an_empty_declaration_prints_nothing_at_zero() {
  let dir = declaring("[]");
  let out = declared_from(dir.path());
  assert!(out.status.success(), "stderr: {}", stderr(&out));
  assert_eq!(stdout(&out), "");
}

/// Outside a project, `declared` refuses with discovery's own words and remedy,
/// at 4, could not judge, as `doctor` does: the commit gate fails open on 4 and
/// refuses on 1 (vc's ruling, 2026-10-23).
#[test]
fn declared_outside_a_project_refuses_with_discoverys_remedy() {
  let out = run_outside(&["lang", "declared"]);
  assert_eq!(out.status.code(), Some(4), "stdout: {}", stdout(&out));
  let err = stderr(&out);
  assert!(err.contains("error: no Intent project found"), "{err}");
  assert!(err.contains("remedy: run `intent init` here"), "{err}");
}

/// **A CONFIG THE BINARY CANNOT READ IS COULD-NOT-JUDGE, 4** (vc's ruling,
/// 2026-10-23), with the parse error's position and remedy, and no language
/// printed: the gate fails open on it, saying so, where it refuses on 1. Text
/// that is not JSON is one; a well-formed config whose `languages` serde
/// refuses is a declaration read and refused, 1.
#[test]
fn declared_on_a_config_that_is_not_json_answers_four() {
  let dir = declaring("[]");
  std::fs::write(
    dir
      .path()
      .join("intent")
      .join(".config")
      .join("config.json"),
    "{ not json\n",
  )
  .expect("break the config");
  let out = declared_from(dir.path());
  assert_eq!(out.status.code(), Some(4), "stdout: {}", stdout(&out));
  assert_eq!(stdout(&out), "");
  let err = stderr(&out);
  assert!(err.contains("is not valid Intent config"), "{err}");
  assert!(
    err.contains("remedy: the parse error above names the position"),
    "{err}"
  );
}

/// **`lang of` IS THE INDEX'S MAP, ASKED BY NAME** (issue 0617): a language on
/// one line, or nothing at 0 for an extension that maps to none -- the answer
/// the advisory hook's `*) exit 0` could not give -- and a usage error with no
/// path. It needs no project and no file.
#[test]
fn of_prints_a_paths_language_or_nothing() {
  for (path, expected) in [
    ("foo.rs", "rust\n"),
    ("lib/foo.ex", "elixir\n"),
    ("foo.txt", ""),
  ] {
    let out = run_outside(&["lang", "of", path]);
    assert!(out.status.success(), "{path}: {}", stderr(&out));
    assert_eq!(stdout(&out), expected, "{path}");
  }
  let out = run_outside(&["lang", "of"]);
  assert_eq!(out.status.code(), Some(1), "no path is a usage error");
}

/// **A WELL-FORMED CONFIG WHOSE DECLARATION SERDE REFUSES IS A REFUSAL, 1**
/// (vc's ruling, 2026-10-23): it was read, and the gate refuses the commit on
/// it, where a config it cannot read at all is 4.
#[test]
fn declared_on_a_config_whose_languages_serde_refuses_answers_one() {
  let dir = declaring(r#""rust""#);
  let out = declared_from(dir.path());
  assert_eq!(out.status.code(), Some(1), "stdout: {}", stdout(&out));
  assert_eq!(stdout(&out), "");
  assert!(
    stderr(&out).contains("is not valid Intent config"),
    "{}",
    stderr(&out)
  );
}
