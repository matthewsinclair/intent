//! ST0092 WP-05: on a board that declares a team, hv is reached through one
//! node, and the tool enforces it; a board that declares none is as it was.
//!
//! **EVERY ARM HERE COMPILES ON THE CODE BEFORE WP-05** (vc's ruling on the
//! AT-05 rows' red): each reads what a caller reads, the files, the boards and
//! the words of a refusal, so its red is this arm failing on that code. The
//! checks that need WP-05's own types, the caller's origin and the refusals'
//! variants, are in `wb_hv_typed.rs`.
//!
//! Every arm runs in a fresh fixture project, reads the shipped profiles from
//! the install this test binary resolves, and takes the human seat and the
//! mediator from the declaration the apply recorded, so no seat is written
//! down here.

use intentsvcs::model::{TeamDeclaration, WbItemKind};
use intentsvcs::teams::Teams;

use crate::common::Fixture;

pub(crate) const TEAM: &str = "team";

pub(crate) fn names(list: &[&str]) -> Vec<String> {
  list.iter().map(|s| (*s).to_string()).collect()
}

/// A fixture whose board declares the `team` profile, and its declaration.
pub(crate) fn declared() -> (Fixture, TeamDeclaration) {
  let fx = Fixture::new();
  let mut f = fx.facade_on_disk();
  f.wb_init(&names(&[TEAM]), true, false)
    .expect("the apply declares the team");
  let d = f
    .canon()
    .declaration
    .clone()
    .expect("the apply recorded a declaration");
  (fx, d)
}

/// A seat the declaration names that is neither the human nor the mediator.
pub(crate) fn worker(d: &TeamDeclaration) -> String {
  let teams = Teams::new(&testkit::repo_root());
  teams
    .resolve(&names(&[TEAM]), None, false)
    .expect("team resolves")
    .seats
    .into_iter()
    .map(|s| s.moniker)
    .find(|m| m != &d.human && m != &d.mediator)
    .expect("the team profile has a seat beside the human and the mediator")
}

/// ST0092 AT-05.1: a declared board renders the human seat's one pair, with the
/// mediator, and no other pair of it, either way.
#[test]
fn at_05_1_a_declared_board_renders_hv_only_with_the_mediator() {
  let (fx, d) = declared();
  let w = worker(&d);
  let inbox = |recipient: &str, sender: &str| {
    fx.path(&format!("intent/whiteboard/{recipient}/inbox.{sender}.md"))
      .is_file()
  };
  assert!(inbox(&d.human, &d.mediator) && inbox(&d.mediator, &d.human));
  assert!(!inbox(&d.human, &w), "hv/inbox.{w}.md was rendered");
  assert!(!inbox(&w, &d.human), "{w}/inbox.hv.md was rendered");
}

/// ST0092 AT-05.2: an ask from a node to the human seat, and from the human
/// seat to a node, is refused unless the other end is the mediator; the refusal
/// names the mediator, and its remedy gives the same message to it as `wb ask`.
#[test]
fn at_05_2_an_ask_on_a_closed_pair_is_refused_naming_the_mediator() {
  let (fx, d) = declared();
  let w = worker(&d);
  let mut f = fx.facade_on_disk();
  let body = "a question for 'hv'";
  for (sender, recipient) in [
    (w.as_str(), d.human.as_str()),
    (d.human.as_str(), w.as_str()),
  ] {
    let err = f
      .wb_ask(sender, recipient, body, None, false)
      .expect_err("a closed pair");
    assert!(
      err.to_string().contains(&format!("`{}`", d.mediator)),
      "{err}"
    );
    let remedy = intentsvcs::remedy::Remedy::remedy(&err);
    assert!(
      remedy.contains(&format!("intent wb ask {} '", d.mediator))
        && remedy.contains(&format!("--node {sender}")),
      "{remedy}"
    );
  }
  f.wb_ask(&w, &d.mediator, body, None, false)
    .expect("the remedy's own message is taken");
  f.wb_ask(&d.mediator, &d.human, body, None, false)
    .expect("the mediator reaches hv");
}

/// ST0092 AT-05.3: an announce sends no copy to the human seat, and reaches
/// every other seat.
#[test]
fn at_05_3_an_announce_sends_no_copy_to_hv() {
  let (fx, d) = declared();
  let w = worker(&d);
  let mut f = fx.facade_on_disk();
  f.wb_announce(&d.mediator, "to everyone")
    .expect("the announce");
  let sent = f.board(&d.mediator).expect("the mediator's board").sent;
  assert!(
    !sent.iter().any(|m| m.recipient == d.human),
    "the announce copied hv"
  );
  assert!(sent.iter().any(|m| m.recipient == w), "{w} was not reached");
}

