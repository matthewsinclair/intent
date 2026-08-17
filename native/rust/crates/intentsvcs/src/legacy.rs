//! **The FROZEN v2 markdown parser** -- WP-10 Phase A (migration.md).
//!
//! This is the one place in v3 that reads v2's format, and it is deliberately
//! different code from [`crate::ingest`]. Ingest is STRICT because current-
//! version data has a schema to be strict against; this reads an estate that
//! never had one, so it is lenient about shape and ruthless about saying what
//! it could not read. Sharing one parser between them would force one of those
//! postures onto the other.
//!
//! **Phase A is READ-ONLY and it gates Phase B.** Nothing here writes: it
//! parses the whole estate, builds the model it can, and reports everything it
//! could not. That split is migration.md's, and it is what makes "refuse what
//! cannot convert without loss, name everything, guess nothing" checkable
//! before a single byte of anyone's project is rewritten.
//!
//! **Residue and carry are different buckets, and the difference is hv's
//! ruling rather than a severity judgement** (migration.md, 2026-08-14). The
//! same unreadable row BLOCKS in a live thread and CARRIES in a closed one:
//! live residue is fixed under v2 tooling and the migration re-run, while a
//! closed thread's legacy grammar is carried whole into the richer model --
//! marked legacy, nothing guessed, nothing dropped. The forcing fact is that
//! the fleet's sweep program is dead, so "fix it under v2 and re-run" is an
//! instruction one estate's owner has permanently refused. Keeping the two in
//! separate fields makes the ruling structural instead of a convention someone
//! has to remember.

use std::collections::BTreeMap;
use std::path::Path;

use crate::finding::{Finding, FindingClass};
use crate::model::{
  AcKind, AcState, AcceptanceTest, AtKind, AtStatus, Criterion, Issue, THREAD_SCHEMA, TShirt,
  Thread, ThreadStatus, WorkPackage, WpStatus,
};
use crate::project::Project;

/// What Phase A found.
/// The template a dropped section is claimed identical to, cited by PATH and
/// REVISION and by the line of shell that substituted it (vc's condition 2,
/// tightened once the substitution became part of the artefact).
///
/// **The path alone is a moving target**: the same estate migrated at two
/// revisions would drop different sections with nothing saying why. A pinned
/// citation makes the drop set re-derivable by someone who was not there.
const WP_TEMPLATE_PATH: &str = "lib/templates/prj/st/WP/info.md";
const WP_TEMPLATE_REV: &str = "0b1b3b5b";
const WP_TEMPLATE_SUBST: &str = "bin/intent_wp:113";

/// What the migration decided about one section, and why.
///
/// **File, heading, verdict, reason -- never a count.** The reason names the
/// artefact and its revision, so the claim is checkable rather than asserted.
///
/// **Two verdicts and not one, because a zero in a conservation report has to
/// be readable** (vc). Their `DOUBLED-SECTION 20 -> 0` on Baize is produced
/// equally by a migrator that deferred to the author and by one that stopped
/// generating the section at all -- they could separate them only by going and
/// reading which pointer survived, and nobody running the tool would.
#[derive(Debug, Clone)]
pub struct Disposition {
  /// The file as Intent names it, the same spelling every finding uses.
  pub owner: String,
  /// The `## ` heading, verbatim.
  pub heading: String,
  pub verdict: Verdict,
  /// What justifies the verdict, with the artefact cited.
  pub reason: String,
}

/// **A DECISION THAT LEAVES NO RECORD CANNOT BE RECONCILED**, and both of these
/// are decisions -- one removes content from canon, the other chooses between
/// two renderings of it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
  /// The section is not carried: byte-identical to the template that made the
  /// file, so no author wrote it.
  Dropped,
  /// The section IS carried and the renderer's own copy stands down for it.
  /// Canon is unchanged either way; what changes is the view.
  Deferred,
  /// The bytes are kept and MOVED to the field the grammar puts them in.
  ///
  /// **The only verdict whose record is the sole evidence.** A drop can be
  /// corroborated by observing canon is empty and a deferral by observing it is
  /// not; a re-filing between two fields of one row is invisible to a census
  /// that hashes the whole row, so nothing can confirm or refute it from the
  /// outside.
  Refiled,
}

impl Verdict {
  pub fn as_str(self) -> &'static str {
    match self {
      Self::Dropped => "dropped",
      Self::Deferred => "deferred",
      Self::Refiled => "refiled",
    }
  }
}

#[derive(Debug, Default)]
pub struct Scan {
  /// The model, as far as it could be built.
  pub threads: Vec<Thread>,
  /// The issue estate, `intent/issues/{OPEN,CLOSED}/<nnnn>/<nnnn>-<slug>.md`.
  ///
  /// **A separate field rather than a member of the thread walk, because v2's
  /// issue tracker is a separate estate on disk and shares no ancestor
  /// directory with the threads.** It went unread until WP-10 measured it, and
  /// the failure mode is the one `retired_settings` names: nothing recognises
  /// an issue, nothing is emitted, and every count reconciles perfectly
  /// against zero.
  pub issues: Vec<Issue>,
  /// Findings in LIVE threads. **These block the migration.**
  pub residue: Vec<Finding>,
  /// The same classes of finding in CLOSED threads. **These convert under the
  /// carry policy** and are reported so the count reconciles, never so that
  /// anyone acts on them.
  pub carried: Vec<Finding>,
  /// What the migration decided about each section it did not simply carry.
  ///
  /// **Per section, never a count** (vc's condition 1): a count reconciles
  /// arithmetically and tells nobody which section went, and **a drop with no
  /// record is indistinguishable from a section that was never there.** This
  /// is the third summary bucket beside modelled and carried, and it is what
  /// lets a declared drop leave `LOST-PROSE` through the conservation check's
  /// existing out-of-model arm while an undeclared one stays a finding.
  ///
  /// Not a `Finding`: a finding describes something a v2 AUTHOR left behind,
  /// with a fix environment and a carry disposition. This describes something
  /// the migration itself decided, and routing it through `record` would put a
  /// remedy on a section nobody needs to act on.
  pub dispositions: Vec<Disposition>,
  /// Thread ids Phase A DECLINED TO RE-PARSE because committed canon already
  /// exists for them, sorted and deduplicated.
  ///
  /// **This is what makes the migration re-runnable, which hv's fix-forward
  /// ruling requires: the recovery operation for a partial migration IS running
  /// it again.** Without it, re-running was not merely non-idempotent, it
  /// ACCRETED WITHOUT BOUND -- ic measured a real work package growing 8562 ->
  /// 8840 -> 9190 -> 9540 bytes over three runs, monotonically, never
  /// converging, with every count reconciling and nothing blocking on any run.
  ///
  /// The mechanism is the D28 catch-all meeting its own output. A rendered view
  /// ends with generated sections and the banner, and the catch-all cannot tell
  /// the renderer's sections from authored ones, so it absorbs them; the next
  /// render appends fresh ones. **The banner reading "do not edit this file, it
  /// is rendered from the model" ends up IN the model, three copies deep, in
  /// committed canon.**
  ///
  /// **So the fix is not a better discriminator, it is not re-reading a view as
  /// a source** -- a heading-level rule cannot work anyway, because `##
  /// Acceptance` is both a generated section and a legitimate authored v2 one.
  /// Canon existing means the thread is migrated and its markdown is now an
  /// extract, and this applies `ViewSkew`'s existing ruling -- canon wins, the
  /// divergence is named -- one step earlier.
  ///
  /// **NAMED rather than skipped, and that is this file's own rule**: a
  /// directory it ignores in silence is how an artefact disappears from a
  /// migration whose whole promise is that nothing does. Counted here, so
  /// `already_migrated + threads` reconciles against the estate and a re-run
  /// reports what it declined instead of quietly doing less.
  pub already_migrated: Vec<String>,
}

