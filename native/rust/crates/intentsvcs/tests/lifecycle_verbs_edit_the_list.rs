//! **AT-05.2 / AC-05.2 -- THE LIFECYCLE VERBS EDIT `.intentfiles`.**
//!
//! hv's architecture in one line: **realisation is driven from `.intentfiles`;
//! commands change `.intentfiles`; `organize` realises it.** This file drives
//! the middle clause. `st new` adds the id and `--dehydrate` does not; `st
//! done` and `st cancel` remove it and `--keep` does not; `st reopen` and
//! `st reinstate` add it back.
//!
//! The two primitives are covered next door -- `pin_writes_to_the_list.rs` and
//! `unpin_removes_from_the_list.rs`. **A green there is a green about a
//! function; this file is the one that is a green about the row.**
//!
//! # The strongest test here is the one about the verbs that do NOTHING
//!
//! [`no_verb_that_only_changes_status_touches_the_list`] is the reason the
//! implementation keys on the OP STRING and never on the target status, and it
//! is arithmetic rather than principle. Eight ops funnel through
//! `set_thread_status` and the destination does not identify them: **`st
//! triage` and `st reinstate` both land on `NotStarted`, and `st start`, `st
//! resume` and `st reopen` all land on `Wip`.** So an implementation that
//! matched on the status it was setting -- the obvious one, and the one that
//! reads as equivalent -- would make `st triage` start listing threads and
//! `st start` silently re-add one a human had removed by hand. Two collisions
//! in a vocabulary of eight.
//!
//! That defect is invisible from the happy-path tests: every assertion about
//! `st done` and `st reopen` below passes under both implementations, because
//! `Completed` and `Cancelled` are reached by exactly one op each.
//!
//! # What this file does NOT cover
//!
//! **The CLI half.** `--dehydrate` and `--keep` reach clap with correct help
//! and are read by nothing until `render.rs` is wired, which is held behind a
//! peer's in-flight edit to that file. The facade doors those flags will call
//! are [`Facade::st_new_listing`] and [`Facade::st_done_listing`], and they are
//! driven here.
//!
//! **THE `UnsyncedAttachments` ARM IS NOT DRIVEN AND I AM SAYING SO RATHER
//! THAN LETTING THE FILE IMPLY IT.** AC-05.2 requires a closing verb to NAME
//! THE PATHS when the artefact holds on-disk bytes no commit contains.
//! Reaching that arm needs a fixture that is a real git repository with a real
//! index, and `common::Fixture` is a bare temp directory. What is driven below
//! is the `None` arm -- the question could not be asked -- plus the suppression
//! rules and the independence of the note from the edit. **So the warning's
//! WIRING is proven and its PAYLOAD is not**, and a green here must not be read
//! as the second.

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::facade::{ListEdit, Note};
use intentsvcs::intentfiles::realised_from;
use intentsvcs::model::ThreadStatus;

/// A manifest with a NEIGHBOUR in it, so every case has a control.
///
/// Every assertion below that a thread was added or removed is paired with one
/// that `ST0099` was not -- otherwise a verb that rewrote the whole file would
/// pass every test in this file.
const MANIFEST: &str = "\
# a hand-maintained note that must survive every verb
# BEGIN INTENT
STEELTHREAD:ST0056
STEELTHREAD:ST0099
# END INTENT
";

fn fixture() -> Fixture {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  fx.write_file("intent/.intentfiles", MANIFEST);
  fx
}

/// What the manifest declares realised, read from disk.
///
/// Driven through [`realised_from`] rather than by grepping the text, because
/// the consumer's question is "is this thread realised" and a substring check
/// can pass while the answer to that question is wrong -- a line inside a
/// comment being the obvious way.
fn declared(fx: &Fixture) -> intentsvcs::intentfiles::Realised {
  realised_from(&fx.read("intent/.intentfiles"))
}

fn manifest_text(fx: &Fixture) -> String {
  fx.read("intent/.intentfiles")
}

// ---------------------------------------------------------------------------
// CREATION
// ---------------------------------------------------------------------------

