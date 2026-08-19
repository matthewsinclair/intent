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

mod common;

use common::Fixture;
use intentsvcs::organize::{Action, OrganizeError, Step, gate};

const RENDERED: &str = "# ST0001\n\nRendered by the model.\n";

fn step_at(fx: &Fixture, rel: &str, content: Option<&str>) -> Step {
  Step {
    path: fx.path(rel),
    action: Action::Dehydrate,
    content: content.map(str::to_string),
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
