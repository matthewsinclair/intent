//! ST0096 WP-05 (AC-05.1): in the release's canon format a thread is one file
//! for its own fields and one file per work package, criterion, test and
//! attachment under its records directory, each named by its stored id or
//! label. No file lists, counts or orders a thread's children, and the layout
//! reads back to the thread it was written from, byte for byte when written
//! again. An opaque attachment's bytes stay in the thread's own directory, a
//! root the records do not share.

use crate::common::{Fixture, sample_thread};
use intentsvcs::canon_format::{CanonFormat, CanonFormatError, encode_thread, encode_thread_files};
use intentsvcs::finding::{Finding, FindingClass};
use intentsvcs::ingest::IngestError;
use intentsvcs::model::{Attachment, Thread};
use intentsvcs::project::{
  CANON_WP_DIR, canon_attachment_rel, canon_blob_rel, canon_children_rel, canon_thread_rel,
  canon_wp_rel,
};
use std::collections::{BTreeMap, BTreeSet};

/// The thread, as every writer writes it, laid out under `fx`'s intent
/// directory, with each opaque attachment's bytes where the reader looks.
fn lay_out(fx: &Fixture, thread: &Thread) -> Vec<(String, String)> {
  let files = encode_thread_files(thread, CanonFormat::V4).expect("the layout");
  let intent = fx.project().intent_dir();
  let put = |rel: &str, bytes: &[u8]| {
    let path = intent.join(rel);
    std::fs::create_dir_all(path.parent().expect("a parent")).expect("mkdir");
    std::fs::write(&path, bytes).expect("write a canon file");
  };
  for (rel, bytes) in &files {
    put(rel, bytes.as_bytes());
  }
  for attachment in thread.attachments.iter().filter(|a| a.is_opaque()) {
    put(
      &canon_blob_rel(&thread.id.stored(), &attachment.path),
      attachment
        .blob
        .as_deref()
        .expect("an opaque attachment's bytes"),
    );
  }
  files
}

/// What `ingest::read` refused with, or a panic naming what it did instead.
fn refused(fx: &Fixture) -> Vec<Finding> {
  match intentsvcs::ingest::read(&fx.project()) {
    Err(IngestError::Refused(refusal)) => refusal.findings,
    other => panic!("the read must refuse, got {other:?}"),
  }
}

/// The sample, with two opaque attachments whose paths a shared directory
/// could not have held: one spelled as a work package's record file, and one
/// that begins with a dot.
fn thread_with_awkward_attachments() -> Thread {
  let mut thread = sample_thread("ST0001");
  let on_a_record = format!("{CANON_WP_DIR}/{}.json", thread.wps[0].seq.stored());
  thread
    .attachments
    .push(Attachment::opaque(on_a_record, vec![0_u8, 1, 2]));
  thread
    .attachments
    .push(Attachment::opaque(".github/logo.bin", vec![3_u8, 4, 5]));
  thread
}

// ST0096 AT-05.1: the arm below.
#[test]
fn a_thread_reads_back_from_its_records_files_and_writes_the_same_bytes() {
  let fx = Fixture::new();
  let thread = thread_with_awkward_attachments();
  for (kind, held) in [
    ("work packages", thread.wps.len()),
    ("criteria", thread.criteria.len()),
    ("tests", thread.tests.len()),
    ("attachments", thread.attachments.len()),
  ] {
    assert!(
      held > 0,
      "the sample must carry {kind}, or that arm is not tested"
    );
  }
  let files = lay_out(&fx, &thread);
  assert_eq!(
    files.len(),
    1 + thread.wps.len() + thread.criteria.len() + thread.tests.len() + thread.attachments.len(),
    "one file for the thread and one per child"
  );

  let own_rel = canon_thread_rel(&thread.id.stored());
  let own = files
    .iter()
    .find(|(rel, _)| *rel == own_rel)
    .expect("the thread's own file");
  let own: serde_json::Value = serde_json::from_str(&own.1).expect("the own file parses");
  for key in ["wps", "criteria", "tests", "attachments"] {
    assert!(
      own.get(key).is_none(),
      "the thread's own file lists its {key}: {own}"
    );
  }

  let canon = intentsvcs::ingest::read(&fx.project()).expect("the layout reads");
  let read = canon
    .threads
    .iter()
    .find(|t| t.id == thread.id)
    .expect("the thread is read");
  for attachment in thread.attachments.iter().filter(|a| a.is_opaque()) {
    let back = read
      .attachments
      .iter()
      .find(|a| a.path == attachment.path)
      .expect("the opaque attachment is read");
    assert_eq!(back.blob, attachment.blob, "{}'s bytes", attachment.path);
  }
  // As sets: a file's bytes are the record, and no file holds an order.
  let set = |files: Vec<(String, String)>| files.into_iter().collect::<BTreeMap<_, _>>();
  assert_eq!(
    set(encode_thread_files(read, CanonFormat::V4).expect("written again")),
    set(files),
    "the thread read from its files writes the same files"
  );
}

#[test]
fn the_ordinal_format_keeps_one_file_per_thread() {
  let thread = sample_thread("ST0001");
  let files = encode_thread_files(&thread, CanonFormat::V3).expect("the ordinal layout");
  assert_eq!(
    files,
    vec![(
      canon_thread_rel(&thread.id.stored()),
      encode_thread(&thread, CanonFormat::V3).expect("the one file")
    )],
    "a project `intent upgrade` has not moved is written as 3.4.0 writes it"
  );
}

