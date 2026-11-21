//! ST0096 WP-01: realisation lives in the record, and the manifest goes (R1).
//!
//! In a project in the release's canon format, whether a thread or an issue is
//! on disk is its own record's `realised`. So every verb in the census of
//! ST0096's design writes only the record it changes: that record's canon
//! file, its events and its realised views (AC-01.1). And `intent upgrade`
//! folds `.intentfiles` into the records and removes it, byte for byte the
//! same on two copies upgraded apart (AC-01.2).

use crate::common::{Fixture, facade_ctx, issue, sample_issue, sample_thread};
use intentsvcs::facade::{Facade, Written};
use intentsvcs::model::{AcKind, AtKind, TShirt};
use intentsvcs::project::Project;
use std::collections::BTreeMap;

/// The record a verb changes: a thread or an issue.
enum Record<'a> {
  Thread(&'a str),
  Issue(intentsvcs::id::IssueId),
}

/// Every path a verb wrote or removed that is not its own record's canon file,
/// one of its events or one of its realised views.
///
/// The two generated lists, `steel_threads.md` and `todo.md`, are aggregate
/// views every clone renders from its own store; ST0096 WP-02 takes them out
/// of git, so they are not this package's to judge.
fn outside(project: &Project, record: &Record<'_>, written: &Written) -> Vec<String> {
  let rel = |p: std::path::PathBuf| project.relative(&p);
  let lists = [rel(project.steel_threads_view()), rel(project.todo_view())];
  let events = rel(project.events_dir());
  // A thread's canon is its own file and its records' files (ST0096 WP-05),
  // so its records directory is its record too.
  let (canon, home, records) = match record {
    Record::Thread(id) => (
      rel(project.thread_json(id)),
      rel(project.thread_dir(id)),
      Some(format!(
        "{}/",
        rel(
          project
            .intent_dir()
            .join(intentsvcs::project::canon_records_rel(id))
        )
      )),
    ),
    Record::Issue(number) => (
      rel(project.issue_json(*number)),
      rel(project.issue_view(*number)),
      None,
    ),
  };
  written
    .wrote
    .iter()
    .chain(&written.removed)
    .filter(|path| {
      !(**path == canon
        || path.starts_with(&home)
        || records
          .as_ref()
          .is_some_and(|dir| path.starts_with(dir.as_str()))
        || path.starts_with(&events)
        || lists.contains(path))
    })
    .cloned()
    .collect()
}

fn realised_thread(f: &Facade, id: &str) -> bool {
  f.st_show(id).expect("the thread").realised
}

