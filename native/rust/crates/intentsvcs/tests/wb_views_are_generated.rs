//! ST0069 AC-14.2 (AT-14.2), the half the cutover satisfies: once the views are
//! wired, a node's board and its inboxes are rendered from the store and nowhere else,
//! so a hand edit of either is view skew -- and an estate whose whiteboard is
//! still hand-authored, with no rows, sees no view and no skew at all.

use crate::common::{Fixture, ctx};
use intentsvcs::facade::FacadeError;
use intentsvcs::finding::{Finding, FindingClass};
use intentsvcs::model::WbItemKind;

const HAND_BOARD: &str = "---\nnode: dc\nname: DevX Claude\nrole: worker\nheartbeat_at: 2026-09-12 18:31Z\nstatus: active\nfocus: \"the migration verb\"\nclaims: []\n---\n\n# DevX Claude (dc)\n\n## DOING\n\n- The busiest section on every real board.\n";

fn whiteboard_findings(fx: &Fixture) -> Vec<Finding> {
  intentsvcs::doctor::diagnose(&fx.project(), &ctx(), None, intentsvcs::doctor::Scope::All)
    .findings
    .into_iter()
    .filter(|f| f.file.starts_with("intent/whiteboard/"))
    .collect()
}

#[test]
fn a_hand_edited_board_is_view_skew_once_its_node_has_rows() {
  let fx = Fixture::new();
  {
    let mut f = fx.facade_on_disk();
    f.wb_register("cc", "Control Claude", "control")
      .expect("register cc");
    f.wb_register("vc", "Validation Claude", "validation")
      .expect("register vc");
    f.wb_add("cc", WbItemKind::Hold, "held until the pair is rebuilt")
      .expect("a hold");
    f.sync_to_disk(&intentsvcs::sync::Scope::All)
      .expect("project the views");
  }
  let board = fx.read("intent/whiteboard/cc/wip.md");
  assert!(
    board.contains("## Holds\n\n- held until the pair is rebuilt\n"),
    "precondition: the projection rendered the board from its rows: {board}"
  );
  assert!(
    fx.read("intent/whiteboard/cc/inbox.vc.md")
      .starts_with("# inbox: vc -> cc\n\n_(empty)_\n"),
    "and every inbox addressed to it, an empty one included"
  );
  let clean = whiteboard_findings(&fx);
  assert!(clean.is_empty(), "a rendered board is clean: {clean:?}");

  std::fs::write(
    fx.path("intent/whiteboard/cc/wip.md"),
    format!("{board}\na hand edit\n"),
  )
  .expect("hand-edit the board");
  let findings = whiteboard_findings(&fx);
  assert!(
    findings
      .iter()
      .any(|f| f.file == "intent/whiteboard/cc/wip.md" && f.class == FindingClass::ViewSkew),
    "a hand edit of a generated board is skew: {findings:?}"
  );
}

#[test]
fn a_hand_authored_whiteboard_with_no_rows_has_no_view_and_no_skew() {
  let fx = Fixture::new();
  let dir = fx.path("intent/whiteboard/cc");
  std::fs::create_dir_all(&dir).expect("node dir");
  std::fs::write(
    dir.join("wip.md"),
    "---\nnode: cc\nname: Control Claude\nrole: control\nstatus: active\n---\n\n# Control Claude (cc)\n\nhand-written\n",
  )
  .expect("a hand-authored board");
  let findings = whiteboard_findings(&fx);
  assert!(findings.is_empty(), "no rows means no view: {findings:?}");
}

#[test]
fn a_board_write_lands_the_node_s_view_on_disk_at_the_write() {
  let fx = Fixture::new();
  {
    let mut f = fx.facade_on_disk();
    f.wb_register("cc", "Control Claude", "control")
      .expect("register cc");
  }
  fx.git_init().git_commit_all();
  let before = fx.read("intent/whiteboard/cc/wip.md");
  {
    let mut f = fx.facade_on_disk();
    f.wb_add("cc", WbItemKind::Hold, "held until the pair is rebuilt")
      .expect("a hold");
  }
  let board = fx.read("intent/whiteboard/cc/wip.md");
  assert!(
    board != before && board.contains("## Holds\n\n- held until the pair is rebuilt\n"),
    "the write rewrote the node's view with its new item: {board}"
  );
  let findings = whiteboard_findings(&fx);
  assert!(
    findings.is_empty(),
    "the view on disk is byte-equal to a fresh render: {findings:?}"
  );
  let status = std::process::Command::new("git")
    .args(["status", "--porcelain", "--", "intent/whiteboard/cc/wip.md"])
    .current_dir(fx.root())
    .output()
    .expect("git status");
  assert_eq!(
    String::from_utf8_lossy(&status.stdout),
    " M intent/whiteboard/cc/wip.md\n",
    "and git sees the view as changed"
  );
}

