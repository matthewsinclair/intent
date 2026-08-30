//! The acceptance contract: AC states, computed satisfaction, and the close
//! gate (AC-04.2, AC-04.3).
//!
//! One home for all three, because they are one question asked at different
//! volumes. v2 split them across `ac_state`, `ac_is_satisfied` and
//! `cmd_ac_gate` in `bin/intent_acceptance`, and the split is where issue 0015
//! came from -- a gate that counted a `to-write` row as coverage because the
//! comparison lived somewhere the gate could not see.
//!
//! **Satisfaction of a test-backed AC is COMPUTED and never stored.** The model
//! has no field for it (data-model.md), so there is nowhere to write a stale
//! answer even by accident. Non-test ACs carry their state inline because their
//! evidence is a human judgement with no green to read.

use crate::model::{AcKind, AcState, AtKind, AtStatus, Criterion, Thread, ThreadStatus};

/// What the gate is being asked about.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
  /// The whole thread.
  Thread,
  /// One work package, by seq. `WP-03` is `WorkPackage(3)`.
  WorkPackage(u32),
}

impl Scope {
  /// The AC group this scope selects: `00` for ST-level, else the WP seq.
  fn group(&self) -> Option<String> {
    match self {
      Self::Thread => None,
      Self::WorkPackage(seq) => Some(format!("{seq:02}")),
    }
  }
}

/// An AC's state as the gate SEES it. Four, not two (issue 0013): a
/// requirement can leave scope while remaining real.
///
/// **This is deliberately a different type from [`crate::model::AcState`], and
/// the pair is the collapse's design rather than an accident.** The model type
/// is what canon RECORDS and has a fifth variant, `Computed`, for a test-backed
/// criterion whose satisfaction is not stored anywhere. This type is what the
/// question resolves TO, and `Computed` is not one of its answers -- resolving
/// it is exactly the work [`resolve`] does. One type for what is written down,
/// one for what is true, and no way to confuse a stored `Computed` for an
/// answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resolved {
  Satisfied,
  Unsatisfied,
  Descoped,
  Withdrawn,
  /// Closed on human authority with the requirement unmet. Counts as closed --
  /// unblocking is what a fiat close is FOR -- and is reported separately from
  /// `Satisfied` everywhere it is counted.
  Fiat,
}

/// `AC-03.1` -> `03`.
pub fn group_of(id: &str) -> String {
  id.split_once('-')
    .and_then(|(_, rest)| rest.split_once('.'))
    .map(|(group, _)| group.to_string())
    .unwrap_or_else(|| id.to_string())
}

/// A test-backed AC is satisfied exactly when it has at least one covering AT
/// and EVERY covering AT is green.
///
/// **AND, not OR -- issue 0032, classified `corrected` rather than
/// `as-observed`.** v2 short-circuits (`bin/intent_acceptance:454`: `[
/// "$(at_status "$atline")" = "green" ] && return 0`), so a criterion
/// decomposed across several tests reported green as soon as its easiest arm
/// landed. **That is not a v2 behaviour to reproduce faithfully, because it was
/// not a decision**: hv's own filing says "the combining rule was chosen by an
/// early-return rather than by a decision", and you cannot faithfully reproduce
/// a decision nobody made. Reproducing it is what `parity.md` forbids in as many
/// words -- laundering a v2 defect into a v3 requirement.
///
/// **This doc comment is why the defect survived being read.** It said "a
/// test-backed AC is satisfied exactly when a covering AT is GREEN", which is an
/// accurate description of `.any` and a false statement of the requirement -- so
/// checking the code against its own documentation found agreement. It now states
/// the requirement, and the code is what has to match it.
///
/// **The non-empty guard is not optional.** `Iterator::all` on an empty iterator
/// is `true`, so `.all` alone would convert "no covering test at all" from
/// unsatisfied to satisfied -- a worse defect than the one being fixed, and
/// precisely the vacuous green issue 0015 is about.
///
/// `to-write` and `red` are not coverage, and neither is `n-a` -- that is the
/// non-test doc/eyeball status, and treating it as green is what let a contract
/// look closed while nothing had been run (issue 0015). **Under AND an `n-a` row
/// stops being inert and starts blocking, and that is the ALREADY-DOCUMENTED
/// consequence rather than a new decision**: `lint`'s L5 refuses a non-test AT
/// covering a test-backed criterion, on the stated grounds that "a non-test AT is
/// never green, so it can never satisfy it". The gate now behaves the way that
/// lint already says it will, so the two agree instead of the lint warning about a
/// consequence the gate did not have.
pub fn satisfied_by_tests(thread: &Thread, ac_id: &str) -> bool {
  let covering: Vec<&crate::model::AcceptanceTest> = thread
    .tests
    .iter()
    .filter(|t| t.covers.iter().any(|c| c == ac_id))
    .collect();
  !covering.is_empty() && covering.iter().all(|t| t.status == AtStatus::Green)
}

