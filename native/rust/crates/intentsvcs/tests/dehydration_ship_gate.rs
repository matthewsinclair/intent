//! AT-00.1 / AC-00.1: **no dehydration path removes any file while any declared
//! precondition is unmet, the refusal names EVERY unmet one, and the denominator
//! is printed -- preconditions checked against preconditions declared.**
//!
//! **THE ROW'S REAL SUBJECT IS THE SINGLE DECLARATION, NOT THE REFUSAL.** A gate
//! that refuses correctly against a list it keeps for itself passes every
//! obvious test and fails the day the list in `design.md` grows an eighth
//! bullet -- silently, in the safe-looking direction, because a gate checking a
//! stale subset still refuses things. So the arms here vary the DECLARATION and
//! assert the checked set follows it. A hand-copied subset is what this row
//! exists to catch, and no assertion about refusing can catch it.
//!
//! **AND THE SURVIVING FILE NEEDS A POSITIVE CONTROL.** "The file is still
//! there" is produced equally by a gate that refused and by a plan that was
//! never going to remove anything -- an empty step list, a path that classified
//! differently, a fixture that quietly stopped producing a dehydration
//! candidate. The control runs the SAME tree through a plan whose declaration is
//! met and requires the file to be gone; if the control ever passes while quiet,
//! the refusing arm means nothing.
//!
//! **THE UNREADABLE CASES ARE ARMS, NOT EDGE CASES.** Absent thread, absent
//! criterion, no block, two blocks, empty block, malformed token: each must
//! REFUSE. This is where a gate is most likely to fall open, because every one
//! of them is a state where the honest thing to return is "I do not know", and
//! "I do not know" is the answer that has to mean no.

mod common;

use common::{Fixture, ctx, declaring_thread, gate_open, sample_thread};
use intentsvcs::ingest::Canon;
use intentsvcs::intentfiles;
use intentsvcs::model::{AcKind, AcState, Criterion, Thread};
use intentsvcs::organize::{Action, OrganizeError, TreeState, plan};
use intentsvcs::preconditions::{self, Unmet, Unreadable};

/// A manifest declaring a thread that is NOT in canon, so every view canon does
/// produce is a dehydration candidate.
const MANIFEST: &str = "\
STEELTHREAD:ST0001

# BEGIN INTENT
# END INTENT
";

fn met(evidence: &str) -> AcState {
  AcState::Satisfied {
    evidence: evidence.to_string(),
  }
}

fn canon_of(threads: Vec<Thread>) -> Canon {
  Canon {
    threads,
    ..Default::default()
  }
}

fn ids(v: &[String]) -> Vec<&str> {
  v.iter().map(String::as_str).collect()
}

// ---------------------------------------------------------------------------
// THE DECLARATION IS READ, NOT RESTATED
// ---------------------------------------------------------------------------

#[test]
fn the_checked_set_is_whatever_the_declaration_names() {
  // The load-bearing arm of the whole row. Two declarations that share NO id,
  // through one unmodified gate: if the module carried its own list, one of
  // these two must disagree with it.
  let one = preconditions::check(&canon_of(vec![declaring_thread(&[
    ("AC-00.2", AcKind::NonTest, met("a")),
    ("AC-03.1", AcKind::NonTest, met("b")),
  ])]));
  let other = preconditions::check(&canon_of(vec![declaring_thread(&[
    ("AC-06.4", AcKind::NonTest, met("c")),
    ("AC-07.5", AcKind::NonTest, met("d")),
    ("AC-07.6", AcKind::NonTest, met("e")),
  ])]));

  assert_eq!(ids(one.declared()), vec!["AC-00.2", "AC-03.1"]);
  assert_eq!(ids(other.declared()), vec!["AC-06.4", "AC-07.5", "AC-07.6"]);
  assert!(one.permits() && other.permits());
}

#[test]
fn ids_outside_the_block_are_not_swept_in() {
  // hv ruled the block DELIMITED for exactly this reason: AC-00.1's real text
  // names six ids in its mapping paragraph, and a whole-text scan would declare
  // preconditions nobody wrote. `declaring_thread` plants `AC-99.9` in the
  // prose on purpose.
  let v = preconditions::check(&canon_of(vec![declaring_thread(&[(
    "AC-00.9",
    AcKind::NonTest,
    met("met"),
  )])]));
  assert_eq!(ids(v.declared()), vec!["AC-00.9"]);
  assert!(
    !v.declared().iter().any(|id| id == "AC-99.9"),
    "an id named in the surrounding prose is not a declared precondition"
  );
}

