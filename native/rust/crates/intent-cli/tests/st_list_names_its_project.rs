//! ST0056 -- `intent st list`'s first column is headed by the project's
//! DIRECTORY name, so a table pasted out of one estate says which estate it
//! came from.
//!
//! **hv's ask, 2026-09-08**: `st list --status=all` in two projects produces
//! two tables and "it isn't obvious at all which project that the output refers
//! to". The id column's heading was the one purely decorative cell in the table
//! -- every value under it already begins `ST` -- so it is where the answer
//! goes without costing a column.
//!
//! # Why the DIRECTORY and not `config.project_name`
//!
//! hv ruled it on 2026-09-03 for `explore`'s info row and the reasoning carries
//! unchanged: the configured name cannot discriminate two checkouts of one
//! project, and telling those apart is the whole question. `the_directory_wins`
//! is the arm that pins the ruling -- every other arm here would pass on a
//! build that read the configured name instead.
//!
//! **THE REAL `Intentv2` IS NO LONGER A WITNESS AND THAT IS WHY THIS IS A
//! FIXTURE.** The comment the ruling came from cites `Intentv2` declaring
//! itself `Intent`; measured 2026-09-08 it declares `Intent (v2 maintenance)`,
//! AND it is an unmigrated v2 project, so `st list` refuses there before any
//! header is rendered. A live example that has moved twice is not a control.
//!
//! # What these must SEE in order to fail
//!
//! A header test is the easiest thing in the world to write vacuously (vc,
//! 2026-09-08). An arm asserting "the header contains the project name" passes
//! on a fixture whose directory is called `ID`, and passes on a build that
//! prints the name in EVERY column. So:
//!
//!   - two fixtures with DIFFERENT directory names must produce DIFFERENT
//!     headers -- one fixture cannot tell a real answer from a constant;
//!   - the directory name must DISAGREE with the configured name, or the arm
//!     cannot tell which source was read;
//!   - the four remaining headers must be EXACTLY the declared ones, which is
//!     what refuses a build that sprays the name across the row;
//!   - and `the_fixture_can_exhibit_it` asserts the fixture's own names are
//!     distinct from each other and from `ID`, rather than trusting it.
//!
//! # Mutations, measured -- each applied to a `cp` snapshot, reverted with
//! # `cp`, verified byte-identical with `cmp`, baseline re-run green after each
//!
//! | mutation                                          | reds                                                      |
//! | ------------------------------------------------- | --------------------------------------------------------- |
//! | no name at all (the pre-change behaviour)         | 4 arms: directory_wins, first_column, slug_table, two_projects |
//! | reads `config.project_name` not the directory     | `the_directory_wins_over_the_configured_name` ONLY        |
//! | sprays the name across EVERY heading              | `no_other_column_heading_changes`, `the_slug_table_...`   |
//! | only the non-slug table gets the name             | `the_slug_table_carries_it_too`                           |
//! | `clamp_heading` never clips                       | `a_long_directory_name_is_clipped_and_the_title_survives` |
//! | `HEADING_MAX` raised to 64                        | `a_long_directory_name_is_clipped_and_the_title_survives` |
//! | an empty-name arm in `clamp_heading` is broken    | **NOTHING** -- so the arm was deleted, see below           |
//!
//! **THE LAST ROW IS A FINDING, NOT A GAP.** `clamp_heading` carried a
//! `0 => declared` branch; breaking it reddened nothing, because
//! `Path::file_name` answers `None` for a root with no final component and
//! never `Some("")`, so the branch was unreachable and the absent case is
//! already handled where `directory_name()` returns `None`. **A mutation that
//! reds nothing is either a missing test or dead code, and the way to tell is
//! to ask what input reaches it.** Here nothing did, so the branch went rather
//! than a test being written to pin an impossible state. Same class vc hit the
//! same evening from the other end: a user-facing sentence changed and 1222
//! tests stayed green, because the string was unpinned.
//!
//! **THE SECOND ROW IS WHY `the_directory_wins` EXISTS.** A build reading the
//! configured name passes every other arm in this file -- the header is still a
//! project name, still differs between two estates, still only in column one.
//! One arm separates hv's ruling from a plausible build that ignores it, and
//! the measurement says which arm that is rather than the author claiming it.
//!
//! **THE GENERATED VIEW IS DELIBERATELY NOT ASSERTED HERE.** `views.rs` renders
//! `steel_threads.md` from an identical header array and MUST keep saying `ID`;
//! changing it would diverge every committed view in the fleet at once. That is
//! guarded already, and more strongly than a unit test could: the pre-commit
//! `thread-view-skew` check compares every generated view against the model
//! (312 on this estate) and `doctor` reports skew on every project. A test
//! asserting the literal `ID` in that array would only restate the array.

use std::process::{Command, Output};

fn intent(dir: &std::path::Path, args: &[&str]) -> Output {
  Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(dir)
    .env("COLUMNS", "120")
    .stdin(std::process::Stdio::null())
    .output()
    .expect("run the v3 binary")
}

/// A project in a directory called `dir_name`, declaring itself `declared`.
fn project(dir_name: &str, declared: &str) -> (tempfile::TempDir, std::path::PathBuf) {
  let tmp = tempfile::tempdir().expect("tempdir");
  let root = tmp.path().join(dir_name);
  std::fs::create_dir_all(&root).expect("project directory");
  assert!(
    intent(&root, &["init", declared]).status.success(),
    "the fixture must initialise"
  );
  assert!(
    intent(&root, &["st", "new", "A thread"]).status.success(),
    "st new must succeed"
  );
  (tmp, root)
}

