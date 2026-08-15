//! `intent doctor` -- the health report (AC-06.2).
//!
//! v2's doctor was a pile of greps over markdown, which is why it could only
//! ever find things that were textually wrong. v3's asks three kinds of
//! question, and they are kept apart because they have three different
//! remedies -- a report that mixed them would tell the operator what is broken
//! without telling them which lever to pull:
//!
//! - **Model integrity.** The canon parses and validates, but says two things
//!   that cannot both be true. The schema cannot catch these: each statement
//!   is individually well-formed and only the RELATIONSHIP is wrong. Remedy:
//!   change the canon, through the CLI.
//! - **DB integrity.** The rebuild-identity invariant (D01): rebuilding the
//!   derived tables from canon must produce exactly what is in the store.
//!   Remedy: delete `intent/.cache/` -- `rm` is always safe, which is the
//!   whole point of D01, and no DB migration ever exists to go wrong.
//! - **File checks.** The working tree disagreeing with the model: a
//!   hand-edited generated view (skew), or a file that cannot be read as what
//!   it claims to be (unparsed). Remedy: regenerate, or fix the file.
//!
//! Everything reports through the one refusal grammar in [`crate::finding`].
//! A second report shape would be a second thing to learn for the same job,
//! and v2 shipped five spellings of one failure before the 0023 voice sweep.
//!
//! **What this deliberately does NOT check.** After [`crate::ingest::load`],
//! the DB has just been rebuilt from canon, so most "is the DB right" checks
//! are true by construction. Those are not written here. A check that cannot
//! fail is worse than no check: it costs the same to read, and it reports
//! confidence it did not earn. The one DB check that survives is
//! rebuild-identity, which is genuinely capable of failing -- it exercises the
//! store's write-then-read path and would catch a field that does not survive
//! the DDL round trip.

use crate::finding::{Finding, FindingClass};
use crate::ingest::Canon;
use crate::model::{AcKind, AcScope, AtKind, AtStatus, Thread, ThreadStatus};
use crate::project::Project;
use crate::store::Store;
use crate::sync::{self, FileState};
use crate::views::{self, RenderContext};

/// A doctor run: every finding, plus what was actually examined.
///
/// The counts are here so a clean report can say what it covered. "No problems
/// found" over an estate the checker never read is the same sentence as "no
/// problems found" over one it read completely, and only one of them is worth
/// anything.
#[derive(Debug, Clone, Default)]
pub struct Report {
  pub findings: Vec<Finding>,
  pub threads_checked: usize,
  pub issues_checked: usize,
  pub files_checked: usize,
  pub views_checked: usize,
}

impl Report {
  pub fn is_healthy(&self) -> bool {
    self.findings.is_empty()
  }

  /// v2's exit contract: 0 when healthy, 1 when anything was found.
  pub fn exit_code(&self) -> i32 {
    i32::from(!self.is_healthy())
  }
}

/// Diagnose a project WITHOUT requiring that it can be opened.
///
/// **This is the entry point, and it takes no [`Store`] and no [`Canon`] --
/// deliberately.** Doctor has to be the most robust command in the tool,
/// because it is what you run when the others have stopped working, and the
/// first version of it was not: it went through `Facade::open`, which loads
/// canon into the DB, so a duplicate criterion id inside one thread tripped a
/// UNIQUE constraint and the whole command died with a SQLite message. The
/// tool then advised running `intent doctor` -- which is what had just failed.
///
/// Every stage therefore degrades into a finding rather than an error:
///
/// - canon that will not parse or validate becomes the ingest findings, and
///   the run stops there, because nothing further can be said about a model
///   that could not be read;
/// - canon that will not load into a store becomes a finding naming the
///   constraint, which is the diagnosis;
/// - a working tree that cannot be scanned becomes a finding.
///
/// A `Report` always comes back. Doctor never refuses.
pub fn diagnose(project: &Project, ctx: &RenderContext<'_>) -> Report {
  let mut report = Report::default();

  let canon = match crate::ingest::read(project) {
    Ok(canon) => canon,
    Err(e) => {
      report.findings.extend(ingest_findings(&e));
      return report;
    }
  };

  report.threads_checked = canon.threads.len();
  report.issues_checked = canon.issues.len();

  for thread in &canon.threads {
    let file = project.relative(&project.thread_json(&thread.id));
    model_checks(thread, &canon, &file, &mut report.findings);
  }

  db_checks(&canon, project, &mut report.findings);
  file_checks(project, &canon, ctx, &mut report);

  report
}

/// Unwrap an ingest failure into findings. A refusal already carries them; any
/// other failure (an unreadable file, a bad directory) becomes one finding, so
/// the report shape does not change with the reason.
fn ingest_findings(e: &crate::ingest::IngestError) -> Vec<Finding> {
  match e {
    crate::ingest::IngestError::Refused(refusal) => refusal.findings.clone(),
    other => vec![Finding::new(
      "intent/",
      FindingClass::UnknownFileShape,
      format!("the committed canon could not be read: {other}"),
    )],
  }
}

