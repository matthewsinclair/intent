//! Issue 0638: a verb that removes a file the store indexed forgets it. The
//! file index is what the store believes is on disk, so after each verb no row
//! remains for a path it printed as `removed:`, while a verb that writes still
//! records what it wrote.
//!
//! **EACH ARM FIRST PROVES IT CAN RED**: some path the verb removed had a row
//! before it ran, so an index that kept the row would fail the arm rather than
//! pass it vacuously.

use std::collections::BTreeSet;

use crate::common::{Fixture, gate_open, sample_issue, sample_thread};
use intentsvcs::address::{Address, Entity};
use intentsvcs::facade::Facade;

fn rows(f: &Facade) -> BTreeSet<String> {
  f.store()
    .file_index()
    .expect("the file index")
    .into_iter()
    .map(|e| e.path)
    .collect()
}

/// The verb's own `removed:` set, the one the printer names.
fn forgotten(before: &BTreeSet<String>, f: &mut Facade) -> Vec<String> {
  let removed = f.take_landed().removed;
  assert!(
    !removed.is_empty(),
    "the verb removed nothing, so this arm proves nothing"
  );
  assert!(
    removed.iter().any(|p| before.contains(p)),
    "no path the verb removed had a row before it ran: {removed:?}"
  );
  let after = rows(f);
  let kept: Vec<&String> = removed.iter().filter(|p| after.contains(*p)).collect();
  assert!(
    kept.is_empty(),
    "the file index still has a row for a path the verb removed: {kept:?}"
  );
  removed
}

#[test]
fn st_dehydrate_forgets_the_views_it_removes() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  fx.write_thread(&gate_open());
  fx.write_file(
    "intent/.intentfiles",
    "STEELTHREAD:ST0001\nSTEELTHREAD:ST0057\n",
  );
  let mut f = fx.facade();
  let at = Address {
    authority: None,
    entity: Entity::Thread {
      id: "ST0001".to_string(),
    },
    format: None,
  };
  f.hydrate(&at).expect("a declared thread realises");
  f.take_landed();
  let before = rows(&f);

  f.dehydrate(&at).expect("dehydrate");

  forgotten(&before, &mut f);
}

#[test]
fn st_renumber_forgets_the_paths_it_moves_away_from() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  fx.write_file("intent/.intentfiles", "STEELTHREAD:ST0001\n");
  let mut f = fx.facade_on_disk();
  f.take_landed();
  let before = rows(&f);

  f.st_renumber("ST0001", "ST0003").expect("renumber");

  let removed = forgotten(&before, &mut f);
  assert!(
    removed.iter().all(|p| !p.contains("ST0003")),
    "a path under the new id was named as removed: {removed:?}"
  );
}

#[test]
fn issues_renumber_forgets_the_paths_it_moves_away_from() {
  let fx = Fixture::new();
  fx.write_issue(&sample_issue(21));
  let mut f = fx.facade_on_disk();
  f.take_landed();
  let before = rows(&f);

  f.issue_renumber(21, 22).expect("renumber");

  forgotten(&before, &mut f);
}

/// **THE CONTROL**: forgetting is not a table that stopped being written. A
/// verb that writes still records what it wrote.
#[test]
fn issues_add_still_records_what_it_writes() {
  let fx = Fixture::new();
  let mut f = fx.facade_on_disk();
  f.take_landed();
  let before = rows(&f);

  f.issue_add("a control issue", Some("low"), None, "a body\n")
    .expect("add");

  let wrote = f.take_landed().wrote;
  let after = rows(&f);
  let recorded: Vec<&String> = wrote
    .iter()
    .filter(|p| after.contains(*p) && !before.contains(*p))
    .collect();
  assert!(
    !recorded.is_empty(),
    "issues add wrote {wrote:?} and the file index recorded none of it"
  );
}
