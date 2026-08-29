//! **AT-05.1 / AC-05.1 + AC-05.3 -- `intent edit <ID>` DISPATCHES ON ID SHAPE,
//! HYDRATES WHEN ABSENT, AND PRINTS A PATH THAT EXISTS AFTER THE CALL.**
//!
//! Three claims, and they fail in different ways, so they are asserted apart:
//!
//! 1. **The path EXISTS.** v2's `st edit` printed one whether or not anything
//!    was there -- its own note says _the thread DIRECTORY must exist; the file
//!    need not_. Under the disk model an artefact may legitimately have no
//!    realised form at all, so the same command on a dehydrated thread names a
//!    path to nothing and sends an editor to create it.
//! 2. **A generated view is REFUSED and the refusal names where to author**
//!    (hv, 2026-08-19). **Detection is not prevention:** the skew check catches
//!    a hand-edited view AFTER the work is gone.
//! 3. **The denominator closes** over every form the verb dispatches on.
//!
//! # The assertion that would be easiest to fake is the denominator
//!
//! A verb that silently skipped four of eleven address forms reports the same
//! number as one that handled all eleven. So [`every_address_form_is_edited_or_
//! refused_by_name`] walks the whole set and requires each member to produce a
//! path or refuse BY NAME, and it asserts the refused set as a DECLARED LIST --
//! a set computed from the code would absorb a form changing buckets in
//! silence, which is exactly how `issue` moved from hydratable to refused
//! without anyone noticing until a declared list said so.

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::address::{Address, Entity};
use intentsvcs::facade::FacadeError;

const MANIFEST: &str = "\
# .intentfiles

# BEGIN INTENT
# END INTENT
";

/// **THE FIXTURE CARRIES A `design.md` AND `sample_thread` DOES NOT.**
///
/// This is worth a sentence because the shared fixture nearly made the whole
/// file vacuous. `sample_thread`'s attachments are `reference.md` and
/// `parity/cmd-st.md`, and **neither is a member of the `file` enum** the verb
/// accepts (`info | design | impl | tasks | acceptance`). Of those five, two
/// are generated views and are refused, so against the stock fixture there is
/// no argument at all for which `edit` returns a path -- every test here would
/// have asserted a refusal and none would have shown the verb working.
fn fixture() -> Fixture {
  let fx = Fixture::new();
  let mut thread = sample_thread("ST0001");
  thread.attachments.push(intentsvcs::model::Attachment::new(
    "design.md",
    "# Design\n\nAuthored on disk, which is why `edit` hands it over.\n",
  ));
  fx.write_thread(&thread);
  fx.write_file("intent/.intentfiles", MANIFEST);
  fx
}

fn at(entity: Entity) -> Address {
  Address {
    authority: None,
    entity,
    format: None,
  }
}

fn thread() -> Address {
  at(Entity::Thread {
    id: "ST0001".to_string(),
  })
}

// ---------------------------------------------------------------------------
// THE CRITERION: A PATH THAT EXISTS
// ---------------------------------------------------------------------------

/// **THE HEADLINE, AND THE FIXTURE STARTS DEHYDRATED SO IT MEANS SOMETHING.**
///
/// The manifest declares nothing and the thread's files are absent, which is
/// the state the whole criterion is about: v2's verb would have printed a path
/// into empty space. This one realises first, so what it prints is there.
#[test]
fn an_absent_artefact_is_realised_and_the_printed_path_exists() {
  let fx = fixture();
  assert!(
    !fx.path("intent/st/ST0001/design.md").exists(),
    "precondition: the fixture starts DEHYDRATED -- if the file is already \
     there, this test cannot show that `edit` realised it"
  );

  let mut facade = fx.facade();
  let path = facade
    .edit(&thread(), "design")
    .expect("an attachment opens");

  assert!(
    path.exists(),
    "AC-05.1's whole criterion is a path that EXISTS after the call: {path:?}"
  );
  assert!(
    path.ends_with("design.md"),
    "and it is the file that was asked for: {path:?}"
  );
}

