//! Issues 0324 and 0337: **the close and verdict verbs enforce what their docs
//! promise.** `at green` went straight from `to-write`, `at na` landed on a
//! test row and `at green` on a non-test one, `st done` closed a thread with a
//! work package still open, and a close on an unwritten objective said nothing.

use crate::common::{Fixture, sample_thread};
use intentsvcs::facade::Note;
use intentsvcs::model::{AtKind, AtStatus};

fn status_of(facade: &intentsvcs::facade::Facade, at: &str) -> AtStatus {
  facade
    .st_show("ST0001")
    .expect("thread")
    .tests
    .iter()
    .find(|t| t.id == at)
    .map(|t| t.status)
    .unwrap_or_else(|| panic!("the fixture has no {at}"))
}

/// 0337 (a): a new test row at `to-write` cannot go green until it has been red.
#[test]
fn green_is_reachable_only_from_red() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();
  facade
    .at_new(
      "ST0001",
      "AT-09.1",
      AtKind::Test,
      None,
      None,
      vec!["AC-03.1".to_string()],
      None,
    )
    .expect("a to-write test row");

  let said = format!(
    "{:?}",
    facade
      .at_set("ST0001", "AT-09.1", AtStatus::Green, None)
      .expect_err("green straight from to-write is refused")
  );
  assert!(
    said.contains("IllegalTransition"),
    "refused for the wrong reason: {said}"
  );
  assert_eq!(
    status_of(&facade, "AT-09.1"),
    AtStatus::ToWrite,
    "a refused verdict moved the row"
  );

  facade
    .at_set("ST0001", "AT-09.1", AtStatus::Red, None)
    .expect("red from to-write");
  facade
    .at_set("ST0001", "AT-09.1", AtStatus::Green, None)
    .expect("green from red");
  assert_eq!(status_of(&facade, "AT-09.1"), AtStatus::Green);
}

/// 0337 (b): `n/a` belongs to a non-test row and `red`/`green` to a test row.
#[test]
fn a_verdict_must_fit_the_rows_kind() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();

  let green_on_prose = format!(
    "{:?}",
    facade
      .at_set("ST0001", "AT-03.2", AtStatus::Green, None)
      .expect_err("green on a non-test row is refused")
  );
  assert!(
    green_on_prose.contains("VerdictWrongForKind"),
    "{green_on_prose}"
  );
  assert_eq!(status_of(&facade, "AT-03.2"), AtStatus::Na);

  let na_on_test = format!(
    "{:?}",
    facade
      .at_set("ST0001", "AT-03.1", AtStatus::Na, None)
      .expect_err("n/a on a test row is refused")
  );
  assert!(na_on_test.contains("VerdictWrongForKind"), "{na_on_test}");
  assert_eq!(status_of(&facade, "AT-03.1"), AtStatus::Green);
}

/// 0324: `st done` refuses while a work package is not settled, naming it.
#[test]
fn st_done_refuses_while_a_work_package_is_open() {
  let fx = Fixture::new();
  // Exempt, so the gate passes and the packages are what stands between it and
  // the close: work package 3 is still WIP in the fixture.
  let mut thread = sample_thread("ST0001");
  thread.acceptance = Some(intentsvcs::model::AcceptanceMode::Exempt);
  fx.write_thread(&thread);
  let mut facade = fx.facade();

  let said = format!(
    "{:?}",
    facade
      .st_done("ST0001")
      .expect_err("a thread with work package 3 still WIP must not close")
  );
  assert!(
    said.contains("OpenWorkPackages"),
    "refused for the wrong reason: {said}"
  );
  assert!(
    said.contains("ST0001/03"),
    "the refusal must name the open package: {said}"
  );
}

/// 0337 (c): a work package closed with its objective unwritten carries a warning.
#[test]
fn a_close_on_an_unwritten_objective_warns() {
  let fx = Fixture::new();
  let mut thread = sample_thread("ST0001");
  thread
    .wps
    .iter_mut()
    .find(|w| w.seq == 3)
    .expect("the fixture has work package 3")
    .objective = String::new();
  fx.write_thread(&thread);
  let mut facade = fx.facade();

  let outcome = facade
    .wp_done("ST0001", 3)
    .expect("wp done on a WIP package");
  let warned = outcome
    .notes()
    .iter()
    .any(|n| matches!(n, Note::UnwrittenObjective(unit) if unit.ends_with("ST0001/wp/03")));
  assert!(
    warned,
    "no unwritten-objective warning: {:?}",
    outcome.notes()
  );
}
