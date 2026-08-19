//! The dehydration ship gate (AC-00.1, ST0057 WP-00).
//!
//! **Two gates carry the word and they guard different things.**
//! [`crate::organize::gate`] is AC-04.2's: per file, re-render and compare, and
//! it answers "can the store put THIS file back". This one is AC-00.1's: whole
//! estate, once per run, and it answers "has this estate proved it can put
//! ANYTHING back yet". A green from either says nothing about the other, which
//! is why they are separate functions with separate criteria rather than one
//! `gate` that grew a second job.
//!
//! **WHAT THIS READS, AND WHY IT IS NOT A SECOND DECLARATION.** The set of
//! preconditions lives in exactly one place -- the delimited
//! `<<PRECONDITIONS ... PRECONDITIONS>>` block inside ST0057's own `AC-00.1`
//! text -- and this module reads it. Naming WHERE the declaration lives is not
//! copying WHAT it says: the two constants below are an address, and an address
//! cannot go stale in the way a hand-copied list can, because a wrong address
//! finds nothing and this gate refuses on finding nothing. A hand-copied subset
//! is the failure AC-00.1 exists to catch, and it is the failure a builder
//! implementing that row literally would have shipped.
//!
//! **AND IT READS CANON, NEVER `acceptance.md`.** `Project::classify` calls
//! that file a `GeneratedView`, so it is a projection `organize` itself
//! regenerates -- a gate reading its own verb's output would be checking a copy
//! whose currency the verb controls. Canon is the authored side.
//!
//! **SATISFACTION IS ASKED, NOT COMPUTED.** [`contract::resolve`] is the one
//! answer to "is this criterion met", shared with `ac list` and the close gate.
//! Re-deriving it here would put a second opinion in front of the destructive
//! path, and the two would agree exactly until one of them moved.
//!
//! **REFUSE-BY-DEFAULT IS STRUCTURAL.** [`check`] returns a [`Verdict`] and
//! nothing else -- there is no `Result` for a caller to `unwrap_or` into
//! permission. An absent thread, an absent criterion, a missing block, two
//! blocks, an empty block or a malformed token all produce a verdict that
//! refuses and says which. This is the same ground `organize::gate` refuses an
//! unreadable view on: if there are no bytes to compare, the claim is unproven,
//! and unproven is not permission.
//!
//! **THIS GATE IS SCAFFOLDING WITH A DEFINED END.** `design.md` says build
//! `organize` now and ship dehydration behind the gate; when every declared
//! precondition is green, the commit that ships dehydration is the one that
//! deletes this module. Until then an empty declaration REFUSES rather than
//! opening -- an empty block is far likelier to be a deletion nobody noticed
//! than a considered "all clear", and the considered version removes the gate
//! rather than emptying its input.

use std::fmt;

use crate::contract::{self, Resolved};
use crate::ingest::Canon;

/// **THE DELIMITER IS THE ADDRESS. There is no thread id and no criterion id in
/// this file, and their absence is the design rather than an omission.**
///
/// The first version hard-coded the declaring thread and criterion as
/// constants, and `no_shipped_string_literal_carries_pm_state` refused it. That
/// guard is a string-literal scan standing in for a semantic rule, and here the
/// proxy caught the real defect: **this gate ships inside a binary other
/// projects run, and Intent's own thread ids mean nothing in a consumer's
/// estate.** A consumer hitting the refusal would have been told to go and read
/// a thread that does not exist for them.
///
/// So the declaration SELF-IDENTIFIES: exactly one criterion in the estate
/// carries the delimited block, and this finds it by the delimiter. The
/// delimiter is a machine-shaped token rather than project-management state, so
/// it is the same in every estate that has one and absent from every estate that
/// does not.
///
/// **THE ADDRESS THAT CANNOT GO STALE IS THE ONE THAT DOES NOT EXIST.** The
/// previous version's virtue was that a wrong address finds nothing and finding
/// nothing refuses. This is strictly better: there is no address to be wrong,
/// and the thing that identifies the declaration is the thing that makes it
/// machine-readable in the first place.
const OPEN: &str = "<<PRECONDITIONS";
const CLOSE: &str = "PRECONDITIONS>>";

/// Why one declared precondition does not permit dehydration.
///
/// **`Descoped` and `Withdrawn` ARE UNMET, and that is a decision rather than
/// an oversight.** Dropping a precondition is not meeting it. Treating either
/// as satisfied would make `intent ac descope` a way to open a gate standing in
/// front of the whole estate -- one command, no evidence, no removal of the
/// gate itself. Both are named distinctly in the refusal so an operator sees
/// that the criterion was dropped rather than failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unmet {
  /// In scope and not satisfied -- the ordinary case.
  Unsatisfied,
  Descoped,
  Withdrawn,
  /// Declared, and no criterion by that id exists. **Unmet, not skipped:** a
  /// declared id that resolves to nothing is exactly how a list quietly
  /// shrinks, and skipping it would make checked disagree with declared while
  /// both numbers still looked healthy.
  NoSuchCriterion,
}

