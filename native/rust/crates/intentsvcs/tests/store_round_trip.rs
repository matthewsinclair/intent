//! The store round trip: `rebuild` then `load_canon` returns exactly what went
//! in.
//!
//! **This is the property that makes the DB safe as the daily driver.** hv,
//! 2026-08-14: "all cli commands are going to go to the intentsvcs -- db
//! route, not to/from the file versions ... for performance and model
//! integrity, we're hitting the db as the daily driver." The moment commands
//! answer from the store instead of re-parsing canon, anything the round trip
//! drops becomes a question the DB answers differently from the files -- and
//! it would answer it confidently, with no error anywhere, because both sides
//! are internally consistent.
//!
//! **Under D01 as reversed the property is the same and its job changed.** The
//! note here used to say committed canon was durable truth and the store merely
//! rebuildable from it. It is the other way round: the DB is truth, the canon is
//! the extract, and losslessness is no longer a licence to stop paying for a
//! rebuild -- it is AC-02.6's openness requirement, checked at the store
//! boundary. A field that does not survive is one that cannot travel (D34).
//!
//! Asserted against the markup-bearing fixture rather than a tame one. A
//! fixture of short ASCII titles would round-trip through almost any encoding
//! bug: the interesting values are the ones carrying pipes, newlines, quotes
//! and JSON-in-TEXT columns.

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::model::{
  AcState, AcceptanceTest, AtKind, AtStatus, ISSUE_SCHEMA, Issue, IssueStatus, Legacy,
};
use intentsvcs::store::Store;

fn issue(number: u32) -> Issue {
  Issue {
    schema: ISSUE_SCHEMA.to_string(),
    number,
    slug: "a-pipe-|-and-a-quote".to_string(),
    title: "An issue with a | pipe, a \"quote\" and a 'tick'".to_string(),
    status: IssueStatus::Open,
    severity: Some("high".to_string()),
    created: "2026-08-14".to_string(),
    closed: None,
    // An apostrophe in a value bound as a SQL parameter -- the one that would
    // matter if any of these statements were ever built by interpolation.
    reporter: Some("Ma'tt \"the\" S|nclair".to_string()),
  }
}

/// The whole model, out and back, unchanged.
#[test]
fn rebuild_then_load_returns_exactly_what_went_in() {
  let threads = vec![sample_thread("ST0056"), sample_thread("ST0057")];
  let issues = vec![issue(1), issue(21)];

  let mut store = Store::open_in_memory().expect("store");
  store.rebuild(&threads, &issues).expect("rebuild");
  let (back_threads, back_issues) = store.load_canon().expect("load");

  assert_eq!(
    back_threads, threads,
    "a thread did not survive the round trip"
  );
  assert_eq!(back_issues, issues, "an issue did not survive");
}

/// Every AC scope variant, including the ones carrying payload.
///
/// `scope` is a tagged enum stored as JSON in a TEXT column, so it is the
/// field most likely to lose its payload and least likely to complain: a
/// descope that came back as plain in-scope would silently un-descope a
/// requirement.
#[test]
fn every_ac_scope_variant_survives_with_its_payload() {
  let mut thread = sample_thread("ST0056");
  thread.criteria[0].state = AcState::Descoped {
    to: "ST0057".to_string(),
    by: Some("hv".to_string()),
    reason: Some("moved, with a | pipe in the reason".to_string()),
  };
  thread.criteria[1].state = AcState::Withdrawn {
    reason: "the premise did not reproduce".to_string(),
    by: Some("vc".to_string()),
  };

  let mut store = Store::open_in_memory().expect("store");
  store.rebuild(std::slice::from_ref(&thread), &[]).unwrap();
  let (back, _) = store.load_canon().expect("load");

  assert_eq!(back[0].criteria[0].state, thread.criteria[0].state);
  assert_eq!(back[0].criteria[1].state, thread.criteria[1].state);
}

/// A legacy-carried AT keeps its verbatim raw reference.
///
/// `Legacy::raw` is evidence, not data -- migration.md is explicit that it is
/// never parsed and never rewritten. A round trip that dropped it would
/// destroy the one thing the carry policy exists to preserve.
#[test]
fn a_legacy_carried_test_keeps_its_raw_reference() {
  let mut thread = sample_thread("ST0056");
  thread.tests.push(AcceptanceTest {
    id: "AT-99.1".to_string(),
    kind: AtKind::Test,
    file: None,
    prose: None,
    covers: vec!["AC-03.1".to_string()],
    status: AtStatus::Green,
    note: None,
    legacy: Some(Legacy {
      raw: "tests/unit/a.bats::some name, tests/unit/b.bats".to_string(),
    }),
  });

  let mut store = Store::open_in_memory().expect("store");
  store.rebuild(std::slice::from_ref(&thread), &[]).unwrap();
  let (back, _) = store.load_canon().expect("load");

  let carried = back[0]
    .tests
    .iter()
    .find(|t| t.id == "AT-99.1")
    .expect("the legacy row survived");
  assert_eq!(
    carried.legacy.as_ref().map(|l| l.raw.as_str()),
    Some("tests/unit/a.bats::some name, tests/unit/b.bats"),
    "the verbatim reference is evidence and must come back byte-identical"
  );
  assert!(
    carried.file.is_none(),
    "and nothing was invented for `file`"
  );
}

/// Order is preserved. ACs and ATs are ordered collections in the contract
/// view, so a round trip that returned them in a different order would make
/// `acceptance.md` regenerate differently and read as skew forever.
#[test]
fn collection_order_survives() {
  let thread = sample_thread("ST0056");
  let mut store = Store::open_in_memory().expect("store");
  store.rebuild(std::slice::from_ref(&thread), &[]).unwrap();
  let (back, _) = store.load_canon().expect("load");

  let ids = |t: &intentsvcs::model::Thread| -> Vec<String> {
    t.criteria.iter().map(|c| c.id.clone()).collect()
  };
  assert_eq!(ids(&back[0]), ids(&thread), "criteria order");
  let ats = |t: &intentsvcs::model::Thread| -> Vec<String> {
    t.tests.iter().map(|x| x.id.clone()).collect()
  };
  assert_eq!(ats(&back[0]), ats(&thread), "test order");
  assert_eq!(
    back[0].related, thread.related,
    "related order, which is a seq column precisely so it is not incidental"
  );
}

/// An empty store loads as an empty model rather than failing.
#[test]
fn an_empty_store_round_trips_to_an_empty_model() {
  let store = Store::open_in_memory().expect("store");
  let (threads, issues) = store.load_canon().expect("load");
  assert!(threads.is_empty() && issues.is_empty());
}

/// The round trip goes through the FILE canon too: what ingest reads and what
/// the store returns must be the same model.
///
/// The tests above compare the store against an in-memory fixture. This one
/// closes the loop the tool actually walks -- canon on disk, ingested, stored,
/// read back -- so a defect in either half is caught by a route that exercises
/// both.
#[test]
fn canon_on_disk_ingested_and_read_back_is_the_same_model() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let project = fx.project();

  let canon = intentsvcs::ingest::read(&project).expect("ingest");
  let mut store = Store::open_in_memory().expect("store");
  store.rebuild(&canon.threads, &canon.issues).expect("build");
  let (back, _) = store.load_canon().expect("load");

  assert_eq!(
    back, canon.threads,
    "the model the store returns is the model the files hold"
  );
}
