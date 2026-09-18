//! **ST0078 WP-02: `st renumber` and `issues renumber`** (AT-02.1, AT-02.2).
//!
//! Two clones mint ids highest-plus-one over their own canon, so both can mint
//! `ST0003`; git refuses the merge, and the person on the losing side moves
//! theirs before merging again. These arms drive that move through the facade:
//! what it rewrites, what it leaves and reports, and what it refuses.
//!
//! # Why the refusals read the whole tree back
//!
//! A refusal that moved a directory before refusing is indistinguishable from
//! one that did not by every observation except reading the tree, so each one
//! compares the tree before and after -- leaving out the store's own files,
//! which a read touches; what the store holds is asserted by asking it.

use crate::common::{Fixture, sample_issue, sample_thread};
use intentsvcs::facade::{Facade, FacadeError};
use intentsvcs::model::Related;
use intentsvcs::remedy::Remedy;

/// Every file in the project but the store's own.
fn files(fx: &Fixture) -> std::collections::BTreeMap<String, Vec<u8>> {
  let mut all = crate::common::tree(fx.root());
  all.retain(|path, _| !path.starts_with("intent/.cache/"));
  all
}

/// ST0001 is the thread that moves. ST0005 names it in `related` and in a
/// sentence; `notes.md` under its directory is a file nobody modelled; the
/// manifest declares both; and `ic` claims one of its packages.
fn estate() -> (Fixture, Facade) {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut other = sample_thread("ST0005");
  other.related = vec![Related {
    id: "ST0001".to_string(),
    note: None,
  }];
  other.objective = "Build on the design ST0001 settled.".to_string();
  fx.write_thread(&other);
  fx.write_prose("ST0001", "notes.md", "an unmodelled file\n");
  fx.write_file(
    "intent/.intentfiles",
    "STEELTHREAD:ST0001\nSTEELTHREAD:ST0005\n",
  );
  let mut f = fx.facade_on_disk();
  f.wb_register("ic", "Interface", "interface")
    .expect("register ic");
  f.wb_claim("ic", "ST0001/01").expect("claim a package");
  (fx, f)
}

#[test]
fn a_thread_renumber_moves_the_record_and_everything_that_names_it_structurally() {
  let (fx, mut f) = estate();
  let created = f.st_show("ST0001").expect("before").created.clone();

  let done = f.st_renumber("ST0001", "ST0003").expect("renumber");

  // The record, under its new id and nothing else changed.
  assert!(matches!(
    f.st_show("ST0001"),
    Err(FacadeError::NoSuchThread { .. })
  ));
  let moved = f.st_show("ST0003").expect("the thread under its new id");
  assert_eq!(moved.title, "Intent v3.0.0");
  assert_eq!(moved.created, created, "a renumber is not a create");
  assert_eq!(moved.attachments.len(), 2, "its attachments came with it");

  // Its canon, its directory, and the file nobody modelled.
  assert!(
    !fx.canon_path("ST0001").exists(),
    "the old canon file is gone"
  );
  assert!(fx.canon_path("ST0003").exists());
  assert!(!fx.path("intent/st/ST0001").exists());
  assert_eq!(fx.read("intent/st/ST0003/notes.md"), "an unmodelled file\n");
  let cover = fx.read("intent/st/ST0003/info.md");
  assert!(
    cover.contains("ST0003") && !cover.contains("ST0001"),
    "{cover}"
  );

  // The manifest row, the related reference and the claim.
  let manifest = fx.read("intent/.intentfiles");
  assert!(manifest.contains("STEELTHREAD:ST0003"), "{manifest}");
  assert!(!manifest.contains("STEELTHREAD:ST0001"), "{manifest}");
  let related: Vec<String> = f
    .st_show("ST0005")
    .expect("ST0005")
    .related
    .iter()
    .map(|r| r.id.clone())
    .collect();
  assert_eq!(related, vec!["ST0003".to_string()]);
  assert_eq!(
    f.store().wb_claims("ic").expect("claims"),
    vec!["ST0003/01".to_string()]
  );
  assert_eq!(
    done.rewritten,
    vec![
      "ST0005 related: ST0001 -> ST0003".to_string(),
      "ic's claims: ST0001/01 -> ST0003/01".to_string(),
    ]
  );

  // Its own event, naming both ids.
  let events = f.store().events().expect("events");
  let event = events
    .iter()
    .find(|e| e.op == "st.renumber" && e.subject.kind == "thread")
    .expect("the renumber wrote its event");
  assert_eq!(event.subject.id, "ST0003");
  assert_eq!(event.payload["from"], "ST0001");
  assert_eq!(event.payload["to"], "ST0003");

  // A store rebuilt from what is on disk agrees: nothing names the old id.
  drop(f);
  let fresh = fx.facade();
  assert!(fresh.st_show("ST0003").is_ok());
  assert!(fresh.st_show("ST0001").is_err());
}

