//! Deterministic view generation: model -> markdown.
//!
//! **The renderer has no clock, and that is structural rather than
//! disciplinary** (vc law, 2026-08-14). Its inputs are a [`RenderContext`] and
//! the model, full stop: no `SystemTime`, no `$HOSTNAME`, no `$USER`, no
//! locale, no absolute paths, no environment. Nothing in this module can reach
//! one, so "the view is deterministic" is a property of the type signature and
//! not of anybody remembering.
//!
//! The reason is AC-03.4 rather than tidiness. A view that stamps its own
//! render time cannot render the same bytes twice (AC-03.2), and it makes the
//! skew check diff every view on every run -- so the check becomes either
//! useless or trained-to-be-ignored, which is the same outcome arriving later.
//! v2 had three instances, and the third is the one that mattered: the
//! generated-banner footer that data-model.md ratifies as "the AGENTS.md
//! pattern" embedded a render date, so the blessed pattern carried the defect
//! into every view nobody had written yet. Git already records when a view was
//! regenerated, and it does it correctly.
//!
//! Views are 100% generated (D02). No region markers survive the port: v2's
//! `steel_threads.md` wrapped a generated table in `<!-- BEGIN: ... -->` inside
//! an authored file, and its authored half rotted (`stp_version: 1.2.0`, a
//! March 2025 verblock) while the generated half stayed current. That is the
//! mixed-file failure as a measurement rather than a principle.

use std::path::PathBuf;

use crate::contract::{group_of, satisfied_by_tests};
use crate::finding::{Finding, FindingClass};
use crate::ingest::Canon;
use crate::model::{AcState, AcceptanceTest, AtKind, Criterion, Thread, ThreadStatus, WorkPackage};
use crate::project::Project;

/// Everything a render is allowed to depend on besides the model.
///
/// Deliberately tiny. Every field here is a fact about the tool or the
/// project's data -- never about the moment of rendering.
#[derive(Debug, Clone)]
pub struct RenderContext<'a> {
  /// The Intent version, for the generated banner.
  pub version: &'a str,
}

/// One rendered view: where it goes and what it says.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct View {
  pub path: PathBuf,
  pub content: String,
}

/// The v2 display vocabulary (`canonical_status`, `bin/intent_helpers:535`).
///
/// One deliberate divergence: v2 collapsed `TBC` into `Not Started` for
/// display, so a thread whose file said TBC appeared in the index as something
/// else. The model distinguishes them, and reproducing the collapse would be
/// v3 faithfully reproducing a v2 defect -- a `corrected` register row, not a
/// parity break.
fn status_display(status: ThreadStatus) -> &'static str {
  match status {
    ThreadStatus::NotStarted => "Not Started",
    ThreadStatus::Wip => "WIP",
    ThreadStatus::Triage => "Triage",
    ThreadStatus::Hold => "On Hold",
    ThreadStatus::Completed => "Completed",
    ThreadStatus::Cancelled => "Cancelled",
  }
}

fn is_closed(status: ThreadStatus) -> bool {
  matches!(status, ThreadStatus::Completed | ThreadStatus::Cancelled)
}

// ---------------------------------------------------------------------------
// Markdown tables -- one aligner, formatter-stable
// ---------------------------------------------------------------------------

/// How a table is being rendered.
///
/// Both modes live in one function for the reason v2 gives at
/// `bin/intent_helpers:render_table` -- `st list`, `st sync` and `wp list`
/// share it "so the two tables cannot drift apart", and
/// `tests/unit/output_width.bats` pins that by asserting `st list --status
/// all` and `st sync` render byte-identically. Two renderers would satisfy
/// that on the day they were written and not afterwards.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableMode {
  /// On-screen: pipeless (`a | b`, `---|---`), expanded to fill `fill`
  /// columns. `0` means content-fit only.
  Terminal { fill: usize },
  /// A persisted file: canonical GFM, always content-fit.
  ///
  /// It ignores any fill target ON PURPOSE. A markdown file whose column
  /// widths depended on the terminal that happened to generate it would
  /// change bytes every time someone regenerated it at a different window
  /// size, and AC-03.4's skew check would report files nobody had touched.
  Markdown,
}

