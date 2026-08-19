//! AT-03.4 / ST0057 AC-03.4: **`sha256` in canon detects a working-copy edit by
//! COMPARISON rather than inference, and `doctor` names the path.**
//!
//! # "By comparison" is the load-bearing half of the criterion
//!
//! The row could have been satisfied by anything that noticed a file had been
//! touched -- an mtime newer than the last sync, a size that changed, a dirty
//! flag set by whoever wrote last. **Every one of those infers content from a
//! proxy, and every one of them is wrong in the direction that costs the file**:
//! a proxy that says "unchanged" ends the enquiry, and an edit that restores a
//! file's size or is written by a tool that preserves mtime is then invisible.
//!
//! So the sharpest arm here is not the one that detects an edit. It is
//! `a_touched_file_whose_bytes_did_not_change_is_not_drift`: mtime moves, size
//! is identical, content is identical, and NOTHING is reported. An
//! implementation keyed on metadata passes every other test in this file and
//! fails that one.
//!
//! # An ABSENT attachment is not drift, and that arm is not a courtesy
//!
//! Under `.intentfiles` a dehydrated thread's attachments are legitimately gone
//! -- that is the feature. A check reporting absence as divergence would make
//! `doctor` unhealthy for every dehydrated thread in the estate, which is a
//! rule flagging a population that is behaving correctly, and the shape that
//! gets a check deleted rather than repaired.
//!
//! # The control comes first in the reasoning, not last
//!
//! A `doctor` that reported drift on every attachment would satisfy the
//! criterion's own wording. So the clean-thread arm is what makes the detection
//! arm mean anything, and both drive the same real `diagnose`.

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::finding::{Finding, FindingClass};
use intentsvcs::model::{Attachment, Thread};
use intentsvcs::project::Project;
use testkit::repo_root;

const ID: &str = "ST0001";
const REL: &str = "reference.md";
const ORIGINAL: &str = "# Reference\n\nAs the author left it.\n";

fn thread_with_an_attachment() -> Thread {
  let mut thread = sample_thread(ID);
  thread.attachments = vec![Attachment::new(REL, ORIGINAL)];
  thread
}

/// Seed canon AND the realised working copy, so the two agree to begin with.
fn seeded() -> Fixture {
  let fx = Fixture::new();
  let thread = thread_with_an_attachment();
  fx.write_thread(&thread);
  let dir = fx.path(&format!("intent/st/{ID}"));
  std::fs::create_dir_all(&dir).expect("mkdir");
  std::fs::write(dir.join(REL), ORIGINAL).expect("write the working copy");
  fx
}

fn drift(fx: &Fixture) -> Vec<Finding> {
  intentsvcs::doctor::diagnose(&fx.project(), &common::ctx(), None)
    .findings
    .into_iter()
    .filter(|f| f.class == FindingClass::AttachmentDrift)
    .collect()
}

/// **THE CONTROL, and it runs first because nothing below means anything
/// without it.**
#[test]
fn an_attachment_that_matches_canon_is_not_reported() {
  let fx = seeded();
  let found = drift(&fx);
  assert!(
    found.is_empty(),
    "an untouched attachment is reported as drift, so every detection below would fire for a \
     project in which nothing is wrong: {found:?}"
  );
}

/// **THE CRITERION: an edit the store never saw is reported, and the report
/// names the path.**
#[test]
fn an_attachment_edited_on_disk_is_reported_as_divergence_by_path() {
  let fx = seeded();
  std::fs::write(
    fx.path(&format!("intent/st/{ID}/{REL}")),
    "# Reference\n\nEdited in the working copy, and nothing told the store.\n",
  )
  .expect("edit the working copy");

  let found = drift(&fx);
  assert_eq!(
    found.len(),
    1,
    "one attachment was edited and {} divergence(s) were reported: {found:?}",
    found.len()
  );
  let rendered = found[0].to_string();
  assert!(
    rendered.contains(REL),
    "the report does not name the path, so an operator with several attachments cannot tell which \
     file to look at: {rendered}"
  );
  assert!(
    rendered.contains(ID),
    "the report does not name the thread: {rendered}"
  );
}