/// ST0092 AT-05.5: hv's directives, decisions and watch-outs come back in full
/// at every other seat's pickup. **THIS HOLDS WHAT IS ALREADY TRUE**, so it is
/// green before WP-05 too: the package must not lose it.
#[test]
fn at_05_5_hvs_standing_items_print_at_every_pickup() {
  let (fx, d) = declared();
  let mut f = fx.facade_on_disk();
  f.wb_add(&d.human, WbItemKind::Directive, "the team's law")
    .expect("a directive");
  f.wb_decide(&d.human, "a ruling").expect("a decision");
  f.wb_add(&d.human, WbItemKind::Watchout, "a hazard")
    .expect("a watch-out");
  for node in [worker(&d), d.mediator.clone()] {
    let standing = f
      .wb_pickup(&node, None, None, false)
      .expect("the pickup")
      .standing
      .expect("hv's standing items");
    let texts: Vec<&str> = standing.iter().map(|i| i.text.as_str()).collect();
    for text in ["the team's law", "a ruling", "a hazard"] {
      assert!(texts.contains(&text), "{node} missed {text}: {texts:?}");
    }
  }
}

/// The seats `team` declares, registered on a board that declares nothing yet,
/// with a worker's live ask to hv on it: the board a crossing starts from.
pub(crate) struct Crossing {
  pub fx: Fixture,
  pub human: String,
  pub w: String,
}

pub(crate) fn crossing() -> Crossing {
  let fx = Fixture::new();
  let resolved = Teams::new(&testkit::repo_root())
    .resolve(&names(&[TEAM]), None, false)
    .expect("team resolves");
  let (human, mediator) = (resolved.human.clone(), resolved.mediator.clone());
  let w = resolved
    .seats
    .iter()
    .map(|s| s.moniker.clone())
    .find(|m| m != &human && m != &mediator)
    .expect("a worker seat");
  let mut f = fx.facade_on_disk();
  for seat in &resolved.seats {
    f.wb_register(&seat.moniker, &seat.name, &seat.role)
      .expect("register before declaring");
  }
  f.wb_ask(&w, &human, "still waiting", None, false)
    .expect("an undeclared board takes the ask");
  Crossing { fx, human, w }
}

/// ST0092 AT-05.6: the apply that would close a pair holding a live message
/// refuses, in the dry run as in the apply, and names it with its sender; once
/// it is handled the apply proceeds, drops the view, and the message stays
/// readable.
#[test]
fn at_05_6_the_crossing_waits_for_a_live_message() {
  let Crossing { fx, human, w } = crossing();
  let mut f = fx.facade_on_disk();
  for apply in [false, true] {
    let err = f
      .wb_init(&names(&[TEAM]), apply, false)
      .expect_err("a live message on a pair the team closes");
    assert!(
      err
        .to_string()
        .contains(&format!("from `{w}` to `{human}`")),
      "{err}"
    );
  }
  f.wb_clear(&human, &w).expect("hv clears it");
  f.wb_init(&names(&[TEAM]), true, false)
    .expect("the crossing proceeds once it is handled");
  let removed = f.take_written_by_the_roster().removed;
  let view = format!("intent/whiteboard/{human}/inbox.{w}.md");
  assert!(removed.contains(&view), "{removed:?}");
  assert!(!fx.path(&view).exists());
  assert!(
    f.board(&w)
      .expect("the sender's board")
      .sent
      .iter()
      .any(|m| m.body == "still waiting"),
    "the handled message is still on its sender's board"
  );
}

/// ST0092 AT-05.7: a board that declares no team renders every pair and takes
/// every ask and announce, as before this thread. **GREEN BEFORE WP-05 TOO, BY
/// DESIGN**: it holds that nothing changed for a board that opts out.
#[test]
fn at_05_7_an_undeclared_board_is_as_it_was() {
  let fx = Fixture::new();
  let mut f = fx.facade_on_disk();
  let nodes = [("hv", "human"), ("vc", "mediator"), ("cc", "control")];
  for (m, role) in nodes {
    f.wb_register(m, &format!("{m} seat"), role)
      .expect("register");
  }
  f.wb_ask("cc", "hv", "direct", None, false)
    .expect("an ask to hv");
  assert_eq!(
    f.wb_announce("vc", "all").expect("an announce"),
    nodes.len() - 1,
    "the announce reached both other nodes, hv included"
  );
  assert!(fx.path("intent/whiteboard/hv/inbox.cc.md").is_file());
  assert!(fx.path("intent/whiteboard/cc/inbox.hv.md").is_file());
}
