//! AT-01.1 / AC-01.1: canon for every thread and every issue resolves under
//! `intent/.canon/`, one file per artefact, and `intent/st/` holds no
//! `thread.json`. **Both populations printed; the criterion is the EQUALITY,
//! not either figure.**
//!
//! **THIS FILE IS THE LOCATION ORACLE FOR THE WHOLE ESTATE, AND THAT IS A ROLE
//! RATHER THAN A DESCRIPTION.**
//!
//! Every other test may assert where a file landed by asking
//! [`Project`] — `thread_json`, `issue_json`, `canon_thread_rel` — and that is
//! correct, because a second independent spelling of the canon path is exactly
//! what AC-01.6 forbids. **But a resolver-based assertion has no oracle of its
//! own: it compares the tool's answer to the tool's answer, and every one of
//! them would pass with canon anywhere at all.** They are only meaningful
//! because THIS file pins the resolver to a literal location that a human
//! wrote down.
//!
//! Before this file existed the estate's only literal on-disk location
//! assertions were `cli_end_to_end.rs:160` and `facade_st_wp.rs:25`, and both
//! were on the list to be repointed through the resolver. **Repointing them
//! first would have left nothing anywhere asserting where canon lives** —
//! every test still green, and the location free to be anything. vc caught
//! that; this file lands before the last literal goes.
//!
//! So: **do not "tidy" the literals below into `Project` calls.** The
//! duplication is the point. An edit that reads as cleanup would remove the
//! only independent check in the chain, which is the third time this week that
//! shape has come up.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::model::{ISSUE_SCHEMA, Issue, IssueStatus};
use intentsvcs::project::{Project, canon_issue_rel, canon_thread_rel};

/// Written out by hand, deliberately, in the one file whose job is to disagree
/// with the resolver if the resolver moves.
const CANON_DIR: &str = "intent/.canon";
const THREAD_CANON: &str = "intent/.canon/st";
const ISSUE_CANON: &str = "intent/.canon/issues";
const VIEW_DIR: &str = "intent/st";

fn issue(number: u32) -> Issue {
  serde_json::from_value(serde_json::json!({
    "schema": ISSUE_SCHEMA,
    "number": number,
    "slug": "a-defect",
    "title": "a defect",
    "status": IssueStatus::Open,
    "body": "",
    "created": "2026-08-18",
  }))
  .expect("issue")
}

/// Every file under `root`, relative and slash-joined.
fn files_under(root: &Path, rel: &str) -> BTreeSet<String> {
  let base = root.join(rel);
  let mut out = BTreeSet::new();
  let mut stack = vec![base.clone()];
  while let Some(dir) = stack.pop() {
    let Ok(entries) = std::fs::read_dir(&dir) else {
      continue;
    };
    for entry in entries.flatten() {
      let path = entry.path();
      if path.is_dir() {
        stack.push(path);
      } else if let Ok(suffix) = path.strip_prefix(root) {
        out.insert(suffix.to_string_lossy().replace('\\', "/"));
      }
    }
  }
  out
}

fn seeded() -> Fixture {
  let fx = Fixture::new();
  for id in ["ST0001", "ST0002", "ST0003"] {
    fx.write_thread(&sample_thread(id));
  }
  for n in [1, 2] {
    fx.write_issue(&issue(n));
  }
  let mut facade = fx.facade();
  facade.sync_from_disk().expect("ingest");
  facade.sync_to_disk().expect("project");
  fx
}

/// **The criterion is the EQUALITY and both populations are printed** -- a
/// canon count on its own says nothing, since 3-of-3 and 3-of-57 are the same
/// number.
#[test]
fn every_artefact_has_exactly_one_canon_file_under_the_canon_dir() {
  let fx = seeded();
  let root = fx.root();

  let threads = files_under(root, THREAD_CANON);
  let issues = files_under(root, ISSUE_CANON);

  println!(
    "canon population: {} thread file(s) under {THREAD_CANON}, {} issue file(s) under {ISSUE_CANON}",
    threads.len(),
    issues.len()
  );

  assert_eq!(
    threads,
    BTreeSet::from([
      "intent/.canon/st/ST0001.json".to_string(),
      "intent/.canon/st/ST0002.json".to_string(),
      "intent/.canon/st/ST0003.json".to_string(),
    ]),
    "one flat file per thread, named for its id"
  );
  assert_eq!(
    issues,
    BTreeSet::from([
      "intent/.canon/issues/0001.json".to_string(),
      "intent/.canon/issues/0002.json".to_string(),
    ]),
    "one flat file per issue, zero-padded"
  );
}