#[test]
fn st_new_adds_the_entry() {
  let fx = fixture();
  let mut facade = fx.facade();
  let id = facade
    .st_new("a thread that should be realised")
    .expect("new");

  assert!(
    declared(&fx).declares(&id),
    "`st new` must list what it created, or the next `organize` removes the files it \
     just wrote:\n{}",
    manifest_text(&fx)
  );
  assert!(
    declared(&fx).declares("ST0099"),
    "and the neighbour is untouched -- without this a verb that rewrote the file \
     would pass"
  );
}

#[test]
fn st_new_dehydrate_does_not() {
  let fx = fixture();
  let mut facade = fx.facade();
  let before = manifest_text(&fx);
  let id = facade
    .st_new_listing("a thread nobody wants on disk", ListEdit::Suppressed)
    .expect("new");

  assert!(
    !declared(&fx).declares(&id),
    "`--dehydrate` must not list it:\n{}",
    manifest_text(&fx)
  );
  assert_eq!(
    manifest_text(&fx),
    before,
    "and the file is byte-identical -- a suppressed edit writes nothing at all, so \
     nothing reflows and no mtime moves"
  );
}

// ---------------------------------------------------------------------------
// CLOSING
// ---------------------------------------------------------------------------

#[test]
fn st_done_removes_the_entry() {
  let fx = fixture();
  let mut facade = fx.facade();
  assert!(
    declared(&fx).declares("ST0056"),
    "precondition: ST0056 starts listed, or this test cannot show a removal"
  );

  facade.st_done("ST0056").expect("done");
  assert!(
    !declared(&fx).declares("ST0056"),
    "`st done` must remove the entry:\n{}",
    manifest_text(&fx)
  );
  assert!(declared(&fx).declares("ST0099"), "neighbour untouched");
}

#[test]
fn st_done_keep_leaves_it() {
  let fx = fixture();
  let mut facade = fx.facade();
  let before = manifest_text(&fx);

  facade
    .st_done_listing("ST0056", ListEdit::Suppressed)
    .expect("done");
  assert_eq!(
    facade.st_show("ST0056").unwrap().status,
    ThreadStatus::Completed,
    "`--keep` suppresses the LIST EDIT and never the close itself -- a flag that \
     quietly declined to close the thread would be a far worse defect than the one \
     it prevents"
  );
  assert_eq!(
    manifest_text(&fx),
    before,
    "and the manifest is byte-identical:\n{}",
    manifest_text(&fx)
  );
}

#[test]
fn st_cancel_removes_the_entry() {
  let fx = fixture();
  let mut facade = fx.facade();
  facade
    .st_cancel("ST0056", "overtaken by events")
    .expect("cancel");

  assert!(
    !declared(&fx).declares("ST0056"),
    "`st cancel` removes it too:\n{}",
    manifest_text(&fx)
  );
  assert!(declared(&fx).declares("ST0099"), "neighbour untouched");
}

/// **`st cancel --keep` LEAVES IT, EXACTLY AS `st done --keep` DOES.**
///
/// hv ruled the asymmetry out on 2026-08-20. AC-05.2 named only `st done
/// --keep`, and the surface shipped with the override on one of two identical
/// acts -- **which is a surface that has to be memorised rather than
/// understood.** Driven beside the `st done` case rather than instead of it,
/// because the property being asserted is that the two verbs AGREE, and a
/// single test cannot show agreement.
#[test]
fn cancel_keep_leaves_it_too() {
  let fx = fixture();
  let mut facade = fx.facade();
  let before = manifest_text(&fx);

  facade
    .st_cancel_listing(
      "ST0056",
      "overtaken, but I still need the notes",
      ListEdit::Suppressed,
    )
    .expect("cancel");

  assert_eq!(
    facade.st_show("ST0056").unwrap().status,
    ThreadStatus::Cancelled,
    "`--keep` suppresses the LIST EDIT and never the cancellation itself"
  );
  assert_eq!(
    manifest_text(&fx),
    before,
    "and the manifest is byte-identical:\n{}",
    manifest_text(&fx)
  );
}

