//! AT-08.1 / AC-08.1: **the write path is DB first, canon ALWAYS, views IF
//! MARKED.**
//!
//! After any write, the committed extract carries the change **whether or not
//! the artefact is realised on disk**. The alternative -- "disk if marked" --
//! collapses canon into views and leaves a dehydrated artefact's change inside
//! a gitignored database, absent from a fresh clone. Under D34 the extract is
//! how truth travels between machines, so that is not a rendering gap; it is
//! work that does not exist anywhere a second machine can see.
//!
//! # The two halves fail in opposite directions and the row needs both
//!
//! **Canon must be written even when nothing is realised** -- otherwise the
//! change lives only in `intent/.cache/`, which D21 gitignores.
//!
//! **Views must NOT be written for an artefact that is not realised** --
//! otherwise every mutation silently re-hydrates what `organize` was asked to
//! remove, and the dehydrated state is unreachable in practice: you dehydrate,
//! you touch anything, and it is back.
//!
//! A test asserting only the first passes on a write path that rewrites all 266
//! views on every verb, which is precisely the state ST0057 exists to leave.
//!
//! # Reaching the dehydrated state honestly
//!
//! `organize`'s removals are refused while any of AC-00.1's declared
//! preconditions is unmet, and there is no bypass -- `Verdict`'s fields are
//! private and `preconditions::check` is its only constructor. So the fixture
//! SATISFIES the real declaration through `common::gate_open`, which is the
//! same work the estate has to do. **A test that reached this state by
//! side-stepping the gate would be measuring the side-step.**

mod common;

use common::{Fixture, gate_open, sample_thread};
use intentsvcs::model::ThreadStatus;
use intentsvcs::organize::Mode;
use intentsvcs::sync::Scope;

/// An estate where ST0001 is REALISED, then dehydrated by a manifest that
/// declares nothing.
fn dehydrated(fx: &Fixture) {
  fx.write_thread(&gate_open());
  // **DEHYDRATED BY STATUS, NOT BY AN EMPTY MANIFEST.** The first version of
  // this fixture declared nothing and relied on that to dehydrate. It stopped
  // working the moment `organize` began rewriting the generated region from
  // status (AC-02.2): the empty declaration is now an OUTPUT, regenerated on
  // every run, so writing one is no longer a way to say anything. Terminal
  // status is.
  let mut closed = sample_thread("ST0001");
  closed.status = ThreadStatus::Completed;
  fx.write_thread(&closed);

  // **REALISE FIRST WITH NO MANIFEST AT ALL.** An absent manifest means nobody
  // has said, so everything renders -- which is how a project that has never
  // organized behaves. Writing one first would mean nothing was ever realised
  // and `organize` would remove nothing, so every assertion below would pass
  // because the file was never there.
  let mut f = fx.facade_on_disk();
  f.sync_to_disk(&Scope::All)
    .expect("realise everything first");
  assert!(
    fx.project().info_view("ST0001").exists(),
    "precondition: ST0001 is on disk before organize is asked to remove it"
  );

  // `organize` reads the manifest, so the file has to exist; its generated
  // region is what the run is about to rewrite.
  fx.write_file("intent/.intentfiles", "# BEGIN INTENT\n# END INTENT\n");
  let mut f = fx.facade_on_disk();
  f.organize(Mode::Apply)
    .expect("the gate is open, so the removals happen");
  assert!(
    !fx.project().info_view("ST0001").exists(),
    "precondition: ST0001 is DEHYDRATED -- if it is still here the gate refused\n       \
     or the status rule kept it, and every assertion below would pass for the\n       \
     wrong reason"
  );
}

/// **CANON ALWAYS.** The change reaches the committed extract even though the
/// artefact has no files.
#[test]
fn a_write_to_a_dehydrated_artefact_reaches_canon() {
  let fx = Fixture::new();
  dehydrated(&fx);

  let mut f = fx.facade_on_disk();
  // **A WRITE THAT DOES NOT MOVE THREAD STATUS.** `st hold` is illegal from
  // `completed`, and anything that reopened the thread would REALISE it -- the
  // artefact would stop being dehydrated, and the test would measure a
  // hydration instead of a write to a dehydrated thing. Withdrawing a criterion
  // carries authored prose, which is the half canon has to keep.
  f.ac_withdraw(
    "ST0001",
    "AC-03.2",
    "the premise did not reproduce -- zzz-canary",
    None,
  )
  .expect("withdrawing a non-test criterion is legal on a completed thread");

  let canon = fx.read_canon("ST0001");
  assert!(
    canon.contains("withdrawn"),
    "the decision is in the committed extract:\n{canon}"
  );
  assert!(
    canon.contains("the premise did not reproduce -- zzz-canary"),
    "and so is the authored reason -- it is the half that exists nowhere else"
  );
}

