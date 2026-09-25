//! The sync engine's change detection -- a git-style index over the project's
//! files (data-model.md `file_index`).
//!
//! **The hash is computed on every scan, and stat is never a gate on it.**
//! design.md sketched "stat scan (mtime/size, SHA-256 rehash on change)", but
//! AC-03.3 requires detecting a same-size same-mtime rewrite, which no amount
//! of stat comparison can see: the whole point of that case is that stat is
//! unchanged while content is not. Where the contract and the architecture
//! narrative disagree, the contract governs (vc ruling, 2026-08-14) -- so size
//! and mtime are carried as reporting metadata, and content identity is
//! decided by SHA-256 alone.
//!
//! The cost is hashing a few hundred files per invocation. If that ever bites,
//! the optimisation is a recorded deviation with its own evidence, never an
//! accident that silently reintroduces the blind spot.

use std::collections::BTreeSet;
use std::io;
use std::path::{Path, PathBuf};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::finding::{Finding, FindingClass};

/// Root-level files inside the sync scope. Explicit and reviewed, never a
/// glob: the scope is "Intent's own artefacts", and a glob over the repo root
/// would sweep in whatever the project happens to keep there.
///
/// `.gitignore` is deliberately absent -- migration converges it, but nothing
/// ingests it, and a file in the index that no reader consumes is a claim the
/// tool cannot back.
pub const ROOT_FILES: &[&str] = &["AGENTS.md", "CLAUDE.md", "usage-rules.md"];

/// Paths under `intent/` that the scan does not walk.
///
/// `.cache/` holds the DB, which under D01 as reversed is TRUTH rather than an
/// index of these files -- scanning it would ingest the store as though it were
/// a source document. `.treeindex/` is an untracked derived cache (issue 0018).
///
/// **`.treeindex` STAYS HERE THROUGH THE RETIREMENT, DELIBERATELY.** hv retired
/// treeindex on 2026-08-15 and v3 never generates the cache -- but a consumer
/// migrating off v2 carries the directory on disk for as long as they keep it,
/// and indexing it would publish one machine's stale prose into the canon.
/// Retiring a producer is not a reason to stop recognising its residue.
/// `.backup/` matters for the same reason once D35's rolling snapshots land
/// there: a copy of truth must never re-enter through the ingest gate.
pub const SKIPPED_DIRS: &[&str] = &[".cache", ".treeindex", ".backup"];

/// One member of the set an extract cannot carry.
///
/// **TWO PHRASES, BECAUSE THE ONE A REPORT READS WELL WITH IS NOT THE ONE THE
/// DOCUMENT USES.** `shown` is what the operator sees; `justified_by` is the
/// literal text `data-model.md`'s `## What is deliberately not modelled` must
/// still contain for that member to be declarable at all. Collapsing them
/// would force a choice between a note that reads like a filename list and a
/// derivation that cannot be checked.
pub struct NotCarried {
  /// How the qualifier names it.
  pub shown: &'static str,
  /// The phrase in `data-model.md` that authorises naming it.
  pub justified_by: &'static str,
}

/// **WHAT `sync --to-disk` DOES NOT WRITE.** Declared HERE, beside the scope
/// constants, because it is the same kind of fact they are: what this
/// operation's reach is, and therefore is not.
///
/// **IT IS DERIVED FROM `data-model.md` AND PINNED TO IT BY A TEST, NEVER
/// PARSED AND NEVER FREELY HARDCODED.** The section is prose and two of its
/// three entries are categories rather than paths, so no parse exists; but a
/// set this code chose for itself would be a denominator certifying itself --
/// ic's `--out-of-model` attack, where naming everything zeroes the loss.
/// `egest_estate.rs` requires the document to still carry every `justified_by`
/// below, so the declaration cannot outlive the sentences it came from.
///
/// **EACH ONE IS MEASURED, NOT ASSUMED.** A fixture holding an instance of all
/// three ingests them, deletes the estate and egests: none of the three
/// returns, and freeform prose is absent from the store entirely rather than
/// merely unprojected. That is why the note may say *not modelled* -- the
/// stronger and more specific claim -- rather than hedging about reach.
pub const NOT_CARRIED: &[NotCarried] = &[
  NotCarried {
    shown: "prose",
    justified_by: "Prose (stored verbatim",
  },
  NotCarried {
    shown: "shipped content",
    justified_by: "shipped content",
  },
  NotCarried {
    shown: "wip/restart",
    justified_by: "wip.md / restart.md",
  },
];

/// The claim `intent sync --to-disk` makes, **qualified in its own sentence.**
///
/// **THE QUALIFIER IS ON THE CLAIM AND IS NOT AN ENUMERATION BESIDE IT** (vc's
/// ruling on AC-10.8, 2026-08-31, under hv's pen granted 2026-08-22). The
/// criterion requires the out-of-model set *"named in the output rather than
/// silently absent"*, and three readings were put up that all answered *where
/// do we print the list*. The ruling refused all three: the defect is that
/// `ok: extract written for 301 thread(s)` **claims a completeness it does not
/// have**, so an operator reads it and believes the extract is the estate.
/// Naming the set anywhere else -- a second line, `doctor`, `export` -- leaves
/// that sentence making the same claim.
///
/// **AND *ARE UNCHANGED* IS THE HALF AN OPERATOR ACTS ON**, so it is measured
/// by its own arm rather than asserted here: an egest over a live estate must
/// leave all three byte-identical. A qualifier that quietly stopped being true
/// One class of thing the model CLAIMS and no build carries yet.
///
/// **A BUILD GAP IS NOT A REACH DECLARATION AND MUST NOT BE SPELLED LIKE ONE.**
/// [`NOT_CARRIED`] says *the model does not cover this*; a member here says
/// *the model covers this and the code has not caught up*. The first closes by
/// fiat and permanently, the second closes on its own when the work lands --
/// **and a gap that expires is worth more than a zero that never does** (vc,
/// 2026-08-31).
pub struct NotYetBuilt {
  /// How the report names it.
  pub shown: &'static str,
  /// Where in the estate it lives, so the operator can see it is still there.
  pub at: &'static str,
  /// The phrase in `data-model.md` that puts it INSIDE the model.
  pub justified_by: &'static str,
  // **THE WORK PACKAGE THAT OWES IT IS DELIBERATELY NOT A FIELD HERE.**
  //
  // It was one for an hour, and `no_pm_state_in_output.rs` refused it: a
  // shipped string literal carrying `WP-14` is one edit away from a terminal,
  // and D37 rules that our own work-package numbers do not reach a user's.
  // Removing the render was not enough and the guard was right to be stricter
  // than the ruling.
  //
  // **AND DROPPING IT OUTRIGHT IS BETTER THAN EXEMPTING IT, BECAUSE THE NUMBER
  // ALREADY HAS A HOME.** `data-model.md` says the whiteboard is "built in
  // WP-14" in the same sentence `justified_by` pins to -- so a second copy here
  // would be the divergent-copy shape in a struct whose whole purpose is
  // deriving a claim from that document rather than restating it.
  //
  // **This is the identical mistake `unwired`'s message made and D37 corrected**,
  // repeated by the author who had read that comment the same morning while
  // repairing that very function.
}

/// **WHAT THE MODEL CLAIMS THAT THE MIGRATOR DOES NOT YET CARRY.**
///
/// **THIS DECLARATION EXISTS BECAUSE ITS ABSENCE ALMOST PRODUCED THE OPPOSITE
/// CLAIM.** Driving the full `AC-10.5` corpus, `conservation_check.sh` reported
/// 135 UNACCOUNTED files on baize and 347 on the canary, and issue `0183` was
/// filed proposing the migrator name them as its out-of-model set. **110 of
/// baize's 135 -- 81% -- are `intent/whiteboard/`, which `data-model.md:510`
/// says LEFT the not-modelled set at D30 and is modelled as
/// `wb_node`/`wb_item`/`wb_message`.** Declaring them out-of-model would have
/// contradicted an hv ruling that moved them INTO the model, and it is exactly
/// the denominator attack [`NOT_CARRIED`] warns about two hundred lines above:
/// *a set this code chose for itself would be a denominator certifying itself.*
/// Pinning the classification to the document is the only thing that caught it;
/// care did not, and the author was quoting the warning at the time.
///
/// **NOTHING HERE IS LOST.** The files are untouched on disk. What is unmet is
/// the model's own claim, which is why this reports rather than refuses.
// **EMPTY BY DELIVERY, AND THE MECHANISM STAYS** (vc, ruled 2026-09-14). The
// whiteboard was its one member until `wb register` and `wb migrate` carried it
// (ST0069), so no build gap is claimed today. The next class the model covers
// before a build carries it is a new member here, and every reader of this set
// already answers nothing for an empty one.
// Issue 0326 and 0363: the whiteboard stayed listed after the build that carries it landed, so upgrade reported it in every project.
pub const NOT_YET_BUILT: &[NotYetBuilt] = &[];

