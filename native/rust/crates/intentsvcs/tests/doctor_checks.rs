//! AT-06.2 / AC-06.2: doctor as model/DB integrity queries plus file checks,
//! including the skew check and unparsed-state reporting.
//!
//! **Every model check gets its own fixture.** One estate carrying all seven
//! faults would pass a "doctor found something" assertion while six of the
//! seven checks were dead, and a dead check is indistinguishable from a clean
//! bill of health. The all-at-once case is asserted too, but for a different
//! property: that the report does not stop at the first finding.
//!
//! The load-bearing test in this file is
//! `doctor_runs_on_a_project_that_cannot_be_opened`. Doctor going through the
//! normal open path is not a small bug: a duplicate criterion id trips a
//! UNIQUE constraint during the DB load, so the command died with a SQLite
//! message before reporting anything -- while the tool's own remedy advised
//! running `intent doctor`. A doctor that only works on healthy projects is
//! not a doctor.

mod common;

use common::{Fixture, VERSION, ctx};
use intentsvcs::finding::{Finding, FindingClass};
use intentsvcs::model::{
  AcKind, AcState, AcceptanceMode, AcceptanceTest, AtKind, AtStatus, Criterion, THREAD_SCHEMA,
  TShirt, Thread, ThreadStatus, WorkPackage, WpStatus,
};

/// A thread with nothing wrong with it: every reference resolves, and it names
/// no related thread, because the shared `sample_thread` cites ST0043/ST0044
/// which do not exist in a one-thread fixture.
fn clean_thread(id: &str) -> Thread {
  Thread {
    attachments: Vec::new(),
    body: String::new(),
    preamble: String::new(),
    schema: THREAD_SCHEMA.to_string(),
    id: id.to_string(),
    title: "A clean thread".to_string(),
    slug: None,
    status: ThreadStatus::Wip,
    status_reason: None,
    created: "2026-08-14".to_string(),
    completed: None,
    acceptance: None,
    objective: "Be internally consistent.".to_string(),
    context: String::new(),
    related: vec![],
    wps: vec![WorkPackage {
      seq: 1,
      title: "The only package".to_string(),
      scope: Some(TShirt::S),
      scope_legacy: None,
      // **`Done`, NOT `Wip`, AND THE FIXTURE'S NAME IS THE ARGUMENT.** This
      // thread's only criterion is satisfied by a green AT, so its gate PASSES
      // -- and a work package recorded WIP over a passing gate is exactly what
      // hv ratified `doctor` should report (data-model.md:472). A fixture
      // called `clean_thread` that carries a live disagreement is a fixture
      // whose name is false, and every test building on it would have been
      // asserting health over an estate that is not healthy.
      //
      // Found by the arm firing on it, which is the fixture being tested by
      // the check rather than the check being tested by the fixture.
      status: WpStatus::Done,
      status_reason: None,
      objective: String::new(),
      body: String::new(),
      preamble: String::new(),
    }],
    criteria: vec![Criterion {
      id: "AC-01.1".to_string(),
      text: "the thing works".to_string(),
      kind: AcKind::Test,
      state: AcState::Computed,
    }],
    tests: vec![AcceptanceTest {
      id: "AT-01.1".to_string(),
      kind: AtKind::Test,
      file: Some("tests/clean.rs".to_string()),
      prose: None,
      covers: vec!["AC-01.1".to_string()],
      status: AtStatus::Green,
      note: None,
      legacy: None,
    }],
  }
}

/// Write a thread, then bring the generated views into step so a test about
/// MODEL faults is not also reporting skew it created itself.
fn seed(fx: &Fixture, thread: &Thread) {
  fx.write_thread(thread);
  let project = fx.project();
  let canon = intentsvcs::ingest::read(&project).expect("fixture canon reads");
  intentsvcs::views::write_all(&project, &canon, &ctx()).expect("write views");
}

fn run(fx: &Fixture) -> Vec<Finding> {
  intentsvcs::doctor::diagnose(&fx.project(), &ctx(), None).findings
}

