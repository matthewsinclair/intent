//! `.intentfiles` -- the realisation manifest and its REFUSING grammar.
//!
//! WP-02 of ST0057. The manifest declares WHICH ARTEFACTS are realised to disk.
//! It is committed, it has two regions, and its parser refuses rather than
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
//! # Two regions (AC-02.2, AC-02.3)
//!
//! Lines between [`BEGIN_MARKER`] and [`END_MARKER`] are GENERATED from status
//! by `intent organize`. Lines outside them are PINS and survive a rewrite
//! byte for byte.
//!
//! The generated region is a FUNCTION OF CURRENT STATUS, not a memory of what
//! realisation last produced. Nothing is remembered, so nothing can go stale --
//! which is why a hand edit is distinguished from generated content BY POSITION
//! rather than by content. A hand realisation written to the pinned region
//! survives; written to the generated region the next run REVERTS it (AC-05.2).
//!
//! Without the split: pin `ST0011` to keep reading it, it closes, `organize`
//! regenerates the block from status, and the pin is gone along with the files
//! with nothing in the output naming the decision (AC-02.3).
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

/// Opens the generated region. Everything after it, up to [`END_MARKER`], is
/// rewritten from status by `organize`.
pub const BEGIN_MARKER: &str = "# BEGIN INTENT";
/// Closes the generated region.
pub const END_MARKER: &str = "# END INTENT";

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
}

impl Sigil {
  /// The wire form, which is also the only accepted spelling.
  pub fn as_str(&self) -> &'static str {
    match self {
      Sigil::SteelThread => "STEELTHREAD",
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
    }
  }
}

/// Which region a line came from. **The pin/generated distinction is
/// POSITIONAL**, so it is carried on the entry rather than inferred later from
/// content -- inferring it from content is exactly the mistake the two-region
/// design exists to prevent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Region {
  /// Outside the markers. Survives an `organize` rewrite byte for byte.
  Pinned,
  /// Between the markers. Rewritten from status on every run.
  Generated,
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
  pub region: Region,
  /// 1-indexed, as a human reads the file.
  pub line: usize,
}

/// A parsed manifest.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Manifest {
  pub entries: Vec<Entry>,
}

impl Manifest {
  /// The pinned entries, in file order.
  pub fn pinned(&self) -> impl Iterator<Item = &Entry> {
    self.entries.iter().filter(|e| e.region == Region::Pinned)
  }

  /// The generated entries, in file order.
  pub fn generated(&self) -> impl Iterator<Item = &Entry> {
    self
      .entries
      .iter()
      .filter(|e| e.region == Region::Generated)
  }
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
    match self {
      Realised::NothingSaid | Realised::Unreadable => true,
      Realised::Declared(set) => set.contains(thread_id),
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
/// **THE SIGIL FILTER BELOW IS A NO-OP TODAY AND IS NOT DEAD.** hv retired
/// `ISSUE:` on 2026-08-20, so every entry is a `SteelThread` and the filter
/// excludes nothing. It stays because the sigil space is queued to grow, and
/// deleting it would put the bug back the day it does. `intentfiles_is_the_
/// list.rs` records that no fixture can currently catch its removal.
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
    .filter(|e| e.sigil == Sigil::SteelThread)
    .map(|e| e.id.clone())
    .collect()
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
  #[error("line {line}: `{found}` is not a known sigil -- expected STEELTHREAD")]
  UnknownSigil { line: usize, found: String },
  #[error("line {line}: `{line_text}` is not `<SIGIL>:<ID>`")]
  NotAnEntry { line: usize, line_text: String },
  #[error("line {line}: `{id}` is not a valid id for {sigil}")]
  MalformedId {
    line: usize,
    sigil: &'static str,
    id: String,
  },
  #[error("line {line}: END marker with no matching BEGIN")]
  UnopenedRegion { line: usize },
  #[error("line {line}: BEGIN marker inside an already-open generated region")]
  NestedRegion { line: usize },
  #[error("line {line}: the generated region opened here is never closed")]
  UnclosedRegion { line: usize },
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
      | IntentfilesError::MalformedId { line, .. }
      | IntentfilesError::UnopenedRegion { line }
      | IntentfilesError::NestedRegion { line }
      | IntentfilesError::UnclosedRegion { line } => *line,
    }
  }
}