/// Resolve one AC's recorded state into the state the gate acts on.
///
/// Scope first: a descoped or withdrawn AC is not asked whether it is
/// satisfied, because the question no longer applies.
///
/// **`Computed` is the only variant that needs work here, and that is the
/// point.** Before the collapse this function had to decide, for every
/// criterion, whether to believe a stored `satisfied` flag or the ATs -- and
/// getting that precedence wrong was a live hazard, since a test-backed AC
/// could carry `satisfied: true` with every covering AT red. Now the recorded
/// state says which question it is: an authored criterion has already answered
/// it, and a test-backed one has no answer to be wrong.
pub fn resolve(thread: &Thread, criterion: &Criterion) -> Resolved {
  match &criterion.state {
    AcState::Descoped { .. } => Resolved::Descoped,
    AcState::Withdrawn { .. } => Resolved::Withdrawn,
    // **ABOVE THE KIND GUARD, AND THE ORDER IS LOAD-BEARING RATHER THAN
    // STYLISTIC.** The guard below resolves a test-backed criterion from its
    // ATs and ignores the recorded state, so a `Fiat` arm placed after it would
    // be unreachable for exactly the rows most likely to carry one -- an
    // over-cooked test-backed criterion is what a fiat close exists to escape.
    // The failure would have been silent: the arm compiles, every existing test
    // passes, and the only symptom is a fiat close that does nothing.
    //
    // It belongs here on the merits too. Same sentence as its two neighbours
    // above: a fiat-closed criterion is not asked whether it is satisfied,
    // because the question no longer applies.
    AcState::Fiat(..) => Resolved::Fiat,
    // **In scope, and the KIND decides which question is being asked -- not the
    // recorded state.** Matching on the state alone would have been the
    // natural way to write this and it would have reintroduced the exact
    // defect `a_stored_satisfied_flag_cannot_satisfy_a_test_backed_ac` exists
    // to prevent: canon can be hand-authored, so a test-backed criterion CAN
    // arrive carrying `satisfied`, and the gate must not believe it. `doctor`
    // reports that row as inconsistent; this refuses to act on it.
    _ if criterion.kind == AcKind::Test => {
      if satisfied_by_tests(thread, &criterion.id) {
        Resolved::Satisfied
      } else {
        Resolved::Unsatisfied
      }
    }
    AcState::Satisfied { .. } => Resolved::Satisfied,
    // `Computed` recorded on a non-test criterion is the mirror inconsistency,
    // reported by `doctor` and treated here as the honest answer: nothing
    // computes satisfaction for an authored criterion, so it has none.
    AcState::Unsatisfied { .. } | AcState::Computed => Resolved::Unsatisfied,
  }
}

/// Whether an AT's cited test file resolves.
///
/// The gate needs this and it is not a model question -- "does this path exist"
/// is a filesystem fact. Injected rather than reached for, so the verdict logic
/// stays a pure function of its inputs and the pure cases stay testable without
/// a temp directory.
///
/// v2 enforces it as lint rule L2 ("the cited test file does not exist"),
/// checked BEFORE the AC loop, so a broken reference blocks regardless of how
/// satisfied the contract looks. Found by running v2's own binary against an
/// equivalent estate rather than by reading its source -- the fixture tests
/// agreed with each other perfectly while missing it.
pub trait References {
  /// v2's L2: the cited test file exists.
  fn resolves(&self, path: &str) -> bool;
  /// v2's L3: the cited file carries the AT's own id, literally.
  ///
  /// This is the coverage mechanism, not a naming convention. The contract
  /// says to name a test by putting the AT's id INSIDE it, precisely because
  /// that is checkable from BOTH ends and survives rewording, where a cited
  /// test NAME survives neither. Without it, a row can cite a real file that
  /// tests something else entirely and the gate cannot tell.
  fn carries_id(&self, path: &str, at_id: &str) -> bool;
}

