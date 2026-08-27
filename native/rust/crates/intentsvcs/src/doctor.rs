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
use crate::model::{AcKind, AcState, AtKind, AtStatus, Thread};
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
  /// Files under a thread that the store does not hold, by path.
  ///
  /// **NOT findings, and the distinction is the whole design.** A finding
  /// makes the project unhealthy and `doctor` exit 1; these files are outside
  /// the carried extensions BY DESIGN, so reporting them as faults would flag
  /// 100% of a population that is behaving correctly -- a rule describing the
  /// model rather than the data, which is the shape that gets a check deleted.
  ///
  /// **They are listed by path anyway, and that is not a compromise between
  /// the two.** The failure this exists to prevent is a disk becoming optional
  /// and something vanishing because no surface ever said it was uncovered.
  /// Silence and a clean bill of health are indistinguishable to a reader; a
  /// list is neither.
  pub unattached: Vec<String>,
}

impl Report {
  /// Healthy means nothing ACTIONABLE: an advisory describes a state, not an
  /// obligation, and a report carrying only advisories exits 0 (hv, 2026-08-26).
  pub fn is_healthy(&self) -> bool {
    self.actionable() == 0
  }
  /// Findings that are advisories: printed, never counted.
  pub fn advisories(&self) -> usize {
    self
      .findings
      .iter()
      .filter(|f| f.class == FindingClass::Advisory)
      .count()
  }
  /// Findings that demand an action; the number the summary line reports.
  pub fn actionable(&self) -> usize {
    self.findings.len() - self.advisories()
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
    report.findings.extend(undeclared_op_findings(store));
  }
  // **OUTSIDE THE STORE BLOCK, BECAUSE THE GATE IS NOT A PROPERTY OF THE
  // STORE.** A project whose store will not open is exactly one whose commit
  // gate an operator most wants reported, and putting this inside would skip it
  // on precisely those estates.
  report.findings.extend(hook_findings(project));

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

  // **ABOVE `db_checks` AND NOT INSIDE IT, BECAUSE THIS READS CANON AND NOTHING
  // ELSE.** `db_checks` returns early three ways -- canon that will not rebuild,
  // a store that will not open, a cold store -- and every one of those is a
  // NORMAL state for a fresh clone. A status arm hooked in there would be
  // silently skipped on exactly the estate a new reader is looking at, and a
  // check that does not run looks the same as one that found nothing.
  status_gate_disagreement(&canon, project, &mut report.findings);
  db_checks(&canon, project, &mut report.findings);
  file_checks(project, &canon, ctx, &mut report);

  report
}