/// Render a table: on-screen for the CLI, or the way `prettier` would write it
/// for a persisted file.
///
/// **Markdown idempotence has to hold THROUGH the formatter, not merely
/// through the renderer** (vc ruling, 2026-08-14, from a defect ic hit for
/// real). This repository -- and every consumer repository -- runs `prettier
/// --write` over staged markdown in its pre-commit hook. A renderer that
/// emitted narrow separator rows would have its output widened at commit time
/// and narrowed again at the next regeneration, so every generated view would
/// oscillate forever and the skew check (AC-03.4) would report files nobody
/// had touched. A check that cries wolf gets ignored, which is the same
/// failure as a non-deterministic renderer arriving by a different road.
///
/// So in markdown mode: columns padded to the widest cell, minimum three,
/// exactly as prettier pads them. `view_determinism.rs` runs the real
/// formatter over the real output rather than trusting this comment.
pub fn table(headers: &[&str], rows: &[Vec<String>], mode: TableMode) -> String {
  let mut widths: Vec<usize> = headers.iter().map(|h| h.chars().count()).collect();
  for row in rows {
    for (i, c) in row.iter().enumerate() {
      if let Some(w) = widths.get_mut(i) {
        *w = (*w).max(c.chars().count());
      }
    }
  }

  match mode {
    // prettier never writes a separator narrower than `---`, and prettier is
    // a SECOND WRITER of this file that wins any disagreement. v2 has no such
    // floor, which is the one place the two modes genuinely differ -- and it
    // is only visible on a column narrower than three, so on an empty table.
    TableMode::Markdown => {
      for w in &mut widths {
        *w = (*w).max(3);
      }
    }
    // Expand to fill the terminal, sharing the slack out in proportion to
    // each column's content width so the widest column absorbs most of it,
    // with the remainder landing on the last. Content-fit is the FLOOR, so a
    // narrow terminal never truncates -- it just stops padding.
    TableMode::Terminal { fill } => {
      let separators = widths.len().saturating_sub(1) * 3;
      let content: usize = widths.iter().sum();
      if fill > content + separators && content > 0 {
        let slack = fill - content - separators;
        let mut distributed = 0;
        let last = widths.len() - 1;
        // Shares are computed from the CONTENT widths, so an earlier column's
        // expansion cannot change a later column's share.
        let shares = widths.clone();
        for (i, w) in widths.iter_mut().enumerate() {
          let add = if i == last {
            slack - distributed
          } else {
            slack * shares[i] / content
          };
          *w += add;
          distributed += add;
        }
      }
    }
  }

  let header_cells: Vec<String> = headers.iter().map(|h| (*h).to_string()).collect();
  let mut out = row_line(&header_cells, &widths, mode);
  let dashes: Vec<String> = widths.iter().map(|w| "-".repeat(*w)).collect();
  match mode {
    TableMode::Markdown => {
      out.push('|');
      for d in &dashes {
        out.push_str(&format!(" {d} |"));
      }
    }
    TableMode::Terminal { .. } => out.push_str(&dashes.join("-|-")),
  }
  out.push('\n');
  for row in rows {
    out.push_str(&row_line(row, &widths, mode));
  }
  out
}

fn row_line(cells: &[String], widths: &[usize], mode: TableMode) -> String {
  let padded: Vec<String> = widths
    .iter()
    .enumerate()
    .map(|(i, width)| {
      let cell = cells.get(i).map(String::as_str).unwrap_or("");
      format!("{cell:<width$}")
    })
    .collect();
  let mut line = match mode {
    TableMode::Markdown => format!("| {} |", padded.join(" | ")),
    TableMode::Terminal { .. } => padded.join(" | "),
  };
  line.push('\n');
  line
}

/// Make a value safe to put in a table cell.
///
/// A `|` inside a cell breaks the table it is rendered into. v2 hit this in
/// `render_table` and answered it by substituting `|` -> `/`, pinned by
/// `tests/unit/title_pipe_sanitize_guard.bats`; the same answer here, for the
/// same reason, so a view cannot be corrupted by its own content. Newlines
/// collapse for the same reason -- a cell is one line by construction.
fn cell(text: &str) -> String {
  text.replace('|', "/").replace(['\n', '\r'], " ")
}