/// The other half of AC-01.1, and it is the half a wholesale move would fail
/// in the opposite direction.
#[test]
fn the_view_directory_holds_no_canon_and_still_holds_its_views() {
  let fx = seeded();
  let root = fx.root();
  let views = files_under(root, VIEW_DIR);

  let canon_left_behind: Vec<&String> = views
    .iter()
    .filter(|p| p.ends_with("thread.json") || p.ends_with(".json"))
    .collect();
  assert!(
    canon_left_behind.is_empty(),
    "{VIEW_DIR} holds no canon, and found: {canon_left_behind:?}"
  );

  // **THE NEGATIVE ARM (AC-01.6). Views do NOT move.** A wholesale relocation
  // of `thread_dir()` satisfies the clause above -- `intent/st/` holds no
  // `thread.json` because `intent/st/` holds nothing at all -- while emptying
  // the directory a reader browses. Only canon leaves.
  for id in ["ST0001", "ST0002", "ST0003"] {
    for name in ["info.md", "acceptance.md"] {
      let rel = format!("intent/st/{id}/{name}");
      assert!(
        root.join(&rel).is_file(),
        "the views stay where a reader browses them: {rel} is missing"
      );
    }
  }
  println!(
    "view population: {} file(s) under {VIEW_DIR}, none of them canon",
    views.len()
  );
}

/// **The literal pin.** Everything else in the estate may ask `Project` where
/// canon lives; this is the one place that says where that answer must be, in
/// bytes a human typed.
#[test]
fn the_resolver_answers_the_written_down_location() {
  let fx = Fixture::new();
  let project = fx.project();
  let root = fx.root();

  assert_eq!(project.canon_dir(), root.join(CANON_DIR));
  assert_eq!(project.canon_st_dir(), root.join(THREAD_CANON));
  assert_eq!(project.issues_dir(), root.join(ISSUE_CANON));
  assert_eq!(
    project.thread_json("ST0001"),
    root.join("intent/.canon/st/ST0001.json")
  );
  assert_eq!(
    project.issue_json(42),
    root.join("intent/.canon/issues/0042.json")
  );

  // The views, pinned in the same place and for the same reason.
  assert_eq!(project.thread_dir("ST0001"), root.join("intent/st/ST0001"));

  // The relative forms every other caller composes from.
  assert_eq!(canon_thread_rel("ST0001"), ".canon/st/ST0001.json");
  assert_eq!(canon_issue_rel(42), ".canon/issues/0042.json");
}

/// **`intent/.canon/` is the ONE deliberate exception to a convention the tree
/// otherwise keeps**, so the exception is asserted rather than remembered.
///
/// Every other `intent/.<x>/` is gitignored and local -- `.treeindex/`,
/// `.cache/`, `.backup/`. A future tidy-up adding `intent/.*/` to
/// `.gitignore` would be natural, tidy-looking, and would silently un-commit
/// the entire estate. AC-01.5's commit guard is the enforcement; this is the
/// statement of what it enforces, beside the location it applies to.
#[test]
fn the_canon_dir_is_a_dot_directory_that_must_travel() {
  let fx = Fixture::new();
  let project = fx.project();
  let name = project
    .canon_dir()
    .file_name()
    .map(|n| n.to_string_lossy().into_owned())
    .expect("canon dir has a name");
  assert_eq!(name, ".canon", "the exception is named, not inferred");

  let ignored: Vec<PathBuf> = [".treeindex", ".cache", ".backup"]
    .iter()
    .map(|d| project.intent_dir().join(d))
    .collect();
  assert!(
    !ignored.contains(&project.canon_dir()),
    "canon must never share a home with the local-only dot directories"
  );
}