/// The detail texts, for asserting that a specific check fired.
fn details(findings: &[Finding]) -> String {
  findings
    .iter()
    .map(std::string::ToString::to_string)
    .collect::<Vec<_>>()
    .join("\n")
}

// ---------------------------------------------------------------------------
// The clean bill of health -- the control for every test below
// ---------------------------------------------------------------------------

#[test]
fn a_consistent_project_reports_nothing() {
  let fx = Fixture::new();
  seed(&fx, &clean_thread("ST0001"));

  let report = intentsvcs::doctor::diagnose(&fx.project(), &ctx(), None);
  assert!(
    report.is_healthy(),
    "a clean estate must report nothing, or every finding below is noise: {}",
    details(&report.findings)
  );
  assert_eq!(report.exit_code(), 0);
  assert_eq!(report.threads_checked, 1);
  assert!(
    report.views_checked > 0 && report.files_checked > 0,
    "a clean report must say what it COVERED -- 'nothing found' over an estate nobody read is the same sentence as a real pass: {report:?}"
  );
}

// ---------------------------------------------------------------------------
// Model integrity, one fixture per check
// ---------------------------------------------------------------------------

#[test]
fn an_acceptance_test_covering_a_criterion_that_does_not_exist_is_found() {
  let fx = Fixture::new();
  let mut thread = clean_thread("ST0001");
  thread.tests[0].covers = vec!["AC-99.9".to_string()];
  seed(&fx, &thread);

  let found = details(&run(&fx));
  assert!(
    found.contains("AT-01.1 covers AC-99.9"),
    "the gate reads `covers` to decide satisfaction, so a typo'd id is a test that proves nothing while looking like one that does: {found}"
  );
}

#[test]
fn an_acceptance_test_covering_nothing_is_found() {
  let fx = Fixture::new();
  let mut thread = clean_thread("ST0001");
  thread.tests[0].covers = vec![];
  seed(&fx, &thread);

  assert!(details(&run(&fx)).contains("covers nothing"));
}

#[test]
fn a_duplicate_criterion_id_within_a_thread_is_found() {
  let fx = Fixture::new();
  let mut thread = clean_thread("ST0001");
  let mut dup = thread.criteria[0].clone();
  dup.text = "a second criterion wearing the same id".to_string();
  // The kind is flipped so the two rows are TELLABLE APART in the finding, and
  // the state moves with it: since the cross-field clause landed on the schema
  // face, `(non-test, computed)` is refused at ingest -- so leaving the state
  // behind would fail this test at a gate that has nothing to do with
  // duplicate ids.
  dup.kind = AcKind::NonTest;
  dup.state = AcState::Unsatisfied;
  thread.criteria.push(dup);
  seed(&fx, &thread);

  let findings = run(&fx);
  assert!(
    findings
      .iter()
      .any(|f| f.class == FindingClass::DuplicateId),
    "the schema constrains each element, never the collection, so a repeated id validates fine: {}",
    details(&findings)
  );
}

#[test]
fn a_criterion_in_a_group_with_no_work_package_is_found() {
  let fx = Fixture::new();
  let mut thread = clean_thread("ST0001");
  thread.criteria[0].id = "AC-07.1".to_string();
  thread.tests[0].covers = vec!["AC-07.1".to_string()];
  seed(&fx, &thread);

  assert!(details(&run(&fx)).contains("AC-07.1 belongs to WP-07"));
}

