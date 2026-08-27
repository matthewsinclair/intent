//! **A THREAD RECORDING TWELVE hv-RULED DESCOPES ARRIVED 24/24 GREEN, AND THE
//! MIGRATION EXITED 0.**
//!
//! `non_test` is a literal `rest.trim_start().starts_with("(non-test)")`, and
//! `field(body, "satisfied")` is evaluated ABOVE the branch that uses it. So a
//! row whose author wrote a full descope record -- destination, authority, date
//! and reason -- but did not open the line with the marker falls to the `else`
//! arm, becomes `(AcKind::Test, AcState::Computed)`, and has its entire claim
//! computed and then dropped on the floor.
//!
//! # Measured on the estate at 2026-08-27, and the first two populations were wrong
//!
//! Rows carrying `satisfied:` with no marker are **792** -- but 789 of those are
//! legitimately test-backed and their `satisfied:` is v2 noise, so that is not
//! the subject. A merely MISPLACED marker (trailing, or behind `**`) is **3**,
//! which is too narrow. The subject is **rows with no marker that carry a
//! NON-TEST-ONLY field** (`evidence` / `by` / `on` / `descoped-to` / `reason` /
//! `withdrawn`): **40** estate-wide, of which **21 carry no evidence** and
//! **Lamplight ST0346 contributes 12**.
//!
//! The discriminator is what makes a refusal scopable rather than a blanket:
//! across **2757** unmarked AC rows only ~20 carry ANY non-test-only field,
//! while `evidence:` sits on **1285 of 1339** marked ones.
//!
//! # Why this REFUSES rather than reclassifies, proved by a row in the fixture
//!
//! The obvious fix -- treat a non-test-only field as the missing marker and
//! read the row as non-test -- is wrong, and the corpus says so. Conflab
//! `AC-01.5` carries `evidence:` and `satisfied: yes` with no marker, and its
//! own prose reads **"Promoted from non-test to test-backed (AT-01.12 Rust,
//! AT-01.13 Swift)"**. It is CORRECTLY test-backed; the two fields are stale
//! residue the author left behind when they promoted it.
//!
//! So the same shape has two opposite correct readings and the row does not say
//! which. **A refusal is right for both** -- it names the ambiguity and stops.
//! Reading the row as non-test would silently reverse Conflab's promotion, and
//! reading it as test-backed silently discards ST0346's twelve. That is vc's
//! "refuse, never reclassify" ruling, and it is not a caution here but the only
//! reading the evidence supports.
//!
//! **The 21 rows carrying no evidence are the other half of the same argument.**
//! Widening them into the non-test branch lands them on `AcState::Unsatisfied`
//! -- trading a silent DROP for a silent FAILURE, which is not a fix.
//!
//! # Why arm 1 and arm 2 are one commit
//!
//! `satisfied_verdict` maps `n/a` to `(false, note)`, justified in its own body
//! by `Descoped` and `Withdrawn` carrying "a reason and a destination that
//! nobody wrote". True of a bare `n/a`. **False of every row below**, which
//! writes `descoped-to:`/`withdrawn:` plus `by:` plus `on:` plus `reason:` in
//! full, into an `AcState::Descoped { to, by, reason }` that already exists.
//!
//! So arm 1 alone walks ST0346 from a green lie (twelve descopes counted
//! `Computed`) to a RED one (those twelve as hard `Unsatisfied`), and arm 2
//! alone never fires because arm 1 never lets a row reach it. Neither half is
//! shippable without the other.

mod common;

use common::Fixture;
use intentsvcs::finding::FindingClass;
use intentsvcs::legacy;
use intentsvcs::model::{AcKind, AcState};

/// Lamplight `intent/history/ST0346/acceptance.md` and Laksa `ST0097`, verbatim,
/// plus the two CONTROLS this fix must not disturb.
///
/// Copied, not composed, for the reason `legacy_satisfied_verdict` gives: a
/// fixture an author writes encodes only what the author already believed, and
/// nobody inventing a descope row would invent one with the marker missing.
const FIXTURE: &str = r##"---
verblock: "19 Aug 2026:v0.4: matts - x"
st_id: ST0002
title: "A thread -- acceptance contract"
---

# ST0002 A thread -- Acceptance

## Acceptance Criteria

