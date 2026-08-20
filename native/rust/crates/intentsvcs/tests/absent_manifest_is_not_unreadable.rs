//! AT-04.7, covering ST0057 AC-04.7 -- **the manifest's ABSENT state is
//! honoured by the verbs that ACT on it, not only by the one that REPORTS it.**
//!
//! **THE FIXTURE IS THE ABSENCE ITSELF, WHICH IS WHY THIS CANNOT REUSE THE
//! EXISTING ORGANIZE HARNESS.** Every other organize test in this suite
//! hand-writes `intent/.intentfiles` before calling the verb, and
//! `intentfiles_is_the_list.rs:60` says why in its own words -- *"Realise
//! everything first, with no manifest"* -- reaching realisation through
//! `sync_to_disk` because `organize` would not run. **That hand-write is the
//! workaround this row exists to remove**, so the estate under test here has
//! canon, realised views, and no manifest at all.
//!
//! **AND IT IS THE SHIPPED INITIAL CONDITION OF EVERY NEW PROJECT, NOT AN EDGE
//! CASE.** `intent init` writes no manifest, so `intent init` followed by
//! `intent organize` -- the first two commands anybody types -- answered rc=1
//! with `could not read .../intent/.intentfiles`.

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::organize::Mode;
use intentsvcs::sync::Scope;

/// An estate with canon, realised views, and **no `intent/.intentfiles`**.
///
/// Realisation is driven through `sync_to_disk` rather than `organize` for the
/// reason above: before this row, `organize` could not be reached from here at
/// all. **The absence is asserted rather than assumed** -- a fixture built to
/// have no manifest that turns out to have one makes every assertion below
/// vacuous in the passing direction, which is the trap `parity.md:338` records
/// against a "no git" fixture that sat inside a repository.
fn estate_with_no_manifest() -> Fixture {
  let fx = Fixture::new();
  for id in ["ST0001", "ST0002"] {
    fx.write_thread(&sample_thread(id));
  }
  fx.facade_on_disk()
    .sync_to_disk(&Scope::All)
    .expect("realise everything");

  let manifest = fx.project().intentfiles_path();
  assert!(
    !manifest.exists(),
    "the fixture is not manifest-less: {} exists, so this file measures something else",
    manifest.display()
  );
  fx
}

/// Every file under the steel-thread directory, as a sorted list.
///
/// **DERIVED BY WALKING THE FILESYSTEM, NEVER FROM THE REPORT.** The report is
/// the thing under test; comparing it to itself would pass on an `organize`
/// that removed the estate and forgot to mention it.
fn files_on_disk(fx: &Fixture) -> Vec<String> {
  fn walk(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
      return;
    };
    for entry in entries.filter_map(|e| e.ok()) {
      let path = entry.path();
      if path.is_dir() {
        walk(&path, out);
      } else {
        out.push(path);
      }
    }
  }
  let mut found = Vec::new();
  walk(&fx.project().st_dir(), &mut found);
  let mut rel: Vec<String> = found.iter().map(|p| fx.project().relative(p)).collect();
  rel.sort();
  rel
}

/// **ARM (a), FIRST HALF: ABSENT IS NOT AN ERROR.**
#[test]
fn organize_runs_on_an_estate_with_no_manifest() {
  let fx = estate_with_no_manifest();
  let report = fx
    .facade_on_disk()
    .organize(Mode::Apply)
    .expect("an absent manifest is nobody having said, which is not a failure");
  // It ran over a real population rather than over nothing.
  assert!(
    !report.unchanged.is_empty() || !report.hydrated.is_empty(),
    "organize returned Ok having considered no file at all, so this proves nothing: {report:?}"
  );
}

