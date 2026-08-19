//! AT-04.3 / AC-04.3: **`organize` NEVER resolves an attachment divergence: it
//! reports the path, names both verbs, and modifies neither side.**
//!
//! **"MODIFIES NEITHER SIDE" AND "REPORTS THE PATH" ARE TWO CLAIMS, AND THE
//! FIRST ONE PASSES VACUOUSLY.** A verb that ignored attachments entirely would
//! satisfy every assertion about not touching them, and would satisfy none of
//! the criterion. The first version of `organize` did exactly that -- it matched
//! `ThreadFile::Attachment` and fell through to `{}` -- so the arm this file
//! drives exists because the silent-pass was noticed, not because it failed.
//!
//! **AND A DIVERGENCE IS NOT A REFUSAL.** It does not abort the run: the other
//! rows still apply. Authority follows AUTHORSHIP, so this one path is the one
//! thing `organize` is forbidden to decide, and stopping the whole verb over it
//! would make an unrelated hydration hostage to a human's unread report.

mod common;

use std::collections::BTreeMap;
use std::path::PathBuf;

use common::{Fixture, ctx, sample_thread};
use intentsvcs::ingest::Canon;
use intentsvcs::intentfiles;
use intentsvcs::model::Attachment;
use intentsvcs::organize::{Action, Step, TreeState, plan};

const DECLARED: &str = "STEELTHREAD:ST0001\n\n# BEGIN INTENT\n# END INTENT\n";
/// Declares a thread that is not in the canon, so ST0001's attachments are
/// present-and-undeclared.
const DECLARES_NOTHING_PRESENT: &str = "STEELTHREAD:ST0009\n\n# BEGIN INTENT\n# END INTENT\n";

fn canon() -> Canon {
  Canon {
    threads: vec![sample_thread("ST0001")],
    ..Default::default()
  }
}

/// The path `sample_thread`'s first attachment realises to.
fn reference_md(project: &intentsvcs::project::Project) -> PathBuf {
  project.st_dir().join("ST0001").join("reference.md")
}

/// The sha the store carries for it, taken from the one constructor rather than
/// hand-written -- a literal here would be a second expression of the hash and
/// would go stale the moment the fixture text changes.
fn carried_sha() -> String {
  Attachment::new("reference.md", "# Reference\n\nA quokka.\n").sha256
}

fn step_for<'a>(steps: &'a [Step], path: &PathBuf) -> Option<&'a Step> {
  steps.iter().find(|s| &s.path == path)
}

/// **ONE FIXTURE PER TEST, PASSED IN.** The first version of this helper made its
/// OWN `Fixture`, so a tree built from an outer fixture's paths was compared
/// against a plan computed under a different tempdir root and five of seven tests
/// failed against correct code. A helper that constructs the world it is asked to
/// measure cannot be handed a world.
fn plan_in(fx: &Fixture, manifest: &str, tree: TreeState) -> Vec<Step> {
  plan(
    &fx.project(),
    &canon(),
    &intentfiles::parse(manifest).expect("manifest parses"),
    &ctx(),
    &tree,
    "d".to_string(),
  )
  .steps
}

#[test]
fn an_attachment_that_agrees_with_the_store_produces_no_step() {
  let fx = Fixture::new();
  let path = reference_md(&fx.project());
  let tree = TreeState {
    present: [path.clone()].into_iter().collect(),
    sha256: BTreeMap::from([(path.clone(), carried_sha())]),
  };
  let steps = plan_in(&fx, DECLARED, tree);
  assert!(
    step_for(&steps, &path).is_none(),
    "an attachment matching the store needs no action, and a step meaning `do nothing` in a list whose every other member means an action is how a report stops being readable"
  );
}

