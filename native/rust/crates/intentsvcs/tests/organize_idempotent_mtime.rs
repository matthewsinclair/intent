//! AT-04.4 / AC-04.4: **`organize` run twice changes nothing, INCLUDING mtimes.**
//!
//! **MEASURED AS MTIME MOVEMENT, NOT AS A CONTENT DIFF, BECAUSE THE DEFECT IS A
//! BYTE-IDENTICAL RE-EMISSION.** A content comparison passes on exactly the bug
//! being closed: the second run rewrites every file with the same bytes, the diff
//! is empty, and `file_index`'s clean/changed state -- computed from mtime -- has
//! been moved for the whole estate. Measured 2026-08-18: 255 of 1000 `.md` files
//! re-emitted identically every pass.
//!
//! It is not avoidable by writing carefully. `write_atomically` is
//! temp-file-plus-rename and a rename swaps in a NEW INODE, so an identical
//! re-emission moves mtime BY CONSTRUCTION. The skip in `WriteSet::commit` is
//! what makes the second run quiet, which is why `organize` writes through it
//! rather than beside it.
//!
//! **THE POSITIVE CONTROL IS NOT OPTIONAL HERE.** "Zero mtimes moved" is
//! produced equally by a correct skip and by a measurement that cannot see
//! movement at all -- coarse filesystem resolution, two writes inside one tick, a
//! snapshot taken wrongly. So the same instrument is pointed at a run that MUST
//! move mtimes, and that arm has to fail-if-quiet for the quiet arm to mean
//! anything.

mod common;

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::time::SystemTime;

use common::{Fixture, ctx, sample_thread};
use intentsvcs::ingest::Canon;
use intentsvcs::intentfiles;
use intentsvcs::model::Attachment;
use intentsvcs::organize::{Action, Mode, Plan, TreeState, plan};
use intentsvcs::project::Project;

const MANIFEST: &str = "STEELTHREAD:ST0001\n\n# BEGIN INTENT\n# END INTENT\n";

fn canon() -> Canon {
  Canon {
    threads: vec![sample_thread("ST0001")],
    ..Default::default()
  }
}

fn a_plan(project: &Project, tree: &TreeState) -> Plan {
  plan(
    project,
    &canon(),
    &intentfiles::parse(MANIFEST).expect("manifest parses"),
    &ctx(),
    tree,
    "d".to_string(),
  )
}

/// mtime of every path the plan names that actually exists.
///
/// **This compares two observations to EACH OTHER and never to a clock** -- there
/// is no time asked for anywhere here, which is the only form of time reasoning
/// D42 leaves standing.
fn mtimes(paths: &[PathBuf]) -> BTreeMap<PathBuf, SystemTime> {
  paths
    .iter()
    .filter_map(|p| {
      std::fs::metadata(p)
        .ok()
        .and_then(|m| m.modified().ok())
        .map(|t| (p.clone(), t))
    })
    .collect()
}

/// The tree as it stands after a run: present paths, plus a real sha for every
/// attachment.
///
/// **THE HASHES ARE NOT OPTIONAL AND OMITTING THEM IS NOT A SMALLER FIXTURE.**
/// `plan` treats an unhashable attachment as DIVERGENT by design -- an
/// unanswered question is not agreement -- so a `TreeState` with no hashes makes
/// every attachment a divergence report and the second run stops being the
/// second run. The first draft of this test omitted them and measured the wrong
/// population.
fn tree_after(hydrated: &[PathBuf], attachments: &[PathBuf]) -> TreeState {
  TreeState {
    present: hydrated.iter().cloned().collect(),
    sha256: attachments
      .iter()
      .filter_map(|p| {
        std::fs::read_to_string(p)
          .ok()
          .map(|text| (p.clone(), Attachment::new("x", text).sha256))
      })
      .collect(),
  }
}

fn moved(
  before: &BTreeMap<PathBuf, SystemTime>,
  after: &BTreeMap<PathBuf, SystemTime>,
) -> Vec<PathBuf> {
  before
    .iter()
    .filter(|(p, t)| after.get(*p).is_some_and(|now| now != *t))
    .map(|(p, _)| p.clone())
    .collect()
}

