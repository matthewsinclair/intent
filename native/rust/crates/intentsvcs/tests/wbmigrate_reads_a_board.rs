//! AT-14.9 / AC-14.9 -- ST0069 WP-14: a hand-authored board reaches the model
//! with nothing dropped silently.
//!
//! **THE FIXTURE IS THE SHAPE REAL BOARDS ACTUALLY HAVE, and that is the whole
//! reason these arms exist rather than a parser unit test.** Every live board on
//! this estate writes DOING as bold prose paragraphs and TODO as bullets, mixes
//! both in one file, and carries a `## Holds` section the model has no kind for.
//! A reader built for bullets alone would carry nothing from the busiest section
//! of every board and reconcile perfectly against zero -- the failure shape this
//! project has met repeatedly, where an instrument agrees with itself about a
//! population it never saw.

use intentsvcs::model::WbItemKind;
use intentsvcs::wbmigrate;

const BOARD: &str = r#"---
node: dc
name: DevX Claude
role: worker
session_id: b9e78c72
heartbeat_at: 2026-09-12 18:31Z
status: active
focus: "WP-14, and the header block is not YAML: this value has quotes in it"
claims: [ST0069/02, ST0069/14]
---

# DevX Claude (dc)

A lead paragraph that belongs to the document rather than to any item.

## DOING -- WP-14, with trailing prose in the heading

**The busiest section on every real board is prose, not bullets.**
It runs to more than one line.

A second paragraph is a second item.

## TODO

- The first bullet.
- The second bullet, which
  continues onto another line.

## Holds

- Held until the fifth kind lands, which is the condition that releases it.

## Watch-outs

- A caution that outlives the work.

## Decisions

- Ruled on 2026-09-12, and recorded here.

## Standing directives

- A section the protocol names for one node and the model maps to no kind.
"#;

#[test]
fn the_header_is_read_as_key_value_and_never_as_yaml() {
  let board = wbmigrate::read_board("dc", BOARD, "intent/whiteboard/dc/wip.md");
  assert_eq!(board.name, "DevX Claude");
  assert_eq!(board.role, "worker");
  assert_eq!(
    board.focus, "WP-14, and the header block is not YAML: this value has quotes in it",
    "one pair of surrounding quotes is stripped and the colon inside survives -- \
     a YAML reader would have refused this line, which is why the protocol is not YAML"
  );
  assert_eq!(board.claims, vec!["ST0069/02", "ST0069/14"]);
  assert_eq!(
    board.authored_at.as_deref(),
    Some("2026-09-12 18:31Z"),
    "the board's own stamp is carried verbatim and UNTRUSTED -- the service \
     stamps recorded_at itself"
  );
}

/// **The discriminating case: prose items and bullet items in one board.**
#[test]
fn prose_sections_and_bullet_sections_both_carry() {
  let board = wbmigrate::read_board("dc", BOARD, "intent/whiteboard/dc/wip.md");
  let doing: Vec<&str> = board
    .items
    .iter()
    .filter(|i| i.kind == WbItemKind::Doing)
    .map(|i| i.text.as_str())
    .collect();
  assert_eq!(
    doing.len(),
    2,
    "a prose DOING section yields one item per paragraph, not zero: {doing:?}"
  );
  assert!(
    doing[0].contains("busiest section") && doing[0].contains("more than one line"),
    "a paragraph's continuation lines stay with it: {doing:?}"
  );

  let todo: Vec<&str> = board
    .items
    .iter()
    .filter(|i| i.kind == WbItemKind::Todo)
    .map(|i| i.text.as_str())
    .collect();
  assert_eq!(todo.len(), 2, "one item per TOP-LEVEL bullet: {todo:?}");
  assert!(
    todo[1].contains("continues onto another line"),
    "a bullet's continuation line stays with its bullet: {todo:?}"
  );

  assert!(
    !board
      .items
      .iter()
      .any(|i| i.text.contains("lead paragraph")),
    "prose above the first `## ` belongs to the document, not to an item"
  );
}