/// The migrator's twin of [`extract_written`], composed from the SAME
/// [`NOT_CARRIED`] because the two operations decline the same three
/// categories for the same reason.
///
/// **ONE DECLARATION, TWO SENTENCES.** A second const naming the same three
/// classes would be the Highlander defect in the one place the estate can
/// least afford it: the two would agree on the day they were written and drift
/// the first time `data-model.md` moves -- and the pinning test would pass on
/// both while they disagreed with each other.
pub fn migration_not_carried() -> String {
  let shown: Vec<&str> = NOT_CARRIED.iter().map(|m| m.shown).collect();
  format!(
    "not carried into the model: {} are not modelled and are unchanged on disk",
    match shown.split_last() {
      Some((last, rest)) if !rest.is_empty() => format!("{} and {last}", rest.join(", ")),
      _ => shown.join(", "),
    }
  )
}

/// The build-gap line, or `None` when the model has no unmet claim.
///
/// **`None` RATHER THAN AN EMPTY SENTENCE**, so the day WP-14 lands the line
/// disappears instead of reading "nothing is missing" -- which is a claim, and
/// one this function is not entitled to make about anything outside its list.
pub fn migration_not_yet_built() -> Option<String> {
  let each: Vec<String> = NOT_YET_BUILT
    .iter()
    .map(|m| format!("{} ({})", m.shown, m.at))
    .collect();
  if each.is_empty() {
    return None;
  }
  Some(format!(
    "not yet carried -- the model claims these and they are still on disk: {}",
    each.join(", ")
  ))
}

/// **THE SAME CLAIM AS [`migration_not_yet_built`], ONE RECORD PER ARTEFACT.**
///
/// The summary line says a directory is not carried; this says which files, by
/// path, each carrying the phrase that puts it inside the model. Both derive
/// from [`NOT_YET_BUILT`] and neither restates the other's content, so the day
/// a member is added or the day the build lands, the line and the enumeration
/// move together or not at all.
///
/// **IT LIVES AT THE DECLARATION AND NOT IN `legacy::scan`** (vc's ruling,
/// 2026-09-05, under hv's pen). Two independent reasons, and the second is the
/// stronger one:
///
/// - `legacy::scan`'s population carries the meaning *this is v2 thread
///   material*, which is false of all 624 of these files. Widening it would
///   also break its other caller: `ingest_from_md` reads a markdown estate, and
///   `migrate.rs` already records what happens when that walk acquires a
///   population every residue class must specially ignore.
/// - The enumeration inherits a contract that already binds it to the
///   document. `the_migrator_says_what_it_did_not_carry.rs` iterates
///   `NOT_YET_BUILT` and asserts each member's `justified_by` appears in
///   `data-model.md`, so a reason emitted from here cannot drift from the
///   ruling that authorises it without a test going red. That is `AC-10.5`'s
///   *the namer is the migration, not the check*, applied one level down.
///
/// **THE WALK HONOURS THE IGNORE RULES (D29), WHICH IS NOT A DETAIL HERE.** The
/// runtime store sits at a gitignored path inside the project, and a census
/// that named it would be reporting a per-machine database as an artefact the
/// model claims.
///
/// **EMPTY IS THE ORDINARY ANSWER AND IS NOT A ZERO WORTH PRINTING.** A project
/// with no whiteboard has nothing here, and the caller emits no section at all
/// rather than a heading over nothing -- the count scales with what the estate
/// actually holds, never with the size of this list.
pub fn migration_not_yet_built_artefacts(project: &crate::project::Project) -> Vec<Finding> {
  let mut out = Vec::new();
  for member in NOT_YET_BUILT {
    let dir = project.root().join(member.at);
    for rel in crate::project::Project::files_in(&dir) {
      let path = dir.join(&rel);
      out.push(Finding::new(
        project.relative(&path),
        FindingClass::ModelledNotBuilt,
        // **`shown` AND `justified_by`, NEVER A SENTENCE OF THIS FUNCTION'S
        // OWN.** The detail's whole authority is that it quotes the document,
        // and a phrasing invented here would be a second home for a ruling --
        // stale the day the ruling moves, with the pinning test still green
        // because it reads the const and not this string.
        //
        // **AND IT CARRIES ONLY WHAT VARIES**, which is `finding.rs`'s own
        // rule arriving here: *what is per-instance goes in the detail, and
        // the class string carries only what is true of the class*. The first
        // draft restated the class on every line -- measured on Lamplight,
        // 1,386 copies of one 150-character sentence -- which is a class
        // remedy inlined 1,386 times, printed once four lines later. What is
        // left is the two member fields, which genuinely differ the day
        // `NOT_YET_BUILT` holds two members.
        //
        // **"unchanged on disk" STAYS, SHORT.** It is the load-bearing half:
        // a line read on its own -- and these WILL be read one at a time, by
        // grep -- must not look like a loss report. The pinning test requires
        // the same phrase of the summary line for the same reason.
        format!(
          "{} -- unchanged on disk; `data-model.md` says it \"{}\"",
          member.shown, member.justified_by
        ),
      ));
    }
  }
  out
}

/// would be worse than the bare count it replaced.
pub fn extract_written(threads: usize) -> String {
  let shown: Vec<&str> = NOT_CARRIED.iter().map(|m| m.shown).collect();
  format!(
    "extract written for {threads} thread(s); {} are not modelled and are unchanged",
    // Oxford-less "a, b and c": the members are a closed declared set, so the
    // join is over whatever `NOT_CARRIED` holds and never a written-out list.
    match shown.split_last() {
      Some((last, rest)) if !rest.is_empty() => format!("{} and {last}", rest.join(", ")),
      _ => shown.join(", "),
    }
  )
}

/// The `--to-store` confirmation, composed here beside [`extract_written`] for
/// the same reason.
///
/// **It names the scope the run operated on and counts what CHANGED (0069).**
/// It used to say the STORE was replaced when one thread was, and to confirm
/// with the number of threads in scope -- a population, which read as a
/// delta: a run whose warning listed three issues confirmed `3 thread(s)`,
/// none of which had changed. `differences` is the overwrite list the
/// operator was shown before the write, so the two lines count the same thing.
pub fn store_restored(scope: &Scope, differences: usize) -> String {
  let what = match scope.named() {
    None => "store".to_string(),
    Some(ids) => ids.join(", "),
  };
  match differences {
    0 => format!(
      "{what} rewritten from the canon extract; nothing the store already held was overwritten"
    ),
    n => {
      format!("{what} replaced from the canon extract, taking the {n} difference(s) listed above")
    }
  }
}

/// The `--apply` confirmation (ST0078 WP-03), composed here beside
/// [`store_restored`] for the same reason.
///
/// **IT BEGINS `took` EXACTLY WHEN SOMETHING WAS TAKEN, AND THE GIT HOOKS READ
/// THAT WORD.** A hook prints one line when the pass changed the store and
/// nothing otherwise, and it tells the two apart by this sentence's first word;
/// `a_pull_is_reflected_by_the_next_verb.rs` holds the hook's template to it.
///
/// **THE EVENTS TAKEN ARE NAMED BESIDE THE SUBJECTS** (ST0078 P1), so a pull
/// that brought only event files still begins `took` and the hook still says
/// so. They are counted rather than listed: an event's id names nothing a
/// reader would look up, and a pull can carry a great many.
pub fn ingested(taken: &[String], events: &[String]) -> String {
  let events_said = match events.len() {
    0 => None,
    n => Some(format!("{n} event file(s)")),
  };
  match (taken, events_said) {
    ([], None) => "nothing to take -- the store already holds what the files say".to_string(),
    ([], Some(events)) => format!("took {events} from the files into the store"),
    (subjects, None) => format!(
      "took {} change(s) from the files into the store: {}",
      subjects.len(),
      subjects.join(", ")
    ),
    (subjects, Some(events)) => format!(
      "took {} change(s) from the files into the store: {}; and {events}",
      subjects.len(),
      subjects.join(", ")
    ),
  }
}

