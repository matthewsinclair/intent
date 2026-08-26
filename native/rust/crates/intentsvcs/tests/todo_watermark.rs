//! **The DONE watermark is durable state, and v2 kept it in a disposable
//! file.** ic's WP-03 constraint, tested.
//!
//! `bin/intent_todo` writes the flush instant INTO `todo.md` and reads it back
//! out (`read_done_watermark`, :228), so the generated view is the watermark's
//! only store. Under the v3 truth model that is not a stylistic difference: a
//! generated view is disposable by construction -- `sync --to-disk` rewrites
//! it, `doctor` regenerates it to compare, and deleting it is supposed to be
//! safe -- so durable state living there means **deleting a derived file
//! silently resets the flush and resurrects every item ever flushed.**
//!
//! The v3 watermark is an EVENT. What that buys is tested below rather than
//! asserted in a comment: it survives losing the store, it is not in the view,
//! and `doctor` agrees with the renderer about it -- the last being a hazard
//! this design introduces and has to answer for.

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::event;
use intentsvcs::model::{AcceptanceMode, Thread, ThreadStatus, WpStatus};

/// A thread that finished long enough ago to be flushable.
///
/// Its work packages are finished too, and that is not tidiness: the view puts
/// a WIP work package in DOING regardless of its thread's status, so a
/// "completed" thread still carrying live work appears in DOING under its own
/// id -- and a test asserting on the id alone would then be measuring the work
/// package while claiming to measure the flush.
fn finished(id: &str, completed: &str) -> Thread {
  let mut t = sample_thread(id);
  t.status = ThreadStatus::Completed;
  t.status_reason = None;
  t.completed = Some(completed.to_string());
  for wp in &mut t.wps {
    wp.status = WpStatus::Done;
  }
  t
}

/// A thread whose completion date the DATABASE sets to today.
///
/// It goes all the way round through the real verbs -- triage, start, done --
/// because the point is the stamp, and the stamp only exists if the write that
/// makes it happens. A test supplying "today" would be confecting the one value
/// under test (D42).
///
/// `acceptance: exempt` because `st.done` is gate-guarded and a thread with
/// zero criteria BLOCKS. That is the gate working; exempting the fixture is the
/// declared way past it, and inventing criteria here would make this test
/// depend on the contract rules it has nothing to do with.
fn complete_today(fx: &Fixture, facade: &mut intentsvcs::facade::Facade, title: &str) -> String {
  let id = facade.st_new(title).expect("st_new");
  facade.st_triage(&id).expect("triage -> not-started");
  facade.st_start(&id).expect("start");

  let mut thread = facade.st_show(&id).expect("show").clone();
  thread.acceptance = Some(AcceptanceMode::Exempt);
  fx.write_thread(&thread);
  facade
    .sync_from_disk(&intentsvcs::sync::Scope::All)
    .expect("take the exemption into the store");

  facade.st_done(&id).expect("done");
  id
}

/// **THE property, and the reason the watermark is an event rather than a
/// settings row or a line in the view.**
///
/// A flush must survive the loss of the database, because losing the database
/// is a NORMAL state: `intent/.cache/` is gitignored (D21), so every fresh
/// clone starts without one. Under D34 the event log is the one durable thing
/// nothing else derives, so it travels in the extract and comes back.
///
/// The fixture reaches the no-store state by CLONING the extract rather than by
/// deleting a database, because deleting one is the idiom D36 rules out -- and
/// it is also the weaker fixture, proving something about a directory that had
/// a store when the case under test is a directory that never did.
#[test]
fn a_flush_survives_a_machine_that_has_no_database() {
  let fx = Fixture::new();
  fx.write_thread(&finished("ST0001", "2020-01-01"));
  let mut facade = fx.facade_on_disk();

  assert!(
    facade.todo_view().expect("view").contains("ST0001"),
    "precondition: the finished thread is in DONE before any flush"
  );
  facade.todo_flush().expect("flush");
  assert!(
    !facade.todo_view().expect("view").contains("ST0001"),
    "precondition: the flush cleared it on this machine"
  );

  // History reaches the extract on sync, like every other event.
  facade
    .sync_to_disk(&intentsvcs::sync::Scope::All)
    .expect("sync");

  // A different machine, holding only what git carries.
  let clone = fx.clone_extract();
  let cloned = clone.facade_on_disk();
  assert!(
    !cloned.todo_view().expect("view").contains("ST0001"),
    "the flush did not travel: a clone rebuilt from the extract has resurrected a flushed item, \
     which is exactly the v2 defect this design exists to prevent"
  );
}

