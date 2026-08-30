//! **Issue 0047: every status value's DISPLAY SPELLING is v2's, and every
//! spelling has a witness that fails when it is renamed.**
//!
//! vc found this by running the canary the issue asked for instead of reading
//! for it. Mutating `ThreadStatus::Wip`'s spelling reds two surfaces; mutating
//! `ThreadStatus::NotStarted`'s reds NEITHER, mutation confirmed applied and
//! control clean. **Both tests read as intentional coverage of the vocabulary**,
//! which is what makes the gap expensive: the display strings are v2 parity, so
//! a silent rename is a parity break with no failing test and no visible symptom
//! until a consumer greps for a status.
//!
//! Two separate mechanisms produced one hole, and neither is a mistake anyone
//! would spot by reading the test:
//!
//! - `facade_st_wp.rs` pins the arm NEGATIVELY -- `!after.contains("Not
//!   Started")` -- so ANY rename satisfies it. A negative assertion about a
//!   string is satisfied by the string not existing, which is exactly what a
//!   rename produces.
//! - `cli_end_to_end.rs` stopped traversing the arm when the ratified machines
//!   moved `st new` onto `Triage`. **A machine ratification defanged a display
//!   assertion**, in a change that had nothing to do with display and could not
//!   have known the assertion existed.
//!
//! **So the fix is not another literal in another end-to-end test.** It is to
//! ask the question from a place that cannot stop traversing an arm: the
//! declared roster below, joined to the schema face, with the variant itself
//! obtained by DESERIALISING the canon value rather than from a hand-written
//! list. A new status value fails here on the day it enters the model, with
//! nobody having to remember this file exists -- the same posture
//! `transitions.rs` takes for the transition graph.
//!
//! **The spellings are literals here on purpose.** Asserting
//! `display() == ThreadStatus::Wip.display()` is a tautology, and asserting
//! against a constant lifted from the model would rename with it. The literal IS
//! the contract with v2, so it has to be written twice, in two files, and this
//! is the second place.

mod common;

use std::collections::BTreeSet;

use common::{Fixture, ctx, sample_thread};
use intentsvcs::ingest::Canon;
use intentsvcs::model::{ThreadStatus, WorkPackage, WpStatus};
use intentsvcs::views;
use serde_json::{Value, json};

/// v2's steel-thread vocabulary, keyed by the canon value so this joins to the
/// schema face without a translation layer that could itself drift.
const THREAD_SPELLINGS: &[(&str, &str)] = &[
  ("triage", "Triage"),
  ("not-started", "Not Started"),
  ("wip", "WIP"),
  ("hold", "On Hold"),
  ("completed", "Completed"),
  ("cancelled", "Cancelled"),
];

/// v2's work-package vocabulary. **Two spellings are shared with the thread
/// table and that is correct, not duplication**: v2 renders both with one
/// vocabulary, so the tables agreeing is the parity fact. Distinctness is
/// therefore asserted WITHIN an enum and never across the two.
const WP_SPELLINGS: &[(&str, &str)] = &[
  ("not-started", "Not Started"),
  ("wip", "WIP"),
  ("done", "Done"),
  ("cancelled", "Cancelled"),
];

/// The enum values one definition of the thread face declares.
fn face_values(definition: &str) -> Vec<String> {
  let text = intentsvcs::faces::face("thread.schema.json").expect("the thread face");
  let schema: Value = serde_json::from_str(&text).expect("a generated face is JSON");
  schema
    .get("$defs")
    .and_then(Value::as_object)
    .and_then(|defs| defs.get(definition))
    .and_then(|def| def.get("enum"))
    .and_then(Value::as_array)
    .unwrap_or_else(|| panic!("the thread face declares {definition} as a closed domain"))
    .iter()
    .map(|v| v.as_str().expect("an enum value is a string").to_string())
    .collect()
}

/// **The rosters are exactly the schema's value sets.**
///
/// Both directions: a value the model gained and this file has not is an
/// unwitnessed spelling, and a value this file carries that the model no longer
/// has is a row outliving its variant.
#[test]
fn the_rosters_are_exactly_the_status_values_the_model_declares() {
  for (definition, roster) in [
    ("ThreadStatus", THREAD_SPELLINGS),
    ("WpStatus", WP_SPELLINGS),
  ] {
    let declared: BTreeSet<String> = roster.iter().map(|(v, _)| v.to_string()).collect();
    let in_schema: BTreeSet<String> = face_values(definition).into_iter().collect();
    assert!(
      !in_schema.is_empty(),
      "{definition} yielded no values from the face, so every assertion in this file would hold vacuously"
    );
    assert_eq!(
      declared, in_schema,
      "{definition}: the spellings declared here are not the values the model has. A NEW value with no spelling is the whole of issue 0047 -- an arm no \
       assertion reaches -- and a spelling with no value is a row that outlived its variant"
    );
  }
}

