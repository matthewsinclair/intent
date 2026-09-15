//! **`intent st show <id> <file>` PRINTS THE FILE IT NAMES** (issue 0398).
//!
//! The row declares `file` as one of `info`, `design`, `impl`, `tasks`,
//! `acceptance` and `all`, default `info`, and the arm read only `id`: every
//! value printed the thread's cover at exit 0, and a thread whose attachment
//! lived only in the store had no read door through this verb at all. vc ruled
//! the shape on 2026-09-14; each arm below drives one value of it against the
//! real binary, and the last drives the store-only thread the finder met.
//!
//! **THE FIXTURE CARRIES TWO ATTACHMENTS AND OMITS ONE ON PURPOSE.** `design`
//! and `tasks` are carried and `impl` is not, so the refusal and the `all`
//! composition are measured on one thread rather than on two fixtures that
//! could disagree about what "carried" means.

use std::path::Path;
use std::process::{Command, Output, Stdio};

const DESIGN: &str = "# design\n\nthe design body\n";
const TASKS: &str = "# tasks\n\n- one\n";
const CRITERION: &str = "The verb prints the file it names";

fn intent(dir: &Path, args: &[&str]) -> Output {
  Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(dir)
    .env("HOME", testkit::fixture_home())
    .stdin(Stdio::null())
    .output()
    .expect("run the v3 binary")
}

/// Run a command that must succeed, and hand back its stdout.
fn ok(dir: &Path, args: &[&str]) -> String {
  let out = intent(dir, args);
  assert!(
    out.status.success(),
    "`intent {}` must succeed: {}",
    args.join(" "),
    String::from_utf8_lossy(&out.stderr)
  );
  String::from_utf8(out.stdout).expect("utf-8 stdout")
}

fn attach(dir: &Path, name: &str, body: &str) {
  let src = dir.join(format!("seed-{name}"));
  std::fs::write(&src, body).expect("write the seed body");
  ok(
    dir,
    &[
      "st",
      "attach",
      "ST0001",
      name,
      "--from",
      src.to_str().expect("utf-8 path"),
    ],
  );
}

/// One thread carrying `design.md` and `tasks.md`, carrying no `impl.md`, with
/// one criterion in its contract.
fn seeded() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  ok(dir.path(), &["init", "showproj"]);
  ok(dir.path(), &["st", "new", "A thread"]);
  ok(
    dir.path(),
    &["ac", "new", "--text", CRITERION, "ST0001", "AC-01.1"],
  );
  attach(dir.path(), "design.md", DESIGN);
  attach(dir.path(), "tasks.md", TASKS);
  dir
}

#[test]
fn info_prints_the_cover_the_bare_verb_prints() {
  let dir = seeded();
  let cover = ok(dir.path(), &["st", "show", "ST0001"]);
  assert!(
    cover.starts_with("ST0001: A thread\n"),
    "the declared default is the cover: {cover}"
  );
  assert_eq!(ok(dir.path(), &["st", "show", "ST0001", "info"]), cover);
}

#[test]
fn design_prints_the_attachment_the_thread_carries() {
  let dir = seeded();
  assert_eq!(ok(dir.path(), &["st", "show", "ST0001", "design"]), DESIGN);
}

#[test]
fn impl_refuses_and_names_the_door_that_attaches_one() {
  let dir = seeded();
  let out = intent(dir.path(), &["st", "show", "ST0001", "impl"]);
  let stderr = String::from_utf8_lossy(&out.stderr);
  assert_eq!(
    out.status.code(),
    Some(1),
    "a file the thread does not carry is a refusal: {stderr}"
  );
  assert!(
    out.stdout.is_empty(),
    "a refusal prints no cover in its place: {}",
    String::from_utf8_lossy(&out.stdout)
  );
  assert!(
    stderr.contains("intent st attach ST0001 impl.md"),
    "the refusal names the verb that attaches one: {stderr}"
  );
}

#[test]
fn tasks_prints_the_attachment_the_thread_carries() {
  let dir = seeded();
  assert_eq!(ok(dir.path(), &["st", "show", "ST0001", "tasks"]), TASKS);
}

/// **THE ORACLE IS THE REALISED VIEW, AND ONLY THE ORACLE READS THE DISK.** The
/// contract is rendered by the function that writes `acceptance.md`, so the
/// view's bytes are what "the rendered contract" means; realising the thread
/// gives the arm something to compare against without the read under test
/// touching it.
#[test]
fn acceptance_prints_the_rendered_contract() {
  let dir = seeded();
  ok(dir.path(), &["st", "edit", "ST0001", "design", "--path"]);
  let view = std::fs::read_to_string(dir.path().join("intent/st/ST0001/acceptance.md"))
    .expect("the realised acceptance view");
  let contract = ok(dir.path(), &["st", "show", "ST0001", "acceptance"]);
  assert!(
    contract.contains(CRITERION),
    "the criterion is in the contract: {contract}"
  );
  assert_eq!(contract, view);
}

#[test]
fn all_prints_the_cover_then_each_carried_file_under_its_name_in_declared_order() {
  let dir = seeded();
  let cover = ok(dir.path(), &["st", "show", "ST0001", "info"]);
  let contract = ok(dir.path(), &["st", "show", "ST0001", "acceptance"]);
  assert_eq!(
    ok(dir.path(), &["st", "show", "ST0001", "all"]),
    format!(
      "{cover}\n-- design.md\n{design}\n-- tasks.md\n{tasks}\n-- acceptance.md\n{contract}",
      design = DESIGN,
      tasks = TASKS
    )
  );
}

/// **THE FINDER'S THREAD: ITS FILES ARE GONE FROM DISK AND ITS RECORD IS NOT.**
/// Devbin's completed thread carried its design only in the store, and this verb
/// printed the cover for `design`. Dehydrating removes every file the thread
/// realised, so the read below has the store to answer from and nothing else.
#[test]
fn a_store_only_attachment_answers() {
  let dir = seeded();
  ok(dir.path(), &["st", "edit", "ST0001", "design", "--path"]);
  ok(dir.path(), &["st", "dehydrate", "ST0001"]);
  assert!(
    !dir.path().join("intent/st/ST0001").exists(),
    "dehydrate left the thread on disk, so this arm would not be reading the store alone"
  );
  assert_eq!(ok(dir.path(), &["st", "show", "ST0001", "design"]), DESIGN);
}
