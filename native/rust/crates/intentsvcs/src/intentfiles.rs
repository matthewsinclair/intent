//! `.intentfiles` -- the realisation manifest and its REFUSING grammar.
//!
//! WP-02 of ST0057. The manifest declares WHICH ARTEFACTS are realised to disk.
//! It is committed, it is one flat list, and its parser refuses rather than
//! skips.
//!
//! # The grammar refuses (AC-02.1)
//!
//! A line the parser cannot read ABORTS the parse with its line number. It is
//! never skipped, and the distinction is the whole point: a skipped line drops
//! an artefact from realisation and leaves an estate **indistinguishable from
//! one that never listed it**. There is no signal, no diff, and no way to tell
//! the two apart afterwards -- the silent-drop shape v2.19.0 already paid for
//! twice, in `ac gate`'s F1 fix and in the AT row grammar's `at lint`.
//!
//! # One flat list (ST0057 D57-9)
//!
//! Every entry declares its artefact realised, wherever it sits in the file.
//! The file once had a generated region between `# BEGIN INTENT` and
//! `# END INTENT`, rewritten from status, and a pinned region outside it that
//! survived the rewrite. hv replaced that design on 2026-08-19: commands change
//! the list and nothing recomputes it, so a hand-added line has no rewrite to
//! survive. The markers went on 2026-08-20 (D57-9, carried out by issue 0338).
//!
//! **A line that is exactly one of the old markers is REFUSED**, not admitted
//! as the comment its `#` would otherwise make it. A manifest written for the
//! two-region grammar is told why, rather than parsing on with a construct
//! that no longer means anything.
//!
//! # Artefacts, never files (AC-02.5)
//!
//! The manifest answers _which artefacts are realised_; [`crate::project::Project::classify`]
//! answers _what is this file_. They COMPOSE -- `STEELTHREAD:ST0056` realises
//! the thread and, through `classify`, whatever files that thread produces.
//!
//! **Neither may acquire a second, independent enumeration of files.** Two
//! declarations of which-files-matter agree for months and then quietly do not.
//! That is held MECHANICALLY here rather than by convention: an id must satisfy
//! [`model::is_thread_id`] or [`model::is_issue_id`], and no path satisfies
//! either -- `/` is not an ascii digit and no path is four digits long. A
//! file-valued line is therefore UNREPRESENTABLE, not merely discouraged, and
//! the refusal costs no separate check.

use crate::model;
use crate::remedy::Remedy;
use thiserror::Error;

/// The two-region design's markers, retired by D57-9. A line that is exactly
/// one of them is refused as [`IntentfilesError::NotAnEntry`].
const RETIRED_MARKERS: [&str; 2] = ["# BEGIN INTENT", "# END INTENT"];

/// What kind of artefact a manifest line names.
///
/// **Exactly ONE as of 2026-08-20, closed, and still an enum.** A second sigil
/// is a model change and must be one -- an open sigil space is the second
/// enumeration AC-02.5 forbids, arriving through the vocabulary instead of
/// through the grammar.
///
/// # `ISSUE` was here and hv retired it
///
/// **Issues are CANON-AND-STORE ONLY: an issue has no realised form in the
/// estate, so a manifest line naming one could never be about a file.** Every
/// issue path in `project.rs` is canon-side (`canon_issue_rel`, `issues_dir`,
/// `issue_json`) and `views.rs` renders no issue view, so `Facade::hydrate`'s
/// issue arm resolved into `intent/.canon/issues/` while its thread arm
/// resolved into the estate -- **two arms of one match addressing two different
/// layers.** Driven once before it was wired, `intent issues hydrate 0001`
/// wrote `ISSUE:0001` into the live manifest and reported `ok` over 0 files.
///
/// # Why this stays an enum at arity one, which is the part worth stating
///
/// A single-variant enum reads as ceremony, and deleting it in favour of a bare
/// `&str` or an implicit STEELTHREAD would be the obvious tidy. **It would also
/// be wrong, because the sigil space is about to GROW rather than disappear.**
/// cc's partition of the 250-odd files under `intent/` that no store row owns
/// finds 59 that are project content wanting an owner, and vc's reading of the
/// same set is that *the blocker is ARITY, not policy* -- the ownable set is
/// empty because nothing but a thread can be named here. That work lands as a
/// new variant beside this one.
///
/// So the enum is the extension point, and collapsing it now would mean
/// re-creating it. **Arity one is a fact about today's grammar, not evidence
/// that the type has stopped earning its place.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sigil {
  SteelThread,
  /// **RESTORED BY ST0069 WP-01, AND ONLY BECAUSE THE REASON IT WENT IS GONE.**
  /// `ISSUE` was retired on 2026-08-20 because an issue had no realised form,
  /// so a manifest line naming one could never be about a file and `intent
  /// issues hydrate 0001` reported `ok` over zero files. An issue now renders
  /// to `intent/issues/<nnnn>.md` through [`crate::views::issue`], so the line
  /// names something that exists. **The retirement was right on its facts and
  /// this is not a reversal of it** -- the facts changed first.
  Issue,
}

