//! Issue 0500: **`intent sync --to-disk` runs beside a daemon WATCHING this
//! project's tree, and the daemon does not ingest what it landed.**
//!
//! `--to-disk` is a projection from the store: it writes only the paths whose
//! bytes differ and records each one as landed, which is what every mutating
//! verb's projection does beside a watching daemon. Until 0500 it opened the
//! store `Exclusive` and was refused beside a watcher, on a reason -- two
//! engines both ingesting one tree -- that is true of `--to-store` and not of
//! it. `routing_is_opt_in.rs` keeps the refusal, on `--to-store`.
//!
//! **THE DAEMON HALF IS AN ABSENCE, SO IT IS MEASURED AGAINST A CONTROL RATHER
//! THAN A WINDOW.** "The daemon did not ingest the landing" is a claim that
//! nothing happened, and load makes nothing-happened easier to see (issue
//! 0481). So the arm does not wait a while and count. It writes a sentinel,
//! waits for the daemon to ingest it -- a presence, which load only delays --
//! and measures what one sentinel costs. It then does the same again with the
//! landing in front of the second sentinel. If the landing cost an ingest, the
//! second measurement is larger than the first. The sentinel's presence is what
//! says the watcher has caught up with everything written before it.
//!
//! **AND SINCE ISSUE 0696 NOTHING IN IT WAITS FOR A QUIET SPELL.** Each
//! measurement ends when the daemon's store holds its sentinel, and each
//! sentinel arrives whole, so the count read then is at rest by construction
//! rather than after enough unchanged polls (see `caught_up`).
//!
//! **WHAT KEEPS THE COST AT ZERO IS SCOPE, AND THE ARM SAYS SO RATHER THAN
//! CREDITING THE LANDING RECORD.** Measured while building it: removing a
//! generated view from a watched tree costs no ingest, and the arm stays green
//! with `sync_to_disk`'s `record_landed` call deleted. So it guards against the
//! watcher's scope taking in the views `--to-disk` lands -- the loop issue 0311
//! was about -- and it does not pin `record_landed`, which other tests own. Its
//! instrument is proven to see an ingest by the sentinel, which costs one.

use std::path::Path;

use crate::common::{ATTEMPTS, PAUSE, RealDaemon, short_dir};
use intentsvcs::wire::{self, Op, Request, Response};

/// Tries for the watcher's FIRST event on a fresh project, which can wait on
/// the watch being armed; the same allowance `intentd`'s own tests give it.
const ARM_ATTEMPTS: u32 = ATTEMPTS * 12;

/// A project carrying one thread and one issue, and the issue's id.
fn project() -> (std::path::PathBuf, String) {
  let root = short_dir("todisk-proj");
  intentsvcs::init::init(&root, "ToDisk", "test", env!("CARGO_PKG_VERSION"))
    .expect("the shipped initialiser creates a project");
  let opened = intentsvcs::project::Project::open(&root).expect("the project just created");
  let ctx = intentsvcs::facade::FacadeContext::for_tests(
    "test".to_string(),
    opened.config().project_id.clone().unwrap_or_default(),
    env!("CARGO_PKG_VERSION").to_string(),
  );
  let mut facade = intentsvcs::facade::Facade::open(opened, ctx).expect("open the new project");
  facade
    .st_new("Landed beside a watcher")
    .expect("mint one thread");
  let issue = facade
    .issue_add(
      "Landed beside a watcher",
      Some("low"),
      None,
      "The view this arm lands.",
    )
    .expect("file one issue");
  (root, issue.stored())
}

fn run(home: &Path, root: &Path, argv: &[&str]) -> std::process::Output {
  crate::common::intent()
    .args(argv)
    .current_dir(root)
    .env("HOME", home)
    .stdin(testkit::lifeline_for(argv))
    .output()
    .expect("the intent binary runs")
}

fn text(out: &std::process::Output) -> String {
  format!(
    "{}{}",
    String::from_utf8_lossy(&out.stdout),
    String::from_utf8_lossy(&out.stderr)
  )
}

/// Wait, in tries and never on a clock, until `done` holds.
fn until(what: &str, mut done: impl FnMut() -> bool) {
  for _ in 0..ARM_ATTEMPTS {
    if done() {
      return;
    }
    std::thread::sleep(PAUSE);
  }
  panic!("{what}: not seen within {ARM_ATTEMPTS} tries");
}

/// Does the daemon's store hold thread `id`? Asked over the wire, as a client
/// asks, so the answer is the daemon's own and not a read of the tree.
fn holds(daemon: &RealDaemon, root: &Path, id: &str) -> bool {
  let endpoint = daemon
    .endpoint()
    .expect("the daemon was answering when this test started");
  let response = wire::ask(
    &endpoint,
    &Request {
      root: root.to_path_buf(),
      op: Op::ThreadList,
    },
  )
  .expect("the shipped client completes a round trip to a live daemon");
  let Response::Threads { threads } = response else {
    panic!("intentd answered Op::ThreadList with something else: {response:?}");
  };
  threads.iter().any(|t| t.id == id)
}

