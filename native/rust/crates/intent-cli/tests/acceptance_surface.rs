//! **`ac status` and `at lint`: two read surfaces that were answering in
//! someone else's voice and in no voice at all.**
//!
//! Both were found by vc measuring v3 against v2 side by side on Intent's own
//! migrated estate, and both are the same class one step apart -- **a command
//! whose output does not describe the question it was asked.**
//!
//! - `ac status` printed the CLOSE GATE's line: `gate: ST0056 BLOCKED -- 46/114
//!   satisfied; unsatisfied: ...` beside **exit 0**. The exit code was right and
//!   the prefix was wrong, and that combination is the harm rather than a
//!   cosmetic complaint: a consumer reading the text gets a refusal, a consumer
//!   reading the code gets a pass, and the pre-commit gate is a consumer.
//! - `at lint` printed **zero bytes at exit 0** on a conforming thread. **A lint
//!   that says nothing and succeeds is byte-identical to a lint that did not
//!   run**, on the one surface a reader trusts the AT contract on.
//!
//! # Why the denominator is the point of the lint line
//!
//! v2 prints `lint: <target> ok -- <N> AT row(s) conform`. The `ok` alone is
//! equally true of a thread with no rows at all, so it is the COUNT that
//! distinguishes "I checked 114 rows and they conform" from "there was nothing
//! here to check" -- the same distinction `intent search` had to draw when an
//! unpopulated index answered every query the way a genuine miss does.
//!
//! **So a fixture with one row count cannot test it**, which is why
//! `the_row_count_is_the_rows_examined_and_not_a_constant` drives two threads
//! with different counts through one binary. Asserting `2 AT row(s)` against a
//! fixture with two rows passes just as well against a hardcoded 2.
//!
//! # Measured against v2 rather than against a fixture
//!
//! All 56 threads of Intent's own estate were put through `bin/intent` on a
//! pristine extract and through this binary on a migrated copy: **the `at lint`
//! line agrees on all 56, across eleven distinct row counts (0, 2, 3, 5, 6, 7,
//! 9, 10, 14, 16, 19, 114), and `ac status` agrees on all 13 threads that carry
//! a contract**, PASS and BLOCKED alike. The 43 that disagree are the
//! contract-free threads, where v3 reports the diagnosis (`the thread has zero
//! acceptance criteria ...`) and v2 reports `0/0 satisfied`. That divergence
//! **predates this fix and is narrowed by it** -- before it, all 56 disagreed --
//! and the residue is a register question for the ruled voice, not a defect in
//! the split.

use std::path::Path;
use std::process::{Command, Output};

fn project() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let config = dir.path().join("intent").join(".config");
  std::fs::create_dir_all(&config).expect("mkdir");
  std::fs::write(
    config.join("config.json"),
    "{\n  \"intent_version\": \"3.0.0\",\n  \"project_name\": \"Acceptance\",\n  \"author\": \"cc\",\n  \"intent_dir\": \"intent\",\n  \"languages\": [\"rust\"]\n}\n",
  )
  .expect("write config");
  dir
}

/// A thread carrying exactly the criteria and tests handed in.
///
/// Canon is written directly rather than built through the verbs: the point of
/// each fixture here is a particular CONTRACT SHAPE, and reaching it through
/// `ac`/`at` movements would make the setup depend on the surfaces under test.
fn seed(root: &Path, id: &str, criteria: &str, tests: &str) {
  // Canon is flat under `intent/.canon/st/` after WP-01; the id is the file
  // name, not a directory. `intent/st/` keeps the VIEWS and nothing else.
  let dir = root.join("intent/.canon/st");
  std::fs::create_dir_all(&dir).expect("mkdir");
  std::fs::write(
    dir.join(format!("{id}.json")),
    format!(
      r#"{{
  "schema": "intent/thread@3.0",
  "id": "{id}",
  "slug": "a-thread",
  "title": "A thread",
  "status": "wip",
  "created": "2026-08-17",
  "objective": "",
  "context": "",
  "wps": [ {{ "seq": 1, "title": "A package", "scope": "S", "status": "wip" }} ],
  "criteria": [{criteria}],
  "tests": [{tests}]
}}
"#
    ),
  )
  .expect("write canon");
}

