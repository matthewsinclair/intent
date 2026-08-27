//! ST0057 **AT-14.7** -- covering **AC-14.7**: `Facade::todo_flush` records the
//! history and the state in ONE transaction, so neither can land without the
//! other.
//!
//! # This criterion was UNBUILT, not uncovered, and the record said otherwise
//!
//! Until 2026-08-27 the flush wrote its two records through two doors:
//! `apply()` reached `Store::commit_mutation`, which **committed** its
//! transaction with the `todo.flush` event in it, and only then did a separate
//! unwrapped `INSERT` set the watermark. **Two transactions, in that order,
//! with a window between them** -- which is precisely the state the criterion
//! says cannot exist.
//!
//! Nobody had looked, and the reason is worth keeping: the AT row for this
//! criterion carried **no note in any revision**, while ic's own board asserted
//! that the row said the property was merely uncovered for want of a harness.
//! **A claim about what a document says, made by the person who wrote the
//! document.** The only control for that is reading it back.
//!
//! # What the failure cost, stated plainly rather than dramatised
//!
//! A failure in the window left a `todo.flush` in the log and an unmoved
//! cutoff. AC-14.2 had deliberately removed the log fallback, so nothing
//! recovered it: **the flush appeared not to have happened while history
//! recorded that it did.** A record/reality divergence rather than data loss,
//! and the window was two SQLite writes wide.
//!
//! # The injection, and why a simpler one proves nothing
//!
//! **THE OBVIOUS TEST -- flush, then assert the event and the watermark are
//! both there -- PASSES UNDER BOTH IMPLEMENTATIONS.** The old code wrote both
//! too; it just wrote them separately. A test that cannot tell two candidate
//! implementations apart is not evidence about either, and this file exists
//! because that green was available and worthless.
//!
//! So the store is broken UNDERNEATH the facade: `DROP TABLE project` makes the
//! watermark write fail, and **that failure lands exactly in the old window**.
//!
//! | implementation                     | event after the failed flush |
//! | ----------------------------------- | ----------------------------- |
//! | two transactions (before)          | **PRESENT** -- already committed |
//! | one transaction (now)              | **ABSENT** -- rolled back with it |
//!
//! That is the whole discriminator, and it is why the assertion below is about
//! the EVENT rather than about the watermark. Both implementations fail to
//! write a watermark into a table that does not exist; only one of them also
//! keeps the history of a flush that did not happen.
//!
//! # Mutations, measured -- each revert re-run to a green baseline
//!
//! | mutation                                              | reds                                  |
//! | ------------------------------------------------------ | -------------------------------------- |
//! | the OLD shape: commit, then write outside the tx      | `a_failed_flush_leaves_no_event` ONLY |
//! | the in-transaction write dropped entirely             | both                                  |
//!
//! **AND THE FIRST ROW READ `NOTHING RED` BEFORE THE INJECTION WAS FIXED**,
//! which is the whole reason this file says so much about its fixture. The
//! first injection dropped the `project` table outright; `todo_flush` READS the
//! watermark before it writes anything, so the run died at the read and never
//! reached the window. The test passed under the old two-transaction shape and
//! under the new one, and would have shipped as evidence for a property it
//! could not see. **The mutation table caught it; review would not have.**

mod common;

use common::{Fixture, sample_thread};

/// Every op in the log, read straight from the file rather than through the
/// facade -- the facade is the thing under test.
fn ops(fx: &Fixture) -> Vec<String> {
  let db = rusqlite::Connection::open(fx.root().join("intent/.cache/intent.db"))
    .expect("open the store directly");
  let mut stmt = db
    .prepare("SELECT op FROM event_log ORDER BY id")
    .expect("prepare");
  let rows = stmt
    .query_map([], |r| r.get::<_, String>(0))
    .expect("query");
  rows.map(|r| r.expect("row")).collect()
}

/// **THE HAPPY PATH, WHICH ALONE PROVES NOTHING** -- see the module note. It is
/// here because the failing arm below must be shown to be about a flush that
/// would otherwise have worked, not about a fixture that never flushes.
#[test]
fn a_flush_sets_the_watermark_and_records_its_event() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade_on_disk();
  let done = facade.todo_flush().expect("flush");

  assert!(
    done.watermark.is_some(),
    "the flush must set a cutoff, or the failing arm below is about nothing"
  );
  assert!(
    ops(&fx).iter().any(|o| o == "todo.flush"),
    "and it must record the history: {:?}",
    ops(&fx)
  );
}

/// **THE CRITERION: A FLUSH THAT CANNOT WRITE ITS STATE LEAVES NO HISTORY
/// EITHER.**
///
/// `DROP TABLE project` puts the failure exactly where the old two-transaction
/// shape had already committed its event. Under that shape the row survives the
/// failure and the log claims a flush that never took effect; under one
/// transaction it rolls back with everything else.
#[test]
fn a_failed_flush_leaves_no_event() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));

  // **THE INJECTION HAS TO FAIL THE WRITE AND LEAVE THE READ WORKING, AND THE
  // FIRST VERSION OF IT DID NEITHER.** It dropped the table outright -- and
  // `todo_flush` READS the watermark first, through `todo_buckets`, so the run
  // died before it reached the two-write window at all. The test passed under
  // BOTH implementations and proved nothing: the exact vacuous green this file
  // is written against, caught by the mutation table rather than by review.
  //
  // So the table is REPLACED with one that permits a NULL watermark (the read
  // still answers "never flushed") and refuses any value (the write fails).
  // The failure now lands precisely where the old shape had already committed
  // its event.
  //
  // **AND IT IS DONE UNDER A LIVE FACADE**, because `Facade::open` lays the
  // schema down again -- so breaking the file between two facades is undone by
  // the very call meant to meet it.
  let mut facade = fx.facade_on_disk();
  {
    let db = rusqlite::Connection::open(fx.root().join("intent/.cache/intent.db"))
      .expect("open the store directly");
    db.execute_batch(
      "DROP TABLE project;
       CREATE TABLE project (
         id INTEGER PRIMARY KEY CHECK (id = 1),
         todo_watermark TEXT CHECK (todo_watermark IS NULL),
         updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
       );
       INSERT INTO project (id, todo_watermark) VALUES (1, NULL);",
    )
    .expect("replace the table with one that reads and will not be written");
  }

  let before = ops(&fx);
  let result = facade.todo_flush();

  assert!(
    result.is_err(),
    "a flush that cannot record its state must not report success: {result:?}"
  );
  let after = ops(&fx);
  assert!(
    !after.iter().any(|o| o == "todo.flush"),
    "**THE EVENT MUST HAVE ROLLED BACK WITH THE STATE.** It is present, so the two \
     records landed through two doors and the log now claims a flush that no cutoff \
     reflects -- the exact divergence AC-14.7 forbids.\nbefore: {before:?}\nafter: {after:?}"
  );
}