/// **A thread that uses NO work packages at all is exempt, and the two threads
/// in this test differ by exactly one thing.**
///
/// Grouping criteria BY work package is a convention used where work packages
/// exist. In a thread that has none, the group number is a bare grouping
/// device: `AC-07.1` never referenced a WP-07, because there was never one to
/// reference. Six threads in Intent's own estate are built that way and carry
/// 72 such rows between them (measured on the hoisted repo, 2026-08-18); v2
/// accepted every one.
///
/// **The control is the point and it comes from the same fixture.** The
/// with-WP arm below is `clean_thread` unchanged, which carries one work
/// package at seq 1; the without-WP arm is the same thread with `wps` cleared
/// and nothing else touched. One variable. Without it, "no finding" would be
/// satisfied by a fixture that could not have produced one -- and this file
/// already has a test asserting the finding IS raised, so an exemption test
/// that never reached the check would agree with it and mean nothing.
#[test]
fn a_thread_with_no_work_packages_at_all_is_exempt_while_one_that_uses_them_is_not() {
  // Arm A: work packages exist, and a group naming a missing one is a real
  // inconsistency. This is the behaviour the clause must NOT remove.
  let fx = Fixture::new();
  let mut with_wps = clean_thread("ST0001");
  with_wps.criteria[0].id = "AC-07.1".to_string();
  with_wps.tests[0].covers = vec!["AC-07.1".to_string()];
  seed(&fx, &with_wps);
  assert!(
    details(&run(&fx)).contains("AC-07.1 belongs to WP-07"),
    "a thread that USES work packages must still be held to them"
  );

  // Arm B: the same thread with its work packages removed and nothing else
  // changed. The group number is now a grouping device, not a reference.
  let fx = Fixture::new();
  let mut no_wps = clean_thread("ST0001");
  no_wps.criteria[0].id = "AC-07.1".to_string();
  no_wps.tests[0].covers = vec!["AC-07.1".to_string()];
  no_wps.wps.clear();
  seed(&fx, &no_wps);
  let found = details(&run(&fx));
  assert!(
    !found.contains("belongs to WP-"),
    "a thread with no work packages cannot have a criterion orphaned from one: {found}"
  );
}

#[test]
fn a_thread_level_group_is_not_mistaken_for_a_missing_work_package() {
  let fx = Fixture::new();
  let mut thread = clean_thread("ST0001");
  thread.criteria[0].id = "AC-00.1".to_string();
  thread.tests[0].covers = vec!["AC-00.1".to_string()];
  seed(&fx, &thread);

  let found = details(&run(&fx));
  assert!(
    !found.contains("WP-00"),
    "group 00 is thread-level and always legitimate: {found}"
  );
}

#[test]
fn a_test_backed_criterion_carrying_stored_satisfaction_is_found() {
  let fx = Fixture::new();
  let mut thread = clean_thread("ST0001");
  thread.criteria[0].state = AcState::Satisfied {
    evidence: "hand-authored on a test-backed AC".to_string(),
  };
  // NOT `seed`, which ingests -- and the reason is the point of this test now.
  // The cross-field clause on the schema face means this estate no longer
  // LOADS, so doctor reports it from the ingest gate rather than from the model
  // check. That is a strictly earlier and louder diagnosis of the same fault,
  // and asserting the old finding text here would have quietly become a test
  // that the schema clause does NOT work.
  fx.write_thread(&thread);

  let found = details(&run(&fx));
  assert!(
    found.contains("/criteria/0/state"),
    "doctor still diagnoses it, and names the criterion: {found}"
  );
  assert!(
    found.contains("satisfied"),
    "and names the state canon must not carry on a test-backed criterion: {found}"
  );
}

/// **And the model check that used to catch it is still live, because one
/// producer of the mismatch does not come through the schema.**
///
/// This is the question the test above raises and must not leave hanging: if
/// ingest refuses the pair, is `doctor`'s kind/state check now dead code? No --
/// the migration reader (WP-10) is deliberately lenient where ingest is strict,
/// so a v2 estate whose AC carried a satisfaction flag with no `(non-test)`
/// marker arrives as exactly this pair, having never met the schema. Doctor is
/// the right instrument there, and deleting the check as "unreachable" would
/// remove the only thing watching that road.
///
/// Asserted against the model directly rather than through a file, because a
/// file is precisely what this pair can no longer be.
#[test]
fn the_model_check_still_reports_the_pair_for_the_paths_that_bypass_the_schema() {
  use intentsvcs::model::AcState;

  let satisfied_on_a_test_ac = AcState::Satisfied {
    evidence: "carried from a v2 estate".to_string(),
  };
  assert!(
    !satisfied_on_a_test_ac.permitted_for(AcKind::Test),
    "the model still knows the pair is wrong"
  );
  assert!(
    !AcState::Computed.permitted_for(AcKind::NonTest),
    "and knows it in both directions"
  );
  assert!(
    AcState::Descoped {
      to: "ST0002".to_string(),
      by: None,
      reason: None,
    }
    .permitted_for(AcKind::Test),
    "while a scope decision stays legal on both kinds -- it is a decision about \
     the requirement, and no test status recomputes one"
  );
}

