//! ST0096 WP-05 slice B (AC-05.3): **`intent upgrade` writes a 3.4.0 project's
//! canon in the release's layout, the same files on every copy, and refuses
//! before any write what the layout cannot carry.**
//!
//! A thread goes from one file to one for its own fields and one per work
//! package, criterion, test and attachment; no id and no value moves (the
//! cover's shape is `views.rs`'s own test). A closed record's views go with the upgrade, through organize's own
//! plan and gate. Two things are refused before anything is written: an
//! attachment of an unrealised thread holding an edit its canon does not
//! record, and two attachments of one thread a disk that folds case stores as
//! one file (issue 0711's guard, asked before any record is encoded).

use crate::common::{Fixture, ctx, facade_ctx, sample_issue, sample_thread, tree};
use intentsvcs::facade::{Facade, FacadeError};
use intentsvcs::id::Named;
use intentsvcs::migrate::Blocked;
use intentsvcs::model::{Attachment, Thread, ThreadStatus};
use intentsvcs::views;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// The ordinal of the issue every estate here holds.
const ISSUE: u32 = 7;

/// The canon under `fx`, each file's bytes keyed by its path: every record's
/// file and the project state. The event log is left out, because each event
/// carries the moment of its own write and is not a record's layout.
fn canon_files(fx: &Fixture) -> BTreeMap<String, Vec<u8>> {
  let project = fx.project();
  let canon = project
    .project_json()
    .parent()
    .expect("the canon directory")
    .to_path_buf();
  let events = project.relative(&project.events_dir());
  tree(&canon)
    .into_iter()
    .filter(|(rel, _)| !project.relative(&canon.join(rel)).starts_with(&events))
    .collect()
}

/// A 3.4.0 project holding `threads` and the sample issue, in git.
fn ordinal_estate(threads: &[Thread]) -> Fixture {
  let fx = Fixture::ordinal();
  for thread in threads {
    fx.write_thread(thread);
  }
  fx.write_issue(&sample_issue(ISSUE));
  fx.git_init();
  fx.git_commit_all();
  fx
}

/// The whole tree under `root` but the store's own files, which a refused
/// upgrade may open and which are not the project's.
fn estate(root: &Path) -> BTreeMap<String, Vec<u8>> {
  tree(root)
    .into_iter()
    .filter(|(rel, _)| !rel.split('/').any(|part| part == ".cache"))
    .collect()
}

