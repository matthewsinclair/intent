//! The plan `intent sync` prints and `intent sync --apply` applies (ST0078).
//!
//! hv, 2026-09-18: nobody should type ten commands to bring a clone and its
//! store back into step. The shape hv chose is `intent sync [--apply]
//! [--to-disk|--to-store]`. Bare `intent sync` prints the plan for this clone
//! and writes nothing, `--apply` applies it, and the two explicit directions
//! keep their meanings.
//!
//! **ONE PLAN TYPE FOR THE WHOLE VERB.** P3 seeded it with one step, the
//! non-destructive ingest. P5 adds the rest as [`Action`] variants here rather
//! than a second plan beside this one, in the order a pull's aftermath is
//! repaired:
//!
//! 1. the branch behind its upstream: said, and nothing run;
//! 2. unmerged paths Intent does not own: said, and left to the person;
//! 3. a canon add/add: the LOCAL id renumbered to the next free one;
//! 4. a canon content conflict: a person picks a side;
//! 5. the store lagging the committed canon: the ingest;
//! 6. unmerged generated views: regenerated from the merged canon and staged;
//! 7. views stale against the store: regenerated, as `organize --apply` does;
//! 8. committed event files the store does not hold: taken (P1);
//! 9. the search index stale against the tree: brought up to date;
//! 10. `doctor`, last, whose verdict is the exit code.
//!
//! **THE UNMERGED VIEWS COME AFTER THE INGEST, NOT BESIDE THE OTHER CONFLICTS**,
//! because a view is rendered from the store and the store holds the merged
//! canon only once the ingest has run. Regenerating one before that would
//! resolve the conflict with the pre-merge model.
//!
//! **THE PLAN IS THE PURE HALF AND APPLYING IT IS THE IMPURE ONE** (PFIC). A
//! plan says what applying would do; it is built by
//! [`crate::facade::Facade::sync_plan`], which writes nothing, and applied by
//! [`crate::facade::Facade::sync_apply`], which runs the steps a person has
//! agreed to and decides each again at the moment it runs. The plan carries
//! the tree's digest, and an apply naming a digest the tree no longer has is
//! refused before any step runs.
//!
//! **WHETHER A STEP RUNS, ASKS OR IS LEFT IS DECIDED HERE, BY [`gate`]**, so the
//! rule has one home and the command line only does the asking. A quiet step
//! never asks. A reversible one asks `y/N`, and `--yes` answers for it. A
//! non-reversible one always asks a person, and no flag answers for it. With no
//! terminal and no `--yes` the quiet steps run and every other is left, which
//! is what the git hooks run.

/// Whether applying a step may ask a person, and which answer lets it run.
///
/// The field the whiteboard verbs already carry, under the same three words.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Recoverability {
  /// Runs without asking: nothing it does can lose work.
  Quiet,
  /// Asks `y/N`, and `--yes` answers for it.
  Reversible,
  /// Always asks a person, and no flag answers for it.
  NonReversible,
}

impl Recoverability {
  /// The word a printed plan uses.
  pub fn as_str(self) -> &'static str {
    match self {
      Self::Quiet => "quiet",
      Self::Reversible => "reversible",
      Self::NonReversible => "non-reversible",
    }
  }
}

/// What a renumber moves: a steel thread or an issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Minted {
  Thread,
  Issue,
}

/// Which side of a merge a person chose.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
  /// This clone's side, `HEAD`.
  Ours,
  /// The side being merged in, `MERGE_HEAD`.
  Theirs,
}

impl Side {
  pub fn as_str(self) -> &'static str {
    match self {
      Self::Ours => "ours",
      Self::Theirs => "theirs",
    }
  }
}

