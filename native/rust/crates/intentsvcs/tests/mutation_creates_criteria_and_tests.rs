//! AT-08.6 / AC-08.6 and AT-08.7 / AC-08.7: **a criterion and an acceptance
//! test are CREATABLE THROUGH THE MUTATION SURFACE, not only by hand-editing
//! canon.**
//!
//! # What this file must NOT be, and the trap is a live one
//!
//! **`AT-08.4` IS ALREADY GREEN AND ALREADY PROVES THAT `Facade::put` CREATES A
//! ROW AT AN ADDRESS THAT DID NOT EXIST, IDEMPOTENTLY**
//! (`mutation_create_splits_two_ways.rs`). So a file here that exercised `put`
//! would pass on the day it was written, report two more green rows, and leave
//! the claim `AC-08.6` and `AC-08.7` actually make completely untested. That is
//! this estate's recurring defect in its purest form -- a scope claim made in
//! prose beside code that does not implement it -- and it would be committed by
//! the very rows minted to close the gap.
//!
//! The gap those two criteria name is **the VERB**. Driven 2026-08-25 and
//! recorded in `AC-08.6`'s own text: `ac --help` lists nine subcommands and
//! `at --help` five, and **not one of the fourteen creates anything** -- every
//! arm is a transition on a row that already exists. The route was a hand-edit
//! of `.canon/st/<ID>.json` plus `sync --to-store`, which is how `AC-08.6`
//! ITSELF reached canon.
//!
//! So this file tests `Facade::ac_new` and `Facade::at_new` -- and, for
//! `AC-08.7`, the half that makes a create more than a `push`: **the created
//! row is held to the grammar `at lint` enforces on every other row, BEFORE the
//! write lands.**
//!
//! # The falsifiers, taken from the criteria verbatim
//!
//! `AC-08.6`: *a criterion that can be brought into existence by editing canon
//! but not through the addressed surface, or a create that is not idempotent
//! under repeat.*
//!
//! `AC-08.7`: *an AT that can be brought into existence by editing canon but
//! not through the addressed surface, or a created row that bypasses the
//! grammar `at lint` enforces on every other row.*
//!
//! **THE FIRST LIMB OF EACH IS A REACHABILITY CLAIM AND IT IS NOT TESTABLE BY
//! LOOKING AT THE CREATED ROW.** Asserting that `ac_new` sets the four fields
//! `Criterion` has today passes forever and says nothing: add a fifth field the
//! surface cannot set, and such a test stays green while the claim becomes
//! false. So reachability is driven as a DIFFERENTIAL against the route the
//! criterion names -- the same entity built both ways, in two fixtures, with
//! the canon FILES compared byte for byte. A field the hand-edit can express
//! and the surface cannot makes the bytes differ, which is the only shape that
//! catches it.
//!
//! # A note on the fixture, because it is doing real work here
//!
//! `sample_thread` carries `AT-03.1` as `green` citing
//! `crates/intentsvcs/tests/ingest_refusal.rs`, a path that does not exist
//! inside a temp fixture. **The fixture therefore ALREADY carries L2 findings
//! before this file writes anything**, which is what makes
//! `at_new_does_not_refuse_on_a_finding_that_is_not_about_this_row` a real
//! test rather than a hypothetical: without the narrowing, no create would
//! ever succeed on this thread, and a verb that refuses on somebody else's
//! defect is one nobody can use.

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::contract;
use intentsvcs::facade::FacadeError;
use intentsvcs::model::{AcKind, AcState, AcceptanceTest, AtKind, AtStatus, Criterion};

const NEW_AC: &str = "AC-09.1";
const NEW_AT: &str = "AT-09.1";

// ---------------------------------------------------------------------------
// AC-08.6 -- a criterion is creatable through the surface
// ---------------------------------------------------------------------------

