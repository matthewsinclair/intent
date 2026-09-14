//! AT-18.2.
//!
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

use crate::common;

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

#[test]
fn the_cap_is_the_projects_to_set() {
  // **THE DEFAULT IS A MEASUREMENT OF ONE ESTATE**, so a project whose
  // documents are larger than this one's has to be able to disagree with it.
  // The arm drives the disagreement rather than reading the field back: a
  // config value nothing consults is a claim the tool cannot back.
  let fx = Fixture::new();
  git_init(&fx, "");
  write(&fx, "notes.md", &vec![b'x'; 4096]);

  let mut facade = fx.facade();
  facade.index_rebuild().expect("rebuild at the default cap");
  assert_eq!(
    facade
      .store()
      .index_files()
      .expect("rows")
      .iter()
      .find(|r| r.path == "notes.md")
      .and_then(|r| r.skipped_reason.clone()),
    None,
    "precondition: the default cap holds this file, or the arm below proves nothing"
  );

  std::fs::write(
    fx.root().join("intent/.config/config.json"),
    "{\n  \"intent_version\": \"3.0.0\",\n  \"project_name\": \"Fixture\",\n  \"author\": \"cc\",\n  \"intent_dir\": \"intent\",\n  \"languages\": [\"rust\"],\n  \"index\": { \"max_file_bytes\": 1024 }\n}\n",
  )
  .expect("write config");

  let mut tighter = fx.facade();
  tighter
    .index_rebuild()
    .expect("rebuild at the project's cap");
  assert_eq!(
    tighter
      .store()
      .index_files()
      .expect("rows")
      .iter()
      .find(|r| r.path == "notes.md")
      .and_then(|r| r.skipped_reason.clone())
      .as_deref(),
    Some("too-large"),
    "the cap the project set is the cap the index uses"
  );
}

#[test]
fn a_rebuild_reads_what_it_says_it_holds() {
  // **A REBUILD THAT RECORDED A CORPUS AND INDEXED NONE OF IT** would leave
  // `index status` reporting files as held while nothing could be found in
  // them, which is the worst of the three states: worse than an empty index,
  // because it claims not to be one.
  let fx = Fixture::new();
  git_init(&fx, "");
  write(
    &fx,
    "README.md",
    b"# Readme\n\nA paragraph about widgets.\n",
  );
  write(&fx, "src/lib.rs", b"fn assemble_widget() {}\n");
  write(&fx, "assets/logo.bin", b"\x00\x01binary");

  let mut facade = fx.facade();
  facade.index_rebuild().expect("rebuild");

  let prose = facade.store().doc_sections().expect("prose");
  assert!(
    prose
      .iter()
      .any(|s| s.file == "README.md" && s.body.contains("widgets")),
    "the repository's own prose joins the prose table, which is what makes one \
     query answer over canon prose and disk prose alike: {prose:?}"
  );
  assert!(
    prose.iter().all(|s| s.file != "assets/logo.bin"),
    "and a skipped file is read by nothing"
  );

  let code = facade.store().src_sections().expect("source");
  let lib = code
    .iter()
    .find(|r| r.path == "src/lib.rs")
    .expect("code joins the source table: {code:?}");
  assert!(lib.body.contains("assemble_widget"));
  assert_eq!(lib.seq, 0, "one row per file at the lexical tier");
  assert!(
    !code.iter().any(|r| r.path == "assets/logo.bin"),
    "and the skipped file is in no table"
  );

  let rows = facade.store().index_files().expect("rows");
  let row = |rel: &str| rows.iter().find(|r| r.path == rel).expect("a row");
  assert!(
    row("src/lib.rs").indexed_sha256.is_some(),
    "and the row says what it was read from, which is what a later reconcile \
     compares against"
  );
  assert!(
    row("assets/logo.bin").indexed_sha256.is_none(),
    "a skipped file has no indexed content, so it has no hash to record"
  );
}

