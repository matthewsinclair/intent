//! AT-03.13 / AC-03.12: **a field the tool refuses to proceed without must
//! reach a human read face.**
//!
//! Stated as a general property rather than about one field. `status_reason`
//! is where it was found -- four verbs demand one, and for a while nothing a
//! person reads rendered it -- but a row that tested only that field would go
//! green and stay green while the next demanded field arrived unread.
//!
//! # The denominator is DERIVED, and that is the whole mechanism
//!
//! The fields the service layer refuses without are not a list anybody keeps;
//! they are exactly the [`FacadeError`] variants that demand one. So
//! [`demanded_field`] is an EXHAUSTIVE match over the error type: **a variant
//! added to the facade does not compile until somebody says whether it demands
//! a field**, and if it does, the field must reach a face or
//! [`reaches_a_read_face`] fails by name.
//!
//! A hand-kept roster is a roster someone has to remember to extend on the day
//! they are thinking about something else, which is the day the field goes
//! unread. The same argument `openness.rs` makes for reading its tables out of
//! the DDL.
//!
//! # What this row does NOT claim, and hv ruled on it
//!
//! The refusal used to justify demanding a reason by citing the event log --
//! "as part of the decision, which is what lets anyone reconstruct why later"
//! -- and **no shipped verb reads the event log**: no `events`, no `log`, no
//! `history`, and `ingest.rs` never mentions the field so `search` does not
//! reach it either. A refusal arguing from a capability the operator cannot
//! exercise is arguing from nothing.
//!
//! **hv ruled the promise comes out rather than a reader goes in**, so the
//! second carrier is gone from the message and this row is about the carrier
//! that remains. `no_promise_the_tool_cannot_keep` is the assertion that keeps
//! it gone.
//!
//! # The estate cannot test this and says so out loud
//!
//! **Zero of the estate's threads carry a `status_reason`**, so an
//! estate-driven version of this row is 0 of 0 -- right verb, right depth, a
//! population that cannot contain the failure. The fixtures below are
//! constructed, and the estate zero is asserted as a zero rather than left to
//! look like coverage.

mod common;

use common::{ctx, sample_thread};
use intentsvcs::facade::FacadeError;
use intentsvcs::model::{AcKind, AcState, Criterion, Thread, ThreadStatus};
use intentsvcs::views;

/// **Which field a refusal DEMANDS, or `None` if it is not that kind of
/// refusal.**
///
/// Exhaustive on purpose: this is the denominator, and it is the compiler that
/// keeps it complete.
fn demanded_field(err: &FacadeError) -> Option<&'static str> {
  match err {
    FacadeError::ReasonRequired { .. } => Some("status_reason"),
    FacadeError::EvidenceRequired { .. } => Some("evidence"),
    FacadeError::DescopeTargetRequired { .. } => Some("descope target"),

    // Everything below refuses for a reason that is not "you left a field out",
    // so there is no authored value to carry to a reader.
    FacadeError::WriteNotAddressable { .. }
    | FacadeError::NoSuchThread { .. }
    | FacadeError::ThreadExists { .. }
    | FacadeError::NoSuchWorkPackage { .. }
    | FacadeError::NoSuchCriterion { .. }
    | FacadeError::NoSuchTest { .. }
    | FacadeError::GateBlocked { .. }
    | FacadeError::ComputedSatisfaction { .. }
    | FacadeError::NotOffScope { .. }
    | FacadeError::NotSatisfied { .. }
    | FacadeError::OffScope { .. }
    | FacadeError::WrongOffScopeState { .. }
    | FacadeError::BadQuery { .. }
    | FacadeError::NoSuchFace { .. }
    | FacadeError::IllegalTransition { .. }
    | FacadeError::DescopeTargetMissing { .. }
    | FacadeError::Unmigrated(_)
    | FacadeError::BelowMigrationFloor(_)
    | FacadeError::Write(_)
    | FacadeError::ViewsNotWritten { .. }
    | FacadeError::Store(_)
    | FacadeError::Ingest(_)
    | FacadeError::NoSuchFormat { .. }
    | FacadeError::LossyFormat { .. }
    | FacadeError::ExportRoundTripFailed { .. }
    | FacadeError::UnhonourableWindow(_)
    | FacadeError::NoSuchIssue { .. }
    | FacadeError::MigrationBlocked(_)
    | FacadeError::MigrationHalted { .. }
    | FacadeError::EgestFromRefusedIngest { .. }
    | FacadeError::EgestWouldEmptyTheEstate { .. }
    | FacadeError::Organize(_)
    | FacadeError::Intentfiles(_)
    | FacadeError::ManifestUnreadable { .. }
    // The manifest exists and will not parse. It reports a file the operator
    // has to correct, not a value they left out of a call -- and the field it
    // carries beyond the cause is the PATH, which the reader needs in order to
    // act and never authored.
    | FacadeError::ManifestMalformed { .. }
    // dc's, arriving mid-session with `Facade::hydrate`. It refuses because an
    // entity form has no file to make exist, not because a value was left out
    // -- there is no authored prose behind it for a reader to lose.
    | FacadeError::NotHydratable { .. }
    // `intent edit`'s two refusals. Both are about WHERE a file may be
    // authored, not about a value the caller left out of one -- the argument
    // was complete and the answer is still no.
    | FacadeError::NotEditable { .. }
    | FacadeError::NoSuchEditable { .. }
    // A wrapped realisation failure. It reports that making files exist did not
    // work, not that a value was left out -- no authored prose behind it.
    | FacadeError::Realise(_) => None,
  }
}

