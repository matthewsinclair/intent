//! ST0095 WP-06, AC-06.3: **no verb mints an ordinal.** Every verb that makes
//! a record's id goes through `id::mint`, whose only constructor draws a
//! git-style id from the event that makes the record. On a 3.4.0-format
//! project, where a new git-style id would be one an older Intent misreads,
//! each of those verbs refuses before it writes, naming `intent upgrade`. The
//! two remints take the id they move and never the one they move to, and the
//! two-argument `renumber` that named one is refused: that half is driven by
//! `two_clones_that_mint_one_id_are_repaired_by_remint.rs`.
//!
//! `sync`'s repair of an ordinal two branches both minted moves it through the
//! same mint, and `a_pull_is_repaired_by_one_command.rs` drives it. The verbs
//! below are named by hand: nothing in the tree lists which verbs mint, so a
//! new one must be added here.

use crate::common::{created, ok, run};
use intentsvcs::id::{IssueId, ThreadId};
use std::collections::BTreeMap;
use std::path::Path;

/// Every file under the project's committed canon with its bytes, so a
/// refusal can be shown to have written nothing: in 3.4.0's format a package
/// is inside its thread's file, so a write would change a file, not add one.
fn canon_files(root: &Path) -> BTreeMap<std::path::PathBuf, Vec<u8>> {
  let canon = root.join("intent/.canon");
  intentsvcs::project::Project::files_in(&canon)
    .into_iter()
    .map(|rel| {
      let bytes = std::fs::read(canon.join(&rel)).expect("read a canon file");
      (rel, bytes)
    })
    .collect()
}

// ST0095 AT-06.3: the arm below.
#[test]
fn every_mint_makes_a_git_style_id_and_refuses_before_the_upgrade() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  ok(root, &["init", "mints"]);
  let st = created(&ok(root, &["st", "new", "a minted thread"]));
  let wp = created(&ok(root, &["wp", "new", st.as_str(), "a minted package"]));
  let issue = created(&ok(root, &["issues", "add", "a minted issue"]));
  assert!(
    ThreadId::parse(&st).expect("a thread id").is_hashed(),
    "st new minted {st}"
  );
  assert!(
    crate::common::package_of(&wp).is_hashed(),
    "wp new minted {wp}"
  );
  assert!(
    IssueId::parse(&issue).expect("an issue id").is_hashed(),
    "issues add minted {issue}"
  );

  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  ok(root, &["init", "ordinal"]);
  crate::common::born_ordinal(root);
  let thread = ThreadId::from_ordinal(1)
    .expect("an ordinal thread")
    .stored();
  crate::common::seed_ordinal_thread(root, &thread, "a 3.4.0 thread", &[]);
  // The project still opens: what refuses below is the mint, not the project.
  assert!(
    ok(root, &["st", "show", thread.as_str()]).contains("a 3.4.0 thread"),
    "a read verb works before the upgrade"
  );
  for verb in [
    &["st", "new", "refused"][..],
    &["wp", "new", thread.as_str(), "refused"][..],
    &["issues", "add", "refused"][..],
  ] {
    let before = canon_files(root);
    let (out, err, code) = run(root, verb);
    assert_ne!(
      code,
      0,
      "`intent {}` minted before the upgrade:\n{out}",
      verb.join(" ")
    );
    assert!(
      err.contains("intent upgrade"),
      "`intent {}` names the upgrade:\n{err}",
      verb.join(" ")
    );
    assert_eq!(
      canon_files(root),
      before,
      "`intent {}` wrote canon before refusing",
      verb.join(" ")
    );
  }
}
