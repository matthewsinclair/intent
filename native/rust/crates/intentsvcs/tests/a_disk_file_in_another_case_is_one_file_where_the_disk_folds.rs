//! Issue 0722, sites 1 and 2: **a file on a live disk spelled as a held row in
//! another case is that row's working copy where the disk folds case, and a
//! second file, carried by nobody and named as the pair it makes, where it does
//! not.**
//!
//! Site 1 is `ingest::collect_attachments_into` (the `sync --to-store` carry),
//! which reads the fold itself, so it is driven on the real disk with the fold
//! read once here and named in every assertion. On a disk that folds (macOS's
//! default) its case-pair arm is NOT DRIVEN by this file: the pair cannot exist
//! there. It is driven on a disk that does not fold (Linux), where the same
//! assertions take the other answer.
//!
//! Site 2 is `organize::plan`, which takes the fold in its `TreeState`, so both
//! answers are driven on every host by injecting it.

use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::common::{Fixture, ctx, sample_thread};
use intentsvcs::finding::FindingClass;
use intentsvcs::ingest::{Canon, collect_attachments_into};
use intentsvcs::intentfiles;
use intentsvcs::model::Attachment;
use intentsvcs::organize::{Action, Step, TreeState, observe, plan};

const DECLARED: &str = "STEELTHREAD:ST0001\n\n";
/// `sample_thread`'s first attachment row, and the bytes the store holds for it.
const ROW: &str = "reference.md";
const CARRIED: &str = "# Reference\n\nA quokka.\n";
/// The row's name in another case, as an editor or a checkout may leave it.
const TWIN: &str = "Reference.md";
const EDITED: &str = "# Reference\n\nA wombat.\n";

fn canon() -> Canon {
  Canon {
    threads: vec![sample_thread("ST0001")],
    ..Default::default()
  }
}

fn in_thread(fx: &Fixture, name: &str) -> PathBuf {
  fx.project().st_dir().join("ST0001").join(name)
}

fn step_for<'a>(steps: &'a [Step], path: &PathBuf) -> Option<&'a Step> {
  steps.iter().find(|s| &s.path == path)
}

fn plan_over(fx: &Fixture, tree: TreeState) -> Vec<Step> {
  plan(
    &fx.project(),
    &canon(),
    &intentfiles::realised_for_action(DECLARED).expect("manifest parses"),
    &ctx(),
    &tree,
    "d".to_string(),
  )
  .steps
}

/// The row's file present only under its twin's spelling, agreeing with the
/// store, with the disk's fold as the test gives it.
fn twin_tree(fx: &Fixture, folds: bool) -> TreeState {
  let twin = in_thread(fx, TWIN);
  TreeState {
    present: [twin.clone()].into_iter().collect(),
    sha256: BTreeMap::from([(twin, Attachment::new(ROW, CARRIED).sha256)]),
    folds,
    ..Default::default()
  }
}

#[test]
fn the_carry_takes_a_twin_as_the_row_where_the_disk_folds_and_reports_it_where_it_does_not() {
  let fx = Fixture::new();
  let folds = fx
    .project()
    .disk_folds_case()
    .expect("the fixture's disk answers whether it folds case");
  let dir = in_thread(&fx, "");
  std::fs::create_dir_all(&dir).expect("mkdir thread dir");
  std::fs::write(dir.join(TWIN), EDITED).expect("write the twin");

  let mut canon = canon();
  let findings = collect_attachments_into(&fx.project(), &mut canon);
  let attachments = &canon.threads[0].attachments;
  let row = attachments
    .iter()
    .find(|a| a.path == ROW)
    .expect("canon's row keeps its name whatever the disk does: no rename is inferred");

  assert_eq!(
    row.sha256 == Attachment::new(ROW, EDITED).sha256,
    folds,
    "folds={folds}: the twin's bytes are the row's exactly when the disk folds case"
  );
  assert!(
    attachments.iter().all(|a| a.path != TWIN),
    "folds={folds}: the twin is never carried as a row of its own, which would make the pair in the store"
  );
  assert_eq!(
    findings.iter().any(|f| f.class == FindingClass::CasePair),
    !folds,
    "folds={folds}: a case pair is reported exactly when the disk keeps two files: {findings:?}"
  );
}

#[test]
fn organize_verifies_a_twin_as_the_row_on_a_disk_that_folds() {
  let fx = Fixture::new();
  let steps = plan_over(&fx, twin_tree(&fx, true));
  assert!(
    step_for(&steps, &in_thread(&fx, ROW)).is_none(),
    "the row's file is on disk under the twin's spelling, so hydrating the row would write over it: {steps:?}"
  );
  assert!(
    step_for(&steps, &in_thread(&fx, TWIN)).is_none(),
    "the twin agrees with the store and is the row's file, so it needs no step and is not unclaimed: {steps:?}"
  );
}

#[test]
fn organize_refuses_a_twin_as_a_case_pair_and_writes_no_pair_on_a_disk_that_does_not_fold() {
  let fx = Fixture::new();
  let steps = plan_over(&fx, twin_tree(&fx, false));
  assert!(
    step_for(&steps, &in_thread(&fx, ROW)).is_none(),
    "writing the row beside its twin makes a pair that collides on the next clone made on a Mac: {steps:?}"
  );
  assert_eq!(
    step_for(&steps, &in_thread(&fx, TWIN)).map(|s| &s.action),
    Some(&Action::CasePair),
    "the twin is a second file here, named as the pair it makes with the row, not as a file nothing carries: {steps:?}"
  );
}