- AC-02.9 Every action available on a surface has a command id in `noun.verb` form, listed in the Commands tab with tab-completion. -- descoped-to: ST0353 -- by: hv -- on: 2026-08-19 -- reason: Violated at scale: 85 of 105 surface phx-click sites are bespoke, across 65 names. Remediation is a rebuild and ST0346 is a nudge. hv ruled 2026-08-19. -- satisfied: n/a
- AC-03.1 Exactly two modals remain across the two tools: ingest-preflight and scope-selector -- both genuine before-anything-else decisions. -- withdrawn: hv, 2026-08-19, on the 23-modal inventory: 'I see no reason to get rid of any of those.' Every modal in the estate has a purpose and live call sites; the criterion's 'exactly two' was an estimate that did not survive contact with the population. -- by: hv -- on: 2026-08-19 -- satisfied: n/a
- AC-15.1 **`cache refresh` has a defined meaning in PRODUCTION and that meaning is not a failure.** -- withdrawn: Measured 2026-08-19: the file cache is unreachable in production, so the verb has no production meaning to state. -- by: matts -- on: 2026-08-19 -- satisfied: n/a
- AC-90.1 A plain test-backed row whose v2 `satisfied:` is noise nobody should read. -- satisfied: yes
- AC-90.2 (non-test) A bare n/a with no record anywhere on the row. -- satisfied: n/a
"##;

fn v2_estate(fixture: &Fixture, acceptance: &str) {
  fixture.write_file(
    "intent/.config/config.json",
    "{\"intent_version\":\"2.19.0\",\"project_name\":\"P\",\"author\":\"cc\",\"intent_dir\":\"intent\",\"languages\":[\"rust\"]}\n",
  );
  fixture.write_file(
    "intent/st/ST0002/info.md",
    "---\nverblock: \"19 Aug 2026:v0.4: matts - x\"\nintent_version: 2.19.0\nstatus: Completed\nslug: a-slug\ncreated: 20260819\ncompleted: 20260819\n---\n\n# ST0002: A thread\n\n## Objective\n\nShip it.\n",
  );
  fixture.write_file("intent/st/ST0002/acceptance.md", acceptance);
}

fn scan(fixture: &Fixture) -> legacy::Scan {
  legacy::scan(&fixture.project()).expect("scan the v2 estate")
}

fn criterion<'a>(scan: &'a legacy::Scan, id: &str) -> Option<&'a intentsvcs::model::Criterion> {
  scan.threads[0].criteria.iter().find(|c| c.id == id)
}

/// Findings, wherever the thread's liveness put them. A refusal on a CLOSED
/// thread is `carried` and on a LIVE one is `residue`; this fix is about the
/// refusal existing at all, not about which side it lands on.
fn findings(scan: &legacy::Scan) -> Vec<&intentsvcs::finding::Finding> {
  scan.residue.iter().chain(scan.carried.iter()).collect()
}

/// **ARM 1 + 2, THE HEADLINE.** A descope record written in full survives the
/// missing marker -- as a descope, not as a green computation and not as a
/// failure.
#[test]
fn a_descope_record_survives_a_missing_marker() {
  let fixture = Fixture::new();
  v2_estate(&fixture, FIXTURE);
  let scan = scan(&fixture);

  let ac = criterion(&scan, "AC-02.9").expect("AC-02.9 reaches canon");
  let AcState::Descoped { to, by, reason } = &ac.state else {
    panic!(
      "AC-02.9 records `descoped-to: ST0353 -- by: hv -- on: 2026-08-19 -- reason: ...`, which is \
       a destination and a warrant the author WROTE. Arriving as {:?} means the whole record was \
       computed and dropped -- and as `Computed` it also counts GREEN toward the thread's total, \
       so the loss reads as a pass.",
      ac.state
    );
  };
  assert_eq!(
    to, "ST0353",
    "the destination is read from the row, not minted"
  );
  assert_eq!(
    by.as_deref(),
    Some("hv"),
    "`by: hv` names the authority for the descope; dropping it keeps the ruling and loses who made it"
  );
  assert!(
    reason
      .as_deref()
      .unwrap_or_default()
      .contains("Violated at scale"),
    "the reason is the half that makes a descope reviewable: {reason:?}"
  );
  assert_eq!(
    ac.kind,
    AcKind::NonTest,
    "a descoped criterion is not test-backed"
  );
}