// ---------------------------------------------------------------------------
// Model integrity
// ---------------------------------------------------------------------------

fn model_checks(thread: &Thread, canon: &Canon, file: &str, out: &mut Vec<Finding>) {
  let mut add = |detail: String, class: FindingClass| {
    out.push(Finding::new(file, class, detail));
  };

  // Duplicate natural ids WITHIN a thread. Across threads is ingest's job; a
  // thread that names the same criterion twice validates fine, because the
  // schema constrains each element and not the collection.
  for (kind, ids) in [
    (
      "work package",
      thread
        .wps
        .iter()
        .map(|w| format!("WP-{:02}", w.seq))
        .collect::<Vec<_>>(),
    ),
    (
      "criterion",
      thread.criteria.iter().map(|c| c.id.clone()).collect(),
    ),
    (
      "acceptance test",
      thread.tests.iter().map(|t| t.id.clone()).collect(),
    ),
  ] {
    let mut seen: Vec<&String> = Vec::new();
    for id in &ids {
      if seen.contains(&id) {
        add(
          format!("{kind} {id} is declared more than once in {}", thread.id),
          FindingClass::DuplicateId,
        );
      } else {
        seen.push(id);
      }
    }
  }

  // An acceptance test covering a criterion that does not exist. This is the
  // one that matters most: the close gate reads `covers` to decide whether a
  // criterion is satisfied, so a typo'd id means a test that proves nothing
  // while looking exactly like one that does.
  for test in &thread.tests {
    if test.covers.is_empty() {
      add(
        format!(
          "{} covers nothing, so no criterion can ever be satisfied by it",
          test.id
        ),
        FindingClass::ModelInconsistent,
      );
    }
    for ac in &test.covers {
      if !thread.criteria.iter().any(|c| &c.id == ac) {
        add(
          format!(
            "{} covers {ac}, which is not a criterion of {}",
            test.id, thread.id
          ),
          FindingClass::ModelInconsistent,
        );
      }
    }
    // A test-kind row with neither a file nor a legacy carry names nothing
    // runnable, so its green cannot be checked by anyone.
    if test.kind == AtKind::Test
      && test.file.is_none()
      && test.legacy.is_none()
      && test.status != AtStatus::ToWrite
    {
      add(
        format!(
          "{} is a test row at {} with no file reference",
          test.id,
          crate::model::enum_str(&test.status)
        ),
        FindingClass::ModelInconsistent,
      );
    }
  }

  // A criterion whose group names a work package that does not exist. Group
  // `00` is thread-level and always legitimate.
  for criterion in &thread.criteria {
    if let Some(seq) = group_seq(&criterion.id)
      && seq != 0
      && !thread.wps.iter().any(|w| w.seq == seq)
    {
      add(
        format!(
          "{} belongs to WP-{seq:02}, which {} does not have",
          criterion.id, thread.id
        ),
        FindingClass::ModelInconsistent,
      );
    }

    // Test-backed satisfaction is COMPUTED from covering green tests and must
    // never be stored (data-model.md). The facade refuses to set it; canon
    // written by hand or carried from v2 can still carry it.
    if criterion.kind == AcKind::Test && criterion.satisfied.is_some() {
      add(
        format!(
          "{} is test-backed but carries a stored `satisfied`, which is double truth -- satisfaction comes from its covering tests",
          criterion.id
        ),
        FindingClass::ModelInconsistent,
      );
    }

    // A descoped criterion naming a thread this project does not have is a
    // dangling promise: the requirement was moved somewhere that does not
    // exist, so nobody is holding it.
    if let AcScope::Descoped { to, .. } = &criterion.scope
      && !canon.threads.iter().any(|t| &t.id == to)
    {
      add(
        format!(
          "{} is descoped to {to}, which is not a steel thread in this project",
          criterion.id
        ),
        FindingClass::ModelInconsistent,
      );
    }
  }

  // Related threads that do not exist.
  for related in &thread.related {
    if !canon.threads.iter().any(|t| t.id == related.id) {
      add(
        format!(
          "{} names {} as related, and there is no such steel thread",
          thread.id, related.id
        ),
        FindingClass::ModelInconsistent,
      );
    }
  }

  // Completion and status must agree in both directions.
  match (thread.status, &thread.completed) {
    (ThreadStatus::Completed, None) => add(
      format!("{} is Completed with no completion date", thread.id),
      FindingClass::ModelInconsistent,
    ),
    (status, Some(date)) if status != ThreadStatus::Completed => add(
      format!(
        "{} carries a completion date ({date}) while its status is {}",
        thread.id,
        crate::model::enum_str(&status)
      ),
      FindingClass::ModelInconsistent,
    ),
    _ => {}
  }
}