impl Scan {
  /// A live-thread finding: blocks.
  fn block(&mut self, finding: Finding) {
    self.residue.push(finding);
  }

  /// A closed-thread finding: carries.
  fn carry(&mut self, finding: Finding) {
    self.carried.push(finding);
  }

  /// Route by whether the thread is closed -- the ONE place the ruling is
  /// applied, so no call site has to remember which bucket it is in.
  fn record(&mut self, closed: bool, finding: Finding) {
    if closed {
      self.carry(finding)
    } else {
      self.block(finding)
    }
  }
}

/// Parse an entire v2 estate. Writes nothing.
pub fn scan(project: &Project) -> Result<Scan, std::io::Error> {
  let mut out = Scan::default();
  retired_settings(project, &mut out);
  // Ids already loaded from canon, so one id reached twice is loaded once.
  //
  // **The migration does NOT empty the v2 status buckets**, so after it runs a
  // migrated thread has a directory at BOTH `st/<ID>/` (written by the
  // migration) and `st/COMPLETED/<ID>/` (left where v2 put it), and
  // `thread_dirs` correctly yields both. Both resolve the same canon, so without
  // this the same `Thread` was pushed twice -- and the 0011 duplicate-id check
  // below would then BLOCK, on the ordinary shape of a re-run rather than on a
  // defect. **The first fix for the accretion would have made a re-run refuse.**
  //
  // It does not weaken 0011. That class is two v2 artefacts claiming one id, and
  // neither of those has canon, so both still reach the markdown path and still
  // collide. Once canon exists, one id in two places is the EXPECTED
  // post-migration shape and not something to report.
  let mut loaded: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
  for (id, dir) in thread_dirs(project) {
    // **CANON WINS: a thread with committed canon is migrated, so its SOURCE is
    // `thread.json` and the markdown beside it is a generated view.** Asked of
    // the FLAT canon path and not of `dir`, because migration relocates -- a
    // thread still sitting in `COMPLETED/` has its canon at
    // `st/<ID>/thread.json`, so asking the directory being walked would answer
    // "no" for every thread the migration had already moved, which is the whole
    // population this exists for. See `Scan::already_migrated`.
    //
    // **IT IS LOADED, NOT DROPPED, AND THAT DISTINCTION IS THE WHOLE FIX** (ic's
    // correction to my first version, which used `continue`). `views::render_all`
    // renders `steel_threads.md` and `todo.md` FROM THE WHOLE THREAD LIST, so a
    // thread missing from `threads` is a thread missing from the index: a re-run
    // of a mostly-migrated estate would have rewritten both globals with two
    // rows and reported `2 converted, 54 already` -- true, and reading as
    // success. **Trading unbounded accretion for a silently truncated index is
    // not a fix.** The category error was parsing a view as a source; the
    // correction is to read the source, not to lose the thread.
    let canon = project.thread_json(&id);
    if canon.is_file() {
      // **CANON THAT EXISTS AND WILL NOT READ IS AN ERROR, NOT RESIDUE, AND
      // THAT IS A CLASSIFICATION RATHER THAN A CONVENIENCE.** Every class in the
      // residue report describes something a v2 AUTHOR left behind, with a fix
      // environment and a carry disposition. This describes canon THIS MIGRATION
      // WROTE. There is no v2 author to attribute it to, no carry policy it
      // could fall under, and no work list an operator could act on -- so
      // reporting it beside `unknown-scope` would put a broken migrator in a
      // table about broken estates.
      //
      // It also keeps Phase A's report Phase A's own. The residue classes are
      // enumerated by grepping THIS FILE for `FindingClass::`, so a class
      // constructed inside `ingest` and merely passed through here would be
      // emitted by the migration and invisible to the check that verifies the
      // contract declares what the migration emits. Turning the refusal into an
      // error rather than laundering ingest's findings into my report is what
      // keeps that enumeration honest -- and it is worth knowing that the
      // enumeration cannot see a delegated construction, whoever delegates next.
      let text = std::fs::read_to_string(&canon)?;
      let thread = crate::ingest::read_thread(project, &id, &text).map_err(|found| {
        std::io::Error::other(format!(
          "committed canon at {} does not read as canon, so this project is mid-migration with a broken artefact rather than an estate to convert: {}",
          project.relative(&canon),
          found
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join("; ")
        ))
      })?;
      if loaded.insert(id.clone()) {
        out.threads.push(thread);
      }
      out.already_migrated.push(id);
      continue;
    }
    let info = dir.join("info.md");
    let Ok(text) = std::fs::read_to_string(&info) else {
      // A directory that looks like a thread and has no `info.md` is not a
      // thread this parser can classify. It is named rather than skipped: a
      // silently ignored directory is how an artefact disappears from a
      // migration whose whole promise is that nothing does.
      out.block(Finding::new(
        project.relative(&info),
        FindingClass::UnknownFileShape,
        "a steel-thread directory with no info.md".to_string(),
      ));
      continue;
    };
    let rel = project.relative(&info);
    let (front, body) = frontmatter(&text);

    // The status decides which bucket every later finding for this thread
    // lands in, so it is read first and a thread whose status cannot be read
    // is treated as LIVE -- the conservative direction, because guessing
    // "closed" would silently carry rows that should have blocked.
    let raw_status = front.get("status").cloned().unwrap_or_default();
    let status = thread_status(&raw_status);
    let closed = matches!(
      status,
      Some(ThreadStatus::Completed | ThreadStatus::Cancelled)
    );
    if status.is_none() {
      out.block(Finding::new(
        &rel,
        FindingClass::UnknownStatus,
        format!("thread status {raw_status:?} is not in the v2 vocabulary"),
      ));
    }

    if let Some(line) = conflict_marker_line(&text) {
      out.record(
        closed,
        Finding::new(&rel, FindingClass::ConflictMarkers, "in info.md").at_line(line),
      );
    }

    let sections = sections(body);
    let (criteria, tests) = acceptance(project, &dir, closed, &mut out);
    let wps = work_packages(project, &dir, closed, &mut out);
    out.threads.push(Thread {
      schema: THREAD_SCHEMA.to_string(),
      id: id.clone(),
      title: title(body).unwrap_or_else(|| id.clone()),
      slug: front.get("slug").filter(|s| !s.is_empty()).cloned(),
      status: status.unwrap_or(ThreadStatus::NotStarted),
      status_reason: None,
      created: date(front.get("created")),
      completed: front
        .get("completed")
        .filter(|s| !s.is_empty())
        .map(|s| date(Some(s))),
      acceptance: None,
      // **Verbatim, never reflowed** (migration.md: the migrator does not
      // improve prose). An absent section stays empty rather than acquiring a
      // placeholder.
      objective: section(&sections, "Objective"),
      context: section(&sections, "Context"),
      related: Vec::new(),
      wps,
      criteria,
      tests,
    });
  }

  // The 0011 class: two artefacts claiming one natural id. Checked across the
  // whole estate rather than within a directory, because the v2 layout puts
  // closed threads in subdirectories -- which is exactly how one id comes to
  // exist in two places.
  let mut seen: BTreeMap<&str, usize> = BTreeMap::new();
  for t in &out.threads {
    *seen.entry(t.id.as_str()).or_default() += 1;
  }
  let duplicates: Vec<String> = seen
    .iter()
    .filter(|(_, n)| **n > 1)
    .map(|(id, n)| format!("{id} appears {n} times"))
    .collect();
  for detail in duplicates {
    out.block(Finding::new("intent/st", FindingClass::DuplicateId, detail));
  }

  // **Deduplicated, and it is NOT tidying.** `thread_dirs` legitimately yields
  // one id twice -- v2's own `st done` leaves a directory behind under
  // `COMPLETED/` while the flat path exists -- and the canon check above fires
  // for both. Reporting `56 already migrated` on an estate of 54 would make the
  // reconciliation that this field exists to serve arithmetically wrong, which
  // is worse than not having it. Sorted so a re-run's report is stable.
  out.already_migrated.sort();
  out.already_migrated.dedup();

  Ok(out)
}

