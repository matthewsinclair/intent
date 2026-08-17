//! **ONE NORMALISER FOR EVERY v2 FREE-TEXT VOCABULARY, and the second of two
//! callers finally telling ABSENT apart from UNREADABLE.**
//!
//! Two defects, both of them one decision that had been applied to one site and
//! not to its neighbour twenty lines away.
//!
//! # The separator family was open on two of three vocabularies
//!
//! `scope` folded ` `, `-` and `_` away before parsing. The two status tables
//! matched literals instead, and spelled out the space and the hyphen forms but
//! not the underscore -- so one file accepted `not started` and `not-started`
//! and rejected `not_started`, and **nothing about the field made underscore
//! the odd one out.**
//!
//! **Adding an arm would have been the wrong fix.** It leaves two rules
//! different in one file and the next spelling finds the same crack; folding
//! CLOSES the family rather than enumerating it, and shortens the tables on the
//! way -- `not started` / `notstarted` / `not-started` were three literals for
//! one token.
//!
//! Measured across the whole fleet before anything moved (working trees,
//! 2026-08-17): **work packages carry `NOT_STARTED` 13 times -- Lamplight 10
//! and Laksa 3.** All 13 sit in Completed threads, so this moves 13 rows from
//! "carried with a finding" to "read correctly" and **changes nothing about
//! what blocks.**
//!
//! **A RECONCILIATION THAT CLOSED ARITHMETICALLY AND WAS FABRICATED, RETRACTED
//! HERE BECAUSE IT SHIPPED IN THIS COMMENT.** vc had published a `10`, so this
//! file first said their corpus reports 10 "because Laksa is not in it" and
//! declared the two figures reconciled. **They were never measuring the same
//! thing**: vc's 10 is what BLOCKS Lamplight -- nine unparseable AT rows and
//! one broken reference, in two `acceptance.md` files, not a status value among
//! them -- and the sentence above says these 13 block nothing, so the two sets
//! are disjoint by this file's own account.
//!
//! **The coincidence is what made the story easy, and that is the lesson worth
//! more than the number.** Two unrelated measurements landed on 10, on one
//! estate, on one day; 13 minus 10 is 3, and Laksa has exactly 3. **The
//! arithmetic closed perfectly, which is the most persuasive form a wrong
//! explanation can take** -- an invented cause that reconciles is far harder to
//! doubt than one that does not. The 13 stands because it was measured
//! directly, on both projects, by counting the rows.
//!
//! # Absent and unreadable were one finding at the thread level
//!
//! The work-package reader already draws the distinction -- `FieldNotRecorded`
//! for a file that predates the convention, `UnknownStatus` for a value v2 read
//! as free text -- and it draws it because **79 work packages fleet-wide have
//! no `status:` line at all.** The thread reader answered both with `thread
//! status "" is not in the v2 vocabulary`: a sentence that sends the operator
//! to fix a vocabulary problem that does not exist, **on the arm that BLOCKS.**
//!
//! **MEASURED, and the two arms have very different populations** (fleet
//! working trees, 2026-08-17, 715 threads). **ABSENT: zero** -- every fleet
//! thread carries a `status:` line, so the rewording corrects no live estate
//! and is not claimed to. **UNREADABLE: two** -- `SUPERSEDED` in Laksa and
//! `DESCOPED` in Lamplight, and Laksa's is the single finding that blocks its
//! whole migration today.
//!
//! **So the arm being reworded is the one nobody can reach, and the arm that IS
//! reached is what makes the pair worth having.** Both land on the same
//! unknowable-`closed` path. The absent arm is here because it is the same
//! decision applied to the second of two callers, and because the state is
//! demonstrably reachable in this data model -- 79 times, for the sibling
//! entity, in the same frontmatter.
//!
//! **The policy does not move, only the diagnosis.** Both arms still block, and
//! must: the thread's status decides `closed`, so a thread that cannot say
//! whether it is closed cannot have the carry policy applied to it at all --
//! and both of the fleet's two live instances sit in a `COMPLETED/` bucket
//! whose name the migrator deliberately does not trust.