// ST0096 AT-05.3: the arm below.
#[test]
fn two_copies_upgraded_apart_write_the_same_files_and_move_no_id_or_value() {
  let thread = sample_thread("ST0001");
  let issue = sample_issue(ISSUE);
  for (kind, empty) in [
    ("work packages", thread.wps.is_empty()),
    ("criteria", thread.criteria.is_empty()),
    ("tests", thread.tests.is_empty()),
    ("attachments", thread.attachments.is_empty()),
  ] {
    assert!(
      !empty,
      "the sample must carry {kind}, or their staying put is not tested"
    );
  }
  let copies = [
    ordinal_estate(std::slice::from_ref(&thread)),
    ordinal_estate(std::slice::from_ref(&thread)),
  ];
  for fx in &copies {
    Facade::upgrade(&fx.project(), &facade_ctx()).expect("the upgrade");
  }
  let [a, b] = &copies;
  assert_eq!(
    canon_files(a),
    canon_files(b),
    "two copies upgraded apart wrote different canon"
  );

  let read = intentsvcs::ingest::read(&a.project()).expect("the upgraded canon reads");
  let upgraded = read
    .threads
    .iter()
    .find(|t| t.id == thread.id)
    .expect("the thread keeps its id");
  // As sets, keyed by each record's own name: a record's file is the record,
  // and no file holds an order (A1's layout).
  fn keyed<T: Clone>(records: &[T], key: impl Fn(&T) -> String) -> BTreeMap<String, T> {
    records.iter().map(|r| (key(r), r.clone())).collect()
  }
  assert_eq!(
    keyed(&upgraded.wps, |w| w.seq.stored()),
    keyed(&thread.wps, |w| w.seq.stored()),
    "a work package moved"
  );
  assert_eq!(
    keyed(&upgraded.criteria, |c| c.id.clone()),
    keyed(&thread.criteria, |c| c.id.clone()),
    "a criterion moved"
  );
  assert_eq!(
    keyed(&upgraded.tests, |t| t.id.clone()),
    keyed(&thread.tests, |t| t.id.clone()),
    "a test moved"
  );
  assert_eq!(
    keyed(&upgraded.attachments, |a| a.path.clone()),
    keyed(&thread.attachments, |a| a.path.clone()),
    "an attachment moved"
  );
  assert_eq!(
    (&upgraded.title, &upgraded.objective, &upgraded.context),
    (&thread.title, &thread.objective, &thread.context),
    "the thread's own fields moved"
  );
  let read_issue = read
    .issues
    .iter()
    .find(|i| i.number == issue.number)
    .expect("the issue keeps its number");
  assert_eq!(
    (&read_issue.title, &read_issue.body, read_issue.status),
    (&issue.title, &issue.body, issue.status),
    "the issue's fields moved"
  );

  let own = std::fs::read_to_string(a.project().thread_json(&thread.id)).expect("the own file");
  let own: serde_json::Value = serde_json::from_str(&own).expect("the own file is json");
  for key in ["wps", "criteria", "tests", "attachments"] {
    assert!(
      own.get(key).is_none(),
      "the upgraded thread's own file still lists its {key}"
    );
  }
}

#[test]
fn an_upgrade_removes_a_closed_issues_view_and_names_it() {
  let fx = ordinal_estate(&[]);
  let mut issue = sample_issue(ISSUE);
  issue.realised = false;
  fx.write_issue(&issue);
  // The view as this build renders it, so the gate has no edit to refuse.
  let project = fx.project();
  let canon = intentsvcs::ingest::read(&project).expect("the estate reads");
  let written: Vec<PathBuf> = views::render_all(&project, &canon, &ctx())
    .into_iter()
    .filter(|v| matches!(views::owner(&project, &v.path, &canon), Some(Named::Issue(n)) if n == issue.number))
    .map(|v| {
      std::fs::create_dir_all(v.path.parent().expect("a parent")).expect("mkdir");
      std::fs::write(&v.path, &v.content).expect("write the issue's view");
      v.path
    })
    .collect();
  assert!(
    !written.is_empty(),
    "the fixture rendered no view for the issue"
  );
  fx.git_commit_all();

  let done = Facade::upgrade(&fx.project(), &facade_ctx()).expect("the upgrade");
  assert!(
    done.dehydrate_refused.is_empty(),
    "the closed issue's view was refused: {:?}",
    done.dehydrate_refused
  );
  for view in &written {
    assert!(
      !view.exists(),
      "{} is still on disk: a closed issue's view outlived the upgrade",
      view.display()
    );
    assert!(
      done.dehydrated.contains(view),
      "{} went without being named in the report: {:?}",
      view.display(),
      done.dehydrated
    );
  }
}

#[test]
fn an_unsynced_edit_to_an_unrealised_threads_attachment_is_refused_before_any_write() {
  // A closed thread: a 3.4.0 project with no manifest realises its WIP threads
  // alone, so the upgrade leaves this one unrealised and removes its files.
  let mut thread = sample_thread("ST0001");
  thread.status = ThreadStatus::Completed;
  thread.completed = Some(thread.created.clone());
  thread.realised = false;
  let fx = ordinal_estate(std::slice::from_ref(&thread));
  let attachment = &thread.attachments[0];
  let on_disk = fx.project().thread_dir(&thread.id).join(&attachment.path);
  std::fs::create_dir_all(on_disk.parent().expect("a parent")).expect("mkdir");
  std::fs::write(&on_disk, "an edit the store never took\n").expect("edit the attachment");
  // Committed, so the tree is clean and the format move's dirty-tree guard
  // passes: what this arm refuses is canon and disk disagreeing, which a
  // clean tree can still hold.
  fx.git_commit_all();
  let before = estate(fx.root());

  match Facade::upgrade(&fx.project(), &facade_ctx()) {
    Err(FacadeError::MigrationBlocked(Blocked::DirtyAttachment { path })) => assert_eq!(
      path,
      fx.project().relative(&on_disk),
      "the refusal names another file"
    ),
    other => panic!("an unsynced attachment edit must be refused, got {other:?}"),
  }
  let after = estate(fx.root());
  assert_eq!(
    crate::common::changed(&before, &after),
    Vec::<String>::new(),
    "the refused upgrade wrote"
  );
}