/// AC-01.1, over every verb in the census table of ST0096's design.
// ST0096 AT-01.1: the arm below.
#[test]
fn every_census_verb_writes_only_the_record_it_changes() {
  let fx = Fixture::new();
  let mut f = fx.facade();
  let project = fx.project();
  let mut strays: Vec<String> = Vec::new();
  let mut check = |verb: &str, record: Record<'_>, f: &mut Facade| {
    let written = f.take_landed();
    strays.extend(
      outside(&project, &record, &written)
        .into_iter()
        .map(|path| format!("`{verb}` wrote {path}")),
    );
  };

  let st = f.st_new("a thread of its own").expect("st new");
  check("st new", Record::Thread(&st), &mut f);
  assert!(!realised_thread(&f, &st), "`st new` declares nothing");
  f.st_start(&st).expect("st start");
  check("st start", Record::Thread(&st), &mut f);
  assert!(realised_thread(&f, &st), "`st start` realises its thread");
  f.st_hold(&st, "waiting on a peer").expect("st hold");
  check("st hold", Record::Thread(&st), &mut f);
  assert!(!realised_thread(&f, &st), "`st hold` unrealises its thread");
  f.st_resume(&st).expect("st resume");
  check("st resume", Record::Thread(&st), &mut f);
  assert!(realised_thread(&f, &st), "`st resume` realises its thread");

  // The contract first, so the closing gates of `wp done` and `st done` pass.
  f.ac_new(&st, "AC-01.1", "a criterion", AcKind::NonTest)
    .expect("ac new");
  check("ac new", Record::Thread(&st), &mut f);
  f.ac_satisfy(&st, "AC-01.1", "the evidence")
    .expect("ac satisfy");
  check("ac satisfy", Record::Thread(&st), &mut f);
  f.ac_new(&st, "AC-01.2", "a tested criterion", AcKind::Test)
    .expect("ac new");
  f.take_landed();
  f.at_new(
    &st,
    "AT-01.1",
    AtKind::Test,
    None,
    None,
    vec!["AC-01.2".to_string()],
    None,
  )
  .expect("at new");
  check("at new", Record::Thread(&st), &mut f);
  // Withdrawn, so its unwritten test does not block the close.
  f.ac_withdraw(&st, "AC-01.2", "the census needs no test", None)
    .expect("ac withdraw");
  f.take_landed();

  let wp = f.wp_new(&st, "a package", TShirt::S).expect("wp new");
  check("wp new", Record::Thread(&st), &mut f);
  f.wp_start(&st, wp).expect("wp start");
  check("wp start", Record::Thread(&st), &mut f);
  f.wp_done(&st, wp).expect("wp done");
  check("wp done", Record::Thread(&st), &mut f);

  let address = intentsvcs::address::promote(&st).expect("an address");
  f.dehydrate(&address).expect("st dehydrate");
  check("st dehydrate", Record::Thread(&st), &mut f);
  assert!(
    !realised_thread(&f, &st),
    "`st dehydrate` unrealises its thread"
  );
  f.hydrate(&address).expect("st hydrate");
  check("st hydrate", Record::Thread(&st), &mut f);
  assert!(realised_thread(&f, &st), "`st hydrate` realises its thread");
  f.st_done(&st).expect("st done");
  check("st done", Record::Thread(&st), &mut f);
  assert!(!realised_thread(&f, &st), "`st done` unrealises its thread");

  let number = f
    .issue_add("an issue of its own", None, None, "")
    .expect("issues add");
  check("issues add", Record::Issue(number), &mut f);
  assert!(
    f.issue_show(number).expect("the issue").realised,
    "`issues add` realises its issue"
  );
  f.issue_close(number).expect("issues close");
  check("issues close", Record::Issue(number), &mut f);
  assert!(
    !f.issue_show(number).expect("the issue").realised,
    "`issues close` unrealises its issue"
  );

  assert!(
    strays.is_empty(),
    "a verb wrote outside the record it changes:\n{}",
    strays.join("\n")
  );
}

/// Every canon record and `project.json`, by path, as the upgrade wrote them.
fn canon_of(project: &Project) -> BTreeMap<String, String> {
  let mut files = BTreeMap::new();
  for dir in [project.canon_st_dir(), project.issues_dir()] {
    for entry in std::fs::read_dir(&dir)
      .expect("a canon directory")
      .flatten()
    {
      let path = entry.path();
      if path.extension().is_some_and(|x| x == "json") {
        let text = std::fs::read_to_string(&path).expect("a canon file");
        files.insert(project.relative(&path), text);
      }
    }
  }
  let state = project.project_json();
  files.insert(
    project.relative(&state),
    std::fs::read_to_string(&state).expect("the upgrade writes project.json"),
  );
  files
}

