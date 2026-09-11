//! `doctor --scope` narrows to LIVE threads by default, and says what it took.
//!
//! **hv ruled the shape 2026-09-07** -- a narrow default is sensible, "but you
//! should be able to specify the scope with a `--scope` param" -- after reading
//! `intent doctor -v` on four estates and asking how hundreds of lines could be
//! called pristine. The measurement behind it: 96 of 331 fleet findings sat on
//! CLOSED threads, 50 on Conflab and 46 on Lamplight, and not one was work
//! anybody was going to do.
//!
//! **THE ARM THAT MATTERS MOST IS NOT THE NARROWING; IT IS
//! [`Report::out_of_scope`].** `0 finding(s)` over live threads is
//! byte-identical to `0 finding(s)` over everything, so a default that hid
//! without counting would be a denominator attack wearing a flag -- the error
//! vc committed the same morning by reporting "15 of 18 estates pristine" off a
//! count that could not see 372 lines of output. Every arm below that asserts a
//! narrowing also asserts the count of what was narrowed away.
//!
//! **EVERY ARM STATES WHAT IT WOULD HAVE TO SEE IN ORDER TO FAIL, AND CHECKS
//! THE FIXTURE CAN PRODUCE IT.** That is cc's phrasing of the A5 family and it
//! is sharper than "positive-control the instrument": twice on 2026-09-07 a
//! green test on this estate could not have reddened -- once because the
//! fixture carried a marker excluding it from the rule under test before the
//! bug could be reached, once because the sibling assertion read detail text
//! identical on both sides of the change. So the expected counts here are
//! MEASURED from the same fixture under [`Scope::All`] rather than written as
//! literals: a fixture that stopped producing findings would fail the
//! precondition instead of passing the conclusion.

use crate::common::{Fixture, ctx};
use intentsvcs::doctor::{Report, Scope};
use intentsvcs::finding::FindingClass;
use intentsvcs::model::{
  AcKind, AcState, AcceptanceTest, AtKind, AtStatus, Criterion, THREAD_SCHEMA, TShirt, Thread,
  ThreadStatus, WorkPackage, WpStatus,
};

/// A thread that reports NOTHING, plus `orphans` acceptance tests that cover no
/// criterion -- one `model-inconsistent` finding each.
///
/// The orphan is the lever because it is the simplest model finding there is:
/// it needs no gate, no store and no disk state, so an arm using it is measuring
/// scope and not some second thing that happens to travel with it.
fn thread(id: &str, status: ThreadStatus, orphans: usize) -> Thread {
  let mut tests = vec![AcceptanceTest {
    fiat: None,
    id: "AT-01.1".to_string(),
    kind: AtKind::Test,
    file: Some("tests/clean.rs".to_string()),
    prose: None,
    covers: vec!["AC-01.1".to_string()],
    status: AtStatus::Green,
    note: None,
    legacy: None,
  }];
  for n in 0..orphans {
    tests.push(AcceptanceTest {
      fiat: None,
      id: format!("AT-01.{}", n + 2),
      kind: AtKind::Test,
      file: Some("tests/orphan.rs".to_string()),
      prose: None,
      // The whole point: covers nothing.
      covers: vec![],
      status: AtStatus::Green,
      note: None,
      legacy: None,
    });
  }
  Thread {
    attachments: Vec::new(),
    body: String::new(),
    preamble: String::new(),
    schema: THREAD_SCHEMA.to_string(),
    id: id.to_string(),
    title: "A thread with a known shape".to_string(),
    slug: None,
    status,
    status_reason: None,
    fiat: None,
    created: "2026-08-14".to_string(),
    // **Recorded even on the live threads, and that is deliberate.** A closed
    // thread with no completion date is its own finding
    // (`field-not-recorded`), which would land in these counts and make the
    // arms measure two things at once.
    completed: status.is_closed().then(|| "2026-08-20".to_string()),
    acceptance: None,
    objective: "Be internally consistent.".to_string(),
    context: String::new(),
    related: vec![],
    wps: vec![WorkPackage {
      seq: 1,
      title: "The only package".to_string(),
      scope: Some(TShirt::S),
      scope_legacy: None,
      status_legacy: None,
      status: WpStatus::Done,
      status_reason: None,
      fiat: None,
      objective: String::new(),
      body: String::new(),
      preamble: String::new(),
    }],
    criteria: vec![Criterion {
      id: "AC-01.1".to_string(),
      text: "the thing works".to_string(),
      kind: AcKind::Test,
      state: AcState::Computed {},
    }],
    tests,
  }
}

fn seed(fx: &Fixture, threads: &[Thread]) {
  for t in threads {
    fx.write_thread(t);
  }
  let project = fx.project();
  let canon = intentsvcs::ingest::read(&project).expect("fixture canon reads");
  intentsvcs::views::write_all(&project, &canon, &ctx()).expect("write views");
}

fn diagnose(fx: &Fixture, scope: Scope) -> Report {
  intentsvcs::doctor::diagnose(&fx.project(), &ctx(), None, scope)
}

/// The precondition every narrowing arm rests on, stated once.
///
/// **Without this the conclusion is unfalsifiable.** "Live reports nothing"
/// passes identically on a correct narrowing and on a `doctor` that has stopped
/// finding anything at all, and the second one reads exactly like the first.
fn findings_under_all(fx: &Fixture) -> usize {
  let all = diagnose(fx, Scope::All);
  assert!(
    !all.findings.is_empty(),
    "the fixture must produce findings under `all` or every arm below is vacuous: {:?}",
    all.findings
  );
  assert_eq!(
    all.out_of_scope, 0,
    "`all` admits every thread, so nothing can be withheld from it"
  );
  all.findings.len()
}

