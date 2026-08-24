//! **WP-10 Phase B -- the join** (migration.md).
//!
//! Phase A ([`crate::legacy`]) reads a v2 estate and writes nothing. This turns
//! what it read into the complete set of files a migrated project holds --
//! structured canon and regenerated views -- and **writes nothing either**.
//!
//! **The uncommitted return is the atomicity, not a step towards it.** AC-10.2
//! requires that a blocked migration leave the estate untouched, and a function
//! that CANNOT write cannot half-write: the guarantee is a property of the
//! signature rather than of anyone getting an early return in the right order.
//! The caller commits the batch, and [`crate::write_set::Applied`] can still
//! undo it if a later step fails.
//!
//! **The order beyond this point is the facade's and it is load-bearing** (cc,
//! carried from v2's ST0043): plan -> commit the writes -> rebuild the DB ->
//! converge gitignore -> **stamp the version LAST**. A stamp written before the
//! canon exists makes [`crate::project::Project::migration`] stop reporting the
//! project as unmigrated, so a half-finished migration would look finished --
//! the failure would have destroyed the one piece of state that says what went
//! wrong.
//!
//! **STAMP LAST STOPPED BEING DISCIPLINE AND ACQUIRED A MECHANISM ON
//! 2026-08-17** (cc, from hv's fix to dc's two-ended-migration finding). v2 now
//! REFUSES to operate on a project declaring a newer Intent than itself, and v3
//! refuses one declaring older. The two requirements are contradictory on one
//! estate and both are correct -- but it means that **between the moment
//! `config.json` says 3.0.0 and the moment the canon is complete, NEITHER tool
//! will touch the project.** A premature stamp does not merely hide the
//! migration state; it locks the estate out of both toolchains at once. The
//! ordering above was an argument until today and is now enforced by a real
//! refusal.
//!
//! # What this deliberately does NOT do, and why each is recorded rather than
//! # quietly omitted
//!
//! **It writes no events.** data-model.md says a migrated thread "restores an
//! `st.new` carrying the authored date", and doing that means inventing a
//! time-of-day for a `created:` that is a DATE -- [`crate::event::Envelope`]'s
//! `ts` is millisecond-precision by contract, because two machines merge logs
//! and order them by it. Manufacturing a precise value from an imprecise one is
//! D42's subject exactly, so the synthesis is a ruling rather than a coding
//! decision. **Nothing is lost by waiting**: `created` and `completed` are
//! carried in `thread.json` itself, so what is absent is the HISTORY, not the
//! dates. The empty `events.jsonl` this emits says "no recorded history", which
//! is a true statement about a v2 estate that never had a log.
//!
//! **It does not relocate a thread out of v2's `COMPLETED/` / `CANCELLED/` /
//! `NOT-STARTED/` buckets** -- see [`plan`]'s own note. That one is a hole, it
//! is measured, and it is not silently absent.

use std::collections::BTreeSet;
use std::path::PathBuf;

use crate::export::{self, Bundle};
use crate::facade::FacadeContext;
use crate::finding::{Finding, FindingClass, Refusal};
use crate::ingest::Canon;
use crate::legacy::Scan;
use crate::model::{Issue, Thread};
use crate::project::Project;
use crate::views::{self, RenderContext};
use crate::write_set::WriteSet;

/// Everything the migration would do, with nothing done yet.
#[derive(Debug)]
pub struct Plan {
  /// Canon and views, **uncommitted**. The caller commits it.
  pub writes: WriteSet,
  /// The model, for the store rebuild the facade runs after the writes land.
  pub threads: Vec<Thread>,
  pub issues: Vec<Issue>,
  /// Phase A's carried findings, passed through untouched.
  ///
  /// **They travel with the plan rather than being printed here**, because a
  /// plan that reported to a stream could not be assembled by a caller that
  /// wanted to inspect it first -- and the report belongs to the door, which
  /// knows whether anyone is watching.
  pub carried: Vec<Finding>,
  /// Threads Phase A took from `thread.json` rather than from their markdown,
  /// because they were already migrated.
  ///
  /// **For the REPORT and deliberately not for control flow.** They are in
  /// [`Plan::threads`] like any other, so the plan re-emits byte-identical
  /// canon and byte-identical views for them and the two global views render
  /// from the whole estate. Skipping them here would be the truncation this
  /// design exists to avoid, one layer down: `steel_threads.md` and `todo.md`
  /// are rendered from the full thread list, so a plan that dropped 54 of 56
  /// would rewrite the index with two rows while reporting `2 converted, 54
  /// already` -- every word true.
  ///
  /// **What it buys is a re-run that says what it did.** `already_migrated +
  /// converted` reconciles against the estate, so doing less stops being
  /// indistinguishable from there being less (cc, `legacy.rs`'s own rule that
  /// a silent skip is how an artefact disappears from a migration whose whole
  /// promise is that nothing does).
  pub already_migrated: Vec<String>,
  /// Issue numbers read back from committed canon rather than converted from a
  /// v2 estate -- [`already_migrated`](Plan::already_migrated)'s counterpart
  /// over the population intent#0070 dropped.
  ///
  /// **For the REPORT, and there is no control-flow twin to resist here**: the
  /// thread field carries that warning because a reader might be tempted to
  /// skip re-emitting; nobody can skip these, because re-emitting them IS the
  /// fix.
  pub already_migrated_issues: Vec<u32>,
  /// Sections Phase A dropped as template scaffolding, passed through
  /// untouched. **The third summary bucket beside modelled and carried**: a
  /// drop with no record cannot be told from a section that was never there.
  pub dispositions: Vec<crate::legacy::Disposition>,
}

