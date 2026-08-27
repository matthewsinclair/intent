//! **AT-05.2 / AC-05.2 -- THE LIFECYCLE VERBS EDIT `.intentfiles`.**
//!
//! hv's architecture in one line: **realisation is driven from `.intentfiles`;
//! commands change `.intentfiles`; `organize` realises it.** This file drives
//! the middle clause. **Which verb makes which edit is hv's, ruled three times
//! on 2026-08-27 and enumerated in the table below**; what this file drives is
//! that each verb makes the edit the table says, that `--keep` and
//! `--dehydrate` suppress it, and that nothing ELSE in the manifest moves when
//! they do.
//!
//! The two primitives are covered next door -- `pin_writes_to_the_list.rs` and
//! `unpin_removes_from_the_list.rs`. **A green there is a green about a
//! function; this file is the one that is a green about the row.**
//!
//! # hv's THREE RULINGS OF 2026-08-27 DISSOLVED THE ARGUMENT BELOW, AND IT IS
//! # KEPT BECAUSE THE CONCLUSION SURVIVED THE ARGUMENT
//!
//! `1d0ce157` took `st.new` out of the adding set; `dfd07cfe` put `st.start`
//! and `st.resume` in and took `st.reinstate` out; `3e5e620c` put `st.hold`
//! and `st.triage` into the removing set. **After all three, declared-iff-WIP
//! holds in BOTH directions** -- enumerated rather than asserted:
//!
//! | destination   | action | ops                                |
//! | ------------- | ------ | ---------------------------------- |
//! | `wip`         | ADD    | `st.start` `st.resume` `st.reopen` |
//! | `completed`   | REMOVE | `st.done`                          |
//! | `cancelled`   | REMOVE | `st.cancel`                        |
//! | `hold`        | REMOVE | `st.hold`                          |
//! | `not-started` | REMOVE | `st.triage`                        |
//! | `not-started` | none   | `st.reinstate`                     |
//!
//! **AND THE LAST TWO ROWS ARE WHY THE IMPLEMENTATION IS STILL KEYED ON THE
//! OP.** After `dfd07cfe` every destination mapped to exactly one action, so
//! op-keying was still true but no longer FORCED, and this doc said so.
//! `3e5e620c` put a second op on `not-started` taking a DIFFERENT action, so
//! the collision is load-bearing again -- in exactly one place, and that place
//! is driven by
//! [`reinstate_touches_nothing_which_is_the_one_case_a_status_keyed_table_gets_wrong`].
//! hv has not ruled on the FORM, and a status-keyed rewrite would be an
//! unruled change wearing a refactor.
//!
//! **THE COST hv WAS TOLD, RECORDED HERE BECAUSE IT IS OPERATOR-VISIBLE:** a
//! thread put on hold LEAVES the manifest and `st resume` re-adds it. The round
//! trip works; what changes is that the entries visibly vanish while the thread
//! is held, and *a held thread stays realised* was the entire content of the
//! old design.
//!
//! **THE REMOVAL CARRIES THE UNSYNCED-ATTACHMENTS WARNING WITH IT**, because
//! that note is tied to the removal and not to the verb (`closing_notes` keys
//! on `declared_list_edit(op) == Remove`). So `st hold` now warns where it was
//! silent. That is not a side effect of this edit -- it is the documented
//! design reaching a new member of its class -- and it is driven below rather
//! than left to be discovered.
//!
//! # The argument this file was built on, now down to one case
//!
//! Kept rather than deleted, because a corrected sentence reads exactly like
//! one that was never wrong. It read: *eight ops funnel through
//! `set_thread_status` and the destination does not identify them -- `st
//! triage` and `st reinstate` both land on `NotStarted`, and `st start`, `st
//! resume` and `st reopen` all land on `Wip` -- so a status-keyed edit would
//! make `st triage` start listing threads and `st start` silently re-add a
//! thread a human had removed by hand.* Two collisions in a vocabulary of
//! eight.
//!
//! **Both hazards are now REAL AND ACCEPTED, which is a different outcome from
//! being wrong.** `st start` does re-add a hand-dehydrated thread; `st triage`
//! does now edit the list, removing rather than adding. What survives of the
//! argument is the `not-started` row-pair above.
//!
//! That defect was invisible from the happy-path tests: every assertion about
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
//! # The payload arm IS driven now, and a fixture was the whole of what was
//! # missing
//!
//! This file used to declare `UnsyncedAttachments` undriven, and the reason
//! was never a judgement -- it was that reaching the arm needs a real
//! repository with a real index and `common::Fixture` was a bare temp
//! directory. **[`Fixture::git_init`] closes that**, so AC-05.2's second limb
//! -- *WARN, NAMING THE PATHS, when the artefact holds on-disk bytes the store
//! has never seen* -- is now driven at its PAYLOAD and not only at its wiring.
//!
//! **The three answers `sync_uncommitted` can give are asserted to be three.**
//! `None` (no repository, so the question could not be asked), `Some([])` (a
//! clean index, so there is nothing to warn about) and `Some([..])` (paths, by
//! name) each have their own test, because the first two collapse into "no
//! note" under any implementation that reads an unanswered question as a clean
//! bill of health -- **the exact folding `sync_uncommitted` returns an
//! `Option` to prevent.** Driving only two of the three cannot tell them apart.
//!
//! **And the fixture commits BOTH attachments and then disturbs ONE.** An
//! implementation that named every attachment of a closing thread -- the
//! plausible wrong one, since canon has the list sitting right there -- passes
//! any test that merely asks whether a path was named. It names two here.

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