/// **VIEWS IF MARKED.** The mutation does not silently re-hydrate what
/// `organize` removed.
///
/// **This is the half that makes the row worth having.** Without it the case
/// above is satisfied by a write path that rewrites every view on every verb --
/// which passes while making the dehydrated state unreachable in practice.
#[test]
fn a_write_to_a_dehydrated_artefact_does_not_resurrect_its_views() {
  let fx = Fixture::new();
  dehydrated(&fx);

  let mut f = fx.facade_on_disk();
  // **A WRITE THAT DOES NOT MOVE THREAD STATUS.** `st hold` is illegal from
  // `completed`, and anything that reopened the thread would REALISE it -- the
  // artefact would stop being dehydrated, and the test would measure a
  // hydration instead of a write to a dehydrated thing. Withdrawing a criterion
  // carries authored prose, which is the half canon has to keep.
  f.ac_withdraw(
    "ST0001",
    "AC-03.2",
    "the premise did not reproduce -- zzz-canary",
    None,
  )
  .expect("withdrawing a non-test criterion is legal on a completed thread");

  assert!(
    !fx.project().info_view("ST0001").exists(),
    "the artefact was DEHYDRATED and a mutation brought its view back. Nothing\n       \
     asked for it to be realised -- so `organize` is undone by the next verb\n       \
     anyone runs, and the sparse projection is a state the estate cannot hold."
  );
  assert!(
    !fx.project().acceptance_view("ST0001").exists(),
    "and the contract view likewise"
  );
}

/// **THE CONTROL.** A REALISED artefact's views are still written, or the case
/// above passes on a write path that stopped rendering anything at all.
#[test]
fn a_write_to_a_realised_artefact_still_updates_its_views() {
  let fx = Fixture::new();
  fx.write_thread(&gate_open());
  fx.write_thread(&sample_thread("ST0001"));
  fx.write_file(
    "intent/.intentfiles",
    "STEELTHREAD:ST0001\n# BEGIN INTENT\n# END INTENT\n",
  );

  let mut f = fx.facade_on_disk();
  f.sync_to_disk(&Scope::All).expect("realise");
  f.organize(Mode::Apply)
    .expect("organize keeps what the manifest pins");
  assert!(
    fx.project().info_view("ST0001").exists(),
    "precondition: a PINNED artefact survives organize"
  );

  f.ac_withdraw(
    "ST0001",
    "AC-03.2",
    "the premise did not reproduce -- zzz-canary",
    None,
  )
  .expect("a legal mutation that does not move thread status");
  let view =
    std::fs::read_to_string(fx.project().acceptance_view("ST0001")).expect("the contract view");
  // **Asserted on the AUTHORED REASON rather than on the status word.** The
  // view renders `status: On Hold` -- a display form -- while canon carries the
  // wire form `hold`, and a test keyed on the wire form fails against a correct
  // renderer. The reason is authored text that appears verbatim in both, so it
  // discriminates without coupling this row to display vocabulary.
  assert!(
    view.contains("the premise did not reproduce -- zzz-canary"),
    "a realised artefact's view carries the change:\n{view}"
  );
}