impl Sigil {
  /// The wire form, which is also the only accepted spelling.
  pub fn as_str(&self) -> &'static str {
    match self {
      Sigil::SteelThread => "STEELTHREAD",
      Sigil::Issue => "ISSUE",
    }
  }

  /// Parse a sigil, or `None`. Case-sensitive: a manifest is committed and
  /// diffed, so one spelling keeps the diff about the change.
  ///
  /// **`ISSUE` is not special-cased into a friendlier error.** A manifest
  /// written against the old grammar hits `UnknownSigil` like any other typo,
  /// and that is the right answer: the line does not name something this tool
  /// can realise, so there is nothing to migrate it TO.
  pub fn parse(s: &str) -> Option<Self> {
    match s {
      "STEELTHREAD" => Some(Sigil::SteelThread),
      "ISSUE" => Some(Sigil::Issue),
      _ => None,
    }
  }

  /// Whether `id` is well-formed FOR THIS SIGIL.
  ///
  /// Delegated to `model`, which owns identity. A shape asserted here as well
  /// would be a second declaration of one fact.
  ///
  /// `model::is_issue_id` is untouched by the retirement and still has callers:
  /// issues keep their identity in canon and the store. **What ended is the
  /// manifest's claim on them, not the id.**
  pub fn accepts(&self, id: &str) -> bool {
    match self {
      Sigil::SteelThread => model::is_thread_id(id),
      Sigil::Issue => model::is_issue_id(id),
    }
  }
}

/// One artefact the manifest names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
  pub sigil: Sigil,
  pub id: String,
  /// The trailing comment with its `#` and surrounding space removed, if the
  /// line carried one. This is where AC-02.3's "nothing names the decision"
  /// gets its answer, so it is preserved rather than discarded at parse.
  pub comment: Option<String>,
  /// 1-indexed, as a human reads the file.
  pub line: usize,
}

/// A parsed manifest.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Manifest {
  pub entries: Vec<Entry>,
}

/// WHAT IS REALISED TO DISK, as the manifest answers it.
///
/// **Three states, because the two absent-ish ones must not be collapsed.**
/// Both `NothingSaid` and `Unreadable` realise everything, so a caller that
/// only decides what to write can treat them alike -- but a caller that
/// REPORTS has to tell them apart, and re-deriving the distinction from the
/// filesystem afterwards is how it gets lost.
///
/// Lifted out of `Facade` (where it was private) when `doctor` became the
/// second reader. **One answer to "what is realised", consulted by the write
/// path and the diagnostic path alike** -- two copies of this would let
/// `projection` and `doctor` disagree about whether a view should exist, which
/// is precisely the divergence `.intentfiles` exists to settle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Realised {
  /// There is no manifest. Nobody has said, so everything is realised -- the
  /// state of every project that has never run `organize`.
  NothingSaid,
  /// A manifest was read. **An EMPTY set is somebody saying NONE and is
  /// honoured**, which is what makes this different from [`Realised::NothingSaid`].
  Declared(std::collections::BTreeSet<String>),
  /// A manifest exists and does not parse. Realises everything, like
  /// [`Realised::NothingSaid`], and kept distinct so a reporter can tell them
  /// apart without going back to the filesystem.
  Unreadable,
}

