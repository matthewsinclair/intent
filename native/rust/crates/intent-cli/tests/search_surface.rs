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

/// Bring disk-authored content INTO the store -- the disk -> db direction.
///
/// **These tests need it for a reason that outlives the transition, and it is
/// worth stating.** Authored prose (`design.md`, an issue body) is
/// DISK-NATIVE: D02 keeps it authored rather than generated, so it exists
/// nowhere else and disk -> db is the only way it reaches the index. That
/// makes this direction routine for prose at the same time as it is a
/// destructive restore for modelled entities -- which is a wrinkle in
/// AC-03.9's clean split, reported to vc rather than papered over here.
///
/// **It used to drop the cache**, because the CLI could not spell the
/// direction and a cold store re-ingests on the next open. That was a D36
/// violation left in deliberately, so that a later D36 sweep could not come
/// back clean while the gap it worked around persisted. This is the named
/// cleanup vc attached to AC-03.9, taken now that the flag exists -- and it is
/// a better test for it, since the command is what a user runs.
fn restore_from_disk(root: &Path) {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(["sync", "--to-store"])
    .current_dir(root)
    .output()
    .expect("run the v3 binary");
  assert_eq!(
    out.status.code(),
    Some(0),
    "`intent sync --to-store` failed: {}",
    String::from_utf8_lossy(&out.stderr)
  );
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
  // Written outside the tool, so the store has not seen it. Daily-driver
  // commands answer from the store and never scan the tree (hv, 2026-08-14),
  // and `intent sync` is the explicit reconciliation. This is the trade
  // being exercised, not worked around.
  restore_from_disk(root);

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

  let issues = root.join("intent/.canon/issues");
  std::fs::create_dir_all(&issues).expect("mkdir issues");
  std::fs::write(
    issues.join("0001.json"),
    "{\n  \"schema\": \"intent/issue@3.0\",\n  \"number\": 1,\n  \"slug\": \"pelican-drift\",\n  \"title\": \"Pelican drift\",\n  \"status\": \"open\",\n  \"created\": \"2026-08-14\",\n  \"body\": \"# 0001: Pelican drift\\n\\nThe pelican index drifts after a rebuild.\\n\"\n}\n",
  )
  .expect("write issue canon");
  restore_from_disk(root);

  let hits = ok(root, &["search", "pelican"]);
  // The prose is IN the canon now, so the address a hit carries is the canon
  // file. It used to be a sibling `0001.md` that only this test ever created.
  assert!(
    hits.contains("0001.json"),
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

/// AC-06.7's discriminator: a phrase that appears ONLY in a work package's
/// BODY is found, and the hit names the work package.
///
/// **A title is not enough and that is the whole point of this test.** The
/// sibling above searches a WP title, and a title hit could equally have come
/// from the parent thread's own index entry -- so it cannot tell a WP hit from
/// a thread hit. A phrase living only in `wps[0].body` can come from nowhere
/// else.
///
/// This arm was verified once by hand at `1ca760b` and then guarded by
/// nothing, which vc found by reading the criterion rather than the file. A
/// property demonstrated once and guarded by nothing is a property that decays
/// silently.
#[test]
fn a_phrase_only_in_a_work_package_body_is_found_and_names_the_work_package() {
  let dir = project();
  let root = dir.path();
  ok(root, &["st", "new", "a thread"]);
  ok(root, &["wp", "new", "ST0001", "Ingest and views"]);

  // The body is authored prose in a modelled field (D22/D28), so it is set in
  // canon and brought in through the ingest gate -- not written to a view.
  let canon_path = root.join("intent/.canon/st/ST0001.json");
  let text = std::fs::read_to_string(&canon_path).expect("canon");
  let edited = text.replace(
    "\"body\": \"\"",
    "\"body\": \"## Notes\\n\\nThe capybara clause, which appears in no title.\"",
  );
  assert_ne!(text, edited, "the fixture must actually set a WP body");
  std::fs::write(&canon_path, &edited).expect("write canon");
  restore_from_disk(root);

  let hits = ok(root, &["search", "capybara"]);
  assert!(
    hits.contains("ST0001/01"),
    "the hit names the WORK PACKAGE, which a title hit could not have proved: {hits:?}"
  );
}

/// A miss is a successful search, not a failure.
///
/// Every grep-shaped use in a script would otherwise have to special-case the
/// commonest answer, and v2's read verbs answer an empty set with exit 0.
///
/// **The fixture has to be INDEXED for this to be a miss at all.** It used to
/// be a bare `st new`, which leaves the prose index empty -- so the test
/// believed it was proving "searched and found nothing" while actually
/// exercising "never searched anything", the two cases AC-06.4 exists to keep
/// apart. It passed either way, which is what made it worth fixing rather than
/// deleting.
#[test]
fn no_match_is_exit_zero_and_silent() {
  let dir = project();
  let root = dir.path();
  ok(root, &["st", "new", "a thread"]);
  std::fs::write(
    root.join("intent/st/ST0001/design.md"),
    "# Design\n\nSomething.\n",
  )
  .expect("author prose");
  restore_from_disk(root);

  let out = run(root, &["search", "nothingwhatsoevermatchesthis"]);
  assert_eq!(out.status.code(), Some(0), "a miss is not an error");
  assert!(
    String::from_utf8_lossy(&out.stdout).trim().is_empty(),
    "and it says nothing"
  );
  assert!(
    String::from_utf8_lossy(&out.stderr).trim().is_empty(),
    "a genuine miss over a populated index diagnoses NOTHING -- the note belongs only to the empty-index case"
  );
}

/// AC-06.4's load-bearing property: an unpopulated index is distinguishable
/// from a genuine miss.
///
/// **The row was never really about the hits.** A search over an index that
/// has nothing in it returns exit 0 and zero bytes -- byte-identical to a
/// search that genuinely matched nothing -- so the tool tells a user their
/// phrase is absent when the truth is that the question was never asked. That
/// is the AC-10.7 silent-empty class in a fourth command.
///
/// stdout stays empty in BOTH cases deliberately: a grep-shaped caller keeps
/// its contract and a miss stays exit 0. The distinction is drawn on stderr,
/// so this asserts the two invocations differ there and agree on stdout.
#[test]
fn an_unpopulated_index_is_not_the_same_answer_as_a_genuine_miss() {
  let dir = project();
  let root = dir.path();
  ok(root, &["st", "new", "a thread"]);

  // Nothing has been indexed yet: the same query a populated project would
  // answer with silence.
  let unindexed = run(root, &["search", "nothingwhatsoevermatchesthis"]);
  assert_eq!(unindexed.status.code(), Some(0));
  let said = String::from_utf8_lossy(&unindexed.stderr).to_string();
  assert!(
    said.contains("nothing is indexed"),
    "an empty index says so rather than answering like a miss: {said:?}"
  );
  assert!(
    said.contains("read into the store"),
    "and names the CONDITION that would populate it: {said:?}"
  );
  // **The blast-radius rule, made testable** (vc, 2026-08-15): a remedy must
  // not propose an operation whose blast radius exceeds the fault it repairs.
  // The fault is an unpopulated prose index; `--to-store` replaces the entire
  // store, and `event_log` is durable truth that no file can reconstruct -- so
  // an operator following that advice to fix a search result could lose
  // history that exists nowhere else. This once named that command.
  assert!(
    !said.contains("--to-store"),
    "and does NOT send the operator to an operation broader than the fault: {said:?}"
  );
  assert!(
    String::from_utf8_lossy(&unindexed.stdout).trim().is_empty(),
    "while stdout stays empty, so a pipe is not corrupted by a diagnosis"
  );

  // Now index the project and ask the SAME question. This is the comparison
  // the criterion is about: same query, same exit code, same stdout, and the
  // two invocations must not be the same bytes.
  std::fs::write(
    root.join("intent/st/ST0001/design.md"),
    "# Design\n\nSomething.\n",
  )
  .expect("author prose");
  restore_from_disk(root);
  let missed = run(root, &["search", "nothingwhatsoevermatchesthis"]);
  assert_eq!(missed.status.code(), Some(0), "both are still exit 0");
  assert_eq!(
    String::from_utf8_lossy(&missed.stdout),
    String::from_utf8_lossy(&unindexed.stdout),
    "and both are still silent on stdout"
  );
  assert_ne!(
    String::from_utf8_lossy(&missed.stderr),
    String::from_utf8_lossy(&unindexed.stderr),
    "a no-match must not be the same bytes as a no-index -- this is the whole criterion"
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

/// Source 4: the thread's OWN reified prose (`objective`, `context`).
///
/// vc's finding, and the same defect as the work-package one two tests up,
/// one level higher. D22 moved a thread's objective and context INTO
/// `thread.json`; the indexer was only ever taught about D28's work-package
/// fields. The phrase was rendered into `info.md` -- where every human looks
/// -- and indexed nowhere, because `info.md` is a generated view that
/// `THREAD_PROSE` deliberately excludes.
///
/// It is worth its own test rather than folding into the WP one for the
/// reason this file already gives: a single search over a project containing
/// all four sources passes while three of them are dark.
#[test]
fn a_phrase_in_a_thread_objective_is_found() {
  let dir = project();
  let root = dir.path();
  ok(root, &["st", "new", "a thread"]);

  // Authored canon: the objective is a modelled field, so writing it here is
  // writing the same bytes the tool writes, not a v2-style hand edit.
  let canon = root.join("intent/.canon/st/ST0001.json");
  let text = std::fs::read_to_string(&canon).expect("read canon");
  let edited = text.replace(
    "\"objective\": \"\"",
    "\"objective\": \"Establish the quokka invariant before ingest.\"",
  );
  assert_ne!(text, edited, "the fixture must actually set an objective");
  std::fs::write(&canon, edited).expect("write canon");
  restore_from_disk(root);

  let hits = ok(root, &["search", "quokka"]);
  assert!(
    hits.contains("ST0001"),
    "a thread's own objective is searchable: {hits:?}"
  );
}
