//! **AC-04.6 / issue 0121: `st list` narrows by default and now SAYS SO.**
//!
//! hv, 2026-08-28, filing it against their own tool: `intent st list` rendered
//! four rows against thirteen thread directories on disk and a 67-thread store,
//! disclosed the filter only in `--help`, and was read as corruption. Nothing
//! was inconsistent -- `67 - 52 completed - 2 cancelled = 13` reconciles the
//! three surfaces exactly. **The output carried no way to know that**, which is
//! No Silent Errors applied to a read surface: a default that narrows the
//! answer must name the narrowing, or every comparison against another surface
//! reads as missing data.
//!
//! # The pair that discriminates, driven against ONE fixture
//!
//! An empty result has two causes and they are different answers:
//!
//! - **the estate is empty** -- v2 prints its header, and so does v3. A
//!   disclosure here would report a filter as the reason for a table that would
//!   be empty under any filter at all.
//! - **the filter emptied it** -- the disclosure IS the output. A header with no
//!   rows under it is precisely the shape that was misread.
//!
//! `an_empty_filter_and_an_empty_estate_are_different_answers` asserts both
//! against the same project, differing only in whether a thread exists. **A fix
//! that discloses unconditionally fails the first arm; the pre-fix behaviour
//! fails the second.** Nothing passes both by accident.
//!
//! # The guard that is not about the disclosure at all
//!
//! `the_json_refusal_does_not_depend_on_having_rows` exists because the natural
//! way to write this feature -- return early when there are no rows -- puts a
//! new instance of a defect this estate has already measured: `--format` is
//! validated as an argument to the RENDERER, so a verb that returns before
//! rendering accepts a format it refuses when it has rows. Four slots were
//! found doing that on 2026-08-27. This arm holds the ordering.

use std::path::Path;
use std::process::Command;

use testkit::workspace_root;

fn bin() -> std::path::PathBuf {
  workspace_root().join("target/debug/intent")
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

fn project() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let (text, code) = run(dir.path(), &["init", "narrowed"]);
  assert_eq!(code, 0, "init: {text}");
  dir
}

/// **THE PAIR. Either arm alone passes on a wrong fix.**
#[test]
fn an_empty_filter_and_an_empty_estate_are_different_answers() {
  let dir = project();
  let root = dir.path();

  // ARM 1 -- nothing exists. The header is the answer, and it is v2's: a
  // command that says nothing at all cannot be told from one that did not run.
  let (empty_estate, code) = run(root, &["st", "list"]);
  assert_eq!(code, 0, "{empty_estate}");
  assert!(
    empty_estate.starts_with("ID "),
    "an empty ESTATE still prints its header: {empty_estate:?}"
  );
  assert!(
    !empty_estate.contains("st list:"),
    "and it does not blame a filter for a table that would be empty under any \
     filter at all: {empty_estate:?}"
  );

  // ARM 2 -- the SAME command over a store with one thread in it. `st new`
  // enters at `Triage` and the bare form shows WIP, so the filter is what
  // emptied the result.
  let (made, code) = run(root, &["st", "new", "a thread"]);
  assert_eq!(code, 0, "{made}");

  let (narrowed, code) = run(root, &["st", "list"]);
  assert_eq!(code, 0, "an empty answer is not a failure: {narrowed}");
  assert!(
    !narrowed.starts_with("ID "),
    "the disclosure IS the output -- a header with no rows under it is the \
     shape that was misread as missing data: {narrowed:?}"
  );
  assert!(
    narrowed.contains("of 1 in this store"),
    "it names the denominator, so the reader can see nothing is lost: {narrowed:?}"
  );
  assert!(
    narrowed.contains("--status all"),
    "and how to widen: {narrowed:?}"
  );
}

/// The scope is the FILTER, not a fixed word -- a disclosure that always said
/// *in progress* would be wrong for every explicit `--status` and would pass
/// this file's other arms.
#[test]
fn the_disclosure_names_the_filter_that_was_actually_applied() {
  let dir = project();
  let root = dir.path();
  run(root, &["st", "new", "one"]);
  run(root, &["st", "new", "two"]);
  run(root, &["st", "start", "ST0001"]);

  let (wip, code) = run(root, &["st", "list", "--width", "120"]);
  assert_eq!(code, 0, "{wip}");
  assert!(
    wip.contains("showing 1 of 2 threads, status `WIP`"),
    "the default filter names itself: {wip:?}"
  );

  let (triaged, code) = run(root, &["st", "list", "--status", "hold", "--width", "120"]);
  assert_eq!(code, 0, "{triaged}");
  assert!(
    triaged.contains("no thread matches status `On Hold`"),
    "an explicit filter names ITSELF, not the default: {triaged:?}"
  );
}

/// **NOTHING NARROWED, NOTHING DISCLOSED -- and the reason is a contract, not a
/// preference.** `st sync`'s dry run renders through the same function and is
/// byte-identical to `st list --status all`; a note naming `st list` would be
/// describing the wrong command on one of its two callers.
#[test]
fn the_unnarrowed_render_says_nothing_and_stays_byte_identical_to_st_sync() {
  let dir = project();
  let root = dir.path();
  run(root, &["st", "new", "one"]);
  run(root, &["st", "new", "two"]);

  let (all, code) = run(root, &["st", "list", "--status", "all"]);
  assert_eq!(code, 0, "{all}");
  assert!(
    !all.contains("st list:"),
    "`--status all` narrows nothing, so there is nothing to disclose: {all:?}"
  );

  let (synced, code) = run(root, &["st", "sync"]);
  assert_eq!(code, 0, "{synced}");
  assert_eq!(
    all, synced,
    "same scope, same renderer, same bytes -- the disclosure must not have \
     broken the index parity"
  );
}

/// **THE ORDERING GUARD.** A `--format` refusal is a property of the verb, not
/// of how much it happened to find. Driven on the narrowed-EMPTY case, which is
/// the one an early return would have let through.
#[test]
fn the_json_refusal_does_not_depend_on_having_rows() {
  let dir = project();
  let root = dir.path();
  run(root, &["st", "new", "a thread"]);

  // The control: with rows, the refusal is the known behaviour.
  let (with_rows, code) = run(root, &["st", "list", "--status", "all", "--format", "json"]);
  assert_eq!(code, 1, "the known refusal: {with_rows}");
  assert!(with_rows.contains("no json projection"), "{with_rows}");

  // The subject: the same flag over a filter that matches nothing.
  let (no_rows, code) = run(root, &["st", "list", "--format", "json"]);
  assert_eq!(
    code, 1,
    "an empty result set must not buy a format the verb refuses: {no_rows}"
  );
  assert!(no_rows.contains("no json projection"), "{no_rows}");
}

/// The persisted form owes the disclosure MORE than the terminal does: a
/// markdown table redirected to a file outlives the command line that scoped
/// it, so a reader meeting it later has nothing else to go on.
#[test]
fn markdown_carries_the_disclosure_too() {
  let dir = project();
  let root = dir.path();
  run(root, &["st", "new", "one"]);
  run(root, &["st", "new", "two"]);
  run(root, &["st", "start", "ST0001"]);

  let (md, code) = run(root, &["st", "list", "--markdown"]);
  assert_eq!(code, 0, "{md}");
  assert!(md.starts_with("| ID "), "canonical GFM: {md:?}");
  assert!(
    md.contains("showing 1 of 2 threads"),
    "and the scope travels with the file: {md:?}"
  );
}