impl Realised {
  /// Does the manifest declare this thread realised?
  ///
  /// **ABSENT IS NOT EMPTY.** `NothingSaid` and `Unreadable` answer `true` for
  /// everything -- a missing or broken manifest keeps the whole estate on disk,
  /// which is the fail-open direction and the only one that cannot delete
  /// anybody's files. Only a manifest that PARSED gets to say no.
  pub fn declares(&self, thread_id: &str) -> bool {
    self.declares_artefact(Sigil::SteelThread, thread_id)
  }

  /// Does the manifest declare THIS artefact realised, whatever kind it is?
  ///
  /// **THE ONE RULE; [`Self::declares`] IS A THREAD-SHAPED CALLER OF IT AND NOT
  /// A SECOND SPELLING.** The set was keyed on bare ids and filtered to
  /// STEELTHREAD at the door, so `ISSUE:0001` parsed, was dropped on the way
  /// in, and every reader answered `false` for a line the operator had
  /// written. `declared_set`'s own doc had predicted exactly that -- *a change
  /// to the sigil space would have to be made twice and the second site would
  /// be found by a user* -- and the second site was this one.
  ///
  /// Keyed on the WIRE FORM rather than the id, so two kinds sharing an id
  /// cannot be confused for each other. Nothing today can collide (`ST0001`
  /// against `0001`), which is a fact about the id shapes rather than a
  /// property of this type.
  pub fn declares_artefact(&self, sigil: Sigil, id: &str) -> bool {
    match self {
      Realised::NothingSaid | Realised::Unreadable => true,
      Realised::Declared(set) => set.contains(&declared_key(sigil, id)),
    }
  }
}

/// Read the manifest at `path` and say what it declares realised.
///
/// **FAIL-OPEN, AND THE DIRECTION IS CHOSEN RATHER THAN INHERITED.** A broken
/// manifest realises everything and can never dehydrate, so the failure cannot
/// delete anybody's files. Refusing instead would make one malformed line break
/// every write in the project, and the grammar's real refusal belongs on the
/// verbs that read the manifest deliberately, where the operator is asking
/// about it and can act on the answer.
pub fn realised(path: &std::path::Path) -> Realised {
  let Ok(raw) = std::fs::read_to_string(path) else {
    return Realised::NothingSaid;
  };
  realised_from(&raw)
}

/// What a manifest's TEXT declares realised -- [`realised`] without the file.
///
/// **Split out on 2026-08-20 so the derivation has one home and a door that
/// does not need the filesystem.** It was inline in [`realised`], which meant
/// the only way to ask "what does this text declare" was to write it to a temp
/// file first -- so anything testing `unpin` against the consumer's real
/// question had to either do that or RE-DERIVE the answer from `parse`, and
/// the second is a second reader of the same rule. `realised` now delegates,
/// so the two can never disagree.
///
/// **BOTH HALVES OF WHAT THIS COMMENT USED TO SAY WERE FALSE, AND THEY FAILED
/// IN OPPOSITE DIRECTIONS** (issue 0514, found by vc 2026-09-22 while
/// establishing whether `ISSUE:<NNNN>` is a live address form for `wb claim`).
/// It read: "THE SIGIL FILTER BELOW IS A NO-OP TODAY AND IS NOT DEAD -- hv
/// retired `ISSUE:` on 2026-08-20, so every entry is a `SteelThread` and the
/// filter excludes nothing."
///
/// `ISSUE:` IS LIVE. ST0069 WP-01 gave an issue a realised form, and
/// `organize --default` writes one `ISSUE:` entry per OPEN issue; the live
/// manifest in this repository carries them right now, and this module's own
/// header states the rule -- every WIP thread and every OPEN issue is
/// declared, and nothing else.
///
/// AND THERE IS NO FILTER BELOW. `declared_set` maps every entry through
/// `declared_key` and excludes nothing of any kind. The filter this comment
/// defended was removed when the set stopped being keyed on bare ids, which
/// `declared_key`'s own comment records a few lines down. So it argued for
/// keeping a mechanism that had already gone, and cited a test as evidence
/// that no fixture could catch its removal -- which was true, and was the
/// reason nothing objected for a month.
///
/// **THE POINT IS NOT THE TWO FACTS BUT THAT A DOC COMMENT HAS NO CHECKER.**
/// This sentence stated a POLICY -- that a sigil kind is retired -- in the
/// module that owns the manifest's vocabulary, which is exactly the kind of
/// sentence a reader takes as current instead of re-deriving. It was taken
/// that way, and the live manifest is what corrected it. Nothing else could
/// have.
pub fn realised_from(text: &str) -> Realised {
  match parse(text) {
    Ok(manifest) => Realised::Declared(declared_set(&manifest)),
    Err(_) => Realised::Unreadable,
  }
}