/// One line of a header block (D13: line-oriented `key: value`, never YAML).
///
/// An absent value emits `key:` with NO trailing space. That is not cosmetic:
/// prettier strips trailing whitespace, so `completed: ` on a thread with no
/// completion date would be rewritten at commit time and regenerated at the
/// next render, oscillating forever. Found by running the real formatter.
///
/// Quotes are deliberately absent too. Under D13 a surrounding quote pair is a
/// display delimiter rather than syntax, and quotes INSIDE a value are literal
/// and never escaped -- so emitting bare values keeps the file honest and
/// sidesteps the escaping trap that made 2-of-5 boards unparseable under the
/// YAML reading.
fn kv(key: &str, value: &str) -> String {
  if value.is_empty() {
    format!("{key}:\n")
  } else {
    format!("{key}: {value}\n")
  }
}

/// Close a view with the generated footer, which names the tool and the source
/// and never the time.
///
/// It normalises the trailing blank lines rather than assuming the caller got
/// them right. Sections end with a varying number of newlines depending on
/// what they contain, and prettier collapses a run of blank lines to one -- so
/// a view whose body happened to end with two would be rewritten at commit
/// time and regenerated back at the next render, oscillating forever. Found by
/// `view_determinism.rs` running the real formatter, not by reading.
fn finish(mut out: String, ctx: &RenderContext<'_>, source: &str) -> String {
  while out.ends_with('\n') {
    out.pop();
  }
  out.push_str(&format!(
    "\n\n---\n\n_Generated by Intent v{} from `{source}`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._\n",
    ctx.version
  ));
  out
}

// ---------------------------------------------------------------------------
// info.md -- a thread's cover
// ---------------------------------------------------------------------------

/// Render one thread's `info.md`.
///
/// 100% generated, which is why `objective`, `context` and `related` are
/// modelled fields rather than authored prose (vc ruling, 2026-08-14): under
/// D02 a file is entirely authored or entirely generated, and v2's info.md was
/// both at once.
pub fn info(thread: &Thread, ctx: &RenderContext<'_>) -> String {
  let mut out = String::new();
  out.push_str("---\n");
  out.push_str(&kv("st_id", &thread.id));
  out.push_str(&kv("title", &thread.title));
  out.push_str(&kv("status", status_display(thread.status)));
  out.push_str(&kv("created", &thread.created));
  out.push_str(&kv("completed", thread.completed.as_deref().unwrap_or("")));
  out.push_str("---\n\n");

  out.push_str(&format!("# {}: {}\n\n", thread.id, thread.title));

  out.push_str("## Objective\n\n");
  out.push_str(&section_body(&thread.objective));

  out.push_str("## Context\n\n");
  out.push_str(&section_body(&thread.context));

  if !thread.wps.is_empty() {
    out.push_str("## Work Packages\n\n");
    let rows: Vec<Vec<String>> = thread
      .wps
      .iter()
      .map(|wp| {
        vec![
          format!("WP-{:02}", wp.seq),
          cell(&wp.title),
          crate::model::enum_str(&wp.scope),
          wp_status_display(wp.status).to_string(),
        ]
      })
      .collect();
    out.push_str(&table(
      &["WP", "Title", "Size", "Status"],
      &rows,
      TableMode::Markdown,
    ));
    out.push('\n');
  }

  out.push_str("## Acceptance\n\n");
  out.push_str(
    "Acceptance Criteria and Acceptance Tests live in `acceptance.md` -- the single source of truth. This cover never restates them.\n\n",
  );

  if !thread.related.is_empty() {
    out.push_str("## Related Steel Threads\n\n");
    for r in &thread.related {
      match &r.note {
        Some(note) => out.push_str(&format!("- {}: {note}\n", r.id)),
        None => out.push_str(&format!("- {}\n", r.id)),
      }
    }
    out.push('\n');
  }

  finish(out, ctx, "thread.json")
}

fn wp_status_display(status: crate::model::WpStatus) -> &'static str {
  match status {
    crate::model::WpStatus::NotStarted => "Not Started",
    crate::model::WpStatus::Wip => "WIP",
    crate::model::WpStatus::Done => "Done",
  }
}

/// A prose block, normalised only in its trailing blank line -- the body
/// itself is carried verbatim, never reflowed.
fn section_body(text: &str) -> String {
  if text.trim().is_empty() {
    return "_(not yet written)_\n\n".to_string();
  }
  format!("{}\n\n", text.trim_end_matches('\n'))
}

