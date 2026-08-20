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

  let findings = views::skew(
    &project,
    &canon(),
    &ctx(),
    &intentsvcs::intentfiles::Realised::NothingSaid,
  );
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

  let findings = views::skew(
    &project,
    &canon(),
    &ctx(),
    &intentsvcs::intentfiles::Realised::NothingSaid,
  );
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

  let findings = views::skew(
    &project,
    &canon(),
    &ctx(),
    &intentsvcs::intentfiles::Realised::NothingSaid,
  );
  assert_eq!(findings.len(), 1);
  assert_eq!(findings[0].file, "intent/st/steel_threads.md");
}

#[test]
fn a_missing_view_is_reported_as_skew() {
  let fx = Fixture::new();
  let project = fx.project();
  views::write_all(&project, &canon(), &ctx()).expect("write");
  std::fs::remove_file(fx.path("intent/todo.md")).expect("rm");

  let findings = views::skew(
    &project,
    &canon(),
    &ctx(),
    &intentsvcs::intentfiles::Realised::NothingSaid,
  );
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

  let mut files: Vec<String> = views::skew(
    &project,
    &changed,
    &ctx(),
    &intentsvcs::intentfiles::Realised::NothingSaid,
  )
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

  let files: Vec<String> = views::skew(
    &project,
    &changed,
    &ctx(),
    &intentsvcs::intentfiles::Realised::NothingSaid,
  )
  .into_iter()
  .map(|f| f.file)
  .collect();
  assert!(
    files.contains(&"intent/st/steel_threads.md".to_string()),
    "the index carries status and completion, so a status change skews it: {files:?}"
  );
}

// ---------------------------------------------------------------------------
// WP-10: a dehydrated view is absent BY DESIGN and is not skew
// ---------------------------------------------------------------------------

/// AT-10.1 / AC-10.1 -- WP-10.
///
/// **The pair, driven together, because either half alone passes on the bug.**
///
/// The evening the estate dehydrated, `doctor` reported 234 findings at rc=1 on
/// a healthy tree -- every one of them a view whose thread `.intentfiles` does
/// not declare, every one instructing the operator to regenerate a file the
/// design says should not exist. The remedy it printed would have been answered
/// by `organize` re-hydrating nothing.
///
/// **A blanket `Err(_) => {}` fixes that count and is the wrong fix**: it trades
/// 234 false findings for one SILENT REAL one, because a view missing for a
/// DECLARED artefact is a genuine loss. So the two arms are asserted in one
/// test against one fixture, differing ONLY in what the manifest says. A fix
/// that silences everything fails the first assertion; the pre-fix behaviour
/// fails the second. Nothing passes both by accident.
#[test]
fn absence_is_silence_only_where_the_manifest_says_undeclared() {
  use intentsvcs::intentfiles::Realised;

  let fx = Fixture::new();
  let project = fx.project();
  views::write_all(&project, &canon(), &ctx()).expect("write");

  // Dehydrate: the views for ST0056 leave disk, exactly as `organize` removes
  // them. `todo.md` and `steel_threads.md` are project-level, belong to no
  // artefact, and stay -- so any finding below is about the thread's views.
  for rel in ["intent/st/ST0056/info.md", "intent/st/ST0056/acceptance.md"] {
    std::fs::remove_file(project.root().join(rel)).expect("dehydrate");
  }

  // ARM 1 -- the manifest declares NOTHING. The thread is not realised, its
  // views are legitimately gone, and doctor must be silent about them.
  let declared_none = Realised::Declared(std::collections::BTreeSet::new());
  let findings = views::skew(&project, &canon(), &ctx(), &declared_none);
  assert!(
    findings.is_empty(),
    "an undeclared thread's views are absent by design, not skew: {findings:?}"
  );

  // ARM 2 -- the SAME missing files, with the manifest declaring the thread
  // realised. Now they are a real loss and must be named.
  let declared_st0056 = Realised::Declared(["ST0056".to_string()].into_iter().collect());
  let findings = views::skew(&project, &canon(), &ctx(), &declared_st0056);
  let files: Vec<&str> = findings.iter().map(|f| f.file.as_str()).collect();
  assert_eq!(
    findings.len(),
    2,
    "a DECLARED thread's missing views are still a finding: {files:?}"
  );
  assert!(
    files.contains(&"intent/st/ST0056/info.md")
      && files.contains(&"intent/st/ST0056/acceptance.md"),
    "and they are named: {files:?}"
  );
}

/// AT-10.2 / AC-10.2 -- WP-10.
///
/// **ABSENT IS NOT EMPTY, and the two absent-ish manifests must behave alike.**
///
/// A missing manifest and an unreadable one both realise EVERYTHING -- the
/// fail-open direction, and the only one that cannot delete anybody's files. A
/// project that has never run `organize` must be checked exactly as it was
/// before WP-10 landed, and a project whose manifest is broken must not have
/// its whole estate silently drop out of the check.
#[test]
fn an_absent_or_unreadable_manifest_keeps_every_view_in_scope() {
  use intentsvcs::intentfiles::Realised;

  let fx = Fixture::new();
  let project = fx.project();
  views::write_all(&project, &canon(), &ctx()).expect("write");
  std::fs::remove_file(project.root().join("intent/st/ST0056/info.md")).expect("remove");

  for state in [Realised::NothingSaid, Realised::Unreadable] {
    let findings = views::skew(&project, &canon(), &ctx(), &state);
    assert_eq!(
      findings.len(),
      1,
      "{state:?} declares everything, so a missing view is still a finding"
    );
    assert_eq!(findings[0].file, "intent/st/ST0056/info.md");
  }
}