/// **`display()` returns v2's spelling, for every value, checked against a
/// literal.**
///
/// This is the assertion that reds on ANY rename, which is the one that was
/// missing. The variant comes from deserialising the canon value, so there is no
/// hand-written variant list here to fall out of step with the enum.
#[test]
fn every_status_value_renders_v2s_spelling() {
  for (value, spelling) in THREAD_SPELLINGS {
    let status: ThreadStatus =
      serde_json::from_value(json!(value)).unwrap_or_else(|e| panic!("`{value}` is a status: {e}"));
    assert_eq!(
      status.display(),
      *spelling,
      "`{value}` renders as `{}` and v2 renders it `{spelling}`. The display strings are parity with v2's vocabulary, so a rename here is a parity break \
       whatever it improves about the wording",
      status.display()
    );
  }
  for (value, spelling) in WP_SPELLINGS {
    let status: WpStatus = serde_json::from_value(json!(value))
      .unwrap_or_else(|e| panic!("`{value}` is a work-package status: {e}"));
    assert_eq!(status.display(), *spelling, "`{value}` renders wrongly");
  }
}

/// **No two values of one enum render the same words.**
///
/// A rename that collapses two arms leaves every per-value assertion above
/// passing for the value it was renamed TO, and produces two statuses a reader
/// cannot tell apart. Cheap to check and invisible otherwise.
#[test]
fn no_two_values_of_one_enum_render_alike() {
  for (definition, roster) in [
    ("ThreadStatus", THREAD_SPELLINGS),
    ("WpStatus", WP_SPELLINGS),
  ] {
    let spellings: BTreeSet<&str> = roster.iter().map(|(_, s)| *s).collect();
    assert_eq!(
      spellings.len(),
      roster.len(),
      "{definition} renders two of its values with the same words, so a reader cannot tell them apart and the per-value assertions still pass"
    );
  }
}

/// **Every spelling REACHES a rendered view, and the other spellings do not.**
///
/// The half `transitions.rs`-style rosters cannot give: a table can be correct
/// while no surface calls `display()` at all. Held two-sided, because presence
/// alone is satisfied by a view that prints every status unconditionally, and
/// absence alone is satisfied by a view that renders nothing.
///
/// The fixture carries ONE thread with NO work packages, so the only status
/// vocabulary in the output is the thread's -- otherwise the work packages'
/// `Not Started` and `WIP` would satisfy the thread assertions and the exclusion
/// half would be measuring the wrong entity.
#[test]
fn every_thread_spelling_reaches_a_view_and_excludes_the_others() {
  for (value, spelling) in THREAD_SPELLINGS {
    let status: ThreadStatus =
      serde_json::from_value(json!(value)).expect("a declared status value");
    let fx = Fixture::new();
    let mut thread = sample_thread("ST0001");
    thread.status = status;
    thread.status_reason = None;
    thread.wps = Vec::new();

    let lines = status_lines(&fx, thread);
    assert!(
      renders(&lines, spelling),
      "a thread at `{value}` renders no `{spelling}` on any status-bearing line, so nothing calls display() for this arm and a rename of it would break no \
       test. Lines seen: {lines:?}"
    );
    for (other, other_spelling) in THREAD_SPELLINGS {
      if other == value || other_spelling == spelling {
        continue;
      }
      assert!(
        !renders(&lines, other_spelling),
        "a thread at `{value}` also renders `{other_spelling}`, so the presence check above would pass for a view that prints the whole vocabulary regardless \
         of state. Lines seen: {lines:?}"
      );
    }
  }
}

/// The work-package half, on a thread whose own spelling cannot collide with any
/// of the FOUR being discriminated.
///
/// **This fixture used `ThreadStatus::Cancelled` for exactly this isolation, and
/// it stopped being isolating on 2026-08-21 when `WpStatus` gained `Cancelled`.**
/// The two vocabularies were disjoint by accident of the WP enum being smaller,
/// and `status_lines` gathers EVERY status-bearing line in the rendered views --
/// the thread's included -- so the exclusion arm cannot tell a WP's spelling
/// from its thread's. It red the moment the vocabularies overlapped, which is
/// the tripwire working; `Triage` is chosen now because it is a thread-only
/// spelling and cannot be reached by any work package.
#[test]
fn every_work_package_spelling_reaches_a_view_and_excludes_the_others() {
  for (value, spelling) in WP_SPELLINGS {
    let status: WpStatus = serde_json::from_value(json!(value)).expect("a declared status value");
    let fx = Fixture::new();
    let mut thread = sample_thread("ST0001");
    thread.status = ThreadStatus::Triage;
    thread.status_reason = Some("so the thread's own spelling is none of the four".to_string());
    thread.wps = vec![WorkPackage {
      preamble: String::new(),
      seq: 1,
      title: "one work package".to_string(),
      scope: None,
      scope_legacy: None,
      status,
      status_reason: None,
      fiat: None,
      objective: "the objective".to_string(),
      body: String::new(),
    }];

    let lines = status_lines(&fx, thread);
    assert!(
      renders(&lines, spelling),
      "a work package at `{value}` renders no `{spelling}` on any status-bearing line. Lines seen: {lines:?}"
    );
    for (other, other_spelling) in WP_SPELLINGS {
      if other == value {
        continue;
      }
      assert!(
        !renders(&lines, other_spelling),
        "a work package at `{value}` also renders `{other_spelling}`. Lines seen: {lines:?}"
      );
    }
  }
}