#[test]
fn a_second_run_moves_no_mtimes() {
  let fx = Fixture::new();
  let project = fx.project();

  // First run: an empty tree, so everything declared hydrates.
  let first = a_plan(&project, &TreeState::default());
  let report = first
    .run(Mode::Apply, &|| "d".to_string())
    .expect("first run applies");
  assert!(
    !report.hydrated.is_empty(),
    "the first run must actually write, or the second proves nothing"
  );
  let written: Vec<PathBuf> = report.hydrated.clone();
  let before = mtimes(&written);
  assert_eq!(
    before.len(),
    written.len(),
    "every hydrated file must exist"
  );

  // Second run: the same plan against a tree that now HAS those files, so every
  // declared view is a Verify. Byte-identical, and must therefore not be written.
  let attachments: Vec<PathBuf> = first
    .with(Action::HydrateAttachment)
    .map(|s| s.path.clone())
    .collect();
  let second = a_plan(&project, &tree_after(&written, &attachments));
  let report2 = second
    .run(Mode::Apply, &|| "d".to_string())
    .expect("second run applies");
  let after = mtimes(&written);

  assert_eq!(
    moved(&before, &after),
    Vec::<PathBuf>::new(),
    "AC-04.4: the second run must move ZERO mtimes"
  );
  assert!(
    report2.rewritten.is_empty(),
    "and it must report nothing rewritten: {:?}",
    report2.rewritten
  );
  assert!(
    report2.diverged.is_empty(),
    "an unchanged attachment is not a divergence: {:?}",
    report2.diverged
  );
  // Derived from the plan rather than written as a literal: a hard-coded count
  // goes stale the moment the shared fixture grows a work package, and it goes
  // stale by passing.
  let verifies = second.with(Action::Verify).count();
  assert!(
    verifies > 0,
    "the second run must actually verify something"
  );
  assert_eq!(
    report2.unchanged.len(),
    verifies,
    "every file it considered must be reported as unchanged, not merely omitted -- a report that lists only writes cannot tell a quiet run from an empty one"
  );
}

#[test]
fn the_measurement_can_see_an_mtime_move() {
  // THE POSITIVE CONTROL. Without this, the test above passes identically if
  // mtime resolution is too coarse to resolve two writes, or if `mtimes` is
  // snapshotting the wrong thing -- and a green that a broken instrument also
  // produces is not evidence.
  let fx = Fixture::new();
  let project = fx.project();

  let first = a_plan(&project, &TreeState::default());
  let report = first
    .run(Mode::Apply, &|| "d".to_string())
    .expect("first run applies");
  let written = report.hydrated.clone();
  let before = mtimes(&written);

  // Hand-edit the VIEWS only. A clobbered attachment is a divergence and is
  // deliberately NOT rewritten -- authority follows authorship -- so including
  // them here would be asserting the opposite of AC-04.3.
  let views: Vec<PathBuf> = first
    .with(Action::Hydrate)
    .map(|s| s.path.clone())
    .collect();
  let attachments: Vec<PathBuf> = first
    .with(Action::HydrateAttachment)
    .map(|s| s.path.clone())
    .collect();
  assert!(!views.is_empty(), "fixture must produce views");
  for p in &views {
    std::fs::write(p, "clobbered by a human\n").expect("write");
  }
  let second = a_plan(&project, &tree_after(&written, &attachments));
  let report2 = second
    .run(Mode::Apply, &|| "d".to_string())
    .expect("second run applies");
  let after = mtimes(&views);

  assert_eq!(
    moved(&before, &after).len(),
    views.len(),
    "the instrument must be able to observe movement, or the quiet arm means nothing"
  );
  assert_eq!(
    report2.rewritten.len(),
    views.len(),
    "and a divergent view is REWRITTEN: a view is authored in the model, so the file is the stale side"
  );
}
