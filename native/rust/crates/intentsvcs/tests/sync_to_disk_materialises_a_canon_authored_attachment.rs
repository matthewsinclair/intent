//! **0082: `sync --to-disk` must materialise an attachment the store carries
//! and the disk does not.**
//!
//! `st attach` writes an attachment to the store and canon and never to disk,
//! so an attachment authored canon-first existed nowhere a reader could open
//! it -- and `--to-disk` reported `ok` over it. `st hydrate` already wrote it,
//! through `organize`'s `HydrateAttachment`; the egest never asked.
//!
//! The control arm is the other half of the same rule. Attachments are
//! AUTHORED on disk, so one that is present and differs from the store is the
//! author's newer work, not a stale copy -- and an egest that "fixed" it would
//! be 0260's loss arriving by a different door.

use crate::common::{Fixture, sample_thread};
use intentsvcs::sync::Scope;

#[test]
fn an_attachment_only_the_store_carries_reaches_disk_and_a_present_one_is_left_alone() {
  let fx = Fixture::new();
  let thread = sample_thread("ST0056");
  fx.write_thread(&thread);
  let home = fx.project().thread_dir("ST0056");
  let absent = home.join("parity/cmd-st.md");
  let present = home.join("reference.md");
  let authored = "# Reference\n\nThe author's newer draft, not yet synced.\n";
  std::fs::create_dir_all(&home).expect("the thread's directory");
  std::fs::write(&present, authored).expect("write the author's draft");
  assert!(
    !absent.exists(),
    "precondition: the nested attachment is in canon and not on disk"
  );

  let mut facade = fx.facade();
  facade.sync_to_disk(&Scope::All).expect("egest");

  let carried = thread
    .attachments
    .iter()
    .find(|a| a.path == "parity/cmd-st.md")
    .and_then(|a| a.text.clone())
    .expect("the fixture's nested attachment carries text");
  assert_eq!(
    std::fs::read_to_string(&absent).ok().as_deref(),
    Some(carried.as_str()),
    "an attachment the store carries and the disk lacks was not materialised by the egest"
  );
  assert_eq!(
    std::fs::read_to_string(&present).expect("read the draft"),
    authored,
    "the egest overwrote an attachment the author had changed on disk"
  );
}