#[test]
fn the_two_prose_writers_leave_each_other_alone() {
  // The prose table is the one place two writers share a table, and they can
  // because `owner_type` says which is which. Driven at the store: the failure
  // would be one writer's delete-missing taking the other's rows, which is the
  // shape `file_index` and `index_file` had to be split to avoid.
  use intentsvcs::prose::{DocSection, FILE_OWNER};
  use intentsvcs::store::Store;

  let dir = tempfile::tempdir().expect("tempdir");
  let mut store = Store::open(&dir.path().join("intent.db")).expect("store");

  let section = |owner_type: &str, owner_id: &str, file: &str| DocSection {
    owner_type: owner_type.to_string(),
    owner_id: owner_id.to_string(),
    file: file.to_string(),
    seq: 0,
    heading: None,
    level: 0,
    body: "a body\n".to_string(),
  };

  store
    .replace_doc_sections(&[section("thread", "ST0001", "intent/st/ST0001/info.md")])
    .expect("canon");
  store
    .replace_file_sections(&[section(FILE_OWNER, "README.md", "README.md")])
    .expect("files");

  let both = store.doc_sections().expect("sections");
  assert_eq!(both.len(), 2, "both halves are in the table: {both:?}");

  store
    .replace_doc_sections(&[section("thread", "ST0002", "intent/st/ST0002/info.md")])
    .expect("canon again");
  let after = store.doc_sections().expect("sections");
  assert!(
    after.iter().any(|s| s.owner_type == FILE_OWNER),
    "a canon rebuild does not delete the index's prose: {after:?}"
  );
  assert!(
    after.iter().any(|s| s.owner_id == "ST0002") && !after.iter().any(|s| s.owner_id == "ST0001"),
    "and it still replaces its own half wholesale"
  );

  store
    .replace_file_sections(&[])
    .expect("the index, now empty");
  let last = store.doc_sections().expect("sections");
  assert!(
    last.iter().all(|s| s.owner_type != FILE_OWNER),
    "the index's half is gone"
  );
  assert!(
    last.iter().any(|s| s.owner_id == "ST0002"),
    "and canon's is untouched"
  );
}

#[test]
fn a_refresh_touches_its_subtree_and_leaves_the_rest_of_the_index_alone() {
  // **THE DOOR A WATCHER CALLS.** Reconciling one file through the rebuild
  // door would unindex the project on every keystroke, because a rebuild
  // deletes every row it was not told about.
  let fx = Fixture::new();
  git_init(&fx, "");
  write(&fx, "docs/guide.md", b"# Guide\n\nabout widgets\n");
  write(&fx, "src/lib.rs", b"fn assemble_widget() {}\n");

  let mut facade = fx.facade();
  facade.index_rebuild().expect("rebuild");

  write(&fx, "src/lib.rs", b"fn assemble_gadget() {}\n");
  let refreshed = facade
    .index_refresh(Some(&[fx.root().join("src")]))
    .expect("refresh");

  assert_eq!(refreshed.updated, vec!["src/lib.rs".to_string()]);
  assert!(refreshed.removed.is_empty());

  let code = facade.store().src_sections().expect("source");
  let lib = code.iter().find(|r| r.path == "src/lib.rs").expect("a row");
  assert!(
    lib.body.contains("assemble_gadget") && !lib.body.contains("assemble_widget"),
    "the file's content is replaced rather than added to: {lib:?}"
  );

  // **THE ROWS OUTSIDE THE SUBTREE ARE THE CLAIM**, and they are what a
  // rebuild-with-a-filter would take: `replace_index_files` deletes every row
  // it was not handed, so putting a refresh through that door unindexes the
  // project on every keystroke while every section table still looks right.
  let rows = facade.store().index_files().expect("rows");
  assert!(
    rows.iter().any(|r| r.path == "docs/guide.md"),
    "a refresh under `src/` leaves the index's rows elsewhere alone: {rows:?}"
  );

  let prose = facade.store().doc_sections().expect("prose");
  assert!(
    prose
      .iter()
      .any(|s| s.file == "docs/guide.md" && s.body.contains("widgets")),
    "and their content with them: {prose:?}"
  );
}

