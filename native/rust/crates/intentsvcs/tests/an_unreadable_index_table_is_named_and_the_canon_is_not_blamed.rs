//! Issue 0447: **a store whose search-index table cannot be read says so, says
//! the canon is intact, and does not name a way out that cannot work.**
//!
//! Driven on a scratch project with `doc_sections` replaced by a plain table,
//! every verb refused in the open-time ingest with "could not read the
//! committed canon" and the remedy "fix the artefacts named above" -- naming
//! none, because none is at fault. `intent index rebuild`, `intent backup` and
//! `intent backup --list` refused with the same error, since each opens the
//! store through the same read; and `intent doctor` read the store at rc 0 with
//! zero findings.
//!
//! **THE DAMAGE IS MADE THE WAY THE DRIVE MADE IT**, on the store file directly
//! and after the canon is in it, so the open reads the store rather than
//! re-ingesting a cold one.

use crate::common::{Fixture, facade_ctx, sample_thread};
use intentsvcs::facade::{Facade, FacadeError};
use intentsvcs::finding::FindingClass;
use intentsvcs::ingest::IngestError;
use intentsvcs::remedy::Remedy;

/// A project with one thread in a warm store, then `doc_sections` replaced by
/// a plain table the read cannot use.
fn a_store_with_an_unreadable_index(fx: &Fixture) {
  fx.write_thread(&sample_thread("ST0001"));
  Facade::open(fx.project(), facade_ctx()).expect("the store warms from canon");
  let db = rusqlite::Connection::open(fx.root().join("intent/.cache/intent.db"))
    .expect("open the store directly");
  db.execute_batch("DROP TABLE doc_sections; CREATE TABLE doc_sections (unrelated TEXT);")
    .expect("replace the fts5 table with a plain one");
}

fn the_open_refusal(fx: &Fixture) -> FacadeError {
  match Facade::open(fx.project(), facade_ctx()) {
    Ok(_) => panic!("a store whose index table cannot be read opened, so nothing below is tested"),
    Err(e) => e,
  }
}

#[test]
fn the_open_names_the_table_and_does_not_blame_the_canon() {
  let fx = Fixture::new();
  a_store_with_an_unreadable_index(&fx);

  let err = the_open_refusal(&fx);
  assert!(
    matches!(
      err,
      FacadeError::Ingest(IngestError::IndexUnreadable {
        table: "doc_sections",
        ..
      })
    ),
    "an unreadable index table is the store's, not a canon refusal: {err:?}"
  );
  assert_eq!(err.to_string(), "could not read the store's search index");

  let remedy = err.remedy();
  assert!(
    remedy.contains("canon is intact") && remedy.contains("not at fault"),
    "{remedy}"
  );
  assert!(
    remedy.contains("`doc_sections`"),
    "the remedy names the table: {remedy}"
  );
  assert!(
    !remedy.contains("fix the artefacts"),
    "the artefacts remedy names none, because none is at fault: {remedy}"
  );
  // **THE ONE VERB THAT OPENS THROUGH A REPAIR IS THE WAY OUT, AND ONLY IT**
  // (issue 0453). `intent backup` still opens through this read, so it is not
  // offered.
  assert!(
    remedy.contains("Run `intent index rebuild`") && remedy.contains("issue 0453"),
    "the remedy names the verb that repairs it: {remedy}"
  );
  assert!(
    !remedy.contains("intent backup`"),
    "`intent backup` refuses on this same open, so it cannot be the remedy: {remedy}"
  );
  assert!(
    remedy.contains("There is no snapshot of this store in intent/.backup/db"),
    "with no snapshot on disk, the remedy says there is none rather than pointing at nothing: {remedy}"
  );
}