impl Remedy for IntentfilesError {
  fn remedy(&self) -> String {
    match self {
      IntentfilesError::UnknownSigil { .. } => {
        "write STEELTHREAD:<ID>; the manifest names artefacts, never files".into()
      }
      IntentfilesError::NotAnEntry { .. } => {
        "each line is blank, a comment, a BEGIN/END marker, or `<SIGIL>:<ID>` with an optional trailing `# comment`".into()
      }
      IntentfilesError::MalformedId { sigil, .. } => match *sigil {
        "STEELTHREAD" => "a steel-thread id is ST followed by four digits, eg ST0000".into(),
        // **UNREACHABLE WHILE THE GRAMMAR HAS ONE SIGIL, AND IT MUST NOT GUESS.**
        // This arm read "an issue id is four digits" -- correct while ISSUE was
        // the only other sigil, and silently wrong for whatever sigil is added
        // next. A remedy that confidently names the wrong shape is worse than
        // one that admits it has none, so it names the sigil and asks for its
        // remedy to be written beside STEELTHREAD's.
        other => format!(
          "`{other}` is a sigil this remedy has no id shape for -- add one beside STEELTHREAD's"
        ),
      },
      IntentfilesError::UnopenedRegion { .. } => {
        format!("remove this line, or add `{BEGIN_MARKER}` above the generated block")
      }
      IntentfilesError::NestedRegion { .. } => {
        format!("close the open region with `{END_MARKER}` before opening another")
      }
      IntentfilesError::UnclosedRegion { .. } => {
        format!("add `{END_MARKER}` after the generated block")
      }
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
  let mut region = Region::Pinned;
  let mut opened_at = 0usize;

  for (idx, raw) in text.lines().enumerate() {
    let line = idx + 1;
    let trimmed = raw.trim();

    if trimmed == BEGIN_MARKER {
      if region == Region::Generated {
        return Err(IntentfilesError::NestedRegion { line });
      }
      region = Region::Generated;
      opened_at = line;
      continue;
    }
    if trimmed == END_MARKER {
      if region == Region::Pinned {
        return Err(IntentfilesError::UnopenedRegion { line });
      }
      region = Region::Pinned;
      continue;
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
      region,
      line,
    });
  }

  if region == Region::Generated {
    return Err(IntentfilesError::UnclosedRegion { line: opened_at });
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
// `Region` and `Manifest::pinned()/generated()` survive because `pin` still
// uses them. Whether the BEGIN/END marker grammar should survive AT ALL is a
// separate question, deliberately not folded into this ruling.

/// Add a PIN for `id`, so the artefact realises regardless of status.
///
/// This is what a hand realisation records (AC-05.2). `intent edit ST0011`
/// hydrates a thread the estate is not otherwise realising, and the record of
/// that decision has exactly one correct home: **the pinned region**. Written
/// to the generated region it would survive until the next `organize` and then
/// vanish, because that region is a function of status and the thing somebody
/// opened by hand is typically the thing status does not offer.
///
/// The pin lands at the END of the pinned region, immediately above the
/// markers, so hand-added pins accumulate in the order they were made and a
/// diff shows one added line rather than a reflow.
///
/// **Idempotent.** Pinning an already-pinned id returns the input unchanged
/// rather than adding a second line -- `intent edit` on the same thread twice
/// is an ordinary thing to do, and a manifest that grows a line each time
/// turns a no-op into a diff.
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
  if existing.pinned().any(|e| e.sigil == sigil && e.id == id) {
    return Ok(original.to_string());
  }

  let line = match reason {
    Some(r) if !r.trim().is_empty() => format!("{}:{}  # {}", sigil.as_str(), id, r.trim()),
    _ => format!("{}:{}", sigil.as_str(), id),
  };

  let mut out: Vec<String> = Vec::new();
  let mut placed = false;
  for raw in original.lines() {
    if raw.trim() == BEGIN_MARKER && !placed {
      out.push(line.clone());
      placed = true;
    }
    out.push(raw.to_string());
  }
  if !placed {
    out.push(line);
  }

  let mut text = out.join("\n");
  text.push('\n');
  Ok(text)
}

/// Remove an artefact from the manifest -- the inverse of [`pin`], and the
/// primitive the CLOSING lifecycle verbs need (AC-05.2).
///
/// **IT REMOVES FROM BOTH REGIONS, AND THAT IS THE WHOLE POINT RATHER THAN A
/// CONVENIENCE.** [`realised`] answers from `manifest.entries` -- every entry,
/// pinned or generated -- so an `unpin` that only cleared the pinned region
/// would leave `st done` reporting success while the artefact stayed realised
/// and `organize` went on writing its files. The asymmetry with `pin` (which
/// only ever WRITES to the pinned region) is deliberate: **where a line goes
/// is a decision, and whether a line is there at all is a fact.**
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
/// Everything else in the file survives byte for byte: comments, blank lines,
/// the markers, and the order of the entries that stay. The lines to drop come
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