/// **INVERTED ON hv's RULING OF 2026-08-27 16:30Z** (hv's board `1d0ce157`,
/// first-hand in vc's session, chosen from options vc authored -- the CHOICE
/// hv's, the FRAMING vc's): *`st new` stops declaring the Triage thread it just
/// made*, because the realised set is WIP alone and a thread is created at
/// `triage`.
///
/// **THE OLD ARM'S JUSTIFICATION WAS MEASURED FALSE RATHER THAN OVERRULED, AND
/// THAT IS WHY THIS IS SAFE.** It read *`st new` must list what it created, or
/// the next `organize` removes the files it just wrote*. `st new` writes no
/// thread directory -- only the `steel_threads.md` index -- so there are no
/// files to remove. Driven on a throwaway estate after the change: `organize`
/// reports `0 to remove`, and `organize --apply` leaves the file count
/// unchanged at 10. Quoted rather than deleted, because a hazard that never
/// existed reads exactly like one that was closed.
#[test]
fn st_new_does_not_declare_the_thread_it_creates() {
  let fx = fixture();
  let mut facade = fx.facade();
  let id = facade
    .st_new("a thread that is created at triage")
    .expect("new");

  assert!(
    !declared(&fx).declares(&id),
    "`st new` declared {id}, which is at `triage` -- the realised set is WIP alone:\n{}",
    manifest_text(&fx)
  );
  assert!(
    declared(&fx).declares("ST0099"),
    "and the neighbour is untouched -- without this a verb that rewrote the file \
     would pass"
  );
}

/// **THIS ARM NOW HOLDS VACUOUSLY AND IS KEPT ONLY FOR ITS SECOND
/// ASSERTION.** Since hv's ruling, plain `st new` does not list either, so
/// `--dehydrate` and its absence produce the same manifest -- **a flag whose
/// two branches became identical.** The `declares` assertion below can no
/// longer fail for the reason it was written for, and a test that cannot
/// distinguish its two cases is the degenerate pass this suite exists to
/// catch. It is NOT deleted, because the byte-identity assertion still says
/// something no other arm does: a suppressed edit writes nothing at all.
///
/// **REPORTED RATHER THAN FIXED HERE: `st new --dehydrate` is now inert, and
/// `flag_reachability` cannot see it** -- that gate asks whether a declared
/// flag is READ, and this one is read; it is the two branches that stopped
/// differing. Whether the flag should be withdrawn is not a test's call.
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

