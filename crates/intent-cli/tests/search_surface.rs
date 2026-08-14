//! AT-06.4 / AC-06.4: `intent search` returns hits across ST prose, issue
//! bodies and WP text from the FTS index, in the shipped voice and exit codes.
//!
//! `search` is an ADDITION, not a port: there is no `bin/intent_search` to
//! deviate from, so the register records it as new surface and this file is
//! the only thing that says what it must do.
//!
//! **The three sources are tested separately and deliberately.** Writing one
//! test that searched a project containing all three would pass while two of
//! them were dark, because a single hit satisfies a single assertion. Reaching
//! WP text in particular was the gap: v3 reifies work packages INTO
//! `thread.json`, so there is no `WP/<NN>/info.md` for the prose walker to
//! find, and a search for a work package's title matched nothing at all until
//! the index learned to carry it.

use std::path::Path;
use std::process::{Command, Output};

fn run(root: &Path, args: &[&str]) -> Output {
  Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(root)
    .output()
    .expect("run the v3 binary")
}

fn ok(root: &Path, args: &[&str]) -> String {
  let out = run(root, args);
  assert_eq!(
    out.status.code(),
    Some(0),
    "`intent {}` failed\nstderr: {}",
    args.join(" "),
    String::from_utf8_lossy(&out.stderr)
  );
  String::from_utf8_lossy(&out.stdout).to_string()
}

fn project() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let config = dir.path().join("intent").join(".config");
  std::fs::create_dir_all(&config).expect("mkdir");
  std::fs::write(
    config.join("config.json"),
    "{\n  \"intent_version\": \"3.0.0\",\n  \"project_name\": \"Search\",\n  \"author\": \"cc\",\n  \"intent_dir\": \"intent\",\n  \"languages\": [\"rust\"]\n}\n",
  )
  .expect("write config");
  dir
}

/// Source 1: authored steel-thread prose (`design.md`, `impl.md`, `tasks.md`).
#[test]
fn a_word_in_authored_thread_prose_is_found() {
  let dir = project();
  let root = dir.path();
  ok(root, &["st", "new", "Add a Rust-based CLI"]);
  std::fs::write(
    root.join("intent/st/ST0001/design.md"),
    "# Notes\n\nThe kestrel combinator returns its first argument.\n",
  )
  .expect("author prose");

  let hits = ok(root, &["search", "kestrel"]);
  assert!(
    hits.contains("design.md"),
    "the hit names the file it came from: {hits:?}"
  );
  assert!(
    hits.contains("ST0001"),
    "and the entity that owns it: {hits:?}"
  );
}

/// Source 2: authored issue bodies (`issues/<nnnn>.md`).
///
/// The canon is hand-written because the issue verbs are not ported yet. That
/// is legitimate for JSON canon -- it is exactly the bytes the tool writes --
/// and `issues/<nnnn>.md` is an AUTHORED file under D02, so writing it by hand
/// is the supported workflow rather than a v2-style manual edit.
#[test]
fn a_word_in_an_issue_body_is_found() {
  let dir = project();
  let root = dir.path();
  ok(root, &["st", "new", "a thread"]);

  let issues = root.join("intent/issues");
  std::fs::create_dir_all(&issues).expect("mkdir issues");
  std::fs::write(
    issues.join("0001.json"),
    "{\n  \"schema\": \"intent/issue@3.0\",\n  \"number\": 1,\n  \"slug\": \"pelican-drift\",\n  \"title\": \"Pelican drift\",\n  \"status\": \"open\",\n  \"created\": \"2026-08-14\"\n}\n",
  )
  .expect("write issue canon");
  std::fs::write(
    issues.join("0001.md"),
    "# Pelican drift\n\nThe pelican index drifts after a rebuild.\n",
  )
  .expect("write issue body");

  let hits = ok(root, &["search", "pelican"]);
  assert!(
    hits.contains("0001.md"),
    "issue bodies are searchable: {hits:?}"
  );
}

/// Source 3: work-package text.
///
/// This is the one that was dark. WP titles live in `thread.json` after the
/// reification, so nothing under `THREAD_PROSE` carries them and the prose
/// walker never saw them.
#[test]
fn a_word_in_a_work_package_title_is_found() {
  let dir = project();
  let root = dir.path();
  ok(root, &["st", "new", "a thread"]);
  ok(root, &["wp", "new", "ST0001", "Ingest the marmoset corpus"]);

  let hits = ok(root, &["search", "marmoset"]);
  assert!(
    hits.contains("ST0001/01"),
    "a work package's text is searchable and the hit names the WP: {hits:?}"
  );
}

/// A miss is a successful search, not a failure.
///
/// Every grep-shaped use in a script would otherwise have to special-case the
/// commonest answer, and v2's read verbs answer an empty set with exit 0.
#[test]
fn no_match_is_exit_zero_and_silent() {
  let dir = project();
  let root = dir.path();
  ok(root, &["st", "new", "a thread"]);

  let out = run(root, &["search", "nothingwhatsoevermatchesthis"]);
  assert_eq!(out.status.code(), Some(0), "a miss is not an error");
  assert!(
    String::from_utf8_lossy(&out.stdout).trim().is_empty(),
    "and it says nothing"
  );
}

/// A malformed FTS expression is refused in v2's voice, with the underlying
/// complaint preserved in the cause chain (AC-04.4).
#[test]
fn a_malformed_query_is_refused_with_its_cause_and_a_remedy() {
  let dir = project();
  let root = dir.path();
  ok(root, &["st", "new", "a thread"]);

  let out = run(root, &["search", "foo:"]);
  assert_eq!(out.status.code(), Some(1));
  let stderr = String::from_utf8_lossy(&out.stderr);
  assert!(stderr.starts_with("error: "), "v2's voice: {stderr}");
  assert!(
    stderr.contains("caused by: "),
    "the real complaint survives rather than being replaced by a guess: {stderr}"
  );
  assert!(stderr.contains("remedy: "), "{stderr}");
  assert!(
    String::from_utf8_lossy(&out.stdout).is_empty(),
    "a failure writes nothing to stdout"
  );
}

/// Outside a project, search refuses like every other project-scoped verb
/// (INV-03) rather than searching nothing and reporting success.
#[test]
fn outside_a_project_search_refuses() {
  let dir = tempfile::tempdir().expect("tempdir");
  let out = run(dir.path(), &["search", "anything"]);
  assert_eq!(out.status.code(), Some(1));
  assert!(
    String::from_utf8_lossy(&out.stderr).contains("no Intent project"),
    "an empty result here would be indistinguishable from a genuine miss"
  );
}

/// The query is required, and its absence is a usage error in v2's voice.
#[test]
fn a_missing_query_is_a_usage_error_exiting_one() {
  let dir = project();
  let out = run(dir.path(), &["search"]);
  assert_eq!(
    out.status.code(),
    Some(1),
    "INV-02: clap's own exit 2 does not reach the operator"
  );
}
