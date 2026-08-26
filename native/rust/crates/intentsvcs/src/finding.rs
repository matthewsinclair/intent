//! Findings -- the one shape in which intentsvcs refuses.
//!
//! Every refusal in v3 names the artefact and the reason, in one grammar, so a
//! caller never has to tell "the tool said no" from "the tool said nothing".
//! The line format is migration.md's residue line, generalised from migration
//! to every refusing path:
//!
//! ```text
//! residue: <file>:<line> -- <class> -- <detail>
//! ```
//!
//! The class vocabulary is deliberately closed. A new refusal reason is a new
//! variant here, reviewed once, rather than a new string spelled slightly
//! differently at each site -- which is how v2 ended up with the same failure
//! reported five ways (the 0023 voice sweep).

use std::fmt;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Why an artefact was refused. Closed vocabulary.
///
/// **This used to say "migration.md's residue classes plus the two WP-03 adds",
/// and by the time anyone read it the enum held seventeen.** A comment
/// describing this type's relationship to ANOTHER document is the one claim no
/// compiler and no test is looking at: the act that adds a variant is not the
/// act that revisits a sentence about a file somewhere else, so it goes stale
/// on the first addition and reads as current forever. Third instance of that
/// shape in this thread, and it is recorded rather than merely corrected --
/// replacing one count with another would just restart the clock.
///
/// So the description is per-variant, where the variant is, and nothing here
/// counts. What IS enforced lives in [`FindingClass::meta`]: one exhaustive
/// match supplies rank, wire spelling and remedy, so a new variant does not
/// compile until all three are decided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum FindingClass {
  /// The project's canon is not in a form this binary can read -- v2 canon
  /// not yet migrated, or a config declaring a pre-v3 version. FIRST, because
  /// every other finding on such a project is downstream of this one and
  /// would send the operator after the wrong thing.
  Unmigrated,
  /// Not parseable as JSON at all.
  MalformedJson,
  /// Parses as JSON; violates the schema. Includes the D05 unknown-field
  /// refusal -- an unknown field is named, never dropped.
  SchemaInvalid,
  /// Git conflict markers present in an artefact. v2 grepped straight through
  /// these; v3 refuses (design.md).
  ConflictMarkers,
  /// A file in a modelled location the parser cannot classify.
  UnknownFileShape,
  /// Two artefacts claiming one natural id (the 0011 class).
  DuplicateId,
  /// An AC/AT/claims/index row the frozen legacy grammar cannot read
  /// (migration.md).
  UnparseableRow,
  /// A status value outside the v2 vocabulary -- which is what
  /// `canonical_status` ACCEPTS, not the set of values v2 prints.
  UnknownStatus,
  /// A T-shirt scope outside the enum. v2 reads `scope:` as free text, so this
  /// is the absence of a vocabulary rather than a violation of one.
  UnknownScope,
  /// An AT file reference or coverage link that does not resolve.
  BrokenReference,
  /// A field the v2 line DOES carry and this grammar never reads.
  ///
  /// **The mirror of [`FindingClass::FieldNotRecorded`], and the pair only
  /// makes sense together**: that one is a field the estate never wrote, this
  /// one is a field the estate wrote and the reader walked past. Both leave the
  /// model without a value; only this one leaves it without a value that was
  /// sitting on the line.
  ///
  /// **IT IS A CLASS RATHER THAN A LIST BECAUSE A LIST CANNOT KNOW WHAT COMES
  /// NEXT.** Lamplight writes `-- withdrawn:` and `-- descoped-to:` with their
  /// own `by:` and `on:`; the migrator reads exactly `evidence:` and
  /// `satisfied:` on an AC row, so 19 disposed rows arrived with their
  /// disposition as prose and nothing said so. Teaching the reader those two
  /// keys closes those 19 and leaves the next convention exactly as silent --
  /// which is the argument `thread_dirs` already lost once, where an allowlist
  /// of three bucket names closed the instance and left the class open.
  ///
  /// **The migrator says it does not know. It never decides.** Naming an
  /// unread key is not a guess about what the key meant, and that restraint is
  /// the whole reason this can be reported for keys nobody has invented yet.
  UnreadField,
  /// A field the v2 estate never recorded, because the artefact predates the
  /// convention that introduced it. **Not a defect and not the reader's to
  /// fix** -- reported so the migration's counts reconcile, and kept apart from
  /// `UnknownStatus` / `UnknownScope`, which describe a value that IS there and
  /// is wrong.
  FieldNotRecorded,
  /// A generated view on disk differs from what the model renders -- a
  /// hand-edit that would otherwise be silently overwritten, or silently
  /// believed.
  ViewSkew,
  /// An ATTACHMENT on disk differs from the bytes canon records for it.
  ///
  /// **Not [`FindingClass::ViewSkew`], and the difference is what the operator
  /// must do about it.** A view is re-derivable, so skew is repaired by
  /// re-rendering and the hand-edit is the thing being lost. An attachment is
  /// AUTHORED -- nothing can regenerate it -- so a divergence is two versions
  /// of a file that only a person can reconcile, and whichever side is
  /// overwritten loses work that has no other copy. Reporting them under one
  /// class would put "run the renderer" and "decide which of these you meant"
  /// behind the same word.
  AttachmentDrift,
  /// The canon parses and validates, but says two things that cannot both be
  /// true -- an acceptance test covering a criterion that does not exist, a
  /// completed thread with no completion date. The schema cannot catch these:
  /// every one of them is individually well-formed, and only the RELATIONSHIP
  /// is wrong. This is what `doctor`'s model half reports (AC-06.2).
  ModelInconsistent,
  /// A unit's recorded STATUS disagrees with what its GATE says.
  ///
  /// **hv ratified this arm on 2026-08-15** (`data-model.md:472`): *`wp done` is
  /// refused on a BLOCKED gate AND `doctor` reports any unit whose status
  /// disagrees with its gate -- both, as recommended.* The refusal landed and
  /// the report did not, so for five days nothing watched the join.
  ///
  /// **A SEPARATE CLASS FROM `ModelInconsistent`, ON THAT CLASS'S OWN
  /// ARGUMENT.** This is a relationship defect and would fit there by shape --
  /// but its remedy is a verb the operator runs (`wp start` / `wp done`) or a
  /// contract to re-examine, where `ModelInconsistent`'s is *correct the
  /// artefact*. `AttachmentDrift` was split from it for exactly this reason:
  /// putting two different remedies behind one word tells an operator nothing
  /// to do.
  ///
  /// **THE DANGEROUS DIRECTION IS `Done` OVER A BLOCKED GATE, AND IT ARRIVES
  /// WITHOUT ANYONE DOING ANYTHING WRONG.** `wp done` consults the gate at the
  /// moment of closing, and nothing re-checks afterwards -- so a WP closed
  /// legitimately becomes a false green the instant its contract GROWS.
  /// Measured 2026-08-20 by vc across all 26 work packages: four disagreed, and
  /// ST0056/04 was `Done` at 5 of 6 because AC-04.6 was minted after the close.
  /// **The `Done` was true when it was set and false afterwards.**
  StatusGateDisagreement,
  /// The durable store has no recent restorable snapshot -- either none has
  /// ever succeeded, or the newest is older than the configured schedule.
  ///
  /// **This is the half of the backup rule that a failure report cannot
  /// cover.** A schedule that never fires produces no failure, so "surface the
  /// failure" leaves a user unable to tell a working backup from one that has
  /// silently never started. It is the two-sided construction: two recorded
  /// values compared to each other rather than an error waited for.
  BackupStale,
  /// This machine holds event history the repository does not carry.
  ///
  /// **Two artefacts disagreeing, and it took two narrowings to get here.** The
  /// first version REFUSED to open an estate with entities and no history, on
  /// the argument that under D34 every mutation writes an envelope. The suite
  /// refuted it in one run: a hand-authored `thread.json` is an entity that
  /// never came from a mutation, and that is exactly the shape WP-10's
  /// migration produces, so the refusal would have refused every migrated
  /// estate. The second version reported the same condition instead of
  /// refusing, and two doctor fixtures fired it immediately -- correctly, which
  /// was the problem: the per-thread mutation path does not rewrite the log
  /// extract, so a normally-used project is in that state routinely and the
  /// finding would have been permanent noise on the path it exists to protect.
  ///
  /// What survives is provable and cannot be noise: history that exists on this
  /// machine and would not survive a clone, reported to the person who still
  /// has it.
  EventLogAbsent,
  /// The project sets a config knob v3 has retired, to a value v3 will not
  /// honour.
  ///
  /// **`st_prefix` is the instance and the reason the class exists** (issue
  /// 0040, hv). Retiring a knob nobody uses is free; retiring it under someone
  /// who DOES use it, silently, is the change this thread exists to prevent --
  /// and here the consequence is total rather than cosmetic. v3 recognises a
  /// steel thread by `crate::model::is_thread_id`, so a project whose threads
  /// are named on any other prefix has NONE of them recognised: the migration
  /// would report a clean conversion of an empty estate. **That is the
  /// answers-confidently-from-partial-evidence bug with the evidence set to
  /// zero**, which is why it blocks rather than carries.
  RetiredSetting,
  /// A setting whose value is well-formed and which the DATA cannot honour --
  /// today, only `todo.window_hours` finer than the resolution of `completed`.
  ///
  /// **Distinct from [`RetiredSetting`](Self::RetiredSetting), which is a key
  /// v3 no longer reads at all.** This key is read, and the value is refused:
  /// the schema cannot catch it because the value is a perfectly good `u32`,
  /// and only its relationship to the precision of another field is wrong --
  /// the same reason `ModelInconsistent` exists one level up.
  ///
  /// **It is here so the operator does not have to run the one affected
  /// command to find out.** The refusal itself lands on `intent todo`; without
  /// this, a config edited once and read months later announces itself as a
  /// command that suddenly stopped working.
  UnhonourableSetting,
  /// A hygiene note, not a fault: the artefact is well-formed and nothing is
  /// blocked by it. Printed under `advisory:` and NOT counted toward the
  /// verdict -- hv, 2026-08-26, on Baize printing 66 of these at rc 1 under the
  /// `model-inconsistent` remedy, which made "pristine doctor" unreachable on
  /// any live estate whose AT rows still cite tests in the v2 grammar.
  Advisory,
}