#[test]
fn the_declaration_is_ordered_as_written() {
  // Not cosmetic: the refusal names the unmet ones, and an operator reading a
  // list that reorders itself between runs cannot tell a new failure from a
  // shuffled one.
  let v = preconditions::check(&canon_of(vec![declaring_thread(&[
    ("AC-07.1", AcKind::NonTest, AcState::Unsatisfied),
    ("AC-00.2", AcKind::NonTest, AcState::Unsatisfied),
    ("AC-03.4", AcKind::NonTest, AcState::Unsatisfied),
  ])]));
  assert_eq!(ids(v.declared()), vec!["AC-07.1", "AC-00.2", "AC-03.4"]);
  let unmet: Vec<&str> = v.unmet().iter().map(|(id, _)| id.as_str()).collect();
  assert_eq!(unmet, vec!["AC-07.1", "AC-00.2", "AC-03.4"]);
}

// ---------------------------------------------------------------------------
// THE REFUSAL NAMES EVERY UNMET ONE, AND PRINTS THE DENOMINATOR
// ---------------------------------------------------------------------------

#[test]
fn every_unmet_precondition_is_named_not_just_the_first() {
  let v = preconditions::check(&canon_of(vec![declaring_thread(&[
    ("AC-00.2", AcKind::NonTest, met("landed")),
    ("AC-00.3", AcKind::NonTest, AcState::Unsatisfied),
    ("AC-00.4", AcKind::NonTest, AcState::Unsatisfied),
    ("AC-06.1", AcKind::NonTest, AcState::Unsatisfied),
  ])]));
  let rendered = v.to_string();
  for id in ["AC-00.3", "AC-00.4", "AC-06.1"] {
    assert!(
      rendered.contains(id),
      "the refusal must name {id} -- naming only the first trains an operator to fix one and re-run: {rendered}"
    );
  }
  assert!(
    !rendered.contains("AC-00.2"),
    "a MET precondition must not appear in the unmet list: {rendered}"
  );
}

#[test]
fn the_denominator_is_printed_on_both_answers() {
  // A gate that prints its counts only when it refuses cannot be told, on a
  // quiet run, from a gate that checked nothing.
  let refusing = preconditions::check(&canon_of(vec![declaring_thread(&[
    ("AC-00.3", AcKind::NonTest, AcState::Unsatisfied),
    ("AC-00.4", AcKind::NonTest, met("landed")),
  ])]));
  let permitting = preconditions::check(&canon_of(vec![declaring_thread(&[
    ("AC-00.3", AcKind::NonTest, met("landed")),
    ("AC-00.4", AcKind::NonTest, met("landed")),
  ])]));

  assert!(
    refusing.to_string().contains("2 checked of 2 declared"),
    "got: {refusing}"
  );
  assert!(
    permitting.to_string().contains("2 checked of 2 declared"),
    "the permitting answer prints the same pair, or a quiet run proves nothing: {permitting}"
  );
  assert_eq!(refusing.checked().len(), refusing.declared().len());
  assert_eq!(permitting.checked().len(), permitting.declared().len());
}

// ---------------------------------------------------------------------------
// WHAT COUNTS AS MET
// ---------------------------------------------------------------------------

#[test]
fn a_descoped_or_withdrawn_precondition_is_unmet() {
  // Dropping a precondition is not meeting it. The alternative makes `intent ac
  // descope` a one-command way to open a gate standing in front of the estate.
  let v = preconditions::check(&canon_of(vec![declaring_thread(&[
    (
      "AC-00.3",
      AcKind::NonTest,
      AcState::Descoped {
        to: "ST0058".to_string(),
        by: Some("hv".to_string()),
        reason: Some("moved".to_string()),
      },
    ),
    (
      "AC-00.4",
      AcKind::NonTest,
      AcState::Withdrawn {
        reason: "the premise did not reproduce".to_string(),
        by: None,
      },
    ),
  ])]));
  assert!(!v.permits());
  assert_eq!(
    v.unmet(),
    &[
      ("AC-00.3".to_string(), Unmet::Descoped),
      ("AC-00.4".to_string(), Unmet::Withdrawn),
    ]
  );
}