#[test]
fn a_divergent_attachment_is_reported_and_neither_side_is_touched() {
  let fx = Fixture::new();
  let path = reference_md(&fx.project());
  let tree = TreeState {
    present: [path.clone()].into_iter().collect(),
    sha256: BTreeMap::from([(path.clone(), "0".repeat(64))]),
  };
  let steps = plan_in(&fx, DECLARED, tree);
  let step = step_for(&steps, &path).expect("a divergent attachment must be REPORTED");
  assert_eq!(step.action, Action::AttachmentDiverged);
  assert!(
    !step.action.is_destructive(),
    "organize must not remove the divergent file"
  );
  assert!(
    step.content.is_none(),
    "carrying the store's bytes on this step is how a later apply comes to overwrite the disk copy: authority follows authorship, and the store is the stale side here"
  );
}

#[test]
fn the_report_names_both_verbs() {
  // The half of AC-04.3 that a report-existence assertion cannot see. Naming one
  // remedy would be organize choosing whose work to discard.
  let remedy = Action::AttachmentDiverged
    .remedy()
    .expect("a divergence must carry a remedy");
  assert!(
    remedy.contains("--to-store"),
    "the take-the-disk-copy verb must be named: {remedy}"
  );
  assert!(
    remedy.contains("restore"),
    "the keep-the-store-copy remedy must be named too: {remedy}"
  );
  assert!(
    remedy.contains("will not choose"),
    "the report must say that organize is declining to decide, not merely offer options: {remedy}"
  );
}

#[test]
fn an_unhashable_attachment_is_treated_as_divergent_not_as_agreeing() {
  // Present, declared, and absent from the hash map: whether it matches the
  // store is UNANSWERED. Reporting an unanswered question as agreement is how a
  // check comes to mean nothing, and it fails in the direction that loses the
  // file rather than the direction that costs a line of output.
  let fx = Fixture::new();
  let path = reference_md(&fx.project());
  let tree = TreeState {
    present: [path.clone()].into_iter().collect(),
    sha256: BTreeMap::new(),
  };
  let steps = plan_in(&fx, DECLARED, tree);
  assert_eq!(
    step_for(&steps, &path).map(|s| s.action),
    Some(Action::AttachmentDiverged),
    "an unhashable attachment must report, not pass"
  );
}

#[test]
fn a_declared_attachment_absent_from_disk_hydrates_from_the_store() {
  let fx = Fixture::new();
  let path = reference_md(&fx.project());
  let steps = plan_in(&fx, DECLARED, TreeState::default());
  let step = step_for(&steps, &path).expect("a declared attachment must be realised");
  assert_eq!(step.action, Action::HydrateAttachment);
  assert_eq!(
    step.content.as_deref(),
    Some("# Reference\n\nA quokka.\n"),
    "it hydrates from the store's carried bytes, not from a render -- nothing renders an attachment"
  );
}

#[test]
fn a_nested_attachment_path_is_realised_under_the_thread() {
  // `parity/cmd-st.md` is the fixture's second attachment and the only one with a
  // separator in it. A path joined wrongly is invisible in a flat fixture and
  // lands outside the thread directory in a real tree.
  let fx = Fixture::new();
  let project = fx.project();
  let nested = project.st_dir().join("ST0001").join("parity/cmd-st.md");
  let steps = plan_in(&fx, DECLARED, TreeState::default());
  assert_eq!(
    step_for(&steps, &nested).map(|s| s.action),
    Some(Action::HydrateAttachment),
    "a nested attachment realises under its own thread directory"
  );
}

#[test]
fn an_undeclared_attachment_dehydrates_through_the_gate() {
  // Row four applies to attachments too, or a de-realised thread leaves its
  // attachments behind and AC-04.6's equality can never hold. It carries the
  // store's bytes precisely so the gate can prove the copy matches before
  // anything is removed -- the store having a copy is what makes removal safe.
  let fx = Fixture::new();
  let path = reference_md(&fx.project());
  let tree = TreeState {
    present: [path.clone()].into_iter().collect(),
    sha256: BTreeMap::from([(path.clone(), carried_sha())]),
  };
  let steps = plan_in(&fx, DECLARES_NOTHING_PRESENT, tree);
  let step = step_for(&steps, &path).expect("an undeclared attachment must be decided");
  assert_eq!(step.action, Action::Dehydrate);
  assert!(
    step.content.is_some(),
    "the gate compares against these bytes; without them every removal is unproven and must be refused"
  );
}