/// **INVERTED ON hv's RULING OF 2026-08-27 16:43Z** (hv's board `dfd07cfe`).
/// `st.reinstate` lands on `not-started`, and the realised set is WIP alone, so
/// it no longer declares. The superseded arm read *`st reinstate` must put it
/// back*. **hv was told this cost before choosing it and nobody had asked for
/// it** -- it is the price of making declared-iff-WIP a property of the
/// mechanism rather than of four sites happening to agree.
///
/// **THIS COVERS THE ADD SIDE ONLY, AND THAT IS NOT A GAP -- IT IS WHY THE
/// TEST BELOW EXISTS.** `st cancel` has already removed the id here, so an
/// implementation that REMOVED on reinstate passes this line unchanged. The
/// distinguishing case needs a cancelled thread that is still declared, which
/// is `st cancel --keep`:
/// [`reinstate_touches_nothing_which_is_the_one_case_a_status_keyed_table_gets_wrong`].
#[test]
fn st_reinstate_does_not_add_it_back_because_it_lands_on_not_started() {
  let fx = fixture();
  let mut facade = fx.facade();
  facade.st_cancel("ST0056", "overtaken").expect("cancel");
  assert!(!declared(&fx).declares("ST0056"), "precondition");

  facade
    .st_reinstate("ST0056", "not overtaken after all")
    .expect("reinstate");
  assert!(
    !declared(&fx).declares("ST0056"),
    "`st reinstate` declared a thread that lands on `not-started` -- the realised set is WIP \
     alone:\n{}",
    manifest_text(&fx)
  );
}

// ---------------------------------------------------------------------------
// THE CONTROL, AND IT IS THE POINT OF THE FILE
// ---------------------------------------------------------------------------

/// **A THREAD LEAVES THE REALISED SET THE MOMENT IT LEAVES `wip`, AND `st hold`
/// IS WHERE THAT COSTS SOMETHING.**
///
/// **INVERTED ON hv's RULING OF 2026-08-27 17:10Z** (hv's board `3e5e620c`,
/// first-hand in vc's session, chosen from options vc authored). The
/// superseded assertion was this file's strongest line -- *`st hold` changes
/// what the thread IS and must not touch the list; a held thread stays
/// realised* -- and that sentence was the entire content of the old design. hv
/// was told so in those terms and chose the other side.
///
/// So the round trip is the test: hold removes, resume adds back, and what an
/// operator sees in between is their entries gone from the manifest while the
/// thread is held.
#[test]
fn hold_removes_and_resume_adds_it_back() {
  let fx = fixture();
  let mut facade = fx.facade();
  facade
    .st_hold("ST0056", "waiting on a ruling")
    .expect("hold");
  assert!(
    !declared(&fx).declares("ST0056"),
    "`st hold` moves the thread off `wip`, so it must leave the realised set:\n{}",
    manifest_text(&fx)
  );
  assert!(
    declared(&fx).declares("ST0099"),
    "the neighbour is the control -- a verb that rewrote the whole file passes the line \
     above and fails here:\n{}",
    manifest_text(&fx)
  );
  assert!(
    manifest_text(&fx).contains("# a hand-maintained note"),
    "removal rewrites the file, so the hand-maintained comment is the thing most likely to \
     be lost by it:\n{}",
    manifest_text(&fx)
  );

  facade.st_resume("ST0056").expect("resume");
  assert!(
    declared(&fx).declares("ST0056"),
    "`st resume` lands on `wip` and must declare, or `st hold` is a one-way door out of the \
     realised set:\n{}",
    manifest_text(&fx)
  );
}

/// The other half of the same ruling, on a thread that is not already `wip`.
///
/// **`st triage` REMOVES on hv's ruling of 17:10Z**, and `st start` puts it
/// back on the ruling of 16:43Z -- so the pair is a round trip on the other
/// side of the vocabulary, and the same one-way-door question gets the same
/// answer.
#[test]
fn triage_removes_and_start_adds_it_back() {
  let fx = Fixture::new();
  let mut thread = sample_thread("ST0056");
  thread.status = ThreadStatus::Triage;
  fx.write_thread(&thread);
  fx.write_file("intent/.intentfiles", MANIFEST);

  let mut facade = fx.facade();
  facade.st_triage("ST0056").expect("triage");
  assert!(
    !declared(&fx).declares("ST0056"),
    "`st triage` lands on `not-started`, which is off `wip`, so it must remove:\n{}",
    manifest_text(&fx)
  );
  assert!(
    declared(&fx).declares("ST0099"),
    "the neighbour is the control:\n{}",
    manifest_text(&fx)
  );

  // **The arm that closes the workflow gap `st new` opened:** `st new` then
  // `st start` leaves the thread WIP and DECLARED by the normal path, with no
  // `st hydrate` needed.
  facade.st_start("ST0056").expect("start");
  assert!(
    declared(&fx).declares("ST0056"),
    "`st start` lands on `wip` and must declare, or a thread you have just started work on is \
     not realised by any normal path:\n{}",
    manifest_text(&fx)
  );
}

