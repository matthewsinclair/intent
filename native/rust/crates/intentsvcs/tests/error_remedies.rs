//! AT-04.4 / AC-04.4: every facade error is typed and renders a remedy with
//! its full cause chain -- no same-text-for-different-causes collapses.
//!
//! **The last clause is what makes this AC non-vacuous, and it is easy to miss.**
//! A test that merely asserts "an error has a remedy" passes on an
//! implementation where every remedy reads "check your input" -- which is
//! exactly the v2 behaviour this replaces. So the assertions here are
//! PAIRWISE: two distinct causes must render distinguishably, checked across
//! the whole variant set rather than sampled.

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::facade::FacadeError;
use intentsvcs::model::AtStatus;

/// Provoke each error through the real facade, so the set under test is what
/// operators can actually reach -- not a hand-built list that could drift from
/// the code that raises them.
fn provoked_errors() -> Vec<(&'static str, FacadeError)> {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();
  // Seeded rather than pushed-into-empty: the ORDER matters, because each
  // error after the first is provoked against a facade the previous calls have
  // already mutated.
  let mut out: Vec<(&'static str, FacadeError)> = vec![(
    "unknown thread",
    facade.st_show("ST9999").expect_err("no such thread"),
  )];
  out.push((
    "unknown work package",
    facade.wp_start("ST0056", 99).expect_err("no such wp"),
  ));
  out.push((
    "unknown criterion",
    facade
      .ac_satisfy("ST0056", "AC-99.9", "x")
      .expect_err("no such ac"),
  ));
  out.push((
    "unknown test",
    facade
      .at_set("ST0056", "AT-99.9", AtStatus::Green)
      .expect_err("no such at"),
  ));
  out.push((
    "computed satisfaction",
    facade
      .ac_satisfy("ST0056", "AC-03.1", "x")
      .expect_err("test-backed"),
  ));
  out.push((
    "reinstate in-scope",
    facade
      .ac_reinstate("ST0056", "AC-03.1")
      .expect_err("in scope"),
  ));

  facade
    .ac_withdraw("ST0056", "AC-03.1", "r", None)
    .expect("withdraw");
  out.push((
    "scope unchanged",
    facade
      .ac_withdraw("ST0056", "AC-03.1", "r", None)
      .expect_err("already withdrawn"),
  ));

  // AC-03.1 is now withdrawn, so the thread cannot close.
  facade.at_set("ST0056", "AT-03.1", AtStatus::Red).unwrap();
  facade.at_set("ST0056", "AT-03.7", AtStatus::Red).unwrap();
  facade
    .ac_reinstate("ST0056", "AC-03.1")
    .expect("back in scope so the gate has something to block on");
  out.push((
    "gate blocked",
    facade.st_done("ST0056").expect_err("gate blocks"),
  ));

  out
}

#[test]
fn every_error_renders_a_message_and_a_remedy() {
  for (label, err) in provoked_errors() {
    let rendered = err.render();
    assert!(
      rendered.starts_with("error: "),
      "{label}: the rendering leads with the lowercase voice (0023): {rendered}"
    );
    assert!(
      rendered.contains("\n  remedy: "),
      "{label}: every error tells the operator what to DO: {rendered}"
    );
    assert!(
      !err.remedy().is_empty(),
      "{label}: an empty remedy is a remedy-shaped hole"
    );
  }
}

/// The anti-collapse assertion, checked pairwise over the whole set.
#[test]
fn no_two_distinct_causes_render_the_same_text() {
  let errors = provoked_errors();
  for (i, (label_a, a)) in errors.iter().enumerate() {
    for (label_b, b) in errors.iter().skip(i + 1) {
      assert_ne!(
        a.render(),
        b.render(),
        "'{label_a}' and '{label_b}' render identically -- an operator hitting either one cannot tell which they hit"
      );
      assert_ne!(
        a.to_string(),
        b.to_string(),
        "'{label_a}' and '{label_b}' share a message"
      );
      assert_ne!(
        a.remedy(),
        b.remedy(),
        "'{label_a}' and '{label_b}' share a remedy -- a remedy that fits two causes tells the operator to guess"
      );
    }
  }
}

