//! AT-04.3 / AC-04.3: the close gate reads the model and reproduces v2's gate
//! verdicts.
//!
//! **Branch coverage**: every verdict path in `cmd_ac_gate`
//! (`bin/intent_acceptance:973`) has a case here, each citing the v2 line it
//! reproduces. Three of v2's BLOCKED paths are deliberately absent and their
//! absence is asserted, not ignored -- malformed AC/AT lines, AT-grammar
//! findings and a missing `acceptance.md` cannot occur when the contract is
//! model state rather than a parsed document.

use crate::common::{Fixture, sample_thread, tid, wp};
use intentsvcs::contract::{AllResolve, RepoFiles, Scope, gate};
use intentsvcs::model::{
  AcKind, AcState, AcceptanceMode, AcceptanceTest, AtKind, AtStatus, Criterion, THREAD_SCHEMA,
  TShirt, Thread, ThreadStatus, WorkPackage, WpStatus,
};

fn thread(criteria: Vec<Criterion>, tests: Vec<AcceptanceTest>) -> Thread {
  Thread {
    attachments: Vec::new(),
    body: String::new(),
    preamble: String::new(),
    schema: THREAD_SCHEMA.to_string(),
    id: tid("ST0001"),
    origin: None,
    realised: false,
    title: "gate parity".to_string(),
    slug: None,
    status: ThreadStatus::Wip,
    status_reason: None,
    fiat: None,
    created: "2026-08-14".to_string(),
    completed: None,
    acceptance: None,
    objective: String::new(),
    context: String::new(),
    related: Vec::new(),
    wps: vec![WorkPackage {
      seq: wp(1),
      origin: None,
      title: "one".to_string(),
      scope: Some(TShirt::M),
      scope_legacy: None,
      status_legacy: None,
      status: WpStatus::Wip,
      status_reason: None,
      fiat: None,
      objective: String::new(),
      body: String::new(),
      preamble: String::new(),
    }],
    criteria,
    tests,
  }
}

fn ac(id: &str, kind: AcKind, state: AcState) -> Criterion {
  Criterion {
    id: id.to_string(),
    text: "a criterion".to_string(),
    kind,
    state,
  }
}

fn at(id: &str, covers: &str, status: AtStatus) -> AcceptanceTest {
  AcceptanceTest {
    fiat: None,
    id: id.to_string(),
    kind: AtKind::Test,
    file: Some("crates/x/tests/y.rs".to_string()),
    prose: None,
    covers: vec![covers.to_string()],
    status,
    note: None,
    legacy: None,
  }
}

#[test]
fn a_fully_satisfied_thread_passes() {
  let t = thread(
    vec![ac("AC-01.1", AcKind::Test, AcState::Computed {})],
    vec![at("AT-01.1", "AC-01.1", AtStatus::Green)],
  );
  assert_eq!(
    gate(&t, Scope::Thread, &AllResolve).line("ST0001"),
    "gate: ST0001 PASS -- 1/1 satisfied"
  );
}

#[test]
fn an_unsatisfied_ac_blocks_and_is_named() {
  let t = thread(
    vec![
      ac("AC-01.1", AcKind::Test, AcState::Computed {}),
      ac(
        "AC-01.2",
        AcKind::NonTest,
        AcState::Satisfied {
          evidence: "e".to_string(),
        },
      ),
    ],
    vec![at("AT-01.1", "AC-01.1", AtStatus::Red)],
  );
  assert_eq!(
    gate(&t, Scope::Thread, &AllResolve).line("ST0001"),
    "gate: ST0001 BLOCKED -- 1/2 satisfied; unsatisfied: AC-01.1"
  );
}

#[test]
fn offscope_counts_are_reported_separately_from_satisfied() {
  let t = thread(
    vec![
      ac("AC-01.1", AcKind::Test, AcState::Computed {}),
      ac(
        "AC-01.2",
        AcKind::Test,
        AcState::Descoped {
          to: intentsvcs::id::ThreadId::parse("ST0002").expect("a stored thread id"),
          by: None,
          reason: None,
        },
      ),
      ac(
        "AC-01.3",
        AcKind::Test,
        AcState::Withdrawn {
          reason: "dropped".to_string(),
          by: None,
        },
      ),
    ],
    vec![at("AT-01.1", "AC-01.1", AtStatus::Green)],
  );
  assert_eq!(
    gate(&t, Scope::Thread, &AllResolve).line("ST0001"),
    "gate: ST0001 PASS -- 1/1 satisfied, 1 descoped, 1 withdrawn",
    "a thread that descoped half its contract must LOOK like one"
  );
}

