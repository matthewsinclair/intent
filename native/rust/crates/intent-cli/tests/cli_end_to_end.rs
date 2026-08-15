//! AT-05.2 / AC-05.2: the wired families work through the real binary, with
//! v2's voice and v2's exit codes.
//!
//! **This file exists because 150 green tests missed three real defects.**
//! Every CLI test I had driven an ERROR path -- a missing argument, an unknown
//! flag -- so the binary had never once been asked to do something and
//! succeed. Running it by hand found, in one go:
//!
//!   1. `intent/.cache/` is gitignored (D21) and therefore absent in every
//!      fresh project, so SQLite could not create the DB and the FIRST command
//!      in any new project failed;
//!   2. the renderer asked for positional names the dispatch table does not
//!      declare (`stid` where the table says `id`), which clap answers by
//!      PANICKING -- exit 101, neither a v2 code nor an Intent error;
//!   3. `get_one` panics on an undeclared id, so a table/renderer disagreement
//!      crashed instead of reporting.
//!
//! None of the three is visible from an error path, and all three are fatal to
//! the first thing a user does. The rule that catches this class is not more
//! unit tests: it is exercising the real binary against a real project.

use std::path::Path;
use std::process::{Command, Output};

fn project() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let config = dir.path().join("intent").join(".config");
  std::fs::create_dir_all(&config).expect("mkdir");
  std::fs::write(
    config.join("config.json"),
    "{\n  \"intent_version\": \"3.0.0\",\n  \"project_name\": \"E2E\",\n  \"author\": \"cc\",\n  \"intent_dir\": \"intent\",\n  \"languages\": [\"rust\"]\n}\n",
  )
  .expect("write config");
  dir
}

fn run(root: &Path, args: &[&str]) -> Output {
  Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(root)
    .output()
    .expect("run the v3 binary")
}

fn stdout(out: &Output) -> String {
  String::from_utf8_lossy(&out.stdout).to_string()
}

fn ok(root: &Path, args: &[&str]) -> String {
  let out = run(root, args);
  assert_eq!(
    out.status.code(),
    Some(0),
    "`intent {}` failed\nstdout: {}\nstderr: {}",
    args.join(" "),
    stdout(&out),
    String::from_utf8_lossy(&out.stderr)
  );
  stdout(&out)
}

/// The very first command in a brand-new project. Defect (1) made this fail.
///
/// **This is AC-07.1's 0022 broken-install class**, arriving two work packages
/// before the AC that forbids it (vc, 2026-08-14). AC-07.1 requires that a
/// fresh `intent init` works from the binary alone and that the broken-install
/// class is unconstructible; this test is what will make that provable, so the
/// connection is recorded here rather than rediscovered in WP-07.
#[test]
fn the_first_command_in_a_fresh_project_succeeds() {
  let dir = project();
  assert!(
    !dir.path().join("intent/.cache").exists(),
    "precondition: the gitignored cache directory does not exist yet"
  );
  let out = ok(dir.path(), &["st", "new", "Add a Rust-based CLI"]);
  assert_eq!(out.trim(), "created: ST0001");
}

/// The full lifecycle, through the binary, writing real canon and real views.
#[test]
fn a_thread_moves_through_its_lifecycle_and_writes_canon_and_views() {
  let dir = project();
  let root = dir.path();

  ok(root, &["st", "new", "Add a Rust-based CLI"]);
  assert!(root.join("intent/st/ST0001/thread.json").is_file(), "canon");
  assert!(
    root.join("intent/st/ST0001/info.md").is_file(),
    "cover view"
  );
  assert!(
    root.join("intent/st/ST0001/acceptance.md").is_file(),
    "contract view"
  );
  assert!(root.join("intent/st/steel_threads.md").is_file(), "index");
  assert!(root.join("intent/todo.md").is_file(), "todo view");

  // `--status all` because bare `st list` shows WIP ONLY, as v2 does, and this
  // thread is Not Started. The bare form used to list everything, which is what
  // made this assertion pass before the filter was ported.
  let listed = ok(root, &["st", "list", "--status", "all"]);
  assert!(listed.contains("ST0001"), "{listed}");
  assert!(listed.contains("Not Started"), "{listed}");
  assert!(
    ok(root, &["st", "list"]).lines().count() == 2,
    "and the bare form is header + separator only, not an error and not silence"
  );

  ok(root, &["st", "start", "ST0001"]);
  let shown = ok(root, &["st", "show", "ST0001"]);
  assert!(shown.contains("status: WIP"), "{shown}");

  ok(root, &["wp", "new", "ST0001", "Ingest and views"]);
  ok(root, &["wp", "start", "ST0001/01"]);
  let wps = ok(root, &["wp", "list", "ST0001"]);
  assert!(
    wps.lines().any(|l| l.starts_with("01 ")),
    "the WP column is v2's bare sequence number: {wps}"
  );
}