/// Which threads a `sync` run takes from its SOURCE.
///
/// **Both directions were whole-estate only until hv ruled otherwise on
/// 2026-08-19, and on a shared board that makes the routine act of saving your
/// own work a read of everybody else's.** vc measured it twice in one day,
/// both times while holding the pen and warning the others, and ran thirteen
/// estate-wide restores in one session -- each carrying whatever four nodes
/// happened to have on disk at that instant. dc's framing is why care cannot
/// fix it: a workflow whose correct form requires an operation only safe for
/// one actor is a single-writer bottleneck wearing a per-node procedure's
/// clothes.
///
/// **IT NAMES WHICH THREADS TAKE THEIR VALUE FROM THE SOURCE, NOT WHICH ONES
/// GET READ.** The distinction is load-bearing on the restore direction:
/// `sync_from_disk` finishes with a whole-store `rebuild`, so a scope that
/// merely narrowed the READ would hand `rebuild` a shortened set and DELETE
/// every thread it did not name. That is a far worse defect than the one this
/// exists to fix, and a silent one -- the node that ran it was saving its own
/// work. So an unnamed thread keeps the value the destination already holds
/// and the rebuild runs unchanged over the union, which keeps one write path
/// rather than growing a second store surface.
///
/// **There is no `None` variant and that is deliberate.** A scope that selects
/// nothing is a typo, not an intention, and the verbs refuse it rather than
/// reporting success over an empty selection -- otherwise a mistyped id is
/// indistinguishable from a completed sync and the operator believes their
/// work has landed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Scope {
  /// Every thread. The only form the verb had before the ruling, and still
  /// the right one for a fresh clone, a full regeneration, or the migrator.
  All,
  /// Only the threads named, by id.
  Threads(Vec<String>),
}

impl Scope {
  /// Does this scope take `id` from the source?
  pub fn selects(&self, id: &str) -> bool {
    match self {
      Scope::All => true,
      Scope::Threads(ids) => ids.iter().any(|named| named == id),
    }
  }

  /// The ids this scope names, or `None` when it names the whole estate.
  ///
  /// Callers that must REFUSE an unmatched id need the list; callers that only
  /// filter do not. Returning `None` for `All` keeps "no restriction" and "an
  /// empty restriction" from collapsing into one value, which is the same
  /// distinction the missing `None` variant above is about.
  pub fn named(&self) -> Option<&[String]> {
    match self {
      Scope::All => None,
      Scope::Threads(ids) => Some(ids),
    }
  }
}

/// What the scan concluded about one file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum FileState {
  /// Content identical to the last indexed state.
  Clean,
  /// Content differs from the last indexed state, or the file is new.
  Changed,
  /// The file is in a modelled location and cannot be read as what it claims
  /// to be. Commands needing it refuse (AC-03.5); nothing reads through it.
  Unparsed,
}

/// One row of the file index.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FileEntry {
  /// Project-relative, forward-slashed.
  pub path: String,
  /// Reporting metadata only -- never a gate on hashing. See the module note.
  pub size: u64,
  /// RFC 3339 UTC. Reporting metadata only.
  pub mtime: String,
  /// Lowercase hex SHA-256 of the file's bytes. The sole identity test.
  pub sha256: String,
  pub state: FileState,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub findings: Vec<Finding>,
}

#[derive(Debug, thiserror::Error)]
pub enum SyncError {
  #[error("reading {path}: {source}")]
  Io {
    path: String,
    #[source]
    source: io::Error,
  },
  #[error("formatting a timestamp: {0}")]
  Time(#[from] time::error::Format),
}

impl crate::remedy::Remedy for SyncError {
  fn remedy(&self) -> String {
    match self {
      Self::Io { path, .. } => format!("check that {path} is readable and that its directory is writable"),
      // Deliberately not an operator action: the format string is compiled in,
      // so a failure here is ours. A remedy telling someone to check their
      // input for a fault in our binary is the misdirection AC-04.4 exists to
      // prevent.
      Self::Time(_) => {
        "this is a build defect -- a timestamp format this binary carries did not apply, and nothing about the project caused it".to_string()
      }
    }
  }
}

fn io_err(path: &Path, source: io::Error) -> SyncError {
  SyncError::Io {
    path: path.display().to_string(),
    source,
  }
}

/// Scan the project's sync scope, deciding each file's state against the
/// previously indexed entries.
///
/// `previous` is the stored index; pass an empty slice for a first scan, where
/// every file is [`FileState::Changed`] because nothing has been ingested yet.
/// Output is ordered by path, so two scans of one tree are comparable without
/// the caller sorting.
/// Which paths a sync would READ, as a predicate rather than as a walk.
///
/// **ONE DEFINITION OF SCOPE WITH TWO CONSUMERS, AND THE SECOND ONE IS WHY THIS
/// EXISTS.** [`scan`] answers the question by walking, which is what a sync
/// needs. `intentd`'s watcher (`AC-08.5`) has to answer it about a path
/// somebody else just changed, and it cannot walk the tree to find out -- so
/// without this it would carry its own opinion about scope, and the two would
/// drift the first time either moved. **A watcher whose idea of scope differs
/// from the sync engine's either ingests nothing when it should or loops
/// forever when it should not.**
///
/// **THE LOOP IS NOT HYPOTHETICAL AND IT IS THE SHARPEST ARGUMENT FOR THE
/// `gitignore-aware` HALF OF `AC-08.5`.** The store lives at
/// `intent/.cache/intent.db`, INSIDE the tree a watcher watches, and every
/// ingest writes it. A watcher that triggered on any change under `intent/`
/// would trigger on the write its own ingest just made, forever, on an idle
/// machine. `.cache` is in [`SKIPPED_DIRS`] and the whole directory is
/// gitignored, so honouring scope is what stops it -- and honouring scope means
/// honouring THIS scope, not a similar one.
pub struct Scanned {
  /// **THE ROOT IS HELD RATHER THAN PASSED TO [`Scanned::includes`], SO A
  /// CALLER CANNOT ASK ABOUT ONE PROJECT WITH ANOTHER PROJECT'S SCOPE.** The
  /// ignore set is derived from a specific root; answering a question about a
  /// different one would be confidently wrong rather than refused, and the
  /// daemon serves N projects at once, which is exactly where two roots are in
  /// scope in one process.
  root: PathBuf,
  ignored: Ignored,
}

impl Scanned {
  /// Build the predicate for a project root.
  ///
  /// **NOT CHEAP, AND CALLED ONCE PER BATCH RATHER THAN ONCE PER PATH.**
  /// `Ignored::for_root` runs two walks; the note on that function records
  /// why the domain is narrow and what it cost when it was not.
  pub fn for_root(root: &Path) -> Scanned {
    Scanned {
      root: root.to_path_buf(),
      ignored: Ignored::for_root(root),
    }
  }

  /// Would the walk descend into this directory?
  fn descends(&self, dir: &Path) -> bool {
    let name = dir
      .file_name()
      .map(|n| n.to_string_lossy().into_owned())
      .unwrap_or_default();
    // **`.git` IS OUT BY RULE AND NOT BY BEING IGNORED**, because git does not
    // ignore its own directory -- it never walks it. The two-walk difference
    // never had to say so, because its domain could not reach `.git`; a matcher
    // asked about any path can, and would answer that the object database is in
    // the repository.
    name != ".git" && !self.ignored.contains(dir) && !SKIPPED_DIRS.contains(&name.as_str())
  }

  /// Would the walk keep this file?
  fn keeps(&self, file: &Path) -> bool {
    !self.ignored.contains(file)
  }

  /// Would a sync read this path?
  ///
  /// **THE ANCESTORS ARE CHECKED, NOT JUST THE FILE, BECAUSE THE WALK PRUNES AT
  /// DIRECTORIES.** An ignored or skipped directory removes its whole subtree,
  /// so a file inside `intent/.cache/` is out of scope while the file itself
  /// carries no ignore rule of its own. Asking only about the leaf would report
  /// the database as in scope, which is precisely the feedback loop above.
  pub fn includes(&self, path: &Path) -> bool {
    if !self.keeps(path) {
      return false;
    }
    let intent_dir = self.root.join("intent");
    if let Ok(rest) = path.strip_prefix(&intent_dir) {
      // **A BARE DIRECTORY IS A CONTAINER, NOT A MEMBER, AND `intent/` ITSELF
      // IS THE ONE THAT BIT US.** An empty remainder means the path IS the
      // intent directory, and every line below is about the components
      // BETWEEN that directory and a file: with none, `components.pop()` on an
      // empty vector is a no-op, the `descends` loop runs zero times, and the
      // function fell through to `true`. The skip list never got a chance to
      // speak, because there was no component left to test it against.
      //
      // **THAT IS HOW THE FEEDBACK LOOP THIS MODULE'S `includes` EXISTS TO
      // PREVENT WAS REACHED ANYWAY** (2026-09-12, dc): macOS coalesces a write
      // to `intent/.cache/` into a directory-granularity event on `intent/`,
      // the watcher asked this predicate, and got `true` -- so the daemon
      // published `fileChanged` naming a directory and ingested its own store
      // write. Membership is a question about a FILE; the watcher now answers
      // the directory question by reconciling the subtree
      // ([`changed_under`]) rather than by asking this.
      if rest.as_os_str().is_empty() {
        return false;
      }
      // Every directory between `intent/` and the file must be one the walk
      // would have descended into.
      let mut at = intent_dir.clone();
      let mut components: Vec<_> = rest.components().collect();
      components.pop();
      for component in components {
        at = at.join(component);
        if !self.descends(&at) {
          return false;
        }
      }
      return true;
    }
    // Otherwise the only in-scope paths are the root files, by name, at the
    // root itself -- the same set and the same place `scan` looks.
    ROOT_FILES
      .iter()
      .any(|name| path == self.root.join(name).as_path())
  }

