//! AT-03.13 / AC-03.12: **a field the tool refuses to proceed without must
//! reach a human read face.**
//!
//! Stated as a general property rather than about one field. `status_reason`
//! is where it was found -- several verbs demand one, and for a while nothing a
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
//! **None of the estate's threads carry a `status_reason`**, so an
//! estate-driven version of this row has an empty denominator -- right verb,
//! right depth, a population that cannot contain the failure. The fixtures
//! below are constructed, and the estate zero is asserted as a zero rather than
//! left to look like coverage.

use crate::common::{ctx, sample_thread};
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
    // The SQL door's refusals are about a STATEMENT, not about a field the
    // caller left out: the remedy is a different statement, never a fuller
    // call.
    // A refusal from the embedder is about an endpoint or a configuration
    // block, not about a field the caller left out of this call.
    FacadeError::Embed(_)
    | FacadeError::SqlMoreThanOneStatement
    | FacadeError::SqlNoStatement
    | FacadeError::SqlUnterminated
    | FacadeError::SqlWouldWrite
    | FacadeError::SqlOutOfReach { .. }
    | FacadeError::SqlOverBudget
    | FacadeError::SqlLimitAboveCeiling { .. }
    // A structural door asked for a tier it cannot answer: the remedy is a
    // different tier filter, never a fuller call.
    | FacadeError::StructuralTierNotAsked
    | FacadeError::SqlDidNotRun { .. }
    | FacadeError::WriteNotAddressable { .. }
    | FacadeError::RowBreaksContract { .. }
    | FacadeError::VerdictWrongForKind { .. }
    | FacadeError::OpenWorkPackages { .. }
    // Issue 0503: the caller gave the date, and the refusal is that a closed
    // thread's date is restated through `intent set` rather than through a
    // close. No field was left out of the call.
    | FacadeError::CompletionDateNotRestated { .. }
    | FacadeError::NoSuchThread { .. }
    | FacadeError::ThreadExists { .. }
    // Both halves of issue 0131's refusal. They report that a KEY is taken, so
    // there is no field the caller failed to supply -- the remedy is a
    // different key, not a fuller call.
    | FacadeError::IssueExists { .. }
    // A renumber's refusals, the same argument: the id asked for is taken, or
    // the filesystem refused a move. Neither is a field left out of the call.
    | FacadeError::RenumberTargetTaken { .. }
    | FacadeError::RenumberDiskStep { .. }
    // `sync --apply`'s refusals: a moved tree, no merge in progress, a disk
    // step or git refusing. Each is a fact about the world, not a field.
    | FacadeError::SyncPlanMoved { .. }
    | FacadeError::RenumberNotMerging { .. }
    | FacadeError::SyncDiskStep { .. }
    | FacadeError::Git(_)
    // The child-row halves of the same ruling, and the same reasoning: they
    // report that a KEY is taken, so there is no field the caller failed to
    // supply -- the remedy is `ac edit` / `at edit`, not a fuller create.
    | FacadeError::CriterionExists { .. }
    | FacadeError::TestExists { .. }
    // **NAMES SEVERAL FIELDS AND DEMANDS NO PARTICULAR ONE, WHICH IS WHY IT IS
    // HERE RATHER THAN ABOVE.** `ReasonRequired` and its two siblings each name
    // ONE field a call left out, and this file exists to prove that field
    // reaches a reader. `NothingToChange` says at least one of `--file`,
    // `--prose` and `--covers` had to be given; picking one of the three to
    // report would be a guess about what the caller meant, so there is no
    // authored value to carry. The roster it CAN offer is in its remedy, which
    // `error_remedies.rs` drives by value.
    | FacadeError::NothingToChange { .. }
    | FacadeError::NoSuchWorkPackage { .. }
    | FacadeError::NoSuchCriterion { .. }
    | FacadeError::NoSuchTest { .. }
    | FacadeError::GateBlocked { .. }
    | FacadeError::ComputedSatisfaction { .. }
    | FacadeError::NotOffScope { .. }
    | FacadeError::NotSatisfied { .. }
    // **CARRIES a field rather than DEMANDING one, and the distinction is what
    // this denominator is about.** It reports that a requirement is already
    // fiat-closed and quotes the standing `because` so the operator can see the
    // judgement they are about to replace -- but the caller left nothing out.
    // Supplying more would not make the call succeed; reinstating first would.
    | FacadeError::AlreadyFiatClosed { .. }
    | FacadeError::OffScope { .. }
    | FacadeError::WrongOffScopeState { .. }
    | FacadeError::BadQuery { .. }
    | FacadeError::SearchUnanswerable { .. }
    | FacadeError::NoSuchFace { .. }
    | FacadeError::IllegalTransition { .. }
    | FacadeError::DescopeTargetMissing { .. }
    | FacadeError::Unmigrated(_)
    | FacadeError::BelowMigrationFloor(_)
    // The two `migration.md` documented and did not implement (0271). Neither
    // demands a field: one reports the absence of a repository and the other
    // reports work the operator has not committed, so in both cases the cure
    // is an action on the world, never a fuller call.
    | FacadeError::MigrationWithoutGit
    | FacadeError::MigrationOverDirtyTree { .. }
    | FacadeError::Write(_)
    | FacadeError::ViewsNotWritten { .. }
    | FacadeError::Store(_)
    | FacadeError::Ingest(_)
    | FacadeError::NoSuchFormat { .. }
    | FacadeError::LossyFormat { .. }
    | FacadeError::ExportRoundTripFailed { .. }
    | FacadeError::NoSuchIssue { .. }
    | FacadeError::MigrationBlocked(_)
    | FacadeError::MigrationHalted { .. }
    | FacadeError::EgestFromRefusedIngest { .. }
    | FacadeError::EgestWouldEmptyTheEstate { .. }
    | FacadeError::EgestFromStaleStore { .. }
    | FacadeError::IngestOutpacedByWrites { .. }
    | FacadeError::WriteWouldEmptyAnAuthoredBody { .. }
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
    | FacadeError::NoManifestToUnlistFrom { .. }
    // A realisation refused because it would write over a difference names
    // the PATHS it would have destroyed; there is no field the caller left out.
    | FacadeError::HydrationWouldOverwrite { .. }
    // Same shape: it names the PATHS it would have removed, and no field was
    // left out of the call.
    | FacadeError::RealisationWouldRemove { .. }
    | FacadeError::DehydrationRefused { .. }
    // `intent edit`'s two refusals. Both are about WHERE a file may be
    // authored, not about a value the caller left out of one -- the argument
    // was complete and the answer is still no.
    | FacadeError::NotEditable { .. }
    | FacadeError::NoSuchEditable { .. }
    // The narrow setter's refusal. It is about WHICH DOOR writes a field, not
    // about a value left out of the call -- the caller supplied both the field
    // and the value, and every cause names the door that does open.
    //
    // **THE ONE CAUSE THAT LOOKS LIKE A DEMAND IS THE ONE TO STATE, RATHER
    // THAN THE ONES THAT PLAINLY ARE NOT.** Clearing a REQUIRED field with a
    // null renders as `missing field ...`, which reads like a demand -- but the
    // field it names is one the caller had and chose to remove, and it reaches a
    // read face already or the entity could not be rendered at all. Nothing
    // authored is lost behind it.
    | FacadeError::FieldNotWritable { .. }
    // A value the caller DID supply, refused for what it says rather than for
    // being absent. `st done --date 2026-02-30` names the field and the value
    // it was given, so nothing authored is hidden behind it -- the operator is
    // holding the string this refusal is about.
    | FacadeError::ValueNotRecordable { .. }
    // **THE CALLER SUPPLIED THE FIELD; THAT IS THE PROBLEM** (issue 0207).
    // `--note` was given and is perfectly recordable -- the refusal is about
    // what writing it would DESTROY. There is no omitted value to carry to a
    // reader, and telling the operator to supply a field they already supplied
    // is the one remedy that cannot help them.
    | FacadeError::NoteWouldBeLost { .. }
    // **THE CALLER SUPPLIED THE FILE; THE ROW'S KIND CANNOT HOLD IT** (0146).
    // Nothing was left out, so there is nothing to carry to a reader.
    | FacadeError::FileOnANonTestRow { .. }
    // A wrapped realisation failure. It reports that making files exist did not
    // work, not that a value was left out -- no authored prose behind it.
    | FacadeError::Realise(_)
    // **ic's, and they landed without these arms at db3f947a** -- `agents
    // generate` / `validate` moved onto the facade for MCP, and this binary
    // stopped compiling for every node until the next workspace-wide check.
    // Both wrap a fault in the INSTALL rather than in the call: the binary
    // outside any `lib/templates/`, or a template unreadable or malformed. The
    // caller supplied nothing and left nothing out; there is no authored value
    // behind either to carry to a reader.
    | FacadeError::Install(_)
    | FacadeError::RootFile(_)
    // 0351: canon's apply, the same install-or-filesystem fault as the two above.
    | FacadeError::Canon(_)
    // Issue 0206's refusal. It reports that the RECORD moved, so there is no
    // field the caller failed to supply -- the remedy is to re-run, not to
    // make a fuller call.
    | FacadeError::RecordMovedUnderTheWrite { .. }
    // Neither demands a field of the caller. `NoFormForEntity` is a fact about
    // the DECLARATION -- this kind has no form -- and no fuller call fixes it;
    // `EntityUnserialisable` is a fault in this process, where the caller
    // supplied nothing wrong and can supply nothing better.
    | FacadeError::NoFormForEntity { .. }
    | FacadeError::EntityUnserialisable { .. }
    // `0262`'s refusal. The caller supplied a path and it names nowhere; there
    // is no field they LEFT OUT, and the remedy is a different value for the
    // one they gave rather than a fuller call. The empty-path arm is the near
    // miss -- it looks like an omission and it is still the same argument,
    // wrong.
    | FacadeError::AttachmentPathNotInThread { .. }
    // `0394`'s refusal, the same argument: the caller named an attachment and
    // the thread carries none at that path. Nothing was LEFT OUT; the remedy is
    // one of the paths the thread does carry.
    | FacadeError::NoSuchAttachment { .. }
    // `0460`'s three refusals, the same argument: the caller named a target,
    // and it is not a thread, is the thread itself, or is not linked. Nothing
    // was LEFT OUT; the remedy is a different target.
    | FacadeError::NoSuchRelatedTarget { .. }
    | FacadeError::RelatedToItself { .. }
    | FacadeError::NoSuchRelated { .. }
    // `0398`'s refusal, the same argument again: the caller named a document and
    // the thread carries none by that name. Nothing was LEFT OUT; the remedy is
    // the door that attaches one.
    | FacadeError::NotCarried { .. }
    // `0338 (i)`'s refusal. The caller gave a whole address; it names another
    // project, and the remedy is this project's own spelling of it, not a field.
    | FacadeError::CrossProjectAddress { .. }
    // `0270`'s refusal. The caller supplied everything the verb takes; what is
    // missing is a FILE ON DISK, which is not a field of the call and cannot be
    // carried to a reader as one.
    // ST0069 WP-01's refusal. The caller supplied an id and it is not an issue
    // number at all; nothing was LEFT OUT, and the remedy is a different value
    // for the one they gave. Same argument as the two above it.
    | FacadeError::MalformedIssueId { .. }
    // ST0069 WP-14's board reader. The caller gave a moniker and no node
    // carries it; nothing was LEFT OUT, and the remedy is a different value for
    // the one they gave, or a registration. Same argument as the two above it.
    | FacadeError::WbNodeNotRegistered { .. }
    // ST0069 WP-14's bounds. Nothing was LEFT OUT in either: one body is longer
    // than the node's bound and one inbox already holds its limit, so the
    // remedy is a shorter entry or a cleared inbox rather than a field.
    | FacadeError::WbBodyOverBound { .. }
    | FacadeError::WbInboxFull { .. }
    // The same reading for the item bound and the claim address: one board holds
    // its limit of a kind, and one value is not an address. Neither is a field
    // left out of the call.
    | FacadeError::WbItemsFull { .. }
    | FacadeError::WbClaimMalformed { .. }
    | FacadeError::WbAlreadyCarried { .. }
    | FacadeError::WbSendersNotRegistered { .. }
    // Issue 0424's refusal: the call takes no fields at all. What is missing is
    // a `key: value` line in a board header ON DISK, which is not a field of the
    // call and cannot be carried to a reader as one.
    | FacadeError::WbHeaderIncomplete { .. }
    | FacadeError::WbNotMigrated { .. }
    // A kind another verb owns: the caller supplied everything, and the remedy
    // is a different VERB rather than a field they left out.
    | FacadeError::WbKindHasItsOwnVerb { .. }
    // A directive on a board that is not `hv`'s, written or carried: the caller
    // supplied everything, and the remedy is a different BOARD.
    | FacadeError::WbDirectiveOffHv { .. }
    | FacadeError::WbDirectivesOnAnotherBoard { .. }
    // A moniker registered with other values: the caller supplied every field,
    // and the remedy is a different moniker rather than one they left out.
    | FacadeError::WbRegisteredDifferently { .. }
    // Arguments against a board's header, and a correction of a moniker nobody
    // registered: every field was supplied, and the remedy is other VALUES or
    // the plain verb, never a field left out.
    | FacadeError::WbRegisterDisagreesWithHeader { .. }
    | FacadeError::WbCorrectUnregistered { .. }
    // The carry's refusals of vc decision 20: the caller supplied everything,
    // and the remedy is an edited board, `--drop-uncarried`, or a snapshot moved
    // aside -- none of them a field left out.
    | FacadeError::WbUncarried { .. }
    | FacadeError::WbSnapshotInTheWay { .. }
    // **A CALL PARAMETER IS MISSING AND A MODEL FIELD IS NOT, and this list is
    // about the second.** `WbNoActingNode` does demand something -- `--node` or
    // `INTENT_NODE` -- but no entity carries it, so there is no read face for
    // it to reach and claiming one here would send this test looking for a
    // field that does not exist.
    | FacadeError::WbNoActingNode
    | FacadeError::NoResolver { .. }
    // A search by target refused on the store's state or on the target's
    // spelling: the caller supplied every field, and the remedy is a run or
    // another spelling, never a field left out.
    | FacadeError::NothingResolved { .. }
    | FacadeError::NoSuchTarget { .. }
    | FacadeError::VerdictCitesAbsentFile { .. } => None,
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

