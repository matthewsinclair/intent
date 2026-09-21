//! Issue `0495`: a board row that reached the store and not `board.json` was
//! reported by nothing, while the same failure on a thread was reported as
//! store-stale.
//!
//! **THE STAGING CHANGES RENDERED CONTENT, AND THE EVIDENCE IS THE ITEM ITSELF.**
//! A refused `wb touch` inside the minute of the last good one leaves a file a
//! fresh render would reproduce byte for byte, which is how an earlier reading of
//! this issue went wrong. So the row written here is an item, and the
//! precondition is that the store holds its text and `board.json` does not.
//!
//! Both controls run on one fixture: quiet before the refused write, reported
//! after it, quiet again once a board write lands the view.

use crate::common::{Fixture, ctx};
use intentsvcs::finding::{Finding, FindingClass};
use intentsvcs::model::WbItemKind;
use std::os::unix::fs::PermissionsExt;

const ROW: &str = "the row whose render was refused";

fn store_stale(fx: &Fixture) -> Vec<Finding> {
  intentsvcs::doctor::diagnose(&fx.project(), &ctx(), None, intentsvcs::doctor::Scope::All)
    .findings
    .into_iter()
    .filter(|f| f.class == FindingClass::StoreStale)
    .collect()
}

fn set_mode(fx: &Fixture, rel: &str, mode: u32) {
  std::fs::set_permissions(fx.path(rel), std::fs::Permissions::from_mode(mode))
    .unwrap_or_else(|e| panic!("chmod {mode:o} {rel}: {e}"));
}

#[test]
fn a_board_row_the_store_holds_and_its_file_does_not_is_store_stale_until_a_board_write_lands_it() {
  let fx = Fixture::new();
  let mut f = fx.facade_on_disk();
  f.wb_register("cc", "Control Claude", "control")
    .expect("register cc");
  f.wb_register("vc", "Validation Claude", "validation")
    .expect("register vc");
  f.wb_add("cc", WbItemKind::Hold, "a row whose render landed")
    .expect("a first row");
  let quiet = store_stale(&fx);
  assert!(
    quiet.is_empty(),
    "negative control: a board whose render landed is not stale: {quiet:?}"
  );

  set_mode(&fx, "intent/whiteboard/cc", 0o555);
  let added = f.wb_add("cc", WbItemKind::Watchout, ROW);
  set_mode(&fx, "intent/whiteboard/cc", 0o755);
  added.expect("the row lands in the store though its render is refused");
  let held = f.board("cc").expect("cc's board");
  assert!(
    held.items.iter().any(|i| i.text == ROW),
    "precondition: the store holds the row: {:?}",
    held.items
  );
  assert!(
    !fx.read("intent/whiteboard/cc/board.json").contains(ROW),
    "precondition: board.json does not, so the render was refused"
  );

  let stale = store_stale(&fx);
  assert!(
    stale.iter().any(|s| s.file == "intent/.cache/intent.db"
      && s.detail.contains("whiteboard rows")
      && s.detail.contains("cc")),
    "the store ahead of board.json is reported, naming the node: {stale:?}"
  );
  assert!(
    !stale.iter().any(|s| s.detail.contains("vc")),
    "and names only the node whose board differs: {stale:?}"
  );

  f.wb_touch("cc").expect("a board write lands the view");
  assert!(
    fx.read("intent/whiteboard/cc/board.json").contains(ROW),
    "precondition: the touch rendered the row"
  );
  let landed = store_stale(&fx);
  assert!(
    landed.is_empty(),
    "negative control: quiet once a board write lands the view: {landed:?}"
  );
}
