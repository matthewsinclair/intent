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

use intent_cli::spine::{EXIT_ERROR, EXIT_OK, EXIT_UNAVAILABLE};

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

/// Seed a thread whose contract is satisfied, as committed canon.
///
/// **A fresh store ingests the working tree on first open**, so this is the
/// real ingest path rather than a back door into the database -- the same one
/// a clone of a real repository takes. It has to be: `intent sync` refuses to
/// guess a direction (AC-03.9's selector is not built), so a thread written
/// beside an EXISTING store would never reach it.
///
/// `AC-01.1` is not an arbitrary id. The id carries the work package, so a
/// criterion numbered `01` is what makes `wp done ST0001/01` evaluable at all;
/// under any other number the WP scope would match nothing and block for
/// arithmetic reasons rather than contractual ones.
fn seed_closeable_thread(root: &Path) {
  let dir = root.join("intent/.canon/st");
  std::fs::create_dir_all(&dir).expect("mkdir");
  std::fs::write(
    dir.join("ST0001.json"),
    r#"{
  "schema": "intent/thread@3.0",
  "id": "ST0001",
  "slug": "a-thread",
  "title": "A thread",
  "status": "wip",
  "created": "2026-08-15",
  "objective": "",
  "context": "",
  "wps": [ { "seq": 1, "title": "A package", "scope": "S", "status": "wip" } ],
  "criteria": [
    { "id": "AC-01.1", "text": "It works", "kind": "non-test",
      "state": { "is": "satisfied", "evidence": "verified by hand" } }
  ]
}
"#,
  )
  .expect("write canon");
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

/// **`st new` then `st start` lands at WIP, without `st triage` in between.**
///
/// The ratified machine made `Triage -> NotStarted -> Wip` the only route, and
/// the first human to type it said so: _"this is STOOPID... I'd expect it to
/// just end up at WIP"_ (hv, 2026-08-18). v2 was two commands and the ratified
/// path cost three, on the only route anyone actually walks.
///
/// hv amended the machine rather than defaulting `st new` to `--start`
/// (`data-model.md:430`, :436): starting work on a triaged item IS accepting
/// it, so `Triage -> Wip` is a legitimate compound rather than a bypass.
/// Defaulting `st new` instead would have left `Triage` with almost no
/// population, which is the state's whole justification.
///
/// **Driven through the binary, because this class only ever showed up when a
/// person typed the sequence** -- the suite expressed the semantics of every
/// arm and could not express that the route was two commands too long.
#[test]
fn start_is_legal_straight_from_triage() {
  let dir = project();
  let root = dir.path();

  ok(root, &["st", "new", "Add a Rust-based CLI"]);
  assert_eq!(
    ok(root, &["st", "start", "ST0001"]).trim(),
    "ok: ST0001 started"
  );

  let shown = ok(root, &["st", "show", "ST0001"]);
  assert!(
    shown.contains("status: WIP"),
    "two commands, and the thread is where the operator expected it: {shown}"
  );
}