/// The thread ids a PARSED manifest declares realised.
///
/// **Extracted so the two doors cannot disagree**, which is the same reason
/// `realised` was lifted out of `Facade` when `doctor` became its second
/// reader. [`realised_from`] and [`realised_for_action`] differ ONLY in what
/// they do with a parse failure; if they also each spelled the filter, a
/// change to the sigil space would have to be made twice and the second site
/// would be found by a user.
fn declared_set(manifest: &Manifest) -> std::collections::BTreeSet<String> {
  manifest
    .entries
    .iter()
    .map(|e| declared_key(e.sigil, &e.id))
    .collect()
}

/// How one artefact is KEYED in a [`Realised::Declared`] set: the wire form.
///
/// **ONE SPELLING, BECAUSE THE SET IS PUBLIC AND HAS READERS OUTSIDE THIS
/// MODULE.** The set held bare ids until ST0069 WP-01 and three places in
/// `facade.rs` matched on `Realised::Declared` and did their own
/// `contains(id)` -- so widening the key here silently stopped every one of
/// them from finding a thread, and `st start` quietly stopped realising the
/// thread it had just declared. A `format!` at each site would have been the
/// same defect waiting for the next kind.
pub fn declared_key(sigil: Sigil, id: &str) -> String {
  format!("{}:{}", sigil.as_str(), id)
}

/// What an ACTING verb sees in the manifest's text: the same three-state model
/// as [`realised_from`], except that **a manifest which exists and will not
/// parse is an `Err` rather than a fail-open `Unreadable`.**
///
/// **THE TWO DOORS EXIST BECAUSE THE TWO CALLERS OWE THE OPERATOR DIFFERENT
/// THINGS, AND THIS MODULE ALREADY SAID SO BEFORE THE DOOR EXISTED.**
/// [`realised`]'s own comment: *"the grammar's real refusal belongs on the
/// verbs that read the manifest deliberately, where the operator is asking
/// about it and can act on the answer."* A REPORTER (`doctor`) must answer
/// about a broken manifest rather than refuse to run, so it fails open. An
/// ACTOR (`organize`, `edit`) is about to write and remove files on the
/// strength of what the manifest says, and **acting on a file it could not
/// read is the one thing it must never do** -- so the refusal, with its line
/// number, belongs here.
///
/// **ABSENCE IS NOT THIS FUNCTION'S BUSINESS**, and that is deliberate: it
/// takes TEXT, so a caller holding text has already established the file is
/// there. Absent is decided at the filesystem, once, by the caller that
/// touches the filesystem -- rather than being inferred here from an empty
/// string, which is a real and different state (a manifest declaring NONE).
pub fn realised_for_action(text: &str) -> Result<Realised, IntentfilesError> {
  Ok(Realised::Declared(declared_set(&parse(text)?)))
}

