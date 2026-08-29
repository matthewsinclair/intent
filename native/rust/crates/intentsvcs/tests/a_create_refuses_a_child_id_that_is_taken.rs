//! **A CREATE REFUSES A CHILD ID THAT IS TAKEN, AND AN EDIT REACHES WHAT THE
//! CREATE NO LONGER WILL** -- hv, 2026-08-28, on issue 0131, sequenced to cc by
//! vc as ONE change.
//!
//! # Why the two halves are one commit
//!
//! Refusing alone strands the estate. `ac` shipped nine subcommands and every
//! one of them is a transition on STATE; not one could change a criterion's
//! text. So a refusal without `ac edit` would remove the only route to
//! rewording an AC and leave a hand-edit of `.canon/st/<ID>.json` as the
//! answer -- which is the route the create verbs were built to retire. ic
//! measured that gap after the refusal was first ruled, hv took the re-raise,
//! and the halves ship together.
//!
//! # The defect being closed is FABRICATION, not replacement
//!
//! `ac_new` derives the whole criterion from `(text, kind)`: it builds a fresh
//! row and never reads the one it lands on. **`AcState` has six variants and
//! four carry a payload** -- `Satisfied{evidence}`, `Descoped{to,by,reason}`,
//! `Withdrawn{reason,by}` and `Fiat(FiatRecord)` -- and a re-create reset every
//! one of them to the kind's entry value. So it destroyed a descope reason, a
//! withdrawal reason and a fiat close, not merely evidence, and `Fiat` landing
//! on 2026-08-28 widened that path with nothing in ST0066 having reason to look
//! here (vc, re-measured at HEAD by ic).
//!
//! **`states_are_driven_by_the_model_not_by_a_list` is the arm that keeps that
//! true**: it takes its population from `AcState`'s own variants rather than
//! from a list typed here, so the next payload-carrying state fails on the day
//! it is added instead of being silently uncovered.
//!
//! # What this does NOT close, asserted rather than promised
//!
//! **`Facade::put` at an entity address still replaces, deliberately.** It is a
//! `PUT`: replace-at-an-address is its contract, it is what the HTTP and
//! GraphQL faces expose, and `ac.put` is its declared op. hv ruled on verbs
//! named `add`/`new`, not on the addressed surface. So what dies is the
//! FABRICATING path, and `put_still_replaces_and_that_is_the_hole_this_leaves`
//! drives the survivor by value -- because "closed by construction" in a commit
//! message is what stops the next reader looking.
//!
//! **And the cross-facade window stays open.** Criteria and tests are CHILD
//! rows; `write_thread` replaces the child set wholesale, so there is no
//! per-child UNIQUE constraint for `store::Door::Create` to fire on and the
//! check reads canon loaded when the facade opened. Two facades opened before
//! either writes can still both add `AC-01.1`. Filed on vc's word rather than
//! left in a comment.

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::facade::{Facade, FacadeError, Outcome};
use intentsvcs::model::{AcKind, AcState, AtKind, AtStatus, Criterion, Thread};
use intentsvcs::remedy::Remedy;

fn criterion<'a>(facade: &'a Facade, ac: &str) -> &'a Criterion {
  facade.canon().threads[0]
    .criteria
    .iter()
    .find(|c| c.id == ac)
    .unwrap_or_else(|| panic!("{ac} is missing from the fixture"))
}

/// A fiat close, built once so the state arms below can carry the variant that
/// landed most recently and is therefore least likely to be covered.
fn fiat() -> AcState {
  AcState::Fiat(intentsvcs::model::FiatRecord {
    because: "the half it asserts is unobservable by unit test".to_string(),
    by: "hv".to_string(),
    at: "2026-08-28T18:30:00.000Z".to_string(),
    invoker: intentsvcs::model::Invoker {
      tty: true,
      env: "darwin/arm64".to_string(),
    },
    inherited_from: None,
  })
}

/// `sample_thread` with one criterion per payload-carrying state, so an arm
/// that walks them is walking real stored rows rather than a constructed one.
fn thread_with_every_payload_state() -> Thread {
  let mut t = sample_thread("ST0001");
  t.criteria.push(Criterion {
    id: "AC-03.6".to_string(),
    text: "closed on human authority with the requirement unmet".to_string(),
    kind: AcKind::NonTest,
    state: fiat(),
  });
  // `sample_thread` carries computed, satisfied, descoped and withdrawn, and
  // NOT unsatisfied -- which the arm's own coverage check found rather than
  // this comment predicting it.
  t.criteria.push(Criterion {
    id: "AC-03.3".to_string(),
    text: "a non-test criterion nobody has settled yet".to_string(),
    kind: AcKind::NonTest,
    state: AcState::Unsatisfied,
  });
  t
}