/// Resolve references against a real repository root.
pub struct RepoFiles<'a>(pub &'a std::path::Path);

impl References for RepoFiles<'_> {
  fn resolves(&self, path: &str) -> bool {
    self.0.join(path).exists()
  }

  fn carries_id(&self, path: &str, at_id: &str) -> bool {
    std::fs::read_to_string(self.0.join(path)).is_ok_and(|text| text.contains(at_id))
  }
}

/// Every reference resolves -- for tests about verdict arithmetic, where the
/// filesystem is not the thing under test.
pub struct AllResolve;

impl References for AllResolve {
  fn resolves(&self, _path: &str) -> bool {
    true
  }

  fn carries_id(&self, _path: &str, _at_id: &str) -> bool {
    true
  }
}

/// The gate's verdict. Every variant carries what was evaluated, because a
/// gate that cannot say what it checked cannot be trusted when it says nothing
/// is wrong (issue 0004) -- silence used to be v2's success signal, and that is
/// precisely what hid the vacuous passes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
  Pass {
    detail: Detail,
  },
  Exempt {
    detail: Detail,
  },
  Blocked {
    detail: Detail,
    /// The ids the gate is waiting on. **Held BESIDE the detail rather than
    /// inside it, because two surfaces want the same arithmetic and only one
    /// wants the enumeration** -- the gate names what blocks it (the operator
    /// hit a wall and needs the actionable set without running a second
    /// command), and `ac status` reports the count and leaves the rows to `ac
    /// list`. Baked into `detail`, the summary could only be recovered by
    /// splitting the string back apart in a renderer, which is a parser of our
    /// own format that fails silently the day the format moves.
    ///
    /// Empty on the four degenerate blocks (absent WP, empty contract, wholly
    /// off-scope, and the exempt escape's sibling), where the refusal IS the
    /// whole story and there is no set to name.
    unsatisfied: Vec<String>,
  },
}

/// What a verdict says beyond its word -- **and therefore where the word goes
/// when the verdict is reported.**
///
/// The two are not a style choice. `ac status` puts the verdict LAST, which is
/// v2's line and reads correctly after a count (`46/114 satisfied -- BLOCKED`)
/// and badly after a sentence (`... declare 'acceptance: exempt'. -- BLOCKED`).
/// **A line that reads badly is one somebody later "improves"**, and the
/// improvement would land on 43 of Intent's own 56 threads.
///
/// **The distinction is a property of the ARM that produced it, never a test on
/// the string.** Sniffing for a trailing full stop would be a parser of our own
/// output, which is the same failure as recovering a summary by splitting the
/// enumeration back off it: correct until the format moves, then silently
/// wrong. vc ruled the placement; this is what makes the ruling structural.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Detail {
  /// `N/M satisfied[, K descoped][, W withdrawn]` -- a phrase, so the verdict
  /// can trail it.
  Tally(String),
  /// A sentence naming what is wrong and what to do about it. The verdict
  /// leads, because nothing should follow a full stop.
  Diagnosis(String),
}

impl Detail {
  pub fn text(&self) -> &str {
    match self {
      Self::Tally(t) | Self::Diagnosis(t) => t,
    }
  }
}

impl Verdict {
  /// A block with nothing to enumerate. All four are diagnoses: they say what
  /// is wrong rather than how far along it is.
  fn blocked(detail: impl Into<String>) -> Self {
    Self::Blocked {
      detail: Detail::Diagnosis(detail.into()),
      unsatisfied: Vec::new(),
    }
  }

  /// The machine-facing line, in v2's exact shape: `gate: <scope> <VERDICT> --
  /// <detail>`. Read by `st done` / `wp done` via the exit code, and by humans
  /// via the text; both carry over unchanged (D17).
  pub fn line(&self, scope_label: &str) -> String {
    let mut line = format!(
      "gate: {scope_label} {} -- {}",
      self.word(),
      self.detail().text()
    );
    if let Self::Blocked { unsatisfied, .. } = self
      && !unsatisfied.is_empty()
    {
      line.push_str(&format!("; unsatisfied: {}", unsatisfied.join(" ")));
    }
    line
  }

