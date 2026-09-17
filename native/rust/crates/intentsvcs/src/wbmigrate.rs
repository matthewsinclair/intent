//! Carry a hand-authored whiteboard board into the model (ST0069 WP-14,
//! AC-14.9).
//!
//! **SEPARATE FROM `legacy.rs` BECAUSE THE SUBJECT IS DIFFERENT**: that module
//! reads a v2 ESTATE, this one reads the protocol's own board form. What it
//! takes from there whole is the accounting -- carried and uncarried reported
//! PER ITEM, never as a total, because **a count that reconciles arithmetically
//! tells nobody which line went**, and a drop with no record is
//! indistinguishable from a line that was never there.
//!
//! **EVERY STAMP THE MARKDOWN CLAIMS IS UNTRUSTED.** It lands verbatim in
//! `authored_at` while the service writes `recorded_at` from the clock at the
//! write (D33, AC-14.11). This is the one field in the model that deliberately
//! carries the class of value the clock guard exists to refuse -- a board's
//! `## (...)` headings have been measured fabricated, an hour out, and ordered
//! before the message they answer -- so it is typed as text and never read as a
//! time.

use crate::model::{WbItemKind, WbNodeStatus};

/// A board line the migration could not carry, named where it was found.
///
/// **THE REASON IS REQUIRED**, because the two causes want opposite actions: a
/// section the model has no kind for waits on the model, and a malformed header
/// waits on the board's author.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Uncarried {
  /// `<file>:<line>`, so the worklist is walkable.
  pub at: String,
  /// The text as it stands, verbatim.
  pub text: String,
  pub reason: String,
}

/// One item read off a board, before it reaches the store.
///
/// **IT CARRIES ITS ADDRESS FOR THE SAME REASON [`Uncarried`] DOES**: the
/// accounting is per item on both sides, and a carried half reported as a
/// number against a named uncarried half is the arithmetic reconciliation that
/// tells nobody which line went where.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceItem {
  /// `<file>:<line>`, so the worklist is walkable.
  pub at: String,
  pub kind: WbItemKind,
  pub text: String,
  /// A paragraph of section prose carried as one item rather than a bullet.
  ///
  /// **CARRIED AND SAID, NEVER CARRIED QUIETLY** (vc decision 20, issue 0404).
  /// A section's prose is how every live board writes DOING, so refusing it
  /// would refuse the busiest section of every board; but a sentence such as
  /// `Nothing.` becomes an item the model counts as work, and the operator is
  /// the only one who can say which paragraphs those are. The report names each
  /// on a `coerced:` line.
  pub coerced: bool,
}

/// One inbox entry read off disk.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceMessage {
  pub sender: String,
  pub recipient: String,
  pub body: String,
  pub re: Option<String>,
  pub fyi: bool,
  /// The `## (...)` heading's stamp, verbatim and untrusted.
  pub authored_at: Option<String>,
}

/// One inbox, read but not yet written: its entries, and every line above them
/// that belongs to no entry.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SourceInbox {
  pub messages: Vec<SourceMessage>,
  pub uncarried: Vec<Uncarried>,
}

/// One node's board, read but not yet written.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SourceBoard {
  pub moniker: String,
  pub name: String,
  pub role: String,
  pub session_id: Option<String>,
  pub status: Option<WbNodeStatus>,
  pub focus: String,
  pub claims: Vec<String>,
  /// The header's own `heartbeat_at:`, verbatim and untrusted.
  pub authored_at: Option<String>,
  pub items: Vec<SourceItem>,
  pub uncarried: Vec<Uncarried>,
  /// Every item-shaped line the source offered, carried or not. **The
  /// denominator of AC-14.9's reconciliation**, counted where the lines are
  /// dispatched rather than re-derived afterwards: a second walk would be a
  /// second reader of the same file, free to disagree with this one.
  pub source_items: usize,
}

