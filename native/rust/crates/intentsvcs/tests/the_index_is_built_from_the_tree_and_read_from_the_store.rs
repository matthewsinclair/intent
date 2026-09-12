//! AC-18.2, the door half: a rebuild records every in-scope path with the
//! reason the index does not hold it, and `index status` READS THAT BACK rather
//! than walking again.
//!
//! **THE TWO HALVES ARE DELIBERATELY DIFFERENT OPERATIONS AND THIS FILE IS
//! WHERE THAT IS ASSERTED.** A status that surveyed the tree would describe the
//! world rather than the index: it would report a file as held on the run
//! before anything held it, and it could never say "this store has no index",
//! which is the one thing an operator needs to be told first.
//!
//! And the ruling of 2026-09-12 gets its own arm. The index and the change
//! detector own a table each, because `replace_file_index` deletes every row
//! its own scan did not produce -- so a shared table meant the writer that ran
//! last deleted the other's rows. `a_rebuild_leaves_the_change_detectors_table
//! _alone` is what keeps that true.

mod common;

use std::process::Command;

use common::Fixture;

/// Make the fixture a real repository, because the index's scope is git's
/// answer and a tree with no repository has no ignore rules at all.
fn git_init(fx: &Fixture, gitignore: &str) {
  let ok = Command::new("git")
    .args(["init", "-q"])
    .current_dir(fx.root())
    .status()
    .expect("run git")
    .success();
  assert!(ok, "git init failed");
  std::fs::write(fx.root().join(".gitignore"), gitignore).expect("gitignore");
}

fn write(fx: &Fixture, rel: &str, bytes: &[u8]) {
  let path = fx.root().join(rel);
  std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
  std::fs::write(path, bytes).expect("write");
}

#[test]
fn a_store_with_no_index_says_so_rather_than_describing_the_tree() {
  let fx = Fixture::new();
  git_init(&fx, "build/\n");
  write(&fx, "README.md", b"# readme\n");

  let status = fx.facade().index_status().expect("status");

  assert!(
    status.is_empty(),
    "a store nobody has built an index in holds nothing, however many files \
     are on disk -- a status that walked would say otherwise and be wrong"
  );
  assert!(status.skipped.is_empty());
}

#[test]
fn a_rebuild_records_the_scope_and_status_reads_it_back() {
  let fx = Fixture::new();
  git_init(&fx, "build/\n");
  write(&fx, "README.md", b"# readme\n");
  write(&fx, "src/lib.rs", b"fn main() {}\n");
  write(&fx, "assets/logo.bin", b"\x00\x01binary");
  write(&fx, "build/out.o", b"ignored\n");

  let mut facade = fx.facade();
  let built = facade.index_rebuild().expect("rebuild");

  let rows = facade.store().index_files().expect("rows");
  let row = |rel: &str| {
    rows
      .iter()
      .find(|r| r.path == rel)
      .unwrap_or_else(|| panic!("no row for `{rel}`; rows: {rows:?}"))
  };
  assert_eq!(row("README.md").corpus, "prose");
  assert_eq!(row("src/lib.rs").corpus, "code");
  assert_eq!(row("src/lib.rs").lang.as_deref(), Some("rust"));
  assert_eq!(
    row("assets/logo.bin").skipped_reason.as_deref(),
    Some("binary")
  );
  assert!(
    !rows.iter().any(|r| r.path.starts_with("build/")),
    "an ignored path gets no row at all: it is not in the index's scope"
  );
  assert!(built.held.get("prose").copied().unwrap_or_default() >= 1);
  assert!(built.held.get("code").copied().unwrap_or_default() >= 1);
  assert_eq!(
    built.skipped.get("binary"),
    Some(&vec!["assets/logo.bin".to_string()]),
    "a file the index does not hold is named, with the reason"
  );
  assert!(
    !built
      .skipped
      .values()
      .flatten()
      .any(|p| p.starts_with("build/")),
    "an ignored path is out of scope, so it is not skipped -- it is not there"
  );

  assert_eq!(
    facade.index_status().expect("status"),
    built,
    "status reads the rows the rebuild wrote; if the two can differ, one of \
     them is not answering about the index"
  );
}

