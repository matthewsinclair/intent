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
//! **WHAT KEEPS THE COST AT ZERO IS SCOPE, AND THE ARM SAYS SO RATHER THAN
//! CREDITING THE LANDING RECORD.** Measured while building it: removing a
//! generated view from a watched tree costs no ingest, and the arm stays green
//! with `sync_to_disk`'s `record_landed` call deleted. So it guards against the
//! watcher's scope taking in the views `--to-disk` lands -- the loop issue 0311
//! was about -- and it does not pin `record_landed`, which other tests own. Its
//! instrument is proven to see an ingest by the sentinel, which costs one.

use std::path::Path;

use crate::common::{ATTEMPTS, PAUSE, RealDaemon, short_dir};

/// Tries for the watcher's FIRST event on a fresh project, which can wait on
/// the watch being armed; the same allowance `intentd`'s own tests give it.
const ARM_ATTEMPTS: u32 = ATTEMPTS * 12;

/// Consecutive unchanged polls that count as the daemon being at rest: longer
/// than the watcher's debounce, as `intentd`'s own `daemon_watch.rs` settles.
const STABLE_FOR: u32 = 25;

fn project() -> std::path::PathBuf {
  let root = short_dir("todisk-proj");
  intentsvcs::init::init(&root, "ToDisk", "test", env!("CARGO_PKG_VERSION"))
    .expect("the shipped initialiser creates a project");
  let opened = intentsvcs::project::Project::open(&root).expect("the project just created");
  let ctx = intentsvcs::facade::FacadeContext {
    principal: "test".to_string(),
    project_id: opened.config().project_id.clone().unwrap_or_default(),
    version: env!("CARGO_PKG_VERSION").to_string(),
  };
  let mut facade = intentsvcs::facade::Facade::open(opened, ctx).expect("open the new project");
  facade
    .st_new("Landed beside a watcher")
    .expect("mint one thread");
  facade
    .issue_add(
      "Landed beside a watcher",
      Some("low"),
      None,
      "The view this arm lands.",
    )
    .expect("file one issue");
  root
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

/// The ingest count once it has stopped moving.
///
/// Panics if it never stops, because a count that climbs with nothing editing
/// the tree is the daemon ingesting its own writes, which is a finding.
fn settle(daemon: &RealDaemon, root: &Path) -> u64 {
  let mut last = daemon.ingested(root);
  let mut still = 0;
  for _ in 0..ATTEMPTS {
    std::thread::sleep(PAUSE);
    let now = daemon.ingested(root);
    if now == last {
      still += 1;
      if still >= STABLE_FOR {
        return now;
      }
    } else {
      last = now;
      still = 0;
    }
  }
  panic!("the ingest count never came to rest ({last} and climbing) with nothing editing the tree");
}

/// What one sentinel edit costs, from rest to rest.
///
/// The sentinel is a hand-written thread file, the edit `intentd`'s own
/// `daemon_watch.rs` drives, because a thread is in the watcher's scope by
/// construction. A thread ingest re-renders the thread views, which is why the
/// view this arm lands is an issue's.
fn sentinel_cost(daemon: &RealDaemon, root: &Path, id: &str) -> u64 {
  let before = settle(daemon, root);
  let path = root.join("intent/.canon/st").join(format!("{id}.json"));
  std::fs::write(
    &path,
    format!(
      "{{\n  \"schema\": \"intent/thread@3.0\",\n  \"id\": \"{id}\",\n  \"title\": \"Written by hand\",\n  \"status\": \"wip\",\n  \"created\": \"2026-09-21\",\n  \"objective\": \"\",\n  \"context\": \"\"\n}}\n"
    ),
  )
  .expect("write the sentinel thread");
  until("the daemon ingested the sentinel", || {
    daemon.ingested(root) > before
  });
  settle(daemon, root) - before
}

#[test]
fn sync_to_disk_lands_beside_a_watching_daemon_and_costs_it_no_ingest() {
  let daemon = RealDaemon::start();
  let root = project();

  let view = root.join("intent").join("issues").join("0001.md");
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

  // The control: what one sentinel costs with nothing else in front of it.
  let alone = sentinel_cost(&daemon, &root, "ST0901");

  // One view put behind the store WHILE WATCHED. Measured, not assumed: the
  // watcher leaves a generated view's path alone, so the removal costs no
  // ingest and the view stays gone -- whereas a view removed before the daemon
  // knew the project is rendered back by its first ingest, and so is this one
  // by the next thread ingest. Nothing else is written until the landing.
  let rest = settle(&daemon, &root);
  std::fs::remove_file(&view).expect("remove the view");
  assert_eq!(
    settle(&daemon, &root),
    rest,
    "removing a generated view cost the daemon an ingest, so the comparison below would count it"
  );
  assert!(
    !view.exists(),
    "the daemon rendered the removed view itself, so `--to-disk` below has nothing to land and the arm proves nothing"
  );

  let synced = run(daemon.home(), &root, &["sync", "--to-disk"]);
  assert_eq!(
    synced.status.code(),
    Some(0),
    "sync --to-disk was refused beside a daemon watching this tree. It is a projection from the store and runs no ingest walk (issue 0500): {}",
    text(&synced)
  );
  let landed = std::fs::read(&view).expect("sync --to-disk wrote the view back");

  let behind = sentinel_cost(&daemon, &root, "ST0902");
  assert_eq!(
    behind, alone,
    "a sentinel cost {alone} ingest(s) alone and {behind} with the landing in front of it, so the daemon ingested what sync --to-disk wrote. The generated views a landing writes are outside the watcher's scope, and this says one is not"
  );
  assert_eq!(
    std::fs::read(&view).expect("the view is still there"),
    landed,
    "the view changed after sync --to-disk landed it, so something rewrote it"
  );

  let _ = std::fs::remove_dir_all(&root);
}
