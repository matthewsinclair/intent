//! **THE CENSUS hv's RULING REQUIRES, AND WHAT IT ACTUALLY COVERS.**
//!
//! `model::fiat_marker`'s doc states the acceptance condition of hv's 2026-08-28
//! ruling: the fiat record sits BESIDE the status rather than inside it, which
//! leaves the ratified lifecycle machine untouched and buys back the property
//! that **a row cannot render as ordinarily closed** -- and it says that is
//! "enforced by a census over the render sites rather than by everyone
//! remembering".
//!
//! **That census did not exist until this file.** The doc asserted an
//! enforcement mechanism as though it were in place, which is the same class as
//! the three stale counts in `AcState::name`'s doc: prose is the one part of
//! these crates the compiler does not read.
//!
//! # What this file checks, and what it deliberately does NOT
//!
//! The property is NOT "every surface calls `fiat_marker`" -- that is a
//! mechanism, and testing a mechanism instead of a property is how a guard comes
//! to certify the status quo. The property is **a surface reporting on a
//! fiat-closed criterion must make the fiat close visible**, and the surfaces
//! legitimately differ in how: the generated view composes the marker, while the
//! close gate reports a COUNT and has no marker to compose.
//!
//! Measured, over the four surfaces `fiat_marker`'s doc names:
//!
//! | surface         | fiat close visible | how                                  |
//! | --------------- | ------------------ | ------------------------------------ |
//! | generated views | yes                | composes `fiat_marker`               |
//! | close gate      | yes                | its own tally, `N fiat-closed`       |
//! | `doctor`        | n/a                | reports inconsistencies, not states  |
//! | **`ac list`**   | **yes**            | its own `fiat-closed: <why>` form     |
//!
//! **`ac list` WAS THE UNCOVERED FOURTH AND IS NOW COVERED -- 0137 IS CLOSED
//! (dc, 2026-08-29).** It rendered a fiat-closed criterion as `satisfied: no`,
//! byte-identical to an ordinary open row, because `Fiat` fell into a wildcard
//! arm sitting beside EXPLICIT arms for `Descoped` and `Withdrawn`.
//!
//! **It was demoted rather than escalated on a census showing zero fiat rows
//! store-wide, watched with that census as its trigger condition. The trigger
//! fired the moment `fc` could write one** -- a defect whose only defence is
//! that nothing can reach the state stops being defended by the change that
//! reaches it, and the change that reaches it is the one that must therefore
//! carry the fix.
//!
//! **THE ARM ASSERTS THE PROPERTY, NOT THE BYTES, WHICH IS WHY IT CAN EXIST
//! NOW.** The earlier version of this note declined to write one because the
//! two candidate fixes differed in SPELLING, and an arm pinning today's output
//! would be a change-detector blocking the fix rather than a guard wanting it.
//! That objection is answered by testing what the ruling actually requires --
//! the row is distinguishable from an ordinary open one and carries its reason
//! -- so any spelling ic ratifies passes and only a regression to
//! indistinguishability fails.
//!
//! # A finding worth carrying: "nowhere else" is already false, benignly
//!
//! The doc claims the marker is composed in `fiat_marker` and nowhere else.
//! There are three independent spellings in `src`: `fiat_marker`'s
//! `FIAT-CLOSED`, `contract.rs`'s `N fiat-closed` tally, and
//! `preconditions.rs`'s refusal reason. Two are not markers and are fine. The
//! claim is still wrong as written, and a claim that is wrong in a harmless way
//! is how the harmful case (`ac list`) stayed invisible: nobody re-reads a
//! sentence they have already accepted.

mod common;

use common::{Fixture, ctx, sample_thread};
use intentsvcs::contract::{Scope, Verdict};
use intentsvcs::ingest::Canon;
use intentsvcs::model::{AcKind, AcState, Criterion, FiatRecord, Invoker, Thread};
use intentsvcs::views;

const BECAUSE: &str = "the panel-survival half is unobservable by unit test";

fn fiat(inherited_from: Option<&str>) -> AcState {
  AcState::Fiat(FiatRecord {
    because: BECAUSE.to_string(),
    by: "hv".to_string(),
    at: "2026-08-28T18:30:00.000Z".to_string(),
    invoker: Invoker {
      tty: true,
      env: "darwin/arm64".to_string(),
    },
    inherited_from: inherited_from.map(str::to_string),
    inherited_event: None,
  })
}

/// `sample_thread` plus one criterion in the state under test. Taking the state
/// as an argument is what lets every arm below run its own control through the
/// identical path -- a control built by a different route proves the two routes
/// differ, not that the property holds.
fn thread_with(id: &str, ac: &str, state: AcState) -> Thread {
  let mut t = sample_thread(id);
  t.criteria.push(Criterion {
    id: ac.to_string(),
    text: "a requirement closed on authority with the work unmet".to_string(),
    kind: AcKind::NonTest,
    state,
  });
  t
}

fn rendered(thread: Thread) -> String {
  let fx = Fixture::new();
  let canon = Canon {
    threads: vec![thread],
    issues: Vec::new(),
    sections: Vec::new(),
  };
  views::render_all(&fx.project(), &canon, &ctx())
    .iter()
    .map(|v| v.content.clone())
    .collect::<Vec<_>>()
    .join("\n")
}

fn tally(thread: Thread) -> String {
  let fx = Fixture::new();
  fx.write_thread(&thread);
  let verdict = fx.facade().gate("ST0001", Scope::Thread).expect("gate");
  match verdict {
    Verdict::Pass { detail } | Verdict::Exempt { detail } => format!("{detail:?}"),
    Verdict::Blocked { detail, .. } => format!("{detail:?}"),
  }
}