/// The gate reaches the CLI with v2's contract: its verdict on stdout, exit 1.
#[test]
fn the_gate_speaks_v2s_contract_through_the_cli() {
  let dir = project();
  let root = dir.path();
  ok(root, &["st", "new", "a thread"]);

  let out = run(root, &["ac", "gate", "ST0001"]);
  assert_eq!(
    out.status.code(),
    Some(1),
    "an empty contract BLOCKS, and `st done` reads that code"
  );
  assert!(
    stdout(&out).starts_with("gate: ST0001 BLOCKED"),
    "the verdict goes to STDOUT, because machines parse it: {}",
    stdout(&out)
  );
}

/// `st done` is gated, and its refusal carries the gate's own verdict.
#[test]
fn closing_through_the_cli_is_gated_and_says_why() {
  let dir = project();
  let root = dir.path();
  ok(root, &["st", "new", "a thread"]);

  let out = run(root, &["st", "done", "ST0001"]);
  assert_eq!(out.status.code(), Some(1));
  let stderr = String::from_utf8_lossy(&out.stderr);
  assert!(stderr.starts_with("error: "), "the voice: {stderr}");
  assert!(stderr.contains("gate: ST0001 BLOCKED"), "{stderr}");
  assert!(stderr.contains("remedy: "), "{stderr}");
  assert!(
    stdout(&out).is_empty(),
    "a failure writes nothing to stdout (INV-06 corrected)"
  );
}

/// INV-03: outside a project, the tool says so rather than half-working.
#[test]
fn outside_a_project_the_tool_refuses_with_a_remedy() {
  let dir = tempfile::tempdir().expect("tempdir");
  let out = run(dir.path(), &["st", "list"]);
  assert_eq!(out.status.code(), Some(1));
  let stderr = String::from_utf8_lossy(&out.stderr);
  assert!(stderr.contains("no Intent project"), "{stderr}");
  assert!(
    stderr.contains("intent init"),
    "the remedy names the fix: {stderr}"
  );
}

/// A renderer asking for a positional the table does not declare must REPORT,
/// not panic. Defect (3): `get_one` panics on an undeclared id, so the binary
/// exited 101 with a clap internal message.
#[test]
fn every_wired_verb_takes_its_arguments_without_panicking() {
  let dir = project();
  let root = dir.path();
  ok(root, &["st", "new", "a thread"]);
  ok(root, &["wp", "new", "ST0001", "a package"]);

  // Drive every wired verb that takes a positional. A panic exits 101; a
  // legitimate refusal exits 1. Neither may be 101.
  for args in [
    vec!["st", "show", "ST0001"],
    vec!["st", "start", "ST0001"],
    vec!["st", "cancel", "ST0001"],
    vec!["wp", "list", "ST0001"],
    vec!["wp", "start", "ST0001/01"],
    vec!["at", "list", "ST0001"],
    vec!["ac", "gate", "ST0001"],
  ] {
    let out = run(root, &args);
    let code = out.status.code().expect("exited");
    assert!(
      code == 0 || code == 1,
      "`intent {}` exited {code} -- 101 is a panic, which is neither a v2 exit code nor an Intent error\nstderr: {}",
      args.join(" "),
      String::from_utf8_lossy(&out.stderr)
    );
  }
}

/// An unwired verb says SO -- it does not claim no command was given.
///
/// Found while classifying the conformance baseline: `intent st repair` used to
/// answer "a steel thread command is required" when a command had plainly been
/// given. That is the same-text-for-different-causes collapse AC-04.4 forbids,
/// one layer out, and it actively misled the classification: 35 conformance
/// rows looked like "no command" when they were "not built yet".
#[test]
fn an_unwired_verb_is_distinguishable_from_a_missing_one() {
  let dir = project();
  let root = dir.path();

  let unwired = run(root, &["st", "repair"]);
  let missing = run(root, &["st"]);

  let unwired_err = String::from_utf8_lossy(&unwired.stderr).to_string();
  let missing_err = String::from_utf8_lossy(&missing.stderr).to_string();

  assert!(
    unwired_err.contains("not yet wired"),
    "an unwired verb names itself: {unwired_err}"
  );
  assert!(
    unwired_err.contains("st repair"),
    "and names WHICH verb: {unwired_err}"
  );
  assert_ne!(
    unwired_err.trim(),
    missing_err.trim(),
    "'you typed nothing' and 'we have not built that' are different problems and only one of them is the operator's"
  );
}