/// Why no plan exists.
#[derive(Debug, thiserror::Error)]
pub enum Blocked {
  /// Live-thread residue: the estate must be repaired under v2 tooling first.
  ///
  /// **Carried as a [`Refusal`] rather than a bare `Vec<Finding>`** so the
  /// per-line classed report AC-10.2 asks for arrives with the error instead of
  /// being re-implemented at whichever door renders it. Not `#[source]`, for
  /// the reason [`crate::ingest::IngestError::Refused`] records: a source here
  /// renders the whole list twice and every residue count reads double.
  #[error("{0}")]
  Residue(Refusal),
  /// The model would not serialise.
  ///
  /// **A defect in Intent, not in the estate, and it says so** -- everything
  /// here was built in memory from data that already parsed, so there is no
  /// artefact for an operator to go and repair. Reported rather than unwrapped
  /// because a panic in the middle of a migration is the one failure mode with
  /// no message and no remedy.
  #[error("the migration's own canon would not serialise")]
  Canon {
    #[source]
    source: serde_json::Error,
  },
  /// Two artefacts resolve to one file.
  ///
  /// **Nothing below this guards it today.** [`WriteSet`] applies its entries
  /// in order, so a duplicated path writes both and keeps the last -- one
  /// artefact silently overwriting another, inside the operation whose whole
  /// promise is that nothing disappears. It would land as a clean migration
  /// with a file missing and every count reconciling.
  ///
  /// **The detection belongs in [`WriteSet::add`] and cc is taking it there**
  /// (2026-08-17): every caller needs uniqueness, so a set that permits
  /// duplicates is wrong on its own terms rather than wrong for this one use.
  /// The variant stays here regardless -- `WriteSet` can say the batch is
  /// malformed and only the join can say WHICH artefacts collided -- and the
  /// scan below is deleted the day `add` becomes fallible, so the two do not
  /// settle into one check nobody can reach.
  #[error("two artefacts would be written to {path}")]
  Collision { path: String },
  /// The canon is short of one file per artefact.
  ///
  /// **Counted from the model rather than from the exporter's output**, which
  /// is the only way the check can disagree with the thing it is checking: one
  /// `thread.json` per thread, one `<n>.json` per issue, one event log. An
  /// equality derived from [`export::canon_parts`] would agree with
  /// `canon_parts` by construction and report a silent drop as correct.
  #[error("the canon carries {actual} file(s) for {expected} artefact(s)")]
  Conservation { expected: usize, actual: usize },
}

impl crate::remedy::Remedy for Blocked {
  fn remedy(&self) -> String {
    match self {
      // The findings above have each named their own artefact and action;
      // `Refusal` already says so in the words the rest of the tool uses.
      Self::Residue(refusal) => crate::remedy::Remedy::remedy(refusal),
      Self::Canon { .. } => {
        "nothing in the project needs repair and a re-run will reproduce this -- report it with the cause line above, which names the field that would not serialise".to_string()
      }
      Self::Collision { path } => format!(
        "two artefacts in the v2 estate resolve to {path}; rename or remove one of them under v2 tooling, then re-run the migration"
      ),
      Self::Conservation { expected, actual } => format!(
        "{} artefact(s) produced no canon; nothing in the project caused this and a re-run will reproduce it -- report it",
        expected.saturating_sub(*actual)
      ),
    }
  }
}

