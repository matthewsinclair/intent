//! AT-04.4 / AC-04.4: every facade error is typed and renders a remedy with
//! its full cause chain -- no same-text-for-different-causes collapses.
//!
//! **The last clause is what makes this AC non-vacuous, and it is easy to miss.**
//! A test that merely asserts "an error has a remedy" passes on an
//! implementation where every remedy reads "check your input" -- which is
//! exactly the v2 behaviour this replaces. So the assertions here are
//! PAIRWISE: two distinct causes must render distinguishably, checked across
//! the whole variant set rather than sampled.
//!
//! **"The whole variant set" was a claim in this comment and nothing made it
//! true.** `provoked_errors` is hand-built, so the sentence above described an
//! intention rather than a mechanism. Measured against this file at
//! `c1e630cf`: **SIX reachable variants had no assertion here at all** --
//! `NotSatisfied`, `OffScope`, `WrongOffScopeState`, `IllegalTransition`,
//! `ReasonRequired` and `DescopeTargetMissing`. The claim is now carried by
//! `every_variant_is_provoked_or_declared_elsewhere`, and the exemptions are
//! declared rather than implied.

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::facade::FacadeError;
use intentsvcs::model::AtStatus;
use intentsvcs::remedy::Remedy;

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
  // The two export refusals a bad ARGUMENT can reach. Provoked here rather
  // than declared elsewhere because that is the point of this file: they are
  // the pair most at risk of collapsing into one message, and the remedy check
  // below is what stops "there is no such format" being said about a format
  // that exists and is declined.
  out.push((
    "unknown export format",
    facade
      .export(Some("xml"))
      .expect_err("there is no xml projection"),
  ));
  out.push((
    "refused export format",
    facade
      .export(Some("md"))
      .expect_err("md cannot be read back"),
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
  // **Six of the refusals below were reachable and asserted nowhere in this
  // file**, measured at `c1e630cf` -- so the module doc's "the whole variant
  // set rather than sampled" was already false before today's two variants
  // existed to widen it. Found by the coverage check below on its first run,
  // which is the argument for having written it.
  //
  // AC-03.2 is the fixture's only NON-TEST criterion, so it is the one that can
  // reach the kind-gated refusals at all, and it is walked through the states
  // deliberately: satisfied -> withdrawn -> back in scope -> unsatisfied. The
  // order IS the fixture here.
  out.push((
    "descope target does not exist",
    facade
      .ac_descope("ST0056", "AC-03.2", "ST9999", None, None)
      .expect_err("no such thread"),
  ));
  out.push((
    "descope target not named",
    facade
      .ac_descope("ST0056", "AC-03.2", "  ", None, None)
      .expect_err("blank target"),
  ));

  facade
    .ac_withdraw("ST0056", "AC-03.2", "the premise did not reproduce", None)
    .expect("withdraw the non-test criterion");
  out.push((
    "satisfy something out of scope",
    facade
      .ac_satisfy("ST0056", "AC-03.2", "x")
      .expect_err("withdrawn"),
  ));
  out.push((
    "rescope what was withdrawn",
    facade
      .ac_rescope("ST0056", "AC-03.2")
      .expect_err("rescope undoes a descope, not a withdrawal"),
  ));

  facade
    .ac_reinstate("ST0056", "AC-03.2")
    .expect("back in scope, unsatisfied");
  out.push((
    "evidence required",
    facade
      .ac_satisfy("ST0056", "AC-03.2", "  ")
      .expect_err("blank evidence"),
  ));
  out.push((
    "nothing to unsatisfy",
    facade
      .ac_unsatisfy("ST0056", "AC-03.2")
      .expect_err("not satisfied"),
  ));
  out.push((
    "reason required",
    facade
      .ac_withdraw("ST0056", "AC-03.2", "   ", None)
      .expect_err("blank reason"),
  ));
  // The from-state refusal, which is a different failure from every guard above
  // it: the value is fine and the thread is in the wrong state to receive it.
  out.push((
    "illegal transition",
    facade
      .st_resume("ST0056")
      .expect_err("resume is declared from `hold`, and the fixture thread is `wip`"),
  ));

  // **Its own fixture, because this one is provoked by CONFIGURATION rather
  // than by a call.** The facade reads `todo.window_hours` from the project it
  // was opened over, so the value has to be on disk before the facade exists --
  // rewriting the config of the fixture above would be read by nothing.
  let bad_window = Fixture::new();
  bad_window.write_file(
    "intent/.config/config.json",
    "{\n  \"intent_version\": \"3.0.0\",\n  \"project_name\": \"Fixture\",\n  \"author\": \"cc\",\n  \"intent_dir\": \"intent\",\n  \"languages\": [\"rust\"],\n  \"todo\": { \"window_hours\": 6 }\n}\n",
  );
  out.push((
    "unhonourable todo window",
    bad_window
      .facade()
      .todo_view_windowed()
      .expect_err("6 hours is not a whole number of days and `completed` is a date"),
  ));

  out
}

/// The variant a value is, as an EXHAUSTIVE match.
///
/// **The module doc says this file checks "the whole variant set rather than
/// sampled", and until now nothing made that true.** `provoked_errors` is a
/// hand-built list; a variant added to the facade and not to it was covered by
/// no assertion at all, and the doc claiming otherwise is the same shape as the
/// model comment that claimed empty evidence was unconstructible -- a written
/// guarantee standing in for a mechanism.
///
/// The match is what closes it: **a new variant does not compile until someone
/// adds an arm here**, and the arm is one line away from the test below telling
/// them to provoke it. The residual is stated rather than hidden: the arm and
/// [`ALL_VARIANTS`] are two lists, so a variant added to the match and not to
/// the list still slips the coverage check. That is a much smaller hole than
/// the one it replaces, and it is the smallest this gets without reflection or
/// a derive dependency.
fn variant(err: &FacadeError) -> &'static str {
  match err {
    FacadeError::NoSuchThread { .. } => "NoSuchThread",
    FacadeError::ThreadExists { .. } => "ThreadExists",
    FacadeError::NoSuchWorkPackage { .. } => "NoSuchWorkPackage",
    FacadeError::NoSuchCriterion { .. } => "NoSuchCriterion",
    FacadeError::NoSuchTest { .. } => "NoSuchTest",
    FacadeError::GateBlocked { .. } => "GateBlocked",
    FacadeError::ComputedSatisfaction { .. } => "ComputedSatisfaction",
    FacadeError::ScopeUnchanged { .. } => "ScopeUnchanged",
    FacadeError::NotOffScope { .. } => "NotOffScope",
    FacadeError::NotSatisfied { .. } => "NotSatisfied",
    FacadeError::OffScope { .. } => "OffScope",
    FacadeError::WrongOffScopeState { .. } => "WrongOffScopeState",
    FacadeError::BadQuery { .. } => "BadQuery",
    FacadeError::NoSuchFace { .. } => "NoSuchFace",
    FacadeError::IllegalTransition { .. } => "IllegalTransition",
    FacadeError::ReasonRequired { .. } => "ReasonRequired",
    FacadeError::EvidenceRequired { .. } => "EvidenceRequired",
    FacadeError::DescopeTargetMissing { .. } => "DescopeTargetMissing",
    FacadeError::DescopeTargetRequired { .. } => "DescopeTargetRequired",
    FacadeError::Unmigrated(_) => "Unmigrated",
    FacadeError::Write(_) => "Write",
    FacadeError::ViewsNotWritten { .. } => "ViewsNotWritten",
    FacadeError::Store(_) => "Store",
    FacadeError::Ingest(_) => "Ingest",
    FacadeError::NoSuchFormat { .. } => "NoSuchFormat",
    FacadeError::LossyFormat { .. } => "LossyFormat",
    FacadeError::ExportRoundTripFailed { .. } => "ExportRoundTripFailed",
    FacadeError::UnhonourableWindow(_) => "UnhonourableWindow",
  }
}

