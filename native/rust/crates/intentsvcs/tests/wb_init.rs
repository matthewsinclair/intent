//! ST0092 WP-03: `wb init` brings a board to what its Team Profiles declare and
//! touches nothing else on it; `wb remit` sets a project's own remit; `wb
//! register` honours a declaration; and a 4.0 board reads unchanged.
//!
//! Every arm runs in a fresh fixture project and reads the shipped profiles
//! from the install this test binary resolves, which is this repository, so
//! the expected seats come from the profile files and are never written down
//! here.

use std::collections::BTreeSet;

use intentsvcs::facade::{Facade, FacadeError};
use intentsvcs::finding::FindingClass;
use intentsvcs::model::WbItemKind;
use intentsvcs::teams::{Teams, TeamsError, Verdict};

use crate::common::{Fixture, ctx};

const TEAM: &str = "team";

fn names(list: &[&str]) -> Vec<String> {
  list.iter().map(|s| (*s).to_string()).collect()
}

/// The `team` profile's seats, read from the shipped files.
fn team_seats() -> Vec<intentsvcs::teams::Seat> {
  let teams = Teams::new(&testkit::repo_root());
  let resolved = teams
    .resolve(&names(&[TEAM]), None, false)
    .expect("team resolves");
  resolved.seats
}

/// Every file in the project but the store's own, and the store's boards and
/// declaration as JSON: what "writes nothing" holds, read from the store as a
/// reader reads it rather than from its bytes, which an open moves.
fn state(fx: &Fixture) -> (std::collections::BTreeMap<String, Vec<u8>>, String) {
  let f = fx.facade_on_disk();
  let rows = serde_json::to_string(&(
    f.boards().expect("boards"),
    intentsvcs::store::Store::open(&fx.project().db_path())
      .expect("the store")
      .wb_declaration()
      .expect("the declaration"),
  ))
  .expect("json");
  drop(f);
  let mut files = crate::common::tree(fx.root());
  files.retain(|path, _| !path.starts_with("intent/.cache/"));
  (files, rows)
}

fn apply(f: &mut Facade, profiles: &[&str]) -> intentsvcs::facade::WbInit {
  f.wb_init(&names(profiles), true, false)
    .expect("the apply runs")
}

/// Every whiteboard path an apply wrote, as the facade reports them, without
/// the event file, which a dry run cannot name before the event is minted.
fn applied_paths(f: &mut Facade) -> BTreeSet<String> {
  let written = f.take_written_by_the_roster();
  written
    .wrote
    .into_iter()
    .chain(written.re_rendered.into_iter().map(|r| r.path))
    .filter(|p| !p.starts_with("intent/.canon/events/"))
    .collect()
}

/// ST0092 AT-03.1: in a project with no whiteboard, an apply registers every
/// seat the profile declares with its name and role, writes no remit,
/// records the declaration in the store and on disk, and names every file.
#[test]
fn at_03_1_an_apply_on_no_whiteboard_registers_every_declared_seat() {
  let fx = Fixture::new();
  let mut f = fx.facade_on_disk();
  let report = apply(&mut f, &[TEAM]);
  assert!(report.applied && report.declaration_moves);
  let boards = f.boards().expect("boards");
  let seats = team_seats();
  assert_eq!(boards.len(), seats.len(), "one board per declared seat");
  for seat in &seats {
    let b = boards
      .iter()
      .find(|b| b.node.moniker == seat.moniker)
      .unwrap_or_else(|| panic!("{} is not registered", seat.moniker));
    assert_eq!(
      (b.node.name.as_str(), b.node.role.as_str()),
      (seat.name.as_str(), seat.role.as_str())
    );
    assert_eq!(b.node.remit, None, "an apply writes no remit");
  }
  let disk = fx.read("intent/whiteboard/declaration.json");
  assert!(disk.contains("\"base\": \"team\""), "{disk}");
  let paths = applied_paths(&mut f);
  assert!(
    paths.contains("intent/whiteboard/declaration.json"),
    "{paths:?}"
  );
  for seat in &seats {
    let board = format!("intent/whiteboard/{}/board.json", seat.moniker);
    assert!(
      paths.contains(&board),
      "{board} is not on a wrote: line: {paths:?}"
    );
  }
}

