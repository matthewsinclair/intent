//! `0189`: `intent edit issue 0056 --path` printed thread ST0000's `info.md`
//! at rc=0.
//!
//! **THE CALLER NAMED AN ISSUE, THE TOOL ANSWERED ABOUT A THREAD, AND REPORTED
//! SUCCESS.** `0149`'s class one step worse: `0149` discarded the kind and
//! REFUSED about a subject nobody named, where this discarded it and SUCCEEDED.
//! Under `--editor` rather than `--path` it puts an operator in the wrong file
//! with nothing saying so.
//!
//! **THE MECHANISM WAS A NAME, NOT A MISSING ARGUMENT** (vc). `render.rs`
//! probed `arg(m, "address")` -- which is `explore`'s argument name, where this
//! verb's is `id` -- so the probe always erred and always fell through to
//! `thread_arg`, the THREAD parser. The declared `address-or-id` type was
//! already on the right argument; the code asked for a different verb's.
//!
//! **IT SURVIVED BECAUSE THE FALLBACK IS RIGHT BY ACCIDENT ON EVERY COMMON
//! PATH**, which is why no test caught it and why the arms below are shaped the
//! way they are. `edit st ST0000` and `edit st 56` both work THROUGH the
//! defect. And because `THREAD_DIGITS == ISSUE_DIGITS == 4`, an issue spelling
//! produces a WELL-FORMED thread id rather than an error -- so the wrong answer
//! is visible only when an issue number is also a thread number, **which is 48
//! of 69 on this estate** (vc). Common and silent, not rare and loud.

use std::path::Path;
use std::process::Command;

fn bin() -> std::path::PathBuf {
  std::path::PathBuf::from(env!("CARGO_BIN_EXE_intent"))
}

/// A project with one thread and one issue **whose numbers collide** -- which
/// is the only configuration in which the defect is observable at all.
fn seeded() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  run(dir.path(), &["init", "kinds"]);
  run(dir.path(), &["st", "new", "a thread"]);
  run(dir.path(), &["issues", "add", "an issue"]);
  dir
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

#[test]
fn naming_an_issue_does_not_answer_about_the_thread_of_the_same_number() {
  let dir = seeded();
  let (thread, rc) = run(dir.path(), &["edit", "st", "0001", "--path"]);
  assert_eq!(rc, 0, "the thread case must keep working: {thread}");
  assert!(
    thread.contains("ST0001"),
    "the fixture's thread did not resolve, so the comparison below proves nothing: {thread}"
  );

  let (issue, rc) = run(dir.path(), &["edit", "issue", "0001", "--path"]);
  // **THE ASSERTION IS THAT IT IS NOT THE THREAD'S ANSWER**, not that it is any
  // particular refusal. An issue has no realised form, so refusing is correct
  // here -- but the DEFECT was answering about something else, and that is what
  // must not come back whatever the refusal later says.
  assert!(
    !issue.contains("ST0001/info.md"),
    "naming `issue` returned the THREAD's file -- 0189, the wrong subject at rc={rc}: {issue}"
  );
}

#[test]
fn the_kind_vocabulary_the_table_declares_is_enforced() {
  // `intent edit banana ST0000 --path` printed a path at rc=0: the enum was
  // declared and nothing honoured it, the same gap `--format` carried until
  // `07ad9876`.
  let dir = seeded();
  let (out, rc) = run(dir.path(), &["edit", "banana", "0001", "--path"]);
  assert_ne!(rc, 0, "an undeclared kind was accepted: {out}");
  assert!(
    out.contains("st") && out.contains("issue"),
    "the refusal must name the vocabulary it is enforcing: {out}"
  );
}

#[test]
fn the_declared_address_or_id_type_accepts_an_address() {
  // The row's own note: "`address::promote` stays the one door". `thread_arg`
  // was not that door -- it splits on `/` and reads `intent:` as a thread id.
  //
  // **THE AUTHORITY IS EMPTY, SO THE CANONICAL FORM CARRIES THREE SLASHES.**
  // Measured 2026-08-31: the two-slash spelling appears ten times across this
  // estate's boards and the canonical one ONCE, because nothing emits it and
  // four parties each reconstructed the same wrong form from intuition.
  let dir = seeded();
  let (out, rc) = run(
    dir.path(),
    &["edit", "st", "intent:///threads/ST0001", "--path"],
  );
  assert_eq!(rc, 0, "the canonical address form was refused: {out}");
  assert!(
    out.contains("ST0001"),
    "the address resolved to the wrong thread: {out}"
  );
}

#[test]
fn a_kind_that_contradicts_the_address_is_refused_rather_than_resolved() {
  // Naming two entities in one invocation and picking either would be the
  // wrong-subject-silently shape this whole file exists to close.
  let dir = seeded();
  let (out, rc) = run(
    dir.path(),
    &["edit", "st", "intent:///issues/0001", "--path"],
  );
  assert_ne!(
    rc, 0,
    "a contradiction between kind and address was resolved rather than refused: {out}"
  );
  assert!(
    out.contains("issue"),
    "the refusal must name what the address actually carries: {out}"
  );
}