/// Why a manifest could not be read.
///
/// **Every variant carries the 1-indexed line number**, because AC-02.1 asks
/// for it in the output and an error that names only the reason sends the
/// operator to search a file for a line the parser already knew.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum IntentfilesError {
  #[error("line {line}: `{found}` is not a known sigil -- expected STEELTHREAD or ISSUE")]
  UnknownSigil { line: usize, found: String },
  #[error("line {line}: `{line_text}` is not `<SIGIL>:<ID>`")]
  NotAnEntry { line: usize, line_text: String },
  #[error("line {line}: `{id}` is not a valid id for {sigil}")]
  MalformedId {
    line: usize,
    sigil: &'static str,
    id: String,
  },
}

impl IntentfilesError {
  /// The 1-indexed line the refusal is about.
  ///
  /// Exposed so a caller can report position without re-parsing the message,
  /// which is the shape that lets one rendering serve a CLI and a test alike.
  pub fn line(&self) -> usize {
    match self {
      IntentfilesError::UnknownSigil { line, .. }
      | IntentfilesError::NotAnEntry { line, .. }
      | IntentfilesError::MalformedId { line, .. } => *line,
    }
  }
}

impl Remedy for IntentfilesError {
  fn remedy(&self) -> String {
    match self {
      IntentfilesError::UnknownSigil { .. } => {
        "write STEELTHREAD:<ID> or ISSUE:<NNNN>; the manifest names artefacts, never files".into()
      }
      IntentfilesError::NotAnEntry { .. } => {
        "each line is blank, a comment, or `<SIGIL>:<ID>` with an optional trailing `# comment`; `# BEGIN INTENT` and `# END INTENT` are no longer part of the grammar, so delete a marker line and keep the entries around it".into()
      }
      IntentfilesError::MalformedId { sigil, .. } => match *sigil {
        "STEELTHREAD" => "a steel-thread id is ST followed by four digits, eg ST0000".into(),
        "ISSUE" => "an issue id is four digits, eg 0001".into(),
        // **STILL NOT A GUESS, AND THE CATCH-ALL EARNS ITS PLACE AGAIN THE DAY
        // A THIRD SIGIL ARRIVES.** This arm briefly covered every sigil but
        // STEELTHREAD, because ISSUE had been retired and the grammar had one
        // member; ISSUE is back above with the shape it actually has. What the
        // arm must never do is name a shape for a sigil nobody has written a
        // remedy for, so it says which sigil it cannot help with.
        other => format!(
          "`{other}` is a sigil this remedy has no id shape for -- add one beside STEELTHREAD's"
        ),
      },
    }
  }
}

/// Parse a manifest, refusing at the first line that is not readable.
///
/// **First failure wins and the parse aborts.** Collecting every bad line and
/// reporting them together reads as more helpful and is not: a manifest is
/// written a line at a time, so the second reported error is usually a
/// consequence of the first, and a caller that receives a list has to decide
/// which one to act on. AC-02.1 asks that the run exit non-zero and name the
/// offending line -- singular.
pub fn parse(text: &str) -> Result<Manifest, IntentfilesError> {
  let mut entries = Vec::new();

  for (idx, raw) in text.lines().enumerate() {
    let line = idx + 1;
    let trimmed = raw.trim();

    // Ahead of the comment arm, which would otherwise admit a retired marker
    // for the `#` it starts with.
    if RETIRED_MARKERS.contains(&trimmed) {
      return Err(IntentfilesError::NotAnEntry {
        line,
        line_text: trimmed.to_string(),
      });
    }

    // Blank and whole-line comments carry no artefact. They are admitted
    // rather than refused because AC-02.3's stated purpose is that the file
    // NAME THE DECISION behind a pin, and a committed file a human edits that
    // cannot hold a sentence defeats that on its own terms.
    if trimmed.is_empty() || trimmed.starts_with('#') {
      continue;
    }

    let (body, comment) = match trimmed.split_once('#') {
      Some((b, c)) => (b.trim_end(), Some(c.trim().to_string())),
      None => (trimmed, None),
    };

    let Some((sigil_text, id)) = body.split_once(':') else {
      return Err(IntentfilesError::NotAnEntry {
        line,
        line_text: trimmed.to_string(),
      });
    };
    let (sigil_text, id) = (sigil_text.trim(), id.trim());

    let Some(sigil) = Sigil::parse(sigil_text) else {
      return Err(IntentfilesError::UnknownSigil {
        line,
        found: sigil_text.to_string(),
      });
    };
    if !sigil.accepts(id) {
      return Err(IntentfilesError::MalformedId {
        line,
        sigil: sigil.as_str(),
        id: id.to_string(),
      });
    }

    entries.push(Entry {
      sigil,
      id: id.to_string(),
      comment,
      line,
    });
  }

  Ok(Manifest { entries })
}