/// Every `ST####` directory, at the top level and under v2's status
/// subdirectories.
///
/// The subdirectory walk is not optional: `intent st done` MOVES a thread into
/// `COMPLETED/`, so a migrator reading only the top level would silently
/// convert the live threads and lose every finished one -- which on this estate
/// is 54 of 56.
fn thread_dirs(project: &Project) -> Vec<(String, std::path::PathBuf)> {
  let root = project.st_dir();
  let mut out = Vec::new();
  let push_from = |dir: &Path, out: &mut Vec<(String, std::path::PathBuf)>| {
    let Ok(entries) = std::fs::read_dir(dir) else {
      return;
    };
    let mut found: Vec<(String, std::path::PathBuf)> = entries
      .flatten()
      .filter(|e| e.path().is_dir())
      .filter_map(|e| {
        let name = e.file_name().to_string_lossy().into_owned();
        is_thread_id(&name).then_some((name, e.path()))
      })
      .collect();
    found.sort();
    out.append(&mut found);
  };
  push_from(&root, &mut out);
  for bucket in ["COMPLETED", "NOT-STARTED", "CANCELLED"] {
    push_from(&root.join(bucket), &mut out);
  }
  out
}

use crate::model::is_thread_id;

/// **Name a retired config knob the project actually set** (issue 0040, hv).
///
/// `st_prefix` is gone from `Config` and lands in `extra`, so the declaration
/// survives in the file. What must not survive is v3 reading a project whose
/// threads are on some other prefix and reporting a clean, empty conversion:
/// `is_thread_id` recognises nothing, `thread_dirs` yields nothing, and every
/// count reconciles perfectly against zero.
///
/// **Blocks rather than carries, and the closed/live split does not apply**,
/// because the finding is about the project rather than about a thread -- and
/// there is no thread to attribute it to precisely when it matters most.
///
/// Costs nothing today: all sixteen fleet projects use the default, so this is
/// silent on every one of them. It is here for the reader who is not in the
/// fleet, who is the only person the retirement could have hurt.
fn retired_settings(project: &Project, out: &mut Scan) {
  let Some(declared) = project
    .config()
    .extra
    .get(crate::project::RETIRED_ST_PREFIX_KEY)
    .and_then(|v| v.as_str())
  else {
    return;
  };
  if declared == crate::model::THREAD_PREFIX {
    // Declared, and declaring the value v3 fixed it to. Nothing is lost and
    // nothing changes, so saying anything here would be noise on a project
    // that did nothing wrong.
    return;
  }
  let key = crate::project::RETIRED_ST_PREFIX_KEY;
  let fixed = crate::model::THREAD_PREFIX;
  out.block(Finding::new(
    project.relative(&Project::config_path(project.root())),
    FindingClass::RetiredSetting,
    format!(
      "`{key}: {declared}` is retired -- v3 fixes the steel-thread prefix at `{fixed}`, so no artefact named `{declared}...` is recognised as a thread"
    ),
  ));
}

