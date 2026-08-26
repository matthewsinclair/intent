//! **The marked-legacy `scope` form** (vc's ruling in `data-model.md`, on hv's
//! carry policy) -- and the `TShirt::M` substitution it replaced.
//!
//! v2 read `scope` as free text, and this repository's own corpus carries
//! eleven spellings for six sizes. Ten are the same six values written
//! differently and canonicalise losslessly. **The eleventh decides the rule**:
//! `Medium-Large`, one work package, in a CLOSED thread. It sits BETWEEN two
//! enum members, so all three obvious moves are forbidden at once --
//! normalising it is a guess, blocking violates lossless-by-carrying for a
//! closed thread, and dropping it is loss.
//!
//! **What the migrator did before this was worse than any of the three.** Both
//! the unparseable value and a work package with no `scope:` line at all fell
//! back to `TShirt::M`. Neither is a medium. One is a value somebody recorded
//! and one is a field nobody wrote, and answering both with the same confident
//! size is the answer-from-partial-evidence habit v3 exists to end -- silently,
//! during a migration, on data whose original was about to be replaced.
//!
//! **THE FORM HAS TWO INSTANCES AND ONLY ONE WAS GUARDED.** `AcceptanceTest`
//! carries `file` / `legacy` in exactly the shape `WorkPackage` carries
//! `scope` / `scope_legacy` -- two optional fields the model generates
//! independently, four constructible combinations, three that mean anything.
//! The second set is at the foot of this file. **Guarding one of two
//! structurally identical invariants is not half the coverage; the uncovered
//! one is no less likely to break, only less likely to be looked at** -- and
//! the file was named after the instance that happened to be built first.

mod common;

use common::Fixture;
use intentsvcs::finding::FindingClass;
use intentsvcs::legacy;
use intentsvcs::model::{Legacy, TShirt};

/// Write a v2 estate: one thread at `status`, one work package whose
/// frontmatter carries `scope_line` verbatim (or nothing, when it is empty).
fn v2_estate(fixture: &Fixture, status: &str, scope_line: &str) {
  fixture.write_file(
    "intent/.config/config.json",
    "{\"intent_version\":\"2.19.0\",\"project_name\":\"P\",\"author\":\"cc\",\"intent_dir\":\"intent\",\"languages\":[\"rust\"]}\n",
  );
  fixture.write_file(
    "intent/st/ST0001/info.md",
    &format!(
      "---\nverblock: \"16 Aug 2026:v0.1: cc - x\"\nintent_version: 2.19.0\nstatus: {status}\nslug: a-slug\ncreated: 20260816\ncompleted:\n---\n\n# ST0001: A thread\n\n## Objective\n\nShip it.\n"
    ),
  );
  let front = if scope_line.is_empty() {
    "---\ntitle: A work package\nstatus: Done\n---\n".to_string()
  } else {
    format!("---\ntitle: A work package\nscope: {scope_line}\nstatus: Done\n---\n")
  };
  fixture.write_file(
    "intent/st/ST0001/WP/01/info.md",
    &format!("{front}\n# WP01: A work package\n\n## Objective\n\nDo it.\n"),
  );
}

fn scan(fixture: &Fixture) -> legacy::Scan {
  legacy::scan(&fixture.project()).expect("scan the v2 estate")
}

/// **A value outside the enum is CARRIED, verbatim, on a closed thread.**
#[test]
fn an_unmappable_scope_on_a_closed_thread_carries_rather_than_blocking() {
  let fixture = Fixture::new();
  v2_estate(&fixture, "Completed", "Medium-Large");
  let scan = scan(&fixture);

  let wp = &scan.threads[0].wps[0];
  assert_eq!(
    wp.scope, None,
    "the enum column holds NOTHING rather than a nearby size -- `Medium-Large` sits between two \
     members and picking one is the guess the carry exists to avoid"
  );
  assert_eq!(
    wp.scope_legacy,
    Some(Legacy {
      raw: "Medium-Large".to_string()
    }),
    "and the value somebody actually wrote is carried byte for byte"
  );

  assert!(
    scan.residue.is_empty(),
    "a CLOSED thread converts lossless-by-carrying, so this must not block: {:?}",
    scan.residue
  );
  assert_eq!(
    scan.carried.len(),
    1,
    "it is reported as carried, never silently counted as ordinary: {:?}",
    scan.carried
  );
  assert_eq!(scan.carried[0].class, FindingClass::UnknownScope);
  assert!(
    scan.carried[0].class.remedy().contains("nothing to do"),
    "and the remedy tells a reader of a closed thread there is nothing for them to fix: {}",
    scan.carried[0].class.remedy()
  );
}

