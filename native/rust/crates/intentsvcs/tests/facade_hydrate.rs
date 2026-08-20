//! AC-05.1's primitive: **`Facade::hydrate` makes an addressed artefact's files
//! exist, and returns the paths that now do.**
//!
//! **THE ARM THAT MATTERS MOST IS THE ONE ABOUT A CALL THAT WRITES NOTHING.**
//! The obvious implementation returns early when the files are already there,
//! and that skips the PIN in exactly the ordinary case: the artefact is on disk
//! because it is currently `wip`, its id sits in the GENERATED region, and it is
//! not pinned. Presence is true and pinned-ness is false, and they disagree on
//! the common path rather than in a corner. So "hydrating something already
//! hydrated still pins it" is not an edge case here -- it is the case.
//!
//! **AND THE REFUSALS ARE COUNTED RATHER THAN ASSUMED.** A verb that silently
//! skipped four of eleven address forms would report the same denominator as one
//! that handled all eleven. The partition test walks every form and requires
//! each to be hydratable or refused BY NAME, so a tenth form cannot arrive and
//! be quietly dropped.

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::address::{Address, Entity, Format};
use intentsvcs::facade::FacadeError;
use intentsvcs::intentfiles;

const MANIFEST: &str = "\
# .intentfiles

# BEGIN INTENT
# END INTENT
";

