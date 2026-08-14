//! AT-04.3 / AC-04.3: the close gate reads the model and reproduces v2's gate
//! verdicts.
//!
//! Two halves, and the second is the one that makes this parity rather than
//! self-agreement.
//!
//! **Branch coverage**: every verdict path in `cmd_ac_gate`
//! (`bin/intent_acceptance:973`) has a case here, each citing the v2 line it
//! reproduces. Three of v2's BLOCKED paths are deliberately absent and their
//! absence is asserted, not ignored -- malformed AC/AT lines, AT-grammar
//! findings and a missing `acceptance.md` cannot occur when the contract is
//! model state rather than a parsed document.
//!
//! **Live differential**: v2's own binary is run over an equivalent v2 estate
//! and its verdict line compared to v3's, byte for byte. A fixture table
//! asserts what I believe v2 says; the differential asserts what it actually
//! says, and only the second can catch a misremembered format.

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::contract::{AllResolve, RepoFiles, Scope, gate};
use intentsvcs::model::{
  AcKind, AcScope, AcceptanceMode, AcceptanceTest, AtKind, AtStatus, Criterion, THREAD_SCHEMA,
  TShirt, Thread, ThreadStatus, WorkPackage, WpStatus,
};

fn thread(criteria: Vec<Criterion>, tests: Vec<AcceptanceTest>) -> Thread {
  Thread {
    schema: THREAD_SCHEMA.to_string(),
    id: "ST0001".to_string(),
    title: "gate parity".to_string(),
    slug: None,
    status: ThreadStatus::Wip,
    created: "2026-08-14".to_string(),
    completed: None,
    acceptance: None,
    objective: String::new(),
    context: String::new(),
    related: Vec::new(),
    wps: vec![WorkPackage {
      seq: 1,
      title: "one".to_string(),
      scope: TShirt::M,
      status: WpStatus::Wip,
    }],
    criteria,
    tests,
  }
}

fn ac(id: &str, kind: AcKind, scope: AcScope, satisfied: Option<bool>) -> Criterion {
  Criterion {
    id: id.to_string(),
    text: "a criterion".to_string(),
    kind,
    scope,
    evidence: None,
    satisfied,
  }
}

fn at(id: &str, covers: &str, status: AtStatus) -> AcceptanceTest {
  AcceptanceTest {
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
    vec![ac("AC-01.1", AcKind::Test, AcScope::InScope, None)],
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
      ac("AC-01.1", AcKind::Test, AcScope::InScope, None),
      ac("AC-01.2", AcKind::NonTest, AcScope::InScope, Some(true)),
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
      ac("AC-01.1", AcKind::Test, AcScope::InScope, None),
      ac(
        "AC-01.2",
        AcKind::Test,
        AcScope::Descoped {
          to: "ST0002".to_string(),
          by: None,
          reason: None,
        },
        None,
      ),
      ac(
        "AC-01.3",
        AcKind::Test,
        AcScope::Withdrawn {
          reason: "dropped".to_string(),
          by: None,
        },
        None,
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
      AcScope::Withdrawn {
        reason: "dropped".to_string(),
        by: None,
      },
      None,
    )],
    vec![],
  );
  let line = gate(&t, Scope::Thread, &AllResolve).line("ST0001");
  assert!(line.contains("BLOCKED"), "{line}");
  assert!(line.contains("nothing is left to verify"), "{line}");
  assert!(
    line.contains("acceptance: exempt"),
    "it routes to the DECLARED escape rather than inventing a second way to say no contract: {line}"
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
    vec![ac("AC-00.1", AcKind::Test, AcScope::InScope, None)],
    vec![at("AT-00.1", "AC-00.1", AtStatus::Green)],
  );
  t.wps.push(WorkPackage {
    seq: 4,
    title: "later".to_string(),
    scope: TShirt::S,
    status: WpStatus::NotStarted,
  });
  assert_eq!(
    gate(&t, Scope::WorkPackage(4), &AllResolve).line("ST0001/04"),
    "gate: ST0001/04 PASS -- no ACs in scope; rolls up to the ST0001 contract (1 AC(s))"
  );
}