/// The plain claim: the row is absent, the verb is driven, the row exists.
///
/// The absence is ASSERTED rather than assumed -- if the fixture ever gained an
/// `AC-09.1`, this file would be testing an update while claiming a create.
#[test]
fn ac_new_creates_a_criterion_that_did_not_exist() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();

  assert!(
    !facade.canon().threads[0]
      .criteria
      .iter()
      .any(|c| c.id == NEW_AC),
    "precondition: {NEW_AC} must be ABSENT, or this tests an update and not a create"
  );

  facade
    .ac_new(
      "ST0001",
      NEW_AC,
      "a criterion minted through the surface",
      AcKind::Test,
    )
    .expect("the verb creates it");

  let row = facade.canon().threads[0]
    .criteria
    .iter()
    .find(|c| c.id == NEW_AC)
    .expect("the row now exists")
    .clone();
  assert_eq!(row.text, "a criterion minted through the surface");
}

/// **THE REPEAT IS REFUSED -- the criterion's second falsifier, INVERTED BY A
/// LATER RULING, and the count assertion is kept because it still bites.**
///
/// This test read `ac_new_is_idempotent_under_repeat` and asserted that a
/// second create was accepted. **hv ruled on 2026-08-28 (issue 0131) that a
/// verb named `add`/`new` must FAIL on an existing key rather than replace it**,
/// which supersedes the idempotent reading ic ratified on 2026-08-26 -- a later
/// first-hand ruling from hv on the same subject wins. So the falsifier is
/// turned over rather than deleted: what was "the repeat is accepted" is now
/// "the repeat is refused, and the stored row is untouched".
///
/// **The COUNT assertion survives the inversion unchanged, and that is the
/// reason to keep this test rather than to write a new one.** Under
/// idempotence it caught a repeat that duplicated the row; under refusal it
/// catches a refusal that appended one anyway. The same arithmetic falsifies
/// both contracts, which is what a good falsifier looks like.
#[test]
fn ac_new_refuses_the_repeat_and_leaves_one_row() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();

  facade
    .ac_new("ST0001", NEW_AC, "minted once", AcKind::Test)
    .expect("creates");
  let err = facade
    .ac_new("ST0001", NEW_AC, "minted twice", AcKind::Test)
    .expect_err("the repeat is refused rather than accepted");
  assert!(
    matches!(&err, FacadeError::CriterionExists { ac, .. } if ac == NEW_AC),
    "the refusal must name the taken key: {err}"
  );
  assert_eq!(
    facade.canon().threads[0]
      .criteria
      .iter()
      .find(|c| c.id == NEW_AC)
      .expect("still there")
      .text,
    "minted once",
    "the refused repeat rewrote the row it refused to create"
  );

  let n = facade.canon().threads[0]
    .criteria
    .iter()
    .filter(|c| c.id == NEW_AC)
    .count();
  assert_eq!(
    n, 1,
    "the refused repeat appended a second row instead of writing nothing"
  );
}

/// **THE REACHABILITY FALSIFIER, DRIVEN AS A DIFFERENTIAL.**
///
/// Two fixtures, the same criterion, built by the two routes the criterion
/// names: a hand-edit of canon, and the addressed surface. The canon files are
/// then compared BYTE FOR BYTE.
///
/// This is the only shape that survives a future field. Asserting that
/// `ac_new` sets the fields `Criterion` has today would pass forever -- add a
/// fifth the surface cannot reach and the assertion stays green while
/// `AC-08.6` becomes false. Here the hand-edit expresses it, the surface does
/// not, and the bytes diverge.
#[test]
fn ac_new_reaches_what_a_canon_hand_edit_reaches() {
  let minted = Criterion {
    id: NEW_AC.to_string(),
    text: "the same criterion, both ways".to_string(),
    kind: AcKind::Test,
    state: AcState::Computed,
  };

  // Route 1: the hand-edit the criterion says was the only route.
  let by_hand = Fixture::new();
  let mut thread = sample_thread("ST0001");
  thread.criteria.push(minted.clone());
  thread.criteria.sort_by(|a, b| a.id.cmp(&b.id));
  by_hand.write_thread(&thread);

  // Route 2: the addressed surface.
  let by_surface = Fixture::new();
  by_surface.write_thread(&sample_thread("ST0001"));
  let mut facade = by_surface.facade();
  facade
    .ac_new("ST0001", NEW_AC, &minted.text, minted.kind)
    .expect("the surface creates it");

  assert_eq!(
    by_hand.read_canon("ST0001"),
    by_surface.read_canon("ST0001"),
    "a criterion reachable by hand-editing canon is not reachable through the addressed surface -- AC-08.6's first falsifier"
  );
}