#[test]
fn a_criterion_descoped_to_a_thread_that_does_not_exist_is_found() {
  let fx = Fixture::new();
  let mut thread = clean_thread("ST0001");
  thread.criteria[0].state = AcState::Descoped {
    to: "ST4242".to_string(),
    by: None,
    reason: None,
  };
  seed(&fx, &thread);

  let found = details(&run(&fx));
  assert!(
    found.contains("descoped to ST4242"),
    "a requirement moved somewhere that does not exist is held by nobody: {found}"
  );
}

#[test]
fn a_related_thread_that_does_not_exist_is_found() {
  let fx = Fixture::new();
  let mut thread = clean_thread("ST0001");
  thread.related = vec![intentsvcs::model::Related {
    id: "ST9999".to_string(),
    note: None,
  }];
  seed(&fx, &thread);

  assert!(details(&run(&fx)).contains("names ST9999 as related"));
}

#[test]
fn completion_and_status_must_agree_in_both_directions() {
  let fx = Fixture::new();
  let mut thread = clean_thread("ST0001");
  thread.status = ThreadStatus::Completed;
  thread.completed = None;
  seed(&fx, &thread);
  assert!(
    details(&run(&fx)).contains("Completed with no completion date"),
    "direction one"
  );

  let fx2 = Fixture::new();
  let mut other = clean_thread("ST0001");
  other.status = ThreadStatus::Wip;
  other.completed = Some("2026-08-14".to_string());
  seed(&fx2, &other);
  assert!(
    details(&run(&fx2)).contains("carries a completion date"),
    "direction two -- checking only the first would let a WIP thread claim a completion date"
  );
}

// ---------------------------------------------------------------------------
// File checks
// ---------------------------------------------------------------------------

#[test]
fn a_hand_edited_generated_view_is_reported_as_skew() {
  let fx = Fixture::new();
  seed(&fx, &clean_thread("ST0001"));
  let view = fx.path("intent/st/ST0001/info.md");
  let edited = format!(
    "{}\n<!-- a hand edit -->\n",
    fx.read("intent/st/ST0001/info.md")
  );
  std::fs::write(&view, edited).expect("hand-edit the view");

  let findings = run(&fx);
  assert!(
    findings.iter().any(|f| f.class == FindingClass::ViewSkew),
    "a hand edit is CAUGHT, never silently outvoted: {}",
    details(&findings)
  );
}

#[test]
fn a_file_with_conflict_markers_is_reported_unparsed() {
  let fx = Fixture::new();
  seed(&fx, &clean_thread("ST0001"));
  fx.write_prose(
    "ST0001",
    "design.md",
    "# Design\n\n<<<<<<< HEAD\nours\n=======\ntheirs\n>>>>>>> branch\n",
  );

  let findings = run(&fx);
  assert!(
    findings
      .iter()
      .any(|f| f.class == FindingClass::ConflictMarkers),
    "v2 grepped straight through conflict markers; v3 reports them: {}",
    details(&findings)
  );
}

// ---------------------------------------------------------------------------
// Robustness -- the reason this command exists
// ---------------------------------------------------------------------------

/// **The load-bearing test.** Doctor must work on a project nothing else can
/// open, because that is exactly when someone reaches for it.
#[test]
fn doctor_runs_on_a_project_that_cannot_be_opened() {
  let fx = Fixture::new();
  let mut thread = clean_thread("ST0001");
  let mut dup = thread.criteria[0].clone();
  dup.kind = AcKind::NonTest;
  dup.state = AcState::Unsatisfied;
  thread.criteria.push(dup);
  seed(&fx, &thread);

  // The control: the ordinary path genuinely cannot open this estate, so the
  // test below is not passing because the fault was too mild to matter.
  let opened = intentsvcs::facade::Facade::open_in_memory(fx.project(), common::facade_ctx());
  assert!(
    opened.is_err(),
    "precondition: a duplicate criterion id must defeat the normal open path"
  );

  let report = intentsvcs::doctor::diagnose(&fx.project(), &ctx(), None);
  assert!(
    !report.is_healthy(),
    "doctor ran and reported on an estate the facade could not open"
  );
  let found = details(&report.findings);
  assert!(
    found.contains("cannot be loaded into a store"),
    "and it names the DB failure as a diagnosis rather than dying of it: {found}"
  );
  assert!(
    found.contains("declared more than once"),
    "and still reports the model fault that CAUSED it: {found}"
  );
}

