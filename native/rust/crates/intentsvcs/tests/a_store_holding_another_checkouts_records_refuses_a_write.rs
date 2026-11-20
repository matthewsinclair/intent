//! Issue 0739 (cc's meaning, 2026-11-20): **a working copy that switches
//! branches without its hooks keeps the other branch's records in its store,
//! and a write over that store refuses, naming `intent sync --apply`, before
//! it writes anything.** One `sync --apply`, the step the post-checkout hook
//! runs, then clears it.
//!
//! Real git, hooks off. The store lives under `intent/.cache/`, which git does
//! not carry, so it survives the switch exactly as a person's does. The red is
//! the base: there the write passes, renders the other branch's records into
//! this checkout, and every verb after it refuses the project as unmigrated.

use std::collections::BTreeMap;
use std::path::Path;

use crate::common::Fixture;
use intentsvcs::facade::FacadeError;

/// Every file in the working tree that git or a person sees, project-relative,
/// with its bytes: everything but `.git/` and the store's own `intent/.cache/`.
/// Bytes too, so a rewrite of a file already there is a write as well.
fn tree(root: &Path) -> BTreeMap<String, Vec<u8>> {
  fn walk(root: &Path, dir: &Path, out: &mut BTreeMap<String, Vec<u8>>) {
    for entry in std::fs::read_dir(dir).expect("read a directory") {
      let path = entry.expect("an entry").path();
      let rel = path
        .strip_prefix(root)
        .expect("under the root")
        .to_string_lossy()
        .into_owned();
      match (path.is_dir(), rel.as_str()) {
        (true, ".git" | "intent/.cache") => {}
        (true, _) => walk(root, &path, out),
        (false, _) => {
          out.insert(rel, std::fs::read(&path).expect("read a file"));
        }
      }
    }
  }
  let mut out = BTreeMap::new();
  walk(root, root, &mut out);
  out
}

#[test]
fn a_write_over_a_store_holding_another_branchs_records_refuses_until_one_sync() {
  let fx = Fixture::new();
  fx.git_init();
  fx.git(&["config", "core.hooksPath", "/dev/null"]);
  std::fs::write(fx.root().join(".git/info/exclude"), "intent/.cache/\n")
    .expect("exclude the store");
  fx.facade_on_disk()
    .st_new("On main")
    .expect("a thread on main");
  fx.git(&["add", "-A"]);
  fx.git(&["commit", "-q", "-m", "main"]);
  fx.git(&["branch", "-M", "main"]);

  fx.git(&["switch", "-q", "-c", "mr-a"]);
  let elsewhere = fx
    .facade_on_disk()
    .st_new("Only on mr-a")
    .expect("a thread on the other branch");
  fx.git(&["add", "-A"]);
  fx.git(&["commit", "-q", "-m", "mr-a"]);

  // Back to main with no hook: the store still holds mr-a's thread.
  fx.git(&["switch", "-q", "main"]);
  let elsewhere_canon = fx.project().relative(&fx.project().thread_json(&elsewhere));
  assert!(
    !fx.root().join(&elsewhere_canon).exists(),
    "CONTROL: main does not carry mr-a's thread"
  );
  let before = tree(fx.root());

  let refused = fx.facade_on_disk().st_new("On main, after the switch");
  assert!(
    matches!(&refused, Err(FacadeError::StoreNotThisCheckouts { paths }) if *paths == vec![elsewhere_canon.clone()]),
    "a write over a store holding mr-a's thread refuses, naming its canon and nothing else: {refused:?}"
  );
  let remedy = intentsvcs::remedy::Remedy::remedy(&refused.expect_err("refused"));
  assert!(
    remedy.contains("intent sync --apply") && !remedy.contains("intent upgrade"),
    "the remedy is the hook's own step, never an upgrade: {remedy}"
  );
  // organize --apply renders every record the store holds: the same harm by
  // another door, refused at that door (cc, 2026-11-20).
  let organized = fx
    .facade_on_disk()
    .organize_as_shown(intentsvcs::organize::Mode::Apply, None);
  assert!(
    matches!(&organized, Err(FacadeError::StoreNotThisCheckouts { paths }) if *paths == vec![elsewhere_canon.clone()]),
    "organize --apply over the same store refuses too: {organized:?}"
  );
  assert_eq!(
    tree(fx.root()),
    before,
    "nothing was written by either: no view of mr-a's records, and no new thread"
  );

  sync(&fx);
  fx.facade_on_disk()
    .st_new("On main, after the sync")
    .expect("the same write passes once the store holds this checkout's canon");
  assert!(
    tree(fx.root())
      .keys()
      .all(|path| !path.contains(elsewhere.as_str())),
    "and nothing of mr-a's was rendered into main"
  );
}

/// The other arm: this checkout carries a record the store never took in.
/// The store is brought to main's canon by one sync, then the switch to mr-a
/// runs with no hook, so mr-a's thread is on disk and not in the store.
#[test]
fn a_write_over_a_store_missing_this_branchs_records_refuses_until_one_sync() {
  let fx = Fixture::new();
  fx.git_init();
  fx.git(&["config", "core.hooksPath", "/dev/null"]);
  std::fs::write(fx.root().join(".git/info/exclude"), "intent/.cache/\n")
    .expect("exclude the store");
  fx.facade_on_disk()
    .st_new("On main")
    .expect("a thread on main");
  fx.git(&["add", "-A"]);
  fx.git(&["commit", "-q", "-m", "main"]);
  fx.git(&["branch", "-M", "main"]);
  fx.git(&["switch", "-q", "-c", "mr-a"]);
  let here = fx
    .facade_on_disk()
    .st_new("Only on mr-a")
    .expect("a thread on mr-a");
  fx.git(&["add", "-A"]);
  fx.git(&["commit", "-q", "-m", "mr-a"]);
  fx.git(&["switch", "-q", "main"]);
  sync(&fx);

  // To mr-a with no hook: the store holds main's canon, without mr-a's thread.
  fx.git(&["switch", "-q", "mr-a"]);
  let here_canon = fx.project().relative(&fx.project().thread_json(&here));
  assert!(
    fx.root().join(&here_canon).exists(),
    "CONTROL: mr-a carries its thread"
  );
  let before = tree(fx.root());

  let refused = fx.facade_on_disk().st_new("On mr-a, after the switch");
  assert!(
    matches!(&refused, Err(FacadeError::StoreNotThisCheckouts { paths }) if *paths == vec![here_canon.clone()]),
    "a write over a store missing mr-a's thread refuses, naming its canon and nothing else: {refused:?}"
  );
  assert_eq!(tree(fx.root()), before, "nothing was written");

  sync(&fx);
  fx.facade_on_disk()
    .st_new("On mr-a, after the sync")
    .expect("the same write passes once the store holds this checkout's canon");
}

/// One `sync --apply`, as the post-checkout hook runs it: no terminal, nobody
/// asked.
fn sync(fx: &Fixture) {
  fx.facade_on_disk()
    .sync_apply(
      &intentsvcs::sync::Scope::All,
      &intentsvcs::plan::Asking {
        yes: true,
        terminal: false,
        shown: None,
      },
      &mut |_| intentsvcs::plan::Decision::Decline,
    )
    .expect("one sync --apply takes this checkout's canon into the store");
}
