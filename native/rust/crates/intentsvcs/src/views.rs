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
use crate::project::{Project, canon_thread_rel};
use crate::write_set::WriteSet;

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
  /// On-screen: pipeless (`a | b`, `---|---`), rendered to `fill` columns
  /// exactly. `0` means content-fit only.
  ///
  /// **`fill` USED TO BE A FLOOR AND IS NOW A TARGET, AND THAT IS A DECLARED
  /// DEVIATION FROM v2.** Both implementations carried the same rule in the
  /// same words -- v2's `render_table` says *content-fit is the floor, so
  /// nothing is ever truncated* and this said *a narrow terminal never
  /// truncates, it just stops padding*. The consequence neither comment states
  /// is that ONE oversized cell sets the width of EVERY row: measured
  /// 2026-08-25, `issues list` rendered 312 columns into an 80-column terminal
  /// because a single title ran to 287 characters. **A width that only ever
  /// grows is not a width.** hv ruled truncation with an ellipsis.
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
      // **THE SHRINK PASS, AND IT IS THE HALF THAT WAS MISSING.** Take the
      // overflow off the WIDEST column each time round, so a table blown out by
      // one runaway cell loses width where the width actually is rather than
      // spreading the loss over columns that were already the right size.
      //
      // Every column floors at its HEADER width: a header is the one part of a
      // table that must stay readable, since a clipped header makes the column
      // unidentifiable and the row below it unreadable rather than merely
      // shortened. If the headers alone cannot fit, the table overflows -- an
      // honest overflow, because there is no narrower correct answer.
      if fill > 0 && content + separators > fill {
        let floors: Vec<usize> = headers.iter().map(|h| h.chars().count().max(1)).collect();
        let mut total = content + separators;
        while total > fill {
          let Some((i, _)) = widths
            .iter()
            .enumerate()
            .filter(|(i, w)| **w > floors[*i])
            .max_by_key(|(_, w)| **w)
          else {
            break;
          };
          widths[i] -= 1;
          total -= 1;
        }
      }
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
      // Clipped by CHARACTERS, never by bytes -- a byte slice through a
      // multi-byte character produces broken output, and `views` already
      // measures every width with `chars().count()` for the same reason.
      // The ellipsis is one character and three bytes, which is exactly the
      // trap a byte-based `truncate` would fall into here.
      let len = cell.chars().count();
      if len > *width {
        let keep = width.saturating_sub(1);
        let clipped: String = cell.chars().take(keep).collect();
        return format!("{clipped}\u{2026}");
      }
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
  out.push_str(&kv("status", thread.status.display()));
  // **Only when there is one, unlike `completed` below.** The field exists only
  // after a guarded transition, so emitting it always would put an empty key on
  // every thread in the estate to say nothing. Conditional also means adding it
  // churns no committed view today: nothing currently carries a reason.
  if let Some(reason) = &thread.status_reason {
    out.push_str(&kv("status_reason", reason));
  }
  out.push_str(&kv("created", &thread.created));
  out.push_str(&kv("completed", thread.completed.as_deref().unwrap_or("")));
  out.push_str("---\n\n");

  out.push_str(&format!("# {}: {}\n\n", thread.id, thread.title));

  // **ABOVE the first generated section, which is the whole reason this is its
  // own field rather than part of `body`.** `body` renders below `## Context`,
  // so a preamble carried there returns beneath two headings it was written
  // above -- bytes preserved, position moved, and a silent MOVE is harder to
  // see than a silent drop. Here the region goes back exactly where its author
  // put it: after the title, before the first heading.
  //
  // Stored stripped, so the renderer re-emits the layout. That is the trade the
  // contract rules explicitly: the blank lines are markdown, not content.
  if !thread.preamble.is_empty() {
    out.push_str(&thread.preamble);
    out.push_str("\n\n");
  }

  out.push_str("## Objective\n\n");
  out.push_str(&section_body(&thread.objective));

  out.push_str("## Context\n\n");
  out.push_str(&section_body(&thread.context));

  // Every other authored section, verbatim and in authored order. Without this
  // slot the field is ingested and dropped on the first projection, which is
  // the same silent loss D28 exists to prevent one level down.
  //
  // **Placed after Context rather than interleaved**, because the model holds
  // two named sections and one opaque block: a thread that authored `## Scope`
  // BETWEEN Objective and Context has it relocated below both. That is a known
  // and accepted relocation -- `wp_info` has made the same trade since D28 --
  // and it is the price of a catch-all that cannot lose a heading nobody
  // foresaw. Reordering is visible and recoverable; dropping is neither.
  if !thread.body.trim().is_empty() {
    out.push_str(thread.body.trim_end());
    out.push_str("\n\n");
  }

  if !thread.wps.is_empty() && !carries_heading(&thread.body, "Work Packages") {
    out.push_str("## Work Packages\n\n");
    let rows: Vec<Vec<String>> = thread
      .wps
      .iter()
      .map(|wp| {
        vec![
          format!("WP-{:02}", wp.seq),
          cell(&wp.title),
          wp.scope_display(),
          wp.status.display().to_string(),
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

  // **An authored section wins and the generated default defers**, the same
  // rule `wp_info` carries and for the same reason: the author's copy states
  // their project's own convention, and the generated one asserts ours.
  //
  // Measured on this estate at the moment the catch-all landed: `Work Packages`
  // doubles on 8 threads (v2 never generated it, so none is template-identical
  // and all 8 carry), `Acceptance` on none (12 of 12 are template-identical and
  // drop), and `Related Steel Threads` on none TODAY only because `related` is
  // empty -- **52 threads carry one, so this deferral is a precondition of
  // parsing `related` rather than a companion to it** (vc's ruling). Landing
  // that field alone would run a renderer path never once exercised on a
  // migrated estate and double 52 threads in the same commit.
  if !carries_heading(&thread.body, "Acceptance") {
    out.push_str("## Acceptance\n\n");
    out.push_str(&format!(
      "Acceptance Criteria and Acceptance Tests are RENDERED into `acceptance.md`, which is a GENERATED VIEW -- a row authored there is discarded by the next sync. The contract is canon in this thread's model: change a state with the `intent ac` / `intent at` verbs, and mint or reword a row in `{}`, then `intent sync --to-store`. This cover never restates them.\n\n",
      canon_thread_rel(&thread.id)
    ));
  }

  if !thread.related.is_empty() && !carries_heading(&thread.body, "Related Steel Threads") {
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

/// Does this carried body already carry `## <heading>` of its own?
///
/// **Exact, line-anchored, and deliberately not a `contains`.** The bodies
/// this is asked about are authored markdown containing fenced code, tables
/// and quoted prose, so a substring test would defer to the word "Acceptance"
/// appearing inside a sentence -- and the failure would be silent, because the
/// generated section simply would not appear.
///
/// One home for it: the same collision exists at thread level the moment
/// `Thread.body` lands, and two private copies of "does the author already
/// have this heading" is the drift the Highlander rule exists to stop.
pub(crate) fn carries_heading(body: &str, heading: &str) -> bool {
  body
    .lines()
    .any(|l| l.strip_prefix("## ").is_some_and(|h| h.trim() == heading))
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
> **THIS FILE IS A GENERATED VIEW, AND A ROW AUTHORED HERE IS DISCARDED BY THE NEXT SYNC.** The acceptance contract is canon in the thread model; this file renders it. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them.
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
      wp.status.display()
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
  // **`display()`, not `enum_str`, and this one was a live migration hazard**
  // (issue 0056). The wire form spells `Na` as `n-a`; every authored row in this
  // estate says `n/a`, and none says `n-a`. So the next
  // projection over a thread with a non-test AT would have rewritten each of those
  // rows into a spelling v2's own linter rejects at L1. A generated view is
  // supposed to reproduce the authored form, not introduce a second one.
  line.push_str(&format!(
    " -- covers {} -- status: {}",
    t.covers.join(", "),
    t.status.display()
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
    a.status
      .is_closed()
      .cmp(&b.status.is_closed())
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
  out.push_str(&kv("scope", &wp.scope_display()));
  out.push_str(&kv("status", wp.status.display()));
  // Same rule as the thread view one level up: present only when set.
  if let Some(reason) = &wp.status_reason {
    out.push_str(&kv("status_reason", reason));
  }
  out.push_str("---\n\n");

  out.push_str(&format!("# WP-{:02}: {}\n\n", wp.seq, wp.title));

  // Same slot one level down -- above the first generated heading, where its
  // author wrote it. 5 of the canary's 20 regions are work-package ones.
  if !wp.preamble.is_empty() {
    out.push_str(&wp.preamble);
    out.push_str("\n\n");
  }

  out.push_str("## Objective\n\n");
  out.push_str(&section_body(&wp.objective));

  if !wp.body.trim().is_empty() {
    out.push_str(wp.body.trim_end());
    out.push_str("\n\n");
  }

  // The acceptance pointer, not the acceptance. v2's template says the same
  // thing and it is load-bearing: ACs live in the thread's `acceptance.md`,
  // and a work package restating them is a second copy that goes stale.
  //
  // **AN AUTHORED ONE WINS, AND THE DEFAULT DEFERS TO IT.** `body` is a
  // catch-all, so a work package whose author wrote their own `## Acceptance`
  // renders two of them -- which shipped, on 40 views here and 104 across the
  // fleet. `27c4ec98` closed the SCAFFOLDING half by dropping sections nobody
  // wrote; **this is the other half, and they were always two problems.** On
  // Baize the fix took 53 doubled views to 20, and the 20 survivors are
  // authored prose that the drop rule is right to leave alone (vc measured it
  // on the estate that did NOT find the defect, which is why it was the
  // estate to verify on).
  //
  // **The deciding argument is not duplication, it is that the generated line
  // can be FALSE.** It asserts the criteria sit "under the `WP-NN` heading",
  // and `AC-NN.M`'s major number is an AC GROUP ordinal on some projects and a
  // work-package number on others -- vc's D47 ruling, after doctor had
  // silently picked one reading. Baize's own authored pointer says `AC-01`,
  // which is correct there and is precisely what the generated one contradicts.
  // **So the collision was the tool asserting a convention over the author who
  // had already stated the right one**, and deferring keeps the true copy.
  //
  // Dropping was never available for these: nobody can say "no author wrote
  // it" about them, and a project-specific fact would go with them. Renaming
  // the generated heading was the other option and is worse -- it changes
  // every view in the fleet to avoid a collision on a minority, and
  // `Acceptance` is the right name for what it is.
  if !carries_heading(&wp.body, "Acceptance") {
    out.push_str("## Acceptance\n\n");
    out.push_str(&format!(
      "Acceptance Criteria for this work package are RENDERED into `{}/acceptance.md`, under the `WP-{:02}` heading. THAT FILE IS A GENERATED VIEW -- a row authored there is discarded by the next sync. The contract is canon in the thread's model: change a state with the `intent ac` / `intent at` verbs, and mint or reword a row in `{}`, then `intent sync --to-store`. This cover never restates them.\n\n",
      thread.id,
      wp.seq,
      canon_thread_rel(&thread.id)
    ));
  }

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
        t.status.display().to_string(),
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
///
/// **It did not carry the status, and that was the whole defect.** The bucket a
/// row lands in is three-valued and the status is six-valued, so bucketing
/// alone DESTROYS the distinction between the states that share a bucket --
/// `Completed` and `Cancelled` both land in DONE. The markdown renderer then
/// had nothing to compute a glyph from and emitted a constant; `--json` had
/// nothing to report and omitted it. **Both faces lost the same fact, so both
/// agreed, which is exactly the agreement this struct exists to guarantee** --
/// the doc above promises the two renderings cannot disagree about what an item
/// is called, and they did not: they were both wrong in the same way.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TodoItem {
  /// `ST0001`, or `ST0001/02` for a work package.
  pub id: String,
  /// `thread` or `work-package`.
  pub kind: &'static str,
  pub title: String,
  /// The state as a human reads it, from
  /// [`ThreadStatus::display`](crate::model::ThreadStatus::display) or
  /// [`WpStatus::display`](crate::model::WpStatus::display). **Carried because
  /// the bucket cannot be read back into it:** DONE means completed-or-cancelled
  /// and a `--json` consumer cannot recover which.
  pub status: &'static str,
  /// The checkbox glyph, from
  /// [`ThreadStatus::glyph`](crate::model::ThreadStatus::glyph) or
  /// [`WpStatus::glyph`](crate::model::WpStatus::glyph). Computed where the
  /// status is still in scope, which is the only place it can be computed.
  pub glyph: char,
  /// Exactly what the markdown view prints after the glyph.
  pub label: String,
  /// The work packages belonging to this thread, rendered INDENTED beneath it.
  ///
  /// **A thread and its work packages are a TREE, not a flat sibling list**
  /// (hv, 2026-08-18). Flat rows repeated the thread id on every line, split
  /// one thread's packages across DOING and TODO by their own status, and gave
  /// no way to see a thread's shape at a glance. Nesting says it once.
  ///
  /// **Every work package appears, whatever its status** -- a `done` package
  /// under a `wip` thread is exactly the progress a reader is looking for, and
  /// the old view dropped it. The BUCKET is chosen by the THREAD's status
  /// alone; a package's own status is carried by its glyph.
  ///
  /// Always empty for a work package: the tree is two levels deep because the
  /// model is.
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub children: Vec<TodoItem>,
}

/// The three buckets.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TodoBuckets {
  pub doing: Vec<TodoItem>,
  pub todo: Vec<TodoItem>,
  pub done: Vec<TodoItem>,
}

/// How much of DONE a rendering shows (D44).
///
/// **An id allowlist, not a cutoff.** The cutoff lives in SQL, where `now` is
/// resolved inside the comparison that consumes it and nothing holds a time
/// (D42); by the time a renderer is involved the question is already answered.
/// Passing hours down here instead would put a clock-relative decision in a
/// pure function, which is where it could not be made honestly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TodoWindow {
  /// Every completion. **This is what the committed file gets**, always.
  All,
  /// Only these thread ids, as answered by
  /// [`crate::store::Store::threads_completed_within`].
  Only(std::collections::BTreeSet<String>),
}

impl TodoWindow {
  /// Whether a DONE row survives. **Work packages are matched by their parent
  /// thread**, whose id prefixes theirs -- `completed` is a thread-level fact
  /// and a WP has no completion date of its own to window on.
  pub fn shows(&self, id: &str) -> bool {
    match self {
      TodoWindow::All => true,
      TodoWindow::Only(ids) => {
        let thread = id.split_once('/').map_or(id, |(t, _)| t);
        ids.contains(thread)
      }
    }
  }
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
    // **The whole roster, in sequence order, whatever each one's status.** The
    // flat view showed a package only when it was `wip`, or `not-started`
    // under a `wip` thread, and dropped every `done` one -- so a thread's
    // finished work was invisible in the view whose job is showing progress.
    let children: Vec<TodoItem> = t
      .wps
      .iter()
      .map(|wp| TodoItem {
        id: format!("{}/{:02}", t.id, wp.seq),
        kind: "work-package",
        title: wp.title.clone(),
        status: wp.status.display(),
        glyph: wp.status.glyph(),
        // **The thread id is NOT repeated.** The row is already nested under
        // it, and saying it twice is what made the flat view unreadable.
        label: format!("{:02}: {}", wp.seq, wp.title),
        children: Vec::new(),
      })
      .collect();

    let item = TodoItem {
      id: t.id.clone(),
      kind: "thread",
      title: t.title.clone(),
      status: t.status.display(),
      glyph: t.status.glyph(),
      label: format!("{}: {}", t.id, t.title),
      children,
    };
    // **The THREAD's status alone chooses the bucket**, and it carries its
    // packages with it. A package cannot be filed apart from its thread, which
    // is what let one thread appear in two buckets at once.
    match t.status {
      ThreadStatus::Wip => doing.push(item),
      ThreadStatus::Triage | ThreadStatus::NotStarted | ThreadStatus::Hold => todo_items.push(item),
      ThreadStatus::Completed | ThreadStatus::Cancelled => {
        done.push(item);
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
/// **The window is a PARAMETER of this one generator, never a second
/// renderer.** vc ruled the surface (2026-08-16, under hv's standing "go with
/// your recs", raised because hv ruled the window and not which surface it
/// applies to): **the window applies to the TERMINAL render, and the committed
/// `todo.md` carries everything.** A window resolved against a clock makes a
/// file's content depend on when it was generated rather than on what
/// happened, and this repository commits `todo.md` -- so a windowed file would
/// diff with no cause in the estate, which is committed churn under D02. A
/// terminal render is a moment and may depend on now; a committed file is a
/// record and may not.
///
/// Two callers, one function, and the divergence is visible at the call site
/// rather than hidden in a second copy of the bucketing rules. `TodoWindow` is
/// an id ALLOWLIST rather than a cutoff, because the cutoff is resolved inside
/// SQL (D42) and this function never learns a time -- it is handed the answer.
pub fn todo(threads: &[Thread], ctx: &RenderContext<'_>, window: &TodoWindow) -> String {
  let mut buckets = todo_buckets(threads);
  buckets.done.retain(|item| window.shows(&item.id));

  let mut out = String::new();
  out.push_str("# TODO\n\n");
  out.push_str("A DOING / TODO / DONE view, projected from steel-thread and work-package status: one row per steel thread, with its work packages nested beneath it. Generated -- change a status with the CLI, never by editing this file.\n\n");
  out.push_str(&bucket("DOING", &buckets.doing));
  out.push_str(&bucket("TODO", &buckets.todo));
  out.push_str("## DONE\n\n");
  out.push_str(&items(&buckets.done));
  finish(out, ctx, "the thread canon")
}

fn bucket(name: &str, rows: &[TodoItem]) -> String {
  format!("## {name}\n\n{}", items(rows))
}

/// **Takes the rows, not their labels.** It was handed `&[String]` and so could
/// only emit a constant glyph -- the status had been dropped one call earlier,
/// which is why no test of this function could have caught it.
fn items(rows: &[TodoItem]) -> String {
  if rows.is_empty() {
    return "_(none)_\n\n".to_string();
  }
  let mut out = String::new();
  for row in rows {
    write_row(&mut out, row, 0);
  }
  out.push('\n');
  out
}

/// One row and everything under it, two spaces per level.
///
/// Recursive rather than a two-level loop because the shape is a tree and a
/// hand-unrolled second level would have to be found and changed if the model
/// ever grows a third. Today `children` is empty below depth one.
fn write_row(out: &mut String, row: &TodoItem, depth: usize) {
  let indent = "  ".repeat(depth);
  out.push_str(&format!("{indent}- [{}] {}\n", row.glyph, row.label));
  for child in &row.children {
    write_row(out, child, depth + 1);
  }
}

// ---------------------------------------------------------------------------
// Rendering the whole estate, and the skew check
// ---------------------------------------------------------------------------

/// Which thread owns a view path, if any.
///
/// **The single answer, consulted by the write path and the diagnostic path
/// alike.** `Facade::projection` uses it to decide what to WRITE and
/// [`skew`] uses it to decide what a missing file MEANS; two copies would let
/// those two disagree about whether a view should exist, which is exactly the
/// divergence `.intentfiles` exists to settle.
///
/// `None` is a real answer and not a failure: `steel_threads.md` and `todo.md`
/// are project-level, belong to no artefact, and so are never subject to the
/// manifest.
pub fn owning_thread(project: &Project, path: &std::path::Path, canon: &Canon) -> Option<String> {
  canon
    .threads
    .iter()
    .find(|t| path.starts_with(project.thread_dir(&t.id)))
    .map(|t| t.id.clone())
}

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
    // **`All`, and it is not a default reached for want of a window.** This is
    // the committed artefact, and D44's window is ruled terminal-only
    // precisely so that a generated file stays a function of the model and
    // nothing else.
    content: todo(&canon.threads, ctx, &TodoWindow::All),
  });
  views
}

/// Render every view and write it through a [`WriteSet`].
///
/// **THIS FUNCTION HAS NO PRODUCTION CALLER AND IS NOT A SECOND WRITE PATH.**
/// Every caller is a test. It once wrote views itself with a bare `fs::write`
/// loop, which made it a divergent expression of the db -> disk direction that
/// [`crate::facade::Facade::projection`] already declares it owns -- and the
/// consequence was not theoretical. A skip-when-unchanged guard was added
/// HERE, was correct, and reached nothing; `view_determinism.rs` drove it
/// directly and stayed green while every real verb churned the estate.
///
/// So it now BUILDS A WRITE SET AND COMMITS IT: one write mechanism, and the
/// tests that use this exercise the one the estate runs. Deleting it instead
/// would have left the concern with no test at all.
///
/// **The mtime skip is NOT here.** It lives in [`WriteSet::commit`], which is
/// where every production write already goes.
///
/// The returned `Vec<View>` is still every view the model implies, not the
/// subset written. Callers ask this function what the views ARE; what it had
/// to touch to get there is its own business.
pub fn write_all(
  project: &Project,
  canon: &Canon,
  ctx: &RenderContext<'_>,
) -> Result<Vec<View>, std::io::Error> {
  let views = render_all(project, canon, ctx);
  let mut set = WriteSet::new();
  for view in &views {
    set.add(view.path.clone(), view.content.clone());
  }
  // `WriteError` is flattened into `io::Error` rather than widening this
  // signature across six test files. The Display chain is preserved, so a
  // torn-rollback still says so in the message it carries.
  set.commit().map_err(std::io::Error::other)?.keep();
  Ok(views)
}

/// The skew check (AC-03.4): regenerate every view and name any that differs
/// from what is on disk.
///
/// A hand-edited view is CAUGHT, never silently outvoted. The distinction
/// matters: `write_all` would overwrite the edit without a word, and the
/// person who made it would have no way to know their change was discarded.
/// `doctor` runs this; the migrator runs it before it converges.
pub fn skew(
  project: &Project,
  canon: &Canon,
  ctx: &RenderContext<'_>,
  realised: &crate::intentfiles::Realised,
) -> Vec<Finding> {
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
      // **ABSENT IS SILENCE ONLY WHERE THE MANIFEST SAYS THE ARTEFACT IS NOT
      // REALISED, and that silence is bounded rather than blanket.** Under
      // `.intentfiles` a dehydrated thread's views are legitimately gone -- that
      // is the feature -- so reporting their absence made `doctor` unhealthy for
      // every dehydrated thread in the estate: 234 findings at rc=1 on a healthy
      // tree, every one instructing the operator to regenerate a file the design
      // says should not exist, and the remedy it printed would have been answered
      // by `organize` re-hydrating nothing.
      //
      // **A view absent for a DECLARED artefact is still a real finding.** That
      // is a genuine loss, and a blanket `Err(_) => {}` would have traded 234
      // false findings for one silent real one -- which is the trade this arm
      // exists to refuse. `Realised::declares` is fail-open, so a missing or
      // unreadable manifest keeps every view in scope and this check keeps its
      // old behaviour exactly.
      //
      // The argument was already written down one file over, at
      // `doctor::attachment_drift`, and applied to attachments and not to views
      // -- a prediction of this exact defect by the author of the sibling path.
      // The two paths now answer the same question the same way.
      Err(_) => {
        let dehydrated = owning_thread(project, &view.path, canon)
          .is_some_and(|owner| !realised.declares(&owner));
        if !dehydrated {
          findings.push(Finding::new(
            &rel,
            FindingClass::ViewSkew,
            "generated view is missing; regenerate it",
          ));
        }
      }
    }
  }
  findings
}

// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use super::*;
  use crate::model::{THREAD_SCHEMA, ThreadStatus, WpStatus};

  /// The wire spelling is kebab-case (`#[serde(rename_all)]`), so these are the
  /// status values as `thread.json` carries them.
  fn thread(id: &str, status: &str) -> Thread {
    serde_json::from_value(serde_json::json!({
      "schema": THREAD_SCHEMA,
      "id": id,
      "title": format!("{id} title"),
      "status": status,
      "created": "2026-08-18",
    }))
    .expect("thread fixture")
  }

  fn with_wp(id: &str, status: &str, wp_status: &str) -> Thread {
    serde_json::from_value(serde_json::json!({
      "schema": THREAD_SCHEMA,
      "id": id,
      "title": format!("{id} title"),
      "status": status,
      "created": "2026-08-18",
      "wps": [{ "seq": 1, "title": "the work package", "status": wp_status }],
    }))
    .expect("thread-with-wp fixture")
  }

  fn ctx() -> RenderContext<'static> {
    RenderContext {
      version: "3.0.0-test",
    }
  }

  /// **THE test, and it drives the real renderer over a fixture carrying both
  /// states.** `Completed` and `Cancelled` share the DONE bucket, so the glyph
  /// is the ONLY thing in the rendered file that distinguishes them -- which is
  /// why a constant glyph presented cancelled work as completed rather than
  /// merely looking plain. Re-constant the glyph and this fails.
  #[test]
  fn cancelled_and_completed_share_the_done_bucket_and_still_render_differently() {
    let threads = vec![thread("ST0001", "completed"), thread("ST0002", "cancelled")];
    let out = todo(&threads, &ctx(), &TodoWindow::All);
    let done = out.split("## DONE").nth(1).expect("a DONE section");

    assert!(done.contains("- [x] ST0001"), "completed row in:{done}");
    assert!(done.contains("- [~] ST0002"), "cancelled row in:{done}");
  }

  /// The anti-collapse canary: six states, six glyphs, pairwise distinct.
  ///
  /// A vocabulary that reuses a glyph re-creates the defect one state at a
  /// time, and the renderer above would still look correct on any fixture that
  /// happened to omit the colliding pair.
  #[test]
  fn the_six_thread_states_have_six_distinct_glyphs() {
    let all = [
      ThreadStatus::Triage,
      ThreadStatus::NotStarted,
      ThreadStatus::Wip,
      ThreadStatus::Hold,
      ThreadStatus::Completed,
      ThreadStatus::Cancelled,
    ];
    let glyphs: Vec<char> = all.iter().map(|s| s.glyph()).collect();
    let mut distinct = glyphs.clone();
    distinct.sort_unstable();
    distinct.dedup();
    assert_eq!(
      distinct.len(),
      all.len(),
      "two states share a glyph: {glyphs:?}"
    );

    let wp: Vec<char> = [WpStatus::NotStarted, WpStatus::Wip, WpStatus::Done]
      .iter()
      .map(|s| s.glyph())
      .collect();
    let mut wp_distinct = wp.clone();
    wp_distinct.sort_unstable();
    wp_distinct.dedup();
    assert_eq!(wp_distinct.len(), 3, "two wp states share a glyph: {wp:?}");
  }

  /// `items` renders all three buckets, so the defect spanned all three and the
  /// fix has to as well. DONE is covered above; this is DOING and TODO.
  #[test]
  fn every_bucket_computes_its_glyph_because_one_function_renders_all_three() {
    let threads = vec![
      thread("ST0001", "wip"),
      thread("ST0002", "not-started"),
      thread("ST0003", "triage"),
      thread("ST0004", "hold"),
    ];
    let out = todo(&threads, &ctx(), &TodoWindow::All);

    assert!(out.contains("- [-] ST0001"), "wip in:\n{out}");
    assert!(out.contains("- [ ] ST0002"), "not-started in:\n{out}");
    assert!(out.contains("- [?] ST0003"), "triage in:\n{out}");
    assert!(out.contains("- [!] ST0004"), "hold in:\n{out}");
  }

  /// Work packages go through the same renderer and lost the same fact.
  ///
  /// **The row shape changed on 2026-08-18 (hv) and the glyph fact did not:**
  /// a package is NESTED under its thread and no longer repeats the thread id,
  /// but it still carries its OWN status glyph rather than its thread's, which
  /// is what this test has always been for.
  #[test]
  fn work_package_rows_carry_their_own_glyph_not_their_threads() {
    let out = todo(
      &[with_wp("ST0001", "wip", "not-started")],
      &ctx(),
      &TodoWindow::All,
    );
    assert!(
      out.contains("- [-] ST0001: ST0001 title"),
      "thread in:\n{out}"
    );
    assert!(
      out.contains("\n  - [ ] 01: the work package"),
      "wp nested under its thread, own glyph, no repeated id, in:\n{out}"
    );
  }

  /// **A thread carries its packages into whichever bucket its OWN status
  /// picks, and every package comes along.**
  ///
  /// The flat view filed each package by its own status, so one thread could
  /// appear in DOING and TODO at once, and a `done` package was dropped
  /// entirely -- the finished work was invisible in the view whose job is
  /// showing progress.
  #[test]
  fn a_thread_carries_every_work_package_into_its_own_bucket() {
    let mut t = with_wp("ST0001", "not-started", "done");
    t.wps.push(
      serde_json::from_value(serde_json::json!({
        "seq": 2, "title": "the second", "status": "wip"
      }))
      .expect("wp"),
    );
    let buckets = todo_buckets(&[t]);

    assert!(buckets.doing.is_empty(), "the THREAD is not-started");
    assert_eq!(buckets.todo.len(), 1, "one row for the thread, not three");
    let rows: Vec<(&str, char)> = buckets.todo[0]
      .children
      .iter()
      .map(|c| (c.label.as_str(), c.glyph))
      .collect();
    assert_eq!(
      rows,
      vec![("01: the work package", 'x'), ("02: the second", '-')],
      "every package, in sequence order, each with its own glyph"
    );
  }

  /// **The `--json` face lost the same fact and nobody had reported it.** The
  /// bucket is three-valued and the status is six, so `done` alone cannot be
  /// read back into a status -- a machine consumer could not tell cancelled
  /// from completed either.
  #[test]
  fn the_json_face_carries_the_status_the_bucket_cannot_recover() {
    let buckets = todo_buckets(&[thread("ST0001", "completed"), thread("ST0002", "cancelled")]);
    let rows: Vec<(&str, &str, char)> = buckets
      .done
      .iter()
      .map(|i| (i.id.as_str(), i.status, i.glyph))
      .collect();
    assert_eq!(
      rows,
      vec![("ST0001", "Completed", 'x'), ("ST0002", "Cancelled", '~')]
    );
  }

  /// The control: an empty bucket is still a sentinel, not a stray glyph.
  #[test]
  fn an_empty_bucket_renders_the_sentinel() {
    assert_eq!(items(&[]), "_(none)_\n\n");
  }

  /// **`status_reason` reached no human face at all.** Four verbs REQUIRE one
  /// and refuse without it; it was written to the entity, stored, and exposed
  /// on the GraphQL SDL -- and rendered by nothing a person reads. Reported by
  /// ic, who round-tripped `st hold` and then `wp reopen` to prove it.
  ///
  /// The negative half is the one that keeps the view quiet: the key appears
  /// only when there IS a reason, so a thread that never took a guarded
  /// transition renders exactly as before.
  #[test]
  fn a_thread_with_a_status_reason_renders_it_and_one_without_stays_silent() {
    let mut held = thread("ST0001", "hold");
    held.status_reason = Some("waiting on the schema ruling".to_string());
    let with = info(&held, &ctx());
    assert!(
      with.contains("status_reason: waiting on the schema ruling"),
      "the view must show the reason the verb demanded:\n{with}"
    );

    let plain = info(&thread("ST0002", "wip"), &ctx());
    assert!(
      !plain.contains("status_reason"),
      "no reason means no key, so nothing churns:\n{plain}"
    );
  }

  /// The work-package half, which `wp reopen` guards the same way and which a
  /// fix scoped to threads would have left open (ic).
  #[test]
  fn a_work_package_with_a_status_reason_renders_it_too() {
    let mut t = with_wp("ST0001", "wip", "wip");
    t.wps[0].status_reason = Some("reopened: the gate was wrong".to_string());
    let rendered = wp_info(&t, &t.wps[0], &ctx());
    assert!(
      rendered.contains("status_reason: reopened: the gate was wrong"),
      "the wp view must show it:\n{rendered}"
    );
  }
}
