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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceItem {
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
  pub messages: Vec<SourceMessage>,
  /// `.history/` files, carried as verbatim snapshot documents and never split
  /// into items (vc, ruled 2026-09-12).
  pub snapshots: Vec<(String, String)>,
  pub uncarried: Vec<Uncarried>,
  /// Every item-shaped line the source offered, carried or not. **The
  /// denominator of AC-14.9's reconciliation**, counted where the lines are
  /// dispatched rather than re-derived afterwards: a second walk would be a
  /// second reader of the same file, free to disagree with this one.
  pub source_items: usize,
  pub source_messages: usize,
}

impl SourceBoard {
  /// AC-14.9's reconciliation: nothing left without being named.
  pub fn reconciles(&self) -> bool {
    self.items.len() + self.uncarried.len() == self.source_items
  }
}

/// The board's item sections, and the kind each maps to.
///
/// **`## Holds` IS DELIBERATELY ABSENT AND IS DELIBERATELY NOT A DROP.**
/// `WbItemKind` has four variants and the protocol's board has five sections;
/// the fifth kind is cc's to add (vc, ruled 2026-09-12) and the migration
/// rebases onto it. Until then every hold is REFUSED BY NAME through
/// [`Uncarried`] rather than skipped -- which is the difference between a
/// section waiting for a model and a section nobody can prove was ever there.
/// The skill is emphatic about which section this is: the item is not the
/// content, the CONDITION is, and a hold survives a fold precisely while its
/// condition stands unmet.
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
    _ => None,
  }
}

/// Whether a heading is the held `## Holds` section.
fn is_holds(heading: &str) -> bool {
  let head = heading.trim_start_matches('#').trim().to_ascii_lowercase();
  head
    .split_whitespace()
    .next()
    .unwrap_or_default()
    .trim_end_matches(':')
    == "holds"
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
  let flush = |heading: &Option<String>,
               block: &mut Vec<(usize, String)>,
               out: &mut SourceBoard| {
    if block.is_empty() {
      return;
    }
    let taken = std::mem::take(block);
    let Some(head) = heading else {
      // Above the first `## `: the board's own title and its lead paragraph,
      // which belong to the document rather than to any item.
      return;
    };
    let held = is_holds(head);
    let kind = kind_of(head);
    if kind.is_none() && !held {
      return; // Not an item section at all.
    }
    for (line_no, text) in blocks_to_items(&taken) {
      out.source_items += 1;
      match (held, kind) {
        (true, _) => out.uncarried.push(Uncarried {
          at: format!("{file}:{line_no}"),
          text,
          reason: "`## Holds` has no `WbItemKind` yet, so this line is refused rather than carried into a kind it does not belong to. The fifth variant is ruled and pending; re-run the migration once it lands".to_string(),
        }),
        (false, Some(kind)) => out.items.push(SourceItem { kind, text }),
        (false, None) => unreachable!("guarded above"),
      }
    }
  };
  for (i, line) in wip_md.lines().enumerate().skip(header_end + 1) {
    if line.starts_with("## ") {
      flush(&heading, &mut block, &mut out);
      heading = Some(line.to_string());
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

/// Read one inbox file into its messages.
///
/// **ENTRIES ARE THE `## (...)` HEADINGS AND NOTHING ELSE.** The `# inbox:
/// <sender> -> <recipient>` line restates the routing the path already encodes,
/// and `_(empty)_` is the no-live-entries sentinel that keeps an inbox from
/// being an ambiguous zero-byte file; neither is a message.
pub fn read_inbox(sender: &str, recipient: &str, text: &str) -> Vec<SourceMessage> {
  let mut out: Vec<SourceMessage> = Vec::new();
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
  for line in text.lines() {
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
    }
  }
  finish(&mut pending, &mut body, &mut out);
  out
}