#[test]
fn one_batch_of_leaf_paths_is_one_refresh_that_names_them_all() {
  // Issue 0354: the daemon handed a watcher batch over a path at a time, and
  // every refresh walked the tree and re-read what the store carries again.
  let fx = Fixture::new();
  git_init(&fx, "");
  for rel in ["src/a.rs", "src/b.rs", "src/c.rs"] {
    write(&fx, rel, b"fn f() {}\n");
  }

  let mut facade = fx.facade();
  facade.index_rebuild().expect("rebuild");
  for rel in ["src/a.rs", "src/b.rs", "src/c.rs"] {
    write(&fx, rel, b"fn f_rewritten() {}\n");
  }

  let batch = ["src/a.rs", "src/b.rs", "src/c.rs"].map(|rel| fx.root().join(rel));
  let mut refreshed = facade.index_refresh(Some(&batch)).expect("refresh");
  refreshed.updated.sort();

  assert_eq!(refreshed.updated, vec!["src/a.rs", "src/b.rs", "src/c.rs"]);
}

#[test]
fn a_refresh_that_names_one_file_reads_nothing_outside_it() {
  // Issue 0355: a refresh naming one file walked and surveyed every file in
  // the repository. A directory the refresh does not name is made unreadable,
  // so a walk that still reached it fails the refresh.
  use std::os::unix::fs::PermissionsExt;
  let fx = Fixture::new();
  git_init(&fx, "");
  write(&fx, "src/lib.rs", b"fn assemble_widget() {}\n");
  write(&fx, "locked/notes.md", b"# Notes\n");

  let mut facade = fx.facade();
  facade.index_rebuild().expect("rebuild");
  write(&fx, "src/lib.rs", b"fn assemble_gadget() {}\n");

  let locked = fx.root().join("locked");
  std::fs::set_permissions(&locked, std::fs::Permissions::from_mode(0o000)).expect("chmod 000");
  let refreshed = facade.index_refresh(Some(&[fx.root().join("src/lib.rs")]));
  std::fs::set_permissions(&locked, std::fs::Permissions::from_mode(0o755)).expect("chmod 755");

  assert_eq!(
    refreshed.expect("refresh").updated,
    vec!["src/lib.rs".to_string()]
  );
}

#[test]
fn the_section_files_door_answers_one_owner_type_and_nothing_else() {
  // Issue 0354: `carried_paths` read and sorted every body to find these.
  use intentsvcs::prose::{DocSection, WB_OWNER};
  use intentsvcs::store::Store;

  let section = |owner_type: &str, owner_id: &str, file: &str, seq: u32| DocSection {
    owner_type: owner_type.to_string(),
    owner_id: owner_id.to_string(),
    file: file.to_string(),
    seq,
    heading: None,
    level: 0,
    body: "text".to_string(),
  };
  let mut store = Store::open_in_memory().expect("store");
  store
    .replace_doc_sections(&[section("thread", "ST0001", "intent/st/ST0001/design.md", 0)])
    .expect("canon sections");
  store
    .replace_wb_sections_for(
      "cc",
      &[
        section(WB_OWNER, "cc", "intent/whiteboard/cc/.history/a.md", 0),
        section(WB_OWNER, "cc", "intent/whiteboard/cc/.history/a.md", 1),
        section(WB_OWNER, "cc", "intent/whiteboard/cc/.history/b.md", 0),
      ],
    )
    .expect("whiteboard sections");

  let mut files = store.section_files(WB_OWNER).expect("files");
  files.sort();

  assert_eq!(
    files,
    vec![
      "intent/whiteboard/cc/.history/a.md",
      "intent/whiteboard/cc/.history/b.md"
    ]
  );
}

#[test]
fn a_refresh_of_an_untouched_subtree_changes_nothing_and_says_so() {
  // A watcher wakes on events it will often have nothing to do about, and a
  // pass that reported work every time would publish noise forever.
  let fx = Fixture::new();
  git_init(&fx, "");
  write(&fx, "src/lib.rs", b"fn a() {}\n");

  let mut facade = fx.facade();
  facade.index_rebuild().expect("rebuild");
  let refreshed = facade
    .index_refresh(Some(&[fx.root().join("src")]))
    .expect("refresh");

  assert!(refreshed.is_empty(), "nothing moved: {refreshed:?}");
}