impl SourceBoard {
  /// AC-14.9's reconciliation: nothing left without being named.
  pub fn reconciles(&self) -> bool {
    self.items.len() + self.uncarried.len() == self.source_items
  }
}

/// The board's item sections, and the kind each maps to.
///
/// **`## Holds` MAPS TO `WbItemKind::Hold`, WHICH IS WHY THE SECTION SURVIVES
/// THE ROUND TRIP.** This reader refused every hold by name for as long as the
/// model had four kinds, because a board carried through a model with no holds
/// comes back without the one section the protocol calls load-bearing -- the
/// item is not the content, the CONDITION is, and a hold with no condition is
/// indistinguishable from work that was quietly dropped. The fifth kind landed
/// (vc's ruling of 2026-09-12, built by cc) and the refusal arm is now pointed
/// at the sections nothing maps, which is where the remaining loss lives.
///
/// **`## Standing directives` MAPS TO `WbItemKind::Directive` ON EVERY BOARD,
/// AND WHOSE BOARD MAY CARRY ONE IS THE DOOR'S QUESTION** (issue 0375). A reader
/// that dropped the section off a board that is not `hv`'s would be the silent
/// loss this function exists to end; `Facade::wb_migrate` refuses that carry by
/// name before it writes.
fn kind_of(heading: &str) -> Option<WbItemKind> {
  kind_and_qualifier(heading).map(|(kind, _)| kind)
}

/// A section heading's kind, and whatever text it carries after the kind's own
/// word.
///
/// **THE QUALIFIER IS RETURNED RATHER THAN DISCARDED** (vc decision 20, issue
/// 0407). A section renders under the heading its kind names and the model has
/// no field for anything after it, so `## TODO -- one upstream, the rest
/// downstream` came back as `## TODO` with nothing said. A trailing colon is
/// punctuation on the kind word and is not a qualifier.
fn kind_and_qualifier(heading: &str) -> Option<(WbItemKind, Option<String>)> {
  // Matched on the heading's leading word rather than the whole line, because
  // a board's headings carry trailing prose -- `## DOING -- WP-02` is one of
  // this estate's own -- and an equality test would silently classify every
  // such section as unknown while looking perfectly correct.
  let original = heading.trim_start_matches('#').trim();
  let head = original.to_ascii_lowercase();
  let qualifier = |from: usize| -> Option<String> {
    // Lowercasing ASCII keeps every byte offset, so `from` indexes both.
    let rest = original[from..].trim_start_matches(':').trim();
    (!rest.is_empty()).then(|| rest.to_string())
  };
  // The one section named by two words, so it is matched before the leading one.
  for words in ["standing directives", "standing directive"] {
    if head.starts_with(words) {
      return Some((WbItemKind::Directive, qualifier(words.len())));
    }
  }
  let first = head.split_whitespace().next().unwrap_or_default();
  let kind = match first.trim_end_matches(':') {
    "doing" => WbItemKind::Doing,
    "todo" => WbItemKind::Todo,
    "decisions" | "decision" => WbItemKind::Decision,
    "watch-outs" | "watch-out" | "watchouts" => WbItemKind::Watchout,
    "holds" | "hold" => WbItemKind::Hold,
    _ => return None,
  };
  Some((kind, qualifier(first.len())))
}