/// **THE TWO CLOSING VERBS AGREE, AND THAT IS THE ASSERTION hv's RULING ASKS
/// FOR.** Not that each behaves correctly on its own -- the tests above do that
/// -- but that they cannot drift apart, which is what an override present on
/// one of them and absent on the other had already done once.
#[test]
fn both_closing_verbs_treat_keep_the_same_way() {
  for suppressed in [ListEdit::AsDeclared, ListEdit::Suppressed] {
    let done = {
      let fx = fixture();
      fx.facade()
        .st_done_listing("ST0056", suppressed)
        .expect("done");
      declared(&fx).declares("ST0056")
    };
    let cancelled = {
      let fx = fixture();
      fx.facade()
        .st_cancel_listing("ST0056", "overtaken", suppressed)
        .expect("cancel");
      declared(&fx).declares("ST0056")
    };
    assert_eq!(
      done, cancelled,
      "`st done` and `st cancel` disagree about {suppressed:?}: done left it listed = {done}, \
       cancel left it listed = {cancelled}. They are the same act with different words and must \
       not diverge."
    );
  }
}

// ---------------------------------------------------------------------------
// COMING BACK
// ---------------------------------------------------------------------------

#[test]
fn st_reopen_adds_it_back() {
  let fx = fixture();
  let mut facade = fx.facade();
  facade.st_done("ST0056").expect("done");
  assert!(
    !declared(&fx).declares("ST0056"),
    "precondition: the close removed it"
  );

  facade
    .st_reopen("ST0056", "the contract grew after it closed")
    .expect("reopen");
  assert!(
    declared(&fx).declares("ST0056"),
    "`st reopen` must put it back, or a reopened thread has no files to work in:\n{}",
    manifest_text(&fx)
  );
}

#[test]
fn st_reinstate_adds_it_back() {
  let fx = fixture();
  let mut facade = fx.facade();
  facade.st_cancel("ST0056", "overtaken").expect("cancel");
  assert!(!declared(&fx).declares("ST0056"), "precondition");

  facade
    .st_reinstate("ST0056", "not overtaken after all")
    .expect("reinstate");
  assert!(
    declared(&fx).declares("ST0056"),
    "`st reinstate` must put it back:\n{}",
    manifest_text(&fx)
  );
}

// ---------------------------------------------------------------------------
// THE CONTROL, AND IT IS THE POINT OF THE FILE
// ---------------------------------------------------------------------------

/// **NOTHING DERIVES THE LIST FROM STATUS, AND THIS IS WHERE THAT IS MEASURED.**
///
/// The four ops below move a thread between states and say nothing about
/// whether it is on disk. **A held thread stays realised** -- that is the whole
/// content of hv's rule, and it is the clause an implementation keyed on the
/// destination status cannot honour, because `st triage` shares `NotStarted`
/// with `st reinstate` and `st start` and `st resume` share `Wip` with
/// `st reopen`.
#[test]
fn no_verb_that_only_changes_status_touches_the_list() {
  // Each op gets its own fixture, so an earlier verb cannot leave the manifest
  // in a state that masks a later one. Written out rather than driven from a
  // table of closures: two cases do not earn the indirection, and the boxed-
  // closure version cost a `clippy::type_complexity` to say less.
  let fx = fixture();
  let before = manifest_text(&fx);
  fx.facade()
    .st_hold("ST0056", "waiting on a ruling")
    .expect("hold");
  assert_eq!(
    manifest_text(&fx),
    before,
    "`st hold` changes what the thread IS and must not touch the list -- **a held \
     thread stays realised**. If this fails, the edit is keyed on the destination \
     status rather than on the op:\n{}",
    manifest_text(&fx)
  );

  let fx = fixture();
  let before = manifest_text(&fx);
  let mut facade = fx.facade();
  facade.st_hold("ST0056", "waiting").expect("hold");
  facade.st_resume("ST0056").expect("resume");
  assert_eq!(
    manifest_text(&fx),
    before,
    "`st resume` shares `Wip` with `st reopen`, which DOES add -- so a status-keyed \
     edit re-adds a thread a human had deliberately removed:\n{}",
    manifest_text(&fx)
  );
}