/// A COLD cache is the normal state and must never be a finding; a STALE one
/// must be.
///
/// `intent/.cache/` is gitignored (D21), so an empty store is what every fresh
/// clone has. The first version of the DB check reported it, which would have
/// fired on the commonest healthy state there is -- and a health check that
/// cries wolf is one nobody reads. The second half of this test is what stops
/// the narrowing from quietly disabling the check altogether.
#[test]
fn a_cold_cache_is_healthy_and_a_stale_one_is_not() {
  let fx = Fixture::new();
  seed(&fx, &clean_thread("ST0001"));

  assert!(
    intentsvcs::doctor::diagnose(&fx.project(), &ctx(), None).is_healthy(),
    "a project whose on-disk cache was never written is healthy"
  );

  // Populate the on-disk cache, then move the canon out from under it.
  intentsvcs::facade::Facade::open(fx.project(), common::facade_ctx()).expect("populate the cache");
  let mut moved = clean_thread("ST0001");
  moved.title = "A different title entirely".to_string();
  fx.write_thread(&moved);

  let findings = run(&fx);
  assert!(
    findings
      .iter()
      .any(|f| f.file.contains("intent.db") && f.detail.contains("does not match a rebuild")),
    "a populated cache disagreeing with canon is a real diagnosis -- doctor does not open the facade, so it can still SEE the staleness rather than silently repairing it: {}",
    details(&findings)
  );
}

/// Canon that will not parse becomes findings, not a crash.
#[test]
fn unreadable_canon_becomes_findings() {
  let fx = Fixture::new();
  fx.write_raw_thread("ST0001", "{ this is not json");

  let report = intentsvcs::doctor::diagnose(&fx.project(), &ctx(), None);
  assert!(!report.is_healthy());
  assert!(
    report
      .findings
      .iter()
      .any(|f| f.class == FindingClass::MalformedJson),
    "{}",
    details(&report.findings)
  );
}

/// The report never stops at the first finding.
#[test]
fn every_fault_is_reported_in_one_pass() {
  let fx = Fixture::new();
  let mut thread = clean_thread("ST0001");
  thread.status = ThreadStatus::Completed;
  thread.tests[0].covers = vec!["AC-99.9".to_string()];
  thread.related = vec![intentsvcs::model::Related {
    id: "ST9999".to_string(),
    note: None,
  }];
  // A descope pointing nowhere, rather than the kind/state mismatch this used
  // to carry: that fault is now refused by the schema face at ingest, and an
  // estate that does not load cannot demonstrate "every fault in one pass" --
  // it demonstrates one refusal. The property under test is unchanged; the
  // fault chosen to exercise it is one the earlier gate does not catch.
  thread.criteria[0].state = AcState::Descoped {
    to: "ST4242".to_string(),
    by: None,
    reason: None,
  };
  seed(&fx, &thread);

  let findings = run(&fx);
  let found = details(&findings);
  for expected in [
    "Completed with no completion date",
    "covers AC-99.9",
    "names ST9999 as related",
    "descoped to ST4242",
  ] {
    assert!(
      found.contains(expected),
      "one fix-and-rerun cycle, not one per defect -- missing {expected:?} in:\n{found}"
    );
  }
}