  /// `intent ac status`, in v2's shape (`bin/intent_acceptance:937`): the
  /// arithmetic and the verdict, and **no scope label** -- v2 prints none, and
  /// the caller just typed the target.
  ///
  /// **This is a different SURFACE, not a different computation.** `status` is
  /// the read and `gate` is the decision; in v2 they were two walks of one
  /// document at two strictnesses, and here the contract is model state, so
  /// there is one answer and two ways of saying it. Reusing `line()` reported a
  /// gate's verdict under a gate's prefix beside `status`'s exit code -- a line
  /// reading `gate: ... BLOCKED` next to exit 0 is what a consumer misreads,
  /// and the pre-commit gate is a consumer.
  ///
  /// DEVIATION from v2, and it is the one place the shared computation shows:
  /// v2's `status` runs `at_lint_report` for its stderr warnings and drops the
  /// result on the floor, so an AT contract finding BLOCKS v2's gate while v2's
  /// `status` still says PASS. v3 answers both from one verdict. **A status
  /// that says PASS where the close gate refuses is telling the operator they
  /// can close when they cannot**, which is the reading the two-walk split
  /// produced by accident rather than by design.
  ///
  /// **The verdict goes where the detail lets it go** (vc, 2026-08-17): after a
  /// tally, which is v2's line; before a diagnosis, so nothing trails a full
  /// stop. See [`Detail`] -- and note the arithmetic case is byte-identical to
  /// v2 on every thread that carries a contract, which is what the placement
  /// rule protects.
  pub fn status_line(&self) -> String {
    match self.detail() {
      Detail::Tally(t) => format!("ac: {t} -- {}", self.word()),
      Detail::Diagnosis(d) => format!("ac: {} -- {d}", self.word()),
    }
  }

  fn word(&self) -> &'static str {
    match self {
      Self::Pass { .. } => "PASS",
      Self::Exempt { .. } => "EXEMPT",
      Self::Blocked { .. } => "BLOCKED",
    }
  }

  fn detail(&self) -> &Detail {
    match self {
      Self::Pass { detail } | Self::Exempt { detail } | Self::Blocked { detail, .. } => detail,
    }
  }

  pub fn is_pass(&self) -> bool {
    matches!(self, Self::Pass { .. } | Self::Exempt { .. })
  }

  /// v2's exit code: 0 for PASS/EXEMPT, 1 for BLOCKED.
  pub fn exit_code(&self) -> i32 {
    if self.is_pass() { 0 } else { 1 }
  }
}

