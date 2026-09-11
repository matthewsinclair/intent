//! **0260: `sync --to-disk` must not write a stale store over committed canon.**
//!
//! The daily driver answers from the store and never looks at the files (hv,
//! 2026-08-14), so a canon change that arrives by git -- a pull, a peer's
//! commit, a merge -- is on disk and in no store until something ingests it.
//! With no intentd running, `sync --to-disk` then rendered the stale store over
//! that canon file at rc=0 and printed _prose ... unchanged_. `doctor` already
//! reported the disagreement; the egest never asked.
//!
//! Driven through two calls with a file edit between them, because that is the
//! state: the store was warm and correct, and then the canon moved under it.

use crate::common::{Fixture, sample_thread};
use intentsvcs::sync::Scope;

const CORRECTION: &str = "a correction committed by a peer and never ingested here";

#[test]
fn a_committed_correction_survives_a_refused_egest() {
  let fx = Fixture::new();
  let mut thread = sample_thread("ST0056");
  fx.write_thread(&thread);
  let mut facade = fx.facade();

  // The peer's commit lands on disk. The open facade's store never sees it.
  thread.objective = CORRECTION.to_string();
  fx.write_thread(&thread);
  assert!(
    fx.read_canon("ST0056").contains(CORRECTION),
    "precondition: the correction is in canon on disk and in no store"
  );

  let err = facade.sync_to_disk(&Scope::All).expect_err(
    "the store disagrees with the committed canon it is about to overwrite, so the egest \
     must refuse rather than report success over the loss",
  );

  assert!(
    fx.read_canon("ST0056").contains(CORRECTION),
    "the refused egest still wrote the stale store over the committed correction"
  );
  let said = err.to_string();
  assert!(
    said.contains("ST0056"),
    "the refusal must name what disagrees, or the operator cannot tell which file to look at: {said}"
  );
}
