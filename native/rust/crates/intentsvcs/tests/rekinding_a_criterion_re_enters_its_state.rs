//! Issue 0346: **`set <ac> kind` leaves a state the new kind can hold.** It
//! wrote the kind and kept the state, so a `computed` criterion flipped to
//! `non-test` became the pair `AcState::permitted_for` forbids: doctor refused
//! the canon and `ac satisfy` refused the row.

use crate::common::{Fixture, sample_thread};
use intentsvcs::address::parse;
use intentsvcs::model::{AcKind, AcState, Criterion};
use serde_json::json;

fn criterion(facade: &intentsvcs::facade::Facade, ac: &str) -> Criterion {
  facade
    .st_show("ST0001")
    .expect("thread")
    .criteria
    .iter()
    .find(|c| c.id == ac)
    .cloned()
    .unwrap_or_else(|| panic!("the fixture has no {ac}"))
}

fn set_kind(facade: &mut intentsvcs::facade::Facade, ac: &str, kind: &str) -> Result<(), String> {
  let address = parse(&format!("intent:///threads/ST0001/ac/{ac}")).expect("resolves");
  facade
    .set(&address, "kind", json!(kind))
    .map(|_| ())
    .map_err(|e| format!("{e:?}"))
}

/// A computed criterion flipped to non-test lands unsatisfied, and flipped
/// back lands computed: each re-enters at the state its kind is created in.
#[test]
fn a_flip_re_enters_the_state_the_new_kind_is_created_in() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();
  let before = criterion(&facade, "AC-03.1");
  assert_eq!(
    (before.kind, before.state.clone()),
    (AcKind::Test, AcState::Computed {}),
    "the fixture moved"
  );

  set_kind(&mut facade, "AC-03.1", "non-test").expect("test to non-test is settable");
  let row = criterion(&facade, "AC-03.1");
  assert_eq!(row.kind, AcKind::NonTest);
  assert_eq!(row.state, AcState::Unsatisfied { note: None });
  assert!(
    row.state.permitted_for(row.kind),
    "the flip left an illegal pair"
  );

  set_kind(&mut facade, "AC-03.1", "test").expect("non-test to test is settable");
  let row = criterion(&facade, "AC-03.1");
  assert_eq!((row.kind, row.state), (AcKind::Test, AcState::Computed {}));
}

/// A satisfied criterion has a recorded satisfaction a test-backed one cannot
/// keep, so the flip is refused with the verb that reopens it, and nothing moves.
#[test]
fn a_satisfied_criterion_is_refused_with_the_verb_that_reopens_it() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();
  let before = criterion(&facade, "AC-03.2");
  assert!(
    matches!(before.state, AcState::Satisfied { .. }),
    "the fixture moved"
  );

  let said =
    set_kind(&mut facade, "AC-03.2", "test").expect_err("a satisfied row must not flip to test");
  assert!(
    said.contains("intent ac unsatisfy"),
    "the refusal must name the verb that reopens it: {said}"
  );
  assert_eq!(
    criterion(&facade, "AC-03.2"),
    before,
    "a refused flip moved the row"
  );
}

/// A decision about the requirement holds for both kinds, so it stays put.
#[test]
fn a_descoped_criterion_keeps_its_state_across_a_flip() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();
  let before = criterion(&facade, "AC-03.9");
  assert!(
    matches!(before.state, AcState::Descoped { .. }),
    "the fixture moved"
  );

  set_kind(&mut facade, "AC-03.9", "non-test").expect("a descoped row flips");
  let row = criterion(&facade, "AC-03.9");
  assert_eq!((row.kind, row.state), (AcKind::NonTest, before.state));
}

/// **A NOTE ON A COMPUTED ROW IS REFUSED WITH THE ROUTE THAT REACHES ONE.** The
/// refusal named `intent ac unsatisfy`, which refuses a test-backed row too, so
/// following it met a second refusal. It names the re-kind now, and this arm
/// follows it to the note.
#[test]
fn a_note_on_a_computed_row_names_the_re_kind_and_the_re_kind_reaches_the_note() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();
  assert_eq!(
    criterion(&facade, "AC-03.1").state,
    AcState::Computed {},
    "the fixture moved"
  );

  let refusal = facade
    .ac_edit(
      "ST0001",
      "AC-03.1",
      None,
      Some("why it is open".to_string()),
    )
    .expect_err("a computed row carries no note");
  let text = intentsvcs::remedy::Remedy::render(&refusal);
  assert!(
    text.contains("kind non-test") && !text.contains("ac unsatisfy"),
    "the refusal does not name the route that works: {text}"
  );

  set_kind(&mut facade, "AC-03.1", "non-test").expect("the named route is settable");
  facade
    .ac_edit(
      "ST0001",
      "AC-03.1",
      None,
      Some("why it is open".to_string()),
    )
    .expect("after the re-kind the note is accepted");
}