/// The generated views are real markdown a human can read, and carry the
/// no-clock banner rather than a render timestamp (D23).
#[test]
fn the_generated_index_is_written_and_carries_no_render_time() {
  let dir = project();
  let root = dir.path();
  ok(root, &["st", "new", "Add a Rust-based CLI"]);

  let index = std::fs::read_to_string(root.join("intent/st/steel_threads.md")).expect("read index");
  assert!(index.contains("| ST0001 |"), "{index}");
  assert!(index.contains("Generated by Intent v"), "{index}");
  assert!(
    !index.contains("<!-- BEGIN"),
    "no region markers survive the port: {index}"
  );
}

/// `intent sync` and `intent st sync` are DIFFERENT commands, and both work.
///
/// I had them wired as one: `st sync` delegated to the store reconciliation,
/// and the dispatch table carries my note saying "both spellings run it". That
/// note was wrong. `tests/unit/output_width.bats` proved it -- v2's `st sync`
/// prints the thread table and `--write` persists the index, neither of which
/// is "reconcile the store from canon".
///
/// The lesson worth keeping is not the fix. It is that I wrote a test called
/// `both_spellings_of_sync_are_wired_and_agree`, asserted they produced the
/// same bytes, watched it pass, and took that as confirmation -- when all it
/// confirmed was that my own wrong model was internally consistent. A test
/// written from the same misreading as the code cannot catch the misreading.
/// The incumbent's behaviour caught it.
#[test]
fn sync_and_st_sync_are_different_commands_and_both_are_wired() {
  let dir = project();
  let root = dir.path();
  ok(root, &["st", "new", "A thread"]);

  // **The bare verb REFUSES (AC-03.9).** It used to run disk -> db, which
  // under D01 as reversed is a RESTORE that overwrites the source of truth
  // with the re-creatable side. A verb whose two directions differ in
  // destructiveness must not have a silent default, and the default it had was
  // the destructive one.
  let refusal = run(root, &["sync"]);
  assert_eq!(
    refusal.status.code(),
    Some(1),
    "the bare verb refuses rather than picking a direction"
  );
  let said = String::from_utf8_lossy(&refusal.stderr).to_string();
  assert!(
    said.contains("two directions"),
    "the refusal says WHY it refused: {said:?}"
  );
  assert!(
    said.contains("DESTRUCTIVE"),
    "and names which direction is the dangerous one, in a word that survives skimming: {said:?}"
  );
  assert!(
    stdout(&refusal).is_empty(),
    "a refusal writes nothing to stdout, so a pipe sees no result: {:?}",
    stdout(&refusal)
  );

  let index = ok(root, &["st", "sync"]);
  assert!(
    index.starts_with("ID "),
    "st sync still reports the index as a table: {index:?}"
  );
  assert!(
    !index.contains("two directions"),
    "st sync is a different command and is NOT the refusing one -- collapsing them is what this test exists to prevent: {index:?}"
  );
}

/// `intent st list` renders v2's table, and renders it even when empty.
///
/// The empty case is the point. v2 prints a 161-byte header for an estate with
/// no threads; v3 printed ZERO BYTES, which is the same shape as the AC-10.7
/// defect one level down -- a command that answers a question by saying
/// nothing at all, so a script cannot tell "ran and found none" from "did not
/// run". The answer here was honest and the silence still was not.
#[test]
fn st_list_prints_the_table_header_even_with_no_threads() {
  let dir = project();
  let out = ok(dir.path(), &["st", "list"]);
  assert!(out.starts_with("ID "), "v2's column order: {out:?}");
  assert!(out.contains("| Slug"), "{out:?}");
  assert!(out.contains("| Completed"), "{out:?}");
  assert!(
    out.lines().nth(1).is_some_and(|l| l.contains("---|---")),
    "and v2's pipeless separator: {out:?}"
  );
}

