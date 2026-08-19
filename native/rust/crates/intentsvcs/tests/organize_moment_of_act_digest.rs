//! AT-04.5 / AC-04.5: **`organize` refuses when the tree digest computed
//! immediately before the irreversible step differs from the one it measured, and
//! the refusal names the difference.**
//!
//! **IT LOCKS AGAINST ANY PROCESS, NOT AGAINST OTHER `organize` RUNS.** `info`,
//! `st list`, `doctor` and `export` all materialise the store on access, so a
//! peer typing a read verb is enough to move the tree. That is why the defence
//! cannot be "the estate was quiet when I measured" -- quietness is not
//! establishable over any window, and a lock held against one's own kind leaves
//! the common case uncovered.
//!
//! **THE ORDER IS THE PROPERTY, AND IT IS ASSERTED DIRECTLY.** A guard that ran
//! after the removals would be a report, not a refusal. The decisive arm here is
//! not that a stale digest returns an error -- it is that the file is STILL THERE
//! afterwards.

mod common;

use std::cell::Cell;
use std::path::PathBuf;

use common::{Fixture, ctx, gate_open, sample_thread};
use intentsvcs::ingest::Canon;
use intentsvcs::intentfiles;
use intentsvcs::organize::{Action, OrganizeError, Plan, Step, TreeState, plan};
use intentsvcs::preconditions;

const PLANNED: &str = "digest-as-planned";
const MOVED: &str = "digest-after-a-peer-wrote";

fn canon() -> Canon {
  Canon {
    threads: vec![sample_thread("ST0001")],
    ..Default::default()
  }
}

/// Canon in which AC-00.1's ship gate is genuinely open.
///
/// **The digest guard is the subject here, so the OTHER gate has to be out of
/// the way -- and the only honest way to move it is to satisfy it.** With the
/// ship gate refusing, `apply` takes no irreversible step at all, so every
/// assertion below would pass for the wrong reason: the file survives because
/// nothing was ever going to remove it, not because the digest moved.
fn gate_is_open() -> Canon {
  Canon {
    threads: vec![gate_open()],
    ..Default::default()
  }
}

/// A plan holding exactly one removal, whose bytes match disk so the gate would
/// clear it. Any refusal is therefore the digest guard and nothing else.
fn removal_plan(fx: &Fixture, rel: &str) -> (PathBuf, Plan) {
  let body = "# gone\n";
  fx.write_file(rel, body);
  let path = fx.path(rel);
  (
    path.clone(),
    Plan {
      steps: vec![Step {
        path,
        action: Action::Dehydrate,
        content: Some(body.to_string()),
      }],
      digest: PLANNED.to_string(),
      preconditions: preconditions::check(&gate_is_open()),
    },
  )
}

#[test]
fn a_matching_digest_lets_the_removal_proceed() {
  let fx = Fixture::new();
  let (path, p) = removal_plan(&fx, "doomed.md");
  let report = p
    .apply(&|| PLANNED.to_string())
    .expect("an unmoved tree applies");
  assert_eq!(report.dehydrated, vec![path.clone()]);
  assert!(!path.exists(), "the removal must actually happen");
}

#[test]
fn a_moved_tree_refuses_and_the_file_survives() {
  // The arm that distinguishes a guard from a report. If the digest were checked
  // after the removals, this assertion on `path.exists()` is the only one that
  // would fail -- the error would arrive exactly the same.
  let fx = Fixture::new();
  let (path, p) = removal_plan(&fx, "doomed.md");
  let err = p.apply(&|| MOVED.to_string()).unwrap_err();
  assert!(
    matches!(err, OrganizeError::TreeMoved { .. }),
    "a moved tree must refuse, got {err:?}"
  );
  assert!(
    path.exists(),
    "THE REFUSAL MUST PRECEDE THE IRREVERSIBLE STEP: the file is still here or the guard is a post-mortem"
  );
}

#[test]
fn the_refusal_names_the_difference() {
  let fx = Fixture::new();
  let (_, p) = removal_plan(&fx, "doomed.md");
  let text = p.apply(&|| MOVED.to_string()).unwrap_err().to_string();
  assert!(
    text.contains(PLANNED) && text.contains(MOVED),
    "AC-04.5 asks for the difference to be NAMED, not merely detected: {text}"
  );
  assert!(
    text.contains("re-run"),
    "a refusal over a race must say the remedy is to re-plan, or the operator has no next move: {text}"
  );
}

#[test]
fn a_plan_with_nothing_to_remove_does_not_consult_the_digest() {
  // A hydration-only run has no irreversible step, so there is nothing for the
  // guard to protect. Refusing it because a peer touched an unrelated file would
  // train operators to re-run until it passes, which is how a guard stops being
  // one -- and the criterion says "immediately before the irreversible step",
  // which is a step this plan does not have.
  let fx = Fixture::new();
  let project = fx.project();
  let p = plan(
    &project,
    &canon(),
    &intentfiles::parse("STEELTHREAD:ST0001\n\n# BEGIN INTENT\n# END INTENT\n")
      .expect("manifest parses"),
    &ctx(),
    &TreeState::default(),
    PLANNED.to_string(),
  );
  assert!(!p.is_destructive(), "fixture must be hydration-only");
  // `Cell` rather than a plain `bool` because `apply` takes `&dyn Fn`, not
  // `FnMut` -- a digest function that could mutate its environment would be a
  // digest function that could disagree with itself between calls.
  let consulted = Cell::new(false);
  let report = p
    .apply(&|| {
      consulted.set(true);
      MOVED.to_string()
    })
    .expect("a non-destructive plan applies regardless of the tree moving");
  assert!(
    !consulted.get(),
    "the digest must not even be COMPUTED when there is nothing irreversible to guard"
  );
  assert!(!report.hydrated.is_empty(), "and it must actually hydrate");
}