/// AC-01.2: the fold maps the manifest exactly, removes it, and two copies
/// upgraded apart write the same bytes, so two branches upgraded apart merge
/// clean.
// ST0096 AT-01.2: the arm below.
#[test]
fn the_upgrade_folds_the_manifest_into_the_records_and_removes_it() {
  const LISTED: &str = "ST0002";
  const UNLISTED: &str = "ST0001";
  const LISTED_ISSUE: u32 = 7;
  const UNLISTED_ISSUE: u32 = 8;
  let copies = [Fixture::ordinal(), Fixture::ordinal()];
  for fx in &copies {
    fx.write_thread(&sample_thread(UNLISTED));
    fx.write_thread(&sample_thread(LISTED));
    fx.write_issue(&sample_issue(LISTED_ISSUE));
    fx.write_issue(&sample_issue(UNLISTED_ISSUE));
    let project = fx.project();
    std::fs::write(
      project.intentfiles_path(),
      format!(
        "STEELTHREAD:{LISTED}\nISSUE:{}\n",
        issue(LISTED_ISSUE).stored()
      ),
    )
    .expect("a manifest");
    let upgraded = Facade::upgrade(&project, &facade_ctx()).expect("the upgrade");
    assert_eq!(
      upgraded.manifest_folded.as_deref(),
      Some(project.intentfiles_path().as_path()),
      "the upgrade did not name the manifest it removed"
    );
    assert!(
      !project.intentfiles_path().exists(),
      "the upgrade left the manifest"
    );
  }
  let [a, b] = &copies;
  assert_eq!(
    canon_of(&a.project()),
    canon_of(&b.project()),
    "two copies upgraded apart wrote different canon"
  );
  let f = a.facade();
  assert!(
    realised_thread(&f, LISTED),
    "a listed thread was not folded in"
  );
  assert!(
    !realised_thread(&f, UNLISTED),
    "an unlisted thread was folded in as realised"
  );
  assert!(
    f.issue_show(issue(LISTED_ISSUE)).expect("there").realised,
    "a listed issue was not folded in"
  );
  assert!(
    !f.issue_show(issue(UNLISTED_ISSUE)).expect("there").realised,
    "an unlisted issue was folded in as realised"
  );
}

/// A manifest that cannot be read is refused by name, and nothing is written:
/// the fold removes the file, so it must know what the file declares.
#[test]
fn the_upgrade_refuses_a_manifest_it_cannot_open() {
  let fx = Fixture::ordinal();
  fx.write_thread(&sample_thread("ST0001"));
  let project = fx.project();
  // A directory where the manifest belongs: present, and not a readable file.
  std::fs::create_dir(project.intentfiles_path()).expect("a directory in the manifest's place");
  let refused = Facade::upgrade(&project, &facade_ctx()).expect_err("an unopenable manifest");
  assert!(
    refused.to_string().contains(".intentfiles")
      && refused.to_string().contains("could not be read"),
    "the refusal does not name the manifest it could not read: {refused}"
  );
  assert!(
    project.intentfiles_path().is_dir(),
    "a refused upgrade removed what stood in the manifest's place"
  );
  assert_eq!(
    intentsvcs::ingest::canon_format(&project).expect("the format"),
    intentsvcs::canon_format::CanonFormat::V3,
    "a refused upgrade moved the format"
  );
}

/// A manifest that will not parse is refused by name, and nothing is written:
/// the fold removes the file, so a row it could not read would be lost.
#[test]
fn the_upgrade_refuses_a_manifest_it_cannot_read() {
  let fx = Fixture::ordinal();
  fx.write_thread(&sample_thread("ST0001"));
  let project = fx.project();
  let manifest = "NOT-A-SIGIL:ST0001\n";
  std::fs::write(project.intentfiles_path(), manifest).expect("a manifest");
  let refused = Facade::upgrade(&project, &facade_ctx()).expect_err("an unreadable manifest");
  assert!(
    refused.to_string().contains(".intentfiles"),
    "the refusal does not name the manifest: {refused}"
  );
  assert_eq!(
    std::fs::read_to_string(project.intentfiles_path()).expect("the manifest stays"),
    manifest,
    "a refused upgrade changed the manifest"
  );
  assert_eq!(
    intentsvcs::ingest::canon_format(&project).expect("the format"),
    intentsvcs::canon_format::CanonFormat::V3,
    "a refused upgrade moved the format"
  );
}