  /// Is this path in the INDEX's scope -- the gitignore-aware repository?
  ///
  /// **ONE SCOPE OBJECT, TWO QUESTIONS, AND THEY ARE DIFFERENT QUESTIONS**
  /// (ST0069 AC-18.1, AC-18.4). [`Scanned::includes`] answers *would a SYNC
  /// read this*, whose corpus is the three [`ROOT_FILES`] and `intent/`; this
  /// answers *is this in the repository the index covers*, which is everything
  /// git would carry. Both derive from ONE ignore matcher, so there is still
  /// exactly one statement of what git ignores -- which is what AC-18.4 is
  /// about -- and neither question is answered by the other's rule.
  ///
  /// **`includes` IS DELIBERATELY NOT WIDENED INTO THIS.** The watcher consults
  /// it to decide what deserves an ingest, so widening it would put every
  /// source-file edit through a canon ingest the moment this landed -- before
  /// the index exists to want them, and before the watcher's own repair. The
  /// watcher is widened as WP-18's last step, on purpose and on its own.
  ///
  /// `.git/` is out BY RULE rather than by path shape: it is not gitignored --
  /// git does not ignore its own directory, it simply never walks it -- so a
  /// predicate that only asked the ignore matcher would take the object
  /// database into the corpus.
  pub fn in_repository(&self, path: &Path) -> bool {
    let Ok(rest) = path.strip_prefix(&self.root) else {
      return false;
    };
    if rest.as_os_str().is_empty() {
      return false;
    }
    if !self.keeps(path) {
      return false;
    }
    // Every directory between the root and the file must be one the walk would
    // descend into, for the reason `includes` gives: the walk prunes at
    // directories, so a file inside a pruned one carries no rule of its own.
    let mut at = self.root.clone();
    let mut components: Vec<_> = rest.components().collect();
    components.pop();
    for component in components {
      at = at.join(component);
      if !self.descends(&at) {
        return false;
      }
    }
    true
  }