/// v2's status synonym table, ported from `bin/intent_helpers:canonical_status`.
///
/// **Ported rather than reimplemented from the canonical spellings, and the
/// difference is a false finding I nearly filed.** A census of this estate
/// showed one work package at `status: Complete`, which is not one of v2's
/// canonical outputs -- so it read as an unknown status. It is not: `complete`
/// is in v2's synonym table and has always resolved to `Completed`. **The
/// vocabulary is what the tool ACCEPTS, not the set of values it prints**, and
/// a migrator that confused the two would report residue against data v2
/// considered perfectly well-formed.
fn thread_status(raw: &str) -> Option<ThreadStatus> {
  match raw.trim().to_ascii_lowercase().as_str() {
    "wip" | "in progress" | "inprogress" | "in-progress" => Some(ThreadStatus::Wip),
    // **TBC maps to NotStarted, NEVER to Triage** (migration.md, ratified).
    // In v2, TBC abbreviates "To Be Commenced" -- `bin/intent_st:46` spells it
    // out in the tool's own usage text. `Triage` reuses the three letters and
    // not the meaning, so mapping to it would invent a triage decision nobody
    // made, for every thread that ever carried the token.
    "not started" | "notstarted" | "not-started" | "tbc" | "to be commenced" => {
      Some(ThreadStatus::NotStarted)
    }
    "completed" | "complete" | "done" => Some(ThreadStatus::Completed),
    "cancelled" | "canceled" => Some(ThreadStatus::Cancelled),
    "on hold" | "onhold" | "on-hold" | "hold" => Some(ThreadStatus::Hold),
    _ => None,
  }
}

fn wp_status(raw: &str) -> Option<WpStatus> {
  match raw.trim().to_ascii_lowercase().as_str() {
    "wip" | "in progress" | "inprogress" | "in-progress" => Some(WpStatus::Wip),
    "not started" | "notstarted" | "not-started" | "tbc" | "to be commenced" => {
      Some(WpStatus::NotStarted)
    }
    "done" | "complete" | "completed" => Some(WpStatus::Done),
    _ => None,
  }
}

/// The eleven scope spellings this estate actually carries.
///
/// **Measured, not guessed, and the count is load-bearing**: `Small` 56,
/// `Medium` 34, `Large` 8, `L` 8, `XL` 5, `M` 5, `S` 4, `ExtraSmall` 4,
/// `Extra Small` 3, `XS` 1, `Medium-Large` 1. v2 reads `scope:` as free text,
/// so this is not a vocabulary anyone chose -- it is the absence of one, which
/// is what modelling the field fixes.
///
/// **`Medium-Large` maps to nothing and is not guessed at.** It sits between
/// two enum values, in a closed thread, so under the carry policy it is
/// reported as carried rather than blocking -- and the model has no
/// marked-legacy form for a scope yet, which is a WP-10 dependency on
/// data-model.md rather than something to invent here.
///
/// **The canonical six are NOT spelled again here.** [`TShirt::parse`] owns
/// them, derived from the enum's own serialisation, and this adds only what is
/// genuinely v2's: the long forms v2's free-text field actually contains. Two
/// tables of the same six spellings is the Highlander shape that lets a rename
/// update one and leave the other -- and the copy that gets left is the ingest
/// one, because nothing an operator types exercises it.
fn scope(raw: &str) -> Option<TShirt> {
  let normalised = raw.trim().to_ascii_lowercase().replace([' ', '-', '_'], "");
  let v2_long_form = match normalised.as_str() {
    "extrasmall" => Some(TShirt::XS),
    "small" => Some(TShirt::S),
    "medium" => Some(TShirt::M),
    "large" => Some(TShirt::L),
    "extralarge" => Some(TShirt::XL),
    _ => None,
  };
  TShirt::parse(&normalised).or(v2_long_form)
}