/// The realisation is `hydrate`'s, not a second one -- so the artefact is
/// pinned too, and a following `organize` will not take back what `edit` just
/// handed the operator.
#[test]
fn editing_pins_the_artefact_so_the_next_organize_keeps_it() {
  let fx = fixture();
  let mut facade = fx.facade();
  facade.edit(&thread(), "design").expect("opens");

  let manifest = fx.read("intent/.intentfiles");
  assert!(
    intentsvcs::intentfiles::realised_from(&manifest).declares("ST0001"),
    "`edit` realised the thread and did not list it, so the next `organize` \
     removes the file it just printed:\n{manifest}"
  );
}

/// **A FILE THE ARTEFACT DOES NOT CARRY IS REFUSED, AND THIS IS A DELIBERATE
/// DEVIATION FROM v2.** Printing the path anyway sends an editor to create an
/// untracked file beside the artefact -- the `Unclaimed` population `organize`
/// already reports.
#[test]
fn a_file_the_artefact_does_not_carry_is_refused_and_says_what_is_there() {
  let fx = fixture();
  let mut facade = fx.facade();
  let err = facade
    .edit(&thread(), "tasks")
    .expect_err("the sample thread carries no tasks.md");

  match &err {
    FacadeError::NoSuchEditable { present, .. } => assert!(
      !present.is_empty(),
      "the refusal must say what IS there -- naming only what is not sends the \
       operator back to `ls`"
    ),
    other => panic!("expected NoSuchEditable, got: {other:?}"),
  }
}

// ---------------------------------------------------------------------------
// hv's RULING: A GENERATED VIEW IS REFUSED, AND THE REFUSAL CARRIES THE
// DESTINATION
// ---------------------------------------------------------------------------

/// **BOTH VIEWS, AND THEY MUST NOT SHARE A MESSAGE.**
///
/// An operator who wanted to add a criterion and one who wanted to retitle a
/// thread need different verbs, and "this is generated" sends both to the same
/// dead end. The two destinations are asserted to DIFFER, which is the half a
/// single generic refusal would still pass.
///
/// **NARROWED 2026-08-29: `info` NO LONGER REFUSES, AND THAT IS hv's RULING
/// RATHER THAN A WEAKENED TEST.** This arm used to drive `info` and
/// `acceptance` and assert their destinations differed. hv drove `intent st
/// edit 68`, met the `info` refusal, and ruled the thread cover round-trips --
/// **because that refusal's own remedy was a dead end: it said author it with
/// `intent st`, and not one of the seventeen `intent st` verbs writes
/// `objective` or `context`.** The differ-by-surface property did not go away
/// with the subject; it moved down to `Project::edit_disposition`'s own unit
/// test, which compares all three destinations without needing a facade.
#[test]
fn a_generated_view_is_refused_and_names_the_surface_that_authors_it() {
  let fx = fixture();
  let mut facade = fx.facade();
  match facade
    .edit(&thread(), "acceptance")
    .expect_err("acceptance is generated")
  {
    FacadeError::NotEditable { author_with, .. } => assert!(
      author_with.contains("intent ac"),
      "the refusal must name the verb that authors criteria, said `{author_with}`"
    ),
    other => panic!("`acceptance` must be NotEditable, got: {other:?}"),
  }

  // **THE OTHER HALF OF THE SAME RULING, ASSERTED HERE SO THE PAIR CANNOT
  // DRIFT APART.** A change that made `info` refuse again would leave the arm
  // above green and this one red, which is the point: the two are one ruling.
  let fx = fixture();
  let mut facade = fx.facade();
  facade
    .edit(&thread(), "info")
    .expect("hv ruled the thread cover open on 2026-08-29");
}

