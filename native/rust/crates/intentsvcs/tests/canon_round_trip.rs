//! AT-03.8 / AC-03.8: canon -> DB -> canon is BYTE-identical, per entity.
//!
//! ic's egest-symmetry proposal, homed in WP-03 by vc rather than at WP-10 for
//! a reason worth restating: an unreversible FIELD is cheap to change now and
//! expensive at migration, which is ic's own "worst possible moment" argument
//! applied to the schedule. **Restated for D01 as reversed, where it gets
//! sharper rather than weaker.** The old reading was that the DB is rebuildable
//! from committed canon and therefore disposable, so a lost field made
//! `rm intent/.cache` lossy. The DB is now truth and the committed canon is the
//! extract that carries it between machines (D34) -- so a field that does not
//! survive this trip is a field that cannot LEAVE, and the loss lands on the
//! clone rather than on the machine that had it. Either way it is silent,
//! because both sides stay internally consistent.
//!
//! **Bytes, not values, and that is the point.** `store_round_trip.rs`
//! already proves model -> DB -> model equality. This proves the stronger
//! thing: that what comes back OUT serialises to exactly the file that went
//! in -- so it also catches field order, number formatting, escaping, and
//! anything else that would make `intent` rewrite a file it had merely read.
//!
//! **The fixture is the hard part, and it is guarded.** A field that is empty
//! or `None` round-trips through anything, so a test with a tame fixture
//! passes while blind. `every_modelled_field_is_exercised` therefore reads the
//! GENERATED schema face and fails if any property is missing from the
//! fixture's canon -- which means adding a field to the model without adding
//! it here is a failing test, not a silent hole.

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::model::{
  AcceptanceMode, AcceptanceTest, AtKind, AtStatus, ISSUE_SCHEMA, Issue, IssueStatus, Thread,
  to_canonical_json,
};

/// Every optional field populated, on top of the shared markup-bearing
/// fixture. Deliberately built by MUTATING `sample_thread` rather than by
/// declaring a second one: a private copy would drift from the shared fixture
/// exactly when someone changed the shared one, which is the drift this test
/// exists to detect.
fn maximal_thread(id: &str) -> Thread {
  let mut t = sample_thread(id);
  t.completed = Some("2026-08-15".to_string());
  t.acceptance = Some(AcceptanceMode::Exempt);
  t.tests = vec![AcceptanceTest {
    id: "AT-03.8".to_string(),
    kind: AtKind::Test,
    file: Some("crates/intentsvcs/tests/canon_round_trip.rs".to_string()),
    prose: Some("a prose reference carrying a `path` and a | pipe".to_string()),
    covers: vec!["AC-03.8".to_string()],
    status: AtStatus::Green,
    note: Some("a note carrying a `path` and a | pipe".to_string()),
    legacy: None,
  }];
  t
}

fn issue(number: u32) -> Issue {
  Issue {
    schema: ISSUE_SCHEMA.to_string(),
    number,
    slug: "a-pipe-|-and-a-quote".to_string(),
    title: "An issue with a | pipe, a \"quote\" and a 'tick'".to_string(),
    status: IssueStatus::Closed,
    severity: Some("high".to_string()),
    created: "2026-08-14".to_string(),
    closed: Some("2026-08-15".to_string()),
  }
}

/// Write canon, rebuild the store from it, read the model back out, and
/// re-serialise. The bytes must be the file.
#[test]
fn canon_survives_the_store_byte_for_byte() {
  let fx = Fixture::new();
  let project = fx.project();

  let threads = [maximal_thread("ST0001"), maximal_thread("ST0002")];
  for t in &threads {
    let path = project.thread_json(&t.id);
    std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
    std::fs::write(&path, to_canonical_json(t).expect("serialise")).expect("write canon");
  }
  let issues = [issue(1), issue(2)];
  std::fs::create_dir_all(project.issues_dir()).expect("mkdir issues");
  for i in &issues {
    std::fs::write(
      project.issue_json(i.number),
      to_canonical_json(i).expect("serialise"),
    )
    .expect("write issue");
  }

  let mut store = intentsvcs::store::Store::open_in_memory().expect("store");
  intentsvcs::ingest::resync(&project, &mut store).expect("resync");
  let (out_threads, out_issues) = store.load_canon().expect("load back");

  assert_eq!(out_threads.len(), threads.len(), "every thread came back");
  assert_eq!(out_issues.len(), issues.len(), "every issue came back");

  for thread in &out_threads {
    let on_disk = std::fs::read_to_string(project.thread_json(&thread.id)).expect("read canon");
    assert_eq!(
      to_canonical_json(thread).expect("serialise"),
      on_disk,
      "{} does not survive the store byte for byte -- the DB is only safe to \
       delete if what it gives back IS the file",
      thread.id
    );
  }
  for issue in &out_issues {
    let on_disk = std::fs::read_to_string(project.issue_json(issue.number)).expect("read issue");
    assert_eq!(
      to_canonical_json(issue).expect("serialise"),
      on_disk,
      "issue {}",
      issue.number
    );
  }
}

/// THE GUARD ON THE FIXTURE. Every property the generated schema face declares
/// must actually appear in the canon this test round-trips.
///
/// Without it the test above is only as good as my memory of the model: an
/// optional field left `None` is omitted from the canon entirely, so it
/// round-trips perfectly by not existing, and the day someone adds a field
/// that the DDL drops, this file goes green anyway. Reading the FACE rather
/// than the struct means the check is against the schema that is actually
/// published.
#[test]
fn every_modelled_field_is_exercised() {
  for (face, canon) in [
    (
      "thread.schema.json",
      to_canonical_json(&maximal_thread("ST0001")).expect("serialise"),
    ),
    (
      "issue.schema.json",
      to_canonical_json(&issue(1)).expect("serialise"),
    ),
  ] {
    let schema: serde_json::Value = serde_json::from_str(
      &intentsvcs::faces::face(face).unwrap_or_else(|| panic!("no `{face}` face")),
    )
    .expect("the face is JSON");
    let properties = schema
      .get("properties")
      .and_then(|p| p.as_object())
      .unwrap_or_else(|| panic!("the `{face}` face declares no properties"));
    assert!(
      properties.len() > 3,
      "a face with almost no properties means the lookup is wrong, not that \
       the model is small"
    );

    let value: serde_json::Value = serde_json::from_str(&canon).expect("canon is JSON");
    let present = value.as_object().expect("canon is an object");
    let missing: Vec<&String> = properties
      .keys()
      .filter(|k| !present.contains_key(*k))
      .collect();
    assert!(
      missing.is_empty(),
      "the `{face}` fixture leaves these fields unset, so the round trip \
       proves nothing about them: {missing:?}"
    );
  }
}

/// A field the store drops must fail HERE, loudly, rather than at migration.
///
/// Demonstrated rather than asserted: the round trip is only a real check if a
/// value that goes missing actually breaks it, and the cheapest honest way to
/// show that is to remove a value from what the store gives back and confirm
/// the comparison notices.
#[test]
fn a_dropped_field_is_caught_by_the_comparison() {
  let intact = maximal_thread("ST0001");
  let mut lossy = intact.clone();
  lossy.tests[0].note = None;

  assert_ne!(
    to_canonical_json(&intact).expect("serialise"),
    to_canonical_json(&lossy).expect("serialise"),
    "if losing a nested optional field did not change the bytes, the whole \
     comparison above would be incapable of failing"
  );
}
