//! ST0069 **AT-01.2 and AT-01.3** -- covering **AC-01.2** (the sigil, and
//! `organize` hydrating and dehydrating an issue by name) and **AC-01.3** (the
//! default declaration, and the lifecycle verbs that maintain it).
//!
//! # The red arm, and what it is guarding
//!
//! **`ISSUE:` was retired from the grammar on 2026-08-20 because a declared
//! issue had nothing to realise**: `intent issues hydrate 0001` wrote
//! `ISSUE:0001` into a live manifest and reported `ok` over ZERO files. The
//! sigil is back because an issue now has a realised form, so the arm that
//! matters is the one that would have caught the original defect --
//! `a_declared_issue_with_no_file_is_hydrated_and_never_ok_over_zero_files`.
//!
//! **It was driven RED before the sigil went back in.** On the parent commit
//! the whole file fails at its fixture, because `issues add` does not declare
//! and `intent/issues/` does not exist; the arm's own assertion -- a declared
//! issue previewing as `1 to hydrate` -- is what turns green when the feature
//! lands, and a preview of `0 to hydrate` is the exact shape of the 2026-08-20
//! report.
//!
//! # Why `--default` is tested against an ABSENT manifest
//!
//! `intent init` writes a manifest, and `--default` over a file that already
//! exists changes nothing without `--force`, which refuses off a terminal.
//! That is AC-11.1's behaviour and not this criterion's business, so the arm
//! removes the file first -- the same move `organize_default_declaration.rs`
//! makes, for the same reason.

use std::process::{Command, Output};

fn intent(dir: &std::path::Path, args: &[&str]) -> Output {
  Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(dir)
    .stdin(std::process::Stdio::null())
    .output()
    .expect("run the v3 binary")
}

fn said(out: &Output) -> String {
  format!(
    "{}{}",
    String::from_utf8_lossy(&out.stdout),
    String::from_utf8_lossy(&out.stderr)
  )
}

fn manifest(root: &std::path::Path) -> String {
  std::fs::read_to_string(root.join("intent/.intentfiles")).expect("manifest")
}

/// Only the declaring lines, in file order.
fn declared(root: &std::path::Path) -> Vec<String> {
  manifest(root)
    .lines()
    .map(str::trim)
    .filter(|l| l.starts_with("STEELTHREAD:") || l.starts_with("ISSUE:"))
    .map(str::to_string)
    .collect()
}

/// The PREVIEW verdict, by prefix rather than line number.
fn preview_line(out: &Output) -> String {
  let text = said(out);
  text
    .lines()
    .find(|l| l.starts_with("organize (preview):"))
    .unwrap_or_else(|| panic!("no organize preview summary in:\n{text}"))
    .to_string()
}

/// The APPLY verdict.
///
/// **ITS OWN HELPER BECAUSE AN APPLYING RUN PRINTS BOTH LINES**, preview first
/// -- so a single finder taking whichever came first read the PREVIEW out of an
/// apply's output and asserted about work that had not happened yet. It failed
/// loudly here; the direction that would not have is an assertion on
/// `0 hydrated` passing because it matched `0 to hydrate`.
fn applied_line(out: &Output) -> String {
  let text = said(out);
  text
    .lines()
    .find(|l| l.starts_with("organize:") && !l.contains("preview only"))
    .unwrap_or_else(|| panic!("no organize apply summary in:\n{text}"))
    .to_string()
}

fn project() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  assert!(
    intent(root, &["init", "Fixture"]).status.success(),
    "the fixture must initialise"
  );
  dir
}

fn view(root: &std::path::Path, n: u32) -> std::path::PathBuf {
  root.join(format!("intent/issues/{n:04}.md"))
}

// ---------------------------------------------------------------------------
// AC-01.2 -- THE SIGIL, AND THE 2026-08-20 DEFECT AS THE RED ARM
// ---------------------------------------------------------------------------

#[test]
fn the_manifest_accepts_the_issue_sigil() {
  let dir = project();
  let root = dir.path();
  assert!(
    intent(root, &["issues", "add", "A defect"])
      .status
      .success()
  );

  // Hand-written, because the criterion is about what the GRAMMAR accepts and
  // a manifest this test wrote through a verb would only prove the verb agrees
  // with itself.
  std::fs::write(
    root.join("intent/.intentfiles"),
    "# hand-written\nISSUE:0001\n",
  )
  .expect("write a manifest by hand");

  let out = intent(root, &["organize"]);
  assert!(
    out.status.success(),
    "`ISSUE:0001` must parse, not abort the run: {}",
    said(&out)
  );
  assert!(
    !said(&out).contains("not a known sigil"),
    "the sigil must be known: {}",
    said(&out)
  );
}

