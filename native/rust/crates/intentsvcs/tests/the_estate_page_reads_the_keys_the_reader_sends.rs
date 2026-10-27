//! ST0086 AT-04.7
//! ST0086 AT-04.6
//!
//! **THE ESTATE PAGE READS ONLY KEYS THE READER SENDS**, and its pipeline reads
//! the two it is drawn from: `recent` for what finished and `queue` for what
//! runs and waits (AC-04.7). The page renders `intent/estate-status@1` as it
//! arrives, so a key it reads that the reader never writes is a panel that is
//! silently empty, and one the reader renames is the same failure later.
//!
//! The page binds the status to `s` in every estate function, and nothing else
//! in the document reads through `s.`, so the scan keys on that spelling, in
//! CODE and never in a COMMENT, the way the wire-field test beside this one
//! scans for `answer.body.`. The keys are taken by SERIALISING the reader's own
//! answer rather than from a list, so they cannot drift from it.
//!
//! **A LANE'S ROWS READ ONLY KEYS THE READER SENDS ON A LANE**, its `queue`
//! among them, the asks waiting for the lane's type (AC-04.6). Both of the
//! page's lane loops bind a lane to `l`, so the same scan keys on `l.`, and the
//! lane's keys are taken by serialising a lane the reader built.
//!
//! **THE CONTROL LINE READS `controller_unreadable`** beside `controller`
//! (AC-04.6): a controller record that will not parse is named UNREADABLE,
//! never read as NO CONTROLLER, so the status arm requires that read too.

use std::collections::BTreeSet;

use testkit::workspace_root;

/// The one shell document intentd serves, read at run time.
///
/// **IT PANICS ON A MISSING FILE, DELIBERATELY**: an absent page must never
/// reach the assertions as a page that reads nothing.
fn page() -> String {
  let path = workspace_root()
    .join("crates")
    .join("intentd")
    .join("src")
    .join("shell.html");
  std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

/// Every key the page reads as `<binder>.<key>`, comment lines first stripped
/// so prose that quotes a key is no read: `s` is the status in every estate
/// function, and `l` a lane in both lane loops.
fn keys_read_through(binder: &str) -> BTreeSet<String> {
  let html = page();
  let code: String = html
    .lines()
    .filter(|line| !line.trim_start().starts_with("//"))
    .collect::<Vec<_>>()
    .join("\n");
  let read = format!("{binder}.");
  let mut found = BTreeSet::new();
  let mut rest = code.as_str();
  while let Some(at) = rest.find(read.as_str()) {
    let before = code.len() - rest.len() + at;
    rest = &rest[at + read.len()..];
    // The binder as a whole name: the character before it is no part of an
    // identifier or a member path, so `xs.` and `a.s.` are not reads of the
    // status, and `html.` is no read of a lane.
    let whole = code[..before]
      .chars()
      .next_back()
      .is_none_or(|c| !(c.is_ascii_alphanumeric() || c == '_' || c == '.'));
    let key: String = rest
      .chars()
      .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
      .collect();
    if whole && !key.is_empty() {
      found.insert(key);
    }
  }
  found
}

/// Every top-level key of the reader's answer, from an empty gathering joined
/// at a fixed instant: every key is present from WP-01, so the empty answer
/// carries them all.
fn keys_the_reader_sends() -> BTreeSet<String> {
  let status = intentsvcs::estate::join(
    intentsvcs::estate::Gathered::default(),
    time::OffsetDateTime::UNIX_EPOCH,
  );
  serde_json::to_value(status)
    .expect("the estate status serialises")
    .as_object()
    .expect("the estate status serialises to an object")
    .keys()
    .cloned()
    .collect()
}

/// Every key of a lane in the reader's answer, from a gathering whose slots
/// hold one lane and nothing else, joined at a fixed instant. The reader emits
/// every lane the slots carry, with a controller or without one, so the lone
/// lane serialises whole, its empty `queue` included.
fn lane_keys_the_reader_sends() -> BTreeSet<String> {
  let lane = intentsvcs::estate::slots::LaneRecord {
    schema: intentsvcs::estate::slots::SLOT_SCHEMA.to_string(),
    name: "cargo".to_string(),
    kind: "heavy".to_string(),
    capacity: 1,
    load_under: None,
    node: None,
  };
  let gathered = intentsvcs::estate::Gathered {
    slots: intentsvcs::estate::slots::Slots {
      lanes: vec![lane],
      ..Default::default()
    },
    ..Default::default()
  };
  let status = intentsvcs::estate::join(gathered, time::OffsetDateTime::UNIX_EPOCH);
  let value = serde_json::to_value(status).expect("the estate status serialises");
  value["lanes"][0]
    .as_object()
    .expect("the lone lane serialises to an object")
    .keys()
    .cloned()
    .collect()
}

#[test]
fn the_estate_page_reads_no_key_the_reader_never_sends() {
  let reads = keys_read_through("s");
  // **VACUITY GUARD**: a page that bound the status to another name would
  // read nothing here, and the subset check below would pass over an empty
  // set exactly when it had stopped checking. `controller_unreadable` is
  // required as well, so a control line that stops naming an unreadable
  // record fails here (AC-04.6).
  for key in [
    "recent",
    "queue",
    "holds",
    "sessions",
    "controller_unreadable",
  ] {
    assert!(
      reads.contains(key),
      "the page reads no `s.{key}`: the scan found {reads:?}"
    );
  }
  let sends = keys_the_reader_sends();
  let missing: Vec<&String> = reads.difference(&sends).collect();
  assert!(
    missing.is_empty(),
    "the estate page reads {missing:?}, which the reader never sends (it sends {sends:?})"
  );
}

#[test]
fn the_estate_page_reads_no_lane_key_the_reader_never_sends() {
  let reads = keys_read_through("l");
  // **VACUITY GUARD**, as above: a lane loop that bound its lane to another
  // name would read nothing here, and `queue` is the key AC-04.6 adds.
  for key in ["name", "type", "capacity", "queue"] {
    assert!(
      reads.contains(key),
      "the page reads no `l.{key}`: the scan found {reads:?}"
    );
  }
  let sends = lane_keys_the_reader_sends();
  let missing: Vec<&String> = reads.difference(&sends).collect();
  assert!(
    missing.is_empty(),
    "the estate page reads {missing:?} off a lane, which no lane carries (a lane carries {sends:?})"
  );
}