#[test]
fn a_stored_satisfied_on_a_test_backed_precondition_does_not_open_the_gate() {
  // Canon is hand-authorable, so a test-backed criterion CAN arrive carrying
  // `satisfied`. `contract::resolve` refuses to believe it, and this gate asks
  // `contract::resolve` rather than reading the state -- which is the whole
  // reason it delegates instead of computing its own answer.
  let v = preconditions::check(&canon_of(vec![declaring_thread(&[(
    "AC-00.9",
    AcKind::Test,
    met("hand-written into canon"),
  )])]));
  assert!(
    !v.permits(),
    "a stored flag cannot satisfy a test-backed criterion, and the gate must not be the one place that believes it"
  );
  assert_eq!(v.unmet(), &[("AC-00.9".to_string(), Unmet::Unsatisfied)]);
}

#[test]
fn a_declared_id_with_no_criterion_is_unmet_rather_than_skipped() {
  // Skipped would make `checked` smaller than `declared` while both numbers
  // still looked healthy -- a list quietly shrinking, which is the defect the
  // printed denominator exists to expose.
  let mut thread = declaring_thread(&[("AC-00.9", AcKind::NonTest, met("landed"))]);
  thread.criteria.retain(|c| c.id != "AC-00.9");
  let v = preconditions::check(&canon_of(vec![thread]));
  assert_eq!(
    v.unmet(),
    &[("AC-00.9".to_string(), Unmet::NoSuchCriterion)]
  );
  assert_eq!(
    v.checked().len(),
    v.declared().len(),
    "an unresolvable id is still CHECKED -- dropping it from the count is how a denominator lies"
  );
}

// ---------------------------------------------------------------------------
// REFUSE-BY-DEFAULT: EVERY WAY THE DECLARATION CAN BE UNREADABLE
// ---------------------------------------------------------------------------

fn with_ac_00_1_text(text: &str) -> Canon {
  let mut thread = declaring_thread(&[("AC-00.9", AcKind::NonTest, met("landed"))]);
  for c in &mut thread.criteria {
    if c.id == "AC-00.1" {
      c.text = text.to_string();
    }
  }
  canon_of(vec![thread])
}

#[test]
fn an_estate_that_declares_nothing_refuses() {
  // **The ordinary state of every project that is not this one**, and the reason
  // the message names no thread: a consumer of the tool would otherwise be sent
  // to read paperwork that does not exist in their estate.
  let v = preconditions::check(&canon_of(vec![sample_thread("ST0001")]));
  assert!(!v.permits());
  assert_eq!(v.unreadable(), Some(&Unreadable::NoDeclaration));
}

#[test]
fn a_criterion_carrying_no_block_is_not_the_declaration() {
  let v = preconditions::check(&with_ac_00_1_text(
    "the preconditions are the seven bullets in design.md",
  ));
  assert!(!v.permits());
  assert_eq!(v.unreadable(), Some(&Unreadable::NoDeclaration));
}

#[test]
fn the_declaration_is_found_by_its_delimiter_not_by_an_address() {
  // The whole point of the rewrite: no thread id and no criterion id in the
  // module, so the same declaration is found wherever an author puts it.
  let mut thread = declaring_thread(&[("AC-00.9", AcKind::NonTest, met("landed"))]);
  let text = thread
    .criteria
    .iter()
    .find(|c| c.id == "AC-00.1")
    .expect("fixture carries it")
    .text
    .clone();
  thread.criteria.retain(|c| c.id != "AC-00.1");
  thread.criteria.push(Criterion {
    id: "AC-04.7".to_string(),
    text,
    kind: AcKind::NonTest,
    state: AcState::Unsatisfied,
  });
  let v = preconditions::check(&canon_of(vec![thread]));
  assert!(
    v.permits(),
    "moving the block to another criterion must change nothing: {v}"
  );
  assert_eq!(ids(v.declared()), vec!["AC-00.9"]);
}

#[test]
fn a_declaration_in_another_thread_is_found_too() {
  let mut carrier = declaring_thread(&[("AC-00.9", AcKind::NonTest, met("landed"))]);
  carrier.id = "ST0099".to_string();
  let v = preconditions::check(&canon_of(vec![sample_thread("ST0001"), carrier]));
  assert!(v.permits(), "got: {v}");
}

#[test]
fn two_carriers_anywhere_in_the_estate_refuse() {
  // **SINGLE BY MEASUREMENT RATHER THAN BY ASSERTION, and this is the arm the
  // address version could not have had.** An address points at one declaration
  // and says nothing about whether a second exists somewhere else.
  let one = declaring_thread(&[("AC-00.9", AcKind::NonTest, met("landed"))]);
  let mut two = declaring_thread(&[("AC-00.8", AcKind::NonTest, met("landed"))]);
  two.id = "ST0099".to_string();
  let v = preconditions::check(&canon_of(vec![one, two]));
  assert!(!v.permits());
  assert_eq!(v.unreadable(), Some(&Unreadable::TwoDeclarations));
}