/// What one step does.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
  /// The branch is behind the upstream it tracks. **Said, and nothing run**:
  /// Intent never pulls.
  Behind { upstream: String, commits: usize },
  /// Unmerged paths that are neither canon nor a generated view. Said, and
  /// left to the person: Intent does not own them.
  Unowned { paths: Vec<String> },
  /// Both clones minted `from`. This clone's record moves to `to`, the next id
  /// free in the store AND the tree, and the pulled side keeps `from`.
  Renumber {
    minted: Minted,
    from: String,
    to: String,
  },
  /// A canon file both sides changed. A person picks a side.
  TakeSide { path: String },
  /// The store lags the committed canon: the daemon's non-destructive ingest
  /// (P3, AC-03.3). `would_take` is what the pass would change in the store,
  /// by subject, removals marked `(removed)`, or `None` when it cannot be
  /// previewed until the canon conflicts above are resolved.
  Ingest { would_take: Option<Vec<String>> },
  /// Generated views git could not merge: regenerated from the merged canon,
  /// then staged.
  ResolveViews { paths: Vec<String> },
  /// Views stale against the store, as `organize --apply` finds them: files
  /// to write and files to remove. `None` when the canon conflicts above must
  /// be resolved first.
  RegenerateViews { would: Option<ViewWork> },
  /// Committed event files whose id the store does not hold (ST0078 P1),
  /// counted by name and read not at all. An event file cannot conflict, since
  /// its name is its id, so this step never waits on the merged canon.
  Events { to_take: usize },
  /// Files the search index has not caught up with.
  Reindex { paths: usize },
  /// `doctor`, last. Its verdict is the exit code.
  Doctor,
}

/// What regenerating the views would write and remove.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ViewWork {
  pub writes: Vec<String>,
  pub removes: Vec<String>,
}

/// One ordered step of a plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step {
  pub action: Action,
  pub recoverability: Recoverability,
}

impl Step {
  /// The ingest step. **QUIET**: it takes the files only where they say
  /// something the store did not write, so it cannot revert a local act.
  pub fn ingest(would_take: Vec<String>) -> Self {
    Self::quiet(Action::Ingest {
      would_take: Some(would_take),
    })
  }

  /// The ingest step while the tree holds conflict markers: it runs once they
  /// are resolved, and what it would take cannot be read before then.
  pub fn ingest_after_conflicts() -> Self {
    Self::quiet(Action::Ingest { would_take: None })
  }

  pub fn behind(upstream: String, commits: usize) -> Self {
    Self::quiet(Action::Behind { upstream, commits })
  }

  pub fn unowned(paths: Vec<String>) -> Self {
    Self::quiet(Action::Unowned { paths })
  }

  /// A renumber. **REVERSIBLE**: `renumber` moves it back.
  pub fn renumber(minted: Minted, from: String, to: String) -> Self {
    Self {
      action: Action::Renumber { minted, from, to },
      recoverability: Recoverability::Reversible,
    }
  }

  /// Taking a side. **NON-REVERSIBLE**: the side not taken is gone from the
  /// working tree, so a person decides.
  pub fn take_side(path: String) -> Self {
    Self {
      action: Action::TakeSide { path },
      recoverability: Recoverability::NonReversible,
    }
  }

  /// Regenerating and staging unmerged views. **REVERSIBLE**, because staging
  /// changes what the next commit holds; the views themselves are generated.
  pub fn resolve_views(paths: Vec<String>) -> Self {
    Self {
      action: Action::ResolveViews { paths },
      recoverability: Recoverability::Reversible,
    }
  }

  /// Regenerating stale views. **QUIET WHEN IT ONLY WRITES**, since every file
  /// it writes is rendered from the store; reversible when it removes one,
  /// because a removal is the act `organize` asks about.
  pub fn regenerate_views(would: Option<ViewWork>) -> Self {
    let removes = would.as_ref().is_some_and(|w| !w.removes.is_empty());
    Self {
      action: Action::RegenerateViews { would },
      recoverability: if removes {
        Recoverability::Reversible
      } else {
        Recoverability::Quiet
      },
    }
  }

  /// Bringing the index up to date. **QUIET**: it is the incremental
  /// reconcile every `intent search` already runs before it answers, it
  /// changes nothing but the index, and a step that asked here would put a
  /// line on every pull, since almost every pull leaves the index behind.
  pub fn reindex(paths: usize) -> Self {
    Self::quiet(Action::Reindex { paths })
  }

  /// Taking the committed event files the store lacks. **QUIET**: the take is
  /// additive, so it inserts records the store did not hold and changes none.
  pub fn events(to_take: usize) -> Self {
    Self::quiet(Action::Events { to_take })
  }