/// **A unit's recorded status must agree with its gate** -- hv, ratified
/// 2026-08-15 (`data-model.md:472`), in these words: *`wp done` is refused on a
/// BLOCKED gate AND `doctor` reports any unit whose status disagrees with its
/// gate -- both, as recommended.*
///
/// **The refusal shipped and this half did not**, so the ruling sat half-built
/// for five days. vc found it on 2026-08-20 by auditing all 26 work packages by
/// hand and finding FOUR that disagreed -- which is the measurement this arm
/// exists to make unnecessary.
///
/// # It asks the gate rather than recomputing satisfaction
///
/// `contract::gate` is the single home for *is this scope closeable*, and it is
/// what `wp done` consults. Recomputing the same arithmetic here would be a
/// second answer to one question, and the two would disagree the first time
/// either changed -- with this arm reporting the disagreement as a defect in
/// the estate rather than in itself. `RepoFiles` is passed for the same reason:
/// it is what the facade's own `gate` passes, so this reports what `wp done`
/// would decide and not what some other reference resolver would.
///
/// # Two directions, and only two, because the gate has only two answers
///
/// **`Done` over a blocked gate is the dangerous one** and it arrives with
/// nobody doing anything wrong: the gate is consulted at the moment of closing
/// and never again, so a legitimately-closed WP becomes a false green the
/// instant its contract grows. **Not-`Done` over a passing gate** is the
/// benign one -- the work is finished and the field understates it, which is
/// what makes a plan sequenced off these fields wrong.
///
/// **WHAT THIS DELIBERATELY DOES NOT REPORT: partial progress.** A
/// `NotStarted` WP at three criteria of four is visibly under way, and vc's
/// hand audit flagged one -- but the GATE cannot say it, because a gate answers
/// closeable or not. Inventing a third predicate here would put a claim in
/// `doctor` that no ruling backs and that `wp done` does not share, which is
/// the second-answer problem in a different direction. It is named here so the
/// gap is a stated limit rather than something a reader assumes is covered.
fn status_gate_disagreement(canon: &Canon, project: &Project, out: &mut Vec<Finding>) {
  let refs = crate::contract::RepoFiles(project.root());
  for thread in &canon.threads {
    // **A THREAD WITH NO CONTRACT IS NOT A DISAGREEMENT, AND THE FIRST CUT OF
    // THIS REPORTED NINETY-SIX OF THEM.** `gate` returns BLOCKED for a thread
    // with zero criteria -- correct for `wp done`, which must refuse to close
    // what has no contract -- and this estate carries 52 completed v2 threads
    // that were closed before criteria existed at all. Comparing a status
    // against a verdict that means "there is nothing to compare against"
    // produced a finding per work package, all of them false.
    //
    // Same for `acceptance: exempt`: the gate returns EXEMPT, which means the
    // thread declined to be judged, and reading that as "every criterion is
    // satisfied" would report a WIP work package as finished on a thread with
    // no criteria at all.
    //
    // **BOTH SKIPS ARE THE SAME RULE: this arm compares a status to a VERDICT,
    // and a gate that declines to judge has not returned one.**
    if thread.acceptance.is_some() || thread.criteria.is_empty() {
      continue;
    }
    let file = project.relative(&project.thread_json(&thread.id));
    for wp in &thread.wps {
      // **A WP WITH NO CRITERIA OF ITS OWN PASSES ITS GATE VACUOUSLY, AND THE
      // SECOND CUT OF THIS REPORTED TWO OF THEM.** `gate` filters the thread's
      // criteria to the WP's group and asks whether all of them are satisfied;
      // over an empty group that is true for the same reason `0 of 0` is always
      // green. ST0056/WP-15 and WP-16 are `Not Started` with zero criteria
      // each, and this arm called them work already done.
      //
      // **The skip is the same rule as the two above it, one level down: this
      // compares a status to a VERDICT ABOUT A CONTRACT, and an empty scope has
      // no contract to have a verdict about.** Three populations, one
      // sentence -- and each was found by driving the arm rather than by
      // reading it, which is why the count went 96, then 8, then this.
      let group = format!("{:02}", wp.seq);
      let in_scope = thread
        .criteria
        .iter()
        .filter(|c| crate::contract::group_of(&c.id) == group)
        .count();
      if in_scope == 0 {
        continue;
      }
      let verdict =
        crate::contract::gate(thread, crate::contract::Scope::WorkPackage(wp.seq), &refs);
      // **`Exempt` IS NOT FOLDED IN WITH `Pass` HERE**, even though both mean
      // closeable to `wp done`. It cannot arise after the skip above, and
      // matching it anyway would leave a live arm nothing could reach -- which
      // reads as coverage and is not.
      let detail = match (wp.status, &verdict) {
        (crate::model::WpStatus::Done, crate::contract::Verdict::Blocked { .. }) => format!(
          "{}/WP-{:02} is recorded Done and its gate is BLOCKED -- it reads as finished work and would be refused if closed today",
          thread.id, wp.seq
        ),
        (
          crate::model::WpStatus::NotStarted | crate::model::WpStatus::Wip,
          crate::contract::Verdict::Pass { .. },
        ) => format!(
          "{}/WP-{:02} is recorded {} and its gate PASSES -- every criterion in its scope is satisfied, so anything sequencing off this field is planning work that is already done",
          thread.id,
          wp.seq,
          match wp.status {
            crate::model::WpStatus::NotStarted => "Not Started",
            _ => "WIP",
          }
        ),
        _ => continue,
      };
      // **THE DETAIL STATES WHAT WAS OBSERVED AND NOT WHY.** Its first version
      // said the close was consulted against a contract that has since GROWN --
      // which is the usual cause, is what happened to ST0056/04, and is not
      // something this arm measures. A real finding is the best possible cover
      // for an invented mechanism attached to it; the remedy on the class names
      // the causes as possibilities, where a detail line reads as a reading.
      out.push(Finding::new(
        file.clone(),
        FindingClass::StatusGateDisagreement,
        detail,
      ));
    }
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

  // **An attachment's `bytes` and `sha256` DESCRIBE its `text`, and this is
  // where that stops being guaranteed.**
  //
  // `Attachment::new` is the only constructor and derives both, so nothing in
  // this codebase can make them disagree. **Deserialisation is not in this
  // codebase's gift**: `thread.json` is a file, a file can be edited, and serde
  // will happily read three fields that contradict each other.
  //
  // Reported rather than recomputed, for the reason the whole estate keeps
  // relearning. Silently fixing the hash makes the record agree with itself and
  // destroys the only evidence that something wrote a value it should not have
  // -- and a stored hash that no longer describes its content is exactly what
  // the skew check will later trust.
  for a in &thread.attachments {
    // **An OPAQUE attachment reaches here with NEITHER half when its sidecar
    // was never loaded, and that is a finding rather than a skip.** Canon
    // names bytes at `.canon/st/<ID>/<path>`; if nothing put them in the
    // model, the record describes a file this process cannot produce. Passing
    // over it would report the one attachment whose content nobody can eyeball
    // as the one with nothing to say about it.
    let Some(content) = a.as_bytes() else {
      add(
        format!(
          "attachment {} is opaque and carries no bytes -- canon names {} byte(s) at sha256 {} \
           and the sidecar was not loaded",
          a.path, a.bytes, a.sha256
        ),
        FindingClass::ModelInconsistent,
      );
      continue;
    };
    let actual = crate::model::sha256_hex(content);
    if actual != a.sha256 {
      add(
        format!(
          "attachment {} carries sha256 {} and its content hashes to {actual}",
          a.path, a.sha256
        ),
        FindingClass::ModelInconsistent,
      );
    } else if a.bytes as usize != content.len() {
      add(
        format!(
          "attachment {} records {} bytes and its content is {}",
          a.path,
          a.bytes,
          content.len()
        ),
        FindingClass::ModelInconsistent,
      );
    }
  }

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
        FindingClass::Advisory,
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
        FindingClass::Advisory,
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
  //
  // **A THREAD WITH NO WORK PACKAGES AT ALL IS EXEMPT, and that is a model
  // correction rather than a suppression.** Grouping criteria BY work package
  // is a convention used where work packages exist; in a thread that has none,
  // the group number is a bare grouping device and `AC-01.1` never referenced a
  // WP-01, because there was never one to reference. Six threads in this estate
  // are built that way -- ST0043, ST0044, ST0045, ST0046, ST0050, ST0051 --
  // carrying 72 such rows between them, and v2 accepted every one.
  //
  // **The check keeps the value it actually had.** Measured on the hoisted
  // estate before the clause landed: of 72 orphan-group findings, 72 came from
  // threads with zero work packages and NONE came from the 37 threads that
  // carry them. So a group naming a missing WP in a thread that uses WPs is
  // still a genuine inconsistency and is still reported -- the clause removes
  // no finding this estate was ever entitled to raise.
  //
  // vc is correcting `data-model.md:193` ("group = WP seq or `00` for
  // ST-level") to match, which was written from ST0056's shape rather than
  // from the estate's.
  for criterion in &thread.criteria {
    if let Some(seq) = group_seq(&criterion.id)
      && seq != 0
      && !thread.wps.is_empty()
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

  // Completion and status must agree in both directions, and the predicate on
  // both sides is `is_closed` rather than an equality with `Completed`. The
  // field records an END, and `Cancelled` is an end -- the facade already says
  // so by CLEARING the date on reopen (`cli_end_to_end.rs:329`), which is a
  // statement about ending rather than about completing.
  //
  // Asking the same question on both arms is what gains the dateless-cancelled
  // case, which an equality could not reach: it sat between the arms, flagged
  // by neither, because one arm asked only about `Completed` and the other
  // excluded everything else. A thread cancelled with no date recorded when
  // is a real inconsistency and nothing reported it.
  match (thread.status, &thread.completed) {
    (status, None) if status.is_closed() => add(
      format!(
        "{} is {} with no completion date",
        thread.id,
        status.display()
      ),
      FindingClass::ModelInconsistent,
    ),
    (status, Some(date)) if !status.is_closed() => add(
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
  // **`doctor` ASKS THE MANIFEST, and until now it did not.** The three-state
  // answer already existed for exactly this reader -- `realised_threads`'s own
  // doc said *"the value exists here so that `doctor` can ask; that it does not
  // yet ask is a gap, not a design"* -- and the gap cost 234 false findings the
  // evening the estate dehydrated.
  //
  // Fail-open by construction: an absent OR unreadable manifest declares
  // everything, so a project that has never run `organize` is checked exactly as
  // it was before this landed.
  let realised = crate::intentfiles::realised(&project.intentfiles_path());
  let skew = views::skew(project, canon, ctx, &realised);
  report.views_checked = views::render_all(project, canon, ctx).len();
  report.findings.extend(skew);

  // **Every file under a thread that nothing in the model holds, named.** The
  // classifier is `Project::classify` rather than a list repeated here: a
  // second opinion about what an attachment is would let a file be carried by
  // ingest and reported as uncovered by this, or the reverse and worse.
  for thread in &canon.threads {
    for rel in project.thread_files(&thread.id) {
      if crate::project::Project::classify(&rel) != crate::project::ThreadFile::Attachment {
        continue;
      }
      let path = project.thread_dir(&thread.id).join(&rel);
      // **THE SAME QUESTION, ASKED OF THE SAME FUNCTION AS THE CARRIER.**
      // `within_attachment_cap` has one home for exactly this pair of callers:
      // if this surface and `collect_attachments` ever disagreed, a file would
      // be refused by the carrier and unlisted here -- the silent gap this
      // report exists to close, arriving through the door built to close it.
      // A comparison written out twice is that disagreement waiting to happen.
      let Ok(meta) = std::fs::metadata(&path) else {
        continue;
      };
      if !crate::project::within_attachment_cap(meta.len()) {
        report.unattached.push(format!(
          "{} ({} bytes, over the {}-byte cap)",
          project.relative(&path),
          meta.len(),
          crate::project::ATTACHMENT_CAP_BYTES
        ));
      }
    }
  }
  report.unattached.sort();

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
      attachment_drift(project, canon, &entries, report);
    }
    Err(e) => report.findings.push(Finding::new(
      project.relative(project.root()),
      FindingClass::UnknownFileShape,
      format!("the working tree could not be scanned: {e}"),
    )),
  }
}

/// **An attachment whose working copy no longer matches the bytes canon
/// records for it, reported BY PATH** (ST0057 AC-03.4).
///
/// # By comparison, which is the criterion's own word
///
/// The hash on disk against the hash in canon. Nothing here consults mtime,
/// size, or whether anything was recently written -- every one of those infers
/// change from a proxy, and a proxy that says "unchanged" is exactly the answer
/// that lets a divergence through. `sync::scan` has already hashed every file
/// in the corpus, so this costs no read: the SOLE identity test its own
/// docstring names is the one being compared.
///
/// **One hasher, and it is not this function.** `organize::observe` builds its
/// `sha256` map from the same `scan`, so doctor and organize cannot come to
/// different views of whether a file has moved. A second hash computed here
/// would be a second opinion about identity, which is how one surface reports
/// drift and the other silently plans a removal over it.
///
/// # An ABSENT attachment is not drift, and getting that wrong would be worse
/// than missing the drift
///
/// Under `.intentfiles` a dehydrated thread's attachments are legitimately gone
/// from disk -- that is the feature, not a fault. Reporting absence as
/// divergence would make `doctor` unhealthy for every dehydrated thread in the
/// estate, which is a check that flags a population behaving correctly, and the
/// shape that gets a check deleted rather than fixed.
///
/// **So absence is silence here, and that silence is bounded rather than
/// blanket**: `Report::unattached` and the ship gate speak to files that ought
/// to be present, and AC-03.1 refuses a canon whose opaque bytes cannot be
/// obtained. This function answers one question -- of the attachments that ARE
/// realised, do they still say what canon says they say.
fn attachment_drift(
  project: &Project,
  canon: &Canon,
  entries: &[crate::sync::FileEntry],
  report: &mut Report,
) {
  let on_disk: std::collections::BTreeMap<&str, &crate::sync::FileEntry> =
    entries.iter().map(|e| (e.path.as_str(), e)).collect();

  for thread in &canon.threads {
    for att in &thread.attachments {
      let path = project.thread_dir(&thread.id).join(&att.path);
      let rel = project.relative(&path);
      // Dehydrated, or outside the corpus. Neither is this check's business.
      let Some(entry) = on_disk.get(rel.as_str()) else {
        continue;
      };
      if entry.sha256 == att.sha256 {
        continue;
      }
      report.findings.push(Finding::new(
        &rel,
        FindingClass::AttachmentDrift,
        format!(
          "the working copy hashes to {} and canon records {} for it. An attachment is authored, \
           so nothing can re-derive either side: one of them is an edit somebody made and the \
           other is what the store will write over it",
          entry.sha256, att.sha256
        ),
      ));
    }
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
/// Ops the LOG holds that this binary does not DECLARE (vc's F1, the live half).
///
/// **THIS CONVERTS AN OPEN DESIGN QUESTION FROM A FORECAST INTO A NUMBER.**
/// Whether the op vocabulary should become a type is hv's to rule on, and the
/// argument for it turns on whether a parse would ever meet a string it does
/// not know. Nobody could answer that from the source, because the answer lives
/// in the logs of real estates -- so it kept being re-derived by hand. This
/// answers it on every `doctor` run, on whatever estate is in front of you.
///
/// # Why it is an advisory and not a fault
///
/// An op this binary does not declare is not a broken estate. History is
/// append-only and correct: the row records what really happened, under a name
/// some earlier build used. Nothing is blocked, nothing needs repairing, and
/// there is no remedy to offer -- so counting it would make `doctor` exit 1 on
/// an estate with nothing wrong with it, which is the failure mode hv already
/// ruled on for the 66 advisory notes on Baize.
///
/// # Why it can never be a commit gate, stated so nobody moves it
///
/// It reads `intent/.cache/`, which is gitignored and per-machine. No clone
/// inherits a single one of these rows and a fresh machine has none, so a gate
/// keyed on this would pass or fail according to whose laptop ran it. `doctor`
/// is the home for a per-machine truth check; CI is not.
///
/// # What a clean result does and does not mean
///
/// It means every op in THIS store is one this binary can write. It does NOT
/// mean the roster is complete -- see [`crate::event::KNOWN_OPS`] for the gap
/// it cannot see and for the two source-side tests that cover the directions
/// this cannot.
fn undeclared_op_findings(store: &crate::store::Store) -> Vec<Finding> {
  // **A STORE THAT WILL NOT ANSWER IS SILENT, NOT A FINDING.** Every other
  // stage of `diagnose` degrades rather than refuses, and an unreadable event
  // table is a normal state on a store mid-migration. Reporting "0 undeclared"
  // would be worse than saying nothing: it is a clean bill of health from a
  // query that never ran.
  let Ok(census) = store.op_census() else {
    return Vec::new();
  };
  census
    .into_iter()
    .filter(|(op, _)| !crate::event::KNOWN_OPS.contains(&op.as_str()))
    .map(|(op, count)| {
      Finding::new(
        "intent/.cache/intent.db",
        FindingClass::Advisory,
        format!(
          "the event log holds {count} row(s) under the op `{op}`, which this build does not declare. \
           Two things produce that and they are told apart by looking, not by this line: an op that was \
           RENAMED OR RETIRED and left history behind it, or one this build writes and has not been added \
           to `event::KNOWN_OPS`. Nothing is broken either way -- the log is append-only and the row is a \
           true record of what happened. It is reported because the first case is the one open question \
           about the op vocabulary that could not be answered from source."
        ),
      )
    })
    .collect()
}

/// What the gate's state is, decided from TEXT alone.
///
/// **SEPARATED FROM THE IO SO THE VERDICT CAN BE DRIVEN TO EVERY ARM** -- the
/// estate's thin-coordinator rule (`IN-AG-THIN-COORD-001`) applied where it
/// pays rather than where it is tidy. Two arms are otherwise unreachable from a
/// test: one needs a machine with no resolvable install, and one needs a
/// carrier byte-identical to whatever template that machine happens to hold. **A
/// check about estates being silently unprotected must not itself have arms
/// verified only by inspection.**
#[derive(Debug, PartialEq, Eq)]
pub enum GateState {
  /// No carrier and nothing referencing one: no Intent gate here, which is a
  /// choice rather than a fault.
  NotInstalled,
  /// The chain calls a carrier that does not exist.
  ChainCallsAMissingCarrier,
  /// A carrier that names no guard runner. The Baize state.
  CarrierRunsNoGuards,
  /// Guards are read live from an install this machine cannot resolve.
  NoResolvableInstall,
  /// Installed, running, and older than the template it was copied from.
  BehindTheTemplate { carrier: usize, template: usize },
  /// Installed and current.
  Current,
}

pub fn gate_state(carrier: Option<&str>, chain: Option<&str>, template: Option<&str>) -> GateState {
  let Some(carrier) = carrier else {
    // **THE DISCRIMINATOR IS WHETHER ANYTHING REFERENCES THE CARRIER**, not
    // whether it exists. Two absent files are an estate that opted out; a chain
    // calling a file that is not there is an estate that believes it is
    // protected and is not.
    return if chain.is_some_and(|t| t.contains("pre-commit.intent")) {
      GateState::ChainCallsAMissingCarrier
    } else {
      GateState::NotInstalled
    };
  };
  // **THE MARKER IS THE GUARD RUNNER THE CARRIER MUST REACH**, not a version
  // string and not a banner. A carrier can carry every comment the template has
  // and still run nothing; what makes guards execute is this path.
  if !carrier.contains("pre-commit-guards.sh") {
    return GateState::CarrierRunsNoGuards;
  }
  let Some(template) = template else {
    return GateState::NoResolvableInstall;
  };
  // **BYTES, NOT A VERSION.** The carrier is an untracked per-machine copy
  // taken at install time, so the only thing that says whether it is the
  // current one is whether it IS the current one.
  if template != carrier {
    return GateState::BehindTheTemplate {
      carrier: carrier.len(),
      template: template.len(),
    };
  }
  GateState::Current
}

/// **AN ESTATE HAS NO WAY TO LEARN THAT ITS COMMIT GATE IS NOT RUNNING** (vc,
/// 2026-08-27), and until this landed `doctor` was one of the surfaces telling
/// it everything was fine.
///
/// Found on Baize: config `3.0.0`, canon present, fully ported, four whiteboard
/// nodes, and a `pre-commit.intent` carrying no guard block whatsoever. `doctor
/// --verbose` there printed 139 lines with ZERO mentions of `hook`, `gate`,
/// `guard`, `INTENT_HOME` or `pre-commit`. Nobody noticed, because an unwired
/// guard does not fail -- it reports NOTHING, which is indistinguishable from
/// having nothing to report.
///
/// # The severity is split by PROPERTY, and that split is the whole design
///
/// The four properties do not share one severity, and giving them one is what
/// would have made this check useless:
///
/// - **installed and cannot execute -> [`FindingClass::GateNotRunning`], which
///   is ACTIONABLE.** Reds 2 of 17 across the fleet. Something is broken and
///   the operator is committing ungated.
/// - **behind the template -> [`FindingClass::Advisory`], printed and not
///   counted.** Reds **17 of 17** today: dc proved by `cmp` that the current
///   template is installed in ZERO estates, Intent's own carrier included. **A
///   check that reds every estate in the fleet permanently is one operators
///   learn to skip**, and a skipped check is not there for the two estates that
///   are actually broken either.
///
/// # An ABSENT carrier is deliberately not a finding at all
///
/// Nothing here demands that a project HAVE a gate. A project that never
/// installed one is not broken, and reporting it would fault every
/// non-adopting estate for a choice it made. **The discriminator is whether
/// something REFERENCES the carrier**: a chain that calls `pre-commit.intent`
/// while no such file exists is broken; two absent files are a project that
/// opted out.
///
/// # WHERE THIS CHECK RUNS IS ANTI-CORRELATED WITH WHERE THE DEFECT LIVES
///
/// **A reader meeting `0 findings` fleet-wide will not infer this, so it is
/// said here rather than only in the test.** [`diagnose`] returns at the
/// migration arm before this runs, so an UNPORTED estate is never examined at
/// all -- and an unported estate is exactly the one least likely to have a
/// working gate. That is not a coverage gap, which would be neutral about
/// which cases it missed; it is a systematic bias toward missing the ones that
/// matter.
///
/// Measured on Conflab, 2026-08-27: `intent doctor` reports `1 finding across
/// 0 thread(s), 0 issue(s), 0 view(s), **0 file(s)**`. Zero files scanned. A
/// newer binary does not change it and neither does this check; only the port
/// does.
///
/// **The limit is DRIVEN, not merely written here** --
/// `gate_not_running_is_reported::an_unmigrated_estate_is_never_reached_and_that_is_the_limit_not_a_pass`
/// reds if the migration arm is ever moved below this call, so whoever moves it
/// learns that a check they had not considered now runs on estates it was never
/// measured against. A limit recorded only in prose is one that stops being
/// true without anyone noticing.
///
/// # No verb repairs any of it, and the findings say so
///
/// No v3 code path writes the carrier. `intent claude upgrade --apply` writes
/// canon and region-edits the chain block; vc drove its dry run to confirm the
/// carrier is not on its list. **So this check makes the rot VISIBLE, not
/// fixable** -- which is a good finding, where saying nothing is how Baize got
/// where it is. The one thing it must not do is offer a command that does not
/// work: `bin/devbin hooks` already prints `dispatcher STALE` and then names a
/// remedy that vc measured does not write the carrier, and running a remedy
/// that changes nothing reads as repair.
fn hook_findings(project: &Project) -> Vec<Finding> {
  let root = project.root();
  // `--git-path hooks` rather than `config core.hooksPath` or a literal
  // `.git/hooks`: it honours the redirect AND a linked worktree in one call,
  // and it is the same resolution the shipped chain itself uses. A hand-rolled
  // version would agree with git on the common case and disagree in exactly the
  // layouts that produce an unwired guard.
  let Ok(out) = std::process::Command::new("git")
    .args([
      "-C",
      &root.display().to_string(),
      "rev-parse",
      "--git-path",
      "hooks",
    ])
    .output()
  else {
    // **NOT A FINDING.** No git, or no repository: `doctor` runs on trees that
    // are neither, and a missing gate is not a defect of a directory that
    // cannot have hooks in the first place.
    return Vec::new();
  };
  if !out.status.success() {
    return Vec::new();
  }
  let hooks = root.join(String::from_utf8_lossy(&out.stdout).trim());
  let carrier_path = hooks.join("pre-commit.intent");
  let chain_path = hooks.join("pre-commit");
  let shown = |p: &std::path::Path| project.relative(p);

  let carrier = std::fs::read_to_string(&carrier_path).ok();
  let chain = std::fs::read_to_string(&chain_path).ok();
  let template = crate::install::home()
    .ok()
    .and_then(|home| std::fs::read_to_string(home.join("lib/templates/hooks/pre-commit.sh")).ok());

  match gate_state(carrier.as_deref(), chain.as_deref(), template.as_deref()) {
    GateState::NotInstalled | GateState::Current => Vec::new(),
    GateState::ChainCallsAMissingCarrier => vec![Finding::new(
      shown(&chain_path),
      FindingClass::GateNotRunning,
      format!(
        "the pre-commit chain calls `{}` and no such file exists, so every guard it would have run is silently skipped on every commit",
        shown(&carrier_path)
      ),
    )],
    GateState::CarrierRunsNoGuards => vec![Finding::new(
      shown(&carrier_path),
      FindingClass::GateNotRunning,
      "the hook carrier is present and names no guard runner at all, so it executes no guards -- this is the Baize state, in which every surface reports health while nothing is enforced".to_string(),
    )],
    GateState::NoResolvableInstall => vec![Finding::new(
      shown(&carrier_path),
      FindingClass::GateNotRunning,
      "the hook carrier reads its guard roster live out of the Intent install and this machine cannot resolve one, so the carrier runs and finds no guards to run".to_string(),
    )],
    GateState::BehindTheTemplate { carrier, template } => vec![Finding::new(
      shown(&carrier_path),
      FindingClass::Advisory,
      format!(
        "the hook carrier is {carrier} byte(s) and the template in the resolved install is {template} -- the carrier is a copy taken at install time and nothing re-copies it, so the guards it runs are the generation it was installed with. Reported and NOT counted: measured across the fleet this is true of every estate, and a finding that is permanently true everywhere is one nobody reads"
      ),
    )],
  }
}

fn backup_findings(project: &Project, store: &crate::store::Store) -> Vec<Finding> {
  let Ok(age) = store.hours_since_last_good_snapshot() else {
    // The store is open but the query failed, which is not a backup problem
    // and must not be reported as one.
    return Vec::new();
  };
  let where_ = project.relative(&crate::backup::snapshot_dir(project));
  let mut findings = Vec::new();

  // **The schedule is read first and reported on its own terms.** A value
  // outside the closed vocabulary silences the staleness COMPARISON and nothing
  // else -- "no snapshot has ever been taken" is not a lateness claim, needs no
  // period to be true, and so must not be gated on reading one. Suppressing it
  // here would be the switch that silences backup failure, arrived at by
  // accident instead of by a key.
  let every = match crate::backup::schedule(project) {
    crate::backup::Schedule::Hours(hours) => Some(f64::from(hours)),
    crate::backup::Schedule::Unrecognised(value) => {
      findings.push(Finding::new(
        where_.clone(),
        FindingClass::SchemaInvalid,
        format!(
          "backup.schedule is {value:?}, which is not one of hourly, daily, weekly \
           -- the newest snapshot's age cannot be judged until it is corrected"
        ),
      ));
      None
    }
  };

  match age {
    // **Never is its own message, not a very large number.** "no restorable
    // snapshot has ever been taken" and "the last one is old" call for
    // different actions -- the first says the mechanism has never run, the
    // second says it has stopped -- and collapsing them loses exactly the
    // distinction this check was added for.
    None => findings.push(Finding::new(
      where_,
      FindingClass::BackupStale,
      "no restorable snapshot has ever been taken of this store".to_string(),
    )),
    // A schedule is a period, not a deadline, so a backup is not late the
    // instant the period elapses. Twice the period is late by any reading, and
    // it keeps a daily schedule from reporting RED every morning before it
    // runs.
    Some(hours) => {
      if let Some(every) = every
        && hours > every * 2.0
      {
        findings.push(Finding::new(
          where_,
          FindingClass::BackupStale,
          format!(
            "the newest restorable snapshot is {hours:.0}h old against a {every:.0}h schedule"
          ),
        ));
      }
    }
  }

  findings
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