/// **The classifier admits a rendered VALUE and refuses a column HEADER**, held
/// in both directions on the two lines that produced the difficulty.
///
/// `steel_threads.md` has a `Completed` column, so a thread in triage is
/// reported as rendering `Completed` by anything reading whole views. This is the
/// canary for the discriminator itself: without it, a classifier that quietly
/// stopped matching would make every exclusion above pass, and a classifier that
/// matched everything would make every presence check above pass.
#[test]
fn the_status_line_filter_admits_a_value_and_refuses_a_column_header() {
  let fx = Fixture::new();
  let mut thread = sample_thread("ST0001");
  thread.status = ThreadStatus::Triage;
  thread.status_reason = None;
  thread.wps = Vec::new();
  let lines = status_lines(&fx, thread);

  assert!(
    !lines.is_empty(),
    "the classifier admitted nothing, so every exclusion assertion in this file holds vacuously"
  );
  assert!(
    renders(&lines, "Triage"),
    "the thread's own status must survive the classifier: {lines:?}"
  );

  // **BOTH SHAPES, asserted separately, and this is the assertion a mutation
  // caught rather than a review.** A status is rendered in two places -- a
  // `status:` field line and an entity row in a table -- and either one alone
  // satisfies every presence check in this file. Stubbing the table-row half of
  // the classifier to `false` left all six tests GREEN, because the field line
  // covered for it: **redundant witnesses where one stops silently and the other
  // masks it, which is issue 0047's own shape one level down.** So each shape has
  // to be required, or half the classifier can rot unobserved.
  assert!(
    lines.iter().any(|l| l.starts_with("status:")),
    "no `status:` field line was admitted, so that half of the classifier has stopped matching and the table rows are covering for it: {lines:?}"
  );
  assert!(
    lines.iter().any(|l| l.starts_with('|')),
    "no entity table row was admitted, so that half of the classifier has stopped matching and the `status:` lines are covering for it: {lines:?}"
  );
  assert!(
    !renders(&lines, "Completed"),
    "`Completed` is a COLUMN HEADER in the thread index and this thread is in triage, so admitting it means the classifier is reading text about the vocabulary \
     as a rendering of it: {lines:?}"
  );
  assert!(
    !lines.iter().any(|l| l.contains("| ID")),
    "a header row reached the classifier: {lines:?}"
  );
}

/// Render every view for a one-thread canon and return only the lines that
/// RENDER A STATUS VALUE.
///
/// **The whole rendered text cannot answer this question, and finding that out
/// is what the exclusion half is worth.** A naive substring search over every
/// view reports `Completed` for a thread in triage, because
/// `steel_threads.md` has a column HEADER named `Completed`; and it reports
/// `Done` for every entity alive, because `acceptance.md` carries a legend
/// sentence defining the word. Both are text ABOUT the vocabulary rather than a
/// rendering of a value, and a checker that cannot tell those apart fails toward
/// whichever answer looks clean.
///
/// So the lines are classified by SHAPE, not filtered by an exemption list: a
/// `status:` field line, or a table row whose first cell is an entity id. A
/// header's first cell is a column name and a legend is not a table row, so both
/// fall outside without being named. `the_status_line_filter_admits_a_value_and_
/// refuses_a_column_header` holds the classifier to both directions.
fn status_lines(fx: &Fixture, thread: intentsvcs::model::Thread) -> Vec<String> {
  let canon = Canon {
    threads: vec![thread],
    issues: Vec::new(),
    sections: Vec::new(),
  };
  let views = views::render_all(&fx.project(), &canon, &ctx());
  assert!(
    !views.is_empty(),
    "the fixture rendered no views at all, so every assertion about their content is vacuous"
  );

  let mut lines = Vec::new();
  for view in &views {
    for raw in view.content.lines() {
      let line = raw.trim();
      if line.starts_with("status:") || is_entity_row(line) {
        lines.push(line.to_string());
      }
    }
  }
  lines
}

/// A markdown table row whose first cell names an entity, as against a header
/// row (first cell is a column name) or a separator.
fn is_entity_row(line: &str) -> bool {
  if !line.starts_with('|') {
    return false;
  }
  let Some(first) = line.split('|').nth(1) else {
    return false;
  };
  let cell = first.trim();
  let rest = cell
    .strip_prefix("ST")
    .or_else(|| cell.strip_prefix("WP-"))
    .unwrap_or("");
  !rest.is_empty() && rest.chars().all(|c| c.is_ascii_digit())
}

/// Whether any status-bearing line renders `spelling`.
fn renders(lines: &[String], spelling: &str) -> bool {
  lines.iter().any(|l| l.contains(spelling))
}
