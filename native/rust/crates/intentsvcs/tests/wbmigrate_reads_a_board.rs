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

- A directive in force, which the model carries as the sixth kind.

## Parking lot

- A section the protocol does not name, which the model maps to no kind.
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

  // **PROSE IS CARRIED AND MARKED, BULLETS ARE CARRIED PLAIN** (vc decision 20,
  // issue 0404): the mark is what puts a paragraph on a `coerced:` line.
  assert!(
    board
      .items
      .iter()
      .all(|i| i.coerced == (i.kind == WbItemKind::Doing)),
    "only the prose DOING paragraphs are coerced: {:?}",
    board.items
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

  // **THE REFUSAL ARM NOW POINTS AT WHERE THE LOSS ACTUALLY IS.** Three things
  // on this board have no field: the lead paragraph above the first section, the
  // text after DOING's kind word, and a section the protocol does not name and
  // the model maps to nothing. All are named; none is passed over, which is what
  // the count means.
  let named: Vec<&str> = board.uncarried.iter().map(|u| u.text.as_str()).collect();
  assert_eq!(named.len(), 3, "{:?}", board.uncarried);
  assert!(
    named.iter().any(|t| t.contains("lead paragraph")),
    "prose above the first `## ` is named rather than carried into a kind: {named:?}"
  );
  let qualifier = board
    .uncarried
    .iter()
    .find(|u| u.text.starts_with("## DOING"))
    .expect("a heading's text after its kind word is named (vc decision 20, issue 0407)");
  assert!(
    qualifier
      .reason
      .contains("WP-14, with trailing prose in the heading"),
    "the reason quotes the words that would be lost: {}",
    qualifier.reason
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
    unmapped.reason.contains("Parking lot"),
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

/// Issue 0375: `## Standing directives` carries as the sixth kind. The reader
/// maps it on every board; whose board may carry one is the migration door's
/// question, and `wb_migrate_carries_a_board.rs` holds that refusal.
#[test]
fn a_standing_directive_carries_as_a_directive() {
  let board = wbmigrate::read_board("hv", BOARD, "intent/whiteboard/hv/wip.md");
  let directives: Vec<&str> = board
    .items
    .iter()
    .filter(|i| i.kind == WbItemKind::Directive)
    .map(|i| i.text.as_str())
    .collect();
  assert_eq!(
    directives,
    vec!["A directive in force, which the model carries as the sixth kind."],
    "the section's lines carry as directives rather than being named uncarried: {:?}",
    board.uncarried
  );
  assert!(board.reconciles());
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

/// **A BOARD THE RENDERER WROTE READS BACK AS THE BOARD IT WAS, EMPTY SECTIONS
/// INCLUDED.** Every section is emitted and one with no live items carries the
/// empty sentinel, which the reader carried as an item until WP-14's cutover put
/// two of them on hv's board. The board is rendered by the real renderer rather
/// than typed here, because the defect was the two sides DISAGREEING: a
/// hand-typed sentinel would prove only that the reader knows one spelling.
#[test]
fn a_rendered_board_reads_back_without_carrying_its_empty_sections() {
  let board = intentsvcs::model::Board {
    schema: intentsvcs::model::BOARD_SCHEMA.to_string(),
    node: intentsvcs::model::WbNode {
      moniker: "dc".to_string(),
      name: "DevX Claude".to_string(),
      role: "worker".to_string(),
      session_id: None,
      heartbeat_at: "2026-09-13T10:24:37.071Z".to_string(),
      status: intentsvcs::model::WbNodeStatus::Active,
      focus: "one live todo and four empty sections".to_string(),
      claims: Vec::new(),
      recorded_at: "2026-09-13T10:24:37.071Z".to_string(),
      authored_at: None,
      migrated_at: Some("2026-09-13T10:24:37.071Z".to_string()),
    },
    items: vec![intentsvcs::model::WbItem {
      node: "dc".to_string(),
      kind: WbItemKind::Todo,
      seq: 1,
      text: "The only live item on this board.".to_string(),
      state: intentsvcs::model::WbItemState::Live,
      archived_at: None,
      recorded_at: "2026-09-13T10:24:37.071Z".to_string(),
      authored_at: None,
      edited_at: None,
    }],
    messages: Vec::new(),
  };
  let rendered = intentsvcs::views::wb_board_body(&board);
  assert!(
    rendered.contains(intentsvcs::views::EMPTY_ITEMS),
    "the fixture must render an empty section, or this arm proves nothing: {rendered}"
  );

  let read = wbmigrate::read_board("dc", &rendered, "intent/whiteboard/dc/wip.md");
  let carried: Vec<(WbItemKind, &str)> = read
    .items
    .iter()
    .map(|i| (i.kind, i.text.as_str()))
    .collect();
  assert_eq!(
    carried,
    vec![(WbItemKind::Todo, "The only live item on this board.")],
    "the four empty sections carry nothing"
  );
  assert!(read.uncarried.is_empty(), "{:?}", read.uncarried);
  assert_eq!(
    read.source_items, 1,
    "and the sentinel is not a unit the source offered"
  );
}

#[test]
fn a_claim_the_verb_would_refuse_is_named_with_the_form_it_takes() {
  // Issue 0383: claims were carried verbatim, including forms `wb claim` refuses.
  let board = BOARD.replace(
    "claims: [ST0069/02, ST0069/14]",
    "claims: [ST0069/02, ST0112/WP-07, the whole thread]",
  );
  let read = wbmigrate::read_board("dc", &board, "intent/whiteboard/dc/wip.md");
  assert_eq!(
    read.claims,
    vec!["ST0069/02"],
    "only an address the verb takes is carried"
  );
  let refused: Vec<&wbmigrate::Uncarried> = read
    .uncarried
    .iter()
    .filter(|u| u.reason.starts_with("not a claim address"))
    .collect();
  assert_eq!(refused.len(), 2, "each refused claim is named: {refused:?}");
  assert!(
    refused
      .iter()
      .any(|u| u.text == "ST0112/WP-07" && u.reason.contains("`ST0112/07` here"))
      && refused
        .iter()
        .all(|u| u.at == "intent/whiteboard/dc/wip.md:9"),
    "with the address the verb takes, where the header said it: {refused:?}"
  );
  assert!(
    read.reconciles(),
    "and counted, so the reconciliation holds"
  );
}

#[test]
fn an_entry_written_in_its_heading_carries_that_text_as_its_body() {
  // Issue 0384: heading text that was neither `Re:` nor the FYI marker was discarded, so the entry carried with an empty body.
  const INBOX: &str = "# inbox: vc -> dc\n\n## (2026-09-14 10:00Z) the whole message sat up here\n\n## (2026-09-14 10:05Z) a lead in the heading\n\nand the rest below it\n";
  let read = wbmigrate::read_inbox("vc", "dc", INBOX, "intent/whiteboard/dc/inbox.vc.md");
  let bodies: Vec<&str> = read.messages.iter().map(|m| m.body.as_str()).collect();
  assert_eq!(
    bodies,
    vec![
      "the whole message sat up here",
      "a lead in the heading\n\nand the rest below it"
    ],
    "text in the heading is carried, never dropped"
  );
}

/// vc decision 20, issues 0403 and 0406: a `###` sub-heading and a table are
/// each one unit the model cannot carry, named where they stand, and neither
/// becomes part of an item. The lines under a sub-heading still carry as the
/// section's kind.
#[test]
fn a_sub_heading_and_a_table_are_named_units_and_never_items() {
  const GROUPED: &str = "---\nnode: dc\nname: DevX Claude\nrole: worker\n---\n\n## TODO\n\n\
### cc's lane\n- The first bullet under a group.\n\n\
| lane | item |\n| ---- | ---- |\n| cc   | 0410 |\n\n- A bullet after the table.\n\n\
The lanes as they stand:\n| lane | item |\n| ---- | ---- |\n";
  let board = wbmigrate::read_board("dc", GROUPED, "intent/whiteboard/dc/wip.md");

  let todo: Vec<&str> = board.items.iter().map(|i| i.text.as_str()).collect();
  assert_eq!(
    todo,
    vec![
      "The first bullet under a group.",
      "A bullet after the table."
    ],
    "neither the sub-heading nor any table row reaches an item: {todo:?}"
  );

  let sub = board
    .uncarried
    .iter()
    .find(|u| u.text == "### cc's lane")
    .expect("the sub-heading is named");
  assert_eq!(sub.at, "intent/whiteboard/dc/wip.md:9");
  assert!(sub.reason.contains("sub-heading"), "{}", sub.reason);

  let table = board
    .uncarried
    .iter()
    .find(|u| u.text.starts_with("| lane"))
    .expect("the table is named");
  assert_eq!(table.at, "intent/whiteboard/dc/wip.md:12");
  assert_eq!(
    table.text.lines().count(),
    3,
    "the table is ONE unit, every row of it: {:?}",
    table.text
  );
  // A table under a prose line with no blank line between is one block, and it
  // is refused whole rather than carried as one coerced prose item.
  let under_prose = board
    .uncarried
    .iter()
    .find(|u| u.text.starts_with("The lanes as they stand:"))
    .expect("a table opened by prose is named too");
  assert_eq!(under_prose.at, "intent/whiteboard/dc/wip.md:18");
  assert!(
    !board
      .items
      .iter()
      .any(|i| i.text.contains("lanes as they stand")),
    "and none of that block reaches an item"
  );
  assert_eq!(board.uncarried.len(), 3, "{:?}", board.uncarried);
  assert!(board.reconciles());
}

/// **`0488`: A NUMBERED LIST CARRIES ONE ITEM PER LINE, AS BULLETS ALWAYS DID.**
///
/// Before this, an ordered line fell into the "anything else is one item,
/// verbatim" branch: ic's four-item numbered TODO arrived as one todo and hv's
/// two unexecuted rulings, written `1.` and `2.`, arrived as one decision with
/// an embedded `2.` -- neither archivable without the other. The board's author
/// got one of two answers depending on a choice of punctuation that nothing
/// told them was load bearing.
///
/// The mixed block is in the same arm on purpose: the opener decides the whole
/// block, so a `- ` line inside a numbered list is a continuation rather than a
/// new item, and an author who mixes them gets one list rather than two
/// interleaved ones.
#[test]
fn a_numbered_list_carries_one_item_per_line_and_a_mixed_block_follows_its_opener() {
  const BOARD: &str = r#"---
node: ic
name: Interface Claude
role: interface
session_id: none
heartbeat_at: 2026-09-19 12:00Z
status: active
focus: "numbered lists"
claims: []
---
# Interface Claude (ic)

## TODO

1. the first numbered thing
2. the second numbered thing
3) the third, with a paren delimiter
10. the tenth, to prove more than one digit

## Decisions

1. an ordered opener
- a bullet inside it, which is a continuation and not a new item

## Watch-outs

- a bullet opener
1. a numbered line inside it, which is a continuation too
"#;

  let board = wbmigrate::read_board("ic", BOARD, "intent/whiteboard/ic/wip.md");

  let todos: Vec<&str> = board
    .items
    .iter()
    .filter(|i| i.kind == WbItemKind::Todo)
    .map(|i| i.text.as_str())
    .collect();
  assert_eq!(
    todos,
    vec![
      "the first numbered thing",
      "the second numbered thing",
      "the third, with a paren delimiter",
      "the tenth, to prove more than one digit",
    ],
    "a numbered list must carry one item per line, with its marker stripped"
  );
  assert!(
    board
      .items
      .iter()
      .filter(|i| i.kind == WbItemKind::Todo)
      .all(|i| !i.coerced),
    "an ordered line is a list entry, so it is not coerced prose"
  );

  let decisions: Vec<&str> = board
    .items
    .iter()
    .filter(|i| i.kind == WbItemKind::Decision)
    .map(|i| i.text.as_str())
    .collect();
  assert_eq!(
    decisions,
    vec!["an ordered opener\n- a bullet inside it, which is a continuation and not a new item"],
    "an ordered block's bullet line is a continuation: the opener decides the block"
  );

  let watchouts: Vec<&str> = board
    .items
    .iter()
    .filter(|i| i.kind == WbItemKind::Watchout)
    .map(|i| i.text.as_str())
    .collect();
  assert_eq!(
    watchouts,
    vec!["a bullet opener\n1. a numbered line inside it, which is a continuation too"],
    "and the same rule the other way round"
  );
  assert!(board.reconciles());
}

/// **`0489`: THE ONE CLASSIFIER BEHIND BOTH REPORT LINES.**
///
/// Most sub-headings are dates and safe to drop. The one reading
/// `### Still live from 2026-08-19 -- the two rulings that are NOT executed`
/// was not, and it was named in the uncarried list identically to the dates, so
/// a node dropping them under `--drop-uncarried` had no signal. The renderer
/// asks this function for the `uncarried:` line and for the `coerced:` line, so
/// the two cannot disagree about what looks state bearing.
///
/// **NOT DRIVEN HERE: that the renderer prints the mark.** Those are two
/// one-line suffixes in `render.rs` over this verdict, and a CLI fixture for a
/// report line is more machinery than the mark is worth while the seven carries
/// are waiting. What is driven is the verdict every such line is taken from.
#[test]
fn a_lead_that_carries_a_state_is_marked_and_a_date_is_not() {
  use intentsvcs::wbmigrate::reads_as_state_bearing;

  assert!(reads_as_state_bearing(
    "### Still live from 2026-08-19 -- the two rulings that are NOT executed"
  ));
  assert!(reads_as_state_bearing(
    "Everything below is held behind hv's fences until the cut"
  ));
  assert!(reads_as_state_bearing("This work is blocked on the daemon"));

  assert!(!reads_as_state_bearing("### 2026-08-19"));
  assert!(!reads_as_state_bearing("### cc's lane"));
  assert!(
    !reads_as_state_bearing("The lanes as they stand:"),
    "a decorative lead must not be marked, or the mark means nothing"
  );
}