/// Every refusal that demands a field, as a value, so the match above is
/// actually exercised rather than merely compiled.
fn field_demanding_refusals() -> Vec<FacadeError> {
  vec![
    FacadeError::ReasonRequired { verb: "st hold" },
    FacadeError::EvidenceRequired {
      ac: "AC-03.2".to_string(),
    },
    FacadeError::DescopeTargetRequired {
      ac: "AC-03.2".to_string(),
    },
  ]
}

/// A canary distinctive enough that finding it in a rendered view cannot be a
/// coincidence with template boilerplate.
const CANARY: &str = "zzz-canary-9f3a";

/// **Render a face carrying the demanded field, and hand back what a person
/// would read.**
///
/// Panics rather than returns for an unregistered field, and the panic IS the
/// failure a fourth demanded field is supposed to hit: the match above forces
/// somebody to classify a new variant, and this forces them to show where its
/// field is read.
fn reaches_a_read_face(field: &str) -> String {
  let c = ctx();
  match field {
    "status_reason" => {
      let mut t = held_thread();
      t.status_reason = Some(CANARY.to_string());
      views::info(&t, &c)
    }
    "evidence" => views::acceptance(
      &thread_whose_criterion_is(AcState::Satisfied {
        evidence: CANARY.to_string(),
      }),
      &c,
    ),
    "descope target" => views::acceptance(
      &thread_whose_criterion_is(AcState::Descoped {
        to: CANARY.to_string(),
        by: None,
        reason: None,
      }),
      &c,
    ),
    other => panic!(
      "`{other}` is demanded by a refusal and no face is registered for it here.\n       \
       Either render it where a person reads it, or stop demanding it -- a value\n       \
       the tool refuses to proceed without and then shows to nobody is authored\n       \
       prose a human typed because the tool asked, recoverable from nothing but\n       \
       the committed extract."
    ),
  }
}

fn held_thread() -> Thread {
  Thread {
    status: ThreadStatus::Hold,
    ..sample_thread("ST0000")
  }
}

fn thread_whose_criterion_is(state: AcState) -> Thread {
  Thread {
    criteria: vec![Criterion {
      id: "AC-01.1".to_string(),
      text: "a criterion carrying an authored decision".to_string(),
      kind: AcKind::NonTest,
      state,
    }],
    ..sample_thread("ST0000")
  }
}

