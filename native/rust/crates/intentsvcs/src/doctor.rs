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
//! - **DB integrity.** The rebuild-identity invariant: re-deriving the tables
//!   from the committed extract must reproduce what is in the store. Remedy:
//!   investigate the disagreement. **NOT "delete the cache"** -- that was the
//!   remedy while the DB was a rebuildable index, and under the reversed D01
//!   (hv, 2026-08-15) the DB is the durable SSOT and the files are a secondary
//!   artefact, so deleting it discards anything newer than the extract.
//!   Re-creating the DB from the extract is a CAPABILITY, not a licence to
//!   treat the DB as disposable.
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
use crate::model::{AcKind, AcState, AtKind, AtStatus, Thread, ThreadStatus};
use crate::project::Project;
use crate::remedy::Remedy;
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
/// **The store is OPTIONAL, and its absence is not a finding.** `doctor` has to
/// work on a project nothing else can open -- that is the whole reason it is a
/// static call rather than a facade method -- so a caller that could not open a
/// store passes `None` and every other check still runs. The backup half is
/// simply not answerable without one, and reporting "no backup" because the
/// store could not be read would be a confident wrong answer at the moment a
/// user is least able to check it.
pub fn diagnose(
  project: &Project,
  ctx: &RenderContext<'_>,
  store: Option<&crate::store::Store>,
) -> Report {
  let mut report = Report::default();

  // FIRST, and it returns rather than continuing. Every check below this line
  // compares the model against the files, and on an unmigrated project the
  // model is EMPTY -- so every one of them fires, and every one of them
  // describes a consequence rather than the cause.
  //
  // Measured before this landed, on a two-thread v2 fixture: doctor reported
  // "2 findings across 0 threads", both of them view-skew saying the
  // generated views were missing. That is a confident RED at first contact
  // whose honest remedy -- regenerate the views -- would have rendered an
  // empty estate over the top of real work.
  if let crate::project::Migration::Pending(pending) = project.migration() {
    report.findings.push(Finding::new(
      project.relative(&Project::config_path(project.root())),
      FindingClass::Unmigrated,
      format!("{pending} -- {}", pending.remedy()),
    ));
    return report;
  }

  // **AFTER the unmigrated return, deliberately.** A window an unmigrated
  // project cannot honour is true and useless: `intent todo` refuses on that
  // project for a much larger reason, so reporting it at first contact would
  // put a second cause beside the one that matters -- which is the noise the
  // early return above exists to prevent.
  //
  // The DETAIL carries the instance arithmetic, exactly as the unmigrated
  // finding carries `Migration::remedy()`: the class remedy says what is true
  // of the class, and the two honourable values either side of a rejected
  // window are true of this project only.
  if let Err(e) = project.config().todo.window() {
    report.findings.push(Finding::new(
      project.relative(&Project::config_path(project.root())),
      FindingClass::UnhonourableSetting,
      format!("{e} -- {}", e.remedy()),
    ));
  }

  if let Some(store) = store {
    report.findings.extend(backup_findings(project, store));
  }

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
  history_checks(project, &canon, store, &mut report.findings);

  report
}