/// Issue Idf2dd7c: the check is organize's gate, at the file
/// `project::disk_match` answers is the row's (issue 0722). On a disk that
/// folds case a file spelled in another case is the row's working copy: its
/// edit is refused, by that file's own name. On one that does not it is a
/// case pair, which nothing removes. Either way the edit survives the upgrade,
/// and a refusal names the file that holds it. A second copy of the gate read
/// only the row's spelling, and on a folding disk named a file nobody wrote.
#[test]
fn an_unsynced_edit_under_another_spelling_survives_and_is_named_by_its_file() {
  const EDIT: &str = "an edit the store never took\n";
  let mut thread = sample_thread("ST0001");
  thread.status = ThreadStatus::Completed;
  thread.completed = Some(thread.created.clone());
  thread.realised = false;
  let fx = ordinal_estate(std::slice::from_ref(&thread));
  let attachment = &thread.attachments[0];
  let dir = fx.project().thread_dir(&thread.id);
  let respelled = dir.join(attachment.path.to_uppercase());
  assert_ne!(
    respelled,
    dir.join(&attachment.path),
    "the fixture's attachment has a spelling another case can differ from"
  );
  std::fs::create_dir_all(respelled.parent().expect("a parent")).expect("mkdir");
  std::fs::write(&respelled, EDIT).expect("edit under another spelling");
  fx.git_commit_all();

  let folds = fx
    .project()
    .disk_folds_case()
    .expect("whether this disk folds case");
  let before = estate(fx.root());

  let upgraded = Facade::upgrade(&fx.project(), &facade_ctx());
  assert_eq!(
    std::fs::read_to_string(&respelled).expect("the edited file is still there"),
    EDIT,
    "the upgrade lost the edit"
  );
  match (folds, upgraded) {
    // The row's working copy holds an edit: refused before any write, by the
    // file that holds it.
    (true, Err(FacadeError::MigrationBlocked(Blocked::DirtyAttachment { path }))) => {
      assert_eq!(
        path,
        fx.project().relative(&respelled),
        "the refusal names another file"
      );
      assert_eq!(
        crate::common::changed(&before, &estate(fx.root())),
        Vec::<String>::new(),
        "the refused upgrade wrote"
      );
    }
    // A case pair, which nothing removes: the upgrade proceeds.
    (false, Ok(_)) => {}
    (folds, other) => panic!("on a disk that folds case: {folds}, the upgrade answered {other:?}"),
  }
}

#[test]
fn two_attachments_one_file_on_a_case_folding_disk_are_refused_before_any_write() {
  let mut thread = sample_thread("ST0001");
  let name = thread.attachments[0].path.clone();
  thread
    .attachments
    .push(Attachment::new(name.to_uppercase(), "the other spelling\n"));
  let fx = ordinal_estate(std::slice::from_ref(&thread));
  let before = estate(fx.root());

  match Facade::upgrade(&fx.project(), &facade_ctx()) {
    Err(FacadeError::MigrationBlocked(Blocked::CaseCollision { path, other })) => assert!(
      path.ends_with(&name) && other.ends_with(&name.to_uppercase()),
      "the refusal names another pair: {path}, {other}"
    ),
    other => panic!("a case pair must be refused, got {other:?}"),
  }
  let after = estate(fx.root());
  assert_eq!(
    crate::common::changed(&before, &after),
    Vec::<String>::new(),
    "the refused upgrade wrote"
  );
}