fn fixture() -> Fixture {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
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

fn manifest_of(fx: &Fixture) -> String {
  std::fs::read_to_string(fx.path("intent/.intentfiles")).expect("manifest")
}

fn is_pinned(fx: &Fixture, id: &str) -> bool {
  intentfiles::parse(&manifest_of(fx))
    .expect("manifest parses")
    .pinned()
    .any(|e| e.id == id)
}

// ---------------------------------------------------------------------------
// THE TWO STEPS, AND THAT NEITHER GUARDS THE OTHER
// ---------------------------------------------------------------------------

#[test]
fn an_absent_artefact_is_realised_and_every_returned_path_exists() {
  let fx = fixture();
  let mut facade = fx.facade();
  let paths = facade
    .hydrate(&at(Entity::Thread {
      id: "ST0001".to_string(),
    }))
    .expect("a thread is hydratable");

  assert!(
    !paths.is_empty(),
    "hydrating an absent thread must produce files"
  );
  for path in &paths {
    assert!(
      path.exists(),
      "the criterion is a path that EXISTS after the call: {path:?}"
    );
  }
  assert!(is_pinned(&fx, "ST0001"), "and it must be pinned");
}

#[test]
fn hydrating_something_already_on_disk_still_pins_it() {
  // **THE DEFECT A PRESENCE CHECK WOULD HAVE SHIPPED, AND IT IS THE ORDINARY
  // CASE RATHER THAN A CORNER.** The files are there, the id is not pinned, and
  // an implementation that returned early on presence would leave it that way --
  // handing the artefact straight back to the next `organize`.
  let fx = fixture();
  let mut facade = fx.facade();
  let address = at(Entity::Thread {
    id: "ST0001".to_string(),
  });
  facade.hydrate(&address).expect("first call");

  // Un-pin, leaving the FILES exactly where they are. This is the state the
  // generated region produces for a live thread.
  fx.write_file("intent/.intentfiles", MANIFEST);
  assert!(!is_pinned(&fx, "ST0001"), "precondition: not pinned");
  assert!(
    fx.path("intent/st/ST0001/info.md").exists(),
    "precondition: the files are present, so presence and pinned-ness disagree"
  );

  facade.hydrate(&address).expect("second call");
  assert!(
    is_pinned(&fx, "ST0001"),
    "a call that writes no file must still pin -- the two steps are independent"
  );
}

#[test]
fn the_pin_is_idempotent() {
  let fx = fixture();
  let mut facade = fx.facade();
  let address = at(Entity::Thread {
    id: "ST0001".to_string(),
  });
  facade.hydrate(&address).expect("first");
  let after_one = manifest_of(&fx);
  facade.hydrate(&address).expect("second");
  assert_eq!(
    after_one,
    manifest_of(&fx),
    "pinning an already-pinned artefact must not add a second line"
  );
}

#[test]
fn the_call_is_idempotent_in_what_it_returns() {
  let fx = fixture();
  let mut facade = fx.facade();
  let address = at(Entity::Thread {
    id: "ST0001".to_string(),
  });
  let first = facade.hydrate(&address).expect("first");
  let second = facade.hydrate(&address).expect("second");
  assert_eq!(first, second, "the same artefact reports the same paths");
}

// ---------------------------------------------------------------------------
// WHAT THE ADDRESS DECIDES, AND WHAT IT MUST NOT
// ---------------------------------------------------------------------------

#[test]
fn every_sub_thread_form_realises_its_thread() {
  // An AC, an AT, a WP and the acceptance collection have no files of their own.
  // `.intentfiles` names artefacts and nothing finer, so the artefact is the
  // smallest thing realisation can address.
  let forms = [
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
  ];
  for entity in forms {
    let form = entity.form();
    let fx = fixture();
    let mut facade = fx.facade();
    let paths = facade
      .hydrate(&at(entity))
      .unwrap_or_else(|e| panic!("{form} must realise its thread: {e}"));
    assert!(!paths.is_empty(), "{form} produced nothing");
    assert!(is_pinned(&fx, "ST0001"), "{form} must pin the THREAD");
  }
}

#[test]
fn the_format_is_ignored_because_it_names_a_representation_not_an_artefact() {
  // `?format=json` and `?format=md` name the SAME artefact. A verb that read the
  // format would have an opinion about representation that AC-05.1 never asked
  // for.
  let mut out = Vec::new();
  for format in [None, Some(Format::Json), Some(Format::Md)] {
    let fx = fixture();
    let mut facade = fx.facade();
    let paths = facade
      .hydrate(&Address {
        authority: None,
        entity: Entity::Thread {
          id: "ST0001".to_string(),
        },
        format,
      })
      .expect("hydratable regardless of format");
    out.push(
      paths
        .iter()
        .map(|p| p.file_name().unwrap().to_string_lossy().into_owned())
        .collect::<Vec<_>>(),
    );
  }
  assert_eq!(out[0], out[1], "json must realise what a bare address does");
  assert_eq!(out[1], out[2], "md must realise what json does");
}

#[test]
fn another_project_s_address_is_refused_rather_than_ignored() {
  // A non-empty authority names a DIFFERENT PROJECT. Ignoring it would realise
  // that project's artefact into this tree, which is not a representation
  // question -- it is wrong.
  let fx = fixture();
  let mut facade = fx.facade();
  let err = facade
    .hydrate(&Address {
      authority: Some("lamplight".to_string()),
      entity: Entity::Thread {
        id: "ST0001".to_string(),
      },
      format: None,
    })
    .expect_err("a foreign authority must refuse");
  assert!(
    matches!(err, FacadeError::NotHydratable { .. }),
    "got {err:?}"
  );
  assert!(
    !fx.path("intent/st/ST0001/info.md").exists(),
    "and it must write nothing"
  );
}

// ---------------------------------------------------------------------------
// THE DENOMINATOR
// ---------------------------------------------------------------------------

#[test]
fn every_address_form_is_hydratable_or_refused_by_name() {
  // **THE PARTITION MUST CLOSE, AND THE REFUSALS ARE IN THE DENOMINATOR.** A
  // verb that silently skipped four of eleven forms reports the same number as
  // one that handled all eleven. Every form is walked; each must either realise
  // something or refuse while NAMING itself.
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

  let mut hydratable = 0;
  let mut refused = Vec::new();
  for entity in forms {
    let form = entity.form();
    let fx = fixture();
    let mut facade = fx.facade();
    match facade.hydrate(&at(entity)) {
      Ok(_) => hydratable += 1,
      Err(FacadeError::NotHydratable { form: named, .. }) => {
        assert_eq!(named, form, "a refusal must name the form it refused");
        refused.push(form);
      }
      Err(other) => panic!("{form} must be hydratable or NotHydratable, got {other:?}"),
    }
  }

  assert_eq!(
    hydratable + refused.len(),
    11,
    "the partition must cover every form: {hydratable} hydratable, refused {refused:?}"
  );
  assert_eq!(
    refused,
    // **`issue` MOVED FROM HYDRATABLE TO REFUSED ON 2026-08-20, AND THIS LINE
    // IS WHY ANYONE SAW IT.** hv ruled issues canon-and-store only, so
    // `Address::artefact` answers `None` for one and `hydrate` refuses at the
    // door. The declared list is the whole point of the assertion -- a set
    // computed from the code would have absorbed the move in silence.
    //
    // It was never HYDRATABLE in any useful sense: its realisation home
    // resolved through `issues_dir()` to `intent/.canon/issues/`, CANON, and
    // it returned `Ok` over zero files while pinning `ISSUE:` into the live
    // manifest. So this is a form arriving in the bucket it always belonged
    // in, not a capability being withdrawn.
    vec!["threads", "issue", "node", "node-inbox", "event"],
    "the refused set is declared, so a form moving between buckets is visible"
  );
}
