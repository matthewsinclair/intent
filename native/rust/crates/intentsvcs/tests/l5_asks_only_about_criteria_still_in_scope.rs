//! Issue 0607, vc's ruling of 2026-10-22: **contract L5 asks only about
//! criteria still in scope.**
//!
//! L5 refuses a non-test AT covering a test-backed criterion, because a non-test
//! AT is never green and the criterion could then never be satisfied. A
//! withdrawn or descoped criterion is never asked whether it is satisfied, so
//! the trap cannot arise for one; refusing anyway left a withdrawn criterion's
//! test rows no honest way out of the green state once the feature and its
//! test file were removed. That was ST0075's seven Console rows, after 0607
//! replaced Intent.app's Console with the macOS Console app.
//!
//! The arms drive the verbs that disposition runs, `ac withdraw` or `ac descope`
//! and then `at edit --kind non-test`, through the facade. The in-scope arm is
//! the control: the same re-kind over a criterion still asked for stays refused,
//! so the guard narrows L5 rather than switching it off.

use crate::common::{Fixture, sample_thread};
use intentsvcs::model::AtKind;

/// The test row over the fixture's test-backed `AC-03.1`, green and citing a
/// file the fixture makes.
const COVER: &str = "AT-03.1";

/// **A withdrawn criterion's test row can be re-kinded non-test**: the
/// disposition 0607 needs for ST0075, refused before the guard.
#[test]
fn a_test_row_over_a_withdrawn_criterion_can_be_rekinded_non_test() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();
  facade
    .ac_withdraw("ST0001", "AC-03.1", "the feature was removed", Some("vc"))
    .expect("withdraw");

  facade
    .at_edit(
      "ST0001",
      COVER,
      None,
      None,
      None,
      None,
      Some(AtKind::NonTest),
    )
    .expect("a non-test AT over a withdrawn criterion makes nothing unclosable");
}

/// **A descoped criterion's test row can be re-kinded non-test too**: descoped
/// is the other state `AcState::in_scope` answers no for.
#[test]
fn a_test_row_over_a_descoped_criterion_can_be_rekinded_non_test() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  fx.write_thread(&sample_thread("ST0002"));
  let mut facade = fx.facade();
  facade
    .ac_descope("ST0001", "AC-03.1", "ST0002", Some("vc"), Some("moved"))
    .expect("descope");

  facade
    .at_edit(
      "ST0001",
      COVER,
      None,
      None,
      None,
      None,
      Some(AtKind::NonTest),
    )
    .expect("a non-test AT over a descoped criterion makes nothing unclosable");
}

/// **THE CONTROL: over a criterion still in scope, the same re-kind stays
/// refused**, naming the criterion it would make unclosable, and canon is
/// unchanged by the refusal.
#[test]
fn the_same_rekind_over_an_in_scope_criterion_stays_refused() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();
  let before = fx.read_canon("ST0001");

  let err = facade
    .at_edit(
      "ST0001",
      COVER,
      None,
      None,
      None,
      None,
      Some(AtKind::NonTest),
    )
    .expect_err("a non-test AT over an in-scope test-backed criterion must still be refused");

  let said = format!("{err:?}");
  assert!(
    said.contains("AC-03.1") && said.contains("never green"),
    "the refusal must be L5's, naming the criterion it would make unclosable: {said}"
  );
  assert_eq!(
    before,
    fx.read_canon("ST0001"),
    "the refusal wrote to canon"
  );
}