/// Turn a Phase A scan into the batch a migration would write.
///
/// Residue blocks. Everything else assembles: canon from
/// [`export::canon_parts`], views from [`views::render_all`], both returned
/// UNCOMMITTED.
///
/// # The bucket hole, named here because it is the join's and it is measured
///
/// v2 relocates a thread's directory on a status transition, so a completed
/// thread lives at `st/COMPLETED/<ID>/` while v3's canonical path is
/// `st/<ID>/thread.json` (data-model.md) and
/// [`Project::thread_ids`](crate::project::Project::thread_ids) reads only the
/// flat level. Phase A finds a bucketed thread and **discards where it was** --
/// [`Thread`] has no directory -- so this cannot move it even in principle.
///
/// Measured on the canary: **55 of 56 threads are bucketed, 386 tracked
/// files**, and a migration writes fresh canon plus regenerated views at the
/// flat path while every one of those files stays put. They split in two.
/// **194 are regenerated at the flat path** (55 thread `info.md`, 126 WP
/// `info.md`, 13 `acceptance.md`). **192 are authored prose the model does not
/// hold and nothing regenerates** -- 54 `design.md`, 54 `impl.md`, 54
/// `tasks.md` and 30 one-offs -- left at a path the model does not know about.
/// Verified independently by vc's path classifier, which shares no code with
/// this one and reads the estate for a different reason: 387/194/193 at their
/// pinned `42fb5269`, the one-file delta being the estate moving between that
/// revision and HEAD.
///
/// **The regenerated half is not "merely doubled" -- the migration MANUFACTURES
/// the 0011 class** (vc, 2026-08-17, and it is the sharper reading). Two
/// `info.md` per thread, one rendered from the model and one v2 artefact that
/// nothing regenerates and everything still reads: two artefacts claiming one
/// natural role at two paths, which is precisely what [`crate::legacy`] refuses
/// when it finds it in an estate. **The migration would create what the parser
/// exists to reject, on 55 of 56 threads.** A reachability check reports the
/// flat path reachable and stops, so this is a third question and not a
/// consequence of the second.
///
/// **Nothing is deleted, which is what makes it hard to see: every count
/// reconciles and every file is still present.** The loss is of reachability,
/// not of bytes, so a conservation check comparing two file listings agrees
/// with itself. AC-10.5's prose conservation has to be an equality between the
/// prose the estate CONTAINED and the prose reachable from the migrated MODEL.
///
/// The two numbers beside each other are the whole finding. Run against the
/// canary at `14c3fb01` this plans **311 files** -- 56 `thread.json`, one
/// event log, 254 views -- and reports no block. **386 files stay in the
/// buckets.** Both figures are correct and the migration is complete by every
/// measure it takes of itself.
///
/// **hv retired `intent organize` on the argument that "a strictly structured
/// model cannot hold data in the wrong spot, so the disorder this repairs
/// cannot arise"** (2026-08-14). The migration is what would make that true,
/// and it does not: the disorder is carried in wholesale and the tool that
/// repaired it is gone. Recorded rather than worked around, because flattening
/// means MOVING authored prose and [`WriteSet`] has no remove -- so the
/// mechanism is a decision, not an oversight to patch here.
/// **Every thread with committed canon that the disk walk never reached.**
///
/// `legacy::scan` derives its population from `thread_dirs` -- the directories
/// under `intent/st/`. Under ST0057 the disk is a SPARSE PROJECTION: a
/// dehydrated thread has canon and no directory, so it never enters that loop.
/// Measured 2026-08-20 on a fresh clone: 57 threads in canon, 3 realised, and
/// `upgrade` rebuilt the store and the project-level views from the 3 --
/// **deleting 239 lines from two committed views and cutting the durable SSOT
/// from 57 rows to 3, at rc=0, under an `ok:`.**
///
/// # Why here and not in `legacy::scan` (vc ruled, 2026-08-20)
///
/// **Highlander forbids two answers to ONE question; it does not require one
/// answer to TWO.** `legacy::scan` answers *what must be converted from v2*;
/// `Project::thread_ids` answers *what threads exist*. Both are correct. The
/// defect was that `upgrade` asked the first when it needed the second, so the
/// union belongs at the join and not inside either.
///
/// **And the scan's other caller is the proof.** `Facade::ingest_from_md` reads
/// a markdown estate, and **a canon-only thread has no markdown to read** --
/// widening the scan would make a correct function wrong at its other call
/// site, and every residue class in `Scan`, all of which describe something a
/// v2 AUTHOR left behind, would acquire a population it must specially ignore.
///
/// # An unreadable canon REFUSES rather than being skipped
///
/// A skipped file is the defect again one level down: the union would be
/// silently incomplete and the rebuild would run off it. A v2 estate has no
/// canon at all, so `thread_ids` answers empty and this adds nothing -- the
/// refusal is reachable only where canon exists and cannot be read, which is
/// exactly when rebuilding the SSOT from it must not proceed.
fn dehydrated(
  project: &Project,
  seen: &BTreeSet<String>,
) -> Result<(Vec<Thread>, Vec<String>), Blocked> {
  let Ok(ids) = project.thread_ids() else {
    return Ok((Vec::new(), Vec::new()));
  };
  let mut threads = Vec::new();
  let mut added = Vec::new();
  let mut residue = Vec::new();
  for id in ids {
    if seen.contains(&id) {
      continue;
    }
    let path = project.thread_json(&id);
    let rel = project.relative(&path);
    match std::fs::read_to_string(&path) {
      Ok(text) => match crate::ingest::read_thread(project, &id, &text) {
        Ok(thread) => {
          added.push(thread.id.clone());
          threads.push(thread);
        }
        Err(mut found) => residue.append(&mut found),
      },
      Err(source) => residue.push(Finding::new(
        &rel,
        FindingClass::BrokenReference,
        format!(
          "canon names {id} and the file is not readable: {source} -- the migration would rebuild \
           the store and the views without it"
        ),
      )),
    }
  }
  if !residue.is_empty() {
    return Err(Blocked::Residue(Refusal::new(residue)));
  }
  Ok((threads, added))
}

