//! **0276: the `sync --to-store` preview must name an attachment whose bytes on
//! disk differ from the store's.**
//!
//! The write carries disk attachments into canon (`collect_attachments_into`),
//! and the preview compared the store against `ingest::read`, which never reads
//! them. So a committed, clean attachment that had diverged was taken into canon
//! under "nothing the store already held was overwritten" -- neither the
//! overwrite list nor the git-cleanliness warning could see it.
//!
//! Both arms, as the issue asks: without the quiet one, a preview that warned
//! unconditionally would pass.

use crate::common::{Fixture, sample_thread};
use intentsvcs::sync::Scope;

#[test]
fn a_diverged_attachment_is_named_and_an_identical_one_is_not() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();
  facade
    .sync_to_disk(&Scope::All)
    .expect("materialise the attachments");

  assert_eq!(
    facade.sync_overwrite(&Scope::All).expect("preview"),
    Vec::<String>::new(),
    "CONTROL: every attachment on disk matches the store, so nothing is overwritten"
  );

  let reference = fx.project().thread_dir("ST0056").join("reference.md");
  std::fs::write(&reference, "# Reference\n\nA quokka, and 33 more bytes.\n")
    .expect("diverge the attachment");

  let preview = facade.sync_overwrite(&Scope::All).expect("preview");
  assert!(
    preview
      .iter()
      .any(|line| line.contains("ST0056") && line.contains("reference.md")),
    "an attachment whose bytes differ on disk is about to enter canon, and the preview did not name it: {preview:?}"
  );
}
