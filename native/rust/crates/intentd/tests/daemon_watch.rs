//! `AT-08.5` / `AC-08.5`: **debounced, gitignore-aware watching drives ingest
//! on external edits.**
//!
//! **THE CRITERION IS ONE SENTENCE WITH THREE CLAIMS IN IT, AND TWO OF THEM ARE
//! ABOUT HOW MANY TIMES SOMETHING HAPPENED.** *Drives ingest* is visible in the
//! store's answers. *Debounced* and *gitignore-aware* are not: a burst of ten
//! writes and a single write leave the store in the same state, and so do a
//! watcher that correctly ignored `intent/.cache/` and one that ingested on
//! every write to it forever. **Both of the adjectives can be entirely absent
//! while every answer the daemon gives is correct.**
//!
//! So the daemon counts its own ingests -- `RegisteredProject::ingested`, beside
//! `dispatched` and deliberately not merged with it -- and this file reads that
//! count over the wire.
//!
//! **THE SELF-TRIGGERING LOOP IS THE SHARPEST CASE AND IT IS WHY THE COUNTER
//! EXISTS.** The store lives at `intent/.cache/intent.db`, INSIDE the tree the
//! daemon watches, and every ingest writes it. A watcher without scope would
//! trigger on the write its own ingest just made and would do so forever, on an
//! idle machine, in a process nobody is looking at -- **while answering every
//! question correctly the whole time.** Nothing in the store's contents can
//! distinguish that from a healthy daemon.

mod common;

use std::path::Path;

use common::{ATTEMPTS, PAUSE, RunningDaemon};
use intentsvcs::wire::{Op, Request, Response};

/// The count of ingests the daemon has run for this project.
///
/// **`Op::Registry` IS IN `wire::UNCOUNTED`, SO ASKING DOES NOT DISTURB THE
/// ANSWER.** That matters more here than for `dispatched`: this file polls the
/// value in a loop, and an instrument that perturbed its own subject would show
/// a count climbing forever and read as exactly the loop it is looking for.
fn ingested(daemon: &RunningDaemon, root: &Path) -> u64 {
  let response = daemon.ask(Request {
    root: root.to_path_buf(),
    op: Op::Registry,
  });
  let Response::Registry { projects } = response else {
    panic!("intentd answered Op::Registry with something else: {response:?}");
  };
  let wanted = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
  projects
    .iter()
    .find(|p| p.root.canonicalize().unwrap_or_else(|_| p.root.clone()) == wanted)
    .map(|p| p.ingested)
    .unwrap_or(0)
}

/// The threads the daemon currently holds for this project.
fn thread_ids(daemon: &RunningDaemon, root: &Path) -> Vec<String> {
  let response = daemon.ask(Request {
    root: root.to_path_buf(),
    op: Op::ThreadList,
  });
  let Response::Threads { threads } = response else {
    panic!("intentd answered Op::ThreadList with something else: {response:?}");
  };
  threads.into_iter().map(|t| t.id).collect()
}

/// Write a canon thread file straight onto disk, as an external editor would.
///
/// **DELIBERATELY NOT THROUGH THE FACADE.** A test that minted the thread
/// through `intentsvcs` would be driving the store and watching the store
/// change -- true, and nothing to do with watching. The criterion says
/// *external edits*, and the only faithful fixture for that is bytes appearing
/// in the tree with the daemon uninvolved.
fn write_thread(root: &Path, id: &str) {
  let path = root.join("intent/.canon/st").join(format!("{id}.json"));
  std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
  std::fs::write(
    &path,
    format!(
      "{{\n  \"schema\": \"intent/thread@3.0\",\n  \"id\": \"{id}\",\n  \"title\": \"Written by hand\",\n  \"status\": \"wip\",\n  \"created\": \"2026-08-30\",\n  \"objective\": \"\",\n  \"context\": \"\"\n}}\n"
    ),
  )
  .expect("write the thread file");
}

/// Block until `condition` holds, or give up saying so.
///
/// A retry count rather than a deadline: what this requires is that it
/// TERMINATES, which says nothing about the time and keeps this workspace's
/// clock guard (D42) true here rather than exempted.
fn until(what: &str, mut condition: impl FnMut() -> bool) {
  for _ in 0..ATTEMPTS {
    if condition() {
      return;
    }
    std::thread::sleep(PAUSE);
  }
  panic!("{what} did not happen in {ATTEMPTS} attempts");
}

