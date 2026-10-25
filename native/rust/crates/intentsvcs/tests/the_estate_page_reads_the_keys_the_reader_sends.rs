//! ST0086 AT-04.7
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

/// Every key the page reads off the status as `s.<key>`, comment lines first
/// stripped so prose that quotes a key is no read.
fn keys_the_page_reads() -> BTreeSet<String> {
  let html = page();
  let code: String = html
    .lines()
    .filter(|line| !line.trim_start().starts_with("//"))
    .collect::<Vec<_>>()
    .join("\n");
  let mut found = BTreeSet::new();
  let mut rest = code.as_str();
  while let Some(at) = rest.find("s.") {
    let before = code.len() - rest.len() + at;
    rest = &rest[at + 2..];
    // `s` as a whole name: the character before it is no part of an identifier
    // or a member path, so `xs.` and `a.s.` are not reads of the status.
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

#[test]
fn the_estate_page_reads_no_key_the_reader_never_sends() {
  let reads = keys_the_page_reads();
  // **VACUITY GUARD**: a page that bound the status to another name would
  // read nothing here, and the subset check below would pass over an empty
  // set exactly when it had stopped checking.
  for key in ["recent", "queue", "sessions"] {
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
