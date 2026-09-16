//! Issue 0402: a thread's `objective` and `context` held two forms of the same
//! text depending on the door that wrote it. `intent set ... --from <file>`
//! kept the file's trailing newline; the `info.md` read-back trimmed it. So a
//! store filled from the realised tree wrote canon one byte shorter per field
//! than the live store's, and a lander read that as movement.
//!
//! **ONE FORM: NEWLINES AT EITHER END ARE NOT AUTHORED TEXT, AND NOTHING ELSE
//! IS TOUCHED.** It is the fixed point of rendering a section and reading it
//! back, so indentation on the first line and trailing spaces survive -- the
//! read-back used to `trim()` them away, which was a second normalisation of
//! its own.

use crate::common::{Fixture, ctx, sample_thread};
use intentsvcs::address::parse;
use intentsvcs::views;
use serde_json::json;

#[test]
fn the_setter_stores_the_one_form() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut f = fx.facade();
  let thread = parse("intent:///threads/ST0001").expect("resolves");
  f.set(&thread, "objective", json!("Ship it.\n"))
    .expect("set the objective from a file");
  f.set(&thread, "context", json!("\nBecause.\n\n"))
    .expect("set the context");
  let held = f.st_show("ST0001").expect("there");
  assert_eq!(held.objective, "Ship it.");
  assert_eq!(held.context, "Because.");
}

#[test]
fn canon_carrying_a_trailing_newline_reads_as_the_one_form() {
  let fx = Fixture::new();
  let mut thread = sample_thread("ST0001");
  thread.objective = "Ship it.\n".to_string();
  thread.context = "Because.\n".to_string();
  fx.write_thread(&thread);
  let f = fx.facade();
  let held = f.st_show("ST0001").expect("there");
  assert_eq!(held.objective, "Ship it.");
  assert_eq!(held.context, "Because.");
}

#[test]
fn a_section_read_back_unedited_is_the_value_it_was_rendered_from() {
  let mut thread = sample_thread("ST0001");
  thread.objective = "    indented first line\nand a hard break  ".to_string();
  thread.context = "Because.".to_string();
  let rendered = views::info(&thread, &ctx());
  let back = views::info_read_back(&thread, &ctx(), &rendered).expect("an unedited cover");
  assert_eq!(
    back.objective, thread.objective,
    "the read-back changed text nobody edited"
  );
}