/// Each message names the specific artefact, not just its kind. "no such
/// thread" is a category; "no steel thread ST9999" is an answer.
#[test]
fn every_message_names_the_artefact_it_is_about() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();

  let cases: Vec<(FacadeError, &str)> = vec![
    (facade.st_show("ST9999").unwrap_err(), "ST9999"),
    (facade.wp_start("ST0056", 42).unwrap_err(), "WP-42"),
    (
      facade.ac_satisfy("ST0056", "AC-77.7", "x").unwrap_err(),
      "AC-77.7",
    ),
    (
      facade
        .at_set("ST0056", "AT-77.7", AtStatus::Green)
        .unwrap_err(),
      "AT-77.7",
    ),
  ];
  for (err, needle) in cases {
    assert!(
      err.to_string().contains(needle),
      "the message names {needle}, got: {err}"
    );
  }
}

/// A wrapped error keeps its cause chain. Collapsing to the outermost sentence
/// is what made two different problems print the same line in v2.
#[cfg(unix)]
#[test]
fn a_wrapped_failure_renders_its_full_cause_chain() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();
  facade.st_start("ST0056").expect("materialise views");

  let mode = fx.make_readonly("intent");
  let result = facade.st_cancel("ST0056");
  fx.restore_mode("intent", mode);

  let err = result.expect_err("the write must fail");
  let rendered = err.render();
  assert!(
    rendered.contains("caused by:"),
    "the underlying I/O failure is reported, not swallowed by the outer message: {rendered}"
  );
  assert!(
    rendered.contains("todo.md"),
    "the chain names the file that actually failed: {rendered}"
  );
  // D01 REVERSED (hv, 2026-08-15): this used to assert "nothing was changed",
  // and that is now FALSE. The DB is the SSOT and it is written first, so by
  // the time a file write fails the mutation IS recorded -- what failed is the
  // projection of it onto disk.
  //
  // The new assertion is the stronger one, because the hazard inverted with
  // the model. Under the old order the operator's risk was believing a change
  // had landed when it had not; under the new one it is RETRYING a change that
  // already landed. So the text must lead with what succeeded and say plainly
  // not to repeat it -- a remedy that merely described the I/O error would be
  // accurate and would still get the estate mutated twice.
  assert!(
    rendered.contains("the change is recorded"),
    "the message leads with what SUCCEEDED, so the operator does not read a projection failure as a failed mutation: {rendered}"
  );
  assert!(
    rendered.contains("do NOT retry"),
    "the remedy names the actual hazard under D01-as-reversed, which is a second application of a change that already landed: {rendered}"
  );
  // It names `intent sync` only to REFUSE it. Under D01 as reversed, sync is
  // disk -> db, so running it against stale files would overwrite the store
  // with them and destroy the change this very error says is safe. The first
  // draft of this remedy told the operator to run it.
  assert!(
    rendered.contains("do NOT run `intent sync`"),
    "the remedy warns off the disk-to-db command rather than recommending it: {rendered}"
  );
}

/// The gate's refusal carries the gate's own verdict line, so the operator
/// sees WHICH criteria blocked rather than being told to go and look.
#[test]
fn the_gate_refusal_carries_the_verdict_line() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();
  facade.at_set("ST0056", "AT-03.1", AtStatus::Red).unwrap();
  facade.at_set("ST0056", "AT-03.7", AtStatus::Red).unwrap();

  let err = facade.st_done("ST0056").expect_err("blocked");
  let rendered = err.render();
  assert!(rendered.contains("gate: ST0056 BLOCKED"), "{rendered}");
  assert!(rendered.contains("AC-03.1"), "{rendered}");
}