#[test]
fn organize_verifies_a_cover_spelled_in_another_case_on_a_disk_that_folds() {
  let fx = Fixture::new();
  let cover = in_thread(&fx, "info.md");
  let twin = in_thread(&fx, "INFO.md");
  let tree = TreeState {
    present: [twin.clone()].into_iter().collect(),
    folds: true,
    ..Default::default()
  };
  let steps = plan_over(&fx, tree);
  assert_eq!(
    step_for(&steps, &cover).map(|s| &s.action),
    Some(&Action::Verify),
    "the cover is on disk under another case, so it is verified, never hydrated over: {steps:?}"
  );
  assert!(
    step_for(&steps, &twin).is_none(),
    "the twin is the cover's file and is decided with it: {steps:?}"
  );
}

#[test]
fn organize_keeps_a_hand_edit_in_a_cover_spelled_in_another_case_on_a_disk_that_folds() {
  let fx = Fixture::new();
  let cover = in_thread(&fx, "info.md");
  let twin = in_thread(&fx, "INFO.md");
  let rendered = plan_over(
    &fx,
    TreeState {
      folds: true,
      ..Default::default()
    },
  )
  .into_iter()
  .find(|s| s.path == cover)
  .and_then(|s| s.content)
  .map(|bytes| String::from_utf8(bytes).expect("a cover renders as UTF-8"))
  .expect("a declared cover with nothing on disk is hydrated from its render");
  let edited = rendered.replacen(
    "## Objective\n\n",
    "## Objective\n\nA line written by hand, which the store does not hold yet.\n",
    1,
  );
  assert_ne!(
    edited, rendered,
    "CONTROL: the hand edit landed in the copy, so this test measures something"
  );
  let tree = TreeState {
    present: [twin.clone()].into_iter().collect(),
    covers: BTreeMap::from([(twin, edited)]),
    folds: true,
    ..Default::default()
  };
  let steps = plan_over(&fx, tree);
  assert_eq!(
    step_for(&steps, &cover).map(|s| &s.action),
    Some(&Action::CoverDiverged),
    "the hand edit is in the cover's own file under another spelling, so it is kept and refused by name, never rendered over: {steps:?}"
  );
}

#[test]
fn organize_hydrates_the_cover_and_names_the_pair_on_a_disk_that_does_not_fold() {
  let fx = Fixture::new();
  let cover = in_thread(&fx, "info.md");
  let twin = in_thread(&fx, "INFO.md");
  let tree = TreeState {
    present: [twin.clone()].into_iter().collect(),
    folds: false,
    ..Default::default()
  };
  let steps = plan_over(&fx, tree);
  assert_eq!(
    step_for(&steps, &cover).map(|s| &s.action),
    Some(&Action::Hydrate),
    "a generated view is the renderer's, and a second file in another case does not stand in for it here: {steps:?}"
  );
  assert_eq!(
    step_for(&steps, &twin).map(|s| &s.action),
    Some(&Action::CasePair),
    "the other spelling is named as the pair it makes with the cover: {steps:?}"
  );
}

#[test]
fn organize_names_every_other_spelling_as_a_pair_with_the_declared_one_beside_it_on_a_disk_that_does_not_fold()
 {
  let fx = Fixture::new();
  let row = in_thread(&fx, ROW);
  let twin = in_thread(&fx, TWIN);
  let cover = in_thread(&fx, "info.md");
  let cover_twin = in_thread(&fx, "INFO.md");
  let carried = Attachment::new(ROW, CARRIED).sha256;
  let tree = TreeState {
    present: [row.clone(), twin.clone(), cover.clone(), cover_twin.clone()]
      .into_iter()
      .collect(),
    sha256: BTreeMap::from([(row.clone(), carried.clone()), (twin.clone(), carried)]),
    folds: false,
    ..Default::default()
  };
  let steps = plan_over(&fx, tree);
  assert_eq!(
    step_for(&steps, &twin).map(|s| &s.action),
    Some(&Action::CasePair),
    "the row's other spelling is the pair with its own spelling beside it, not a file nothing carries: {steps:?}"
  );
  assert_eq!(
    step_for(&steps, &cover_twin).map(|s| &s.action),
    Some(&Action::CasePair),
    "the cover's other spelling is the pair on every run, not only the run that wrote the cover: {steps:?}"
  );
}

#[test]
fn observe_records_a_cover_spelled_in_another_case_exactly_where_the_disk_folds() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let project = fx.project();
  let folds = project
    .disk_folds_case()
    .expect("the fixture's disk answers whether it folds case");
  let twin = in_thread(&fx, "INFO.md");
  std::fs::create_dir_all(twin.parent().expect("a thread dir")).expect("mkdir thread dir");
  std::fs::write(&twin, "# ST0001\n\n## Objective\n\nWritten by hand.\n").expect("write the twin");
  let (tree, _digest) = observe(&project, &[]).expect("the tree is observed");
  assert_eq!(
    tree.folds, folds,
    "observe carries the fold it read, the one the fixture read"
  );
  assert_eq!(
    tree.covers.contains_key(&twin),
    folds,
    "folds={folds}: a cover spelled in another case is the cover's own file, and its bytes are kept for the hand-edit check, exactly when the disk folds case"
  );
}