/// **Does the repository carry the history this machine has?** (AC-03.11.)
///
/// `event_log` is the one table derived from nothing (D34), so it is the one
/// whose absence nothing else reveals: every other question the estate answers
/// is answerable without it, which is why a clone with no history at all read
/// as perfectly healthy for as long as it did.
///
/// **The condition is two artefacts disagreeing, not one artefact missing**,
/// and the difference is what makes this reportable at all. "Entities and no
/// log" was the first version and it is too broad to be useful: the per-thread
/// mutation path deliberately does not rewrite the log extract, so a project
/// mutated normally sits in that state routinely, and a finding that fires
/// routinely is the trained-to-be-ignored failure. Worse, a hand-authored or
/// freshly migrated estate is permanently in it -- so the check would have been
/// loudest exactly where AC-03.11 needs it trusted.
///
/// What IS provable is this store holding envelopes the repository does not.
/// That is not a guess about intent; it is history that exists on one machine
/// and would not survive a clone, reported to the person who still has it. The
/// case it does not cover -- a clone that arrived with no log, where the data is
/// already gone and nothing local can prove it ever existed -- is with vc,
/// because answering it means ruling on how current the committed extract must
/// be, which is a D34 question rather than a diagnostic one.
fn history_checks(
  project: &Project,
  canon: &crate::ingest::Canon,
  store: Option<&crate::store::Store>,
  findings: &mut Vec<Finding>,
) {
  if canon.threads.is_empty() && canon.issues.is_empty() {
    return;
  }
  let Some(store) = store else {
    return;
  };
  let recorded = store.events().map(|e| e.len()).unwrap_or_default();
  if recorded == 0 {
    return;
  }
  let path = project.events_jsonl();
  // Asked by SIZE rather than by existence, so a truncated log -- which a
  // failed write or a bad merge leaves behind, and which `exists()` calls
  // healthy -- answers the same as an absent one.
  let committed = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
  if committed == 0 {
    findings.push(Finding::new(
      project.relative(&path),
      FindingClass::EventLogAbsent,
      format!(
        "this store holds {recorded} event(s) and the repository carries none of them -- a clone of this project would arrive with no history"
      ),
    ));
  }
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

  // **The marked-legacy scope form's two rules, which the TYPE cannot state.**
  //
  // `scope` and `scope_legacy` are two optional fields, so the type permits
  // four combinations and only three of them mean anything: a recorded size, a
  // carried v2 value, or a field nobody ever wrote. **Carrying BOTH is the
  // contradiction** -- it says the work package is an `L` and also that its
  // size could not be expressed -- and a shape that can represent a
  // contradiction eventually stores one.
  //
  // And the carry policy is a rule about WHERE a legacy value may appear, not
  // about its shape: hv ratified that CLOSED threads convert
  // lossless-by-carrying while LIVE threads stay BLOCKED-until-clean.
  // Ingest's `record` applies that split at migration time; this catches one
  // that arrived any other way, including by hand.
  //
  // **BOTH OF THESE ARE ADVISORY AND THE FIRST WORDING SAID OTHERWISE, WHICH
  // COST TWO READERS AN EVENING.** It read "a live one is fixed, not carried"
  // -- an OBLIGATION, indistinguishable in tone from `broken-reference`, which
  // genuinely does refuse. Baize emits 66 of these and migrates at exit 0, so
  // vc read 66 refusals that had not happened and went looking for a hole in
  // AC-10.2's block arm; ic then read the `Blocked` enum, the nine residue
  // classes and the class gate's output to establish there was none. **The
  // estate did not send them, the sentence did**, and it fired toward
  // suspecting the migrator both times.
  //
  // vc has since ruled these do NOT block (D47): the reference RESOLVES, so
  // migrating one loses nothing, and `broken-reference` is the different
  // predicate that earns a refusal. A blocking class that fires on well-formed
  // estates is the guard that gets worked around.
  //
  // **So: a hygiene note describes a STATE, a refusal describes an OBLIGATION**
  // (ic). The policy explanation is kept deliberately -- it is the only place
  // the output says what the carry policy IS -- and losing that to gain the
  // tone would be the wrong trade.
  for wp in &thread.wps {
    if wp.scope.is_some() && wp.scope_legacy.is_some() {
      add(
        format!(
          "WP-{:02} records a scope AND carries a legacy one ({:?}) -- these are alternatives, not a pair",
          wp.seq,
          wp.scope_legacy.as_ref().map(|l| &l.raw)
        ),
        FindingClass::ModelInconsistent,
      );
    }
    if wp.scope_legacy.is_some() && !thread.status.is_closed() {
      add(
        format!(
          "WP-{:02} carries a legacy scope and its thread is still {} -- ADVISORY, not a refusal: the value is well-formed and nothing is blocked by it. The carry converts losslessly on a CLOSED thread, so a live one is worth rewriting in the v3 vocabulary next time the thread is touched",
          wp.seq,
          thread.status.display()
        ),
        FindingClass::ModelInconsistent,
      );
    }
  }

  // **THE SAME TWO RULES ON THE ACCEPTANCE TEST'S OWN MARKED-LEGACY FORM, which
  // had the precedent and not the guard.**
  //
  // `file` and `legacy` are the same shape as `scope` / `scope_legacy` one type
  // over: the model generates both independently, so all four combinations are
  // constructible and only three mean anything. **Carrying BOTH says the row
  // cites a resolvable test path AND that its v2 reference could not be
  // expressed in the 0017 grammar** -- and a reference that is both migrated
  // and unmigratable is the state `--fix` produced when it destroyed one end of
  // a two-ended migration, which is the whole reason `Legacy` exists.
  //
  // Guarding one of two structurally identical invariants is the shape this
  // thread keeps finding: the uncovered one is not less likely to break, it is
  // only less likely to be looked at.
  for at in &thread.tests {
    if at.file.is_some() && at.legacy.is_some() {
      add(
        format!(
          "{} cites a test file AND carries a legacy reference ({:?}) -- these are alternatives, not a pair",
          at.id,
          at.legacy.as_ref().map(|l| &l.raw)
        ),
        FindingClass::ModelInconsistent,
      );
    }
    if at.legacy.is_some() && !thread.status.is_closed() {
      add(
        format!(
          "{} carries a legacy reference and its thread is still {} -- ADVISORY, not a refusal: the reference RESOLVES and nothing is blocked by it. The carry converts losslessly on a CLOSED thread, so a live one is worth rewriting in the v3 grammar next time the thread is touched",
          at.id,
          thread.status.display()
        ),
        FindingClass::ModelInconsistent,
      );
    }
  }

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

    // **Kind and recorded state must agree, and the collapse made this
    // checkable in BOTH directions.** It used to ask one question -- does a
    // test-backed AC carry a stored `satisfied`? -- because that was the only
    // way the two-field model could contradict itself that anyone had named.
    // With one enum the rule is total: a test-backed criterion in scope records
    // `Computed` and nothing else, and an authored one never records
    // `Computed`.
    //
    // **The DECISION is `AcState::permitted_for`, not this match.** What lives
    // here is only the wording, because a finding has to read like something a
    // person can act on. The match below used to make the decision too, with a
    // `_ => None` arm -- so a sixth variant would have been consistent with
    // every kind and this check would have gone quiet about it. Now the model
    // decides, exhaustively, and the worst this can do is describe a real
    // mismatch generically.
    //
    // Three enforcement points, one rule: the facade refuses the transition,
    // the schema face refuses the file, and this reports an estate that already
    // carries it.
    //
    // **Which road is left, now that the first two are shut?** This used to say
    // "canon written by hand or carried from v2", and the hand-written half
    // stopped being true the moment the cross-field clause reached the schema
    // face -- such a file is refused at ingest and never becomes a model. What
    // remains is the CARRIED half, and it is enough on its own: the migration
    // reader (WP-10) is deliberately lenient where ingest is strict, so a v2 AC
    // that carried a satisfaction flag without a `(non-test)` marker arrives as
    // exactly this pair, having never met a schema. That road has no other
    // watcher.
    if !criterion.state.permitted_for(criterion.kind) {
      let complaint = match (criterion.kind, &criterion.state) {
        (AcKind::Test, AcState::Satisfied { .. } | AcState::Unsatisfied) =>
          "is test-backed but records its own satisfaction, which is double truth -- satisfaction comes from its covering tests, so its recorded state must be `computed`".to_string(),
        (AcKind::NonTest, AcState::Computed) =>
          "is `(non-test)` but records `computed`, which claims a satisfaction nothing computes -- an authored criterion has no covering tests to derive one from".to_string(),
        (kind, state) => format!(
          "records a state its kind cannot hold: {state:?} on a {kind:?} criterion"
        ),
      };
      add(
        format!("{} {complaint}", criterion.id),
        FindingClass::ModelInconsistent,
      );
    }

    // **A satisfaction with nothing behind it, on the one road still open to
    // it.** Same three enforcement points as the kind/state rule above and the
    // same division of labour: `Guard::EvidenceRecorded` refuses the API call,
    // `minLength` on the face refuses the file, and this reports an estate that
    // already carries one. What that leaves is the CARRIED half -- the
    // migration reader (WP-10) is deliberately lenient where ingest is strict,
    // so a v2 AC marked satisfied whose evidence text was blank arrives here
    // having never met a schema.
    //
    // **Reported rather than refused, because refusing it is a migration-policy
    // decision and not this module's.** The ruled policy is that a closed
    // thread converts lossless-by-carrying and a live one stays blocked until
    // clean; which of those a blank evidence makes it is hv's and vc's call, so
    // the estate says what it found and does not pre-empt them.
    if let AcState::Satisfied { evidence } = &criterion.state
      && evidence.trim().is_empty()
    {
      add(
        format!(
          "{} records satisfied with no evidence, and a non-test criterion has nothing else to show -- there is no test to re-run to find out whether it holds",
          criterion.id
        ),
        FindingClass::ModelInconsistent,
      );
    }

    // A descoped criterion naming a thread this project does not have is a
    // dangling promise: the requirement was moved somewhere that does not
    // exist, so nobody is holding it.
    if let AcState::Descoped { to, .. } = &criterion.state
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
    .and_then(|fresh| fresh.derived_dump())
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
  let Ok(on_disk) = store.derived_dump() else {
    return;
  };

  // A COLD store is not a stale one. `intent/.cache/` is gitignored (D21), so
  // an empty store is the normal state of every fresh clone, and the extract
  // on disk is what it is re-created from. Reporting it would fire on the
  // commonest healthy state there is, which is how a health check teaches
  // people to ignore health checks.
  //
  // Note the reasoning changed under the reversed D01 even though the
  // behaviour did not: this is silent because a cold store is EXPECTED and
  // re-creatable from the committed extract, NOT because losing a store is
  // harmless. It is not harmless once the store holds anything the extract
  // does not.
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

/// **Backup health, as the two-sided test.**
///
/// The question is not "did a backup fail" -- a schedule that never fires
/// produces no failure to report, so waiting for an error cannot distinguish a
/// working backup from one that has silently never started. It is "how does
/// the newest good snapshot compare to the schedule", which is two recorded
/// values compared to each other and needs nothing to have gone wrong.
///
/// Nothing here learns the time. `hours_since_last_good_snapshot` returns an
/// interval computed inside SQLite, and an interval cannot be written into a
/// record or mistaken for a moment.
fn backup_findings(project: &Project, store: &crate::store::Store) -> Vec<Finding> {
  let Ok(age) = store.hours_since_last_good_snapshot() else {
    // The store is open but the query failed, which is not a backup problem
    // and must not be reported as one.
    return Vec::new();
  };
  let every = f64::from(crate::backup::schedule_hours(project));
  let where_ = project.relative(&crate::backup::snapshot_dir(project));

  match age {
    // **Never is its own message, not a very large number.** "no restorable
    // snapshot has ever been taken" and "the last one is old" call for
    // different actions -- the first says the mechanism has never run, the
    // second says it has stopped -- and collapsing them loses exactly the
    // distinction this check was added for.
    None => vec![Finding::new(
      where_,
      FindingClass::BackupStale,
      "no restorable snapshot has ever been taken of this store".to_string(),
    )],
    // A schedule is a period, not a deadline, so a backup is not late the
    // instant the period elapses. Twice the period is late by any reading, and
    // it keeps a daily schedule from reporting RED every morning before it
    // runs.
    Some(hours) if hours > every * 2.0 => vec![Finding::new(
      where_,
      FindingClass::BackupStale,
      format!("the newest restorable snapshot is {hours:.0}h old against a {every:.0}h schedule"),
    )],
    Some(_) => Vec::new(),
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
