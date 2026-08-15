//! AT-03.1 / AC-03.1: strict ingest refuses schema-invalid canon with the
//! file and the finding named, and nothing is partially loaded.
//!
//! The last clause is the one worth testing hard. "Refuses" is easy to get
//! right and easy to test; "nothing partially loaded" is the property that
//! decides whether a failed ingest leaves a project queryable-but-wrong, and
//! it is invisible unless a test looks at the store AFTER the refusal.

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::finding::FindingClass;
use intentsvcs::ingest::{self, IngestError};
use intentsvcs::store::Store;

/// The findings a refusal carries, or a panic naming what came back instead.
fn refusal(err: IngestError) -> Vec<intentsvcs::finding::Finding> {
  match err {
    IngestError::Refused(r) => r.findings,
    other => panic!("expected a refusal, got: {other}"),
  }
}

#[test]
fn an_unknown_field_is_refused_by_name() {
  let fx = Fixture::new();
  fx.write_raw_thread(
    "ST0056",
    r#"{
  "schema": "intent/thread@3.0",
  "id": "ST0056",
  "title": "Intent v3.0.0",
  "status": "wip",
  "created": "2026-08-14",
  "bogus_field": 1
}
"#,
  );

  let findings = refusal(ingest::read(&fx.project()).expect_err("must refuse"));
  assert_eq!(findings.len(), 1, "one defect, one finding: {findings:?}");
  assert_eq!(findings[0].class, FindingClass::SchemaInvalid);
  assert_eq!(findings[0].file, "intent/st/ST0056/thread.json");
  assert!(
    findings[0].detail.contains("bogus_field"),
    "the refusal names the offending field, got: {}",
    findings[0].detail
  );
}

#[test]
fn a_bad_enum_value_is_refused_and_names_the_path() {
  let fx = Fixture::new();
  fx.write_raw_thread(
    "ST0056",
    r#"{
  "schema": "intent/thread@3.0",
  "id": "ST0056",
  "title": "Intent v3.0.0",
  "status": "in-progress",
  "created": "2026-08-14"
}
"#,
  );

  let findings = refusal(ingest::read(&fx.project()).expect_err("must refuse"));
  assert_eq!(findings[0].class, FindingClass::SchemaInvalid);
  assert!(
    findings[0].detail.contains("/status"),
    "the finding points at the instance path, got: {}",
    findings[0].detail
  );
}

#[test]
fn malformed_json_is_refused_with_its_line() {
  let fx = Fixture::new();
  fx.write_raw_thread(
    "ST0056",
    "{\n  \"schema\": \"intent/thread@3.0\",\n  oops\n}\n",
  );

  let findings = refusal(ingest::read(&fx.project()).expect_err("must refuse"));
  assert_eq!(findings[0].class, FindingClass::MalformedJson);
  assert_eq!(
    findings[0].line,
    Some(3),
    "malformed JSON is located, not merely reported"
  );
}

#[test]
fn an_id_that_disagrees_with_its_directory_is_refused() {
  let fx = Fixture::new();
  let mut thread = sample_thread("ST0056");
  thread.id = "ST0099".to_string();
  // Deliberately write ST0099's content into ST0056's directory.
  fx.write_raw_thread(
    "ST0056",
    &intentsvcs::model::to_canonical_json(&thread).expect("render"),
  );

  let findings = refusal(ingest::read(&fx.project()).expect_err("must refuse"));
  assert_eq!(findings[0].class, FindingClass::DuplicateId);
  assert!(
    findings[0].detail.contains("ST0099") && findings[0].detail.contains("ST0056"),
    "the finding names both ids, got: {}",
    findings[0].detail
  );
}

#[test]
fn every_finding_is_reported_never_only_the_first() {
  let fx = Fixture::new();
  fx.write_raw_thread(
    "ST0056",
    r#"{"schema":"intent/thread@3.0","id":"ST0056","title":"a","status":"wip","created":"2026-08-14","bogus_one":1}"#,
  );
  fx.write_raw_thread(
    "ST0057",
    r#"{"schema":"intent/thread@3.0","id":"ST0057","title":"b","status":"nope","created":"2026-08-14"}"#,
  );

  let findings = refusal(ingest::read(&fx.project()).expect_err("must refuse"));
  let files: Vec<&str> = findings.iter().map(|f| f.file.as_str()).collect();
  assert!(
    files.contains(&"intent/st/ST0056/thread.json")
      && files.contains(&"intent/st/ST0057/thread.json"),
    "both broken threads are named -- one fix-and-rerun cycle, not one per defect: {files:?}"
  );
}

/// The clause that matters: a refusal must leave the store exactly as it was.
#[test]
fn a_refusal_loads_nothing_into_the_store() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));

  let mut store = Store::open_in_memory().expect("open");
  ingest::load(&fx.project(), &mut store).expect("the good estate loads");
  let before = store.snapshot().expect("snapshot");
  assert_eq!(
    before["threads"].as_array().expect("threads").len(),
    1,
    "precondition: the good thread is loaded"
  );

  // Now add a second, broken thread and re-ingest.
  fx.write_raw_thread("ST0057", r#"{"schema":"intent/thread@3.0","id":"ST0057"}"#);
  let err = ingest::load(&fx.project(), &mut store).expect_err("must refuse");
  assert!(matches!(err, IngestError::Refused(_)));

  assert_eq!(
    store.snapshot().expect("snapshot"),
    before,
    "a refused ingest leaves the store byte-identical -- no half-loaded estate, and specifically NOT the valid subset, which would be a queryable project silently missing a thread"
  );
}

#[test]
fn a_valid_estate_ingests() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  fx.write_issue(&common::sample_issue(21));

  let canon = ingest::read(&fx.project()).expect("a valid estate ingests");
  assert_eq!(canon.threads.len(), 1);
  assert_eq!(canon.issues.len(), 1);
  assert_eq!(canon.threads[0].related.len(), 2);
  assert_eq!(
    canon.threads[0].tests[2]
      .legacy
      .as_ref()
      .map(|l| l.raw.as_str()),
    Some("test/some_test.exs::\"the named case\""),
    "a marked-legacy reference survives ingest verbatim"
  );
}