/// Doctor repairs nothing. A report about a state the reporter has already
/// changed describes something that no longer exists.
#[test]
fn doctor_changes_nothing_it_looks_at() {
  let fx = Fixture::new();
  seed(&fx, &clean_thread("ST0001"));
  let view = fx.path("intent/st/ST0001/info.md");
  std::fs::write(&view, "deliberately skewed\n").expect("skew the view");

  let first = run(&fx);
  assert!(!first.is_empty(), "precondition: there is something to fix");
  assert_eq!(
    fx.read("intent/st/ST0001/info.md"),
    "deliberately skewed\n",
    "doctor did not rewrite the file it reported on"
  );

  let second = run(&fx);
  assert_eq!(
    details(&first),
    details(&second),
    "running twice reports the same thing, because the first run repaired nothing"
  );
}

/// The version reaches the renderer, so skew is judged against the same
/// banner the tool writes -- not against a default that would make every view
/// look skewed.
#[test]
fn skew_is_judged_against_the_rendering_version() {
  let fx = Fixture::new();
  seed(&fx, &clean_thread("ST0001"));

  let other = intentsvcs::views::RenderContext {
    version: "9.9.9-not-the-fixture-version",
    todo_watermark: None,
  };
  let findings = intentsvcs::doctor::diagnose(&fx.project(), &other, None).findings;
  assert!(
    findings.iter().any(|f| f.class == FindingClass::ViewSkew),
    "a different version renders a different banner, which IS skew: {}",
    details(&findings)
  );
  assert_ne!(
    VERSION, "9.9.9-not-the-fixture-version",
    "the two versions must actually differ or this proves nothing"
  );
}

/// **A stored hash that no longer describes its content is reported, not
/// repaired.**
///
/// `Attachment::new` derives `bytes` and `sha256` from `text` and is the only
/// constructor, so nothing in the codebase can make them disagree --
/// **deserialisation is where that guarantee ends**, because `thread.json` is a
/// file and a file can be edited. Recomputing the hash here would make the
/// record agree with itself and destroy the only evidence that something wrote
/// a value it should not have.
#[test]
fn an_attachment_whose_hash_does_not_describe_its_text_is_found() {
  let fx = Fixture::new();
  let mut thread = clean_thread("ST0001");
  let mut a = intentsvcs::model::Attachment::new("reference.md", "# Reference\n");
  a.text = Some("# Reference\n\nEdited after the hash was taken.\n".to_string());
  thread.attachments = vec![a];
  seed(&fx, &thread);

  assert!(
    details(&run(&fx)).contains("attachment reference.md carries sha256"),
    "the finding names the attachment and both hashes: {}",
    details(&run(&fx))
  );
}

/// The counter-arm: an attachment built the one legitimate way is silent.
///
/// Without it the test above passes on a check that flags EVERY attachment,
/// which would red every healthy project carrying one -- the failure mode of
/// a rule that describes the model rather than the data.
#[test]
fn an_attachment_built_through_its_constructor_is_not_reported() {
  let fx = Fixture::new();
  let mut thread = clean_thread("ST0001");
  thread.attachments = vec![intentsvcs::model::Attachment::new(
    "reference.md",
    "# Reference\n",
  )];
  seed(&fx, &thread);

  assert!(
    !details(&run(&fx)).contains("attachment "),
    "a consistent attachment says nothing: {}",
    details(&run(&fx))
  );
}

/// **A file under a thread that the store does not hold is NAMED, and it is
/// not a fault.**
///
/// The failure this prevents is a disk becoming optional and something
/// vanishing because no surface ever said it was uncovered -- silence and full
/// coverage read identically. **But it must not be a finding**: these files are
/// uncarried by design, so counting them as faults would red 100% of a
/// population behaving correctly, which is a rule describing the model rather
/// than the data and is how a check gets deleted.
///
/// **The subject moved on 2026-08-26 and the property did not.** "Uncarried"
/// used to mean an extension outside `ATTACHMENT_EXTENSIONS`; the list is gone,
/// and what is uncarried now is what will not FIT. The report's meaning changed
/// with it -- from "an extension we do not carry" to "over the cap" -- and it is
/// a great deal shorter, which was announced rather than discovered.
#[test]
fn an_uncarried_file_is_listed_by_path_without_making_the_project_unhealthy() {
  let fx = Fixture::new();
  let thread = clean_thread("ST0001");
  seed(&fx, &thread);
  std::fs::create_dir_all(fx.path("intent/st/ST0001/parity")).expect("mkdir");
  std::fs::write(
    fx.path("intent/st/ST0001/parity/huge.png"),
    vec![b'x'; intentsvcs::project::ATTACHMENT_CAP_BYTES as usize + 1],
  )
  .expect("write a file over the cap");

  let report = intentsvcs::doctor::diagnose(&fx.project(), &ctx(), None);
  assert_eq!(
    report.unattached.len(),
    1,
    "exactly one: {:?}",
    report.unattached
  );
  assert!(
    report.unattached[0].starts_with("intent/st/ST0001/parity/huge.png"),
    "named by path, from the thread root down: {:?}",
    report.unattached
  );
  assert!(
    report.is_healthy(),
    "and it is inventory, not a fault: {}",
    details(&report.findings)
  );
}