/// **THE MOVE TO THE RELEASE'S FORMAT REFUSES A DIRTY TREE** (intent-cc's
/// ruling, on devbin-cc's drive of a 3.4.x
/// project that upgraded at exit 0 over a tracked edit and an untracked file).
/// The move is one commit whose revert is the rollback, so it refuses before
/// any write and names every path, the untracked one too.
#[test]
fn a_dirty_tree_refuses_the_format_move_naming_every_path_and_writing_nothing() {
  let fx = ordinal_estate(&[sample_thread("ST0001")]);
  fx.write_file("NOTES.md", "notes\n");
  fx.git_commit_all();
  fx.write_file("NOTES.md", "notes\nan edit not yet committed\n");
  fx.write_file("untracked.txt", "scratch work\n");
  let before = estate(fx.root());

  match Facade::upgrade(&fx.project(), &facade_ctx()) {
    Err(FacadeError::MigrationOverDirtyTree { paths, .. }) => {
      let named: Vec<&str> = paths.iter().map(|p| p.path.as_str()).collect();
      for path in ["NOTES.md", "untracked.txt"] {
        assert!(
          named.contains(&path),
          "the refusal does not name {path}: {named:?}"
        );
      }
    }
    other => panic!("a dirty tree must refuse the format move, got {other:?}"),
  }
  let after = estate(fx.root());
  assert_eq!(
    crate::common::changed(&before, &after),
    Vec::<String>::new(),
    "the refused upgrade wrote"
  );
}

/// **A RE-RUN THAT MOVES NO FORMAT STILL RUNS ON A DIRTY TREE**, as the
/// convergent upgrade always has, and makes no claim about the tree: only a run
/// that passed a clean-tree guard may say every change is its own.
#[test]
fn a_re_run_on_an_upgraded_project_runs_on_a_dirty_tree_and_claims_nothing_about_it() {
  let fx = ordinal_estate(&[sample_thread("ST0001")]);
  let first =
    Facade::upgrade(&fx.project(), &facade_ctx()).expect("the first upgrade, on a clean tree");
  assert!(
    first.on_a_clean_tree,
    "the format move passed its guard, so it may say so"
  );
  fx.write_file("NOTES.md", "notes\n");
  fx.git_commit_all();
  fx.write_file("NOTES.md", "notes\nan edit not yet committed\n");
  fx.write_file("untracked.txt", "scratch work\n");

  let again = Facade::upgrade(&fx.project(), &facade_ctx())
    .expect("a re-run moves no format, so it runs on a dirty tree");
  assert!(
    !again.on_a_clean_tree,
    "a run that passed no guard claims nothing about the tree"
  );
}