// ---------------------------------------------------------------------------
// acceptance.md -- the contract and its coverage map
// ---------------------------------------------------------------------------

const ACCEPTANCE_PREAMBLE: &str = "\
> Canonical acceptance contract. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them.
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Test-backed satisfaction is COMPUTED from covering green ATs and never stored -- storing it would be double truth. An AC has four states, not two: beyond satisfied and unsatisfied, a requirement can be **descoped** to a named thread or **withdrawn** with its reason on the record. Both are non-blocking and both are reported separately, so a thread that descoped half its contract looks like one.
";

/// Render one thread's `acceptance.md`.
pub fn acceptance(thread: &Thread, ctx: &RenderContext<'_>) -> String {
  let mut out = String::new();
  out.push_str("---\n");
  out.push_str(&kv("st_id", &thread.id));
  out.push_str(&kv("title", &thread.title));
  out.push_str("---\n\n");
  out.push_str(&format!(
    "# {}: {} -- Acceptance\n\n",
    thread.id, thread.title
  ));
  out.push_str(ACCEPTANCE_PREAMBLE);
  out.push('\n');

  out.push_str("## Acceptance Criteria\n\n");
  for group in groups(thread) {
    out.push_str(&format!("### {}\n\n", group_heading(thread, &group)));
    let mut any = false;
    for c in thread.criteria.iter().filter(|c| group_of(&c.id) == group) {
      out.push_str(&criterion_line(thread, c));
      any = true;
    }
    if !any {
      out.push_str("_(no criteria in this group)_\n");
    }
    out.push('\n');
  }

  out.push_str("## Acceptance Tests\n\n");
  for group in groups(thread) {
    out.push_str(&format!("### {}\n\n", group_heading(thread, &group)));
    let mut any = false;
    for t in thread.tests.iter().filter(|t| group_of(&t.id) == group) {
      out.push_str(&test_line(t));
      any = true;
    }
    if !any {
      out.push_str("_(no tests in this group)_\n");
    }
    out.push('\n');
  }

  finish(out, ctx, "thread.json")
}

/// Every AC/AT group present, in id order -- `00` (ST-level) first.
fn groups(thread: &Thread) -> Vec<String> {
  let mut seen: Vec<String> = thread
    .criteria
    .iter()
    .map(|c| group_of(&c.id))
    .chain(thread.tests.iter().map(|t| group_of(&t.id)))
    .collect();
  seen.sort();
  seen.dedup();
  seen
}

fn group_heading(thread: &Thread, group: &str) -> String {
  if group == "00" {
    return "ST-level".to_string();
  }
  match group
    .parse::<u32>()
    .ok()
    .and_then(|seq| thread.wps.iter().find(|w| w.seq == seq))
  {
    Some(wp) => format!(
      "WP-{:02} -- {} (status: {})",
      wp.seq,
      wp.title,
      wp_status_display(wp.status)
    ),
    None => format!("Group {group}"),
  }
}

fn criterion_line(thread: &Thread, c: &Criterion) -> String {
  let mut line = format!("- {} ", c.id);
  if c.kind == crate::model::AcKind::NonTest {
    line.push_str("(non-test) ");
  }
  line.push_str(&c.text);
  if let Some(evidence) = c.state.evidence() {
    line.push_str(&format!(" -- evidence: {evidence}"));
  }
  match &c.state {
    AcState::Computed | AcState::Unsatisfied | AcState::Satisfied { .. } => {
      if c.kind == crate::model::AcKind::NonTest {
        line.push_str(&format!(
          " -- satisfied: {}",
          if matches!(c.state, AcState::Satisfied { .. }) {
            "yes"
          } else {
            "no"
          }
        ));
      } else {
        line.push_str(&format!(
          " -- satisfied: {} (computed)",
          if satisfied_by_tests(thread, &c.id) {
            "yes"
          } else {
            "no"
          }
        ));
      }
    }
    AcState::Descoped { to, by, reason } => {
      line.push_str(&format!(" -- DESCOPED to {to}"));
      if let Some(by) = by {
        line.push_str(&format!(" by {by}"));
      }
      if let Some(reason) = reason {
        line.push_str(&format!(": {reason}"));
      }
    }
    AcState::Withdrawn { reason, by } => {
      line.push_str(&format!(" -- WITHDRAWN: {reason}"));
      if let Some(by) = by {
        line.push_str(&format!(" (by {by})"));
      }
    }
  }
  line.push('\n');
  line
}

