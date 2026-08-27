//! **THE `st.*` OP VOCABULARY HAS TWO CONSUMERS, AND THEY FAIL IN OPPOSITE
//! DIRECTIONS -- ONE LOUD, ONE SILENT.** vc's Highlander finding F1, measured
//! 2026-08-27; this file is the assertion it asked for.
//!
//! The vocabulary has three spelling sites and no type: the edge table in
//! `transitions.rs`, the match in `facade::declared_list_edit`, and each verb
//! handing its own literal to `set_thread_status`. Every one is a bare
//! `&'static str`.
//!
//! **THE ASYMMETRY IS THE FINDING, NOT THE DUPLICATION.** An op that reaches
//! `check_transition` and is not in the edge table is REFUSED -- that direction
//! is safe, because the failure is an error someone reads. An op that reaches
//! `declared_list_edit` and is named in no arm falls through the wildcard and
//! **silently makes no list edit**: the transition succeeds, the declaration
//! quietly does not follow, and the only detector is a person noticing in the
//! field.
//!
//! **THAT IS NOT A HYPOTHETICAL. IT HAPPENED THREE TIMES IN ONE DAY**, and the
//! rulings are the receipts: `cce816a4` (`st.new` stops declaring), `6ff37c0f`
//! (the three unnamed ops), `26111785` (`st.hold` and `st.triage` both remove).
//! Each was one member of one vocabulary handled in one table and not the
//! other. The cost is already paid; this file is what stops the fourth.
//!
//! # What this file does NOT do, stated because both were considered
//!
//! **It introduces no `Op` enum.** That is a change of FORM, hv has not ruled
//! on it, and a type arriving by way of a test is a ruling nobody made.
//!
//! **It does not make the table status-keyed.** `st.triage` REMOVES and
//! `st.reinstate` does nothing, both landing on `not-started`, so op-keying is
//! load-bearing -- driven next door by
//! `lifecycle_verbs_edit_the_list::reinstate_touches_nothing_which_is_the_one_case_a_status_keyed_table_gets_wrong`.
//!
//! What it adds is a red the day a ninth op lands: no behaviour change, no new
//! type, and no population anyone maintains by hand.
//!
//! # Two derivations, because either alone can be vacuous
//!
//! The transition population is read at RUNTIME from the edge table, so it
//! cannot drift from the machine it describes. The facade population is read
//! from the SOURCE, because `st.new` is not a transition at all -- it creates a
//! thread rather than moving one -- and reaches the list path by its own door.
//! Neither derivation can see the other's members, so both are here.
//!
//! **AND A DERIVATION THAT RETURNS NOTHING PASSES EVERY `for` LOOP BELOW.**
//! That is the failure mode of a derived population, so each one is asserted
//! non-empty and the two are checked against each other before either is
//! trusted.
//!
//! # Mutations, measured -- each revert re-run to a green baseline
//!
//! | mutation                                          | reds                                                    |
//! | -------------------------------------------------- | ------------------------------------------------------- |
//! | `st.hold` dropped from the Remove arm              | both `..._named_in_an_arm` tests                        |
//! | a ninth edge `st.park` added to the machine        | the transition test AND the agreement test              |
//! | `is_code` made to count comment lines              | the reader control ONLY                                 |
//!
//! The second row is the scenario this file was written for, and it is worth
//! reading twice: a ninth op reds TWO tests with two different messages,
//! because `st.park` is in the machine and spelled nowhere in `facade.rs` --
//! unanswered by `declared_list_edit` AND unreachable by any verb. Two defects,
//! not one, and a single assertion would have reported whichever it happened to
//! check first.
//!
//! The third row is the one that keeps the other two honest. If the reader
//! counted comments, every op in this vocabulary would read as answered
//! whatever the arms said -- and the real file cannot tell those two readers
//! apart, because everything its comments name its arms also name.

use intentsvcs::transitions::{Disposition, find};
use std::collections::BTreeSet;

