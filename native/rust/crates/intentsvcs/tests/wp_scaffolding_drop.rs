//! **A section byte-identical to the template that created the file was not
//! written by an author, and carrying it FABRICATES AUTHORSHIP** (vc's ruling,
//! and the half I had backwards -- carrying is not the conservative option).
//!
//! It was already shipping a visible defect. Measured by running the migrator
//! on a sacrificial copy of this estate: **40 of 140 migrated work-package
//! views carried TWO `## Acceptance` sections**, the carried one and the one
//! `views::wp_info` generates, saying the same thing in different words:
//!
//! ```text
//! carried:   ... live in the steel thread's `acceptance.md`, under the `WP-01`
//!            heading (single source of truth). Do not restate ACs here.
//! generated: ... live in `ST0047/acceptance.md`, under the `WP-01` heading --
//!            the single source of truth. This cover never restates them.
//! ```
//!
//! 104 across the captured fleet -- Utilz 11, Baize 53, Intent 40 -- and **every
//! one of them is `Acceptance`; no other heading doubles anywhere** (vc, by a
//! different method on a different copy). It is one section, and it is the one
//! section whose template body carries a substituted placeholder.
//!
//! **THE PLACEHOLDER IS WHY THE OBVIOUS RULE MISSES ALL OF THEM.**
//! `bin/intent_wp:113` creates every work package with
//! `sed -e "s/WP-NN/WP-$WP_NUM/g"`, so against the RAW template `## Acceptance`
//! matches 0 of 40 while `Deliverables` and `Dependencies` match 20 each. The
//! artefact these files were copied from is the template WITH the substitution
//! applied, which is citable to a line of shell rather than inferred from
//! shape -- and the raw template is a source no file was ever a copy of.
//!
//! **This is NOT a heading-name rule.** `## Acceptance` is a legitimate
//! authored section elsewhere in this estate, so the drop is keyed on bytes and
//! never on the name, and every failure mode goes toward CARRYING: a file
//! seeded from an older template generation fails the match and is kept.
//!
//! **The negative control below is vc's condition and it is the point of the
//! file.** Against this estate the detector hits 40 of 40 -- a 100% rate
//! against 15-16% for the sections that do not double -- and **a detector that
//! has only ever been shown positives is indistinguishable from one that
//! matches anything.** `an_edited_acceptance_section_is_carried_not_dropped` is
//! the only test here that can tell those two apart.

mod common;

use common::{Fixture, ctx};
use intentsvcs::{legacy, views};

/// The v2 work-package template body, verbatim from
/// `lib/templates/prj/st/WP/info.md` at revision `0b1b3b5b`.
///
/// **Held here as a literal rather than imported from the parser**, so the test
/// and the implementation do not agree by construction. A test that asked the
/// code for its own definition of scaffolding would pass against any
/// definition, including a wrong one.
const TEMPLATE: &str = "\
# WP-NN: [Title]

## Objective

[Clear statement of what this work package aims to accomplish]

## Deliverables

- [List of concrete deliverables]

## Acceptance

Acceptance Criteria for this work package live in the steel thread's `acceptance.md`, under the `WP-NN` heading (single source of truth). Do not restate ACs here.

## Dependencies

- [List any dependencies on other WPs or external factors]
";

/// One work package as `intent wp new` would have written it: the template with
/// `WP-NN` substituted for this sequence number, plus whatever `extra` the
/// author added afterwards.
fn wp(fixture: &Fixture, seq: u32, extra: &str) {
  let body = TEMPLATE.replace("WP-NN", &format!("WP-{seq:02}"));
  fixture.write_file(
    &format!("intent/st/ST0001/WP/{seq:02}/info.md"),
    &format!(
      "---\nwp_id: WP-{seq:02}\ntitle: \"A work package\"\nscope: Small\nstatus: Done\n---\n\n{body}{extra}"
    ),
  );
}

fn estate(fixture: &Fixture) {
  fixture.write_file(
    "intent/.config/config.json",
    "{\"intent_version\":\"2.19.0\",\"project_name\":\"P\",\"author\":\"cc\",\"intent_dir\":\"intent\",\"languages\":[\"rust\"]}\n",
  );
  fixture.write_file(
    "intent/st/ST0001/info.md",
    "---\nstatus: Completed\ncreated: 20260816\n---\n\n# ST0001: A thread\n\n## Objective\n\nShip it.\n",
  );
}

fn scan(fixture: &Fixture) -> legacy::Scan {
  legacy::scan(&fixture.project()).expect("scan the v2 estate")
}

fn body_of(scan: &legacy::Scan, seq: u32) -> String {
  scan.threads[0]
    .wps
    .iter()
    .find(|w| w.seq == seq)
    .unwrap_or_else(|| panic!("work package {seq} is missing from the scan"))
    .body
    .clone()
}

// ---------------------------------------------------------------------------
// The drop
// ---------------------------------------------------------------------------

