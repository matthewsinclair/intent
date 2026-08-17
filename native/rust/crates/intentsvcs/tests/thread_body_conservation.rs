//! **`Thread.body`: the two-field shape applied one level up, and the 178
//! sections that were reaching neither `objective` nor `context`.**
//!
//! Measured across this project's own 56 threads against
//! `lib/templates/prj/st/ST####/info.md` @ `0b1b3b5b`: 283 sections, of which
//! 35 are byte-identical to the template and drop (`Acceptance` 12 of 12,
//! `Context for LLM` 20 of 41, `Related Steel Threads` 3 of 55) and 178 carry.
//! **44 headings appear exactly once each**, which is why the field is a
//! catch-all and not a named set: a model naming sections drops what it did not
//! foresee, and here that is most of the estate.
//!
//! **The collision was found BEFORE shipping this time.** `views::info`
//! generates `## Work Packages`, `## Acceptance` and `## Related Steel
//! Threads`. 8 threads author a `## Work Packages` section, none is
//! template-identical (v2 never generated that section), and all 8 have work
//! packages -- so without the deferral this field would have doubled that
//! heading on 8 threads the day it landed, which is exactly what shipped at the
//! work-package level and had to be fixed after the fact.

mod common;

use common::{Fixture, ctx};
use intentsvcs::legacy::{self, Verdict};
use intentsvcs::views;

const TEMPLATE_ACCEPTANCE: &str = "Acceptance Criteria and Acceptance Tests for this steel thread live in `acceptance.md` (the single source of truth). Do not restate ACs here -- see that file for the ratified completeness boundary and live status.";

fn estate(fixture: &Fixture, sections: &str) {
  fixture.write_file(
    "intent/.config/config.json",
    "{\"intent_version\":\"2.19.0\",\"project_name\":\"P\",\"author\":\"cc\",\"intent_dir\":\"intent\",\"languages\":[\"rust\"]}\n",
  );
  fixture.write_file(
    "intent/st/ST0001/info.md",
    &format!(
      "---\nstatus: Completed\ncreated: 20260816\n---\n\n# ST0001: A thread\n\n## Objective\n\nShip it.\n\n## Context\n\nBecause.\n{sections}"
    ),
  );
}

fn scan(fixture: &Fixture) -> legacy::Scan {
  legacy::scan(&fixture.project()).expect("scan the v2 estate")
}

#[test]
fn every_section_but_the_two_modelled_ones_is_carried_in_authored_order() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "\n## Scope\n\nThe quokka clause.\n\n## Diagnosis\n\nIt broke.\n",
  );
  let thread = &scan(&fixture).threads[0];

  assert_eq!(thread.objective, "Ship it.");
  assert_eq!(thread.context, "Because.");
  assert_eq!(
    thread.body, "## Scope\n\nThe quokka clause.\n\n## Diagnosis\n\nIt broke.",
    "verbatim, and in the order the author wrote them -- a section list plus an \
     order is a document; a section list alone is not"
  );
}

/// The template's own `## Acceptance` -- 12 of 12 identical on this estate --
/// is not authored prose and does not survive as though it were.
#[test]
fn a_section_byte_identical_to_the_template_drops_and_is_recorded() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    &format!("\n## Acceptance\n\n{TEMPLATE_ACCEPTANCE}\n\n## Scope\n\nReal prose.\n"),
  );
  let scan = scan(&fixture);

  assert_eq!(
    scan.threads[0].body, "## Scope\n\nReal prose.",
    "the scaffolding is gone and the authored section is untouched"
  );
  let dropped: Vec<&legacy::Disposition> = scan
    .dispositions
    .iter()
    .filter(|d| d.verdict == Verdict::Dropped)
    .collect();
  assert_eq!(dropped.len(), 1, "{:?}", scan.dispositions);
  assert_eq!(dropped[0].heading, "Acceptance");
  assert!(
    dropped[0]
      .reason
      .contains("lib/templates/prj/st/ST####/info.md")
      && dropped[0].reason.contains("0b1b3b5b"),
    "cited by path AND revision, so the drop set is re-derivable by someone who \
     was not there: {:?}",
    dropped[0]
  );
}