/// Read one node's board file.
///
/// **THE HEADER IS READ THE WAY `Facade::register_roster` READS IT** -- a
/// line-oriented `key: value` block, one pair of surrounding double quotes
/// stripped, quotes inside left alone -- because the protocol ruled that format
/// after measuring that nodes writing prose-heavy `focus:` values cannot write
/// valid YAML: two of five boards were unparseable at a point in time, and all
/// of them repaired themselves before anyone noticed.
pub fn read_board(moniker: &str, wip_md: &str, file: &str) -> SourceBoard {
  // **THE RENDERER'S BANNER IS NOT A UNIT, FOR THE REASON `views::EMPTY_ITEMS`
  // IS NOT** (issue 0439): a registered node's board is a view, and read whole
  // its closing rule and sentence carried as a coerced item. The body is a
  // prefix of the file, so every `<file>:<line>` still addresses the file.
  let wip_md = crate::views::view_body(wip_md).unwrap_or(wip_md);
  let mut out = SourceBoard {
    moniker: moniker.to_string(),
    ..Default::default()
  };
  let header_end = wip_md
    .lines()
    .enumerate()
    .skip(1)
    .find(|(_, l)| l.trim() == "---")
    .map(|(i, _)| i)
    .unwrap_or(0);
  let field = |key: &str| -> Option<String> {
    wip_md
      .lines()
      .take(header_end + 1)
      .find_map(|line| line.strip_prefix(&format!("{key}: ")))
      .map(|v| {
        let v = v.trim();
        v.strip_prefix('"')
          .and_then(|r| r.strip_suffix('"'))
          .unwrap_or(v)
          .to_string()
      })
  };
  out.name = field("name").unwrap_or_default();
  out.role = field("role").unwrap_or_default();
  out.focus = field("focus").unwrap_or_default();
  // **`none` IS THE HUMAN'S NODE AND NOT A MISSING VALUE.** The protocol says
  // hv carries `session_id: none` because it runs no session loop, so reading
  // it as an id would put the literal string in the model.
  out.session_id = field("session_id").filter(|v| v != "none" && !v.is_empty());
  out.authored_at = field("heartbeat_at");
  out.status = match field("status").as_deref() {
    Some("paused") => Some(WbNodeStatus::Paused),
    Some("active") => Some(WbNodeStatus::Active),
    _ => None,
  };
  let claims: Vec<String> = field("claims")
    .map(|v| {
      v.trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|c| c.trim().to_string())
        .filter(|c| !c.is_empty())
        .collect()
    })
    .unwrap_or_default();
  // **A CLAIM `wb claim` WOULD REFUSE IS NOT CARRIED** (vc, ruled 2026-09-14).
  // The verb admits only an address the board can point at, so carrying any
  // other spelling would put in the column a value no command could have
  // written. It is named with the form the verb takes, and counted, so the
  // reconciliation still accounts for every unit the header offered.
  // Issue 0383: claims were carried verbatim, and `ST0112/WP-07` reached a board `wb claim` refuses.
  let claims_line = wip_md
    .lines()
    .take(header_end + 1)
    .position(|l| l.starts_with("claims: "))
    .map_or(1, |i| i + 1);
  for claim in claims {
    if crate::model::is_claim_address(&claim) {
      out.claims.push(claim);
      continue;
    }
    let spelled = claim
      .split_once("/WP-")
      .map(|(thread, seq)| format!("{thread}/{seq}"))
      .filter(|address| crate::model::is_claim_address(address))
      .map(|address| format!(" -- `{address}` here"))
      .unwrap_or_default();
    out.source_items += 1;
    out.uncarried.push(Uncarried {
      at: format!("{file}:{claims_line}"),
      reason: format!(
        "not a claim address: `wb claim` takes a thread as `ST0000` or a work package as \
         `ST0000/01`{spelled}"
      ),
      text: claim,
    });
  }

  // The body, section by section. Blocks are separated by blank lines; a block
  // opening with `- ` yields one item per TOP-LEVEL bullet, and any other block
  // is one item carrying its text verbatim.
  //
  // **A NON-BULLET BLOCK IS AN ITEM RATHER THAN A LOSS, and that is measured
  // rather than assumed**: every live board on this estate writes its DOING
  // section as bold prose paragraphs, not bullets, so a bullets-only reader
  // would carry nothing from the busiest section of every board and reconcile
  // perfectly against zero.
  let mut heading: Option<String> = None;
  let mut block: Vec<(usize, String)> = Vec::new();
  let flush =
    |heading: &Option<String>, block: &mut Vec<(usize, String)>, out: &mut SourceBoard| {
      if block.is_empty() {
        return;
      }
      let taken = std::mem::take(block);
      // **A SECTION THE RENDERER WROTE EMPTY IS EMPTY, NOT AN ITEM.** A board
      // section with no live items renders as `views::EMPTY_ITEMS` on its own,
      // and without this the block is carried as one item reading it --
      // measured at WP-14's cutover on hv's board, and waiting on every board
      // that folds a section to empty. **It is not counted either**: the
      // renderer's statement that nothing is there is not a unit the source
      // offered, exactly as the inbox reader treats `views::EMPTY_INBOX`.
      if taken.len() == 1 && taken[0].1.trim() == crate::views::EMPTY_ITEMS {
        return;
      }
      // **A TABLE IS REFUSED WHOLE, WITH THE BLOCK THAT HOLDS IT** (vc decision
      // 20, issue 0406). An item is a list entry, so a table carried as one
      // renders as a bullet whose first line is the header row and whose other
      // rows are continuation lines: the table is gone while every byte of it is
      // still in the store. **ANY `|`-LED LINE, NOT ONLY THE FIRST**: a table
      // under a prose line with no blank line between is one block, and testing
      // the opening line alone carried it as one coerced prose item.
      if taken.iter().any(|(_, l)| l.trim_start().starts_with('|')) {
        out.source_items += 1;
        out.uncarried.push(Uncarried {
          at: format!("{file}:{}", taken[0].0),
          text: taken
            .iter()
            .map(|(_, l)| l.as_str())
            .collect::<Vec<_>>()
            .join("\n"),
          reason: "a table, with any line in its block: an item is one list entry, so its \
                   rows would render as the continuation lines of a bullet and the table would \
                   no longer be one"
            .to_string(),
        });
        return;
      }
      for (line_no, text, bullet) in blocks_to_items(&taken) {
        // **COUNTED HERE, WHERE THE LINE IS DISPATCHED, WHATEVER BECOMES OF IT.**
        // A section nothing maps used to return before this line, so its content
        // never entered the denominator and the reconciliation was a claim about
        // the sections the reader already understood. Every board on this estate
        // carries sections the protocol does not name -- the human's node is
        // mostly such sections -- so that is where the remaining loss is, and it
        // is now refused by name in the same worklist as everything else.
        out.source_items += 1;
        match heading.as_deref().and_then(kind_of) {
          Some(kind) => out.items.push(SourceItem {
            at: format!("{file}:{line_no}"),
            kind,
            text,
            coerced: !bullet,
          }),
          None => out.uncarried.push(Uncarried {
            at: format!("{file}:{line_no}"),
            text,
            reason: match heading {
              Some(head) => format!(
                "`{}` is not one of the protocol's item sections, so no `WbItemKind` carries it",
                head.trim()
              ),
              None => "above the first `## ` section: a board's lead paragraph belongs to the \
                     document rather than to any item, and the model has no field for it"
                .to_string(),
            },
          }),
        }
      }
    };
  for (i, line) in wip_md.lines().enumerate().skip(header_end + 1) {
    if line.starts_with("## ") {
      flush(&heading, &mut block, &mut out);
      if let Some((_, Some(qualifier))) = kind_and_qualifier(line) {
        out.source_items += 1;
        out.uncarried.push(Uncarried {
          at: format!("{file}:{}", i + 1),
          text: line.to_string(),
          reason: format!(
            "`{qualifier}` follows the section's kind word, and the section renders under its \
             kind's own heading: the model has no field for the rest"
          ),
        });
      }
      heading = Some(line.to_string());
      continue;
    }
    // **A SUB-HEADING IS A UNIT OF ITS OWN AND NEVER PART OF AN ITEM** (vc
    // decision 20, issue 0403). It records whose work the lines under it are,
    // and an item's `seq` carries no group, so carried inside a block it became
    // an item reading `###` and the grouping survived only as an order the next
    // archive can break. It ends the block above it, as a section heading does.
    if line.starts_with("###") {
      flush(&heading, &mut block, &mut out);
      out.source_items += 1;
      out.uncarried.push(Uncarried {
        at: format!("{file}:{}", i + 1),
        text: line.to_string(),
        reason: "a sub-heading: an item has no field for the group it names, so the grouping \
                 would survive only as order within a kind"
          .to_string(),
      });
      continue;
    }
    // The board's own `# <Name> (<node>)` title, which the header block already
    // carries as data and a renderer writes back from `name` and `moniker`. It
    // ends whatever section preceded it, so a `#` title mid-document cannot
    // silently adopt the section above it.
    if line.starts_with("# ") {
      flush(&heading, &mut block, &mut out);
      heading = None;
      continue;
    }
    if line.trim().is_empty() {
      flush(&heading, &mut block, &mut out);
      continue;
    }
    block.push((i + 1, line.to_string()));
  }
  flush(&heading, &mut block, &mut out);
  out
}