/// The other two of the four, which need a thread that is not already `wip`.
#[test]
fn triage_and_start_do_not_touch_the_list_either() {
  let fx = Fixture::new();
  let mut thread = sample_thread("ST0056");
  thread.status = ThreadStatus::Triage;
  fx.write_thread(&thread);
  fx.write_file("intent/.intentfiles", MANIFEST);

  let before = manifest_text(&fx);
  let mut facade = fx.facade();
  facade.st_triage("ST0056").expect("triage");
  assert_eq!(
    manifest_text(&fx),
    before,
    "`st triage` shares its destination -- `NotStarted` -- with `st reinstate`, which \
     DOES add. If this line fails, that is the collision:\n{}",
    manifest_text(&fx)
  );

  facade.st_start("ST0056").expect("start");
  assert_eq!(
    manifest_text(&fx),
    before,
    "`st start` shares `Wip` with `st reopen`, which DOES add:\n{}",
    manifest_text(&fx)
  );
}

// ---------------------------------------------------------------------------
// ABSENT IS NOT EMPTY
// ---------------------------------------------------------------------------

/// **NO LIFECYCLE VERB CREATES `.intentfiles`, AND THE REASON IS THE WHOLE
/// ESTATE RATHER THAN THIS THREAD.**
///
/// hv ruled that a missing manifest means nobody has said and everything is
/// realised, while a manifest present and declaring nothing means keep nothing.
/// So a verb that helpfully created one to hold its single entry would declare
/// that entry **the whole of what is realised**, and the next `organize` would
/// remove every other thread's files on the strength of one `st new`.
///
/// **The no-op is the rule applying, not a case being skipped** -- which is why
/// it is asserted for every verb rather than for the one that happened to be
/// convenient.
#[test]
fn no_verb_creates_a_manifest_that_was_not_there() {
  let path_rel = "intent/.intentfiles";

  // `st new` -- the one most likely to be implemented as "create and list".
  {
    let fx = Fixture::new();
    fx.write_thread(&sample_thread("ST0056"));
    let mut facade = fx.facade();
    facade
      .st_new("a thread in a project that never listed anything")
      .expect("new");
    assert!(
      !fx.path(path_rel).exists(),
      "`st new` created a manifest -- every other thread in that estate is now \
       unrealised and `organize` will remove their files"
    );
  }

  // `st done` -- nothing to remove, and creating a file to record that would be
  // the same catastrophe in the other direction.
  {
    let fx = Fixture::new();
    fx.write_thread(&sample_thread("ST0056"));
    let mut facade = fx.facade();
    facade.st_done("ST0056").expect("done");
    assert!(!fx.path(path_rel).exists(), "`st done` created a manifest");
  }

  // `st reopen`, which ADDS -- the verb with the strongest excuse to create one.
  {
    let fx = Fixture::new();
    fx.write_thread(&sample_thread("ST0056"));
    let mut facade = fx.facade();
    facade.st_done("ST0056").expect("done");
    facade
      .st_reopen("ST0056", "the contract grew")
      .expect("reopen");
    assert!(
      !fx.path(path_rel).exists(),
      "`st reopen` created a manifest to hold the id it wanted to add"
    );
  }
}

// ---------------------------------------------------------------------------
// THE SELF-LOOP
// ---------------------------------------------------------------------------

/// **A CLOSING VERB ON AN ALREADY-CLOSED THREAD LEAVES A HUMAN'S EDIT ALONE.**
///
/// `.intentfiles` is durable state a human may edit by hand, so re-listing a
/// completed thread is a legitimate act -- it is how you get a closed thread's
/// files back without reopening it. `set_thread_status` returns `AlreadyThere`
/// before touching anything, so the second `st done` cannot undo that decision.
///
/// **The early return is doing two jobs and only one of them was designed.**
/// It exists so a self-loop cannot re-run the close gate against a criterion
/// added after the close. That it also protects the manifest is a consequence
/// worth pinning down, because an implementation that moved the list edit ahead
/// of the self-loop test would keep every other test in this file green.
#[test]
fn closing_an_already_closed_thread_does_not_undo_a_hand_edit() {
  let fx = fixture();
  let mut facade = fx.facade();
  facade.st_done("ST0056").expect("done");
  assert!(!declared(&fx).declares("ST0056"), "precondition: removed");

  // The human puts it back by hand -- the supported way to work in a closed
  // thread's files without reopening it.
  fx.write_file(
    "intent/.intentfiles",
    &format!("{MANIFEST}STEELTHREAD:ST0056  # I want these files back\n"),
  );
  assert!(
    declared(&fx).declares("ST0056"),
    "precondition: hand-listed"
  );

  let outcome = facade
    .st_done("ST0056")
    .expect("a self-loop is not an error");
  assert!(
    outcome.already().is_some(),
    "precondition: the second close is a self-loop, got {outcome:?}"
  );
  assert!(
    declared(&fx).declares("ST0056"),
    "the second `st done` removed an entry a human deliberately added:\n{}",
    manifest_text(&fx)
  );
}

