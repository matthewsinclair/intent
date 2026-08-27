//! **IS THE RESIDUE CHECK BUCKET-BLIND, OR DID THE SCAN ABORT BEFORE REACHING
//! THE BUCKETS?** vc measured hop 2 on Lamplight naming 8 findings in 3 FLAT
//! threads and none in 10 BUCKETED ones, with perfect correlation to location,
//! and read it as a second instance of the attachment defect.
//!
//! The two explanations predict the SAME observation and need different fixes:
//! passing `dir` fixes one and does nothing for the other. `thread_dirs` yields
//! the top level BEFORE the buckets, and the row accounting returns `Err`, which
//! propagates out of the whole scan -- so the first refusing thread ends the run
//! and every thread after it goes unread, buckets included.
//!
//! These arms separate them by putting the SAME defective row in each location.

mod common;

use common::Fixture;
use intentsvcs::legacy;

const ROW: &str = "# Acceptance\n\n## Criteria\n\n\
  - AC-01.1 The field round-trips without loss. -- control: none\n\n\
  ## Tests\n\n\
  - AT-01.1 test/a_test.exs -- covers AC-01.1 -- status: green\n";

fn info(status: &str) -> String {
  format!(
    "---\nstatus: {status}\ncreated: 20260624\n---\n\n# A thread\n\n## Objective\n\nShip it.\n"
  )
}

fn estate(fixture: &Fixture, bucket: &str, id: &str, status: &str) {
  fixture.write_file(
    "intent/.config/config.json",
    "{\"intent_version\":\"2.19.0\",\"project_name\":\"P\",\"author\":\"cc\",\"intent_dir\":\"intent\",\"languages\":[\"rust\"]}\n",
  );
  fixture.write_file(&format!("intent/st/{bucket}{id}/info.md"), &info(status));
  fixture.write_file(&format!("intent/st/{bucket}{id}/acceptance.md"), ROW);
}

fn findings(scan: &legacy::Scan) -> Vec<String> {
  scan
    .residue
    .iter()
    .chain(scan.carried.iter())
    .map(|f| f.detail.clone())
    .collect()
}

/// The control: the same row in a FLAT thread is reported.
#[test]
fn a_flat_threads_unread_field_is_reported() {
  let fx = Fixture::new();
  estate(&fx, "", "ST0002", "WIP");
  let scan = legacy::scan(&fx.project()).expect("scan");
  let said = findings(&scan);
  assert!(
    said.iter().any(|d| d.contains("control")),
    "the flat thread's unread field is named: {said:?}"
  );
}

/// **THE DECISIVE ARM.** Same row, BUCKETED. If this is reported, the residue
/// check is not bucket-blind and vc's correlation has another cause.
#[test]
fn a_bucketed_threads_unread_field_is_reported_too() {
  let fx = Fixture::new();
  estate(&fx, "COMPLETED/", "ST0002", "Completed");
  let scan = legacy::scan(&fx.project()).expect("scan");
  let said = findings(&scan);
  assert!(
    said.iter().any(|d| d.contains("control")),
    "a COMPLETED/ thread's unread field is named exactly as a flat one is: {said:?}"
  );
}

/// **THE ORDER THAT MAKES AN ABORT LOOK LIKE BLINDNESS.** `thread_dirs` yields
/// the top level first, so a refusal in a flat thread ends the scan before any
/// bucketed thread is read -- and the surviving log names only flat paths.
#[test]
fn flat_threads_are_walked_before_bucketed_ones() {
  let fx = Fixture::new();
  estate(&fx, "", "ST0009", "WIP");
  fx.write_file("intent/st/COMPLETED/ST0001/info.md", &info("Completed"));
  fx.write_file("intent/st/COMPLETED/ST0001/acceptance.md", ROW);
  let scan = legacy::scan(&fx.project()).expect("scan");
  let ids: Vec<&str> = scan.threads.iter().map(|t| t.id.as_str()).collect();
  assert_eq!(
    ids,
    vec!["ST0009", "ST0001"],
    "flat first despite the higher id, then the buckets -- so an early refusal buries the buckets"
  );
}

/// **THE FIX'S OWN ARM: A REFUSAL NAMES EVERY THREAD, NOT THE FIRST.**
///
/// Two threads that cannot be read, one FLAT and one BUCKETED. `thread_dirs`
/// yields the flat one first, so under the old `?` the run ended there and the
/// bucketed one was never reached -- which is precisely the shape that read as
/// bucket-blindness on Lamplight. The refusal must name both.
///
/// The unreadable condition is a DIRECTORY where `acceptance.md` should be: a
/// real per-thread read failure that needs no permissions and no root.
#[test]
fn a_refusal_names_every_thread_that_could_not_be_accounted_for() {
  let fx = Fixture::new();
  estate(&fx, "", "ST0009", "WIP");
  fx.write_file("intent/st/COMPLETED/ST0001/info.md", &info("Completed"));
  fx.write_file("intent/st/COMPLETED/ST0001/acceptance.md", ROW);

  // Replace each acceptance.md with a directory of the same name.
  for rel in [
    "intent/st/ST0009/acceptance.md",
    "intent/st/COMPLETED/ST0001/acceptance.md",
  ] {
    let path = fx.path(rel);
    std::fs::remove_file(&path).expect("remove the file");
    std::fs::create_dir(&path).expect("a directory where the file was");
  }

  let refusal = legacy::scan(&fx.project())
    .expect_err("two unreadable threads must refuse")
    .to_string();

  assert!(
    refusal.contains("ST0009"),
    "the flat thread is named: {refusal}"
  );
  assert!(
    refusal.contains("ST0001"),
    "**THE BUCKETED THREAD IS NAMED TOO** -- under the old `?` the run ended at \
     the flat thread and this one was never read: {refusal}"
  );
  assert!(
    refusal.contains("2 thread(s)"),
    "the refusal counts what it names: {refusal}"
  );
}

/// **THE ARM THAT STOPS THE NEW REFUSAL BEING TOO STRICT.** A thread that never
/// had an `acceptance.md` is an ordinary thread, not a failure -- most v2
/// threads have none. Only `NotFound` is benign, and this pins that the narrow
/// case stayed narrow.
#[test]
fn a_thread_with_no_acceptance_file_is_not_a_refusal() {
  let fx = Fixture::new();
  estate(&fx, "", "ST0002", "WIP");
  std::fs::remove_file(fx.path("intent/st/ST0002/acceptance.md")).expect("remove it");

  let scan =
    legacy::scan(&fx.project()).expect("an absent acceptance file is a state, not an error");
  assert_eq!(scan.threads.len(), 1, "the thread still migrates");
  assert!(
    scan.threads[0].criteria.is_empty(),
    "with no criteria, which is the honest answer for a thread that declared none"
  );
}