#[test]
fn an_entirely_offscope_contract_blocks_rather_than_passing_vacuously() {
  let t = thread(
    vec![ac(
      "AC-01.1",
      AcKind::Test,
      AcState::Withdrawn {
        reason: "dropped".to_string(),
        by: None,
      },
    )],
    vec![],
  );
  let line = gate(&t, Scope::Thread, &AllResolve).line("ST0001");
  assert!(line.contains("BLOCKED"), "{line}");
  assert!(line.contains("nothing is left to verify"), "{line}");
  // **THE EXEMPTION IS FIXED WHEN A THREAD IS AUTHORED AND NO VERB WRITES IT**
  // (issue 0400, hv's ruling of 2026-09-15), so the refusal names the routes
  // that exist and never the declaration.
  assert!(
    !line.contains("acceptance: exempt"),
    "the refusal names a declaration no verb writes: {line}"
  );
  let routes = [
    "intent ac new",
    "intent ac rescope",
    "intent ac reinstate",
    "intent st cancel",
    "intent wp cancel",
  ];
  let missing: Vec<&str> = routes.into_iter().filter(|r| !line.contains(r)).collect();
  assert!(
    missing.is_empty(),
    "the refusal names every route that exists, missing {missing:?}: {line}"
  );
}

#[test]
fn an_empty_contract_blocks() {
  let line = gate(&thread(vec![], vec![]), Scope::Thread, &AllResolve).line("ST0001");
  assert!(
    line.contains("BLOCKED -- the thread has zero acceptance criteria"),
    "{line}"
  );
}

#[test]
fn exempt_passes_and_announces_itself() {
  let mut t = thread(vec![], vec![]);
  t.acceptance = Some(AcceptanceMode::Exempt);
  assert_eq!(
    gate(&t, Scope::Thread, &AllResolve).line("ST0001"),
    "gate: ST0001 EXEMPT -- the thread declares 'acceptance: exempt'"
  );
}

#[test]
fn an_ac_free_wp_rolls_up_and_says_so() {
  let mut t = thread(
    vec![ac("AC-00.1", AcKind::Test, AcState::Computed {})],
    vec![at("AT-00.1", "AC-00.1", AtStatus::Green)],
  );
  t.wps.push(WorkPackage {
    seq: wp(4),
    origin: None,
    title: "later".to_string(),
    scope: Some(TShirt::S),
    scope_legacy: None,
    status_legacy: None,
    status: WpStatus::NotStarted,
    status_reason: None,
    fiat: None,
    objective: String::new(),
    body: String::new(),
    preamble: String::new(),
  });
  assert_eq!(
    gate(&t, Scope::WorkPackage(wp(4)), &AllResolve).line("ST0001/04"),
    "gate: ST0001/04 PASS -- no ACs in scope; rolls up to the ST0001 contract (1 AC(s))"
  );
}

/// Issue 0004: a typo'd WP must not inherit the rollup. Only a lookup tells a
/// nonexistent WP from a genuinely AC-free one -- they are arithmetically
/// identical.
#[test]
fn a_nonexistent_wp_blocks_rather_than_rolling_up() {
  let t = thread(
    vec![ac("AC-00.1", AcKind::Test, AcState::Computed {})],
    vec![at("AT-00.1", "AC-00.1", AtStatus::Green)],
  );
  let line = gate(&t, Scope::WorkPackage(wp(99)), &AllResolve).line("ST0001/99");
  assert!(line.contains("BLOCKED"), "{line}");
  assert!(line.contains("nothing to evaluate"), "{line}");
}