fn test_line(t: &AcceptanceTest) -> String {
  let mut line = format!("- {} ", t.id);
  match t.kind {
    AtKind::NonTest => {
      line.push_str("(non-test) ");
      line.push_str(t.prose.as_deref().unwrap_or(""));
    }
    AtKind::Test => match (&t.file, &t.legacy) {
      (Some(file), _) => line.push_str(&format!("`{file}`")),
      (None, Some(legacy)) => line.push_str(&format!("(legacy) {}", legacy.raw)),
      (None, None) => line.push_str("(no reference)"),
    },
  }
  line.push_str(&format!(
    " -- covers {} -- status: {}",
    t.covers.join(", "),
    crate::model::enum_str(&t.status)
  ));
  if let Some(note) = &t.note {
    line.push_str(&format!(" -- {note}"));
  }
  if t.legacy.is_some() && t.file.is_some() {
    line.push_str(" -- (carried from v2; raw reference preserved in the model)");
  }
  line.push('\n');
  line
}

// ---------------------------------------------------------------------------
// steel_threads.md -- the thread index
// ---------------------------------------------------------------------------

/// Render the thread index.
///
/// Ordering matches v2's index exactly: in-flight threads first, then closed
/// ones, each block by id descending. Taken from the committed exemplar and
/// confirmed against `update_steel_threads_index` (`bin/intent_st:244`).
/// THE thread ordering: open threads first, then newest id first.
///
/// One function because `intent st list` and the generated index must agree --
/// `tests/unit/output_width.bats` asserts `st list --status all` and `st sync`
/// render byte-identically, and two sorts that happen to match today would
/// satisfy it until one of them changed.
pub fn index_order(threads: &[Thread]) -> Vec<&Thread> {
  let mut ordered: Vec<&Thread> = threads.iter().collect();
  ordered.sort_by(|a, b| {
    is_closed(a.status)
      .cmp(&is_closed(b.status))
      .then_with(|| b.id.cmp(&a.id))
  });
  ordered
}

/// `WP/<NN>/info.md` -- a work package's cover.
///
/// **The last view v3 was missing, and the one D22 never got applied to.**
/// v2's `WP/<NN>/info.md` is the same mixed authored/generated file that D22
/// split at thread level: template sections beside prose a human wrote. D28
/// reified the prose into `thread.json` as `objective` and `body`; this is the
/// other end of that, and without it AC-06.7's canon -> view -> canon has no
/// view to pass through.
///
/// `body` is emitted VERBATIM, and that is the whole point of D28's two-field
/// shape. Real work packages exceed the template freely -- ST0056's own WP-13
/// runs to hundreds of lines with sections the template never named -- so a
/// renderer that re-derived a fixed set of headings would silently drop
/// whatever it did not foresee, which is exactly what WP-10 would have done to
/// them.
///
/// `## Deliverables` is deliberately absent: D28 left it unmodelled, so it
/// arrives inside `body` like any other authored section rather than being
/// invented here from nothing.
pub fn wp_info(thread: &Thread, wp: &WorkPackage, ctx: &RenderContext<'_>) -> String {
  let mut out = String::new();
  out.push_str("---\n");
  out.push_str(&kv("wp_id", &format!("WP-{:02}", wp.seq)));
  out.push_str(&kv("title", &wp.title));
  out.push_str(&kv("scope", &crate::model::enum_str(&wp.scope)));
  out.push_str(&kv("status", wp_status_display(wp.status)));
  out.push_str("---\n\n");

  out.push_str(&format!("# WP-{:02}: {}\n\n", wp.seq, wp.title));

  out.push_str("## Objective\n\n");
  out.push_str(&section_body(&wp.objective));

  if !wp.body.trim().is_empty() {
    out.push_str(wp.body.trim_end());
    out.push_str("\n\n");
  }

  // The acceptance pointer, not the acceptance. v2's template says the same
  // thing and it is load-bearing: ACs live in the thread's `acceptance.md`,
  // and a work package restating them is a second copy that goes stale.
  out.push_str("## Acceptance\n\n");
  out.push_str(&format!(
    "Acceptance Criteria for this work package live in `{}/acceptance.md`, under the `WP-{:02}` heading -- the single source of truth. This cover never restates them.\n\n",
    thread.id, wp.seq
  ));

  finish(out, ctx, "the thread canon")
}