  pub fn doctor() -> Self {
    Self::quiet(Action::Doctor)
  }

  fn quiet(action: Action) -> Self {
    Self {
      action,
      recoverability: Recoverability::Quiet,
    }
  }

  /// Whether this step has anything to do or to say. `doctor` always runs and
  /// is not counted as work.
  pub fn has_work(&self) -> bool {
    match &self.action {
      Action::Behind { commits, .. } => *commits > 0,
      Action::Unowned { paths } | Action::ResolveViews { paths } => !paths.is_empty(),
      Action::Renumber { .. } | Action::TakeSide { .. } => true,
      Action::Ingest { would_take } => would_take.as_ref().is_none_or(|t| !t.is_empty()),
      Action::RegenerateViews { would } => would
        .as_ref()
        .is_none_or(|w| !w.writes.is_empty() || !w.removes.is_empty()),
      Action::Events { to_take } => *to_take > 0,
      Action::Reindex { paths } => *paths > 0,
      Action::Doctor => false,
    }
  }

  /// A step that says something and runs nothing.
  pub fn reports_only(&self) -> bool {
    matches!(self.action, Action::Behind { .. } | Action::Unowned { .. })
  }

  /// A step that resolves a canon conflict. Until every one of these has run,
  /// canon holds conflict markers and nothing can be read from it.
  pub fn resolves_canon(&self) -> bool {
    matches!(
      self.action,
      Action::Renumber { .. } | Action::TakeSide { .. }
    )
  }

  /// A step that reads the merged canon, and so waits on every step that
  /// resolves it.
  pub fn needs_merged_canon(&self) -> bool {
    matches!(
      self.action,
      Action::Ingest { .. } | Action::ResolveViews { .. } | Action::RegenerateViews { .. }
    )
  }

  /// The step's name in a printed plan.
  pub fn name(&self) -> &'static str {
    match self.action {
      Action::Behind { .. } => "behind",
      Action::Unowned { .. } => "unmerged",
      Action::Renumber { .. } => "renumber",
      Action::TakeSide { .. } => "take a side",
      Action::Ingest { .. } => "ingest",
      Action::ResolveViews { .. } => "resolve views",
      Action::RegenerateViews { .. } => "views",
      Action::Events { .. } => "events",
      Action::Reindex { .. } => "index",
      Action::Doctor => "doctor",
    }
  }

  /// What the step would do, in one sentence.
  pub fn describe(&self) -> String {
    match &self.action {
      Action::Behind { upstream, commits } => format!(
        "{upstream} holds {commits} commit(s) this branch does not -- pull them yourself; intent never pulls"
      ),
      Action::Unowned { paths } => format!(
        "{} unmerged path(s) that are not intent's to resolve: {}",
        paths.len(),
        paths.join(", ")
      ),
      Action::Renumber { minted, from, to } => format!(
        "both sides minted {} {from}: this clone's moves to {to} and the pulled one keeps {from}, both staged",
        match minted {
          Minted::Thread => "steel thread",
          Minted::Issue => "issue",
        }
      ),
      Action::TakeSide { path } => format!(
        "both sides changed {path}: a person chooses ours or theirs, and it is staged -- or resolve it by hand and `git add` it"
      ),
      Action::Ingest { would_take } => match would_take.as_deref() {
        None => {
          "take the merged canon into the store; what it takes can be read once the conflicts are resolved".to_string()
        }
        Some([]) => "nothing to take -- the store already holds what the files say".to_string(),
        Some(subjects) => format!(
          "take {} change(s) from the files into the store: {}",
          subjects.len(),
          subjects.join(", ")
        ),
      },
      Action::ResolveViews { paths } => format!(
        "regenerate {} unmerged view(s) from the merged canon and stage them: {}",
        paths.len(),
        paths.join(", ")
      ),
      Action::RegenerateViews { would } => match would {
        None => {
          "regenerate the views the merged canon changes; which ones can be read once the conflicts are resolved"
            .to_string()
        }
        Some(work) => {
          let mut said = format!("write {} view(s)", work.writes.len());
          if !work.removes.is_empty() {
            said.push_str(&format!(
              " and remove {}: {}",
              work.removes.len(),
              work.removes.join(", ")
            ));
          }
          said
        }
      },
      Action::Events { to_take } => format!("{to_take} event file(s) to take"),
      Action::Reindex { paths } => {
        format!("bring {paths} file(s) the search index has not caught up with into it")
      }
      Action::Doctor => "run doctor last; its verdict is the exit code".to_string(),
    }
  }
}

