//! AT-08.3 / AC-08.3: **`PUT` accepts json only and rejects md -- except for
//! attachments, where text-in is correct.**
//!
//! **The exception is not an exception.** An attachment is AUTHORED on disk,
//! so the authority runs the other way: for everything else canon is the
//! source and a view is derived, while for an attachment the file is the
//! source and canon carries its bytes. Authorship decides direction, and
//! `Project::classify` is the single answer to what a file is -- which is what
//! makes the asymmetry implementable rather than aspirational.
//!
//! The harm the json-only rule prevents: writing markdown to an address would
//! promote a STALE RENDERING into canon. A view is a function of the model, so
//! accepting one back inverts the dependency and lets a regeneration lose
//! whatever the writer thought they were saving.

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::address::parse;

fn body_for(at: &str) -> String {
  format!(
    r#"{{"id":"{at}","kind":"test","file":"native/rust/crates/intentsvcs/tests/x.rs",
        "covers":["AC-03.1"],"status":"to-write"}}"#
  )
}

#[test]
fn md_is_refused_for_an_authored_entity() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();

  let address = parse("intent:///threads/ST0001/at/AT-03.9?format=md").expect("resolves");
  let err = facade
    .put(&address, &body_for("AT-03.9"))
    .expect_err("markdown must not be written to an entity address");
  assert!(
    err.to_string().contains("stale rendering"),
    "the refusal must name the harm rather than the syntax: {err}"
  );
}

/// **The discriminating half.** Without it the test above passes against a
/// surface that refuses every `PUT`.
#[test]
fn json_is_accepted_for_the_same_entity() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();

  let address = parse("intent:///threads/ST0001/at/AT-03.9?format=json").expect("resolves");
  facade
    .put(&address, &body_for("AT-03.9"))
    .expect("json is the mutation format");

  let landed = facade
    .canon()
    .threads
    .first()
    .expect("one thread")
    .tests
    .iter()
    .any(|t| t.id == "AT-03.9");
  assert!(landed, "the row must be in canon after the write");
}

/// An address with no `?format=` is not asking for markdown, so it is not
/// refused by the markdown rule. Included because an absent format and an
/// explicit one are different facts and the parser keeps them apart.
#[test]
fn an_absent_format_is_not_a_markdown_request() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();
  let address = parse("intent:///threads/ST0001/at/AT-03.9").expect("resolves");
  facade
    .put(&address, &body_for("AT-03.9"))
    .expect("a bare address defaults to the mutation format, not to a refusal");
}

/// The body must name the entity the address names. A `PUT` whose body carries
/// a different id would write to a row nobody addressed -- the silent-wrong-row
/// class, and the address is the only thing that could catch it.
#[test]
fn a_body_that_names_a_different_row_is_refused() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();
  let address = parse("intent:///threads/ST0001/at/AT-03.9").expect("resolves");
  let err = facade
    .put(&address, &body_for("AT-03.8"))
    .expect_err("the body and the address must agree");
  assert!(err.to_string().contains("AT-03.8"), "{err}");
}

/// A cross-project write is refused here rather than attempted: it resolves
/// against intentd's project registry, which this in-process surface does not
/// have. Refused with the reason, not silently treated as local -- writing to
/// the wrong project is unrecoverable.
#[test]
fn a_cross_project_address_is_not_written_locally() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();
  let address = parse("intent://elsewhere/threads/ST0001/at/AT-03.9").expect("resolves");
  let err = facade
    .put(&address, &body_for("AT-03.9"))
    .expect_err("a slug must not be written to the local project");
  assert!(err.to_string().contains("registry"), "{err}");
}
