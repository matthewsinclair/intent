//! ST0092 WP-04 bank (i): `wb retire` takes a node off the roster as a state,
//! never a deletion; `wb register` reinstates it; and `wb init <base>
//! --change` retires the old base's seats the new one does not declare.
//!
//! Every arm applies a shipped profile in a fresh fixture project and reads
//! its seats, mediator and human seat from the profile files, so no seat is
//! written down here as a literal.

use intentsvcs::facade::{Facade, FacadeError};
use intentsvcs::teams::{Teams, Verdict};

use crate::common::{Fixture, sample_thread};

const TEAM: &str = "team";
const SOLO: &str = "solo";
const REASON: &str = "the hub's work moved to its successor";

fn names(list: &[&str]) -> Vec<String> {
  list.iter().map(|s| (*s).to_string()).collect()
}

fn resolved(profile: &str) -> intentsvcs::teams::Resolved {
  Teams::new(&testkit::repo_root())
    .resolve(&names(&[profile]), None, false)
    .expect("the profile resolves")
}

/// A declared seat that is neither the mediator nor the human seat: one a
/// retirement may take.
fn a_retirable_seat(profile: &str) -> String {
  let r = resolved(profile);
  r.seats
    .iter()
    .map(|s| s.moniker.clone())
    .find(|m| *m != r.mediator && *m != r.human)
    .expect("the profile declares a seat that is neither the mediator nor the human")
}

fn applied(fx: &Fixture, profile: &str) -> Facade {
  let mut f = fx.facade_on_disk();
  f.wb_init(&names(&[profile]), true, false)
    .expect("the apply runs");
  f
}

fn retired_of(f: &Facade, moniker: &str) -> Option<intentsvcs::model::WbRetired> {
  f.boards()
    .expect("boards")
    .into_iter()
    .find(|b| b.node.moniker == moniker)
    .and_then(|b| b.node.retired)
}

/// The project's files, without the cache, for "nothing moved".
fn files(fx: &Fixture) -> std::collections::BTreeMap<String, Vec<u8>> {
  let mut files = crate::common::tree(fx.root());
  files.retain(|path, _| !path.starts_with("intent/.cache/"));
  files
}

/// ST0092 AT-04.1, bank (i)'s part: a retired node keeps its board and is
/// read without `--all`, saying retired; `wb status` lists it only with
/// `--all`.
#[test]
fn at_04_1_a_retired_node_keeps_its_board_and_status_lists_it_only_with_all() {
  let fx = Fixture::new();
  let mut f = applied(&fx, TEAM);
  let seat = a_retirable_seat(TEAM);
  f.wb_add(&seat, intentsvcs::model::WbItemKind::Todo, "work it held")
    .expect("an item");

  let report = f.wb_retire(&seat, REASON).expect("the retirement");
  assert!(report.retired);
  let held = retired_of(&f, &seat).expect("the node is retired");
  assert_eq!(held.reason, REASON);

  let read = f
    .board_read(&seat, false)
    .expect("a retired board reads without --all");
  assert!(
    read.board.node.retired.is_some(),
    "the read does not say retired"
  );
  assert!(
    read.board.items.iter().any(|i| i.text == "work it held"),
    "the retired node's items are gone"
  );

  let listed = |all: bool| -> Vec<String> {
    f.wb_status(all)
      .expect("status")
      .rows
      .into_iter()
      .map(|r| r.board.node.moniker)
      .collect()
  };
  assert!(
    !listed(false).contains(&seat),
    "status lists a retired node by default"
  );
  assert!(
    listed(true).contains(&seat),
    "status --all omits a retired node"
  );
}