/// **THE ARM THAT SEPARATES COMPARISON FROM INFERENCE.**
///
/// The file is rewritten with byte-identical content, which moves its mtime and
/// leaves its size alone. **An implementation keyed on mtime reports drift here
/// and is wrong; one keyed on size misses the real edit above.** Only a
/// comparison of the bytes gets both.
#[test]
fn a_touched_file_whose_bytes_did_not_change_is_not_drift() {
  let fx = seeded();
  let path = fx.path(&format!("intent/st/{ID}/{REL}"));
  let before = std::fs::metadata(&path)
    .expect("stat")
    .modified()
    .expect("mtime");

  // Rewrite with the SAME bytes. A temp-file-plus-rename lands a new inode, so
  // this is exactly what a re-emission of unchanged content does.
  std::fs::write(&path, ORIGINAL).expect("rewrite identically");
  let after = std::fs::metadata(&path)
    .expect("stat")
    .modified()
    .expect("mtime");

  assert_eq!(
    std::fs::read_to_string(&path).expect("read"),
    ORIGINAL,
    "precondition: the bytes really are unchanged, or this arm is the detection test again"
  );
  // **Not asserted as a hard precondition.** Filesystem timestamp granularity
  // means an immediate rewrite can land in the same tick, and a test that
  // FAILED on that would be flaky for a reason unrelated to its subject. When
  // it does move, this arm proves the check ignored it; when it does not, the
  // arm still asserts the correct outcome and simply proves less.
  if after == before {
    eprintln!(
      "AT-03.4: the rewrite landed within one timestamp tick, so this run does not distinguish an \
       mtime-keyed implementation. The assertion below still holds."
    );
  }

  let found = drift(&fx);
  assert!(
    found.is_empty(),
    "a file rewritten with IDENTICAL bytes was reported as drift. The check is keyed on when the \
     file was written rather than on what is in it, so it will report every re-emission and go \
     quiet for any edit that preserves its proxy: {found:?}"
  );
}

/// **AN ABSENT ATTACHMENT IS NOT DRIFT.** Dehydration is the feature; a check
/// that called it a fault would be unhealthy on every dehydrated thread.
#[test]
fn an_attachment_that_is_not_realised_at_all_is_not_drift() {
  let fx = seeded();
  std::fs::remove_file(fx.path(&format!("intent/st/{ID}/{REL}"))).expect("dehydrate it");

  let found = drift(&fx);
  assert!(
    found.is_empty(),
    "a dehydrated attachment is reported as divergence. Under `.intentfiles` that is the state of \
     every closed thread's files, so `doctor` would be unhealthy for a population behaving exactly \
     as designed: {found:?}"
  );
}

/// **The remedy's first instruction must be the one that cannot lose anything.**
///
/// Neither side of an attachment divergence is re-derivable, so the safe first
/// act is to copy the working file out before deciding. It names only the
/// disk-ward command because the blast-radius rule forbids every remedy from
/// naming the store-ward one -- an asymmetry imposed on this remedy rather than
/// chosen by it, and reported to vc rather than worked around.
#[test]
fn the_remedy_leads_with_the_step_that_cannot_lose_anything() {
  let fx = seeded();
  std::fs::write(
    fx.path(&format!("intent/st/{ID}/{REL}")),
    "# Reference\n\nedited\n",
  )
  .expect("edit");

  let found = drift(&fx);
  let remedy = found[0].class.remedy();
  let copy_aside = remedy
    .find("copy")
    .expect("the remedy tells the operator to copy it aside");
  let overwrite = remedy
    .find("--to-disk")
    .expect("the remedy names the command that resolves it");
  assert!(
    copy_aside < overwrite,
    "the remedy names the overwriting command BEFORE telling the operator to save the file it \
     overwrites. An operator who acts on the first sentence loses the copy the second one was \
     about to protect: {remedy}"
  );
  assert!(
    !remedy.contains("--to-store"),
    "the blast-radius rule forbids a remedy from naming the store-ward direction, and this one \
     does: {remedy}"
  );
}

/// **THE ESTATE ARM, with the denominator printed.**
///
/// Unlike AC-03.1's, this one has a real population: every realised attachment
/// in the estate is compared on every run.
#[test]
fn every_realised_attachment_in_the_estate_still_matches_canon() {
  let root = repo_root();
  let project = Project::open(&root).expect("the real project opens");
  let report = intentsvcs::doctor::diagnose(&project, &common::ctx(), None);
  let canon = intentsvcs::ingest::read(&project).expect("canon reads");

  let realised = canon
    .threads
    .iter()
    .flat_map(|t| {
      let project = &project;
      t.attachments
        .iter()
        .map(move |a| project.thread_dir(&t.id).join(&a.path))
    })
    .filter(|p| p.exists())
    .count();

  let found: Vec<&Finding> = report
    .findings
    .iter()
    .filter(|f| f.class == FindingClass::AttachmentDrift)
    .collect();

  eprintln!(
    "AT-03.4: {realised} realised attachment(s) compared across {} thread(s), {} divergent",
    canon.threads.len(),
    found.len()
  );

  assert!(
    realised > 0,
    "no attachment in the estate is realised on disk, so this arm compared nothing and its green \
     is a statement about the corpus rather than about the check"
  );
  assert!(
    found.is_empty(),
    "{} attachment(s) in the estate diverge from canon:\n  {}",
    found.len(),
    found
      .iter()
      .map(std::string::ToString::to_string)
      .collect::<Vec<_>>()
      .join("\n  ")
  );
}
