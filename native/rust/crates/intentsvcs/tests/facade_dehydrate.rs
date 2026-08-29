//! ST0061 AC-00.2 .. AC-00.6: **`Facade::dehydrate` is the inverse of
//! `hydrate`, and its whole safety property is one sentence -- a realised file
//! the store cannot be SHOWN to hold is a refusal naming the file, never a
//! deletion.**
//!
//! **EVERY REFUSING ARM HERE IS PAIRED WITH A CONTROL THAT MUST REMOVE, and the
//! pairing is the point rather than thoroughness.** "The file is still there"
//! is produced equally by a gate that refused and by a plan that was never
//! going to remove anything -- an empty step list, a path that classified
//! differently, a fixture that quietly stopped producing a dehydration
//! candidate. Without a control that goes the other way on the SAME tree, every
//! assertion in this file passes against a `dehydrate` that does nothing at
//! all.
//!
//! **AND THE CONTROLS ARE BUILT SO THEY COULD ONLY PASS IF THE DISCRIMINATION
//! WORKS.** A control that would also pass under the failure being guarded
//! against is decoration; the ones here differ from their refusing arm by
//! exactly the one fact under test and nothing else.

mod common;

use common::{Fixture, declaring_thread, gate_open, sample_thread};
use intentsvcs::address::{Address, Entity};
use intentsvcs::facade::FacadeError;
use intentsvcs::model::{AcKind, AcState};
use intentsvcs::remedy::Remedy;

/// Both threads declared: `ST0001` is the subject, and `ST0057` is
/// [`gate_open`]'s declaring thread, whose met precondition is what holds the
/// estate gate open. Declaring both keeps `ST0057`'s own files out of the
/// dehydration candidates, so nothing in these tests turns on the filter.
const MANIFEST: &str = "\
STEELTHREAD:ST0001
STEELTHREAD:ST0057
";

fn fixture() -> Fixture {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  fx.write_thread(&gate_open());
  fx.write_file("intent/.intentfiles", MANIFEST);
  fx
}

fn at(id: &str) -> Address {
  Address {
    authority: None,
    entity: Entity::Thread { id: id.to_string() },
    format: None,
  }
}

fn manifest_of(fx: &Fixture) -> String {
  std::fs::read_to_string(fx.path("intent/.intentfiles")).expect("manifest")
}

// ---------------------------------------------------------------------------
// THE CONTROL EVERY OTHER TEST IN THIS FILE LEANS ON
// ---------------------------------------------------------------------------

/// **Read this one first: it is the positive control for the whole file.**
/// If `dehydrate` ever stops removing anything, this is the only test here that
/// notices -- every other assertion is about something NOT happening.
#[test]
fn a_declared_thread_leaves_the_disk_and_the_manifest() {
  let fx = fixture();
  let mut f = fx.facade();
  let realised = f
    .hydrate(&at("ST0001"))
    .expect("a declared thread realises");
  assert!(
    !realised.is_empty(),
    "the fixture must produce at least one realised file, or every arm below is vacuous"
  );

  let done = f.dehydrate(&at("ST0001")).expect("permitted");

  assert!(done.unlisted, "the manifest entry went");
  assert!(!done.removed.is_empty(), "and so did files");
  for path in &realised {
    assert!(
      !path.exists(),
      "{} survived a permitted run",
      path.display()
    );
  }
  assert!(
    !manifest_of(&fx).contains("ST0001"),
    "the entry is gone from the manifest itself, not merely from the report"
  );
}

// ---------------------------------------------------------------------------
// AC-00.2 -- THE RAIL
// ---------------------------------------------------------------------------