#[test]
fn two_attachments_a_folding_disk_stores_as_one_file_are_refused() {
  let mut thread = sample_thread("ST0001");
  let id = thread.id.stored();
  let (upper, lower) = ("Notes.md", "notes.md");
  thread.attachments.push(Attachment::new(upper, "one"));
  thread.attachments.push(Attachment::new(lower, "two"));
  let pair = BTreeSet::from([
    canon_attachment_rel(&id, upper),
    canon_attachment_rel(&id, lower),
  ]);
  match encode_thread_files(&thread, CanonFormat::V4) {
    Err(CanonFormatError::SharedPath { first, second, .. }) => assert_eq!(
      BTreeSet::from([first, second]),
      pair,
      "the refusal names the two attachments' files"
    ),
    other => panic!("a case pair must be refused, got {other:?}"),
  }
}

#[test]
fn a_record_file_whose_name_is_not_its_record_is_refused_by_name() {
  let fx = Fixture::new();
  let thread = sample_thread("ST0001");
  let wp_file = canon_wp_rel(&thread.id.stored(), &thread.wps[0].seq.stored());
  let files = lay_out(&fx, &thread);
  assert!(
    files.iter().any(|(rel, _)| *rel == wp_file),
    "the package's file was written"
  );
  let intent = fx.project().intent_dir();
  let misnamed = wp_file.replace(".json", "-moved.json");
  std::fs::rename(intent.join(&wp_file), intent.join(&misnamed))
    .expect("rename the package's file");
  let misnamed = fx.project().relative(&intent.join(&misnamed));
  let findings = refused(&fx);
  assert!(
    findings
      .iter()
      .any(|f| f.class == FindingClass::SchemaInvalid
        && f.file == misnamed
        && f.detail.contains(&wp_file)),
    "the refusal names the file and the record's own: {findings:?}"
  );
}

#[test]
fn a_file_in_a_records_directory_that_is_no_record_is_named() {
  let fx = Fixture::new();
  let thread = sample_thread("ST0001");
  lay_out(&fx, &thread);
  let stray = fx
    .project()
    .intent_dir()
    .join(canon_children_rel(&thread.id.stored(), CANON_WP_DIR))
    .join("notes.txt");
  std::fs::write(&stray, "not a record").expect("write a stray file");
  let stray = fx.project().relative(&stray);
  let findings = refused(&fx);
  assert!(
    findings
      .iter()
      .any(|f| f.class == FindingClass::UnknownFileShape && f.file == stray),
    "the stray file is named: {findings:?}"
  );
}

#[test]
fn a_detached_attachments_record_file_leaves_the_disk() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();
  let thread = facade.st_show("ST0001").expect("the fixture thread");
  let path = thread.attachments[0].path.clone();
  let kept: Vec<String> = thread.attachments[1..]
    .iter()
    .map(|a| a.path.clone())
    .collect();
  let intent = fx.project().intent_dir();
  let record = intent.join(canon_attachment_rel("ST0001", &path));
  assert!(record.is_file(), "the attachment's record file was written");
  let address = intentsvcs::address::parse(&format!("intent:///threads/ST0001/attachments/{path}"))
    .expect("an attachment address");
  facade.detach_attachment(&address).expect("the detach");
  assert!(
    !record.exists(),
    "a record the store no longer holds leaves its file behind"
  );
  for other in kept {
    assert!(
      intent
        .join(canon_attachment_rel("ST0001", &other))
        .is_file(),
      "{other}'s record file is kept"
    );
  }
}

#[test]
fn a_thread_file_in_the_release_format_that_lists_its_children_is_refused() {
  let fx = Fixture::new();
  let thread = sample_thread("ST0001");
  lay_out(&fx, &thread);
  let own = fx.project().thread_json("ST0001");
  let mut value: serde_json::Value =
    serde_json::from_str(&std::fs::read_to_string(&own).expect("the own file"))
      .expect("the own file parses");
  value["wps"] = serde_json::to_value(&thread.wps).expect("the packages");
  std::fs::write(&own, value.to_string()).expect("list the packages inline");
  let own = fx.project().relative(&own);
  let findings = refused(&fx);
  assert!(
    findings
      .iter()
      .any(|f| f.class == FindingClass::SchemaInvalid && f.file == own),
    "the own file that lists its children is refused by name: {findings:?}"
  );
}

#[test]
fn criteria_read_from_their_files_take_the_label_order() {
  use intentsvcs::id::label_order;
  let mut labels = vec!["AC-01.10", "AC-02.1", "AC-01.2", "AC-01.1"];
  labels.sort_by(|a, b| label_order(a, b));
  assert_eq!(labels, ["AC-01.1", "AC-01.2", "AC-01.10", "AC-02.1"]);
}

#[test]
fn records_whose_thread_has_no_file_of_its_own_are_named() {
  let fx = Fixture::new();
  let thread = sample_thread("ST0001");
  lay_out(&fx, &thread);
  std::fs::remove_file(fx.project().thread_json("ST0001")).expect("drop the thread's own file");
  let records = fx
    .project()
    .intent_dir()
    .join(intentsvcs::project::canon_records_rel("ST0001"));
  let records = fx.project().relative(&records);
  let findings = refused(&fx);
  assert!(
    findings
      .iter()
      .any(|f| f.class == FindingClass::UnknownFileShape && f.file == records),
    "the records nothing reads are named: {findings:?}"
  );
}