/// The ordered steps that would bring this clone and its store into step.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Plan {
  pub steps: Vec<Step>,
  /// The tree's digest as the plan read it, the value `organize` prints for
  /// the same purpose. `--plan <digest>` names it, and an apply after the tree
  /// moved is refused.
  pub digest: String,
}

impl Plan {
  /// The steps that would change or say something, in order.
  pub fn work(&self) -> impl Iterator<Item = &Step> {
    self.steps.iter().filter(|s| s.has_work())
  }
}

/// What applying a step needs before it runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gate {
  /// It runs without asking.
  Run,
  /// A person is asked.
  Ask,
  /// It is left, and named as left.
  Leave,
}

/// **THE ONE RULE FOR WHETHER A STEP RUNS** (AC-05.3).
///
/// `yes` is `--yes`, and it answers a reversible step and nothing else.
/// `terminal` is whether a person is there to ask. A non-reversible step asks
/// when one is and is left when one is not, whatever `yes` says, so no flag
/// and no environment variable can answer for a person.
pub fn gate(recoverability: Recoverability, yes: bool, terminal: bool) -> Gate {
  match recoverability {
    Recoverability::Quiet => Gate::Run,
    Recoverability::Reversible if yes => Gate::Run,
    Recoverability::Reversible | Recoverability::NonReversible if terminal => Gate::Ask,
    Recoverability::Reversible | Recoverability::NonReversible => Gate::Leave,
  }
}

/// What the caller of an apply knows about asking.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Asking {
  /// `--yes`: answers every reversible step, and nothing else.
  pub yes: bool,
  /// Whether a person is there to ask.
  pub terminal: bool,
  /// `--plan <digest>`: the digest of the plan a person read. The apply is
  /// refused when the tree no longer has it.
  pub shown: Option<String>,
}

/// A person's answer to one asked step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
  /// Run it.
  Run,
  /// Run it taking this side (a canon content conflict).
  Take(Side),
  /// Leave it.
  Decline,
}

/// Why a step was not run, or not run over every file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeftBecause {
  /// A person was asked and said no.
  Declined,
  /// It needs a person and none was there to ask.
  NeedsAPerson,
  /// It reads the merged canon and a canon conflict before it was left.
  Waits,
  /// It ran, and left these files as they are: each holds a change the store
  /// can take in, which rewriting it from the store would have discarded
  /// (issues 0554 (b), 0556 and the hooks' path in 0559's family). Paths are
  /// project-relative.
  Kept(Vec<String>),
}

/// A step that was not run, and why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Left {
  pub step: Step,
  pub because: LeftBecause,
}

/// **THE ONE LINE NAMING WHAT AN APPLY LEFT**, or `None` when it left nothing.
/// The command line prints it after `left: `.
///
/// It is what the hooks print after a pull, so it names each step with its
/// recoverability and says which command finishes the job.
pub fn left_line(left: &[Left]) -> Option<String> {
  if left.is_empty() {
    return None;
  }
  let named: Vec<String> = left
    .iter()
    .map(|l| {
      let why = match &l.because {
        LeftBecause::Declined => "declined".to_string(),
        LeftBecause::NeedsAPerson => l.step.recoverability.as_str().to_string(),
        LeftBecause::Waits => "waits on the conflicts".to_string(),
        LeftBecause::Kept(paths) => format!("kept {}", paths.join(", ")),
      };
      format!("{} ({why})", l.step.name())
    })
    .collect();
  let unrun: Vec<&Left> = left
    .iter()
    .filter(|l| !matches!(l.because, LeftBecause::Kept(_)))
    .collect();
  let mut finish = Vec::new();
  if !unrun.is_empty() {
    let reversible_only = unrun
      .iter()
      .all(|l| l.step.recoverability != Recoverability::NonReversible);
    finish.push(if reversible_only {
      "run `intent sync --apply` on a terminal, or with `--yes`"
    } else {
      "run `intent sync --apply` on a terminal; `--yes` answers the reversible steps and never a non-reversible one"
    });
  }
  // **THE CHOICE IS NAMED, NOT MADE FOR THE READER** (vc, 2026-09-24). A kept
  // file is usually what a pull brought, and then carrying it is right; a
  // checkout of an older commit puts bytes on disk the store never wrote too,
  // and carrying those rolls the store back. So the line says which version
  // wins rather than calling the verb a repair.
  if unrun.len() < left.len() {
    finish.push(
      "each kept file holds a change the store does not, and rewriting it from the store would have discarded it: `intent sync --to-store` carries the files' version into the store, over the store's; to keep the store's instead, delete the kept files and run `intent sync --to-disk`, which re-creates them from it",
    );
  }
  Some(format!(
    "{} step(s): {} -- {}",
    left.len(),
    named.join(", "),
    finish.join("; ")
  ))
}

