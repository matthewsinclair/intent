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
      .any(|f| f.class == FindingClass::ModelInconsistent
        && f.detail.contains("carry policy is for CLOSED threads")),
    "a live thread is fixed, not carried: {:?}",
    report.findings
  );
}
