//! AT-01.3 / ST0057 AC-01.3: **the relocation is lossless -- every FIELD of
//! every artefact round-trips through the move byte-identically**, with the
//! field count printed as the denominator.
//!
//! # Why a field denominator and not an artefact one
//!
//! Counting artefacts answers "did all 97 files survive", which they visibly
//! do -- the move is a rename and a rename does not drop files. **The loss
//! this criterion is about is one level down: a FIELD that the writer at the
//! new location emits and the reader there does not take back.** 97 of 97
//! artefacts present, one field silently absent from each, is a green under
//! any artefact-level count and is data loss at the clone boundary (ST0056
//! AC-02.6 applied to the move).
//!
//! So the denominator is every leaf in the serialised form, enumerated from
//! the artefacts themselves rather than from a list kept here. A hand-kept
//! field roster is one someone must extend on the day they add a field, which
//! is the day they are thinking about something else -- the argument
//! `openness.rs` makes for enumerating tables from the DDL.
//!
//! # It is a round trip through the RESOLVER, not through serde
//!
//! Serialising a struct and parsing it back tests serde and would pass on a
//! project that had never been moved at all. What is under test is the pair of
//! ends the move created: `export::canon_parts` names each file through
//! `canon_thread_rel` / `canon_issue_rel`, and `ingest::read` opens it through
//! the same resolver. **A field lost between those two is lost in exactly the
//! place the relocation put it**, and nothing else in this thread looks there
//! -- `canon_relocation.rs` asserts WHERE canon lives and
//! `canon_resolver_singularity.rs` asserts that one resolver answers, and both
//! pass with a field missing from every file.
//!
//! # The corpus is the real estate
//!
//! 57 threads and 40 issues, carrying the fields people have actually
//! authored -- optional ones populated, empty strings, nested criteria and
//! tests, attachments with paths that have separators in them. A constructed
//! fixture would be built by someone who already knew which fields to worry
//! about, which is the sampling error this thread has paid for before.

mod common;

use common::Fixture;
use intentsvcs::export::{self, Bundle};
use intentsvcs::ingest;
use intentsvcs::project::Project;
use serde_json::Value;
use std::collections::BTreeMap;
use testkit::repo_root;

/// Every leaf of a serialised artefact, keyed by its dotted path.
///
/// Arrays are indexed rather than summarised, so a reordering or a dropped
/// element is a differing key rather than a differing count -- the failure
/// names the field instead of the artefact.
fn leaves(value: &Value, prefix: &str, out: &mut BTreeMap<String, String>) {
  match value {
    Value::Object(map) => {
      for (key, inner) in map {
        leaves(inner, &format!("{prefix}.{key}"), out);
      }
    }
    Value::Array(items) => {
      for (i, inner) in items.iter().enumerate() {
        leaves(inner, &format!("{prefix}[{i}]"), out);
      }
    }
    // A leaf. `to_string` rather than the display form so `"1"` and `1` are
    // distinguishable -- a field that changes TYPE across the move is a loss
    // that a stringly comparison would pass.
    other => {
      out.insert(prefix.to_string(), other.to_string());
    }
  }
}

fn field_map<T: serde::Serialize>(id: &str, artefact: &T) -> BTreeMap<String, String> {
  let value = serde_json::to_value(artefact).expect("an artefact serialises");
  let mut out = BTreeMap::new();
  leaves(&value, id, &mut out);
  out
}

/// Every field of `before` checked against `after`, returning the number
/// EXAMINED and the differences by name.
///
/// **A function of two models, so the discriminating case below is a standing
/// assertion rather than a mutation somebody ran by hand once and did not
/// leave behind.**
fn compare(before: &ingest::Canon, after: &ingest::Canon) -> (usize, Vec<String>) {
  let mut examined = 0usize;
  let mut out: Vec<String> = Vec::new();

  let mut check = |id: &str, a: BTreeMap<String, String>, b: BTreeMap<String, String>| {
    examined += a.len();
    for (field, value) in &a {
      match b.get(field) {
        None => out.push(format!("{field}: LOST -- present before, absent after")),
        Some(other) if other != value => {
          out.push(format!("{field}: CHANGED -- {value} -> {other}"))
        }
        Some(_) => {}
      }
    }
    for field in b.keys() {
      if !a.contains_key(field) {
        out.push(format!("{field}: INVENTED -- absent before, present after"));
      }
    }
    let _ = id;
  };

  for original in &before.threads {
    let returned = after
      .threads
      .iter()
      .find(|t| t.id == original.id)
      .unwrap_or_else(|| panic!("{} is absent after the round trip", original.id));
    check(
      &original.id,
      field_map(&original.id, original),
      field_map(&original.id, returned),
    );
  }

  for original in &before.issues {
    let id = format!("issue-{:04}", original.number);
    let returned = after
      .issues
      .iter()
      .find(|i| i.number == original.number)
      .unwrap_or_else(|| panic!("{id} is absent after the round trip"));
    check(
      &id,
      field_map(&id, original),
      field_map(&id, returned),
    );
  }

  (examined, out)
}

