//! `AT-03.4` (ST0074 WP-03): intentd lists a project written into the project
//! registry after it started, without a restart, and without opening it.
//!
//! **DRIVEN AGAINST A REAL DAEMON, BECAUSE THE CLAIM IS ABOUT A PROCESS OVER
//! TIME.** A unit test of the reload proves the reload; only a daemon that was
//! already running when the file changed proves the watch.

use std::time::Duration;

use crate::common::{RunningDaemon, project};
use intentsvcs::projects::{self, AddedBy};
use intentsvcs::userstate::{self, Dirs};
use intentsvcs::wire::{Op, RegisteredProject, Request, Response};

fn registry(daemon: &RunningDaemon, root: &std::path::Path) -> Vec<RegisteredProject> {
  match daemon.ask(Request {
    root: root.to_path_buf(),
    op: Op::Registry,
  }) {
    Response::Registry { projects } => projects,
    other => panic!("expected a registry listing, got {other:?}"),
  }
}

#[test]
fn a_project_added_to_the_registry_is_listed_without_a_restart() {
  let daemon = RunningDaemon::start();
  let alpha = project("Alpha").canonicalize().expect("canonical root");
  assert!(
    registry(&daemon, &alpha).iter().all(|p| p.root != alpha),
    "listed before anything registered it"
  );

  let file = userstate::project_registry_under(&Dirs::at_home(daemon.home()));
  projects::add(&file, std::slice::from_ref(&alpha), AddedBy::Discover).expect("register");

  let mut seen = Vec::new();
  for _ in 0..100 {
    seen = registry(&daemon, &alpha);
    if let Some(found) = seen.iter().find(|p| p.root == alpha) {
      assert!(
        found.listed && !found.watched && found.dispatched == 0,
        "listed and not opened: {found:?}"
      );
      let _ = std::fs::remove_dir_all(&alpha);
      return;
    }
    std::thread::sleep(Duration::from_millis(100));
  }
  panic!("the registry file changed and intentd never listed the project: {seen:?}");
}