// ---------------------------------------------------------------------------
// Generated views -- the surface that composes the marker
// ---------------------------------------------------------------------------

#[test]
fn a_fiat_close_is_visible_in_the_generated_view() {
  let out = rendered(thread_with("ST0001", "AC-09.1", fiat(None)));
  assert!(
    out.contains("FIAT-CLOSED"),
    "the generated view does not mark the fiat close, so a reader sees a row closed with its \
     requirement unmet and nothing saying a human decided that: {out}"
  );
  assert!(
    out.contains(BECAUSE),
    "the marker is present but the REASON is not, and a fiat close whose reason is not published \
     is an unaccountable one: {out}"
  );
  assert!(
    out.contains("by hv"),
    "the close does not name who took it: {out}"
  );
}

#[test]
fn the_view_says_nothing_about_a_fiat_close_when_there_is_none() {
  // THE CONTROL. Without it, an arm asserting `contains("FIAT-CLOSED")` would
  // pass just as well against a renderer that stamped the marker on every row.
  let out = rendered(thread_with(
    "ST0001",
    "AC-09.1",
    AcState::Unsatisfied { note: None },
  ));
  assert!(
    !out.contains("FIAT-CLOSED"),
    "a criterion that was NOT closed by fiat rendered the marker anyway, so the assertion above \
     proves nothing about fiat rows: {out}"
  );
}

#[test]
fn a_cascaded_close_does_not_render_as_one_that_was_individually_judged() {
  // The distinction AC-00.3 rests on, held at the SURFACE and not only in the
  // model: `fiat_state_serde` proves the two are distinct in the extract, and
  // this proves the distinction survives rendering. A projection that dropped
  // `inherited_from` would satisfy the serde arm and still tell every reader
  // that hv judged a row nobody judged.
  let direct = rendered(thread_with("ST0001", "AC-09.1", fiat(None)));
  let cascaded = rendered(thread_with("ST0001", "AC-09.1", fiat(Some("ST0066"))));
  assert!(
    cascaded.contains("by cascade from ST0066"),
    "the cascade marker did not reach the view, so a row the cascade reached is indistinguishable \
     from one hv judged individually: {cascaded}"
  );
  assert!(
    !direct.contains("by cascade"),
    "a directly-judged close claims to be a cascade: {direct}"
  );
}

// ---------------------------------------------------------------------------
// The close gate -- a surface with no marker to compose
// ---------------------------------------------------------------------------

#[test]
fn the_close_gate_counts_a_fiat_close_apart_from_a_satisfaction() {
  // `Resolved::Fiat`'s doc promises it is "reported separately from `Satisfied`
  // everywhere it is counted". Checked rather than trusted: folding fiat rows
  // into the satisfied count is the single change that would make the gate say
  // a thread met requirements it was excused from.
  let out = tally(thread_with("ST0001", "AC-09.1", fiat(None)));
  assert!(
    out.contains("fiat-closed"),
    "the gate's own summary does not distinguish a fiat close, so its count reads as work that \
     was done: {out}"
  );
}

#[test]
fn the_gate_says_nothing_about_a_fiat_close_when_there_is_none() {
  let out = tally(thread_with(
    "ST0001",
    "AC-09.1",
    AcState::Unsatisfied { note: None },
  ));
  assert!(
    !out.contains("fiat-closed"),
    "the gate reported a fiat close on a thread that has none, so the arm above proves nothing: \
     {out}"
  );
}

/// The fourth surface, and the property rather than the spelling.
///
/// **THE CONTROL IS THE OTHER HALF OF THE ASSERTION, NOT DECORATION.** "The
/// fiat row renders differently" is satisfiable by a renderer that mangles
/// every row, so an ordinary open criterion is rendered in the same call and
/// required to be untouched. Without it this arm would pass against a change
/// that broke `ac list` completely.
#[test]
fn ac_list_does_not_render_a_fiat_close_as_an_ordinary_open_row() {
  let fx = Fixture::new();
  let mut facade = fx.facade_on_disk();
  fx.write_thread(&sample_thread("ST0056"));
  facade
    .sync_from_disk(&intentsvcs::sync::Scope::All)
    .expect("ingest");

  // AC-03.1 is test-backed and `Computed` -- one of the two states `ac.fc` is
  // declared from. AC-03.2 is left alone as the control.
  let open_before = facade
    .ac_list("ST0056")
    .expect("list")
    .into_iter()
    .find(|r| r.id == "AC-03.1")
    .expect("the criterion")
    .state;

  facade
    .ac_fc("ST0056", "AC-03.1", BECAUSE, "hv")
    .expect("fc");

  let rows = facade.ac_list("ST0056").expect("list");
  let closed = rows
    .iter()
    .find(|r| r.id == "AC-03.1")
    .expect("the closed criterion");

  assert_ne!(
    closed.state, open_before,
    "**ISSUE 0137**: a fiat-closed criterion rendered exactly as it did when it was open, so \
     nothing about this line tells a reader a human closed it against the evidence"
  );
  assert!(
    closed.state.contains(BECAUSE),
    "the reason is the whole record -- a line saying a close happened without saying why sends \
     the reader to another verb to learn the one thing that matters. Got: {}",
    closed.state
  );

  // THE CONTROL.
  let untouched = rows
    .iter()
    .find(|r| r.id == "AC-03.2")
    .expect("the control criterion");
  assert!(
    untouched.state.starts_with("satisfied:"),
    "a criterion nobody closed must render exactly as it always did, or the arm above passes \
     against a renderer that mangles every row. Got: {}",
    untouched.state
  );
}
