//! ST0057 **AT-11.6** -- covering **AC-11.6**: `intent organize --default
//! --force`, answered `y` on a terminal, is the destructive arm and the only
//! one. It regenerates the declaration from status and then APPLIES it in the
//! same run.
//!
//! hv's word on the criterion, first-hand and unchanged: `--default` never
//! removes a file _"unless it is used with --force, which does remove files,
//! after a confirm"_.
//!
//! # This file exists because the arm was unreachable, not merely untested
//!
//! **THE SUBJECT IS GATED ON `IsTerminal`, SO NO ORDINARY TEST CAN REACH IT.**
//! `organize_default_declaration.rs` says so in its own words -- _"AC-11.2's
//! `--force` answered `y` ON A TTY is not covered. It needs a pseudo-terminal,
//! which this estate has no harness for"_ -- and that sentence sat there while
//! `--default --force` became the one spelling in this binary that removes
//! files. The harness below is the smaller half of this change; the arm it
//! reaches is the point.
//!
//! **AND IT IS SELF-CONTROLLING, WHICH IS WHY THERE IS NO SEPARATE HARNESS
//! TEST.** A pty that failed to present a terminal does not make these tests
//! pass quietly -- the binary takes the no-tty branch, refuses, writes nothing,
//! and every assertion below reds naming the refusal. The failure direction of
//! a broken harness here is loud, so a control asserting "the terminal is a
//! terminal" would be a control that cannot go red.
//!
//! # What discriminates the two candidate implementations
//!
//! The implementation this file was written against applies; the one it
//! replaced regenerated the declaration and stopped. **They agree on
//! everything except the tree**: same prompt, same `regenerated from status`
//! line, same exit code, same declaration on disk. Only the FILES differ, so
//! every assertion that matters here is about a file appearing or disappearing.
//!
//! `ST0003` is the sharpest of them. It is WIP and NOT realised, so a correct
//! run CREATES its files -- and creation is ungated, which the removal is not.
//! On an estate whose preconditions are unmet the removal is held back and a
//! test resting on removal alone passes under an implementation that applies
//! nothing at all. That trap is not hypothetical: it is the finding recorded in
//! `organize_default_declaration.rs`'s own mutation table, one criterion over.
//!
//! # Mutations, measured -- every assertion has been SEEN to fail by name
//!
//! | mutation                                                    | reds                                                      |
//! | ------------------------------------------------------------ | ---------------------------------------------------------- |
//! | the apply becomes a `Preview`, so `--force` only writes      | `regenerates_...` AND `a_refused_removal_...`              |
//! | the confirm is printed but its answer is not required        | `answering_no_...` ONLY                                    |
//! | the refusal drops the thread names it now carries            | `a_refused_removal_names_the_thread_and_the_precondition`  |
//! | the confirm text is restored to `it removes no files`        | `the_confirm_says_what_the_arm_actually_does` ONLY         |
//!
//! **THE FIRST ROW REDS TWO TESTS AND THAT IS THE INTERESTING PART.** An arm
//! that regenerates and applies nothing fails the open-gate estate (nothing
//! created, nothing removed) AND the shut-gate one -- because the refusal test
//! asserts the ungated HYDRATION happened, which is what tells a gate that
//! refused from a run that never ran. Without that clause the shut-gate test
//! would have stayed green under an implementation that does nothing at all,
//! since "the files are still there" is exactly what doing nothing produces.
//!
//! **AND THE HARNESS THAT MEASURED THIS REPORTED FOUR EMPTY ROWS FIRST.** The
//! reader picked test names out of `cargo test` output with a filter that also
//! matched the `test result: FAILED.` summary line, so every mutation came back
//! naming a test called `result:`. It is the same class as everything else in
//! this file: the instrument returned a confident value about the wrong thing,
//! and only a row that was obviously nonsense gave it away. A row that had come
//! back plausibly wrong would have been written down.

use std::io::Write as _;
mod common;

use std::path::Path;
use std::process::{Command, Output, Stdio};

// ---------------------------------------------------------------------------
// THE HARNESS
// ---------------------------------------------------------------------------