/// The unmerged paths of a merge, sorted by who resolves them and how.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Conflicts {
  /// Ids both sides minted, each with what it names: canon add/adds.
  pub minted_twice: Vec<(Minted, String)>,
  /// Canon files both sides changed, and canon sidecars of a thread that is
  /// not being renumbered.
  pub take_sides: Vec<String>,
  /// Generated views, outside any renumbered id's paths.
  pub views: Vec<String>,
  /// Everything else: authored files Intent does not resolve.
  pub unowned: Vec<String>,
}

impl Conflicts {
  pub fn is_empty(&self) -> bool {
    self.minted_twice.is_empty()
      && self.take_sides.is_empty()
      && self.views.is_empty()
      && self.unowned.is_empty()
  }

  /// Whether canon holds conflict markers until a person or a renumber
  /// resolves them.
  pub fn canon_is_unmerged(&self) -> bool {
    !self.minted_twice.is_empty() || !self.take_sides.is_empty()
  }
}

/// Sort a merge's unmerged paths (AC-05.2). **PURE**: the paths come from
/// [`crate::gitstate::unmerged`] and every spelling from the project.
///
/// A canon file both sides ADDED is an id both clones minted, and the repair
/// is a renumber; every path under that id -- its canon sidecars, its realised
/// directory, its views -- belongs to the renumber and is not listed again. A
/// canon file both sides CHANGED is a person's choice. A generated view is
/// regenerated. Anything else is authored, and Intent does not resolve it.
pub fn classify(
  project: &crate::project::Project,
  unmerged: &[crate::gitstate::Unmerged],
) -> Conflicts {
  let rel = |p: std::path::PathBuf| format!("{}/", project.relative(&p));
  let canon_st = rel(project.canon_st_dir());
  let canon_issues = rel(project.issues_dir());
  let canon = rel(project.canon_dir());
  let st = rel(project.st_dir());
  let issue_views = rel(project.issues_view_dir());
  let patterns = project.generated_view_patterns();

  let mut out = Conflicts::default();
  // Paths whose owner is decided once every twice-minted id is known.
  let mut sidecars: Vec<(String, String)> = Vec::new();
  let mut under_threads: Vec<(String, String)> = Vec::new();
  let mut views: Vec<String> = Vec::new();

  for entry in unmerged {
    let path = entry.path.as_str();
    if let Some(rest) = path.strip_prefix(&canon_st) {
      match rest.split_once('/') {
        None => match rest
          .strip_suffix(".json")
          .filter(|id| crate::model::thread_seq(id).is_some())
        {
          Some(id) if entry.is_add_add() => out.minted_twice.push((Minted::Thread, id.to_string())),
          _ => out.take_sides.push(path.to_string()),
        },
        Some((id, _)) if crate::model::thread_seq(id).is_some() => {
          sidecars.push((id.to_string(), path.to_string()));
        }
        Some(_) => out.take_sides.push(path.to_string()),
      }
    } else if let Some(rest) = path.strip_prefix(&canon_issues) {
      match rest
        .strip_suffix(".json")
        .filter(|n| crate::model::issue_seq(n).is_some())
      {
        Some(n) if entry.is_add_add() => out.minted_twice.push((Minted::Issue, n.to_string())),
        _ => out.take_sides.push(path.to_string()),
      }
    } else if path.starts_with(&canon) {
      out.take_sides.push(path.to_string());
    } else if patterns
      .iter()
      .any(|p| crate::search::glob_matches(p, path))
    {
      views.push(path.to_string());
    } else if let Some((id, _)) = path
      .strip_prefix(&st)
      .and_then(|rest| rest.split_once('/'))
      .filter(|(id, _)| crate::model::thread_seq(id).is_some())
    {
      under_threads.push((id.to_string(), path.to_string()));
    } else {
      out.unowned.push(path.to_string());
    }
  }

  let renumbered = |minted: Minted, id: &str| {
    out
      .minted_twice
      .iter()
      .any(|(m, twice)| *m == minted && twice == id)
  };
  let thread_of_view = |path: &str| {
    path
      .strip_prefix(&st)
      .and_then(|rest| rest.split_once('/'))
      .map(|(id, _)| id.to_string())
  };
  let issue_of_view = |path: &str| {
    path
      .strip_prefix(&issue_views)
      .and_then(|rest| rest.strip_suffix(".md"))
      .map(str::to_string)
  };
  let mut take_sides = Vec::new();
  for (id, path) in sidecars {
    if !renumbered(Minted::Thread, &id) {
      take_sides.push(path);
    }
  }
  let mut unowned = Vec::new();
  for (id, path) in under_threads {
    if !renumbered(Minted::Thread, &id) {
      unowned.push(path);
    }
  }
  for path in views {
    let covered = thread_of_view(&path).is_some_and(|id| renumbered(Minted::Thread, &id))
      || issue_of_view(&path).is_some_and(|n| renumbered(Minted::Issue, &n));
    if !covered {
      out.views.push(path);
    }
  }
  out.take_sides.extend(take_sides);
  out.unowned.extend(unowned);
  out.take_sides.sort();
  out.unowned.sort();
  out.views.sort();
  out
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_quiet_step_runs_whatever_the_flags_say() {
    for (yes, terminal) in [(false, false), (true, false), (false, true), (true, true)] {
      assert_eq!(gate(Recoverability::Quiet, yes, terminal), Gate::Run);
    }
  }

  #[test]
  fn yes_answers_a_reversible_step_and_nothing_answers_a_non_reversible_one() {
    assert_eq!(gate(Recoverability::Reversible, true, false), Gate::Run);
    assert_eq!(gate(Recoverability::Reversible, false, true), Gate::Ask);
    assert_eq!(gate(Recoverability::Reversible, false, false), Gate::Leave);
    assert_eq!(gate(Recoverability::NonReversible, true, true), Gate::Ask);
    assert_eq!(
      gate(Recoverability::NonReversible, true, false),
      Gate::Leave
    );
    assert_eq!(
      gate(Recoverability::NonReversible, false, false),
      Gate::Leave
    );
  }

  #[test]
  fn the_left_line_names_each_step_and_the_command_that_finishes() {
    assert_eq!(left_line(&[]), None);
    let line = left_line(&[
      Left {
        step: Step::renumber(Minted::Thread, "ST0001".into(), "ST0003".into()),
        because: LeftBecause::NeedsAPerson,
      },
      Left {
        step: Step::take_side("intent/.canon/st/ST0002.json".into()),
        because: LeftBecause::NeedsAPerson,
      },
    ])
    .expect("two steps were left");
    assert!(
      line.starts_with("2 step(s): renumber (reversible), take a side (non-reversible)"),
      "{line}"
    );
    assert!(line.contains("never a non-reversible one"), "{line}");
  }

  #[test]
  fn a_regeneration_that_removes_asks_and_one_that_only_writes_does_not() {
    let writes = ViewWork {
      writes: vec!["intent/todo.md".into()],
      removes: Vec::new(),
    };
    assert_eq!(
      Step::regenerate_views(Some(writes)).recoverability,
      Recoverability::Quiet
    );
    let removes = ViewWork {
      writes: Vec::new(),
      removes: vec!["intent/st/ST0001/info.md".into()],
    };
    assert_eq!(
      Step::regenerate_views(Some(removes)).recoverability,
      Recoverability::Reversible
    );
  }
}
