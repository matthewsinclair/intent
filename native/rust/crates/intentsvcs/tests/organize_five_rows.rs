//! AT-04.1 / AC-04.1: **`organize` implements exactly the five rows of D57-3's
//! table, and every row is exercised.**
//!
//! **EVERY ROW GETS ITS OWN ASSERTION, INCLUDING THE ONE THAT DOES NOTHING.**
//! Row three -- not declared, absent -- has no observable action, so the only way
//! to check it is to assert that no step names that path. Skipping it would
//! leave "emits a spurious step for an artefact that is neither declared nor
//! present" undetected, and that defect is invisible in any test that only looks
//! at the steps it expected to find.
//!
//! **AND THE FIFTH ROW IS ASSERTED TWICE: reported AND not removed.** Those are
//! two claims. A verb that classified a path as `Unclaimed` and then removed it
//! anyway would satisfy a report-only assertion perfectly.
//!
//! **THE UNCLAIMED FIXTURE IS `diagram.png` AND NOT A `.md`, WHICH IS NARROWER
//! THAN IT LOOKS.** `ATTACHMENT_EXTENSIONS` is `md`, `txt`, `sh`, so a hand-written
//! markdown file dropped in a thread directory is CARRIED as an attachment, not
//! reported as unclaimed. The first draft of this test used `notes-by-hand.md`
//! and failed -- correctly, against correct code. Row five is "neither a view nor
//! an attachment", and reading it as "anything the renderer did not make" makes
//! the common case look broken while the rare one goes unexercised.

mod common;

use std::path::PathBuf;

use common::{Fixture, ctx, sample_thread};
use intentsvcs::ingest::Canon;
use intentsvcs::intentfiles;
use intentsvcs::organize::{Action, Step, plan};

/// A manifest declaring ST0001 and deliberately NOT ST0002.
///
/// The undeclared thread is what makes rows three and four reachable at all: with
/// every thread declared, `organize` can only ever hydrate and verify, and a
/// dehydration bug ships green.
const MANIFEST: &str = "\
STEELTHREAD:ST0001

# BEGIN INTENT
# END INTENT
";

fn canon() -> Canon {
  Canon {
    threads: vec![sample_thread("ST0001"), sample_thread("ST0002")],
    ..Default::default()
  }
}

fn step_for<'a>(steps: &'a [Step], path: &PathBuf) -> Option<&'a Step> {
  steps.iter().find(|s| &s.path == path)
}

#[test]
fn the_five_rows_of_d57_3() {
  let fx = Fixture::new();
  let project = fx.project();
  let canon = canon();
  let manifest = intentfiles::parse(MANIFEST).expect("manifest parses");

  // ROW 1 declared + absent      -> ST0001/acceptance.md  (not in on_disk)
  // ROW 2 declared + present     -> ST0001/info.md
  // ROW 3 undeclared + absent    -> ST0002/acceptance.md  (not in on_disk)
  // ROW 4 undeclared + present   -> ST0002/info.md
  // ROW 5 not renderer-produced  -> ST0001/diagram.png
  let declared_present = project.info_view("ST0001");
  let declared_absent = project.acceptance_view("ST0001");
  let undeclared_present = project.info_view("ST0002");
  let undeclared_absent = project.acceptance_view("ST0002");
  let human_file = project.st_dir().join("ST0001").join("diagram.png");
  let index_view = project.steel_threads_view();

  let on_disk = vec![
    declared_present.clone(),
    undeclared_present.clone(),
    human_file.clone(),
    index_view.clone(),
  ];

  let p = plan(
    &project,
    &canon,
    &manifest,
    &ctx(),
    &on_disk,
    "digest-under-test".to_string(),
  );

  assert_eq!(
    step_for(&p.steps, &declared_absent).map(|s| s.action),
    Some(Action::Hydrate),
    "row 1: declared and absent must HYDRATE"
  );
  assert_eq!(
    step_for(&p.steps, &declared_present).map(|s| s.action),
    Some(Action::Verify),
    "row 2: declared and present must VERIFY"
  );
  assert!(
    step_for(&p.steps, &undeclared_absent).is_none(),
    "row 3: undeclared and absent must produce NO step -- {:?} was named anyway",
    undeclared_absent
  );
  assert_eq!(
    step_for(&p.steps, &undeclared_present).map(|s| s.action),
    Some(Action::Dehydrate),
    "row 4: undeclared and present must DEHYDRATE"
  );
  assert_eq!(
    step_for(&p.steps, &human_file).map(|s| s.action),
    Some(Action::Unclaimed),
    "row 5: a path the renderer does not produce must be UNCLAIMED"
  );
}

#[test]
fn the_fifth_row_is_never_destructive() {
  // The second half of AC-04.1's fifth row, and it is a separate claim from
  // being reported. `is_destructive` is what the apply path will branch on, so
  // this asserts the property the removal actually consults rather than the
  // label a human reads.
  let fx = Fixture::new();
  let project = fx.project();
  let human_file = project.st_dir().join("ST0001").join("diagram.png");
  let p = plan(
    &project,
    &canon(),
    &intentfiles::parse(MANIFEST).expect("manifest parses"),
    &ctx(),
    std::slice::from_ref(&human_file),
    "d".to_string(),
  );
  let step = step_for(&p.steps, &human_file).expect("the unclaimed file is reported");
  assert_eq!(step.action, Action::Unclaimed);
  assert!(
    !step.action.is_destructive(),
    "UNCLAIMED must never be destructive: organize is not the thing that decides an unrecognised file is rubbish"
  );
  assert!(
    step.content.is_none(),
    "an unclaimed path carries no rendered bytes -- that is what makes it unclaimed"
  );
}

#[test]
fn an_index_view_is_exempt_rather_than_dehydrated() {
  // AC-04.6. `steel_threads.md` is renderer-produced, so the fifth row will never
  // claim it, and it names no artefact, so no manifest entry can ever declare it.
  // Without the explicit exempt set it is undeclared-and-present: row four, and
  // the first real run deletes the estate's index.
  let fx = Fixture::new();
  let project = fx.project();
  let index_view = project.steel_threads_view();
  let p = plan(
    &project,
    &canon(),
    &intentfiles::parse(MANIFEST).expect("manifest parses"),
    &ctx(),
    std::slice::from_ref(&index_view),
    "d".to_string(),
  );
  let step = step_for(&p.steps, &index_view).expect("the index view is accounted for");
  assert_eq!(
    step.action,
    Action::Exempt,
    "an index view must be EXEMPT, not dehydrated"
  );
  assert!(!step.action.is_destructive());
}

#[test]
fn a_pinned_declaration_realises_exactly_as_a_generated_one() {
  // AC-02.3's decision, seen from this side. The two regions differ in who writes
  // them, not in whether they declare -- so consulting the region when deciding
  // realisation would silently dehydrate every pinned thread, which is the whole
  // reason pins exist.
  let pinned_only = "\
STEELTHREAD:ST0002  # kept realised after close

# BEGIN INTENT
STEELTHREAD:ST0001
# END INTENT
";
  let fx = Fixture::new();
  let project = fx.project();
  let manifest = intentfiles::parse(pinned_only).expect("manifest parses");
  let p = plan(&project, &canon(), &manifest, &ctx(), &[], "d".to_string());
  for id in ["ST0001", "ST0002"] {
    let path = project.info_view(id);
    assert_eq!(
      step_for(&p.steps, &path).map(|s| s.action),
      Some(Action::Hydrate),
      "{id} is declared -- pinned or generated -- so it must hydrate"
    );
  }
}