impl FindingClass {
  /// Rank, wire spelling and REMEDY, from ONE exhaustive match.
  ///
  /// Exhaustive because the compiler must refuse a new variant that forgets
  /// any of them -- an omission here is a class that reports under the wrong
  /// name, sorts arbitrarily, or tells an operator nothing to do, and none of
  /// the three announces itself.
  ///
  /// **The remedy is carried because `doctor --fix` was WITHDRAWN** (hv,
  /// 2026-08-15), and the ruling generalises past that flag: a diagnostic that
  /// NAMES the exact remedy is strictly better than one that performs it. The
  /// operator sees what will happen, decides whether it is what they meant,
  /// and keeps the blast radius in their own hands. A repair verb claims the
  /// tool understands the fault well enough to act unattended; a named remedy
  /// claims only that it understands it well enough to describe it -- and the
  /// second is the claim `doctor` can actually make.
  ///
  /// Two rules bind every string below. **No remedy proposes an operation
  /// whose blast radius exceeds the fault it repairs** (vc, 2026-08-15), which
  /// is why none of them reaches for `sync --to-store`: it replaces the whole
  /// store, and `event_log` is durable truth no file can reconstruct. And **no
  /// remedy names deleting the store** (D36) -- it is the source of truth, not
  /// a cache.
  fn meta(self) -> (u8, &'static str, &'static str) {
    match self {
      // The detail already carries `Migration::remedy()`, which names the
      // version and the command. This says the part that is true of the class:
      // nothing else is worth reading until it is done.
      Self::Unmigrated => (
        0,
        "unmigrated",
        "migrate the project first -- every other finding on it is downstream of this one",
      ),
      // **The four migration classes. Their remedy names the FIXING
      // ENVIRONMENT, which is the last v2 release rather than this binary**
      // (migration.md's two-hop): v3 refuses what it cannot convert without
      // loss, and the tool that can repair a v2 artefact is v2. A remedy
      // sending someone to fix v2 markdown with a v3 command would be a
      // remedy that cannot be acted on.
      Self::UnparseableRow => (
        1,
        "unparseable-row",
        "repair the row under v2 tooling (`intent at lint --fix` where it applies, by hand where it does not), then re-run the migration",
      ),
      Self::UnknownStatus => (
        1,
        "unknown-status",
        "set the status to one v2 accepts, using the v2 CLI, then re-run the migration",
      ),
      // It says the model cannot hold it YET, because that is the true
      // reason and it is not the reader's problem to solve.
      // **"wait for a build whose model carries the value verbatim" is GONE,
      // because this is that build.** A remedy that outlives the state it
      // describes reads as current and sends the reader to do nothing; this one
      // was telling them to wait for a capability they already had, on the one
      // row in the corpus that needed it.
      Self::UnknownScope => (
        1,
        "unknown-scope",
        "nothing to do for a closed thread -- the value is carried verbatim and stays visible as legacy. On a LIVE thread, set the scope to a T-shirt size under v2 tooling; nothing here guesses which size was meant",
      ),
      Self::BrokenReference => (
        1,
        "broken-reference",
        "point the reference at something that exists, or remove it, under v2 tooling -- then re-run the migration",
      ),
      // **It says there is nothing to do, because there is nothing to do.** The
      // artefact predates the field; v2 was content with it and v3 carries it
      // as it is. A remedy that suggested authoring the value would be asking
      // someone to invent data about finished work.
      // **No "wait for a build that reads it", deliberately.** Some of these
      // keys become known in a later build and some never will, and a remedy
      // that promised a build would be wrong for the second kind while reading
      // as current for both. What is true of every one of them is that the v2
      // line still has the value.
      Self::UnreadField => (
        1,
        "unread-field",
        "nothing is lost from the v2 line -- the row is carried and the key is named here. If the value matters in v3, set it with the verb that owns it; nothing here guesses what the key meant",
      ),
      Self::FieldNotRecorded => (
        1,
        "field-not-recorded",
        "nothing to fix -- the artefact predates the field, and the migration carries it as it is",
      ),
      Self::MalformedJson => (
        1,
        "malformed-json",
        "repair the file's JSON, or restore that one file from version control",
      ),
      Self::SchemaInvalid => (
        2,
        "schema-invalid",
        "correct the field named above; `intent schema` prints the shape the file must match",
      ),
      Self::ConflictMarkers => (
        3,
        "conflict-markers",
        "finish the merge in the named file -- Intent will not read around a conflict marker",
      ),
      Self::UnknownFileShape => (
        4,
        "unknown-file-shape",
        "move or rename it -- a modelled directory carries only the artefacts Intent writes",
      ),
      Self::DuplicateId => (
        5,
        "duplicate-id",
        "two artefacts claim one id; rename or remove one of them",
      ),
      // The one remedy that is a command, and it is bounded on purpose: it
      // rewrites artefacts that are re-creatable from the store by
      // definition, so nothing authored is at risk. It says what it costs
      // anyway, because the finding exists BECAUSE someone hand-edited the
      // view, and regenerating is precisely what discards that edit.
      Self::ViewSkew => (
        6,
        "view-skew",
        "`intent sync --to-disk` regenerates the views from the store, DISCARDING the hand edit -- copy anything you meant to keep out first",
      ),
      // **THE FIRST INSTRUCTION IS TO COPY THE FILE ASIDE, AND THAT IS NOT
      // padding.** Unlike `ViewSkew` above, neither side here is derivable:
      // both are authored bytes, and whichever one loses is gone. So the first
      // safe act is the one that costs nothing and removes the irreversibility,
      // before any question of which version was meant.
      //
      // **It names ONE command, and the asymmetry is imposed rather than
      // chosen.** The blast-radius rule (vc, 2026-08-15) forbids every remedy
      // from naming the store-ward direction, on the ground that it replaces
      // the whole store. That conclusion holds for the UNSCOPED form and its
      // stated reason does not survive reading `Store::rebuild`, which deletes
      // tests, criteria, related, attachments, wps, threads and issues -- and
      // NOT `events`. Reported to vc rather than worked around here: a check
      // whose premise has moved is theirs to re-cut, and editing their rule to
      // let my remedy through would be the reverse of taking it seriously.
      //
      // The consequence is real and worth stating where a reader meets it: for
      // an authored attachment the disk copy may be the only good one, and no
      // remedy is currently permitted to name the command that keeps it. Hence
      // the copy-aside instruction, which reaches the same safety without a
      // command at all.
      Self::AttachmentDrift => (
        6,
        "attachment-drift",
        "copy the working file somewhere outside the project FIRST -- nothing can re-derive either side, so this is the only step that cannot lose anything. Then compare it against what the store holds and decide which one you meant; `intent sync --to-disk <ID>` writes the store's version over the file, discarding the working copy you just saved",
      ),
      Self::ModelInconsistent => (
        7,
        "model-inconsistent",
        "the canon says two things that cannot both be true; correct the artefact named above",
      ),
      Self::StatusGateDisagreement => (
        7,
        "status-gate-disagreement",
        "the status and the gate disagree, and only you can say which one is wrong. If the work really is finished, the contract is missing something -- read the blocking ids and either satisfy them or take them out of scope. If it is not finished, `intent wp start <ST>/<NN>` says so. Do NOT reach for `wp done`: it is refused on a blocked gate, which is the same ruling as this report",
      ),
      Self::BackupStale => (
        8,
        "backup-stale",
        "run `intent backup` -- and if a schedule was supposed to be doing this, it is not running",
      ),
      Self::EventLogAbsent => (
        9,
        "event-log-absent",
        "run `intent sync --to-disk` and commit the result -- nothing recomputes history, so until the extract is in the repository it exists only here",
      ),
      // Rank 1, beside `unmigrated`: every other finding on such a project is
      // downstream of it, and on a non-default `st_prefix` there will BE no
      // other findings, because no thread is recognised in the first place.
      Self::RetiredSetting => (
        1,
        "retired-setting",
        "v3 fixes this setting and does not read it -- rename the artefacts to the fixed form before migrating, or the migration will not see them at all. The declaration is left in config.json rather than removed for you",
      ),
      // Last, because nothing is at risk: the estate is intact, one display
      // command refuses, and the fix is one number in config.json. The
      // DETAIL carries the arithmetic -- which value was configured and the two
      // honourable ones either side of it -- because that is per-instance and
      // this string is per-class.
      Self::UnhonourableSetting => (
        10,
        "unhonourable-setting",
        "the value is well-formed and the data cannot honour it; the detail above names what to set instead",
      ),
      // Outside the verdict altogether: a state, not an obligation. It sorts last
      // so the totals line ends with what nobody has to act on.
      Self::Advisory => (
        11,
        "advisory",
        "nothing is owed now: the row is well-formed and resolves. Rewrite it in the v3 grammar when the thread is next touched; a closed thread carries it as it is",
      ),
    }
  }