/// AC-14.9's own words: a hold carries AS A HOLD, and what no kind maps is
/// NAMED per item rather than passed over.
#[test]
fn a_hold_carries_as_a_hold_and_an_unmapped_section_is_named() {
  let board = wbmigrate::read_board("dc", BOARD, "intent/whiteboard/dc/wip.md");
  let holds: Vec<&str> = board
    .items
    .iter()
    .filter(|i| i.kind == WbItemKind::Hold)
    .map(|i| i.text.as_str())
    .collect();
  assert_eq!(
    holds.len(),
    1,
    "the fifth kind landed, so the section the protocol calls load-bearing carries as itself \
     rather than being refused: {holds:?}"
  );
  assert!(
    holds[0].contains("condition that releases it"),
    "and it carries the CONDITION, which is the field that makes it a hold: {holds:?}"
  );

  // **THE REFUSAL ARM NOW POINTS AT WHERE THE LOSS ACTUALLY IS.** Two things on
  // this board have no kind: the lead paragraph above the first section, and a
  // section the protocol names for one node and the model maps to nothing. Both
  // are named; neither is passed over, which is what the count means.
  let named: Vec<&str> = board.uncarried.iter().map(|u| u.text.as_str()).collect();
  assert_eq!(named.len(), 2, "{:?}", board.uncarried);
  assert!(
    named.iter().any(|t| t.contains("lead paragraph")),
    "prose above the first `## ` is named rather than carried into a kind: {named:?}"
  );
  let unmapped = board
    .uncarried
    .iter()
    .find(|u| u.text.contains("maps to no kind"))
    .expect("the unmapped section is named");
  assert!(
    unmapped.at.starts_with("intent/whiteboard/dc/wip.md:"),
    "and it is named where it was found: {}",
    unmapped.at
  );
  assert!(
    unmapped.reason.contains("Standing directives"),
    "the reason names the section, so the reader knows which one to decide about: {}",
    unmapped.reason
  );
  assert!(
    board.reconciles(),
    "carried {} + named {} must equal the {} the source offered",
    board.items.len(),
    board.uncarried.len(),
    board.source_items
  );
}

/// The inbox half: entries are the `## (...)` headings and nothing else.
#[test]
fn an_inbox_carries_its_entries_and_not_its_scaffolding() {
  const INBOX: &str = r#"# inbox: vc -> dc

_(empty)_

A line somebody typed above the first entry, belonging to no message.

## (2026-09-12 15:03Z) Re: your 15:01Z -- three rulings

**Nothing is with me.** All three are ruled.

## (2026-09-12 18:15Z) FYI only -- no response needed.

**BROADCAST: the live store is at schema 24.**
"#;
  let read = wbmigrate::read_inbox("vc", "dc", INBOX, "intent/whiteboard/dc/inbox.vc.md");
  let messages = read.messages;
  assert_eq!(
    messages.len(),
    2,
    "the routing header and the `_(empty)_` sentinel are scaffolding, not entries: {messages:?}"
  );
  assert_eq!(
    read.uncarried.len(),
    1,
    "and a line above the first entry that is NEITHER of those is named rather than assumed \
     away: {:?}",
    read.uncarried
  );
  assert!(
    read.uncarried[0].text.contains("belonging to no message")
      && read.uncarried[0]
        .at
        .starts_with("intent/whiteboard/dc/inbox.vc.md:"),
    "{:?}",
    read.uncarried
  );
  assert_eq!(
    messages[0].authored_at.as_deref(),
    Some("2026-09-12 15:03Z"),
    "the entry's claimed stamp is carried verbatim and untrusted"
  );
  assert_eq!(
    messages[0].re.as_deref(),
    Some("your 15:01Z -- three rulings"),
    "the Re: field is read from the field, never from the spacing around it"
  );
  assert!(!messages[0].fyi);
  assert!(
    messages[0].body.contains("Nothing is with me"),
    "the body is everything under the heading: {:?}",
    messages[0].body
  );
  assert!(
    messages[1].fyi && messages[1].re.is_none(),
    "FYI is read as a field too: {:?}",
    messages[1]
  );
}