#[test]
fn a_file_that_has_gone_leaves_the_index_and_takes_its_content_with_it() {
  let fx = Fixture::new();
  git_init(&fx, "");
  write(&fx, "docs/old.md", b"# Old\n\nabout widgets\n");
  write(&fx, "docs/new.md", b"# New\n\nabout gadgets\n");

  let mut facade = fx.facade();
  facade.index_rebuild().expect("rebuild");
  std::fs::remove_file(fx.root().join("docs/old.md")).expect("remove");

  let refreshed = facade
    .index_refresh(Some(&[fx.root().join("docs")]))
    .expect("refresh");
  assert_eq!(refreshed.removed, vec!["docs/old.md".to_string()]);

  let prose = facade.store().doc_sections().expect("prose");
  assert!(
    prose.iter().all(|s| s.file != "docs/old.md"),
    "content whose file has gone goes with it, or a search answers from a file \
     that is not there: {prose:?}"
  );
  assert!(
    prose.iter().any(|s| s.file == "docs/new.md"),
    "and its neighbour is untouched"
  );
}

#[test]
fn a_source_file_is_found_lexically_and_its_corpus_is_named() {
  // **CODE THAT IS INDEXED AND ONLY FINDABLE STRUCTURALLY IS A HOLE IN "SEARCH
  // THE WHOLE PROJECT"** (vc, 2026-09-12). The source table is a second corpus
  // answered by the same lexical question, so its rows join the lexical group
  // and add an ENTRY to the freshness block -- not a group of their own.
  use intentsvcs::search::{SearchQuery, Tier};

  let fx = Fixture::new();
  git_init(&fx, "");
  write(&fx, "src/lib.rs", b"fn assemble_widget() -> usize { 1 }\n");

  let mut facade = fx.facade();
  facade.index_rebuild().expect("rebuild");

  let answer = facade
    .search_all("assemble_widget", &SearchQuery::default())
    .expect("the search answered");

  let lexical = answer
    .groups
    .iter()
    .find(|g| g.tier == Tier::Lexical)
    .expect("the lexical group");
  let hit = lexical
    .hits
    .iter()
    .find(|h| h.path == "src/lib.rs")
    .unwrap_or_else(|| panic!("the source file is not in the lexical answer: {answer:?}"));
  assert!(
    hit.snippet.contains("assemble_widget"),
    "and the line that matched is shown: {hit:?}"
  );
  assert_eq!(
    hit.span.map(|s| s.start_line),
    Some(1),
    "the line survives because the indexed body is the file's bytes"
  );
  assert_eq!(hit.lang.as_deref(), Some("rust"), "read from `index_file`");

  let code = answer
    .index
    .corpora
    .get("code")
    .expect("the source corpus is named in the freshness block");
  assert_eq!(
    code.policy, "stat-then-hash",
    "and it says how its freshness is decided, which is the half a block that \
     named only the corpus would leave out"
  );
  assert!(code.files >= 1);
}

#[test]
fn a_declared_language_that_names_no_symbols_says_why() {
  // **THREE FACTS PRODUCE AN EMPTY STRUCTURAL ANSWER** -- no grammar compiled
  // in, a grammar with no tags query, a language nothing supports -- and a
  // reader told none of them concludes the index is broken or that their code
  // has no definitions in it. The fixture declares `rust`; `shell` is added
  // because its grammar ships no tags query in any build, which is the answer
  // no feature flag changes.
  let fx = Fixture::new();
  git_init(&fx, "");
  std::fs::write(
    fx.root().join("intent/.config/config.json"),
    "{\n  \"intent_version\": \"3.0.0\",\n  \"project_name\": \"Fixture\",\n  \"author\": \"cc\",\n  \"intent_dir\": \"intent\",\n  \"languages\": [\"rust\", \"shell\"]\n}\n",
  )
  .expect("write config");

  let mut facade = fx.facade();
  let built = facade.index_rebuild().expect("rebuild");

  for lang in ["rust", "shell"] {
    assert!(
      built.grammars.contains_key(lang),
      "every declared language is answered for: {built:?}"
    );
  }
  assert_ne!(
    built.grammars.get("shell").map(String::as_str),
    Some("ready"),
    "`shell`'s grammar ships no tags query in any build, so it is never ready; \
     what it must not be is silently empty"
  );
  assert_eq!(
    facade.index_status().expect("status").grammars,
    built.grammars,
    "a status read from the store and one returned by a rebuild answer the \
     same, because this is a fact about the build and not about the rows"
  );
}
