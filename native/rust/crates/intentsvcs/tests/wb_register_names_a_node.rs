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