/// ST0092 AT-03.2: the dry run gives each seat its verdict, writes nothing,
/// and names exactly the files the apply then writes (vc's ruling).
#[test]
fn at_03_2_the_dry_run_names_each_verdict_writes_nothing_and_names_what_the_apply_writes() {
  let fx = Fixture::new();
  let seats = team_seats();
  let (matching, misroled, absent) = (&seats[0], &seats[1], &seats[2]);
  {
    let mut f = fx.facade_on_disk();
    f.wb_register(&matching.moniker, &matching.name, &matching.role)
      .expect("a matching seat");
    f.wb_register(&misroled.moniker, &misroled.name, &misroled.moniker)
      .expect("a seat registered with its moniker as its role");
    f.wb_register("zz", "Extra Claude", "extra")
      .expect("a node no profile names");
  }
  let before = state(&fx);
  let mut f = fx.facade_on_disk();
  let dry = f
    .wb_init(&names(&[TEAM]), false, false)
    .expect("the dry run");
  assert_eq!(state(&fx), before, "the dry run wrote a file");
  let verdict = |m: &str| {
    dry
      .seats
      .iter()
      .find(|s| s.moniker == m)
      .map(|s| s.verdict.clone())
      .unwrap_or_else(|| panic!("no verdict for {m}"))
  };
  assert_eq!(verdict(&matching.moniker), Verdict::Unchanged);
  assert_eq!(
    verdict(&misroled.moniker),
    Verdict::Correct {
      held_name: misroled.name.clone(),
      held_role: misroled.moniker.clone(),
    }
  );
  assert_eq!(verdict(&absent.moniker), Verdict::Register);
  assert_eq!(verdict("zz"), Verdict::Outside);

  let would: BTreeSet<String> = dry.would_write.iter().cloned().collect();
  let mut f = fx.facade_on_disk();
  apply(&mut f, &[TEAM]);
  assert_eq!(
    would,
    applied_paths(&mut f),
    "the dry run named other files than the apply wrote"
  );
}

/// ST0092 AT-03.3: an apply over a populated board moves names, roles and the
/// roster set and nothing else on any board.
#[test]
fn at_03_3_an_apply_changes_only_names_roles_and_the_roster() {
  let fx = Fixture::new();
  let seats = team_seats();
  let (held, other) = (&seats[0], &seats[1]);
  {
    let mut f = fx.facade_on_disk();
    f.wb_register(&held.moniker, &held.name, &held.moniker)
      .expect("a seat registered wrong");
    f.wb_register(&other.moniker, &other.name, &other.role)
      .expect("a second seat");
    f.wb_add(&held.moniker, WbItemKind::Doing, "an item")
      .expect("an item");
    f.wb_ask(&held.moniker, &other.moniker, "a message", None, false)
      .expect("a message");
    f.wb_claim(&held.moniker, "ST0001").expect("a claim");
    f.wb_remit(&held.moniker, Some("this project's own"))
      .expect("a remit");
  }
  let kept = |f: &Facade| {
    f.boards()
      .expect("boards")
      .into_iter()
      .map(|b| {
        let mut node = b.node.clone();
        node.name = String::new();
        node.role = String::new();
        (node, b.items, b.sent, b.handled)
      })
      .collect::<Vec<_>>()
  };
  let before = kept(&fx.facade_on_disk());
  let mut f = fx.facade_on_disk();
  apply(&mut f, &[TEAM]);
  let after: Vec<_> = kept(&f)
    .into_iter()
    .filter(|(n, ..)| before.iter().any(|(b, ..)| b.moniker == n.moniker))
    .collect();
  assert_eq!(
    after, before,
    "an apply moved a field other than name, role and the roster"
  );
}

/// ST0092 AT-03.4: a second apply writes nothing and says so, and an apply
/// with no profile named re-applies the declaration.
#[test]
fn at_03_4_a_second_apply_writes_nothing_and_no_profile_reapplies() {
  let fx = Fixture::new();
  let mut f = fx.facade_on_disk();
  apply(&mut f, &[TEAM]);
  let again = apply(&mut f, &[TEAM]);
  assert_eq!((again.moved, again.declaration_moves), (0, false));
  let reinit = apply(&mut f, &[]);
  assert_eq!(
    reinit.resolved.base, TEAM,
    "the reinit re-applies the declared base"
  );
  assert_eq!((reinit.moved, reinit.declaration_moves), (0, false));
}