// ---------------------------------------------------------------------------
// The refusal
// ---------------------------------------------------------------------------

#[test]
fn ac_new_refuses_an_id_the_thread_already_has() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();

  let before = criterion(&facade, "AC-03.2").clone();
  let err = facade
    .ac_new("ST0001", "AC-03.2", "a reworded sentence", AcKind::NonTest)
    .expect_err("a create on a taken id must refuse");

  assert!(
    matches!(&err, FacadeError::CriterionExists { st, ac } if st == "ST0001" && ac == "AC-03.2"),
    "the refusal must name the key that is taken rather than a generic write failure: {err}"
  );
  assert_eq!(
    criterion(&facade, "AC-03.2"),
    &before,
    "the refused create still changed the stored row"
  );
}

/// **THE REMEDY IS PART OF THE REFUSAL, AND A REFUSAL THAT NAMES NO WAY FORWARD
/// IS THE WALL THIS CHANGE EXISTS TO AVOID BUILDING.** Asserted by value
/// because the whole argument for shipping the halves together is that the
/// operator is told where to go.
#[test]
fn the_refusal_names_the_verb_that_can_do_what_the_caller_wanted() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();

  let err = facade
    .ac_new("ST0001", "AC-03.2", "a reworded sentence", AcKind::NonTest)
    .expect_err("refused");
  let remedy = err.remedy();
  assert!(
    remedy.contains("ac edit ST0001 AC-03.2"),
    "the remedy must name the edit verb WITH the ids, or it is a pointer to a manual: {remedy}"
  );

  let err = facade
    .at_new(
      "ST0001",
      "AT-03.1",
      AtKind::Test,
      None,
      None,
      vec!["AC-03.1".to_string()],
      AtStatus::ToWrite,
      None,
    )
    .expect_err("refused");
  let remedy = err.remedy();
  assert!(
    remedy.contains("at edit ST0001 AT-03.1"),
    "the AT refusal must name its own edit verb: {remedy}"
  );
}

/// The ordinary create is not collateral. A door that refused everything would
/// pass every arm above and be useless.
#[test]
fn a_genuinely_free_id_is_still_created() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();

  facade
    .ac_new("ST0001", "AC-03.5", "a new requirement", AcKind::NonTest)
    .expect("a free id is created");

  assert_eq!(criterion(&facade, "AC-03.5").text, "a new requirement");
  assert!(matches!(
    criterion(&facade, "AC-03.5").state,
    AcState::Unsatisfied
  ));
}

// ---------------------------------------------------------------------------
// The edit
// ---------------------------------------------------------------------------

/// **THE HEADLINE: A REWORD LEAVES SATISFACTION WHERE IT WAS.** This is the
/// exact loss `ac new` caused -- repairing one sentence of a satisfied
/// criterion discarded the evidence that settled it.
#[test]
fn ac_edit_rewords_and_leaves_the_evidence_standing() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();

  let before = criterion(&facade, "AC-03.2").clone();
  assert!(
    matches!(before.state, AcState::Satisfied { .. }),
    "precondition: AC-03.2 must be SATISFIED, or this arm proves nothing"
  );

  facade
    .ac_edit("ST0001", "AC-03.2", "view rendering is byte-deterministic")
    .expect("rewording is what this verb is for");

  let after = criterion(&facade, "AC-03.2");
  assert_eq!(after.text, "view rendering is byte-deterministic");
  assert_eq!(
    after.state, before.state,
    "the reword reset the state, which is the whole defect wearing a different verb's name"
  );
  assert_eq!(after.kind, before.kind, "the reword rewrote the kind");
}

/// **THE POPULATION COMES FROM THE MODEL, NOT FROM A LIST TYPED HERE.** A
/// hand-written roster of states goes stale by being right: `Fiat` landed on
/// 2026-08-28 and every list written before it was silently one short. This
/// walks the criteria the fixture stores -- one per payload-carrying variant --
/// and fails if the fixture stops covering one.
#[test]
fn states_are_driven_by_the_model_not_by_a_list() {
  let fx = Fixture::new();
  fx.write_thread(&thread_with_every_payload_state());
  let mut facade = fx.facade();

  // Every variant `AcState` can hold, named by the model's own wire spelling so
  // a variant added upstream shows up here as an uncovered name rather than as
  // nothing at all.
  let stored: Vec<(String, AcState)> = facade.canon().threads[0]
    .criteria
    .iter()
    .map(|c| (c.id.clone(), c.state.clone()))
    .collect();
  // **`enum_str` PANICS HERE AND THAT IS THE TYPE TELLING THE TRUTH.**
  // `AcState` is `#[serde(tag = "is")]`, so it serialises to an OBJECT and not
  // to a string -- four of its six variants carry a payload, which is the whole
  // reason this arm exists. The tag is read out of the serialised form rather
  // than matched here, so a variant added upstream arrives as a name this loop
  // has never heard of instead of as a silently-missing case.
  let covered: std::collections::BTreeSet<String> = stored
    .iter()
    .map(|(_, s)| {
      serde_json::to_value(s)
        .ok()
        .and_then(|v| v.get("is").and_then(|t| t.as_str()).map(str::to_string))
        .expect("every AcState serialises with an `is` tag")
    })
    .collect();
  for expected in [
    "computed",
    "unsatisfied",
    "satisfied",
    "descoped",
    "withdrawn",
    "fiat",
  ] {
    assert!(
      covered.contains(expected),
      "the fixture carries no `{expected}` criterion, so this arm cannot speak for that state. \
       Covered: {covered:?}"
    );
  }

  for (id, was) in stored {
    let text = format!("reworded {id}");
    facade
      .ac_edit("ST0001", &id, &text)
      .unwrap_or_else(|e| panic!("{id} must be rewordable whatever state it is in: {e}"));
    let now = criterion(&facade, &id);
    assert_eq!(now.text, text, "{id}: the reword did not land");
    assert_eq!(
      now.state, was,
      "{id}: the reword moved the state -- a payload-carrying variant lost its payload"
    );
  }
}

