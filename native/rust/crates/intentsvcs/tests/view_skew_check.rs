//! AT-03.4 / AC-03.4: the skew check catches a hand-edited generated view and
//! names the file -- never silently outvotes it.
//!
//! "Never silently" is the load-bearing half. Regenerating over a hand-edit is
//! easy and is what v2 did; the defect is that the person who made the edit
//! has no way to learn it was discarded. The check exists so the tool can say
//! "you edited a generated file" instead of quietly winning.

use crate::common::{Fixture, ctx, issue, sample_thread};
use intentsvcs::finding::FindingClass;
use intentsvcs::ingest::Canon;
use intentsvcs::views;

fn canon() -> Canon {
  Canon {
    threads: vec![sample_thread("ST0056")],
    issues: Vec::new(),
    sections: Vec::new(),
    boards: Vec::new(),
    declaration: None,
  }
}

/// The skew check as these tests ask it: with no record of what the store last
/// rendered and no view that a commit leaves out, so every difference counts.
/// doctor asks the same door, [`views::skew_against`], with both answers.
fn skew(
  project: &intentsvcs::project::Project,
  canon: &Canon,
  ctx: &views::RenderContext<'_>,
  format: intentsvcs::canon_format::CanonFormat,
  realised: &intentsvcs::intentfiles::Realised,
) -> Vec<intentsvcs::finding::Finding> {
  views::skew_against(
    project,
    canon,
    ctx,
    format,
    realised,
    &|_, _| false,
    &|_| false,
  )
}