/// **The create HONOURS the kind/state invariant; it does not restate it.**
///
/// `ac_kind_state_invariant.rs` already holds `AcState::permitted_for` and the
/// generated schema to each other over every declared variant. That file owns
/// the rule. What is untested there, and is this file's business, is that the
/// CREATE PATH lands on a legal pair at all -- `kind` and `state` are not
/// independent, so a create taking only `kind` has to choose the state, and a
/// wrong choice would mint nonsense through the front door of the very surface
/// this criterion exists to open.
#[test]
fn ac_new_honours_the_kind_state_invariant() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();

  facade
    .ac_new("ST0001", "AC-09.1", "test-backed", AcKind::Test)
    .expect("creates");
  facade
    .ac_new("ST0001", "AC-09.2", "not test-backed", AcKind::NonTest)
    .expect("creates");

  let state_of = |facade: &intentsvcs::facade::Facade, id: &str| {
    facade.canon().threads[0]
      .criteria
      .iter()
      .find(|c| c.id == id)
      .unwrap_or_else(|| panic!("{id} exists"))
      .state
      .clone()
  };

  assert!(
    matches!(state_of(&facade, "AC-09.1"), AcState::Computed),
    "a test-backed criterion starts `computed` -- its satisfaction is DERIVED, so any other start claims a measurement nothing ran"
  );
  assert!(
    matches!(state_of(&facade, "AC-09.2"), AcState::Unsatisfied),
    "a non-test criterion starts `unsatisfied` -- `computed` would claim a derivation with nothing to derive from"
  );
}

// ---------------------------------------------------------------------------
// AC-08.7 -- an AT is creatable, AND is held to the grammar on the way in
// ---------------------------------------------------------------------------

/// The plain claim for the AT side.
#[test]
fn at_new_creates_an_acceptance_test_that_did_not_exist() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();

  assert!(
    !facade.canon().threads[0]
      .tests
      .iter()
      .any(|t| t.id == NEW_AT),
    "precondition: {NEW_AT} must be ABSENT"
  );

  facade
    .at_new(
      "ST0001",
      NEW_AT,
      AtKind::Test,
      None,
      None,
      vec!["AC-03.1".to_string()],
      AtStatus::ToWrite,
      None,
    )
    .expect("the verb creates it");

  let row = facade.canon().threads[0]
    .tests
    .iter()
    .find(|t| t.id == NEW_AT)
    .expect("the row now exists")
    .clone();
  assert_eq!(row.covers, vec!["AC-03.1".to_string()]);
  assert_eq!(row.status, AtStatus::ToWrite);
}

/// Refused under repeat, asserted on the count, inverted by the same hv ruling
/// and for the same reason as the criterion side.
#[test]
fn at_new_refuses_the_repeat_and_leaves_one_row() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();

  let drive = |facade: &mut intentsvcs::facade::Facade| {
    facade.at_new(
      "ST0001",
      NEW_AT,
      AtKind::Test,
      None,
      None,
      vec!["AC-03.1".to_string()],
      AtStatus::ToWrite,
      None,
    )
  };

  drive(&mut facade).expect("creates");
  let err = drive(&mut facade).expect_err("the repeat is refused rather than accepted");
  assert!(
    matches!(&err, FacadeError::TestExists { at, .. } if at == NEW_AT),
    "the refusal must name the taken key: {err}"
  );

  let n = facade.canon().threads[0]
    .tests
    .iter()
    .filter(|t| t.id == NEW_AT)
    .count();
  assert_eq!(
    n, 1,
    "the refused repeat appended a second row instead of writing nothing"
  );
}