/// The full lifecycle, through the binary, writing real canon and real views.
#[test]
fn a_thread_moves_through_its_lifecycle_and_writes_canon_and_views() {
  let dir = project();
  let root = dir.path();

  ok(root, &["st", "new", "Add a Rust-based CLI"]);
  assert!(root.join("intent/.canon/st/ST0001.json").is_file(), "canon");
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

  // `--status all` because bare `st list` shows WIP ONLY, as v2 does, and a
  // fresh thread is not WIP. The bare form used to list everything, which is
  // what made this assertion pass before the filter was ported.
  //
  // **`Triage` rather than `Not Started` since the machines were ratified**:
  // `st new` enters at triage and `st triage` accepts it into the backlog.
  let listed = ok(root, &["st", "list", "--status", "all"]);
  assert!(listed.contains("ST0001"), "{listed}");
  assert!(listed.contains("Triage"), "{listed}");
  assert!(
    ok(root, &["st", "list"]).lines().count() == 2,
    "and the bare form is header + separator only, not an error and not silence"
  );

  // **`st start` from `Triage` used to be asserted here as a REFUSAL and is
  // now legal** (hv, 2026-08-18, `data-model.md:430`). The compound edge has
  // its own test below; this one keeps driving the long way round, arm by arm,
  // because every intermediate verb still has to work.

  // **Now drive the machine, arm by arm, asserting each verb's own success
  // line.** The previous version of this block stopped at the refusal above
  // and left a comment saying the rest was a surface gap owed by ic. It was
  // not: the dispatch rows had landed, the facade had every verb, and what was
  // missing was the wiring in this crate's renderer -- mine. The test could
  // not tell, because it asserted only that `st start` was REFUSED, and an
  // unwired verb refuses too. A test written to make an ask concrete made the
  // ask invisible.
  //
  // So every assertion below is on a SUCCESS line, which `unwired` cannot
  // produce: it exits 1 and says "not implemented yet". Exit 0 plus the verb's
  // own wording is the only shape that distinguishes a wired arm from a
  // present-but-unbuilt one.
  assert_eq!(
    ok(root, &["st", "triage", "ST0001"]).trim(),
    "ok: ST0001 accepted out of triage",
    "and the message names where the thread LANDED -- `triage` reads in both directions"
  );
  assert_eq!(
    ok(root, &["st", "start", "ST0001"]).trim(),
    "ok: ST0001 started"
  );
  assert_eq!(
    ok(
      root,
      &["st", "hold", "ST0001", "--reason", "waiting on the fleet"]
    )
    .trim(),
    "ok: ST0001 on hold"
  );
  assert_eq!(
    ok(root, &["st", "resume", "ST0001"]).trim(),
    "ok: ST0001 resumed"
  );

  ok(root, &["wp", "new", "ST0001", "Ingest and views"]);
  ok(root, &["wp", "start", "ST0001/01"]);
  assert_eq!(
    ok(root, &["wp", "unstart", "ST0001/01"]).trim(),
    "ok: ST0001/01 back to not started"
  );
  ok(root, &["wp", "start", "ST0001/01"]);
  let wps = ok(root, &["wp", "list", "ST0001"]);
  assert!(
    wps.lines().any(|l| l.starts_with("01 ")),
    "the WP column is v2's bare sequence number: {wps}"
  );

  // `cancel` and `reinstate` are the machine's other exit and re-entry, and
  // neither consults the gate -- so the round trip runs here rather than in
  // the closed-state test below.
  assert_eq!(
    ok(root, &["st", "cancel", "ST0001", "--reason", "overtaken"]).trim(),
    "ok: ST0001 cancelled"
  );
  assert_eq!(
    ok(
      root,
      &["st", "reinstate", "ST0001", "--reason", "wanted again"]
    )
    .trim(),
    "ok: ST0001 reinstated to the backlog",
    "a reinstated thread lands in the BACKLOG, not back at wip, and the message says so"
  );
}

/// `st new -s` COMPOSES two declared transitions and records both.
///
/// **The final status is not the test**, and that is the whole point of vc's
/// ruling. Constructing the thread directly in `Wip` produces exactly the same
/// status, so an assertion on status alone passes on the defect -- while the
/// audit trail shows a thread that was never triaged and the machine acquires
/// an effective `Triage -> Wip` edge nobody declared.
///
/// So this reads the EVENT LOG, which is where the difference is visible. The
/// log is written out by the routine sync direction, which is also the first
/// consumer proving `--to-disk` carries history rather than just entities.
#[test]
fn st_new_start_composes_the_two_transitions_rather_than_constructing_the_end_state() {
  let dir = project();
  let root = dir.path();
  ok(root, &["st", "new", "A thread", "-s"]);

  assert!(
    ok(root, &["st", "show", "ST0001"]).contains("status: WIP"),
    "the flag still does what v2's flag did, from the operator's side"
  );

  ok(root, &["sync", "--to-disk"]);
  // **THE LOG IS NO LONGER PROJECTED INTO THE TREE (D53), so the operator's
  // route to it is the exporter.** Read through the SHIPPED VERB rather than
  // through the store directly: this file's whole job is to drive what a person
  // can actually type, and a test that reached past the CLI would go green on a
  // history no command could show anybody.
  let bundle: serde_json::Value =
    serde_json::from_str(&ok(root, &["export"])).expect("the export bundle is JSON");
  let ops: Vec<String> = bundle["events"]
    .as_array()
    .expect("the bundle carries the history")
    .iter()
    .map(|e| {
      e["op"]
        .as_str()
        .expect("every envelope names its op")
        .to_string()
    })
    .collect();

  assert_eq!(
    ops,
    vec!["st.new", "st.triage", "st.start"],
    "both transitions are recorded, in order -- a skipped state is not a cosmetic gap, it is a mutation that never happened in the log that exists to say what happened"
  );
}

