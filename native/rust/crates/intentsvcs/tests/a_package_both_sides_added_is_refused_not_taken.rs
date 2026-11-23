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

/// Issue If57165a: **A FILE UNDER A THREAD'S PACKAGES WHOSE NAME IS NOT A
/// PACKAGE ID IS NO PACKAGE.** `classify` took any name there as a package, so
/// an add/add on such a file was refused as a package both sides added, naming
/// a package that does not exist. Its name is now read as a package id, and a
/// name that is not one is a side to take, as any other path is (vc's ruling),
/// and ingest reports it after the merge, by its parse finding or as
/// `SchemaInvalid`, since its name and its record disagree.
#[test]
fn a_file_under_a_threads_packages_whose_name_is_not_a_package_id_is_a_side_to_take() {
  let fixture = Fixture::new();
  let project = fixture.project();
  let thread = intentsvcs::id::ThreadId::from_ordinal(7)
    .expect("an ordinal thread")
    .stored();
  let not_a_package = "not-a-package-id";
  assert!(
    intentsvcs::id::WpId::parse(not_a_package).is_err(),
    "the fixture needs a name that is not a package id"
  );
  let path = project.relative(
    &project
      .intent_dir()
      .join(canon_wp_rel(&thread, not_a_package)),
  );
  let conflicts = classify(&project, &[both_sides(&path, false)]);
  assert!(
    conflicts.packages_added_twice.is_empty(),
    "a name that is not a package id is no package: {:?}",
    conflicts.packages_added_twice
  );
  assert_eq!(conflicts.take_sides, vec![path], "it is a side to take");
}

/// Issue Ic884775: **ONE PACKAGE TWO BRANCHES UPGRADED APART BOTH WROTE IS A
/// SIDE TO TAKE, NOT TWO PACKAGES.** Each branch's move to the release's format
/// writes every package it holds as a file, so a package the branches share
/// is an add/add with no base whenever its state differs between them. `sync`
/// read that as two packages both sides added and told the person to
/// re-create one and cancel the other. The merge base held the package, so it
/// is one record both sides changed, and the plan offers it as a side to take.
#[test]
fn one_package_branches_upgraded_apart_both_wrote_is_a_side_to_take_not_two_packages() {
  use intentsvcs::facade::Facade;
  use intentsvcs::model::WpStatus;
  use intentsvcs::plan::Action;
  let fx = Fixture::ordinal();
  let thread = sample_thread("ST0001");
  fx.write_thread(&thread);
  // As 3.4.0's `intent init` leaves a project: the store is ignored, and the
  // config already carries its project id, which an upgrade would otherwise
  // mint on each branch apart.
  fx.write_file(".gitignore", "intent/.cache/\n");
  fx.write_file(
    "intent/.config/config.json",
    "{\n  \"intent_version\": \"3.4.0\",\n  \"project_id\": \"00000000-0000-4000-8000-000000000001\",\n  \"created\": \"2026-11-01T00:00:00.000Z\",\n  \"project_name\": \"Fixture\",\n  \"author\": \"cc\",\n  \"intent_dir\": \"intent\",\n  \"languages\": [\"rust\"]\n}\n",
  );
  fx.git_init();
  fx.git_commit_all();
  let upgraded = |fx: &Fixture| {
    Facade::upgrade(&fx.project(), &crate::common::facade_ctx()).expect("the upgrade");
    fx.git_commit_all();
  };

  fx.git(&["switch", "-q", "-c", "side"]);
  let mut moved = thread.clone();
  let first = moved
    .wps
    .first_mut()
    .expect("the sample thread has packages");
  first.status = if first.status == WpStatus::Wip {
    WpStatus::NotStarted
  } else {
    WpStatus::Wip
  };
  let package = first.seq.stored();
  fx.write_thread(&moved);
  fx.git_commit_all();
  upgraded(&fx);
  fx.git(&["switch", "-q", "-"]);
  // The switch leaves the upgraded branch's aggregate views, which this branch
  // does not ignore yet; the upgrade names them and says to remove them
  // (issue Ie7fb84c), and they are views, so removing them loses nothing.
  fx.git(&["clean", "-q", "-f", "-d", "--", "intent"]);
  upgraded(&fx);

  let merged = std::process::Command::new("git")
    .args(["merge", "--no-edit", "side"])
    .current_dir(fx.root())
    .output()
    .expect("run git merge");
  let project = fx.project();
  let record = project.relative(
    &project
      .intent_dir()
      .join(canon_wp_rel(&thread.id.stored(), &package)),
  );
  assert!(
    !merged.status.success()
      && intentsvcs::gitstate::unmerged(fx.root())
        .expect("the unmerged paths")
        .iter()
        .any(|u| u.path == record && !u.base),
    "the fixture must stop the merge on the package's file as an add/add: {}",
    String::from_utf8_lossy(&merged.stdout)
  );

  let plan = fx
    .facade_on_disk()
    .sync_plan(&intentsvcs::sync::Scope::All)
    .expect("one package in two states must be planned, not refused");
  assert!(
    plan
      .steps
      .iter()
      .any(|s| matches!(&s.action, Action::TakeSide { path } if *path == record)),
    "the package's file must be a side to take"
  );
}