#[test]
fn an_unterminated_block_refuses() {
  let v = preconditions::check(&with_ac_00_1_text(
    "<<PRECONDITIONS AC-00.2 and then the sentence just carries on",
  ));
  assert!(!v.permits());
  assert_eq!(v.unreadable(), Some(&Unreadable::Unterminated));
}

#[test]
fn two_blocks_refuse_because_there_is_no_single_declaration() {
  // The duplication AC-00.1 exists to forbid, arriving inside the criterion
  // that forbids it. Picking the first would be a gate choosing which of two
  // declarations to believe.
  let v = preconditions::check(&with_ac_00_1_text(
    "<<PRECONDITIONS AC-00.2 PRECONDITIONS>> and also <<PRECONDITIONS AC-00.3 PRECONDITIONS>>",
  ));
  assert!(!v.permits());
  assert_eq!(v.unreadable(), Some(&Unreadable::TwoBlocks));
}

#[test]
fn an_empty_block_refuses_rather_than_opening() {
  // An empty declaration is far likelier to be a deletion nobody noticed than
  // a considered all-clear -- and the considered version removes this gate
  // rather than emptying its input.
  let v = preconditions::check(&with_ac_00_1_text("<<PRECONDITIONS PRECONDITIONS>>"));
  assert!(!v.permits());
  assert_eq!(v.unreadable(), Some(&Unreadable::Empty));
}

#[test]
fn a_malformed_token_refuses_rather_than_being_filtered_out() {
  // Filtering is how a declaration of nineteen becomes a check of eighteen
  // with nothing saying so.
  let v = preconditions::check(&with_ac_00_1_text(
    "<<PRECONDITIONS AC-00.2 attachments AC-00.3 PRECONDITIONS>>",
  ));
  assert!(!v.permits());
  assert_eq!(
    v.unreadable(),
    Some(&Unreadable::Malformed("attachments".to_string()))
  );
}

#[test]
fn a_near_miss_token_is_malformed_rather_than_accepted() {
  // `AT-` is one character from `AC-`, and an acceptance TEST id in the block
  // would resolve to nothing and be reported as a missing criterion -- a
  // refusal for the wrong reason, which sends the operator to the wrong fix.
  for token in ["AT-00.2", "AC-0.2", "AC-00", "AC-00.x"] {
    let v = preconditions::check(&with_ac_00_1_text(&format!(
      "<<PRECONDITIONS {token} PRECONDITIONS>>"
    )));
    assert!(
      matches!(v.unreadable(), Some(Unreadable::Malformed(t)) if t == token),
      "{token} must be refused as malformed, got {:?}",
      v.unreadable()
    );
  }
}

#[test]
fn an_unreadable_declaration_prints_zero_of_zero_and_says_why() {
  let v = preconditions::check(&canon_of(vec![sample_thread("ST0001")]));
  let rendered = v.to_string();
  assert!(
    rendered.contains("0 checked of 0 declared"),
    "got: {rendered}"
  );
  assert!(
    rendered.contains("declares no dehydration preconditions"),
    "got: {rendered}"
  );
}

#[test]
fn no_refusal_this_gate_can_produce_names_this_project_s_own_paperwork() {
  // **THE CONSUMER-FACING ARM, AND THE STRING-LITERAL GUARD IS A PROXY FOR IT.**
  // This gate ships inside a binary other projects run. A refusal naming a
  // thread of Intent's own is unactionable in a consumer's estate -- they cannot
  // read it, cannot satisfy it, and cannot tell whether it is about them.
  // `no_pm_state_in_output.rs` caught the first version by scanning literals;
  // this asserts the property that scan stands in for, from the side that
  // renders, so the two fail independently.
  let every = [
    Unreadable::NoDeclaration,
    Unreadable::TwoDeclarations,
    Unreadable::TwoBlocks,
    Unreadable::Unterminated,
    Unreadable::Empty,
    Unreadable::Malformed("attachments".to_string()),
  ];
  for why in every {
    let rendered = why.to_string();
    assert!(
      !rendered.contains("ST00") && !rendered.contains("AC-0"),
      "a shipped refusal must not name this project's own paperwork: {rendered}"
    );
  }
}