/// Put a hand-written thread file into the watched canon WHOLE.
///
/// The sentinel is a hand-written thread file, the edit `intentd`'s own
/// `daemon_watch.rs` drives, because a thread is in the watcher's scope by
/// construction. A thread ingest re-renders the thread views, which is why the
/// view this arm lands is an issue's.
///
/// **STAGED UNDER `intent/.cache/` AND RENAMED IN, BECAUSE A PLAIN WRITE IS TWO
/// CHANGES THE WATCHER CAN SEE APART** (issue 0696). `std::fs::write` creates
/// the file empty and then fills it, and under load the watcher can reconcile
/// the empty file before the bytes land. One sentinel then costs two ingests,
/// the first failing and both counted, and the arm's two measurements disagree
/// with nothing wrong. A rename within one filesystem shows the file whole or
/// not at all. `intent/.cache/` holds the store and is outside the watcher's
/// scope by construction, so staging there costs nothing.
fn write_sentinel(root: &Path, id: &str) {
  let staged = root.join("intent/.cache").join(format!("{id}.json"));
  std::fs::write(
    &staged,
    format!(
      "{{\n  \"schema\": \"intent/thread@3.0\",\n  \"id\": \"{id}\",\n  \"title\": \"Written by hand\",\n  \"status\": \"wip\",\n  \"created\": \"2026-09-21\",\n  \"objective\": \"\",\n  \"context\": \"\"\n}}\n"
    ),
  )
  .expect("stage the sentinel thread");
  std::fs::rename(
    &staged,
    root.join("intent/.canon/st").join(format!("{id}.json")),
  )
  .expect("move the sentinel thread into the canon whole");
}

/// Write a sentinel, and read the ingest count once the daemon's store holds it.
///
/// **THE BARRIER IS THE DAEMON'S OWN ANSWER, NOT A QUIET SPELL** (issue 0696).
/// The store holds the sentinel once the watcher has reconciled it, and with it
/// every in-scope write made before it, which the daemon reconciles in the same
/// pass or an earlier one. The sentinel arrived whole, and the store records the
/// bytes it writes in reply, so nothing the sentinel set off is still on its
/// way: the count read here is at rest by construction. Until 0696 the count was
/// called at rest after 25 unchanged polls, and under load an ingest could land
/// after that and be counted in the next measurement.
fn caught_up(daemon: &RealDaemon, root: &Path, id: &str) -> u64 {
  write_sentinel(root, id);
  until(&format!("the daemon holds {id}"), || {
    holds(daemon, root, id)
  });
  daemon.ingested(root)
}

#[test]
fn sync_to_disk_lands_beside_a_watching_daemon_and_costs_it_no_ingest() {
  let daemon = RealDaemon::start();
  let (root, issue) = project();

  let view = root
    .join("intent")
    .join("issues")
    .join(format!("{issue}.md"));
  assert!(
    view.exists(),
    "the fixture renders no {}, so there is no view to put behind the store",
    view.display()
  );

  let contacted = run(daemon.home(), &root, &["--daemon", "st", "list"]);
  assert_eq!(
    contacted.status.code(),
    Some(0),
    "the daemon could not answer for the project it is meant to watch: {}",
    text(&contacted)
  );
  // **ANTI-VACUITY.** Served and not watched is a real state, and in it the
  // run below would pass without being beside a watcher at all.
  until("the daemon is watching the project", || {
    daemon.watching(&root)
  });

  // Whatever the fixture and the daemon's open wrote is behind this sentinel.
  let start = caught_up(&daemon, &root, "ST0900");

  // The control: what one sentinel costs with nothing else in front of it.
  let control = caught_up(&daemon, &root, "ST0901");
  let alone = control - start;
  assert!(
    alone > 0,
    "the daemon holds a hand-written sentinel and its ingest count did not move, so this instrument cannot see an ingest and every comparison below is vacuous"
  );

  // One view put behind the store WHILE WATCHED. Measured, not assumed: the
  // watcher leaves a generated view's path alone, so the removal costs no
  // ingest and the view stays gone -- whereas a view removed before the daemon
  // knew the project is rendered back by its first ingest, and so is this one
  // by the next thread ingest. Nothing else is written until the landing.
  //
  // **SO NO BARRIER STANDS BETWEEN THE REMOVAL AND THE LANDING** (issue 0696).
  // A sentinel there is a thread ingest, and a thread ingest renders the view
  // back before `--to-disk` can land it. The removal's cost is measured by the
  // one barrier after the landing instead, and the landing's own `wrote:` line
  // is what says the view was still gone when it ran.
  std::fs::remove_file(&view).expect("remove the view");

  let synced = run(daemon.home(), &root, &["sync", "--to-disk"]);
  assert_eq!(
    synced.status.code(),
    Some(0),
    "sync --to-disk was refused beside a daemon watching this tree. It is a projection from the store and runs no ingest walk (issue 0500): {}",
    text(&synced)
  );
  let wrote = format!("wrote: intent/issues/{issue}.md");
  assert!(
    String::from_utf8_lossy(&synced.stdout)
      .lines()
      .any(|line| line == wrote),
    "sync --to-disk did not write the removed view back, so the daemon had rendered it already and the arm proves nothing: {}",
    text(&synced)
  );
  let landed = std::fs::read(&view).expect("sync --to-disk wrote the view back");

  let behind = caught_up(&daemon, &root, "ST0902") - control;
  assert_eq!(
    behind, alone,
    "a sentinel cost {alone} ingest(s) alone and {behind} with the view's removal and its landing in front of it, so the daemon ingested one of them. A generated view is outside the watcher's scope, removed or landed, and this says one is not"
  );
  assert_eq!(
    std::fs::read(&view).expect("the view is still there"),
    landed,
    "the view changed after sync --to-disk landed it, so something rewrote it"
  );

  let _ = std::fs::remove_dir_all(&root);
}
