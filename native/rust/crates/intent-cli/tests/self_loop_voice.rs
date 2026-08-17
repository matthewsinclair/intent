//! **Every mutating verb reports a no-op, and it names the STATE** -- issue 0050.
//!
//! Nineteen render arms called the facade as a statement --
//! `open()?.st_done(&id).map_err(fail)?;` -- which propagates the error and
//! DISCARDS the `Ok` value, then printed the movement message unconditionally. So
//! `intent st done` on a completed thread printed `ok: ST0001 done` having done
//! nothing, at exit 0, while `intent todo done` -- which DELEGATES to that same
//! `st_done` -- reported the no-op. The wrapper was honest and the thing it
//! wrapped was not.
//!
//! **THIS FILE EXISTS BECAUSE THE FIX WAS INVISIBLE TO ITS OWN SUITE.** The
//! workspace was 485 passed / 0 failed before the nineteen arms changed and 485
//! passed / 0 failed after: no test asserted the old spelling and none asserted
//! the new one. A sweep whose green is identical either side of it has not been
//! measured, it has been performed -- and a later author reverting any single arm
//! would have found the suite agreeing with them.
//!
//! **The verbs are driven TWICE, and repetition is the point rather than a
//! shortcut.** Idempotence is the property hv's ruling actually bought -- "a
//! caller can rely on the state without checking it first" -- so calling a verb
//! again IS the property under test, and it needs no per-state fixture to reach.
//! It also catches the exact defect directly: an arm that prints its movement
//! message unconditionally produces two IDENTICAL lines, and this file refuses
//! that.
//!
//! **The state is read back from the tool, never written as a literal here.**
//! Seventeen hard-coded state words would be seventeen spellings a rename could
//! not reach, which is issue 0047 rebuilt in a new file. So each case asks
//! `intent st show` / `wp show` what the entity's state is and requires the no-op
//! line to name that.

use std::path::Path;
use std::process::{Command, Output};

use intentsvcs::model::{TShirt, enum_str};

fn project() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let config = dir.path().join("intent").join(".config");
  std::fs::create_dir_all(&config).expect("mkdir");
  std::fs::write(
    config.join("config.json"),
    "{\n  \"intent_version\": \"3.0.0\",\n  \"project_name\": \"Loop\",\n  \"author\": \"cc\",\n  \"intent_dir\": \"intent\",\n  \"languages\": [\"rust\"]\n}\n",
  )
  .expect("write config");
  dir
}

/// A thread with one work package, one authored criterion and one acceptance
/// test -- enough to reach every family's self-loop arms.
///
/// `wip` rather than `triage`, because most lifecycle verbs are declared from
/// somewhere downstream of triage and a fixture at the entry state would make
/// half these cases refusals instead of movements.
fn seed(root: &Path) {
  let dir = root.join("intent/st/ST0001");
  std::fs::create_dir_all(&dir).expect("mkdir");
  std::fs::write(
    dir.join("thread.json"),
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
      "state": { "is": "unsatisfied" } }
  ],
  "tests": [
    { "id": "AT-01.1", "covers": ["AC-01.1"], "kind": "test", "status": "to-write" }
  ]
}
"#,
  )
  .expect("write canon");
  std::fs::create_dir_all(root.join("intent/issues")).expect("mkdir issues");
  std::fs::write(
    root.join("intent/issues/0001.json"),
    "{\"schema\":\"intent/issue@3.0\",\"number\":1,\"slug\":\"a-thing\",\"title\":\"A thing\",\"status\":\"open\",\"created\":\"2026-08-01\"}\n",
  )
  .expect("write issue");
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

/// Run a verb, requiring it to succeed, and hand back its single line.
fn line(root: &Path, args: &[&str]) -> String {
  let out = run(root, args);
  assert_eq!(
    out.status.code(),
    Some(0),
    "`intent {}` must succeed:\n{}",
    args.join(" "),
    String::from_utf8_lossy(&out.stderr)
  );
  stdout(&out).trim().to_string()
}

