//! AT-14.9 / AC-14.9 -- ST0069 WP-14: `wb migrate` carries a node's whole
//! hand-authored board into the model, and names every line it does not.
//!
//! **THE ARMS BELOW ARE ABOUT THE WHOLE NODE, not about the parser.**
//! `wbmigrate_reads_a_board.rs` holds the reader to the shapes real boards
//! have; this one holds the VERB to what it wrote: which rows landed, which
//! stamps survived, which documents came across, and what came back named
//! instead. The fixture is built from the estate's own board shape -- prose
//! DOING, bullet TODO, a `## Holds` section, a section the model maps to
//! nothing, two inboxes, one of them from a sender the roster does not carry,
//! and a `.history/` fold beside a file that is not a document.

use crate::common::Fixture;
use intentsvcs::model::{WbItemKind, WbMessageState};

const BOARD: &str = r#"---
node: dc
name: DevX Claude
role: worker
session_id: b9e78c72
heartbeat_at: 2026-09-12 18:31Z
status: active
focus: "WP-14: the migration verb"
claims: [ST0069/14]
---

# DevX Claude (dc)

## DOING

**The busiest section on every real board is prose.**
It runs to more than one line.

## TODO

- The queued thing.

## Holds

- Held until the fifth kind lands, which is the condition that releases it.

## Watch-outs

- A caution that outlives the work.

## Decisions

- Ruled on 2026-09-12.

## Standing directives

- A section the model maps to no kind.
"#;

const FROM_VC: &str = r#"# inbox: vc -> dc

## (2026-09-12 15:03Z) Re: your 15:01Z -- three rulings

All three are ruled.

## (2026-09-12 18:15Z) FYI only -- no response needed.

**BROADCAST: the live store is at schema 24.**
"#;

const FROM_A_STRANGER: &str = r#"# inbox: laksa-vc -> dc

## (2026-09-12 09:00Z)

A message from a node that is not on this project's roster.
"#;

const FOLD: &str = "# DevX Claude (dc)\n\n## DOING\n\nWhat the board said before the fold.\n";

/// Write the node's whole directory, register the roster, and carry it.
fn carried() -> (
  Fixture,
  intentsvcs::facade::Facade,
  intentsvcs::facade::WbMigration,
) {
  let fx = Fixture::new();
  let home = fx.root().join("intent/whiteboard/dc");
  std::fs::create_dir_all(home.join(".history/20260912")).expect("the node's directories");
  std::fs::write(home.join("wip.md"), BOARD).expect("the board");
  std::fs::write(home.join("inbox.vc.md"), FROM_VC).expect("an inbox from a registered peer");
  std::fs::write(home.join("inbox.laksa-vc.md"), FROM_A_STRANGER)
    .expect("an inbox from a stranger");
  std::fs::write(home.join(".history/20260912/wip-prefold-1400Z.md"), FOLD).expect("a fold");
  std::fs::write(home.join(".history/20260912/board.png"), b"not a document").expect("not a doc");
  let vc = fx.root().join("intent/whiteboard/vc");
  std::fs::create_dir_all(&vc).expect("the peer's directory");
  std::fs::write(
    vc.join("wip.md"),
    "---\nnode: vc\nname: Validation Claude\nrole: validation\nstatus: active\n---\n",
  )
  .expect("the peer's header, so the roster carries the sender");

  let mut facade = fx.facade();
  facade.register_roster().expect("register the roster");
  let carried = facade.wb_migrate("dc").expect("carry the board");
  (fx, facade, carried)
}

/// The board half: every section the model has a kind for lands as that kind,
/// and the header's claimed stamp lands where a claim belongs.
#[test]
fn the_board_lands_with_its_holds_and_its_untrusted_stamp() {
  let (_fx, facade, carried) = carried();
  let board = facade.board("dc").expect("the board");

  let kinds: Vec<(WbItemKind, &str)> = board
    .items
    .iter()
    .map(|i| (i.kind, i.text.as_str()))
    .collect();
  assert!(
    kinds
      .iter()
      .any(|(k, t)| *k == WbItemKind::Hold && t.contains("condition that releases it")),
    "a hold carries as a hold, with the condition that makes it one: {kinds:?}"
  );
  assert!(
    kinds
      .iter()
      .any(|(k, t)| *k == WbItemKind::Doing && t.contains("more than one line")),
    "a prose DOING paragraph carries whole: {kinds:?}"
  );

  assert_eq!(
    board.node.authored_at.as_deref(),
    Some("2026-09-12 18:31Z"),
    "the board's own heartbeat claim is carried verbatim, untrusted"
  );
  assert_ne!(
    board.node.heartbeat_at.as_str(),
    "2026-09-12 18:31Z",
    "and the SERVICE stamp takes the ingest instant rather than the claim -- which is the whole \
     of AC-14.4 against AC-14.9: the claimed value is precisely the one we know may be invented"
  );
  assert_eq!(board.node.focus, "WP-14: the migration verb");
  assert_eq!(board.node.claims, vec!["ST0069/14"]);

  assert!(
    board
      .items
      .iter()
      .all(|i| i.authored_at.is_none() && !i.recorded_at.is_empty()),
    "an item has never claimed a time -- a board stamps its header and its entries -- so giving \
     one the header's heartbeat would invent a per-item stamp out of a per-board one"
  );
  assert!(carried.reconciles());
}