/// The table's header cells, split on the column separator.
fn headers(root: &std::path::Path, extra: &[&str]) -> Vec<String> {
  let mut args = vec!["st", "list", "--status=all"];
  args.extend_from_slice(extra);
  let out = intent(root, &args);
  let said = String::from_utf8_lossy(&out.stdout).into_owned();
  let first = said
    .lines()
    .next()
    .unwrap_or_else(|| panic!("no output from st list:\n{said}"));
  first.split('|').map(|c| c.trim().to_string()).collect()
}

/// **THE POSITIVE CONTROL.** The three names this file discriminates between
/// must actually differ, or every arm below is satisfied by a constant.
#[test]
fn the_fixture_can_exhibit_it() {
  let (_t, root) = project("Checkout-B", "Shared Name");
  let dir = "Checkout-B";
  assert_ne!(
    dir, "ID",
    "a fixture directory called ID makes every arm vacuous"
  );
  assert_ne!(
    dir, "Shared Name",
    "the directory and the configured name must DISAGREE, or `the_directory_wins` \
     cannot tell which one the build read"
  );
  let got = headers(&root, &[]);
  assert_eq!(
    got.len(),
    5,
    "the table must still have five columns: {got:?}"
  );
}

#[test]
fn the_first_column_is_headed_by_the_project_directory() {
  let (_t, root) = project("Alpha", "Alpha");
  assert_eq!(headers(&root, &[])[0], "Alpha");
}

#[test]
fn two_projects_produce_two_different_headers() {
  let (_a, alpha) = project("Alpha", "Alpha");
  let (_b, beta) = project("Beta", "Beta");
  let (a, b) = (headers(&alpha, &[]), headers(&beta, &[]));
  assert_eq!((a[0].as_str(), b[0].as_str()), ("Alpha", "Beta"));
  assert_ne!(
    a[0], b[0],
    "**hv's ACTUAL QUESTION**: two estates must not produce the same heading, \
     or the table still does not say which one it came from"
  );
}

/// **THE ARM THAT PINS hv's RULING RATHER THAN RESTATING IT.**
#[test]
fn the_directory_wins_over_the_configured_name() {
  let (_t, root) = project("Checkout-B", "Shared Name");
  let got = headers(&root, &[]);
  assert_eq!(
    got[0], "Checkout-B",
    "the DIRECTORY name, not `config.project_name` (hv, 2026-09-03): a configured \
     name cannot discriminate two checkouts of one project, which is the question \
     this heading exists to answer. Got: {got:?}"
  );
  assert_ne!(got[0], "Shared Name");
}

/// **THE HEADING IS BOUNDED, AND THIS ARM PINS THE REASON RATHER THAN THE
/// NUMBER.** The first column's values are always six characters, so every
/// character of heading beyond that is width the column does not use --
/// `views::table` floors each column at its header and takes the overflow off
/// the widest column, which is `Title`. Measured while building this: a
/// 35-character directory clipped a 34-character thread title off the screen
/// entirely, and four existing tests in two files caught it by looking for a
/// title that was no longer there.
///
/// **SO THE ARM ASSERTS BOTH HALVES**: the heading is clipped, AND the title it
/// was stealing from survives. Asserting only the clip would pass on a build
/// that clipped the heading and still ate the title.
#[test]
fn a_long_directory_name_is_clipped_and_the_title_survives() {
  let long = "a-very-long-checkout-directory-name";
  assert!(
    long.chars().count() > 16,
    "the fixture must exceed the bound"
  );
  let (_t, root) = project(long, "Fixture");
  let title = "a thread title long enough to be worth protecting";
  assert!(
    intent(&root, &["st", "new", title]).status.success(),
    "the second thread must be created"
  );
  let got = headers(&root, &[]);
  assert_eq!(
    got[0].chars().count(),
    16,
    "the heading is clipped to the bound: {got:?}"
  );
  assert!(
    long.starts_with(&got[0]),
    "and it is the START of the name, so it still discriminates: {got:?}"
  );
  let said =
    String::from_utf8_lossy(&intent(&root, &["st", "list", "--status=all"]).stdout).into_owned();
  assert!(
    said.contains(title),
    "**THE HALF THE BOUND EXISTS FOR**: the title must survive a long project \
     directory. Got:\n{said}"
  );
}

#[test]
fn the_slug_table_carries_it_too() {
  let (_t, root) = project("Alpha", "Alpha");
  let got = headers(&root, &["--slug"]);
  assert_eq!(
    got[0], "Alpha",
    "`--slug` is the same table with a different descriptive column; leaving it \
     on `ID` would give one question two answers depending on an unrelated flag"
  );
  assert_eq!(got[1], "Slug", "and it is still the slug table: {got:?}");
}

/// **ONLY THE FIRST COLUMN MOVES.** This is what refuses a build that prints
/// the project name in every heading -- which would satisfy every other arm.
#[test]
fn no_other_column_heading_changes() {
  let (_t, root) = project("Alpha", "Alpha");
  assert_eq!(
    &headers(&root, &[])[1..],
    &["Title", "Status", "Created", "Completed"]
  );
  assert_eq!(
    &headers(&root, &["--slug"])[1..],
    &["Slug", "Status", "Created", "Completed"]
  );
}