/// Issue 0024: a scoped gate answers the NARROWED question. An equality-only
/// suite would have carried v2's silent scope-drop into v3 as certified.
#[test]
fn a_wp_scope_evaluates_only_that_wps_criteria() {
  let mut t = thread(
    vec![
      ac("AC-01.1", AcKind::Test, AcState::Computed {}),
      ac("AC-02.1", AcKind::Test, AcState::Computed {}),
    ],
    vec![at("AT-01.1", "AC-01.1", AtStatus::Green)],
  );
  t.wps.push(WorkPackage {
    seq: wp(2),
    origin: None,
    title: "two".to_string(),
    scope: Some(TShirt::M),
    scope_legacy: None,
    status_legacy: None,
    status: WpStatus::Wip,
    status_reason: None,
    fiat: None,
    objective: String::new(),
    body: String::new(),
    preamble: String::new(),
  });

  assert_eq!(
    gate(&t, Scope::WorkPackage(wp(1)), &AllResolve).line("ST0001/01"),
    "gate: ST0001/01 PASS -- 1/1 satisfied",
    "WP-01 is complete even though the thread is not"
  );
  assert_eq!(
    gate(&t, Scope::WorkPackage(wp(2)), &AllResolve).line("ST0001/02"),
    "gate: ST0001/02 BLOCKED -- 0/1 satisfied; unsatisfied: AC-02.1"
  );
  assert_eq!(
    gate(&t, Scope::Thread, &AllResolve).line("ST0001"),
    "gate: ST0001 BLOCKED -- 1/2 satisfied; unsatisfied: AC-02.1",
    "the thread scope sees both"
  );
}

/// Issue 0015: `n-a` is the non-test doc status and is not coverage.
#[test]
fn an_n_a_test_does_not_satisfy_a_test_backed_ac() {
  let mut t = thread(
    vec![ac("AC-01.1", AcKind::Test, AcState::Computed {})],
    vec![at("AT-01.1", "AC-01.1", AtStatus::Na)],
  );
  t.tests[0].kind = AtKind::NonTest;
  assert!(
    gate(&t, Scope::Thread, &AllResolve)
      .line("ST0001")
      .contains("BLOCKED"),
    "a recording AT never satisfies anything"
  );
}

/// The three v2 BLOCKED paths that cannot occur in v3, asserted as absent
/// rather than assumed. Their absence is a design consequence (the contract is
/// model state, so there is no row grammar to violate), and it belongs in the
/// register as a deviation rather than being discovered in triage.
#[test]
fn the_grammar_blocked_paths_are_unconstructible() {
  let t = thread(
    vec![ac("AC-01.1", AcKind::Test, AcState::Computed {})],
    vec![at("AT-01.1", "AC-01.1", AtStatus::Green)],
  );
  let line = gate(&t, Scope::Thread, &AllResolve).line("ST0001");
  for gone in ["malformed", "AT contract finding", "no acceptance.md"] {
    assert!(
      !line.contains(gone),
      "v3 cannot report {gone:?}: a malformed row cannot be represented in the model at all"
    );
  }
}

/// v2's L3, isolated: the cited file EXISTS but does not carry the AT's id.
///
/// This is the coverage mechanism rather than a naming convention. Without it
/// a row can cite a real file that tests something else entirely, and the gate
/// has no way to tell -- the contract would be green while nothing verified it.
#[test]
fn a_cited_file_that_does_not_carry_the_at_id_blocks() {
  let fx = Fixture::new();
  fx.write_file("crates/x/tests/y.rs", "// a test about something else\n");

  let t = thread(
    vec![ac("AC-01.1", AcKind::Test, AcState::Computed {})],
    vec![at("AT-01.1", "AC-01.1", AtStatus::Green)],
  );
  let verdict = gate(&t, Scope::Thread, &RepoFiles(fx.root()));
  assert!(
    !verdict.is_pass(),
    "the file exists, so L2 passes; L3 must still block: {}",
    verdict.line("ST0001")
  );
  assert!(
    verdict
      .line("ST0001")
      .contains("does not carry the literal id"),
    "and it says WHY, distinctly from the missing-file case: {}",
    verdict.line("ST0001")
  );

  // With the id present, the same contract passes -- so the check discriminates
  // rather than blocking everything that has a file.
  fx.write_file("crates/x/tests/y.rs", "// AT-01.1: the real test\n");
  assert!(
    gate(&t, Scope::Thread, &RepoFiles(fx.root())).is_pass(),
    "adding the id is the whole difference"
  );
}