/// Whether a store snapshot holds nothing at all -- a cold cache.
///
/// Every table empty, not merely the threads table: a snapshot with rows in
/// `issues` and none in `threads` is a populated cache that disagrees, which
/// is exactly what the caller wants to hear about.
fn is_empty_snapshot(snapshot: &serde_json::Value) -> bool {
  snapshot.as_object().is_some_and(|tables| {
    tables
      .values()
      .all(|rows| rows.as_array().is_some_and(Vec::is_empty))
  })
}

/// The `gg` of an `AC-<gg>.<n>` id.
fn group_seq(id: &str) -> Option<u32> {
  id.strip_prefix("AC-")
    .and_then(|rest| rest.split('.').next())
    .and_then(|gg| gg.parse().ok())
}

// ---------------------------------------------------------------------------
// DB integrity -- the D01 rebuild-identity invariant
// ---------------------------------------------------------------------------

/// Rebuild the derived tables from canon into a throwaway in-memory store and
/// require the result to equal what is on disk.
///
/// This is the one DB check that can actually fail, and it is worth having
/// because it exercises the store's write-then-read path end to end: a model
/// field that does not survive the DDL round trip shows up here and nowhere
/// else. It uses an in-memory store so a health check never writes to the
/// project's own cache -- a doctor that repaired what it was measuring would
/// report on a state that no longer existed by the time anyone read it.
fn db_checks(canon: &Canon, project: &Project, out: &mut Vec<Finding>) {
  // Rebuild into a throwaway in-memory store FIRST. If canon cannot be loaded
  // at all -- a duplicate id tripping a UNIQUE constraint is the live case --
  // that is the finding, and it is a far better diagnosis than the constraint
  // message the operator would otherwise meet through some unrelated command.
  let rebuilt = match Store::open_in_memory()
    .and_then(|mut fresh| fresh.rebuild(&canon.threads, &canon.issues).map(|()| fresh))
    .and_then(|fresh| fresh.snapshot())
  {
    Ok(snapshot) => snapshot,
    Err(e) => {
      out.push(Finding::new(
        "intent/",
        FindingClass::ModelInconsistent,
        format!(
          "the committed canon cannot be loaded into a store, so no command that needs the DB can run: {e}"
        ),
      ));
      return;
    }
  };

  // Only now compare against what is on disk. A cache that will not open is
  // not a finding worth reporting: it is deleted and rebuilt by the next
  // command, which is exactly what D01 buys.
  let Ok(store) = Store::open(&project.db_path()) else {
    return;
  };
  let Ok(on_disk) = store.snapshot() else {
    return;
  };

  // A COLD cache is not a stale one. `intent/.cache/` is gitignored (D21), so
  // an empty store is the normal state of every fresh clone and every project
  // whose reads have run in memory -- and D01 makes it always safe. Reporting
  // it would fire on the commonest healthy state there is, which is how a
  // health check teaches people to ignore health checks.
  //
  // A cache with CONTENT that disagrees is different: it was written by an
  // older binary or by something out of band, and reads served from it would
  // be answering from a model nobody committed.
  if is_empty_snapshot(&on_disk) {
    return;
  }

  if on_disk != rebuilt {
    out.push(Finding::new(
      "intent/.cache/intent.db",
      FindingClass::ModelInconsistent,
      "the runtime store does not match a rebuild from committed canon -- commands are answering from the store and will report the stale model until it is refreshed; run `intent sync` (deleting intent/.cache/ also works, since the store is derived)",
    ));
  }
}

// ---------------------------------------------------------------------------
// File checks
// ---------------------------------------------------------------------------

fn file_checks(project: &Project, canon: &Canon, ctx: &RenderContext<'_>, report: &mut Report) {
  let skew = views::skew(project, canon, ctx);
  report.views_checked = views::render_all(project, canon, ctx).len();
  report.findings.extend(skew);

  // The working tree, scanned fresh. `previous` is empty deliberately: doctor
  // asks "can every modelled file be read as what it claims to be", which does
  // not depend on what changed since last time, and passing the stored index
  // would make the answer depend on when doctor last ran.
  match sync::scan(project.root(), &[]) {
    Ok(entries) => {
      report.files_checked = entries.len();
      for entry in entries.iter().filter(|e| e.state == FileState::Unparsed) {
        report.findings.extend(entry.findings.iter().cloned());
      }
    }
    Err(e) => report.findings.push(Finding::new(
      project.relative(project.root()),
      FindingClass::UnknownFileShape,
      format!("the working tree could not be scanned: {e}"),
    )),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_group_number_is_read_off_a_criterion_id() {
    assert_eq!(group_seq("AC-06.2"), Some(6));
    assert_eq!(
      group_seq("AC-00.1"),
      Some(0),
      "00 is the thread-level group"
    );
    assert_eq!(group_seq("AT-06.2"), None, "an AT id is not an AC id");
    assert_eq!(group_seq("nonsense"), None);
  }
}
