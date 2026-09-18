//! The plan `intent sync` prints and `intent sync --apply` applies (ST0078).
//!
//! hv, 2026-09-18: nobody should type ten commands to bring a clone and its
//! store back into step. The shape hv chose is `intent sync [--apply]
//! [--to-disk|--to-store]`. Bare `intent sync` prints the plan for this clone
//! and writes nothing, `--apply` applies it, and the two explicit directions
//! keep their meanings.
//!
//! **ONE PLAN TYPE FOR THE WHOLE VERB.** P3 seeds it with one step, the
//! non-destructive ingest. P5 adds the rest (a branch behind its upstream,
//! unmerged paths, stale views, a stale index, doctor last) by adding
//! [`Action`] variants here, not by writing a second plan beside this one.
//!
//! **THE PLAN IS THE PURE HALF AND APPLYING IT IS THE IMPURE ONE** (PFIC). A
//! plan says what applying would do; it is built by
//! [`crate::facade::Facade::sync_plan`], which writes nothing, and applied by
//! [`crate::facade::Facade::sync_apply`], which computes each step again at the
//! moment it runs. So a plan shown a minute ago can be stale, and the apply
//! still does what the rule says now; P5's `--plan <digest>` is what will
//! refuse an apply after the tree moved.

/// Whether applying a step may ask a person, and which answer lets it run.
///
/// **QUIET STEPS NEVER ASK.** The hooks run `intent sync --apply` with no
/// terminal, which runs the quiet steps and skips every other, so a step is
/// quiet only when nothing it does can lose a person's work.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Recoverability {
  /// Runs without asking: nothing it does can lose work.
  Quiet,
  /// Asks `y/N`, and `--yes` answers for it (P5).
  Reversible,
  /// Always asks a person, and no flag answers for it (P5).
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

/// What one step does.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
  /// The store lags the committed canon: the daemon's non-destructive ingest,
  /// run from the command line (P3, AC-03.3). `would_take` is what the pass
  /// would change in the store, by subject, removals marked `(removed)`.
  Ingest { would_take: Vec<String> },
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
    Self {
      action: Action::Ingest { would_take },
      recoverability: Recoverability::Quiet,
    }
  }

  /// Whether applying this step would change anything.
  pub fn has_work(&self) -> bool {
    match &self.action {
      Action::Ingest { would_take } => !would_take.is_empty(),
    }
  }

  /// The step's name in a printed plan.
  pub fn name(&self) -> &'static str {
    match self.action {
      Action::Ingest { .. } => "ingest",
    }
  }

  /// What the step would do, in one sentence.
  pub fn describe(&self) -> String {
    match &self.action {
      Action::Ingest { would_take } => match would_take.as_slice() {
        [] => "nothing to take -- the store already holds what the files say".to_string(),
        subjects => format!(
          "take {} change(s) from the files into the store: {}",
          subjects.len(),
          subjects.join(", ")
        ),
      },
    }
  }
}

/// The ordered steps that would bring this clone and its store into step.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Plan {
  pub steps: Vec<Step>,
}

impl Plan {
  /// The steps that would change something, in order.
  pub fn work(&self) -> impl Iterator<Item = &Step> {
    self.steps.iter().filter(|s| s.has_work())
  }
}