// ---------------------------------------------------------------------------
// AND THE PROPERTY THE ROW IS ACTUALLY ABOUT: NO FILE IS REMOVED
// ---------------------------------------------------------------------------

/// Build a tree holding exactly one dehydration candidate whose bytes match the
/// render, so `organize::gate` would clear it and the ship gate is the only
/// thing that can stop the removal.
///
/// Returns the path and the plan, planned against `declaration`.
fn one_removal(
  fx: &Fixture,
  declaration: Thread,
) -> (std::path::PathBuf, intentsvcs::organize::Plan) {
  let project = fx.project();
  let canon = canon_of(vec![declaration, sample_thread("ST0002")]);
  let manifest = intentfiles::parse(MANIFEST).expect("manifest parses");
  let doomed = project.info_view("ST0002");
  let tree = TreeState {
    present: [doomed.clone()].into_iter().collect(),
    ..Default::default()
  };
  let p = plan(
    &project,
    &canon,
    &manifest,
    &ctx(),
    &tree,
    "digest".to_string(),
  );
  assert!(
    p.with(Action::Dehydrate).any(|s| s.path == doomed),
    "the fixture must produce exactly this dehydration candidate"
  );
  materialise(&p);
  (doomed, p)
}

/// Write every removal candidate to disk, from the plan's OWN render.
///
/// **THE FILES HAVE TO EXIST OR THE MEASUREMENT IS VACUOUS, AND THAT IS NOT A
/// THEORETICAL RISK -- IT WAS MEASURED HERE.** The first draft of
/// `one_refusal_covers_the_run_rather_than_one_per_file` left the tree empty and
/// asserted `report.dehydrated.is_empty()`. With no file on disk the per-file
/// gate cannot READ it, every removal is refused for an unrelated reason, and
/// `dehydrated` is empty no matter what the ship gate does -- so the mutation
/// that deletes the ship gate outright left that test GREEN. Found by the
/// mutation battery, not by review.
///
/// Writing the plan's own bytes matters for the same reason: a hand-written body
/// would be refused by AC-04.2's per-file gate instead, which is a DIFFERENT
/// gate, and this test would pass while proving nothing about this one.
fn materialise(p: &intentsvcs::organize::Plan) {
  for step in p.with(Action::Dehydrate) {
    let content = step
      .content
      .as_ref()
      .expect("a dehydration step carries the render");
    if let Some(parent) = step.path.parent() {
      std::fs::create_dir_all(parent).expect("fixture dirs");
    }
    std::fs::write(&step.path, content).expect("fixture write");
  }
}

#[test]
fn no_file_is_removed_while_a_precondition_is_unmet() {
  let fx = Fixture::new();
  let (doomed, p) = one_removal(
    &fx,
    declaring_thread(&[
      ("AC-00.3", AcKind::NonTest, AcState::Unsatisfied),
      ("AC-00.4", AcKind::NonTest, met("landed")),
    ]),
  );
  let report = p.apply(&|| "digest".to_string()).expect("apply returns");

  assert!(
    doomed.exists(),
    "the file must survive -- this is the whole criterion"
  );
  assert!(
    report.dehydrated.is_empty(),
    "nothing may be reported as dehydrated: {:?}",
    report.dehydrated
  );

  let refusal = report
    .refused
    .iter()
    .find(|e| matches!(e, OrganizeError::PreconditionsUnmet { .. }))
    .expect("the run must REPORT its refusal -- a silent skip is indistinguishable from having nothing to do");
  let rendered = refusal.to_string();
  assert!(rendered.contains("AC-00.3"), "got: {rendered}");
  assert!(
    rendered.contains("1 checked of 2 declared") || rendered.contains("2 checked of 2 declared"),
    "the refusal must print the denominator: {rendered}"
  );
}

#[test]
fn the_positive_control_removes_the_same_file_when_the_declaration_is_met() {
  // Without this, "the file survives" is produced equally by a working gate and
  // by a fixture that stopped producing a dehydration candidate at all.
  let fx = Fixture::new();
  let (doomed, p) = one_removal(&fx, gate_open());
  let report = p.apply(&|| "digest".to_string()).expect("apply returns");

  assert!(
    !doomed.exists(),
    "CONTROL FAILED: with every precondition met the file must actually be removed, or the refusing arm proves nothing"
  );
  assert_eq!(report.dehydrated, vec![doomed]);
  assert!(
    !report
      .refused
      .iter()
      .any(|e| matches!(e, OrganizeError::PreconditionsUnmet { .. })),
    "a met declaration must not refuse"
  );
}