mod common;

use common::Fixture;
use intentsvcs::finding::FindingClass;
use intentsvcs::legacy;
use intentsvcs::model::{ThreadStatus, WpStatus};

/// A v2 estate whose thread frontmatter is written verbatim, so a test can omit
/// the `status:` line entirely rather than merely blank it.
fn estate(fixture: &Fixture, thread_front: &str, wp_status_line: &str) {
  fixture.write_file(
    "intent/.config/config.json",
    "{\"intent_version\":\"2.19.0\",\"project_name\":\"P\",\"author\":\"cc\",\"intent_dir\":\"intent\",\"languages\":[\"rust\"]}\n",
  );
  fixture.write_file(
    "intent/st/ST0001/info.md",
    &format!("---\n{thread_front}---\n\n# ST0001: A thread\n\n## Objective\n\nShip it.\n"),
  );
  fixture.write_file(
    "intent/st/ST0001/WP/01/info.md",
    &format!(
      "---\ntitle: A work package\nscope: S\n{wp_status_line}---\n\n# WP01: A work package\n\n## Objective\n\nDo it.\n"
    ),
  );
}

fn scan(fixture: &Fixture) -> legacy::Scan {
  legacy::scan(&fixture.project()).expect("scan the v2 estate")
}

fn thread_at(status: &str) -> String {
  format!("intent_version: 2.19.0\nstatus: {status}\ncreated: 20260816\n")
}

// ---------------------------------------------------------------------------
// One normaliser
// ---------------------------------------------------------------------------

/// **The underscore family, on both status vocabularies and the scope one.**
///
/// Driven as a table because the point is that ONE rule serves all of them: a
/// per-vocabulary fix would satisfy any one row here and fail its neighbours.
#[test]
fn every_separator_spelling_of_a_token_reads_the_same() {
  for spelling in ["Not Started", "NOT-STARTED", "not_started", "NotStarted"] {
    let fixture = Fixture::new();
    estate(
      &fixture,
      &thread_at(spelling),
      &format!("status: {spelling}\n"),
    );
    let scan = scan(&fixture);

    assert_eq!(
      scan.threads[0].status,
      ThreadStatus::NotStarted,
      "thread status {spelling:?} did not read as Not Started"
    );
    assert_eq!(
      scan.threads[0].wps[0].status,
      WpStatus::NotStarted,
      "work-package status {spelling:?} did not read as Not Started"
    );
    assert!(
      scan.residue.is_empty() && scan.carried.is_empty(),
      "{spelling:?} is a spelling of a value in the vocabulary, so it is not a \
       finding: {:?} {:?}",
      scan.residue,
      scan.carried
    );
  }
}

/// The same rule on the third vocabulary, which is where it came from.
#[test]
fn the_scope_vocabulary_folds_the_same_separators() {
  for spelling in ["Extra Small", "extra-small", "EXTRA_SMALL"] {
    let fixture = Fixture::new();
    fixture.write_file(
      "intent/.config/config.json",
      "{\"intent_version\":\"2.19.0\",\"project_name\":\"P\",\"author\":\"cc\",\"intent_dir\":\"intent\",\"languages\":[\"rust\"]}\n",
    );
    fixture.write_file(
      "intent/st/ST0001/info.md",
      &format!("---\n{}---\n\n# ST0001: A thread\n", thread_at("Completed")),
    );
    fixture.write_file(
      "intent/st/ST0001/WP/01/info.md",
      &format!(
        "---\ntitle: A work package\nscope: {spelling}\nstatus: Done\n---\n\n# WP01: A work package\n"
      ),
    );
    let scan = scan(&fixture);
    assert_eq!(
      scan.threads[0].wps[0].scope,
      Some(intentsvcs::model::TShirt::XS),
      "scope {spelling:?} did not read as XS"
    );
  }
}