/// Run the close gate over a thread at a scope.
///
/// Reproduces `cmd_ac_gate` (`bin/intent_acceptance:973`) verdict for verdict.
/// Three of v2's BLOCKED paths are absent here and their absence is the point:
/// malformed AC/AT lines, AT-grammar findings, and a missing `acceptance.md`
/// cannot occur, because the contract is model state rather than a parsed
/// document. There is no row grammar left to violate.
pub fn gate(thread: &Thread, scope: Scope, refs: &dyn References) -> Verdict {
  if thread.acceptance.is_some() {
    return Verdict::Exempt {
      detail: Detail::Diagnosis("the thread declares 'acceptance: exempt'".to_string()),
    };
  }

  // A WP scope must name a WP that exists. A typo'd `/99` matches no ACs for
  // the same arithmetic reason a genuinely AC-free WP does, and only a lookup
  // tells the two apart -- v2 learned this as issue 0004.
  if let Scope::WorkPackage(seq) = scope
    && !thread.wps.iter().any(|w| w.seq == seq)
  {
    return Verdict::blocked(format!(
      "WP-{seq:02} does not exist in {} (nothing to evaluate)",
      thread.id
    ));
  }

  // **A CANCELLED WORK PACKAGE IS AN ANNOUNCED EXEMPTION, WHICH IS WHY IT IS
  // NOT THE `active == 0` PATH BELOW.** ST0048's rule is that an exemption is
  // announced and never inferred from emptiness, and before `WpStatus::Cancelled`
  // existed there was nowhere at WP scope to announce it -- so a unit whose scope
  // was removed emptied its contract one descope at a time, hit that arm, and
  // could never be closed at all. The only escape named was `acceptance: exempt`,
  // which is THREAD-scoped: closing one unit with it discarded the standing of
  // every AC in the thread (measured on a live consumer: 37 of them).
  //
  // The status field is that announcement, as DATA rather than as prose a
  // counter cannot read. This arm satisfies ST0048 rather than bypassing it.
  if let Scope::WorkPackage(seq) = scope
    && thread
      .wps
      .iter()
      .any(|w| w.seq == seq && w.status == crate::model::WpStatus::Cancelled)
  {
    return Verdict::Exempt {
      detail: Detail::Diagnosis(format!(
        "WP-{seq:02} is cancelled: its scope was removed, so it has no live contract to verify"
      )),
    };
  }

  let thread_total = thread.criteria.len();
  if thread_total == 0 {
    return Verdict::blocked(
      "the thread has zero acceptance criteria (empty contract). Define ACs, or declare 'acceptance: exempt'.",
    );
  }

  let wanted = scope.group();

  // The AT contract rules, checked before the AC loop exactly as v2 checks
  // them. v2's gate calls `at_lint_report` and blocks on ANY finding
  // (`bin/intent_acceptance:1008`), so L1-L5 are gate rules rather than merely
  // lint rules -- `at lint` is a VALIDATOR the gate calls, not a read surface,
  // and filing it under the wrong noun is how L4 and L5 nearly shipped missing.
  //
  // L1 (row grammar) is gone by construction: a row is model fields, so there
  // is no grammar left to violate. The other four survive.
  //
  // DEVIATION (register row, ruled `corrected`): verdict and exit code match
  // v2, the remedy text does not. v2 sends the operator to `at lint --fix`,
  // which in v3 has nothing to fix -- pointing at a command that cannot help
  // is a v2 defect, not a v3 design consequence.
  //
  // **`over N row(s)` is RESTORED, not added** (vc ruled, 2026-08-17). v2's line
  // carries the denominator -- `$AT_LINT_FAILS AT contract finding(s) over
  // $AT_LINT_ROWS row(s)` at `bin/intent_acceptance:1009` -- and v3 had dropped
  // it, so this is a regression against D17 rather than a deviation D17 has to
  // license. Three findings out of three rows and three out of a hundred and
  // fourteen are different situations, and the number was the only thing saying
  // which.
  let report = contract_report(thread, wanted.as_deref(), refs);
  if !report.findings.is_empty() {
    return Verdict::blocked(format!(
      "{} acceptance test contract finding(s) over {} row(s): {}",
      report.findings.len(),
      report.rows,
      report.findings.join("; ")
    ));
  }

  let in_scope: Vec<&Criterion> = thread
    .criteria
    .iter()
    .filter(|c| match &wanted {
      None => true,
      Some(group) => &group_of(&c.id) == group,
    })
    .collect();

  if in_scope.is_empty() {
    // The WP-lenient rollup (ST0044): a WP with no ACs of its own rolls up to
    // the thread's contract. Announced, never inferred -- same rule as EXEMPT.
    return Verdict::Pass {
      detail: Detail::Diagnosis(format!(
        "no ACs in scope; rolls up to the {} contract ({thread_total} AC(s))",
        thread.id
      )),
    };
  }

  let total = in_scope.len();
  let mut satisfied = 0;
  let mut descoped = 0;
  let mut withdrawn = 0;
  let mut fiat = 0;
  let mut unsatisfied: Vec<&str> = Vec::new();
  for c in &in_scope {
    match resolve(thread, c) {
      Resolved::Descoped => descoped += 1,
      Resolved::Withdrawn => withdrawn += 1,
      Resolved::Satisfied => satisfied += 1,
      Resolved::Fiat => fiat += 1,
      Resolved::Unsatisfied => unsatisfied.push(&c.id),
    }
  }
  let active = total - descoped - withdrawn;

  if active == 0 {
    // Routed to the declared escape rather than passing on an empty set: a
    // contract emptied one descope at a time is still emptiness, and ST0048's
    // rule is that an exemption is announced, never inferred from it.
    return Verdict::blocked(format!(
      "all {total} in-scope AC(s) are descoped or withdrawn; nothing is left to verify. If this unit is deliberately contract-free, declare 'acceptance: exempt'."
    ));
  }

  // The only two arms that report a COUNT rather than a diagnosis, and so the
  // only two where `ac status` trails the verdict after the detail. See
  // [`Detail`].
  let suffix = offscope_suffix(descoped, withdrawn);
  // **COUNTED AND DISTINGUISHED IN ONE LINE**, which is what the ruling asks
  // for: a fiat close unblocks the gate, and the line that counts it says so in
  // the same breath. `fiat` is NOT folded into `satisfied` for the reason
  // `offscope_suffix` gives about its own two states -- a thread that fiat-closed
  // half its contract has to look like one.
  let fiat_suffix = if fiat > 0 {
    format!(", {fiat} fiat-closed")
  } else {
    String::new()
  };
  let tally = Detail::Tally(format!(
    "{satisfied}/{active} satisfied{fiat_suffix}{suffix}"
  ));
  if satisfied + fiat == active {
    Verdict::Pass { detail: tally }
  } else {
    Verdict::Blocked {
      detail: tally,
      unsatisfied: unsatisfied.iter().map(|id| (*id).to_string()).collect(),
    }
  }
}