/// **A VIEW LEFT BY `git switch` IS NAMED AS ONE, AND ITS REMEDY IS REMOVAL**
/// (issue Ie7fb84c). A branch already upgraded ignores its acceptance views,
/// so the switch leaves them on disk, untracked here, often in the directory
/// of a thread this branch does not have. "Commit or stash" was the wrong
/// remedy for a view no one wrote: the refusal names it apart from real dirt,
/// says to remove it, and writes nothing.
#[test]
fn a_view_left_by_another_branch_is_named_apart_with_removal_its_remedy() {
  use intentsvcs::remedy::Remedy;
  let fx = ordinal_estate(&[sample_thread("ST0001")]);
  let project = fx.project();
  let left = project.relative(&project.acceptance_view("ST0002"));
  fx.write_file(&left, "# Acceptance: a thread only the other branch has\n");
  let before = estate(fx.root());

  match Facade::upgrade(&project, &facade_ctx()) {
    Err(refused @ FacadeError::MigrationOverDirtyTree { .. }) => {
      let FacadeError::MigrationOverDirtyTree {
        paths,
        left_views,
        emptied,
      } = &refused
      else {
        unreachable!()
      };
      assert_eq!(
        left_views,
        &vec![left.clone()],
        "the left view is not named as one"
      );
      assert!(
        paths.is_empty(),
        "the left view was also named as work: {paths:?}"
      );
      // Its thread is not on this branch, so removing it empties the thread's
      // directory, which the conversion refuses as residue unless it goes too.
      let dir = project.relative(&project.thread_dir("ST0002"));
      assert_eq!(
        emptied,
        &vec![dir.clone()],
        "the emptied directory is not named"
      );
      let remedy = refused.remedy();
      assert!(
        remedy.contains(&format!("rm {left} && rmdir {dir}"))
          && !remedy.contains("commit or stash"),
        "the remedy must be removal and only removal: {remedy}"
      );
    }
    other => panic!("a left view must be refused and named, got {other:?}"),
  }
  assert_eq!(
    crate::common::changed(&before, &estate(fx.root())),
    Vec::<String>::new(),
    "the refused upgrade wrote"
  );
}

/// **THE FORMAT MOVE NAMES EACH BRANCH THAT CHANGED A THREAD SINCE IT SPLIT**
/// (issue I84286b7). Such a branch and this upgrade conflict add/add at merge
/// on every record whose state differs, however the branch takes the move. A
/// branch that changed no thread converts to the same bytes and merges clean,
/// so it is not named.
#[test]
fn the_format_move_names_each_branch_that_changed_a_thread_since_it_split() {
  let thread = sample_thread("ST0001");
  let fx = ordinal_estate(std::slice::from_ref(&thread));
  fx.git(&["switch", "-q", "-c", "changed-a-thread"]);
  let mut moved = thread.clone();
  moved.title = format!("{} on the branch", thread.title);
  fx.write_thread(&moved);
  fx.git_commit_all();
  fx.git(&["switch", "-q", "-"]);
  fx.git(&["switch", "-q", "-c", "changed-no-thread"]);
  fx.write_file("NOTES.md", "notes\n");
  fx.git_commit_all();
  fx.git(&["switch", "-q", "-"]);

  let done = Facade::upgrade(&fx.project(), &facade_ctx()).expect("the format move");
  assert_eq!(
    done.other_branches,
    vec!["changed-a-thread".to_string()],
    "the move must name the branch that changed a thread, and only it"
  );
}

/// **A PROJECT WITH NO IDENTITY GETS ONE DERIVED FROM ITS HISTORY, NOT A FRESH
/// ONE PER RUN** (issue Ie99dbb6). Two branches of such a project, each
/// upgraded on its own, minted two ids and conflicted on config.json at merge.
/// Run twice from the same commit, with the first run thrown away whole, as a
/// second branch would run it, the upgrade stamps the same id.
#[test]
fn an_unstamped_projects_id_is_the_same_on_every_branch_that_upgrades_it() {
  let fx = ordinal_estate(&[sample_thread("ST0001")]);
  let id = |fx: &Fixture| -> String {
    let config: serde_json::Value =
      serde_json::from_str(&fx.read("intent/.config/config.json")).expect("config.json reads");
    config["project_id"]
      .as_str()
      .expect("the upgrade stamps a project_id")
      .to_string()
  };
  assert!(
    !fx.read("intent/.config/config.json").contains("project_id"),
    "the fixture must start with no project_id, or nothing is minted"
  );
  Facade::upgrade(&fx.project(), &facade_ctx()).expect("the first upgrade");
  let first = id(&fx);
  fx.git(&["reset", "-q", "--hard"]);
  fx.git(&["clean", "-q", "-f", "-d", "-x"]);
  Facade::upgrade(&fx.project(), &facade_ctx())
    .expect("the second upgrade, as another branch runs it");
  assert_eq!(
    id(&fx),
    first,
    "two upgrades of one history stamped two identities"
  );
}