/// L2/L3 apply only to a row that CLAIMS to have been run.
///
/// `to-write` is exempt because a missing file is the correct state for a test
/// not yet written. v2 restricts the arm to `green|red` for exactly this
/// reason, and a naive existence check would red every red-first row in a
/// contract that is being written properly.
#[test]
fn a_to_write_row_is_exempt_from_the_file_checks() {
  let fx = Fixture::new();
  // Deliberately no file on disk.
  let t = thread(
    vec![ac("AC-01.1", AcKind::Test, AcState::Computed {})],
    vec![at("AT-01.1", "AC-01.1", AtStatus::ToWrite)],
  );
  let verdict = gate(&t, Scope::Thread, &RepoFiles(fx.root()));
  assert!(
    verdict.line("ST0001").contains("0/1 satisfied"),
    "a to-write row blocks on SATISFACTION, not on its absent file: {}",
    verdict.line("ST0001")
  );
  assert!(
    !verdict.line("ST0001").contains("does not exist"),
    "red-first is the correct workflow and must not be punished: {}",
    verdict.line("ST0001")
  );
}

/// L3 is exempt on a completed thread: retrofitting id labels into a closed
/// thread's tests is archaeology, and v2 says so in as many words.
#[test]
fn a_completed_thread_is_exempt_from_the_id_check() {
  let fx = Fixture::new();
  fx.write_file("crates/x/tests/y.rs", "// a test with no id label\n");

  let mut t = thread(
    vec![ac("AC-01.1", AcKind::Test, AcState::Computed {})],
    vec![at("AT-01.1", "AC-01.1", AtStatus::Green)],
  );
  assert!(
    !gate(&t, Scope::Thread, &RepoFiles(fx.root())).is_pass(),
    "precondition: while open, the missing id label blocks"
  );

  t.status = ThreadStatus::Completed;
  assert!(
    gate(&t, Scope::Thread, &RepoFiles(fx.root())).is_pass(),
    "a closed thread is not made to retrofit id labels into its tests"
  );
}

/// L4: the REVERSE direction. Satisfaction is computed forwards -- for each AC,
/// find covering ATs -- so a `covers` id matching nothing is silently skipped
/// by that walk, and the AT reads as coverage it is not.
#[test]
fn a_covers_id_that_names_no_criterion_blocks() {
  let t = thread(
    vec![ac("AC-01.1", AcKind::Test, AcState::Computed {})],
    vec![at("AT-01.1", "AC-09.9", AtStatus::Green)],
  );
  let line = gate(&t, Scope::Thread, &AllResolve).line("ST0001");
  assert!(line.contains("BLOCKED"), "{line}");
  assert!(
    line.contains("covers AC-09.9, which is not a criterion in this contract"),
    "the finding names the dangling id: {line}"
  );
}

/// L5: the trap the non-test arm would otherwise bless.
///
/// A non-test AT is `n-a` by definition and `n-a` is never green, so a
/// test-backed AC covered only by one can NEVER be satisfied. v2's own comment
/// names the failure: "an unclosable contract whose only symptom is a gate that
/// will not move". Reproducing the symptom without the diagnosis would leave
/// the operator staring at a permanently unsatisfied AC with nothing to act on.
#[test]
fn a_non_test_at_covering_a_test_backed_ac_blocks_with_the_reason() {
  let mut t = thread(
    vec![ac("AC-01.1", AcKind::Test, AcState::Computed {})],
    vec![at("AT-01.1", "AC-01.1", AtStatus::Na)],
  );
  t.tests[0].kind = AtKind::NonTest;
  t.tests[0].file = None;
  t.tests[0].prose = Some("eyeballed".to_string());

  let line = gate(&t, Scope::Thread, &AllResolve).line("ST0001");
  assert!(line.contains("BLOCKED"), "{line}");
  assert!(
    line.contains("can never satisfy it"),
    "the gate diagnoses the unclosable contract rather than merely refusing to move: {line}"
  );
  assert!(
    !line.contains("0/1 satisfied"),
    "and it does NOT report this as ordinary unsatisfaction -- that is the collapse: {line}"
  );

  // A non-test AT covering a NON-test AC is legitimate and must pass.
  t.criteria[0].kind = AcKind::NonTest;
  t.criteria[0].state = AcState::Satisfied {
    evidence: "read it".to_string(),
  };
  assert!(
    gate(&t, Scope::Thread, &AllResolve).is_pass(),
    "the check discriminates the trap from the legitimate pairing"
  );
}