  /// Does the index's scope reach this directory, or this path that has gone?
  ///
  /// [`Scanned::in_repository`] with the path's own name held to the walk's
  /// directory rule too. An event naming a directory or a vanished path cannot
  /// be classified as a file, so `.git`, an ignored directory and a skipped one
  /// are refused as themselves, and anything beneath them by their ancestors.
  // Issue 0355.
  pub fn reaches(&self, path: &Path) -> bool {
    self.in_repository(path) && self.descends(path)
  }
}

/// The in-scope files under `under` whose bytes differ from `previous`.
///
/// **THIS IS THE ANSWER TO A DIRECTORY-GRANULARITY EVENT, AND A DIRECTORY EVENT
/// IS A QUESTION RATHER THAN AN ANSWER.** The OS may report *something under
/// here changed* instead of naming the file -- macOS coalesces under load, and
/// a deleted path cannot be classified at all. Neither can be answered by
/// asking [`Scanned::includes`] about the directory: membership is a property
/// of a file. It is answered the way the tool answers that question everywhere
/// else, by reconciling against what the store last recorded.
///
/// **SCOPE AND POLICY STAY IN THIS MODULE, WHICH IS THE POINT OF THE FUNCTION
/// EXISTING AT ALL.** The walk is `walk` with the same [`Scanned`] the sync
/// engine uses, so the skip list speaks at the LEAF as the walk descends --
/// which is why the daemon's own `intent/.cache/` write reconciles to an empty
/// set without anything having to name `.cache` here. The comparison is
/// SHA-256 over the bytes, the sole identity test (D24), never mtime or size.
/// A watcher that walked the tree itself would be a second statement of scope,
/// and a second statement of scope drifts in the direction that loops.
///
/// A file the index has never seen differs by definition. `under` not existing
/// is not an error: a deleted subtree yields nothing in scope, which is the
/// honest answer and is what a caller reconciling a removal needs.
/// Does the store's recorded index already hold this path with these bytes?
///
/// **ONE HOME FOR *THE STORE HAS ALREADY SEEN THIS*, BECAUSE THE TWO CALLERS
/// ARE THE TWO DOORS AN EVENT ARRIVES THROUGH** (issue `0311`). A
/// directory-granularity event reconciles a subtree through
/// [`changed_under`]; a leaf event is judged one path at a time in the
/// daemon's watcher. Those were one question with one answer and one
/// implementation, and the leaf door had none at all -- so a file the daemon
/// itself had just written was published as a change, which is the feedback
/// loop scope exists to prevent, reached through the other door.
pub fn recorded_holds(previous: &[FileEntry], rel: &str, sha256: &str) -> bool {
  previous
    .iter()
    .any(|entry| entry.path == rel && entry.sha256 == sha256)
}

/// The bytes at `path` against what the store last recorded for it.
///
/// `Ok(true)` means publish-worthy: the file differs from the store's index, or
/// the store has never recorded it. `Ok(false)` means the store already holds
/// exactly these bytes -- an echo of a write the daemon itself made, or a touch
/// that changed nothing.
pub fn differs_from_recorded(
  root: &Path,
  path: &Path,
  previous: &[FileEntry],
) -> Result<bool, SyncError> {
  let bytes = std::fs::read(path).map_err(|e| io_err(path, e))?;
  let rel = crate::project::relative(root, path);
  Ok(!recorded_holds(previous, &rel, &sha256_hex(&bytes)))
}

pub fn changed_under(
  root: &Path,
  under: &Path,
  previous: &[FileEntry],
) -> Result<Vec<PathBuf>, SyncError> {
  if !under.exists() {
    return Ok(Vec::new());
  }
  let scope = Scanned::for_root(root);
  let changed = candidates(root, &scope)?
    .into_iter()
    .filter(|path| path.starts_with(under))
    .map(|path| {
      let rel = crate::project::relative(root, &path);
      let bytes = std::fs::read(&path).map_err(|e| io_err(&path, e))?;
      let sha256 = sha256_hex(&bytes);
      Ok((path, rel, sha256))
    })
    .collect::<Result<Vec<_>, SyncError>>()?
    .into_iter()
    .filter(|(_, rel, sha256)| !recorded_holds(previous, rel, sha256))
    .map(|(path, _, _)| path)
    .collect();
  Ok(changed)
}

/// The paths a scan of `root` reads, in ONE place.
///
/// **TWO ENUMERATIONS OF THE CORPUS ARE TWO STATEMENTS OF SCOPE, WHICH IS THE
/// THING [`Scanned`] EXISTS TO PREVENT.** [`walk`] keeps every file git would
/// commit under the directory it is handed, so walking from the project root --
/// which is where a coalesced directory event lands -- enumerates the whole
/// repository, while the corpus out there is [`ROOT_FILES`] by name and nothing
/// else. When [`changed_under`] had its own walk it reconciled `.prettierignore`
/// against the store's index and published it, for a file [`Scanned::includes`]
/// refuses as a leaf: one scope object, two answers, decided by which door the
/// event arrived through (2026-09-12, dc; vc's ruling is this function).
fn candidates(root: &Path, scope: &Scanned) -> Result<Vec<PathBuf>, SyncError> {
  let mut paths = Vec::new();
  for name in ROOT_FILES {
    let candidate = root.join(name);
    if candidate.is_file() && scope.keeps(&candidate) {
      paths.push(candidate);
    }
  }
  let intent_dir = root.join("intent");
  if intent_dir.is_dir() {
    walk(&intent_dir, scope, &mut paths)?;
  }
  paths.sort();
  Ok(paths)
}

/// Every file in the INDEX's scope: the gitignore-aware repository.
///
/// **THE COUNTERPART OF `candidates`, SHARING ITS WALK AND ITS SCOPE OBJECT.**
/// `candidates` enumerates the canon corpus -- the named [`ROOT_FILES`] plus
/// `intent/` -- and this enumerates everything git would carry. Two
/// populations, one walker, one statement of what git ignores, and neither
/// population is derived from the other's rule: that is the arrangement
/// [`Scanned::in_repository`] describes from the predicate side, and a second
/// walker here would undo it from the enumeration side.
///
/// The caller passes the scope so that a survey which also asks
/// [`Scanned::in_repository`] asks the same object rather than building a
/// second one -- the build is a filtered walk of the tree, and it is the
/// expensive half.
pub fn repository_files(root: &Path, scope: &Scanned) -> Result<Vec<PathBuf>, SyncError> {
  let mut paths = Vec::new();
  if root.is_dir() {
    walk(root, scope, &mut paths)?;
  }
  paths.sort();
  Ok(paths)
}

/// [`repository_files`] for what a batch of paths names, and nothing else.
///
/// The same walker and the same scope object: a named directory is walked when
/// [`Scanned::reaches`] admits it, a named file is kept when
/// [`Scanned::in_repository`] does, and the root names only its own files. The
/// answer is the whole-repository enumeration filtered to the named paths,
/// without paying for the rest of the repository. A path that has gone names
/// nothing here; its rows are the caller's to remove.
// Issue 0355: a refresh naming one file walked and surveyed every file in the
// repository.
pub fn repository_files_under(
  root: &Path,
  scope: &Scanned,
  under: &[PathBuf],
) -> Result<Vec<PathBuf>, SyncError> {
  let mut paths = Vec::new();
  for path in under {
    if path == root {
      for child in children(root)? {
        if child.is_file() && scope.keeps(&child) {
          paths.push(child);
        }
      }
    } else if path.is_dir() && scope.reaches(path) {
      walk(path, scope, &mut paths)?;
    } else if path.is_file() && scope.in_repository(path) {
      paths.push(path.clone());
    }
  }
  paths.sort();
  paths.dedup();
  Ok(paths)
}

pub fn scan(root: &Path, previous: &[FileEntry]) -> Result<Vec<FileEntry>, SyncError> {
  // **THE WALK AND THE PREDICATE ASK THE SAME OBJECT, WHICH IS THE WHOLE POINT
  // OF [`Scanned`].** Leaving this function with its own `ignored.contains` and
  // `SKIPPED_DIRS` checks would have made the predicate a SECOND statement of
  // scope that happened to agree today -- and a watcher built on a second
  // statement of scope drifts silently in the direction that loops.
  let scope = Scanned::for_root(root);

  let paths = candidates(root, &scope)?;

  let mut entries = Vec::with_capacity(paths.len());
  for path in paths {
    entries.push(entry_for(root, &path, previous)?);
  }
  Ok(entries)
}

/// The set of paths git would never commit, which are therefore never canon.
///
/// **D29 / AC-03.7.** Ingest walks the filesystem; git does not. On macOS every
/// directory acquires a `.DS_Store`, `.gitignore` excludes them, and strict
/// ingest then correctly refused a corpus containing what it correctly could
/// not parse -- so `intent search` exited 1 having read nothing, on a clean
/// checkout, on every Mac. Because AC-10.2 makes residue a migration BLOCK,
/// that failure propagated to the fleet rollout's first step.
///
/// **D05 is not weakened; the CORPUS is defined.** The rule still derives from
/// the truth model rather than being picked to fit -- but the derivation
/// changed under D01's reversal while the conclusion did not, which is worth
/// recording rather than quietly restating. It used to run: durable truth is
/// committed schema-validated JSON, so a path git can never commit can never be
/// canon. It now runs through D34: the committed extract is the interchange and
/// ingest is the only door into the DB, so a path git can never commit can
/// never TRAVEL, and therefore can never become canon -- and must never produce
/// residue or block a read.
///
/// **Not a `.DS_Store` special case, deliberately.** The same rule is already
/// load-bearing and currently held by luck: `intent/.cache/intent.db` escapes
/// today through path shape (`SKIPPED_DIRS`) rather than through any rule, and
/// WP-13 widens the corpus to the whole project for search, at which point a
/// binary SQLite file walks into scope. One rule now, or two bugs later.
///
/// Two edges, both of which are worse to get backwards than the original bug:
///
/// - It keys on IGNORED, never on untracked. A `thread.json` you just created
///   and have not committed must still ingest -- that is what most of a
///   working session looks like.
/// - A project with no git has no ignore rules, so nothing is ignored and the
///   corpus degrades to everything-in-scope rather than to nothing. That falls
///   out of the walker's `require_git` default rather than being special-cased.
///   Note that "no git" means no repository at or ABOVE the project, which is
///   git's own reading: a project nested inside an outer repository is subject
///   to that repository's committed rules, and correctly so.
///
/// **Only the repository's OWN committed rules count** -- not the user's
/// global excludes, and not `.git/info/exclude`. Found by vc: with the
/// walker's defaults, `intent/probe.sql` was silently out of corpus on their
/// machine because their `~/.gitignore_global` carries `*.sql`, and in corpus
/// on a machine without it. That is not a cosmetic difference. AC-10.2 makes
/// residue a migration BLOCK, so the same fleet member migrates cleanly for
/// one operator and blocks for another, with nothing in the repository to
/// explain why.
///
/// It also fails D29's own derivation on its own terms. The rule is "a path
/// git can NEVER commit can never be canon", and a path excluded only by my
/// global config is one `git add` away from being committed by anybody else --
/// it was never in that class. This repository already collides with it:
/// `.gitignore` carries `!schema/ddl.sql` purely to defeat a global `*.sql`,
/// so a committed, generated, load-bearing schema face was invisible to the
/// corpus on exactly the machines that have that global rule.
///
/// `.git/info/exclude` goes for the same reason one step weaker: it is
/// per-clone and uncommitted, so a fresh clone of the same repository
/// disagrees with this one about what the project contains.
struct Ignored {
  /// Lowest precedence first. See [`Ignored::contains`] for why every one is
  /// asked rather than the first match taken.
  matchers: Vec<ignore::gitignore::Gitignore>,
}

impl Ignored {
  /// Build the matchers for a project root.
  ///
  /// **A MATCHER, NOT A SET, AND THE DIFFERENCE IS THE WHOLE OF WP-18's FIRST
  /// RISK.** This enumerated the ignored paths by walking TWICE -- once with
  /// ignore rules off, once on -- and taking the difference. That technique
  /// gets git's answer from git's own rules rather than from a hand-maintained
  /// list, which is right, and its cost is the size of the UNFILTERED walk.
  /// Measured on this tree, 2026-08-18:
  ///
  /// ```text
  ///   paths the gitignore-respecting walk sees      1,929
  ///   paths the unfiltered walk visited           613,811
  ///   of which native/rust/target/                601,783
  /// ```
  ///
  /// **That is why `doctor` took ten seconds while every other verb took ten
  /// milliseconds**, and why the DOMAIN was narrowed to `intent/` plus the root
  /// at depth 1. WP-18 widens the scope to the repository, so narrowing the
  /// domain is no longer available -- and keeping the difference technique at
  /// the wider domain would rebuild that defect exactly, with an excess that is
  /// unbounded and machine-local: a build directory's size depends on who has
  /// compiled what and how recently.
  ///
  /// So the technique changes and the RULE does not. The `ignore` crate's
  /// gitignore matchers are built once and answered per path, enumerating
  /// nothing. The only walk left is the FILTERED one that finds the
  /// `.gitignore` files themselves -- a handful of paths, and a `.gitignore`
  /// inside an ignored subtree is irrelevant by construction, because that
  /// subtree is out.
  ///
  /// **THE COMMITTED RULES ONLY, WHICH IS THE RULING AS IT NOW STANDS.** The
  /// design carried an amendment honouring `.git/info/exclude` and the global
  /// excludes file; it was withdrawn on 2026-09-12 once the landed test was
  /// read. `ignored_paths_corpus.rs`'s `a_clone_local_exclude_does_not_shrink_
  /// the_corpus` derives the opposite from D29 itself -- *the rule is that a
  /// path git can NEVER commit can never be canon, and one excluded per-clone
  /// is one `git add` away from being committed by anybody who has not written
  /// that exclude*. Two operators with the same commit would otherwise
  /// disagree about what the project contains, and under AC-10.2 about whether
  /// it migrates. So `git_global(false)` and `git_exclude(false)` stay.
  ///
  /// **THE OTHER TWO HALVES OF THE OLD ANSWER ARE RESTORED EXPLICITLY, AND
  /// THEY DO NOT SURVIVE THE CHANGE OF INSTRUMENT ON THEIR OWN.** The
  /// difference technique got them free from the walker: `parents(true)` read
  /// the `.gitignore` files ABOVE a project nested inside a wider repository,
  /// and `require_git`'s default meant a tree with no repository had no rules
  /// at all. A matcher built only from the `.gitignore` files found INSIDE the
  /// root silently loses the first and silently gains the opposite of the
  /// second -- a project with no git but a `.gitignore` would start losing
  /// files from its corpus, which is the degradation [`Ignored`]'s own note
  /// rules out in as many words. Both are conditions on the ancestry rather
  /// than on anything the walk can see, so both are asked here.
  fn for_root(root: &Path) -> Self {
    // **NO REPOSITORY, NO RULES.** A tree git does not govern has nothing that
    // can never be committed, so the corpus degrades to everything-in-scope
    // rather than to nothing -- including when such a tree carries a
    // `.gitignore` file that no git would ever read.
    let Some(git_root) = root.ancestors().find(|dir| dir.join(".git").exists()) else {
      return Self {
        matchers: Vec::new(),
      };
    };

    let mut matchers = Vec::new();

    // The rules ABOVE the project, outermost first, when the project sits
    // inside a wider repository rather than being one. git reads them and so
    // does this: a nested project is subject to the repository it is in.
    let mut above: Vec<PathBuf> = Vec::new();
    if git_root != root {
      let mut at = root.parent();
      while let Some(dir) = at {
        above.push(dir.join(".gitignore"));
        if dir == git_root {
          break;
        }
        at = dir.parent();
      }
      above.reverse();
    }

    // The FILTERED walk, which is the cheap one: it visits what a
    // gitignore-respecting walk visits and nothing else.
    let mut found: Vec<PathBuf> = Vec::new();
    let mut vb = ignore::WalkBuilder::new(root);
    vb.hidden(false)
      .git_ignore(true)
      .parents(true)
      // Machine-local and clone-local respectively. Honouring either makes the
      // corpus a property of who is running the tool -- see the note above.
      .git_global(false)
      .git_exclude(false);
    for entry in vb.build().filter_map(Result::ok) {
      if entry.file_name() == ".gitignore" {
        found.push(entry.into_path());
      }
    }
    // Root-first, so a nested file's rules override the ones above it -- git's
    // rule that the closest `.gitignore` decides. The ancestors are already in
    // that order and are all shallower, so they go in front.
    found.sort_by_key(|p| p.components().count());
    above.append(&mut found);
    for file in above {
      let Some(dir) = file.parent() else {
        continue;
      };
      let mut b = ignore::gitignore::GitignoreBuilder::new(dir);
      b.add(&file);
      if let Ok(gi) = b.build()
        && !gi.is_empty()
      {
        matchers.push(gi);
      }
    }

    Self { matchers }
  }