/// Split one blank-line-delimited block into items.
///
/// A block opening with `- ` is a bullet list: one item per top-level bullet,
/// with its continuation lines attached. Anything else is one item, verbatim,
/// and the third field says which of the two it was.
fn blocks_to_items(block: &[(usize, String)]) -> Vec<(usize, String, bool)> {
  let opens_a_list = block
    .first()
    .is_some_and(|(_, l)| l.trim_start().starts_with("- "));
  if !opens_a_list {
    let line_no = block.first().map(|(n, _)| *n).unwrap_or(0);
    let text = block
      .iter()
      .map(|(_, l)| l.as_str())
      .collect::<Vec<_>>()
      .join("\n");
    return vec![(line_no, text, false)];
  }
  let mut out: Vec<(usize, String, bool)> = Vec::new();
  for (line_no, line) in block {
    if line.starts_with("- ") {
      out.push((*line_no, line.trim_start_matches("- ").to_string(), true));
    } else if let Some((_, last, _)) = out.last_mut() {
      last.push('\n');
      last.push_str(line);
    } else {
      out.push((*line_no, line.clone(), true));
    }
  }
  out
}

/// Carry one `.history/` file as a verbatim snapshot document.
///
/// **A FOLD'S ARCHIVE IS A SNAPSHOT OF A BOARD AT A MOMENT, NEVER A SOURCE OF
/// ITEMS** (hv, ruled 2026-09-12). Splitting one into `WbItem`s would
/// manufacture a second, competing history of the same node -- every archived
/// DOING line back on the board as live work, and every later count ambiguous
/// about which history it measured. It goes through the ordinary prose splitter,
/// so the bytes that went in come out identical: [`crate::prose::join`] of what
/// this returns is the file.
///
/// The sections are addressed by the NODE rather than by the file alone, so a
/// search that hits one can say whose board it was.
pub fn snapshot_sections(node: &str, file: &str, text: &str) -> Vec<crate::prose::DocSection> {
  crate::prose::split(crate::prose::WB_OWNER, node, file, text)
}

