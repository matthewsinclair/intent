//! ST0069 AC-14.2, the half the cutover satisfies: once the views are wired, a
//! node's board and its inboxes are rendered from the store and nowhere else,
//! so a hand edit of either is view skew -- and an estate whose whiteboard is
//! still hand-authored, with no rows, sees no view and no skew at all.

use crate::common::{Fixture, ctx};
use intentsvcs::finding::{Finding, FindingClass};
use intentsvcs::model::WbItemKind;

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