// ---------------------------------------------------------------------------
// The writer -- AC-02.2, AC-02.3
// ---------------------------------------------------------------------------
//
// **`Generated` AND `render` WERE DELETED HERE (hv ruling, 2026-08-20).** They
// rewrote a GENERATED REGION from status, which is the design hv replaced on
// 2026-08-19: `.intentfiles` is durable state, commands CHANGE it, and nothing
// recomputes it. `facade.rs` had already written the epitaph -- *`render` had
// no production caller because the thing it does is not needed* -- and remove
// the regeneration and the protected region has nothing left to protect
// against.
//
// **They were deleted rather than left because two test files were still
// driving them GREEN.** `edit_writes_pinned_region.rs` -- since renamed
// `pin_writes_to_the_list.rs` -- at least sat behind a RED row (AT-05.2)
// naming it; `intentfiles_pin_survives_close.rs` was named by
// no AT row at all after AT-02.3 was re-pointed onto
// `intentfiles_is_the_list.rs`, so it was nine passing assertions over a design
// that no longer exists. **A red row says work is owed; an unnamed green file
// says work is done, which is strictly worse.**
//
// `Region`, `Manifest::pinned()/generated()` and the BEGIN/END markers
// outlived them only because `pin` used them, and went on hv's D57-9 ruling of
// 2026-08-20, carried out by issue 0338: the list is flat.

/// The header a generated default carries, and the ONLY home for that text.
///
/// hv hand-wrote Intent's own manifest, so until now no header existed in the
/// tool at all -- which is why this constant is here rather than in a template:
/// the file is written by a function, and a template would be a second home for
/// a string only that function emits.
const DEFAULT_HEADER: &str = "\
# .intentfiles -- WHICH DATABASE ARTEFACTS ALSO HAVE A REALISED FORM ON DISK.
#
# Written by `intent organize --default` from status: every WIP thread and every
# OPEN issue is declared, and nothing else. A thread that is Not Started, Triage,
# Hold, Completed or Cancelled is NOT realised, and neither is a closed issue --
# they live in the store, in full, and `intent st hydrate` brings a thread back
# to disk on demand.
#
# REALISATION IS DRIVEN FROM THIS FILE. Commands change it; `intent organize`
# realises it. Nothing recomputes it from status afterwards, so a write here is
# a CHANGE TO STATE and never a REGENERATION of it -- which is why re-running
# `--default` over a file that already exists changes nothing without `--force`.
#
# Many writers, one meaning. `st start`, `st resume` and `st reopen` add the id;
# `st done`, `st cancel`, `st hold` and `st triage` remove it; `st new` and
# `st reinstate` do neither; `st hydrate` / `st dehydrate` do it directly; and a
# human may edit it by hand. All of those are ordinary writers; none is
# privileged. The op decides, not the status it lands on -- `st triage` removes
# and `st reinstate` does not, and both end at Not Started.
#
# THE DATABASE ALWAYS HOLDS EVERY ARTEFACT IN FULL. This file decides only what
# is ALSO on disk, so dehydration is never a loss and hydration is a
# regeneration rather than a restore.
#
# ABSENT is not EMPTY. A missing file means nobody has said, and everything
# stays; a file present and declaring nothing means keep nothing.
#
# Grammar: `<SIGIL>:<ID>`, sigil STEELTHREAD or ISSUE, optional trailing
# `# comment`. Nothing else. A line the parser cannot read ABORTS the run with
# its line number.
";

