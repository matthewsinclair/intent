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
#[test]
fn a_generated_view_is_refused_and_names_a_different_surface_for_each() {
  let mut said = Vec::new();
  for view in ["info", "acceptance"] {
    let fx = fixture();
    let mut facade = fx.facade();
    let err = facade.edit(&thread(), view).expect_err("refused");
    match err {
      FacadeError::NotEditable { author_with, .. } => said.push(author_with),
      other => panic!("`{view}` must be NotEditable, got: {other:?}"),
    }
  }
  assert_ne!(
    said[0], said[1],
    "`info.md` and `acceptance.md` are authored by DIFFERENT verbs, and one \
     message for both is the dead end the ruling exists to avoid: {said:?}"
  );
}

/// **AND THE REFUSAL WRITES NOTHING TO THE PATH IT REFUSED.** A refusal that
/// realised the view anyway would leave the operator with a file they were just
/// told not to edit.
#[test]
fn a_refused_view_is_still_realised_because_the_refusal_is_about_authoring() {
  // **THIS IS NOT THE OBVIOUS ANSWER AND IT IS THE RIGHT ONE.** `edit` refuses
  // to hand over the PATH; it does not refuse to realise the artefact, and the
  // realisation already happened -- `hydrate` runs first and the whole thread
  // comes back. Asserting the file is ABSENT here would be asserting that a
  // refusal rolls back an unrelated act.
  let fx = fixture();
  let mut facade = fx.facade();
  facade.edit(&thread(), "info").expect_err("refused");
  assert!(
    fx.path("intent/st/ST0001/info.md").exists(),
    "the thread was realised before the disposition was consulted, and the \
     refusal is about where to AUTHOR rather than about what may exist"
  );
}

// ---------------------------------------------------------------------------
// THE DENOMINATOR
// ---------------------------------------------------------------------------

/// **EVERY FORM THE VERB DISPATCHES ON IS EDITED OR REFUSED BY NAME.**
#[test]
fn every_address_form_is_edited_or_refused_by_name() {
  let forms = [
    Entity::Threads,
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
    11,
    "the partition must cover every form: {editable} editable, refused {refused:?}"
  );
  assert_eq!(
    refused,
    // **DECLARED, NOT COMPUTED.** A set derived from the code absorbs a form
    // moving between buckets in silence -- which is how `issue` travelled from
    // hydratable to refused with nothing reporting it until a list like this
    // one said otherwise.
    vec!["threads", "issue", "node", "node-inbox", "event"],
    "the refused set is declared, so a form moving between buckets is visible"
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