/// **The same value on a LIVE thread BLOCKS**, which is the other half of the
/// ruled policy and the half a single-severity model cannot express.
///
/// Asserted with the SAME defect in both kinds of thread, so the test cannot
/// pass by treating everything alike.
#[test]
fn the_same_unmappable_scope_on_a_live_thread_blocks() {
  let fixture = Fixture::new();
  v2_estate(&fixture, "WIP", "Medium-Large");
  let scan = scan(&fixture);

  assert_eq!(
    scan.residue.len(),
    1,
    "a LIVE thread stays BLOCKED-until-clean: {:?}",
    scan.residue
  );
  assert!(
    scan.carried.is_empty(),
    "and it is not ALSO reported as carried -- one fact, one bucket: {:?}",
    scan.carried
  );
}

/// **A work package with no `scope:` line gets NOTHING, not a medium.**
///
/// The anti-guess arm, and the one that would have been green before this
/// change while the model held a value nobody wrote. Eight work packages in
/// this repository's own ST0023 predate the frontmatter convention entirely.
#[test]
fn a_work_package_that_never_recorded_a_scope_is_not_given_one() {
  let fixture = Fixture::new();
  v2_estate(&fixture, "Completed", "");
  let scan = scan(&fixture);

  let wp = &scan.threads[0].wps[0];
  assert_eq!(
    wp.scope, None,
    "no size was recorded, so none is reported -- substituting M here is the defect this replaced"
  );
  assert_eq!(
    wp.scope_legacy, None,
    "and there is no legacy value either: nothing was written, so there is nothing to carry. \
     ABSENT IS NOT INVALID, and it is not a carried value pretending to be one"
  );
  assert!(
    !scan
      .carried
      .iter()
      .chain(scan.residue.iter())
      .any(|f| f.class == FindingClass::UnknownScope),
    "an absent field is not an out-of-vocabulary value: {:?}",
    scan.carried
  );
}

/// **A carried value renders AS ITSELF AND MARKED**, which is what "stays
/// visible as legacy rather than silently canonicalised into a lie" means at
/// the point a reader meets it.
#[test]
fn the_three_scope_states_render_as_three_different_things() {
  let fixture = Fixture::new();
  v2_estate(&fixture, "Completed", "Medium-Large");
  let carried = scan(&fixture).threads[0].wps[0].scope_display();
  assert_eq!(carried, "Medium-Large (legacy)");

  let fixture = Fixture::new();
  v2_estate(&fixture, "Completed", "Large");
  assert_eq!(scan(&fixture).threads[0].wps[0].scope_display(), "L");

  let fixture = Fixture::new();
  v2_estate(&fixture, "Completed", "");
  assert_eq!(
    scan(&fixture).threads[0].wps[0].scope_display(),
    "",
    "a scope nobody recorded renders as nothing -- inventing a size for the column is the \
     substitution the whole form exists to remove"
  );
}

