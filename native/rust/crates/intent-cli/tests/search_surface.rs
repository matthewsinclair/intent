//! AT-06.4 / AC-06.4: `intent search` returns hits across ST prose, issue
//! bodies and WP text from the FTS index, in the shipped voice and exit codes.
//!
//! **AT-22.2 is the `--no-reconcile` arm at the foot of this file**, which
//! carries both clauses of its criterion: an answer from the index as it stands,
//! and the paths that have moved underneath it named by path.
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
    .stdin(testkit::lifeline_for(args))
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
  //
  // **`--no-reconcile` IS HOW AN UNPOPULATED INDEX IS REACHED SINCE WP-22**, and
  // it is on BOTH invocations below rather than only this one. A daemonless
  // query now reconciles before it answers, so a bare `intent search` in a fresh
  // project populates the very index this arm is about and the state stops
  // existing. Putting the flag on one side only would leave the two invocations
  // differing by a flag as well as by the index, and the criterion is about the
  // index.
  let unindexed = run(
    root,
    &["search", "nothingwhatsoevermatchesthis", "--no-reconcile"],
  );
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
  let missed = run(
    root,
    &["search", "nothingwhatsoevermatchesthis", "--no-reconcile"],
  );
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
///
/// **THE SPECIMEN MOVED ON 2026-09-05 AND THE ASSERTION DID NOT RELAX.** This
/// arm used `foo:`, which `0247` made a LITERAL rather than a malformed
/// expression -- bare punctuation is now quoted, so a colon searches for
/// itself instead of asking FTS5 for a column. **The property under test is
/// unchanged; only the example of it stopped being an example.** Deleting the
/// arm would have removed coverage that disagreed with a change of mine, which
/// is the one thing a change must never do to its own witnesses.
///
/// **AN UNBALANCED PAREN IS A STRICTLY BETTER SPECIMEN THAN `foo:` EVER WAS**:
/// it is structurally malformed under ANY escaping policy, so this arm can no
/// longer be quietly retired by a future change to what counts as punctuation.
#[test]
fn a_malformed_query_is_refused_with_its_cause_and_a_remedy() {
  let dir = project();
  let root = dir.path();
  ok(root, &["st", "new", "a thread"]);

  let out = run(root, &["search", "(foo"]);
  assert_eq!(out.status.code(), Some(1));
  let stderr = String::from_utf8_lossy(&out.stderr);
  assert!(stderr.starts_with("error: "), "v2's voice: {stderr}");
  // Issue 0443: a malformed expression is still the READER's, not a search
  // that could not be answered -- the over-correction on the far side of that fix.
  assert!(
    stderr.contains("the search query `(foo` was refused"),
    "{stderr}"
  );
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

/// `0247`: **ordinary punctuation is searched literally rather than reaching
/// FTS5 as column syntax.**
///
/// **THE FOUR QUERIES THIS ESTATE ACTUALLY TYPES WERE ALL REFUSED**: a source
/// filename, the artefact its own canon mandates every agent regenerate, its
/// release number, and a phrase from an issue title. Each came back as
/// `sqlite: no such column: root` or `fts5: syntax error near "."` -- a
/// DATABASE SCHEMA error, three layers of causation deep, about a query nobody
/// wrote.
///
/// **THE CONTROL IS THE SECOND HALF AND IT IS WHAT MAKES rc=0 MEAN ANYTHING.**
/// Exit zero alone would pass on a build that answered nothing at all, and a
/// no-match is also exit zero here by design (`AC-04.3`). So the hyphenated
/// query must find the SPECIFIC section that carries it, and a hyphenated term
/// that is NOT in the corpus must still come back empty -- otherwise the arm
/// would pass on a query that matched everything.
#[test]
fn ordinary_punctuation_is_searched_literally_and_finds_what_it_names() {
  let dir = project();
  let root = dir.path();
  ok(root, &["st", "new", "a thread"]);
  std::fs::write(
    root.join("intent/st/ST0001/design.md"),
    "# Notes\n\nThe family-root disclaimer lives in render.rs and shipped in v3.0.1.\n",
  )
  .expect("author prose");
  restore_from_disk(root);

  for query in ["family-root", "render.rs", "v3.0.1"] {
    let out = run(root, &["search", query]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
      out.status.code(),
      Some(0),
      "`intent search {query}` was refused. Before 0247 this reached FTS5 raw and \
       answered with sqlite's schema rather than the operator's words: {stderr}"
    );
    let hits = String::from_utf8_lossy(&out.stdout);
    assert!(
      hits.contains("design.md"),
      "`intent search {query}` exited 0 without finding the section that contains it, \
       so exit zero here is a silence rather than an answer: {hits:?}"
    );
  }

  // **THE ANTI-VACUITY ARM.** Without it every assertion above passes on a
  // build whose escaping matched everything -- which is exactly how a fix to a
  // refusal turns into a fix that returns the whole corpus.
  let absent = run(root, &["search", "kestrel-combinator"]);
  assert_eq!(
    absent.status.code(),
    Some(0),
    "an absent hyphenated term is a no-match, not a refusal: {}",
    String::from_utf8_lossy(&absent.stderr)
  );
  assert!(
    String::from_utf8_lossy(&absent.stdout).is_empty(),
    "a hyphenated term nothing carries must find nothing, or the quoting matched everything"
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

/// Issue 0195: a hit printed `path:0` whatever line the match was on, and one
/// file was once listed once per match.
///
/// `path:N` is the shape a jump-to-line consumer trusts, so N must be a line
/// in THAT file, checked against the file's own bytes and not the index. A hit
/// inside canon JSON has no prose line and prints the file alone. So does an
/// attachment edited on disk since it was carried, because its indexed body is
/// no longer the file. And a phrase that occurs once in one file is one row.
#[test]
fn a_hit_names_the_line_it_is_on_or_no_line_at_all() {
  let dir = project();
  let root = dir.path();
  ok(root, &["st", "new", "a thread"]);
  let design = root.join("intent/st/ST0001/design.md");
  std::fs::create_dir_all(design.parent().expect("a thread dir")).expect("mkdir");
  std::fs::write(
    &design,
    "# Notes\n\nA first paragraph.\n\n## Detail\n\nThe kestrel combinator returns its first argument.\n",
  )
  .expect("author prose");
  let issues = root.join("intent/.canon/issues");
  std::fs::create_dir_all(&issues).expect("mkdir issues");
  std::fs::write(
    issues.join("0001.json"),
    "{\n  \"schema\": \"intent/issue@3.0\",\n  \"number\": 1,\n  \"slug\": \"pelican-drift\",\n  \"title\": \"Pelican drift\",\n  \"status\": \"open\",\n  \"created\": \"2026-08-14\",\n  \"body\": \"The pelican index drifts after a rebuild.\\n\"\n}\n",
  )
  .expect("write issue canon");
  restore_from_disk(root);

  // The FILE's own line for the phrase, read off disk rather than the index.
  let on_disk = std::fs::read_to_string(&design).expect("read the file back");
  let line = on_disk
    .lines()
    .position(|l| l.contains("kestrel"))
    .expect("the phrase is in the file")
    + 1;
  // **`--no-reconcile`, BECAUSE THIS ARM'S SUBJECT IS 0195 AND NOT 0304.** A
  // daemonless query reconciles before answering since WP-22, and a reconcile
  // adds a `file` row for a document the store already carries prose for -- so
  // the phrase comes back twice, on the same path and the same line, differing
  // only in kind. That is issue 0304, an overlap between two corpora each of
  // which is right about its own scope, and the ruling on it is not this test's.
  // Left un-flagged, this arm would go red for 0304 under a name that says 0195,
  // and whoever met it would read the count and adjust it -- which is how a
  // guard quietly becomes the guard for a question nobody asked it.
  let hits = ok(root, &["search", "kestrel", "--no-reconcile"]);
  let rows: Vec<&str> = hits.lines().collect();
  assert_eq!(
    rows.len(),
    1,
    "a phrase occurring once in one file is ONE row: {hits:?}"
  );
  assert!(
    rows[0].starts_with(&format!("intent/st/ST0001/design.md:{line}  ")),
    "the hit names line {line}, where the file has it: {hits:?}"
  );

  let hits = ok(root, &["search", "pelican"]);
  assert!(
    hits.starts_with("intent/.canon/issues/0001.json  "),
    "a hit inside canon JSON has no prose line, so it prints the file alone: {hits:?}"
  );

  // **AN EDIT ELSEWHERE IN THE FILE LEAVES THE CLAIM TRUE, AND THE LINE MOVES
  // WITH IT** (vc's ruling, 2026-09-12, WP-19 AC-19.5). This asserted that ANY
  // edit since the carry withholds the line, which was whole-file equality
  // answering a broader question than the hit asks: 0195 forbids a line that
  // cannot be VERIFIED, and prepending two lines to this file does not move the
  // phrase out of it. The locator finds the indexed section in the file as it
  // now stands, so the line is re-verified on every search -- which is what
  // this test's name asks for -- and it is absent exactly when the bytes are
  // gone.
  std::fs::write(&design, format!("# Moved\n\n{on_disk}")).expect("edit without a carry");
  let hits = ok(root, &["search", "kestrel"]);
  let moved_line = std::fs::read_to_string(&design)
    .expect("read the edited file")
    .lines()
    .position(|l| l.contains("kestrel"))
    .expect("the phrase survived the edit")
    + 1;
  assert!(
    hits.starts_with(&format!("intent/st/ST0001/design.md:{moved_line}  ")),
    "an edit elsewhere in the file gets a verified line at the phrase's NEW position: {hits:?}"
  );

  // The other sign of the same rule: an edit that takes the indexed bytes away
  // withholds the line. **THE HIT SURVIVES** -- the index still holds the
  // section and the file is still the answer -- so what changes is the claim,
  // not the row.
  std::fs::write(
    &design,
    "# Notes\n\nThe kestrel went somewhere else entirely.\n",
  )
  .expect("edit the indexed bytes away");
  let hits = ok(root, &["search", "combinator"]);
  assert_eq!(
    // Issue 0361: the row carries the matched snippet after the name.
    hits,
    "intent/st/ST0001/design.md  thread  ST0001  design.md  The kestrel combinator returns its first argument.\n",
    "the indexed bytes are gone, so the hit keeps its file and loses its line: {hits:?}"
  );
}

/// **A DOCUMENT THE STORE CARRIES ANSWERS ONCE, AS THE STORE'S** (issue 0304).
///
/// The reproduction dc filed: a thread's `design.md` is realised on disk AND
/// carried by the store as an attachment, so before the fix one phrase on one
/// line came back twice -- `file` from the disk prose corpus and `thread` from
/// the store's doc sections, same path, same line, differing only in kind.
///
/// **NO `--no-reconcile` HERE, AND THAT IS THE POINT.** Since WP-22 a
/// daemonless query reconciles before it answers, which is what made the
/// doubling visible by default rather than only under an explicit rebuild. The
/// sibling arm above flags OUT of the reconcile to stay on 0195; this one flags
/// into it, because the reconcile is the thing under test.
#[test]
fn a_document_the_store_carries_is_not_indexed_again_from_the_disk() {
  let dir = project();
  let root = dir.path();
  ok(root, &["st", "new", "a thread"]);
  let design = root.join("intent/st/ST0001/design.md");
  std::fs::create_dir_all(design.parent().expect("a thread dir")).expect("mkdir");
  std::fs::write(
    &design,
    "# Notes\n\n## Detail\n\nThe kestrel combinator returns its first argument.\n",
  )
  .expect("author prose");
  // **THE CONTROL, and it is what makes the assertion above mean anything.**
  // Prose the store does NOT carry must stay in the disk corpus, or the fix
  // would be "stop indexing markdown" wearing the fix's name. It lives outside
  // any thread directory because EVERY non-view file under one is an
  // attachment -- see `Project::classify` -- so a sibling `notes.md` would be
  // carried too and would prove nothing.
  let note = root.join("docs/design/note.md");
  std::fs::create_dir_all(note.parent().expect("a docs dir")).expect("mkdir docs");
  std::fs::write(&note, "# Note\n\nThe pelican is not in the store.\n").expect("author a note");
  restore_from_disk(root);

  let hits = ok(root, &["search", "kestrel"]);
  let rows: Vec<&str> = hits.lines().collect();
  assert_eq!(
    rows.len(),
    1,
    "a document the store carries is ONE row, not one per corpus: {hits:?}"
  );
  assert!(
    rows[0].contains("  thread  "),
    "and the row is the store's, which is the one that knows what owns it: {hits:?}"
  );

  let hits = ok(root, &["search", "pelican"]);
  assert!(
    hits.starts_with("docs/design/note.md:") && hits.contains("  file  "),
    "prose the store does not carry is still the disk corpus's to answer: {hits:?}"
  );
}

/// **AT-22.2: `--no-reconcile` answers from the index AS IT STANDS, and names
/// what moved underneath it.**
///
/// Both clauses of the criterion, and the second is the one that makes the
/// first safe to offer. An answer from a stale index is a useful thing to ask
/// for -- it is what the index HOLDS, which is a different question from what
/// is in the tree -- but only if the caller is told which of its paths have
/// moved since. Without that it is the estate's dominant defect class again:
/// a confident answer about a file that no longer says what the answer claims.
///
/// **THE HIT SURVIVES AND LOSES ITS LINE, AND THE WARNING IS ON STDERR.** A
/// span is a claim about the disk (issue 0195), so it goes when the bytes move;
/// the file is still the answer. stdout stays parseable for a pipe and the
/// diagnosis goes where diagnoses go.
#[test]
fn no_reconcile_answers_from_the_index_as_it_stands_and_names_what_moved() {
  let dir = project();
  let root = dir.path();
  let note = root.join("docs/note.md");
  std::fs::create_dir_all(note.parent().expect("a docs dir")).expect("mkdir");
  std::fs::write(&note, "# Notes\n\nThe kestrel combinator.\n").expect("author prose");

  // The default path reconciles, so this is what puts the file in the index --
  // and the control that the query finds it at all before anything moves.
  let fresh = run(root, &["search", "kestrel", "--json"]);
  assert_eq!(fresh.status.code(), Some(0), "the fresh query answers");
  let fresh_json = String::from_utf8_lossy(&fresh.stdout).to_string();
  assert!(
    fresh_json.contains("docs/note.md"),
    "the reconcile indexed the file, or nothing below is about staleness: {fresh_json}"
  );
  assert!(
    fresh_json.contains("\"complete\": true"),
    "and the index is complete before the file moves: {fresh_json}"
  );

  // Move the bytes underneath the index without telling it.
  std::fs::write(&note, "# Notes\n\nThe pelican combinator moved here.\n").expect("rewrite");

  let out = run(root, &["search", "kestrel", "--no-reconcile"]);
  assert_eq!(
    out.status.code(),
    Some(0),
    "an answer from a stale index is still an answer, not a refusal"
  );
  let hits = String::from_utf8_lossy(&out.stdout).to_string();
  let said = String::from_utf8_lossy(&out.stderr).to_string();

  // **AS IT STANDS**: the index still holds the old bytes, so the old word is
  // still found. This is the whole point of the flag, and it is what a query
  // that had reconciled would NOT return.
  assert!(
    hits.contains("docs/note.md"),
    "`--no-reconcile` answers from the index as it stands: {hits:?} / {said:?}"
  );

  // **AND NAMES WHAT MOVED**, by path, so the caller can tell which part of the
  // answer to distrust rather than being told the whole thing is suspect.
  assert!(
    said.contains("docs/note.md"),
    "and names the path that moved underneath it: {said:?}"
  );

  // The envelope carries the same fact, which is what the hooks read.
  let json = ok(root, &["search", "kestrel", "--json", "--no-reconcile"]);
  assert!(
    json.contains("\"complete\": false"),
    "the envelope says the index is not complete: {json}"
  );
  assert!(
    json.contains("docs/note.md"),
    "and `stale` names the path, which is what a per-path freshness rule reads: {json}"
  );
}

/// Issue 0305: `intent search --kind def <name>` -- the spelling `CLAUDE.md`,
/// `AGENTS.md`, the Highlander rule and three skills all prescribe as THE
/// prior-art check -- refused with `nothing to search for`. A flag declaring
/// `arity: "1..n"` was built greedy, so `def` and the name were read as two
/// values of `--kind` and the positional query was left empty.
///
/// **THE QUERY-FIRST ARM IS A CONTROL RATHER THAN A SECOND ASSERTION.** In a
/// build carrying no Rust grammar both spellings answer nothing, so an arm
/// that only compared the two would pass on the defect and on an empty index
/// alike. It establishes that the fixture really produces the definition; the
/// flag-first assertion beneath it is then a claim about the PARSER and not
/// about the index.
///
/// **AND IT IS HERE RATHER THAN BESIDE AC-20.3'S OWN TEST FOR THE REASON THE
/// DEFECT SURVIVED AT ALL.** `intentsvcs/tests/symbols_answer_the_highlander_
/// question.rs` drives the same question through the facade, which is BELOW the
/// parser: it builds a `SearchQuery` in Rust and never spells a command line,
/// so no assertion it could carry would have met this. The claim the canon
/// makes is about something a person types.
#[test]
fn the_documented_kind_filter_may_be_written_before_the_query() {
  let dir = project();
  let root = dir.path();
  std::fs::write(root.join("lib.rs"), "fn assemble_widget() {}\n").expect("write source");
  restore_from_disk(root);

  let query_first = ok(root, &["search", "assemble_widget", "--kind", "def"]);
  assert!(
    query_first.contains("lib.rs") && query_first.contains("def"),
    "the fixture produces the definition the canon spelling asks for: {query_first:?}"
  );

  let flag_first = ok(root, &["search", "--kind", "def", "assemble_widget"]);
  assert_eq!(
    flag_first, query_first,
    "a repeatable filter written BEFORE the query does not swallow it, so the \
     canon's own `intent search --kind def <name>` answers"
  );
}
