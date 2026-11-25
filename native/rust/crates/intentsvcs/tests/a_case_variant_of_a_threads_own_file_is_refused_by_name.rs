//! Issue 0719: **A NAME A DISK FOLDING CASE STORES AS A THREAD'S OWN FILE IS
//! NEVER AN ATTACHMENT.** `Project::classify` matched the thread's own files
//! by exact string, so an author's `Acceptance.md` was carried, and on macOS
//! the next render wrote the generated view over it.
//!
//! cc's ruling: the thread's own files are matched through the one-file key,
//! no door carries, renames or claims a case variant of one, each names it by
//! its exact spelling, and `doctor` counts it.

use crate::common::{Fixture, ctx, sample_thread};
use intentsvcs::address::parse;
use intentsvcs::facade::FacadeError;
use intentsvcs::finding::FindingClass;
use intentsvcs::project::{Project, ThreadFile};
use std::path::{Path, PathBuf};

fn case_variant_of(own: &str) -> ThreadFile {
  ThreadFile::CaseVariant {
    of: PathBuf::from(own),
  }
}

/// A fixture thread with no realised views, so the author's file is the only
/// one of its key on disk whatever the disk folds.
fn thread_holding(rel: &str) -> Fixture {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  fx.write_prose("ST0001", rel, "# the author's own notes\n");
  fx
}

#[test]
fn the_threads_own_files_are_matched_through_the_one_file_key() {
  for (rel, expected) in [
    ("info.md", ThreadFile::GeneratedView),
    ("acceptance.md", ThreadFile::GeneratedView),
    ("WP/01/info.md", ThreadFile::GeneratedView),
    ("thread.json", ThreadFile::Canon),
    ("Acceptance.md", case_variant_of("acceptance.md")),
    ("INFO.md", case_variant_of("info.md")),
    ("wp/01/info.md", case_variant_of("WP/01/info.md")),
    ("WP/01/Info.md", case_variant_of("WP/01/info.md")),
    ("Thread.json", case_variant_of("thread.json")),
    ("notes.md", ThreadFile::Attachment),
    ("design/Info.md", ThreadFile::Attachment),
  ] {
    assert_eq!(Project::classify(Path::new(rel)), expected, "{rel}");
  }
}

#[test]
fn the_carry_refuses_a_case_variant_by_name_and_carries_nothing_of_it() {
  let fx = thread_holding("Acceptance.md");
  let (carried, refused) = fx.project().collect_attachments("ST0001");
  assert!(
    carried.iter().all(|a| a.path != "Acceptance.md"),
    "carried: {carried:?}"
  );
  assert!(
    refused
      .iter()
      .any(|(path, why)| path == "intent/st/ST0001/Acceptance.md"
        && why.contains("`acceptance.md`")
        && why.contains(FindingClass::CaseVariant.remedy())),
    "refused by name, naming the file it is one with and the remedy: {refused:?}"
  );
}

#[test]
fn the_attach_door_refuses_a_case_variant_by_name() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();
  let address =
    parse("intent:///threads/ST0001/attachments/Acceptance.md").expect("an attachment address");
  match facade.put_attachment(&address, b"# the author's own notes\n") {
    Err(FacadeError::CaseVariant { path, of }) => {
      assert_eq!(path, "intent/st/ST0001/Acceptance.md");
      assert_eq!(of, PathBuf::from("acceptance.md"));
    }
    other => panic!("the door refuses the case variant by name, and answered {other:?}"),
  }
  let thread = facade.st_show("ST0001").expect("the fixture thread");
  assert!(
    thread
      .attachments
      .iter()
      .all(|a| !a.path.eq_ignore_ascii_case("acceptance.md")),
    "nothing of the case variant was carried: {:?}",
    thread.attachments
  );
}

#[test]
fn doctor_counts_a_case_variant() {
  let fx = thread_holding("Acceptance.md");
  let findings =
    intentsvcs::doctor::diagnose(&fx.project(), &ctx(), None, intentsvcs::doctor::Scope::All)
      .findings;
  assert!(
    findings
      .iter()
      .any(|f| f.class == FindingClass::CaseVariant && f.file == "intent/st/ST0001/Acceptance.md"),
    "{findings:?}"
  );
}

/// The refusal every door gives: `FacadeError::CaseVariant`, naming the file
/// project-relative and the thread's own file it is one with.
fn assert_refused_by_name(answer: Result<impl std::fmt::Debug, FacadeError>) {
  match answer {
    Err(FacadeError::CaseVariant { path, of }) => {
      assert_eq!(path, "intent/st/ST0001/Acceptance.md");
      assert_eq!(of, PathBuf::from("acceptance.md"));
    }
    other => panic!("the door refuses the case variant by name, and answered {other:?}"),
  }
}

#[test]
fn the_text_put_refuses_a_case_variant_by_name() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();
  let address = parse("intent:///threads/ST0001/attachments/Acceptance.md").expect("an address");
  assert_refused_by_name(facade.put(&address, "# the author's own notes\n"));
}

#[test]
fn edit_refuses_a_case_variant_by_name() {
  let fx = thread_holding("Acceptance.md");
  let mut facade = fx.facade();
  let address = parse("intent:///threads/ST0001").expect("a thread address");
  assert_refused_by_name(facade.edit(&address, "Acceptance"));
}

#[test]
fn reading_a_case_variant_names_the_file_it_is_one_with() {
  let fx = thread_holding("Acceptance.md");
  let facade = fx.facade();
  assert_eq!(
    facade.read_thread_file("ST0001", "Acceptance.md").ok(),
    Some(intentsvcs::views::ThreadFileRead::CaseVariant {
      of: PathBuf::from("acceptance.md")
    })
  );
}

/// organize reports a case variant, and a file under a directory that names no
/// thread, as unclaimed: never dehydrated, so never removed.
#[test]
fn organize_reports_a_case_variant_and_a_file_no_thread_owns_and_claims_neither() {
  let fx = thread_holding("Acceptance.md");
  fx.write_file("intent/st/COMPLETED/ST0001/info.md", "# a v2 cover\n");
  let project = fx.project();
  let canon = intentsvcs::ingest::Canon {
    threads: vec![sample_thread("ST0001")],
    ..Default::default()
  };
  let (tree, digest) = intentsvcs::organize::observe(&project, &[]).expect("observes the tree");
  let realised = intentsvcs::intentfiles::realised_for_action("").expect("the manifest parses");
  let plan = intentsvcs::organize::plan(&project, &canon, &realised, &ctx(), &tree, digest);
  for rel in [
    "intent/st/ST0001/Acceptance.md",
    "intent/st/COMPLETED/ST0001/info.md",
  ] {
    let actions: Vec<&intentsvcs::organize::Action> = plan
      .steps
      .iter()
      .filter(|step| step.path == fx.path(rel))
      .map(|step| &step.action)
      .collect();
    assert_eq!(actions, [&intentsvcs::organize::Action::Unclaimed], "{rel}");
  }
}
