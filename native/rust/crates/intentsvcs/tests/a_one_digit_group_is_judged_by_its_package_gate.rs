//! Issue 0712: **a criterion written with a one-digit group belongs to its
//! package's gate.** `AC-3.1` is a conforming label (hv's ruling, 182 such rows
//! across three estates), and its group names WP-03 as `AC-03.1`'s does. The
//! gate used to compare the group as text, `3` against `03`, so a package whose
//! criteria were written that way had none in scope and closed unjudged.
//! `id::group_of` is now the one reader, and these arms hold it to that.

use crate::common::{Fixture, sample_thread, tid, wp};
use intentsvcs::facade::FacadeError;
use intentsvcs::id::{Group, group_of};
use intentsvcs::model::{AcKind, AcState, Criterion, Thread};

/// The package every arm closes: the sample's one package still in progress.
const PACKAGE: u32 = 3;

/// The sample, whose in-progress package has one criterion, written with a
/// one-digit group, in `state`, and nothing else in its contract.
fn thread_with_one_digit_criterion(state: AcState) -> Thread {
  let mut thread = sample_thread("ST0001");
  thread.criteria = vec![Criterion {
    id: format!("AC-{PACKAGE}.1"),
    text: "it holds".to_string(),
    kind: AcKind::NonTest,
    state,
  }];
  thread.tests = vec![];
  thread
}

#[test]
fn a_one_digit_group_and_a_two_digit_group_name_one_package() {
  let package = Group::Package(wp(PACKAGE));
  assert_eq!(group_of(&format!("AC-{PACKAGE}.1")), package);
  assert_eq!(group_of(&format!("AC-{PACKAGE:02}.1")), package);
}

#[test]
fn wp_done_is_blocked_by_an_unsatisfied_criterion_with_a_one_digit_group() {
  let fx = Fixture::new();
  fx.write_thread(&thread_with_one_digit_criterion(AcState::Unsatisfied {
    note: None,
  }));
  let mut facade = fx.facade();
  let refused = facade.wp_done(&tid("ST0001"), wp(PACKAGE));
  assert!(
    matches!(refused, Err(FacadeError::GateBlocked { .. })),
    "the gate judges AC-{PACKAGE}.1 as WP-{PACKAGE:02}'s own and refuses the close: {refused:?}"
  );
}

/// The control: the same package with that criterion satisfied closes, so the
/// refusal above is the criterion's, not something else in the fixture's.
#[test]
fn wp_done_closes_the_package_once_that_criterion_is_satisfied() {
  let fx = Fixture::new();
  fx.write_thread(&thread_with_one_digit_criterion(AcState::Satisfied {
    evidence: "read and held".to_string(),
  }));
  let mut facade = fx.facade();
  let closed = facade.wp_done(&tid("ST0001"), wp(PACKAGE));
  assert!(closed.is_ok(), "a satisfied contract closes: {closed:?}");
}