pub fn steel_threads(threads: &[Thread], ctx: &RenderContext<'_>) -> String {
  let ordered = index_order(threads);

  let mut out = String::new();
  out.push_str("# Steel Threads\n\n");
  out.push_str(
    "An index of every steel thread in the project. A steel thread is a self-contained unit of work focused on implementing one piece of functionality.\n\n",
  );
  let rows: Vec<Vec<String>> = ordered
    .iter()
    .map(|t| {
      vec![
        t.id.clone(),
        cell(t.slug.as_deref().unwrap_or("")),
        status_display(t.status).to_string(),
        t.created.clone(),
        t.completed.clone().unwrap_or_default(),
      ]
    })
    .collect();
  out.push_str(&table(
    &["ID", "Slug", "Status", "Created", "Completed"],
    &rows,
    TableMode::Markdown,
  ));
  finish(out, ctx, "the thread canon")
}

// ---------------------------------------------------------------------------
// todo.md -- the flat DOING / TODO / DONE view
// ---------------------------------------------------------------------------

/// One row of the flat work view.
///
/// It carries the parts AND the rendered label rather than letting each
/// consumer re-join them: the markdown view and `intent todo --json` are two
/// renderings of one bucketing, and a second place that builds `"{id}: {title}"`
/// is a second place for the two to disagree about what an item is called.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TodoItem {
  /// `ST0001`, or `ST0001/02` for a work package.
  pub id: String,
  /// `thread` or `work-package`.
  pub kind: &'static str,
  pub title: String,
  /// Exactly what the markdown view prints.
  pub label: String,
}

/// The three buckets.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TodoBuckets {
  pub doing: Vec<TodoItem>,
  pub todo: Vec<TodoItem>,
  pub done: Vec<TodoItem>,
}

/// **THE bucketing.** Both renderings go through it.
///
/// Split out from [`todo`] when `--json` arrived: the alternative was a second
/// traversal applying the same status rules, and the rules are the whole
/// content of this view. Two copies would agree until someone changed one.
pub fn todo_buckets(threads: &[Thread]) -> TodoBuckets {
  let mut ordered: Vec<&Thread> = threads.iter().collect();
  ordered.sort_by(|a, b| a.id.cmp(&b.id));

  let mut doing = Vec::new();
  let mut todo_items = Vec::new();
  let mut done = Vec::new();

  for t in &ordered {
    let item = TodoItem {
      id: t.id.clone(),
      kind: "thread",
      title: t.title.clone(),
      label: format!("{}: {}", t.id, t.title),
    };
    match t.status {
      ThreadStatus::Wip => doing.push(item),
      ThreadStatus::Triage | ThreadStatus::NotStarted | ThreadStatus::Hold => todo_items.push(item),
      ThreadStatus::Completed | ThreadStatus::Cancelled => {
        done.push(item);
      }
    }
    for wp in &t.wps {
      let item = TodoItem {
        id: format!("{}/{:02}", t.id, wp.seq),
        kind: "work-package",
        title: wp.title.clone(),
        label: format!("{} / WP-{:02}: {}", t.id, wp.seq, wp.title),
      };
      match wp.status {
        crate::model::WpStatus::Wip => doing.push(item),
        crate::model::WpStatus::NotStarted if t.status == ThreadStatus::Wip => {
          todo_items.push(item)
        }
        _ => {}
      }
    }
  }

  TodoBuckets {
    doing,
    todo: todo_items,
    done,
  }
}