/// **The view is not the watermark's store**, stated as the thing v2 got wrong.
///
/// Deleting `todo.md` is meant to be safe. If the watermark lived in it, the
/// regenerated file would come back unflushed -- and it would look completely
/// normal, because an unflushed view is a perfectly valid view.
#[test]
fn deleting_the_generated_view_does_not_undo_a_flush() {
  let fx = Fixture::new();
  fx.write_thread(&finished("ST0001", "2020-01-01"));
  let mut facade = fx.facade_on_disk();
  facade.todo_flush().expect("flush");
  facade.todo_update().expect("write todo.md");

  let flushed = fx.read("intent/todo.md");
  assert!(!flushed.contains("ST0001"), "precondition: flushed on disk");

  std::fs::remove_file(fx.path("intent/todo.md")).expect("rm todo.md");
  facade.todo_update().expect("regenerate");

  assert_eq!(
    fx.read("intent/todo.md"),
    flushed,
    "regenerating a deleted view brought a flushed item back, so the view was the watermark's \
     only store"
  );
}

/// **`doctor` and the renderer agree about the watermark.**
///
/// This is a hazard the design introduces rather than one it inherits, so it is
/// tested rather than reasoned about. `doctor` re-renders every view to detect
/// a hand-edited one, and it runs on projects with no database. If it computed
/// the watermark differently from the writer -- or not at all -- it would
/// re-render `todo.md` unflushed, disagree with the committed file, and report
/// skew on every project that had ever flushed, permanently, with nothing
/// wrong.
///
/// Both states are checked, because they fail for different reasons: with a
/// store the watermark comes from the event table, without one it comes from
/// `events.jsonl`, and the second path is the one a fresh clone takes.
#[test]
fn doctor_does_not_report_a_flushed_view_as_hand_edited() {
  let fx = Fixture::new();
  fx.write_thread(&finished("ST0001", "2020-01-01"));
  let mut facade = fx.facade_on_disk();
  facade.todo_flush().expect("flush");
  facade
    .sync_to_disk(&intentsvcs::sync::Scope::All)
    .expect("sync");

  let skew = |dir: &Fixture, store: Option<&intentsvcs::store::Store>| -> Vec<String> {
    intentsvcs::facade::Facade::doctor(&dir.project(), &common::facade_ctx(), store)
      .findings
      .iter()
      .filter(|f| f.file.contains("todo.md"))
      .map(|f| format!("{}: {}", f.file, f.detail))
      .collect()
  };

  let store = intentsvcs::store::Store::open(&fx.project().db_path()).expect("store");
  assert!(
    skew(&fx, Some(&store)).is_empty(),
    "doctor reports the view it just wrote as skewed: {:?}",
    skew(&fx, Some(&store))
  );

  // The fresh-clone path: no database at all, watermark from the extract.
  let clone = fx.clone_extract();
  assert!(
    !clone.path("intent/.cache/intent.db").exists(),
    "precondition: the clone genuinely has no store"
  );
  assert!(
    skew(&clone, None).is_empty(),
    "with no store, doctor re-rendered todo.md without the watermark and called the committed \
     file hand-edited: {:?}",
    skew(&clone, None)
  );
}

/// **A flush clears the work finished TODAY, which is the whole complaint that
/// reopened this.**
///
/// This test asserted the opposite until 2026-08-26, and it was not wrong about
/// the code -- it was faithful to a design hv overruled. The watermark was
/// truncated to a date, so a completion recorded today compared EQUAL to it and
/// stayed in DONE; `--prune` on a day's work cleared nothing and the command
/// explained why. **A todo file whose DONE bucket cannot be emptied on the day
/// you did the work is the file that grew to thousands of lines.**
///
/// The instant plus midnight-widening ([`views`]' `completed_instant`) is what
/// makes it work, and this is where that is proved rather than described:
/// `ST0002` finishes today, through the real verbs, and the flush takes it.
///
/// **The old completion is here as the control.** Without it a build that
/// cleared everything unconditionally -- or one whose DONE bucket was empty for
/// some unrelated reason -- would pass the assertion that matters.
#[test]
fn a_flush_clears_work_completed_today() {
  let fx = Fixture::new();
  fx.write_thread(&finished("ST0001", "2020-01-01"));
  let mut facade = fx.facade_on_disk();

  // A thread completed TODAY, by the database's own reckoning -- no clock is
  // read here either. `st_done` stamps `completed` at the write.
  let today = complete_today(&fx, &mut facade, "finished today");

  let flushed = facade.todo_flush().expect("flush");
  assert!(
    flushed.cleared.iter().any(|l| l.contains("ST0001")),
    "the old completion is cleared -- the control, so that the assertion below cannot pass \
     because DONE was empty to begin with: {:?}",
    flushed.cleared
  );
  assert!(
    flushed.cleared.iter().any(|l| l.contains(&today)),
    "TODAY's completion is cleared: a date-granular watermark left it sitting in DONE until \
     tomorrow, and that is the behaviour hv reversed on 2026-08-26: {flushed:?}"
  );
  assert!(
    flushed.remaining.is_empty(),
    "nothing survives a flush but a completion dated at or after it, and this fixture has \
     none: {flushed:?}"
  );
  assert!(
    flushed.watermark.is_some(),
    "a flush that clears nothing still sets a watermark"
  );
}

