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
    .wb_pickup("cc", Some("session-1"), Some("the renderers"))
    .expect("pick up");
  assert_eq!(up.board.node.status, WbNodeStatus::Active);
  assert_eq!(up.board.node.session_id.as_deref(), Some("session-1"));
  assert_eq!(up.board.node.focus, "the renderers");

  let again = f.wb_pickup("cc", None, None).expect("pick up again");
  assert_eq!(again.board.node.session_id.as_deref(), Some("session-1"));
  assert_eq!(
    again.board.node.focus, "the renderers",
    "an unnamed focus is not an empty one"
  );
}
