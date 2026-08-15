//! AT-10.7 / AC-10.7: a 2.19.0 project that has not been migrated is DETECTED
//! and NAMED, never answered from an empty model as though the estate were
//! empty.
//!
//! **This is the state every project on earth is in the first time v3 runs**,
//! and it was unspecified: AC-10.1 covers below the floor and AC-00.8/AC-10.3
//! cover the migration itself, but the state BETWEEN -- v3 installed, project
//! at 2.19.0, canon not yet written -- had no contract. Measured on a two-
//! thread fixture before the fix, all four of these were wrong at once:
//!
//! ```text
//! intent st list        exit 0, ZERO BYTES on both streams
//! intent st show ST0001 exit 1, "no steel thread ST0001 in this project"
//! intent search thing   exit 0, silent
//! intent doctor         exit 1, 2 findings, both "generated view is missing"
//! ```
//!
//! `st show` is the worst of the four and the reason this is a No Silent
//! Errors defect rather than a cosmetic one: it is not silence, it is a
//! confident FALSE STATEMENT OF FACT about a thread sitting on disk, and its
//! remedy sends the operator to `st list`, which prints nothing and thereby
//! corroborates the lie. A user's first contact with v3 is being told their
//! work does not exist.
//!
//! The detection is deliberately TWO independent signals, and the tests below
//! pin each one separately, because either alone leaves a hole:
//!
//! - the **declaration** (`config.json`'s `intent_version` below 3) catches a
//!   project with nothing in it yet, which has no evidence to find;
//! - the **evidence** (a thread directory with v2 `info.md` and no v3
//!   `thread.json`) catches a project whose config says 3.0.0 while its canon
//!   does not -- a half-migrated estate, which is strictly worse than an
//!   unmigrated one and which a version check alone would wave through.
//!
//! `project_id` is NOT the marker, though D15 makes it look like the obvious
//! candidate ("stamped at migration ... which is how the migrator tells not-
//! yet-migrated from migrated"). It is a migration-PROVENANCE stamp, and a
//! project created natively under v3 was never migrated at all, so gating on
//! it would refuse every project that never needed migrating.

mod common;

use common::{Fixture, facade_ctx};
use intentsvcs::facade::{Facade, FacadeError};
use intentsvcs::project::{Migration, Project};

/// A project whose config declares `version`, with `threads` in v2 shape:
/// `info.md` and no `thread.json`, which is exactly what v2.19.0 leaves on
/// disk.
fn legacy(version: &str, threads: &[&str]) -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let config = dir.path().join("intent").join(".config");
  std::fs::create_dir_all(&config).expect("mkdir .config");
  std::fs::write(
    config.join("config.json"),
    format!(
      "{{\n  \"intent_version\": \"{version}\",\n  \"project_name\": \"Legacy\",\n  \"author\": \"matts\",\n  \"intent_dir\": \"intent\",\n  \"languages\": [\"shell\"]\n}}\n"
    ),
  )
  .expect("write config");
  for id in threads {
    let td = dir.path().join("intent").join("st").join(id);
    std::fs::create_dir_all(&td).expect("mkdir thread");
    std::fs::write(
      td.join("info.md"),
      format!("---\nstatus: In Progress\n---\n\n# {id}: a real thread\n"),
    )
    .expect("write v2 info.md");
  }
  dir
}

/// `expect_err` needs `T: Debug`, and a `Facade` holds a live SQLite handle
/// that has no business being printable. The panic message carries the same
/// information.
fn refusal(opened: Result<Facade, FacadeError>) -> FacadeError {
  match opened {
    Ok(_) => panic!("opening an unmigrated project must refuse, and it did not"),
    Err(e) => e,
  }
}

fn pending(project: &Project) -> intentsvcs::project::Pending {
  match project.migration() {
    Migration::Pending(p) => p,
    Migration::Done => panic!("expected this project to read as unmigrated"),
  }
}

// ---------------------------------------------------------------------------
// Detection
// ---------------------------------------------------------------------------

/// Signal 1: the project's own declaration, with no evidence to corroborate
/// it. An empty 2.19.0 project has nothing on disk to find, and is still not a
/// v3 project -- migration is what stamps the version.
#[test]
fn a_2_19_0_project_with_no_threads_at_all_is_still_unmigrated() {
  let dir = legacy("2.19.0", &[]);
  let project = Project::open(dir.path()).expect("open");
  let p = pending(&project);
  assert_eq!(p.declared, "2.19.0");
  assert!(
    p.legacy_threads.is_empty(),
    "there is nothing on disk to find"
  );
  assert!(!p.below_floor, "2.19.0 IS the floor, not below it");
}