fn work_packages(project: &Project, dir: &Path, closed: bool, out: &mut Scan) -> Vec<WorkPackage> {
  let wp_root = dir.join("WP");
  let Ok(entries) = std::fs::read_dir(&wp_root) else {
    return Vec::new();
  };
  let mut dirs: Vec<std::path::PathBuf> = entries
    .flatten()
    .map(|e| e.path())
    .filter(|p| p.is_dir())
    .collect();
  dirs.sort();

  let mut wps = Vec::new();
  for wp_dir in dirs {
    let info = wp_dir.join("info.md");
    let Ok(text) = std::fs::read_to_string(&info) else {
      continue;
    };
    let rel = project.relative(&info);
    let (front, body) = frontmatter(&text);

    let Some(seq) = front
      .get("wp_id")
      .and_then(|v| v.trim().strip_prefix("WP-"))
      .and_then(|v| v.trim().parse::<u32>().ok())
      .or_else(|| {
        wp_dir
          .file_name()
          .and_then(|n| n.to_str())
          .and_then(|n| n.parse::<u32>().ok())
      })
    else {
      out.record(
        closed,
        Finding::new(
          &rel,
          FindingClass::UnparseableRow,
          "work package has no readable wp_id".to_string(),
        ),
      );
      continue;
    };

    // **ABSENT IS NOT INVALID, and conflating them invents work.** The first
    // run of this parser reported 20 findings against this estate; 19 of them
    // were fields that were never authored, reported as values "not in the v2
    // vocabulary". Three closed threads predate the work-package frontmatter
    // convention entirely -- ST0023's work packages have no frontmatter at all
    // -- so the honest reading is "v2 never recorded this", not "v2 recorded
    // something wrong". A migrator that says the second sends someone to
    // repair files their tooling was perfectly happy with, which is precisely
    // the confident-from-partial-evidence habit v3 exists to end.
    //
    // The real count on this estate is ONE: `scope: Medium-Large`.
    let raw_status = front.get("status").cloned().unwrap_or_default();
    let status = match (raw_status.trim().is_empty(), wp_status(&raw_status)) {
      (_, Some(status)) => status,
      (true, None) => {
        out.record(
          closed,
          Finding::new(
            &rel,
            FindingClass::FieldNotRecorded,
            "this work package predates the frontmatter convention: no status was ever recorded",
          ),
        );
        WpStatus::NotStarted
      }
      (false, None) => {
        out.record(
          closed,
          Finding::new(
            &rel,
            FindingClass::UnknownStatus,
            format!("work-package status {raw_status:?} is not in the v2 vocabulary"),
          ),
        );
        WpStatus::NotStarted
      }
    };

    // **THE `TShirt::M` SUBSTITUTION IS GONE, AND IT WAS THE THING THIS
    // MIGRATION EXISTS NOT TO DO.** Both arms below used to fall back to `M`:
    // a work package whose file never carried a `scope:` line, and one whose
    // scope v2 read as free text and the enum cannot express. Neither is a
    // medium. The first is a field nobody recorded and the second is a value
    // somebody did record, and answering both with the same confident size is
    // the answer-from-partial-evidence habit v3 exists to end -- silently, in
    // a migration, on data whose original was about to be replaced.
    //
    // Three states now, all of them true statements:
    //   Some(x) / None       -- recorded, inside the enum
    //   None    / Some(raw)  -- recorded, outside it, carried verbatim
    //   None    / None       -- never recorded
    let raw_scope = front.get("scope").cloned().unwrap_or_default();
    let (scope, scope_legacy) = match (raw_scope.trim().is_empty(), scope(&raw_scope)) {
      (_, Some(scope)) => (Some(scope), None),
      // Absent. Reported once per work package rather than once per absent
      // field: the fact is "this file predates the convention", and saying it
      // twice makes one old file look like two problems.
      (true, None) => (None, None),
      // Recorded and outside the enum. **Carried, and the CLOSED/LIVE split is
      // `record`'s job, not this arm's** -- hv's policy is that a closed thread
      // converts lossless-by-carrying while a live one stays BLOCKED-until-
      // clean, and `record` is the one place that ruling is applied.
      (false, None) => (
        None,
        Some(crate::model::Legacy {
          raw: raw_scope.trim().to_string(),
        }),
      ),
    };
    if let Some(carried) = &scope_legacy {
      out.record(
        closed,
        Finding::new(
          &rel,
          FindingClass::UnknownScope,
          format!(
            "work-package scope {:?} is outside the T-shirt enum, so it is carried verbatim as legacy rather than guessed at",
            carried.raw
          ),
        ),
      );
    }

    if let Some(line) = conflict_marker_line(&text) {
      out.record(
        closed,
        Finding::new(&rel, FindingClass::ConflictMarkers, "in a work package").at_line(line),
      );
    }

    let sections = sections(body);

    // **The template's own words are not this author's.** A section byte-
    // identical to the artefact that created the file is evidence that NOBODY
    // WROTE IT, so carrying it is not the conservative option -- it files
    // scaffolding as authored prose and the renderer emits it forever as
    // though someone had written it, which FABRICATES AUTHORSHIP (vc's
    // ruling, and the half I had backwards).
    //
    // **It was already shipping a visible defect: 40 of 140 migrated work-
    // package views carried TWO `## Acceptance` sections** -- the carried one
    // and the one `views::wp_info` generates -- saying the same thing in
    // different words. 104 across the captured fleet, and every one of them is
    // `Acceptance`; no other heading doubles anywhere (vc, by a different
    // method on a different copy).
    //
    // **This is NOT a heading-name rule and must never become one.** `##
    // Acceptance` is a legitimate authored section elsewhere in this estate,
    // and the drop is keyed on the BYTES matching the substituted template,
    // never on the name. Every error goes toward carrying: a file seeded from
    // an older template generation fails the match and is carried, which is
    // the safe outcome.
    let template = wp_template_sections(seq);
    let mut kept: Vec<(String, String)> = Vec::new();
    for (heading, text) in &sections {
      if heading == "Objective" {
        continue;
      }
      match template.iter().any(|(k, v)| k == heading && v == text) {
        true => out.dispositions.push(Disposition {
          owner: rel.clone(),
          heading: heading.clone(),
          verdict: Verdict::Dropped,
          reason: format!(
            "byte-identical to `{WP_TEMPLATE_PATH}` at {WP_TEMPLATE_REV} with `WP-NN` substituted per `{WP_TEMPLATE_SUBST}`: no author wrote it"
          ),
        }),
        false => kept.push((heading.clone(), text.clone())),
      }
    }

    // **AND THE DEFERRAL IS A DECISION TOO, so it leaves a record** (vc's
    // finding, and it is this file's own rule turned on the code that had just
    // landed: 39 drop records and zero deferral records for 20 deferrals).
    //
    // The predicate is `views::carries_heading` rather than a second copy of
    // it -- the renderer decides this and the record must be about what the
    // renderer will actually do, so asking a different question here would be
    // a record of something that never happens.
    let carried_body = kept
      .iter()
      .map(|(k, v)| format!("## {k}\n\n{v}"))
      .collect::<Vec<_>>()
      .join("\n\n");
    if crate::views::carries_heading(&carried_body, "Acceptance") {
      out.dispositions.push(Disposition {
        owner: rel.clone(),
        heading: "Acceptance".to_string(),
        verdict: Verdict::Deferred,
        reason:
          "this work package authors its own acceptance pointer, so the generated one stands down: canon is unchanged and the view carries the author's"
            .to_string(),
      });
    }

    wps.push(WorkPackage {
      seq,
      title: front
        .get("title")
        .map(|t| t.trim_matches('"').to_string())
        .or_else(|| title(body))
        .unwrap_or_default(),
      scope,
      scope_legacy,
      status,
      status_reason: None,
      objective: section(&sections, "Objective"),
      // D28: everything that is not the objective is the body, carried
      // verbatim, so a section the template never named survives -- **and in
      // the order it was written**, which is what `sections` returning a Vec
      // rather than a map buys. Minus the template's own scaffolding, above.
      body: carried_body,
    });
  }
  wps
}