fn criterion(id: &str) -> String {
  format!(
    r#"{{ "id": "{id}", "text": "It works", "kind": "non-test", "state": {{ "is": "unsatisfied" }} }}"#
  )
}

fn at_row(id: &str, covers: &str) -> String {
  format!(r#"{{ "id": "{id}", "covers": ["{covers}"], "kind": "test", "status": "to-write" }}"#)
}

fn run(root: &Path, args: &[&str]) -> Output {
  Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(root)
    .output()
    .expect("run the v3 binary")
}

fn stdout(out: &Output) -> String {
  String::from_utf8_lossy(&out.stdout).trim().to_string()
}

/// Satisfy a criterion through the real verb, so the state under test is one
/// the tool can actually reach.
fn satisfy(root: &Path, id: &str, ac: &str) {
  let out = run(root, &["ac", "satisfy", id, ac, "--evidence", "checked"]);
  assert_eq!(
    out.status.code(),
    Some(0),
    "seeding `{ac}` satisfied must succeed:\n{}",
    String::from_utf8_lossy(&out.stderr)
  );
}

/// A three-criterion contract with one satisfied -- BLOCKED, with two ids the
/// gate is waiting on.
fn blocked(root: &Path) {
  let criteria = ["AC-01.1", "AC-01.2", "AC-01.3"].map(criterion).join(", ");
  seed(root, "ST0001", &criteria, &at_row("AT-01.1", "AC-01.1"));
  satisfy(root, "ST0001", "AC-01.1");
}

// ---------------------------------------------------------------------------
// `ac status` -- the read, in the read's voice
// ---------------------------------------------------------------------------

#[test]
fn ac_status_reports_the_arithmetic_in_its_own_voice() {
  let dir = project();
  blocked(dir.path());
  let out = run(dir.path(), &["ac", "status", "ST0001"]);

  assert_eq!(
    stdout(&out),
    "ac: 1/3 satisfied -- BLOCKED",
    "v2's shape verbatim (`bin/intent_acceptance:937`): the counts first, the \
     verdict last, and no scope label -- the caller just typed the target"
  );
  assert_eq!(out.status.code(), Some(0), "`status` is a read");
}

/// **The defect, stated as the thing that must not come back.** Two assertions
/// rather than one, because the line was wrong in two independent ways and a
/// single equality would not say which.
#[test]
fn ac_status_prints_neither_the_gates_prefix_nor_the_gates_enumeration() {
  let dir = project();
  blocked(dir.path());
  let printed = stdout(&run(dir.path(), &["ac", "status", "ST0001"]));

  assert!(
    !printed.contains("gate:"),
    "a line reading `gate: ... BLOCKED` beside exit 0 is what a consumer \
     misreads, and the pre-commit gate is a consumer: {printed:?}"
  );
  assert!(
    !printed.contains("AC-01.2"),
    "the enumeration is `ac list`'s job; `status` is the count. 68 ids in a \
     one-line summary is the shape v2 deliberately did not have: {printed:?}"
  );
}

#[test]
fn ac_status_says_pass_when_the_contract_is_met() {
  let dir = project();
  let root = dir.path();
  seed(
    root,
    "ST0001",
    &[criterion("AC-01.1"), criterion("AC-01.2")].join(", "),
    &at_row("AT-01.1", "AC-01.1"),
  );
  satisfy(root, "ST0001", "AC-01.1");
  satisfy(root, "ST0001", "AC-01.2");

  let out = run(root, &["ac", "status", "ST0001"]);
  assert_eq!(stdout(&out), "ac: 2/2 satisfied -- PASS");
  assert_eq!(out.status.code(), Some(0));
}

/// **THE CONTROL ON THE SPLIT.** The enumeration moved SURFACE; it did not get
/// deleted. If `status` were made quiet by dropping the unsatisfied ids from
/// the verdict rather than from one renderer, the gate would lose the only
/// thing that makes its refusal actionable and every test above would still be
/// green.
#[test]
fn the_close_gate_still_names_what_it_is_waiting_on() {
  let dir = project();
  blocked(dir.path());
  let out = run(dir.path(), &["ac", "gate", "ST0001"]);

  assert_eq!(
    stdout(&out),
    "gate: ST0001 BLOCKED -- 1/3 satisfied; unsatisfied: AC-01.2 AC-01.3",
    "the gate's line is unchanged (D17)"
  );
  assert_eq!(
    out.status.code(),
    Some(1),
    "and the gate carries its verdict"
  );
}

/// **The verdict goes where the detail lets it go** (vc ruled, 2026-08-17).
///
/// A tally is a phrase, so the verdict trails it and matches v2 byte for byte.
/// A diagnosis is a sentence, and `... declare 'acceptance: exempt'. --
/// BLOCKED` puts the verdict after a full stop -- which reads badly, and **a
/// line that reads badly is one somebody later "improves"**, on 43 of Intent's
/// own 56 threads. So the diagnosis case leads with the verdict instead.
///
/// **This is a property of the ARM, never a test on the string.** Sniffing for
/// a trailing full stop would be a parser of our own output: right until the
/// format moves, then silently wrong.
#[test]
fn ac_status_leads_with_the_verdict_when_the_detail_is_a_diagnosis() {
  let dir = project();
  seed(dir.path(), "ST0001", "", "");

  assert_eq!(
    stdout(&run(dir.path(), &["ac", "status", "ST0001"])),
    "ac: BLOCKED -- the thread has zero acceptance criteria (empty contract). Define ACs, or declare 'acceptance: exempt'.",
    "nothing trails the full stop"
  );
}

/// **The tally case keeps v2's order, and this is the assertion that stops the
/// placement rule being applied everywhere.** Without it, moving every verdict
/// to the front reads as a tidy-up and breaks parity on the 13 threads of
/// Intent's own estate that carry a contract.
#[test]
fn ac_status_keeps_v2s_order_when_the_detail_is_a_tally() {
  let dir = project();
  blocked(dir.path());

  let printed = stdout(&run(dir.path(), &["ac", "status", "ST0001"]));
  assert!(
    printed.starts_with("ac: 1/3"),
    "the count leads and the verdict trails, which is `bin/intent_acceptance:937`: {printed:?}"
  );
}

// ---------------------------------------------------------------------------
// `at lint` -- a check with an output
// ---------------------------------------------------------------------------

#[test]
fn at_lint_on_a_conforming_thread_says_what_it_examined() {
  let dir = project();
  let root = dir.path();
  seed(
    root,
    "ST0001",
    &criterion("AC-01.1"),
    &[at_row("AT-01.1", "AC-01.1"), at_row("AT-01.2", "AC-01.1")].join(", "),
  );

  let out = run(root, &["at", "lint", "ST0001"]);
  assert_eq!(
    stdout(&out),
    "lint: ST0001 ok -- 2 AT row(s) conform",
    "v2's positive control (`bin/intent_acceptance:1278`). Zero bytes here is \
     indistinguishable from a lint that never ran"
  );
  assert_eq!(out.status.code(), Some(0));
}

/// **The control the single-fixture test cannot be.** Two threads, two row
/// counts, one binary: a constant satisfies at most one of them.
#[test]
fn the_row_count_is_the_rows_examined_and_not_a_constant() {
  let dir = project();
  let root = dir.path();
  seed(
    root,
    "ST0001",
    &criterion("AC-01.1"),
    &at_row("AT-01.1", "AC-01.1"),
  );
  seed(
    root,
    "ST0002",
    &criterion("AC-01.1"),
    &["AT-01.1", "AT-01.2", "AT-01.3", "AT-01.4"]
      .map(|id| at_row(id, "AC-01.1"))
      .join(", "),
  );

  assert_eq!(
    stdout(&run(root, &["at", "lint", "ST0001"])),
    "lint: ST0001 ok -- 1 AT row(s) conform"
  );
  assert_eq!(
    stdout(&run(root, &["at", "lint", "ST0002"])),
    "lint: ST0002 ok -- 4 AT row(s) conform"
  );
}

/// A thread with no AT rows at all reports `0`, and that is the reading `ok`
/// alone could never give: **43 of Intent's own 56 threads are in exactly this
/// state**, and before the denominator existed they were indistinguishable
/// from the 13 that were genuinely checked.
#[test]
fn a_thread_with_no_at_rows_says_zero_rather_than_looking_checked() {
  let dir = project();
  seed(dir.path(), "ST0001", &criterion("AC-01.1"), "");

  assert_eq!(
    stdout(&run(dir.path(), &["at", "lint", "ST0001"])),
    "lint: ST0001 ok -- 0 AT row(s) conform"
  );
}

/// **One bad row out of TWO, deliberately.** With one finding over one row the
/// two numbers are indistinguishable, so a line that printed the finding count
/// in the denominator's place would read correctly and assert green.
fn one_finding_over_two_rows(root: &Path) {
  seed(
    root,
    "ST0001",
    &criterion("AC-01.1"),
    // L4 on the first row: a `covers` id naming no criterion in the contract.
    &[at_row("AT-01.1", "AC-01.9"), at_row("AT-01.2", "AC-01.1")].join(", "),
  );
}

/// The failing path carries the verdict too -- and its stderr stays EMPTY,
/// because `Failure::Verdict` is the declared contract for this arm: the answer
/// is on stdout where machines read it.
#[test]
fn at_lint_failing_prints_the_verdict_beside_the_findings() {
  let dir = project();
  one_finding_over_two_rows(dir.path());

  let out = run(dir.path(), &["at", "lint", "ST0001"]);
  let printed = stdout(&out);
  assert!(
    printed.contains("AT-01.1 covers AC-01.9, which is not a criterion"),
    "the finding still names which rule fired: {printed:?}"
  );
  assert!(
    printed.contains("lint: ST0001 FAILED -- 1 finding(s) over 2 AT row(s)"),
    "and the verdict says how much was examined, so a reader can tell one bad \
     row out of two from one out of a hundred: {printed:?}"
  );
  assert_eq!(out.status.code(), Some(1));
  assert_eq!(
    String::from_utf8_lossy(&out.stderr),
    "",
    "`Failure::Verdict` is silent on stderr by construction -- the verdict is \
     already on stdout"
  );
}

/// **The gate's denominator was RESTORED, not added** (vc ruled, 2026-08-17).
///
/// v2 blocks with `N AT contract finding(s) over M row(s)`
/// (`bin/intent_acceptance:1009`) and v3 had dropped the `over M`, so this is a
/// regression against D17 rather than a deviation D17 has to license. **Three
/// findings out of three rows and three out of a hundred and fourteen are
/// different situations**, and the number was the only thing saying which.
///
/// The enumeration after the colon is v3's, and ruled: v2 sends the operator to
/// its lint warnings instead.
#[test]
fn the_gate_says_how_many_rows_it_examined() {
  let dir = project();
  one_finding_over_two_rows(dir.path());

  let out = run(dir.path(), &["ac", "gate", "ST0001"]);
  assert_eq!(
    stdout(&out),
    "gate: ST0001 BLOCKED -- 1 acceptance test contract finding(s) over 2 row(s): AT-01.1 covers AC-01.9, which is not a criterion in this contract",
  );
  assert_eq!(out.status.code(), Some(1));
}