/// A `withdrawn:` record is the same shape and the same loss.
#[test]
fn a_withdrawn_record_survives_a_missing_marker() {
  let fixture = Fixture::new();
  v2_estate(&fixture, FIXTURE);
  let scan = scan(&fixture);

  for id in ["AC-03.1", "AC-15.1"] {
    let ac = criterion(&scan, id).unwrap_or_else(|| panic!("{id} reaches canon"));
    let AcState::Withdrawn { reason, by } = &ac.state else {
      panic!(
        "{id} records a `withdrawn:` with `by:` and `on:`: {:?}",
        ac.state
      );
    };
    assert!(
      !reason.trim().is_empty(),
      "{id} withdrew with a reason the author wrote"
    );
    assert!(by.is_some(), "{id} names who withdrew it");
  }
}

/// **THE THREAD-LEVEL CONSEQUENCE, which is the number an operator actually
/// reads.** Three rows that recorded no claim to satisfaction must not be
/// counted green, and must not be counted as failures either.
#[test]
fn a_descoped_row_is_neither_green_nor_a_failure() {
  let fixture = Fixture::new();
  v2_estate(&fixture, FIXTURE);
  let scan = scan(&fixture);

  let bad: Vec<&str> = ["AC-02.9", "AC-03.1", "AC-15.1"]
    .into_iter()
    .filter(|id| {
      criterion(&scan, id)
        .map(|c| matches!(c.state, AcState::Computed | AcState::Unsatisfied))
        .unwrap_or(true)
    })
    .collect();
  assert!(
    bad.is_empty(),
    "`Computed` counts these GREEN and `Unsatisfied` counts them FAILED. Both are wrong and they \
     are wrong in opposite directions -- which is why arm 1 cannot ship without arm 2: {bad:?}"
  );
}

/// **CONTROL -- the 789.** A test-backed row's leftover v2 `satisfied:` is noise
/// and must stay ignored, with no finding. If this goes red the refusal is
/// mis-scoped and would refuse a third of the estate.
#[test]
fn a_bare_satisfied_field_on_a_test_backed_row_is_still_ignored() {
  let fixture = Fixture::new();
  v2_estate(&fixture, FIXTURE);
  let scan = scan(&fixture);

  let ac = criterion(&scan, "AC-90.1").expect("AC-90.1 reaches canon");
  assert_eq!(
    ac.kind,
    AcKind::Test,
    "no marker and no non-test-only field: test-backed"
  );
  assert!(
    matches!(ac.state, AcState::Computed),
    "a test-backed criterion's satisfaction is computed from its covering tests: {:?}",
    ac.state
  );
  assert!(
    !findings(&scan).iter().any(|f| f.detail.contains("AC-90.1")),
    "789 estate rows look exactly like this one; a finding here is a finding on all of them"
  );
}

/// **CONTROL -- the bare `n/a`.** With nothing written beside it there is no
/// record to carry, so today's reading stands and nothing is minted.
#[test]
fn a_bare_n_a_with_no_record_stays_unsatisfied() {
  let fixture = Fixture::new();
  v2_estate(&fixture, FIXTURE);
  let scan = scan(&fixture);

  let ac = criterion(&scan, "AC-90.2").expect("AC-90.2 reaches canon");
  assert!(
    matches!(ac.state, AcState::Unsatisfied),
    "a bare `n/a` names no destination and no reason, so `Descoped`/`Withdrawn` would be MINTED \
     from nothing -- the offence this fix exists to stop: {:?}",
    ac.state
  );
}

