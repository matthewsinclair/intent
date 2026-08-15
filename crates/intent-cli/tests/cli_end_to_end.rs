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

  let listed = ok(root, &["st", "list"]);
  assert!(listed.contains("ST0001"), "{listed}");
  assert!(listed.contains("Not Started"), "{listed}");

  ok(root, &["st", "start", "ST0001"]);
  let shown = ok(root, &["st", "show", "ST0001"]);
  assert!(shown.contains("status: WIP"), "{shown}");

  ok(root, &["wp", "new", "ST0001", "Ingest and views"]);
  ok(root, &["wp", "start", "ST0001/01"]);
  let wps = ok(root, &["wp", "list", "ST0001"]);
  assert!(wps.contains("WP-01"), "{wps}");
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

/// Both spellings of the reconciliation are wired, and they do the same thing.
///
/// `intent sync` is the name hv gave the manual half of the daily-driver split;
/// `intent st sync` is v2's own command, whose job is a strict subset of it now
/// that the thread index is generated from the model. Both have to work, so the
/// risk is not that one is missing but that they DRIFT -- and this asserts they
/// are one implementation by asserting they produce the same answer.
///
/// It is here because they had already drifted in the worst direction: only
/// `st sync` was wired, so the spelling the dispatch table advertises and hv
/// actually named answered "not yet wired to the facade". vc hit it while
/// trying to verify something else and reasonably concluded sync was unbuilt.
#[test]
fn both_spellings_of_sync_are_wired_and_agree() {
  let dir = project();
  let root = dir.path();
  ok(root, &["st", "new", "A thread"]);

  // A hand edit behind the tool's back: the store still serves the old title
  // until a reconciliation runs, which is exactly what sync is for.
  let canon = root.join("intent/st/ST0001/thread.json");
  let edited = std::fs::read_to_string(&canon)
    .expect("read canon")
    .replace("A thread", "A renamed thread");
  std::fs::write(&canon, edited).expect("write canon");

  let long = ok(root, &["st", "sync"]);
  let short = ok(root, &["sync"]);
  assert_eq!(
    long, short,
    "two spellings, one implementation -- if these ever differ, one of them \
     has grown its own copy"
  );
  assert!(
    ok(root, &["st", "list"]).contains("A renamed thread"),
    "and the reconciliation actually took"
  );
}