/// The watermark is an ISO 8601 INSTANT at second resolution, taken from an
/// event the database stamped -- `2026-08-26T21:40:25Z`, hv's ruling and v2's
/// `date -u '+%Y-%m-%dT%H:%M:%SZ'`.
///
/// Asserted on shape and on RELATION to the event, never on a value: a test
/// that knew the value would have had to read a clock, which is the thing the
/// database stamp exists to avoid (D42).
#[test]
fn the_watermark_is_an_instant_derived_from_a_database_stamped_event() {
  let fx = Fixture::new();
  fx.write_thread(&finished("ST0001", "2020-01-01"));
  let mut facade = fx.facade_on_disk();
  facade.todo_flush().expect("flush");

  let events = facade.store().events().expect("events");
  let flushes: Vec<_> = events
    .iter()
    .filter(|e| e.op == event::TODO_FLUSH)
    .collect();
  assert_eq!(flushes.len(), 1, "one flush, one event");
  let ts = &flushes[0].ts;
  assert!(
    ts.len() == 24 && ts.ends_with('Z') && ts.contains('T'),
    "the event carries a full instant the database set: {ts:?}"
  );

  let mark = event::todo_watermark(&events).expect("a watermark");
  assert_eq!(
    mark.len(),
    20,
    "the watermark is a full instant at second resolution -- `2026-08-26T21:40:25Z` is 20 \
     characters, and the DATE this used to return was 10: {mark:?}"
  );
  assert!(
    !mark.contains('.'),
    "the millisecond fraction the database stamps is dropped, because a cutoff is read and \
     retyped by people and the date it is compared against cannot use it: {mark:?}"
  );
  assert_eq!(
    mark,
    format!("{}Z", &ts[..19]),
    "and it is that event's own instant, seconds kept and fraction dropped -- not a value \
     from anywhere else"
  );

  // Two machines' logs are a UNION under D34, so arrival order is not time
  // order and the watermark must be the MAXIMUM rather than the last appended.
  let mut shuffled = events.clone();
  shuffled.reverse();
  assert_eq!(
    event::todo_watermark(&shuffled),
    Some(mark),
    "the watermark depends on the stamps, not on the order the log happens to be in"
  );
}

/// The JSON buckets and the markdown view are one bucketing.
///
/// Every label the structured form carries appears in the rendered view, and
/// the counts match. Two traversals applying the same status rules would agree
/// on the day they were written and not afterwards.
#[test]
fn the_structured_buckets_and_the_rendered_view_say_the_same_thing() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  fx.write_thread(&finished("ST0002", "2020-01-01"));
  let facade = fx.facade_on_disk();

  let buckets = facade.todo_buckets().expect("buckets");
  let view = facade.todo_view().expect("view");

  let all: Vec<_> = buckets
    .doing
    .iter()
    .chain(buckets.todo.iter())
    .chain(buckets.done.iter())
    .collect();
  assert!(
    !all.is_empty(),
    "the fixture must put something in a bucket, or this compares two empty sets"
  );
  for item in &all {
    assert!(
      view.contains(&item.label),
      "{:?} is in the structured form and not in the view",
      item.label
    );
    assert!(
      item.label.contains(&item.title),
      "a label carries its own title: {item:?}"
    );
  }
  // **Top-level rows only, and by GLYPH-AGNOSTIC prefix.** This counted
  // `- [ ] ` -- an unchecked box -- and was written before rows carried a
  // status glyph, so it now matches nothing at all and reports 0 against a
  // populated view. It also has to exclude nested work packages, which are
  // indented and are not members of `all`.
  assert_eq!(
    view.lines().filter(|l| l.starts_with("- [")).count(),
    all.len(),
    "the view renders exactly the items the buckets carry, and no others"
  );
}