/// Run the binary with a terminal on stdin, having already typed `answer`.
///
/// **THE ANSWER IS WRITTEN BEFORE THE CHILD IS SPAWNED, AND THAT IS NOT A
/// SHORTCUT.** The terminal's line discipline buffers it, so the child finds it
/// waiting when it reads. Writing it afterwards would need a reader thread on
/// stdout to avoid deadlocking against the prompt, and a thread is a second
/// thing that can go wrong in a harness whose whole job is to be trustworthy.
fn intent_on_a_tty(dir: &Path, args: &[&str], answer: &str) -> Output {
  let (mut master, slave) = common::pty_pair();
  master
    .write_all(answer.as_bytes())
    .expect("type the answer into the terminal");
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(dir)
    .stdin(Stdio::from(slave))
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .output()
    .expect("run the v3 binary against a terminal");
  drop(master);
  out
}

fn intent(dir: &Path, args: &[&str]) -> Output {
  Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(dir)
    .stdin(Stdio::null())
    .output()
    .expect("run the v3 binary")
}

fn ok(dir: &Path, args: &[&str]) -> String {
  let out = intent(dir, args);
  assert!(
    out.status.success(),
    "`intent {}` must succeed -- stdout: {}\nstderr: {}",
    args.join(" "),
    String::from_utf8_lossy(&out.stdout),
    String::from_utf8_lossy(&out.stderr)
  );
  String::from_utf8_lossy(&out.stdout).into_owned()
}

fn stdout(out: &Output) -> String {
  String::from_utf8_lossy(&out.stdout).into_owned()
}

fn stderr(out: &Output) -> String {
  String::from_utf8_lossy(&out.stderr).into_owned()
}

/// Every file under `intent/st`, project-relative, in order.
///
/// The store is excluded: it is per-machine truth, it changes on every command,
/// and this verb's subject is the realised form on disk.
fn realised(root: &Path) -> Vec<String> {
  fn walk(dir: &Path, root: &Path, out: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
      return;
    };
    for e in entries.flatten() {
      let p = e.path();
      if p.is_dir() {
        walk(&p, root, out);
      } else {
        out.push(p.strip_prefix(root).unwrap_or(&p).display().to_string());
      }
    }
  }
  let mut out = Vec::new();
  walk(&root.join("intent/st"), root, &mut out);
  out.sort();
  out
}

/// The ids the manifest declares, in file order.
fn declared(root: &Path) -> Vec<String> {
  std::fs::read_to_string(root.join("intent/.intentfiles"))
    .expect("the manifest must be readable")
    .lines()
    .filter_map(|l| l.strip_prefix("STEELTHREAD:"))
    .map(|id| id.split('#').next().unwrap_or(id).trim().to_string())
    .collect()
}

/// Three threads chosen so that a correct run must both CREATE and REMOVE.
///
/// - `ST0001` carries the estate's precondition declaration and is WIP, so it
///   is declared and already realised: the unchanged control.
/// - `ST0002` is realised on disk and stays at `triage`, so the regenerated
///   declaration does not name it: the removal.
/// - `ST0003` is WIP and NOT realised, so the regenerated declaration names it
///   and nothing on disk answers to it yet: the creation.
///
/// **THE DECLARATION IS SINGLE, WHICH THE GATE REQUIRES.** Exactly one
/// criterion in the estate may carry a `<<PRECONDITIONS ... PRECONDITIONS>>`
/// block; two carriers anywhere refuse, and the refusal is not the one under
/// test here.
///
/// `satisfied` is the whole difference between this fixture's two callers: with
/// the precondition met the removal proceeds, and with it unmet the removal is
/// held. **Both are AC-11.6 arms**, so the fixture takes it as a parameter
/// rather than existing twice.
fn project(satisfied: bool) -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  ok(root, &["init", "Fixture"]);
  ok(root, &["st", "new", "Gate declaration"]);
  ok(root, &["st", "new", "To be removed"]);
  ok(root, &["st", "new", "To be created"]);
  // **REALISED EXPLICITLY, BECAUSE `st new` NO LONGER DECLARES WHAT IT
  // CREATES** (hv, 2026-08-27). A fixture that inherited its realised state
  // from an unrelated verb would stop saying what it depends on the day that
  // verb was ruled on -- which is exactly what happened to this estate's other
  // fixtures that morning.
  ok(root, &["st", "hydrate", "ST0001"]);
  ok(root, &["st", "hydrate", "ST0002"]);
  ok(
    root,
    &[
      "ac",
      "new",
      "ST0001",
      "AC-00.1",
      "--text",
      "No dehydration path removes any file while any declared precondition is \
       unmet. <<PRECONDITIONS AC-00.9 PRECONDITIONS>>",
    ],
  );
  ok(
    root,
    &["ac", "new", "ST0001", "AC-00.9", "--text", "a precondition"],
  );
  if satisfied {
    ok(
      root,
      &[
        "ac",
        "satisfy",
        "ST0001",
        "AC-00.9",
        "--evidence",
        "met by construction in this fixture",
      ],
    );
  }
  ok(root, &["st", "start", "ST0001"]);
  ok(root, &["st", "start", "ST0003"]);
  dir
}

