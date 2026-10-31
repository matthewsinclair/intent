//! AT-01.2 / AC-01.4 (ST0090 WP-01): **`intent index status` and `intent index
//! rebuild`, text and JSON, name when the index was last reconciled, read from
//! the store's `reconciled_at` and never from a clock at render, and an index
//! no reconcile has stamped names none.**
//!
//! **THE STAMP IS THE STORE'S, SO EVERY READ NAMES THE SAME ONE.** A report
//! that read a clock would name a later time on every run. One that reads the
//! column names the time the last reconcile wrote, and moves only when
//! another reconcile writes it, the one a search runs before it answers
//! included.

use crate::common::level_three::{envelope, estate, run};
use serde_json::json;
use std::path::Path;

/// The `reconciled:` line `intent index <verb>` prints.
fn reconciled_line(root: &Path, verb: &str) -> String {
  let (out, err, code) = run(&["index", verb], root);
  assert_eq!(code, 0, "index {verb} failed: {err}");
  out
    .lines()
    .find(|line| line.starts_with("reconciled: "))
    .unwrap_or_else(|| panic!("index {verb} names no reconcile time: {out}"))
    .to_string()
}

/// `reconciled_at` in what `intent index <verb> --json` prints.
fn reconciled_json(root: &Path, verb: &str) -> serde_json::Value {
  let (out, err, code) = run(&["index", verb, "--json"], root);
  assert_eq!(code, 0, "index {verb} --json failed: {err}");
  let report: serde_json::Value = serde_json::from_str(&out).expect("the report is JSON");
  report
    .get("reconciled_at")
    .cloned()
    .unwrap_or_else(|| panic!("index {verb} --json carries no reconciled_at: {report}"))
}

#[test]
fn an_index_no_reconcile_has_stamped_names_none() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  let (_, err, code) = run(&["init", "reconcile-fixture"], root);
  assert_eq!(code, 0, "fixture init failed: {err}");

  assert_eq!(reconciled_line(root, "status"), "reconciled: none");
  assert_eq!(
    reconciled_json(root, "status"),
    json!(null),
    "an index no reconcile has stamped names no time"
  );
}

#[test]
fn the_report_names_the_stamp_the_last_reconcile_wrote() {
  let dir = estate();
  let root = dir.path();

  let line = reconciled_line(root, "rebuild");
  let at = line
    .strip_prefix("reconciled: ")
    .expect("the line names a time")
    .to_string();
  assert!(
    at.ends_with('Z'),
    "the stamp is the store's UTC time: {line}"
  );
  assert_eq!(
    reconciled_line(root, "status"),
    line,
    "status names the stamp the rebuild wrote, where a clock read at render would name a later time"
  );
  assert_eq!(reconciled_json(root, "status"), json!(at));

  let rebuilt = reconciled_json(root, "rebuild");
  assert!(
    rebuilt.as_str().is_some_and(|again| again > at.as_str()),
    "each rebuild stamps anew, and its JSON names the stamp it wrote: {rebuilt} after {at}"
  );
  assert_eq!(reconciled_json(root, "status"), rebuilt);

  let answered = envelope(root, &["helper"])["index"]["reconciled_at"].clone();
  assert!(
    answered
      .as_str()
      .zip(rebuilt.as_str())
      .is_some_and(|(search, before)| search > before),
    "a search reconciles before it answers and stamps: {answered} after {rebuilt}"
  );
  assert_eq!(
    reconciled_json(root, "status"),
    answered,
    "the report reads the column the search door stamped, which is the one its envelope names"
  );
}