/// v2's L2-L5, each with its own diagnosis.
///
/// **Every finding names WHICH rule fired, and no two read alike.** That is not
/// presentation: "the file is missing", "the file is the wrong one", "this
/// covers an AC that does not exist" and "this can never satisfy what it
/// covers" have four different fixes. Reporting them identically is the same
/// same-text-for-different-causes collapse AC-04.4 forbids one layer up -- and
/// the mutation battery proved the point, because L2 and L3 were mutually
/// covering until the test started asserting which one fired.
/// Public because `intent at lint` reports exactly what the gate enforces.
/// Two rule sets would be the drift where the lint says clean and the gate
/// refuses -- which is the shape that made v2's `at lint` untrustworthy.
///
/// **Returns the denominator with the findings, and they are counted in the
/// same walk on purpose.** `at lint`'s clean report is `N AT row(s) conform`;
/// counting the rows anywhere else lets the two disagree about what was
/// examined the moment a scope filter appears on one side and not the other --
/// which is issue 0024, recorded at `bin/intent_acceptance:629` as the reason
/// v2 moved its own row count inside the filter.
pub fn contract_report(
  thread: &Thread,
  wanted: Option<&str>,
  refs: &dyn References,
) -> ContractReport {
  // L3 is exempt on a closed thread: retrofitting id labels into a completed
  // thread's tests is archaeology, and v2 says so at `at_row_findings`.
  let completed = thread.status == ThreadStatus::Completed;
  let mut out = Vec::new();
  let mut rows = 0;

  for t in thread.tests.iter().filter(|t| match wanted {
    None => true,
    Some(group) => group_of(&t.id) == group,
  }) {
    rows += 1;
    // L2/L3 apply to a REAL test row that claims to have been run. `to-write`
    // is exempt because a missing file is the CORRECT state for a test not yet
    // written -- a naive existence check reds five correct rows, which is why
    // v2 restricts the arm to green|red.
    if t.kind == AtKind::Test
      && matches!(t.status, AtStatus::Green | AtStatus::Red)
      && let Some(path) = t.file.as_deref()
    {
      if !refs.resolves(path) {
        out.push(format!("{} cites a file that does not exist: {path}", t.id));
      } else if !completed && !refs.carries_id(path, &t.id) {
        out.push(format!("{path} does not carry the literal id {}", t.id));
      }
    }

    for covered in &t.covers {
      match thread.criteria.iter().find(|c| &c.id == covered) {
        // L4: the reverse direction. Satisfaction is computed forwards -- for
        // each AC, find covering ATs -- so a `covers` id that matches nothing
        // is silently ignored by that walk and the AT looks like coverage it
        // is not.
        None => out.push(format!(
          "{} covers {covered}, which is not a criterion in this contract",
          t.id
        )),
        // L5: the trap the non-test arm would otherwise bless. A non-test AT
        // is `n-a` by definition and `n-a` is never green, so a test-backed AC
        // covered by one can NEVER be satisfied -- an unclosable contract whose
        // only symptom is a gate that will not move. Reproducing the symptom
        // without the diagnosis would leave the operator staring at a
        // permanently unsatisfied AC with nothing to act on.
        //
        // **"covered ONLY by one" until issue 0032 landed, and the word mattered.**
        // Under the OR gate such a row was inert beside a green sibling, so the
        // trap needed the row to be the criterion's only coverage. Under AND it
        // blocks unconditionally -- which is what this lint's own sentence always
        // claimed, so the gate has stopped disagreeing with it.
        Some(c) if t.kind == AtKind::NonTest && c.kind != AcKind::NonTest => out.push(format!(
          "{} is a non-test AT covering {covered}, which is test-backed -- a non-test AT is never green, so it can never satisfy it. Mark {covered} non-test and satisfy it by evidence, or cover it with a real test",
          t.id
        )),
        Some(_) => {}
      }
    }
  }
  ContractReport {
    findings: out,
    rows,
  }
}