#[test]
fn ac_edit_refuses_an_id_the_thread_does_not_have() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();

  let err = facade
    .ac_edit(
      "ST0001",
      "AC-09.9",
      "a sentence for a row that is not there",
    )
    .expect_err("an edit must not create");

  assert!(
    matches!(&err, FacadeError::NoSuchCriterion { ac, .. } if ac == "AC-09.9"),
    "an edit that CREATED on a missing id would be the create door wearing the other name: {err}"
  );
  assert!(
    !facade.canon().threads[0]
      .criteria
      .iter()
      .any(|c| c.id == "AC-09.9"),
    "the refused edit left a row behind"
  );
}

#[test]
fn ac_edit_on_identical_text_writes_nothing() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();

  let same = criterion(&facade, "AC-03.2").text.clone();
  let outcome = facade
    .ac_edit("ST0001", "AC-03.2", &same)
    .expect("a no-op is not an error");

  assert!(
    matches!(outcome, Outcome::AlreadyThere { .. }),
    "an unchanged reword must write no envelope, or history records a movement that did not \
     happen: {outcome:?}"
  );
}

/// **A CALL THAT NAMES NO FIELD IS REFUSED, NOT ANSWERED `unchanged`.** The
/// caller believes they changed something; exit 0 tells them they did.
#[test]
fn at_edit_refuses_a_call_that_names_nothing_to_change() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();

  let err = facade
    .at_edit("ST0001", "AT-03.1", None, None, None)
    .expect_err("an edit with no field named must refuse");

  assert!(
    matches!(&err, FacadeError::NothingToChange { .. }),
    "silence must not be reported as success: {err}"
  );
  let remedy = err.remedy();
  for flag in ["--file", "--prose", "--covers"] {
    assert!(
      remedy.contains(flag),
      "the refusal must name the fields it CAN move, or it raises a question it does not answer: \
       {remedy}"
    );
  }
}

/// **THE REPAIR PATH, AND IT IS THE ONE A STRICTER CHECK WOULD HAVE BROKEN.**
/// A contract check that refused ANY finding about the row would make a broken
/// row uneditable at exactly the moment someone needs to fix it. The rule is
/// "you may not make it worse".
///
/// **THE BROKEN STATE IS BUILT, NOT ASSUMED, AND THE FIRST DRAFT OF THIS ARM
/// ASSUMED IT.** `sample_thread`'s `AT-03.1` cites
/// `crates/intentsvcs/tests/ingest_refusal.rs`, and it is natural to expect
/// that path to be absent from a temp fixture -- a sibling test file's header
/// says in terms that the fixture "ALREADY carries L2 findings". It does not:
/// `Fixture::write_thread` CREATES every cited file and writes the row's
/// literal id into it, so the fixture satisfies L2 and L3 by construction and
/// `at_lint` returns nothing. The precondition below caught that; without it
/// this arm would have passed while proving nothing, because a row with no
/// finding trivially introduces none.
#[test]
fn at_edit_can_repair_a_row_that_already_carries_a_finding() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  // The real scenario, reproduced rather than simulated: the test file moved or
  // was deleted, so a green row now cites a path that is not there.
  std::fs::remove_file(fx.root().join("crates/intentsvcs/tests/ingest_refusal.rs"))
    .expect("the fixture created it, so it is there to remove");
  let mut facade = fx.facade();

  let report = facade.at_lint("ST0001").expect("lint runs");
  assert!(
    report.findings.iter().any(|f| f.contains("AT-03.1")),
    "precondition: AT-03.1 must ALREADY be in finding, or the diff rule is untested here. \
     Findings: {:?}",
    report.findings
  );

  facade
    .at_edit(
      "ST0001",
      "AT-03.1",
      None,
      None,
      Some(vec!["AC-03.2".to_string()]),
    )
    .expect("a row's inherited breakage must not make its other fields uneditable");

  assert_eq!(
    facade.canon().threads[0]
      .tests
      .iter()
      .find(|t| t.id == "AT-03.1")
      .expect("still there")
      .covers,
    vec!["AC-03.2".to_string()]
  );
}

