//! ST0069 AC-14.6: archival is the API's own state change. A board at its
//! bound is freed by the write that says an item is finished or a message was
//! handled -- no sweep, no fold, no later act -- and what was archived stays a
//! row.
//!
//! ST0069 AT-14.6 cites this file.

use crate::common::Fixture;
use intentsvcs::facade::FacadeError;
use intentsvcs::model::{WbItemKind, WbItemState};

#[test]
fn a_bound_is_freed_by_the_state_change_and_nothing_is_deleted() {
  let fx = Fixture::new();
  let mut f = fx.facade();
  f.wb_register("cc", "Control Claude", "control")
    .expect("register cc");
  f.wb_register("vc", "Validation Claude", "validation")
    .expect("register vc");
  let bound = f.project().config().whiteboard.clone();

  for i in 0..bound.live_items {
    f.wb_add("vc", WbItemKind::Todo, &format!("todo {i}"))
      .expect("inside the item bound");
  }
  assert!(
    matches!(
      f.wb_add("vc", WbItemKind::Todo, "one more"),
      Err(FacadeError::WbItemsFull { .. })
    ),
    "a kind at its bound refuses the next item"
  );
  assert!(
    f.wb_archive("vc", WbItemKind::Todo, 1).expect("archive"),
    "the item moved"
  );
  f.wb_add("vc", WbItemKind::Todo, "one more")
    .expect("archiving one item admits the next");

  for i in 0..bound.live_messages {
    f.wb_ask("vc", "cc", &format!("message {i}"), None, false)
      .expect("inside the inbox bound");
  }
  assert!(
    matches!(
      f.wb_ask("vc", "cc", "one more", None, false),
      Err(FacadeError::WbInboxFull { .. })
    ),
    "an inbox at its bound refuses the next message"
  );
  f.wb_clear("cc", "vc").expect("the recipient clears");
  f.wb_ask("vc", "cc", "one more", None, false)
    .expect("clearing admits the next message");

  let vc = f.board("vc").expect("vc's board");
  assert_eq!(
    vc.items
      .iter()
      .filter(|i| i.state == WbItemState::Archived)
      .count(),
    1,
    "the archived item is still a row"
  );
  let cc = f.board("cc").expect("cc's board");
  assert_eq!(
    cc.messages.len(),
    bound.live_messages + 1,
    "the handled messages are still rows"
  );
}