/// The two `reopen` verbs, driven to success -- the machine has no terminal
/// states, and this is the test that proves it from the operator's side.
///
/// They need a CLOSED unit, so they need a gate that passes, so they need a
/// contract. **A thread created by `st new` in this session cannot supply
/// one**: no CLI verb creates an acceptance criterion, and `ac satisfy` only
/// moves one that already exists. Criteria enter the store by ingest, so the
/// fixture seeds them the way a real repository does -- as committed canon a
/// fresh open reads.
#[test]
fn a_closed_thread_and_work_package_can_both_be_reopened() {
  let dir = project();
  let root = dir.path();
  seed_closeable_thread(root);

  assert!(
    ok(root, &["ac", "gate", "ST0001"]).contains("PASS"),
    "precondition: the seeded contract is satisfied, so `done` is reachable"
  );

  ok(root, &["wp", "done", "ST0001/01"]);
  assert_eq!(
    ok(
      root,
      &[
        "wp",
        "reopen",
        "ST0001/01",
        "--reason",
        "an AC arrived after it closed"
      ]
    )
    .trim(),
    "ok: ST0001/01 reopened"
  );
  // **`WIP`, not `wip` -- this assertion was PINNING A DIVERGENCE.** `wp show`
  // printed `enum_str` where `st show`, `issues show` and the generated `info.md`
  // all print `display()`, and v2 implements `wp show` by catting that very file.
  // The assertion's subject is that the reopen moved the package, which it still
  // is; the spelling it happened to capture was the defect, and a test that
  // records a defect as expected output is what makes the defect permanent.
  assert!(
    ok(root, &["wp", "show", "ST0001/01"]).contains("status: WIP"),
    "and the reopen actually moved it, rather than only printing that it had"
  );

  ok(root, &["st", "done", "ST0001"]);
  assert_eq!(
    ok(
      root,
      &["st", "reopen", "ST0001", "--reason", "the contract grew"]
    )
    .trim(),
    "ok: ST0001 reopened"
  );
  let shown = ok(root, &["st", "show", "ST0001"]);
  assert!(shown.contains("status: WIP"), "{shown}");
  assert!(
    !shown.contains("completed:"),
    "and reopening CLEARS the completion date, which would otherwise outlive the state it recorded: {shown}"
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
///
/// **The thread is driven to `wip` first, and until the gate moved behind the
/// transition check this test never reached the gate at all.** A freshly created
/// thread is in `triage`, where `st.done` is not a declared transition; the old
/// ordering ran the gate before checking the machine, so a thread that could not
/// legally be closed was answered with a verdict about its acceptance criteria.
/// The refusal is now the accurate one -- `st.done` is declared only from `wip` --
/// which meant this test, whose whole subject is the gate, was passing on a path
/// that never consulted it.
#[test]
fn closing_through_the_cli_is_gated_and_says_why() {
  let dir = project();
  let root = dir.path();
  ok(root, &["st", "new", "a thread"]);
  ok(root, &["st", "triage", "ST0001"]);
  ok(root, &["st", "start", "ST0001"]);

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
    // **The set comes from `spine`, not from literals, and that is the fix
    // rather than a tidy-up.** This read `code == 0 || code == 1` while its
    // message talked about 101 -- so a legitimate `EXIT_UNAVAILABLE` would
    // have failed it, reported as a panic, and sent the reader looking for a
    // crash that never happened. The assertion and the message disagreed about
    // what was being checked, and the message was the one telling the truth.
    let declared = [EXIT_OK, EXIT_ERROR, EXIT_UNAVAILABLE];
    assert!(
      declared.contains(&code),
      "`intent {}` exited {code}, which is not one of this tool's declared codes {declared:?} -- 101 is a panic, and anything else is an exit nobody chose\nstderr: {}",
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
    unwired_err.contains("not implemented yet"),
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

  // D37, on the message most likely to reach for an internal citation: this
  // one used to read "(ST0056 WP-06)", naming the work package that owed the
  // verb. Asserted here rather than left to review because the pressure to put
  // it back is real -- the id is genuinely the most informative thing WE know,
  // and it is information about us, not about the operator's problem.
  for leak in ["ST00", "WP-", "AC-", "AT-"] {
    assert!(
      !unwired_err.contains(leak),
      "shipped output carries Intent's own project-management state ({leak}): {unwired_err}"
    );
  }
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
  // **The refusal's remedy names the SAFE direction only.** AC-03.9 is explicit
  // that a remedy sending an operator to the destructive direction to recover
  // is itself the defect, so `--to-store` may appear in the refusal as a COST
  // and never on the `remedy:` line.
  let remedy = said
    .lines()
    .find(|l| l.contains("remedy:"))
    .expect("the refusal carries a remedy");
  assert!(remedy.contains("--to-disk"), "{remedy:?}");
  assert!(
    !remedy.contains("--to-store"),
    "no remedy sends an operator to the destructive direction: {remedy:?}"
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

/// AC-03.9's selector: both directions RUN, naming both refuses, and the
/// destructive one states what it will overwrite BEFORE it overwrites it.
///
/// **The rows were declared for a day before anything read them.** `run`
/// matched `Some(("sync", _))` and discarded the `ArgMatches`, so clap accepted
/// `--to-disk` and the renderer never saw it -- while the bare verb's remedy
/// told the operator the selector was not built and `sync --help` listed it.
/// A surface that advertises a flag and an implementation that denies it exists
/// disagree in the one place a user checks, so this test drives the flags
/// rather than asserting they parse.
#[test]
fn sync_runs_the_direction_it_is_given_and_names_the_loss_before_taking_it() {
  let dir = project();
  let root = dir.path();
  ok(root, &["st", "new", "A thread"]);

  assert!(
    ok(root, &["sync", "--to-disk"]).starts_with("ok: extract written"),
    "the routine direction runs and says what it did"
  );

  // Opposite directions over the same two endpoints: running both would make
  // one pointless and the other authoritative by accident of ordering.
  let both = run(root, &["sync", "--to-disk", "--to-store"]);
  assert_eq!(both.status.code(), Some(1));
  assert!(
    String::from_utf8_lossy(&both.stderr).contains("opposite directions"),
    "naming both chooses neither, and says so"
  );

  // The discriminating case: make the EXTRACT disagree with the store, then
  // restore from it. The restore must name the overwrite before taking it --
  // a summary afterwards is a receipt for a loss the operator needed one
  // moment earlier.
  let canon = root.join("intent/.canon/st/ST0001.json");
  let text = std::fs::read_to_string(&canon).expect("canon");
  std::fs::write(
    &canon,
    text.replace("\"A thread\"", "\"A stale title from the extract\""),
  )
  .expect("write canon");

  let restored = run(root, &["sync", "--to-store"]);
  assert_eq!(restored.status.code(), Some(0));
  let warned = String::from_utf8_lossy(&restored.stderr).to_string();
  assert!(
    warned.contains("OVERWRITES") && warned.contains("ST0001"),
    "the destructive direction names WHAT it overwrites, on stderr: {warned:?}"
  );
  assert!(
    ok(root, &["st", "show", "ST0001"]).contains("A stale title from the extract"),
    "and it really did overwrite -- the warning describes an act, not an intention"
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

/// The table tracks the terminal width in BOTH directions: it pads up to the
/// width and clips down to it.
///
/// **THIS TEST'S PROPERTY WAS RE-CUT ON 2026-08-25 AND THE OLD ONE IS NOT
/// MERELY RELAXED, IT IS REVERSED.** It read *content-fit is the FLOOR -- a
/// narrow terminal stops padding, it never truncates*, which was a faithful pin
/// of v2 (`render_table`: *content-fit is the floor, so nothing is ever
/// truncated*). **What neither implementation's comment said is the
/// consequence: one oversized cell sets the width of EVERY row.** Measured, an
/// `issues list` rendered 312 columns into an 80-column terminal because a
/// single title ran to 287 characters. hv ruled truncate-with-ellipsis, so the
/// deviation is declared and this pins the new contract.
///
/// **THE NEW ASSERTIONS ARE STRICTLY STRONGER THAN THE ONES THEY REPLACE.** The
/// old test asserted `wide > narrow` -- a relationship that holds for a table
/// that merely stops padding. This asserts EXACT equality with the requested
/// width at both ends, which the old behaviour could not satisfy.
///
/// Measured byte-identical against the v2 binary at COLUMNS 250/130/100/60
/// before the ruling; that measurement is what the deviation is declared
/// AGAINST, and it is kept here rather than deleted because a deviation whose
/// baseline is gone cannot be checked.
#[test]
fn the_table_tracks_the_terminal_width_in_both_directions() {
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
      !text.is_empty(),
      "the command produced output at COLUMNS={cols}: {text:?}"
    );
    text.lines().map(|l| l.chars().count()).max().unwrap_or(0)
  };

  // **EXACT, NOT A RANGE, AT BOTH ENDS.** Padding up to 250 and clipping down
  // to 60 are the same property measured in two directions, and only the second
  // one distinguishes this contract from the one it replaced.
  assert_eq!(width_at("250"), 250, "pads up to a wide terminal");
  assert_eq!(width_at("60"), 60, "and clips down to a narrow one");

  // The clip is visible rather than silent: a reader must be able to tell a
  // shortened value from a short one.
  let narrow_text = {
    let out = Command::new(env!("CARGO_BIN_EXE_intent"))
      .args(["st", "list", "--status", "all"])
      .current_dir(root)
      .env("COLUMNS", "60")
      .output()
      .expect("run");
    String::from_utf8_lossy(&out.stdout).to_string()
  };
  assert!(
    narrow_text.contains('\u{2026}'),
    "a clipped cell says so: {narrow_text:?}"
  );
  assert!(
    narrow_text.contains("Slug") && narrow_text.contains("Completed"),
    "and every header survives, or the columns stop being identifiable: {narrow_text:?}"
  );

  // **THE REVERSED ARM, KEPT RATHER THAN DELETED.** This read *at 60 columns the
  // table is WIDER than 60, because the slug alone does not fit and nothing is
  // ever cut* -- the exact behaviour hv ruled against. It is re-cut instead of
  // removed because it is the one assertion that FAILS under the old renderer,
  // so it is what makes this test a check rather than a description.
  assert!(
    !narrow_text.lines().any(|l| l.chars().count() > 60),
    "no line escapes the width, including the one with the long slug: {narrow_text:?}"
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
  // No `st start` here: this test is about the sync table sharing the list
  // renderer, and driving a status was only ever incidental. A test that
  // reaches for an unrelated verb to make state acquires a dependency it did
  // not need -- which is the whole reason for leaving it out, and it stays the
  // reason now that `st triage` is wired and the state IS reachable.

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

  let canon = std::fs::read_to_string(root.join("intent/.canon/st/ST0001.json")).expect("canon");
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

  let canon_path = root.join("intent/.canon/st/ST0001.json");
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
  // store before it can be rendered -- the disk -> db direction.
  //
  // **This line used to delete the store**, because the CLI could not spell
  // that direction and a cold open re-ingests from the files. It was a D36
  // violation left in deliberately, on the argument that hiding it behind a
  // clone fixture would remove the only pressure to ship the selector -- a
  // later D36 sweep would then come back clean while the gap persisted. This
  // is the named cleanup vc attached to AC-03.9, taken now that the flag it
  // was waiting for exists.
  ok(root, &["sync", "--to-store"]);

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

/// **The reason four verbs DEMAND, finally shown to the human they demanded it
/// from.** `st hold`, `st cancel`, `st reopen` and `wp reopen` refuse without a
/// reason; the value then reached `thread.json` and the GraphQL SDL and no face
/// a person reads. Reported by ic, who drove `st hold` and then `wp reopen`.
///
/// **THE CANON ASSERTION IS THE CONTROL AND IT IS LOAD-BEARING** (ic). Their
/// first drive of the work-package half hit a fixture that never fired -- the
/// gate refused the `wp done`, so `wp reopen` answered `ok: already WIP`
/// without writing anything, and every read face came back empty. Emptiness
/// from a face that does not render and emptiness from a verb that never
/// recorded are indistinguishable when you only ask the face. So: prove the
/// value reached canon, THEN ask the face.
#[test]
fn a_status_reason_reaches_a_human_face_on_both_entities() {
  let dir = project();
  let root = dir.path();
  seed_closeable_thread(root);

  // ---- thread half ----
  ok(
    root,
    &["st", "hold", "ST0001", "--reason", "CANARY-THREAD-XYZZY"],
  );

  let canon = std::fs::read_to_string(root.join("intent/.canon/st/ST0001.json")).expect("canon");
  assert!(
    canon.contains("CANARY-THREAD-XYZZY"),
    "CONTROL: the verb must have recorded the reason before the face is worth asking:\n{canon}"
  );

  let shown = ok(root, &["st", "show", "ST0001"]);
  assert!(
    shown.contains("CANARY-THREAD-XYZZY"),
    "st show must render the reason it refused to proceed without:\n{shown}"
  );
}

/// The work-package half, guarded by `wp reopen` and left open by any fix
/// scoped to `st show`.
#[test]
fn a_work_package_status_reason_reaches_the_human_face_too() {
  let dir = project();
  let root = dir.path();
  seed_closeable_thread(root);

  ok(root, &["wp", "done", "ST0001/01"]);
  ok(
    root,
    &["wp", "reopen", "ST0001/01", "--reason", "CANARY-WP-PLUGH"],
  );

  let canon = std::fs::read_to_string(root.join("intent/.canon/st/ST0001.json")).expect("canon");
  assert!(
    canon.contains("CANARY-WP-PLUGH"),
    "CONTROL: `wp reopen` must have fired -- an `already WIP` no-op writes nothing and makes the face's silence meaningless:\n{canon}"
  );

  let shown = ok(root, &["wp", "show", "ST0001/01"]);
  assert!(
    shown.contains("CANARY-WP-PLUGH"),
    "wp show must render it:\n{shown}"
  );
}

/// **The scope reaches the facade FROM THE COMMAND LINE**, which the unit
/// tests cannot show.
///
/// `sync_scope.rs` proves the facade narrows when handed a scope. It cannot
/// prove that typing one produces a scope: the dispatch table declares `id` at
/// `0..n`, and clap accepts a declaration whether or not anything reads it --
/// the class this file's `sync` comment already names. **A declared narrowing
/// nothing consumes is worse than an undeclared one, because the operator
/// believes their peers' files are safe.**
///
/// The unscoped run is the control. "1 thread" alone is also what a scope that
/// silently matched nothing would print if the count came from the wrong place.
#[test]
fn sync_narrows_to_the_threads_named_on_the_command_line() {
  let dir = project();
  let root = dir.path();
  ok(root, &["st", "new", "First"]);
  ok(root, &["st", "new", "Second"]);

  let scoped = ok(root, &["sync", "--to-disk", "ST0001"]);
  let all = ok(root, &["sync", "--to-disk"]);

  assert!(
    all.contains("2 thread"),
    "precondition: the unscoped run covers both threads, so the scoped count below is a NARROWING \
     rather than the only number this can print. got: {all:?}"
  );
  assert!(
    scoped.contains("1 thread"),
    "naming one thread did not narrow the run: {scoped:?}"
  );
}

/// A scope naming no such thread refuses, and names the id it could not find.
///
/// The refusal is the arm that proves the ids reach the facade rather than
/// being parsed and dropped: a discarded argument produces a whole-estate
/// success, which is indistinguishable from a completed sync at the terminal
/// and leaves the operator believing a thread they mistyped has been saved.
#[test]
fn a_sync_scope_naming_no_such_thread_refuses_and_says_which() {
  let dir = project();
  let root = dir.path();
  ok(root, &["st", "new", "First"]);

  let out = run(root, &["sync", "--to-disk", "ST9999"]);
  assert_ne!(
    out.status.code(),
    Some(0),
    "a scope naming a thread that does not exist SUCCEEDED, so the ids are being dropped rather \
     than honoured"
  );
  let said = format!("{}{}", stdout(&out), String::from_utf8_lossy(&out.stderr));
  assert!(
    said.contains("ST9999"),
    "the refusal does not name the id that could not be found, so the operator cannot tell which \
     of their arguments was wrong: {said:?}"
  );
}

/// **THE VERB THAT CLOSED AC-08.5's LAST FIELD-AXIS GAP.**
///
/// `Attachment.text` was writable through `Facade::put` with no route on the
/// mutation surface, so the criterion's first clause failed on a field whose own
/// refusal correctly said *there is no CLI verb for this today*. This is that
/// verb, driven at the CLI because what it writes is an attachment's CONTENT
/// rather than a field of a document.
///
/// **THE SPELLING IS PROVISIONAL AND ROUTED TO hv** (vc authorised the
/// capability and declined the name, 2026-08-25). The criterion asks whether the
/// field is settable through the mutation surface and has no opinion on what the
/// verb is called, **so a rename does not move the row** -- and vc refused to
/// pick the name under the pen because a name chosen inside a fix becomes the
/// ruling by default, which this criterion produced twice in a day.
///
/// **TEXT ONLY, AND THE REFUSAL SAYS SO RATHER THAN MANGLING.** `put`'s
/// attachment arm is `Attachment::new(path, body)`, which takes a string, so a
/// non-UTF-8 file has no route through here. Claiming to carry bytes this path
/// cannot carry would be the false-remedy class that cost this criterion four
/// instances in one day.
#[test]
fn st_attach_writes_an_attachments_content_and_refuses_what_it_cannot_carry() {
  let dir = project();
  let root = dir.path();
  ok(root, &["st", "new", "attachment probe"]);

  let src = root.join("design.md");
  std::fs::write(&src, "# design\n\nthe original\n").expect("write source");
  let from = src.to_str().expect("utf-8 path");

  // **THE ID IS NORMALISED ON THE WAY IN**, so the verb inherits the five v2
  // spellings rather than declaring its own.
  let said = ok(root, &["st", "attach", "1", "design.md", "--from", from]);
  assert!(
    said.contains("ST0001"),
    "the reply names the canonical id: {said}"
  );

  let canon = std::fs::read_to_string(root.join("intent/.canon/st/ST0001.json")).expect("canon");
  assert!(
    canon.contains("the original"),
    "the content reached canon: {canon}"
  );

  // **OVERWRITING IS THE DECLARED BEHAVIOUR AND IT IS WHY THE ROW IS
  // `one-way`.** Nothing keeps the previous bytes, which is also why the verb is
  // withheld from MCP: an agent must not irreversibly replace an attachment.
  std::fs::write(&src, "# design\n\nrewritten\n").expect("rewrite source");
  ok(root, &["st", "attach", "s1", "design.md", "--from", from]);
  let canon = std::fs::read_to_string(root.join("intent/.canon/st/ST0001.json")).expect("canon");
  assert!(canon.contains("rewritten"), "the second write landed");
  assert!(
    !canon.contains("the original"),
    "and replaced rather than appended: {canon}"
  );

  // Refusals, each naming what is wrong rather than what is missing generally.
  let binary = root.join("opaque.dat");
  std::fs::write(&binary, [0x62, 0x80, 0xff]).expect("write bytes");
  let out = run(
    root,
    &[
      "st",
      "attach",
      "1",
      "opaque.dat",
      "--from",
      binary.to_str().expect("path"),
    ],
  );
  assert_eq!(out.status.code(), Some(1), "a non-UTF-8 file is refused");
  let said = String::from_utf8_lossy(&out.stderr).to_string();
  assert!(
    said.contains("not UTF-8"),
    "and says which half is missing: {said}"
  );
  assert!(
    !said.contains("sync --to-store"),
    "**AND NAMES NO ROUTE THAT DOES NOT REACH THE OUTCOME.** The first draft of this remedy \
     claimed an opaque attachment reaches canon through `sync --to-store` after the file is on \
     disk; `ingest` fills those bytes from a sidecar canon ALREADY records, so a dropped file \
     reaches nothing. That would have been the fifth false remedy of the day, drafted inside \
     the fix for the fourth: {said}"
  );

  let out = run(root, &["st", "attach", "99", "x.md", "--from", from]);
  assert_eq!(out.status.code(), Some(1), "an absent thread is refused");
  assert!(
    String::from_utf8_lossy(&out.stderr).contains("ST0099"),
    "and the refusal names the NORMALISED id rather than the spelling typed"
  );
}