/// **Every issue with committed canon that the v2 scan never reached.**
///
/// The exact counterpart of [`dehydrated`], and its absence WAS intent#0070.
/// `legacy::scan` derives issues from the v2 estate, so on an already-migrated
/// project it finds none; `plan` passed that empty vector straight through, and
/// `store.rebuild` -- which runs off this model -- replaced a populated issue
/// table with nothing. **Threads survived the identical re-run because they had
/// the union above and issues did not: one population topped up from canon, the
/// other not, in adjacent lines.**
///
/// **Measured: a probe holding 5 issues reported `0 issue(s)` and left 0, at
/// rc=0, under an `ok:`** -- and on the live repo it took 47.
///
/// **The report was HONEST, which is what made it invisible.** `0 issue(s)`
/// meant it carried none, and it had carried none. Nothing in the output was
/// false, so there was nothing for an operator to notice -- the reason this
/// could not have been fixed by rewording the line.
///
/// Same three rulings as [`dehydrated`], and they transfer without amendment.
/// The union belongs at the JOIN and not inside `legacy::scan`, whose other
/// caller reads a markdown estate that a canon-only issue has no entry in.
/// `issue_numbers` answers empty for a missing directory, so a genuine v2
/// estate -- no `.canon/` at all -- adds nothing here and the first-run path is
/// untouched. And an unreadable or invalid canon REFUSES rather than being
/// skipped, because a silently incomplete union is this same defect one level
/// down, and the rebuild would run off it.
fn orphaned_issues(
  project: &Project,
  seen: &BTreeSet<u32>,
) -> Result<(Vec<Issue>, Vec<u32>), Blocked> {
  let Ok(numbers) = project.issue_numbers() else {
    return Ok((Vec::new(), Vec::new()));
  };
  let mut issues = Vec::new();
  let mut added = Vec::new();
  let mut residue = Vec::new();
  for number in numbers {
    if seen.contains(&number) {
      continue;
    }
    let path = project.issue_json(number);
    let rel = project.relative(&path);
    match std::fs::read_to_string(&path) {
      Ok(text) => match crate::ingest::read_issue(project, number, &text) {
        Ok(issue) => {
          added.push(issue.number);
          issues.push(issue);
        }
        Err(mut found) => residue.append(&mut found),
      },
      Err(source) => residue.push(Finding::new(
        &rel,
        FindingClass::BrokenReference,
        format!(
          "canon names issue {number} and the file is not readable: {source} -- the migration \
           would rebuild the store and the views without it"
        ),
      )),
    }
  }
  if !residue.is_empty() {
    return Err(Blocked::Residue(Refusal::new(residue)));
  }
  Ok((issues, added))
}

