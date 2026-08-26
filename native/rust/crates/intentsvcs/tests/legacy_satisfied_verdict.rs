//! **A RATIFIED CONTRACT WAS SILENTLY INVERTED BY AN EXACT MATCH, AND THE
//! MIGRATION EXITED 0.**
//!
//! `field` returns a `satisfied:` value whole -- parenthetical and all -- and
//! the arm that read it was `(Some("yes"), Some(e))`. So `satisfied: yes (hv
//! signed off 2026-06-22)` matched nothing and fell into a `_` catch-all that
//! DEFAULTED to unsatisfied.
//!
//! # Measured, not reasoned, on Courses ST0002 at `d18aca7^`
//!
//! Ten criteria. **2 of 2 carrying a bare `yes` survived; 8 of 8 carrying a
//! parenthetical were downgraded.** Perfect correlation, no exceptions. The
//! phrase `hv signed off` survived neither in canon nor in the regenerated
//! view, so a COMPLETED thread arrived recording eight of ten criteria
//! unsatisfied and nothing anywhere reported it.
//!
//! **THE CATCH-ALL WAS THE WHOLE DEFECT.** A classifier whose default bucket
//! absorbs the unrecognised case cannot report that it met one -- so the
//! failure is invisible by construction, and the louder the author was about
//! their evidence the more likely they were to trip it. The projects with the
//! richest sign-off records are the worst affected, which is the worst
//! correlation such a bug could have.
//!
//! # Why the fixture is COPIED and not composed
//!
//! `FIXTURE` below is that file byte for byte. A fixture an author writes can
//! only encode what the author already believes -- and this defect turns
//! entirely on a spelling nobody would think to invent, because everybody
//! writing a test for `satisfied:` writes `satisfied: yes`. That is precisely
//! the spelling that WORKED.

mod common;

use common::Fixture;
use intentsvcs::finding::FindingClass;
use intentsvcs::legacy;
use intentsvcs::model::{AcKind, AcState};

/// Courses `intent/st/COMPLETED/ST0002/acceptance.md` at `d18aca7^`, verbatim.
const FIXTURE: &str = r#"---
verblock: "24 Jun 2026:v0.4: matts - All ACs satisfied (live on laksa.io, hv-confirmed); ST0002 complete"
st_id: ST0002
title: "Course2.0 content packaging for Laksa and e-book sales -- acceptance contract"
---

# ST0002 Course2.0 content packaging for Laksa and e-book sales -- Acceptance

> Canonical acceptance contract for ST0002. All ACs are non-test (doc / eyeball / gate), satisfied by named evidence. Complete (2026-06-24): design gate (AC-00.1) + contract/content (AC-01.1..03.1) + platform/integration (AC-04.1..09.1) all satisfied -- the courseware is live on laksa.io. Done is read from this map, never a hand-ticked box.

## Acceptance Criteria

### ST-level / design gate

- AC-00.1 (non-test) The design (`design.md`), the 10-WP breakdown with CC/LC ownership, and the `courseware.yaml` contract (`interface-contract.md`) are reviewed and ratified by hv -- evidence: hv reviewed + signed off 2026-06-21 -- satisfied: yes

### Contract + content (CC: WP-01..03)