/// The file the arms live in, read from disk rather than `include_str!`.
///
/// `include_str!` would freeze it at THIS file's compile time, and the thing
/// being asserted is a property of the source as it stands -- the same reason
/// `flag_reachability.rs` gives for the same choice.
fn facade_source() -> String {
  let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/facade.rs");
  std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

/// A line that is code rather than prose.
///
/// **THIS PREDICATE IS THE WHOLE INSTRUMENT AND IT IS WHY THERE IS A CONTROL
/// BELOW.** `declared_list_edit`'s body is mostly comment, and those comments
/// name `st.hold`, `st.triage`, `st.new` and `st.reinstate` repeatedly -- so a
/// reader that did not exclude them would find every op "named in an arm"
/// whatever the arms actually said, and would go on passing after the arm was
/// deleted. A comment is not a mechanism.
///
/// Line comments only. This estate does not use `/* */`, and a reader that
/// tried to track block comments would be a parser pretending to be a grep.
fn is_code(line: &str) -> bool {
  !line.trim_start().starts_with("//")
}

/// Every `"st.<something>"` literal on a code line of `text`.
fn st_literals(text: &str) -> BTreeSet<String> {
  let mut out = BTreeSet::new();
  for line in text.lines().filter(|l| is_code(l)) {
    let mut rest = line;
    while let Some(open) = rest.find("\"st.") {
      let after = &rest[open + 1..];
      let Some(close) = after.find('"') else { break };
      out.insert(after[..close].to_string());
      rest = &after[close..];
    }
  }
  out
}

/// The body of a free function, from its signature to the `}` in column zero.
fn body_of(source: &str, signature: &str) -> String {
  let start = source
    .find(signature)
    .unwrap_or_else(|| panic!("{signature} is gone from facade.rs -- this test is about it"));
  let rest = &source[start..];
  let end = rest
    .find("\n}\n")
    .unwrap_or_else(|| panic!("no closing brace in column zero after {signature}"));
  rest[..end].to_string()
}

/// The ops named in `declared_list_edit`'s match arms, comments excluded.
fn arm_ops(source: &str) -> BTreeSet<String> {
  let body = body_of(source, "fn declared_list_edit");
  let arms: String = body
    .lines()
    .filter(|l| is_code(l) && l.contains("=>"))
    .collect::<Vec<_>>()
    .join("\n");
  st_literals(&arms)
}

/// The `st.*` ops the state machine declares, read from the table itself.
fn transition_ops() -> BTreeSet<String> {
  let Some(Disposition::State { edges, .. }) = find("Thread", "status").map(|f| &f.disposition)
  else {
    panic!("Thread.status is not a state machine in the transitions table");
  };
  edges.iter().map(|e| e.verb.to_string()).collect()
}

// ---------------------------------------------------------------------------
// THE CONTROL COMES FIRST, BECAUSE EVERYTHING BELOW IS THE READER'S ANSWER
// ---------------------------------------------------------------------------

/// **THE READER MUST NOT COUNT AN OP THAT ONLY APPEARS IN A COMMENT.**
///
/// Driven on a fixture rather than on the real file, because the real file
/// cannot distinguish the two answers: every op named in its comments is also
/// named in its arms, so a reader that read comments would agree with a correct
/// one on every line of it. **A control that the broken instrument would also
/// pass is decoration**, and this is the only input where the two disagree.
#[test]
fn the_arm_reader_ignores_an_op_named_only_in_a_comment() {
  let fixture = "\
fn declared_list_edit(op: &str) -> Option<ListAction> {
  match op {
    // `st.ghost` is discussed at length here and wired nowhere.
    // \"st.ghost\" => Some(ListAction::Add),
    \"st.real\" => Some(ListAction::Add),
    _ => None,
  }
}
";
  let found = arm_ops(fixture);
  assert!(
    found.contains("st.real"),
    "the reader lost a real arm: {found:?}"
  );
  assert!(
    !found.contains("st.ghost"),
    "the reader counted an op that only appears in a comment, so every assertion below \
     would pass on prose alone: {found:?}"
  );
}

/// **AND NEITHER POPULATION MAY BE EMPTY, BECAUSE AN EMPTY ONE PASSES EVERY
/// LOOP.**
///
/// The two derivations are independent -- one reads the edge table at runtime,
/// the other reads source text -- so this is also the only place they are
/// checked against each other. Every op the machine declares must appear in the
/// facade as a literal; if one does not, a transition exists that no verb can
/// reach, which is a different defect from the one below and would otherwise
/// hide inside a green.
#[test]
fn both_derivations_find_something_and_agree_where_they_overlap() {
  let transitions = transition_ops();
  let facade = st_literals(&facade_source());

  assert!(
    !transitions.is_empty(),
    "the edge table yielded no ops, so every assertion derived from it is vacuous"
  );
  assert!(
    !facade.is_empty(),
    "facade.rs yielded no op literals, so the source-derived assertions are vacuous"
  );

  let missing: Vec<_> = transitions.difference(&facade).collect();
  assert!(
    missing.is_empty(),
    "the state machine declares {missing:?}, and no verb in facade.rs spells them -- a \
     transition nothing can reach"
  );
}

// ---------------------------------------------------------------------------
// THE FINDING
// ---------------------------------------------------------------------------

/// **EVERY OP THE STATE MACHINE DECLARES HAS A DECLARED ANSWER, NOT A WILDCARD
/// ONE.**
///
/// Add, Remove, or an explicit `None` -- what matters is that the arm NAMES it.
/// A `_ => None` that answers for a lifecycle op cannot tell a ruled `None`
/// from an op nobody wired, and the two look identical from every side except
/// the manifest.
#[test]
fn every_transition_op_is_named_in_an_arm() {
  let arms = arm_ops(&facade_source());
  let unanswered: Vec<_> = transition_ops().difference(&arms).cloned().collect();
  assert!(
    unanswered.is_empty(),
    "{unanswered:?} reach `declared_list_edit` through the wildcard, so the transition \
     succeeds and the declaration silently does not follow. Name each one -- in the Add \
     arm, the Remove arm, or the explicit `None` arm with the reason beside it.\n\
     arms currently name: {arms:?}"
  );
}

/// The same property over the ops the FACADE spells, which is the wider set.
///
/// **`st.new` is why this test exists separately.** It is not a transition --
/// it creates a thread rather than moving one -- so the edge table has never
/// heard of it, and it reaches the list path by its own call to `edit_list`. A
/// check derived only from the machine would miss exactly the op hv's first
/// ruling of the day was about.
///
/// **THE QUESTION IS DELIBERATELY BROADER THAN _CAN IT REACH_.** Answering that
/// needs a call graph; this asks whether the literal is spelled in this file at
/// all, which is mechanical. The cost is that an `st.*` string used for some
/// other purpose in `facade.rs` would be asked for an arm it does not need --
/// acceptable, because the failure direction is a person reading a message,
/// which is the direction the wildcard does not have.
#[test]
fn every_st_op_the_facade_spells_is_named_in_an_arm() {
  let source = facade_source();
  let arms = arm_ops(&source);
  let unanswered: Vec<_> = st_literals(&source).difference(&arms).cloned().collect();
  assert!(
    unanswered.is_empty(),
    "{unanswered:?} are spelled in facade.rs and named in no arm of `declared_list_edit`, \
     so the wildcard answers for them in silence.\narms currently name: {arms:?}"
  );
}