#[test]
fn a_node_registered_from_its_header_refuses_a_board_write_until_it_is_migrated() {
  let fx = Fixture::new();
  let dir = fx.path("intent/whiteboard/dc");
  std::fs::create_dir_all(&dir).expect("node dir");
  let hand = HAND_BOARD;
  std::fs::write(dir.join("wip.md"), hand).expect("a hand-authored board");
  let mut f = fx.facade_on_disk();
  f.register_roster().expect("register by header");
  assert!(
    matches!(f.wb_touch("dc"), Err(FacadeError::WbNotMigrated { .. })),
    "a board write before the migration refuses by name"
  );
  assert_eq!(
    fx.read("intent/whiteboard/dc/wip.md"),
    hand,
    "and the hand-authored board is byte-identical"
  );

  f.wb_migrate("dc").expect("carry the board");
  f.wb_touch("dc").expect("a board write after the migration");
  let board = fx.read("intent/whiteboard/dc/wip.md");
  assert!(
    board != hand && board.contains("- The busiest section on every real board.\n"),
    "the write lands the render of what was carried: {board}"
  );
  let findings = whiteboard_findings(&fx);
  assert!(findings.is_empty(), "and the render is clean: {findings:?}");
}

#[test]
fn a_migration_lands_the_carried_board_on_disk() {
  // Issue 0380: migrate refreshed the index only, so the carried board stayed hand-authored on disk and doctor reported it as skew.
  let fx = Fixture::new();
  let dir = fx.path("intent/whiteboard/dc");
  std::fs::create_dir_all(&dir).expect("node dir");
  std::fs::write(dir.join("wip.md"), HAND_BOARD).expect("a hand-authored board");
  let mut f = fx.facade_on_disk();
  f.register_roster().expect("register by header");
  f.wb_migrate("dc").expect("carry the board");
  let board = fx.read("intent/whiteboard/dc/wip.md");
  assert!(
    board != HAND_BOARD && board.contains("- The busiest section on every real board.\n"),
    "the migration itself lands the render of what it carried: {board}"
  );
  let findings = whiteboard_findings(&fx);
  assert!(
    findings.is_empty(),
    "and the tree agrees with the store: {findings:?}"
  );
}

/// Issue 0375: a directive renders under `## Standing directives` on `hv`'s
/// board, no other board carries the section, and no other node can write one.
#[test]
fn standing_directives_render_on_hv_s_board_and_no_other() {
  let fx = Fixture::new();
  {
    let mut f = fx.facade_on_disk();
    f.wb_register("hv", "Hypervisor", "hypervisor")
      .expect("register hv");
    f.wb_register("cc", "Control Claude", "control")
      .expect("register cc");
    f.wb_add(
      "hv",
      WbItemKind::Directive,
      "no release without hv at the terminal",
    )
    .expect("a directive on hv's board");
    assert!(
      matches!(
        f.wb_add("cc", WbItemKind::Directive, "a node issuing a directive"),
        Err(FacadeError::WbDirectiveOffHv { .. })
      ),
      "a directive on any board but hv's is refused by name"
    );
    f.sync_to_disk(&intentsvcs::sync::Scope::All)
      .expect("project the views");
  }
  let hv = fx.read("intent/whiteboard/hv/wip.md");
  assert!(
    hv.contains("## Standing directives\n\n- no release without hv at the terminal\n"),
    "hv's board renders its directive under the protocol's section: {hv}"
  );
  let cc = fx.read("intent/whiteboard/cc/wip.md");
  assert!(
    !cc.contains("## Standing directives"),
    "and a board that is not hv's carries no such section, empty or not: {cc}"
  );
}