/// **AT-00.2** (with `a_file_the_run_cannot_remove_refuses_it_and_names_it`).
///
/// A view whose bytes the store cannot reproduce. `organize::gate` matches
/// `Some(rendered) if *rendered == on_disk` and everything else falls to the
/// refusing arm, so this and an opaque attachment carrying `None` refuse
/// through the SAME wildcard rather than through two cases that could drift.
#[test]
fn a_hand_edited_view_refuses_the_run_and_nothing_is_removed() {
  let fx = fixture();
  let mut f = fx.facade();
  let realised = f.hydrate(&at("ST0001")).expect("realises");
  let victim = realised.first().expect("at least one file").clone();
  std::fs::write(&victim, "a hand edit no render could have produced\n").expect("edit it");

  let before: Vec<(std::path::PathBuf, Vec<u8>)> = realised
    .iter()
    .map(|p| (p.clone(), std::fs::read(p).expect("readable")))
    .collect();

  let err = f.dehydrate(&at("ST0001")).expect_err("refused");
  let rendered = err.render();
  let name = victim
    .file_name()
    .expect("a file name")
    .to_string_lossy()
    .to_string();
  assert!(
    rendered.contains(&name),
    "the refusal must NAME the file rather than count it -- this is the class that let \
     `1 refused` speak for 423 files: {rendered}"
  );

  for (path, bytes) in &before {
    assert_eq!(
      &std::fs::read(path).expect("still readable"),
      bytes,
      "{} changed on a refused run",
      path.display()
    );
  }
}

/// **AT-00.2**, second arm.
///
/// **vc's case, measured on Lamplight ST0306.** A file a human put in the
/// thread's directory that the renderer does not produce and the store does not
/// carry classifies `Unclaimed` -- *report, never remove*. The narrow gate
/// checked only DESTRUCTIVE steps, so this run would have removed the views,
/// delisted the thread and answered `dehydrated` with the file still there:
/// **the manifest saying one thing and the disk another, which is the exact
/// divergence this verb exists to prevent.**
#[test]
fn a_file_the_run_cannot_remove_refuses_it_and_names_it() {
  let fx = fixture();
  let mut f = fx.facade();
  f.hydrate(&at("ST0001")).expect("realises");
  fx.write_file(
    "intent/st/ST0001/a-human-put-this-here.md",
    "not rendered, not stored\n",
  );

  let err = f.dehydrate(&at("ST0001")).expect_err("refused");
  let rendered = err.render();
  assert!(
    rendered.contains("a-human-put-this-here.md"),
    "the unremovable file is named: {rendered}"
  );
  assert!(
    fx.path("intent/st/ST0001/a-human-put-this-here.md")
      .exists(),
    "and it is still there"
  );
}

// ---------------------------------------------------------------------------
// AC-00.3 -- THE REFUSAL IS DECIDED BEFORE THE DECLARATION MOVES
// ---------------------------------------------------------------------------

/// **AT-00.3.**
///
/// **A verb that unpinned first would convert a refusal into a DEFERRED
/// DELETION** -- carried out later by whoever next ran `organize --apply`
/// against a thread the manifest no longer declares, with nobody having decided
/// anything. Asserted by reading the file, never by inspecting the order of two
/// statements.
#[test]
fn a_refused_run_leaves_the_manifest_byte_identical() {
  let fx = fixture();
  let mut f = fx.facade();
  f.hydrate(&at("ST0001")).expect("realises");
  fx.write_file("intent/st/ST0001/blocker.md", "unremovable\n");

  let before = manifest_of(&fx);
  f.dehydrate(&at("ST0001")).expect_err("refused");
  assert_eq!(
    manifest_of(&fx),
    before,
    "a refused run must not have moved the declaration"
  );

  // **THE CONTROL, AND IT DIFFERS BY EXACTLY ONE FACT.** Same fixture, same
  // facade, same call -- only the blocker is gone. Without it, the assertion
  // above passes against a `dehydrate` that never writes the manifest at all.
  std::fs::remove_file(fx.path("intent/st/ST0001/blocker.md")).expect("clear the blocker");
  f.dehydrate(&at("ST0001")).expect("now permitted");
  assert_ne!(
    manifest_of(&fx),
    before,
    "the control must MOVE the declaration, or the assertion above proves nothing"
  );
}

// ---------------------------------------------------------------------------
// AC-00.4 -- AN ABSENT MANIFEST REFUSES
// ---------------------------------------------------------------------------

