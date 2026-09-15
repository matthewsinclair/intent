//! AT-04.2 / AC-04.2: **the dehydration gate re-renders each view into memory,
//! compares to the bytes on disk, refuses on ANY difference, and names the path.**
//!
//! **"ANY DIFFERENCE" IS THE CLAIM, SO THE SMALLEST ONE IS THE TEST.** A gate
//! that refused on a wholesale rewrite and waved through a single trailing
//! newline would pass every plausible fixture and lose exactly the edits people
//! actually make -- a word changed, a line added at the end. One byte is
//! therefore an arm here, not a curiosity.
//!
//! **AND THE UNPROVABLE CASE IS REFUSED, NOT PASSED.** A step with no rendered
//! bytes cannot support the claim "the store can reproduce this file". The gate's
//! job is to prove removal safe, and an absent proof is not a weak proof -- it is
//! no proof. Passing there would make the gate strictest exactly when it knows
//! most and silent exactly when it knows nothing.

use crate::common::Fixture;
use intentsvcs::organize::{Action, OrganizeError, Step, gate};

const RENDERED: &str = "# ST0001\n\nRendered by the model.\n";

fn step_at(fx: &Fixture, rel: &str, content: Option<&str>) -> Step {
  Step {
    path: fx.path(rel),
    action: Action::Dehydrate,
    content: content.map(|text| text.as_bytes().to_vec()),
  }
}

#[test]
fn bytes_matching_the_render_are_cleared_for_removal() {
  let fx = Fixture::new();
  fx.write_file("gated.md", RENDERED);
  let step = step_at(&fx, "gated.md", Some(RENDERED));
  assert!(
    gate(&step).is_ok(),
    "a file the store reproduces exactly carries nothing that removing it would destroy"
  );
}

#[test]
fn a_hand_edited_view_is_refused_and_the_path_is_named() {
  let fx = Fixture::new();
  fx.write_file("gated.md", "# ST0001\n\nSomebody wrote this by hand.\n");
  let step = step_at(&fx, "gated.md", Some(RENDERED));
  match gate(&step) {
    Err(OrganizeError::HandEdited { path, bytes }) => {
      assert_eq!(path, fx.path("gated.md"), "the refusal must name the path");
      assert_eq!(bytes, "# ST0001\n\nSomebody wrote this by hand.\n".len());
    }
    other => panic!("a hand-edited view must be refused, got {other:?}"),
  }
}

#[test]
fn one_byte_of_difference_is_enough() {
  // AC-04.2 says ANY difference. The realistic hand edit is a newline or a word,
  // not a rewrite, so a gate calibrated to obvious differences protects nothing
  // that needs protecting.
  let fx = Fixture::new();
  fx.write_file("gated.md", &format!("{RENDERED}\n"));
  let step = step_at(&fx, "gated.md", Some(RENDERED));
  assert!(
    matches!(gate(&step), Err(OrganizeError::HandEdited { .. })),
    "a single trailing newline is a difference, and the file holds it while the store does not"
  );
}

#[test]
fn a_step_with_no_rendered_bytes_is_refused_rather_than_passed() {
  let fx = Fixture::new();
  fx.write_file("gated.md", RENDERED);
  let step = step_at(&fx, "gated.md", None);
  assert!(
    matches!(gate(&step), Err(OrganizeError::HandEdited { .. })),
    "with nothing to compare against, `the store carries this` is UNPROVEN -- and unproven is not permission"
  );
}

#[test]
fn an_unreadable_file_is_an_error_not_a_clearance() {
  // The failure mode that would be invisible: if a read error resolved to `Ok`,
  // the gate would clear for removal exactly the files it could not inspect, and
  // every subsequent report would say the removal was proven safe.
  let fx = Fixture::new();
  let step = step_at(&fx, "never-written.md", Some(RENDERED));
  match gate(&step) {
    Err(OrganizeError::Io { path, .. }) => {
      assert_eq!(path, fx.path("never-written.md"))
    }
    other => panic!("an unreadable file must surface as an error, got {other:?}"),
  }
}

#[test]
fn the_refusal_tells_the_operator_where_the_edit_belongs() {
  // A refusal that names a path and not a remedy sends someone to re-run the
  // same command harder. The message has to say that canon is where a wanted
  // edit goes, because the file they edited is the one the model overwrites.
  let fx = Fixture::new();
  fx.write_file("gated.md", "hand written\n");
  let err = gate(&step_at(&fx, "gated.md", Some(RENDERED))).unwrap_err();
  let text = err.to_string();
  assert!(
    text.contains("canon"),
    "the refusal must say where a wanted edit belongs: {text}"
  );
  assert!(
    text.contains("destroy"),
    "the refusal must say what removal would cost, not merely that it declined: {text}"
  );
}

/// Issue 0343: hv drove `organize --apply` and found a tree half-dehydrated,
/// views kept and attachments gone, because each removal was gated alone. A
/// thread's files are one set, so one refused file withholds every removal of
/// its thread.
#[test]
fn one_refused_file_withholds_every_removal_of_its_thread() {
  let fx = Fixture::new();
  let mut thread = crate::common::sample_thread("ST0001");
  thread.attachments.push(intentsvcs::model::Attachment::new(
    "design.md",
    "# Design\n",
  ));
  fx.write_thread(&thread);
  fx.write_file("intent/st/ST0001/design.md", "# Design\n");
  let project = fx.project();
  let canon = intentsvcs::ingest::read(&project).expect("canon reads");
  intentsvcs::views::write_all(&project, &canon, &crate::common::ctx()).expect("write views");
  let cover = fx.path("intent/st/ST0001/info.md");
  let edited = format!(
    "{}\n<!-- a hand edit -->\n",
    fx.read("intent/st/ST0001/info.md")
  );
  std::fs::write(&cover, edited).expect("hand-edit the cover");

  // A manifest declaring none, so every file of ST0001 is planned for removal.
  let realised =
    intentsvcs::intentfiles::realised_for_action("").expect("an empty manifest parses");
  let (tree, digest) = intentsvcs::organize::observe(&project, &[]).expect("observes the tree");
  let plan = intentsvcs::organize::plan(
    &project,
    &canon,
    &realised,
    &crate::common::ctx(),
    &tree,
    digest.clone(),
  );
  let report = plan
    .run(intentsvcs::organize::Mode::Apply, &|| digest.clone())
    .expect("the run completes");

  assert!(
    cover.exists(),
    "precondition: the hand-edited cover is refused"
  );
  assert!(
    fx.path("intent/st/ST0001/design.md").exists(),
    "the attachment stays with the thread's refused cover: removed {:?}",
    report.dehydrated
  );
  assert!(
    report
      .dehydrated
      .iter()
      .all(|p| !p.starts_with(fx.path("intent/st/ST0001"))),
    "no file of the thread is removed: {:?}",
    report.dehydrated
  );
  assert!(
    report
      .refused
      .iter()
      .any(|r| r.to_string().contains("ST0001") && r.to_string().contains("withheld")),
    "and the run says the rest of the thread was withheld: {:?}",
    report.refused
  );
}
