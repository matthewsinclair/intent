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

use std::collections::BTreeSet;

use common::{Fixture, PROJECT_ID, sample_thread};
use intentsvcs::model::{
  AcceptanceMode, AcceptanceTest, AtKind, AtStatus, FiatRecord, ISSUE_SCHEMA, Invoker, Issue,
  IssueStatus, Thread, to_canonical_json,
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
  // **ST0066: the fiat record has to be SET here or the round trip proves
  // nothing about it** -- `Option` is `skip_serializing_if`, so an unset field
  // survives every encoder including a broken one, which is the whole reason
  // `every_modelled_field_is_exercised` exists and the reason it went red the
  // moment this field landed.
  //
  // **HOSTILE ON PURPOSE, like every other value in this fixture.** The reason
  // carries a pipe and a quote because a fiat record is free text written by a
  // human under pressure, and it is rendered into markdown tables by
  // `fiat_marker` -- so an unescaped pipe here is a broken row somewhere else.
  // The WORK PACKAGE carries `inherited_from` and the thread does not: that is
  // the cascade shape, and the two serde paths (absent and present) are
  // different journeys through a `skip_serializing_if` field.
  t.fiat = Some(FiatRecord {
    because: "closed on hv's word: the panel-survival half is unobservable by               unit test | and waiting on a live sitting"
      .to_string(),
    by: "hv".to_string(),
    at: "2026-08-15T09:30:00.000Z".to_string(),
    invoker: Invoker {
      tty: true,
      env: "darwin/arm64".to_string(),
    },
    inherited_from: None,
    inherited_event: None,
  });
  t.wps[0].fiat = Some(FiatRecord {
    because: "cascaded from the thread's close".to_string(),
    by: "hv".to_string(),
    at: "2026-08-15T09:30:00.000Z".to_string(),
    invoker: Invoker {
      tty: true,
      env: "darwin/arm64".to_string(),
    },
    inherited_from: Some(id.to_string()),
    inherited_event: None,
  });
  t.tests = vec![AcceptanceTest {
    fiat: None,
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
    // Hostile for the same reason the title is: this fixture exists to prove
    // the canonical JSON survives the characters that break naive quoting, and
    // a name is the field most likely to carry an apostrophe in real data.
    reporter: Some("Ma'tt \"the\" S|nclair".to_string()),
    // The body is where JSON's own escapes have to survive a round trip -- a
    // literal backslash, an embedded quote and the newlines that make a one-
    // line encoder look correct until it meets prose.
    body: "# 0007: an issue with a \"quote\"\n\n## Detail\n\nA backslash \\\\ and a tab\tin the middle.\n"
      .to_string(),
  }
}

/// **Canon written where the EXPORTER names it must be canon the READERS can
/// open** -- the two path builders compared by putting bytes on a disk between
/// them.
///
/// The defect this closes: `export::canon_parts` emitted `issues/46.json` while
/// `Project::issue_json` resolves `issues/0046.json`. Two spellings of one path,
/// and every consumer afterwards is on the second -- so a migrated project wrote
/// its issue canon, `issue_numbers()` enumerated it (the stem parse is tolerant
/// and reads both spellings as the same number), and the very next open failed
/// on a file that did not exist while the file that did sat beside it.
///
/// **It survived because both spellings were tested and neither test crossed
/// between them** (ic's diagnosis, and it is what decided where this test
/// lives). The test above writes canon THROUGH `Project::issue_json` and reads it
/// back, so it is green on the padded spelling. `export_round_trip.rs` compares
/// `canon_parts` to `canon_parts`, so it is green on the unpadded one -- it never
/// puts a byte at a path anything else resolves. Each side was internally
/// consistent and the boundary between them had no test at all, which is why
/// "it has no callers" and "it is invisible" were one fact rather than two.
///
/// So the guard sits AT the crossing rather than inside either side, and it is
/// two-sided by construction: a path the exporter invents that no reader
/// resolves fails the set equality from one direction, and a reader path the
/// exporter never writes fails it from the other. **Then it is DRIVEN** -- an
/// equality between two path builders is still two builders agreeing, and only
/// the resync proves the bytes are reachable.
#[test]
fn canon_written_where_the_exporter_names_it_is_canon_the_readers_can_open() {
  let fx = Fixture::new();
  let project = fx.project();
  let threads = vec![maximal_thread("ST0001")];
  // 1 and 46 rather than two neighbours: the defect was a MISSING pad, so a
  // fixture whose numbers are already four digits wide cannot see it.
  let issues = vec![issue(1), issue(46)];
  let bundle =
    intentsvcs::export::Bundle::new(PROJECT_ID, threads.clone(), issues.clone(), Vec::new());

  let intent_dir = project.intent_dir();
  let mut written = BTreeSet::new();
  for (rel, text) in intentsvcs::export::canon_parts(&bundle).expect("canon of the bundle") {
    let path = intent_dir.join(&rel);
    std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
    std::fs::write(&path, text).expect("write canon");
    // **THE EVENT LOG IS A BUNDLE MEMBER WITH NO WORKING-TREE HOME (D53).**
    // `intent/events.jsonl` is deleted and untracked; the log lives in the
    // store, and its file form -- the one AC-02.6's 1-1 mapping requires -- is
    // produced on demand by `export` rather than kept projected in the tree.
    // So the exporter naming it here is correct and no reader resolves it,
    // which is exactly the asymmetry this crossing check would otherwise flag.
    // **Its round trip is covered in `export_round_trip.rs`**, where both sides
    // are bundles, which is the only place the comparison is meaningful now.
    if rel == intentsvcs::event::JSONL {
      continue;
    }
    written.insert(path);
  }

  // The readers' side, built from the resolvers every later consumer uses and
  // never from the strings above.
  let mut resolved = BTreeSet::new();
  for t in &threads {
    resolved.insert(project.thread_json(&t.id));
  }
  for i in &issues {
    resolved.insert(project.issue_json(i.number));
  }

  assert_eq!(
    written, resolved,
    "the exporter and the readers disagree about where canon lives -- whichever \
     side is short, the estate has files nothing can open"
  );

  let mut store = intentsvcs::store::Store::open_in_memory().expect("store");
  intentsvcs::ingest::resync(&project, &mut store, &intentsvcs::sync::Scope::All)
    .expect("the canon the exporter wrote must be readable by ingest, unchanged");
  let (out_threads, out_issues) = store.load_canon().expect("load back");
  assert_eq!(
    out_threads.len(),
    threads.len(),
    "every thread was reachable"
  );
  assert_eq!(out_issues.len(), issues.len(), "every issue was reachable");
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
  intentsvcs::ingest::resync(&project, &mut store, &intentsvcs::sync::Scope::All).expect("resync");
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