/// **AT-00.4.**
///
/// ABSENT means *nobody has said*, so everything is realised. Creating a
/// manifest here would declare that everything EXCEPT this thread is realised
/// -- an estate-wide assertion nobody made, arrived at through a single-thread
/// verb.
#[test]
fn an_absent_manifest_refuses_and_one_that_omits_the_id_does_not() {
  let fx = fixture();
  let mut f = fx.facade();
  std::fs::remove_file(fx.path("intent/.intentfiles")).expect("take it away");

  let err = f.dehydrate(&at("ST0001")).expect_err("absent refuses");
  assert!(
    matches!(err, FacadeError::NoManifestToUnlistFrom { .. }),
    "absence has its own variant rather than borrowing unreadable's: {}",
    err.render()
  );

  // **THE CONTROL, AND IT IS THE DISTINCTION THE CRITERION IS ABOUT.** A
  // manifest that EXISTS and simply does not name this thread is an ordinary
  // exit 0, not a refusal -- so the arm above is about absence and not about
  // the id being missing from a list.
  fx.write_file("intent/.intentfiles", "STEELTHREAD:ST0057\n");
  let done = f
    .dehydrate(&at("ST0001"))
    .expect("a manifest that does not list the id is not an error");
  assert!(
    !done.unlisted,
    "nothing was delisted, because nothing was listed"
  );
}

// ---------------------------------------------------------------------------
// AC-00.5 -- THE ESTATE PRECONDITIONS GATE THIS DOOR TOO
// ---------------------------------------------------------------------------

/// **AT-00.5, the no-declaration arm.** Its sibling is
/// `a_declared_but_unmet_precondition_refuses_and_names_it` and the met control
/// is `a_declared_thread_leaves_the_disk_and_the_manifest`: three states --
/// cannot read a list, read one and found it wanting, read one and it permits
/// -- because the first two are both refusals and only the third proves the
/// gate can ever say yes.
///
/// **A per-thread verb must not be a way around the estate gate.** If it were,
/// the gate would protect only the operator who happened to reach for
/// `organize`, and the NARROWER verb would be the one that deletes.
#[test]
fn an_estate_with_no_declaration_refuses_this_door_too() {
  // Deliberately NOT `fixture()`: no `gate_open()` thread, so the estate
  // declares no preconditions at all and the honest answer is "I do not know"
  // -- which has to mean no.
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  fx.write_file("intent/.intentfiles", "STEELTHREAD:ST0001\n");
  let mut f = fx.facade();
  let realised = f.hydrate(&at("ST0001")).expect("realises");

  f.dehydrate(&at("ST0001"))
    .expect_err("an estate that has declared nothing has proved nothing");
  for path in &realised {
    assert!(
      path.exists(),
      "{} was removed through a door the estate gate does not cover",
      path.display()
    );
  }
}

// ---------------------------------------------------------------------------
// AC-00.6 -- NOTHING TO DO NEVER READS AS DID SOMETHING
// ---------------------------------------------------------------------------

/// **AT-00.6.**
///
/// `unpin` is idempotent by contract; this is about the REPORT, because a count
/// that reads the same for zero and for one is the class that let `1 refused`
/// speak for 423 files.
#[test]
fn a_second_run_reports_differently_from_the_first() {
  let fx = fixture();
  let mut f = fx.facade();
  f.hydrate(&at("ST0001")).expect("realises");

  let first = f.dehydrate(&at("ST0001")).expect("permitted");
  let second = f.dehydrate(&at("ST0001")).expect("idempotent");

  assert!(
    first.unlisted && !first.removed.is_empty(),
    "the first run did something"
  );
  assert!(
    !second.unlisted && second.removed.is_empty() && second.pruned.is_empty(),
    "the second had nothing to do"
  );
  assert_ne!(
    (first.unlisted, first.removed.len()),
    (second.unlisted, second.removed.len()),
    "the two reports must be distinguishable by a caller, not merely both succeed"
  );
}

// ---------------------------------------------------------------------------
// A DIRECTORY THE RUN COULD NOT EMPTY IS NAMED (vc, 2026-08-26)
// ---------------------------------------------------------------------------