/// Read one inbox file into its messages.
///
/// **ENTRIES ARE THE `## (...)` HEADINGS AND NOTHING ELSE.** The `# inbox:
/// <sender> -> <recipient>` line restates the routing the path already encodes,
/// and `_(empty)_` is the no-live-entries sentinel that keeps an inbox from
/// being an ambiguous zero-byte file; neither is a message.
/// **AND THE PROLOGUE IS ACCOUNTED FOR RATHER THAN ASSUMED EMPTY.** Anything
/// above the first entry that is neither of those two lines is prose somebody
/// wrote into an inbox by hand, and it is named in [`SourceInbox::uncarried`] --
/// the same rule the board reader holds to, because "the file only ever has a
/// header" is exactly the class of assumption that drops content quietly.
pub fn read_inbox(sender: &str, recipient: &str, text: &str, file: &str) -> SourceInbox {
  // **NOR IS AN INBOX VIEW'S BANNER** (issue 0439): above the first entry it
  // was refused as a line belonging to no message, and below the last it would
  // have been read into that message's body.
  let text = crate::views::view_body(text).unwrap_or(text);
  let mut out: Vec<SourceMessage> = Vec::new();
  let mut uncarried: Vec<Uncarried> = Vec::new();
  let mut body: Vec<&str> = Vec::new();
  let mut pending: Option<SourceMessage> = None;
  let finish =
    |pending: &mut Option<SourceMessage>, body: &mut Vec<&str>, out: &mut Vec<SourceMessage>| {
      if let Some(mut message) = pending.take() {
        // The heading's own prose, when it had any, leads the body.
        let lead = std::mem::take(&mut message.body);
        let below = body.join("\n").trim().to_string();
        message.body = match (lead.is_empty(), below.is_empty()) {
          (_, true) => lead,
          (true, false) => below,
          (false, false) => format!("{lead}\n\n{below}"),
        };
        out.push(message);
      }
      body.clear();
    };
  for (i, line) in text.lines().enumerate() {
    if let Some(rest) = line.strip_prefix("## (") {
      finish(&mut pending, &mut body, &mut out);
      let (stamp, tail) = match rest.split_once(')') {
        Some((stamp, tail)) => (stamp.trim().to_string(), tail),
        None => (rest.trim().to_string(), ""),
      };
      // **THE SEPARATOR BETWEEN THE FIELDS IS NOT SIGNIFICANT** -- the protocol
      // documented three spaces for a year while the formatter collapsed every
      // one of them, so 39 headings carried the fields and none retained the
      // spacing. Matched on the field, never on the gap.
      let re = tail.split_once("Re: ").map(|(_, r)| {
        r.split("FYI only")
          .next()
          .unwrap_or(r)
          .trim()
          .trim_end_matches('.')
          .to_string()
      });
      pending = Some(SourceMessage {
        sender: sender.to_string(),
        recipient: recipient.to_string(),
        // Prose in the heading that is neither field leads the body.
        // Issue 0384: it was discarded, so an entry written in its heading carried with an empty body.
        body: heading_prose(tail),
        re,
        fyi: tail.contains("FYI only"),
        authored_at: Some(stamp),
      });
      continue;
    }
    if pending.is_some() {
      body.push(line);
      continue;
    }
    // The prologue: the routing header the path already encodes, and the
    // no-live-entries sentinel that keeps an inbox from being an ambiguous
    // zero-byte file. Neither is a message; anything else here is.
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed == crate::views::EMPTY_INBOX || trimmed.starts_with("# inbox:")
    {
      continue;
    }
    uncarried.push(Uncarried {
      at: format!("{file}:{}", i + 1),
      text: line.to_string(),
      reason: "above the first `## (...)` entry, where an inbox carries only its routing header \
               and the empty sentinel: this line belongs to no message"
        .to_string(),
    });
  }
  finish(&mut pending, &mut body, &mut out);
  SourceInbox {
    messages: out,
    uncarried,
  }
}

/// The prose an inbox heading carries after its stamp that is neither the
/// `Re:` field nor the FYI marker.
///
/// **CARRIED, NEVER DROPPED.** Protocol headings carry only those two fields,
/// but a hand-authored inbox is what it is, and text read off a heading and
/// thrown away leaves no record that it was ever there.
fn heading_prose(tail: &str) -> String {
  const FYI: &str = "FYI only -- no response needed.";
  let cut = [tail.find("Re: "), tail.find("FYI only")]
    .into_iter()
    .flatten()
    .min()
    .unwrap_or(tail.len());
  let mut parts = vec![tail[..cut].trim()];
  if !tail.contains("Re: ")
    && let Some((_, after)) = tail.split_once(FYI)
  {
    parts.push(after.trim());
  }
  parts.retain(|p| !p.is_empty());
  parts.join(" ")
}