#[test]
fn a_path_that_leaves_the_scope_leaves_the_index() {
  let fx = Fixture::new();
  git_init(&fx, "");
  write(&fx, "README.md", b"# readme\n");
  write(&fx, "docs/old.md", b"# old\n");

  let mut facade = fx.facade();
  facade.index_rebuild().expect("first rebuild");
  let held = |f: &intentsvcs::facade::Facade| -> Vec<String> {
    f.store()
      .index_files()
      .expect("rows")
      .into_iter()
      .map(|r| r.path)
      .collect()
  };
  assert!(held(&facade).contains(&"docs/old.md".to_string()));

  std::fs::remove_file(fx.root().join("docs/old.md")).expect("remove");
  facade.index_rebuild().expect("second rebuild");

  let after = held(&facade);
  assert!(
    !after.contains(&"docs/old.md".to_string()),
    "a row the survey did not produce is a path that has left the scope, and \
     this table has one writer, so nothing else has to be consulted to know it"
  );
  assert!(
    after.contains(&"README.md".to_string()),
    "and the rows that are still in scope are still there, or the delete would \
     be a wipe with extra steps"
  );
}

#[test]
fn a_rebuild_leaves_the_change_detectors_table_alone() {
  // **THE RULING, AS AN ARM.** A shared table meant the writer that ran last
  // deleted the other's rows -- the index's corpus and the canon corpus are
  // not nested in either direction, so neither could be given the other's
  // delete rule.
  let fx = Fixture::new();
  git_init(&fx, "");
  write(&fx, "README.md", b"# readme\n");
  write(&fx, "src/lib.rs", b"fn main() {}\n");

  let mut facade = fx.facade();
  let before = facade.store().file_index().expect("file index");
  facade.index_rebuild().expect("rebuild");
  let after = facade.store().file_index().expect("file index");

  assert_eq!(
    before, after,
    "a rebuild of the index must not touch the change detector's rows"
  );
  assert!(
    !facade.index_status().expect("status").held.is_empty(),
    "precondition: the rebuild must have written SOMETHING, or this arm passes \
     against an operation that did nothing at all"
  );
}

#[test]
fn a_writer_that_does_not_know_what_was_indexed_does_not_say_it_was_nothing() {
  // **A SURVEY STATS AND DOES NOT READ**, so it arrives with `indexed_sha256`
  // unset for every row. Taking that literally would erase, on every reconcile,
  // the record of what the index actually holds -- leaving a column that says
  // "nothing is indexed here" for a file whose content is indexed.
  //
  // Driven at the store, because the erasure would be the store's: the survey
  // is right to arrive with `None`, and NULL from a writer means "I do not
  // know", not "nothing".
  use intentsvcs::index::Row;
  use intentsvcs::store::Store;

  let dir = tempfile::tempdir().expect("tempdir");
  let mut store = Store::open(&dir.path().join("intent.db")).expect("store");

  let row = |sha: Option<&str>| Row {
    path: "README.md".to_string(),
    corpus: "prose".to_string(),
    lang: None,
    size: 9,
    mtime: "2026-09-12T10:00:00Z".to_string(),
    indexed_sha256: sha.map(str::to_string),
    skipped_reason: None,
  };

  store
    .replace_index_files(&[row(None)])
    .expect("first write");
  store
    .replace_index_files(&[row(Some("deadbeef"))])
    .expect("what the content indexer read");
  store
    .replace_index_files(&[row(None)])
    .expect("a later survey, which read nothing");

  assert_eq!(
    store.index_files().expect("rows")[0]
      .indexed_sha256
      .as_deref(),
    Some("deadbeef"),
    "the record of what the index holds survives a writer that has no opinion \
     about it"
  );
}