- AC-01.1 (non-test) `courseware.yaml` schema is defined with worked examples for 001 and 002, agreed by cc + lc -- evidence: `interface-contract.md` v0.3 + lc ACK (`cc/inbox.lc.md` 2026-06-21 17:40) with redlines 1-3 folded in -- satisfied: yes (hv signed off 2026-06-22)
- AC-02.1 (non-test) Each course emits both artefact kinds in the contract layout: downloadable PDFs (from `bin/publish`) + browseable markdown -- evidence: validated 2026-06-22 -- all `courseware.yaml` artefact paths resolve (`{version}`->0.1.0) + browseable sources present (validator: ALL GOOD); 001 PDF + 13 002 docset PDFs exist at v0.1.0 -- satisfied: yes (hv signed off 2026-06-22)
- AC-03.1 (non-test) 001 + 002 each carry a valid `courseware.yaml` + the frontmatter/hierarchy the content type needs -- evidence: both `courseware.yaml` written + validated (ALL GOOD); 002 content re-tagged to lc's ratified lesson frontmatter (type/order/group/role) after lc built WP-04 to it; cc verified the tagging against lc's as-built `hierarchy.ex`/`config.ex` (group-ordering, course-root exclusion, H1 title fallback) 2026-06-22; contract v0.3 -- satisfied: yes (hv re-signed 2026-06-22 on cc's code-level verification; lc courtesy ack pending)

### Platform (LC, built in Laksa: WP-04..08)

- AC-04.1 (non-test) A `courseware` content type renders a course's browseable hierarchy from its `courseware.yaml` -- evidence: rendered course on a dev-local site -- satisfied: yes (live on laksa.io, hv-confirmed 2026-06-24)
- AC-05.1 (non-test) Both artefact kinds are gated: browseable + downloads require an entitlement; `free_sample` is public -- evidence: paywall demo -- satisfied: yes (live on laksa.io, hv-confirmed 2026-06-24)
- AC-06.1 (non-test) A courseware theme presents the course (nav, breadcrumb, lesson view, downloads) -- evidence: themed site -- satisfied: yes (live on laksa.io, hv-confirmed 2026-06-24)
- AC-07.1 (non-test) A Stripe purchase grants an entitlement that unlocks the product's `grants`; **no charity** anywhere; rate limiting **reuses** Laksa's existing mechanism; comp/grant works **without Backpex** -- evidence: live on laksa.io (hv-confirmed 2026-06-24); NB lc deferred the full e2e Stripe round-trip test -- legs covered + live redirect verified (COMPLETED/ST0076/impl.md) -- satisfied: yes

### Integration (CC + LC: WP-09..10)

- AC-08.1 (non-test) Dev-x works: the course dirs symlinked into `Laksa/priv/laksa/sites` serve dev-local; the same `courseware.yaml` works under prod GitHub sync -- evidence: dev-local render + a prod sync -- satisfied: yes (live on laksa.io, hv-confirmed 2026-06-24)
- AC-09.1 (non-test) End-to-end on laksa.io: a course deployed, paywalled, browseable + PDF downloadable, a test purchase unlocks both -- evidence: live (or staging) walkthrough -- satisfied: yes (live on laksa.io, hv-confirmed 2026-06-24)

## Acceptance Tests

Content / platform thread; every AC is non-test (doc / eyeball / gate), satisfied by named evidence. No code ATs apply.

- Coverage: every AC carries inline evidence + satisfied state; non-test by construction. Gates: AC-00.1 (design sign-off), AC-01.1 (contract agreed), AC-09.1 (end-to-end).
"#;

fn v2_estate(fixture: &Fixture, acceptance: &str) {
  v2_estate_at(fixture, "Completed", acceptance)
}

fn v2_estate_at(fixture: &Fixture, status: &str, acceptance: &str) {
  fixture.write_file(
    "intent/.config/config.json",
    "{\"intent_version\":\"2.19.0\",\"project_name\":\"P\",\"author\":\"cc\",\"intent_dir\":\"intent\",\"languages\":[\"rust\"]}\n",
  );
  fixture.write_file(
    "intent/st/ST0002/info.md",
    &format!("---\nverblock: \"24 Jun 2026:v0.4: matts - x\"\nintent_version: 2.19.0\nstatus: {status}\nslug: a-slug\ncreated: 20260624\ncompleted: 20260624\n---\n\n# ST0002: A thread\n\n## Objective\n\nShip it.\n"),
  );
  fixture.write_file("intent/st/ST0002/acceptance.md", acceptance);
}

fn scan(fixture: &Fixture) -> legacy::Scan {
  legacy::scan(&fixture.project()).expect("scan the v2 estate")
}

/// **THE REGRESSION, DRIVEN ON THE REAL FILE: every non-test criterion that
/// claimed satisfaction arrives SATISFIED, parenthetical or not.**
#[test]
fn a_parenthetical_after_yes_does_not_downgrade_the_criterion() {
  let fixture = Fixture::new();
  v2_estate(&fixture, FIXTURE);
  let scan = scan(&fixture);
  let thread = &scan.threads[0];

  let non_test: Vec<_> = thread
    .criteria
    .iter()
    .filter(|c| c.kind == AcKind::NonTest)
    .collect();
  assert_eq!(
    non_test.len(),
    10,
    "the fixture declares ten non-test criteria; a different count means the ROW parser moved and \
     this test is no longer about the verdict parser"
  );

  let downgraded: Vec<&str> = non_test
    .iter()
    .filter(|c| matches!(c.state, AcState::Unsatisfied))
    .map(|c| c.id.as_str())
    .collect();
  assert!(
    downgraded.is_empty(),
    "criteria that recorded `satisfied: yes` arrived UNSATISFIED -- a completed thread's ratified \
     contract, silently reversed: {downgraded:?}"
  );
}

/// **AND THE PARENTHETICAL IS CARRIED, because the verdict without its warrant
/// is the half that cannot be checked.**
#[test]
fn the_parenthetical_is_carried_as_evidence_rather_than_dropped() {
  let fixture = Fixture::new();
  v2_estate(&fixture, FIXTURE);
  let scan = scan(&fixture);

  let ac = scan.threads[0]
    .criteria
    .iter()
    .find(|c| c.id == "AC-01.1")
    .expect("AC-01.1 is in the fixture");

  let AcState::Satisfied { evidence } = &ac.state else {
    panic!(
      "AC-01.1 recorded `satisfied: yes (hv signed off 2026-06-22)`: {:?}",
      ac.state
    );
  };
  assert!(
    evidence.contains("hv signed off 2026-06-22"),
    "the sign-off naming a person and a date is the WARRANT for the claim, and dropping it while \
     keeping the verdict preserves an assertion nobody can check: {evidence}"
  );
  assert!(
    evidence.contains("interface-contract.md"),
    "and the row's own `evidence:` field is still there beside it: {evidence}"
  );
}

/// **AN UNRECOGNISED VERDICT REFUSES. IT DOES NOT DEFAULT.**
///
/// This is the arm that makes the fix a fix rather than one more spelling
/// added to a list. The old code could not report meeting a value it did not
/// know, because its default bucket swallowed it; the next unanticipated
/// spelling would have failed exactly as silently.
#[test]
fn a_verdict_that_is_neither_yes_nor_no_is_refused_rather_than_defaulted() {
  let fixture = Fixture::new();
  v2_estate(
    &fixture,
    "## Acceptance Criteria\n\n- AC-01.1 (non-test) A thing -- evidence: somewhere -- satisfied: probably\n",
  );
  let scan = scan(&fixture);

  assert!(
    scan.threads[0].criteria.is_empty(),
    "an unreadable verdict must not be carried as though it had been read"
  );
  // **The refusal is RECORDED, and it routes by the thread's state like every
  // other finding does.** This fixture is a CLOSED thread, so the finding
  // CARRIES rather than blocking -- `legacy_scope_carry.rs`'s whole subject.
  // Asserting on `residue` here was this test's own first draft and it failed
  // for the right reason: a silent drop and a carried finding are the same
  // `criteria.is_empty()`, and only the second is recoverable.
  assert!(
    scan
      .carried
      .iter()
      .any(|f| f.class == FindingClass::UnparseableRow),
    "a refused row on a closed thread must be CARRIED, not dropped in silence: {:?}",
    scan.carried
  );
}

/// The same unreadable verdict on a LIVE thread BLOCKS instead of carrying.
///
/// The pair matters more than either half: it shows the refusal is subject to
/// the estate's carry policy rather than being a special case that happens to
/// print something.
#[test]
fn the_same_unreadable_verdict_on_a_live_thread_blocks() {
  let fixture = Fixture::new();
  v2_estate_at(
    &fixture,
    "WIP",
    "## Acceptance Criteria\n\n- AC-01.1 (non-test) A thing -- evidence: somewhere -- satisfied: probably\n",
  );
  let scan = scan(&fixture);

  assert!(
    scan
      .residue
      .iter()
      .any(|f| f.class == FindingClass::UnparseableRow),
    "an unreadable verdict in a LIVE thread must block the migration: {:?}",
    scan.residue
  );
}

/// An unclosed parenthetical is a truncation, and reading it as a bare `yes`
/// would silently discard whatever the truncation ate.
#[test]
fn an_unclosed_parenthetical_is_refused() {
  let fixture = Fixture::new();
  v2_estate(
    &fixture,
    "## Acceptance Criteria\n\n- AC-01.1 (non-test) A thing -- evidence: somewhere -- satisfied: yes (hv signed off\n",
  );
  let scan = scan(&fixture);

  assert!(
    scan.threads[0].criteria.is_empty(),
    "a truncated verdict must refuse rather than round to the nearest readable one"
  );
}

/// A row making NO claim is not a malformed one -- absent and unreadable are
/// different, and the refusal above must not swallow the ordinary case.
#[test]
fn a_row_with_no_satisfied_field_is_carried_unsatisfied() {
  let fixture = Fixture::new();
  v2_estate(
    &fixture,
    "## Acceptance Criteria\n\n- AC-01.1 (non-test) A thing nobody has ruled on yet\n",
  );
  let scan = scan(&fixture);

  let ac = &scan.threads[0].criteria[0];
  assert!(
    matches!(ac.state, AcState::Unsatisfied),
    "a claim nobody made reads as unsatisfied: {:?}",
    ac.state
  );
  assert!(
    !scan
      .residue
      .iter()
      .any(|f| f.class == FindingClass::UnparseableRow),
    "and it is NOT a finding -- absent is not unreadable"
  );
}

/// **`n/a` IS KNOWN VOCABULARY AND IS CARRIED, NOT REFUSED.**
///
/// The arm that makes this fix a fix rather than a second defect. Measured
/// across the estate's `acceptance.md` AC rows: `yes` 1836, `yes (note)` 614,
/// `no (note)` 180, `no` 159, **`n/a` 20**. Those twenty fell into the old
/// catch-all and read unsatisfied; a refusal would DROP them from canon
/// entirely, **losing more than the bug being fixed ever did**.
///
/// It reads unsatisfied -- what it already did -- rather than being mapped to
/// `Descoped` or `Withdrawn`, both of which carry a reason and a destination
/// nobody wrote. Inventing one is the same offence as inventing evidence.
#[test]
fn n_a_is_known_vocabulary_and_is_not_refused() {
  let fixture = Fixture::new();
  v2_estate(
    &fixture,
    "## Acceptance Criteria\n\n- AC-01.1 (non-test) A thing that does not apply -- satisfied: n/a\n",
  );
  let scan = scan(&fixture);

  assert_eq!(
    scan.threads[0].criteria.len(),
    1,
    "`n/a` is v2 vocabulary, not a malformed value -- refusing it drops a row that used to land"
  );
  assert!(
    matches!(scan.threads[0].criteria[0].state, AcState::Unsatisfied),
    "and it reads exactly as it did before the fix: {:?}",
    scan.threads[0].criteria[0].state
  );
  assert!(
    !scan
      .carried
      .iter()
      .any(|f| f.class == FindingClass::UnparseableRow),
    "and it raises no finding -- known is not unreadable"
  );
}