#[test]
fn every_field_of_every_artefact_survives_the_relocation() {
  let root = repo_root();
  let project = Project::open(&root).expect("the real project opens");
  let before = ingest::read(&project).expect("canon reads from the real estate");

  assert!(
    !before.threads.is_empty() && !before.issues.is_empty(),
    "precondition: the corpus is {} thread(s) and {} issue(s) -- a round trip over an empty \
     estate compares two empty maps and proves nothing",
    before.threads.len(),
    before.issues.len()
  );

  // Write the whole model out through the resolver, into a project that has
  // never held canon at any other location.
  let fx = Fixture::new();
  let bundle = Bundle::new(
    "roundtrip",
    before.threads.clone(),
    before.issues.clone(),
    Vec::new(),
  );
  let parts = export::canon_parts(&bundle).expect("canon serialises");
  for (rel, body) in &parts {
    fx.write_file(&format!("intent/{rel}"), body);
  }

  let after = ingest::read(&fx.project()).expect("canon reads back from the resolved location");

  // ---- artefact-level conservation, which is necessary and not sufficient --
  assert_eq!(
    after.threads.len(),
    before.threads.len(),
    "threads did not survive the move as artefacts, so the field comparison below would be over \
     a smaller set and would pass for the wrong reason"
  );
  assert_eq!(
    after.issues.len(),
    before.issues.len(),
    "issues did not survive the move as artefacts"
  );

  // ---- the field-level comparison, which is the criterion ------------------
  let (examined, differences) = compare(&before, &after);

  // **The denominator is printed whether or not anything failed.** A pass that
  // does not say how much it examined is indistinguishable from a pass over
  // nothing, which is the shape this thread keeps meeting.
  eprintln!(
    "AT-01.3: {} field(s) compared across {} thread(s) and {} issue(s), {} file(s) written \
     through the resolver",
    examined,
    before.threads.len(),
    before.issues.len(),
    parts.len()
  );

  assert!(
    examined > before.threads.len() + before.issues.len(),
    "the enumeration found {examined} field(s) over {} artefact(s), which is at most one each -- \
     `leaves` is not descending, so every comparison below it is vacuous",
    before.threads.len() + before.issues.len()
  );

  assert!(
    differences.is_empty(),
    "{} field(s) did not survive the relocation, out of {examined} compared:\n  {}",
    differences.len(),
    differences.join("\n  ")
  );
}

/// **THE DISCRIMINATING CASE: a field dropped in transit is reported BY NAME.**
///
/// Without this the green above is unfalsifiable -- a `compare` that returned
/// an empty list unconditionally would satisfy it perfectly, and so would a
/// `leaves` that stopped descending.
///
/// **It drops an OPTIONAL field, and that is the whole point rather than a
/// convenience.** A required field removed from a canon file is caught by
/// schema validation, so it refuses loudly and never reaches the comparison --
/// which means the LOST arm is only ever exercised by optional fields, and
/// optional fields are exactly where silent loss lives: `skip_serializing_if`
/// makes "absent" and "None" the same bytes, so a writer that forgot one and a
/// model that never had one are indistinguishable on disk.
#[test]
fn a_field_dropped_in_transit_is_reported_by_name() {
  let root = repo_root();
  let project = Project::open(&root).expect("the real project opens");
  let before = ingest::read(&project).expect("canon reads");

  let victim = before
    .threads
    .iter()
    .find(|t| t.slug.is_some())
    .expect("precondition: some thread carries a slug, or this arm drops nothing")
    .clone();

  let fx = Fixture::new();
  let bundle = Bundle::new(
    "roundtrip",
    before.threads.clone(),
    before.issues.clone(),
    Vec::new(),
  );
  for (rel, body) in export::canon_parts(&bundle).expect("canon serialises") {
    let body = if rel.contains(&victim.id) {
      let mut value: Value = serde_json::from_str(&body).expect("a canon part is JSON");
      let removed = value
        .as_object_mut()
        .expect("a thread is an object")
        .remove("slug");
      assert!(
        removed.is_some(),
        "the mutation removed nothing, so this arm proves the comparator can see a loss that \
         never happened"
      );
      serde_json::to_string_pretty(&value).expect("re-serialises")
    } else {
      body
    };
    fx.write_file(&format!("intent/{rel}"), &body);
  }

  let after = ingest::read(&fx.project()).expect("the mutated estate still reads");
  let (examined, differences) = compare(&before, &after);

  assert!(
    examined > 0,
    "nothing was examined, so an empty difference list would mean nothing"
  );
  assert_eq!(
    differences.len(),
    1,
    "exactly one field was dropped and the comparison reports {} difference(s): {:?}",
    differences.len(),
    differences
  );
  assert!(
    differences[0].starts_with(&format!("{}.slug: LOST", victim.id)),
    "the loss is reported, but not by the name of the field that was lost: {}",
    differences[0]
  );
}
