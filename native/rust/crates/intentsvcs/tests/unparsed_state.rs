//! AT-03.5 / AC-03.5: a conflict-markered artefact enters the named unparsed
//! state, commands needing it refuse with the finding, and v2's silent
//! grep-through is unconstructible.
//!
//! The third clause is the one that needs a test with teeth. It is easy to
//! record a state and still let a caller proceed; what AC-03.5 asks is that
//! the caller CANNOT proceed. So the assertions are about the refusal reaching
//! the caller, not merely about a row in a table.

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::finding::FindingClass;
use intentsvcs::ingest::{self, IngestError};
use intentsvcs::store::Store;
use intentsvcs::sync::FileState;

const CONFLICTED: &str = "\
# Work In Progress

<<<<<<< HEAD
ours
=======
theirs
>>>>>>> feature-branch
";

fn findings(err: IngestError) -> Vec<intentsvcs::finding::Finding> {
  match err {
    IngestError::Refused(r) => r.findings,
    other => panic!("expected a refusal, got: {other}"),
  }
}

#[test]
fn a_conflict_markered_file_enters_the_unparsed_state() {
  let fx = Fixture::new();
  fx.write_file("intent/wip.md", CONFLICTED);

  let entries = intentsvcs::sync::scan(fx.root(), &[]).expect("scan");
  let entry = entries
    .iter()
    .find(|e| e.path == "intent/wip.md")
    .expect("indexed");
  assert_eq!(entry.state, FileState::Unparsed);
  assert_eq!(entry.findings.len(), 1);
  assert_eq!(entry.findings[0].class, FindingClass::ConflictMarkers);
  assert_eq!(
    entry.findings[0].line,
    Some(3),
    "the finding points at the marker, not merely at the file"
  );
}

#[test]
fn a_command_that_needs_the_estate_refuses() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  fx.write_file("intent/wip.md", CONFLICTED);

  let mut store = Store::open_in_memory().expect("open");
  let err = ingest::refresh_index(&fx.project(), &mut store).expect_err("must refuse");
  let found = findings(err);
  assert_eq!(found[0].class, FindingClass::ConflictMarkers);
  assert_eq!(found[0].file, "intent/wip.md");
}

/// The unparsed state is still RECORDED on refusal -- that is what makes
/// `doctor` able to list what is broken. Refusing and recording are different
/// jobs and both have to happen.
#[test]
fn the_index_still_records_what_is_broken() {
  let fx = Fixture::new();
  fx.write_file("intent/wip.md", CONFLICTED);

  let mut store = Store::open_in_memory().expect("open");
  let _ = ingest::refresh_index(&fx.project(), &mut store).expect_err("must refuse");

  let indexed = store.file_index().expect("read index");
  let entry = indexed
    .iter()
    .find(|e| e.path == "intent/wip.md")
    .expect("the broken file is in the index");
  assert_eq!(entry.state, FileState::Unparsed);
  assert_eq!(entry.findings[0].class, FindingClass::ConflictMarkers);
}

#[test]
fn a_conflict_markered_thread_json_is_refused_by_ingest() {
  let fx = Fixture::new();
  fx.write_raw_thread(
    "ST0056",
    "{\n<<<<<<< HEAD\n  \"id\": \"ST0056\"\n=======\n  \"id\": \"ST0057\"\n>>>>>>> other\n}\n",
  );

  // Structured canon full of conflict markers is not JSON, so ingest refuses
  // it too. v2 grepped straight through markers and answered from whichever
  // side its regex hit first, which is the silent-wrong-answer this replaces.
  let found = findings(ingest::read(&fx.project()).expect_err("must refuse"));
  assert_eq!(found[0].class, FindingClass::MalformedJson);
  assert_eq!(found[0].file, "intent/st/ST0056/thread.json");
}

#[test]
fn a_clean_estate_does_not_refuse() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  fx.write_file("intent/wip.md", "# Work In Progress\n\nAll fine.\n");

  let mut store = Store::open_in_memory().expect("open");
  ingest::refresh_index(&fx.project(), &mut store).expect("a clean estate passes");
}

/// The false-positive control. Intent's own documentation discusses git
/// conflicts and markdown uses `=======` as a setext underline, so a check
/// keyed on the divider would report the docs as broken. This asserts the
/// check is safe to run over prose that talks about the thing it detects.
#[test]
fn prose_about_conflicts_is_not_reported_as_conflicted() {
  let fx = Fixture::new();
  fx.write_file(
    "intent/docs/merging.md",
    "Resolving conflicts\n===================\n\nGit writes a `<<<<<<<` line when a merge conflicts.\n",
  );

  let entries = intentsvcs::sync::scan(fx.root(), &[]).expect("scan");
  let entry = entries
    .iter()
    .find(|e| e.path == "intent/docs/merging.md")
    .expect("indexed");
  assert_eq!(
    entry.state,
    FileState::Changed,
    "a setext underline and a lone marker mentioned in prose are not a conflict: {:?}",
    entry.findings
  );
}