// ---------------------------------------------------------------------------
// THE CRITERION
// ---------------------------------------------------------------------------

/// **THE WHOLE OF AC-11.6 ON AN ESTATE WHOSE GATE IS OPEN**: regenerate, then
/// apply, in one run -- realise what is declared, remove what is not, and leave
/// a preview afterwards with nothing to say.
#[test]
fn regenerates_then_applies_in_the_same_run() {
  let dir = project(true);
  let root = dir.path();

  // Before: ST0002 is on disk and ST0003 is not. Asserted rather than assumed,
  // because every claim below is a claim about a CHANGE, and a change measured
  // from an unverified start is not measured at all.
  let before = realised(root);
  assert!(
    before.iter().any(|p| p.starts_with("intent/st/ST0002/")),
    "the fixture must start with ST0002 realised, or the removal proves nothing: {before:?}"
  );
  assert!(
    !before.iter().any(|p| p.starts_with("intent/st/ST0003/")),
    "the fixture must start with ST0003 UNrealised, or the creation proves nothing: {before:?}"
  );

  let out = intent_on_a_tty(root, &["organize", "--default", "--force"], "y\n");
  let said = format!("{}{}", stdout(&out), stderr(&out));
  assert!(
    out.status.success(),
    "the confirmed force arm must succeed on an estate whose gate is open: {said}"
  );

  // The declaration is now status-derived: the two WIP threads and nothing else.
  assert_eq!(
    declared(root),
    vec!["ST0001".to_string(), "ST0003".to_string()],
    "the regenerated manifest must declare exactly the WIP threads: {said}"
  );

  let after = realised(root);
  // **THE CREATION IS THE ASSERTION THAT SURVIVES A SHUT GATE.** See the module
  // note: removal is gated and hydration is not, so this is the arm that reds
  // under an implementation that regenerates and applies nothing.
  assert!(
    after.iter().any(|p| p == "intent/st/ST0003/info.md"),
    "the declared-but-unrealised thread must have been realised in the same run: {after:?}\n{said}"
  );
  assert!(
    !after.iter().any(|p| p.starts_with("intent/st/ST0002/")),
    "no file of the undeclared realised thread may survive the destructive arm: {after:?}\n{said}"
  );
  assert!(
    after.iter().any(|p| p == "intent/st/ST0001/info.md"),
    "the declared and already-realised thread is the unchanged control and must still be here: {after:?}"
  );

  // **AND THE CRITERION'S CLOSING CLAUSE, WHICH IS THE ONLY ONE THAT ASKS
  // WHETHER THE RUN LEFT THE ESTATE CONSISTENT.** A run that removed and
  // hydrated correctly but left the store disagreeing with the disk would pass
  // every assertion above.
  let preview = ok(root, &["organize"]);
  assert!(
    preview.contains("0 to remove,"),
    "a preview afterwards must have nothing left to remove -- and no ` (N blocked)` \
     parenthetical, which would mean the removals were refused rather than done: {preview}"
  );
  assert!(
    preview.contains("0 diverged"),
    "a preview afterwards must report nothing diverged: {preview}"
  );
}

