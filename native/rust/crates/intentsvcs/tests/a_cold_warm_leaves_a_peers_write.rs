//! **0131: warming a cold store must not delete a record a peer wrote first.**
//!
//! Two `issues add` on a fresh project each found the store empty and each
//! warmed it from disk -- and the later warm's rebuild deleted every row,
//! including the peer's just-committed `0001`, whose file had not landed yet.
//! The number was free again, the create landed on it, and both writers were
//! told `created`. Measured: 3 of 3 cold runs of ten paired rounds lost `0001`;
//! 0 of 3 on a store warmed first.
//!
//! The race is inside one call, so this pins the invariant that closes it
//! rather than the interleaving: two handles on one on-disk store, as two
//! processes have, and a warm from a disk that does not carry the peer's write.

use crate::common::{Fixture, sample_issue};
use intentsvcs::store::Store;

#[test]
fn a_cold_warm_from_a_stale_disk_leaves_a_record_a_peer_wrote_first() {
  let fx = Fixture::new();
  let mut peer = Store::open(&fx.project().db_path()).expect("the peer's handle");
  let mut warmer = Store::open(&fx.project().db_path()).expect("the warmer's handle");

  peer
    .rebuild(&[], &[sample_issue(1)])
    .expect("the peer lands its first record");

  let warmed = warmer
    .warm_if_cold(&[], &[])
    .expect("the warm runs against the stale disk canon, which holds nothing");

  let (_, issues) = warmer.load_canon().expect("read the store");
  assert_eq!(
    issues.iter().map(|i| i.number).collect::<Vec<_>>(),
    vec![1],
    "the warm deleted the record a peer wrote after the store was found empty"
  );
  assert!(
    !warmed,
    "a store holding a record is not cold, and the warm must say it did nothing"
  );
}