/// The variants a reader of this file should expect to see provoked.
///
/// Some are reachable only through a failing filesystem or a damaged store, and
/// those are declared here as deliberately-not-provoked rather than left to
/// look like oversights -- an exemption that is announced, never inferred
/// (ST0048's rule).
const ALL_VARIANTS: &[&str] = &[
  "NoSuchThread",
  "ThreadExists",
  "NoSuchWorkPackage",
  "NoSuchCriterion",
  "NoSuchTest",
  "GateBlocked",
  "ComputedSatisfaction",
  "ScopeUnchanged",
  "NotOffScope",
  "NotSatisfied",
  "OffScope",
  "WrongOffScopeState",
  "BadQuery",
  "NoSuchFace",
  "IllegalTransition",
  "ReasonRequired",
  "EvidenceRequired",
  "DescopeTargetMissing",
  "DescopeTargetRequired",
  "Unmigrated",
  "Write",
  "ViewsNotWritten",
  "Store",
  "Ingest",
  "NoSuchFormat",
  "LossyFormat",
  "ExportRoundTripFailed",
  "UnhonourableWindow",
];

/// Variants that need a broken world rather than a bad call, and are covered by
/// the tests that break that world instead.
const NOT_PROVOKED_HERE: &[&str] = &[
  "Write",           // an unwritable directory -- `write_set_rollback.rs`
  "ViewsNotWritten", // the same, after the DB has committed
  "Store",           // a damaged SQLite file
  "Ingest",          // schema-invalid canon -- `ingest_refusal.rs`
  "Unmigrated",      // an older store -- `unmigrated_project.rs`
  "ThreadExists",    // needs a colliding id, which `st new` allocates around
  "BadQuery",        // FTS5 syntax -- `facade_search.rs` territory
  "NoSuchFace",      // `intent schema <name>` with an unknown face
  // Needs a projection that LIES -- a format claiming to round-trip and
  // dropping data. Only `export::project_with` can be handed one, and
  // `export_round_trip.rs` does exactly that; a call through the facade cannot
  // reach it, because every format the roster carries is honest.
  "ExportRoundTripFailed",
];