#[test]
fn freshly_written_views_have_no_skew() {
  let fx = Fixture::new();
  let project = fx.project();
  views::write_all(&project, &canon(), &ctx()).expect("write");

  let findings = skew(
    &project,
    &canon(),
    &ctx(),
    intentsvcs::intentfiles::reported_format(&project),
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

  let todo = project.todo_view();
  let edited = format!("{}\nsomeone typed this by hand\n", fx.read_at(&todo));
  fx.write_at(&todo, &edited);

  let findings = skew(
    &project,
    &canon(),
    &ctx(),
    intentsvcs::intentfiles::reported_format(&project),
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

  let index = project.steel_threads_view();
  let original = fx.read_at(&index);
  fx.write_at(
    &index,
    &original.replacen("Steel Threads", "Steel threads", 1),
  );

  let findings = skew(
    &project,
    &canon(),
    &ctx(),
    intentsvcs::intentfiles::reported_format(&project),
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
  std::fs::remove_file(project.todo_view()).expect("rm");

  let findings = skew(
    &project,
    &canon(),
    &ctx(),
    intentsvcs::intentfiles::reported_format(&project),
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

  let mut files: Vec<String> = skew(
    &project,
    &changed,
    &ctx(),
    intentsvcs::intentfiles::reported_format(&project),
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
      "intent/st/steel_threads.md",
      "intent/todo.md",
    ],
    "the index carries the thread title as its descriptive column, so a title change skews it along with the thread's own views -- and nothing else"
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

  let files: Vec<String> = skew(
    &project,
    &changed,
    &ctx(),
    intentsvcs::intentfiles::reported_format(&project),
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
  for view in [
    project.info_view("ST0056"),
    project.acceptance_view("ST0056"),
  ] {
    std::fs::remove_file(view).expect("dehydrate");
  }

  // ARM 1 -- the manifest declares NOTHING. The thread is not realised, its
  // two removed views are legitimately gone, and doctor must be silent about
  // them. The views it still has on disk, byte for byte their render, are
  // named as unrealised (issue 0707): shown, and never counted.
  let declared_none = Realised::Declared(std::collections::BTreeSet::new());
  let findings = skew(
    &project,
    &canon(),
    &ctx(),
    intentsvcs::intentfiles::reported_format(&project),
    &declared_none,
  );
  let removed = ["intent/st/ST0056/info.md", "intent/st/ST0056/acceptance.md"];
  assert!(
    findings
      .iter()
      .all(|f| f.class == FindingClass::UnrealisedView && !removed.contains(&f.file.as_str())),
    "an undeclared thread's views are absent by design, not skew, and one kept is only named: {findings:?}"
  );

  // ARM 2 -- the SAME missing files, with the manifest declaring the thread
  // realised. Now they are a real loss and must be named.
  // **KEYED THROUGH `declared_key` RATHER THAN SPELLED BY HAND.** The set held
  // bare ids until ST0069 WP-01 widened it to the wire form so a second sigil
  // could live in it; a literal "ST0056" here silently declares nothing, and
  // this arm would then assert that a DECLARED thread's missing views are
  // silent -- the exact opposite of what it is for.
  let declared_st0056 = Realised::Declared(
    [intentsvcs::intentfiles::declared_key(
      intentsvcs::intentfiles::Sigil::SteelThread,
      "ST0056",
    )]
    .into_iter()
    .collect(),
  );
  let findings = skew(
    &project,
    &canon(),
    &ctx(),
    intentsvcs::intentfiles::reported_format(&project),
    &declared_st0056,
  );
  // Counted findings only: a view of another record the manifest leaves
  // unrealised is named beside them, and is not what this arm is about.
  let files: Vec<&str> = findings
    .iter()
    .filter(|f| f.class.is_actionable())
    .map(|f| f.file.as_str())
    .collect();
  assert_eq!(
    files.len(),
    2,
    "a DECLARED thread's missing views are still a finding: {findings:?}"
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
  std::fs::remove_file(project.info_view("ST0056")).expect("remove");

  for state in [Realised::NothingSaid, Realised::Unreadable] {
    let findings = skew(
      &project,
      &canon(),
      &ctx(),
      intentsvcs::intentfiles::reported_format(&project),
      &state,
    );
    assert_eq!(
      findings.len(),
      1,
      "{state:?} declares everything, so a missing view is still a finding"
    );
    assert_eq!(findings[0].file, "intent/st/ST0056/info.md");
  }
}

/// **0283 HALF B: A MUTATION REFRESHES AN UNDECLARED THREAD'S VIEW ONLY WHEN
/// THE DISK STILL HOLDS WHAT THE STORE LAST RENDERED.** A closed thread drops
/// out of `.intentfiles` while its files stay on disk, and the projection
/// skipped every undeclared view -- so `ac new` on it left `acceptance.md`
/// stale, and doctor called the store-ahead gap a hand edit.
///
/// Disk equal to the render of canon BEFORE the change is store-ahead by
/// construction, so it is refreshed; anything else is a hand edit and is left
/// for doctor. Either way the thread is not re-declared: that is `st hydrate`'s
/// act, not a mutation's side effect.
#[test]
fn a_mutation_refreshes_an_undeclared_view_only_when_the_disk_is_what_the_store_last_rendered() {
  use intentsvcs::address::{Address, Entity};
  use intentsvcs::model::{AcKind, ThreadStatus};
  const NOTHING_DECLARED: &str = "# .intentfiles\n\n";

  // Realise a closed thread, then undeclare it, optionally hand-edit its
  // contract view, then add a criterion. Returns the view before and after the
  // mutation, and the manifest after.
  let drive = |hand_edit: bool| -> (String, String, String) {
    // 3.4.0's format, where the thread is undeclared through `.intentfiles`
    // (ST0096 WP-01).
    let fx = Fixture::ordinal();
    let mut thread = sample_thread("ST0001");
    thread.status = ThreadStatus::Completed;
    fx.write_thread(&thread);
    fx.write_manifest(NOTHING_DECLARED);
    let mut facade = fx.facade();
    facade
      .hydrate(&Address {
        authority: None,
        entity: Entity::Thread {
          id: "ST0001".to_string(),
        },
        format: None,
      })
      .expect("realise the thread");
    fx.write_manifest(NOTHING_DECLARED);
    let view = fx.project().acceptance_view("ST0001");
    if hand_edit {
      let mut text = std::fs::read_to_string(&view).expect("the realised view");
      text.push_str("\na hand edit\n");
      std::fs::write(&view, text).expect("hand-edit the view");
    }
    let before = std::fs::read_to_string(&view).expect("the view before");
    facade
      .ac_new(
        "ST0001",
        "AC-03.12",
        "added after the close",
        AcKind::NonTest,
      )
      .expect("add a criterion to the closed thread");
    let after = std::fs::read_to_string(&view).expect("the view after");
    let manifest = fx.read_manifest();
    (before, after, manifest)
  };

  let (before, after, manifest) = drive(false);
  assert!(
    !before.contains("AC-03.12") && after.contains("AC-03.12"),
    "the store is ahead and the disk held its last render, so the view is refreshed: {after}"
  );
  assert_eq!(
    manifest, NOTHING_DECLARED,
    "the mutation does not re-declare the thread"
  );

  let (before, after, _) = drive(true);
  assert_eq!(
    after, before,
    "a hand-edited view is left for doctor, byte for byte"
  );
}

/// **ST0069 WP-01's defect, found by the live estate the moment its 284
/// undeclared issue views were removed: `doctor` reported all 284 as missing.**
///
/// The absent-view arm asks [`views::owning_thread`] and nothing else, and that
/// function answers `None` for `intent/issues/<nnnn>.md` -- so `is_some_and`
/// made `dehydrated` false for every issue view in the estate and the finding
/// fired. **The same `None`-makes-the-whole-condition-false shape as the
/// projection defect two commits earlier**, in the sibling reader, found the
/// same way: by an estate whose issues are undeclared, which no fixture reaching
/// an issue through `issues add` can be, because that verb declares it.
///
/// The pair is driven together for the reason [AT-10.1's arm] states above:
/// either half alone passes on a bug. A blanket silence for issue views fails
/// arm 2; the pre-fix behaviour fails arm 1.
#[test]
fn an_issues_absent_view_is_silent_only_where_the_manifest_says_undeclared() {
  use intentsvcs::intentfiles::{Realised, Sigil, declared_key};

  let fx = Fixture::new();
  let project = fx.project();
  let mut canon = canon();
  canon.issues = vec![crate::common::sample_issue(21)];
  views::write_all(&project, &canon, &ctx()).expect("write");

  // Dehydrate the issue's view alone. The thread's views stay, and it stays
  // declared in both arms, so every finding below is about the issue.
  std::fs::remove_file(project.issue_view(issue(21))).expect("dehydrate the issue view");

  let thread_only = Realised::Declared(
    [declared_key(Sigil::SteelThread, "ST0056")]
      .into_iter()
      .collect(),
  );
  let findings = skew(
    &project,
    &canon,
    &ctx(),
    intentsvcs::intentfiles::reported_format(&project),
    &thread_only,
  );
  assert!(
    findings.is_empty(),
    "an undeclared issue's view is absent by design, exactly as a thread's is: {findings:?}"
  );

  let with_the_issue = Realised::Declared(
    [
      declared_key(Sigil::SteelThread, "ST0056"),
      declared_key(Sigil::Issue, "0021"),
    ]
    .into_iter()
    .collect(),
  );
  let findings = skew(
    &project,
    &canon,
    &ctx(),
    intentsvcs::intentfiles::reported_format(&project),
    &with_the_issue,
  );
  assert_eq!(
    findings.len(),
    1,
    "a DECLARED issue's missing view is still a real loss: {findings:?}"
  );
  assert_eq!(findings[0].file, "intent/issues/0021.md");
  assert!(
    findings[0].detail.contains("missing"),
    "and it is the absent condition, not the edited one: {}",
    findings[0].detail
  );
}

/// Issue 0412: an undeclared issue's view that differs from its render names a
/// remedy that clears it. `sync --to-disk` skips an undeclared view and
/// `organize --apply` refuses to remove one it cannot tell from a hand edit, so
/// the old remedy exited 0 and left the finding standing -- which wedged a
/// commit gate. The two routes named are the ones that clear it.
#[test]
fn an_undeclared_issue_s_differing_view_names_a_remedy_that_clears_it() {
  use intentsvcs::intentfiles::{Realised, Sigil, declared_key};

  // 3.4.0's format, where an issue is declared by its manifest key; an
  // upgraded project's wording is `a_skewed_unrealised_view_names_what_realises_it_in_each_format`'s.
  let fx = Fixture::ordinal();
  let project = fx.project();
  let mut canon = canon();
  canon.issues = vec![crate::common::sample_issue(21)];
  views::write_all(&project, &canon, &ctx()).expect("write");
  let view = project.issue_view(crate::common::issue(21));
  let edited = format!("{}\na leftover line\n", fx.read_at(&view));
  fx.write_at(&view, &edited);

  let thread_only = Realised::Declared(
    [declared_key(Sigil::SteelThread, "ST0056")]
      .into_iter()
      .collect(),
  );
  let findings = skew(
    &project,
    &canon,
    &ctx(),
    intentsvcs::intentfiles::reported_format(&project),
    &thread_only,
  );
  assert_eq!(findings.len(), 1, "{findings:?}");
  assert_eq!(findings[0].file, "intent/issues/0021.md");
  assert_eq!(findings[0].class, FindingClass::ViewSkew);
  // Issue 0740: the detail says what is true of the view, the key included,
  // and the finding's one remedy says what clears it.
  let detail = &findings[0].detail;
  let remedy = findings[0].remedy();
  assert!(
    detail.contains("`ISSUE:0021`") && detail.contains("is not declared in"),
    "the detail names the issue's key and that nothing declares it: {detail}"
  );
  assert!(
    remedy.contains("delete it") && remedy.contains("`intent organize --apply`"),
    "the remedy names removing the file or declaring it: {remedy}"
  );
  assert!(
    !remedy.contains("`intent sync --to-disk` regenerates it"),
    "and never the sync that skips an undeclared view: {remedy}"
  );
}

/// **EACH FORMAT NAMES WHAT REALISES A RECORD IN ITS OWN WORDS** (ST0096
/// WP-01; cc's wording, ST0095 WP-10). A hand-edited view of a thread and of
/// two issues, one closed and one open, with nothing realised: 3.4.0's format
/// names the manifest by its path, and an upgraded project names the record,
/// saying of an issue whether it is closed or open. Each arm reads its own
/// distinguishing phrase, so a swapped branch fails here.
#[test]
fn a_skewed_unrealised_view_names_what_realises_it_in_each_format() {
  use intentsvcs::intentfiles::Realised;
  use intentsvcs::model::IssueStatus;

  let mut open = crate::common::sample_issue(22);
  open.status = IssueStatus::Open;
  open.closed = None;
  let closed = crate::common::sample_issue(21);
  let mut canon = canon();
  canon.issues = vec![closed.clone(), open.clone()];

  // The three views, as the project lays them out: the thread's, the closed
  // issue's and the open issue's.
  let views_of = |project: &intentsvcs::project::Project| {
    [
      project.info_view("ST0056"),
      project.issue_view(closed.number),
      project.issue_view(open.number),
    ]
  };
  let skewed = |fx: &Fixture| -> Vec<intentsvcs::finding::Finding> {
    let project = fx.project();
    views::write_all(&project, &canon, &ctx()).expect("write");
    for path in views_of(&project) {
      let text = std::fs::read_to_string(&path).expect("a rendered view");
      std::fs::write(&path, format!("{text}\nsomeone typed this by hand\n")).expect("edit");
    }
    let nothing = Realised::Declared(std::collections::BTreeSet::new());
    skew(
      &project,
      &canon,
      &ctx(),
      intentsvcs::intentfiles::reported_format(&project),
      &nothing,
    )
  };
  // What a finding says and the one remedy it names (issue 0740): the hydrate
  // verb is a remedy, and the issue's realisation a fact of its detail.
  let found =
    |findings: &[intentsvcs::finding::Finding], rel: &str| -> intentsvcs::finding::Finding {
      findings
        .iter()
        .find(|f| f.file == rel && f.class == FindingClass::ViewSkew)
        .cloned()
        .unwrap_or_else(|| panic!("a skew finding for {rel}: {findings:?}"))
    };
  let said = |findings: &[intentsvcs::finding::Finding], rel: &str| -> String {
    found(findings, rel).detail
  };
  let remedied = |findings: &[intentsvcs::finding::Finding], rel: &str| -> String {
    found(findings, rel).remedy().to_string()
  };
  let rels_of = |fx: &Fixture| {
    let project = fx.project();
    views_of(&project).map(|path| project.relative(&path))
  };

  let ordinal = Fixture::ordinal();
  let findings = skewed(&ordinal);
  let [thread_rel, closed_rel, _] = rels_of(&ordinal);
  assert!(
    remedied(&findings, &thread_rel).contains("pins ST0056 in `intent/.intentfiles`"),
    "3.4.0's format names the manifest by its path: {findings:?}"
  );
  assert!(
    said(&findings, &closed_rel).contains("is not declared in `intent/.intentfiles`"),
    "and so does its issue sentence: {findings:?}"
  );

  let upgraded = Fixture::new();
  let findings = skewed(&upgraded);
  let [thread_rel, closed_rel, open_rel] = rels_of(&upgraded);
  assert!(
    remedied(&findings, &thread_rel).contains("sets ST0056 realised on its canon record"),
    "an upgraded project names the record: {findings:?}"
  );
  assert!(
    said(&findings, &closed_rel).contains("is closed, so its record does not realise it"),
    "a closed issue says it is closed: {findings:?}"
  );
  assert!(
    said(&findings, &open_rel).contains("is open, but its record does not realise it"),
    "an open issue is never called closed: {findings:?}"
  );
}