/// **hv's RULED SHAPE, 2026-08-31: `<KIND>` IS OPTIONAL WHEN THE FIRST
/// POSITIONAL IS AN ADDRESS.**
///
/// `ST0064` `AC-01.5` has the menubar app hand a URL to the resolver and open
/// what comes back. Demanding a kind beside it would make the app parse the
/// kind OUT of the URL in Swift, which is the second resolver that row forbids
/// -- so the kind-free door is the criterion's requirement rather than a
/// convenience.
///
/// **vc recommended an `--address` OPTION and hv chose hand-parsed positionals,
/// having read the cost stated.** The trade is recorded in `render.rs` beside
/// the parse; what is pinned HERE is that all three forms answer, because the
/// risk of a hand-parsed shape is the next argument added silently breaking one
/// branch while the others stay green.
#[test]
fn all_three_ruled_spellings_reach_the_same_artefact() {
  let dir = seeded();
  let mut answers = Vec::new();
  for argv in [
    ["edit", "intent:///threads/ST0001", "--path"].as_slice(),
    ["edit", "st", "ST0001", "--path"].as_slice(),
    ["edit", "st", "1", "--path"].as_slice(),
  ] {
    let (out, rc) = run(dir.path(), argv);
    assert_eq!(rc, 0, "{argv:?} was refused: {out}");
    answers.push(out.trim().to_string());
  }
  assert!(
    answers.windows(2).all(|w| w[0] == w[1]),
    "the three ruled spellings must name ONE artefact, and they named {answers:?}"
  );
}

/// **THE FILE ARGUMENT MOVES UP A SLOT UNDER THE ADDRESS FORM, AND THAT IS THE
/// PART A HAND-PARSED SHAPE GETS WRONG.** clap puts it in `id` when the kind is
/// absent, so both branches have to thread it and only a drive can tell.
///
/// **THE DISCRIMINATOR IS THAT THE ARGUMENT CHANGES THE ANSWER, AND FINDING ONE
/// TOOK TWO WRONG FIXTURES.** A fresh thread carries `info.md` and
/// `acceptance.md`; `design` is not there at all, and `acceptance` is a
/// GENERATED view this verb refuses to open. `info` is the DEFAULT, so passing
/// it proves nothing -- an arm that ignored the argument entirely would pass.
/// **So the file is asserted by its EFFECT: naming `acceptance` must change the
/// outcome, and must change it the same way down both branches.**
#[test]
fn the_file_argument_survives_both_branches() {
  let dir = seeded();
  let (default_file, rc) = run(dir.path(), &["edit", "st", "ST0001", "--path"]);
  assert_eq!(rc, 0, "the default file must resolve: {default_file}");

  let (with_kind, _) = run(
    dir.path(),
    &["edit", "st", "ST0001", "acceptance", "--path"],
  );
  let (with_address, _) = run(
    dir.path(),
    &["edit", "intent:///threads/ST0001", "acceptance", "--path"],
  );

  assert_ne!(
    with_kind.trim(),
    default_file.trim(),
    "naming a file changed nothing, so this arm cannot tell a read argument from an ignored one"
  );
  assert_eq!(
    with_kind.trim(),
    with_address.trim(),
    "the file argument reached one branch and not the other -- which is exactly the failure a \
     hand-parsed positional shape invites"
  );
  assert!(
    with_kind.contains("acceptance"),
    "neither branch named the file it was given: {with_kind}"
  );
}

/// **THE TWO EMPTY SHAPES REFUSE, AND THEY NAME DIFFERENT THINGS.** Both
/// positionals are `0..1`, so clap accepts the bare verb and a lone kind; the
/// spine's own comment records that a surface accepting an invented verb is a
/// No Silent Errors failure rather than a gap.
#[test]
fn the_shapes_clap_can_no_longer_refuse_are_refused_here() {
  let dir = seeded();
  let (bare, rc) = run(dir.path(), &["edit", "--path"]);
  assert_ne!(rc, 0, "the bare verb was accepted: {bare}");
  assert!(
    bare.contains("intent:///") && bare.contains("st"),
    "the refusal must name BOTH working shapes, not the one it prefers: {bare}"
  );

  let (kind_only, rc) = run(dir.path(), &["edit", "st", "--path"]);
  assert_ne!(
    rc, 0,
    "a kind with nothing to open was accepted: {kind_only}"
  );
}

/// **A LONE BARE ID MEETS THE LADDER'S SEAT, NOT THE ENUM CHECK.** `intent edit
/// 0056` is the natural thing to type. Routing it through the kind vocabulary
/// answers *`0056` is not a kind*, which is true and useless; the refusal that
/// helps names both entities the number could mean -- which is the question
/// hv's resolution ladder will answer and, until it exists, the one thing this
/// verb must not guess at.
#[test]
fn a_bare_id_is_told_what_it_is_ambiguous_between() {
  let dir = seeded();
  let (out, rc) = run(dir.path(), &["edit", "0001", "--path"]);
  assert_ne!(rc, 0, "a bare id was resolved without a tie-breaker: {out}");
  assert!(
    out.contains("thread") && out.contains("issue"),
    "the refusal must name what the number is ambiguous BETWEEN: {out}"
  );
}