/// Issue 0004: a typo'd WP must not inherit the rollup. Only a lookup tells a
/// nonexistent WP from a genuinely AC-free one -- they are arithmetically
/// identical.
#[test]
fn a_nonexistent_wp_blocks_rather_than_rolling_up() {
  let t = thread(
    vec![ac("AC-00.1", AcKind::Test, AcScope::InScope, None)],
    vec![at("AT-00.1", "AC-00.1", AtStatus::Green)],
  );
  let line = gate(&t, Scope::WorkPackage(99), &AllResolve).line("ST0001/99");
  assert!(line.contains("BLOCKED"), "{line}");
  assert!(line.contains("nothing to evaluate"), "{line}");
}

/// Issue 0024: a scoped gate answers the NARROWED question. An equality-only
/// suite would have carried v2's silent scope-drop into v3 as certified.
#[test]
fn a_wp_scope_evaluates_only_that_wps_criteria() {
  let mut t = thread(
    vec![
      ac("AC-01.1", AcKind::Test, AcScope::InScope, None),
      ac("AC-02.1", AcKind::Test, AcScope::InScope, None),
    ],
    vec![at("AT-01.1", "AC-01.1", AtStatus::Green)],
  );
  t.wps.push(WorkPackage {
    seq: 2,
    title: "two".to_string(),
    scope: TShirt::M,
    status: WpStatus::Wip,
  });

  assert_eq!(
    gate(&t, Scope::WorkPackage(1), &AllResolve).line("ST0001/01"),
    "gate: ST0001/01 PASS -- 1/1 satisfied",
    "WP-01 is complete even though the thread is not"
  );
  assert_eq!(
    gate(&t, Scope::WorkPackage(2), &AllResolve).line("ST0001/02"),
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
    vec![ac("AC-01.1", AcKind::Test, AcScope::InScope, None)],
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
    vec![ac("AC-01.1", AcKind::Test, AcScope::InScope, None)],
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
    vec![ac("AC-01.1", AcKind::Test, AcScope::InScope, None)],
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

/// The live differential: v2's OWN binary, over an equivalent v2 estate.
///
/// The fixture tests above assert what I believe v2 prints. This asserts what
/// it actually prints, which is the only half that can catch a misremembered
/// format. Read-only: `ac gate` writes nothing, and `bin/` is never mutated.
#[test]
fn v2_and_v3_agree_on_a_real_contract() {
  let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
    .ancestors()
    .nth(2)
    .expect("workspace root");
  let v2 = repo.join("bin/intent");
  if !v2.is_file() {
    eprintln!("SKIPPED the live differential: bin/intent is absent (post-cutover tree?)");
    return;
  }

  let fx = Fixture::new();
  // A v2 estate: config.json is already written by the fixture; add a v2
  // thread whose contract is one satisfied AC backed by a green AT.
  //
  // The cited test file must EXIST. v2's lint rule L2 checks that, before the
  // AC loop, and this fixture originally cited a path that was not there --
  // which is how the differential earned its place: it found a v2 gate branch
  // the fixture tests had no idea about, and which v3 was silently missing.
  fx.write_file("crates/x/tests/y.rs", "// AT-01.1: the cited test\n");
  fx.write_file(
    "intent/st/ST0001/info.md",
    "---\nstatus: WIP\ncreated: 20260814\n---\n\n# ST0001: gate parity\n",
  );
  fx.write_file(
    "intent/st/ST0001/acceptance.md",
    "---\nst_id: ST0001\n---\n\n# ST0001 Acceptance\n\n## Acceptance Criteria\n\n- AC-01.1 a criterion\n\n## Acceptance Tests\n\n- AT-01.1 `crates/x/tests/y.rs` -- covers AC-01.1 -- status: green\n",
  );

  let out = std::process::Command::new(&v2)
    .args(["ac", "gate", "ST0001"])
    .current_dir(fx.root())
    .output()
    .expect("run v2 ac gate");
  let v2_line = String::from_utf8_lossy(&out.stdout)
    .lines()
    .find(|l| l.starts_with("gate: "))
    .map(str::to_string)
    .unwrap_or_else(|| {
      panic!(
        "v2 printed no gate line.\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
      )
    });

  let t = thread(
    vec![ac("AC-01.1", AcKind::Test, AcScope::InScope, None)],
    vec![at("AT-01.1", "AC-01.1", AtStatus::Green)],
  );
  let verdict = gate(&t, Scope::Thread, &RepoFiles(fx.root()));

  assert_eq!(
    verdict.line("ST0001"),
    v2_line,
    "v3's verdict line must be byte-identical to v2's for the same contract"
  );
  assert_eq!(
    out.status.code(),
    Some(verdict.exit_code()),
    "and so must the exit code -- `st done` reads it"
  );
}

/// The branch the differential found: v2's L2 blocks a contract whose cited
/// test file is absent, however green its rows are.
///
/// Verdict and exit code are compared; the remedy TEXT is a recorded deviation
/// (v2 sends the operator to `at lint --fix`, which in v3 has nothing to fix --
/// the row grammar it repaired is unconstructible, and a broken path is a
/// broken path). Comparing the words would be reproducing v2's phrasing rather
/// than its meaning.
#[test]
fn v2_and_v3_agree_that_a_missing_cited_file_blocks() {
  let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
    .ancestors()
    .nth(2)
    .expect("workspace root");
  let v2 = repo.join("bin/intent");
  if !v2.is_file() {
    eprintln!("SKIPPED the live differential: bin/intent is absent (post-cutover tree?)");
    return;
  }

  let fx = Fixture::new();
  // Deliberately do NOT create crates/x/tests/y.rs.
  fx.write_file(
    "intent/st/ST0001/info.md",
    "---\nstatus: WIP\ncreated: 20260814\n---\n\n# ST0001: gate parity\n",
  );
  fx.write_file(
    "intent/st/ST0001/acceptance.md",
    "---\nst_id: ST0001\n---\n\n# ST0001 Acceptance\n\n## Acceptance Criteria\n\n- AC-01.1 a criterion\n\n## Acceptance Tests\n\n- AT-01.1 `crates/x/tests/y.rs` -- covers AC-01.1 -- status: green\n",
  );

  let out = std::process::Command::new(&v2)
    .args(["ac", "gate", "ST0001"])
    .current_dir(fx.root())
    .output()
    .expect("run v2 ac gate");
  let v2_line = String::from_utf8_lossy(&out.stdout)
    .lines()
    .find(|l| l.starts_with("gate: "))
    .map(str::to_string)
    .expect("v2 printed a gate line");

  let t = thread(
    vec![ac("AC-01.1", AcKind::Test, AcScope::InScope, None)],
    vec![at("AT-01.1", "AC-01.1", AtStatus::Green)],
  );
  let verdict = gate(&t, Scope::Thread, &RepoFiles(fx.root()));

  assert!(
    v2_line.contains("BLOCKED"),
    "precondition: v2 blocks on the missing file: {v2_line}"
  );
  assert!(
    !verdict.is_pass(),
    "v3 must block too -- a contract whose tests point at files that are not there is not verifiable: {}",
    verdict.line("ST0001")
  );
  assert_eq!(
    out.status.code(),
    Some(verdict.exit_code()),
    "the exit code is the machine-facing contract and matches exactly"
  );
  assert!(
    verdict.line("ST0001").contains("AT-01.1"),
    "and v3 names the offending row: {}",
    verdict.line("ST0001")
  );
  // The DIAGNOSIS, not merely the block. Without this the test does not
  // discriminate L2 at all: a file that does not exist cannot carry an id
  // either, so L3 blocks the same case and removing L2 changes nothing
  // observable. Found by the mutation battery -- N4 removed L2 and this test
  // stayed green until it started asserting which rule fired.
  //
  // It also matters on its own terms: "the file is missing" and "the file is
  // the wrong one" are different problems with different fixes, and reporting
  // them identically is the same-text-for-different-causes collapse AC-04.4
  // forbids one layer up.
  assert!(
    verdict
      .line("ST0001")
      .contains("cites a file that does not exist"),
    "the finding says the file is MISSING, not that it fails some other rule: {}",
    verdict.line("ST0001")
  );
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
      .gate("ST0056", Scope::WorkPackage(3))
      .unwrap()
      .is_pass()
  );
  facade.wp_done("ST0056", 3).expect("wp done");

  facade.at_set("ST0056", "AT-03.1", AtStatus::Red).unwrap();
  facade.at_set("ST0056", "AT-03.7", AtStatus::Red).unwrap();
  assert!(
    facade.wp_done("ST0056", 3).is_err(),
    "the same gate refuses once the coverage goes red"
  );
}