  /// Would git ignore this path?
  ///
  /// **EVERY MATCHER IS ASKED AND THE LAST DECISIVE ANSWER WINS**, because that
  /// is how git resolves a negation: a deeper `.gitignore` un-ignoring what a
  /// shallower one ignored is the shape `!` exists for, and stopping at the
  /// first match would make the outermost rule permanent.
  ///
  /// A matcher whose own directory is not an ancestor of the path is skipped:
  /// its patterns say nothing about a path outside it, and asking anyway is how
  /// a matcher built for one subtree comes to judge another.
  fn contains(&self, path: &Path) -> bool {
    let is_dir = path.is_dir();
    let mut ignored = false;
    for matcher in &self.matchers {
      if !path.starts_with(matcher.path()) {
        continue;
      }
      match matcher.matched_path_or_any_parents(path, is_dir) {
        ignore::Match::Ignore(_) => ignored = true,
        ignore::Match::Whitelist(_) => ignored = false,
        ignore::Match::None => {}
      }
    }
    ignored
  }
}

/// Depth-first, name-ordered walk. Ordering is explicit because `read_dir`
/// order is filesystem-dependent, and an index whose row order varies by
/// machine cannot be compared across them.
fn walk(dir: &Path, scope: &Scanned, out: &mut Vec<PathBuf>) -> Result<(), SyncError> {
  for child in children(dir)? {
    if child.is_dir() {
      if !scope.descends(&child) {
        continue;
      }
      walk(&child, scope, out)?;
    } else if child.is_file() && scope.keeps(&child) {
      out.push(child);
    }
  }
  Ok(())
}

/// A directory's entries, sorted by name.
fn children(dir: &Path) -> Result<Vec<PathBuf>, SyncError> {
  let mut children: Vec<PathBuf> = std::fs::read_dir(dir)
    .map_err(|e| io_err(dir, e))?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| io_err(dir, e))?
    .into_iter()
    .map(|e| e.path())
    .collect();
  children.sort();
  Ok(children)
}

/// One file's index entry, hashed from its bytes on disk now.
///
/// `pub(crate)` because the facade records the canon files a projection lands
/// (0260), and a second hasher there would be a second answer to "what are
/// this file's bytes".
pub(crate) fn entry_for(
  root: &Path,
  path: &Path,
  previous: &[FileEntry],
) -> Result<FileEntry, SyncError> {
  let rel = crate::project::relative(root, path);
  let bytes = std::fs::read(path).map_err(|e| io_err(path, e))?;
  let (size, mtime) = stamp_of(path)?;
  let sha256 = sha256_hex(&bytes);

  let findings = inspect(&rel, &bytes);
  let state = if !findings.is_empty() {
    FileState::Unparsed
  } else if previous
    .iter()
    .any(|p| p.path == rel && p.sha256 == sha256 && p.state != FileState::Unparsed)
  {
    FileState::Clean
  } else {
    FileState::Changed
  };

  Ok(FileEntry {
    path: rel,
    size,
    mtime,
    sha256,
    state,
    findings,
  })
}

/// A file's size and modified time, spelled the way the index stores them.
///
/// **ONE ANSWER TO "WHEN DID THIS FILE LAST MOVE", BECAUSE TWO SPELLINGS OF AN
/// MTIME COMPARE UNEQUAL AND NOTHING SAYS WHY.** The search index's
/// stat-then-hash policy decides freshness by comparing a recorded stamp with
/// a fresh one, and the recorded one is written by [`entry_for`] here. A second
/// formatting of the same instant -- a truncated fraction, a different offset
/// spelling -- would make every file look modified on the first reconcile after
/// whichever of the two changed, and the symptom would be a slow index rather
/// than an error.
pub(crate) fn stamp_of(path: &Path) -> Result<(u64, String), SyncError> {
  let meta = std::fs::metadata(path).map_err(|e| io_err(path, e))?;
  Ok((
    meta.len(),
    OffsetDateTime::from(meta.modified().map_err(|e| io_err(path, e))?).format(&Rfc3339)?,
  ))
}

/// Everything that makes a file unreadable-as-what-it-claims-to-be.
fn inspect(rel: &str, bytes: &[u8]) -> Vec<Finding> {
  let Ok(text) = std::str::from_utf8(bytes) else {
    // **AN ATTACHMENT CLAIMS NOTHING ABOUT ITS BYTES (0084).** The collector
    // carries a non-UTF-8 attachment as opaque bytes, and retired its own
    // UTF-8 refusal to do so; this was the survivor, and it ran first -- so
    // the whole thread refused, under a remedy telling the operator to move
    // the one file the model keeps.
    if is_carried_attachment(rel) {
      return Vec::new();
    }
    return vec![Finding::new(
      rel,
      FindingClass::UnknownFileShape,
      "not valid UTF-8",
    )];
  };

  let mut findings = conflict_markers(rel, text);
  if findings.is_empty() {
    // **`.jsonl` IS HANDLED EXPLICITLY, and the reason is that it was already
    // handled correctly BY ACCIDENT.** `events.jsonl` does not end with
    // `.json` -- the suffix is one character longer -- so it escaped the
    // whole-document parse below through path shape rather than through any
    // decision. That is the same passing-by-luck D29 named for the database
    // file, arrived at a second time; a later `contains(".json")`, or an
    // extension normaliser, would start reporting the one file that carries
    // all history as malformed JSON, and a corrupt-looking history file blocks
    // every ingest.
    //
    // Reading it as what it is also buys the right diagnosis: a damaged line
    // is located, rather than the whole file being called broken.
    if rel.ends_with(".jsonl") {
      for (n, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
          continue;
        }
        if let Err(e) = serde_json::from_str::<serde_json::Value>(line) {
          findings.push(
            Finding::new(rel, FindingClass::MalformedJson, e.to_string()).at_line(n as u32 + 1),
          );
          break;
        }
      }
    } else if rel.ends_with(".json")
      && let Err(e) = serde_json::from_str::<serde_json::Value>(text)
    {
      findings.push(
        Finding::new(rel, FindingClass::MalformedJson, e.to_string()).at_line(e.line() as u32),
      );
    }
  }
  findings
}

