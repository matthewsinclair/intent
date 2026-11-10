//! **`.gitattributes`' archive block is rewritten where it stands, appended
//! only when absent, and held when it cannot be bounded** (issue 0650).
//!
//! The block keeps the commit gate's tracked dispatchers out of every release
//! archive. In `.gitattributes` the last line that matches a path wins, so a
//! project's own opt-out after the block must keep the last word through every
//! later upgrade, which is why the block is never moved to the end.

use intentsvcs::canon::{BlockHeld, archive_block, tracked_hooks_rel, with_archive_block};
use std::path::Path;

#[test]
fn an_absent_block_is_appended_after_a_blank_line_and_the_file_kept() {
  let devbin =
    "# >>> devbin: line endings >>>\nbin/devbin text=auto eol=lf\n# <<< devbin: line endings <<<\n";
  let written = with_archive_block(devbin, ".githooks")
    .expect("an absent block is not held")
    .expect("an absent block is written");
  assert_eq!(written, format!("{devbin}\n{}", archive_block(".githooks")));
  assert!(
    written.contains("\n/.githooks export-ignore\n"),
    "the block names the hooks directory anchored at the root: {written}"
  );
  let fresh = with_archive_block("", "bin/hooks")
    .expect("an absent file is not held")
    .expect("an absent file is written");
  assert_eq!(fresh, archive_block("bin/hooks"));
}

#[test]
fn a_canonical_block_writes_nothing() {
  let canonical = format!(
    "*.ex linguist-vendored=true\n\n{}",
    archive_block(".githooks")
  );
  assert_eq!(with_archive_block(&canonical, ".githooks"), Ok(None));
}

#[test]
fn a_stale_block_is_rewritten_where_it_stands_and_the_opt_out_after_it_keeps_the_last_word() {
  let opt_out = "/bin/hooks -export-ignore\n";
  let stale = format!("{}{opt_out}", archive_block(".githooks"));
  let rewritten = with_archive_block(&stale, "bin/hooks")
    .expect("a bounded block is not held")
    .expect("a stale block is written");
  assert_eq!(
    rewritten,
    format!("{}{opt_out}", archive_block("bin/hooks"))
  );
}

#[test]
fn a_block_that_cannot_be_bounded_is_held_with_its_reason() {
  let block = archive_block(".githooks");
  let open = block.lines().next().expect("the block has an opening line");
  assert_eq!(
    with_archive_block(&format!("{open}\n/.githooks export-ignore\n"), ".githooks"),
    Err(BlockHeld::ArchiveUnclosed)
  );
  assert_eq!(
    with_archive_block(&format!("{block}{block}"), ".githooks"),
    Err(BlockHeld::ArchiveDoubled)
  );
}

#[test]
fn only_a_hooks_directory_inside_the_work_tree_is_tracked() {
  let root = Path::new("/project");
  assert_eq!(
    tracked_hooks_rel(root, &root.join(".githooks")),
    Some(".githooks".to_string())
  );
  assert_eq!(
    tracked_hooks_rel(root, &root.join("bin/hooks")),
    Some("bin/hooks".to_string())
  );
  assert_eq!(tracked_hooks_rel(root, &root.join(".git/hooks")), None);
  assert_eq!(
    tracked_hooks_rel(root, Path::new("/elsewhere/.git/hooks")),
    None
  );
}