#[test]
fn one_refusal_covers_the_run_rather_than_one_per_file() {
  // The unmet precondition is a property of the ESTATE. N copies of an
  // identical sentence would bury the per-file refusals that ARE about their
  // file.
  let fx = Fixture::new();
  let project = fx.project();
  let canon = canon_of(vec![
    declaring_thread(&[("AC-00.3", AcKind::NonTest, AcState::Unsatisfied)]),
    sample_thread("ST0002"),
    sample_thread("ST0003"),
  ]);
  let manifest = intentfiles::parse(MANIFEST).expect("manifest parses");
  let present: Vec<_> = vec![
    project.info_view("ST0002"),
    project.acceptance_view("ST0002"),
    project.info_view("ST0003"),
  ];
  let tree = TreeState {
    present: present.iter().cloned().collect(),
    ..Default::default()
  };
  let p = plan(
    &project,
    &canon,
    &manifest,
    &ctx(),
    &tree,
    "digest".to_string(),
  );
  assert!(
    p.with(Action::Dehydrate).count() >= 3,
    "the fixture must offer several removals for this to measure anything"
  );
  materialise(&p);
  let candidates: Vec<_> = p.with(Action::Dehydrate).map(|s| s.path.clone()).collect();

  let report = p.apply(&|| "digest".to_string()).expect("apply returns");
  let refusals = report
    .refused
    .iter()
    .filter(|e| matches!(e, OrganizeError::PreconditionsUnmet { .. }))
    .count();
  assert_eq!(refusals, 1, "one refusal for the run, not one per file");
  assert!(report.dehydrated.is_empty());
  for path in &candidates {
    assert!(path.exists(), "{path:?} must survive");
  }
}

#[test]
fn a_plan_with_no_removals_does_not_report_the_ship_gate() {
  // An always-on refusal occupies the slot where a signal would go. A pure
  // hydration is safe and additive by design, so it must run clean even with
  // every precondition unmet.
  let fx = Fixture::new();
  let project = fx.project();
  let canon = canon_of(vec![declaring_thread(&[(
    "AC-00.3",
    AcKind::NonTest,
    AcState::Unsatisfied,
  )])]);
  let manifest =
    intentfiles::parse("STEELTHREAD:ST0057\n\n# BEGIN INTENT\n# END INTENT\n").expect("parses");
  let p = plan(
    &project,
    &canon,
    &manifest,
    &ctx(),
    &TreeState::default(),
    "digest".to_string(),
  );
  assert_eq!(p.with(Action::Dehydrate).count(), 0);
  let report = p.apply(&|| "digest".to_string()).expect("apply returns");
  assert!(
    !report
      .refused
      .iter()
      .any(|e| matches!(e, OrganizeError::PreconditionsUnmet { .. })),
    "a run with nothing to remove has nothing for this gate to refuse"
  );
  assert!(
    !report.hydrated.is_empty(),
    "and the safe half must still happen -- hydration is additive"
  );
}

// ---------------------------------------------------------------------------
// THE ESTATE'S OWN DECLARATION
// ---------------------------------------------------------------------------

#[test]
fn the_shipped_criterion_carries_a_readable_declaration() {
  // The arms above all run against fixtures. This one reads the REAL AC-00.1
  // text out of this repo's canon, because a gate that parses every fixture
  // perfectly and cannot parse the one declaration that exists is a gate that
  // refuses for the wrong reason -- and it would refuse, so nothing else here
  // would notice.
  let canon_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("../../../../intent/.canon/st/ST0057.json");
  let Ok(raw) = std::fs::read_to_string(&canon_path) else {
    // Not a skip-in-disguise: this test asserts a property of THIS repo, and a
    // checkout without it is not a failing estate.
    eprintln!("no ST0057 canon at {canon_path:?}; nothing to check");
    return;
  };
  let thread: Thread = serde_json::from_str(&raw).expect("ST0057 canon parses");
  let criterion: &Criterion = thread
    .criteria
    .iter()
    .find(|c| c.id == "AC-00.1")
    .expect("ST0057 carries AC-00.1");

  let declared = preconditions::declared_in(&criterion.text)
    .expect("the shipped declaration must be readable by the gate that depends on it");
  assert!(
    declared.len() >= 2,
    "a declaration this short is likelier to be a parse landing in the wrong place: {declared:?}"
  );
  assert!(
    !declared.iter().any(|id| id == "AC-00.1"),
    "the declaring criterion must not declare ITSELF a precondition -- that is a gate that can never open"
  );
}