/// Is `rel` (relative to the project root, as [`scan`] walks it) a file the
/// attachment collector carries: one under a thread's directory that
/// [`crate::project::Project::classify`] calls the author's -- or that
/// attachment's canon sidecar ([`crate::project::canon_blob_rel`]), which holds
/// the same bytes and so claims no more about them?
fn is_carried_attachment(rel: &str) -> bool {
  let Some(under) = rel
    .strip_prefix("intent/st/")
    .or_else(|| rel.strip_prefix("intent/.canon/st/"))
  else {
    return false;
  };
  match under.split_once('/') {
    Some((id, file)) => {
      crate::model::is_thread_id(id)
        && crate::project::Project::classify(Path::new(file))
          == crate::project::ThreadFile::Attachment
    }
    None => false,
  }
}

/// Git conflict markers.
///
/// Keyed on the `<<<<<<<` and `>>>>>>>` lines and requiring BOTH, never on the
/// `=======` divider: a run of `=` at the start of a line is also a setext H1
/// underline, which is ordinary markdown. Keying on the divider would have
/// reported Intent's own prose as conflicted.
fn conflict_markers(rel: &str, text: &str) -> Vec<Finding> {
  let mut opens = Vec::new();
  let mut closes = Vec::new();
  for (idx, line) in text.lines().enumerate() {
    let line_no = idx as u32 + 1;
    if is_marker(line, '<') {
      opens.push(line_no);
    } else if is_marker(line, '>') {
      closes.push(line_no);
    }
  }
  if opens.is_empty() || closes.is_empty() {
    return Vec::new();
  }
  opens
    .into_iter()
    .map(|line| {
      Finding::new(
        rel,
        FindingClass::ConflictMarkers,
        "git conflict markers present; resolve the merge before Intent can read this file",
      )
      .at_line(line)
    })
    .collect()
}

/// Exactly seven of `c`, then end-of-line or a space -- git's marker shape.
fn is_marker(line: &str, c: char) -> bool {
  let run = line.chars().take_while(|&ch| ch == c).count();
  run == 7 && line[run..].chars().next().is_none_or(|ch| ch == ' ')
}

/// The file's bytes as the index names them, or `None` when it cannot be read
/// now. One digest with [`entry_for`], so "what are this file's bytes" has one
/// answer (issue `0216` compares the two).
pub(crate) fn file_sha256(path: &Path) -> Option<String> {
  std::fs::read(path).ok().map(|bytes| sha256_hex(&bytes))
}

/// The hash of bytes already in hand.
///
/// **ONE HASHER FOR THE ESTATE, WHICH IS WHY THIS IS EXPOSED RATHER THAN
/// REIMPLEMENTED.** The index reads a file's bytes to turn them into rows and
/// needs the same answer the change detector would give about the same file;
/// a second hasher would be a second answer to "what are this file's bytes".
pub(crate) fn sha256_of(bytes: &[u8]) -> String {
  sha256_hex(bytes)
}

fn sha256_hex(bytes: &[u8]) -> String {
  hex(Sha256::digest(bytes).as_slice())
}

fn hex(bytes: &[u8]) -> String {
  bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Why a file's bytes are not in the index (ST0057 AC-03.5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotInIndex {
  /// Tracked, and the working tree differs from what is staged.
  Modified,
  /// Tracked, and gone from the working tree without the deletion being staged
  /// (issue 0315). `in_head` says whether HEAD holds its bytes -- which it
  /// usually does, and which is exactly what the `Modified` label denied.
  Deleted { in_head: bool },
  /// Not in the index at all, so no commit can contain these bytes.
  Untracked,
}

impl NotInIndex {
  fn describe(self) -> &'static str {
    match self {
      Self::Modified => "edited in the working tree and not staged",
      Self::Deleted { in_head: true } => {
        "deleted in the working tree and not staged, HEAD holds its bytes"
      }
      Self::Deleted { in_head: false } => {
        "deleted in the working tree and not staged, and no commit holds its bytes"
      }
      Self::Untracked => "untracked, so no commit contains it",
    }
  }
}

/// One file whose worktree bytes the index does not hold.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Uncommitted {
  /// Project-relative, forward-slashed -- the same spelling [`scan`] uses.
  pub path: String,
  pub state: NotInIndex,
}

impl std::fmt::Display for Uncommitted {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}: {}", self.path, self.state.describe())
  }
}

/// Run a git plumbing command under `root` and split its NUL-separated output.
///
/// **`-z` AND NUL SPLITTING, NEVER WHITESPACE, AND THIS ESTATE HAS THE
/// COUNTEREXAMPLE ON DISK.** It carries a file whose name contains spaces --
/// the `.webloc` AC-03.3's naming gate exists to reject -- and a whitespace
/// split of a path list turned that ONE file into EIGHT fragments while the run
/// printed a plausible four-digit total. Porcelain output additionally QUOTES
/// such paths, which a reader then has to unquote correctly or silently
/// mis-handle; `-z` emits them raw and the problem does not arise.
///
/// A non-zero exit is `None` rather than an empty list. The two are not the
/// same: no repository, or a git that could not run, means the question was not
/// answered, and reporting an unanswered question as "nothing is wrong" is how
/// a check comes to mean nothing.
/// Whether `root` sits inside a git work tree, asked WITHOUT resolving `HEAD`.
///
/// `rev-parse --is-inside-work-tree` answers from the repository's existence
/// alone, so it is true on a repo with no commits where every `HEAD`-resolving
/// command fails. That is the only question [`tree_state`] needs when
/// `diff-index` refuses, and asking it separately is what keeps "no git" and
/// "no commits yet" from collapsing into one answer.
fn is_work_tree(root: &Path) -> bool {
  std::process::Command::new("git")
    .args(["rev-parse", "--is-inside-work-tree"])
    .current_dir(root)
    .output()
    .map(|out| out.status.success() && out.stdout.starts_with(b"true"))
    .unwrap_or(false)
}

fn git_paths(root: &Path, args: &[&str]) -> Option<Vec<String>> {
  let out = std::process::Command::new("git")
    .args(args)
    .current_dir(root)
    .output()
    .ok()?;
  if !out.status.success() {
    return None;
  }
  Some(
    out
      .stdout
      .split(|b| *b == 0)
      .filter(|s| !s.is_empty())
      .map(|s| String::from_utf8_lossy(s).into_owned())
      .collect(),
  )
}

/// Which of `paths` have bytes the index does not hold (ST0057 AC-03.5).
///
/// # The comparison is against the INDEX, not against HEAD
///
/// `git commit` records the index, so the index is what the NEXT commit will
/// contain -- which makes it the right thing to ask about a file whose bytes are
/// on their way into canon. A comparison against HEAD would report every staged
/// file as uncommitted, which is the normal state of a commit being assembled,
/// and a check that fires on ordinary work is one people learn to skip.
///
/// # Two questions, because "not in the index" has two shapes
///
/// `diff-files` compares the working tree to the index and answers for TRACKED
/// files. It says nothing about a file the index has never heard of -- so an
/// attachment created ten seconds ago is absent from its output, and a check
/// built on it alone would be silent about the file whose bytes are LEAST
/// likely to be in any commit. `ls-files --others` is the other half.
///
/// # No repository is `None`, not an empty list
///
/// Same reason as `git_paths`: an unanswered question reported as a clean
/// answer is worse than no check. The caller decides what to do about not
/// knowing; this cannot decide it by returning a reassuring value.
pub fn uncommitted(root: &Path, paths: &[String]) -> Option<Vec<Uncommitted>> {
  let wanted: BTreeSet<&str> = paths.iter().map(String::as_str).collect();

  // Whole-repo queries intersected in memory, rather than passing 280 paths as
  // arguments -- an argv list that grows with the estate is a limit nobody
  // notices until the day it truncates, and a truncated list here reads as
  // "nothing is wrong".
  let modified = git_paths(root, &["diff-files", "--name-only", "-z"])?;
  // **A DELETION IS NOT AN EDIT** (issue 0315). `diff-files` lists a tracked
  // file gone from the working tree beside one edited in it, and labelling both
  // `Modified` told an operator that bytes HEAD holds were in no commit --
  // measured on seven carried attachments whose every byte HEAD held. git
  // answers which hits are deletions, so this asks git rather than the disk.
  let deleted: BTreeSet<String> = git_paths(
    root,
    &["diff-files", "--name-only", "--diff-filter=D", "-z"],
  )?
  .into_iter()
  .collect();
  let untracked = git_paths(root, &["ls-files", "--others", "--exclude-standard", "-z"])?;

  let mut out = Vec::new();
  for path in modified {
    if wanted.contains(path.as_str()) {
      let state = if deleted.contains(&path) {
        NotInIndex::Deleted {
          in_head: head_holds(root, &path),
        }
      } else {
        NotInIndex::Modified
      };
      out.push(Uncommitted { path, state });
    }
  }
  for path in untracked {
    if wanted.contains(path.as_str()) {
      out.push(Uncommitted {
        path,
        state: NotInIndex::Untracked,
      });
    }
  }
  out.sort_by(|a, b| a.path.cmp(&b.path));
  Some(out)
}