/// **The whole property, applied to one verb.**
///
/// `subject` is what the line names. `state` is a READER, called after the first
/// invocation -- not a value.
///
/// **It has to be a reader, and getting that wrong is instructive.** Passing
/// `&thread_state(root)` evaluates it as an argument, ie BEFORE the verb runs, so
/// every case compared the no-op line against the state the entity was in
/// beforehand. Eight of nine failed and each failure printed the correct no-op
/// line beside a stale expectation -- **a green would have meant the verb had not
/// moved anything.** The instrument was measuring the wrong instant, which is the
/// same shape as measuring the wrong tree or the wrong corpus.
fn twice(root: &Path, args: &[&str], subject: &str, state: impl Fn(&Path) -> String) {
  let first = line(root, args);
  let state = state(root);
  let second = line(root, args);

  assert_ne!(
    first,
    second,
    "`intent {}` printed the SAME line twice, so the second call reported a movement it did not make -- this is issue 0050's defect, in this arm",
    args.join(" ")
  );
  assert_eq!(
    second,
    format!("ok: {subject} already {state}"),
    "the no-op line names the STATE the entity is in, in the house form (issue 0050). `intent {}` said: {second}",
    args.join(" ")
  );
}

/// What `intent st show` reports as the thread's status -- the tool's own
/// spelling, so a rename of the display vocabulary reds this rather than sailing
/// past a literal.
fn thread_state(root: &Path) -> String {
  field(&stdout(&run(root, &["st", "show", "ST0001"])), "status")
}

fn wp_state(root: &Path) -> String {
  field(&stdout(&run(root, &["wp", "show", "ST0001/01"])), "status")
}

/// The work package's SIZE, which is a different field of the same entity from
/// [`wp_state`] -- and `wp rescope` is the only verb here whose no-op names a
/// field other than a status.
fn wp_scope(root: &Path) -> String {
  field(&stdout(&run(root, &["wp", "show", "ST0001/01"])), "scope")
}

/// Satisfy the fixture's one criterion, so the close gate passes.
///
/// **A precondition, not a subject.** `st done` and `wp done` declare
/// `Guard::GatePass`, so without this they refuse before any self-loop question
/// is reached -- and the refusal would look like this file failing rather than
/// like a fixture that was never closeable.
fn satisfy(root: &Path) {
  line(
    root,
    &[
      "ac",
      "satisfy",
      "ST0001",
      "AC-01.1",
      "--evidence",
      "checked",
    ],
  );
}

fn field(text: &str, key: &str) -> String {
  text
    .lines()
    .find_map(|l| l.strip_prefix(&format!("{key}:")))
    .unwrap_or_else(|| panic!("no `{key}:` line in:\n{text}"))
    .trim()
    .to_string()
}

// ---------------------------------------------------------------------------
// Steel thread
// ---------------------------------------------------------------------------

/// **`st done` is the arm issue 0050 names first**, and it is the one where the
/// false report costs most: `ok: ST0001 done` is the confirmation of a state
/// change, and against an already-closed thread it is a false report at exit 0.
#[test]
fn st_done_reports_the_second_call_as_a_no_op() {
  let dir = project();
  seed(dir.path());
  // The gate is a DECLARED guard on `st.done`, so it has to pass before the verb
  // can move at all -- and it must NOT be consulted on the second call, which is
  // the load-bearing half of hv's ruling. If it were, this would exit 1.
  satisfy(dir.path());
  twice(
    dir.path(),
    &["st", "done", "ST0001"],
    "ST0001",
    thread_state,
  );
}

/// **`st hold` is the case that killed `was already <verb>`.**
///
/// The state is `On Hold` and the verb is `hold`, so the participle spelling
/// would have to invent "was already held" -- a phrase naming neither the verb
/// nor the state. This case is why the house form names the state.
#[test]
fn st_hold_reports_the_state_and_not_the_verb() {
  let dir = project();
  seed(dir.path());
  let args = ["st", "hold", "ST0001", "--reason", "waiting on hv"];
  twice(dir.path(), &args, "ST0001", thread_state);
  assert_eq!(
    thread_state(dir.path()),
    "On Hold",
    "and the state the line names is the display vocabulary's, not a word derived from the verb"
  );
}