/// Parse `acceptance.md` -- absent on most threads, which is not a finding.
///
/// **42 of this estate's 56 threads have no `acceptance.md` at all**, because
/// the contract arrived in v2.11.13. An empty contract is the ordinary case;
/// treating a missing file as residue would report 42 findings about a feature
/// those threads predate.
fn acceptance(
  project: &Project,
  dir: &Path,
  closed: bool,
  out: &mut Scan,
) -> (Vec<Criterion>, Vec<AcceptanceTest>) {
  let path = dir.join("acceptance.md");
  let Ok(text) = std::fs::read_to_string(&path) else {
    return (Vec::new(), Vec::new());
  };
  let rel = project.relative(&path);
  let mut criteria = Vec::new();
  let mut tests = Vec::new();

  for (i, line) in text.lines().enumerate() {
    let line_no = (i + 1) as u32;
    let Some(row) = line.strip_prefix("- ") else {
      continue;
    };
    if row.starts_with("AC-") {
      match criterion(row) {
        Some(c) => criteria.push(c),
        None => out.record(
          closed,
          Finding::new(&rel, FindingClass::UnparseableRow, "AC row").at_line(line_no),
        ),
      }
    } else if row.starts_with("AT-") {
      match acceptance_test(row) {
        Some((t, qualifiers)) => {
          for (ac, qualifier) in &qualifiers {
            out.dispositions.push(Disposition {
              owner: format!("{rel}:{line_no}"),
              heading: t.id.clone(),
              verdict: Verdict::Refiled,
              reason: format!(
                "`{ac} ({qualifier})` put prose inside an id in the covers clause, so the id could not resolve; the id is now {ac} and the qualifier is in the row's note, keyed to it"
              ),
            });
          }
          tests.push(t);
        }
        None => out.record(
          closed,
          Finding::new(&rel, FindingClass::UnparseableRow, "AT row").at_line(line_no),
        ),
      }
    }
  }

  // The broken-reference class: an AT covering a criterion that is not in this
  // thread's contract. Reported rather than dropped -- the coverage link is
  // the thing the contract is FOR.
  let ids: Vec<&str> = criteria.iter().map(|c| c.id.as_str()).collect();
  for test in &tests {
    for covered in &test.covers {
      if !ids.contains(&covered.as_str()) {
        out.record(
          closed,
          Finding::new(
            &rel,
            FindingClass::BrokenReference,
            format!(
              "{} covers {covered}, which is not in this contract",
              test.id
            ),
          ),
        );
      }
    }
  }

  (criteria, tests)
}

/// `- AC-<gg>.<n> [(non-test)] <text> [-- evidence: <e>] [-- satisfied: yes|no]`
fn criterion(row: &str) -> Option<Criterion> {
  let (id, rest) = row.split_once(' ')?;
  if !id.starts_with("AC-") {
    return None;
  }
  let non_test = rest.trim_start().starts_with("(non-test)");
  let body = rest.trim_start().trim_start_matches("(non-test)").trim();

  let evidence = field(body, "evidence");
  let satisfied = field(body, "satisfied");
  let text = body
    .split(" -- evidence:")
    .next()
    .unwrap_or(body)
    .split(" -- satisfied:")
    .next()
    .unwrap_or(body)
    .trim()
    .to_string();

  let (kind, state) = if non_test {
    let state = match (satisfied.as_deref(), evidence) {
      // **Evidence is required for a satisfied non-test criterion and is NOT
      // invented when it is missing.** A synthesised sentence reads as
      // evidence forever after and nothing downstream can tell it from the
      // real thing (vc's migration ruling). A `satisfied: yes` with no
      // evidence therefore arrives UNSATISFIED, and the v2 claim is residue
      // rather than a value.
      (Some("yes"), Some(e)) if !e.trim().is_empty() => AcState::Satisfied { evidence: e },
      _ => AcState::Unsatisfied,
    };
    (AcKind::NonTest, state)
  } else {
    // A test-backed criterion's satisfaction is COMPUTED from its covering
    // tests, so nothing is carried onto the row.
    (AcKind::Test, AcState::Computed)
  };

  Some(Criterion {
    id: id.to_string(),
    text,
    kind,
    state,
  })
}

/// `- AT-<gg>.<n> <subject> -- covers <ids> -- status: <s> [-- test: <name>] [-- note]`
///
/// **The keyed fields are FOUND rather than split out**, and that is the whole
/// difficulty of this grammar. A note routinely contains ` -- ` itself, so
/// splitting the row on the separator over-splits exactly the rows carrying the
/// most information. Searching for the three known keys leaves everything else
/// as the note, whatever it contains.
fn acceptance_test(row: &str) -> Option<(AcceptanceTest, Vec<(String, String)>)> {
  let (id, rest) = row.split_once(' ')?;
  if !id.starts_with("AT-") {
    return None;
  }
  let Covers {
    ids: covers,
    qualifiers,
  } = covers(rest)?;
  if covers.is_empty() {
    return None;
  }
  let status = match field(rest, "status")?.trim() {
    "green" => AtStatus::Green,
    "red" => AtStatus::Red,
    "to-write" | "towrite" => AtStatus::ToWrite,
    "n/a" | "n-a" | "na" => AtStatus::Na,
    _ => return None,
  };

  let subject = rest.split(" -- ").next().unwrap_or("").trim();
  let non_test = subject.starts_with("(non-test)");
  let file = subject.trim_matches('`').to_string();
  // The 0017 reference rules: a test file has at least one `/` and no `:`.
  // A reference failing them is a legacy citation, and it is CARRIED whole
  // rather than reshaped into something that satisfies the grammar.
  let is_path = !non_test && file.contains('/') && !file.contains(':');

  let test = AcceptanceTest {
    id: id.to_string(),
    kind: if non_test {
      AtKind::NonTest
    } else {
      AtKind::Test
    },
    file: is_path.then(|| file.clone()),
    prose: non_test.then(|| subject.trim_start_matches("(non-test)").trim().to_string()),
    covers,
    status,
    // **KEYED TO THE AC IT QUALIFIES, never appended bare** (vc's ruling, and
    // the requirement is forced by a real row rather than added defensively).
    // `ST0009/AT-09.3` covers TWO criteria and only one carries a qualifier --
    // `AC-09.3, AC-09.1 (render)` -- so a bare `render` in the note loses the
    // association and lands on the wrong one half the time. The note is where
    // this belongs: the row grammar already has a slot for unkeyed prose, and
    // the qualifier is prose that was written into the id slot.
    note: with_qualifiers(note(rest), &qualifiers),
    legacy: (!non_test && !is_path && !file.is_empty())
      .then(|| crate::model::Legacy { raw: file.clone() }),
  };
  // Returned rather than recorded here: this function has no `Scan`, and
  // re-deriving them at the call site would be a second copy of the predicate
  // -- the shape that makes a record describe something the code never did.
  Some((test, qualifiers))
}

