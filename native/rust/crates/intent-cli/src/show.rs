//! The one text rendering of an entity, shared by the CLI `show` arms and the
//! MCP resource read (`AC-09.5`).
//!
//! # Why this exists as one home
//!
//! `AC-09.5` says an MCP resource's contents match what the equivalent CLI read
//! returns. The strongest form of "match" is byte-identity, and byte-identity
//! needs ONE renderer: two functions that agree today are `AC-09.4`'s
//! "agreement is not derivation" — invisible drift until someone edits one. So
//! `render.rs`'s `st show` / `wp show` / `issues show` arms and `mcp.rs`'s
//! resource read call THIS, and the match clause holds by construction rather
//! than by a comparison that could go stale.
//!
//! # Why here and not in `render.rs`
//!
//! Same reason `guide.rs` is separate: `render.rs` is parse → facade → render
//! and every arm opens a project. These functions touch no facade — they are a
//! pure projection of a model the caller already holds — so the MCP tier can
//! call them after its own `Facade::*_show`, keeping vc's rule that the tier
//! calls the facade and never the CLI dispatch arm.
//!
//! # The contract: what each function returns
//!
//! The EXACT bytes the matching `show` arm prints, trailing newline included,
//! so the CLI arm is `print!("{}", show::thread(t))` and the resource read
//! returns the same string. A line that the arm emits only when a field is
//! present is emitted here only then; the order is the arm's order. The `issue`
//! renderer is the TEXT branch of `issues show`; its `--format json` branch is
//! a different surface and stays in `render.rs`.

use std::fmt::Write;

use intentsvcs::model::{Issue, Thread, WorkPackage};

/// `intent st show <id>`'s text: id, title, status, its reason if any, created,
/// completed if any. No body — `st show` does not print it.
pub fn thread(t: &Thread) -> String {
  let mut s = String::new();
  let _ = writeln!(s, "{}: {}", t.id, t.title);
  let _ = writeln!(s, "status: {}", t.status.display());
  if let Some(reason) = &t.status_reason {
    let _ = writeln!(s, "reason: {reason}");
  }
  let _ = writeln!(s, "created: {}", t.created);
  if let Some(done) = &t.completed {
    let _ = writeln!(s, "completed: {done}");
  }
  s
}

/// `intent wp show <st> <seq>`'s text. The parent `st` is the caller's — the
/// model is identified within a thread and carries no parent id — matching the
/// arm, which takes `st` from its argument.
pub fn work_package(st: &str, wp: &WorkPackage) -> String {
  let mut s = String::new();
  let _ = writeln!(s, "{st}/WP-{:02}: {}", wp.seq, wp.title);
  let _ = writeln!(s, "status: {}", wp.status.display());
  if let Some(reason) = &wp.status_reason {
    let _ = writeln!(s, "reason: {reason}");
  }
  let _ = writeln!(s, "scope: {}", wp.scope_display());
  s
}

/// `intent issues show <n>`'s TEXT branch: number, title, status, severity if
/// any, created, closed if any, reporter if any, then the body after a blank
/// line if it is non-empty.
pub fn issue(i: &Issue) -> String {
  let mut s = String::new();
  let _ = writeln!(s, "{:04}: {}", i.number, i.title);
  let _ = writeln!(s, "status: {}", i.status.display());
  if let Some(sev) = &i.severity {
    let _ = writeln!(s, "severity: {sev}");
  }
  let _ = writeln!(s, "created: {}", i.created);
  if let Some(closed) = &i.closed {
    let _ = writeln!(s, "closed: {closed}");
  }
  if let Some(reporter) = &i.reporter {
    let _ = writeln!(s, "reporter: {reporter}");
  }
  if !i.body.is_empty() {
    let _ = writeln!(s);
    let _ = writeln!(s, "{}", i.body);
  }
  s
}