/// **THE NEGATIVE CONTROL, and without it every test above passes against a
/// normaliser that strips every character it does not like.** A token that is
/// genuinely outside the vocabulary must still be outside it after folding.
#[test]
fn folding_separators_does_not_admit_a_value_that_is_not_in_the_vocabulary() {
  let fixture = Fixture::new();
  estate(&fixture, &thread_at("Super_Seded"), "status: Done\n");
  let scan = scan(&fixture);

  assert!(
    scan
      .residue
      .iter()
      .any(|f| f.class == FindingClass::UnknownStatus),
    "`Super_Seded` folds to `superseded`, which is still not a v2 status: {:?}",
    scan.residue
  );
}

// ---------------------------------------------------------------------------
// Absent is not unreadable
// ---------------------------------------------------------------------------

/// A thread with no `status:` line at all is told what is actually wrong.
#[test]
fn a_thread_that_never_recorded_a_status_says_so() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "intent_version: 2.19.0\ncreated: 20260816\n",
    "status: Done\n",
  );
  let scan = scan(&fixture);

  let finding = scan
    .residue
    .iter()
    .find(|f| f.class == FindingClass::FieldNotRecorded)
    .unwrap_or_else(|| panic!("no FieldNotRecorded finding: {:?}", scan.residue));
  assert!(
    finding
      .detail
      .contains("no thread status was ever recorded"),
    "{finding:?}"
  );
  assert!(
    !finding.detail.contains("not in the v2 vocabulary"),
    "an absent field is not a vocabulary problem, and saying it is sends the \
     operator to repair a value that does not exist: {finding:?}"
  );
}

/// **The other half, and the pair is the whole point.** A test for the absent
/// arm alone passes against a reader that calls everything `FieldNotRecorded`.
#[test]
fn a_thread_whose_status_is_outside_the_vocabulary_still_says_that() {
  let fixture = Fixture::new();
  estate(&fixture, &thread_at("Quokka"), "status: Done\n");
  let scan = scan(&fixture);

  let finding = scan
    .residue
    .iter()
    .find(|f| f.class == FindingClass::UnknownStatus)
    .unwrap_or_else(|| panic!("no UnknownStatus finding: {:?}", scan.residue));
  assert!(
    finding.detail.contains("\"Quokka\"") && finding.detail.contains("not in the v2 vocabulary"),
    "the value somebody recorded is quoted back at them: {finding:?}"
  );
}

/// **An unclassifiable thread is never guessed CLOSED, because `closed` is what
/// routes every finding underneath it.**
///
/// **THE FIRST VERSION OF THIS TEST COULD NOT SEE THE HAZARD ITS OWN DOC
/// COMMENT NAMED**, and only a mutation found that. It asserted
/// `!scan.residue.is_empty()` -- which is satisfied by the thread's OWN status
/// finding, emitted through `out.block` on the line beside the code under test,
/// whatever `closed` is set to. Guessing `closed = true` for an unreadable
/// status left it green.
///
/// So the subject has to be a SECOND finding, one that is routed rather than
/// blocked outright: the work package below carries a status v2 never had.
/// **On a thread nobody can classify that finding must block; if the thread is
/// guessed closed it silently becomes a carry**, and hv's policy -- closed
/// threads convert lossless-by-carrying, live ones stay BLOCKED-until-clean --
/// gets applied on the strength of a guess.
#[test]
fn an_unclassifiable_thread_does_not_carry_the_rows_underneath_it() {
  for front in [
    "intent_version: 2.19.0\ncreated: 20260816\n".to_string(),
    thread_at("Quokka"),
  ] {
    let fixture = Fixture::new();
    estate(&fixture, &front, "status: Kumquat\n");
    let scan = scan(&fixture);

    let wp_finding = |findings: &[intentsvcs::finding::Finding]| {
      findings
        .iter()
        .any(|f| f.detail.contains("work-package status"))
    };
    assert!(
      wp_finding(&scan.residue),
      "the work package's own finding is not blocking on a thread whose status \
       could not be read: residue={:?} carried={:?}",
      scan.residue,
      scan.carried
    );
    assert!(
      !wp_finding(&scan.carried),
      "it was CARRIED, which means the thread was guessed closed: {:?}",
      scan.carried
    );
  }
}