/// **AND THE REFUSAL WRITES NOTHING TO THE PATH IT REFUSED.** A refusal that
/// realised the view anyway would leave the operator with a file they were just
/// told not to edit.
#[test]
fn a_refusal_does_not_roll_back_a_hydrate_that_already_happened() {
  // **THIS IS NOT THE OBVIOUS ANSWER AND IT IS THE RIGHT ONE.** `edit` refuses
  // to hand over the PATH; it does not refuse to realise the artefact, and the
  // realisation already happened. Asserting the file is ABSENT here would be
  // asserting that a refusal rolls back an unrelated act.
  //
  // **AMENDED 2026-08-22 (vc), MOVED FROM THE `info` ARM AND NOT WEAKENED.**
  // It used to drive `edit(.., "info")`, whose refusal is a pure function of
  // the FILENAME and now decides BEFORE `hydrate` runs -- so on that arm there
  // is no completed act for a rollback to reach, and the proposition became
  // untestable rather than false. **A ruling about rollback cannot reach an
  // act that was never performed.**
  //
  // `impl` is an AUTHORED name, so it passes the disposition gate, hydrate
  // runs, and only then does the artefact turn out not to carry it. That is a
  // real refusal after a real write, which is exactly the shape this test was
  // written to govern. The argument survives; only its arm moved.
  let fx = fixture();
  let mut facade = fx.facade();
  facade.edit(&thread(), "impl").expect_err("refused");
  assert!(
    fx.path("intent/st/ST0001/info.md").exists(),
    "the thread was realised before the file turned out to be absent, and the \
     refusal is about what this artefact CARRIES rather than about what may exist"
  );
}

/// **THE REFUSAL THAT NEEDS NOTHING FROM DISK DECIDES BEFORE ANYTHING IS
/// WRITTEN** -- ic's finding, and the one affected project is this one.
///
/// Driven at `21ea0e8f` before the fix: `intent st edit ST0001` -- where `file`
/// defaults to `info`, the one name this verb refuses -- created two files and
/// appended `STEELTHREAD:ST0001` to the TRACKED `.intentfiles`, then exited 1.
///
/// **THE DEFECT IS NOT THAT `edit` WRITES. IT IS THAT THE EXIT CODE AND THE
/// EFFECT DISAGREE**, which is the arm IN-AG-NO-SILENT-001 never names: the
/// error is surfaced correctly and the EFFECT is hidden. A caller told the
/// operation did not happen has a dirty tracked file and no reason to look.
///
/// **BOTH LIMBS, BECAUSE EITHER ALONE PASSES FOR THE WRONG REASON.** A verb
/// that refused everything would leave the estate untouched and satisfy the
/// second limb; one that never refused would satisfy neither. The manifest is
/// asserted separately from the views because they are written by different
/// steps and a fix could plausibly reach one and not the other.
#[test]
fn the_filename_refusal_writes_nothing_at_all() {
  let fx = fixture();
  let mut facade = fx.facade();

  // A clean starting point: no realised views, an empty-but-present manifest.
  let dir = fx.path("intent/st/ST0001");
  std::fs::remove_dir_all(&dir).ok();
  std::fs::write(fx.path("intent/.intentfiles"), MANIFEST).unwrap();

  // **THE SUBJECT MOVED WITH hv's RULING AND THE DEFECT DID NOT.** This arm
  // drove `info` because `info` was the register's default AND a name this
  // verb refused. hv ruled the cover open on 2026-08-29, so `info` is not a
  // refusal at all any more; `acceptance` is the generated view that still is,
  // and the defect being guarded -- a refusal whose EXIT CODE and EFFECT
  // disagree -- is a property of the refusal, not of which view triggered it.
  //
  // **`design` WAS TRIED HERE FIRST AND IS THE WRONG SUBJECT, WHICH IS WORTH
  // A LINE BECAUSE IT LOOKS RIGHT.** It is an authored attachment: `hydrate`
  // REALISES it and `edit` returns its path at rc=0. Reaching for it as "a
  // file that is not realised" confuses a file the artefact does not CARRY
  // with one that is merely not on disk yet, and the fixture carries it.
  facade
    .edit(&thread(), "acceptance")
    .expect_err("`acceptance` is a generated view and is refused");

  assert!(
    !dir.exists(),
    "the refusal realised the thread: {:?}",
    std::fs::read_dir(&dir)
      .map(|d| d.flatten().map(|e| e.file_name()).collect::<Vec<_>>())
      .unwrap_or_default()
  );
  let manifest = fx.read("intent/.intentfiles");
  assert!(
    !intentsvcs::intentfiles::realised_from(&manifest).declares("ST0001"),
    "the refusal pinned the artefact in a TRACKED file while reporting rc=1:\n{manifest}"
  );
}