/// The reachability differential for the AT side. Same shape, same reason.
#[test]
fn at_new_reaches_what_a_canon_hand_edit_reaches() {
  let minted = AcceptanceTest {
    id: NEW_AT.to_string(),
    kind: AtKind::Test,
    file: None,
    prose: None,
    covers: vec!["AC-03.1".to_string()],
    status: AtStatus::ToWrite,
    note: None,
    legacy: None,
  };

  let by_hand = Fixture::new();
  let mut thread = sample_thread("ST0001");
  thread.tests.push(minted.clone());
  thread.tests.sort_by(|a, b| a.id.cmp(&b.id));
  by_hand.write_thread(&thread);

  let by_surface = Fixture::new();
  by_surface.write_thread(&sample_thread("ST0001"));
  let mut facade = by_surface.facade();
  facade
    .at_new(
      "ST0001",
      NEW_AT,
      minted.kind,
      minted.file.clone(),
      minted.prose.clone(),
      minted.covers.clone(),
      minted.status,
      None,
    )
    .expect("the surface creates it");

  assert_eq!(
    by_hand.read_canon("ST0001"),
    by_surface.read_canon("ST0001"),
    "an AT reachable by hand-editing canon is not reachable through the addressed surface -- AC-08.7's first falsifier"
  );
}

/// **L4: a `covers` id that matches no criterion in this contract is REFUSED at
/// the create.**
///
/// This is the second falsifier -- *a created row that bypasses the grammar
/// `at lint` enforces on every other row.* L4 exists because satisfaction is
/// computed FORWARDS, so a `covers` id matching nothing is silently ignored by
/// that walk and the AT looks like coverage it is not. A create that admitted
/// one would mint exactly that lie.
#[test]
fn at_new_refuses_a_row_whose_covers_matches_no_criterion() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();

  let err = facade
    .at_new(
      "ST0001",
      NEW_AT,
      AtKind::Test,
      None,
      None,
      vec!["AC-99.9".to_string()],
      AtStatus::ToWrite,
      None,
    )
    .expect_err("a row covering a criterion that does not exist must be refused");

  let said = format!("{err:?}");
  assert!(
    said.contains("AC-99.9"),
    "the refusal must NAME the offending id or the operator cannot act on it: {said}"
  );
}

/// **L5: a non-test AT covering a test-backed criterion is REFUSED at the
/// create.**
///
/// The trap L5 exists for: a non-test AT is `n-a` by definition and `n-a` is
/// never green, so a test-backed AC covered by one can NEVER be satisfied. The
/// contract becomes unclosable and the only symptom is a gate that will not
/// move. Minting one through a create would plant that with no diagnosis
/// attached.
#[test]
fn at_new_refuses_the_unclosable_pairing() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();

  // AC-03.1 is test-backed in the fixture.
  let err = facade
    .at_new(
      "ST0001",
      NEW_AT,
      AtKind::NonTest,
      None,
      Some("eyeballed".to_string()),
      vec!["AC-03.1".to_string()],
      AtStatus::Na,
      None,
    )
    .expect_err("a non-test AT covering a test-backed AC must be refused");

  let said = format!("{err:?}");
  assert!(
    said.contains("AC-03.1"),
    "the refusal must name the criterion it would make unclosable: {said}"
  );
}

/// **THE REFUSAL HAPPENS BEFORE THE WRITE, AND CANON IS BYTE-IDENTICAL AFTER
/// IT.**
///
/// Lint-after-write leaves the bad row in canon on refusal and asks the
/// operator to clean up after a verb that already told them no -- the shape
/// `issues hydrate` was retired for. A refusal that has already written is not
/// a refusal, it is a write plus an apology, and nothing downstream can tell
/// the difference by reading canon.
#[test]
fn at_new_refuses_before_it_writes() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();

  let before = fx.read_canon("ST0001");

  facade
    .at_new(
      "ST0001",
      NEW_AT,
      AtKind::Test,
      None,
      None,
      vec!["AC-99.9".to_string()],
      AtStatus::ToWrite,
      None,
    )
    .expect_err("refused");

  assert_eq!(
    before,
    fx.read_canon("ST0001"),
    "the refusal wrote to canon before deciding -- a rejected row is sitting in the file"
  );
}

