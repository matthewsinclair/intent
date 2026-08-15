//! AT-03.4 / AC-03.4: the skew check catches a hand-edited generated view and
//! names the file -- never silently outvotes it.
//!
//! "Never silently" is the load-bearing half. Regenerating over a hand-edit is
//! easy and is what v2 did; the defect is that the person who made the edit
//! has no way to learn it was discarded. The check exists so the tool can say
//! "you edited a generated file" instead of quietly winning.

mod common;

use common::{Fixture, ctx, sample_thread};
use intentsvcs::finding::FindingClass;
use intentsvcs::ingest::Canon;
use intentsvcs::views;

fn canon() -> Canon {
  Canon {
    threads: vec![sample_thread("ST0056")],
    issues: Vec::new(),
    sections: Vec::new(),
  }
}

#[test]
fn freshly_written_views_have_no_skew() {
  let fx = Fixture::new();
  let project = fx.project();
  views::write_all(&project, &canon(), &ctx()).expect("write");

  let findings = views::skew(&project, &canon(), &ctx());
  assert!(
    findings.is_empty(),
    "a view just written by the renderer cannot be skewed: {findings:?}"
  );
}

#[test]
fn a_hand_edited_view_is_caught_and_named() {
  let fx = Fixture::new();
  let project = fx.project();
  views::write_all(&project, &canon(), &ctx()).expect("write");

  let edited = format!(
    "{}\nsomeone typed this by hand\n",
    fx.read("intent/todo.md")
  );
  fx.write_file("intent/todo.md", &edited);

  let findings = views::skew(&project, &canon(), &ctx());
  assert_eq!(findings.len(), 1, "exactly the edited file: {findings:?}");
  assert_eq!(findings[0].file, "intent/todo.md");
  assert_eq!(findings[0].class, FindingClass::ViewSkew);
  assert!(
    findings[0].detail.contains("edited by hand"),
    "the finding says what happened and what to do, got: {}",
    findings[0].detail
  );
}

/// A one-character edit is still an edit. A check that only caught large
/// differences would pass on precisely the changes people make by hand.
#[test]
fn a_single_character_edit_is_caught() {
  let fx = Fixture::new();
  let project = fx.project();
  views::write_all(&project, &canon(), &ctx()).expect("write");

  let original = fx.read("intent/st/steel_threads.md");
  fx.write_file(
    "intent/st/steel_threads.md",
    &original.replacen("Steel Threads", "Steel threads", 1),
  );

  let findings = views::skew(&project, &canon(), &ctx());
  assert_eq!(findings.len(), 1);
  assert_eq!(findings[0].file, "intent/st/steel_threads.md");
}

#[test]
fn a_missing_view_is_reported_as_skew() {
  let fx = Fixture::new();
  let project = fx.project();
  views::write_all(&project, &canon(), &ctx()).expect("write");
  std::fs::remove_file(fx.path("intent/todo.md")).expect("rm");

  let findings = views::skew(&project, &canon(), &ctx());
  assert_eq!(findings.len(), 1);
  assert!(
    findings[0].detail.contains("missing"),
    "an absent view is a distinct condition from an edited one, got: {}",
    findings[0].detail
  );
}

/// A model change makes the on-disk views stale, and the check says so for
/// every view that actually carries the changed fact -- and stays quiet about
/// the ones that do not. This is the case that matters in practice: nobody
/// edited anything, the truth moved.
///
/// Reporting only the AFFECTED views is half the value. A check that flagged
/// everything on every change is back to crying wolf, which is the failure the
/// no-clock law and the formatter-stability rule both exist to prevent.
#[test]
fn a_title_change_skews_exactly_the_views_that_carry_the_title() {
  let fx = Fixture::new();
  let project = fx.project();
  views::write_all(&project, &canon(), &ctx()).expect("write");

  let mut changed = canon();
  changed.threads[0].title = "Intent v3.0.0 -- retitled".to_string();

  let mut files: Vec<String> = views::skew(&project, &changed, &ctx())
    .into_iter()
    .map(|f| f.file)
    .collect();
  files.sort();
  assert_eq!(
    files,
    vec![
      "intent/st/ST0056/acceptance.md",
      "intent/st/ST0056/info.md",
      "intent/todo.md",
    ],
    "steel_threads.md indexes by slug and status and carries no title column, so a title change must NOT skew it"
  );
}

#[test]
fn a_status_change_skews_the_index() {
  let fx = Fixture::new();
  let project = fx.project();
  views::write_all(&project, &canon(), &ctx()).expect("write");

  let mut changed = canon();
  changed.threads[0].status = intentsvcs::model::ThreadStatus::Completed;
  changed.threads[0].completed = Some("2026-08-15".to_string());

  let files: Vec<String> = views::skew(&project, &changed, &ctx())
    .into_iter()
    .map(|f| f.file)
    .collect();
  assert!(
    files.contains(&"intent/st/steel_threads.md".to_string()),
    "the index carries status and completion, so a status change skews it: {files:?}"
  );
}