/// **Rescoping RESOLVES the carry**, so the two never coexist afterwards.
#[test]
fn rescoping_a_carried_work_package_clears_the_legacy_value() {
  let fixture = Fixture::new();
  let mut thread = common::sample_thread("ST0001");
  thread.wps[0].scope = None;
  thread.wps[0].scope_legacy = Some(Legacy {
    raw: "Medium-Large".to_string(),
  });
  fixture.write_thread(&thread);

  let mut facade = fixture.facade();
  let seq = thread.wps[0].seq;
  facade
    .wp_rescope("ST0001", seq, TShirt::L)
    .expect("rescope");

  let wp = facade.wp_show("ST0001", seq).expect("wp");
  assert_eq!(wp.scope, Some(TShirt::L));
  assert_eq!(
    wp.scope_legacy, None,
    "leaving it would keep `Medium-Large` beside a deliberate `L` forever, and a reader could not \
     tell which one the project meant"
  );
}

/// **The no-op is decidable from the two states alone in every LEGAL state, and
/// the clause that says otherwise is reachable from exactly one state: the
/// contradiction.**
///
/// `Facade::wp_rescope` reports a self-loop when `scope == Some(wanted) &&
/// scope_legacy.is_none()`, and the dispatch row explains the second half as
/// "with a carry present, the same from and the same to IS a movement, because
/// resolving the carry is the change". **That reading assumes a carry can sit
/// beside a recorded size, and in a valid model it cannot.** The three legal
/// combinations are settled (a size, no carry), carried (no size, a carry) and
/// absent (neither) -- and in the last two `from` is `"<raw> (legacy)"` or the
/// empty string, so it is never equal to a size and the same-state question
/// never arises. The clause's inputs are only jointly reachable in the fourth
/// combination, which is the one `doctor` reports as residue.
///
/// **So the clause is right and its stated reason is not.** What it actually
/// buys is a REPAIR: `wp rescope L` on a package recording `L` and carrying
/// `Medium-Large` clears the carry and reports a movement, which is exactly
/// right because the operator is deciding. Without it the command would answer
/// `already L`, leave the residue in place, and there would be no verb in the
/// tool that could clear it -- an operator's only route back would be editing
/// the file the CLI exists to own, which is the shape issue 0052 was filed on.
///
/// Asserted as a trio in one test on purpose: each arm is meaningless without
/// the others, because the property is that the three states are told APART.
#[test]
fn the_self_loop_is_decidable_from_the_states_except_where_the_model_contradicts_itself() {
  // 1. SETTLED -- a size, no carry. The same size is a no-op, and it names the
  //    size rather than the work package's status.
  let fixture = Fixture::new();
  let mut thread = common::sample_thread("ST0001");
  thread.wps[0].scope = Some(TShirt::L);
  thread.wps[0].scope_legacy = None;
  fixture.write_thread(&thread);
  let seq = thread.wps[0].seq;
  let mut facade = fixture.facade();
  assert_eq!(
    facade
      .wp_rescope("ST0001", seq, TShirt::L)
      .expect("a self-loop is legal")
      .already(),
    Some("L"),
    "a settled package asked for the size it already has has nothing to do"
  );

  // 2. CARRIED -- no size, a carry. `from` is the marked legacy display, so this
  //    is a movement whatever size is asked for. There is no same-state case to
  //    reach: the carry is not one of the six.
  let fixture = Fixture::new();
  let mut thread = common::sample_thread("ST0001");
  thread.wps[0].scope = None;
  thread.wps[0].scope_legacy = Some(Legacy {
    raw: "Medium-Large".to_string(),
  });
  fixture.write_thread(&thread);
  let mut facade = fixture.facade();
  assert_eq!(
    facade
      .wp_rescope("ST0001", seq, TShirt::L)
      .expect("rescope")
      .already(),
    None,
    "resolving a carry is a movement, and `from` was never a size"
  );

  // 3. THE CONTRADICTION -- both, which is the only state that reaches the
  //    `scope_legacy.is_none()` half of the guard. Same size in, same size out,
  //    and it is still a movement because the carry goes.
  let fixture = Fixture::new();
  let mut thread = common::sample_thread("ST0001");
  thread.wps[0].scope = Some(TShirt::L);
  thread.wps[0].scope_legacy = Some(Legacy {
    raw: "Medium-Large".to_string(),
  });
  fixture.write_thread(&thread);
  let mut facade = fixture.facade();
  assert_eq!(
    facade
      .wp_rescope("ST0001", seq, TShirt::L)
      .expect("rescope")
      .already(),
    None,
    "the same size in and the same size out, and reporting `already L` here would leave a residue \
     `doctor` names with no verb able to clear it"
  );
  let wp = facade.wp_show("ST0001", seq).expect("wp");
  assert_eq!(wp.scope, Some(TShirt::L));
  assert_eq!(
    wp.scope_legacy, None,
    "and the contradiction is REPAIRED -- this is what the clause is for, rather than the \
     carry-resolution case the row describes, which cannot occur"
  );
}

