//! ST0092 WP-05: the checks that need WP-05's own types, the caller's origin,
//! the refusals' variants and the dry run's removals.
//!
//! **THESE CANNOT COMPILE ON THE CODE BEFORE WP-05**, which has no `Origin`, no
//! such variants and no `would_remove`, so their red is not a run of these arms.
//! It is read instead from the arms that do compile there: `wb_hv.rs` for the
//! pairs, the asks and the crossing, and intent-cli's `hv_seat_door.rs` for a
//! write as hv from a session that is not the mediator's, over MCP, which the
//! code before WP-05 takes from any caller.

use intentsvcs::facade::{Facade, FacadeError, Origin};
use intentsvcs::model::WbItemKind;

use crate::common::{Fixture, facade_ctx};
use crate::wb_hv::{Crossing, TEAM, crossing, declared, names, worker};

/// A facade over the fixture whose calls come from `origin`.
fn from(fx: &Fixture, origin: Origin) -> Facade {
  let mut ctx = facade_ctx();
  ctx.origin = origin;
  Facade::open(fx.project(), ctx).expect("open the fixture facade")
}

/// ST0092 AT-05.2, typed: the refusal is `WbPairClosed`, and its remedy is
/// exactly the ask that carries the same message to the mediator.
#[test]
fn at_05_2_the_closed_pair_refusal_carries_the_message_to_the_mediator() {
  let (fx, d) = declared();
  let w = worker(&d);
  let mut f = fx.facade_on_disk();
  let body = "a question for 'hv'";
  let err = f
    .wb_ask(&w, &d.human, body, None, false)
    .expect_err("a closed pair");
  assert!(
    matches!(&err, FacadeError::WbPairClosed { mediator, .. } if mediator == &d.mediator),
    "{err:?}"
  );
  assert!(
    intentsvcs::remedy::Remedy::remedy(&err).contains(&intentsvcs::model::ask_form(
      &d.mediator,
      body,
      &w
    )),
    "{err:?}"
  );
}

/// ST0092 AT-05.4: a write as the human seat is taken from the mediator's
/// session and from a shell with no session, and refused from another node's
/// session and from a door that carries no caller.
#[test]
fn at_05_4_only_the_mediators_session_or_a_terminal_writes_as_hv() {
  let (fx, d) = declared();
  let w = worker(&d);
  {
    let mut f = fx.facade_on_disk();
    f.wb_pickup(&d.mediator, Some("session-of-the-mediator"), None, false)
      .expect("bind the mediator's session");
    f.wb_pickup(&w, Some("session-of-a-worker"), None, false)
      .expect("bind a worker's session");
  }
  for (origin, unattributed_expected) in [
    (Origin::Session("session-of-a-worker".to_string()), false),
    (Origin::Unattributed, true),
  ] {
    let err = from(&fx, origin)
      .wb_add(&d.human, WbItemKind::Todo, "written as hv")
      .expect_err("a write as hv from the wrong door");
    assert!(
      matches!(
        &err,
        FacadeError::WbHumanSeatNotYours { mediator, unattributed, .. }
          if mediator == &d.mediator && *unattributed == unattributed_expected
      ),
      "{err:?}"
    );
  }
  for origin in [
    Origin::Session("session-of-the-mediator".to_string()),
    Origin::Terminal,
  ] {
    from(&fx, origin)
      .wb_add(&d.human, WbItemKind::Todo, "written as hv")
      .expect("the mediator's session and hv's terminal write as hv");
  }
  from(&fx, Origin::Unattributed)
    .wb_add(&w, WbItemKind::Todo, "a node's own write")
    .expect("a write as any other seat is not the check's");
}

/// ST0092 AT-05.6, typed: the refusal is `WbClosedPairHoldsLive`, and once the
/// message is handled the dry run names exactly the views the apply removes.
#[test]
fn at_05_6_the_dry_run_names_the_views_the_apply_removes() {
  let Crossing { fx, human, w } = crossing();
  let mut f = fx.facade_on_disk();
  let err = f
    .wb_init(&names(&[TEAM]), false, false)
    .expect_err("a live message on a pair the team closes");
  let FacadeError::WbClosedPairHoldsLive { live, .. } = &err else {
    panic!("{err:?}");
  };
  assert!(
    live.iter().any(|(s, r, _)| s == &w && r == &human),
    "{live:?}"
  );
  f.wb_clear(&human, &w).expect("hv clears it");
  let would_remove: std::collections::BTreeSet<String> = f
    .wb_init(&names(&[TEAM]), false, false)
    .expect("the dry run")
    .would_remove
    .into_iter()
    .collect();
  f.wb_init(&names(&[TEAM]), true, false).expect("the apply");
  let removed: std::collections::BTreeSet<String> =
    f.take_written_by_the_roster().removed.into_iter().collect();
  assert!(!removed.is_empty(), "the apply removed no view");
  assert_eq!(
    removed, would_remove,
    "the dry run named other removals than the apply made"
  );
}

/// ST0092 AT-05.7, typed: a board that declares no team lets any caller write
/// as hv, a door with no caller included.
#[test]
fn at_05_7_an_undeclared_board_takes_a_write_as_hv_from_any_caller() {
  let fx = Fixture::new();
  let mut f = fx.facade_on_disk();
  f.wb_register("hv", "hv seat", "human").expect("register");
  from(&fx, Origin::Unattributed)
    .wb_add("hv", WbItemKind::Todo, "any caller")
    .expect("an undeclared board does not ask who writes as hv");
}