  /// What an operator should do about it, in words they can act on.
  pub fn remedy(&self) -> &'static str {
    self.meta().2
  }

  /// The wire spelling. Asserted against serde's by test rather than routed
  /// through it: the return is `&'static str`, and serde's is an owned
  /// `String`, so the two cannot be the same function. The test is what makes
  /// this a single authority in practice.
  pub fn as_str(&self) -> &'static str {
    self.meta().1
  }

  /// Declaration order, for a stable totals line.
  fn rank(&self) -> u8 {
    self.meta().0
  }
}

/// One refusal: what was refused, where, and why.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Finding {
  /// Project-relative path of the offending artefact.
  pub file: String,
  /// 1-indexed line, where the class can point at one.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub line: Option<u32>,
  pub class: FindingClass,
  /// Human-actionable detail. Names the specific thing -- the unknown field,
  /// the duplicate id -- never just restates the class.
  pub detail: String,
}

impl Finding {
  pub fn new(file: impl Into<String>, class: FindingClass, detail: impl Into<String>) -> Self {
    Self {
      file: file.into(),
      line: None,
      class,
      detail: detail.into(),
    }
  }

  pub fn at_line(mut self, line: u32) -> Self {
    self.line = Some(line);
    self
  }

  /// Where it is and what it is, with no verdict word and no remedy.
  ///
  /// One body, two leads. A carried finding and a blocking one differ in the
  /// verdict and in whether anything is owed, never in the facts, so factoring
  /// the facts out is what stops the two renderings drifting apart.
  fn body(&self) -> String {
    let line = self.line.map(|l| format!(":{l}")).unwrap_or_default();
    format!(
      "{}{line} -- {} -- {}",
      self.file,
      self.class.as_str(),
      self.detail
    )
  }