/// ST0092 AT-03.5: each refusal comes before anything is written.
#[test]
fn at_03_5_refusals_come_before_any_write() {
  let fx = Fixture::new();
  let teams = Teams::new(&testkit::repo_root());
  let layer = teams
    .profiles()
    .expect("profiles")
    .into_iter()
    .find(|p| p.front.kind == intentsvcs::teams::ProfileKind::Layer)
    .expect("a shipped layer");
  let other_base = teams
    .profiles()
    .expect("profiles")
    .into_iter()
    .find(|p| p.front.kind == intentsvcs::teams::ProfileKind::Base && p.name != TEAM)
    .expect("a second shipped base");

  let before = state(&fx);
  let mut f = fx.facade_on_disk();
  let err = f
    .wb_init(&names(&[&layer.name]), true, false)
    .expect_err("a layer with no base");
  assert!(
    matches!(err, FacadeError::Teams(TeamsError::LayerWithoutBase { .. })),
    "{err:?}"
  );
  assert_eq!(state(&fx), before);

  apply(&mut f, &[TEAM]);
  let before = state(&fx);
  let err = f
    .wb_init(&names(&[&other_base.name]), false, false)
    .expect_err("another base without --change");
  assert!(
    matches!(err, FacadeError::Teams(TeamsError::BaseDiffers { .. })),
    "{err:?}"
  );
  assert!(intentsvcs::remedy::Remedy::remedy(&err).contains("--change"));
  assert_eq!(state(&fx), before);

  let hand = Fixture::new();
  let seat = &team_seats()[0];
  hand.write_file(
    &format!("intent/whiteboard/{}/wip.md", seat.moniker),
    "# a hand-authored board\n",
  );
  let before = state(&hand);
  let mut f = hand.facade_on_disk();
  let err = f
    .wb_init(&names(&[TEAM]), true, false)
    .expect_err("an unmigrated node");
  assert!(
    matches!(err, FacadeError::WbNotMigrated { ref node } if node == &seat.moniker),
    "{err:?}"
  );
  assert!(intentsvcs::remedy::Remedy::remedy(&err).contains("wb migrate"));
  assert_eq!(state(&hand), before);
}

/// ST0092 AT-03.6: on a declared board, a declared moniker registers only as
/// its profile says and the refusal prints the profile's values; an
/// undeclared moniker registers as before, and the dry run calls it outside.
#[test]
fn at_03_6_register_honours_the_declaration() {
  let fx = Fixture::new();
  let mut f = fx.facade_on_disk();
  let seat = team_seats()[0].clone();
  f.wb_init(&names(&[TEAM]), true, false).expect("declare");
  f.wb_register("qq", "Q Claude", "extra")
    .expect("a moniker no declared profile names registers as before");
  let err = f
    .wb_register(&seat.moniker, "Someone", "else")
    .expect_err("a declared seat with other values");
  let FacadeError::WbRegisterNotAsDeclared {
    declared_name,
    declared_role,
    ..
  } = &err
  else {
    panic!("{err:?}");
  };
  assert_eq!((declared_name, declared_role), (&seat.name, &seat.role));
  let dry = f.wb_init(&[], false, false).expect("the dry run");
  assert!(
    dry
      .seats
      .iter()
      .any(|s| s.moniker == "qq" && s.verdict == Verdict::Outside),
    "{:?}",
    dry.seats
  );
}

/// ST0092 AT-03.6, through `--correct` (vc's ruling (i)): the same door, so a
/// declared seat is corrected only to the profile's values, and the refusal
/// moves nothing; a moniker no profile names corrects as before.
#[test]
fn at_03_6_correct_honours_the_declaration_too() {
  let fx = Fixture::new();
  let mut f = fx.facade_on_disk();
  let seat = team_seats()[0].clone();
  f.wb_init(&names(&[TEAM]), true, false).expect("declare");
  f.wb_register("qq", "Q Claude", "extra")
    .expect("an undeclared moniker");
  let before = state(&fx);
  let err = f
    .wb_correct(&seat.moniker, "Someone", "else")
    .expect_err("a declared seat corrected off its profile");
  let FacadeError::WbRegisterNotAsDeclared {
    declared_name,
    declared_role,
    ..
  } = &err
  else {
    panic!("{err:?}");
  };
  assert_eq!((declared_name, declared_role), (&seat.name, &seat.role));
  assert_eq!(state(&fx), before, "the refused correct moved something");
  assert!(
    !f.wb_correct(&seat.moniker, &seat.name, &seat.role)
      .expect("the profile's own values"),
    "the profile's values, already held, move nothing"
  );
  assert!(
    f.wb_correct("qq", "Q Claude", "other")
      .expect("an undeclared moniker corrects as before")
  );
}