/// The message half: entries land under their sender, in source order, with
/// the heading's claimed stamp verbatim -- and an inbox from a sender the
/// roster does not know is reported by file rather than written.
#[test]
fn messages_land_under_a_registered_sender_and_a_stranger_is_named() {
  let (_fx, facade, carried) = carried();
  let board = facade.board("dc").expect("the board");

  let senders: Vec<&str> = board.messages.iter().map(|m| m.sender.as_str()).collect();
  assert_eq!(
    senders,
    vec!["vc", "vc"],
    "only the registered sender's entries are rows: {senders:?}"
  );
  assert_eq!(
    board.messages[0].authored_at.as_deref(),
    Some("2026-09-12 15:03Z"),
    "the entry's claimed stamp is carried verbatim -- this is the field that deliberately holds \
     the class of value the clock guard refuses"
  );
  assert_eq!(
    board.messages[0].re.as_deref(),
    Some("your 15:01Z -- three rulings")
  );
  assert!(board.messages[1].fyi);
  assert!(
    board
      .messages
      .iter()
      .all(|m| m.state == WbMessageState::Live && m.handled_at.is_none()),
    "the migration marks nothing handled: the bound is a refusal on the next write, not a state \
     the carry gets to declare on its owner's behalf"
  );

  let stranger = carried
    .uncarried
    .iter()
    .find(|u| u.at.ends_with("inbox.laksa-vc.md"))
    .expect("the stranger's inbox is named by file");
  assert!(
    stranger.text.contains("1 entry"),
    "with what it holds, so a reader knows whether anything is at stake: {stranger:?}"
  );
  assert!(
    stranger.reason.contains("laksa-vc"),
    "and why: a message row names its sender, so carrying this would address it from a node the \
     roster does not have: {}",
    stranger.reason
  );
}

/// The document half: a fold is a SNAPSHOT, carried byte for byte and never
/// split into items.
#[test]
fn a_fold_is_carried_as_a_document_and_never_as_items() {
  let (_fx, facade, carried) = carried();
  let sections: Vec<intentsvcs::prose::DocSection> = facade
    .store()
    .doc_sections()
    .expect("sections")
    .into_iter()
    .filter(|s| s.owner_type == intentsvcs::prose::WB_OWNER)
    .collect();

  assert!(
    !sections.is_empty() && sections.iter().all(|s| s.owner_id == "dc"),
    "the snapshot is addressed by the node whose board it was: {sections:?}"
  );
  assert_eq!(
    intentsvcs::prose::join(&sections),
    FOLD,
    "verbatim means verbatim: the file that went in comes back out of its sections byte for byte"
  );

  let board = facade.board("dc").expect("the board");
  assert!(
    !board
      .items
      .iter()
      .any(|i| i.text.contains("before the fold")),
    "and NOTHING from the fold is an item: splitting an archive per item would put every \
     archived line back on the board as live work and manufacture a second history of one node"
  );

  assert!(
    carried
      .snapshots
      .iter()
      .any(|f| f.ends_with("wip-prefold-1400Z.md")),
    "the carried document is named: {:?}",
    carried.snapshots
  );
  let not_a_doc = carried
    .uncarried
    .iter()
    .find(|u| u.at.ends_with("board.png"))
    .expect("a `.history/` file that is not markdown is named rather than passed over");
  assert!(not_a_doc.reason.contains("not markdown"), "{not_a_doc:?}");
}

/// AC-14.9's accounting, against a fixture whose every line is known: each one
/// is either carried with its address or refused by name.
#[test]
fn every_line_the_board_offered_is_on_one_side_or_the_other() {
  let (_fx, _facade, carried) = carried();
  for line in [
    "The busiest section on every real board is prose.",
    "The queued thing.",
    "Held until the fifth kind lands",
    "A caution that outlives the work.",
    "Ruled on 2026-09-12.",
  ] {
    assert!(
      carried.items.iter().any(|i| i.text.contains(line)),
      "carried, with an address: {line}"
    );
  }
  assert!(
    carried
      .uncarried
      .iter()
      .any(|u| u.text.contains("maps to no kind")),
    "and the section no kind maps is NAMED rather than passed over: {:?}",
    carried.uncarried
  );
  assert!(
    carried
      .items
      .iter()
      .all(|i| i.at.starts_with("intent/whiteboard/dc/wip.md:")),
    "every carried item says where it came from, so the two halves are the same kind of record"
  );
  assert!(
    carried.reconciles(),
    "carried {} + {} message(s) + {} snapshot(s) + {} named must account for the {} units read",
    carried.items.len(),
    carried.messages,
    carried.snapshots.len(),
    carried.uncarried.len(),
    carried.offered
  );
}