/// **THE NEGATIVE CONTROL.** One edited word is the difference between
/// scaffolding and prose, and a detector shown only positives cannot be told
/// from one that matches anything.
#[test]
fn an_edited_template_section_is_carried_not_dropped() {
  let fixture = Fixture::new();
  let edited = TEMPLATE_ACCEPTANCE.replace("Do not restate ACs here", "See ST0002 instead");
  estate(&fixture, &format!("\n## Acceptance\n\n{edited}\n"));
  let scan = scan(&fixture);

  assert!(
    scan.threads[0].body.contains("See ST0002 instead"),
    "an author touched it, so it is prose: {}",
    scan.threads[0].body
  );
  assert!(
    !scan
      .dispositions
      .iter()
      .any(|d| d.verdict == Verdict::Dropped),
    "and nothing was dropped: {:?}",
    scan.dispositions
  );
}

/// **The collision, caught before shipping.** 8 threads on this estate author a
/// `## Work Packages` section that the renderer also generates.
#[test]
fn an_authored_section_wins_over_the_generated_one_of_the_same_name() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "\n## Work Packages\n\n**WP02**: SKIPPED (folded into WP03).\n",
  );
  fixture.write_file(
    "intent/st/ST0001/WP/01/info.md",
    "---\nwp_id: WP-01\ntitle: \"A work package\"\nscope: Small\nstatus: Done\n---\n\n# WP-01: A work package\n\n## Objective\n\nDo it.\n",
  );
  let scan = scan(&fixture);
  let thread = &scan.threads[0];
  assert!(
    !thread.wps.is_empty(),
    "premise: the renderer would generate its own table"
  );

  let view = views::info(thread, &ctx());
  assert_eq!(
    view.lines().filter(|l| *l == "## Work Packages").count(),
    1,
    "one section, not two:\n{view}"
  );
  assert!(
    view.contains("**WP02**: SKIPPED (folded into WP03)."),
    "and the one that survives is the AUTHOR'S -- a decision record that exists \
     nowhere else. Keeping the generated table instead gives the same count and \
     destroys it:\n{view}"
  );

  let deferred: Vec<&legacy::Disposition> = scan
    .dispositions
    .iter()
    .filter(|d| d.verdict == Verdict::Deferred)
    .collect();
  assert_eq!(
    deferred.len(),
    1,
    "the deferral is recorded: {:?}",
    scan.dispositions
  );
  assert_eq!(deferred[0].heading, "Work Packages");
}

/// **A deferral is only a deferral if the generated section would otherwise
/// have been emitted**, and the first cut of this recorded 60 on the real
/// estate where 8 had happened.
///
/// 52 threads author `## Related Steel Threads`; `related` is empty out of
/// ingest and the renderer guards its block on that, so nothing stood down.
/// Recording those is a record of a decision never made -- the class the record
/// exists to prevent, arriving inside the record.
#[test]
fn a_generated_section_that_would_not_have_been_emitted_is_not_a_deferral() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "\n## Related Steel Threads\n\n- ST0002: the other one\n",
  );
  let scan = scan(&fixture);

  assert!(
    scan.threads[0].related.is_empty(),
    "premise: ingest does not populate `related`, so the renderer's block is skipped"
  );
  assert!(
    scan.threads[0].body.contains("## Related Steel Threads"),
    "the authored section is carried: {}",
    scan.threads[0].body
  );
  assert!(
    !scan
      .dispositions
      .iter()
      .any(|d| d.verdict == Verdict::Deferred),
    "nothing deferred, because nothing was going to be generated: {:?}",
    scan.dispositions
  );

  let view = views::info(&scan.threads[0], &ctx());
  assert_eq!(
    view
      .lines()
      .filter(|l| *l == "## Related Steel Threads")
      .count(),
    1,
    "and the view still carries exactly one:\n{view}"
  );
}

/// The control on the whole rule: a thread with nothing beyond the two modelled
/// sections has an empty body, drops nothing and defers nothing. Without it the
/// tests above pass against an implementation that records constantly.
#[test]
fn a_thread_with_only_the_modelled_sections_records_nothing() {
  let fixture = Fixture::new();
  estate(&fixture, "");
  let scan = scan(&fixture);

  assert!(
    scan.threads[0].body.is_empty(),
    "{:?}",
    scan.threads[0].body
  );
  assert!(scan.dispositions.is_empty(), "{:?}", scan.dispositions);
}