/// The anti-collapse assertion across all four rules.
///
/// Four different problems with four different fixes. If any two read alike,
/// the operator has to guess which they hit -- the same defect AC-04.4 forbids
/// in the error type, and the same one the mutation battery found when L2 and
/// L3 turned out to be mutually covering.
#[test]
fn the_four_contract_rules_have_four_distinguishable_diagnoses() {
  let fx = Fixture::new();
  fx.write_file(
    "crates/x/tests/wrong.rs",
    "// a test about something else\n",
  );

  let l2 = thread(
    vec![ac("AC-01.1", AcKind::Test, AcState::Computed {})],
    vec![at("AT-01.1", "AC-01.1", AtStatus::Green)],
  );

  let mut l3 = l2.clone();
  l3.tests[0].file = Some("crates/x/tests/wrong.rs".to_string());

  let l4 = thread(
    vec![ac("AC-01.1", AcKind::Test, AcState::Computed {})],
    vec![at("AT-01.1", "AC-09.9", AtStatus::ToWrite)],
  );

  let mut l5 = thread(
    vec![ac("AC-01.1", AcKind::Test, AcState::Computed {})],
    vec![at("AT-01.1", "AC-01.1", AtStatus::Na)],
  );
  l5.tests[0].kind = AtKind::NonTest;
  l5.tests[0].file = None;

  let lines: Vec<String> = [&l2, &l3, &l4, &l5]
    .iter()
    .map(|t| gate(t, Scope::Thread, &RepoFiles(fx.root())).line("ST0001"))
    .collect();

  for (i, a) in lines.iter().enumerate() {
    assert!(a.contains("BLOCKED"), "case {i} must block: {a}");
    for (j, b) in lines.iter().enumerate().skip(i + 1) {
      assert_ne!(
        a, b,
        "cases {i} and {j} are indistinguishable:\n  {a}\n  {b}"
      );
    }
  }
}

/// The facade cannot close a unit the gate blocks, and the sample fixture's
/// real contract exercises the pass side.
#[test]
fn the_facade_routes_closes_through_the_gate() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();

  assert!(
    facade
      .gate("ST0056", Scope::WorkPackage(wp(3)))
      .unwrap()
      .is_pass()
  );
  facade.wp_done("ST0056", wp(3)).expect("wp done");

  facade
    .at_set("ST0056", "AT-03.1", AtStatus::Red, None)
    .unwrap();
  facade
    .at_set("ST0056", "AT-03.7", AtStatus::Red, None)
    .unwrap();

  // **A CLOSED unit is NOT re-blocked by coverage that went red after the close,
  // and this assertion used to require the opposite** (hv, 2026-08-17). Asking
  // `wp done` of an already-done package is a self-loop: accepted at 0, with the
  // gate not re-run. The rule is that a self-loop must not be able to fail for a
  // reason that did not exist when the state was entered -- and this test was the
  // clearest statement of the behaviour being retired, since it turned the
  // coverage red AFTER the close and then demanded a refusal.
  assert_eq!(
    facade
      .wp_done("ST0056", wp(3))
      .expect("a self-loop is accepted")
      .already(),
    Some(WpStatus::Done.display()),
    "a package that is already done stays done, and the gate is not consulted again"
  );

  // The test's real subject, on a package the gate is actually asked about.
  // Reopening is what makes the question live again -- which is the point of
  // `wp reopen` existing.
  facade
    .wp_reopen("ST0056", wp(3), "the coverage went red")
    .expect("reopen");
  assert!(
    facade.wp_done("ST0056", wp(3)).is_err(),
    "the same gate refuses once the coverage goes red, asked of a package that is open"
  );
}
