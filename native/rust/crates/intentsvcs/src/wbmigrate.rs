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
fn kind_of(heading: &str) -> Option<WbItemKind> {
  // Matched on the heading's leading word rather than the whole line, because
  // a board's headings carry trailing prose -- `## DOING -- WP-02` is one of
  // this estate's own -- and an equality test would silently classify every
  // such section as unknown while looking perfectly correct.
  let head = heading.trim_start_matches('#').trim().to_ascii_lowercase();
  let first = head.split_whitespace().next().unwrap_or_default();
  match first.trim_end_matches(':') {
    "doing" => Some(WbItemKind::Doing),
    "todo" => Some(WbItemKind::Todo),
    "decisions" | "decision" => Some(WbItemKind::Decision),
    "watch-outs" | "watch-out" | "watchouts" => Some(WbItemKind::Watchout),
    "holds" | "hold" => Some(WbItemKind::Hold),
    _ => None,
  }
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
  out.claims = field("claims")
    .map(|v| {
      v.trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|c| c.trim().to_string())
        .filter(|c| !c.is_empty())
        .collect()
    })
    .unwrap_or_default();

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
      for (line_no, text) in blocks_to_items(&taken) {
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
      heading = Some(line.to_string());
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
/// with its continuation lines attached. Anything else is one item, verbatim.
fn blocks_to_items(block: &[(usize, String)]) -> Vec<(usize, String)> {
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
    return vec![(line_no, text)];
  }
  let mut out: Vec<(usize, String)> = Vec::new();
  for (line_no, line) in block {
    if line.starts_with("- ") {
      out.push((*line_no, line.trim_start_matches("- ").to_string()));
    } else if let Some((_, last)) = out.last_mut() {
      last.push('\n');
      last.push_str(line);
    } else {
      out.push((*line_no, line.clone()));
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
  let mut out: Vec<SourceMessage> = Vec::new();
  let mut uncarried: Vec<Uncarried> = Vec::new();
  let mut body: Vec<&str> = Vec::new();
  let mut pending: Option<SourceMessage> = None;
  let finish =
    |pending: &mut Option<SourceMessage>, body: &mut Vec<&str>, out: &mut Vec<SourceMessage>| {
      if let Some(mut message) = pending.take() {
        message.body = body.join("\n").trim().to_string();
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
        body: String::new(),
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