/// Whether HEAD holds a blob at `path` (repository-relative, as `diff-files`
/// spells it). No HEAD at all -- a repository with no commit -- holds nothing.
fn head_holds(root: &Path, path: &str) -> bool {
  std::process::Command::new("git")
    .args(["cat-file", "-e", &format!("HEAD:{path}")])
    .current_dir(root)
    .stderr(std::process::Stdio::null())
    .status()
    .is_ok_and(|s| s.success())
}

/// What git says about the working tree, for the MIGRATION preconditions.
///
/// **`NoWorkTree` is a third value and not a flavour of clean**, for
/// `git_paths`'s reason one level up: a question git could not answer is not
/// an answer, and the migration precondition it feeds refuses on exactly that.
pub enum TreeState {
  /// git reported no work tree here -- no repository, or no runnable git.
  NoWorkTree,
  Clean,
  /// Every path that would ride the next commit, sorted, never truncated.
  Dirty(Vec<Uncommitted>),
}

/// The migration precondition's read of the working tree.
///
/// # This asks HEAD, and [`uncommitted`] asks the INDEX, and that is deliberate
///
/// They are different questions and sharing a function would silently answer
/// one with the other. [`uncommitted`] asks *are this attachment's bytes on
/// their way into a commit*, so the index is right and its own docstring argues
/// against HEAD: staged work is the normal state of a commit being assembled,
/// and a check firing on ordinary work is one people learn to skip.
///
/// **THE MIGRATION ASKS THE OPPOSITE QUESTION AND WANTS THE OPPOSITE ANSWER.**
/// `migration.md`'s rollback is `git revert <the migration commit>`, and it is
/// cheap only because that commit contains the migration ALONE. Staged work is
/// precisely what would ride it -- `git commit` records the index as it stands
/// -- so a revert would take somebody's unrelated work with it. Against the
/// index a staged file reads clean, which is the one state this check exists to
/// catch.
///
/// Untracked files are the other half, for the same reason they are in
/// [`uncommitted`]: a file the index has never heard of is the one least likely
/// to be in any commit and the likeliest to be swept into the next one.
pub fn tree_state(root: &Path) -> TreeState {
  // **A REPOSITORY WITH NO COMMITS IS STILL A REPOSITORY, AND A DIFF AGAINST
  // `HEAD` CANNOT SAY SO.** It resolves `HEAD`, which does not exist until the first
  // commit, so it exits non-zero on a freshly `git init`ed tree -- exactly the
  // same signal as "there is no git here at all". Conflating the two made this
  // function answer `NoWorkTree` for a repo that plainly has a work tree, and
  // the migration then refused with *this project is not in a git repository*
  // in front of somebody who had just created one.
  //
  // **FOUND BY DRIVING IT, NOT BY READING IT** (2026-09-08): the fixtures for
  // 21 migration tests init a repo and commit nothing, which is the case the
  // two branches disagree about, and every one of them refused with the wrong
  // reason.
  //
  // With no HEAD there is nothing committed, so nothing can DIFFER from what
  // was committed; the whole estate is untracked and the `ls-files` arm below
  // reports it. An empty repo therefore reads as Dirty, which is the honest
  // answer -- a migration there is unprotected in precisely the way this
  // precondition exists to prevent.
  //
  // **PORCELAIN `diff`, NOT `diff-index`, BECAUSE A STAT CHANGE IS NOT A CHANGE.**
  // `diff-index` compares the index's cached stat data and reports every file
  // whose mtime or inode moved, so a tree `git status` called clean listed 25
  // "uncommitted" paths. `diff` refreshes that data in memory and compares
  // content; `--no-optional-locks` keeps the refresh from writing the index,
  // and `--no-renames` keeps a rename's source path in the list.
  let changed = match git_paths(
    root,
    &[
      "--no-optional-locks",
      "diff",
      "--no-renames",
      "--name-only",
      "-z",
      "HEAD",
      "--",
    ],
  ) {
    Some(changed) => changed,
    None if is_work_tree(root) => Vec::new(),
    None => return TreeState::NoWorkTree,
  };
  let Some(untracked) = git_paths(root, &["ls-files", "--others", "--exclude-standard", "-z"])
  else {
    return TreeState::NoWorkTree;
  };
  let mut out: Vec<Uncommitted> = changed
    .into_iter()
    .map(|path| Uncommitted {
      path,
      state: NotInIndex::Modified,
    })
    .chain(untracked.into_iter().map(|path| Uncommitted {
      path,
      state: NotInIndex::Untracked,
    }))
    .collect();
  if out.is_empty() {
    return TreeState::Clean;
  }
  out.sort_by(|a, b| a.path.cmp(&b.path));
  TreeState::Dirty(out)
}

#[cfg(test)]
mod tests {
  /// **`includes` IS A MEMBERSHIP PREDICATE FOR FILES, AND A BARE DIRECTORY IS
  /// A CONTAINER.** `intent/` itself returned `true` -- an empty remainder
  /// skipped every `descends` check below it -- and the watcher, handed a
  /// directory-granularity event by macOS, asked this and was told the
  /// daemon's own store write was in scope. Driven 2026-09-12 from the real
  /// failure, not from reading the code.
  #[test]
  fn the_intent_directory_itself_is_not_a_member_of_the_corpus() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    crate::init::init(root, "scoped", "dc", "3.0.2").expect("init a project");
    let scope = super::Scanned::for_root(root);

    assert!(
      !scope.includes(&root.join("intent")),
      "the `intent` directory reports as a member of the corpus, so a directory-granularity event on it passes the scope filter"
    );
    // The neighbouring truths, so the fix is pinned as NARROW: a real file
    // under it is still in scope, and the store is still out of it.
    assert!(
      scope.includes(&root.join("intent/wip.md")),
      "a file the sync reads stopped being in scope"
    );
    assert!(
      !scope.includes(&root.join("intent/.cache/intent.db")),
      "the store is in scope, which is the feedback loop itself"
    );
  }

  use super::*;

  #[test]
  fn a_setext_underline_is_not_a_conflict_marker() {
    let text = "Title\n=======\n\nbody\n";
    assert!(
      conflict_markers("intent/wip.md", text).is_empty(),
      "a run of = under a heading is markdown, not a merge conflict"
    );
  }

  #[test]
  fn an_open_marker_without_a_close_is_not_reported() {
    // A lone `<<<<<<<` line is far more likely to be prose about conflicts
    // than a conflict; requiring both ends is what makes the check safe to
    // run over documentation that discusses git.
    let text = "<<<<<<< HEAD\nno close marker\n";
    assert!(conflict_markers("intent/wip.md", text).is_empty());
  }

  #[test]
  fn a_real_conflict_is_reported_with_its_line() {
    let text = "a\n<<<<<<< HEAD\nours\n=======\ntheirs\n>>>>>>> branch\n";
    let found = conflict_markers("intent/wip.md", text);
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].line, Some(2));
    assert_eq!(found[0].class, FindingClass::ConflictMarkers);
  }

  #[test]
  fn eight_angle_brackets_is_not_a_marker() {
    assert!(!is_marker("<<<<<<<< eight", '<'));
    assert!(is_marker("<<<<<<< seven", '<'));
    assert!(is_marker("<<<<<<<", '<'));
  }
}