#[test]
fn a_declared_issue_with_no_file_is_hydrated_and_never_ok_over_zero_files() {
  let dir = project();
  let root = dir.path();
  assert!(
    intent(root, &["issues", "add", "A defect"])
      .status
      .success()
  );

  // The projection realises on write, so remove the file to reach the state the
  // criterion is about: DECLARED, and absent.
  std::fs::remove_file(view(root, 1)).expect("start from declared-and-absent");
  assert!(
    declared(root).contains(&"ISSUE:0001".to_string()),
    "the fixture must leave the issue declared: {:?}",
    declared(root)
  );

  let preview = preview_line(&intent(root, &["organize"]));
  assert!(
    preview.contains("1 to hydrate"),
    "a declared issue with no file is ONE to hydrate -- `0 to hydrate` here is \
     the 2026-08-20 defect, `ok` over zero files: {preview}"
  );

  let out = intent(root, &["organize", "--apply"]);
  assert!(out.status.success(), "apply: {}", said(&out));
  assert!(
    view(root, 1).is_file(),
    "apply must put the declared issue's view on disk"
  );
  assert!(
    applied_line(&out).contains("1 hydrated"),
    "and must say it did, by name rather than as a silent success: {}",
    applied_line(&out)
  );
}

#[test]
fn a_realised_file_whose_issue_is_undeclared_is_dehydrated() {
  let dir = project();
  let root = dir.path();
  assert!(
    intent(root, &["issues", "add", "A defect"])
      .status
      .success()
  );
  assert!(view(root, 1).is_file(), "realised by the write");

  // Undeclare it by hand rather than by closing: this arm is about what the
  // MANIFEST says, and closing would also change the record, so a green here
  // would not say which of the two drove the removal.
  std::fs::write(root.join("intent/.intentfiles"), "# nothing declared\n")
    .expect("undeclare by hand");

  let preview = preview_line(&intent(root, &["organize"]));
  assert!(
    preview.contains("1 to remove"),
    "an undeclared issue's realised view is ONE to remove: {preview}"
  );
}

// ---------------------------------------------------------------------------
// AC-01.3 -- THE DEFAULT DECLARATION AND THE LIFECYCLE VERBS
// ---------------------------------------------------------------------------

#[test]
fn default_declares_exactly_the_wip_threads_and_the_open_issues() {
  let dir = project();
  let root = dir.path();
  assert!(
    intent(root, &["st", "new", "A thread at triage"])
      .status
      .success()
  );
  assert!(
    intent(root, &["st", "new", "A thread to start"])
      .status
      .success()
  );
  assert!(intent(root, &["st", "start", "ST0002"]).status.success());
  assert!(
    intent(root, &["issues", "add", "An open defect"])
      .status
      .success()
  );
  assert!(
    intent(root, &["issues", "add", "A closed defect"])
      .status
      .success()
  );
  assert!(intent(root, &["issues", "close", "0002"]).status.success());

  std::fs::remove_file(root.join("intent/.intentfiles")).expect("start from absent");
  let out = intent(root, &["organize", "--default"]);
  assert!(out.status.success(), "--default: {}", said(&out));

  // **ST0001 IS THE DISCRIMINATOR ON THE THREAD SIDE AND 0002 ON THE ISSUE
  // SIDE.** A fixture of one WIP thread and one open issue cannot tell this
  // rule from one that declares everything.
  assert_eq!(
    declared(root),
    vec!["STEELTHREAD:ST0002".to_string(), "ISSUE:0001".to_string()],
    "exactly the WIP thread and the OPEN issue: ST0001 is triage and 0002 is closed"
  );
}

#[test]
fn adding_an_issue_declares_it_and_closing_it_undeclares_and_dehydrates() {
  let dir = project();
  let root = dir.path();

  assert!(
    intent(root, &["issues", "add", "A defect"])
      .status
      .success()
  );
  assert!(
    declared(root).contains(&"ISSUE:0001".to_string()),
    "`issues add` adds its id: {:?}",
    declared(root)
  );

  assert!(intent(root, &["issues", "close", "0001"]).status.success());
  assert!(
    !declared(root).contains(&"ISSUE:0001".to_string()),
    "closing removes it: {:?}",
    declared(root)
  );

  let out = intent(root, &["organize", "--apply"]);
  assert!(out.status.success(), "apply: {}", said(&out));
  assert!(
    !view(root, 1).is_file(),
    "a closed issue is undeclared, so its file is dehydrated on apply"
  );

  // The round trip, because a removal that cannot be undone is a different
  // feature from a declaration that follows status.
  assert!(intent(root, &["issues", "open", "0001"]).status.success());
  assert!(
    declared(root).contains(&"ISSUE:0001".to_string()),
    "reopening re-declares it: {:?}",
    declared(root)
  );
}
