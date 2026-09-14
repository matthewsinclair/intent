//! ST0069 AC-14.7: a node joins by its arguments, once, and keeps the values
//! it joined with -- the door that still exists when no header is hand-written.

use crate::common::Fixture;
use intentsvcs::facade::FacadeError;

#[test]
fn a_node_named_by_its_arguments_is_registered_once_and_keeps_its_values() {
  let fx = Fixture::new();
  let mut f = fx.facade();
  assert_eq!(f.wb_register("zz", "Zed", "worker").expect("register"), 1);
  let node = f.board("zz").expect("the node has a board").node;
  assert_eq!((node.name.as_str(), node.role.as_str()), ("Zed", "worker"));
  assert_eq!(
    f.wb_register("zz", "Zed", "worker")
      .expect("the same values"),
    0,
    "the same values write nothing"
  );
  assert!(
    matches!(
      f.wb_register("zz", "Zee", "worker"),
      Err(FacadeError::WbRegisteredDifferently { .. })
    ),
    "different values for a registered moniker are refused"
  );
}

#[test]
fn registering_a_node_with_a_hand_authored_board_leaves_the_board_for_its_migration() {
  // Issue 0379: register rendered the row over wip.md, so the migrate that followed carried an empty board.
  let fx = Fixture::new();
  let dir = fx.path("intent/whiteboard/gg");
  std::fs::create_dir_all(&dir).expect("node dir");
  let hand = "---\nnode: gg\nname: Gee\nrole: worker\nstatus: active\n---\n\n# Gee (gg)\n\n## DOING\n\n- The work the board already carries.\n";
  std::fs::write(dir.join("wip.md"), hand).expect("a hand-authored board");
  let mut f = fx.facade_on_disk();
  f.wb_register("gg", "Gee", "worker").expect("register");
  assert_eq!(
    fx.read("intent/whiteboard/gg/wip.md"),
    hand,
    "registering writes no file over a hand-authored board"
  );
  assert!(
    matches!(f.wb_touch("gg"), Err(FacadeError::WbNotMigrated { .. })),
    "and the node is not born migrated, so no board write can render over the board first"
  );
  f.wb_migrate("gg").expect("carry the board");
  assert!(
    f.board("gg")
      .expect("the board")
      .items
      .iter()
      .any(|i| i.text.contains("already carries")),
    "the migration carries what the board held"
  );
}
