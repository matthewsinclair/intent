//! Issue 0722, sites 3 to 5: **doctor, the attachment read doors and realise
//! answer a name spelled as a held row in another case as that row where the
//! disk folds case, and name the pair where it does not; a pair already in
//! canon is doctor's finding and realise's refusal on any disk.**
//!
//! The disk's fold is read once per test on the real disk and named in every
//! assertion that depends on it. On a disk that folds (macOS's default) the
//! not-folding arms of doctor and the read doors are NOT DRIVEN by this file,
//! and on one that does not (Linux) the folding arms are not: the same
//! assertions take the other answer there. The canon pair does not depend on
//! the disk and is driven everywhere.

use crate::common::{Fixture, ctx, sample_thread};
use intentsvcs::facade::FacadeError;
use intentsvcs::finding::{Finding, FindingClass};
use intentsvcs::ingest::Canon;
use intentsvcs::model::{Attachment, Thread};
use intentsvcs::realise::{self, RealiseError};
use intentsvcs::views::ThreadFileRead;

const ID: &str = "ST0001";
/// `sample_thread`'s first attachment row, and the bytes the store holds for it.
const ROW: &str = "reference.md";
const CARRIED: &str = "# Reference\n\nA quokka.\n";
/// The row's name in another case, as an editor or a checkout may leave it.
const TWIN: &str = "Reference.md";
const EDITED: &str = "# Reference\n\nA wombat.\n";

fn findings(fx: &Fixture, class: FindingClass) -> Vec<Finding> {
  intentsvcs::doctor::diagnose(&fx.project(), &ctx(), None, intentsvcs::doctor::Scope::All)
    .findings
    .into_iter()
    .filter(|f| f.class == class)
    .collect()
}

fn folds(fx: &Fixture) -> bool {
  fx.project()
    .disk_folds_case()
    .expect("the fixture's disk answers whether it folds case")
}

/// A thread holding both spellings, which only a disk that does not fold could
/// have carried in.
fn thread_holding_a_pair() -> Thread {
  let mut thread = sample_thread(ID);
  thread.attachments = vec![Attachment::new(ROW, CARRIED), Attachment::new(TWIN, EDITED)];
  thread
}

#[test]
fn doctor_compares_a_twin_as_the_row_where_the_disk_folds_and_names_the_pair_where_it_does_not() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread(ID));
  let folds = folds(&fx);
  let twin = format!("intent/st/{ID}/{TWIN}");
  fx.write_file(&twin, EDITED);

  let drift = findings(&fx, FindingClass::AttachmentDrift);
  assert_eq!(
    drift.iter().any(|f| f.file == twin),
    folds,
    "folds={folds}: the twin is the row's working copy, and its edit is drift, exactly when the disk folds case: {drift:?}"
  );
  let pairs = findings(&fx, FindingClass::CasePair);
  assert_eq!(
    pairs.iter().any(|f| f.file == twin),
    !folds,
    "folds={folds}: the twin is named as a pair with the row exactly when the disk keeps two files: {pairs:?}"
  );
}

#[test]
fn doctor_names_a_pair_held_in_canon_whatever_the_disk_does() {
  let fx = Fixture::new();
  fx.write_thread(&thread_holding_a_pair());
  let canon_file = fx.canon_rel(ID);
  let pairs = findings(&fx, FindingClass::CasePair);
  assert!(
    pairs
      .iter()
      .any(|f| f.file == canon_file && f.detail.contains(ROW) && f.detail.contains(TWIN)),
    "a pair in canon is seen only here, since the store keeps no key column, and a clone made on a Mac would realise it as one file: {pairs:?}"
  );
}

#[test]
fn a_name_in_another_case_reads_as_its_row_where_the_disk_folds_and_as_not_carried_where_it_does_not()
 {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread(ID));
  let folds = folds(&fx);
  let facade = fx.facade();
  // Each arm is pinned to its own answer, so an error from the facade fails
  // both: the row where the disk folds, `NotCarried` where it does not.
  assert_eq!(
    facade
      .st_attachment(ID, TWIN)
      .map(|a| a.path.as_str())
      .map_err(|e| matches!(e, FacadeError::NotCarried { .. })),
    folds.then_some(ROW).ok_or(true),
    "folds={folds}: the name answers as the row it is on a disk that folds, keeping the row's name, and is not carried on one that does not"
  );
  let read = facade.read_thread_file(ID, TWIN).ok();
  assert_eq!(
    read == Some(ThreadFileRead::Text(CARRIED.to_string())),
    folds,
    "folds={folds}: the pane reads the row's text for its other spelling exactly where the disk folds: {read:?}"
  );
  assert_eq!(
    read == Some(ThreadFileRead::Absent),
    !folds,
    "folds={folds}: and reads it as absent, not as an error, where the disk does not: {read:?}"
  );
}

#[test]
fn realise_refuses_a_pair_held_in_canon_before_it_writes_anything() {
  let fx = Fixture::new();
  let canon = Canon {
    threads: vec![thread_holding_a_pair()],
    ..Default::default()
  };
  let root = fx.path("realised");
  let err = realise::realise(&fx.project(), &canon, &ctx(), &root)
    .expect_err("a pair in canon would be one file on a disk that folds case");
  assert!(
    matches!(&err, RealiseError::CasePair { held, path, .. } if held == ROW && path == TWIN),
    "the refusal names both spellings: {err:?}"
  );
  assert!(
    !root.exists(),
    "refused before the first write, so no half a realisation is left behind"
  );
}