/// **`doctor` reports the one combination that is a contradiction.**
///
/// Two optional fields permit four combinations and only three mean anything.
/// Carrying BOTH says the work package is an `L` and also that its size could
/// not be expressed -- and a shape that can represent a contradiction
/// eventually stores one.
#[test]
fn a_work_package_carrying_both_a_scope_and_a_legacy_one_is_reported() {
  let fixture = Fixture::new();
  let mut thread = common::sample_thread("ST0001");
  thread.status = intentsvcs::model::ThreadStatus::Completed;
  thread.completed = Some("2026-08-16".to_string());
  thread.wps[0].scope = Some(TShirt::L);
  thread.wps[0].scope_legacy = Some(Legacy {
    raw: "Medium-Large".to_string(),
  });
  fixture.write_thread(&thread);

  let project = fixture.project();
  let report = intentsvcs::facade::Facade::doctor(&project, &common::facade_ctx(), None);
  assert!(
    report
      .findings
      .iter()
      .any(|f| f.class == FindingClass::ModelInconsistent
        && f.detail.contains("alternatives, not a pair")),
    "the contradiction must be named: {:?}",
    report.findings
  );
}

/// **And a carried scope on a LIVE thread is a defect however well-formed.**
///
/// The carry policy is a rule about WHERE a legacy value may appear, not about
/// its shape. Ingest applies the closed/live split at migration time; this
/// catches one that arrived any other way, including by hand.
#[test]
fn a_carried_scope_on_a_live_thread_is_reported() {
  let fixture = Fixture::new();
  let mut thread = common::sample_thread("ST0001");
  thread.status = intentsvcs::model::ThreadStatus::Wip;
  thread.completed = None;
  thread.wps[0].scope = None;
  thread.wps[0].scope_legacy = Some(Legacy {
    raw: "Medium-Large".to_string(),
  });
  fixture.write_thread(&thread);

  let project = fixture.project();
  let report = intentsvcs::facade::Facade::doctor(&project, &common::facade_ctx(), None);
  assert!(
    report
      .findings
      .iter()
      .any(|f| f.class == FindingClass::Advisory && f.detail.contains("ADVISORY, not a refusal")),
    "a live thread is fixed, not carried: {:?}",
    report.findings
  );
}

// ---------------------------------------------------------------------------
// The same form on `AcceptanceTest`: `file` / `legacy`
// ---------------------------------------------------------------------------

/// **An AT row that cites a test file AND carries a legacy reference is a
/// contradiction**, exactly as a work package carrying both scopes is.
///
/// It says the reference was migrated into the 0017 grammar and also that it
/// could not be expressed in it. **That is the state `at lint --fix` produced
/// when it destroyed one end of a two-ended migration**, which is the whole
/// reason `Legacy` exists rather than a hypothetical: a `::name` citation or a
/// multi-file list has no 0017 spelling, and neither is guessed at.
#[test]
fn an_acceptance_test_carrying_both_a_file_and_a_legacy_reference_is_reported() {
  let fixture = Fixture::new();
  let mut thread = common::sample_thread("ST0001");
  thread.status = intentsvcs::model::ThreadStatus::Completed;
  thread.completed = Some("2026-08-15".to_string());
  let at = thread
    .tests
    .iter_mut()
    .find(|t| t.file.is_some())
    .expect("the fixture has a test-kind row with a file");
  at.legacy = Some(Legacy {
    raw: "tests/foo_test.exs::the name".to_string(),
  });
  let id = at.id.clone();
  fixture.write_thread(&thread);

  let project = fixture.project();
  let report = intentsvcs::facade::Facade::doctor(&project, &common::facade_ctx(), None);
  assert!(
    report
      .findings
      .iter()
      .any(|f| f.class == FindingClass::ModelInconsistent
        && f.detail.contains(&id)
        && f.detail.contains("alternatives, not a pair")),
    "the contradiction must be named, and named on the ROW -- a finding that says a thread is inconsistent without saying which test row cannot be acted on: {:?}",
    report.findings
  );
}