/// Signal 2: the evidence, against a config that claims otherwise.
///
/// **This is the discriminating case.** A detector that only read the declared
/// version would pass every other test in this file and wave this one through
/// -- and this is the worse estate of the two, because half of it is v3 canon
/// the tool will happily answer from while the other half is invisible.
#[test]
fn a_config_claiming_3_0_0_over_v2_canon_is_still_unmigrated() {
  let dir = legacy("3.0.0", &["ST0001"]);
  let project = Project::open(dir.path()).expect("open");
  let p = pending(&project);
  assert_eq!(
    p.declared, "3.0.0",
    "the declaration is reported as it stands"
  );
  assert_eq!(
    p.legacy_threads,
    vec!["ST0001".to_string()],
    "the evidence is what convicts here"
  );
}

/// THE CONTROL. It must stay green, and it is what stops the detector
/// degenerating into "always pending" -- which would pass every assertion
/// above while bricking the tool.
#[test]
fn a_real_v3_project_is_not_flagged() {
  let fixture = Fixture::new();
  let facade = fixture.facade();
  drop(facade);
  assert!(
    matches!(fixture.project().migration(), Migration::Done),
    "a 3.0.0 project with no v2 canon is migrated"
  );
}

/// The second control: a v3 project with real threads, written by v3 itself.
///
/// v3 renders `info.md` as a generated VIEW, so every migrated thread has both
/// `info.md` and `thread.json` -- the evidence rule keys on `thread.json`
/// being ABSENT, and a rule that merely looked for `info.md` would flag every
/// healthy project in existence.
#[test]
fn a_v3_thread_carrying_a_generated_info_md_is_not_evidence() {
  let fixture = Fixture::new();
  let mut facade = fixture.facade();
  facade.st_new("A thread").expect("st new");
  drop(facade);

  let project = fixture.project();
  assert!(
    project.info_view("ST0001").is_file(),
    "v3 writes info.md as a generated view -- the premise of this test"
  );
  assert!(
    matches!(project.migration(), Migration::Done),
    "info.md beside thread.json is a healthy v3 thread, not v2 residue"
  );
}

/// v2's `st done` RELOCATES a thread to `st/<STATUS>/<ID>/`, so a scan that
/// only looked at `st/`'s immediate children would see the open threads and
/// miss the entire archive.
///
/// The case this protects is NOT the one it looks like. A wholly unmigrated
/// project is caught by its declared version regardless -- which is how this
/// hole survived being written, since the repository it was measured on
/// reported correctly while missing 55 of its 56 threads. The case that fails
/// is a project whose live threads are migrated and whose archive is not: the
/// declaration says 3.0.0, the shallow scan finds nothing, and 52 completed
/// threads quietly stop existing.
#[test]
fn v2_threads_relocated_into_a_status_directory_are_found() {
  let dir = legacy("3.0.0", &[]);
  for (status, id) in [("COMPLETED", "ST0001"), ("CANCELLED", "ST0002")] {
    let td = dir.path().join("intent").join("st").join(status).join(id);
    std::fs::create_dir_all(&td).expect("mkdir archived thread");
    std::fs::write(
      td.join("info.md"),
      "---\nstatus: Completed\n---\n\n# archived\n",
    )
    .expect("write v2 info.md");
  }

  let project = Project::open(dir.path()).expect("open");
  let p = pending(&project);
  assert_eq!(
    p.legacy_threads,
    vec!["ST0001".to_string(), "ST0002".to_string()],
    "the archive is canon too, and it is where most of a mature project lives"
  );
}

// ---------------------------------------------------------------------------
// Every command that reads the model
// ---------------------------------------------------------------------------