/// ST0092 AT-04.2: a retirement refuses without a reason, a node holding a
/// claim (naming each), the board's mediator and its human seat, and writes
/// nothing for any of them.
#[test]
fn at_04_2_a_retirement_refuses_no_reason_claims_the_mediator_and_the_human() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut f = applied(&fx, TEAM);
  let team = resolved(TEAM);
  let seat = a_retirable_seat(TEAM);
  let before = files(&fx);

  assert!(matches!(
    f.wb_retire(&seat, " "),
    Err(FacadeError::WbRetireNoReason { .. })
  ));
  assert!(matches!(
    f.wb_retire(&team.mediator, REASON),
    Err(FacadeError::WbRetireMediator { .. })
  ));
  assert!(matches!(
    f.wb_retire(&team.human, REASON),
    Err(FacadeError::WbRetireHuman { .. })
  ));
  assert_eq!(files(&fx), before, "a refused retirement wrote");

  f.wb_claim(&seat, "ST0001").expect("a claim");
  let refused = f
    .wb_retire(&seat, REASON)
    .expect_err("a node holding a claim");
  assert!(
    matches!(&refused, FacadeError::WbRetireHoldsClaims { claims, .. } if claims == &["ST0001".to_string()]),
    "{refused:?}"
  );
  assert!(
    refused.to_string().contains("ST0001"),
    "the refusal does not name the claim: {refused}"
  );
  assert!(
    retired_of(&f, &seat).is_none(),
    "a refused retirement retired the node"
  );

  assert!(matches!(
    f.wb_retire("zz", REASON),
    Err(FacadeError::WbNodeNotRegistered { .. })
  ));
}

/// ST0092 AT-04.3, bank (i)'s part: a live message does not refuse, and one
/// addressed to the leaver that it never handled is named in the report.
#[test]
fn at_04_3_a_live_message_to_the_leaver_is_named_in_the_report() {
  let fx = Fixture::new();
  let mut f = applied(&fx, TEAM);
  let team = resolved(TEAM);
  let seat = a_retirable_seat(TEAM);
  f.wb_ask(&team.mediator, &seat, "one for you", None, false)
    .expect("an ask");
  let report = f
    .wb_retire(&seat, REASON)
    .expect("a live message does not refuse");
  assert!(report.retired);
  assert_eq!(report.awaiting.len(), 1, "{:?}", report.awaiting);
  assert_eq!(report.awaiting[0].sender, team.mediator);
}

/// ST0092 AT-04.4: a retired node's pair views go, on `removed:`, only when
/// they hold no live message; one holding a live message stays until it is
/// handled.
#[test]
fn at_04_4_a_retired_pairs_views_go_only_when_nothing_in_them_is_live() {
  let fx = Fixture::new();
  let mut f = applied(&fx, TEAM);
  let team = resolved(TEAM);
  let seat = a_retirable_seat(TEAM);
  let peer = team.mediator.clone();
  let quiet = team
    .seats
    .iter()
    .map(|s| s.moniker.clone())
    .find(|m| *m != seat && *m != peer)
    .expect("a third seat");
  f.wb_ask(&seat, &peer, "the leaver's last word", None, false)
    .expect("an ask from the leaver");
  let _ = f.take_written_by_the_roster();

  let live_view = format!("intent/whiteboard/{peer}/inbox.{seat}.md");
  let quiet_views = [
    format!("intent/whiteboard/{quiet}/inbox.{seat}.md"),
    format!("intent/whiteboard/{seat}/inbox.{quiet}.md"),
  ];
  f.wb_retire(&seat, REASON).expect("the retirement");
  let written = f.take_written_by_the_roster();
  for view in &quiet_views {
    assert!(
      !fx.root().join(view).exists(),
      "{view} holds nothing live and stayed"
    );
    assert!(
      written.removed.contains(view),
      "{view} is not on a removed: line: {written:?}"
    );
  }
  assert!(
    fx.root().join(&live_view).exists(),
    "a view holding a live message was removed"
  );

  f.wb_clear(&peer, &seat).expect("the recipient handles it");
  assert!(
    !fx.root().join(&live_view).exists(),
    "the view stayed once nothing in it was live"
  );
}