/// **`prune_emptied` skips a directory it cannot delete through an `is_ok()`,
/// which is the right floor and a silent report.** Content outside the corpus
/// legitimately survives and must not refuse the run -- but the manifest now
/// says dehydrated while a directory tree remains. git leaves ignored files
/// behind too and says nothing, because git keeps no manifest to contradict.
/// We do, so the verdict names them.
///
/// **THE FIXTURE IS AN EMPTY SUBDIRECTORY RATHER THAN AN IGNORED FILE, AND THE
/// LIMIT IS STATED RATHER THAN HIDDEN.** The real-world shape is Lamplight
/// ST0306's gitignored review gifs; reproducing *ignored-ness* needs git's own
/// rules and would make this test's subject the ignore machinery instead of the
/// report. An empty subdirectory reds on exactly the same defect: it is not an
/// ancestor of any removed file, so a `left_in_place` derived from the removal
/// set -- which is what the first implementation did -- cannot see it.
#[test]
fn a_directory_the_run_could_not_empty_is_named() {
  let fx = fixture();
  let mut f = fx.facade();
  f.hydrate(&at("ST0001")).expect("realises");
  std::fs::create_dir_all(fx.path("intent/st/ST0001/images")).expect("a surviving subdirectory");

  let done = f
    .dehydrate(&at("ST0001"))
    .expect("permitted -- nothing here refuses");

  assert!(
    done.left_in_place.iter().any(|p| p.ends_with("images")),
    "the surviving directory must be NAMED in the verdict, never left to a bare `dehydrated`: \
     {:?}",
    done.left_in_place
  );
}

/// **AT-00.5's MISSING ARM, and it is a different question from the one above.**
///
/// `an_estate_with_no_declaration_refuses_this_door_too` is the gate unable to
/// read a list at all. This is the gate reading one and finding it wanting --
/// the ordinary case, and the one an estate actually lives in.
///
/// **TWO PRECONDITIONS ARE DECLARED AND ONLY ONE IS UNMET, WHICH IS WHAT MAKES
/// THIS MORE THAN A REFUSAL TEST.** The denominator is the assertion that
/// matters -- preconditions CHECKED against preconditions DECLARED -- because a
/// gate quietly checking a stale SUBSET would still refuse, would still name a
/// real unmet id, and would look correct from every angle an "it refused" test
/// can see. With two declared, a subset check reports `1 of 1` where the truth
/// is `2 of 2`, and the arithmetic is what catches it. The met one must also
/// NOT appear, which a refusal that printed the whole declaration would fail.
#[test]
fn a_declared_but_unmet_precondition_refuses_and_names_it() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  fx.write_thread(&declaring_thread(&[
    (
      "AC-00.8",
      AcKind::NonTest,
      AcState::Satisfied {
        evidence: "met, and it must not be reported as unmet".to_string(),
      },
    ),
    (
      "AC-00.9",
      AcKind::NonTest,
      AcState::Unsatisfied { note: None },
    ),
  ]));
  fx.write_file("intent/.intentfiles", MANIFEST);
  let mut f = fx.facade();
  let realised = f.hydrate(&at("ST0001")).expect("realises");

  let err = f
    .dehydrate(&at("ST0001"))
    .expect_err("a declared precondition is unmet, so the gate answers no");
  let rendered = err.render();

  assert!(
    rendered.contains("AC-00.9"),
    "the unmet precondition must be NAMED, not counted: {rendered}"
  );
  assert!(
    rendered.contains("2 checked of 2 declared"),
    "the denominator must follow the DECLARATION -- a gate checking a stale subset would \
     report `1 checked of 1 declared` here and still refuse, which no refusal assertion \
     can tell apart: {rendered}"
  );
  assert!(
    !rendered.contains("AC-00.8"),
    "a MET precondition must not appear in the refusal, or the gate is printing the \
     declaration rather than the verdict: {rendered}"
  );
  for path in &realised {
    assert!(
      path.exists(),
      "{} was removed while a declared precondition was unmet",
      path.display()
    );
  }
}