/// `st start` from `wip` -- a self-loop that hv's ruling made legal, and which
/// the 0046 class deliberately does NOT cover from `done`.
#[test]
fn st_start_on_a_wip_thread_is_a_no_op() {
  let dir = project();
  seed(dir.path());
  // **Back to `not-started` first, because the fixture is already `wip` and
  // `twice` requires the FIRST call to move.** Driven at `wip` both calls are
  // no-ops and print the same line -- correct behaviour that trips the
  // did-it-move assertion, which is the assertion earning its keep rather than
  // misfiring: a case where every call is a no-op cannot show that a no-op reads
  // differently from a movement.
  line(dir.path(), &["st", "hold", "ST0001", "--reason", "parked"]);
  line(dir.path(), &["st", "resume", "ST0001"]);
  line(dir.path(), &["wp", "unstart", "ST0001/01"]);
  twice(
    dir.path(),
    &["wp", "start", "ST0001/01"],
    "ST0001/01",
    wp_state,
  );
}

// ---------------------------------------------------------------------------
// Work package
// ---------------------------------------------------------------------------

/// `wp done` on an already-done package, which is the arm whose gate must NOT be
/// re-run -- the load-bearing half of hv's ruling. If the gate ran again here the
/// second call would refuse rather than report, and `line` would fail on the exit
/// code before the message was ever compared.
#[test]
fn wp_done_reports_the_second_call_as_a_no_op() {
  let dir = project();
  seed(dir.path());
  satisfy(dir.path());
  twice(
    dir.path(),
    &["wp", "done", "ST0001/01"],
    "ST0001/01",
    wp_state,
  );
}

/// `wp unstart` -- and the subject here is the `ST0001/01` form rather than a
/// bare id, which is the one arm family where the no-op line's subject is
/// composed rather than passed through.
#[test]
fn wp_unstart_reports_the_second_call_as_a_no_op() {
  let dir = project();
  seed(dir.path());
  twice(
    dir.path(),
    &["wp", "unstart", "ST0001/01"],
    "ST0001/01",
    wp_state,
  );
}

/// **`wp rescope` -- the no-op that names a size rather than a status**, and the
/// first arm born into the corrected voice instead of swept into it.
///
/// Wired on 2026-08-17 (issue 0052) after the other twenty-one had been fixed,
/// deliberately in that order: an arm added to a renderer where twenty-one
/// siblings printed a movement they did not make would have been written to match
/// them.
///
/// **Driven to `L` from a fixture at `S`, because `twice` requires the first call
/// to MOVE** -- and the moving call is what shows the reader is being asked at the
/// right instant. Driven at `S` both calls are no-ops, both lines are correct, and
/// the file would prove nothing.
#[test]
fn wp_rescope_reports_the_second_call_as_a_no_op() {
  let dir = project();
  seed(dir.path());
  twice(
    dir.path(),
    &["wp", "rescope", "ST0001/01", "L"],
    "ST0001/01",
    wp_scope,
  );
}