/// The table fills the terminal, and content-fit is the FLOOR rather than a
/// target -- a narrow terminal stops padding, it never truncates.
///
/// Measured byte-identical against the v2 binary over the same estate at
/// COLUMNS 250/130/100/60 before this was written; this pins the relationships
/// that survive without v2 present to compare against.
#[test]
fn the_table_tracks_the_terminal_width_and_never_truncates() {
  let dir = project();
  let root = dir.path();
  ok(
    root,
    &[
      "st",
      "new",
      "a deliberately long steel thread title for measuring",
    ],
  );

  let width_at = |cols: &str| -> usize {
    let out = Command::new(env!("CARGO_BIN_EXE_intent"))
      .args(["st", "list", "--status", "all"])
      .current_dir(root)
      .env("COLUMNS", cols)
      .output()
      .expect("run");
    let text = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(
      text.contains("a-deliberately-long"),
      "the slug is never truncated to fit: {text:?}"
    );
    text.lines().map(|l| l.chars().count()).max().unwrap_or(0)
  };

  let wide = width_at("250");
  let narrow = width_at("60");
  assert!(wide >= 200, "fills a wide terminal: {wide}");
  assert!(
    wide > narrow,
    "width tracks the terminal: 250 -> {wide}, 60 -> {narrow}"
  );

  // Content-fit is the floor: at 60 columns the table is WIDER than 60,
  // because the slug alone does not fit and nothing is ever cut.
  assert!(
    narrow > 60,
    "a narrow terminal stops padding rather than truncating: {narrow}"
  );
}

/// `--width` beats the terminal, and `--markdown` ignores both.
#[test]
fn width_is_overridable_and_markdown_is_width_independent() {
  let dir = project();
  let root = dir.path();
  ok(
    root,
    &[
      "st",
      "new",
      "yet another long steel thread title to exercise the override",
    ],
  );

  let run_at = |cols: &str, args: &[&str]| -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_intent"))
      .args(args)
      .current_dir(root)
      .env("COLUMNS", cols)
      .output()
      .expect("run");
    String::from_utf8_lossy(&out.stdout).to_string()
  };

  let overridden = run_at("250", &["st", "list", "--status", "all", "--width", "120"]);
  let longest = overridden
    .lines()
    .map(|l| l.chars().count())
    .max()
    .unwrap_or(0);
  assert!(
    (110..=130).contains(&longest),
    "--width 120 beats COLUMNS=250: got {longest}"
  );

  // A persisted file must not depend on the window that generated it.
  let a = run_at("200", &["st", "list", "--status", "all", "--markdown"]);
  let b = run_at("60", &["st", "list", "--status", "all", "--markdown"]);
  assert_eq!(a, b, "markdown is content-fit at every terminal width");
  assert!(a.starts_with("| ID "), "canonical piped GFM: {a:?}");
}

/// `st sync` is v2's INDEX sync, not the store reconciliation, and its dry run
/// is byte-identical to `st list --status all`.
///
/// They were wired as the same command. `tests/unit/output_width.bats` is what
/// caught it, and this keeps it caught without the BATS estate in the loop.
#[test]
fn st_sync_dry_run_is_the_index_table_not_a_reconciliation_report() {
  let dir = project();
  let root = dir.path();
  ok(root, &["st", "new", "alpha"]);
  ok(root, &["st", "new", "bravo"]);
  ok(root, &["st", "start", "ST0002"]);

  let listed = ok(root, &["st", "list", "--status", "all"]);
  let synced = ok(root, &["st", "sync"]);
  assert_eq!(
    listed, synced,
    "same scope, same width, same renderer -- so the same bytes"
  );
  assert!(
    !synced.contains("ok: synced"),
    "the dry run reports the index, not the store: {synced:?}"
  );
  assert!(
    ok(root, &["st", "sync", "--write"]).starts_with("updated: "),
    "and --write says what it wrote"
  );
}

/// `wp list` renders through the SAME table as `st list`, and its empty case
/// is v2's sentence rather than a bare header.
///
/// The dispatch row asks for the shared renderer in as many words -- "so `wp
/// list` and `st list` column layout cannot drift apart" -- so this asserts
/// the sharing, not the appearance: same separator, same fill behaviour.
/// v3 had been printing `WP-01  Wip  title`, which is wrong on the prefix, the
/// status vocabulary and the shape at once.
#[test]
fn wp_list_shares_the_table_and_v2s_empty_case() {
  let dir = project();
  let root = dir.path();
  ok(root, &["st", "new", "a thread"]);

  // The empty case is a SENTENCE at exit 0 -- deliberately unlike `st list`,
  // which prints its header. Both are v2's, and they differ.
  let empty = ok(root, &["wp", "list", "ST0001"]);
  assert_eq!(empty.trim(), "no work packages for ST0001");

  ok(root, &["wp", "new", "ST0001", "Ingest and views"]);
  let listed = ok(root, &["wp", "list", "ST0001"]);
  let header = listed.lines().next().unwrap_or_default();
  assert!(
    header.starts_with("WP") && header.contains("| Title") && header.contains("| Scope"),
    "v2's columns, whatever the padding: {header:?}"
  );
  assert!(
    listed.lines().nth(1).is_some_and(|l| l.contains("---|---")),
    "the shared pipeless separator: {listed:?}"
  );
  assert!(
    listed.lines().any(|l| l.starts_with("01 ")),
    "v2 numbers the column `01`, not `WP-01`: {listed:?}"
  );
  assert!(
    listed.contains("Not Started"),
    "v2's status vocabulary, not the enum's Debug spelling: {listed:?}"
  );
}

