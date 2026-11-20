//! **A VERB NAMED `add`/`new` FAILS ON AN EXISTING KEY RATHER THAN REPLACING
//! IT** -- issue 0131, ruled by hv 2026-08-28.
//!
//! # The incident this is the deterministic form of
//!
//! Separate nodes ran `intent issues add` on 2026-08-28 and **both were told
//! `created: intent/.canon/issues/0126.json`**. One filing survived; the other
//! reached neither the store nor the extract, and nothing said so. `git show
//! 9e9ee8ab:intent/.canon/issues/0126.json` -- the losing node's own commit --
//! carries the other node's issue.
//!
//! Two code facts made it possible. `next_issue_number()` is `max() + 1` over
//! the canon this facade loaded, a read with no reservation; and the landing
//! was `INSERT ... ON CONFLICT (number) DO UPDATE SET ... body =
//! excluded.body`. **A create implemented as an upsert.** There was no
//! unique-violation path: the collision was converted into a full overwrite and
//! the call returned `Ok`.
//!
//! # The issue reported that the race would not reproduce, and that stands
//!
//! Several drives failed to reproduce it, and the informative one explains why:
//! `next_issue_number()` reads canon from the STORE, not from the extract, so
//! two SEQUENTIAL adds can never collide. The issue says in terms that the
//! window is not characterised, and this file does not claim to characterise it
//! either.
//!
//! **So this drives the PROPERTY rather than the race.** Two facades opened
//! before either writes are the state the incident was in, however it got
//! there. They used to compute the same next number, and the second write was
//! refused. Since ST0095 WP-06 each mint draws its record's own creation event,
//! so the two get two ids and both filings survive, and the store's create
//! door, which still refuses a held key, is driven at the store, where a key
//! can be named. A test that waited for a scheduler to produce that state would
//! be measuring the scheduler.
//!
//! # What the arms are for
//!
//! The refusal is worth nothing on its own: a store that refused every write
//! would pass a refusal test and fail at everything else. So the ordinary
//! create, the ordinary CHANGE, and a whole-estate resync are all driven here.
//! **The change arms are not padding -- they are the ones that would have gone
//! red if the door had been implemented by dropping the `ON CONFLICT` clause
//! outright**, which is the obvious reading of the ruling and is wrong:
//! `issues close` clones canon, edits one field and writes the whole row back.

//! # Every facade here is `facade_on_disk`, and that is not incidental
//!
//! `Fixture::facade()` opens an IN-MEMORY store, so two of them share no
//! database at all -- each writes its own and then renders the extract, and the
//! second simply overwrites the first one's FILE. Written that way first, this
//! file reproduced a loss with the same symptom and a different cause, and the
//! refusal under test could not have prevented it. **A fixture that cannot
//! exhibit the defect cannot clear it, and one that exhibits a DIFFERENT defect
//! with the same symptom is worse, because it looks like coverage.** Two nodes
//! running `intent` share one on-disk store; that is the population.

use crate::common::{Fixture, sample_issue, sample_thread};
use intentsvcs::model::IssueStatus;

/// The ordinary path, and the non-vacuity control for everything below.
///
/// Two sequential adds through ONE facade allocate two numbers and both
/// survive. If this ever goes red, the refusal arms are passing because the
/// verb is broken rather than because the door is shut.
#[test]
fn two_sequential_adds_allocate_two_numbers_and_both_survive() {
  let fx = Fixture::new();
  let mut f = fx.facade_on_disk();

  let first = f
    .issue_add("the first filing", None, None, "body one")
    .unwrap();
  let second = f
    .issue_add("the second filing", None, None, "body two")
    .unwrap();

  assert_ne!(first, second, "two adds must not land on one number");
  assert_eq!(
    f.issue_show(first).unwrap().title,
    "the first filing",
    "the first filing must survive the second"
  );
  assert_eq!(f.issue_show(second).unwrap().title, "the second filing");
}

/// **THE INCIDENT'S STATE, AND NOW NOTHING COLLIDES** (ST0095 WP-06). Two
/// facades, both loaded before either wrote, are the state 0131 was in, and
/// both used to compute the same next number. A mint now draws each record's
/// own creation event, so the two filings get two ids and both survive, read
/// back byte intact through a third facade. That is 0131's ruling, a filing is
/// never lost, held by construction.
#[test]
fn two_facades_loaded_before_either_writes_both_mint_and_both_filings_survive() {
  let fx = Fixture::new();
  let mut a = fx.facade_on_disk();
  let mut b = fx.facade_on_disk();

  let first = a
    .issue_add("the first filing", None, None, "the first body")
    .expect("the first add");
  let second = b
    .issue_add("the second filing", None, None, "the second body")
    .expect("the second add, from a facade that never saw the first");
  assert_ne!(first, second, "two facades minted one id");

  let after = fx.facade_on_disk();
  assert_eq!(after.issue_show(first).unwrap().body, "the first body");
  assert_eq!(after.issue_show(second).unwrap().body, "the second body");
}