/// **THE PROPERTY.** Every demanded field reaches a face.
#[test]
fn every_field_a_refusal_demands_reaches_a_human_read_face() {
  let refusals = field_demanding_refusals();
  assert!(
    !refusals.is_empty(),
    "vacuous unless the tool demands something"
  );

  for err in &refusals {
    let field = demanded_field(err).unwrap_or_else(|| {
      panic!("`{err}` is in the demanding set and `demanded_field` says it demands nothing")
    });
    let rendered = reaches_a_read_face(field);
    assert!(
      rendered.contains(CANARY),
      "`{field}` is demanded by `{err}` and does not survive to anything a person\n       \
       reads. The view rendered was:\n{rendered}"
    );
  }
}

/// **THE CONTROL.** Without it the case above passes on a renderer that emits
/// the canary unconditionally, or on views that print every field they are
/// handed regardless of whether it was set.
#[test]
fn a_field_that_was_not_authored_does_not_appear() {
  let quiet = views::info(&sample_thread("ST0000"), &ctx());
  assert!(
    !quiet.contains(CANARY),
    "the canary is only ever written by this test's fixtures"
  );
  let mut plain = sample_thread("ST0000");
  plain.status_reason = None;
  assert!(
    !views::info(&plain, &ctx()).contains("status_reason"),
    "no reason means no key -- a view that prints the label unconditionally\n       \
     would satisfy the property above while showing nobody anything"
  );
}

/// **The demanding set is exactly what the exhaustive match says it is.**
///
/// Three today. If a fourth arrives, `demanded_field` will not compile until it
/// is classified, and this count is what stops somebody classifying it `None`
/// to make the build go green.
#[test]
fn the_demanding_set_is_three_and_the_compiler_keeps_it_honest() {
  let demanded: Vec<&'static str> = field_demanding_refusals()
    .iter()
    .filter_map(demanded_field)
    .collect();
  assert_eq!(
    demanded,
    vec!["status_reason", "evidence", "descope target"],
    "the fields the service layer refuses without. A fourth belongs here AND in\n       \
     `reaches_a_read_face`; classifying it `None` to quiet the compiler is the\n       \
     move this assertion exists to make visible"
  );
}

/// **hv's ruling, pinned: the refusal does not promise a reader the tool does
/// not have.**
///
/// The message cited the event log -- "as part of the decision, which is what
/// lets anyone reconstruct why later" -- while no shipped verb could read it.
/// Removing the promise is what closed AC-03.12, so a re-introduction has to
/// fail here rather than ship as a helpful-sounding sentence.
#[test]
fn no_promise_the_tool_cannot_keep() {
  use intentsvcs::remedy::Remedy;
  let remedy = FacadeError::ReasonRequired { verb: "st hold" }.remedy();
  assert!(
    remedy.contains("recorded on the entity"),
    "the carrier the tool DOES have is still named: {remedy}"
  );
  for promise in ["event log", "reconstruct", "history", "audit"] {
    assert!(
      !remedy.contains(promise),
      "the refusal cites `{promise}` as a reason to comply, and nothing shipped\n       \
       can read it -- `intent --help` declares no verb that does. Either build\n       \
       the reader or do not argue from it: {remedy}"
    );
  }
}

/// **The estate is a zero, and it is printed as one.**
///
/// Asserted rather than assumed, because if the estate ever DOES carry a
/// `status_reason` the fixtures above stop being the only evidence and this
/// row can be strengthened to drive the real thing.
#[test]
fn the_estate_carries_no_status_reason_and_that_is_why_the_fixtures_exist() {
  use intentsvcs::project::Project;
  let root = testkit::repo_root();
  let project = Project::open(&root).expect("this repository is a project");
  let canon = intentsvcs::ingest::read(&project).expect("the estate reads");

  let carrying: Vec<&str> = canon
    .threads
    .iter()
    .filter(|t| t.status_reason.is_some())
    .map(|t| t.id.as_str())
    .collect();
  assert!(
    !canon.threads.is_empty(),
    "vacuous unless the estate has threads at all"
  );
  assert!(
    carrying.is_empty(),
    "the estate now carries a status_reason on {carrying:?} -- {} of {} threads.\n       \
     This row can now be driven against the real estate instead of only\n       \
     constructed fixtures, and it should be.",
    carrying.len(),
    canon.threads.len()
  );
}