/// **THE NARROWING: a create is NOT blocked by a finding that is not about the
/// row being created -- and the fixture proves the finding is really there.**
///
/// Inherited breakage is `at lint`'s to report, not this verb's to block on. A
/// verb that refused on somebody else's defect would be uncallable on any real
/// thread, and this fixture is a real thread in exactly that sense: `AT-03.1`
/// is `green` citing a path that does not exist inside a temp directory, so L2
/// fires before this test writes anything.
///
/// **THE PRE-EXISTING FINDING IS ASSERTED, NOT ASSUMED.** If the fixture were
/// ever cleaned up so that no inherited finding existed, this test would still
/// pass while testing nothing at all -- the blind-instrument shape. The first
/// assertion is what stops that.
#[test]
fn at_new_does_not_refuse_on_a_finding_that_is_not_about_this_row() {
  let fx = Fixture::new();

  // **THE INHERITED FINDING IS PLANTED, NOT BORROWED, AND THE FIRST DRAFT OF
  // THIS TEST GOT THAT WRONG.** It assumed `sample_thread`'s `AT-03.1` -- green,
  // citing a path absent from a temp dir -- would raise L2 and supply the
  // finding for free. The control fired on the first run and reported
  // `findings: []`: the fixture carries none. **Had the control not been there,
  // this test would have passed while proving nothing** -- a create that
  // succeeds on a thread with NO findings says exactly nothing about whether
  // the verb narrows, because there was never anything for it to refuse on.
  //
  // So the instance is SYNTHETIC, which is this estate's standing ruling for
  // red-first arms (`dispatch_ssot.rs`): an instrument that borrows a live
  // defect has made the defect a fixture, and the estate is then not free to
  // fix it. `AT-03.4` below covers a criterion that does not exist in this
  // contract, which is an L4 finding by construction and cannot stop being one.
  let mut thread = sample_thread("ST0001");
  thread.tests.push(AcceptanceTest {
    id: "AT-03.4".to_string(),
    kind: AtKind::Test,
    file: None,
    prose: None,
    covers: vec!["AC-77.7".to_string()],
    status: AtStatus::ToWrite,
    note: None,
    legacy: None,
  });
  thread.tests.sort_by(|a, b| a.id.cmp(&b.id));
  fx.write_thread(&thread);
  let mut facade = fx.facade();

  let loaded = facade.st_show("ST0001").expect("the thread").clone();
  let report = contract::contract_report(&loaded, None, &contract::RepoFiles(fx.root()));
  let inherited: Vec<&String> = report
    .findings
    .iter()
    .filter(|f| !f.contains(NEW_AT))
    .collect();
  assert!(
    !inherited.is_empty(),
    "positive control: the planted row must raise a finding about a row OTHER than the one being \
     created, or this test cannot tell a narrowed refusal from no refusal at all\n  findings: {:#?}",
    report.findings
  );

  // The row being created is clean. If `at_new` blocked on the whole report
  // rather than on findings naming its own row, this would refuse -- and a verb
  // that refuses on somebody else's defect is one nobody can use, because every
  // real thread carries somebody else's defect eventually.
  facade
    .at_new(
      "ST0001",
      NEW_AT,
      AtKind::Test,
      None,
      None,
      vec!["AC-03.1".to_string()],
      AtStatus::ToWrite,
      None,
    )
    .expect("a clean row is creatable on a thread that carries somebody else's finding");

  // **AND THE INHERITED FINDING IS STILL THERE AFTERWARDS.** Without this, a
  // build that silently REPAIRED the planted row would pass the assertion above
  // for the wrong reason -- the create would have succeeded because there was
  // no longer anything to refuse on, which is the vacuity this test was just
  // rewritten to escape.
  let after = facade.st_show("ST0001").expect("the thread").clone();
  let still = contract::contract_report(&after, None, &contract::RepoFiles(fx.root()));
  assert!(
    still.findings.iter().any(|f| f.contains("AT-03.4")),
    "the create silently repaired somebody else's row -- inherited breakage is `at lint`'s to \
     report, not this verb's to fix\n  findings: {:#?}",
    still.findings
  );
}