#[test]
fn the_templates_own_sections_are_dropped_and_each_one_is_recorded() {
  let fixture = Fixture::new();
  estate(&fixture);
  wp(&fixture, 1, "");
  let scan = scan(&fixture);

  let body = body_of(&scan, 1);
  assert!(
    body.trim().is_empty(),
    "a work package that is nothing but the template has no authored body at \
     all, and every section of it should have been dropped:\n{body}"
  );

  // Per section, never a count -- a total reconciles arithmetically and tells
  // nobody which section went.
  let headings: Vec<&str> = scan.dropped.iter().map(|d| d.heading.as_str()).collect();
  assert_eq!(
    headings,
    vec!["Deliverables", "Acceptance", "Dependencies"],
    "each dropped section is named, in authored order; `Objective` is modelled \
     rather than dropped and must not appear here"
  );
  for d in &scan.dropped {
    assert!(
      d.owner.contains("WP/01/info.md"),
      "the record names the file: {d:?}"
    );
    assert!(
      d.reason.contains("lib/templates/prj/st/WP/info.md")
        && d.reason.contains("0b1b3b5b")
        && d.reason.contains("bin/intent_wp:113"),
      "the reason cites the artefact by PATH, REVISION and the line that \
       substituted it, so the claim is checkable rather than asserted: {d:?}"
    );
  }
}

/// **THE NEGATIVE CONTROL (vc's condition).** The detector hits 40 of 40 on the
/// real estate, and a detector shown only positives has not been tested: if
/// nothing in the population is authored, "matches the substituted template"
/// and "matches anything" produce the same output.
///
/// One edited word is the whole difference between scaffolding and prose.
#[test]
fn an_edited_acceptance_section_is_carried_not_dropped() {
  let fixture = Fixture::new();
  estate(&fixture);
  let edited = TEMPLATE.replace("Do not restate ACs here.", "See ST0002 for why.");
  fixture.write_file(
    "intent/st/ST0001/WP/01/info.md",
    &format!(
      "---\nwp_id: WP-01\ntitle: \"A work package\"\nscope: Small\nstatus: Done\n---\n\n{}",
      edited.replace("WP-NN", "WP-01")
    ),
  );
  let scan = scan(&fixture);

  let body = body_of(&scan, 1);
  assert!(
    body.contains("See ST0002 for why."),
    "an author touched this section, so it is prose and must survive:\n{body}"
  );
  assert!(
    !scan.dropped.iter().any(|d| d.heading == "Acceptance"),
    "and it must not be recorded as a drop either: {:?}",
    scan.dropped
  );
  // The sections around it are untouched scaffolding and still go, so this is
  // a test of the discriminator rather than of the detector being switched off.
  assert!(
    scan.dropped.iter().any(|d| d.heading == "Deliverables"),
    "the unedited sections of the same file still drop: {:?}",
    scan.dropped
  );
}

/// **The substitution is per work package, and this is what proves it is being
/// applied rather than ignored.** Against the raw template neither of these
/// would match, because both carry a substituted `WP-NN`; against a template
/// substituted for the wrong sequence, exactly one would.
#[test]
fn each_work_package_is_compared_against_the_template_written_for_it() {
  let fixture = Fixture::new();
  estate(&fixture);
  wp(&fixture, 1, "");
  wp(&fixture, 2, "");
  let scan = scan(&fixture);

  for seq in [1, 2] {
    assert!(
      body_of(&scan, seq).trim().is_empty(),
      "WP-{seq:02}'s Acceptance section names WP-{seq:02}, and the comparison \
       has to substitute the same number or it never matches"
    );
  }
  assert_eq!(
    scan.dropped.len(),
    6,
    "three sections each, both work packages: {:?}",
    scan.dropped
  );
}

/// The other half of the discriminator: a heading the template never named is
/// not scaffolding under any comparison, and D28's catch-all still carries it.
#[test]
fn a_section_the_template_never_named_is_untouched() {
  let fixture = Fixture::new();
  estate(&fixture);
  wp(
    &fixture,
    1,
    "\n## Risks and Edge Cases\n\nThe quokka clause.\n",
  );
  let scan = scan(&fixture);

  let body = body_of(&scan, 1);
  assert!(
    body.contains("## Risks and Edge Cases") && body.contains("The quokka clause."),
    "a section the template never named survives whole -- that is what D28's \
     two-field shape buys:\n{body}"
  );
}

// ---------------------------------------------------------------------------
// The defect this closes, at the surface where it was visible
// ---------------------------------------------------------------------------

/// **The user-visible half.** `wp_info` generates its own `## Acceptance`, so a
/// carried copy renders a second one -- which is what 104 views across the
/// fleet were shipping.
#[test]
fn the_rendered_view_carries_exactly_one_acceptance_section() {
  let fixture = Fixture::new();
  estate(&fixture);
  wp(&fixture, 1, "");
  let scan = scan(&fixture);
  let thread = &scan.threads[0];
  let view = views::wp_info(thread, &thread.wps[0], &ctx());

  let count = view.lines().filter(|l| *l == "## Acceptance").count();
  assert_eq!(
    count, 1,
    "the generated section is the only one; a carried copy is the doubling:\n{view}"
  );
  assert!(
    view.contains("This cover never restates them."),
    "and the one that survives is the GENERATED one, not the carried one -- \
     dropping the generated half instead would produce the same count:\n{view}"
  );
}