/// Render the flat work view.
///
/// **DONE is currently every finished thread, and that is a stated gap rather
/// than the finished design.** v2 kept a watermark INSIDE this generated file
/// and read it back out, which made the view its own database -- deleting a
/// disposable file silently resurrected every flushed item. v3 replaced that
/// with an event-derived watermark, and hv then removed the concept entirely
/// (D44): with the whole estate in the database, a DONE bucket is a display
/// WINDOW computed at render time, and there is no durable state behind it to
/// keep anywhere.
///
/// The window itself is not built here yet. It needs a cutoff relative to now,
/// and D42 forbids obtaining a now -- not from the OS, not from the database.
/// The shape that satisfies both is a comparison evaluated INSIDE the query,
/// where SQLite resolves `now` as part of the statement and no caller ever
/// holds a time. Until that lands, this renders every completion rather than
/// silently applying a window nobody ruled on.
pub fn todo(threads: &[Thread], ctx: &RenderContext<'_>) -> String {
  let buckets = todo_buckets(threads);
  let labels =
    |rows: &[TodoItem]| -> Vec<String> { rows.iter().map(|i| i.label.clone()).collect() };

  let mut out = String::new();
  out.push_str("# TODO\n\n");
  out.push_str("A flat DOING / TODO / DONE view, projected from steel-thread and work-package status. Generated -- change a status with the CLI, never by editing this file.\n\n");
  out.push_str(&bucket("DOING", &labels(&buckets.doing)));
  out.push_str(&bucket("TODO", &labels(&buckets.todo)));
  out.push_str("## DONE\n\n");
  out.push_str(&items(&labels(&buckets.done)));
  finish(out, ctx, "the thread canon")
}

fn bucket(name: &str, entries: &[String]) -> String {
  format!("## {name}\n\n{}", items(entries))
}

fn items(entries: &[String]) -> String {
  if entries.is_empty() {
    return "_(none)_\n\n".to_string();
  }
  let mut out = String::new();
  for entry in entries {
    out.push_str(&format!("- [ ] {entry}\n"));
  }
  out.push('\n');
  out
}

// ---------------------------------------------------------------------------
// Rendering the whole estate, and the skew check
// ---------------------------------------------------------------------------

/// Every view the model implies, in a stable order.
pub fn render_all(project: &Project, canon: &Canon, ctx: &RenderContext<'_>) -> Vec<View> {
  let mut views = Vec::new();
  for thread in &canon.threads {
    views.push(View {
      path: project.info_view(&thread.id),
      content: info(thread, ctx),
    });
    views.push(View {
      path: project.acceptance_view(&thread.id),
      content: acceptance(thread, ctx),
    });
    for wp in &thread.wps {
      views.push(View {
        path: project.wp_info_view(&thread.id, wp.seq),
        content: wp_info(thread, wp, ctx),
      });
    }
  }
  views.push(View {
    path: project.steel_threads_view(),
    content: steel_threads(&canon.threads, ctx),
  });
  views.push(View {
    path: project.todo_view(),
    content: todo(&canon.threads, ctx),
  });
  views
}

/// Write every view to disk.
pub fn write_all(
  project: &Project,
  canon: &Canon,
  ctx: &RenderContext<'_>,
) -> Result<Vec<View>, std::io::Error> {
  let views = render_all(project, canon, ctx);
  for view in &views {
    if let Some(parent) = view.path.parent() {
      std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&view.path, &view.content)?;
  }
  Ok(views)
}

/// The skew check (AC-03.4): regenerate every view and name any that differs
/// from what is on disk.
///
/// A hand-edited view is CAUGHT, never silently outvoted. The distinction
/// matters: `write_all` would overwrite the edit without a word, and the
/// person who made it would have no way to know their change was discarded.
/// `doctor` runs this; the migrator runs it before it converges.
pub fn skew(project: &Project, canon: &Canon, ctx: &RenderContext<'_>) -> Vec<Finding> {
  let mut findings = Vec::new();
  for view in render_all(project, canon, ctx) {
    let rel = project.relative(&view.path);
    match std::fs::read_to_string(&view.path) {
      Ok(on_disk) if on_disk == view.content => {}
      Ok(on_disk) => findings.push(Finding::new(
        &rel,
        FindingClass::ViewSkew,
        format!(
          "generated view differs from the model ({} bytes on disk, {} rendered); the file was edited by hand or written by an older version -- regenerate to discard the edit, or make the change through the CLI so it lands in the model",
          on_disk.len(),
          view.content.len()
        ),
      )),
      Err(_) => findings.push(Finding::new(
        &rel,
        FindingClass::ViewSkew,
        "generated view is missing; regenerate it",
      )),
    }
  }
  findings
}