/// Fold the covers-clause qualifiers into the row's note, each keyed to the
/// criterion it describes.
///
/// **Re-filing rather than dropping**, because the qualifier is the author
/// saying WHY a coverage link holds and nobody writes that twice. It is a
/// structural move -- the grammar's prose slot is the note -- and it is
/// declared through the disposition mechanism, which on this one is not
/// corroborating evidence but the ONLY evidence: vc's census declares `test`
/// rows UNCOMPARED (it hashes the whole authored row; canon holds
/// `.tests[].note`; no common bytes), so a note that gains text produces no
/// ALTERED and no ADDED. **A conservation check cannot see this in either
/// direction.**
fn with_qualifiers(note: Option<String>, qualifiers: &[(String, String)]) -> Option<String> {
  if qualifiers.is_empty() {
    return note;
  }
  let folded = qualifiers
    .iter()
    .map(|(id, q)| format!("{id}: {q}"))
    .collect::<Vec<_>>()
    .join(" -- ");
  Some(match note {
    Some(existing) if !existing.trim().is_empty() => format!("{existing} -- {folded}"),
    _ => folded,
  })
}

/// The covered AC ids: ` -- covers AC-01.2, AC-01.3 -- ...`.
///
/// **Its own accessor because `covers` carries NO COLON**, where every other
/// keyed field does -- `status:`, `test:`, `evidence:`, `satisfied:`. One
/// accessor assuming a uniform `key: value` shape rejected 226 of 226 AT rows
/// in this estate and reported each one as an unparseable row, which is a
/// migrator confidently blaming an estate for its own grammar. Measured by
/// running it, not by reading the spec: the spec says "covers" and the reader
/// supplies the colon from habit.
/// A parsed covers clause: the ids, and any prose that was written inside one.
///
/// A named type rather than a tuple because the two halves are read in
/// different places -- `ids` goes to the model and `qualifiers` to the note and
/// the disposition record -- and positional access at three call sites is how
/// they get swapped.
struct Covers {
  ids: Vec<String>,
  /// `(criterion id, the prose that qualified it)`, keyed so a row covering
  /// several criteria can say which one each qualifier belongs to.
  qualifiers: Vec<(String, String)>,
}

fn covers(row: &str) -> Option<Covers> {
  const MARKER: &str = " -- covers ";
  let start = row.find(MARKER)? + MARKER.len();
  let rest = &row[start..];
  let end = rest.find(" -- ").unwrap_or(rest.len());
  let mut ids = Vec::new();
  let mut qualifiers = Vec::new();
  for span in rest[..end].split(',') {
    let span = span.trim();
    if span.is_empty() {
      continue;
    }
    // **A PARENTHETICAL QUALIFIER IS NOT PART OF THE ID, AND READING IT AS ONE
    // MANUFACTURES A BROKEN REFERENCE AGAINST A CLEAN ESTATE.** Three rows
    // fleet-wide carry `AC-04.1 (render contract)`; the whole span was compared
    // against the criterion ids, matched nothing, and the migrator reported a
    // dangling reference to a criterion that is sitting in the same file.
    //
    // Zero instances on this estate, which is why it was vc's to find and why
    // the fixture below is constructed rather than captured.
    match span.split_once(" (") {
      Some((id, qualifier)) => {
        let qualifier = qualifier.trim_end().trim_end_matches(')').trim();
        ids.push(id.trim().to_string());
        if !qualifier.is_empty() {
          qualifiers.push((id.trim().to_string(), qualifier.to_string()));
        }
      }
      None => ids.push(span.to_string()),
    }
  }
  Some(Covers { ids, qualifiers })
}

/// Everything after the status value, verbatim: the row's note.
///
/// **This read `field(rest, "test")` and lost the note on almost every authored
/// row in the estate.** Measured against a corpus of 14 v2-authored rows
/// captured by vc from three threads predating issue 0056: **all 14 failed to
/// round-trip.** Twelve carry an UNKEYED note -- ` -- doc / eyeball`,
/// ` -- red-first; modules check -- unregistered fixture flagged` -- and
/// `field` found no ` -- test: ` to key on, so the note never reached the model
/// at all. The other two carry ` -- test: <text>` and came back as ` -- <text>`,
/// because the renderer writes the note unkeyed. **A row that round-trips to a
/// different string which is still a valid row is the worse of the two failures.**
///
/// **v2 settles what the region is: `AT_G_NOTE='( -- .*)?'`, greedy to end of
/// line, is v2 declining to parse it.** So there is exactly ONE tail and it has
/// no interior structure. vc's ruling: a keyed parse of an unkeyed region invents
/// structure the author never asserted -- ` -- test: X` and ` -- X` are the same
/// note in v2's model, and treating the first as data and the second as absent is
/// v3 deciding a distinction the canon does not make. That is how the old reader
/// could drop twelve notes while looking correct: it was faithful to a grammar
/// nobody ratified.
///
/// The consequence for callers is that a note may itself contain ` -- `, which is
/// the common case rather than the exotic one: nine of the corpus's rows have a
/// note introduced by the separator that then contains it. Nothing downstream may
/// split on it.
fn note(row: &str) -> Option<String> {
  const MARKER: &str = " -- status: ";
  let start = row.find(MARKER)? + MARKER.len();
  let rest = &row[start..];
  let sep = rest.find(" -- ")?;
  let tail = rest[sep + " -- ".len()..].trim();
  (!tail.is_empty()).then(|| tail.to_string())
}

/// The value of ` -- <key>: ...`, up to the next ` -- ` or the end.
fn field(row: &str, key: &str) -> Option<String> {
  let marker = format!(" -- {key}: ");
  let start = row.find(&marker)? + marker.len();
  let rest = &row[start..];
  let end = rest.find(" -- ").unwrap_or(rest.len());
  Some(rest[..end].trim().to_string())
}

/// Split v2's `---` frontmatter from the body.
///
/// Line-oriented `key: value`, exactly like the whiteboard header block and for
/// the same reason: it is hand-written prose-bearing data, and a
/// quoting-sensitive parser on hand-written values fails on the values people
/// actually write.
fn frontmatter(text: &str) -> (BTreeMap<String, String>, &str) {
  let mut map = BTreeMap::new();
  let Some(rest) = text.strip_prefix("---\n") else {
    return (map, text);
  };
  let Some(end) = rest.find("\n---\n") else {
    return (map, text);
  };
  for line in rest[..end].lines() {
    if let Some((k, v)) = line.split_once(':') {
      map.insert(k.trim().to_string(), v.trim().to_string());
    }
  }
  (map, &rest[end + 5..])
}

