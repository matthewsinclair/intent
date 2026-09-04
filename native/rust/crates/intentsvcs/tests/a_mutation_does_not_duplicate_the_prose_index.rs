//! **THE PROSE INDEX IS A REPLACEMENT, NOT AN ACCUMULATION** (issue 0234).
//!
//! Every mutation recomputes `doc_sections` alongside the model, because work
//! package text is DERIVED FROM CANON (D28) and an index left behind would let
//! `intent search` answer from the previous model. The recompute kept the
//! sections it did not derive and re-derived the rest -- and the two sets were
//! named by a filter on `owner_type` that only ever excluded `work-package`.
//!
//! The emitter was called `collect_wp_text` and emits THREE kinds of section:
//! a thread's own objective and context, one per text attachment, and one per
//! work package. The first two carry `owner_type: "thread"`, so they were kept
//! from canon AND re-derived -- **one extra copy of every thread-owned section
//! on every mutation, for the life of the store.** It is `collect_thread_prose`
//! now, with the predicate naming what it derives beside it, because a filter
//! written against the old name was a reasonable mistake to make.
//!
//! Measured on the estate before the fix, rows against distinct
//! `(file, owner_id, seq)`: Baize 1.0x (a store with no mutations since it was
//! built), Conflab 5.9x, Intent 8.9x, Lamplight 13.1x, Laksa 56.6x -- whose
//! store had reached 2.3 GB for 6.4 MB of canon. **The factor tracks how often
//! a project has been WRITTEN, not how much it holds.**
//!
//! # Why this asserts uniqueness rather than a count
//!
//! A count assertion pins the arithmetic of one mechanism. **This file was
//! written after the mechanism in the filing turned out to be the wrong one**
//! -- the report blamed `Store::write_doc_sections`, whose `DELETE` is fine --
//! so the property is stated in the terms the defect is actually about:
//! **an address appears once.** That holds for any future writer, catches a
//! duplicate arriving through a route nobody has thought of yet, and needs no
//! revision when a fixture grows a section.
//!
//! It is also the exact question the estate measurement asked, which is what
//! makes the 1.0x control above comparable to a green here.

use crate::common::{Fixture, sample_thread};
use std::collections::BTreeMap;

/// Every section's address, and how many rows claim it.
fn by_address(fx: &Fixture, facade: &intentsvcs::facade::Facade) -> BTreeMap<String, usize> {
  let _ = fx;
  let mut seen = BTreeMap::new();
  for s in facade.store().doc_sections().expect("read the prose index") {
    *seen
      .entry(format!("{}|{}|{}", s.file, s.owner_id, s.seq))
      .or_insert(0) += 1;
  }
  seen
}

fn duplicates(index: &BTreeMap<String, usize>) -> Vec<(&String, &usize)> {
  index.iter().filter(|(_, n)| **n > 1).collect()
}

#[test]
fn mutations_do_not_accumulate_copies_of_a_thread_s_prose() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));

  // **ONE FACADE ACROSS EVERY MUTATION, AND THAT IS THE FIXTURE'S WHOLE
  // DIFFICULTY.** Opening a facade ingests canon from disk, which replaces the
  // index wholesale -- so a test that reopened between mutations would reset
  // the very thing it is measuring and pass against the broken writer.
  let mut facade = fx.facade();

  let baseline = by_address(&fx, &facade);
  assert!(
    duplicates(&baseline).is_empty(),
    "a freshly ingested index already had duplicates, so this fixture cannot measure the defect: {:?}",
    duplicates(&baseline)
  );
  let thread_sections = baseline.len();

  for n in 1..=3 {
    facade
      .issue_add(&format!("a filing, number {n}"), None, None, "prose")
      .expect("the create door works");
  }

  let after = by_address(&fx, &facade);
  assert!(
    duplicates(&after).is_empty(),
    "three mutations left duplicate prose rows -- the index accumulated instead of being replaced: {:?}",
    duplicates(&after)
  );

  // The issues legitimately ADD addresses; nothing may multiply an existing
  // one. Stated as a floor rather than an equality so a fixture that grows a
  // section does not red a test about duplication.
  // **THE CONTROL THAT KEEPS THE ASSERTION ABOVE FROM PASSING VACUOUSLY**, and
  // it pins the other half of the same defect. Nothing outside `ingest::read`
  // used to split an issue body, so a body written by `issues add` sat in the
  // model, rendered everywhere a human looks, and reached the FTS index only
  // at the next full ingest. If the mutation path stopped rewriting the index
  // altogether, the duplicate check would go green for the wrong reason; this
  // arm can only pass if the mutation actually re-derived it.
  let issue_rows = facade
    .store()
    .doc_sections()
    .expect("read the prose index")
    .into_iter()
    .filter(|s| s.owner_type == "issue")
    .count();
  assert_eq!(
    issue_rows, 3,
    "three issue bodies were written; the mutation-time index holds {issue_rows} of them"
  );
  assert!(
    after.len() > thread_sections,
    "the index did not grow by the three new addresses, so the duplicate check above measured a stale index"
  );
}