impl fmt::Display for Unmet {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let s = match self {
      Self::Unsatisfied => "not satisfied",
      Self::Descoped => "descoped -- dropped, which is not met",
      Self::Withdrawn => "withdrawn -- dropped, which is not met",
      Self::NoSuchCriterion => "declared, but no such criterion exists",
    };
    f.write_str(s)
  }
}

/// Why the declaration itself could not be read.
///
/// Every variant refuses. They are distinguished because the remedies differ
/// entirely -- a missing block is an editing accident, two blocks is the very
/// duplication AC-00.1 forbids, and a malformed token means the gate would have
/// silently checked a subset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unreadable {
  /// No criterion anywhere in canon carries the block.
  ///
  /// **This is the ordinary state of every project that is not Intent, and its
  /// message says so rather than naming Intent's own paperwork.** The refusal
  /// still stands -- an estate that has declared no preconditions has proved
  /// nothing about its ability to put files back.
  NoDeclaration,
  /// More than one criterion carries a block. **The duplication the whole row
  /// exists to forbid, so choosing between them is the one thing this must not
  /// do.**
  TwoDeclarations,
  /// One criterion carries two blocks -- the same duplication, one level in.
  TwoBlocks,
  Empty,
  /// An opening delimiter with no closer.
  Unterminated,
  /// A token inside the block that is not an `AC-<gg>.<n>` id. **Refused
  /// rather than filtered out**: filtering is how a declaration of nineteen
  /// becomes a check of eighteen with nothing saying so.
  Malformed(String),
}

impl fmt::Display for Unreadable {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      // **No Intent identifier here, and that is the point.** This is what a
      // consumer of the tool sees, and it has to be a sentence about THEIR
      // estate.
      Self::NoDeclaration => write!(
        f,
        "this project declares no dehydration preconditions, so nothing has been proved about removing files from it"
      ),
      Self::TwoDeclarations => write!(
        f,
        "more than one acceptance criterion carries a `{OPEN} ... {CLOSE}` block, so there is no single declaration to read and choosing between them is exactly what a gate must not do"
      ),
      Self::TwoBlocks => write!(
        f,
        "one acceptance criterion carries more than one `{OPEN} ... {CLOSE}` block, so there is no single declaration to read"
      ),
      Self::Unterminated => write!(
        f,
        "a `{OPEN}` opens a declaration that is never closed with `{CLOSE}`"
      ),
      Self::Empty => write!(
        f,
        "the `{OPEN} ... {CLOSE}` block is empty; an all-clear removes this gate rather than emptying its input"
      ),
      Self::Malformed(token) => write!(
        f,
        "the `{OPEN} ... {CLOSE}` block carries `{token}`, which is not an AC id"
      ),
    }
  }
}

/// What the gate found. **The only answer this module produces, and it has no
/// error channel** -- see the module docs on refuse-by-default.
///
/// **THE FIELDS ARE PRIVATE AND [`check`] IS THE ONLY CONSTRUCTOR, WHICH IS
/// WHAT MAKES REFUSE-BY-DEFAULT STRUCTURAL RATHER THAN POLITE.** Public fields
/// would let any caller -- a test in a hurry most of all -- write a permitting
/// verdict by hand and hand it to the destructive path, and the gate would then
/// be enforced by everyone remembering not to do that. A test that needs
/// removals has to build canon the real declaration is satisfied in, which is
/// the same work the estate itself has to do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verdict {
  declared: Vec<String>,
  checked: Vec<String>,
  unmet: Vec<(String, Unmet)>,
  unreadable: Option<Unreadable>,
}

impl Verdict {
  /// Whether dehydration may remove anything.
  ///
  /// The only route to `true` is a readable, non-empty declaration with every
  /// one of its entries resolved satisfied.
  pub fn permits(&self) -> bool {
    self.unreadable.is_none() && !self.declared.is_empty() && self.unmet.is_empty()
  }

  /// Every precondition the declaration names, in declaration order.
  pub fn declared(&self) -> &[String] {
    &self.declared
  }

  /// Every declared precondition this gate resolved -- equal to [`Self::declared`]
  /// unless something stopped it reading the list at all.
  ///
  /// **Carried as its own list so the denominator can be PRINTED rather than
  /// asserted.** AC-00.1 asks for preconditions checked against preconditions
  /// declared; two counts equal by construction still have to be visible, or
  /// the day they stop being equal nothing says so.
  pub fn checked(&self) -> &[String] {
    &self.checked
  }

  /// Every declared precondition that does not permit dehydration, with why.
  pub fn unmet(&self) -> &[(String, Unmet)] {
    &self.unmet
  }

  /// Why the declaration itself could not be read, if it could not.
  pub fn unreadable(&self) -> Option<&Unreadable> {
    self.unreadable.as_ref()
  }

  fn refuse(unreadable: Unreadable) -> Self {
    Self {
      declared: Vec::new(),
      checked: Vec::new(),
      unmet: Vec::new(),
      unreadable: Some(unreadable),
    }
  }
}