// ---------------------------------------------------------------------------
// THE CLOSING WARNING
// ---------------------------------------------------------------------------

/// **A CLOSING VERB CARRIES A NOTE AND NEVER A REFUSAL.**
///
/// The fixture is a bare temp directory with no git repository, so
/// `sync_uncommitted` answers `None` -- the question could not be asked -- and
/// the close reports that rather than passing over it. **Silence would be read
/// as "no uncommitted bytes"** by anyone who knows the verb warns, so it is the
/// clean bill of health `sync_uncommitted`'s own contract refuses to let a
/// caller print.
#[test]
fn a_close_reports_what_it_could_not_ask_and_still_closes() {
  let fx = fixture();
  let mut facade = fx.facade();
  let outcome = facade.st_done("ST0056").expect("done");

  assert!(
    outcome.moved(),
    "a note is not a refusal and not a no-op: {outcome:?}"
  );
  assert_eq!(
    facade.st_show("ST0056").unwrap().status,
    ThreadStatus::Completed,
    "and the thread is closed -- the whole ruling is that this WARNS rather than \
     gating, because `organize` holds the only line that removes an estate file"
  );
  assert_eq!(
    outcome.notes(),
    &[Note::UnsyncedUnknown],
    "no repository means the question could not be asked, and that is not an empty answer"
  );
}

/// **THE NOTE IS TIED TO THE REMOVAL, NOT TO THE VERB.**
///
/// `--keep` closes the thread and leaves it listed, so no dehydration is
/// coming and there is nothing to warn about. A warning keyed on the verb
/// would fire here -- correct-looking, and the way an operator learns to skim
/// the warnings that mean something.
#[test]
fn keep_closes_without_a_note_because_nothing_is_being_dehydrated() {
  let fx = fixture();
  let mut facade = fx.facade();
  let outcome = facade
    .st_done_listing("ST0056", ListEdit::Suppressed)
    .expect("done");

  assert!(outcome.notes().is_empty(), "got {outcome:?}");
  assert_eq!(
    facade.st_show("ST0056").unwrap().status,
    ThreadStatus::Completed
  );
}

/// A verb that does not remove says nothing, on the same ground.
#[test]
fn a_non_closing_verb_carries_no_note() {
  let fx = fixture();
  let mut facade = fx.facade();
  let outcome = facade
    .st_hold("ST0056", "waiting on a ruling")
    .expect("hold");
  assert!(outcome.notes().is_empty(), "got {outcome:?}");
}

/// **A THREAD WITH NO ATTACHMENTS IS NOT UNCERTAIN, AND THAT IS ARITHMETIC.**
///
/// The uncertainty reported above is git's. With no paths to ask about there is
/// no question for git to fail to answer: zero attachments hold zero
/// uncommitted bytes, in a repository or out of one. **This is the one case
/// where the answer does not depend on the check**, so reporting it as unknown
/// would be uncertainty theatre -- and it would fire on every close in every
/// non-git project, which is how a warning becomes wallpaper.
#[test]
fn a_thread_with_no_attachments_is_not_reported_as_unknown() {
  let fx = Fixture::new();
  let mut thread = sample_thread("ST0056");
  thread.attachments.clear();
  fx.write_thread(&thread);
  fx.write_file("intent/.intentfiles", MANIFEST);

  let mut facade = fx.facade();
  let outcome = facade.st_done("ST0056").expect("done");
  assert!(
    outcome.notes().is_empty(),
    "an attachment-less thread has nothing for git to be asked about: {outcome:?}"
  );
  assert!(
    !declared(&fx).declares("ST0056"),
    "and it still leaves the list -- the note and the edit are independent"
  );
}