#[test]
fn the_newest_snapshot_on_disk_is_named_because_no_verb_can_list_it() {
  let fx = Fixture::new();
  a_store_with_an_unreadable_index(&fx);
  // Two snapshots by their stamped names; the newer sorts greater.
  fx.write_file("intent/.backup/db/2026-09-17T10-00-00-000Z.db", "older");
  fx.write_file("intent/.backup/db/2026-09-18T08-00-00-000Z.db", "newer");

  let remedy = the_open_refusal(&fx).remedy();
  assert!(
    remedy.contains("intent/.backup/db/2026-09-18T08-00-00-000Z.db"),
    "the newest snapshot is named by path, since `intent backup --list` refuses on this open: {remedy}"
  );
  assert!(
    !remedy.contains("2026-09-17T10-00-00-000Z"),
    "and only the newest: {remedy}"
  );
  assert!(
    remedy.contains("no restore verb ships"),
    "the remedy must not imply a restore command exists: {remedy}"
  );
}

#[test]
fn doctor_counts_an_unreadable_index_table_as_a_finding() {
  let fx = Fixture::new();
  a_store_with_an_unreadable_index(&fx);

  // **THE STORE IS PASSED, BECAUSE THE CHECK IS A STORE CHECK.** `doctor`
  // opens the store without the ingest and hands it in; a `None` here skips
  // every store-side check, and the first build of this arm measured exactly
  // that and read it as the class never firing.
  let store = intentsvcs::store::Store::open(&fx.project().db_path()).expect("the store opens");
  let report = intentsvcs::doctor::diagnose(
    &fx.project(),
    &crate::common::ctx(),
    Some(&store),
    intentsvcs::doctor::Scope::All,
  );
  let found = report
    .findings
    .iter()
    .find(|f| f.class == FindingClass::IndexUnreadable)
    .unwrap_or_else(|| {
      panic!(
        "doctor read a store whose every verb refuses and said nothing about it: {:?}",
        report
          .findings
          .iter()
          .map(ToString::to_string)
          .collect::<Vec<_>>()
      )
    });
  assert!(
    found.class.is_actionable(),
    "COUNTED, so the commit gate and the fleet trawl can see a store with every verb down"
  );
  assert!(
    found.detail.contains("`doc_sections`") && found.detail.contains("canon is intact"),
    "the finding carries the same sentence the refusal does: {}",
    found.detail
  );
  assert_ne!(report.exit_code(), 0, "and the verdict moves");
}

/// **THE CONTROL.** A store whose index reads cleanly carries no such finding,
/// or the class would be a label every store wears.
#[test]
fn a_readable_index_carries_no_such_finding() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  Facade::open(fx.project(), facade_ctx()).expect("the store warms from canon");

  // **THE STORE IS PASSED, BECAUSE THE CHECK IS A STORE CHECK.** `doctor`
  // opens the store without the ingest and hands it in; a `None` here skips
  // every store-side check, and the first build of this arm measured exactly
  // that and read it as the class never firing.
  let store = intentsvcs::store::Store::open(&fx.project().db_path()).expect("the store opens");
  let report = intentsvcs::doctor::diagnose(
    &fx.project(),
    &crate::common::ctx(),
    Some(&store),
    intentsvcs::doctor::Scope::All,
  );
  assert!(
    !report
      .findings
      .iter()
      .any(|f| f.class == FindingClass::IndexUnreadable),
    "a readable index table was reported as unreadable"
  );
}

/// **THE SHAPE THE CLI HANDS IN.** `intent doctor` opens the store through the
/// facade, whose open refuses on this table, so the report is built with NO
/// store -- and the first build of this fix passed the arm above while the CLI
/// still read the damaged store at rc 0. This is the arm that caught it.
#[test]
fn doctor_given_no_store_still_finds_the_unreadable_table() {
  let fx = Fixture::new();
  a_store_with_an_unreadable_index(&fx);

  let report = intentsvcs::doctor::diagnose(
    &fx.project(),
    &crate::common::ctx(),
    None,
    intentsvcs::doctor::Scope::All,
  );
  assert!(
    report
      .findings
      .iter()
      .any(|f| f.class == FindingClass::IndexUnreadable),
    "with no store handed in, doctor did not look -- which is what the CLI does on this store"
  );
}