/// **THE ESTATE STOPPED BEING A ZERO, SO THIS ROW NOW DRIVES IT.**
///
/// It used to assert the estate carried NO `status_reason`, and its own
/// failure message said what to do when that stopped being true: *this row can
/// now be driven against the real estate instead of only constructed fixtures,
/// and it should be.* **That is a tripwire rather than a prohibition, and it
/// fired on 2026-08-25** when vc parked ST0059 with `st hold --reason` under
/// hv's instruction. Ordinary work created the first real instance; the row was
/// waiting for exactly that and said so in advance.
///
/// **THE FIXTURES ABOVE ARE NOT REPLACED.** They cover every demanded field,
/// including ones the estate does not carry, and they hold the negative
/// control. This adds the arm the fixtures cannot have: the property proven on
/// data nobody constructed for it.
#[test]
fn the_estate_status_reasons_reach_a_read_face() {
  use intentsvcs::project::Project;
  let root = testkit::repo_root();
  let project = Project::open(&root).expect("this repository is a project");
  let canon = intentsvcs::ingest::read(&project).expect("the estate reads");

  assert!(
    !canon.threads.is_empty(),
    "vacuous unless the estate has threads at all"
  );

  let carrying: Vec<&Thread> = canon
    .threads
    .iter()
    .filter(|t| t.status_reason.is_some())
    .collect();

  // NOT a silent skip when the estate is empty of them. A row that passes both
  // when the property holds and when there is nothing to check is the shape
  // this file exists to refuse -- so say which case ran.
  assert!(
    !carrying.is_empty(),
    "no thread in the estate carries a status_reason, so this row proved\n       \
     nothing. It was written when ST0059 acquired one. If the estate has\n       \
     legitimately returned to zero, restore the assertion that it IS zero --\n       \
     do not leave a row that reads green over an empty population."
  );

  for thread in &carrying {
    let reason = thread
      .status_reason
      .as_deref()
      .expect("filtered on is_some");
    let rendered = views::info(thread, &ctx());
    assert!(
      rendered.contains(reason),
      "{}'s status_reason does not survive to anything a person reads.\n       \
       The reason is:\n{reason}\n       \
       The view rendered was:\n{rendered}",
      thread.id
    );
  }
}