/// **And the carry policy is the same rule here**: a legacy reference belongs
/// to a CLOSED thread, because hv ratified that closed threads convert
/// lossless-by-carrying while live ones stay blocked-until-clean.
///
/// A live thread's AT row is FIXED, not carried -- the test it points at is
/// still being written, so there is nothing to preserve verbatim.
#[test]
fn a_carried_reference_on_a_live_thread_is_reported() {
  let fixture = Fixture::new();
  let mut thread = common::sample_thread("ST0001");
  thread.status = intentsvcs::model::ThreadStatus::Wip;
  thread.completed = None;
  let at = thread
    .tests
    .iter_mut()
    .find(|t| t.file.is_some())
    .expect("the fixture has a test-kind row with a file");
  at.file = None;
  at.legacy = Some(Legacy {
    raw: "tests/foo_test.exs::the name".to_string(),
  });
  let id = at.id.clone();
  fixture.write_thread(&thread);

  let project = fixture.project();
  let report = intentsvcs::facade::Facade::doctor(&project, &common::facade_ctx(), None);
  assert!(
    report
      .findings
      .iter()
      .any(|f| f.class == FindingClass::Advisory
        && f.detail.contains(&id)
        && f.detail.contains("ADVISORY, not a refusal")),
    "a live thread is fixed, not carried: {:?}",
    report.findings
  );
}

// ---------------------------------------------------------------------------
// The covers clause: a parenthetical qualifier is prose, not part of the id
// ---------------------------------------------------------------------------

/// **A qualifier read as part of the id MANUFACTURES A BROKEN REFERENCE AGAINST
/// A CLEAN ESTATE**, which is the worst kind of finding: it sends an operator
/// to repair a link that is not broken, in a file where the criterion it names
/// is sitting a few lines away.
///
/// Three rows fleet-wide, all on Baize, **and ZERO on this estate** -- so the
/// fixture is constructed rather than captured, and the mixed row below is
/// vc's, measured, not invented to be thorough:
///
/// ```text
/// AT-04.2  covers AC-04.1 (render contract)
/// AT-06.2  covers AC-06.2 (revoke + last-superadmin guard)
/// AT-09.3  covers AC-09.3, AC-09.1 (render)          <-- MIXED
/// ```
///
/// **The mixed row is what forces the note to be KEYED.** `AT-09.3` covers two
/// criteria and only one carries a qualifier, so a bare `render` appended to
/// the note loses the association and lands on the wrong criterion half the
/// time -- a coin flip, silently, in a migration.
fn contract(fixture: &Fixture, rows: &str) {
  fixture.write_file(
    "intent/.config/config.json",
    "{\"intent_version\":\"2.19.0\",\"project_name\":\"P\",\"author\":\"cc\",\"intent_dir\":\"intent\",\"languages\":[\"rust\"]}\n",
  );
  fixture.write_file(
    "intent/st/ST0001/info.md",
    "---\nstatus: Completed\ncreated: 20260816\n---\n\n# ST0001: A thread\n\n## Objective\n\nShip it.\n",
  );
  fixture.write_file(
    "intent/st/ST0001/acceptance.md",
    &format!("# ST0001 -- Acceptance\n\n## Criteria\n\n- AC-09.1 the first -- status: satisfied\n- AC-09.3 the third -- status: satisfied\n\n## Tests\n\n{rows}\n"),
  );
}