/// **THE HELD REMOVAL KEEPS EVERY FILE OF THE THREAD, AND THE REFUSAL NAMES
/// BOTH THE THREAD AND THE PRECONDITION.**
///
/// The hydration half is asserted in the same breath deliberately: the refusal
/// is estate-wide, and an implementation that answered it by abandoning the
/// whole run would also leave ST0002 intact. **Survival alone cannot tell a
/// gate that refused from a run that never happened**, and the created files
/// are what distinguishes them.
#[test]
fn a_refused_removal_names_the_thread_and_the_precondition() {
  let dir = project(false);
  let root = dir.path();

  let out = intent_on_a_tty(root, &["organize", "--default", "--force"], "y\n");
  let said = format!("{}{}", stdout(&out), stderr(&out));
  assert!(
    !out.status.success(),
    "a run that was asked to remove and did not must not report success: {said}"
  );

  let after = realised(root);
  assert!(
    after.iter().any(|p| p == "intent/st/ST0002/info.md")
      && after.iter().any(|p| p == "intent/st/ST0002/acceptance.md"),
    "not one file of the blocked thread may be removed: {after:?}\n{said}"
  );
  assert!(
    after.iter().any(|p| p == "intent/st/ST0003/info.md"),
    "the ungated half of the run must still have happened, or this test cannot tell a \
     refusal from a run that did nothing: {after:?}\n{said}"
  );

  let err = stderr(&out);
  assert!(
    err.contains("ST0002"),
    "the refusal must name the thread whose files are held, which a file COUNT does not: {err}"
  );
  assert!(
    err.contains("AC-00.9"),
    "the refusal must name the unmet precondition: {err}"
  );
}

/// **THE CONFIRM IS WHAT MAKES THIS ARM DESTRUCTIVE, SO A NO MUST LEAVE
/// EVERYTHING.** Including the declaration -- a run that regenerated the
/// manifest and then declined to act on it would have changed the estate's mind
/// about what is realised without changing a file, which is the harder half to
/// notice.
#[test]
fn answering_no_leaves_the_declaration_and_the_tree_alone() {
  let dir = project(true);
  let root = dir.path();
  let before_files = realised(root);
  let before_declared = declared(root);

  let out = intent_on_a_tty(root, &["organize", "--default", "--force"], "n\n");
  let said = format!("{}{}", stdout(&out), stderr(&out));
  assert!(
    !out.status.success(),
    "a declined confirm must not report success: {said}"
  );
  assert_eq!(
    realised(root),
    before_files,
    "a declined confirm must leave every file: {said}"
  );
  assert_eq!(
    declared(root),
    before_declared,
    "a declined confirm must leave the declaration byte-identical: {said}"
  );
}

/// **THE PROMPT IS THE ONE PIECE OF PROSE THAT HAS TO BE TRUE AT THE MOMENT A
/// HUMAN DECIDES.**
///
/// It read `it removes no files` for as long as `--default` had no destructive
/// arm, and stayed on the screen while AC-11.6 gave it one. A stale sentence in
/// a confirm prompt is not a documentation defect -- it is the operator being
/// told the opposite of what is about to happen, in the sentence that asks for
/// permission.
///
/// Driven by answering `n`, so this test never removes anything: the prompt is
/// printed before the answer is read.
#[test]
fn the_confirm_says_what_the_arm_actually_does() {
  let dir = project(true);
  let out = intent_on_a_tty(dir.path(), &["organize", "--default", "--force"], "n\n");
  let said = stdout(&out);
  assert!(
    !said.contains("it removes no files"),
    "the prompt still promises what this arm now disproves: {said}"
  );
  assert!(
    said.contains("REMOVES"),
    "the prompt must say that it removes files, in the sentence that asks: {said}"
  );
  assert!(
    said.contains("without --force"),
    "the prompt must name the spelling that does NOT remove, or the only choice \
     offered is between proceeding and abandoning the command: {said}"
  );
}