/// **The size is one of six values and the sixth spelling is refused by NAME.**
///
/// `spine.rs` builds no `value_parser` from a positional's `values`, so the six on
/// the row are documentation and this layer is the only enforcement there is
/// (`arg_values_note`). Two arms, because "it refuses" and "it says what to type
/// instead" are different properties and only the second one ends the operator's
/// guessing.
///
/// **`Small` is the interesting refusal.** It is a size v2's free-text `scope:`
/// field really does contain, and `legacy.rs` still reads it at ingest -- so this
/// asserts the boundary rather than an accident: the set an operator may type and
/// the set v2 may have written are different questions, and only the first one is
/// this positional's.
#[test]
fn wp_rescope_refuses_a_size_outside_the_six_and_names_them() {
  let dir = project();
  seed(dir.path());
  for bad in ["Medium", "Small", "huge", ""] {
    let out = run(dir.path(), &["wp", "rescope", "ST0001/01", bad]);
    assert_eq!(
      out.status.code(),
      Some(1),
      "`{bad}` is not one of the six, so it must be refused rather than resolved to a nearby size"
    );
    let text = String::from_utf8_lossy(&out.stderr).to_string();
    // **The expectation is DERIVED from the enum, not spelled here.** A literal
    // `"XS, S, M, L, XL, XXL"` in this file would be a seventh copy of the
    // vocabulary -- the exact thing the helper's doc comment argues against --
    // and it would pass while the remedy went stale in a different way. What
    // this asserts is the property: the refusal names the permitted set.
    let permitted: Vec<String> = TShirt::ALL.iter().map(enum_str).collect();
    assert!(
      text.contains(&permitted.join(", ")),
      "the refusal must name the permitted set -- a bare parse error blames the spelling without \
       saying what the spellings are. Got: {text}"
    );
  }
  assert_eq!(
    wp_scope(dir.path()),
    "S",
    "and a refused rescope changed nothing"
  );
}

// ---------------------------------------------------------------------------
// Acceptance criteria and tests
// ---------------------------------------------------------------------------

/// **`ac unsatisfy` is the arm where the movement phrase must NOT survive into
/// the no-op**, and it is the only one with that shape.
///
/// Its movement line is `ok: AC-01.1 unsatisfied (evidence cleared)`. Nothing was
/// cleared if nothing was satisfied, so a no-op repeating that parenthetical
/// would report a clearing that did not happen. `reported` composes the no-op
/// from the state instead, so the parenthetical is structurally unable to leak --
/// and this asserts it.
#[test]
fn ac_unsatisfy_does_not_carry_its_parenthetical_into_the_no_op() {
  let dir = project();
  seed(dir.path());
  satisfy(dir.path());
  let first = line(dir.path(), &["ac", "unsatisfy", "ST0001", "AC-01.1"]);
  let second = line(dir.path(), &["ac", "unsatisfy", "ST0001", "AC-01.1"]);
  assert_eq!(first, "ok: AC-01.1 unsatisfied (evidence cleared)");
  assert_eq!(second, "ok: AC-01.1 already unsatisfied");
  assert!(
    !second.contains("evidence cleared"),
    "a no-op cleared no evidence, so it must not say it did: {second}"
  );
}

/// `at set` on a row already at that status.
///
/// **This arm was writing a real envelope for a non-movement before the ruling**
/// -- `at.set` is declared with an empty from-set, so `from` refuses nothing and
/// `at set green` on a green row recorded a second transition at a second time
/// for one event (D42). The no-op line is the visible half of that being fixed.
#[test]
fn at_set_to_the_current_status_is_a_no_op() {
  let dir = project();
  seed(dir.path());
  // **`at red`, not `at set`** -- the facade verb is `at_set` and the SURFACE
  // spells it as three verbs (`green` / `red` / `na`), which is v2's shape. A test
  // written from the facade's names rather than the table's does not reach the
  // binary at all: this said `at set` and got `unrecognized subcommand`.
  twice(
    dir.path(),
    &["at", "red", "ST0001", "AT-01.1"],
    "AT-01.1",
    |_| "red".to_string(),
  );
}