pub fn plan(project: &Project, ctx: &FacadeContext, scan: Scan) -> Result<Plan, Blocked> {
  // **Exhaustive on purpose: a field added to `Scan` must not compile.** `..`
  // here would make a new Phase A output something the join silently drops,
  // which is the exact class this whole operation exists to prevent -- and it
  // would drop it in the one place with no reader to notice.
  //
  // **It fired on the first field to arrive after it was written**
  // (`already_migrated`, cc, 2026-08-17), which is the whole of its case: the
  // compiler stopped the workspace rather than the join quietly ignoring a
  // Phase A output, and the decision about what to do with it was made here
  // instead of defaulting to nothing.
  let Scan {
    threads,
    issues,
    residue,
    carried,
    already_migrated,
    dispositions,
  } = scan;

  if !residue.is_empty() {
    return Err(Blocked::Residue(Refusal::new(residue)));
  }

  // **THE UNION, AND IT IS WHY THIS VERB STOPPED SHRINKING THE ESTATE.**
  // Everything past this point -- `store.rebuild`, `views::render_all`, the
  // canon export -- runs off `threads`, so a thread absent HERE is absent from
  // the SSOT and from the index the migration leaves behind.
  let seen: BTreeSet<String> = threads.iter().map(|t| t.id.clone()).collect();
  let (extra, extra_ids) = dehydrated(project, &seen)?;
  let mut threads = threads;
  threads.extend(extra);

  // **THE SAME UNION, FOR THE SAME REASON, OVER THE OTHER POPULATION.**
  // `store.rebuild(&threads, &issues)` takes both, so an issue absent here is
  // absent from the SSOT exactly as a thread would be -- and for four days it
  // was, because only one of the two lines existed (intent#0070).
  let seen_issues: BTreeSet<u32> = issues.iter().map(|i| i.number).collect();
  let (extra_issues, extra_numbers) = orphaned_issues(project, &seen_issues)?;
  let mut issues = issues;
  issues.extend(extra_issues);

  let mut plan = assemble(project, ctx, threads, issues, carried)?;
  // **They ARE already migrated, so the report says so rather than counting
  // them as conversions.** A dehydrated thread reached this plan from its own
  // committed canon, which is the same warrant the disk-walk arm uses for the
  // threads it re-emits.
  let mut already_migrated = already_migrated;
  already_migrated.extend(extra_ids);
  already_migrated.sort();
  already_migrated.dedup();
  plan.already_migrated = already_migrated;
  // **The union above would otherwise make `migrated: N issue(s)` over-claim on
  // a re-run**, reporting as converted a population that was read back from its
  // own committed canon -- the disclosure threads already get one line down.
  // Fixing the loss and leaving the report asymmetric would rebuild 0070's
  // shape in the layer 0070 called the worst part.
  plan.already_migrated_issues = extra_numbers;
  plan.dispositions = dispositions;
  Ok(plan)
}

