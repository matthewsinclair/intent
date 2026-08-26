//! **THE CAP THAT REPLACED THE EXTENSION ALLOWLIST, AND THE ONE PROPERTY THAT
//! MATTERS MORE THAN THE NUMBER: THE CARRIER AND THE REPORT MUST AGREE.**
//!
//! `ATTACHMENT_EXTENSIONS` was `["md", "txt", "sh"]`, and it decided what could
//! be carried by asking the FILENAME -- one step above the place that decides
//! text-vs-opaque by DECODING. So the opaque-attachment machinery was complete
//! and unreachable: a `.pdf` was classified out before anything opened it.
//!
//! The replacement is a size, because size is the actual cost: the committed
//! canon extract carries attachment bytes inline, so an opaque row is base64 in
//! a versioned JSON file.
//!
//! **1 MiB is a placeholder and is deliberately not tuned** (vc, 2026-08-26).
//! It stands until attachment bytes live outside the extract. **Over the cap is
//! a refusal, never a deletion.**

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::project::{ATTACHMENT_CAP_BYTES, Project, ThreadFile};

const ID: &str = "ST0001";

fn estate_with(rel: &str, bytes: usize) -> Fixture {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread(ID));
  let path = fx.path(&format!("intent/st/{ID}")).join(rel);
  std::fs::create_dir_all(path.parent().expect("a file has a parent")).expect("mkdir");
  std::fs::write(&path, vec![b'x'; bytes]).expect("write the file");
  fx
}

fn carried_and_refused(fx: &Fixture) -> (Vec<String>, Vec<(String, String)>) {
  let (carried, refused) = fx.project().collect_attachments(ID);
  (carried.into_iter().map(|a| a.path).collect(), refused)
}

/// **THE BOUNDARY, LOW SIDE. A cap is a maximum, so exactly the cap CARRIES.**
///
/// Written as a pair with the arm below because an off-by-one is invisible to
/// any test that only tries one oversized file: `>` and `>=` both refuse a 2 MiB
/// file and both look correct from a single fixture.
#[test]
fn a_file_at_exactly_the_cap_is_carried() {
  let fx = estate_with("notes.md", ATTACHMENT_CAP_BYTES as usize);
  let (carried, refused) = carried_and_refused(&fx);

  assert!(
    carried.contains(&"notes.md".to_string()),
    "a file of exactly the cap is within it: carried={carried:?} refused={refused:?}"
  );
  assert!(refused.is_empty(), "and nothing is refused: {refused:?}");
}

/// **THE BOUNDARY, HIGH SIDE -- one byte over, and the refusal carries the
/// SIZE.**
///
/// The byte count is not decoration. A refusal that says only "too large" tells
/// an operator to go and look; one that says how large tells them whether the
/// file is a stray screenshot or the thing the thread is about.
#[test]
fn a_file_one_byte_over_the_cap_is_refused_and_named_with_its_size() {
  let over = ATTACHMENT_CAP_BYTES as usize + 1;
  let fx = estate_with("huge.png", over);
  let (carried, refused) = carried_and_refused(&fx);

  assert!(
    !carried.contains(&"huge.png".to_string()),
    "one byte over the cap must not be carried: {carried:?}"
  );
  let named = refused
    .iter()
    .find(|(name, _)| name.contains("huge.png"))
    .unwrap_or_else(|| panic!("the refusal must name the file: {refused:?}"));
  assert!(
    named.1.contains(&over.to_string()),
    "and must carry its byte count: {:?}",
    named.1
  );
}

/// **THE CARRIER AND THE REPORT MUST NAME THE SAME FILE. This is the arm the
/// whole design is for.**
///
/// `within_attachment_cap` exists as a function with ONE home precisely so
/// these two callers cannot drift. If `doctor` and `collect_attachments` ever
/// answered differently, a file would be refused by the carrier and unlisted by
/// the report -- **the silent gap the report exists to close, arriving through
/// the door built to close it.** Inline a second literal in either and this
/// goes red.
#[test]
fn the_carrier_refuses_exactly_what_the_report_names() {
  let fx = estate_with("huge.png", ATTACHMENT_CAP_BYTES as usize + 1);

  let (_, refused) = carried_and_refused(&fx);
  assert!(
    refused.iter().any(|(name, _)| name.contains("huge.png")),
    "the carrier refuses it: {refused:?}"
  );

  let report = intentsvcs::doctor::diagnose(&fx.project(), &common::ctx(), None);
  assert!(
    report.unattached.iter().any(|u| u.contains("huge.png")),
    "and the report names the same file, or the two have drifted: {:?}",
    report.unattached
  );
  assert!(
    report
      .unattached
      .iter()
      .any(|u| u.contains(&ATTACHMENT_CAP_BYTES.to_string())),
    "naming the cap it exceeded: {:?}",
    report.unattached
  );
}

/// **AND THE OTHER DIRECTION, WHICH IS THE HALF A SINGLE-SIDED TEST MISSES.**
///
/// A file UNDER the cap must be carried AND absent from the report. Without
/// this arm, a `doctor` that reported every attachment regardless of size would
/// pass the agreement test above by reporting everything.
#[test]
fn a_file_under_the_cap_is_carried_and_is_not_reported() {
  let fx = estate_with("small.png", 1024);

  let (carried, _) = carried_and_refused(&fx);
  assert!(
    carried.contains(&"small.png".to_string()),
    "carried: {carried:?}"
  );

  let report = intentsvcs::doctor::diagnose(&fx.project(), &common::ctx(), None);
  assert!(
    !report.unattached.iter().any(|u| u.contains("small.png")),
    "a carried file is not an uncovered one: {:?}",
    report.unattached
  );
}

/// **AN EXTENSION NO LIST EVER HELD IS CARRIED, WHICH IS THE POINT OF THE
/// CHANGE.**
///
/// `.tap` was excluded, and so were the fleet's `.tsv`, `.json`, `.yaml`,
/// `.cli`, `.py` and `.lensmd` -- **every one of them valid UTF-8, and every one
/// excluded because the gate asked the extension rather than the bytes.** This
/// project's own tree carries 196 `.tap` files that were pinned to disk by a
/// three-entry list.
#[test]
fn a_text_file_with_an_extension_no_list_ever_held_is_carried() {
  let fx = estate_with("parity/baseline.tap", 64);
  let (carried, refused) = carried_and_refused(&fx);

  assert!(
    carried.contains(&"parity/baseline.tap".to_string()),
    "carried={carried:?} refused={refused:?}"
  );
  assert_eq!(
    Project::classify(std::path::Path::new("parity/baseline.tap")),
    ThreadFile::Attachment,
    "and the classifier says so without opening the file"
  );
}