/// **THE TWO ARMS ISSUE 0050's ENUMERATION MISSED.**
///
/// 0050 counted nineteen dropped sites by scanning for `open()?.<verb>(..)` on one
/// line. `ac descope` and `ac withdraw` break that call across lines, so a
/// line-oriented scan cannot see them -- **the real count was twenty-one**, and
/// both printed their movement message on a no-op at exit 0 after the other
/// nineteen were fixed.
///
/// **They were found by driving every self-loop-capable verb twice, not by
/// re-reading the enumeration** -- which is the only method that could have found
/// them, since the enumeration is the thing that was wrong. A list that has gone
/// short cannot be checked against itself.
///
/// `descope`'s movement phrase carries its target (`descoped to ST0001`) and the
/// no-op must not, because the state's own name is `descoped` and repeating the
/// target would assert a move to a thread nothing moved to.
#[test]
fn the_two_arms_a_line_oriented_count_could_not_see() {
  let dir = project();
  seed(dir.path());

  let first = line(
    dir.path(),
    &[
      "ac",
      "withdraw",
      "ST0001",
      "AC-01.1",
      "--reason",
      "not wanted",
    ],
  );
  let second = line(
    dir.path(),
    &[
      "ac",
      "withdraw",
      "ST0001",
      "AC-01.1",
      "--reason",
      "not wanted",
    ],
  );
  assert_eq!(first, "ok: AC-01.1 withdrawn");
  assert_eq!(second, "ok: AC-01.1 already withdrawn");

  line(dir.path(), &["ac", "reinstate", "ST0001", "AC-01.1"]);
  let first = line(
    dir.path(),
    &["ac", "descope", "ST0001", "AC-01.1", "--to", "ST0001"],
  );
  let second = line(
    dir.path(),
    &["ac", "descope", "ST0001", "AC-01.1", "--to", "ST0001"],
  );
  assert_eq!(first, "ok: AC-01.1 descoped to ST0001");
  assert_eq!(
    second, "ok: AC-01.1 already descoped",
    "the no-op names the STATE, so it drops the target the movement line carries"
  );
}