/// **THE AMBIGUOUS ROW IS REFUSED, NOT GUESSED.** Conflab `AC-01.5`'s shape --
/// `evidence:` + `satisfied:` + no marker -- reads as a promoted test-backed row
/// in Conflab and as a missing marker elsewhere, and the row does not say which.
#[test]
fn a_non_test_only_field_with_no_marker_is_refused_rather_than_read() {
  let fixture = Fixture::new();
  v2_estate(
    &fixture,
    &FIXTURE.replace(
      "- AC-90.1 A plain test-backed row whose v2 `satisfied:` is noise nobody should read. -- satisfied: yes\n",
      "- AC-01.5 The `[conflab-error]` tag is migrated onto this contract. **Promoted from non-test to test-backed** (AT-01.12 Rust, AT-01.13 Swift). -- evidence: the frame is retained, its content is now the shared `ConflabError` JSON -- satisfied: yes\n",
    ),
  );
  let scan = scan(&fixture);

  let all = findings(&scan);
  let refusal = all
    .iter()
    .find(|f| f.class == FindingClass::UnparseableRow && f.detail.contains("AC-01.5"));
  assert!(
    refusal.is_some(),
    "the row carries a non-test-only field with no non-test marker. Reading it as non-test \
     silently reverses Conflab's promotion; reading it as test-backed silently discards ST0346's \
     twelve. Only a refusal is correct for both, and today there is no finding at all: {:?}",
    all.iter().map(|f| &f.detail).collect::<Vec<_>>()
  );
  let message = &refusal.expect("checked above").detail;
  assert!(
    message.contains("evidence"),
    "the refusal must NAME the field it could not place, or it cannot be acted on: {message}"
  );
}

/// **A MARKER MIS-PLACED IS STILL A MARKER, and refusing it costs more than the
/// mis-reading.**
///
/// Lamplight `ST0283 AC-08.4` writes the token after the sentence rather than
/// before it. `starts_with` read it as test-backed; a REFUSAL would have
/// dropped the criterion from canon entirely -- measured, the thread went 67
/// rows to 65 and two ratified criteria vanished. The row says `(non-test)`
/// plainly, so its author's intent is on the row and nothing has to be guessed.
#[test]
fn a_marker_after_the_prose_is_still_a_marker() {
  let fixture = Fixture::new();
  v2_estate(
    &fixture,
    "---\nst_id: ST0002\n---\n\n# T\n\n## Acceptance Criteria\n\n\
     - AC-08.4 Progress renders via the existing `ProgressFill` (reuse; no second progress bar). (non-test) -- evidence: `jobs/board.ex` imports it for both rows -- satisfied: yes\n",
  );
  let scan = scan(&fixture);
  let ac = criterion(&scan, "AC-08.4").expect("the row is NOT refused -- refusing it loses it");
  let AcState::Satisfied { evidence } = &ac.state else {
    panic!(
      "AC-08.4 is an authored criterion with evidence and a yes: {:?}",
      ac.state
    );
  };
  assert!(
    evidence.contains("jobs/board.ex"),
    "the evidence is carried: {evidence}"
  );
  assert!(
    !ac.text.contains("(non-test)"),
    "the marker is a marker, not part of the requirement's text: {}",
    ac.text
  );
}

/// **AND THE ROW THAT ONLY TALKS ABOUT THE MARKER IS NOT MARKED BY IT.**
///
/// This is Intent's own `ST0056 AC-03.17`, which quotes the renderer that emits
/// the token. A `contains` test -- this fix's first cut -- promoted a
/// test-backed criterion to a satisfied authored one, silently. **It was caught
/// by driving the real corpus, not by reading the diff**, which is why it has an
/// arm: reading prose ABOUT a thing as the thing has cost this estate an
/// accidental `intent upgrade` and a commit gate that stripped one quoting style
/// and not the other.
#[test]
fn prose_quoting_the_marker_does_not_mark_the_row() {
  let fixture = Fixture::new();
  v2_estate(
    &fixture,
    "---\nst_id: ST0002\n---\n\n# T\n\n## Acceptance Criteria\n\n\
     - AC-03.17 A generated view is a fixed point of the formatter. `criterion_line` is `format!(\"- {} \", c.id)` + an optional `(non-test) ` + `push_str(&c.text)` VERBATIM. -- satisfied: yes\n\
     \n## Acceptance Tests\n\n\
     - AT-03.1 tests/views.rs -- covers AC-03.17 -- status: green\n",
  );
  let scan = scan(&fixture);
  let ac = criterion(&scan, "AC-03.17").expect("AC-03.17 reaches canon");
  assert_eq!(
    ac.kind,
    AcKind::Test,
    "the row DESCRIBES the marker its renderer emits; it does not carry one. Reading the \
     description as a declaration promotes a test-backed criterion to a satisfied authored one, \
     and the covering test stops counting for anything"
  );
  assert!(
    matches!(ac.state, AcState::Computed),
    "and its satisfaction still comes from AT-03.1: {:?}",
    ac.state
  );
}