#[test]
fn a_parenthetical_qualifier_leaves_the_id_resolvable() {
  let fixture = Fixture::new();
  contract(
    &fixture,
    "- AT-09.1 `tests/a.rs` -- covers AC-09.1 (render contract) -- status: green",
  );
  let scan = scan(&fixture);
  let test = &scan.threads[0].tests[0];

  assert_eq!(
    test.covers,
    vec!["AC-09.1".to_string()],
    "the id is the id; the parenthetical is not part of it"
  );
  assert!(
    !scan
      .residue
      .iter()
      .chain(scan.carried.iter())
      .any(|f| f.class == FindingClass::BrokenReference),
    "and no broken reference is manufactured against a criterion that is \
     sitting in the same file: {:?}",
    scan.carried
  );
  assert_eq!(
    test.note.as_deref(),
    Some("AC-09.1: render contract"),
    "the qualifier is KEPT, keyed to the criterion it qualifies"
  );
}

/// **vc's mixed row, and the reason the note is keyed rather than appended.**
#[test]
fn a_qualifier_on_one_of_two_covered_criteria_names_which_one() {
  let fixture = Fixture::new();
  contract(
    &fixture,
    "- AT-09.3 `tests/b.rs` -- covers AC-09.3, AC-09.1 (render) -- status: green",
  );
  let scan = scan(&fixture);
  let test = &scan.threads[0].tests[0];

  assert_eq!(
    test.covers,
    vec!["AC-09.3".to_string(), "AC-09.1".to_string()],
    "both ids resolve and the order the author wrote them survives"
  );
  assert_eq!(
    test.note.as_deref(),
    Some("AC-09.1: render"),
    "keyed to AC-09.1 and NOT to AC-09.3 -- a bare `render` here is the wrong \
     criterion on a coin flip"
  );
}

/// The re-filing is declared, and on this one the record is the ONLY evidence:
/// a census that hashes the whole authored row cannot see text moving between
/// two fields of it, in either direction.
#[test]
fn the_refiling_is_declared_because_nothing_else_can_observe_it() {
  let fixture = Fixture::new();
  contract(
    &fixture,
    "- AT-09.1 `tests/a.rs` -- covers AC-09.1 (render contract) -- status: green",
  );
  let scan = scan(&fixture);

  let refiled: Vec<&intentsvcs::legacy::Disposition> = scan
    .dispositions
    .iter()
    .filter(|d| d.verdict == intentsvcs::legacy::Verdict::Refiled)
    .collect();
  assert_eq!(refiled.len(), 1, "{:?}", scan.dispositions);
  assert_eq!(refiled[0].heading, "AT-09.1", "the row it happened on");
  assert!(
    refiled[0].reason.contains("AC-09.1") && refiled[0].reason.contains("render contract"),
    "the record carries the authored text, so the move is checkable by \
     someone who was not there: {:?}",
    refiled[0]
  );
}

/// **THE CONTROL.** An ordinary covers clause must be untouched and must record
/// nothing -- otherwise the rule above is indistinguishable from one that
/// rewrites every row it sees.
#[test]
fn an_ordinary_covers_clause_is_untouched_and_records_nothing() {
  let fixture = Fixture::new();
  contract(
    &fixture,
    "- AT-09.1 `tests/a.rs` -- covers AC-09.1, AC-09.3 -- status: green -- the authored note",
  );
  let scan = scan(&fixture);
  let test = &scan.threads[0].tests[0];

  assert_eq!(
    test.covers,
    vec!["AC-09.1".to_string(), "AC-09.3".to_string()]
  );
  assert_eq!(
    test.note.as_deref(),
    Some("the authored note"),
    "an existing note is not touched when there is nothing to fold into it"
  );
  assert!(
    !scan
      .dispositions
      .iter()
      .any(|d| d.verdict == intentsvcs::legacy::Verdict::Refiled),
    "nothing was re-filed, so nothing is declared: {:?}",
    scan.dispositions
  );
}