  /// **A CARRIED finding: not residue, and it owes no remedy.**
  ///
  /// Both halves of `Display` are wrong for a carried row and the second half is
  /// the harmful one. `residue:` is the word the report reserves for the
  /// BLOCKING bucket, so a carried line led by it contradicts the count printed
  /// beside it -- and the `remedy:` that followed told the operator to go and fix
  /// a row hv's ruling says CONVERTS AS IT IS. **A remedy for a non-problem is
  /// worse than a missing one**: it is work the tool asked for and did not need,
  /// on the operator's first contact with the migrator.
  ///
  /// Measured on the canary by ic: nine carried findings, all in COMPLETED
  /// threads, each printed under its own copy of the section header and each led
  /// `residue:` against a summary line reading `0 blocking, 9 carried`.
  pub fn carried_line(&self) -> String {
    format!("carried: {}", self.body())
  }
}

impl fmt::Display for Finding {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // The two-line refusal grammar the rest of the estate uses: what is wrong,
    // then what to do about it. `doctor --fix` is withdrawn, so this line is
    // the whole of the tool's repair offer -- and it has to be runnable.
    write!(
      f,
      "{}: {}\n  remedy: {}",
      if self.class == FindingClass::Advisory {
        "advisory"
      } else {
        "residue"
      },
      self.body(),
      self.class.remedy()
    )
  }
}