/// The same for threads, because `st new` mints the same way and one ruling
/// covers the mutation surface rather than one verb.
#[test]
fn two_facades_loaded_before_either_writes_both_mint_a_thread() {
  let fx = Fixture::new();
  let mut a = fx.facade_on_disk();
  let mut b = fx.facade_on_disk();

  let first = a.st_new("the first thread").unwrap();
  let second = b.st_new("the second thread").unwrap();
  assert_ne!(first, second, "two facades minted one id");

  let after = fx.facade_on_disk();
  assert_eq!(after.st_show(&first).unwrap().title, "the first thread");
  assert_eq!(after.st_show(&second).unwrap().title, "the second thread");
}

/// **THE DOOR STAYS SHUT WHERE NO VERB CAN REACH IT.** A create is written
/// through the store's create door, so the DATABASE decides inside the
/// transaction whether the key was free. No mint can bring it a held key at
/// sixty-four bits, so it is driven here at the store, the one place a key
/// can be named, and the row it holds must survive the refused write.
#[test]
fn the_store_refuses_a_create_of_a_key_it_holds_and_keeps_the_row() {
  let mut store = intentsvcs::store::Store::open_in_memory().expect("store");
  let held = sample_issue(1);
  create_issue(&mut store, &held).expect("the first create");

  let mut other = sample_issue(1);
  other.title = "the create that must not overwrite it".to_string();
  let refused = create_issue(&mut store, &other).expect_err("the key is held");
  assert!(
    matches!(
      &refused,
      intentsvcs::store::StoreError::CreateHitAnExistingKey { key, .. } if *key == held.number.stored()
    ),
    "the refusal must name the key it hit, as its own variant: {refused:?}"
  );
  let (_, issues) = store.load_canon().expect("load");
  assert_eq!(
    issues.len(),
    1,
    "the refused write landed under another key"
  );
  assert_eq!(
    issues[0].title, held.title,
    "the refused write replaced the row"
  );
}

/// One issue brought into existence through the store's create door.
fn create_issue(
  store: &mut intentsvcs::store::Store,
  issue: &intentsvcs::model::Issue,
) -> Result<intentsvcs::store::StoredDates, intentsvcs::store::StoreError> {
  let envelope = intentsvcs::event::Envelope::minted(
    "test",
    "",
    "issues.add",
    intentsvcs::event::Subject {
      kind: "issue".to_string(),
      id: issue.number.stored(),
    },
    serde_json::json!({}),
  );
  store.commit_mutation(intentsvcs::store::Mutation {
    threads: &[],
    issues: &[issue],
    removed_threads: &[],
    removed_issues: &[],
    created_threads: &[],
    created_issues: &[issue.number],
    expected_threads: &[],
    expected_issues: &[],
    sections: &[],
    envelopes: &[&envelope],
    project_state: intentsvcs::store::ProjectStateEdit::Unchanged,
  })
}

/// **THE ARM THAT WOULD CATCH THE OBVIOUS WRONG FIX.** Dropping `ON CONFLICT`
/// outright shuts the create door and every other door with it: `issues close`
/// clones canon, edits `status`, and writes the whole row back through the same
/// statement.
#[test]
fn an_ordinary_change_still_writes_the_whole_row_back() {
  let fx = Fixture::new();
  let mut f = fx.facade_on_disk();

  let n = f
    .issue_add("an issue to close", None, None, "with a body")
    .unwrap();
  f.issue_close(n)
    .expect("closing an issue is a CHANGE, not a create, and must not refuse");

  let after = fx.facade_on_disk();
  let issue = after.issue_show(n).unwrap();
  assert_eq!(issue.status, IssueStatus::Closed);
  assert_eq!(
    issue.body, "with a body",
    "a change must carry the fields it does not set"
  );
  assert!(
    issue.closed.as_ref().is_some_and(|d| !d.is_empty()),
    "the close date is filled by the database inside the write"
  );

  f = fx.facade_on_disk();
  f.issue_open(n).expect("reopening is a change too");
}

/// **AND A WHOLE-ESTATE RESYNC STILL RELOADS ROWS IT ALREADY HOLDS.** `rebuild`
/// is the disk -> db direction and REPLACES the estate, so every row it writes
/// exists already by construction. Sending those through the create door would
/// refuse every resync there is -- the loudest possible regression, and the
/// reason `rebuild` names its door explicitly rather than inheriting one.
#[test]
fn a_resync_over_an_estate_the_store_already_holds_does_not_refuse() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  fx.write_issue(&sample_issue(1));

  // Twice. The first warms the store from the extract; the second is the one
  // that would refuse if a reload were read as a create.
  for pass in 1..=2 {
    let f = fx.facade_on_disk();
    assert_eq!(
      f.issue_list().len(),
      1,
      "pass {pass}: the resync must reload the estate it already holds"
    );
    assert!(f.st_show("ST0001").is_ok(), "pass {pass}");
  }
}