/// Let any pending debounce fire and settle, then report the resting count.
///
/// **`quiet` IS DEFINED AS *THE COUNT STOPPED MOVING*, NEVER AS *ENOUGH TIME
/// PASSED*.** A duration would be a claim about the machine, and on a loaded
/// one it is the claim that fails first -- which would report the watcher as
/// broken because a peer was compiling.
/// How many consecutive unchanged polls mean the daemon has stopped ingesting.
///
/// **IT MUST EXCEED THE DEBOUNCE WINDOW, OR `quiet` MEANS `QUIETER THAN THE
/// MECHANISM'S OWN LATENCY`, AND THE FIRST VERSION OF THIS FILE GOT THAT
/// WRONG.** The watcher waits 250ms of stillness before ingesting; ten polls is
/// 200ms, so [`settle`] could declare the daemon at rest while a batch was
/// still inside its own debounce and had not fired yet. The arm below then read
/// that pending ingest as a self-trigger.
///
/// **AND IT FAILED ONLY SOMETIMES, WHICH IS THE WORSE HALF.** The same code
/// passed on the next run, so the reading available at that moment was *it was
/// transient* -- and the race would have shipped. **A test that is right about a
/// defect half the time reports the defect as flakiness in itself.**
///
/// **THE VALUE IS A RELATIONSHIP, NOT A NUMBER SOMEBODY RAISED UNTIL IT
/// PASSED**: enough polls to cover the debounce window twice over. Tuning until
/// green would look identical on the page, which is why the relationship is
/// written down rather than the arithmetic.
const STABLE_FOR: u32 = 25;

fn settle(daemon: &RunningDaemon, root: &Path) -> u64 {
  let mut last = ingested(daemon, root);
  let mut still = 0;
  for _ in 0..ATTEMPTS {
    std::thread::sleep(PAUSE);
    let now = ingested(daemon, root);
    if now == last {
      still += 1;
      if still >= STABLE_FOR {
        return now;
      }
    } else {
      still = 0;
      last = now;
    }
  }
  panic!(
    "the ingest count never stopped moving, which is the self-triggering loop this file exists to catch: last seen {last}"
  );
}

