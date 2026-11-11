//! ST0092 WP-09: `intent doctor` names a board that has left the profile it
//! declares, each way as its own class whose remedy is `intent wb init`, and
//! none of them is counted, so the pre-commit gate passes a tree that carries
//! them.
//!
//! **EVERY ARM HERE COMPILES ON THE CODE BEFORE WP-09**: a class is read by the
//! id it prints, so the red is the class not being reported.
//!
//! The drifted board is built the way one arrives: its seats registered before
//! a declaration reached it, then the declaration written as a pull or another
//! machine's apply would leave it, with nothing on this machine running the
//! apply that converges the board.

use intentsvcs::teams::Teams;

use crate::common::{Fixture, ctx};

const TEAM: &str = "team";
const DIFFERS: &str = "seat-differs-from-profile";
const UNREGISTERED: &str = "seat-not-registered";
const CLOSED_VIEW: &str = "closed-pair-view";
const NOT_THE_PROFILES: &str = "Not the profile's name";
const RETIRED: &str = "seat-retired";
const WHY_RETIRED: &str = "moved to another project";

fn report(fx: &Fixture) -> intentsvcs::doctor::Report {
  intentsvcs::doctor::diagnose(&fx.project(), &ctx(), None, intentsvcs::doctor::Scope::Live)
}

/// Doctor's findings of `class`, by the id it prints: each one's file, detail
/// and remedy, with whether it is counted and whether it is shown by default.
fn found(fx: &Fixture, class: &str) -> Vec<(String, String, String, bool, bool)> {
  report(fx)
    .findings
    .iter()
    .filter(|f| f.class.as_str() == class)
    .map(|f| {
      (
        f.file.clone(),
        f.detail.clone(),
        f.class.remedy().to_string(),
        f.class.is_actionable(),
        f.class.is_shown_by_default(),
      )
    })
    .collect()
}

/// The drifted board, and the seats each class should name.
struct Drifted {
  fx: Fixture,
  differs: String,
  unregistered: String,
  view: String,
  /// Doctor's count on the same board before the declaration reached it.
  counted_before: usize,
}

fn drifted() -> Drifted {
  let fx = Fixture::new();
  let resolved = Teams::new(&testkit::repo_root())
    .resolve(&[TEAM.to_string()], None, false)
    .expect("team resolves");
  let workers: Vec<&intentsvcs::teams::Seat> = resolved
    .seats
    .iter()
    .filter(|s| s.moniker != resolved.human && s.moniker != resolved.mediator)
    .collect();
  let [differs, unregistered, ..] = workers.as_slice() else {
    panic!("the team profile has two seats beside the human and the mediator");
  };
  let mut f = fx.facade_on_disk();
  for seat in resolved
    .seats
    .iter()
    .filter(|s| s.moniker != unregistered.moniker)
  {
    let name = if seat.moniker == differs.moniker {
      NOT_THE_PROFILES
    } else {
      seat.name.as_str()
    };
    f.wb_register(&seat.moniker, name, &seat.role)
      .expect("register before declaring");
  }
  f.wb_ask(
    &differs.moniker,
    &resolved.human,
    "before the team",
    None,
    false,
  )
  .expect("an undeclared board takes the ask");
  f.wb_clear(&resolved.human, &differs.moniker)
    .expect("hv handles it");
  let counted_before = report(&fx).actionable();
  let declaration =
    serde_json::to_string_pretty(&resolved.declaration(None)).expect("a declaration serialises");
  fx.write_file("intent/whiteboard/declaration.json", &declaration);
  Drifted {
    fx,
    differs: differs.moniker.clone(),
    unregistered: unregistered.moniker.clone(),
    view: format!(
      "intent/whiteboard/{}/inbox.{}.md",
      resolved.human, differs.moniker
    ),
    counted_before,
  }
}