/// What the AT contract check examined and what it found.
///
/// **The row count is not decoration on the clean path, it is the clean path's
/// only content.** `at lint` on a conforming thread has nothing to enumerate,
/// so without a denominator its success and its non-execution are the same
/// zero bytes -- and a lint whose green is indistinguishable from not having
/// run is the vacuous-pass shape this estate has now paid for at three levels.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractReport {
  /// One entry per L2-L5 violation, each naming which rule fired.
  pub findings: Vec<String>,
  /// AT rows examined, counted after the scope filter.
  pub rows: usize,
}

/// Descoped and withdrawn counts are reported SEPARATELY, never folded into
/// the satisfied count, so a thread that descoped half its contract looks like
/// one.
fn offscope_suffix(descoped: usize, withdrawn: usize) -> String {
  let mut out = String::new();
  if descoped > 0 {
    out.push_str(&format!(", {descoped} descoped"));
  }
  if withdrawn > 0 {
    out.push_str(&format!(", {withdrawn} withdrawn"));
  }
  out
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::model::{AcceptanceTest, AtKind, THREAD_SCHEMA, ThreadStatus, WorkPackage};

  fn thread(criteria: Vec<Criterion>, tests: Vec<AcceptanceTest>) -> Thread {
    Thread {
      // A thread created here has no files beside it yet; the walk that finds
      // them runs at ingest, not at creation.
      attachments: Vec::new(),
      body: String::new(),
      preamble: String::new(),
      schema: THREAD_SCHEMA.to_string(),
      id: "ST0056".to_string(),
      title: "t".to_string(),
      slug: None,
      status: ThreadStatus::Wip,
      status_reason: None,
      fiat: None,
      created: "2026-08-14".to_string(),
      completed: None,
      acceptance: None,
      objective: String::new(),
      context: String::new(),
      related: Vec::new(),
      wps: vec![WorkPackage {
        preamble: String::new(),
        seq: 3,
        title: "w".to_string(),
        scope: Some(crate::model::TShirt::L),
        scope_legacy: None,
        status: crate::model::WpStatus::Wip,
        status_reason: None,
        fiat: None,
        objective: String::new(),
        body: String::new(),
      }],
      criteria,
      tests,
    }
  }

  fn ac(id: &str, kind: AcKind, state: AcState) -> Criterion {
    Criterion {
      id: id.to_string(),
      text: "x".to_string(),
      kind,
      state,
    }
  }

  /// A test-backed criterion in scope, which under the collapse records
  /// `Computed` and nothing else.
  fn computed(id: &str) -> Criterion {
    ac(id, AcKind::Test, AcState::Computed)
  }

  fn at(id: &str, covers: &str, status: AtStatus) -> AcceptanceTest {
    AcceptanceTest {
      fiat: None,
      id: id.to_string(),
      kind: AtKind::Test,
      file: Some("a/b.rs".to_string()),
      prose: None,
      covers: vec![covers.to_string()],
      status,
      note: None,
      legacy: None,
    }
  }

  #[test]
  fn a_green_at_satisfies_its_ac_and_a_red_one_does_not() {
    let t = thread(
      vec![computed("AC-03.1")],
      vec![at("AT-03.1", "AC-03.1", AtStatus::Green)],
    );
    assert_eq!(resolve(&t, &t.criteria[0]), Resolved::Satisfied);

    let t = thread(
      vec![computed("AC-03.1")],
      vec![at("AT-03.1", "AC-03.1", AtStatus::Red)],
    );
    assert_eq!(resolve(&t, &t.criteria[0]), Resolved::Unsatisfied);
  }

  /// `n-a` is the non-test doc status. Counting it as coverage is issue 0015.
  #[test]
  fn an_n_a_at_never_satisfies_a_test_backed_ac() {
    let t = thread(
      vec![computed("AC-03.1")],
      vec![at("AT-03.1", "AC-03.1", AtStatus::Na)],
    );
    assert_eq!(resolve(&t, &t.criteria[0]), Resolved::Unsatisfied);
  }

  #[test]
  fn a_stored_satisfied_flag_cannot_satisfy_a_test_backed_ac() {
    // The flag is non-test-only. If a test-backed AC carried one, honouring it
    // would be exactly the double truth data-model.md forbids.
    let t = thread(
      vec![ac(
        "AC-03.1",
        AcKind::Test,
        AcState::Satisfied {
          evidence: "hand-authored, which canon can be".to_string(),
        },
      )],
      vec![at("AT-03.1", "AC-03.1", AtStatus::Red)],
    );
    assert_eq!(
      resolve(&t, &t.criteria[0]),
      Resolved::Unsatisfied,
      "only a green AT satisfies a test-backed AC"
    );
  }

  #[test]
  fn the_gate_passes_a_fully_satisfied_scope() {
    let t = thread(
      vec![computed("AC-03.1")],
      vec![at("AT-03.1", "AC-03.1", AtStatus::Green)],
    );
    assert_eq!(
      gate(&t, Scope::WorkPackage(3), &AllResolve).line("ST0056/03"),
      "gate: ST0056/03 PASS -- 1/1 satisfied"
    );
  }

  #[test]
  fn the_gate_reports_offscope_counts_separately() {
    let t = thread(
      vec![
        computed("AC-03.1"),
        ac(
          "AC-03.2",
          AcKind::Test,
          AcState::Descoped {
            to: "ST0057".to_string(),
            by: None,
            reason: None,
          },
        ),
        ac(
          "AC-03.3",
          AcKind::Test,
          AcState::Withdrawn {
            reason: "r".to_string(),
            by: None,
          },
        ),
      ],
      vec![at("AT-03.1", "AC-03.1", AtStatus::Green)],
    );
    assert_eq!(
      gate(&t, Scope::WorkPackage(3), &AllResolve).line("ST0056/03"),
      "gate: ST0056/03 PASS -- 1/1 satisfied, 1 descoped, 1 withdrawn"
    );
  }

  #[test]
  fn an_all_offscope_contract_is_blocked_not_passed() {
    let t = thread(
      vec![ac(
        "AC-03.1",
        AcKind::Test,
        AcState::Withdrawn {
          reason: "r".to_string(),
          by: None,
        },
      )],
      vec![],
    );
    let verdict = gate(&t, Scope::WorkPackage(3), &AllResolve);
    assert!(!verdict.is_pass());
    assert!(
      verdict
        .line("ST0056/03")
        .contains("nothing is left to verify")
    );
  }

  #[test]
  fn a_nonexistent_wp_is_blocked_rather_than_rolling_up() {
    let t = thread(
      vec![computed("AC-03.1")],
      vec![at("AT-03.1", "AC-03.1", AtStatus::Green)],
    );
    let verdict = gate(&t, Scope::WorkPackage(99), &AllResolve);
    assert!(!verdict.is_pass(), "a typo must not inherit the rollup");
    assert!(verdict.line("ST0056/99").contains("does not exist"));
  }

  #[test]
  fn an_ac_free_wp_rolls_up_and_says_so() {
    let mut t = thread(
      vec![computed("AC-00.1")],
      vec![at("AT-00.1", "AC-00.1", AtStatus::Green)],
    );
    t.wps.push(WorkPackage {
      preamble: String::new(),
      seq: 7,
      title: "later".to_string(),
      scope: Some(crate::model::TShirt::S),
      scope_legacy: None,
      status: crate::model::WpStatus::NotStarted,
      status_reason: None,
      fiat: None,
      objective: String::new(),
      body: String::new(),
    });
    assert_eq!(
      gate(&t, Scope::WorkPackage(7), &AllResolve).line("ST0056/07"),
      "gate: ST0056/07 PASS -- no ACs in scope; rolls up to the ST0056 contract (1 AC(s))"
    );
  }

  #[test]
  fn an_empty_contract_is_blocked() {
    let t = thread(vec![], vec![]);
    assert!(
      gate(&t, Scope::Thread, &AllResolve)
        .line("ST0056")
        .contains("zero acceptance criteria")
    );
  }

  #[test]
  fn exempt_is_announced_never_inferred() {
    let mut t = thread(vec![], vec![]);
    t.acceptance = Some(crate::model::AcceptanceMode::Exempt);
    let verdict = gate(&t, Scope::Thread, &AllResolve);
    assert!(verdict.is_pass());
    assert!(verdict.line("ST0056").contains("EXEMPT"));
  }

  #[test]
  fn the_blocked_line_names_every_unsatisfied_ac() {
    let t = thread(vec![computed("AC-03.1"), computed("AC-03.2")], vec![]);
    assert_eq!(
      gate(&t, Scope::WorkPackage(3), &AllResolve).line("ST0056/03"),
      "gate: ST0056/03 BLOCKED -- 0/2 satisfied; unsatisfied: AC-03.1 AC-03.2"
    );
  }
}
