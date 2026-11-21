//! ST0096 WP-05, on intent-vc's and intent-cc's rulings that retired the step
//! which kept both sides' work packages (issue 0709): **A MERGE WHERE TAKING A
//! SIDE WOULD DROP A WORK PACKAGE IS REFUSED, NEVER OFFERED AS A SIDE TO
//! TAKE** (issue 0555). In the release's format that is a package file both
//! sides added under one id, which only an ordinal two branches made before
//! the upgrade can share; in 3.4.0's it is a thread both sides added packages
//! to, whose packages are inside its own file. A criterion or test both sides
//! added stays a side to take, as it was in 3.4.0's one file.

use crate::common::{Fixture, sample_thread};
use intentsvcs::gitstate::Unmerged;
use intentsvcs::model::Thread;
use intentsvcs::plan::{both_sides_added_packages, classify};
use intentsvcs::project::{canon_ac_rel, canon_wp_rel};

/// An unmerged path both sides hold: added by both when `base` is false,
/// changed by both when it is true.
fn both_sides(path: &str, base: bool) -> Unmerged {
  Unmerged {
    path: path.to_string(),
    base,
    ours: true,
    theirs: true,
  }
}

/// `thread` with one more package than it has, as one side of a merge adds it.
fn with_a_package_added(thread: &Thread, title: &str) -> Thread {
  let mut side = thread.clone();
  let mut added = side
    .wps
    .last()
    .expect("the sample thread has packages")
    .clone();
  // The first ordinal the thread does not use, as both sides of a merge in
  // 3.4.0's format each took the same next number.
  added.seq = (1..)
    .map(|n| intentsvcs::id::WpId::from_ordinal(n).expect("an ordinal package"))
    .find(|seq| !thread.wps.iter().any(|held| held.seq == *seq))
    .expect("an unused ordinal");
  added.title = title.to_string();
  side.wps.push(added);
  side
}

#[test]
fn a_package_file_both_sides_added_is_its_own_conflict_and_a_criterion_is_a_side_to_take() {
  let fixture = Fixture::new();
  let project = fixture.project();
  let thread = intentsvcs::id::ThreadId::from_ordinal(7)
    .expect("an ordinal thread")
    .stored();
  let package = intentsvcs::id::WpId::from_ordinal(2)
    .expect("an ordinal package")
    .stored();
  let in_intent = |rel: String| project.relative(&project.intent_dir().join(rel));
  let added_package = in_intent(canon_wp_rel(&thread, &package));
  let added_criterion = in_intent(canon_ac_rel(&thread, "AC-01.1"));

  let conflicts = classify(
    &project,
    &[
      both_sides(&added_package, false),
      both_sides(&added_criterion, false),
    ],
  );
  assert_eq!(
    conflicts.packages_added_twice,
    vec![intentsvcs::plan::PackageAddedTwice {
      thread: thread.clone(),
      package: package.clone(),
      path: added_package.clone(),
    }],
    "a package file both sides added is refused, never a side to take"
  );
  assert_eq!(
    conflicts.take_sides,
    vec![added_criterion],
    "a criterion both sides added stays a side to take"
  );
  assert!(
    conflicts.canon_is_unmerged(),
    "canon still holds the conflict"
  );

  let changed = classify(&project, &[both_sides(&added_package, true)]);
  assert!(
    changed.packages_added_twice.is_empty(),
    "one package both sides changed is one record, a side to take"
  );
  assert_eq!(changed.take_sides, vec![added_package]);
}

#[test]
fn a_thread_both_sides_added_a_package_to_answers_yes_and_one_side_alone_does_not() {
  let base = sample_thread("ST0007");
  let ours = with_a_package_added(&base, "Bob's");
  let theirs = with_a_package_added(&base, "Alice's");
  assert!(
    both_sides_added_packages(&base, &ours, &theirs),
    "both sides added a package to the thread's one file"
  );
  assert!(
    !both_sides_added_packages(&base, &ours, &base),
    "only this side added one, so the merge drops nothing"
  );
  assert!(
    !both_sides_added_packages(&base, &base, &theirs),
    "only the pulled side added one, so the merge drops nothing"
  );
}
