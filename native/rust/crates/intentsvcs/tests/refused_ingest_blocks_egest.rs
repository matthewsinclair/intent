//! AT-03.14 / AC-03.13: **a write path whose input was REFUSED must not then
//! be used as a source of truth.**
//!
//! The live instance, with vc as the cause (2026-08-18): `sync --to-store`
//! refused a canon extract on `UNIQUE constraint failed: tests.thread_id,
//! tests.id` and left the store at its previous contents -- correctly.
//! `sync --to-disk` then wrote that stale store over canon at rc=0, with
//! nothing in its output indicating the store had rejected the most recent
//! write. **The same authored criterion was destroyed twice.**
//!
//! # Why the pair is the subject and neither verb is
//!
//! Both verbs are individually correct. The ingest refused and rolled back;
//! the egest wrote exactly what it was given. The defect is that **nothing
//! carries the failure of one into the other**, and a defect that lives in the
//! gap between two correct components is invisible to any test that drives one
//! of them. So every case here drives BOTH, in order, and the assertion is
//! about what survives.
//!
//! # The red-first arm, and it is exactly reproducible
//!
//! Duplicate `id` in one thread's `tests` array. It is valid JSON, it passes
//! the schema, and it dies at the composite primary key inside
//! `Store::rebuild` -- which is a transaction, so the store rolls back whole.
//! That is what makes the store STALE rather than half-written, and stale is
//! the state the egest cannot see.
//!
//! **Driven with the output read, never suppressed.** The original loss was
//! silent only because the write verb was piped to `/dev/null`; a test that
//! reproduces it under suppression is measuring the suppression. The refusal
//! is captured and asserted on here, and the egest's refusal is asserted to
//! NAME what it is refusing over.
//!
//! # Seven mutation arms, six red, and the survivor is named rather than left
//!
//! Two of the cases below exist because a mutant survived without them:
//! `succeeded()` written as `!= "refused"` (which calls a CRASHED load healthy)
//! and the guard reading `None` as a refusal (which would block the egest on
//! every project the moment it upgraded). Both are now driven.
//!
//! **The seventh SURVIVES, it was predicted to survive before the arm was run,
//! and it is recorded here rather than quietly left.** `Store::begin_ingest` /
//! `finish_ingest` are re-entrant so that the OUTERMOST load owns the record --
//! `Facade::sync_from_disk` wraps a region in which `ingest::resync` opens its
//! own. Removing the depth check changes exactly one outcome: an inner load
//! that SUCCEEDS followed by an outer refusal, which is the attachment-carry
//! and event-log-restore steps that run after `resync` returns.
//!
//! **Nothing here provokes that, and the reason it is deferred rather than
//! solved is dated.** The refusals that would provoke it live in
//! `Project::collect_attachments`, which cc is rewriting this session for
//! ST0057 WP-03 -- the non-UTF-8 arm is being turned from a refusal into a
//! carry. A test built on a refusal that is being deleted is the
//! provoker-stopped-provoking trap this estate has already been caught by three
//! times in `error_remedies.rs`, and doing it knowingly would be worse than the
//! three accidents. **So the guard is correct, cheap, load-bearing on a hole
//! that is real, and UNFALSIFIED by this suite** -- which is a different claim
//! from covered, and is made in those words on purpose.

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::model::{AcKind, AcState, Criterion, Thread};
use intentsvcs::sync::Scope;

/// The authored text destroyed in the live instance, in the shape it was in:
/// a criterion somebody typed into canon and had not yet synced.
const AUTHORED: &str =
  "a write path whose input was refused must not then be used as a source of truth";

/// Canon carrying an authored edit AND the defect that refuses it.
///
/// **The two travel together on purpose** -- that is the situation, not a
/// contrivance. The duplicate arrives from a hand edit or a merge, and the
/// authored work arrives in the same sitting; if they were separable the
/// operator would simply not have lost anything.
fn canon_with_an_authored_edit_and_a_duplicate(fx: &Fixture) -> Thread {
  let mut thread = sample_thread("ST0056");
  // The defect: a second test carrying an id the thread already has. Valid
  // JSON, accepted by the generated schema face, and dead on `tests`'s
  // composite primary key inside `Store::rebuild` -- which is a transaction,
  // so the store rolls back WHOLE and is left stale rather than half-written.
  // That distinction is the one this row is about.
  //
  // **Kept local rather than hoisted into `common/`.** It was in the shared
  // fixture module for one commit; cc is mid-flight in that file with the
  // dehydration-gate helpers, and a `--only` commit from here would have
  // carried their work under this message.
  let dupe = thread.tests[0].clone();
  thread.tests.push(dupe);
  thread.criteria.push(Criterion {
    id: "AC-03.13".to_string(),
    text: AUTHORED.to_string(),
    kind: AcKind::Test,
    state: AcState::Computed,
  });
  fx.write_thread(&thread);
  thread
}