#[test]
fn a_renumber_reports_the_prose_that_names_the_old_id_and_leaves_it() {
  let (_fx, mut f) = estate();

  let done = f.st_renumber("ST0001", "ST0003").expect("renumber");

  assert!(
    done.prose.iter().any(|m| m.path.contains("ST0005")),
    "ST0005's objective names ST0001 and must be reported: {:?}",
    done.prose
  );
  assert_eq!(
    f.st_show("ST0005").expect("ST0005").objective,
    "Build on the design ST0001 settled.",
    "prose is authored and is never rewritten"
  );
}

#[test]
fn a_renumber_onto_an_id_the_store_holds_is_refused_and_changes_nothing() {
  let (fx, mut f) = estate();
  let before = files(&fx);

  let err = f
    .st_renumber("ST0001", "ST0005")
    .expect_err("ST0005 is taken");

  assert!(
    matches!(&err, FacadeError::RenumberTargetTaken { held_by, .. } if held_by == "the store"),
    "{err}"
  );
  assert!(!err.remedy().is_empty());
  assert_eq!(files(&fx), before, "a refusal moves nothing");
  assert!(f.st_show("ST0001").is_ok());
}

#[test]
fn a_renumber_onto_an_id_only_the_tree_holds_is_refused_and_changes_nothing() {
  // A pull not yet loaded: the canon file is on disk and the store has never
  // seen it. Renumbering onto it would put two threads in one place.
  let (fx, mut f) = estate();
  fx.write_raw_thread("ST0003", "{}");
  let before = files(&fx);

  let err = f
    .st_renumber("ST0001", "ST0003")
    .expect_err("the tree holds ST0003");

  assert!(
    matches!(&err, FacadeError::RenumberTargetTaken { held_by, .. } if held_by.ends_with("ST0003.json")),
    "{err}"
  );
  assert_eq!(files(&fx), before);
}

#[cfg(unix)]
#[test]
fn a_move_the_filesystem_refuses_puts_everything_back() {
  let (fx, mut f) = estate();
  let before = files(&fx);
  let mode = fx.make_readonly("intent/st");

  let err = f.st_renumber("ST0001", "ST0003");
  fx.restore_mode("intent/st", mode);
  let err = err.expect_err("intent/st is not writable");

  assert!(matches!(err, FacadeError::RenumberDiskStep { .. }), "{err}");
  assert!(err.remedy().contains("nothing was renumbered"));
  assert_eq!(files(&fx), before, "the manifest edit is put back");
  assert!(f.st_show("ST0001").is_ok());
  assert!(f.st_show("ST0003").is_err());
}

#[test]
fn an_issue_renumber_moves_its_canon_and_view_and_reports_its_own_heading() {
  let fx = Fixture::new();
  fx.write_issue(&sample_issue(21));
  fx.write_issue(&sample_issue(30));
  let mut f = fx.facade_on_disk();
  let project = fx.project();

  let taken = f
    .issue_renumber(21, 30)
    .expect_err("issue 0030 is in the store");
  assert!(matches!(taken, FacadeError::RenumberTargetTaken { .. }));

  let done = f.issue_renumber(21, 22).expect("renumber");

  assert!(f.issue_show(21).is_err());
  let moved = f.issue_show(22).expect("the issue under its new number");
  assert_eq!(moved.title, "prune the dead mechanism");
  assert!(!project.issue_json(21).exists());
  assert!(project.issue_json(22).exists());
  assert!(!project.issue_view(21).exists());
  // The body's own `# 0021:` line is authored text: kept, and reported.
  assert!(moved.body.starts_with("# 0021:"));
  assert!(
    done.prose.iter().any(|m| m.path.contains("0022")),
    "the issue's own heading still names 0021: {:?}",
    done.prose
  );
  let events = f.store().events().expect("events");
  assert!(
    events
      .iter()
      .any(|e| e.op == "issues.renumber" && e.subject.id == "0022")
  );
}