/// The denominator line, printed on BOTH answers.
///
/// A gate that prints its counts only when it refuses cannot be distinguished,
/// on a quiet run, from a gate that checked nothing.
impl fmt::Display for Verdict {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if let Some(why) = &self.unreadable {
      return write!(
        f,
        "the dehydration preconditions could not be read: {why}. 0 checked of 0 declared, so nothing is proved and nothing may be removed"
      );
    }
    write!(
      f,
      "{} checked of {} declared",
      self.checked.len(),
      self.declared.len()
    )?;
    if self.unmet.is_empty() {
      return write!(f, ", all met");
    }
    write!(f, ", {} unmet: ", self.unmet.len())?;
    let named: Vec<String> = self
      .unmet
      .iter()
      .map(|(id, why)| format!("{id} ({why})"))
      .collect();
    f.write_str(&named.join(", "))
  }
}

/// Read the declaration and resolve every precondition it names.
///
/// Takes the whole canon rather than the declaring thread, so the address above
/// is resolved HERE -- a caller that had to find `ST0057` itself would be the
/// second place the address lives.
pub fn check(canon: &Canon) -> Verdict {
  // **Every criterion in the estate is a candidate, and finding TWO refuses.**
  // Scanning for the sole carrier is what makes the declaration single by
  // MEASUREMENT rather than by an address that asserts it -- an address points
  // at one and says nothing about whether a second exists somewhere else.
  let mut carriers = canon
    .threads
    .iter()
    .flat_map(|t| t.criteria.iter().map(move |c| (t, c)))
    .filter(|(_, c)| c.text.contains(OPEN));

  let Some((thread, criterion)) = carriers.next() else {
    return Verdict::refuse(Unreadable::NoDeclaration);
  };
  if carriers.next().is_some() {
    return Verdict::refuse(Unreadable::TwoDeclarations);
  }

  let declared = match declared_in(&criterion.text) {
    Ok(ids) => ids,
    Err(why) => return Verdict::refuse(why),
  };

  // Resolved against the SAME thread the declaration lives in. Every declared
  // id is ST0057's own, which is why one lookup covers the list; an id from
  // another thread would land in `NoSuchCriterion` and refuse rather than being
  // resolved somewhere this gate never looked.
  let mut unmet = Vec::new();
  for id in &declared {
    let state = thread
      .criteria
      .iter()
      .find(|c| &c.id == id)
      .map(|c| contract::resolve(thread, c));
    match state {
      Some(Resolved::Satisfied) => {}
      Some(Resolved::Unsatisfied) => unmet.push((id.clone(), Unmet::Unsatisfied)),
      Some(Resolved::Descoped) => unmet.push((id.clone(), Unmet::Descoped)),
      Some(Resolved::Withdrawn) => unmet.push((id.clone(), Unmet::Withdrawn)),
      None => unmet.push((id.clone(), Unmet::NoSuchCriterion)),
    }
  }

  Verdict {
    checked: declared.clone(),
    declared,
    unmet,
    unreadable: None,
  }
}

/// The ids inside the delimited block, in the order written.
///
/// Delimited rather than scanned from the whole text because AC ids are named
/// elsewhere in that criterion for other reasons -- the mapping paragraph names
/// six of them -- and a whole-text scan would sweep those in, producing a
/// declaration nobody wrote.
pub fn declared_in(text: &str) -> Result<Vec<String>, Unreadable> {
  let mut opens = text.match_indices(OPEN);
  let Some((open_at, _)) = opens.next() else {
    return Err(Unreadable::NoDeclaration);
  };
  if opens.next().is_some() {
    return Err(Unreadable::TwoBlocks);
  }
  let after = &text[open_at + OPEN.len()..];
  let Some(close_at) = after.find(CLOSE) else {
    // An opening delimiter with no closer. Reachable from [`check`] only
    // because the carrier was found BY the opener, so a half-written block
    // refuses rather than reading to the end of the criterion.
    return Err(Unreadable::Unterminated);
  };
  let body = &after[..close_at];

  let mut ids = Vec::new();
  for token in body.split_whitespace() {
    if !is_ac_id(token) {
      return Err(Unreadable::Malformed(token.to_string()));
    }
    ids.push(token.to_string());
  }
  if ids.is_empty() {
    return Err(Unreadable::Empty);
  }
  Ok(ids)
}

/// `AC-<gg>.<n>` -- the machine-shaped token hv's ruling turns on.
///
/// Hand-written rather than a regex so the shape is readable at the site that
/// depends on it, and so the module carries no dependency for eight lines.
fn is_ac_id(token: &str) -> bool {
  let Some(rest) = token.strip_prefix("AC-") else {
    return false;
  };
  let Some((group, seq)) = rest.split_once('.') else {
    return false;
  };
  group.len() == 2
    && group.bytes().all(|b| b.is_ascii_digit())
    && !seq.is_empty()
    && seq.bytes().all(|b| b.is_ascii_digit())
}