/// ST0092 AT-09.1: each way a board leaves its profile is its own class, named
/// on the file it is about, saying which profile it measured against, with the
/// dry run and then the apply of `intent wb init` as its remedy; and the apply
/// clears all three.
#[test]
fn at_09_1_doctor_names_each_way_a_board_has_left_its_profile() {
  let d = drifted();
  let differs = found(&d.fx, DIFFERS);
  assert!(
    differs.iter().any(|(file, detail, ..)| file
      == &format!("intent/whiteboard/{}/board.json", d.differs)
      && detail.contains(NOT_THE_PROFILES)),
    "{differs:?}"
  );
  let unregistered = found(&d.fx, UNREGISTERED);
  assert!(
    unregistered
      .iter()
      .any(|(_, detail, ..)| detail.contains(&format!("`{}`", d.unregistered))),
    "{unregistered:?}"
  );
  let view = found(&d.fx, CLOSED_VIEW);
  assert!(view.iter().any(|(file, ..)| file == &d.view), "{view:?}");
  for (_, detail, remedy, ..) in differs.iter().chain(&unregistered).chain(&view) {
    assert!(detail.contains(&format!("`{TEAM}`")), "{detail}");
    let dry = remedy.find("`intent wb init`").expect(remedy);
    let apply = remedy.find("`intent wb init --apply`").expect(remedy);
    assert!(dry < apply, "the dry run comes first: {remedy}");
  }

  let mut f = d.fx.facade_on_disk();
  f.wb_init(&[TEAM.to_string()], true, false)
    .expect("the apply converges the board");
  for class in [DIFFERS, UNREGISTERED, CLOSED_VIEW] {
    let left = found(&d.fx, class);
    assert!(left.is_empty(), "{class} after the apply: {left:?}");
  }
}

/// ST0092 AT-09.2: none of the three is counted, and each is shown by default,
/// so doctor's count, which the pre-commit gate reads, is what it was before
/// the declaration reached the board.
#[test]
fn at_09_2_no_class_of_a_board_off_its_profile_refuses_a_commit() {
  let d = drifted();
  for class in [DIFFERS, UNREGISTERED, CLOSED_VIEW] {
    let each = found(&d.fx, class);
    assert!(!each.is_empty(), "{class} was not reported");
    assert!(
      each.iter().all(|(.., counted, shown)| !counted && *shown),
      "{class}: {each:?}"
    );
  }
  assert_eq!(report(&d.fx).actionable(), d.counted_before);
}

/// ST0092 AT-09.4: a declared seat that is retired is named as retired, on its
/// own board, with the profile and the reason, shown and never counted; it is
/// not a declared seat that no board registers, and an apply leaves it
/// retired, so doctor still names it after one.
#[test]
fn at_09_4_doctor_names_a_retired_declared_seat_as_retired() {
  let d = drifted();
  let resolved = Teams::new(&testkit::repo_root())
    .resolve(&[TEAM.to_string()], None, false)
    .expect("team resolves");
  let retiree = resolved
    .seats
    .iter()
    .map(|s| s.moniker.as_str())
    .find(|m| {
      ![
        resolved.human.as_str(),
        resolved.mediator.as_str(),
        d.differs.as_str(),
        d.unregistered.as_str(),
      ]
      .contains(m)
    })
    .expect("the team profile has a third seat beside the human and the mediator")
    .to_string();
  let mut f = d.fx.facade_on_disk();
  f.wb_retire(&retiree, WHY_RETIRED)
    .expect("a declared worker seat retires");

  let named = |fx: &Fixture| {
    let retired = found(fx, RETIRED);
    assert!(
      retired.iter().any(|(file, detail, _, counted, shown)| file
        == &format!("intent/whiteboard/{retiree}/board.json")
        && detail.contains(&format!("`{retiree}`"))
        && detail.contains(&format!("`{TEAM}`"))
        && detail.contains(WHY_RETIRED)
        && detail.contains(&intentsvcs::model::register_form(&retiree))
        && !counted
        && *shown),
      "{retired:?}"
    );
    let unregistered = found(fx, UNREGISTERED);
    assert!(
      unregistered
        .iter()
        .all(|(_, detail, ..)| !detail.contains(&format!("`{retiree}`"))),
      "{unregistered:?}"
    );
  };
  named(&d.fx);
  assert_eq!(report(&d.fx).actionable(), d.counted_before);

  f.wb_init(&[TEAM.to_string()], true, false)
    .expect("the apply converges the board");
  named(&d.fx);
}