#[test]
fn every_variant_is_provoked_or_declared_elsewhere() {
  let covered: std::collections::BTreeSet<&str> =
    provoked_errors().iter().map(|(_, e)| variant(e)).collect();

  let missing: Vec<&&str> = ALL_VARIANTS
    .iter()
    .filter(|v| !covered.contains(**v) && !NOT_PROVOKED_HERE.contains(*v))
    .collect();
  assert!(
    missing.is_empty(),
    "these variants are neither provoked here nor declared as covered elsewhere: {missing:?} -- \
     the module doc says this file checks the whole variant set, so a variant with no assertion \
     anywhere makes that claim false"
  );

  // The mirror: a name in the exemption list that IS provoked means the
  // exemption has gone stale and is now hiding a variant rather than
  // explaining one.
  let stale: Vec<&&str> = NOT_PROVOKED_HERE
    .iter()
    .filter(|v| covered.contains(**v))
    .collect();
  assert!(
    stale.is_empty(),
    "these are declared unreachable here and were provoked anyway: {stale:?}"
  );
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
  facade
    .st_hold("ST0056", "waiting on the fleet")
    .expect("a legal mutation from wip");

  let mode = fx.make_readonly("intent");
  let result = facade.st_cancel("ST0056", "superseded by the v3 line");
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
  // **This remedy has now been edited twice, for two different reasons, and
  // the pair is the point.** The first draft told the operator to run `intent
  // sync` -- disk -> db, which would have destroyed the change this error
  // calls safe -- and was fixed by warning them OFF it. That warning was then
  // the entire remedy for exactly as long as there was no db -> disk direction
  // to point AT, and AC-03.9 landed one the same day. So a remedy that only
  // said "do not" went from honest to under-serving without anybody touching
  // it: the same class as the first edit, arriving from the opposite side.
  //
  // The assertions therefore check the two surviving PROPERTIES rather than
  // the sentence, because the sentence has already moved twice.
  assert!(
    !rendered.contains("run `intent sync`"),
    "the remedy must never RECOMMEND the disk -> db direction -- that is the data-loss instruction this assertion exists to keep out, and it was once here: {rendered}"
  );
  assert!(
    rendered.contains("disk -> db") && rendered.contains("Do NOT reach"),
    "it still warns off that direction by name: {rendered}"
  );
  assert!(
    rendered.contains("intent st sync"),
    "and it names the repair that EXISTS, rather than telling the operator to wait for the next mutation as it did before AC-03.9: {rendered}"
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