/// And the repair itself completes: re-citing to a file that DOES exist and
/// carries the id clears the finding the row arrived with.
#[test]
fn a_re_cite_to_a_live_file_clears_the_finding_the_row_arrived_with() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  std::fs::remove_file(fx.root().join("crates/intentsvcs/tests/ingest_refusal.rs"))
    .expect("remove the cited file");
  fx.write_file(
    "crates/intentsvcs/tests/ingest_refusal_moved.rs",
    "// AT-03.1: the test, at its new home\n",
  );
  let mut facade = fx.facade();
  assert!(
    facade
      .at_lint("ST0001")
      .expect("lint runs")
      .findings
      .iter()
      .any(|f| f.contains("AT-03.1")),
    "precondition: the row starts broken"
  );

  facade
    .at_edit(
      "ST0001",
      "AT-03.1",
      Some("crates/intentsvcs/tests/ingest_refusal_moved.rs".to_string()),
      None,
      None,
    )
    .expect("re-citing to a live file is the repair");

  assert!(
    !facade
      .at_lint("ST0001")
      .expect("lint runs")
      .findings
      .iter()
      .any(|f| f.contains("AT-03.1")),
    "the re-cite left the row in finding, so the repair path does not actually repair"
  );
}

/// The other direction, which is what keeps the arm above from being a hole:
/// an edit that INTRODUCES a finding is still refused, with nothing written.
#[test]
fn at_edit_refuses_a_change_that_introduces_a_finding() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();

  let before = facade.canon().threads[0]
    .tests
    .iter()
    .find(|t| t.id == "AT-03.2")
    .expect("the fixture's non-test row")
    .clone();

  // `covers` naming a criterion the thread does not have is L4, and it is a
  // finding this edit brings rather than one the row arrived with.
  let err = facade
    .at_edit(
      "ST0001",
      "AT-03.2",
      None,
      None,
      Some(vec!["AC-77.7".to_string()]),
    )
    .expect_err("an edit that breaks the contract must refuse");

  assert!(
    format!("{err}").contains("contract"),
    "the refusal must say what it refused on: {err}"
  );
  assert_eq!(
    facade.canon().threads[0]
      .tests
      .iter()
      .find(|t| t.id == "AT-03.2")
      .expect("still there"),
    &before,
    "the refused edit wrote anyway -- a refusal that leaves a durable claim behind is the shape \
     `issues hydrate` was retired for"
  );
}

// ---------------------------------------------------------------------------
// The hole this leaves, driven rather than promised
// ---------------------------------------------------------------------------

/// **THE ADDRESSED SURFACE STILL REPLACES, AND THAT IS DELIBERATE.** `put` is a
/// `PUT`: it is the shape the HTTP and GraphQL faces expose and `ac.put` is its
/// declared op. hv ruled on verbs named `add`/`new`.
///
/// This is driven by VALUE so the limit is a fact in the suite rather than a
/// sentence in a commit message -- and so that anyone who later decides the
/// addressed surface should refuse too has a red test telling them what they
/// are changing, instead of discovering the intent from prose.
#[test]
fn put_still_replaces_and_that_is_the_hole_this_leaves() {
  use intentsvcs::address::{Address, Entity};

  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();

  assert!(
    matches!(
      criterion(&facade, "AC-03.2").state,
      AcState::Satisfied { .. }
    ),
    "precondition: the row starts satisfied"
  );

  let replacement = Criterion {
    id: "AC-03.2".to_string(),
    text: "written straight at the address".to_string(),
    kind: AcKind::NonTest,
    state: AcState::Unsatisfied,
  };
  facade
    .put(
      &Address {
        authority: None,
        entity: Entity::Ac {
          thread: "ST0001".to_string(),
          ac: "AC-03.2".to_string(),
        },
        format: None,
      },
      &serde_json::to_string(&replacement).expect("serialises"),
    )
    .expect("an addressed PUT replaces, which is its contract");

  assert!(
    matches!(criterion(&facade, "AC-03.2").state, AcState::Unsatisfied),
    "the addressed PUT stopped replacing. That may well be right -- but it is a change to the \
     HTTP and GraphQL faces' contract, and it is not what hv ruled on 2026-08-28, which was about \
     verbs named `add` and `new`."
  );
}