/// `# ST0001: Some title` -> `Some title`.
fn title(body: &str) -> Option<String> {
  let line = body.lines().find(|l| l.starts_with("# "))?;
  let after = line.trim_start_matches("# ").trim();
  Some(match after.split_once(ididy_separator(after)?) {
    Some((_, title)) => title.trim().to_string(),
    None => after.to_string(),
  })
}

/// The `: ` that separates an id prefix from a title, when there is one.
fn ididy_separator(heading: &str) -> Option<&'static str> {
  heading.contains(": ").then_some(": ")
}

/// v2's work-package template, embedded verbatim at
/// `lib/templates/prj/st/WP/info.md`, revision `0b1b3b5b`.
///
/// **Embedded rather than read from the install, and that is vc's condition 3
/// rather than a convenience.** The drop set has to be EXACTLY ONE template
/// version or it becomes a function of which Intent happens to be on the
/// machine -- the subject moving under the instrument, so the same estate
/// migrated twice would lose different sections with nothing recording why.
/// Pinned here, the drop set is re-derivable by someone who was not there.
///
/// **v2 is frozen, so this has no future revisions to track.**
const WP_TEMPLATE_V2: &str = r#"---
verblock: "[Date]:v0.1: [Author] - Initial version"
wp_id: WP-NN
title: "[Title]"
scope: Small
status: Not Started
---

# WP-NN: [Title]

## Objective

[Clear statement of what this work package aims to accomplish]

## Deliverables

- [List of concrete deliverables]

## Acceptance

Acceptance Criteria for this work package live in the steel thread's `acceptance.md`, under the `WP-NN` heading (single source of truth). Do not restate ACs here.

## Dependencies

- [List any dependencies on other WPs or external factors]
"#;

/// The template as it would have been WRITTEN for work package `seq`, parsed
/// by the same reader the estate goes through.
///
/// **The substitution is the whole point and its absence was a real hole.**
/// `bin/intent_wp:113` creates every work package with
/// `sed -e "s/WP-NN/WP-$WP_NUM/g"`, so the artefact that produced these files
/// is not the template -- it is the template with that substitution applied,
/// which is citable to a line of shell rather than inferred from shape. A
/// section carrying a placeholder can therefore never be byte-identical to the
/// raw template, and `## Acceptance` is the one template section whose body
/// carries one.
///
/// Measured against the raw template, `## Acceptance` matched **0 of 40** on
/// this estate while `Deliverables` and `Dependencies` matched 20 each -- and
/// the 40 it missed are exactly the 40 that were doubling in the rendered
/// view. **Comparing against the raw template compares against a source no
/// file was ever a copy of.**
///
/// Only `WP-NN` is applied. The template's other placeholders (`[Title]`,
/// `[Date]`, `[Author]`) live in the frontmatter and the `# ` title line,
/// which are outside every `## ` section.
fn wp_template_sections(seq: u32) -> Vec<(String, String)> {
  let filled = WP_TEMPLATE_V2.replace("WP-NN", &format!("WP-{seq:02}"));
  let (_, body) = frontmatter(&filled);
  sections(body)
}

/// `## Heading` sections with their verbatim bodies, **in the order the author
/// wrote them**.
///
/// **A `Vec` and not a map, and it was a `BTreeMap` until the reassembled
/// document was compared to the authored one.** The consumer below rebuilds a
/// body by joining these, so a map ordered the result ALPHABETICALLY: measured
/// across this estate, 140 of 140 work packages come back in an order nobody
/// wrote. `ST0056/WP/03` authors Objective / Deliverables / Acceptance /
/// Dependencies and was emitting Acceptance / Deliverables / Dependencies /
/// Objective.
///
/// **The comment on the consumer was true and the conclusion drawn from it was
/// not**: every section survived, so a per-section check passed on the whole
/// population while the document was reordered in every one of them. A section
/// is conserved; a document is a section list PLUS an order, and only a
/// document-level comparison can see the difference. (vc measured it; ic then
/// caught themselves about to copy the same line into the thread parser, where
/// it would have closed a real hole by propagating this defect and made the
/// section counts reconcile while doing it.)
///
/// **A `Vec` also stops a repeated heading being swallowed.** `insert` on a map
/// let a second `## Notes` overwrite the first and the loss was invisible; two
/// entries now come back as two.
fn sections(body: &str) -> Vec<(String, String)> {
  let mut out = Vec::new();
  let mut current: Option<String> = None;
  let mut buffer = String::new();
  for line in body.lines() {
    if let Some(heading) = line.strip_prefix("## ") {
      if let Some(name) = current.take() {
        out.push((name, buffer.trim().to_string()));
      }
      buffer.clear();
      current = Some(heading.trim().to_string());
    } else if current.is_some() {
      buffer.push_str(line);
      buffer.push('\n');
    }
  }
  if let Some(name) = current {
    out.push((name, buffer.trim().to_string()));
  }
  out
}

/// One named section's body, or nothing.
///
/// **First match rather than last**, which is the map's old behaviour inverted
/// on purpose: with duplicates now preserved, the authored document's first
/// `## Objective` is the one a reader sees at the top of the file.
fn section(sections: &[(String, String)], name: &str) -> String {
  sections
    .iter()
    .find(|(k, _)| k == name)
    .map(|(_, v)| v.clone())
    .unwrap_or_default()
}

/// v2 writes `20260814`; the model holds `2026-08-14`.
///
/// A value that is neither is passed through untouched rather than reshaped --
/// an absent date stays absent, and an unrecognised one stays exactly as
/// authored so the residue can name it.
fn date(raw: Option<&String>) -> String {
  let Some(raw) = raw.map(|s| s.trim()) else {
    return String::new();
  };
  if raw.len() == 8 && raw.bytes().all(|b| b.is_ascii_digit()) {
    format!("{}-{}-{}", &raw[0..4], &raw[4..6], &raw[6..8])
  } else {
    raw.to_string()
  }
}

/// The first line carrying a git conflict marker, if any.
fn conflict_marker_line(text: &str) -> Option<u32> {
  text
    .lines()
    .enumerate()
    .find(|(_, l)| l.starts_with("<<<<<<< ") || l.starts_with(">>>>>>> ") || *l == "=======")
    .map(|(i, _)| (i + 1) as u32)
}