// ---------------------------------------------------------------------------
// THE DENOMINATOR
// ---------------------------------------------------------------------------

/// **A TYPO IS REPORTED AS A TYPO -- `intent#0144`'s second half.**
///
/// Neither answer the verb used to give named the thing the operator actually
/// got wrong. An authored file got `is not a file this artefact carries` and a
/// view got `is generated from the model`, both describing a FILE inside a
/// thread that was never there -- so the one diagnosis that would have helped
/// was the one diagnosis unavailable. **The file argument decided which wrong
/// story you got**, which is why both are asserted here rather than one.
#[test]
fn an_unknown_id_is_reported_as_unknown_whichever_file_is_asked_for() {
  for file in ["design", "impl", "tasks", "info", "acceptance"] {
    let fx = fixture();
    let mut facade = fx.facade();
    let err = facade
      .edit(
        &at(Entity::Thread {
          id: "ST9997".to_string(),
        }),
        file,
      )
      .expect_err("ST9997 names no thread");

    assert!(
      matches!(&err, FacadeError::NoSuchThread { id } if id == "ST9997"),
      "`{file}` on an unknown id must name the ID as the fault, not a file \
       inside a thread that does not exist: {err:?}"
    );

    // **THE TRACKED FILE IS THE POINT, NOT THE MESSAGE.** The authored three
    // realised first and refused afterwards, leaving `STEELTHREAD:ST9997`
    // behind in version control for a peer to commit under their own name.
    let manifest = fx.read("intent/.intentfiles");
    assert!(
      !intentsvcs::intentfiles::realised_from(&manifest).declares("ST9997"),
      "`{file}` pinned a thread that does not exist into a TRACKED file:\n{manifest}"
    );
  }
}

/// **EVERY FORM THE VERB DISPATCHES ON IS EDITED OR REFUSED BY NAME.**
#[test]
fn every_address_form_is_edited_or_refused_by_name() {
  let forms = [
    Entity::Threads,
    Entity::Issues,
    Entity::WpCollection {
      thread: "ST0001".to_string(),
    },
    Entity::Thread {
      id: "ST0001".to_string(),
    },
    Entity::AcCollection {
      thread: "ST0001".to_string(),
    },
    Entity::Wp {
      thread: "ST0001".to_string(),
      wp: "02".to_string(),
    },
    Entity::Ac {
      thread: "ST0001".to_string(),
      ac: "AC-03.1".to_string(),
    },
    Entity::At {
      thread: "ST0001".to_string(),
      at: "AT-03.1".to_string(),
    },
    Entity::Attachment {
      thread: "ST0001".to_string(),
      path: "reference.md".to_string(),
    },
    Entity::Issue {
      id: "0021".to_string(),
    },
    Entity::Node {
      moniker: "dc".to_string(),
    },
    Entity::NodeInbox {
      moniker: "dc".to_string(),
      sender: "vc".to_string(),
      stamp: "2026-08-19 16:00Z".to_string(),
    },
    Entity::Event {
      id: "1".to_string(),
    },
  ];

  let mut editable = 0;
  let mut refused = Vec::new();
  for entity in forms {
    let form = entity.form();
    let fx = fixture();
    let mut facade = fx.facade();
    match facade.edit(&at(entity), "design") {
      Ok(path) => {
        assert!(path.exists(), "{form} returned a path that is not there");
        editable += 1;
      }
      Err(FacadeError::NotHydratable { form: named, .. }) => {
        assert_eq!(named, form, "a refusal must name the form it refused");
        refused.push(form);
      }
      Err(other) => panic!("{form} must be editable or NotHydratable, got {other:?}"),
    }
  }

  assert_eq!(
    editable + refused.len(),
    13,
    "the partition must cover every form: {editable} editable, refused {refused:?}"
  );
  assert_eq!(
    refused,
    // **DECLARED, NOT COMPUTED.** A set derived from the code absorbs a form
    // moving between buckets in silence -- which is how `issue` travelled from
    // hydratable to refused with nothing reporting it until a list like this
    // one said otherwise.
    vec!["threads", "issues", "issue", "node", "node-inbox", "event"],
    "the refused set is declared, so a form moving between buckets is visible.\n\n  \
     `wp-collection` is EDITABLE and I had it here on a guess until this \
     assertion\n  \
     said otherwise: it resolves to its thread exactly as `ac-collection` \
     does, so\n  \
     editing one realises the thread and hands over the thread's file. The two \
     COLLECTIONS\n  \
     that refuse are the ones owning no thread -- `threads` and `issues`."
  );
}

