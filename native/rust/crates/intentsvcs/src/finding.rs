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
  /// fix** -- reported so the counts reconcile, and kept apart from
  /// `UnknownStatus` / `UnknownScope`, which describe a value that IS there and
  /// is wrong.
  ///
  /// **IT IS NO LONGER THE MIGRATION SCAN'S ALONE, AND THE DOC SAID IT WAS
  /// UNTIL 2026-09-07.** `doctor` emits it for a CLOSED thread carrying no
  /// completion date -- the largest single finding class in the fleet, 50 of
  /// Conflab's 53 counted findings (vc's audit). That case had been
  /// `ModelInconsistent`, whose remedy reads *the canon says two things that
  /// cannot both be true*. It does not: it says one thing and omits another,
  /// and the omitted value never existed in v2 for the migration to carry.
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
  /// The same disagreement, where the gate passes because part of the scope was
  /// FIAT-CLOSED rather than met.
  ///
  /// **A SEPARATE CLASS BECAUSE THE REMEDY IS DIFFERENT, WHICH IS THE ONLY
  /// GROUND THIS FILE HAS EVER SPLIT ON.** `AttachmentDrift` was split from
  /// `ModelInconsistent` on it, and `StatusGateDisagreement` from
  /// `ModelInconsistent` on it again: putting two different remedies behind one
  /// word tells an operator nothing to do.
  ///
  /// **AND THE SHARED REMEDY WAS NOT MERELY VAGUE HERE, IT WAS WRONG.** Its
  /// sibling says *read the blocking ids and either satisfy them or take them
  /// out of scope*. Over a fiat-closed row that is an instruction to convert a
  /// recorded human ruling into an ordinary close -- **laundering, proposed by
  /// the tool, in the estate whose own `IN-AG-FIAT-001` forbids an LLM the
  /// verb.** A remedy that is correct for one population and harmful for
  /// another is why the populations get their own classes.
  ///
  /// **The status is the stale half here, never the close.** A fiat close is a
  /// decision that was made; a `Not Started` beside it is a field nobody
  /// updated.
  StatusGateDisagreementOverFiat,
  /// The durable store has no recent restorable snapshot -- either none has
  /// ever succeeded, or the newest is older than the configured schedule.
  ///
  /// **This is the half of the backup rule that a failure report cannot
  /// cover.** A schedule that never fires produces no failure, so "surface the
  /// failure" leaves a user unable to tell a working backup from one that has
  /// silently never started. It is the two-sided construction: two recorded
  /// values compared to each other rather than an error waited for.
  BackupStale,
  /// The backup mechanism is running and its attempts are failing.
  ///
  /// **THE OTHER HALF OF THE BACKUP RULE, AND [`FindingClass::BackupStale`]
  /// SAYS SO IN ITS OWN NOTE: _the half that a failure report cannot cover._**
  /// This is the failure report. The two are separate classes because they
  /// call for opposite actions -- stale says the mechanism is not running, and
  /// this says it is running and something is stopping it -- and because only
  /// this one has a recorded reason to show.
  ///
  /// **IT FIRES INDEPENDENTLY OF STALENESS, WHICH IS THE POINT.** A store
  /// backed up successfully an hour ago and failing every attempt since is not
  /// stale by any reading, and is on its way to being so with the evidence
  /// already on disk. Waiting for the staleness threshold to report it would
  /// mean the one instrument that can see the cause stays silent until the
  /// symptom arrives.
  BackupFailing,
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
  /// **THE COMMIT GATE IS INSTALLED AND IS NOT RUNNING.**
  ///
  /// Not a stale gate and not an out-of-date one -- those are advisories. This
  /// is the state where a carrier exists, so somebody installed the gate and
  /// the estate believes it has one, and nothing it names can actually execute:
  /// no guard block in the carrier at all, a chain calling a carrier that is
  /// not there, or an install the carrier cannot resolve its guards from.
  ///
  /// **IT IS ACTIONABLE DESPITE HAVING NO VERB THAT REPAIRS IT** (vc, 2026-08-27),
  /// and that pairing is deliberate rather than an oversight. Something IS
  /// broken; the operator's commits are going through ungated while every
  /// surface reports health. Measured across the fleet the property reds 2 of
  /// 17, which is a real and small set -- unlike being behind the template,
  /// which reds 17 of 17 and is therefore an advisory, because a check that
  /// reds every estate permanently is one operators learn to skip, and then it
  /// is not there for the two that need it either.
  GateNotRunning,
  /// A hygiene note, not a fault: the artefact is well-formed and nothing is
  /// blocked by it. Printed under `advisory:` and NOT counted toward the
  /// verdict -- hv, 2026-08-26, on Baize printing 66 of these at rc 1 under the
  /// `model-inconsistent` remedy, which made "pristine doctor" unreachable on
  /// any live estate whose AT rows still cite tests in the v2 grammar.
  /// **THE MODEL CLAIMS THIS ARTEFACT AND NO BUILD CARRIES IT YET.**
  ///
  /// **A CLASS RATHER THAN RESIDUE OR AN ADVISORY BECAUSE BOTH OF THOSE SAY
  /// SOMETHING FALSE ABOUT IT**, which is the only ground this file has ever
  /// split on. Residue is *something a v2 AUTHOR left behind* (`migrate.rs`),
  /// whose remedy names the fixing environment -- nothing here is anyone's
  /// mistake and no v2 command touches it. [`FindingClass::Advisory`] is the
  /// class of what is worth doing *when that artefact is next touched*, and
  /// touching one of these changes nothing: what is unmet is a claim the MODEL
  /// makes, discharged by a build rather than by an operator.
  ///
  /// **AND IT MUST NOT ENTER `Scan`, WHICH IS WHY IT IS NOT ROUTED THROUGH
  /// `record`.** Both of that type's buckets are wrong in a way that is worse
  /// than untidy: `residue` BLOCKS (`migrate.rs`), so every estate carrying one
  /// of these directories would be refused a migration permanently, and
  /// `carried` prints under *converts as-is, no action* -- which is the one
  /// thing these files demonstrably do not do.
  ///
  /// **PER ARTEFACT, BECAUSE THE ESTATE ALREADY HOLDS THAT STANDARD AND WAS
  /// APPLYING IT TO THE SMALLER CLASS.** `legacy.rs` names each oversized
  /// attachment individually, by path, with its own reason; the whole
  /// whiteboard reached the same report as a single directory noun. Measured
  /// 2026-09-05: 8 files named one by one, against 1,386 collapsed into a noun
  /// on Lamplight and 624 on this repository. The collapse is applied in exactly
  /// the direction that makes it least defensible.
  ///
  /// **THE DETAIL SAYS THE FILE IS STILL ON DISK, AND THAT HALF IS
  /// LOAD-BEARING** -- the same requirement `migration_not_yet_built`'s own
  /// test pins on the summary line. Without it a per-artefact enumeration reads
  /// as a loss manifest, which is the opposite of what it records.
  ///
  /// **IT DOES NOT LICENCE A DROP.** A declared exclusion silences
  /// `conservation_check.sh` for the paths it names, so a class that named
  /// everything would be the denominator attack `NOT_CARRIED` warns about. What
  /// keeps it honest is that the claim is verifiable in the direction that
  /// matters: `Verdict::Dropped` is corroborated by canon being EMPTY, and this
  /// by the file being PRESENT and unchanged -- so naming a file that had
  /// actually gone would be refutable rather than merely unattractive.
  ModelledNotBuilt,
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
      //
      // THIS REMEDY USED TO END BY SAYING `--to-disk` "writes the store's
      // version over the file, discarding the working copy you just saved". It
      // does not: `projection` carries no attachment path and `views.rs` emits
      // no attachment file, so that direction cannot touch this file at all.
      // Driven 2026-08-30 with the store holding the old text and the working
      // file holding new edits -- after `sync --to-disk` both were still
      // modified, which a write-back could not have left. Issue 0165. The
      // backup advice stays because it is good practice; what went is the false
      // mechanism, because A REMEDY THAT OVERSTATES A DESTRUCTIVE STEP TEACHES
      // DISTRUST OF THE ONES THAT DO NOT, and the cost lands on the remedy that
      // is telling the truth.
      Self::AttachmentDrift => (
        6,
        "attachment-drift",
        "copy the working file somewhere outside the project FIRST -- nothing can re-derive either side, so this is the only step that cannot lose anything. Then compare it against what the store holds and decide which one you meant, and put the text you meant where you want it BY HAND: an attachment is AUTHORED, so no sync direction rewrites it for you -- `--to-disk` re-derives the generated views and leaves this file exactly as it stands",
      ),
      Self::ModelInconsistent => (
        7,
        "model-inconsistent",
        "the canon says two things that cannot both be true; correct the artefact named above",
      ),
      Self::StatusGateDisagreement => (
        7,
        "status-gate-disagreement",
        "the status and the gate disagree, and only you can say which one is wrong. THIS CLASS CARRIES BOTH DIRECTIONS and they take opposite actions. GATE BLOCKED, status Done: the contract is missing something -- read the blocking ids and either satisfy them or take them out of scope; do NOT reach for `wp done`, which is refused on a blocked gate, the same ruling as this report. GATE PASSES, status not Done: either the work is finished and the field is stale -- `intent wp done <ST>/<NN>`, and from `Not Started` that needs `intent wp start <ST>/<NN>` first because `wp.done` is declared only from `wip` -- or the scope is satisfied for a reason other than completion, which is what PARKED work looks like: a park records its own satisfied criterion, so the gate passes over work nobody has started. Read the work package before you close it",
      ),
      Self::StatusGateDisagreementOverFiat => (
        7,
        "status-gate-disagreement-over-fiat",
        "a human closed part of this scope ON AUTHORITY, so the gate passes and it is the STATUS that is stale -- `intent wp done <ST>/<NN>` records the close the gate already allows, and from `Not Started` that needs `intent wp start <ST>/<NN>` first because `wp.done` is declared only from `wip`. Do NOT satisfy the fiat-closed rows or take them out of scope to tidy this: that converts a recorded ruling into an ordinary close and erases who decided, which is the one thing a fiat close exists to preserve. If the ruling itself was wrong, reverse it deliberately with `intent ac reinstate <ST> <AC>`, which puts the row back in scope unsatisfied and leaves the reversal on the record",
      ),
      Self::BackupStale => (
        8,
        "backup-stale",
        "run `intent backup` -- and if a schedule was supposed to be doing this, it is not running",
      ),
      // **Rank 8 beside `BackupStale`, and the two can BOTH fire**, which is
      // the state a store failing for longer than its period is actually in.
      // The remedies are deliberately opposite in one respect: that one says
      // run the verb, and this one says do not -- an attempt that just failed
      // for a recorded reason will fail again the same way, and a remedy that
      // sends an operator to re-run it converts a true finding into a second
      // failure they now believe they caused.
      Self::BackupFailing => (
        8,
        "backup-failing",
        "the backup mechanism IS running and its attempts are failing -- the detail above carries what the newest one recorded, and `intent backup --list` carries the rest. Fix what the detail names; re-running `intent backup` will fail the same way and tells you nothing new",
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
      // Beside `BackupStale`, and for the same reason: both are a protection the
      // operator believes they have and does not.
      //
      // **THE REMEDY WORDING IS dc's, MATCHED RATHER THAN RE-AUTHORED**
      // (`bin/.devbin/cmd/hooks`, ruled 2026-08-27, authority vc). One fact
      // about the estate reported by two tools in two phrasings is the same
      // drift this codebase calls Highlander everywhere else, and the fact here
      // is unusually easy to phrase almost-right: `intent claude upgrade
      // --apply` region-edits the chain and never writes the carrier, so **a
      // detector that names it hands the operator a command which does not
      // repair what it just reported** -- they run it, see no error, and the
      // true finding becomes a false reassurance.
      Self::GateNotRunning => (
        8,
        "gate-not-running",
        "the pre-commit gate is installed and cannot execute, so commits are going through ungated and an unwired guard does not fail -- it reports nothing, which is indistinguishable from passing. NO VERB CURRENTLY REPAIRS THIS, and naming one would be worse than naming none: installing the carrier is an OPEN ITEM -- report it, do not improvise a fix across the fleet. The detail above names which of the three ways it is broken",
      ),
      // Outside the verdict altogether: a state, not an obligation. It sorts last
      // so the totals line ends with what nobody has to act on.
      // **A PER-CLASS STRING CANNOT NAME A SUBJECT, AND THIS ONE DID** (issue
      // 0106). It read "the ROW is well-formed and resolves. Rewrite it in the
      // v3 GRAMMAR when the THREAD is next touched" -- which is the remedy for
      // a legacy AT reference, and `Advisory` is not that class. It is the
      // class of everything reported and not counted, and the hook-carrier
      // advisory is also in it: a hook carrier is not a row, is not a thread,
      // is not touched, and has no grammar.
      //
      // It was never a WRONG STRING; it was one subject's remedy promoted to a
      // class shared by two. **AND NOTHING WAS LOST BY DEMOTING IT**, because
      // the AT advisory's own DETAIL already ends "worth rewriting in the v3
      // grammar next time the thread is touched" -- so the sentence had two
      // homes, and the per-class one was the copy that could not stay true as
      // the class grew members. Same rule the `UnhonourableSetting` comment
      // states a few lines up: what is per-instance goes in the detail, and
      // this string carries only what is true of the class.
      // Outside every verdict, like `Advisory` and for a DIFFERENT reason: an
      // advisory is a state of the artefact, this is a state of the BUILD.
      // Ranked with it because both sort past everything anyone must act on,
      // and the rank is the only one of these three fields a per-artefact
      // record consumes indirectly -- this class reaches no `Report` and no
      // `Scan`, so nothing totals it today. Kept correct anyway: a rank that is
      // wrong only while nothing reads it is wrong on the day something does.
      Self::ModelledNotBuilt => (
        11,
        "modelled-not-built",
        "nothing is owed and nothing was lost -- the file is untouched on disk, and what is unmet is a claim the MODEL makes about it. No command repairs this and none should be offered: it is discharged when the build carrying this part of the model lands, at which point these lines stop being emitted on their own",
      ),
      Self::Advisory => (
        11,
        "advisory",
        "nothing is owed now: this is reported for visibility and is not counted in the verdict. The detail above is the whole finding -- it names the subject and says what, if anything, is worth doing when that artefact is next touched",
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

  /// Whether a finding of this class OBLIGES anything.
  ///
  /// **THE CONCEPT IS "IS ANYTHING OWED", NOT "IS IT SPELLED `Advisory`", AND
  /// THE DIFFERENCE IS WHY THIS IS A METHOD ON THE CLASS RATHER THAN A FILTER
  /// AT A CALL SITE.** `Report::actionable` asked `class == Advisory` and so
  /// did the renderer's print filter -- **one concept spelled twice, in two
  /// crates**, so widening it in one place would have uncounted a class and
  /// gone on printing it. Here a new class must answer the question where it
  /// is defined, beside its own remedy, and both callers get the same answer
  /// by construction.
  ///
  /// **THE TEST FOR MEMBERSHIP IS THE CLASS'S OWN REMEDY.** A class whose
  /// remedy is the words *nothing to fix* is not an obligation, and the doc on
  /// `Report::is_healthy` already says so in hv's words (2026-08-26): *an
  /// advisory describes a state, not an obligation*. Each member states its
  /// ground rather than inheriting one:
  ///
  /// - [`FindingClass::Advisory`] -- reported for visibility; the original
  ///   member and the reason the old name was accurate when it was written.
  /// - [`FindingClass::FieldNotRecorded`] -- *the artefact predates the field,
  ///   and the migration carries it as it is*. Nothing was lost and nothing can
  ///   be authored: asking someone to fill it in is asking them to invent data
  ///   about finished work.
  /// - [`FindingClass::ModelledNotBuilt`] -- *nothing is owed and nothing was
  ///   lost*. **It reaches no `Report` today** -- emitted only from
  ///   `sync.rs`, verified rather than assumed -- so its membership changes no
  ///   count now. Classified anyway on its own note's reasoning: a rank that is
  ///   wrong only while nothing reads it is wrong on the day something does.
  ///
  /// **NOT A JUDGEMENT ABOUT SEVERITY.** A class can be serious and still owe
  /// nothing; what is being asked is whether an operator has something to DO.
  pub fn is_actionable(&self) -> bool {
    !matches!(
      self,
      Self::Advisory | Self::FieldNotRecorded | Self::ModelledNotBuilt
    )
  }

  /// The word a report leads with for this class.
  ///
  /// **IT ASKS THE SAME QUESTION THE VERDICT ASKS, AND THAT IS THE WHOLE
  /// RULE.** `residue:` is the word reserved for the blocking bucket, so a line
  /// leading with it while the summary does not count it makes the report
  /// contradict itself in one breath.
  ///
  /// **DRIVEN, NOT IMAGINED (hv, 2026-09-07, on Conflab):** the previous form
  /// enumerated CLASSES -- `Advisory`, `ModelledNotBuilt`, `_` -- while the
  /// verdict asked [`FindingClass::is_actionable`]. `FieldNotRecorded` is not
  /// actionable and was not in that list, so `intent doctor -v` printed FIFTY
  /// lines leading `residue:` above a summary reading `0 finding(s)`, each
  /// carrying the remedy "nothing to fix".
  ///
  /// **AND THE COMMENT THIS REPLACES CLAIMED A PROTECTION IT DID NOT HAVE** --
  /// "a match is refused by the compiler when the next variant forgets to
  /// choose". A `_` arm is never refused; it is exactly what let the new class
  /// through in silence. Asking the predicate cannot go stale that way: a class
  /// added to `is_actionable`'s exempt list changes its lead in the same edit.
  ///
  /// **ONE HOME, TWO READERS**: `Display` for an ungrouped line, and the
  /// grouped report's header, which asks it once per group instead of once per
  /// member.
  ///
  /// `ModelledNotBuilt` keeps its own word ahead of the predicate: it is also
  /// uncounted, and "not-yet-carried" says the more specific true thing about a
  /// shape nothing is wrong with.
  pub fn lead(&self) -> &'static str {
    match self {
      Self::ModelledNotBuilt => "not-yet-carried",
      class if !class.is_actionable() => "advisory",
      _ => "residue",
    }
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

  /// Where it is and what is true of it -- WITHOUT the class name.
  ///
  /// **For a caller that has already said the class**, which is what grouping
  /// by class makes possible: repeating the class on every member of a group
  /// headed by that class is the same duplication as repeating the remedy, one
  /// field over. [`Finding::body`] stays as it is, because an ungrouped line
  /// still has to name its own class.
  pub fn where_and_what(&self) -> String {
    let line = self.line.map(|l| format!(":{l}")).unwrap_or_default();
    format!("{}{line} -- {}", self.file, self.detail)
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

  /// **A MODELLED-NOT-BUILT record: one line, no remedy, and never `residue:`.**
  ///
  /// `carried_line`'s argument, arriving at a third class and settled the same
  /// way: the facts are factored into [`Finding::body`] so the renderings
  /// cannot drift, and what differs is the lead and whether anything is owed.
  ///
  /// **NO REMEDY, AND HERE THAT IS ABOUT VOLUME AS WELL AS TRUTH.** The class
  /// remedy is a real sentence and belongs in the report exactly once; appended
  /// per line it would repeat itself 624 times on this repository and 1,386 on
  /// Lamplight, which is how a report that names everything becomes one nobody
  /// reads -- and an unread enumeration is the directory noun again, spelled
  /// out at length.
  pub fn not_built_line(&self) -> String {
    format!("not-yet-carried: {}", self.body())
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
      // **THE LEAD ASKS THE SAME QUESTION THE COUNT ASKS, AND THAT IS THE
      // WHOLE RULE.** `residue:` is the word this report reserves for the
      // blocking bucket, so a line leading with it while the verdict does not
      // count it makes the report contradict itself in the same breath.
      //
      // **DRIVEN, NOT IMAGINED (hv, 2026-09-07, on Conflab):** the previous
      // form enumerated CLASSES -- `Advisory`, `ModelledNotBuilt`, `_` -- while
      // the verdict asked `is_actionable()`. `FieldNotRecorded` is not
      // actionable and was not in the list, so `intent doctor -v` printed FIFTY
      // lines leading `residue:` above a summary reading `0 finding(s)`. Every
      // one of them carried the remedy "nothing to fix", which is the sentence
      // that makes the contradiction unmissable once it is on screen.
      //
      // **AND THE COMMENT THIS REPLACES CLAIMED A PROTECTION IT DID NOT HAVE**
      // -- "a match is refused by the compiler when the next variant forgets to
      // choose". A `_` arm is never refused; it is exactly what let the new
      // class through in silence. Asking the predicate cannot go stale that
      // way: a class added to `is_actionable`'s exempt list changes its lead in
      // the same edit.
      //
      // `ModelledNotBuilt` keeps its own word ahead of the predicate: it is
      // also uncounted, and "not-yet-carried" says the more specific true
      // thing about a shape nothing is wrong with.
      self.class.lead(),
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