/// **THE PROPERTY.** An egest from a store whose last ingest was refused does
/// not quietly succeed.
///
/// Stated as "does not quietly succeed" rather than "refuses" because
/// AC-03.13 allows either refusal or a named report -- but it allows neither
/// to be silent, and silence is what the assertion is against. What ships is
/// the refusal; the assertion below would hold for a loud egest too.
#[test]
fn an_egest_from_a_store_whose_last_ingest_was_refused_is_not_silent() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();

  canon_with_an_authored_edit_and_a_duplicate(&fx);
  assert!(
    fx.read_canon("ST0056").contains(AUTHORED),
    "precondition: the authored criterion is on disk and in no store"
  );

  let refusal = facade
    .sync_from_disk(&Scope::All)
    .expect_err("the duplicate test id must refuse the ingest")
    .to_string();
  assert!(
    !refusal.is_empty(),
    "the ingest refusal is the loud half and it was loud in the live instance too"
  );

  let err = facade.sync_to_disk(&Scope::All).expect_err(
    "an egest from a store whose last ingest was REFUSED must not report success --\n       \
     the store is older than the canon it is about to overwrite, and rc=0 here is\n       \
     the data-loss path presenting as a routine sync",
  );

  let said = err.to_string();
  assert!(
    said.contains("ingest") || said.contains("refused"),
    "the refusal must name WHY it refused, or the operator's next move is to\n       \
     retry the same command: {said}"
  );
}

/// **THE HARM, asserted directly.** Without this the case above passes against
/// an egest that refuses and destroys the file anyway.
#[test]
fn the_authored_edit_survives_the_egest_that_used_to_destroy_it() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();

  canon_with_an_authored_edit_and_a_duplicate(&fx);
  facade
    .sync_from_disk(&Scope::All)
    .expect_err("precondition: the ingest refuses");

  let _ = facade.sync_to_disk(&Scope::All);

  assert!(
    fx.read_canon("ST0056").contains(AUTHORED),
    "THE LOSS. The authored criterion was on disk, the store never accepted it,\n       \
     and the egest wrote the store over it. This is the destruction AC-03.13\n       \
     exists about, and it happened twice to the same text."
  );
}

/// **THE CONTROL, and without it every case above passes on `sync --to-disk`
/// returning an error unconditionally** -- which would break the one direction
/// the estate reaches for to repair stale files.
#[test]
fn an_egest_after_a_successful_ingest_runs() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();

  facade
    .sync_from_disk(&Scope::All)
    .expect("a clean estate ingests");
  assert_eq!(
    facade
      .sync_to_disk(&Scope::All)
      .expect("and the egest that follows it is ordinary"),
    1
  );
}

/// **The block CLEARS.** A store that refused once must not be bricked: fixing
/// the canon and re-running the ingest restores the egest.
///
/// This is the half that makes the refusal's remedy true. A refusal telling an
/// operator to fix the canon and re-run is a lie if re-running leaves the
/// block in place, and the lie would be discovered only by someone already in
/// the failure.
#[test]
fn repairing_the_canon_and_re_ingesting_clears_the_block() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();

  let mut thread = canon_with_an_authored_edit_and_a_duplicate(&fx);
  facade
    .sync_from_disk(&Scope::All)
    .expect_err("precondition: refused");
  facade
    .sync_to_disk(&Scope::All)
    .expect_err("precondition: blocked");

  // The operator does what the remedy says: drop the duplicate, keep the work.
  thread.tests.pop();
  fx.write_thread(&thread);
  facade
    .sync_from_disk(&Scope::All)
    .expect("the repaired canon ingests");

  facade
    .sync_to_disk(&Scope::All)
    .expect("and the egest is available again");
  assert!(
    fx.read_canon("ST0056").contains(AUTHORED),
    "and the authored criterion is now in the store AND on disk -- it was never lost"
  );
}

/// The store answers the question directly, because a property that can only
/// be observed by provoking a second verb is one nothing else can consult --
/// `doctor` needs this, and so does anything reporting estate health.
#[test]
fn the_store_records_the_outcome_of_its_last_ingest() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();

  facade
    .sync_from_disk(&Scope::All)
    .expect("a clean estate ingests");
  let good = facade
    .store()
    .last_ingest()
    .expect("the store answers")
    .expect("an ingest just ran, so there is a record of one");
  assert!(good.succeeded(), "the clean ingest recorded success");

  canon_with_an_authored_edit_and_a_duplicate(&fx);
  facade
    .sync_from_disk(&Scope::All)
    .expect_err("the duplicate refuses");
  let bad = facade
    .store()
    .last_ingest()
    .expect("the store answers")
    .expect("the refusal is an ingest and is recorded as one");
  assert!(
    !bad.succeeded(),
    "a REFUSED ingest is recorded as refused -- recording only the successes\n       \
     leaves `last_ingest` reading `succeeded` forever after the first good one,\n       \
     which is a stale answer wearing a fresh name"
  );
  assert!(
    bad
      .detail
      .as_deref()
      .is_some_and(|d| d.contains("UNIQUE constraint failed")),
    "and it carries WHAT was refused, verbatim, so the egest's refusal can name\n       \
     the actual cause rather than that something went wrong: {:?}",
    bad.detail
  );
}

