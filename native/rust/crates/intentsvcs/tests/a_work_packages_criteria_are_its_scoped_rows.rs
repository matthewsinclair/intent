//! Issue 0310: **`wp show` lists the criteria a work package is judged on.**
//! `Facade::wp_criteria` gives the rows `ac list` composes, narrowed to the
//! package by the id group the close gate's `Scope::WorkPackage` reads, so the
//! listing and the verdict cannot disagree about what is in scope.

use crate::common::{Fixture, sample_thread};

#[test]
fn a_packages_criteria_are_the_rows_in_its_group() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let facade = fx.facade();

  let all: Vec<String> = facade
    .ac_list("ST0001")
    .expect("ac list")
    .into_iter()
    .map(|r| r.id)
    .collect();
  let in_three: Vec<String> = facade
    .wp_criteria("ST0001", 3)
    .expect("package 3 exists")
    .into_iter()
    .map(|r| r.id)
    .collect();
  let expected: Vec<String> = all
    .iter()
    .filter(|id| id.starts_with("AC-03."))
    .cloned()
    .collect();
  assert!(
    !expected.is_empty(),
    "the fixture has no AC-03.x rows, so the arm would prove nothing"
  );
  assert_eq!(
    in_three, expected,
    "package 3's criteria are exactly its group's rows, in ac list's order"
  );

  assert!(
    facade
      .wp_criteria("ST0001", 2)
      .expect("package 2 exists")
      .is_empty(),
    "a package with no criteria in its group lists none"
  );
}

#[test]
fn a_package_the_thread_does_not_have_is_refused() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let facade = fx.facade();
  let said = format!(
    "{:?}",
    facade
      .wp_criteria("ST0001", 99)
      .expect_err("package 99 does not exist")
  );
  assert!(said.contains("NoSuchWorkPackage"), "{said}");
}