#[test]
fn the_default_withholds_a_closed_thread_and_counts_what_it_withheld() {
  let fx = Fixture::new();
  seed(&fx, &[thread("ST0001", ThreadStatus::Completed, 1)]);
  let expected = findings_under_all(&fx);

  let live = diagnose(&fx, Scope::Live);
  assert!(
    live.findings.is_empty(),
    "a completed thread's model findings are not live work: {:?}",
    live.findings
  );
  assert_eq!(
    live.out_of_scope, expected,
    "the count of what the default took away is the whole safety property"
  );
  assert_eq!(live.scope, Scope::Live, "the report carries its own scope");
}

#[test]
fn scope_all_is_the_behaviour_every_build_before_the_flag_had() {
  let fx = Fixture::new();
  seed(&fx, &[thread("ST0001", ThreadStatus::Completed, 1)]);
  let expected = findings_under_all(&fx);

  let all = diagnose(&fx, Scope::All);
  assert_eq!(all.findings.len(), expected);
  assert_eq!(all.out_of_scope, 0);
  assert_eq!(all.scope, Scope::All);
}

#[test]
fn live_and_closed_partition_the_model_findings_exactly() {
  let fx = Fixture::new();
  seed(
    &fx,
    &[
      thread("ST0001", ThreadStatus::Wip, 1),
      thread("ST0002", ThreadStatus::Completed, 1),
    ],
  );
  let total = findings_under_all(&fx);

  let live = diagnose(&fx, Scope::Live);
  let closed = diagnose(&fx, Scope::Closed);

  // **THE ARITHMETIC IS THE ASSERTION.** Each scope's kept-plus-withheld must
  // reconstruct the whole, which is what makes "nothing fell out of the report
  // unannounced" checkable rather than asserted.
  assert_eq!(live.findings.len() + live.out_of_scope, total);
  assert_eq!(closed.findings.len() + closed.out_of_scope, total);
  assert_eq!(
    live.findings.len() + closed.findings.len(),
    total,
    "live and closed are complements over the model checks, so they tile the whole"
  );
  assert!(
    live.findings.iter().all(|f| f.file.contains("ST0001")),
    "live keeps the WIP thread only: {:?}",
    live.findings
  );
  assert!(
    closed.findings.iter().all(|f| f.file.contains("ST0002")),
    "closed keeps the completed thread only: {:?}",
    closed.findings
  );
}

/// **THIS ARM WAS VACUOUS AT TWO ORPHANS AND A CONTROL IS WHAT SAID SO.**
///
/// Written with two, it asserted `out_of_scope == 2` on one thread -- and a
/// deliberate break replacing `+= found.len()` with `+= 1` LEFT IT GREEN.
/// [`Report::admit`] is called twice per thread, once for the model checks and
/// once for the gate arm, so a per-CALL counter also reaches 2. The two
/// quantities this arm exists to separate collided at the fixture's own number,
/// and nothing about the green read wrong.
///
/// **THREE is the smallest fixture where findings, threads and admit calls are
/// three different numbers** -- 3, 1 and 2. That is the whole reason for the
/// count, and it is written down because the instinct is to trim it back.
#[test]
fn the_withheld_count_is_findings_and_not_threads() {
  let fx = Fixture::new();
  seed(&fx, &[thread("ST0001", ThreadStatus::Completed, 3)]);
  let expected = findings_under_all(&fx);
  assert_eq!(
    expected, 3,
    "ONE thread, THREE findings, TWO admit calls -- all three must differ or \
     the arm cannot tell which one the field counts"
  );

  let live = diagnose(&fx, Scope::Live);
  assert_eq!(
    live.out_of_scope, 3,
    "findings withheld -- not threads skipped (1) and not admit calls made (2)"
  );
}

#[test]
fn a_disk_fact_is_reported_at_every_scope() {
  let fx = Fixture::new();
  seed(&fx, &[thread("ST0001", ThreadStatus::Completed, 0)]);
  // A hand-edited generated view on a CLOSED thread. `sync --to-disk` repairs
  // it whatever the thread's status, so scoping it away would hide a live
  // divergence behind a finished thread.
  let view = "intent/st/ST0001/info.md";
  fx.write_file(
    view,
    "hand-written rubbish that the model does not render\n",
  );

  for scope in Scope::ALL {
    let report = diagnose(&fx, scope);
    assert!(
      report
        .findings
        .iter()
        .any(|f| f.class == FindingClass::ViewSkew),
      "view-skew is a DISK fact and survives `--scope {}`: {:?}",
      scope.wire(),
      report.findings
    );
  }
}

#[test]
fn every_declared_scope_round_trips_its_wire_word() {
  // Over the POPULATION, not over three literals: a round trip written against
  // hand-typed words passes on the day a fourth variant becomes unreachable,
  // and goes wrong by being right.
  for scope in Scope::ALL {
    assert_eq!(
      Scope::from_wire(scope.wire()),
      Some(scope),
      "`{}` must survive the round trip",
      scope.wire()
    );
  }
  assert_eq!(
    Scope::ALL.len(),
    3,
    "a new variant needs a wire word and a table row"
  );
  assert_eq!(Scope::default(), Scope::Live, "hv ruled the default narrow");
  assert_eq!(Scope::from_wire("bogus"), None);
  assert_eq!(
    Scope::from_wire("LIVE"),
    None,
    "the vocabulary is exact; a near miss is refused rather than guessed at"
  );
}