/// **THE ONE CASE THAT STILL FORCES THE TABLE TO BE KEYED ON THE OP.**
///
/// `st.triage` and `st.reinstate` both land on `not-started` and they take
/// DIFFERENT actions -- triage removes, reinstate does nothing -- so a
/// status-keyed rewrite cannot express both. Every other destination is now
/// reached by ops that agree.
///
/// **DRIVEN AS A MUTATION RATHER THAN ASSERTED**, because a test that claims to
/// discriminate and does not is worth less than no test: adding `"st.reinstate"`
/// to the removing arm -- the minimal faithful spelling of a status-keyed table
/// -- reds this test and
/// [`a_verb_that_removes_nothing_carries_no_note`], and NOTHING ELSE in the
/// file. Both reds are this same reinstate case seen twice, once in the
/// manifest and once in the note that follows the removal.
///
/// **AND THE MUTATION LEAVES
/// [`st_reinstate_does_not_add_it_back_because_it_lands_on_not_started`]
/// GREEN**, which is the control that matters: that test covers the ADD side of
/// the same op and passes under both implementations, so it is not the one
/// holding the form in place. Without the test below, a status-keyed rewrite
/// would land green.
///
/// **The setup is the only shape where the difference is observable**, and it
/// is a real one rather than a contrivance: `st cancel --keep` closes the
/// thread and deliberately leaves it declared, so a `cancelled` thread that is
/// still in the manifest is a supported state. Reinstating it must leave the
/// manifest alone; a status-keyed table would delist it on the strength of its
/// destination, silently undoing the `--keep` the operator asked for.
#[test]
fn reinstate_touches_nothing_which_is_the_one_case_a_status_keyed_table_gets_wrong() {
  let fx = fixture();
  let mut facade = fx.facade();
  facade
    .st_cancel_listing("ST0056", "superseded", ListEdit::Suppressed)
    .expect("cancel --keep");
  assert!(
    declared(&fx).declares("ST0056"),
    "the fixture is wrong, not the subject: `--keep` must leave the id declared or the \
     assertion below proves nothing:\n{}",
    manifest_text(&fx)
  );

  let before = manifest_text(&fx);
  facade
    .st_reinstate("ST0056", "the successor was withdrawn")
    .expect("reinstate");
  assert_eq!(
    manifest_text(&fx),
    before,
    "`st reinstate` is not in the table and must touch nothing. If this fails, the edit is \
     keyed on the destination status -- `not-started` -- which it shares with `st triage`, \
     and a thread kept by `--keep` has just been delisted by a verb that says nothing about \
     realisation:\n{}",
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

/// **A VERB THAT REMOVES NOTHING SAYS NOTHING, ON THE SAME GROUND** -- and
/// after hv's ruling of 17:10Z that class has two members left rather than the
/// four it had this morning, so both are driven.
///
/// **THE SUBJECT MOVED, AND WHY IT MOVED IS THE POINT.** This test used to
/// drive `st hold`, on the reasoning that hold is the obvious verb that changes
/// status without removing. `3e5e620c` made hold REMOVE, so hold now carries
/// the note -- see
/// [`hold_now_carries_the_note_because_the_note_follows_the_removal`]. A test
/// whose subject has quietly joined the other class still passes for whatever
/// its new subject happens to do, which is why this one redded rather than
/// drifting: the assertion was about the class, not the verb.
#[test]
fn a_verb_that_removes_nothing_carries_no_note() {
  // The ADD side: `st start` lands on `wip` and adds. Nothing is being
  // dehydrated, so there is nothing to warn about.
  let fx = Fixture::new();
  let mut thread = sample_thread("ST0056");
  thread.status = ThreadStatus::Triage;
  fx.write_thread(&thread);
  fx.write_file("intent/.intentfiles", MANIFEST);
  let outcome = fx.facade().st_start("ST0056").expect("start");
  assert!(outcome.notes().is_empty(), "got {outcome:?}");

  // The NONE side: `st reinstate` is not in the table at all.
  let fx = fixture();
  let mut facade = fx.facade();
  facade
    .st_cancel_listing("ST0056", "superseded", ListEdit::Suppressed)
    .expect("cancel --keep");
  let outcome = facade
    .st_reinstate("ST0056", "the successor was withdrawn")
    .expect("reinstate");
  assert!(outcome.notes().is_empty(), "got {outcome:?}");
}

/// **THE CLASS OF WARNING VERBS GAINED A MEMBER, AND IT IS ASSERTED RATHER
/// THAN LEFT TO BE DISCOVERED.**
///
/// `closing_notes` keys on `declared_list_edit(op) == Remove`, so hv's ruling
/// of 17:10Z did not only move `st hold` in the manifest -- it made `st hold`
/// warn about unsynced attachments where it was previously silent. That
/// follows correctly from *the note is tied to the removal, not to the verb*,
/// which is the two tests above; it is still an operator-visible change to a
/// verb nobody was asked about, so it gets its own green.
///
/// `UnsyncedUnknown` rather than a path list because the fixture is not a
/// repository -- the same reasoning as
/// [`a_close_reports_what_it_could_not_ask_and_still_closes`], and what is
/// being asserted here is membership of the warning class, not which warning.
#[test]
fn hold_now_carries_the_note_because_the_note_follows_the_removal() {
  let fx = fixture();
  let outcome = fx
    .facade()
    .st_hold("ST0056", "waiting on a ruling")
    .expect("hold");
  assert!(
    !outcome.notes().is_empty(),
    "`st hold` removes now, and the note follows the removal: {outcome:?}"
  );
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

// ---------------------------------------------------------------------------
// AC-05.2's SECOND LIMB: WARN, **NAMING THE PATHS**
// ---------------------------------------------------------------------------

/// A real repository holding **both** of the sample thread's attachments in a
/// commit, so a test can disturb exactly one of them.
///
/// **Committing both is what makes the assertions discriminating.** The
/// attachment list is sitting in canon, so the plausible wrong implementation
/// names all of it; with one attachment in the fixture that implementation is
/// indistinguishable from the right one.
fn committed_fixture() -> Fixture {
  let fx = Fixture::new();
  fx.git_init();
  fx.write_thread(&sample_thread("ST0056"));
  fx.write_file("intent/.intentfiles", MANIFEST);
  fx.write_prose("ST0056", "reference.md", "bytes that are in a commit\n");
  fx.write_prose("ST0056", "parity/cmd-st.md", "bytes that are in a commit\n");
  fx.git(&["add", "-A"]);
  fx.git(&["commit", "-qm", "the estate as it stands"]);
  fx
}

/// The paths a closing note names, or a panic saying what came back instead.
fn named_paths(notes: &[Note]) -> &[String] {
  match notes {
    [Note::UnsyncedAttachments(paths)] => paths,
    other => panic!("expected one UnsyncedAttachments note, got {other:?}"),
  }
}

/// **THE HEADLINE: A CLOSE NAMES THE FILE WHOSE BYTES ARE IN NO COMMIT, AND
/// NAMES ONLY THAT ONE.**
#[test]
fn a_close_names_the_attachment_whose_bytes_are_in_no_commit() {
  let fx = committed_fixture();
  fx.write_prose("ST0056", "reference.md", "bytes that are in no commit\n");

  let mut facade = fx.facade();
  let outcome = facade.st_done("ST0056").expect("done");

  assert_eq!(
    named_paths(outcome.notes()),
    ["intent/st/ST0056/reference.md: edited in the working tree and not staged"],
    "AC-05.2 names the paths AT RISK. `parity/cmd-st.md` is an attachment of \
     the same thread and is in the commit, so naming it would be telling the \
     operator to check a file that is safe -- which is how the ones that matter \
     stop being read"
  );
}

/// **THE NOTE DOES NOT GATE, AND THE TWO LIMBS ARE INDEPENDENT.** hv's ruling
/// is that `organize` holds the only line that removes an estate file, so this
/// warns and closes. A note that also suppressed the list edit would leave the
/// thread closed and still listed -- silently doing `--keep` on the operator's
/// behalf, which is a decision they did not make.
#[test]
fn the_warning_closes_the_thread_and_still_makes_the_edit() {
  let fx = committed_fixture();
  fx.write_prose("ST0056", "reference.md", "bytes that are in no commit\n");

  let mut facade = fx.facade();
  let outcome = facade.st_done("ST0056").expect("done");

  assert!(outcome.moved(), "a note is not a refusal: {outcome:?}");
  assert!(
    !named_paths(outcome.notes()).is_empty(),
    "precondition: it warned"
  );
  assert_eq!(
    facade.st_show("ST0056").unwrap().status,
    ThreadStatus::Completed
  );
  let list = declared(&fx);
  assert!(
    !list.declares("ST0056"),
    "the edit is independent of the warning -- a note that suppressed it would \
     be `--keep` chosen by the tool"
  );
  assert!(
    list.declares("ST0099"),
    "and the neighbour is untouched, or the verb rewrote the whole file"
  );
}

/// **UNTRACKED AND MODIFIED DO NOT SHARE A SENTENCE.** They are different
/// situations for the operator: one file has bytes in some earlier commit to
/// fall back on and the other has none anywhere. A single "not committed"
/// message would be true of both and useful for neither.
#[test]
fn an_untracked_attachment_is_named_and_says_so_in_its_own_words() {
  let fx = Fixture::new();
  fx.git_init();
  fx.write_thread(&sample_thread("ST0056"));
  fx.write_file("intent/.intentfiles", MANIFEST);
  fx.write_prose("ST0056", "parity/cmd-st.md", "bytes that are in a commit\n");
  fx.git(&["add", "-A"]);
  fx.git(&[
    "commit",
    "-qm",
    "one attachment committed, the other never added",
  ]);
  fx.write_prose("ST0056", "reference.md", "bytes that were never added\n");

  let mut facade = fx.facade();
  let outcome = facade.st_done("ST0056").expect("done");

  assert_eq!(
    named_paths(outcome.notes()),
    ["intent/st/ST0056/reference.md: untracked, so no commit contains it"],
    "an untracked attachment is the worse case and must not be reported in the \
     modified case's words"
  );
}

/// **A CLEAN INDEX SAYS NOTHING, AND THAT IS A DIFFERENT ANSWER FROM `UNKNOWN`.**
///
/// This is the arm nothing drove. Paired with
/// [`a_close_reports_what_it_could_not_ask_and_still_closes`] it pins the
/// three-way split: an implementation that folded "the question could not be
/// asked" into "nothing is wrong" passes THIS test and fails that one, and an
/// implementation that warned on every close passes that one and fails this.
/// **Neither test can catch that alone.**
#[test]
fn a_clean_repository_says_nothing_and_that_is_not_the_unknown_answer() {
  let fx = committed_fixture();
  let mut facade = fx.facade();
  let outcome = facade.st_done("ST0056").expect("done");

  assert!(
    outcome.notes().is_empty(),
    "every attachment of this thread is in a commit, so there is nothing at \
     risk and nothing to say: {outcome:?}"
  );
}

/// **`--keep` STAYS SILENT EVEN WITH REAL UNCOMMITTED BYTES ON DISK, AND THIS
/// IS THE SUPPRESSION TEST THAT COULD FAIL.**
///
/// [`keep_closes_without_a_note_because_nothing_is_being_dehydrated`] runs
/// outside a repository, where the note would have been `UnsyncedUnknown`
/// anyway -- so it cannot tell "suppressed because nothing is being
/// dehydrated" from "suppressed because there was nothing to say". Here there
/// is something to say and the answer is still nothing, because **the note is
/// tied to the REMOVAL and not to the verb.**
#[test]
fn keep_stays_silent_even_when_there_are_uncommitted_bytes_to_warn_about() {
  let fx = committed_fixture();
  fx.write_prose("ST0056", "reference.md", "bytes that are in no commit\n");

  let mut facade = fx.facade();
  let outcome = facade
    .st_done_listing("ST0056", ListEdit::Suppressed)
    .expect("done");

  assert!(
    outcome.notes().is_empty(),
    "`--keep` leaves the artefact listed, so no dehydration is coming and the \
     files are not at risk. Warning anyway would be correct-looking and is how \
     an operator learns to skim: {outcome:?}"
  );
  assert!(
    declared(&fx).declares("ST0056"),
    "precondition: `--keep` really did leave it listed, or the silence above \
     was measured on the wrong situation"
  );
}