/// The facade is the one surface every skin calls (D06), so refusing here is
/// what makes "every command" true rather than a list someone has to maintain.
#[test]
fn opening_the_facade_refuses_and_names_what_it_found() {
  let dir = legacy("2.19.0", &["ST0001", "ST0002"]);
  let project = Project::open(dir.path()).expect("open");
  let err = refusal(Facade::open(project, facade_ctx()));

  let rendered = match &err {
    FacadeError::Unmigrated(_) => err.render(),
    other => panic!("expected an Unmigrated refusal, got {other:?}"),
  };

  assert!(rendered.starts_with("error: "), "v2's voice: {rendered}");
  assert!(
    rendered.contains("has not been migrated"),
    "it says WHAT is wrong: {rendered}"
  );
  assert!(
    rendered.contains("2.19.0"),
    "it names the version the project declares: {rendered}"
  );
  assert!(
    rendered.contains("ST0001"),
    "it names the work it cannot see, so the operator can tell this from an \
     empty project: {rendered}"
  );
  assert!(
    rendered.contains("remedy: ") && rendered.contains("intent upgrade"),
    "and what to do about it: {rendered}"
  );
}

/// The in-memory path is a second door into the same house. It was already
/// possible to open one and not the other, which is the shape of gap that
/// makes a guard true of the code someone read and false of the code they ran.
#[test]
fn the_in_memory_facade_refuses_identically() {
  let dir = legacy("2.19.0", &["ST0001"]);
  let project = Project::open(dir.path()).expect("open");
  let err = refusal(Facade::open_in_memory(project, facade_ctx()));
  assert!(matches!(err, FacadeError::Unmigrated(_)), "{err:?}");
}

/// Below the v2.19.0 floor the remedy is DIFFERENT -- the two-hop (D09), not a
/// v3 migration that cannot run. One remedy for both would be telling half the
/// operators who read it to run something that will refuse them.
#[test]
fn below_the_floor_the_remedy_is_the_two_hop_not_the_v3_migrator() {
  let dir = legacy("2.11.0", &["ST0001"]);
  let project = Project::open(dir.path()).expect("open");
  let p = pending(&project);
  assert!(p.below_floor, "2.11.0 is below the 2.19.0 floor");

  let err = refusal(Facade::open(project, facade_ctx()));
  let rendered = err.render();
  assert!(
    rendered.contains("2.19.0"),
    "the floor is named, not just missed: {rendered}"
  );
  assert!(
    rendered.contains("intent@2"),
    "the two-hop instruction names the v2 tool: {rendered}"
  );

  let at_floor = legacy("2.19.0", &["ST0001"]);
  let at_floor_remedy = refusal(Facade::open(
    Project::open(at_floor.path()).expect("open"),
    facade_ctx(),
  ))
  .render();
  assert_ne!(
    rendered, at_floor_remedy,
    "the two states must not read identically -- that is the whole point"
  );
}

// ---------------------------------------------------------------------------
// doctor
// ---------------------------------------------------------------------------

/// The false RED. Before the fix, doctor reported two view-skew findings and
/// "0 thread(s)" -- both artefacts of rendering an EMPTY model against real
/// files -- so the operator read it as v3 declaring their generated views
/// broken, and the honest remedy (regenerate them) would have been actively
/// harmful.
#[test]
fn doctor_reports_the_migration_and_nothing_downstream_of_it() {
  let dir = legacy("2.19.0", &["ST0001", "ST0002"]);
  let project = Project::open(dir.path()).expect("open");
  let report = Facade::doctor(&project, &facade_ctx(), None);

  assert_eq!(
    report.findings.len(),
    1,
    "exactly one finding: everything else is downstream of this one and would \
     send the operator after the wrong thing -- {:#?}",
    report.findings
  );
  let finding = &report.findings[0];
  assert_eq!(finding.class.as_str(), "unmigrated");
  assert!(
    finding.detail.contains("2.19.0"),
    "the finding names the declared version: {finding:?}"
  );
  assert!(
    !report.is_healthy() && report.exit_code() == 1,
    "an unmigrated project is not healthy"
  );
}

/// And the same control on the doctor side: a healthy v3 project must not
/// acquire a phantom migration finding.
#[test]
fn doctor_does_not_invent_a_migration_on_a_v3_project() {
  let fixture = Fixture::new();
  let mut facade = fixture.facade();
  facade.st_new("A thread").expect("st new");
  drop(facade);

  let report = Facade::doctor(&fixture.project(), &facade_ctx(), None);
  assert!(
    !report
      .findings
      .iter()
      .any(|f| f.class.as_str() == "unmigrated"),
    "{:#?}",
    report.findings
  );
}