/// ST0092 AT-03.7: a remit is the project's own, survives an apply, and
/// `--default` removes it; a seat with none is marked as showing the default.
#[test]
fn at_03_7_a_remit_survives_an_apply_and_default_removes_it() {
  let fx = Fixture::new();
  let mut f = fx.facade_on_disk();
  apply(&mut f, &[TEAM]);
  let seat = team_seats()[0].moniker.clone();
  assert!(f.wb_remit(&seat, Some("this project's own")).expect("set"));
  apply(&mut f, &[TEAM]);
  let remit = |f: &Facade| {
    f.boards()
      .expect("boards")
      .into_iter()
      .find(|b| b.node.moniker == seat)
      .and_then(|b| b.node.remit)
  };
  assert_eq!(remit(&f).as_deref(), Some("this project's own"));
  let marked = |f: &mut Facade| {
    f.wb_init(&[], false, false)
      .expect("dry run")
      .seats
      .into_iter()
      .find(|s| s.moniker == seat)
      .map(|s| s.default_remit)
  };
  assert_eq!(marked(&mut f), Some(false));
  assert!(f.wb_remit(&seat, None).expect("--default"));
  assert_eq!(remit(&f), None);
  assert_eq!(marked(&mut f), Some(true));
}

/// ST0092 AT-03.8: a clone of an applied project ingests the same
/// declaration, seats and remits from the canon on disk.
#[test]
fn at_03_8_a_clone_ingests_the_declaration_seats_and_remits() {
  let fx = Fixture::new();
  let seat = team_seats()[0].moniker.clone();
  let (declared, boards) = {
    let mut f = fx.facade_on_disk();
    apply(&mut f, &[TEAM]);
    f.wb_remit(&seat, Some("this project's own")).expect("set");
    (
      std::fs::read_to_string(fx.path("intent/whiteboard/declaration.json")).expect("declaration"),
      f.boards().expect("boards"),
    )
  };
  let clone = fx.clone_extract();
  let f = clone.facade_on_disk();
  let identity = |b: &[intentsvcs::model::Board]| {
    b.iter()
      .map(|b| {
        (
          b.node.moniker.clone(),
          b.node.name.clone(),
          b.node.role.clone(),
          b.node.remit.clone(),
        )
      })
      .collect::<Vec<_>>()
  };
  assert_eq!(
    identity(&f.boards().expect("clone boards")),
    identity(&boards)
  );
  assert_eq!(
    f.canon().declaration.as_ref().map(|d| d.base.as_str()),
    Some(TEAM)
  );
  assert_eq!(
    std::fs::read_to_string(clone.path("intent/whiteboard/declaration.json"))
      .expect("clone's file"),
    declared
  );
}

/// ST0092 AT-03.9, and vc's condition on the schema ruling: a board at
/// `intent/board@4.0` reads unchanged under this build, doctor names it as
/// shown and never counted, and the next board write re-renders it at the
/// current shape.
#[test]
fn at_03_9_a_4_0_board_reads_unchanged_is_named_and_re_renders() {
  let fx = Fixture::new();
  let seat = team_seats()[0].moniker.clone();
  let board_rel = format!("intent/whiteboard/{seat}/board.json");
  let held = {
    let mut f = fx.facade_on_disk();
    apply(&mut f, &[TEAM]);
    f.boards().expect("boards")
  };
  let text = fx.read(&board_rel);
  let old = text.replace(
    intentsvcs::model::BOARD_SCHEMA,
    intentsvcs::model::BOARD_SCHEMA_V4_0,
  );
  assert_ne!(
    old, text,
    "the fixture names the current schema, so it can be put back a version"
  );
  fx.write_file(&board_rel, &old);

  let clone = fx.clone_extract();
  let read = clone.facade_on_disk().boards().expect("a 4.0 board reads");
  let node = |b: &[intentsvcs::model::Board]| b.iter().find(|b| b.node.moniker == seat).cloned();
  assert_eq!(
    node(&read).map(|b| (b.node, b.items, b.sent, b.handled)),
    node(&held).map(|b| (b.node, b.items, b.sent, b.handled)),
    "a 4.0 board read differently"
  );

  let report =
    intentsvcs::doctor::diagnose(&fx.project(), &ctx(), None, intentsvcs::doctor::Scope::Live);
  let named: Vec<_> = report
    .findings
    .iter()
    .filter(|f| f.class == FindingClass::BoardSchemaBehind)
    .collect();
  assert!(
    named.iter().any(|f| f.file.contains(&seat)),
    "{:?}",
    report.findings
  );
  assert!(
    named
      .iter()
      .all(|f| !f.class.is_actionable() && f.class.is_shown_by_default())
  );

  let mut f = fx.facade_on_disk();
  f.wb_touch(&seat).expect("a board write");
  assert!(
    fx.read(&board_rel)
      .contains(intentsvcs::model::BOARD_SCHEMA),
    "the next board write left the board at 4.0"
  );
}