/// A refusal carrying every finding, never only the first.
///
/// The report never truncates (migration.md's no-silent-caps rule): a capped
/// list reads as complete when it is not, which sends the reader round the
/// fix-and-rerun loop once per finding instead of once.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub struct Refusal {
  pub findings: Vec<Finding>,
}

impl crate::remedy::Remedy for Refusal {
  /// **The findings have already said what to do, one line each.**
  ///
  /// A refusal carrying twelve findings has twelve remedies above it, each
  /// naming its own file and class. A thirteenth line summarising them would
  /// be the least specific advice on screen sitting in the most prominent
  /// position, so this points AT them rather than restating them.
  fn remedy(&self) -> String {
    format!(
      "act on the {} finding(s) above -- each names its artefact and what to do; nothing here needs a decision this message could make for you",
      self.findings.len()
    )
  }
}

impl Refusal {
  pub fn new(findings: Vec<Finding>) -> Self {
    Self { findings }
  }

  /// Count per class, in the class's declaration order -- the per-class totals
  /// migration.md's report prints.
  ///
  /// **Counted from the findings present, never from a list of classes.** The
  /// list version was a hand-maintained array the compiler could not check, so
  /// a class added to the enum would simply never appear in the totals line --
  /// a silent undercount, in the function that exists to honour the
  /// no-silent-caps rule. Ordering comes from [`FindingClass::rank`], which is
  /// an exhaustive match, so a new variant cannot be silently dropped OR
  /// silently unordered.
  pub fn totals(&self) -> Vec<(FindingClass, usize)> {
    let mut out: Vec<(FindingClass, usize)> = Vec::new();
    for finding in &self.findings {
      match out.iter_mut().find(|(c, _)| *c == finding.class) {
        Some((_, n)) => *n += 1,
        None => out.push((finding.class, 1)),
      }
    }
    out.sort_by_key(|(c, _)| c.rank());
    out
  }
}

impl fmt::Display for Refusal {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for finding in &self.findings {
      writeln!(f, "{finding}")?;
    }
    let totals = self
      .totals()
      .into_iter()
      .map(|(c, n)| format!("{}: {n}", c.as_str()))
      .collect::<Vec<_>>()
      .join(", ");
    write!(
      f,
      "error: refused {} finding(s) -- {totals}",
      self.findings.len()
    )
  }
}