/// ST0092 AT-04.5: registering a retired moniker again reinstates the same
/// node with its board, and never makes a second.
#[test]
fn at_04_5_registering_a_retired_moniker_reinstates_the_same_node() {
  let fx = Fixture::new();
  let mut f = applied(&fx, TEAM);
  let seat = a_retirable_seat(TEAM);
  f.wb_add(
    &seat,
    intentsvcs::model::WbItemKind::Todo,
    "kept across the retirement",
  )
  .expect("an item");
  f.wb_retire(&seat, REASON).expect("the retirement");
  let count = f.boards().expect("boards").len();

  let declared = resolved(TEAM)
    .seats
    .into_iter()
    .find(|s| s.moniker == seat)
    .expect("the seat");
  assert_eq!(
    f.wb_register(&seat, &declared.name, &declared.role)
      .expect("the reinstatement"),
    1
  );
  assert!(retired_of(&f, &seat).is_none(), "the node is still retired");
  let boards = f.boards().expect("boards");
  assert_eq!(boards.len(), count, "a second node was made");
  let board = boards
    .iter()
    .find(|b| b.node.moniker == seat)
    .expect("the node");
  assert!(
    board
      .items
      .iter()
      .any(|i| i.text == "kept across the retirement")
  );

  assert!(
    f.wb_retire(&seat, REASON).expect("again").retired,
    "a reinstated node could not be retired again"
  );
  assert!(
    !f.wb_retire(&seat, REASON).expect("a no-op").retired,
    "retiring a retired node is not a no-op"
  );
}

/// ST0092 AT-04.6: `wb init <base> --change --apply` registers the new
/// base's seats and retires the old base's seats the new one does not
/// declare; its dry run lists both; and a re-apply without `--change` retires
/// nothing.
#[test]
fn at_04_6_a_change_of_base_registers_the_new_seats_and_retires_the_old() {
  let fx = Fixture::new();
  applied(&fx, SOLO);
  let (solo, team) = (resolved(SOLO), resolved(TEAM));
  let leaving: Vec<String> = solo
    .seats
    .iter()
    .map(|s| s.moniker.clone())
    .filter(|m| !team.seats.iter().any(|s| &s.moniker == m))
    .collect();
  assert!(
    !leaving.is_empty(),
    "the fixture needs a seat solo declares and team does not"
  );

  let mut f = fx.facade_on_disk();
  let plan = f
    .wb_init(&names(&[TEAM]), false, true)
    .expect("the dry run");
  for m in &leaving {
    assert!(
      plan
        .seats
        .iter()
        .any(|s| &s.moniker == m && s.verdict == Verdict::Retire),
      "the dry run does not list {m} as retired: {:?}",
      plan.seats
    );
  }
  assert!(plan.seats.iter().any(|s| s.verdict == Verdict::Register));
  assert!(
    leaving.iter().all(|m| retired_of(&f, m).is_none()),
    "the dry run retired"
  );

  f.wb_init(&names(&[TEAM]), true, true)
    .expect("the change applies");
  for m in &leaving {
    assert!(retired_of(&f, m).is_some(), "{m} was not retired");
  }
  for seat in &team.seats {
    assert!(
      f.boards()
        .expect("boards")
        .iter()
        .any(|b| b.node.moniker == seat.moniker && b.node.retired.is_none()),
      "{} is not registered",
      seat.moniker
    );
  }
}

/// vc's ruling (2) on WP-04: a re-apply reinstates nothing, as it retires
/// nothing. A declared seat retired by hand is left retired by `wb init
/// --apply`, no file moves, and the plan lists it as retired with its reason.
#[test]
fn a_reapply_leaves_a_retired_declared_seat_retired_and_moves_nothing() {
  let fx = Fixture::new();
  let mut f = applied(&fx, TEAM);
  let seat = a_retirable_seat(TEAM);
  f.wb_retire(&seat, REASON).expect("the retirement");
  let before = files(&fx);

  let plan = f
    .wb_init(&names(&[TEAM]), false, false)
    .expect("the dry run");
  assert!(
    plan.seats.iter().any(|s| s.moniker == seat
      && s.verdict
        == Verdict::Retired {
          reason: REASON.to_string()
        }),
    "the plan does not list {seat} as retired: {:?}",
    plan.seats
  );
  f.wb_init(&names(&[TEAM]), true, false)
    .expect("the re-apply");
  assert!(
    retired_of(&f, &seat).is_some(),
    "the re-apply reinstated {seat}"
  );
  assert_eq!(files(&fx), before, "the re-apply moved a file");
}
