//! **0520: A FACADE THAT OUTLIVES A PEER'S COMMIT CATCHES UP TO IT.**
//!
//! A facade answers from the canon it loaded when it opened, and that copy
//! moves only for its own writes. The explorer holds one facade for the whole
//! session, so a write from another terminal, intentd's ingest of a hand edit
//! and the explorer's own `/` commands, which run through a second facade,
//! never reached it: re-entering a view repainted the model as it stood at
//! launch, and an edit to a record another writer had touched was refused
//! every time until the explorer restarted.
//!
//! Two handles on one on-disk store, as two processes have: `facade_on_disk`
//! and never `facade`, whose in-memory stores share nothing.

use crate::common::Fixture;
use intentsvcs::facade::Facade;

fn thread_ids(facade: &Facade) -> Vec<String> {
  facade.st_list().iter().map(|t| t.id.clone()).collect()
}

#[test]
fn a_peers_commit_reaches_a_facade_that_catches_up() {
  let fx = Fixture::new();
  let mut explorer = fx.facade_on_disk();
  let mut peer = fx.facade_on_disk();
  peer
    .st_new("Written by a peer")
    .expect("the peer lands a thread");

  assert!(
    thread_ids(&explorer).is_empty(),
    "the facade saw the peer's write before catching up, so this arm cannot tell a catch-up from none"
  );
  assert!(
    explorer.catch_up().expect("the catch-up reads the store"),
    "a peer committed, and the facade did not say it reloaded"
  );
  assert_eq!(thread_ids(&explorer), vec!["ST0001".to_string()]);
}

#[test]
fn a_facade_that_nothing_moved_under_does_not_reload() {
  let fx = Fixture::new();
  let mut explorer = fx.facade_on_disk();

  assert!(
    !explorer.catch_up().expect("the catch-up reads the store"),
    "nothing committed, and the facade reloaded anyway"
  );
}

/// **THE FACADE'S OWN WRITE IS ALREADY IN ITS CANON**, which is what keeps a
/// catch-up after the explorer's own save to one pragma read.
#[test]
fn a_facades_own_write_is_not_news_to_it() {
  let fx = Fixture::new();
  let mut explorer = fx.facade_on_disk();
  explorer
    .st_new("Written by the explorer")
    .expect("the explorer lands a thread");

  assert!(
    !explorer.catch_up().expect("the catch-up reads the store"),
    "the facade's own write moved the version it compares against"
  );
  assert_eq!(thread_ids(&explorer), vec!["ST0001".to_string()]);
}
