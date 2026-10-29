//! Issue 0662: board writes from several facades at once, as several nodes'
//! sessions make them during a fold. Each write renders every board from the
//! store as it reads it, so a write that read before another committed put its
//! older board over the newer file: the store held the edit and `board.json`
//! did not, until some later write happened to render that board again.
//!
//! **EACH WRITER IS ITS OWN FACADE ON ITS OWN CONNECTION**, as each session is
//! its own process, and they start together on a barrier so their writes land
//! inside one second. The race is not forced, so the arm repeats; what it holds
//! is that after every round each board carries its own node's last edit, and
//! an event file under `intent/.canon/events` carries it too.

use std::sync::{Arc, Barrier};

use crate::common::{Fixture, facade_ctx};
use intentsvcs::facade::Facade;
use intentsvcs::model::WbItemKind;

const NODES: [&str; 4] = ["na", "nb", "nc", "nd"];
const ROUNDS: usize = 12;

fn text(round: usize, node: &str) -> String {
  format!("round {round} text for {node}")
}

#[test]
fn concurrent_edits_leave_every_board_holding_its_own_last_edit() {
  let fx = Fixture::new();
  fx.git_init();
  {
    let mut f = fx.facade_on_disk();
    for node in NODES {
      f.wb_register(node, node, "worker").expect("register");
      f.wb_add(node, WbItemKind::Todo, "first text").expect("add");
    }
  }
  for round in 0..ROUNDS {
    let start = Arc::new(Barrier::new(NODES.len()));
    let writers: Vec<_> = NODES
      .iter()
      .map(|&node| {
        let project = fx.project();
        let start = Arc::clone(&start);
        std::thread::spawn(move || {
          let mut f = Facade::open(project, facade_ctx()).expect("open a writer's facade");
          start.wait();
          f.wb_edit(node, WbItemKind::Todo, 1, &text(round, node))
            .expect("edit");
        })
      })
      .collect();
    for writer in writers {
      writer.join().expect("a writer finished");
    }
    let events = event_files(&fx);
    for node in NODES {
      let edit = text(round, node);
      let board = fx.read(&format!("intent/whiteboard/{node}/board.json"));
      assert!(
        board.contains(&edit),
        "round {round}: {node}'s board.json lacks its own last edit, which the store holds"
      );
      assert!(
        events.iter().any(|body| body.contains(&edit)),
        "round {round}: no event file under intent/.canon/events carries {node}'s last edit"
      );
    }
  }
}

/// Every event file's text, however deep the dated tree under it.
fn event_files(fx: &Fixture) -> Vec<String> {
  fn walk(dir: &std::path::Path, out: &mut Vec<String>) {
    for entry in std::fs::read_dir(dir).expect("read dir") {
      let path = entry.expect("entry").path();
      if path.is_dir() {
        walk(&path, out);
      } else {
        out.push(std::fs::read_to_string(&path).expect("read an event file"));
      }
    }
  }
  let mut out = Vec::new();
  walk(&fx.path("intent/.canon/events"), &mut out);
  out
}