/// `wp new` writes v2's default scope.
///
/// v2 takes no scope flag, so every work package it creates carries the
/// template's `scope: Small`. v3 hardcoded `M`, which is the same command
/// writing different canon -- a parity break with no visible output to give it
/// away.
#[test]
fn wp_new_defaults_to_v2s_template_scope() {
  let dir = project();
  let root = dir.path();
  ok(root, &["st", "new", "a thread"]);
  ok(root, &["wp", "new", "ST0001", "a package"]);

  let canon = std::fs::read_to_string(root.join("intent/st/ST0001/thread.json")).expect("canon");
  assert!(
    canon.contains("\"scope\": \"S\""),
    "the canon carries S, as v2's template seeds: {canon}"
  );
}

/// AC-06.7's last clause: a work package carrying sections the template never
/// named survives canon -> view -> canon byte-identical.
///
/// The `WP/<NN>/info.md` view was the missing third of this AC (vc): the canon
/// half and the search half were already verified, and there was no view for
/// the round trip to pass through. D28 gave work packages `objective` and
/// `body` as TWO fields rather than a set of named sections precisely because
/// real work packages exceed the template freely -- ST0056's own WP-13 runs to
/// hundreds of lines -- so `body` is emitted verbatim and a renderer that
/// re-derived fixed headings would drop whatever it did not foresee.
#[test]
fn a_work_package_body_survives_canon_to_view_to_canon() {
  let dir = project();
  let root = dir.path();
  ok(root, &["st", "new", "a thread"]);
  ok(root, &["wp", "new", "ST0001", "Ingest and views"]);

  let canon_path = root.join("intent/st/ST0001/thread.json");
  let authored = "## Why the incumbents go\n\nA section the template never named, carried verbatim.\n\n| a | b |\n| --- | --- |\n| 1 | 2 |\n\n## The seams\n\nA `pipe | inside` prose, and _emphasis_ the formatter rewrites.";
  let text = std::fs::read_to_string(&canon_path).expect("canon");
  let edited = text.replace(
    "\"body\": \"\"",
    &format!(
      "\"body\": {}",
      serde_json::to_string(authored).expect("json")
    ),
  );
  assert_ne!(text, edited, "the fixture must actually set a body");
  std::fs::write(&canon_path, &edited).expect("write canon");

  // **D01 REVERSED: a hand-edited canon file is not truth any more.** The
  // projection reads the STORE, so the edit above has to be brought INTO the
  // store before it can be rendered -- which is the disk -> db direction, and
  // the direction the CLI cannot yet spell (AC-03.9, owed by WP-06).
  //
  // Dropping the cache forces it: a cold store ingests from the files on the
  // next open (`ingest::load_fresh`). That is the same movement the restore
  // will make explicit, so this is a stand-in for the command rather than a
  // trick -- and it is exactly what the reversal means, since editing canon by
  // hand now changes a projection and not the source of truth.
  std::fs::remove_file(root.join("intent/.cache/intent.db")).expect("drop the cold store");

  // The projection, via the safe direction -- the bare verb now refuses.
  ok(root, &["st", "sync", "--write"]);

  // The view carries the authored sections VERBATIM.
  let view = std::fs::read_to_string(root.join("intent/st/ST0001/WP/01/info.md")).expect("view");
  for fragment in [
    "## Why the incumbents go",
    "A section the template never named",
    "| --- | --- |",
    "## The seams",
    "`pipe | inside`",
  ] {
    assert!(
      view.contains(fragment),
      "missing {fragment:?} from:\n{view}"
    );
  }

  // And the canon is untouched by having been rendered -- the round trip does
  // not rewrite a file it merely read.
  let after = std::fs::read_to_string(&canon_path).expect("canon after");
  assert_eq!(
    edited, after,
    "canon -> view -> canon must be byte-identical"
  );

  // Idempotent: rendering twice writes the same bytes (AC-03.2).
  // The projection, via the safe direction -- the bare verb now refuses.
  ok(root, &["st", "sync", "--write"]);
  let again = std::fs::read_to_string(root.join("intent/st/ST0001/WP/01/info.md")).expect("view");
  assert_eq!(view, again, "the view renders the same bytes twice");
}
