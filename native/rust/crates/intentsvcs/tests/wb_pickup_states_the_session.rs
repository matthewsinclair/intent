//! ST0069 AC-14.7: `wb pickup` states a session start -- the node is active,
//! its heartbeat moves, and the session and focus it names are recorded --
//! while a pickup that names neither keeps what the header holds.

use crate::common::Fixture;
use intentsvcs::model::WbNodeStatus;

#[test]
fn a_pickup_makes_a_paused_node_active_and_records_what_it_names() {
  let fx = Fixture::new();
  let mut f = fx.facade();
  f.wb_register("cc", "Control Claude", "control")
    .expect("register cc");
  f.wb_release("cc").expect("release");
  assert_eq!(
    f.board("cc").expect("board").node.status,
    WbNodeStatus::Paused
  );

  let up = f
    .wb_pickup("cc", Some("session-1"), Some("the renderers"), false)
    .expect("pick up");
  assert_eq!(up.board.board.node.status, WbNodeStatus::Active);
  assert_eq!(up.board.board.node.session_id.as_deref(), Some("session-1"));
  assert_eq!(up.board.board.node.focus, "the renderers");

  let again = f.wb_pickup("cc", None, None, false).expect("pick up again");
  assert_eq!(
    again.board.board.node.session_id.as_deref(),
    Some("session-1")
  );
  assert_eq!(
    again.board.board.node.focus, "the renderers",
    "an unnamed focus is not an empty one"
  );
}

/// Issue 0416: a pickup returns `hv`'s directives, decisions and watch-outs in
/// full, in board order, and says for every peer what it leaves out -- so a
/// peer's header is never read as its whole board. `hv`'s own pickup carries no
/// standing list, because its board already holds them.
#[test]
fn a_pickup_carries_hv_s_standing_content_and_counts_what_it_leaves_out() {
  use intentsvcs::model::WbItemKind;

  let fx = Fixture::new();
  let mut f = fx.facade();
  for (node, name, role) in [
    ("hv", "Hypervisor", "hypervisor"),
    ("cc", "Control Claude", "control"),
    ("dc", "DevX Claude", "worker"),
  ] {
    f.wb_register(node, name, role).expect("register");
  }
  f.wb_decide("hv", "the flake is known").expect("a decision");
  f.wb_add(
    "hv",
    WbItemKind::Watchout,
    "a_daemon_outlives_nobody is ruled",
  )
  .expect("a watch-out");
  f.wb_add("hv", WbItemKind::Directive, "NO RELEASE, NO PUSH")
    .expect("a directive");
  f.wb_add("hv", WbItemKind::Todo, "hv's own chore")
    .expect("a todo");
  let archived = f
    .wb_add("hv", WbItemKind::Watchout, "an archived caution")
    .expect("a watch-out");
  f.wb_archive("hv", WbItemKind::Watchout, archived)
    .expect("archive it");
  f.wb_add("cc", WbItemKind::Watchout, "cc's caution")
    .expect("a peer's watch-out");
  f.wb_add("cc", WbItemKind::Doing, "cc's work")
    .expect("a peer's doing");

  let up = f.wb_pickup("dc", None, None, false).expect("pick up");
  let standing: Vec<(WbItemKind, &str)> = up
    .standing
    .as_deref()
    .expect("dc reads hv's board")
    .iter()
    .map(|i| (i.kind, i.text.as_str()))
    .collect();
  assert_eq!(
    standing,
    vec![
      (WbItemKind::Directive, "NO RELEASE, NO PUSH"),
      (WbItemKind::Watchout, "a_daemon_outlives_nobody is ruled"),
      (WbItemKind::Decision, "the flake is known"),
    ],
    "hv's live standing kinds, in board order, and nothing archived"
  );

  let unshown = |moniker: &str| -> Vec<(WbItemKind, usize)> {
    up.peers
      .iter()
      .find(|p| p.node.moniker == moniker)
      .expect("the peer")
      .unshown
      .iter()
      .map(|c| (c.kind, c.count))
      .collect()
  };
  assert_eq!(
    unshown("cc"),
    vec![(WbItemKind::Doing, 1), (WbItemKind::Watchout, 1)],
    "a peer's header carries the count of what the read leaves out"
  );
  assert_eq!(
    unshown("hv"),
    vec![(WbItemKind::Todo, 1)],
    "hv's standing kinds are shown above, so only the rest is counted"
  );

  let hv = f.wb_pickup("hv", None, None, false).expect("hv picks up");
  assert_eq!(
    hv.standing, None,
    "hv's own board already carries its standing content"
  );
}

/// Issue 0416: with no `hv` board there is no standing content to read, and the
/// pickup says so by leaving `standing` absent -- an empty list would claim `hv`
/// holds none, about a board the read never looked at.
#[test]
fn a_pickup_with_no_hv_board_carries_no_standing_list() {
  let fx = Fixture::new();
  let mut f = fx.facade();
  f.wb_register("cc", "Control Claude", "control")
    .expect("register cc");
  f.wb_register("dc", "DevX Claude", "worker")
    .expect("register dc");

  let up = f.wb_pickup("dc", None, None, false).expect("pick up");
  assert_eq!(up.standing, None, "no hv board, so nothing was read");
  assert_eq!(up.peers.len(), 1, "cc is still a peer");
}