/// **THE RECORD OUTLIVES THE PROCESS, and that is the whole reason it is a row
/// rather than a return value.**
///
/// The two verbs in AC-03.13 are separate invocations. A value handed back by
/// the ingest reaches nothing -- the process that would read it has not started
/// yet, and by the time it does, the only thing still standing is the database.
/// So the block is asserted across a facade boundary: refuse in one, open a
/// fresh facade on the SAME on-disk store, and find it still blocked.
///
/// Driven `on_disk` deliberately: an in-memory store cannot express this,
/// because it never existed anywhere to be re-opened.
#[test]
fn the_block_is_still_there_for_the_next_process() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));

  {
    let mut first = fx.facade_on_disk();
    canon_with_an_authored_edit_and_a_duplicate(&fx);
    first
      .sync_from_disk(&Scope::All)
      .expect_err("the ingest refuses");
  }

  // A new process, the same store, and canon it has never accepted.
  let mut second = fx.facade_on_disk();
  second.sync_to_disk(&Scope::All).expect_err(
    "the refusal must survive the process that suffered it -- an in-memory\n       \
     carrier would have cleared here, and the operator's next command is\n       \
     exactly where the loss happened",
  );
  assert!(
    fx.read_canon("ST0056").contains(AUTHORED),
    "and the authored criterion is still on disk"
  );
}

/// **A store with nothing on record is not treated as refused, DRIVEN THROUGH
/// THE GUARD.**
///
/// The tolerance is stated rather than implied: absence means nothing was
/// recorded, not that something failed. Reading it as failure would block the
/// egest on every project the moment it upgraded, because the `ingests` table
/// arrives EMPTY on an existing store -- for something nobody observed.
///
/// **Written this way because the obvious version of it does not test the
/// guard.** Asserting `last_ingest().is_none()` on a bare store proves a fact
/// about the store and says nothing about what the egest does with it, and a
/// mutant reading `None` as a refusal survives it untouched. So the store is
/// warmed BY HAND -- the shape of one carried across a schema upgrade, full of
/// data with no history of how it got there -- and then a real facade is opened
/// over it, which finds it warm, ingests nothing, and reaches the guard with
/// nothing on record.
#[test]
fn a_store_with_no_ingest_on_record_does_not_block_the_egest() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));

  {
    let mut bare = intentsvcs::store::Store::open(&fx.project().db_path()).expect("open");
    assert!(
      bare.last_ingest().expect("the store answers").is_none(),
      "precondition: nothing has been recorded into this store"
    );
    let canon = intentsvcs::ingest::read(&fx.project()).expect("valid canon");
    bare
      .rebuild(&canon.threads, &canon.issues)
      .expect("data lands, with no record of how");
    assert!(
      bare.last_ingest().expect("the store answers").is_none(),
      "a store holding data and no record answers NO EVIDENCE, never a refusal"
    );
  }

  let mut facade = fx.facade_on_disk();
  assert!(
    facade
      .store()
      .last_ingest()
      .expect("the store answers")
      .is_none(),
    "precondition: the facade found the store WARM and ingested nothing, so the\n       \
     guard below is genuinely reading an empty history rather than a fresh success"
  );
  facade
    .sync_to_disk(&Scope::All)
    .expect("an egest must run against a store with no load on record");
}

/// **A LOAD THAT NEVER FINISHED BLOCKS AS FIRMLY AS ONE THAT WAS REFUSED.**
///
/// The killed-process case: `begin_ingest` committed its row and nothing ever
/// closed it, so the store holds `attempted`. That is a store which may be
/// half-way through anything, and it is exactly the state a `!= "refused"` test
/// would call healthy.
///
/// Added because a mutant survived without it. Every other case here produces a
/// row that is explicitly `refused`, so `succeeded()` implemented as "not
/// refused" passed the whole file while calling a crashed load fine -- and a
/// crashed load leaves the same stale store, reached by a route nobody chose.
#[test]
fn a_load_that_never_finished_blocks_the_egest_too() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));

  {
    let mut bare = intentsvcs::store::Store::open(&fx.project().db_path()).expect("open");
    let canon = intentsvcs::ingest::read(&fx.project()).expect("valid canon");
    bare
      .rebuild(&canon.threads, &canon.issues)
      .expect("warm it");
    // The process dies here: the row is open and nothing will ever close it.
    bare.begin_ingest().expect("the attempt is recorded");
  }

  let record = intentsvcs::store::Store::open(&fx.project().db_path())
    .expect("reopen")
    .last_ingest()
    .expect("the store answers")
    .expect("the attempt survived the process that opened it");
  assert_eq!(
    record.outcome, "attempted",
    "an unfinished load reads `attempted` -- no caller can spell that value, so\n       \
     it can only ever mean nothing reported an outcome"
  );
  assert!(
    !record.succeeded(),
    "and `succeeded` is a POSITIVE test, so an unfinished load is not success"
  );

  let mut facade = fx.facade_on_disk();
  facade.sync_to_disk(&Scope::All).expect_err(
    "a store whose last load never finished is in an unknown relation to canon,\n       \
     and projecting it is the same wager as projecting a refused one",
  );
}