/// The counter-arm, and it is what makes the test above mean anything: a file
/// the store DOES hold is absent from the list.
///
/// Without it, a `unattached` that simply listed every file under a thread
/// would satisfy the assertion above perfectly and report nothing true.
#[test]
fn a_carried_file_and_a_generated_view_are_absent_from_the_uncarried_list() {
  let fx = Fixture::new();
  let mut thread = clean_thread("ST0001");
  thread.attachments = vec![intentsvcs::model::Attachment::new(
    "reference.md",
    "# Reference\n",
  )];
  seed(&fx, &thread);
  fx.write_file("intent/st/ST0001/reference.md", "# Reference\n");
  fx.write_file("intent/st/ST0001/parity/baseline.tap", "ok 1\n");
  std::fs::write(
    fx.path("intent/st/ST0001/parity/huge.png"),
    vec![b'x'; intentsvcs::project::ATTACHMENT_CAP_BYTES as usize + 1],
  )
  .expect("write a file over the cap");

  let report = intentsvcs::doctor::diagnose(&fx.project(), &ctx(), None);
  assert_eq!(
    report.unattached.len(),
    1,
    "exactly one: {:?}",
    report.unattached
  );
  assert!(
    report.unattached[0].starts_with("intent/st/ST0001/parity/huge.png"),
    "the attachment, the generated views AND the .tap are all held now -- only \
     the file that will not fit is uncarried: {:?}",
    report.unattached
  );
}

// ---------------------------------------------------------------------------
// AC-09.2 (B): history that exists only in this store is REPORTED, not judged
// ---------------------------------------------------------------------------

/// **hv RATIFIED THIS ARM ON 2026-08-15 AND ONLY HALF OF IT SHIPPED**
/// (`data-model.md:472`): *`wp done` is refused on a BLOCKED gate AND `doctor`
/// reports any unit whose status disagrees with its gate -- both, as
/// recommended.* The refusal landed; the report did not, and nothing watched
/// the join for five days.
///
/// **THE INSTANCE IS SYNTHETIC AND THAT IS THE RULING, NOT A CONVENIENCE** (vc,
/// 2026-08-20): neither the instance, nor the control, nor the predicate may be
/// drawn from the thing under test. A red-first keyed on a live disagreement
/// would make the disagreement a fixture, and the estate would not be free to
/// fix it -- which is precisely what this arm exists to help it do.
///
/// **THE DANGEROUS DIRECTION IS THE ONE DRIVEN HERE**, because it reported ZERO
/// against the live estate and a zero is not a result until the check has
/// produced a non-zero.
#[test]
fn a_done_work_package_over_a_blocked_gate_is_found() {
  let fx = Fixture::new();
  let mut thread = clean_thread("ST0001");
  // The contract GROWS after the close: a second criterion nothing satisfies.
  // This is ST0056/04's real history in one line -- the `Done` was true when it
  // was set and false afterwards, with nobody doing anything wrong.
  thread.criteria.push(Criterion {
    id: "AC-01.2".to_string(),
    text: "the second thing works".to_string(),
    kind: AcKind::Test,
    state: AcState::Computed,
  });
  seed(&fx, &thread);

  let report = intentsvcs::doctor::diagnose(&fx.project(), &ctx(), None);
  let hit: Vec<&Finding> = report
    .findings
    .iter()
    .filter(|f| f.class == FindingClass::StatusGateDisagreement)
    .collect();
  assert_eq!(
    hit.len(),
    1,
    "a Done work package over a blocked gate must be reported exactly once: {}",
    details(&report.findings)
  );
  assert!(
    hit[0].detail.contains("ST0001/WP-01") && hit[0].detail.contains("BLOCKED"),
    "the finding must NAME the unit and the direction -- an operator cannot act on a count: {}",
    hit[0].detail
  );
}

