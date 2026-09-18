//! ST0078 AT-03.2 (AC-03.2), and issue 0455: **what `intent doctor` says
//! about a store that lags its canon, and about one it cannot read, on a
//! DEFAULT run.**
//!
//! Both are driven through the real verb, because the CLI hands the library a
//! different shape from the one a facade arm passes: `intent doctor` gets its
//! store through `engine(..).ok()`, so a store that will not open reaches it as
//! NO store (0447's lesson).
//!
//! **STALE: SHOWN, NOT COUNTED.** Measured 2026-09-18 (ST0078 `design.md`, E3):
//! after a pull, a default `doctor` printed `0 finding(s)` while `st show` of
//! the pulled thread refused. The check had fired, but only `--verbose` showed
//! it.
//!
//! **UNREADABLE: COUNTED.** Driven 2026-09-18 with a store file that is not a
//! database: `st list`, `index rebuild` and `sync --to-store` refused at rc 1,
//! each remedy said to run `intent doctor`, and `doctor` printed `0 finding(s)`
//! at rc 0.

use std::path::Path;
use std::process::Command;

use crate::common::short_dir;

fn doctor(root: &Path, home: &Path, args: &[&str]) -> (String, i32) {
  let mut argv = vec!["doctor"];
  argv.extend_from_slice(args);
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(&argv)
    .current_dir(root)
    .env("HOME", home)
    .env_remove("XDG_CONFIG_HOME")
    .env_remove("XDG_DATA_HOME")
    .env_remove("XDG_STATE_HOME")
    .env_remove("XDG_RUNTIME_DIR")
    .stdin(testkit::lifeline_for(&argv))
    .output()
    .expect("the intent binary runs");
  (
    format!(
      "{}{}",
      String::from_utf8_lossy(&out.stdout),
      String::from_utf8_lossy(&out.stderr)
    ),
    out.status.code().unwrap_or(-1),
  )
}

/// A project whose store holds ST0001 and has written its canon file.
fn project(tag: &str) -> (std::path::PathBuf, std::path::PathBuf) {
  let dir = short_dir(tag);
  let root = dir.join("p");
  let home = dir.join("home");
  std::fs::create_dir_all(&home).expect("the isolated HOME");
  intentsvcs::init::init(&root, "Stale", "test", env!("CARGO_PKG_VERSION"))
    .expect("the shipped initialiser creates a project");
  let opened = intentsvcs::project::Project::open(&root).expect("the project just created");
  let ctx = intentsvcs::facade::FacadeContext {
    principal: "test".to_string(),
    project_id: opened.config().project_id.clone().unwrap_or_default(),
    version: env!("CARGO_PKG_VERSION").to_string(),
  };
  let mut facade = intentsvcs::facade::Facade::open(opened, ctx).expect("open the new project");
  facade.st_new("a thread in the store").expect("mint one");
  (root, home)
}

/// The store lags its canon, which is what a pull leaves behind: the canon and
/// the views agree with each other, and the store holds the model from before.
///
/// **MADE ON THE STORE, NOT ON THE FILES.** Rewriting the canon alone also
/// leaves its views stale, which a real pull never does, and doctor then
/// counts view skew, which is not this test's subject.
fn store_lags_its_canon(root: &Path) {
  let project = intentsvcs::project::Project::open(root).expect("the project");
  let mut store = intentsvcs::store::Store::open(&project.db_path()).expect("the store opens");
  let (mut threads, issues) = store.load_canon().expect("the store reads back");
  threads[0].title = "the title from before the pull".to_string();
  store
    .rebuild(&threads, &issues)
    .expect("the store takes the older model");
}

#[test]
fn a_stale_store_is_shown_on_a_default_run_and_not_counted() {
  let (root, home) = project("stale");
  store_lags_its_canon(&root);

  let (said, code) = doctor(&root, &home, &[]);
  assert!(
    said.contains("advisory: store-stale -- 1 finding, not counted in the verdict"),
    "a default run shows the store-stale line: {said}"
  );
  assert!(
    said.contains("intent sync --apply"),
    "and its remedy names the pass that clears it: {said}"
  );
  assert!(
    said.contains("ANOTHER node's canon write"),
    "and keeps 0313's caveat, that it fires during a peer's write: {said}"
  );
  assert!(said.contains("doctor: 0 finding(s)"), "NOT counted: {said}");
  assert_eq!(code, 0, "and the exit code is untouched: {said}");

  let (quiet, _) = doctor(&root, &home, &["--quiet"]);
  assert!(
    !quiet.contains("store-stale"),
    "`--quiet` drops what does not move the exit code, this included: {quiet}"
  );
}

#[test]
fn a_store_in_step_with_its_canon_shows_no_such_line() {
  let (root, home) = project("fresh");
  let (said, code) = doctor(&root, &home, &[]);
  assert!(
    !said.contains("store-stale"),
    "a store in step with canon reported stale, so the line is noise: {said}"
  );
  assert_eq!(code, 0, "{said}");
}

#[test]
fn a_store_that_will_not_open_is_a_counted_finding() {
  let (root, home) = project("unreadable");
  let db = root.join("intent/.cache/intent.db");
  std::fs::write(&db, "this is not a sqlite database").expect("damage the store");
  let _ = std::fs::remove_file(root.join("intent/.cache/intent.db-wal"));
  let _ = std::fs::remove_file(root.join("intent/.cache/intent.db-shm"));

  let (said, code) = doctor(&root, &home, &[]);
  assert!(
    said.contains("residue: store-unreadable -- 1 finding"),
    "every other verb refuses on this store and sends the reader here: {said}"
  );
  assert!(
    said.contains("file is not a database") && said.contains("canon is intact"),
    "it names the cause and says the canon is not at fault: {said}"
  );
  assert!(
    said.contains("no snapshot of this store"),
    "and says there is no snapshot rather than naming none: {said}"
  );
  assert_eq!(code, 1, "COUNTED, so the commit gate sees it: {said}");
  assert_eq!(
    std::fs::read(&db).expect("the store file"),
    b"this is not a sqlite database",
    "doctor left the damaged store exactly as it found it"
  );
}