#[test]
fn an_external_edit_reaches_the_store_with_nobody_running_sync() {
  let daemon = RunningDaemon::start();
  let root = common::project("Watched");

  // First contact registers the project AND starts its watch -- there is no
  // separate `watch` verb, deliberately, so this is the whole setup.
  let before = thread_ids(&daemon, &root);
  assert_eq!(
    before.len(),
    1,
    "the fixture project should hold exactly its one minted thread: {before:?}"
  );

  write_thread(&root, "ST0042");

  until("the daemon ingested the hand-written thread", || {
    thread_ids(&daemon, &root).contains(&"ST0042".to_string())
  });

  // **THE CONTROL THAT SAYS IT WAS THE WATCHER.** Nothing in this test ran a
  // sync, opened a facade, or asked the daemon to do anything but list -- and
  // a listing does not ingest. Without this assertion the arm above would also
  // pass if `Op::ThreadList` had quietly grown a re-read.
  assert!(
    ingested(&daemon, &root) >= 1,
    "the thread appeared and the daemon reports no ingests, so it arrived by some route other than watching -- most likely a read path that re-reads the tree, which would make this criterion untestable"
  );

  let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn one_external_edit_costs_a_bounded_number_of_ingests() {
  // **THIS ROW MEASURED SOMETHING REAL ON ITS FIRST RUN AND THE FINDING IS
  // WORTH MORE THAN THE ASSERTION.** A hand-written edit costs TWO ingests and
  // the second is the daemon reacting to ITSELF: `sync_from_disk` is not
  // read-only -- it rewrites the canon and the views of everything it read,
  // which are in scope by construction -- so normalising an external edit
  // generates a filesystem event of the daemon's own making.
  //
  // **IT CONVERGES, AND THE REASON LIVES SOMEWHERE ELSE ENTIRELY.**
  // `WriteSet::commit` skips a path whose bytes already match, so the second
  // pass writes nothing and there is no third event. That guard was built for a
  // different problem -- `intent st hold` rewriting every view of every
  // unrelated thread, because `write_atomically` is temp-file-plus-rename and a
  // rename swaps in a NEW INODE, so a byte-identical re-emission moves mtime by
  // construction. **The watcher's termination is a downstream consequence of an
  // unrelated optimisation, pinned by `write_moves_only_what_changed` and by
  // nothing here.** Delete that skip to fix some future mtime problem and this
  // daemon ingests forever on an idle machine, answering correctly throughout.
  //
  // So the claim is stated as the bound it actually is, and `settle` carries
  // the unbounded case.
  let daemon = RunningDaemon::start();
  let root = common::project("Quiet");

  let _ = thread_ids(&daemon, &root);
  write_thread(&root, "ST0043");
  until("the daemon ingested", || ingested(&daemon, &root) >= 1);

  // `settle` panics if the count never stops climbing, which IS the loop.
  let resting = settle(&daemon, &root);
  assert!(
    (1..=3).contains(&resting),
    "one hand-written thread cost {resting} ingests. TWO is what this build does -- the edit, then the daemon noticing its own normalisation of it -- and more than that means the projection is not converging on a fixed point"
  );

  // And it stays put with nothing touching the tree.
  for _ in 0..STABLE_FOR {
    std::thread::sleep(PAUSE);
  }
  assert_eq!(
    ingested(&daemon, &root),
    resting,
    "the ingest count moved with nothing editing the project. The daemon is watching its own writes: an ingest touches `intent/.cache/intent.db` inside the watched tree, and only scope stops that becoming a loop"
  );

  let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_change_to_a_path_outside_the_sync_scope_drives_no_ingest() {
  // The gitignore-aware half, driven at both of its mechanisms: a
  // `SKIPPED_DIRS` member and a real gitignore rule in a real repository.
  let daemon = RunningDaemon::start();
  let root = common::project("Scoped");

  let _ = thread_ids(&daemon, &root);
  write_thread(&root, "ST0044");
  until("the daemon ingested once", || ingested(&daemon, &root) >= 1);
  let resting = settle(&daemon, &root);

  // Real git, because the rule IS git's ignore semantics and a reimplementation
  // would disagree exactly where it matters.
  for args in [
    &["init", "-q"][..],
    &["config", "user.email", "t@example.com"][..],
    &["config", "user.name", "t"][..],
  ] {
    let ok = std::process::Command::new("git")
      .args(args)
      .current_dir(&root)
      .status()
      .expect("run git")
      .success();
    assert!(ok, "git {args:?} failed");
  }
  std::fs::write(root.join(".gitignore"), "*.log\n").expect("gitignore");
  // The `.gitignore` itself is at the root and is NOT one of `sync::ROOT_FILES`,
  // so writing it is already an out-of-scope change; settle again so the arm
  // below measures only what it names.
  let resting = settle(&daemon, &root).max(resting);

  std::fs::create_dir_all(root.join("intent/.cache")).expect("mkdir");
  std::fs::write(root.join("intent/.cache/scratch"), b"not canon").expect("write");
  std::fs::write(root.join("intent/noise.log"), b"ignored").expect("write");

  // Give the debounce every chance to fire before concluding it did not: an
  // absence claim is only as good as the window it was observed over.
  for _ in 0..STABLE_FOR {
    std::thread::sleep(PAUSE);
  }
  assert_eq!(
    ingested(&daemon, &root),
    resting,
    "a write inside `intent/.cache/` or to a gitignored file drove an ingest. Neither is in the sync's scope, and the first one is the daemon's own store: acting on it is the self-triggering loop"
  );

  // **THE POSITIVE CONTROL, IN THE SAME WINDOW AND AGAINST THE SAME DAEMON.**
  // Without it, every assertion above holds for a watcher that died, a
  // debouncer that never fires, or a project that stopped being watched when
  // `git init` appeared -- all of which produce a count that does not move.
  write_thread(&root, "ST0045");
  until("an in-scope edit still drives an ingest", || {
    ingested(&daemon, &root) > resting
  });

  let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_burst_of_edits_is_debounced_into_far_fewer_ingests() {
  // **THE `debounced` CLAIM, AND IT NEEDS A COUNT BECAUSE THE STORE CANNOT
  // SHOW IT.** Ten writes and one write leave identical contents. What differs
  // is how many whole-corpus transactions ran to get there, which on a real
  // project is the difference between a save being free and a `git checkout`
  // pinning a core.
  const WRITES: u64 = 12;

  let daemon = RunningDaemon::start();
  let root = common::project("Burst");

  let _ = thread_ids(&daemon, &root);
  let before = settle(&daemon, &root);

  for n in 0..WRITES {
    write_thread(&root, &format!("ST01{n:02}"));
  }

  until("the burst was ingested", || {
    ingested(&daemon, &root) > before
  });
  let after = settle(&daemon, &root);
  let ingests = after - before;

  assert!(
    ingests >= 1,
    "a burst of {WRITES} in-scope writes drove no ingest at all"
  );
  assert!(
    ingests < WRITES,
    "{WRITES} writes drove {ingests} ingests, which is one per write -- the debouncer is not debouncing, and each of those is a whole-corpus transaction"
  );

  // The state is still correct, which is the half debouncing must not cost.
  let ids = thread_ids(&daemon, &root);
  for n in 0..WRITES {
    let id = format!("ST01{n:02}");
    assert!(
      ids.contains(&id),
      "`{id}` was written during the burst and never reached the store, so the debounce dropped work rather than coalescing it: {ids:?}"
    );
  }

  let _ = std::fs::remove_dir_all(&root);
}