/// **PAIRED, SO THE ARM ABOVE CANNOT PASS BY REPORTING EVERYTHING.** The same
/// thread with its contract intact must report nothing.
#[test]
fn a_done_work_package_over_a_passing_gate_is_not_found() {
  let fx = Fixture::new();
  seed(&fx, &clean_thread("ST0001"));

  let report = intentsvcs::doctor::diagnose(&fx.project(), &ctx(), None);
  assert!(
    !report
      .findings
      .iter()
      .any(|f| f.class == FindingClass::StatusGateDisagreement),
    "a Done work package whose gate passes is the healthy state and must be silent: {}",
    details(&report.findings)
  );
}

/// **THE THREE POPULATIONS THIS ARM DELIBERATELY DOES NOT JUDGE, driven rather
/// than asserted in a comment.** Each was found by running the arm against the
/// live estate and reading what it said: the count went 96, then 8, then 6.
///
/// All three are one rule -- **this compares a status to a VERDICT ABOUT A
/// CONTRACT, and a gate with no contract to judge has not returned one** -- and
/// each produced findings that were true of the arithmetic and false of the
/// estate.
#[test]
fn a_gate_with_no_contract_to_judge_is_not_a_disagreement() {
  // 1. A thread with ZERO criteria. `gate` blocks it by design, which is right
  //    for `wp done` and meaningless here. 52 completed v2 threads are in this
  //    state and the first cut reported 96 findings across them.
  let fx = Fixture::new();
  let mut empty = clean_thread("ST0001");
  empty.criteria.clear();
  empty.tests.clear();
  empty.wps[0].status = WpStatus::Done;
  seed(&fx, &empty);
  let report = intentsvcs::doctor::diagnose(&fx.project(), &ctx(), None);
  assert!(
    !report
      .findings
      .iter()
      .any(|f| f.class == FindingClass::StatusGateDisagreement),
    "a thread with no contract cannot disagree with a verdict about one: {}",
    details(&report.findings)
  );

  // 2. `acceptance: exempt`. The thread declined to be judged, and reading that
  //    as "every criterion is satisfied" would call a WIP package finished.
  let fx = Fixture::new();
  let mut exempt = clean_thread("ST0001");
  exempt.acceptance = Some(AcceptanceMode::Exempt);
  exempt.wps[0].status = WpStatus::Wip;
  seed(&fx, &exempt);
  let report = intentsvcs::doctor::diagnose(&fx.project(), &ctx(), None);
  assert!(
    !report
      .findings
      .iter()
      .any(|f| f.class == FindingClass::StatusGateDisagreement),
    "an exempt thread has declined to be judged: {}",
    details(&report.findings)
  );

  // 3. A WP with no criteria IN ITS OWN SCOPE passes vacuously, for the same
  //    reason `0 of 0` is always green. ST0056/WP-15 and WP-16 are exactly
  //    this, and the second cut called them work already done.
  let fx = Fixture::new();
  let mut vacuous = clean_thread("ST0001");
  vacuous.wps.push(WorkPackage {
    seq: 9,
    title: "A package with no criteria of its own".to_string(),
    scope: Some(TShirt::S),
    scope_legacy: None,
    status: WpStatus::NotStarted,
    status_reason: None,
    objective: String::new(),
    body: String::new(),
    preamble: String::new(),
  });
  seed(&fx, &vacuous);
  let report = intentsvcs::doctor::diagnose(&fx.project(), &ctx(), None);
  assert!(
    !report
      .findings
      .iter()
      .any(|f| f.class == FindingClass::StatusGateDisagreement),
    "an empty scope passes vacuously and is not evidence the work is done: {}",
    details(&report.findings)
  );
}