/// **ARM (a), SECOND HALF, AND IT IS THE ASSERTION THAT MATTERS: THE REMOVAL
/// COUNT IS ZERO.**
///
/// An arm that only checked `organize` does not error would pass on an
/// `organize` that runs and **deletes the estate** -- which is the exact
/// failure hv's ABSENT IS NOT EMPTY was written to prevent, so the exit code is
/// not the assertion here and the removal count is.
///
/// **`Mode::Apply`, NEVER `Preview`.** A preview removes nothing by
/// construction, so asserting zero removals from one is vacuous.
#[test]
fn an_absent_manifest_removes_nothing_and_leaves_every_view_in_place() {
  let fx = estate_with_no_manifest();

  // **THE POSITIVE CONTROL.** Zero removals is trivially true of an estate with
  // nothing in it, so the population that COULD have been removed is counted
  // first. Without this the test passes on a fixture that silently realised
  // nothing.
  let before = files_on_disk(&fx);
  assert!(
    before.len() >= 2,
    "the fixture realised {} files, so 'removed nothing' would be vacuous",
    before.len()
  );

  let report = fx
    .facade_on_disk()
    .organize(Mode::Apply)
    .expect("organize proceeds");

  assert!(
    report.dehydrated.is_empty(),
    "an absent manifest removed {} file(s): {:?}. Nobody has said, so everything is realised.",
    report.dehydrated.len(),
    report.dehydrated
  );
  assert!(
    report.pruned.is_empty(),
    "an absent manifest pruned {} director(ies): {:?}",
    report.pruned.len(),
    report.pruned
  );

  // And the estate is checked directly, because a report that under-counts its
  // own removals would satisfy both assertions above.
  // **A SUBSET, NOT AN EQUALITY, AND THE DIFFERENCE IS THE RULE ITSELF.**
  // Under `NothingSaid` everything is realised, so `organize` legitimately
  // HYDRATES what was not on disk yet -- here the attachments and prose that
  // `sync_to_disk` does not write. The criterion is *removes zero files and
  // leaves every previously-realised view in place*; an equality would fail on
  // the verb doing exactly what absence means.
  //
  // **THE FIRST VERSION OF THIS ASSERTION WAS AN EQUALITY AND I READ ITS
  // FAILURE BACKWARDS**, calling four ADDED files a silent removal, because
  // `assert_eq!(after, before)` prints `after` as `left`. The polarity is now
  // carried by the assertion's SHAPE rather than by whoever reads the output --
  // which is the only version that cannot be misread.
  let after = files_on_disk(&fx);
  let removed: Vec<&String> = before.iter().filter(|p| !after.contains(p)).collect();
  assert!(
    removed.is_empty(),
    "an absent manifest removed {} previously-realised file(s): {:?}. \
     The report claimed dehydrated={:?} pruned={:?}, so a non-empty list here \
     ALSO means the removal went unreported.",
    removed.len(),
    removed,
    report.dehydrated,
    report.pruned
  );
}

/// **ARM (b): A MANIFEST THAT EXISTS AND WILL NOT PARSE STILL REFUSES, AND THE
/// REFUSAL STILL NAMES THE PATH.**
///
/// Without this arm the fix for (a) is satisfied by **deleting the refusal**,
/// which converts a documented hard stop into a silent fail-open on exactly the
/// input where the grammar's refusal is load-bearing: a verb about to remove
/// files, acting on a declaration it could not read.
#[test]
fn a_manifest_that_will_not_parse_still_refuses_and_names_the_path() {
  let fx = estate_with_no_manifest();
  fx.write_file("intent/.intentfiles", "STEELTHREAD:ST0001\nNONSENSE\n");

  let err = fx
    .facade_on_disk()
    .organize(Mode::Apply)
    .expect_err("a manifest that does not parse must not be acted on");

  let rendered = err.to_string();
  assert!(
    rendered.contains(".intentfiles"),
    "the refusal does not name the file the operator has to fix: {rendered}"
  );
  // **THE LINE NUMBER SURVIVES THE PATH BEING ADDED.** The whole reason
  // `.intentfiles` faults are their own variant is that they carry the line,
  // and a refusal that gained a path by losing it would be a worse message
  // that passed the assertion above.
  assert!(
    rendered.contains("line 2"),
    "the refusal lost the line number: {rendered}"
  );
}

/// **ARM (c), AND IT IS THE ONE THE FIX WOULD OTHERWISE HAVE LEFT BEHIND: THE
/// SURVIVING REFUSAL MUST NOT STATE hv's RULE BACKWARDS.**
///
/// The remedy read *"an absent manifest declares nothing, so `organize` would
/// read the whole estate as undeclared"* -- **the pre-reversal reading hv
/// overturned**, four files from `Realised::declares`, whose own comment is
/// ABSENT IS NOT EMPTY and whose code answers `true` for everything. Absent
/// means realise everything; the message said absent means realise nothing.
///
/// **Arms (a) and (b) both pass with that string intact**: (a) removes the
/// absent path from the refusal entirely, and (b) is satisfied by any refusal
/// that names the path. So without this arm the fix lands and **the teaching
/// defect survives inside the arm that stays** -- in the first message a new v3
/// user ever sees, addressed to the one person with no other source.
#[test]
fn the_surviving_refusal_does_not_teach_the_reversed_rule() {
  use intentsvcs::facade::FacadeError;
  use intentsvcs::remedy::Remedy;

  let err = FacadeError::ManifestUnreadable {
    path: "intent/.intentfiles".to_string(),
    source: std::io::Error::other("a fault that is not absence"),
  };
  let remedy = err.remedy();

  // The banned claim. **Not sufficient on its own** -- an empty message passes
  // it -- which is why the positive below is asserted in the same test rather
  // than left to a neighbour that could be deleted separately.
  assert!(
    !remedy.contains("undeclared"),
    "the refusal still teaches the pre-reversal rule: {remedy}"
  );

  // **THE POSITIVE, WHICH IS THE ONE THAT PINS THE TEACHING.** The message has
  // to say what absence actually means, not merely avoid saying the wrong
  // thing.
  assert!(
    remedy.contains("not an error"),
    "the refusal does not say that a missing manifest is not an error: {remedy}"
  );
}