/// The whole of Phase B's work, over a model rather than over a scan.
///
/// **PRIVATE, and it was public for an afternoon on a justification that did
/// not survive.** The argument was that AC-10.5's conservation work would want
/// to drive Phase B from a model rather than from a v2 estate. vc built that
/// check in shell against the file estate, so the seam has no second caller --
/// and the case against it is one I had already made to cc that morning, when I
/// asked them to land `Scan.issues` bare precisely so I would not have to add a
/// public function for a reason that expires. Then I added it anyway.
///
/// It stays as a separate function because residue-gating and assembly are
/// different jobs, and `Scan` derives `Default` with public fields, so every
/// test reaches it through [`plan`] without fabricating an estate.
fn assemble(
  project: &Project,
  ctx: &FacadeContext,
  threads: Vec<Thread>,
  issues: Vec<Issue>,
  carried: Vec<Finding>,
) -> Result<Plan, Blocked> {
  let ctx_render = RenderContext {
    version: &ctx.version,
  };

  // `render_all` reads only `threads`, and `sections` is authored prose that
  // stays on disk as files (export.rs: "authored prose is OUT"). Exhaustive
  // for the same reason the `Scan` destructure is.
  let canon = Canon {
    threads,
    issues,
    sections: Vec::new(),
  };
  // **TWO POPULATIONS, AND CONFLATING THEM IS HOW THE FIRST CUT OF THE UNION
  // TRADED ONE DEFECT FOR ANOTHER.** The STORE and the project-level index must
  // cover every thread canon holds; the per-thread files on disk must cover
  // only what `.intentfiles` declares. `render_all` renders both kinds, so the
  // union above -- which is right for the model -- hydrated all 57 threads onto
  // a disk whose manifest declares 3. Measured: 326 files written against 38,
  // and 54 untracked directories appearing under an `ok:`.
  //
  // **NOTHING WAS CONSULTING `.intentfiles` HERE BEFORE, AND IT LOOKED
  // CORRECT.** The scan's disk-walk population happened to EQUAL the declared
  // population -- both were "the threads with directories" -- so the migrator
  // respected the manifest by coincidence. Breaking the coincidence is what
  // showed there was no mechanism.
  //
  // **`Realised::declares` answers TRUE for an absent or unreadable manifest**
  // (ABSENT IS NOT EMPTY, hv), so a v2 estate -- which has no manifest at all
  // -- realises everything exactly as before.
  let realised = crate::intentfiles::realised(&project.intentfiles_path());
  let unrealised: Vec<PathBuf> = canon
    .threads
    .iter()
    .filter(|t| !realised.declares(&t.id))
    .map(|t| project.thread_dir(&t.id))
    .collect();
  let views: Vec<views::View> = views::render_all(project, &canon, &ctx_render)
    .into_iter()
    .filter(|v| !unrealised.iter().any(|dir| v.path.starts_with(dir)))
    .collect();
  let Canon {
    threads,
    issues,
    sections: _,
  } = canon;

  // One thread.json per thread, one <n>.json per issue, one event log --
  // stated from the model, before the exporter is asked anything.
  let expected = threads.len() + issues.len() + 1;

  // `project_id` is empty on a pre-migration project and `Bundle` records that
  // honestly rather than inventing one.
  //
  // **THIS COMMENT USED TO END `the facade mints and stamps it, last`, AND
  // THAT WAS NEVER TRUE** -- not stale, never true. `stamp_version` inserted
  // `intent_version` and nothing else, so the sentence promised a fix, in the
  // one place a reader tracing the empty id would arrive, and sent them away
  // satisfied. It is true as of this commit; it is written here as history
  // because the failure was the PROMISE, and a corrected promise reads exactly
  // like the one that was wrong.
  //
  // **The empty id is inert HERE regardless, which is the part worth knowing
  // and is why the stamp belongs at the facade rather than earlier**:
  // `canon_parts` reads `bundle.threads`, `bundle.issues` and `bundle.events`,
  // and never `bundle.project_id`. Migration passes `Vec::new()` for events, so
  // nothing this bundle emits carries the id at all.
  let bundle = Bundle::new(&ctx.project_id, threads, issues, Vec::new());
  let parts = export::canon_parts(&bundle).map_err(|source| Blocked::Canon { source })?;
  if parts.len() != expected {
    return Err(Blocked::Conservation {
      expected,
      actual: parts.len(),
    });
  }

  // `canon_parts` names its files relative to the intent directory so a
  // refusal can quote a path an operator recognises; the views already carry
  // absolute paths from `Project`.
  let intent_dir = project.intent_dir();
  let mut batch: Vec<(PathBuf, String)> = Vec::with_capacity(parts.len() + views.len());
  batch.extend(
    parts
      .into_iter()
      .map(|(rel, content)| (intent_dir.join(rel), content)),
  );
  batch.extend(views.into_iter().map(|view| (view.path, view.content)));

  {
    let mut seen: BTreeSet<&PathBuf> = BTreeSet::new();
    for (path, _) in &batch {
      if !seen.insert(path) {
        return Err(Blocked::Collision {
          path: project.relative(path),
        });
      }
    }
  }

  let mut writes = WriteSet::new();
  for (path, content) in batch {
    writes.add(path, content);
  }

  Ok(Plan {
    writes,
    threads: bundle.threads,
    issues: bundle.issues,
    carried,
    // `assemble` works over a model and has no idea where any of it came
    // from; `plan` fills this in from the scan. Empty here rather than a
    // parameter, because a fifth argument carrying a value this function
    // never reads would be a seam pretending to be a dependency.
    already_migrated: Vec::new(),
    already_migrated_issues: Vec::new(),
    // Same reason as above: `assemble` never sees a scan, so it cannot know
    // what Phase A dropped.
    dispositions: Vec::new(),
  })
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::finding::FindingClass;
  use crate::model::THREAD_SCHEMA;

  const VERSION: &str = "3.0.0-test";

  fn project(dir: &std::path::Path) -> Project {
    let config = dir.join("intent").join(".config");
    std::fs::create_dir_all(&config).expect("mkdir");
    std::fs::write(
      config.join("config.json"),
      r#"{"intent_version":"2.19.0","intent_dir":"intent"}
"#,
    )
    .expect("config");
    Project::discover(dir).expect("discover")
  }

  fn ctx() -> FacadeContext {
    FacadeContext {
      principal: "ic".to_string(),
      project_id: String::new(),
      version: VERSION.to_string(),
    }
  }

  fn thread(id: &str) -> Thread {
    serde_json::from_value(serde_json::json!({
      "schema": THREAD_SCHEMA,
      "id": id,
      "title": format!("{id} title"),
      "status": "wip",
      "created": "2026-08-17",
    }))
    .expect("thread fixture")
  }

  fn wp(seq: u32) -> crate::model::WorkPackage {
    serde_json::from_value(serde_json::json!({
      "seq": seq,
      "title": format!("WP-{seq:02}"),
      "status": "not-started",
    }))
    .expect("wp fixture")
  }

  /// **Built through serde rather than as a struct literal, deliberately.**
  /// `Issue` is gaining `reporter` (cc, 2026-08-17); a literal here would red
  /// their commit from this file, and a fixture that breaks on a field it does
  /// not care about is a fixture that gets weakened to stop breaking.
  fn issue(number: u32) -> Issue {
    serde_json::from_value(serde_json::json!({
      "schema": crate::model::ISSUE_SCHEMA,
      "number": number,
      "slug": format!("issue-{number}"),
      "title": format!("issue {number}"),
      "status": "closed",
      "created": "2026-08-17",
    }))
    .expect("issue fixture")
  }

  #[test]
  fn residue_blocks_and_the_report_names_every_finding() {
    let dir = tempfile::tempdir().expect("tempdir");
    let project = project(dir.path());
    let scan = Scan {
      threads: vec![thread("ST0001")],
      residue: vec![
        Finding::new("intent/st/ST0001/info.md", FindingClass::UnknownStatus, "a"),
        Finding::new(
          "intent/st/ST0002/info.md",
          FindingClass::UnparseableRow,
          "b",
        ),
      ],
      ..Default::default()
    };

    let Err(Blocked::Residue(refusal)) = plan(&project, &ctx(), scan) else {
      panic!("live residue must block");
    };
    assert_eq!(refusal.findings.len(), 2);
    let report = refusal.to_string();
    assert!(report.contains("ST0001"), "report: {report}");
    assert!(report.contains("ST0002"), "report: {report}");
  }

  #[test]
  fn a_blocked_plan_writes_nothing_because_it_cannot() {
    let dir = tempfile::tempdir().expect("tempdir");
    let project = project(dir.path());
    let before = tree(dir.path());

    let scan = Scan {
      threads: vec![thread("ST0001")],
      residue: vec![Finding::new("x", FindingClass::UnknownStatus, "a")],
      ..Default::default()
    };
    assert!(plan(&project, &ctx(), scan).is_err());

    assert_eq!(before, tree(dir.path()), "a blocked plan touched the tree");
  }

  #[test]
  fn a_clean_scan_plans_canon_and_views_and_still_writes_nothing() {
    let dir = tempfile::tempdir().expect("tempdir");
    let project = project(dir.path());
    let before = tree(dir.path());

    let planned = plan(
      &project,
      &ctx(),
      Scan {
        threads: vec![thread("ST0001"), thread("ST0002")],
        issues: vec![issue(46)],
        ..Default::default()
      },
    )
    .expect("a clean estate plans");

    // Canon: two threads, one issue, one event log. Views: info + acceptance
    // per thread, plus the index and the todo view.
    assert_eq!(planned.threads.len(), 2);
    assert_eq!(planned.issues.len(), 1);
    assert_eq!(planned.writes.len(), 4 + 6);
    // **The equality below is worthless if `tree` sees nothing**, and a
    // before/after comparison of two empty vectors passes for any behaviour.
    // This is what makes the next line an assertion rather than a shape.
    assert_eq!(
      before,
      vec!["intent/.config/config.json".to_string()],
      "the walker must be able to see a file that IS there"
    );
    assert_eq!(
      before,
      tree(dir.path()),
      "planning is read-only and this is the assertion that says so"
    );
  }

  #[test]
  fn carried_findings_pass_through_untouched() {
    let dir = tempfile::tempdir().expect("tempdir");
    let project = project(dir.path());
    let carried = vec![Finding::new(
      "intent/st/COMPLETED/ST0001/info.md",
      FindingClass::FieldNotRecorded,
      "no completed date",
    )];

    let planned = plan(
      &project,
      &ctx(),
      Scan {
        threads: vec![thread("ST0001")],
        carried: carried.clone(),
        ..Default::default()
      },
    )
    .expect("carried findings do not block");

    assert_eq!(planned.carried, carried);
  }

  #[test]
  fn two_artefacts_claiming_one_path_are_refused_rather_than_overwritten() {
    let dir = tempfile::tempdir().expect("tempdir");
    let project = project(dir.path());

    let Err(Blocked::Collision { path }) = plan(
      &project,
      &ctx(),
      Scan {
        threads: vec![thread("ST0001"), thread("ST0001")],
        ..Default::default()
      },
    ) else {
      panic!("a duplicated id must not silently overwrite");
    };
    assert!(
      path.contains("ST0001"),
      "the refusal names the file: {path}"
    );
  }

  #[test]
  fn a_duplicated_issue_number_is_caught_by_the_same_guard() {
    let dir = tempfile::tempdir().expect("tempdir");
    let project = project(dir.path());

    let Err(Blocked::Collision { path }) = plan(
      &project,
      &ctx(),
      Scan {
        issues: vec![issue(46), issue(46)],
        ..Default::default()
      },
    ) else {
      panic!("two issues on one number must not silently overwrite");
    };
    assert!(path.contains("46"), "the refusal names the file: {path}");
  }

  #[test]
  fn every_planned_write_lands_under_the_project() {
    let dir = tempfile::tempdir().expect("tempdir");
    let project = project(dir.path());

    let planned = plan(
      &project,
      &ctx(),
      Scan {
        threads: vec![thread("ST0001")],
        issues: vec![issue(1)],
        ..Default::default()
      },
    )
    .expect("plan");

    // The batch is private to `WriteSet`, so this proves the property the only
    // way available from outside: commit it into a throwaway tree and look at
    // what appeared. It is also the one test that exercises the whole path.
    let count = planned.writes.len();
    planned.writes.commit().expect("commit").keep();
    let after = tree(dir.path());

    // **Named, not counted.** A count assertion passes for any set of the
    // right size, and the property at stake is that the canon landed at the
    // path every later reader resolves -- `Project::thread_json` and
    // `Project::issue_json`, which is where the exporter's unpadded issue
    // spelling would have shown up as a file nothing can find.
    for expected in [
      project.thread_json("ST0001"),
      project.info_view("ST0001"),
      project.acceptance_view("ST0001"),
      project.issue_json(1),
      project.steel_threads_view(),
      project.todo_view(),
    ] {
      let rel = project.relative(&expected);
      assert!(after.contains(&rel), "missing {rel} in {after:?}");
    }
    assert_eq!(
      after.len(),
      count + 1,
      "everything planned landed, and nothing else did (+1 is the config that was already there)"
    );
    for path in &after {
      assert!(
        path.starts_with("intent/"),
        "a migration wrote outside intent/: {path}"
      );
    }
  }

  /// **A thread WITH work packages, because `st/<ID>/WP/<NN>/info.md` is the
  /// other zero-padded path in this estate and the issue canon has just shown
  /// what a padding disagreement costs.**
  ///
  /// Every other fixture here has no WPs, so the WP view path was reached only
  /// by the canary run -- which counts files and never checks where they went.
  /// `Project::wp_info_view` is the single builder for it, so there is no
  /// second spelling to disagree with today; this pins that there is not one
  /// tomorrow either, and it pins the padding rather than the count.
  #[test]
  fn a_work_package_view_lands_at_the_zero_padded_path() {
    let dir = tempfile::tempdir().expect("tempdir");
    let project = project(dir.path());

    let mut thread = thread("ST0001");
    thread.wps = vec![wp(1), wp(12)];

    let planned = plan(
      &project,
      &ctx(),
      Scan {
        threads: vec![thread],
        ..Default::default()
      },
    )
    .expect("plan");

    // canon: 1 thread + 1 event log. views: info + acceptance + 2 WP covers +
    // the index and the todo view. The globals are the two I left out of this
    // line first time round, and the count is what said so.
    assert_eq!(planned.writes.len(), 2 + 6);
    planned.writes.commit().expect("commit").keep();

    let after = tree(dir.path());
    for seq in [1u32, 12] {
      let rel = project.relative(&project.wp_info_view("ST0001", seq));
      assert!(after.contains(&rel), "missing {rel} in {after:?}");
    }
    // Named literally as well as derived: the derived assertion above passes
    // if BOTH sides drop the padding together, which is exactly how the issue
    // defect stayed invisible -- each side self-consistent, neither crossed.
    assert!(
      after.contains(&"intent/st/ST0001/WP/01/info.md".to_string()),
      "WP 1 must be `01`, not `1`: {after:?}"
    );
  }

  /// **The invariant [`Blocked::Conservation`] guards, asserted from the other
  /// side so that it is not a tripwire nobody can reach.**
  ///
  /// The guard itself cannot be provoked -- `canon_parts` returns exactly one
  /// part per artefact plus the log, for every input -- so a test that tried to
  /// construct the refusal would have to break the exporter to do it. This
  /// pins the equality instead: change the shape of `canon_parts` and this reds
  /// in the same commit that arms the guard. An untested error path and an
  /// untested invariant are the same liability read from two ends.
  #[test]
  fn the_canon_carries_one_file_per_artefact_plus_the_log() {
    let dir = tempfile::tempdir().expect("tempdir");
    let project = project(dir.path());

    for (threads, issues) in [(0, 0), (1, 0), (0, 1), (3, 2)] {
      let planned = plan(
        &project,
        &ctx(),
        Scan {
          threads: (1..=threads)
            .map(|n| thread(&format!("ST{n:04}")))
            .collect(),
          issues: (1..=issues).map(issue).collect(),
          ..Default::default()
        },
      )
      .expect("plan");

      // Views: info + acceptance per thread (no WPs in these fixtures), plus
      // the index and the todo view. Stated here rather than derived, so a
      // change in what the estate renders has to be acknowledged.
      let views = 2 * threads as usize + 2;
      let canon = threads as usize + issues as usize + 1;
      assert_eq!(
        planned.writes.len(),
        canon + views,
        "{threads} thread(s) + {issues} issue(s)"
      );
    }
  }

  /// Every file under `root`, project-relative and sorted.
  fn tree(root: &std::path::Path) -> Vec<String> {
    fn walk(dir: &std::path::Path, root: &std::path::Path, out: &mut Vec<String>) {
      let Ok(entries) = std::fs::read_dir(dir) else {
        return;
      };
      for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
          walk(&path, root, out);
        } else {
          out.push(crate::project::relative(root, &path));
        }
      }
    }
    let mut out = Vec::new();
    walk(root, root, &mut out);
    out.sort();
    out
  }
}
