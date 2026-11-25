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
//!
//! # Two formats (ST0096 WP-01)
//!
//! A project `intent upgrade` has moved keeps the declaration on each record,
//! and `.intentfiles` is gone. The arms about what `issues add` and `close`
//! declare run there, against the record. The arms about what the MANIFEST
//! says run on a project in 3.4.0's format ([`ordinal_project`]), where it is
//! still the declaration; a mint refuses there, so its records are seeded.

use std::process::Output;

fn intent(dir: &std::path::Path, args: &[&str]) -> Output {
  crate::common::intent()
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
  std::fs::read_to_string(crate::common::manifest_file(root)).expect("manifest")
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

/// A project `init` made, put back in 3.4.0's format, holding one open issue
/// and no manifest (nobody has said). Returned with the issue's id.
fn ordinal_project() -> (tempfile::TempDir, String) {
  let dir = project();
  let root = dir.path();
  crate::common::born_ordinal(root);
  crate::common::seed_ordinal_issue(root, 1, "A defect");
  let issue = intentsvcs::id::IssueId::from_ordinal(1)
    .expect("an ordinal issue")
    .stored();
  (dir, issue)
}

/// Seed thread ordinal `n` in [`ordinal_project`]'s format, not started.
fn seed_thread(root: &std::path::Path, n: u32, title: &str) -> String {
  let st = intentsvcs::id::ThreadId::from_ordinal(n)
    .expect("an ordinal thread")
    .stored();
  crate::common::seed_ordinal_thread(root, &st, title, &[]);
  st
}

/// The realised view of the issue whose id is `issue`.
fn view(root: &std::path::Path, issue: &str) -> std::path::PathBuf {
  intentsvcs::project::Project::open(root)
    .expect("the project opens")
    .issue_view(intentsvcs::id::IssueId::parse(issue).expect("a stored issue id"))
}

// ---------------------------------------------------------------------------
// AC-01.2 -- THE SIGIL, AND THE 2026-08-20 DEFECT AS THE RED ARM
// ---------------------------------------------------------------------------

#[test]
fn the_manifest_accepts_the_issue_sigil() {
  let (dir, issue) = ordinal_project();
  let root = dir.path();

  // Hand-written, because the criterion is about what the GRAMMAR accepts and
  // a manifest this test wrote through a verb would only prove the verb agrees
  // with itself.
  std::fs::write(
    crate::common::manifest_file(root),
    format!("# hand-written\nISSUE:{issue}\n"),
  )
  .expect("write a manifest by hand");

  let out = intent(root, &["organize"]);
  assert!(
    out.status.success(),
    "`ISSUE:{issue}` must parse, not abort the run: {}",
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
  let out = intent(root, &["issues", "add", "A defect"]);
  assert!(out.status.success(), "issues add: {}", said(&out));
  let issue = crate::common::created(&String::from_utf8_lossy(&out.stdout));

  // The projection realises on write, so remove the file to reach the state the
  // criterion is about: DECLARED, and absent.
  std::fs::remove_file(view(root, &issue)).expect("start from declared-and-absent");
  assert!(
    crate::common::realised_issue(root, &issue),
    "the fixture must leave the issue declared on its record"
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
    view(root, &issue).is_file(),
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
  let (dir, issue) = ordinal_project();
  let root = dir.path();
  // No manifest: nobody has said, so everything is realised, and the apply
  // puts the issue's view on disk.
  let out = intent(root, &["organize", "--apply"]);
  assert!(out.status.success(), "apply: {}", said(&out));
  assert!(view(root, &issue).is_file(), "realised by the apply");

  // Undeclare it by hand rather than by closing: this arm is about what the
  // MANIFEST says, and closing would also change the record, so a green here
  // would not say which of the two drove the removal.
  std::fs::write(crate::common::manifest_file(root), "# nothing declared\n")
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
  // 3.4.0's format, where `--default` writes `.intentfiles`, and no manifest
  // there, so it is written rather than left alone.
  let (dir, open) = ordinal_project();
  let root = dir.path();
  let at_triage = seed_thread(root, 1, "A thread at triage");
  // At `triage`, where `st new` leaves a thread.
  let canon = crate::common::thread_canon_file(root, &at_triage);
  let text = std::fs::read_to_string(&canon).expect("the seeded canon");
  let triage = text.replace("\"status\": \"not-started\"", "\"status\": \"triage\"");
  assert_ne!(text, triage, "the seeded thread must move to triage");
  std::fs::write(&canon, triage).expect("write the seeded canon");
  let started = seed_thread(root, 2, "A thread to start");
  // Seeded before the first verb opens a store, which ingests the tree once.
  crate::common::seed_ordinal_issue(root, 2, "A closed defect");
  let closed = intentsvcs::id::IssueId::from_ordinal(2)
    .expect("an ordinal issue")
    .stored();
  assert!(
    intent(root, &["st", "start", started.as_str()])
      .status
      .success()
  );
  assert!(
    intent(root, &["issues", "close", closed.as_str()])
      .status
      .success()
  );

  assert!(
    !crate::common::manifest_file(root).exists(),
    "start from absent"
  );
  let out = intent(root, &["organize", "--default"]);
  assert!(out.status.success(), "--default: {}", said(&out));

  // **THE TRIAGE THREAD IS THE DISCRIMINATOR ON THE THREAD SIDE AND THE CLOSED
  // ISSUE ON THE ISSUE SIDE.** A fixture of one WIP thread and one open issue
  // cannot tell this rule from one that declares everything.
  assert_eq!(
    declared(root),
    vec![format!("STEELTHREAD:{started}"), format!("ISSUE:{open}")],
    "exactly the WIP thread and the OPEN issue: {at_triage} is triage and {closed} is closed"
  );
}

#[test]
fn adding_an_issue_declares_it_and_closing_it_undeclares_and_dehydrates() {
  let dir = project();
  let root = dir.path();

  let out = intent(root, &["issues", "add", "A defect"]);
  assert!(out.status.success(), "issues add: {}", said(&out));
  let issue = crate::common::created(&String::from_utf8_lossy(&out.stdout));
  assert!(
    crate::common::realised_issue(root, &issue),
    "`issues add` declares it on its record"
  );

  assert!(
    intent(root, &["issues", "close", issue.as_str()])
      .status
      .success()
  );
  assert!(
    !crate::common::realised_issue(root, &issue),
    "closing takes the declaration off its record"
  );

  let out = intent(root, &["organize", "--apply"]);
  assert!(out.status.success(), "apply: {}", said(&out));
  assert!(
    !view(root, &issue).is_file(),
    "a closed issue is undeclared, so its file is dehydrated on apply"
  );

  // The round trip, because a removal that cannot be undone is a different
  // feature from a declaration that follows status.
  assert!(
    intent(root, &["issues", "open", issue.as_str()])
      .status
      .success()
  );
  assert!(
    crate::common::realised_issue(root, &issue),
    "reopening re-declares it on its record"
  );
}

// ---------------------------------------------------------------------------
// AC-01.2 -- THE PROJECTION OBEYS THE MANIFEST, NOT JUST THE PLAN
// ---------------------------------------------------------------------------

/// **THE LIVE ESTATE'S POPULATION, WHICH NO FIXTURE ABOVE CONTAINS.**
///
/// Every arm above reaches an issue through `issues add`, which DECLARES it --
/// so all of them drive a declared issue and none can see what happens to an
/// undeclared one when something writes. The estate's own issues all predate
/// the sigil and none is declared, and on the day WP-01 landed a single
/// `wp done` materialised a view for every one of them: the projection's
/// skip-an-undeclared-view test reads `owning_thread`, which answers `None`
/// for `intent/issues/<nnnn>.md`, and `None` made the whole `&&` false.
///
/// **The projection and `organize`'s plan then disagreed about the same file**
/// -- one writing it, the other listing it to remove -- which is the state this
/// arm exists to keep out.
#[test]
fn an_undeclared_issue_gets_no_view_when_something_writes() {
  let (dir, issue) = ordinal_project();
  let root = dir.path();
  let st = seed_thread(root, 1, "An unrelated thread");

  // Undeclared and with no view: the estate's shape, reached deliberately
  // rather than by closing, so the record still says OPEN and only the
  // MANIFEST says undeclared.
  std::fs::write(crate::common::manifest_file(root), "# nothing declared\n")
    .expect("undeclare by hand");
  assert!(!view(root, &issue).is_file(), "start from absent");

  // Any write at all runs the projection. `st start` is deliberately about a
  // DIFFERENT entity: the issue is untouched, so nothing about this command
  // asks for its view.
  let out = intent(root, &["st", "start", st.as_str()]);
  assert!(out.status.success(), "st start: {}", said(&out));

  assert!(
    !view(root, &issue).is_file(),
    "an undeclared issue's view must not be written by the projection -- the manifest \
     is what decides which artefacts are on disk, and `organize` would remove this file \
     immediately after the projection wrote it"
  );
}
