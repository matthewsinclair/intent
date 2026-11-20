//! ST0095 WP-09: **a hashed work package's criteria follow its id, and doctor
//! proves each hashed id against the event that made it.**
//!
//! AC-09.1's arm has no red on the base: the close gate already reads a
//! label's group as a typed package id (issue 0712, and WP-07's whole labels),
//! so it is a regression arm for the hashed family. AC-09.2's and AC-09.3's
//! arms are red on the base, which has neither check.

use crate::common::{Fixture, ctx, sample_issue, sample_thread};
use intentsvcs::facade::FacadeError;
use intentsvcs::finding::{Finding, FindingClass};
use intentsvcs::id::{IssueId, ThreadId, WpId};
use intentsvcs::model::{AcKind, TShirt};

const ORIGIN: &str = "01M7SQ544Q2FK5BNMYAMXV3Z4Z";
const OTHER: &str = "01M7SQ544Q2FK5BNMYAMXV3Z50";

fn findings(fx: &Fixture, class: FindingClass) -> Vec<Finding> {
  intentsvcs::doctor::diagnose(&fx.project(), &ctx(), None, intentsvcs::doctor::Scope::All)
    .findings
    .into_iter()
    .filter(|f| f.class == class)
    .collect()
}

/// AT-09.1 (ST0095 AC-09.1): `wp done` refuses a hashed package while a
/// criterion of its own is unsatisfied. A regression arm: no red on the base,
/// whose gate already reads the group as a typed package id.
#[test]
fn wp_done_refuses_a_hashed_package_while_a_criterion_of_its_own_is_unsatisfied() {
  let fx = Fixture::new();
  let mut facade = fx.facade();
  let st = facade.st_new("A thread").expect("a thread");
  let seq = facade
    .wp_new(&st, "A package", TShirt::S)
    .expect("a package");
  assert!(seq.is_hashed(), "CONTROL: the mint makes a hashed package");
  facade
    .ac_new(
      &st,
      &format!("AC-{}.1", seq.shown()),
      "It holds.",
      AcKind::NonTest,
    )
    .expect("a criterion of its own, named by the package's short form");
  facade.wp_start(&st, seq).expect("started");
  let refused = facade.wp_done(&st, seq);
  assert!(
    matches!(refused, Err(FacadeError::GateBlocked { .. })),
    "the close gate refuses the package on its own unsatisfied criterion, never passing it as having none: {refused:?}"
  );
}

/// AT-09.2 (ST0095 AC-09.2): a hashed id that does not answer to its origin,
/// or names none, is a counted finding.
#[test]
fn doctor_counts_a_hashed_id_that_does_not_answer_to_its_origin() {
  let fx = Fixture::new();
  let mut thread = sample_thread("ST0001");
  thread.id = ThreadId::from_origin(ORIGIN);
  thread.origin = Some(OTHER.to_string());
  fx.write_thread(&thread);
  let mut issue = sample_issue(1);
  issue.number = IssueId::from_origin(ORIGIN);
  issue.origin = None;
  fx.write_issue(&issue);

  let found = findings(&fx, FindingClass::IdNotFromOrigin);
  assert!(
    found
      .iter()
      .any(|f| f.detail.contains(&thread.id.stored()) && f.detail.contains(OTHER)),
    "a thread whose id is not the hash of its origin is named, with the origin: {found:?}"
  );
  assert!(
    found
      .iter()
      .any(|f| f.detail.contains(&issue.number.stored()) && f.detail.contains("no origin")),
    "a hashed issue that names no origin is named: {found:?}"
  );
  assert!(
    FindingClass::IdNotFromOrigin.is_actionable(),
    "and it is counted: an id that does not answer to its event was written by hand"
  );
}

/// AT-09.3 (ST0095 AC-09.3): two packages of one thread sharing a short form
/// are named as an advisory, and nothing counted names them.
#[test]
fn doctor_names_two_packages_of_one_thread_that_share_a_short_form_as_advisory() {
  // The first two origins, in one deterministic sequence, whose packages share
  // a short form: one pass, keyed on the short form.
  let mut seen: std::collections::HashMap<String, String> = std::collections::HashMap::new();
  // One more candidate than there are short forms, so a pair must exist.
  let candidates = 16_u32.pow(intentsvcs::id::WP_SHOWN_HEX as u32);
  let (a, b) = (0..=candidates)
    .map(|n| format!("01M7SQ544Q2FK5BNMYA{n:07}"))
    .find_map(|origin| {
      seen
        .insert(WpId::from_origin(&origin).shown(), origin.clone())
        .map(|earlier| (earlier, origin))
    })
    .expect("two origins whose packages share a short form");
  assert_ne!(
    WpId::from_origin(&a),
    WpId::from_origin(&b),
    "CONTROL: two packages, not one named twice"
  );
  let fx = Fixture::new();
  let mut thread = sample_thread("ST0001");
  for (wp, origin) in thread.wps.iter_mut().zip([&a, &b]) {
    wp.seq = WpId::from_origin(origin);
    wp.origin = Some(origin.clone());
  }
  fx.write_thread(&thread);

  let found = findings(&fx, FindingClass::ShortFormShared);
  assert!(
    found
      .iter()
      .any(|f| f.detail.contains(&WpId::from_origin(&a).stored())
        && f.detail.contains(&WpId::from_origin(&b).stored())),
    "both whole ids are named under their one short form: {found:?}"
  );
  let counted = findings(&fx, FindingClass::DuplicateId);
  assert!(
    counted.is_empty(),
    "two packages that share a short form are not one package declared twice, so nothing counted names them: {counted:?}"
  );
  assert!(
    !FindingClass::ShortFormShared.is_actionable(),
    "advisory: each id is whole in canon, so nothing is owed"
  );
}