/// The DEFAULT declaration written from status: every WIP thread and every
/// OPEN issue, nothing else (AC-01.3).
///
/// **ONE FUNCTION, FOUR CALLERS** (WP-11): the `organize --default` verb, `intent
/// init`, the migration's hop 2, and `intent upgrade` when the file is ABSENT.
/// A second implementation of "what does default mean" is how two projects come
/// to disagree about which threads are realised while both believe they took the
/// default.
///
/// **OPEN IS `!is_closed()`, NEVER AN ENUMERATION OF THE OPEN STATUSES.**
/// WP-11 names them -- WIP, Triage, Not Started, On Hold -- and listing them
/// here would mean a sixth status silently dropping out of every project's
/// default the day it is added, with nothing to report it.
/// [`model::ThreadStatus::is_closed`] already owns that question.
///
/// **SORTED BY ID.** The output is committed and diffed across nineteen
/// estates; iteration order that varies by machine turns "did the default
/// change" into a question nobody can answer by eye. Same discipline as the
/// census id lists in `critic`.
///
/// An empty input yields the header and no declarations, which is the correct
/// content for `intent init`: the file is PRESENT and declares nothing, meaning
/// keep nothing -- as distinct from ABSENT, which means nobody has said.
pub fn default_declaration(
  threads: &[(String, model::ThreadStatus)],
  issues: &[(u32, model::IssueStatus)],
) -> String {
  // **WIP ONLY, AND THIS WAS `!is_closed()` UNTIL hv SAW WHAT IT PRODUCED.**
  // hv, 2026-08-26, first-hand on a 57-thread realised set: _"Now it has NOT
  // STARTED STs!??!"_ and _"It should ONLY HAVE WIP STs!!!!!"_
  //
  // **THE DEFECT WAS A DEFINITION BY EXCLUSION.** "Every status except
  // Completed and Cancelled" reads as a careful rule and is really a list of
  // what to leave out, so every status nobody thought about -- Triage, Not
  // Started, Hold -- was swept IN by default. A thread nobody has started has
  // nothing on disk worth reading, and realising it puts files in the tree that
  // no work refers to.
  //
  // Stated positively, it cannot acquire members by accident: the realised set
  // is the set somebody is WORKING ON. Everything else lives in the store, in
  // full, and `st hydrate` brings any of it back.
  let mut open: Vec<&str> = threads
    .iter()
    .filter(|(_, status)| *status == model::ThreadStatus::Wip)
    .map(|(id, _)| id.as_str())
    .collect();
  open.sort_unstable();
  open.dedup();

  // **OPEN ISSUES, AND `open` MEANS A DIFFERENT THING HERE THAN IT DOES ABOVE
  // -- DELIBERATELY, BECAUSE EACH ENTITY'S OWN STATUS VOCABULARY DEFINES IT.**
  // A thread has a rich status set of which `Wip` is the subset somebody is
  // working on; an issue has exactly Open and Closed, so every open issue is
  // being worked on by definition. Forcing one word to mean one thing across
  // two vocabularies would either sweep every Triage thread back in -- the
  // 2026-08-26 defect hv rejected in as many words -- or exclude open issues
  // for symmetry with a distinction issues do not have.
  let mut open_issues: Vec<u32> = issues
    .iter()
    .filter(|(_, status)| *status == model::IssueStatus::Open)
    .map(|(number, _)| *number)
    .collect();
  open_issues.sort_unstable();
  open_issues.dedup();

  let mut out = String::from(DEFAULT_HEADER);
  for id in open {
    out.push_str(Sigil::SteelThread.as_str());
    out.push(':');
    out.push_str(id);
    out.push('\n');
  }
  for number in open_issues {
    out.push_str(Sigil::Issue.as_str());
    out.push(':');
    out.push_str(&format!("{number:04}"));
    out.push('\n');
  }
  out
}