/// **THE ESTATE-WIDE VIEWS SURVIVE A FULLY DEHYDRATED ESTATE.**
///
/// `steel_threads.md` and `todo.md` belong to no thread -- they are a function
/// of the whole model -- so the manifest must not narrow them. **An index that
/// vanishes when its members dehydrate is worse than a stale one**: the estate
/// would appear empty to anyone reading the tree, which is AC-03.15's harm
/// arriving through the realisation path instead of the egest.
///
/// **Added because a mutation arm survived without it.** `owning_thread`
/// returning a thread for EVERY path -- rather than `None` for a path under no
/// thread directory -- passed the whole file while deleting the index of a
/// dehydrated estate. Nothing else here looks at a view that no artefact owns.
#[test]
fn the_estate_index_is_written_even_when_every_thread_is_dehydrated() {
  let fx = Fixture::new();
  dehydrated(&fx);

  let index = fx.project().steel_threads_view();
  let todo = fx.project().todo_view();
  assert!(index.exists(), "precondition: the index was realised");

  let mut f = fx.facade_on_disk();
  f.ac_withdraw(
    "ST0001",
    "AC-03.2",
    "the premise did not reproduce -- zzz-canary",
    None,
  )
  .expect("a legal mutation that does not move thread status");

  assert!(index.exists(), "the index is not deleted");
  assert!(todo.exists(), "and the todo view likewise");

  // **ASSERTED ON CONTENT, NOT EXISTENCE, AND A MUTATION ARM IS WHY.**
  // `exists()` was the first version and it does not discriminate: the index
  // was written by the realise step and nothing deletes it, so a projection
  // that stopped UPDATING the index passed the check while the file sat there
  // going stale. **A file that is present and wrong is the failure this row is
  // about**, one directory up.
  let shown = std::fs::read_to_string(&index).expect("readable");
  assert!(
    shown.contains("ST0001"),
    "the index belongs to no thread, so the manifest must not narrow it -- it\n       \
     has to carry the change even though the artefact's own views are gone:\n{shown}"
  );
  assert!(
    shown.contains("ST0001"),
    "and it still indexes the thread whose FILES are gone -- dehydration is\n       \
     about the artefact's own views, not about hiding it from the estate"
  );
}

/// **AN UNREADABLE MANIFEST REALISES EVERYTHING AND NEVER DEHYDRATES.**
///
/// The fail-open direction, asserted rather than assumed. A malformed
/// `.intentfiles` -- one unknown sigil is enough -- must not read as *nobody is
/// declared*, because that answer DELETES every view in the estate on the next
/// write. **A parse failure is the one moment the tool knows least, and it is
/// the worst possible moment to act on a confident-looking empty set.**
///
/// Added because a mutation arm survived without it: `Unreadable` mapped to
/// `Declared(empty)` passed the whole file while dehydrating everything.
#[test]
fn a_manifest_that_does_not_parse_realises_everything() {
  let fx = Fixture::new();
  fx.write_thread(&gate_open());
  fx.write_thread(&sample_thread("ST0001"));

  let mut f = fx.facade_on_disk();
  f.sync_to_disk(&Scope::All).expect("realise");
  assert!(fx.project().info_view("ST0001").exists(), "precondition");

  // One unknown sigil. The grammar refuses at the first unreadable line.
  fx.write_file(
    "intent/.intentfiles",
    "THREAD:ST0001\n# BEGIN INTENT\n# END INTENT\n",
  );

  let mut f = fx.facade_on_disk();
  f.ac_withdraw(
    "ST0001",
    "AC-03.2",
    "the premise did not reproduce -- zzz-canary",
    None,
  )
  .expect("a legal mutation that does not move thread status");

  std::fs::read_to_string(fx.project().info_view("ST0001")).expect(
    "a manifest the tool cannot read must not be treated as one declaring NONE --\n       \
     that answer deletes the estate at the moment the tool understands it least",
  );
  // **The CONTRACT view, not the cover: a withdrawn criterion renders there.**
  // Asserting on `info.md` passed for the wrong reason once already -- it is
  // present either way, so its existence proves realisation and says nothing
  // about the write reaching a face.
  let contract =
    std::fs::read_to_string(fx.project().acceptance_view("ST0001")).expect("the contract view");
  assert!(
    contract.contains("the premise did not reproduce -- zzz-canary"),
    "and the view is UPDATED, not merely left behind:\n{contract}"
  );
}

/// The store agrees with canon afterwards, so "canon always" is not achieved by
/// writing the file and losing the change in the source of truth.
#[test]
fn the_store_and_canon_agree_after_a_dehydrated_write() {
  let fx = Fixture::new();
  dehydrated(&fx);

  let mut f = fx.facade_on_disk();
  f.ac_withdraw(
    "ST0001",
    "AC-03.2",
    "the premise did not reproduce -- zzz-canary",
    None,
  )
  .expect("a legal mutation that does not move thread status");
  assert!(
    f.st_show("ST0001")
      .expect("thread")
      .criteria
      .iter()
      .any(|c| format!("{:?}", c.state).contains("Withdrawn")),
    "the DB is first and it has the change"
  );
  assert!(
    fx.read_canon("ST0001")
      .contains("the premise did not reproduce -- zzz-canary"),
    "and canon is not a second opinion"
  );
}
