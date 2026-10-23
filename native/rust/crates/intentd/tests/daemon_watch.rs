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

use std::path::Path;

use crate::common::{ARM_ATTEMPTS, ATTEMPTS, PAUSE, RunningDaemon};
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

/// The id of the sentinel [`arm`] writes; no arm below uses it for anything
/// else.
const SENTINEL: &str = "ST0900";

/// Prove the project's watch is delivering before any bounded wait begins,
/// and return the resting ingest count after it (issue 0481).
///
/// A sentinel thread is written as an external editor would write one, and
/// the arm waits up to [`ARM_ATTEMPTS`] for the daemon to hold it. Only the
/// FIRST event of a new watch waits behind the host's filesystem backlog, so
/// after this every [`ATTEMPTS`] bound below is a claim about the watcher.
fn arm(daemon: &RunningDaemon, root: &Path) -> u64 {
  write_thread(root, SENTINEL);
  for _ in 0..ARM_ATTEMPTS {
    if thread_ids(daemon, root).iter().any(|id| id == SENTINEL) {
      return settle(daemon, root);
    }
    std::thread::sleep(PAUSE);
  }
  panic!(
    "the watch never delivered its first event in {ARM_ATTEMPTS} attempts, so no bounded wait after it could measure anything"
  );
}

#[test]
fn an_external_edit_reaches_the_store_with_nobody_running_sync() {
  let daemon = RunningDaemon::start();
  let root = crate::common::project("Watched");

  // First contact registers the project AND starts its watch -- there is no
  // separate `watch` verb, deliberately, so this is the whole setup.
  let before = thread_ids(&daemon, &root);
  assert_eq!(
    before.len(),
    1,
    "the fixture project should hold exactly its one minted thread: {before:?}"
  );
  arm(&daemon, &root);

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

/// **ST0083 AT-02.2, COVERING AC-02.3: THE RUNNING DAEMON'S HALF.** In a `dotclaude` project an
/// edit to `.claude/AGENTS.md` reaches the store with nobody running a sync, as
/// a root copy's edit does in a `root` project. The layout is declared and
/// `.claude/` made before first contact, because first contact is when the
/// watch registers its directories.
#[test]
fn a_dotclaude_contract_edit_reaches_the_store_with_nobody_running_sync() {
  let daemon = RunningDaemon::start();
  let root = crate::common::project("Dotclaude");
  let mut config = intentsvcs::project::Project::open(&root)
    .expect("the fixture opens")
    .config()
    .clone();
  config.contracts = intentsvcs::project::Contracts::Dotclaude;
  intentsvcs::project::write_config(&root, &config).expect("the config writes");
  std::fs::create_dir_all(root.join(".claude")).expect("mkdir .claude");
  let agents = root.join(".claude/AGENTS.md");
  std::fs::write(&agents, "# AGENTS.md\n").expect("the contract");

  thread_ids(&daemon, &root);
  let rest = arm(&daemon, &root);

  std::fs::write(&agents, "# AGENTS.md\n\nan external edit\n").expect("edit the contract");

  until("the daemon ingested the .claude/AGENTS.md edit", || {
    ingested(&daemon, &root) > rest
  });

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
  let root = crate::common::project("Quiet");

  let _ = thread_ids(&daemon, &root);
  let armed = arm(&daemon, &root);
  write_thread(&root, "ST0043");
  until("the daemon ingested", || ingested(&daemon, &root) > armed);

  // `settle` panics if the count never stops climbing, which IS the loop.
  let resting = settle(&daemon, &root);
  let cost = resting - armed;
  assert!(
    (1..=3).contains(&cost),
    "one hand-written thread cost {cost} ingests. TWO is what this build does -- the edit, then the daemon noticing its own normalisation of it -- and more than that means the projection is not converging on a fixed point"
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
  let root = crate::common::project("Scoped");

  let _ = thread_ids(&daemon, &root);
  arm(&daemon, &root);
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
  let root = crate::common::project("Burst");

  let _ = thread_ids(&daemon, &root);
  let before = arm(&daemon, &root);

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

/// A facade on `root`, opened fresh, the way a command-line verb opens one.
fn facade(root: &Path) -> intentsvcs::facade::Facade {
  let project = intentsvcs::project::Project::open(root).expect("the project opens");
  let ctx = intentsvcs::facade::FacadeContext {
    principal: "test".to_string(),
    project_id: project.config().project_id.clone().unwrap_or_default(),
    version: env!("CARGO_PKG_VERSION").to_string(),
  };
  intentsvcs::facade::Facade::open(project, ctx).expect("the facade opens")
}

/// Run git in `dir`, refusing with what git said.
///
/// Identity, signing and hooks are set on every call, so nothing here reads
/// the user's configuration: a global hooks path would run some other
/// repository's gate on this fixture's commits.
fn git(dir: &Path, args: &[&str]) {
  let out = std::process::Command::new("git")
    .args([
      "-c",
      "user.email=t@example.com",
      "-c",
      "user.name=t",
      "-c",
      "commit.gpgsign=false",
      "-c",
      "core.hooksPath=/dev/null",
    ])
    .args(args)
    .current_dir(dir)
    .output()
    .expect("run git");
  assert!(
    out.status.success(),
    "git {args:?} failed in {}:\n{}{}",
    dir.display(),
    String::from_utf8_lossy(&out.stdout),
    String::from_utf8_lossy(&out.stderr)
  );
}

/// The store's file index, the table the watcher reconciles against, read
/// beside the daemon's writer as
/// `a_source_edit_reaches_the_index_and_costs_canon_nothing.rs` reads it.
fn recorded(root: &Path) -> Vec<intentsvcs::sync::FileEntry> {
  intentsvcs::store::Store::open(&root.join("intent/.cache/intent.db"))
    .expect("the daemon's store opens beside it")
    .file_index()
    .expect("the file index reads")
}

/// What the next backstop pass would ingest: every in-scope file whose bytes
/// the store has not recorded.
///
/// **THE BACKSTOP'S OWN QUESTION, ASKED NOW RATHER THAN WAITED FOR.** The
/// backstop reconciles the root through `sync::changed_under` against the
/// file index every thirty seconds; this asks the same function the same
/// question of the same table. A file it names after an ingest has settled is
/// a file every later pass ingests again, which `settle` cannot see: between
/// two passes the count is still.
fn left_for_a_backstop(root: &Path) -> Vec<std::path::PathBuf> {
  intentsvcs::sync::changed_under(root, root, &recorded(root)).expect("the scope reads")
}

/// **A RECORDED VIEW THAT A PULL REPLACED WITH A PEER'S NEWER RENDER IS
/// INGESTED, ITS ROW THEN HOLDS THE PULLED BYTES, AND NO BACKSTOP INGESTS IT
/// AGAIN** (issue 0630, vc's order of 2026-10-23). A view's row is only ever
/// what a projection wrote or found right. The pull brings the peer's canon
/// with the view rendered from it; the ingest takes the canon in, and its
/// projection finds the view already right and leaves it. Had that
/// projection not recorded what it found, the row would keep the old render
/// and the backstop would ingest the view at every pass.
///
/// "Once" is held as the count coming to rest with nothing left for a
/// backstop, not as an exact count: a pull's files can reach the watcher in
/// more than one debounced batch.
#[test]
fn a_pulled_render_of_a_recorded_view_is_ingested_and_then_left_alone() {
  const RETITLED: &str = "Retitled by a peer";
  let daemon = RunningDaemon::start();
  let ours = crate::common::project("Pulled");
  let id = thread_ids(&daemon, &ours)
    .pop()
    .expect("the fixture's one thread");
  arm(&daemon, &ours);

  let address =
    intentsvcs::address::parse(&format!("intent:///threads/{id}")).expect("an addressable thread");
  facade(&ours)
    .hydrate(&address)
    .expect("the thread's views realise");
  let rel = format!("intent/st/{id}/info.md");
  assert!(
    ours.join(&rel).is_file(),
    "hydrating {id} realised no `{rel}`"
  );

  git(&ours, &["init", "-q"]);
  git(&ours, &["add", "-A"]);
  git(&ours, &["commit", "-q", "-m", "ours"]);
  let theirs = crate::common::short_dir("intentd-peer");
  let ours_path = ours.to_str().expect("a utf-8 path");
  git(&theirs, &["clone", "-q", ours_path, "."]);
  facade(&theirs)
    .set(&address, "title", RETITLED.into())
    .expect("the peer's write");
  git(&theirs, &["add", "-A"]);
  git(&theirs, &["commit", "-q", "-m", "theirs"]);
  let pulled = std::fs::read(theirs.join(&rel)).expect("the peer's render");
  assert_ne!(
    std::fs::read(ours.join(&rel)).expect("our render"),
    pulled,
    "the peer's write left `{rel}` as it was, so the pull replaces no view and this arm asks nothing"
  );

  let rest = settle(&daemon, &ours);
  let theirs_path = theirs.to_str().expect("a utf-8 path");
  git(&ours, &["pull", "-q", "--ff-only", theirs_path, "HEAD"]);
  until("the daemon ingested the pull", || {
    ingested(&daemon, &ours) > rest
  });
  settle(&daemon, &ours);

  let Response::Threads { threads } = daemon.ask(Request {
    root: ours.clone(),
    op: Op::ThreadList,
  }) else {
    panic!("intentd answered Op::ThreadList with something else");
  };
  assert!(
    threads.iter().any(|t| t.id == id && t.title == RETITLED),
    "the ingest did not take the pulled canon in: {threads:?}"
  );
  assert_eq!(
    std::fs::read(ours.join(&rel)).expect("the view after the ingest"),
    pulled,
    "the daemon rendered over the pulled `{rel}`"
  );
  let rows = recorded(&ours);
  let sha = intentsvcs::model::sha256_hex(&pulled);
  assert!(
    rows.iter().any(|e| e.path == rel && e.sha256 == sha),
    "`{rel}`'s row does not hold the pulled bytes: {:?}",
    rows.iter().find(|e| e.path == rel)
  );
  let left = left_for_a_backstop(&ours);
  assert!(
    left.is_empty(),
    "after the pull's ingest the backstop still finds {left:?} changed, so it ingests them again at every pass"
  );

  let _ = std::fs::remove_dir_all(&ours);
  let _ = std::fs::remove_dir_all(&theirs);
}

/// **A FILE AT A VIEW'S PATH THAT NO RENDERER WRITES IS INGESTED AT MOST
/// ONCE, NEVER AT EVERY BACKSTOP** (issue 0630, vc's order of 2026-10-23).
/// The scan no longer records a view's bytes, so a file matching a view's
/// pattern that no projection writes has no row unless the ingest that met it
/// records one, and the backstop would find it changed at every pass. An
/// issue view for an issue the store does not hold is such a file: no
/// renderer writes it and nothing reads it in.
#[test]
fn a_stray_file_at_a_view_path_is_ingested_at_most_once() {
  let daemon = RunningDaemon::start();
  let root = crate::common::project("Stray");
  thread_ids(&daemon, &root);
  let rest = arm(&daemon, &root);

  let rel = "intent/issues/0099.md";
  let project = intentsvcs::project::Project::open(&root).expect("the fixture opens");
  assert!(
    project
      .generated_view_patterns()
      .iter()
      .any(|pattern| intentsvcs::search::glob_matches(pattern, rel)),
    "`{rel}` matches no generated view's pattern, so this arm would not be asking about a view"
  );
  let stray = root.join(rel);
  std::fs::create_dir_all(stray.parent().expect("a parent")).expect("mkdir");
  std::fs::write(
    &stray,
    "# 0099\n\nLeft behind by a branch this store never saw.\n",
  )
  .expect("write the stray");

  until("the daemon ingested the stray", || {
    ingested(&daemon, &root) > rest
  });
  settle(&daemon, &root);

  assert!(
    stray.is_file(),
    "the ingest removed `{rel}`, so nothing is left to ingest again and this arm cannot see the loop it asks about"
  );
  let left = left_for_a_backstop(&root);
  assert!(
    left.is_empty(),
    "after the stray's ingest the backstop still finds {left:?} changed, so it ingests them again at every pass: no row holds their bytes"
  );

  let _ = std::fs::remove_dir_all(&root);
}

/// **THE SAME FILE, EDITED AFTER ITS INGEST, IS INGESTED ONCE MORE AND THEN
/// LEFT ALONE** (vc's ruling, 2026-10-23). Its first sighting records its row,
/// and no renderer ever records over it, so unless the ingest takes the scan's
/// entry again over a row no renderer wrote, the row keeps the first bytes and
/// the backstop finds every later edit changed at every pass.
#[test]
fn a_stray_file_edited_after_its_ingest_is_ingested_once_more_and_then_left_alone() {
  let daemon = RunningDaemon::start();
  let root = crate::common::project("Restray");
  thread_ids(&daemon, &root);
  let rest = arm(&daemon, &root);

  let stray = root.join("intent/issues/0099.md");
  std::fs::create_dir_all(stray.parent().expect("a parent")).expect("mkdir");
  std::fs::write(&stray, "# 0099\n\nFirst seen.\n").expect("write the stray");
  until("the daemon ingested the stray", || {
    ingested(&daemon, &root) > rest
  });
  let first = settle(&daemon, &root);

  std::fs::write(&stray, "# 0099\n\nEdited after its ingest.\n").expect("edit the stray");
  until("the daemon ingested the edit", || {
    ingested(&daemon, &root) > first
  });
  settle(&daemon, &root);

  let left = left_for_a_backstop(&root);
  assert!(
    left.is_empty(),
    "after the edit's ingest the backstop still finds {left:?} changed, so it ingests them again at every pass: the row holds the first sighting's bytes"
  );

  let _ = std::fs::remove_dir_all(&root);
}