/// Add `id` to the list, so the artefact realises regardless of status.
///
/// This is what a hand realisation records (AC-05.2). `intent edit ST0011`
/// hydrates a thread the list does not otherwise declare, and without a line
/// here the next `organize` would dehydrate what was just opened.
///
/// The line is APPENDED, so added entries accumulate in the order they were
/// made and a diff shows one added line rather than a reflow.
///
/// **Idempotent.** An id the list already names returns the input unchanged
/// rather than adding a second line -- `intent edit` on the same thread twice
/// is an ordinary thing to do, and a manifest that grows a line each time
/// turns a no-op into a diff. In a flat list, presence is the whole question:
/// an entry declares its artefact wherever it sits.
///
/// **A pin whose id the grammar would refuse is refused here**, at the point
/// of writing, rather than being written and refused on the next read. The
/// alternative writes a file that the tool cannot subsequently parse, which
/// takes a typo and makes it a broken manifest.
pub fn pin(
  original: &str,
  sigil: Sigil,
  id: &str,
  reason: Option<&str>,
) -> Result<String, IntentfilesError> {
  let existing = parse(original)?;
  if !sigil.accepts(id) {
    return Err(IntentfilesError::MalformedId {
      line: 0,
      sigil: sigil.as_str(),
      id: id.to_string(),
    });
  }
  if existing
    .entries
    .iter()
    .any(|e| e.sigil == sigil && e.id == id)
  {
    return Ok(original.to_string());
  }

  let line = match reason {
    Some(r) if !r.trim().is_empty() => format!("{}:{}  # {}", sigil.as_str(), id, r.trim()),
    _ => format!("{}:{}", sigil.as_str(), id),
  };

  let mut text = original.to_string();
  if !text.is_empty() && !text.ends_with('\n') {
    text.push('\n');
  }
  text.push_str(&line);
  text.push('\n');
  Ok(text)
}

/// Remove an artefact from the manifest -- the inverse of [`pin`], and the
/// primitive the CLOSING lifecycle verbs need (AC-05.2).
///
/// **IT REMOVES EVERY LINE NAMING THE ARTEFACT.** [`realised`] answers from
/// `manifest.entries`, every entry, so an `unpin` that left one behind would
/// leave `st done` reporting success while the artefact stayed realised and
/// `organize` went on writing its files.
///
/// # Refusing a malformed id, when nothing could have matched it anyway
///
/// `unpin(m, SteelThread, "ST56")` matches nothing, so returning the manifest
/// unchanged would be defensible and is wrong. **A caller passing an
/// unwritable id has a bug, and the no-op answer is indistinguishable from
/// "that thread was not listed"** -- which is the ordinary, expected outcome
/// this function reports on every second call. One of those two states needs
/// fixing and the other does not, so they must not share an answer. Refusing
/// at the write is what [`pin`] does for the same reason, and the symmetry is
/// worth more here than a permissive removal.
///
/// # Idempotent, because a closing verb must be re-runnable
///
/// Removing an id the manifest does not name returns the original unchanged.
/// `st done` on an already-closed thread, a re-run after a partial failure,
/// and a thread created with `--dehydrate` and then closed all arrive here
/// with nothing to remove, and none of them is an error.
///
/// Everything else in the file survives byte for byte: comments, blank lines
/// and the order of the entries that stay. The lines to drop come
/// from [`parse`], which owns the grammar -- **re-deciding here which lines are
/// entries would be a second reader of the same syntax, and the two would
/// disagree on the first line either got wrong.**
pub fn unpin(original: &str, sigil: Sigil, id: &str) -> Result<String, IntentfilesError> {
  let existing = parse(original)?;
  if !sigil.accepts(id) {
    return Err(IntentfilesError::MalformedId {
      line: 0,
      sigil: sigil.as_str(),
      id: id.to_string(),
    });
  }

  let doomed: std::collections::BTreeSet<usize> = existing
    .entries
    .iter()
    .filter(|e| e.sigil == sigil && e.id == id)
    .map(|e| e.line)
    .collect();
  if doomed.is_empty() {
    return Ok(original.to_string());
  }

  let kept: Vec<&str> = original
    .lines()
    .enumerate()
    .filter(|(i, _)| !doomed.contains(&(i + 1)))
    .map(|(_, l)| l)
    .collect();

  let mut text = kept.join("\n");
  if !text.is_empty() {
    text.push('\n');
  }
  Ok(text)
}