// ---------------------------------------------------------------------------
// AC-05.3: ONE HOME
// ---------------------------------------------------------------------------

/// **PATH-PRINTING HAS ONE HOME, AND THIS ASSERTS IT ON THE SOURCE BECAUSE
/// BEHAVIOUR CANNOT.**
///
/// Two implementations that agree today pass every behavioural test in this
/// file; Highlander is a claim about how many there ARE, and the only thing
/// that can go red when a second one appears is a count of the call sites.
#[test]
fn the_renderer_calls_the_edit_door_exactly_once() {
  let src = std::fs::read_to_string(
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
      .join("../intent-cli/src/render.rs")
      .canonicalize()
      .expect("the renderer is beside this crate"),
  )
  .expect("read the renderer");

  let calls = src.matches(".edit(&address").count();
  assert_eq!(
    calls, 1,
    "AC-05.3: `intent edit` and `intent st edit` are ONE implementation. {calls} \
     call sites means a second one has appeared, and two that agree today are \
     two that can drift tomorrow."
  );
}

// ---------------------------------------------------------------------------
// THE DENOMINATOR'S DENOMINATOR
// ---------------------------------------------------------------------------

/// **THE COMPILER IS THE ONLY THING THAT CAN COUNT AN ENUM'S VARIANTS, SO IT
/// COUNTS THEM.**
///
/// The list in [`every_address_form_is_edited_or_refused_by_name`] is written
/// by hand, and a hand list stops covering on the day somebody adds a variant.
/// **That is not hypothetical: `Issues` and `WpCollection` landed on
/// 2026-08-20 and the list of eleven went on passing**, reporting a complete
/// partition over eleven of thirteen forms. The test that existed to close the
/// denominator had a denominator of its own and nothing was watching it.
///
/// This match is EXHAUSTIVE, so a fourteenth variant fails to compile HERE --
/// in the file that then has to grow a case for it -- rather than passing
/// quietly next door.
///
/// **What it does NOT do is check that the list above was updated too**, and
/// saying so matters: it converts a silent miss into a compile error pointing
/// at the right file, which is a smaller claim than coverage and is the whole
/// of what a language without variant reflection allows.
fn _every_variant_is_accounted_for(entity: &Entity) -> &'static str {
  match entity {
    Entity::Threads => "threads",
    Entity::Issues => "issues",
    Entity::WpCollection { .. } => "wp-collection",
    Entity::Thread { .. } => "thread",
    Entity::AcCollection { .. } => "ac-collection",
    Entity::Wp { .. } => "wp",
    Entity::Ac { .. } => "ac",
    Entity::At { .. } => "at",
    Entity::Attachment { .. } => "attachment",
    Entity::Issue { .. } => "issue",
    Entity::Node { .. } => "node",
    Entity::NodeInbox { .. } => "node-inbox",
    Entity::Event { .. } => "event",
  }
}

/// The witness above agrees with the implementation's own naming, so the two
/// cannot drift into calling the same form different things.
#[test]
fn the_witness_names_forms_the_way_the_implementation_does() {
  for entity in [
    Entity::Threads,
    Entity::Issues,
    Entity::WpCollection {
      thread: "ST0001".to_string(),
    },
  ] {
    assert_eq!(_every_variant_is_accounted_for(&entity), entity.form());
  }
}