/// **THE THIRD AND FOURTH INSTANCES OF 0051'S MECHANISM, AND THEY WERE TWENTY
/// LINES BELOW THE COPY I FIXED** -- issue 0053, found by ic driving the binary
/// twice rather than by reading my announcement.
///
/// `ac_rescope` and `ac_reinstate` each raised `NotOffScope` from a `_` arm placed
/// ahead of `set_ac_state`, so the shared self-loop test was unreachable for both.
/// `intent ac rescope` on an in-scope criterion exited 1 where hv's ruling makes
/// it a self-loop at 0.
///
/// **My own report is what let them survive.** I reasoned from
/// `AcState::entry(kind)` and published `already unsatisfied | already computed`
/// -- exactly right about the value the SUCCESS arm hands the setter, and the
/// self-loop never reached that arm. The subject I reasoned about and the subject
/// I reported came apart, and the reasoning was well-formed either way.
///
/// **The refusal each verb still gives now names the verb the caller typed.**
/// `NotOffScope` hardcoded `reinstate` in its message and its remedy, so typing
/// `rescope` was answered with advice about a different command, twice. v2 gets
/// this right, so it was a regression rather than a gap.
#[test]
fn ac_rescope_and_ac_reinstate_accept_a_self_loop_and_name_themselves_when_they_refuse() {
  let dir = project();
  seed(dir.path());

  // AC-01.1 is non-test and unsatisfied, which IS the entry state both verbs
  // target -- so both are self-loops from the fixture, with no setup at all.
  // That is the measure of how reachable this was: two verbs, exit 1, on the
  // fixture's own initial state.
  for verb in ["rescope", "reinstate"] {
    let out = run(dir.path(), &["ac", verb, "ST0001", "AC-01.1"]);
    assert_eq!(
      out.status.code(),
      Some(0),
      "`intent ac {verb}` on an in-scope criterion already at the entry state is a self-loop:\n{}",
      String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(stdout(&out).trim(), "ok: AC-01.1 already unsatisfied");
  }

  // And from a state neither verb is declared from, each still refuses -- naming
  // ITSELF, and naming the one state it undoes rather than the union of both.
  line(
    dir.path(),
    &[
      "ac",
      "satisfy",
      "ST0001",
      "AC-01.1",
      "--evidence",
      "checked",
    ],
  );
  for (verb, wanted, other) in [
    ("rescope", "descoped", "reinstate"),
    ("reinstate", "withdrawn", "rescope"),
  ] {
    let out = run(dir.path(), &["ac", verb, "ST0001", "AC-01.1"]);
    assert_eq!(
      out.status.code(),
      Some(1),
      "a satisfied criterion is in scope"
    );
    let text = String::from_utf8_lossy(&out.stderr).to_string();
    assert!(
      text.contains(&format!("nothing to {verb}")),
      "the refusal names the verb typed: {text}"
    );
    assert!(
      text.contains(&format!("applies only to a {wanted} criterion")),
      "and the remedy names the state THIS verb undoes: {text}"
    );
    assert!(
      !text.contains(other),
      "`{other}` must not appear -- naming it is what sent the reader to the wrong command: {text}"
    );
  }
}

/// **`ac unsatisfy` on an already-unsatisfied criterion is ACCEPTED, and it
/// refused at exit 1 until 2026-08-17.**
///
/// `ac_unsatisfy` carried its own from-state check ahead of delegating, so the
/// shared setter's self-loop test was unreachable for this verb -- 0051's
/// mechanism, in a second live instance. hv's ruling makes this a self-loop at 0.
///
/// **And the refusal it used to give is still given where it belongs**: the
/// message survives, mapped from the declared machine's refusal rather than
/// hand-written ahead of it.
#[test]
fn ac_unsatisfy_accepts_a_self_loop_and_still_refuses_a_computed_criterion() {
  let dir = project();
  seed(dir.path());
  let out = run(dir.path(), &["ac", "unsatisfy", "ST0001", "AC-01.1"]);
  assert_eq!(
    out.status.code(),
    Some(0),
    "an already-unsatisfied criterion is a self-loop, not a refusal:\n{}",
    String::from_utf8_lossy(&out.stderr)
  );
  assert_eq!(stdout(&out).trim(), "ok: AC-01.1 already unsatisfied");
}

// ---------------------------------------------------------------------------
// Issues -- MACHINE 4, and the family the house form comes from
// ---------------------------------------------------------------------------

/// **v2's spelling, reached through the general helper.**
///
/// `issues close` passes `-> CLOSED` as its movement phrase, so `reported`
/// composes v2's two lines exactly: `ok: issue 0001 -> CLOSED` and `ok: issue
/// 0001 already CLOSED`. This is the pair hv's self-loop ruling cites, so it is
/// also the case that would tell us the general helper had drifted from the one
/// spelling parity actually requires.
#[test]
fn issues_close_composes_v2s_two_lines_through_the_shared_helper() {
  let dir = project();
  seed(dir.path());
  let first = line(dir.path(), &["issues", "close", "1"]);
  let second = line(dir.path(), &["issues", "close", "1"]);
  assert_eq!(first, "ok: issue 0001 -> CLOSED");
  assert_eq!(second, "ok: issue 0001 already CLOSED");
}

// ---------------------------------------------------------------------------
// The delegation issue 0050 was actually filed on
// ---------------------------------------------------------------------------

/// **`todo done` and `st done` must now AGREE, and their disagreement is what
/// made 0050 a defect rather than a style preference.**
///
/// `todo done` was one of two arms that read the outcome, and what it delegates
/// to -- `st_done` -- was one of the nineteen that did not. So the same facade
/// call reached two ways gave two different answers, and the caller reaching for
/// the direct verb got LESS information than the one reaching through the
/// convenience command.
///
/// Asserted as equality between the two commands rather than against a literal:
/// that is the property, and a literal would let both drift together.
#[test]
fn todo_done_and_st_done_report_a_no_op_identically() {
  let dir = project();
  seed(dir.path());
  satisfy(dir.path());
  line(dir.path(), &["st", "done", "ST0001"]);

  let direct = line(dir.path(), &["st", "done", "ST0001"]);
  let through_todo = line(dir.path(), &["todo", "done", "ST0001"]);
  assert_eq!(
    direct, through_todo,
    "one facade call reached two ways must give one answer -- the wrapper being more honest than the thing it wraps is issue 0050"
  );
  assert_eq!(
    direct,
    format!("ok: ST0001 already {}", thread_state(dir.path()))
  );
}
